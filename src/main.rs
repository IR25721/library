use readname::User;
use sqlx::*;
mod readname;
mod user_table;
#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    let user = User::get_userinfo();
    println!(
        "Your name is {:?}\nYour card ID is {:?}",
        user.get_username(),
        user.get_userid()
    );
    Ok(())
}
