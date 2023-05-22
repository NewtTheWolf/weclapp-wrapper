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
pub struct BlanketPurchaseOrderStatusHistory {
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub status: Option<Status>,
    #[serde(rename = "statusDate", skip_serializing_if = "Option::is_none")]
    pub status_date: Option<i32>,
    #[serde(rename = "userId", skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
}

impl BlanketPurchaseOrderStatusHistory {
    pub fn new() -> BlanketPurchaseOrderStatusHistory {
        BlanketPurchaseOrderStatusHistory {
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
    #[serde(rename = "CONFIRMED")]
    Confirmed,
    #[serde(rename = "DOCUMENT_PRINTED")]
    DocumentPrinted,
    #[serde(rename = "NEW")]
    New,
}

impl Default for Status {
    fn default() -> Status {
        Self::Cancelled
    }
}
