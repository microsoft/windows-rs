//+---------------------------------------------------------------------------
//
//  Microsoft Windows
//  Copyright (c) Microsoft Corporation. All rights reserved.
//
//----------------------------------------------------------------------------

#pragma once

#include <cor.h>
#include <activation.h>
#include <hstring.h>

#include <windows.foundation.h>
#include <windows.foundation.collections.h>

#ifdef ____x_ABI_CWindows_CFoundation_CIClosable_FWD_DEFINED__
#include <windows.storage.streams.h>
#else
#include <robytestream.h>
#endif // ____x_ABI_CWindows_CFoundation_CIClosable_FWD_DEFINED__

// If ABI WinRT headers have already been included or "MIDL_NS_PREFIX" behavior has been explicitly requested,
// then prefix any WinRT types referenced in this file with "ABI" to allow this header to be used in environemnts
// where WinRT types are defined under the "ABI" root namespace. Otherwise, define "ABI" as nothing so this header
// will work in environemnts where WinRT types aren't part of the "ABI" root namespace.
#if !defined(____x_ABI_CWindows_CFoundation_CIClosable_FWD_DEFINED__) && !defined(MIDL_NS_PREFIX)
#pragma push_macro("ABI")
#undef ABI
#define ABI
#endif

#ifdef __cplusplus
typedef ABI::Windows::Foundation::Collections::IPropertySet __PropertySet__;
typedef ABI::Windows::Storage::Streams::IPropertySetSerializer __IPropertySetSerializer__;
#else
typedef __x_ABI_CWindows_CFoundation_CCollections_CIPropertySet __PropertySet__;
typedef __x_ABI_CWindows_CStorage_CStreams_CIPropertySetSerializer __IPropertySetSerializer__;
#endif // __cplusplus


#if (NTDDI_VERSION >= NTDDI_WIN8)
_Check_return_ extern "C" HRESULT WINAPI RoGetMetaDataFile(
    _In_ const HSTRING name,
    _In_opt_ IMetaDataDispenserEx *metaDataDispenser,
    _Out_opt_ HSTRING *metaDataFilePath,
    _Outptr_opt_ IMetaDataImport2 **metaDataImport,
    _Out_opt_ mdTypeDef *typeDefToken);

_When_(return == S_OK, _At_(typeNameParts, _Outptr_result_buffer_(*partsCount)))
_When_(return != S_OK, _At_(typeNameParts, _Outptr_))
_Success_(return == S_OK)
_Check_return_  extern "C" HRESULT WINAPI RoParseTypeName(
    _In_ HSTRING typeName,
    _Out_ DWORD *partsCount,
    _Out_writes_(*partsCount) HSTRING **typeNameParts);

_Check_return_ extern "C" HRESULT WINAPI RoResolveNamespace(
    _In_opt_ const HSTRING name,
    _In_opt_ const HSTRING windowsMetaDataDir,
    _In_ const DWORD packageGraphDirsCount,
    _In_reads_opt_(packageGraphDirsCount) const HSTRING *packageGraphDirs,
    _Out_opt_ DWORD *metaDataFilePathsCount,
    _Outptr_opt_result_buffer_(*metaDataFilePathsCount) HSTRING **metaDataFilePaths,
    _Out_opt_ DWORD *subNamespacesCount,
    _Outptr_opt_result_buffer_(*subNamespacesCount) HSTRING **subNamespaces);
#endif

#if (NTDDI_VERSION >= NTDDI_WIN10_RS5)
_Check_return_ STDAPI RoIsApiContractPresent(
    _In_ PCWSTR name,
    _In_ UINT16 majorVersion,
    _In_ UINT16 minorVersion,
    _Out_ BOOL* present);

_Check_return_ STDAPI RoIsApiContractMajorVersionPresent(
    _In_ PCWSTR name,
    _In_ UINT16 majorVersion,
    _Out_ BOOL* present);

#ifdef __cplusplus
extern "C"
{
#endif // __cplusplus

_Check_return_ STDAPI RoCreateNonAgilePropertySet(
    _COM_Outptr_ __PropertySet__** ppPropertySet
    );

_Check_return_ STDAPI RoCreatePropertySetSerializer(
    _COM_Outptr_ __IPropertySetSerializer__** ppPropertySetSerializer
    );

#ifdef __cplusplus
}
#endif // __cplusplus

#if !defined(____x_ABI_CWindows_CFoundation_CIClosable_FWD_DEFINED__) && !defined(MIDL_NS_PREFIX)
#pragma pop_macro("ABI")
#endif

#endif
