use serde::Deserialize;

#[derive(Deserialize)]
pub struct WebHook {
    event_type: String,
    pub user: User,
    pub object_attributes: Option<ObjectAttr>
}

#[derive(Deserialize)]
pub struct User {
    pub id: i64,
    pub username: String,
    pub name: String
}

#[derive(Deserialize)]
pub struct ObjectAttr {
    pub id: Option<i64>,
    pub target_branch: Option<String>,
    pub source_branch: Option<String>,
    pub state: Option<String>,
    pub title: Option<String>,
    pub description: Option<String>,
    pub url: Option<String>,
    pub action: Option<String>
}
