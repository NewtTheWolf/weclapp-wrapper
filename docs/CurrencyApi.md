# \CurrencyApi

All URIs are relative to *https://www.weclapp.com/webapp/api/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**currency_company_currency_get**](CurrencyApi.md#currency_company_currency_get) | **GET** /currency/companyCurrency | 
[**currency_count_get**](CurrencyApi.md#currency_count_get) | **GET** /currency/count | 
[**currency_get**](CurrencyApi.md#currency_get) | **GET** /currency | 
[**currency_id_id_get**](CurrencyApi.md#currency_id_id_get) | **GET** /currency/id/{id} | 



## currency_company_currency_get

> crate::models::CurrencyCompanyCurrencyGet200Response currency_company_currency_get()


### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::CurrencyCompanyCurrencyGet200Response**](_currency_companyCurrency_get_200_response.md)

### Authorization

[API token](../README.md#API token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## currency_count_get

> crate::models::AccountingTransactionCountGet200Response currency_count_get(page, page_size, sort)


count currency

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


## currency_get

> crate::models::CurrencyGet200Response currency_get(page, page_size, sort)


query currency

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page** | Option<**i32**> |  |  |
**page_size** | Option<**i32**> |  |  |
**sort** | Option<**String**> |  |  |

### Return type

[**crate::models::CurrencyGet200Response**](_currency_get_200_response.md)

### Authorization

[API token](../README.md#API token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## currency_id_id_get

> crate::models::Currency currency_id_id_get(id)


query currency

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

[**crate::models::Currency**](currency.md)

### Authorization

[API token](../README.md#API token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

