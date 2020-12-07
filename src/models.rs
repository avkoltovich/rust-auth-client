use serde::Deserialize;

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
pub enum RawNode {
    Org {
    id: Option<u32>,
    level: Option<u32>,
    childs: Vec<RawNode>,
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
    childs: Vec<RawNode>,
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
    childs: Vec<RawNode>,
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
    childs: Vec<RawNode>,
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
pub struct ZonePolygon {
    pub r#type: String,
    pub coordinates: Vec<Vec<(f32, f32)>>
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct RawHierarchy {
    pub custom_tags: Vec<CustomTag>,
    pub hierarchy: RawNode,
    pub system_tags: Vec<String>
}