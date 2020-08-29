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
pub struct LolLootLootGrantNotification {
    #[serde(rename = "accountId", skip_serializing_if = "Option::is_none")]
    pub account_id: Option<i64>,
    #[serde(rename = "championId", skip_serializing_if = "Option::is_none")]
    pub champion_id: Option<i32>,
    #[serde(rename = "gameId", skip_serializing_if = "Option::is_none")]
    pub game_id: Option<i64>,
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    #[serde(rename = "lootName", skip_serializing_if = "Option::is_none")]
    pub loot_name: Option<String>,
    #[serde(rename = "messageKey", skip_serializing_if = "Option::is_none")]
    pub message_key: Option<String>,
    #[serde(rename = "msgId", skip_serializing_if = "Option::is_none")]
    pub msg_id: Option<String>,
    #[serde(rename = "playerGrade", skip_serializing_if = "Option::is_none")]
    pub player_grade: Option<String>,
    #[serde(rename = "playerId", skip_serializing_if = "Option::is_none")]
    pub player_id: Option<i64>,
}

impl LolLootLootGrantNotification {
    pub fn new() -> LolLootLootGrantNotification {
        LolLootLootGrantNotification {
            account_id: None,
            champion_id: None,
            game_id: None,
            id: None,
            loot_name: None,
            message_key: None,
            msg_id: None,
            player_grade: None,
            player_id: None,
        }
    }
}


