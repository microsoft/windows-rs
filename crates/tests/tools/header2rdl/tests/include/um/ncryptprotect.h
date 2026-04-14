/*-------------------------------------------------------------------
//
//  Copyright (C) Microsoft, 2010
//
//
//  File:       ncryptprotect.h
//
//  Contents:   
//              NCryptRegisterProtectionDescriptorName
//              NCryptQueryProtectionDescriptorName
//              NCryptCreateProtectionDescriptor
//              NCryptCloseProtectionDescriptor
//              NCryptProtectSecret
//              NCryptUnprotectSecret
//
//
//
--------------------------------------------------------------------*/
#if _MSC_VER > 1000
#pragma once
#endif

#include <winapifamily.h>

#pragma region Desktop Family or OneCore or Games Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES)


/* "C""C""C""C""C""C""C""C""C""C""C""C""C""C""C""C""C""C""C""C""C""C""C""C""C" */
#ifdef __cplusplus
extern "C" {
#endif /* __cplusplus */


#define NCRYPT_DESCR_DELIMITER_OR    L"OR"
#define NCRYPT_DESCR_DELIMITER_AND   L"AND"
#define NCRYPT_DESCR_EQUAL           L"="

/****************************************************************************
    Examples of Protection Descriptor:


    "SID=S-1-5-21-4392301 AND SID=S-1-5-21-3101812"
    "SDDL=O:S-1-5-5-0-290724G:SYD:(A;;CCDC;;;S-1-5-5-0-290724)(A;;DC;;;WD)"
    "LOCAL=user"
    "LOCAL=machine"

    "WEBCREDENTIALS=MyPasswordName"
    "WEBCREDENTIALS=MyPasswordName,myweb.com"

****************************************************************************/


/****************************************************************************
  Microsoft Key Protection Provider

    NCRYPT_KEY_PROTECTION_ALGORITHM_SID
    NCRYPT_KEY_PROTECTION_ALGORITHM_SDDL
    NCRYPT_KEY_PROTECTION_ALGORITHM_LOCAL
****************************************************************************/

#define MS_KEY_PROTECTION_PROVIDER      L"Microsoft Key Protection Provider"

//
// Microsoft Key Protection Provider supports the following formats:
//


#define NCRYPT_KEY_PROTECTION_ALGORITHM_SID             L"SID"
//
// SID=%SidString%
//
// %SidString% is a SID string that identifies the object's group or principal identity
//

#define NCRYPT_KEY_PROTECTION_ALGORITHM_LOCAL     L"LOCAL"

#define NCRYPT_KEY_PROTECTION_LOCAL_LOGON         L"logon"
#define NCRYPT_KEY_PROTECTION_LOCAL_USER          L"user"
#define NCRYPT_KEY_PROTECTION_LOCAL_MACHINE       L"machine"
//
// Cases for LOCAL protector
//
// Local=logon        : protects to the current logon session, 
//                    - user will not be able to unprotect after logoff or reboot;
// Local=user         : protects to the user on local machine,
//                    - only this caller on the local machine will be able to unprotect;
// Local=machine      : protects to Local Machine,
//                    - all users on the local machine will be able to unprotect;
//


#define NCRYPT_KEY_PROTECTION_ALGORITHM_SDDL            L"SDDL"
//
// SDDL=%SecurityDescriptor%
//
// %SecurityDescriptor% is a SDDL string that identifies the Security Descriptor
//

#define NCRYPT_KEY_PROTECTION_ALGORITHM_WEBCREDENTIALS  L"WEBCREDENTIALS"
//
// WEBCREDENTIALS=%Identity%[,%Source%]
//
// Credential Vault stores web passwords by Source:Identity name
// If %Source% is not specified, then the default value will be used
//

#define NCRYPT_KEY_PROTECTION_ALGORITHM_LOCKEDCREDENTIALS     L"LOCKEDCREDENTIALS"
//
// LOCKEDCREDENTIALS=%ProtectionDomain%[,%EnterpriseID%]
//

#if (NTDDI_VERSION >= NTDDI_WINBLUE)
#define NCRYPT_KEY_PROTECTION_ALGORITHM_CERTIFICATE	L"CERTIFICATE"
#define NCRYPT_KEY_PROTECTION_CERT_HASHID         L"HashId"
//
// CERTIFICATE=HashId:%HexValue%
//
//    %HexValue% is hex-encoded SHA1 thumbprint of the certificate
//
#define NCRYPT_KEY_PROTECTION_CERT_CERTBLOB         L"CertBlob"
//
// CERTIFICATE=CertBlob:%Base64String%
//
//    %Base64String% is base64-encoded certificate blob
//
#endif //(NTDDI_VERSION >= NTDDI_WINBLUE)

//
// NCRYPT_DESCRIPTOR_HANDLE
// 
DECLARE_HANDLE( NCRYPT_DESCRIPTOR_HANDLE );

#if (NTDDI_VERSION >= NTDDI_WIN8)

/****************************************************************************
 NCryptRegisterProtectionDescriptorName

    Creates a persistent association between the specified descriptor name 
    and the descriptor string value. 
    The descriptor name can then be used in calls to NCryptProtectSecret
    with NCRYPT_NAMED_DESCRIPTOR_FLAG flag.

    Named Descriptors are recommended for applications and systems,
    where an administrator or Group Policy should be able to configure
    the protection descriptor.

 pwszName
    [in] Specifies a Unicode string of named descriptor to be registered.

 pwszDescriptorString
    [in, optional] Specifies a Unicode string of Protection Descriptor.
    When this parameter is NULL or an empty string, the name
    will be unregistered.

 dwFlags
    The following flags are supported.
        NCRYPT_MACHINE_KEY_FLAG

 Return Value
    Returns a status code that indicates the success or failure of the function.
    Possible return codes include, but are not limited to, the following.
    
    ERROR_SUCCESS
    NTE_INVALID_PARAMETER
    NTE_BAD_FLAGS
    NTE_BAD_DATA
    NTE_NO_MEMORY
    NTE_NOT_FOUND
    NTE_NOT_SUPPORTED
    NTE_INVALID_HANDLE
    NTE_BAD_KEY
    NTE_BAD_PROVIDER
    NTE_BAD_TYPE


****************************************************************************/
SECURITY_STATUS
WINAPI
NCryptRegisterProtectionDescriptorName(
    _In_            LPCWSTR                             pwszName,
    _In_opt_        LPCWSTR                             pwszDescriptorString,
                    DWORD                               dwFlags
    );

/****************************************************************************
 NCryptQueryProtectionDescriptorName

 pwszName
    [in] Specifies a Unicode string of named descriptor to be registered.

 pwszDescriptorString
    [out] Specifies a buffer that receive a Unicode string of Protection Descriptor.

 pcDescriptorString
    [in, out] Specifies size, in characters, of buffer that receive a 
    Unicode string of Protection Descriptor, including terminating '\0'.

 dwFlags
    The following flags are supported.
        NCRYPT_MACHINE_KEY_FLAG

 Return Value
    Returns a status code that indicates the success or failure of the function.
    Possible return codes include, but are not limited to, the following.
    
    ERROR_SUCCESS
    NTE_INVALID_PARAMETER
    NTE_BAD_FLAGS
    NTE_BAD_DATA
    NTE_NO_MEMORY
    NTE_NOT_FOUND
    NTE_NOT_SUPPORTED
    NTE_INVALID_HANDLE
    NTE_BAD_KEY
    NTE_BAD_PROVIDER
    NTE_BAD_TYPE
    

****************************************************************************/
SECURITY_STATUS
WINAPI
NCryptQueryProtectionDescriptorName(
    _In_            LPCWSTR                             pwszName,
    _Out_writes_to_opt_(*pcDescriptorString, *pcDescriptorString)
                    LPWSTR                              pwszDescriptorString,
    _Inout_         SIZE_T                              *pcDescriptorString,
                    DWORD                               dwFlags
    );


/****************************************************************************
 NCryptCreateProtectionDescriptor

 dwFlags
    The following flags are supported.
        
        NCRYPT_NAMED_DESCRIPTOR_FLAG
        NCRYPT_MACHINE_KEY_FLAG

 pwszDescriptorString
    [in] Specifies a Unicode string of Protection Descriptor.

 phDescriptor
    [out] Pointer to Handle of Protection Descriptor.

 Return Value
    Returns a status code that indicates the success or failure of the function.
    Possible return codes include, but are not limited to, the following.
    
    ERROR_SUCCESS
    NTE_INVALID_PARAMETER
    NTE_BAD_FLAGS
    NTE_BAD_DATA
    NTE_NO_MEMORY
    NTE_NOT_FOUND
    NTE_NOT_SUPPORTED
    NTE_INVALID_HANDLE
    NTE_BAD_KEY
    NTE_BAD_PROVIDER
    NTE_BAD_TYPE

****************************************************************************/
SECURITY_STATUS
WINAPI
NCryptCreateProtectionDescriptor(
    _In_            LPCWSTR                             pwszDescriptorString,
                    DWORD                               dwFlags,
    _Out_           NCRYPT_DESCRIPTOR_HANDLE            *phDescriptor
    );

//
// The NCRYPT_NAMED_DESCRIPTOR_FLAG flag indicates that pwszDescriptorString
// value is a name registered by NCryptRegisterProtectionDescriptorName()
//
#define NCRYPT_NAMED_DESCRIPTOR_FLAG                    0x00000001

/****************************************************************************
 NCryptCloseProtectionDescriptor

 hDescriptor
    [in] Handle of Protection Descriptor created by 
    NCryptCreateProtectionDescriptor function

 Return Value
    Returns a status code that indicates the success or failure of the function.
    Possible return codes include, but are not limited to, the following.
    
 Return Value
    Returns a status code that indicates the success or failure of the function.
    Possible return codes include, but are not limited to, the following.
    
    ERROR_SUCCESS
    NTE_INVALID_HANDLE

****************************************************************************/
SECURITY_STATUS
WINAPI
NCryptCloseProtectionDescriptor(
    _In_            NCRYPT_DESCRIPTOR_HANDLE             hDescriptor
    );

/****************************************************************************
 NCryptGetProtectionDescriptorInfo

    Retrieves Protection Descriptor information from the descriptor handle.

 hDescriptor
    [in] Handle of Protection Descriptor created by 
    NCryptCreateProtectionDescriptor or NCryptUnprotectSecret functions.
    See Remarks section for more information.

 pMemPara
    [in, optional] Pointer to NCRYPT_ALLOC_PARA that specifies memory management 
    functions. If this parameter is NULL, then LocalAlloc() function is used
    to allocate memory, and the caller must use LocalFree() free to release
    memory pointed by *ppvInfo.

 dwInfoType
    [in] Indicates the parameter types of data to be retrieved.
    The type of data to be retrieved determines the type of structure to use for *ppvInfo.

 ppvInfo
    [out] A pointer to a buffer that receives the data retrieved. 
    The form of this data will vary depending on the value of the dwInfoType parameter.

 Return Value
    Returns a status code that indicates the success or failure of the function.
    Possible return codes include, but are not limited to, the following.
    
    ERROR_SUCCESS
    NTE_INVALID_PARAMETER
    NTE_BAD_FLAGS
    NTE_BAD_DATA
    NTE_NO_MEMORY
    NTE_NOT_FOUND
    NTE_NOT_SUPPORTED
    NTE_INVALID_HANDLE
    NTE_BAD_KEY
    NTE_BAD_PROVIDER
    NTE_BAD_TYPE

  Remarks:
    Applications can retrieve information about Protection Descriptor used
    to protect data from a protected blob by calling NCryptUnprotectSecret
    function with NCRYPT_UNPROTECT_NO_DECRYPT.
    When this flag is set, then only blob header will be decoded and 
    no actual decryption will occur.

****************************************************************************/

//
//      dwInfoType                                      Value       *ppvInfo
//      ----------------------------------------------  ----------- ----------------------------------
//
#define NCRYPT_PROTECTION_INFO_TYPE_DESCRIPTOR_STRING   0x00000001  // LPWSTR

_Success_(return == 0)
_When_(dwInfoType == NCRYPT_PROTECTION_INFO_TYPE_DESCRIPTOR_STRING,
       _At_((LPWSTR*)ppvInfo, _Outptr_result_nullonfailure_))
SECURITY_STATUS
WINAPI
NCryptGetProtectionDescriptorInfo(
    _In_            NCRYPT_DESCRIPTOR_HANDLE            hDescriptor,
    _In_opt_        const NCRYPT_ALLOC_PARA             *pMemPara,
                    DWORD                               dwInfoType,
    _Outptr_        void*                               *ppvInfo
    );

/****************************************************************************
 NCryptProtectSecret

  Performs cryptographic protection on the secret or key material. 
  For large data protection, applications should use NCryptProtectMessage function.

 hDescriptor
    [in] Handle of Protection Descriptor.
 
 dwFlags
    The following flags are supported.
    NCRYPT_SILENT_FLAG

 pbData
    [in] A pointer to an array of bytes to be protected.

 cbData
    [in] Specifies count of bytes in pbData.

 pMemPara
    [in, optional] Pointer to NCRYPT_ALLOC_PARA that specifies memory management 
    functions. If this parameter is NULL, then LocalAlloc() function is used
    to allocate memory, and the caller must use LocalFree() free to release
    memory pointed by *ppbProtectedBlob.

 hWnd
    [in, optional] A window handle (HWND) to be used as the parent of any user 
    interface that is displayed.

 ppbProtectedBlob
    [out, deref] Receives a pointer to an allocated Protected Blob. 
    The caller must free the memory using NCRYPT_ALLOC_PARA.

 pcbProtectedBlob
    [out] Receives a count of bytes in ppbProtectedBlob.

 Return Value
    Returns a status code that indicates the success or failure of the function.
    Possible return codes include, but are not limited to, the following.
    
    ERROR_SUCCESS
    NTE_INVALID_PARAMETER
    NTE_BAD_FLAGS
    NTE_BAD_DATA
    NTE_NO_MEMORY
    NTE_NOT_FOUND
    NTE_NOT_SUPPORTED
    NTE_INVALID_HANDLE
    NTE_BAD_KEY
    NTE_BAD_PROVIDER
    NTE_BAD_TYPE

****************************************************************************/
SECURITY_STATUS
WINAPI
NCryptProtectSecret(
    _In_            NCRYPT_DESCRIPTOR_HANDLE            hDescriptor,
                    DWORD                               dwFlags,
    _In_reads_bytes_(cbData)
                    const BYTE                          *pbData,
                    ULONG                               cbData,
    _In_opt_        const NCRYPT_ALLOC_PARA             *pMemPara,
    _In_opt_        HWND                hWnd,
    _Outptr_result_bytebuffer_(*pcbProtectedBlob)
                    BYTE                                **ppbProtectedBlob,
    _Out_           ULONG                               *pcbProtectedBlob
    );


/****************************************************************************
 NCryptUnprotectSecret

 phDescriptor
    [out, optional] Pointer to Handle of Protection Descriptor.

 pbProtectedBlob
    [in] A pointer to an array of bytes that holds the encrypted data

 cbProtectedBlob
    [in] Specifies count of bytes in pbProtectedBlob.

 dwFlags
    The following flags are supported.

    NCRYPT_UNPROTECT_NO_DECRYPT
    NCRYPT_SILENT_FLAG

    See Remarks section for more info.

 pMemPara
    [in, optional] Pointer to NCRYPT_ALLOC_PARA that specifies memory management 
    functions. If this parameter is NULL, then LocalAlloc() function is used
    to allocate memory, and the caller must use LocalFree() free to release
    memory pointed by *ppbData.

 hWnd
    [in, optional] A window handle (HWND) to be used as the parent of any user 
    interface that is displayed.

 ppbData
    [out, deref] Receives a pointer to an allocated buffer with decrypted data. 
    The caller must free the memory using NCRYPT_ALLOC_PARA.

 pcbData
    [out] Receives a count of bytes in ppbData.

 Return Value
    Returns a status code that indicates the success or failure of the function.
    Possible return codes include, but are not limited to, the following.
    
    ERROR_SUCCESS
    NTE_INVALID_PARAMETER
    NTE_BAD_FLAGS
    NTE_BAD_DATA
    NTE_NO_MEMORY
    NTE_NOT_FOUND
    NTE_NOT_SUPPORTED
    NTE_INVALID_HANDLE
    NTE_BAD_KEY
    NTE_BAD_PROVIDER
    NTE_BAD_TYPE
    NTE_DECRYPTION_FAILURE

  Remarks:
    Applications can retrieve information about Protection Descriptor used
    to protect data from a protected blob by calling NCryptUnprotectSecret
    function with NCRYPT_UNPROTECT_NO_DECRYPT.
    When this flag is set, then only blob header will be decoded and 
    no actual decryption will occur.

****************************************************************************/
SECURITY_STATUS
WINAPI
NCryptUnprotectSecret(
    _Out_opt_       NCRYPT_DESCRIPTOR_HANDLE            *phDescriptor,
     _In_            DWORD                               dwFlags,
    _In_reads_bytes_(cbProtectedBlob)
                    const BYTE                          *pbProtectedBlob,
                    ULONG                               cbProtectedBlob,
    _In_opt_        const NCRYPT_ALLOC_PARA             *pMemPara,
    _In_opt_        HWND                hWnd,
    _Outptr_result_bytebuffer_(*pcbData)
                    BYTE                                **ppbData,
    _Out_           ULONG                               *pcbData
    );

#define NCRYPT_UNPROTECT_NO_DECRYPT                     0x00000001

/*--------------------------------------------------------------------------
//
//                               STREAM API
//
---------------------------------------------------------------------------*/

DECLARE_HANDLE( NCRYPT_STREAM_HANDLE );


/****************************************************************************
  PFNCryptStreamOutputCallback

    pvCallbackCtxt  
        The arguments specified by NCRYPT_PROTECT_STREAM_INFO.

    pbData  
        A pointer to a block of processed data that is available to the application.

    cbData  
        The size, in bytes, of the block of processed data at pbData. 

    fFinal  
        Specifies that the last block of data is being processed and that this
        is the last time the callback will be executed.

 Return Value
    Returns a status code that indicates the success or failure of the function.
    Possible return codes include, but are not limited to, the following.
    
    ERROR_SUCCESS
    NTE_INVALID_PARAMETER
    NTE_BAD_FLAGS
    NTE_BAD_DATA
    NTE_NO_MEMORY
    NTE_NOT_FOUND
    NTE_NOT_SUPPORTED
    NTE_INVALID_HANDLE
    NTE_BAD_KEY
    NTE_BAD_PROVIDER
    NTE_BAD_TYPE

****************************************************************************/
typedef 
SECURITY_STATUS
(WINAPI *PFNCryptStreamOutputCallback)(
    _In_            void                                *pvCallbackCtxt,
    _In_reads_bytes_(cbData)
                    const BYTE                          *pbData,
                    SIZE_T                              cbData,
                    BOOL                                fFinal
    );

/****************************************************************************
 NCRYPT_PROTECT_STREAM_INFO

    The NCRYPT_PROTECT_STREAM_INFO structure is used to enable stream processing 
    of data rather than single block processing. 
    This structure is passed to the NCryptStreamOpenToProtect and 
    NCryptStreamOpenToUnprotect functions.

    pfnStreamOutput
        [in] The address of a callback function used to read from and write
        data to a disk when processing large messages.

    pvCallbackCtxt
        [in] A pointer to the argument to pass to the callback function.

****************************************************************************/
typedef struct NCRYPT_PROTECT_STREAM_INFO {
    PFNCryptStreamOutputCallback        pfnStreamOutput;
    void                                *pvCallbackCtxt;
} NCRYPT_PROTECT_STREAM_INFO;

/****************************************************************************
 NCryptStreamOpenToProtect

    Performs cryptographic protection on large data in stream mode.

 hDescriptor
    [in] Handle of Protection Descriptor.

 dwFlags
    The following flags are supported.
    NCRYPT_SILENT_FLAG

 hWnd
    [in, optional] A window handle (HWND) to be used as the parent of any user 
    interface that is displayed.

 pStreamInfo
    [in] A pointer to NCRYPT_PROTECT_STREAM_INFO.

 phStream
    [out] Receives a pointer to a stream handle. 

 Return Value
    Returns a status code that indicates the success or failure of the function.
    Possible return codes include, but are not limited to, the following.
    
    ERROR_SUCCESS
    NTE_INVALID_PARAMETER
    NTE_BAD_FLAGS
    NTE_BAD_DATA
    NTE_NO_MEMORY
    NTE_NOT_FOUND
    NTE_NOT_SUPPORTED
    NTE_INVALID_HANDLE
    NTE_BAD_KEY
    NTE_BAD_PROVIDER
    NTE_BAD_TYPE

****************************************************************************/
SECURITY_STATUS
WINAPI
NCryptStreamOpenToProtect(
    _In_            NCRYPT_DESCRIPTOR_HANDLE            hDescriptor,
                    DWORD                               dwFlags,
    _In_opt_        HWND                    hWnd,
    _In_            NCRYPT_PROTECT_STREAM_INFO          *pStreamInfo,
    _Out_           NCRYPT_STREAM_HANDLE                *phStream
    );

/****************************************************************************
 NCryptStreamOpenToUnprotect

 pStreamInfo
    [in] A pointer to NCRYPT_PROTECT_STREAM_INFO.

 dwFlags
    The following flags are supported.
    NCRYPT_SILENT_FLAG

 hWnd
    [in, optional] A window handle (HWND) to be used as the parent of any user 
    interface that is displayed.

 phStream
    [out] Receives a pointer to a stream handle. 

 Return Value
    Returns a status code that indicates the success or failure of the function.
    Possible return codes include, but are not limited to, the following.
    
    ERROR_SUCCESS
    NTE_INVALID_PARAMETER
    NTE_BAD_FLAGS
    NTE_BAD_DATA
    NTE_NO_MEMORY
    NTE_NOT_FOUND
    NTE_NOT_SUPPORTED
    NTE_INVALID_HANDLE
    NTE_BAD_KEY
    NTE_BAD_PROVIDER
    NTE_BAD_TYPE
    NTE_DECRYPTION_FAILURE

****************************************************************************/
SECURITY_STATUS
WINAPI
NCryptStreamOpenToUnprotect(
    _In_            NCRYPT_PROTECT_STREAM_INFO          *pStreamInfo,
                    DWORD                               dwFlags,
    _In_opt_        HWND                hWnd,
    _Out_           NCRYPT_STREAM_HANDLE                *phStream
    );

#if (NTDDI_VERSION >= NTDDI_WINTHRESHOLD)

/****************************************************************************
 PFNCryptStreamOutputCallbackEx

 pvCallbackCtxt
    The arguments specified by NCRYPT_PROTECT_STREAM_INFO_EX.

 pbData
    A pointer to a block of processed data that is available to the application.  If
    data is not available yet, but the descriptor is, this will be NULL.

 cbData
    The size, in bytes, of the block of processed data at pbData.

 hDescriptor
    Handle of Protection Descriptor.

 fFinal
    Specifies that the last block of data is being processed and that this
    is the last time the callback will be executed.

 Return Value
    Returns a status code that indicates the success or failure of the function.
    Possible return codes include, but are not limited to, the following.

    ERROR_SUCCESS
    NTE_INVALID_PARAMETER
    NTE_BAD_FLAGS
    NTE_BAD_DATA
    NTE_NO_MEMORY
    NTE_NOT_FOUND
    NTE_NOT_SUPPORTED
    NTE_INVALID_HANDLE
    NTE_BAD_KEY
    NTE_BAD_PROVIDER
    NTE_BAD_TYPE

****************************************************************************/
typedef
SECURITY_STATUS
(WINAPI *PFNCryptStreamOutputCallbackEx)(
    _In_            void                                *pvCallbackCtxt,
    _In_reads_bytes_opt_(cbData)
                    const BYTE                          *pbData,
                    SIZE_T                              cbData,
    _In_            NCRYPT_DESCRIPTOR_HANDLE            hDescriptor,
                    BOOL                                fFinal
);

/****************************************************************************
 NCRYPT_PROTECT_STREAM_INFO_EX

    The NCRYPT_PROTECT_STREAM_INFO_EX structure is used to enable stream processing
    of data rather than single block processing.
    This structure is passed to the NCryptStreamOpenToUnprotectEx function.  There
    is not equivalent NCryptStreamOpenToProtectEx function, thus you need to use
    the PFNCryptStreamOutputCallback, NCRYPT_PROTECT_STREAM_INFO and
    NCryptStreamOpenToProtect functions.

    pfnStreamOutput
        [in] The address of a callback function used to read from and write
        data to a disk when processing large messages.

    pvCallbackCtxt
        [in] A pointer to the argument to pass to the callback function.

****************************************************************************/
typedef struct NCRYPT_PROTECT_STREAM_INFO_EX {
    PFNCryptStreamOutputCallbackEx      pfnStreamOutput;
    void                                *pvCallbackCtxt;
} NCRYPT_PROTECT_STREAM_INFO_EX;

/****************************************************************************
 NCryptStreamOpenToUnprotectEx

    pStreamInfo
        [in] A pointer to NCRYPT_PROTECT_STREAM_INFO_EX.

    dwFlags
        The following flags are supported.
        NCRYPT_SILENT_FLAG

    hWnd
        [in, optional] A window handle (HWND) to be used as the parent of any user
        interface that is displayed.

    phStream
        [out] Receives a pointer to a stream handle.

    Return Value
        Returns a status code that indicates the success or failure of the function.
        Possible return codes include, but are not limited to, the following.

        ERROR_SUCCESS
        NTE_INVALID_PARAMETER
        NTE_BAD_FLAGS
        NTE_BAD_DATA
        NTE_NO_MEMORY
        NTE_NOT_FOUND
        NTE_NOT_SUPPORTED
        NTE_INVALID_HANDLE
        NTE_BAD_KEY
        NTE_BAD_PROVIDER
        NTE_BAD_TYPE
        NTE_DECRYPTION_FAILURE

****************************************************************************/
SECURITY_STATUS
WINAPI
NCryptStreamOpenToUnprotectEx(
    _In_            NCRYPT_PROTECT_STREAM_INFO_EX       *pStreamInfo,
                    DWORD                               dwFlags,
    _In_opt_        HWND                                hWnd,
    _Out_           NCRYPT_STREAM_HANDLE                *phStream
);

#endif

/****************************************************************************
 NCryptStreamUpdate

 The NCryptStreamUpdate encrypts or decrypts a chunk of data.

 hStream
    [in] Handle returned by NCryptStreamOpenToProtect or 
    NCryptStreamOpenToUnprotect function.

 pbData
    [in] A pointer to an array of bytes to be protected.

 cbData
    [in] Specifies count of bytes in pbData.

 fFinal
    [in] Indicates that the last block of data for protecting or unprotecting 
    is being processed. 

 Return Value
    Returns a status code that indicates the success or failure of the function.
    Possible return codes include, but are not limited to, the following.
    
    ERROR_SUCCESS
    NTE_INVALID_PARAMETER
    NTE_BAD_FLAGS
    NTE_BAD_DATA
    NTE_NO_MEMORY
    NTE_NOT_FOUND
    NTE_NOT_SUPPORTED
    NTE_INVALID_HANDLE
    NTE_BAD_KEY
    NTE_BAD_PROVIDER
    NTE_BAD_TYPE
    NTE_DECRYPTION_FAILURE

****************************************************************************/
SECURITY_STATUS
WINAPI
NCryptStreamUpdate(
    _In_            NCRYPT_STREAM_HANDLE                hStream,
    _In_reads_bytes_(cbData)
                    const BYTE                          *pbData,
                    SIZE_T                              cbData,
                    BOOL                                fFinal
    );

/****************************************************************************
 NCryptStreamClose

 hStream
    [in] Handle returned by NCryptStreamOpenToProtect or 
    NCryptStreamOpenToUnprotect function.

 Return Value
    Returns a status code that indicates the success or failure of the function.
    Possible return codes include, but are not limited to, the following.
    
    ERROR_SUCCESS
    NTE_INVALID_PARAMETER
    NTE_INVALID_HANDLE

****************************************************************************/
SECURITY_STATUS
WINAPI
NCryptStreamClose(
    _In_            NCRYPT_STREAM_HANDLE                hStream
    );

#endif // (NTDDI_VERSION >= NTDDI_WIN8)

#ifdef __cplusplus
} //extern "C"
#endif /* __cplusplus */



#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES) */
#pragma endregion


