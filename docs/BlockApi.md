# \BlockApi

All URIs are relative to *https://yser.p7m.de/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**delete_block_by_id**](BlockApi.md#delete_block_by_id) | **DELETE** /block/{id} | Delete a block by its ID
[**get_block_by_id**](BlockApi.md#get_block_by_id) | **GET** /block/{id} | Get a single block by its ID
[**get_blocks**](BlockApi.md#get_blocks) | **GET** /block | Get the list of all waiting queue blocks
[**post_block**](BlockApi.md#post_block) | **POST** /block | Create a new block for a waiting queue
[**put_block_by_id**](BlockApi.md#put_block_by_id) | **PUT** /block/{id} | Update an existing block



## delete_block_by_id

> delete_block_by_id(id)
Delete a block by its ID

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **uuid::Uuid** | ID of the block | [required] |

### Return type

 (empty response body)

### Authorization

[oauth](../README.md#oauth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_block_by_id

> models::Block get_block_by_id(id)
Get a single block by its ID

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **uuid::Uuid** | ID of the block | [required] |

### Return type

[**models::Block**](Block.md)

### Authorization

[oauth](../README.md#oauth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_blocks

> Vec<models::Block> get_blocks()
Get the list of all waiting queue blocks

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<models::Block>**](Block.md)

### Authorization

[oauth](../README.md#oauth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_block

> models::Block post_block(new_block)
Create a new block for a waiting queue

The value given for `announce` defines the announcement played to the caller. For the values `orga` (please call again later) and `today` (please call again tomorrow) there are prepared audio files on the server.  The files `startTime` and `endTime` are values for which starting and ending the block has been planned/configured. While the system fills in the fields `started` (on yser)/`started2` (on volp) and `ended` (on yser)/`ended2` (on volp) automatically when it starts and ends the block on the VoIP servers. Therefore, when creating a new block, the `started`, `started2`, `ended`, and `ended2` fields should be left empty, because the server will not process the events if there is already something in this field.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**new_block** | [**NewBlock**](NewBlock.md) | The block that should be created | [required] |

### Return type

[**models::Block**](Block.md)

### Authorization

[oauth](../README.md#oauth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## put_block_by_id

> models::Block put_block_by_id(id, new_block)
Update an existing block

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **uuid::Uuid** | ID of the block | [required] |
**new_block** | [**NewBlock**](NewBlock.md) | The updated block | [required] |

### Return type

[**models::Block**](Block.md)

### Authorization

[oauth](../README.md#oauth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

