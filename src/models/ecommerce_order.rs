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
pub struct EcommerceOrder {
    #[serde(rename = "ecommerceId")]
    pub ecommerce_id: String,
    #[serde(rename = "externalConnectionId")]
    pub external_connection_id: String,
}

impl EcommerceOrder {
    pub fn new(ecommerce_id: String, external_connection_id: String) -> EcommerceOrder {
        EcommerceOrder {
            ecommerce_id,
            external_connection_id,
        }
    }
}
