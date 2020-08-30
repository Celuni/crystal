/*
 * 
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 1.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LcdsGameMap {
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "displayName", skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[serde(rename = "mapId", skip_serializing_if = "Option::is_none")]
    pub map_id: Option<i32>,
    #[serde(rename = "minCustomPlayers", skip_serializing_if = "Option::is_none")]
    pub min_custom_players: Option<i32>,
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "totalPlayers", skip_serializing_if = "Option::is_none")]
    pub total_players: Option<i32>,
}

impl LcdsGameMap {
    pub fn new() -> LcdsGameMap {
        LcdsGameMap {
            description: None,
            display_name: None,
            map_id: None,
            min_custom_players: None,
            name: None,
            total_players: None,
        }
    }
}

