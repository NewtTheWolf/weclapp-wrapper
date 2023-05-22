# \SalesStageApi

All URIs are relative to *https://www.weclapp.com/webapp/api/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**sales_stage_count_get**](SalesStageApi.md#sales_stage_count_get) | **GET** /salesStage/count | 
[**sales_stage_get**](SalesStageApi.md#sales_stage_get) | **GET** /salesStage | 
[**sales_stage_id_id_delete**](SalesStageApi.md#sales_stage_id_id_delete) | **DELETE** /salesStage/id/{id} | 
[**sales_stage_id_id_get**](SalesStageApi.md#sales_stage_id_id_get) | **GET** /salesStage/id/{id} | 
[**sales_stage_id_id_put**](SalesStageApi.md#sales_stage_id_id_put) | **PUT** /salesStage/id/{id} | 
[**sales_stage_post**](SalesStageApi.md#sales_stage_post) | **POST** /salesStage | 



## sales_stage_count_get

> crate::models::AccountingTransactionCountGet200Response sales_stage_count_get(page, page_size, sort)


count salesStage

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


## sales_stage_get

> crate::models::SalesStageGet200Response sales_stage_get(page, page_size, sort)


query salesStage

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page** | Option<**i32**> |  |  |
**page_size** | Option<**i32**> |  |  |
**sort** | Option<**String**> |  |  |

### Return type

[**crate::models::SalesStageGet200Response**](_salesStage_get_200_response.md)

### Authorization

[API token](../README.md#API token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## sales_stage_id_id_delete

> sales_stage_id_id_delete(id)


delete a salesStage

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


## sales_stage_id_id_get

> crate::models::SalesStage sales_stage_id_id_get(id)


query salesStage

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

[**crate::models::SalesStage**](salesStage.md)

### Authorization

[API token](../README.md#API token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## sales_stage_id_id_put

> crate::models::SalesStage sales_stage_id_id_put(id, body)


update salesStage

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |
**body** | [**SalesStage**](SalesStage.md) |  | [required] |

### Return type

[**crate::models::SalesStage**](salesStage.md)

### Authorization

[API token](../README.md#API token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## sales_stage_post

> crate::models::SalesStage sales_stage_post(body)


create a salesStage

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**SalesStage**](SalesStage.md) |  | [required] |

### Return type

[**crate::models::SalesStage**](salesStage.md)

### Authorization

[API token](../README.md#API token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

