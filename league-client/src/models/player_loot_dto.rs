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
pub struct PlayerLootDto {
    #[serde(rename = "count", skip_serializing_if = "Option::is_none")]
    pub count: Option<i32>,
    #[serde(rename = "expiryTime", skip_serializing_if = "Option::is_none")]
    pub expiry_time: Option<i64>,
    #[serde(rename = "lootName", skip_serializing_if = "Option::is_none")]
    pub loot_name: Option<String>,
    #[serde(rename = "refId", skip_serializing_if = "Option::is_none")]
    pub ref_id: Option<String>,
}

impl PlayerLootDto {
    pub fn new() -> PlayerLootDto {
        PlayerLootDto {
            count: None,
            expiry_time: None,
            loot_name: None,
            ref_id: None,
        }
    }
}


