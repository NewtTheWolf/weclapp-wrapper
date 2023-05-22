# \LedgerAccountApi

All URIs are relative to *https://www.weclapp.com/webapp/api/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**ledger_account_count_get**](LedgerAccountApi.md#ledger_account_count_get) | **GET** /ledgerAccount/count | 
[**ledger_account_get**](LedgerAccountApi.md#ledger_account_get) | **GET** /ledgerAccount | 
[**ledger_account_id_id_delete**](LedgerAccountApi.md#ledger_account_id_id_delete) | **DELETE** /ledgerAccount/id/{id} | 
[**ledger_account_id_id_get**](LedgerAccountApi.md#ledger_account_id_id_get) | **GET** /ledgerAccount/id/{id} | 
[**ledger_account_id_id_put**](LedgerAccountApi.md#ledger_account_id_id_put) | **PUT** /ledgerAccount/id/{id} | 
[**ledger_account_post**](LedgerAccountApi.md#ledger_account_post) | **POST** /ledgerAccount | 



## ledger_account_count_get

> crate::models::AccountingTransactionCountGet200Response ledger_account_count_get(page, page_size, sort)


count ledgerAccount

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


## ledger_account_get

> crate::models::LedgerAccountGet200Response ledger_account_get(page, page_size, sort)


query ledgerAccount

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page** | Option<**i32**> |  |  |
**page_size** | Option<**i32**> |  |  |
**sort** | Option<**String**> |  |  |

### Return type

[**crate::models::LedgerAccountGet200Response**](_ledgerAccount_get_200_response.md)

### Authorization

[API token](../README.md#API token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## ledger_account_id_id_delete

> ledger_account_id_id_delete(id)


delete a ledgerAccount

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


## ledger_account_id_id_get

> crate::models::LedgerAccount ledger_account_id_id_get(id)


query ledgerAccount

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

[**crate::models::LedgerAccount**](ledgerAccount.md)

### Authorization

[API token](../README.md#API token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## ledger_account_id_id_put

> crate::models::LedgerAccount ledger_account_id_id_put(id, body)


update ledgerAccount

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |
**body** | [**LedgerAccount**](LedgerAccount.md) |  | [required] |

### Return type

[**crate::models::LedgerAccount**](ledgerAccount.md)

### Authorization

[API token](../README.md#API token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## ledger_account_post

> crate::models::LedgerAccount ledger_account_post(body)


create a ledgerAccount

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**LedgerAccount**](LedgerAccount.md) |  | [required] |

### Return type

[**crate::models::LedgerAccount**](ledgerAccount.md)

### Authorization

[API token](../README.md#API token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

