

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

#ifndef __encdec_h__
#define __encdec_h__

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

#ifndef __IETFilterConfig_FWD_DEFINED__
#define __IETFilterConfig_FWD_DEFINED__
typedef interface IETFilterConfig IETFilterConfig;

#endif 	/* __IETFilterConfig_FWD_DEFINED__ */


#ifndef __IDTFilterConfig_FWD_DEFINED__
#define __IDTFilterConfig_FWD_DEFINED__
typedef interface IDTFilterConfig IDTFilterConfig;

#endif 	/* __IDTFilterConfig_FWD_DEFINED__ */


#ifndef __IXDSCodecConfig_FWD_DEFINED__
#define __IXDSCodecConfig_FWD_DEFINED__
typedef interface IXDSCodecConfig IXDSCodecConfig;

#endif 	/* __IXDSCodecConfig_FWD_DEFINED__ */


#ifndef __IDTFilterLicenseRenewal_FWD_DEFINED__
#define __IDTFilterLicenseRenewal_FWD_DEFINED__
typedef interface IDTFilterLicenseRenewal IDTFilterLicenseRenewal;

#endif 	/* __IDTFilterLicenseRenewal_FWD_DEFINED__ */


#ifndef __IPTFilterLicenseRenewal_FWD_DEFINED__
#define __IPTFilterLicenseRenewal_FWD_DEFINED__
typedef interface IPTFilterLicenseRenewal IPTFilterLicenseRenewal;

#endif 	/* __IPTFilterLicenseRenewal_FWD_DEFINED__ */


#ifndef __IMceBurnerControl_FWD_DEFINED__
#define __IMceBurnerControl_FWD_DEFINED__
typedef interface IMceBurnerControl IMceBurnerControl;

#endif 	/* __IMceBurnerControl_FWD_DEFINED__ */


#ifndef __IETFilter_FWD_DEFINED__
#define __IETFilter_FWD_DEFINED__
typedef interface IETFilter IETFilter;

#endif 	/* __IETFilter_FWD_DEFINED__ */


#ifndef __IETFilterEvents_FWD_DEFINED__
#define __IETFilterEvents_FWD_DEFINED__
typedef interface IETFilterEvents IETFilterEvents;

#endif 	/* __IETFilterEvents_FWD_DEFINED__ */


#ifndef __ETFilter_FWD_DEFINED__
#define __ETFilter_FWD_DEFINED__

#ifdef __cplusplus
typedef class ETFilter ETFilter;
#else
typedef struct ETFilter ETFilter;
#endif /* __cplusplus */

#endif 	/* __ETFilter_FWD_DEFINED__ */


#ifndef __IDTFilter_FWD_DEFINED__
#define __IDTFilter_FWD_DEFINED__
typedef interface IDTFilter IDTFilter;

#endif 	/* __IDTFilter_FWD_DEFINED__ */


#ifndef __IDTFilter2_FWD_DEFINED__
#define __IDTFilter2_FWD_DEFINED__
typedef interface IDTFilter2 IDTFilter2;

#endif 	/* __IDTFilter2_FWD_DEFINED__ */


#ifndef __IDTFilter3_FWD_DEFINED__
#define __IDTFilter3_FWD_DEFINED__
typedef interface IDTFilter3 IDTFilter3;

#endif 	/* __IDTFilter3_FWD_DEFINED__ */


#ifndef __IDTFilterEvents_FWD_DEFINED__
#define __IDTFilterEvents_FWD_DEFINED__
typedef interface IDTFilterEvents IDTFilterEvents;

#endif 	/* __IDTFilterEvents_FWD_DEFINED__ */


#ifndef __DTFilter_FWD_DEFINED__
#define __DTFilter_FWD_DEFINED__

#ifdef __cplusplus
typedef class DTFilter DTFilter;
#else
typedef struct DTFilter DTFilter;
#endif /* __cplusplus */

#endif 	/* __DTFilter_FWD_DEFINED__ */


#ifndef __IXDSCodec_FWD_DEFINED__
#define __IXDSCodec_FWD_DEFINED__
typedef interface IXDSCodec IXDSCodec;

#endif 	/* __IXDSCodec_FWD_DEFINED__ */


#ifndef __IXDSCodecEvents_FWD_DEFINED__
#define __IXDSCodecEvents_FWD_DEFINED__
typedef interface IXDSCodecEvents IXDSCodecEvents;

#endif 	/* __IXDSCodecEvents_FWD_DEFINED__ */


#ifndef __XDSCodec_FWD_DEFINED__
#define __XDSCodec_FWD_DEFINED__

#ifdef __cplusplus
typedef class XDSCodec XDSCodec;
#else
typedef struct XDSCodec XDSCodec;
#endif /* __cplusplus */

#endif 	/* __XDSCodec_FWD_DEFINED__ */


#ifndef __CXDSData_FWD_DEFINED__
#define __CXDSData_FWD_DEFINED__

#ifdef __cplusplus
typedef class CXDSData CXDSData;
#else
typedef struct CXDSData CXDSData;
#endif /* __cplusplus */

#endif 	/* __CXDSData_FWD_DEFINED__ */


/* header files for imported files */
#include "oaidl.h"
#include "ocidl.h"

#ifdef __cplusplus
extern "C"{
#endif 


/* interface __MIDL_itf_encdec_0000_0000 */
/* [local] */ 

//+-------------------------------------------------------------------------
//
//  Microsoft Windows
//  Copyright (C) Microsoft Corporation, 2002.
//
//--------------------------------------------------------------------------
#pragma once
#include <winapifamily.h>
#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)
#pragma warning(push)
#pragma warning(disable:4001) 
#pragma once
#pragma warning(push)
#pragma warning(disable:4001) 
#pragma once
#pragma warning(pop)
#pragma warning(pop)
#pragma region Desktop Family
#pragma region Desktop Family
#pragma endregion
typedef 
enum EnTvRat_System
    {
        MPAA	= 0,
        US_TV	= 1,
        Canadian_English	= 2,
        Canadian_French	= 3,
        Reserved4	= 4,
        System5	= 5,
        System6	= 6,
        Reserved7	= 7,
        PBDA	= 8,
        AgeBased	= 9,
        TvRat_kSystems	= 10,
        TvRat_SystemDontKnow	= 255
    } 	EnTvRat_System;

typedef 
enum EnTvRat_GenericLevel
    {
        TvRat_0	= 0,
        TvRat_1	= 1,
        TvRat_2	= 2,
        TvRat_3	= 3,
        TvRat_4	= 4,
        TvRat_5	= 5,
        TvRat_6	= 6,
        TvRat_7	= 7,
        TvRat_8	= 8,
        TvRat_9	= 9,
        TvRat_10	= 10,
        TvRat_11	= 11,
        TvRat_12	= 12,
        TvRat_13	= 13,
        TvRat_14	= 14,
        TvRat_15	= 15,
        TvRat_16	= 16,
        TvRat_17	= 17,
        TvRat_18	= 18,
        TvRat_19	= 19,
        TvRat_20	= 20,
        TvRat_21	= 21,
        TvRat_kLevels	= 22,
        TvRat_Unblock	= -1,
        TvRat_LevelDontKnow	= 255
    } 	EnTvRat_GenericLevel;

typedef 
enum EnTvRat_MPAA
    {
        MPAA_NotApplicable	= TvRat_0,
        MPAA_G	= TvRat_1,
        MPAA_PG	= TvRat_2,
        MPAA_PG13	= TvRat_3,
        MPAA_R	= TvRat_4,
        MPAA_NC17	= TvRat_5,
        MPAA_X	= TvRat_6,
        MPAA_NotRated	= TvRat_7
    } 	EnTvRat_MPAA;

typedef 
enum EnTvRat_US_TV
    {
        US_TV_None	= TvRat_0,
        US_TV_Y	= TvRat_1,
        US_TV_Y7	= TvRat_2,
        US_TV_G	= TvRat_3,
        US_TV_PG	= TvRat_4,
        US_TV_14	= TvRat_5,
        US_TV_MA	= TvRat_6,
        US_TV_None7	= TvRat_7
    } 	EnTvRat_US_TV;

typedef 
enum EnTvRat_CAE_TV
    {
        CAE_TV_Exempt	= TvRat_0,
        CAE_TV_C	= TvRat_1,
        CAE_TV_C8	= TvRat_2,
        CAE_TV_G	= TvRat_3,
        CAE_TV_PG	= TvRat_4,
        CAE_TV_14	= TvRat_5,
        CAE_TV_18	= TvRat_6,
        CAE_TV_Reserved	= TvRat_7
    } 	EnTvRat_CAE_TV;

typedef 
enum EnTvRat_CAF_TV
    {
        CAF_TV_Exempt	= TvRat_0,
        CAF_TV_G	= TvRat_1,
        CAF_TV_8	= TvRat_2,
        CAF_TV_13	= TvRat_3,
        CAF_TV_16	= TvRat_4,
        CAF_TV_18	= TvRat_5,
        CAF_TV_Reserved6	= TvRat_6,
        CAF_TV_Reserved	= TvRat_7
    } 	EnTvRat_CAF_TV;

typedef 
enum BfEnTvRat_GenericAttributes
    {
        BfAttrNone	= 0,
        BfIsBlocked	= 1,
        BfIsAttr_1	= 2,
        BfIsAttr_2	= 4,
        BfIsAttr_3	= 8,
        BfIsAttr_4	= 16,
        BfIsAttr_5	= 32,
        BfIsAttr_6	= 64,
        BfIsAttr_7	= 128,
        BfValidAttrSubmask	= 255
    } 	BfEnTvRat_GenericAttributes;

typedef 
enum BfEnTvRat_Attributes_US_TV
    {
        US_TV_IsBlocked	= BfIsBlocked,
        US_TV_IsViolent	= BfIsAttr_1,
        US_TV_IsSexualSituation	= BfIsAttr_2,
        US_TV_IsAdultLanguage	= BfIsAttr_3,
        US_TV_IsSexuallySuggestiveDialog	= BfIsAttr_4,
        US_TV_ValidAttrSubmask	= 31
    } 	BfEnTvRat_Attributes_US_TV;

typedef 
enum BfEnTvRat_Attributes_MPAA
    {
        MPAA_IsBlocked	= BfIsBlocked,
        MPAA_ValidAttrSubmask	= 1
    } 	BfEnTvRat_Attributes_MPAA;

typedef 
enum BfEnTvRat_Attributes_CAE_TV
    {
        CAE_IsBlocked	= BfIsBlocked,
        CAE_ValidAttrSubmask	= 1
    } 	BfEnTvRat_Attributes_CAE_TV;

typedef 
enum BfEnTvRat_Attributes_CAF_TV
    {
        CAF_IsBlocked	= BfIsBlocked,
        CAF_ValidAttrSubmask	= 1
    } 	BfEnTvRat_Attributes_CAF_TV;

#pragma region Desktop Family
#pragma endregion
#pragma endregion
// {C4C4C4C4-0049-4E2B-98FB-9537F6CE516D}
DEFINE_GUID(SID_DRMSecureServiceChannel,
0xC4C4C4C4, 0x0049, 0x4E2B, 0x98, 0xFB, 0x95, 0x37, 0xF6, 0xCE, 0x51, 0x6D);
// {C4C4C481-0049-4E2B-98FB-9537F6CE516D}
DEFINE_GUID(CLSID_ETFilterEncProperties,
0xC4C4C481, 0x0049, 0x4E2B, 0x98, 0xFB, 0x95, 0x37, 0xF6, 0xCE, 0x51, 0x6D);
// {C4C4C491-0049-4E2B-98FB-9537F6CE516D}
DEFINE_GUID(CLSID_ETFilterTagProperties,
0xC4C4C491, 0x0049, 0x4E2B, 0x98, 0xFB, 0x95, 0x37, 0xF6, 0xCE, 0x51, 0x6D);
// {9CD31617-B303-4f96-8330-2EB173EA4DC6}
DEFINE_GUID(CLSID_PTFilter,
0x9cd31617, 0xb303, 0x4f96, 0x83, 0x30, 0x2e, 0xb1, 0x73, 0xea, 0x4d, 0xc6);
// {C4C4C482-0049-4E2B-98FB-9537F6CE516D}
DEFINE_GUID(CLSID_DTFilterEncProperties,
0xC4C4C482, 0x0049, 0x4E2B, 0x98, 0xFB, 0x95, 0x37, 0xF6, 0xCE, 0x51, 0x6D);
// {C4C4C492-0049-4E2B-98FB-9537F6CE516D}
DEFINE_GUID(CLSID_DTFilterTagProperties,
0xC4C4C492, 0x0049, 0x4E2B, 0x98, 0xFB, 0x95, 0x37, 0xF6, 0xCE, 0x51, 0x6D);
// {C4C4C483-0049-4E2B-98FB-9537F6CE516D}
DEFINE_GUID(CLSID_XDSCodecProperties,
0xC4C4C483, 0x0049, 0x4E2B, 0x98, 0xFB, 0x95, 0x37, 0xF6, 0xCE, 0x51, 0x6D);
// {C4C4C493-0049-4E2B-98FB-9537F6CE516D}
DEFINE_GUID(CLSID_XDSCodecTagProperties,
0xC4C4C493, 0x0049, 0x4E2B, 0x98, 0xFB, 0x95, 0x37, 0xF6, 0xCE, 0x51, 0x6D);
// {C4C4C4FC-0049-4E2B-98FB-9537F6CE516D}
DEFINE_GUID(CLSID_CPCAFiltersCategory,
0xC4C4C4FC, 0x0049, 0x4E2B, 0x98, 0xFB, 0x95, 0x37, 0xF6, 0xCE, 0x51, 0x6D);
// {C4C4C4E0-0049-4E2B-98FB-9537F6CE516D}
DEFINE_GUID(EVENTID_XDSCodecNewXDSRating,
0xC4C4C4E0, 0x0049, 0x4E2B, 0x98, 0xFB, 0x95, 0x37, 0xF6, 0xCE, 0x51, 0x6D);
// {C4C4C4DF-0049-4E2B-98FB-9537F6CE516D}
DEFINE_GUID(EVENTID_XDSCodecDuplicateXDSRating,
0xC4C4C4DF, 0x0049, 0x4E2B, 0x98, 0xFB, 0x95, 0x37, 0xF6, 0xCE, 0x51, 0x6D);
// {C4C4C4E1-0049-4E2B-98FB-9537F6CE516D}
DEFINE_GUID(EVENTID_XDSCodecNewXDSPacket,
0xC4C4C4E1, 0x0049, 0x4E2B, 0x98, 0xFB, 0x95, 0x37, 0xF6, 0xCE, 0x51, 0x6D);
// {C4C4C4E2-0049-4E2B-98FB-9537F6CE516D}
DEFINE_GUID(EVENTID_DTFilterRatingChange,
0xC4C4C4E2, 0x0049, 0x4E2B, 0x98, 0xFB, 0x95, 0x37, 0xF6, 0xCE, 0x51, 0x6D);
// {C4C4C4E3-0049-4E2B-98FB-9537F6CE516D}
DEFINE_GUID(EVENTID_DTFilterRatingsBlock,
0xC4C4C4E3, 0x0049, 0x4E2B, 0x98, 0xFB, 0x95, 0x37, 0xF6, 0xCE, 0x51, 0x6D);
// {C4C4C4E4-0049-4E2B-98FB-9537F6CE516D}
DEFINE_GUID(EVENTID_DTFilterRatingsUnblock,
0xC4C4C4E4, 0x0049, 0x4E2B, 0x98, 0xFB, 0x95, 0x37, 0xF6, 0xCE, 0x51, 0x6D);
// {C4C4C4E5-0049-4E2B-98FB-9537F6CE516D}
DEFINE_GUID(EVENTID_DTFilterXDSPacket,
0xC4C4C4E5, 0x0049, 0x4E2B, 0x98, 0xFB, 0x95, 0x37, 0xF6, 0xCE, 0x51, 0x6D);
// {C4C4C4E6-0049-4E2B-98FB-9537F6CE516D}
DEFINE_GUID(EVENTID_ETFilterEncryptionOn,
0xC4C4C4E6, 0x0049, 0x4E2B, 0x98, 0xFB, 0x95, 0x37, 0xF6, 0xCE, 0x51, 0x6D);
// {C4C4C4E7-0049-4E2B-98FB-9537F6CE516D}
DEFINE_GUID(EVENTID_ETFilterEncryptionOff,
0xC4C4C4E7, 0x0049, 0x4E2B, 0x98, 0xFB, 0x95, 0x37, 0xF6, 0xCE, 0x51, 0x6D);
// {C4C4C4E8-0049-4E2B-98FB-9537F6CE516D}
DEFINE_GUID(EVENTID_DTFilterCOPPUnblock,
0xC4C4C4E8, 0x0049, 0x4E2B, 0x98, 0xFB, 0x95, 0x37, 0xF6, 0xCE, 0x51, 0x6D);
// {C4C4C4E9-0049-4E2B-98FB-9537F6CE516D}
DEFINE_GUID(EVENTID_EncDecFilterError,
0xC4C4C4E9, 0x0049, 0x4E2B, 0x98, 0xFB, 0x95, 0x37, 0xF6, 0xCE, 0x51, 0x6D);
// {C4C4C4EA-0049-4E2B-98FB-9537F6CE516D}
DEFINE_GUID(EVENTID_DTFilterCOPPBlock ,
0xC4C4C4EA, 0x0049, 0x4E2B, 0x98, 0xFB, 0x95, 0x37, 0xF6, 0xCE, 0x51, 0x6D);
// {C4C4C4EB-0049-4E2B-98FB-9537F6CE516D}
DEFINE_GUID(EVENTID_ETFilterCopyOnce,
0xC4C4C4EB, 0x0049, 0x4E2B, 0x98, 0xFB, 0x95, 0x37, 0xF6, 0xCE, 0x51, 0x6D);
// {C4C4C4F0-0049-4E2B-98FB-9537F6CE516D}
DEFINE_GUID(EVENTID_ETFilterCopyNever,
0xC4C4C4F0, 0x0049, 0x4E2B, 0x98, 0xFB, 0x95, 0x37, 0xF6, 0xCE, 0x51, 0x6D);
// {C4C4C4EC-0049-4E2B-98FB-9537F6CE516D}
DEFINE_GUID(EVENTID_DTFilterDataFormatOK,
0xC4C4C4EC, 0x0049, 0x4E2B, 0x98, 0xFB, 0x95, 0x37, 0xF6, 0xCE, 0x51, 0x6D);
// {C4C4C4ED-0049-4E2B-98FB-9537F6CE516D}
DEFINE_GUID(EVENTID_DTFilterDataFormatFailure,
0xC4C4C4ED, 0x0049, 0x4E2B, 0x98, 0xFB, 0x95, 0x37, 0xF6, 0xCE, 0x51, 0x6D);
// {C4C4C4EE-0049-4E2B-98FB-9537F6CE516D}
DEFINE_GUID(EVENTID_ETDTFilterLicenseOK,
0xC4C4C4EE, 0x0049, 0x4E2B, 0x98, 0xFB, 0x95, 0x37, 0xF6, 0xCE, 0x51, 0x6D);
// {C4C4C4EF-0049-4E2B-98FB-9537F6CE516D}
DEFINE_GUID(EVENTID_ETDTFilterLicenseFailure,
0xC4C4C4EF, 0x0049, 0x4E2B, 0x98, 0xFB, 0x95, 0x37, 0xF6, 0xCE, 0x51, 0x6D);
// {C4C4C4D0-0049-4E2B-98FB-9537F6CE516D}
DEFINE_GUID(MEDIASUBTYPE_ETDTFilter_Tagged,
0xC4C4C4D0, 0x0049, 0x4E2B, 0x98, 0xFB, 0x95, 0x37, 0xF6, 0xCE, 0x51, 0x6D);
// {C4C4C4D1-0049-4E2B-98FB-9537F6CE516D}
DEFINE_GUID(FORMATTYPE_ETDTFilter_Tagged,
0xC4C4C4D1, 0x0049, 0x4E2B, 0x98, 0xFB, 0x95, 0x37, 0xF6, 0xCE, 0x51, 0x6D);
// {46adbd28-6fd0-4796-93b2-155c51dc048d}
DEFINE_GUID( MEDIASUBTYPE_CPFilters_Processed, 0x46adbd28, 0x6fd0, 0x4796, 0x93, 0xb2, 0x15, 0x5c, 0x51, 0xdc, 0x4, 0x8d );
// {6739b36f-1d5f-4ac2-8192-28bb0e73d16a}
DEFINE_GUID( FORMATTYPE_CPFilters_Processed, 0x6739b36f, 0x1d5f, 0x4ac2, 0x81, 0x92, 0x28, 0xbb, 0xe, 0x73, 0xd1, 0x6a );
// {4A1B465B-0FB9-4159-AFBD-E33006A0F9F4}
DEFINE_GUID(EVENTID_EncDecFilterEvent, 
0x4a1b465b, 0xfb9, 0x4159, 0xaf, 0xbd, 0xe3, 0x30, 0x6, 0xa0, 0xf9, 0xf4);

enum FormatNotSupportedEvents
    {
        FORMATNOTSUPPORTED_CLEAR	= 0,
        FORMATNOTSUPPORTED_NOTSUPPORTED	= 1
    } ;
// {24B2280A-B2AA-4777-BF65-63F35E7B024A}
DEFINE_GUID(EVENTID_FormatNotSupportedEvent, 
0x24b2280a, 0xb2aa, 0x4777, 0xbf, 0x65, 0x63, 0xf3, 0x5e, 0x7b, 0x2, 0x4a);
// {16155770-AED5-475c-BB98-95A33070DF0C}
DEFINE_GUID(EVENTID_DemultiplexerFilterDiscontinuity, 
0x16155770, 0xaed5, 0x475c, 0xbb, 0x98, 0x95, 0xa3, 0x30, 0x70, 0xdf, 0xc);
// {40749583-6b9d-4eec-b43c-67a1801e1a9b}
DEFINE_GUID( DSATTRIB_WMDRMProtectionInfo, 0x40749583, 0x6b9d, 0x4eec, 0xb4, 0x3c, 0x67, 0xa1, 0x80, 0x1e, 0x1a, 0x9b );
// {e4846dda-5838-42b4-b897-6f7e5faa2f2f}
DEFINE_GUID( DSATTRIB_BadSampleInfo, 0xe4846dda, 0x5838, 0x42b4, 0xb8, 0x97, 0x6f, 0x7e, 0x5f, 0xaa, 0x2f, 0x2f );

#pragma pack(push, 1)
typedef /* [public] */ struct __MIDL___MIDL_itf_encdec_0000_0000_0001
    {
    unsigned short wszKID[ 25 ];
    unsigned __int64 qwCounter;
    unsigned __int64 qwIndex;
    unsigned char bOffset;
    } 	WMDRMProtectionInfo;

typedef /* [public] */ struct __MIDL___MIDL_itf_encdec_0000_0000_0002
    {
    HRESULT hrReason;
    } 	BadSampleInfo;


#pragma pack(pop)
typedef LONGLONG REFERENCE_TIME;

typedef LONG PackedTvRating;

#pragma region Desktop Family
#pragma region Desktop Family
#pragma endregion
typedef /* [v1_enum][uuid] */  DECLSPEC_UUID("25AEE876-3D61-4486-917E-7C0CB3D9983C") 
enum ProtType
    {
        PROT_COPY_FREE	= 1,
        PROT_COPY_ONCE	= 2,
        PROT_COPY_NEVER	= 3,
        PROT_COPY_NEVER_REALLY	= 4,
        PROT_COPY_NO_MORE	= 5,
        PROT_COPY_FREE_CIT	= 6,
        PROT_COPY_BF	= 7,
        PROT_COPY_CN_RECORDING_STOP	= 8,
        PROT_COPY_FREE_SECURE	= 9,
        PROT_COPY_INVALID	= 50
    } 	ProtType;

typedef /* [v1_enum] */ 
enum EncDecEvents
    {
        ENCDEC_CPEVENT	= 0,
        ENCDEC_RECORDING_STATUS	= ( ENCDEC_CPEVENT + 1 ) 
    } 	EncDecEvents;

typedef /* [v1_enum] */ 
enum CPRecordingStatus
    {
        RECORDING_STOPPED	= 0,
        RECORDING_STARTED	= 1
    } 	CPRecordingStatus;

typedef /* [v1_enum] */ 
enum CPEventBitShift
    {
        CPEVENT_BITSHIFT_RATINGS	= 0,
        CPEVENT_BITSHIFT_COPP	= ( CPEVENT_BITSHIFT_RATINGS + 1 ) ,
        CPEVENT_BITSHIFT_LICENSE	= ( CPEVENT_BITSHIFT_COPP + 1 ) ,
        CPEVENT_BITSHIFT_ROLLBACK	= ( CPEVENT_BITSHIFT_LICENSE + 1 ) ,
        CPEVENT_BITSHIFT_SAC	= ( CPEVENT_BITSHIFT_ROLLBACK + 1 ) ,
        CPEVENT_BITSHIFT_DOWNRES	= ( CPEVENT_BITSHIFT_SAC + 1 ) ,
        CPEVENT_BITSHIFT_STUBLIB	= ( CPEVENT_BITSHIFT_DOWNRES + 1 ) ,
        CPEVENT_BITSHIFT_UNTRUSTEDGRAPH	= ( CPEVENT_BITSHIFT_STUBLIB + 1 ) ,
        CPEVENT_BITSHIFT_PENDING_CERTIFICATE	= ( CPEVENT_BITSHIFT_UNTRUSTEDGRAPH + 1 ) ,
        CPEVENT_BITSHIFT_NO_PLAYREADY	= ( CPEVENT_BITSHIFT_PENDING_CERTIFICATE + 1 ) 
    } 	CPEventBitShift;

typedef /* [v1_enum] */ 
enum CPEvents
    {
        CPEVENT_NONE	= 0,
        CPEVENT_RATINGS	= ( CPEVENT_NONE + 1 ) ,
        CPEVENT_COPP	= ( CPEVENT_RATINGS + 1 ) ,
        CPEVENT_LICENSE	= ( CPEVENT_COPP + 1 ) ,
        CPEVENT_ROLLBACK	= ( CPEVENT_LICENSE + 1 ) ,
        CPEVENT_SAC	= ( CPEVENT_ROLLBACK + 1 ) ,
        CPEVENT_DOWNRES	= ( CPEVENT_SAC + 1 ) ,
        CPEVENT_STUBLIB	= ( CPEVENT_DOWNRES + 1 ) ,
        CPEVENT_UNTRUSTEDGRAPH	= ( CPEVENT_STUBLIB + 1 ) ,
        CPEVENT_PROTECTWINDOWED	= ( CPEVENT_UNTRUSTEDGRAPH + 1 ) 
    } 	CPEvents;

typedef /* [v1_enum] */ 
enum RevokedComponent
    {
        REVOKED_COPP	= 0,
        REVOKED_SAC	= ( REVOKED_COPP + 1 ) ,
        REVOKED_APP_STUB	= ( REVOKED_SAC + 1 ) ,
        REVOKED_SECURE_PIPELINE	= ( REVOKED_APP_STUB + 1 ) ,
        REVOKED_MAX_TYPES	= ( REVOKED_SECURE_PIPELINE + 1 ) 
    } 	RevokedComponent;

typedef /* [v1_enum] */ 
enum EnTag_Mode
    {
        EnTag_Remove	= 0,
        EnTag_Once	= 0x1,
        EnTag_Repeat	= 0x2
    } 	EnTag_Mode;

typedef /* [v1_enum][uuid] */  DECLSPEC_UUID("6F8C2442-2BFB-4180-9EE5-EA1FB47AE35C") 
enum COPPEventBlockReason
    {
        COPP_Unknown	= -1,
        COPP_BadDriver	= 0,
        COPP_NoCardHDCPSupport	= 1,
        COPP_NoMonitorHDCPSupport	= 2,
        COPP_BadCertificate	= 3,
        COPP_InvalidBusProtection	= 4,
        COPP_AeroGlassOff	= 5,
        COPP_RogueApp	= 6,
        COPP_ForbiddenVideo	= 7,
        COPP_Activate	= 8,
        COPP_DigitalAudioUnprotected	= 9
    } 	COPPEventBlockReason;

typedef /* [v1_enum][uuid] */  DECLSPEC_UUID("57BCA1BE-DF7A-434e-8B89-26D6A0541FDA") 
enum LicenseEventBlockReason
    {
        LIC_BadLicense	= 0,
        LIC_NeedIndiv	= 1,
        LIC_Expired	= 2,
        LIC_NeedActivation	= 3,
        LIC_ExtenderBlocked	= 4
    } 	LicenseEventBlockReason;

typedef /* [v1_enum][uuid] */  DECLSPEC_UUID("D5CC1CDC-EF31-48dc-95B8-AFD34C08036B") 
enum DownResEventParam
    {
        DOWNRES_Always	= 0,
        DOWNRES_InWindowOnly	= 1,
        DOWNRES_Undefined	= 2
    } 	DownResEventParam;

#pragma region Desktop Family
#pragma endregion
#pragma endregion


extern RPC_IF_HANDLE __MIDL_itf_encdec_0000_0000_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_encdec_0000_0000_v0_0_s_ifspec;

#ifndef __IETFilterConfig_INTERFACE_DEFINED__
#define __IETFilterConfig_INTERFACE_DEFINED__

/* interface IETFilterConfig */
/* [unique][helpstring][uuid][object][restricted] */ 


EXTERN_C const IID IID_IETFilterConfig;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("C4C4C4D1-0049-4E2B-98FB-9537F6CE516D")
    IETFilterConfig : public IUnknown
    {
    public:
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE InitLicense( 
            /* [in] */ int LicenseId) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE GetSecureChannelObject( 
            /* [out] */ __RPC__deref_out_opt IUnknown **ppUnkDRMSecureChannel) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IETFilterConfigVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IETFilterConfig * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IETFilterConfig * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IETFilterConfig * This);
        
        DECLSPEC_XFGVIRT(IETFilterConfig, InitLicense)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *InitLicense )( 
            __RPC__in IETFilterConfig * This,
            /* [in] */ int LicenseId);
        
        DECLSPEC_XFGVIRT(IETFilterConfig, GetSecureChannelObject)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *GetSecureChannelObject )( 
            __RPC__in IETFilterConfig * This,
            /* [out] */ __RPC__deref_out_opt IUnknown **ppUnkDRMSecureChannel);
        
        END_INTERFACE
    } IETFilterConfigVtbl;

    interface IETFilterConfig
    {
        CONST_VTBL struct IETFilterConfigVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IETFilterConfig_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IETFilterConfig_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IETFilterConfig_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IETFilterConfig_InitLicense(This,LicenseId)	\
    ( (This)->lpVtbl -> InitLicense(This,LicenseId) ) 

#define IETFilterConfig_GetSecureChannelObject(This,ppUnkDRMSecureChannel)	\
    ( (This)->lpVtbl -> GetSecureChannelObject(This,ppUnkDRMSecureChannel) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IETFilterConfig_INTERFACE_DEFINED__ */


#ifndef __IDTFilterConfig_INTERFACE_DEFINED__
#define __IDTFilterConfig_INTERFACE_DEFINED__

/* interface IDTFilterConfig */
/* [unique][helpstring][uuid][object][restricted] */ 


EXTERN_C const IID IID_IDTFilterConfig;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("C4C4C4D2-0049-4E2B-98FB-9537F6CE516D")
    IDTFilterConfig : public IUnknown
    {
    public:
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE GetSecureChannelObject( 
            /* [out] */ __RPC__deref_out_opt IUnknown **ppUnkDRMSecureChannel) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IDTFilterConfigVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IDTFilterConfig * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IDTFilterConfig * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IDTFilterConfig * This);
        
        DECLSPEC_XFGVIRT(IDTFilterConfig, GetSecureChannelObject)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *GetSecureChannelObject )( 
            __RPC__in IDTFilterConfig * This,
            /* [out] */ __RPC__deref_out_opt IUnknown **ppUnkDRMSecureChannel);
        
        END_INTERFACE
    } IDTFilterConfigVtbl;

    interface IDTFilterConfig
    {
        CONST_VTBL struct IDTFilterConfigVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IDTFilterConfig_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IDTFilterConfig_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IDTFilterConfig_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IDTFilterConfig_GetSecureChannelObject(This,ppUnkDRMSecureChannel)	\
    ( (This)->lpVtbl -> GetSecureChannelObject(This,ppUnkDRMSecureChannel) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IDTFilterConfig_INTERFACE_DEFINED__ */


#ifndef __IXDSCodecConfig_INTERFACE_DEFINED__
#define __IXDSCodecConfig_INTERFACE_DEFINED__

/* interface IXDSCodecConfig */
/* [unique][helpstring][uuid][object] */ 


EXTERN_C const IID IID_IXDSCodecConfig;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("C4C4C4D3-0049-4E2B-98FB-9537F6CE516D")
    IXDSCodecConfig : public IUnknown
    {
    public:
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE GetSecureChannelObject( 
            /* [out] */ __RPC__deref_out_opt IUnknown **ppUnkDRMSecureChannel) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE SetPauseBufferTime( 
            /* [in] */ DWORD dwPauseBufferTime) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IXDSCodecConfigVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IXDSCodecConfig * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IXDSCodecConfig * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IXDSCodecConfig * This);
        
        DECLSPEC_XFGVIRT(IXDSCodecConfig, GetSecureChannelObject)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *GetSecureChannelObject )( 
            __RPC__in IXDSCodecConfig * This,
            /* [out] */ __RPC__deref_out_opt IUnknown **ppUnkDRMSecureChannel);
        
        DECLSPEC_XFGVIRT(IXDSCodecConfig, SetPauseBufferTime)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *SetPauseBufferTime )( 
            __RPC__in IXDSCodecConfig * This,
            /* [in] */ DWORD dwPauseBufferTime);
        
        END_INTERFACE
    } IXDSCodecConfigVtbl;

    interface IXDSCodecConfig
    {
        CONST_VTBL struct IXDSCodecConfigVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IXDSCodecConfig_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IXDSCodecConfig_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IXDSCodecConfig_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IXDSCodecConfig_GetSecureChannelObject(This,ppUnkDRMSecureChannel)	\
    ( (This)->lpVtbl -> GetSecureChannelObject(This,ppUnkDRMSecureChannel) ) 

#define IXDSCodecConfig_SetPauseBufferTime(This,dwPauseBufferTime)	\
    ( (This)->lpVtbl -> SetPauseBufferTime(This,dwPauseBufferTime) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IXDSCodecConfig_INTERFACE_DEFINED__ */


#ifndef __IDTFilterLicenseRenewal_INTERFACE_DEFINED__
#define __IDTFilterLicenseRenewal_INTERFACE_DEFINED__

/* interface IDTFilterLicenseRenewal */
/* [unique][helpstring][uuid][object] */ 


EXTERN_C const IID IID_IDTFilterLicenseRenewal;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("8A78B317-E405-4a43-994A-620D8F5CE25E")
    IDTFilterLicenseRenewal : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetLicenseRenewalData( 
            /* [out] */ __RPC__deref_out_opt LPWSTR *ppwszFileName,
            /* [out] */ __RPC__deref_out_opt LPWSTR *ppwszExpiredKid,
            /* [out] */ __RPC__deref_out_opt LPWSTR *ppwszTunerId) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IDTFilterLicenseRenewalVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IDTFilterLicenseRenewal * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IDTFilterLicenseRenewal * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IDTFilterLicenseRenewal * This);
        
        DECLSPEC_XFGVIRT(IDTFilterLicenseRenewal, GetLicenseRenewalData)
        HRESULT ( STDMETHODCALLTYPE *GetLicenseRenewalData )( 
            __RPC__in IDTFilterLicenseRenewal * This,
            /* [out] */ __RPC__deref_out_opt LPWSTR *ppwszFileName,
            /* [out] */ __RPC__deref_out_opt LPWSTR *ppwszExpiredKid,
            /* [out] */ __RPC__deref_out_opt LPWSTR *ppwszTunerId);
        
        END_INTERFACE
    } IDTFilterLicenseRenewalVtbl;

    interface IDTFilterLicenseRenewal
    {
        CONST_VTBL struct IDTFilterLicenseRenewalVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IDTFilterLicenseRenewal_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IDTFilterLicenseRenewal_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IDTFilterLicenseRenewal_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IDTFilterLicenseRenewal_GetLicenseRenewalData(This,ppwszFileName,ppwszExpiredKid,ppwszTunerId)	\
    ( (This)->lpVtbl -> GetLicenseRenewalData(This,ppwszFileName,ppwszExpiredKid,ppwszTunerId) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IDTFilterLicenseRenewal_INTERFACE_DEFINED__ */


#ifndef __IPTFilterLicenseRenewal_INTERFACE_DEFINED__
#define __IPTFilterLicenseRenewal_INTERFACE_DEFINED__

/* interface IPTFilterLicenseRenewal */
/* [unique][helpstring][uuid][object] */ 


EXTERN_C const IID IID_IPTFilterLicenseRenewal;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("26D836A5-0C15-44c7-AC59-B0DA8728F240")
    IPTFilterLicenseRenewal : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE RenewLicenses( 
            /* [in] */ __RPC__in WCHAR *wszFileName,
            /* [in] */ __RPC__in WCHAR *wszExpiredKid,
            /* [in] */ DWORD dwCallersId,
            /* [in] */ BOOL bHighPriority) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE CancelLicenseRenewal( void) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IPTFilterLicenseRenewalVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IPTFilterLicenseRenewal * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IPTFilterLicenseRenewal * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IPTFilterLicenseRenewal * This);
        
        DECLSPEC_XFGVIRT(IPTFilterLicenseRenewal, RenewLicenses)
        HRESULT ( STDMETHODCALLTYPE *RenewLicenses )( 
            __RPC__in IPTFilterLicenseRenewal * This,
            /* [in] */ __RPC__in WCHAR *wszFileName,
            /* [in] */ __RPC__in WCHAR *wszExpiredKid,
            /* [in] */ DWORD dwCallersId,
            /* [in] */ BOOL bHighPriority);
        
        DECLSPEC_XFGVIRT(IPTFilterLicenseRenewal, CancelLicenseRenewal)
        HRESULT ( STDMETHODCALLTYPE *CancelLicenseRenewal )( 
            __RPC__in IPTFilterLicenseRenewal * This);
        
        END_INTERFACE
    } IPTFilterLicenseRenewalVtbl;

    interface IPTFilterLicenseRenewal
    {
        CONST_VTBL struct IPTFilterLicenseRenewalVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IPTFilterLicenseRenewal_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IPTFilterLicenseRenewal_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IPTFilterLicenseRenewal_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IPTFilterLicenseRenewal_RenewLicenses(This,wszFileName,wszExpiredKid,dwCallersId,bHighPriority)	\
    ( (This)->lpVtbl -> RenewLicenses(This,wszFileName,wszExpiredKid,dwCallersId,bHighPriority) ) 

#define IPTFilterLicenseRenewal_CancelLicenseRenewal(This)	\
    ( (This)->lpVtbl -> CancelLicenseRenewal(This) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IPTFilterLicenseRenewal_INTERFACE_DEFINED__ */


#ifndef __IMceBurnerControl_INTERFACE_DEFINED__
#define __IMceBurnerControl_INTERFACE_DEFINED__

/* interface IMceBurnerControl */
/* [unique][helpstring][uuid][object] */ 


EXTERN_C const IID IID_IMceBurnerControl;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("5A86B91A-E71E-46c1-88A9-9BB338710552")
    IMceBurnerControl : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetBurnerNoDecryption( void) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IMceBurnerControlVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IMceBurnerControl * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IMceBurnerControl * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IMceBurnerControl * This);
        
        DECLSPEC_XFGVIRT(IMceBurnerControl, GetBurnerNoDecryption)
        HRESULT ( STDMETHODCALLTYPE *GetBurnerNoDecryption )( 
            __RPC__in IMceBurnerControl * This);
        
        END_INTERFACE
    } IMceBurnerControlVtbl;

    interface IMceBurnerControl
    {
        CONST_VTBL struct IMceBurnerControlVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IMceBurnerControl_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IMceBurnerControl_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IMceBurnerControl_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IMceBurnerControl_GetBurnerNoDecryption(This)	\
    ( (This)->lpVtbl -> GetBurnerNoDecryption(This) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IMceBurnerControl_INTERFACE_DEFINED__ */



#ifndef __EncDec_LIBRARY_DEFINED__
#define __EncDec_LIBRARY_DEFINED__

/* library EncDec */
/* [helpstring][version][uuid] */ 


EXTERN_C const IID LIBID_EncDec;

#ifndef __IETFilter_INTERFACE_DEFINED__
#define __IETFilter_INTERFACE_DEFINED__

/* interface IETFilter */
/* [unique][helpstring][uuid][object] */ 


EXTERN_C const IID IID_IETFilter;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("C4C4C4B1-0049-4E2B-98FB-9537F6CE516D")
    IETFilter : public IUnknown
    {
    public:
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_EvalRatObjOK( 
            /* [retval][out] */ __RPC__out HRESULT *pHrCoCreateRetVal) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE GetCurrRating( 
            /* [out] */ __RPC__out EnTvRat_System *pEnSystem,
            /* [out] */ __RPC__out EnTvRat_GenericLevel *pEnRating,
            /* [out] */ __RPC__out LONG *plbfEnAttr) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE GetCurrLicenseExpDate( 
            /* [in] */ __RPC__in ProtType *protType,
            /* [out] */ __RPC__out long *lpDateTime) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE GetLastErrorCode( void) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE SetRecordingOn( 
            BOOL fRecState) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IETFilterVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IETFilter * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IETFilter * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IETFilter * This);
        
        DECLSPEC_XFGVIRT(IETFilter, get_EvalRatObjOK)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_EvalRatObjOK )( 
            __RPC__in IETFilter * This,
            /* [retval][out] */ __RPC__out HRESULT *pHrCoCreateRetVal);
        
        DECLSPEC_XFGVIRT(IETFilter, GetCurrRating)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *GetCurrRating )( 
            __RPC__in IETFilter * This,
            /* [out] */ __RPC__out EnTvRat_System *pEnSystem,
            /* [out] */ __RPC__out EnTvRat_GenericLevel *pEnRating,
            /* [out] */ __RPC__out LONG *plbfEnAttr);
        
        DECLSPEC_XFGVIRT(IETFilter, GetCurrLicenseExpDate)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *GetCurrLicenseExpDate )( 
            __RPC__in IETFilter * This,
            /* [in] */ __RPC__in ProtType *protType,
            /* [out] */ __RPC__out long *lpDateTime);
        
        DECLSPEC_XFGVIRT(IETFilter, GetLastErrorCode)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *GetLastErrorCode )( 
            __RPC__in IETFilter * This);
        
        DECLSPEC_XFGVIRT(IETFilter, SetRecordingOn)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *SetRecordingOn )( 
            __RPC__in IETFilter * This,
            BOOL fRecState);
        
        END_INTERFACE
    } IETFilterVtbl;

    interface IETFilter
    {
        CONST_VTBL struct IETFilterVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IETFilter_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IETFilter_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IETFilter_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IETFilter_get_EvalRatObjOK(This,pHrCoCreateRetVal)	\
    ( (This)->lpVtbl -> get_EvalRatObjOK(This,pHrCoCreateRetVal) ) 

#define IETFilter_GetCurrRating(This,pEnSystem,pEnRating,plbfEnAttr)	\
    ( (This)->lpVtbl -> GetCurrRating(This,pEnSystem,pEnRating,plbfEnAttr) ) 

#define IETFilter_GetCurrLicenseExpDate(This,protType,lpDateTime)	\
    ( (This)->lpVtbl -> GetCurrLicenseExpDate(This,protType,lpDateTime) ) 

#define IETFilter_GetLastErrorCode(This)	\
    ( (This)->lpVtbl -> GetLastErrorCode(This) ) 

#define IETFilter_SetRecordingOn(This,fRecState)	\
    ( (This)->lpVtbl -> SetRecordingOn(This,fRecState) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IETFilter_INTERFACE_DEFINED__ */


#ifndef __IETFilterEvents_DISPINTERFACE_DEFINED__
#define __IETFilterEvents_DISPINTERFACE_DEFINED__

/* dispinterface IETFilterEvents */
/* [helpstring][uuid] */ 


EXTERN_C const IID DIID_IETFilterEvents;

#if defined(__cplusplus) && !defined(CINTERFACE)

    MIDL_INTERFACE("C4C4C4C1-0049-4E2B-98FB-9537F6CE516D")
    IETFilterEvents : public IDispatch
    {
    };
    
#else 	/* C style interface */

    typedef struct IETFilterEventsVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IETFilterEvents * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IETFilterEvents * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IETFilterEvents * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IETFilterEvents * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IETFilterEvents * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IETFilterEvents * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IETFilterEvents * This,
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
    } IETFilterEventsVtbl;

    interface IETFilterEvents
    {
        CONST_VTBL struct IETFilterEventsVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IETFilterEvents_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IETFilterEvents_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IETFilterEvents_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IETFilterEvents_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IETFilterEvents_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IETFilterEvents_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IETFilterEvents_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */


#endif 	/* __IETFilterEvents_DISPINTERFACE_DEFINED__ */


EXTERN_C const CLSID CLSID_ETFilter;

#ifdef __cplusplus

class DECLSPEC_UUID("C4C4C4F1-0049-4E2B-98FB-9537F6CE516D")
ETFilter;
#endif

#ifndef __IDTFilter_INTERFACE_DEFINED__
#define __IDTFilter_INTERFACE_DEFINED__

/* interface IDTFilter */
/* [unique][helpstring][uuid][object] */ 


EXTERN_C const IID IID_IDTFilter;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("C4C4C4B2-0049-4E2B-98FB-9537F6CE516D")
    IDTFilter : public IUnknown
    {
    public:
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_EvalRatObjOK( 
            /* [retval][out] */ __RPC__out HRESULT *pHrCoCreateRetVal) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE GetCurrRating( 
            /* [out] */ __RPC__out EnTvRat_System *pEnSystem,
            /* [out] */ __RPC__out EnTvRat_GenericLevel *pEnRating,
            /* [out] */ __RPC__out LONG *plbfEnAttr) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_BlockedRatingAttributes( 
            /* [in] */ EnTvRat_System enSystem,
            /* [in] */ EnTvRat_GenericLevel enLevel,
            /* [retval][out] */ __RPC__out LONG *plbfEnAttr) = 0;
        
        virtual /* [helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_BlockedRatingAttributes( 
            /* [in] */ EnTvRat_System enSystem,
            /* [in] */ EnTvRat_GenericLevel enLevel,
            /* [in] */ LONG lbfAttrs) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_BlockUnRated( 
            /* [retval][out] */ __RPC__out BOOL *pfBlockUnRatedShows) = 0;
        
        virtual /* [helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_BlockUnRated( 
            /* [in] */ BOOL fBlockUnRatedShows) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_BlockUnRatedDelay( 
            /* [retval][out] */ __RPC__out LONG *pmsecsDelayBeforeBlock) = 0;
        
        virtual /* [helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_BlockUnRatedDelay( 
            /* [in] */ LONG msecsDelayBeforeBlock) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IDTFilterVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IDTFilter * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IDTFilter * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IDTFilter * This);
        
        DECLSPEC_XFGVIRT(IDTFilter, get_EvalRatObjOK)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_EvalRatObjOK )( 
            __RPC__in IDTFilter * This,
            /* [retval][out] */ __RPC__out HRESULT *pHrCoCreateRetVal);
        
        DECLSPEC_XFGVIRT(IDTFilter, GetCurrRating)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *GetCurrRating )( 
            __RPC__in IDTFilter * This,
            /* [out] */ __RPC__out EnTvRat_System *pEnSystem,
            /* [out] */ __RPC__out EnTvRat_GenericLevel *pEnRating,
            /* [out] */ __RPC__out LONG *plbfEnAttr);
        
        DECLSPEC_XFGVIRT(IDTFilter, get_BlockedRatingAttributes)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_BlockedRatingAttributes )( 
            __RPC__in IDTFilter * This,
            /* [in] */ EnTvRat_System enSystem,
            /* [in] */ EnTvRat_GenericLevel enLevel,
            /* [retval][out] */ __RPC__out LONG *plbfEnAttr);
        
        DECLSPEC_XFGVIRT(IDTFilter, put_BlockedRatingAttributes)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_BlockedRatingAttributes )( 
            __RPC__in IDTFilter * This,
            /* [in] */ EnTvRat_System enSystem,
            /* [in] */ EnTvRat_GenericLevel enLevel,
            /* [in] */ LONG lbfAttrs);
        
        DECLSPEC_XFGVIRT(IDTFilter, get_BlockUnRated)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_BlockUnRated )( 
            __RPC__in IDTFilter * This,
            /* [retval][out] */ __RPC__out BOOL *pfBlockUnRatedShows);
        
        DECLSPEC_XFGVIRT(IDTFilter, put_BlockUnRated)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_BlockUnRated )( 
            __RPC__in IDTFilter * This,
            /* [in] */ BOOL fBlockUnRatedShows);
        
        DECLSPEC_XFGVIRT(IDTFilter, get_BlockUnRatedDelay)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_BlockUnRatedDelay )( 
            __RPC__in IDTFilter * This,
            /* [retval][out] */ __RPC__out LONG *pmsecsDelayBeforeBlock);
        
        DECLSPEC_XFGVIRT(IDTFilter, put_BlockUnRatedDelay)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_BlockUnRatedDelay )( 
            __RPC__in IDTFilter * This,
            /* [in] */ LONG msecsDelayBeforeBlock);
        
        END_INTERFACE
    } IDTFilterVtbl;

    interface IDTFilter
    {
        CONST_VTBL struct IDTFilterVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IDTFilter_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IDTFilter_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IDTFilter_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IDTFilter_get_EvalRatObjOK(This,pHrCoCreateRetVal)	\
    ( (This)->lpVtbl -> get_EvalRatObjOK(This,pHrCoCreateRetVal) ) 

#define IDTFilter_GetCurrRating(This,pEnSystem,pEnRating,plbfEnAttr)	\
    ( (This)->lpVtbl -> GetCurrRating(This,pEnSystem,pEnRating,plbfEnAttr) ) 

#define IDTFilter_get_BlockedRatingAttributes(This,enSystem,enLevel,plbfEnAttr)	\
    ( (This)->lpVtbl -> get_BlockedRatingAttributes(This,enSystem,enLevel,plbfEnAttr) ) 

#define IDTFilter_put_BlockedRatingAttributes(This,enSystem,enLevel,lbfAttrs)	\
    ( (This)->lpVtbl -> put_BlockedRatingAttributes(This,enSystem,enLevel,lbfAttrs) ) 

#define IDTFilter_get_BlockUnRated(This,pfBlockUnRatedShows)	\
    ( (This)->lpVtbl -> get_BlockUnRated(This,pfBlockUnRatedShows) ) 

#define IDTFilter_put_BlockUnRated(This,fBlockUnRatedShows)	\
    ( (This)->lpVtbl -> put_BlockUnRated(This,fBlockUnRatedShows) ) 

#define IDTFilter_get_BlockUnRatedDelay(This,pmsecsDelayBeforeBlock)	\
    ( (This)->lpVtbl -> get_BlockUnRatedDelay(This,pmsecsDelayBeforeBlock) ) 

#define IDTFilter_put_BlockUnRatedDelay(This,msecsDelayBeforeBlock)	\
    ( (This)->lpVtbl -> put_BlockUnRatedDelay(This,msecsDelayBeforeBlock) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IDTFilter_INTERFACE_DEFINED__ */


#ifndef __IDTFilter2_INTERFACE_DEFINED__
#define __IDTFilter2_INTERFACE_DEFINED__

/* interface IDTFilter2 */
/* [unique][helpstring][uuid][object] */ 


EXTERN_C const IID IID_IDTFilter2;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("C4C4C4B4-0049-4E2B-98FB-9537F6CE516D")
    IDTFilter2 : public IDTFilter
    {
    public:
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_ChallengeUrl( 
            /* [out] */ __RPC__deref_out_opt BSTR *pbstrChallengeUrl) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE GetCurrLicenseExpDate( 
            /* [in] */ __RPC__in ProtType *protType,
            /* [out] */ __RPC__out long *lpDateTime) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE GetLastErrorCode( void) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IDTFilter2Vtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IDTFilter2 * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IDTFilter2 * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IDTFilter2 * This);
        
        DECLSPEC_XFGVIRT(IDTFilter, get_EvalRatObjOK)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_EvalRatObjOK )( 
            __RPC__in IDTFilter2 * This,
            /* [retval][out] */ __RPC__out HRESULT *pHrCoCreateRetVal);
        
        DECLSPEC_XFGVIRT(IDTFilter, GetCurrRating)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *GetCurrRating )( 
            __RPC__in IDTFilter2 * This,
            /* [out] */ __RPC__out EnTvRat_System *pEnSystem,
            /* [out] */ __RPC__out EnTvRat_GenericLevel *pEnRating,
            /* [out] */ __RPC__out LONG *plbfEnAttr);
        
        DECLSPEC_XFGVIRT(IDTFilter, get_BlockedRatingAttributes)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_BlockedRatingAttributes )( 
            __RPC__in IDTFilter2 * This,
            /* [in] */ EnTvRat_System enSystem,
            /* [in] */ EnTvRat_GenericLevel enLevel,
            /* [retval][out] */ __RPC__out LONG *plbfEnAttr);
        
        DECLSPEC_XFGVIRT(IDTFilter, put_BlockedRatingAttributes)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_BlockedRatingAttributes )( 
            __RPC__in IDTFilter2 * This,
            /* [in] */ EnTvRat_System enSystem,
            /* [in] */ EnTvRat_GenericLevel enLevel,
            /* [in] */ LONG lbfAttrs);
        
        DECLSPEC_XFGVIRT(IDTFilter, get_BlockUnRated)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_BlockUnRated )( 
            __RPC__in IDTFilter2 * This,
            /* [retval][out] */ __RPC__out BOOL *pfBlockUnRatedShows);
        
        DECLSPEC_XFGVIRT(IDTFilter, put_BlockUnRated)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_BlockUnRated )( 
            __RPC__in IDTFilter2 * This,
            /* [in] */ BOOL fBlockUnRatedShows);
        
        DECLSPEC_XFGVIRT(IDTFilter, get_BlockUnRatedDelay)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_BlockUnRatedDelay )( 
            __RPC__in IDTFilter2 * This,
            /* [retval][out] */ __RPC__out LONG *pmsecsDelayBeforeBlock);
        
        DECLSPEC_XFGVIRT(IDTFilter, put_BlockUnRatedDelay)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_BlockUnRatedDelay )( 
            __RPC__in IDTFilter2 * This,
            /* [in] */ LONG msecsDelayBeforeBlock);
        
        DECLSPEC_XFGVIRT(IDTFilter2, get_ChallengeUrl)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_ChallengeUrl )( 
            __RPC__in IDTFilter2 * This,
            /* [out] */ __RPC__deref_out_opt BSTR *pbstrChallengeUrl);
        
        DECLSPEC_XFGVIRT(IDTFilter2, GetCurrLicenseExpDate)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *GetCurrLicenseExpDate )( 
            __RPC__in IDTFilter2 * This,
            /* [in] */ __RPC__in ProtType *protType,
            /* [out] */ __RPC__out long *lpDateTime);
        
        DECLSPEC_XFGVIRT(IDTFilter2, GetLastErrorCode)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *GetLastErrorCode )( 
            __RPC__in IDTFilter2 * This);
        
        END_INTERFACE
    } IDTFilter2Vtbl;

    interface IDTFilter2
    {
        CONST_VTBL struct IDTFilter2Vtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IDTFilter2_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IDTFilter2_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IDTFilter2_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IDTFilter2_get_EvalRatObjOK(This,pHrCoCreateRetVal)	\
    ( (This)->lpVtbl -> get_EvalRatObjOK(This,pHrCoCreateRetVal) ) 

#define IDTFilter2_GetCurrRating(This,pEnSystem,pEnRating,plbfEnAttr)	\
    ( (This)->lpVtbl -> GetCurrRating(This,pEnSystem,pEnRating,plbfEnAttr) ) 

#define IDTFilter2_get_BlockedRatingAttributes(This,enSystem,enLevel,plbfEnAttr)	\
    ( (This)->lpVtbl -> get_BlockedRatingAttributes(This,enSystem,enLevel,plbfEnAttr) ) 

#define IDTFilter2_put_BlockedRatingAttributes(This,enSystem,enLevel,lbfAttrs)	\
    ( (This)->lpVtbl -> put_BlockedRatingAttributes(This,enSystem,enLevel,lbfAttrs) ) 

#define IDTFilter2_get_BlockUnRated(This,pfBlockUnRatedShows)	\
    ( (This)->lpVtbl -> get_BlockUnRated(This,pfBlockUnRatedShows) ) 

#define IDTFilter2_put_BlockUnRated(This,fBlockUnRatedShows)	\
    ( (This)->lpVtbl -> put_BlockUnRated(This,fBlockUnRatedShows) ) 

#define IDTFilter2_get_BlockUnRatedDelay(This,pmsecsDelayBeforeBlock)	\
    ( (This)->lpVtbl -> get_BlockUnRatedDelay(This,pmsecsDelayBeforeBlock) ) 

#define IDTFilter2_put_BlockUnRatedDelay(This,msecsDelayBeforeBlock)	\
    ( (This)->lpVtbl -> put_BlockUnRatedDelay(This,msecsDelayBeforeBlock) ) 


#define IDTFilter2_get_ChallengeUrl(This,pbstrChallengeUrl)	\
    ( (This)->lpVtbl -> get_ChallengeUrl(This,pbstrChallengeUrl) ) 

#define IDTFilter2_GetCurrLicenseExpDate(This,protType,lpDateTime)	\
    ( (This)->lpVtbl -> GetCurrLicenseExpDate(This,protType,lpDateTime) ) 

#define IDTFilter2_GetLastErrorCode(This)	\
    ( (This)->lpVtbl -> GetLastErrorCode(This) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IDTFilter2_INTERFACE_DEFINED__ */


#ifndef __IDTFilter3_INTERFACE_DEFINED__
#define __IDTFilter3_INTERFACE_DEFINED__

/* interface IDTFilter3 */
/* [unique][helpstring][uuid][object] */ 


EXTERN_C const IID IID_IDTFilter3;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("513998cc-e929-4cdf-9fbd-bad1e0314866")
    IDTFilter3 : public IDTFilter2
    {
    public:
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE GetProtectionType( 
            /* [out] */ __RPC__out ProtType *pProtectionType) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE LicenseHasExpirationDate( 
            /* [out] */ __RPC__out BOOL *pfLicenseHasExpirationDate) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE SetRights( 
            /* [in] */ __RPC__in BSTR bstrRights) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IDTFilter3Vtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IDTFilter3 * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IDTFilter3 * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IDTFilter3 * This);
        
        DECLSPEC_XFGVIRT(IDTFilter, get_EvalRatObjOK)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_EvalRatObjOK )( 
            __RPC__in IDTFilter3 * This,
            /* [retval][out] */ __RPC__out HRESULT *pHrCoCreateRetVal);
        
        DECLSPEC_XFGVIRT(IDTFilter, GetCurrRating)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *GetCurrRating )( 
            __RPC__in IDTFilter3 * This,
            /* [out] */ __RPC__out EnTvRat_System *pEnSystem,
            /* [out] */ __RPC__out EnTvRat_GenericLevel *pEnRating,
            /* [out] */ __RPC__out LONG *plbfEnAttr);
        
        DECLSPEC_XFGVIRT(IDTFilter, get_BlockedRatingAttributes)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_BlockedRatingAttributes )( 
            __RPC__in IDTFilter3 * This,
            /* [in] */ EnTvRat_System enSystem,
            /* [in] */ EnTvRat_GenericLevel enLevel,
            /* [retval][out] */ __RPC__out LONG *plbfEnAttr);
        
        DECLSPEC_XFGVIRT(IDTFilter, put_BlockedRatingAttributes)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_BlockedRatingAttributes )( 
            __RPC__in IDTFilter3 * This,
            /* [in] */ EnTvRat_System enSystem,
            /* [in] */ EnTvRat_GenericLevel enLevel,
            /* [in] */ LONG lbfAttrs);
        
        DECLSPEC_XFGVIRT(IDTFilter, get_BlockUnRated)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_BlockUnRated )( 
            __RPC__in IDTFilter3 * This,
            /* [retval][out] */ __RPC__out BOOL *pfBlockUnRatedShows);
        
        DECLSPEC_XFGVIRT(IDTFilter, put_BlockUnRated)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_BlockUnRated )( 
            __RPC__in IDTFilter3 * This,
            /* [in] */ BOOL fBlockUnRatedShows);
        
        DECLSPEC_XFGVIRT(IDTFilter, get_BlockUnRatedDelay)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_BlockUnRatedDelay )( 
            __RPC__in IDTFilter3 * This,
            /* [retval][out] */ __RPC__out LONG *pmsecsDelayBeforeBlock);
        
        DECLSPEC_XFGVIRT(IDTFilter, put_BlockUnRatedDelay)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_BlockUnRatedDelay )( 
            __RPC__in IDTFilter3 * This,
            /* [in] */ LONG msecsDelayBeforeBlock);
        
        DECLSPEC_XFGVIRT(IDTFilter2, get_ChallengeUrl)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_ChallengeUrl )( 
            __RPC__in IDTFilter3 * This,
            /* [out] */ __RPC__deref_out_opt BSTR *pbstrChallengeUrl);
        
        DECLSPEC_XFGVIRT(IDTFilter2, GetCurrLicenseExpDate)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *GetCurrLicenseExpDate )( 
            __RPC__in IDTFilter3 * This,
            /* [in] */ __RPC__in ProtType *protType,
            /* [out] */ __RPC__out long *lpDateTime);
        
        DECLSPEC_XFGVIRT(IDTFilter2, GetLastErrorCode)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *GetLastErrorCode )( 
            __RPC__in IDTFilter3 * This);
        
        DECLSPEC_XFGVIRT(IDTFilter3, GetProtectionType)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *GetProtectionType )( 
            __RPC__in IDTFilter3 * This,
            /* [out] */ __RPC__out ProtType *pProtectionType);
        
        DECLSPEC_XFGVIRT(IDTFilter3, LicenseHasExpirationDate)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *LicenseHasExpirationDate )( 
            __RPC__in IDTFilter3 * This,
            /* [out] */ __RPC__out BOOL *pfLicenseHasExpirationDate);
        
        DECLSPEC_XFGVIRT(IDTFilter3, SetRights)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *SetRights )( 
            __RPC__in IDTFilter3 * This,
            /* [in] */ __RPC__in BSTR bstrRights);
        
        END_INTERFACE
    } IDTFilter3Vtbl;

    interface IDTFilter3
    {
        CONST_VTBL struct IDTFilter3Vtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IDTFilter3_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IDTFilter3_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IDTFilter3_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IDTFilter3_get_EvalRatObjOK(This,pHrCoCreateRetVal)	\
    ( (This)->lpVtbl -> get_EvalRatObjOK(This,pHrCoCreateRetVal) ) 

#define IDTFilter3_GetCurrRating(This,pEnSystem,pEnRating,plbfEnAttr)	\
    ( (This)->lpVtbl -> GetCurrRating(This,pEnSystem,pEnRating,plbfEnAttr) ) 

#define IDTFilter3_get_BlockedRatingAttributes(This,enSystem,enLevel,plbfEnAttr)	\
    ( (This)->lpVtbl -> get_BlockedRatingAttributes(This,enSystem,enLevel,plbfEnAttr) ) 

#define IDTFilter3_put_BlockedRatingAttributes(This,enSystem,enLevel,lbfAttrs)	\
    ( (This)->lpVtbl -> put_BlockedRatingAttributes(This,enSystem,enLevel,lbfAttrs) ) 

#define IDTFilter3_get_BlockUnRated(This,pfBlockUnRatedShows)	\
    ( (This)->lpVtbl -> get_BlockUnRated(This,pfBlockUnRatedShows) ) 

#define IDTFilter3_put_BlockUnRated(This,fBlockUnRatedShows)	\
    ( (This)->lpVtbl -> put_BlockUnRated(This,fBlockUnRatedShows) ) 

#define IDTFilter3_get_BlockUnRatedDelay(This,pmsecsDelayBeforeBlock)	\
    ( (This)->lpVtbl -> get_BlockUnRatedDelay(This,pmsecsDelayBeforeBlock) ) 

#define IDTFilter3_put_BlockUnRatedDelay(This,msecsDelayBeforeBlock)	\
    ( (This)->lpVtbl -> put_BlockUnRatedDelay(This,msecsDelayBeforeBlock) ) 


#define IDTFilter3_get_ChallengeUrl(This,pbstrChallengeUrl)	\
    ( (This)->lpVtbl -> get_ChallengeUrl(This,pbstrChallengeUrl) ) 

#define IDTFilter3_GetCurrLicenseExpDate(This,protType,lpDateTime)	\
    ( (This)->lpVtbl -> GetCurrLicenseExpDate(This,protType,lpDateTime) ) 

#define IDTFilter3_GetLastErrorCode(This)	\
    ( (This)->lpVtbl -> GetLastErrorCode(This) ) 


#define IDTFilter3_GetProtectionType(This,pProtectionType)	\
    ( (This)->lpVtbl -> GetProtectionType(This,pProtectionType) ) 

#define IDTFilter3_LicenseHasExpirationDate(This,pfLicenseHasExpirationDate)	\
    ( (This)->lpVtbl -> LicenseHasExpirationDate(This,pfLicenseHasExpirationDate) ) 

#define IDTFilter3_SetRights(This,bstrRights)	\
    ( (This)->lpVtbl -> SetRights(This,bstrRights) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IDTFilter3_INTERFACE_DEFINED__ */


#ifndef __IDTFilterEvents_DISPINTERFACE_DEFINED__
#define __IDTFilterEvents_DISPINTERFACE_DEFINED__

/* dispinterface IDTFilterEvents */
/* [helpstring][uuid] */ 


EXTERN_C const IID DIID_IDTFilterEvents;

#if defined(__cplusplus) && !defined(CINTERFACE)

    MIDL_INTERFACE("C4C4C4C2-0049-4E2B-98FB-9537F6CE516D")
    IDTFilterEvents : public IDispatch
    {
    };
    
#else 	/* C style interface */

    typedef struct IDTFilterEventsVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IDTFilterEvents * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IDTFilterEvents * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IDTFilterEvents * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IDTFilterEvents * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IDTFilterEvents * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IDTFilterEvents * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IDTFilterEvents * This,
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
    } IDTFilterEventsVtbl;

    interface IDTFilterEvents
    {
        CONST_VTBL struct IDTFilterEventsVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IDTFilterEvents_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IDTFilterEvents_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IDTFilterEvents_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IDTFilterEvents_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IDTFilterEvents_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IDTFilterEvents_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IDTFilterEvents_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */


#endif 	/* __IDTFilterEvents_DISPINTERFACE_DEFINED__ */


EXTERN_C const CLSID CLSID_DTFilter;

#ifdef __cplusplus

class DECLSPEC_UUID("C4C4C4F2-0049-4E2B-98FB-9537F6CE516D")
DTFilter;
#endif

#ifndef __IXDSCodec_INTERFACE_DEFINED__
#define __IXDSCodec_INTERFACE_DEFINED__

/* interface IXDSCodec */
/* [unique][helpstring][uuid][object] */ 


EXTERN_C const IID IID_IXDSCodec;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("C4C4C4B3-0049-4E2B-98FB-9537F6CE516D")
    IXDSCodec : public IUnknown
    {
    public:
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_XDSToRatObjOK( 
            /* [retval][out] */ __RPC__out HRESULT *pHrCoCreateRetVal) = 0;
        
        virtual /* [helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_CCSubstreamService( 
            /* [in] */ long SubstreamMask) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_CCSubstreamService( 
            /* [retval][out] */ __RPC__out long *pSubstreamMask) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE GetContentAdvisoryRating( 
            /* [out] */ __RPC__out PackedTvRating *pRat,
            /* [out] */ __RPC__out long *pPktSeqID,
            /* [out] */ __RPC__out long *pCallSeqID,
            /* [out] */ __RPC__out REFERENCE_TIME *pTimeStart,
            /* [out] */ __RPC__out REFERENCE_TIME *pTimeEnd) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE GetXDSPacket( 
            /* [out] */ __RPC__out long *pXDSClassPkt,
            /* [out] */ __RPC__out long *pXDSTypePkt,
            /* [out] */ __RPC__deref_out_opt BSTR *pBstrXDSPkt,
            /* [out] */ __RPC__out long *pPktSeqID,
            /* [out] */ __RPC__out long *pCallSeqID,
            /* [out] */ __RPC__out REFERENCE_TIME *pTimeStart,
            /* [out] */ __RPC__out REFERENCE_TIME *pTimeEnd) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE GetCurrLicenseExpDate( 
            /* [in] */ __RPC__in ProtType *protType,
            /* [out] */ __RPC__out long *lpDateTime) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE GetLastErrorCode( void) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IXDSCodecVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IXDSCodec * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IXDSCodec * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IXDSCodec * This);
        
        DECLSPEC_XFGVIRT(IXDSCodec, get_XDSToRatObjOK)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_XDSToRatObjOK )( 
            __RPC__in IXDSCodec * This,
            /* [retval][out] */ __RPC__out HRESULT *pHrCoCreateRetVal);
        
        DECLSPEC_XFGVIRT(IXDSCodec, put_CCSubstreamService)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_CCSubstreamService )( 
            __RPC__in IXDSCodec * This,
            /* [in] */ long SubstreamMask);
        
        DECLSPEC_XFGVIRT(IXDSCodec, get_CCSubstreamService)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_CCSubstreamService )( 
            __RPC__in IXDSCodec * This,
            /* [retval][out] */ __RPC__out long *pSubstreamMask);
        
        DECLSPEC_XFGVIRT(IXDSCodec, GetContentAdvisoryRating)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *GetContentAdvisoryRating )( 
            __RPC__in IXDSCodec * This,
            /* [out] */ __RPC__out PackedTvRating *pRat,
            /* [out] */ __RPC__out long *pPktSeqID,
            /* [out] */ __RPC__out long *pCallSeqID,
            /* [out] */ __RPC__out REFERENCE_TIME *pTimeStart,
            /* [out] */ __RPC__out REFERENCE_TIME *pTimeEnd);
        
        DECLSPEC_XFGVIRT(IXDSCodec, GetXDSPacket)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *GetXDSPacket )( 
            __RPC__in IXDSCodec * This,
            /* [out] */ __RPC__out long *pXDSClassPkt,
            /* [out] */ __RPC__out long *pXDSTypePkt,
            /* [out] */ __RPC__deref_out_opt BSTR *pBstrXDSPkt,
            /* [out] */ __RPC__out long *pPktSeqID,
            /* [out] */ __RPC__out long *pCallSeqID,
            /* [out] */ __RPC__out REFERENCE_TIME *pTimeStart,
            /* [out] */ __RPC__out REFERENCE_TIME *pTimeEnd);
        
        DECLSPEC_XFGVIRT(IXDSCodec, GetCurrLicenseExpDate)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *GetCurrLicenseExpDate )( 
            __RPC__in IXDSCodec * This,
            /* [in] */ __RPC__in ProtType *protType,
            /* [out] */ __RPC__out long *lpDateTime);
        
        DECLSPEC_XFGVIRT(IXDSCodec, GetLastErrorCode)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *GetLastErrorCode )( 
            __RPC__in IXDSCodec * This);
        
        END_INTERFACE
    } IXDSCodecVtbl;

    interface IXDSCodec
    {
        CONST_VTBL struct IXDSCodecVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IXDSCodec_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IXDSCodec_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IXDSCodec_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IXDSCodec_get_XDSToRatObjOK(This,pHrCoCreateRetVal)	\
    ( (This)->lpVtbl -> get_XDSToRatObjOK(This,pHrCoCreateRetVal) ) 

#define IXDSCodec_put_CCSubstreamService(This,SubstreamMask)	\
    ( (This)->lpVtbl -> put_CCSubstreamService(This,SubstreamMask) ) 

#define IXDSCodec_get_CCSubstreamService(This,pSubstreamMask)	\
    ( (This)->lpVtbl -> get_CCSubstreamService(This,pSubstreamMask) ) 

#define IXDSCodec_GetContentAdvisoryRating(This,pRat,pPktSeqID,pCallSeqID,pTimeStart,pTimeEnd)	\
    ( (This)->lpVtbl -> GetContentAdvisoryRating(This,pRat,pPktSeqID,pCallSeqID,pTimeStart,pTimeEnd) ) 

#define IXDSCodec_GetXDSPacket(This,pXDSClassPkt,pXDSTypePkt,pBstrXDSPkt,pPktSeqID,pCallSeqID,pTimeStart,pTimeEnd)	\
    ( (This)->lpVtbl -> GetXDSPacket(This,pXDSClassPkt,pXDSTypePkt,pBstrXDSPkt,pPktSeqID,pCallSeqID,pTimeStart,pTimeEnd) ) 

#define IXDSCodec_GetCurrLicenseExpDate(This,protType,lpDateTime)	\
    ( (This)->lpVtbl -> GetCurrLicenseExpDate(This,protType,lpDateTime) ) 

#define IXDSCodec_GetLastErrorCode(This)	\
    ( (This)->lpVtbl -> GetLastErrorCode(This) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IXDSCodec_INTERFACE_DEFINED__ */


#ifndef __IXDSCodecEvents_DISPINTERFACE_DEFINED__
#define __IXDSCodecEvents_DISPINTERFACE_DEFINED__

/* dispinterface IXDSCodecEvents */
/* [helpstring][uuid] */ 


EXTERN_C const IID DIID_IXDSCodecEvents;

#if defined(__cplusplus) && !defined(CINTERFACE)

    MIDL_INTERFACE("C4C4C4C3-0049-4E2B-98FB-9537F6CE516D")
    IXDSCodecEvents : public IDispatch
    {
    };
    
#else 	/* C style interface */

    typedef struct IXDSCodecEventsVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IXDSCodecEvents * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IXDSCodecEvents * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IXDSCodecEvents * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IXDSCodecEvents * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IXDSCodecEvents * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IXDSCodecEvents * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IXDSCodecEvents * This,
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
    } IXDSCodecEventsVtbl;

    interface IXDSCodecEvents
    {
        CONST_VTBL struct IXDSCodecEventsVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IXDSCodecEvents_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IXDSCodecEvents_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IXDSCodecEvents_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IXDSCodecEvents_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IXDSCodecEvents_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IXDSCodecEvents_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IXDSCodecEvents_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */


#endif 	/* __IXDSCodecEvents_DISPINTERFACE_DEFINED__ */


EXTERN_C const CLSID CLSID_XDSCodec;

#ifdef __cplusplus

class DECLSPEC_UUID("C4C4C4F3-0049-4E2B-98FB-9537F6CE516D")
XDSCodec;
#endif

EXTERN_C const CLSID CLSID_CXDSData;

#ifdef __cplusplus

class DECLSPEC_UUID("C4C4C4F4-0049-4E2B-98FB-9537F6CE516D")
CXDSData;
#endif
#endif /* __EncDec_LIBRARY_DEFINED__ */

/* interface __MIDL_itf_encdec_0000_0007 */
/* [local] */ 

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion


extern RPC_IF_HANDLE __MIDL_itf_encdec_0000_0007_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_encdec_0000_0007_v0_0_s_ifspec;

/* Additional Prototypes for ALL interfaces */

/* end of Additional Prototypes */

#ifdef __cplusplus
}
#endif

#endif


