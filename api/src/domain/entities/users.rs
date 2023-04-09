use serde::Serialize;

#[derive(Debug, Serialize, PartialEq, Clone)]
pub struct User {
    pub id: uuid::Uuid,
    pub email: String,
}
