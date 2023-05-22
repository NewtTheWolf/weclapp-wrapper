# \CashAccountApi

All URIs are relative to *https://www.weclapp.com/webapp/api/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**cash_account_count_get**](CashAccountApi.md#cash_account_count_get) | **GET** /cashAccount/count | 
[**cash_account_get**](CashAccountApi.md#cash_account_get) | **GET** /cashAccount | 
[**cash_account_id_id_get**](CashAccountApi.md#cash_account_id_id_get) | **GET** /cashAccount/id/{id} | 
[**cash_account_id_id_put**](CashAccountApi.md#cash_account_id_id_put) | **PUT** /cashAccount/id/{id} | 
[**cash_account_post**](CashAccountApi.md#cash_account_post) | **POST** /cashAccount | 



## cash_account_count_get

> crate::models::AccountingTransactionCountGet200Response cash_account_count_get(page, page_size, sort)


count cashAccount

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


## cash_account_get

> crate::models::CashAccountGet200Response cash_account_get(page, page_size, sort)


query cashAccount

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page** | Option<**i32**> |  |  |
**page_size** | Option<**i32**> |  |  |
**sort** | Option<**String**> |  |  |

### Return type

[**crate::models::CashAccountGet200Response**](_cashAccount_get_200_response.md)

### Authorization

[API token](../README.md#API token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## cash_account_id_id_get

> crate::models::CashAccount cash_account_id_id_get(id)


query cashAccount

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

[**crate::models::CashAccount**](cashAccount.md)

### Authorization

[API token](../README.md#API token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## cash_account_id_id_put

> crate::models::CashAccount cash_account_id_id_put(id, body)


update cashAccount

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |
**body** | [**CashAccount**](CashAccount.md) |  | [required] |

### Return type

[**crate::models::CashAccount**](cashAccount.md)

### Authorization

[API token](../README.md#API token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## cash_account_post

> crate::models::CashAccount cash_account_post(body)


create a cashAccount

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**CashAccount**](CashAccount.md) |  | [required] |

### Return type

[**crate::models::CashAccount**](cashAccount.md)

### Authorization

[API token](../README.md#API token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

