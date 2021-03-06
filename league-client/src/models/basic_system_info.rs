/*
 * 
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 1.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// BasicSystemInfo : User Experience Settings System Information



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BasicSystemInfo {
    #[serde(rename = "operatingSystem", skip_serializing_if = "Option::is_none")]
    pub operating_system: Option<crate::models::BasicOperatingSystemInfo>,
    #[serde(rename = "physicalMemory", skip_serializing_if = "Option::is_none")]
    pub physical_memory: Option<i64>,
    #[serde(rename = "physicalProcessorCores", skip_serializing_if = "Option::is_none")]
    pub physical_processor_cores: Option<i32>,
    #[serde(rename = "processorSpeed", skip_serializing_if = "Option::is_none")]
    pub processor_speed: Option<i32>,
}

impl BasicSystemInfo {
    /// User Experience Settings System Information
    pub fn new() -> BasicSystemInfo {
        BasicSystemInfo {
            operating_system: None,
            physical_memory: None,
            physical_processor_cores: None,
            processor_speed: None,
        }
    }
}


