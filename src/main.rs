use discord_bot::Handler;
use readname::User;
use sqlx::*;
use tokio::io::AsyncReadExt;
use user_table::LoginInfo;
use user_table::LoginUserInfo;
use user_table::is_user_exists;
use user_table::register_userinfo;
mod discord_bot;
mod readname;
mod user_table;
use serenity::prelude::*;
use std::env;

#[tokio::main]
async fn main() -> Result<()> {
    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");
    let intents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::DIRECT_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT;
    let mut client = Client::builder(&token, intents)
        .event_handler(Handler)
        .await
        .expect("Err creating client");

    let _bot_task = tokio::spawn(async move {
        if let Err(why) = client.start().await {
            println!("Client error: {why:?}");
        }
    });

    let pool = SqlitePool::connect("sqlite:database.db").await?;

    let user = User::get_userinfo();
    let user_id = user.get_userid();
    let user_name = user.get_username();
    let register_userinfo = register_userinfo(&pool, &user);
    let login = LoginUserInfo::log_user_access(&pool, &user_id);
    let user_access_info = LoginUserInfo::get_access_info(&pool, &user_id).await?;
    let is_exist = is_user_exists(&pool, &user_id);

    if is_exist.await? {
        let greeting = format!(
            "こんにちは{}さん.現在時刻は{}です.あなたはこれまで{:?}回ログインしました．",
            user_name,
            login.await?,
            user_access_info.get_count_login()
        );
        println!("{}", greeting);
    } else {
        println!(
            "はじめまして！\nDBにUser情報を登録しますか？\n登録するならEnterを，しないなら終了してください．"
        );
        let mut stdin = tokio::io::stdin();
        let mut buffer = [0; 1];
        stdin.read_exact(&mut buffer).await?;

        if buffer[0] == b'\n' {
            match register_userinfo.await {
                Ok(_) => println!("登録が完了しました！"),
                Err(e) => eprintln!("登録に失敗しました: {}", e),
            }
        } else {
            println!("登録をキャンセルしました");
        }
    }

    Ok(())
}
