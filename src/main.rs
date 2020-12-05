use rust_auth_client::{auth_module::{AuthData, get_access_token}, requests::{RawHierarchy, UserData, get_hierarchy}};
use rust_auth_client::requests::get_user_info;

fn main() {
    let auth_data: AuthData = get_access_token().unwrap();

    let _user_data: UserData = get_user_info(&auth_data).unwrap();

    let hierarchy_view: RawHierarchy = get_hierarchy(&auth_data).unwrap();

    println!("\n{:#?}", hierarchy_view);
}