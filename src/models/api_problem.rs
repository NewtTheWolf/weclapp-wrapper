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
pub struct ApiProblem {
    #[serde(rename = "detail", skip_serializing_if = "Option::is_none")]
    pub detail: Option<String>,
    #[serde(rename = "error", skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
    #[serde(rename = "instance", skip_serializing_if = "Option::is_none")]
    pub instance: Option<String>,
    #[serde(rename = "messages", skip_serializing_if = "Option::is_none")]
    pub messages: Option<Vec<serde_json::Value>>,
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub status: Option<i32>,
    #[serde(rename = "title", skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    #[serde(rename = "validationErrors", skip_serializing_if = "Option::is_none")]
    pub validation_errors: Option<Vec<crate::models::ValidationError>>,
}

impl ApiProblem {
    pub fn new() -> ApiProblem {
        ApiProblem {
            detail: None,
            error: None,
            instance: None,
            messages: None,
            status: None,
            title: None,
            r#type: None,
            validation_errors: None,
        }
    }
}