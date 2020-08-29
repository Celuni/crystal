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
pub struct LolItemSetsValidateItemSetNameResponse {
    #[serde(rename = "nameCheckResponse", skip_serializing_if = "Option::is_none")]
    pub name_check_response: Option<crate::models::LolItemSetsNameCheckResponse>,
    #[serde(rename = "success", skip_serializing_if = "Option::is_none")]
    pub success: Option<bool>,
}

impl LolItemSetsValidateItemSetNameResponse {
    pub fn new() -> LolItemSetsValidateItemSetNameResponse {
        LolItemSetsValidateItemSetNameResponse {
            name_check_response: None,
            success: None,
        }
    }
}


