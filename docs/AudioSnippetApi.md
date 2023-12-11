# \AudioSnippetApi

All URIs are relative to *https://yser.p7m.de/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**audio_snippet_get**](AudioSnippetApi.md#audio_snippet_get) | **GET** /audio/snippet | 
[**audio_snippet_post**](AudioSnippetApi.md#audio_snippet_post) | **POST** /audio/snippet | 



## audio_snippet_get

> crate::models::AudioSnippetData audio_snippet_get()


Get the list of audio snippets

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::AudioSnippetData**](AudioSnippetData.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## audio_snippet_post

> crate::models::AudioSnippet audio_snippet_post(new_audio_snippet)


Create a new audio snippet

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**new_audio_snippet** | [**NewAudioSnippet**](NewAudioSnippet.md) | the audio snippet to be created | [required] |

### Return type

[**crate::models::AudioSnippet**](AudioSnippet.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

