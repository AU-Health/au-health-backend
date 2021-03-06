use std::convert::TryFrom;

use argon2::{
    password_hash::{rand_core::OsRng, SaltString},
    Argon2, PasswordHasher,
};
use async_graphql::{Error, InputObject};
use chrono::Utc;
use sqlx::{Pool, Postgres};
use unicode_segmentation::UnicodeSegmentation;
use uuid::Uuid;
use validator::validate_email;

use super::{Role, User};

#[derive(InputObject, Clone)]
/// Input from GQL
pub struct NewUser {
    pub email: String,
    pub password: String,
}

pub struct ValidEmail(String);


//valid emails end in american.edu. we are not checking against the full
//AU database to make sure the emails themselves are actually valid, though.
impl ValidEmail {
    pub fn parse(s: String) -> Result<Self, String> {
        match validate_email(&s) && s.contains("american.edu") {
            true => Ok(Self(s)),
            false => Err(format!("{} is not a valid email", s)),
        }
    }
}


//rust thing where the strings have to be references and not literals
//helps with lifetimes and ownership.
impl AsRef<str> for ValidEmail {
    fn as_ref(&self) -> &str {
        &self.0
    }
}


// Checks if user's inputted password is valid. If not valid, the program will tell the user it's invalid
pub struct ValidPassword(String);

impl ValidPassword {
    pub fn parse(s: String) -> Result<Self, String> {
        let is_empty_or_whitespace = s.trim().is_empty();

        let len = s.graphemes(true).count();

        let is_too_long_or_too_short = !(5..=256).contains(&len);

        let forbidden_chars = ['/', '(', ')', '"', '<', '>', '\\', '{', '}'];
        let contains_forbidden_chars = s.chars().any(|c| forbidden_chars.contains(&c));

        if is_empty_or_whitespace || is_too_long_or_too_short || contains_forbidden_chars {
            Err(format!("{} is not a valid subscriber name.", s))
        } else {
            Ok(Self(s))
        }
    }
}


//same as the other AsRef above
impl AsRef<str> for ValidPassword {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

pub struct VerifiedNewUser {
    email: ValidEmail,
    password: ValidPassword,
}

// Part 1: when the user is successfully verified, the program will parse the user's credentials
impl TryFrom<NewUser> for VerifiedNewUser {
    type Error = String;

    fn try_from(new_user: NewUser) -> Result<VerifiedNewUser, String> {
        let email = ValidEmail::parse(new_user.email)?;

        let password = ValidPassword::parse(new_user.password)?;

        Ok(VerifiedNewUser { email, password })
    }
}

// Part 2: when the user is successfully verified, the program will parse the user's credentials
impl VerifiedNewUser {
    pub async fn register_user(
        self,
        pool: &Pool<Postgres>,
        argon2: &Argon2<'_>,
    ) -> Result<User, Error> {
        let salt = SaltString::generate(&mut OsRng);

        let hashed_password = argon2
            .hash_password(self.password.as_ref().as_bytes(), &salt)?
            .to_string();

        let user = sqlx::query_as!(
            User,
            r#"INSERT INTO user_account (id, email, password, created_at, updated_at, role)
        VALUES ($1, $2, $3, $4, $5, $6)
        RETURNING id, email, password, created_at, updated_at, role as "role: _";"#,
            Uuid::new_v4(),
            self.email.as_ref(),
            hashed_password,
            Utc::now(),
            Utc::now(),
            Role::User as Role
        )
        .fetch_one(pool)
        .await
        .map_err(|e| {
            tracing::error!("Failed to execute query: {:?}", e);
            e
        })?;

        Ok(user)
    }
}
