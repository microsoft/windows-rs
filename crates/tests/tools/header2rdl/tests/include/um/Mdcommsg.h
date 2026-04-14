/*++

Copyright (c) 1997-1999 Microsoft Corporation

Module Name:

    commsg.h

Abstract:

    HRESULT <-> Win32 error mapping macros.

Author:

    Michael W. Thomas (michth)   24-Sep-1996

Revision History:

    Keith Moore (keithmo)        07-Feb-1997
        Cleanup, comment, made Metadata errors "real" HRESULTs.

--*/


#ifndef _COMMSG_H_
#define _COMMSG_H_

#include <winapifamily.h>

#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)


//
// RETURNCODETOHRESULT() maps a return code to an HRESULT. If the return
// code is a Win32 error (identified by a zero high word) then it is mapped
// using the standard HRESULT_FROM_WIN32() macro. Otherwise, the return
// code is assumed to already be an HRESULT and is returned unchanged.
//

#define RETURNCODETOHRESULT(rc)                             \
            (((rc) < 0x10000)                               \
                ? HRESULT_FROM_WIN32(rc)                  \
                : (rc))


//
// HRESULTTOWIN32() maps an HRESULT to a Win32 error. If the facility code
// of the HRESULT is FACILITY_WIN32, then the code portion (i.e. the
// original Win32 error) is returned. Otherwise, the original HRESULT is
// returned unchagned.
//

#define HRESULTTOWIN32(hres)                                \
            ((HRESULT_FACILITY(hres) == FACILITY_WIN32)     \
                ? HRESULT_CODE(hres)                        \
                : (hres))


#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion

#endif  // _COMMSG_H_


