/*****************************************************************************\
*                                                                             *
* coguid.h -    Master definition of GUIDs for compobj.dll                    *
*                                                                             *
*               OLE Version 2.0                                               *
*                                                                             *
*               Copyright (c) Microsoft Corporation. All rights reserved.     *
*                                                                             *
\*****************************************************************************/

/* this file is the master definition of all GUIDs for the component object
   model and is included in compobj.h.  Some GUIDs for moinkers and storage
   appear here as well.  All of these GUIDs are OLE GUIDs only in the sense
   that part of the GUID range owned by OLE was used to define them.

   NOTE: The second byte of all of these GUIDs is 0.
*/

#if _MSC_VER > 1000
#pragma once
#endif
#include <winapifamily.h>

#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)



DEFINE_GUID(GUID_NULL, 0L, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0);

DEFINE_OLEGUID(IID_IUnknown,            0x00000000L, 0, 0);
DEFINE_OLEGUID(IID_IClassFactory,       0x00000001L, 0, 0);
DEFINE_OLEGUID(IID_IMalloc,             0x00000002L, 0, 0);
DEFINE_OLEGUID(IID_IMarshal,            0x00000003L, 0, 0);

/* RPC related interfaces */
DEFINE_OLEGUID(IID_IRpcChannel,         0x00000004L, 0, 0);
DEFINE_OLEGUID(IID_IRpcStub,            0x00000005L, 0, 0);
DEFINE_OLEGUID(IID_IStubManager,        0x00000006L, 0, 0);
DEFINE_OLEGUID(IID_IRpcProxy,           0x00000007L, 0, 0);
DEFINE_OLEGUID(IID_IProxyManager,       0x00000008L, 0, 0);
DEFINE_OLEGUID(IID_IPSFactory,          0x00000009L, 0, 0);

/* storage related interfaces */
DEFINE_OLEGUID(IID_ILockBytes,          0x0000000aL, 0, 0);
DEFINE_OLEGUID(IID_IStorage,            0x0000000bL, 0, 0);
DEFINE_OLEGUID(IID_IStream,             0x0000000cL, 0, 0);
DEFINE_OLEGUID(IID_IEnumSTATSTG,        0x0000000dL, 0, 0);

/* moniker related interfaces */
DEFINE_OLEGUID(IID_IBindCtx,            0x0000000eL, 0, 0);
DEFINE_OLEGUID(IID_IMoniker,            0x0000000fL, 0, 0);
DEFINE_OLEGUID(IID_IRunningObjectTable, 0x00000010L, 0, 0);
DEFINE_OLEGUID(IID_IInternalMoniker,    0x00000011L, 0, 0);

/* storage related interfaces */
DEFINE_OLEGUID(IID_IRootStorage,        0x00000012L, 0, 0);
DEFINE_OLEGUID(IID_IDfReserved1,        0x00000013L, 0, 0);
DEFINE_OLEGUID(IID_IDfReserved2,        0x00000014L, 0, 0);
DEFINE_OLEGUID(IID_IDfReserved3,        0x00000015L, 0, 0);

/* concurrency releated interfaces */
DEFINE_OLEGUID(IID_IMessageFilter,      0x00000016L, 0, 0);

/* CLSID of standard marshaler */
DEFINE_OLEGUID(CLSID_StdMarshal,        0x00000017L, 0, 0);

/* interface on server for getting info for std marshaler */
DEFINE_OLEGUID(IID_IStdMarshalInfo,     0x00000018L, 0, 0);

/* interface to inform object of number of external connections */
DEFINE_OLEGUID(IID_IExternalConnection, 0x00000019L, 0, 0);

/* CLSID of aggregated standard marshaler */
DEFINE_OLEGUID(CLSID_AggStdMarshal,     0x00000027L, 0, 0);

/* NOTE: LSB 0x33 through 0xff are reserved for future use */


// CLSID of various implementations of ISynchronize
//DEFINE_OLEGUID(CLSID_Synchronize_AutoComplete,     0x00000324L, 0, 0); obsolete
//DEFINE_OLEGUID(CLSID_Synchronize_ManualResetEvent, 0x00000325L, 0, 0); obsolete
//DEFINE_OLEGUID(CLSID_WaitMultiple,                 0x00000326L, 0, 0); obsolete


DEFINE_OLEGUID(CLSID_StdEvent,                     0x0000032b, 0, 0);
DEFINE_OLEGUID(CLSID_ManualResetEvent,             0x0000032c, 0, 0);
DEFINE_OLEGUID(CLSID_SynchronizeContainer,         0x0000032d, 0, 0);
DEFINE_OLEGUID(CLSID_SurrogateManager,             0x0000034c, 0, 0);


#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion

