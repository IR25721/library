use crate::User;
use chrono::Local;
use sqlx::Result;
use sqlx::sqlite::SqlitePool;
pub trait LoginInfo {
    async fn log_user_access(pool: &SqlitePool, card_id: &str) -> sqlx::Result<String>;
    async fn get_access_info(pool: &SqlitePool, card_id: &str) -> sqlx::Result<LoginUserInfo>;
}
#[derive(Debug, Clone)]
pub struct LoginUserInfo {
    last_login: String,
    latest_login: String,
    count_login: usize,
}
//構造体にいらなくね
pub async fn register_userinfo(pool: &SqlitePool, userinfo: &User) -> Result<()> {
    let userid = userinfo.get_userid();
    let username = userinfo.get_username();
    sqlx::query!(
        r#"
        INSERT INTO users (card_id, name)
        VALUES (?, ?)
        "#,
        userid,
        username,
    )
    .execute(pool)
    .await?;

    Ok(())
}
pub async fn is_user_exists(pool: &SqlitePool, card_id: &str) -> sqlx::Result<bool> {
    let result = sqlx::query!(
        r#"
        SELECT card_id FROM users WHERE card_id = ?
        "#,
        card_id
    )
    .fetch_optional(pool)
    .await?;

    Ok(result.is_some())
}
impl LoginInfo for LoginUserInfo {
    async fn log_user_access(pool: &SqlitePool, card_id: &str) -> sqlx::Result<String> {
        if is_user_exists(pool, card_id).await? {
            println!("ログイン処理開始！");
            let now = Local::now().to_rfc3339();

            sqlx::query!(
                r#"
            INSERT INTO access_logs (card_id, accessed_at)
            VALUES (?, ?)
            "#,
                card_id,
                now
            )
            .execute(pool)
            .await?;
            Ok(now)
        } else {
            Err(sqlx::Error::RowNotFound)
        }
    }
    async fn get_access_info(pool: &SqlitePool, card_id: &str) -> sqlx::Result<Self> {
        let records = sqlx::query!(
            r#"
            SELECT accessed_at
            FROM access_logs
            WHERE card_id = ?
            ORDER BY accessed_at DESC
            "#,
            card_id
        )
        .fetch_all(pool)
        .await?;

        let count_login = records.len();

        let latest_login = records
            .first()
            .map(|r| r.accessed_at.clone())
            .unwrap_or_else(|| "なし".to_string());
        let last_login = records
            .get(1)
            .map(|r| r.accessed_at.clone())
            .unwrap_or_else(|| "なし".to_string());

        Ok(Self {
            last_login,
            latest_login,
            count_login,
        })
    }
}

impl LoginUserInfo {
    pub fn get_last_login(&self) -> String {
        self.last_login.clone()
    }
    pub fn get_latest_login(&self) -> String {
        self.latest_login.clone()
    }
    pub fn get_count_login(&self) -> usize {
        self.count_login
    }
}
