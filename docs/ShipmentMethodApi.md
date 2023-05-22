# \ShipmentMethodApi

All URIs are relative to *https://www.weclapp.com/webapp/api/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**shipment_method_count_get**](ShipmentMethodApi.md#shipment_method_count_get) | **GET** /shipmentMethod/count | 
[**shipment_method_get**](ShipmentMethodApi.md#shipment_method_get) | **GET** /shipmentMethod | 
[**shipment_method_id_id_delete**](ShipmentMethodApi.md#shipment_method_id_id_delete) | **DELETE** /shipmentMethod/id/{id} | 
[**shipment_method_id_id_get**](ShipmentMethodApi.md#shipment_method_id_id_get) | **GET** /shipmentMethod/id/{id} | 
[**shipment_method_id_id_put**](ShipmentMethodApi.md#shipment_method_id_id_put) | **PUT** /shipmentMethod/id/{id} | 
[**shipment_method_post**](ShipmentMethodApi.md#shipment_method_post) | **POST** /shipmentMethod | 



## shipment_method_count_get

> crate::models::AccountingTransactionCountGet200Response shipment_method_count_get(page, page_size, sort)


count shipmentMethod

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


## shipment_method_get

> crate::models::ShipmentMethodGet200Response shipment_method_get(page, page_size, sort)


query shipmentMethod

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page** | Option<**i32**> |  |  |
**page_size** | Option<**i32**> |  |  |
**sort** | Option<**String**> |  |  |

### Return type

[**crate::models::ShipmentMethodGet200Response**](_shipmentMethod_get_200_response.md)

### Authorization

[API token](../README.md#API token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## shipment_method_id_id_delete

> shipment_method_id_id_delete(id)


delete a shipmentMethod

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


## shipment_method_id_id_get

> crate::models::ShipmentMethod shipment_method_id_id_get(id)


query shipmentMethod

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

[**crate::models::ShipmentMethod**](shipmentMethod.md)

### Authorization

[API token](../README.md#API token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## shipment_method_id_id_put

> crate::models::ShipmentMethod shipment_method_id_id_put(id, body)


update shipmentMethod

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |
**body** | [**ShipmentMethod**](ShipmentMethod.md) |  | [required] |

### Return type

[**crate::models::ShipmentMethod**](shipmentMethod.md)

### Authorization

[API token](../README.md#API token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## shipment_method_post

> crate::models::ShipmentMethod shipment_method_post(body)


create a shipmentMethod

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**ShipmentMethod**](ShipmentMethod.md) |  | [required] |

### Return type

[**crate::models::ShipmentMethod**](shipmentMethod.md)

### Authorization

[API token](../README.md#API token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

