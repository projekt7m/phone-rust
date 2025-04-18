# NewQueueCaller

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**queue_number_id** | [**uuid::Uuid**](uuid::Uuid.md) | The number to caller called | 
**caller_number** | **String** | CLIP of the number, if known (empty else) | 
**prio** | **i32** | Priority of the call (higher number is higher priority) | 
**call_state** | [**models::CallState**](CallState.md) | The state this call is in, for NewQueueCaller this probably should be 'INTRO' | 
**start_time** | **String** | The timestamp when the call started | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


