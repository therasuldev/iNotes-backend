use regex::Regex;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct User {
    pub id: i32,
    pub avatar: Option<String>,
    pub username: String,
    pub email: String,
}

#[derive(Deserialize)]
pub struct RegisterForm {
    pub email: String,
    pub username: String,
    pub password: String,
}

#[derive(Deserialize)]
pub struct LoginForm {
    pub email: Option<String>,
    pub username: String,
    pub password: String,
}

lazy_static::lazy_static! {
    // Matches most common email formats
    pub static ref EMAIL_REGEX: Regex = Regex::new(r"^[\w\-\.]+@([\w\-]+\.)+[\w\-]{2,4}$").unwrap();
    // Matches usernames with alphanumeric characters and underscores, 3 to 15 characters long
    pub static ref USERNAME_REGEX: Regex = Regex::new(r"^[a-zA-Z0-9_]{3,15}$").unwrap();
    // Matches any string at least 8 characters long, including special characters
    // pub static ref PASSWORD_REGEX: Regex = Regex::new(r#"^(?=(.*[\d]){1,})(?=(.*[a-z]){1,})(?=(.*[A-Z]){1,})(?=(.*[@#$%!?._-]){1,})(?:[\da-zA-Z@#$%!?._-]){8,25}$"#).unwrap();
}