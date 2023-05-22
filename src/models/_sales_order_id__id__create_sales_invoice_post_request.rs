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
pub struct SalesOrderIdIdCreateSalesInvoicePostRequest {
    #[serde(
        rename = "additionalSalesOrderIds",
        skip_serializing_if = "Option::is_none"
    )]
    pub additional_sales_order_ids: Option<Vec<String>>,
}

impl SalesOrderIdIdCreateSalesInvoicePostRequest {
    pub fn new() -> SalesOrderIdIdCreateSalesInvoicePostRequest {
        SalesOrderIdIdCreateSalesInvoicePostRequest {
            additional_sales_order_ids: None,
        }
    }
}