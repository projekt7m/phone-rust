# \CallApi

All URIs are relative to *https://yser.p7m.de/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_call_alarm**](CallApi.md#get_call_alarm) | **GET** /call/alarm | Get the list of all alarm calls
[**get_call_alarm_by_callee_e164_unsecure**](CallApi.md#get_call_alarm_by_callee_e164_unsecure) | **GET** /call/alarm/by-callee/{e164} | Get the alarm calls for a given telephone number
[**post_call_alarm**](CallApi.md#post_call_alarm) | **POST** /call/alarm | Create a new alarm call
[**post_call_code**](CallApi.md#post_call_code) | **POST** /call/code | RPC to trigger an outgoing call to send a PIN code to a user
[**put_call_alarm_id**](CallApi.md#put_call_alarm_id) | **PUT** /call/alarm/{id} | Update the state of an existing alarm call



## get_call_alarm

> crate::models::AlarmCallData get_call_alarm()
Get the list of all alarm calls

Get the list of all alarm calls

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::AlarmCallData**](AlarmCallData.md)

### Authorization

[oauth](../README.md#oauth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_call_alarm_by_callee_e164_unsecure

> crate::models::AlarmCallData get_call_alarm_by_callee_e164_unsecure(e164)
Get the alarm calls for a given telephone number

Get the alarm calls for a given telephone number

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**e164** | **String** | Telephone number in E.164 format | [required] |

### Return type

[**crate::models::AlarmCallData**](AlarmCallData.md)

### Authorization

[oauth](../README.md#oauth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_call_alarm

> crate::models::AlarmCall post_call_alarm(new_alarm_call)
Create a new alarm call

Create a new alarm call

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**new_alarm_call** | [**NewAlarmCall**](NewAlarmCall.md) | The alarm call to be created | [required] |

### Return type

[**crate::models::AlarmCall**](AlarmCall.md)

### Authorization

[oauth](../README.md#oauth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_call_code

> crate::models::CodeCallResult post_call_code(code_call_request)
RPC to trigger an outgoing call to send a PIN code to a user

RPC to trigger an outgoing call to send a PIN code to a user

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**code_call_request** | [**CodeCallRequest**](CodeCallRequest.md) | Parameters to define the call that should be made | [required] |

### Return type

[**crate::models::CodeCallResult**](CodeCallResult.md)

### Authorization

[oauth](../README.md#oauth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## put_call_alarm_id

> crate::models::AlarmCall put_call_alarm_id(id, alarm_call_update)
Update the state of an existing alarm call

Update the state of an existing alarm call

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | ID of the alarm call | [required] |
**alarm_call_update** | [**AlarmCallUpdate**](AlarmCallUpdate.md) | The status update to the alarm call | [required] |

### Return type

[**crate::models::AlarmCall**](AlarmCall.md)

### Authorization

[oauth](../README.md#oauth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

