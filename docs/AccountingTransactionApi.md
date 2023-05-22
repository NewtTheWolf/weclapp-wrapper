# \AccountingTransactionApi

All URIs are relative to *https://www.weclapp.com/webapp/api/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**accounting_transaction_count_get**](AccountingTransactionApi.md#accounting_transaction_count_get) | **GET** /accountingTransaction/count | 
[**accounting_transaction_get**](AccountingTransactionApi.md#accounting_transaction_get) | **GET** /accountingTransaction | 
[**accounting_transaction_id_id_get**](AccountingTransactionApi.md#accounting_transaction_id_id_get) | **GET** /accountingTransaction/id/{id} | 



## accounting_transaction_count_get

> crate::models::AccountingTransactionCountGet200Response accounting_transaction_count_get(page, page_size, sort)


count accountingTransaction

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


## accounting_transaction_get

> crate::models::AccountingTransactionGet200Response accounting_transaction_get(page, page_size, sort)


query accountingTransaction

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page** | Option<**i32**> |  |  |
**page_size** | Option<**i32**> |  |  |
**sort** | Option<**String**> |  |  |

### Return type

[**crate::models::AccountingTransactionGet200Response**](_accountingTransaction_get_200_response.md)

### Authorization

[API token](../README.md#API token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## accounting_transaction_id_id_get

> crate::models::AccountingTransaction accounting_transaction_id_id_get(id)


query accountingTransaction

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

[**crate::models::AccountingTransaction**](accountingTransaction.md)

### Authorization

[API token](../README.md#API token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

