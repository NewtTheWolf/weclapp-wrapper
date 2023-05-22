# \PurchaseOpenItemApi

All URIs are relative to *https://www.weclapp.com/webapp/api/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**purchase_open_item_count_get**](PurchaseOpenItemApi.md#purchase_open_item_count_get) | **GET** /purchaseOpenItem/count | 
[**purchase_open_item_get**](PurchaseOpenItemApi.md#purchase_open_item_get) | **GET** /purchaseOpenItem | 
[**purchase_open_item_id_id_create_payment_application_post**](PurchaseOpenItemApi.md#purchase_open_item_id_id_create_payment_application_post) | **POST** /purchaseOpenItem/id/{id}/createPaymentApplication | 
[**purchase_open_item_id_id_get**](PurchaseOpenItemApi.md#purchase_open_item_id_id_get) | **GET** /purchaseOpenItem/id/{id} | 



## purchase_open_item_count_get

> crate::models::AccountingTransactionCountGet200Response purchase_open_item_count_get(page, page_size, sort)


count purchaseOpenItem

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


## purchase_open_item_get

> crate::models::PurchaseOpenItemGet200Response purchase_open_item_get(page, page_size, sort)


query purchaseOpenItem

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page** | Option<**i32**> |  |  |
**page_size** | Option<**i32**> |  |  |
**sort** | Option<**String**> |  |  |

### Return type

[**crate::models::PurchaseOpenItemGet200Response**](_purchaseOpenItem_get_200_response.md)

### Authorization

[API token](../README.md#API token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## purchase_open_item_id_id_create_payment_application_post

> crate::models::PurchaseOpenItemIdIdCreatePaymentApplicationPost200Response purchase_open_item_id_id_create_payment_application_post(id, body)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |
**body** | [**PurchaseOpenItemIdIdCreatePaymentApplicationPostRequest**](PurchaseOpenItemIdIdCreatePaymentApplicationPostRequest.md) |  | [required] |

### Return type

[**crate::models::PurchaseOpenItemIdIdCreatePaymentApplicationPost200Response**](_purchaseOpenItem_id__id__createPaymentApplication_post_200_response.md)

### Authorization

[API token](../README.md#API token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## purchase_open_item_id_id_get

> crate::models::PurchaseOpenItem purchase_open_item_id_id_get(id)


query purchaseOpenItem

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

[**crate::models::PurchaseOpenItem**](purchaseOpenItem.md)

### Authorization

[API token](../README.md#API token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

