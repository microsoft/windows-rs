// Copyright (c) Microsoft Corporation.  All rights reserved.

#pragma once

#if !defined(__cplusplus)
#error C++11 required
#endif

#include <stdint.h>
#include <unknwn.h>

#pragma region Application Family or OneCore Family or Games Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES)

extern "C" {

enum class LampPurposes : uint32_t
{
    Undefined       = 0x00,
    Control         = 0x01,
    Accent          = 0x02,
    Branding        = 0x04,
    Status          = 0x08,
    Illumination    = 0x10,
    Presentation    = 0x20,
};

DEFINE_ENUM_FLAG_OPERATORS(LampPurposes);

enum class LampArrayKind : uint32_t
{
    Undefined       = 0,
    Keyboard        = 1,
    Mouse           = 2,
    GameController  = 3,
    Peripheral      = 4,
    Scene           = 5,
    Notification    = 6,
    Chassis         = 7,
    Wearable        = 8,
    Furniture       = 9,
    Art             = 10,
    Headset         = 11,
    Microphone      = 12,
    Speaker         = 13
};

enum class LampArrayEnumerationKind : uint32_t
{
    Async       = 1,
    Blocking    = 2
};

enum class LampArrayStatus : uint32_t
{
    None            = 0x00000000,
    Connected       = 0x00000001,
    Available       = 0x00000002
};

DEFINE_ENUM_FLAG_OPERATORS(LampArrayStatus);

typedef interface ILampArray ILampArray;
typedef interface ILampInfo ILampInfo;

typedef uint64_t LampArrayCallbackToken;

inline constexpr uint64_t LAMPARRAY_INVALID_CALLBACK_TOKEN_VALUE = 0x0000000000000000ULL;

typedef void (CALLBACK * LampArrayCallback)(
    _In_opt_ void * context,
    bool isAttached,
    _In_ ILampArray * lampArray);

typedef void (CALLBACK * LampArrayStatusCallback)(
    _In_opt_ void * context,
    LampArrayStatus currentStatus,
    LampArrayStatus previousStatus,
    _In_ ILampArray * lampArray);

struct LampArrayColor
{
    uint8_t r;
    uint8_t g;
    uint8_t b;
    uint8_t a;
};

struct LampArrayPosition
{
    float xInMeters;
    float yInMeters;
    float zInMeters;
};

#undef INTERFACE
#define INTERFACE ILampInfo
DECLARE_INTERFACE_IID_(INTERFACE, IUnknown, "AE82EDD1-6B75-43CF-9BEA-F600A49A320C")
{
    _Must_inspect_result_
    IFACEMETHOD(QueryInterface)(THIS_
        _In_ REFIID riid,
        _COM_Outptr_ void ** ppvObj) PURE;

    IFACEMETHOD_(ULONG, AddRef)(THIS) PURE;

    IFACEMETHOD_(ULONG, Release)(THIS) PURE;

    IFACEMETHOD_(uint32_t, GetIndex)(THIS) PURE;

    IFACEMETHOD_(LampPurposes, GetPurposes)(THIS) PURE;

    IFACEMETHOD_(void, GetPosition)(THIS_
        _Out_ LampArrayPosition * position) PURE;

    IFACEMETHOD_(uint32_t, GetRedLevelCount)(THIS) PURE;

    IFACEMETHOD_(uint32_t, GetGreenLevelCount)(THIS) PURE;

    IFACEMETHOD_(uint32_t, GetBlueLevelCount)(THIS) PURE;

    IFACEMETHOD_(uint32_t, GetGainLevelCount)(THIS) PURE;

    IFACEMETHOD_(bool, GetFixedColor)(THIS_
        _Out_ LampArrayColor * fixedColor) PURE;

    IFACEMETHOD_(LampArrayColor, GetNearestSupportedColor)(THIS_
        LampArrayColor desiredColor) PURE;

    IFACEMETHOD_(uint64_t, GetUpdateLatencyInMicroseconds)(THIS) PURE;

    IFACEMETHOD_(uint32_t, GetScanCode)(THIS) PURE;
};

#undef INTERFACE
#define INTERFACE ILampArray
DECLARE_INTERFACE_IID_(INTERFACE, IUnknown, "45577878-466D-40ED-AB72-C448C70E1252")
{
    _Must_inspect_result_
    IFACEMETHOD(QueryInterface)(THIS_
        _In_ REFIID riid,
        _COM_Outptr_ void ** ppvObj) PURE;

    IFACEMETHOD_(ULONG, AddRef)(THIS) PURE;

    IFACEMETHOD_(ULONG, Release)(THIS) PURE;

    IFACEMETHOD_(void, GetDeviceId)(THIS_
        _Out_ APP_LOCAL_DEVICE_ID * deviceId) PURE;

    IFACEMETHOD_(uint16_t, GetVendorId)(THIS) PURE;

    IFACEMETHOD_(uint16_t, GetProductId)(THIS) PURE;

    IFACEMETHOD_(uint16_t, GetHardwareVersion)(THIS) PURE;

    IFACEMETHOD_(LampArrayKind, GetLampArrayKind)(THIS) PURE;

    IFACEMETHOD_(uint32_t, GetLampCount)(THIS) PURE;

    IFACEMETHOD_(uint64_t, GetMinUpdateIntervalInMicroseconds)(THIS) PURE;

    IFACEMETHOD_(void, GetBoundingBox)(THIS_
        _Out_ LampArrayPosition * boundingBox) PURE;

    IFACEMETHOD_(bool, GetIsEnabled)(THIS) PURE;

    IFACEMETHOD_(void, SetIsEnabled)(THIS_
        bool isEnabled) PURE;

    IFACEMETHOD_(double, GetBrightnessLevel)(THIS) PURE;

    IFACEMETHOD_(void, SetBrightnessLevel)(THIS_
        double brightnessLevel) PURE;
    
    IFACEMETHOD_(bool, SupportsScanCodes)(THIS) PURE;

    _Must_inspect_result_
    IFACEMETHOD(GetLampInfo)(THIS_
        uint32_t lampIndex,
        _COM_Outptr_ ILampInfo ** lampInfo) PURE;

    IFACEMETHOD_(uint32_t, GetIndicesCountForPurposes)(THIS_
        LampPurposes lampPurposes) PURE;

    IFACEMETHOD_(void, GetIndicesForPurposes)(THIS_
        LampPurposes lampPurposes,
        uint32_t lampIndicesCount,
        _Out_writes_(lampIndicesCount) uint32_t * lampIndices) PURE;

    IFACEMETHOD_(void, SetColor)(THIS_
        LampArrayColor desiredColor) PURE;

    IFACEMETHOD_(void, SetColorsForIndices)(THIS_
        uint32_t lampIndicesCount,
        _In_reads_(lampIndicesCount) const uint32_t * lampIndices,
        _In_reads_(lampIndicesCount) const LampArrayColor * desiredColors) PURE;

    IFACEMETHOD_(void, SetColorsForScanCodes)(THIS_
        uint32_t scanCodesCount,
        _In_reads_(scanCodesCount) const uint32_t * scanCodes,
        _In_reads_(scanCodesCount) const LampArrayColor * desiredColors) PURE;
};

#undef INTERFACE

STDAPI RegisterLampArrayCallback(
    _In_ LampArrayCallback lampArrayCallback,
    _In_opt_ void * context,
    _Out_ _Result_zeroonfailure_ LampArrayCallbackToken * callbackToken);

STDAPI RegisterLampArrayStatusCallback(
    _In_ LampArrayStatusCallback callbackFunc,
    LampArrayEnumerationKind enumerationKind,
    _In_opt_ void * context,
    _Out_ _Result_zeroonfailure_ LampArrayCallbackToken * callbackToken);

STDAPI_(bool) UnregisterLampArrayCallback(
    LampArrayCallbackToken callbackToken,
    uint64_t timeoutInMicroseconds);

STDAPI TrySetLampArrayWorkerThreadAffinityMask(
    uint64_t threadAffinityMask);

}       // extern "C"

#endif  // #if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES)
#pragma endregion

//
// MessageId: LAMPARRAY_E_CALLBACK_REGISTRATION_WRONG_THREAD
//
// MessageText:
//
// The callback is being registered or unregistered from within a registered callback, which is not supported.
//
#define LAMPARRAY_E_CALLBACK_REGISTRATION_WRONG_THREAD _HRESULT_TYPEDEF_(0x84980001L)
