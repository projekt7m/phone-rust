# \StatusApi

All URIs are relative to *https://yser.p7m.de/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**status_queue_get**](StatusApi.md#status_queue_get) | **GET** /status/{queue} | 



## status_queue_get

> Vec<crate::models::QueueCall> status_queue_get(queue)


Request the current state of a waiting queue

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**queue** | **String** | queue token to get the status for (caution: no UUID here) | [required] |

### Return type

[**Vec<crate::models::QueueCall>**](QueueCall.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

