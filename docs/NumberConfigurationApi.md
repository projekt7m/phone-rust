# \NumberConfigurationApi

All URIs are relative to *https://yser.p7m.de/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**delete_number_configurations_number_configuration_id**](NumberConfigurationApi.md#delete_number_configurations_number_configuration_id) | **DELETE** /numberconfigurations/{ncid} | Delete a number configuration
[**get_number_configurations_number_configuration_id**](NumberConfigurationApi.md#get_number_configurations_number_configuration_id) | **GET** /numberconfigurations/{ncid} | Get a single number configuration
[**get_number_configurations_number_superadmin**](NumberConfigurationApi.md#get_number_configurations_number_superadmin) | **GET** /numberconfigurations/{phone_number} | Get a number configuration for a given phone number
[**get_number_configurations_superadmin**](NumberConfigurationApi.md#get_number_configurations_superadmin) | **GET** /numberconfigurations | Get the list of all number configurations
[**post_number_configurations**](NumberConfigurationApi.md#post_number_configurations) | **POST** /numberconfigurations | Create a new number configuration



## delete_number_configurations_number_configuration_id

> delete_number_configurations_number_configuration_id(ncid)
Delete a number configuration

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ncid** | **uuid::Uuid** | ID of the number configuration | [required] |

### Return type

 (empty response body)

### Authorization

[oauth](../README.md#oauth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_number_configurations_number_configuration_id

> models::NumberConfiguration get_number_configurations_number_configuration_id(ncid)
Get a single number configuration

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ncid** | **uuid::Uuid** | ID of the number configuration | [required] |

### Return type

[**models::NumberConfiguration**](NumberConfiguration.md)

### Authorization

[oauth](../README.md#oauth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_number_configurations_number_superadmin

> models::NumberConfiguration get_number_configurations_number_superadmin(phone_number)
Get a number configuration for a given phone number

Can only be called with system or super-admin privileges

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**phone_number** | **String** | The phone number to get configuration for | [required] |

### Return type

[**models::NumberConfiguration**](NumberConfiguration.md)

### Authorization

[oauth](../README.md#oauth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_number_configurations_superadmin

> models::ListWrapperNumberConfiguration get_number_configurations_superadmin()
Get the list of all number configurations

Super-admins users get all number configurations, other users only those of their tenant

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::ListWrapperNumberConfiguration**](ListWrapper_NumberConfiguration.md)

### Authorization

[oauth](../README.md#oauth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_number_configurations

> models::NumberConfiguration post_number_configurations(new_number_configuration)
Create a new number configuration

Can only be called with super-admin rights

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**new_number_configuration** | [**NewNumberConfiguration**](NewNumberConfiguration.md) | The number configuration to be created | [required] |

### Return type

[**models::NumberConfiguration**](NumberConfiguration.md)

### Authorization

[oauth](../README.md#oauth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

