use crate::shared::product::{PriceType, SubscriptionInterval};
use serde::Deserialize;
use validator::Validate;

#[derive(Deserialize, Validate)]
#[serde(deny_unknown_fields)]
pub struct CreatePriceDto {
    #[validate(length(min = 3, max = 30))]
    pub name: String,
    pub price_type: PriceType,
    pub amount: String,
    pub subscription_interval: Option<SubscriptionInterval>,
    pub subscription_interval_count: Option<i32>,
    pub trial_days: Option<i32>,
}
