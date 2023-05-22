# \PaymentMethodApi

All URIs are relative to *https://www.weclapp.com/webapp/api/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**payment_method_count_get**](PaymentMethodApi.md#payment_method_count_get) | **GET** /paymentMethod/count | 
[**payment_method_get**](PaymentMethodApi.md#payment_method_get) | **GET** /paymentMethod | 
[**payment_method_id_id_delete**](PaymentMethodApi.md#payment_method_id_id_delete) | **DELETE** /paymentMethod/id/{id} | 
[**payment_method_id_id_get**](PaymentMethodApi.md#payment_method_id_id_get) | **GET** /paymentMethod/id/{id} | 
[**payment_method_id_id_put**](PaymentMethodApi.md#payment_method_id_id_put) | **PUT** /paymentMethod/id/{id} | 
[**payment_method_post**](PaymentMethodApi.md#payment_method_post) | **POST** /paymentMethod | 



## payment_method_count_get

> crate::models::AccountingTransactionCountGet200Response payment_method_count_get(page, page_size, sort)


count paymentMethod

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


## payment_method_get

> crate::models::PaymentMethodGet200Response payment_method_get(page, page_size, sort)


query paymentMethod

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page** | Option<**i32**> |  |  |
**page_size** | Option<**i32**> |  |  |
**sort** | Option<**String**> |  |  |

### Return type

[**crate::models::PaymentMethodGet200Response**](_paymentMethod_get_200_response.md)

### Authorization

[API token](../README.md#API token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## payment_method_id_id_delete

> payment_method_id_id_delete(id)


delete a paymentMethod

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


## payment_method_id_id_get

> crate::models::PaymentMethod payment_method_id_id_get(id)


query paymentMethod

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

[**crate::models::PaymentMethod**](paymentMethod.md)

### Authorization

[API token](../README.md#API token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## payment_method_id_id_put

> crate::models::PaymentMethod payment_method_id_id_put(id, body)


update paymentMethod

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |
**body** | [**PaymentMethod**](PaymentMethod.md) |  | [required] |

### Return type

[**crate::models::PaymentMethod**](paymentMethod.md)

### Authorization

[API token](../README.md#API token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## payment_method_post

> crate::models::PaymentMethod payment_method_post(body)


create a paymentMethod

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**PaymentMethod**](PaymentMethod.md) |  | [required] |

### Return type

[**crate::models::PaymentMethod**](paymentMethod.md)

### Authorization

[API token](../README.md#API token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

