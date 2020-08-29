/*
 * 
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 1.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */


/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum LolClashFoundationError {
    #[serde(rename = "CLASH_NOT_INITIALIZED")]
    CLASHNOTINITIALIZED,
    #[serde(rename = "CLASH_DISABLED")]
    CLASHDISABLED,
    #[serde(rename = "DESERIALIZATION_FAILED")]
    DESERIALIZATIONFAILED,
    #[serde(rename = "GAMEFLOW_UNAVAILABLE")]
    GAMEFLOWUNAVAILABLE,
    #[serde(rename = "LOL_INVENTORY_NOT_READY")]
    LOLINVENTORYNOTREADY,
    #[serde(rename = "INVALID_SIMPLE_STATE_FLAG")]
    INVALIDSIMPLESTATEFLAG,

}




