# \CustomerLeadLossReasonApi

All URIs are relative to *https://www.weclapp.com/webapp/api/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**customer_lead_loss_reason_count_get**](CustomerLeadLossReasonApi.md#customer_lead_loss_reason_count_get) | **GET** /customerLeadLossReason/count | 
[**customer_lead_loss_reason_get**](CustomerLeadLossReasonApi.md#customer_lead_loss_reason_get) | **GET** /customerLeadLossReason | 
[**customer_lead_loss_reason_id_id_delete**](CustomerLeadLossReasonApi.md#customer_lead_loss_reason_id_id_delete) | **DELETE** /customerLeadLossReason/id/{id} | 
[**customer_lead_loss_reason_id_id_get**](CustomerLeadLossReasonApi.md#customer_lead_loss_reason_id_id_get) | **GET** /customerLeadLossReason/id/{id} | 
[**customer_lead_loss_reason_id_id_put**](CustomerLeadLossReasonApi.md#customer_lead_loss_reason_id_id_put) | **PUT** /customerLeadLossReason/id/{id} | 
[**customer_lead_loss_reason_post**](CustomerLeadLossReasonApi.md#customer_lead_loss_reason_post) | **POST** /customerLeadLossReason | 



## customer_lead_loss_reason_count_get

> crate::models::AccountingTransactionCountGet200Response customer_lead_loss_reason_count_get(page, page_size, sort)


count customerLeadLossReason

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


## customer_lead_loss_reason_get

> crate::models::ArticleAccountingCodeGet200Response customer_lead_loss_reason_get(page, page_size, sort)


query customerLeadLossReason

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


## customer_lead_loss_reason_id_id_delete

> customer_lead_loss_reason_id_id_delete(id)


delete a customerLeadLossReason

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


## customer_lead_loss_reason_id_id_get

> crate::models::CustomValue customer_lead_loss_reason_id_id_get(id)


query customerLeadLossReason

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


## customer_lead_loss_reason_id_id_put

> crate::models::CustomValue customer_lead_loss_reason_id_id_put(id, body)


update customerLeadLossReason

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


## customer_lead_loss_reason_post

> crate::models::CustomValue customer_lead_loss_reason_post(body)


create a customerLeadLossReason

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

