use crate::{
    api_context::ApiContext,
    app::{auth, product, user},
    util::auth::{auth_admin, auth_admin_user, auth_admin_user_anonymous},
};
use axum::{
    handler::Handler,
    http::StatusCode,
    middleware::from_fn_with_state,
    response::IntoResponse,
    routing::{get, patch, post},
    Router,
};

use super::{health, subscription};

pub fn app_router(context: &ApiContext) -> Router<ApiContext> {
    Router::new().nest("/api", api_router(context))
}

pub fn api_router(context: &ApiContext) -> Router<ApiContext> {
    Router::new()
        .route("/healthz", get(health::get_app_health::get_app_health))
        .route(
            "/users",
            get(user::list_users::list_users)
                .route_layer(from_fn_with_state(context.clone(), auth_admin)),
        )
        .route(
            "/users/:user_id",
            get(user::get_user::get_user)
                .patch(user::update_user::update_user)
                .route_layer(from_fn_with_state(context.clone(), auth_admin_user)),
        )
        .route(
            "/users/registrations",
            post(user::register_user::register_user),
        )
        .route("/users/queries/exists", get(user::user_exists::user_exists))
        .route("/auth/logins", post(auth::login_user::login_user))
        .route(
            "/auth/logins/reset-password",
            post(auth::reset_password::reset_password),
        )
        .route(
            "/auth/logins/passwords",
            patch(auth::update_password::update_password)
                .route_layer(from_fn_with_state(context.clone(), auth_admin_user)),
        )
        .route(
            "/auth/confirm-email",
            post(auth::confirm_email::confirm_email),
        )
        .route(
            "/auth/resend-confirm-email",
            post(auth::resend_confirm_email::resend_confirm_email)
                .route_layer(from_fn_with_state(context.clone(), auth_admin_user)),
        )
        .route(
            "/products",
            post(
                product::create_product::create_product
                    .layer(from_fn_with_state(context.clone(), auth_admin_user)),
            )
            .get(
                product::list_products::list_products.layer(from_fn_with_state(
                    context.clone(),
                    auth_admin_user_anonymous,
                )),
            ),
        )
        .route(
            "/products/:product_id",
            get(product::get_product::get_product.layer(from_fn_with_state(
                context.clone(),
                auth_admin_user_anonymous,
            )))
            .patch(
                product::update_product::update_product
                    .layer(from_fn_with_state(context.clone(), auth_admin_user)),
            ),
        )
        .route(
            "/products/:product_id/purchase",
            post(
                product::purchase_product::purchase_product
                    .layer(from_fn_with_state(context.clone(), auth_admin_user)),
            ),
        )
        .route(
            "/subscriptions",
            get(subscription::list_subscriptions::list_subscriptions
                .layer(from_fn_with_state(context.clone(), auth_admin_user))),
        )
        .route(
            "/subscriptions/:subscription_id",
            get(subscription::get_subscription::get_subscription
                .layer(from_fn_with_state(context.clone(), auth_admin_user)))
            .patch(
                subscription::update_subscription::update_subscription
                    .layer(from_fn_with_state(context.clone(), auth_admin_user)),
            ),
        )
        .route("/*path", get(handler_404)) // Handle unknown routes under /api
}

async fn handler_404() -> impl IntoResponse {
    (StatusCode::NOT_FOUND, "Resource not found")
}
