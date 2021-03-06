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
pub struct LolClashRegisteredRosterNotification {
    #[serde(rename = "notifyReason", skip_serializing_if = "Option::is_none")]
    pub notify_reason: Option<crate::models::LolClashRosterNotifyReason>,
    #[serde(rename = "roster", skip_serializing_if = "Option::is_none")]
    pub roster: Option<crate::models::RosterDto>,
}

impl LolClashRegisteredRosterNotification {
    pub fn new() -> LolClashRegisteredRosterNotification {
        LolClashRegisteredRosterNotification {
            notify_reason: None,
            roster: None,
        }
    }
}


