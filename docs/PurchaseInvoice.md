# PurchaseInvoice

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**String**> |  | [optional]
**version** | Option<**String**> |  | [optional]
**booking_text** | Option<**String**> |  | [optional]
**commercial_language** | Option<**String**> |  | [optional]
**cost_center_id** | Option<**String**> |  | [optional]
**cost_center_number** | Option<**String**> |  | [optional]
**created_date** | Option<**i32**> |  | [optional]
**currency_conversion_date** | Option<**i32**> |  | [optional]
**currency_conversion_rate** | Option<[**crate::models::custom_attribute_definition::AttributeType**](decimal.md)> |  | [optional]
**custom_attributes** | Option<[**Vec<crate::models::CustomAttribute>**](customAttribute.md)> |  | [optional]
**delivery_address** | Option<[**crate::models::RecordAddress**](recordAddress.md)> |  | [optional]
**description** | Option<**String**> |  | [optional]
**disable_email_template** | Option<**bool**> |  | [optional]
**due_date** | Option<**i32**> |  | [optional]
**gross_amount** | Option<[**crate::models::custom_attribute_definition::AttributeType**](decimal.md)> |  | [optional]
**gross_amount_in_company_currency** | Option<[**crate::models::custom_attribute_definition::AttributeType**](decimal.md)> |  | [optional]
**header_discount** | Option<[**crate::models::custom_attribute_definition::AttributeType**](decimal.md)> |  | [optional]
**header_surcharge** | Option<[**crate::models::custom_attribute_definition::AttributeType**](decimal.md)> |  | [optional]
**internal_invoice_number** | Option<**String**> |  | [optional]
**invoice_address** | Option<[**crate::models::RecordAddress**](recordAddress.md)> |  | [optional]
**invoice_date** | Option<**i32**> |  | [optional]
**invoice_number** | Option<**String**> |  | [optional]
**last_modified_date** | Option<**i32**> |  | [optional]
**net_amount** | Option<[**crate::models::custom_attribute_definition::AttributeType**](decimal.md)> |  | [optional]
**net_amount_in_company_currency** | Option<[**crate::models::custom_attribute_definition::AttributeType**](decimal.md)> |  | [optional]
**non_standard_tax_id** | Option<**String**> |  | [optional]
**non_standard_tax_name** | Option<**String**> |  | [optional]
**payment_method_id** | Option<**String**> |  | [optional]
**payment_method_name** | Option<**String**> |  | [optional]
**payment_status** | **String** |  | 
**pricing_date** | Option<**i32**> |  | [optional]
**purchase_invoice_items** | Option<[**Vec<crate::models::PurchaseInvoiceItem>**](purchaseInvoiceItem.md)> |  | [optional]
**purchase_invoice_type** | Option<**String**> |  | [optional]
**purchase_orders** | Option<[**Vec<crate::models::OnlyId>**](onlyId.md)> |  | [optional]
**record_address** | Option<[**crate::models::RecordAddress**](recordAddress.md)> |  | [optional]
**record_comment** | Option<**String**> |  | [optional]
**record_currency_id** | Option<**String**> |  | [optional]
**record_currency_name** | Option<**String**> |  | [optional]
**record_email_addresses** | Option<[**crate::models::EmailAddresses**](emailAddresses.md)> |  | [optional]
**record_free_text** | Option<**String**> |  | [optional]
**record_opening** | Option<**String**> |  | [optional]
**responsible_user_id** | Option<**String**> |  | [optional]
**responsible_user_username** | Option<**String**> |  | [optional]
**sender_country_code** | Option<**String**> |  | [optional]
**sent_to_recipient** | Option<**bool**> |  | [optional]
**service_period_from** | Option<**i32**> |  | [optional]
**service_period_to** | Option<**i32**> |  | [optional]
**shipping_cost_items** | Option<[**Vec<crate::models::PurchaseShippingCostItem>**](purchaseShippingCostItem.md)> |  | [optional]
**status** | **String** |  | 
**status_history** | Option<[**Vec<crate::models::PurchaseInvoiceStatusHistory>**](purchaseInvoiceStatusHistory.md)> |  | [optional]
**supplier_habitual_exporter_letter_of_intent_id** | Option<**String**> |  | [optional]
**supplier_id** | **String** |  | 
**supplier_number** | Option<**String**> |  | [optional]
**tags** | Option<**Vec<String>**> |  | [optional]
**term_of_payment_id** | Option<**String**> |  | [optional]
**term_of_payment_name** | Option<**String**> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


