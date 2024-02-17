# \CollectionApi

All URIs are relative to *http://127.0.0.1:23119/endpoints*

Method | HTTP request | Description
------------- | ------------- | -------------
[**collection_add_to_current**](CollectionApi.md#collection_add_to_current) | **GET** /collection/addToCurrent | 



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

