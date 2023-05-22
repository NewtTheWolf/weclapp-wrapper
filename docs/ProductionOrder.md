# ProductionOrder

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**String**> |  | [optional]
**version** | Option<**String**> |  | [optional]
**actual_end_date** | Option<**i32**> |  | [optional]
**actual_quantity** | Option<[**crate::models::custom_attribute_definition::AttributeType**](decimal.md)> |  | [optional]
**actual_start_date** | Option<**i32**> |  | [optional]
**all_withdrawals_confirmed** | Option<**bool**> |  | [optional]
**article_id** | **String** |  | 
**article_number** | Option<**String**> |  | [optional]
**assigned_picking_user_id** | Option<**String**> |  | [optional]
**assigned_picking_user_update_date** | Option<**i32**> |  | [optional]
**assigned_picking_user_username** | Option<**String**> |  | [optional]
**availability** | Option<**String**> |  | [optional]
**availability_for_all_warehouses** | Option<**String**> |  | [optional]
**created_date** | Option<**i32**> |  | [optional]
**custom_attributes** | Option<[**Vec<crate::models::CustomAttribute>**](customAttribute.md)> |  | [optional]
**last_modified_date** | Option<**i32**> |  | [optional]
**picking_instructions** | Option<**String**> |  | [optional]
**production_order_items** | Option<[**Vec<crate::models::ProductionOrderItem>**](productionOrderItem.md)> |  | [optional]
**production_order_number** | Option<**String**> |  | [optional]
**status** | **String** |  | 
**status_history** | Option<[**Vec<crate::models::ProductionOrderStatusHistory>**](productionOrderStatusHistory.md)> |  | [optional]
**target_end_date** | **i32** |  | 
**target_quantity** | [**crate::models::custom_attribute_definition::AttributeType**](decimal.md) |  | 
**target_start_date** | **i32** |  | 
**warehouse_id** | Option<**String**> |  | [optional]
**warehouse_name** | Option<**String**> |  | [optional]
**withdrawals_complete** | Option<**bool**> |  | [optional]
**work_items** | Option<[**Vec<crate::models::ProductionOrderWorkItem>**](productionOrderWorkItem.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


