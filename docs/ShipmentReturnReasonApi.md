# \ShipmentReturnReasonApi

All URIs are relative to *https://www.weclapp.com/webapp/api/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**shipment_return_reason_count_get**](ShipmentReturnReasonApi.md#shipment_return_reason_count_get) | **GET** /shipmentReturnReason/count | 
[**shipment_return_reason_get**](ShipmentReturnReasonApi.md#shipment_return_reason_get) | **GET** /shipmentReturnReason | 
[**shipment_return_reason_id_id_delete**](ShipmentReturnReasonApi.md#shipment_return_reason_id_id_delete) | **DELETE** /shipmentReturnReason/id/{id} | 
[**shipment_return_reason_id_id_get**](ShipmentReturnReasonApi.md#shipment_return_reason_id_id_get) | **GET** /shipmentReturnReason/id/{id} | 
[**shipment_return_reason_id_id_put**](ShipmentReturnReasonApi.md#shipment_return_reason_id_id_put) | **PUT** /shipmentReturnReason/id/{id} | 
[**shipment_return_reason_post**](ShipmentReturnReasonApi.md#shipment_return_reason_post) | **POST** /shipmentReturnReason | 



## shipment_return_reason_count_get

> crate::models::AccountingTransactionCountGet200Response shipment_return_reason_count_get(page, page_size, sort)


count shipmentReturnReason

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


## shipment_return_reason_get

> crate::models::ShipmentReturnAssessmentGet200Response shipment_return_reason_get(page, page_size, sort)


query shipmentReturnReason

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


## shipment_return_reason_id_id_delete

> shipment_return_reason_id_id_delete(id)


delete a shipmentReturnReason

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


## shipment_return_reason_id_id_get

> crate::models::ShipmentReturnDescription shipment_return_reason_id_id_get(id)


query shipmentReturnReason

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


## shipment_return_reason_id_id_put

> crate::models::ShipmentReturnDescription shipment_return_reason_id_id_put(id, body)


update shipmentReturnReason

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


## shipment_return_reason_post

> crate::models::ShipmentReturnDescription shipment_return_reason_post(body)


create a shipmentReturnReason

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

