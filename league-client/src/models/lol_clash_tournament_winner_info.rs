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
pub struct LolClashTournamentWinnerInfo {
    #[serde(rename = "averageWinDuration", skip_serializing_if = "Option::is_none")]
    pub average_win_duration: Option<i64>,
    #[serde(rename = "createTime", skip_serializing_if = "Option::is_none")]
    pub create_time: Option<i64>,
    #[serde(rename = "logo", skip_serializing_if = "Option::is_none")]
    pub logo: Option<i32>,
    #[serde(rename = "logoColor", skip_serializing_if = "Option::is_none")]
    pub logo_color: Option<i32>,
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "playerIds", skip_serializing_if = "Option::is_none")]
    pub player_ids: Option<Vec<i64>>,
    #[serde(rename = "rosterId", skip_serializing_if = "Option::is_none")]
    pub roster_id: Option<i64>,
    #[serde(rename = "shortName", skip_serializing_if = "Option::is_none")]
    pub short_name: Option<String>,
    #[serde(rename = "tier", skip_serializing_if = "Option::is_none")]
    pub tier: Option<i32>,
}

impl LolClashTournamentWinnerInfo {
    pub fn new() -> LolClashTournamentWinnerInfo {
        LolClashTournamentWinnerInfo {
            average_win_duration: None,
            create_time: None,
            logo: None,
            logo_color: None,
            name: None,
            player_ids: None,
            roster_id: None,
            short_name: None,
            tier: None,
        }
    }
}


