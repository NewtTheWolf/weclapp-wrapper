# \CustomerCategoryApi

All URIs are relative to *https://www.weclapp.com/webapp/api/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**customer_category_count_get**](CustomerCategoryApi.md#customer_category_count_get) | **GET** /customerCategory/count | 
[**customer_category_get**](CustomerCategoryApi.md#customer_category_get) | **GET** /customerCategory | 
[**customer_category_id_id_delete**](CustomerCategoryApi.md#customer_category_id_id_delete) | **DELETE** /customerCategory/id/{id} | 
[**customer_category_id_id_get**](CustomerCategoryApi.md#customer_category_id_id_get) | **GET** /customerCategory/id/{id} | 
[**customer_category_id_id_put**](CustomerCategoryApi.md#customer_category_id_id_put) | **PUT** /customerCategory/id/{id} | 
[**customer_category_post**](CustomerCategoryApi.md#customer_category_post) | **POST** /customerCategory | 



## customer_category_count_get

> crate::models::AccountingTransactionCountGet200Response customer_category_count_get(page, page_size, sort)


count customerCategory

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


## customer_category_get

> crate::models::ArticleAccountingCodeGet200Response customer_category_get(page, page_size, sort)


query customerCategory

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page** | Option<**i32**> |  |  |
**page_size** | Option<**i32**> |  |  |
**sort** | Option<**String**> |  |  |

### Return type

[**crate::models::ArticleAccountingCodeGet200Response**](_articleAccountingCode_get_200_response.md)

### Authorization

[API token](../README.md#API token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## customer_category_id_id_delete

> customer_category_id_id_delete(id)


delete a customerCategory

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


## customer_category_id_id_get

> crate::models::CustomValue customer_category_id_id_get(id)


query customerCategory

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

[**crate::models::CustomValue**](customValue.md)

### Authorization

[API token](../README.md#API token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## customer_category_id_id_put

> crate::models::CustomValue customer_category_id_id_put(id, body)


update customerCategory

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |
**body** | [**CustomValue**](CustomValue.md) |  | [required] |

### Return type

[**crate::models::CustomValue**](customValue.md)

### Authorization

[API token](../README.md#API token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## customer_category_post

> crate::models::CustomValue customer_category_post(body)


create a customerCategory

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**CustomValue**](CustomValue.md) |  | [required] |

### Return type

[**crate::models::CustomValue**](customValue.md)

### Authorization

[API token](../README.md#API token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

