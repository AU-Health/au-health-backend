use async_graphql::{Error, InputObject};
use chrono::Utc;
use futures::future::join_all;
use sqlx::{Pool, Postgres};
use uuid::Uuid;

use super::SurveyResponse;

#[derive(InputObject)]
pub struct NewAnswer {
    pub question_id: Uuid,
    pub answer: String,
}

#[derive(InputObject)]
pub struct NewSurveyResponse {
    pub answers: Vec<NewAnswer>,
}


//this will create an error if the server is not running- that is ok!

impl NewSurveyResponse {
    pub async fn save_to_db(
        self,
        pool: &Pool<Postgres>,
        user_id: Uuid,
    ) -> Result<SurveyResponse, Error> {
        let survey_response = sqlx::query_as!(
            SurveyResponse,
            "INSERT INTO survey_response (id, user_id, created_at, updated_at)
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


        // Inserts the answer elements from the user into the query.
        let answer_queries = self.answers.into_iter().map(|ans| {
            sqlx::query!(
                "INSERT INTO answer (id, created_at, updated_at, answer, survey_response_id, question_id)
            VALUES ($1, $2, $3, $4, $5, $6);",
                Uuid::new_v4(),
                Utc::now(),
                Utc::now(),
                ans.answer,
                survey_response.id,
                ans.question_id
            )
            .execute(pool)
        });

        // Suspends the execution of an aysnchronous funtion (ie. save_to_db)
        let futures = join_all(answer_queries).await;

        for result in futures {
            if let Err(e) = result {
                return Err(Error::new(e.to_string()));
            }
        }

        Ok(survey_response)
    }
}
