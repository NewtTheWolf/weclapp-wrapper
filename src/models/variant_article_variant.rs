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
pub struct VariantArticleVariant {
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "version", skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    #[serde(rename = "articleId", skip_serializing_if = "Option::is_none")]
    pub article_id: Option<String>,
    #[serde(rename = "articleNumber", skip_serializing_if = "Option::is_none")]
    pub article_number: Option<String>,
    #[serde(rename = "attributeOptions", skip_serializing_if = "Option::is_none")]
    pub attribute_options: Option<Vec<crate::models::OnlyId>>,
    #[serde(rename = "createdDate", skip_serializing_if = "Option::is_none")]
    pub created_date: Option<i32>,
    #[serde(rename = "lastModifiedDate", skip_serializing_if = "Option::is_none")]
    pub last_modified_date: Option<i32>,
    #[serde(rename = "positionNumber", skip_serializing_if = "Option::is_none")]
    pub position_number: Option<i32>,
    #[serde(rename = "variantArticleId")]
    pub variant_article_id: String,
    #[serde(
        rename = "variantArticleNumber",
        skip_serializing_if = "Option::is_none"
    )]
    pub variant_article_number: Option<String>,
}

impl VariantArticleVariant {
    pub fn new(variant_article_id: String) -> VariantArticleVariant {
        VariantArticleVariant {
            id: None,
            version: None,
            article_id: None,
            article_number: None,
            attribute_options: None,
            created_date: None,
            last_modified_date: None,
            position_number: None,
            variant_article_id,
            variant_article_number: None,
        }
    }
}
