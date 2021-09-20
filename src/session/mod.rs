mod cookie;
pub use cookie::*;
mod manager;
pub use manager::*;

pub const AUTH_COOKIE_NAME: &str = "auth";
pub const USER_ID_SESSION_KEY: &str = "user_id";
