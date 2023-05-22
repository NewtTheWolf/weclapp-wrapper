# \IncomingGoodsApi

All URIs are relative to *https://www.weclapp.com/webapp/api/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**incoming_goods_count_get**](IncomingGoodsApi.md#incoming_goods_count_get) | **GET** /incomingGoods/count | 
[**incoming_goods_get**](IncomingGoodsApi.md#incoming_goods_get) | **GET** /incomingGoods | 
[**incoming_goods_id_id_add_purchase_orders_post**](IncomingGoodsApi.md#incoming_goods_id_id_add_purchase_orders_post) | **POST** /incomingGoods/id/{id}/addPurchaseOrders | 
[**incoming_goods_id_id_create_compensation_shipment_post**](IncomingGoodsApi.md#incoming_goods_id_id_create_compensation_shipment_post) | **POST** /incomingGoods/id/{id}/createCompensationShipment | 
[**incoming_goods_id_id_create_credit_note_post**](IncomingGoodsApi.md#incoming_goods_id_id_create_credit_note_post) | **POST** /incomingGoods/id/{id}/createCreditNote | 
[**incoming_goods_id_id_create_supplier_return_post**](IncomingGoodsApi.md#incoming_goods_id_id_create_supplier_return_post) | **POST** /incomingGoods/id/{id}/createSupplierReturn | 
[**incoming_goods_id_id_delete**](IncomingGoodsApi.md#incoming_goods_id_id_delete) | **DELETE** /incomingGoods/id/{id} | 
[**incoming_goods_id_id_get**](IncomingGoodsApi.md#incoming_goods_id_id_get) | **GET** /incomingGoods/id/{id} | 
[**incoming_goods_id_id_put**](IncomingGoodsApi.md#incoming_goods_id_id_put) | **PUT** /incomingGoods/id/{id} | 
[**incoming_goods_post**](IncomingGoodsApi.md#incoming_goods_post) | **POST** /incomingGoods | 



## incoming_goods_count_get

> crate::models::AccountingTransactionCountGet200Response incoming_goods_count_get(page, page_size, sort)


count incomingGoods

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


## incoming_goods_get

> crate::models::IncomingGoodsGet200Response incoming_goods_get(page, page_size, sort)


query incomingGoods

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page** | Option<**i32**> |  |  |
**page_size** | Option<**i32**> |  |  |
**sort** | Option<**String**> |  |  |

### Return type

[**crate::models::IncomingGoodsGet200Response**](_incomingGoods_get_200_response.md)

### Authorization

[API token](../README.md#API token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## incoming_goods_id_id_add_purchase_orders_post

> crate::models::IncomingGoodsIdIdAddPurchaseOrdersPost200Response incoming_goods_id_id_add_purchase_orders_post(id, body)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |
**body** | [**IncomingGoodsIdIdAddPurchaseOrdersPostRequest**](IncomingGoodsIdIdAddPurchaseOrdersPostRequest.md) |  | [required] |

### Return type

[**crate::models::IncomingGoodsIdIdAddPurchaseOrdersPost200Response**](_incomingGoods_id__id__addPurchaseOrders_post_200_response.md)

### Authorization

[API token](../README.md#API token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## incoming_goods_id_id_create_compensation_shipment_post

> crate::models::IncomingGoodsIdIdCreateCompensationShipmentPost200Response incoming_goods_id_id_create_compensation_shipment_post(id, body)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |
**body** | **serde_json::Value** |  | [required] |

### Return type

[**crate::models::IncomingGoodsIdIdCreateCompensationShipmentPost200Response**](_incomingGoods_id__id__createCompensationShipment_post_200_response.md)

### Authorization

[API token](../README.md#API token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## incoming_goods_id_id_create_credit_note_post

> crate::models::IncomingGoodsIdIdCreateCreditNotePost200Response incoming_goods_id_id_create_credit_note_post(id, body)


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


## incoming_goods_id_id_create_supplier_return_post

> crate::models::IncomingGoodsIdIdCreateCompensationShipmentPost200Response incoming_goods_id_id_create_supplier_return_post(id, body)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |
**body** | [**IncomingGoodsIdIdCreateSupplierReturnPostRequest**](IncomingGoodsIdIdCreateSupplierReturnPostRequest.md) |  | [required] |

### Return type

[**crate::models::IncomingGoodsIdIdCreateCompensationShipmentPost200Response**](_incomingGoods_id__id__createCompensationShipment_post_200_response.md)

### Authorization

[API token](../README.md#API token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## incoming_goods_id_id_delete

> incoming_goods_id_id_delete(id)


delete a incomingGoods

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


## incoming_goods_id_id_get

> crate::models::IncomingGoods incoming_goods_id_id_get(id)


query incomingGoods

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

[**crate::models::IncomingGoods**](incomingGoods.md)

### Authorization

[API token](../README.md#API token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## incoming_goods_id_id_put

> crate::models::IncomingGoods incoming_goods_id_id_put(id, body)


update incomingGoods

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |
**body** | [**IncomingGoods**](IncomingGoods.md) |  | [required] |

### Return type

[**crate::models::IncomingGoods**](incomingGoods.md)

### Authorization

[API token](../README.md#API token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## incoming_goods_post

> crate::models::IncomingGoods incoming_goods_post(body)


create a incomingGoods

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**IncomingGoods**](IncomingGoods.md) |  | [required] |

### Return type

[**crate::models::IncomingGoods**](incomingGoods.md)

### Authorization

[API token](../README.md#API token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

