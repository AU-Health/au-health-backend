use async_graphql::{Error, InputObject};
use chrono::Utc;
use futures::future::join_all;
use sqlx::{Pool, Postgres};
use uuid::Uuid;

use super::Survey;

#[derive(InputObject)]
pub struct NewAnswer {
    pub question_id: Uuid,
    pub answer: String,
}

#[derive(InputObject)]
pub struct NewSurvey {
    pub answers: Vec<NewAnswer>,
}

impl NewSurvey {
    pub async fn create_survey(
        self,
        pool: &Pool<Postgres>,
        user_id: Uuid,
    ) -> Result<Survey, Error> {
        let survey = sqlx::query_as!(
            Survey,
            "INSERT INTO survey (id, user_id, created_at, updated_at)
        VALUES ($1, $2, $3, $4)
        RETURNING *;",
            Uuid::new_v4(),
            user_id,
            Utc::now(),
            Utc::now()
        )
        .fetch_one(pool)
        .await
        .map_err(|e| {
            tracing::error!("Failed to execute query: {:?}", e);
            e
        })?;

        let answer_queries = self.answers.into_iter().map(|ans| {
            sqlx::query!(
                "INSERT INTO answer (id, created_at, updated_at, answer, survey_id, question_id) 
            VALUES ($1, $2, $3, $4, $5, $6);",
                Uuid::new_v4(),
                Utc::now(),
                Utc::now(),
                ans.answer,
                survey.id,
                ans.question_id
            )
            .execute(pool)
        });

        let futures = join_all(answer_queries).await;

        for result in futures {
            if let Err(e) = result {
                return Err(Error::new(e.to_string()));
            }
        }

        Ok(survey)
    }
}
