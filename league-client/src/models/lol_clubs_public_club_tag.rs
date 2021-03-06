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
pub struct LolClubsPublicClubTag {
    #[serde(rename = "clubName", skip_serializing_if = "Option::is_none")]
    pub club_name: Option<String>,
    #[serde(rename = "clubTag", skip_serializing_if = "Option::is_none")]
    pub club_tag: Option<String>,
    #[serde(rename = "summonerId", skip_serializing_if = "Option::is_none")]
    pub summoner_id: Option<i64>,
}

impl LolClubsPublicClubTag {
    pub fn new() -> LolClubsPublicClubTag {
        LolClubsPublicClubTag {
            club_name: None,
            club_tag: None,
            summoner_id: None,
        }
    }
}


