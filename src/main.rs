use rust_auth_client::{auth::auth_module::{get_access_token}, requests::requests::UserData, requests::requests::{get_user_info}};

fn main() {
    let access_token = get_access_token();

    // let _user_data: UserData = get_user_info(&access_token, 1388).unwrap();

    // let _hierarchy_view: RawHierarchy = get_hierarchy(&access_token, 4).unwrap();

    println!("\n{:#?}", _user_data);
}