/*++

Copyright (c) 2000  Microsoft Corporation

Module Name:

    pdrvver.h

Abstract:

    Printer Driver Version Resource File

Environment:

    Windows NT printer drivers

Revision History:


--*/

//
// Printer Driver Version Resource File
// This file describes a template for the versioning scheme for printer drivers.
// It is recommended that this file be modified and included in every
// printer driver DLL resource file, even for monolithic printer drivers,
// which are not Unidrv-based minidrivers.
//
// The versioning scheme:
// The versioning scheme is defined in detail in the DDK documentation.
// Please refer to it for more information.
//

#include <winapifamily.h>

#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)

#include <ntverp.h>

//
// The following field sets the type of DLL. For printer drivers it should
// always be set to VFT_DRV. Don't change this field.
//

#define VER_FILETYPE                VFT_DRV

//
// If the DLL is of type VFT_DRV then the following field contains a more
// specific description of the driver. For versioned printer drivers it should
// always be set to VFT2_DRV_VERSIONED_PRINTER. Don't change this field.
//

#define VER_FILESUBTYPE             VFT2_DRV_VERSIONED_PRINTER

//
// For a versioned printer driver, the following field indicates the File
// Version of the DLL. The file version consists of four 16-bit (WORD) numbers.
// The description of each of them is as follows. From left to right,
//
//      First WORD  :   Reserved. Should be set to 0.
//      Second WORD :   Major version of the driver. For user-mode drivers, set
//                      this to 3. For kernel-mode drivers, set this to 2.
//      Third WORD  :   Feature set number.
//                      The high byte of this number represents the major
//                      feature set and should always be incremented with the
//                      next major release. A newer release is assumed to
//                      have a superset of the functionality of the previous
//                      release.
//                      The low byte represents minor releases -
//                      new releases from the same code base or the
//                      same architecture. The low byte should be
//                      incremented with each new minor release.
//      Fourth WORD :   Update number or service pack number. This field
//                      represents the releases of major feature set
//                      binaries for bugs or service packs and should always
//                      be incremented with each minor release.
//
// Example     :   0, 3, 0x0100, 0x0000
// In the above example the third number shows that the DLL belongs to the first
// major release of the driver. After fixing some bugs in this binary the
// version should be changed to  0, 3, 0x0100, 0x0001. For the next minor
// release of the driver, which is a superset of the previous release, the
// version  should be changed to 0, 3, 0x0101, 0x0001. In this case the low
// byte of the feature set was changed instead of the high byte. This allows
// more control on the versions. It is recommended that the high byte be used
// to describe different code bases or major architecture changes and the low
// byte be used for new releases from the same code base or from the same
// architecture.
//
// Basically the Feature set and Update numbers should be used in such a way
// that when they are combined (Feature set = HIWORD, Update = LOWORD) in a
// 32-bit quantity (DWORD), the new number represents all the features of the
// DLL. A larger number represents newer versions, which are supersets of
// functionality and updates with respect to all previous releases.
//
// This field needs to be changed per the description above.
//
//

#define VER_FILEVERSION             0, 3, 0, 0

//
// The following field describes the driver. It should include a specific name,
// which identifies the driver.
//
// This field needs to be changed.
//

#define VER_FILEDESCRIPTION_STR     "Sample Printer Driver Resource DLL"

//
// The following field specifies the internal name of the file. For more
// information, refer to the SDK documentation.
//
// This field needs to be changed.
//

#define VER_INTERNALNAME_STR        "SAMPLERES.DLL"

//
// The following field specifies the original name of the file, not including a
// path. For more information refer to the SDK documentation.
//
// This field needs to be changed.
//

#define VER_ORIGINALFILENAME_STR    "SAMPLERES.DLL"

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion

