use async_trait::async_trait;
use chrono::Duration;
use serde::Serialize;

use crate::domain::error::Result;

pub mod accounts;
pub mod mail;
pub mod otp;
pub mod tokens;
pub mod users;

#[async_trait]
pub trait KVStore<T: Serialize> {
    async fn get(&self, key: &str, delete: bool) -> Result<Option<T>>;
    async fn set(&self, key: &str, value: T, expiration: Option<Duration>) -> Result<T>;
}

#[async_trait]
pub trait Repository<T: Serialize> {
    async fn get_all(&self) -> Result<Vec<T>>;
    async fn find_by_id(&self, id: uuid::Uuid) -> Result<T>;
    async fn insert(&self, item: T) -> Result<T>;
    async fn update(&self, item: T) -> Result<T>;
    async fn delete(&self, item: T) -> Result<T>;
}
