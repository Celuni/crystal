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
pub struct LolLobbyRegistrationCredentials {
    #[serde(rename = "gameClientVersion", skip_serializing_if = "Option::is_none")]
    pub game_client_version: Option<String>,
    #[serde(rename = "inventoryToken", skip_serializing_if = "Option::is_none")]
    pub inventory_token: Option<String>,
    #[serde(rename = "inventoryTokens", skip_serializing_if = "Option::is_none")]
    pub inventory_tokens: Option<Vec<String>>,
    #[serde(rename = "rankedOverviewToken", skip_serializing_if = "Option::is_none")]
    pub ranked_overview_token: Option<String>,
    #[serde(rename = "simpleInventoryToken", skip_serializing_if = "Option::is_none")]
    pub simple_inventory_token: Option<String>,
    #[serde(rename = "summonerToken", skip_serializing_if = "Option::is_none")]
    pub summoner_token: Option<String>,
    #[serde(rename = "userInfoToken", skip_serializing_if = "Option::is_none")]
    pub user_info_token: Option<String>,
}

impl LolLobbyRegistrationCredentials {
    pub fn new() -> LolLobbyRegistrationCredentials {
        LolLobbyRegistrationCredentials {
            game_client_version: None,
            inventory_token: None,
            inventory_tokens: None,
            ranked_overview_token: None,
            simple_inventory_token: None,
            summoner_token: None,
            user_info_token: None,
        }
    }
}


