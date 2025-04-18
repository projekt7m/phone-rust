# QueueNumber

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**queue_number_id** | [**uuid::Uuid**](uuid::Uuid.md) | The ID representing this queue number | 
**tenant_id** | [**uuid::Uuid**](uuid::Uuid.md) | The tenant this number belongs to | 
**number** | **String** | The phone number in E.164 format including a '+' prefix | 
**queue_id** | [**uuid::Uuid**](uuid::Uuid.md) | The queue this number belongs to | 
**prio** | **i32** | The priority calls on this number get assigned by default | 
**forced_caller_name** | **String** | Set this as the name of the caller when ringing the destinations (empty to not force a caller name) | 
**last_change** | **String** | When this number has been changed the last time | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


