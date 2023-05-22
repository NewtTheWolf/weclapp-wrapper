# \ArticleCategoryApi

All URIs are relative to *https://www.weclapp.com/webapp/api/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**article_category_count_get**](ArticleCategoryApi.md#article_category_count_get) | **GET** /articleCategory/count | 
[**article_category_get**](ArticleCategoryApi.md#article_category_get) | **GET** /articleCategory | 
[**article_category_id_id_delete**](ArticleCategoryApi.md#article_category_id_id_delete) | **DELETE** /articleCategory/id/{id} | 
[**article_category_id_id_download_image_get**](ArticleCategoryApi.md#article_category_id_id_download_image_get) | **GET** /articleCategory/id/{id}/downloadImage | 
[**article_category_id_id_get**](ArticleCategoryApi.md#article_category_id_id_get) | **GET** /articleCategory/id/{id} | 
[**article_category_id_id_put**](ArticleCategoryApi.md#article_category_id_id_put) | **PUT** /articleCategory/id/{id} | 
[**article_category_id_id_upload_image_post**](ArticleCategoryApi.md#article_category_id_id_upload_image_post) | **POST** /articleCategory/id/{id}/uploadImage | 
[**article_category_post**](ArticleCategoryApi.md#article_category_post) | **POST** /articleCategory | 



## article_category_count_get

> crate::models::AccountingTransactionCountGet200Response article_category_count_get(page, page_size, sort)


count articleCategory

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


## article_category_get

> crate::models::ArticleCategoryGet200Response article_category_get(page, page_size, sort)


query articleCategory

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page** | Option<**i32**> |  |  |
**page_size** | Option<**i32**> |  |  |
**sort** | Option<**String**> |  |  |

### Return type

[**crate::models::ArticleCategoryGet200Response**](_articleCategory_get_200_response.md)

### Authorization

[API token](../README.md#API token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## article_category_id_id_delete

> article_category_id_id_delete(id)


delete a articleCategory

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


## article_category_id_id_download_image_get

> article_category_id_id_download_image_get(id, scale_width, scale_height, image_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |
**scale_width** | Option<**i32**> |  |  |
**scale_height** | Option<**i32**> |  |  |
**image_id** | Option<**String**> |  |  |

### Return type

 (empty response body)

### Authorization

[API token](../README.md#API token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: image/jpeg, image/png, application/pdf, */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## article_category_id_id_get

> crate::models::ArticleCategory article_category_id_id_get(id)


query articleCategory

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

[**crate::models::ArticleCategory**](articleCategory.md)

### Authorization

[API token](../README.md#API token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## article_category_id_id_put

> crate::models::ArticleCategory article_category_id_id_put(id, body)


update articleCategory

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |
**body** | [**ArticleCategory**](ArticleCategory.md) |  | [required] |

### Return type

[**crate::models::ArticleCategory**](articleCategory.md)

### Authorization

[API token](../README.md#API token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## article_category_id_id_upload_image_post

> crate::models::ArticleIdIdUploadArticleImagePost200Response article_category_id_id_upload_image_post(id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

[**crate::models::ArticleIdIdUploadArticleImagePost200Response**](_article_id__id__uploadArticleImage_post_200_response.md)

### Authorization

[API token](../README.md#API token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## article_category_post

> crate::models::ArticleCategory article_category_post(body)


create a articleCategory

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**ArticleCategory**](ArticleCategory.md) |  | [required] |

### Return type

[**crate::models::ArticleCategory**](articleCategory.md)

### Authorization

[API token](../README.md#API token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

