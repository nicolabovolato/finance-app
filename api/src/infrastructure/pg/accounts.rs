use async_trait::async_trait;
use sqlx::postgres::PgPool;
use uuid::{self, Uuid};

use crate::application::services::accounts::AccountService;
use crate::application::services::Repository;
use crate::domain::entities::accounts::{Account, Movement};
use crate::domain::error::Result;

pub struct PgAccountService {
    db: PgPool,
}

impl PgAccountService {
    pub fn new(db: PgPool) -> Self {
        Self { db }
    }
}

#[async_trait]
impl AccountService for PgAccountService {
    async fn find_by_id_and_user_id(&self, id: Uuid, user_id: Uuid) -> Result<Account> {
        let data = sqlx::query_as!(
            Account,
            r#"SELECT id, user_id, name, balance, currency as "currency: _" FROM accounts WHERE id = $1 AND user_id = $2"#,
            id,
            user_id,
        )
        .fetch_one(&self.db)
        .await?;
        Ok(data)
    }

    async fn find_many_by_user_id(&self, user_id: Uuid) -> Result<Vec<Account>> {
        let data = sqlx::query_as!(
            Account,
            r#"SELECT id, user_id, name, balance, currency as "currency: _" FROM accounts WHERE user_id = $1 ORDER BY name DESC"#,
            user_id,
        )
        .fetch_all(&self.db)
        .await?;
        Ok(data)
    }

    async fn find_movements(&self, account_id: Uuid) -> Result<Vec<Movement>> {
        let data = sqlx::query_as!(
            Movement,
            r#"SELECT id, account_id, timestamp, title, amount, category as "category: _" 
            FROM movements
            WHERE account_id = $1
            ORDER BY timestamp DESC"#,
            account_id
        )
        .fetch_all(&self.db)
        .await?;
        Ok(data)
    }
    async fn insert_movement(&self, movement: Movement) -> Result<Movement> {
        let mut tx = self.db.begin().await?;

        let data = sqlx::query_as!(
            Movement,
            r#"INSERT INTO movements(id, account_id, timestamp, title, amount, category) 
            VALUES ($1, $2, $3, $4, $5, $6)
            RETURNING id, account_id, timestamp, title, amount, category as "category: _""#,
            movement.id,
            movement.account_id,
            movement.timestamp,
            movement.title,
            movement.amount,
            movement.category as _
        )
        .fetch_one(&mut tx)
        .await?;

        sqlx::query!(
            "UPDATE accounts SET balance = balance + $2 WHERE id = $1",
            movement.account_id,
            movement.amount
        )
        .execute(&mut tx)
        .await?;

        tx.commit().await?;
        Ok(data)
    }
}

#[async_trait]
impl Repository<Account> for PgAccountService {
    async fn get_all(&self) -> Result<Vec<Account>> {
        let data = sqlx::query_as!(
            Account,
            r#"SELECT id, user_id, name, balance, currency as "currency: _" FROM accounts ORDER BY name ASC"#
        )
        .fetch_all(&self.db)
        .await?;
        Ok(data)
    }

    async fn find_by_id(&self, id: Uuid) -> Result<Account> {
        let data = sqlx::query_as!(
            Account,
            r#"SELECT id, user_id, name, balance, currency as "currency: _" FROM accounts WHERE id = $1"#,
            id
        )
        .fetch_one(&self.db)
        .await?;
        Ok(data)
    }

    async fn insert(&self, item: Account) -> Result<Account> {
        let data = sqlx::query_as!(
            Account,
            r#"INSERT INTO accounts(id, user_id, name, balance, currency)
            VALUES($1, $2, $3, $4, $5)
            RETURNING id, user_id, name, balance, currency as "currency: _""#,
            item.id,
            item.user_id,
            item.name,
            item.balance,
            item.currency as _
        )
        .fetch_one(&self.db)
        .await?;
        Ok(data)
    }

    async fn update(&self, item: Account) -> Result<Account> {
        let data = sqlx::query_as!(
            Account,
            r#"UPDATE accounts
            SET user_id=$2, name=$3, balance=$4, currency=$5
            WHERE id=$1
            RETURNING id, user_id, name, balance, currency as "currency: _""#,
            item.id,
            item.user_id,
            item.name,
            item.balance,
            item.currency as _
        )
        .fetch_one(&self.db)
        .await?;
        Ok(data)
    }

    async fn delete(&self, item: Account) -> Result<Account> {
        let data = sqlx::query_as!(
            Account,
            r#"DELETE FROM accounts WHERE id=$1 RETURNING id, user_id, name, balance, currency as "currency: _""#,
            item.id
        )
        .fetch_one(&self.db)
        .await?;
        Ok(data)
    }
}

#[cfg(test)]
mod integration_tests {
    use chrono::Utc;
    use rust_decimal::Decimal;
    use sqlx::{Pool, Postgres};

    use super::*;
    use crate::{
        domain::entities::{
            accounts::{CategoryType, CurrencyType},
            users::User,
        },
        infrastructure::pg::users::PgUserService,
    };

    async fn insert_user(pool: Pool<Postgres>) -> User {
        let user_service = PgUserService::new(pool);
        user_service
            .insert(User {
                id: Uuid::new_v4(),
                email: "".to_string(),
            })
            .await
            .unwrap()
    }

    #[sqlx::test]
    async fn get_all(pool: Pool<Postgres>) {
        let service = PgAccountService::new(pool.clone());
        let user = insert_user(pool).await;
        let account = service
            .insert(Account {
                id: Uuid::new_v4(),
                user_id: user.id,
                name: "".to_string(),
                balance: Decimal::from(0),
                currency: CurrencyType::Usd,
            })
            .await
            .unwrap();
        assert_eq!(service.get_all().await.unwrap(), vec![account]);
    }

    #[sqlx::test]
    async fn find_by_id(pool: Pool<Postgres>) {
        let service = PgAccountService::new(pool.clone());
        let id = Uuid::new_v4();
        let user = insert_user(pool).await;
        let account = service
            .insert(Account {
                id,
                user_id: user.id,
                name: "".to_string(),
                balance: Decimal::from(0),
                currency: CurrencyType::Usd,
            })
            .await
            .unwrap();
        assert_eq!(service.find_by_id(id).await.unwrap(), account);
    }

    #[sqlx::test]
    #[should_panic(expected = "Repository(NotFound)")]
    async fn find_by_id_not_found(pool: Pool<Postgres>) {
        let service = PgAccountService::new(pool);
        service.find_by_id(Uuid::new_v4()).await.unwrap();
    }

    #[sqlx::test]
    async fn find_by_id_and_user_id(pool: Pool<Postgres>) {
        let service = PgAccountService::new(pool.clone());
        let id = Uuid::new_v4();
        let user = insert_user(pool).await;
        let account = service
            .insert(Account {
                id,
                user_id: user.id,
                name: "".to_string(),
                balance: Decimal::from(0),
                currency: CurrencyType::Usd,
            })
            .await
            .unwrap();
        assert_eq!(
            service.find_by_id_and_user_id(id, user.id).await.unwrap(),
            account
        );
    }

    #[sqlx::test]
    #[should_panic(expected = "Repository(NotFound)")]
    async fn find_by_id_and_user_id_not_found(pool: Pool<Postgres>) {
        let service = PgAccountService::new(pool);
        service
            .find_by_id_and_user_id(Uuid::new_v4(), Uuid::new_v4())
            .await
            .unwrap();
    }

    #[sqlx::test]
    async fn find_movements(pool: Pool<Postgres>) {
        let service = PgAccountService::new(pool.clone());
        let id = Uuid::new_v4();
        let movement_id = Uuid::new_v4();
        let user = insert_user(pool).await;
        let account = service
            .insert(Account {
                id,
                user_id: user.id,
                name: "".to_string(),
                balance: Decimal::from(0),
                currency: CurrencyType::Usd,
            })
            .await
            .unwrap();
        let movement = service
            .insert_movement(Movement {
                id: movement_id,
                account_id: id,
                amount: Decimal::from(0),
                category: CategoryType::Generic,
                timestamp: Utc::now(),
                title: "".to_string(),
            })
            .await
            .unwrap();
        assert_eq!(
            service.find_movements(account.id).await.unwrap(),
            vec![movement]
        );
    }

    #[sqlx::test]
    async fn insert(pool: Pool<Postgres>) {
        let service = PgAccountService::new(pool.clone());
        let id = Uuid::new_v4();
        let user = insert_user(pool).await;
        assert_eq!(
            service
                .insert(Account {
                    id,
                    user_id: user.id,
                    name: "".to_string(),
                    balance: Decimal::from(0),
                    currency: CurrencyType::Usd,
                })
                .await
                .unwrap(),
            Account {
                id,
                user_id: user.id,
                name: "".to_string(),
                balance: Decimal::from(0),
                currency: CurrencyType::Usd,
            }
        );
    }

    #[sqlx::test]
    #[should_panic(expected = "Repository(Conflict)")]
    async fn insert_conflict(pool: Pool<Postgres>) {
        let service = PgAccountService::new(pool.clone());
        let id = Uuid::new_v4();
        let user = insert_user(pool.clone()).await;
        let user2 = insert_user(pool).await;
        service
            .insert(Account {
                id,
                user_id: user.id,
                name: "".to_string(),
                balance: Decimal::from(0),
                currency: CurrencyType::Usd,
            })
            .await
            .unwrap();
        service
            .insert(Account {
                id,
                user_id: user2.id,
                name: "".to_string(),
                balance: Decimal::from(0),
                currency: CurrencyType::Usd,
            })
            .await
            .unwrap();
    }

    #[sqlx::test]
    async fn insert_movement(pool: Pool<Postgres>) {
        let service = PgAccountService::new(pool.clone());
        let movement_id = Uuid::new_v4();
        let user = insert_user(pool).await;
        let timestamp = Utc::now();
        let account = service
            .insert(Account {
                id: Uuid::new_v4(),
                user_id: user.id,
                name: "".to_string(),
                balance: Decimal::from(0),
                currency: CurrencyType::Usd,
            })
            .await
            .unwrap();
        let movement = service
            .insert_movement(Movement {
                id: movement_id,
                account_id: account.id,
                amount: Decimal::from(0),
                category: CategoryType::Generic,
                timestamp,
                title: "title".to_string(),
            })
            .await
            .unwrap();

        assert_eq!(movement.id, movement_id);
        assert_eq!(movement.account_id, account.id);
        assert_eq!(movement.amount, Decimal::from(0));
        assert_eq!(movement.category, CategoryType::Generic);
        assert_eq!(movement.title, "title".to_string());
    }

    #[sqlx::test]
    #[should_panic(expected = "Repository(Conflict)")]
    async fn insert_movement_conflict(pool: Pool<Postgres>) {
        let service = PgAccountService::new(pool.clone());
        let movement_id = Uuid::new_v4();
        let user = insert_user(pool).await;
        let account = service
            .insert(Account {
                id: Uuid::new_v4(),
                user_id: user.id,
                name: "".to_string(),
                balance: Decimal::from(0),
                currency: CurrencyType::Usd,
            })
            .await
            .unwrap();
        service
            .insert_movement(Movement {
                id: movement_id,
                account_id: account.id,
                amount: Decimal::from(0),
                category: CategoryType::Generic,
                timestamp: Utc::now(),
                title: "".to_string(),
            })
            .await
            .unwrap();
        service
            .insert_movement(Movement {
                id: movement_id,
                account_id: account.id,
                amount: Decimal::from(0),
                category: CategoryType::Generic,
                timestamp: Utc::now(),
                title: "".to_string(),
            })
            .await
            .unwrap();
    }

    #[sqlx::test]
    async fn update(pool: Pool<Postgres>) {
        let service = PgAccountService::new(pool.clone());
        let id = Uuid::new_v4();
        let user = insert_user(pool).await;
        service
            .insert(Account {
                id,
                user_id: user.id,
                name: "".to_string(),
                balance: Decimal::from(0),
                currency: CurrencyType::Usd,
            })
            .await
            .unwrap();
        assert_eq!(
            service
                .update(Account {
                    id,
                    user_id: user.id,
                    name: "name".to_string(),
                    balance: Decimal::from(1),
                    currency: CurrencyType::Eur,
                })
                .await
                .unwrap(),
            Account {
                id,
                user_id: user.id,
                name: "name".to_string(),
                balance: Decimal::from(1),
                currency: CurrencyType::Eur,
            }
        );
    }

    #[sqlx::test]
    #[should_panic(expected = "Repository(NotFound)")]
    async fn update_not_found(pool: Pool<Postgres>) {
        let service = PgAccountService::new(pool);
        service
            .update(Account {
                id: Uuid::new_v4(),
                user_id: Uuid::new_v4(),
                name: "".to_string(),
                balance: Decimal::from(0),
                currency: CurrencyType::Usd,
            })
            .await
            .unwrap();
    }

    #[sqlx::test]
    async fn delete(pool: Pool<Postgres>) {
        let service = PgAccountService::new(pool.clone());
        let id = Uuid::new_v4();
        let user = insert_user(pool).await;
        service
            .insert(Account {
                id,
                user_id: user.id,
                name: "".to_string(),
                balance: Decimal::from(0),
                currency: CurrencyType::Usd,
            })
            .await
            .unwrap();
        assert_eq!(
            service
                .delete(Account {
                    id,
                    user_id: user.id,
                    name: "".to_string(),
                    balance: Decimal::from(0),
                    currency: CurrencyType::Usd,
                })
                .await
                .unwrap(),
            Account {
                id,
                user_id: user.id,
                name: "".to_string(),
                balance: Decimal::from(0),
                currency: CurrencyType::Usd,
            }
        );
    }

    #[sqlx::test]
    #[should_panic(expected = "Repository(NotFound)")]
    async fn delete_not_found(pool: Pool<Postgres>) {
        let service = PgAccountService::new(pool);
        service
            .delete(Account {
                id: Uuid::new_v4(),
                user_id: Uuid::new_v4(),
                name: "".to_string(),
                balance: Decimal::from(0),
                currency: CurrencyType::Usd,
            })
            .await
            .unwrap();
    }
}
