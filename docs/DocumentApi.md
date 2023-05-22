# \DocumentApi

All URIs are relative to *https://www.weclapp.com/webapp/api/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**document_count_get**](DocumentApi.md#document_count_get) | **GET** /document/count | 
[**document_get**](DocumentApi.md#document_get) | **GET** /document | 
[**document_id_id_delete**](DocumentApi.md#document_id_id_delete) | **DELETE** /document/id/{id} | 
[**document_id_id_download_document_version_get**](DocumentApi.md#document_id_id_download_document_version_get) | **GET** /document/id/{id}/downloadDocumentVersion | 
[**document_id_id_download_get**](DocumentApi.md#document_id_id_download_get) | **GET** /document/id/{id}/download | 
[**document_id_id_get**](DocumentApi.md#document_id_id_get) | **GET** /document/id/{id} | 
[**document_id_id_put**](DocumentApi.md#document_id_id_put) | **PUT** /document/id/{id} | 
[**document_id_id_upload_post**](DocumentApi.md#document_id_id_upload_post) | **POST** /document/id/{id}/upload | 
[**document_upload_post**](DocumentApi.md#document_upload_post) | **POST** /document/upload | 



## document_count_get

> crate::models::AccountingTransactionCountGet200Response document_count_get(entity_id, entity_name, page, page_size, sort)


count document

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**entity_id** | **String** |  | [required] |
**entity_name** | **String** |  | [required] |
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


## document_get

> crate::models::DocumentGet200Response document_get(entity_id, entity_name, page, page_size, sort)


query document

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**entity_id** | **String** |  | [required] |
**entity_name** | **String** |  | [required] |
**page** | Option<**i32**> |  |  |
**page_size** | Option<**i32**> |  |  |
**sort** | Option<**String**> |  |  |

### Return type

[**crate::models::DocumentGet200Response**](_document_get_200_response.md)

### Authorization

[API token](../README.md#API token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## document_id_id_delete

> document_id_id_delete(id)


delete a document

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


## document_id_id_download_document_version_get

> document_id_id_download_document_version_get(id, id2)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |
**id2** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

[API token](../README.md#API token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: image/jpeg, image/png, application/pdf, */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## document_id_id_download_get

> document_id_id_download_get(id)


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
- **Accept**: image/jpeg, image/png, application/pdf, */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## document_id_id_get

> crate::models::Document document_id_id_get(id)


query document

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

[**crate::models::Document**](document.md)

### Authorization

[API token](../README.md#API token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## document_id_id_put

> crate::models::Document document_id_id_put(id, body)


update document

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |
**body** | [**Document**](Document.md) |  | [required] |

### Return type

[**crate::models::Document**](document.md)

### Authorization

[API token](../README.md#API token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## document_id_id_upload_post

> crate::models::DocumentIdIdUploadPost200Response document_id_id_upload_post(id, comment)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |
**comment** | Option<**String**> |  |  |

### Return type

[**crate::models::DocumentIdIdUploadPost200Response**](_document_id__id__upload_post_200_response.md)

### Authorization

[API token](../README.md#API token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## document_upload_post

> crate::models::DocumentIdIdUploadPost200Response document_upload_post(entity_name, entity_id, name, description)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**entity_name** | **String** |  | [required] |
**entity_id** | **String** |  | [required] |
**name** | **String** |  | [required] |
**description** | Option<**String**> |  |  |

### Return type

[**crate::models::DocumentIdIdUploadPost200Response**](_document_id__id__upload_post_200_response.md)

### Authorization

[API token](../README.md#API token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

