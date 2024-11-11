use serde::Deserialize;
use validator::Validate;

use crate::shared::subscription::SubscriptionStatus;

#[derive(Deserialize, Validate)]
#[serde(deny_unknown_fields)]
pub struct UpdateSubscriptionDto {
    pub status: Option<SubscriptionStatus>,
}
