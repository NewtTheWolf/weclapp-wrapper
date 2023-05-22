# \CustomsTariffNumberApi

All URIs are relative to *https://www.weclapp.com/webapp/api/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**customs_tariff_number_count_get**](CustomsTariffNumberApi.md#customs_tariff_number_count_get) | **GET** /customsTariffNumber/count | 
[**customs_tariff_number_get**](CustomsTariffNumberApi.md#customs_tariff_number_get) | **GET** /customsTariffNumber | 
[**customs_tariff_number_id_id_delete**](CustomsTariffNumberApi.md#customs_tariff_number_id_id_delete) | **DELETE** /customsTariffNumber/id/{id} | 
[**customs_tariff_number_id_id_get**](CustomsTariffNumberApi.md#customs_tariff_number_id_id_get) | **GET** /customsTariffNumber/id/{id} | 
[**customs_tariff_number_id_id_put**](CustomsTariffNumberApi.md#customs_tariff_number_id_id_put) | **PUT** /customsTariffNumber/id/{id} | 
[**customs_tariff_number_post**](CustomsTariffNumberApi.md#customs_tariff_number_post) | **POST** /customsTariffNumber | 



## customs_tariff_number_count_get

> crate::models::AccountingTransactionCountGet200Response customs_tariff_number_count_get(page, page_size, sort)


count customsTariffNumber

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


## customs_tariff_number_get

> crate::models::ArticleAccountingCodeGet200Response customs_tariff_number_get(page, page_size, sort)


query customsTariffNumber

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


## customs_tariff_number_id_id_delete

> customs_tariff_number_id_id_delete(id)


delete a customsTariffNumber

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


## customs_tariff_number_id_id_get

> crate::models::CustomValue customs_tariff_number_id_id_get(id)


query customsTariffNumber

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


## customs_tariff_number_id_id_put

> crate::models::CustomValue customs_tariff_number_id_id_put(id, body)


update customsTariffNumber

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


## customs_tariff_number_post

> crate::models::CustomValue customs_tariff_number_post(body)


create a customsTariffNumber

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

