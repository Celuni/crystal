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
pub enum LolLoginLoginConnectionMode {
    #[serde(rename = "Preparing")]
    Preparing,
    #[serde(rename = "Legacy")]
    Legacy,
    #[serde(rename = "Partner")]
    Partner,
    #[serde(rename = "RiotClient")]
    RiotClient,

}



