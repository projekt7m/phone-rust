# \VoicemailApi

All URIs are relative to *https://yser.p7m.de/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_voicemail_qid**](VoicemailApi.md#get_voicemail_qid) | **GET** /voicemail/{qid} | Get the list of all voicemails of a queue
[**post_voicemail_qid**](VoicemailApi.md#post_voicemail_qid) | **POST** /voicemail/{qid} | Create a new voicemail message
[**put_voicemail_qid_vid**](VoicemailApi.md#put_voicemail_qid_vid) | **PUT** /voicemail/{qid}/{vid} | Update the message text of a voicemail



## get_voicemail_qid

> Vec<models::Voicemail> get_voicemail_qid(qid)
Get the list of all voicemails of a queue

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**qid** | **uuid::Uuid** | ID of the queue | [required] |

### Return type

[**Vec<models::Voicemail>**](Voicemail.md)

### Authorization

[oauth](../README.md#oauth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_voicemail_qid

> Vec<models::Voicemail> post_voicemail_qid(qid, new_voicemail)
Create a new voicemail message

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**qid** | **uuid::Uuid** | ID of the queue | [required] |
**new_voicemail** | [**NewVoicemail**](NewVoicemail.md) | The voicemail that should be created | [required] |

### Return type

[**Vec<models::Voicemail>**](Voicemail.md)

### Authorization

[oauth](../README.md#oauth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## put_voicemail_qid_vid

> models::Voicemail put_voicemail_qid_vid(qid, vid, voicemail_update)
Update the message text of a voicemail

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**qid** | **uuid::Uuid** | ID of the queue | [required] |
**vid** | **uuid::Uuid** | ID of the voicemail | [required] |
**voicemail_update** | [**VoicemailUpdate**](VoicemailUpdate.md) | The update to the voicemail message | [required] |

### Return type

[**models::Voicemail**](Voicemail.md)

### Authorization

[oauth](../README.md#oauth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

