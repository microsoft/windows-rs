/*++

Copyright (c) Microsoft Corporation. All rights reserved.

Module Name:

    VfWExt.h

Abstract:

    Constants and function prototypes needed to create IHV's extension DLL
    and constants used to programatically open a target capture device.
    

--*/
#include <winapifamily.h>

#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)


#include <prsht.h>

#define VFW_HIDE_SETTINGS_PAGE       0x00000001
#define VFW_HIDE_VIDEOSRC_PAGE       0x00000002
#define VFW_HIDE_CAMERACONTROL_PAGE  0x00000004
#define VFW_HIDE_ALL_PAGES           (VFW_HIDE_SETTINGS_PAGE |\
                                     VFW_HIDE_VIDEOSRC_PAGE  |\
                                     VFW_HIDE_CAMERACONTROL_PAGE)
#define VFW_OEM_ADD_PAGE             0x80000000  // If OEM has added any page


#define VFW_USE_DEVICE_HANDLE        0x00000001
#define VFW_USE_STREAM_HANDLE        0x00000002
#define VFW_QUERY_DEV_CHANGED        0x00000100  // if selected_dev == streaming_dev


//
// This is the function pointer that vfwwdm mapper calls to add an page
//
typedef 
DWORD (CALLBACK FAR * VFWWDMExtensionProc)(
	LPVOID					pfnDeviceIoControl, 
	LPFNADDPROPSHEETPAGE	pfnAddPropertyPage, 
	LPARAM					lParam);

//
// This is the function pointer that you can call to make DeviceIoControl() calls.
//
typedef 
BOOL (CALLBACK FAR * LPFNEXTDEVIO)(
					LPARAM lParam,	
					DWORD dwFlags,
					DWORD dwIoControlCode, 
					LPVOID lpInBuffer, 
					DWORD nInBufferSize, 
					LPVOID lpOutBuffer, 
					DWORD nOutBufferSize, 
					LPDWORD lpBytesReturned,
					LPOVERLAPPED lpOverlapped);
               

//                                                             
// HLM\System\CurrentControlSet\Control\MediaResources\msvideo\MSVideo.VFWWDM 
//
// Registry values used to allow a VfW client application to programatically
// open a target capture device.  The first is the FriendlyName of the capture
// device; and the 2nd flag if set, vfwwdm mapper will open only; if failed, 
// no attempt will be made by VfWWDM mapper to open other WDM capture device.
//
// Both registry value should be clear after capDriverConnect().  VfWWDM mapper
// will not clear them unless video source dialog box is chosen.
//                
#define TARGET_DEVICE_FRIENDLY_NAME     "TargetDeviceFriendlyName"      // REG_SZ
#define TARGET_DEVICE_OPEN_EXCLUSIVELY  "TargetDeviceOpenExclusively"   // REG_DWORD
               


#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion

