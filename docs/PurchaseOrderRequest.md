# PurchaseOrderRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**String**> |  | [optional]
**version** | Option<**String**> |  | [optional]
**commercial_language** | Option<**String**> |  | [optional]
**confirmation_deadline** | Option<**i32**> |  | [optional]
**created_date** | Option<**i32**> |  | [optional]
**custom_attributes** | Option<[**Vec<crate::models::CustomAttribute>**](customAttribute.md)> |  | [optional]
**delivery_address** | Option<[**crate::models::RecordAddress**](recordAddress.md)> |  | [optional]
**description** | Option<**String**> |  | [optional]
**disable_email_template** | Option<**bool**> |  | [optional]
**invoice_address** | Option<[**crate::models::RecordAddress**](recordAddress.md)> |  | [optional]
**last_modified_date** | Option<**i32**> |  | [optional]
**planned_delivery_date** | Option<**i32**> |  | [optional]
**purchase_order_request_items** | Option<[**Vec<crate::models::PurchaseOrderRequestItem>**](purchaseOrderRequestItem.md)> |  | [optional]
**purchase_order_request_number** | **String** |  | 
**purchase_order_request_offers** | Option<[**Vec<crate::models::PurchaseOrderRequestOffer>**](purchaseOrderRequestOffer.md)> |  | [optional]
**purchase_order_request_type** | **String** |  | 
**quotation_id** | Option<**String**> |  | [optional]
**quotation_number** | Option<**String**> |  | [optional]
**record_address** | Option<[**crate::models::RecordAddress**](recordAddress.md)> |  | [optional]
**record_comment** | Option<**String**> |  | [optional]
**record_free_text** | Option<**String**> |  | [optional]
**record_opening** | Option<**String**> |  | [optional]
**responsible_user_id** | Option<**String**> |  | [optional]
**responsible_user_username** | Option<**String**> |  | [optional]
**sales_order_id** | Option<**String**> |  | [optional]
**sales_order_number** | Option<**String**> |  | [optional]
**sent_to_recipient** | Option<**bool**> |  | [optional]
**status** | **String** |  | 
**status_history** | Option<[**Vec<crate::models::PurchaseOrderRequestStatusHistory>**](purchaseOrderRequestStatusHistory.md)> |  | [optional]
**tags** | Option<**Vec<String>**> |  | [optional]
**warehouse_id** | Option<**String**> |  | [optional]
**warehouse_name** | Option<**String**> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


