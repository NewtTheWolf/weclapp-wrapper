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
pub struct PaymentApplication {
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "version", skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    #[serde(rename = "amountApplied")]
    pub amount_applied: crate::models::custom_attribute_definition::AttributeType,
    #[serde(rename = "amountAppliedOriginCurrency")]
    pub amount_applied_origin_currency: crate::models::custom_attribute_definition::AttributeType,
    #[serde(rename = "amountCostsOfMonetaryTraffic")]
    pub amount_costs_of_monetary_traffic: crate::models::custom_attribute_definition::AttributeType,
    #[serde(rename = "amountDiscountApplied")]
    pub amount_discount_applied: crate::models::custom_attribute_definition::AttributeType,
    #[serde(rename = "createdById", skip_serializing_if = "Option::is_none")]
    pub created_by_id: Option<String>,
    #[serde(rename = "createdDate", skip_serializing_if = "Option::is_none")]
    pub created_date: Option<i32>,
    #[serde(rename = "lastModifiedDate", skip_serializing_if = "Option::is_none")]
    pub last_modified_date: Option<i32>,
    #[serde(rename = "moneyTransactionId", skip_serializing_if = "Option::is_none")]
    pub money_transaction_id: Option<String>,
}

impl PaymentApplication {
    pub fn new(
        amount_applied: crate::models::custom_attribute_definition::AttributeType,
        amount_applied_origin_currency: crate::models::custom_attribute_definition::AttributeType,
        amount_costs_of_monetary_traffic: crate::models::custom_attribute_definition::AttributeType,
        amount_discount_applied: crate::models::custom_attribute_definition::AttributeType,
    ) -> PaymentApplication {
        PaymentApplication {
            id: None,
            version: None,
            amount_applied,
            amount_applied_origin_currency,
            amount_costs_of_monetary_traffic,
            amount_discount_applied,
            created_by_id: None,
            created_date: None,
            last_modified_date: None,
            money_transaction_id: None,
        }
    }
}
