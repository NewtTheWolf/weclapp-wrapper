# \VariantArticleAttributeApi

All URIs are relative to *https://www.weclapp.com/webapp/api/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**variant_article_attribute_count_get**](VariantArticleAttributeApi.md#variant_article_attribute_count_get) | **GET** /variantArticleAttribute/count | 
[**variant_article_attribute_get**](VariantArticleAttributeApi.md#variant_article_attribute_get) | **GET** /variantArticleAttribute | 
[**variant_article_attribute_id_id_delete**](VariantArticleAttributeApi.md#variant_article_attribute_id_id_delete) | **DELETE** /variantArticleAttribute/id/{id} | 
[**variant_article_attribute_id_id_get**](VariantArticleAttributeApi.md#variant_article_attribute_id_id_get) | **GET** /variantArticleAttribute/id/{id} | 
[**variant_article_attribute_id_id_put**](VariantArticleAttributeApi.md#variant_article_attribute_id_id_put) | **PUT** /variantArticleAttribute/id/{id} | 
[**variant_article_attribute_post**](VariantArticleAttributeApi.md#variant_article_attribute_post) | **POST** /variantArticleAttribute | 



## variant_article_attribute_count_get

> crate::models::AccountingTransactionCountGet200Response variant_article_attribute_count_get(page, page_size, sort)


count variantArticleAttribute

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


## variant_article_attribute_get

> crate::models::VariantArticleAttributeGet200Response variant_article_attribute_get(page, page_size, sort)


query variantArticleAttribute

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page** | Option<**i32**> |  |  |
**page_size** | Option<**i32**> |  |  |
**sort** | Option<**String**> |  |  |

### Return type

[**crate::models::VariantArticleAttributeGet200Response**](_variantArticleAttribute_get_200_response.md)

### Authorization

[API token](../README.md#API token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## variant_article_attribute_id_id_delete

> variant_article_attribute_id_id_delete(id)


delete a variantArticleAttribute

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


## variant_article_attribute_id_id_get

> crate::models::VariantArticleAttribute variant_article_attribute_id_id_get(id)


query variantArticleAttribute

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

[**crate::models::VariantArticleAttribute**](variantArticleAttribute.md)

### Authorization

[API token](../README.md#API token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## variant_article_attribute_id_id_put

> crate::models::VariantArticleAttribute variant_article_attribute_id_id_put(id, body)


update variantArticleAttribute

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |
**body** | [**VariantArticleAttribute**](VariantArticleAttribute.md) |  | [required] |

### Return type

[**crate::models::VariantArticleAttribute**](variantArticleAttribute.md)

### Authorization

[API token](../README.md#API token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## variant_article_attribute_post

> crate::models::VariantArticleAttribute variant_article_attribute_post(body)


create a variantArticleAttribute

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**VariantArticleAttribute**](VariantArticleAttribute.md) |  | [required] |

### Return type

[**crate::models::VariantArticleAttribute**](variantArticleAttribute.md)

### Authorization

[API token](../README.md#API token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

