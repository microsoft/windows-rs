

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


#ifndef __wtypesbase_h__
#define __wtypesbase_h__

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
#include "basetsd.h"
#include "guiddef.h"

#ifdef __cplusplus
extern "C"{
#endif 


/* interface __MIDL_itf_wtypesbase_0000_0000 */
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
#if _MSC_VER >= 1200
#pragma warning(push)
#pragma warning(disable:4820) /* padding added after data member */
#endif


extern RPC_IF_HANDLE __MIDL_itf_wtypesbase_0000_0000_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_wtypesbase_0000_0000_v0_0_s_ifspec;

#ifndef __IWinTypesBase_INTERFACE_DEFINED__
#define __IWinTypesBase_INTERFACE_DEFINED__

/* interface IWinTypesBase */
/* [unique][version][uuid] */ 

#if 0
typedef byte BYTE;

typedef unsigned short WORD;

typedef unsigned int UINT;

typedef int INT;

typedef long BOOL;

typedef long LONG;

typedef unsigned long DWORD;

typedef void *HANDLE;

typedef WORD *LPWORD;

typedef DWORD *LPDWORD;

typedef char CHAR;

typedef /* [string] */  __RPC_string CHAR *LPSTR;

typedef /* [string] */  __RPC_string const CHAR *LPCSTR;

typedef wchar_t WCHAR;

typedef WCHAR TCHAR;

typedef /* [string] */  __RPC_string WCHAR *LPWSTR;

typedef /* [string] */  __RPC_string TCHAR *LPTSTR;

typedef /* [string] */  __RPC_string const WCHAR *LPCWSTR;

typedef /* [string] */  __RPC_string const TCHAR *LPCTSTR;

typedef HANDLE *LPHANDLE;

#endif // 0
#if defined(_WIN32) && !defined(OLE2ANSI)
typedef WCHAR OLECHAR;

typedef /* [string] */  __RPC_string OLECHAR *LPOLESTR;

typedef /* [string] */  __RPC_string const OLECHAR *LPCOLESTR;

#define OLESTR(str) L##str

#else

typedef char      OLECHAR;
typedef LPSTR     LPOLESTR;
typedef LPCSTR    LPCOLESTR;
#define OLESTR(str) str
#endif
#ifndef _WINDEF_
#ifndef _MINWINDEF_
typedef void *PVOID;

typedef void *LPVOID;

typedef float FLOAT;

#endif  //_MINWINDEF_
#endif  //_WINDEF_
typedef unsigned char UCHAR;

typedef short SHORT;

typedef unsigned short USHORT;

typedef DWORD ULONG;

typedef double DOUBLE;

#ifndef _DWORDLONG_
typedef unsigned __int64 DWORDLONG;

typedef DWORDLONG *PDWORDLONG;

#endif // !_DWORDLONG_
#ifndef _ULONGLONG_
typedef __int64 LONGLONG;

typedef unsigned __int64 ULONGLONG;

typedef LONGLONG *PLONGLONG;

typedef ULONGLONG *PULONGLONG;

#endif // _ULONGLONG_
#if 0
typedef struct _LARGE_INTEGER
    {
    LONGLONG QuadPart;
    } 	LARGE_INTEGER;

typedef LARGE_INTEGER *PLARGE_INTEGER;

typedef struct _ULARGE_INTEGER
    {
    ULONGLONG QuadPart;
    } 	ULARGE_INTEGER;

#endif // 0
#ifndef _WINBASE_
#ifndef _FILETIME_
#define _FILETIME_
typedef struct _FILETIME
    {
    DWORD dwLowDateTime;
    DWORD dwHighDateTime;
    } 	FILETIME;

typedef struct _FILETIME *PFILETIME;

typedef struct _FILETIME *LPFILETIME;

#endif // !_FILETIME
#ifndef _SYSTEMTIME_
#define _SYSTEMTIME_
typedef struct _SYSTEMTIME
    {
    WORD wYear;
    WORD wMonth;
    WORD wDayOfWeek;
    WORD wDay;
    WORD wHour;
    WORD wMinute;
    WORD wSecond;
    WORD wMilliseconds;
    } 	SYSTEMTIME;

typedef struct _SYSTEMTIME *PSYSTEMTIME;

typedef struct _SYSTEMTIME *LPSYSTEMTIME;

#endif // !_SYSTEMTIME
#ifndef _SECURITY_ATTRIBUTES_
#define _SECURITY_ATTRIBUTES_
typedef struct _SECURITY_ATTRIBUTES
    {
    DWORD nLength;
    LPVOID lpSecurityDescriptor;
    BOOL bInheritHandle;
    } 	SECURITY_ATTRIBUTES;

typedef struct _SECURITY_ATTRIBUTES *PSECURITY_ATTRIBUTES;

typedef struct _SECURITY_ATTRIBUTES *LPSECURITY_ATTRIBUTES;

#endif // !_SECURITY_ATTRIBUTES_
#ifndef SECURITY_DESCRIPTOR_REVISION
typedef USHORT SECURITY_DESCRIPTOR_CONTROL;

typedef USHORT *PSECURITY_DESCRIPTOR_CONTROL;

typedef PVOID PSID;

typedef struct _ACL
    {
    UCHAR AclRevision;
    UCHAR Sbz1;
    USHORT AclSize;
    USHORT AceCount;
    USHORT Sbz2;
    } 	ACL;

typedef ACL *PACL;

typedef struct _SECURITY_DESCRIPTOR
    {
    UCHAR Revision;
    UCHAR Sbz1;
    SECURITY_DESCRIPTOR_CONTROL Control;
    PSID Owner;
    PSID Group;
    PACL Sacl;
    PACL Dacl;
    } 	SECURITY_DESCRIPTOR;

typedef struct _SECURITY_DESCRIPTOR *PISECURITY_DESCRIPTOR;

#endif // !SECURITY_DESCRIPTOR_REVISION
#endif //_WINBASE_
typedef struct _COAUTHIDENTITY
    {
    /* [size_is] */ USHORT *User;
    /* [range] */ ULONG UserLength;
    /* [size_is] */ USHORT *Domain;
    /* [range] */ ULONG DomainLength;
    /* [size_is] */ USHORT *Password;
    /* [range] */ ULONG PasswordLength;
    ULONG Flags;
    } 	COAUTHIDENTITY;

typedef struct _COAUTHINFO
    {
    DWORD dwAuthnSvc;
    DWORD dwAuthzSvc;
    LPWSTR pwszServerPrincName;
    DWORD dwAuthnLevel;
    DWORD dwImpersonationLevel;
    COAUTHIDENTITY *pAuthIdentityData;
    DWORD dwCapabilities;
    } 	COAUTHINFO;

typedef LONG SCODE;

typedef SCODE *PSCODE;

#ifndef _HRESULT_DEFINED
#define _HRESULT_DEFINED
#ifdef __midl
typedef LONG HRESULT;

#else // __midl
typedef _Return_type_success_(return >= 0) LONG HRESULT;
#endif // __midl
#endif // !_HRESULT_DEFINED

#ifndef __OBJECTID_DEFINED
#define __OBJECTID_DEFINED
#define _OBJECTID_DEFINED
typedef struct _OBJECTID
    {
    GUID Lineage;
    ULONG Uniquifier;
    } 	OBJECTID;

#endif // !_OBJECTID_DEFINED
#if 0
typedef GUID *REFGUID;

typedef IID *REFIID;

typedef CLSID *REFCLSID;

#endif // 0
typedef 
enum tagMEMCTX
    {
        MEMCTX_TASK	= 1,
        MEMCTX_SHARED	= 2,
        MEMCTX_MACSYSTEM	= 3,
        MEMCTX_UNKNOWN	= -1,
        MEMCTX_SAME	= -2
    } 	MEMCTX;

#ifndef _ROTREGFLAGS_DEFINED
#define _ROTREGFLAGS_DEFINED
#define ROTREGFLAGS_ALLOWANYCLIENT 0x1
#endif // !_ROTREGFLAGS_DEFINED
#ifndef _APPIDREGFLAGS_DEFINED
#define _APPIDREGFLAGS_DEFINED
#define APPIDREGFLAGS_ACTIVATE_IUSERVER_INDESKTOP 0x1
#define APPIDREGFLAGS_SECURE_SERVER_PROCESS_SD_AND_BIND 0x2
#define APPIDREGFLAGS_ISSUE_ACTIVATION_RPC_AT_IDENTIFY 0x4
#define APPIDREGFLAGS_IUSERVER_UNMODIFIED_LOGON_TOKEN 0x8
#define APPIDREGFLAGS_IUSERVER_SELF_SID_IN_LAUNCH_PERMISSION 0x10
#define APPIDREGFLAGS_IUSERVER_ACTIVATE_IN_CLIENT_SESSION_ONLY 0x20
#define APPIDREGFLAGS_RESERVED1 0x40
#define APPIDREGFLAGS_RESERVED2 0x80
#define APPIDREGFLAGS_RESERVED3 0x100
#define APPIDREGFLAGS_RESERVED4 0x200
#define APPIDREGFLAGS_RESERVED5 0x400
#define APPIDREGFLAGS_AAA_NO_IMPLICIT_ACTIVATE_AS_IU 0x800
#define APPIDREGFLAGS_RESERVED7 0x1000
#define APPIDREGFLAGS_RESERVED8 0x2000
#define APPIDREGFLAGS_RESERVED9 0x4000
#define APPIDREGFLAGS_RESERVED10 0x8000
#endif // !_APPIDREGFLAGS_DEFINED
#ifndef _DCOMSCM_REMOTECALL_FLAGS_DEFINED
#define _DCOMSCM_REMOTECALL_FLAGS_DEFINED
#define DCOMSCM_ACTIVATION_USE_ALL_AUTHNSERVICES 0x1
#define DCOMSCM_ACTIVATION_DISALLOW_UNSECURE_CALL 0x2
#define DCOMSCM_RESOLVE_USE_ALL_AUTHNSERVICES 0x4
#define DCOMSCM_RESOLVE_DISALLOW_UNSECURE_CALL 0x8
#define DCOMSCM_PING_USE_MID_AUTHNSERVICE 0x10
#define DCOMSCM_PING_DISALLOW_UNSECURE_CALL 0x20
#endif // !_DCOMSCM_REMOTECALL_FLAGS_DEFINED
typedef 
enum tagCLSCTX
    {
        CLSCTX_INPROC_SERVER	= 0x1,
        CLSCTX_INPROC_HANDLER	= 0x2,
        CLSCTX_LOCAL_SERVER	= 0x4,
        CLSCTX_INPROC_SERVER16	= 0x8,
        CLSCTX_REMOTE_SERVER	= 0x10,
        CLSCTX_INPROC_HANDLER16	= 0x20,
        CLSCTX_RESERVED1	= 0x40,
        CLSCTX_RESERVED2	= 0x80,
        CLSCTX_RESERVED3	= 0x100,
        CLSCTX_RESERVED4	= 0x200,
        CLSCTX_NO_CODE_DOWNLOAD	= 0x400,
        CLSCTX_RESERVED5	= 0x800,
        CLSCTX_NO_CUSTOM_MARSHAL	= 0x1000,
        CLSCTX_ENABLE_CODE_DOWNLOAD	= 0x2000,
        CLSCTX_NO_FAILURE_LOG	= 0x4000,
        CLSCTX_DISABLE_AAA	= 0x8000,
        CLSCTX_ENABLE_AAA	= 0x10000,
        CLSCTX_FROM_DEFAULT_CONTEXT	= 0x20000,
        CLSCTX_ACTIVATE_X86_SERVER	= 0x40000,
        CLSCTX_ACTIVATE_32_BIT_SERVER	= CLSCTX_ACTIVATE_X86_SERVER,
        CLSCTX_ACTIVATE_64_BIT_SERVER	= 0x80000,
        CLSCTX_ENABLE_CLOAKING	= 0x100000,
        CLSCTX_APPCONTAINER	= 0x400000,
        CLSCTX_ACTIVATE_AAA_AS_IU	= 0x800000,
        CLSCTX_RESERVED6	= 0x1000000,
        CLSCTX_ACTIVATE_ARM32_SERVER	= 0x2000000,
        CLSCTX_ALLOW_LOWER_TRUST_REGISTRATION	= 0x4000000,
        CLSCTX_SERVER_MUST_BE_EQUAL_OR_GREATER_PRIVILEGE	= 0x8000000,
        CLSCTX_DO_NOT_ELEVATE_SERVER	= 0x10000000,
        CLSCTX_PS_DLL	= 0x80000000
    } 	CLSCTX;

DEFINE_ENUM_FLAG_OPERATORS(CLSCTX);
#define CLSCTX_VALID_MASK \
   (CLSCTX_INPROC_SERVER | \
    CLSCTX_INPROC_HANDLER | \
    CLSCTX_LOCAL_SERVER | \
    CLSCTX_INPROC_SERVER16 | \
    CLSCTX_REMOTE_SERVER | \
    CLSCTX_NO_CODE_DOWNLOAD | \
    CLSCTX_NO_CUSTOM_MARSHAL | \
    CLSCTX_ENABLE_CODE_DOWNLOAD | \
    CLSCTX_NO_FAILURE_LOG | \
    CLSCTX_DISABLE_AAA | \
    CLSCTX_ENABLE_AAA | \
    CLSCTX_FROM_DEFAULT_CONTEXT | \
    CLSCTX_ACTIVATE_X86_SERVER | \
    CLSCTX_ACTIVATE_64_BIT_SERVER | \
    CLSCTX_ENABLE_CLOAKING | \
    CLSCTX_APPCONTAINER | \
    CLSCTX_ACTIVATE_AAA_AS_IU | \
    CLSCTX_RESERVED6 | \
    CLSCTX_ACTIVATE_ARM32_SERVER | \
    CLSCTX_ALLOW_LOWER_TRUST_REGISTRATION | \
    CLSCTX_DO_NOT_ELEVATE_SERVER | \
    CLSCTX_SERVER_MUST_BE_EQUAL_OR_GREATER_PRIVILEGE | \
    CLSCTX_PS_DLL)
typedef 
enum tagMSHLFLAGS
    {
        MSHLFLAGS_NORMAL	= 0,
        MSHLFLAGS_TABLESTRONG	= 1,
        MSHLFLAGS_TABLEWEAK	= 2,
        MSHLFLAGS_NOPING	= 4,
        MSHLFLAGS_RESERVED1	= 8,
        MSHLFLAGS_RESERVED2	= 16,
        MSHLFLAGS_RESERVED3	= 32,
        MSHLFLAGS_RESERVED4	= 64
    } 	MSHLFLAGS;

typedef 
enum tagMSHCTX
    {
        MSHCTX_LOCAL	= 0,
        MSHCTX_NOSHAREDMEM	= 1,
        MSHCTX_DIFFERENTMACHINE	= 2,
        MSHCTX_INPROC	= 3,
        MSHCTX_CROSSCTX	= 4,
        MSHCTX_CONTAINER	= 5
    } 	MSHCTX;

typedef struct _BYTE_BLOB
    {
    ULONG clSize;
    /* [size_is] */ byte abData[ 1 ];
    } 	BYTE_BLOB;

typedef /* [unique] */  __RPC_unique_pointer BYTE_BLOB *UP_BYTE_BLOB;

typedef struct _WORD_BLOB
    {
    ULONG clSize;
    /* [size_is] */ unsigned short asData[ 1 ];
    } 	WORD_BLOB;

typedef /* [unique] */  __RPC_unique_pointer WORD_BLOB *UP_WORD_BLOB;

typedef struct _DWORD_BLOB
    {
    ULONG clSize;
    /* [size_is] */ ULONG alData[ 1 ];
    } 	DWORD_BLOB;

typedef /* [unique] */  __RPC_unique_pointer DWORD_BLOB *UP_DWORD_BLOB;

typedef struct _FLAGGED_BYTE_BLOB
    {
    ULONG fFlags;
    ULONG clSize;
    /* [size_is] */ byte abData[ 1 ];
    } 	FLAGGED_BYTE_BLOB;

typedef /* [unique] */  __RPC_unique_pointer FLAGGED_BYTE_BLOB *UP_FLAGGED_BYTE_BLOB;

typedef struct _FLAGGED_WORD_BLOB
    {
    ULONG fFlags;
    ULONG clSize;
    /* [size_is] */ unsigned short asData[ 1 ];
    } 	FLAGGED_WORD_BLOB;

typedef /* [unique] */  __RPC_unique_pointer FLAGGED_WORD_BLOB *UP_FLAGGED_WORD_BLOB;

typedef struct _BYTE_SIZEDARR
    {
    ULONG clSize;
    /* [size_is] */ byte *pData;
    } 	BYTE_SIZEDARR;

typedef struct _SHORT_SIZEDARR
    {
    ULONG clSize;
    /* [size_is] */ unsigned short *pData;
    } 	WORD_SIZEDARR;

typedef struct _LONG_SIZEDARR
    {
    ULONG clSize;
    /* [size_is] */ ULONG *pData;
    } 	DWORD_SIZEDARR;

typedef struct _HYPER_SIZEDARR
    {
    ULONG clSize;
    /* [size_is] */ hyper *pData;
    } 	HYPER_SIZEDARR;



extern RPC_IF_HANDLE IWinTypesBase_v0_1_c_ifspec;
extern RPC_IF_HANDLE IWinTypesBase_v0_1_s_ifspec;
#endif /* __IWinTypesBase_INTERFACE_DEFINED__ */

/* interface __MIDL_itf_wtypesbase_0000_0001 */
/* [local] */ 

typedef boolean BOOLEAN;

#ifndef _tagBLOB_DEFINED
#define _tagBLOB_DEFINED
#define _BLOB_DEFINED
#define _LPBLOB_DEFINED
typedef struct tagBLOB
    {
    ULONG cbSize;
    /* [size_is] */ BYTE *pBlobData;
    } 	BLOB;

typedef struct tagBLOB *LPBLOB;

#endif
#ifndef SID_IDENTIFIER_AUTHORITY_DEFINED
#define SID_IDENTIFIER_AUTHORITY_DEFINED
typedef struct _SID_IDENTIFIER_AUTHORITY
    {
    UCHAR Value[ 6 ];
    } 	SID_IDENTIFIER_AUTHORITY;

typedef struct _SID_IDENTIFIER_AUTHORITY *PSID_IDENTIFIER_AUTHORITY;

#endif
#ifndef SID_DEFINED
#define SID_DEFINED
typedef struct _SID
    {
    BYTE Revision;
    BYTE SubAuthorityCount;
    SID_IDENTIFIER_AUTHORITY IdentifierAuthority;
    /* [size_is] */ ULONG SubAuthority[ 1 ];
    } 	SID;

typedef struct _SID *PISID;

typedef struct _SID_AND_ATTRIBUTES
    {
    SID *Sid;
    DWORD Attributes;
    } 	SID_AND_ATTRIBUTES;

typedef struct _SID_AND_ATTRIBUTES *PSID_AND_ATTRIBUTES;

#endif
#if _MSC_VER >= 1200
#pragma warning(pop)
#endif


extern RPC_IF_HANDLE __MIDL_itf_wtypesbase_0000_0001_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_wtypesbase_0000_0001_v0_0_s_ifspec;

/* Additional Prototypes for ALL interfaces */

/* end of Additional Prototypes */

#ifdef __cplusplus
}
#endif

#endif


