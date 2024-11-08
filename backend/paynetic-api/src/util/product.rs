use lib_api::error::api_error::ApiError;
use lib_types::{
    dto::product::{create_price_dto::CreatePriceDto, update_price_dto::UpdatePriceDto},
    shared::product::PaymentCurrency,
};

use crate::{
    app::helpers::{str_opt_to_bigdecimal, str_to_bigdecimal},
    db::product_repo::{PriceCreateProps, PriceUpdateProps},
};

pub fn to_price_update_props(
    dto: Option<UpdatePriceDto>,
) -> Result<Option<PriceUpdateProps>, ApiError> {
    if let Some(price_dto) = dto {
        Ok(Some(PriceUpdateProps {
            id: price_dto.id,
            active: price_dto.active,
            name: price_dto.name,
            price_type: price_dto.price_type,
            amount: str_opt_to_bigdecimal(price_dto.amount, "amount")?,
            subscription_interval: price_dto.subscription_interval,
            subscription_interval_count: price_dto.subscription_interval_count,
            trial_days: price_dto.trial_days,
        }))
    } else {
        Ok(None)
    }
}

pub fn to_price_create_props(
    dto: Option<CreatePriceDto>,
) -> Result<Option<PriceCreateProps>, ApiError> {
    if let Some(price_dto) = dto {
        Ok(Some(PriceCreateProps {
            name: price_dto.name,
            active: true,
            price_type: price_dto.price_type,
            amount: str_to_bigdecimal(&price_dto.amount, "amount")?,
            base_currency: PaymentCurrency::Ethereum,
            subscription_interval: price_dto.subscription_interval,
            subscription_interval_count: price_dto.subscription_interval_count,
            trial_days: price_dto.trial_days,
        }))
    } else {
        Ok(None)
    }
}
