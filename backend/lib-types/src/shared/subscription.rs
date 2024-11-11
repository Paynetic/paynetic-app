use serde::{Deserialize, Serialize};
use strum::{Display, EnumString};

#[derive(
    Debug, Serialize, Deserialize, Clone, Copy, PartialEq, Eq, EnumString, Display, sqlx::Type,
)]
pub enum SubscriptionStatus {
    Initial,
    Pending,
    Trial,
    Active,
    PastDue,
    Cancelled,
    Paused,
}
