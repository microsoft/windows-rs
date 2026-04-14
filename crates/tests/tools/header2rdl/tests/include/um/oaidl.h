

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

#ifndef COM_NO_WINDOWS_H
#include "windows.h"
#include "ole2.h"
#endif /*COM_NO_WINDOWS_H*/

#ifndef __oaidl_h__
#define __oaidl_h__

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

#ifndef __ICreateTypeInfo_FWD_DEFINED__
#define __ICreateTypeInfo_FWD_DEFINED__
typedef interface ICreateTypeInfo ICreateTypeInfo;

#endif 	/* __ICreateTypeInfo_FWD_DEFINED__ */


#ifndef __ICreateTypeInfo2_FWD_DEFINED__
#define __ICreateTypeInfo2_FWD_DEFINED__
typedef interface ICreateTypeInfo2 ICreateTypeInfo2;

#endif 	/* __ICreateTypeInfo2_FWD_DEFINED__ */


#ifndef __ICreateTypeLib_FWD_DEFINED__
#define __ICreateTypeLib_FWD_DEFINED__
typedef interface ICreateTypeLib ICreateTypeLib;

#endif 	/* __ICreateTypeLib_FWD_DEFINED__ */


#ifndef __ICreateTypeLib2_FWD_DEFINED__
#define __ICreateTypeLib2_FWD_DEFINED__
typedef interface ICreateTypeLib2 ICreateTypeLib2;

#endif 	/* __ICreateTypeLib2_FWD_DEFINED__ */


#ifndef __IDispatch_FWD_DEFINED__
#define __IDispatch_FWD_DEFINED__
typedef interface IDispatch IDispatch;

#endif 	/* __IDispatch_FWD_DEFINED__ */


#ifndef __IEnumVARIANT_FWD_DEFINED__
#define __IEnumVARIANT_FWD_DEFINED__
typedef interface IEnumVARIANT IEnumVARIANT;

#endif 	/* __IEnumVARIANT_FWD_DEFINED__ */


#ifndef __ITypeComp_FWD_DEFINED__
#define __ITypeComp_FWD_DEFINED__
typedef interface ITypeComp ITypeComp;

#endif 	/* __ITypeComp_FWD_DEFINED__ */


#ifndef __ITypeInfo_FWD_DEFINED__
#define __ITypeInfo_FWD_DEFINED__
typedef interface ITypeInfo ITypeInfo;

#endif 	/* __ITypeInfo_FWD_DEFINED__ */


#ifndef __ITypeInfo2_FWD_DEFINED__
#define __ITypeInfo2_FWD_DEFINED__
typedef interface ITypeInfo2 ITypeInfo2;

#endif 	/* __ITypeInfo2_FWD_DEFINED__ */


#ifndef __ITypeLib_FWD_DEFINED__
#define __ITypeLib_FWD_DEFINED__
typedef interface ITypeLib ITypeLib;

#endif 	/* __ITypeLib_FWD_DEFINED__ */


#ifndef __ITypeLib2_FWD_DEFINED__
#define __ITypeLib2_FWD_DEFINED__
typedef interface ITypeLib2 ITypeLib2;

#endif 	/* __ITypeLib2_FWD_DEFINED__ */


#ifndef __ITypeChangeEvents_FWD_DEFINED__
#define __ITypeChangeEvents_FWD_DEFINED__
typedef interface ITypeChangeEvents ITypeChangeEvents;

#endif 	/* __ITypeChangeEvents_FWD_DEFINED__ */


#ifndef __IErrorInfo_FWD_DEFINED__
#define __IErrorInfo_FWD_DEFINED__
typedef interface IErrorInfo IErrorInfo;

#endif 	/* __IErrorInfo_FWD_DEFINED__ */


#ifndef __ICreateErrorInfo_FWD_DEFINED__
#define __ICreateErrorInfo_FWD_DEFINED__
typedef interface ICreateErrorInfo ICreateErrorInfo;

#endif 	/* __ICreateErrorInfo_FWD_DEFINED__ */


#ifndef __ISupportErrorInfo_FWD_DEFINED__
#define __ISupportErrorInfo_FWD_DEFINED__
typedef interface ISupportErrorInfo ISupportErrorInfo;

#endif 	/* __ISupportErrorInfo_FWD_DEFINED__ */


#ifndef __ITypeFactory_FWD_DEFINED__
#define __ITypeFactory_FWD_DEFINED__
typedef interface ITypeFactory ITypeFactory;

#endif 	/* __ITypeFactory_FWD_DEFINED__ */


#ifndef __ITypeMarshal_FWD_DEFINED__
#define __ITypeMarshal_FWD_DEFINED__
typedef interface ITypeMarshal ITypeMarshal;

#endif 	/* __ITypeMarshal_FWD_DEFINED__ */


#ifndef __IRecordInfo_FWD_DEFINED__
#define __IRecordInfo_FWD_DEFINED__
typedef interface IRecordInfo IRecordInfo;

#endif 	/* __IRecordInfo_FWD_DEFINED__ */


#ifndef __IErrorLog_FWD_DEFINED__
#define __IErrorLog_FWD_DEFINED__
typedef interface IErrorLog IErrorLog;

#endif 	/* __IErrorLog_FWD_DEFINED__ */


#ifndef __IPropertyBag_FWD_DEFINED__
#define __IPropertyBag_FWD_DEFINED__
typedef interface IPropertyBag IPropertyBag;

#endif 	/* __IPropertyBag_FWD_DEFINED__ */


#ifndef __ITypeLibRegistrationReader_FWD_DEFINED__
#define __ITypeLibRegistrationReader_FWD_DEFINED__
typedef interface ITypeLibRegistrationReader ITypeLibRegistrationReader;

#endif 	/* __ITypeLibRegistrationReader_FWD_DEFINED__ */


#ifndef __ITypeLibRegistration_FWD_DEFINED__
#define __ITypeLibRegistration_FWD_DEFINED__
typedef interface ITypeLibRegistration ITypeLibRegistration;

#endif 	/* __ITypeLibRegistration_FWD_DEFINED__ */


/* header files for imported files */
#include "objidl.h"

#ifdef __cplusplus
extern "C"{
#endif 


/* interface __MIDL_itf_oaidl_0000_0000 */
/* [local] */ 

#include <winapifamily.h>
//+-------------------------------------------------------------------------
//
//  Microsoft Windows
//  Copyright (c) Microsoft Corporation. All rights reserved.
//
//--------------------------------------------------------------------------
#if ( _MSC_VER >= 800 )
#if _MSC_VER >= 1200
#pragma warning(push)
#pragma warning(disable:4820) /* padding added after data member */
#endif
#pragma warning(disable:4201)    /* Nameless struct/union */
#endif
#if ( _MSC_VER >= 1020 )
#pragma once
#endif
#pragma region Desktop Family or OneCore Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM)




#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM) */
#pragma endregion
#pragma region Application Family or OneCore Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM)



#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM) */
#pragma endregion
#pragma region Desktop Family or OneCore Family or OneCore Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM)











#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM) */
#pragma endregion
#pragma region Application Family or OneCore Family or Games Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES)


extern RPC_IF_HANDLE __MIDL_itf_oaidl_0000_0000_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_oaidl_0000_0000_v0_0_s_ifspec;

#ifndef __IOleAutomationTypes_INTERFACE_DEFINED__
#define __IOleAutomationTypes_INTERFACE_DEFINED__

/* interface IOleAutomationTypes */
/* [unique][version] */ 

typedef CY CURRENCY;

typedef struct tagSAFEARRAYBOUND
    {
    ULONG cElements;
    LONG lLbound;
    } 	SAFEARRAYBOUND;

typedef struct tagSAFEARRAYBOUND *LPSAFEARRAYBOUND;

/* the following is what MIDL knows how to remote */
typedef /* [unique] */  __RPC_unique_pointer struct _wireVARIANT *wireVARIANT;

typedef /* [unique] */  __RPC_unique_pointer struct _wireBRECORD *wireBRECORD;

typedef struct _wireSAFEARR_BSTR
    {
    ULONG Size;
    /* [ref][size_is] */ wireBSTR *aBstr;
    } 	SAFEARR_BSTR;

typedef struct _wireSAFEARR_UNKNOWN
    {
    ULONG Size;
    /* [ref][size_is] */ IUnknown **apUnknown;
    } 	SAFEARR_UNKNOWN;

typedef struct _wireSAFEARR_DISPATCH
    {
    ULONG Size;
    /* [ref][size_is] */ IDispatch **apDispatch;
    } 	SAFEARR_DISPATCH;

typedef struct _wireSAFEARR_VARIANT
    {
    ULONG Size;
    /* [ref][size_is] */ wireVARIANT *aVariant;
    } 	SAFEARR_VARIANT;

typedef struct _wireSAFEARR_BRECORD
    {
    ULONG Size;
    /* [ref][size_is] */ wireBRECORD *aRecord;
    } 	SAFEARR_BRECORD;

typedef struct _wireSAFEARR_HAVEIID
    {
    ULONG Size;
    /* [ref][size_is] */ IUnknown **apUnknown;
    IID iid;
    } 	SAFEARR_HAVEIID;

typedef /* [v1_enum] */ 
enum tagSF_TYPE
    {
        SF_ERROR	= VT_ERROR,
        SF_I1	= VT_I1,
        SF_I2	= VT_I2,
        SF_I4	= VT_I4,
        SF_I8	= VT_I8,
        SF_BSTR	= VT_BSTR,
        SF_UNKNOWN	= VT_UNKNOWN,
        SF_DISPATCH	= VT_DISPATCH,
        SF_VARIANT	= VT_VARIANT,
        SF_RECORD	= VT_RECORD,
        SF_HAVEIID	= ( VT_UNKNOWN | VT_RESERVED ) 
    } 	SF_TYPE;

typedef struct _wireSAFEARRAY_UNION
    {
    ULONG sfType;
    /* [switch_is] */ /* [switch_type] */ union __MIDL_IOleAutomationTypes_0001
        {
        /* [case()] */ SAFEARR_BSTR BstrStr;
        /* [case()] */ SAFEARR_UNKNOWN UnknownStr;
        /* [case()] */ SAFEARR_DISPATCH DispatchStr;
        /* [case()] */ SAFEARR_VARIANT VariantStr;
        /* [case()] */ SAFEARR_BRECORD RecordStr;
        /* [case()] */ SAFEARR_HAVEIID HaveIidStr;
        /* [case()] */ BYTE_SIZEDARR ByteStr;
        /* [case()] */ WORD_SIZEDARR WordStr;
        /* [case()] */ DWORD_SIZEDARR LongStr;
        /* [case()] */ HYPER_SIZEDARR HyperStr;
        } 	u;
    } 	SAFEARRAYUNION;

typedef /* [unique] */  __RPC_unique_pointer struct _wireSAFEARRAY
    {
    USHORT cDims;
    USHORT fFeatures;
    ULONG cbElements;
    ULONG cLocks;
    SAFEARRAYUNION uArrayStructs;
    /* [size_is] */ SAFEARRAYBOUND rgsabound[ 1 ];
    } 	*wireSAFEARRAY;

typedef /* [unique] */  __RPC_unique_pointer wireSAFEARRAY *wirePSAFEARRAY;

typedef struct tagSAFEARRAY
    {
    USHORT cDims;
    USHORT fFeatures;
    ULONG cbElements;
    ULONG cLocks;
    PVOID pvData;
    SAFEARRAYBOUND rgsabound[ 1 ];
    } 	SAFEARRAY;

typedef /* [wire_marshal] */ SAFEARRAY *LPSAFEARRAY;

#define	FADF_AUTO	( 0x1 )

#define	FADF_STATIC	( 0x2 )

#define	FADF_EMBEDDED	( 0x4 )

#define	FADF_FIXEDSIZE	( 0x10 )

#define	FADF_RECORD	( 0x20 )

#define	FADF_HAVEIID	( 0x40 )

#define	FADF_HAVEVARTYPE	( 0x80 )

#define	FADF_BSTR	( 0x100 )

#define	FADF_UNKNOWN	( 0x200 )

#define	FADF_DISPATCH	( 0x400 )

#define	FADF_VARIANT	( 0x800 )

#define	FADF_RESERVED	( 0xf008 )

/* VARIANT STRUCTURE
 *
 *  VARTYPE vt;
 *  WORD wReserved1;
 *  WORD wReserved2;
 *  WORD wReserved3;
 *  union {
 *    LONGLONG       VT_I8
 *    LONG           VT_I4
 *    BYTE           VT_UI1
 *    SHORT          VT_I2
 *    FLOAT          VT_R4
 *    DOUBLE         VT_R8
 *    VARIANT_BOOL   VT_BOOL
 *    SCODE          VT_ERROR
 *    CY             VT_CY
 *    DATE           VT_DATE
 *    BSTR           VT_BSTR
 *    IUnknown *     VT_UNKNOWN
 *    IDispatch *    VT_DISPATCH
 *    SAFEARRAY *    VT_ARRAY
 *    BYTE *         VT_BYREF|VT_UI1
 *    SHORT *        VT_BYREF|VT_I2
 *    LONG *         VT_BYREF|VT_I4
 *    LONGLONG *     VT_BYREF|VT_I8
 *    FLOAT *        VT_BYREF|VT_R4
 *    DOUBLE *       VT_BYREF|VT_R8
 *    VARIANT_BOOL * VT_BYREF|VT_BOOL
 *    SCODE *        VT_BYREF|VT_ERROR
 *    CY *           VT_BYREF|VT_CY
 *    DATE *         VT_BYREF|VT_DATE
 *    BSTR *         VT_BYREF|VT_BSTR
 *    IUnknown **    VT_BYREF|VT_UNKNOWN
 *    IDispatch **   VT_BYREF|VT_DISPATCH
 *    SAFEARRAY **   VT_BYREF|VT_ARRAY
 *    VARIANT *      VT_BYREF|VT_VARIANT
 *    PVOID          VT_BYREF (Generic ByRef)
 *    CHAR           VT_I1
 *    USHORT         VT_UI2
 *    ULONG          VT_UI4
 *    ULONGLONG      VT_UI8
 *    INT            VT_INT
 *    UINT           VT_UINT
 *    DECIMAL *      VT_BYREF|VT_DECIMAL
 *    CHAR *         VT_BYREF|VT_I1
 *    USHORT *       VT_BYREF|VT_UI2
 *    ULONG *        VT_BYREF|VT_UI4
 *    ULONGLONG *    VT_BYREF|VT_UI8
 *    INT *          VT_BYREF|VT_INT
 *    UINT *         VT_BYREF|VT_UINT
 *  }
 */
#if (__STDC__ && !defined(_FORCENAMELESSUNION)) || defined(NONAMELESSUNION) || (!defined(_MSC_EXTENSIONS) && !defined(_FORCENAMELESSUNION))
#define __VARIANT_NAME_1 n1
#define __VARIANT_NAME_2 n2
#define __VARIANT_NAME_3 n3
#define __VARIANT_NAME_4 brecVal
#else
#define __tagVARIANT
#define __VARIANT_NAME_1
#define __VARIANT_NAME_2
#define __VARIANT_NAME_3
#define __tagBRECORD
#define __VARIANT_NAME_4
#endif
typedef /* [wire_marshal] */ struct tagVARIANT VARIANT;

struct tagVARIANT
    {
    union 
        {
        struct __tagVARIANT
            {
            VARTYPE vt;
            WORD wReserved1;
            WORD wReserved2;
            WORD wReserved3;
            union 
                {
                LONGLONG llVal;
                LONG lVal;
                BYTE bVal;
                SHORT iVal;
                FLOAT fltVal;
                DOUBLE dblVal;
                VARIANT_BOOL boolVal;
                VARIANT_BOOL __OBSOLETE__VARIANT_BOOL;
                SCODE scode;
                CY cyVal;
                DATE date;
                BSTR bstrVal;
                IUnknown *punkVal;
                IDispatch *pdispVal;
                SAFEARRAY *parray;
                BYTE *pbVal;
                SHORT *piVal;
                LONG *plVal;
                LONGLONG *pllVal;
                FLOAT *pfltVal;
                DOUBLE *pdblVal;
                VARIANT_BOOL *pboolVal;
                VARIANT_BOOL *__OBSOLETE__VARIANT_PBOOL;
                SCODE *pscode;
                CY *pcyVal;
                DATE *pdate;
                BSTR *pbstrVal;
                IUnknown **ppunkVal;
                IDispatch **ppdispVal;
                SAFEARRAY **pparray;
                VARIANT *pvarVal;
                PVOID byref;
                CHAR cVal;
                USHORT uiVal;
                ULONG ulVal;
                ULONGLONG ullVal;
                INT intVal;
                UINT uintVal;
                DECIMAL *pdecVal;
                CHAR *pcVal;
                USHORT *puiVal;
                ULONG *pulVal;
                ULONGLONG *pullVal;
                INT *pintVal;
                UINT *puintVal;
                struct __tagBRECORD
                    {
                    PVOID pvRecord;
                    IRecordInfo *pRecInfo;
                    } 	__VARIANT_NAME_4;
                } 	__VARIANT_NAME_3;
            } 	__VARIANT_NAME_2;
        DECIMAL decVal;
        } 	__VARIANT_NAME_1;
    } ;
typedef VARIANT *LPVARIANT;

typedef VARIANT VARIANTARG;

typedef VARIANT *LPVARIANTARG;

#ifdef MIDL_PASS
typedef const VARIANT *REFVARIANT;

#else

#ifndef _REFVARIANT_DEFINED
#define _REFVARIANT_DEFINED
#ifdef __cplusplus
#define REFVARIANT const VARIANT &
#else
#define REFVARIANT const VARIANT * __MIDL_CONST
#endif
#endif

#endif // MIDL_PASS

/* the following is what MIDL knows how to remote */
struct _wireBRECORD
    {
    ULONG fFlags;
    ULONG clSize;
    IRecordInfo *pRecInfo;
    /* [size_is] */ byte *pRecord;
    } ;
struct _wireVARIANT
    {
    DWORD clSize;
    DWORD rpcReserved;
    USHORT vt;
    USHORT wReserved1;
    USHORT wReserved2;
    USHORT wReserved3;
    /* [switch_is][switch_type] */ union 
        {
        /* [case()] */ LONGLONG llVal;
        /* [case()] */ LONG lVal;
        /* [case()] */ BYTE bVal;
        /* [case()] */ SHORT iVal;
        /* [case()] */ FLOAT fltVal;
        /* [case()] */ DOUBLE dblVal;
        /* [case()] */ VARIANT_BOOL boolVal;
        /* [case()] */ SCODE scode;
        /* [case()] */ CY cyVal;
        /* [case()] */ DATE date;
        /* [case()] */ wireBSTR bstrVal;
        /* [case()] */ IUnknown *punkVal;
        /* [case()] */ IDispatch *pdispVal;
        /* [case()] */ wirePSAFEARRAY parray;
        /* [case()] */ wireBRECORD brecVal;
        /* [case()] */ BYTE *pbVal;
        /* [case()] */ SHORT *piVal;
        /* [case()] */ LONG *plVal;
        /* [case()] */ LONGLONG *pllVal;
        /* [case()] */ FLOAT *pfltVal;
        /* [case()] */ DOUBLE *pdblVal;
        /* [case()] */ VARIANT_BOOL *pboolVal;
        /* [case()] */ SCODE *pscode;
        /* [case()] */ CY *pcyVal;
        /* [case()] */ DATE *pdate;
        /* [case()] */ wireBSTR *pbstrVal;
        /* [case()] */ IUnknown **ppunkVal;
        /* [case()] */ IDispatch **ppdispVal;
        /* [case()] */ wirePSAFEARRAY *pparray;
        /* [case()] */ wireVARIANT *pvarVal;
        /* [case()] */ CHAR cVal;
        /* [case()] */ USHORT uiVal;
        /* [case()] */ ULONG ulVal;
        /* [case()] */ ULONGLONG ullVal;
        /* [case()] */ INT intVal;
        /* [case()] */ UINT uintVal;
        /* [case()] */ DECIMAL decVal;
        /* [case()] */ DECIMAL *pdecVal;
        /* [case()] */ CHAR *pcVal;
        /* [case()] */ USHORT *puiVal;
        /* [case()] */ ULONG *pulVal;
        /* [case()] */ ULONGLONG *pullVal;
        /* [case()] */ INT *pintVal;
        /* [case()] */ UINT *puintVal;
        /* [case()] */  /* Empty union arm */ 
        /* [case()] */  /* Empty union arm */ 
        } 	DUMMYUNIONNAME;
    } ;
typedef LONG DISPID;

typedef DISPID MEMBERID;

typedef DWORD HREFTYPE;

typedef /* [v1_enum] */ 
enum tagTYPEKIND
    {
        TKIND_ENUM	= 0,
        TKIND_RECORD	= ( TKIND_ENUM + 1 ) ,
        TKIND_MODULE	= ( TKIND_RECORD + 1 ) ,
        TKIND_INTERFACE	= ( TKIND_MODULE + 1 ) ,
        TKIND_DISPATCH	= ( TKIND_INTERFACE + 1 ) ,
        TKIND_COCLASS	= ( TKIND_DISPATCH + 1 ) ,
        TKIND_ALIAS	= ( TKIND_COCLASS + 1 ) ,
        TKIND_UNION	= ( TKIND_ALIAS + 1 ) ,
        TKIND_MAX	= ( TKIND_UNION + 1 ) 
    } 	TYPEKIND;

typedef struct tagTYPEDESC
    {
    /* [switch_is][switch_type] */ union 
        {
        /* [case()] */ struct tagTYPEDESC *lptdesc;
        /* [case()] */ struct tagARRAYDESC *lpadesc;
        /* [case()] */ HREFTYPE hreftype;
        /* [default] */  /* Empty union arm */ 
        } 	DUMMYUNIONNAME;
    VARTYPE vt;
    } 	TYPEDESC;

typedef struct tagARRAYDESC
    {
    TYPEDESC tdescElem;
    USHORT cDims;
    /* [size_is] */ SAFEARRAYBOUND rgbounds[ 1 ];
    } 	ARRAYDESC;

typedef struct tagPARAMDESCEX
    {
    ULONG cBytes;
    VARIANTARG varDefaultValue;
    } 	PARAMDESCEX;

typedef struct tagPARAMDESCEX *LPPARAMDESCEX;

typedef struct tagPARAMDESC
    {
    LPPARAMDESCEX pparamdescex;
    USHORT wParamFlags;
    } 	PARAMDESC;

typedef struct tagPARAMDESC *LPPARAMDESC;

#define	PARAMFLAG_NONE	( 0 )

#define	PARAMFLAG_FIN	( 0x1 )

#define	PARAMFLAG_FOUT	( 0x2 )

#define	PARAMFLAG_FLCID	( 0x4 )

#define	PARAMFLAG_FRETVAL	( 0x8 )

#define	PARAMFLAG_FOPT	( 0x10 )

#define	PARAMFLAG_FHASDEFAULT	( 0x20 )

#define	PARAMFLAG_FHASCUSTDATA	( 0x40 )

typedef struct tagIDLDESC
    {
    ULONG_PTR dwReserved;
    USHORT wIDLFlags;
    } 	IDLDESC;

typedef struct tagIDLDESC *LPIDLDESC;

#define	IDLFLAG_NONE	( PARAMFLAG_NONE )

#define	IDLFLAG_FIN	( PARAMFLAG_FIN )

#define	IDLFLAG_FOUT	( PARAMFLAG_FOUT )

#define	IDLFLAG_FLCID	( PARAMFLAG_FLCID )

#define	IDLFLAG_FRETVAL	( PARAMFLAG_FRETVAL )

//;begin_internal
#if 0
/* the following is what MIDL knows how to remote */
typedef struct tagELEMDESC
    {
    TYPEDESC tdesc;
    PARAMDESC paramdesc;
    } 	ELEMDESC;

#else /* 0 */
//;end_internal
typedef struct tagELEMDESC {
    TYPEDESC tdesc;             /* the type of the element */
    union {
        IDLDESC idldesc;        /* info for remoting the element */
        PARAMDESC paramdesc;    /* info about the parameter */
    } DUMMYUNIONNAME;
} ELEMDESC, * LPELEMDESC;
//;begin_internal
#endif /* 0 */
//;end_internal
typedef struct tagTYPEATTR
    {
    GUID guid;
    LCID lcid;
    DWORD dwReserved;
    MEMBERID memidConstructor;
    MEMBERID memidDestructor;
    LPOLESTR lpstrSchema;
    ULONG cbSizeInstance;
    TYPEKIND typekind;
    WORD cFuncs;
    WORD cVars;
    WORD cImplTypes;
    WORD cbSizeVft;
    WORD cbAlignment;
    WORD wTypeFlags;
    WORD wMajorVerNum;
    WORD wMinorVerNum;
    TYPEDESC tdescAlias;
    IDLDESC idldescType;
    } 	TYPEATTR;

typedef struct tagTYPEATTR *LPTYPEATTR;

typedef struct tagDISPPARAMS
    {
    /* [size_is] */ VARIANTARG *rgvarg;
    /* [size_is] */ DISPID *rgdispidNamedArgs;
    UINT cArgs;
    UINT cNamedArgs;
    } 	DISPPARAMS;

//;begin_internal
#if 0
/* the following is what MIDL knows how to remote */

typedef struct tagEXCEPINFO
    {
    WORD wCode;
    WORD wReserved;
    BSTR bstrSource;
    BSTR bstrDescription;
    BSTR bstrHelpFile;
    DWORD dwHelpContext;
    ULONG_PTR pvReserved;
    ULONG_PTR pfnDeferredFillIn;
    SCODE scode;
    } 	EXCEPINFO;

#else /* 0 */
//;end_internal
typedef struct tagEXCEPINFO {
    WORD  wCode;
    WORD  wReserved;
    BSTR  bstrSource;
    BSTR  bstrDescription;
    BSTR  bstrHelpFile;
    DWORD dwHelpContext;
    PVOID pvReserved;
    HRESULT (__stdcall *pfnDeferredFillIn)(struct tagEXCEPINFO *);
    SCODE scode;
} EXCEPINFO, * LPEXCEPINFO;
//;begin_internal
#endif /* 0 */
//;end_internal
typedef /* [v1_enum] */ 
enum tagCALLCONV
    {
        CC_FASTCALL	= 0,
        CC_CDECL	= 1,
        CC_MSCPASCAL	= ( CC_CDECL + 1 ) ,
        CC_PASCAL	= CC_MSCPASCAL,
        CC_MACPASCAL	= ( CC_PASCAL + 1 ) ,
        CC_STDCALL	= ( CC_MACPASCAL + 1 ) ,
        CC_FPFASTCALL	= ( CC_STDCALL + 1 ) ,
        CC_SYSCALL	= ( CC_FPFASTCALL + 1 ) ,
        CC_MPWCDECL	= ( CC_SYSCALL + 1 ) ,
        CC_MPWPASCAL	= ( CC_MPWCDECL + 1 ) ,
        CC_MAX	= ( CC_MPWPASCAL + 1 ) 
    } 	CALLCONV;

typedef /* [v1_enum] */ 
enum tagFUNCKIND
    {
        FUNC_VIRTUAL	= 0,
        FUNC_PUREVIRTUAL	= ( FUNC_VIRTUAL + 1 ) ,
        FUNC_NONVIRTUAL	= ( FUNC_PUREVIRTUAL + 1 ) ,
        FUNC_STATIC	= ( FUNC_NONVIRTUAL + 1 ) ,
        FUNC_DISPATCH	= ( FUNC_STATIC + 1 ) 
    } 	FUNCKIND;

typedef /* [v1_enum] */ 
enum tagINVOKEKIND
    {
        INVOKE_FUNC	= 1,
        INVOKE_PROPERTYGET	= 2,
        INVOKE_PROPERTYPUT	= 4,
        INVOKE_PROPERTYPUTREF	= 8
    } 	INVOKEKIND;

typedef struct tagFUNCDESC
    {
    MEMBERID memid;
    /* [size_is] */ SCODE *lprgscode;
    /* [size_is] */ ELEMDESC *lprgelemdescParam;
    FUNCKIND funckind;
    INVOKEKIND invkind;
    CALLCONV callconv;
    SHORT cParams;
    SHORT cParamsOpt;
    SHORT oVft;
    SHORT cScodes;
    ELEMDESC elemdescFunc;
    WORD wFuncFlags;
    } 	FUNCDESC;

typedef struct tagFUNCDESC *LPFUNCDESC;

typedef /* [v1_enum] */ 
enum tagVARKIND
    {
        VAR_PERINSTANCE	= 0,
        VAR_STATIC	= ( VAR_PERINSTANCE + 1 ) ,
        VAR_CONST	= ( VAR_STATIC + 1 ) ,
        VAR_DISPATCH	= ( VAR_CONST + 1 ) 
    } 	VARKIND;

#define	IMPLTYPEFLAG_FDEFAULT	( 0x1 )

#define	IMPLTYPEFLAG_FSOURCE	( 0x2 )

#define	IMPLTYPEFLAG_FRESTRICTED	( 0x4 )

#define	IMPLTYPEFLAG_FDEFAULTVTABLE	( 0x8 )

typedef struct tagVARDESC
    {
    MEMBERID memid;
    LPOLESTR lpstrSchema;
    /* [switch_is][switch_type] */ union 
        {
        /* [case()] */ ULONG oInst;
        /* [case()] */ VARIANT *lpvarValue;
        } 	DUMMYUNIONNAME;
    ELEMDESC elemdescVar;
    WORD wVarFlags;
    VARKIND varkind;
    } 	VARDESC;

typedef struct tagVARDESC *LPVARDESC;

typedef 
enum tagTYPEFLAGS
    {
        TYPEFLAG_FAPPOBJECT	= 0x1,
        TYPEFLAG_FCANCREATE	= 0x2,
        TYPEFLAG_FLICENSED	= 0x4,
        TYPEFLAG_FPREDECLID	= 0x8,
        TYPEFLAG_FHIDDEN	= 0x10,
        TYPEFLAG_FCONTROL	= 0x20,
        TYPEFLAG_FDUAL	= 0x40,
        TYPEFLAG_FNONEXTENSIBLE	= 0x80,
        TYPEFLAG_FOLEAUTOMATION	= 0x100,
        TYPEFLAG_FRESTRICTED	= 0x200,
        TYPEFLAG_FAGGREGATABLE	= 0x400,
        TYPEFLAG_FREPLACEABLE	= 0x800,
        TYPEFLAG_FDISPATCHABLE	= 0x1000,
        TYPEFLAG_FREVERSEBIND	= 0x2000,
        TYPEFLAG_FPROXY	= 0x4000
    } 	TYPEFLAGS;

typedef 
enum tagFUNCFLAGS
    {
        FUNCFLAG_FRESTRICTED	= 0x1,
        FUNCFLAG_FSOURCE	= 0x2,
        FUNCFLAG_FBINDABLE	= 0x4,
        FUNCFLAG_FREQUESTEDIT	= 0x8,
        FUNCFLAG_FDISPLAYBIND	= 0x10,
        FUNCFLAG_FDEFAULTBIND	= 0x20,
        FUNCFLAG_FHIDDEN	= 0x40,
        FUNCFLAG_FUSESGETLASTERROR	= 0x80,
        FUNCFLAG_FDEFAULTCOLLELEM	= 0x100,
        FUNCFLAG_FUIDEFAULT	= 0x200,
        FUNCFLAG_FNONBROWSABLE	= 0x400,
        FUNCFLAG_FREPLACEABLE	= 0x800,
        FUNCFLAG_FIMMEDIATEBIND	= 0x1000
    } 	FUNCFLAGS;

typedef 
enum tagVARFLAGS
    {
        VARFLAG_FREADONLY	= 0x1,
        VARFLAG_FSOURCE	= 0x2,
        VARFLAG_FBINDABLE	= 0x4,
        VARFLAG_FREQUESTEDIT	= 0x8,
        VARFLAG_FDISPLAYBIND	= 0x10,
        VARFLAG_FDEFAULTBIND	= 0x20,
        VARFLAG_FHIDDEN	= 0x40,
        VARFLAG_FRESTRICTED	= 0x80,
        VARFLAG_FDEFAULTCOLLELEM	= 0x100,
        VARFLAG_FUIDEFAULT	= 0x200,
        VARFLAG_FNONBROWSABLE	= 0x400,
        VARFLAG_FREPLACEABLE	= 0x800,
        VARFLAG_FIMMEDIATEBIND	= 0x1000
    } 	VARFLAGS;

typedef /* [wire_marshal] */ struct tagCLEANLOCALSTORAGE
    {
    IUnknown *pInterface;
    PVOID pStorage;
    DWORD flags;
    } 	CLEANLOCALSTORAGE;

typedef struct tagCUSTDATAITEM
    {
    GUID guid;
    VARIANTARG varValue;
    } 	CUSTDATAITEM;

typedef struct tagCUSTDATAITEM *LPCUSTDATAITEM;

typedef struct tagCUSTDATA
    {
    DWORD cCustData;
    /* [size_is] */ LPCUSTDATAITEM prgCustData;
    } 	CUSTDATA;

typedef struct tagCUSTDATA *LPCUSTDATA;



extern RPC_IF_HANDLE IOleAutomationTypes_v1_0_c_ifspec;
extern RPC_IF_HANDLE IOleAutomationTypes_v1_0_s_ifspec;
#endif /* __IOleAutomationTypes_INTERFACE_DEFINED__ */

/* interface __MIDL_itf_oaidl_0000_0001 */
/* [local] */ 

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES) */
#pragma endregion
#pragma region Desktop Family or OneCore Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM)


extern RPC_IF_HANDLE __MIDL_itf_oaidl_0000_0001_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_oaidl_0000_0001_v0_0_s_ifspec;

#ifndef __ICreateTypeInfo_INTERFACE_DEFINED__
#define __ICreateTypeInfo_INTERFACE_DEFINED__

/* interface ICreateTypeInfo */
/* [local][unique][uuid][object] */ 

typedef /* [unique] */ ICreateTypeInfo *LPCREATETYPEINFO;


EXTERN_C const IID IID_ICreateTypeInfo;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("00020405-0000-0000-C000-000000000046")
    ICreateTypeInfo : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE SetGuid( 
            /* [in] */ REFGUID guid) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetTypeFlags( 
            /* [in] */ UINT uTypeFlags) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetDocString( 
            /* [annotation][in] */ 
            __RPC__in  LPOLESTR pStrDoc) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetHelpContext( 
            /* [in] */ DWORD dwHelpContext) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetVersion( 
            /* [in] */ WORD wMajorVerNum,
            /* [in] */ WORD wMinorVerNum) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE AddRefTypeInfo( 
            /* [in] */ ITypeInfo *pTInfo,
            /* [in] */ HREFTYPE *phRefType) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE AddFuncDesc( 
            /* [in] */ UINT index,
            /* [in] */ FUNCDESC *pFuncDesc) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE AddImplType( 
            /* [in] */ UINT index,
            /* [in] */ HREFTYPE hRefType) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetImplTypeFlags( 
            /* [in] */ UINT index,
            /* [in] */ INT implTypeFlags) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetAlignment( 
            /* [in] */ WORD cbAlignment) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetSchema( 
            /* [annotation][in] */ 
            __RPC__in  LPOLESTR pStrSchema) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE AddVarDesc( 
            /* [in] */ UINT index,
            /* [in] */ VARDESC *pVarDesc) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetFuncAndParamNames( 
            /* [in] */ UINT index,
            /* [annotation][in][size_is][in] */ 
            __RPC__in_ecount(cNames)  LPOLESTR *rgszNames,
            /* [in] */ UINT cNames) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetVarName( 
            /* [in] */ UINT index,
            /* [annotation][in] */ 
            __RPC__in  LPOLESTR szName) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetTypeDescAlias( 
            /* [in] */ TYPEDESC *pTDescAlias) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE DefineFuncAsDllEntry( 
            /* [in] */ UINT index,
            /* [annotation][in] */ 
            __RPC__in  LPOLESTR szDllName,
            /* [annotation][in] */ 
            __RPC__in  LPOLESTR szProcName) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetFuncDocString( 
            /* [in] */ UINT index,
            /* [annotation][in] */ 
            __RPC__in  LPOLESTR szDocString) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetVarDocString( 
            /* [in] */ UINT index,
            /* [annotation][in] */ 
            __RPC__in  LPOLESTR szDocString) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetFuncHelpContext( 
            /* [in] */ UINT index,
            /* [in] */ DWORD dwHelpContext) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetVarHelpContext( 
            /* [in] */ UINT index,
            /* [in] */ DWORD dwHelpContext) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetMops( 
            /* [in] */ UINT index,
            /* [annotation][in] */ 
            _In_  BSTR bstrMops) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetTypeIdldesc( 
            /* [in] */ IDLDESC *pIdlDesc) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE LayOut( void) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ICreateTypeInfoVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            ICreateTypeInfo * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            ICreateTypeInfo * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            ICreateTypeInfo * This);
        
        DECLSPEC_XFGVIRT(ICreateTypeInfo, SetGuid)
        HRESULT ( STDMETHODCALLTYPE *SetGuid )( 
            ICreateTypeInfo * This,
            /* [in] */ REFGUID guid);
        
        DECLSPEC_XFGVIRT(ICreateTypeInfo, SetTypeFlags)
        HRESULT ( STDMETHODCALLTYPE *SetTypeFlags )( 
            ICreateTypeInfo * This,
            /* [in] */ UINT uTypeFlags);
        
        DECLSPEC_XFGVIRT(ICreateTypeInfo, SetDocString)
        HRESULT ( STDMETHODCALLTYPE *SetDocString )( 
            ICreateTypeInfo * This,
            /* [annotation][in] */ 
            __RPC__in  LPOLESTR pStrDoc);
        
        DECLSPEC_XFGVIRT(ICreateTypeInfo, SetHelpContext)
        HRESULT ( STDMETHODCALLTYPE *SetHelpContext )( 
            ICreateTypeInfo * This,
            /* [in] */ DWORD dwHelpContext);
        
        DECLSPEC_XFGVIRT(ICreateTypeInfo, SetVersion)
        HRESULT ( STDMETHODCALLTYPE *SetVersion )( 
            ICreateTypeInfo * This,
            /* [in] */ WORD wMajorVerNum,
            /* [in] */ WORD wMinorVerNum);
        
        DECLSPEC_XFGVIRT(ICreateTypeInfo, AddRefTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *AddRefTypeInfo )( 
            ICreateTypeInfo * This,
            /* [in] */ ITypeInfo *pTInfo,
            /* [in] */ HREFTYPE *phRefType);
        
        DECLSPEC_XFGVIRT(ICreateTypeInfo, AddFuncDesc)
        HRESULT ( STDMETHODCALLTYPE *AddFuncDesc )( 
            ICreateTypeInfo * This,
            /* [in] */ UINT index,
            /* [in] */ FUNCDESC *pFuncDesc);
        
        DECLSPEC_XFGVIRT(ICreateTypeInfo, AddImplType)
        HRESULT ( STDMETHODCALLTYPE *AddImplType )( 
            ICreateTypeInfo * This,
            /* [in] */ UINT index,
            /* [in] */ HREFTYPE hRefType);
        
        DECLSPEC_XFGVIRT(ICreateTypeInfo, SetImplTypeFlags)
        HRESULT ( STDMETHODCALLTYPE *SetImplTypeFlags )( 
            ICreateTypeInfo * This,
            /* [in] */ UINT index,
            /* [in] */ INT implTypeFlags);
        
        DECLSPEC_XFGVIRT(ICreateTypeInfo, SetAlignment)
        HRESULT ( STDMETHODCALLTYPE *SetAlignment )( 
            ICreateTypeInfo * This,
            /* [in] */ WORD cbAlignment);
        
        DECLSPEC_XFGVIRT(ICreateTypeInfo, SetSchema)
        HRESULT ( STDMETHODCALLTYPE *SetSchema )( 
            ICreateTypeInfo * This,
            /* [annotation][in] */ 
            __RPC__in  LPOLESTR pStrSchema);
        
        DECLSPEC_XFGVIRT(ICreateTypeInfo, AddVarDesc)
        HRESULT ( STDMETHODCALLTYPE *AddVarDesc )( 
            ICreateTypeInfo * This,
            /* [in] */ UINT index,
            /* [in] */ VARDESC *pVarDesc);
        
        DECLSPEC_XFGVIRT(ICreateTypeInfo, SetFuncAndParamNames)
        HRESULT ( STDMETHODCALLTYPE *SetFuncAndParamNames )( 
            ICreateTypeInfo * This,
            /* [in] */ UINT index,
            /* [annotation][in][size_is][in] */ 
            __RPC__in_ecount(cNames)  LPOLESTR *rgszNames,
            /* [in] */ UINT cNames);
        
        DECLSPEC_XFGVIRT(ICreateTypeInfo, SetVarName)
        HRESULT ( STDMETHODCALLTYPE *SetVarName )( 
            ICreateTypeInfo * This,
            /* [in] */ UINT index,
            /* [annotation][in] */ 
            __RPC__in  LPOLESTR szName);
        
        DECLSPEC_XFGVIRT(ICreateTypeInfo, SetTypeDescAlias)
        HRESULT ( STDMETHODCALLTYPE *SetTypeDescAlias )( 
            ICreateTypeInfo * This,
            /* [in] */ TYPEDESC *pTDescAlias);
        
        DECLSPEC_XFGVIRT(ICreateTypeInfo, DefineFuncAsDllEntry)
        HRESULT ( STDMETHODCALLTYPE *DefineFuncAsDllEntry )( 
            ICreateTypeInfo * This,
            /* [in] */ UINT index,
            /* [annotation][in] */ 
            __RPC__in  LPOLESTR szDllName,
            /* [annotation][in] */ 
            __RPC__in  LPOLESTR szProcName);
        
        DECLSPEC_XFGVIRT(ICreateTypeInfo, SetFuncDocString)
        HRESULT ( STDMETHODCALLTYPE *SetFuncDocString )( 
            ICreateTypeInfo * This,
            /* [in] */ UINT index,
            /* [annotation][in] */ 
            __RPC__in  LPOLESTR szDocString);
        
        DECLSPEC_XFGVIRT(ICreateTypeInfo, SetVarDocString)
        HRESULT ( STDMETHODCALLTYPE *SetVarDocString )( 
            ICreateTypeInfo * This,
            /* [in] */ UINT index,
            /* [annotation][in] */ 
            __RPC__in  LPOLESTR szDocString);
        
        DECLSPEC_XFGVIRT(ICreateTypeInfo, SetFuncHelpContext)
        HRESULT ( STDMETHODCALLTYPE *SetFuncHelpContext )( 
            ICreateTypeInfo * This,
            /* [in] */ UINT index,
            /* [in] */ DWORD dwHelpContext);
        
        DECLSPEC_XFGVIRT(ICreateTypeInfo, SetVarHelpContext)
        HRESULT ( STDMETHODCALLTYPE *SetVarHelpContext )( 
            ICreateTypeInfo * This,
            /* [in] */ UINT index,
            /* [in] */ DWORD dwHelpContext);
        
        DECLSPEC_XFGVIRT(ICreateTypeInfo, SetMops)
        HRESULT ( STDMETHODCALLTYPE *SetMops )( 
            ICreateTypeInfo * This,
            /* [in] */ UINT index,
            /* [annotation][in] */ 
            _In_  BSTR bstrMops);
        
        DECLSPEC_XFGVIRT(ICreateTypeInfo, SetTypeIdldesc)
        HRESULT ( STDMETHODCALLTYPE *SetTypeIdldesc )( 
            ICreateTypeInfo * This,
            /* [in] */ IDLDESC *pIdlDesc);
        
        DECLSPEC_XFGVIRT(ICreateTypeInfo, LayOut)
        HRESULT ( STDMETHODCALLTYPE *LayOut )( 
            ICreateTypeInfo * This);
        
        END_INTERFACE
    } ICreateTypeInfoVtbl;

    interface ICreateTypeInfo
    {
        CONST_VTBL struct ICreateTypeInfoVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ICreateTypeInfo_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ICreateTypeInfo_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ICreateTypeInfo_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ICreateTypeInfo_SetGuid(This,guid)	\
    ( (This)->lpVtbl -> SetGuid(This,guid) ) 

#define ICreateTypeInfo_SetTypeFlags(This,uTypeFlags)	\
    ( (This)->lpVtbl -> SetTypeFlags(This,uTypeFlags) ) 

#define ICreateTypeInfo_SetDocString(This,pStrDoc)	\
    ( (This)->lpVtbl -> SetDocString(This,pStrDoc) ) 

#define ICreateTypeInfo_SetHelpContext(This,dwHelpContext)	\
    ( (This)->lpVtbl -> SetHelpContext(This,dwHelpContext) ) 

#define ICreateTypeInfo_SetVersion(This,wMajorVerNum,wMinorVerNum)	\
    ( (This)->lpVtbl -> SetVersion(This,wMajorVerNum,wMinorVerNum) ) 

#define ICreateTypeInfo_AddRefTypeInfo(This,pTInfo,phRefType)	\
    ( (This)->lpVtbl -> AddRefTypeInfo(This,pTInfo,phRefType) ) 

#define ICreateTypeInfo_AddFuncDesc(This,index,pFuncDesc)	\
    ( (This)->lpVtbl -> AddFuncDesc(This,index,pFuncDesc) ) 

#define ICreateTypeInfo_AddImplType(This,index,hRefType)	\
    ( (This)->lpVtbl -> AddImplType(This,index,hRefType) ) 

#define ICreateTypeInfo_SetImplTypeFlags(This,index,implTypeFlags)	\
    ( (This)->lpVtbl -> SetImplTypeFlags(This,index,implTypeFlags) ) 

#define ICreateTypeInfo_SetAlignment(This,cbAlignment)	\
    ( (This)->lpVtbl -> SetAlignment(This,cbAlignment) ) 

#define ICreateTypeInfo_SetSchema(This,pStrSchema)	\
    ( (This)->lpVtbl -> SetSchema(This,pStrSchema) ) 

#define ICreateTypeInfo_AddVarDesc(This,index,pVarDesc)	\
    ( (This)->lpVtbl -> AddVarDesc(This,index,pVarDesc) ) 

#define ICreateTypeInfo_SetFuncAndParamNames(This,index,rgszNames,cNames)	\
    ( (This)->lpVtbl -> SetFuncAndParamNames(This,index,rgszNames,cNames) ) 

#define ICreateTypeInfo_SetVarName(This,index,szName)	\
    ( (This)->lpVtbl -> SetVarName(This,index,szName) ) 

#define ICreateTypeInfo_SetTypeDescAlias(This,pTDescAlias)	\
    ( (This)->lpVtbl -> SetTypeDescAlias(This,pTDescAlias) ) 

#define ICreateTypeInfo_DefineFuncAsDllEntry(This,index,szDllName,szProcName)	\
    ( (This)->lpVtbl -> DefineFuncAsDllEntry(This,index,szDllName,szProcName) ) 

#define ICreateTypeInfo_SetFuncDocString(This,index,szDocString)	\
    ( (This)->lpVtbl -> SetFuncDocString(This,index,szDocString) ) 

#define ICreateTypeInfo_SetVarDocString(This,index,szDocString)	\
    ( (This)->lpVtbl -> SetVarDocString(This,index,szDocString) ) 

#define ICreateTypeInfo_SetFuncHelpContext(This,index,dwHelpContext)	\
    ( (This)->lpVtbl -> SetFuncHelpContext(This,index,dwHelpContext) ) 

#define ICreateTypeInfo_SetVarHelpContext(This,index,dwHelpContext)	\
    ( (This)->lpVtbl -> SetVarHelpContext(This,index,dwHelpContext) ) 

#define ICreateTypeInfo_SetMops(This,index,bstrMops)	\
    ( (This)->lpVtbl -> SetMops(This,index,bstrMops) ) 

#define ICreateTypeInfo_SetTypeIdldesc(This,pIdlDesc)	\
    ( (This)->lpVtbl -> SetTypeIdldesc(This,pIdlDesc) ) 

#define ICreateTypeInfo_LayOut(This)	\
    ( (This)->lpVtbl -> LayOut(This) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ICreateTypeInfo_INTERFACE_DEFINED__ */


#ifndef __ICreateTypeInfo2_INTERFACE_DEFINED__
#define __ICreateTypeInfo2_INTERFACE_DEFINED__

/* interface ICreateTypeInfo2 */
/* [local][unique][uuid][object] */ 

typedef /* [unique] */ ICreateTypeInfo2 *LPCREATETYPEINFO2;


EXTERN_C const IID IID_ICreateTypeInfo2;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("0002040E-0000-0000-C000-000000000046")
    ICreateTypeInfo2 : public ICreateTypeInfo
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE DeleteFuncDesc( 
            /* [in] */ UINT index) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE DeleteFuncDescByMemId( 
            /* [in] */ MEMBERID memid,
            /* [in] */ INVOKEKIND invKind) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE DeleteVarDesc( 
            /* [in] */ UINT index) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE DeleteVarDescByMemId( 
            /* [in] */ MEMBERID memid) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE DeleteImplType( 
            /* [in] */ UINT index) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetCustData( 
            /* [in] */ REFGUID guid,
            /* [in] */ VARIANT *pVarVal) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetFuncCustData( 
            /* [in] */ UINT index,
            /* [in] */ REFGUID guid,
            /* [in] */ VARIANT *pVarVal) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetParamCustData( 
            /* [in] */ UINT indexFunc,
            /* [in] */ UINT indexParam,
            /* [in] */ REFGUID guid,
            /* [in] */ VARIANT *pVarVal) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetVarCustData( 
            /* [in] */ UINT index,
            /* [in] */ REFGUID guid,
            /* [in] */ VARIANT *pVarVal) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetImplTypeCustData( 
            /* [in] */ UINT index,
            /* [in] */ REFGUID guid,
            /* [in] */ VARIANT *pVarVal) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetHelpStringContext( 
            /* [in] */ ULONG dwHelpStringContext) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetFuncHelpStringContext( 
            /* [in] */ UINT index,
            /* [in] */ ULONG dwHelpStringContext) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetVarHelpStringContext( 
            /* [in] */ UINT index,
            /* [in] */ ULONG dwHelpStringContext) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Invalidate( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetName( 
            /* [annotation][in] */ 
            __RPC__in  LPOLESTR szName) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ICreateTypeInfo2Vtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            ICreateTypeInfo2 * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            ICreateTypeInfo2 * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            ICreateTypeInfo2 * This);
        
        DECLSPEC_XFGVIRT(ICreateTypeInfo, SetGuid)
        HRESULT ( STDMETHODCALLTYPE *SetGuid )( 
            ICreateTypeInfo2 * This,
            /* [in] */ REFGUID guid);
        
        DECLSPEC_XFGVIRT(ICreateTypeInfo, SetTypeFlags)
        HRESULT ( STDMETHODCALLTYPE *SetTypeFlags )( 
            ICreateTypeInfo2 * This,
            /* [in] */ UINT uTypeFlags);
        
        DECLSPEC_XFGVIRT(ICreateTypeInfo, SetDocString)
        HRESULT ( STDMETHODCALLTYPE *SetDocString )( 
            ICreateTypeInfo2 * This,
            /* [annotation][in] */ 
            __RPC__in  LPOLESTR pStrDoc);
        
        DECLSPEC_XFGVIRT(ICreateTypeInfo, SetHelpContext)
        HRESULT ( STDMETHODCALLTYPE *SetHelpContext )( 
            ICreateTypeInfo2 * This,
            /* [in] */ DWORD dwHelpContext);
        
        DECLSPEC_XFGVIRT(ICreateTypeInfo, SetVersion)
        HRESULT ( STDMETHODCALLTYPE *SetVersion )( 
            ICreateTypeInfo2 * This,
            /* [in] */ WORD wMajorVerNum,
            /* [in] */ WORD wMinorVerNum);
        
        DECLSPEC_XFGVIRT(ICreateTypeInfo, AddRefTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *AddRefTypeInfo )( 
            ICreateTypeInfo2 * This,
            /* [in] */ ITypeInfo *pTInfo,
            /* [in] */ HREFTYPE *phRefType);
        
        DECLSPEC_XFGVIRT(ICreateTypeInfo, AddFuncDesc)
        HRESULT ( STDMETHODCALLTYPE *AddFuncDesc )( 
            ICreateTypeInfo2 * This,
            /* [in] */ UINT index,
            /* [in] */ FUNCDESC *pFuncDesc);
        
        DECLSPEC_XFGVIRT(ICreateTypeInfo, AddImplType)
        HRESULT ( STDMETHODCALLTYPE *AddImplType )( 
            ICreateTypeInfo2 * This,
            /* [in] */ UINT index,
            /* [in] */ HREFTYPE hRefType);
        
        DECLSPEC_XFGVIRT(ICreateTypeInfo, SetImplTypeFlags)
        HRESULT ( STDMETHODCALLTYPE *SetImplTypeFlags )( 
            ICreateTypeInfo2 * This,
            /* [in] */ UINT index,
            /* [in] */ INT implTypeFlags);
        
        DECLSPEC_XFGVIRT(ICreateTypeInfo, SetAlignment)
        HRESULT ( STDMETHODCALLTYPE *SetAlignment )( 
            ICreateTypeInfo2 * This,
            /* [in] */ WORD cbAlignment);
        
        DECLSPEC_XFGVIRT(ICreateTypeInfo, SetSchema)
        HRESULT ( STDMETHODCALLTYPE *SetSchema )( 
            ICreateTypeInfo2 * This,
            /* [annotation][in] */ 
            __RPC__in  LPOLESTR pStrSchema);
        
        DECLSPEC_XFGVIRT(ICreateTypeInfo, AddVarDesc)
        HRESULT ( STDMETHODCALLTYPE *AddVarDesc )( 
            ICreateTypeInfo2 * This,
            /* [in] */ UINT index,
            /* [in] */ VARDESC *pVarDesc);
        
        DECLSPEC_XFGVIRT(ICreateTypeInfo, SetFuncAndParamNames)
        HRESULT ( STDMETHODCALLTYPE *SetFuncAndParamNames )( 
            ICreateTypeInfo2 * This,
            /* [in] */ UINT index,
            /* [annotation][in][size_is][in] */ 
            __RPC__in_ecount(cNames)  LPOLESTR *rgszNames,
            /* [in] */ UINT cNames);
        
        DECLSPEC_XFGVIRT(ICreateTypeInfo, SetVarName)
        HRESULT ( STDMETHODCALLTYPE *SetVarName )( 
            ICreateTypeInfo2 * This,
            /* [in] */ UINT index,
            /* [annotation][in] */ 
            __RPC__in  LPOLESTR szName);
        
        DECLSPEC_XFGVIRT(ICreateTypeInfo, SetTypeDescAlias)
        HRESULT ( STDMETHODCALLTYPE *SetTypeDescAlias )( 
            ICreateTypeInfo2 * This,
            /* [in] */ TYPEDESC *pTDescAlias);
        
        DECLSPEC_XFGVIRT(ICreateTypeInfo, DefineFuncAsDllEntry)
        HRESULT ( STDMETHODCALLTYPE *DefineFuncAsDllEntry )( 
            ICreateTypeInfo2 * This,
            /* [in] */ UINT index,
            /* [annotation][in] */ 
            __RPC__in  LPOLESTR szDllName,
            /* [annotation][in] */ 
            __RPC__in  LPOLESTR szProcName);
        
        DECLSPEC_XFGVIRT(ICreateTypeInfo, SetFuncDocString)
        HRESULT ( STDMETHODCALLTYPE *SetFuncDocString )( 
            ICreateTypeInfo2 * This,
            /* [in] */ UINT index,
            /* [annotation][in] */ 
            __RPC__in  LPOLESTR szDocString);
        
        DECLSPEC_XFGVIRT(ICreateTypeInfo, SetVarDocString)
        HRESULT ( STDMETHODCALLTYPE *SetVarDocString )( 
            ICreateTypeInfo2 * This,
            /* [in] */ UINT index,
            /* [annotation][in] */ 
            __RPC__in  LPOLESTR szDocString);
        
        DECLSPEC_XFGVIRT(ICreateTypeInfo, SetFuncHelpContext)
        HRESULT ( STDMETHODCALLTYPE *SetFuncHelpContext )( 
            ICreateTypeInfo2 * This,
            /* [in] */ UINT index,
            /* [in] */ DWORD dwHelpContext);
        
        DECLSPEC_XFGVIRT(ICreateTypeInfo, SetVarHelpContext)
        HRESULT ( STDMETHODCALLTYPE *SetVarHelpContext )( 
            ICreateTypeInfo2 * This,
            /* [in] */ UINT index,
            /* [in] */ DWORD dwHelpContext);
        
        DECLSPEC_XFGVIRT(ICreateTypeInfo, SetMops)
        HRESULT ( STDMETHODCALLTYPE *SetMops )( 
            ICreateTypeInfo2 * This,
            /* [in] */ UINT index,
            /* [annotation][in] */ 
            _In_  BSTR bstrMops);
        
        DECLSPEC_XFGVIRT(ICreateTypeInfo, SetTypeIdldesc)
        HRESULT ( STDMETHODCALLTYPE *SetTypeIdldesc )( 
            ICreateTypeInfo2 * This,
            /* [in] */ IDLDESC *pIdlDesc);
        
        DECLSPEC_XFGVIRT(ICreateTypeInfo, LayOut)
        HRESULT ( STDMETHODCALLTYPE *LayOut )( 
            ICreateTypeInfo2 * This);
        
        DECLSPEC_XFGVIRT(ICreateTypeInfo2, DeleteFuncDesc)
        HRESULT ( STDMETHODCALLTYPE *DeleteFuncDesc )( 
            ICreateTypeInfo2 * This,
            /* [in] */ UINT index);
        
        DECLSPEC_XFGVIRT(ICreateTypeInfo2, DeleteFuncDescByMemId)
        HRESULT ( STDMETHODCALLTYPE *DeleteFuncDescByMemId )( 
            ICreateTypeInfo2 * This,
            /* [in] */ MEMBERID memid,
            /* [in] */ INVOKEKIND invKind);
        
        DECLSPEC_XFGVIRT(ICreateTypeInfo2, DeleteVarDesc)
        HRESULT ( STDMETHODCALLTYPE *DeleteVarDesc )( 
            ICreateTypeInfo2 * This,
            /* [in] */ UINT index);
        
        DECLSPEC_XFGVIRT(ICreateTypeInfo2, DeleteVarDescByMemId)
        HRESULT ( STDMETHODCALLTYPE *DeleteVarDescByMemId )( 
            ICreateTypeInfo2 * This,
            /* [in] */ MEMBERID memid);
        
        DECLSPEC_XFGVIRT(ICreateTypeInfo2, DeleteImplType)
        HRESULT ( STDMETHODCALLTYPE *DeleteImplType )( 
            ICreateTypeInfo2 * This,
            /* [in] */ UINT index);
        
        DECLSPEC_XFGVIRT(ICreateTypeInfo2, SetCustData)
        HRESULT ( STDMETHODCALLTYPE *SetCustData )( 
            ICreateTypeInfo2 * This,
            /* [in] */ REFGUID guid,
            /* [in] */ VARIANT *pVarVal);
        
        DECLSPEC_XFGVIRT(ICreateTypeInfo2, SetFuncCustData)
        HRESULT ( STDMETHODCALLTYPE *SetFuncCustData )( 
            ICreateTypeInfo2 * This,
            /* [in] */ UINT index,
            /* [in] */ REFGUID guid,
            /* [in] */ VARIANT *pVarVal);
        
        DECLSPEC_XFGVIRT(ICreateTypeInfo2, SetParamCustData)
        HRESULT ( STDMETHODCALLTYPE *SetParamCustData )( 
            ICreateTypeInfo2 * This,
            /* [in] */ UINT indexFunc,
            /* [in] */ UINT indexParam,
            /* [in] */ REFGUID guid,
            /* [in] */ VARIANT *pVarVal);
        
        DECLSPEC_XFGVIRT(ICreateTypeInfo2, SetVarCustData)
        HRESULT ( STDMETHODCALLTYPE *SetVarCustData )( 
            ICreateTypeInfo2 * This,
            /* [in] */ UINT index,
            /* [in] */ REFGUID guid,
            /* [in] */ VARIANT *pVarVal);
        
        DECLSPEC_XFGVIRT(ICreateTypeInfo2, SetImplTypeCustData)
        HRESULT ( STDMETHODCALLTYPE *SetImplTypeCustData )( 
            ICreateTypeInfo2 * This,
            /* [in] */ UINT index,
            /* [in] */ REFGUID guid,
            /* [in] */ VARIANT *pVarVal);
        
        DECLSPEC_XFGVIRT(ICreateTypeInfo2, SetHelpStringContext)
        HRESULT ( STDMETHODCALLTYPE *SetHelpStringContext )( 
            ICreateTypeInfo2 * This,
            /* [in] */ ULONG dwHelpStringContext);
        
        DECLSPEC_XFGVIRT(ICreateTypeInfo2, SetFuncHelpStringContext)
        HRESULT ( STDMETHODCALLTYPE *SetFuncHelpStringContext )( 
            ICreateTypeInfo2 * This,
            /* [in] */ UINT index,
            /* [in] */ ULONG dwHelpStringContext);
        
        DECLSPEC_XFGVIRT(ICreateTypeInfo2, SetVarHelpStringContext)
        HRESULT ( STDMETHODCALLTYPE *SetVarHelpStringContext )( 
            ICreateTypeInfo2 * This,
            /* [in] */ UINT index,
            /* [in] */ ULONG dwHelpStringContext);
        
        DECLSPEC_XFGVIRT(ICreateTypeInfo2, Invalidate)
        HRESULT ( STDMETHODCALLTYPE *Invalidate )( 
            ICreateTypeInfo2 * This);
        
        DECLSPEC_XFGVIRT(ICreateTypeInfo2, SetName)
        HRESULT ( STDMETHODCALLTYPE *SetName )( 
            ICreateTypeInfo2 * This,
            /* [annotation][in] */ 
            __RPC__in  LPOLESTR szName);
        
        END_INTERFACE
    } ICreateTypeInfo2Vtbl;

    interface ICreateTypeInfo2
    {
        CONST_VTBL struct ICreateTypeInfo2Vtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ICreateTypeInfo2_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ICreateTypeInfo2_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ICreateTypeInfo2_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ICreateTypeInfo2_SetGuid(This,guid)	\
    ( (This)->lpVtbl -> SetGuid(This,guid) ) 

#define ICreateTypeInfo2_SetTypeFlags(This,uTypeFlags)	\
    ( (This)->lpVtbl -> SetTypeFlags(This,uTypeFlags) ) 

#define ICreateTypeInfo2_SetDocString(This,pStrDoc)	\
    ( (This)->lpVtbl -> SetDocString(This,pStrDoc) ) 

#define ICreateTypeInfo2_SetHelpContext(This,dwHelpContext)	\
    ( (This)->lpVtbl -> SetHelpContext(This,dwHelpContext) ) 

#define ICreateTypeInfo2_SetVersion(This,wMajorVerNum,wMinorVerNum)	\
    ( (This)->lpVtbl -> SetVersion(This,wMajorVerNum,wMinorVerNum) ) 

#define ICreateTypeInfo2_AddRefTypeInfo(This,pTInfo,phRefType)	\
    ( (This)->lpVtbl -> AddRefTypeInfo(This,pTInfo,phRefType) ) 

#define ICreateTypeInfo2_AddFuncDesc(This,index,pFuncDesc)	\
    ( (This)->lpVtbl -> AddFuncDesc(This,index,pFuncDesc) ) 

#define ICreateTypeInfo2_AddImplType(This,index,hRefType)	\
    ( (This)->lpVtbl -> AddImplType(This,index,hRefType) ) 

#define ICreateTypeInfo2_SetImplTypeFlags(This,index,implTypeFlags)	\
    ( (This)->lpVtbl -> SetImplTypeFlags(This,index,implTypeFlags) ) 

#define ICreateTypeInfo2_SetAlignment(This,cbAlignment)	\
    ( (This)->lpVtbl -> SetAlignment(This,cbAlignment) ) 

#define ICreateTypeInfo2_SetSchema(This,pStrSchema)	\
    ( (This)->lpVtbl -> SetSchema(This,pStrSchema) ) 

#define ICreateTypeInfo2_AddVarDesc(This,index,pVarDesc)	\
    ( (This)->lpVtbl -> AddVarDesc(This,index,pVarDesc) ) 

#define ICreateTypeInfo2_SetFuncAndParamNames(This,index,rgszNames,cNames)	\
    ( (This)->lpVtbl -> SetFuncAndParamNames(This,index,rgszNames,cNames) ) 

#define ICreateTypeInfo2_SetVarName(This,index,szName)	\
    ( (This)->lpVtbl -> SetVarName(This,index,szName) ) 

#define ICreateTypeInfo2_SetTypeDescAlias(This,pTDescAlias)	\
    ( (This)->lpVtbl -> SetTypeDescAlias(This,pTDescAlias) ) 

#define ICreateTypeInfo2_DefineFuncAsDllEntry(This,index,szDllName,szProcName)	\
    ( (This)->lpVtbl -> DefineFuncAsDllEntry(This,index,szDllName,szProcName) ) 

#define ICreateTypeInfo2_SetFuncDocString(This,index,szDocString)	\
    ( (This)->lpVtbl -> SetFuncDocString(This,index,szDocString) ) 

#define ICreateTypeInfo2_SetVarDocString(This,index,szDocString)	\
    ( (This)->lpVtbl -> SetVarDocString(This,index,szDocString) ) 

#define ICreateTypeInfo2_SetFuncHelpContext(This,index,dwHelpContext)	\
    ( (This)->lpVtbl -> SetFuncHelpContext(This,index,dwHelpContext) ) 

#define ICreateTypeInfo2_SetVarHelpContext(This,index,dwHelpContext)	\
    ( (This)->lpVtbl -> SetVarHelpContext(This,index,dwHelpContext) ) 

#define ICreateTypeInfo2_SetMops(This,index,bstrMops)	\
    ( (This)->lpVtbl -> SetMops(This,index,bstrMops) ) 

#define ICreateTypeInfo2_SetTypeIdldesc(This,pIdlDesc)	\
    ( (This)->lpVtbl -> SetTypeIdldesc(This,pIdlDesc) ) 

#define ICreateTypeInfo2_LayOut(This)	\
    ( (This)->lpVtbl -> LayOut(This) ) 


#define ICreateTypeInfo2_DeleteFuncDesc(This,index)	\
    ( (This)->lpVtbl -> DeleteFuncDesc(This,index) ) 

#define ICreateTypeInfo2_DeleteFuncDescByMemId(This,memid,invKind)	\
    ( (This)->lpVtbl -> DeleteFuncDescByMemId(This,memid,invKind) ) 

#define ICreateTypeInfo2_DeleteVarDesc(This,index)	\
    ( (This)->lpVtbl -> DeleteVarDesc(This,index) ) 

#define ICreateTypeInfo2_DeleteVarDescByMemId(This,memid)	\
    ( (This)->lpVtbl -> DeleteVarDescByMemId(This,memid) ) 

#define ICreateTypeInfo2_DeleteImplType(This,index)	\
    ( (This)->lpVtbl -> DeleteImplType(This,index) ) 

#define ICreateTypeInfo2_SetCustData(This,guid,pVarVal)	\
    ( (This)->lpVtbl -> SetCustData(This,guid,pVarVal) ) 

#define ICreateTypeInfo2_SetFuncCustData(This,index,guid,pVarVal)	\
    ( (This)->lpVtbl -> SetFuncCustData(This,index,guid,pVarVal) ) 

#define ICreateTypeInfo2_SetParamCustData(This,indexFunc,indexParam,guid,pVarVal)	\
    ( (This)->lpVtbl -> SetParamCustData(This,indexFunc,indexParam,guid,pVarVal) ) 

#define ICreateTypeInfo2_SetVarCustData(This,index,guid,pVarVal)	\
    ( (This)->lpVtbl -> SetVarCustData(This,index,guid,pVarVal) ) 

#define ICreateTypeInfo2_SetImplTypeCustData(This,index,guid,pVarVal)	\
    ( (This)->lpVtbl -> SetImplTypeCustData(This,index,guid,pVarVal) ) 

#define ICreateTypeInfo2_SetHelpStringContext(This,dwHelpStringContext)	\
    ( (This)->lpVtbl -> SetHelpStringContext(This,dwHelpStringContext) ) 

#define ICreateTypeInfo2_SetFuncHelpStringContext(This,index,dwHelpStringContext)	\
    ( (This)->lpVtbl -> SetFuncHelpStringContext(This,index,dwHelpStringContext) ) 

#define ICreateTypeInfo2_SetVarHelpStringContext(This,index,dwHelpStringContext)	\
    ( (This)->lpVtbl -> SetVarHelpStringContext(This,index,dwHelpStringContext) ) 

#define ICreateTypeInfo2_Invalidate(This)	\
    ( (This)->lpVtbl -> Invalidate(This) ) 

#define ICreateTypeInfo2_SetName(This,szName)	\
    ( (This)->lpVtbl -> SetName(This,szName) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ICreateTypeInfo2_INTERFACE_DEFINED__ */


#ifndef __ICreateTypeLib_INTERFACE_DEFINED__
#define __ICreateTypeLib_INTERFACE_DEFINED__

/* interface ICreateTypeLib */
/* [local][unique][uuid][object] */ 

typedef /* [unique] */ ICreateTypeLib *LPCREATETYPELIB;


EXTERN_C const IID IID_ICreateTypeLib;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("00020406-0000-0000-C000-000000000046")
    ICreateTypeLib : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE CreateTypeInfo( 
            /* [annotation][in] */ 
            __RPC__in  LPOLESTR szName,
            /* [in] */ TYPEKIND tkind,
            /* [out] */ ICreateTypeInfo **ppCTInfo) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetName( 
            /* [annotation][in] */ 
            __RPC__in  LPOLESTR szName) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetVersion( 
            /* [in] */ WORD wMajorVerNum,
            /* [in] */ WORD wMinorVerNum) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetGuid( 
            /* [in] */ REFGUID guid) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetDocString( 
            /* [annotation][in] */ 
            __RPC__in  LPOLESTR szDoc) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetHelpFileName( 
            /* [annotation][in] */ 
            __RPC__in  LPOLESTR szHelpFileName) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetHelpContext( 
            /* [in] */ DWORD dwHelpContext) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetLcid( 
            /* [in] */ LCID lcid) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetLibFlags( 
            /* [in] */ UINT uLibFlags) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SaveAllChanges( void) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ICreateTypeLibVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            ICreateTypeLib * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            ICreateTypeLib * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            ICreateTypeLib * This);
        
        DECLSPEC_XFGVIRT(ICreateTypeLib, CreateTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *CreateTypeInfo )( 
            ICreateTypeLib * This,
            /* [annotation][in] */ 
            __RPC__in  LPOLESTR szName,
            /* [in] */ TYPEKIND tkind,
            /* [out] */ ICreateTypeInfo **ppCTInfo);
        
        DECLSPEC_XFGVIRT(ICreateTypeLib, SetName)
        HRESULT ( STDMETHODCALLTYPE *SetName )( 
            ICreateTypeLib * This,
            /* [annotation][in] */ 
            __RPC__in  LPOLESTR szName);
        
        DECLSPEC_XFGVIRT(ICreateTypeLib, SetVersion)
        HRESULT ( STDMETHODCALLTYPE *SetVersion )( 
            ICreateTypeLib * This,
            /* [in] */ WORD wMajorVerNum,
            /* [in] */ WORD wMinorVerNum);
        
        DECLSPEC_XFGVIRT(ICreateTypeLib, SetGuid)
        HRESULT ( STDMETHODCALLTYPE *SetGuid )( 
            ICreateTypeLib * This,
            /* [in] */ REFGUID guid);
        
        DECLSPEC_XFGVIRT(ICreateTypeLib, SetDocString)
        HRESULT ( STDMETHODCALLTYPE *SetDocString )( 
            ICreateTypeLib * This,
            /* [annotation][in] */ 
            __RPC__in  LPOLESTR szDoc);
        
        DECLSPEC_XFGVIRT(ICreateTypeLib, SetHelpFileName)
        HRESULT ( STDMETHODCALLTYPE *SetHelpFileName )( 
            ICreateTypeLib * This,
            /* [annotation][in] */ 
            __RPC__in  LPOLESTR szHelpFileName);
        
        DECLSPEC_XFGVIRT(ICreateTypeLib, SetHelpContext)
        HRESULT ( STDMETHODCALLTYPE *SetHelpContext )( 
            ICreateTypeLib * This,
            /* [in] */ DWORD dwHelpContext);
        
        DECLSPEC_XFGVIRT(ICreateTypeLib, SetLcid)
        HRESULT ( STDMETHODCALLTYPE *SetLcid )( 
            ICreateTypeLib * This,
            /* [in] */ LCID lcid);
        
        DECLSPEC_XFGVIRT(ICreateTypeLib, SetLibFlags)
        HRESULT ( STDMETHODCALLTYPE *SetLibFlags )( 
            ICreateTypeLib * This,
            /* [in] */ UINT uLibFlags);
        
        DECLSPEC_XFGVIRT(ICreateTypeLib, SaveAllChanges)
        HRESULT ( STDMETHODCALLTYPE *SaveAllChanges )( 
            ICreateTypeLib * This);
        
        END_INTERFACE
    } ICreateTypeLibVtbl;

    interface ICreateTypeLib
    {
        CONST_VTBL struct ICreateTypeLibVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ICreateTypeLib_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ICreateTypeLib_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ICreateTypeLib_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ICreateTypeLib_CreateTypeInfo(This,szName,tkind,ppCTInfo)	\
    ( (This)->lpVtbl -> CreateTypeInfo(This,szName,tkind,ppCTInfo) ) 

#define ICreateTypeLib_SetName(This,szName)	\
    ( (This)->lpVtbl -> SetName(This,szName) ) 

#define ICreateTypeLib_SetVersion(This,wMajorVerNum,wMinorVerNum)	\
    ( (This)->lpVtbl -> SetVersion(This,wMajorVerNum,wMinorVerNum) ) 

#define ICreateTypeLib_SetGuid(This,guid)	\
    ( (This)->lpVtbl -> SetGuid(This,guid) ) 

#define ICreateTypeLib_SetDocString(This,szDoc)	\
    ( (This)->lpVtbl -> SetDocString(This,szDoc) ) 

#define ICreateTypeLib_SetHelpFileName(This,szHelpFileName)	\
    ( (This)->lpVtbl -> SetHelpFileName(This,szHelpFileName) ) 

#define ICreateTypeLib_SetHelpContext(This,dwHelpContext)	\
    ( (This)->lpVtbl -> SetHelpContext(This,dwHelpContext) ) 

#define ICreateTypeLib_SetLcid(This,lcid)	\
    ( (This)->lpVtbl -> SetLcid(This,lcid) ) 

#define ICreateTypeLib_SetLibFlags(This,uLibFlags)	\
    ( (This)->lpVtbl -> SetLibFlags(This,uLibFlags) ) 

#define ICreateTypeLib_SaveAllChanges(This)	\
    ( (This)->lpVtbl -> SaveAllChanges(This) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ICreateTypeLib_INTERFACE_DEFINED__ */


#ifndef __ICreateTypeLib2_INTERFACE_DEFINED__
#define __ICreateTypeLib2_INTERFACE_DEFINED__

/* interface ICreateTypeLib2 */
/* [local][unique][uuid][object] */ 

typedef /* [unique] */ ICreateTypeLib2 *LPCREATETYPELIB2;


EXTERN_C const IID IID_ICreateTypeLib2;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("0002040F-0000-0000-C000-000000000046")
    ICreateTypeLib2 : public ICreateTypeLib
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE DeleteTypeInfo( 
            /* [annotation][in] */ 
            __RPC__in  LPOLESTR szName) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetCustData( 
            /* [in] */ REFGUID guid,
            /* [in] */ VARIANT *pVarVal) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetHelpStringContext( 
            /* [in] */ ULONG dwHelpStringContext) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetHelpStringDll( 
            /* [annotation][in] */ 
            __RPC__in  LPOLESTR szFileName) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ICreateTypeLib2Vtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            ICreateTypeLib2 * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            ICreateTypeLib2 * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            ICreateTypeLib2 * This);
        
        DECLSPEC_XFGVIRT(ICreateTypeLib, CreateTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *CreateTypeInfo )( 
            ICreateTypeLib2 * This,
            /* [annotation][in] */ 
            __RPC__in  LPOLESTR szName,
            /* [in] */ TYPEKIND tkind,
            /* [out] */ ICreateTypeInfo **ppCTInfo);
        
        DECLSPEC_XFGVIRT(ICreateTypeLib, SetName)
        HRESULT ( STDMETHODCALLTYPE *SetName )( 
            ICreateTypeLib2 * This,
            /* [annotation][in] */ 
            __RPC__in  LPOLESTR szName);
        
        DECLSPEC_XFGVIRT(ICreateTypeLib, SetVersion)
        HRESULT ( STDMETHODCALLTYPE *SetVersion )( 
            ICreateTypeLib2 * This,
            /* [in] */ WORD wMajorVerNum,
            /* [in] */ WORD wMinorVerNum);
        
        DECLSPEC_XFGVIRT(ICreateTypeLib, SetGuid)
        HRESULT ( STDMETHODCALLTYPE *SetGuid )( 
            ICreateTypeLib2 * This,
            /* [in] */ REFGUID guid);
        
        DECLSPEC_XFGVIRT(ICreateTypeLib, SetDocString)
        HRESULT ( STDMETHODCALLTYPE *SetDocString )( 
            ICreateTypeLib2 * This,
            /* [annotation][in] */ 
            __RPC__in  LPOLESTR szDoc);
        
        DECLSPEC_XFGVIRT(ICreateTypeLib, SetHelpFileName)
        HRESULT ( STDMETHODCALLTYPE *SetHelpFileName )( 
            ICreateTypeLib2 * This,
            /* [annotation][in] */ 
            __RPC__in  LPOLESTR szHelpFileName);
        
        DECLSPEC_XFGVIRT(ICreateTypeLib, SetHelpContext)
        HRESULT ( STDMETHODCALLTYPE *SetHelpContext )( 
            ICreateTypeLib2 * This,
            /* [in] */ DWORD dwHelpContext);
        
        DECLSPEC_XFGVIRT(ICreateTypeLib, SetLcid)
        HRESULT ( STDMETHODCALLTYPE *SetLcid )( 
            ICreateTypeLib2 * This,
            /* [in] */ LCID lcid);
        
        DECLSPEC_XFGVIRT(ICreateTypeLib, SetLibFlags)
        HRESULT ( STDMETHODCALLTYPE *SetLibFlags )( 
            ICreateTypeLib2 * This,
            /* [in] */ UINT uLibFlags);
        
        DECLSPEC_XFGVIRT(ICreateTypeLib, SaveAllChanges)
        HRESULT ( STDMETHODCALLTYPE *SaveAllChanges )( 
            ICreateTypeLib2 * This);
        
        DECLSPEC_XFGVIRT(ICreateTypeLib2, DeleteTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *DeleteTypeInfo )( 
            ICreateTypeLib2 * This,
            /* [annotation][in] */ 
            __RPC__in  LPOLESTR szName);
        
        DECLSPEC_XFGVIRT(ICreateTypeLib2, SetCustData)
        HRESULT ( STDMETHODCALLTYPE *SetCustData )( 
            ICreateTypeLib2 * This,
            /* [in] */ REFGUID guid,
            /* [in] */ VARIANT *pVarVal);
        
        DECLSPEC_XFGVIRT(ICreateTypeLib2, SetHelpStringContext)
        HRESULT ( STDMETHODCALLTYPE *SetHelpStringContext )( 
            ICreateTypeLib2 * This,
            /* [in] */ ULONG dwHelpStringContext);
        
        DECLSPEC_XFGVIRT(ICreateTypeLib2, SetHelpStringDll)
        HRESULT ( STDMETHODCALLTYPE *SetHelpStringDll )( 
            ICreateTypeLib2 * This,
            /* [annotation][in] */ 
            __RPC__in  LPOLESTR szFileName);
        
        END_INTERFACE
    } ICreateTypeLib2Vtbl;

    interface ICreateTypeLib2
    {
        CONST_VTBL struct ICreateTypeLib2Vtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ICreateTypeLib2_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ICreateTypeLib2_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ICreateTypeLib2_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ICreateTypeLib2_CreateTypeInfo(This,szName,tkind,ppCTInfo)	\
    ( (This)->lpVtbl -> CreateTypeInfo(This,szName,tkind,ppCTInfo) ) 

#define ICreateTypeLib2_SetName(This,szName)	\
    ( (This)->lpVtbl -> SetName(This,szName) ) 

#define ICreateTypeLib2_SetVersion(This,wMajorVerNum,wMinorVerNum)	\
    ( (This)->lpVtbl -> SetVersion(This,wMajorVerNum,wMinorVerNum) ) 

#define ICreateTypeLib2_SetGuid(This,guid)	\
    ( (This)->lpVtbl -> SetGuid(This,guid) ) 

#define ICreateTypeLib2_SetDocString(This,szDoc)	\
    ( (This)->lpVtbl -> SetDocString(This,szDoc) ) 

#define ICreateTypeLib2_SetHelpFileName(This,szHelpFileName)	\
    ( (This)->lpVtbl -> SetHelpFileName(This,szHelpFileName) ) 

#define ICreateTypeLib2_SetHelpContext(This,dwHelpContext)	\
    ( (This)->lpVtbl -> SetHelpContext(This,dwHelpContext) ) 

#define ICreateTypeLib2_SetLcid(This,lcid)	\
    ( (This)->lpVtbl -> SetLcid(This,lcid) ) 

#define ICreateTypeLib2_SetLibFlags(This,uLibFlags)	\
    ( (This)->lpVtbl -> SetLibFlags(This,uLibFlags) ) 

#define ICreateTypeLib2_SaveAllChanges(This)	\
    ( (This)->lpVtbl -> SaveAllChanges(This) ) 


#define ICreateTypeLib2_DeleteTypeInfo(This,szName)	\
    ( (This)->lpVtbl -> DeleteTypeInfo(This,szName) ) 

#define ICreateTypeLib2_SetCustData(This,guid,pVarVal)	\
    ( (This)->lpVtbl -> SetCustData(This,guid,pVarVal) ) 

#define ICreateTypeLib2_SetHelpStringContext(This,dwHelpStringContext)	\
    ( (This)->lpVtbl -> SetHelpStringContext(This,dwHelpStringContext) ) 

#define ICreateTypeLib2_SetHelpStringDll(This,szFileName)	\
    ( (This)->lpVtbl -> SetHelpStringDll(This,szFileName) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ICreateTypeLib2_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_oaidl_0000_0005 */
/* [local] */ 

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM) */
#pragma endregion
#pragma region Application Family or OneCore Family or Games Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES)


extern RPC_IF_HANDLE __MIDL_itf_oaidl_0000_0005_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_oaidl_0000_0005_v0_0_s_ifspec;

#ifndef __IDispatch_INTERFACE_DEFINED__
#define __IDispatch_INTERFACE_DEFINED__

/* interface IDispatch */
/* [unique][uuid][object] */ 

typedef /* [unique] */  __RPC_unique_pointer IDispatch *LPDISPATCH;

/* DISPID reserved to indicate an "unknown" name */
/* only reserved for data members (properties); reused as a method dispid below */
#define	DISPID_UNKNOWN	( -1 )

/* DISPID reserved for the "value" property */
#define	DISPID_VALUE	( 0 )

/* The following DISPID is reserved to indicate the param
 * that is the right-hand-side (or "put" value) of a PropertyPut
 */
#define	DISPID_PROPERTYPUT	( -3 )

/* DISPID reserved for the standard "NewEnum" method */
#define	DISPID_NEWENUM	( -4 )

/* DISPID reserved for the standard "Evaluate" method */
#define	DISPID_EVALUATE	( -5 )

#define	DISPID_CONSTRUCTOR	( -6 )

#define	DISPID_DESTRUCTOR	( -7 )

#define	DISPID_COLLECT	( -8 )

/* The range -500 through -999 is reserved for Controls */
/* The range 0x80010000 through 0x8001FFFF is reserved for Controls */
/* The range -5000 through -5499 is reserved for ActiveX Accessability */
/* The range -2000 through -2499 is reserved for VB5 */
/* The range -3900 through -3999 is reserved for Forms */
/* The range -5500 through -5550 is reserved for Forms */
/* The remainder of the negative DISPIDs are reserved for future use */

EXTERN_C const IID IID_IDispatch;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("00020400-0000-0000-C000-000000000046")
    IDispatch : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetTypeInfoCount( 
            /* [out] */ __RPC__out UINT *pctinfo) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetTypeInfo( 
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetIDsOfNames( 
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId) = 0;
        
        virtual /* [local] */ HRESULT STDMETHODCALLTYPE Invoke( 
            /* [annotation][in] */ 
            _In_  DISPID dispIdMember,
            /* [annotation][in] */ 
            _In_  REFIID riid,
            /* [annotation][in] */ 
            _In_  LCID lcid,
            /* [annotation][in] */ 
            _In_  WORD wFlags,
            /* [annotation][out][in] */ 
            _In_  DISPPARAMS *pDispParams,
            /* [annotation][out] */ 
            _Out_opt_  VARIANT *pVarResult,
            /* [annotation][out] */ 
            _Out_opt_  EXCEPINFO *pExcepInfo,
            /* [annotation][out] */ 
            _Out_opt_  UINT *puArgErr) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IDispatchVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IDispatch * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IDispatch * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IDispatch * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IDispatch * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IDispatch * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IDispatch * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IDispatch * This,
            /* [annotation][in] */ 
            _In_  DISPID dispIdMember,
            /* [annotation][in] */ 
            _In_  REFIID riid,
            /* [annotation][in] */ 
            _In_  LCID lcid,
            /* [annotation][in] */ 
            _In_  WORD wFlags,
            /* [annotation][out][in] */ 
            _In_  DISPPARAMS *pDispParams,
            /* [annotation][out] */ 
            _Out_opt_  VARIANT *pVarResult,
            /* [annotation][out] */ 
            _Out_opt_  EXCEPINFO *pExcepInfo,
            /* [annotation][out] */ 
            _Out_opt_  UINT *puArgErr);
        
        END_INTERFACE
    } IDispatchVtbl;

    interface IDispatch
    {
        CONST_VTBL struct IDispatchVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IDispatch_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IDispatch_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IDispatch_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IDispatch_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IDispatch_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IDispatch_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IDispatch_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */



/* [call_as] */ HRESULT STDMETHODCALLTYPE IDispatch_RemoteInvoke_Proxy( 
    __RPC__in IDispatch * This,
    /* [in] */ DISPID dispIdMember,
    /* [in] */ __RPC__in REFIID riid,
    /* [in] */ LCID lcid,
    /* [in] */ DWORD dwFlags,
    /* [in] */ __RPC__in DISPPARAMS *pDispParams,
    /* [out] */ __RPC__out VARIANT *pVarResult,
    /* [out] */ __RPC__out EXCEPINFO *pExcepInfo,
    /* [out] */ __RPC__out UINT *pArgErr,
    /* [in] */ UINT cVarRef,
    /* [size_is][in] */ __RPC__in_ecount_full(cVarRef) UINT *rgVarRefIdx,
    /* [size_is][out][in] */ __RPC__inout_ecount_full(cVarRef) VARIANTARG *rgVarRef);


void __RPC_STUB IDispatch_RemoteInvoke_Stub(
    IRpcStubBuffer *This,
    IRpcChannelBuffer *_pRpcChannelBuffer,
    PRPC_MESSAGE _pRpcMessage,
    DWORD *_pdwStubPhase);



#endif 	/* __IDispatch_INTERFACE_DEFINED__ */


#ifndef __IEnumVARIANT_INTERFACE_DEFINED__
#define __IEnumVARIANT_INTERFACE_DEFINED__

/* interface IEnumVARIANT */
/* [unique][uuid][object] */ 

typedef /* [unique] */  __RPC_unique_pointer IEnumVARIANT *LPENUMVARIANT;


EXTERN_C const IID IID_IEnumVARIANT;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("00020404-0000-0000-C000-000000000046")
    IEnumVARIANT : public IUnknown
    {
    public:
        virtual /* [local] */ HRESULT STDMETHODCALLTYPE Next( 
            /* [in] */ ULONG celt,
            /* [length_is][size_is][out] */ VARIANT *rgVar,
            /* [out] */ ULONG *pCeltFetched) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Skip( 
            /* [in] */ ULONG celt) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Reset( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Clone( 
            /* [out] */ __RPC__deref_out_opt IEnumVARIANT **ppEnum) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IEnumVARIANTVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IEnumVARIANT * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IEnumVARIANT * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IEnumVARIANT * This);
        
        DECLSPEC_XFGVIRT(IEnumVARIANT, Next)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Next )( 
            IEnumVARIANT * This,
            /* [in] */ ULONG celt,
            /* [length_is][size_is][out] */ VARIANT *rgVar,
            /* [out] */ ULONG *pCeltFetched);
        
        DECLSPEC_XFGVIRT(IEnumVARIANT, Skip)
        HRESULT ( STDMETHODCALLTYPE *Skip )( 
            __RPC__in IEnumVARIANT * This,
            /* [in] */ ULONG celt);
        
        DECLSPEC_XFGVIRT(IEnumVARIANT, Reset)
        HRESULT ( STDMETHODCALLTYPE *Reset )( 
            __RPC__in IEnumVARIANT * This);
        
        DECLSPEC_XFGVIRT(IEnumVARIANT, Clone)
        HRESULT ( STDMETHODCALLTYPE *Clone )( 
            __RPC__in IEnumVARIANT * This,
            /* [out] */ __RPC__deref_out_opt IEnumVARIANT **ppEnum);
        
        END_INTERFACE
    } IEnumVARIANTVtbl;

    interface IEnumVARIANT
    {
        CONST_VTBL struct IEnumVARIANTVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IEnumVARIANT_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IEnumVARIANT_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IEnumVARIANT_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IEnumVARIANT_Next(This,celt,rgVar,pCeltFetched)	\
    ( (This)->lpVtbl -> Next(This,celt,rgVar,pCeltFetched) ) 

#define IEnumVARIANT_Skip(This,celt)	\
    ( (This)->lpVtbl -> Skip(This,celt) ) 

#define IEnumVARIANT_Reset(This)	\
    ( (This)->lpVtbl -> Reset(This) ) 

#define IEnumVARIANT_Clone(This,ppEnum)	\
    ( (This)->lpVtbl -> Clone(This,ppEnum) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */



/* [call_as] */ HRESULT STDMETHODCALLTYPE IEnumVARIANT_RemoteNext_Proxy( 
    __RPC__in IEnumVARIANT * This,
    /* [in] */ ULONG celt,
    /* [length_is][size_is][out] */ __RPC__out_ecount_part(celt, *pCeltFetched) VARIANT *rgVar,
    /* [out] */ __RPC__out ULONG *pCeltFetched);


void __RPC_STUB IEnumVARIANT_RemoteNext_Stub(
    IRpcStubBuffer *This,
    IRpcChannelBuffer *_pRpcChannelBuffer,
    PRPC_MESSAGE _pRpcMessage,
    DWORD *_pdwStubPhase);



#endif 	/* __IEnumVARIANT_INTERFACE_DEFINED__ */


#ifndef __ITypeComp_INTERFACE_DEFINED__
#define __ITypeComp_INTERFACE_DEFINED__

/* interface ITypeComp */
/* [unique][uuid][object] */ 

typedef /* [unique] */  __RPC_unique_pointer ITypeComp *LPTYPECOMP;

typedef /* [v1_enum] */ 
enum tagDESCKIND
    {
        DESCKIND_NONE	= 0,
        DESCKIND_FUNCDESC	= ( DESCKIND_NONE + 1 ) ,
        DESCKIND_VARDESC	= ( DESCKIND_FUNCDESC + 1 ) ,
        DESCKIND_TYPECOMP	= ( DESCKIND_VARDESC + 1 ) ,
        DESCKIND_IMPLICITAPPOBJ	= ( DESCKIND_TYPECOMP + 1 ) ,
        DESCKIND_MAX	= ( DESCKIND_IMPLICITAPPOBJ + 1 ) 
    } 	DESCKIND;

typedef union tagBINDPTR
    {
    FUNCDESC *lpfuncdesc;
    VARDESC *lpvardesc;
    ITypeComp *lptcomp;
    } 	BINDPTR;

typedef union tagBINDPTR *LPBINDPTR;


EXTERN_C const IID IID_ITypeComp;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("00020403-0000-0000-C000-000000000046")
    ITypeComp : public IUnknown
    {
    public:
        virtual /* [local] */ HRESULT STDMETHODCALLTYPE Bind( 
            /* [annotation][in] */ 
            __RPC__in  LPOLESTR szName,
            /* [in] */ ULONG lHashVal,
            /* [in] */ WORD wFlags,
            /* [out] */ ITypeInfo **ppTInfo,
            /* [out] */ DESCKIND *pDescKind,
            /* [out] */ BINDPTR *pBindPtr) = 0;
        
        virtual /* [local] */ HRESULT STDMETHODCALLTYPE BindType( 
            /* [annotation][in] */ 
            __RPC__in  LPOLESTR szName,
            /* [in] */ ULONG lHashVal,
            /* [out] */ ITypeInfo **ppTInfo,
            /* [out] */ ITypeComp **ppTComp) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ITypeCompVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in ITypeComp * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in ITypeComp * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in ITypeComp * This);
        
        DECLSPEC_XFGVIRT(ITypeComp, Bind)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Bind )( 
            ITypeComp * This,
            /* [annotation][in] */ 
            __RPC__in  LPOLESTR szName,
            /* [in] */ ULONG lHashVal,
            /* [in] */ WORD wFlags,
            /* [out] */ ITypeInfo **ppTInfo,
            /* [out] */ DESCKIND *pDescKind,
            /* [out] */ BINDPTR *pBindPtr);
        
        DECLSPEC_XFGVIRT(ITypeComp, BindType)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *BindType )( 
            ITypeComp * This,
            /* [annotation][in] */ 
            __RPC__in  LPOLESTR szName,
            /* [in] */ ULONG lHashVal,
            /* [out] */ ITypeInfo **ppTInfo,
            /* [out] */ ITypeComp **ppTComp);
        
        END_INTERFACE
    } ITypeCompVtbl;

    interface ITypeComp
    {
        CONST_VTBL struct ITypeCompVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ITypeComp_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ITypeComp_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ITypeComp_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ITypeComp_Bind(This,szName,lHashVal,wFlags,ppTInfo,pDescKind,pBindPtr)	\
    ( (This)->lpVtbl -> Bind(This,szName,lHashVal,wFlags,ppTInfo,pDescKind,pBindPtr) ) 

#define ITypeComp_BindType(This,szName,lHashVal,ppTInfo,ppTComp)	\
    ( (This)->lpVtbl -> BindType(This,szName,lHashVal,ppTInfo,ppTComp) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */



/* [call_as] */ HRESULT STDMETHODCALLTYPE ITypeComp_RemoteBind_Proxy( 
    __RPC__in ITypeComp * This,
    /* [in] */ __RPC__in LPOLESTR szName,
    /* [in] */ ULONG lHashVal,
    /* [in] */ WORD wFlags,
    /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo,
    /* [out] */ __RPC__out DESCKIND *pDescKind,
    /* [out] */ __RPC__deref_out_opt LPFUNCDESC *ppFuncDesc,
    /* [out] */ __RPC__deref_out_opt LPVARDESC *ppVarDesc,
    /* [out] */ __RPC__deref_out_opt ITypeComp **ppTypeComp,
    /* [out] */ __RPC__out CLEANLOCALSTORAGE *pDummy);


void __RPC_STUB ITypeComp_RemoteBind_Stub(
    IRpcStubBuffer *This,
    IRpcChannelBuffer *_pRpcChannelBuffer,
    PRPC_MESSAGE _pRpcMessage,
    DWORD *_pdwStubPhase);


/* [call_as] */ HRESULT STDMETHODCALLTYPE ITypeComp_RemoteBindType_Proxy( 
    __RPC__in ITypeComp * This,
    /* [in] */ __RPC__in LPOLESTR szName,
    /* [in] */ ULONG lHashVal,
    /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);


void __RPC_STUB ITypeComp_RemoteBindType_Stub(
    IRpcStubBuffer *This,
    IRpcChannelBuffer *_pRpcChannelBuffer,
    PRPC_MESSAGE _pRpcMessage,
    DWORD *_pdwStubPhase);



#endif 	/* __ITypeComp_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_oaidl_0000_0008 */
/* [local] */ 

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES) */
#pragma endregion
#pragma region Desktop Family or OneCore Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM)


extern RPC_IF_HANDLE __MIDL_itf_oaidl_0000_0008_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_oaidl_0000_0008_v0_0_s_ifspec;

#ifndef __ITypeInfo_INTERFACE_DEFINED__
#define __ITypeInfo_INTERFACE_DEFINED__

/* interface ITypeInfo */
/* [unique][uuid][object] */ 

typedef /* [unique] */  __RPC_unique_pointer ITypeInfo *LPTYPEINFO;


EXTERN_C const IID IID_ITypeInfo;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("00020401-0000-0000-C000-000000000046")
    ITypeInfo : public IUnknown
    {
    public:
        virtual /* [local] */ HRESULT STDMETHODCALLTYPE GetTypeAttr( 
            /* [out] */ TYPEATTR **ppTypeAttr) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetTypeComp( 
            /* [out] */ __RPC__deref_out_opt ITypeComp **ppTComp) = 0;
        
        virtual /* [local] */ HRESULT STDMETHODCALLTYPE GetFuncDesc( 
            /* [in] */ UINT index,
            /* [out] */ FUNCDESC **ppFuncDesc) = 0;
        
        virtual /* [local] */ HRESULT STDMETHODCALLTYPE GetVarDesc( 
            /* [in] */ UINT index,
            /* [out] */ VARDESC **ppVarDesc) = 0;
        
        virtual /* [local] */ HRESULT STDMETHODCALLTYPE GetNames( 
            /* [in] */ MEMBERID memid,
            /* [annotation][length_is][size_is][out] */ 
            __RPC__out_ecount_part(cMaxNames, *pcNames)  BSTR *rgBstrNames,
            /* [in] */ UINT cMaxNames,
            /* [annotation][out] */ 
            __RPC__out  UINT *pcNames) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetRefTypeOfImplType( 
            /* [in] */ UINT index,
            /* [out] */ __RPC__out HREFTYPE *pRefType) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetImplTypeFlags( 
            /* [in] */ UINT index,
            /* [out] */ __RPC__out INT *pImplTypeFlags) = 0;
        
        virtual /* [local] */ HRESULT STDMETHODCALLTYPE GetIDsOfNames( 
            /* [annotation][size_is][in] */ 
            __RPC__in_ecount(cNames)  LPOLESTR *rgszNames,
            /* [in] */ UINT cNames,
            /* [size_is][out] */ MEMBERID *pMemId) = 0;
        
        virtual /* [local] */ HRESULT STDMETHODCALLTYPE Invoke( 
            /* [in] */ PVOID pvInstance,
            /* [in] */ MEMBERID memid,
            /* [in] */ WORD wFlags,
            /* [out][in] */ DISPPARAMS *pDispParams,
            /* [out] */ VARIANT *pVarResult,
            /* [out] */ EXCEPINFO *pExcepInfo,
            /* [out] */ UINT *puArgErr) = 0;
        
        virtual /* [local] */ HRESULT STDMETHODCALLTYPE GetDocumentation( 
            /* [in] */ MEMBERID memid,
            /* [annotation][out] */ 
            _Outptr_opt_  BSTR *pBstrName,
            /* [annotation][out] */ 
            _Outptr_opt_  BSTR *pBstrDocString,
            /* [out] */ DWORD *pdwHelpContext,
            /* [annotation][out] */ 
            _Outptr_opt_  BSTR *pBstrHelpFile) = 0;
        
        virtual /* [local] */ HRESULT STDMETHODCALLTYPE GetDllEntry( 
            /* [in] */ MEMBERID memid,
            /* [in] */ INVOKEKIND invKind,
            /* [annotation][out] */ 
            _Outptr_opt_  BSTR *pBstrDllName,
            /* [annotation][out] */ 
            _Outptr_opt_  BSTR *pBstrName,
            /* [out] */ WORD *pwOrdinal) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetRefTypeInfo( 
            /* [in] */ HREFTYPE hRefType,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo) = 0;
        
        virtual /* [local] */ HRESULT STDMETHODCALLTYPE AddressOfMember( 
            /* [in] */ MEMBERID memid,
            /* [in] */ INVOKEKIND invKind,
            /* [out] */ PVOID *ppv) = 0;
        
        virtual /* [local] */ HRESULT STDMETHODCALLTYPE CreateInstance( 
            /* [in] */ IUnknown *pUnkOuter,
            /* [in] */ REFIID riid,
            /* [iid_is][out] */ PVOID *ppvObj) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetMops( 
            /* [in] */ MEMBERID memid,
            /* [out] */ __RPC__deref_out_opt BSTR *pBstrMops) = 0;
        
        virtual /* [local] */ HRESULT STDMETHODCALLTYPE GetContainingTypeLib( 
            /* [out] */ ITypeLib **ppTLib,
            /* [out] */ UINT *pIndex) = 0;
        
        virtual /* [local] */ void STDMETHODCALLTYPE ReleaseTypeAttr( 
            /* [in] */ TYPEATTR *pTypeAttr) = 0;
        
        virtual /* [local] */ void STDMETHODCALLTYPE ReleaseFuncDesc( 
            /* [in] */ FUNCDESC *pFuncDesc) = 0;
        
        virtual /* [local] */ void STDMETHODCALLTYPE ReleaseVarDesc( 
            /* [in] */ VARDESC *pVarDesc) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ITypeInfoVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in ITypeInfo * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in ITypeInfo * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in ITypeInfo * This);
        
        DECLSPEC_XFGVIRT(ITypeInfo, GetTypeAttr)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *GetTypeAttr )( 
            ITypeInfo * This,
            /* [out] */ TYPEATTR **ppTypeAttr);
        
        DECLSPEC_XFGVIRT(ITypeInfo, GetTypeComp)
        HRESULT ( STDMETHODCALLTYPE *GetTypeComp )( 
            __RPC__in ITypeInfo * This,
            /* [out] */ __RPC__deref_out_opt ITypeComp **ppTComp);
        
        DECLSPEC_XFGVIRT(ITypeInfo, GetFuncDesc)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *GetFuncDesc )( 
            ITypeInfo * This,
            /* [in] */ UINT index,
            /* [out] */ FUNCDESC **ppFuncDesc);
        
        DECLSPEC_XFGVIRT(ITypeInfo, GetVarDesc)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *GetVarDesc )( 
            ITypeInfo * This,
            /* [in] */ UINT index,
            /* [out] */ VARDESC **ppVarDesc);
        
        DECLSPEC_XFGVIRT(ITypeInfo, GetNames)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *GetNames )( 
            ITypeInfo * This,
            /* [in] */ MEMBERID memid,
            /* [annotation][length_is][size_is][out] */ 
            __RPC__out_ecount_part(cMaxNames, *pcNames)  BSTR *rgBstrNames,
            /* [in] */ UINT cMaxNames,
            /* [annotation][out] */ 
            __RPC__out  UINT *pcNames);
        
        DECLSPEC_XFGVIRT(ITypeInfo, GetRefTypeOfImplType)
        HRESULT ( STDMETHODCALLTYPE *GetRefTypeOfImplType )( 
            __RPC__in ITypeInfo * This,
            /* [in] */ UINT index,
            /* [out] */ __RPC__out HREFTYPE *pRefType);
        
        DECLSPEC_XFGVIRT(ITypeInfo, GetImplTypeFlags)
        HRESULT ( STDMETHODCALLTYPE *GetImplTypeFlags )( 
            __RPC__in ITypeInfo * This,
            /* [in] */ UINT index,
            /* [out] */ __RPC__out INT *pImplTypeFlags);
        
        DECLSPEC_XFGVIRT(ITypeInfo, GetIDsOfNames)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            ITypeInfo * This,
            /* [annotation][size_is][in] */ 
            __RPC__in_ecount(cNames)  LPOLESTR *rgszNames,
            /* [in] */ UINT cNames,
            /* [size_is][out] */ MEMBERID *pMemId);
        
        DECLSPEC_XFGVIRT(ITypeInfo, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            ITypeInfo * This,
            /* [in] */ PVOID pvInstance,
            /* [in] */ MEMBERID memid,
            /* [in] */ WORD wFlags,
            /* [out][in] */ DISPPARAMS *pDispParams,
            /* [out] */ VARIANT *pVarResult,
            /* [out] */ EXCEPINFO *pExcepInfo,
            /* [out] */ UINT *puArgErr);
        
        DECLSPEC_XFGVIRT(ITypeInfo, GetDocumentation)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *GetDocumentation )( 
            ITypeInfo * This,
            /* [in] */ MEMBERID memid,
            /* [annotation][out] */ 
            _Outptr_opt_  BSTR *pBstrName,
            /* [annotation][out] */ 
            _Outptr_opt_  BSTR *pBstrDocString,
            /* [out] */ DWORD *pdwHelpContext,
            /* [annotation][out] */ 
            _Outptr_opt_  BSTR *pBstrHelpFile);
        
        DECLSPEC_XFGVIRT(ITypeInfo, GetDllEntry)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *GetDllEntry )( 
            ITypeInfo * This,
            /* [in] */ MEMBERID memid,
            /* [in] */ INVOKEKIND invKind,
            /* [annotation][out] */ 
            _Outptr_opt_  BSTR *pBstrDllName,
            /* [annotation][out] */ 
            _Outptr_opt_  BSTR *pBstrName,
            /* [out] */ WORD *pwOrdinal);
        
        DECLSPEC_XFGVIRT(ITypeInfo, GetRefTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetRefTypeInfo )( 
            __RPC__in ITypeInfo * This,
            /* [in] */ HREFTYPE hRefType,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(ITypeInfo, AddressOfMember)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *AddressOfMember )( 
            ITypeInfo * This,
            /* [in] */ MEMBERID memid,
            /* [in] */ INVOKEKIND invKind,
            /* [out] */ PVOID *ppv);
        
        DECLSPEC_XFGVIRT(ITypeInfo, CreateInstance)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *CreateInstance )( 
            ITypeInfo * This,
            /* [in] */ IUnknown *pUnkOuter,
            /* [in] */ REFIID riid,
            /* [iid_is][out] */ PVOID *ppvObj);
        
        DECLSPEC_XFGVIRT(ITypeInfo, GetMops)
        HRESULT ( STDMETHODCALLTYPE *GetMops )( 
            __RPC__in ITypeInfo * This,
            /* [in] */ MEMBERID memid,
            /* [out] */ __RPC__deref_out_opt BSTR *pBstrMops);
        
        DECLSPEC_XFGVIRT(ITypeInfo, GetContainingTypeLib)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *GetContainingTypeLib )( 
            ITypeInfo * This,
            /* [out] */ ITypeLib **ppTLib,
            /* [out] */ UINT *pIndex);
        
        DECLSPEC_XFGVIRT(ITypeInfo, ReleaseTypeAttr)
        /* [local] */ void ( STDMETHODCALLTYPE *ReleaseTypeAttr )( 
            ITypeInfo * This,
            /* [in] */ TYPEATTR *pTypeAttr);
        
        DECLSPEC_XFGVIRT(ITypeInfo, ReleaseFuncDesc)
        /* [local] */ void ( STDMETHODCALLTYPE *ReleaseFuncDesc )( 
            ITypeInfo * This,
            /* [in] */ FUNCDESC *pFuncDesc);
        
        DECLSPEC_XFGVIRT(ITypeInfo, ReleaseVarDesc)
        /* [local] */ void ( STDMETHODCALLTYPE *ReleaseVarDesc )( 
            ITypeInfo * This,
            /* [in] */ VARDESC *pVarDesc);
        
        END_INTERFACE
    } ITypeInfoVtbl;

    interface ITypeInfo
    {
        CONST_VTBL struct ITypeInfoVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ITypeInfo_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ITypeInfo_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ITypeInfo_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ITypeInfo_GetTypeAttr(This,ppTypeAttr)	\
    ( (This)->lpVtbl -> GetTypeAttr(This,ppTypeAttr) ) 

#define ITypeInfo_GetTypeComp(This,ppTComp)	\
    ( (This)->lpVtbl -> GetTypeComp(This,ppTComp) ) 

#define ITypeInfo_GetFuncDesc(This,index,ppFuncDesc)	\
    ( (This)->lpVtbl -> GetFuncDesc(This,index,ppFuncDesc) ) 

#define ITypeInfo_GetVarDesc(This,index,ppVarDesc)	\
    ( (This)->lpVtbl -> GetVarDesc(This,index,ppVarDesc) ) 

#define ITypeInfo_GetNames(This,memid,rgBstrNames,cMaxNames,pcNames)	\
    ( (This)->lpVtbl -> GetNames(This,memid,rgBstrNames,cMaxNames,pcNames) ) 

#define ITypeInfo_GetRefTypeOfImplType(This,index,pRefType)	\
    ( (This)->lpVtbl -> GetRefTypeOfImplType(This,index,pRefType) ) 

#define ITypeInfo_GetImplTypeFlags(This,index,pImplTypeFlags)	\
    ( (This)->lpVtbl -> GetImplTypeFlags(This,index,pImplTypeFlags) ) 

#define ITypeInfo_GetIDsOfNames(This,rgszNames,cNames,pMemId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,rgszNames,cNames,pMemId) ) 

#define ITypeInfo_Invoke(This,pvInstance,memid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,pvInstance,memid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 

#define ITypeInfo_GetDocumentation(This,memid,pBstrName,pBstrDocString,pdwHelpContext,pBstrHelpFile)	\
    ( (This)->lpVtbl -> GetDocumentation(This,memid,pBstrName,pBstrDocString,pdwHelpContext,pBstrHelpFile) ) 

#define ITypeInfo_GetDllEntry(This,memid,invKind,pBstrDllName,pBstrName,pwOrdinal)	\
    ( (This)->lpVtbl -> GetDllEntry(This,memid,invKind,pBstrDllName,pBstrName,pwOrdinal) ) 

#define ITypeInfo_GetRefTypeInfo(This,hRefType,ppTInfo)	\
    ( (This)->lpVtbl -> GetRefTypeInfo(This,hRefType,ppTInfo) ) 

#define ITypeInfo_AddressOfMember(This,memid,invKind,ppv)	\
    ( (This)->lpVtbl -> AddressOfMember(This,memid,invKind,ppv) ) 

#define ITypeInfo_CreateInstance(This,pUnkOuter,riid,ppvObj)	\
    ( (This)->lpVtbl -> CreateInstance(This,pUnkOuter,riid,ppvObj) ) 

#define ITypeInfo_GetMops(This,memid,pBstrMops)	\
    ( (This)->lpVtbl -> GetMops(This,memid,pBstrMops) ) 

#define ITypeInfo_GetContainingTypeLib(This,ppTLib,pIndex)	\
    ( (This)->lpVtbl -> GetContainingTypeLib(This,ppTLib,pIndex) ) 

#define ITypeInfo_ReleaseTypeAttr(This,pTypeAttr)	\
    ( (This)->lpVtbl -> ReleaseTypeAttr(This,pTypeAttr) ) 

#define ITypeInfo_ReleaseFuncDesc(This,pFuncDesc)	\
    ( (This)->lpVtbl -> ReleaseFuncDesc(This,pFuncDesc) ) 

#define ITypeInfo_ReleaseVarDesc(This,pVarDesc)	\
    ( (This)->lpVtbl -> ReleaseVarDesc(This,pVarDesc) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */



/* [call_as] */ HRESULT STDMETHODCALLTYPE ITypeInfo_RemoteGetTypeAttr_Proxy( 
    __RPC__in ITypeInfo * This,
    /* [out] */ __RPC__deref_out_opt LPTYPEATTR *ppTypeAttr,
    /* [out] */ __RPC__out CLEANLOCALSTORAGE *pDummy);


void __RPC_STUB ITypeInfo_RemoteGetTypeAttr_Stub(
    IRpcStubBuffer *This,
    IRpcChannelBuffer *_pRpcChannelBuffer,
    PRPC_MESSAGE _pRpcMessage,
    DWORD *_pdwStubPhase);


/* [call_as] */ HRESULT STDMETHODCALLTYPE ITypeInfo_RemoteGetFuncDesc_Proxy( 
    __RPC__in ITypeInfo * This,
    /* [in] */ UINT index,
    /* [out] */ __RPC__deref_out_opt LPFUNCDESC *ppFuncDesc,
    /* [out] */ __RPC__out CLEANLOCALSTORAGE *pDummy);


void __RPC_STUB ITypeInfo_RemoteGetFuncDesc_Stub(
    IRpcStubBuffer *This,
    IRpcChannelBuffer *_pRpcChannelBuffer,
    PRPC_MESSAGE _pRpcMessage,
    DWORD *_pdwStubPhase);


/* [call_as] */ HRESULT STDMETHODCALLTYPE ITypeInfo_RemoteGetVarDesc_Proxy( 
    __RPC__in ITypeInfo * This,
    /* [in] */ UINT index,
    /* [out] */ __RPC__deref_out_opt LPVARDESC *ppVarDesc,
    /* [out] */ __RPC__out CLEANLOCALSTORAGE *pDummy);


void __RPC_STUB ITypeInfo_RemoteGetVarDesc_Stub(
    IRpcStubBuffer *This,
    IRpcChannelBuffer *_pRpcChannelBuffer,
    PRPC_MESSAGE _pRpcMessage,
    DWORD *_pdwStubPhase);


/* [call_as] */ HRESULT STDMETHODCALLTYPE ITypeInfo_RemoteGetNames_Proxy( 
    __RPC__in ITypeInfo * This,
    /* [in] */ MEMBERID memid,
    /* [length_is][size_is][out] */ __RPC__out_ecount_part(cMaxNames, *pcNames) BSTR *rgBstrNames,
    /* [in] */ UINT cMaxNames,
    /* [out] */ __RPC__out UINT *pcNames);


void __RPC_STUB ITypeInfo_RemoteGetNames_Stub(
    IRpcStubBuffer *This,
    IRpcChannelBuffer *_pRpcChannelBuffer,
    PRPC_MESSAGE _pRpcMessage,
    DWORD *_pdwStubPhase);


/* [nocode][call_as] */ HRESULT STDMETHODCALLTYPE ITypeInfo_LocalGetIDsOfNames_Proxy( 
    __RPC__in ITypeInfo * This);


void __RPC_STUB ITypeInfo_LocalGetIDsOfNames_Stub(
    IRpcStubBuffer *This,
    IRpcChannelBuffer *_pRpcChannelBuffer,
    PRPC_MESSAGE _pRpcMessage,
    DWORD *_pdwStubPhase);


/* [nocode][call_as] */ HRESULT STDMETHODCALLTYPE ITypeInfo_LocalInvoke_Proxy( 
    __RPC__in ITypeInfo * This);


void __RPC_STUB ITypeInfo_LocalInvoke_Stub(
    IRpcStubBuffer *This,
    IRpcChannelBuffer *_pRpcChannelBuffer,
    PRPC_MESSAGE _pRpcMessage,
    DWORD *_pdwStubPhase);


/* [call_as] */ HRESULT STDMETHODCALLTYPE ITypeInfo_RemoteGetDocumentation_Proxy( 
    __RPC__in ITypeInfo * This,
    /* [in] */ MEMBERID memid,
    /* [in] */ DWORD refPtrFlags,
    /* [out] */ __RPC__deref_out_opt BSTR *pBstrName,
    /* [out] */ __RPC__deref_out_opt BSTR *pBstrDocString,
    /* [out] */ __RPC__out DWORD *pdwHelpContext,
    /* [out] */ __RPC__deref_out_opt BSTR *pBstrHelpFile);


void __RPC_STUB ITypeInfo_RemoteGetDocumentation_Stub(
    IRpcStubBuffer *This,
    IRpcChannelBuffer *_pRpcChannelBuffer,
    PRPC_MESSAGE _pRpcMessage,
    DWORD *_pdwStubPhase);


/* [call_as] */ HRESULT STDMETHODCALLTYPE ITypeInfo_RemoteGetDllEntry_Proxy( 
    __RPC__in ITypeInfo * This,
    /* [in] */ MEMBERID memid,
    /* [in] */ INVOKEKIND invKind,
    /* [in] */ DWORD refPtrFlags,
    /* [out] */ __RPC__deref_out_opt BSTR *pBstrDllName,
    /* [out] */ __RPC__deref_out_opt BSTR *pBstrName,
    /* [out] */ __RPC__out WORD *pwOrdinal);


void __RPC_STUB ITypeInfo_RemoteGetDllEntry_Stub(
    IRpcStubBuffer *This,
    IRpcChannelBuffer *_pRpcChannelBuffer,
    PRPC_MESSAGE _pRpcMessage,
    DWORD *_pdwStubPhase);


/* [nocode][call_as] */ HRESULT STDMETHODCALLTYPE ITypeInfo_LocalAddressOfMember_Proxy( 
    __RPC__in ITypeInfo * This);


void __RPC_STUB ITypeInfo_LocalAddressOfMember_Stub(
    IRpcStubBuffer *This,
    IRpcChannelBuffer *_pRpcChannelBuffer,
    PRPC_MESSAGE _pRpcMessage,
    DWORD *_pdwStubPhase);


/* [call_as] */ HRESULT STDMETHODCALLTYPE ITypeInfo_RemoteCreateInstance_Proxy( 
    __RPC__in ITypeInfo * This,
    /* [in] */ __RPC__in REFIID riid,
    /* [iid_is][out] */ __RPC__deref_out_opt IUnknown **ppvObj);


void __RPC_STUB ITypeInfo_RemoteCreateInstance_Stub(
    IRpcStubBuffer *This,
    IRpcChannelBuffer *_pRpcChannelBuffer,
    PRPC_MESSAGE _pRpcMessage,
    DWORD *_pdwStubPhase);


/* [call_as] */ HRESULT STDMETHODCALLTYPE ITypeInfo_RemoteGetContainingTypeLib_Proxy( 
    __RPC__in ITypeInfo * This,
    /* [out] */ __RPC__deref_out_opt ITypeLib **ppTLib,
    /* [out] */ __RPC__out UINT *pIndex);


void __RPC_STUB ITypeInfo_RemoteGetContainingTypeLib_Stub(
    IRpcStubBuffer *This,
    IRpcChannelBuffer *_pRpcChannelBuffer,
    PRPC_MESSAGE _pRpcMessage,
    DWORD *_pdwStubPhase);


/* [nocode][call_as] */ HRESULT STDMETHODCALLTYPE ITypeInfo_LocalReleaseTypeAttr_Proxy( 
    __RPC__in ITypeInfo * This);


void __RPC_STUB ITypeInfo_LocalReleaseTypeAttr_Stub(
    IRpcStubBuffer *This,
    IRpcChannelBuffer *_pRpcChannelBuffer,
    PRPC_MESSAGE _pRpcMessage,
    DWORD *_pdwStubPhase);


/* [nocode][call_as] */ HRESULT STDMETHODCALLTYPE ITypeInfo_LocalReleaseFuncDesc_Proxy( 
    __RPC__in ITypeInfo * This);


void __RPC_STUB ITypeInfo_LocalReleaseFuncDesc_Stub(
    IRpcStubBuffer *This,
    IRpcChannelBuffer *_pRpcChannelBuffer,
    PRPC_MESSAGE _pRpcMessage,
    DWORD *_pdwStubPhase);


/* [nocode][call_as] */ HRESULT STDMETHODCALLTYPE ITypeInfo_LocalReleaseVarDesc_Proxy( 
    __RPC__in ITypeInfo * This);


void __RPC_STUB ITypeInfo_LocalReleaseVarDesc_Stub(
    IRpcStubBuffer *This,
    IRpcChannelBuffer *_pRpcChannelBuffer,
    PRPC_MESSAGE _pRpcMessage,
    DWORD *_pdwStubPhase);



#endif 	/* __ITypeInfo_INTERFACE_DEFINED__ */


#ifndef __ITypeInfo2_INTERFACE_DEFINED__
#define __ITypeInfo2_INTERFACE_DEFINED__

/* interface ITypeInfo2 */
/* [unique][uuid][object] */ 

typedef /* [unique] */  __RPC_unique_pointer ITypeInfo2 *LPTYPEINFO2;


EXTERN_C const IID IID_ITypeInfo2;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("00020412-0000-0000-C000-000000000046")
    ITypeInfo2 : public ITypeInfo
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetTypeKind( 
            /* [out] */ __RPC__out TYPEKIND *pTypeKind) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetTypeFlags( 
            /* [out] */ __RPC__out ULONG *pTypeFlags) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetFuncIndexOfMemId( 
            /* [in] */ MEMBERID memid,
            /* [in] */ INVOKEKIND invKind,
            /* [out] */ __RPC__out UINT *pFuncIndex) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetVarIndexOfMemId( 
            /* [in] */ MEMBERID memid,
            /* [out] */ __RPC__out UINT *pVarIndex) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetCustData( 
            /* [in] */ __RPC__in REFGUID guid,
            /* [out] */ __RPC__out VARIANT *pVarVal) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetFuncCustData( 
            /* [in] */ UINT index,
            /* [in] */ __RPC__in REFGUID guid,
            /* [out] */ __RPC__out VARIANT *pVarVal) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetParamCustData( 
            /* [in] */ UINT indexFunc,
            /* [in] */ UINT indexParam,
            /* [in] */ __RPC__in REFGUID guid,
            /* [out] */ __RPC__out VARIANT *pVarVal) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetVarCustData( 
            /* [in] */ UINT index,
            /* [in] */ __RPC__in REFGUID guid,
            /* [out] */ __RPC__out VARIANT *pVarVal) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetImplTypeCustData( 
            /* [in] */ UINT index,
            /* [in] */ __RPC__in REFGUID guid,
            /* [out] */ __RPC__out VARIANT *pVarVal) = 0;
        
        virtual /* [local] */ HRESULT STDMETHODCALLTYPE GetDocumentation2( 
            /* [in] */ MEMBERID memid,
            /* [in] */ LCID lcid,
            /* [annotation][out] */ 
            _Outptr_opt_  BSTR *pbstrHelpString,
            /* [out] */ DWORD *pdwHelpStringContext,
            /* [annotation][out] */ 
            _Outptr_opt_  BSTR *pbstrHelpStringDll) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetAllCustData( 
            /* [out] */ __RPC__out CUSTDATA *pCustData) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetAllFuncCustData( 
            /* [in] */ UINT index,
            /* [out] */ __RPC__out CUSTDATA *pCustData) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetAllParamCustData( 
            /* [in] */ UINT indexFunc,
            /* [in] */ UINT indexParam,
            /* [out] */ __RPC__out CUSTDATA *pCustData) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetAllVarCustData( 
            /* [in] */ UINT index,
            /* [out] */ __RPC__out CUSTDATA *pCustData) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetAllImplTypeCustData( 
            /* [in] */ UINT index,
            /* [out] */ __RPC__out CUSTDATA *pCustData) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ITypeInfo2Vtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in ITypeInfo2 * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in ITypeInfo2 * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in ITypeInfo2 * This);
        
        DECLSPEC_XFGVIRT(ITypeInfo, GetTypeAttr)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *GetTypeAttr )( 
            ITypeInfo2 * This,
            /* [out] */ TYPEATTR **ppTypeAttr);
        
        DECLSPEC_XFGVIRT(ITypeInfo, GetTypeComp)
        HRESULT ( STDMETHODCALLTYPE *GetTypeComp )( 
            __RPC__in ITypeInfo2 * This,
            /* [out] */ __RPC__deref_out_opt ITypeComp **ppTComp);
        
        DECLSPEC_XFGVIRT(ITypeInfo, GetFuncDesc)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *GetFuncDesc )( 
            ITypeInfo2 * This,
            /* [in] */ UINT index,
            /* [out] */ FUNCDESC **ppFuncDesc);
        
        DECLSPEC_XFGVIRT(ITypeInfo, GetVarDesc)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *GetVarDesc )( 
            ITypeInfo2 * This,
            /* [in] */ UINT index,
            /* [out] */ VARDESC **ppVarDesc);
        
        DECLSPEC_XFGVIRT(ITypeInfo, GetNames)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *GetNames )( 
            ITypeInfo2 * This,
            /* [in] */ MEMBERID memid,
            /* [annotation][length_is][size_is][out] */ 
            __RPC__out_ecount_part(cMaxNames, *pcNames)  BSTR *rgBstrNames,
            /* [in] */ UINT cMaxNames,
            /* [annotation][out] */ 
            __RPC__out  UINT *pcNames);
        
        DECLSPEC_XFGVIRT(ITypeInfo, GetRefTypeOfImplType)
        HRESULT ( STDMETHODCALLTYPE *GetRefTypeOfImplType )( 
            __RPC__in ITypeInfo2 * This,
            /* [in] */ UINT index,
            /* [out] */ __RPC__out HREFTYPE *pRefType);
        
        DECLSPEC_XFGVIRT(ITypeInfo, GetImplTypeFlags)
        HRESULT ( STDMETHODCALLTYPE *GetImplTypeFlags )( 
            __RPC__in ITypeInfo2 * This,
            /* [in] */ UINT index,
            /* [out] */ __RPC__out INT *pImplTypeFlags);
        
        DECLSPEC_XFGVIRT(ITypeInfo, GetIDsOfNames)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            ITypeInfo2 * This,
            /* [annotation][size_is][in] */ 
            __RPC__in_ecount(cNames)  LPOLESTR *rgszNames,
            /* [in] */ UINT cNames,
            /* [size_is][out] */ MEMBERID *pMemId);
        
        DECLSPEC_XFGVIRT(ITypeInfo, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            ITypeInfo2 * This,
            /* [in] */ PVOID pvInstance,
            /* [in] */ MEMBERID memid,
            /* [in] */ WORD wFlags,
            /* [out][in] */ DISPPARAMS *pDispParams,
            /* [out] */ VARIANT *pVarResult,
            /* [out] */ EXCEPINFO *pExcepInfo,
            /* [out] */ UINT *puArgErr);
        
        DECLSPEC_XFGVIRT(ITypeInfo, GetDocumentation)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *GetDocumentation )( 
            ITypeInfo2 * This,
            /* [in] */ MEMBERID memid,
            /* [annotation][out] */ 
            _Outptr_opt_  BSTR *pBstrName,
            /* [annotation][out] */ 
            _Outptr_opt_  BSTR *pBstrDocString,
            /* [out] */ DWORD *pdwHelpContext,
            /* [annotation][out] */ 
            _Outptr_opt_  BSTR *pBstrHelpFile);
        
        DECLSPEC_XFGVIRT(ITypeInfo, GetDllEntry)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *GetDllEntry )( 
            ITypeInfo2 * This,
            /* [in] */ MEMBERID memid,
            /* [in] */ INVOKEKIND invKind,
            /* [annotation][out] */ 
            _Outptr_opt_  BSTR *pBstrDllName,
            /* [annotation][out] */ 
            _Outptr_opt_  BSTR *pBstrName,
            /* [out] */ WORD *pwOrdinal);
        
        DECLSPEC_XFGVIRT(ITypeInfo, GetRefTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetRefTypeInfo )( 
            __RPC__in ITypeInfo2 * This,
            /* [in] */ HREFTYPE hRefType,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(ITypeInfo, AddressOfMember)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *AddressOfMember )( 
            ITypeInfo2 * This,
            /* [in] */ MEMBERID memid,
            /* [in] */ INVOKEKIND invKind,
            /* [out] */ PVOID *ppv);
        
        DECLSPEC_XFGVIRT(ITypeInfo, CreateInstance)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *CreateInstance )( 
            ITypeInfo2 * This,
            /* [in] */ IUnknown *pUnkOuter,
            /* [in] */ REFIID riid,
            /* [iid_is][out] */ PVOID *ppvObj);
        
        DECLSPEC_XFGVIRT(ITypeInfo, GetMops)
        HRESULT ( STDMETHODCALLTYPE *GetMops )( 
            __RPC__in ITypeInfo2 * This,
            /* [in] */ MEMBERID memid,
            /* [out] */ __RPC__deref_out_opt BSTR *pBstrMops);
        
        DECLSPEC_XFGVIRT(ITypeInfo, GetContainingTypeLib)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *GetContainingTypeLib )( 
            ITypeInfo2 * This,
            /* [out] */ ITypeLib **ppTLib,
            /* [out] */ UINT *pIndex);
        
        DECLSPEC_XFGVIRT(ITypeInfo, ReleaseTypeAttr)
        /* [local] */ void ( STDMETHODCALLTYPE *ReleaseTypeAttr )( 
            ITypeInfo2 * This,
            /* [in] */ TYPEATTR *pTypeAttr);
        
        DECLSPEC_XFGVIRT(ITypeInfo, ReleaseFuncDesc)
        /* [local] */ void ( STDMETHODCALLTYPE *ReleaseFuncDesc )( 
            ITypeInfo2 * This,
            /* [in] */ FUNCDESC *pFuncDesc);
        
        DECLSPEC_XFGVIRT(ITypeInfo, ReleaseVarDesc)
        /* [local] */ void ( STDMETHODCALLTYPE *ReleaseVarDesc )( 
            ITypeInfo2 * This,
            /* [in] */ VARDESC *pVarDesc);
        
        DECLSPEC_XFGVIRT(ITypeInfo2, GetTypeKind)
        HRESULT ( STDMETHODCALLTYPE *GetTypeKind )( 
            __RPC__in ITypeInfo2 * This,
            /* [out] */ __RPC__out TYPEKIND *pTypeKind);
        
        DECLSPEC_XFGVIRT(ITypeInfo2, GetTypeFlags)
        HRESULT ( STDMETHODCALLTYPE *GetTypeFlags )( 
            __RPC__in ITypeInfo2 * This,
            /* [out] */ __RPC__out ULONG *pTypeFlags);
        
        DECLSPEC_XFGVIRT(ITypeInfo2, GetFuncIndexOfMemId)
        HRESULT ( STDMETHODCALLTYPE *GetFuncIndexOfMemId )( 
            __RPC__in ITypeInfo2 * This,
            /* [in] */ MEMBERID memid,
            /* [in] */ INVOKEKIND invKind,
            /* [out] */ __RPC__out UINT *pFuncIndex);
        
        DECLSPEC_XFGVIRT(ITypeInfo2, GetVarIndexOfMemId)
        HRESULT ( STDMETHODCALLTYPE *GetVarIndexOfMemId )( 
            __RPC__in ITypeInfo2 * This,
            /* [in] */ MEMBERID memid,
            /* [out] */ __RPC__out UINT *pVarIndex);
        
        DECLSPEC_XFGVIRT(ITypeInfo2, GetCustData)
        HRESULT ( STDMETHODCALLTYPE *GetCustData )( 
            __RPC__in ITypeInfo2 * This,
            /* [in] */ __RPC__in REFGUID guid,
            /* [out] */ __RPC__out VARIANT *pVarVal);
        
        DECLSPEC_XFGVIRT(ITypeInfo2, GetFuncCustData)
        HRESULT ( STDMETHODCALLTYPE *GetFuncCustData )( 
            __RPC__in ITypeInfo2 * This,
            /* [in] */ UINT index,
            /* [in] */ __RPC__in REFGUID guid,
            /* [out] */ __RPC__out VARIANT *pVarVal);
        
        DECLSPEC_XFGVIRT(ITypeInfo2, GetParamCustData)
        HRESULT ( STDMETHODCALLTYPE *GetParamCustData )( 
            __RPC__in ITypeInfo2 * This,
            /* [in] */ UINT indexFunc,
            /* [in] */ UINT indexParam,
            /* [in] */ __RPC__in REFGUID guid,
            /* [out] */ __RPC__out VARIANT *pVarVal);
        
        DECLSPEC_XFGVIRT(ITypeInfo2, GetVarCustData)
        HRESULT ( STDMETHODCALLTYPE *GetVarCustData )( 
            __RPC__in ITypeInfo2 * This,
            /* [in] */ UINT index,
            /* [in] */ __RPC__in REFGUID guid,
            /* [out] */ __RPC__out VARIANT *pVarVal);
        
        DECLSPEC_XFGVIRT(ITypeInfo2, GetImplTypeCustData)
        HRESULT ( STDMETHODCALLTYPE *GetImplTypeCustData )( 
            __RPC__in ITypeInfo2 * This,
            /* [in] */ UINT index,
            /* [in] */ __RPC__in REFGUID guid,
            /* [out] */ __RPC__out VARIANT *pVarVal);
        
        DECLSPEC_XFGVIRT(ITypeInfo2, GetDocumentation2)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *GetDocumentation2 )( 
            ITypeInfo2 * This,
            /* [in] */ MEMBERID memid,
            /* [in] */ LCID lcid,
            /* [annotation][out] */ 
            _Outptr_opt_  BSTR *pbstrHelpString,
            /* [out] */ DWORD *pdwHelpStringContext,
            /* [annotation][out] */ 
            _Outptr_opt_  BSTR *pbstrHelpStringDll);
        
        DECLSPEC_XFGVIRT(ITypeInfo2, GetAllCustData)
        HRESULT ( STDMETHODCALLTYPE *GetAllCustData )( 
            __RPC__in ITypeInfo2 * This,
            /* [out] */ __RPC__out CUSTDATA *pCustData);
        
        DECLSPEC_XFGVIRT(ITypeInfo2, GetAllFuncCustData)
        HRESULT ( STDMETHODCALLTYPE *GetAllFuncCustData )( 
            __RPC__in ITypeInfo2 * This,
            /* [in] */ UINT index,
            /* [out] */ __RPC__out CUSTDATA *pCustData);
        
        DECLSPEC_XFGVIRT(ITypeInfo2, GetAllParamCustData)
        HRESULT ( STDMETHODCALLTYPE *GetAllParamCustData )( 
            __RPC__in ITypeInfo2 * This,
            /* [in] */ UINT indexFunc,
            /* [in] */ UINT indexParam,
            /* [out] */ __RPC__out CUSTDATA *pCustData);
        
        DECLSPEC_XFGVIRT(ITypeInfo2, GetAllVarCustData)
        HRESULT ( STDMETHODCALLTYPE *GetAllVarCustData )( 
            __RPC__in ITypeInfo2 * This,
            /* [in] */ UINT index,
            /* [out] */ __RPC__out CUSTDATA *pCustData);
        
        DECLSPEC_XFGVIRT(ITypeInfo2, GetAllImplTypeCustData)
        HRESULT ( STDMETHODCALLTYPE *GetAllImplTypeCustData )( 
            __RPC__in ITypeInfo2 * This,
            /* [in] */ UINT index,
            /* [out] */ __RPC__out CUSTDATA *pCustData);
        
        END_INTERFACE
    } ITypeInfo2Vtbl;

    interface ITypeInfo2
    {
        CONST_VTBL struct ITypeInfo2Vtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ITypeInfo2_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ITypeInfo2_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ITypeInfo2_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ITypeInfo2_GetTypeAttr(This,ppTypeAttr)	\
    ( (This)->lpVtbl -> GetTypeAttr(This,ppTypeAttr) ) 

#define ITypeInfo2_GetTypeComp(This,ppTComp)	\
    ( (This)->lpVtbl -> GetTypeComp(This,ppTComp) ) 

#define ITypeInfo2_GetFuncDesc(This,index,ppFuncDesc)	\
    ( (This)->lpVtbl -> GetFuncDesc(This,index,ppFuncDesc) ) 

#define ITypeInfo2_GetVarDesc(This,index,ppVarDesc)	\
    ( (This)->lpVtbl -> GetVarDesc(This,index,ppVarDesc) ) 

#define ITypeInfo2_GetNames(This,memid,rgBstrNames,cMaxNames,pcNames)	\
    ( (This)->lpVtbl -> GetNames(This,memid,rgBstrNames,cMaxNames,pcNames) ) 

#define ITypeInfo2_GetRefTypeOfImplType(This,index,pRefType)	\
    ( (This)->lpVtbl -> GetRefTypeOfImplType(This,index,pRefType) ) 

#define ITypeInfo2_GetImplTypeFlags(This,index,pImplTypeFlags)	\
    ( (This)->lpVtbl -> GetImplTypeFlags(This,index,pImplTypeFlags) ) 

#define ITypeInfo2_GetIDsOfNames(This,rgszNames,cNames,pMemId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,rgszNames,cNames,pMemId) ) 

#define ITypeInfo2_Invoke(This,pvInstance,memid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,pvInstance,memid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 

#define ITypeInfo2_GetDocumentation(This,memid,pBstrName,pBstrDocString,pdwHelpContext,pBstrHelpFile)	\
    ( (This)->lpVtbl -> GetDocumentation(This,memid,pBstrName,pBstrDocString,pdwHelpContext,pBstrHelpFile) ) 

#define ITypeInfo2_GetDllEntry(This,memid,invKind,pBstrDllName,pBstrName,pwOrdinal)	\
    ( (This)->lpVtbl -> GetDllEntry(This,memid,invKind,pBstrDllName,pBstrName,pwOrdinal) ) 

#define ITypeInfo2_GetRefTypeInfo(This,hRefType,ppTInfo)	\
    ( (This)->lpVtbl -> GetRefTypeInfo(This,hRefType,ppTInfo) ) 

#define ITypeInfo2_AddressOfMember(This,memid,invKind,ppv)	\
    ( (This)->lpVtbl -> AddressOfMember(This,memid,invKind,ppv) ) 

#define ITypeInfo2_CreateInstance(This,pUnkOuter,riid,ppvObj)	\
    ( (This)->lpVtbl -> CreateInstance(This,pUnkOuter,riid,ppvObj) ) 

#define ITypeInfo2_GetMops(This,memid,pBstrMops)	\
    ( (This)->lpVtbl -> GetMops(This,memid,pBstrMops) ) 

#define ITypeInfo2_GetContainingTypeLib(This,ppTLib,pIndex)	\
    ( (This)->lpVtbl -> GetContainingTypeLib(This,ppTLib,pIndex) ) 

#define ITypeInfo2_ReleaseTypeAttr(This,pTypeAttr)	\
    ( (This)->lpVtbl -> ReleaseTypeAttr(This,pTypeAttr) ) 

#define ITypeInfo2_ReleaseFuncDesc(This,pFuncDesc)	\
    ( (This)->lpVtbl -> ReleaseFuncDesc(This,pFuncDesc) ) 

#define ITypeInfo2_ReleaseVarDesc(This,pVarDesc)	\
    ( (This)->lpVtbl -> ReleaseVarDesc(This,pVarDesc) ) 


#define ITypeInfo2_GetTypeKind(This,pTypeKind)	\
    ( (This)->lpVtbl -> GetTypeKind(This,pTypeKind) ) 

#define ITypeInfo2_GetTypeFlags(This,pTypeFlags)	\
    ( (This)->lpVtbl -> GetTypeFlags(This,pTypeFlags) ) 

#define ITypeInfo2_GetFuncIndexOfMemId(This,memid,invKind,pFuncIndex)	\
    ( (This)->lpVtbl -> GetFuncIndexOfMemId(This,memid,invKind,pFuncIndex) ) 

#define ITypeInfo2_GetVarIndexOfMemId(This,memid,pVarIndex)	\
    ( (This)->lpVtbl -> GetVarIndexOfMemId(This,memid,pVarIndex) ) 

#define ITypeInfo2_GetCustData(This,guid,pVarVal)	\
    ( (This)->lpVtbl -> GetCustData(This,guid,pVarVal) ) 

#define ITypeInfo2_GetFuncCustData(This,index,guid,pVarVal)	\
    ( (This)->lpVtbl -> GetFuncCustData(This,index,guid,pVarVal) ) 

#define ITypeInfo2_GetParamCustData(This,indexFunc,indexParam,guid,pVarVal)	\
    ( (This)->lpVtbl -> GetParamCustData(This,indexFunc,indexParam,guid,pVarVal) ) 

#define ITypeInfo2_GetVarCustData(This,index,guid,pVarVal)	\
    ( (This)->lpVtbl -> GetVarCustData(This,index,guid,pVarVal) ) 

#define ITypeInfo2_GetImplTypeCustData(This,index,guid,pVarVal)	\
    ( (This)->lpVtbl -> GetImplTypeCustData(This,index,guid,pVarVal) ) 

#define ITypeInfo2_GetDocumentation2(This,memid,lcid,pbstrHelpString,pdwHelpStringContext,pbstrHelpStringDll)	\
    ( (This)->lpVtbl -> GetDocumentation2(This,memid,lcid,pbstrHelpString,pdwHelpStringContext,pbstrHelpStringDll) ) 

#define ITypeInfo2_GetAllCustData(This,pCustData)	\
    ( (This)->lpVtbl -> GetAllCustData(This,pCustData) ) 

#define ITypeInfo2_GetAllFuncCustData(This,index,pCustData)	\
    ( (This)->lpVtbl -> GetAllFuncCustData(This,index,pCustData) ) 

#define ITypeInfo2_GetAllParamCustData(This,indexFunc,indexParam,pCustData)	\
    ( (This)->lpVtbl -> GetAllParamCustData(This,indexFunc,indexParam,pCustData) ) 

#define ITypeInfo2_GetAllVarCustData(This,index,pCustData)	\
    ( (This)->lpVtbl -> GetAllVarCustData(This,index,pCustData) ) 

#define ITypeInfo2_GetAllImplTypeCustData(This,index,pCustData)	\
    ( (This)->lpVtbl -> GetAllImplTypeCustData(This,index,pCustData) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */



/* [call_as] */ HRESULT STDMETHODCALLTYPE ITypeInfo2_RemoteGetDocumentation2_Proxy( 
    __RPC__in ITypeInfo2 * This,
    /* [in] */ MEMBERID memid,
    /* [in] */ LCID lcid,
    /* [in] */ DWORD refPtrFlags,
    /* [out] */ __RPC__deref_out_opt BSTR *pbstrHelpString,
    /* [out] */ __RPC__out DWORD *pdwHelpStringContext,
    /* [out] */ __RPC__deref_out_opt BSTR *pbstrHelpStringDll);


void __RPC_STUB ITypeInfo2_RemoteGetDocumentation2_Stub(
    IRpcStubBuffer *This,
    IRpcChannelBuffer *_pRpcChannelBuffer,
    PRPC_MESSAGE _pRpcMessage,
    DWORD *_pdwStubPhase);



#endif 	/* __ITypeInfo2_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_oaidl_0000_0010 */
/* [local] */ 

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM) */
#pragma endregion
#pragma region Application Family or OneCore Family or Games Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES)


extern RPC_IF_HANDLE __MIDL_itf_oaidl_0000_0010_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_oaidl_0000_0010_v0_0_s_ifspec;

#ifndef __ITypeLib_INTERFACE_DEFINED__
#define __ITypeLib_INTERFACE_DEFINED__

/* interface ITypeLib */
/* [unique][uuid][object] */ 

typedef /* [v1_enum] */ 
enum tagSYSKIND
    {
        SYS_WIN16	= 0,
        SYS_WIN32	= ( SYS_WIN16 + 1 ) ,
        SYS_MAC	= ( SYS_WIN32 + 1 ) ,
        SYS_WIN64	= ( SYS_MAC + 1 ) 
    } 	SYSKIND;

typedef /* [v1_enum] */ 
enum tagLIBFLAGS
    {
        LIBFLAG_FRESTRICTED	= 0x1,
        LIBFLAG_FCONTROL	= 0x2,
        LIBFLAG_FHIDDEN	= 0x4,
        LIBFLAG_FHASDISKIMAGE	= 0x8
    } 	LIBFLAGS;

typedef /* [unique] */  __RPC_unique_pointer ITypeLib *LPTYPELIB;

typedef struct tagTLIBATTR
    {
    GUID guid;
    LCID lcid;
    SYSKIND syskind;
    WORD wMajorVerNum;
    WORD wMinorVerNum;
    WORD wLibFlags;
    } 	TLIBATTR;

typedef struct tagTLIBATTR *LPTLIBATTR;


EXTERN_C const IID IID_ITypeLib;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("00020402-0000-0000-C000-000000000046")
    ITypeLib : public IUnknown
    {
    public:
        virtual /* [local] */ UINT STDMETHODCALLTYPE GetTypeInfoCount( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetTypeInfo( 
            /* [in] */ UINT index,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetTypeInfoType( 
            /* [in] */ UINT index,
            /* [out] */ __RPC__out TYPEKIND *pTKind) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetTypeInfoOfGuid( 
            /* [in] */ __RPC__in REFGUID guid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTinfo) = 0;
        
        virtual /* [local] */ HRESULT STDMETHODCALLTYPE GetLibAttr( 
            /* [out] */ TLIBATTR **ppTLibAttr) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetTypeComp( 
            /* [out] */ __RPC__deref_out_opt ITypeComp **ppTComp) = 0;
        
        virtual /* [local] */ HRESULT STDMETHODCALLTYPE GetDocumentation( 
            /* [in] */ INT index,
            /* [annotation][out] */ 
            _Outptr_opt_  BSTR *pBstrName,
            /* [annotation][out] */ 
            _Outptr_opt_  BSTR *pBstrDocString,
            /* [out] */ DWORD *pdwHelpContext,
            /* [annotation][out] */ 
            _Outptr_opt_  BSTR *pBstrHelpFile) = 0;
        
        virtual /* [local] */ HRESULT STDMETHODCALLTYPE IsName( 
            /* [annotation][out][in] */ 
            __RPC__inout  LPOLESTR szNameBuf,
            /* [in] */ ULONG lHashVal,
            /* [out] */ BOOL *pfName) = 0;
        
        virtual /* [local] */ HRESULT STDMETHODCALLTYPE FindName( 
            /* [annotation][out][in] */ 
            __RPC__inout  LPOLESTR szNameBuf,
            /* [in] */ ULONG lHashVal,
            /* [length_is][size_is][out] */ ITypeInfo **ppTInfo,
            /* [length_is][size_is][out] */ MEMBERID *rgMemId,
            /* [out][in] */ USHORT *pcFound) = 0;
        
        virtual /* [local] */ void STDMETHODCALLTYPE ReleaseTLibAttr( 
            /* [in] */ TLIBATTR *pTLibAttr) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ITypeLibVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in ITypeLib * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in ITypeLib * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in ITypeLib * This);
        
        DECLSPEC_XFGVIRT(ITypeLib, GetTypeInfoCount)
        /* [local] */ UINT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            ITypeLib * This);
        
        DECLSPEC_XFGVIRT(ITypeLib, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in ITypeLib * This,
            /* [in] */ UINT index,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(ITypeLib, GetTypeInfoType)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoType )( 
            __RPC__in ITypeLib * This,
            /* [in] */ UINT index,
            /* [out] */ __RPC__out TYPEKIND *pTKind);
        
        DECLSPEC_XFGVIRT(ITypeLib, GetTypeInfoOfGuid)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoOfGuid )( 
            __RPC__in ITypeLib * This,
            /* [in] */ __RPC__in REFGUID guid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTinfo);
        
        DECLSPEC_XFGVIRT(ITypeLib, GetLibAttr)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *GetLibAttr )( 
            ITypeLib * This,
            /* [out] */ TLIBATTR **ppTLibAttr);
        
        DECLSPEC_XFGVIRT(ITypeLib, GetTypeComp)
        HRESULT ( STDMETHODCALLTYPE *GetTypeComp )( 
            __RPC__in ITypeLib * This,
            /* [out] */ __RPC__deref_out_opt ITypeComp **ppTComp);
        
        DECLSPEC_XFGVIRT(ITypeLib, GetDocumentation)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *GetDocumentation )( 
            ITypeLib * This,
            /* [in] */ INT index,
            /* [annotation][out] */ 
            _Outptr_opt_  BSTR *pBstrName,
            /* [annotation][out] */ 
            _Outptr_opt_  BSTR *pBstrDocString,
            /* [out] */ DWORD *pdwHelpContext,
            /* [annotation][out] */ 
            _Outptr_opt_  BSTR *pBstrHelpFile);
        
        DECLSPEC_XFGVIRT(ITypeLib, IsName)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *IsName )( 
            ITypeLib * This,
            /* [annotation][out][in] */ 
            __RPC__inout  LPOLESTR szNameBuf,
            /* [in] */ ULONG lHashVal,
            /* [out] */ BOOL *pfName);
        
        DECLSPEC_XFGVIRT(ITypeLib, FindName)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *FindName )( 
            ITypeLib * This,
            /* [annotation][out][in] */ 
            __RPC__inout  LPOLESTR szNameBuf,
            /* [in] */ ULONG lHashVal,
            /* [length_is][size_is][out] */ ITypeInfo **ppTInfo,
            /* [length_is][size_is][out] */ MEMBERID *rgMemId,
            /* [out][in] */ USHORT *pcFound);
        
        DECLSPEC_XFGVIRT(ITypeLib, ReleaseTLibAttr)
        /* [local] */ void ( STDMETHODCALLTYPE *ReleaseTLibAttr )( 
            ITypeLib * This,
            /* [in] */ TLIBATTR *pTLibAttr);
        
        END_INTERFACE
    } ITypeLibVtbl;

    interface ITypeLib
    {
        CONST_VTBL struct ITypeLibVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ITypeLib_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ITypeLib_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ITypeLib_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ITypeLib_GetTypeInfoCount(This)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This) ) 

#define ITypeLib_GetTypeInfo(This,index,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,index,ppTInfo) ) 

#define ITypeLib_GetTypeInfoType(This,index,pTKind)	\
    ( (This)->lpVtbl -> GetTypeInfoType(This,index,pTKind) ) 

#define ITypeLib_GetTypeInfoOfGuid(This,guid,ppTinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoOfGuid(This,guid,ppTinfo) ) 

#define ITypeLib_GetLibAttr(This,ppTLibAttr)	\
    ( (This)->lpVtbl -> GetLibAttr(This,ppTLibAttr) ) 

#define ITypeLib_GetTypeComp(This,ppTComp)	\
    ( (This)->lpVtbl -> GetTypeComp(This,ppTComp) ) 

#define ITypeLib_GetDocumentation(This,index,pBstrName,pBstrDocString,pdwHelpContext,pBstrHelpFile)	\
    ( (This)->lpVtbl -> GetDocumentation(This,index,pBstrName,pBstrDocString,pdwHelpContext,pBstrHelpFile) ) 

#define ITypeLib_IsName(This,szNameBuf,lHashVal,pfName)	\
    ( (This)->lpVtbl -> IsName(This,szNameBuf,lHashVal,pfName) ) 

#define ITypeLib_FindName(This,szNameBuf,lHashVal,ppTInfo,rgMemId,pcFound)	\
    ( (This)->lpVtbl -> FindName(This,szNameBuf,lHashVal,ppTInfo,rgMemId,pcFound) ) 

#define ITypeLib_ReleaseTLibAttr(This,pTLibAttr)	\
    ( (This)->lpVtbl -> ReleaseTLibAttr(This,pTLibAttr) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */



/* [call_as] */ HRESULT STDMETHODCALLTYPE ITypeLib_RemoteGetTypeInfoCount_Proxy( 
    __RPC__in ITypeLib * This,
    /* [out] */ __RPC__out UINT *pcTInfo);


void __RPC_STUB ITypeLib_RemoteGetTypeInfoCount_Stub(
    IRpcStubBuffer *This,
    IRpcChannelBuffer *_pRpcChannelBuffer,
    PRPC_MESSAGE _pRpcMessage,
    DWORD *_pdwStubPhase);


/* [call_as] */ HRESULT STDMETHODCALLTYPE ITypeLib_RemoteGetLibAttr_Proxy( 
    __RPC__in ITypeLib * This,
    /* [out] */ __RPC__deref_out_opt LPTLIBATTR *ppTLibAttr,
    /* [out] */ __RPC__out CLEANLOCALSTORAGE *pDummy);


void __RPC_STUB ITypeLib_RemoteGetLibAttr_Stub(
    IRpcStubBuffer *This,
    IRpcChannelBuffer *_pRpcChannelBuffer,
    PRPC_MESSAGE _pRpcMessage,
    DWORD *_pdwStubPhase);


/* [call_as] */ HRESULT STDMETHODCALLTYPE ITypeLib_RemoteGetDocumentation_Proxy( 
    __RPC__in ITypeLib * This,
    /* [in] */ INT index,
    /* [in] */ DWORD refPtrFlags,
    /* [out] */ __RPC__deref_out_opt BSTR *pBstrName,
    /* [out] */ __RPC__deref_out_opt BSTR *pBstrDocString,
    /* [out] */ __RPC__out DWORD *pdwHelpContext,
    /* [out] */ __RPC__deref_out_opt BSTR *pBstrHelpFile);


void __RPC_STUB ITypeLib_RemoteGetDocumentation_Stub(
    IRpcStubBuffer *This,
    IRpcChannelBuffer *_pRpcChannelBuffer,
    PRPC_MESSAGE _pRpcMessage,
    DWORD *_pdwStubPhase);


/* [call_as] */ HRESULT STDMETHODCALLTYPE ITypeLib_RemoteIsName_Proxy( 
    __RPC__in ITypeLib * This,
    /* [in] */ __RPC__in LPOLESTR szNameBuf,
    /* [in] */ ULONG lHashVal,
    /* [out] */ __RPC__out BOOL *pfName,
    /* [out] */ __RPC__deref_out_opt BSTR *pBstrLibName);


void __RPC_STUB ITypeLib_RemoteIsName_Stub(
    IRpcStubBuffer *This,
    IRpcChannelBuffer *_pRpcChannelBuffer,
    PRPC_MESSAGE _pRpcMessage,
    DWORD *_pdwStubPhase);


/* [call_as] */ HRESULT STDMETHODCALLTYPE ITypeLib_RemoteFindName_Proxy( 
    __RPC__in ITypeLib * This,
    /* [in] */ __RPC__in LPOLESTR szNameBuf,
    /* [in] */ ULONG lHashVal,
    /* [length_is][size_is][out] */ __RPC__out_ecount_part(*pcFound, *pcFound) ITypeInfo **ppTInfo,
    /* [length_is][size_is][out] */ __RPC__out_ecount_part(*pcFound, *pcFound) MEMBERID *rgMemId,
    /* [out][in] */ __RPC__inout USHORT *pcFound,
    /* [out] */ __RPC__deref_out_opt BSTR *pBstrLibName);


void __RPC_STUB ITypeLib_RemoteFindName_Stub(
    IRpcStubBuffer *This,
    IRpcChannelBuffer *_pRpcChannelBuffer,
    PRPC_MESSAGE _pRpcMessage,
    DWORD *_pdwStubPhase);


/* [nocode][call_as] */ HRESULT STDMETHODCALLTYPE ITypeLib_LocalReleaseTLibAttr_Proxy( 
    __RPC__in ITypeLib * This);


void __RPC_STUB ITypeLib_LocalReleaseTLibAttr_Stub(
    IRpcStubBuffer *This,
    IRpcChannelBuffer *_pRpcChannelBuffer,
    PRPC_MESSAGE _pRpcMessage,
    DWORD *_pdwStubPhase);



#endif 	/* __ITypeLib_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_oaidl_0000_0011 */
/* [local] */ 

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES) */
#pragma endregion
#pragma region Desktop Family or OneCore Family or OneCore Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM)


extern RPC_IF_HANDLE __MIDL_itf_oaidl_0000_0011_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_oaidl_0000_0011_v0_0_s_ifspec;

#ifndef __ITypeLib2_INTERFACE_DEFINED__
#define __ITypeLib2_INTERFACE_DEFINED__

/* interface ITypeLib2 */
/* [unique][uuid][object] */ 

typedef /* [unique] */  __RPC_unique_pointer ITypeLib2 *LPTYPELIB2;


EXTERN_C const IID IID_ITypeLib2;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("00020411-0000-0000-C000-000000000046")
    ITypeLib2 : public ITypeLib
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetCustData( 
            /* [in] */ __RPC__in REFGUID guid,
            /* [out] */ __RPC__out VARIANT *pVarVal) = 0;
        
        virtual /* [local] */ HRESULT STDMETHODCALLTYPE GetLibStatistics( 
            /* [out] */ ULONG *pcUniqueNames,
            /* [out] */ ULONG *pcchUniqueNames) = 0;
        
        virtual /* [local] */ HRESULT STDMETHODCALLTYPE GetDocumentation2( 
            /* [in] */ INT index,
            /* [in] */ LCID lcid,
            /* [annotation][out] */ 
            _Outptr_opt_  BSTR *pbstrHelpString,
            /* [out] */ DWORD *pdwHelpStringContext,
            /* [annotation][out] */ 
            _Outptr_opt_  BSTR *pbstrHelpStringDll) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetAllCustData( 
            /* [out] */ __RPC__out CUSTDATA *pCustData) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ITypeLib2Vtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in ITypeLib2 * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in ITypeLib2 * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in ITypeLib2 * This);
        
        DECLSPEC_XFGVIRT(ITypeLib, GetTypeInfoCount)
        /* [local] */ UINT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            ITypeLib2 * This);
        
        DECLSPEC_XFGVIRT(ITypeLib, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in ITypeLib2 * This,
            /* [in] */ UINT index,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(ITypeLib, GetTypeInfoType)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoType )( 
            __RPC__in ITypeLib2 * This,
            /* [in] */ UINT index,
            /* [out] */ __RPC__out TYPEKIND *pTKind);
        
        DECLSPEC_XFGVIRT(ITypeLib, GetTypeInfoOfGuid)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoOfGuid )( 
            __RPC__in ITypeLib2 * This,
            /* [in] */ __RPC__in REFGUID guid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTinfo);
        
        DECLSPEC_XFGVIRT(ITypeLib, GetLibAttr)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *GetLibAttr )( 
            ITypeLib2 * This,
            /* [out] */ TLIBATTR **ppTLibAttr);
        
        DECLSPEC_XFGVIRT(ITypeLib, GetTypeComp)
        HRESULT ( STDMETHODCALLTYPE *GetTypeComp )( 
            __RPC__in ITypeLib2 * This,
            /* [out] */ __RPC__deref_out_opt ITypeComp **ppTComp);
        
        DECLSPEC_XFGVIRT(ITypeLib, GetDocumentation)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *GetDocumentation )( 
            ITypeLib2 * This,
            /* [in] */ INT index,
            /* [annotation][out] */ 
            _Outptr_opt_  BSTR *pBstrName,
            /* [annotation][out] */ 
            _Outptr_opt_  BSTR *pBstrDocString,
            /* [out] */ DWORD *pdwHelpContext,
            /* [annotation][out] */ 
            _Outptr_opt_  BSTR *pBstrHelpFile);
        
        DECLSPEC_XFGVIRT(ITypeLib, IsName)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *IsName )( 
            ITypeLib2 * This,
            /* [annotation][out][in] */ 
            __RPC__inout  LPOLESTR szNameBuf,
            /* [in] */ ULONG lHashVal,
            /* [out] */ BOOL *pfName);
        
        DECLSPEC_XFGVIRT(ITypeLib, FindName)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *FindName )( 
            ITypeLib2 * This,
            /* [annotation][out][in] */ 
            __RPC__inout  LPOLESTR szNameBuf,
            /* [in] */ ULONG lHashVal,
            /* [length_is][size_is][out] */ ITypeInfo **ppTInfo,
            /* [length_is][size_is][out] */ MEMBERID *rgMemId,
            /* [out][in] */ USHORT *pcFound);
        
        DECLSPEC_XFGVIRT(ITypeLib, ReleaseTLibAttr)
        /* [local] */ void ( STDMETHODCALLTYPE *ReleaseTLibAttr )( 
            ITypeLib2 * This,
            /* [in] */ TLIBATTR *pTLibAttr);
        
        DECLSPEC_XFGVIRT(ITypeLib2, GetCustData)
        HRESULT ( STDMETHODCALLTYPE *GetCustData )( 
            __RPC__in ITypeLib2 * This,
            /* [in] */ __RPC__in REFGUID guid,
            /* [out] */ __RPC__out VARIANT *pVarVal);
        
        DECLSPEC_XFGVIRT(ITypeLib2, GetLibStatistics)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *GetLibStatistics )( 
            ITypeLib2 * This,
            /* [out] */ ULONG *pcUniqueNames,
            /* [out] */ ULONG *pcchUniqueNames);
        
        DECLSPEC_XFGVIRT(ITypeLib2, GetDocumentation2)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *GetDocumentation2 )( 
            ITypeLib2 * This,
            /* [in] */ INT index,
            /* [in] */ LCID lcid,
            /* [annotation][out] */ 
            _Outptr_opt_  BSTR *pbstrHelpString,
            /* [out] */ DWORD *pdwHelpStringContext,
            /* [annotation][out] */ 
            _Outptr_opt_  BSTR *pbstrHelpStringDll);
        
        DECLSPEC_XFGVIRT(ITypeLib2, GetAllCustData)
        HRESULT ( STDMETHODCALLTYPE *GetAllCustData )( 
            __RPC__in ITypeLib2 * This,
            /* [out] */ __RPC__out CUSTDATA *pCustData);
        
        END_INTERFACE
    } ITypeLib2Vtbl;

    interface ITypeLib2
    {
        CONST_VTBL struct ITypeLib2Vtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ITypeLib2_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ITypeLib2_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ITypeLib2_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ITypeLib2_GetTypeInfoCount(This)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This) ) 

#define ITypeLib2_GetTypeInfo(This,index,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,index,ppTInfo) ) 

#define ITypeLib2_GetTypeInfoType(This,index,pTKind)	\
    ( (This)->lpVtbl -> GetTypeInfoType(This,index,pTKind) ) 

#define ITypeLib2_GetTypeInfoOfGuid(This,guid,ppTinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoOfGuid(This,guid,ppTinfo) ) 

#define ITypeLib2_GetLibAttr(This,ppTLibAttr)	\
    ( (This)->lpVtbl -> GetLibAttr(This,ppTLibAttr) ) 

#define ITypeLib2_GetTypeComp(This,ppTComp)	\
    ( (This)->lpVtbl -> GetTypeComp(This,ppTComp) ) 

#define ITypeLib2_GetDocumentation(This,index,pBstrName,pBstrDocString,pdwHelpContext,pBstrHelpFile)	\
    ( (This)->lpVtbl -> GetDocumentation(This,index,pBstrName,pBstrDocString,pdwHelpContext,pBstrHelpFile) ) 

#define ITypeLib2_IsName(This,szNameBuf,lHashVal,pfName)	\
    ( (This)->lpVtbl -> IsName(This,szNameBuf,lHashVal,pfName) ) 

#define ITypeLib2_FindName(This,szNameBuf,lHashVal,ppTInfo,rgMemId,pcFound)	\
    ( (This)->lpVtbl -> FindName(This,szNameBuf,lHashVal,ppTInfo,rgMemId,pcFound) ) 

#define ITypeLib2_ReleaseTLibAttr(This,pTLibAttr)	\
    ( (This)->lpVtbl -> ReleaseTLibAttr(This,pTLibAttr) ) 


#define ITypeLib2_GetCustData(This,guid,pVarVal)	\
    ( (This)->lpVtbl -> GetCustData(This,guid,pVarVal) ) 

#define ITypeLib2_GetLibStatistics(This,pcUniqueNames,pcchUniqueNames)	\
    ( (This)->lpVtbl -> GetLibStatistics(This,pcUniqueNames,pcchUniqueNames) ) 

#define ITypeLib2_GetDocumentation2(This,index,lcid,pbstrHelpString,pdwHelpStringContext,pbstrHelpStringDll)	\
    ( (This)->lpVtbl -> GetDocumentation2(This,index,lcid,pbstrHelpString,pdwHelpStringContext,pbstrHelpStringDll) ) 

#define ITypeLib2_GetAllCustData(This,pCustData)	\
    ( (This)->lpVtbl -> GetAllCustData(This,pCustData) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */



/* [call_as] */ HRESULT STDMETHODCALLTYPE ITypeLib2_RemoteGetLibStatistics_Proxy( 
    __RPC__in ITypeLib2 * This,
    /* [out] */ __RPC__out ULONG *pcUniqueNames,
    /* [out] */ __RPC__out ULONG *pcchUniqueNames);


void __RPC_STUB ITypeLib2_RemoteGetLibStatistics_Stub(
    IRpcStubBuffer *This,
    IRpcChannelBuffer *_pRpcChannelBuffer,
    PRPC_MESSAGE _pRpcMessage,
    DWORD *_pdwStubPhase);


/* [call_as] */ HRESULT STDMETHODCALLTYPE ITypeLib2_RemoteGetDocumentation2_Proxy( 
    __RPC__in ITypeLib2 * This,
    /* [in] */ INT index,
    /* [in] */ LCID lcid,
    /* [in] */ DWORD refPtrFlags,
    /* [out] */ __RPC__deref_out_opt BSTR *pbstrHelpString,
    /* [out] */ __RPC__out DWORD *pdwHelpStringContext,
    /* [out] */ __RPC__deref_out_opt BSTR *pbstrHelpStringDll);


void __RPC_STUB ITypeLib2_RemoteGetDocumentation2_Stub(
    IRpcStubBuffer *This,
    IRpcChannelBuffer *_pRpcChannelBuffer,
    PRPC_MESSAGE _pRpcMessage,
    DWORD *_pdwStubPhase);



#endif 	/* __ITypeLib2_INTERFACE_DEFINED__ */


#ifndef __ITypeChangeEvents_INTERFACE_DEFINED__
#define __ITypeChangeEvents_INTERFACE_DEFINED__

/* interface ITypeChangeEvents */
/* [local][unique][uuid][object] */ 

typedef /* [unique] */ ITypeChangeEvents *LPTYPECHANGEEVENTS;

typedef 
enum tagCHANGEKIND
    {
        CHANGEKIND_ADDMEMBER	= 0,
        CHANGEKIND_DELETEMEMBER	= ( CHANGEKIND_ADDMEMBER + 1 ) ,
        CHANGEKIND_SETNAMES	= ( CHANGEKIND_DELETEMEMBER + 1 ) ,
        CHANGEKIND_SETDOCUMENTATION	= ( CHANGEKIND_SETNAMES + 1 ) ,
        CHANGEKIND_GENERAL	= ( CHANGEKIND_SETDOCUMENTATION + 1 ) ,
        CHANGEKIND_INVALIDATE	= ( CHANGEKIND_GENERAL + 1 ) ,
        CHANGEKIND_CHANGEFAILED	= ( CHANGEKIND_INVALIDATE + 1 ) ,
        CHANGEKIND_MAX	= ( CHANGEKIND_CHANGEFAILED + 1 ) 
    } 	CHANGEKIND;


EXTERN_C const IID IID_ITypeChangeEvents;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("00020410-0000-0000-C000-000000000046")
    ITypeChangeEvents : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE RequestTypeChange( 
            /* [in] */ CHANGEKIND changeKind,
            /* [in] */ ITypeInfo *pTInfoBefore,
            /* [annotation][in] */ 
            __RPC__in  LPOLESTR pStrName,
            /* [out] */ INT *pfCancel) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE AfterTypeChange( 
            /* [in] */ CHANGEKIND changeKind,
            /* [in] */ ITypeInfo *pTInfoAfter,
            /* [annotation][in] */ 
            __RPC__in  LPOLESTR pStrName) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ITypeChangeEventsVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            ITypeChangeEvents * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            ITypeChangeEvents * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            ITypeChangeEvents * This);
        
        DECLSPEC_XFGVIRT(ITypeChangeEvents, RequestTypeChange)
        HRESULT ( STDMETHODCALLTYPE *RequestTypeChange )( 
            ITypeChangeEvents * This,
            /* [in] */ CHANGEKIND changeKind,
            /* [in] */ ITypeInfo *pTInfoBefore,
            /* [annotation][in] */ 
            __RPC__in  LPOLESTR pStrName,
            /* [out] */ INT *pfCancel);
        
        DECLSPEC_XFGVIRT(ITypeChangeEvents, AfterTypeChange)
        HRESULT ( STDMETHODCALLTYPE *AfterTypeChange )( 
            ITypeChangeEvents * This,
            /* [in] */ CHANGEKIND changeKind,
            /* [in] */ ITypeInfo *pTInfoAfter,
            /* [annotation][in] */ 
            __RPC__in  LPOLESTR pStrName);
        
        END_INTERFACE
    } ITypeChangeEventsVtbl;

    interface ITypeChangeEvents
    {
        CONST_VTBL struct ITypeChangeEventsVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ITypeChangeEvents_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ITypeChangeEvents_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ITypeChangeEvents_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ITypeChangeEvents_RequestTypeChange(This,changeKind,pTInfoBefore,pStrName,pfCancel)	\
    ( (This)->lpVtbl -> RequestTypeChange(This,changeKind,pTInfoBefore,pStrName,pfCancel) ) 

#define ITypeChangeEvents_AfterTypeChange(This,changeKind,pTInfoAfter,pStrName)	\
    ( (This)->lpVtbl -> AfterTypeChange(This,changeKind,pTInfoAfter,pStrName) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ITypeChangeEvents_INTERFACE_DEFINED__ */


#ifndef __IErrorInfo_INTERFACE_DEFINED__
#define __IErrorInfo_INTERFACE_DEFINED__

/* interface IErrorInfo */
/* [unique][uuid][object] */ 

typedef /* [unique] */  __RPC_unique_pointer IErrorInfo *LPERRORINFO;


EXTERN_C const IID IID_IErrorInfo;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("1CF2B120-547D-101B-8E65-08002B2BD119")
    IErrorInfo : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetGUID( 
            /* [out] */ __RPC__out GUID *pGUID) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetSource( 
            /* [out] */ __RPC__deref_out_opt BSTR *pBstrSource) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetDescription( 
            /* [out] */ __RPC__deref_out_opt BSTR *pBstrDescription) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetHelpFile( 
            /* [out] */ __RPC__deref_out_opt BSTR *pBstrHelpFile) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetHelpContext( 
            /* [out] */ __RPC__out DWORD *pdwHelpContext) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IErrorInfoVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IErrorInfo * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IErrorInfo * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IErrorInfo * This);
        
        DECLSPEC_XFGVIRT(IErrorInfo, GetGUID)
        HRESULT ( STDMETHODCALLTYPE *GetGUID )( 
            __RPC__in IErrorInfo * This,
            /* [out] */ __RPC__out GUID *pGUID);
        
        DECLSPEC_XFGVIRT(IErrorInfo, GetSource)
        HRESULT ( STDMETHODCALLTYPE *GetSource )( 
            __RPC__in IErrorInfo * This,
            /* [out] */ __RPC__deref_out_opt BSTR *pBstrSource);
        
        DECLSPEC_XFGVIRT(IErrorInfo, GetDescription)
        HRESULT ( STDMETHODCALLTYPE *GetDescription )( 
            __RPC__in IErrorInfo * This,
            /* [out] */ __RPC__deref_out_opt BSTR *pBstrDescription);
        
        DECLSPEC_XFGVIRT(IErrorInfo, GetHelpFile)
        HRESULT ( STDMETHODCALLTYPE *GetHelpFile )( 
            __RPC__in IErrorInfo * This,
            /* [out] */ __RPC__deref_out_opt BSTR *pBstrHelpFile);
        
        DECLSPEC_XFGVIRT(IErrorInfo, GetHelpContext)
        HRESULT ( STDMETHODCALLTYPE *GetHelpContext )( 
            __RPC__in IErrorInfo * This,
            /* [out] */ __RPC__out DWORD *pdwHelpContext);
        
        END_INTERFACE
    } IErrorInfoVtbl;

    interface IErrorInfo
    {
        CONST_VTBL struct IErrorInfoVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IErrorInfo_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IErrorInfo_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IErrorInfo_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IErrorInfo_GetGUID(This,pGUID)	\
    ( (This)->lpVtbl -> GetGUID(This,pGUID) ) 

#define IErrorInfo_GetSource(This,pBstrSource)	\
    ( (This)->lpVtbl -> GetSource(This,pBstrSource) ) 

#define IErrorInfo_GetDescription(This,pBstrDescription)	\
    ( (This)->lpVtbl -> GetDescription(This,pBstrDescription) ) 

#define IErrorInfo_GetHelpFile(This,pBstrHelpFile)	\
    ( (This)->lpVtbl -> GetHelpFile(This,pBstrHelpFile) ) 

#define IErrorInfo_GetHelpContext(This,pdwHelpContext)	\
    ( (This)->lpVtbl -> GetHelpContext(This,pdwHelpContext) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IErrorInfo_INTERFACE_DEFINED__ */


#ifndef __ICreateErrorInfo_INTERFACE_DEFINED__
#define __ICreateErrorInfo_INTERFACE_DEFINED__

/* interface ICreateErrorInfo */
/* [unique][uuid][object] */ 

typedef /* [unique] */  __RPC_unique_pointer ICreateErrorInfo *LPCREATEERRORINFO;


EXTERN_C const IID IID_ICreateErrorInfo;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("22F03340-547D-101B-8E65-08002B2BD119")
    ICreateErrorInfo : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE SetGUID( 
            /* [in] */ __RPC__in REFGUID rguid) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetSource( 
            /* [in] */ __RPC__in LPOLESTR szSource) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetDescription( 
            /* [in] */ __RPC__in LPOLESTR szDescription) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetHelpFile( 
            /* [in] */ __RPC__in LPOLESTR szHelpFile) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetHelpContext( 
            /* [in] */ DWORD dwHelpContext) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ICreateErrorInfoVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in ICreateErrorInfo * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in ICreateErrorInfo * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in ICreateErrorInfo * This);
        
        DECLSPEC_XFGVIRT(ICreateErrorInfo, SetGUID)
        HRESULT ( STDMETHODCALLTYPE *SetGUID )( 
            __RPC__in ICreateErrorInfo * This,
            /* [in] */ __RPC__in REFGUID rguid);
        
        DECLSPEC_XFGVIRT(ICreateErrorInfo, SetSource)
        HRESULT ( STDMETHODCALLTYPE *SetSource )( 
            __RPC__in ICreateErrorInfo * This,
            /* [in] */ __RPC__in LPOLESTR szSource);
        
        DECLSPEC_XFGVIRT(ICreateErrorInfo, SetDescription)
        HRESULT ( STDMETHODCALLTYPE *SetDescription )( 
            __RPC__in ICreateErrorInfo * This,
            /* [in] */ __RPC__in LPOLESTR szDescription);
        
        DECLSPEC_XFGVIRT(ICreateErrorInfo, SetHelpFile)
        HRESULT ( STDMETHODCALLTYPE *SetHelpFile )( 
            __RPC__in ICreateErrorInfo * This,
            /* [in] */ __RPC__in LPOLESTR szHelpFile);
        
        DECLSPEC_XFGVIRT(ICreateErrorInfo, SetHelpContext)
        HRESULT ( STDMETHODCALLTYPE *SetHelpContext )( 
            __RPC__in ICreateErrorInfo * This,
            /* [in] */ DWORD dwHelpContext);
        
        END_INTERFACE
    } ICreateErrorInfoVtbl;

    interface ICreateErrorInfo
    {
        CONST_VTBL struct ICreateErrorInfoVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ICreateErrorInfo_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ICreateErrorInfo_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ICreateErrorInfo_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ICreateErrorInfo_SetGUID(This,rguid)	\
    ( (This)->lpVtbl -> SetGUID(This,rguid) ) 

#define ICreateErrorInfo_SetSource(This,szSource)	\
    ( (This)->lpVtbl -> SetSource(This,szSource) ) 

#define ICreateErrorInfo_SetDescription(This,szDescription)	\
    ( (This)->lpVtbl -> SetDescription(This,szDescription) ) 

#define ICreateErrorInfo_SetHelpFile(This,szHelpFile)	\
    ( (This)->lpVtbl -> SetHelpFile(This,szHelpFile) ) 

#define ICreateErrorInfo_SetHelpContext(This,dwHelpContext)	\
    ( (This)->lpVtbl -> SetHelpContext(This,dwHelpContext) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ICreateErrorInfo_INTERFACE_DEFINED__ */


#ifndef __ISupportErrorInfo_INTERFACE_DEFINED__
#define __ISupportErrorInfo_INTERFACE_DEFINED__

/* interface ISupportErrorInfo */
/* [unique][uuid][object] */ 

typedef /* [unique] */  __RPC_unique_pointer ISupportErrorInfo *LPSUPPORTERRORINFO;


EXTERN_C const IID IID_ISupportErrorInfo;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("DF0B3D60-548F-101B-8E65-08002B2BD119")
    ISupportErrorInfo : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE InterfaceSupportsErrorInfo( 
            /* [in] */ __RPC__in REFIID riid) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ISupportErrorInfoVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in ISupportErrorInfo * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in ISupportErrorInfo * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in ISupportErrorInfo * This);
        
        DECLSPEC_XFGVIRT(ISupportErrorInfo, InterfaceSupportsErrorInfo)
        HRESULT ( STDMETHODCALLTYPE *InterfaceSupportsErrorInfo )( 
            __RPC__in ISupportErrorInfo * This,
            /* [in] */ __RPC__in REFIID riid);
        
        END_INTERFACE
    } ISupportErrorInfoVtbl;

    interface ISupportErrorInfo
    {
        CONST_VTBL struct ISupportErrorInfoVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ISupportErrorInfo_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ISupportErrorInfo_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ISupportErrorInfo_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ISupportErrorInfo_InterfaceSupportsErrorInfo(This,riid)	\
    ( (This)->lpVtbl -> InterfaceSupportsErrorInfo(This,riid) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ISupportErrorInfo_INTERFACE_DEFINED__ */


#ifndef __ITypeFactory_INTERFACE_DEFINED__
#define __ITypeFactory_INTERFACE_DEFINED__

/* interface ITypeFactory */
/* [uuid][object] */ 


EXTERN_C const IID IID_ITypeFactory;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("0000002E-0000-0000-C000-000000000046")
    ITypeFactory : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE CreateFromTypeInfo( 
            /* [in] */ __RPC__in_opt ITypeInfo *pTypeInfo,
            /* [in] */ __RPC__in REFIID riid,
            /* [iid_is][out] */ __RPC__deref_out_opt IUnknown **ppv) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ITypeFactoryVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in ITypeFactory * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in ITypeFactory * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in ITypeFactory * This);
        
        DECLSPEC_XFGVIRT(ITypeFactory, CreateFromTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *CreateFromTypeInfo )( 
            __RPC__in ITypeFactory * This,
            /* [in] */ __RPC__in_opt ITypeInfo *pTypeInfo,
            /* [in] */ __RPC__in REFIID riid,
            /* [iid_is][out] */ __RPC__deref_out_opt IUnknown **ppv);
        
        END_INTERFACE
    } ITypeFactoryVtbl;

    interface ITypeFactory
    {
        CONST_VTBL struct ITypeFactoryVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ITypeFactory_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ITypeFactory_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ITypeFactory_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ITypeFactory_CreateFromTypeInfo(This,pTypeInfo,riid,ppv)	\
    ( (This)->lpVtbl -> CreateFromTypeInfo(This,pTypeInfo,riid,ppv) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ITypeFactory_INTERFACE_DEFINED__ */


#ifndef __ITypeMarshal_INTERFACE_DEFINED__
#define __ITypeMarshal_INTERFACE_DEFINED__

/* interface ITypeMarshal */
/* [uuid][object][local] */ 


EXTERN_C const IID IID_ITypeMarshal;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("0000002D-0000-0000-C000-000000000046")
    ITypeMarshal : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE Size( 
            /* [in] */ PVOID pvType,
            /* [in] */ DWORD dwDestContext,
            /* [in] */ PVOID pvDestContext,
            /* [out] */ ULONG *pSize) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Marshal( 
            /* [in] */ PVOID pvType,
            /* [in] */ DWORD dwDestContext,
            /* [in] */ PVOID pvDestContext,
            /* [in] */ ULONG cbBufferLength,
            /* [annotation][out] */ 
            _Out_writes_bytes_to_(cbBufferLength, *pcbWritten)  BYTE *pBuffer,
            /* [annotation][out] */ 
            _Out_  ULONG *pcbWritten) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Unmarshal( 
            /* [out] */ PVOID pvType,
            /* [in] */ DWORD dwFlags,
            /* [in] */ ULONG cbBufferLength,
            /* [annotation][in] */ 
            _In_reads_(cbBufferLength)  BYTE *pBuffer,
            /* [annotation][out] */ 
            _Out_  ULONG *pcbRead) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Free( 
            /* [in] */ PVOID pvType) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ITypeMarshalVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            ITypeMarshal * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            ITypeMarshal * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            ITypeMarshal * This);
        
        DECLSPEC_XFGVIRT(ITypeMarshal, Size)
        HRESULT ( STDMETHODCALLTYPE *Size )( 
            ITypeMarshal * This,
            /* [in] */ PVOID pvType,
            /* [in] */ DWORD dwDestContext,
            /* [in] */ PVOID pvDestContext,
            /* [out] */ ULONG *pSize);
        
        DECLSPEC_XFGVIRT(ITypeMarshal, Marshal)
        HRESULT ( STDMETHODCALLTYPE *Marshal )( 
            ITypeMarshal * This,
            /* [in] */ PVOID pvType,
            /* [in] */ DWORD dwDestContext,
            /* [in] */ PVOID pvDestContext,
            /* [in] */ ULONG cbBufferLength,
            /* [annotation][out] */ 
            _Out_writes_bytes_to_(cbBufferLength, *pcbWritten)  BYTE *pBuffer,
            /* [annotation][out] */ 
            _Out_  ULONG *pcbWritten);
        
        DECLSPEC_XFGVIRT(ITypeMarshal, Unmarshal)
        HRESULT ( STDMETHODCALLTYPE *Unmarshal )( 
            ITypeMarshal * This,
            /* [out] */ PVOID pvType,
            /* [in] */ DWORD dwFlags,
            /* [in] */ ULONG cbBufferLength,
            /* [annotation][in] */ 
            _In_reads_(cbBufferLength)  BYTE *pBuffer,
            /* [annotation][out] */ 
            _Out_  ULONG *pcbRead);
        
        DECLSPEC_XFGVIRT(ITypeMarshal, Free)
        HRESULT ( STDMETHODCALLTYPE *Free )( 
            ITypeMarshal * This,
            /* [in] */ PVOID pvType);
        
        END_INTERFACE
    } ITypeMarshalVtbl;

    interface ITypeMarshal
    {
        CONST_VTBL struct ITypeMarshalVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ITypeMarshal_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ITypeMarshal_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ITypeMarshal_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ITypeMarshal_Size(This,pvType,dwDestContext,pvDestContext,pSize)	\
    ( (This)->lpVtbl -> Size(This,pvType,dwDestContext,pvDestContext,pSize) ) 

#define ITypeMarshal_Marshal(This,pvType,dwDestContext,pvDestContext,cbBufferLength,pBuffer,pcbWritten)	\
    ( (This)->lpVtbl -> Marshal(This,pvType,dwDestContext,pvDestContext,cbBufferLength,pBuffer,pcbWritten) ) 

#define ITypeMarshal_Unmarshal(This,pvType,dwFlags,cbBufferLength,pBuffer,pcbRead)	\
    ( (This)->lpVtbl -> Unmarshal(This,pvType,dwFlags,cbBufferLength,pBuffer,pcbRead) ) 

#define ITypeMarshal_Free(This,pvType)	\
    ( (This)->lpVtbl -> Free(This,pvType) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ITypeMarshal_INTERFACE_DEFINED__ */


#ifndef __IRecordInfo_INTERFACE_DEFINED__
#define __IRecordInfo_INTERFACE_DEFINED__

/* interface IRecordInfo */
/* [uuid][object][local] */ 

typedef /* [unique] */ IRecordInfo *LPRECORDINFO;


EXTERN_C const IID IID_IRecordInfo;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("0000002F-0000-0000-C000-000000000046")
    IRecordInfo : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE RecordInit( 
            /* [out] */ PVOID pvNew) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE RecordClear( 
            /* [in] */ PVOID pvExisting) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE RecordCopy( 
            /* [in] */ PVOID pvExisting,
            /* [out] */ PVOID pvNew) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetGuid( 
            /* [out] */ GUID *pguid) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetName( 
            /* [annotation][out] */ 
            __RPC__deref_out_opt  BSTR *pbstrName) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetSize( 
            /* [out] */ ULONG *pcbSize) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetTypeInfo( 
            /* [out] */ ITypeInfo **ppTypeInfo) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetField( 
            /* [in] */ PVOID pvData,
            /* [in] */ LPCOLESTR szFieldName,
            /* [out] */ VARIANT *pvarField) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetFieldNoCopy( 
            /* [in] */ PVOID pvData,
            /* [in] */ LPCOLESTR szFieldName,
            /* [out] */ VARIANT *pvarField,
            /* [out] */ PVOID *ppvDataCArray) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE PutField( 
            /* [in] */ ULONG wFlags,
            /* [out][in] */ PVOID pvData,
            /* [in] */ LPCOLESTR szFieldName,
            /* [in] */ VARIANT *pvarField) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE PutFieldNoCopy( 
            /* [in] */ ULONG wFlags,
            /* [out][in] */ PVOID pvData,
            /* [in] */ LPCOLESTR szFieldName,
            /* [in] */ VARIANT *pvarField) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetFieldNames( 
            /* [out][in] */ ULONG *pcNames,
            /* [annotation][length_is][size_is][out] */ 
            __RPC__out_ecount_part(*pcNames, *pcNames)  BSTR *rgBstrNames) = 0;
        
        virtual BOOL STDMETHODCALLTYPE IsMatchingType( 
            /* [in] */ IRecordInfo *pRecordInfo) = 0;
        
        virtual PVOID STDMETHODCALLTYPE RecordCreate( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE RecordCreateCopy( 
            /* [in] */ PVOID pvSource,
            /* [out] */ PVOID *ppvDest) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE RecordDestroy( 
            /* [in] */ PVOID pvRecord) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IRecordInfoVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IRecordInfo * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IRecordInfo * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IRecordInfo * This);
        
        DECLSPEC_XFGVIRT(IRecordInfo, RecordInit)
        HRESULT ( STDMETHODCALLTYPE *RecordInit )( 
            IRecordInfo * This,
            /* [out] */ PVOID pvNew);
        
        DECLSPEC_XFGVIRT(IRecordInfo, RecordClear)
        HRESULT ( STDMETHODCALLTYPE *RecordClear )( 
            IRecordInfo * This,
            /* [in] */ PVOID pvExisting);
        
        DECLSPEC_XFGVIRT(IRecordInfo, RecordCopy)
        HRESULT ( STDMETHODCALLTYPE *RecordCopy )( 
            IRecordInfo * This,
            /* [in] */ PVOID pvExisting,
            /* [out] */ PVOID pvNew);
        
        DECLSPEC_XFGVIRT(IRecordInfo, GetGuid)
        HRESULT ( STDMETHODCALLTYPE *GetGuid )( 
            IRecordInfo * This,
            /* [out] */ GUID *pguid);
        
        DECLSPEC_XFGVIRT(IRecordInfo, GetName)
        HRESULT ( STDMETHODCALLTYPE *GetName )( 
            IRecordInfo * This,
            /* [annotation][out] */ 
            __RPC__deref_out_opt  BSTR *pbstrName);
        
        DECLSPEC_XFGVIRT(IRecordInfo, GetSize)
        HRESULT ( STDMETHODCALLTYPE *GetSize )( 
            IRecordInfo * This,
            /* [out] */ ULONG *pcbSize);
        
        DECLSPEC_XFGVIRT(IRecordInfo, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            IRecordInfo * This,
            /* [out] */ ITypeInfo **ppTypeInfo);
        
        DECLSPEC_XFGVIRT(IRecordInfo, GetField)
        HRESULT ( STDMETHODCALLTYPE *GetField )( 
            IRecordInfo * This,
            /* [in] */ PVOID pvData,
            /* [in] */ LPCOLESTR szFieldName,
            /* [out] */ VARIANT *pvarField);
        
        DECLSPEC_XFGVIRT(IRecordInfo, GetFieldNoCopy)
        HRESULT ( STDMETHODCALLTYPE *GetFieldNoCopy )( 
            IRecordInfo * This,
            /* [in] */ PVOID pvData,
            /* [in] */ LPCOLESTR szFieldName,
            /* [out] */ VARIANT *pvarField,
            /* [out] */ PVOID *ppvDataCArray);
        
        DECLSPEC_XFGVIRT(IRecordInfo, PutField)
        HRESULT ( STDMETHODCALLTYPE *PutField )( 
            IRecordInfo * This,
            /* [in] */ ULONG wFlags,
            /* [out][in] */ PVOID pvData,
            /* [in] */ LPCOLESTR szFieldName,
            /* [in] */ VARIANT *pvarField);
        
        DECLSPEC_XFGVIRT(IRecordInfo, PutFieldNoCopy)
        HRESULT ( STDMETHODCALLTYPE *PutFieldNoCopy )( 
            IRecordInfo * This,
            /* [in] */ ULONG wFlags,
            /* [out][in] */ PVOID pvData,
            /* [in] */ LPCOLESTR szFieldName,
            /* [in] */ VARIANT *pvarField);
        
        DECLSPEC_XFGVIRT(IRecordInfo, GetFieldNames)
        HRESULT ( STDMETHODCALLTYPE *GetFieldNames )( 
            IRecordInfo * This,
            /* [out][in] */ ULONG *pcNames,
            /* [annotation][length_is][size_is][out] */ 
            __RPC__out_ecount_part(*pcNames, *pcNames)  BSTR *rgBstrNames);
        
        DECLSPEC_XFGVIRT(IRecordInfo, IsMatchingType)
        BOOL ( STDMETHODCALLTYPE *IsMatchingType )( 
            IRecordInfo * This,
            /* [in] */ IRecordInfo *pRecordInfo);
        
        DECLSPEC_XFGVIRT(IRecordInfo, RecordCreate)
        PVOID ( STDMETHODCALLTYPE *RecordCreate )( 
            IRecordInfo * This);
        
        DECLSPEC_XFGVIRT(IRecordInfo, RecordCreateCopy)
        HRESULT ( STDMETHODCALLTYPE *RecordCreateCopy )( 
            IRecordInfo * This,
            /* [in] */ PVOID pvSource,
            /* [out] */ PVOID *ppvDest);
        
        DECLSPEC_XFGVIRT(IRecordInfo, RecordDestroy)
        HRESULT ( STDMETHODCALLTYPE *RecordDestroy )( 
            IRecordInfo * This,
            /* [in] */ PVOID pvRecord);
        
        END_INTERFACE
    } IRecordInfoVtbl;

    interface IRecordInfo
    {
        CONST_VTBL struct IRecordInfoVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IRecordInfo_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IRecordInfo_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IRecordInfo_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IRecordInfo_RecordInit(This,pvNew)	\
    ( (This)->lpVtbl -> RecordInit(This,pvNew) ) 

#define IRecordInfo_RecordClear(This,pvExisting)	\
    ( (This)->lpVtbl -> RecordClear(This,pvExisting) ) 

#define IRecordInfo_RecordCopy(This,pvExisting,pvNew)	\
    ( (This)->lpVtbl -> RecordCopy(This,pvExisting,pvNew) ) 

#define IRecordInfo_GetGuid(This,pguid)	\
    ( (This)->lpVtbl -> GetGuid(This,pguid) ) 

#define IRecordInfo_GetName(This,pbstrName)	\
    ( (This)->lpVtbl -> GetName(This,pbstrName) ) 

#define IRecordInfo_GetSize(This,pcbSize)	\
    ( (This)->lpVtbl -> GetSize(This,pcbSize) ) 

#define IRecordInfo_GetTypeInfo(This,ppTypeInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,ppTypeInfo) ) 

#define IRecordInfo_GetField(This,pvData,szFieldName,pvarField)	\
    ( (This)->lpVtbl -> GetField(This,pvData,szFieldName,pvarField) ) 

#define IRecordInfo_GetFieldNoCopy(This,pvData,szFieldName,pvarField,ppvDataCArray)	\
    ( (This)->lpVtbl -> GetFieldNoCopy(This,pvData,szFieldName,pvarField,ppvDataCArray) ) 

#define IRecordInfo_PutField(This,wFlags,pvData,szFieldName,pvarField)	\
    ( (This)->lpVtbl -> PutField(This,wFlags,pvData,szFieldName,pvarField) ) 

#define IRecordInfo_PutFieldNoCopy(This,wFlags,pvData,szFieldName,pvarField)	\
    ( (This)->lpVtbl -> PutFieldNoCopy(This,wFlags,pvData,szFieldName,pvarField) ) 

#define IRecordInfo_GetFieldNames(This,pcNames,rgBstrNames)	\
    ( (This)->lpVtbl -> GetFieldNames(This,pcNames,rgBstrNames) ) 

#define IRecordInfo_IsMatchingType(This,pRecordInfo)	\
    ( (This)->lpVtbl -> IsMatchingType(This,pRecordInfo) ) 

#define IRecordInfo_RecordCreate(This)	\
    ( (This)->lpVtbl -> RecordCreate(This) ) 

#define IRecordInfo_RecordCreateCopy(This,pvSource,ppvDest)	\
    ( (This)->lpVtbl -> RecordCreateCopy(This,pvSource,ppvDest) ) 

#define IRecordInfo_RecordDestroy(This,pvRecord)	\
    ( (This)->lpVtbl -> RecordDestroy(This,pvRecord) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IRecordInfo_INTERFACE_DEFINED__ */


#ifndef __IErrorLog_INTERFACE_DEFINED__
#define __IErrorLog_INTERFACE_DEFINED__

/* interface IErrorLog */
/* [unique][uuid][object] */ 

typedef IErrorLog *LPERRORLOG;


EXTERN_C const IID IID_IErrorLog;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("3127CA40-446E-11CE-8135-00AA004BB851")
    IErrorLog : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE AddError( 
            /* [in] */ __RPC__in LPCOLESTR pszPropName,
            /* [in] */ __RPC__in EXCEPINFO *pExcepInfo) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IErrorLogVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IErrorLog * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IErrorLog * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IErrorLog * This);
        
        DECLSPEC_XFGVIRT(IErrorLog, AddError)
        HRESULT ( STDMETHODCALLTYPE *AddError )( 
            __RPC__in IErrorLog * This,
            /* [in] */ __RPC__in LPCOLESTR pszPropName,
            /* [in] */ __RPC__in EXCEPINFO *pExcepInfo);
        
        END_INTERFACE
    } IErrorLogVtbl;

    interface IErrorLog
    {
        CONST_VTBL struct IErrorLogVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IErrorLog_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IErrorLog_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IErrorLog_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IErrorLog_AddError(This,pszPropName,pExcepInfo)	\
    ( (This)->lpVtbl -> AddError(This,pszPropName,pExcepInfo) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IErrorLog_INTERFACE_DEFINED__ */


#ifndef __IPropertyBag_INTERFACE_DEFINED__
#define __IPropertyBag_INTERFACE_DEFINED__

/* interface IPropertyBag */
/* [unique][uuid][object] */ 

typedef IPropertyBag *LPPROPERTYBAG;


EXTERN_C const IID IID_IPropertyBag;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("55272A00-42CB-11CE-8135-00AA004BB851")
    IPropertyBag : public IUnknown
    {
    public:
        virtual /* [local] */ HRESULT STDMETHODCALLTYPE Read( 
            /* [in] */ LPCOLESTR pszPropName,
            /* [out][in] */ VARIANT *pVar,
            /* [unique][in] */ IErrorLog *pErrorLog) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Write( 
            /* [in] */ __RPC__in LPCOLESTR pszPropName,
            /* [in] */ __RPC__in VARIANT *pVar) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IPropertyBagVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IPropertyBag * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IPropertyBag * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IPropertyBag * This);
        
        DECLSPEC_XFGVIRT(IPropertyBag, Read)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Read )( 
            IPropertyBag * This,
            /* [in] */ LPCOLESTR pszPropName,
            /* [out][in] */ VARIANT *pVar,
            /* [unique][in] */ IErrorLog *pErrorLog);
        
        DECLSPEC_XFGVIRT(IPropertyBag, Write)
        HRESULT ( STDMETHODCALLTYPE *Write )( 
            __RPC__in IPropertyBag * This,
            /* [in] */ __RPC__in LPCOLESTR pszPropName,
            /* [in] */ __RPC__in VARIANT *pVar);
        
        END_INTERFACE
    } IPropertyBagVtbl;

    interface IPropertyBag
    {
        CONST_VTBL struct IPropertyBagVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IPropertyBag_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IPropertyBag_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IPropertyBag_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IPropertyBag_Read(This,pszPropName,pVar,pErrorLog)	\
    ( (This)->lpVtbl -> Read(This,pszPropName,pVar,pErrorLog) ) 

#define IPropertyBag_Write(This,pszPropName,pVar)	\
    ( (This)->lpVtbl -> Write(This,pszPropName,pVar) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */



/* [call_as] */ HRESULT STDMETHODCALLTYPE IPropertyBag_RemoteRead_Proxy( 
    __RPC__in IPropertyBag * This,
    /* [in] */ __RPC__in LPCOLESTR pszPropName,
    /* [out] */ __RPC__out VARIANT *pVar,
    /* [unique][in] */ __RPC__in_opt IErrorLog *pErrorLog,
    /* [in] */ DWORD varType,
    /* [in] */ __RPC__in_opt IUnknown *pUnkObj);


void __RPC_STUB IPropertyBag_RemoteRead_Stub(
    IRpcStubBuffer *This,
    IRpcChannelBuffer *_pRpcChannelBuffer,
    PRPC_MESSAGE _pRpcMessage,
    DWORD *_pdwStubPhase);



#endif 	/* __IPropertyBag_INTERFACE_DEFINED__ */


#ifndef __ITypeLibRegistrationReader_INTERFACE_DEFINED__
#define __ITypeLibRegistrationReader_INTERFACE_DEFINED__

/* interface ITypeLibRegistrationReader */
/* [uuid][object][local] */ 


EXTERN_C const IID IID_ITypeLibRegistrationReader;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("ED6A8A2A-B160-4E77-8F73-AA7435CD5C27")
    ITypeLibRegistrationReader : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE EnumTypeLibRegistrations( 
            /* [out] */ IEnumUnknown **ppEnumUnknown) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ITypeLibRegistrationReaderVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            ITypeLibRegistrationReader * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            ITypeLibRegistrationReader * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            ITypeLibRegistrationReader * This);
        
        DECLSPEC_XFGVIRT(ITypeLibRegistrationReader, EnumTypeLibRegistrations)
        HRESULT ( STDMETHODCALLTYPE *EnumTypeLibRegistrations )( 
            ITypeLibRegistrationReader * This,
            /* [out] */ IEnumUnknown **ppEnumUnknown);
        
        END_INTERFACE
    } ITypeLibRegistrationReaderVtbl;

    interface ITypeLibRegistrationReader
    {
        CONST_VTBL struct ITypeLibRegistrationReaderVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ITypeLibRegistrationReader_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ITypeLibRegistrationReader_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ITypeLibRegistrationReader_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ITypeLibRegistrationReader_EnumTypeLibRegistrations(This,ppEnumUnknown)	\
    ( (This)->lpVtbl -> EnumTypeLibRegistrations(This,ppEnumUnknown) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ITypeLibRegistrationReader_INTERFACE_DEFINED__ */


#ifndef __ITypeLibRegistration_INTERFACE_DEFINED__
#define __ITypeLibRegistration_INTERFACE_DEFINED__

/* interface ITypeLibRegistration */
/* [uuid][object][local] */ 


EXTERN_C const IID IID_ITypeLibRegistration;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("76A3E735-02DF-4A12-98EB-043AD3600AF3")
    ITypeLibRegistration : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetGuid( 
            /* [out] */ GUID *pGuid) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetVersion( 
            /* [out] */ BSTR *pVersion) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetLcid( 
            /* [out] */ LCID *pLcid) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetWin32Path( 
            /* [out] */ BSTR *pWin32Path) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetWin64Path( 
            /* [out] */ BSTR *pWin64Path) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetDisplayName( 
            /* [out] */ BSTR *pDisplayName) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetFlags( 
            /* [out] */ DWORD *pFlags) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetHelpDir( 
            /* [out] */ BSTR *pHelpDir) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ITypeLibRegistrationVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            ITypeLibRegistration * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            ITypeLibRegistration * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            ITypeLibRegistration * This);
        
        DECLSPEC_XFGVIRT(ITypeLibRegistration, GetGuid)
        HRESULT ( STDMETHODCALLTYPE *GetGuid )( 
            ITypeLibRegistration * This,
            /* [out] */ GUID *pGuid);
        
        DECLSPEC_XFGVIRT(ITypeLibRegistration, GetVersion)
        HRESULT ( STDMETHODCALLTYPE *GetVersion )( 
            ITypeLibRegistration * This,
            /* [out] */ BSTR *pVersion);
        
        DECLSPEC_XFGVIRT(ITypeLibRegistration, GetLcid)
        HRESULT ( STDMETHODCALLTYPE *GetLcid )( 
            ITypeLibRegistration * This,
            /* [out] */ LCID *pLcid);
        
        DECLSPEC_XFGVIRT(ITypeLibRegistration, GetWin32Path)
        HRESULT ( STDMETHODCALLTYPE *GetWin32Path )( 
            ITypeLibRegistration * This,
            /* [out] */ BSTR *pWin32Path);
        
        DECLSPEC_XFGVIRT(ITypeLibRegistration, GetWin64Path)
        HRESULT ( STDMETHODCALLTYPE *GetWin64Path )( 
            ITypeLibRegistration * This,
            /* [out] */ BSTR *pWin64Path);
        
        DECLSPEC_XFGVIRT(ITypeLibRegistration, GetDisplayName)
        HRESULT ( STDMETHODCALLTYPE *GetDisplayName )( 
            ITypeLibRegistration * This,
            /* [out] */ BSTR *pDisplayName);
        
        DECLSPEC_XFGVIRT(ITypeLibRegistration, GetFlags)
        HRESULT ( STDMETHODCALLTYPE *GetFlags )( 
            ITypeLibRegistration * This,
            /* [out] */ DWORD *pFlags);
        
        DECLSPEC_XFGVIRT(ITypeLibRegistration, GetHelpDir)
        HRESULT ( STDMETHODCALLTYPE *GetHelpDir )( 
            ITypeLibRegistration * This,
            /* [out] */ BSTR *pHelpDir);
        
        END_INTERFACE
    } ITypeLibRegistrationVtbl;

    interface ITypeLibRegistration
    {
        CONST_VTBL struct ITypeLibRegistrationVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ITypeLibRegistration_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ITypeLibRegistration_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ITypeLibRegistration_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ITypeLibRegistration_GetGuid(This,pGuid)	\
    ( (This)->lpVtbl -> GetGuid(This,pGuid) ) 

#define ITypeLibRegistration_GetVersion(This,pVersion)	\
    ( (This)->lpVtbl -> GetVersion(This,pVersion) ) 

#define ITypeLibRegistration_GetLcid(This,pLcid)	\
    ( (This)->lpVtbl -> GetLcid(This,pLcid) ) 

#define ITypeLibRegistration_GetWin32Path(This,pWin32Path)	\
    ( (This)->lpVtbl -> GetWin32Path(This,pWin32Path) ) 

#define ITypeLibRegistration_GetWin64Path(This,pWin64Path)	\
    ( (This)->lpVtbl -> GetWin64Path(This,pWin64Path) ) 

#define ITypeLibRegistration_GetDisplayName(This,pDisplayName)	\
    ( (This)->lpVtbl -> GetDisplayName(This,pDisplayName) ) 

#define ITypeLibRegistration_GetFlags(This,pFlags)	\
    ( (This)->lpVtbl -> GetFlags(This,pFlags) ) 

#define ITypeLibRegistration_GetHelpDir(This,pHelpDir)	\
    ( (This)->lpVtbl -> GetHelpDir(This,pHelpDir) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ITypeLibRegistration_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_oaidl_0000_0023 */
/* [local] */ 

EXTERN_C const CLSID CLSID_TypeLibRegistrationReader;
#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM) */
#pragma endregion
#if ( _MSC_VER >= 800 )
#if _MSC_VER >= 1200
#pragma warning(pop)
#else
#pragma warning(default:4201) /* Nameless struct/union */
#endif
#endif


extern RPC_IF_HANDLE __MIDL_itf_oaidl_0000_0023_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_oaidl_0000_0023_v0_0_s_ifspec;

/* Additional Prototypes for ALL interfaces */

unsigned long             __RPC_USER  BSTR_UserSize(     __RPC__in unsigned long *, unsigned long            , __RPC__in BSTR * ); 
unsigned char * __RPC_USER  BSTR_UserMarshal(  __RPC__in unsigned long *, __RPC__inout_xcount(0) unsigned char *, __RPC__in BSTR * ); 
unsigned char * __RPC_USER  BSTR_UserUnmarshal(__RPC__in unsigned long *, __RPC__in_xcount(0) unsigned char *, __RPC__out BSTR * ); 
void                      __RPC_USER  BSTR_UserFree(     __RPC__in unsigned long *, __RPC__in BSTR * ); 

unsigned long             __RPC_USER  CLEANLOCALSTORAGE_UserSize(     __RPC__in unsigned long *, unsigned long            , __RPC__in CLEANLOCALSTORAGE * ); 
unsigned char * __RPC_USER  CLEANLOCALSTORAGE_UserMarshal(  __RPC__in unsigned long *, __RPC__inout_xcount(0) unsigned char *, __RPC__in CLEANLOCALSTORAGE * ); 
unsigned char * __RPC_USER  CLEANLOCALSTORAGE_UserUnmarshal(__RPC__in unsigned long *, __RPC__in_xcount(0) unsigned char *, __RPC__out CLEANLOCALSTORAGE * ); 
void                      __RPC_USER  CLEANLOCALSTORAGE_UserFree(     __RPC__in unsigned long *, __RPC__in CLEANLOCALSTORAGE * ); 

unsigned long             __RPC_USER  VARIANT_UserSize(     __RPC__in unsigned long *, unsigned long            , __RPC__in VARIANT * ); 
unsigned char * __RPC_USER  VARIANT_UserMarshal(  __RPC__in unsigned long *, __RPC__inout_xcount(0) unsigned char *, __RPC__in VARIANT * ); 
unsigned char * __RPC_USER  VARIANT_UserUnmarshal(__RPC__in unsigned long *, __RPC__in_xcount(0) unsigned char *, __RPC__out VARIANT * ); 
void                      __RPC_USER  VARIANT_UserFree(     __RPC__in unsigned long *, __RPC__in VARIANT * ); 

unsigned long             __RPC_USER  BSTR_UserSize64(     __RPC__in unsigned long *, unsigned long            , __RPC__in BSTR * ); 
unsigned char * __RPC_USER  BSTR_UserMarshal64(  __RPC__in unsigned long *, __RPC__inout_xcount(0) unsigned char *, __RPC__in BSTR * ); 
unsigned char * __RPC_USER  BSTR_UserUnmarshal64(__RPC__in unsigned long *, __RPC__in_xcount(0) unsigned char *, __RPC__out BSTR * ); 
void                      __RPC_USER  BSTR_UserFree64(     __RPC__in unsigned long *, __RPC__in BSTR * ); 

unsigned long             __RPC_USER  CLEANLOCALSTORAGE_UserSize64(     __RPC__in unsigned long *, unsigned long            , __RPC__in CLEANLOCALSTORAGE * ); 
unsigned char * __RPC_USER  CLEANLOCALSTORAGE_UserMarshal64(  __RPC__in unsigned long *, __RPC__inout_xcount(0) unsigned char *, __RPC__in CLEANLOCALSTORAGE * ); 
unsigned char * __RPC_USER  CLEANLOCALSTORAGE_UserUnmarshal64(__RPC__in unsigned long *, __RPC__in_xcount(0) unsigned char *, __RPC__out CLEANLOCALSTORAGE * ); 
void                      __RPC_USER  CLEANLOCALSTORAGE_UserFree64(     __RPC__in unsigned long *, __RPC__in CLEANLOCALSTORAGE * ); 

unsigned long             __RPC_USER  VARIANT_UserSize64(     __RPC__in unsigned long *, unsigned long            , __RPC__in VARIANT * ); 
unsigned char * __RPC_USER  VARIANT_UserMarshal64(  __RPC__in unsigned long *, __RPC__inout_xcount(0) unsigned char *, __RPC__in VARIANT * ); 
unsigned char * __RPC_USER  VARIANT_UserUnmarshal64(__RPC__in unsigned long *, __RPC__in_xcount(0) unsigned char *, __RPC__out VARIANT * ); 
void                      __RPC_USER  VARIANT_UserFree64(     __RPC__in unsigned long *, __RPC__in VARIANT * ); 

/* [local] */ HRESULT STDMETHODCALLTYPE IDispatch_Invoke_Proxy( 
    IDispatch * This,
    /* [annotation][in] */ 
    _In_  DISPID dispIdMember,
    /* [annotation][in] */ 
    _In_  REFIID riid,
    /* [annotation][in] */ 
    _In_  LCID lcid,
    /* [annotation][in] */ 
    _In_  WORD wFlags,
    /* [annotation][out][in] */ 
    _In_  DISPPARAMS *pDispParams,
    /* [annotation][out] */ 
    _Out_opt_  VARIANT *pVarResult,
    /* [annotation][out] */ 
    _Out_opt_  EXCEPINFO *pExcepInfo,
    /* [annotation][out] */ 
    _Out_opt_  UINT *puArgErr);


/* [call_as] */ HRESULT STDMETHODCALLTYPE IDispatch_Invoke_Stub( 
    __RPC__in IDispatch * This,
    /* [in] */ DISPID dispIdMember,
    /* [in] */ __RPC__in REFIID riid,
    /* [in] */ LCID lcid,
    /* [in] */ DWORD dwFlags,
    /* [in] */ __RPC__in DISPPARAMS *pDispParams,
    /* [out] */ __RPC__out VARIANT *pVarResult,
    /* [out] */ __RPC__out EXCEPINFO *pExcepInfo,
    /* [out] */ __RPC__out UINT *pArgErr,
    /* [in] */ UINT cVarRef,
    /* [size_is][in] */ __RPC__in_ecount_full(cVarRef) UINT *rgVarRefIdx,
    /* [size_is][out][in] */ __RPC__inout_ecount_full(cVarRef) VARIANTARG *rgVarRef);

/* [local] */ HRESULT STDMETHODCALLTYPE IEnumVARIANT_Next_Proxy( 
    IEnumVARIANT * This,
    /* [in] */ ULONG celt,
    /* [length_is][size_is][out] */ VARIANT *rgVar,
    /* [out] */ ULONG *pCeltFetched);


/* [call_as] */ HRESULT STDMETHODCALLTYPE IEnumVARIANT_Next_Stub( 
    __RPC__in IEnumVARIANT * This,
    /* [in] */ ULONG celt,
    /* [length_is][size_is][out] */ __RPC__out_ecount_part(celt, *pCeltFetched) VARIANT *rgVar,
    /* [out] */ __RPC__out ULONG *pCeltFetched);

/* [local] */ HRESULT STDMETHODCALLTYPE ITypeComp_Bind_Proxy( 
    ITypeComp * This,
    /* [annotation][in] */ 
    __RPC__in  LPOLESTR szName,
    /* [in] */ ULONG lHashVal,
    /* [in] */ WORD wFlags,
    /* [out] */ ITypeInfo **ppTInfo,
    /* [out] */ DESCKIND *pDescKind,
    /* [out] */ BINDPTR *pBindPtr);


/* [call_as] */ HRESULT STDMETHODCALLTYPE ITypeComp_Bind_Stub( 
    __RPC__in ITypeComp * This,
    /* [in] */ __RPC__in LPOLESTR szName,
    /* [in] */ ULONG lHashVal,
    /* [in] */ WORD wFlags,
    /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo,
    /* [out] */ __RPC__out DESCKIND *pDescKind,
    /* [out] */ __RPC__deref_out_opt LPFUNCDESC *ppFuncDesc,
    /* [out] */ __RPC__deref_out_opt LPVARDESC *ppVarDesc,
    /* [out] */ __RPC__deref_out_opt ITypeComp **ppTypeComp,
    /* [out] */ __RPC__out CLEANLOCALSTORAGE *pDummy);

/* [local] */ HRESULT STDMETHODCALLTYPE ITypeComp_BindType_Proxy( 
    ITypeComp * This,
    /* [annotation][in] */ 
    __RPC__in  LPOLESTR szName,
    /* [in] */ ULONG lHashVal,
    /* [out] */ ITypeInfo **ppTInfo,
    /* [out] */ ITypeComp **ppTComp);


/* [call_as] */ HRESULT STDMETHODCALLTYPE ITypeComp_BindType_Stub( 
    __RPC__in ITypeComp * This,
    /* [in] */ __RPC__in LPOLESTR szName,
    /* [in] */ ULONG lHashVal,
    /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);

/* [local] */ HRESULT STDMETHODCALLTYPE ITypeInfo_GetTypeAttr_Proxy( 
    ITypeInfo * This,
    /* [out] */ TYPEATTR **ppTypeAttr);


/* [call_as] */ HRESULT STDMETHODCALLTYPE ITypeInfo_GetTypeAttr_Stub( 
    __RPC__in ITypeInfo * This,
    /* [out] */ __RPC__deref_out_opt LPTYPEATTR *ppTypeAttr,
    /* [out] */ __RPC__out CLEANLOCALSTORAGE *pDummy);

/* [local] */ HRESULT STDMETHODCALLTYPE ITypeInfo_GetFuncDesc_Proxy( 
    ITypeInfo * This,
    /* [in] */ UINT index,
    /* [out] */ FUNCDESC **ppFuncDesc);


/* [call_as] */ HRESULT STDMETHODCALLTYPE ITypeInfo_GetFuncDesc_Stub( 
    __RPC__in ITypeInfo * This,
    /* [in] */ UINT index,
    /* [out] */ __RPC__deref_out_opt LPFUNCDESC *ppFuncDesc,
    /* [out] */ __RPC__out CLEANLOCALSTORAGE *pDummy);

/* [local] */ HRESULT STDMETHODCALLTYPE ITypeInfo_GetVarDesc_Proxy( 
    ITypeInfo * This,
    /* [in] */ UINT index,
    /* [out] */ VARDESC **ppVarDesc);


/* [call_as] */ HRESULT STDMETHODCALLTYPE ITypeInfo_GetVarDesc_Stub( 
    __RPC__in ITypeInfo * This,
    /* [in] */ UINT index,
    /* [out] */ __RPC__deref_out_opt LPVARDESC *ppVarDesc,
    /* [out] */ __RPC__out CLEANLOCALSTORAGE *pDummy);

/* [local] */ HRESULT STDMETHODCALLTYPE ITypeInfo_GetNames_Proxy( 
    ITypeInfo * This,
    /* [in] */ MEMBERID memid,
    /* [annotation][length_is][size_is][out] */ 
    __RPC__out_ecount_part(cMaxNames, *pcNames)  BSTR *rgBstrNames,
    /* [in] */ UINT cMaxNames,
    /* [annotation][out] */ 
    __RPC__out  UINT *pcNames);


/* [call_as] */ HRESULT STDMETHODCALLTYPE ITypeInfo_GetNames_Stub( 
    __RPC__in ITypeInfo * This,
    /* [in] */ MEMBERID memid,
    /* [length_is][size_is][out] */ __RPC__out_ecount_part(cMaxNames, *pcNames) BSTR *rgBstrNames,
    /* [in] */ UINT cMaxNames,
    /* [out] */ __RPC__out UINT *pcNames);

/* [local] */ HRESULT STDMETHODCALLTYPE ITypeInfo_GetIDsOfNames_Proxy( 
    ITypeInfo * This,
    /* [annotation][size_is][in] */ 
    __RPC__in_ecount(cNames)  LPOLESTR *rgszNames,
    /* [in] */ UINT cNames,
    /* [size_is][out] */ MEMBERID *pMemId);


/* [nocode][call_as] */ HRESULT STDMETHODCALLTYPE ITypeInfo_GetIDsOfNames_Stub( 
    __RPC__in ITypeInfo * This);

/* [local] */ HRESULT STDMETHODCALLTYPE ITypeInfo_Invoke_Proxy( 
    ITypeInfo * This,
    /* [in] */ PVOID pvInstance,
    /* [in] */ MEMBERID memid,
    /* [in] */ WORD wFlags,
    /* [out][in] */ DISPPARAMS *pDispParams,
    /* [out] */ VARIANT *pVarResult,
    /* [out] */ EXCEPINFO *pExcepInfo,
    /* [out] */ UINT *puArgErr);


/* [nocode][call_as] */ HRESULT STDMETHODCALLTYPE ITypeInfo_Invoke_Stub( 
    __RPC__in ITypeInfo * This);

/* [local] */ HRESULT STDMETHODCALLTYPE ITypeInfo_GetDocumentation_Proxy( 
    ITypeInfo * This,
    /* [in] */ MEMBERID memid,
    /* [annotation][out] */ 
    _Outptr_opt_  BSTR *pBstrName,
    /* [annotation][out] */ 
    _Outptr_opt_  BSTR *pBstrDocString,
    /* [out] */ DWORD *pdwHelpContext,
    /* [annotation][out] */ 
    _Outptr_opt_  BSTR *pBstrHelpFile);


/* [call_as] */ HRESULT STDMETHODCALLTYPE ITypeInfo_GetDocumentation_Stub( 
    __RPC__in ITypeInfo * This,
    /* [in] */ MEMBERID memid,
    /* [in] */ DWORD refPtrFlags,
    /* [out] */ __RPC__deref_out_opt BSTR *pBstrName,
    /* [out] */ __RPC__deref_out_opt BSTR *pBstrDocString,
    /* [out] */ __RPC__out DWORD *pdwHelpContext,
    /* [out] */ __RPC__deref_out_opt BSTR *pBstrHelpFile);

/* [local] */ HRESULT STDMETHODCALLTYPE ITypeInfo_GetDllEntry_Proxy( 
    ITypeInfo * This,
    /* [in] */ MEMBERID memid,
    /* [in] */ INVOKEKIND invKind,
    /* [annotation][out] */ 
    _Outptr_opt_  BSTR *pBstrDllName,
    /* [annotation][out] */ 
    _Outptr_opt_  BSTR *pBstrName,
    /* [out] */ WORD *pwOrdinal);


/* [call_as] */ HRESULT STDMETHODCALLTYPE ITypeInfo_GetDllEntry_Stub( 
    __RPC__in ITypeInfo * This,
    /* [in] */ MEMBERID memid,
    /* [in] */ INVOKEKIND invKind,
    /* [in] */ DWORD refPtrFlags,
    /* [out] */ __RPC__deref_out_opt BSTR *pBstrDllName,
    /* [out] */ __RPC__deref_out_opt BSTR *pBstrName,
    /* [out] */ __RPC__out WORD *pwOrdinal);

/* [local] */ HRESULT STDMETHODCALLTYPE ITypeInfo_AddressOfMember_Proxy( 
    ITypeInfo * This,
    /* [in] */ MEMBERID memid,
    /* [in] */ INVOKEKIND invKind,
    /* [out] */ PVOID *ppv);


/* [nocode][call_as] */ HRESULT STDMETHODCALLTYPE ITypeInfo_AddressOfMember_Stub( 
    __RPC__in ITypeInfo * This);

/* [local] */ HRESULT STDMETHODCALLTYPE ITypeInfo_CreateInstance_Proxy( 
    ITypeInfo * This,
    /* [in] */ IUnknown *pUnkOuter,
    /* [in] */ REFIID riid,
    /* [iid_is][out] */ PVOID *ppvObj);


/* [call_as] */ HRESULT STDMETHODCALLTYPE ITypeInfo_CreateInstance_Stub( 
    __RPC__in ITypeInfo * This,
    /* [in] */ __RPC__in REFIID riid,
    /* [iid_is][out] */ __RPC__deref_out_opt IUnknown **ppvObj);

/* [local] */ HRESULT STDMETHODCALLTYPE ITypeInfo_GetContainingTypeLib_Proxy( 
    ITypeInfo * This,
    /* [out] */ ITypeLib **ppTLib,
    /* [out] */ UINT *pIndex);


/* [call_as] */ HRESULT STDMETHODCALLTYPE ITypeInfo_GetContainingTypeLib_Stub( 
    __RPC__in ITypeInfo * This,
    /* [out] */ __RPC__deref_out_opt ITypeLib **ppTLib,
    /* [out] */ __RPC__out UINT *pIndex);

/* [local] */ void STDMETHODCALLTYPE ITypeInfo_ReleaseTypeAttr_Proxy( 
    ITypeInfo * This,
    /* [in] */ TYPEATTR *pTypeAttr);


/* [nocode][call_as] */ HRESULT STDMETHODCALLTYPE ITypeInfo_ReleaseTypeAttr_Stub( 
    __RPC__in ITypeInfo * This);

/* [local] */ void STDMETHODCALLTYPE ITypeInfo_ReleaseFuncDesc_Proxy( 
    ITypeInfo * This,
    /* [in] */ FUNCDESC *pFuncDesc);


/* [nocode][call_as] */ HRESULT STDMETHODCALLTYPE ITypeInfo_ReleaseFuncDesc_Stub( 
    __RPC__in ITypeInfo * This);

/* [local] */ void STDMETHODCALLTYPE ITypeInfo_ReleaseVarDesc_Proxy( 
    ITypeInfo * This,
    /* [in] */ VARDESC *pVarDesc);


/* [nocode][call_as] */ HRESULT STDMETHODCALLTYPE ITypeInfo_ReleaseVarDesc_Stub( 
    __RPC__in ITypeInfo * This);

/* [local] */ HRESULT STDMETHODCALLTYPE ITypeInfo2_GetDocumentation2_Proxy( 
    ITypeInfo2 * This,
    /* [in] */ MEMBERID memid,
    /* [in] */ LCID lcid,
    /* [annotation][out] */ 
    _Outptr_opt_  BSTR *pbstrHelpString,
    /* [out] */ DWORD *pdwHelpStringContext,
    /* [annotation][out] */ 
    _Outptr_opt_  BSTR *pbstrHelpStringDll);


/* [call_as] */ HRESULT STDMETHODCALLTYPE ITypeInfo2_GetDocumentation2_Stub( 
    __RPC__in ITypeInfo2 * This,
    /* [in] */ MEMBERID memid,
    /* [in] */ LCID lcid,
    /* [in] */ DWORD refPtrFlags,
    /* [out] */ __RPC__deref_out_opt BSTR *pbstrHelpString,
    /* [out] */ __RPC__out DWORD *pdwHelpStringContext,
    /* [out] */ __RPC__deref_out_opt BSTR *pbstrHelpStringDll);

/* [local] */ UINT STDMETHODCALLTYPE ITypeLib_GetTypeInfoCount_Proxy( 
    ITypeLib * This);


/* [call_as] */ HRESULT STDMETHODCALLTYPE ITypeLib_GetTypeInfoCount_Stub( 
    __RPC__in ITypeLib * This,
    /* [out] */ __RPC__out UINT *pcTInfo);

/* [local] */ HRESULT STDMETHODCALLTYPE ITypeLib_GetLibAttr_Proxy( 
    ITypeLib * This,
    /* [out] */ TLIBATTR **ppTLibAttr);


/* [call_as] */ HRESULT STDMETHODCALLTYPE ITypeLib_GetLibAttr_Stub( 
    __RPC__in ITypeLib * This,
    /* [out] */ __RPC__deref_out_opt LPTLIBATTR *ppTLibAttr,
    /* [out] */ __RPC__out CLEANLOCALSTORAGE *pDummy);

/* [local] */ HRESULT STDMETHODCALLTYPE ITypeLib_GetDocumentation_Proxy( 
    ITypeLib * This,
    /* [in] */ INT index,
    /* [annotation][out] */ 
    _Outptr_opt_  BSTR *pBstrName,
    /* [annotation][out] */ 
    _Outptr_opt_  BSTR *pBstrDocString,
    /* [out] */ DWORD *pdwHelpContext,
    /* [annotation][out] */ 
    _Outptr_opt_  BSTR *pBstrHelpFile);


/* [call_as] */ HRESULT STDMETHODCALLTYPE ITypeLib_GetDocumentation_Stub( 
    __RPC__in ITypeLib * This,
    /* [in] */ INT index,
    /* [in] */ DWORD refPtrFlags,
    /* [out] */ __RPC__deref_out_opt BSTR *pBstrName,
    /* [out] */ __RPC__deref_out_opt BSTR *pBstrDocString,
    /* [out] */ __RPC__out DWORD *pdwHelpContext,
    /* [out] */ __RPC__deref_out_opt BSTR *pBstrHelpFile);

/* [local] */ HRESULT STDMETHODCALLTYPE ITypeLib_IsName_Proxy( 
    ITypeLib * This,
    /* [annotation][out][in] */ 
    __RPC__inout  LPOLESTR szNameBuf,
    /* [in] */ ULONG lHashVal,
    /* [out] */ BOOL *pfName);


/* [call_as] */ HRESULT STDMETHODCALLTYPE ITypeLib_IsName_Stub( 
    __RPC__in ITypeLib * This,
    /* [in] */ __RPC__in LPOLESTR szNameBuf,
    /* [in] */ ULONG lHashVal,
    /* [out] */ __RPC__out BOOL *pfName,
    /* [out] */ __RPC__deref_out_opt BSTR *pBstrLibName);

/* [local] */ HRESULT STDMETHODCALLTYPE ITypeLib_FindName_Proxy( 
    ITypeLib * This,
    /* [annotation][out][in] */ 
    __RPC__inout  LPOLESTR szNameBuf,
    /* [in] */ ULONG lHashVal,
    /* [length_is][size_is][out] */ ITypeInfo **ppTInfo,
    /* [length_is][size_is][out] */ MEMBERID *rgMemId,
    /* [out][in] */ USHORT *pcFound);


/* [call_as] */ HRESULT STDMETHODCALLTYPE ITypeLib_FindName_Stub( 
    __RPC__in ITypeLib * This,
    /* [in] */ __RPC__in LPOLESTR szNameBuf,
    /* [in] */ ULONG lHashVal,
    /* [length_is][size_is][out] */ __RPC__out_ecount_part(*pcFound, *pcFound) ITypeInfo **ppTInfo,
    /* [length_is][size_is][out] */ __RPC__out_ecount_part(*pcFound, *pcFound) MEMBERID *rgMemId,
    /* [out][in] */ __RPC__inout USHORT *pcFound,
    /* [out] */ __RPC__deref_out_opt BSTR *pBstrLibName);

/* [local] */ void STDMETHODCALLTYPE ITypeLib_ReleaseTLibAttr_Proxy( 
    ITypeLib * This,
    /* [in] */ TLIBATTR *pTLibAttr);


/* [nocode][call_as] */ HRESULT STDMETHODCALLTYPE ITypeLib_ReleaseTLibAttr_Stub( 
    __RPC__in ITypeLib * This);

/* [local] */ HRESULT STDMETHODCALLTYPE ITypeLib2_GetLibStatistics_Proxy( 
    ITypeLib2 * This,
    /* [out] */ ULONG *pcUniqueNames,
    /* [out] */ ULONG *pcchUniqueNames);


/* [call_as] */ HRESULT STDMETHODCALLTYPE ITypeLib2_GetLibStatistics_Stub( 
    __RPC__in ITypeLib2 * This,
    /* [out] */ __RPC__out ULONG *pcUniqueNames,
    /* [out] */ __RPC__out ULONG *pcchUniqueNames);

/* [local] */ HRESULT STDMETHODCALLTYPE ITypeLib2_GetDocumentation2_Proxy( 
    ITypeLib2 * This,
    /* [in] */ INT index,
    /* [in] */ LCID lcid,
    /* [annotation][out] */ 
    _Outptr_opt_  BSTR *pbstrHelpString,
    /* [out] */ DWORD *pdwHelpStringContext,
    /* [annotation][out] */ 
    _Outptr_opt_  BSTR *pbstrHelpStringDll);


/* [call_as] */ HRESULT STDMETHODCALLTYPE ITypeLib2_GetDocumentation2_Stub( 
    __RPC__in ITypeLib2 * This,
    /* [in] */ INT index,
    /* [in] */ LCID lcid,
    /* [in] */ DWORD refPtrFlags,
    /* [out] */ __RPC__deref_out_opt BSTR *pbstrHelpString,
    /* [out] */ __RPC__out DWORD *pdwHelpStringContext,
    /* [out] */ __RPC__deref_out_opt BSTR *pbstrHelpStringDll);

/* [local] */ HRESULT STDMETHODCALLTYPE IPropertyBag_Read_Proxy( 
    IPropertyBag * This,
    /* [in] */ LPCOLESTR pszPropName,
    /* [out][in] */ VARIANT *pVar,
    /* [unique][in] */ IErrorLog *pErrorLog);


/* [call_as] */ HRESULT STDMETHODCALLTYPE IPropertyBag_Read_Stub( 
    __RPC__in IPropertyBag * This,
    /* [in] */ __RPC__in LPCOLESTR pszPropName,
    /* [out] */ __RPC__out VARIANT *pVar,
    /* [unique][in] */ __RPC__in_opt IErrorLog *pErrorLog,
    /* [in] */ DWORD varType,
    /* [in] */ __RPC__in_opt IUnknown *pUnkObj);



/* end of Additional Prototypes */

#ifdef __cplusplus
}
#endif

#endif


