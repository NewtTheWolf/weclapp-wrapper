# \ArchivedEmailApi

All URIs are relative to *https://www.weclapp.com/webapp/api/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**archived_email_count_get**](ArchivedEmailApi.md#archived_email_count_get) | **GET** /archivedEmail/count | 
[**archived_email_get**](ArchivedEmailApi.md#archived_email_get) | **GET** /archivedEmail | 
[**archived_email_id_id_delete**](ArchivedEmailApi.md#archived_email_id_id_delete) | **DELETE** /archivedEmail/id/{id} | 
[**archived_email_id_id_get**](ArchivedEmailApi.md#archived_email_id_id_get) | **GET** /archivedEmail/id/{id} | 



## archived_email_count_get

> crate::models::AccountingTransactionCountGet200Response archived_email_count_get(entity_id, entity_name, page, page_size, sort)


count archivedEmail

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


## archived_email_get

> crate::models::ArchivedEmailGet200Response archived_email_get(entity_id, entity_name, page, page_size, sort)


query archivedEmail

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**entity_id** | **String** |  | [required] |
**entity_name** | **String** |  | [required] |
**page** | Option<**i32**> |  |  |
**page_size** | Option<**i32**> |  |  |
**sort** | Option<**String**> |  |  |

### Return type

[**crate::models::ArchivedEmailGet200Response**](_archivedEmail_get_200_response.md)

### Authorization

[API token](../README.md#API token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## archived_email_id_id_delete

> archived_email_id_id_delete(id)


delete a archivedEmail

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


## archived_email_id_id_get

> crate::models::ArchivedEmail archived_email_id_id_get(id)


query archivedEmail

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

[**crate::models::ArchivedEmail**](archivedEmail.md)

### Authorization

[API token](../README.md#API token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

