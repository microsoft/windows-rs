/*++

Copyright (c) Microsoft Corporation.  All rights reserved.

Module Name:

    WBEMIDL.H

Abstract:

  Include file for all WBEM related interface definitions. To be included
  in projects that use any WBEM interfaces.

History:

--*/

#ifndef __WBEMIDL_H_
#define __WBEMIDL_H_
#include <winapifamily.h>

#pragma region Desktop Family or WinMgmt Package
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_PKG_WINMGMT)


#include <wbemcli.h>
#include <wbemprov.h>
#include <wbemtran.h>
#include <wbemdisp.h>


#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_PKG_WINMGMT) */
#pragma endregion

#endif
