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
pub struct FinancialYearIdIdGeneratePeriodsPost200Response {
    #[serde(rename = "result", skip_serializing_if = "Option::is_none")]
    pub result: Option<Box<crate::models::FinancialYear>>,
}

impl FinancialYearIdIdGeneratePeriodsPost200Response {
    pub fn new() -> FinancialYearIdIdGeneratePeriodsPost200Response {
        FinancialYearIdIdGeneratePeriodsPost200Response { result: None }
    }
}
