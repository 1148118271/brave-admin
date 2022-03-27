use serde::Deserialize;
use serde::Serialize;


#[derive(Deserialize, Serialize, Debug)]
pub struct User {
    pub username: Option<String>,
    pub password: Option<String>
}

impl User {
    pub fn new() -> Self {
        User {
            username: None,
            password: None
        }
    }
}