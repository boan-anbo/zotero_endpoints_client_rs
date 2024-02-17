# \DefaultApi

All URIs are relative to *http://127.0.0.1:23119/endpoints*

Method | HTTP request | Description
------------- | ------------- | -------------
[**collection_add_to_current**](DefaultApi.md#collection_add_to_current) | **GET** /collection/addToCurrent | 
[**get_attachment_paths**](DefaultApi.md#get_attachment_paths) | **GET** /select/attachmentPaths | 
[**items_get**](DefaultApi.md#items_get) | **GET** /items | 
[**search_items_post**](DefaultApi.md#search_items_post) | **POST** /search/items | 



## collection_add_to_current

> crate::models::EndpointResponse collection_add_to_current(cite_key, uris)


Use citation keys or Zotero URIs to add items to the current selected collection.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**cite_key** | Option<**String**> |  |  |
**uris** | Option<**String**> |  |  |

### Return type

[**crate::models::EndpointResponse**](EndpointResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_attachment_paths

> crate::models::EndpointResponseWithStringArray get_attachment_paths()


Get the paths of the attachments of the selected items

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::EndpointResponseWithStringArray**](EndpointResponseWithStringArray.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## items_get

> items_get(cite_key)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**cite_key** | Option<**String**> | Citekey to look up items |  |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## search_items_post

> crate::models::EndpointResponse search_items_post(search_request, include_citation)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**search_request** | [**SearchRequest**](SearchRequest.md) | A set of conditions | [required] |
**include_citation** | Option<**bool**> |  |  |

### Return type

[**crate::models::EndpointResponse**](EndpointResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

