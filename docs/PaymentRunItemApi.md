# \PaymentRunItemApi

All URIs are relative to *https://www.weclapp.com/webapp/api/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**payment_run_item_count_get**](PaymentRunItemApi.md#payment_run_item_count_get) | **GET** /paymentRunItem/count | 
[**payment_run_item_get**](PaymentRunItemApi.md#payment_run_item_get) | **GET** /paymentRunItem | 
[**payment_run_item_id_id_get**](PaymentRunItemApi.md#payment_run_item_id_id_get) | **GET** /paymentRunItem/id/{id} | 



## payment_run_item_count_get

> crate::models::AccountingTransactionCountGet200Response payment_run_item_count_get(page, page_size, sort)


count paymentRunItem

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


## payment_run_item_get

> crate::models::PaymentRunItemGet200Response payment_run_item_get(page, page_size, sort)


query paymentRunItem

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page** | Option<**i32**> |  |  |
**page_size** | Option<**i32**> |  |  |
**sort** | Option<**String**> |  |  |

### Return type

[**crate::models::PaymentRunItemGet200Response**](_paymentRunItem_get_200_response.md)

### Authorization

[API token](../README.md#API token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## payment_run_item_id_id_get

> crate::models::PaymentRunItem payment_run_item_id_id_get(id)


query paymentRunItem

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

[**crate::models::PaymentRunItem**](paymentRunItem.md)

### Authorization

[API token](../README.md#API token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

