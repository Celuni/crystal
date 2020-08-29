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
pub struct LolClashPlayerData {
    #[serde(rename = "isClashBanned", skip_serializing_if = "Option::is_none")]
    pub is_clash_banned: Option<bool>,
    #[serde(rename = "tickets", skip_serializing_if = "Option::is_none")]
    pub tickets: Option<::std::collections::HashMap<String, i32>>,
    #[serde(rename = "tier", skip_serializing_if = "Option::is_none")]
    pub tier: Option<i32>,
}

impl LolClashPlayerData {
    pub fn new() -> LolClashPlayerData {
        LolClashPlayerData {
            is_clash_banned: None,
            tickets: None,
            tier: None,
        }
    }
}


