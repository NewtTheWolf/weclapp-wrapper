# \BankAccountApi

All URIs are relative to *https://www.weclapp.com/webapp/api/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**bank_account_count_get**](BankAccountApi.md#bank_account_count_get) | **GET** /bankAccount/count | 
[**bank_account_get**](BankAccountApi.md#bank_account_get) | **GET** /bankAccount | 
[**bank_account_id_id_delete**](BankAccountApi.md#bank_account_id_id_delete) | **DELETE** /bankAccount/id/{id} | 
[**bank_account_id_id_get**](BankAccountApi.md#bank_account_id_id_get) | **GET** /bankAccount/id/{id} | 
[**bank_account_id_id_put**](BankAccountApi.md#bank_account_id_id_put) | **PUT** /bankAccount/id/{id} | 
[**bank_account_post**](BankAccountApi.md#bank_account_post) | **POST** /bankAccount | 



## bank_account_count_get

> crate::models::AccountingTransactionCountGet200Response bank_account_count_get(page, page_size, sort)


count bankAccount

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


## bank_account_get

> crate::models::BankAccountGet200Response bank_account_get(page, page_size, sort)


query bankAccount

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page** | Option<**i32**> |  |  |
**page_size** | Option<**i32**> |  |  |
**sort** | Option<**String**> |  |  |

### Return type

[**crate::models::BankAccountGet200Response**](_bankAccount_get_200_response.md)

### Authorization

[API token](../README.md#API token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## bank_account_id_id_delete

> bank_account_id_id_delete(id)


delete a bankAccount

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


## bank_account_id_id_get

> crate::models::BankAccount bank_account_id_id_get(id)


query bankAccount

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

[**crate::models::BankAccount**](bankAccount.md)

### Authorization

[API token](../README.md#API token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## bank_account_id_id_put

> crate::models::BankAccount bank_account_id_id_put(id, body)


update bankAccount

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |
**body** | [**BankAccount**](BankAccount.md) |  | [required] |

### Return type

[**crate::models::BankAccount**](bankAccount.md)

### Authorization

[API token](../README.md#API token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## bank_account_post

> crate::models::BankAccount bank_account_post(body)


create a bankAccount

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**BankAccount**](BankAccount.md) |  | [required] |

### Return type

[**crate::models::BankAccount**](bankAccount.md)

### Authorization

[API token](../README.md#API token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

