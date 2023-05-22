# \ArticleApi

All URIs are relative to *https://www.weclapp.com/webapp/api/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**article_count_get**](ArticleApi.md#article_count_get) | **GET** /article/count | 
[**article_get**](ArticleApi.md#article_get) | **GET** /article | 
[**article_id_id_create_datasheet_pdf_post**](ArticleApi.md#article_id_id_create_datasheet_pdf_post) | **POST** /article/id/{id}/createDatasheetPdf | 
[**article_id_id_create_label_pdf_post**](ArticleApi.md#article_id_id_create_label_pdf_post) | **POST** /article/id/{id}/createLabelPdf | 
[**article_id_id_delete**](ArticleApi.md#article_id_id_delete) | **DELETE** /article/id/{id} | 
[**article_id_id_download_article_image_get**](ArticleApi.md#article_id_id_download_article_image_get) | **GET** /article/id/{id}/downloadArticleImage | 
[**article_id_id_download_main_article_image_get**](ArticleApi.md#article_id_id_download_main_article_image_get) | **GET** /article/id/{id}/downloadMainArticleImage | 
[**article_id_id_get**](ArticleApi.md#article_id_id_get) | **GET** /article/id/{id} | 
[**article_id_id_put**](ArticleApi.md#article_id_id_put) | **PUT** /article/id/{id} | 
[**article_id_id_upload_article_image_post**](ArticleApi.md#article_id_id_upload_article_image_post) | **POST** /article/id/{id}/uploadArticleImage | 
[**article_post**](ArticleApi.md#article_post) | **POST** /article | 



## article_count_get

> crate::models::AccountingTransactionCountGet200Response article_count_get(page, page_size, sort)


count article

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


## article_get

> crate::models::ArticleGet200Response article_get(page, page_size, sort)


query article

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page** | Option<**i32**> |  |  |
**page_size** | Option<**i32**> |  |  |
**sort** | Option<**String**> |  |  |

### Return type

[**crate::models::ArticleGet200Response**](_article_get_200_response.md)

### Authorization

[API token](../README.md#API token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## article_id_id_create_datasheet_pdf_post

> article_id_id_create_datasheet_pdf_post(id, body)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |
**body** | [**ArticleIdIdCreateDatasheetPdfPostRequest**](ArticleIdIdCreateDatasheetPdfPostRequest.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

[API token](../README.md#API token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: image/jpeg, image/png, application/pdf, */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## article_id_id_create_label_pdf_post

> article_id_id_create_label_pdf_post(id, body)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |
**body** | [**ArticleIdIdCreateLabelPdfPostRequest**](ArticleIdIdCreateLabelPdfPostRequest.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

[API token](../README.md#API token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: image/jpeg, image/png, application/pdf, */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## article_id_id_delete

> article_id_id_delete(id)


delete a article

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


## article_id_id_download_article_image_get

> article_id_id_download_article_image_get(id, article_image_id, preview, scale_width, scale_height)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |
**article_image_id** | **String** |  | [required] |
**preview** | Option<**bool**> |  |  |
**scale_width** | Option<**i32**> |  |  |
**scale_height** | Option<**i32**> |  |  |

### Return type

 (empty response body)

### Authorization

[API token](../README.md#API token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: image/jpeg, image/png, application/pdf, */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## article_id_id_download_main_article_image_get

> article_id_id_download_main_article_image_get(id, preview, scale_width, scale_height)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |
**preview** | Option<**bool**> |  |  |
**scale_width** | Option<**i32**> |  |  |
**scale_height** | Option<**i32**> |  |  |

### Return type

 (empty response body)

### Authorization

[API token](../README.md#API token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: image/jpeg, image/png, application/pdf, */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## article_id_id_get

> crate::models::Article article_id_id_get(id)


query article

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

[**crate::models::Article**](article.md)

### Authorization

[API token](../README.md#API token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## article_id_id_put

> crate::models::Article article_id_id_put(id, body)


update article

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |
**body** | [**Article**](Article.md) |  | [required] |

### Return type

[**crate::models::Article**](article.md)

### Authorization

[API token](../README.md#API token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## article_id_id_upload_article_image_post

> crate::models::ArticleIdIdUploadArticleImagePost200Response article_id_id_upload_article_image_post(id, name, main_image, article_image_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |
**name** | **String** |  | [required] |
**main_image** | Option<**bool**> |  |  |
**article_image_id** | Option<**String**> |  |  |

### Return type

[**crate::models::ArticleIdIdUploadArticleImagePost200Response**](_article_id__id__uploadArticleImage_post_200_response.md)

### Authorization

[API token](../README.md#API token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## article_post

> crate::models::Article article_post(body)


create a article

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**Article**](Article.md) |  | [required] |

### Return type

[**crate::models::Article**](article.md)

### Authorization

[API token](../README.md#API token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

