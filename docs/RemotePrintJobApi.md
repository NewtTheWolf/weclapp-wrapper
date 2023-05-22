# \RemotePrintJobApi

All URIs are relative to *https://www.weclapp.com/webapp/api/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**remote_print_job_count_get**](RemotePrintJobApi.md#remote_print_job_count_get) | **GET** /remotePrintJob/count | 
[**remote_print_job_create_print_job_with_document_post**](RemotePrintJobApi.md#remote_print_job_create_print_job_with_document_post) | **POST** /remotePrintJob/createPrintJobWithDocument | 
[**remote_print_job_get**](RemotePrintJobApi.md#remote_print_job_get) | **GET** /remotePrintJob | 
[**remote_print_job_id_id_delete**](RemotePrintJobApi.md#remote_print_job_id_id_delete) | **DELETE** /remotePrintJob/id/{id} | 
[**remote_print_job_id_id_get**](RemotePrintJobApi.md#remote_print_job_id_id_get) | **GET** /remotePrintJob/id/{id} | 
[**remote_print_job_id_id_put**](RemotePrintJobApi.md#remote_print_job_id_id_put) | **PUT** /remotePrintJob/id/{id} | 
[**remote_print_job_post**](RemotePrintJobApi.md#remote_print_job_post) | **POST** /remotePrintJob | 



## remote_print_job_count_get

> crate::models::AccountingTransactionCountGet200Response remote_print_job_count_get(page, page_size, sort)


count remotePrintJob

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


## remote_print_job_create_print_job_with_document_post

> crate::models::RemotePrintJobCreatePrintJobWithDocumentPost200Response remote_print_job_create_print_job_with_document_post(weclapp_os_id, printer_name, quantity, document_name, document_description)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**weclapp_os_id** | **String** |  | [required] |
**printer_name** | **String** |  | [required] |
**quantity** | **i32** |  | [required] |
**document_name** | **String** |  | [required] |
**document_description** | Option<**String**> |  |  |

### Return type

[**crate::models::RemotePrintJobCreatePrintJobWithDocumentPost200Response**](_remotePrintJob_createPrintJobWithDocument_post_200_response.md)

### Authorization

[API token](../README.md#API token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## remote_print_job_get

> crate::models::RemotePrintJobGet200Response remote_print_job_get(page, page_size, sort)


query remotePrintJob

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page** | Option<**i32**> |  |  |
**page_size** | Option<**i32**> |  |  |
**sort** | Option<**String**> |  |  |

### Return type

[**crate::models::RemotePrintJobGet200Response**](_remotePrintJob_get_200_response.md)

### Authorization

[API token](../README.md#API token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## remote_print_job_id_id_delete

> remote_print_job_id_id_delete(id)


delete a remotePrintJob

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


## remote_print_job_id_id_get

> crate::models::RemotePrintJob remote_print_job_id_id_get(id)


query remotePrintJob

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

[**crate::models::RemotePrintJob**](remotePrintJob.md)

### Authorization

[API token](../README.md#API token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## remote_print_job_id_id_put

> crate::models::RemotePrintJob remote_print_job_id_id_put(id, body)


update remotePrintJob

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |
**body** | [**RemotePrintJob**](RemotePrintJob.md) |  | [required] |

### Return type

[**crate::models::RemotePrintJob**](remotePrintJob.md)

### Authorization

[API token](../README.md#API token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## remote_print_job_post

> crate::models::RemotePrintJob remote_print_job_post(body)


create a remotePrintJob

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**RemotePrintJob**](RemotePrintJob.md) |  | [required] |

### Return type

[**crate::models::RemotePrintJob**](remotePrintJob.md)

### Authorization

[API token](../README.md#API token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

