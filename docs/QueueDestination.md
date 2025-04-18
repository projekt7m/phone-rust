# QueueDestination

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**queue_destination_id** | [**uuid::Uuid**](uuid::Uuid.md) | ID of this destination | 
**tenant_id** | [**uuid::Uuid**](uuid::Uuid.md) | The tenant this destination belongs to | 
**prio_min** | **i32** | Minimum priority a call needs to get signalled on this destination | 
**prio_max** | **i32** | Maximum priority a call can have to get signalled on this destination | 
**destination** | **String** | Phone number where to ring (E.164 format including '+' prefix) | 
**description** | **String** | Name of this destination for representation on the UI | 
**last_change** | **String** | When this destination has changed the last time | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


