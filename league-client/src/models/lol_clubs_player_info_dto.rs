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
pub struct LolClubsPlayerInfoDto {
    #[serde(rename = "currentAccountId", skip_serializing_if = "Option::is_none")]
    pub current_account_id: Option<i64>,
    #[serde(rename = "currentPlatformId", skip_serializing_if = "Option::is_none")]
    pub current_platform_id: Option<String>,
    #[serde(rename = "summonerId", skip_serializing_if = "Option::is_none")]
    pub summoner_id: Option<i64>,
}

impl LolClubsPlayerInfoDto {
    pub fn new() -> LolClubsPlayerInfoDto {
        LolClubsPlayerInfoDto {
            current_account_id: None,
            current_platform_id: None,
            summoner_id: None,
        }
    }
}


