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
pub struct ProductionOrderItem {
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "version", skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    #[serde(
        rename = "actualWithdrawalDate",
        skip_serializing_if = "Option::is_none"
    )]
    pub actual_withdrawal_date: Option<i32>,
    #[serde(
        rename = "actualWithdrawalQuantity",
        skip_serializing_if = "Option::is_none"
    )]
    pub actual_withdrawal_quantity:
        Option<crate::models::custom_attribute_definition::AttributeType>,
    #[serde(rename = "articleId", skip_serializing_if = "Option::is_none")]
    pub article_id: Option<String>,
    #[serde(rename = "articleNumber", skip_serializing_if = "Option::is_none")]
    pub article_number: Option<String>,
    #[serde(rename = "availability", skip_serializing_if = "Option::is_none")]
    pub availability: Option<Availability>,
    #[serde(
        rename = "availabilityForAllWarehouses",
        skip_serializing_if = "Option::is_none"
    )]
    pub availability_for_all_warehouses: Option<AvailabilityForAllWarehouses>,
    #[serde(rename = "createdDate", skip_serializing_if = "Option::is_none")]
    pub created_date: Option<i32>,
    #[serde(rename = "customAttributes", skip_serializing_if = "Option::is_none")]
    pub custom_attributes: Option<Vec<crate::models::CustomAttribute>>,
    #[serde(rename = "lastModifiedDate", skip_serializing_if = "Option::is_none")]
    pub last_modified_date: Option<i32>,
    #[serde(rename = "note", skip_serializing_if = "Option::is_none")]
    pub note: Option<String>,
    #[serde(rename = "positionNumber", skip_serializing_if = "Option::is_none")]
    pub position_number: Option<i32>,
    #[serde(rename = "quantity", skip_serializing_if = "Option::is_none")]
    pub quantity: Option<crate::models::custom_attribute_definition::AttributeType>,
    #[serde(
        rename = "targetWithdrawalDate",
        skip_serializing_if = "Option::is_none"
    )]
    pub target_withdrawal_date: Option<i32>,
    #[serde(
        rename = "targetWithdrawalQuantity",
        skip_serializing_if = "Option::is_none"
    )]
    pub target_withdrawal_quantity:
        Option<crate::models::custom_attribute_definition::AttributeType>,
}

impl ProductionOrderItem {
    pub fn new() -> ProductionOrderItem {
        ProductionOrderItem {
            id: None,
            version: None,
            actual_withdrawal_date: None,
            actual_withdrawal_quantity: None,
            article_id: None,
            article_number: None,
            availability: None,
            availability_for_all_warehouses: None,
            created_date: None,
            custom_attributes: None,
            last_modified_date: None,
            note: None,
            position_number: None,
            quantity: None,
            target_withdrawal_date: None,
            target_withdrawal_quantity: None,
        }
    }
}

///
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Availability {
    #[serde(rename = "COMPLETELY_AVAILABLE")]
    CompletelyAvailable,
    #[serde(rename = "NOTHING_AVAILABLE")]
    NothingAvailable,
    #[serde(rename = "NOT_CHECKED")]
    NotChecked,
    #[serde(rename = "PARTIALLY_AVAILABLE")]
    PartiallyAvailable,
}

impl Default for Availability {
    fn default() -> Availability {
        Self::CompletelyAvailable
    }
}
///
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum AvailabilityForAllWarehouses {
    #[serde(rename = "COMPLETELY_AVAILABLE")]
    CompletelyAvailable,
    #[serde(rename = "NOTHING_AVAILABLE")]
    NothingAvailable,
    #[serde(rename = "NOT_CHECKED")]
    NotChecked,
    #[serde(rename = "PARTIALLY_AVAILABLE")]
    PartiallyAvailable,
}

impl Default for AvailabilityForAllWarehouses {
    fn default() -> AvailabilityForAllWarehouses {
        Self::CompletelyAvailable
    }
}
