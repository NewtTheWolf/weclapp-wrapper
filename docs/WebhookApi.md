# \WebhookApi

All URIs are relative to *https://www.weclapp.com/webapp/api/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**webhook_count_get**](WebhookApi.md#webhook_count_get) | **GET** /webhook/count | 
[**webhook_get**](WebhookApi.md#webhook_get) | **GET** /webhook | 
[**webhook_id_id_delete**](WebhookApi.md#webhook_id_id_delete) | **DELETE** /webhook/id/{id} | 
[**webhook_id_id_get**](WebhookApi.md#webhook_id_id_get) | **GET** /webhook/id/{id} | 
[**webhook_id_id_put**](WebhookApi.md#webhook_id_id_put) | **PUT** /webhook/id/{id} | 
[**webhook_post**](WebhookApi.md#webhook_post) | **POST** /webhook | 



## webhook_count_get

> crate::models::AccountingTransactionCountGet200Response webhook_count_get(page, page_size, sort)


count webhook

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


## webhook_get

> crate::models::WebhookGet200Response webhook_get(page, page_size, sort)


query webhook

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page** | Option<**i32**> |  |  |
**page_size** | Option<**i32**> |  |  |
**sort** | Option<**String**> |  |  |

### Return type

[**crate::models::WebhookGet200Response**](_webhook_get_200_response.md)

### Authorization

[API token](../README.md#API token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## webhook_id_id_delete

> webhook_id_id_delete(id)


delete a webhook

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


## webhook_id_id_get

> crate::models::Webhook webhook_id_id_get(id)


query webhook

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

[**crate::models::Webhook**](webhook.md)

### Authorization

[API token](../README.md#API token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## webhook_id_id_put

> crate::models::Webhook webhook_id_id_put(id, body)


update webhook

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |
**body** | [**Webhook**](Webhook.md) |  | [required] |

### Return type

[**crate::models::Webhook**](webhook.md)

### Authorization

[API token](../README.md#API token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## webhook_post

> crate::models::Webhook webhook_post(body)


create a webhook

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**Webhook**](Webhook.md) |  | [required] |

### Return type

[**crate::models::Webhook**](webhook.md)

### Authorization

[API token](../README.md#API token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

