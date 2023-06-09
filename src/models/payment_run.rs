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
pub struct PaymentRun {
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "version", skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    #[serde(rename = "createdDate", skip_serializing_if = "Option::is_none")]
    pub created_date: Option<i32>,
    #[serde(rename = "lastModifiedDate", skip_serializing_if = "Option::is_none")]
    pub last_modified_date: Option<i32>,
    #[serde(rename = "paymentRunDate", skip_serializing_if = "Option::is_none")]
    pub payment_run_date: Option<i32>,
    #[serde(rename = "paymentRunItems", skip_serializing_if = "Option::is_none")]
    pub payment_run_items: Option<Vec<crate::models::PaymentRunItem>>,
    #[serde(rename = "paymentRunNumber", skip_serializing_if = "Option::is_none")]
    pub payment_run_number: Option<String>,
    #[serde(rename = "runByUserId", skip_serializing_if = "Option::is_none")]
    pub run_by_user_id: Option<String>,
    #[serde(rename = "totalAmount", skip_serializing_if = "Option::is_none")]
    pub total_amount: Option<crate::models::custom_attribute_definition::AttributeType>,
}

impl PaymentRun {
    pub fn new() -> PaymentRun {
        PaymentRun {
            id: None,
            version: None,
            created_date: None,
            last_modified_date: None,
            payment_run_date: None,
            payment_run_items: None,
            payment_run_number: None,
            run_by_user_id: None,
            total_amount: None,
        }
    }
}
