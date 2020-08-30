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
pub struct LolLoginAuthorization {
    #[serde(rename = "currentAccountId", skip_serializing_if = "Option::is_none")]
    pub current_account_id: Option<i64>,
    #[serde(rename = "currentPlatformId", skip_serializing_if = "Option::is_none")]
    pub current_platform_id: Option<String>,
    #[serde(rename = "subject", skip_serializing_if = "Option::is_none")]
    pub subject: Option<String>,
}

impl LolLoginAuthorization {
    pub fn new() -> LolLoginAuthorization {
        LolLoginAuthorization {
            current_account_id: None,
            current_platform_id: None,
            subject: None,
        }
    }
}

