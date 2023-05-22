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
pub struct PurchaseOrderRequestStatusHistory {
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub status: Option<Status>,
    #[serde(rename = "statusDate", skip_serializing_if = "Option::is_none")]
    pub status_date: Option<i32>,
    #[serde(rename = "userId", skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
}

impl PurchaseOrderRequestStatusHistory {
    pub fn new() -> PurchaseOrderRequestStatusHistory {
        PurchaseOrderRequestStatusHistory {
            status: None,
            status_date: None,
            user_id: None,
        }
    }
}

///
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Status {
    #[serde(rename = "CANCELLED")]
    Cancelled,
    #[serde(rename = "CLOSED")]
    Closed,
    #[serde(rename = "DOCUMENT_PRINTED")]
    DocumentPrinted,
    #[serde(rename = "NEW")]
    New,
    #[serde(rename = "OFFER_RECEIVED")]
    OfferReceived,
}

impl Default for Status {
    fn default() -> Status {
        Self::Cancelled
    }
}
