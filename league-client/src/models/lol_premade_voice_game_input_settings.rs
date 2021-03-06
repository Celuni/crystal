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
pub struct LolPremadeVoiceGameInputSettings {
    #[serde(rename = "GameEvents", skip_serializing_if = "Option::is_none")]
    pub game_events: Option<crate::models::LolPremadeVoiceGameEventHotkeys>,
}

impl LolPremadeVoiceGameInputSettings {
    pub fn new() -> LolPremadeVoiceGameInputSettings {
        LolPremadeVoiceGameInputSettings {
            game_events: None,
        }
    }
}


