use async_trait::async_trait;

use super::Repository;
use crate::domain::entities::users::User;
use crate::domain::error::Result;

#[async_trait]
pub trait UserService: Repository<User> + Send + Sync {
    async fn find_by_email(&self, email: &str) -> Result<User>;
}

#[cfg(test)]
use mockall::*;
#[cfg(test)]
mock! {
    pub UserService {}
    #[async_trait]
    impl Repository<User> for UserService {
        async fn get_all(&self) -> Result<Vec<User>>;
        async fn find_by_id(&self, id: uuid::Uuid) -> Result<User>;
        async fn insert(&self, item: User) -> Result<User>;
        async fn update(&self, item: User) -> Result<User>;
        async fn delete(&self, item: User) -> Result<User>;
    }
    #[async_trait]
    impl UserService for UserService {
        async fn find_by_email(&self, email: &str) -> Result<User>;
    }
}
