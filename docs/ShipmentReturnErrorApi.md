# \ShipmentReturnErrorApi

All URIs are relative to *https://www.weclapp.com/webapp/api/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**shipment_return_error_count_get**](ShipmentReturnErrorApi.md#shipment_return_error_count_get) | **GET** /shipmentReturnError/count | 
[**shipment_return_error_get**](ShipmentReturnErrorApi.md#shipment_return_error_get) | **GET** /shipmentReturnError | 
[**shipment_return_error_id_id_delete**](ShipmentReturnErrorApi.md#shipment_return_error_id_id_delete) | **DELETE** /shipmentReturnError/id/{id} | 
[**shipment_return_error_id_id_get**](ShipmentReturnErrorApi.md#shipment_return_error_id_id_get) | **GET** /shipmentReturnError/id/{id} | 
[**shipment_return_error_id_id_put**](ShipmentReturnErrorApi.md#shipment_return_error_id_id_put) | **PUT** /shipmentReturnError/id/{id} | 
[**shipment_return_error_post**](ShipmentReturnErrorApi.md#shipment_return_error_post) | **POST** /shipmentReturnError | 



## shipment_return_error_count_get

> crate::models::AccountingTransactionCountGet200Response shipment_return_error_count_get(page, page_size, sort)


count shipmentReturnError

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


## shipment_return_error_get

> crate::models::ShipmentReturnAssessmentGet200Response shipment_return_error_get(page, page_size, sort)


query shipmentReturnError

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page** | Option<**i32**> |  |  |
**page_size** | Option<**i32**> |  |  |
**sort** | Option<**String**> |  |  |

### Return type

[**crate::models::ShipmentReturnAssessmentGet200Response**](_shipmentReturnAssessment_get_200_response.md)

### Authorization

[API token](../README.md#API token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## shipment_return_error_id_id_delete

> shipment_return_error_id_id_delete(id)


delete a shipmentReturnError

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


## shipment_return_error_id_id_get

> crate::models::ShipmentReturnDescription shipment_return_error_id_id_get(id)


query shipmentReturnError

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

[**crate::models::ShipmentReturnDescription**](shipmentReturnDescription.md)

### Authorization

[API token](../README.md#API token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## shipment_return_error_id_id_put

> crate::models::ShipmentReturnDescription shipment_return_error_id_id_put(id, body)


update shipmentReturnError

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |
**body** | [**ShipmentReturnDescription**](ShipmentReturnDescription.md) |  | [required] |

### Return type

[**crate::models::ShipmentReturnDescription**](shipmentReturnDescription.md)

### Authorization

[API token](../README.md#API token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## shipment_return_error_post

> crate::models::ShipmentReturnDescription shipment_return_error_post(body)


create a shipmentReturnError

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**ShipmentReturnDescription**](ShipmentReturnDescription.md) |  | [required] |

### Return type

[**crate::models::ShipmentReturnDescription**](shipmentReturnDescription.md)

### Authorization

[API token](../README.md#API token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

