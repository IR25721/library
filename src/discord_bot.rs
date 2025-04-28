use crate::readname::{Fromcard, User};
use crate::user_table::{LoginInfo, LoginUserInfo, is_user_exists, register_userinfo};
use serenity::async_trait;
use serenity::model::channel::Message;
use serenity::model::gateway::Ready;
use serenity::prelude::*;
use sqlx::{Pool, Sqlite, SqlitePool};

pub struct AccessInfo {
    pool: Pool<Sqlite>,
    user: User,
    login: String,
    user_access_info: LoginUserInfo,
}

pub trait Access {
    fn get_user(&self) -> User;
    async fn new(pass: &str) -> AccessInfo;
    async fn get_pool(&self) -> Pool<Sqlite>;
    async fn get_login(&self) -> String;
    async fn get_access_info(&self) -> LoginUserInfo;
}

impl Access for AccessInfo {
    fn get_user(&self) -> User {
        self.user.clone()
    }
    async fn get_pool(&self) -> Pool<Sqlite> {
        self.pool.clone()
    }

    async fn get_login(&self) -> String {
        self.login.clone()
    }
    async fn get_access_info(&self) -> LoginUserInfo {
        self.user_access_info.clone()
    }

    async fn new(pass: &str) -> Self {
        let p = SqlitePool::connect(pass).await.unwrap();
        let u = User::get_userinfo();
        let l = LoginUserInfo::log_user_access(&p, &u.get_userid())
            .await
            .unwrap();
        let ai = LoginUserInfo::get_access_info(&p, &u.get_userid())
            .await
            .unwrap();
        Self {
            pool: p,
            user: u,
            login: l,
            user_access_info: ai,
        }
    }
}

#[derive(Debug)]
pub struct Greeting {
    message: String,
}

impl Greeting {
    pub fn new(user_name: &str, login_time: String, count_login: usize) -> Self {
        let message = format!(
            "こんにちは{}さん.現在時刻は{}です.あなたはこれまで{:?}回ログインしました．",
            user_name, login_time, count_login
        );
        Greeting { message }
    }
    pub fn get_message(&self) -> String {
        self.message.clone()
    }
}
