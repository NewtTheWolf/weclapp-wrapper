# \QuotationApi

All URIs are relative to *https://www.weclapp.com/webapp/api/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**quotation_count_get**](QuotationApi.md#quotation_count_get) | **GET** /quotation/count | 
[**quotation_get**](QuotationApi.md#quotation_get) | **GET** /quotation | 
[**quotation_id_id_accept_post**](QuotationApi.md#quotation_id_id_accept_post) | **POST** /quotation/id/{id}/accept | 
[**quotation_id_id_create_new_version_post**](QuotationApi.md#quotation_id_id_create_new_version_post) | **POST** /quotation/id/{id}/createNewVersion | 
[**quotation_id_id_create_quotation_pdf_post**](QuotationApi.md#quotation_id_id_create_quotation_pdf_post) | **POST** /quotation/id/{id}/createQuotationPdf | 
[**quotation_id_id_delete**](QuotationApi.md#quotation_id_id_delete) | **DELETE** /quotation/id/{id} | 
[**quotation_id_id_download_latest_quotation_pdf_get**](QuotationApi.md#quotation_id_id_download_latest_quotation_pdf_get) | **GET** /quotation/id/{id}/downloadLatestQuotationPdf | 
[**quotation_id_id_get**](QuotationApi.md#quotation_id_id_get) | **GET** /quotation/id/{id} | 
[**quotation_id_id_inquire_post**](QuotationApi.md#quotation_id_id_inquire_post) | **POST** /quotation/id/{id}/inquire | 
[**quotation_id_id_put**](QuotationApi.md#quotation_id_id_put) | **PUT** /quotation/id/{id} | 
[**quotation_post**](QuotationApi.md#quotation_post) | **POST** /quotation | 



## quotation_count_get

> crate::models::AccountingTransactionCountGet200Response quotation_count_get(page, page_size, sort)


count quotation

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


## quotation_get

> crate::models::QuotationGet200Response quotation_get(page, page_size, sort)


query quotation

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page** | Option<**i32**> |  |  |
**page_size** | Option<**i32**> |  |  |
**sort** | Option<**String**> |  |  |

### Return type

[**crate::models::QuotationGet200Response**](_quotation_get_200_response.md)

### Authorization

[API token](../README.md#API token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## quotation_id_id_accept_post

> quotation_id_id_accept_post(id, body)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |
**body** | [**QuotationIdIdAcceptPostRequest**](QuotationIdIdAcceptPostRequest.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

[API token](../README.md#API token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## quotation_id_id_create_new_version_post

> crate::models::QuotationIdIdCreateNewVersionPost200Response quotation_id_id_create_new_version_post(id, body)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |
**body** | **serde_json::Value** |  | [required] |

### Return type

[**crate::models::QuotationIdIdCreateNewVersionPost200Response**](_quotation_id__id__createNewVersion_post_200_response.md)

### Authorization

[API token](../README.md#API token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## quotation_id_id_create_quotation_pdf_post

> quotation_id_id_create_quotation_pdf_post(id, body)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |
**body** | **serde_json::Value** |  | [required] |

### Return type

 (empty response body)

### Authorization

[API token](../README.md#API token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: image/jpeg, image/png, application/pdf, */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## quotation_id_id_delete

> quotation_id_id_delete(id)


delete a quotation

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


## quotation_id_id_download_latest_quotation_pdf_get

> quotation_id_id_download_latest_quotation_pdf_get(id)


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


## quotation_id_id_get

> crate::models::Quotation quotation_id_id_get(id)


query quotation

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

[**crate::models::Quotation**](quotation.md)

### Authorization

[API token](../README.md#API token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## quotation_id_id_inquire_post

> crate::models::QuotationIdIdCreateNewVersionPost200Response quotation_id_id_inquire_post(id, body)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |
**body** | [**QuotationIdIdInquirePostRequest**](QuotationIdIdInquirePostRequest.md) |  | [required] |

### Return type

[**crate::models::QuotationIdIdCreateNewVersionPost200Response**](_quotation_id__id__createNewVersion_post_200_response.md)

### Authorization

[API token](../README.md#API token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## quotation_id_id_put

> crate::models::Quotation quotation_id_id_put(id, body)


update quotation

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |
**body** | [**Quotation**](Quotation.md) |  | [required] |

### Return type

[**crate::models::Quotation**](quotation.md)

### Authorization

[API token](../README.md#API token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## quotation_post

> crate::models::Quotation quotation_post(body)


create a quotation

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**Quotation**](Quotation.md) |  | [required] |

### Return type

[**crate::models::Quotation**](quotation.md)

### Authorization

[API token](../README.md#API token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

