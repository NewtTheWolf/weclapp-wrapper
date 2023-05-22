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
pub struct PurchaseOrderRequestOfferItemScaleValue {
    #[serde(rename = "price", skip_serializing_if = "Option::is_none")]
    pub price: Option<crate::models::custom_attribute_definition::AttributeType>,
    #[serde(rename = "scale", skip_serializing_if = "Option::is_none")]
    pub scale: Option<crate::models::custom_attribute_definition::AttributeType>,
}

impl PurchaseOrderRequestOfferItemScaleValue {
    pub fn new() -> PurchaseOrderRequestOfferItemScaleValue {
        PurchaseOrderRequestOfferItemScaleValue {
            price: None,
            scale: None,
        }
    }
}