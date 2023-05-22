# \ContractAuthorizationUnitApi

All URIs are relative to *https://www.weclapp.com/webapp/api/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**contract_authorization_unit_count_get**](ContractAuthorizationUnitApi.md#contract_authorization_unit_count_get) | **GET** /contractAuthorizationUnit/count | 
[**contract_authorization_unit_get**](ContractAuthorizationUnitApi.md#contract_authorization_unit_get) | **GET** /contractAuthorizationUnit | 
[**contract_authorization_unit_id_id_get**](ContractAuthorizationUnitApi.md#contract_authorization_unit_id_id_get) | **GET** /contractAuthorizationUnit/id/{id} | 



## contract_authorization_unit_count_get

> crate::models::AccountingTransactionCountGet200Response contract_authorization_unit_count_get(page, page_size, sort)


count contractAuthorizationUnit

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


## contract_authorization_unit_get

> crate::models::ContractAuthorizationUnitGet200Response contract_authorization_unit_get(page, page_size, sort)


query contractAuthorizationUnit

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page** | Option<**i32**> |  |  |
**page_size** | Option<**i32**> |  |  |
**sort** | Option<**String**> |  |  |

### Return type

[**crate::models::ContractAuthorizationUnitGet200Response**](_contractAuthorizationUnit_get_200_response.md)

### Authorization

[API token](../README.md#API token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## contract_authorization_unit_id_id_get

> crate::models::ContractAuthorizationUnit contract_authorization_unit_id_id_get(id)


query contractAuthorizationUnit

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

[**crate::models::ContractAuthorizationUnit**](contractAuthorizationUnit.md)

### Authorization

[API token](../README.md#API token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

