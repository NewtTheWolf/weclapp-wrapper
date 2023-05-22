# \ShipmentApi

All URIs are relative to *https://www.weclapp.com/webapp/api/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**shipment_count_get**](ShipmentApi.md#shipment_count_get) | **GET** /shipment/count | 
[**shipment_get**](ShipmentApi.md#shipment_get) | **GET** /shipment | 
[**shipment_id_id_create_sales_invoice_post**](ShipmentApi.md#shipment_id_id_create_sales_invoice_post) | **POST** /shipment/id/{id}/createSalesInvoice | 
[**shipment_id_id_create_shipping_label_pdf_post**](ShipmentApi.md#shipment_id_id_create_shipping_label_pdf_post) | **POST** /shipment/id/{id}/createShippingLabelPdf | 
[**shipment_id_id_delete**](ShipmentApi.md#shipment_id_id_delete) | **DELETE** /shipment/id/{id} | 
[**shipment_id_id_download_latest_delivery_note_pdf_get**](ShipmentApi.md#shipment_id_id_download_latest_delivery_note_pdf_get) | **GET** /shipment/id/{id}/downloadLatestDeliveryNotePdf | 
[**shipment_id_id_download_latest_picking_list_pdf_get**](ShipmentApi.md#shipment_id_id_download_latest_picking_list_pdf_get) | **GET** /shipment/id/{id}/downloadLatestPickingListPdf | 
[**shipment_id_id_download_latest_shipping_label_pdf_get**](ShipmentApi.md#shipment_id_id_download_latest_shipping_label_pdf_get) | **GET** /shipment/id/{id}/downloadLatestShippingLabelPdf | 
[**shipment_id_id_get**](ShipmentApi.md#shipment_id_id_get) | **GET** /shipment/id/{id} | 
[**shipment_id_id_put**](ShipmentApi.md#shipment_id_id_put) | **PUT** /shipment/id/{id} | 
[**shipment_post**](ShipmentApi.md#shipment_post) | **POST** /shipment | 



## shipment_count_get

> crate::models::AccountingTransactionCountGet200Response shipment_count_get(page, page_size, sort)


count shipment

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


## shipment_get

> crate::models::ShipmentGet200Response shipment_get(page, page_size, sort)


query shipment

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page** | Option<**i32**> |  |  |
**page_size** | Option<**i32**> |  |  |
**sort** | Option<**String**> |  |  |

### Return type

[**crate::models::ShipmentGet200Response**](_shipment_get_200_response.md)

### Authorization

[API token](../README.md#API token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## shipment_id_id_create_sales_invoice_post

> crate::models::IncomingGoodsIdIdCreateCreditNotePost200Response shipment_id_id_create_sales_invoice_post(id, body)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |
**body** | **serde_json::Value** |  | [required] |

### Return type

[**crate::models::IncomingGoodsIdIdCreateCreditNotePost200Response**](_incomingGoods_id__id__createCreditNote_post_200_response.md)

### Authorization

[API token](../README.md#API token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## shipment_id_id_create_shipping_label_pdf_post

> crate::models::ArticleIdIdUploadArticleImagePost200Response shipment_id_id_create_shipping_label_pdf_post(id, name, description)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |
**name** | **String** |  | [required] |
**description** | Option<**String**> |  |  |

### Return type

[**crate::models::ArticleIdIdUploadArticleImagePost200Response**](_article_id__id__uploadArticleImage_post_200_response.md)

### Authorization

[API token](../README.md#API token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## shipment_id_id_delete

> shipment_id_id_delete(id)


delete a shipment

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


## shipment_id_id_download_latest_delivery_note_pdf_get

> shipment_id_id_download_latest_delivery_note_pdf_get(id)


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


## shipment_id_id_download_latest_picking_list_pdf_get

> shipment_id_id_download_latest_picking_list_pdf_get(id)


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


## shipment_id_id_download_latest_shipping_label_pdf_get

> shipment_id_id_download_latest_shipping_label_pdf_get(id)


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


## shipment_id_id_get

> crate::models::Shipment shipment_id_id_get(id)


query shipment

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

[**crate::models::Shipment**](shipment.md)

### Authorization

[API token](../README.md#API token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## shipment_id_id_put

> crate::models::Shipment shipment_id_id_put(id, body)


update shipment

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |
**body** | [**Shipment**](Shipment.md) |  | [required] |

### Return type

[**crate::models::Shipment**](shipment.md)

### Authorization

[API token](../README.md#API token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## shipment_post

> crate::models::Shipment shipment_post(body)


create a shipment

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**Shipment**](Shipment.md) |  | [required] |

### Return type

[**crate::models::Shipment**](shipment.md)

### Authorization

[API token](../README.md#API token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

