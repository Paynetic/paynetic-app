use serde::Serialize;
use uuid::Uuid;

use crate::{
    entity::price_entity::PriceEntityRelation,
    shared::product::{PaymentCurrency, PriceType, SubscriptionInterval},
};

use super::get_product_dto::serialize_big;

#[derive(Serialize)]
pub struct PriceViewModel {
    pub id: Uuid,
    pub active: bool,
    pub name: String,
    pub price_type: PriceType,
    pub amount: String,
    pub base_currency: PaymentCurrency,
    pub subscription_interval: Option<SubscriptionInterval>,
    pub subscription_interval_count: Option<i32>,
    pub trial_days: Option<i32>,
}

pub fn to_api_response(price_entity: PriceEntityRelation) -> PriceViewModel {
    return PriceViewModel {
        id: price_entity.id,
        active: price_entity.active,
        name: price_entity.name,
        price_type: price_entity.price_type,
        amount: serialize_big(&price_entity.amount),
        base_currency: price_entity.base_currency,
        subscription_interval: price_entity.subscription_interval,
        subscription_interval_count: price_entity.subscription_interval_count,
        trial_days: price_entity.trial_days,
    };
}
