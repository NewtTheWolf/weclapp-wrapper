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
pub struct TermOfPaymentCondition {
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "version", skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    #[serde(rename = "createdDate", skip_serializing_if = "Option::is_none")]
    pub created_date: Option<i32>,
    #[serde(rename = "discountPercentage", skip_serializing_if = "Option::is_none")]
    pub discount_percentage: Option<crate::models::custom_attribute_definition::AttributeType>,
    #[serde(rename = "lastModifiedDate", skip_serializing_if = "Option::is_none")]
    pub last_modified_date: Option<i32>,
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "numberOfDays", skip_serializing_if = "Option::is_none")]
    pub number_of_days: Option<i32>,
}

impl TermOfPaymentCondition {
    pub fn new() -> TermOfPaymentCondition {
        TermOfPaymentCondition {
            id: None,
            version: None,
            created_date: None,
            discount_percentage: None,
            last_modified_date: None,
            name: None,
            number_of_days: None,
        }
    }
}
