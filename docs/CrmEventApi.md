# \CrmEventApi

All URIs are relative to *https://www.weclapp.com/webapp/api/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**crm_event_count_get**](CrmEventApi.md#crm_event_count_get) | **GET** /crmEvent/count | 
[**crm_event_get**](CrmEventApi.md#crm_event_get) | **GET** /crmEvent | 
[**crm_event_id_id_delete**](CrmEventApi.md#crm_event_id_id_delete) | **DELETE** /crmEvent/id/{id} | 
[**crm_event_id_id_get**](CrmEventApi.md#crm_event_id_id_get) | **GET** /crmEvent/id/{id} | 
[**crm_event_id_id_put**](CrmEventApi.md#crm_event_id_id_put) | **PUT** /crmEvent/id/{id} | 
[**crm_event_post**](CrmEventApi.md#crm_event_post) | **POST** /crmEvent | 



## crm_event_count_get

> crate::models::AccountingTransactionCountGet200Response crm_event_count_get(page, page_size, sort)


count crmEvent

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


## crm_event_get

> crate::models::CrmEventGet200Response crm_event_get(page, page_size, sort)


query crmEvent

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page** | Option<**i32**> |  |  |
**page_size** | Option<**i32**> |  |  |
**sort** | Option<**String**> |  |  |

### Return type

[**crate::models::CrmEventGet200Response**](_crmEvent_get_200_response.md)

### Authorization

[API token](../README.md#API token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## crm_event_id_id_delete

> crm_event_id_id_delete(id)


delete a crmEvent

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


## crm_event_id_id_get

> crate::models::CrmEvent crm_event_id_id_get(id)


query crmEvent

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

[**crate::models::CrmEvent**](crmEvent.md)

### Authorization

[API token](../README.md#API token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## crm_event_id_id_put

> crate::models::CrmEvent crm_event_id_id_put(id, body)


update crmEvent

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |
**body** | [**CrmEvent**](CrmEvent.md) |  | [required] |

### Return type

[**crate::models::CrmEvent**](crmEvent.md)

### Authorization

[API token](../README.md#API token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## crm_event_post

> crate::models::CrmEvent crm_event_post(body)


create a crmEvent

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**CrmEvent**](CrmEvent.md) |  | [required] |

### Return type

[**crate::models::CrmEvent**](crmEvent.md)

### Authorization

[API token](../README.md#API token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

