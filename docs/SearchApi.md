# \SearchApi

All URIs are relative to *http://127.0.0.1:23119/endpoints*

Method | HTTP request | Description
------------- | ------------- | -------------
[**search_items_post**](SearchApi.md#search_items_post) | **POST** /search/items | 



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

