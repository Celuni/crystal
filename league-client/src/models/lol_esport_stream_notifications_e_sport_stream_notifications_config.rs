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
pub struct LolEsportStreamNotificationsESportStreamNotificationsConfig {
    #[serde(rename = "beappFailureLongPollMinutes", skip_serializing_if = "Option::is_none")]
    pub beapp_failure_long_poll_minutes: Option<i64>,
    #[serde(rename = "notificationsAssetMagickURL", skip_serializing_if = "Option::is_none")]
    pub notifications_asset_magick_url: Option<String>,
    #[serde(rename = "notificationsEnabled", skip_serializing_if = "Option::is_none")]
    pub notifications_enabled: Option<bool>,
    #[serde(rename = "notificationsLongPollMinutes", skip_serializing_if = "Option::is_none")]
    pub notifications_long_poll_minutes: Option<i64>,
    #[serde(rename = "notificationsServiceEndpoint", skip_serializing_if = "Option::is_none")]
    pub notifications_service_endpoint: Option<String>,
    #[serde(rename = "notificationsServiceEndpointV2", skip_serializing_if = "Option::is_none")]
    pub notifications_service_endpoint_v2: Option<String>,
    #[serde(rename = "notificationsShortPollMinutes", skip_serializing_if = "Option::is_none")]
    pub notifications_short_poll_minutes: Option<i64>,
    #[serde(rename = "notificationsStreamGroupSlug", skip_serializing_if = "Option::is_none")]
    pub notifications_stream_group_slug: Option<String>,
    #[serde(rename = "notificationsStreamURL", skip_serializing_if = "Option::is_none")]
    pub notifications_stream_url: Option<String>,
    #[serde(rename = "useServiceEndpointV2", skip_serializing_if = "Option::is_none")]
    pub use_service_endpoint_v2: Option<bool>,
}

impl LolEsportStreamNotificationsESportStreamNotificationsConfig {
    pub fn new() -> LolEsportStreamNotificationsESportStreamNotificationsConfig {
        LolEsportStreamNotificationsESportStreamNotificationsConfig {
            beapp_failure_long_poll_minutes: None,
            notifications_asset_magick_url: None,
            notifications_enabled: None,
            notifications_long_poll_minutes: None,
            notifications_service_endpoint: None,
            notifications_service_endpoint_v2: None,
            notifications_short_poll_minutes: None,
            notifications_stream_group_slug: None,
            notifications_stream_url: None,
            use_service_endpoint_v2: None,
        }
    }
}


