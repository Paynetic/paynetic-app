use axum::{extract::State, Extension, Json};
use lib_api::{
    error::{api_error::ApiError, helpers::check_bad_form},
    util::conversion::str_opt_to_uuid,
};
use lib_types::{
    dto::product::{
        list_products_dto::{ListProductsQuery, ListProductsResponse},
        product_view_model::{to_api_response, ProductViewModel},
    },
    shared::{
        api_error::ApiErrorCode,
        product::ProductStatus,
        user::{RequestUser, UserType},
    },
};
use validator::Validate;

use crate::{api_context::ApiContext, app::Qs};

pub async fn list_products(
    State(context): State<ApiContext>,
    Qs(query): Qs<ListProductsQuery>,
    Extension(request_user): Extension<RequestUser>,
) -> Result<Json<ListProductsResponse>, ApiError> {
    check_bad_form(query.validate())?;

    let default_statuses = vec![
        ProductStatus::Active,
        ProductStatus::Unavailable,
        ProductStatus::Cancelled,
    ];

    let statuses = if request_user.user_type == UserType::Admin {
        query.statuses
    } else {
        // If the user filters by their own ID, filter by any status
        if query.user_id.is_some() && str_opt_to_uuid(&query.user_id) == request_user.user_id {
            query.statuses
        } else if let Some(statuses) = query.statuses.clone() {
            let restricted_status = statuses
                .iter()
                .find(|s| matches!(s, ProductStatus::Active | ProductStatus::Unavailable));
            if let Some(restricted) = restricted_status {
                return Err(ApiError::bad_request()
                    .code(ApiErrorCode::RestrictedStatus)
                    .message(format!("Cannot filter by status: {}", restricted)));
            } else if statuses.len() == 0 {
                Some(default_statuses)
            } else {
                query.statuses
            }
        } else {
            Some(default_statuses)
        }
    };
    let validated_query = ListProductsQuery {
        from: query.from,
        to: query.to,
        statuses,
        categories: query.categories,
        user_id: query.user_id,
        column: query.column,
        direction: query.direction,
    };

    let products = context
        .repo
        .product
        .list_products(validated_query)
        .await
        .map_err(|e| {
            ApiError::internal_error().message(format!("Failed to list products: {}", e))
        })?;

    let view_models: Vec<ProductViewModel> = products
        .results
        .into_iter()
        .map(|products| to_api_response(products))
        .collect();

    Ok(Json(ListProductsResponse {
        total: products.total,
        results: view_models,
    }))
}
