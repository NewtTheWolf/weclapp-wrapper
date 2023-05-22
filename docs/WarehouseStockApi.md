# \WarehouseStockApi

All URIs are relative to *https://www.weclapp.com/webapp/api/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**warehouse_stock_count_get**](WarehouseStockApi.md#warehouse_stock_count_get) | **GET** /warehouseStock/count | 
[**warehouse_stock_get**](WarehouseStockApi.md#warehouse_stock_get) | **GET** /warehouseStock | 
[**warehouse_stock_id_id_get**](WarehouseStockApi.md#warehouse_stock_id_id_get) | **GET** /warehouseStock/id/{id} | 



## warehouse_stock_count_get

> crate::models::AccountingTransactionCountGet200Response warehouse_stock_count_get(page, page_size, sort)


count warehouseStock

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


## warehouse_stock_get

> crate::models::WarehouseStockGet200Response warehouse_stock_get(page, page_size, sort)


query warehouseStock

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page** | Option<**i32**> |  |  |
**page_size** | Option<**i32**> |  |  |
**sort** | Option<**String**> |  |  |

### Return type

[**crate::models::WarehouseStockGet200Response**](_warehouseStock_get_200_response.md)

### Authorization

[API token](../README.md#API token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## warehouse_stock_id_id_get

> crate::models::WarehouseStock warehouse_stock_id_id_get(id)


query warehouseStock

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

[**crate::models::WarehouseStock**](warehouseStock.md)

### Authorization

[API token](../README.md#API token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

