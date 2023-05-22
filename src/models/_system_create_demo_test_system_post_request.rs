/*
 * weclapp api
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 1
 * Contact: support@weclapp.com
 * Generated by: https://openapi-generator.tech
 */

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct SystemCreateDemoTestSystemPostRequest {
    #[serde(rename = "label")]
    pub label: String,
    #[serde(rename = "preset")]
    pub preset: Preset,
    #[serde(rename = "allUsers", skip_serializing_if = "Option::is_none")]
    pub all_users: Option<bool>,
}

impl SystemCreateDemoTestSystemPostRequest {
    pub fn new(label: String, preset: Preset) -> SystemCreateDemoTestSystemPostRequest {
        SystemCreateDemoTestSystemPostRequest {
            label,
            preset,
            all_users: None,
        }
    }
}

///
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Preset {
    #[serde(rename = "NONE")]
    None,
    #[serde(rename = "PROD_SYSTEM")]
    ProdSystem,
    #[serde(rename = "TEMPLATE_SYSTEM")]
    TemplateSystem,
}

impl Default for Preset {
    fn default() -> Preset {
        Self::None
    }
}