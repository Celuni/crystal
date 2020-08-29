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
pub struct LolClashGameflowPartiesRegistrationStatus {
    #[serde(rename = "complete", skip_serializing_if = "Option::is_none")]
    pub complete: Option<bool>,
    #[serde(rename = "errorCodes", skip_serializing_if = "Option::is_none")]
    pub error_codes: Option<Vec<String>>,
}

impl LolClashGameflowPartiesRegistrationStatus {
    pub fn new() -> LolClashGameflowPartiesRegistrationStatus {
        LolClashGameflowPartiesRegistrationStatus {
            complete: None,
            error_codes: None,
        }
    }
}


