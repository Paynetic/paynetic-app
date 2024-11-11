use chrono::{DateTime, Utc};
use serde::Serialize;
use uuid::Uuid;

use crate::{
    dto::product::{get_product_dto::serialize_big, price_view_model::PriceViewModel},
    entity::subscription_entity::SubscriptionEntity,
    shared::{product::BlockchainStatus, subscription::SubscriptionStatus},
};

#[derive(Serialize)]
pub struct SubscriptionViewModel {
    pub id: Uuid,
    pub user_id: Uuid,
    pub product_id: Uuid,
    pub contract_address: String,
    pub status: SubscriptionStatus,
    pub prices: Vec<PriceViewModel>,
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

pub fn to_api_response(entity: SubscriptionEntity) -> SubscriptionViewModel {
    return SubscriptionViewModel {
        id: entity.id,
        user_id: entity.user_id,
        product_id: entity.product_id,
        contract_address: entity.contract_address,
        status: entity.status,
        blockchain_status: entity.blockchain_status,
        transaction_hash: entity.transaction_hash,
        prices: entity
            .prices
            .into_iter()
            .map(|pr| PriceViewModel {
                id: pr.id,
                active: pr.active,
                name: "".into(),
                price_type: pr.price_type,
                amount: serialize_big(&pr.amount),
                base_currency: pr.base_currency,
                subscription_interval: pr.subscription_interval,
                subscription_interval_count: pr.subscription_interval_count,
                trial_days: pr.trial_days,
            })
            .collect(),
        fee_percent: entity.fee_percent,
        start_time: entity.start_time,
        current_start: entity.current_start,
        current_end: entity.current_end,
        grace: entity.grace,
        created_at: entity.created_at,
        updated_at: entity.updated_at,
    };
}
