//+-----------------------------------------------------------------------
//
// Microsoft Windows
//
// Copyright (C) Microsoft Corporation
//
// File:        tokenbinding.h
//
// Contents:    Common definitions for the token binding library
//
//------------------------------------------------------------------------

#ifndef __TOKENBINDING_H__
#define __TOKENBINDING_H__

#if defined(_MSC_VER) && (_MSC_VER >= 1020)
#pragma once
#endif

#include <winapifamily.h>

#pragma region Desktop Family or OneCore Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM)

//
// Public APIs and structures
//

#if(_WIN32_WINNT >= _WIN32_WINNT_WINTHRESHOLD)

typedef enum TOKENBINDING_TYPE{
    TOKENBINDING_TYPE_PROVIDED = 0,
    TOKENBINDING_TYPE_REFERRED = 1,
}TOKENBINDING_TYPE;

typedef enum TOKENBINDING_EXTENSION_FORMAT{
    TOKENBINDING_EXTENSION_FORMAT_UNDEFINED = 0,
}TOKENBINDING_EXTENSION_FORMAT;

typedef enum TOKENBINDING_KEY_PARAMETERS_TYPE{
    TOKENBINDING_KEY_PARAMETERS_TYPE_RSA2048_PKCS = 0,
    TOKENBINDING_KEY_PARAMETERS_TYPE_RSA2048_PSS = 1,
    TOKENBINDING_KEY_PARAMETERS_TYPE_ECDSAP256 = 2,
    TOKENBINDING_KEY_PARAMETERS_TYPE_ANYEXISTING = 255,
}TOKENBINDING_KEY_PARAMETERS_TYPE;

#pragma pack(push, 1)
typedef struct TOKENBINDING_IDENTIFIER{
    BYTE keyType; // TOKENBINDING_KEY_PARAMETERS_TYPE
}TOKENBINDING_IDENTIFIER;
#pragma pack(pop)

typedef struct TOKENBINDING_RESULT_DATA{
    TOKENBINDING_TYPE bindingType;
    DWORD identifierSize;
    TOKENBINDING_IDENTIFIER *identifierData;
    TOKENBINDING_EXTENSION_FORMAT extensionFormat;
    DWORD extensionSize;
    PVOID extensionData;
}TOKENBINDING_RESULT_DATA;

typedef struct TOKENBINDING_RESULT_LIST{
    DWORD resultCount;
    TOKENBINDING_RESULT_DATA *resultData;
}TOKENBINDING_RESULT_LIST;

typedef struct TOKENBINDING_KEY_TYPES{
    DWORD keyCount;
    TOKENBINDING_KEY_PARAMETERS_TYPE *keyType;
}TOKENBINDING_KEY_TYPES;

STDAPI_(SECURITY_STATUS)
TokenBindingGenerateBinding(
    _In_ TOKENBINDING_KEY_PARAMETERS_TYPE keyType,
    _In_ PCWSTR targetURL,
    _In_ TOKENBINDING_TYPE bindingType,
    _In_reads_bytes_(tlsEKMSize) const void *tlsEKM,
    _In_ DWORD tlsEKMSize,
    _In_ TOKENBINDING_EXTENSION_FORMAT extensionFormat,
    _In_ const void *extensionData,
    _Outptr_result_buffer_(*tokenBindingSize) void **tokenBinding,
    _Out_ DWORD *tokenBindingSize,
    _Outptr_opt_ TOKENBINDING_RESULT_DATA **resultData
);

STDAPI_(SECURITY_STATUS)
TokenBindingGenerateMessage(
    _In_count_(tokenBindingsCount) const void *tokenBindings[],
    _In_count_(tokenBindingsCount) const DWORD tokenBindingsSize[],
    _In_ DWORD tokenBindingsCount,
    _Outptr_result_buffer_(*tokenBindingMessageSize) void **tokenBindingMessage,
    _Out_ DWORD *tokenBindingMessageSize
);

STDAPI_(SECURITY_STATUS)
TokenBindingVerifyMessage(
    _In_reads_bytes_(tokenBindingMessageSize) const void *tokenBindingMessage,
    _In_ DWORD tokenBindingMessageSize,
    _In_ TOKENBINDING_KEY_PARAMETERS_TYPE keyType,
    _In_reads_bytes_(tlsEKMSize) const void *tlsEKM,
    _In_ DWORD tlsEKMSize,
    _Outptr_ TOKENBINDING_RESULT_LIST **resultList
);

STDAPI_(SECURITY_STATUS)
TokenBindingGetKeyTypesClient(
    _Outptr_ TOKENBINDING_KEY_TYPES **keyTypes
);

STDAPI_(SECURITY_STATUS)
TokenBindingGetKeyTypesServer(
    _Outptr_ TOKENBINDING_KEY_TYPES **keyTypes
);

STDAPI_(SECURITY_STATUS)
TokenBindingDeleteBinding(
    _In_ PCWSTR targetURL
);

STDAPI_(SECURITY_STATUS)
TokenBindingDeleteAllBindings();

STDAPI_(SECURITY_STATUS)
TokenBindingGenerateID(
    _In_ TOKENBINDING_KEY_PARAMETERS_TYPE keyType,
    _In_reads_bytes_(publicKeySize) const void *publicKey,
    _In_ DWORD publicKeySize,
    _Outptr_ TOKENBINDING_RESULT_DATA **resultData
);

#if (NTDDI_VERSION >= NTDDI_WIN10_19H1)
STDAPI_(SECURITY_STATUS)
TokenBindingGenerateIDForUri(
    _In_ TOKENBINDING_KEY_PARAMETERS_TYPE keyType,
    _In_ PCWSTR targetUri,
    _Outptr_ TOKENBINDING_RESULT_DATA **resultData
);
#endif

STDAPI_(SECURITY_STATUS)
TokenBindingGetHighestSupportedVersion(
    _Out_ BYTE *majorVersion,
    _Out_ BYTE *minorVersion
);

#endif // (_WIN32_WINNT >= _WIN32_WINNT_WINTHRESHOLD)

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM) */
#pragma endregion

#endif
