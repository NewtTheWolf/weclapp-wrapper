# \UserApi

All URIs are relative to *https://www.weclapp.com/webapp/api/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**user_count_get**](UserApi.md#user_count_get) | **GET** /user/count | 
[**user_current_user_get**](UserApi.md#user_current_user_get) | **GET** /user/currentUser | 
[**user_get**](UserApi.md#user_get) | **GET** /user | 
[**user_id_id_delete**](UserApi.md#user_id_id_delete) | **DELETE** /user/id/{id} | 
[**user_id_id_get**](UserApi.md#user_id_id_get) | **GET** /user/id/{id} | 
[**user_id_id_put**](UserApi.md#user_id_id_put) | **PUT** /user/id/{id} | 
[**user_id_id_user_image_get**](UserApi.md#user_id_id_user_image_get) | **GET** /user/id/{id}/userImage | 
[**user_id_id_user_image_thumbnail_get**](UserApi.md#user_id_id_user_image_thumbnail_get) | **GET** /user/id/{id}/userImageThumbnail | 
[**user_post**](UserApi.md#user_post) | **POST** /user | 



## user_count_get

> crate::models::AccountingTransactionCountGet200Response user_count_get(page, page_size, sort)


count user

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


## user_current_user_get

> crate::models::UserCurrentUserGet200Response user_current_user_get()


### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::UserCurrentUserGet200Response**](_user_currentUser_get_200_response.md)

### Authorization

[API token](../README.md#API token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## user_get

> crate::models::UserGet200Response user_get(page, page_size, sort)


query user

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page** | Option<**i32**> |  |  |
**page_size** | Option<**i32**> |  |  |
**sort** | Option<**String**> |  |  |

### Return type

[**crate::models::UserGet200Response**](_user_get_200_response.md)

### Authorization

[API token](../README.md#API token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## user_id_id_delete

> user_id_id_delete(id)


delete a user

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


## user_id_id_get

> crate::models::User user_id_id_get(id)


query user

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

[**crate::models::User**](user.md)

### Authorization

[API token](../README.md#API token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## user_id_id_put

> crate::models::User user_id_id_put(id, body)


update user

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |
**body** | [**User**](User.md) |  | [required] |

### Return type

[**crate::models::User**](user.md)

### Authorization

[API token](../README.md#API token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## user_id_id_user_image_get

> user_id_id_user_image_get(id, scale_width, scale_height, image_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |
**scale_width** | Option<**i32**> |  |  |
**scale_height** | Option<**i32**> |  |  |
**image_id** | Option<**String**> |  |  |

### Return type

 (empty response body)

### Authorization

[API token](../README.md#API token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: image/jpeg, image/png, application/pdf, */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## user_id_id_user_image_thumbnail_get

> user_id_id_user_image_thumbnail_get(id, scale_width, scale_height, image_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |
**scale_width** | Option<**i32**> |  |  |
**scale_height** | Option<**i32**> |  |  |
**image_id** | Option<**String**> |  |  |

### Return type

 (empty response body)

### Authorization

[API token](../README.md#API token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: image/jpeg, image/png, application/pdf, */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## user_post

> crate::models::User user_post(body)


create a user

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**User**](User.md) |  | [required] |

### Return type

[**crate::models::User**](user.md)

### Authorization

[API token](../README.md#API token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

