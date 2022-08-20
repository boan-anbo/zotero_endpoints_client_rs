# \DefaultApi

All URIs are relative to *http://127.0.0.1:23119/endpoints*

Method | HTTP request | Description
------------- | ------------- | -------------
[**collection_add_to_current_get**](DefaultApi.md#collection_add_to_current_get) | **GET** /collection/addToCurrent | 
[**items_get**](DefaultApi.md#items_get) | **GET** /items | 
[**search_items_post**](DefaultApi.md#search_items_post) | **POST** /search/items | 



## collection_add_to_current_get

> Vec<crate::models::EndpointResponseInner> collection_add_to_current_get(cite_key, uris)


Use citation keys or Zotero URIs to add items to the current selected collection.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**cite_key** | Option<**String**> |  |  |
**uris** | Option<**String**> |  |  |

### Return type

[**Vec<crate::models::EndpointResponseInner>**](EndpointResponse_inner.md)

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

> Vec<crate::models::EndpointResponseInner> search_items_post(search_request, include_citation)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**search_request** | [**SearchRequest**](SearchRequest.md) | A set of conditions | [required] |
**include_citation** | Option<**bool**> |  |  |

### Return type

[**Vec<crate::models::EndpointResponseInner>**](EndpointResponse_inner.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

