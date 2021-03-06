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
pub struct LolClashRosterStats {
    #[serde(rename = "endTimeMs", skip_serializing_if = "Option::is_none")]
    pub end_time_ms: Option<i64>,
    #[serde(rename = "periodStats", skip_serializing_if = "Option::is_none")]
    pub period_stats: Option<Vec<crate::models::LolClashRosterPeriodAggregatedStats>>,
    #[serde(rename = "playerStats", skip_serializing_if = "Option::is_none")]
    pub player_stats: Option<::std::collections::HashMap<String, crate::models::LolClashRosterPlayerAggregatedStats>>,
    #[serde(rename = "rosterIconColorId", skip_serializing_if = "Option::is_none")]
    pub roster_icon_color_id: Option<i32>,
    #[serde(rename = "rosterIconId", skip_serializing_if = "Option::is_none")]
    pub roster_icon_id: Option<i32>,
    #[serde(rename = "rosterId", skip_serializing_if = "Option::is_none")]
    pub roster_id: Option<i64>,
    #[serde(rename = "rosterName", skip_serializing_if = "Option::is_none")]
    pub roster_name: Option<String>,
    #[serde(rename = "rosterShortName", skip_serializing_if = "Option::is_none")]
    pub roster_short_name: Option<String>,
    #[serde(rename = "startTimeMs", skip_serializing_if = "Option::is_none")]
    pub start_time_ms: Option<i64>,
    #[serde(rename = "tier", skip_serializing_if = "Option::is_none")]
    pub tier: Option<i32>,
    #[serde(rename = "tournamentNameLocKey", skip_serializing_if = "Option::is_none")]
    pub tournament_name_loc_key: Option<String>,
    #[serde(rename = "tournamentNameLocKeySecondary", skip_serializing_if = "Option::is_none")]
    pub tournament_name_loc_key_secondary: Option<String>,
    #[serde(rename = "tournamentPeriods", skip_serializing_if = "Option::is_none")]
    pub tournament_periods: Option<i32>,
    #[serde(rename = "tournamentThemeId", skip_serializing_if = "Option::is_none")]
    pub tournament_theme_id: Option<i32>,
}

impl LolClashRosterStats {
    pub fn new() -> LolClashRosterStats {
        LolClashRosterStats {
            end_time_ms: None,
            period_stats: None,
            player_stats: None,
            roster_icon_color_id: None,
            roster_icon_id: None,
            roster_id: None,
            roster_name: None,
            roster_short_name: None,
            start_time_ms: None,
            tier: None,
            tournament_name_loc_key: None,
            tournament_name_loc_key_secondary: None,
            tournament_periods: None,
            tournament_theme_id: None,
        }
    }
}


