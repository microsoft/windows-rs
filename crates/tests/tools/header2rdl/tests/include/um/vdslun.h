

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


#ifndef __vdslun_h__
#define __vdslun_h__

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
#include "oaidl.h"
#include "ocidl.h"

#ifdef __cplusplus
extern "C"{
#endif 


/* interface __MIDL_itf_vdslun_0000_0000 */
/* [local] */ 

#include <winapifamily.h>
#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)
#if _MSC_VER >= 1200
#pragma warning(push)
#pragma warning(disable:4820) /* padding added after data member */
#endif
typedef 
enum _VDS_STORAGE_IDENTIFIER_CODE_SET
    {
        VDSStorageIdCodeSetReserved	= 0,
        VDSStorageIdCodeSetBinary	= 1,
        VDSStorageIdCodeSetAscii	= 2,
        VDSStorageIdCodeSetUtf8	= 3
    } 	VDS_STORAGE_IDENTIFIER_CODE_SET;

typedef 
enum _VDS_STORAGE_IDENTIFIER_TYPE
    {
        VDSStorageIdTypeVendorSpecific	= 0,
        VDSStorageIdTypeVendorId	= 1,
        VDSStorageIdTypeEUI64	= 2,
        VDSStorageIdTypeFCPHName	= 3,
        VDSStorageIdTypePortRelative	= 4,
        VDSStorageIdTypeTargetPortGroup	= 5,
        VDSStorageIdTypeLogicalUnitGroup	= 6,
        VDSStorageIdTypeMD5LogicalUnitIdentifier	= 7,
        VDSStorageIdTypeScsiNameString	= 8
    } 	VDS_STORAGE_IDENTIFIER_TYPE;

typedef 
enum _VDS_STORAGE_BUS_TYPE
    {
        VDSBusTypeUnknown	= 0,
        VDSBusTypeScsi	= 0x1,
        VDSBusTypeAtapi	= 0x2,
        VDSBusTypeAta	= 0x3,
        VDSBusType1394	= 0x4,
        VDSBusTypeSsa	= 0x5,
        VDSBusTypeFibre	= 0x6,
        VDSBusTypeUsb	= 0x7,
        VDSBusTypeRAID	= 0x8,
        VDSBusTypeiScsi	= 0x9,
        VDSBusTypeSas	= 0xa,
        VDSBusTypeSata	= 0xb,
        VDSBusTypeSd	= 0xc,
        VDSBusTypeMmc	= 0xd,
        VDSBusTypeMax	= 0xe,
        VDSBusTypeVirtual	= 0xe,
        VDSBusTypeFileBackedVirtual	= 0xf,
        VDSBusTypeSpaces	= 0x10,
        VDSBusTypeNVMe	= 0x11,
        VDSBusTypeScm	= 0x12,
        VDSBusTypeUfs	= 0x13,
        VDSBusTypeMaxReserved	= 0x7f
    } 	VDS_STORAGE_BUS_TYPE;

typedef struct _VDS_STORAGE_IDENTIFIER
    {
    VDS_STORAGE_IDENTIFIER_CODE_SET m_CodeSet;
    VDS_STORAGE_IDENTIFIER_TYPE m_Type;
    ULONG m_cbIdentifier;
    /* [size_is] */ BYTE *m_rgbIdentifier;
    } 	VDS_STORAGE_IDENTIFIER;

typedef struct _VDS_STORAGE_DEVICE_ID_DESCRIPTOR
    {
    ULONG m_version;
    ULONG m_cIdentifiers;
    /* [size_is] */ VDS_STORAGE_IDENTIFIER *m_rgIdentifiers;
    } 	VDS_STORAGE_DEVICE_ID_DESCRIPTOR;

typedef 
enum _VDS_INTERCONNECT_ADDRESS_TYPE
    {
        VDS_IA_UNKNOWN	= 0,
        VDS_IA_FCFS	= 1,
        VDS_IA_FCPH	= 2,
        VDS_IA_FCPH3	= 3,
        VDS_IA_MAC	= 4,
        VDS_IA_SCSI	= 5
    } 	VDS_INTERCONNECT_ADDRESS_TYPE;

typedef struct _VDS_INTERCONNECT
    {
    VDS_INTERCONNECT_ADDRESS_TYPE m_addressType;
    ULONG m_cbPort;
    /* [size_is] */ BYTE *m_pbPort;
    ULONG m_cbAddress;
    /* [size_is] */ BYTE *m_pbAddress;
    } 	VDS_INTERCONNECT;

typedef struct _VDS_LUN_INFORMATION
    {
    ULONG m_version;
    BYTE m_DeviceType;
    BYTE m_DeviceTypeModifier;
    BOOL m_bCommandQueueing;
    VDS_STORAGE_BUS_TYPE m_BusType;
    /* [string] */ char *m_szVendorId;
    /* [string] */ char *m_szProductId;
    /* [string] */ char *m_szProductRevision;
    /* [string] */ char *m_szSerialNumber;
    GUID m_diskSignature;
    VDS_STORAGE_DEVICE_ID_DESCRIPTOR m_deviceIdDescriptor;
    ULONG m_cInterconnects;
    /* [size_is] */ VDS_INTERCONNECT *m_rgInterconnects;
    } 	VDS_LUN_INFORMATION;

#define	VER_VDS_LUN_INFORMATION	( 1 )

#if _MSC_VER >= 1200
#pragma warning(pop)
#endif
#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion


extern RPC_IF_HANDLE __MIDL_itf_vdslun_0000_0000_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_vdslun_0000_0000_v0_0_s_ifspec;

/* Additional Prototypes for ALL interfaces */

/* end of Additional Prototypes */

#ifdef __cplusplus
}
#endif

#endif


