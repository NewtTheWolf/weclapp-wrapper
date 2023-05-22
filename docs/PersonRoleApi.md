# \PersonRoleApi

All URIs are relative to *https://www.weclapp.com/webapp/api/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**person_role_count_get**](PersonRoleApi.md#person_role_count_get) | **GET** /personRole/count | 
[**person_role_get**](PersonRoleApi.md#person_role_get) | **GET** /personRole | 
[**person_role_id_id_delete**](PersonRoleApi.md#person_role_id_id_delete) | **DELETE** /personRole/id/{id} | 
[**person_role_id_id_get**](PersonRoleApi.md#person_role_id_id_get) | **GET** /personRole/id/{id} | 
[**person_role_id_id_put**](PersonRoleApi.md#person_role_id_id_put) | **PUT** /personRole/id/{id} | 
[**person_role_post**](PersonRoleApi.md#person_role_post) | **POST** /personRole | 



## person_role_count_get

> crate::models::AccountingTransactionCountGet200Response person_role_count_get(page, page_size, sort)


count personRole

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


## person_role_get

> crate::models::ArticleAccountingCodeGet200Response person_role_get(page, page_size, sort)


query personRole

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


## person_role_id_id_delete

> person_role_id_id_delete(id)


delete a personRole

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


## person_role_id_id_get

> crate::models::CustomValue person_role_id_id_get(id)


query personRole

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


## person_role_id_id_put

> crate::models::CustomValue person_role_id_id_put(id, body)


update personRole

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


## person_role_post

> crate::models::CustomValue person_role_post(body)


create a personRole

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

