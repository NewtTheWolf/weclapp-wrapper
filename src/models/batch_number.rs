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
pub struct BatchNumber {
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "version", skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    #[serde(rename = "articleId")]
    pub article_id: String,
    #[serde(rename = "articleNumber", skip_serializing_if = "Option::is_none")]
    pub article_number: Option<String>,
    #[serde(rename = "batchNumber")]
    pub batch_number: String,
    #[serde(rename = "createdDate", skip_serializing_if = "Option::is_none")]
    pub created_date: Option<i32>,
    #[serde(rename = "customAttributes", skip_serializing_if = "Option::is_none")]
    pub custom_attributes: Option<Vec<crate::models::CustomAttribute>>,
    #[serde(rename = "expirationDate", skip_serializing_if = "Option::is_none")]
    pub expiration_date: Option<i32>,
    #[serde(rename = "lastModifiedDate", skip_serializing_if = "Option::is_none")]
    pub last_modified_date: Option<i32>,
}

impl BatchNumber {
    pub fn new(article_id: String, batch_number: String) -> BatchNumber {
        BatchNumber {
            id: None,
            version: None,
            article_id,
            article_number: None,
            batch_number,
            created_date: None,
            custom_attributes: None,
            expiration_date: None,
            last_modified_date: None,
        }
    }
}