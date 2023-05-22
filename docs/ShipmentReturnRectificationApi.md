# \ShipmentReturnRectificationApi

All URIs are relative to *https://www.weclapp.com/webapp/api/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**shipment_return_rectification_count_get**](ShipmentReturnRectificationApi.md#shipment_return_rectification_count_get) | **GET** /shipmentReturnRectification/count | 
[**shipment_return_rectification_get**](ShipmentReturnRectificationApi.md#shipment_return_rectification_get) | **GET** /shipmentReturnRectification | 
[**shipment_return_rectification_id_id_delete**](ShipmentReturnRectificationApi.md#shipment_return_rectification_id_id_delete) | **DELETE** /shipmentReturnRectification/id/{id} | 
[**shipment_return_rectification_id_id_get**](ShipmentReturnRectificationApi.md#shipment_return_rectification_id_id_get) | **GET** /shipmentReturnRectification/id/{id} | 
[**shipment_return_rectification_id_id_put**](ShipmentReturnRectificationApi.md#shipment_return_rectification_id_id_put) | **PUT** /shipmentReturnRectification/id/{id} | 
[**shipment_return_rectification_post**](ShipmentReturnRectificationApi.md#shipment_return_rectification_post) | **POST** /shipmentReturnRectification | 



## shipment_return_rectification_count_get

> crate::models::AccountingTransactionCountGet200Response shipment_return_rectification_count_get(page, page_size, sort)


count shipmentReturnRectification

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


## shipment_return_rectification_get

> crate::models::ShipmentReturnAssessmentGet200Response shipment_return_rectification_get(page, page_size, sort)


query shipmentReturnRectification

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


## shipment_return_rectification_id_id_delete

> shipment_return_rectification_id_id_delete(id)


delete a shipmentReturnRectification

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


## shipment_return_rectification_id_id_get

> crate::models::ShipmentReturnDescription shipment_return_rectification_id_id_get(id)


query shipmentReturnRectification

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


## shipment_return_rectification_id_id_put

> crate::models::ShipmentReturnDescription shipment_return_rectification_id_id_put(id, body)


update shipmentReturnRectification

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


## shipment_return_rectification_post

> crate::models::ShipmentReturnDescription shipment_return_rectification_post(body)


create a shipmentReturnRectification

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

