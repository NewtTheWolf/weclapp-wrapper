# BlanketPurchaseOrder

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**String**> |  | [optional]
**version** | Option<**String**> |  | [optional]
**article_id** | **String** |  | 
**blanket_purchase_order_number** | Option<**String**> |  | [optional]
**calculation_mode** | Option<**String**> |  | [optional]
**comment** | Option<**String**> |  | [optional]
**commercial_language** | Option<**String**> |  | [optional]
**confirmation_number** | Option<**String**> |  | [optional]
**created_date** | Option<**i32**> |  | [optional]
**custom_attributes** | Option<[**Vec<crate::models::CustomAttribute>**](customAttribute.md)> |  | [optional]
**delivery_address** | Option<[**crate::models::RecordAddress**](recordAddress.md)> |  | [optional]
**description** | Option<**String**> |  | [optional]
**discount_percentage** | Option<[**crate::models::custom_attribute_definition::AttributeType**](decimal.md)> |  | [optional]
**end_date** | Option<**i32**> |  | [optional]
**form_settings_from_distribution_channel** | Option<**String**> |  | [optional]
**header_discount** | Option<[**crate::models::custom_attribute_definition::AttributeType**](decimal.md)> |  | [optional]
**header_surcharge** | Option<[**crate::models::custom_attribute_definition::AttributeType**](decimal.md)> |  | [optional]
**invoice_address** | Option<[**crate::models::RecordAddress**](recordAddress.md)> |  | [optional]
**last_modified_date** | Option<**i32**> |  | [optional]
**non_standard_tax_id** | Option<**String**> |  | [optional]
**order_date** | Option<**i32**> |  | [optional]
**order_quantity** | Option<[**crate::models::custom_attribute_definition::AttributeType**](decimal.md)> |  | [optional]
**payment_method_id** | Option<**String**> |  | [optional]
**recipient_country_code** | Option<**String**> |  | [optional]
**record_address** | Option<[**crate::models::RecordAddress**](recordAddress.md)> |  | [optional]
**record_comment** | Option<**String**> |  | [optional]
**record_currency_id** | Option<**String**> |  | [optional]
**record_email_addresses** | Option<[**crate::models::EmailAddresses**](emailAddresses.md)> |  | [optional]
**record_free_text** | Option<**String**> |  | [optional]
**record_opening** | Option<**String**> |  | [optional]
**releases** | Option<[**Vec<crate::models::Releases>**](releases.md)> |  | [optional]
**residual_quantity** | Option<[**crate::models::custom_attribute_definition::AttributeType**](decimal.md)> |  | [optional]
**responsible_user_id** | Option<**String**> |  | [optional]
**sent_to_recipient** | Option<**bool**> |  | [optional]
**shipment_method_id** | Option<**String**> |  | [optional]
**start_date** | Option<**i32**> |  | [optional]
**status** | **String** |  | 
**status_history** | Option<[**Vec<crate::models::BlanketPurchaseOrderStatusHistory>**](blanketPurchaseOrderStatusHistory.md)> |  | [optional]
**supplier_blanket_purchase_order_number** | Option<**String**> |  | [optional]
**supplier_id** | **String** |  | 
**tags** | Option<**Vec<String>**> |  | [optional]
**tax_id** | Option<**String**> |  | [optional]
**term_of_payment_id** | Option<**String**> |  | [optional]
**unit_price** | Option<[**crate::models::custom_attribute_definition::AttributeType**](decimal.md)> |  | [optional]
**use_manual_unit_price** | Option<**bool**> |  | [optional]
**warehouse_id** | Option<**String**> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


