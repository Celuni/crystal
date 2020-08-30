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
pub struct LolClashRosterPlayerNotification {
    #[serde(rename = "notifyReason", skip_serializing_if = "Option::is_none")]
    pub notify_reason: Option<crate::models::LolClashRosterNotifyReason>,
    #[serde(rename = "playerNotificationDTO", skip_serializing_if = "Option::is_none")]
    pub player_notification_dto: Option<crate::models::PlayerDto>,
    #[serde(rename = "roster", skip_serializing_if = "Option::is_none")]
    pub roster: Option<crate::models::RosterDto>,
    #[serde(rename = "sourcePlayerId", skip_serializing_if = "Option::is_none")]
    pub source_player_id: Option<i64>,
}

impl LolClashRosterPlayerNotification {
    pub fn new() -> LolClashRosterPlayerNotification {
        LolClashRosterPlayerNotification {
            notify_reason: None,
            player_notification_dto: None,
            roster: None,
            source_player_id: None,
        }
    }
}

