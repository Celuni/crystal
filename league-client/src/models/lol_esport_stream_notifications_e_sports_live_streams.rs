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
pub struct LolEsportStreamNotificationsESportsLiveStreams {
    #[serde(rename = "liveStreams", skip_serializing_if = "Option::is_none")]
    pub live_streams: Option<Vec<crate::models::LolEsportStreamNotificationsESportsStreams>>,
}

impl LolEsportStreamNotificationsESportsLiveStreams {
    pub fn new() -> LolEsportStreamNotificationsESportsLiveStreams {
        LolEsportStreamNotificationsESportsLiveStreams {
            live_streams: None,
        }
    }
}


