# \SerialNumberApi

All URIs are relative to *https://www.weclapp.com/webapp/api/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**serial_number_count_get**](SerialNumberApi.md#serial_number_count_get) | **GET** /serialNumber/count | 
[**serial_number_get**](SerialNumberApi.md#serial_number_get) | **GET** /serialNumber | 
[**serial_number_id_id_get**](SerialNumberApi.md#serial_number_id_id_get) | **GET** /serialNumber/id/{id} | 
[**serial_number_id_id_put**](SerialNumberApi.md#serial_number_id_id_put) | **PUT** /serialNumber/id/{id} | 



## serial_number_count_get

> crate::models::AccountingTransactionCountGet200Response serial_number_count_get(page, page_size, sort)


count serialNumber

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


## serial_number_get

> crate::models::SerialNumberGet200Response serial_number_get(page, page_size, sort)


query serialNumber

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page** | Option<**i32**> |  |  |
**page_size** | Option<**i32**> |  |  |
**sort** | Option<**String**> |  |  |

### Return type

[**crate::models::SerialNumberGet200Response**](_serialNumber_get_200_response.md)

### Authorization

[API token](../README.md#API token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## serial_number_id_id_get

> crate::models::SerialNumber serial_number_id_id_get(id)


query serialNumber

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

[**crate::models::SerialNumber**](serialNumber.md)

### Authorization

[API token](../README.md#API token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## serial_number_id_id_put

> crate::models::SerialNumber serial_number_id_id_put(id, body)


update serialNumber

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |
**body** | [**SerialNumber**](SerialNumber.md) |  | [required] |

### Return type

[**crate::models::SerialNumber**](serialNumber.md)

### Authorization

[API token](../README.md#API token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

