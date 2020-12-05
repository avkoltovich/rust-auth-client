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
    use serde::Deserialize;
    use crate::auth_module::AuthData;

    #[derive(Deserialize, Debug)]
    #[serde(rename_all = "camelCase")]
    pub struct UserData {
        pub id: u32,
        pub org_id: u32,
        pub login: Option<String>,
        pub role: Option<String>,
        pub name: Option<String>,
        pub job_position: Option<String>,
        pub phone: Option<String>,
        pub email: Option<String>,
        pub last_auth_time: String,
        pub comment: Option<String>,
        pub scopes: Vec<String>
    }

    #[derive(Deserialize, Debug)]
    #[serde(rename_all = "camelCase")]
    pub struct CustomTag {
        pub id: u32,
        pub org_id: u32,
        pub tag: String,
        pub color: String
    }

    #[derive(Deserialize, Debug)]
    pub enum NodeType {
        ORG,
        GROUP,
        VEHICLE,
        ZONE
    }

    #[derive(Deserialize, Debug)]
    pub enum TreeType {
        OBJECTS,
        ZONES
    }

    #[derive(Deserialize, Debug)]
    #[serde(rename_all = "SCREAMING_SNAKE_CASE")]
    pub enum SystemTag {
        Analog,
        CommonSwitchable,
        Fuel,
        Ignition,
        Maintenance,
        Temperature,
        UssFuel,
        BlockingRelay,
        Frequency,
        UssLiquid
    }

    #[derive(Deserialize, Debug)]
    #[serde(tag = "type", rename_all = "UPPERCASE")]
    pub enum RawNodes {
       Org {
        id: Option<u32>,
        level: Option<u32>,
        childs: Vec<RawNodes>,
        parent_org_id: Option<u32>,
        parent_group_id: Option<u32>,
        all_object_count: Option<u32>,
        all_zone_count: Option<u32>,
        zone_count: Option<u32>,
        object_count: Option<u32>,
        short_name: Option<String>,
        full_name: Option<String>,
       },
       Zone {
        id: Option<u32>,
        level: Option<u32>,
        childs: Vec<RawNodes>,
        parent_org_id: Option<u32>,
        parent_group_id: Option<u32>,
        name: Option<String>,
        color: Option<String>,
        polygon: Option<ZonePolygon>,
        area: Option<f32>,
        perimeter: Option<f32>,
        addressing_enabled: Option<bool>
       },
       Vehicle {
        id: Option<u32>,
        level: Option<u32>,
        childs: Vec<RawNodes>,
        parent_org_id: Option<u32>,
        parent_group_id: Option<u32>,
        tracker_id: Option<u32>,
        icon: Option<String>,
        color: Option<String>,
        model: Option<String>,
        reg_number: Option<String>,
        system_tags: Option<Vec<SystemTag>>,
       },
       Group {
        id: Option<u32>,
        level: Option<u32>,
        childs: Vec<RawNodes>,
        parent_org_id: Option<u32>,
        parent_group_id: Option<u32>,
        all_object_count: Option<u32>,
        all_zone_count: Option<u32>,
        zone_count: Option<u32>,
        object_count: Option<u32>,
        name: Option<String>,
        group_type: Option<TreeType>,
       }
    }

    #[derive(Deserialize, Debug)]
    pub struct ZonePolygon {
        pub r#type: String,
        pub coordinates: Vec<Vec<(f32, f32)>>
    }

    #[derive(Deserialize, Debug)]
    #[serde(rename_all = "camelCase")]
    pub struct RawHierarchy {
        pub custom_tags: Vec<CustomTag>,
        pub hierarchy: RawNodes,
        pub system_tags: Vec<String>
    }

    #[tokio::main]
    pub async fn get_user_info(auth_data: &AuthData) -> Result<UserData, Box<dyn std::error::Error>> {
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

    #[tokio::main]
    pub async fn get_hierarchy(auth_data: &AuthData) -> Result<RawHierarchy, Box<dyn std::error::Error>> {
        let token = format!("Bearer {}", auth_data.access_token);
        let url = &format!("https://api.waliot.com/api/customers/organizations/{}/hierarchy-view", auth_data.org_id)[..];

        let response = reqwest::Client::new()
            .get(url)
            .header("Authorization", token)
            .query(&[("child-zones", "false")])
            .send()
            .await?
            .text()
            .await?;

        let hierarchy_view: RawHierarchy = serde_json::from_str(&response[..])?;

        Ok(hierarchy_view)
    }
}
