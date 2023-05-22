# WarehouseStockMovementBookTransferMovementPostRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**article_id** | **String** |  | 
**quantity** | [**crate::models::custom_attribute_definition::AttributeType**](decimal.md) |  | 
**source_warehouse_level_id** | **String** |  | 
**target_warehouse_level_id** | **String** |  | 
**movement_note** | Option<**String**> |  | [optional]
**batch_number** | Option<**String**> |  | [optional]
**serial_numbers** | Option<**Vec<String>**> |  | [optional]
**custom_attributes** | Option<[**Vec<crate::models::CustomAttribute>**](customAttribute.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


