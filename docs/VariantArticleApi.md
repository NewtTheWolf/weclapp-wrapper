# \VariantArticleApi

All URIs are relative to *https://www.weclapp.com/webapp/api/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**variant_article_count_get**](VariantArticleApi.md#variant_article_count_get) | **GET** /variantArticle/count | 
[**variant_article_get**](VariantArticleApi.md#variant_article_get) | **GET** /variantArticle | 
[**variant_article_id_id_delete**](VariantArticleApi.md#variant_article_id_id_delete) | **DELETE** /variantArticle/id/{id} | 
[**variant_article_id_id_get**](VariantArticleApi.md#variant_article_id_id_get) | **GET** /variantArticle/id/{id} | 
[**variant_article_id_id_put**](VariantArticleApi.md#variant_article_id_id_put) | **PUT** /variantArticle/id/{id} | 
[**variant_article_post**](VariantArticleApi.md#variant_article_post) | **POST** /variantArticle | 



## variant_article_count_get

> crate::models::AccountingTransactionCountGet200Response variant_article_count_get(page, page_size, sort)


count variantArticle

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


## variant_article_get

> crate::models::VariantArticleGet200Response variant_article_get(page, page_size, sort)


query variantArticle

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page** | Option<**i32**> |  |  |
**page_size** | Option<**i32**> |  |  |
**sort** | Option<**String**> |  |  |

### Return type

[**crate::models::VariantArticleGet200Response**](_variantArticle_get_200_response.md)

### Authorization

[API token](../README.md#API token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## variant_article_id_id_delete

> variant_article_id_id_delete(id)


delete a variantArticle

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


## variant_article_id_id_get

> crate::models::VariantArticle variant_article_id_id_get(id)


query variantArticle

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

[**crate::models::VariantArticle**](variantArticle.md)

### Authorization

[API token](../README.md#API token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## variant_article_id_id_put

> crate::models::VariantArticle variant_article_id_id_put(id, body)


update variantArticle

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |
**body** | [**VariantArticle**](VariantArticle.md) |  | [required] |

### Return type

[**crate::models::VariantArticle**](variantArticle.md)

### Authorization

[API token](../README.md#API token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## variant_article_post

> crate::models::VariantArticle variant_article_post(body)


create a variantArticle

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**VariantArticle**](VariantArticle.md) |  | [required] |

### Return type

[**crate::models::VariantArticle**](variantArticle.md)

### Authorization

[API token](../README.md#API token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

