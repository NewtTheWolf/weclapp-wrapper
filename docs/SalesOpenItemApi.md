# \SalesOpenItemApi

All URIs are relative to *https://www.weclapp.com/webapp/api/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**sales_open_item_count_get**](SalesOpenItemApi.md#sales_open_item_count_get) | **GET** /salesOpenItem/count | 
[**sales_open_item_get**](SalesOpenItemApi.md#sales_open_item_get) | **GET** /salesOpenItem | 
[**sales_open_item_id_id_create_payment_application_post**](SalesOpenItemApi.md#sales_open_item_id_id_create_payment_application_post) | **POST** /salesOpenItem/id/{id}/createPaymentApplication | 
[**sales_open_item_id_id_get**](SalesOpenItemApi.md#sales_open_item_id_id_get) | **GET** /salesOpenItem/id/{id} | 



## sales_open_item_count_get

> crate::models::AccountingTransactionCountGet200Response sales_open_item_count_get(page, page_size, sort)


count salesOpenItem

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


## sales_open_item_get

> crate::models::SalesOpenItemGet200Response sales_open_item_get(page, page_size, sort)


query salesOpenItem

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page** | Option<**i32**> |  |  |
**page_size** | Option<**i32**> |  |  |
**sort** | Option<**String**> |  |  |

### Return type

[**crate::models::SalesOpenItemGet200Response**](_salesOpenItem_get_200_response.md)

### Authorization

[API token](../README.md#API token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## sales_open_item_id_id_create_payment_application_post

> crate::models::SalesOpenItemIdIdCreatePaymentApplicationPost200Response sales_open_item_id_id_create_payment_application_post(id, body)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |
**body** | [**PurchaseOpenItemIdIdCreatePaymentApplicationPostRequest**](PurchaseOpenItemIdIdCreatePaymentApplicationPostRequest.md) |  | [required] |

### Return type

[**crate::models::SalesOpenItemIdIdCreatePaymentApplicationPost200Response**](_salesOpenItem_id__id__createPaymentApplication_post_200_response.md)

### Authorization

[API token](../README.md#API token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## sales_open_item_id_id_get

> crate::models::SalesOpenItem sales_open_item_id_id_get(id)


query salesOpenItem

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

[**crate::models::SalesOpenItem**](salesOpenItem.md)

### Authorization

[API token](../README.md#API token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

