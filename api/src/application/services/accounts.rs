use async_trait::async_trait;
use uuid::Uuid;

use super::Repository;
use crate::{
    domain::entities::accounts::{Account, Movement},
    domain::error::Result,
};

#[async_trait]
pub trait AccountService: Repository<Account> + Send + Sync {
    async fn find_by_id_and_user_id(&self, id: Uuid, user_id: Uuid) -> Result<Account>;
    async fn find_many_by_user_id(&self, user_id: Uuid) -> Result<Vec<Account>>;
    async fn find_movements(&self, account_id: Uuid) -> Result<Vec<Movement>>;
    async fn insert_movement(&self, movement: Movement) -> Result<Movement>;
}

#[cfg(test)]
use mockall::*;
#[cfg(test)]
mock! {
    pub AccountService {}
    #[async_trait]
    impl Repository<Account> for AccountService {
        async fn get_all(&self) -> Result<Vec<Account>>;
        async fn find_by_id(&self, id: uuid::Uuid) -> Result<Account>;
        async fn insert(&self, item: Account) -> Result<Account>;
        async fn update(&self, item: Account) -> Result<Account>;
        async fn delete(&self, item: Account) -> Result<Account>;
    }
    #[async_trait]
    impl AccountService for AccountService {
        async fn find_by_id_and_user_id(&self, id: Uuid, user_id: Uuid) -> Result<Account>;
        async fn find_many_by_user_id(&self, user_id: Uuid) -> Result<Vec<Account>>;
        async fn find_movements(&self, account_id: Uuid) -> Result<Vec<Movement>>;
        async fn insert_movement(&self, movement: Movement) -> Result<Movement>;
    }
}
