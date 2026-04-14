//  Copyright (C) 1995-1999 Microsoft Corporation.  All rights reserved.

#ifndef __MTXDM_H__
#define __MTXDM_H__
#include <winapifamily.h>

#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)


// mtxdm.h
// Defines legacy mtxdm startup interface, remove asap, when all dispensers stop doing this.
// New stuff is in ccpublic.idl and dispman.idl

#include "comsvcs.h"

#ifdef __cplusplus
extern "C"{
#endif 

//
// GetDispenserManager
// A Dispenser calls this API to get a reference to DispenserManager.
//
__declspec(dllimport) HRESULT __cdecl GetDispenserManager(IDispenserManager**);


#ifdef __cplusplus
}
#endif



#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion

#endif
