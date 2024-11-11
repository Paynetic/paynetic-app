use axum::{
    extract::{Path, State},
    Extension, Json,
};
use lib_api::error::api_error::ApiError;
use lib_types::{
    dto::subscription::subscription_view_model::{to_api_response, SubscriptionViewModel},
    shared::user::RequestUser,
};
use uuid::Uuid;

use crate::{
    api_context::ApiContext,
    app::helpers::{not_found_or_internal, verify_admin_or_user},
};

pub async fn get_subscription(
    Path(id): Path<Uuid>,
    State(context): State<ApiContext>,
    Extension(request_user): Extension<RequestUser>,
) -> Result<Json<SubscriptionViewModel>, ApiError> {
    println!("GETSUBBY {}", id);
    // Get subscription from DB
    let subscription = context
        .repo
        .subscription
        .get_subscription_by_id(id)
        .await
        .map_err(not_found_or_internal)?;

    verify_admin_or_user(&request_user, subscription.user_id.to_string())?;

    Ok(Json(to_api_response(subscription)))
}
