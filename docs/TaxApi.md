# \TaxApi

All URIs are relative to *https://www.weclapp.com/webapp/api/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**tax_configure_purchase_taxes_post**](TaxApi.md#tax_configure_purchase_taxes_post) | **POST** /tax/configurePurchaseTaxes | 
[**tax_configure_sales_taxes_post**](TaxApi.md#tax_configure_sales_taxes_post) | **POST** /tax/configureSalesTaxes | 
[**tax_count_get**](TaxApi.md#tax_count_get) | **GET** /tax/count | 
[**tax_find_purchase_tax_get**](TaxApi.md#tax_find_purchase_tax_get) | **GET** /tax/findPurchaseTax | 
[**tax_find_sales_tax_get**](TaxApi.md#tax_find_sales_tax_get) | **GET** /tax/findSalesTax | 
[**tax_get**](TaxApi.md#tax_get) | **GET** /tax | 
[**tax_id_id_delete**](TaxApi.md#tax_id_id_delete) | **DELETE** /tax/id/{id} | 
[**tax_id_id_get**](TaxApi.md#tax_id_id_get) | **GET** /tax/id/{id} | 
[**tax_id_id_put**](TaxApi.md#tax_id_id_put) | **PUT** /tax/id/{id} | 
[**tax_post**](TaxApi.md#tax_post) | **POST** /tax | 
[**tax_reset_system_taxes_post**](TaxApi.md#tax_reset_system_taxes_post) | **POST** /tax/resetSystemTaxes | 



## tax_configure_purchase_taxes_post

> crate::models::ArticleIdIdUploadArticleImagePost200Response tax_configure_purchase_taxes_post(body)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**TaxConfigurePurchaseTaxesPostRequest**](TaxConfigurePurchaseTaxesPostRequest.md) |  | [required] |

### Return type

[**crate::models::ArticleIdIdUploadArticleImagePost200Response**](_article_id__id__uploadArticleImage_post_200_response.md)

### Authorization

[API token](../README.md#API token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## tax_configure_sales_taxes_post

> crate::models::ArticleIdIdUploadArticleImagePost200Response tax_configure_sales_taxes_post(body)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**TaxConfigureSalesTaxesPostRequest**](TaxConfigureSalesTaxesPostRequest.md) |  | [required] |

### Return type

[**crate::models::ArticleIdIdUploadArticleImagePost200Response**](_article_id__id__uploadArticleImage_post_200_response.md)

### Authorization

[API token](../README.md#API token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## tax_count_get

> crate::models::AccountingTransactionCountGet200Response tax_count_get(page, page_size, sort)


count tax

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


## tax_find_purchase_tax_get

> crate::models::TaxFindPurchaseTaxGet200Response tax_find_purchase_tax_get(recipient_country_code, dispatch_country_code, tax_rate_type, party_type, debtor_creditor_code_id, product_code_id, date)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**recipient_country_code** | **String** |  | [required] |
**dispatch_country_code** | Option<**String**> |  |  |
**tax_rate_type** | Option<**String**> |  |  |
**party_type** | Option<**String**> |  |  |
**debtor_creditor_code_id** | Option<**String**> |  |  |
**product_code_id** | Option<**String**> |  |  |
**date** | Option<**i32**> |  |  |

### Return type

[**crate::models::TaxFindPurchaseTaxGet200Response**](_tax_findPurchaseTax_get_200_response.md)

### Authorization

[API token](../README.md#API token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## tax_find_sales_tax_get

> crate::models::TaxFindPurchaseTaxGet200Response tax_find_sales_tax_get(dispatch_country_code, recipient_country_code, tax_rate_type, party_type, debtor_creditor_code_id, product_code_id, valid_vat_id, date)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**dispatch_country_code** | **String** |  | [required] |
**recipient_country_code** | Option<**String**> |  |  |
**tax_rate_type** | Option<**String**> |  |  |
**party_type** | Option<**String**> |  |  |
**debtor_creditor_code_id** | Option<**String**> |  |  |
**product_code_id** | Option<**String**> |  |  |
**valid_vat_id** | Option<**bool**> |  |  |
**date** | Option<**i32**> |  |  |

### Return type

[**crate::models::TaxFindPurchaseTaxGet200Response**](_tax_findPurchaseTax_get_200_response.md)

### Authorization

[API token](../README.md#API token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## tax_get

> crate::models::TaxGet200Response tax_get(page, page_size, sort)


query tax

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page** | Option<**i32**> |  |  |
**page_size** | Option<**i32**> |  |  |
**sort** | Option<**String**> |  |  |

### Return type

[**crate::models::TaxGet200Response**](_tax_get_200_response.md)

### Authorization

[API token](../README.md#API token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## tax_id_id_delete

> tax_id_id_delete(id)


delete a tax

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


## tax_id_id_get

> crate::models::Tax tax_id_id_get(id)


query tax

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

[**crate::models::Tax**](tax.md)

### Authorization

[API token](../README.md#API token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## tax_id_id_put

> crate::models::Tax tax_id_id_put(id, body)


update tax

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |
**body** | [**Tax**](Tax.md) |  | [required] |

### Return type

[**crate::models::Tax**](tax.md)

### Authorization

[API token](../README.md#API token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## tax_post

> crate::models::Tax tax_post(body)


create a tax

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**Tax**](Tax.md) |  | [required] |

### Return type

[**crate::models::Tax**](tax.md)

### Authorization

[API token](../README.md#API token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## tax_reset_system_taxes_post

> crate::models::ArticleIdIdUploadArticleImagePost200Response tax_reset_system_taxes_post(body)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**TaxResetSystemTaxesPostRequest**](TaxResetSystemTaxesPostRequest.md) |  | [required] |

### Return type

[**crate::models::ArticleIdIdUploadArticleImagePost200Response**](_article_id__id__uploadArticleImage_post_200_response.md)

### Authorization

[API token](../README.md#API token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

