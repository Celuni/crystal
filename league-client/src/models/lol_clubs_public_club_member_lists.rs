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
pub struct LolClubsPublicClubMemberLists {
    #[serde(rename = "activeMembers", skip_serializing_if = "Option::is_none")]
    pub active_members: Option<Vec<crate::models::LolClubsPublicClubPlayer>>,
}

impl LolClubsPublicClubMemberLists {
    pub fn new() -> LolClubsPublicClubMemberLists {
        LolClubsPublicClubMemberLists {
            active_members: None,
        }
    }
}


