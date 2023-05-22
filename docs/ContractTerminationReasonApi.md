# \ContractTerminationReasonApi

All URIs are relative to *https://www.weclapp.com/webapp/api/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**contract_termination_reason_count_get**](ContractTerminationReasonApi.md#contract_termination_reason_count_get) | **GET** /contractTerminationReason/count | 
[**contract_termination_reason_get**](ContractTerminationReasonApi.md#contract_termination_reason_get) | **GET** /contractTerminationReason | 
[**contract_termination_reason_id_id_delete**](ContractTerminationReasonApi.md#contract_termination_reason_id_id_delete) | **DELETE** /contractTerminationReason/id/{id} | 
[**contract_termination_reason_id_id_get**](ContractTerminationReasonApi.md#contract_termination_reason_id_id_get) | **GET** /contractTerminationReason/id/{id} | 
[**contract_termination_reason_id_id_put**](ContractTerminationReasonApi.md#contract_termination_reason_id_id_put) | **PUT** /contractTerminationReason/id/{id} | 
[**contract_termination_reason_post**](ContractTerminationReasonApi.md#contract_termination_reason_post) | **POST** /contractTerminationReason | 



## contract_termination_reason_count_get

> crate::models::AccountingTransactionCountGet200Response contract_termination_reason_count_get(page, page_size, sort)


count contractTerminationReason

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


## contract_termination_reason_get

> crate::models::ArticleAccountingCodeGet200Response contract_termination_reason_get(page, page_size, sort)


query contractTerminationReason

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


## contract_termination_reason_id_id_delete

> contract_termination_reason_id_id_delete(id)


delete a contractTerminationReason

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


## contract_termination_reason_id_id_get

> crate::models::CustomValue contract_termination_reason_id_id_get(id)


query contractTerminationReason

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


## contract_termination_reason_id_id_put

> crate::models::CustomValue contract_termination_reason_id_id_put(id, body)


update contractTerminationReason

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


## contract_termination_reason_post

> crate::models::CustomValue contract_termination_reason_post(body)


create a contractTerminationReason

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

