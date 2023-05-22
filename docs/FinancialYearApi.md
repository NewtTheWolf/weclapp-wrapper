# \FinancialYearApi

All URIs are relative to *https://www.weclapp.com/webapp/api/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**financial_year_count_get**](FinancialYearApi.md#financial_year_count_get) | **GET** /financialYear/count | 
[**financial_year_get**](FinancialYearApi.md#financial_year_get) | **GET** /financialYear | 
[**financial_year_id_id_delete**](FinancialYearApi.md#financial_year_id_id_delete) | **DELETE** /financialYear/id/{id} | 
[**financial_year_id_id_generate_periods_post**](FinancialYearApi.md#financial_year_id_id_generate_periods_post) | **POST** /financialYear/id/{id}/generatePeriods | 
[**financial_year_id_id_get**](FinancialYearApi.md#financial_year_id_id_get) | **GET** /financialYear/id/{id} | 
[**financial_year_id_id_put**](FinancialYearApi.md#financial_year_id_id_put) | **PUT** /financialYear/id/{id} | 
[**financial_year_post**](FinancialYearApi.md#financial_year_post) | **POST** /financialYear | 



## financial_year_count_get

> crate::models::AccountingTransactionCountGet200Response financial_year_count_get(page, page_size, sort)


count financialYear

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


## financial_year_get

> crate::models::FinancialYearGet200Response financial_year_get(page, page_size, sort)


query financialYear

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page** | Option<**i32**> |  |  |
**page_size** | Option<**i32**> |  |  |
**sort** | Option<**String**> |  |  |

### Return type

[**crate::models::FinancialYearGet200Response**](_financialYear_get_200_response.md)

### Authorization

[API token](../README.md#API token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## financial_year_id_id_delete

> financial_year_id_id_delete(id)


delete a financialYear

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


## financial_year_id_id_generate_periods_post

> crate::models::FinancialYearIdIdGeneratePeriodsPost200Response financial_year_id_id_generate_periods_post(id, body)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |
**body** | **serde_json::Value** |  | [required] |

### Return type

[**crate::models::FinancialYearIdIdGeneratePeriodsPost200Response**](_financialYear_id__id__generatePeriods_post_200_response.md)

### Authorization

[API token](../README.md#API token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## financial_year_id_id_get

> crate::models::FinancialYear financial_year_id_id_get(id)


query financialYear

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

[**crate::models::FinancialYear**](financialYear.md)

### Authorization

[API token](../README.md#API token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## financial_year_id_id_put

> crate::models::FinancialYear financial_year_id_id_put(id, body)


update financialYear

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |
**body** | [**FinancialYear**](FinancialYear.md) |  | [required] |

### Return type

[**crate::models::FinancialYear**](financialYear.md)

### Authorization

[API token](../README.md#API token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## financial_year_post

> crate::models::FinancialYear financial_year_post(body)


create a financialYear

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**FinancialYear**](FinancialYear.md) |  | [required] |

### Return type

[**crate::models::FinancialYear**](financialYear.md)

### Authorization

[API token](../README.md#API token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

