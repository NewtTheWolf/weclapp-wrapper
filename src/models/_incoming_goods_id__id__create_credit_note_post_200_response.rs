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
pub struct IncomingGoodsIdIdCreateCreditNotePost200Response {
    #[serde(rename = "result", skip_serializing_if = "Option::is_none")]
    pub result: Option<Box<crate::models::SalesInvoice>>,
}

impl IncomingGoodsIdIdCreateCreditNotePost200Response {
    pub fn new() -> IncomingGoodsIdIdCreateCreditNotePost200Response {
        IncomingGoodsIdIdCreateCreditNotePost200Response { result: None }
    }
}