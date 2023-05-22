# \LeadApi

All URIs are relative to *https://www.weclapp.com/webapp/api/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**lead_count_get**](LeadApi.md#lead_count_get) | **GET** /lead/count | 
[**lead_get**](LeadApi.md#lead_get) | **GET** /lead | 
[**lead_id_id_convert_lead_to_customer_get**](LeadApi.md#lead_id_id_convert_lead_to_customer_get) | **GET** /lead/id/{id}/convertLeadToCustomer | 
[**lead_id_id_delete**](LeadApi.md#lead_id_id_delete) | **DELETE** /lead/id/{id} | 
[**lead_id_id_download_image_get**](LeadApi.md#lead_id_id_download_image_get) | **GET** /lead/id/{id}/downloadImage | 
[**lead_id_id_get**](LeadApi.md#lead_id_id_get) | **GET** /lead/id/{id} | 
[**lead_id_id_put**](LeadApi.md#lead_id_id_put) | **PUT** /lead/id/{id} | 
[**lead_id_id_upload_image_post**](LeadApi.md#lead_id_id_upload_image_post) | **POST** /lead/id/{id}/uploadImage | 
[**lead_post**](LeadApi.md#lead_post) | **POST** /lead | 



## lead_count_get

> crate::models::AccountingTransactionCountGet200Response lead_count_get(page, page_size, sort)


count lead

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


## lead_get

> crate::models::LeadGet200Response lead_get(page, page_size, sort)


query lead

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page** | Option<**i32**> |  |  |
**page_size** | Option<**i32**> |  |  |
**sort** | Option<**String**> |  |  |

### Return type

[**crate::models::LeadGet200Response**](_lead_get_200_response.md)

### Authorization

[API token](../README.md#API token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## lead_id_id_convert_lead_to_customer_get

> crate::models::LeadIdIdConvertLeadToCustomerGet200Response lead_id_id_convert_lead_to_customer_get(id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

[**crate::models::LeadIdIdConvertLeadToCustomerGet200Response**](_lead_id__id__convertLeadToCustomer_get_200_response.md)

### Authorization

[API token](../README.md#API token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## lead_id_id_delete

> lead_id_id_delete(id)


delete a lead

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


## lead_id_id_download_image_get

> lead_id_id_download_image_get(id, scale_width, scale_height)


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


## lead_id_id_get

> crate::models::Lead lead_id_id_get(id)


query lead

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

[**crate::models::Lead**](lead.md)

### Authorization

[API token](../README.md#API token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## lead_id_id_put

> crate::models::Lead lead_id_id_put(id, body)


update lead

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |
**body** | [**Lead**](Lead.md) |  | [required] |

### Return type

[**crate::models::Lead**](lead.md)

### Authorization

[API token](../README.md#API token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## lead_id_id_upload_image_post

> crate::models::ArticleIdIdUploadArticleImagePost200Response lead_id_id_upload_image_post(id)


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


## lead_post

> crate::models::Lead lead_post(body)


create a lead

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**Lead**](Lead.md) |  | [required] |

### Return type

[**crate::models::Lead**](lead.md)

### Authorization

[API token](../README.md#API token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

