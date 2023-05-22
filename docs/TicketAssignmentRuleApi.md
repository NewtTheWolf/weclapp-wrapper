# \TicketAssignmentRuleApi

All URIs are relative to *https://www.weclapp.com/webapp/api/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**ticket_assignment_rule_count_get**](TicketAssignmentRuleApi.md#ticket_assignment_rule_count_get) | **GET** /ticketAssignmentRule/count | 
[**ticket_assignment_rule_get**](TicketAssignmentRuleApi.md#ticket_assignment_rule_get) | **GET** /ticketAssignmentRule | 
[**ticket_assignment_rule_id_id_delete**](TicketAssignmentRuleApi.md#ticket_assignment_rule_id_id_delete) | **DELETE** /ticketAssignmentRule/id/{id} | 
[**ticket_assignment_rule_id_id_get**](TicketAssignmentRuleApi.md#ticket_assignment_rule_id_id_get) | **GET** /ticketAssignmentRule/id/{id} | 
[**ticket_assignment_rule_id_id_put**](TicketAssignmentRuleApi.md#ticket_assignment_rule_id_id_put) | **PUT** /ticketAssignmentRule/id/{id} | 
[**ticket_assignment_rule_post**](TicketAssignmentRuleApi.md#ticket_assignment_rule_post) | **POST** /ticketAssignmentRule | 



## ticket_assignment_rule_count_get

> crate::models::AccountingTransactionCountGet200Response ticket_assignment_rule_count_get(page, page_size, sort)


count ticketAssignmentRule

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


## ticket_assignment_rule_get

> crate::models::TicketAssignmentRuleGet200Response ticket_assignment_rule_get(page, page_size, sort)


query ticketAssignmentRule

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page** | Option<**i32**> |  |  |
**page_size** | Option<**i32**> |  |  |
**sort** | Option<**String**> |  |  |

### Return type

[**crate::models::TicketAssignmentRuleGet200Response**](_ticketAssignmentRule_get_200_response.md)

### Authorization

[API token](../README.md#API token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## ticket_assignment_rule_id_id_delete

> ticket_assignment_rule_id_id_delete(id)


delete a ticketAssignmentRule

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


## ticket_assignment_rule_id_id_get

> crate::models::TicketAssignmentRule ticket_assignment_rule_id_id_get(id)


query ticketAssignmentRule

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

[**crate::models::TicketAssignmentRule**](ticketAssignmentRule.md)

### Authorization

[API token](../README.md#API token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## ticket_assignment_rule_id_id_put

> crate::models::TicketAssignmentRule ticket_assignment_rule_id_id_put(id, body)


update ticketAssignmentRule

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |
**body** | [**TicketAssignmentRule**](TicketAssignmentRule.md) |  | [required] |

### Return type

[**crate::models::TicketAssignmentRule**](ticketAssignmentRule.md)

### Authorization

[API token](../README.md#API token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## ticket_assignment_rule_post

> crate::models::TicketAssignmentRule ticket_assignment_rule_post(body)


create a ticketAssignmentRule

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**TicketAssignmentRule**](TicketAssignmentRule.md) |  | [required] |

### Return type

[**crate::models::TicketAssignmentRule**](ticketAssignmentRule.md)

### Authorization

[API token](../README.md#API token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

