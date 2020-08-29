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
pub struct LolLobbyLobbyMatchmakingSearchResource {
    #[serde(rename = "errors", skip_serializing_if = "Option::is_none")]
    pub errors: Option<Vec<crate::models::LolLobbyLobbyMatchmakingSearchErrorResource>>,
    #[serde(rename = "lowPriorityData", skip_serializing_if = "Option::is_none")]
    pub low_priority_data: Option<crate::models::LolLobbyLobbyMatchmakingLowPriorityDataResource>,
    #[serde(rename = "searchState", skip_serializing_if = "Option::is_none")]
    pub search_state: Option<crate::models::LolLobbyLobbyMatchmakingSearchState>,
}

impl LolLobbyLobbyMatchmakingSearchResource {
    pub fn new() -> LolLobbyLobbyMatchmakingSearchResource {
        LolLobbyLobbyMatchmakingSearchResource {
            errors: None,
            low_priority_data: None,
            search_state: None,
        }
    }
}


