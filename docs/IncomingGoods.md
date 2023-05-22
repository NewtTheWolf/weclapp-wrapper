# IncomingGoods

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**String**> |  | [optional]
**version** | Option<**String**> |  | [optional]
**commercial_language** | Option<**String**> |  | [optional]
**created_date** | Option<**i32**> |  | [optional]
**custom_attributes** | Option<[**Vec<crate::models::CustomAttribute>**](customAttribute.md)> |  | [optional]
**customer_delivery_address** | Option<[**crate::models::RecordAddress**](recordAddress.md)> |  | [optional]
**customer_invoice_address** | Option<[**crate::models::RecordAddress**](recordAddress.md)> |  | [optional]
**delivery_note_number** | Option<**String**> |  | [optional]
**description** | Option<**String**> |  | [optional]
**disable_email_template** | Option<**bool**> |  | [optional]
**incoming_goods_items** | Option<[**Vec<crate::models::IncomingGoodsItem>**](incomingGoodsItem.md)> |  | [optional]
**incoming_goods_number** | Option<**String**> |  | [optional]
**incoming_goods_type** | Option<**String**> |  | [optional]
**invoice_address** | Option<[**crate::models::RecordAddress**](recordAddress.md)> |  | [optional]
**invoice_recipient_id** | Option<**String**> |  | [optional]
**last_modified_date** | Option<**i32**> |  | [optional]
**purchase_order_id** | Option<**String**> |  | [optional]
**purchase_order_number** | Option<**String**> |  | [optional]
**purchase_orders** | Option<[**Vec<crate::models::OnlyId>**](onlyId.md)> |  | [optional]
**recipient_address** | Option<[**crate::models::RecordAddress**](recordAddress.md)> |  | [optional]
**record_comment** | Option<**String**> |  | [optional]
**record_free_text** | Option<**String**> |  | [optional]
**record_opening** | Option<**String**> |  | [optional]
**related_shipment_id** | Option<**String**> |  | [optional]
**responsible_user_id** | Option<**String**> |  | [optional]
**return_address** | Option<[**crate::models::RecordAddress**](recordAddress.md)> |  | [optional]
**sales_order_id** | Option<**String**> |  | [optional]
**sales_order_number** | Option<**String**> |  | [optional]
**sender_customer_number** | Option<**String**> |  | [optional]
**sender_party_id** | Option<**String**> |  | [optional]
**sender_supplier_number** | Option<**String**> |  | [optional]
**sent_to_recipient** | Option<**bool**> |  | [optional]
**source_warehouse_id** | Option<**String**> |  | [optional]
**source_warehouse_name** | Option<**String**> |  | [optional]
**status** | **String** |  | 
**status_history** | Option<[**Vec<crate::models::ShipmentStatus>**](shipmentStatus.md)> |  | [optional]
**tags** | Option<**Vec<String>**> |  | [optional]
**warehouse_id** | Option<**String**> |  | [optional]
**warehouse_name** | Option<**String**> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


