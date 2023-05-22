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
pub struct PurchaseInvoice {
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "version", skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    #[serde(rename = "bookingText", skip_serializing_if = "Option::is_none")]
    pub booking_text: Option<String>,
    #[serde(rename = "commercialLanguage", skip_serializing_if = "Option::is_none")]
    pub commercial_language: Option<String>,
    #[serde(rename = "costCenterId", skip_serializing_if = "Option::is_none")]
    pub cost_center_id: Option<String>,
    #[serde(rename = "costCenterNumber", skip_serializing_if = "Option::is_none")]
    pub cost_center_number: Option<String>,
    #[serde(rename = "createdDate", skip_serializing_if = "Option::is_none")]
    pub created_date: Option<i32>,
    #[serde(
        rename = "currencyConversionDate",
        skip_serializing_if = "Option::is_none"
    )]
    pub currency_conversion_date: Option<i32>,
    #[serde(
        rename = "currencyConversionRate",
        skip_serializing_if = "Option::is_none"
    )]
    pub currency_conversion_rate: Option<crate::models::custom_attribute_definition::AttributeType>,
    #[serde(rename = "customAttributes", skip_serializing_if = "Option::is_none")]
    pub custom_attributes: Option<Vec<crate::models::CustomAttribute>>,
    #[serde(rename = "deliveryAddress", skip_serializing_if = "Option::is_none")]
    pub delivery_address: Option<Box<crate::models::RecordAddress>>,
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(
        rename = "disableEmailTemplate",
        skip_serializing_if = "Option::is_none"
    )]
    pub disable_email_template: Option<bool>,
    #[serde(rename = "dueDate", skip_serializing_if = "Option::is_none")]
    pub due_date: Option<i32>,
    #[serde(rename = "grossAmount", skip_serializing_if = "Option::is_none")]
    pub gross_amount: Option<crate::models::custom_attribute_definition::AttributeType>,
    #[serde(
        rename = "grossAmountInCompanyCurrency",
        skip_serializing_if = "Option::is_none"
    )]
    pub gross_amount_in_company_currency:
        Option<crate::models::custom_attribute_definition::AttributeType>,
    #[serde(rename = "headerDiscount", skip_serializing_if = "Option::is_none")]
    pub header_discount: Option<crate::models::custom_attribute_definition::AttributeType>,
    #[serde(rename = "headerSurcharge", skip_serializing_if = "Option::is_none")]
    pub header_surcharge: Option<crate::models::custom_attribute_definition::AttributeType>,
    #[serde(
        rename = "internalInvoiceNumber",
        skip_serializing_if = "Option::is_none"
    )]
    pub internal_invoice_number: Option<String>,
    #[serde(rename = "invoiceAddress", skip_serializing_if = "Option::is_none")]
    pub invoice_address: Option<Box<crate::models::RecordAddress>>,
    #[serde(rename = "invoiceDate", skip_serializing_if = "Option::is_none")]
    pub invoice_date: Option<i32>,
    #[serde(rename = "invoiceNumber", skip_serializing_if = "Option::is_none")]
    pub invoice_number: Option<String>,
    #[serde(rename = "lastModifiedDate", skip_serializing_if = "Option::is_none")]
    pub last_modified_date: Option<i32>,
    #[serde(rename = "netAmount", skip_serializing_if = "Option::is_none")]
    pub net_amount: Option<crate::models::custom_attribute_definition::AttributeType>,
    #[serde(
        rename = "netAmountInCompanyCurrency",
        skip_serializing_if = "Option::is_none"
    )]
    pub net_amount_in_company_currency:
        Option<crate::models::custom_attribute_definition::AttributeType>,
    #[serde(rename = "nonStandardTaxId", skip_serializing_if = "Option::is_none")]
    pub non_standard_tax_id: Option<String>,
    #[serde(rename = "nonStandardTaxName", skip_serializing_if = "Option::is_none")]
    pub non_standard_tax_name: Option<String>,
    #[serde(rename = "paymentMethodId", skip_serializing_if = "Option::is_none")]
    pub payment_method_id: Option<String>,
    #[serde(rename = "paymentMethodName", skip_serializing_if = "Option::is_none")]
    pub payment_method_name: Option<String>,
    #[serde(rename = "paymentStatus")]
    pub payment_status: PaymentStatus,
    #[serde(rename = "pricingDate", skip_serializing_if = "Option::is_none")]
    pub pricing_date: Option<i32>,
    #[serde(
        rename = "purchaseInvoiceItems",
        skip_serializing_if = "Option::is_none"
    )]
    pub purchase_invoice_items: Option<Vec<crate::models::PurchaseInvoiceItem>>,
    #[serde(
        rename = "purchaseInvoiceType",
        skip_serializing_if = "Option::is_none"
    )]
    pub purchase_invoice_type: Option<PurchaseInvoiceType>,
    #[serde(rename = "purchaseOrders", skip_serializing_if = "Option::is_none")]
    pub purchase_orders: Option<Vec<crate::models::OnlyId>>,
    #[serde(rename = "recordAddress", skip_serializing_if = "Option::is_none")]
    pub record_address: Option<Box<crate::models::RecordAddress>>,
    #[serde(rename = "recordComment", skip_serializing_if = "Option::is_none")]
    pub record_comment: Option<String>,
    #[serde(rename = "recordCurrencyId", skip_serializing_if = "Option::is_none")]
    pub record_currency_id: Option<String>,
    #[serde(rename = "recordCurrencyName", skip_serializing_if = "Option::is_none")]
    pub record_currency_name: Option<String>,
    #[serde(
        rename = "recordEmailAddresses",
        skip_serializing_if = "Option::is_none"
    )]
    pub record_email_addresses: Option<Box<crate::models::EmailAddresses>>,
    #[serde(rename = "recordFreeText", skip_serializing_if = "Option::is_none")]
    pub record_free_text: Option<String>,
    #[serde(rename = "recordOpening", skip_serializing_if = "Option::is_none")]
    pub record_opening: Option<String>,
    #[serde(rename = "responsibleUserId", skip_serializing_if = "Option::is_none")]
    pub responsible_user_id: Option<String>,
    #[serde(
        rename = "responsibleUserUsername",
        skip_serializing_if = "Option::is_none"
    )]
    pub responsible_user_username: Option<String>,
    #[serde(rename = "senderCountryCode", skip_serializing_if = "Option::is_none")]
    pub sender_country_code: Option<String>,
    #[serde(rename = "sentToRecipient", skip_serializing_if = "Option::is_none")]
    pub sent_to_recipient: Option<bool>,
    #[serde(rename = "servicePeriodFrom", skip_serializing_if = "Option::is_none")]
    pub service_period_from: Option<i32>,
    #[serde(rename = "servicePeriodTo", skip_serializing_if = "Option::is_none")]
    pub service_period_to: Option<i32>,
    #[serde(rename = "shippingCostItems", skip_serializing_if = "Option::is_none")]
    pub shipping_cost_items: Option<Vec<crate::models::PurchaseShippingCostItem>>,
    #[serde(rename = "status")]
    pub status: Status,
    #[serde(rename = "statusHistory", skip_serializing_if = "Option::is_none")]
    pub status_history: Option<Vec<crate::models::PurchaseInvoiceStatusHistory>>,
    #[serde(
        rename = "supplierHabitualExporterLetterOfIntentId",
        skip_serializing_if = "Option::is_none"
    )]
    pub supplier_habitual_exporter_letter_of_intent_id: Option<String>,
    #[serde(rename = "supplierId")]
    pub supplier_id: String,
    #[serde(rename = "supplierNumber", skip_serializing_if = "Option::is_none")]
    pub supplier_number: Option<String>,
    #[serde(rename = "tags", skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,
    #[serde(rename = "termOfPaymentId", skip_serializing_if = "Option::is_none")]
    pub term_of_payment_id: Option<String>,
    #[serde(rename = "termOfPaymentName", skip_serializing_if = "Option::is_none")]
    pub term_of_payment_name: Option<String>,
}

impl PurchaseInvoice {
    pub fn new(
        payment_status: PaymentStatus,
        status: Status,
        supplier_id: String,
    ) -> PurchaseInvoice {
        PurchaseInvoice {
            id: None,
            version: None,
            booking_text: None,
            commercial_language: None,
            cost_center_id: None,
            cost_center_number: None,
            created_date: None,
            currency_conversion_date: None,
            currency_conversion_rate: None,
            custom_attributes: None,
            delivery_address: None,
            description: None,
            disable_email_template: None,
            due_date: None,
            gross_amount: None,
            gross_amount_in_company_currency: None,
            header_discount: None,
            header_surcharge: None,
            internal_invoice_number: None,
            invoice_address: None,
            invoice_date: None,
            invoice_number: None,
            last_modified_date: None,
            net_amount: None,
            net_amount_in_company_currency: None,
            non_standard_tax_id: None,
            non_standard_tax_name: None,
            payment_method_id: None,
            payment_method_name: None,
            payment_status,
            pricing_date: None,
            purchase_invoice_items: None,
            purchase_invoice_type: None,
            purchase_orders: None,
            record_address: None,
            record_comment: None,
            record_currency_id: None,
            record_currency_name: None,
            record_email_addresses: None,
            record_free_text: None,
            record_opening: None,
            responsible_user_id: None,
            responsible_user_username: None,
            sender_country_code: None,
            sent_to_recipient: None,
            service_period_from: None,
            service_period_to: None,
            shipping_cost_items: None,
            status,
            status_history: None,
            supplier_habitual_exporter_letter_of_intent_id: None,
            supplier_id,
            supplier_number: None,
            tags: None,
            term_of_payment_id: None,
            term_of_payment_name: None,
        }
    }
}

///
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum PaymentStatus {
    #[serde(rename = "CLEARED_WITH_CREDIT_NOTE")]
    ClearedWithCreditNote,
    #[serde(rename = "CREDIT_NOTE_CLEARED")]
    CreditNoteCleared,
    #[serde(rename = "NO_OPEN_ITEM")]
    NoOpenItem,
    #[serde(rename = "OPEN")]
    Open,
    #[serde(rename = "PAID")]
    Paid,
    #[serde(rename = "UNKNOWN")]
    Unknown,
}

impl Default for PaymentStatus {
    fn default() -> PaymentStatus {
        Self::ClearedWithCreditNote
    }
}
///
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum PurchaseInvoiceType {
    #[serde(rename = "CREDIT_NOTE")]
    CreditNote,
    #[serde(rename = "STANDARD_INVOICE")]
    StandardInvoice,
}

impl Default for PurchaseInvoiceType {
    fn default() -> PurchaseInvoiceType {
        Self::CreditNote
    }
}
///
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Status {
    #[serde(rename = "ENTRY_COMPLETED")]
    EntryCompleted,
    #[serde(rename = "INVOICE_CHECKED")]
    InvoiceChecked,
    #[serde(rename = "INVOICE_VERIFICATION")]
    InvoiceVerification,
    #[serde(rename = "NEW")]
    New,
    #[serde(rename = "OPEN_ITEM_CREATED")]
    OpenItemCreated,
    #[serde(rename = "VOID")]
    Void,
}

impl Default for Status {
    fn default() -> Status {
        Self::EntryCompleted
    }
}