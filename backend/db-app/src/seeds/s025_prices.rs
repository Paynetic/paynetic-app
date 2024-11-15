use std::str::FromStr;

use chrono::Utc;
use lib_api::db::db_error::DbError;
use lib_types::{
    entity::price_entity::PriceEntity,
    shared::product::{PaymentCurrency, PriceType, SubscriptionInterval},
};
use sqlx::PgPool;
use uuid::Uuid;

use crate::util::bulk_insert;

pub async fn seed(db: &PgPool) -> Result<(), DbError> {
    let table = "prices";

    let data = vec![
        PriceEntity {
            id: Uuid::from_str("2f46f1b9-0a24-4a68-bbc7-a26895a54e4d").unwrap(),
            product_id: Uuid::from_str("c55864f1-8a47-4de2-9dd7-80d06479e70f").unwrap(),
            name: "Monthly Refills".into(),
            active: true,
            price_type: PriceType::Subscription,
            amount: 8000000000000000i128.into(),
            base_currency: PaymentCurrency::Ethereum,
            subscription_interval: Some(SubscriptionInterval::Month),
            subscription_interval_count: Some(1),
            trial_days: Some(0),
            created_at: Utc::now(),
            updated_at: Utc::now(),
        },
        PriceEntity {
            id: Uuid::from_str("52514d8f-6a70-4df5-812b-3fa82f89ab36").unwrap(),
            product_id: Uuid::from_str("26eb78aa-442f-4470-b724-d4ad090b9010").unwrap(),
            name: "Donation".into(),
            active: true,
            price_type: PriceType::Subscription,
            amount: 2000000000000000i128.into(),
            base_currency: PaymentCurrency::Ethereum,
            subscription_interval: Some(SubscriptionInterval::Month),
            subscription_interval_count: Some(1),
            trial_days: Some(0),
            created_at: Utc::now(),
            updated_at: Utc::now(),
        },
        PriceEntity {
            id: Uuid::from_str("9d41436f-6c4b-4c91-8f90-7ed11783efe4").unwrap(),
            product_id: Uuid::from_str("31153d5e-5c78-4156-85ee-fad95a25047b").unwrap(),
            name: "Weeklies".into(),
            active: true,
            price_type: PriceType::Subscription,
            amount: 10000000000000000i128.into(),
            base_currency: PaymentCurrency::Ethereum,
            subscription_interval: Some(SubscriptionInterval::Week),
            subscription_interval_count: Some(1),
            trial_days: Some(0),
            created_at: Utc::now(),
            updated_at: Utc::now(),
        },
        PriceEntity {
            id: Uuid::from_str("d6fdf6f1-8048-410a-ab47-6c456ab5595f").unwrap(),
            product_id: Uuid::from_str("49d5b949-e4a1-4c21-a502-8b1d310698dc").unwrap(),
            name: "Nothing".into(),
            active: true,
            price_type: PriceType::Once,
            amount: 10000000000000000i128.into(),
            base_currency: PaymentCurrency::Ethereum,
            subscription_interval: None,
            subscription_interval_count: None,
            trial_days: None,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        },
        PriceEntity {
            id: Uuid::from_str("96ddf74e-24f3-4469-a766-4e780a5a5be3").unwrap(),
            product_id: Uuid::from_str("a8708f5f-8b91-4cd2-a0e4-bbf1859a52db").unwrap(),
            name: "Three Shoes".into(),
            active: true,
            price_type: PriceType::Subscription,
            amount: 10000000000000000i128.into(),
            base_currency: PaymentCurrency::Ethereum,
            subscription_interval: Some(SubscriptionInterval::Year),
            subscription_interval_count: Some(1),
            trial_days: Some(0),
            created_at: Utc::now(),
            updated_at: Utc::now(),
        },
        PriceEntity {
            id: Uuid::from_str("4a318adb-be74-48c4-9ec3-89aaf77ed56d").unwrap(),
            product_id: Uuid::from_str("cf8550ca-3542-4165-8435-3f5c6c77a761").unwrap(),
            name: "Album".into(),
            active: true,
            price_type: PriceType::Once,
            amount: 1000000000000000i128.into(),
            base_currency: PaymentCurrency::Ethereum,
            subscription_interval: None,
            subscription_interval_count: None,
            trial_days: None,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        },
        PriceEntity {
            id: Uuid::from_str("7f30f816-3990-4152-8693-9137e96331ec").unwrap(),
            product_id: Uuid::from_str("673fad5b-626a-4a74-8cbd-afd45c4cf7d3").unwrap(),
            name: "Tunes".into(),
            active: true,
            price_type: PriceType::Subscription,
            amount: 7000000000000000i128.into(),
            base_currency: PaymentCurrency::Ethereum,
            subscription_interval: Some(SubscriptionInterval::Month),
            subscription_interval_count: Some(1),
            trial_days: Some(15),
            created_at: Utc::now(),
            updated_at: Utc::now(),
        },
        PriceEntity {
            id: Uuid::from_str("b4a4c2a1-577d-4853-bb4d-ee7741a414b9").unwrap(),
            product_id: Uuid::from_str("7922b138-65dc-412e-87c1-059325574595").unwrap(),
            name: "Tools".into(),
            active: true,
            price_type: PriceType::Once,
            amount: 10000000000000000i128.into(),
            base_currency: PaymentCurrency::Ethereum,
            subscription_interval: None,
            subscription_interval_count: None,
            trial_days: None,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        },
        PriceEntity {
            id: Uuid::from_str("81414bd8-86b1-4fac-9cd0-e9e437483c2a").unwrap(),
            product_id: Uuid::from_str("d105b284-e516-42c1-8690-6612e7cd7bc7").unwrap(),
            name: "Tech Delivery".into(),
            active: true,
            price_type: PriceType::Subscription,
            amount: 100000000000000000i128.into(),
            base_currency: PaymentCurrency::Ethereum,
            subscription_interval: Some(SubscriptionInterval::Month),
            subscription_interval_count: Some(1),
            trial_days: Some(0),
            created_at: Utc::now(),
            updated_at: Utc::now(),
        },
        PriceEntity {
            id: Uuid::from_str("e1b6f8a4-0f84-42f1-9684-f8c14ca3e87c").unwrap(),
            product_id: Uuid::from_str("1342c40f-dada-4e12-af3d-6e073ad171a4").unwrap(),
            name: "Donations".into(),
            active: true,
            price_type: PriceType::Subscription,
            amount: 9000000000000000i128.into(),
            base_currency: PaymentCurrency::Ethereum,
            subscription_interval: Some(SubscriptionInterval::Month),
            subscription_interval_count: Some(1),
            trial_days: Some(0),
            created_at: Utc::now(),
            updated_at: Utc::now(),
        },
    ];

    Ok(bulk_insert(&db, table, &data).await?)
}
