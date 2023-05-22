# \ContractBillingGroupApi

All URIs are relative to *https://www.weclapp.com/webapp/api/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**contract_billing_group_count_get**](ContractBillingGroupApi.md#contract_billing_group_count_get) | **GET** /contractBillingGroup/count | 
[**contract_billing_group_get**](ContractBillingGroupApi.md#contract_billing_group_get) | **GET** /contractBillingGroup | 
[**contract_billing_group_id_id_delete**](ContractBillingGroupApi.md#contract_billing_group_id_id_delete) | **DELETE** /contractBillingGroup/id/{id} | 
[**contract_billing_group_id_id_get**](ContractBillingGroupApi.md#contract_billing_group_id_id_get) | **GET** /contractBillingGroup/id/{id} | 
[**contract_billing_group_id_id_put**](ContractBillingGroupApi.md#contract_billing_group_id_id_put) | **PUT** /contractBillingGroup/id/{id} | 
[**contract_billing_group_post**](ContractBillingGroupApi.md#contract_billing_group_post) | **POST** /contractBillingGroup | 



## contract_billing_group_count_get

> crate::models::AccountingTransactionCountGet200Response contract_billing_group_count_get(page, page_size, sort)


count contractBillingGroup

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


## contract_billing_group_get

> crate::models::ArticleAccountingCodeGet200Response contract_billing_group_get(page, page_size, sort)


query contractBillingGroup

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


## contract_billing_group_id_id_delete

> contract_billing_group_id_id_delete(id)


delete a contractBillingGroup

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


## contract_billing_group_id_id_get

> crate::models::CustomValue contract_billing_group_id_id_get(id)


query contractBillingGroup

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


## contract_billing_group_id_id_put

> crate::models::CustomValue contract_billing_group_id_id_put(id, body)


update contractBillingGroup

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


## contract_billing_group_post

> crate::models::CustomValue contract_billing_group_post(body)


create a contractBillingGroup

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

