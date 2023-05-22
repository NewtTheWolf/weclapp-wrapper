# \OpportunityTopicApi

All URIs are relative to *https://www.weclapp.com/webapp/api/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**opportunity_topic_count_get**](OpportunityTopicApi.md#opportunity_topic_count_get) | **GET** /opportunityTopic/count | 
[**opportunity_topic_get**](OpportunityTopicApi.md#opportunity_topic_get) | **GET** /opportunityTopic | 
[**opportunity_topic_id_id_delete**](OpportunityTopicApi.md#opportunity_topic_id_id_delete) | **DELETE** /opportunityTopic/id/{id} | 
[**opportunity_topic_id_id_get**](OpportunityTopicApi.md#opportunity_topic_id_id_get) | **GET** /opportunityTopic/id/{id} | 
[**opportunity_topic_id_id_put**](OpportunityTopicApi.md#opportunity_topic_id_id_put) | **PUT** /opportunityTopic/id/{id} | 
[**opportunity_topic_post**](OpportunityTopicApi.md#opportunity_topic_post) | **POST** /opportunityTopic | 



## opportunity_topic_count_get

> crate::models::AccountingTransactionCountGet200Response opportunity_topic_count_get(page, page_size, sort)


count opportunityTopic

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


## opportunity_topic_get

> crate::models::ArticleAccountingCodeGet200Response opportunity_topic_get(page, page_size, sort)


query opportunityTopic

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page** | Option<**i32**> |  |  |
**page_size** | Option<**i32**> |  |  |
**sort** | Option<**String**> |  |  |

### Return type

[**crate::models::ArticleAccountingCodeGet200Response**](_articleAccountingCode_get_200_response.md)

### Authorization

[API token](../README.md#API token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## opportunity_topic_id_id_delete

> opportunity_topic_id_id_delete(id)


delete a opportunityTopic

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


## opportunity_topic_id_id_get

> crate::models::CustomValue opportunity_topic_id_id_get(id)


query opportunityTopic

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

[**crate::models::CustomValue**](customValue.md)

### Authorization

[API token](../README.md#API token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## opportunity_topic_id_id_put

> crate::models::CustomValue opportunity_topic_id_id_put(id, body)


update opportunityTopic

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |
**body** | [**CustomValue**](CustomValue.md) |  | [required] |

### Return type

[**crate::models::CustomValue**](customValue.md)

### Authorization

[API token](../README.md#API token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## opportunity_topic_post

> crate::models::CustomValue opportunity_topic_post(body)


create a opportunityTopic

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**CustomValue**](CustomValue.md) |  | [required] |

### Return type

[**crate::models::CustomValue**](customValue.md)

### Authorization

[API token](../README.md#API token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

