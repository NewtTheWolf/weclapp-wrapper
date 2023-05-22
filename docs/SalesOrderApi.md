# \SalesOrderApi

All URIs are relative to *https://www.weclapp.com/webapp/api/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**sales_order_count_get**](SalesOrderApi.md#sales_order_count_get) | **GET** /salesOrder/count | 
[**sales_order_default_values_for_create_get**](SalesOrderApi.md#sales_order_default_values_for_create_get) | **GET** /salesOrder/defaultValuesForCreate | 
[**sales_order_get**](SalesOrderApi.md#sales_order_get) | **GET** /salesOrder | 
[**sales_order_id_id_cancel_or_manually_close_post**](SalesOrderApi.md#sales_order_id_id_cancel_or_manually_close_post) | **POST** /salesOrder/id/{id}/cancelOrManuallyClose | 
[**sales_order_id_id_create_advance_payment_request_post**](SalesOrderApi.md#sales_order_id_id_create_advance_payment_request_post) | **POST** /salesOrder/id/{id}/createAdvancePaymentRequest | 
[**sales_order_id_id_create_customer_return_post**](SalesOrderApi.md#sales_order_id_id_create_customer_return_post) | **POST** /salesOrder/id/{id}/createCustomerReturn | 
[**sales_order_id_id_create_dropshipping_post**](SalesOrderApi.md#sales_order_id_id_create_dropshipping_post) | **POST** /salesOrder/id/{id}/createDropshipping | 
[**sales_order_id_id_create_part_payment_invoice_post**](SalesOrderApi.md#sales_order_id_id_create_part_payment_invoice_post) | **POST** /salesOrder/id/{id}/createPartPaymentInvoice | 
[**sales_order_id_id_create_prepayment_final_invoice_post**](SalesOrderApi.md#sales_order_id_id_create_prepayment_final_invoice_post) | **POST** /salesOrder/id/{id}/createPrepaymentFinalInvoice | 
[**sales_order_id_id_create_purchase_order_post**](SalesOrderApi.md#sales_order_id_id_create_purchase_order_post) | **POST** /salesOrder/id/{id}/createPurchaseOrder | 
[**sales_order_id_id_create_sales_invoice_post**](SalesOrderApi.md#sales_order_id_id_create_sales_invoice_post) | **POST** /salesOrder/id/{id}/createSalesInvoice | 
[**sales_order_id_id_create_shipment_post**](SalesOrderApi.md#sales_order_id_id_create_shipment_post) | **POST** /salesOrder/id/{id}/createShipment | 
[**sales_order_id_id_delete**](SalesOrderApi.md#sales_order_id_id_delete) | **DELETE** /salesOrder/id/{id} | 
[**sales_order_id_id_download_latest_order_confirmation_pdf_get**](SalesOrderApi.md#sales_order_id_id_download_latest_order_confirmation_pdf_get) | **GET** /salesOrder/id/{id}/downloadLatestOrderConfirmationPdf | 
[**sales_order_id_id_get**](SalesOrderApi.md#sales_order_id_id_get) | **GET** /salesOrder/id/{id} | 
[**sales_order_id_id_manually_close_post**](SalesOrderApi.md#sales_order_id_id_manually_close_post) | **POST** /salesOrder/id/{id}/manuallyClose | 
[**sales_order_id_id_put**](SalesOrderApi.md#sales_order_id_id_put) | **PUT** /salesOrder/id/{id} | 
[**sales_order_id_id_ship_order_for_external_fulfillment_post**](SalesOrderApi.md#sales_order_id_id_ship_order_for_external_fulfillment_post) | **POST** /salesOrder/id/{id}/shipOrderForExternalFulfillment | 
[**sales_order_id_id_toggle_project_team_post**](SalesOrderApi.md#sales_order_id_id_toggle_project_team_post) | **POST** /salesOrder/id/{id}/toggleProjectTeam | 
[**sales_order_post**](SalesOrderApi.md#sales_order_post) | **POST** /salesOrder | 



## sales_order_count_get

> crate::models::AccountingTransactionCountGet200Response sales_order_count_get(page, page_size, sort)


count salesOrder

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page** | Option<**i32**> |  |  |
**page_size** | Option<**i32**> |  |  |
**sort** | Option<**String**> |  |  |

### Return type

[**crate::models::AccountingTransactionCountGet200Response**](_accountingTransaction_count_get_200_response.md)

### Authorization

[API token](../README.md#API token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## sales_order_default_values_for_create_get

> crate::models::SalesOrderDefaultValuesForCreateGet200Response sales_order_default_values_for_create_get(customer_id, responsible_user_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**customer_id** | **String** |  | [required] |
**responsible_user_id** | Option<**String**> |  |  |

### Return type

[**crate::models::SalesOrderDefaultValuesForCreateGet200Response**](_salesOrder_defaultValuesForCreate_get_200_response.md)

### Authorization

[API token](../README.md#API token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## sales_order_get

> crate::models::SalesOrderGet200Response sales_order_get(page, page_size, sort)


query salesOrder

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page** | Option<**i32**> |  |  |
**page_size** | Option<**i32**> |  |  |
**sort** | Option<**String**> |  |  |

### Return type

[**crate::models::SalesOrderGet200Response**](_salesOrder_get_200_response.md)

### Authorization

[API token](../README.md#API token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## sales_order_id_id_cancel_or_manually_close_post

> crate::models::SalesOrderDefaultValuesForCreateGet200Response sales_order_id_id_cancel_or_manually_close_post(id, body)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |
**body** | **serde_json::Value** |  | [required] |

### Return type

[**crate::models::SalesOrderDefaultValuesForCreateGet200Response**](_salesOrder_defaultValuesForCreate_get_200_response.md)

### Authorization

[API token](../README.md#API token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## sales_order_id_id_create_advance_payment_request_post

> crate::models::IncomingGoodsIdIdCreateCreditNotePost200Response sales_order_id_id_create_advance_payment_request_post(id, body)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |
**body** | **serde_json::Value** |  | [required] |

### Return type

[**crate::models::IncomingGoodsIdIdCreateCreditNotePost200Response**](_incomingGoods_id__id__createCreditNote_post_200_response.md)

### Authorization

[API token](../README.md#API token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## sales_order_id_id_create_customer_return_post

> crate::models::IncomingGoodsIdIdAddPurchaseOrdersPost200Response sales_order_id_id_create_customer_return_post(id, body)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |
**body** | **serde_json::Value** |  | [required] |

### Return type

[**crate::models::IncomingGoodsIdIdAddPurchaseOrdersPost200Response**](_incomingGoods_id__id__addPurchaseOrders_post_200_response.md)

### Authorization

[API token](../README.md#API token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## sales_order_id_id_create_dropshipping_post

> crate::models::PurchaseOrderIdIdProcessDropshippingPost200Response sales_order_id_id_create_dropshipping_post(id, body)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |
**body** | [**SalesOrderIdIdCreateDropshippingPostRequest**](SalesOrderIdIdCreateDropshippingPostRequest.md) |  | [required] |

### Return type

[**crate::models::PurchaseOrderIdIdProcessDropshippingPost200Response**](_purchaseOrder_id__id__processDropshipping_post_200_response.md)

### Authorization

[API token](../README.md#API token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## sales_order_id_id_create_part_payment_invoice_post

> crate::models::IncomingGoodsIdIdCreateCreditNotePost200Response sales_order_id_id_create_part_payment_invoice_post(id, body)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |
**body** | **serde_json::Value** |  | [required] |

### Return type

[**crate::models::IncomingGoodsIdIdCreateCreditNotePost200Response**](_incomingGoods_id__id__createCreditNote_post_200_response.md)

### Authorization

[API token](../README.md#API token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## sales_order_id_id_create_prepayment_final_invoice_post

> crate::models::IncomingGoodsIdIdCreateCreditNotePost200Response sales_order_id_id_create_prepayment_final_invoice_post(id, body)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |
**body** | **serde_json::Value** |  | [required] |

### Return type

[**crate::models::IncomingGoodsIdIdCreateCreditNotePost200Response**](_incomingGoods_id__id__createCreditNote_post_200_response.md)

### Authorization

[API token](../README.md#API token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## sales_order_id_id_create_purchase_order_post

> crate::models::PurchaseOrderIdIdProcessDropshippingPost200Response sales_order_id_id_create_purchase_order_post(id, body)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |
**body** | [**SalesOrderIdIdCreatePurchaseOrderPostRequest**](SalesOrderIdIdCreatePurchaseOrderPostRequest.md) |  | [required] |

### Return type

[**crate::models::PurchaseOrderIdIdProcessDropshippingPost200Response**](_purchaseOrder_id__id__processDropshipping_post_200_response.md)

### Authorization

[API token](../README.md#API token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## sales_order_id_id_create_sales_invoice_post

> crate::models::IncomingGoodsIdIdCreateCreditNotePost200Response sales_order_id_id_create_sales_invoice_post(id, body)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |
**body** | [**SalesOrderIdIdCreateSalesInvoicePostRequest**](SalesOrderIdIdCreateSalesInvoicePostRequest.md) |  | [required] |

### Return type

[**crate::models::IncomingGoodsIdIdCreateCreditNotePost200Response**](_incomingGoods_id__id__createCreditNote_post_200_response.md)

### Authorization

[API token](../README.md#API token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## sales_order_id_id_create_shipment_post

> crate::models::IncomingGoodsIdIdCreateCompensationShipmentPost200Response sales_order_id_id_create_shipment_post(id, body)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |
**body** | **serde_json::Value** |  | [required] |

### Return type

[**crate::models::IncomingGoodsIdIdCreateCompensationShipmentPost200Response**](_incomingGoods_id__id__createCompensationShipment_post_200_response.md)

### Authorization

[API token](../README.md#API token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## sales_order_id_id_delete

> sales_order_id_id_delete(id)


delete a salesOrder

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

[API token](../README.md#API token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## sales_order_id_id_download_latest_order_confirmation_pdf_get

> sales_order_id_id_download_latest_order_confirmation_pdf_get(id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

[API token](../README.md#API token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: image/jpeg, image/png, application/pdf, */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## sales_order_id_id_get

> crate::models::SalesOrder sales_order_id_id_get(id)


query salesOrder

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

[**crate::models::SalesOrder**](salesOrder.md)

### Authorization

[API token](../README.md#API token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## sales_order_id_id_manually_close_post

> crate::models::SalesOrderDefaultValuesForCreateGet200Response sales_order_id_id_manually_close_post(id, body)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |
**body** | **serde_json::Value** |  | [required] |

### Return type

[**crate::models::SalesOrderDefaultValuesForCreateGet200Response**](_salesOrder_defaultValuesForCreate_get_200_response.md)

### Authorization

[API token](../README.md#API token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## sales_order_id_id_put

> crate::models::SalesOrder sales_order_id_id_put(id, body)


update salesOrder

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |
**body** | [**SalesOrder**](SalesOrder.md) |  | [required] |

### Return type

[**crate::models::SalesOrder**](salesOrder.md)

### Authorization

[API token](../README.md#API token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## sales_order_id_id_ship_order_for_external_fulfillment_post

> crate::models::SalesOrderDefaultValuesForCreateGet200Response sales_order_id_id_ship_order_for_external_fulfillment_post(id, body)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |
**body** | **serde_json::Value** |  | [required] |

### Return type

[**crate::models::SalesOrderDefaultValuesForCreateGet200Response**](_salesOrder_defaultValuesForCreate_get_200_response.md)

### Authorization

[API token](../README.md#API token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## sales_order_id_id_toggle_project_team_post

> crate::models::SalesOrderDefaultValuesForCreateGet200Response sales_order_id_id_toggle_project_team_post(id, body)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |
**body** | **serde_json::Value** |  | [required] |

### Return type

[**crate::models::SalesOrderDefaultValuesForCreateGet200Response**](_salesOrder_defaultValuesForCreate_get_200_response.md)

### Authorization

[API token](../README.md#API token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## sales_order_post

> crate::models::SalesOrder sales_order_post(body)


create a salesOrder

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**SalesOrder**](SalesOrder.md) |  | [required] |

### Return type

[**crate::models::SalesOrder**](salesOrder.md)

### Authorization

[API token](../README.md#API token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

