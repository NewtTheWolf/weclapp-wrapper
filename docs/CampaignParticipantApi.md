# \CampaignParticipantApi

All URIs are relative to *https://www.weclapp.com/webapp/api/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**campaign_participant_count_get**](CampaignParticipantApi.md#campaign_participant_count_get) | **GET** /campaignParticipant/count | 
[**campaign_participant_get**](CampaignParticipantApi.md#campaign_participant_get) | **GET** /campaignParticipant | 
[**campaign_participant_id_id_delete**](CampaignParticipantApi.md#campaign_participant_id_id_delete) | **DELETE** /campaignParticipant/id/{id} | 
[**campaign_participant_id_id_get**](CampaignParticipantApi.md#campaign_participant_id_id_get) | **GET** /campaignParticipant/id/{id} | 
[**campaign_participant_id_id_put**](CampaignParticipantApi.md#campaign_participant_id_id_put) | **PUT** /campaignParticipant/id/{id} | 
[**campaign_participant_post**](CampaignParticipantApi.md#campaign_participant_post) | **POST** /campaignParticipant | 



## campaign_participant_count_get

> crate::models::AccountingTransactionCountGet200Response campaign_participant_count_get(page, page_size, sort)


count campaignParticipant

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


## campaign_participant_get

> crate::models::CampaignParticipantGet200Response campaign_participant_get(page, page_size, sort)


query campaignParticipant

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page** | Option<**i32**> |  |  |
**page_size** | Option<**i32**> |  |  |
**sort** | Option<**String**> |  |  |

### Return type

[**crate::models::CampaignParticipantGet200Response**](_campaignParticipant_get_200_response.md)

### Authorization

[API token](../README.md#API token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## campaign_participant_id_id_delete

> campaign_participant_id_id_delete(id)


delete a campaignParticipant

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


## campaign_participant_id_id_get

> crate::models::CampaignParticipant campaign_participant_id_id_get(id)


query campaignParticipant

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

[**crate::models::CampaignParticipant**](campaignParticipant.md)

### Authorization

[API token](../README.md#API token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## campaign_participant_id_id_put

> crate::models::CampaignParticipant campaign_participant_id_id_put(id, body)


update campaignParticipant

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |
**body** | [**CampaignParticipant**](CampaignParticipant.md) |  | [required] |

### Return type

[**crate::models::CampaignParticipant**](campaignParticipant.md)

### Authorization

[API token](../README.md#API token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## campaign_participant_post

> crate::models::CampaignParticipant campaign_participant_post(body)


create a campaignParticipant

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**CampaignParticipant**](CampaignParticipant.md) |  | [required] |

### Return type

[**crate::models::CampaignParticipant**](campaignParticipant.md)

### Authorization

[API token](../README.md#API token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

