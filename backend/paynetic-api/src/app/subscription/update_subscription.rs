use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::Extension;
use lib_api::error::api_error::ApiError;

use lib_api::error::helpers::check_bad_form;
use lib_api::util::json_extractor::PnJson;
use lib_types::dto::subscription::update_subscription_dto::UpdateSubscriptionDto;
use lib_types::shared::subscription::SubscriptionStatus;
use lib_types::shared::user::{RequestUser, UserType};
use uuid::Uuid;
use validator::Validate;

use crate::api_context::ApiContext;

use crate::app::helpers::{not_found_or_internal, verify_admin_or_user};
use crate::db::subscription_repo::SubscriptionUpdateProps;

pub async fn update_subscription(
    Path(subscription_id): Path<Uuid>,
    State(context): State<ApiContext>,
    Extension(request_user): Extension<RequestUser>,
    PnJson(dto): PnJson<UpdateSubscriptionDto>,
) -> Result<(StatusCode, ()), ApiError> {
    check_bad_form(dto.validate())?;

    if request_user.user_type == UserType::User && dto.status != Some(SubscriptionStatus::Cancelled)
    {
        return Err(ApiError::forbidden().message("User can only cancel subscription"));
    }

    // Get subscription to be updated
    let subscription_to_be_updated = context
        .repo
        .subscription
        .get_subscription_by_id(subscription_id)
        .await
        .map_err(|_| {
            ApiError::not_found().message(format!(
                "Subscription with ID {} not found",
                subscription_id
            ))
        })?;

    // Verify user owns subscription
    verify_admin_or_user(
        &request_user,
        subscription_to_be_updated.user_id.to_string(),
    )?;

    let props = SubscriptionUpdateProps { status: dto.status };

    // Update subscription
    context
        .repo
        .subscription
        .update_subscription(subscription_id, props)
        .await
        .map_err(not_found_or_internal)?;

    // Return response
    Ok((StatusCode::OK, ()))
}
