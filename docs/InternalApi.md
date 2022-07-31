# \InternalApi

All URIs are relative to *https://yser.p7m.de/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**alarm_trigger_post**](InternalApi.md#alarm_trigger_post) | **POST** /alarm/trigger | 
[**block_check_pending_get**](InternalApi.md#block_check_pending_get) | **GET** /block/check_pending | 



## alarm_trigger_post

> crate::models::TriggerResult alarm_trigger_post(trigger)


This is used internally only and provides no externally usable service

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**trigger** | Option<[**Trigger**](Trigger.md)> | not documented |  |

### Return type

[**crate::models::TriggerResult**](TriggerResult.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## block_check_pending_get

> String block_check_pending_get()


This is used internally only and provides no externally usable service

### Parameters

This endpoint does not need any parameter.

### Return type

**String**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

