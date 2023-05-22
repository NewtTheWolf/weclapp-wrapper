use std::error;
use std::fmt;

#[derive(Debug, Clone)]
pub struct ResponseContent<T> {
    pub status: reqwest::StatusCode,
    pub content: String,
    pub entity: Option<T>,
}

#[derive(Debug)]
pub enum Error<T> {
    Reqwest(reqwest::Error),
    Serde(serde_json::Error),
    Io(std::io::Error),
    ResponseError(ResponseContent<T>),
}

impl<T> fmt::Display for Error<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let (module, e) = match self {
            Error::Reqwest(e) => ("reqwest", e.to_string()),
            Error::Serde(e) => ("serde", e.to_string()),
            Error::Io(e) => ("IO", e.to_string()),
            Error::ResponseError(e) => ("response", format!("status code {}", e.status)),
        };
        write!(f, "error in {}: {}", module, e)
    }
}

impl<T: fmt::Debug> error::Error for Error<T> {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        Some(match self {
            Error::Reqwest(e) => e,
            Error::Serde(e) => e,
            Error::Io(e) => e,
            Error::ResponseError(_) => return None,
        })
    }
}

impl<T> From<reqwest::Error> for Error<T> {
    fn from(e: reqwest::Error) -> Self {
        Error::Reqwest(e)
    }
}

impl<T> From<serde_json::Error> for Error<T> {
    fn from(e: serde_json::Error) -> Self {
        Error::Serde(e)
    }
}

impl<T> From<std::io::Error> for Error<T> {
    fn from(e: std::io::Error) -> Self {
        Error::Io(e)
    }
}

pub fn urlencode<T: AsRef<str>>(s: T) -> String {
    ::url::form_urlencoded::byte_serialize(s.as_ref().as_bytes()).collect()
}

pub fn parse_deep_object(prefix: &str, value: &serde_json::Value) -> Vec<(String, String)> {
    if let serde_json::Value::Object(object) = value {
        let mut params = vec![];

        for (key, value) in object {
            match value {
                serde_json::Value::Object(_) => params.append(&mut parse_deep_object(
                    &format!("{}[{}]", prefix, key),
                    value,
                )),
                serde_json::Value::Array(array) => {
                    for (i, value) in array.iter().enumerate() {
                        params.append(&mut parse_deep_object(
                            &format!("{}[{}][{}]", prefix, key, i),
                            value,
                        ));
                    }
                }
                serde_json::Value::String(s) => {
                    params.push((format!("{}[{}]", prefix, key), s.clone()))
                }
                _ => params.push((format!("{}[{}]", prefix, key), value.to_string())),
            }
        }

        return params;
    }

    unimplemented!("Only objects are supported with style=deepObject")
}

pub mod accounting_transaction_api;
pub mod archived_email_api;
pub mod article_accounting_code_api;
pub mod article_api;
pub mod article_category_api;
pub mod article_price_api;
pub mod article_rating_api;
pub mod article_status_api;
pub mod article_supply_source_api;
pub mod attendance_api;
pub mod bank_account_api;
pub mod batch_number_api;
pub mod blanket_purchase_order_api;
pub mod calendar_api;
pub mod calendar_event_api;
pub mod campaign_api;
pub mod campaign_participant_api;
pub mod cash_account_api;
pub mod comment_api;
pub mod commercial_language_api;
pub mod company_size_api;
pub mod contact_api;
pub mod contract_api;
pub mod contract_authorization_unit_api;
pub mod contract_billing_group_api;
pub mod contract_termination_reason_api;
pub mod cost_center_api;
pub mod cost_center_group_api;
pub mod cost_type_api;
pub mod crm_call_category_api;
pub mod crm_event_api;
pub mod crm_event_category_api;
pub mod currency_api;
pub mod custom_attribute_definition_api;
pub mod customer_api;
pub mod customer_category_api;
pub mod customer_lead_loss_reason_api;
pub mod customer_topic_api;
pub mod customs_tariff_number_api;
pub mod document_api;
pub mod external_connection_api;
pub mod financial_year_api;
pub mod fulfillment_provider_api;
pub mod incoming_goods_api;
pub mod lead_api;
pub mod lead_rating_api;
pub mod lead_source_api;
pub mod ledger_account_api;
pub mod legal_form_api;
pub mod manufacturer_api;
pub mod meta_api;
pub mod notification_api;
pub mod opportunity_api;
pub mod opportunity_topic_api;
pub mod opportunity_win_loss_reason_api;
pub mod party_api;
pub mod party_rating_api;
pub mod payment_method_api;
pub mod payment_run_api;
pub mod payment_run_item_api;
pub mod person_department_api;
pub mod person_role_api;
pub mod personal_accounting_code_api;
pub mod place_of_service_api;
pub mod production_order_api;
pub mod production_work_schedule_api;
pub mod production_work_schedule_assignment_api;
pub mod purchase_invoice_api;
pub mod purchase_open_item_api;
pub mod purchase_order_api;
pub mod purchase_order_request_api;
pub mod quotation_api;
pub mod remote_print_job_api;
pub mod sales_channel_api;
pub mod sales_invoice_api;
pub mod sales_open_item_api;
pub mod sales_order_api;
pub mod sales_stage_api;
pub mod sector_api;
pub mod sepa_direct_debit_mandate_api;
pub mod serial_number_api;
pub mod shipment_api;
pub mod shipment_method_api;
pub mod shipment_return_assessment_api;
pub mod shipment_return_error_api;
pub mod shipment_return_reason_api;
pub mod shipment_return_rectification_api;
pub mod shipping_carrier_api;
pub mod supplier_api;
pub mod system_api;
pub mod tag_api;
pub mod tax_api;
pub mod tax_determination_rule_api;
pub mod term_of_payment_api;
pub mod ticket_api;
pub mod ticket_assignment_rule_api;
pub mod title_api;
pub mod unit_api;
pub mod user_api;
pub mod variant_article_api;
pub mod variant_article_attribute_api;
pub mod variant_article_variant_api;
pub mod warehouse_api;
pub mod warehouse_level_api;
pub mod warehouse_stock_api;
pub mod warehouse_stock_movement_api;
pub mod webhook_api;
pub mod weclapp_os_api;

pub mod configuration;
