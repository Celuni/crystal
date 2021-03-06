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
pub struct ChampSelectLcdsPlayerChampionSelectionDto {
    #[serde(rename = "championId", skip_serializing_if = "Option::is_none")]
    pub champion_id: Option<i32>,
    #[serde(rename = "selectedSkinIndex", skip_serializing_if = "Option::is_none")]
    pub selected_skin_index: Option<i32>,
    #[serde(rename = "spell1Id", skip_serializing_if = "Option::is_none")]
    pub spell1_id: Option<i32>,
    #[serde(rename = "spell2Id", skip_serializing_if = "Option::is_none")]
    pub spell2_id: Option<i32>,
    #[serde(rename = "summonerInternalName", skip_serializing_if = "Option::is_none")]
    pub summoner_internal_name: Option<String>,
}

impl ChampSelectLcdsPlayerChampionSelectionDto {
    pub fn new() -> ChampSelectLcdsPlayerChampionSelectionDto {
        ChampSelectLcdsPlayerChampionSelectionDto {
            champion_id: None,
            selected_skin_index: None,
            spell1_id: None,
            spell2_id: None,
            summoner_internal_name: None,
        }
    }
}


