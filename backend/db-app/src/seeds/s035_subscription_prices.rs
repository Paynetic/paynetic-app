use std::str::FromStr;

use lib_api::db::db_error::DbError;
use serde::Serialize;
use sqlx::PgPool;
use uuid::Uuid;

use crate::util::bulk_insert;

#[derive(Serialize)]
pub struct SubscriptionPricesDbProps {
    id: Uuid,
    subscription_id: Uuid,
    price_id: Uuid,
}

pub async fn seed(db: &PgPool) -> Result<(), DbError> {
    let table = "subscription_prices";

    let data = vec![
        SubscriptionPricesDbProps {
            id: Uuid::from_str("4987637b-73b4-49b2-a5f7-6170ff510b20").unwrap(),
            subscription_id: Uuid::from_str("ebbd2025-62f6-4444-bc01-54ceea0d1fba").unwrap(),
            price_id: Uuid::from_str("7f30f816-3990-4152-8693-9137e96331ec").unwrap(),
        },
        SubscriptionPricesDbProps {
            id: Uuid::from_str("01348b11-1135-4c2b-956d-759bb59c6745").unwrap(),
            subscription_id: Uuid::from_str("ebbd2025-62f6-4444-bc01-54ceea0d1fba").unwrap(),
            price_id: Uuid::from_str("7f30f816-3990-4152-8693-9137e96331ec").unwrap(),
        },
        SubscriptionPricesDbProps {
            id: Uuid::from_str("5b84868c-c550-4722-8558-268f6c692cf5").unwrap(),
            subscription_id: Uuid::from_str("daf969f2-740a-4a4e-ab41-b9ba31e32bbb").unwrap(),
            price_id: Uuid::from_str("7f30f816-3990-4152-8693-9137e96331ec").unwrap(),
        },
        SubscriptionPricesDbProps {
            id: Uuid::from_str("caf0f765-a5f4-46eb-8e03-f630a334a612").unwrap(),
            subscription_id: Uuid::from_str("4aff4df5-e17b-4a23-8d58-bc2d376ff022").unwrap(),
            price_id: Uuid::from_str("81414bd8-86b1-4fac-9cd0-e9e437483c2a").unwrap(),
        },
        SubscriptionPricesDbProps {
            id: Uuid::from_str("2f851243-aa2c-4e72-ab3d-41b595a63246").unwrap(),
            subscription_id: Uuid::from_str("4aff4df5-e17b-4a23-8d58-bc2d376ff022").unwrap(),
            price_id: Uuid::from_str("81414bd8-86b1-4fac-9cd0-e9e437483c2a").unwrap(),
        },
        SubscriptionPricesDbProps {
            id: Uuid::from_str("abf9d01d-cade-40da-adbe-b4d998f3b95b").unwrap(),
            subscription_id: Uuid::from_str("fed3fcc1-b1c7-4cdc-8ff3-9f6f150a9671").unwrap(),
            price_id: Uuid::from_str("81414bd8-86b1-4fac-9cd0-e9e437483c2a").unwrap(),
        },
        SubscriptionPricesDbProps {
            id: Uuid::from_str("a9553eb8-b352-403c-a97f-2e115c8d26c7").unwrap(),
            subscription_id: Uuid::from_str("886122be-ac6c-4101-955f-ba603bede6fc").unwrap(),
            price_id: Uuid::from_str("81414bd8-86b1-4fac-9cd0-e9e437483c2a").unwrap(),
        },
        SubscriptionPricesDbProps {
            id: Uuid::from_str("4cc7ac9d-0538-486e-81ee-a3ffec011e48").unwrap(),
            subscription_id: Uuid::from_str("bad3ed40-d8c3-4b5a-bd7b-44f54c8b4022").unwrap(),
            price_id: Uuid::from_str("e1b6f8a4-0f84-42f1-9684-f8c14ca3e87c").unwrap(),
        },
    ];

    Ok(bulk_insert(&db, table, &data).await?)
}
