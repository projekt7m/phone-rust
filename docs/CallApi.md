# \CallApi

All URIs are relative to *https://yser.p7m.de/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**call_alarm_all_of_id_delete**](CallApi.md#call_alarm_all_of_id_delete) | **DELETE** /call/alarm/all-of/{id} | 
[**call_alarm_get**](CallApi.md#call_alarm_get) | **GET** /call/alarm | 
[**call_alarm_id_delete**](CallApi.md#call_alarm_id_delete) | **DELETE** /call/alarm/{id} | 
[**call_alarm_id_get**](CallApi.md#call_alarm_id_get) | **GET** /call/alarm/{id} | 
[**call_alarm_id_put**](CallApi.md#call_alarm_id_put) | **PUT** /call/alarm/{id} | 
[**call_alarm_post**](CallApi.md#call_alarm_post) | **POST** /call/alarm | 
[**call_code_post**](CallApi.md#call_code_post) | **POST** /call/code | 



## call_alarm_all_of_id_delete

> call_alarm_all_of_id_delete(id)


Canceles all alarm calls of an alarm

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | alarm id | [required] |

### Return type

 (empty response body)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## call_alarm_get

> crate::models::AlarmCallData call_alarm_get()


Get the list of alarm calls

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::AlarmCallData**](AlarmCallData.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## call_alarm_id_delete

> call_alarm_id_delete(id)


Cancel a single alarm call

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | alarm call id | [required] |

### Return type

 (empty response body)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## call_alarm_id_get

> crate::models::AlarmCall call_alarm_id_get(id)


Returns the given alarm call

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | alarm call id | [required] |

### Return type

[**crate::models::AlarmCall**](AlarmCall.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## call_alarm_id_put

> crate::models::AlarmCall call_alarm_id_put(id, alarm_call_update)


Updates the state of an alarm call

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | alarm call id | [required] |
**alarm_call_update** | [**AlarmCallUpdate**](AlarmCallUpdate.md) | the alarm call update | [required] |

### Return type

[**crate::models::AlarmCall**](AlarmCall.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## call_alarm_post

> crate::models::AlarmCall call_alarm_post(new_alarm_call)


creates a new alarm call

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**new_alarm_call** | [**NewAlarmCall**](NewAlarmCall.md) | the alarm call to be created | [required] |

### Return type

[**crate::models::AlarmCall**](AlarmCall.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## call_code_post

> crate::models::CodeCallResult call_code_post(code_call_request)


This is a RPC call used to trigger an outgoing call to send a PIN code to a user

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**code_call_request** | Option<[**CodeCallRequest**](CodeCallRequest.md)> | Parameters to define the call the should be made |  |

### Return type

[**crate::models::CodeCallResult**](CodeCallResult.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

