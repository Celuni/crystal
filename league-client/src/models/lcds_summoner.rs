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
pub struct LcdsSummoner {
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "sumId", skip_serializing_if = "Option::is_none")]
    pub sum_id: Option<i64>,
}

impl LcdsSummoner {
    pub fn new() -> LcdsSummoner {
        LcdsSummoner {
            name: None,
            sum_id: None,
        }
    }
}


