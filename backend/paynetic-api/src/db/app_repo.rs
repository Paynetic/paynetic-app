use std::sync::Arc;

use lib_api::{
    db::{db_error::DbError, db_setup::app_db_connect},
    error::api_error::ApiError,
};
use sqlx::{PgPool, Postgres, Transaction};

use super::{
    product_repo::{DynProductRepo, ProductRepo},
    user_repo::{DynUserRepo, UserRepo},
};

#[derive(Clone)]
pub struct AppRepo {
    pub db: PgPool,
    pub user: DynUserRepo,
    pub product: DynProductRepo,
}

impl AppRepo {
    pub async fn new(db_url: &str, db_name: &str) -> Result<Self, DbError> {
        let db_url = format!("{}{}", db_url, db_name);
        let db = app_db_connect(&db_url, db_name).await?;

        Ok(AppRepo {
            db: db.clone(),
            user: Arc::new(UserRepo { db: db.clone() }) as DynUserRepo,
            product: Arc::new(ProductRepo { db: db.clone() }) as DynProductRepo,
        })
    }

    pub async fn start_transaction(&self) -> Result<Transaction<'_, Postgres>, ApiError> {
        let transaction = self
            .db
            .begin()
            .await
            .map_err(|e| ApiError::internal_error().message(e))?;
        Ok(transaction)
    }
}
