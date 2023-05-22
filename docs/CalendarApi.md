# \CalendarApi

All URIs are relative to *https://www.weclapp.com/webapp/api/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**calendar_count_get**](CalendarApi.md#calendar_count_get) | **GET** /calendar/count | 
[**calendar_get**](CalendarApi.md#calendar_get) | **GET** /calendar | 
[**calendar_id_id_delete**](CalendarApi.md#calendar_id_id_delete) | **DELETE** /calendar/id/{id} | 
[**calendar_id_id_delete_calendar_and_move_events_post**](CalendarApi.md#calendar_id_id_delete_calendar_and_move_events_post) | **POST** /calendar/id/{id}/deleteCalendarAndMoveEvents | 
[**calendar_id_id_get**](CalendarApi.md#calendar_id_id_get) | **GET** /calendar/id/{id} | 
[**calendar_id_id_importi_cal_post**](CalendarApi.md#calendar_id_id_importi_cal_post) | **POST** /calendar/id/{id}/importiCal | 
[**calendar_id_id_put**](CalendarApi.md#calendar_id_id_put) | **PUT** /calendar/id/{id} | 
[**calendar_post**](CalendarApi.md#calendar_post) | **POST** /calendar | 



## calendar_count_get

> crate::models::AccountingTransactionCountGet200Response calendar_count_get(page, page_size, sort)


count calendar

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


## calendar_get

> crate::models::CalendarGet200Response calendar_get(page, page_size, sort)


query calendar

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page** | Option<**i32**> |  |  |
**page_size** | Option<**i32**> |  |  |
**sort** | Option<**String**> |  |  |

### Return type

[**crate::models::CalendarGet200Response**](_calendar_get_200_response.md)

### Authorization

[API token](../README.md#API token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## calendar_id_id_delete

> calendar_id_id_delete(id)


delete a calendar

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


## calendar_id_id_delete_calendar_and_move_events_post

> calendar_id_id_delete_calendar_and_move_events_post(id, body)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |
**body** | [**CalendarIdIdDeleteCalendarAndMoveEventsPostRequest**](CalendarIdIdDeleteCalendarAndMoveEventsPostRequest.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

[API token](../README.md#API token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## calendar_id_id_get

> crate::models::Calendar calendar_id_id_get(id)


query calendar

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

[**crate::models::Calendar**](calendar.md)

### Authorization

[API token](../README.md#API token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## calendar_id_id_importi_cal_post

> crate::models::ArticleIdIdUploadArticleImagePost200Response calendar_id_id_importi_cal_post(id)


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


## calendar_id_id_put

> crate::models::Calendar calendar_id_id_put(id, body)


update calendar

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |
**body** | [**Calendar**](Calendar.md) |  | [required] |

### Return type

[**crate::models::Calendar**](calendar.md)

### Authorization

[API token](../README.md#API token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## calendar_post

> crate::models::Calendar calendar_post(body)


create a calendar

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**Calendar**](Calendar.md) |  | [required] |

### Return type

[**crate::models::Calendar**](calendar.md)

### Authorization

[API token](../README.md#API token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

