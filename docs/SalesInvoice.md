# SalesInvoice

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**String**> |  | [optional]
**version** | Option<**String**> |  | [optional]
**cancellation_date** | Option<**i32**> |  | [optional]
**cancellation_number** | Option<**String**> |  | [optional]
**collective_invoice_position_print_type** | Option<**String**> |  | [optional]
**commercial_language** | Option<**String**> |  | [optional]
**commission** | Option<**String**> |  | [optional]
**commission_sales_partners** | Option<[**Vec<crate::models::CommissionSalesPartner>**](commissionSalesPartner.md)> |  | [optional]
**created_date** | Option<**i32**> |  | [optional]
**currency_conversion_date** | Option<**i32**> |  | [optional]
**currency_conversion_rate** | Option<[**crate::models::custom_attribute_definition::AttributeType**](decimal.md)> |  | [optional]
**custom_attributes** | Option<[**Vec<crate::models::CustomAttribute>**](customAttribute.md)> |  | [optional]
**customer_habitual_exporter_letter_of_intent_id** | Option<**String**> |  | [optional]
**customer_id** | **String** |  | 
**customer_number** | Option<**String**> |  | [optional]
**delivery_address** | Option<[**crate::models::RecordAddress**](recordAddress.md)> |  | [optional]
**delivery_date** | Option<**i32**> |  | [optional]
**description** | Option<**String**> |  | [optional]
**disable_email_template** | Option<**bool**> |  | [optional]
**dispatch_country_code** | Option<**String**> |  | [optional]
**due_date** | Option<**i32**> |  | [optional]
**factoring** | Option<**bool**> |  | [optional]
**gross_amount** | Option<[**crate::models::custom_attribute_definition::AttributeType**](decimal.md)> |  | [optional]
**gross_amount_in_company_currency** | Option<[**crate::models::custom_attribute_definition::AttributeType**](decimal.md)> |  | [optional]
**header_discount** | Option<[**crate::models::custom_attribute_definition::AttributeType**](decimal.md)> |  | [optional]
**header_surcharge** | Option<[**crate::models::custom_attribute_definition::AttributeType**](decimal.md)> |  | [optional]
**invoice_date** | Option<**i32**> |  | [optional]
**invoice_number** | Option<**String**> |  | [optional]
**last_modified_date** | Option<**i32**> |  | [optional]
**net_amount** | Option<[**crate::models::custom_attribute_definition::AttributeType**](decimal.md)> |  | [optional]
**net_amount_in_company_currency** | Option<[**crate::models::custom_attribute_definition::AttributeType**](decimal.md)> |  | [optional]
**non_standard_tax_id** | Option<**String**> |  | [optional]
**non_standard_tax_name** | Option<**String**> |  | [optional]
**order_number_at_customer** | Option<**String**> |  | [optional]
**paid** | Option<**bool**> |  | [optional]
**payment_method_id** | Option<**String**> |  | [optional]
**payment_method_name** | Option<**String**> |  | [optional]
**payment_status** | **String** |  | 
**pricing_date** | Option<**i32**> |  | [optional]
**record_address** | Option<[**crate::models::RecordAddress**](recordAddress.md)> |  | [optional]
**record_comment** | Option<**String**> |  | [optional]
**record_currency_id** | Option<**String**> |  | [optional]
**record_currency_name** | Option<**String**> |  | [optional]
**record_email_addresses** | Option<[**crate::models::EmailAddresses**](emailAddresses.md)> |  | [optional]
**record_free_text** | Option<**String**> |  | [optional]
**record_opening** | Option<**String**> |  | [optional]
**responsible_user_id** | Option<**String**> |  | [optional]
**responsible_user_username** | Option<**String**> |  | [optional]
**sales_channel** | Option<**String**> |  | [optional]
**sales_invoice_items** | Option<[**Vec<crate::models::SalesInvoiceItem>**](salesInvoiceItem.md)> |  | [optional]
**sales_invoice_type** | Option<**String**> |  | [optional]
**sales_order_id** | Option<**String**> |  | [optional]
**sales_order_number** | Option<**String**> |  | [optional]
**sales_orders** | Option<[**Vec<crate::models::OnlyId>**](onlyId.md)> |  | [optional]
**sent_to_recipient** | Option<**bool**> |  | [optional]
**service_period_from** | Option<**i32**> |  | [optional]
**service_period_to** | Option<**i32**> |  | [optional]
**shipment_method_id** | Option<**String**> |  | [optional]
**shipment_method_name** | Option<**String**> |  | [optional]
**shipping_cost_items** | Option<[**Vec<crate::models::SalesShippingCostItem>**](salesShippingCostItem.md)> |  | [optional]
**shipping_date** | Option<**i32**> |  | [optional]
**status** | **String** |  | 
**status_history** | Option<[**Vec<crate::models::SalesInvoiceStatusHistory>**](salesInvoiceStatusHistory.md)> |  | [optional]
**tags** | Option<**Vec<String>**> |  | [optional]
**term_of_payment_id** | Option<**String**> |  | [optional]
**term_of_payment_name** | Option<**String**> |  | [optional]
**vat_registration_number** | Option<**String**> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)

