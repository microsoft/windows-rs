/*++

Copyright (c) Microsoft Corporation.  All rights reserved.

Module Name:

    spapidef.h

Abstract:

    Public header file for Windows NT Setup and Device Installer services Dll.

--*/

#ifndef _INC_SPAPIDEF
#define _INC_SPAPIDEF

#if _MSC_VER > 1000
#pragma once
#endif
#include <winapifamily.h>

#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)


#ifndef SP_LOG_TOKEN
typedef DWORDLONG SP_LOG_TOKEN;
typedef DWORDLONG *PSP_LOG_TOKEN;
#endif


//
// Special txtlog token values
//

#define LOGTOKEN_TYPE_MASK              3

#define LOGTOKEN_UNSPECIFIED            0
#define LOGTOKEN_NO_LOG                 1
#define LOGTOKEN_SETUPAPI_APPLOG        2
#define LOGTOKEN_SETUPAPI_DEVLOG        3


//
// Flags for SetupCreateTextLogSection
//

#define TXTLOG_SETUPAPI_DEVLOG      0x00000001            // 1 = setupdi.log, 0 = setupapi.log
#define TXTLOG_SETUPAPI_CMDLINE     0x00000002            // log the command line

#define TXTLOG_SETUPAPI_BITS        0x00000003


//
// Flags for SetupWriteTextLog
//

//
// Event Levels (bits 0-3)
//

#define TXTLOG_ERROR                    0x1             // shows entries which indicate a real problem
#define TXTLOG_WARNING                  0x2             // shows entries which indicate a potential problem
#define TXTLOG_SYSTEM_STATE_CHANGE      0x3             // system changes only
#define TXTLOG_SUMMARY                  0x4             // show basic operation surrounding system changes
#define TXTLOG_DETAILS                  0x5             // detailed operation of the install process
#define TXTLOG_VERBOSE                  0x6             // log entries which potentially generate a lot of data
#define TXTLOG_VERY_VERBOSE             0x7             // highest level shows all log entries

//
// Bits reserved for internal use
//

#define TXTLOG_RESERVED_FLAGS   0x0000FFF0

//
// Basic flags (bits 4-31)
//

#define TXTLOG_TIMESTAMP        0x00010000
#define TXTLOG_DEPTH_INCR       0x00020000
#define TXTLOG_DEPTH_DECR       0x00040000
#define TXTLOG_TAB_1            0x00080000
#define TXTLOG_FLUSH_FILE       0x00100000

#define TXTLOG_LEVEL(flags) (flags & 0xf)


//
// Setupapi, Setupdi event categories
//

#define TXTLOG_DEVINST          0x00000001
#define TXTLOG_INF              0x00000002
#define TXTLOG_FILEQ            0x00000004
#define TXTLOG_COPYFILES        0x00000008

#define TXTLOG_SIGVERIF         0x00000020

#define TXTLOG_BACKUP           0x00000080
#define TXTLOG_UI               0x00000100
#define TXTLOG_UTIL             0x00000200
#define TXTLOG_INFDB            0x00000400

#define TXTLOG_DRVSETUP         0x00400000
#define TXTLOG_POLICY           0x00800000
#define TXTLOG_NEWDEV           0x01000000
#define TXTLOG_UMPNPMGR         0x02000000
#define TXTLOG_DRIVER_STORE     0x04000000
#define TXTLOG_SETUP            0x08000000
#define TXTLOG_CMI              0x10000000
#define TXTLOG_DEVMGR           0x20000000

#define TXTLOG_INSTALLER        0x40000000
#define TXTLOG_VENDOR           0x80000000



#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion

#endif // _INC_SPAPIDEF

