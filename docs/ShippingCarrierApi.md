# \ShippingCarrierApi

All URIs are relative to *https://www.weclapp.com/webapp/api/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**shipping_carrier_count_get**](ShippingCarrierApi.md#shipping_carrier_count_get) | **GET** /shippingCarrier/count | 
[**shipping_carrier_get**](ShippingCarrierApi.md#shipping_carrier_get) | **GET** /shippingCarrier | 
[**shipping_carrier_id_id_delete**](ShippingCarrierApi.md#shipping_carrier_id_id_delete) | **DELETE** /shippingCarrier/id/{id} | 
[**shipping_carrier_id_id_get**](ShippingCarrierApi.md#shipping_carrier_id_id_get) | **GET** /shippingCarrier/id/{id} | 
[**shipping_carrier_id_id_put**](ShippingCarrierApi.md#shipping_carrier_id_id_put) | **PUT** /shippingCarrier/id/{id} | 
[**shipping_carrier_post**](ShippingCarrierApi.md#shipping_carrier_post) | **POST** /shippingCarrier | 



## shipping_carrier_count_get

> crate::models::AccountingTransactionCountGet200Response shipping_carrier_count_get(page, page_size, sort)


count shippingCarrier

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


## shipping_carrier_get

> crate::models::ShippingCarrierGet200Response shipping_carrier_get(page, page_size, sort)


query shippingCarrier

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page** | Option<**i32**> |  |  |
**page_size** | Option<**i32**> |  |  |
**sort** | Option<**String**> |  |  |

### Return type

[**crate::models::ShippingCarrierGet200Response**](_shippingCarrier_get_200_response.md)

### Authorization

[API token](../README.md#API token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## shipping_carrier_id_id_delete

> shipping_carrier_id_id_delete(id)


delete a shippingCarrier

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


## shipping_carrier_id_id_get

> crate::models::ShippingCarrier shipping_carrier_id_id_get(id)


query shippingCarrier

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

[**crate::models::ShippingCarrier**](shippingCarrier.md)

### Authorization

[API token](../README.md#API token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## shipping_carrier_id_id_put

> crate::models::ShippingCarrier shipping_carrier_id_id_put(id, body)


update shippingCarrier

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |
**body** | [**ShippingCarrier**](ShippingCarrier.md) |  | [required] |

### Return type

[**crate::models::ShippingCarrier**](shippingCarrier.md)

### Authorization

[API token](../README.md#API token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## shipping_carrier_post

> crate::models::ShippingCarrier shipping_carrier_post(body)


create a shippingCarrier

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**ShippingCarrier**](ShippingCarrier.md) |  | [required] |

### Return type

[**crate::models::ShippingCarrier**](shippingCarrier.md)

### Authorization

[API token](../README.md#API token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

