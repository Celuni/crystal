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
pub struct LolMatchHistoryRecentlyPlayedSummoner {
    #[serde(rename = "championId", skip_serializing_if = "Option::is_none")]
    pub champion_id: Option<i64>,
    #[serde(rename = "gameCreationDate", skip_serializing_if = "Option::is_none")]
    pub game_creation_date: Option<String>,
    #[serde(rename = "gameId", skip_serializing_if = "Option::is_none")]
    pub game_id: Option<i64>,
    #[serde(rename = "puuid", skip_serializing_if = "Option::is_none")]
    pub puuid: Option<String>,
    #[serde(rename = "summonerId", skip_serializing_if = "Option::is_none")]
    pub summoner_id: Option<i64>,
    #[serde(rename = "summonerName", skip_serializing_if = "Option::is_none")]
    pub summoner_name: Option<String>,
    #[serde(rename = "teamId", skip_serializing_if = "Option::is_none")]
    pub team_id: Option<i64>,
}

impl LolMatchHistoryRecentlyPlayedSummoner {
    pub fn new() -> LolMatchHistoryRecentlyPlayedSummoner {
        LolMatchHistoryRecentlyPlayedSummoner {
            champion_id: None,
            game_creation_date: None,
            game_id: None,
            puuid: None,
            summoner_id: None,
            summoner_name: None,
            team_id: None,
        }
    }
}


