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
pub struct LolMissionsMissionsNotificationResource {
    #[serde(rename = "backgroundUrl", skip_serializing_if = "Option::is_none")]
    pub background_url: Option<String>,
    #[serde(rename = "created", skip_serializing_if = "Option::is_none")]
    pub created: Option<String>,
    #[serde(rename = "critical", skip_serializing_if = "Option::is_none")]
    pub critical: Option<bool>,
    #[serde(rename = "data", skip_serializing_if = "Option::is_none")]
    pub data: Option<::std::collections::HashMap<String, String>>,
    #[serde(rename = "detailKey", skip_serializing_if = "Option::is_none")]
    pub detail_key: Option<String>,
    #[serde(rename = "expires", skip_serializing_if = "Option::is_none")]
    pub expires: Option<String>,
    #[serde(rename = "iconUrl", skip_serializing_if = "Option::is_none")]
    pub icon_url: Option<String>,
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    #[serde(rename = "source", skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
    #[serde(rename = "state", skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    #[serde(rename = "titleKey", skip_serializing_if = "Option::is_none")]
    pub title_key: Option<String>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub _type: Option<String>,
}

impl LolMissionsMissionsNotificationResource {
    pub fn new() -> LolMissionsMissionsNotificationResource {
        LolMissionsMissionsNotificationResource {
            background_url: None,
            created: None,
            critical: None,
            data: None,
            detail_key: None,
            expires: None,
            icon_url: None,
            id: None,
            source: None,
            state: None,
            title_key: None,
            _type: None,
        }
    }
}


