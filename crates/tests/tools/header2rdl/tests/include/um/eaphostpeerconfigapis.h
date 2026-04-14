///////////////////////////////////////////////////////////////////////////////
//
// Copyright (c) Microsoft Corporation
//
// SYNOPSIS
//
//   Declares methods that are used to configure EAP methods.
//
///////////////////////////////////////////////////////////////////////////////

#ifndef EAPHOSTPEERCONFIGAPIS_H
#define EAPHOSTPEERCONFIGAPIS_H
#pragma once
#include <winapifamily.h>

#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)

#include "objbase.h" //Required by msxml2.h
#include "msxml6.h"

#include "EapTypes.h"

#ifdef __cplusplus
extern "C"
{
#endif

    // This API is used to enumerate all the EAP Methods installed and available for use; this
    // includes legacy EAP Methods too. Returns non zero return code up on error.
    DWORD WINAPI EapHostPeerGetMethods(
        // API fills the structure to have installed EAP Methods. Caller should free the inner pointers
        // using EapHostPeerFreeMemory starting at the inner most pointer.
        _Out_ EAP_METHOD_INFO_ARRAY* pEapMethodInfoArray,
        // In case of error, API fills ppEapError if possible. Caller should free ppEapError using EapHostPeerFreeErrorMemory
        _Outptr_ EAP_ERROR** ppEapError);

    // This API is used to retrieve the method properties for a specific configuration
    // It returns non zero return code upon error.
    DWORD WINAPI EapHostPeerGetMethodProperties(
        // The version number of the API.
        _In_ DWORD dwVersion,
        // A combination of EAP flags that describe the EAP authentication session behavior.
        _In_ DWORD dwFlags,
        // An EAP_METHOD_TYPE structure that identifies the EAP method the supplicant is to use.
        _In_ EAP_METHOD_TYPE eapMethodType,
        // A handle to the user impersonation token to use in this session.
        _In_ HANDLE hUserImpersonationToken,
        // The size, in bytes, of the connection data buffer provided in pbEapConnData.
        _In_ DWORD dwEapConnDataSize,
        // Connection data used for the EAP method.
        _In_reads_(dwEapConnDataSize) const BYTE* pbEapConnData,
        // The size in bytes of the user data buffer provided in pbUserData.
        _In_ DWORD dwUserDataSize,
        // A pointer to a byte buffer that contains the opaque user data BLOB.
        _In_reads_(dwUserDataSize) const BYTE* pbUserData,
        // A pointer to the method properties array. Caller should free the inner pointers using EapHostPeerFreeMemory starting
        // at the inner most pointer. The caller should free empvString value only when the type is empvtString.
        _Out_ EAP_METHOD_PROPERTY_ARRAY* pMethodPropertyArray,
        // A pointer to a pointer to an EAP_ERROR structure that contains any errors raised by EAPHost
        // during the execution of this function call.
        _Outptr_ EAP_ERROR** ppEapError);

    // This API is used to invoke configure UI of the specified EAP Method.
    // API Returns non zero return code up on error.
    DWORD WINAPI EapHostPeerInvokeConfigUI(
        // window handle of the parent window under which configuration dialog will show up
        _In_ HWND hwndParent,
        // Flags to control the behavior of the EAP Method.
        _In_ DWORD dwFlags,
        // Identifies the EAP Method to configure
        _In_ EAP_METHOD_TYPE eapMethodType,
        // Size of input configuration; this could be 0 when there is no configuration
        _In_ DWORD dwSizeOfConfigIn,
        // input configuration, this could be NULL when there is no configuration
        _In_reads_opt_(dwSizeOfConfigIn) const BYTE* pConfigIn,
        // pointer to DWORD that receives configuration after user updated using UI
        _Out_ DWORD* pdwSizeOfConfigOut,
        // buffer that receives updated configuration after user updated using UI.
        // Caller should free the memory using EapHostPeerFreeMemory.
        _Outptr_result_buffer_(*pdwSizeOfConfigOut) BYTE** ppConfigOut,
        // In case of error, API fills ppEapError if possible. Caller should free ppEapError using EapHostPeerFreeErrorMemory
        _Outptr_ EAP_ERROR** ppEapError);

    // This API is used to obtain the fields to be shown in Single Sign On scenario for
    // showing UI in long screen. The structures returned have details on how to show the fields.
    // API Returns non zero return code up on error.
    DWORD WINAPI EapHostPeerQueryCredentialInputFields(
        // handle to the impersonation token that is used while single sign on.
        _In_ HANDLE hUserImpersonationToken,
        // identifies the EAP Method supplicant wants to use
        _In_ EAP_METHOD_TYPE eapMethodType,
        // control the behavior of the EAP Methods
        _In_ DWORD dwFlags,
        // size of input configuration data
        _In_ DWORD dwEapConnDataSize,
        // configuration data that is used for the EAP method
        _In_reads_(dwEapConnDataSize) const BYTE* pbEapConnData,
        // structure that gets filled with fields and how they should be shown to the user.
        // Caller should free the inner pointers using EapHostPeerFreeMemory starting
        // at the inner most pointer.
        _Out_ EAP_CONFIG_INPUT_FIELD_ARRAY* pEapConfigInputFieldArray,
        // In case of error, API fills ppEapError if possible. Caller should free ppEapError using EapHostPeerFreeErrorMemory
        _Outptr_ EAP_ERROR** ppEapError);

    // Supplicant up on receiving user inputs from Single Sign On UI, calls this API to obtain
    // credential blob that could used start authentication
    // API Returns non zero return code up on error.
    DWORD WINAPI EapHostPeerQueryUserBlobFromCredentialInputFields(
        // handle to the impersonation token that is used while single sign on.
        _In_ HANDLE hUserImpersonationToken,
        // identifies the EAP Method supplicant wants to use
        _In_ EAP_METHOD_TYPE eapMethodType,
        // control the behavior of the EAP Methods
        _In_ DWORD dwFlags,
        // size of input configuration data
        _In_ DWORD dwEapConnDataSize,
        // configuration data that is used for the EAP method
        _In_reads_(dwEapConnDataSize) const BYTE* pbEapConnData,
        // structure containing the data entered by the user in Single Sign On UI
        _In_ const EAP_CONFIG_INPUT_FIELD_ARRAY* pEapConfigInputFieldArray,
        // pointer to DWORD that receives the size of credential blob, if supplicant passes in
        // non-zero size and non-NULL data below, EAPHost will just attempt to Update
        // the blob with passed in values (if method supports) instead of creating a new one.
        _Inout_ DWORD* pdwUserBlobSize,
        // Pointer that receives the credential blob that can be used in authentication.
        // For incoming data caller should always allocate this memory using LocalAlloc()
        // Caller should free the memory using EapHostPeerFreeMemory.
        _Inout_ _At_(*ppbUserBlob, _When_(*ppbUserBlob != NULL, _Pre_writable_size_(*pdwUserBlobSize)) _Post_readable_size_(*pdwUserBlobSize))
            BYTE** ppbUserBlob,
        // In case of error, API fills ppEapError if possible. Caller should free ppEapError using EapHostPeerFreeErrorMemory
        _Out_ EAP_ERROR** ppEapError);

    // Tunnel Methods call this function to invoke Identity UI of the inner Methods.
    // This function returns the identity as well as the credentials to use to start the authentication.
    DWORD WINAPI EapHostPeerInvokeIdentityUI(
        // API's version number for ease of interoperability. It must be 0.
        _In_ DWORD dwVersion,
        // identifies the EAP Method supplicant wants to use
        _In_ EAP_METHOD_TYPE eapMethodType,
        // EAP_FLAG_xxx defined in eaptypes.w
        _In_ DWORD dwFlags,
        // window handle of the parent window under which configuration dialog will show up
        _In_ HWND hwndParent,
        // size of the buffer (pConnectionData) in bytes
        _In_ DWORD dwSizeofConnectionData,
        // configuration data that is used for the EAP method
        _In_reads_(dwSizeofConnectionData) const BYTE* pConnectionData,
        // size of the ubuffer pUserData
        _In_ DWORD dwSizeofUserData,
        // user credential information pertinent to this auth
        _In_reads_opt_(dwSizeofUserData) const BYTE* pUserData,
        // size of the buffer pUserDataOut.
        _Out_ DWORD* pdwSizeOfUserDataOut,
        // user data information returned by method.
        // Caller should release this using EapHostPeerFreeMemory
        _Outptr_result_buffer_(*pdwSizeOfUserDataOut) BYTE** ppUserDataOut,
        // identity returned by method. Caller should release this using EapHostPeerFreeMemory
        _Outptr_ LPWSTR* ppwszIdentity,
        // In case of error, API fills ppEapError if possible. Caller should free ppEapError
        // using EapHostPeerFreeErrorMemory
        _Outptr_ EAP_ERROR** ppEapError,
        // Reserved for future usage. It must be NULL.
        _Inout_ LPVOID* ppvReserved);

    // While doing authentication with EapHost, when supplicant receives action code of EapHostPeerResponseInvokeUi
    // supplicant should call EapHostPeerGetUIContext to get UI context data. After that. it then should call this
    // API from a process where UI can be brought up.
    // API Returns non zero return code up on error.
    DWORD WINAPI EapHostPeerInvokeInteractiveUI(
        // window handle of the parent window under which dialog will show up
        _In_ HWND hwndParent,
        // size of UIcontext data received from EapHostPeerGetUIContext call
        _In_ DWORD dwSizeofUIContextData,
        // UIcontext data received from EapHostPeerGetUIContext call
        _In_reads_opt_(dwSizeofUIContextData) const BYTE* pUIContextData,
        // pointer to DWORD that receives data from interactive UI that is used for authentication
        // to continue
        _Out_ DWORD* pdwSizeOfDataFromInteractiveUI,
        // pointer that receives buffer filled with interactive UI that is used for authentication
        // to continue. Caller should free the memory using EapHostPeerFreeMemory.
        _Outptr_result_buffer_(*pdwSizeOfDataFromInteractiveUI) BYTE** ppDataFromInteractiveUI,
        // In case of error, API fills ppEapError if possible. Caller should free ppEapError using EapHostPeerFreeErrorMemory
        _Outptr_ EAP_ERROR** ppEapError);

    // This API will enable supplicants to collect necessary information from EAPHost to raise interactive UI and collect user
    // information supplicant should call EapHostPeerQueryInteractiveUIInputFields() API first after it receives
    // EapHostPeerResponseInvokeUi actionCode from EAPHost. If the return value from this API is
    // EAP_E_EAPHOST_METHOD_OPERATION_NOT_SUPPORTED, then supplicant should fall back to traditional model of invoking method
    // interactive UI, i.e. by calling EapHostPeerInvokeInteractiveUI() API Returns non zero return code up on error.
    DWORD WINAPI EapHostPeerQueryInteractiveUIInputFields(
        // API's version number for ease of interoperability. It must be 0.
        _In_ DWORD dwVersion,
        // control the behavior of the EAP Methods
        _In_ DWORD dwFlags,
        // size of UIcontext data EAPHost runtime
        _In_ DWORD dwSizeofUIContextData,
        // UIContext data received from EAPHost runtime
        _In_reads_(dwSizeofUIContextData) const BYTE* pUIContextData,
        // structure that gets filled with fields and how they should be shown to the user.
        // Caller should free the inner pointers using EapHostPeerFreeMemory starting at the inner most pointer.
        _Out_ EAP_INTERACTIVE_UI_DATA* pEapInteractiveUIData,
        // In case of error, API fills ppEapError if possible.  Caller should free ppEapError using EapHostPeerFreeErrorMemory
        _Outptr_ EAP_ERROR** ppEapError,
        // Reserved for future usage. It must be NULL.
        _Inout_ LPVOID* ppvReserved);

    // This API will enable supplicants to convert user information into a user-blob which can be consumed by EAPHost runtime APIs
    // API Returns non zero return code up on error.
    DWORD WINAPI EapHostPeerQueryUIBlobFromInteractiveUIInputFields(
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
        // pointer to DWORD that receives size of credential blob, if supplicant passes in
        // non-zero size and non-NULL data below, EAPHost will just attempt to Update
        // the blob with passed in values (if method supports) instead of creating a new one.
        _Out_ DWORD* pdwSizeOfDataFromInteractiveUI,
        // Pointer that receives the credential blob that can be used in authentication.
        // Caller should free the memory using EapHostPeerFreeMemory.
        _Outptr_result_buffer_(*pdwSizeOfDataFromInteractiveUI) BYTE** ppDataFromInteractiveUI,
        // In case of error, API fills ppEapError if possible.  Caller should free ppEapError using EapHostPeerFreeErrorMemory
        _Outptr_ EAP_ERROR** ppEapError,
        // Reserved for future usage. It must be NULL.
        _Inout_ LPVOID* ppvReserved);

    // Supplicant can utilize XML based EAP configuration storing, managing, editing. When it wants to call EapHostPeerConfigureUI
    // or when it has to start authentication, it calls this API to convert XML configuration into blob.
    // API Returns non zero return code up on error.
    DWORD WINAPI EapHostPeerConfigXml2Blob(
        // controls the behavior of the EAP Method
        _In_ DWORD dwFlags,
        // a node containing EAP XML configuration inside it
        _In_ IXMLDOMNode* pConfigDoc,
        // pointer to DWORD that receives the configuration blob size
        _Out_ DWORD* pdwSizeOfConfigOut,
        // pointer that receives configuration blob. Caller should free the memory using EapHostPeerFreeMemory.
        _Outptr_result_buffer_(*pdwSizeOfConfigOut) BYTE** ppConfigOut,
        // pointer that receives the EAP Method specified in the XML configuration
        _Out_ EAP_METHOD_TYPE* pEapMethodType,
        // In case of error, API fills ppEapError if possible. Caller should free ppEapError using EapHostPeerFreeErrorMemory
        _Outptr_ EAP_ERROR** ppEapError);

    // Supplicant can utilize XML based EAP credentials storing, managing, editing. When it wants to start
    // authentication, it calls this API to convert XML based credentials into blob.
    // API Returns non zero return code up on error.
    DWORD WINAPI EapHostPeerCredentialsXml2Blob(
        // controls the behavior of the EAP Method
        _In_ DWORD dwFlags,
        // XML node that contains credentials
        _In_ IXMLDOMNode* pCredentialsDoc,
        // size of configuration blob that the credentials are configured for
        _In_ DWORD dwSizeOfConfigIn,
        // configuration blob that the credentials are configured for
        _In_reads_(dwSizeOfConfigIn) BYTE* pConfigIn,
        // pointer to DWORD that receives size of the credentials blob.
        _Out_ DWORD* pdwSizeOfCredentialsOut,
        // pointer that receives credential blob buffer.  Caller should free the memory using
        // EapHostPeerFreeMemory.
        _Outptr_result_buffer_(*pdwSizeOfCredentialsOut) BYTE** ppCredentialsOut,
        // pointer that receives the EAP Method specified in the XML configuration
        _Out_ EAP_METHOD_TYPE* pEapMethodType,
        // In case of error, API fills ppEapError if possible. Caller should free ppEapError using EapHostPeerFreeErrorMemory
        _Outptr_ EAP_ERROR** ppEapError);

    // Supplicant can utilize XML based EAP credentials storing, managing, editing. When it wants to start
    // convert configuration blob to XML, it can call this API.
    // API Returns non zero return code up on error.
    DWORD WINAPI EapHostPeerConfigBlob2Xml(
        // controls the behavior of the EAP Method
        _In_ DWORD dwFlags,
        // identifies the EAP Method
        _In_ EAP_METHOD_TYPE eapMethodType,
        // Size of configuration blob that supplicant wants to convert
        _In_ DWORD dwSizeOfConfigIn,
        // Configuration blob that supplicant wants to convert
        _In_reads_(dwSizeOfConfigIn) BYTE* pConfigIn,
        // XML document that contains XML form of the blob. If the EAP Method does not support
        // EapPeerConfigBlob2Xml function, the XML contains ConfigBlob node with blob in string form
        _COM_Outptr_ IXMLDOMDocument2** ppConfigDoc,
        // In case of error, API fills ppEapError if possible. Caller should free ppEapError using EapHostPeerFreeErrorMemory
        _Outptr_ EAP_ERROR** ppEapError);

    // Supplicant calls this API to free memory returned by Config APIs. Supplicant should not use this API
    // for freeing EAP_ERROR structure.
    VOID WINAPI EapHostPeerFreeMemory(BYTE* pData);

    // Supplicant calls this API to free EAP_ERROR memory, which will get filled when an API fails.
    VOID WINAPI EapHostPeerFreeErrorMemory(EAP_ERROR* pEapError);

#ifdef __cplusplus
}
#endif

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion

#endif // EAPHOSTPEERCONFIGAPIS_H
