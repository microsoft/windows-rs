

/* this ALWAYS GENERATED file contains the definitions for the interfaces */


 /* File created by MIDL compiler version 8.01.0628 */
/* @@MIDL_FILE_HEADING(  ) */



/* verify that the <rpcndr.h> version is high enough to compile this file*/
#ifndef __REQUIRED_RPCNDR_H_VERSION__
#define __REQUIRED_RPCNDR_H_VERSION__ 500
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

#ifndef COM_NO_WINDOWS_H
#include "windows.h"
#include "ole2.h"
#endif /*COM_NO_WINDOWS_H*/

#ifndef __activscp_h__
#define __activscp_h__

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

#ifndef __IActiveScriptSite_FWD_DEFINED__
#define __IActiveScriptSite_FWD_DEFINED__
typedef interface IActiveScriptSite IActiveScriptSite;

#endif 	/* __IActiveScriptSite_FWD_DEFINED__ */


#ifndef __IActiveScriptError_FWD_DEFINED__
#define __IActiveScriptError_FWD_DEFINED__
typedef interface IActiveScriptError IActiveScriptError;

#endif 	/* __IActiveScriptError_FWD_DEFINED__ */


#ifndef __IActiveScriptError64_FWD_DEFINED__
#define __IActiveScriptError64_FWD_DEFINED__
typedef interface IActiveScriptError64 IActiveScriptError64;

#endif 	/* __IActiveScriptError64_FWD_DEFINED__ */


#ifndef __IActiveScriptSiteWindow_FWD_DEFINED__
#define __IActiveScriptSiteWindow_FWD_DEFINED__
typedef interface IActiveScriptSiteWindow IActiveScriptSiteWindow;

#endif 	/* __IActiveScriptSiteWindow_FWD_DEFINED__ */


#ifndef __IActiveScriptSiteUIControl_FWD_DEFINED__
#define __IActiveScriptSiteUIControl_FWD_DEFINED__
typedef interface IActiveScriptSiteUIControl IActiveScriptSiteUIControl;

#endif 	/* __IActiveScriptSiteUIControl_FWD_DEFINED__ */


#ifndef __IActiveScriptSiteInterruptPoll_FWD_DEFINED__
#define __IActiveScriptSiteInterruptPoll_FWD_DEFINED__
typedef interface IActiveScriptSiteInterruptPoll IActiveScriptSiteInterruptPoll;

#endif 	/* __IActiveScriptSiteInterruptPoll_FWD_DEFINED__ */


#ifndef __IActiveScript_FWD_DEFINED__
#define __IActiveScript_FWD_DEFINED__
typedef interface IActiveScript IActiveScript;

#endif 	/* __IActiveScript_FWD_DEFINED__ */


#ifndef __IActiveScriptParse32_FWD_DEFINED__
#define __IActiveScriptParse32_FWD_DEFINED__
typedef interface IActiveScriptParse32 IActiveScriptParse32;

#endif 	/* __IActiveScriptParse32_FWD_DEFINED__ */


#ifndef __IActiveScriptParse64_FWD_DEFINED__
#define __IActiveScriptParse64_FWD_DEFINED__
typedef interface IActiveScriptParse64 IActiveScriptParse64;

#endif 	/* __IActiveScriptParse64_FWD_DEFINED__ */


#ifndef __IActiveScriptParseProcedureOld32_FWD_DEFINED__
#define __IActiveScriptParseProcedureOld32_FWD_DEFINED__
typedef interface IActiveScriptParseProcedureOld32 IActiveScriptParseProcedureOld32;

#endif 	/* __IActiveScriptParseProcedureOld32_FWD_DEFINED__ */


#ifndef __IActiveScriptParseProcedureOld64_FWD_DEFINED__
#define __IActiveScriptParseProcedureOld64_FWD_DEFINED__
typedef interface IActiveScriptParseProcedureOld64 IActiveScriptParseProcedureOld64;

#endif 	/* __IActiveScriptParseProcedureOld64_FWD_DEFINED__ */


#ifndef __IActiveScriptParseProcedure32_FWD_DEFINED__
#define __IActiveScriptParseProcedure32_FWD_DEFINED__
typedef interface IActiveScriptParseProcedure32 IActiveScriptParseProcedure32;

#endif 	/* __IActiveScriptParseProcedure32_FWD_DEFINED__ */


#ifndef __IActiveScriptParseProcedure64_FWD_DEFINED__
#define __IActiveScriptParseProcedure64_FWD_DEFINED__
typedef interface IActiveScriptParseProcedure64 IActiveScriptParseProcedure64;

#endif 	/* __IActiveScriptParseProcedure64_FWD_DEFINED__ */


#ifndef __IActiveScriptParseProcedure2_32_FWD_DEFINED__
#define __IActiveScriptParseProcedure2_32_FWD_DEFINED__
typedef interface IActiveScriptParseProcedure2_32 IActiveScriptParseProcedure2_32;

#endif 	/* __IActiveScriptParseProcedure2_32_FWD_DEFINED__ */


#ifndef __IActiveScriptParseProcedure2_64_FWD_DEFINED__
#define __IActiveScriptParseProcedure2_64_FWD_DEFINED__
typedef interface IActiveScriptParseProcedure2_64 IActiveScriptParseProcedure2_64;

#endif 	/* __IActiveScriptParseProcedure2_64_FWD_DEFINED__ */


#ifndef __IActiveScriptEncode_FWD_DEFINED__
#define __IActiveScriptEncode_FWD_DEFINED__
typedef interface IActiveScriptEncode IActiveScriptEncode;

#endif 	/* __IActiveScriptEncode_FWD_DEFINED__ */


#ifndef __IActiveScriptHostEncode_FWD_DEFINED__
#define __IActiveScriptHostEncode_FWD_DEFINED__
typedef interface IActiveScriptHostEncode IActiveScriptHostEncode;

#endif 	/* __IActiveScriptHostEncode_FWD_DEFINED__ */


#ifndef __IBindEventHandler_FWD_DEFINED__
#define __IBindEventHandler_FWD_DEFINED__
typedef interface IBindEventHandler IBindEventHandler;

#endif 	/* __IBindEventHandler_FWD_DEFINED__ */


#ifndef __IActiveScriptStats_FWD_DEFINED__
#define __IActiveScriptStats_FWD_DEFINED__
typedef interface IActiveScriptStats IActiveScriptStats;

#endif 	/* __IActiveScriptStats_FWD_DEFINED__ */


#ifndef __IActiveScriptProperty_FWD_DEFINED__
#define __IActiveScriptProperty_FWD_DEFINED__
typedef interface IActiveScriptProperty IActiveScriptProperty;

#endif 	/* __IActiveScriptProperty_FWD_DEFINED__ */


#ifndef __ITridentEventSink_FWD_DEFINED__
#define __ITridentEventSink_FWD_DEFINED__
typedef interface ITridentEventSink ITridentEventSink;

#endif 	/* __ITridentEventSink_FWD_DEFINED__ */


#ifndef __IActiveScriptGarbageCollector_FWD_DEFINED__
#define __IActiveScriptGarbageCollector_FWD_DEFINED__
typedef interface IActiveScriptGarbageCollector IActiveScriptGarbageCollector;

#endif 	/* __IActiveScriptGarbageCollector_FWD_DEFINED__ */


#ifndef __IActiveScriptSIPInfo_FWD_DEFINED__
#define __IActiveScriptSIPInfo_FWD_DEFINED__
typedef interface IActiveScriptSIPInfo IActiveScriptSIPInfo;

#endif 	/* __IActiveScriptSIPInfo_FWD_DEFINED__ */


#ifndef __IActiveScriptSiteTraceInfo_FWD_DEFINED__
#define __IActiveScriptSiteTraceInfo_FWD_DEFINED__
typedef interface IActiveScriptSiteTraceInfo IActiveScriptSiteTraceInfo;

#endif 	/* __IActiveScriptSiteTraceInfo_FWD_DEFINED__ */


#ifndef __IActiveScriptTraceInfo_FWD_DEFINED__
#define __IActiveScriptTraceInfo_FWD_DEFINED__
typedef interface IActiveScriptTraceInfo IActiveScriptTraceInfo;

#endif 	/* __IActiveScriptTraceInfo_FWD_DEFINED__ */


#ifndef __IActiveScriptStringCompare_FWD_DEFINED__
#define __IActiveScriptStringCompare_FWD_DEFINED__
typedef interface IActiveScriptStringCompare IActiveScriptStringCompare;

#endif 	/* __IActiveScriptStringCompare_FWD_DEFINED__ */


/* header files for imported files */
#include "ocidl.h"

#ifdef __cplusplus
extern "C"{
#endif 


/* interface __MIDL_itf_activscp_0000_0000 */
/* [local] */ 

//=--------------------------------------------------------------------------=
// ActivScp.h
//=--------------------------------------------------------------------------=
// (C) Copyright Microsoft Corporation.  All Rights Reserved.
//
// THIS CODE AND INFORMATION IS PROVIDED "AS IS" WITHOUT WARRANTY OF
// ANY KIND, EITHER EXPRESSED OR IMPLIED, INCLUDING BUT NOT LIMITED TO
// THE IMPLIED WARRANTIES OF MERCHANTABILITY AND/OR FITNESS FOR A
// PARTICULAR PURPOSE.
//=--------------------------------------------------------------------------=
//
#pragma comment(lib,"uuid.lib")
//
// Declarations for ActiveX Scripting host applications and script engines.
//

#ifndef __ActivScp_h
#define __ActivScp_h

#include <winapifamily.h>

#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)
/* GUIDs
 ********/

#ifndef _NO_SCRIPT_GUIDS
// {F0B7A1A1-9847-11cf-8F20-00805F2CD064}
DEFINE_GUID(CATID_ActiveScript, 0xf0b7a1a1, 0x9847, 0x11cf, 0x8f, 0x20, 0x00, 0x80, 0x5f, 0x2c, 0xd0, 0x64);

// {F0B7A1A2-9847-11cf-8F20-00805F2CD064}
DEFINE_GUID(CATID_ActiveScriptParse, 0xf0b7a1a2, 0x9847, 0x11cf, 0x8f, 0x20, 0x00, 0x80, 0x5f, 0x2c, 0xd0, 0x64);

// {F0B7A1A3-9847-11cf-8F20-00805F2CD064}
DEFINE_GUID(CATID_ActiveScriptEncode, 0xf0b7a1a3, 0x9847, 0x11cf, 0x8f, 0x20, 0x00, 0x80, 0x5f, 0x2c, 0xd0, 0x64);

// {BB1A2AE1-A4F9-11cf-8F20-00805F2CD064}
DEFINE_GUID(IID_IActiveScript, 0xbb1a2ae1, 0xa4f9, 0x11cf, 0x8f, 0x20, 0x00, 0x80, 0x5f, 0x2c, 0xd0, 0x64);

// {BB1A2AE2-A4F9-11cf-8F20-00805F2CD064}
DEFINE_GUID(IID_IActiveScriptParse32, 0xbb1a2ae2, 0xa4f9, 0x11cf, 0x8f, 0x20, 0x00, 0x80, 0x5f, 0x2c, 0xd0, 0x64);
// {C7EF7658-E1EE-480E-97EA-D52CB4D76D17}
DEFINE_GUID(IID_IActiveScriptParse64, 0xc7ef7658, 0xe1ee, 0x480e, 0x97, 0xea, 0xd5, 0x2c, 0xb4, 0xd7, 0x6d, 0x17);

// {BB1A2AE3-A4F9-11cf-8F20-00805F2CD064}
DEFINE_GUID(IID_IActiveScriptEncode, 0xbb1a2ae3, 0xa4f9, 0x11cf, 0x8f, 0x20, 0x00, 0x80, 0x5f, 0x2c, 0xd0, 0x64);

// {BEE9B76E-CFE3-11d1-B747-00C04FC2B085}
DEFINE_GUID(IID_IActiveScriptHostEncode, 0xbee9b76e, 0xcfe3, 0x11d1, 0xb7, 0x47, 0x00, 0xc0, 0x4f, 0xc2, 0xb0, 0x85);

// {1CFF0050-6FDD-11d0-9328-00A0C90DCAA9}
DEFINE_GUID(IID_IActiveScriptParseProcedureOld32, 0x1cff0050, 0x6fdd, 0x11d0, 0x93, 0x28, 0x00, 0xa0, 0xc9, 0x0d, 0xca, 0xa9);
// {21F57128-08C9-4638-BA12-22D15D88DC5C}
DEFINE_GUID(IID_IActiveScriptParseProcedureOld64, 0x21f57128, 0x08c9, 0x4638, 0xba, 0x12, 0x22, 0xd1, 0x5d, 0x88, 0xdc, 0x5c);

// {AA5B6A80-B834-11d0-932F-00A0C90DCAA9}
DEFINE_GUID(IID_IActiveScriptParseProcedure32, 0xaa5b6a80, 0xb834, 0x11d0, 0x93, 0x2f, 0x00, 0xa0, 0xc9, 0x0d, 0xca, 0xa9);
// {C64713B6-E029-4CC5-9200-438B72890B6A}
DEFINE_GUID(IID_IActiveScriptParseProcedure64, 0xc64713b6, 0xe029, 0x4cc5, 0x92, 0x00, 0x43, 0x8b, 0x72, 0x89, 0x0b, 0x6a);

// {71EE5B20-FB04-11d1-B3A8-00A0C911E8B2}
DEFINE_GUID(IID_IActiveScriptParseProcedure2_32, 0x71ee5b20, 0xfb04, 0x11d1, 0xb3, 0xa8, 0x00, 0xa0, 0xc9, 0x11, 0xe8, 0xb2);
// {FE7C4271-210C-448D-9F54-76DAB7047B28}
DEFINE_GUID(IID_IActiveScriptParseProcedure2_64, 0xfe7c4271, 0x210c, 0x448d, 0x9f, 0x54, 0x76, 0xda, 0xb7, 0x04, 0x7b, 0x28);

// {DB01A1E3-A42B-11cf-8F20-00805F2CD064}
DEFINE_GUID(IID_IActiveScriptSite, 0xdb01a1e3, 0xa42b, 0x11cf, 0x8f, 0x20, 0x00, 0x80, 0x5f, 0x2c, 0xd0, 0x64);

// {4B7272AE-1955-4bfe-98B0-780621888569}
DEFINE_GUID(IID_IActiveScriptSiteTraceInfo, 0x4b7272ae, 0x1955, 0x4bfe, 0x98, 0xb0, 0x78, 0x6, 0x21, 0x88, 0x85, 0x69);

// {D10F6761-83E9-11cf-8F20-00805F2CD064}
DEFINE_GUID(IID_IActiveScriptSiteWindow, 0xd10f6761, 0x83e9, 0x11cf, 0x8f, 0x20, 0x00, 0x80, 0x5f, 0x2c, 0xd0, 0x64);

// {539698A0-CDCA-11CF-A5EB-00AA0047A063}
DEFINE_GUID(IID_IActiveScriptSiteInterruptPoll, 0x539698a0, 0xcdca, 0x11cf, 0xa5, 0xeb, 0x00, 0xaa, 0x00, 0x47, 0xa0, 0x63);

// {AEDAE97E-D7EE-4796-B960-7F092AE844AB}
DEFINE_GUID(IID_IActiveScriptSiteUIControl, 0xaedae97e, 0xd7ee, 0x4796, 0xb9, 0x60, 0x7f, 0x9, 0x2a, 0xe8, 0x44, 0xab);

// {EAE1BA61-A4ED-11cf-8F20-00805F2CD064}
DEFINE_GUID(IID_IActiveScriptError,   0xeae1ba61, 0xa4ed, 0x11cf, 0x8f, 0x20, 0x00, 0x80, 0x5f, 0x2c, 0xd0, 0x64);
// {B21FB2A1-5B8F-4963-8C21-21450F84ED7F}
DEFINE_GUID(IID_IActiveScriptError64, 0xb21fb2a1, 0x5b8f, 0x4963, 0x8c, 0x21, 0x21, 0x45, 0x0f, 0x84, 0xed, 0x7f);

// {63CDBCB0-C1B1-11d0-9336-00A0C90DCAA9}
DEFINE_GUID(IID_IBindEventHandler, 0x63cdbcb0, 0xc1b1, 0x11d0, 0x93, 0x36, 0x00, 0xa0, 0xc9, 0x0d, 0xca, 0xa9);

// {B8DA6310-E19B-11d0-933C-00A0C90DCAA9}
DEFINE_GUID(IID_IActiveScriptStats, 0xb8da6310, 0xe19b, 0x11d0, 0x93, 0x3c, 0x00, 0xa0, 0xc9, 0x0d, 0xca, 0xa9);

// {4954E0D0-FBC7-11D1-8410-006008C3FBFC}
DEFINE_GUID(IID_IActiveScriptProperty, 0x4954E0D0, 0xFBC7, 0x11D1, 0x84, 0x10, 0x00, 0x60, 0x08, 0xC3, 0xFB, 0xFC);

// {1DC9CA50-06EF-11d2-8415-006008C3FBFC}
DEFINE_GUID(IID_ITridentEventSink, 0x1dc9ca50, 0x6ef, 0x11d2, 0x84, 0x15, 0x00, 0x60, 0x08, 0xc3, 0xfb, 0xfc);

// {6AA2C4A0-2B53-11d4-A2A0-00104BD35090}
DEFINE_GUID(IID_IActiveScriptGarbageCollector, 0x6aa2c4a0, 0x2b53, 0x11d4, 0xa2, 0xa0, 0x00, 0x10, 0x4b, 0xd3, 0x50, 0x90);

// {764651D0-38DE-11d4-A2A3-00104BD35090}
DEFINE_GUID(IID_IActiveScriptSIPInfo, 0x764651d0, 0x38de, 0x11d4, 0xa2, 0xa3, 0x00, 0x10, 0x4b, 0xd3, 0x50, 0x90);

// {C35456E7-BEBF-4a1b-86A9-24D56BE8B369}
DEFINE_GUID(IID_IActiveScriptTraceInfo, 0xC35456E7, 0xBEBF, 0x4a1b, 0x86, 0xA9, 0x24, 0xD5, 0x6B, 0xE8, 0xB3, 0x69);

// {1629F04E-2799-4db5-8FE5-ACE10F17EBAB}
DEFINE_GUID(OID_VBSSIP, 0x1629f04e, 0x2799, 0x4db5, 0x8f, 0xe5, 0xac, 0xe1, 0x0f, 0x17, 0xeb, 0xab);

// {06C9E010-38CE-11d4-A2A3-00104BD35090}
DEFINE_GUID(OID_JSSIP,  0x6c9e010, 0x38ce, 0x11d4, 0xa2, 0xa3, 0x00, 0x10, 0x4b, 0xd3, 0x50, 0x90);

// {1A610570-38CE-11d4-A2A3-00104BD35090}
DEFINE_GUID(OID_WSFSIP, 0x1a610570, 0x38ce, 0x11d4, 0xa2, 0xa3, 0x00, 0x10, 0x4b, 0xd3, 0x50, 0x90);

// {58562769-ED52-42f7-8403-4963514E1F11}
DEFINE_GUID(IID_IActiveScriptStringCompare, 0x58562769, 0xED52, 0x42f7, 0x84, 0x03, 0x49, 0x63, 0x51, 0x4E, 0x1F, 0x11);

#endif // _NO_SCRIPT_GUIDS

// Constants used by ActiveX Scripting:
//

/* IActiveScript::AddNamedItem() input flags */

#define SCRIPTITEM_ISVISIBLE            0x00000002
#define SCRIPTITEM_ISSOURCE             0x00000004
#define SCRIPTITEM_GLOBALMEMBERS        0x00000008
#define SCRIPTITEM_ISPERSISTENT         0x00000040
#define SCRIPTITEM_CODEONLY             0x00000200
#define SCRIPTITEM_NOCODE               0x00000400

#define SCRIPTITEM_ALL_FLAGS            (SCRIPTITEM_ISSOURCE | \
                                         SCRIPTITEM_ISVISIBLE | \
                                         SCRIPTITEM_ISPERSISTENT | \
                                         SCRIPTITEM_GLOBALMEMBERS | \
                                         SCRIPTITEM_NOCODE | \
                                         SCRIPTITEM_CODEONLY)

/* IActiveScript::AddTypeLib() input flags */

#define SCRIPTTYPELIB_ISCONTROL         0x00000010
#define SCRIPTTYPELIB_ISPERSISTENT      0x00000040
#define SCRIPTTYPELIB_ALL_FLAGS         (SCRIPTTYPELIB_ISCONTROL | SCRIPTTYPELIB_ISPERSISTENT)

/* IActiveScriptParse::AddScriptlet() and IActiveScriptParse::ParseScriptText() input flags */

#define SCRIPTTEXT_DELAYEXECUTION       0x00000001
#define SCRIPTTEXT_ISVISIBLE            0x00000002
#define SCRIPTTEXT_ISEXPRESSION         0x00000020
#define SCRIPTTEXT_ISPERSISTENT         0x00000040
#define SCRIPTTEXT_HOSTMANAGESSOURCE    0x00000080
#define SCRIPTTEXT_ISXDOMAIN            0x00000100
#define SCRIPTTEXT_ISNONUSERCODE        0x00000200
#define SCRIPTTEXT_ALL_FLAGS            (SCRIPTTEXT_DELAYEXECUTION | \
                                         SCRIPTTEXT_ISVISIBLE | \
                                         SCRIPTTEXT_ISEXPRESSION | \
                                         SCRIPTTEXT_ISPERSISTENT | \
                                         SCRIPTTEXT_HOSTMANAGESSOURCE | \
                                         SCRIPTTEXT_ISXDOMAIN | \
                                         SCRIPTTEXT_ISNONUSERCODE)

/* IActiveScriptParseProcedure::ParseProcedureText() input flags */

#define SCRIPTPROC_ISEXPRESSION         0x00000020
#define SCRIPTPROC_HOSTMANAGESSOURCE    0x00000080
#define SCRIPTPROC_IMPLICIT_THIS        0x00000100
#define SCRIPTPROC_IMPLICIT_PARENTS     0x00000200
#define SCRIPTPROC_ISXDOMAIN            0x00000400
#define SCRIPTPROC_ALL_FLAGS            (SCRIPTPROC_HOSTMANAGESSOURCE | \
                                         SCRIPTPROC_ISEXPRESSION | \
                                         SCRIPTPROC_IMPLICIT_THIS | \
                                         SCRIPTPROC_IMPLICIT_PARENTS | \
                                         SCRIPTPROC_ISXDOMAIN)

/* IActiveScriptSite::GetItemInfo() input flags */

#define SCRIPTINFO_IUNKNOWN             0x00000001
#define SCRIPTINFO_ITYPEINFO            0x00000002
#define SCRIPTINFO_ALL_FLAGS            (SCRIPTINFO_IUNKNOWN | \
                                         SCRIPTINFO_ITYPEINFO)

/* IActiveScript::Interrupt() Flags */

#define SCRIPTINTERRUPT_DEBUG           0x00000001
#define SCRIPTINTERRUPT_RAISEEXCEPTION  0x00000002
#define SCRIPTINTERRUPT_ALL_FLAGS       (SCRIPTINTERRUPT_DEBUG | \
                                         SCRIPTINTERRUPT_RAISEEXCEPTION)

/* IActiveScriptStats::GetStat() values */

#define SCRIPTSTAT_STATEMENT_COUNT       1
#define SCRIPTSTAT_INSTRUCTION_COUNT     2
#define SCRIPTSTAT_INTSTRUCTION_TIME     3
#define SCRIPTSTAT_TOTAL_TIME            4

/* IActiveScriptEncode::AddSection() input flags */

#define SCRIPT_ENCODE_SECTION         0x00000001

#define SCRIPT_ENCODE_DEFAULT_LANGUAGE        0x00000001
#define SCRIPT_ENCODE_NO_ASP_LANGUAGE         0x00000002

/* Properties for IActiveScriptProperty */
#define SCRIPTPROP_NAME                     0x00000000
#define SCRIPTPROP_MAJORVERSION             0x00000001
#define SCRIPTPROP_MINORVERSION             0x00000002
#define SCRIPTPROP_BUILDNUMBER              0x00000003

#define SCRIPTPROP_DELAYEDEVENTSINKING      0x00001000
#define SCRIPTPROP_CATCHEXCEPTION           0x00001001
#define SCRIPTPROP_CONVERSIONLCID           0x00001002
#define SCRIPTPROP_HOSTSTACKREQUIRED        0x00001003
#define SCRIPTPROP_SCRIPTSAREFULLYTRUSTED   0x00001004

#define SCRIPTPROP_DEBUGGER                 0x00001100
#define SCRIPTPROP_JITDEBUG                 0x00001101

#define SCRIPTPROP_GCCONTROLSOFTCLOSE       0x00002000

#define SCRIPTPROP_INTEGERMODE              0x00003000
#define SCRIPTPROP_STRINGCOMPAREINSTANCE    0x00003001

#define SCRIPTPROP_INVOKEVERSIONING         0x00004000

// These properties are defined and available, but are not
// officially supported.
#define SCRIPTPROP_HACK_FIBERSUPPORT        0x70000000
#define SCRIPTPROP_HACK_TRIDENTEVENTSINK    0x70000001
#define SCRIPTPROP_ABBREVIATE_GLOBALNAME_RESOLUTION  0x70000002
#define SCRIPTPROP_HOSTKEEPALIVE         0x70000004

// An error has been recorded to be passed between script engine 
// and host. The host needs to pass the error code to caller. 
#define SCRIPT_E_RECORDED   0x86664004L
// Script engine has reported an unhandled exception to the host via 
// IActiveScriptSite::OnScriptError. Host can ignore this error
#define SCRIPT_E_REPORTED   0x80020101L 
// An script error is being propagated to the caller which might be in a different thread
// host should pass the error code to the caller.
#define SCRIPT_E_PROPAGATE  0x80020102L

/* script language version values for SCRIPTPROP_INVOKEVERSIONING property */

typedef 
enum tagSCRIPTLANGUAGEVERSION
    {
        SCRIPTLANGUAGEVERSION_DEFAULT	= 0,
        SCRIPTLANGUAGEVERSION_5_7	= 1,
        SCRIPTLANGUAGEVERSION_5_8	= 2,
        SCRIPTLANGUAGEVERSION_MAX	= 255
    } 	SCRIPTLANGUAGEVERSION;

typedef 
enum tagSCRIPTSTATE
    {
        SCRIPTSTATE_UNINITIALIZED	= 0,
        SCRIPTSTATE_INITIALIZED	= 5,
        SCRIPTSTATE_STARTED	= 1,
        SCRIPTSTATE_CONNECTED	= 2,
        SCRIPTSTATE_DISCONNECTED	= 3,
        SCRIPTSTATE_CLOSED	= 4
    } 	SCRIPTSTATE;

typedef 
enum tagSCRIPTTRACEINFO
    {
        SCRIPTTRACEINFO_SCRIPTSTART	= 0,
        SCRIPTTRACEINFO_SCRIPTEND	= 1,
        SCRIPTTRACEINFO_COMCALLSTART	= 2,
        SCRIPTTRACEINFO_COMCALLEND	= 3,
        SCRIPTTRACEINFO_CREATEOBJSTART	= 4,
        SCRIPTTRACEINFO_CREATEOBJEND	= 5,
        SCRIPTTRACEINFO_GETOBJSTART	= 6,
        SCRIPTTRACEINFO_GETOBJEND	= 7
    } 	SCRIPTTRACEINFO;


/* script thread state values */

typedef 
enum tagSCRIPTTHREADSTATE
    {
        SCRIPTTHREADSTATE_NOTINSCRIPT	= 0,
        SCRIPTTHREADSTATE_RUNNING	= 1
    } 	SCRIPTTHREADSTATE;


/* IActiveScriptCollectGarbage constants */

typedef 
enum tagSCRIPTGCTYPE
    {
        SCRIPTGCTYPE_NORMAL	= 0,
        SCRIPTGCTYPE_EXHAUSTIVE	= 1
    } 	SCRIPTGCTYPE;


/* IActiveScriptSiteUIControl constants*/

typedef 
enum tagSCRIPTUICITEM
    {
        SCRIPTUICITEM_INPUTBOX	= 1,
        SCRIPTUICITEM_MSGBOX	= 2
    } 	SCRIPTUICITEM;


typedef 
enum tagSCRIPTUICHANDLING
    {
        SCRIPTUICHANDLING_ALLOW	= 0,
        SCRIPTUICHANDLING_NOUIERROR	= 1,
        SCRIPTUICHANDLING_NOUIDEFAULT	= 2
    } 	SCRIPTUICHANDLING;


/* Thread IDs */

typedef DWORD SCRIPTTHREADID;


#define SCRIPTTHREADID_CURRENT  ((SCRIPTTHREADID)-1)
#define SCRIPTTHREADID_BASE     ((SCRIPTTHREADID)-2)
#define SCRIPTTHREADID_ALL      ((SCRIPTTHREADID)-3)

/* Structures */

/* Interfaces
 *************/




























extern RPC_IF_HANDLE __MIDL_itf_activscp_0000_0000_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_activscp_0000_0000_v0_0_s_ifspec;

#ifndef __IActiveScriptSite_INTERFACE_DEFINED__
#define __IActiveScriptSite_INTERFACE_DEFINED__

/* interface IActiveScriptSite */
/* [unique][uuid][object] */ 


EXTERN_C const IID IID_IActiveScriptSite;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("DB01A1E3-A42B-11cf-8F20-00805F2CD064")
    IActiveScriptSite : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetLCID( 
            /* [out] */ __RPC__out LCID *plcid) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetItemInfo( 
            /* [in] */ __RPC__in LPCOLESTR pstrName,
            /* [in] */ DWORD dwReturnMask,
            /* [out] */ __RPC__deref_out_opt IUnknown **ppiunkItem,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppti) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetDocVersionString( 
            /* [out] */ __RPC__deref_out_opt BSTR *pbstrVersion) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE OnScriptTerminate( 
            /* [in] */ __RPC__in const VARIANT *pvarResult,
            /* [in] */ __RPC__in const EXCEPINFO *pexcepinfo) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE OnStateChange( 
            /* [in] */ SCRIPTSTATE ssScriptState) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE OnScriptError( 
            /* [in] */ __RPC__in_opt IActiveScriptError *pscripterror) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE OnEnterScript( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE OnLeaveScript( void) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IActiveScriptSiteVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IActiveScriptSite * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IActiveScriptSite * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IActiveScriptSite * This);
        
        DECLSPEC_XFGVIRT(IActiveScriptSite, GetLCID)
        HRESULT ( STDMETHODCALLTYPE *GetLCID )( 
            __RPC__in IActiveScriptSite * This,
            /* [out] */ __RPC__out LCID *plcid);
        
        DECLSPEC_XFGVIRT(IActiveScriptSite, GetItemInfo)
        HRESULT ( STDMETHODCALLTYPE *GetItemInfo )( 
            __RPC__in IActiveScriptSite * This,
            /* [in] */ __RPC__in LPCOLESTR pstrName,
            /* [in] */ DWORD dwReturnMask,
            /* [out] */ __RPC__deref_out_opt IUnknown **ppiunkItem,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppti);
        
        DECLSPEC_XFGVIRT(IActiveScriptSite, GetDocVersionString)
        HRESULT ( STDMETHODCALLTYPE *GetDocVersionString )( 
            __RPC__in IActiveScriptSite * This,
            /* [out] */ __RPC__deref_out_opt BSTR *pbstrVersion);
        
        DECLSPEC_XFGVIRT(IActiveScriptSite, OnScriptTerminate)
        HRESULT ( STDMETHODCALLTYPE *OnScriptTerminate )( 
            __RPC__in IActiveScriptSite * This,
            /* [in] */ __RPC__in const VARIANT *pvarResult,
            /* [in] */ __RPC__in const EXCEPINFO *pexcepinfo);
        
        DECLSPEC_XFGVIRT(IActiveScriptSite, OnStateChange)
        HRESULT ( STDMETHODCALLTYPE *OnStateChange )( 
            __RPC__in IActiveScriptSite * This,
            /* [in] */ SCRIPTSTATE ssScriptState);
        
        DECLSPEC_XFGVIRT(IActiveScriptSite, OnScriptError)
        HRESULT ( STDMETHODCALLTYPE *OnScriptError )( 
            __RPC__in IActiveScriptSite * This,
            /* [in] */ __RPC__in_opt IActiveScriptError *pscripterror);
        
        DECLSPEC_XFGVIRT(IActiveScriptSite, OnEnterScript)
        HRESULT ( STDMETHODCALLTYPE *OnEnterScript )( 
            __RPC__in IActiveScriptSite * This);
        
        DECLSPEC_XFGVIRT(IActiveScriptSite, OnLeaveScript)
        HRESULT ( STDMETHODCALLTYPE *OnLeaveScript )( 
            __RPC__in IActiveScriptSite * This);
        
        END_INTERFACE
    } IActiveScriptSiteVtbl;

    interface IActiveScriptSite
    {
        CONST_VTBL struct IActiveScriptSiteVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IActiveScriptSite_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IActiveScriptSite_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IActiveScriptSite_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IActiveScriptSite_GetLCID(This,plcid)	\
    ( (This)->lpVtbl -> GetLCID(This,plcid) ) 

#define IActiveScriptSite_GetItemInfo(This,pstrName,dwReturnMask,ppiunkItem,ppti)	\
    ( (This)->lpVtbl -> GetItemInfo(This,pstrName,dwReturnMask,ppiunkItem,ppti) ) 

#define IActiveScriptSite_GetDocVersionString(This,pbstrVersion)	\
    ( (This)->lpVtbl -> GetDocVersionString(This,pbstrVersion) ) 

#define IActiveScriptSite_OnScriptTerminate(This,pvarResult,pexcepinfo)	\
    ( (This)->lpVtbl -> OnScriptTerminate(This,pvarResult,pexcepinfo) ) 

#define IActiveScriptSite_OnStateChange(This,ssScriptState)	\
    ( (This)->lpVtbl -> OnStateChange(This,ssScriptState) ) 

#define IActiveScriptSite_OnScriptError(This,pscripterror)	\
    ( (This)->lpVtbl -> OnScriptError(This,pscripterror) ) 

#define IActiveScriptSite_OnEnterScript(This)	\
    ( (This)->lpVtbl -> OnEnterScript(This) ) 

#define IActiveScriptSite_OnLeaveScript(This)	\
    ( (This)->lpVtbl -> OnLeaveScript(This) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IActiveScriptSite_INTERFACE_DEFINED__ */


#ifndef __IActiveScriptError_INTERFACE_DEFINED__
#define __IActiveScriptError_INTERFACE_DEFINED__

/* interface IActiveScriptError */
/* [unique][uuid][object] */ 


EXTERN_C const IID IID_IActiveScriptError;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("EAE1BA61-A4ED-11cf-8F20-00805F2CD064")
    IActiveScriptError : public IUnknown
    {
    public:
        virtual /* [local] */ HRESULT STDMETHODCALLTYPE GetExceptionInfo( 
            /* [out] */ EXCEPINFO *pexcepinfo) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetSourcePosition( 
            /* [out] */ __RPC__out DWORD *pdwSourceContext,
            /* [out] */ __RPC__out ULONG *pulLineNumber,
            /* [out] */ __RPC__out LONG *plCharacterPosition) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetSourceLineText( 
            /* [out] */ __RPC__deref_out_opt BSTR *pbstrSourceLine) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IActiveScriptErrorVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IActiveScriptError * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IActiveScriptError * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IActiveScriptError * This);
        
        DECLSPEC_XFGVIRT(IActiveScriptError, GetExceptionInfo)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *GetExceptionInfo )( 
            IActiveScriptError * This,
            /* [out] */ EXCEPINFO *pexcepinfo);
        
        DECLSPEC_XFGVIRT(IActiveScriptError, GetSourcePosition)
        HRESULT ( STDMETHODCALLTYPE *GetSourcePosition )( 
            __RPC__in IActiveScriptError * This,
            /* [out] */ __RPC__out DWORD *pdwSourceContext,
            /* [out] */ __RPC__out ULONG *pulLineNumber,
            /* [out] */ __RPC__out LONG *plCharacterPosition);
        
        DECLSPEC_XFGVIRT(IActiveScriptError, GetSourceLineText)
        HRESULT ( STDMETHODCALLTYPE *GetSourceLineText )( 
            __RPC__in IActiveScriptError * This,
            /* [out] */ __RPC__deref_out_opt BSTR *pbstrSourceLine);
        
        END_INTERFACE
    } IActiveScriptErrorVtbl;

    interface IActiveScriptError
    {
        CONST_VTBL struct IActiveScriptErrorVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IActiveScriptError_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IActiveScriptError_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IActiveScriptError_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IActiveScriptError_GetExceptionInfo(This,pexcepinfo)	\
    ( (This)->lpVtbl -> GetExceptionInfo(This,pexcepinfo) ) 

#define IActiveScriptError_GetSourcePosition(This,pdwSourceContext,pulLineNumber,plCharacterPosition)	\
    ( (This)->lpVtbl -> GetSourcePosition(This,pdwSourceContext,pulLineNumber,plCharacterPosition) ) 

#define IActiveScriptError_GetSourceLineText(This,pbstrSourceLine)	\
    ( (This)->lpVtbl -> GetSourceLineText(This,pbstrSourceLine) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */



/* [call_as] */ HRESULT STDMETHODCALLTYPE IActiveScriptError_RemoteGetExceptionInfo_Proxy( 
    __RPC__in IActiveScriptError * This,
    /* [out] */ __RPC__out EXCEPINFO *pexcepinfo);


void __RPC_STUB IActiveScriptError_RemoteGetExceptionInfo_Stub(
    IRpcStubBuffer *This,
    IRpcChannelBuffer *_pRpcChannelBuffer,
    PRPC_MESSAGE _pRpcMessage,
    DWORD *_pdwStubPhase);



#endif 	/* __IActiveScriptError_INTERFACE_DEFINED__ */


#ifndef __IActiveScriptError64_INTERFACE_DEFINED__
#define __IActiveScriptError64_INTERFACE_DEFINED__

/* interface IActiveScriptError64 */
/* [unique][uuid][object] */ 


EXTERN_C const IID IID_IActiveScriptError64;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("B21FB2A1-5B8F-4963-8C21-21450F84ED7F")
    IActiveScriptError64 : public IActiveScriptError
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetSourcePosition64( 
            /* [out] */ __RPC__out DWORDLONG *pdwSourceContext,
            /* [out] */ __RPC__out ULONG *pulLineNumber,
            /* [out] */ __RPC__out LONG *plCharacterPosition) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IActiveScriptError64Vtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IActiveScriptError64 * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IActiveScriptError64 * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IActiveScriptError64 * This);
        
        DECLSPEC_XFGVIRT(IActiveScriptError, GetExceptionInfo)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *GetExceptionInfo )( 
            IActiveScriptError64 * This,
            /* [out] */ EXCEPINFO *pexcepinfo);
        
        DECLSPEC_XFGVIRT(IActiveScriptError, GetSourcePosition)
        HRESULT ( STDMETHODCALLTYPE *GetSourcePosition )( 
            __RPC__in IActiveScriptError64 * This,
            /* [out] */ __RPC__out DWORD *pdwSourceContext,
            /* [out] */ __RPC__out ULONG *pulLineNumber,
            /* [out] */ __RPC__out LONG *plCharacterPosition);
        
        DECLSPEC_XFGVIRT(IActiveScriptError, GetSourceLineText)
        HRESULT ( STDMETHODCALLTYPE *GetSourceLineText )( 
            __RPC__in IActiveScriptError64 * This,
            /* [out] */ __RPC__deref_out_opt BSTR *pbstrSourceLine);
        
        DECLSPEC_XFGVIRT(IActiveScriptError64, GetSourcePosition64)
        HRESULT ( STDMETHODCALLTYPE *GetSourcePosition64 )( 
            __RPC__in IActiveScriptError64 * This,
            /* [out] */ __RPC__out DWORDLONG *pdwSourceContext,
            /* [out] */ __RPC__out ULONG *pulLineNumber,
            /* [out] */ __RPC__out LONG *plCharacterPosition);
        
        END_INTERFACE
    } IActiveScriptError64Vtbl;

    interface IActiveScriptError64
    {
        CONST_VTBL struct IActiveScriptError64Vtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IActiveScriptError64_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IActiveScriptError64_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IActiveScriptError64_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IActiveScriptError64_GetExceptionInfo(This,pexcepinfo)	\
    ( (This)->lpVtbl -> GetExceptionInfo(This,pexcepinfo) ) 

#define IActiveScriptError64_GetSourcePosition(This,pdwSourceContext,pulLineNumber,plCharacterPosition)	\
    ( (This)->lpVtbl -> GetSourcePosition(This,pdwSourceContext,pulLineNumber,plCharacterPosition) ) 

#define IActiveScriptError64_GetSourceLineText(This,pbstrSourceLine)	\
    ( (This)->lpVtbl -> GetSourceLineText(This,pbstrSourceLine) ) 


#define IActiveScriptError64_GetSourcePosition64(This,pdwSourceContext,pulLineNumber,plCharacterPosition)	\
    ( (This)->lpVtbl -> GetSourcePosition64(This,pdwSourceContext,pulLineNumber,plCharacterPosition) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IActiveScriptError64_INTERFACE_DEFINED__ */


#ifndef __IActiveScriptSiteWindow_INTERFACE_DEFINED__
#define __IActiveScriptSiteWindow_INTERFACE_DEFINED__

/* interface IActiveScriptSiteWindow */
/* [unique][uuid][object] */ 


EXTERN_C const IID IID_IActiveScriptSiteWindow;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("D10F6761-83E9-11cf-8F20-00805F2CD064")
    IActiveScriptSiteWindow : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetWindow( 
            /* [out] */ __RPC__deref_out_opt HWND *phwnd) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE EnableModeless( 
            /* [in] */ BOOL fEnable) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IActiveScriptSiteWindowVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IActiveScriptSiteWindow * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IActiveScriptSiteWindow * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IActiveScriptSiteWindow * This);
        
        DECLSPEC_XFGVIRT(IActiveScriptSiteWindow, GetWindow)
        HRESULT ( STDMETHODCALLTYPE *GetWindow )( 
            __RPC__in IActiveScriptSiteWindow * This,
            /* [out] */ __RPC__deref_out_opt HWND *phwnd);
        
        DECLSPEC_XFGVIRT(IActiveScriptSiteWindow, EnableModeless)
        HRESULT ( STDMETHODCALLTYPE *EnableModeless )( 
            __RPC__in IActiveScriptSiteWindow * This,
            /* [in] */ BOOL fEnable);
        
        END_INTERFACE
    } IActiveScriptSiteWindowVtbl;

    interface IActiveScriptSiteWindow
    {
        CONST_VTBL struct IActiveScriptSiteWindowVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IActiveScriptSiteWindow_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IActiveScriptSiteWindow_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IActiveScriptSiteWindow_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IActiveScriptSiteWindow_GetWindow(This,phwnd)	\
    ( (This)->lpVtbl -> GetWindow(This,phwnd) ) 

#define IActiveScriptSiteWindow_EnableModeless(This,fEnable)	\
    ( (This)->lpVtbl -> EnableModeless(This,fEnable) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IActiveScriptSiteWindow_INTERFACE_DEFINED__ */


#ifndef __IActiveScriptSiteUIControl_INTERFACE_DEFINED__
#define __IActiveScriptSiteUIControl_INTERFACE_DEFINED__

/* interface IActiveScriptSiteUIControl */
/* [unique][uuid][object] */ 


EXTERN_C const IID IID_IActiveScriptSiteUIControl;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("AEDAE97E-D7EE-4796-B960-7F092AE844AB")
    IActiveScriptSiteUIControl : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetUIBehavior( 
            /* [in] */ SCRIPTUICITEM UicItem,
            /* [out] */ __RPC__out SCRIPTUICHANDLING *pUicHandling) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IActiveScriptSiteUIControlVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IActiveScriptSiteUIControl * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IActiveScriptSiteUIControl * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IActiveScriptSiteUIControl * This);
        
        DECLSPEC_XFGVIRT(IActiveScriptSiteUIControl, GetUIBehavior)
        HRESULT ( STDMETHODCALLTYPE *GetUIBehavior )( 
            __RPC__in IActiveScriptSiteUIControl * This,
            /* [in] */ SCRIPTUICITEM UicItem,
            /* [out] */ __RPC__out SCRIPTUICHANDLING *pUicHandling);
        
        END_INTERFACE
    } IActiveScriptSiteUIControlVtbl;

    interface IActiveScriptSiteUIControl
    {
        CONST_VTBL struct IActiveScriptSiteUIControlVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IActiveScriptSiteUIControl_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IActiveScriptSiteUIControl_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IActiveScriptSiteUIControl_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IActiveScriptSiteUIControl_GetUIBehavior(This,UicItem,pUicHandling)	\
    ( (This)->lpVtbl -> GetUIBehavior(This,UicItem,pUicHandling) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IActiveScriptSiteUIControl_INTERFACE_DEFINED__ */


#ifndef __IActiveScriptSiteInterruptPoll_INTERFACE_DEFINED__
#define __IActiveScriptSiteInterruptPoll_INTERFACE_DEFINED__

/* interface IActiveScriptSiteInterruptPoll */
/* [unique][uuid][object] */ 


EXTERN_C const IID IID_IActiveScriptSiteInterruptPoll;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("539698A0-CDCA-11CF-A5EB-00AA0047A063")
    IActiveScriptSiteInterruptPoll : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE QueryContinue( void) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IActiveScriptSiteInterruptPollVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IActiveScriptSiteInterruptPoll * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IActiveScriptSiteInterruptPoll * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IActiveScriptSiteInterruptPoll * This);
        
        DECLSPEC_XFGVIRT(IActiveScriptSiteInterruptPoll, QueryContinue)
        HRESULT ( STDMETHODCALLTYPE *QueryContinue )( 
            __RPC__in IActiveScriptSiteInterruptPoll * This);
        
        END_INTERFACE
    } IActiveScriptSiteInterruptPollVtbl;

    interface IActiveScriptSiteInterruptPoll
    {
        CONST_VTBL struct IActiveScriptSiteInterruptPollVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IActiveScriptSiteInterruptPoll_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IActiveScriptSiteInterruptPoll_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IActiveScriptSiteInterruptPoll_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IActiveScriptSiteInterruptPoll_QueryContinue(This)	\
    ( (This)->lpVtbl -> QueryContinue(This) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IActiveScriptSiteInterruptPoll_INTERFACE_DEFINED__ */


#ifndef __IActiveScript_INTERFACE_DEFINED__
#define __IActiveScript_INTERFACE_DEFINED__

/* interface IActiveScript */
/* [unique][uuid][object] */ 


EXTERN_C const IID IID_IActiveScript;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("BB1A2AE1-A4F9-11cf-8F20-00805F2CD064")
    IActiveScript : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE SetScriptSite( 
            /* [in] */ __RPC__in_opt IActiveScriptSite *pass) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetScriptSite( 
            /* [in] */ __RPC__in REFIID riid,
            /* [iid_is][out] */ __RPC__deref_out_opt void **ppvObject) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetScriptState( 
            /* [in] */ SCRIPTSTATE ss) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetScriptState( 
            /* [out] */ __RPC__out SCRIPTSTATE *pssState) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Close( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE AddNamedItem( 
            /* [in] */ __RPC__in LPCOLESTR pstrName,
            /* [in] */ DWORD dwFlags) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE AddTypeLib( 
            /* [in] */ __RPC__in REFGUID rguidTypeLib,
            /* [in] */ DWORD dwMajor,
            /* [in] */ DWORD dwMinor,
            /* [in] */ DWORD dwFlags) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetScriptDispatch( 
            /* [in] */ __RPC__in LPCOLESTR pstrItemName,
            /* [out] */ __RPC__deref_out_opt IDispatch **ppdisp) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetCurrentScriptThreadID( 
            /* [out] */ __RPC__out SCRIPTTHREADID *pstidThread) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetScriptThreadID( 
            /* [in] */ DWORD dwWin32ThreadId,
            /* [out] */ __RPC__out SCRIPTTHREADID *pstidThread) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetScriptThreadState( 
            /* [in] */ SCRIPTTHREADID stidThread,
            /* [out] */ __RPC__out SCRIPTTHREADSTATE *pstsState) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE InterruptScriptThread( 
            /* [in] */ SCRIPTTHREADID stidThread,
            /* [in] */ __RPC__in const EXCEPINFO *pexcepinfo,
            /* [in] */ DWORD dwFlags) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Clone( 
            /* [out] */ __RPC__deref_out_opt IActiveScript **ppscript) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IActiveScriptVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IActiveScript * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IActiveScript * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IActiveScript * This);
        
        DECLSPEC_XFGVIRT(IActiveScript, SetScriptSite)
        HRESULT ( STDMETHODCALLTYPE *SetScriptSite )( 
            __RPC__in IActiveScript * This,
            /* [in] */ __RPC__in_opt IActiveScriptSite *pass);
        
        DECLSPEC_XFGVIRT(IActiveScript, GetScriptSite)
        HRESULT ( STDMETHODCALLTYPE *GetScriptSite )( 
            __RPC__in IActiveScript * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [iid_is][out] */ __RPC__deref_out_opt void **ppvObject);
        
        DECLSPEC_XFGVIRT(IActiveScript, SetScriptState)
        HRESULT ( STDMETHODCALLTYPE *SetScriptState )( 
            __RPC__in IActiveScript * This,
            /* [in] */ SCRIPTSTATE ss);
        
        DECLSPEC_XFGVIRT(IActiveScript, GetScriptState)
        HRESULT ( STDMETHODCALLTYPE *GetScriptState )( 
            __RPC__in IActiveScript * This,
            /* [out] */ __RPC__out SCRIPTSTATE *pssState);
        
        DECLSPEC_XFGVIRT(IActiveScript, Close)
        HRESULT ( STDMETHODCALLTYPE *Close )( 
            __RPC__in IActiveScript * This);
        
        DECLSPEC_XFGVIRT(IActiveScript, AddNamedItem)
        HRESULT ( STDMETHODCALLTYPE *AddNamedItem )( 
            __RPC__in IActiveScript * This,
            /* [in] */ __RPC__in LPCOLESTR pstrName,
            /* [in] */ DWORD dwFlags);
        
        DECLSPEC_XFGVIRT(IActiveScript, AddTypeLib)
        HRESULT ( STDMETHODCALLTYPE *AddTypeLib )( 
            __RPC__in IActiveScript * This,
            /* [in] */ __RPC__in REFGUID rguidTypeLib,
            /* [in] */ DWORD dwMajor,
            /* [in] */ DWORD dwMinor,
            /* [in] */ DWORD dwFlags);
        
        DECLSPEC_XFGVIRT(IActiveScript, GetScriptDispatch)
        HRESULT ( STDMETHODCALLTYPE *GetScriptDispatch )( 
            __RPC__in IActiveScript * This,
            /* [in] */ __RPC__in LPCOLESTR pstrItemName,
            /* [out] */ __RPC__deref_out_opt IDispatch **ppdisp);
        
        DECLSPEC_XFGVIRT(IActiveScript, GetCurrentScriptThreadID)
        HRESULT ( STDMETHODCALLTYPE *GetCurrentScriptThreadID )( 
            __RPC__in IActiveScript * This,
            /* [out] */ __RPC__out SCRIPTTHREADID *pstidThread);
        
        DECLSPEC_XFGVIRT(IActiveScript, GetScriptThreadID)
        HRESULT ( STDMETHODCALLTYPE *GetScriptThreadID )( 
            __RPC__in IActiveScript * This,
            /* [in] */ DWORD dwWin32ThreadId,
            /* [out] */ __RPC__out SCRIPTTHREADID *pstidThread);
        
        DECLSPEC_XFGVIRT(IActiveScript, GetScriptThreadState)
        HRESULT ( STDMETHODCALLTYPE *GetScriptThreadState )( 
            __RPC__in IActiveScript * This,
            /* [in] */ SCRIPTTHREADID stidThread,
            /* [out] */ __RPC__out SCRIPTTHREADSTATE *pstsState);
        
        DECLSPEC_XFGVIRT(IActiveScript, InterruptScriptThread)
        HRESULT ( STDMETHODCALLTYPE *InterruptScriptThread )( 
            __RPC__in IActiveScript * This,
            /* [in] */ SCRIPTTHREADID stidThread,
            /* [in] */ __RPC__in const EXCEPINFO *pexcepinfo,
            /* [in] */ DWORD dwFlags);
        
        DECLSPEC_XFGVIRT(IActiveScript, Clone)
        HRESULT ( STDMETHODCALLTYPE *Clone )( 
            __RPC__in IActiveScript * This,
            /* [out] */ __RPC__deref_out_opt IActiveScript **ppscript);
        
        END_INTERFACE
    } IActiveScriptVtbl;

    interface IActiveScript
    {
        CONST_VTBL struct IActiveScriptVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IActiveScript_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IActiveScript_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IActiveScript_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IActiveScript_SetScriptSite(This,pass)	\
    ( (This)->lpVtbl -> SetScriptSite(This,pass) ) 

#define IActiveScript_GetScriptSite(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> GetScriptSite(This,riid,ppvObject) ) 

#define IActiveScript_SetScriptState(This,ss)	\
    ( (This)->lpVtbl -> SetScriptState(This,ss) ) 

#define IActiveScript_GetScriptState(This,pssState)	\
    ( (This)->lpVtbl -> GetScriptState(This,pssState) ) 

#define IActiveScript_Close(This)	\
    ( (This)->lpVtbl -> Close(This) ) 

#define IActiveScript_AddNamedItem(This,pstrName,dwFlags)	\
    ( (This)->lpVtbl -> AddNamedItem(This,pstrName,dwFlags) ) 

#define IActiveScript_AddTypeLib(This,rguidTypeLib,dwMajor,dwMinor,dwFlags)	\
    ( (This)->lpVtbl -> AddTypeLib(This,rguidTypeLib,dwMajor,dwMinor,dwFlags) ) 

#define IActiveScript_GetScriptDispatch(This,pstrItemName,ppdisp)	\
    ( (This)->lpVtbl -> GetScriptDispatch(This,pstrItemName,ppdisp) ) 

#define IActiveScript_GetCurrentScriptThreadID(This,pstidThread)	\
    ( (This)->lpVtbl -> GetCurrentScriptThreadID(This,pstidThread) ) 

#define IActiveScript_GetScriptThreadID(This,dwWin32ThreadId,pstidThread)	\
    ( (This)->lpVtbl -> GetScriptThreadID(This,dwWin32ThreadId,pstidThread) ) 

#define IActiveScript_GetScriptThreadState(This,stidThread,pstsState)	\
    ( (This)->lpVtbl -> GetScriptThreadState(This,stidThread,pstsState) ) 

#define IActiveScript_InterruptScriptThread(This,stidThread,pexcepinfo,dwFlags)	\
    ( (This)->lpVtbl -> InterruptScriptThread(This,stidThread,pexcepinfo,dwFlags) ) 

#define IActiveScript_Clone(This,ppscript)	\
    ( (This)->lpVtbl -> Clone(This,ppscript) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IActiveScript_INTERFACE_DEFINED__ */


#ifndef __IActiveScriptParse32_INTERFACE_DEFINED__
#define __IActiveScriptParse32_INTERFACE_DEFINED__

/* interface IActiveScriptParse32 */
/* [unique][uuid][object] */ 


EXTERN_C const IID IID_IActiveScriptParse32;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("BB1A2AE2-A4F9-11cf-8F20-00805F2CD064")
    IActiveScriptParse32 : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE InitNew( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE AddScriptlet( 
            /* [in] */ __RPC__in LPCOLESTR pstrDefaultName,
            /* [in] */ __RPC__in LPCOLESTR pstrCode,
            /* [in] */ __RPC__in LPCOLESTR pstrItemName,
            /* [in] */ __RPC__in LPCOLESTR pstrSubItemName,
            /* [in] */ __RPC__in LPCOLESTR pstrEventName,
            /* [in] */ __RPC__in LPCOLESTR pstrDelimiter,
            /* [in] */ DWORD dwSourceContextCookie,
            /* [in] */ ULONG ulStartingLineNumber,
            /* [in] */ DWORD dwFlags,
            /* [out] */ __RPC__deref_out_opt BSTR *pbstrName,
            /* [out] */ __RPC__out EXCEPINFO *pexcepinfo) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE ParseScriptText( 
            /* [in] */ __RPC__in LPCOLESTR pstrCode,
            /* [in] */ __RPC__in LPCOLESTR pstrItemName,
            /* [in] */ __RPC__in_opt IUnknown *punkContext,
            /* [in] */ __RPC__in LPCOLESTR pstrDelimiter,
            /* [in] */ DWORD dwSourceContextCookie,
            /* [in] */ ULONG ulStartingLineNumber,
            /* [in] */ DWORD dwFlags,
            /* [out] */ __RPC__out VARIANT *pvarResult,
            /* [out] */ __RPC__out EXCEPINFO *pexcepinfo) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IActiveScriptParse32Vtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IActiveScriptParse32 * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IActiveScriptParse32 * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IActiveScriptParse32 * This);
        
        DECLSPEC_XFGVIRT(IActiveScriptParse32, InitNew)
        HRESULT ( STDMETHODCALLTYPE *InitNew )( 
            __RPC__in IActiveScriptParse32 * This);
        
        DECLSPEC_XFGVIRT(IActiveScriptParse32, AddScriptlet)
        HRESULT ( STDMETHODCALLTYPE *AddScriptlet )( 
            __RPC__in IActiveScriptParse32 * This,
            /* [in] */ __RPC__in LPCOLESTR pstrDefaultName,
            /* [in] */ __RPC__in LPCOLESTR pstrCode,
            /* [in] */ __RPC__in LPCOLESTR pstrItemName,
            /* [in] */ __RPC__in LPCOLESTR pstrSubItemName,
            /* [in] */ __RPC__in LPCOLESTR pstrEventName,
            /* [in] */ __RPC__in LPCOLESTR pstrDelimiter,
            /* [in] */ DWORD dwSourceContextCookie,
            /* [in] */ ULONG ulStartingLineNumber,
            /* [in] */ DWORD dwFlags,
            /* [out] */ __RPC__deref_out_opt BSTR *pbstrName,
            /* [out] */ __RPC__out EXCEPINFO *pexcepinfo);
        
        DECLSPEC_XFGVIRT(IActiveScriptParse32, ParseScriptText)
        HRESULT ( STDMETHODCALLTYPE *ParseScriptText )( 
            __RPC__in IActiveScriptParse32 * This,
            /* [in] */ __RPC__in LPCOLESTR pstrCode,
            /* [in] */ __RPC__in LPCOLESTR pstrItemName,
            /* [in] */ __RPC__in_opt IUnknown *punkContext,
            /* [in] */ __RPC__in LPCOLESTR pstrDelimiter,
            /* [in] */ DWORD dwSourceContextCookie,
            /* [in] */ ULONG ulStartingLineNumber,
            /* [in] */ DWORD dwFlags,
            /* [out] */ __RPC__out VARIANT *pvarResult,
            /* [out] */ __RPC__out EXCEPINFO *pexcepinfo);
        
        END_INTERFACE
    } IActiveScriptParse32Vtbl;

    interface IActiveScriptParse32
    {
        CONST_VTBL struct IActiveScriptParse32Vtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IActiveScriptParse32_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IActiveScriptParse32_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IActiveScriptParse32_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IActiveScriptParse32_InitNew(This)	\
    ( (This)->lpVtbl -> InitNew(This) ) 

#define IActiveScriptParse32_AddScriptlet(This,pstrDefaultName,pstrCode,pstrItemName,pstrSubItemName,pstrEventName,pstrDelimiter,dwSourceContextCookie,ulStartingLineNumber,dwFlags,pbstrName,pexcepinfo)	\
    ( (This)->lpVtbl -> AddScriptlet(This,pstrDefaultName,pstrCode,pstrItemName,pstrSubItemName,pstrEventName,pstrDelimiter,dwSourceContextCookie,ulStartingLineNumber,dwFlags,pbstrName,pexcepinfo) ) 

#define IActiveScriptParse32_ParseScriptText(This,pstrCode,pstrItemName,punkContext,pstrDelimiter,dwSourceContextCookie,ulStartingLineNumber,dwFlags,pvarResult,pexcepinfo)	\
    ( (This)->lpVtbl -> ParseScriptText(This,pstrCode,pstrItemName,punkContext,pstrDelimiter,dwSourceContextCookie,ulStartingLineNumber,dwFlags,pvarResult,pexcepinfo) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IActiveScriptParse32_INTERFACE_DEFINED__ */


#ifndef __IActiveScriptParse64_INTERFACE_DEFINED__
#define __IActiveScriptParse64_INTERFACE_DEFINED__

/* interface IActiveScriptParse64 */
/* [unique][uuid][object] */ 


EXTERN_C const IID IID_IActiveScriptParse64;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("C7EF7658-E1EE-480E-97EA-D52CB4D76D17")
    IActiveScriptParse64 : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE InitNew( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE AddScriptlet( 
            /* [in] */ __RPC__in LPCOLESTR pstrDefaultName,
            /* [in] */ __RPC__in LPCOLESTR pstrCode,
            /* [in] */ __RPC__in LPCOLESTR pstrItemName,
            /* [in] */ __RPC__in LPCOLESTR pstrSubItemName,
            /* [in] */ __RPC__in LPCOLESTR pstrEventName,
            /* [in] */ __RPC__in LPCOLESTR pstrDelimiter,
            /* [in] */ DWORDLONG dwSourceContextCookie,
            /* [in] */ ULONG ulStartingLineNumber,
            /* [in] */ DWORD dwFlags,
            /* [out] */ __RPC__deref_out_opt BSTR *pbstrName,
            /* [out] */ __RPC__out EXCEPINFO *pexcepinfo) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE ParseScriptText( 
            /* [in] */ __RPC__in LPCOLESTR pstrCode,
            /* [in] */ __RPC__in LPCOLESTR pstrItemName,
            /* [in] */ __RPC__in_opt IUnknown *punkContext,
            /* [in] */ __RPC__in LPCOLESTR pstrDelimiter,
            /* [in] */ DWORDLONG dwSourceContextCookie,
            /* [in] */ ULONG ulStartingLineNumber,
            /* [in] */ DWORD dwFlags,
            /* [out] */ __RPC__out VARIANT *pvarResult,
            /* [out] */ __RPC__out EXCEPINFO *pexcepinfo) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IActiveScriptParse64Vtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IActiveScriptParse64 * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IActiveScriptParse64 * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IActiveScriptParse64 * This);
        
        DECLSPEC_XFGVIRT(IActiveScriptParse64, InitNew)
        HRESULT ( STDMETHODCALLTYPE *InitNew )( 
            __RPC__in IActiveScriptParse64 * This);
        
        DECLSPEC_XFGVIRT(IActiveScriptParse64, AddScriptlet)
        HRESULT ( STDMETHODCALLTYPE *AddScriptlet )( 
            __RPC__in IActiveScriptParse64 * This,
            /* [in] */ __RPC__in LPCOLESTR pstrDefaultName,
            /* [in] */ __RPC__in LPCOLESTR pstrCode,
            /* [in] */ __RPC__in LPCOLESTR pstrItemName,
            /* [in] */ __RPC__in LPCOLESTR pstrSubItemName,
            /* [in] */ __RPC__in LPCOLESTR pstrEventName,
            /* [in] */ __RPC__in LPCOLESTR pstrDelimiter,
            /* [in] */ DWORDLONG dwSourceContextCookie,
            /* [in] */ ULONG ulStartingLineNumber,
            /* [in] */ DWORD dwFlags,
            /* [out] */ __RPC__deref_out_opt BSTR *pbstrName,
            /* [out] */ __RPC__out EXCEPINFO *pexcepinfo);
        
        DECLSPEC_XFGVIRT(IActiveScriptParse64, ParseScriptText)
        HRESULT ( STDMETHODCALLTYPE *ParseScriptText )( 
            __RPC__in IActiveScriptParse64 * This,
            /* [in] */ __RPC__in LPCOLESTR pstrCode,
            /* [in] */ __RPC__in LPCOLESTR pstrItemName,
            /* [in] */ __RPC__in_opt IUnknown *punkContext,
            /* [in] */ __RPC__in LPCOLESTR pstrDelimiter,
            /* [in] */ DWORDLONG dwSourceContextCookie,
            /* [in] */ ULONG ulStartingLineNumber,
            /* [in] */ DWORD dwFlags,
            /* [out] */ __RPC__out VARIANT *pvarResult,
            /* [out] */ __RPC__out EXCEPINFO *pexcepinfo);
        
        END_INTERFACE
    } IActiveScriptParse64Vtbl;

    interface IActiveScriptParse64
    {
        CONST_VTBL struct IActiveScriptParse64Vtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IActiveScriptParse64_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IActiveScriptParse64_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IActiveScriptParse64_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IActiveScriptParse64_InitNew(This)	\
    ( (This)->lpVtbl -> InitNew(This) ) 

#define IActiveScriptParse64_AddScriptlet(This,pstrDefaultName,pstrCode,pstrItemName,pstrSubItemName,pstrEventName,pstrDelimiter,dwSourceContextCookie,ulStartingLineNumber,dwFlags,pbstrName,pexcepinfo)	\
    ( (This)->lpVtbl -> AddScriptlet(This,pstrDefaultName,pstrCode,pstrItemName,pstrSubItemName,pstrEventName,pstrDelimiter,dwSourceContextCookie,ulStartingLineNumber,dwFlags,pbstrName,pexcepinfo) ) 

#define IActiveScriptParse64_ParseScriptText(This,pstrCode,pstrItemName,punkContext,pstrDelimiter,dwSourceContextCookie,ulStartingLineNumber,dwFlags,pvarResult,pexcepinfo)	\
    ( (This)->lpVtbl -> ParseScriptText(This,pstrCode,pstrItemName,punkContext,pstrDelimiter,dwSourceContextCookie,ulStartingLineNumber,dwFlags,pvarResult,pexcepinfo) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IActiveScriptParse64_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_activscp_0000_0009 */
/* [local] */ 

#ifdef _WIN64
#define     IActiveScriptParse     IActiveScriptParse64
#define IID_IActiveScriptParse IID_IActiveScriptParse64
#else
#define     IActiveScriptParse     IActiveScriptParse32
#define IID_IActiveScriptParse IID_IActiveScriptParse32
#endif
typedef IActiveScriptParse *PIActiveScriptParse;



extern RPC_IF_HANDLE __MIDL_itf_activscp_0000_0009_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_activscp_0000_0009_v0_0_s_ifspec;

#ifndef __IActiveScriptParseProcedureOld32_INTERFACE_DEFINED__
#define __IActiveScriptParseProcedureOld32_INTERFACE_DEFINED__

/* interface IActiveScriptParseProcedureOld32 */
/* [unique][uuid][object] */ 


EXTERN_C const IID IID_IActiveScriptParseProcedureOld32;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("1CFF0050-6FDD-11d0-9328-00A0C90DCAA9")
    IActiveScriptParseProcedureOld32 : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE ParseProcedureText( 
            /* [in] */ __RPC__in LPCOLESTR pstrCode,
            /* [in] */ __RPC__in LPCOLESTR pstrFormalParams,
            /* [in] */ __RPC__in LPCOLESTR pstrItemName,
            /* [in] */ __RPC__in_opt IUnknown *punkContext,
            /* [in] */ __RPC__in LPCOLESTR pstrDelimiter,
            /* [in] */ DWORD dwSourceContextCookie,
            /* [in] */ ULONG ulStartingLineNumber,
            /* [in] */ DWORD dwFlags,
            /* [out] */ __RPC__deref_out_opt IDispatch **ppdisp) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IActiveScriptParseProcedureOld32Vtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IActiveScriptParseProcedureOld32 * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IActiveScriptParseProcedureOld32 * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IActiveScriptParseProcedureOld32 * This);
        
        DECLSPEC_XFGVIRT(IActiveScriptParseProcedureOld32, ParseProcedureText)
        HRESULT ( STDMETHODCALLTYPE *ParseProcedureText )( 
            __RPC__in IActiveScriptParseProcedureOld32 * This,
            /* [in] */ __RPC__in LPCOLESTR pstrCode,
            /* [in] */ __RPC__in LPCOLESTR pstrFormalParams,
            /* [in] */ __RPC__in LPCOLESTR pstrItemName,
            /* [in] */ __RPC__in_opt IUnknown *punkContext,
            /* [in] */ __RPC__in LPCOLESTR pstrDelimiter,
            /* [in] */ DWORD dwSourceContextCookie,
            /* [in] */ ULONG ulStartingLineNumber,
            /* [in] */ DWORD dwFlags,
            /* [out] */ __RPC__deref_out_opt IDispatch **ppdisp);
        
        END_INTERFACE
    } IActiveScriptParseProcedureOld32Vtbl;

    interface IActiveScriptParseProcedureOld32
    {
        CONST_VTBL struct IActiveScriptParseProcedureOld32Vtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IActiveScriptParseProcedureOld32_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IActiveScriptParseProcedureOld32_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IActiveScriptParseProcedureOld32_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IActiveScriptParseProcedureOld32_ParseProcedureText(This,pstrCode,pstrFormalParams,pstrItemName,punkContext,pstrDelimiter,dwSourceContextCookie,ulStartingLineNumber,dwFlags,ppdisp)	\
    ( (This)->lpVtbl -> ParseProcedureText(This,pstrCode,pstrFormalParams,pstrItemName,punkContext,pstrDelimiter,dwSourceContextCookie,ulStartingLineNumber,dwFlags,ppdisp) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IActiveScriptParseProcedureOld32_INTERFACE_DEFINED__ */


#ifndef __IActiveScriptParseProcedureOld64_INTERFACE_DEFINED__
#define __IActiveScriptParseProcedureOld64_INTERFACE_DEFINED__

/* interface IActiveScriptParseProcedureOld64 */
/* [unique][uuid][object] */ 


EXTERN_C const IID IID_IActiveScriptParseProcedureOld64;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("21F57128-08C9-4638-BA12-22D15D88DC5C")
    IActiveScriptParseProcedureOld64 : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE ParseProcedureText( 
            /* [in] */ __RPC__in LPCOLESTR pstrCode,
            /* [in] */ __RPC__in LPCOLESTR pstrFormalParams,
            /* [in] */ __RPC__in LPCOLESTR pstrItemName,
            /* [in] */ __RPC__in_opt IUnknown *punkContext,
            /* [in] */ __RPC__in LPCOLESTR pstrDelimiter,
            /* [in] */ DWORDLONG dwSourceContextCookie,
            /* [in] */ ULONG ulStartingLineNumber,
            /* [in] */ DWORD dwFlags,
            /* [out] */ __RPC__deref_out_opt IDispatch **ppdisp) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IActiveScriptParseProcedureOld64Vtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IActiveScriptParseProcedureOld64 * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IActiveScriptParseProcedureOld64 * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IActiveScriptParseProcedureOld64 * This);
        
        DECLSPEC_XFGVIRT(IActiveScriptParseProcedureOld64, ParseProcedureText)
        HRESULT ( STDMETHODCALLTYPE *ParseProcedureText )( 
            __RPC__in IActiveScriptParseProcedureOld64 * This,
            /* [in] */ __RPC__in LPCOLESTR pstrCode,
            /* [in] */ __RPC__in LPCOLESTR pstrFormalParams,
            /* [in] */ __RPC__in LPCOLESTR pstrItemName,
            /* [in] */ __RPC__in_opt IUnknown *punkContext,
            /* [in] */ __RPC__in LPCOLESTR pstrDelimiter,
            /* [in] */ DWORDLONG dwSourceContextCookie,
            /* [in] */ ULONG ulStartingLineNumber,
            /* [in] */ DWORD dwFlags,
            /* [out] */ __RPC__deref_out_opt IDispatch **ppdisp);
        
        END_INTERFACE
    } IActiveScriptParseProcedureOld64Vtbl;

    interface IActiveScriptParseProcedureOld64
    {
        CONST_VTBL struct IActiveScriptParseProcedureOld64Vtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IActiveScriptParseProcedureOld64_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IActiveScriptParseProcedureOld64_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IActiveScriptParseProcedureOld64_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IActiveScriptParseProcedureOld64_ParseProcedureText(This,pstrCode,pstrFormalParams,pstrItemName,punkContext,pstrDelimiter,dwSourceContextCookie,ulStartingLineNumber,dwFlags,ppdisp)	\
    ( (This)->lpVtbl -> ParseProcedureText(This,pstrCode,pstrFormalParams,pstrItemName,punkContext,pstrDelimiter,dwSourceContextCookie,ulStartingLineNumber,dwFlags,ppdisp) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IActiveScriptParseProcedureOld64_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_activscp_0000_0011 */
/* [local] */ 

#ifdef _WIN64
#define     IActiveScriptParseProcedureOld     IActiveScriptParseProcedureOld64
#define IID_IActiveScriptParseProcedureOld IID_IActiveScriptParseProcedureOld64
#else
#define     IActiveScriptParseProcedureOld     IActiveScriptParseProcedureOld32
#define IID_IActiveScriptParseProcedureOld IID_IActiveScriptParseProcedureOld32
#endif
typedef IActiveScriptParseProcedureOld *PIActiveScriptParseProcedureOld;


extern RPC_IF_HANDLE __MIDL_itf_activscp_0000_0011_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_activscp_0000_0011_v0_0_s_ifspec;

#ifndef __IActiveScriptParseProcedure32_INTERFACE_DEFINED__
#define __IActiveScriptParseProcedure32_INTERFACE_DEFINED__

/* interface IActiveScriptParseProcedure32 */
/* [unique][uuid][object] */ 


EXTERN_C const IID IID_IActiveScriptParseProcedure32;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("AA5B6A80-B834-11d0-932F-00A0C90DCAA9")
    IActiveScriptParseProcedure32 : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE ParseProcedureText( 
            /* [in] */ __RPC__in LPCOLESTR pstrCode,
            /* [in] */ __RPC__in LPCOLESTR pstrFormalParams,
            /* [in] */ __RPC__in LPCOLESTR pstrProcedureName,
            /* [in] */ __RPC__in LPCOLESTR pstrItemName,
            /* [in] */ __RPC__in_opt IUnknown *punkContext,
            /* [in] */ __RPC__in LPCOLESTR pstrDelimiter,
            /* [in] */ DWORD dwSourceContextCookie,
            /* [in] */ ULONG ulStartingLineNumber,
            /* [in] */ DWORD dwFlags,
            /* [out] */ __RPC__deref_out_opt IDispatch **ppdisp) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IActiveScriptParseProcedure32Vtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IActiveScriptParseProcedure32 * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IActiveScriptParseProcedure32 * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IActiveScriptParseProcedure32 * This);
        
        DECLSPEC_XFGVIRT(IActiveScriptParseProcedure32, ParseProcedureText)
        HRESULT ( STDMETHODCALLTYPE *ParseProcedureText )( 
            __RPC__in IActiveScriptParseProcedure32 * This,
            /* [in] */ __RPC__in LPCOLESTR pstrCode,
            /* [in] */ __RPC__in LPCOLESTR pstrFormalParams,
            /* [in] */ __RPC__in LPCOLESTR pstrProcedureName,
            /* [in] */ __RPC__in LPCOLESTR pstrItemName,
            /* [in] */ __RPC__in_opt IUnknown *punkContext,
            /* [in] */ __RPC__in LPCOLESTR pstrDelimiter,
            /* [in] */ DWORD dwSourceContextCookie,
            /* [in] */ ULONG ulStartingLineNumber,
            /* [in] */ DWORD dwFlags,
            /* [out] */ __RPC__deref_out_opt IDispatch **ppdisp);
        
        END_INTERFACE
    } IActiveScriptParseProcedure32Vtbl;

    interface IActiveScriptParseProcedure32
    {
        CONST_VTBL struct IActiveScriptParseProcedure32Vtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IActiveScriptParseProcedure32_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IActiveScriptParseProcedure32_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IActiveScriptParseProcedure32_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IActiveScriptParseProcedure32_ParseProcedureText(This,pstrCode,pstrFormalParams,pstrProcedureName,pstrItemName,punkContext,pstrDelimiter,dwSourceContextCookie,ulStartingLineNumber,dwFlags,ppdisp)	\
    ( (This)->lpVtbl -> ParseProcedureText(This,pstrCode,pstrFormalParams,pstrProcedureName,pstrItemName,punkContext,pstrDelimiter,dwSourceContextCookie,ulStartingLineNumber,dwFlags,ppdisp) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IActiveScriptParseProcedure32_INTERFACE_DEFINED__ */


#ifndef __IActiveScriptParseProcedure64_INTERFACE_DEFINED__
#define __IActiveScriptParseProcedure64_INTERFACE_DEFINED__

/* interface IActiveScriptParseProcedure64 */
/* [unique][uuid][object] */ 


EXTERN_C const IID IID_IActiveScriptParseProcedure64;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("C64713B6-E029-4CC5-9200-438B72890B6A")
    IActiveScriptParseProcedure64 : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE ParseProcedureText( 
            /* [in] */ __RPC__in LPCOLESTR pstrCode,
            /* [in] */ __RPC__in LPCOLESTR pstrFormalParams,
            /* [in] */ __RPC__in LPCOLESTR pstrProcedureName,
            /* [in] */ __RPC__in LPCOLESTR pstrItemName,
            /* [in] */ __RPC__in_opt IUnknown *punkContext,
            /* [in] */ __RPC__in LPCOLESTR pstrDelimiter,
            /* [in] */ DWORDLONG dwSourceContextCookie,
            /* [in] */ ULONG ulStartingLineNumber,
            /* [in] */ DWORD dwFlags,
            /* [out] */ __RPC__deref_out_opt IDispatch **ppdisp) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IActiveScriptParseProcedure64Vtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IActiveScriptParseProcedure64 * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IActiveScriptParseProcedure64 * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IActiveScriptParseProcedure64 * This);
        
        DECLSPEC_XFGVIRT(IActiveScriptParseProcedure64, ParseProcedureText)
        HRESULT ( STDMETHODCALLTYPE *ParseProcedureText )( 
            __RPC__in IActiveScriptParseProcedure64 * This,
            /* [in] */ __RPC__in LPCOLESTR pstrCode,
            /* [in] */ __RPC__in LPCOLESTR pstrFormalParams,
            /* [in] */ __RPC__in LPCOLESTR pstrProcedureName,
            /* [in] */ __RPC__in LPCOLESTR pstrItemName,
            /* [in] */ __RPC__in_opt IUnknown *punkContext,
            /* [in] */ __RPC__in LPCOLESTR pstrDelimiter,
            /* [in] */ DWORDLONG dwSourceContextCookie,
            /* [in] */ ULONG ulStartingLineNumber,
            /* [in] */ DWORD dwFlags,
            /* [out] */ __RPC__deref_out_opt IDispatch **ppdisp);
        
        END_INTERFACE
    } IActiveScriptParseProcedure64Vtbl;

    interface IActiveScriptParseProcedure64
    {
        CONST_VTBL struct IActiveScriptParseProcedure64Vtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IActiveScriptParseProcedure64_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IActiveScriptParseProcedure64_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IActiveScriptParseProcedure64_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IActiveScriptParseProcedure64_ParseProcedureText(This,pstrCode,pstrFormalParams,pstrProcedureName,pstrItemName,punkContext,pstrDelimiter,dwSourceContextCookie,ulStartingLineNumber,dwFlags,ppdisp)	\
    ( (This)->lpVtbl -> ParseProcedureText(This,pstrCode,pstrFormalParams,pstrProcedureName,pstrItemName,punkContext,pstrDelimiter,dwSourceContextCookie,ulStartingLineNumber,dwFlags,ppdisp) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IActiveScriptParseProcedure64_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_activscp_0000_0013 */
/* [local] */ 

#ifdef _WIN64
#define     IActiveScriptParseProcedure     IActiveScriptParseProcedure64
#define IID_IActiveScriptParseProcedure IID_IActiveScriptParseProcedure64
#else
#define     IActiveScriptParseProcedure     IActiveScriptParseProcedure32
#define IID_IActiveScriptParseProcedure IID_IActiveScriptParseProcedure32
#endif
typedef IActiveScriptParseProcedure *PIActiveScriptParseProcedure;


extern RPC_IF_HANDLE __MIDL_itf_activscp_0000_0013_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_activscp_0000_0013_v0_0_s_ifspec;

#ifndef __IActiveScriptParseProcedure2_32_INTERFACE_DEFINED__
#define __IActiveScriptParseProcedure2_32_INTERFACE_DEFINED__

/* interface IActiveScriptParseProcedure2_32 */
/* [unique][uuid][object] */ 


EXTERN_C const IID IID_IActiveScriptParseProcedure2_32;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("71EE5B20-FB04-11d1-B3A8-00A0C911E8B2")
    IActiveScriptParseProcedure2_32 : public IActiveScriptParseProcedure32
    {
    public:
    };
    
    
#else 	/* C style interface */

    typedef struct IActiveScriptParseProcedure2_32Vtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IActiveScriptParseProcedure2_32 * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IActiveScriptParseProcedure2_32 * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IActiveScriptParseProcedure2_32 * This);
        
        DECLSPEC_XFGVIRT(IActiveScriptParseProcedure32, ParseProcedureText)
        HRESULT ( STDMETHODCALLTYPE *ParseProcedureText )( 
            __RPC__in IActiveScriptParseProcedure2_32 * This,
            /* [in] */ __RPC__in LPCOLESTR pstrCode,
            /* [in] */ __RPC__in LPCOLESTR pstrFormalParams,
            /* [in] */ __RPC__in LPCOLESTR pstrProcedureName,
            /* [in] */ __RPC__in LPCOLESTR pstrItemName,
            /* [in] */ __RPC__in_opt IUnknown *punkContext,
            /* [in] */ __RPC__in LPCOLESTR pstrDelimiter,
            /* [in] */ DWORD dwSourceContextCookie,
            /* [in] */ ULONG ulStartingLineNumber,
            /* [in] */ DWORD dwFlags,
            /* [out] */ __RPC__deref_out_opt IDispatch **ppdisp);
        
        END_INTERFACE
    } IActiveScriptParseProcedure2_32Vtbl;

    interface IActiveScriptParseProcedure2_32
    {
        CONST_VTBL struct IActiveScriptParseProcedure2_32Vtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IActiveScriptParseProcedure2_32_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IActiveScriptParseProcedure2_32_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IActiveScriptParseProcedure2_32_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IActiveScriptParseProcedure2_32_ParseProcedureText(This,pstrCode,pstrFormalParams,pstrProcedureName,pstrItemName,punkContext,pstrDelimiter,dwSourceContextCookie,ulStartingLineNumber,dwFlags,ppdisp)	\
    ( (This)->lpVtbl -> ParseProcedureText(This,pstrCode,pstrFormalParams,pstrProcedureName,pstrItemName,punkContext,pstrDelimiter,dwSourceContextCookie,ulStartingLineNumber,dwFlags,ppdisp) ) 


#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IActiveScriptParseProcedure2_32_INTERFACE_DEFINED__ */


#ifndef __IActiveScriptParseProcedure2_64_INTERFACE_DEFINED__
#define __IActiveScriptParseProcedure2_64_INTERFACE_DEFINED__

/* interface IActiveScriptParseProcedure2_64 */
/* [unique][uuid][object] */ 


EXTERN_C const IID IID_IActiveScriptParseProcedure2_64;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("FE7C4271-210C-448D-9F54-76DAB7047B28")
    IActiveScriptParseProcedure2_64 : public IActiveScriptParseProcedure64
    {
    public:
    };
    
    
#else 	/* C style interface */

    typedef struct IActiveScriptParseProcedure2_64Vtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IActiveScriptParseProcedure2_64 * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IActiveScriptParseProcedure2_64 * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IActiveScriptParseProcedure2_64 * This);
        
        DECLSPEC_XFGVIRT(IActiveScriptParseProcedure64, ParseProcedureText)
        HRESULT ( STDMETHODCALLTYPE *ParseProcedureText )( 
            __RPC__in IActiveScriptParseProcedure2_64 * This,
            /* [in] */ __RPC__in LPCOLESTR pstrCode,
            /* [in] */ __RPC__in LPCOLESTR pstrFormalParams,
            /* [in] */ __RPC__in LPCOLESTR pstrProcedureName,
            /* [in] */ __RPC__in LPCOLESTR pstrItemName,
            /* [in] */ __RPC__in_opt IUnknown *punkContext,
            /* [in] */ __RPC__in LPCOLESTR pstrDelimiter,
            /* [in] */ DWORDLONG dwSourceContextCookie,
            /* [in] */ ULONG ulStartingLineNumber,
            /* [in] */ DWORD dwFlags,
            /* [out] */ __RPC__deref_out_opt IDispatch **ppdisp);
        
        END_INTERFACE
    } IActiveScriptParseProcedure2_64Vtbl;

    interface IActiveScriptParseProcedure2_64
    {
        CONST_VTBL struct IActiveScriptParseProcedure2_64Vtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IActiveScriptParseProcedure2_64_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IActiveScriptParseProcedure2_64_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IActiveScriptParseProcedure2_64_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IActiveScriptParseProcedure2_64_ParseProcedureText(This,pstrCode,pstrFormalParams,pstrProcedureName,pstrItemName,punkContext,pstrDelimiter,dwSourceContextCookie,ulStartingLineNumber,dwFlags,ppdisp)	\
    ( (This)->lpVtbl -> ParseProcedureText(This,pstrCode,pstrFormalParams,pstrProcedureName,pstrItemName,punkContext,pstrDelimiter,dwSourceContextCookie,ulStartingLineNumber,dwFlags,ppdisp) ) 


#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IActiveScriptParseProcedure2_64_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_activscp_0000_0015 */
/* [local] */ 

#ifdef _WIN64
#define     IActiveScriptParseProcedure2     IActiveScriptParseProcedure2_64
#define IID_IActiveScriptParseProcedure2 IID_IActiveScriptParseProcedure2_64
#else
#define     IActiveScriptParseProcedure2     IActiveScriptParseProcedure2_32
#define IID_IActiveScriptParseProcedure2 IID_IActiveScriptParseProcedure2_32
#endif
typedef IActiveScriptParseProcedure2 *PIActiveScriptParseProcedure2;


extern RPC_IF_HANDLE __MIDL_itf_activscp_0000_0015_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_activscp_0000_0015_v0_0_s_ifspec;

#ifndef __IActiveScriptEncode_INTERFACE_DEFINED__
#define __IActiveScriptEncode_INTERFACE_DEFINED__

/* interface IActiveScriptEncode */
/* [unique][uuid][object] */ 


EXTERN_C const IID IID_IActiveScriptEncode;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("BB1A2AE3-A4F9-11cf-8F20-00805F2CD064")
    IActiveScriptEncode : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE EncodeSection( 
            /* [in] */ __RPC__in LPCOLESTR pchIn,
            /* [in] */ DWORD cchIn,
            /* [out][in] */ __RPC__inout LPOLESTR pchOut,
            /* [in] */ DWORD cchOut,
            /* [out][in] */ __RPC__inout DWORD *pcchRet) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE DecodeScript( 
            /* [in] */ __RPC__in LPCOLESTR pchIn,
            /* [in] */ DWORD cchIn,
            /* [out][in] */ __RPC__inout LPOLESTR pchOut,
            /* [in] */ DWORD cchOut,
            /* [out][in] */ __RPC__inout DWORD *pcchRet) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetEncodeProgId( 
            /* [out][in] */ __RPC__deref_inout_opt BSTR *pbstrOut) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IActiveScriptEncodeVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IActiveScriptEncode * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IActiveScriptEncode * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IActiveScriptEncode * This);
        
        DECLSPEC_XFGVIRT(IActiveScriptEncode, EncodeSection)
        HRESULT ( STDMETHODCALLTYPE *EncodeSection )( 
            __RPC__in IActiveScriptEncode * This,
            /* [in] */ __RPC__in LPCOLESTR pchIn,
            /* [in] */ DWORD cchIn,
            /* [out][in] */ __RPC__inout LPOLESTR pchOut,
            /* [in] */ DWORD cchOut,
            /* [out][in] */ __RPC__inout DWORD *pcchRet);
        
        DECLSPEC_XFGVIRT(IActiveScriptEncode, DecodeScript)
        HRESULT ( STDMETHODCALLTYPE *DecodeScript )( 
            __RPC__in IActiveScriptEncode * This,
            /* [in] */ __RPC__in LPCOLESTR pchIn,
            /* [in] */ DWORD cchIn,
            /* [out][in] */ __RPC__inout LPOLESTR pchOut,
            /* [in] */ DWORD cchOut,
            /* [out][in] */ __RPC__inout DWORD *pcchRet);
        
        DECLSPEC_XFGVIRT(IActiveScriptEncode, GetEncodeProgId)
        HRESULT ( STDMETHODCALLTYPE *GetEncodeProgId )( 
            __RPC__in IActiveScriptEncode * This,
            /* [out][in] */ __RPC__deref_inout_opt BSTR *pbstrOut);
        
        END_INTERFACE
    } IActiveScriptEncodeVtbl;

    interface IActiveScriptEncode
    {
        CONST_VTBL struct IActiveScriptEncodeVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IActiveScriptEncode_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IActiveScriptEncode_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IActiveScriptEncode_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IActiveScriptEncode_EncodeSection(This,pchIn,cchIn,pchOut,cchOut,pcchRet)	\
    ( (This)->lpVtbl -> EncodeSection(This,pchIn,cchIn,pchOut,cchOut,pcchRet) ) 

#define IActiveScriptEncode_DecodeScript(This,pchIn,cchIn,pchOut,cchOut,pcchRet)	\
    ( (This)->lpVtbl -> DecodeScript(This,pchIn,cchIn,pchOut,cchOut,pcchRet) ) 

#define IActiveScriptEncode_GetEncodeProgId(This,pbstrOut)	\
    ( (This)->lpVtbl -> GetEncodeProgId(This,pbstrOut) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IActiveScriptEncode_INTERFACE_DEFINED__ */


#ifndef __IActiveScriptHostEncode_INTERFACE_DEFINED__
#define __IActiveScriptHostEncode_INTERFACE_DEFINED__

/* interface IActiveScriptHostEncode */
/* [unique][uuid][object] */ 


EXTERN_C const IID IID_IActiveScriptHostEncode;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("BEE9B76E-CFE3-11d1-B747-00C04FC2B085")
    IActiveScriptHostEncode : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE EncodeScriptHostFile( 
            /* [in] */ __RPC__in BSTR bstrInFile,
            /* [out][in] */ __RPC__deref_inout_opt BSTR *pbstrOutFile,
            /* [in] */ unsigned long cFlags,
            /* [in] */ __RPC__in BSTR bstrDefaultLang) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IActiveScriptHostEncodeVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IActiveScriptHostEncode * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IActiveScriptHostEncode * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IActiveScriptHostEncode * This);
        
        DECLSPEC_XFGVIRT(IActiveScriptHostEncode, EncodeScriptHostFile)
        HRESULT ( STDMETHODCALLTYPE *EncodeScriptHostFile )( 
            __RPC__in IActiveScriptHostEncode * This,
            /* [in] */ __RPC__in BSTR bstrInFile,
            /* [out][in] */ __RPC__deref_inout_opt BSTR *pbstrOutFile,
            /* [in] */ unsigned long cFlags,
            /* [in] */ __RPC__in BSTR bstrDefaultLang);
        
        END_INTERFACE
    } IActiveScriptHostEncodeVtbl;

    interface IActiveScriptHostEncode
    {
        CONST_VTBL struct IActiveScriptHostEncodeVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IActiveScriptHostEncode_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IActiveScriptHostEncode_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IActiveScriptHostEncode_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IActiveScriptHostEncode_EncodeScriptHostFile(This,bstrInFile,pbstrOutFile,cFlags,bstrDefaultLang)	\
    ( (This)->lpVtbl -> EncodeScriptHostFile(This,bstrInFile,pbstrOutFile,cFlags,bstrDefaultLang) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IActiveScriptHostEncode_INTERFACE_DEFINED__ */


#ifndef __IBindEventHandler_INTERFACE_DEFINED__
#define __IBindEventHandler_INTERFACE_DEFINED__

/* interface IBindEventHandler */
/* [unique][uuid][object] */ 


EXTERN_C const IID IID_IBindEventHandler;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("63CDBCB0-C1B1-11d0-9336-00A0C90DCAA9")
    IBindEventHandler : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE BindHandler( 
            /* [in] */ __RPC__in LPCOLESTR pstrEvent,
            /* [in] */ __RPC__in_opt IDispatch *pdisp) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IBindEventHandlerVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IBindEventHandler * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IBindEventHandler * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IBindEventHandler * This);
        
        DECLSPEC_XFGVIRT(IBindEventHandler, BindHandler)
        HRESULT ( STDMETHODCALLTYPE *BindHandler )( 
            __RPC__in IBindEventHandler * This,
            /* [in] */ __RPC__in LPCOLESTR pstrEvent,
            /* [in] */ __RPC__in_opt IDispatch *pdisp);
        
        END_INTERFACE
    } IBindEventHandlerVtbl;

    interface IBindEventHandler
    {
        CONST_VTBL struct IBindEventHandlerVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IBindEventHandler_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IBindEventHandler_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IBindEventHandler_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IBindEventHandler_BindHandler(This,pstrEvent,pdisp)	\
    ( (This)->lpVtbl -> BindHandler(This,pstrEvent,pdisp) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IBindEventHandler_INTERFACE_DEFINED__ */


#ifndef __IActiveScriptStats_INTERFACE_DEFINED__
#define __IActiveScriptStats_INTERFACE_DEFINED__

/* interface IActiveScriptStats */
/* [unique][uuid][object] */ 


EXTERN_C const IID IID_IActiveScriptStats;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("B8DA6310-E19B-11d0-933C-00A0C90DCAA9")
    IActiveScriptStats : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetStat( 
            /* [in] */ DWORD stid,
            /* [out] */ __RPC__out ULONG *pluHi,
            /* [out] */ __RPC__out ULONG *pluLo) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetStatEx( 
            /* [in] */ __RPC__in REFGUID guid,
            /* [out] */ __RPC__out ULONG *pluHi,
            /* [out] */ __RPC__out ULONG *pluLo) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE ResetStats( void) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IActiveScriptStatsVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IActiveScriptStats * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IActiveScriptStats * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IActiveScriptStats * This);
        
        DECLSPEC_XFGVIRT(IActiveScriptStats, GetStat)
        HRESULT ( STDMETHODCALLTYPE *GetStat )( 
            __RPC__in IActiveScriptStats * This,
            /* [in] */ DWORD stid,
            /* [out] */ __RPC__out ULONG *pluHi,
            /* [out] */ __RPC__out ULONG *pluLo);
        
        DECLSPEC_XFGVIRT(IActiveScriptStats, GetStatEx)
        HRESULT ( STDMETHODCALLTYPE *GetStatEx )( 
            __RPC__in IActiveScriptStats * This,
            /* [in] */ __RPC__in REFGUID guid,
            /* [out] */ __RPC__out ULONG *pluHi,
            /* [out] */ __RPC__out ULONG *pluLo);
        
        DECLSPEC_XFGVIRT(IActiveScriptStats, ResetStats)
        HRESULT ( STDMETHODCALLTYPE *ResetStats )( 
            __RPC__in IActiveScriptStats * This);
        
        END_INTERFACE
    } IActiveScriptStatsVtbl;

    interface IActiveScriptStats
    {
        CONST_VTBL struct IActiveScriptStatsVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IActiveScriptStats_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IActiveScriptStats_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IActiveScriptStats_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IActiveScriptStats_GetStat(This,stid,pluHi,pluLo)	\
    ( (This)->lpVtbl -> GetStat(This,stid,pluHi,pluLo) ) 

#define IActiveScriptStats_GetStatEx(This,guid,pluHi,pluLo)	\
    ( (This)->lpVtbl -> GetStatEx(This,guid,pluHi,pluLo) ) 

#define IActiveScriptStats_ResetStats(This)	\
    ( (This)->lpVtbl -> ResetStats(This) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IActiveScriptStats_INTERFACE_DEFINED__ */


#ifndef __IActiveScriptProperty_INTERFACE_DEFINED__
#define __IActiveScriptProperty_INTERFACE_DEFINED__

/* interface IActiveScriptProperty */
/* [unique][uuid][object] */ 


EXTERN_C const IID IID_IActiveScriptProperty;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("4954E0D0-FBC7-11D1-8410-006008C3FBFC")
    IActiveScriptProperty : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetProperty( 
            /* [in] */ DWORD dwProperty,
            /* [in] */ __RPC__in VARIANT *pvarIndex,
            /* [out] */ __RPC__out VARIANT *pvarValue) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetProperty( 
            /* [in] */ DWORD dwProperty,
            /* [in] */ __RPC__in VARIANT *pvarIndex,
            /* [in] */ __RPC__in VARIANT *pvarValue) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IActiveScriptPropertyVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IActiveScriptProperty * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IActiveScriptProperty * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IActiveScriptProperty * This);
        
        DECLSPEC_XFGVIRT(IActiveScriptProperty, GetProperty)
        HRESULT ( STDMETHODCALLTYPE *GetProperty )( 
            __RPC__in IActiveScriptProperty * This,
            /* [in] */ DWORD dwProperty,
            /* [in] */ __RPC__in VARIANT *pvarIndex,
            /* [out] */ __RPC__out VARIANT *pvarValue);
        
        DECLSPEC_XFGVIRT(IActiveScriptProperty, SetProperty)
        HRESULT ( STDMETHODCALLTYPE *SetProperty )( 
            __RPC__in IActiveScriptProperty * This,
            /* [in] */ DWORD dwProperty,
            /* [in] */ __RPC__in VARIANT *pvarIndex,
            /* [in] */ __RPC__in VARIANT *pvarValue);
        
        END_INTERFACE
    } IActiveScriptPropertyVtbl;

    interface IActiveScriptProperty
    {
        CONST_VTBL struct IActiveScriptPropertyVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IActiveScriptProperty_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IActiveScriptProperty_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IActiveScriptProperty_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IActiveScriptProperty_GetProperty(This,dwProperty,pvarIndex,pvarValue)	\
    ( (This)->lpVtbl -> GetProperty(This,dwProperty,pvarIndex,pvarValue) ) 

#define IActiveScriptProperty_SetProperty(This,dwProperty,pvarIndex,pvarValue)	\
    ( (This)->lpVtbl -> SetProperty(This,dwProperty,pvarIndex,pvarValue) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IActiveScriptProperty_INTERFACE_DEFINED__ */


#ifndef __ITridentEventSink_INTERFACE_DEFINED__
#define __ITridentEventSink_INTERFACE_DEFINED__

/* interface ITridentEventSink */
/* [unique][uuid][object] */ 


EXTERN_C const IID IID_ITridentEventSink;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("1DC9CA50-06EF-11d2-8415-006008C3FBFC")
    ITridentEventSink : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE FireEvent( 
            /* [in] */ __RPC__in LPCOLESTR pstrEvent,
            /* [in] */ __RPC__in DISPPARAMS *pdp,
            /* [out] */ __RPC__out VARIANT *pvarRes,
            /* [out] */ __RPC__out EXCEPINFO *pei) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ITridentEventSinkVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in ITridentEventSink * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in ITridentEventSink * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in ITridentEventSink * This);
        
        DECLSPEC_XFGVIRT(ITridentEventSink, FireEvent)
        HRESULT ( STDMETHODCALLTYPE *FireEvent )( 
            __RPC__in ITridentEventSink * This,
            /* [in] */ __RPC__in LPCOLESTR pstrEvent,
            /* [in] */ __RPC__in DISPPARAMS *pdp,
            /* [out] */ __RPC__out VARIANT *pvarRes,
            /* [out] */ __RPC__out EXCEPINFO *pei);
        
        END_INTERFACE
    } ITridentEventSinkVtbl;

    interface ITridentEventSink
    {
        CONST_VTBL struct ITridentEventSinkVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ITridentEventSink_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ITridentEventSink_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ITridentEventSink_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ITridentEventSink_FireEvent(This,pstrEvent,pdp,pvarRes,pei)	\
    ( (This)->lpVtbl -> FireEvent(This,pstrEvent,pdp,pvarRes,pei) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ITridentEventSink_INTERFACE_DEFINED__ */


#ifndef __IActiveScriptGarbageCollector_INTERFACE_DEFINED__
#define __IActiveScriptGarbageCollector_INTERFACE_DEFINED__

/* interface IActiveScriptGarbageCollector */
/* [unique][uuid][object] */ 


EXTERN_C const IID IID_IActiveScriptGarbageCollector;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("6AA2C4A0-2B53-11d4-A2A0-00104BD35090")
    IActiveScriptGarbageCollector : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE CollectGarbage( 
            SCRIPTGCTYPE scriptgctype) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IActiveScriptGarbageCollectorVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IActiveScriptGarbageCollector * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IActiveScriptGarbageCollector * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IActiveScriptGarbageCollector * This);
        
        DECLSPEC_XFGVIRT(IActiveScriptGarbageCollector, CollectGarbage)
        HRESULT ( STDMETHODCALLTYPE *CollectGarbage )( 
            __RPC__in IActiveScriptGarbageCollector * This,
            SCRIPTGCTYPE scriptgctype);
        
        END_INTERFACE
    } IActiveScriptGarbageCollectorVtbl;

    interface IActiveScriptGarbageCollector
    {
        CONST_VTBL struct IActiveScriptGarbageCollectorVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IActiveScriptGarbageCollector_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IActiveScriptGarbageCollector_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IActiveScriptGarbageCollector_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IActiveScriptGarbageCollector_CollectGarbage(This,scriptgctype)	\
    ( (This)->lpVtbl -> CollectGarbage(This,scriptgctype) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IActiveScriptGarbageCollector_INTERFACE_DEFINED__ */


#ifndef __IActiveScriptSIPInfo_INTERFACE_DEFINED__
#define __IActiveScriptSIPInfo_INTERFACE_DEFINED__

/* interface IActiveScriptSIPInfo */
/* [unique][uuid][object] */ 


EXTERN_C const IID IID_IActiveScriptSIPInfo;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("764651D0-38DE-11d4-A2A3-00104BD35090")
    IActiveScriptSIPInfo : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetSIPOID( 
            /* [out] */ __RPC__out GUID *poid_sip) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IActiveScriptSIPInfoVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IActiveScriptSIPInfo * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IActiveScriptSIPInfo * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IActiveScriptSIPInfo * This);
        
        DECLSPEC_XFGVIRT(IActiveScriptSIPInfo, GetSIPOID)
        HRESULT ( STDMETHODCALLTYPE *GetSIPOID )( 
            __RPC__in IActiveScriptSIPInfo * This,
            /* [out] */ __RPC__out GUID *poid_sip);
        
        END_INTERFACE
    } IActiveScriptSIPInfoVtbl;

    interface IActiveScriptSIPInfo
    {
        CONST_VTBL struct IActiveScriptSIPInfoVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IActiveScriptSIPInfo_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IActiveScriptSIPInfo_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IActiveScriptSIPInfo_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IActiveScriptSIPInfo_GetSIPOID(This,poid_sip)	\
    ( (This)->lpVtbl -> GetSIPOID(This,poid_sip) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IActiveScriptSIPInfo_INTERFACE_DEFINED__ */


#ifndef __IActiveScriptSiteTraceInfo_INTERFACE_DEFINED__
#define __IActiveScriptSiteTraceInfo_INTERFACE_DEFINED__

/* interface IActiveScriptSiteTraceInfo */
/* [unique][uuid][object] */ 


EXTERN_C const IID IID_IActiveScriptSiteTraceInfo;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("4B7272AE-1955-4bfe-98B0-780621888569")
    IActiveScriptSiteTraceInfo : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE SendScriptTraceInfo( 
            /* [in] */ SCRIPTTRACEINFO stiEventType,
            /* [in] */ GUID guidContextID,
            /* [in] */ DWORD dwScriptContextCookie,
            /* [in] */ LONG lScriptStatementStart,
            /* [in] */ LONG lScriptStatementEnd,
            /* [in] */ DWORD64 dwReserved) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IActiveScriptSiteTraceInfoVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IActiveScriptSiteTraceInfo * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IActiveScriptSiteTraceInfo * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IActiveScriptSiteTraceInfo * This);
        
        DECLSPEC_XFGVIRT(IActiveScriptSiteTraceInfo, SendScriptTraceInfo)
        HRESULT ( STDMETHODCALLTYPE *SendScriptTraceInfo )( 
            __RPC__in IActiveScriptSiteTraceInfo * This,
            /* [in] */ SCRIPTTRACEINFO stiEventType,
            /* [in] */ GUID guidContextID,
            /* [in] */ DWORD dwScriptContextCookie,
            /* [in] */ LONG lScriptStatementStart,
            /* [in] */ LONG lScriptStatementEnd,
            /* [in] */ DWORD64 dwReserved);
        
        END_INTERFACE
    } IActiveScriptSiteTraceInfoVtbl;

    interface IActiveScriptSiteTraceInfo
    {
        CONST_VTBL struct IActiveScriptSiteTraceInfoVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IActiveScriptSiteTraceInfo_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IActiveScriptSiteTraceInfo_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IActiveScriptSiteTraceInfo_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IActiveScriptSiteTraceInfo_SendScriptTraceInfo(This,stiEventType,guidContextID,dwScriptContextCookie,lScriptStatementStart,lScriptStatementEnd,dwReserved)	\
    ( (This)->lpVtbl -> SendScriptTraceInfo(This,stiEventType,guidContextID,dwScriptContextCookie,lScriptStatementStart,lScriptStatementEnd,dwReserved) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IActiveScriptSiteTraceInfo_INTERFACE_DEFINED__ */


#ifndef __IActiveScriptTraceInfo_INTERFACE_DEFINED__
#define __IActiveScriptTraceInfo_INTERFACE_DEFINED__

/* interface IActiveScriptTraceInfo */
/* [unique][uuid][object] */ 


EXTERN_C const IID IID_IActiveScriptTraceInfo;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("C35456E7-BEBF-4a1b-86A9-24D56BE8B369")
    IActiveScriptTraceInfo : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE StartScriptTracing( 
            /* [in] */ __RPC__in_opt IActiveScriptSiteTraceInfo *pSiteTraceInfo,
            /* [in] */ GUID guidContextID) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE StopScriptTracing( void) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IActiveScriptTraceInfoVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IActiveScriptTraceInfo * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IActiveScriptTraceInfo * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IActiveScriptTraceInfo * This);
        
        DECLSPEC_XFGVIRT(IActiveScriptTraceInfo, StartScriptTracing)
        HRESULT ( STDMETHODCALLTYPE *StartScriptTracing )( 
            __RPC__in IActiveScriptTraceInfo * This,
            /* [in] */ __RPC__in_opt IActiveScriptSiteTraceInfo *pSiteTraceInfo,
            /* [in] */ GUID guidContextID);
        
        DECLSPEC_XFGVIRT(IActiveScriptTraceInfo, StopScriptTracing)
        HRESULT ( STDMETHODCALLTYPE *StopScriptTracing )( 
            __RPC__in IActiveScriptTraceInfo * This);
        
        END_INTERFACE
    } IActiveScriptTraceInfoVtbl;

    interface IActiveScriptTraceInfo
    {
        CONST_VTBL struct IActiveScriptTraceInfoVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IActiveScriptTraceInfo_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IActiveScriptTraceInfo_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IActiveScriptTraceInfo_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IActiveScriptTraceInfo_StartScriptTracing(This,pSiteTraceInfo,guidContextID)	\
    ( (This)->lpVtbl -> StartScriptTracing(This,pSiteTraceInfo,guidContextID) ) 

#define IActiveScriptTraceInfo_StopScriptTracing(This)	\
    ( (This)->lpVtbl -> StopScriptTracing(This) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IActiveScriptTraceInfo_INTERFACE_DEFINED__ */


#ifndef __IActiveScriptStringCompare_INTERFACE_DEFINED__
#define __IActiveScriptStringCompare_INTERFACE_DEFINED__

/* interface IActiveScriptStringCompare */
/* [unique][uuid][object] */ 


EXTERN_C const IID IID_IActiveScriptStringCompare;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("58562769-ED52-42f7-8403-4963514E1F11")
    IActiveScriptStringCompare : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE StrComp( 
            /* [in] */ __RPC__in BSTR bszStr1,
            /* [in] */ __RPC__in BSTR bszStr2,
            /* [retval][out] */ __RPC__out LONG *iRet) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IActiveScriptStringCompareVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IActiveScriptStringCompare * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IActiveScriptStringCompare * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IActiveScriptStringCompare * This);
        
        DECLSPEC_XFGVIRT(IActiveScriptStringCompare, StrComp)
        HRESULT ( STDMETHODCALLTYPE *StrComp )( 
            __RPC__in IActiveScriptStringCompare * This,
            /* [in] */ __RPC__in BSTR bszStr1,
            /* [in] */ __RPC__in BSTR bszStr2,
            /* [retval][out] */ __RPC__out LONG *iRet);
        
        END_INTERFACE
    } IActiveScriptStringCompareVtbl;

    interface IActiveScriptStringCompare
    {
        CONST_VTBL struct IActiveScriptStringCompareVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IActiveScriptStringCompare_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IActiveScriptStringCompare_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IActiveScriptStringCompare_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IActiveScriptStringCompare_StrComp(This,bszStr1,bszStr2,iRet)	\
    ( (This)->lpVtbl -> StrComp(This,bszStr1,bszStr2,iRet) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IActiveScriptStringCompare_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_activscp_0000_0026 */
/* [local] */ 

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion

#endif  // __ActivScp_h



extern RPC_IF_HANDLE __MIDL_itf_activscp_0000_0026_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_activscp_0000_0026_v0_0_s_ifspec;

/* Additional Prototypes for ALL interfaces */

unsigned long             __RPC_USER  BSTR_UserSize(     __RPC__in unsigned long *, unsigned long            , __RPC__in BSTR * ); 
unsigned char * __RPC_USER  BSTR_UserMarshal(  __RPC__in unsigned long *, __RPC__inout_xcount(0) unsigned char *, __RPC__in BSTR * ); 
unsigned char * __RPC_USER  BSTR_UserUnmarshal(__RPC__in unsigned long *, __RPC__in_xcount(0) unsigned char *, __RPC__out BSTR * ); 
void                      __RPC_USER  BSTR_UserFree(     __RPC__in unsigned long *, __RPC__in BSTR * ); 

unsigned long             __RPC_USER  HWND_UserSize(     __RPC__in unsigned long *, unsigned long            , __RPC__in HWND * ); 
unsigned char * __RPC_USER  HWND_UserMarshal(  __RPC__in unsigned long *, __RPC__inout_xcount(0) unsigned char *, __RPC__in HWND * ); 
unsigned char * __RPC_USER  HWND_UserUnmarshal(__RPC__in unsigned long *, __RPC__in_xcount(0) unsigned char *, __RPC__out HWND * ); 
void                      __RPC_USER  HWND_UserFree(     __RPC__in unsigned long *, __RPC__in HWND * ); 

unsigned long             __RPC_USER  VARIANT_UserSize(     __RPC__in unsigned long *, unsigned long            , __RPC__in VARIANT * ); 
unsigned char * __RPC_USER  VARIANT_UserMarshal(  __RPC__in unsigned long *, __RPC__inout_xcount(0) unsigned char *, __RPC__in VARIANT * ); 
unsigned char * __RPC_USER  VARIANT_UserUnmarshal(__RPC__in unsigned long *, __RPC__in_xcount(0) unsigned char *, __RPC__out VARIANT * ); 
void                      __RPC_USER  VARIANT_UserFree(     __RPC__in unsigned long *, __RPC__in VARIANT * ); 

unsigned long             __RPC_USER  BSTR_UserSize64(     __RPC__in unsigned long *, unsigned long            , __RPC__in BSTR * ); 
unsigned char * __RPC_USER  BSTR_UserMarshal64(  __RPC__in unsigned long *, __RPC__inout_xcount(0) unsigned char *, __RPC__in BSTR * ); 
unsigned char * __RPC_USER  BSTR_UserUnmarshal64(__RPC__in unsigned long *, __RPC__in_xcount(0) unsigned char *, __RPC__out BSTR * ); 
void                      __RPC_USER  BSTR_UserFree64(     __RPC__in unsigned long *, __RPC__in BSTR * ); 

unsigned long             __RPC_USER  HWND_UserSize64(     __RPC__in unsigned long *, unsigned long            , __RPC__in HWND * ); 
unsigned char * __RPC_USER  HWND_UserMarshal64(  __RPC__in unsigned long *, __RPC__inout_xcount(0) unsigned char *, __RPC__in HWND * ); 
unsigned char * __RPC_USER  HWND_UserUnmarshal64(__RPC__in unsigned long *, __RPC__in_xcount(0) unsigned char *, __RPC__out HWND * ); 
void                      __RPC_USER  HWND_UserFree64(     __RPC__in unsigned long *, __RPC__in HWND * ); 

unsigned long             __RPC_USER  VARIANT_UserSize64(     __RPC__in unsigned long *, unsigned long            , __RPC__in VARIANT * ); 
unsigned char * __RPC_USER  VARIANT_UserMarshal64(  __RPC__in unsigned long *, __RPC__inout_xcount(0) unsigned char *, __RPC__in VARIANT * ); 
unsigned char * __RPC_USER  VARIANT_UserUnmarshal64(__RPC__in unsigned long *, __RPC__in_xcount(0) unsigned char *, __RPC__out VARIANT * ); 
void                      __RPC_USER  VARIANT_UserFree64(     __RPC__in unsigned long *, __RPC__in VARIANT * ); 

/* [local] */ HRESULT STDMETHODCALLTYPE IActiveScriptError_GetExceptionInfo_Proxy( 
    IActiveScriptError * This,
    /* [out] */ EXCEPINFO *pexcepinfo);


/* [call_as] */ HRESULT STDMETHODCALLTYPE IActiveScriptError_GetExceptionInfo_Stub( 
    __RPC__in IActiveScriptError * This,
    /* [out] */ __RPC__out EXCEPINFO *pexcepinfo);



/* end of Additional Prototypes */

#ifdef __cplusplus
}
#endif

#endif


