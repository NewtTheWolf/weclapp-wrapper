# \WarehouseStockMovementApi

All URIs are relative to *https://www.weclapp.com/webapp/api/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**warehouse_stock_movement_book_incoming_movement_post**](WarehouseStockMovementApi.md#warehouse_stock_movement_book_incoming_movement_post) | **POST** /warehouseStockMovement/bookIncomingMovement | 
[**warehouse_stock_movement_book_outgoing_movement_post**](WarehouseStockMovementApi.md#warehouse_stock_movement_book_outgoing_movement_post) | **POST** /warehouseStockMovement/bookOutgoingMovement | 
[**warehouse_stock_movement_book_transfer_movement_post**](WarehouseStockMovementApi.md#warehouse_stock_movement_book_transfer_movement_post) | **POST** /warehouseStockMovement/bookTransferMovement | 
[**warehouse_stock_movement_count_get**](WarehouseStockMovementApi.md#warehouse_stock_movement_count_get) | **GET** /warehouseStockMovement/count | 
[**warehouse_stock_movement_get**](WarehouseStockMovementApi.md#warehouse_stock_movement_get) | **GET** /warehouseStockMovement | 
[**warehouse_stock_movement_id_id_get**](WarehouseStockMovementApi.md#warehouse_stock_movement_id_id_get) | **GET** /warehouseStockMovement/id/{id} | 



## warehouse_stock_movement_book_incoming_movement_post

> crate::models::ArticleIdIdUploadArticleImagePost200Response warehouse_stock_movement_book_incoming_movement_post(body)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**WarehouseStockMovementBookIncomingMovementPostRequest**](WarehouseStockMovementBookIncomingMovementPostRequest.md) |  | [required] |

### Return type

[**crate::models::ArticleIdIdUploadArticleImagePost200Response**](_article_id__id__uploadArticleImage_post_200_response.md)

### Authorization

[API token](../README.md#API token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## warehouse_stock_movement_book_outgoing_movement_post

> crate::models::ArticleIdIdUploadArticleImagePost200Response warehouse_stock_movement_book_outgoing_movement_post(body)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**WarehouseStockMovementBookOutgoingMovementPostRequest**](WarehouseStockMovementBookOutgoingMovementPostRequest.md) |  | [required] |

### Return type

[**crate::models::ArticleIdIdUploadArticleImagePost200Response**](_article_id__id__uploadArticleImage_post_200_response.md)

### Authorization

[API token](../README.md#API token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## warehouse_stock_movement_book_transfer_movement_post

> crate::models::ArticleIdIdUploadArticleImagePost200Response warehouse_stock_movement_book_transfer_movement_post(body)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**WarehouseStockMovementBookTransferMovementPostRequest**](WarehouseStockMovementBookTransferMovementPostRequest.md) |  | [required] |

### Return type

[**crate::models::ArticleIdIdUploadArticleImagePost200Response**](_article_id__id__uploadArticleImage_post_200_response.md)

### Authorization

[API token](../README.md#API token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## warehouse_stock_movement_count_get

> crate::models::AccountingTransactionCountGet200Response warehouse_stock_movement_count_get(page, page_size, sort)


count warehouseStockMovement

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


## warehouse_stock_movement_get

> crate::models::WarehouseStockMovementGet200Response warehouse_stock_movement_get(page, page_size, sort)


query warehouseStockMovement

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page** | Option<**i32**> |  |  |
**page_size** | Option<**i32**> |  |  |
**sort** | Option<**String**> |  |  |

### Return type

[**crate::models::WarehouseStockMovementGet200Response**](_warehouseStockMovement_get_200_response.md)

### Authorization

[API token](../README.md#API token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## warehouse_stock_movement_id_id_get

> crate::models::WarehouseStockMovement warehouse_stock_movement_id_id_get(id)


query warehouseStockMovement

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

[**crate::models::WarehouseStockMovement**](warehouseStockMovement.md)

### Authorization

[API token](../README.md#API token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

