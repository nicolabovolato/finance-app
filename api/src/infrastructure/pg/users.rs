use async_trait::async_trait;
use sqlx::postgres::PgPool;
use uuid::Uuid;

use crate::application::services::users::UserService;
use crate::application::services::Repository;
use crate::domain::entities::users::User;
use crate::domain::error::Result;

pub struct PgUserService {
    db: PgPool,
}

impl PgUserService {
    pub fn new(db: PgPool) -> Self {
        Self { db }
    }
}

#[async_trait]
impl UserService for PgUserService {
    async fn find_by_email(&self, email: &str) -> Result<User> {
        let data = sqlx::query_as!(User, "SELECT * FROM users WHERE email = $1", email)
            .fetch_one(&self.db)
            .await?;
        Ok(data)
    }
}

#[async_trait]
impl Repository<User> for PgUserService {
    async fn get_all(&self) -> Result<Vec<User>> {
        let data = sqlx::query_as!(User, r#"SELECT * FROM users"#)
            .fetch_all(&self.db)
            .await?;
        Ok(data)
    }

    async fn find_by_id(&self, id: Uuid) -> Result<User> {
        let data = sqlx::query_as!(User, r#"SELECT * FROM users WHERE id = $1"#, id)
            .fetch_one(&self.db)
            .await?;
        Ok(data)
    }

    async fn insert(&self, item: User) -> Result<User> {
        let data = sqlx::query_as!(
            User,
            r#"INSERT INTO users(id, email)
            VALUES($1, $2)
            RETURNING *"#,
            item.id,
            item.email
        )
        .fetch_one(&self.db)
        .await?;
        Ok(data)
    }

    async fn update(&self, item: User) -> Result<User> {
        let data = sqlx::query_as!(
            User,
            r#"UPDATE users
            SET email=$2
            WHERE id=$1
            RETURNING *"#,
            item.id,
            item.email
        )
        .fetch_one(&self.db)
        .await?;
        Ok(data)
    }

    async fn delete(&self, item: User) -> Result<User> {
        let data = sqlx::query_as!(User, "DELETE FROM users WHERE id=$1 RETURNING *", item.id)
            .fetch_one(&self.db)
            .await?;
        Ok(data)
    }
}

#[cfg(test)]
mod integration_tests {
    use sqlx::{Pool, Postgres};

    use super::*;

    #[sqlx::test]
    async fn get_all(pool: Pool<Postgres>) {
        let service = PgUserService::new(pool);
        let user = service
            .insert(User {
                id: Uuid::new_v4(),
                email: "".to_string(),
            })
            .await
            .unwrap();
        assert_eq!(service.get_all().await.unwrap(), vec![user]);
    }

    #[sqlx::test]
    async fn find_by_id(pool: Pool<Postgres>) {
        let service = PgUserService::new(pool);
        let id = Uuid::new_v4();
        let user = service
            .insert(User {
                id,
                email: "".to_string(),
            })
            .await
            .unwrap();
        assert_eq!(service.find_by_id(id).await.unwrap(), user);
    }

    #[sqlx::test]
    #[should_panic(expected = "Repository(NotFound)")]
    async fn find_by_id_not_found(pool: Pool<Postgres>) {
        let service = PgUserService::new(pool);
        service.find_by_id(Uuid::new_v4()).await.unwrap();
    }

    #[sqlx::test]
    async fn find_by_email(pool: Pool<Postgres>) {
        let service = PgUserService::new(pool);
        let id = Uuid::new_v4();
        let user = service
            .insert(User {
                id,
                email: "email".to_string(),
            })
            .await
            .unwrap();
        assert_eq!(service.find_by_email("email").await.unwrap(), user);
    }

    #[sqlx::test]
    #[should_panic(expected = "Repository(NotFound)")]
    async fn find_by_email_not_found(pool: Pool<Postgres>) {
        let service = PgUserService::new(pool);
        service.find_by_email("email").await.unwrap();
    }

    #[sqlx::test]
    async fn insert(pool: Pool<Postgres>) {
        let service = PgUserService::new(pool);
        let id = Uuid::new_v4();
        assert_eq!(
            service
                .insert(User {
                    id,
                    email: "".to_string(),
                })
                .await
                .unwrap(),
            User {
                id,
                email: "".to_string()
            }
        );
    }

    #[sqlx::test]
    #[should_panic(expected = "Repository(Conflict)")]
    async fn insert_conflict(pool: Pool<Postgres>) {
        let service = PgUserService::new(pool);
        let id = Uuid::new_v4();
        service
            .insert(User {
                id,
                email: "".to_string(),
            })
            .await
            .unwrap();
        service
            .insert(User {
                id,
                email: "email".to_string(),
            })
            .await
            .unwrap();
    }

    #[sqlx::test]
    async fn update(pool: Pool<Postgres>) {
        let service = PgUserService::new(pool);
        let id = Uuid::new_v4();
        service
            .insert(User {
                id,
                email: "".to_string(),
            })
            .await
            .unwrap();
        assert_eq!(
            service
                .update(User {
                    id,
                    email: "email".to_string(),
                })
                .await
                .unwrap(),
            User {
                id,
                email: "email".to_string()
            }
        );
    }

    #[sqlx::test]
    #[should_panic(expected = "Repository(NotFound)")]
    async fn update_not_found(pool: Pool<Postgres>) {
        let service = PgUserService::new(pool);
        service
            .update(User {
                id: Uuid::new_v4(),
                email: "".to_string(),
            })
            .await
            .unwrap();
    }

    #[sqlx::test]
    async fn delete(pool: Pool<Postgres>) {
        let service = PgUserService::new(pool);
        let id = Uuid::new_v4();
        service
            .insert(User {
                id,
                email: "".to_string(),
            })
            .await
            .unwrap();
        assert_eq!(
            service
                .delete(User {
                    id,
                    email: "".to_string(),
                })
                .await
                .unwrap(),
            User {
                id,
                email: "".to_string()
            }
        );
    }

    #[sqlx::test]
    #[should_panic(expected = "Repository(NotFound)")]
    async fn delete_not_found(pool: Pool<Postgres>) {
        let service = PgUserService::new(pool);
        service
            .delete(User {
                id: Uuid::new_v4(),
                email: "".to_string(),
            })
            .await
            .unwrap();
    }
}
