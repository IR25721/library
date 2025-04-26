use crate::User;
use sqlx::Result;
use sqlx::sqlite::SqlitePool;

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
