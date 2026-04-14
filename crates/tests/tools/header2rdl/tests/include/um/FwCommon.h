//***************************************************************************
//
//  Copyright (c) Microsoft Corporation.  All rights reserved.
//
//  FWcommon.h
//
//  Purpose: Definition of EventProvider class
//
//***************************************************************************

#if _MSC_VER > 1000
#pragma once
#endif

#define FRAMEWORK_ALLOW_DEPRECATED 0

// ALWAYS INCLUDE THIS FILE FIRST!

#ifndef _FW_COMMON_H_
#define _FW_COMMON_H_

#include <winapifamily.h>

#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)

// move warnings to level four
#pragma warning(4 : 4275 4800 4786 4251)
//	C4275: non DLL-interface classkey 'identifier' used as base for DLL-interface classkey 'identifier'
//	C4800: forcing bool variable to one or zero, possible performance loss
//	C4786: 'identifier' : identifier was truncated to 'number' characters in the debug information
//	C4251: 'identifier' : class 'type' needs to have dll-interface to export class 'type2'


#include "windows.h"

// This function gets macro'ed to GetObjectW and GetObjectA based on the
// contents of UNICODE.  However, we USE a function called GetObject.  For
// single binary support, we can't have this being macro'ed.  If the actual
// win32 api GetObject is needed, call the GetObjectW or GetObjectA directly.
#undef GetObject
#include <wbemidl.h>
#include <wmiutils.h>

// If USE_POLARITY not defined, some Framedyn.dll symbols may not resolve correctly
// It's a declspec(dllimport/dllexport) issue.
#ifndef USE_POLARITY

// For most users, this is the correct setting for POLARITY.
#define USE_POLARITY

#endif

// Required since TCHAR.H defines _ttoi64 -> atoi64, which requires oldnames.lib, which
// we don't use.
#define atoi64 _atoi64

#include <Polarity.h>


#ifndef EXPORT_LOCKIT

#undef _CRTIMP
#define _CRTIMP
#include <yvals.h>
#undef _CRTIMP

#else

#undef _CRTIMP
#define _CRTIMP POLARITY
#include <yvals.h>
#undef _CRTIMP

#endif

#include <stllock.h>
extern POLARITY CCritSec g_cs;

#include "utillib.h"

class CFramework_Exception
{
private:

	CHString m_sErrorText;
    HRESULT m_hr;

public:

	CFramework_Exception ( LPCWSTR lpwszErrorText, HRESULT hr) : m_hr ( hr ) , m_sErrorText ( lpwszErrorText) {}
	CFramework_Exception ( LPCWSTR lpwszErrorText) : m_hr ( WBEM_S_NO_ERROR ) , m_sErrorText ( lpwszErrorText) {}
	~CFramework_Exception () {}

	HRESULT GetHRESULT() { return m_hr ; }
	CHString &GetErrorText() { return m_sErrorText; }
} ;

#include "ThrdBase.h"
#include "RefPtrCo.h"
#include "MethodCo.h"
#include "FRQuery.h"
#include "wbemglue.h"
#include "instance.h"

#ifndef _DBG_ASSERT
  #ifdef DBG
    #define _DBG_ASSERT(X) { if (!(X)) { DebugBreak(); } }
  #else
    #define _DBG_ASSERT(X)
  #endif
#endif

// MACRO for tracing the safe string return failure  -- currently empty
#ifndef DoTraceHRFailureEmpty
#define DoTraceHRFailureEmpty(hr1, szTraceInfo, hr2)    ;
#endif



#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion

#endif
