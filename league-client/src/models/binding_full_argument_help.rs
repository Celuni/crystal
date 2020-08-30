/*
 * 
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 1.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// BindingFullArgumentHelp : Describes a function parameter.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BindingFullArgumentHelp {
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "optional", skip_serializing_if = "Option::is_none")]
    pub optional: Option<bool>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub _type: Option<crate::models::BindingFullTypeIdentifier>,
}

impl BindingFullArgumentHelp {
    /// Describes a function parameter.
    pub fn new() -> BindingFullArgumentHelp {
        BindingFullArgumentHelp {
            description: None,
            name: None,
            optional: None,
            _type: None,
        }
    }
}

