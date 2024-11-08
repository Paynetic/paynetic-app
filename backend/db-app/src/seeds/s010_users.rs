use std::str::FromStr;

use chrono::Utc;
use lib_api::db::db_error::DbError;
use lib_api::db::password::hash;
use lib_types::{
    entity::user_entity::UserEntity,
    shared::user::{UserStatus, UserType},
};
use sqlx::PgPool;
use uuid::Uuid;

use crate::util::bulk_insert;

pub async fn seed(db: &PgPool) -> Result<(), DbError> {
    let table = "users";

    let data = vec![
        UserEntity {
            id: Uuid::from_str("efa88c17-a471-45a9-b6e4-c243edcd2d06").unwrap(),
            name: "Admin".into(),
            description: "Paynetic Admin".into(),
            link: "https://paynetic.net".into(),
            location: "Hong Kong".into(),
            email: "admin1@paynetic.net".into(),
            password_hash: hash("admin.password1".into()).await.unwrap(),
            eth_address: "0x0000000000000000000000000000000000000000".into(),
            created_at: Utc::now(),
            updated_at: Utc::now(),
            user_type: UserType::Admin,
            user_status: UserStatus::Active,
            email_confirmed: true,
        },
        UserEntity {
            id: Uuid::from_str("e10aa33c-94ef-4035-98d1-6372f82454d5").unwrap(),
            name: "user1@paynetic.net".into(),
            description: "Platform first user".into(),
            link: "https://paynetic.net/user/e10aa33c-94ef-4035-98d1-6372f82454d5".into(),
            location: "Japan".into(),
            email: "user1@paynetic.net".into(),
            password_hash: hash("password1".into()).await.unwrap(),
            eth_address: "0x871dF04f23EA830C3b65e16D76433004EbeDd001".into(),
            created_at: Utc::now(),
            updated_at: Utc::now(),
            user_type: UserType::User,
            user_status: UserStatus::Active,
            email_confirmed: true,
        },
        UserEntity {
            id: Uuid::from_str("2199c39f-4fb3-4f72-b685-fe62b90fcef0").unwrap(),
            name: "Technology For You".into(),
            description: "We make technology and help promote tech projects.".into(),
            link: "https://me.hello.com".into(),
            location: "California".into(),
            email: "user2@paynetic.net".into(),
            password_hash: hash("password2".into()).await.unwrap(),
            eth_address: "0x0000000000000000000000000000000000000002".into(),
            created_at: Utc::now(),
            updated_at: Utc::now(),
            user_type: UserType::User,
            user_status: UserStatus::Blocked,
            email_confirmed: true,
        },
        UserEntity {
            id: Uuid::from_str("90679368-ba27-4d7f-be85-849b4328d93a").unwrap(),
            name: "user3@paynetic.net".into(),
            description: "Big buyer".into(),
            link: "".into(),
            location: "Europe".into(),
            email: "user3@paynetic.net".into(),
            password_hash: hash("password3".into()).await.unwrap(),
            eth_address: "0x0000000000000000000000000000000000000003".into(),
            created_at: Utc::now(),
            updated_at: Utc::now(),
            user_type: UserType::User,
            user_status: UserStatus::Active,
            email_confirmed: false,
        },
    ];

    Ok(bulk_insert(&db, table, &data).await?)
}
