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
pub struct ProductionOrder {
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "version", skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    #[serde(rename = "actualEndDate", skip_serializing_if = "Option::is_none")]
    pub actual_end_date: Option<i32>,
    #[serde(rename = "actualQuantity", skip_serializing_if = "Option::is_none")]
    pub actual_quantity: Option<crate::models::custom_attribute_definition::AttributeType>,
    #[serde(rename = "actualStartDate", skip_serializing_if = "Option::is_none")]
    pub actual_start_date: Option<i32>,
    #[serde(
        rename = "allWithdrawalsConfirmed",
        skip_serializing_if = "Option::is_none"
    )]
    pub all_withdrawals_confirmed: Option<bool>,
    #[serde(rename = "articleId")]
    pub article_id: String,
    #[serde(rename = "articleNumber", skip_serializing_if = "Option::is_none")]
    pub article_number: Option<String>,
    #[serde(
        rename = "assignedPickingUserId",
        skip_serializing_if = "Option::is_none"
    )]
    pub assigned_picking_user_id: Option<String>,
    #[serde(
        rename = "assignedPickingUserUpdateDate",
        skip_serializing_if = "Option::is_none"
    )]
    pub assigned_picking_user_update_date: Option<i32>,
    #[serde(
        rename = "assignedPickingUserUsername",
        skip_serializing_if = "Option::is_none"
    )]
    pub assigned_picking_user_username: Option<String>,
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
    #[serde(
        rename = "pickingInstructions",
        skip_serializing_if = "Option::is_none"
    )]
    pub picking_instructions: Option<String>,
    #[serde(
        rename = "productionOrderItems",
        skip_serializing_if = "Option::is_none"
    )]
    pub production_order_items: Option<Vec<crate::models::ProductionOrderItem>>,
    #[serde(
        rename = "productionOrderNumber",
        skip_serializing_if = "Option::is_none"
    )]
    pub production_order_number: Option<String>,
    #[serde(rename = "status")]
    pub status: Status,
    #[serde(rename = "statusHistory", skip_serializing_if = "Option::is_none")]
    pub status_history: Option<Vec<crate::models::ProductionOrderStatusHistory>>,
    #[serde(rename = "targetEndDate")]
    pub target_end_date: i32,
    #[serde(rename = "targetQuantity")]
    pub target_quantity: crate::models::custom_attribute_definition::AttributeType,
    #[serde(rename = "targetStartDate")]
    pub target_start_date: i32,
    #[serde(rename = "warehouseId", skip_serializing_if = "Option::is_none")]
    pub warehouse_id: Option<String>,
    #[serde(rename = "warehouseName", skip_serializing_if = "Option::is_none")]
    pub warehouse_name: Option<String>,
    #[serde(
        rename = "withdrawalsComplete",
        skip_serializing_if = "Option::is_none"
    )]
    pub withdrawals_complete: Option<bool>,
    #[serde(rename = "workItems", skip_serializing_if = "Option::is_none")]
    pub work_items: Option<Vec<crate::models::ProductionOrderWorkItem>>,
}

impl ProductionOrder {
    pub fn new(
        article_id: String,
        status: Status,
        target_end_date: i32,
        target_quantity: crate::models::custom_attribute_definition::AttributeType,
        target_start_date: i32,
    ) -> ProductionOrder {
        ProductionOrder {
            id: None,
            version: None,
            actual_end_date: None,
            actual_quantity: None,
            actual_start_date: None,
            all_withdrawals_confirmed: None,
            article_id,
            article_number: None,
            assigned_picking_user_id: None,
            assigned_picking_user_update_date: None,
            assigned_picking_user_username: None,
            availability: None,
            availability_for_all_warehouses: None,
            created_date: None,
            custom_attributes: None,
            last_modified_date: None,
            picking_instructions: None,
            production_order_items: None,
            production_order_number: None,
            status,
            status_history: None,
            target_end_date,
            target_quantity,
            target_start_date,
            warehouse_id: None,
            warehouse_name: None,
            withdrawals_complete: None,
            work_items: None,
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
///
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Status {
    #[serde(rename = "CANCELLED")]
    Cancelled,
    #[serde(rename = "CLOSED")]
    Closed,
    #[serde(rename = "DOCUMENTS_PRINTED")]
    DocumentsPrinted,
    #[serde(rename = "ENTRY_IN_PROGRESS")]
    EntryInProgress,
    #[serde(rename = "INTERRUPTED")]
    Interrupted,
    #[serde(rename = "NEW")]
    New,
    #[serde(rename = "STARTED")]
    Started,
}

impl Default for Status {
    fn default() -> Status {
        Self::Cancelled
    }
}
