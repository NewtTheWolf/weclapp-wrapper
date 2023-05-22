# \ProductionWorkScheduleAssignmentApi

All URIs are relative to *https://www.weclapp.com/webapp/api/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**production_work_schedule_assignment_count_get**](ProductionWorkScheduleAssignmentApi.md#production_work_schedule_assignment_count_get) | **GET** /productionWorkScheduleAssignment/count | 
[**production_work_schedule_assignment_get**](ProductionWorkScheduleAssignmentApi.md#production_work_schedule_assignment_get) | **GET** /productionWorkScheduleAssignment | 
[**production_work_schedule_assignment_id_id_delete**](ProductionWorkScheduleAssignmentApi.md#production_work_schedule_assignment_id_id_delete) | **DELETE** /productionWorkScheduleAssignment/id/{id} | 
[**production_work_schedule_assignment_id_id_get**](ProductionWorkScheduleAssignmentApi.md#production_work_schedule_assignment_id_id_get) | **GET** /productionWorkScheduleAssignment/id/{id} | 
[**production_work_schedule_assignment_id_id_put**](ProductionWorkScheduleAssignmentApi.md#production_work_schedule_assignment_id_id_put) | **PUT** /productionWorkScheduleAssignment/id/{id} | 
[**production_work_schedule_assignment_post**](ProductionWorkScheduleAssignmentApi.md#production_work_schedule_assignment_post) | **POST** /productionWorkScheduleAssignment | 



## production_work_schedule_assignment_count_get

> crate::models::AccountingTransactionCountGet200Response production_work_schedule_assignment_count_get(page, page_size, sort)


count productionWorkScheduleAssignment

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


## production_work_schedule_assignment_get

> crate::models::ProductionWorkScheduleAssignmentGet200Response production_work_schedule_assignment_get(page, page_size, sort)


query productionWorkScheduleAssignment

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page** | Option<**i32**> |  |  |
**page_size** | Option<**i32**> |  |  |
**sort** | Option<**String**> |  |  |

### Return type

[**crate::models::ProductionWorkScheduleAssignmentGet200Response**](_productionWorkScheduleAssignment_get_200_response.md)

### Authorization

[API token](../README.md#API token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## production_work_schedule_assignment_id_id_delete

> production_work_schedule_assignment_id_id_delete(id)


delete a productionWorkScheduleAssignment

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


## production_work_schedule_assignment_id_id_get

> crate::models::ProductionWorkScheduleAssignment production_work_schedule_assignment_id_id_get(id)


query productionWorkScheduleAssignment

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

[**crate::models::ProductionWorkScheduleAssignment**](productionWorkScheduleAssignment.md)

### Authorization

[API token](../README.md#API token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## production_work_schedule_assignment_id_id_put

> crate::models::ProductionWorkScheduleAssignment production_work_schedule_assignment_id_id_put(id, body)


update productionWorkScheduleAssignment

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |
**body** | [**ProductionWorkScheduleAssignment**](ProductionWorkScheduleAssignment.md) |  | [required] |

### Return type

[**crate::models::ProductionWorkScheduleAssignment**](productionWorkScheduleAssignment.md)

### Authorization

[API token](../README.md#API token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## production_work_schedule_assignment_post

> crate::models::ProductionWorkScheduleAssignment production_work_schedule_assignment_post(body)


create a productionWorkScheduleAssignment

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**ProductionWorkScheduleAssignment**](ProductionWorkScheduleAssignment.md) |  | [required] |

### Return type

[**crate::models::ProductionWorkScheduleAssignment**](productionWorkScheduleAssignment.md)

### Authorization

[API token](../README.md#API token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

