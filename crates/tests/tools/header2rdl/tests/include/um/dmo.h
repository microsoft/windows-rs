//------------------------------------------------------------------------------
// File: DMO.h
//
// Desc: Headers needed by almost all DMOs.
//
// Copyright (c) 1999 - 2001, Microsoft Corporation.  All rights reserved.
//------------------------------------------------------------------------------


#ifndef __DMO_H__
#define __DMO_H__
#include <winapifamily.h>

#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)



#include "mediaerr.h"

// When using ATL we get collisions on Lock, so in this case rename
// IMediaObject::Lock to IMediaObject::DMOLock

#ifdef FIX_LOCK_NAME
#define Lock DMOLock
#endif
#include "mediaobj.h"
#ifdef FIX_LOCK_NAME
#undef Lock
#endif
#include "dmoreg.h"
#include "dmort.h"


#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion

#endif //__DMO_H__
