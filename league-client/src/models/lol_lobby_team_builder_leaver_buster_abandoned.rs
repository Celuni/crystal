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
pub struct LolLobbyTeamBuilderLeaverBusterAbandoned {
    #[serde(rename = "abandonerName", skip_serializing_if = "Option::is_none")]
    pub abandoner_name: Option<String>,
}

impl LolLobbyTeamBuilderLeaverBusterAbandoned {
    pub fn new() -> LolLobbyTeamBuilderLeaverBusterAbandoned {
        LolLobbyTeamBuilderLeaverBusterAbandoned {
            abandoner_name: None,
        }
    }
}


