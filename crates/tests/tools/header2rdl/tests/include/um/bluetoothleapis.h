/*++

Copyright (c) 2010      Microsoft Corporation

Module Name:

   BluetoothLEApis.h

Abstract:

   Public Bluetooth Low Energy Application Development Interfaces

Environment:

    User mode
  --*/

#ifdef _MSC_VER
#pragma once
#endif //_MSC_VER
#include <winapifamily.h>
#include <BthLEDef.h>

#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)


#ifdef __cplusplus
extern "C"{
#endif

#if NTDDI_VERSION >= NTDDI_WIN8



//
//  Description:
//      The BluetoothGATTGetServices function gets all the primary services available for a server.
//
//  Parameters:
//      hDevice [in]
//          Handle to the Bluetooth device from which to obtain the list of primary services.
//
//      ServicesBufferCount [in]
//          The number of elements allocated for the ServicesBuffer parameter.
//
//      ServicesBuffer [out, optional]
//          Pointer to buffer into which to return services.
//
//      ServicesBufferActual [out]
//          Pointer to buffer into which the actual number of services were returned in the ServicesBuffer parameter.
//
//      Flags [in]
//          BLUETOOTH_GATT_FLAG_NONE: The client does not have specific GATT requirements (default).
//
//  Return Value:
//      S_OK
//          The operation completed successfully.
//
//      ERROR_MORE_DATA
//          The buffer parameter is NULL and the number of items available is being returned.
//
//      ERROR_INVALID_PARAMETER
//          One of the parameters were invalid.
//
//      ERROR_INVALID_USER_BUFFER
//          A buffer is specified, but the buffer count size is smaller than what is required, in bytes.
//
//      ERROR_INVALID_FUNCTION
//          No services are present in the cache.
//
//      ERROR_BAD_COMMAND
//          The current data in the cache appears to be inconsistent, and is leading to internal errors.
//
//      ERROR_NO_SYSTEM_RESOURCES
//          The operation ran out of memory.
//
_Success_(S_OK == return)
HRESULT WINAPI
BluetoothGATTGetServices(
    _In_ HANDLE hDevice,
    _In_ USHORT ServicesBufferCount,
    _Out_writes_to_opt_(ServicesBufferCount, *ServicesBufferActual) PBTH_LE_GATT_SERVICE ServicesBuffer,
    _Out_ USHORT* ServicesBufferActual,
    _In_ ULONG Flags
    );

//
//  Description:
//      The BluetoothGATTGetIncludedServices function gets all the included services available for a given service.
//
//  Parameters:
//      hDevice [in]
//          Handle to the Bluetooth device or parent service.
//
//      ParentService [in]
//          The parent service of the included services to be retrieved. This parameter
//          is required if a device handle is passed to hDevice. This parameter is optional
//          if a service handle was passed to hDevice, in which case the service specified
//          by the service handle will be treated as the parent unless the parent service
//          is provided.  If both a parent service and a service handle is provided and the
//          parent service hierarchy do not roll up to the service handle that is provided,
//          the function will fail with ERROR_ACCESS_DENIED.
//
//      IncludedServicesBufferCount [in]
//          The number of elements allocated for the IncludedServicesBuffer parameter.
//
//      IncludedServicesBuffer [out, optional]
//          Pointer to buffer into which to return included services.
//
//      IncludedServicesBufferActual [out]
//          Pointer to buffer into which the actual number of included services were returned in
//          the IncludedServicesBuffer parameter.
//
//      Flags [in]
//          BLUETOOTH_GATT_FLAG_NONE: The client does not have specific GATT requirements (default).
//
//  Return Value:
//      S_OK
//          The operation completed successfully.
//
//      ERROR_MORE_DATA
//          The buffer parameter is NULL and the number of items available is being returned.
//
//      ERROR_INVALID_PARAMETER
//          One of the parameters were invalid.
//
//      ERROR_INVALID_USER_BUFFER
//          A buffer is specified, but the buffer count size is smaller than what is required, in bytes.
//
//      ERROR_INVALID_FUNCTION
//          No services are present in the cache.
//
//      ERROR_BAD_COMMAND
//          The current data in the cache appears to be inconsistent, and is leading to internal errors.
//
//      ERROR_NO_SYSTEM_RESOURCES
//          The operation ran out of memory.
//
//      ERROR_ACCESS_DENIED
//          If both a parent service and a service handle is provided and the
//          parent service hierarchy do not roll up to the service handle that is provided.
//
_Success_(S_OK == return)
HRESULT WINAPI
BluetoothGATTGetIncludedServices(
    _In_ HANDLE hDevice,
    _In_opt_ PBTH_LE_GATT_SERVICE ParentService,
    _In_ USHORT IncludedServicesBufferCount,
    _Out_writes_to_opt_(IncludedServicesBufferCount, *IncludedServicesBufferActual) PBTH_LE_GATT_SERVICE IncludedServicesBuffer,
    _Out_ USHORT* IncludedServicesBufferActual,
    _In_ ULONG Flags
    );

//
//  Description:
//      The BluetoothGATTGetCharacteristics function gets all the characteristics available for the specified service.
//
//  Parameters:
//      hDevice [in]
//          Handle to the Bluetooth device or service.
//
//      Service [in]
//          The parent service of the characteristics to be retrieved. This parameter
//          is required if a device handle is passed to hDevice. This parameter is optional
//          if a service handle was passed to hDevice, in which case the service specified
//          by the service handle will be treated as the parent.
//
//      CharacteristicsBufferCount [in]
//          The number of elements allocated for the CharacteristicsBuffer parameter.
//
//      CharacteristicsBuffer [out, optional]
//          Pointer to buffer into which to return characteristics.
//
//      IncludedServicesBufferActual [out]
//          Pointer to buffer into which the actual number of characteristics were returned in
//          the CharacteristicsBuffer parameter.
//
//      Flags [in]
//          BLUETOOTH_GATT_FLAG_NONE: The client does not have specific GATT requirements (default).
//
//  Return Value:
//      S_OK
//          The operation completed successfully.
//
//      ERROR_MORE_DATA
//          The buffer parameter is NULL and the number of items available is being returned.
//
//      ERROR_INVALID_PARAMETER
//          One of the parameters were invalid.
//
//      ERROR_INVALID_USER_BUFFER
//          A buffer is specified, but the buffer count size is smaller than what is required, in bytes.
//
//      ERROR_BAD_COMMAND
//          The current data in the cache appears to be inconsistent, and is leading to internal errors.
//
///      ERROR_NO_SYSTEM_RESOURCES
//          The operation ran out of memory.
//
_Success_(S_OK == return)
HRESULT WINAPI
BluetoothGATTGetCharacteristics(
    _In_ HANDLE hDevice,
    _In_opt_ PBTH_LE_GATT_SERVICE Service,
    _In_ USHORT CharacteristicsBufferCount,
    _Out_writes_to_opt_(CharacteristicsBufferCount, *CharacteristicsBufferActual) PBTH_LE_GATT_CHARACTERISTIC CharacteristicsBuffer,
    _Out_ USHORT* CharacteristicsBufferActual,
    _In_ ULONG Flags
    );

//
//  Description:
//      The BluetoothGATTGetDescriptors function gets all the descriptors available for the specified characteristic.
//
//  Parameters:
//      hDevice [in]
//          Handle to the Bluetooth device or service. If a service handle is passed, then the service must be
//          the grandparent of the descriptor.
//
//      Characteristic [in]
//          The parent characteristic of the descriptors to be retrieved.
//
//      DescriptorsBufferCount [in]
//          The number of elements allocated for the DescriptorsBuffer parameter.
//
//      DescriptorsBuffer [out, optional]
//          Pointer to buffer into which to return descriptors.
//
//      DescriptorsBufferActual [out]
//          Pointer to buffer into which the actual number of descriptors were returned in
//          the DescriptorsBuffer parameter.
//
//      Flags [in]
//          BLUETOOTH_GATT_FLAG_NONE: The client does not have specific GATT requirements (default).
//
//  Return Value:
//      S_OK
//          The operation completed successfully.
//
//      ERROR_MORE_DATA
//          The buffer parameter is NULL and the number of items available is being returned.
//
//      ERROR_INVALID_PARAMETER
//          One of the parameters were invalid.
//
//      ERROR_INVALID_USER_BUFFER
//          A buffer is specified, but the buffer count size is smaller than what is required, in bytes.
//
//      ERROR_BAD_COMMAND
//          The current data in the cache appears to be inconsistent, and is leading to internal errors.
//
///      ERROR_NO_SYSTEM_RESOURCES
//          The operation ran out of memory.
//
_Success_(S_OK == return)
HRESULT WINAPI
BluetoothGATTGetDescriptors(
    _In_ HANDLE hDevice,
    _In_ PBTH_LE_GATT_CHARACTERISTIC Characteristic,
    _In_ USHORT DescriptorsBufferCount,
    _Out_writes_to_opt_(DescriptorsBufferCount, *DescriptorsBufferActual) PBTH_LE_GATT_DESCRIPTOR DescriptorsBuffer,
    _Out_ USHORT* DescriptorsBufferActual,
    _In_ ULONG Flags
    );

//
//  Description:
//      The BluetoothGATTGetCharacteristicValue function gets the value of the specified characteristic.
//
//  Parameters:
//      hDevice [in]
//          Handle to the service.
//
//      Characteristic [in]
//          The parent characteristic of the characteristic value to be retrieved.
//
//      CharacteristicValueDataSize [in]
//          The number of elements allocated for the CharacteristicValue parameter.
//
//      CharacteristicValue [out, optional]
//          Pointer to buffer into which to return characteristic value.
//
//      CharacteristicValueSizeRequired [out]
//          Pointer to buffer into which to store the number of bytes
//          needed to return data in the buffer pointed to by CharacteristicValue.
//
//      Flags [in]
//          BLUETOOTH_GATT_FLAG_NONE: The client does not have specific GATT requirements (default).
//          BLUETOOTH_GATT_FLAG_CONNECTION_ENCRYPTED: The client requests the data to be transmitted over an encrypted channel.
//          BLUETOOTH_GATT_FLAG_CONNECTION_AUTHENTICATED: The client requests the data to be transmitted over an authenticated channel.
//          BLUETOOTH_GATT_FLAG_FORCE_READ_FROM_CACHE: The characteristic value is to be read from the cache
//                                                     (regardless of whether it is present in the cache or not).
//          BLUETOOTH_GATT_FLAG_FORCE_READ_FROM_DEVICE: The characteristic value is to be read directly from the device.
//                                                      This overwrites the one in the cache if one is already present.
//
//  Return Value:
//      S_OK
//          The operation completed successfully.
//
//      ERROR_MORE_DATA
//          The buffer parameter is NULL and the number of bytes needed is being returned.
//
//      ERROR_INVALID_PARAMETER
//          One of the parameters were invalid.
//
//      ERROR_INVALID_USER_BUFFER
//          A buffer is specified, but the buffer count size is smaller than what is required, in bytes.
//
//      ERROR_BAD_COMMAND
//          The current data in the cache appears to be inconsistent, and is leading to internal errors.
//
//      ERROR_BAD_NET_RESP
//          The target server did not provide an appropriate network response.
//
//      ERROR_SEM_TIMEOUT
//          The request timed-out.
//
//      ERROR_PRIVILEGE_NOT_HELD
//          The characteristic value is not readable as dictated by the characteristic properties.
//
///     ERROR_NO_SYSTEM_RESOURCES
//          The operation ran out of memory.
//
//      E_BLUETOOTH_ATT_<ERROR_CODE>
//          An error code returned by the device indicating a specific error has occured.
//          Consult the Bluetooth specification for more information.
//
_Success_(S_OK == return)
HRESULT WINAPI
BluetoothGATTGetCharacteristicValue(
    _In_ HANDLE hDevice,
    _In_ PBTH_LE_GATT_CHARACTERISTIC Characteristic,
    _In_ ULONG CharacteristicValueDataSize,
    _Out_opt_ PBTH_LE_GATT_CHARACTERISTIC_VALUE CharacteristicValue,
    _Out_opt_ USHORT* CharacteristicValueSizeRequired,
    _In_ ULONG Flags
    );

//
//  Description:
//      The BluetoothGATTGetDescriptorValue function gets the value of the specified descriptor.
//
//  Parameters:
//      hDevice [in]
//          Handle to the service.
//
//      Descriptor [in]
//          The parent descriptor of the descriptor value to be retrieved.
//
//      DescriptorValueDataSize [in]
//          The number of elements allocated for the DescriptorValue parameter.
//
//      DescriptorValue [out, optional]
//          Pointer to buffer into which to return descriptor value.
//
//      DescriptorValueSizeRequired [out]
//          Pointer to buffer into which to store the number of bytes
//          needed to return data in the buffer pointed to by DescriptorValue.
//
//      Flags [in]
//          BLUETOOTH_GATT_FLAG_NONE: The client does not have specific GATT requirements (default).
//          BLUETOOTH_GATT_FLAG_CONNECTION_ENCRYPTED: The client requests the data to be transmitted over an encrypted channel.
//          BLUETOOTH_GATT_FLAG_CONNECTION_AUTHENTICATED: The client requests the data to be transmitted over an authenticated channel.
//          BLUETOOTH_GATT_FLAG_FORCE_READ_FROM_CACHE: The descriptor value is to be read from the cache
//                                                     (regardless of whether it is present in the cache or not).
//          BLUETOOTH_GATT_FLAG_FORCE_READ_FROM_DEVICE: The descriptor value is to be read directly from the device.
//                                                      This overwrites the one in the cache if one is already present.
//
//  Return Value:
//      S_OK
//          The operation completed successfully.
//
//      ERROR_MORE_DATA
//          The buffer parameter is NULL and the number of bytes needed is being returned.
//
//      ERROR_INVALID_PARAMETER
//          One of the parameters were invalid.
//
//      ERROR_INVALID_USER_BUFFER
//          A buffer is specified, but the buffer count size is smaller than what is required, in bytes.
//
//      ERROR_BAD_COMMAND
//          The current data in the cache appears to be inconsistent, and is leading to internal errors.
//
//      ERROR_BAD_NET_RESP
//          The target server did not provide an appropriate network response.
//
//      ERROR_SEM_TIMEOUT
//          The request timed-out.
//
///     ERROR_NO_SYSTEM_RESOURCES
//          The operation ran out of memory.
//
//      E_BLUETOOTH_ATT_<ERROR_CODE>
//          An error code returned by the device indicating a specific error has occured.
//          Consult the Bluetooth specification for more information.
//
_Success_(S_OK == return)
HRESULT WINAPI
BluetoothGATTGetDescriptorValue(
    _In_ HANDLE hDevice,
    _In_ PBTH_LE_GATT_DESCRIPTOR Descriptor,
    _In_ ULONG DescriptorValueDataSize,
    _Out_opt_ PBTH_LE_GATT_DESCRIPTOR_VALUE DescriptorValue,
    _Out_opt_ USHORT* DescriptorValueSizeRequired,
    _In_ ULONG Flags
    );

//
//  Description:
//      The BluetoothGATTBeginReliableWrite function specifies that reliable writes are about to begin.
//
//  Parameters:
//      hDevice [in]
//          Handle to the service.
//
//      ReliableWriteContext [out]
//          Pointer to the context describing the reliable write operation.
//
//      Flags [in]
//          Reserved. Must be set to BLUETOOTH_GATT_FLAG_NONE.
//
//  Return Value:
//      S_OK
//          The operation completed successfully.
//
//      ERROR_INVALID_FUNCTION
//          A reliable write operation is already presently underway.
//
_Success_(S_OK == return)
HRESULT WINAPI
BluetoothGATTBeginReliableWrite(
    _In_ HANDLE hDevice,
    _Out_ PBTH_LE_GATT_RELIABLE_WRITE_CONTEXT ReliableWriteContext,
    _In_ ULONG Flags
    );

//
//  Description:
//      The BluetoothGATTSetCharacteristicValue function writes the specified characteristic value to the Bluetooth device.
//
//  Parameters:
//      hDevice [in]
//          Handle to the service.
//
//      Characteristic [in]
//          The parent characteristic.
//
//      CharacteristicValue [in]
//          Pointer to the characteristic value.
//
//      ReliableWriteContext [in]
//          The context describing the reliable write operation returned from a
//          previous call to BluetoothGATTBeginReliableWrite.
//
//      Flags [in]
//          BLUETOOTH_GATT_FLAG_NONE: The client does not have specific GATT requirements (default).
//          BLUETOOTH_GATT_FLAG_CONNECTION_ENCRYPTED: The client requests the data to be transmitted over an encrypted channel.
//          BLUETOOTH_GATT_FLAG_CONNECTION_AUTHENTICATED: The client requests the data to be transmitted over an authenticated channel.
//          BLUETOOTH_GATT_FLAG_WRITE_WITHOUT_RESPONSE : Write without response.
//          BLUETOOTH_GATT_FLAG_SIGNED_WRITE: Signed write. Profile drivers must use with
//                                            BLUETOOTH_GATT_FLAG_WRITE_WITHOUT_RESPONSE in order to produce
//                                            signed write without a response.
//
//  Return Value:
//      S_OK
//          The operation completed successfully.
//
//      ERROR_INVALID_PARAMETER
//          One of the parameters were invalid.
//
//      ERROR_BAD_NET_RESP
//          The target server did not provide an appropriate network response.
//
//      ERROR_SEM_TIMEOUT
//          The request timed-out.
//
///     ERROR_NO_SYSTEM_RESOURCES
//          The operation ran out of memory.
//
//      ERROR_INVALID_FUNCTION
//          A reliable write operation is already presently underway.
//
//      E_BLUETOOTH_ATT_<ERROR_CODE>
//          An error code returned by the device indicating a specific error has occured.
//          Consult the Bluetooth specification for more information.
//
_Success_(S_OK == return)
HRESULT WINAPI
BluetoothGATTSetCharacteristicValue(
    _In_ HANDLE hDevice,
    _In_ PBTH_LE_GATT_CHARACTERISTIC Characteristic,
    _In_ PBTH_LE_GATT_CHARACTERISTIC_VALUE CharacteristicValue,
    _In_opt_ BTH_LE_GATT_RELIABLE_WRITE_CONTEXT ReliableWriteContext,
    _In_ ULONG Flags
    );

//
//  Description:
//      The BluetoothGATTEndReliableWrite function specifies the end of reliable writes,
//      and the writes should be committed.
//
//  Parameters:
//      hDevice [in]
//          Handle to the service.
//
//      ReliableWriteContext [in]
//          The context describing the reliable write operation returned from a previous
//          call to BluetoothGATTBeginReliableWrite.
//
//      Flags [in]
//          Reserved. Must be set to BLUETOOTH_GATT_FLAG_NONE.
//
//  Return Value:
//      S_OK
//          The operation completed successfully.
//
//      ERROR_INVALID_FUNCTION
//          A reliable write operation is no presently underway.
//
//      ERROR_BAD_NET_RESP
//          The target server did not provide an appropriate network response.
//
//      ERROR_SEM_TIMEOUT
//          The request timed-out.
//
///     ERROR_NO_SYSTEM_RESOURCES
//          The operation ran out of memory.
//
//      E_BLUETOOTH_ATT_<ERROR_CODE>
//          An error code returned by the device indicating a specific error has occured.
//          Consult the Bluetooth specification for more information.
//
_Success_(S_OK == return)
HRESULT WINAPI
BluetoothGATTEndReliableWrite(
    _In_ HANDLE hDevice,
    _In_ BTH_LE_GATT_RELIABLE_WRITE_CONTEXT ReliableWriteContext,
    _In_ ULONG Flags
    );


//
//  Description:
//      The BluetoothGATTAbortReliableWrite function specifies the end of reliable
//      write procedures, and the writes should be aborted.
//
//  Parameters:
//      hDevice [in]
//          Handle to the service.
//
//      ReliableWriteContext [in]
//          The context describing the reliable write operation returned from a previous
//          call to BluetoothGATTBeginReliableWrite.
//
//      Flags [in]
//          Reserved. Must be set to BLUETOOTH_GATT_FLAG_NONE.
//
//  Return Value:
//      S_OK
//          The operation completed successfully.
//
//      ERROR_INVALID_FUNCTION
//          A reliable write operation is no presently underway.
//
//      ERROR_BAD_NET_RESP
//          The target server did not provide an appropriate network response.
//
//      ERROR_SEM_TIMEOUT
//          The request timed-out.
//
///     ERROR_NO_SYSTEM_RESOURCES
//          The operation ran out of memory.
//
//      E_BLUETOOTH_ATT_<ERROR_CODE>
//          An error code returned by the device indicating a specific error has occured.
//          Consult the Bluetooth specification for more information.
//
_Success_(S_OK == return)
HRESULT WINAPI
BluetoothGATTAbortReliableWrite(
    _In_ HANDLE hDevice,
    _In_ BTH_LE_GATT_RELIABLE_WRITE_CONTEXT ReliableWriteContext,
    _In_ ULONG Flags
    );


//
//  Description:
//      The BluetoothGATTSetDescriptorValue function writes the specified descriptor value to the Bluetooth device.
//
//  Parameters:
//      hDevice [in]
//          Handle to the service.
//
//      Descriptor [in]
//          The parent descriptor.
//
//      DescriptorValue [in]
//          Pointer to the descriptor value.
//
//      Flags [in]
//          BLUETOOTH_GATT_FLAG_NONE: The client does not have specific GATT requirements (default).
//          BLUETOOTH_GATT_FLAG_CONNECTION_ENCRYPTED: The client requests the data to be transmitted over an encrypted channel.
//          BLUETOOTH_GATT_FLAG_CONNECTION_AUTHENTICATED: The client requests the data to be transmitted over an authenticated channel.
//
//  Return Value:
//      S_OK
//          The operation completed successfully.
//
//      ERROR_INVALID_PARAMETER
//          One of the parameters were invalid.
//
//      ERROR_BAD_NET_RESP
//          The target server did not provide an appropriate network response.
//
//      ERROR_SEM_TIMEOUT
//          The request timed-out.
//
///     ERROR_NO_SYSTEM_RESOURCES
//          The operation ran out of memory.
//
//      ERROR_INVALID_FUNCTION
//          A reliable write operation is already presently underway.
//
//      E_BLUETOOTH_ATT_<ERROR_CODE>
//          An error code returned by the device indicating a specific error has occured.
//          Consult the Bluetooth specification for more information.
//
_Success_(S_OK == return)
HRESULT WINAPI
BluetoothGATTSetDescriptorValue(
    _In_ HANDLE hDevice,
    _In_ PBTH_LE_GATT_DESCRIPTOR Descriptor,
    _In_ PBTH_LE_GATT_DESCRIPTOR_VALUE DescriptorValue,
    _In_ ULONG Flags
    );


//
//  Description:
//      Registers for a characteristic value change event on the given characteristic
//      identified by its characteristic handle.
//
//  Parameters:
//      hDevice [in]
//          Handle to the Bluetooth device or a service in the Bluetooth device.
//          For CharacteristicValueChangedEvent, this must be a service handle.
//
//      EventType [in]
//          CharacteristicValueChangedEvent: event callback to receive Handle Value Notifications or Handle Value Indications.
//
//      EventParameters [in]
//          CharacteristicValueChangedEvent: A pointer to BLUETOOTH_GATT_NOTIFICATION_REGISTRATION
//                                           that specifies the characteristics to be registered for the changed event notifications.
//
//      Callback [in]
//          The callback method to call when the Characteristic value changes.
//
//      CallbackContext [in]
//          A client context provided in the client provided callback function.
//
//      EventHandle [out]
//          Upon successful registration, this parameter receives a handle to this registration.
//          The client shall use this handle when calling BluetoothGATTUnregisterEvent.
//
//      Flags [in]
//          BLUETOOTH_GATT_FLAG_NONE: The client does not have specific GATT requirements (default).
//          BLUETOOTH_GATT_FLAG_CONNECTION_ENCRYPTED: The client requests the data to be transmitted over an encrypted channel.
//          BLUETOOTH_GATT_FLAG_CONNECTION_AUTHENTICATED: The client requests the data to be transmitted over an authenticated channel.
//
//  Return Value:
//      S_OK
//          The operation completed successfully.
//
//      ERROR_INVALID_PARAMETER
//          One of the parameters were invalid.
//
HRESULT WINAPI
BluetoothGATTRegisterEvent(
    _In_ HANDLE hService,
    _In_ BTH_LE_GATT_EVENT_TYPE EventType,
    _In_ PVOID EventParameterIn,
    _In_ PFNBLUETOOTH_GATT_EVENT_CALLBACK Callback,
    _In_opt_ PVOID CallbackContext,
    _Out_ BLUETOOTH_GATT_EVENT_HANDLE * pEventHandle,
    _In_ ULONG Flags
    );

//
//  Description:
//      Unregisters the given characteristic value change event.
//
//  Parameters:
//      EventHandle [in]
//          Obtained via BluetoothGATTRegisterEvent
//
//      Flags [in]
//          BLUETOOTH_GATT_FLAG_NONE: The client does not have specific GATT requirements (default).
//
//  Return Value:
//      S_OK
//          The operation completed successfully.
//
//      ERROR_INVALID_PARAMETER
//          One of the parameters were invalid.
//
HRESULT WINAPI
BluetoothGATTUnregisterEvent(
    _In_ BLUETOOTH_GATT_EVENT_HANDLE EventHandle,
    _In_ ULONG Flags
    );


#endif //NTDDI_WIN8

#ifdef __cplusplus
    }
#endif



#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion


