# PurchaseOrderRequestOffer

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**String**> |  | [optional]
**version** | Option<**String**> |  | [optional]
**commercial_language** | Option<**String**> |  | [optional]
**created_date** | Option<**i32**> |  | [optional]
**currency_conversion_date** | Option<**i32**> |  | [optional]
**currency_conversion_rate** | Option<[**crate::models::custom_attribute_definition::AttributeType**](decimal.md)> |  | [optional]
**custom_attributes** | Option<[**Vec<crate::models::CustomAttribute>**](customAttribute.md)> |  | [optional]
**description** | Option<**String**> |  | [optional]
**disable_email_template** | Option<**bool**> |  | [optional]
**end_date** | Option<**i32**> |  | [optional]
**gross_amount** | Option<[**crate::models::custom_attribute_definition::AttributeType**](decimal.md)> |  | [optional]
**gross_amount_in_company_currency** | Option<[**crate::models::custom_attribute_definition::AttributeType**](decimal.md)> |  | [optional]
**header_discount** | Option<[**crate::models::custom_attribute_definition::AttributeType**](decimal.md)> |  | [optional]
**header_surcharge** | Option<[**crate::models::custom_attribute_definition::AttributeType**](decimal.md)> |  | [optional]
**last_modified_date** | Option<**i32**> |  | [optional]
**net_amount** | Option<[**crate::models::custom_attribute_definition::AttributeType**](decimal.md)> |  | [optional]
**net_amount_in_company_currency** | Option<[**crate::models::custom_attribute_definition::AttributeType**](decimal.md)> |  | [optional]
**non_standard_tax_id** | Option<**String**> |  | [optional]
**non_standard_tax_name** | Option<**String**> |  | [optional]
**offer_date** | Option<**i32**> |  | [optional]
**payment_method_id** | Option<**String**> |  | [optional]
**payment_method_name** | Option<**String**> |  | [optional]
**planned_delivery_date** | Option<**i32**> |  | [optional]
**purchase_order_request_offer_items** | Option<[**Vec<crate::models::PurchaseOrderRequestOfferItem>**](purchaseOrderRequestOfferItem.md)> |  | [optional]
**record_comment** | Option<**String**> |  | [optional]
**record_currency_id** | Option<**String**> |  | [optional]
**record_currency_name** | Option<**String**> |  | [optional]
**record_email_addresses** | Option<[**crate::models::EmailAddresses**](emailAddresses.md)> |  | [optional]
**record_free_text** | Option<**String**> |  | [optional]
**record_opening** | Option<**String**> |  | [optional]
**reply_date** | Option<**i32**> |  | [optional]
**request_date** | Option<**i32**> |  | [optional]
**sent_to_recipient** | Option<**bool**> |  | [optional]
**start_date** | Option<**i32**> |  | [optional]
**status** | **String** |  | 
**supplier_id** | **String** |  | 
**supplier_number** | Option<**String**> |  | [optional]
**supplier_reference** | Option<**String**> |  | [optional]
**tags** | Option<**Vec<String>**> |  | [optional]
**term_of_payment_id** | Option<**String**> |  | [optional]
**term_of_payment_name** | Option<**String**> |  | [optional]
**valid_from** | Option<**i32**> |  | [optional]
**valid_to** | Option<**i32**> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


