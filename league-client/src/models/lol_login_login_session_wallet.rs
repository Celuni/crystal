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
pub struct LolLoginLoginSessionWallet {
    #[serde(rename = "ip", skip_serializing_if = "Option::is_none")]
    pub ip: Option<i64>,
    #[serde(rename = "rp", skip_serializing_if = "Option::is_none")]
    pub rp: Option<i64>,
}

impl LolLoginLoginSessionWallet {
    pub fn new() -> LolLoginLoginSessionWallet {
        LolLoginLoginSessionWallet {
            ip: None,
            rp: None,
        }
    }
}


