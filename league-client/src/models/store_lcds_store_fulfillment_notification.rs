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
pub struct StoreLcdsStoreFulfillmentNotification {
    #[serde(rename = "data", skip_serializing_if = "Option::is_none")]
    pub data: Option<serde_json::Value>,
    #[serde(rename = "inventoryType", skip_serializing_if = "Option::is_none")]
    pub inventory_type: Option<String>,
    #[serde(rename = "ip", skip_serializing_if = "Option::is_none")]
    pub ip: Option<i64>,
    #[serde(rename = "rp", skip_serializing_if = "Option::is_none")]
    pub rp: Option<i64>,
}

impl StoreLcdsStoreFulfillmentNotification {
    pub fn new() -> StoreLcdsStoreFulfillmentNotification {
        StoreLcdsStoreFulfillmentNotification {
            data: None,
            inventory_type: None,
            ip: None,
            rp: None,
        }
    }
}


