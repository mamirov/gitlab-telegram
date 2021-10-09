use serde::Deserialize;

#[derive(Deserialize)]
pub struct WebHook {
    event_type: String,
    pub user: User
}

#[derive(Deserialize)]
pub struct User {
    id: i64,
    username: String,
    pub name: String
}
