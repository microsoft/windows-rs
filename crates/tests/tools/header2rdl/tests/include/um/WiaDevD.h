/****************************************************************************
*
*  (C) COPYRIGHT 1998-2000, MICROSOFT CORP.
*
*  FILE:        wiadevd.h
*
*  VERSION:     1.0
*
*  DATE:        7/5/1999
*
*  DESCRIPTION:
*    Device Dialog and UI extensibility declarations.
*
*****************************************************************************/

#if (_WIN32_WINNT >= 0x0501) // Windows XP and later

#ifndef _WIADEVD_H_INCLUDED
#define _WIADEVD_H_INCLUDED
#include <winapifamily.h>

#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)


#include "wia.h"

#if defined(__cplusplus)
extern "C" {
#endif

#include <pshpack8.h>

#if (_WIN32_WINNT >= 0x0600) // Windows Vista and later

#undef  INTERFACE
#define INTERFACE IWiaUIExtension2
typedef struct tagDEVICEDIALOGDATA2
{
    DWORD           cbSize;           // Size of the structure in bytes
    IWiaItem2       *pIWiaItemRoot;   // Valid root item
    DWORD           dwFlags;          // Flags
    HWND            hwndParent;       // Parent window
    BSTR            bstrFolderName;   // Folder name where the files are transferred
    BSTR            bstrFilename;     // template file name.
    LONG            lNumFiles;        // Number of items in ppbstrFilePaths array.  Filled on return.
    BSTR            *pbstrFilePaths;  // file names created after successful transfers.
    IWiaItem2       *pWiaItem;        // IWiaItem2 interface pointer.  This is the IWiaItem2 used for transfer.
} DEVICEDIALOGDATA2, *LPDEVICEDIALOGDATA2, *PDEVICEDIALOGDATA2;

DECLARE_INTERFACE_IID_(IWiaUIExtension2, IUnknown, "305600d7-5088-46d7-9a15-b77b09cdba7a")
{
    // *** IUnknown methods ***
    STDMETHOD(QueryInterface) (THIS_ REFIID riid, _COM_Outptr_ LPVOID* ppvObj) PURE;
    STDMETHOD_(ULONG,AddRef) (THIS) PURE;
    STDMETHOD_(ULONG,Release) (THIS) PURE;

    // *** IWiaUIExtension2 methods ***
    STDMETHOD(DeviceDialog)(_In_ THIS_ PDEVICEDIALOGDATA2 pDeviceDialogData ) PURE;
    STDMETHOD(GetDeviceIcon)(_In_ THIS_ BSTR bstrDeviceId, _Out_ HICON *phIcon, ULONG nSize) PURE;
};

// {305600d7-5088-46d7-9a15-b77b09cdba7a}
DEFINE_GUID(IID_IWiaUIExtension2, 0x305600D7, 0x5088, 0x46D7, 0x9A, 0x15, 0xB7, 0x7B, 0x09, 0xCD, 0xBA, 0x7A);

#endif //#if (_WIN32_WINNT >= 0x0600)

typedef struct tagDEVICEDIALOGDATA
{
    DWORD            cbSize;           // Size of the structure in bytes
    HWND             hwndParent;       // Parent window
    IWiaItem         *pIWiaItemRoot;   // Valid root item
    DWORD            dwFlags;          // Flags
    LONG             lIntent;          // Intent flags
    LONG             lItemCount;       // Number of items in ppWiaItems array.  Filled on return.
    IWiaItem         **ppWiaItems;     // Array of IWiaItem interface pointers.  Array must
                                       // be allocated using CoTaskMemAlloc, and all interface pointers must be AddRef'ed
} DEVICEDIALOGDATA, *LPDEVICEDIALOGDATA, *PDEVICEDIALOGDATA;

HRESULT WINAPI DeviceDialog(_In_ PDEVICEDIALOGDATA pDeviceDialogData);

// IWiaUIExtension provides a means to replace a device's image acquisition dialog
// and to provide custom icons and logo bitmaps to appear on the standard dialog
#undef  INTERFACE
#define INTERFACE IWiaUIExtension
DECLARE_INTERFACE_IID_(IWiaUIExtension, IUnknown, "da319113-50ee-4c80-b460-57d005d44a2c")
{
    // *** IUnknown methods ***
    STDMETHOD(QueryInterface) (THIS_ REFIID riid, _COM_Outptr_ LPVOID* ppvObj) PURE;
    STDMETHOD_(ULONG,AddRef) (THIS) PURE;
    STDMETHOD_(ULONG,Release) (THIS) PURE;

    // *** IWiaUIExtension methods ***
    STDMETHOD(DeviceDialog)(_In_ THIS_ PDEVICEDIALOGDATA pDeviceDialogData) PURE;
    STDMETHOD(GetDeviceIcon)(_In_ THIS_ BSTR bstrDeviceId, _Out_ HICON *phIcon, ULONG nSize) PURE;
    STDMETHOD(GetDeviceBitmapLogo)(_In_ THIS_ BSTR bstrDeviceId, _Out_ HBITMAP *phBitmap, ULONG nMaxWidth, ULONG nMaxHeight) PURE;
};

// {da319113-50ee-4c80-b460-57d005d44a2c}
DEFINE_GUID(IID_IWiaUIExtension, 0xDA319113, 0x50EE, 0x4C80, 0xB4, 0x60, 0x57, 0xD0, 0x05, 0xD4, 0x4A, 0x2C);

typedef HRESULT (WINAPI *DeviceDialogFunction)(PDEVICEDIALOGDATA);

#define SHELLEX_WIAUIEXTENSION_NAME TEXT("WiaDialogExtensionHandlers")

// Define clipboard format names for retrieving data from an IDataObject
#define CFSTR_WIAITEMNAMES TEXT("WIAItemNames")
#define CFSTR_WIAITEMPTR   TEXT("WIAItemPointer")

#include <poppack.h>

#if defined(__cplusplus)
};
#endif


#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion

#endif // !_WIADEVD_H_INCLUDED

#endif //#if (_WIN32_WINNT >= 0x0501)
