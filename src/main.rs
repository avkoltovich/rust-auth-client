use std::collections::HashMap;
use std::io;
use serde::Deserialize;

#[derive(Deserialize)]
struct AuthData {
    access_token: String,
    expires_in: u32,
    scope: String,
    user_id: u32,
    token_type: String,
    org_id: u32,
    scopes: Vec<String>
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let body = get_login_and_password();
    let org_key = get_org_key();

    let client = reqwest::Client::new();
    let res = client.post("https://auth.waliot.com/uaa/oauth/token")
    .header("org_key", org_key)
    .json(&body)
    .send()
    .await?;

    let result = res.json::<AuthData>().await?;
    println!("\naccess_token:\n{:#?}", result.access_token);
    Ok(())
}

fn get_login_and_password() -> HashMap<String, String> {
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

fn get_org_key() -> String {
    println!("Ключ организации:");
    let mut org_key = String::new();
    io::stdin()
        .read_line(&mut org_key)
        .expect("Failed to read line");

    org_key.trim().to_uppercase()
}