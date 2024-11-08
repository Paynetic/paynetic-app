use serde::Deserialize;
use uuid::Uuid;
use validator::Validate;

use crate::{
    shared::product::{PriceType, SubscriptionInterval},
    type_util::REGEX_POSITIVE_NUMBER,
};

#[derive(Deserialize, Validate)]
#[serde(deny_unknown_fields)]
pub struct UpdatePriceDto {
    pub id: Uuid,
    pub active: Option<bool>,
    #[validate(length(min = 3, max = 30))]
    pub name: Option<String>,
    pub price_type: Option<PriceType>,
    #[validate(length(min = 0, max = 100), regex(path = "*REGEX_POSITIVE_NUMBER"))]
    pub amount: Option<String>,
    pub subscription_interval: Option<SubscriptionInterval>,
    pub subscription_interval_count: Option<i32>,
    pub trial_days: Option<i32>,
}
