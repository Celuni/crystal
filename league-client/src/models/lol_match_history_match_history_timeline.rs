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
pub struct LolMatchHistoryMatchHistoryTimeline {
    #[serde(rename = "creepsPerMinDeltas", skip_serializing_if = "Option::is_none")]
    pub creeps_per_min_deltas: Option<::std::collections::HashMap<String, f64>>,
    #[serde(rename = "csDiffPerMinDeltas", skip_serializing_if = "Option::is_none")]
    pub cs_diff_per_min_deltas: Option<::std::collections::HashMap<String, f64>>,
    #[serde(rename = "damageTakenDiffPerMinDeltas", skip_serializing_if = "Option::is_none")]
    pub damage_taken_diff_per_min_deltas: Option<::std::collections::HashMap<String, f64>>,
    #[serde(rename = "damageTakenPerMinDeltas", skip_serializing_if = "Option::is_none")]
    pub damage_taken_per_min_deltas: Option<::std::collections::HashMap<String, f64>>,
    #[serde(rename = "goldPerMinDeltas", skip_serializing_if = "Option::is_none")]
    pub gold_per_min_deltas: Option<::std::collections::HashMap<String, f64>>,
    #[serde(rename = "lane", skip_serializing_if = "Option::is_none")]
    pub lane: Option<String>,
    #[serde(rename = "participantId", skip_serializing_if = "Option::is_none")]
    pub participant_id: Option<i32>,
    #[serde(rename = "role", skip_serializing_if = "Option::is_none")]
    pub role: Option<String>,
    #[serde(rename = "xpDiffPerMinDeltas", skip_serializing_if = "Option::is_none")]
    pub xp_diff_per_min_deltas: Option<::std::collections::HashMap<String, f64>>,
    #[serde(rename = "xpPerMinDeltas", skip_serializing_if = "Option::is_none")]
    pub xp_per_min_deltas: Option<::std::collections::HashMap<String, f64>>,
}

impl LolMatchHistoryMatchHistoryTimeline {
    pub fn new() -> LolMatchHistoryMatchHistoryTimeline {
        LolMatchHistoryMatchHistoryTimeline {
            creeps_per_min_deltas: None,
            cs_diff_per_min_deltas: None,
            damage_taken_diff_per_min_deltas: None,
            damage_taken_per_min_deltas: None,
            gold_per_min_deltas: None,
            lane: None,
            participant_id: None,
            role: None,
            xp_diff_per_min_deltas: None,
            xp_per_min_deltas: None,
        }
    }
}


