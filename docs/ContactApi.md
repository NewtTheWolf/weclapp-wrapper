# \ContactApi

All URIs are relative to *https://www.weclapp.com/webapp/api/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**contact_count_get**](ContactApi.md#contact_count_get) | **GET** /contact/count | 
[**contact_get**](ContactApi.md#contact_get) | **GET** /contact | 
[**contact_id_id_delete**](ContactApi.md#contact_id_id_delete) | **DELETE** /contact/id/{id} | 
[**contact_id_id_download_image_get**](ContactApi.md#contact_id_id_download_image_get) | **GET** /contact/id/{id}/downloadImage | 
[**contact_id_id_get**](ContactApi.md#contact_id_id_get) | **GET** /contact/id/{id} | 
[**contact_id_id_put**](ContactApi.md#contact_id_id_put) | **PUT** /contact/id/{id} | 
[**contact_id_id_upload_image_post**](ContactApi.md#contact_id_id_upload_image_post) | **POST** /contact/id/{id}/uploadImage | 
[**contact_post**](ContactApi.md#contact_post) | **POST** /contact | 



## contact_count_get

> crate::models::AccountingTransactionCountGet200Response contact_count_get(page, page_size, sort)


count contact

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


## contact_get

> crate::models::ContactGet200Response contact_get(page, page_size, sort)


query contact

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page** | Option<**i32**> |  |  |
**page_size** | Option<**i32**> |  |  |
**sort** | Option<**String**> |  |  |

### Return type

[**crate::models::ContactGet200Response**](_contact_get_200_response.md)

### Authorization

[API token](../README.md#API token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## contact_id_id_delete

> contact_id_id_delete(id)


delete a contact

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


## contact_id_id_download_image_get

> contact_id_id_download_image_get(id, scale_width, scale_height)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |
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


## contact_id_id_get

> crate::models::Contact contact_id_id_get(id)


query contact

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

[**crate::models::Contact**](contact.md)

### Authorization

[API token](../README.md#API token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## contact_id_id_put

> crate::models::Contact contact_id_id_put(id, body)


update contact

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |
**body** | [**Contact**](Contact.md) |  | [required] |

### Return type

[**crate::models::Contact**](contact.md)

### Authorization

[API token](../README.md#API token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## contact_id_id_upload_image_post

> crate::models::ArticleIdIdUploadArticleImagePost200Response contact_id_id_upload_image_post(id)


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


## contact_post

> crate::models::Contact contact_post(body)


create a contact

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**Contact**](Contact.md) |  | [required] |

### Return type

[**crate::models::Contact**](contact.md)

### Authorization

[API token](../README.md#API token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

