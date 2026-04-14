

/* this ALWAYS GENERATED file contains the definitions for the interfaces */


 /* File created by MIDL compiler version 8.01.0628 */
/* @@MIDL_FILE_HEADING(  ) */



/* verify that the <rpcndr.h> version is high enough to compile this file*/
#ifndef __REQUIRED_RPCNDR_H_VERSION__
#define __REQUIRED_RPCNDR_H_VERSION__ 501
#endif

/* verify that the <rpcsal.h> version is high enough to compile this file*/
#ifndef __REQUIRED_RPCSAL_H_VERSION__
#define __REQUIRED_RPCSAL_H_VERSION__ 100
#endif

#include "rpc.h"
#include "rpcndr.h"

#ifndef __RPCNDR_H_VERSION__
#error this stub requires an updated version of <rpcndr.h>
#endif /* __RPCNDR_H_VERSION__ */


#ifndef __hstring_h__
#define __hstring_h__

#if defined(_MSC_VER) && (_MSC_VER >= 1020)
#pragma once
#endif

#ifndef DECLSPEC_XFGVIRT
#if defined(_CONTROL_FLOW_GUARD_XFG)
#define DECLSPEC_XFGVIRT(base, func) __declspec(xfg_virtual(base, func))
#else
#define DECLSPEC_XFGVIRT(base, func)
#endif
#endif

/* Forward Declarations */ 

/* header files for imported files */
#include "wtypes.h"

#ifdef __cplusplus
extern "C"{
#endif 


/* interface __MIDL_itf_hstring_0000_0000 */
/* [local] */ 

//+-------------------------------------------------------------------------
//
//  Microsoft Windows
//  Copyright (c) Microsoft Corporation. All rights reserved.
//
//--------------------------------------------------------------------------
#if ( _MSC_VER >= 1020 )
#pragma once
#endif
// Declare the HSTRING handle as wire_marshal for midl only
#if 0
typedef struct HSTRING__
    {
    int unused;
    } 	HSTRING__;

typedef /* [unique][wire_marshal] */  __RPC_unique_pointer HSTRING__ *HSTRING;

#endif

// Declaring a handle dummy struct for HSTRING the same way DECLARE_HANDLE does.
typedef struct HSTRING__{
    int unused;
} HSTRING__;

// Declare the HSTRING handle for C/C++
typedef __RPC_unique_pointer HSTRING__* HSTRING;

// Declare the HSTRING_HEADER
typedef struct HSTRING_HEADER
{
    union{
        PVOID Reserved1;
#if defined(_WIN64)
        char Reserved2[24];
#else
        char Reserved2[20];
#endif
    } Reserved;
} HSTRING_HEADER;

// Declare the HSTRING_BUFFER for the HSTRING's two-phase construction functions.
// 
// This route eliminates the PCWSTR string copy that happens when passing it to
// the traditional WindowsCreateString().  The caller preallocates a string buffer,
// sets the wide character string values in that buffer, and finally promotes the
// buffer to an HSTRING.  If a buffer is never promoted, it can still be deleted.
DECLARE_HANDLE(HSTRING_BUFFER);



extern RPC_IF_HANDLE __MIDL_itf_hstring_0000_0000_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_hstring_0000_0000_v0_0_s_ifspec;

/* Additional Prototypes for ALL interfaces */

/* end of Additional Prototypes */

#ifdef __cplusplus
}
#endif

#endif


