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
pub struct EndOfGameLcdsClientReportV1 {
    #[serde(rename = "comments", skip_serializing_if = "Option::is_none")]
    pub comments: Option<String>,
    #[serde(rename = "gameId", skip_serializing_if = "Option::is_none")]
    pub game_id: Option<i64>,
    #[serde(rename = "offenderSummonerId", skip_serializing_if = "Option::is_none")]
    pub offender_summoner_id: Option<i64>,
    #[serde(rename = "offenses", skip_serializing_if = "Option::is_none")]
    pub offenses: Option<String>,
}

impl EndOfGameLcdsClientReportV1 {
    pub fn new() -> EndOfGameLcdsClientReportV1 {
        EndOfGameLcdsClientReportV1 {
            comments: None,
            game_id: None,
            offender_summoner_id: None,
            offenses: None,
        }
    }
}


