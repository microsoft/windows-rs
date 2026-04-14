//
//    Copyright (C) Microsoft.  All rights reserved.
//
/*
 * WAB.H
 *
 * Top level public header for WAB API functions.
 *
 */

#include <winapifamily.h>

#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)

#include <wabdefs.h>
#include <wabcode.h>
#include <wabtags.h>
#include <wabutil.h>
#include <wabiab.h>
#include <wabapi.h>
#include <wabmem.h>
#include <wabnot.h>

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion

