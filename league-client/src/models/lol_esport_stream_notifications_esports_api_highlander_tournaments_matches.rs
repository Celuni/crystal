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
pub struct LolEsportStreamNotificationsEsportsApiHighlanderTournamentsMatches {
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "input", skip_serializing_if = "Option::is_none")]
    pub input: Option<Vec<crate::models::LolEsportStreamNotificationsEsportsApiHighlanderTournamentsRoster>>,
}

impl LolEsportStreamNotificationsEsportsApiHighlanderTournamentsMatches {
    pub fn new() -> LolEsportStreamNotificationsEsportsApiHighlanderTournamentsMatches {
        LolEsportStreamNotificationsEsportsApiHighlanderTournamentsMatches {
            id: None,
            input: None,
        }
    }
}

