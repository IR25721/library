use readname::User;
use sqlx::*;
use tokio::io::AsyncReadExt;
use user_table::{is_user_exists, register_userinfo};
mod readname;
mod user_table;
#[tokio::main]
async fn main() -> Result<()> {
    let pool = SqlitePool::connect("sqlite:database.db").await?;
    let user = User::get_userinfo();
    let user_id = user.get_userid();
    let _user_name = user.get_username();
    let register_userinfo = register_userinfo(&pool, &user);
    println!(
        "Your name is {:?}\nYour card ID is {:?}",
        user.get_username(),
        user.get_userid()
    );
    let is_exist = is_user_exists(&pool, &user_id);
    if is_exist.await? {
        todo!();
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
