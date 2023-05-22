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
pub struct VariantArticle {
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "version", skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    #[serde(rename = "createdDate", skip_serializing_if = "Option::is_none")]
    pub created_date: Option<i32>,
    #[serde(rename = "lastModifiedDate", skip_serializing_if = "Option::is_none")]
    pub last_modified_date: Option<i32>,
    #[serde(rename = "primaryArticleId", skip_serializing_if = "Option::is_none")]
    pub primary_article_id: Option<String>,
    #[serde(
        rename = "primaryArticleNumber",
        skip_serializing_if = "Option::is_none"
    )]
    pub primary_article_number: Option<String>,
    #[serde(rename = "variantArticleName", skip_serializing_if = "Option::is_none")]
    pub variant_article_name: Option<String>,
    #[serde(rename = "variantArticleNumber")]
    pub variant_article_number: String,
    #[serde(rename = "variants", skip_serializing_if = "Option::is_none")]
    pub variants: Option<Vec<crate::models::VariantArticleVariantWithoutReference>>,
}

impl VariantArticle {
    pub fn new(variant_article_number: String) -> VariantArticle {
        VariantArticle {
            id: None,
            version: None,
            created_date: None,
            last_modified_date: None,
            primary_article_id: None,
            primary_article_number: None,
            variant_article_name: None,
            variant_article_number,
            variants: None,
        }
    }
}