# PurchaseInvoiceItem

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**String**> |  | [optional]
**version** | Option<**String**> |  | [optional]
**account_id** | Option<**String**> |  | [optional]
**account_number** | Option<**String**> |  | [optional]
**article_id** | Option<**String**> |  | [optional]
**article_number** | Option<**String**> |  | [optional]
**cost_center_items** | Option<[**Vec<crate::models::CostCenterWithDistributionPercentage>**](costCenterWithDistributionPercentage.md)> |  | [optional]
**created_date** | Option<**i32**> |  | [optional]
**credited_invoice_item_id** | Option<**String**> |  | [optional]
**custom_attributes** | Option<[**Vec<crate::models::CustomAttribute>**](customAttribute.md)> |  | [optional]
**description** | Option<**String**> |  | [optional]
**description_fixed** | Option<**bool**> |  | [optional]
**discount_percentage** | Option<[**crate::models::custom_attribute_definition::AttributeType**](decimal.md)> |  | [optional]
**gross_amount** | Option<[**crate::models::custom_attribute_definition::AttributeType**](decimal.md)> |  | [optional]
**gross_amount_in_company_currency** | Option<[**crate::models::custom_attribute_definition::AttributeType**](decimal.md)> |  | [optional]
**last_modified_date** | Option<**i32**> |  | [optional]
**manual_quantity** | Option<**bool**> |  | [optional]
**manual_unit_price** | Option<**bool**> |  | [optional]
**net_amount** | Option<[**crate::models::custom_attribute_definition::AttributeType**](decimal.md)> |  | [optional]
**net_amount_for_statistics** | Option<[**crate::models::custom_attribute_definition::AttributeType**](decimal.md)> |  | [optional]
**net_amount_for_statistics_in_company_currency** | Option<[**crate::models::custom_attribute_definition::AttributeType**](decimal.md)> |  | [optional]
**net_amount_in_company_currency** | Option<[**crate::models::custom_attribute_definition::AttributeType**](decimal.md)> |  | [optional]
**note** | Option<**String**> |  | [optional]
**parent_item_id** | Option<**String**> |  | [optional]
**position_number** | Option<**i32**> |  | [optional]
**purchase_invoice_item_relationship** | Option<[**Vec<crate::models::PurchaseInvoiceItemRelationship>**](purchaseInvoiceItemRelationship.md)> |  | [optional]
**quantity** | Option<[**crate::models::custom_attribute_definition::AttributeType**](decimal.md)> |  | [optional]
**reduction_addition_items** | Option<[**Vec<crate::models::ReductionAdditionItem>**](reductionAdditionItem.md)> |  | [optional]
**supplier_article_id** | Option<**String**> |  | [optional]
**tax_id** | Option<**String**> |  | [optional]
**tax_name** | Option<**String**> |  | [optional]
**title** | Option<**String**> |  | [optional]
**unit_id** | Option<**String**> |  | [optional]
**unit_name** | Option<**String**> |  | [optional]
**unit_price** | Option<[**crate::models::custom_attribute_definition::AttributeType**](decimal.md)> |  | [optional]
**unit_price_in_company_currency** | Option<[**crate::models::custom_attribute_definition::AttributeType**](decimal.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


