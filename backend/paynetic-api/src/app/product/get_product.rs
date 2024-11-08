use axum::{
    extract::{Path, State},
    Extension, Json,
};
use lib_api::error::api_error::ApiError;
use lib_types::{
    dto::product::get_product_dto::{to_api_response, GetProductResponse},
    shared::{
        product::ProductStatus,
        user::{RequestUser, UserType},
    },
};
use uuid::Uuid;

use crate::{api_context::ApiContext, app::helpers::not_found_or_internal};

pub async fn get_product(
    Path(id): Path<Uuid>,
    State(context): State<ApiContext>,
    Extension(request_user): Extension<RequestUser>,
) -> Result<Json<GetProductResponse>, ApiError> {
    // Get product from DB
    let product = context
        .repo
        .product
        .get_product_by_id(id)
        .await
        .map_err(not_found_or_internal)?;

    // Verify user or admin, if the product is not published
    let visible = matches!(
        product.status,
        ProductStatus::Active | ProductStatus::Cancelled | ProductStatus::Unavailable
    );
    if !visible && request_user.user_type != UserType::Admin {
        if let Some(request_user_id) = request_user.user_id {
            if request_user_id != product.user_id {
                return Err(ApiError::forbidden());
            }
        } else {
            return Err(ApiError::forbidden());
        }
    }

    Ok(Json(to_api_response(product)))
}
