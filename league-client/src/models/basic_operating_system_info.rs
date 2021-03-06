/*
 * 
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 1.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// BasicOperatingSystemInfo : User Experience Settings Operating System Information



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BasicOperatingSystemInfo {
    #[serde(rename = "edition", skip_serializing_if = "Option::is_none")]
    pub edition: Option<String>,
    #[serde(rename = "platform", skip_serializing_if = "Option::is_none")]
    pub platform: Option<String>,
    #[serde(rename = "versionMajor", skip_serializing_if = "Option::is_none")]
    pub version_major: Option<String>,
    #[serde(rename = "versionMinor", skip_serializing_if = "Option::is_none")]
    pub version_minor: Option<String>,
}

impl BasicOperatingSystemInfo {
    /// User Experience Settings Operating System Information
    pub fn new() -> BasicOperatingSystemInfo {
        BasicOperatingSystemInfo {
            edition: None,
            platform: None,
            version_major: None,
            version_minor: None,
        }
    }
}


