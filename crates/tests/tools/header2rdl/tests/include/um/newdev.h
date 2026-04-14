/*++

Copyright (c) Microsoft Corporation.  All rights reserved.

Module Name:

    newdev.h

Abstract:

    Public header file for Windows NT Driver Installation DLL.

--*/

#ifndef _INC_NEWDEV
#define _INC_NEWDEV

#include <winapifamily.h>

#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)

#include <pshpack1.h>   // Assume byte packing throughout
#include <setupapi.h>   // for HDEVINFO, PSP_DEVINFO_DATA, and PSP_DRVINFO_DATA

#ifdef __cplusplus
extern "C" {
#endif

//
// Flags for UpdateDriverForPlugAndPlayDevices
//
#define INSTALLFLAG_FORCE               0x00000001  // Force the installation of the specified driver
#define INSTALLFLAG_READONLY            0x00000002  // Do a read-only install (no file copy)
#define INSTALLFLAG_NONINTERACTIVE      0x00000004  // No UI shown at all. API will fail if any UI must be shown.
#define INSTALLFLAG_BITS                0x00000007

#if (WINVER >= _WIN32_WINNT_WIN2K)

BOOL
WINAPI
UpdateDriverForPlugAndPlayDevicesA(
    _In_opt_  HWND hwndParent,
    _In_      LPCSTR HardwareId,
    _In_      LPCSTR FullInfPath,
    _In_      DWORD InstallFlags,
    _Out_opt_ PBOOL bRebootRequired
    );

BOOL
WINAPI
UpdateDriverForPlugAndPlayDevicesW(
    _In_opt_  HWND hwndParent,
    _In_      LPCWSTR HardwareId,
    _In_      LPCWSTR FullInfPath,
    _In_      DWORD InstallFlags,
    _Out_opt_ PBOOL bRebootRequired
    );

#ifdef UNICODE
#define UpdateDriverForPlugAndPlayDevices UpdateDriverForPlugAndPlayDevicesW
#else
#define UpdateDriverForPlugAndPlayDevices UpdateDriverForPlugAndPlayDevicesA
#endif
#endif // (WINVER >= _WIN32_WINNT_WIN2K)

#if (WINVER >= _WIN32_WINNT_LONGHORN)
//
// Flags for DiInstallDevice
//
#define DIIDFLAG_SHOWSEARCHUI           0x00000001      // Show search UI if no drivers can be found.
#define DIIDFLAG_NOFINISHINSTALLUI      0x00000002      // Do NOT show the finish install UI.
#define DIIDFLAG_INSTALLNULLDRIVER      0x00000004      // Install the NULL driver on this device.
#define DIIDFLAG_INSTALLCOPYINFDRIVERS  0x00000008      // Install any extra INFs specified via CopyInf directive.
#define DIIDFLAG_BITS                   0x0000000F

BOOL
WINAPI
DiInstallDevice(
    _In_opt_  HWND hwndParent,
    _In_      HDEVINFO DeviceInfoSet,
    _In_      PSP_DEVINFO_DATA DeviceInfoData,
    _In_opt_  PSP_DRVINFO_DATA DriverInfoData,
    _In_      DWORD Flags,
    _Out_opt_ PBOOL NeedReboot
    );

//
// Flags for DiInstallDriver
//
#define DIIRFLAG_INF_ALREADY_COPIED 0x00000001   // Don't copy inf, it has been published
#define DIIRFLAG_FORCE_INF          0x00000002   // use the inf as if users picked it.
#define DIIRFLAG_HW_USING_THE_INF   0x00000004   // limit installs on hw using the inf
#define DIIRFLAG_HOTPATCH           0x00000008   // Perform a hotpatch service pack install
#define DIIRFLAG_NOBACKUP           0x00000010   // install w/o backup and no rollback

#if (WINVER >= _WIN32_WINNT_WINBLUE)
#define DIIRFLAG_PRE_CONFIGURE_INF  0x00000020   // Pre-install inf, if possible
#define DIIRFLAG_INSTALL_AS_SET     0x00000040
#endif

#if (WINVER < _WIN32_WINNT_WINBLUE)

#define DIIRFLAG_BITS        ( DIIRFLAG_FORCE_INF |\
                               DIIRFLAG_HOTPATCH)
#define DIIRFLAG_SYSTEM_BITS ( DIIRFLAG_INF_ALREADY_COPIED |\
                               DIIRFLAG_FORCE_INF          |\
                               DIIRFLAG_HW_USING_THE_INF   |\
                               DIIRFLAG_HOTPATCH           |\
                               DIIRFLAG_NOBACKUP )

#else // (WINVER >= _WIN32_WINNT_WINBLUE)

#define DIIRFLAG_BITS        ( DIIRFLAG_FORCE_INF         |\
                               DIIRFLAG_HOTPATCH          |\
                               DIIRFLAG_PRE_CONFIGURE_INF |\
                               DIIRFLAG_INSTALL_AS_SET )
#define DIIRFLAG_SYSTEM_BITS ( DIIRFLAG_INF_ALREADY_COPIED |\
                               DIIRFLAG_FORCE_INF          |\
                               DIIRFLAG_HW_USING_THE_INF   |\
                               DIIRFLAG_HOTPATCH           |\
                               DIIRFLAG_NOBACKUP           |\
                               DIIRFLAG_PRE_CONFIGURE_INF  |\
                               DIIRFLAG_INSTALL_AS_SET )

#endif // (WINVER >= _WIN32_WINNT_WINBLUE)

BOOL
WINAPI
DiInstallDriverW(
    _In_opt_  HWND hwndParent,
    _In_      LPCWSTR InfPath,
    _In_      DWORD Flags,
    _Out_opt_ PBOOL NeedReboot
    );

BOOL
WINAPI
DiInstallDriverA(
    _In_opt_  HWND hwndParent,
    _In_      LPCSTR InfPath,
    _In_      DWORD Flags,
    _Out_opt_ PBOOL NeedReboot
    );


#ifdef UNICODE
#define DiInstallDriver DiInstallDriverW
#else
#define DiInstallDriver DiInstallDriverA
#endif
#endif // (WINVER >= _WIN32_WINNT_LONGHORN)


#if (WINVER >= _WIN32_WINNT_WIN7)
BOOL
WINAPI
DiUninstallDevice(
    _In_      HWND hwndParent,
    _In_      HDEVINFO DeviceInfoSet,
    _In_      PSP_DEVINFO_DATA DeviceInfoData,
    _In_      DWORD Flags,
    _Out_opt_ PBOOL NeedReboot
    );
#endif // (WINVER >= _WIN32_WINNT_WIN7)


#if (WINVER >= _WIN32_WINNT_WIN10) // _WIN32_WINNT_WIN10_RS2

//
// Flags for DiUninstallDriver
//
#define DIURFLAG_NO_REMOVE_INF      (0x00000001)   // Do not remove inf from the system
#define DIURFLAG_RESERVED           (0x00000002)   // DIURFLAG_UNCONFIGURE_INF is obsolete.

#define DIURFLAG_VALID              (DIURFLAG_NO_REMOVE_INF |\
                                     DIURFLAG_RESERVED      )

BOOL
WINAPI
DiUninstallDriverW(
    _In_opt_  HWND hwndParent,
    _In_      LPCWSTR InfPath,
    _In_      DWORD Flags,
    _Out_opt_ PBOOL NeedReboot
    );

BOOL
WINAPI
DiUninstallDriverA(
    _In_opt_  HWND hwndParent,
    _In_      LPCSTR InfPath,
    _In_      DWORD Flags,
    _Out_opt_ PBOOL NeedReboot
    );

#ifdef UNICODE
#define DiUninstallDriver DiUninstallDriverW
#else
#define DiUninstallDriver DiUninstallDriverA
#endif
#endif // (WINVER >= _WIN32_WINNT_WIN10) // _WIN32_WINNT_WIN10_RS2


#if (WINVER >= _WIN32_WINNT_LONGHORN)
BOOL
WINAPI
DiShowUpdateDevice(
    _In_opt_  HWND hwndParent,
    _In_      HDEVINFO DeviceInfoSet,
    _In_      PSP_DEVINFO_DATA DeviceInfoData,
    _In_      DWORD Flags,
    _Out_opt_ PBOOL NeedReboot
    );


//
// DiRollbackDriver Flag values
//
#define ROLLBACK_FLAG_NO_UI             0x00000001  // don't show any UI (this could cause failures if UI must be displayed)
#define ROLLBACK_BITS                   0x00000001


BOOL
WINAPI
DiRollbackDriver(
    _In_      HDEVINFO DeviceInfoSet,
    _In_      PSP_DEVINFO_DATA DeviceInfoData,
    _In_opt_  HWND hwndParent,
    _In_      DWORD Flags,
    _Out_opt_ PBOOL NeedReboot
    );
#endif // (WINVER >= _WIN32_WINNT_LONGHORN)

#if (WINVER >= _WIN32_WINNT_WIN10)
BOOL
WINAPI
DiShowUpdateDriver(
    __in_opt  HWND hwndParent,
    __in_opt  PCWSTR FilePath,
    __in      DWORD Flags,
    __out_opt PBOOL NeedReboot
    );
#endif // (WINVER >= _WIN32_WINNT_WIN10)

#ifdef __cplusplus
}
#endif

#include <poppack.h>


#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion

#endif // _INC_NEWDEV

