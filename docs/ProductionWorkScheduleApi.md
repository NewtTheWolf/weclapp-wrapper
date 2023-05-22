# \ProductionWorkScheduleApi

All URIs are relative to *https://www.weclapp.com/webapp/api/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**production_work_schedule_count_get**](ProductionWorkScheduleApi.md#production_work_schedule_count_get) | **GET** /productionWorkSchedule/count | 
[**production_work_schedule_get**](ProductionWorkScheduleApi.md#production_work_schedule_get) | **GET** /productionWorkSchedule | 
[**production_work_schedule_id_id_delete**](ProductionWorkScheduleApi.md#production_work_schedule_id_id_delete) | **DELETE** /productionWorkSchedule/id/{id} | 
[**production_work_schedule_id_id_get**](ProductionWorkScheduleApi.md#production_work_schedule_id_id_get) | **GET** /productionWorkSchedule/id/{id} | 
[**production_work_schedule_id_id_put**](ProductionWorkScheduleApi.md#production_work_schedule_id_id_put) | **PUT** /productionWorkSchedule/id/{id} | 
[**production_work_schedule_post**](ProductionWorkScheduleApi.md#production_work_schedule_post) | **POST** /productionWorkSchedule | 



## production_work_schedule_count_get

> crate::models::AccountingTransactionCountGet200Response production_work_schedule_count_get(page, page_size, sort)


count productionWorkSchedule

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


## production_work_schedule_get

> crate::models::ProductionWorkScheduleGet200Response production_work_schedule_get(page, page_size, sort)


query productionWorkSchedule

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page** | Option<**i32**> |  |  |
**page_size** | Option<**i32**> |  |  |
**sort** | Option<**String**> |  |  |

### Return type

[**crate::models::ProductionWorkScheduleGet200Response**](_productionWorkSchedule_get_200_response.md)

### Authorization

[API token](../README.md#API token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## production_work_schedule_id_id_delete

> production_work_schedule_id_id_delete(id)


delete a productionWorkSchedule

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


## production_work_schedule_id_id_get

> crate::models::ProductionWorkSchedule production_work_schedule_id_id_get(id)


query productionWorkSchedule

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

[**crate::models::ProductionWorkSchedule**](productionWorkSchedule.md)

### Authorization

[API token](../README.md#API token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## production_work_schedule_id_id_put

> crate::models::ProductionWorkSchedule production_work_schedule_id_id_put(id, body)


update productionWorkSchedule

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |
**body** | [**ProductionWorkSchedule**](ProductionWorkSchedule.md) |  | [required] |

### Return type

[**crate::models::ProductionWorkSchedule**](productionWorkSchedule.md)

### Authorization

[API token](../README.md#API token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## production_work_schedule_post

> crate::models::ProductionWorkSchedule production_work_schedule_post(body)


create a productionWorkSchedule

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**ProductionWorkSchedule**](ProductionWorkSchedule.md) |  | [required] |

### Return type

[**crate::models::ProductionWorkSchedule**](productionWorkSchedule.md)

### Authorization

[API token](../README.md#API token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

