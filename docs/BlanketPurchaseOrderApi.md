# \BlanketPurchaseOrderApi

All URIs are relative to *https://www.weclapp.com/webapp/api/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**blanket_purchase_order_count_get**](BlanketPurchaseOrderApi.md#blanket_purchase_order_count_get) | **GET** /blanketPurchaseOrder/count | 
[**blanket_purchase_order_get**](BlanketPurchaseOrderApi.md#blanket_purchase_order_get) | **GET** /blanketPurchaseOrder | 
[**blanket_purchase_order_id_id_delete**](BlanketPurchaseOrderApi.md#blanket_purchase_order_id_id_delete) | **DELETE** /blanketPurchaseOrder/id/{id} | 
[**blanket_purchase_order_id_id_download_latest_blanket_purchase_order_pdf_get**](BlanketPurchaseOrderApi.md#blanket_purchase_order_id_id_download_latest_blanket_purchase_order_pdf_get) | **GET** /blanketPurchaseOrder/id/{id}/downloadLatestBlanketPurchaseOrderPdf | 
[**blanket_purchase_order_id_id_generate_releases_post**](BlanketPurchaseOrderApi.md#blanket_purchase_order_id_id_generate_releases_post) | **POST** /blanketPurchaseOrder/id/{id}/generateReleases | 
[**blanket_purchase_order_id_id_get**](BlanketPurchaseOrderApi.md#blanket_purchase_order_id_id_get) | **GET** /blanketPurchaseOrder/id/{id} | 
[**blanket_purchase_order_id_id_put**](BlanketPurchaseOrderApi.md#blanket_purchase_order_id_id_put) | **PUT** /blanketPurchaseOrder/id/{id} | 
[**blanket_purchase_order_post**](BlanketPurchaseOrderApi.md#blanket_purchase_order_post) | **POST** /blanketPurchaseOrder | 



## blanket_purchase_order_count_get

> crate::models::AccountingTransactionCountGet200Response blanket_purchase_order_count_get(page, page_size, sort)


count blanketPurchaseOrder

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


## blanket_purchase_order_get

> crate::models::BlanketPurchaseOrderGet200Response blanket_purchase_order_get(page, page_size, sort)


query blanketPurchaseOrder

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page** | Option<**i32**> |  |  |
**page_size** | Option<**i32**> |  |  |
**sort** | Option<**String**> |  |  |

### Return type

[**crate::models::BlanketPurchaseOrderGet200Response**](_blanketPurchaseOrder_get_200_response.md)

### Authorization

[API token](../README.md#API token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## blanket_purchase_order_id_id_delete

> blanket_purchase_order_id_id_delete(id)


delete a blanketPurchaseOrder

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


## blanket_purchase_order_id_id_download_latest_blanket_purchase_order_pdf_get

> blanket_purchase_order_id_id_download_latest_blanket_purchase_order_pdf_get(id)


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


## blanket_purchase_order_id_id_generate_releases_post

> crate::models::BlanketPurchaseOrderIdIdGenerateReleasesPost200Response blanket_purchase_order_id_id_generate_releases_post(id, body)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |
**body** | [**BlanketPurchaseOrderIdIdGenerateReleasesPostRequest**](BlanketPurchaseOrderIdIdGenerateReleasesPostRequest.md) |  | [required] |

### Return type

[**crate::models::BlanketPurchaseOrderIdIdGenerateReleasesPost200Response**](_blanketPurchaseOrder_id__id__generateReleases_post_200_response.md)

### Authorization

[API token](../README.md#API token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## blanket_purchase_order_id_id_get

> crate::models::BlanketPurchaseOrder blanket_purchase_order_id_id_get(id)


query blanketPurchaseOrder

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

[**crate::models::BlanketPurchaseOrder**](blanketPurchaseOrder.md)

### Authorization

[API token](../README.md#API token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## blanket_purchase_order_id_id_put

> crate::models::BlanketPurchaseOrder blanket_purchase_order_id_id_put(id, body)


update blanketPurchaseOrder

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |
**body** | [**BlanketPurchaseOrder**](BlanketPurchaseOrder.md) |  | [required] |

### Return type

[**crate::models::BlanketPurchaseOrder**](blanketPurchaseOrder.md)

### Authorization

[API token](../README.md#API token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## blanket_purchase_order_post

> crate::models::BlanketPurchaseOrder blanket_purchase_order_post(body)


create a blanketPurchaseOrder

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**BlanketPurchaseOrder**](BlanketPurchaseOrder.md) |  | [required] |

### Return type

[**crate::models::BlanketPurchaseOrder**](blanketPurchaseOrder.md)

### Authorization

[API token](../README.md#API token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

