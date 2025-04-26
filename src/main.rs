use readname::User;
use sqlx::*;
use tokio::io::AsyncReadExt;
use user_table::register_userinfo;
mod readname;
mod user_table;
#[tokio::main]
async fn main() -> Result<()> {
    let user = User::get_userinfo();
    let register_userinfo = register_userinfo(&user);
    println!(
        "Your name is {:?}\nYour card ID is {:?}",
        user.get_username(),
        user.get_userid()
    );
    println!("登録しますか？\n登録するならEnterを，しないなら終了してください．");
    let mut stdin = tokio::io::stdin();
    let mut buffer = [0; 1];
    stdin.read_exact(&mut buffer).await?;

    // 5. 登録処理（Enterが押された場合）
    if buffer[0] == b'\n' {
        match register_userinfo.await {
            Ok(_) => println!("登録が完了しました！"),
            Err(e) => eprintln!("登録に失敗しました: {}", e),
        }
    } else {
        println!("登録をキャンセルしました");
    }
    Ok(())
}
