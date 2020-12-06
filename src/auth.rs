pub mod auth_module {
    use std::collections::HashMap;
    use std::io;
    use serde::Deserialize;

    use crate::file_io::file_io::{get_access_token_from_file, get_login_data_from_file, store_access_token_to_file, store_refresh_token_to_file};

    #[derive(Deserialize)]
    pub struct AuthData {
        pub access_token: String,
        pub refresh_token: String,
        pub expires_in: u32,
        pub scope: String,
        pub user_id: u32,
        pub token_type: String,
        pub org_id: u32,
        pub scopes: Vec<String>
    }

    #[tokio::main]
    pub async fn log_in() -> Result<AuthData, Box<dyn std::error::Error>> {
        let login_data= get_login_data_from_file();
        let mut body: HashMap<String, String> = HashMap::new();
        let org_key: String;

        if let Err(_) = &login_data {
            body = get_login_and_password();
            org_key = get_org_key();
        } else {
            let unwraped_login_data = login_data.unwrap();
            body.insert("username".to_owned(), unwraped_login_data.login);
            body.insert("password".to_owned(), unwraped_login_data.password);
            body.insert("grant_type".to_owned(), "password".to_owned());
            body.insert("client_id".to_owned(), "web".to_owned());

            org_key = unwraped_login_data.org_key.to_uppercase();
        }

        let client = reqwest::Client::new();
        let res = client.post("https://auth.waliot.com/uaa/oauth/token")
        .header("org_key", org_key)
        .json(&body)
        .send()
        .await?;

        let result = res.json::<AuthData>().await?;
        
        Ok(result)
    }

    pub fn get_login_and_password() -> HashMap<String, String> {
        println!("Логин:");
        let mut login = String::new();
        io::stdin()
            .read_line(&mut login)
            .expect("Failed to read line");

        println!("Пароль:");
        let password = rpassword::read_password_from_tty(Some("")).unwrap();
            
        let mut map = HashMap::new();
        map.insert("username".to_owned(), login.trim().to_owned());
        map.insert("password".to_owned(), password.trim().to_owned());
        map.insert("grant_type".to_owned(), "password".to_owned());
        map.insert("client_id".to_owned(), "web".to_owned());

        map
    }

    pub fn get_org_key() -> String {
        println!("Ключ организации:");
        let mut org_key = String::new();
        io::stdin()
            .read_line(&mut org_key)
            .expect("Failed to read line");

        org_key.trim().to_uppercase()
    }

    pub fn get_access_token() -> String {
        // Для автоматического логина:
        // создать JSON файл login_data.json в корне проекта
        // и заполнить в нем поля login, password, org_key

        let access_token: String;

        if let Err(_) = get_access_token_from_file() {
            let auth_data: AuthData = log_in().unwrap();

            store_access_token_to_file(&auth_data.access_token).unwrap_or_else(|_| println!("Ошибка создания файла access_token.txt"));
            store_refresh_token_to_file(&auth_data.refresh_token).unwrap_or_else(|_| println!("Ошибка создания файла refresh_token.txt"));

            access_token = get_access_token_from_file().unwrap();
        } else {
            access_token = get_access_token_from_file().unwrap();
        }

        access_token
    }
}