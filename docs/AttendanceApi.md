# \AttendanceApi

All URIs are relative to *https://www.weclapp.com/webapp/api/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**attendance_count_get**](AttendanceApi.md#attendance_count_get) | **GET** /attendance/count | 
[**attendance_current_attendance_get**](AttendanceApi.md#attendance_current_attendance_get) | **GET** /attendance/currentAttendance | 
[**attendance_get**](AttendanceApi.md#attendance_get) | **GET** /attendance | 
[**attendance_id_id_delete**](AttendanceApi.md#attendance_id_id_delete) | **DELETE** /attendance/id/{id} | 
[**attendance_id_id_get**](AttendanceApi.md#attendance_id_id_get) | **GET** /attendance/id/{id} | 
[**attendance_id_id_put**](AttendanceApi.md#attendance_id_id_put) | **PUT** /attendance/id/{id} | 
[**attendance_log_off_post**](AttendanceApi.md#attendance_log_off_post) | **POST** /attendance/logOff | 
[**attendance_log_on_post**](AttendanceApi.md#attendance_log_on_post) | **POST** /attendance/logOn | 
[**attendance_post**](AttendanceApi.md#attendance_post) | **POST** /attendance | 



## attendance_count_get

> crate::models::AccountingTransactionCountGet200Response attendance_count_get(page, page_size, sort)


count attendance

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


## attendance_current_attendance_get

> crate::models::AttendanceCurrentAttendanceGet200Response attendance_current_attendance_get()


### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::AttendanceCurrentAttendanceGet200Response**](_attendance_currentAttendance_get_200_response.md)

### Authorization

[API token](../README.md#API token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## attendance_get

> crate::models::AttendanceGet200Response attendance_get(page, page_size, sort)


query attendance

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page** | Option<**i32**> |  |  |
**page_size** | Option<**i32**> |  |  |
**sort** | Option<**String**> |  |  |

### Return type

[**crate::models::AttendanceGet200Response**](_attendance_get_200_response.md)

### Authorization

[API token](../README.md#API token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## attendance_id_id_delete

> attendance_id_id_delete(id)


delete a attendance

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


## attendance_id_id_get

> crate::models::Attendance attendance_id_id_get(id)


query attendance

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

[**crate::models::Attendance**](attendance.md)

### Authorization

[API token](../README.md#API token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## attendance_id_id_put

> crate::models::Attendance attendance_id_id_put(id, body)


update attendance

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |
**body** | [**Attendance**](Attendance.md) |  | [required] |

### Return type

[**crate::models::Attendance**](attendance.md)

### Authorization

[API token](../README.md#API token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## attendance_log_off_post

> crate::models::AttendanceCurrentAttendanceGet200Response attendance_log_off_post(body)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | **serde_json::Value** |  | [required] |

### Return type

[**crate::models::AttendanceCurrentAttendanceGet200Response**](_attendance_currentAttendance_get_200_response.md)

### Authorization

[API token](../README.md#API token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## attendance_log_on_post

> crate::models::AttendanceCurrentAttendanceGet200Response attendance_log_on_post(body)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | **serde_json::Value** |  | [required] |

### Return type

[**crate::models::AttendanceCurrentAttendanceGet200Response**](_attendance_currentAttendance_get_200_response.md)

### Authorization

[API token](../README.md#API token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## attendance_post

> crate::models::Attendance attendance_post(body)


create a attendance

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**Attendance**](Attendance.md) |  | [required] |

### Return type

[**crate::models::Attendance**](attendance.md)

### Authorization

[API token](../README.md#API token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

