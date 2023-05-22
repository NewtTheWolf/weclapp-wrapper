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
pub struct PartyBankAccount {
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "version", skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    #[serde(rename = "accountHolder", skip_serializing_if = "Option::is_none")]
    pub account_holder: Option<String>,
    #[serde(rename = "accountNumber")]
    pub account_number: String,
    #[serde(rename = "bankCode")]
    pub bank_code: String,
    #[serde(rename = "createdDate", skip_serializing_if = "Option::is_none")]
    pub created_date: Option<i32>,
    #[serde(rename = "creditInstitute", skip_serializing_if = "Option::is_none")]
    pub credit_institute: Option<String>,
    #[serde(rename = "lastModifiedDate", skip_serializing_if = "Option::is_none")]
    pub last_modified_date: Option<i32>,
    #[serde(rename = "partyId")]
    pub party_id: String,
    #[serde(rename = "primary", skip_serializing_if = "Option::is_none")]
    pub primary: Option<bool>,
}

impl PartyBankAccount {
    pub fn new(account_number: String, bank_code: String, party_id: String) -> PartyBankAccount {
        PartyBankAccount {
            id: None,
            version: None,
            account_holder: None,
            account_number,
            bank_code,
            created_date: None,
            credit_institute: None,
            last_modified_date: None,
            party_id,
            primary: None,
        }
    }
}
