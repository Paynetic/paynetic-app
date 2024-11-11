use axum::{extract::State, Extension, Json};
use lib_api::{
    db::util::option_string_to_uuid,
    error::{api_error::ApiError, helpers::check_bad_form},
};
use lib_types::{
    dto::subscription::{
        list_subscriptions_dto::{ListSubscriptionsQuery, ListSubscriptionsResponse},
        subscription_view_model::{to_api_response, SubscriptionViewModel},
    },
    shared::user::{RequestUser, UserType},
};
use validator::Validate;

use crate::{api_context::ApiContext, app::Qs};

pub async fn list_subscriptions(
    State(context): State<ApiContext>,
    Qs(query): Qs<ListSubscriptionsQuery>,
    Extension(request_user): Extension<RequestUser>,
) -> Result<Json<ListSubscriptionsResponse>, ApiError> {
    check_bad_form(query.validate())?;

    if request_user.user_type != UserType::Admin && request_user.user_type != UserType::Cron {
        let user_id = request_user.user_id;
        if user_id.is_none() || user_id != option_string_to_uuid(query.user_id.clone()) {
            return Err(ApiError::forbidden().message("User must filter by own ID"));
        }
    }

    let subscriptions = context
        .repo
        .subscription
        .list_subscriptions(query)
        .await
        .map_err(|e| {
            ApiError::internal_error().message(format!("Failed to list subscriptions: {}", e))
        })?;

    let view_models: Vec<SubscriptionViewModel> = subscriptions
        .results
        .into_iter()
        .map(|subscriptions| to_api_response(subscriptions))
        .collect();

    Ok(Json(ListSubscriptionsResponse {
        total: subscriptions.total,
        results: view_models,
    }))
}
