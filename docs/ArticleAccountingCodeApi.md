# \ArticleAccountingCodeApi

All URIs are relative to *https://www.weclapp.com/webapp/api/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**article_accounting_code_count_get**](ArticleAccountingCodeApi.md#article_accounting_code_count_get) | **GET** /articleAccountingCode/count | 
[**article_accounting_code_get**](ArticleAccountingCodeApi.md#article_accounting_code_get) | **GET** /articleAccountingCode | 
[**article_accounting_code_id_id_delete**](ArticleAccountingCodeApi.md#article_accounting_code_id_id_delete) | **DELETE** /articleAccountingCode/id/{id} | 
[**article_accounting_code_id_id_get**](ArticleAccountingCodeApi.md#article_accounting_code_id_id_get) | **GET** /articleAccountingCode/id/{id} | 
[**article_accounting_code_id_id_put**](ArticleAccountingCodeApi.md#article_accounting_code_id_id_put) | **PUT** /articleAccountingCode/id/{id} | 
[**article_accounting_code_post**](ArticleAccountingCodeApi.md#article_accounting_code_post) | **POST** /articleAccountingCode | 



## article_accounting_code_count_get

> crate::models::AccountingTransactionCountGet200Response article_accounting_code_count_get(page, page_size, sort)


count articleAccountingCode

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


## article_accounting_code_get

> crate::models::ArticleAccountingCodeGet200Response article_accounting_code_get(page, page_size, sort)


query articleAccountingCode

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page** | Option<**i32**> |  |  |
**page_size** | Option<**i32**> |  |  |
**sort** | Option<**String**> |  |  |

### Return type

[**crate::models::ArticleAccountingCodeGet200Response**](_articleAccountingCode_get_200_response.md)

### Authorization

[API token](../README.md#API token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## article_accounting_code_id_id_delete

> article_accounting_code_id_id_delete(id)


delete a articleAccountingCode

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


## article_accounting_code_id_id_get

> crate::models::CustomValue article_accounting_code_id_id_get(id)


query articleAccountingCode

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

[**crate::models::CustomValue**](customValue.md)

### Authorization

[API token](../README.md#API token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## article_accounting_code_id_id_put

> crate::models::CustomValue article_accounting_code_id_id_put(id, body)


update articleAccountingCode

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |
**body** | [**CustomValue**](CustomValue.md) |  | [required] |

### Return type

[**crate::models::CustomValue**](customValue.md)

### Authorization

[API token](../README.md#API token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## article_accounting_code_post

> crate::models::CustomValue article_accounting_code_post(body)


create a articleAccountingCode

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**CustomValue**](CustomValue.md) |  | [required] |

### Return type

[**crate::models::CustomValue**](customValue.md)

### Authorization

[API token](../README.md#API token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

