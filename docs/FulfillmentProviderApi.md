# \FulfillmentProviderApi

All URIs are relative to *https://www.weclapp.com/webapp/api/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**fulfillment_provider_count_get**](FulfillmentProviderApi.md#fulfillment_provider_count_get) | **GET** /fulfillmentProvider/count | 
[**fulfillment_provider_get**](FulfillmentProviderApi.md#fulfillment_provider_get) | **GET** /fulfillmentProvider | 
[**fulfillment_provider_id_id_get**](FulfillmentProviderApi.md#fulfillment_provider_id_id_get) | **GET** /fulfillmentProvider/id/{id} | 



## fulfillment_provider_count_get

> crate::models::AccountingTransactionCountGet200Response fulfillment_provider_count_get(page, page_size, sort)


count fulfillmentProvider

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


## fulfillment_provider_get

> crate::models::FulfillmentProviderGet200Response fulfillment_provider_get(page, page_size, sort)


query fulfillmentProvider

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page** | Option<**i32**> |  |  |
**page_size** | Option<**i32**> |  |  |
**sort** | Option<**String**> |  |  |

### Return type

[**crate::models::FulfillmentProviderGet200Response**](_fulfillmentProvider_get_200_response.md)

### Authorization

[API token](../README.md#API token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## fulfillment_provider_id_id_get

> crate::models::FulfillmentProvider fulfillment_provider_id_id_get(id)


query fulfillmentProvider

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

[**crate::models::FulfillmentProvider**](fulfillmentProvider.md)

### Authorization

[API token](../README.md#API token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

