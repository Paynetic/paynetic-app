use std::sync::Arc;

use axum::async_trait;
use const_format::formatcp;
use lib_api::db::{
    db_error::{map_sqlx_err, DbError},
    util::{
        append_and_eq, append_comma, append_in, append_limit_offset, append_order_by,
        option_enum_to_string, option_string_to_uuid,
    },
};
use lib_types::{
    dto::{
        sort_direction::SortDirection,
        subscription::list_subscriptions_dto::{ListSubscriptionsQuery, SubscriptionSortColumn},
    },
    entity::subscription_entity::{
        PriceEntityRelation, SubscriptionCreateResult, SubscriptionEntity, SubscriptionListResults,
    },
    shared::{product::BlockchainStatus, subscription::SubscriptionStatus},
};
use serde::{Deserialize, Serialize};
use serde_json::to_string;
use sqlx::{postgres::PgRow, PgPool, Postgres, QueryBuilder, Row, Transaction};
use uuid::Uuid;

pub type DynSubscriptionRepo = Arc<dyn SubscriptionRepoTrait + Send + Sync>;

#[derive(Debug, Deserialize, Serialize, sqlx::Type)]
pub struct SubscriptionCreateProps {
    pub user_id: Uuid,
    pub product_id: Uuid,
    pub contract_address: String,
    pub status: SubscriptionStatus,
    pub blockchain_status: BlockchainStatus,
    pub price_id: Uuid,
    pub start_time: i64,
    pub current_start: i64,
    pub current_end: i64,
    pub grace: i64,
}

#[derive(Debug, Deserialize, Serialize, sqlx::Type)]
pub struct SubscriptionUpdateProps {
    pub status: Option<SubscriptionStatus>,
}

#[async_trait]
pub trait SubscriptionRepoTrait {
    fn get_db(&self) -> &PgPool;
    async fn create_subscription(
        &self,
        tx: &mut Transaction<'_, Postgres>,
        props: SubscriptionCreateProps,
    ) -> Result<SubscriptionCreateResult, DbError>;
    async fn create_subscription_price(
        &self,
        tx: &mut Transaction<'_, Postgres>,
        subscription_id: Uuid,
        price_id: Uuid,
    ) -> Result<(), DbError>;
    async fn update_subscription(
        &self,
        id: Uuid,
        props: SubscriptionUpdateProps,
    ) -> Result<(), DbError>;
    async fn get_subscription_by_id(&self, id: Uuid) -> Result<SubscriptionEntity, DbError>;
    async fn list_subscriptions(
        &self,
        query: ListSubscriptionsQuery,
    ) -> Result<SubscriptionListResults, DbError>;
}

pub struct SubscriptionRepo {
    pub db: PgPool,
}

const SUBSCRIPTION_COLUMNS: &str = formatcp!(
    r#"{p}.id, {p}.product_id, {p}.user_id, {p}.contract_address, {p}.status, {p}.fee_percent, {p}.start_time, {p}.current_start, {p}.current_end, {p}.grace, {p}.blockchain_status, {p}.transaction_hash, {p}.created_at, {p}.updated_at"#,
    p = "subscriptions"
);

const SUBSCRIPTION_RELATION_COLUMNS: &str = formatcp!(
    r#"{subscriptions}, {pr}.id as pr_id, {pr}.active as pr_active, {pr}.price_type as pr_type, {pr}.amount as pr_amount, {pr}.base_currency as pr_base_currency, {pr}.subscription_interval as pr_subscription_interval, {pr}.subscription_interval_count as pr_subscription_interval_count, {pr}.trial_days as pr_trial_days"#,
    subscriptions = SUBSCRIPTION_COLUMNS,
    pr = "pr"
);

// Helper for setting up subscription in list results
fn subscription_from_row_helper(row: &PgRow) -> Result<Option<SubscriptionEntity>, DbError> {
    let pr_id: Option<Uuid> = row.try_get("pr_id")?;
    let prices = if pr_id.is_some() {
        vec![price_from_row(row).map_err(map_sqlx_err)?]
    } else {
        vec![]
    };
    let next = subscription_from_row(&row, prices).map_err(map_sqlx_err)?;
    Ok(Some(next))
}

fn subscription_from_row(
    row: &PgRow,
    prices: Vec<PriceEntityRelation>,
) -> Result<SubscriptionEntity, sqlx::Error> {
    Ok(SubscriptionEntity {
        id: row.try_get("id")?,
        product_id: row.try_get("product_id")?,
        user_id: row.try_get("user_id")?,
        contract_address: row.try_get("contract_address")?,
        status: row.try_get_unchecked("status")?,
        fee_percent: row.try_get_unchecked("fee_percent")?,
        start_time: row.try_get_unchecked("start_time")?,
        current_start: row.try_get_unchecked("current_start")?,
        current_end: row.try_get_unchecked("current_end")?,
        grace: row.try_get_unchecked("grace")?,
        blockchain_status: row.try_get_unchecked("blockchain_status")?,
        transaction_hash: row.try_get("transaction_hash")?,
        prices,
        created_at: row.try_get("created_at")?,
        updated_at: row.try_get("updated_at")?,
    })
}

fn price_from_row(row: &PgRow) -> Result<PriceEntityRelation, sqlx::Error> {
    Ok(PriceEntityRelation {
        id: row.try_get("pr_id")?,
        active: row.try_get("pr_active")?,
        price_type: row.try_get_unchecked("pr_type")?,
        amount: row.try_get("pr_amount")?,
        base_currency: row.try_get_unchecked("pr_base_currency")?,
        subscription_interval: row.try_get_unchecked("pr_subscription_interval")?,
        subscription_interval_count: row.try_get("pr_subscription_interval_count")?,
        trial_days: row.try_get("pr_trial_days")?,
    })
}

fn map_subscription_entity(rows: Vec<PgRow>) -> Result<SubscriptionEntity, sqlx::Error> {
    let mut prices: Vec<PriceEntityRelation> = vec![];
    for row in rows.iter() {
        if row.try_get::<Option<Uuid>, &str>("pr_id")?.is_none() {
            continue;
        }
        prices.push(price_from_row(row)?);
    }
    let row = rows.into_iter().nth(0).ok_or(sqlx::Error::RowNotFound)?;
    Ok(subscription_from_row(&row, prices)?)
}

#[async_trait]
impl SubscriptionRepoTrait for SubscriptionRepo {
    fn get_db(&self) -> &PgPool {
        &self.db
    }

    async fn create_subscription(
        &self,
        tx: &mut Transaction<'_, Postgres>,
        props: SubscriptionCreateProps,
    ) -> Result<SubscriptionCreateResult, DbError> {
        let row = sqlx::query(formatcp!(
            // language=PostgreSQL
            r#"
              INSERT INTO "subscriptions" (user_id, product_id, contract_address, status, fee_percent, start_time, current_start, current_end, grace, blockchain_status)
              values ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)
              RETURNING subscriptions.id
            "#
        ))
        .bind(props.user_id)
        .bind(props.product_id)
        .bind(props.contract_address)
        .bind(props.status.to_string())
        .bind(100)
        .bind(props.start_time)
        .bind(props.current_start)
        .bind(props.current_end)
        .bind(props.grace)
        .bind(props.blockchain_status.to_string())
        .fetch_one(tx.as_mut())
        .await.map_err(map_sqlx_err)?;

        let subscription_id: Uuid = row.try_get("id")?;
        self.create_subscription_price(tx, subscription_id, props.price_id)
            .await?;
        Ok(SubscriptionCreateResult {
            id: subscription_id,
        })
    }

    async fn create_subscription_price(
        &self,
        tx: &mut Transaction<'_, Postgres>,
        subscription_id: Uuid,
        price_id: Uuid,
    ) -> Result<(), DbError> {
        sqlx::query(formatcp!(
            // language=PostgreSQL
            r#"
                INSERT INTO subscription_prices (subscription_id, price_id) values ($1, $2)
                "#
        ))
        .bind(subscription_id)
        .bind(price_id)
        .execute(tx.as_mut())
        .await
        .map_err(map_sqlx_err)?;
        Ok(())
    }

    async fn update_subscription(
        &self,
        id: Uuid,
        props: SubscriptionUpdateProps,
    ) -> Result<(), DbError> {
        let query = QueryBuilder::new("UPDATE subscriptions SET");
        let update_count = 0;

        let (mut query, update_count) = append_comma(
            query,
            "status",
            option_enum_to_string(props.status),
            update_count,
        );

        if update_count == 0 {
            return Err(DbError::NoUpdate);
        }

        query.push(" WHERE id = ");
        query.push_bind(id);

        query
            .build()
            .execute(&self.db)
            .await
            .map_err(map_sqlx_err)?;
        Ok(())
    }

    async fn get_subscription_by_id(&self, id: Uuid) -> Result<SubscriptionEntity, DbError> {
        let rows = sqlx::query(formatcp!(
            "SELECT {} FROM \"subscriptions\"
                LEFT OUTER JOIN subscription_prices sp on sp.subscription_id = subscriptions.id
                LEFT OUTER JOIN prices pr on sp.price_id = pr.id
                WHERE subscriptions.id = $1",
            SUBSCRIPTION_RELATION_COLUMNS
        ))
        .bind(id)
        .fetch_all(&self.db)
        .await
        .map_err(map_sqlx_err)?;

        map_subscription_entity(rows).map_err(map_sqlx_err)
    }

    async fn list_subscriptions(
        &self,
        query: ListSubscriptionsQuery,
    ) -> Result<SubscriptionListResults, DbError> {
        let mut filtered_query = QueryBuilder::new(formatcp!(
            "SELECT {} FROM \"subscriptions\"
                LEFT OUTER JOIN subscription_prices sp on sp.subscription_id = subscriptions.id
                LEFT OUTER JOIN prices pr on sp.price_id = pr.id",
            SUBSCRIPTION_RELATION_COLUMNS
        ));
        if query.product_id.is_some() || query.statuses.is_some() || query.user_id.is_some() {
            filtered_query.push(" WHERE");
        }
        // Filter statuses
        let (filtered_query, count) = append_in(filtered_query, "status", query.statuses, 0);
        // Filter user_id
        let (filtered_query, _) = append_and_eq(
            filtered_query,
            "user_id",
            option_string_to_uuid(query.user_id),
            count,
        );
        // Filter product_id
        let (mut filtered_query, _) = append_and_eq(
            filtered_query,
            "product_id",
            option_string_to_uuid(query.product_id),
            count,
        );
        // ORDER BY
        let column = to_string(&query.column.unwrap_or(SubscriptionSortColumn::CreatedAt))
            .map_err(|e| DbError::Serialize(e.to_string()))?;
        let direction = query.direction.unwrap_or(SortDirection::Desc);

        filtered_query.push(" GROUP BY subscriptions.id, pr.id");
        filtered_query = append_order_by(filtered_query, column, direction.to_string());
        filtered_query = append_limit_offset(filtered_query, query.from, query.to);
        let rows = filtered_query.build().fetch_all(&self.db).await?;

        let mut results: Vec<SubscriptionEntity> = vec![];
        let mut result_opt: Option<SubscriptionEntity> = None;
        for row in rows.iter() {
            if let Some(ref mut cur) = result_opt {
                let id: Uuid = row.try_get("id")?;
                if id == cur.id {
                    cur.prices.push(price_from_row(row).map_err(map_sqlx_err)?)
                } else {
                    results.push(cur.clone());
                    result_opt = subscription_from_row_helper(&row)?;
                }
            } else {
                result_opt = subscription_from_row_helper(&row)?;
            }
        }
        if let Some(result) = result_opt {
            results.push(result);
        }
        println!("R {:?}", results);
        Ok(SubscriptionListResults {
            total: results.len() as i64,
            results,
        })
    }
}
