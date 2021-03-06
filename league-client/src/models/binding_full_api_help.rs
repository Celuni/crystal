/*
 * 
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 1.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// BindingFullApiHelp : Describes the exposed native API.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BindingFullApiHelp {
    #[serde(rename = "events", skip_serializing_if = "Option::is_none")]
    pub events: Option<Vec<crate::models::BindingFullEventHelp>>,
    #[serde(rename = "functions", skip_serializing_if = "Option::is_none")]
    pub functions: Option<Vec<crate::models::BindingFullFunctionHelp>>,
    #[serde(rename = "types", skip_serializing_if = "Option::is_none")]
    pub types: Option<Vec<crate::models::BindingFullTypeHelp>>,
}

impl BindingFullApiHelp {
    /// Describes the exposed native API.
    pub fn new() -> BindingFullApiHelp {
        BindingFullApiHelp {
            events: None,
            functions: None,
            types: None,
        }
    }
}


