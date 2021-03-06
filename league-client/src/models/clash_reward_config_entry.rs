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
pub struct ClashRewardConfigEntry {
    #[serde(rename = "key", skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    #[serde(rename = "vals", skip_serializing_if = "Option::is_none")]
    pub vals: Option<Vec<crate::models::ClashRewardOutput>>,
}

impl ClashRewardConfigEntry {
    pub fn new() -> ClashRewardConfigEntry {
        ClashRewardConfigEntry {
            key: None,
            vals: None,
        }
    }
}


