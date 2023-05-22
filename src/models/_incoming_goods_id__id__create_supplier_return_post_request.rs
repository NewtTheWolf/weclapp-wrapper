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
pub struct IncomingGoodsIdIdCreateSupplierReturnPostRequest {
    #[serde(rename = "supplierId")]
    pub supplier_id: String,
    #[serde(rename = "purchaseOrderId", skip_serializing_if = "Option::is_none")]
    pub purchase_order_id: Option<String>,
}

impl IncomingGoodsIdIdCreateSupplierReturnPostRequest {
    pub fn new(supplier_id: String) -> IncomingGoodsIdIdCreateSupplierReturnPostRequest {
        IncomingGoodsIdIdCreateSupplierReturnPostRequest {
            supplier_id,
            purchase_order_id: None,
        }
    }
}
