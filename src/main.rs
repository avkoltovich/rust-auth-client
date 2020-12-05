use rust_auth_client::{requests::UserData, auth_module::{AuthData, get_access_token}};
use rust_auth_client::requests::get_user_info;

fn main() {
    let auth_data: AuthData = get_access_token().unwrap();

    let user_data: UserData = get_user_info(auth_data).unwrap();

    println!("\n{:#?}", user_data);
}