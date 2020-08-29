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
pub struct ClientConfigSession {
    #[serde(rename = "connections", skip_serializing_if = "Option::is_none")]
    pub connections: Option<Vec<crate::models::ClientConfigAuthenticatedConnection>>,
    #[serde(rename = "isInternal", skip_serializing_if = "Option::is_none")]
    pub is_internal: Option<bool>,
    #[serde(rename = "patchlineId", skip_serializing_if = "Option::is_none")]
    pub patchline_id: Option<String>,
    #[serde(rename = "productId", skip_serializing_if = "Option::is_none")]
    pub product_id: Option<String>,
    #[serde(rename = "version", skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

impl ClientConfigSession {
    pub fn new() -> ClientConfigSession {
        ClientConfigSession {
            connections: None,
            is_internal: None,
            patchline_id: None,
            product_id: None,
            version: None,
        }
    }
}


