use axum::extract::State;
use axum::http::StatusCode;
use axum::{Extension, Json};
use axum_macros::debug_handler;
use lib_api::db::db_error::DbError;
use lib_api::db::util::commit_or_rollback;
use lib_api::error::api_error::ApiError;
use lib_api::error::helpers::check_bad_form;
use lib_api::util::json_extractor::PnJson;
use lib_types::dto::product::create_product_dto::{CreateProductDto, CreateProductResponse};
use lib_types::entity::product_entity::ProductEntity;
use lib_types::shared::api_error::ApiErrorCode;
use lib_types::shared::product::{BlockchainStatus, ProductStatus, ACTIVE_SUBSCRIPTION_CONTRACT};
use lib_types::shared::user::RequestUser;
use validator::Validate;

use crate::api_context::ApiContext;
use crate::app::helpers::get_request_user;
use crate::db::product_repo::ProductCreateProps;
use crate::util::product::to_price_create_props;

fn to_api_response(result: ProductEntity) -> Json<CreateProductResponse> {
    return Json(CreateProductResponse { id: result.id });
}

#[debug_handler]
pub async fn create_product(
    State(context): State<ApiContext>,
    Extension(request_user): Extension<RequestUser>,
    PnJson(dto): PnJson<CreateProductDto>,
) -> Result<(StatusCode, Json<CreateProductResponse>), ApiError> {
    check_bad_form(dto.validate())?;
    let user = get_request_user(&context, &request_user).await?;

    let props = ProductCreateProps {
        user_id: user.id,
        name: dto.name,
        description: dto.description,
        blurb: dto.blurb,
        contract_address: ACTIVE_SUBSCRIPTION_CONTRACT.into(),
        category: dto.category,
        payout_address: user.eth_address,
        status: ProductStatus::Initial,
        price: to_price_create_props(dto.price)?,
        blockchain_status: BlockchainStatus::None,
    };
    let mut tx = context.repo.start_transaction().await?;

    let product_result = context
        .repo
        .product
        .create_product(&mut tx, props)
        .await
        .map_err(|e| match e {
            DbError::Unique(_) => ApiError::bad_request()
                .code(ApiErrorCode::ProductExists)
                .message("Product with name already exists"),
            _ => ApiError::internal_error().message(format!("Failed to create product: {}", e)),
        });

    let product = commit_or_rollback(tx, product_result).await?;

    Ok((StatusCode::CREATED, to_api_response(product)))
}
