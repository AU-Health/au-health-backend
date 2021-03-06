use std::convert::{TryFrom, TryInto};

use serde_aux::field_attributes::deserialize_number_from_string;
use sqlx::postgres::{PgConnectOptions, PgSslMode};

//this file currently works as intended. It sets everything up to run with settings and etc.

pub fn get_configuration() -> Result<Settings, config::ConfigError> {
    let mut settings = config::Config::default();

    let base_path = std::env::current_dir().expect("Failed to load current directory");
    let config_dir = base_path.join("configuration");

    settings.merge(config::File::from(config_dir.join("base")).required(true))?;

    let environment: Environment = std::env::var("APP_ENVIRONMENT")
        .unwrap_or_else(|_| "local".into())
        .try_into()
        .expect("Failed to get APP_ENVIRONMENT");

    settings.merge(config::File::from(config_dir.join(environment.as_str())).required(true))?;

    settings.merge(config::Environment::with_prefix("APP_").separator("__"))?;

    settings.try_into()
}

// [derive(serde::Deserialize)] is a data structure that can be deserialized from any data format supported
// by Serde. Refer to https://docs.serde.rs/serde/trait.Deserialize.html

#[derive(serde::Deserialize)]
pub struct Settings {
    pub database: DatabaseSettings,
    pub application: ApplicationSettings,
}

#[derive(serde::Deserialize)]

pub struct DatabaseSettings {
    pub postgres: PostgresSettings,
    pub redis: RedisSettings,
}

#[derive(serde::Deserialize)]
pub struct PostgresSettings {
    pub username: String,
    pub password: String,
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub port: u16,
    pub host: String,
    pub database_name: String,
    pub require_ssl: bool,
}

#[derive(serde::Deserialize)]
pub struct RedisSettings {
    host: String,
    port: String,
    username: String,
    password: String,
}

impl RedisSettings {
    pub fn with_port(&self) -> String {
        format!("{}:{}", self.without_port(), self.port)
    }
    pub fn without_port(&self) -> String {
        format!("redis://{}:{}@{}", self.username, self.password, self.host)
    }
}

impl PostgresSettings {
    pub fn with_db(&self) -> PgConnectOptions {
        self.without_db().database(&self.database_name)
    }
    pub fn without_db(&self) -> PgConnectOptions {
        let ssl_mode = if self.require_ssl {
            PgSslMode::Require
        } else {
            PgSslMode::Prefer
        };

        PgConnectOptions::new()
            .host(&self.host)
            .username(&self.username)
            .password(&self.password)
            .port(self.port)
            .ssl_mode(ssl_mode)
    }
}

#[derive(serde::Deserialize)]
pub struct GraphQlSettings {
    pub path: String,
    pub playground_enabled: bool,
}

#[derive(serde::Deserialize)]
pub struct ApplicationSettings {
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub port: u16,
    pub host: String,
    pub graphql: GraphQlSettings,
    pub root_email: String,
    pub root_password: String,
}

pub enum Environment {
    Local,
    Production,
    Docker,
}

impl Environment {
    pub fn as_str(&self) -> &'static str {
        match self {
            Environment::Local => "local",
            Environment::Production => "production",
            Environment::Docker => "docker",
        }
    }
}

impl TryFrom<String> for Environment {
    type Error = String;

    fn try_from(s: String) -> Result<Self, Self::Error> {
        match s.to_lowercase().as_str() {
            "local" => Ok(Self::Local),
            "production" => Ok(Self::Production),
            "docker" => Ok(Self::Docker),
            other => Err(format!(
                "{} is not a supported environment. Use either 'local' or 'production'",
                other
            )),
        }
    }
}
