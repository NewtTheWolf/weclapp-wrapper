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
pub struct ArticleCalculationPrice {
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "version", skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    #[serde(rename = "articleCalculationPriceType")]
    pub article_calculation_price_type: ArticleCalculationPriceType,
    #[serde(rename = "createdDate", skip_serializing_if = "Option::is_none")]
    pub created_date: Option<i32>,
    #[serde(rename = "endDate", skip_serializing_if = "Option::is_none")]
    pub end_date: Option<i32>,
    #[serde(rename = "lastModifiedDate", skip_serializing_if = "Option::is_none")]
    pub last_modified_date: Option<i32>,
    #[serde(rename = "positionNumber", skip_serializing_if = "Option::is_none")]
    pub position_number: Option<i32>,
    #[serde(rename = "price")]
    pub price: crate::models::custom_attribute_definition::AttributeType,
    #[serde(rename = "salesChannel", skip_serializing_if = "Option::is_none")]
    pub sales_channel: Option<String>,
    #[serde(rename = "startDate", skip_serializing_if = "Option::is_none")]
    pub start_date: Option<i32>,
}

impl ArticleCalculationPrice {
    pub fn new(
        article_calculation_price_type: ArticleCalculationPriceType,
        price: crate::models::custom_attribute_definition::AttributeType,
    ) -> ArticleCalculationPrice {
        ArticleCalculationPrice {
            id: None,
            version: None,
            article_calculation_price_type,
            created_date: None,
            end_date: None,
            last_modified_date: None,
            position_number: None,
            price,
            sales_channel: None,
            start_date: None,
        }
    }
}

///
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum ArticleCalculationPriceType {
    #[serde(rename = "CALCULATION_PRICE")]
    CalculationPrice,
    #[serde(rename = "RECOMMENDED_RETAIL_PRICE")]
    RecommendedRetailPrice,
}

impl Default for ArticleCalculationPriceType {
    fn default() -> ArticleCalculationPriceType {
        Self::CalculationPrice
    }
}
