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
pub struct LolNpeTutorialPathTutorialMetadata {
    #[serde(rename = "displayRewards", skip_serializing_if = "Option::is_none")]
    pub display_rewards: Option<::std::collections::HashMap<String, String>>,
    #[serde(rename = "queueId", skip_serializing_if = "Option::is_none")]
    pub queue_id: Option<String>,
    #[serde(rename = "stepNumber", skip_serializing_if = "Option::is_none")]
    pub step_number: Option<i32>,
    #[serde(rename = "useChosenChampion", skip_serializing_if = "Option::is_none")]
    pub use_chosen_champion: Option<bool>,
    #[serde(rename = "useQuickSearchMatchmaking", skip_serializing_if = "Option::is_none")]
    pub use_quick_search_matchmaking: Option<bool>,
}

impl LolNpeTutorialPathTutorialMetadata {
    pub fn new() -> LolNpeTutorialPathTutorialMetadata {
        LolNpeTutorialPathTutorialMetadata {
            display_rewards: None,
            queue_id: None,
            step_number: None,
            use_chosen_champion: None,
            use_quick_search_matchmaking: None,
        }
    }
}


