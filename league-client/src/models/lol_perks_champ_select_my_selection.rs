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
pub struct LolPerksChampSelectMySelection {
    #[serde(rename = "selectedSkinId", skip_serializing_if = "Option::is_none")]
    pub selected_skin_id: Option<i32>,
    #[serde(rename = "spell1Id", skip_serializing_if = "Option::is_none")]
    pub spell1_id: Option<i64>,
    #[serde(rename = "spell2Id", skip_serializing_if = "Option::is_none")]
    pub spell2_id: Option<i64>,
    #[serde(rename = "wardSkinId", skip_serializing_if = "Option::is_none")]
    pub ward_skin_id: Option<i64>,
}

impl LolPerksChampSelectMySelection {
    pub fn new() -> LolPerksChampSelectMySelection {
        LolPerksChampSelectMySelection {
            selected_skin_id: None,
            spell1_id: None,
            spell2_id: None,
            ward_skin_id: None,
        }
    }
}


