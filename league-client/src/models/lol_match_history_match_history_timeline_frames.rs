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
pub struct LolMatchHistoryMatchHistoryTimelineFrames {
    #[serde(rename = "frames", skip_serializing_if = "Option::is_none")]
    pub frames: Option<Vec<crate::models::LolMatchHistoryMatchHistoryTimelineFrame>>,
}

impl LolMatchHistoryMatchHistoryTimelineFrames {
    pub fn new() -> LolMatchHistoryMatchHistoryTimelineFrames {
        LolMatchHistoryMatchHistoryTimelineFrames {
            frames: None,
        }
    }
}


