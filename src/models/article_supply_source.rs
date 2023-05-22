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
pub struct ArticleSupplySource {
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "version", skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    #[serde(rename = "articleNumber", skip_serializing_if = "Option::is_none")]
    pub article_number: Option<String>,
    #[serde(rename = "articlePrices", skip_serializing_if = "Option::is_none")]
    pub article_prices: Option<Vec<crate::models::ArticlePriceWithoutSalesChannel>>,
    #[serde(rename = "createdDate", skip_serializing_if = "Option::is_none")]
    pub created_date: Option<i32>,
    #[serde(rename = "customAttributes", skip_serializing_if = "Option::is_none")]
    pub custom_attributes: Option<Vec<crate::models::CustomAttribute>>,
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "ean", skip_serializing_if = "Option::is_none")]
    pub ean: Option<String>,
    #[serde(
        rename = "fixedPurchaseQuantity",
        skip_serializing_if = "Option::is_none"
    )]
    pub fixed_purchase_quantity: Option<crate::models::custom_attribute_definition::AttributeType>,
    #[serde(rename = "internalNote", skip_serializing_if = "Option::is_none")]
    pub internal_note: Option<String>,
    #[serde(rename = "lastModifiedDate", skip_serializing_if = "Option::is_none")]
    pub last_modified_date: Option<i32>,
    #[serde(
        rename = "manufacturerPartNumber",
        skip_serializing_if = "Option::is_none"
    )]
    pub manufacturer_part_number: Option<String>,
    #[serde(rename = "matchCode", skip_serializing_if = "Option::is_none")]
    pub match_code: Option<String>,
    #[serde(
        rename = "minimumPurchaseQuantity",
        skip_serializing_if = "Option::is_none"
    )]
    pub minimum_purchase_quantity:
        Option<crate::models::custom_attribute_definition::AttributeType>,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(
        rename = "procurementLeadDays",
        skip_serializing_if = "Option::is_none"
    )]
    pub procurement_lead_days: Option<i32>,
    #[serde(rename = "shortDescription1", skip_serializing_if = "Option::is_none")]
    pub short_description1: Option<String>,
    #[serde(rename = "shortDescription2", skip_serializing_if = "Option::is_none")]
    pub short_description2: Option<String>,
    #[serde(rename = "supplierId", skip_serializing_if = "Option::is_none")]
    pub supplier_id: Option<String>,
    #[serde(rename = "supplierNumber", skip_serializing_if = "Option::is_none")]
    pub supplier_number: Option<String>,
    #[serde(
        rename = "supplierStockQuantity",
        skip_serializing_if = "Option::is_none"
    )]
    pub supplier_stock_quantity: Option<crate::models::custom_attribute_definition::AttributeType>,
    #[serde(rename = "taxRateType", skip_serializing_if = "Option::is_none")]
    pub tax_rate_type: Option<TaxRateType>,
    #[serde(rename = "unitId")]
    pub unit_id: String,
    #[serde(rename = "unitName", skip_serializing_if = "Option::is_none")]
    pub unit_name: Option<String>,
}

impl ArticleSupplySource {
    pub fn new(name: String, unit_id: String) -> ArticleSupplySource {
        ArticleSupplySource {
            id: None,
            version: None,
            article_number: None,
            article_prices: None,
            created_date: None,
            custom_attributes: None,
            description: None,
            ean: None,
            fixed_purchase_quantity: None,
            internal_note: None,
            last_modified_date: None,
            manufacturer_part_number: None,
            match_code: None,
            minimum_purchase_quantity: None,
            name,
            procurement_lead_days: None,
            short_description1: None,
            short_description2: None,
            supplier_id: None,
            supplier_number: None,
            supplier_stock_quantity: None,
            tax_rate_type: None,
            unit_id,
            unit_name: None,
        }
    }
}

///
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum TaxRateType {
    #[serde(rename = "REDUCED")]
    Reduced,
    #[serde(rename = "SLIGHTLY_REDUCED")]
    SlightlyReduced,
    #[serde(rename = "STANDARD")]
    Standard,
    #[serde(rename = "SUPER_REDUCED")]
    SuperReduced,
    #[serde(rename = "ZERO")]
    Zero,
}

impl Default for TaxRateType {
    fn default() -> TaxRateType {
        Self::Reduced
    }
}