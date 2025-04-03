# \StatusApi

All URIs are relative to *https://yser.p7m.de/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**status**](StatusApi.md#status) | **GET** /status/{queue} | Get the current calls present in a queue



## status

> Vec<models::QueueCall> status(queue)
Get the current calls present in a queue

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**queue** | **String** | Queue token to get the status for (caution: **no UUID** here) | [required] |

### Return type

[**Vec<models::QueueCall>**](QueueCall.md)

### Authorization

[oauth](../README.md#oauth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

