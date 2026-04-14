//////////////////////////////////////////////////////////////////////////////
//
// Copyright (c) Microsoft Corporation
//
// SYNOPSIS
//
//   Declares apis to invoke peer apis on eaphost.
//
///////////////////////////////////////////////////////////////////////////////

#ifndef EAPPAPIS_H
#define EAPPAPIS_H
#pragma once
#include <winapifamily.h>

#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)

#ifdef __cplusplus
extern "C"
{
#endif

    typedef void(CALLBACK* NotificationHandler)(_In_ GUID connectionId, _In_ void* pContextData);

    // General note about the memory allocated for the _Out_ parameters:
    //
    // If memory is dynamically allocated for the _Out_ parameters, the memory
    // will be available until the caller makes the next call. If the caller
    // want to keep the content cross multiple calls, the caller must keep its
    // own copy.
    // To have all the allocated memory (allocated by this DLL) reclaimed,
    // the caller finally must call either EapHostPeerEndSession() or
    // EapHostPeerClearConnection().
    //
    // EAP_ERROR** _Out_ parameter is an exception. The caller must call
    // EapHostPeerFreeEapError() to free the memory.
    //
    //
    // Thread safety:
    // EapHostPeerInitialize() and EapHostPeerUninitialize() are always thread
    // safe.
    //
    // EapHostPeerFreeEapError() is not thread safe. There should be only one
    // thread being called to free the same EAP_ERROR*. And obviously it should
    // not be called more than once for the same EAP_ERROR*.
    //
    // For all the other functions, there should only be one thread calling them
    // for the same session. However, it is safe for multiple threads to call them
    // for different sessions.
    //
    // If the functions succeed, they return ERROR_SUCCESS. Otherwise, it is
    // considered failure.
    //
    // If an error code is returned, the out parameter ppEapError will contain
    // related error information. The caller is expected to look into it to
    // find out the error informations.
    // However, there are some situation that it may not be possible to fill
    // out information for ppEapError. For example, in out of memory situation,
    // the method implementation may not be able to allocate memory for
    // ppEapError as well. If for any reason it is not possible to fill ppEapError,
    // *ppEapError should be set to NULL.

    // Must call this before it starts to call any other fucntion.
    // If this function fails, no other EapHostPeer... functions can be called.
    //
    // Inside this function, it calls CoInitializeEx(NULL, COINIT_MULTITHREADED)
    // So the client should not initialize a conflicting COM environment.
    // If the client really needs a different COM environment, it should use the
    // COM APIs directly instead of using this C style APIs.
    DWORD APIENTRY EapHostPeerInitialize();

    // Must call this after it is done calling any other EapHostPeer...
    // functions.
    // Inside this function it calls CoUninitialize().
    void APIENTRY EapHostPeerUninitialize();

    // If this function succeeds, no matter the calls to other functions
    // succeed or fail, the caller MUST call EapHostPeerEndSession() to end
    // the authentication session. if EapHostPeerBeginSession() provides a
    // valid GUID to register the connectionand, then EapHostPeerClearConnection()
    // must be called at the very end (i.e. there will be no more authentication
    // on this GUID) to remove the connection. The connection GUID
    // is valid cross multiple authentications.
    //
    // However, if this function fails, the caller can not proceed with
    // other calls. It should NOT call EapHostPeerEndSession() or
    // EapHostPeerClearConnection() if this function fails.
    //
    // For the same pConnectionId, EapHostPeerEndSession() must be called
    // before EapHostPeerBeginSession() can be called again.
    //
    // Parameters:
    // - dwFlags:
    //   EAP_FLAG_xxx defined in eaptypes.w
    //
    // - eapType:
    //   the EAP method type to be used for the authentication.
    //
    // - pAttributeArray:
    //   attributes passed to the method.
    //
    // - hTokenImpersonateUser:
    //   Impersonation token for the user to be authenticated.
    //
    // - dwSizeofConnectionData:
    //   size of the buffer (pConnectionData) in bytes
    //
    // - pConnectionData:
    //   See the comment for pConnectionData for EapPeerGetIdentity()
    //
    // - dwSizeofUserData:
    //   size of the buffer (pUserData) in bytes
    //
    // - pUserData:
    //   user data returned from EapPeerGetIdentity()
    //
    // - dwMaxSendPacketSize:
    //   maximum packet size that the method can send. If the method needs to
    //   send a message larger than this size, the method has to handle fragmentation
    //   and resembling.
    //
    // - pConnectionId:
    //   It can be NULL. Otherwise, it should point to a valid GUID which respresent
    //   the connection. When re-authentication is needed due to system health change,
    //   the callback function func will be called and this GUID pointer will
    //   be passed as an _In_ parameter to indicate which connection the callback
    //   is for.
    //
    // - func:
    //   callback function pointer used for re-authentication. This function
    //   will be called when system health changes and a re-authentication is needed.
    //   if the function handler is NULL, pContextData will be ignored,
    //   and it means the caller is not interested in system health change
    //   notification
    //
    // - pSessionId:
    //   Return session ID in it. This is used to identify the authentication session.
    DWORD APIENTRY EapHostPeerBeginSession(
        _In_ DWORD dwFlags,
        _In_ EAP_METHOD_TYPE eapType,
        _In_ const EapAttributes* const pAttributeArray,
        _In_ HANDLE hTokenImpersonateUser,
        _In_ DWORD dwSizeofConnectionData,
        _In_reads_(dwSizeofConnectionData) const BYTE* const pConnectionData,
        _In_ DWORD dwSizeofUserData,
        _In_reads_(dwSizeofUserData) const BYTE* const pUserData,
        _In_ DWORD dwMaxSendPacketSize,
        // If the supplicant is intrested in re-auth caused by NAP health change,
        // it should provide a unique GUID.
        // When this function is called by PEAP inner method, it will be NULL.
        //
        // When pConnectionId is NULL, func and pContextData will be ignored.
        _In_opt_ const GUID* const pConnectionId,
        // if the function handler is NULL, pContextData will be ignored,
        // and it means the caller is not interested in SoH change notification
        // from EapQec.
        _In_opt_ NotificationHandler func,
        // a pointer to some data that the supplicant want to associate with
        // the connection when NotificationHandler call back is called.
        // When NotificationHandler is called, it will be called as:
        // func(*pCOnnectionId, pContextData).
        _In_opt_ void* pContextData,
        _Out_ EAP_SESSIONID* pSessionId,
        _Outptr_ EAP_ERROR** ppEapError);

    // Supplicant will pass the packet to EAPHost for processing.
    // Parameters:
    // - sessionHandle:
    //   the value returned through *pSessionId from EapHostPeerBeginSession()
    //
    // - cbReceivePacket:
    //   buffer size of (pReceivePacket) in bytes
    //
    // - pReceivePacket:
    //   pointer to received packet
    //
    // - pEapOutput:
    //   It will tell the supplicant what's the next EAPHost API that it should call.
    DWORD APIENTRY EapHostPeerProcessReceivedPacket(
        _In_ EAP_SESSIONID sessionHandle,
        _In_ DWORD cbReceivePacket,
        _In_reads_(cbReceivePacket) const BYTE* const pReceivePacket,
        _Out_ EapHostPeerResponseAction* pEapOutput,
        _Outptr_ EAP_ERROR** ppEapError);

    // This function returns the packet to be sent. Supplicant should send the
    // returned packet to server.
    //
    // Parameters:
    // - sessionHandle:
    //   the value returned through *pSessionId from EapHostPeerBeginSession()
    //
    // - pcbSendPacket:
    //   Upon return, *pcbSendPacket is the sizeof the actual content in pSendPacket
    //
    // - ppSendPacket:
    //   pointer to a buffer that contains the packet to send upon return
    DWORD APIENTRY EapHostPeerGetSendPacket(
        _In_ EAP_SESSIONID sessionHandle,
        _Out_ DWORD* pcbSendPacket,
        _Outptr_result_buffer_(*pcbSendPacket) BYTE** ppSendPacket,
        _Outptr_ EAP_ERROR** ppEapError);

    // Supplicant calls this function to retrieve authentication result.
    // Parameters:
    // - sessionHandle:
    //   the value returned through *pSessionId from EapHostPeerBeginSession()
    //
    // - reason:
    //   One of the reason code defined in eaphostPeerTypes.h
    //
    // - ppResult:
    //   EAPHost will fill this struct with related information as defined
    //   by EapHostPeerMethodResult
    DWORD APIENTRY EapHostPeerGetResult(
        _In_ EAP_SESSIONID sessionHandle,
        _In_ EapHostPeerMethodResultReason reason,
        _Out_ EapHostPeerMethodResult* ppResult,
        _Outptr_ EAP_ERROR** ppEapError);

    // This will get called if UI should be raised during authentication session.
    // This function will always be followed by EapHostPeerInvokeInteractiveUI()
    // and then followed by EapHostPeerSetUIContext()
    // Parameters:
    // - sessionHandle:
    //   the value returned through *pSessionId from EapHostPeerBeginSession()
    //
    // - pdwSizeOfUIContextData:
    //   *pdwSizeOfUIContextData is the size of *ppUIContextData in bytes.
    //
    // - ppUIContextData:
    //   It will be passed to EapHostPeerInvokeInteractiveUI() as _In_ parameter
    //   pUIContextData
    DWORD APIENTRY EapHostPeerGetUIContext(
        _In_ EAP_SESSIONID sessionHandle,
        _Out_ DWORD* pdwSizeOfUIContextData,
        _Outptr_result_buffer_(*pdwSizeOfUIContextData) BYTE** ppUIContextData,
        _Outptr_ EAP_ERROR** ppEapError);

    // It will be called after UI has been raised. Refer to EapHostPeerGetUIContext()
    // for more information.
    //
    // Parameters:
    // - sessionHandle:
    //   the value returned through *pSessionId from EapHostPeerBeginSession()
    //
    // - dwSizeOfUIContextData:
    //   size of pUIContextData in bytes.
    //
    // - pUIContextData:
    //   This is the data returned from EapHostPeerInvokeInteractiveUI() _Out_
    //   parameter
    //   *ppDataFromInteractiveUI
    //
    // - pEapOutput:
    //   It will tell the supplicant what's the next EAPHost API that it should call.
    DWORD APIENTRY EapHostPeerSetUIContext(
        _In_ EAP_SESSIONID sessionHandle,
        _In_ DWORD dwSizeOfUIContextData,
        _In_reads_(dwSizeOfUIContextData) const BYTE* const pUIContextData,
        _Out_ EapHostPeerResponseAction* pEapOutput,
        _Outptr_ EAP_ERROR** ppEapError);

    // Return EAP attributes to supplicant. Supplicant should call
    // EapHostPeerSetResponseAttributes() after it calls this function and finishes
    // processing of the EAP attributes.
    //
    // Parameters:
    // - sessionHandle:
    //   the value returned through *pSessionId from EapHostPeerBeginSession()
    //
    // - pAttributes:
    //   EAP attributes returned to supplicant.
    DWORD APIENTRY EapHostPeerGetResponseAttributes(_In_ EAP_SESSIONID sessionHandle, _Out_ EapAttributes* pAttribs, _Outptr_ EAP_ERROR** ppEapError);

    // See EapHostPeerGetResponseAttributes() for when this function will be called.
    // Even if supplicant doesn't care about or doesn't understand the attributes
    // returned from EapHostPeerGetResponseAttributes(), it should still call
    // this function and pass a valid EapAttributes* which contains 0 EapAttribute
    //
    // Parameters:
    // - sessionHandle:
    //   the value returned through *pSessionId from EapHostPeerBeginSession()
    //
    // - pAttribs:
    //   EAP attributes that the supplicant passes to EAPHost
    DWORD APIENTRY EapHostPeerSetResponseAttributes(
        _In_ EAP_SESSIONID sessionHandle,
        _In_ const EapAttributes* const pAttribs,
        _Out_ EapHostPeerResponseAction* pEapOutput,
        _Outptr_ EAP_ERROR** ppEapError);

    // Supplicant calls it to retrieve authentication status.
    //
    // Parameters:
    // - sessionHandle:
    //   the value returned through *pSessionId from EapHostPeerBeginSession()
    //
    // - authParam:
    //   one of the EapHostPeerAuthParams values defined in Eaphostpeertypes.h
    //
    // - pcbAuthData:
    //   *pcbAuthData is the size of *ppAuthData in bytes
    //
    // - ppAuthData:
    //   contains the returned authentication data. Interpretation of the data
    //   depends on the "authParam" passed in
    DWORD APIENTRY EapHostPeerGetAuthStatus(
        _In_ EAP_SESSIONID sessionHandle,
        _In_ EapHostPeerAuthParams authParam,
        _Out_ DWORD* pcbAuthData,
        _Outptr_result_buffer_(*pcbAuthData) BYTE** ppAuthData,
        _Outptr_ EAP_ERROR** ppEapError);

    // Authentication has finished so clear up the data stored for this session.
    // After this call, the session will no longer be valid.
    //
    // Parameters:
    // - sessionHandle:
    //   the value returned through *pSessionId from EapHostPeerBeginSession()
    DWORD APIENTRY EapHostPeerEndSession(_In_ EAP_SESSIONID sessionHandle, _Outptr_ EAP_ERROR** ppEapError);

    DWORD APIENTRY EapHostPeerGetDataToUnplumbCredentials(
        _Out_ GUID* pConnectionIdThatLastSavedCreds,
        _Out_ __int3264* phCredentialImpersonationToken,
        _In_ EAP_SESSIONID sessionHandle,
        _Outptr_ EAP_ERROR** ppEapError,
        _Out_ BOOL* fSaveToCredMan);

    // Clear up the connection. After this call, all states associated with this
    // pConnectionId will be deleted, and no re-authentication will be triggered
    // for connection associated with this GUID.
    //
    // Parameters:
    // - pConnectionId:
    //   the same connection GUID passed into EapHostPeerBeginSession(). If
    //   a NULL is passed to EapHostPeerBeginSession() as the connection id,
    //   then there is no need to call this API.
    DWORD APIENTRY EapHostPeerClearConnection(_In_ GUID* pConnectionId, _Outptr_ EAP_ERROR** ppEapError);

    // If any of the previous function is called, and an EAP_ERROR is returned,
    // this function must be called to free the memory.
    void APIENTRY EapHostPeerFreeEapError(_In_ EAP_ERROR* pEapError);

    // EAPHost will call this function, and it would return user data and user identity.

    DWORD APIENTRY EapHostPeerGetIdentity(
        // API's version number for ease of interoperability. It must be 0.
        _In_ DWORD dwVersion,
        // EAP_FLAG_xxx defined in eaptypes.w
        _In_ DWORD dwFlags,
        // Eap method type
        _In_ EAP_METHOD_TYPE eapMethodType,
        // size of the buffer (pConnectionData) in bytes
        _In_ DWORD dwSizeofConnectionData,
        // configuration data that is used for the EAP method
        _In_reads_(dwSizeofConnectionData) const BYTE* pConnectionData,
        // size of the ubuffer pUserData
        _In_ DWORD dwSizeofUserData,
        // user credential information pertinent to this auth
        _In_reads_opt_(dwSizeofUserData) const BYTE* pUserData,
        // impersonation token for logged on user, to obtain user-related information
        _In_ HANDLE hTokenImpersonateUser,
        // should return true if the user identity and user data blob can not be returned
        // successfully and in needs to be collected through UI
        _Out_ BOOL* pfInvokeUI,
        // size of the buffer pUserDataOut. caller should free this using EapHostPeerFreeRuntimeMemory
        _Out_ DWORD* pdwSizeOfUserDataOut,
        // user data information returned by method.
        _Outptr_result_buffer_(*pdwSizeOfUserDataOut) BYTE** ppUserDataOut,
        // identity returned by method. caller should free this using EapHostPeerFreeRuntimeMemory
        _Outptr_ LPWSTR* ppwszIdentity,
        // In case of error, API fills ppEapError if possible. Caller should free ppEapError
        // using EapHostPeerFreeErrorMemory
        _Outptr_ EAP_ERROR** ppEapError,
        // Reserved for future usage. It must be NULL.
        _Inout_ BYTE** ppvReserved);

    DWORD APIENTRY EapHostPeerGetEncryptedPassword(
        _In_ DWORD dwSizeofPassword, _In_reads_bytes_(dwSizeofPassword) LPWSTR szPassword, _Outptr_ LPWSTR* ppszEncPassword);

    // In case of GetInentity returned memory needs to be released using this API
    void APIENTRY EapHostPeerFreeRuntimeMemory(_In_ BYTE* pData);

#ifdef __cplusplus
}
#endif

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion

#endif // EAPPAPIS_H
