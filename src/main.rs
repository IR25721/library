use discord_bot::Access;
use discord_bot::AccessInfo;
use discord_bot::Greeting;
use readname::User;
use tokio::io::AsyncReadExt;
use user_table::{LoginInfo, LoginUserInfo, is_user_exists, register_userinfo};
mod discord_bot;
mod readname;
mod user_table;

use serenity::async_trait;
use serenity::model::gateway::Ready;
use serenity::prelude::*;
use std::env;
use std::sync::atomic::{AtomicBool, Ordering};

pub struct Handler {
    is_exist: bool,
    greeting: String,
    has_sent: AtomicBool,
}

impl Handler {
    pub fn new(is_exist: bool, greeting: String) -> Self {
        Self {
            is_exist,
            greeting,
            has_sent: AtomicBool::new(false),
        }
    }
}
#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }

    async fn guild_create(
        &self,
        ctx: Context,
        guild: serenity::model::guild::Guild,
        _is_new: Option<bool>,
    ) {
        println!("GuildCreateイベント発生: {}", guild.name);

        if self.is_exist && !self.has_sent.load(Ordering::Relaxed) {
            let _channel_id = guild
                .channels
                .iter()
                .find(|(_, c)| c.kind == serenity::model::channel::ChannelType::Text)
                .map(|(&id, _)| id);

            if let Some((id, channel)) = guild
                .channels
                .iter()
                .find(|(_, c)| c.kind == serenity::model::channel::ChannelType::Text)
            {
                println!(
                    "メッセージを送信するチャンネル {} (名前: {}) を見つけました",
                    id, channel.name
                );
                match id.say(&ctx.http, self.greeting.clone()).await {
                    Ok(_) => {
                        println!("メッセージ送信に成功しました");
                        self.has_sent.store(true, Ordering::Relaxed);
                    }
                    Err(why) => {
                        println!("送信失敗: {:?}", why);
                    }
                }
            } else {
                println!("適切なテキストチャンネルが見つかりませんでした");
            }
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let access_data = AccessInfo::new("sqlite:database.db").await;
    let pool = &access_data.get_pool().await;
    let user = &access_data.get_user();
    let user_id = access_data.get_user().get_userid();
    let user_name = access_data.get_user().get_username();
    let register_userinfo = register_userinfo(pool, user);
    let login = access_data.get_login();
    let user_access_info = LoginUserInfo::get_access_info(pool, &user_id).await?;
    let is_exist = is_user_exists(pool, &user_id).await?;
    let greeting = Greeting::get_message(&Greeting::new(
        &user_name,
        login.await,
        user_access_info.get_count_login(),
    ));
    if is_exist {
        println!("{}", &greeting);
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
    let handler = Handler::new(is_exist, greeting.clone());
    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");
    let intents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::GUILDS
        | GatewayIntents::DIRECT_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT;

    let mut client = Client::builder(&token, intents)
        .event_handler(handler)
        .await
        .expect("Err creating client");

    if let Err(why) = client.start().await {
        println!("Client error: {why:?}");
    };

    Ok(())
}
