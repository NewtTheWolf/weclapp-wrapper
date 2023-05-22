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
pub struct SalesInvoiceIdIdAddSalesOrdersPostRequest {
    #[serde(rename = "salesOrderIds")]
    pub sales_order_ids: Vec<String>,
    #[serde(
        rename = "collectiveInvoicePositionPrintType",
        skip_serializing_if = "Option::is_none"
    )]
    pub collective_invoice_position_print_type: Option<CollectiveInvoicePositionPrintType>,
}

impl SalesInvoiceIdIdAddSalesOrdersPostRequest {
    pub fn new(sales_order_ids: Vec<String>) -> SalesInvoiceIdIdAddSalesOrdersPostRequest {
        SalesInvoiceIdIdAddSalesOrdersPostRequest {
            sales_order_ids,
            collective_invoice_position_print_type: None,
        }
    }
}

///
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum CollectiveInvoicePositionPrintType {
    #[serde(rename = "ORDER_POSITION_GROUP")]
    OrderPositionGroup,
    #[serde(rename = "PERFORMANCE_RECORD_POSITION_GROUP")]
    PerformanceRecordPositionGroup,
    #[serde(rename = "SHIPMENT_POSITION_GROUP")]
    ShipmentPositionGroup,
}

impl Default for CollectiveInvoicePositionPrintType {
    fn default() -> CollectiveInvoicePositionPrintType {
        Self::OrderPositionGroup
    }
}
