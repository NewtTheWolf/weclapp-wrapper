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
pub struct SalesOrderIdIdCreateDropshippingPostRequest {
    #[serde(rename = "supplierId")]
    pub supplier_id: String,
}

impl SalesOrderIdIdCreateDropshippingPostRequest {
    pub fn new(supplier_id: String) -> SalesOrderIdIdCreateDropshippingPostRequest {
        SalesOrderIdIdCreateDropshippingPostRequest { supplier_id }
    }
}
