# \ProductionOrderApi

All URIs are relative to *https://www.weclapp.com/webapp/api/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**production_order_count_get**](ProductionOrderApi.md#production_order_count_get) | **GET** /productionOrder/count | 
[**production_order_fast_production_booking_post**](ProductionOrderApi.md#production_order_fast_production_booking_post) | **POST** /productionOrder/fastProductionBooking | 
[**production_order_get**](ProductionOrderApi.md#production_order_get) | **GET** /productionOrder | 
[**production_order_id_id_delete**](ProductionOrderApi.md#production_order_id_id_delete) | **DELETE** /productionOrder/id/{id} | 
[**production_order_id_id_download_latest_production_order_pdf_get**](ProductionOrderApi.md#production_order_id_id_download_latest_production_order_pdf_get) | **GET** /productionOrder/id/{id}/downloadLatestProductionOrderPdf | 
[**production_order_id_id_get**](ProductionOrderApi.md#production_order_id_id_get) | **GET** /productionOrder/id/{id} | 
[**production_order_id_id_put**](ProductionOrderApi.md#production_order_id_id_put) | **PUT** /productionOrder/id/{id} | 
[**production_order_post**](ProductionOrderApi.md#production_order_post) | **POST** /productionOrder | 



## production_order_count_get

> crate::models::AccountingTransactionCountGet200Response production_order_count_get(page, page_size, sort)


count productionOrder

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


## production_order_fast_production_booking_post

> crate::models::ProductionOrderFastProductionBookingPost200Response production_order_fast_production_booking_post(body)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**ProductionOrderFastProductionBookingPostRequest**](ProductionOrderFastProductionBookingPostRequest.md) |  | [required] |

### Return type

[**crate::models::ProductionOrderFastProductionBookingPost200Response**](_productionOrder_fastProductionBooking_post_200_response.md)

### Authorization

[API token](../README.md#API token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## production_order_get

> crate::models::ProductionOrderGet200Response production_order_get(page, page_size, sort)


query productionOrder

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page** | Option<**i32**> |  |  |
**page_size** | Option<**i32**> |  |  |
**sort** | Option<**String**> |  |  |

### Return type

[**crate::models::ProductionOrderGet200Response**](_productionOrder_get_200_response.md)

### Authorization

[API token](../README.md#API token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## production_order_id_id_delete

> production_order_id_id_delete(id)


delete a productionOrder

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


## production_order_id_id_download_latest_production_order_pdf_get

> production_order_id_id_download_latest_production_order_pdf_get(id)


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
- **Accept**: image/jpeg, image/png, application/pdf, */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## production_order_id_id_get

> crate::models::ProductionOrder production_order_id_id_get(id)


query productionOrder

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

[**crate::models::ProductionOrder**](productionOrder.md)

### Authorization

[API token](../README.md#API token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## production_order_id_id_put

> crate::models::ProductionOrder production_order_id_id_put(id, body)


update productionOrder

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |
**body** | [**ProductionOrder**](ProductionOrder.md) |  | [required] |

### Return type

[**crate::models::ProductionOrder**](productionOrder.md)

### Authorization

[API token](../README.md#API token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## production_order_post

> crate::models::ProductionOrder production_order_post(body)


create a productionOrder

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**ProductionOrder**](ProductionOrder.md) |  | [required] |

### Return type

[**crate::models::ProductionOrder**](productionOrder.md)

### Authorization

[API token](../README.md#API token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

