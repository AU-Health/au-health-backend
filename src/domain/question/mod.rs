use async_graphql::{Error, InputObject, SimpleObject};
use chrono::{DateTime, Utc};
use sqlx::{Pool, Postgres};
use uuid::Uuid;

#[derive(InputObject)]
pub struct NewQuestion {
    pub question: String,
    pub category: String,
    pub response_type: String,
    pub responses: Option<Vec<String>>,
}

#[derive(SimpleObject, Debug)]
pub struct Question {
    pub id: Uuid,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub question: String,
    pub category_id: Uuid,
    pub response_type_id: Uuid,
}

pub struct Category {
    pub id: Uuid,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub name: String,
}

pub struct ResponseType {
    pub id: Uuid,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub response_type: String,
}

impl NewQuestion {
    pub async fn save_to_db(self, pool: &Pool<Postgres>) -> Result<Question, Error> {
        let category_query = sqlx::query_as!(Category, "SELECT id, created_at, updated_at, name FROM question_category WHERE name = $1 LIMIT 1;", self.category).fetch_one(pool).await;

        let category = match category_query {
            Ok(cat) => Ok(cat),
            Err(_) => {
                sqlx::query_as!(Category, "INSERT INTO question_category (id, created_at, updated_at, name) VALUES ($1, $2, $3, $4) RETURNING *;", Uuid::new_v4(), Utc::now(), Utc::now(), self.category).fetch_one(pool).await
            }
        }?;

        let response_type = sqlx::query_as!(ResponseType, "SELECT id, created_at, updated_at, response_type FROM response_type WHERE response_type = $1 LIMIT 1;", self.response_type).fetch_one(pool).await?;

        let question = sqlx::query_as!(Question, "INSERT INTO question (id, created_at, updated_at, question, category_id, response_type_id) 
        VALUES ($1, $2, $3, $4, $5, $6)
        RETURNING *;", Uuid::new_v4(), Utc::now(), Utc::now(), self.question, category.id, response_type.id).fetch_one(pool).await?;

        Ok(question)
    }
}
