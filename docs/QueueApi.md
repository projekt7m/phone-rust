# \QueueApi

All URIs are relative to *https://yser.p7m.de/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**queue_announce_get**](QueueApi.md#queue_announce_get) | **GET** /queue/announce | 
[**queue_get**](QueueApi.md#queue_get) | **GET** /queue | 



## queue_announce_get

> Vec<crate::models::AnnounceSwitchtime> queue_announce_get()


This API endpoint is not completed and should not be used

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<crate::models::AnnounceSwitchtime>**](AnnounceSwitchtime.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## queue_get

> Vec<crate::models::Queue> queue_get()


returns a list of all waiting queues

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<crate::models::Queue>**](Queue.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

