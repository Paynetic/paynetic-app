use lib_api::db::db_error::DbError;
use sqlx::PgPool;

pub mod s010_users;
pub mod s020_products;
pub mod s025_prices;

pub async fn seed_all(db: &PgPool) -> Result<(), DbError> {
    s010_users::seed(db).await?;
    s020_products::seed(db).await?;
    s025_prices::seed(db).await?;
    Ok(())
}
