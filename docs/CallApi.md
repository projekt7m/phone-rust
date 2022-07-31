# \CallApi

All URIs are relative to *https://yser.p7m.de/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**call_code_post**](CallApi.md#call_code_post) | **POST** /call/code | 



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

