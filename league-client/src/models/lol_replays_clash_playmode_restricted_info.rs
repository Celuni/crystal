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
pub struct LolReplaysClashPlaymodeRestrictedInfo {
    #[serde(rename = "isRestricted", skip_serializing_if = "Option::is_none")]
    pub is_restricted: Option<bool>,
}

impl LolReplaysClashPlaymodeRestrictedInfo {
    pub fn new() -> LolReplaysClashPlaymodeRestrictedInfo {
        LolReplaysClashPlaymodeRestrictedInfo {
            is_restricted: None,
        }
    }
}


