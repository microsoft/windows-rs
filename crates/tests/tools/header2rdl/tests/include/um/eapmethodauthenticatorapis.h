///////////////////////////////////////////////////////////////////////////////
//
// Copyright (c) Microsoft Corporation
//
// SYNOPSIS
//
//   Declares datastructures that are needed by eap methods on the Peer.
//
///////////////////////////////////////////////////////////////////////////////

#ifndef EAP_METHOD_AUTHENTICATOR_APIS_H
#define EAP_METHOD_AUTHENTICATOR_APIS_H
#pragma once
#include <winapifamily.h>

#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)

#ifndef __eapauthenticatortypes_h__
#include "EapAuthenticatorTypes.h"
#endif

#ifndef EAPAUTHENTICATORACTIONDEFINE_H
#include "EapAuthenticatorActionDefine.h"
#endif

#ifdef __cplusplus
extern "C"
{
#endif

    //
    // Defines used for installation of EAP DLL
    // HKLM\System\CCS\Services\Eaphost\Methods\[AuthorId]\[EapTypeId])
    //
    // Custom EAP DLL registry installation example:
    //                     Name      =  Sample.dll,
    //                     AuthorId  =  311(Microsoft),
    //                     EapTypeId = (decimal 40)
    // HKLM\System\CCS\Services\Eaphost\Methods\311\40
    //
    //      AuthenticatorDllPath             (REG_EXPAND_SZ) %SystemRoot%\system32\sample.dll
    //      AuthenticatorFriendlyName        (REG_SZ) Sample EAP Protocol

    //
    // Defines used for installation of EAP DLL
    // HKLM\System\CCS\Services\Eaphost\Methods\[AuthorId]\254\[VendorId]\[EapTypeId])
    //
    // Custom Expanded EAP DLL registry installation example:
    //                     Name      =  Sample.dll,
    //                     AuthorId  =  311(Microsoft),
    //                     VendorId  =  311(Microsoft),
    //                     EapTypeId = (decimal 40)
    // HKLM\System\CCS\Services\Eaphost\Methods\311\254\311\40
    //
    //      AuthenticatorDllPath             (REG_EXPAND_SZ) %SystemRoot%\system32\sample.dll
    //      AuthenticatorFriendlyName        (REG_SZ) Sample EAP Protocol

    //      ConfigCLSID         (REG_SZ)        {0000031A-0000-0000-C000-000000000046}
    //      StandaloneSupported (REG_DWORD)     1
    //

#define EAP_REGISTRY_LOCATION TEXT("System\\CurrentControlSet\\Services\\EapHost\\Methods")

#define EAP_AUTHENTICATOR_VALUENAME_DLL_PATH TEXT("AuthenticatorDllPath")
#define EAP_AUTHENTICATOR_VALUENAME_FRIENDLY_NAME TEXT("AuthenticatorFriendlyName")
#define EAP_AUTHENTICATOR_VALUENAME_PROPERTIES TEXT("Properties")
#define EAP_AUTHENTICATOR_VALUENAME_CONFIGUI TEXT("AuthenticatorConfigUIPath")

    // Structures used by EAP methods on the peer to communicate with the host.

    // List of functions corresponding to the particular EAP method
    typedef struct _EAP_AUTHENTICATOR_METHOD_ROUTINES
    {
        DWORD dwSizeInBytes; // unused
        EAP_METHOD_TYPE* pEapType;

        DWORD(APIENTRY* EapMethodAuthenticatorInitialize)(_In_ EAP_METHOD_TYPE* pEapType, _Outptr_ EAP_ERROR** ppEapError);

        DWORD(APIENTRY* EapMethodAuthenticatorBeginSession)(
            // Flags to qualify the authentication process.
            _In_ DWORD dwFlags,
            // Identity of the user being authenticated
            _In_ LPCWSTR pwszIdentity,
            // Pointer to an array of attributes. This array contains attributes that
            // describe the entity being authenticated.
            _In_ const EapAttributes* const pAttributeArray,
            // Specifies the size in bytes of the data pointed to by pConnectionData.
            // If pConnectionData is NULL, this member is zero.
            _In_ DWORD dwSizeofConnectionData,
            // Pointer to connection data received from the authentication protocol's
            // configuration user interface.
            _In_reads_(dwSizeofConnectionData) const BYTE* const pConnectionData,
            // This is the maximum size of an eap packet that the authenticator can send.
            _In_ DWORD dwMaxSendPacketSize,
            // The session handle that identifies the current authentication session.
            _Out_ EAP_SESSION_HANDLE* pSessionHandle,
            // On an unsuccessful call, this will contain any error information about
            // the failure. This will be null on a successful call.
            _Outptr_ EAP_ERROR** ppEapError);

        DWORD(APIENTRY* EapMethodAuthenticatorUpdateInnerMethodParams)(
            // context handle as returned from a successful call to
            // EapAuthenticatorBeginSession
            _In_ EAP_SESSION_HANDLE sessionHandle,
            _In_ DWORD dwFlags,
            _In_ LPCWSTR pwszIdentity,
            // Pointer to an array of attributes. This array contains attributes that
            // describe the entity being authenticated.
            _In_ const EapAttributes* const pAttributeArray,
            // On an unsuccessful call, this will contain any error information about
            // the failure. This will be null on a successful call.
            _Outptr_ EAP_ERROR** ppEapError);

        // The authenticator calls this any time it receives a packet that the eaphost
        // needs to process. This should be called only after a successful call to
        // EapAuthenticatorBeginSession.
        DWORD(APIENTRY* EapMethodAuthenticatorReceivePacket)(
            // context handle as returned from a successful call to
            // EapAuthenticatorBeginSession
            _In_ EAP_SESSION_HANDLE sessionHandle,
            // Specifies the size, in bytes, of the buffer pointed to by
            // pReceivePacket
            _In_ DWORD cbReceivePacket,
            // Pointer to a buffer that contains the incoming EAP data received by
            // the supplicant.
            _In_reads_bytes_(cbReceivePacket) const EapPacket* const pReceivePacket,
            // This enumeration tells the supplicant to take an appropriate action.
            // The supplicant will typically look at this action and either call
            // another method on eaphost or do something else on its own.
            _Out_ EAP_METHOD_AUTHENTICATOR_RESPONSE_ACTION* pEapOutput,
            // On an unsuccessful call, this will contain any error information about
            // the failure. This will be null on a successful call.
            _Outptr_ EAP_ERROR** ppEapError);

        DWORD(APIENTRY* EapMethodAuthenticatorSendPacket)(
            // context handle as returned from a successful call to
            // EapHostAuthenticatorBeginSession
            _In_ EAP_SESSION_HANDLE sessionHandle,
            // Id to use when constructing the SendPacket
            _In_ BYTE bPacketId,
            // Specifies the limit on the size, in bytes, on the packet generated
            // by eaphost. On a successful return, this will contain the size of the
            // data added by the eap module.
            _Inout_ _Pre_satisfies_(*pcbSendPacket >= sizeof(EapPacket)) DWORD* pcbSendPacket,
            // Pointer to a buffer that is allocated by the client and populated
            // by the eap module. The value of the incoming buffer is ignored and
            // the method populates it from the beginning of the buffer.
            _Inout_ _Pre_writable_byte_size_(*pcbSendPacket) _Pre_readable_byte_size_(*pcbSendPacket) EapPacket* pSendPacket,
            // Timeout option for sending the packet
            _Out_ EAP_AUTHENTICATOR_SEND_TIMEOUT* pTimeout,
            // On an unsuccessful call, this will contain any error information about
            // the failure. This will be null on a successful call.
            _Outptr_ EAP_ERROR** ppEapError);

        // Returns an array of attributes that the caller needs to act on.
        // The supplicant will call this when a call to
        // EapHostAuthenticatorProcessRequestPacket returns EapHostAuthenticatorResponseRespond.
        DWORD(APIENTRY* EapMethodAuthenticatorGetAttributes)(
            // context handle as returned from a successful call to
            // EapHostAuthenticatorBeginSession
            _In_ EAP_SESSION_HANDLE sessionHandle,
            // Array of attributes that the caller needs to act on.
            _Out_ EapAttributes* pAttribs,
            _Outptr_ EAP_ERROR** ppEapError);

        // Sets an array of attributes that the caller wants the eap method to act
        // on.
        DWORD(APIENTRY* EapMethodAuthenticatorSetAttributes)(
            // context handle as returned from a successful call to
            // EapHostAuthenticatorBeginSession
            _In_ EAP_SESSION_HANDLE sessionHandle,
            _In_ const EapAttributes* const pAttribs,
            // This enumeration tells the supplicant to take an appropriate action.
            // The supplicant will typically look at this action and either call
            // another method on eaphost or do something else on its own.
            _Out_ EAP_METHOD_AUTHENTICATOR_RESPONSE_ACTION* pEapOutput,
            // On an unsuccessful call, this will contain any error information about
            // the failure. This will be null on a successful call.
            _Outptr_ EAP_ERROR** ppEapError);

        // The authenticator will call this on completion of an authentication. This
        // can happen in any of the following scenarios:
        // 1. A call to EapHostAuthenticatorReceivePacket returned
        //    EAP_HOST_AUTHENTICATOR_RESPONSE_SUCCESS or EAP_HOST_AUTHENTICATOR_RESPONSE_FAILURE
        //    Even if the action returned above was a success, the authenticator can choose to call
        //    this method with a failure.
        // 2. The server can choose to terminate an authentication with a failure in the middle of
        //    an authentication.
        DWORD(APIENTRY* EapMethodAuthenticatorGetResult)(
            // context handle as returned from a successful call to
            // EapHostPeerBeginSession
            _In_ EAP_SESSION_HANDLE sessionHandle,
            // A structure that indicates the result and any state that the
            // supplicant needs to save for future authentications.
            _Out_ EAP_METHOD_AUTHENTICATOR_RESULT* pResult,
            // On an unsuccessful call, this will contain any error information about
            // the failure. This will be null on a successful call.
            _Outptr_ EAP_ERROR** ppEapError);

        // Ends the authentication session. This cleans up any state that the eap
        // method or eaphost might be keeping.
        DWORD(APIENTRY* EapMethodAuthenticatorEndSession)(
            // context handle as returned from a successful call to
            // EapHostPeerBeginSession. This will be set to NULL on a successful call.
            _In_ EAP_SESSION_HANDLE sessionHandle,
            // On an unsuccessful call, this will contain any error information about
            // the failure. This will be null on a successful call.
            _Outptr_ EAP_ERROR** ppEapError);

        DWORD(APIENTRY* EapMethodAuthenticatorShutdown)(_In_ EAP_METHOD_TYPE* pEapType, _Outptr_ EAP_ERROR** ppEapError);

    } EAP_AUTHENTICATOR_METHOD_ROUTINES, *PEAP_AUTHENTICATOR_METHOD_ROUTINES;

    //
    // EAP Functions
    //
    void WINAPI EapMethodAuthenticatorFreeErrorMemory(_In_ EAP_ERROR* pEapError);
    void WINAPI EapMethodAuthenticatorFreeMemory(_In_ void* pUIContextData);

    DWORD EapMethodAuthenticatorGetInfo(
        _In_ EAP_METHOD_TYPE* pEapType, _Out_ EAP_AUTHENTICATOR_METHOD_ROUTINES* pEapInfo, _Outptr_ EAP_ERROR** ppEapError);

    DWORD EapMethodAuthenticatorInitialize(_In_ EAP_METHOD_TYPE* pEapType, _Outptr_ EAP_ERROR** ppEapError);

    DWORD EapMethodAuthenticatorBeginSession(
        // Flags to qualify the authentication process.
        _In_ DWORD dwFlags,
        // Initial Id
        _In_ BYTE bInitialId,
        // Identity of the user being authenticated
        _In_ LPCWSTR pwszIdentity,
        // Pointer to an array of attributes. This array contains attributes that
        // describe the entity being authenticated.
        _In_ const EapAttributes* const pAttributeArray,
        // Specifies the size in bytes of the data pointed to by pConnectionData.
        // If pConnectionData is NULL, this member is zero.
        _In_ DWORD dwSizeofConnectionData,
        // Pointer to connection data received from the authentication protocol's
        // configuration user interface.
        _In_reads_(dwSizeofConnectionData) const BYTE* const pConnectionData,
        // This is the maximum size of an eap packet that the authenticator can send.
        _In_ DWORD dwMaxSendPacketSize,
        // The session handle that identifies the current authentication session.
        _Out_ EAP_SESSION_HANDLE* pSessionHandle,
        // On an unsuccessful call, this will contain any error information about
        // the failure. This will be null on a successful call.
        _Outptr_ EAP_ERROR** ppEapError);

    DWORD EapMethodAuthenticatorUpdateInnerMethodParams(
        // context handle as returned from a successful call to
        // EapAuthenticatorBeginSession
        _In_ EAP_SESSION_HANDLE sessionHandle,
        _In_ DWORD dwFlags,
        _In_ LPCWSTR pwszIdentity,
        // Pointer to an array of attributes. This array contains attributes that
        // describe the entity being authenticated.
        _In_ const EapAttributes* const pAttributeArray,
        // On an unsuccessful call, this will contain any error information about
        // the failure. This will be null on a successful call.
        _Outptr_ EAP_ERROR** ppEapError);

    // The authenticator calls this any time it receives a packet that the eaphost
    // needs to process. This should be called only after a successful call to
    // EapAuthenticatorBeginSession.
    DWORD EapMethodAuthenticatorReceivePacket(
        // context handle as returned from a successful call to
        // EapAuthenticatorBeginSession
        _In_ EAP_SESSION_HANDLE sessionHandle,
        // Specifies the size, in bytes, of the buffer pointed to by
        // pReceivePacket
        _In_ DWORD cbReceivePacket,
        // Pointer to a buffer that contains the incoming EAP data received by
        // the supplicant.
        _In_reads_bytes_(cbReceivePacket) const EapPacket* const pReceivePacket,
        // This enumeration tells the supplicant to take an appropriate action.
        // The supplicant will typically look at this action and either call
        // another method on eaphost or do something else on its own.
        _Out_ EAP_METHOD_AUTHENTICATOR_RESPONSE_ACTION* pEapOutput,
        // On an unsuccessful call, this will contain any error information about
        // the failure. This will be null on a successful call.
        _Outptr_ EAP_ERROR** ppEapError);

    DWORD EapMethodAuthenticatorSendPacket(
        // context handle as returned from a successful call to
        // EapHostAuthenticatorBeginSession
        _In_ EAP_SESSION_HANDLE sessionHandle,
        // Id to use when constructing the SendPacket
        _In_ BYTE bPacketId,
        // Specifies the limit on the size, in bytes, on the packet generated
        // by eaphost. On a successful return, this will contain the size of the
        // data added by the eap module.
        _Inout_ _Pre_satisfies_(*pcbSendPacket >= sizeof(EapPacket)) DWORD* pcbSendPacket,
        // Pointer to a buffer that is allocated by the client and populated
        // by the eap module. The value of the incoming buffer is ignored and
        // the method populates it from the beginning of the buffer.
        _Inout_ _Pre_writable_byte_size_(*pcbSendPacket) _Pre_readable_byte_size_(*pcbSendPacket) EapPacket* pSendPacket,
        // Timeout option for sending the packet
        _Out_ EAP_AUTHENTICATOR_SEND_TIMEOUT* pTimeout,
        // On an unsuccessful call, this will contain any error information about
        // the failure. This will be null on a successful call.
        _Outptr_ EAP_ERROR** ppEapError);

    // Returns an array of attributes that the caller needs to act on.
    // The supplicant will call this when a call to
    // EapHostAuthenticatorProcessRequestPacket returns EapHostAuthenticatorResponseRespond.
    DWORD EapMethodAuthenticatorGetAttributes(
        // context handle as returned from a successful call to
        // EapHostAuthenticatorBeginSession
        _In_ EAP_SESSION_HANDLE sessionHandle,
        // Array of attributes that the caller needs to act on.
        _Out_ EapAttributes* pAttribs,
        _Outptr_ EAP_ERROR** ppEapError);

    // Sets an array of attributes that the caller wants the eap method to act
    // on.
    DWORD EapMethodAuthenticatorSetAttributes(
        // context handle as returned from a successful call to
        // EapHostAuthenticatorBeginSession
        _In_ EAP_SESSION_HANDLE sessionHandle,
        _In_ const EapAttributes* const pAttribs,
        // This enumeration tells the supplicant to take an appropriate action.
        // The supplicant will typically look at this action and either call
        // another method on eaphost or do something else on its own.
        _Out_ EAP_METHOD_AUTHENTICATOR_RESPONSE_ACTION* pEapOutput,
        // On an unsuccessful call, this will contain any error information about
        // the failure. This will be null on a successful call.
        _Outptr_ EAP_ERROR** ppEapError);

    // The authenticator will call this on completion of an authentication. This
    // can happen in any of the following scenarios:
    // 1. A call to EapHostAuthenticatorReceivePacket returned
    //    EAP_HOST_AUTHENTICATOR_RESPONSE_SUCCESS or EAP_HOST_AUTHENTICATOR_RESPONSE_FAILURE
    //    Even if the action returned above was a success, the authenticator can choose to call
    //    this method with a failure.
    // 2. The server can choose to terminate an authentication with a failure in the middle of
    //    an authentication.
    DWORD EapMethodAuthenticatorGetResult(
        // context handle as returned from a successful call to
        // EapHostPeerBeginSession
        _In_ EAP_SESSION_HANDLE sessionHandle,
        // A structure that indicates the result and any state that the
        // supplicant needs to save for future authentications.
        _Out_ EAP_METHOD_AUTHENTICATOR_RESULT* pResult,
        // On an unsuccessful call, this will contain any error information about
        // the failure. This will be null on a successful call.
        _Outptr_ EAP_ERROR** ppEapError);

    // Ends the authentication session. This cleans up any state that the eap
    // method or eaphost might be keeping.
    DWORD EapMethodAuthenticatorEndSession(
        // context handle as returned from a successful call to
        // EapHostPeerBeginSession. This will be set to NULL on a successful call.
        _In_ EAP_SESSION_HANDLE sessionHandle,
        // On an unsuccessful call, this will contain any error information about
        // the failure. This will be null on a successful call.
        _Outptr_ EAP_ERROR** ppEapError);

    DWORD EapMethodAuthenticatorShutdown(_In_ EAP_METHOD_TYPE* pEapType, _Outptr_ EAP_ERROR** ppEapError);

    /// Config API

#define EAP_METHOD_AUTHENTICATOR_CONFIG_IS_IDENTITY_PRIVACY 0x1

    DWORD WINAPI EapMethodAuthenticatorInvokeConfigUI(
        _In_ EAP_METHOD_TYPE* pEapMethodType,
        _In_ HWND hwndParent,
        _In_ DWORD dwFlags,
        _In_ LPCWSTR pwszMachineName,
        _In_ DWORD dwSizeOfConfigIn,
        _In_reads_(dwSizeOfConfigIn) BYTE* pConfigIn,
        _Out_ DWORD* pdwSizeOfConfigOut,
        _Outptr_result_buffer_(*pdwSizeOfConfigOut) BYTE** ppConfigOut,
        _Outptr_ EAP_ERROR** ppEapError);

#ifdef __cplusplus
}
#endif

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion

#endif // EAP_METHOD_AUTHENTICATOR_APIS_H
