use std::str::FromStr;

use chrono::{DateTime, Duration, Utc};
use lib_api::db::db_error::DbError;
use lib_types::shared::{product::BlockchainStatus, subscription::SubscriptionStatus};
use serde::Serialize;
use sqlx::PgPool;
use uuid::Uuid;

use crate::util::bulk_insert;

#[derive(Serialize)]
pub struct SubscriptionDbProps {
    pub id: Uuid,
    pub user_id: Uuid,
    pub product_id: Uuid,
    pub contract_address: String,
    pub status: SubscriptionStatus,
    pub fee_percent: i32,
    pub start_time: i64,
    pub current_start: i64,
    pub current_end: i64,
    pub grace: i64,
    pub blockchain_status: BlockchainStatus,
    pub transaction_hash: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

pub async fn seed(db: &PgPool) -> Result<(), DbError> {
    let table = "subscriptions";

    let data = vec![
        SubscriptionDbProps {
            id: Uuid::from_str("ebbd2025-62f6-4444-bc01-54ceea0d1fba").unwrap(),
            user_id: Uuid::from_str("e10aa33c-94ef-4035-98d1-6372f82454d5").unwrap(), // User1
            product_id: Uuid::from_str("673fad5b-626a-4a74-8cbd-afd45c4cf7d3").unwrap(), // Tunes
            contract_address: "0x0000000000000000000000000000000000000000".into(),
            status: SubscriptionStatus::Initial,
            fee_percent: 100,
            start_time: (Utc::now() - Duration::weeks(1)).timestamp(),
            current_start: (Utc::now() - Duration::weeks(1)).timestamp(),
            current_end: (Utc::now() + Duration::weeks(3)).timestamp(),
            grace: Duration::days(5).num_seconds(),
            blockchain_status: BlockchainStatus::None,
            transaction_hash: None,
            created_at: Utc::now() - Duration::days(30),
            updated_at: Utc::now(),
        },
        SubscriptionDbProps {
            id: Uuid::from_str("a643749f-c59a-4ff6-9d6b-3f519bd40acd").unwrap(),
            user_id: Uuid::from_str("2199c39f-4fb3-4f72-b685-fe62b90fcef0").unwrap(), // User2
            product_id: Uuid::from_str("673fad5b-626a-4a74-8cbd-afd45c4cf7d3").unwrap(), // Tunes
            contract_address: "0x0000000000000000000000000000000000000000".into(),
            status: SubscriptionStatus::Pending,
            fee_percent: 100,
            start_time: (Utc::now() - Duration::weeks(1)).timestamp(),
            current_start: (Utc::now() - Duration::weeks(1)).timestamp(),
            current_end: (Utc::now() + Duration::weeks(3)).timestamp(),
            grace: Duration::days(5).num_seconds(),
            blockchain_status: BlockchainStatus::None,
            transaction_hash: None,
            created_at: Utc::now() - Duration::days(30),
            updated_at: Utc::now(),
        },
        SubscriptionDbProps {
            id: Uuid::from_str("daf969f2-740a-4a4e-ab41-b9ba31e32bbb").unwrap(),
            user_id: Uuid::from_str("90679368-ba27-4d7f-be85-849b4328d93a").unwrap(), // User3
            product_id: Uuid::from_str("673fad5b-626a-4a74-8cbd-afd45c4cf7d3").unwrap(), // Tunes
            contract_address: "0x0000000000000000000000000000000000000000".into(),
            status: SubscriptionStatus::Trial,
            fee_percent: 100,
            start_time: (Utc::now() - Duration::days(1)).timestamp(),
            current_start: (Utc::now() + Duration::weeks(1)).timestamp(),
            current_end: (Utc::now() + Duration::weeks(5)).timestamp(),
            grace: Duration::days(5).num_seconds(),
            blockchain_status: BlockchainStatus::None,
            transaction_hash: None,
            created_at: Utc::now() - Duration::days(30),
            updated_at: Utc::now(),
        },
        SubscriptionDbProps {
            id: Uuid::from_str("4aff4df5-e17b-4a23-8d58-bc2d376ff022").unwrap(),
            user_id: Uuid::from_str("e10aa33c-94ef-4035-98d1-6372f82454d5").unwrap(), // User1
            product_id: Uuid::from_str("d105b284-e516-42c1-8690-6612e7cd7bc7").unwrap(), // Tech
            contract_address: "0x0000000000000000000000000000000000000000".into(),
            status: SubscriptionStatus::Active,
            fee_percent: 100,
            start_time: (Utc::now() - Duration::weeks(1)).timestamp(),
            current_start: (Utc::now() - Duration::weeks(1)).timestamp(),
            current_end: (Utc::now() + Duration::weeks(3)).timestamp(),
            grace: Duration::days(5).num_seconds(),
            blockchain_status: BlockchainStatus::None,
            transaction_hash: None,
            created_at: Utc::now() - Duration::days(30),
            updated_at: Utc::now(),
        },
        SubscriptionDbProps {
            id: Uuid::from_str("fed3fcc1-b1c7-4cdc-8ff3-9f6f150a9671").unwrap(),
            user_id: Uuid::from_str("2199c39f-4fb3-4f72-b685-fe62b90fcef0").unwrap(), // User2
            product_id: Uuid::from_str("d105b284-e516-42c1-8690-6612e7cd7bc7").unwrap(), // Tech
            contract_address: "0x0000000000000000000000000000000000000000".into(),
            status: SubscriptionStatus::PastDue,
            fee_percent: 100,
            start_time: (Utc::now() - Duration::weeks(1)).timestamp(),
            current_start: (Utc::now() - Duration::weeks(1)).timestamp(),
            current_end: (Utc::now() + Duration::weeks(3)).timestamp(),
            grace: Duration::days(5).num_seconds(),
            blockchain_status: BlockchainStatus::None,
            transaction_hash: None,
            created_at: Utc::now() - Duration::days(30),
            updated_at: Utc::now(),
        },
        SubscriptionDbProps {
            id: Uuid::from_str("886122be-ac6c-4101-955f-ba603bede6fc").unwrap(),
            user_id: Uuid::from_str("90679368-ba27-4d7f-be85-849b4328d93a").unwrap(), // User3
            product_id: Uuid::from_str("d105b284-e516-42c1-8690-6612e7cd7bc7").unwrap(), // Tech
            contract_address: "0x0000000000000000000000000000000000000000".into(),
            status: SubscriptionStatus::Cancelled,
            fee_percent: 100,
            start_time: (Utc::now() - Duration::weeks(1)).timestamp(),
            current_start: (Utc::now() - Duration::weeks(1)).timestamp(),
            current_end: (Utc::now() + Duration::weeks(3)).timestamp(),
            grace: Duration::days(5).num_seconds(),
            blockchain_status: BlockchainStatus::None,
            transaction_hash: None,
            created_at: Utc::now() - Duration::days(30),
            updated_at: Utc::now(),
        },
        SubscriptionDbProps {
            id: Uuid::from_str("bad3ed40-d8c3-4b5a-bd7b-44f54c8b4022").unwrap(),
            user_id: Uuid::from_str("e10aa33c-94ef-4035-98d1-6372f82454d5").unwrap(), // User1
            product_id: Uuid::from_str("1342c40f-dada-4e12-af3d-6e073ad171a4").unwrap(), // Donation
            contract_address: "0x0000000000000000000000000000000000000000".into(),
            status: SubscriptionStatus::Paused,
            fee_percent: 100,
            start_time: (Utc::now() - Duration::weeks(1)).timestamp(),
            current_start: (Utc::now() - Duration::weeks(1)).timestamp(),
            current_end: (Utc::now() + Duration::weeks(3)).timestamp(),
            grace: Duration::days(5).num_seconds(),
            blockchain_status: BlockchainStatus::None,
            transaction_hash: None,
            created_at: Utc::now() - Duration::days(30),
            updated_at: Utc::now(),
        },
    ];

    Ok(bulk_insert(&db, table, &data).await?)
}
