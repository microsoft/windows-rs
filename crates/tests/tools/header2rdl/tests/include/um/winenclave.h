#include <winapifamily.h>
/*++

Copyright (c) Microsoft Corporation. All rights reserved.

Module Name:

    winenclave.h

Abstract:

    Master include file for Windows software enclaves.

--*/

#ifndef _WINENCLAVE_
#define _WINENCLAVE_

#if defined(_WINDOWS_)
#error Cannot include both windows.h and winenclave.h
#endif

#include <windows.h>
#include <ntenclv.h>
#include <enclaveium.h>
#include <winenclaveapi.h>

#endif /* _WINENCLAVE_ */
