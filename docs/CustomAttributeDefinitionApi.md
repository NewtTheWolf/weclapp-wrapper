# \CustomAttributeDefinitionApi

All URIs are relative to *https://www.weclapp.com/webapp/api/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**custom_attribute_definition_count_get**](CustomAttributeDefinitionApi.md#custom_attribute_definition_count_get) | **GET** /customAttributeDefinition/count | 
[**custom_attribute_definition_get**](CustomAttributeDefinitionApi.md#custom_attribute_definition_get) | **GET** /customAttributeDefinition | 
[**custom_attribute_definition_id_id_get**](CustomAttributeDefinitionApi.md#custom_attribute_definition_id_id_get) | **GET** /customAttributeDefinition/id/{id} | 



## custom_attribute_definition_count_get

> crate::models::AccountingTransactionCountGet200Response custom_attribute_definition_count_get(page, page_size, sort)


count customAttributeDefinition

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


## custom_attribute_definition_get

> crate::models::CustomAttributeDefinitionGet200Response custom_attribute_definition_get(page, page_size, sort)


query customAttributeDefinition

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page** | Option<**i32**> |  |  |
**page_size** | Option<**i32**> |  |  |
**sort** | Option<**String**> |  |  |

### Return type

[**crate::models::CustomAttributeDefinitionGet200Response**](_customAttributeDefinition_get_200_response.md)

### Authorization

[API token](../README.md#API token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## custom_attribute_definition_id_id_get

> crate::models::CustomAttributeDefinition custom_attribute_definition_id_id_get(id)


query customAttributeDefinition

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

[**crate::models::CustomAttributeDefinition**](customAttributeDefinition.md)

### Authorization

[API token](../README.md#API token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

