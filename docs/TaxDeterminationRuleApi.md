# \TaxDeterminationRuleApi

All URIs are relative to *https://www.weclapp.com/webapp/api/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**tax_determination_rule_count_get**](TaxDeterminationRuleApi.md#tax_determination_rule_count_get) | **GET** /taxDeterminationRule/count | 
[**tax_determination_rule_get**](TaxDeterminationRuleApi.md#tax_determination_rule_get) | **GET** /taxDeterminationRule | 
[**tax_determination_rule_id_id_delete**](TaxDeterminationRuleApi.md#tax_determination_rule_id_id_delete) | **DELETE** /taxDeterminationRule/id/{id} | 
[**tax_determination_rule_id_id_get**](TaxDeterminationRuleApi.md#tax_determination_rule_id_id_get) | **GET** /taxDeterminationRule/id/{id} | 
[**tax_determination_rule_id_id_put**](TaxDeterminationRuleApi.md#tax_determination_rule_id_id_put) | **PUT** /taxDeterminationRule/id/{id} | 
[**tax_determination_rule_post**](TaxDeterminationRuleApi.md#tax_determination_rule_post) | **POST** /taxDeterminationRule | 



## tax_determination_rule_count_get

> crate::models::AccountingTransactionCountGet200Response tax_determination_rule_count_get(page, page_size, sort)


count taxDeterminationRule

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


## tax_determination_rule_get

> crate::models::TaxDeterminationRuleGet200Response tax_determination_rule_get(page, page_size, sort)


query taxDeterminationRule

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page** | Option<**i32**> |  |  |
**page_size** | Option<**i32**> |  |  |
**sort** | Option<**String**> |  |  |

### Return type

[**crate::models::TaxDeterminationRuleGet200Response**](_taxDeterminationRule_get_200_response.md)

### Authorization

[API token](../README.md#API token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## tax_determination_rule_id_id_delete

> tax_determination_rule_id_id_delete(id)


delete a taxDeterminationRule

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


## tax_determination_rule_id_id_get

> crate::models::TaxDeterminationRule tax_determination_rule_id_id_get(id)


query taxDeterminationRule

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

[**crate::models::TaxDeterminationRule**](taxDeterminationRule.md)

### Authorization

[API token](../README.md#API token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## tax_determination_rule_id_id_put

> crate::models::TaxDeterminationRule tax_determination_rule_id_id_put(id, body)


update taxDeterminationRule

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |
**body** | [**TaxDeterminationRule**](TaxDeterminationRule.md) |  | [required] |

### Return type

[**crate::models::TaxDeterminationRule**](taxDeterminationRule.md)

### Authorization

[API token](../README.md#API token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## tax_determination_rule_post

> crate::models::TaxDeterminationRule tax_determination_rule_post(body)


create a taxDeterminationRule

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**TaxDeterminationRule**](TaxDeterminationRule.md) |  | [required] |

### Return type

[**crate::models::TaxDeterminationRule**](taxDeterminationRule.md)

### Authorization

[API token](../README.md#API token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

