use bigdecimal::BigDecimal;
use chrono::{DateTime, Utc};
use serde::Serialize;
use uuid::Uuid;

use crate::{
    entity::product_entity::ProductEntity,
    shared::product::{BlockchainStatus, ProductCategory, ProductStatus},
};

use super::price_view_model::{self, PriceViewModel};

#[derive(Serialize)]
pub struct GetProductResponse {
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
    pub price: Option<PriceViewModel>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

pub fn serialize_big(dec: &BigDecimal) -> String {
    dec.with_scale(0).to_string()
}

pub fn to_api_response(product_entity: ProductEntity) -> GetProductResponse {
    return GetProductResponse {
        id: product_entity.id,
        user_id: product_entity.user_id,
        name: product_entity.name,
        description: product_entity.description,
        blurb: product_entity.blurb,
        contract_address: product_entity.contract_address,
        payout_address: product_entity.payout_address,
        category: product_entity.category,
        status: product_entity.status,
        blockchain_status: product_entity.blockchain_status,
        transaction_hash: product_entity.transaction_hash,
        price: product_entity
            .price
            .and_then(|pr| Some(price_view_model::to_api_response(pr))),
        created_at: product_entity.created_at,
        updated_at: product_entity.updated_at,
    };
}
