# SalesOrderItem

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**String**> |  | [optional]
**version** | Option<**String**> |  | [optional]
**add_page_break_before** | Option<**bool**> |  | [optional]
**article_id** | Option<**String**> |  | [optional]
**article_number** | Option<**String**> |  | [optional]
**availability** | Option<**String**> |  | [optional]
**availability_for_all_warehouses** | Option<**String**> |  | [optional]
**commission_sales_partners** | Option<[**Vec<crate::models::CommissionSalesPartner>**](commissionSalesPartner.md)> |  | [optional]
**created_date** | Option<**i32**> |  | [optional]
**custom_attributes** | Option<[**Vec<crate::models::CustomAttribute>**](customAttribute.md)> |  | [optional]
**description** | Option<**String**> |  | [optional]
**description_fixed** | Option<**bool**> |  | [optional]
**discount_percentage** | Option<[**crate::models::custom_attribute_definition::AttributeType**](decimal.md)> |  | [optional]
**ecommerce_order_item_id** | Option<**String**> |  | [optional]
**free_text_item** | Option<**bool**> |  | [optional]
**gross_amount** | Option<[**crate::models::custom_attribute_definition::AttributeType**](decimal.md)> |  | [optional]
**gross_amount_in_company_currency** | Option<[**crate::models::custom_attribute_definition::AttributeType**](decimal.md)> |  | [optional]
**group_name** | Option<**String**> |  | [optional]
**invoiced_quantity** | Option<[**crate::models::custom_attribute_definition::AttributeType**](decimal.md)> |  | [optional]
**invoicing_type** | Option<**String**> |  | [optional]
**last_modified_date** | Option<**i32**> |  | [optional]
**manual_quantity** | Option<**bool**> |  | [optional]
**manual_unit_cost** | Option<**bool**> |  | [optional]
**manual_unit_price** | Option<**bool**> |  | [optional]
**net_amount** | Option<[**crate::models::custom_attribute_definition::AttributeType**](decimal.md)> |  | [optional]
**net_amount_for_statistics** | Option<[**crate::models::custom_attribute_definition::AttributeType**](decimal.md)> |  | [optional]
**net_amount_for_statistics_in_company_currency** | Option<[**crate::models::custom_attribute_definition::AttributeType**](decimal.md)> |  | [optional]
**net_amount_in_company_currency** | Option<[**crate::models::custom_attribute_definition::AttributeType**](decimal.md)> |  | [optional]
**note** | Option<**String**> |  | [optional]
**parent_item_id** | Option<**String**> |  | [optional]
**planned_shipping_date** | Option<**i32**> |  | [optional]
**planned_working_time_per_unit** | Option<[**crate::models::custom_attribute_definition::AttributeType**](decimal.md)> |  | [optional]
**position_number** | Option<**i32**> |  | [optional]
**quantity** | Option<[**crate::models::custom_attribute_definition::AttributeType**](decimal.md)> |  | [optional]
**reduction_addition_items** | Option<[**Vec<crate::models::ReductionAdditionItem>**](reductionAdditionItem.md)> |  | [optional]
**service_item** | Option<**bool**> |  | [optional]
**service_period_from** | Option<**i32**> |  | [optional]
**service_period_to** | Option<**i32**> |  | [optional]
**shipped** | Option<**bool**> |  | [optional]
**shipped_quantity** | Option<[**crate::models::custom_attribute_definition::AttributeType**](decimal.md)> |  | [optional]
**tasks** | Option<[**Vec<crate::models::OnlyId>**](onlyId.md)> |  | [optional]
**tax_id** | Option<**String**> |  | [optional]
**tax_name** | Option<**String**> |  | [optional]
**title** | Option<**String**> |  | [optional]
**unit_cost** | Option<[**crate::models::custom_attribute_definition::AttributeType**](decimal.md)> |  | [optional]
**unit_cost_in_company_currency** | Option<[**crate::models::custom_attribute_definition::AttributeType**](decimal.md)> |  | [optional]
**unit_id** | Option<**String**> |  | [optional]
**unit_name** | Option<**String**> |  | [optional]
**unit_price** | Option<[**crate::models::custom_attribute_definition::AttributeType**](decimal.md)> |  | [optional]
**unit_price_in_company_currency** | Option<[**crate::models::custom_attribute_definition::AttributeType**](decimal.md)> |  | [optional]
**withdrawal_batch_number** | Option<**String**> |  | [optional]
**withdrawal_serial_numbers** | **Vec<String>** |  | 
**withdrawal_warehouse_level_id** | Option<**String**> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)

