use std::sync::Arc;

use axum::async_trait;
use bigdecimal::BigDecimal;
use const_format::formatcp;
use lib_api::db::{
    db_error::{map_sqlx_err, DbError},
    db_result::list_result,
    util::{
        append_and_eq, append_comma, append_in, append_limit_offset, append_order_by,
        option_enum_to_string, option_string_to_uuid,
    },
};
use lib_types::{
    dto::{
        product::list_products_dto::{ListProductsQuery, ProductSortColumn},
        sort_direction::SortDirection,
    },
    entity::{
        price_entity::PriceEntityRelation,
        product_entity::{ProductEntity, ProductListResults},
    },
    shared::product::{
        BlockchainStatus, PaymentCurrency, PriceType, ProductCategory, ProductStatus,
        SubscriptionInterval,
    },
};
use serde::{Deserialize, Serialize};
use serde_json::to_string;
use sqlx::{postgres::PgRow, PgPool, Postgres, QueryBuilder, Row, Transaction};
use uuid::Uuid;

pub type DynProductRepo = Arc<dyn ProductRepoTrait + Send + Sync>;

#[derive(Debug, Deserialize, Serialize, sqlx::Type)]
pub struct ProductCreateProps {
    pub user_id: Uuid,
    pub name: String,
    pub description: String,
    pub blurb: String,
    pub contract_address: String,
    pub payout_address: String,
    pub category: ProductCategory,
    pub status: ProductStatus,
    pub blockchain_status: BlockchainStatus,
    pub price: Option<PriceCreateProps>,
}

#[derive(Debug, Deserialize, Serialize, sqlx::Type)]
pub struct PriceCreateProps {
    pub active: bool,
    pub name: String,
    pub price_type: PriceType,
    pub amount: BigDecimal,
    pub base_currency: PaymentCurrency,
    pub subscription_interval: Option<SubscriptionInterval>,
    pub subscription_interval_count: Option<i32>,
    pub trial_days: Option<i32>,
}

#[derive(Debug, Deserialize, Serialize, sqlx::Type)]
pub struct ProductUpdateProps {
    pub name: Option<String>,
    pub description: Option<String>,
    pub blurb: Option<String>,
    pub payout_address: Option<String>,
    pub category: Option<ProductCategory>,
    pub status: Option<ProductStatus>,
    pub blockchain_status: Option<BlockchainStatus>,
    pub price: Option<PriceUpdateProps>,
}

#[derive(Debug, Deserialize, Serialize, sqlx::Type)]
pub struct PriceUpdateProps {
    pub id: Uuid,
    pub active: Option<bool>,
    pub name: Option<String>,
    pub price_type: Option<PriceType>,
    pub amount: Option<BigDecimal>,
    pub subscription_interval: Option<SubscriptionInterval>,
    pub subscription_interval_count: Option<i32>,
    pub trial_days: Option<i32>,
}

#[async_trait]
pub trait ProductRepoTrait {
    fn get_db(&self) -> &PgPool;
    async fn create_product(
        &self,
        tx: &mut Transaction<'_, Postgres>,
        props: ProductCreateProps,
    ) -> Result<ProductEntity, DbError>;
    async fn create_price(
        &self,
        tx: &mut Transaction<'_, Postgres>,
        product_id: Uuid,
        props: PriceCreateProps,
    ) -> Result<PriceEntityRelation, DbError>;
    async fn update_product(
        &self,
        tx: &mut Transaction<'_, Postgres>,
        id: Uuid,
        props: ProductUpdateProps,
    ) -> Result<ProductEntity, DbError>;
    async fn update_price(
        &self,
        tx: &mut Transaction<'_, Postgres>,
        props: PriceUpdateProps,
    ) -> Result<PriceEntityRelation, DbError>;
    async fn get_product_by_id(&self, id: Uuid) -> Result<ProductEntity, DbError>;
    async fn list_products(&self, query: ListProductsQuery) -> Result<ProductListResults, DbError>;
}

pub struct ProductRepo {
    pub db: PgPool,
}

const PRODUCT_COLUMNS: &str = formatcp!(
    r#"{p}.id, {p}.user_id, {p}.name, {p}.description, {p}.blurb, {p}.contract_address, {p}.payout_address, {p}.category, {p}.status, {p}.blockchain_status, {p}.transaction_hash, {p}.created_at, {p}.updated_at"#,
    p = "products"
);

const PRODUCT_RELATION_COLUMNS: &str = formatcp!(
    r#"{products}, {pr}.id as pr_id, {pr}.active as pr_active, {pr}.name as pr_name, {pr}.price_type as pr_type, {pr}.amount as pr_amount, {pr}.base_currency as pr_base_currency, {pr}.subscription_interval as pr_subscription_interval, {pr}.subscription_interval_count as pr_subscription_interval_count, {pr}.trial_days as pr_trial_days"#,
    products = PRODUCT_COLUMNS,
    pr = "pr"
);

const PRICE_COLUMNS: &str = formatcp!(
    r#"{pr}.id, {pr}.product_id, {pr}.active, {pr}.name, {pr}.price_type, {pr}.amount, {pr}.base_currency, {pr}.subscription_interval, {pr}.subscription_interval_count, {pr}.trial_days, {pr}.created_at, {pr}.updated_at"#,
    pr = "prices"
);

fn map_product_entity(row: PgRow) -> Result<ProductEntity, sqlx::Error> {
    let price_id_option = row.try_get("pr_id").ok();
    let mut price: Option<PriceEntityRelation> = None;
    if let Some(price_id) = price_id_option {
        price = Some(PriceEntityRelation {
            id: price_id,
            active: row.try_get("pr_active")?,
            name: row.try_get("pr_name")?,
            price_type: row.try_get_unchecked("pr_type")?,
            amount: row.try_get("pr_amount")?,
            base_currency: row.try_get_unchecked("pr_base_currency")?,
            subscription_interval: row.try_get_unchecked("pr_subscription_interval")?,
            subscription_interval_count: row.try_get("pr_subscription_interval_count")?,
            trial_days: row.try_get("pr_trial_days")?,
        });
    }
    Ok(ProductEntity {
        id: row.try_get("id")?,
        user_id: row.try_get("user_id")?,
        name: row.try_get("name")?,
        description: row.try_get("description")?,
        blurb: row.try_get("blurb")?,
        contract_address: row.try_get("contract_address")?,
        payout_address: row.try_get("payout_address")?,
        category: row.try_get_unchecked("category")?,
        status: row.try_get_unchecked("status")?,
        blockchain_status: row.try_get_unchecked("blockchain_status")?,
        transaction_hash: row.try_get("transaction_hash")?,
        price,
        created_at: row.try_get("created_at")?,
        updated_at: row.try_get("updated_at")?,
    })
}

fn map_price_entity_relation(row: PgRow) -> Result<PriceEntityRelation, sqlx::Error> {
    Ok(PriceEntityRelation {
        id: row.try_get("id")?,
        active: row.try_get("active")?,
        name: row.try_get("name")?,
        price_type: row.try_get_unchecked("price_type")?,
        amount: row.try_get("amount")?,
        base_currency: row.try_get_unchecked("base_currency")?,
        subscription_interval: row.try_get_unchecked("subscription_interval")?,
        subscription_interval_count: row.try_get("subscription_interval_count")?,
        trial_days: row.try_get("trial_days")?,
    })
}

fn map_product_list_entity(row: PgRow) -> Result<(ProductEntity, i64), sqlx::Error> {
    let count = row.try_get("count")?;
    let entity = map_product_entity(row)?;
    Ok((entity, count))
}

#[async_trait]
impl ProductRepoTrait for ProductRepo {
    fn get_db(&self) -> &PgPool {
        &self.db
    }

    async fn create_product(
        &self,
        tx: &mut Transaction<'_, Postgres>,
        props: ProductCreateProps,
    ) -> Result<ProductEntity, DbError> {
        let mut product = sqlx::query(formatcp!(
            // language=PostgreSQL
            r#"
              INSERT INTO "products" (user_id, name, description, blurb, contract_address, payout_address, category, status, blockchain_status)
              values ($1, $2, $3, $4, $5, $6, $7, $8, $9)
              RETURNING {}
            "#,
            PRODUCT_COLUMNS
        ))
        .bind(props.user_id)
        .bind(props.name)
        .bind(props.description)
        .bind(props.blurb)
        .bind(props.contract_address)
        .bind(props.payout_address)
        .bind(props.category.to_string())
        .bind(props.status.to_string())
        .bind(props.blockchain_status.to_string())
        .try_map(map_product_entity)
        .fetch_one(tx.as_mut())
        .await.map_err(|e| match e {
            sqlx::Error::Database(dbe) if dbe.constraint() == Some("products_name_key") => {
                DbError::Unique("name".into())
            }
            _ => DbError::Query(e.to_string()),
        })?;

        if let Some(price_props) = props.price {
            product.price = Some(
                self.create_price(tx, product.id.clone(), price_props)
                    .await?,
            );
        }
        Ok(product)
    }

    async fn create_price(
        &self,
        tx: &mut Transaction<'_, Postgres>,
        product_id: Uuid,
        props: PriceCreateProps,
    ) -> Result<PriceEntityRelation, DbError> {
        Ok(sqlx::query(formatcp!(
            // language=PostgreSQL
            r#"
              INSERT INTO "prices" (product_id, active, name, price_type, amount, base_currency, subscription_interval, subscription_interval_count, trial_days)
              values ($1, $2, $3, $4, $5, $6, $7, $8, $9)
              RETURNING {}
            "#,
            PRICE_COLUMNS
        ))
        .bind(product_id)
        .bind(props.active)
        .bind(props.name)
        .bind(props.price_type.to_string())
        .bind(props.amount)
        .bind(props.base_currency.to_string())
        .bind(option_enum_to_string(props.subscription_interval))
        .bind(props.subscription_interval_count)
        .bind(props.trial_days)
        .try_map(map_price_entity_relation)
        .fetch_one(tx.as_mut())
        .await.map_err(map_sqlx_err)?)
    }

    async fn update_product(
        &self,
        tx: &mut Transaction<'_, Postgres>,
        id: Uuid,
        props: ProductUpdateProps,
    ) -> Result<ProductEntity, DbError> {
        let query = QueryBuilder::new("UPDATE products SET");
        let update_count = 0;

        let (query, update_count) = append_comma(query, "name", props.name, update_count);
        let (query, update_count) =
            append_comma(query, "description", props.description, update_count);
        let (query, update_count) = append_comma(query, "blurb", props.blurb, update_count);

        let (query, update_count) = append_comma(
            query,
            "category",
            option_enum_to_string(props.category),
            update_count,
        );
        let (query, update_count) =
            append_comma(query, "payout_address", props.payout_address, update_count);

        let (query, update_count) = append_comma(
            query,
            "status",
            option_enum_to_string(props.status),
            update_count,
        );

        let (mut query, update_count) = append_comma(
            query,
            "blockchain_status",
            option_enum_to_string(props.blockchain_status),
            update_count,
        );

        if update_count == 0 && !props.price.is_none() {
            return Err(DbError::NoUpdate);
        }

        query.push(" WHERE id = ");
        query.push_bind(id);
        query.push(formatcp!(" RETURNING {}", PRODUCT_COLUMNS));

        let mut product = query
            .build()
            .try_map(map_product_entity)
            .fetch_one(&self.db)
            .await
            .map_err(|e| match e {
                sqlx::Error::RowNotFound => DbError::EntityNotFound(),
                sqlx::Error::Database(dbe) if dbe.constraint() == Some("products_name_key") => {
                    DbError::Unique("name".into())
                }
                _ => DbError::Query(e.to_string()),
            })?;

        if let Some(price_props) = props.price {
            product.price = Some(self.update_price(tx, price_props).await?);
        }
        Ok(product)
    }

    async fn update_price(
        &self,
        tx: &mut Transaction<'_, Postgres>,
        props: PriceUpdateProps,
    ) -> Result<PriceEntityRelation, DbError> {
        let query = QueryBuilder::new("UPDATE prices SET");
        let update_count = 0;

        let (query, update_count) = append_comma(query, "active", props.active, update_count);
        let (query, update_count) = append_comma(query, "name", props.name, update_count);
        let (query, update_count) = append_comma(
            query,
            "price_type",
            option_enum_to_string(props.price_type),
            update_count,
        );

        let (query, update_count) = append_comma(query, "amount", props.amount, update_count);
        let (query, update_count) = append_comma(
            query,
            "subscription_interval",
            option_enum_to_string(props.subscription_interval),
            update_count,
        );
        let (query, update_count) = append_comma(
            query,
            "subscription_interval_count",
            props.subscription_interval_count,
            update_count,
        );

        let (mut query, _) = append_comma(query, "trial_days", props.trial_days, update_count);

        query.push(" WHERE id = ");
        query.push_bind(props.id);
        query.push(formatcp!(" RETURNING {}", PRICE_COLUMNS));

        Ok(query
            .build()
            .try_map(map_price_entity_relation)
            .fetch_one(tx.as_mut())
            .await
            .map_err(|e| match e {
                sqlx::Error::RowNotFound => DbError::EntityNotFound(),
                sqlx::Error::Database(dbe) if dbe.constraint() == Some("products_name_key") => {
                    DbError::Unique("name".into())
                }
                _ => DbError::Query(e.to_string()),
            })?)
    }

    async fn get_product_by_id(&self, id: Uuid) -> Result<ProductEntity, DbError> {
        Ok(sqlx::query(formatcp!(
            "SELECT {} FROM \"products\" LEFT OUTER JOIN prices pr on pr.product_id = products.id WHERE products.id = $1",
            PRODUCT_RELATION_COLUMNS
        ))
        .bind(id)
        .try_map(map_product_entity)
        .fetch_one(&self.db)
        .await
        .map_err(map_sqlx_err)?)
    }

    async fn list_products(&self, query: ListProductsQuery) -> Result<ProductListResults, DbError> {
        let filtered_query =
            if query.categories.is_none() && query.statuses.is_none() && query.user_id.is_none() {
                QueryBuilder::new("SELECT *, COUNT(*) OVER () FROM \"products\"")
            } else {
                QueryBuilder::new("SELECT *, COUNT(*) OVER () FROM \"products\" WHERE")
            };

        // Filter categories
        let (filtered_query, count) = append_in(filtered_query, "category", query.categories, 0);
        // Filter statuses
        let (filtered_query, count) = append_in(filtered_query, "status", query.statuses, count);
        // Filter user_id
        let (mut filtered_query, _) = append_and_eq(
            filtered_query,
            "user_id",
            option_string_to_uuid(query.user_id),
            count,
        );
        // ORDER BY
        let column = to_string(&query.column.unwrap_or(ProductSortColumn::CreatedAt))
            .map_err(|e| DbError::Serialize(e.to_string()))?;
        let direction = query.direction.unwrap_or(SortDirection::Desc);

        filtered_query = append_order_by(filtered_query, column, direction.to_string());
        filtered_query = append_limit_offset(filtered_query, query.from, query.to);
        let results = filtered_query
            .build()
            .try_map(map_product_list_entity)
            .fetch_all(&self.db)
            .await?;

        let (results, total) = list_result(results);

        Ok(ProductListResults { total, results })
    }
}
