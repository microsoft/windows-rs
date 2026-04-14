
#ifdef _MSC_VER
#pragma once
#endif // _MSC_VER
#include <shtypes.h>

#include <winapifamily.h>

#pragma region Desktop Family or OneCore Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM)

#ifndef SCALING_ENUMS_DECLARED

// The scaling API only operates over the primary display and the immersive display.  Scaling information
// is not provided for displays other than those.
// This enum matches #defines in IETouch.h.  Don't update this without keeping that in sync.
typedef enum
{
    DEVICE_PRIMARY = 0,
    DEVICE_IMMERSIVE = 1,
} DISPLAY_DEVICE_TYPE;

typedef enum
{
    SCF_VALUE_NONE = 0x00,
    SCF_SCALE = 0x01,
    SCF_PHYSICAL = 0x02,
} SCALE_CHANGE_FLAGS;
DEFINE_ENUM_FLAG_OPERATORS(SCALE_CHANGE_FLAGS);

#define SCALING_ENUMS_DECLARED
#endif

#if (NTDDI_VERSION >= NTDDI_WIN8)

// Given a display device, return the preferred DEVICE_SCALE_FACTOR to be used for scaling values.
// Default is SCALE_100_PERCENT
STDAPI_(DEVICE_SCALE_FACTOR) GetScaleFactorForDevice(_In_ DISPLAY_DEVICE_TYPE deviceType);

// Register a window to receive callbacks when scaling information changes.  The uMsgNotify param specifies a message
// that will be posted to the requesting window.  The wParam for this message is a combination of SCALE_CHANGE_FLAGS
STDAPI RegisterScaleChangeNotifications(_In_ DISPLAY_DEVICE_TYPE displayDevice, _In_ HWND hwndNotify, _In_ UINT uMsgNotify, _Out_ DWORD *pdwCookie);
STDAPI RevokeScaleChangeNotifications(_In_ DISPLAY_DEVICE_TYPE displayDevice, _In_ DWORD dwCookie);

#endif // (NTDDI_VERSION >= NTDDI_WIN8)

#if (NTDDI_VERSION >= NTDDI_WINBLUE)

STDAPI GetScaleFactorForMonitor(_In_ HMONITOR hMon, _Out_ DEVICE_SCALE_FACTOR *pScale);
STDAPI RegisterScaleChangeEvent(_In_ HANDLE hEvent, _Out_ DWORD_PTR *pdwCookie);
STDAPI UnregisterScaleChangeEvent(_In_ DWORD_PTR dwCookie);

#endif // (NTDDI_VERSION >= NTDDI_WINBLUE)

#ifndef DPI_ENUMS_DECLARED

typedef enum PROCESS_DPI_AWARENESS {
    PROCESS_DPI_UNAWARE = 0,
    PROCESS_SYSTEM_DPI_AWARE = 1,
    PROCESS_PER_MONITOR_DPI_AWARE = 2
} PROCESS_DPI_AWARENESS;

typedef enum MONITOR_DPI_TYPE {
    MDT_EFFECTIVE_DPI = 0,
    MDT_ANGULAR_DPI = 1,
    MDT_RAW_DPI = 2,
    MDT_DEFAULT = MDT_EFFECTIVE_DPI
} MONITOR_DPI_TYPE;

#define DPI_ENUMS_DECLARED
#endif // (DPI_ENUMS_DECLARED)

#if (NTDDI_VERSION >= NTDDI_WINBLUE)

STDAPI SetProcessDpiAwareness(
    _In_ PROCESS_DPI_AWARENESS value);

STDAPI GetProcessDpiAwareness(
    _In_opt_ HANDLE hprocess,
    _Out_ PROCESS_DPI_AWARENESS *value);

STDAPI GetDpiForMonitor(
    _In_ HMONITOR hmonitor,
    _In_ MONITOR_DPI_TYPE dpiType,
    _Out_ UINT *dpiX,
    _Out_ UINT *dpiY);

#endif // (NTDDI_VERSION >= NTDDI_WINBLUE)

#if (NTDDI_VERSION >= NTDDI_WINTHRESHOLD)

#ifndef SHELL_UI_COMPONENT_ENUMS_DECLARED

typedef enum
{
    SHELL_UI_COMPONENT_TASKBARS         = 0,
    SHELL_UI_COMPONENT_NOTIFICATIONAREA = 1,
    SHELL_UI_COMPONENT_DESKBAND         = 2,
} SHELL_UI_COMPONENT;

#define SHELL_UI_COMPONENT_ENUMS_DECLARED
#endif // (SHELL_UI_COMPONENT_ENUMS_DECLARED)

STDAPI_(UINT) GetDpiForShellUIComponent(
    _In_ SHELL_UI_COMPONENT);

#endif // (NTDDI_VERSION >= NTDDI_WINTHRESHOLD)

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM) */
#pragma endregion

