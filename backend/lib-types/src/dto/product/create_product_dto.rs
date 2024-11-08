use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

use crate::shared::product::ProductCategory;

use super::create_price_dto::CreatePriceDto;

#[derive(Deserialize, Validate)]
#[serde(deny_unknown_fields)]
pub struct CreateProductDto {
    #[validate(length(min = 3, max = 60))]
    pub name: String,
    #[validate(length(min = 10, max = 10000))]
    pub description: String,
    #[validate(length(min = 5, max = 200))]
    pub blurb: String,
    pub category: ProductCategory,
    #[validate(nested)]
    pub price: Option<CreatePriceDto>,
}

#[derive(Serialize)]
pub struct CreateProductResponse {
    pub id: Uuid,
}
