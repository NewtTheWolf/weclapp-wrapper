# \SupplierApi

All URIs are relative to *https://www.weclapp.com/webapp/api/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**supplier_count_get**](SupplierApi.md#supplier_count_get) | **GET** /supplier/count | 
[**supplier_get**](SupplierApi.md#supplier_get) | **GET** /supplier | 
[**supplier_id_id_delete**](SupplierApi.md#supplier_id_id_delete) | **DELETE** /supplier/id/{id} | 
[**supplier_id_id_download_image_get**](SupplierApi.md#supplier_id_id_download_image_get) | **GET** /supplier/id/{id}/downloadImage | 
[**supplier_id_id_get**](SupplierApi.md#supplier_id_id_get) | **GET** /supplier/id/{id} | 
[**supplier_id_id_put**](SupplierApi.md#supplier_id_id_put) | **PUT** /supplier/id/{id} | 
[**supplier_id_id_upload_image_post**](SupplierApi.md#supplier_id_id_upload_image_post) | **POST** /supplier/id/{id}/uploadImage | 
[**supplier_post**](SupplierApi.md#supplier_post) | **POST** /supplier | 



## supplier_count_get

> crate::models::AccountingTransactionCountGet200Response supplier_count_get(page, page_size, sort)


count supplier

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


## supplier_get

> crate::models::SupplierGet200Response supplier_get(page, page_size, sort)


query supplier

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page** | Option<**i32**> |  |  |
**page_size** | Option<**i32**> |  |  |
**sort** | Option<**String**> |  |  |

### Return type

[**crate::models::SupplierGet200Response**](_supplier_get_200_response.md)

### Authorization

[API token](../README.md#API token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## supplier_id_id_delete

> supplier_id_id_delete(id)


delete a supplier

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


## supplier_id_id_download_image_get

> supplier_id_id_download_image_get(id, scale_width, scale_height)


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


## supplier_id_id_get

> crate::models::Supplier supplier_id_id_get(id)


query supplier

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

[**crate::models::Supplier**](supplier.md)

### Authorization

[API token](../README.md#API token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## supplier_id_id_put

> crate::models::Supplier supplier_id_id_put(id, body)


update supplier

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |
**body** | [**Supplier**](Supplier.md) |  | [required] |

### Return type

[**crate::models::Supplier**](supplier.md)

### Authorization

[API token](../README.md#API token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## supplier_id_id_upload_image_post

> crate::models::ArticleIdIdUploadArticleImagePost200Response supplier_id_id_upload_image_post(id)


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


## supplier_post

> crate::models::Supplier supplier_post(body)


create a supplier

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**Supplier**](Supplier.md) |  | [required] |

### Return type

[**crate::models::Supplier**](supplier.md)

### Authorization

[API token](../README.md#API token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

