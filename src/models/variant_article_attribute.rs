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
pub struct VariantArticleAttribute {
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "version", skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    #[serde(rename = "attributeOptions", skip_serializing_if = "Option::is_none")]
    pub attribute_options: Option<Vec<crate::models::VariantArticleAttributeOption>>,
    #[serde(rename = "createdDate", skip_serializing_if = "Option::is_none")]
    pub created_date: Option<i32>,
    #[serde(rename = "lastModifiedDate", skip_serializing_if = "Option::is_none")]
    pub last_modified_date: Option<i32>,
    #[serde(rename = "name")]
    pub name: String,
}

impl VariantArticleAttribute {
    pub fn new(name: String) -> VariantArticleAttribute {
        VariantArticleAttribute {
            id: None,
            version: None,
            attribute_options: None,
            created_date: None,
            last_modified_date: None,
            name,
        }
    }
}
