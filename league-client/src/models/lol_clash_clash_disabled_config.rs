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
pub struct LolClashClashDisabledConfig {
    #[serde(rename = "disabledReason", skip_serializing_if = "Option::is_none")]
    pub disabled_reason: Option<String>,
    #[serde(rename = "estimatedEnableTimeMillis", skip_serializing_if = "Option::is_none")]
    pub estimated_enable_time_millis: Option<i64>,
}

impl LolClashClashDisabledConfig {
    pub fn new() -> LolClashClashDisabledConfig {
        LolClashClashDisabledConfig {
            disabled_reason: None,
            estimated_enable_time_millis: None,
        }
    }
}


