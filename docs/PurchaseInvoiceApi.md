# \PurchaseInvoiceApi

All URIs are relative to *https://www.weclapp.com/webapp/api/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**purchase_invoice_count_get**](PurchaseInvoiceApi.md#purchase_invoice_count_get) | **GET** /purchaseInvoice/count | 
[**purchase_invoice_get**](PurchaseInvoiceApi.md#purchase_invoice_get) | **GET** /purchaseInvoice | 
[**purchase_invoice_id_id_delete**](PurchaseInvoiceApi.md#purchase_invoice_id_id_delete) | **DELETE** /purchaseInvoice/id/{id} | 
[**purchase_invoice_id_id_get**](PurchaseInvoiceApi.md#purchase_invoice_id_id_get) | **GET** /purchaseInvoice/id/{id} | 
[**purchase_invoice_id_id_put**](PurchaseInvoiceApi.md#purchase_invoice_id_id_put) | **PUT** /purchaseInvoice/id/{id} | 
[**purchase_invoice_post**](PurchaseInvoiceApi.md#purchase_invoice_post) | **POST** /purchaseInvoice | 



## purchase_invoice_count_get

> crate::models::AccountingTransactionCountGet200Response purchase_invoice_count_get(page, page_size, sort)


count purchaseInvoice

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


## purchase_invoice_get

> crate::models::PurchaseInvoiceGet200Response purchase_invoice_get(page, page_size, sort)


query purchaseInvoice

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page** | Option<**i32**> |  |  |
**page_size** | Option<**i32**> |  |  |
**sort** | Option<**String**> |  |  |

### Return type

[**crate::models::PurchaseInvoiceGet200Response**](_purchaseInvoice_get_200_response.md)

### Authorization

[API token](../README.md#API token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## purchase_invoice_id_id_delete

> purchase_invoice_id_id_delete(id)


delete a purchaseInvoice

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


## purchase_invoice_id_id_get

> crate::models::PurchaseInvoice purchase_invoice_id_id_get(id)


query purchaseInvoice

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

[**crate::models::PurchaseInvoice**](purchaseInvoice.md)

### Authorization

[API token](../README.md#API token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## purchase_invoice_id_id_put

> crate::models::PurchaseInvoice purchase_invoice_id_id_put(id, body)


update purchaseInvoice

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |
**body** | [**PurchaseInvoice**](PurchaseInvoice.md) |  | [required] |

### Return type

[**crate::models::PurchaseInvoice**](purchaseInvoice.md)

### Authorization

[API token](../README.md#API token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## purchase_invoice_post

> crate::models::PurchaseInvoice purchase_invoice_post(body)


create a purchaseInvoice

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**PurchaseInvoice**](PurchaseInvoice.md) |  | [required] |

### Return type

[**crate::models::PurchaseInvoice**](purchaseInvoice.md)

### Authorization

[API token](../README.md#API token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

