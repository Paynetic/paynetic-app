use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::{Extension, Json};
use lib_api::db::db_error::DbError;
use lib_api::db::util::commit_or_rollback;
use lib_api::error::api_error::ApiError;

use lib_api::error::helpers::check_bad_form;
use lib_api::util::json_extractor::PnJson;
use lib_types::dto::product::product_view_model::{to_api_response, ProductViewModel};
use lib_types::dto::product::update_product_dto::UpdateProductDto;
use lib_types::shared::api_error::ApiErrorCode;
use lib_types::shared::product::ProductStatus;
use lib_types::shared::user::{RequestUser, UserType};
use uuid::Uuid;
use validator::Validate;

use crate::api_context::ApiContext;

use crate::app::helpers::verify_admin_or_user;
use crate::db::product_repo::ProductUpdateProps;
use crate::util::product::to_price_update_props;

pub async fn update_product(
    Path(product_id): Path<Uuid>,
    State(context): State<ApiContext>,
    Extension(request_user): Extension<RequestUser>,
    PnJson(dto): PnJson<UpdateProductDto>,
) -> Result<(StatusCode, Json<ProductViewModel>), ApiError> {
    check_bad_form(dto.validate())?;

    // Get product to be updated
    let product_to_be_updated = context
        .repo
        .product
        .get_product_by_id(product_id)
        .await
        .map_err(|_| {
            ApiError::not_found().message(format!("Product with ID {} not found", product_id))
        })?;

    let is_admin = request_user.user_type == UserType::Admin;
    let is_active = matches!(
        product_to_be_updated.status,
        ProductStatus::Active | ProductStatus::Cancelled | ProductStatus::Denied
    );

    // Verify request
    verify_admin_or_user(&request_user, product_to_be_updated.user_id.to_string())?;

    if !is_admin {
        // Verify active product name can't change
        if let Some(_) = dto.name {
            if is_active {
                return Err(ApiError::bad_request().code(ApiErrorCode::ProductActive));
            }
        }
        // Only admin can update status
        if dto.status.is_some() {
            return Err(ApiError::forbidden());
        }
    }

    let price = to_price_update_props(dto.price)?;

    let props = ProductUpdateProps {
        name: dto.name,
        description: dto.description,
        blurb: dto.blurb,
        payout_address: dto.payout_address,
        category: dto.category,
        status: dto.status,
        blockchain_status: None,
        price,
    };
    let mut tx = context.repo.start_transaction().await?;

    // Update product
    let product_result = context
        .repo
        .product
        .update_product(&mut tx, product_id, props)
        .await
        .map_err(|e| match e {
            DbError::Unique(_) => ApiError::bad_request().code(ApiErrorCode::ProductExists),
            _ => ApiError::internal_error().message(format!("Failed to update product: {}", e)),
        });
    let product = commit_or_rollback(tx, product_result).await?;

    // Return response
    Ok((StatusCode::OK, Json(to_api_response(product))))
}
