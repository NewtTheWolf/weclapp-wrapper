# \SepaDirectDebitMandateApi

All URIs are relative to *https://www.weclapp.com/webapp/api/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**sepa_direct_debit_mandate_count_get**](SepaDirectDebitMandateApi.md#sepa_direct_debit_mandate_count_get) | **GET** /sepaDirectDebitMandate/count | 
[**sepa_direct_debit_mandate_get**](SepaDirectDebitMandateApi.md#sepa_direct_debit_mandate_get) | **GET** /sepaDirectDebitMandate | 
[**sepa_direct_debit_mandate_id_id_delete**](SepaDirectDebitMandateApi.md#sepa_direct_debit_mandate_id_id_delete) | **DELETE** /sepaDirectDebitMandate/id/{id} | 
[**sepa_direct_debit_mandate_id_id_get**](SepaDirectDebitMandateApi.md#sepa_direct_debit_mandate_id_id_get) | **GET** /sepaDirectDebitMandate/id/{id} | 
[**sepa_direct_debit_mandate_id_id_put**](SepaDirectDebitMandateApi.md#sepa_direct_debit_mandate_id_id_put) | **PUT** /sepaDirectDebitMandate/id/{id} | 
[**sepa_direct_debit_mandate_post**](SepaDirectDebitMandateApi.md#sepa_direct_debit_mandate_post) | **POST** /sepaDirectDebitMandate | 



## sepa_direct_debit_mandate_count_get

> crate::models::AccountingTransactionCountGet200Response sepa_direct_debit_mandate_count_get(page, page_size, sort)


count sepaDirectDebitMandate

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


## sepa_direct_debit_mandate_get

> crate::models::SepaDirectDebitMandateGet200Response sepa_direct_debit_mandate_get(page, page_size, sort)


query sepaDirectDebitMandate

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page** | Option<**i32**> |  |  |
**page_size** | Option<**i32**> |  |  |
**sort** | Option<**String**> |  |  |

### Return type

[**crate::models::SepaDirectDebitMandateGet200Response**](_sepaDirectDebitMandate_get_200_response.md)

### Authorization

[API token](../README.md#API token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## sepa_direct_debit_mandate_id_id_delete

> sepa_direct_debit_mandate_id_id_delete(id)


delete a sepaDirectDebitMandate

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


## sepa_direct_debit_mandate_id_id_get

> crate::models::SepaDirectDebitMandate sepa_direct_debit_mandate_id_id_get(id)


query sepaDirectDebitMandate

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

[**crate::models::SepaDirectDebitMandate**](sepaDirectDebitMandate.md)

### Authorization

[API token](../README.md#API token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## sepa_direct_debit_mandate_id_id_put

> crate::models::SepaDirectDebitMandate sepa_direct_debit_mandate_id_id_put(id, body)


update sepaDirectDebitMandate

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |
**body** | [**SepaDirectDebitMandate**](SepaDirectDebitMandate.md) |  | [required] |

### Return type

[**crate::models::SepaDirectDebitMandate**](sepaDirectDebitMandate.md)

### Authorization

[API token](../README.md#API token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## sepa_direct_debit_mandate_post

> crate::models::SepaDirectDebitMandate sepa_direct_debit_mandate_post(body)


create a sepaDirectDebitMandate

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**SepaDirectDebitMandate**](SepaDirectDebitMandate.md) |  | [required] |

### Return type

[**crate::models::SepaDirectDebitMandate**](sepaDirectDebitMandate.md)

### Authorization

[API token](../README.md#API token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

