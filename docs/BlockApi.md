# \BlockApi

All URIs are relative to *https://yser.p7m.de/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**block_get**](BlockApi.md#block_get) | **GET** /block | 
[**block_id_delete**](BlockApi.md#block_id_delete) | **DELETE** /block/{id} | 
[**block_id_get**](BlockApi.md#block_id_get) | **GET** /block/{id} | 
[**block_id_put**](BlockApi.md#block_id_put) | **PUT** /block/{id} | 
[**block_post**](BlockApi.md#block_post) | **POST** /block | 



## block_get

> Vec<crate::models::Block> block_get()


Returns the intervals at which a waiting queue is blocked (e.g. callers cannot enter the queue but get an announcement) 

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<crate::models::Block>**](Block.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## block_id_delete

> block_id_delete(id)


Deletes the specified block  **Note:** Normally blocks should not be deleted because they serve as a history element for some services. If you want to end a block prematurely update the `end_time` of the block using a PUT request instead. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | block id | [required] |

### Return type

 (empty response body)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## block_id_get

> crate::models::Block block_id_get(id)


Get a specific block by its ID value

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | block id | [required] |

### Return type

[**crate::models::Block**](Block.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## block_id_put

> crate::models::Block block_id_put(id, block)


Update the specified block

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | block id | [required] |
**block** | [**Block**](Block.md) | the updated block | [required] |

### Return type

[**crate::models::Block**](Block.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## block_post

> crate::models::Block block_post(new_block)


Creates a new block for a waiting queue  The value given for `announce` defines the announcement played to the caller. For the values `orga` (please call again later) and `today` (please call again tomorrow) there are prepared audio files on the server.  The fields `startTime` and `endTime` are values for which starting and ending the block has been planned/configured. While the system fills in the fields `started` and `ended` automatically when it starts and ends the block on the VoIP server. Therefore when creating a new block, the `started` and `ended` field should be left empty, because the server will not process the events if there is already something in this field. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**new_block** | [**NewBlock**](NewBlock.md) | the new block to be created | [required] |

### Return type

[**crate::models::Block**](Block.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

