# \QueueApi

All URIs are relative to *https://yser.p7m.de/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**delete_queue_openings_id**](QueueApi.md#delete_queue_openings_id) | **DELETE** /queues/{qid}/openings/{qoid} | Delete a queue opening interval by its ID
[**get_queue_opening**](QueueApi.md#get_queue_opening) | **GET** /queues/{qid}/openings | Get the list of intervals where the waiting queue is opened
[**get_queue_openings_id**](QueueApi.md#get_queue_openings_id) | **GET** /queues/{qid}/openings/{qoid} | Get an opening interval by its ID
[**get_queues**](QueueApi.md#get_queues) | **GET** /queues | Get the list of all waiting queues
[**post_queue_openings**](QueueApi.md#post_queue_openings) | **POST** /queues/{qid}/openings | Create a new queue opening interval
[**put_queue_openings_id**](QueueApi.md#put_queue_openings_id) | **PUT** /queues/{qid}/openings/{qoid} | Update an existing queue opening interval
[**queue**](QueueApi.md#queue) | **GET** /queue | Get the list of all waiting queues



## delete_queue_openings_id

> delete_queue_openings_id(qid, qoid)
Delete a queue opening interval by its ID

Delete a queue opening interval by its ID

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**qid** | **uuid::Uuid** | ID of the queue | [required] |
**qoid** | **uuid::Uuid** | ID of the queue opening interval | [required] |

### Return type

 (empty response body)

### Authorization

[oauth](../README.md#oauth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_queue_opening

> models::QueueOpeningData get_queue_opening(qid)
Get the list of intervals where the waiting queue is opened

Get the list of intervals where the waiting queue is opened

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**qid** | **uuid::Uuid** | ID of the waiting queue | [required] |

### Return type

[**models::QueueOpeningData**](QueueOpeningData.md)

### Authorization

[oauth](../README.md#oauth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_queue_openings_id

> models::QueueOpening get_queue_openings_id(qid, qoid)
Get an opening interval by its ID

Get an opening interval by its ID

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**qid** | **uuid::Uuid** | ID of the queue | [required] |
**qoid** | **uuid::Uuid** | ID of the queue opening interval | [required] |

### Return type

[**models::QueueOpening**](QueueOpening.md)

### Authorization

[oauth](../README.md#oauth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_queues

> Vec<models::Queue> get_queues()
Get the list of all waiting queues

Get the list of all waiting queues

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<models::Queue>**](Queue.md)

### Authorization

[oauth](../README.md#oauth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_queue_openings

> models::QueueOpening post_queue_openings(qid, new_queue_opening)
Create a new queue opening interval

Create a new queue opening interval

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**qid** | **uuid::Uuid** | ID of the queue | [required] |
**new_queue_opening** | [**NewQueueOpening**](NewQueueOpening.md) | Definition of the new opening interval | [required] |

### Return type

[**models::QueueOpening**](QueueOpening.md)

### Authorization

[oauth](../README.md#oauth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## put_queue_openings_id

> models::QueueOpening put_queue_openings_id(qid, qoid, new_queue_opening)
Update an existing queue opening interval

Update an existing queue opening interval

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**qid** | **uuid::Uuid** | ID of the queue | [required] |
**qoid** | **uuid::Uuid** | ID of the queue opening interval | [required] |
**new_queue_opening** | [**NewQueueOpening**](NewQueueOpening.md) | The updated queue opening interval | [required] |

### Return type

[**models::QueueOpening**](QueueOpening.md)

### Authorization

[oauth](../README.md#oauth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## queue

> Vec<models::LegacyQueue> queue()
Get the list of all waiting queues

Get the list of all waiting queues

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<models::LegacyQueue>**](LegacyQueue.md)

### Authorization

[oauth](../README.md#oauth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

