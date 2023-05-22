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
pub struct CashAccount {
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "version", skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    #[serde(rename = "accountId", skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    #[serde(rename = "active", skip_serializing_if = "Option::is_none")]
    pub active: Option<bool>,
    #[serde(rename = "createdDate", skip_serializing_if = "Option::is_none")]
    pub created_date: Option<i32>,
    #[serde(rename = "currencyId")]
    pub currency_id: String,
    #[serde(rename = "currencyName", skip_serializing_if = "Option::is_none")]
    pub currency_name: Option<String>,
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "lastModifiedDate", skip_serializing_if = "Option::is_none")]
    pub last_modified_date: Option<i32>,
    #[serde(rename = "openingBalance", skip_serializing_if = "Option::is_none")]
    pub opening_balance: Option<crate::models::custom_attribute_definition::AttributeType>,
    #[serde(rename = "treasurerId")]
    pub treasurer_id: String,
}

impl CashAccount {
    pub fn new(currency_id: String, treasurer_id: String) -> CashAccount {
        CashAccount {
            id: None,
            version: None,
            account_id: None,
            active: None,
            created_date: None,
            currency_id,
            currency_name: None,
            description: None,
            last_modified_date: None,
            opening_balance: None,
            treasurer_id,
        }
    }
}
