# \PaymentRunApi

All URIs are relative to *https://www.weclapp.com/webapp/api/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**payment_run_count_get**](PaymentRunApi.md#payment_run_count_get) | **GET** /paymentRun/count | 
[**payment_run_get**](PaymentRunApi.md#payment_run_get) | **GET** /paymentRun | 
[**payment_run_id_id_delete**](PaymentRunApi.md#payment_run_id_id_delete) | **DELETE** /paymentRun/id/{id} | 
[**payment_run_id_id_get**](PaymentRunApi.md#payment_run_id_id_get) | **GET** /paymentRun/id/{id} | 
[**payment_run_id_id_put**](PaymentRunApi.md#payment_run_id_id_put) | **PUT** /paymentRun/id/{id} | 



## payment_run_count_get

> crate::models::AccountingTransactionCountGet200Response payment_run_count_get(page, page_size, sort)


count paymentRun

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


## payment_run_get

> crate::models::PaymentRunGet200Response payment_run_get(page, page_size, sort)


query paymentRun

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page** | Option<**i32**> |  |  |
**page_size** | Option<**i32**> |  |  |
**sort** | Option<**String**> |  |  |

### Return type

[**crate::models::PaymentRunGet200Response**](_paymentRun_get_200_response.md)

### Authorization

[API token](../README.md#API token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## payment_run_id_id_delete

> payment_run_id_id_delete(id)


delete a paymentRun

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


## payment_run_id_id_get

> crate::models::PaymentRun payment_run_id_id_get(id)


query paymentRun

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

[**crate::models::PaymentRun**](paymentRun.md)

### Authorization

[API token](../README.md#API token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## payment_run_id_id_put

> crate::models::PaymentRun payment_run_id_id_put(id, body)


update paymentRun

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |
**body** | [**PaymentRun**](PaymentRun.md) |  | [required] |

### Return type

[**crate::models::PaymentRun**](paymentRun.md)

### Authorization

[API token](../README.md#API token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

