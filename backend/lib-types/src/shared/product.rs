use serde::{Deserialize, Serialize};
use strum::{Display, EnumString};

pub static ACTIVE_SUBSCRIPTION_CONTRACT: &str = "0x0000000000000000000000000000000000000000";

#[derive(
    Debug, Serialize, Deserialize, Clone, Copy, PartialEq, Eq, EnumString, Display, sqlx::Type,
)]
pub enum ProductCategory {
    Technology,
    Digital,
    Fashion,
    Games,
    ArtDesign,
    Music,
    Misc,
}

#[derive(
    Debug, Serialize, Deserialize, Clone, Copy, PartialEq, Eq, EnumString, Display, sqlx::Type,
)]
pub enum PriceType {
    Subscription,
    Once,
}

#[derive(
    Debug, Serialize, Deserialize, Clone, Copy, PartialEq, Eq, EnumString, Display, sqlx::Type,
)]
pub enum SubscriptionInterval {
    Day,
    Week,
    Month,
    Year,
}

#[derive(
    Debug, Serialize, Deserialize, Clone, Copy, PartialEq, Eq, EnumString, Display, sqlx::Type,
)]
pub enum BlockchainStatus {
    None,
    Pending,
    Error,
    Success,
}

#[derive(
    Debug, Serialize, Deserialize, Clone, Copy, PartialEq, Eq, EnumString, Display, sqlx::Type,
)]
pub enum ProductStatus {
    Initial,
    Review,
    Approved,
    Denied,
    Active,
    Unavailable,
    Cancelled,
}

#[derive(
    Debug, Serialize, Deserialize, Clone, Copy, PartialEq, Eq, EnumString, Display, sqlx::Type,
)]
pub enum PaymentCurrency {
    Ethereum,
    Pnt,
}
