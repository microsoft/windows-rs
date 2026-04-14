/*
 *  SyncDeviceService.h 
 *
 *  Contains definitions for the general sync properties and formats 
 *
 *  Copyright (c) Microsoft Corporation, All Rights Reserved.
 *
 */


#ifndef _SYNCDEVICESERVICE_H_
#define _SYNCDEVICESERVICE_H_


/*****************************************************************************
     Service Properties
******************************************************************************/

DEFINE_DEVSVCGUID(NAMESPACE_SyncSvc,
     0x703d392c, 0x532c, 0x4607, 0x91, 0x58, 0x9c, 0xea, 0x74, 0x2f, 0x3a, 0x16 );


/*  PKEY_SyncSvc_SyncFormat
 *
 *  Indicates the format GUID for the object format that is to be used in the
 *  sync operation.
 *  
 *  Type: UInt128
 *  Form: None
 */

DEFINE_DEVSVCPROPKEY(PKEY_SyncSvc_SyncFormat,
     0x703d392c, 0x532c, 0x4607, 0x91, 0x58, 0x9c, 0xea, 0x74, 0x2f, 0x3a, 0x16 ,
     2 );

#define NAME_SyncSvc_SyncFormat L"SyncFormat"


/*  PKEY_SyncSvc_LocalOnlyDelete
 *
 *  Boolean flag indicating whether deletes of objects on the service host
 *  should be treated as "local only" and not propogated to other sync
 *  participants. The alternative is "true sync" in which deletes on the
 *  service host are propogated to all other sync participants.
 *  
 *  Type: UInt8
 *  Form: None
 */

DEFINE_DEVSVCPROPKEY(PKEY_SyncSvc_LocalOnlyDelete,
     0x703d392c, 0x532c, 0x4607, 0x91, 0x58, 0x9c, 0xea, 0x74, 0x2f, 0x3a, 0x16 ,
     3 );

#define NAME_SyncSvc_LocalOnlyDelete L"LocalOnlyDelete"


/*  PKEY_SyncSvc_FilterType
 *
 *  Value describing type of the filter
 *  
 *  Type: UInt8
 *  Form: None
 */

DEFINE_DEVSVCPROPKEY(PKEY_SyncSvc_FilterType,
     0x703d392c, 0x532c, 0x4607, 0x91, 0x58, 0x9c, 0xea, 0x74, 0x2f, 0x3a, 0x16 ,
     4 );

#define NAME_SyncSvc_FilterType L"FilterType"

#define SYNCSVC_FILTER_NONE 0 
#define SYNCSVC_FILTER_CONTACTS_WITH_PHONE 1 
#define SYNCSVC_FILTER_TASK_ACTIVE 2 
#define SYNCSVC_FILTER_CALENDAR_WINDOW_WITH_RECURRENCE 3 


/*  PKEY_SyncSvc_SyncObjectReferences
 *
 *  Value describing whether object references should be included as part of
 *  the sync process or not
 *  
 *  Type: UInt8
 *  Form: Enum
 */

DEFINE_DEVSVCPROPKEY(PKEY_SyncSvc_SyncObjectReferences,
     0x703d392c, 0x532c, 0x4607, 0x91, 0x58, 0x9c, 0xea, 0x74, 0x2f, 0x3a, 0x16 ,
     5 );

#define NAME_SyncSvc_SyncObjectReferences L"SyncObjectReferences"

#define ENUM_SyncSvc_SyncObjectReferencesDisabled 0x00 
#define ENUM_SyncSvc_SyncObjectReferencesEnabled 0xff 


/*****************************************************************************
     Service Object Property Keys
******************************************************************************/

DEFINE_DEVSVCGUID(NAMESPACE_SyncObj,
     0x37364f58, 0x2f74, 0x4981, 0x99, 0xa5, 0x7a, 0xe2, 0x8a, 0xee, 0xe3, 0x19 );


/*  SyncObj.LastAuthorProxyID
 *
 *  Contains a GUID indicating the proxy ID of the last proxy to author the
 *  object
 *  
 *  Type: UInt128
 *  Form: None
 */

DEFINE_DEVSVCPROPKEY(PKEY_SyncObj_LastAuthorProxyID,
     0x37364f58, 0x2f74, 0x4981, 0x99, 0xa5, 0x7a, 0xe2, 0x8a, 0xee, 0xe3, 0x19 ,
     2 );

#define NAME_SyncObj_LastAuthorProxyID L"LastAuthorProxyID"


/*****************************************************************************
     Service Methods
******************************************************************************/


/*  METHOD_SyncSvc_BeginSync
 */

DEFINE_DEVSVCGUID(METHOD_SyncSvc_BeginSync,
     0x63803e07, 0xc713, 0x45d3, 0x81, 0x19, 0x34, 0x79, 0xb3, 0x1d, 0x35, 0x92 );

#define NAME_SyncSvc_BeginSync L"BeginSync"

/*  METHOD_SyncSvc_EndSync
 */

DEFINE_DEVSVCGUID(METHOD_SyncSvc_EndSync,
     0x40f3f0f7, 0xa539, 0x422e, 0x98, 0xdd, 0xfd, 0x8d, 0x38, 0x5c, 0x88, 0x49 );

#define NAME_SyncSvc_EndSync L"EndSync"

#endif /*_SYNCDEVICESERVICE_H_*/
