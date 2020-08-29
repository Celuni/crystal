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
pub struct LolPatchNotification {
    #[serde(rename = "data", skip_serializing_if = "Option::is_none")]
    pub data: Option<::std::collections::HashMap<String, serde_json::Value>>,
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "notificationId", skip_serializing_if = "Option::is_none")]
    pub notification_id: Option<crate::models::LolPatchNotificationId>,
}

impl LolPatchNotification {
    pub fn new() -> LolPatchNotification {
        LolPatchNotification {
            data: None,
            id: None,
            notification_id: None,
        }
    }
}


