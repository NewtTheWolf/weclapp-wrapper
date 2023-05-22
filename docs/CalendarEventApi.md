# \CalendarEventApi

All URIs are relative to *https://www.weclapp.com/webapp/api/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**calendar_event_count_get**](CalendarEventApi.md#calendar_event_count_get) | **GET** /calendarEvent/count | 
[**calendar_event_get**](CalendarEventApi.md#calendar_event_get) | **GET** /calendarEvent | 
[**calendar_event_id_id_delete**](CalendarEventApi.md#calendar_event_id_id_delete) | **DELETE** /calendarEvent/id/{id} | 
[**calendar_event_id_id_get**](CalendarEventApi.md#calendar_event_id_id_get) | **GET** /calendarEvent/id/{id} | 
[**calendar_event_id_id_put**](CalendarEventApi.md#calendar_event_id_id_put) | **PUT** /calendarEvent/id/{id} | 
[**calendar_event_post**](CalendarEventApi.md#calendar_event_post) | **POST** /calendarEvent | 



## calendar_event_count_get

> crate::models::AccountingTransactionCountGet200Response calendar_event_count_get(page, page_size, sort)


count calendarEvent

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


## calendar_event_get

> crate::models::CalendarEventGet200Response calendar_event_get(page, page_size, sort)


query calendarEvent

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page** | Option<**i32**> |  |  |
**page_size** | Option<**i32**> |  |  |
**sort** | Option<**String**> |  |  |

### Return type

[**crate::models::CalendarEventGet200Response**](_calendarEvent_get_200_response.md)

### Authorization

[API token](../README.md#API token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## calendar_event_id_id_delete

> calendar_event_id_id_delete(id)


delete a calendarEvent

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


## calendar_event_id_id_get

> crate::models::CalendarEvent calendar_event_id_id_get(id)


query calendarEvent

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

[**crate::models::CalendarEvent**](calendarEvent.md)

### Authorization

[API token](../README.md#API token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## calendar_event_id_id_put

> crate::models::CalendarEvent calendar_event_id_id_put(id, body)


update calendarEvent

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |
**body** | [**CalendarEvent**](CalendarEvent.md) |  | [required] |

### Return type

[**crate::models::CalendarEvent**](calendarEvent.md)

### Authorization

[API token](../README.md#API token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## calendar_event_post

> crate::models::CalendarEvent calendar_event_post(body)


create a calendarEvent

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**CalendarEvent**](CalendarEvent.md) |  | [required] |

### Return type

[**crate::models::CalendarEvent**](calendarEvent.md)

### Authorization

[API token](../README.md#API token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

