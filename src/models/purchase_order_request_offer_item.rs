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
pub struct PurchaseOrderRequestOfferItem {
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "version", skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    #[serde(rename = "accepted", skip_serializing_if = "Option::is_none")]
    pub accepted: Option<bool>,
    #[serde(rename = "articleId", skip_serializing_if = "Option::is_none")]
    pub article_id: Option<String>,
    #[serde(rename = "articleNumber", skip_serializing_if = "Option::is_none")]
    pub article_number: Option<String>,
    #[serde(rename = "containerQuantity", skip_serializing_if = "Option::is_none")]
    pub container_quantity: Option<crate::models::custom_attribute_definition::AttributeType>,
    #[serde(rename = "createdDate", skip_serializing_if = "Option::is_none")]
    pub created_date: Option<i32>,
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "descriptionFixed", skip_serializing_if = "Option::is_none")]
    pub description_fixed: Option<bool>,
    #[serde(rename = "discountPercentage", skip_serializing_if = "Option::is_none")]
    pub discount_percentage: Option<crate::models::custom_attribute_definition::AttributeType>,
    #[serde(rename = "grossAmount", skip_serializing_if = "Option::is_none")]
    pub gross_amount: Option<crate::models::custom_attribute_definition::AttributeType>,
    #[serde(
        rename = "grossAmountInCompanyCurrency",
        skip_serializing_if = "Option::is_none"
    )]
    pub gross_amount_in_company_currency:
        Option<crate::models::custom_attribute_definition::AttributeType>,
    #[serde(rename = "lastModifiedDate", skip_serializing_if = "Option::is_none")]
    pub last_modified_date: Option<i32>,
    #[serde(rename = "manualQuantity", skip_serializing_if = "Option::is_none")]
    pub manual_quantity: Option<bool>,
    #[serde(rename = "manualUnitPrice", skip_serializing_if = "Option::is_none")]
    pub manual_unit_price: Option<bool>,
    #[serde(rename = "minQuantity", skip_serializing_if = "Option::is_none")]
    pub min_quantity: Option<crate::models::custom_attribute_definition::AttributeType>,
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
    #[serde(rename = "orderedQuantity", skip_serializing_if = "Option::is_none")]
    pub ordered_quantity: Option<crate::models::custom_attribute_definition::AttributeType>,
    #[serde(rename = "parentItemId", skip_serializing_if = "Option::is_none")]
    pub parent_item_id: Option<String>,
    #[serde(rename = "positionNumber", skip_serializing_if = "Option::is_none")]
    pub position_number: Option<i32>,
    #[serde(rename = "procurementLeadDays")]
    pub procurement_lead_days: i32,
    #[serde(
        rename = "purchaseOrderRequestItemId",
        skip_serializing_if = "Option::is_none"
    )]
    pub purchase_order_request_item_id: Option<i32>,
    #[serde(rename = "quantity", skip_serializing_if = "Option::is_none")]
    pub quantity: Option<crate::models::custom_attribute_definition::AttributeType>,
    #[serde(
        rename = "reductionAdditionItems",
        skip_serializing_if = "Option::is_none"
    )]
    pub reduction_addition_items: Option<Vec<crate::models::ReductionAdditionItem>>,
    #[serde(rename = "scaleType", skip_serializing_if = "Option::is_none")]
    pub scale_type: Option<ScaleType>,
    #[serde(rename = "scaleValues", skip_serializing_if = "Option::is_none")]
    pub scale_values: Option<Vec<crate::models::PurchaseOrderRequestOfferItemScaleValue>>,
    #[serde(
        rename = "supplierArticleNumber",
        skip_serializing_if = "Option::is_none"
    )]
    pub supplier_article_number: Option<String>,
    #[serde(rename = "taxId", skip_serializing_if = "Option::is_none")]
    pub tax_id: Option<String>,
    #[serde(rename = "taxName", skip_serializing_if = "Option::is_none")]
    pub tax_name: Option<String>,
    #[serde(rename = "title", skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
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

impl PurchaseOrderRequestOfferItem {
    pub fn new(procurement_lead_days: i32) -> PurchaseOrderRequestOfferItem {
        PurchaseOrderRequestOfferItem {
            id: None,
            version: None,
            accepted: None,
            article_id: None,
            article_number: None,
            container_quantity: None,
            created_date: None,
            description: None,
            description_fixed: None,
            discount_percentage: None,
            gross_amount: None,
            gross_amount_in_company_currency: None,
            last_modified_date: None,
            manual_quantity: None,
            manual_unit_price: None,
            min_quantity: None,
            net_amount: None,
            net_amount_for_statistics: None,
            net_amount_for_statistics_in_company_currency: None,
            net_amount_in_company_currency: None,
            note: None,
            ordered_quantity: None,
            parent_item_id: None,
            position_number: None,
            procurement_lead_days,
            purchase_order_request_item_id: None,
            quantity: None,
            reduction_addition_items: None,
            scale_type: None,
            scale_values: None,
            supplier_article_number: None,
            tax_id: None,
            tax_name: None,
            title: None,
            unit_id: None,
            unit_name: None,
            unit_price: None,
            unit_price_in_company_currency: None,
        }
    }
}

///
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum ScaleType {
    #[serde(rename = "SCALE_FROM")]
    From,
    #[serde(rename = "SCALE_TO")]
    To,
}

impl Default for ScaleType {
    fn default() -> ScaleType {
        Self::From
    }
}