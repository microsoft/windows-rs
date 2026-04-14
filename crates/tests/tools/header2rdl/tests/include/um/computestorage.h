/**
  \copyright Copyright (c) Microsoft Corporation

  \file ComputeStorage.ext

  \brief API set contract for ext-ms-win-hyperv-computestorage-l1
**/

#pragma once

#ifndef _HYPERV_COMPUTESTORAGE_H_
#define _HYPERV_COMPUTESTORAGE_H_

#if defined(_MSC_VER) && (_MSC_VER >= 1200)
#pragma once
#endif

#include <apiset.h>
#include <apisetcconv.h>
#include <minwindef.h>
#include <winapifamily.h>

#pragma region Desktop Family or OneCore Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM)

#ifdef __cplusplus
extern "C" {
#endif

// Imports a container layer.
HRESULT
WINAPI
HcsImportLayer(
    _In_ PCWSTR layerPath,
    _In_ PCWSTR sourceFolderPath,
    _In_ PCWSTR layerData
    );

// Exports a container layer.
HRESULT
WINAPI
HcsExportLayer(
    _In_ PCWSTR layerPath,
    _In_ PCWSTR exportFolderPath,
    _In_ PCWSTR layerData,
    _In_ PCWSTR options
    );

// Exports a legacy container writable layer.
HRESULT
WINAPI
HcsExportLegacyWritableLayer(
    _In_ PCWSTR writableLayerMountPath,
    _In_ PCWSTR writableLayerFolderPath,
    _In_ PCWSTR exportFolderPath,
    _In_ PCWSTR layerData
    );

// Deletes a container layer.
HRESULT
WINAPI
HcsDestroyLayer(
    _In_ PCWSTR layerPath
    );

// Sets up a layer that contains a base OS for a container.
HRESULT
WINAPI
HcsSetupBaseOSLayer(
    _In_ PCWSTR layerPath,
    _In_ HANDLE vhdHandle,
    _In_ PCWSTR options
    );

// Initializes a writable layer for a container.
HRESULT
WINAPI
HcsInitializeWritableLayer(
    _In_ PCWSTR writableLayerPath,
    _In_ PCWSTR layerData,
    _In_opt_ PCWSTR options
    );

// Initializes a writable layer for a container using the legacy hive folder format.
HRESULT
WINAPI
HcsInitializeLegacyWritableLayer(
    _In_ PCWSTR writableLayerMountPath,
    _In_ PCWSTR writableLayerFolderPath,
    _In_ PCWSTR layerData,
    _In_opt_ PCWSTR options
    );

// Sets up the layer storage filter on a writable container layer.
HRESULT
WINAPI
HcsAttachLayerStorageFilter(
    _In_ PCWSTR layerPath,
    _In_ PCWSTR layerData
    );

// Detaches the layer storage filter from a writable container layer.
HRESULT
WINAPI
HcsDetachLayerStorageFilter(
    _In_ PCWSTR layerPath
    );

// Formats a virtual disk for the use as a writable container layer.
HRESULT
WINAPI
HcsFormatWritableLayerVhd(
    _In_ HANDLE vhdHandle
    );

// Returns the volume path for a virtual disk of a writable container layer.
HRESULT
WINAPI
HcsGetLayerVhdMountPath(
    _In_ HANDLE vhdHandle,
    _Outptr_ PWSTR* mountPath
    );

// Same as HcsSetupBaseOSLayer except that this works on a volume.
HRESULT
WINAPI
HcsSetupBaseOSVolume(
    _In_ PCWSTR layerPath,
    _In_ PCWSTR volumePath,
    _In_ PCWSTR options
    );

// Sets up the unionFS layer storage filter on a writable container layer.
HRESULT
WINAPI
HcsAttachOverlayFilter(
    _In_ PCWSTR VolumeMountPoint,
    _In_ PCWSTR LayerData
    );

// Detaches the UnionFS filter from a writable container layer.
HRESULT
WINAPI
HcsDetachOverlayFilter(
    _In_ PCWSTR VolumeMountPoint,
    _In_ PCWSTR LayerData
    );

#ifdef __cplusplus
}
#endif

#endif // WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM)
#pragma endregion

#endif // _HYPERV_COMPUTESTORAGE_H_

#ifndef ext_ms_win_hyperv_computestorage_l1_1_2_query_routines
#define ext_ms_win_hyperv_computestorage_l1_1_2_query_routines

//
//Private Extension API Query Routines
//

#ifdef __cplusplus
extern "C" {
#endif

BOOLEAN
__stdcall
IsHcsImportLayerPresent(
    VOID
    );

BOOLEAN
__stdcall
IsHcsExportLayerPresent(
    VOID
    );

BOOLEAN
__stdcall
IsHcsExportLegacyWritableLayerPresent(
    VOID
    );

BOOLEAN
__stdcall
IsHcsDestroyLayerPresent(
    VOID
    );

BOOLEAN
__stdcall
IsHcsSetupBaseOSLayerPresent(
    VOID
    );

BOOLEAN
__stdcall
IsHcsInitializeWritableLayerPresent(
    VOID
    );

BOOLEAN
__stdcall
IsHcsInitializeLegacyWritableLayerPresent(
    VOID
    );

BOOLEAN
__stdcall
IsHcsAttachLayerStorageFilterPresent(
    VOID
    );

BOOLEAN
__stdcall
IsHcsDetachLayerStorageFilterPresent(
    VOID
    );

BOOLEAN
__stdcall
IsHcsFormatWritableLayerVhdPresent(
    VOID
    );

BOOLEAN
__stdcall
IsHcsGetLayerVhdMountPathPresent(
    VOID
    );

BOOLEAN
__stdcall
IsHcsSetupBaseOSVolumePresent(
    VOID
    );

BOOLEAN
__stdcall
IsHcsAttachOverlayFilterPresent(
    VOID
    );

BOOLEAN
__stdcall
IsHcsDetachOverlayFilterPresent(
    VOID
    );

#ifdef __cplusplus
}
#endif

#endif // endof guard

