use rust_auth_client::{auth::auth_module::{AuthData, get_access_token}, file_io::file_io::read_from_file, file_io::file_io::store_access_token_to_file, requests::requests::UserData, requests::requests::{RawHierarchy, get_hierarchy, get_user_info}, file_io::file_io::store_refresh_token_to_file};

fn main() {
    // Для автоматического логина:
    // создать JSON файл в корне проекта
    // и заполнить в нем поля login, password, org_key
    let login_data= read_from_file("login_data.json");

    let auth_data: AuthData = get_access_token(login_data).unwrap();
    store_access_token_to_file(&auth_data.access_token).unwrap_or_else(|_| println!("Ошибка создания файла access_token.txt"));
    store_refresh_token_to_file(&auth_data.refresh_token).unwrap_or_else(|_| println!("Ошибка создания файла refresh_token.txt"));

    let _user_data: UserData = get_user_info(&auth_data).unwrap();

    let _hierarchy_view: RawHierarchy = get_hierarchy(&auth_data).unwrap();

    println!("\n{:#?}", _user_data);
}