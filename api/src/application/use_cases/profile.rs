use async_trait::async_trait;
use chrono::Utc;
use rust_decimal::Decimal;
use uuid::Uuid;

use crate::application::services::accounts::AccountService;
use crate::domain::entities::accounts::{Account, CategoryType, CurrencyType, Movement};
use crate::domain::error::Result;

#[async_trait]
pub trait ProfileUseCaseTrait: Send + Sync {
    async fn get_accounts(&self, user_id: Uuid) -> Result<Vec<Account>>;
    async fn get_account(&self, user_id: Uuid, account_id: Uuid) -> Result<Account>;
    async fn create_account(
        &self,
        user_id: Uuid,
        name: &str,
        currency: CurrencyType,
    ) -> Result<Account>;
    async fn get_movements(&self, user_id: Uuid, account_id: Uuid) -> Result<Vec<Movement>>;
    async fn create_movement(
        &self,
        user_id: Uuid,
        account_id: Uuid,
        title: &str,
        category: CategoryType,
        amount: Decimal,
    ) -> Result<Movement>;
}

pub struct ProfileUseCase {
    account_service: Box<dyn AccountService>,
}

impl ProfileUseCase {
    pub fn new(account_service: Box<dyn AccountService>) -> Self {
        Self { account_service }
    }
}

#[async_trait]
impl ProfileUseCaseTrait for ProfileUseCase {
    async fn get_accounts(&self, user_id: Uuid) -> Result<Vec<Account>> {
        let accounts = self.account_service.find_many_by_user_id(user_id).await?;
        Ok(accounts)
    }

    async fn get_account(&self, user_id: Uuid, account_id: Uuid) -> Result<Account> {
        let account = self
            .account_service
            .find_by_id_and_user_id(account_id, user_id)
            .await?;
        Ok(account)
    }

    async fn create_account(
        &self,
        user_id: Uuid,
        name: &str,
        currency: CurrencyType,
    ) -> Result<Account> {
        let account = self
            .account_service
            .insert(Account {
                id: Uuid::new_v4(),
                user_id,
                balance: Decimal::from(0),
                name: name.to_string(),
                currency,
            })
            .await?;
        Ok(account)
    }

    async fn get_movements(&self, user_id: Uuid, account_id: Uuid) -> Result<Vec<Movement>> {
        self.get_account(user_id, account_id).await?;
        let movements = self.account_service.find_movements(account_id).await?;
        Ok(movements)
    }

    async fn create_movement(
        &self,
        user_id: Uuid,
        account_id: Uuid,
        title: &str,
        category: CategoryType,
        amount: Decimal,
    ) -> Result<Movement> {
        self.get_account(user_id, account_id).await?;
        let movement = self
            .account_service
            .insert_movement(Movement {
                id: Uuid::new_v4(),
                account_id,
                timestamp: Utc::now(),
                title: title.to_string(),
                category,
                amount,
            })
            .await?;
        Ok(movement)
    }
}

#[cfg(test)]
use mockall::*;
#[cfg(test)]
mock! {
    pub ProfileUseCase {}
    #[async_trait]
    impl ProfileUseCaseTrait for ProfileUseCase {
        async fn get_accounts(&self, user_id: Uuid) -> Result<Vec<Account>>;
        async fn get_account(&self, user_id: Uuid, account_id: Uuid) -> Result<Account>;
        async fn create_account(
            &self,
            user_id: Uuid,
            name: &str,
            currency: CurrencyType,
        ) -> Result<Account>;
        async fn get_movements(&self, user_id: Uuid, account_id: Uuid) -> Result<Vec<Movement>>;
        async fn create_movement(
            &self,
            user_id: Uuid,
            account_id: Uuid,
            title: &str,
            category: CategoryType,
            amount: Decimal,
        ) -> Result<Movement>;
    }
}

#[cfg(test)]
mod tests {
    use mockall::predicate;
    use rust_decimal::Decimal;
    use tokio;

    use super::*;
    use crate::application::services::accounts::MockAccountService;

    fn get_mock_use_case(accounts_service: MockAccountService) -> ProfileUseCase {
        ProfileUseCase {
            account_service: Box::new(accounts_service),
        }
    }

    #[tokio::test]
    async fn get_accounts_successful() {
        let user_id = uuid::Uuid::new_v4();
        let accounts = vec![Account {
            balance: Decimal::from(0),
            currency: CurrencyType::Usd,
            id: uuid::Uuid::new_v4(),
            user_id,
            name: "name".to_string(),
        }];
        let accounts2 = accounts.clone();

        let mut account_service = MockAccountService::new();
        account_service
            .expect_find_many_by_user_id()
            .with(predicate::eq(user_id))
            .return_once(move |_| Ok(accounts));

        let use_case = get_mock_use_case(account_service);

        let result = use_case.get_accounts(user_id).await.unwrap();
        assert_eq!(result, accounts2);
    }

    #[tokio::test]
    async fn create_account_successful() {
        let user_id = uuid::Uuid::new_v4();
        let name = "name".to_string();
        let currency = CurrencyType::Usd;
        let account = Account {
            balance: Decimal::from(0),
            currency: currency.clone(),
            id: uuid::Uuid::new_v4(),
            user_id,
            name: name.clone(),
        };
        let account2 = account.clone();
        let account3 = account.clone();

        let mut account_service = MockAccountService::new();
        account_service
            .expect_insert()
            .withf(move |x: &Account| {
                x.name == account.name
                    && x.currency == account.currency
                    && x.user_id == account.user_id
            })
            .return_once(move |_| Ok(account2));

        let use_case = get_mock_use_case(account_service);

        let result = use_case
            .create_account(user_id, &name, currency)
            .await
            .unwrap();

        assert_eq!(result, account3);
    }

    #[tokio::test]
    async fn get_account_successful() {
        let user_id = uuid::Uuid::new_v4();
        let account_id = uuid::Uuid::new_v4();
        let account = Account {
            balance: Decimal::from(0),
            currency: CurrencyType::Usd,
            id: account_id,
            user_id,
            name: "name".to_string(),
        };
        let account2 = account.clone();

        let mut account_service = MockAccountService::new();
        account_service
            .expect_find_by_id_and_user_id()
            .with(predicate::eq(account_id), predicate::eq(user_id))
            .return_once(move |_, _| Ok(account2));

        let use_case = get_mock_use_case(account_service);

        let result = use_case.get_account(user_id, account_id).await.unwrap();

        assert_eq!(result, account);
    }

    #[tokio::test]
    async fn create_movement_successful() {
        let user_id = uuid::Uuid::new_v4();
        let account_id = uuid::Uuid::new_v4();
        let title = "title".to_string();
        let category = CategoryType::Generic;
        let amount = Decimal::from(0);
        let movement_id = uuid::Uuid::new_v4();
        let movement = Movement {
            account_id,
            id: movement_id,
            amount,
            category: category.clone(),
            timestamp: chrono::Utc::now(),
            title: title.clone(),
        };
        let movement2 = movement.clone();
        let movement3 = movement.clone();

        let mut account_service = MockAccountService::new();
        account_service
            .expect_find_by_id_and_user_id()
            .with(predicate::eq(account_id), predicate::eq(user_id))
            .return_once(move |_, _| {
                Ok(Account {
                    balance: Decimal::from(0),
                    currency: CurrencyType::Usd,
                    id: account_id,
                    user_id,
                    name: "name".to_string(),
                })
            });
        account_service
            .expect_insert_movement()
            .withf(move |x: &Movement| {
                x.title == movement.title
                    && x.amount == movement.amount
                    && x.account_id == movement.account_id
            })
            .return_once(move |_| Ok(movement2));

        let use_case = get_mock_use_case(account_service);

        let result = use_case
            .create_movement(user_id, account_id, &title, category, amount)
            .await
            .unwrap();

        assert_eq!(result, movement3);
    }

    #[tokio::test]
    async fn get_movements_successful() {
        let user_id = uuid::Uuid::new_v4();
        let account_id = uuid::Uuid::new_v4();
        let movements = vec![Movement {
            account_id,
            id: uuid::Uuid::new_v4(),
            amount: Decimal::from(0),
            category: CategoryType::Generic,
            timestamp: chrono::Utc::now(),
            title: "title".to_string(),
        }];
        let movements2 = movements.clone();

        let mut account_service = MockAccountService::new();
        account_service
            .expect_find_by_id_and_user_id()
            .with(predicate::eq(account_id), predicate::eq(user_id))
            .return_once(move |_, _| {
                Ok(Account {
                    balance: Decimal::from(0),
                    currency: CurrencyType::Usd,
                    id: account_id,
                    user_id,
                    name: "name".to_string(),
                })
            });
        account_service
            .expect_find_movements()
            .with(predicate::eq(account_id))
            .return_once(move |_| Ok(movements));

        let use_case = get_mock_use_case(account_service);

        let result = use_case.get_movements(user_id, account_id).await.unwrap();

        assert_eq!(result, movements2);
    }
}
