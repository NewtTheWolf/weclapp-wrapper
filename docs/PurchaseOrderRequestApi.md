# \PurchaseOrderRequestApi

All URIs are relative to *https://www.weclapp.com/webapp/api/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**purchase_order_request_count_get**](PurchaseOrderRequestApi.md#purchase_order_request_count_get) | **GET** /purchaseOrderRequest/count | 
[**purchase_order_request_get**](PurchaseOrderRequestApi.md#purchase_order_request_get) | **GET** /purchaseOrderRequest | 
[**purchase_order_request_id_id_create_blanket_purchase_order_post**](PurchaseOrderRequestApi.md#purchase_order_request_id_id_create_blanket_purchase_order_post) | **POST** /purchaseOrderRequest/id/{id}/createBlanketPurchaseOrder | 
[**purchase_order_request_id_id_create_purchase_order_post**](PurchaseOrderRequestApi.md#purchase_order_request_id_id_create_purchase_order_post) | **POST** /purchaseOrderRequest/id/{id}/createPurchaseOrder | 
[**purchase_order_request_id_id_delete**](PurchaseOrderRequestApi.md#purchase_order_request_id_id_delete) | **DELETE** /purchaseOrderRequest/id/{id} | 
[**purchase_order_request_id_id_get**](PurchaseOrderRequestApi.md#purchase_order_request_id_id_get) | **GET** /purchaseOrderRequest/id/{id} | 
[**purchase_order_request_id_id_put**](PurchaseOrderRequestApi.md#purchase_order_request_id_id_put) | **PUT** /purchaseOrderRequest/id/{id} | 
[**purchase_order_request_post**](PurchaseOrderRequestApi.md#purchase_order_request_post) | **POST** /purchaseOrderRequest | 



## purchase_order_request_count_get

> crate::models::AccountingTransactionCountGet200Response purchase_order_request_count_get(page, page_size, sort)


count purchaseOrderRequest

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


## purchase_order_request_get

> crate::models::PurchaseOrderRequestGet200Response purchase_order_request_get(page, page_size, sort)


query purchaseOrderRequest

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page** | Option<**i32**> |  |  |
**page_size** | Option<**i32**> |  |  |
**sort** | Option<**String**> |  |  |

### Return type

[**crate::models::PurchaseOrderRequestGet200Response**](_purchaseOrderRequest_get_200_response.md)

### Authorization

[API token](../README.md#API token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## purchase_order_request_id_id_create_blanket_purchase_order_post

> crate::models::PurchaseOrderRequestIdIdCreateBlanketPurchaseOrderPost200Response purchase_order_request_id_id_create_blanket_purchase_order_post(id, body)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |
**body** | [**PurchaseOrderRequestIdIdCreateBlanketPurchaseOrderPostRequest**](PurchaseOrderRequestIdIdCreateBlanketPurchaseOrderPostRequest.md) |  | [required] |

### Return type

[**crate::models::PurchaseOrderRequestIdIdCreateBlanketPurchaseOrderPost200Response**](_purchaseOrderRequest_id__id__createBlanketPurchaseOrder_post_200_response.md)

### Authorization

[API token](../README.md#API token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## purchase_order_request_id_id_create_purchase_order_post

> crate::models::PurchaseOrderIdIdProcessDropshippingPost200Response purchase_order_request_id_id_create_purchase_order_post(id, body)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |
**body** | [**PurchaseOrderRequestIdIdCreatePurchaseOrderPostRequest**](PurchaseOrderRequestIdIdCreatePurchaseOrderPostRequest.md) |  | [required] |

### Return type

[**crate::models::PurchaseOrderIdIdProcessDropshippingPost200Response**](_purchaseOrder_id__id__processDropshipping_post_200_response.md)

### Authorization

[API token](../README.md#API token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## purchase_order_request_id_id_delete

> purchase_order_request_id_id_delete(id)


delete a purchaseOrderRequest

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


## purchase_order_request_id_id_get

> crate::models::PurchaseOrderRequest purchase_order_request_id_id_get(id)


query purchaseOrderRequest

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

[**crate::models::PurchaseOrderRequest**](purchaseOrderRequest.md)

### Authorization

[API token](../README.md#API token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## purchase_order_request_id_id_put

> crate::models::PurchaseOrderRequest purchase_order_request_id_id_put(id, body)


update purchaseOrderRequest

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |
**body** | [**PurchaseOrderRequest**](PurchaseOrderRequest.md) |  | [required] |

### Return type

[**crate::models::PurchaseOrderRequest**](purchaseOrderRequest.md)

### Authorization

[API token](../README.md#API token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## purchase_order_request_post

> crate::models::PurchaseOrderRequest purchase_order_request_post(body)


create a purchaseOrderRequest

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**PurchaseOrderRequest**](PurchaseOrderRequest.md) |  | [required] |

### Return type

[**crate::models::PurchaseOrderRequest**](purchaseOrderRequest.md)

### Authorization

[API token](../README.md#API token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

