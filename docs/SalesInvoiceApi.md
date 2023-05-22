# \SalesInvoiceApi

All URIs are relative to *https://www.weclapp.com/webapp/api/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**sales_invoice_count_get**](SalesInvoiceApi.md#sales_invoice_count_get) | **GET** /salesInvoice/count | 
[**sales_invoice_get**](SalesInvoiceApi.md#sales_invoice_get) | **GET** /salesInvoice | 
[**sales_invoice_id_id_add_sales_orders_post**](SalesInvoiceApi.md#sales_invoice_id_id_add_sales_orders_post) | **POST** /salesInvoice/id/{id}/addSalesOrders | 
[**sales_invoice_id_id_create_credit_note_post**](SalesInvoiceApi.md#sales_invoice_id_id_create_credit_note_post) | **POST** /salesInvoice/id/{id}/createCreditNote | 
[**sales_invoice_id_id_delete**](SalesInvoiceApi.md#sales_invoice_id_id_delete) | **DELETE** /salesInvoice/id/{id} | 
[**sales_invoice_id_id_download_latest_sales_invoice_pdf_get**](SalesInvoiceApi.md#sales_invoice_id_id_download_latest_sales_invoice_pdf_get) | **GET** /salesInvoice/id/{id}/downloadLatestSalesInvoicePdf | 
[**sales_invoice_id_id_get**](SalesInvoiceApi.md#sales_invoice_id_id_get) | **GET** /salesInvoice/id/{id} | 
[**sales_invoice_id_id_put**](SalesInvoiceApi.md#sales_invoice_id_id_put) | **PUT** /salesInvoice/id/{id} | 
[**sales_invoice_post**](SalesInvoiceApi.md#sales_invoice_post) | **POST** /salesInvoice | 



## sales_invoice_count_get

> crate::models::AccountingTransactionCountGet200Response sales_invoice_count_get(page, page_size, sort)


count salesInvoice

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


## sales_invoice_get

> crate::models::SalesInvoiceGet200Response sales_invoice_get(page, page_size, sort)


query salesInvoice

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page** | Option<**i32**> |  |  |
**page_size** | Option<**i32**> |  |  |
**sort** | Option<**String**> |  |  |

### Return type

[**crate::models::SalesInvoiceGet200Response**](_salesInvoice_get_200_response.md)

### Authorization

[API token](../README.md#API token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## sales_invoice_id_id_add_sales_orders_post

> crate::models::IncomingGoodsIdIdCreateCreditNotePost200Response sales_invoice_id_id_add_sales_orders_post(id, body)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |
**body** | [**SalesInvoiceIdIdAddSalesOrdersPostRequest**](SalesInvoiceIdIdAddSalesOrdersPostRequest.md) |  | [required] |

### Return type

[**crate::models::IncomingGoodsIdIdCreateCreditNotePost200Response**](_incomingGoods_id__id__createCreditNote_post_200_response.md)

### Authorization

[API token](../README.md#API token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## sales_invoice_id_id_create_credit_note_post

> crate::models::IncomingGoodsIdIdCreateCreditNotePost200Response sales_invoice_id_id_create_credit_note_post(id, body)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |
**body** | [**SalesInvoiceIdIdCreateCreditNotePostRequest**](SalesInvoiceIdIdCreateCreditNotePostRequest.md) |  | [required] |

### Return type

[**crate::models::IncomingGoodsIdIdCreateCreditNotePost200Response**](_incomingGoods_id__id__createCreditNote_post_200_response.md)

### Authorization

[API token](../README.md#API token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## sales_invoice_id_id_delete

> sales_invoice_id_id_delete(id)


delete a salesInvoice

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


## sales_invoice_id_id_download_latest_sales_invoice_pdf_get

> sales_invoice_id_id_download_latest_sales_invoice_pdf_get(id)


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
- **Accept**: image/jpeg, image/png, application/pdf, */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## sales_invoice_id_id_get

> crate::models::SalesInvoice sales_invoice_id_id_get(id)


query salesInvoice

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

[**crate::models::SalesInvoice**](salesInvoice.md)

### Authorization

[API token](../README.md#API token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## sales_invoice_id_id_put

> crate::models::SalesInvoice sales_invoice_id_id_put(id, body)


update salesInvoice

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |
**body** | [**SalesInvoice**](SalesInvoice.md) |  | [required] |

### Return type

[**crate::models::SalesInvoice**](salesInvoice.md)

### Authorization

[API token](../README.md#API token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## sales_invoice_post

> crate::models::SalesInvoice sales_invoice_post(body)


create a salesInvoice

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**SalesInvoice**](SalesInvoice.md) |  | [required] |

### Return type

[**crate::models::SalesInvoice**](salesInvoice.md)

### Authorization

[API token](../README.md#API token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

