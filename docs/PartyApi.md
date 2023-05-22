# \PartyApi

All URIs are relative to *https://www.weclapp.com/webapp/api/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**party_count_get**](PartyApi.md#party_count_get) | **GET** /party/count | 
[**party_get**](PartyApi.md#party_get) | **GET** /party | 
[**party_id_id_delete**](PartyApi.md#party_id_id_delete) | **DELETE** /party/id/{id} | 
[**party_id_id_download_image_get**](PartyApi.md#party_id_id_download_image_get) | **GET** /party/id/{id}/downloadImage | 
[**party_id_id_get**](PartyApi.md#party_id_id_get) | **GET** /party/id/{id} | 
[**party_id_id_put**](PartyApi.md#party_id_id_put) | **PUT** /party/id/{id} | 
[**party_id_id_upload_image_post**](PartyApi.md#party_id_id_upload_image_post) | **POST** /party/id/{id}/uploadImage | 
[**party_post**](PartyApi.md#party_post) | **POST** /party | 



## party_count_get

> crate::models::AccountingTransactionCountGet200Response party_count_get(page, page_size, sort)


count party

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


## party_get

> crate::models::PartyGet200Response party_get(page, page_size, sort)


query party

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page** | Option<**i32**> |  |  |
**page_size** | Option<**i32**> |  |  |
**sort** | Option<**String**> |  |  |

### Return type

[**crate::models::PartyGet200Response**](_party_get_200_response.md)

### Authorization

[API token](../README.md#API token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## party_id_id_delete

> party_id_id_delete(id)


delete a party

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


## party_id_id_download_image_get

> party_id_id_download_image_get(id, scale_width, scale_height, image_id)


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


## party_id_id_get

> crate::models::Party party_id_id_get(id)


query party

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

[**crate::models::Party**](party.md)

### Authorization

[API token](../README.md#API token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## party_id_id_put

> crate::models::Party party_id_id_put(id, body)


update party

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |
**body** | [**Party**](Party.md) |  | [required] |

### Return type

[**crate::models::Party**](party.md)

### Authorization

[API token](../README.md#API token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## party_id_id_upload_image_post

> crate::models::ArticleIdIdUploadArticleImagePost200Response party_id_id_upload_image_post(id)


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


## party_post

> crate::models::Party party_post(body)


create a party

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**Party**](Party.md) |  | [required] |

### Return type

[**crate::models::Party**](party.md)

### Authorization

[API token](../README.md#API token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

