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
pub struct PurchaseOrderRequestIdIdCreatePurchaseOrderPostRequest {
    #[serde(rename = "offerId")]
    pub offer_id: i32,
    #[serde(rename = "salesOrderId", skip_serializing_if = "Option::is_none")]
    pub sales_order_id: Option<i32>,
    #[serde(rename = "offerItemToUpdateSupplierInformation")]
    pub offer_item_to_update_supplier_information:
        Vec<crate::models::PurchaseOrderRequestOfferItemInformation>,
}

impl PurchaseOrderRequestIdIdCreatePurchaseOrderPostRequest {
    pub fn new(
        offer_id: i32,
        offer_item_to_update_supplier_information: Vec<
            crate::models::PurchaseOrderRequestOfferItemInformation,
        >,
    ) -> PurchaseOrderRequestIdIdCreatePurchaseOrderPostRequest {
        PurchaseOrderRequestIdIdCreatePurchaseOrderPostRequest {
            offer_id,
            sales_order_id: None,
            offer_item_to_update_supplier_information,
        }
    }
}
