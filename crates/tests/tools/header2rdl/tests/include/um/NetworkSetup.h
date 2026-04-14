// Copyright (C) Microsoft.  All rights reserved.
#pragma once

#ifdef __cplusplus
extern "C" {
#endif

HRESULT WINAPI
NetSetupSetFilterClassTopDriver(
    _In_ PCWSTR ClassName,
    _In_ const GUID *NetCfgInstanceId);

HRESULT WINAPI
NetSetupGetFilterClassTopDriver(
    _In_ PCWSTR ClassName,
    _Out_ GUID *NetCfgInstanceId);

HRESULT WINAPI
NetSetupResetFilterClassTopDriver(
    _In_ PCWSTR ClassName);

HRESULT WINAPI
NetSetupSetFilterClassBottomDriver(
    _In_ PCWSTR ClassName,
    _In_ const GUID *NetCfgInstanceId);

HRESULT WINAPI
NetSetupGetFilterClassBottomDriver(
    _In_ PCWSTR ClassName,
    _Out_ GUID *NetCfgInstanceId);

HRESULT WINAPI
NetSetupResetFilterClassBottomDriver(
    _In_ PCWSTR ClassName);

#ifdef __cplusplus
}
#endif
