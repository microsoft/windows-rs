#include <winapifamily.h>

/*++ BUILD Version: 0002    // Increment this if a change has global effects

Copyright (c) Microsoft Corporation. All rights reserved.

Module Name:

    wincon.h

Abstract:

    This module contains the public data structures, data types,
    and procedures exported by the NT console subsystem.

Created:

    26-Oct-1990

Revision History:

--*/

#ifndef _WINCON_
#define _WINCON_

#pragma once

#ifdef __cplusplus
extern "C" {
#endif

#if _MSC_VER >= 1200
#pragma warning(push)
#pragma warning(disable:4820) // padding added after data member
#endif

#include <wincontypes.h>


#ifndef NOGDI
#include <wingdi.h>
#endif

#ifndef NOAPISET
#include <consoleapi.h>
#include <consoleapi2.h>
#include <consoleapi3.h>
#endif

#define CONSOLE_REAL_OUTPUT_HANDLE (LongToHandle(-2))
#define CONSOLE_REAL_INPUT_HANDLE (LongToHandle(-3))

#define CONSOLE_TEXTMODE_BUFFER  1

#if(_WIN32_WINNT >= 0x0500)
#endif /* _WIN32_WINNT >= 0x0500 */


#if _MSC_VER >= 1200
#pragma warning(pop)
#endif

#ifdef __cplusplus
}
#endif

#endif // _WINCON_

