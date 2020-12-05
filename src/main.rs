use rust_auth_client::{auth_module::{AuthData, get_access_token}, file_methods::read_from_file, requests::{RawHierarchy, UserData, get_hierarchy}};
use rust_auth_client::requests::get_user_info;

fn main() {
    // Для автоматического логина:
    // создать этот файл в корне проекта
    // и на 3х строчках написать логин, пароль и ключ организации
    let login_data = read_from_file("pass.txt");

    let auth_data: AuthData = get_access_token(login_data).unwrap();

    let _user_data: UserData = get_user_info(&auth_data).unwrap();

    let hierarchy_view: RawHierarchy = get_hierarchy(&auth_data).unwrap();

    println!("\n{:#?}", hierarchy_view);
}