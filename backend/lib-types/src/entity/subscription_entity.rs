use bigdecimal::BigDecimal;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::shared::{
    product::{BlockchainStatus, PaymentCurrency, PriceType, SubscriptionInterval},
    subscription::SubscriptionStatus,
};

#[derive(Debug, Deserialize, Serialize, Clone, sqlx::Type)]
pub struct SubscriptionEntity {
    pub id: Uuid,
    pub user_id: Uuid,
    pub product_id: Uuid,
    pub contract_address: String,
    pub status: SubscriptionStatus,
    pub prices: Vec<PriceEntityRelation>,
    pub fee_percent: i32,
    pub start_time: i64,
    pub current_start: i64,
    pub current_end: i64,
    pub grace: i64,
    pub blockchain_status: BlockchainStatus,
    pub transaction_hash: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize, Serialize, Clone, sqlx::Type)]
pub struct PriceEntityRelation {
    pub id: Uuid,
    pub active: bool,
    pub price_type: PriceType,
    pub amount: BigDecimal,
    pub base_currency: PaymentCurrency,
    pub subscription_interval: Option<SubscriptionInterval>,
    pub subscription_interval_count: Option<i32>,
    pub trial_days: Option<i32>,
}

#[derive(Debug)]
pub struct SubscriptionListResults {
    pub total: i64,
    pub results: Vec<SubscriptionEntity>,
}

#[derive(Serialize)]
pub struct SubscriptionCreateResult {
    pub id: Uuid,
}
