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
pub struct PurchaseOrderRequestOffer {
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "version", skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    #[serde(rename = "commercialLanguage", skip_serializing_if = "Option::is_none")]
    pub commercial_language: Option<String>,
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
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(
        rename = "disableEmailTemplate",
        skip_serializing_if = "Option::is_none"
    )]
    pub disable_email_template: Option<bool>,
    #[serde(rename = "endDate", skip_serializing_if = "Option::is_none")]
    pub end_date: Option<i32>,
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
    #[serde(rename = "offerDate", skip_serializing_if = "Option::is_none")]
    pub offer_date: Option<i32>,
    #[serde(rename = "paymentMethodId", skip_serializing_if = "Option::is_none")]
    pub payment_method_id: Option<String>,
    #[serde(rename = "paymentMethodName", skip_serializing_if = "Option::is_none")]
    pub payment_method_name: Option<String>,
    #[serde(
        rename = "plannedDeliveryDate",
        skip_serializing_if = "Option::is_none"
    )]
    pub planned_delivery_date: Option<i32>,
    #[serde(
        rename = "purchaseOrderRequestOfferItems",
        skip_serializing_if = "Option::is_none"
    )]
    pub purchase_order_request_offer_items:
        Option<Vec<crate::models::PurchaseOrderRequestOfferItem>>,
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
    #[serde(rename = "replyDate", skip_serializing_if = "Option::is_none")]
    pub reply_date: Option<i32>,
    #[serde(rename = "requestDate", skip_serializing_if = "Option::is_none")]
    pub request_date: Option<i32>,
    #[serde(rename = "sentToRecipient", skip_serializing_if = "Option::is_none")]
    pub sent_to_recipient: Option<bool>,
    #[serde(rename = "startDate", skip_serializing_if = "Option::is_none")]
    pub start_date: Option<i32>,
    #[serde(rename = "status")]
    pub status: Status,
    #[serde(rename = "supplierId")]
    pub supplier_id: String,
    #[serde(rename = "supplierNumber", skip_serializing_if = "Option::is_none")]
    pub supplier_number: Option<String>,
    #[serde(rename = "supplierReference", skip_serializing_if = "Option::is_none")]
    pub supplier_reference: Option<String>,
    #[serde(rename = "tags", skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,
    #[serde(rename = "termOfPaymentId", skip_serializing_if = "Option::is_none")]
    pub term_of_payment_id: Option<String>,
    #[serde(rename = "termOfPaymentName", skip_serializing_if = "Option::is_none")]
    pub term_of_payment_name: Option<String>,
    #[serde(rename = "validFrom", skip_serializing_if = "Option::is_none")]
    pub valid_from: Option<i32>,
    #[serde(rename = "validTo", skip_serializing_if = "Option::is_none")]
    pub valid_to: Option<i32>,
}

impl PurchaseOrderRequestOffer {
    pub fn new(status: Status, supplier_id: String) -> PurchaseOrderRequestOffer {
        PurchaseOrderRequestOffer {
            id: None,
            version: None,
            commercial_language: None,
            created_date: None,
            currency_conversion_date: None,
            currency_conversion_rate: None,
            custom_attributes: None,
            description: None,
            disable_email_template: None,
            end_date: None,
            gross_amount: None,
            gross_amount_in_company_currency: None,
            header_discount: None,
            header_surcharge: None,
            last_modified_date: None,
            net_amount: None,
            net_amount_in_company_currency: None,
            non_standard_tax_id: None,
            non_standard_tax_name: None,
            offer_date: None,
            payment_method_id: None,
            payment_method_name: None,
            planned_delivery_date: None,
            purchase_order_request_offer_items: None,
            record_comment: None,
            record_currency_id: None,
            record_currency_name: None,
            record_email_addresses: None,
            record_free_text: None,
            record_opening: None,
            reply_date: None,
            request_date: None,
            sent_to_recipient: None,
            start_date: None,
            status,
            supplier_id,
            supplier_number: None,
            supplier_reference: None,
            tags: None,
            term_of_payment_id: None,
            term_of_payment_name: None,
            valid_from: None,
            valid_to: None,
        }
    }
}

///
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Status {
    #[serde(rename = "ACCEPTED")]
    Accepted,
    #[serde(rename = "DECLINED")]
    Declined,
    #[serde(rename = "OFFER_RECEIVED")]
    OfferReceived,
    #[serde(rename = "OPEN")]
    Open,
    #[serde(rename = "REQUESTED")]
    Requested,
    #[serde(rename = "REVISED")]
    Revised,
    #[serde(rename = "REVISED_REQUEST")]
    RevisedRequest,
}

impl Default for Status {
    fn default() -> Status {
        Self::Accepted
    }
}
