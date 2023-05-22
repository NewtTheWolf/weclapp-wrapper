# \VariantArticleVariantApi

All URIs are relative to *https://www.weclapp.com/webapp/api/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**variant_article_variant_count_get**](VariantArticleVariantApi.md#variant_article_variant_count_get) | **GET** /variantArticleVariant/count | 
[**variant_article_variant_get**](VariantArticleVariantApi.md#variant_article_variant_get) | **GET** /variantArticleVariant | 
[**variant_article_variant_id_id_get**](VariantArticleVariantApi.md#variant_article_variant_id_id_get) | **GET** /variantArticleVariant/id/{id} | 



## variant_article_variant_count_get

> crate::models::AccountingTransactionCountGet200Response variant_article_variant_count_get(page, page_size, sort)


count variantArticleVariant

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


## variant_article_variant_get

> crate::models::VariantArticleVariantGet200Response variant_article_variant_get(page, page_size, sort)


query variantArticleVariant

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page** | Option<**i32**> |  |  |
**page_size** | Option<**i32**> |  |  |
**sort** | Option<**String**> |  |  |

### Return type

[**crate::models::VariantArticleVariantGet200Response**](_variantArticleVariant_get_200_response.md)

### Authorization

[API token](../README.md#API token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## variant_article_variant_id_id_get

> crate::models::VariantArticleVariant variant_article_variant_id_id_get(id)


query variantArticleVariant

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

[**crate::models::VariantArticleVariant**](variantArticleVariant.md)

### Authorization

[API token](../README.md#API token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

