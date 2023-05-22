# \PurchaseOrderApi

All URIs are relative to *https://www.weclapp.com/webapp/api/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**purchase_order_count_get**](PurchaseOrderApi.md#purchase_order_count_get) | **GET** /purchaseOrder/count | 
[**purchase_order_get**](PurchaseOrderApi.md#purchase_order_get) | **GET** /purchaseOrder | 
[**purchase_order_id_id_create_dropshipping_delivery_note_pdf_post**](PurchaseOrderApi.md#purchase_order_id_id_create_dropshipping_delivery_note_pdf_post) | **POST** /purchaseOrder/id/{id}/createDropshippingDeliveryNotePdf | 
[**purchase_order_id_id_create_incoming_goods_post**](PurchaseOrderApi.md#purchase_order_id_id_create_incoming_goods_post) | **POST** /purchaseOrder/id/{id}/createIncomingGoods | 
[**purchase_order_id_id_create_purchase_invoice_post**](PurchaseOrderApi.md#purchase_order_id_id_create_purchase_invoice_post) | **POST** /purchaseOrder/id/{id}/createPurchaseInvoice | 
[**purchase_order_id_id_create_supplier_return_post**](PurchaseOrderApi.md#purchase_order_id_id_create_supplier_return_post) | **POST** /purchaseOrder/id/{id}/createSupplierReturn | 
[**purchase_order_id_id_delete**](PurchaseOrderApi.md#purchase_order_id_id_delete) | **DELETE** /purchaseOrder/id/{id} | 
[**purchase_order_id_id_download_latest_dropshipping_delivery_note_pdf_get**](PurchaseOrderApi.md#purchase_order_id_id_download_latest_dropshipping_delivery_note_pdf_get) | **GET** /purchaseOrder/id/{id}/downloadLatestDropshippingDeliveryNotePdf | 
[**purchase_order_id_id_download_latest_purchase_order_pdf_get**](PurchaseOrderApi.md#purchase_order_id_id_download_latest_purchase_order_pdf_get) | **GET** /purchaseOrder/id/{id}/downloadLatestPurchaseOrderPdf | 
[**purchase_order_id_id_get**](PurchaseOrderApi.md#purchase_order_id_id_get) | **GET** /purchaseOrder/id/{id} | 
[**purchase_order_id_id_process_dropshipping_post**](PurchaseOrderApi.md#purchase_order_id_id_process_dropshipping_post) | **POST** /purchaseOrder/id/{id}/processDropshipping | 
[**purchase_order_id_id_put**](PurchaseOrderApi.md#purchase_order_id_id_put) | **PUT** /purchaseOrder/id/{id} | 
[**purchase_order_post**](PurchaseOrderApi.md#purchase_order_post) | **POST** /purchaseOrder | 



## purchase_order_count_get

> crate::models::AccountingTransactionCountGet200Response purchase_order_count_get(page, page_size, sort)


count purchaseOrder

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


## purchase_order_get

> crate::models::PurchaseOrderGet200Response purchase_order_get(page, page_size, sort)


query purchaseOrder

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page** | Option<**i32**> |  |  |
**page_size** | Option<**i32**> |  |  |
**sort** | Option<**String**> |  |  |

### Return type

[**crate::models::PurchaseOrderGet200Response**](_purchaseOrder_get_200_response.md)

### Authorization

[API token](../README.md#API token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## purchase_order_id_id_create_dropshipping_delivery_note_pdf_post

> purchase_order_id_id_create_dropshipping_delivery_note_pdf_post(id, body)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |
**body** | **serde_json::Value** |  | [required] |

### Return type

 (empty response body)

### Authorization

[API token](../README.md#API token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: image/jpeg, image/png, application/pdf, */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## purchase_order_id_id_create_incoming_goods_post

> crate::models::IncomingGoodsIdIdAddPurchaseOrdersPost200Response purchase_order_id_id_create_incoming_goods_post(id, body)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |
**body** | [**PurchaseOrderIdIdCreateIncomingGoodsPostRequest**](PurchaseOrderIdIdCreateIncomingGoodsPostRequest.md) |  | [required] |

### Return type

[**crate::models::IncomingGoodsIdIdAddPurchaseOrdersPost200Response**](_incomingGoods_id__id__addPurchaseOrders_post_200_response.md)

### Authorization

[API token](../README.md#API token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## purchase_order_id_id_create_purchase_invoice_post

> crate::models::PurchaseOrderIdIdCreatePurchaseInvoicePost200Response purchase_order_id_id_create_purchase_invoice_post(id, body)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |
**body** | **serde_json::Value** |  | [required] |

### Return type

[**crate::models::PurchaseOrderIdIdCreatePurchaseInvoicePost200Response**](_purchaseOrder_id__id__createPurchaseInvoice_post_200_response.md)

### Authorization

[API token](../README.md#API token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## purchase_order_id_id_create_supplier_return_post

> crate::models::IncomingGoodsIdIdCreateCompensationShipmentPost200Response purchase_order_id_id_create_supplier_return_post(id, body)


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


## purchase_order_id_id_delete

> purchase_order_id_id_delete(id)


delete a purchaseOrder

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


## purchase_order_id_id_download_latest_dropshipping_delivery_note_pdf_get

> purchase_order_id_id_download_latest_dropshipping_delivery_note_pdf_get(id)


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


## purchase_order_id_id_download_latest_purchase_order_pdf_get

> purchase_order_id_id_download_latest_purchase_order_pdf_get(id)


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


## purchase_order_id_id_get

> crate::models::PurchaseOrder purchase_order_id_id_get(id)


query purchaseOrder

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

[**crate::models::PurchaseOrder**](purchaseOrder.md)

### Authorization

[API token](../README.md#API token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## purchase_order_id_id_process_dropshipping_post

> crate::models::PurchaseOrderIdIdProcessDropshippingPost200Response purchase_order_id_id_process_dropshipping_post(id, body)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |
**body** | **serde_json::Value** |  | [required] |

### Return type

[**crate::models::PurchaseOrderIdIdProcessDropshippingPost200Response**](_purchaseOrder_id__id__processDropshipping_post_200_response.md)

### Authorization

[API token](../README.md#API token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## purchase_order_id_id_put

> crate::models::PurchaseOrder purchase_order_id_id_put(id, body)


update purchaseOrder

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |
**body** | [**PurchaseOrder**](PurchaseOrder.md) |  | [required] |

### Return type

[**crate::models::PurchaseOrder**](purchaseOrder.md)

### Authorization

[API token](../README.md#API token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## purchase_order_post

> crate::models::PurchaseOrder purchase_order_post(body)


create a purchaseOrder

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**PurchaseOrder**](PurchaseOrder.md) |  | [required] |

### Return type

[**crate::models::PurchaseOrder**](purchaseOrder.md)

### Authorization

[API token](../README.md#API token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

