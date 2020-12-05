pub mod auth_module {
    use std::collections::HashMap;
    use std::io;
    use serde::Deserialize;

    #[derive(Deserialize)]
    pub struct AuthData {
        pub access_token: String,
        pub expires_in: u32,
        pub scope: String,
        pub user_id: u32,
        pub token_type: String,
        pub org_id: u32,
        pub scopes: Vec<String>
    }

    #[tokio::main]
    pub async fn get_access_token() -> Result<AuthData, Box<dyn std::error::Error>> {
        let body = get_login_and_password();
        let org_key = get_org_key();

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
}

pub mod requests {
    use reqwest::Response;
    use serde::Deserialize;
    use crate::auth_module::AuthData;

    #[derive(Deserialize, Debug)]
    pub struct UserData {
        pub id: u32,
        pub orgId: u32,
        pub login: String,
        pub role: String,
        pub name: String,
        pub jobPosition: String,
        pub phone: String,
        pub email: String,
        pub lastAuthTime: String,
        pub comment: Option<String>,
        pub scopes: Vec<String>
    }

    #[tokio::main]
    pub async fn get_user_info(auth_data: AuthData) -> Result<UserData, Box<dyn std::error::Error>> {
        let token = format!("Bearer {}", auth_data.access_token);
        let url = &format!("https://api.waliot.com/api/customers/users/{}", auth_data.user_id)[..];

        let response = reqwest::Client::new()
            .get(url)
            .header("Authorization", token)
            .send()
            .await?
            .text()
            .await?;

        let user_info: UserData = serde_json::from_str(&response[..])?;

        Ok(user_info)
    }
}
