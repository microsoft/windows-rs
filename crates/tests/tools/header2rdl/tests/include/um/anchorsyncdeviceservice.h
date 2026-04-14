/*
 *  AnchorSyncDeviceService.h 
 *
 *  Contains definitions for the Anchor Sync Device Service 
 *
 *  Copyright (c) Microsoft Corporation, All Rights Reserved.
 *
 */


#ifndef _ANCHORSYNCSERVICE_H_
#define _ANCHORSYNCSERVICE_H_

#include <DeviceServices.h>

#include <SyncDeviceService.h>


/*****************************************************************************
    Anchor Sync Service Info
******************************************************************************/

DEFINE_DEVSVCGUID(SERVICE_AnchorSync,
     0x056d8b9e, 0xad7a, 0x44fc, 0x94, 0x6f, 0x1d, 0x63, 0xa2, 0x5c, 0xda, 0x9a );

#define NAME_AnchorSyncSvc L"AnchorSync"
#define TYPE_AnchorSyncSvc DEVSVCTYPE_ABSTRACT


/*****************************************************************************
    Anchor Sync Service Properties
******************************************************************************/

DEFINE_DEVSVCGUID(NAMESPACE_AnchorSyncSvc,
     0xe65b8fb7, 0x8fc7, 0x4278, 0xb9, 0xa3, 0xba, 0x14, 0xc2, 0xdb, 0x40, 0xfa );


/*  PKEY_AnchorSyncSvc_VersionProps
 *
 *  Provides information about change units and version properties. The
 *  format for the dataset is
 *  
 *  UINT32 Number of change units
 *  UINT128 Namespace GUID for first change unit property key
 *  UINT32 Namespace ID for the first change unit property key
 *  UINT32 Number of properties associated with this change unit
 *  UINT128 Namespace GUID for first property key in change unit
 *  UINT32 Namespace ID for first property key in change unit
 *  ... Repeat for number of property keys
 *  ... Repeat for number of change units
 *  
 *  NOTE: If all change units use the same property key specify a namespace
 *  GUID of GUID_NULL (all 0's) and a namespace ID of 0.
 *  
 *  Type: AUInt8
 *  Form: None
 */

DEFINE_DEVSVCPROPKEY(PKEY_AnchorSyncSvc_VersionProps,
     0xe65b8fb7, 0x8fc7, 0x4278, 0xb9, 0xa3, 0xba, 0x14, 0xc2, 0xdb, 0x40, 0xfa ,
     2 );

#define NAME_AnchorSyncSvc_VersionProps L"AnchorVersionProps"


/*  PKEY_AnchorSyncSvc_ReplicaID
 *
 *  Contains the GUID representing this replica in the sync community.
 *  
 *  Type: UInt128
 *  Form: None
 */

DEFINE_DEVSVCPROPKEY(PKEY_AnchorSyncSvc_ReplicaID,
     0xe65b8fb7, 0x8fc7, 0x4278, 0xb9, 0xa3, 0xba, 0x14, 0xc2, 0xdb, 0x40, 0xfa ,
     3 );

#define NAME_AnchorSyncSvc_ReplicaID L"AnchorReplicaID"


/*  PKEY_AnchorSyncSvc_KnowledgeObjectID
 *
 *  Object ID to be used for the knowledge object
 *  
 *  Type: UInt32
 *  Form: ObjectID
 */

DEFINE_DEVSVCPROPKEY(PKEY_AnchorSyncSvc_KnowledgeObjectID,
     0xe65b8fb7, 0x8fc7, 0x4278, 0xb9, 0xa3, 0xba, 0x14, 0xc2, 0xdb, 0x40, 0xfa ,
     4 );

#define NAME_AnchorSyncSvc_KnowledgeObjectID L"AnchorKnowledgeObjectID"


/*  PKEY_AnchorSyncSvc_LastSyncProxyID
 *
 *  Contains a GUID indicating the last sync proxy to perform a sync operation
 *  
 *  Type: UInt128
 *  Form: None
 */

DEFINE_DEVSVCPROPKEY(PKEY_AnchorSyncSvc_LastSyncProxyID,
     0xe65b8fb7, 0x8fc7, 0x4278, 0xb9, 0xa3, 0xba, 0x14, 0xc2, 0xdb, 0x40, 0xfa ,
     5 );

#define NAME_AnchorSyncSvc_LastSyncProxyID L"AnchorLastSyncProxyID"


/*  PKEY_AnchorSyncSvc_CurrentAnchor
 *
 *  Contains a blob of data representing the current anchor for the device.
 *  As the anchor may be transient depending on the current state of the sync
 *  the value of PKEY_AnchorSyncSvc_CurrentAnchor may not reflect the current
 *  state of the database unless the current session holds a lock (via the
 *  BeginSync method) on the service.
 *  
 *  Type: AUInt8
 *  Form: BinaryArray
 */

DEFINE_DEVSVCPROPKEY(PKEY_AnchorSyncSvc_CurrentAnchor,
     0xe65b8fb7, 0x8fc7, 0x4278, 0xb9, 0xa3, 0xba, 0x14, 0xc2, 0xdb, 0x40, 0xfa ,
     6 );

#define NAME_AnchorSyncSvc_CurrentAnchor L"AnchorCurrentAnchor"


/*  PKEY_AnchorSyncSvc_ProviderVersion
 *
 *  Contains a device defined value giving the version of the provider
 *  currently in use on the device. This version must be incremented whenever
 *  new properties are added to the device implementation so that they will
 *  be recognized and managed as part of synchronization. 0 is reserved.
 *  
 *  Type: UInt16
 *  Form: None
 */

DEFINE_DEVSVCPROPKEY(PKEY_AnchorSyncSvc_ProviderVersion,
     0xe65b8fb7, 0x8fc7, 0x4278, 0xb9, 0xa3, 0xba, 0x14, 0xc2, 0xdb, 0x40, 0xfa ,
     7 );

#define NAME_AnchorSyncSvc_ProviderVersion L"AnchorProviderVersion"


/*  PKEY_AnchorSyncSvc_SyncFormat
 *
 *  Indicates the format GUID for the object format that is to be used in the
 *  sync operation.
 *  
 *  Type: UInt128
 *  Form: None
 */

#define PKEY_AnchorSyncSvc_SyncFormat PKEY_SyncSvc_SyncFormat
#define NAME_AnchorSyncSvc_SyncFormat NAME_SyncSvc_SyncFormat

/*  PKEY_AnchorSyncSvc_LocalOnlyDelete
 *
 *  Boolean flag indicating whether deletes of objects on the service host
 *  should be treated as "local only" and not propogated to other sync
 *  participants. The alternative is "true sync" in which deletes on the
 *  service host are propogated to all other sync participants.
 *  
 *  Type: UInt8
 *  Form: None
 */

#define PKEY_AnchorSyncSvc_LocalOnlyDelete PKEY_SyncSvc_LocalOnlyDelete
#define NAME_AnchorSyncSvc_LocalOnlyDelete NAME_SyncSvc_LocalOnlyDelete

/*  PKEY_AnchorSyncSvc_FilterType
 *
 *  Boolean flag indicating whether the default filter is being applied to
 *  this endpoint. Note that the meaning of the default filter is determined
 *  by the content type service.
 *  
 *  Type: UInt8
 *  Form: None
 */

#define PKEY_AnchorSyncSvc_FilterType PKEY_SyncSvc_FilterType
#define NAME_AnchorSyncSvc_FilterType NAME_SyncSvc_FilterType

/*****************************************************************************
    Anchor Sync Service Object Formats
******************************************************************************/

/*  FORMAT_AnchorSyncKnowledge
 */

DEFINE_DEVSVCGUID(FORMAT_AnchorSyncKnowledge,
     0x37c550bc, 0xf231, 0x4727, 0xbb, 0xbc, 0x4c, 0xb3, 0x3a, 0x3f, 0x3e, 0xcd );

#define NAME_AnchorSyncKnowledge L"AnchorSyncKnowledge"


/*  FORMAT_AnchorSyncSvc_AnchorResults
 *  
 *  GetChangesSinceAnchor results format
 *  
 */

DEFINE_DEVSVCGUID(FORMAT_AnchorSyncSvc_AnchorResults,
     0xf35527c1, 0xce4a, 0x487a, 0x9d, 0x29, 0x93, 0x83, 0x35, 0x69, 0x32, 0x1e );

#define NAME_AnchorResults L"AnchorResults"



/*****************************************************************************
    Anchor Sync Service Object Property Keys
******************************************************************************/

DEFINE_DEVSVCGUID(NAMESPACE_AnchorResults,
     0x516b5dce, 0x8d45, 0x430f, 0x80, 0x5c, 0x25, 0xe5, 0x10, 0x6d, 0x8b, 0x1f );


/*  PKEY_AnchorResults_AnchorState
 *
 *  Output parameter from GetChangesSinceAnchor method. Contains the state
 *  of the current anchor result:
 *  
 *  Type: UInt32
 *  Form: Enum
 */

DEFINE_DEVSVCPROPKEY(PKEY_AnchorResults_AnchorState,
     0x516b5dce, 0x8d45, 0x430f, 0x80, 0x5c, 0x25, 0xe5, 0x10, 0x6d, 0x8b, 0x1f ,
     2 );

#define NAME_AnchorResults_AnchorState L"AnchorState"

#define ENUM_AnchorResults_AnchorStateNormal 0x00000000 
#define ENUM_AnchorResults_AnchorStateInvalid 0x00000001 
#define ENUM_AnchorResults_AnchorStateOld 0x00000002 


/*  PKEY_AnchorResults_Anchor
 *
 *  Input parameter for GetChangesSinceAnchor method. Contains the anchor for
 *  which data is being requested.
 *  
 *  Type: AUInt8
 *  Form: None
 */

DEFINE_DEVSVCPROPKEY(PKEY_AnchorResults_Anchor,
     0x516b5dce, 0x8d45, 0x430f, 0x80, 0x5c, 0x25, 0xe5, 0x10, 0x6d, 0x8b, 0x1f ,
     3 );

#define NAME_AnchorResults_Anchor L"Anchor"


/*  PKEY_AnchorResults_ResultObjectID
 *
 *  Output parameter from GetChangesSinceAnchor method. Contains the object
 *  ID of the AnchorResults object that has the results of the
 *  GetChangesSinceAnchor operation.
 *  
 *  Type: UInt32
 *  Form: ObjectID
 */

DEFINE_DEVSVCPROPKEY(PKEY_AnchorResults_ResultObjectID,
     0x516b5dce, 0x8d45, 0x430f, 0x80, 0x5c, 0x25, 0xe5, 0x10, 0x6d, 0x8b, 0x1f ,
     4 );

#define NAME_AnchorResults_ResultObjectID L"ResultObjectID"


/*****************************************************************************
    Anchor Sync Service Methods
******************************************************************************/


/*  METHOD_AnchorSyncSvc_GetChangesSinceAnchor
 */

DEFINE_DEVSVCGUID(METHOD_AnchorSyncSvc_GetChangesSinceAnchor,
     0x37c550bc, 0xf231, 0x4727, 0xbb, 0xbc, 0x4c, 0xb3, 0x3a, 0x3f, 0x3e, 0xcd );

#define NAME_AnchorSyncSvc_GetChangesSinceAnchor L"GetChangesSinceAnchor"

/*  Inherited methods */

#define METHOD_AnchorSyncSvc_BeginSync METHOD_SyncSvc_BeginSync
#define NAME_AnchorSyncSvc_BeginSync NAME_SyncSvc_BeginSync

#define METHOD_AnchorSyncSvc_EndSync METHOD_SyncSvc_EndSync
#define NAME_AnchorSyncSvc_EndSync NAME_SyncSvc_EndSync


/*****************************************************************************
    Anchor Sync Service Additional Defines
******************************************************************************/

/* ENUM_AnchorResults_ItemState*
 *
 *  This enum is used when encoding the Anchor results stream. It defines the
 *  current state of an object. If a device is capable of distinguishing
 *  between item update and create operations the *_ItemStateCreated and
 *  *_ItemStateUpdated enumerations should be used. If the device cannot
 *  distinuish between a create and an up updated the *_ItemStateChanged result
 *  should be used.
 *  
 *  Type: UInt32
 *  Form: Enum
 */

#define ENUM_AnchorResults_ItemStateInvalid 0x00000000 
#define ENUM_AnchorResults_ItemStateDeleted 0x00000001 
#define ENUM_AnchorResults_ItemStateCreated 0x00000002 
#define ENUM_AnchorResults_ItemStateUpdated 0x00000003 
#define ENUM_AnchorResults_ItemStateChanged 0x00000004 

#endif /*_ANCHORSYNCSERVICE_H_*/
