use serde::Deserialize;
use validator::Validate;

use crate::{
    shared::product::{ProductCategory, ProductStatus},
    type_util::REGEX_ETH_ADDRESS,
};

use super::update_price_dto::UpdatePriceDto;

#[derive(Deserialize, Validate)]
#[serde(deny_unknown_fields)]
pub struct UpdateProductDto {
    #[validate(length(min = 3, max = 50))]
    pub name: Option<String>,
    #[validate(length(min = 10, max = 10000))]
    pub description: Option<String>,
    #[validate(length(min = 5, max = 200))]
    pub blurb: Option<String>,
    #[validate(regex(path = "*REGEX_ETH_ADDRESS"))]
    pub payout_address: Option<String>,
    pub category: Option<ProductCategory>,
    pub status: Option<ProductStatus>,
    #[validate(nested)]
    pub price: Option<UpdatePriceDto>,
}
