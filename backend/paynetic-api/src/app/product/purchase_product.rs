use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::{Extension, Json};
use axum_macros::debug_handler;
use chrono::{Duration, Months, Utc};
use lib_api::db::util::commit_or_rollback;
use lib_api::error::api_error::ApiError;
use lib_types::entity::subscription_entity::SubscriptionCreateResult;
use lib_types::shared::api_error::ApiErrorCode;
use lib_types::shared::product::{BlockchainStatus, ProductStatus, ACTIVE_SUBSCRIPTION_CONTRACT};
use lib_types::shared::subscription::SubscriptionStatus;
use lib_types::shared::user::RequestUser;
use uuid::Uuid;

use crate::api_context::ApiContext;
use crate::app::helpers::{get_request_user, not_found_or_internal};
use crate::db::subscription_repo::SubscriptionCreateProps;

async fn create_charge() -> Result<(), ApiError> {
    // TODO -- implement single purchase/charge
    Ok(())
}

#[debug_handler]
pub async fn purchase_product(
    Path(product_id): Path<Uuid>,
    State(context): State<ApiContext>,
    Extension(request_user): Extension<RequestUser>,
) -> Result<(StatusCode, Json<SubscriptionCreateResult>), ApiError> {
    let user = get_request_user(&context, &request_user).await?;

    let product = context
        .repo
        .product
        .get_product_by_id(product_id)
        .await
        .map_err(not_found_or_internal)?;
    let price = product.price.ok_or(
        ApiError::bad_request()
            .code(ApiErrorCode::ProductNoPrice)
            .message("Product cannot be purchased"),
    )?;
    if product.status != ProductStatus::Active {
        return Err(ApiError::bad_request()
            .code(ApiErrorCode::ProductInactive)
            .message("Product cannot be purchased"));
    }

    let now = Utc::now();
    let props = SubscriptionCreateProps {
        user_id: user.id,
        product_id,
        price_id: price.id,
        start_time: now.timestamp(),
        current_start: now.timestamp(),
        // TODO -- get from price interval
        // TODO -- get fallback from interval and add days according to month length
        current_end: now
            .checked_add_months(Months::new(1))
            .unwrap_or(now + Duration::days(31))
            .timestamp(),
        grace: Duration::days(5).num_seconds(),
        contract_address: ACTIVE_SUBSCRIPTION_CONTRACT.into(),
        status: SubscriptionStatus::Initial,
        blockchain_status: BlockchainStatus::None,
    };
    let mut tx = context.repo.start_transaction().await?;

    let subscription_result = context
        .repo
        .subscription
        .create_subscription(&mut tx, props)
        .await
        .map_err(not_found_or_internal);

    let subscription = commit_or_rollback(tx, subscription_result).await?;

    Ok((StatusCode::CREATED, Json(subscription)))
}
