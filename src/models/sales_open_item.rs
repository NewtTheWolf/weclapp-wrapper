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
pub struct SalesOpenItem {
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "version", skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    #[serde(rename = "amount")]
    pub amount: crate::models::custom_attribute_definition::AttributeType,
    #[serde(rename = "amountDiscount")]
    pub amount_discount: crate::models::custom_attribute_definition::AttributeType,
    #[serde(rename = "amountOpen")]
    pub amount_open: crate::models::custom_attribute_definition::AttributeType,
    #[serde(rename = "amountPaid")]
    pub amount_paid: crate::models::custom_attribute_definition::AttributeType,
    #[serde(rename = "clearanceDate", skip_serializing_if = "Option::is_none")]
    pub clearance_date: Option<i32>,
    #[serde(rename = "cleared", skip_serializing_if = "Option::is_none")]
    pub cleared: Option<bool>,
    #[serde(rename = "createdDate", skip_serializing_if = "Option::is_none")]
    pub created_date: Option<i32>,
    #[serde(rename = "lastModifiedDate", skip_serializing_if = "Option::is_none")]
    pub last_modified_date: Option<i32>,
    #[serde(rename = "openItemNumber")]
    pub open_item_number: String,
    #[serde(rename = "openItemType")]
    pub open_item_type: OpenItemType,
    #[serde(
        rename = "paymentApplications",
        skip_serializing_if = "Option::is_none"
    )]
    pub payment_applications: Option<Vec<crate::models::PaymentApplication>>,
    #[serde(rename = "salesInvoiceId", skip_serializing_if = "Option::is_none")]
    pub sales_invoice_id: Option<String>,
}

impl SalesOpenItem {
    pub fn new(
        amount: crate::models::custom_attribute_definition::AttributeType,
        amount_discount: crate::models::custom_attribute_definition::AttributeType,
        amount_open: crate::models::custom_attribute_definition::AttributeType,
        amount_paid: crate::models::custom_attribute_definition::AttributeType,
        open_item_number: String,
        open_item_type: OpenItemType,
    ) -> SalesOpenItem {
        SalesOpenItem {
            id: None,
            version: None,
            amount,
            amount_discount,
            amount_open,
            amount_paid,
            clearance_date: None,
            cleared: None,
            created_date: None,
            last_modified_date: None,
            open_item_number,
            open_item_type,
            payment_applications: None,
            sales_invoice_id: None,
        }
    }
}

///
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum OpenItemType {
    #[serde(rename = "CREDITOR")]
    Creditor,
    #[serde(rename = "CREDITOR_INVERTED")]
    CreditorInverted,
    #[serde(rename = "CREDIT_NOTE_CREDITOR")]
    CreditNoteCreditor,
    #[serde(rename = "CREDIT_NOTE_CREDITOR_INVERTED")]
    CreditNoteCreditorInverted,
    #[serde(rename = "CREDIT_NOTE_DEBITOR")]
    CreditNoteDebitor,
    #[serde(rename = "CREDIT_NOTE_DEBITOR_INVERTED")]
    CreditNoteDebitorInverted,
    #[serde(rename = "DEBTOR")]
    Debtor,
    #[serde(rename = "DEBTOR_INVERTED")]
    DebtorInverted,
}

impl Default for OpenItemType {
    fn default() -> OpenItemType {
        Self::Creditor
    }
}
