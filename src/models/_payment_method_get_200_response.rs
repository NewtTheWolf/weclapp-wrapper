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
pub struct PaymentMethodGet200Response {
    #[serde(rename = "result", skip_serializing_if = "Option::is_none")]
    pub result: Option<Vec<crate::models::PaymentMethod>>,
}

impl PaymentMethodGet200Response {
    pub fn new() -> PaymentMethodGet200Response {
        PaymentMethodGet200Response { result: None }
    }
}
