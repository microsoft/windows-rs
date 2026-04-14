///////////////////////////////////////////////////////////////////////////////
//
// Copyright (c) Microsoft Corporation
//
// SYNOPSIS
//
//   Declares datastructures that are needed by eap methods on the Peer.
//
///////////////////////////////////////////////////////////////////////////////

#ifndef EAPPEERMETHODAPIS_H
#define EAPPEERMETHODAPIS_H
#pragma once
#include <winapifamily.h>

#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)

#include "objbase.h"
#include "msxml6.h"
#include "EapMethodTypes.h"

#ifndef EAPAUTHENTICATORACTIONDEFINE_H
#include "EapAuthenticatorActionDefine.h"
#endif

#ifndef _NGCTICKETCONTEXT_
#define _NGCTICKETCONTEXT_
// The NCRYPT_PIN_CACHE_PIN_PROPERTY and NCRYPT_PIN_CACHE_APPLICATION_TICKET_PROPERTY properties
// return a 32 byte random unique ID encoded as a null terminated base64 Unicode string. The string length
// is 32 * 4/3 + 1 characters = 45 characters, 90 bytes
#define NCRYPT_PIN_CACHE_PIN_BYTE_LENGTH 90

#define NGC_TICKET_PROPERTY_STRING_LENGTH (NCRYPT_PIN_CACHE_PIN_BYTE_LENGTH / sizeof(WCHAR))

typedef ULONG_PTR NCRYPT_KEY_HANDLE;

typedef struct _NgcTicketContext
{
    WCHAR wszTicket[NGC_TICKET_PROPERTY_STRING_LENGTH];
    NCRYPT_KEY_HANDLE hKey;
    HANDLE hImpersonateToken;
} NgcTicketContext;

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
    //      PeerDllPath             (REG_EXPAND_SZ) %SystemRoot%\system32\sample.dll
    //      PeerConfigUIPath        (REG_EXPAND_SZ) %SystemRoot%\system32\sample.dll
    //      PeerInteractiveUIPath   (REG_EXPAND_SZ) %SystemRoot%\system32\sample.dll
    //      PeerIdentityPath        (REG_EXPAND_SZ) %SystemRoot%\system32\sample.dll
    //      PeerFriendlyName        (REG_SZ) Sample EAP Protocol
    //      PeerRequireConfigUI     (REG_DWORD)     1

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
    //      PeerDllPath             (REG_EXPAND_SZ) %SystemRoot%\system32\sample.dll
    //      PeerConfigUIPath        (REG_EXPAND_SZ) %SystemRoot%\system32\sample.dll
    //      PeerInteractiveUIPath   (REG_EXPAND_SZ) %SystemRoot%\system32\sample.dll
    //      PeerIdentityPath        (REG_EXPAND_SZ) %SystemRoot%\system32\sample.dll
    //      PeerFriendlyName        (REG_SZ) Sample EAP Protocol
    //      PeerRequireConfigUI     (REG_DWORD)     1

#define EAP_REGISTRY_LOCATION TEXT("System\\CurrentControlSet\\Services\\EapHost\\Methods")

#define EAP_PEER_VALUENAME_DLL_PATH TEXT("PeerDllPath")
#define EAP_PEER_VALUENAME_FRIENDLY_NAME TEXT("PeerFriendlyName")
#define EAP_PEER_VALUENAME_CONFIGUI TEXT("PeerConfigUIPath")
#define EAP_PEER_VALUENAME_REQUIRE_CONFIGUI TEXT("PeerRequireConfigUI")
#define EAP_PEER_VALUENAME_IDENTITY TEXT("PeerIdentityPath")
#define EAP_PEER_VALUENAME_INTERACTIVEUI TEXT("PeerInteractiveUIPath")
#define EAP_PEER_VALUENAME_INVOKE_NAMEDLG TEXT("PeerInvokeUsernameDialog")
#define EAP_PEER_VALUENAME_INVOKE_PWDDLG TEXT("PeerInvokePasswordDialog")
#define EAP_PEER_VALUENAME_PROPERTIES TEXT("Properties")

    // This structure is used in EapPeerGetResult API to convey the authentication result and pass
    // additional information to the EapHost and Supplicant.
    typedef struct tagEapPeerMethodResult
    {
        BOOL fIsSuccess;
        DWORD dwFailureReasonCode;
        BOOL fSaveConnectionData;
        DWORD dwSizeofConnectionData;
        _Field_size_bytes_(dwSizeofConnectionData) BYTE* pConnectionData;
        BOOL fSaveUserData;
        DWORD dwSizeofUserData;
        _Field_size_bytes_(dwSizeofUserData) BYTE* pUserData;
        EAP_ATTRIBUTES* pAttribArray;
        EAP_ERROR* pEapError;
        NgcTicketContext* pNgcKerbTicket;
        BOOL fSaveToCredMan;
    } EapPeerMethodResult;

    // List of functions corresponding to the particular EAP method
    typedef struct _EAP_PEER_METHOD_ROUTINES
    {
        DWORD dwVersion;
        EAP_TYPE* pEapType;

        DWORD(APIENTRY* EapPeerInitialize)(_Outptr_ EAP_ERROR** ppEapError);

        DWORD(APIENTRY* EapPeerGetIdentity)(
            _In_ DWORD flags,
            _In_ DWORD dwSizeofConnectionData,
            _In_reads_(dwSizeofConnectionData) const BYTE* pConnectionData,
            _In_ DWORD dwSizeofUserData,
            _In_reads_(dwSizeofUserData) const BYTE* pUserData,
            _In_ HANDLE hTokenImpersonateUser,
            _Out_ BOOL* pfInvokeUI,
            _Out_ DWORD* pdwSizeOfUserDataOut,
            _Outptr_result_buffer_(*pdwSizeOfUserDataOut) BYTE** ppUserDataOut,
            _Outptr_ LPWSTR* ppwszIdentity,
            _Outptr_ EAP_ERROR** ppEapError);

        DWORD(APIENTRY* EapPeerBeginSession)(
            _In_ DWORD dwFlags,
            _In_ const EapAttributes* const pAttributeArray,
            _In_ HANDLE hTokenImpersonateUser,
            _In_ DWORD dwSizeofConnectionData,
            _In_reads_(dwSizeofConnectionData) BYTE* pConnectionData,
            _In_ DWORD dwSizeofUserData,
            _In_reads_(dwSizeofUserData) BYTE* pUserData,
            _In_ DWORD dwMaxSendPacketSize,
            _Out_ EAP_SESSION_HANDLE* pSessionHandle,
            _Outptr_ EAP_ERROR** ppEapError);

        DWORD(APIENTRY* EapPeerSetCredentials)(
            _In_ EAP_SESSION_HANDLE sessionHandle, _In_ WCHAR* pwszIdentity, _In_ WCHAR* pwszPassword, _Outptr_ EAP_ERROR** ppEapError);

        DWORD(APIENTRY* EapPeerProcessRequestPacket)(
            _In_ EAP_SESSION_HANDLE sessionHandle,
            _In_ DWORD cbReceivePacket,
            _In_reads_bytes_(cbReceivePacket) EapPacket* pReceivePacket,
            _Out_ EapPeerMethodOutput* pEapOutput,
            _Outptr_ EAP_ERROR** ppEapError);

        DWORD(APIENTRY* EapPeerGetResponsePacket)(
            _In_ EAP_SESSION_HANDLE sessionHandle,
            _Inout_ _Pre_satisfies_(*pcbSendPacket >= sizeof(EapPacket)) DWORD* pcbSendPacket,
            _Inout_ _Pre_writable_byte_size_(*pcbSendPacket) _Pre_readable_byte_size_(*pcbSendPacket) EapPacket* pSendPacket,
            _Outptr_ EAP_ERROR** ppEapError);

        DWORD(APIENTRY* EapPeerGetResult)(
            _In_ EAP_SESSION_HANDLE sessionHandle,
            _In_ EapPeerMethodResultReason reason,
            _Out_ EapPeerMethodResult* pResult,
            _Outptr_ EAP_ERROR** ppEapError);

        DWORD(APIENTRY* EapPeerGetUIContext)(
            _In_ EAP_SESSION_HANDLE sessionHandle,
            _Out_ DWORD* pdwSizeOfUIContextData,
            _Outptr_result_buffer_(*pdwSizeOfUIContextData) BYTE** ppUIContextData,
            _Outptr_ EAP_ERROR** ppEapError);

        DWORD(APIENTRY* EapPeerSetUIContext)(
            _In_ EAP_SESSION_HANDLE sessionHandle,
            _In_ DWORD dwSizeOfUIContextData,
            _In_reads_(dwSizeOfUIContextData) const BYTE* pUIContextData,
            _Out_ EapPeerMethodOutput* pEapOutput,
            _Outptr_ EAP_ERROR** ppEapError);

        DWORD(APIENTRY* EapPeerGetResponseAttributes)(
            _In_ EAP_SESSION_HANDLE sessionHandle, _Out_ EapAttributes* pAttribs, _Outptr_ EAP_ERROR** ppEapError);

        DWORD(APIENTRY* EapPeerSetResponseAttributes)(
            _In_ EAP_SESSION_HANDLE sessionHandle,
            _In_ EapAttributes* pAttribs,
            _Out_ EapPeerMethodOutput* pEapOutput,
            _Outptr_ EAP_ERROR** ppEapError);

        DWORD(APIENTRY* EapPeerEndSession)(_In_ EAP_SESSION_HANDLE sessionHandle, _Outptr_ EAP_ERROR** ppEapError);

        DWORD(APIENTRY* EapPeerShutdown)(_Outptr_ EAP_ERROR** ppEapError);
    } EAP_PEER_METHOD_ROUTINES;

    //
    // EAP Functions
    //

    // General comment regarding all APIs defined in this file:
    //
    // If the function succeeds, it returns ERROR_SUCCESS. Otherwise, it is
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

    // This function should be an exported function from the DLL
    //
    // If any other APIs in this file returns EAP_ERROR, use this
    // function to free the memory allocated for it
    VOID WINAPI EapPeerFreeErrorMemory(_In_ EAP_ERROR* pEapError);

    // This function should be an exported function from the DLL
    //
    // Returns information related to a given EAP type.
    // Parameters:
    // - pEapType:
    //   the EAP type for this EAP method. Implementation should check this input
    //   to make sure that it is the same as the method being implemented.
    //
    // - pEapInfo:
    //   returns all the information related for pEapType.
    DWORD WINAPI EapPeerGetInfo(_In_ EAP_TYPE* pEapType, _Out_ EAP_PEER_METHOD_ROUTINES* pEapInfo, _Out_opt_ EAP_ERROR** ppEapError);

    // This is the first function that EAPHost should call on this method.
    // The only exception is EapPeerFreeErrorMemory() and EapPeerGetInfo(),
    // which can be called at any time.
    // This function should be called only once and it should initialize
    // everything needed for this method work properly.
    DWORD WINAPI EapPeerInitialize(_Outptr_ EAP_ERROR** ppEapError);

    // EAPHost will call this function, and it would return user data
    // and user identity.
    // Parameters:
    // - dwFlags:
    //   EAP_FLAG_xxx defined in eaptypes.w
    //
    // - dwSizeofConnectionData:
    //   size of the buffer (pConnectionData) in bytes
    //
    // - pConnectionData:
    //   connection data specific to this method. It will be used to decide
    //   the user data returned from this API, when the user data depends on
    //   certain connection data configuration. The method implementation should
    //   have default values for connection data, and when this parameter
    //   is NULL, the default connection data should be used.
    //
    // - dwSizeofUserData:
    //   size of the buffer (pUserData) in bytes.
    //
    // - pUserData:
    //   the user data specific to this authentication. It will be used to
    //   pre-populate the user data.
    //   When this API is called for the first time, or when it needs to start
    //   a fresh authentication sessin, this parameter will be NULL.
    //   When it is not NULL, its content should be the user data returned
    //   from EapPeerGetResult() (*ppResult)->pUserData from previous successful
    //   authentication session, if that session indicated that user data should be
    //   saved.
    //
    // - pfInvokeUI:
    //   should return true if the user identity and user data blob can not be
    //   returned successfully and the method decides that it has to collect the
    //   information from the user through UI.
    //
    // - pdwSizeOfUserDataOut:
    //   *pdwSizeOfUserDataOut is the size of buffer *ppUserDataOut
    //
    // - ppUserDataOut:
    //   returned user data. The data will be passed to EapPeerBeginSession()
    //   as input pUserData
    //
    // - ppwszIdentity:
    //   returned user identity. It will be included in the identity response packet
    //   and returned to the server.
    DWORD WINAPI EapPeerGetIdentity(
        _In_ DWORD dwFlags,
        _In_ DWORD dwSizeofConnectionData,
        _In_reads_(dwSizeofConnectionData) const BYTE* pConnectionData,
        _In_ DWORD dwSizeofUserData,
        _In_reads_(dwSizeofUserData) const BYTE* pUserData,
        _In_ HANDLE hTokenImpersonateUser,
        _Out_ BOOL* pfInvokeUI,
        _Out_ DWORD* pdwSizeOfUserDataOut,
        _Outptr_result_buffer_(*pdwSizeOfUserDataOut) BYTE** ppUserDataOut,
        _Out_ LPWSTR* ppwszIdentity,
        _Outptr_ EAP_ERROR** ppEapError);

    // Start an authentication session.
    // Parameters:
    // - dwFlags:
    //   EAP_FLAG_xxx defined in eaptypes.w
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
    // - pSessionHandle:
    //   This is an "identifier" of the authentication session.
    //   When other EapPeerXxxx() functions with a sessionHandle parameter
    //   being called, *pSessionHandle will be passed as the session handle.
    //   The method should be able to use this to find the information related
    //   the authentication session.
    DWORD WINAPI EapPeerBeginSession(
        _In_ DWORD dwFlags,
        _In_ const EapAttributes* const pAttributeArray,
        _In_ HANDLE hTokenImpersonateUser,
        _In_ DWORD dwSizeofConnectionData,
        _In_reads_(dwSizeofConnectionData) BYTE* pConnectionData,
        _In_ DWORD dwSizeofUserData,
        _In_reads_(dwSizeofUserData) BYTE* pUserData,
        _In_ DWORD dwMaxSendPacketSize,
        _Out_ EAP_SESSION_HANDLE* pSessionHandle,
        _Outptr_ EAP_ERROR** ppEapError);

    // A method exports either EapPeerGetIdentity (and EapPeerInvokeIdentityUI) or
    // exports EapPeerSetCredentials (and sets the InvokeUserNameDlg regkey). The
    // registry key controls which of the two apis will get called.
    DWORD WINAPI EapPeerSetCredentials(EAP_SESSION_HANDLE sessionHandle, LPWSTR pwszIdentity, LPWSTR pwszPassword, _Outptr_ EAP_ERROR** ppEapError);

    // EAPHost will pass the packet to the method for processing.
    // Parameters:
    // - sessionHandle:
    //   the value returned through *pSessionHandle from EapPeerBeginSession()
    //
    // - cbRecevedPacket:
    //   buffer size of (pReceivePacket) in bytes
    //
    // - pReceivedPacket:
    //   pointer to received packet
    //
    // - pEapOutput:
    //   the method should fill this struct to tell the supplicant what to do.
    DWORD WINAPI EapPeerProcessRequestPacket(
        _In_ EAP_SESSION_HANDLE sessionHandle,
        _In_ DWORD cbReceivedPacket,
        _In_reads_bytes_(cbReceivedPacket) EapPacket* pReceivedPacket,
        _Out_ EapPeerMethodOutput* pEapOutput,
        _Outptr_ EAP_ERROR** ppEapError);

    // This function returns the packet to be sent to the server.
    // Parameters:
    // - sessionHandle:
    //   the value returned through *pSessionHandle from EapPeerBeginSession()
    //
    // - pcbSendPacket:
    //   *pcbSendPacket is the size of the buffer for (pSendPacket) when passed in.
    //   Upon return, *pcbSendPacket is the sizeof the actual content in pSendPacket
    //
    // - pSendPacket:
    //   pointer to a buffer that contains the packet to send upon return
    DWORD WINAPI EapPeerGetResponsePacket(
        _In_ EAP_SESSION_HANDLE sessionHandle,
        _Inout_ _Pre_satisfies_(*pcbSendPacket >= sizeof(EapPacket)) DWORD* pcbSendPacket,
        _Inout_ _Pre_writable_byte_size_(*pcbSendPacket) _Pre_readable_byte_size_(*pcbSendPacket) EapPacket* pSendPacket,
        _Outptr_ EAP_ERROR** ppEapError);

    // This will get called either when a method says that it has completed auth.
    // or when the lower layer receives an alternative result.
    // parameters:
    // - sessionHandle:
    //   the value returned through *pSessionHandle from EapPeerBeginSession()
    //
    // - reason:
    //   one of the reason enum defined above in this file.
    //
    // - ppResult:
    //   Method should fill this struct with related information as defined
    //   by EapPeerMethodResult
    DWORD WINAPI EapPeerGetResult(
        _In_ EAP_SESSION_HANDLE sessionHandle,
        _In_ EapPeerMethodResultReason reason,
        _Out_ EapPeerMethodResult* pResult,
        _Outptr_ EAP_ERROR** ppEapError);

    // This will get called if UI should be raised during authentication session.
    // This function will always be followed by EapPeerInvokeInteractiveUI() and
    // then followed by EapPeerSetUIContext()
    // parameters:
    // - sessionHandle:
    //   the value returned through *pSessionHandle from EapPeerBeginSession()
    //
    // - pdwSizeOfUIContextData:
    //   *pdwSizeOfUIContextData is the size of *ppUIContextData in bytes.
    //
    // - ppUIContextData:
    //   It will be passed to EapPeerInvokeInteractiveUI() as _In_ parameter
    //   pUIContextData
    DWORD WINAPI EapPeerGetUIContext(
        _In_ EAP_SESSION_HANDLE sessionHandle,
        _Out_ DWORD* pdwSizeOfUIContextData,
        _Outptr_result_buffer_(*pdwSizeOfUIContextData) BYTE** ppUIContextData,
        _Outptr_ EAP_ERROR** ppEapError);

    // It will be called after UI has been raised. Refer to EapPeerGetUIContext()
    // for more information.
    // parameters:
    // - sessionHandle:
    //   the value returned through *pSessionHandle from EapPeerBeginSession()
    //
    // - dwSizeOfUIContextData:
    //   size of pUIContextData in bytes.
    //
    // - pUIContextData:
    //   This is the data returned from EapPeerInvokeInteractiveUI() OUT parameter
    //   *ppDataFromInteractiveUI
    //
    // - pEapOutput:
    //   the method should fill this struct to tell the supplicant what to do.
    DWORD WINAPI EapPeerSetUIContext(
        _In_ EAP_SESSION_HANDLE sessionHandle,
        _In_ DWORD dwSizeOfUIContextData,
        _In_reads_(dwSizeOfUIContextData) const BYTE* pUIContextData,
        _Out_ EapPeerMethodOutput* pEapOutput,
        _Outptr_ EAP_ERROR** ppEapError);

    // If EAPHost calls any of the EapPeerXxxx() function, and that function has
    // pEapOutput as a OUT parameter, and the action code in it is
    // EapPeerMethodResponseActionRespond, this function will be the next function
    // that EAPHost will call.
    // And it will be follow by a call to EapPeerSetResponseAttributes()
    // parameters:
    // - sessionHandle:
    //   the value returned through *pSessionHandle from EapPeerBeginSession()
    //
    // - pAttributes:
    //   method should fill this with the attributes it need to return.
    DWORD WINAPI EapPeerGetResponseAttributes(_In_ EAP_SESSION_HANDLE sessionHandle, _Out_ EapAttributes* pAttribs, _Outptr_ EAP_ERROR** ppEapError);

    // See EapPeerGetResponseAttributes() for when this function will be called.
    // parameters:
    // - sessionHandle:
    //   the value returned through *pSessionHandle from EapPeerBeginSession()
    //
    // - pAttributes:
    //   attributes passed to this method.
    //
    // - pEapOutput:
    //   the method should fill this struct to tell the supplicant what to do.
    DWORD WINAPI EapPeerSetResponseAttributes(
        _In_ EAP_SESSION_HANDLE sessionHandle, _In_ EapAttributes* pAttribs, _Out_ EapPeerMethodOutput* pEapOutput, _Outptr_ EAP_ERROR** ppEapError);

    // The last function that will be called for this authentication session.
    // - sessionHandle:
    //   the value returned through *pSessionHandle from EapPeerBeginSession()
    //
    DWORD WINAPI EapPeerEndSession(_In_ EAP_SESSION_HANDLE sessionHandle, _Outptr_ EAP_ERROR** ppEapError);

    // This is the last function that EAPHost should call on this method.
    // The only exception is EapPeerFreeErrorMemory() and EapPeerGetInfo(),
    // which can be called at any time.
    // This function should be called only once and it should un-initialize
    // everything for this method.
    DWORD WINAPI EapPeerShutdown(_Outptr_ EAP_ERROR** ppEapError);

    // It should raise a configuration UI so that the user can configure the method
    // parameters:
    // - pEapType:
    //   method type for this method
    //
    // - hwndParent:
    //   The parent window for the UI to be raised by this function.
    //
    // - dwFlags:
    //   EAP_FLAG_xxx defined in eaptypes.w
    //
    // - dwSizeOfConnectionDataIn:
    //   size of pConnectionDataIn in bytes
    //
    // - pConnectionDataIn:
    //   Used to pre-populate the configuration UI. The first time this function
    //   is called, or if it is desired to start from scratch, it will be NULL,
    //   otherwise, it will be the data returned from previous
    //   EapPeerInvokeConfigUI() ppConnectionDataOut OUT parameter
    //
    // - pdwSizeOfConnectionDataOut:
    //   *pdwSizeOfConnectionDataOut is the size of *ppConnectionDataOut
    //
    // - ppConnectionDataOut:
    //   *ppConnectionDataOut is configuration data returned from this functioin.
    //   It will be used to pass to next EapPeerInvokeConfigUI() call.
    DWORD WINAPI EapPeerInvokeConfigUI(
        _In_ EAP_METHOD_TYPE* pEapType,
        _In_ HWND hwndParent,
        _In_ DWORD dwFlags,
        _In_ DWORD dwSizeOfConnectionDataIn,
        _In_reads_(dwSizeOfConnectionDataIn) BYTE* pConnectionDataIn,
        _Out_ DWORD* dwSizeOfConnectionDataOut,
        _Outptr_result_buffer_(*dwSizeOfConnectionDataOut) BYTE** ppConnectionDataOut,
        _Outptr_ EAP_ERROR** ppEapError);

    // This function should raise interactive UI for the Method. What UI to be raised and how to raise
    // is Method specific information that is embedded in pUIContextData.
    //
    // Parameters:
    // - pEapType:
    //   method type for this method
    //
    // - hwndParent:
    //   The parent window for the UI to be raised by this function.
    //
    // - dwFlags:
    //   EAP_FLAG_xxx defined in eaptypes.w
    // - dwSizeofUIContextData:
    //   Number of bytes of UIContext data referred by pUIContextData.
    // - pUIContextData:
    //   A blob that conveys the state of the authentication and describes what and how to show the UI
    //   to the user.
    // - pdwSizeOfDataFromInteractiveUI
    //   Number of bytes of blob referred by *ppDataFromInteractiveUI.
    // - ppDataFromInteractiveUI
    //   A UI context blob that captures the result of UI interaction
    // - pEapError
    //   Pointer to pointer to EAP_ERROR that will be filled by the Method in the case of error. In some cases
    //  the structure might not be filled.
    DWORD WINAPI EapPeerInvokeInteractiveUI(
        _In_ EAP_METHOD_TYPE* pEapType,
        _In_ HWND hwndParent,
        _In_ DWORD dwSizeofUIContextData,
        _In_reads_(dwSizeofUIContextData) BYTE* pUIContextData,
        _Out_ DWORD* pdwSizeOfDataFromInteractiveUI,
        _Outptr_result_buffer_(*pdwSizeOfDataFromInteractiveUI) BYTE** ppDataFromInteractiveUI,
        _Outptr_ EAP_ERROR** ppEapError);

    // This function should raise credential UI for the Method. Method can use the configuration passed in to
    // decide what kind of UI should be shown to the user.
    //
    // Parameters:
    // - pEapType:
    //   method type for this method
    //
    // - dwFlags:
    //   EAP_FLAG_xxx defined in eaptypes.w
    //
    // - hwndParent:
    //   The parent window for the UI to be raised by this function.
    //
    // - dwSizeOfConnectionData:
    //   Number of bytes of pConnectionData.
    //
    // - pConnectionData:
    //   Configuration blob for the Method.
    //
    // - dwSizeOfUserData
    //   Number of bytes of pUserData.
    //
    // - pUserData
    //   Cached credential blob. This would be NULL if this is the first time successful authentication is being done.
    //
    // - ppwszIdentity
    //   Pointer to unicode string that receives the identity that EapHost should use as part of EAP Identity reponse.
    //
    // - pEapError
    //   Pointer to pointer to EAP_ERROR that will be filled by the Method in the case of error. In some cases
    //  the structure might not be filled.
    DWORD WINAPI EapPeerInvokeIdentityUI(
        _In_ EAP_METHOD_TYPE* pEapType,
        _In_ DWORD dwFlags,
        _In_ HWND hwndParent,
        _In_ DWORD dwSizeOfConnectionData,
        _In_reads_(dwSizeOfConnectionData) const BYTE* pConnectionData,
        _In_ DWORD dwSizeOfUserData,
        _In_reads_(dwSizeOfUserData) const BYTE* pUserData,
        _Out_ DWORD* pdwSizeOfUserDataOut,
        _Outptr_result_buffer_(*pdwSizeOfDataFromInteractiveUI) BYTE** ppUserDataOut,
        _Out_ LPWSTR* ppwszIdentity,
        _Outptr_ EAP_ERROR** ppEapError);

    // This function returns credential fields to be shown to the user for the authentication. As part of this function
    // Method should not bring up UI instead it should describe what credentials are to be obtained. Based on the
    // return values, Supplicant draws the UI to the user and obtains the requested credentials.
    //
    // Parameters:
    // - hUserImpersonationToken:
    //   Impersonation token of the user/machine who is being authenticated.
    //
    // - eapMethodType
    //   method type for this method
    //
    // - dwFlags:
    //   EAP_FLAG_xxx defined in eaptypes.w
    //
    // - dwEapConnDataSize:
    //   Number of bytes of pbEapConnData.
    //
    // - pbEapConnData:
    //   Configuration blob for the Method.
    //
    // - dwSizeOfUserData
    //   Number of bytes of pUserData.
    //
    // - pEapConfigFieldsArray
    //   Pointer to EAP_CONFIG_INPUT_FIELD_ARRAY that receives array of credential fields.
    //
    // - pEapError
    //   Pointer to pointer to EAP_ERROR that will be filled by the Method in the case of error. In some cases
    //  the structure might not be filled.
    DWORD WINAPI EapPeerQueryCredentialInputFields(
        _In_ HANDLE hUserImpersonationToken,
        _In_ EAP_METHOD_TYPE eapMethodType,
        _In_ DWORD dwFlags,
        _In_ DWORD dwEapConnDataSize,
        _In_reads_(dwEapConnDataSize) BYTE* pbEapConnData,
        _Out_ EAP_CONFIG_INPUT_FIELD_ARRAY* pEapConfigFieldsArray,
        _Outptr_ EAP_ERROR** ppEapError);

    // This function returns credential blob based on the credentials passed in the credential array structure.
    // Method should not bring up UI. It should convert the credentials in to a blob that could be passed to
    // EapPeerBeginSession.
    //
    // Parameters:
    // - hUserImpersonationToken:
    //   Impersonation token of the user/machine who is being authenticated.
    //
    // - eapMethodType
    //   method type for this method
    //
    // - dwFlags:
    //   EAP_FLAG_xxx defined in eaptypes.w
    //
    // - dwEapConnDataSize:
    //   Number of bytes of pbEapConnData.
    //
    // - pbEapConnData:
    //   Configuration blob for the Method.
    //
    // - dwSizeOfUserData
    //   Number of bytes of pUserData.
    //
    // - pEapConfigFieldsArray
    //   Pointer to EAP_CONFIG_INPUT_FIELD_ARRAY that contains array of credential fields filled by supplicant.
    //
    // - pdwUserBlobSize:
    //  Pointer to DWORD that receives the size of the ppbUserBlob.
    //
    // - ppbUserBlob
    //  Pointer to buffer that receives credential blob.
    //
    // - pEapError
    //   Pointer to pointer to EAP_ERROR that will be filled by the Method in the case of error. In some cases
    //  the structure might not be filled.
    DWORD WINAPI EapPeerQueryUserBlobFromCredentialInputFields(
        _In_ HANDLE hUserImpersonationToken,
        _In_ EAP_METHOD_TYPE eapMethodType,
        _In_ DWORD dwFlags,
        _In_ DWORD dwEapConnDataSize,
        _In_reads_(dwEapConnDataSize) BYTE* pbEapConnData,
        _In_ const EAP_CONFIG_INPUT_FIELD_ARRAY* pEapConfigInputFieldArray,
        // pointer to DWORD that receives size of credential blob, if EAPHost passes in
        // non-zero size and non-NULL data below, EAP-method will just attempt to Update
        // the blob with passed in values (if supported) instead of creating a new one. EAP-method
        // is not expected to release the passed in buffer, EAPHost will release it after API completion..
        _Inout_ _Pre_satisfies_(*pdwUserBlobSize >= sizeof(UINT_PTR)) DWORD* pdwUserBlobSize,
        _Inout_ _At_(*ppUserBlob, _Pre_writable_size_(*pdwUserBlobSize) _Post_readable_size_(*pdwUserBlobSize)) BYTE** ppUserBlob,
        _Outptr_ EAP_ERROR** ppEapError);

    // This function returns fields to be displayed during interactive time when alternative UI is used.
    // Method should not bring up UI in this function.
    //
    DWORD WINAPI EapPeerQueryInteractiveUIInputFields(
        // API's version number for ease of interoperability. It must be 0.
        _In_ DWORD dwVersion,
        // control the behavior of the EAP Methods
        _In_ DWORD dwFlags,
        // size of UIcontext data EAPHost runtime
        _In_ DWORD dwSizeofUIContextData,
        // UIContext data received from EAPHost runtime
        _In_reads_(dwSizeofUIContextData) const BYTE* pUIContextData,
        // structure that gets filled with fields and how they should be shown to the user.
        //  Caller (EapHost) should free the memory using EapPeerFreeMemory.
        _Out_ EAP_INTERACTIVE_UI_DATA* pEapInteractiveUIData,
        // In case of error, API fills ppEapError if possible.  Caller should free ppEapError using EapPeerFreeErrorMemory
        _Outptr_ EAP_ERROR** ppEapError,
        // Reserved for future usage. It must be NULL.
        _Inout_ LPVOID* ppvReserved);

    // This function returns interactive UI blob from the UI fields filled by the user
    //
    DWORD WINAPI EapPeerQueryUIBlobFromInteractiveUIInputFields(
        // API's version number for ease of interoperability. It must be 0.
        _In_ DWORD dwVersion,
        // control the behavior of the EAP Methods
        _In_ DWORD dwFlags,
        // size of UIcontext data EAPHost runtime
        _In_ DWORD dwSizeofUIContextData,
        // UIContext data received from EAPHost runtime
        _In_reads_(dwSizeofUIContextData) const BYTE* pUIContextData,
        // structure that carries data provided by user
        _In_ const EAP_INTERACTIVE_UI_DATA* pEapInteractiveUIData,
        // pointer to DWORD that receives size of credential blob
        _Out_ DWORD* pdwSizeOfDataFromInteractiveUI,
        // Pointer that receives the credential blob that can be used in authentication.
        // Caller (EapHost) should free the memory using EapPeerFreeMemory.
        _Outptr_result_buffer_(*pdwSizeOfDataFromInteractiveUI) BYTE** ppDataFromInteractiveUI,
        // In case of error, API fills ppEapError if possible.  Caller should free ppEapError using EapPeerFreeErrorMemory
        _Outptr_ EAP_ERROR** ppEapError,
        // Reserved for future usage. It must be NULL.
        _Inout_ LPVOID* ppvReserved);

    // This function converts XML document containing configuration information into a blob that could be passed to other
    // APIs that expect configuration blob.
    //
    // Parameters:
    // - dwFlags:
    //   EAP_FLAG_xxx defined in eaptypes.w
    //
    // - eapMethodType
    //   method type for this method
    //
    // - dwEapConnDataSize:
    //   Number of bytes of pbEapConnData.
    //
    // - pConfigDoc:
    //   Pointer to XMLDOMDocument2 that refers to the XML document containg the configuration XML
    //
    // - ppConfigOut
    //   Pointer to buffer that receives configuration blob.
    //
    // - pdwSizeOfConfigOut
    //   Pointer to DWORD that receives size of ppConfigOut.
    //
    // - pEapError
    //   Pointer to pointer to EAP_ERROR that will be filled by the Method in the case of error. In some cases
    //  the structure might not be filled.
    DWORD WINAPI EapPeerConfigXml2Blob(
        _In_ DWORD dwFlags,
        _In_ EAP_METHOD_TYPE eapMethodType,
        _In_ IXMLDOMDocument2* pConfigDoc,
        _Outptr_result_buffer_(*pdwSizeOfConfigOut) BYTE** ppConfigOut,
        _Out_ DWORD* pdwSizeOfConfigOut,
        _Outptr_ EAP_ERROR** ppEapError);

    // This function converts XML document containing credential information into a blob that could be passed to other
    // APIs that expect credential blob.
    //
    // Parameters:
    // - dwFlags:
    //   EAP_FLAG_xxx defined in eaptypes.w
    //
    // - eapMethodType
    //   method type for this method
    //
    // - pCredentialsDoc:
    //   Pointer to XMLDOMDocument2 that refers to the XML document containg the credential XML
    //
    // - pConfigIn
    //   Pointer to buffer that contains configuration blob.
    //
    // - dwSizeOfConfigIn
    //   Size of pConfigIn
    //
    // - ppCredentialsOut
    //  Pointer to buffer that receives the credential blob
    //
    // - pdwSizeOfCredentialsOut
    //  Pointer to DWORD that receives size of ppCredentialsOut
    //
    // - pEapError
    //   Pointer to pointer to EAP_ERROR that will be filled by the Method in the case of error. In some cases
    //  the structure might not be filled.
    DWORD WINAPI EapPeerCredentialsXml2Blob(
        _In_ DWORD dwFlags,
        _In_ EAP_METHOD_TYPE eapMethodType,
        _In_ IXMLDOMDocument2* pCredentialsDoc,
        _In_reads_(dwSizeOfConfigIn) const BYTE* pConfigIn,
        _In_ DWORD dwSizeOfConfigIn,
        _Outptr_result_buffer_(*pdwSizeOfCredentialsOut) BYTE** ppCredentialsOut,
        _Out_ DWORD* pdwSizeOfCredentialsOut,
        _Outptr_ EAP_ERROR** ppEapError);

    // This function converts configuration blob into configuration XML document. Methods exporting this API
    // will be supporting exporting of and importing of configuration with out problem. It also helps administrators
    // modify the configuration.
    //
    // Parameters:
    // - dwFlags:
    //   EAP_FLAG_xxx defined in eaptypes.w
    //
    // - eapMethodType
    //   method type for this method
    //
    // - pConfigIn:
    //   Configuration blob
    //
    // - dwSizeOfConfigIn
    //   Size of pConfigIn
    //
    // - ppConfigDoc
    //  Pointer to IXMLDOMDocument2 that receives configuration XML for the given configuration blob.
    //
    // - pEapError
    //   Pointer to pointer to EAP_ERROR that will be filled by the Method in the case of error. In some cases
    //  the structure might not be filled.
    DWORD WINAPI EapPeerConfigBlob2Xml(
        _In_ DWORD dwFlags,
        _In_ EAP_METHOD_TYPE eapMethodType,
        _In_reads_(dwSizeOfConfigIn) const BYTE* pConfigIn,
        _In_ DWORD dwSizeOfConfigIn,
        _COM_Outptr_ IXMLDOMDocument2** ppConfigDoc,
        _Outptr_ EAP_ERROR** ppEapError);

    // This function retrieves the method properties for a specific configuration data. The method properties
    // returned by the API will be a subset of the properties value in the registry.
    //
    // Parameters:
    // - dwVersion:
    //   The version number of the API.
    //
    // - dwFlags
    //   A combination of EAP flags that describe the EAP authentication session behavior.
    //
    // - hUserImpersonationToken
    //   An impersonation token for the user whose credentials are to be requested and obtained.
    //
    // - eapMethodType
    //   An EAP_METHOD_TYPE structure that contains vendor and author information about the
    //   EAP method used for authenticating the connection.
    //
    // - dwSizeOfConnectionDataIn
    //   The size, in bytes, of the connection data buffer provided in pConnectionDataIn
    //
    // - pConnectionDataIn
    //   Connection data used for the EAP method
    //
    // - dwSizeOfUserDataIn
    //   The size in bytes of the user data buffer provided in pUserDataIn
    //
    // - pUserDataIn
    //   A pointer to a byte buffer that contains the opaque user data BLOB
    //
    // - pMethodPropertyArray
    //   A pointer to the method properties array.
    //
    // - ppEapError
    //   A pointer to a pointer to an EAP_ERROR structure that contains any errors raised by
    //   EAPHost during the execution of this function call.
    DWORD WINAPI EapPeerGetMethodProperties(
        _In_ DWORD dwVersion,
        _In_ DWORD dwFlags,
        _In_ EAP_METHOD_TYPE eapMethodType,
        _In_ HANDLE hUserImpersonationToken,
        _In_ DWORD dwSizeOfConnectionDataIn,
        _In_reads_(dwSizeOfConnectionDataIn) const BYTE* pConnectionDataIn,
        _In_ DWORD dwSizeOfUserDataIn,
        _In_reads_(dwSizeOfUserDataIn) const BYTE* pUserDataIn,
        _Out_ EAP_METHOD_PROPERTY_ARRAY* pMethodPropertyArray,
        _Outptr_ EAP_ERROR** ppEapError);

    // This function allows the EAP method developers to provide the various
    // connection properties and user properties supported by the method. EAPHost
    // will invoke this function to create the connection property and user
    // property of the EAP method.
    //
    // Parameters:
    // - dwFlags
    //   A combination of EAP flags that describe the EAP authentication session
    //   behavior.
    //
    // - eapMethodType
    //   An EAP_METHOD_TYPE structure that contains vendor and author information
    //   about the EAP method used for authenticating the connection.
    //
    // - eapCredential
    //   An EapCredential structure that contains the credential type and the
    //   appropriate credentials.
    //
    // - pdwConfigBlobSize
    //   Receives a pointer to the size, in bytes, of the ppConfigBlob parameter.
    //
    // - ppConfigBlob
    //   Receives a pointer to a pointer that contains a byte buffer with
    //   configured connection data.
    //
    // - pdwUserBlobSize
    //   Receives a pointer to the size in byte of the ppUserBlob parameter.
    //
    // - ppUserBlobp
    //   Receives a pointer to a pointer that contains a byte buffer with the
    //   methods user data.
    //
    // - ppEapError
    //   A pointer to the address of an EAP_ERROR structure that contains any
    //   errors raised during the execution of this function call. After consuming
    //   the error data this memory will be freed by calling EapPeerFreeErrorMemory
    DWORD WINAPI EapPeerGetConfigBlobAndUserBlob(
        _In_ DWORD dwFlags,
        _In_ EAP_METHOD_TYPE eapMethodType,
        _In_ EapCredential eapCredential,
        _Out_ DWORD* pdwConfigBlobSize,
        _Outptr_result_buffer_(*pdwConfigBlobSize) BYTE** ppConfigBlob,
        _Out_ DWORD* pdwUserBlobSize,
        _Outptr_result_buffer_(*pdwUserBlobSize) BYTE** ppUserBlob,
        _Outptr_ EAP_ERROR** ppEapError);

    // This function frees memory returned by the configuration APIs. This function is called by EapHost when
    // it is done with the memory returned by the Method.
    //
    // Parameters:
    // - pUIContextData:
    //   Any memory returned by configuration APIs.
    //
    VOID WINAPI EapPeerFreeMemory(_In_ void* pUIContextData);

#ifdef __cplusplus
}
#endif

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion

#endif // EAP_PEER_METHOD_APIS_H
