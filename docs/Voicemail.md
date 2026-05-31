# Voicemail

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**voicemail_id** | [**uuid::Uuid**](uuid::Uuid.md) |  | 
**tenant_id** | [**uuid::Uuid**](uuid::Uuid.md) |  | 
**queue_id** | [**uuid::Uuid**](uuid::Uuid.md) |  | 
**caller** | **String** |  | 
**caller_phone_number** | Option<[**models::PhoneNumber**](PhoneNumber.md)> |  | [optional]
**topic** | **String** |  | 
**message** | **String** |  | 
**done_at** | Option<**String**> |  | [optional]
**done_by** | Option<[**uuid::Uuid**](uuid::Uuid.md)> |  | [optional]
**language** | **String** |  | 
**audio_filename** | **String** |  | 
**signed_url** | **String** |  | 
**url_valid_to** | Option<**String**> |  | [optional]
**timestamp** | **String** |  | 
**last_change** | **String** |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


