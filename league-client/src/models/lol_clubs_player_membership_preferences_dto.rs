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
pub struct LolClubsPlayerMembershipPreferencesDto {
    #[serde(rename = "primaryActiveClubKey", skip_serializing_if = "Option::is_none")]
    pub primary_active_club_key: Option<String>,
    #[serde(rename = "shareClubAffiliation", skip_serializing_if = "Option::is_none")]
    pub share_club_affiliation: Option<bool>,
}

impl LolClubsPlayerMembershipPreferencesDto {
    pub fn new() -> LolClubsPlayerMembershipPreferencesDto {
        LolClubsPlayerMembershipPreferencesDto {
            primary_active_club_key: None,
            share_club_affiliation: None,
        }
    }
}


