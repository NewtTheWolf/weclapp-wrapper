# \PersonalAccountingCodeApi

All URIs are relative to *https://www.weclapp.com/webapp/api/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**personal_accounting_code_count_get**](PersonalAccountingCodeApi.md#personal_accounting_code_count_get) | **GET** /personalAccountingCode/count | 
[**personal_accounting_code_get**](PersonalAccountingCodeApi.md#personal_accounting_code_get) | **GET** /personalAccountingCode | 
[**personal_accounting_code_id_id_delete**](PersonalAccountingCodeApi.md#personal_accounting_code_id_id_delete) | **DELETE** /personalAccountingCode/id/{id} | 
[**personal_accounting_code_id_id_get**](PersonalAccountingCodeApi.md#personal_accounting_code_id_id_get) | **GET** /personalAccountingCode/id/{id} | 
[**personal_accounting_code_id_id_put**](PersonalAccountingCodeApi.md#personal_accounting_code_id_id_put) | **PUT** /personalAccountingCode/id/{id} | 
[**personal_accounting_code_post**](PersonalAccountingCodeApi.md#personal_accounting_code_post) | **POST** /personalAccountingCode | 



## personal_accounting_code_count_get

> crate::models::AccountingTransactionCountGet200Response personal_accounting_code_count_get(page, page_size, sort)


count personalAccountingCode

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


## personal_accounting_code_get

> crate::models::ArticleAccountingCodeGet200Response personal_accounting_code_get(page, page_size, sort)


query personalAccountingCode

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


## personal_accounting_code_id_id_delete

> personal_accounting_code_id_id_delete(id)


delete a personalAccountingCode

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


## personal_accounting_code_id_id_get

> crate::models::CustomValue personal_accounting_code_id_id_get(id)


query personalAccountingCode

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


## personal_accounting_code_id_id_put

> crate::models::CustomValue personal_accounting_code_id_id_put(id, body)


update personalAccountingCode

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


## personal_accounting_code_post

> crate::models::CustomValue personal_accounting_code_post(body)


create a personalAccountingCode

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

