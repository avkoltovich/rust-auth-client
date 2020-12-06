use rust_auth_client::{auth::auth_module::{AuthData, get_access_token}, file_io::file_io::read_from_file, requests::requests::UserData, requests::requests::{RawHierarchy, get_hierarchy, get_user_info}};

fn main() {
    // Для автоматического логина:
    // создать JSON файл в корне проекта
    // и заполнить в нем поля login, password, org_key
    let login_data= read_from_file("login_data.json");

    let auth_data: AuthData = get_access_token(login_data).unwrap();

    let _user_data: UserData = get_user_info(&auth_data).unwrap();

    let _hierarchy_view: RawHierarchy = get_hierarchy(&auth_data).unwrap();

    println!("\n{:#?}", _user_data);
}