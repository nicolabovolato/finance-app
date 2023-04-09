use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct Account {
    pub id: uuid::Uuid,
    pub user_id: uuid::Uuid,
    pub name: String,
    pub balance: Decimal,
    pub currency: CurrencyType,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct Movement {
    pub id: uuid::Uuid,
    pub account_id: uuid::Uuid,
    pub timestamp: DateTime<Utc>,
    pub title: String,
    pub category: CategoryType,
    pub amount: Decimal,
}

// What is worse, code duplication or non respecting layer segregation?
// Choose your poison, I chose mine! (given the fact that these enums would be quite big in a real case scenario)

#[derive(Deserialize, Serialize, sqlx::Type, PartialEq, Debug, Clone)]
#[serde(rename_all = "UPPERCASE")]
#[sqlx(type_name = "varchar", rename_all = "UPPERCASE")]
pub enum CurrencyType {
    Usd,
    Eur,
}

#[derive(Deserialize, Serialize, sqlx::Type, PartialEq, Debug, Clone)]
#[serde(rename_all = "UPPERCASE")]
#[sqlx(type_name = "varchar", rename_all = "UPPERCASE")]
pub enum CategoryType {
    Generic,
    Bills,
    Shopping,
    Income,
    Insurance,
}
