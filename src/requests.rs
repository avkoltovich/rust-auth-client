use crate::models::{RawHierarchy, UserData};

#[tokio::main]
pub async fn get_user_info(access_token: &String, user_id: u32) -> Result<UserData, Box<dyn std::error::Error>> {
    let token = format!("Bearer {}", access_token);
    let url = &format!("https://api.waliot.com/api/customers/users/{}", user_id)[..];

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
pub async fn get_hierarchy(access_token: &String, org_id: u32) -> Result<RawHierarchy, Box<dyn std::error::Error>> {
    let token = format!("Bearer {}", access_token);
    let url = &format!("https://api.waliot.com/api/customers/organizations/{}/hierarchy-view", org_id)[..];

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
