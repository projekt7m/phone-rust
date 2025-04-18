# Queue

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**queue_id** | [**uuid::Uuid**](uuid::Uuid.md) | ID of this queue | 
**tenant_id** | [**uuid::Uuid**](uuid::Uuid.md) | The tenant this queue belongs to | 
**queue_token** | **String** | Legacy ID of this queue | 
**name** | **String** | Name descripting this queue | 
**timezone** | **String** | The time zone used for calculations regarding this queue (e.g. for opening hours) | 
**numbers** | [**Vec<models::QueueNumber>**](QueueNumber.md) | The numbers a caller can use to reach to this queue | 
**destinations** | [**Vec<models::QueueDestination>**](QueueDestination.md) | The phones that get called when there is a caller in this queue | 
**prio** | [**Vec<models::QueuePrio>**](QueuePrio.md) | Legacy data on the priorities (see 'numbers' now) | 
**last_change** | **String** | When this queue has been updated the last time | 
**holiday_region_id** | [**uuid::Uuid**](uuid::Uuid.md) | The holiday calender being used by this queue | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


