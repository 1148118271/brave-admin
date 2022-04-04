use serde::Deserialize;
use serde::Serialize;


#[derive(Deserialize, Serialize, Debug)]
pub struct User {
    pub username: Option<String>,
    pub password: Option<String>
}