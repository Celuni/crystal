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
pub struct LolClubsClubMemberLists {
    #[serde(rename = "activeMembers", skip_serializing_if = "Option::is_none")]
    pub active_members: Option<Vec<crate::models::LolClubsClubMember>>,
    #[serde(rename = "invitedMembers", skip_serializing_if = "Option::is_none")]
    pub invited_members: Option<Vec<crate::models::LolClubsClubMember>>,
    #[serde(rename = "nominatedMembers", skip_serializing_if = "Option::is_none")]
    pub nominated_members: Option<Vec<crate::models::LolClubsClubMember>>,
    #[serde(rename = "removedMembers", skip_serializing_if = "Option::is_none")]
    pub removed_members: Option<Vec<crate::models::LolClubsClubMember>>,
}

impl LolClubsClubMemberLists {
    pub fn new() -> LolClubsClubMemberLists {
        LolClubsClubMemberLists {
            active_members: None,
            invited_members: None,
            nominated_members: None,
            removed_members: None,
        }
    }
}


