use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::shared::product::{BlockchainStatus, ProductCategory, ProductStatus};

use super::price_entity::PriceEntityRelation;

#[derive(Debug, Deserialize, Serialize, sqlx::Type)]
pub struct ProductEntity {
    pub id: Uuid,
    pub user_id: Uuid,
    pub name: String,
    pub description: String,
    pub blurb: String,
    pub contract_address: String,
    pub payout_address: String,
    pub category: ProductCategory,
    pub status: ProductStatus,
    pub blockchain_status: BlockchainStatus,
    pub transaction_hash: Option<String>,
    pub price: Option<PriceEntityRelation>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug)]
pub struct ProductListResults {
    pub total: i64,
    pub results: Vec<ProductEntity>,
}

pub struct ProductCreateResult {
    pub id: Uuid,
}
