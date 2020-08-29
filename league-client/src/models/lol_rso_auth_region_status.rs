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
pub struct LolRsoAuthRegionStatus {
    #[serde(rename = "enabled", skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[serde(rename = "isLQFallbackAllowed", skip_serializing_if = "Option::is_none")]
    pub is_lq_fallback_allowed: Option<bool>,
    #[serde(rename = "isUserInfoEnabled", skip_serializing_if = "Option::is_none")]
    pub is_user_info_enabled: Option<bool>,
    #[serde(rename = "platformId", skip_serializing_if = "Option::is_none")]
    pub platform_id: Option<String>,
}

impl LolRsoAuthRegionStatus {
    pub fn new() -> LolRsoAuthRegionStatus {
        LolRsoAuthRegionStatus {
            enabled: None,
            is_lq_fallback_allowed: None,
            is_user_info_enabled: None,
            platform_id: None,
        }
    }
}


