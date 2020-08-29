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
pub struct RecofrienderNetworkConfig {
    #[serde(rename = "enabled", skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[serde(rename = "linkUrlTemplate", skip_serializing_if = "Option::is_none")]
    pub link_url_template: Option<String>,
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "unlinkUrlTemplate", skip_serializing_if = "Option::is_none")]
    pub unlink_url_template: Option<String>,
}

impl RecofrienderNetworkConfig {
    pub fn new() -> RecofrienderNetworkConfig {
        RecofrienderNetworkConfig {
            enabled: None,
            link_url_template: None,
            name: None,
            unlink_url_template: None,
        }
    }
}


