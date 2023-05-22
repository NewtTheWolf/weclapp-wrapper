# \OpportunityApi

All URIs are relative to *https://www.weclapp.com/webapp/api/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**opportunity_count_get**](OpportunityApi.md#opportunity_count_get) | **GET** /opportunity/count | 
[**opportunity_get**](OpportunityApi.md#opportunity_get) | **GET** /opportunity | 
[**opportunity_id_id_delete**](OpportunityApi.md#opportunity_id_id_delete) | **DELETE** /opportunity/id/{id} | 
[**opportunity_id_id_get**](OpportunityApi.md#opportunity_id_id_get) | **GET** /opportunity/id/{id} | 
[**opportunity_id_id_put**](OpportunityApi.md#opportunity_id_id_put) | **PUT** /opportunity/id/{id} | 
[**opportunity_post**](OpportunityApi.md#opportunity_post) | **POST** /opportunity | 



## opportunity_count_get

> crate::models::AccountingTransactionCountGet200Response opportunity_count_get(page, page_size, sort)


count opportunity

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


## opportunity_get

> crate::models::OpportunityGet200Response opportunity_get(page, page_size, sort)


query opportunity

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page** | Option<**i32**> |  |  |
**page_size** | Option<**i32**> |  |  |
**sort** | Option<**String**> |  |  |

### Return type

[**crate::models::OpportunityGet200Response**](_opportunity_get_200_response.md)

### Authorization

[API token](../README.md#API token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## opportunity_id_id_delete

> opportunity_id_id_delete(id)


delete a opportunity

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


## opportunity_id_id_get

> crate::models::Opportunity opportunity_id_id_get(id)


query opportunity

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

[**crate::models::Opportunity**](opportunity.md)

### Authorization

[API token](../README.md#API token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## opportunity_id_id_put

> crate::models::Opportunity opportunity_id_id_put(id, body)


update opportunity

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |
**body** | [**Opportunity**](Opportunity.md) |  | [required] |

### Return type

[**crate::models::Opportunity**](opportunity.md)

### Authorization

[API token](../README.md#API token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## opportunity_post

> crate::models::Opportunity opportunity_post(body)


create a opportunity

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**Opportunity**](Opportunity.md) |  | [required] |

### Return type

[**crate::models::Opportunity**](opportunity.md)

### Authorization

[API token](../README.md#API token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

