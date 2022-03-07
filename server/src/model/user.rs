use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Serialize, Deserialize)]
pub struct UserBody<T> {
    pub user: T,
}

#[derive(Debug, Validate, Deserialize)]
pub struct NewUser {
    #[validate(length(min = 3, max = 16))]
    username: String,
    #[validate(email)]
    email: String,
    #[validate(length(min = 6, max = 16))]
    password: String,
}