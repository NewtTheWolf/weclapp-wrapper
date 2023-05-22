# \TicketApi

All URIs are relative to *https://www.weclapp.com/webapp/api/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**ticket_count_get**](TicketApi.md#ticket_count_get) | **GET** /ticket/count | 
[**ticket_get**](TicketApi.md#ticket_get) | **GET** /ticket | 
[**ticket_id_id_create_public_page_post**](TicketApi.md#ticket_id_id_create_public_page_post) | **POST** /ticket/id/{id}/createPublicPage | 
[**ticket_id_id_delete**](TicketApi.md#ticket_id_id_delete) | **DELETE** /ticket/id/{id} | 
[**ticket_id_id_disable_public_page_post**](TicketApi.md#ticket_id_id_disable_public_page_post) | **POST** /ticket/id/{id}/disablePublicPage | 
[**ticket_id_id_get**](TicketApi.md#ticket_id_id_get) | **GET** /ticket/id/{id} | 
[**ticket_id_id_put**](TicketApi.md#ticket_id_id_put) | **PUT** /ticket/id/{id} | 
[**ticket_post**](TicketApi.md#ticket_post) | **POST** /ticket | 



## ticket_count_get

> crate::models::AccountingTransactionCountGet200Response ticket_count_get(page, page_size, sort)


count ticket

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


## ticket_get

> crate::models::TicketGet200Response ticket_get(page, page_size, sort)


query ticket

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page** | Option<**i32**> |  |  |
**page_size** | Option<**i32**> |  |  |
**sort** | Option<**String**> |  |  |

### Return type

[**crate::models::TicketGet200Response**](_ticket_get_200_response.md)

### Authorization

[API token](../README.md#API token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## ticket_id_id_create_public_page_post

> crate::models::TicketIdIdCreatePublicPagePost200Response ticket_id_id_create_public_page_post(id, body)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |
**body** | **serde_json::Value** |  | [required] |

### Return type

[**crate::models::TicketIdIdCreatePublicPagePost200Response**](_ticket_id__id__createPublicPage_post_200_response.md)

### Authorization

[API token](../README.md#API token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## ticket_id_id_delete

> ticket_id_id_delete(id)


delete a ticket

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


## ticket_id_id_disable_public_page_post

> crate::models::TicketIdIdCreatePublicPagePost200Response ticket_id_id_disable_public_page_post(id, body)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |
**body** | **serde_json::Value** |  | [required] |

### Return type

[**crate::models::TicketIdIdCreatePublicPagePost200Response**](_ticket_id__id__createPublicPage_post_200_response.md)

### Authorization

[API token](../README.md#API token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## ticket_id_id_get

> crate::models::Ticket ticket_id_id_get(id)


query ticket

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

[**crate::models::Ticket**](ticket.md)

### Authorization

[API token](../README.md#API token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## ticket_id_id_put

> crate::models::Ticket ticket_id_id_put(id, body)


update ticket

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |
**body** | [**Ticket**](Ticket.md) |  | [required] |

### Return type

[**crate::models::Ticket**](ticket.md)

### Authorization

[API token](../README.md#API token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## ticket_post

> crate::models::Ticket ticket_post(body)


create a ticket

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**Ticket**](Ticket.md) |  | [required] |

### Return type

[**crate::models::Ticket**](ticket.md)

### Authorization

[API token](../README.md#API token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

