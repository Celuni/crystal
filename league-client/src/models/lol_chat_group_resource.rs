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
pub struct LolChatGroupResource {
    #[serde(rename = "collapsed", skip_serializing_if = "Option::is_none")]
    pub collapsed: Option<bool>,
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    #[serde(rename = "isLocalized", skip_serializing_if = "Option::is_none")]
    pub is_localized: Option<bool>,
    #[serde(rename = "isMetaGroup", skip_serializing_if = "Option::is_none")]
    pub is_meta_group: Option<bool>,
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "priority", skip_serializing_if = "Option::is_none")]
    pub priority: Option<i32>,
}

impl LolChatGroupResource {
    pub fn new() -> LolChatGroupResource {
        LolChatGroupResource {
            collapsed: None,
            id: None,
            is_localized: None,
            is_meta_group: None,
            name: None,
            priority: None,
        }
    }
}


