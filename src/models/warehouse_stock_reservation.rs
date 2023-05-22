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
pub struct WarehouseStockReservation {
    #[serde(
        rename = "productionOrderItemId",
        skip_serializing_if = "Option::is_none"
    )]
    pub production_order_item_id: Option<String>,
    #[serde(rename = "reservedQuantity")]
    pub reserved_quantity: crate::models::custom_attribute_definition::AttributeType,
    #[serde(rename = "shipmentItemId", skip_serializing_if = "Option::is_none")]
    pub shipment_item_id: Option<String>,
}

impl WarehouseStockReservation {
    pub fn new(
        reserved_quantity: crate::models::custom_attribute_definition::AttributeType,
    ) -> WarehouseStockReservation {
        WarehouseStockReservation {
            production_order_item_id: None,
            reserved_quantity,
            shipment_item_id: None,
        }
    }
}
