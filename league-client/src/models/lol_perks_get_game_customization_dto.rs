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
pub struct LolPerksGetGameCustomizationDto {
    #[serde(rename = "queueType", skip_serializing_if = "Option::is_none")]
    pub queue_type: Option<String>,
}

impl LolPerksGetGameCustomizationDto {
    pub fn new() -> LolPerksGetGameCustomizationDto {
        LolPerksGetGameCustomizationDto {
            queue_type: None,
        }
    }
}

