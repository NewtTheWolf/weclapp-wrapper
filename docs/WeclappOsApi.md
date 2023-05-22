# \WeclappOsApi

All URIs are relative to *https://www.weclapp.com/webapp/api/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**weclapp_os_count_get**](WeclappOsApi.md#weclapp_os_count_get) | **GET** /weclappOs/count | 
[**weclapp_os_get**](WeclappOsApi.md#weclapp_os_get) | **GET** /weclappOs | 
[**weclapp_os_id_id_delete**](WeclappOsApi.md#weclapp_os_id_id_delete) | **DELETE** /weclappOs/id/{id} | 
[**weclapp_os_id_id_get**](WeclappOsApi.md#weclapp_os_id_id_get) | **GET** /weclappOs/id/{id} | 
[**weclapp_os_id_id_put**](WeclappOsApi.md#weclapp_os_id_id_put) | **PUT** /weclappOs/id/{id} | 
[**weclapp_os_post**](WeclappOsApi.md#weclapp_os_post) | **POST** /weclappOs | 



## weclapp_os_count_get

> crate::models::AccountingTransactionCountGet200Response weclapp_os_count_get(page, page_size, sort)


count weclappOs

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


## weclapp_os_get

> crate::models::WeclappOsGet200Response weclapp_os_get(page, page_size, sort)


query weclappOs

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page** | Option<**i32**> |  |  |
**page_size** | Option<**i32**> |  |  |
**sort** | Option<**String**> |  |  |

### Return type

[**crate::models::WeclappOsGet200Response**](_weclappOs_get_200_response.md)

### Authorization

[API token](../README.md#API token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## weclapp_os_id_id_delete

> weclapp_os_id_id_delete(id)


delete a weclappOs

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


## weclapp_os_id_id_get

> crate::models::WeclappOs weclapp_os_id_id_get(id)


query weclappOs

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

[**crate::models::WeclappOs**](weclappOs.md)

### Authorization

[API token](../README.md#API token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## weclapp_os_id_id_put

> crate::models::WeclappOs weclapp_os_id_id_put(id, body)


update weclappOs

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |
**body** | [**WeclappOs**](WeclappOs.md) |  | [required] |

### Return type

[**crate::models::WeclappOs**](weclappOs.md)

### Authorization

[API token](../README.md#API token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## weclapp_os_post

> crate::models::WeclappOs weclapp_os_post(body)


create a weclappOs

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**WeclappOs**](WeclappOs.md) |  | [required] |

### Return type

[**crate::models::WeclappOs**](weclappOs.md)

### Authorization

[API token](../README.md#API token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

