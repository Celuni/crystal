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
pub enum ReplayResponseStatus {
    #[serde(rename = "OK")]
    OK,
    #[serde(rename = "NOT_FOUND")]
    NOTFOUND,
    #[serde(rename = "EXPIRED")]
    EXPIRED,
    #[serde(rename = "BAD_REQUEST")]
    BADREQUEST,
    #[serde(rename = "INTERNAL_SERVER_ERROR")]
    INTERNALSERVERERROR,

}




