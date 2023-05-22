# \TermOfPaymentApi

All URIs are relative to *https://www.weclapp.com/webapp/api/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**term_of_payment_count_get**](TermOfPaymentApi.md#term_of_payment_count_get) | **GET** /termOfPayment/count | 
[**term_of_payment_get**](TermOfPaymentApi.md#term_of_payment_get) | **GET** /termOfPayment | 
[**term_of_payment_id_id_delete**](TermOfPaymentApi.md#term_of_payment_id_id_delete) | **DELETE** /termOfPayment/id/{id} | 
[**term_of_payment_id_id_get**](TermOfPaymentApi.md#term_of_payment_id_id_get) | **GET** /termOfPayment/id/{id} | 
[**term_of_payment_id_id_put**](TermOfPaymentApi.md#term_of_payment_id_id_put) | **PUT** /termOfPayment/id/{id} | 
[**term_of_payment_post**](TermOfPaymentApi.md#term_of_payment_post) | **POST** /termOfPayment | 



## term_of_payment_count_get

> crate::models::AccountingTransactionCountGet200Response term_of_payment_count_get(page, page_size, sort)


count termOfPayment

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


## term_of_payment_get

> crate::models::TermOfPaymentGet200Response term_of_payment_get(page, page_size, sort)


query termOfPayment

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page** | Option<**i32**> |  |  |
**page_size** | Option<**i32**> |  |  |
**sort** | Option<**String**> |  |  |

### Return type

[**crate::models::TermOfPaymentGet200Response**](_termOfPayment_get_200_response.md)

### Authorization

[API token](../README.md#API token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## term_of_payment_id_id_delete

> term_of_payment_id_id_delete(id)


delete a termOfPayment

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


## term_of_payment_id_id_get

> crate::models::TermOfPayment term_of_payment_id_id_get(id)


query termOfPayment

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

[**crate::models::TermOfPayment**](termOfPayment.md)

### Authorization

[API token](../README.md#API token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## term_of_payment_id_id_put

> crate::models::TermOfPayment term_of_payment_id_id_put(id, body)


update termOfPayment

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |
**body** | [**TermOfPayment**](TermOfPayment.md) |  | [required] |

### Return type

[**crate::models::TermOfPayment**](termOfPayment.md)

### Authorization

[API token](../README.md#API token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## term_of_payment_post

> crate::models::TermOfPayment term_of_payment_post(body)


create a termOfPayment

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**TermOfPayment**](TermOfPayment.md) |  | [required] |

### Return type

[**crate::models::TermOfPayment**](termOfPayment.md)

### Authorization

[API token](../README.md#API token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

