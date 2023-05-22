# \ArticleSupplySourceApi

All URIs are relative to *https://www.weclapp.com/webapp/api/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**article_supply_source_count_get**](ArticleSupplySourceApi.md#article_supply_source_count_get) | **GET** /articleSupplySource/count | 
[**article_supply_source_get**](ArticleSupplySourceApi.md#article_supply_source_get) | **GET** /articleSupplySource | 
[**article_supply_source_id_id_delete**](ArticleSupplySourceApi.md#article_supply_source_id_id_delete) | **DELETE** /articleSupplySource/id/{id} | 
[**article_supply_source_id_id_get**](ArticleSupplySourceApi.md#article_supply_source_id_id_get) | **GET** /articleSupplySource/id/{id} | 
[**article_supply_source_id_id_put**](ArticleSupplySourceApi.md#article_supply_source_id_id_put) | **PUT** /articleSupplySource/id/{id} | 
[**article_supply_source_post**](ArticleSupplySourceApi.md#article_supply_source_post) | **POST** /articleSupplySource | 



## article_supply_source_count_get

> crate::models::AccountingTransactionCountGet200Response article_supply_source_count_get(page, page_size, sort)


count articleSupplySource

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


## article_supply_source_get

> crate::models::ArticleSupplySourceGet200Response article_supply_source_get(page, page_size, sort)


query articleSupplySource

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page** | Option<**i32**> |  |  |
**page_size** | Option<**i32**> |  |  |
**sort** | Option<**String**> |  |  |

### Return type

[**crate::models::ArticleSupplySourceGet200Response**](_articleSupplySource_get_200_response.md)

### Authorization

[API token](../README.md#API token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## article_supply_source_id_id_delete

> article_supply_source_id_id_delete(id)


delete a articleSupplySource

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


## article_supply_source_id_id_get

> crate::models::ArticleSupplySource article_supply_source_id_id_get(id)


query articleSupplySource

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

[**crate::models::ArticleSupplySource**](articleSupplySource.md)

### Authorization

[API token](../README.md#API token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## article_supply_source_id_id_put

> crate::models::ArticleSupplySource article_supply_source_id_id_put(id, body)


update articleSupplySource

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |
**body** | [**ArticleSupplySource**](ArticleSupplySource.md) |  | [required] |

### Return type

[**crate::models::ArticleSupplySource**](articleSupplySource.md)

### Authorization

[API token](../README.md#API token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## article_supply_source_post

> crate::models::ArticleSupplySource article_supply_source_post(body)


create a articleSupplySource

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**ArticleSupplySource**](ArticleSupplySource.md) |  | [required] |

### Return type

[**crate::models::ArticleSupplySource**](articleSupplySource.md)

### Authorization

[API token](../README.md#API token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

