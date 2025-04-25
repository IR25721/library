use readname::User;
mod readname;
fn main() {
    let user = User::get_userinfo();
    println!(
        "Your name is {:?}\nYour card ID is {:?}",
        user.get_username(),
        user.get_userid()
    );
}
