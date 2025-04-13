# \QueueApi

All URIs are relative to *https://yser.p7m.de/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**delete_queue_openings_id**](QueueApi.md#delete_queue_openings_id) | **DELETE** /queues/{qid}/openings/{qoid} | Delete a queue opening interval by its ID
[**delete_queue_queue_id_callers_queue_caller_id**](QueueApi.md#delete_queue_queue_id_callers_queue_caller_id) | **DELETE** /queues/{qid}/callers/{qcid} | Remove a caller from a queue
[**delete_queues_queue_id_numbers_queue_number_id**](QueueApi.md#delete_queues_queue_id_numbers_queue_number_id) | **DELETE** /queues/{qid}/numbers/{qnid} | Delete a number from a queue
[**get_queue_opening**](QueueApi.md#get_queue_opening) | **GET** /queues/{qid}/openings | Get the list of intervals where the waiting queue is opened
[**get_queue_openings_id**](QueueApi.md#get_queue_openings_id) | **GET** /queues/{qid}/openings/{qoid} | Get an opening interval by its ID
[**get_queue_queue_id_callers**](QueueApi.md#get_queue_queue_id_callers) | **GET** /queues/{qid}/callers | Get the list of callers in a waiting queue
[**get_queues**](QueueApi.md#get_queues) | **GET** /queues | Get the list of all waiting queues
[**get_queues_by_number_phone_number**](QueueApi.md#get_queues_by_number_phone_number) | **GET** /queues/bynumber/{phone_number} | Find a queue by its number
[**get_queues_queue_id**](QueueApi.md#get_queues_queue_id) | **GET** /queues/{qid} | Get a single queue by its ID
[**get_queues_queue_id_callers_queue_caller_id**](QueueApi.md#get_queues_queue_id_callers_queue_caller_id) | **GET** /queues/{qid}/callers/{qcid} | Get an invidivual caller in a queue
[**get_queues_queue_id_queue_number_id**](QueueApi.md#get_queues_queue_id_queue_number_id) | **GET** /queues/{qid}/numbers/{qnid} | Get a single queue number
[**post_queue_openings**](QueueApi.md#post_queue_openings) | **POST** /queues/{qid}/openings | Create a new queue opening interval
[**post_queues_queue_id_callers**](QueueApi.md#post_queues_queue_id_callers) | **POST** /queues/{qid}/callers | Add a caller to a waiting queue
[**post_queues_queue_id_number**](QueueApi.md#post_queues_queue_id_number) | **POST** /queues/{qid}/numbers | Add a number to a queue
[**put_queue_openings_id**](QueueApi.md#put_queue_openings_id) | **PUT** /queues/{qid}/openings/{qoid} | Update an existing queue opening interval
[**put_queue_queue_id_callers_queue_caller_id**](QueueApi.md#put_queue_queue_id_callers_queue_caller_id) | **PUT** /queues/{qid}/callers/{qcid} | Update or just keep-alive for a queue caller
[**put_queues_queue_id_numbers_queue_number_id**](QueueApi.md#put_queues_queue_id_numbers_queue_number_id) | **PUT** /queues/{qid}/numbers/{qnid} | Update a number configuration of a queue
[**queue**](QueueApi.md#queue) | **GET** /queue | Get the list of all waiting queues



## delete_queue_openings_id

> delete_queue_openings_id(qid, qoid)
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


## delete_queue_queue_id_callers_queue_caller_id

> delete_queue_queue_id_callers_queue_caller_id(qid, qcid)
Remove a caller from a queue

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**qid** | **uuid::Uuid** | ID of the queue | [required] |
**qcid** | **uuid::Uuid** | ID of the queue caller | [required] |

### Return type

 (empty response body)

### Authorization

[oauth](../README.md#oauth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_queues_queue_id_numbers_queue_number_id

> delete_queues_queue_id_numbers_queue_number_id(qid, qnid)
Delete a number from a queue

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**qid** | **uuid::Uuid** | ID of the queue | [required] |
**qnid** | **uuid::Uuid** | ID of the queue number | [required] |

### Return type

 (empty response body)

### Authorization

[oauth](../README.md#oauth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_queue_opening

> models::ListWrapperQueueOpening get_queue_opening(qid)
Get the list of intervals where the waiting queue is opened

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**qid** | **uuid::Uuid** | ID of the waiting queue | [required] |

### Return type

[**models::ListWrapperQueueOpening**](ListWrapper_QueueOpening.md)

### Authorization

[oauth](../README.md#oauth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_queue_openings_id

> models::QueueOpening get_queue_openings_id(qid, qoid)
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


## get_queue_queue_id_callers

> models::ListWrapperQueueCaller get_queue_queue_id_callers(qid)
Get the list of callers in a waiting queue

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**qid** | **uuid::Uuid** | ID of the waiting queue | [required] |

### Return type

[**models::ListWrapperQueueCaller**](ListWrapper_QueueCaller.md)

### Authorization

[oauth](../README.md#oauth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_queues

> Vec<models::Queue> get_queues()
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


## get_queues_by_number_phone_number

> models::Queue get_queues_by_number_phone_number(phone_number)
Find a queue by its number

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**phone_number** | **String** | Phone number to get the queue for | [required] |

### Return type

[**models::Queue**](Queue.md)

### Authorization

[oauth](../README.md#oauth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_queues_queue_id

> models::Queue get_queues_queue_id(qid)
Get a single queue by its ID

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**qid** | **uuid::Uuid** | ID of the queue | [required] |

### Return type

[**models::Queue**](Queue.md)

### Authorization

[oauth](../README.md#oauth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_queues_queue_id_callers_queue_caller_id

> models::QueueCaller get_queues_queue_id_callers_queue_caller_id(qid, qcid)
Get an invidivual caller in a queue

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**qid** | **uuid::Uuid** | ID of the queue | [required] |
**qcid** | **uuid::Uuid** | ID of the queue caller | [required] |

### Return type

[**models::QueueCaller**](QueueCaller.md)

### Authorization

[oauth](../README.md#oauth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_queues_queue_id_queue_number_id

> models::QueueNumber get_queues_queue_id_queue_number_id(qid, qnid)
Get a single queue number

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**qid** | **uuid::Uuid** | ID of the queue | [required] |
**qnid** | **uuid::Uuid** | ID of the queue number | [required] |

### Return type

[**models::QueueNumber**](QueueNumber.md)

### Authorization

[oauth](../README.md#oauth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_queue_openings

> models::QueueOpening post_queue_openings(qid, new_queue_opening)
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


## post_queues_queue_id_callers

> models::QueueCaller post_queues_queue_id_callers(qid, new_queue_caller)
Add a caller to a waiting queue

This API endpoint is only available with special privileges.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**qid** | **uuid::Uuid** | ID of the queue | [required] |
**new_queue_caller** | [**NewQueueCaller**](NewQueueCaller.md) | Definition of the new call | [required] |

### Return type

[**models::QueueCaller**](QueueCaller.md)

### Authorization

[oauth](../README.md#oauth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_queues_queue_id_number

> models::QueueNumber post_queues_queue_id_number(qid, new_queue_number)
Add a number to a queue

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**qid** | **uuid::Uuid** | ID of the queue | [required] |
**new_queue_number** | [**NewQueueNumber**](NewQueueNumber.md) | The number to add | [required] |

### Return type

[**models::QueueNumber**](QueueNumber.md)

### Authorization

[oauth](../README.md#oauth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## put_queue_openings_id

> models::QueueOpening put_queue_openings_id(qid, qoid, new_queue_opening)
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


## put_queue_queue_id_callers_queue_caller_id

> models::QueueCaller put_queue_queue_id_callers_queue_caller_id(qid, qcid, queue_caller_update)
Update or just keep-alive for a queue caller

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**qid** | **uuid::Uuid** | ID of the queue | [required] |
**qcid** | **uuid::Uuid** | ID of the caller | [required] |
**queue_caller_update** | [**QueueCallerUpdate**](QueueCallerUpdate.md) | The update to the queue caller | [required] |

### Return type

[**models::QueueCaller**](QueueCaller.md)

### Authorization

[oauth](../README.md#oauth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## put_queues_queue_id_numbers_queue_number_id

> models::QueueNumber put_queues_queue_id_numbers_queue_number_id(qid, qnid, new_queue_number)
Update a number configuration of a queue

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**qid** | **uuid::Uuid** | ID of the queue | [required] |
**qnid** | **uuid::Uuid** | ID of the queue number | [required] |
**new_queue_number** | [**NewQueueNumber**](NewQueueNumber.md) | The updated data for the queue number | [required] |

### Return type

[**models::QueueNumber**](QueueNumber.md)

### Authorization

[oauth](../README.md#oauth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## queue

> Vec<models::LegacyQueue> queue()
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

