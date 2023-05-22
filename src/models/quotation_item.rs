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
pub struct QuotationItem {
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "version", skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    #[serde(rename = "addPageBreakBefore", skip_serializing_if = "Option::is_none")]
    pub add_page_break_before: Option<bool>,
    #[serde(rename = "alternative", skip_serializing_if = "Option::is_none")]
    pub alternative: Option<bool>,
    #[serde(rename = "articleId", skip_serializing_if = "Option::is_none")]
    pub article_id: Option<String>,
    #[serde(rename = "articleNumber", skip_serializing_if = "Option::is_none")]
    pub article_number: Option<String>,
    #[serde(
        rename = "commissionSalesPartners",
        skip_serializing_if = "Option::is_none"
    )]
    pub commission_sales_partners: Option<Vec<crate::models::CommissionSalesPartner>>,
    #[serde(rename = "createdDate", skip_serializing_if = "Option::is_none")]
    pub created_date: Option<i32>,
    #[serde(rename = "customAttributes", skip_serializing_if = "Option::is_none")]
    pub custom_attributes: Option<Vec<crate::models::CustomAttribute>>,
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "descriptionFixed", skip_serializing_if = "Option::is_none")]
    pub description_fixed: Option<bool>,
    #[serde(rename = "discountPercentage", skip_serializing_if = "Option::is_none")]
    pub discount_percentage: Option<crate::models::custom_attribute_definition::AttributeType>,
    #[serde(rename = "freeTextItem", skip_serializing_if = "Option::is_none")]
    pub free_text_item: Option<bool>,
    #[serde(rename = "grossAmount", skip_serializing_if = "Option::is_none")]
    pub gross_amount: Option<crate::models::custom_attribute_definition::AttributeType>,
    #[serde(
        rename = "grossAmountInCompanyCurrency",
        skip_serializing_if = "Option::is_none"
    )]
    pub gross_amount_in_company_currency:
        Option<crate::models::custom_attribute_definition::AttributeType>,
    #[serde(rename = "groupName", skip_serializing_if = "Option::is_none")]
    pub group_name: Option<String>,
    #[serde(rename = "invoicingType", skip_serializing_if = "Option::is_none")]
    pub invoicing_type: Option<InvoicingType>,
    #[serde(rename = "lastModifiedDate", skip_serializing_if = "Option::is_none")]
    pub last_modified_date: Option<i32>,
    #[serde(rename = "manualQuantity", skip_serializing_if = "Option::is_none")]
    pub manual_quantity: Option<bool>,
    #[serde(rename = "manualUnitCost", skip_serializing_if = "Option::is_none")]
    pub manual_unit_cost: Option<bool>,
    #[serde(rename = "manualUnitPrice", skip_serializing_if = "Option::is_none")]
    pub manual_unit_price: Option<bool>,
    #[serde(rename = "netAmount", skip_serializing_if = "Option::is_none")]
    pub net_amount: Option<crate::models::custom_attribute_definition::AttributeType>,
    #[serde(
        rename = "netAmountForStatistics",
        skip_serializing_if = "Option::is_none"
    )]
    pub net_amount_for_statistics:
        Option<crate::models::custom_attribute_definition::AttributeType>,
    #[serde(
        rename = "netAmountForStatisticsInCompanyCurrency",
        skip_serializing_if = "Option::is_none"
    )]
    pub net_amount_for_statistics_in_company_currency:
        Option<crate::models::custom_attribute_definition::AttributeType>,
    #[serde(
        rename = "netAmountInCompanyCurrency",
        skip_serializing_if = "Option::is_none"
    )]
    pub net_amount_in_company_currency:
        Option<crate::models::custom_attribute_definition::AttributeType>,
    #[serde(rename = "note", skip_serializing_if = "Option::is_none")]
    pub note: Option<String>,
    #[serde(rename = "optional", skip_serializing_if = "Option::is_none")]
    pub optional: Option<bool>,
    #[serde(rename = "parentItemId", skip_serializing_if = "Option::is_none")]
    pub parent_item_id: Option<String>,
    #[serde(
        rename = "plannedWorkingTimePerUnit",
        skip_serializing_if = "Option::is_none"
    )]
    pub planned_working_time_per_unit:
        Option<crate::models::custom_attribute_definition::AttributeType>,
    #[serde(rename = "positionNumber", skip_serializing_if = "Option::is_none")]
    pub position_number: Option<i32>,
    #[serde(rename = "quantity", skip_serializing_if = "Option::is_none")]
    pub quantity: Option<crate::models::custom_attribute_definition::AttributeType>,
    #[serde(
        rename = "reductionAdditionItems",
        skip_serializing_if = "Option::is_none"
    )]
    pub reduction_addition_items: Option<Vec<crate::models::ReductionAdditionItem>>,
    #[serde(rename = "serviceItem", skip_serializing_if = "Option::is_none")]
    pub service_item: Option<bool>,
    #[serde(rename = "servicePeriodFrom", skip_serializing_if = "Option::is_none")]
    pub service_period_from: Option<i32>,
    #[serde(rename = "servicePeriodTo", skip_serializing_if = "Option::is_none")]
    pub service_period_to: Option<i32>,
    #[serde(rename = "taxId", skip_serializing_if = "Option::is_none")]
    pub tax_id: Option<String>,
    #[serde(rename = "taxName", skip_serializing_if = "Option::is_none")]
    pub tax_name: Option<String>,
    #[serde(rename = "title", skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(rename = "unitCost", skip_serializing_if = "Option::is_none")]
    pub unit_cost: Option<crate::models::custom_attribute_definition::AttributeType>,
    #[serde(
        rename = "unitCostInCompanyCurrency",
        skip_serializing_if = "Option::is_none"
    )]
    pub unit_cost_in_company_currency:
        Option<crate::models::custom_attribute_definition::AttributeType>,
    #[serde(rename = "unitId", skip_serializing_if = "Option::is_none")]
    pub unit_id: Option<String>,
    #[serde(rename = "unitName", skip_serializing_if = "Option::is_none")]
    pub unit_name: Option<String>,
    #[serde(rename = "unitPrice", skip_serializing_if = "Option::is_none")]
    pub unit_price: Option<crate::models::custom_attribute_definition::AttributeType>,
    #[serde(
        rename = "unitPriceInCompanyCurrency",
        skip_serializing_if = "Option::is_none"
    )]
    pub unit_price_in_company_currency:
        Option<crate::models::custom_attribute_definition::AttributeType>,
}

impl QuotationItem {
    pub fn new() -> QuotationItem {
        QuotationItem {
            id: None,
            version: None,
            add_page_break_before: None,
            alternative: None,
            article_id: None,
            article_number: None,
            commission_sales_partners: None,
            created_date: None,
            custom_attributes: None,
            description: None,
            description_fixed: None,
            discount_percentage: None,
            free_text_item: None,
            gross_amount: None,
            gross_amount_in_company_currency: None,
            group_name: None,
            invoicing_type: None,
            last_modified_date: None,
            manual_quantity: None,
            manual_unit_cost: None,
            manual_unit_price: None,
            net_amount: None,
            net_amount_for_statistics: None,
            net_amount_for_statistics_in_company_currency: None,
            net_amount_in_company_currency: None,
            note: None,
            optional: None,
            parent_item_id: None,
            planned_working_time_per_unit: None,
            position_number: None,
            quantity: None,
            reduction_addition_items: None,
            service_item: None,
            service_period_from: None,
            service_period_to: None,
            tax_id: None,
            tax_name: None,
            title: None,
            unit_cost: None,
            unit_cost_in_company_currency: None,
            unit_id: None,
            unit_name: None,
            unit_price: None,
            unit_price_in_company_currency: None,
        }
    }
}

///
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum InvoicingType {
    #[serde(rename = "EFFORT")]
    Effort,
    #[serde(rename = "FIXED_PRICE")]
    FixedPrice,
}

impl Default for InvoicingType {
    fn default() -> InvoicingType {
        Self::Effort
    }
}
