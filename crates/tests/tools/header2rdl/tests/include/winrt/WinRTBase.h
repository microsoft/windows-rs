/* Header file automatically generated from winrtbase.idl */
/*
 * File built with Microsoft(R) MIDLRT Compiler Engine Version 10.00.0231 
 */

#pragma warning( disable: 4049 )  /* more than 64k source lines */

/* verify that the <rpcndr.h> version is high enough to compile this file*/
#ifndef __REQUIRED_RPCNDR_H_VERSION__
#define __REQUIRED_RPCNDR_H_VERSION__ 500
#endif

/* verify that the <rpcsal.h> version is high enough to compile this file*/
#ifndef __REQUIRED_RPCSAL_H_VERSION__
#define __REQUIRED_RPCSAL_H_VERSION__ 100
#endif

#include <rpc.h>
#include <rpcndr.h>

#ifndef __RPCNDR_H_VERSION__
#error this stub requires an updated version of <rpcndr.h>
#endif /* __RPCNDR_H_VERSION__ */

#ifndef COM_NO_WINDOWS_H
#include <windows.h>
#include <ole2.h>
#endif /*COM_NO_WINDOWS_H*/
#ifndef __winrtbase_h__
#define __winrtbase_h__
#ifndef __winrtbase_p_h__
#define __winrtbase_p_h__


#pragma once

// Ensure that the setting of the /ns_prefix command line switch is consistent for all headers.
// If you get an error from the compiler indicating "warning C4005: 'CHECK_NS_PREFIX_STATE': macro redefinition", this
// indicates that you have included two different headers with different settings for the /ns_prefix MIDL command line switch
#if !defined(DISABLE_NS_PREFIX_CHECKS)
#if defined(MIDL_NS_PREFIX)
#define CHECK_NS_PREFIX_STATE "always"
#else
#define CHECK_NS_PREFIX_STATE "never"
#endif // MIDL_NS_PREFIX
#endif // !defined(DISABLE_NS_PREFIX_CHECKS)


#pragma push_macro("ABI_CONCAT")
#pragma push_macro("ABI_PARAMETER")
#pragma push_macro("ABI_NAMESPACE_BEGIN")
#pragma push_macro("ABI_NAMESPACE_END")
#pragma push_macro("C_IID")
#undef ABI_CONCAT
#undef ABI_PARAMETER
#undef ABI_NAMESPACE_BEGIN
#undef ABI_NAMESPACE_END
#undef C_IID
#define ABI_CONCAT(x,y)  x##y

// /ns_prefix optional state
#if defined(MIDL_NS_PREFIX)
#if defined(__cplusplus) && !defined(CINTERFACE)
#define ABI_PARAMETER(x) ABI::x
#define ABI_NAMESPACE_BEGIN namespace ABI {
#define ABI_NAMESPACE_END }
#else // !defined(__cplusplus) || defined(CINTERFACE)
#define C_ABI_PARAMETER(x) ABI_CONCAT(__x_ABI_C, x)
#endif // !defined(__cplusplus)
#define C_IID(x) ABI_CONCAT(IID___x_ABI_C, x)
#else
#if defined(__cplusplus) && !defined(CINTERFACE)
#define ABI_PARAMETER(x) x
#define ABI_NAMESPACE_BEGIN 
#define ABI_NAMESPACE_END 
#else // !defined(__cplusplus) || defined(CINTERFACE)
#define C_ABI_PARAMETER(x) ABI_CONCAT(__x_, x)
#endif // !defined(__cplusplus)
#define C_IID(x) ABI_CONCAT(IID___x_, x)
#endif // defined(MIDL_NS_PREFIX)

#pragma push_macro("MIDL_CONST_ID")
#undef MIDL_CONST_ID
#define MIDL_CONST_ID const __declspec(selectany)


// Header files for imported files
#include "midlbase.h"

#if defined(__cplusplus) && !defined(CINTERFACE)
/* Forward Declarations */



















#else // !defined(__cplusplus)
/* Forward Declarations */


















#endif // defined(__cplusplus)
#pragma pop_macro("MIDL_CONST_ID")
#pragma pop_macro("C_IID")
#pragma pop_macro("ABI_CONCAT")
#pragma pop_macro("ABI_PARAMETER")
#pragma pop_macro("ABI_NAMESPACE_BEGIN")
#pragma pop_macro("ABI_NAMESPACE_END")


#endif // __winrtbase_p_h__

#endif // __winrtbase_h__
