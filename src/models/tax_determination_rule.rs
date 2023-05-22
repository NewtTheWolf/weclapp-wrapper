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
pub struct TaxDeterminationRule {
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "version", skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    #[serde(rename = "accountingCodeId", skip_serializing_if = "Option::is_none")]
    pub accounting_code_id: Option<String>,
    #[serde(rename = "createdDate", skip_serializing_if = "Option::is_none")]
    pub created_date: Option<i32>,
    #[serde(
        rename = "customerDebtorAccountingCodeId",
        skip_serializing_if = "Option::is_none"
    )]
    pub customer_debtor_accounting_code_id: Option<String>,
    #[serde(
        rename = "dispatchCountryCode",
        skip_serializing_if = "Option::is_none"
    )]
    pub dispatch_country_code: Option<String>,
    #[serde(rename = "endDate", skip_serializing_if = "Option::is_none")]
    pub end_date: Option<i32>,
    #[serde(rename = "lastModifiedDate", skip_serializing_if = "Option::is_none")]
    pub last_modified_date: Option<i32>,
    #[serde(rename = "partyType", skip_serializing_if = "Option::is_none")]
    pub party_type: Option<PartyType>,
    #[serde(
        rename = "recipientCountryCode",
        skip_serializing_if = "Option::is_none"
    )]
    pub recipient_country_code: Option<String>,
    #[serde(rename = "sales", skip_serializing_if = "Option::is_none")]
    pub sales: Option<bool>,
    #[serde(rename = "startDate", skip_serializing_if = "Option::is_none")]
    pub start_date: Option<i32>,
    #[serde(rename = "taxId")]
    pub tax_id: String,
    #[serde(rename = "taxRateType", skip_serializing_if = "Option::is_none")]
    pub tax_rate_type: Option<TaxRateType>,
    #[serde(rename = "validVatId", skip_serializing_if = "Option::is_none")]
    pub valid_vat_id: Option<bool>,
}

impl TaxDeterminationRule {
    pub fn new(tax_id: String) -> TaxDeterminationRule {
        TaxDeterminationRule {
            id: None,
            version: None,
            accounting_code_id: None,
            created_date: None,
            customer_debtor_accounting_code_id: None,
            dispatch_country_code: None,
            end_date: None,
            last_modified_date: None,
            party_type: None,
            recipient_country_code: None,
            sales: None,
            start_date: None,
            tax_id,
            tax_rate_type: None,
            valid_vat_id: None,
        }
    }
}

///
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum PartyType {
    #[serde(rename = "ORGANIZATION")]
    Organization,
    #[serde(rename = "PERSON")]
    Person,
}

impl Default for PartyType {
    fn default() -> PartyType {
        Self::Organization
    }
}
///
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum TaxRateType {
    #[serde(rename = "REDUCED")]
    Reduced,
    #[serde(rename = "SLIGHTLY_REDUCED")]
    SlightlyReduced,
    #[serde(rename = "STANDARD")]
    Standard,
    #[serde(rename = "SUPER_REDUCED")]
    SuperReduced,
    #[serde(rename = "ZERO")]
    Zero,
}

impl Default for TaxRateType {
    fn default() -> TaxRateType {
        Self::Reduced
    }
}
