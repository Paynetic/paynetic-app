use bigdecimal::BigDecimal;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::shared::product::{PaymentCurrency, PriceType, SubscriptionInterval};

#[derive(Debug, Deserialize, Serialize, sqlx::Type)]
pub struct PriceEntity {
    pub id: Uuid,
    pub product_id: Uuid,
    pub active: bool,
    pub name: String,
    pub price_type: PriceType,
    pub amount: BigDecimal,
    pub base_currency: PaymentCurrency,
    pub subscription_interval: Option<SubscriptionInterval>,
    pub subscription_interval_count: Option<i32>,
    pub trial_days: Option<i32>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize, Serialize, sqlx::Type)]
pub struct PriceEntityRelation {
    pub id: Uuid,
    pub active: bool,
    pub name: String,
    pub price_type: PriceType,
    pub amount: BigDecimal,
    pub base_currency: PaymentCurrency,
    pub subscription_interval: Option<SubscriptionInterval>,
    pub subscription_interval_count: Option<i32>,
    pub trial_days: Option<i32>,
}
