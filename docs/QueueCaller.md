# QueueCaller

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**queue_caller_id** | [**uuid::Uuid**](uuid::Uuid.md) | ID of this caller in the queue | 
**tenant_id** | [**uuid::Uuid**](uuid::Uuid.md) | The tenant this call belongs to | 
**queue_id** | [**uuid::Uuid**](uuid::Uuid.md) | The queue this call is on | 
**queue_number_id** | [**uuid::Uuid**](uuid::Uuid.md) | The number of the queue that has been called | 
**caller_number** | **String** | CLIP of the call if known | 
**prio** | **i32** | Priority in the queue (higher number is higher priority) | 
**call_state** | [**models::CallState**](CallState.md) | The state of this call | 
**ring_until** | Option<**String**> | If set, this call should ring until this timestamp | [optional]
**start_time** | **String** | When the incoming call started | 
**last_change** | **String** | When this structure has been updated the last time | 
**next_check** | **String** | When the REST caller should check the next time for changed states | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


