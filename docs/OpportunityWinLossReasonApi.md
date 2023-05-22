# \OpportunityWinLossReasonApi

All URIs are relative to *https://www.weclapp.com/webapp/api/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**opportunity_win_loss_reason_count_get**](OpportunityWinLossReasonApi.md#opportunity_win_loss_reason_count_get) | **GET** /opportunityWinLossReason/count | 
[**opportunity_win_loss_reason_get**](OpportunityWinLossReasonApi.md#opportunity_win_loss_reason_get) | **GET** /opportunityWinLossReason | 
[**opportunity_win_loss_reason_id_id_delete**](OpportunityWinLossReasonApi.md#opportunity_win_loss_reason_id_id_delete) | **DELETE** /opportunityWinLossReason/id/{id} | 
[**opportunity_win_loss_reason_id_id_get**](OpportunityWinLossReasonApi.md#opportunity_win_loss_reason_id_id_get) | **GET** /opportunityWinLossReason/id/{id} | 
[**opportunity_win_loss_reason_id_id_put**](OpportunityWinLossReasonApi.md#opportunity_win_loss_reason_id_id_put) | **PUT** /opportunityWinLossReason/id/{id} | 
[**opportunity_win_loss_reason_post**](OpportunityWinLossReasonApi.md#opportunity_win_loss_reason_post) | **POST** /opportunityWinLossReason | 



## opportunity_win_loss_reason_count_get

> crate::models::AccountingTransactionCountGet200Response opportunity_win_loss_reason_count_get(page, page_size, sort)


count opportunityWinLossReason

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


## opportunity_win_loss_reason_get

> crate::models::ArticleAccountingCodeGet200Response opportunity_win_loss_reason_get(page, page_size, sort)


query opportunityWinLossReason

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


## opportunity_win_loss_reason_id_id_delete

> opportunity_win_loss_reason_id_id_delete(id)


delete a opportunityWinLossReason

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


## opportunity_win_loss_reason_id_id_get

> crate::models::CustomValue opportunity_win_loss_reason_id_id_get(id)


query opportunityWinLossReason

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


## opportunity_win_loss_reason_id_id_put

> crate::models::CustomValue opportunity_win_loss_reason_id_id_put(id, body)


update opportunityWinLossReason

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


## opportunity_win_loss_reason_post

> crate::models::CustomValue opportunity_win_loss_reason_post(body)


create a opportunityWinLossReason

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

