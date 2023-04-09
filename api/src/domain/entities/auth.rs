use uuid::Uuid;

#[derive(Debug, PartialEq, Clone)]
pub struct Claims {
    pub sub: Uuid,
}
