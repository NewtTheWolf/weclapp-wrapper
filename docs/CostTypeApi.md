# \CostTypeApi

All URIs are relative to *https://www.weclapp.com/webapp/api/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**cost_type_count_get**](CostTypeApi.md#cost_type_count_get) | **GET** /costType/count | 
[**cost_type_get**](CostTypeApi.md#cost_type_get) | **GET** /costType | 
[**cost_type_id_id_delete**](CostTypeApi.md#cost_type_id_id_delete) | **DELETE** /costType/id/{id} | 
[**cost_type_id_id_get**](CostTypeApi.md#cost_type_id_id_get) | **GET** /costType/id/{id} | 
[**cost_type_id_id_put**](CostTypeApi.md#cost_type_id_id_put) | **PUT** /costType/id/{id} | 
[**cost_type_post**](CostTypeApi.md#cost_type_post) | **POST** /costType | 



## cost_type_count_get

> crate::models::AccountingTransactionCountGet200Response cost_type_count_get(page, page_size, sort)


count costType

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


## cost_type_get

> crate::models::CostTypeGet200Response cost_type_get(page, page_size, sort)


query costType

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page** | Option<**i32**> |  |  |
**page_size** | Option<**i32**> |  |  |
**sort** | Option<**String**> |  |  |

### Return type

[**crate::models::CostTypeGet200Response**](_costType_get_200_response.md)

### Authorization

[API token](../README.md#API token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## cost_type_id_id_delete

> cost_type_id_id_delete(id)


delete a costType

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


## cost_type_id_id_get

> crate::models::CostType cost_type_id_id_get(id)


query costType

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

[**crate::models::CostType**](costType.md)

### Authorization

[API token](../README.md#API token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## cost_type_id_id_put

> crate::models::CostType cost_type_id_id_put(id, body)


update costType

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |
**body** | [**CostType**](CostType.md) |  | [required] |

### Return type

[**crate::models::CostType**](costType.md)

### Authorization

[API token](../README.md#API token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## cost_type_post

> crate::models::CostType cost_type_post(body)


create a costType

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**CostType**](CostType.md) |  | [required] |

### Return type

[**crate::models::CostType**](costType.md)

### Authorization

[API token](../README.md#API token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

