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
pub struct LcdsRemovedFromLobbyNotification {
    #[serde(rename = "removalReason", skip_serializing_if = "Option::is_none")]
    pub removal_reason: Option<crate::models::LcdsRemovalReason>,
}

impl LcdsRemovedFromLobbyNotification {
    pub fn new() -> LcdsRemovedFromLobbyNotification {
        LcdsRemovedFromLobbyNotification {
            removal_reason: None,
        }
    }
}


