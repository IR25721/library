use readname::{Fromcard, User};
mod readname;
fn main() {
    let userinfo = User::readcard();
    let user = userinfo.as_ref().expect("not found name");
    println!(
        "Your name is {:?}\nYour card ID is {:?}",
        user.get_username(),
        user.get_userid()
    );
}
