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
pub struct LolChampSelectLegacyGameflowGameClient {
    #[serde(rename = "running", skip_serializing_if = "Option::is_none")]
    pub running: Option<bool>,
    #[serde(rename = "visible", skip_serializing_if = "Option::is_none")]
    pub visible: Option<bool>,
}

impl LolChampSelectLegacyGameflowGameClient {
    pub fn new() -> LolChampSelectLegacyGameflowGameClient {
        LolChampSelectLegacyGameflowGameClient {
            running: None,
            visible: None,
        }
    }
}

