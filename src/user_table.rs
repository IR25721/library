use crate::User;
use sqlx::Result;
use sqlx::sqlite::SqlitePool;

pub async fn register_userinfo(userinfo: &User) -> Result<()> {
    let pool = SqlitePool::connect("sqlite:database.db").await?;
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
    .execute(&pool)
    .await?;

    Ok(())
}
