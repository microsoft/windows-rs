

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

#ifndef __msclus_h__
#define __msclus_h__

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

#ifndef __ClusApplication_FWD_DEFINED__
#define __ClusApplication_FWD_DEFINED__

#ifdef __cplusplus
typedef class ClusApplication ClusApplication;
#else
typedef struct ClusApplication ClusApplication;
#endif /* __cplusplus */

#endif 	/* __ClusApplication_FWD_DEFINED__ */


#ifndef __Cluster_FWD_DEFINED__
#define __Cluster_FWD_DEFINED__

#ifdef __cplusplus
typedef class Cluster Cluster;
#else
typedef struct Cluster Cluster;
#endif /* __cplusplus */

#endif 	/* __Cluster_FWD_DEFINED__ */


#ifndef __ClusVersion_FWD_DEFINED__
#define __ClusVersion_FWD_DEFINED__

#ifdef __cplusplus
typedef class ClusVersion ClusVersion;
#else
typedef struct ClusVersion ClusVersion;
#endif /* __cplusplus */

#endif 	/* __ClusVersion_FWD_DEFINED__ */


#ifndef __ClusResType_FWD_DEFINED__
#define __ClusResType_FWD_DEFINED__

#ifdef __cplusplus
typedef class ClusResType ClusResType;
#else
typedef struct ClusResType ClusResType;
#endif /* __cplusplus */

#endif 	/* __ClusResType_FWD_DEFINED__ */


#ifndef __ClusProperty_FWD_DEFINED__
#define __ClusProperty_FWD_DEFINED__

#ifdef __cplusplus
typedef class ClusProperty ClusProperty;
#else
typedef struct ClusProperty ClusProperty;
#endif /* __cplusplus */

#endif 	/* __ClusProperty_FWD_DEFINED__ */


#ifndef __ClusProperties_FWD_DEFINED__
#define __ClusProperties_FWD_DEFINED__

#ifdef __cplusplus
typedef class ClusProperties ClusProperties;
#else
typedef struct ClusProperties ClusProperties;
#endif /* __cplusplus */

#endif 	/* __ClusProperties_FWD_DEFINED__ */


#ifndef __DomainNames_FWD_DEFINED__
#define __DomainNames_FWD_DEFINED__

#ifdef __cplusplus
typedef class DomainNames DomainNames;
#else
typedef struct DomainNames DomainNames;
#endif /* __cplusplus */

#endif 	/* __DomainNames_FWD_DEFINED__ */


#ifndef __ClusNetwork_FWD_DEFINED__
#define __ClusNetwork_FWD_DEFINED__

#ifdef __cplusplus
typedef class ClusNetwork ClusNetwork;
#else
typedef struct ClusNetwork ClusNetwork;
#endif /* __cplusplus */

#endif 	/* __ClusNetwork_FWD_DEFINED__ */


#ifndef __ClusNetInterface_FWD_DEFINED__
#define __ClusNetInterface_FWD_DEFINED__

#ifdef __cplusplus
typedef class ClusNetInterface ClusNetInterface;
#else
typedef struct ClusNetInterface ClusNetInterface;
#endif /* __cplusplus */

#endif 	/* __ClusNetInterface_FWD_DEFINED__ */


#ifndef __ClusNetInterfaces_FWD_DEFINED__
#define __ClusNetInterfaces_FWD_DEFINED__

#ifdef __cplusplus
typedef class ClusNetInterfaces ClusNetInterfaces;
#else
typedef struct ClusNetInterfaces ClusNetInterfaces;
#endif /* __cplusplus */

#endif 	/* __ClusNetInterfaces_FWD_DEFINED__ */


#ifndef __ClusResDependencies_FWD_DEFINED__
#define __ClusResDependencies_FWD_DEFINED__

#ifdef __cplusplus
typedef class ClusResDependencies ClusResDependencies;
#else
typedef struct ClusResDependencies ClusResDependencies;
#endif /* __cplusplus */

#endif 	/* __ClusResDependencies_FWD_DEFINED__ */


#ifndef __ClusResGroupResources_FWD_DEFINED__
#define __ClusResGroupResources_FWD_DEFINED__

#ifdef __cplusplus
typedef class ClusResGroupResources ClusResGroupResources;
#else
typedef struct ClusResGroupResources ClusResGroupResources;
#endif /* __cplusplus */

#endif 	/* __ClusResGroupResources_FWD_DEFINED__ */


#ifndef __ClusResTypeResources_FWD_DEFINED__
#define __ClusResTypeResources_FWD_DEFINED__

#ifdef __cplusplus
typedef class ClusResTypeResources ClusResTypeResources;
#else
typedef struct ClusResTypeResources ClusResTypeResources;
#endif /* __cplusplus */

#endif 	/* __ClusResTypeResources_FWD_DEFINED__ */


#ifndef __ClusResGroupPreferredOwnerNodes_FWD_DEFINED__
#define __ClusResGroupPreferredOwnerNodes_FWD_DEFINED__

#ifdef __cplusplus
typedef class ClusResGroupPreferredOwnerNodes ClusResGroupPreferredOwnerNodes;
#else
typedef struct ClusResGroupPreferredOwnerNodes ClusResGroupPreferredOwnerNodes;
#endif /* __cplusplus */

#endif 	/* __ClusResGroupPreferredOwnerNodes_FWD_DEFINED__ */


#ifndef __ClusResPossibleOwnerNodes_FWD_DEFINED__
#define __ClusResPossibleOwnerNodes_FWD_DEFINED__

#ifdef __cplusplus
typedef class ClusResPossibleOwnerNodes ClusResPossibleOwnerNodes;
#else
typedef struct ClusResPossibleOwnerNodes ClusResPossibleOwnerNodes;
#endif /* __cplusplus */

#endif 	/* __ClusResPossibleOwnerNodes_FWD_DEFINED__ */


#ifndef __ClusNetworks_FWD_DEFINED__
#define __ClusNetworks_FWD_DEFINED__

#ifdef __cplusplus
typedef class ClusNetworks ClusNetworks;
#else
typedef struct ClusNetworks ClusNetworks;
#endif /* __cplusplus */

#endif 	/* __ClusNetworks_FWD_DEFINED__ */


#ifndef __ClusNetworkNetInterfaces_FWD_DEFINED__
#define __ClusNetworkNetInterfaces_FWD_DEFINED__

#ifdef __cplusplus
typedef class ClusNetworkNetInterfaces ClusNetworkNetInterfaces;
#else
typedef struct ClusNetworkNetInterfaces ClusNetworkNetInterfaces;
#endif /* __cplusplus */

#endif 	/* __ClusNetworkNetInterfaces_FWD_DEFINED__ */


#ifndef __ClusNodeNetInterfaces_FWD_DEFINED__
#define __ClusNodeNetInterfaces_FWD_DEFINED__

#ifdef __cplusplus
typedef class ClusNodeNetInterfaces ClusNodeNetInterfaces;
#else
typedef struct ClusNodeNetInterfaces ClusNodeNetInterfaces;
#endif /* __cplusplus */

#endif 	/* __ClusNodeNetInterfaces_FWD_DEFINED__ */


#ifndef __ClusRefObject_FWD_DEFINED__
#define __ClusRefObject_FWD_DEFINED__

#ifdef __cplusplus
typedef class ClusRefObject ClusRefObject;
#else
typedef struct ClusRefObject ClusRefObject;
#endif /* __cplusplus */

#endif 	/* __ClusRefObject_FWD_DEFINED__ */


#ifndef __ClusterNames_FWD_DEFINED__
#define __ClusterNames_FWD_DEFINED__

#ifdef __cplusplus
typedef class ClusterNames ClusterNames;
#else
typedef struct ClusterNames ClusterNames;
#endif /* __cplusplus */

#endif 	/* __ClusterNames_FWD_DEFINED__ */


#ifndef __ClusNode_FWD_DEFINED__
#define __ClusNode_FWD_DEFINED__

#ifdef __cplusplus
typedef class ClusNode ClusNode;
#else
typedef struct ClusNode ClusNode;
#endif /* __cplusplus */

#endif 	/* __ClusNode_FWD_DEFINED__ */


#ifndef __ClusNodes_FWD_DEFINED__
#define __ClusNodes_FWD_DEFINED__

#ifdef __cplusplus
typedef class ClusNodes ClusNodes;
#else
typedef struct ClusNodes ClusNodes;
#endif /* __cplusplus */

#endif 	/* __ClusNodes_FWD_DEFINED__ */


#ifndef __ClusResGroup_FWD_DEFINED__
#define __ClusResGroup_FWD_DEFINED__

#ifdef __cplusplus
typedef class ClusResGroup ClusResGroup;
#else
typedef struct ClusResGroup ClusResGroup;
#endif /* __cplusplus */

#endif 	/* __ClusResGroup_FWD_DEFINED__ */


#ifndef __ClusResGroups_FWD_DEFINED__
#define __ClusResGroups_FWD_DEFINED__

#ifdef __cplusplus
typedef class ClusResGroups ClusResGroups;
#else
typedef struct ClusResGroups ClusResGroups;
#endif /* __cplusplus */

#endif 	/* __ClusResGroups_FWD_DEFINED__ */


#ifndef __ClusResource_FWD_DEFINED__
#define __ClusResource_FWD_DEFINED__

#ifdef __cplusplus
typedef class ClusResource ClusResource;
#else
typedef struct ClusResource ClusResource;
#endif /* __cplusplus */

#endif 	/* __ClusResource_FWD_DEFINED__ */


#ifndef __ClusResources_FWD_DEFINED__
#define __ClusResources_FWD_DEFINED__

#ifdef __cplusplus
typedef class ClusResources ClusResources;
#else
typedef struct ClusResources ClusResources;
#endif /* __cplusplus */

#endif 	/* __ClusResources_FWD_DEFINED__ */


#ifndef __ClusResTypes_FWD_DEFINED__
#define __ClusResTypes_FWD_DEFINED__

#ifdef __cplusplus
typedef class ClusResTypes ClusResTypes;
#else
typedef struct ClusResTypes ClusResTypes;
#endif /* __cplusplus */

#endif 	/* __ClusResTypes_FWD_DEFINED__ */


#ifndef __ClusResTypePossibleOwnerNodes_FWD_DEFINED__
#define __ClusResTypePossibleOwnerNodes_FWD_DEFINED__

#ifdef __cplusplus
typedef class ClusResTypePossibleOwnerNodes ClusResTypePossibleOwnerNodes;
#else
typedef struct ClusResTypePossibleOwnerNodes ClusResTypePossibleOwnerNodes;
#endif /* __cplusplus */

#endif 	/* __ClusResTypePossibleOwnerNodes_FWD_DEFINED__ */


#ifndef __ClusPropertyValue_FWD_DEFINED__
#define __ClusPropertyValue_FWD_DEFINED__

#ifdef __cplusplus
typedef class ClusPropertyValue ClusPropertyValue;
#else
typedef struct ClusPropertyValue ClusPropertyValue;
#endif /* __cplusplus */

#endif 	/* __ClusPropertyValue_FWD_DEFINED__ */


#ifndef __ClusPropertyValues_FWD_DEFINED__
#define __ClusPropertyValues_FWD_DEFINED__

#ifdef __cplusplus
typedef class ClusPropertyValues ClusPropertyValues;
#else
typedef struct ClusPropertyValues ClusPropertyValues;
#endif /* __cplusplus */

#endif 	/* __ClusPropertyValues_FWD_DEFINED__ */


#ifndef __ClusPropertyValueData_FWD_DEFINED__
#define __ClusPropertyValueData_FWD_DEFINED__

#ifdef __cplusplus
typedef class ClusPropertyValueData ClusPropertyValueData;
#else
typedef struct ClusPropertyValueData ClusPropertyValueData;
#endif /* __cplusplus */

#endif 	/* __ClusPropertyValueData_FWD_DEFINED__ */


#ifndef __ClusPartition_FWD_DEFINED__
#define __ClusPartition_FWD_DEFINED__

#ifdef __cplusplus
typedef class ClusPartition ClusPartition;
#else
typedef struct ClusPartition ClusPartition;
#endif /* __cplusplus */

#endif 	/* __ClusPartition_FWD_DEFINED__ */


#ifndef __ClusPartitionEx_FWD_DEFINED__
#define __ClusPartitionEx_FWD_DEFINED__

#ifdef __cplusplus
typedef class ClusPartitionEx ClusPartitionEx;
#else
typedef struct ClusPartitionEx ClusPartitionEx;
#endif /* __cplusplus */

#endif 	/* __ClusPartitionEx_FWD_DEFINED__ */


#ifndef __ClusPartitions_FWD_DEFINED__
#define __ClusPartitions_FWD_DEFINED__

#ifdef __cplusplus
typedef class ClusPartitions ClusPartitions;
#else
typedef struct ClusPartitions ClusPartitions;
#endif /* __cplusplus */

#endif 	/* __ClusPartitions_FWD_DEFINED__ */


#ifndef __ClusDisk_FWD_DEFINED__
#define __ClusDisk_FWD_DEFINED__

#ifdef __cplusplus
typedef class ClusDisk ClusDisk;
#else
typedef struct ClusDisk ClusDisk;
#endif /* __cplusplus */

#endif 	/* __ClusDisk_FWD_DEFINED__ */


#ifndef __ClusDisks_FWD_DEFINED__
#define __ClusDisks_FWD_DEFINED__

#ifdef __cplusplus
typedef class ClusDisks ClusDisks;
#else
typedef struct ClusDisks ClusDisks;
#endif /* __cplusplus */

#endif 	/* __ClusDisks_FWD_DEFINED__ */


#ifndef __ClusScsiAddress_FWD_DEFINED__
#define __ClusScsiAddress_FWD_DEFINED__

#ifdef __cplusplus
typedef class ClusScsiAddress ClusScsiAddress;
#else
typedef struct ClusScsiAddress ClusScsiAddress;
#endif /* __cplusplus */

#endif 	/* __ClusScsiAddress_FWD_DEFINED__ */


#ifndef __ClusRegistryKeys_FWD_DEFINED__
#define __ClusRegistryKeys_FWD_DEFINED__

#ifdef __cplusplus
typedef class ClusRegistryKeys ClusRegistryKeys;
#else
typedef struct ClusRegistryKeys ClusRegistryKeys;
#endif /* __cplusplus */

#endif 	/* __ClusRegistryKeys_FWD_DEFINED__ */


#ifndef __ClusCryptoKeys_FWD_DEFINED__
#define __ClusCryptoKeys_FWD_DEFINED__

#ifdef __cplusplus
typedef class ClusCryptoKeys ClusCryptoKeys;
#else
typedef struct ClusCryptoKeys ClusCryptoKeys;
#endif /* __cplusplus */

#endif 	/* __ClusCryptoKeys_FWD_DEFINED__ */


#ifndef __ClusResDependents_FWD_DEFINED__
#define __ClusResDependents_FWD_DEFINED__

#ifdef __cplusplus
typedef class ClusResDependents ClusResDependents;
#else
typedef struct ClusResDependents ClusResDependents;
#endif /* __cplusplus */

#endif 	/* __ClusResDependents_FWD_DEFINED__ */


#ifndef __ISClusApplication_FWD_DEFINED__
#define __ISClusApplication_FWD_DEFINED__
typedef interface ISClusApplication ISClusApplication;

#endif 	/* __ISClusApplication_FWD_DEFINED__ */


#ifndef __ISDomainNames_FWD_DEFINED__
#define __ISDomainNames_FWD_DEFINED__
typedef interface ISDomainNames ISDomainNames;

#endif 	/* __ISDomainNames_FWD_DEFINED__ */


#ifndef __ISClusterNames_FWD_DEFINED__
#define __ISClusterNames_FWD_DEFINED__
typedef interface ISClusterNames ISClusterNames;

#endif 	/* __ISClusterNames_FWD_DEFINED__ */


#ifndef __ISClusRefObject_FWD_DEFINED__
#define __ISClusRefObject_FWD_DEFINED__
typedef interface ISClusRefObject ISClusRefObject;

#endif 	/* __ISClusRefObject_FWD_DEFINED__ */


#ifndef __ISClusVersion_FWD_DEFINED__
#define __ISClusVersion_FWD_DEFINED__
typedef interface ISClusVersion ISClusVersion;

#endif 	/* __ISClusVersion_FWD_DEFINED__ */


#ifndef __ISCluster_FWD_DEFINED__
#define __ISCluster_FWD_DEFINED__
typedef interface ISCluster ISCluster;

#endif 	/* __ISCluster_FWD_DEFINED__ */


#ifndef __ISClusNode_FWD_DEFINED__
#define __ISClusNode_FWD_DEFINED__
typedef interface ISClusNode ISClusNode;

#endif 	/* __ISClusNode_FWD_DEFINED__ */


#ifndef __ISClusNodes_FWD_DEFINED__
#define __ISClusNodes_FWD_DEFINED__
typedef interface ISClusNodes ISClusNodes;

#endif 	/* __ISClusNodes_FWD_DEFINED__ */


#ifndef __ISClusNetwork_FWD_DEFINED__
#define __ISClusNetwork_FWD_DEFINED__
typedef interface ISClusNetwork ISClusNetwork;

#endif 	/* __ISClusNetwork_FWD_DEFINED__ */


#ifndef __ISClusNetworks_FWD_DEFINED__
#define __ISClusNetworks_FWD_DEFINED__
typedef interface ISClusNetworks ISClusNetworks;

#endif 	/* __ISClusNetworks_FWD_DEFINED__ */


#ifndef __ISClusNetInterface_FWD_DEFINED__
#define __ISClusNetInterface_FWD_DEFINED__
typedef interface ISClusNetInterface ISClusNetInterface;

#endif 	/* __ISClusNetInterface_FWD_DEFINED__ */


#ifndef __ISClusNetInterfaces_FWD_DEFINED__
#define __ISClusNetInterfaces_FWD_DEFINED__
typedef interface ISClusNetInterfaces ISClusNetInterfaces;

#endif 	/* __ISClusNetInterfaces_FWD_DEFINED__ */


#ifndef __ISClusNodeNetInterfaces_FWD_DEFINED__
#define __ISClusNodeNetInterfaces_FWD_DEFINED__
typedef interface ISClusNodeNetInterfaces ISClusNodeNetInterfaces;

#endif 	/* __ISClusNodeNetInterfaces_FWD_DEFINED__ */


#ifndef __ISClusNetworkNetInterfaces_FWD_DEFINED__
#define __ISClusNetworkNetInterfaces_FWD_DEFINED__
typedef interface ISClusNetworkNetInterfaces ISClusNetworkNetInterfaces;

#endif 	/* __ISClusNetworkNetInterfaces_FWD_DEFINED__ */


#ifndef __ISClusResGroup_FWD_DEFINED__
#define __ISClusResGroup_FWD_DEFINED__
typedef interface ISClusResGroup ISClusResGroup;

#endif 	/* __ISClusResGroup_FWD_DEFINED__ */


#ifndef __ISClusResGroups_FWD_DEFINED__
#define __ISClusResGroups_FWD_DEFINED__
typedef interface ISClusResGroups ISClusResGroups;

#endif 	/* __ISClusResGroups_FWD_DEFINED__ */


#ifndef __ISClusResource_FWD_DEFINED__
#define __ISClusResource_FWD_DEFINED__
typedef interface ISClusResource ISClusResource;

#endif 	/* __ISClusResource_FWD_DEFINED__ */


#ifndef __ISClusResDependencies_FWD_DEFINED__
#define __ISClusResDependencies_FWD_DEFINED__
typedef interface ISClusResDependencies ISClusResDependencies;

#endif 	/* __ISClusResDependencies_FWD_DEFINED__ */


#ifndef __ISClusResGroupResources_FWD_DEFINED__
#define __ISClusResGroupResources_FWD_DEFINED__
typedef interface ISClusResGroupResources ISClusResGroupResources;

#endif 	/* __ISClusResGroupResources_FWD_DEFINED__ */


#ifndef __ISClusResTypeResources_FWD_DEFINED__
#define __ISClusResTypeResources_FWD_DEFINED__
typedef interface ISClusResTypeResources ISClusResTypeResources;

#endif 	/* __ISClusResTypeResources_FWD_DEFINED__ */


#ifndef __ISClusResources_FWD_DEFINED__
#define __ISClusResources_FWD_DEFINED__
typedef interface ISClusResources ISClusResources;

#endif 	/* __ISClusResources_FWD_DEFINED__ */


#ifndef __ISClusResGroupPreferredOwnerNodes_FWD_DEFINED__
#define __ISClusResGroupPreferredOwnerNodes_FWD_DEFINED__
typedef interface ISClusResGroupPreferredOwnerNodes ISClusResGroupPreferredOwnerNodes;

#endif 	/* __ISClusResGroupPreferredOwnerNodes_FWD_DEFINED__ */


#ifndef __ISClusResPossibleOwnerNodes_FWD_DEFINED__
#define __ISClusResPossibleOwnerNodes_FWD_DEFINED__
typedef interface ISClusResPossibleOwnerNodes ISClusResPossibleOwnerNodes;

#endif 	/* __ISClusResPossibleOwnerNodes_FWD_DEFINED__ */


#ifndef __ISClusResTypePossibleOwnerNodes_FWD_DEFINED__
#define __ISClusResTypePossibleOwnerNodes_FWD_DEFINED__
typedef interface ISClusResTypePossibleOwnerNodes ISClusResTypePossibleOwnerNodes;

#endif 	/* __ISClusResTypePossibleOwnerNodes_FWD_DEFINED__ */


#ifndef __ISClusResType_FWD_DEFINED__
#define __ISClusResType_FWD_DEFINED__
typedef interface ISClusResType ISClusResType;

#endif 	/* __ISClusResType_FWD_DEFINED__ */


#ifndef __ISClusResTypes_FWD_DEFINED__
#define __ISClusResTypes_FWD_DEFINED__
typedef interface ISClusResTypes ISClusResTypes;

#endif 	/* __ISClusResTypes_FWD_DEFINED__ */


#ifndef __ISClusProperty_FWD_DEFINED__
#define __ISClusProperty_FWD_DEFINED__
typedef interface ISClusProperty ISClusProperty;

#endif 	/* __ISClusProperty_FWD_DEFINED__ */


#ifndef __ISClusPropertyValue_FWD_DEFINED__
#define __ISClusPropertyValue_FWD_DEFINED__
typedef interface ISClusPropertyValue ISClusPropertyValue;

#endif 	/* __ISClusPropertyValue_FWD_DEFINED__ */


#ifndef __ISClusPropertyValues_FWD_DEFINED__
#define __ISClusPropertyValues_FWD_DEFINED__
typedef interface ISClusPropertyValues ISClusPropertyValues;

#endif 	/* __ISClusPropertyValues_FWD_DEFINED__ */


#ifndef __ISClusProperties_FWD_DEFINED__
#define __ISClusProperties_FWD_DEFINED__
typedef interface ISClusProperties ISClusProperties;

#endif 	/* __ISClusProperties_FWD_DEFINED__ */


#ifndef __ISClusPropertyValueData_FWD_DEFINED__
#define __ISClusPropertyValueData_FWD_DEFINED__
typedef interface ISClusPropertyValueData ISClusPropertyValueData;

#endif 	/* __ISClusPropertyValueData_FWD_DEFINED__ */


#ifndef __ISClusPartition_FWD_DEFINED__
#define __ISClusPartition_FWD_DEFINED__
typedef interface ISClusPartition ISClusPartition;

#endif 	/* __ISClusPartition_FWD_DEFINED__ */


#ifndef __ISClusPartitionEx_FWD_DEFINED__
#define __ISClusPartitionEx_FWD_DEFINED__
typedef interface ISClusPartitionEx ISClusPartitionEx;

#endif 	/* __ISClusPartitionEx_FWD_DEFINED__ */


#ifndef __ISClusPartitions_FWD_DEFINED__
#define __ISClusPartitions_FWD_DEFINED__
typedef interface ISClusPartitions ISClusPartitions;

#endif 	/* __ISClusPartitions_FWD_DEFINED__ */


#ifndef __ISClusDisk_FWD_DEFINED__
#define __ISClusDisk_FWD_DEFINED__
typedef interface ISClusDisk ISClusDisk;

#endif 	/* __ISClusDisk_FWD_DEFINED__ */


#ifndef __ISClusDisks_FWD_DEFINED__
#define __ISClusDisks_FWD_DEFINED__
typedef interface ISClusDisks ISClusDisks;

#endif 	/* __ISClusDisks_FWD_DEFINED__ */


#ifndef __ISClusScsiAddress_FWD_DEFINED__
#define __ISClusScsiAddress_FWD_DEFINED__
typedef interface ISClusScsiAddress ISClusScsiAddress;

#endif 	/* __ISClusScsiAddress_FWD_DEFINED__ */


#ifndef __ISClusRegistryKeys_FWD_DEFINED__
#define __ISClusRegistryKeys_FWD_DEFINED__
typedef interface ISClusRegistryKeys ISClusRegistryKeys;

#endif 	/* __ISClusRegistryKeys_FWD_DEFINED__ */


#ifndef __ISClusCryptoKeys_FWD_DEFINED__
#define __ISClusCryptoKeys_FWD_DEFINED__
typedef interface ISClusCryptoKeys ISClusCryptoKeys;

#endif 	/* __ISClusCryptoKeys_FWD_DEFINED__ */


#ifndef __ISClusResDependents_FWD_DEFINED__
#define __ISClusResDependents_FWD_DEFINED__
typedef interface ISClusResDependents ISClusResDependents;

#endif 	/* __ISClusResDependents_FWD_DEFINED__ */


/* header files for imported files */
#include "basetsd.h"
#include "oaidl.h"

#ifdef __cplusplus
extern "C"{
#endif 


/* interface __MIDL_itf_msclus_0000_0000 */
/* [local] */ 

#include <winapifamily.h>
#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)
#ifndef _CLUSTER_API_TYPES_
#define _CLUSTER_API_TYPES_
#pragma once
#pragma warning(push)
#pragma warning(disable:4668) 
#pragma once
#pragma region Input Buffer SAL 1 compatibility macros
#pragma endregion Input Buffer SAL 1 compatibility macros
#pragma once
#pragma once
#pragma warning(pop)
#pragma once
#pragma warning(push)
#pragma warning(disable:4001) 
#pragma once
#pragma warning(push)
#pragma warning(disable:4001) 
#pragma once
#pragma warning(pop)
#pragma warning(pop)
#pragma region Desktop Family or FailoverCluster Package
typedef struct _HCLUSTER *HCLUSTER;

typedef struct _HNODE *HNODE;

typedef struct _HRESOURCE *HRESOURCE;

typedef struct _HGROUP *HGROUP;

typedef struct _HNETWORK *HNETWORK;

typedef struct _HNETINTERFACE *HNETINTERFACE;

typedef struct _HCHANGE *HCHANGE;

typedef struct _HCLUSENUM *HCLUSENUM;

typedef struct _HGROUPENUM *HGROUPENUM;

typedef struct _HRESENUM *HRESENUM;

typedef struct _HNETWORKENUM *HNETWORKENUM;

typedef struct _HNODEENUM *HNODEENUM;

typedef struct _HNETINTERFACEENUM *HNETINTERFACEENUM;

typedef struct _HRESTYPEENUM *HRESTYPEENUM;

typedef struct _HREGBATCH *HREGBATCH;

typedef struct _HREGBATCHPORT *HREGBATCHPORT;

typedef struct _HREGBATCHNOTIFICATION *HREGBATCHNOTIFICATION;

typedef struct _HREGREADBATCH *HREGREADBATCH;

typedef struct _HREGREADBATCHREPLY *HREGREADBATCHREPLY;

typedef struct _HKEYVALUEBATCH *HKEYVALUEBATCH;

typedef struct _HKEYVALUEBATCHNOTIFICATION *HKEYVALUEBATCHNOTIFICATION;

typedef struct _HKEYVALUEREADBATCH *HKEYVALUEREADBATCH;

typedef struct _HKEYVALUEREADBATCHREPLY *HKEYVALUEREADBATCHREPLY;

typedef struct _HKEYVALUESTORE *HKEYVALUESTORE;

typedef struct _HNODEENUMEX *HNODEENUMEX;

typedef struct _HCLUSENUMEX *HCLUSENUMEX;

typedef struct _HGROUPENUMEX *HGROUPENUMEX;

typedef struct _HRESENUMEX *HRESENUMEX;

typedef struct _HGROUPSET *HGROUPSET;

typedef struct _HGROUPSETENUM *HGROUPSETENUM;

typedef 
enum CLUSTER_QUORUM_TYPE
    {
        OperationalQuorum	= 0,
        ModifyQuorum	= ( OperationalQuorum + 1 ) 
    } 	CLUSTER_QUORUM_TYPE;

typedef 
enum NODE_CLUSTER_STATE
    {
        ClusterStateNotInstalled	= 0,
        ClusterStateNotConfigured	= 0x1,
        ClusterStateNotRunning	= ( 0x1 | 0x2 ) ,
        ClusterStateRunning	= ( ( 0x1 | 0x2 )  | 0x10 ) 
    } 	NODE_CLUSTER_STATE;

typedef 
enum CLUSTER_RESOURCE_STATE_CHANGE_REASON
    {
        eResourceStateChangeReasonUnknown	= 0,
        eResourceStateChangeReasonMove	= ( eResourceStateChangeReasonUnknown + 1 ) ,
        eResourceStateChangeReasonFailover	= ( eResourceStateChangeReasonMove + 1 ) ,
        eResourceStateChangeReasonFailedMove	= ( eResourceStateChangeReasonFailover + 1 ) ,
        eResourceStateChangeReasonShutdown	= ( eResourceStateChangeReasonFailedMove + 1 ) ,
        eResourceStateChangeReasonRundown	= ( eResourceStateChangeReasonShutdown + 1 ) 
    } 	CLUSTER_RESOURCE_STATE_CHANGE_REASON;

typedef 
enum _CLUSTER_REG_COMMAND
    {
        CLUSREG_COMMAND_NONE	= 0,
        CLUSREG_SET_VALUE	= 1,
        CLUSREG_CREATE_KEY	= ( CLUSREG_SET_VALUE + 1 ) ,
        CLUSREG_DELETE_KEY	= ( CLUSREG_CREATE_KEY + 1 ) ,
        CLUSREG_DELETE_VALUE	= ( CLUSREG_DELETE_KEY + 1 ) ,
        CLUSREG_SET_KEY_SECURITY	= ( CLUSREG_DELETE_VALUE + 1 ) ,
        CLUSREG_VALUE_DELETED	= ( CLUSREG_SET_KEY_SECURITY + 1 ) ,
        CLUSREG_READ_KEY	= ( CLUSREG_VALUE_DELETED + 1 ) ,
        CLUSREG_READ_VALUE	= ( CLUSREG_READ_KEY + 1 ) ,
        CLUSREG_READ_ERROR	= ( CLUSREG_READ_VALUE + 1 ) ,
        CLUSREG_CONTROL_COMMAND	= ( CLUSREG_READ_ERROR + 1 ) ,
        CLUSREG_CONDITION_EXISTS	= ( CLUSREG_CONTROL_COMMAND + 1 ) ,
        CLUSREG_CONDITION_NOT_EXISTS	= ( CLUSREG_CONDITION_EXISTS + 1 ) ,
        CLUSREG_CONDITION_IS_EQUAL	= ( CLUSREG_CONDITION_NOT_EXISTS + 1 ) ,
        CLUSREG_CONDITION_IS_NOT_EQUAL	= ( CLUSREG_CONDITION_IS_EQUAL + 1 ) ,
        CLUSREG_CONDITION_IS_GREATER_THAN	= ( CLUSREG_CONDITION_IS_NOT_EQUAL + 1 ) ,
        CLUSREG_CONDITION_IS_LESS_THAN	= ( CLUSREG_CONDITION_IS_GREATER_THAN + 1 ) ,
        CLUSREG_CONDITION_KEY_EXISTS	= ( CLUSREG_CONDITION_IS_LESS_THAN + 1 ) ,
        CLUSREG_CONDITION_KEY_NOT_EXISTS	= ( CLUSREG_CONDITION_KEY_EXISTS + 1 ) ,
        CLUSREG_LAST_COMMAND	= ( CLUSREG_CONDITION_KEY_NOT_EXISTS + 1 ) 
    } 	CLUSTER_REG_COMMAND;

typedef struct _CLUSTER_ENUM_ITEM
    {
    DWORD dwVersion;
    DWORD dwType;
    DWORD cbId;
    LPWSTR lpszId;
    DWORD cbName;
    LPWSTR lpszName;
    } 	CLUSTER_ENUM_ITEM;

typedef struct _CLUSTER_ENUM_ITEM *PCLUSTER_ENUM_ITEM;

typedef /* [public][public][public] */ 
enum __MIDL___MIDL_itf_msclus_0000_0000_0001
    {
        ClusGroupTypeCoreCluster	= 1,
        ClusGroupTypeAvailableStorage	= 2,
        ClusGroupTypeTemporary	= 3,
        ClusGroupTypeSharedVolume	= 4,
        ClusGroupTypeStoragePool	= 5,
        ClusGroupTypeFileServer	= 100,
        ClusGroupTypePrintServer	= 101,
        ClusGroupTypeDhcpServer	= 102,
        ClusGroupTypeDtc	= 103,
        ClusGroupTypeMsmq	= 104,
        ClusGroupTypeWins	= 105,
        ClusGroupTypeStandAloneDfs	= 106,
        ClusGroupTypeGenericApplication	= 107,
        ClusGroupTypeGenericService	= 108,
        ClusGroupTypeGenericScript	= 109,
        ClusGroupTypeIScsiNameService	= 110,
        ClusGroupTypeVirtualMachine	= 111,
        ClusGroupTypeTsSessionBroker	= 112,
        ClusGroupTypeIScsiTarget	= 113,
        ClusGroupTypeScaleoutFileServer	= 114,
        ClusGroupTypeVMReplicaBroker	= 115,
        ClusGroupTypeTaskScheduler	= 116,
        ClusGroupTypeClusterUpdateAgent	= 117,
        ClusGroupTypeScaleoutCluster	= 118,
        ClusGroupTypeStorageReplica	= 119,
        ClusGroupTypeVMReplicaCoordinator	= 120,
        ClusGroupTypeCrossClusterOrchestrator	= 121,
        ClusGroupTypeInfrastructureFileServer	= 122,
        ClusGroupTypeCoreSddc	= 123,
        ClusGroupTypeUserManager	= 124,
        ClusGroupTypeKeyValueStoreManager	= 125,
        ClusGroupTypeHcsVirtualMachine	= 126,
        ClusGroupTypeMetaVirtualMachine	= 127,
        ClusGroupTypeUnknown	= 9999
    } 	CLUSGROUP_TYPE;

typedef enum __MIDL___MIDL_itf_msclus_0000_0000_0001 *PCLUSGROUP_TYPE;

typedef struct _CLUSTER_CREATE_GROUP_INFO
    {
    DWORD dwVersion;
    CLUSGROUP_TYPE groupType;
    } 	CLUSTER_CREATE_GROUP_INFO;

typedef struct _CLUSTER_CREATE_GROUP_INFO *PCLUSTER_CREATE_GROUP_INFO;

typedef /* [public] */ 
enum __MIDL___MIDL_itf_msclus_0000_0000_0002
    {
        CLUSTER_MGMT_POINT_TYPE_NONE	= 0,
        CLUSTER_MGMT_POINT_TYPE_CNO	= ( CLUSTER_MGMT_POINT_TYPE_NONE + 1 ) ,
        CLUSTER_MGMT_POINT_TYPE_DNS_ONLY	= ( CLUSTER_MGMT_POINT_TYPE_CNO + 1 ) ,
        CLUSTER_MGMT_POINT_TYPE_CNO_ONLY	= ( CLUSTER_MGMT_POINT_TYPE_DNS_ONLY + 1 ) 
    } 	CLUSTER_MGMT_POINT_TYPE;

typedef /* [public] */ 
enum __MIDL___MIDL_itf_msclus_0000_0000_0003
    {
        CLUSTER_MGMT_POINT_RESTYPE_AUTO	= 0,
        CLUSTER_MGMT_POINT_RESTYPE_SNN	= 1,
        CLUSTER_MGMT_POINT_RESTYPE_DNN	= 2
    } 	CLUSTER_MGMT_POINT_RESTYPE;

typedef enum __MIDL___MIDL_itf_msclus_0000_0000_0003 *PCLUSTER_MGMT_POINT_RESTYPE;

typedef /* [public] */ 
enum __MIDL___MIDL_itf_msclus_0000_0000_0004
    {
        CLUSTER_CLOUD_TYPE_NONE	= 0,
        CLUSTER_CLOUD_TYPE_AZURE	= 1,
        CLUSTER_CLOUD_TYPE_MIXED	= 128,
        CLUSTER_CLOUD_TYPE_UNKNOWN	= -1
    } 	CLUSTER_CLOUD_TYPE;

typedef enum __MIDL___MIDL_itf_msclus_0000_0000_0004 *PCLUSTER_CLOUD_TYPE;

typedef /* [public] */ 
enum __MIDL___MIDL_itf_msclus_0000_0000_0005
    {
        CLUS_GROUP_START_ALWAYS	= 0,
        CLUS_GROUP_DO_NOT_START	= 1,
        CLUS_GROUP_START_ALLOWED	= 2
    } 	CLUS_GROUP_START_SETTING;

typedef /* [public] */ 
enum __MIDL___MIDL_itf_msclus_0000_0000_0006
    {
        CLUS_AFFINITY_RULE_NONE	= 0,
        CLUS_AFFINITY_RULE_SAME_FAULT_DOMAIN	= 1,
        CLUS_AFFINITY_RULE_SAME_NODE	= 2,
        CLUS_AFFINITY_RULE_DIFFERENT_FAULT_DOMAIN	= 3,
        CLUS_AFFINITY_RULE_DIFFERENT_NODE	= 4,
        CLUS_AFFINITY_RULE_MIN	= CLUS_AFFINITY_RULE_NONE,
        CLUS_AFFINITY_RULE_MAX	= CLUS_AFFINITY_RULE_DIFFERENT_NODE
    } 	CLUS_AFFINITY_RULE_TYPE;

typedef /* [public] */ 
enum __MIDL___MIDL_itf_msclus_0000_0000_0007
    {
        CLUS_ADAPTER_EXCLUSION_TYPE_IPPREFIX	= 0,
        CLUS_ADAPTER_EXCLUSION_TYPE_DESCRIPTION	= 1,
        CLUS_ADAPTER_EXCLUSION_TYPE_FRIENDLYNAME	= 2
    } 	CLUS_ADAPTER_EXCLUSION_TYPE;

typedef 
enum CLUSTER_QUORUM_VALUE
    {
        CLUSTER_QUORUM_MAINTAINED	= 0,
        CLUSTER_QUORUM_LOST	= 1
    } 	CLUSTER_QUORUM_VALUE;

typedef 
enum CLUSTER_CHANGE
    {
        CLUSTER_CHANGE_NODE_STATE	= 0x1,
        CLUSTER_CHANGE_NODE_DELETED	= 0x2,
        CLUSTER_CHANGE_NODE_ADDED	= 0x4,
        CLUSTER_CHANGE_NODE_PROPERTY	= 0x8,
        CLUSTER_CHANGE_REGISTRY_NAME	= 0x10,
        CLUSTER_CHANGE_REGISTRY_ATTRIBUTES	= 0x20,
        CLUSTER_CHANGE_REGISTRY_VALUE	= 0x40,
        CLUSTER_CHANGE_REGISTRY_SUBTREE	= 0x80,
        CLUSTER_CHANGE_RESOURCE_STATE	= 0x100,
        CLUSTER_CHANGE_RESOURCE_DELETED	= 0x200,
        CLUSTER_CHANGE_RESOURCE_ADDED	= 0x400,
        CLUSTER_CHANGE_RESOURCE_PROPERTY	= 0x800,
        CLUSTER_CHANGE_GROUP_STATE	= 0x1000,
        CLUSTER_CHANGE_GROUP_DELETED	= 0x2000,
        CLUSTER_CHANGE_GROUP_ADDED	= 0x4000,
        CLUSTER_CHANGE_GROUP_PROPERTY	= 0x8000,
        CLUSTER_CHANGE_RESOURCE_TYPE_DELETED	= 0x10000,
        CLUSTER_CHANGE_RESOURCE_TYPE_ADDED	= 0x20000,
        CLUSTER_CHANGE_RESOURCE_TYPE_PROPERTY	= 0x40000,
        CLUSTER_CHANGE_CLUSTER_RECONNECT	= 0x80000,
        CLUSTER_CHANGE_NETWORK_STATE	= 0x100000,
        CLUSTER_CHANGE_NETWORK_DELETED	= 0x200000,
        CLUSTER_CHANGE_NETWORK_ADDED	= 0x400000,
        CLUSTER_CHANGE_NETWORK_PROPERTY	= 0x800000,
        CLUSTER_CHANGE_NETINTERFACE_STATE	= 0x1000000,
        CLUSTER_CHANGE_NETINTERFACE_DELETED	= 0x2000000,
        CLUSTER_CHANGE_NETINTERFACE_ADDED	= 0x4000000,
        CLUSTER_CHANGE_NETINTERFACE_PROPERTY	= 0x8000000,
        CLUSTER_CHANGE_QUORUM_STATE	= 0x10000000,
        CLUSTER_CHANGE_CLUSTER_STATE	= 0x20000000,
        CLUSTER_CHANGE_CLUSTER_PROPERTY	= 0x40000000,
        CLUSTER_CHANGE_HANDLE_CLOSE	= 0x80000000,
        CLUSTER_CHANGE_ALL	= ( ( ( ( ( ( ( ( ( ( ( ( ( ( ( ( ( ( ( ( ( ( ( ( ( ( ( ( ( ( ( CLUSTER_CHANGE_NODE_STATE | CLUSTER_CHANGE_NODE_DELETED )  | CLUSTER_CHANGE_NODE_ADDED )  | CLUSTER_CHANGE_NODE_PROPERTY )  | CLUSTER_CHANGE_REGISTRY_NAME )  | CLUSTER_CHANGE_REGISTRY_ATTRIBUTES )  | CLUSTER_CHANGE_REGISTRY_VALUE )  | CLUSTER_CHANGE_REGISTRY_SUBTREE )  | CLUSTER_CHANGE_RESOURCE_STATE )  | CLUSTER_CHANGE_RESOURCE_DELETED )  | CLUSTER_CHANGE_RESOURCE_ADDED )  | CLUSTER_CHANGE_RESOURCE_PROPERTY )  | CLUSTER_CHANGE_GROUP_STATE )  | CLUSTER_CHANGE_GROUP_DELETED )  | CLUSTER_CHANGE_GROUP_ADDED )  | CLUSTER_CHANGE_GROUP_PROPERTY )  | CLUSTER_CHANGE_RESOURCE_TYPE_DELETED )  | CLUSTER_CHANGE_RESOURCE_TYPE_ADDED )  | CLUSTER_CHANGE_RESOURCE_TYPE_PROPERTY )  | CLUSTER_CHANGE_NETWORK_STATE )  | CLUSTER_CHANGE_NETWORK_DELETED )  | CLUSTER_CHANGE_NETWORK_ADDED )  | CLUSTER_CHANGE_NETWORK_PROPERTY )  | CLUSTER_CHANGE_NETINTERFACE_STATE )  | CLUSTER_CHANGE_NETINTERFACE_DELETED )  | CLUSTER_CHANGE_NETINTERFACE_ADDED )  | CLUSTER_CHANGE_NETINTERFACE_PROPERTY )  | CLUSTER_CHANGE_QUORUM_STATE )  | CLUSTER_CHANGE_CLUSTER_STATE )  | CLUSTER_CHANGE_CLUSTER_PROPERTY )  | CLUSTER_CHANGE_CLUSTER_RECONNECT )  | CLUSTER_CHANGE_HANDLE_CLOSE ) 
    } 	CLUSTER_CHANGE;

typedef 
enum CLUSTER_NOTIFICATIONS_VERSION
    {
        CLUSTER_NOTIFICATIONS_V1	= 0x1,
        CLUSTER_NOTIFICATIONS_V2	= 0x2
    } 	CLUSTER_NOTIFICATIONS_VERSION;

typedef 
enum CLUSTER_CHANGE_CLUSTER_V2
    {
        CLUSTER_CHANGE_CLUSTER_RECONNECT_V2	= 0x1,
        CLUSTER_CHANGE_CLUSTER_STATE_V2	= 0x2,
        CLUSTER_CHANGE_CLUSTER_GROUP_ADDED_V2	= 0x4,
        CLUSTER_CHANGE_CLUSTER_HANDLE_CLOSE_V2	= 0x8,
        CLUSTER_CHANGE_CLUSTER_NETWORK_ADDED_V2	= 0x10,
        CLUSTER_CHANGE_CLUSTER_NODE_ADDED_V2	= 0x20,
        CLUSTER_CHANGE_CLUSTER_RESOURCE_TYPE_ADDED_V2	= 0x40,
        CLUSTER_CHANGE_CLUSTER_COMMON_PROPERTY_V2	= 0x80,
        CLUSTER_CHANGE_CLUSTER_PRIVATE_PROPERTY_V2	= 0x100,
        CLUSTER_CHANGE_CLUSTER_LOST_NOTIFICATIONS_V2	= 0x200,
        CLUSTER_CHANGE_CLUSTER_RENAME_V2	= 0x400,
        CLUSTER_CHANGE_CLUSTER_MEMBERSHIP_V2	= 0x800,
        CLUSTER_CHANGE_CLUSTER_UPGRADED_V2	= 0x1000,
        CLUSTER_CHANGE_CLUSTER_ALL_V2	= ( ( ( ( ( ( ( ( ( ( ( ( CLUSTER_CHANGE_CLUSTER_RECONNECT_V2 | CLUSTER_CHANGE_CLUSTER_STATE_V2 )  | CLUSTER_CHANGE_CLUSTER_GROUP_ADDED_V2 )  | CLUSTER_CHANGE_CLUSTER_HANDLE_CLOSE_V2 )  | CLUSTER_CHANGE_CLUSTER_NETWORK_ADDED_V2 )  | CLUSTER_CHANGE_CLUSTER_NODE_ADDED_V2 )  | CLUSTER_CHANGE_CLUSTER_RESOURCE_TYPE_ADDED_V2 )  | CLUSTER_CHANGE_CLUSTER_COMMON_PROPERTY_V2 )  | CLUSTER_CHANGE_CLUSTER_PRIVATE_PROPERTY_V2 )  | CLUSTER_CHANGE_CLUSTER_LOST_NOTIFICATIONS_V2 )  | CLUSTER_CHANGE_CLUSTER_RENAME_V2 )  | CLUSTER_CHANGE_CLUSTER_MEMBERSHIP_V2 )  | CLUSTER_CHANGE_CLUSTER_UPGRADED_V2 ) 
    } 	CLUSTER_CHANGE_CLUSTER_V2;

typedef 
enum CLUSTER_CHANGE_GROUP_V2
    {
        CLUSTER_CHANGE_GROUP_DELETED_V2	= 0x1,
        CLUSTER_CHANGE_GROUP_COMMON_PROPERTY_V2	= 0x2,
        CLUSTER_CHANGE_GROUP_PRIVATE_PROPERTY_V2	= 0x4,
        CLUSTER_CHANGE_GROUP_STATE_V2	= 0x8,
        CLUSTER_CHANGE_GROUP_OWNER_NODE_V2	= 0x10,
        CLUSTER_CHANGE_GROUP_PREFERRED_OWNERS_V2	= 0x20,
        CLUSTER_CHANGE_GROUP_RESOURCE_ADDED_V2	= 0x40,
        CLUSTER_CHANGE_GROUP_RESOURCE_GAINED_V2	= 0x80,
        CLUSTER_CHANGE_GROUP_RESOURCE_LOST_V2	= 0x100,
        CLUSTER_CHANGE_GROUP_HANDLE_CLOSE_V2	= 0x200,
        CLUSTER_CHANGE_GROUP_ALL_V2	= ( ( ( ( ( ( ( ( ( CLUSTER_CHANGE_GROUP_DELETED_V2 | CLUSTER_CHANGE_GROUP_COMMON_PROPERTY_V2 )  | CLUSTER_CHANGE_GROUP_PRIVATE_PROPERTY_V2 )  | CLUSTER_CHANGE_GROUP_STATE_V2 )  | CLUSTER_CHANGE_GROUP_OWNER_NODE_V2 )  | CLUSTER_CHANGE_GROUP_PREFERRED_OWNERS_V2 )  | CLUSTER_CHANGE_GROUP_RESOURCE_ADDED_V2 )  | CLUSTER_CHANGE_GROUP_RESOURCE_GAINED_V2 )  | CLUSTER_CHANGE_GROUP_RESOURCE_LOST_V2 )  | CLUSTER_CHANGE_GROUP_HANDLE_CLOSE_V2 ) 
    } 	CLUSTER_CHANGE_GROUP_V2;

typedef 
enum CLUSTER_CHANGE_GROUPSET_V2
    {
        CLUSTER_CHANGE_GROUPSET_DELETED_v2	= 0x1,
        CLUSTER_CHANGE_GROUPSET_COMMON_PROPERTY_V2	= 0x2,
        CLUSTER_CHANGE_GROUPSET_PRIVATE_PROPERTY_V2	= 0x4,
        CLUSTER_CHANGE_GROUPSET_STATE_V2	= 0x8,
        CLUSTER_CHANGE_GROUPSET_GROUP_ADDED	= 0x10,
        CLUSTER_CHANGE_GROUPSET_GROUP_REMOVED	= 0x20,
        CLUSTER_CHANGE_GROUPSET_DEPENDENCIES_V2	= 0x40,
        CLUSTER_CHANGE_GROUPSET_DEPENDENTS_V2	= 0x80,
        CLUSTER_CHANGE_GROUPSET_HANDLE_CLOSE_v2	= 0x100,
        CLUSTER_CHANGE_GROUPSET_ALL_V2	= ( ( ( ( ( ( ( ( CLUSTER_CHANGE_GROUPSET_DELETED_v2 | CLUSTER_CHANGE_GROUPSET_COMMON_PROPERTY_V2 )  | CLUSTER_CHANGE_GROUPSET_PRIVATE_PROPERTY_V2 )  | CLUSTER_CHANGE_GROUPSET_STATE_V2 )  | CLUSTER_CHANGE_GROUPSET_GROUP_ADDED )  | CLUSTER_CHANGE_GROUPSET_GROUP_REMOVED )  | CLUSTER_CHANGE_GROUPSET_DEPENDENCIES_V2 )  | CLUSTER_CHANGE_GROUPSET_DEPENDENTS_V2 )  | CLUSTER_CHANGE_GROUPSET_HANDLE_CLOSE_v2 ) 
    } 	CLUSTER_CHANGE_GROUPSET_V2;

typedef 
enum CLUSTER_CHANGE_RESOURCE_V2
    {
        CLUSTER_CHANGE_RESOURCE_COMMON_PROPERTY_V2	= 0x1,
        CLUSTER_CHANGE_RESOURCE_PRIVATE_PROPERTY_V2	= 0x2,
        CLUSTER_CHANGE_RESOURCE_STATE_V2	= 0x4,
        CLUSTER_CHANGE_RESOURCE_OWNER_GROUP_V2	= 0x8,
        CLUSTER_CHANGE_RESOURCE_DEPENDENCIES_V2	= 0x10,
        CLUSTER_CHANGE_RESOURCE_DEPENDENTS_V2	= 0x20,
        CLUSTER_CHANGE_RESOURCE_POSSIBLE_OWNERS_V2	= 0x40,
        CLUSTER_CHANGE_RESOURCE_DELETED_V2	= 0x80,
        CLUSTER_CHANGE_RESOURCE_DLL_UPGRADED_V2	= 0x100,
        CLUSTER_CHANGE_RESOURCE_HANDLE_CLOSE_V2	= 0x200,
        CLUSTER_CHANGE_RESOURCE_TERMINAL_STATE_V2	= 0x400,
        CLUSTER_CHANGE_RESOURCE_ALL_V2	= ( ( ( ( ( ( ( ( ( ( CLUSTER_CHANGE_RESOURCE_COMMON_PROPERTY_V2 | CLUSTER_CHANGE_RESOURCE_PRIVATE_PROPERTY_V2 )  | CLUSTER_CHANGE_RESOURCE_STATE_V2 )  | CLUSTER_CHANGE_RESOURCE_OWNER_GROUP_V2 )  | CLUSTER_CHANGE_RESOURCE_DEPENDENCIES_V2 )  | CLUSTER_CHANGE_RESOURCE_DEPENDENTS_V2 )  | CLUSTER_CHANGE_RESOURCE_POSSIBLE_OWNERS_V2 )  | CLUSTER_CHANGE_RESOURCE_DELETED_V2 )  | CLUSTER_CHANGE_RESOURCE_DLL_UPGRADED_V2 )  | CLUSTER_CHANGE_RESOURCE_HANDLE_CLOSE_V2 )  | CLUSTER_CHANGE_RESOURCE_TERMINAL_STATE_V2 ) 
    } 	CLUSTER_CHANGE_RESOURCE_V2;

typedef 
enum CLUSTER_CHANGE_RESOURCE_TYPE_V2
    {
        CLUSTER_CHANGE_RESOURCE_TYPE_DELETED_V2	= 0x1,
        CLUSTER_CHANGE_RESOURCE_TYPE_COMMON_PROPERTY_V2	= 0x2,
        CLUSTER_CHANGE_RESOURCE_TYPE_PRIVATE_PROPERTY_V2	= 0x4,
        CLUSTER_CHANGE_RESOURCE_TYPE_POSSIBLE_OWNERS_V2	= 0x8,
        CLUSTER_CHANGE_RESOURCE_TYPE_DLL_UPGRADED_V2	= 0x10,
        CLUSTER_RESOURCE_TYPE_SPECIFIC_V2	= 0x20,
        CLUSTER_CHANGE_RESOURCE_TYPE_ALL_V2	= ( ( ( ( ( CLUSTER_CHANGE_RESOURCE_TYPE_DELETED_V2 | CLUSTER_CHANGE_RESOURCE_TYPE_COMMON_PROPERTY_V2 )  | CLUSTER_CHANGE_RESOURCE_TYPE_PRIVATE_PROPERTY_V2 )  | CLUSTER_CHANGE_RESOURCE_TYPE_POSSIBLE_OWNERS_V2 )  | CLUSTER_CHANGE_RESOURCE_TYPE_DLL_UPGRADED_V2 )  | CLUSTER_RESOURCE_TYPE_SPECIFIC_V2 ) 
    } 	CLUSTER_CHANGE_RESOURCE_TYPE_V2;

typedef 
enum CLUSTER_CHANGE_NETINTERFACE_V2
    {
        CLUSTER_CHANGE_NETINTERFACE_DELETED_V2	= 0x1,
        CLUSTER_CHANGE_NETINTERFACE_COMMON_PROPERTY_V2	= 0x2,
        CLUSTER_CHANGE_NETINTERFACE_PRIVATE_PROPERTY_V2	= 0x4,
        CLUSTER_CHANGE_NETINTERFACE_STATE_V2	= 0x8,
        CLUSTER_CHANGE_NETINTERFACE_HANDLE_CLOSE_V2	= 0x10,
        CLUSTER_CHANGE_NETINTERFACE_ALL_V2	= ( ( ( ( CLUSTER_CHANGE_NETINTERFACE_DELETED_V2 | CLUSTER_CHANGE_NETINTERFACE_COMMON_PROPERTY_V2 )  | CLUSTER_CHANGE_NETINTERFACE_PRIVATE_PROPERTY_V2 )  | CLUSTER_CHANGE_NETINTERFACE_STATE_V2 )  | CLUSTER_CHANGE_NETINTERFACE_HANDLE_CLOSE_V2 ) 
    } 	CLUSTER_CHANGE_NETINTERFACE_V2;

typedef 
enum CLUSTER_CHANGE_NETWORK_V2
    {
        CLUSTER_CHANGE_NETWORK_DELETED_V2	= 0x1,
        CLUSTER_CHANGE_NETWORK_COMMON_PROPERTY_V2	= 0x2,
        CLUSTER_CHANGE_NETWORK_PRIVATE_PROPERTY_V2	= 0x4,
        CLUSTER_CHANGE_NETWORK_STATE_V2	= 0x8,
        CLUSTER_CHANGE_NETWORK_HANDLE_CLOSE_V2	= 0x10,
        CLUSTER_CHANGE_NETWORK_ALL_V2	= ( ( ( ( CLUSTER_CHANGE_NETWORK_DELETED_V2 | CLUSTER_CHANGE_NETWORK_COMMON_PROPERTY_V2 )  | CLUSTER_CHANGE_NETWORK_PRIVATE_PROPERTY_V2 )  | CLUSTER_CHANGE_NETWORK_STATE_V2 )  | CLUSTER_CHANGE_NETWORK_HANDLE_CLOSE_V2 ) 
    } 	CLUSTER_CHANGE_NETWORK_V2;

typedef 
enum CLUSTER_CHANGE_NODE_V2
    {
        CLUSTER_CHANGE_NODE_NETINTERFACE_ADDED_V2	= 0x1,
        CLUSTER_CHANGE_NODE_DELETED_V2	= 0x2,
        CLUSTER_CHANGE_NODE_COMMON_PROPERTY_V2	= 0x4,
        CLUSTER_CHANGE_NODE_PRIVATE_PROPERTY_V2	= 0x8,
        CLUSTER_CHANGE_NODE_STATE_V2	= 0x10,
        CLUSTER_CHANGE_NODE_GROUP_GAINED_V2	= 0x20,
        CLUSTER_CHANGE_NODE_GROUP_LOST_V2	= 0x40,
        CLUSTER_CHANGE_NODE_HANDLE_CLOSE_V2	= 0x80,
        CLUSTER_CHANGE_NODE_ALL_V2	= ( ( ( ( ( ( ( CLUSTER_CHANGE_NODE_NETINTERFACE_ADDED_V2 | CLUSTER_CHANGE_NODE_DELETED_V2 )  | CLUSTER_CHANGE_NODE_COMMON_PROPERTY_V2 )  | CLUSTER_CHANGE_NODE_PRIVATE_PROPERTY_V2 )  | CLUSTER_CHANGE_NODE_STATE_V2 )  | CLUSTER_CHANGE_NODE_GROUP_GAINED_V2 )  | CLUSTER_CHANGE_NODE_GROUP_LOST_V2 )  | CLUSTER_CHANGE_NODE_HANDLE_CLOSE_V2 ) 
    } 	CLUSTER_CHANGE_NODE_V2;

typedef 
enum CLUSTER_CHANGE_REGISTRY_V2
    {
        CLUSTER_CHANGE_REGISTRY_ATTRIBUTES_V2	= 0x1,
        CLUSTER_CHANGE_REGISTRY_NAME_V2	= 0x2,
        CLUSTER_CHANGE_REGISTRY_SUBTREE_V2	= 0x4,
        CLUSTER_CHANGE_REGISTRY_VALUE_V2	= 0x8,
        CLUSTER_CHANGE_REGISTRY_HANDLE_CLOSE_V2	= 0x10,
        CLUSTER_CHANGE_REGISTRY_ALL_V2	= ( ( ( ( CLUSTER_CHANGE_REGISTRY_ATTRIBUTES_V2 | CLUSTER_CHANGE_REGISTRY_NAME_V2 )  | CLUSTER_CHANGE_REGISTRY_SUBTREE_V2 )  | CLUSTER_CHANGE_REGISTRY_VALUE_V2 )  | CLUSTER_CHANGE_REGISTRY_HANDLE_CLOSE_V2 ) 
    } 	CLUSTER_CHANGE_REGISTRY_V2;

typedef 
enum CLUSTER_CHANGE_QUORUM_V2
    {
        CLUSTER_CHANGE_QUORUM_STATE_V2	= 0x1,
        CLUSTER_CHANGE_QUORUM_ALL_V2	= CLUSTER_CHANGE_QUORUM_STATE_V2
    } 	CLUSTER_CHANGE_QUORUM_V2;

typedef 
enum CLUSTER_CHANGE_SHARED_VOLUME_V2
    {
        CLUSTER_CHANGE_SHARED_VOLUME_STATE_V2	= 0x1,
        CLUSTER_CHANGE_SHARED_VOLUME_ADDED_V2	= 0x2,
        CLUSTER_CHANGE_SHARED_VOLUME_REMOVED_V2	= 0x4,
        CLUSTER_CHANGE_SHARED_VOLUME_ALL_V2	= ( ( CLUSTER_CHANGE_SHARED_VOLUME_STATE_V2 | CLUSTER_CHANGE_SHARED_VOLUME_ADDED_V2 )  | CLUSTER_CHANGE_SHARED_VOLUME_REMOVED_V2 ) 
    } 	CLUSTER_CHANGE_SHARED_VOLUME_V2;

typedef 
enum CLUSTER_CHANGE_SPACEPORT_V2
    {
        CLUSTER_CHANGE_SPACEPORT_CUSTOM_PNP_V2	= 0x1
    } 	CLUSTER_CHANGE_SPACEPORT_V2;

typedef 
enum CLUSTER_CHANGE_NODE_UPGRADE_PHASE_V2
    {
        CLUSTER_CHANGE_UPGRADE_NODE_PREPARE	= 0x1,
        CLUSTER_CHANGE_UPGRADE_NODE_COMMIT	= 0x2,
        CLUSTER_CHANGE_UPGRADE_NODE_POSTCOMMIT	= 0x4,
        CLUSTER_CHANGE_UPGRADE_ALL	= ( ( CLUSTER_CHANGE_UPGRADE_NODE_PREPARE | CLUSTER_CHANGE_UPGRADE_NODE_COMMIT )  | CLUSTER_CHANGE_UPGRADE_NODE_POSTCOMMIT ) 
    } 	CLUSTER_CHANGE_NODE_UPGRADE_PHASE_V2;

typedef 
enum CLUSTER_OBJECT_TYPE
    {
        CLUSTER_OBJECT_TYPE_NONE	= 0,
        CLUSTER_OBJECT_TYPE_CLUSTER	= 0x1,
        CLUSTER_OBJECT_TYPE_GROUP	= 0x2,
        CLUSTER_OBJECT_TYPE_RESOURCE	= 0x3,
        CLUSTER_OBJECT_TYPE_RESOURCE_TYPE	= 0x4,
        CLUSTER_OBJECT_TYPE_NETWORK_INTERFACE	= 0x5,
        CLUSTER_OBJECT_TYPE_NETWORK	= 0x6,
        CLUSTER_OBJECT_TYPE_NODE	= 0x7,
        CLUSTER_OBJECT_TYPE_REGISTRY	= 0x8,
        CLUSTER_OBJECT_TYPE_QUORUM	= 0x9,
        CLUSTER_OBJECT_TYPE_SHARED_VOLUME	= 0xa,
        CLUSTER_OBJECT_TYPE_GROUPSET	= 0xd,
        CLUSTER_OBJECT_TYPE_AFFINITYRULE	= 0x10,
        CLUSTER_OBJECT_TYPE_FAULTDOMAIN	= 0x11
    } 	CLUSTER_OBJECT_TYPE;

typedef 
enum CLUSTERSET_OBJECT_TYPE
    {
        CLUSTERSET_OBJECT_TYPE_NONE	= 0,
        CLUSTERSET_OBJECT_TYPE_MEMBER	= 0x1,
        CLUSTERSET_OBJECT_TYPE_WORKLOAD	= 0x2,
        CLUSTERSET_OBJECT_TYPE_DATABASE	= 0x3
    } 	CLUSTERSET_OBJECT_TYPE;

typedef struct _NOTIFY_FILTER_AND_TYPE
    {
    DWORD dwObjectType;
    LONGLONG FilterFlags;
    } 	NOTIFY_FILTER_AND_TYPE;

typedef struct _NOTIFY_FILTER_AND_TYPE *PNOTIFY_FILTER_AND_TYPE;

typedef struct _CLUSTER_MEMBERSHIP_INFO
    {
    BOOL HasQuorum;
    DWORD UpnodesSize;
    BYTE Upnodes[ 1 ];
    } 	CLUSTER_MEMBERSHIP_INFO;

typedef struct _CLUSTER_MEMBERSHIP_INFO *PCLUSTER_MEMBERSHIP_INFO;

typedef 
enum CLUSTER_ENUM
    {
        CLUSTER_ENUM_NODE	= 0x1,
        CLUSTER_ENUM_RESTYPE	= 0x2,
        CLUSTER_ENUM_RESOURCE	= 0x4,
        CLUSTER_ENUM_GROUP	= 0x8,
        CLUSTER_ENUM_NETWORK	= 0x10,
        CLUSTER_ENUM_NETINTERFACE	= 0x20,
        CLUSTER_ENUM_CAPACITY_NODE	= 0x10000000,
        CLUSTER_ENUM_SHARED_VOLUME_GROUP	= 0x20000000,
        CLUSTER_ENUM_SHARED_VOLUME_RESOURCE	= 0x40000000,
        CLUSTER_ENUM_INTERNAL_NETWORK	= 0x80000000,
        CLUSTER_ENUM_ALL	= ( ( ( ( ( CLUSTER_ENUM_NODE | CLUSTER_ENUM_RESTYPE )  | CLUSTER_ENUM_RESOURCE )  | CLUSTER_ENUM_GROUP )  | CLUSTER_ENUM_NETWORK )  | CLUSTER_ENUM_NETINTERFACE ) 
    } 	CLUSTER_ENUM;

typedef 
enum CLUSTER_NODE_ENUM
    {
        CLUSTER_NODE_ENUM_NETINTERFACES	= 0x1,
        CLUSTER_NODE_ENUM_GROUPS	= 0x2,
        CLUSTER_NODE_ENUM_PREFERRED_GROUPS	= 0x4,
        CLUSTER_NODE_ENUM_ALL	= ( CLUSTER_NODE_ENUM_NETINTERFACES | CLUSTER_NODE_ENUM_GROUPS ) 
    } 	CLUSTER_NODE_ENUM;

typedef 
enum CLUSTER_NODE_STATE
    {
        ClusterNodeStateUnknown	= -1,
        ClusterNodeUp	= ( ClusterNodeStateUnknown + 1 ) ,
        ClusterNodeDown	= ( ClusterNodeUp + 1 ) ,
        ClusterNodePaused	= ( ClusterNodeDown + 1 ) ,
        ClusterNodeJoining	= ( ClusterNodePaused + 1 ) 
    } 	CLUSTER_NODE_STATE;

typedef 
enum CLUSTER_STORAGENODE_STATE
    {
        ClusterStorageNodeStateUnknown	= 0,
        ClusterStorageNodeUp	= ( ClusterStorageNodeStateUnknown + 1 ) ,
        ClusterStorageNodeDown	= ( ClusterStorageNodeUp + 1 ) ,
        ClusterStorageNodePaused	= ( ClusterStorageNodeDown + 1 ) ,
        ClusterStorageNodeStarting	= ( ClusterStorageNodePaused + 1 ) ,
        ClusterStorageNodeStopping	= ( ClusterStorageNodeStarting + 1 ) 
    } 	CLUSTER_STORAGENODE_STATE;

typedef 
enum CLUSTER_NODE_DRAIN_STATUS
    {
        NodeDrainStatusNotInitiated	= 0,
        NodeDrainStatusInProgress	= ( NodeDrainStatusNotInitiated + 1 ) ,
        NodeDrainStatusCompleted	= ( NodeDrainStatusInProgress + 1 ) ,
        NodeDrainStatusFailed	= ( NodeDrainStatusCompleted + 1 ) ,
        ClusterNodeDrainStatusCount	= ( NodeDrainStatusFailed + 1 ) 
    } 	CLUSTER_NODE_DRAIN_STATUS;

typedef 
enum CLUSTER_NODE_STATUS
    {
        NodeStatusNormal	= 0,
        NodeStatusIsolated	= 0x1,
        NodeStatusQuarantined	= 0x2,
        NodeStatusDrainInProgress	= 0x4,
        NodeStatusDrainCompleted	= 0x8,
        NodeStatusDrainFailed	= 0x10,
        NodeStatusAvoidPlacement	= 0x20,
        NodeStatusMax	= ( ( ( NodeStatusIsolated | NodeStatusQuarantined )  | NodeStatusDrainFailed )  | NodeStatusAvoidPlacement ) 
    } 	CLUSTER_NODE_STATUS;

typedef 
enum CLUSTER_NODE_FAILBACK_STATUS
    {
        NodeFailbackStatusNotInitiated	= 0,
        NodeFailbackStatusInProgress	= ( NodeFailbackStatusNotInitiated + 1 ) ,
        NodeFailbackStatusCompleted	= ( NodeFailbackStatusInProgress + 1 ) ,
        NodeFailbackStatusFailed	= ( NodeFailbackStatusCompleted + 1 ) ,
        ClusterNodeFailbackStatusCount	= ( NodeFailbackStatusFailed + 1 ) 
    } 	CLUSTER_NODE_FAILBACK_STATUS;

typedef 
enum CLUSTER_GROUP_ENUM
    {
        CLUSTER_GROUP_ENUM_CONTAINS	= 0x1,
        CLUSTER_GROUP_ENUM_NODES	= 0x2,
        CLUSTER_GROUP_ENUM_ALL	= ( CLUSTER_GROUP_ENUM_CONTAINS | CLUSTER_GROUP_ENUM_NODES ) 
    } 	CLUSTER_GROUP_ENUM;

typedef 
enum CLUSTER_GROUP_STATE
    {
        ClusterGroupStateUnknown	= -1,
        ClusterGroupOnline	= ( ClusterGroupStateUnknown + 1 ) ,
        ClusterGroupOffline	= ( ClusterGroupOnline + 1 ) ,
        ClusterGroupFailed	= ( ClusterGroupOffline + 1 ) ,
        ClusterGroupPartialOnline	= ( ClusterGroupFailed + 1 ) ,
        ClusterGroupPending	= ( ClusterGroupPartialOnline + 1 ) 
    } 	CLUSTER_GROUP_STATE;

typedef 
enum CLUSTER_GROUP_PRIORITY
    {
        PriorityDisabled	= 0,
        PriorityLow	= 1000,
        PriorityMedium	= 2000,
        PriorityHigh	= 3000
    } 	CLUSTER_GROUP_PRIORITY;

typedef 
enum CLUSTER_GROUP_AUTOFAILBACK_TYPE
    {
        ClusterGroupPreventFailback	= 0,
        ClusterGroupAllowFailback	= ( ClusterGroupPreventFailback + 1 ) ,
        ClusterGroupFailbackTypeCount	= ( ClusterGroupAllowFailback + 1 ) 
    } 	CLUSTER_GROUP_AUTOFAILBACK_TYPE;

typedef enum CLUSTER_GROUP_AUTOFAILBACK_TYPE CGAFT;

typedef struct _CLUSTER_GROUP_ENUM_ITEM
    {
    DWORD dwVersion;
    DWORD cbId;
    LPWSTR lpszId;
    DWORD cbName;
    LPWSTR lpszName;
    CLUSTER_GROUP_STATE state;
    DWORD cbOwnerNode;
    LPWSTR lpszOwnerNode;
    DWORD dwFlags;
    DWORD cbProperties;
    PVOID pProperties;
    DWORD cbRoProperties;
    PVOID pRoProperties;
    } 	CLUSTER_GROUP_ENUM_ITEM;

typedef struct _CLUSTER_GROUP_ENUM_ITEM *PCLUSTER_GROUP_ENUM_ITEM;

typedef struct _CLUSTER_RESOURCE_ENUM_ITEM
    {
    DWORD dwVersion;
    DWORD cbId;
    LPWSTR lpszId;
    DWORD cbName;
    LPWSTR lpszName;
    DWORD cbOwnerGroupName;
    LPWSTR lpszOwnerGroupName;
    DWORD cbOwnerGroupId;
    LPWSTR lpszOwnerGroupId;
    DWORD cbProperties;
    PVOID pProperties;
    DWORD cbRoProperties;
    PVOID pRoProperties;
    } 	CLUSTER_RESOURCE_ENUM_ITEM;

typedef struct _CLUSTER_RESOURCE_ENUM_ITEM *PCLUSTER_RESOURCE_ENUM_ITEM;

typedef 
enum CLUSTER_RESOURCE_STATE
    {
        ClusterResourceStateUnknown	= -1,
        ClusterResourceInherited	= ( ClusterResourceStateUnknown + 1 ) ,
        ClusterResourceInitializing	= ( ClusterResourceInherited + 1 ) ,
        ClusterResourceOnline	= ( ClusterResourceInitializing + 1 ) ,
        ClusterResourceOffline	= ( ClusterResourceOnline + 1 ) ,
        ClusterResourceFailed	= ( ClusterResourceOffline + 1 ) ,
        ClusterResourcePending	= 128,
        ClusterResourceOnlinePending	= ( ClusterResourcePending + 1 ) ,
        ClusterResourceOfflinePending	= ( ClusterResourceOnlinePending + 1 ) 
    } 	CLUSTER_RESOURCE_STATE;

typedef 
enum CLUSTER_RESOURCE_RESTART_ACTION
    {
        ClusterResourceDontRestart	= 0,
        ClusterResourceRestartNoNotify	= ( ClusterResourceDontRestart + 1 ) ,
        ClusterResourceRestartNotify	= ( ClusterResourceRestartNoNotify + 1 ) ,
        ClusterResourceRestartActionCount	= ( ClusterResourceRestartNotify + 1 ) 
    } 	CLUSTER_RESOURCE_RESTART_ACTION;

typedef enum CLUSTER_RESOURCE_RESTART_ACTION CRRA;

typedef 
enum CLUSTER_RESOURCE_EMBEDDED_FAILURE_ACTION
    {
        ClusterResourceEmbeddedFailureActionNone	= 0,
        ClusterResourceEmbeddedFailureActionLogOnly	= 1,
        ClusterResourceEmbeddedFailureActionRecover	= ( ClusterResourceEmbeddedFailureActionLogOnly + 1 ) 
    } 	CLUSTER_RESOURCE_EMBEDDED_FAILURE_ACTION;

typedef 
enum CLUSTER_RESOURCE_CREATE_FLAGS
    {
        CLUSTER_RESOURCE_DEFAULT_MONITOR	= 0,
        CLUSTER_RESOURCE_SEPARATE_MONITOR	= 1,
        CLUSTER_RESOURCE_VALID_FLAGS	= CLUSTER_RESOURCE_SEPARATE_MONITOR
    } 	CLUSTER_RESOURCE_CREATE_FLAGS;

typedef 
enum CLUSTER_SHARED_VOLUME_SNAPSHOT_STATE
    {
        ClusterSharedVolumeSnapshotStateUnknown	= 0,
        ClusterSharedVolumePrepareForHWSnapshot	= ( ClusterSharedVolumeSnapshotStateUnknown + 1 ) ,
        ClusterSharedVolumeHWSnapshotCompleted	= ( ClusterSharedVolumePrepareForHWSnapshot + 1 ) ,
        ClusterSharedVolumePrepareForFreeze	= ( ClusterSharedVolumeHWSnapshotCompleted + 1 ) 
    } 	CLUSTER_SHARED_VOLUME_SNAPSHOT_STATE;

typedef 
enum CLUSTER_PROPERTY_TYPE
    {
        CLUSPROP_TYPE_UNKNOWN	= -1,
        CLUSPROP_TYPE_ENDMARK	= 0,
        CLUSPROP_TYPE_LIST_VALUE	= ( CLUSPROP_TYPE_ENDMARK + 1 ) ,
        CLUSPROP_TYPE_RESCLASS	= ( CLUSPROP_TYPE_LIST_VALUE + 1 ) ,
        CLUSPROP_TYPE_RESERVED1	= ( CLUSPROP_TYPE_RESCLASS + 1 ) ,
        CLUSPROP_TYPE_NAME	= ( CLUSPROP_TYPE_RESERVED1 + 1 ) ,
        CLUSPROP_TYPE_SIGNATURE	= ( CLUSPROP_TYPE_NAME + 1 ) ,
        CLUSPROP_TYPE_SCSI_ADDRESS	= ( CLUSPROP_TYPE_SIGNATURE + 1 ) ,
        CLUSPROP_TYPE_DISK_NUMBER	= ( CLUSPROP_TYPE_SCSI_ADDRESS + 1 ) ,
        CLUSPROP_TYPE_PARTITION_INFO	= ( CLUSPROP_TYPE_DISK_NUMBER + 1 ) ,
        CLUSPROP_TYPE_FTSET_INFO	= ( CLUSPROP_TYPE_PARTITION_INFO + 1 ) ,
        CLUSPROP_TYPE_DISK_SERIALNUMBER	= ( CLUSPROP_TYPE_FTSET_INFO + 1 ) ,
        CLUSPROP_TYPE_DISK_GUID	= ( CLUSPROP_TYPE_DISK_SERIALNUMBER + 1 ) ,
        CLUSPROP_TYPE_DISK_SIZE	= ( CLUSPROP_TYPE_DISK_GUID + 1 ) ,
        CLUSPROP_TYPE_PARTITION_INFO_EX	= ( CLUSPROP_TYPE_DISK_SIZE + 1 ) ,
        CLUSPROP_TYPE_PARTITION_INFO_EX2	= ( CLUSPROP_TYPE_PARTITION_INFO_EX + 1 ) ,
        CLUSPROP_TYPE_STORAGE_DEVICE_ID_DESCRIPTOR	= ( CLUSPROP_TYPE_PARTITION_INFO_EX2 + 1 ) ,
        CLUSPROP_TYPE_USER	= 32768
    } 	CLUSTER_PROPERTY_TYPE;

typedef 
enum CLUSTER_PROPERTY_FORMAT
    {
        CLUSPROP_FORMAT_UNKNOWN	= 0,
        CLUSPROP_FORMAT_BINARY	= ( CLUSPROP_FORMAT_UNKNOWN + 1 ) ,
        CLUSPROP_FORMAT_DWORD	= ( CLUSPROP_FORMAT_BINARY + 1 ) ,
        CLUSPROP_FORMAT_SZ	= ( CLUSPROP_FORMAT_DWORD + 1 ) ,
        CLUSPROP_FORMAT_EXPAND_SZ	= ( CLUSPROP_FORMAT_SZ + 1 ) ,
        CLUSPROP_FORMAT_MULTI_SZ	= ( CLUSPROP_FORMAT_EXPAND_SZ + 1 ) ,
        CLUSPROP_FORMAT_ULARGE_INTEGER	= ( CLUSPROP_FORMAT_MULTI_SZ + 1 ) ,
        CLUSPROP_FORMAT_LONG	= ( CLUSPROP_FORMAT_ULARGE_INTEGER + 1 ) ,
        CLUSPROP_FORMAT_EXPANDED_SZ	= ( CLUSPROP_FORMAT_LONG + 1 ) ,
        CLUSPROP_FORMAT_SECURITY_DESCRIPTOR	= ( CLUSPROP_FORMAT_EXPANDED_SZ + 1 ) ,
        CLUSPROP_FORMAT_LARGE_INTEGER	= ( CLUSPROP_FORMAT_SECURITY_DESCRIPTOR + 1 ) ,
        CLUSPROP_FORMAT_WORD	= ( CLUSPROP_FORMAT_LARGE_INTEGER + 1 ) ,
        CLUSPROP_FORMAT_FILETIME	= ( CLUSPROP_FORMAT_WORD + 1 ) ,
        CLUSPROP_FORMAT_VALUE_LIST	= ( CLUSPROP_FORMAT_FILETIME + 1 ) ,
        CLUSPROP_FORMAT_PROPERTY_LIST	= ( CLUSPROP_FORMAT_VALUE_LIST + 1 ) ,
        CLUSPROP_FORMAT_USER	= 32768
    } 	CLUSTER_PROPERTY_FORMAT;

typedef 
enum CLUSTER_PROPERTY_SYNTAX
    {
        CLUSPROP_SYNTAX_ENDMARK	= ( DWORD  )(( ( CLUSPROP_TYPE_ENDMARK << 16 )  | CLUSPROP_FORMAT_UNKNOWN ) ),
        CLUSPROP_SYNTAX_NAME	= ( DWORD  )(( ( CLUSPROP_TYPE_NAME << 16 )  | CLUSPROP_FORMAT_SZ ) ),
        CLUSPROP_SYNTAX_RESCLASS	= ( DWORD  )(( ( CLUSPROP_TYPE_RESCLASS << 16 )  | CLUSPROP_FORMAT_DWORD ) ),
        CLUSPROP_SYNTAX_LIST_VALUE_SZ	= ( DWORD  )(( ( CLUSPROP_TYPE_LIST_VALUE << 16 )  | CLUSPROP_FORMAT_SZ ) ),
        CLUSPROP_SYNTAX_LIST_VALUE_EXPAND_SZ	= ( DWORD  )(( ( CLUSPROP_TYPE_LIST_VALUE << 16 )  | CLUSPROP_FORMAT_EXPAND_SZ ) ),
        CLUSPROP_SYNTAX_LIST_VALUE_DWORD	= ( DWORD  )(( ( CLUSPROP_TYPE_LIST_VALUE << 16 )  | CLUSPROP_FORMAT_DWORD ) ),
        CLUSPROP_SYNTAX_LIST_VALUE_BINARY	= ( DWORD  )(( ( CLUSPROP_TYPE_LIST_VALUE << 16 )  | CLUSPROP_FORMAT_BINARY ) ),
        CLUSPROP_SYNTAX_LIST_VALUE_MULTI_SZ	= ( DWORD  )(( ( CLUSPROP_TYPE_LIST_VALUE << 16 )  | CLUSPROP_FORMAT_MULTI_SZ ) ),
        CLUSPROP_SYNTAX_LIST_VALUE_LONG	= ( DWORD  )(( ( CLUSPROP_TYPE_LIST_VALUE << 16 )  | CLUSPROP_FORMAT_LONG ) ),
        CLUSPROP_SYNTAX_LIST_VALUE_EXPANDED_SZ	= ( DWORD  )(( ( CLUSPROP_TYPE_LIST_VALUE << 16 )  | CLUSPROP_FORMAT_EXPANDED_SZ ) ),
        CLUSPROP_SYNTAX_LIST_VALUE_SECURITY_DESCRIPTOR	= ( DWORD  )(( ( CLUSPROP_TYPE_LIST_VALUE << 16 )  | CLUSPROP_FORMAT_SECURITY_DESCRIPTOR ) ),
        CLUSPROP_SYNTAX_LIST_VALUE_LARGE_INTEGER	= ( DWORD  )(( ( CLUSPROP_TYPE_LIST_VALUE << 16 )  | CLUSPROP_FORMAT_LARGE_INTEGER ) ),
        CLUSPROP_SYNTAX_LIST_VALUE_ULARGE_INTEGER	= ( DWORD  )(( ( CLUSPROP_TYPE_LIST_VALUE << 16 )  | CLUSPROP_FORMAT_ULARGE_INTEGER ) ),
        CLUSPROP_SYNTAX_LIST_VALUE_WORD	= ( DWORD  )(( ( CLUSPROP_TYPE_LIST_VALUE << 16 )  | CLUSPROP_FORMAT_WORD ) ),
        CLUSPROP_SYNTAX_LIST_VALUE_PROPERTY_LIST	= ( DWORD  )(( ( CLUSPROP_TYPE_LIST_VALUE << 16 )  | CLUSPROP_FORMAT_PROPERTY_LIST ) ),
        CLUSPROP_SYNTAX_LIST_VALUE_FILETIME	= ( DWORD  )(( ( CLUSPROP_TYPE_LIST_VALUE << 16 )  | CLUSPROP_FORMAT_FILETIME ) ),
        CLUSPROP_SYNTAX_DISK_SIGNATURE	= ( DWORD  )(( ( CLUSPROP_TYPE_SIGNATURE << 16 )  | CLUSPROP_FORMAT_DWORD ) ),
        CLUSPROP_SYNTAX_SCSI_ADDRESS	= ( DWORD  )(( ( CLUSPROP_TYPE_SCSI_ADDRESS << 16 )  | CLUSPROP_FORMAT_DWORD ) ),
        CLUSPROP_SYNTAX_DISK_NUMBER	= ( DWORD  )(( ( CLUSPROP_TYPE_DISK_NUMBER << 16 )  | CLUSPROP_FORMAT_DWORD ) ),
        CLUSPROP_SYNTAX_PARTITION_INFO	= ( DWORD  )(( ( CLUSPROP_TYPE_PARTITION_INFO << 16 )  | CLUSPROP_FORMAT_BINARY ) ),
        CLUSPROP_SYNTAX_FTSET_INFO	= ( DWORD  )(( ( CLUSPROP_TYPE_FTSET_INFO << 16 )  | CLUSPROP_FORMAT_BINARY ) ),
        CLUSPROP_SYNTAX_DISK_SERIALNUMBER	= ( DWORD  )(( ( CLUSPROP_TYPE_DISK_SERIALNUMBER << 16 )  | CLUSPROP_FORMAT_SZ ) ),
        CLUSPROP_SYNTAX_DISK_GUID	= ( DWORD  )(( ( CLUSPROP_TYPE_DISK_GUID << 16 )  | CLUSPROP_FORMAT_SZ ) ),
        CLUSPROP_SYNTAX_DISK_SIZE	= ( DWORD  )(( ( CLUSPROP_TYPE_DISK_SIZE << 16 )  | CLUSPROP_FORMAT_ULARGE_INTEGER ) ),
        CLUSPROP_SYNTAX_PARTITION_INFO_EX	= ( DWORD  )(( ( CLUSPROP_TYPE_PARTITION_INFO_EX << 16 )  | CLUSPROP_FORMAT_BINARY ) ),
        CLUSPROP_SYNTAX_PARTITION_INFO_EX2	= ( DWORD  )(( ( CLUSPROP_TYPE_PARTITION_INFO_EX2 << 16 )  | CLUSPROP_FORMAT_BINARY ) ),
        CLUSPROP_SYNTAX_STORAGE_DEVICE_ID_DESCRIPTOR	= ( DWORD  )(( ( CLUSPROP_TYPE_STORAGE_DEVICE_ID_DESCRIPTOR << 16 )  | CLUSPROP_FORMAT_BINARY ) )
    } 	CLUSTER_PROPERTY_SYNTAX;

typedef struct GROUP_FAILURE_INFO
    {
    DWORD dwFailoverAttemptsRemaining;
    DWORD dwFailoverPeriodRemaining;
    } 	GROUP_FAILURE_INFO;

typedef struct GROUP_FAILURE_INFO *PGROUP_FAILURE_INFO;

typedef struct GROUP_FAILURE_INFO_BUFFER
    {
    DWORD dwVersion;
    GROUP_FAILURE_INFO Info;
    } 	GROUP_FAILURE_INFO_BUFFER;

typedef struct GROUP_FAILURE_INFO_BUFFER *PGROUP_FAILURE_INFO_BUFFER;

typedef struct RESOURCE_FAILURE_INFO
    {
    DWORD dwRestartAttemptsRemaining;
    DWORD dwRestartPeriodRemaining;
    } 	RESOURCE_FAILURE_INFO;

typedef struct RESOURCE_FAILURE_INFO *PRESOURCE_FAILURE_INFO;

typedef struct RESOURCE_FAILURE_INFO_BUFFER
    {
    DWORD dwVersion;
    RESOURCE_FAILURE_INFO Info;
    } 	RESOURCE_FAILURE_INFO_BUFFER;

typedef struct RESOURCE_FAILURE_INFO_BUFFER *PRESOURCE_FAILURE_INFO_BUFFER;

typedef struct RESOURCE_TERMINAL_FAILURE_INFO_BUFFER
    {
    BOOL isTerminalFailure;
    DWORD restartPeriodRemaining;
    } 	RESOURCE_TERMINAL_FAILURE_INFO_BUFFER;

typedef struct RESOURCE_TERMINAL_FAILURE_INFO_BUFFER *PRESOURCE_TERMINAL_FAILURE_INFO_BUFFER;

typedef 
enum CLUSTER_CONTROL_OBJECT
    {
        CLUS_OBJECT_INVALID	= 0,
        CLUS_OBJECT_RESOURCE	= ( CLUS_OBJECT_INVALID + 1 ) ,
        CLUS_OBJECT_RESOURCE_TYPE	= ( CLUS_OBJECT_RESOURCE + 1 ) ,
        CLUS_OBJECT_GROUP	= ( CLUS_OBJECT_RESOURCE_TYPE + 1 ) ,
        CLUS_OBJECT_NODE	= ( CLUS_OBJECT_GROUP + 1 ) ,
        CLUS_OBJECT_NETWORK	= ( CLUS_OBJECT_NODE + 1 ) ,
        CLUS_OBJECT_NETINTERFACE	= ( CLUS_OBJECT_NETWORK + 1 ) ,
        CLUS_OBJECT_CLUSTER	= ( CLUS_OBJECT_NETINTERFACE + 1 ) ,
        CLUS_OBJECT_GROUPSET	= ( CLUS_OBJECT_CLUSTER + 1 ) ,
        CLUS_OBJECT_AFFINITYRULE	= ( CLUS_OBJECT_GROUPSET + 1 ) ,
        CLUS_OBJECT_USER	= 128
    } 	CLUSTER_CONTROL_OBJECT;

typedef 
enum CLCTL_CODES
    {
        CLCTL_UNKNOWN	= ( ( ( 0 << 0 )  | ( ( 0 + 0 )  << 2 )  )  | ( 0 << 22 )  ) ,
        CLCTL_GET_CHARACTERISTICS	= ( ( ( 0x1 << 0 )  | ( ( 0 + 1 )  << 2 )  )  | ( 0 << 22 )  ) ,
        CLCTL_GET_FLAGS	= ( ( ( 0x1 << 0 )  | ( ( 0 + 2 )  << 2 )  )  | ( 0 << 22 )  ) ,
        CLCTL_GET_CLASS_INFO	= ( ( ( 0x1 << 0 )  | ( ( 0 + 3 )  << 2 )  )  | ( 0 << 22 )  ) ,
        CLCTL_GET_REQUIRED_DEPENDENCIES	= ( ( ( 0x1 << 0 )  | ( ( 0 + 4 )  << 2 )  )  | ( 0 << 22 )  ) ,
        CLCTL_GET_ARB_TIMEOUT	= ( ( ( 0x1 << 0 )  | ( ( 0 + 5 )  << 2 )  )  | ( 0 << 22 )  ) ,
        CLCTL_GET_FAILURE_INFO	= ( ( ( 0x1 << 0 )  | ( ( 0 + 6 )  << 2 )  )  | ( 0 << 22 )  ) ,
        CLCTL_GET_NAME	= ( ( ( 0x1 << 0 )  | ( ( 0 + 10 )  << 2 )  )  | ( 0 << 22 )  ) ,
        CLCTL_GET_RESOURCE_TYPE	= ( ( ( 0x1 << 0 )  | ( ( 0 + 11 )  << 2 )  )  | ( 0 << 22 )  ) ,
        CLCTL_GET_NODE	= ( ( ( 0x1 << 0 )  | ( ( 0 + 12 )  << 2 )  )  | ( 0 << 22 )  ) ,
        CLCTL_GET_NETWORK	= ( ( ( 0x1 << 0 )  | ( ( 0 + 13 )  << 2 )  )  | ( 0 << 22 )  ) ,
        CLCTL_GET_ID	= ( ( ( 0x1 << 0 )  | ( ( 0 + 14 )  << 2 )  )  | ( 0 << 22 )  ) ,
        CLCTL_GET_FQDN	= ( ( ( 0x1 << 0 )  | ( ( 0 + 15 )  << 2 )  )  | ( 0 << 22 )  ) ,
        CLCTL_GET_CLUSTER_SERVICE_ACCOUNT_NAME	= ( ( ( 0x1 << 0 )  | ( ( 0 + 16 )  << 2 )  )  | ( 0 << 22 )  ) ,
        CLCTL_CHECK_VOTER_EVICT	= ( ( ( 0x1 << 0 )  | ( ( 0 + 17 )  << 2 )  )  | ( 0 << 22 )  ) ,
        CLCTL_CHECK_VOTER_DOWN	= ( ( ( 0x1 << 0 )  | ( ( 0 + 18 )  << 2 )  )  | ( 0 << 22 )  ) ,
        CLCTL_SHUTDOWN	= ( ( ( 0x1 << 0 )  | ( ( 0 + 19 )  << 2 )  )  | ( 0 << 22 )  ) ,
        CLCTL_ENUM_COMMON_PROPERTIES	= ( ( ( 0x1 << 0 )  | ( ( 0 + 20 )  << 2 )  )  | ( 0 << 22 )  ) ,
        CLCTL_GET_RO_COMMON_PROPERTIES	= ( ( ( 0x1 << 0 )  | ( ( 0 + 21 )  << 2 )  )  | ( 0 << 22 )  ) ,
        CLCTL_GET_COMMON_PROPERTIES	= ( ( ( 0x1 << 0 )  | ( ( 0 + 22 )  << 2 )  )  | ( 0 << 22 )  ) ,
        CLCTL_SET_COMMON_PROPERTIES	= ( ( ( 0x2 << 0 )  | ( ( 0 + 23 )  << 2 )  )  | ( 0x1 << 22 )  ) ,
        CLCTL_VALIDATE_COMMON_PROPERTIES	= ( ( ( 0x1 << 0 )  | ( ( 0 + 24 )  << 2 )  )  | ( 0 << 22 )  ) ,
        CLCTL_GET_COMMON_PROPERTY_FMTS	= ( ( ( 0x1 << 0 )  | ( ( 0 + 25 )  << 2 )  )  | ( 0 << 22 )  ) ,
        CLCTL_GET_COMMON_RESOURCE_PROPERTY_FMTS	= ( ( ( 0x1 << 0 )  | ( ( 0 + 26 )  << 2 )  )  | ( 0 << 22 )  ) ,
        CLCTL_CHECK_VOTER_EVICT_WITNESS	= ( ( ( 0x1 << 0 )  | ( ( 0 + 27 )  << 2 )  )  | ( 0 << 22 )  ) ,
        CLCTL_CHECK_VOTER_DOWN_WITNESS	= ( ( ( 0x1 << 0 )  | ( ( 0 + 28 )  << 2 )  )  | ( 0 << 22 )  ) ,
        CLCTL_ENUM_PRIVATE_PROPERTIES	= ( ( ( 0x1 << 0 )  | ( ( 0 + 30 )  << 2 )  )  | ( 0 << 22 )  ) ,
        CLCTL_GET_RO_PRIVATE_PROPERTIES	= ( ( ( 0x1 << 0 )  | ( ( 0 + 31 )  << 2 )  )  | ( 0 << 22 )  ) ,
        CLCTL_GET_PRIVATE_PROPERTIES	= ( ( ( 0x1 << 0 )  | ( ( 0 + 32 )  << 2 )  )  | ( 0 << 22 )  ) ,
        CLCTL_SET_PRIVATE_PROPERTIES	= ( ( ( 0x2 << 0 )  | ( ( 0 + 33 )  << 2 )  )  | ( 0x1 << 22 )  ) ,
        CLCTL_VALIDATE_PRIVATE_PROPERTIES	= ( ( ( 0x1 << 0 )  | ( ( 0 + 34 )  << 2 )  )  | ( 0 << 22 )  ) ,
        CLCTL_GET_PRIVATE_PROPERTY_FMTS	= ( ( ( 0x1 << 0 )  | ( ( 0 + 35 )  << 2 )  )  | ( 0 << 22 )  ) ,
        CLCTL_GET_PRIVATE_RESOURCE_PROPERTY_FMTS	= ( ( ( 0x1 << 0 )  | ( ( 0 + 36 )  << 2 )  )  | ( 0 << 22 )  ) ,
        CLCTL_ADD_REGISTRY_CHECKPOINT	= ( ( ( 0x2 << 0 )  | ( ( 0 + 40 )  << 2 )  )  | ( 0x1 << 22 )  ) ,
        CLCTL_DELETE_REGISTRY_CHECKPOINT	= ( ( ( 0x2 << 0 )  | ( ( 0 + 41 )  << 2 )  )  | ( 0x1 << 22 )  ) ,
        CLCTL_GET_REGISTRY_CHECKPOINTS	= ( ( ( 0x1 << 0 )  | ( ( 0 + 42 )  << 2 )  )  | ( 0 << 22 )  ) ,
        CLCTL_ADD_CRYPTO_CHECKPOINT	= ( ( ( 0x2 << 0 )  | ( ( 0 + 43 )  << 2 )  )  | ( 0x1 << 22 )  ) ,
        CLCTL_DELETE_CRYPTO_CHECKPOINT	= ( ( ( 0x2 << 0 )  | ( ( 0 + 44 )  << 2 )  )  | ( 0x1 << 22 )  ) ,
        CLCTL_GET_CRYPTO_CHECKPOINTS	= ( ( ( 0x1 << 0 )  | ( ( 0 + 45 )  << 2 )  )  | ( 0 << 22 )  ) ,
        CLCTL_RESOURCE_UPGRADE_DLL	= ( ( ( 0x2 << 0 )  | ( ( 0 + 46 )  << 2 )  )  | ( 0x1 << 22 )  ) ,
        CLCTL_ADD_REGISTRY_CHECKPOINT_64BIT	= ( ( ( 0x2 << 0 )  | ( ( 0 + 47 )  << 2 )  )  | ( 0x1 << 22 )  ) ,
        CLCTL_ADD_REGISTRY_CHECKPOINT_32BIT	= ( ( ( 0x2 << 0 )  | ( ( 0 + 48 )  << 2 )  )  | ( 0x1 << 22 )  ) ,
        CLCTL_GET_LOADBAL_PROCESS_LIST	= ( ( ( 0x1 << 0 )  | ( ( 0 + 50 )  << 2 )  )  | ( 0 << 22 )  ) ,
        CLCTL_SET_ACCOUNT_ACCESS	= ( ( ( 0x2 << 0 )  | ( ( 0 + 60 )  << 2 )  )  | ( 0x1 << 22 )  ) ,
        CLCTL_GET_NETWORK_NAME	= ( ( ( 0x1 << 0 )  | ( ( 0 + 90 )  << 2 )  )  | ( 0 << 22 )  ) ,
        CLCTL_NETNAME_GET_VIRTUAL_SERVER_TOKEN	= ( ( ( 0x1 << 0 )  | ( ( 0 + 91 )  << 2 )  )  | ( 0 << 22 )  ) ,
        CLCTL_NETNAME_REGISTER_DNS_RECORDS	= ( ( ( 0x2 << 0 )  | ( ( 0 + 92 )  << 2 )  )  | ( 0 << 22 )  ) ,
        CLCTL_GET_DNS_NAME	= ( ( ( 0x1 << 0 )  | ( ( 0 + 93 )  << 2 )  )  | ( 0 << 22 )  ) ,
        CLCTL_NETNAME_SET_PWD_INFO	= ( ( ( 0x2 << 0 )  | ( ( 0 + 94 )  << 2 )  )  | ( 0 << 22 )  ) ,
        CLCTL_NETNAME_DELETE_CO	= ( ( ( 0x2 << 0 )  | ( ( 0 + 95 )  << 2 )  )  | ( 0 << 22 )  ) ,
        CLCTL_NETNAME_VALIDATE_VCO	= ( ( ( 0x1 << 0 )  | ( ( 0 + 96 )  << 2 )  )  | ( 0 << 22 )  ) ,
        CLCTL_NETNAME_RESET_VCO	= ( ( ( 0x1 << 0 )  | ( ( 0 + 97 )  << 2 )  )  | ( 0 << 22 )  ) ,
        CLCTL_NETNAME_REPAIR_VCO	= ( ( ( 0x1 << 0 )  | ( ( 0 + 99 )  << 2 )  )  | ( 0 << 22 )  ) ,
        CLCTL_STORAGE_GET_DISK_INFO	= ( ( ( 0x1 << 0 )  | ( ( 0 + 100 )  << 2 )  )  | ( 0 << 22 )  ) ,
        CLCTL_STORAGE_GET_AVAILABLE_DISKS	= ( ( ( 0x1 << 0 )  | ( ( 0 + 101 )  << 2 )  )  | ( 0 << 22 )  ) ,
        CLCTL_STORAGE_IS_PATH_VALID	= ( ( ( 0x1 << 0 )  | ( ( 0 + 102 )  << 2 )  )  | ( 0 << 22 )  ) ,
        CLCTL_STORAGE_SYNC_CLUSDISK_DB	= ( ( ( 0x2 << 0 )  | ( ( 0 + 103 )  << 2 )  )  | ( 0x1 << 22 )  ) ,
        CLCTL_STORAGE_GET_DISK_NUMBER_INFO	= ( ( ( 0x1 << 0 )  | ( ( 0 + 104 )  << 2 )  )  | ( 0 << 22 )  ) ,
        CLCTL_QUERY_DELETE	= ( ( ( 0x1 << 0 )  | ( ( 0 + 110 )  << 2 )  )  | ( 0 << 22 )  ) ,
        CLCTL_IPADDRESS_RENEW_LEASE	= ( ( ( 0x2 << 0 )  | ( ( 0 + 111 )  << 2 )  )  | ( 0x1 << 22 )  ) ,
        CLCTL_IPADDRESS_RELEASE_LEASE	= ( ( ( 0x2 << 0 )  | ( ( 0 + 112 )  << 2 )  )  | ( 0x1 << 22 )  ) ,
        CLCTL_QUERY_MAINTENANCE_MODE	= ( ( ( 0x1 << 0 )  | ( ( 0 + 120 )  << 2 )  )  | ( 0 << 22 )  ) ,
        CLCTL_SET_MAINTENANCE_MODE	= ( ( ( 0x2 << 0 )  | ( ( 0 + 121 )  << 2 )  )  | ( 0x1 << 22 )  ) ,
        CLCTL_STORAGE_SET_DRIVELETTER	= ( ( ( 0x2 << 0 )  | ( ( 0 + 122 )  << 2 )  )  | ( 0x1 << 22 )  ) ,
        CLCTL_STORAGE_GET_DRIVELETTERS	= ( ( ( 0x1 << 0 )  | ( ( 0 + 123 )  << 2 )  )  | ( 0 << 22 )  ) ,
        CLCTL_STORAGE_GET_DISK_INFO_EX	= ( ( ( 0x1 << 0 )  | ( ( 0 + 124 )  << 2 )  )  | ( 0 << 22 )  ) ,
        CLCTL_STORAGE_GET_AVAILABLE_DISKS_EX	= ( ( ( 0x1 << 0 )  | ( ( 0 + 125 )  << 2 )  )  | ( 0 << 22 )  ) ,
        CLCTL_STORAGE_GET_DISK_INFO_EX2	= ( ( ( 0x1 << 0 )  | ( ( 0 + 126 )  << 2 )  )  | ( 0 << 22 )  ) ,
        CLCTL_STORAGE_GET_CLUSPORT_DISK_COUNT	= ( ( ( 0x1 << 0 )  | ( ( 0 + 127 )  << 2 )  )  | ( 0 << 22 )  ) ,
        CLCTL_STORAGE_REMAP_DRIVELETTER	= ( ( ( 0x1 << 0 )  | ( ( 0 + 128 )  << 2 )  )  | ( 0 << 22 )  ) ,
        CLCTL_STORAGE_GET_DISKID	= ( ( ( 0x1 << 0 )  | ( ( 0 + 129 )  << 2 )  )  | ( 0 << 22 )  ) ,
        CLCTL_STORAGE_IS_CLUSTERABLE	= ( ( ( 0x1 << 0 )  | ( ( 0 + 130 )  << 2 )  )  | ( 0 << 22 )  ) ,
        CLCTL_STORAGE_REMOVE_VM_OWNERSHIP	= ( ( ( 0x2 << 0 )  | ( ( 0 + 131 )  << 2 )  )  | ( 0x1 << 22 )  ) ,
        CLCTL_STORAGE_GET_MOUNTPOINTS	= ( ( ( 0x1 << 0 )  | ( ( 0 + 132 )  << 2 )  )  | ( 0 << 22 )  ) ,
        CLCTL_STORAGE_GET_DIRTY	= ( ( ( 0x1 << 0 )  | ( ( 0 + 134 )  << 2 )  )  | ( 0 << 22 )  ) ,
        CLCTL_STORAGE_GET_SHARED_VOLUME_INFO	= ( ( ( 0x1 << 0 )  | ( ( 0 + 137 )  << 2 )  )  | ( 0 << 22 )  ) ,
        CLCTL_STORAGE_IS_CSV_FILE	= ( ( ( 0x1 << 0 )  | ( ( 0 + 138 )  << 2 )  )  | ( 0 << 22 )  ) ,
        CLCTL_STORAGE_GET_RESOURCEID	= ( ( ( 0x1 << 0 )  | ( ( 0 + 139 )  << 2 )  )  | ( 0 << 22 )  ) ,
        CLCTL_VALIDATE_PATH	= ( ( ( 0x1 << 0 )  | ( ( 0 + 140 )  << 2 )  )  | ( 0 << 22 )  ) ,
        CLCTL_VALIDATE_NETNAME	= ( ( ( 0x1 << 0 )  | ( ( 0 + 141 )  << 2 )  )  | ( 0 << 22 )  ) ,
        CLCTL_VALIDATE_DIRECTORY	= ( ( ( 0x1 << 0 )  | ( ( 0 + 142 )  << 2 )  )  | ( 0 << 22 )  ) ,
        CLCTL_BATCH_BLOCK_KEY	= ( ( ( 0x2 << 0 )  | ( ( 0 + 143 )  << 2 )  )  | ( 0 << 22 )  ) ,
        CLCTL_BATCH_UNBLOCK_KEY	= ( ( ( 0x1 << 0 )  | ( ( 0 + 144 )  << 2 )  )  | ( 0 << 22 )  ) ,
        CLCTL_FILESERVER_SHARE_ADD	= ( ( ( 0x2 << 0 )  | ( ( 0 + 145 )  << 2 )  )  | ( 0x1 << 22 )  ) ,
        CLCTL_FILESERVER_SHARE_DEL	= ( ( ( 0x2 << 0 )  | ( ( 0 + 146 )  << 2 )  )  | ( 0x1 << 22 )  ) ,
        CLCTL_FILESERVER_SHARE_MODIFY	= ( ( ( 0x2 << 0 )  | ( ( 0 + 147 )  << 2 )  )  | ( 0x1 << 22 )  ) ,
        CLCTL_FILESERVER_SHARE_REPORT	= ( ( ( 0x1 << 0 )  | ( ( 0 + 148 )  << 2 )  )  | ( 0 << 22 )  ) ,
        CLCTL_NETNAME_GET_OU_FOR_VCO	= ( ( ( 0x2 << 0 )  | ( ( 0 + 155 )  << 2 )  )  | ( 0x1 << 22 )  ) ,
        CLCTL_ENABLE_SHARED_VOLUME_DIRECTIO	= ( ( ( 0x2 << 0 )  | ( ( 0 + 162 )  << 2 )  )  | ( 0x1 << 22 )  ) ,
        CLCTL_DISABLE_SHARED_VOLUME_DIRECTIO	= ( ( ( 0x2 << 0 )  | ( ( 0 + 163 )  << 2 )  )  | ( 0x1 << 22 )  ) ,
        CLCTL_GET_SHARED_VOLUME_ID	= ( ( ( 0x1 << 0 )  | ( ( 0 + 164 )  << 2 )  )  | ( 0 << 22 )  ) ,
        CLCTL_SET_CSV_MAINTENANCE_MODE	= ( ( ( 0x2 << 0 )  | ( ( 0 + 165 )  << 2 )  )  | ( 0x1 << 22 )  ) ,
        CLCTL_SET_SHARED_VOLUME_BACKUP_MODE	= ( ( ( 0x2 << 0 )  | ( ( 0 + 166 )  << 2 )  )  | ( 0x1 << 22 )  ) ,
        CLCTL_STORAGE_GET_SHARED_VOLUME_PARTITION_NAMES	= ( ( ( 0x1 << 0 )  | ( ( 0 + 167 )  << 2 )  )  | ( 0 << 22 )  ) ,
        CLCTL_STORAGE_GET_SHARED_VOLUME_STATES	= ( ( ( 0x2 << 0 )  | ( ( 0 + 168 )  << 2 )  )  | ( 0x1 << 22 )  ) ,
        CLCTL_STORAGE_IS_SHARED_VOLUME	= ( ( ( 0x1 << 0 )  | ( ( 0 + 169 )  << 2 )  )  | ( 0 << 22 )  ) ,
        CLCTL_GET_CLUSDB_TIMESTAMP	= ( ( ( 0x1 << 0 )  | ( ( 0 + 170 )  << 2 )  )  | ( 0 << 22 )  ) ,
        CLCTL_RW_MODIFY_NOOP	= ( ( ( 0x2 << 0 )  | ( ( 0 + 171 )  << 2 )  )  | ( 0x1 << 22 )  ) ,
        CLCTL_IS_QUORUM_BLOCKED	= ( ( ( 0x1 << 0 )  | ( ( 0 + 172 )  << 2 )  )  | ( 0 << 22 )  ) ,
        CLCTL_POOL_GET_DRIVE_INFO	= ( ( ( 0x1 << 0 )  | ( ( 0 + 173 )  << 2 )  )  | ( 0 << 22 )  ) ,
        CLCTL_GET_GUM_LOCK_OWNER	= ( ( ( 0x1 << 0 )  | ( ( 0 + 174 )  << 2 )  )  | ( 0 << 22 )  ) ,
        CLCTL_GET_STUCK_NODES	= ( ( ( 0x1 << 0 )  | ( ( 0 + 175 )  << 2 )  )  | ( 0 << 22 )  ) ,
        CLCTL_INJECT_GEM_FAULT	= ( ( ( 0x1 << 0 )  | ( ( 0 + 176 )  << 2 )  )  | ( 0 << 22 )  ) ,
        CLCTL_INTRODUCE_GEM_REPAIR_DELAY	= ( ( ( 0x1 << 0 )  | ( ( 0 + 177 )  << 2 )  )  | ( 0 << 22 )  ) ,
        CLCTL_SEND_DUMMY_GEM_MESSAGES	= ( ( ( 0x1 << 0 )  | ( ( 0 + 178 )  << 2 )  )  | ( 0 << 22 )  ) ,
        CLCTL_BLOCK_GEM_SEND_RECV	= ( ( ( 0x1 << 0 )  | ( ( 0 + 179 )  << 2 )  )  | ( 0 << 22 )  ) ,
        CLCTL_GET_GEMID_VECTOR	= ( ( ( 0x1 << 0 )  | ( ( 0 + 180 )  << 2 )  )  | ( 0 << 22 )  ) ,
        CLCTL_ADD_CRYPTO_CHECKPOINT_EX	= ( ( ( 0x2 << 0 )  | ( ( 0 + 181 )  << 2 )  )  | ( 0x1 << 22 )  ) ,
        CLCTL_GROUP_GET_LAST_MOVE_TIME	= ( ( ( 0x1 << 0 )  | ( ( 0 + 182 )  << 2 )  )  | ( 0 << 22 )  ) ,
        CLCTL_SET_STORAGE_CONFIGURATION	= ( ( ( 0x2 << 0 )  | ( ( 0 + 184 )  << 2 )  )  | ( 0x1 << 22 )  ) ,
        CLCTL_GET_STORAGE_CONFIGURATION	= ( ( ( 0x1 << 0 )  | ( ( 0 + 185 )  << 2 )  )  | ( 0 << 22 )  ) ,
        CLCTL_GET_STORAGE_CONFIG_ATTRIBUTES	= ( ( ( 0x1 << 0 )  | ( ( 0 + 186 )  << 2 )  )  | ( 0 << 22 )  ) ,
        CLCTL_REMOVE_NODE	= ( ( ( 0x2 << 0 )  | ( ( 0 + 187 )  << 2 )  )  | ( 0x1 << 22 )  ) ,
        CLCTL_IS_FEATURE_INSTALLED	= ( ( ( 0x1 << 0 )  | ( ( 0 + 188 )  << 2 )  )  | ( 0 << 22 )  ) ,
        CLCTL_IS_S2D_FEATURE_SUPPORTED	= ( ( ( 0x1 << 0 )  | ( ( 0 + 189 )  << 2 )  )  | ( 0 << 22 )  ) ,
        CLCTL_STORAGE_GET_PHYSICAL_DISK_INFO	= ( ( ( 0x1 << 0 )  | ( ( 0 + 190 )  << 2 )  )  | ( 0 << 22 )  ) ,
        CLCTL_STORAGE_GET_CLUSBFLT_PATHS	= ( ( ( 0x1 << 0 )  | ( ( 0 + 191 )  << 2 )  )  | ( 0 << 22 )  ) ,
        CLCTL_STORAGE_GET_CLUSBFLT_PATHINFO	= ( ( ( 0x1 << 0 )  | ( ( 0 + 192 )  << 2 )  )  | ( 0 << 22 )  ) ,
        CLCTL_CLEAR_NODE_CONNECTION_INFO	= ( ( ( 0x2 << 0 )  | ( ( 0 + 193 )  << 2 )  )  | ( 0x1 << 22 )  ) ,
        CLCTL_SET_DNS_DOMAIN	= ( ( ( 0x2 << 0 )  | ( ( 0 + 194 )  << 2 )  )  | ( 0x1 << 22 )  ) ,
        CTCTL_GET_ROUTESTATUS_BASIC	= ( ( ( 0x1 << 0 )  | ( ( 0 + 195 )  << 2 )  )  | ( 0 << 22 )  ) ,
        CTCTL_GET_ROUTESTATUS_EXTENDED	= ( ( ( 0x1 << 0 )  | ( ( 0 + 196 )  << 2 )  )  | ( 0 << 22 )  ) ,
        CTCTL_GET_FAULT_DOMAIN_STATE	= ( ( ( 0x1 << 0 )  | ( ( 0 + 197 )  << 2 )  )  | ( 0 << 22 )  ) ,
        CLCTL_NETNAME_SET_PWD_INFOEX	= ( ( ( 0x2 << 0 )  | ( ( 0 + 198 )  << 2 )  )  | ( 0 << 22 )  ) ,
        CLCTL_GET_NODE_NETWORK_CONNECTIVITY	= ( ( ( 0x1 << 0 )  | ( ( 0 + 199 )  << 2 )  )  | ( 0 << 22 )  ) ,
        CLCTL_STORAGE_GET_AVAILABLE_DISKS_EX2_INT	= ( ( ( 0x1 << 0 )  | ( ( 0 + 2040 )  << 2 )  )  | ( 0 << 22 )  ) ,
        CLCTL_CLOUD_WITNESS_RESOURCE_TYPE_VALIDATE_CREDENTIALS	= ( ( ( 0x1 << 0 )  | ( ( 0 + 2104 )  << 2 )  )  | ( 0 << 22 )  ) ,
        CLCTL_CLOUD_WITNESS_RESOURCE_UPDATE_TOKEN	= ( ( ( 0x2 << 0 )  | ( ( 0 + 2105 )  << 2 )  )  | ( 0x1 << 22 )  ) ,
        CLCTL_RESOURCE_PREPARE_UPGRADE	= ( ( ( 0x2 << 0 )  | ( ( 0 + 2106 )  << 2 )  )  | ( 0x1 << 22 )  ) ,
        CLCTL_RESOURCE_UPGRADE_COMPLETED	= ( ( ( 0x2 << 0 )  | ( ( 0 + 2107 )  << 2 )  )  | ( 0x1 << 22 )  ) ,
        CLCTL_CLOUD_WITNESS_RESOURCE_TYPE_VALIDATE_CREDENTIALS_WITH_KEY	= ( ( ( 0x1 << 0 )  | ( ( 0 + 2108 )  << 2 )  )  | ( 0 << 22 )  ) ,
        CLCTL_CLOUD_WITNESS_RESOURCE_UPDATE_KEY	= ( ( ( 0x2 << 0 )  | ( ( 0 + 2109 )  << 2 )  )  | ( 0x1 << 22 )  ) ,
        CLCTL_REPLICATION_ADD_REPLICATION_GROUP	= ( ( ( 0x2 << 0 )  | ( ( 0 + 2128 )  << 2 )  )  | ( 0 << 22 )  ) ,
        CLCTL_REPLICATION_GET_LOG_INFO	= ( ( ( 0x1 << 0 )  | ( ( 0 + 2129 )  << 2 )  )  | ( 0 << 22 )  ) ,
        CLCTL_REPLICATION_GET_ELIGIBLE_LOGDISKS	= ( ( ( 0x1 << 0 )  | ( ( 0 + 2130 )  << 2 )  )  | ( 0 << 22 )  ) ,
        CLCTL_REPLICATION_GET_ELIGIBLE_TARGET_DATADISKS	= ( ( ( 0x1 << 0 )  | ( ( 0 + 2131 )  << 2 )  )  | ( 0 << 22 )  ) ,
        CLCTL_REPLICATION_GET_ELIGIBLE_SOURCE_DATADISKS	= ( ( ( 0x1 << 0 )  | ( ( 0 + 2132 )  << 2 )  )  | ( 0 << 22 )  ) ,
        CLCTL_REPLICATION_GET_REPLICATED_DISKS	= ( ( ( 0x1 << 0 )  | ( ( 0 + 2133 )  << 2 )  )  | ( 0 << 22 )  ) ,
        CLCTL_REPLICATION_GET_REPLICA_VOLUMES	= ( ( ( 0x1 << 0 )  | ( ( 0 + 2134 )  << 2 )  )  | ( 0 << 22 )  ) ,
        CLCTL_REPLICATION_GET_LOG_VOLUME	= ( ( ( 0x1 << 0 )  | ( ( 0 + 2135 )  << 2 )  )  | ( 0 << 22 )  ) ,
        CLCTL_REPLICATION_GET_RESOURCE_GROUP	= ( ( ( 0x1 << 0 )  | ( ( 0 + 2136 )  << 2 )  )  | ( 0 << 22 )  ) ,
        CLCTL_REPLICATION_GET_REPLICATED_PARTITION_INFO	= ( ( ( 0x1 << 0 )  | ( ( 0 + 2137 )  << 2 )  )  | ( 0 << 22 )  ) ,
        CLCTL_GET_STATE_CHANGE_TIME	= ( ( ( 0x1 << 0 )  | ( ( 0 + 2903 )  << 2 )  )  | ( 0 << 22 )  ) ,
        CLCTL_SET_CLUSTER_S2D_ENABLED	= ( ( ( 0x2 << 0 )  | ( ( 0 + 2904 )  << 2 )  )  | ( 0x1 << 22 )  ) ,
        CLCTL_SET_CLUSTER_S2D_CACHE_METADATA_RESERVE_BYTES	= ( ( ( 0x2 << 0 )  | ( ( 0 + 2907 )  << 2 )  )  | ( 0x1 << 22 )  ) ,
        CLCTL_GROUPSET_GET_GROUPS	= ( ( ( 0x1 << 0 )  | ( ( 0 + 2908 )  << 2 )  )  | ( 0 << 22 )  ) ,
        CLCTL_GROUPSET_GET_PROVIDER_GROUPS	= ( ( ( 0x1 << 0 )  | ( ( 0 + 2909 )  << 2 )  )  | ( 0 << 22 )  ) ,
        CLCTL_GROUPSET_GET_PROVIDER_GROUPSETS	= ( ( ( 0x1 << 0 )  | ( ( 0 + 2910 )  << 2 )  )  | ( 0 << 22 )  ) ,
        CLCTL_GROUP_GET_PROVIDER_GROUPS	= ( ( ( 0x1 << 0 )  | ( ( 0 + 2911 )  << 2 )  )  | ( 0 << 22 )  ) ,
        CLCTL_GROUP_GET_PROVIDER_GROUPSETS	= ( ( ( 0x1 << 0 )  | ( ( 0 + 2912 )  << 2 )  )  | ( 0 << 22 )  ) ,
        CLCTL_GROUP_SET_CCF_FROM_MASTER	= ( ( ( 0x2 << 0 )  | ( ( 0 + 2913 )  << 2 )  )  | ( 0x1 << 22 )  ) ,
        CLCTL_GET_INFRASTRUCTURE_SOFS_BUFFER	= ( ( ( 0x1 << 0 )  | ( ( 0 + 2914 )  << 2 )  )  | ( 0 << 22 )  ) ,
        CLCTL_SET_INFRASTRUCTURE_SOFS_BUFFER	= ( ( ( 0x2 << 0 )  | ( ( 0 + 2915 )  << 2 )  )  | ( 0x1 << 22 )  ) ,
        CLCTL_NOTIFY_INFRASTRUCTURE_SOFS_CHANGED	= ( ( ( 0x2 << 0 )  | ( ( 0 + 2916 )  << 2 )  )  | ( 0x1 << 22 )  ) ,
        CLCTL_SCALEOUT_COMMAND	= ( ( ( 0x2 << 0 )  | ( ( 0 + 2917 )  << 2 )  )  | ( 0x1 << 22 )  ) ,
        CLCTL_SCALEOUT_CONTROL	= ( ( ( 0x2 << 0 )  | ( ( 0 + 2918 )  << 2 )  )  | ( 0x1 << 22 )  ) ,
        CLCTL_SCALEOUT_GET_CLUSTERS	= ( ( ( 0x1 << 0 )  | ( ( 0 + 2919 )  << 2 )  )  | ( 0x1 << 22 )  ) ,
        CLCTL_RELOAD_AUTOLOGGER_CONFIG	= ( ( ( 0x2 << 0 )  | ( ( 0 + 2932 )  << 2 )  )  | ( 0 << 22 )  ) ,
        CLCTL_STORAGE_RENAME_SHARED_VOLUME	= ( ( ( 0x2 << 0 )  | ( ( 0 + 2933 )  << 2 )  )  | ( 0 << 22 )  ) ,
        CLCTL_STORAGE_RENAME_SHARED_VOLUME_GUID	= ( ( ( 0x2 << 0 )  | ( ( 0 + 2934 )  << 2 )  )  | ( 0 << 22 )  ) ,
        CLCTL_ENUM_AFFINITY_RULE_NAMES	= ( ( ( 0x1 << 0 )  | ( ( 0 + 2935 )  << 2 )  )  | ( 0 << 22 )  ) ,
        CLCTL_GET_NODES_IN_FD	= ( ( ( 0x1 << 0 )  | ( ( 0 + 2936 )  << 2 )  )  | ( 0 << 22 )  ) ,
        CLCTL_FORCE_DB_FLUSH	= ( ( ( 0x2 << 0 )  | ( ( 0 + 2937 )  << 2 )  )  | ( 0x1 << 22 )  ) ,
        CLCTL_DELETE	= ( ( ( ( 0x2 << 0 )  | ( 1 << 20 )  )  | ( ( 0 + 1 )  << 2 )  )  | ( 0x1 << 22 )  ) ,
        CLCTL_INSTALL_NODE	= ( ( ( ( 0x2 << 0 )  | ( 1 << 20 )  )  | ( ( 0 + 2 )  << 2 )  )  | ( 0x1 << 22 )  ) ,
        CLCTL_EVICT_NODE	= ( ( ( ( 0x2 << 0 )  | ( 1 << 20 )  )  | ( ( 0 + 3 )  << 2 )  )  | ( 0x1 << 22 )  ) ,
        CLCTL_ADD_DEPENDENCY	= ( ( ( ( 0x2 << 0 )  | ( 1 << 20 )  )  | ( ( 0 + 4 )  << 2 )  )  | ( 0x1 << 22 )  ) ,
        CLCTL_REMOVE_DEPENDENCY	= ( ( ( ( 0x2 << 0 )  | ( 1 << 20 )  )  | ( ( 0 + 5 )  << 2 )  )  | ( 0x1 << 22 )  ) ,
        CLCTL_ADD_OWNER	= ( ( ( ( 0x2 << 0 )  | ( 1 << 20 )  )  | ( ( 0 + 6 )  << 2 )  )  | ( 0x1 << 22 )  ) ,
        CLCTL_REMOVE_OWNER	= ( ( ( ( 0x2 << 0 )  | ( 1 << 20 )  )  | ( ( 0 + 7 )  << 2 )  )  | ( 0x1 << 22 )  ) ,
        CLCTL_SET_NAME	= ( ( ( ( 0x2 << 0 )  | ( 1 << 20 )  )  | ( ( 0 + 9 )  << 2 )  )  | ( 0x1 << 22 )  ) ,
        CLCTL_CLUSTER_NAME_CHANGED	= ( ( ( ( 0x2 << 0 )  | ( 1 << 20 )  )  | ( ( 0 + 10 )  << 2 )  )  | ( 0x1 << 22 )  ) ,
        CLCTL_CLUSTER_VERSION_CHANGED	= ( ( ( ( 0x2 << 0 )  | ( 1 << 20 )  )  | ( ( 0 + 11 )  << 2 )  )  | ( 0x1 << 22 )  ) ,
        CLCTL_FIXUP_ON_UPGRADE	= ( ( ( ( 0x2 << 0 )  | ( 1 << 20 )  )  | ( ( 0 + 12 )  << 2 )  )  | ( 0x1 << 22 )  ) ,
        CLCTL_STARTING_PHASE1	= ( ( ( ( 0x2 << 0 )  | ( 1 << 20 )  )  | ( ( 0 + 13 )  << 2 )  )  | ( 0x1 << 22 )  ) ,
        CLCTL_STARTING_PHASE2	= ( ( ( ( 0x2 << 0 )  | ( 1 << 20 )  )  | ( ( 0 + 14 )  << 2 )  )  | ( 0x1 << 22 )  ) ,
        CLCTL_HOLD_IO	= ( ( ( ( 0x2 << 0 )  | ( 1 << 20 )  )  | ( ( 0 + 15 )  << 2 )  )  | ( 0x1 << 22 )  ) ,
        CLCTL_RESUME_IO	= ( ( ( ( 0x2 << 0 )  | ( 1 << 20 )  )  | ( ( 0 + 16 )  << 2 )  )  | ( 0x1 << 22 )  ) ,
        CLCTL_FORCE_QUORUM	= ( ( ( ( 0x2 << 0 )  | ( 1 << 20 )  )  | ( ( 0 + 17 )  << 2 )  )  | ( 0x1 << 22 )  ) ,
        CLCTL_INITIALIZE	= ( ( ( ( 0x2 << 0 )  | ( 1 << 20 )  )  | ( ( 0 + 18 )  << 2 )  )  | ( 0x1 << 22 )  ) ,
        CLCTL_STATE_CHANGE_REASON	= ( ( ( ( 0x2 << 0 )  | ( 1 << 20 )  )  | ( ( 0 + 19 )  << 2 )  )  | ( 0x1 << 22 )  ) ,
        CLCTL_PROVIDER_STATE_CHANGE	= ( ( ( ( 0x2 << 0 )  | ( 1 << 20 )  )  | ( ( 0 + 20 )  << 2 )  )  | ( 0x1 << 22 )  ) ,
        CLCTL_LEAVING_GROUP	= ( ( ( ( 0x2 << 0 )  | ( 1 << 20 )  )  | ( ( 0 + 21 )  << 2 )  )  | ( 0x1 << 22 )  ) ,
        CLCTL_JOINING_GROUP	= ( ( ( ( 0x2 << 0 )  | ( 1 << 20 )  )  | ( ( 0 + 22 )  << 2 )  )  | ( 0x1 << 22 )  ) ,
        CLCTL_FSWITNESS_GET_EPOCH_INFO	= ( ( ( ( 0x1 << 0 )  | ( 1 << 20 )  )  | ( ( 0 + 23 )  << 2 )  )  | ( 0 << 22 )  ) ,
        CLCTL_FSWITNESS_SET_EPOCH_INFO	= ( ( ( ( 0x2 << 0 )  | ( 1 << 20 )  )  | ( ( 0 + 24 )  << 2 )  )  | ( 0x1 << 22 )  ) ,
        CLCTL_FSWITNESS_RELEASE_LOCK	= ( ( ( ( 0x2 << 0 )  | ( 1 << 20 )  )  | ( ( 0 + 25 )  << 2 )  )  | ( 0x1 << 22 )  ) ,
        CLCTL_NETNAME_CREDS_NOTIFYCAM	= ( ( ( ( 0x2 << 0 )  | ( 1 << 20 )  )  | ( ( 0 + 26 )  << 2 )  )  | ( 0x1 << 22 )  ) ,
        CLCTL_NOTIFY_QUORUM_STATUS	= ( ( ( ( 0x2 << 0 )  | ( 1 << 20 )  )  | ( ( 0 + 31 )  << 2 )  )  | ( 0x1 << 22 )  ) ,
        CLCTL_NOTIFY_MONITOR_SHUTTING_DOWN	= ( ( ( ( 0x1 << 0 )  | ( 1 << 20 )  )  | ( ( 0 + 32 )  << 2 )  )  | ( 0 << 22 )  ) ,
        CLCTL_UNDELETE	= ( ( ( ( 0x2 << 0 )  | ( 1 << 20 )  )  | ( ( 0 + 33 )  << 2 )  )  | ( 0x1 << 22 )  ) ,
        CLCTL_GET_OPERATION_CONTEXT	= ( ( ( ( 0x1 << 0 )  | ( 1 << 20 )  )  | ( ( 0 + 2106 )  << 2 )  )  | ( 0 << 22 )  ) ,
        CLCTL_NOTIFY_OWNER_CHANGE	= ( ( ( ( 0x2 << 0 )  | ( 1 << 20 )  )  | ( ( 0 + 2120 )  << 2 )  )  | ( 0x1 << 22 )  ) ,
        CLCTL_VALIDATE_CHANGE_GROUP	= ( ( ( ( 0x1 << 0 )  | ( 1 << 20 )  )  | ( ( 0 + 2121 )  << 2 )  )  | ( 0 << 22 )  ) ,
        CLCTL_CHECK_DRAIN_VETO	= ( ( ( ( 0x1 << 0 )  | ( 1 << 20 )  )  | ( ( 0 + 2123 )  << 2 )  )  | ( 0 << 22 )  ) ,
        CLCTL_NOTIFY_DRAIN_COMPLETE	= ( ( ( ( 0x1 << 0 )  | ( 1 << 20 )  )  | ( ( 0 + 2124 )  << 2 )  )  | ( 0 << 22 )  ) 
    } 	CLCTL_CODES;

typedef 
enum CLUSCTL_RESOURCE_CODES
    {
        CLUSCTL_RESOURCE_UNKNOWN	= ( ( CLUS_OBJECT_RESOURCE << 24 )  | CLCTL_UNKNOWN ) ,
        CLUSCTL_RESOURCE_GET_CHARACTERISTICS	= ( ( CLUS_OBJECT_RESOURCE << 24 )  | CLCTL_GET_CHARACTERISTICS ) ,
        CLUSCTL_RESOURCE_GET_FLAGS	= ( ( CLUS_OBJECT_RESOURCE << 24 )  | CLCTL_GET_FLAGS ) ,
        CLUSCTL_RESOURCE_GET_CLASS_INFO	= ( ( CLUS_OBJECT_RESOURCE << 24 )  | CLCTL_GET_CLASS_INFO ) ,
        CLUSCTL_RESOURCE_GET_REQUIRED_DEPENDENCIES	= ( ( CLUS_OBJECT_RESOURCE << 24 )  | CLCTL_GET_REQUIRED_DEPENDENCIES ) ,
        CLUSCTL_RESOURCE_GET_NAME	= ( ( CLUS_OBJECT_RESOURCE << 24 )  | CLCTL_GET_NAME ) ,
        CLUSCTL_RESOURCE_GET_ID	= ( ( CLUS_OBJECT_RESOURCE << 24 )  | CLCTL_GET_ID ) ,
        CLUSCTL_RESOURCE_GET_RESOURCE_TYPE	= ( ( CLUS_OBJECT_RESOURCE << 24 )  | CLCTL_GET_RESOURCE_TYPE ) ,
        CLUSCTL_RESOURCE_ENUM_COMMON_PROPERTIES	= ( ( CLUS_OBJECT_RESOURCE << 24 )  | CLCTL_ENUM_COMMON_PROPERTIES ) ,
        CLUSCTL_RESOURCE_GET_RO_COMMON_PROPERTIES	= ( ( CLUS_OBJECT_RESOURCE << 24 )  | CLCTL_GET_RO_COMMON_PROPERTIES ) ,
        CLUSCTL_RESOURCE_GET_COMMON_PROPERTIES	= ( ( CLUS_OBJECT_RESOURCE << 24 )  | CLCTL_GET_COMMON_PROPERTIES ) ,
        CLUSCTL_RESOURCE_SET_COMMON_PROPERTIES	= ( ( CLUS_OBJECT_RESOURCE << 24 )  | CLCTL_SET_COMMON_PROPERTIES ) ,
        CLUSCTL_RESOURCE_VALIDATE_COMMON_PROPERTIES	= ( ( CLUS_OBJECT_RESOURCE << 24 )  | CLCTL_VALIDATE_COMMON_PROPERTIES ) ,
        CLUSCTL_RESOURCE_GET_COMMON_PROPERTY_FMTS	= ( ( CLUS_OBJECT_RESOURCE << 24 )  | CLCTL_GET_COMMON_PROPERTY_FMTS ) ,
        CLUSCTL_RESOURCE_ENUM_PRIVATE_PROPERTIES	= ( ( CLUS_OBJECT_RESOURCE << 24 )  | CLCTL_ENUM_PRIVATE_PROPERTIES ) ,
        CLUSCTL_RESOURCE_GET_RO_PRIVATE_PROPERTIES	= ( ( CLUS_OBJECT_RESOURCE << 24 )  | CLCTL_GET_RO_PRIVATE_PROPERTIES ) ,
        CLUSCTL_RESOURCE_GET_PRIVATE_PROPERTIES	= ( ( CLUS_OBJECT_RESOURCE << 24 )  | CLCTL_GET_PRIVATE_PROPERTIES ) ,
        CLUSCTL_RESOURCE_SET_PRIVATE_PROPERTIES	= ( ( CLUS_OBJECT_RESOURCE << 24 )  | CLCTL_SET_PRIVATE_PROPERTIES ) ,
        CLUSCTL_RESOURCE_VALIDATE_PRIVATE_PROPERTIES	= ( ( CLUS_OBJECT_RESOURCE << 24 )  | CLCTL_VALIDATE_PRIVATE_PROPERTIES ) ,
        CLUSCTL_RESOURCE_GET_PRIVATE_PROPERTY_FMTS	= ( ( CLUS_OBJECT_RESOURCE << 24 )  | CLCTL_GET_PRIVATE_PROPERTY_FMTS ) ,
        CLUSCTL_RESOURCE_ADD_REGISTRY_CHECKPOINT	= ( ( CLUS_OBJECT_RESOURCE << 24 )  | CLCTL_ADD_REGISTRY_CHECKPOINT ) ,
        CLUSCTL_RESOURCE_DELETE_REGISTRY_CHECKPOINT	= ( ( CLUS_OBJECT_RESOURCE << 24 )  | CLCTL_DELETE_REGISTRY_CHECKPOINT ) ,
        CLUSCTL_RESOURCE_GET_REGISTRY_CHECKPOINTS	= ( ( CLUS_OBJECT_RESOURCE << 24 )  | CLCTL_GET_REGISTRY_CHECKPOINTS ) ,
        CLUSCTL_RESOURCE_ADD_CRYPTO_CHECKPOINT	= ( ( CLUS_OBJECT_RESOURCE << 24 )  | CLCTL_ADD_CRYPTO_CHECKPOINT ) ,
        CLUSCTL_RESOURCE_DELETE_CRYPTO_CHECKPOINT	= ( ( CLUS_OBJECT_RESOURCE << 24 )  | CLCTL_DELETE_CRYPTO_CHECKPOINT ) ,
        CLUSCTL_RESOURCE_ADD_CRYPTO_CHECKPOINT_EX	= ( ( CLUS_OBJECT_RESOURCE << 24 )  | CLCTL_ADD_CRYPTO_CHECKPOINT_EX ) ,
        CLUSCTL_RESOURCE_GET_CRYPTO_CHECKPOINTS	= ( ( CLUS_OBJECT_RESOURCE << 24 )  | CLCTL_GET_CRYPTO_CHECKPOINTS ) ,
        CLUSCTL_RESOURCE_GET_LOADBAL_PROCESS_LIST	= ( ( CLUS_OBJECT_RESOURCE << 24 )  | CLCTL_GET_LOADBAL_PROCESS_LIST ) ,
        CLUSCTL_RESOURCE_GET_NETWORK_NAME	= ( ( CLUS_OBJECT_RESOURCE << 24 )  | CLCTL_GET_NETWORK_NAME ) ,
        CLUSCTL_RESOURCE_NETNAME_GET_VIRTUAL_SERVER_TOKEN	= ( ( CLUS_OBJECT_RESOURCE << 24 )  | CLCTL_NETNAME_GET_VIRTUAL_SERVER_TOKEN ) ,
        CLUSCTL_RESOURCE_NETNAME_SET_PWD_INFO	= ( ( CLUS_OBJECT_RESOURCE << 24 )  | CLCTL_NETNAME_SET_PWD_INFO ) ,
        CLUSCTL_RESOURCE_NETNAME_SET_PWD_INFOEX	= ( ( CLUS_OBJECT_RESOURCE << 24 )  | CLCTL_NETNAME_SET_PWD_INFOEX ) ,
        CLUSCTL_RESOURCE_NETNAME_DELETE_CO	= ( ( CLUS_OBJECT_RESOURCE << 24 )  | CLCTL_NETNAME_DELETE_CO ) ,
        CLUSCTL_RESOURCE_NETNAME_VALIDATE_VCO	= ( ( CLUS_OBJECT_RESOURCE << 24 )  | CLCTL_NETNAME_VALIDATE_VCO ) ,
        CLUSCTL_RESOURCE_NETNAME_RESET_VCO	= ( ( CLUS_OBJECT_RESOURCE << 24 )  | CLCTL_NETNAME_RESET_VCO ) ,
        CLUSCTL_RESOURCE_NETNAME_REPAIR_VCO	= ( ( CLUS_OBJECT_RESOURCE << 24 )  | CLCTL_NETNAME_REPAIR_VCO ) ,
        CLUSCTL_RESOURCE_NETNAME_REGISTER_DNS_RECORDS	= ( ( CLUS_OBJECT_RESOURCE << 24 )  | CLCTL_NETNAME_REGISTER_DNS_RECORDS ) ,
        CLUSCTL_RESOURCE_GET_DNS_NAME	= ( ( CLUS_OBJECT_RESOURCE << 24 )  | CLCTL_GET_DNS_NAME ) ,
        CLUSCTL_RESOURCE_STORAGE_GET_DISK_INFO	= ( ( CLUS_OBJECT_RESOURCE << 24 )  | CLCTL_STORAGE_GET_DISK_INFO ) ,
        CLUSCTL_RESOURCE_STORAGE_GET_DISK_NUMBER_INFO	= ( ( CLUS_OBJECT_RESOURCE << 24 )  | CLCTL_STORAGE_GET_DISK_NUMBER_INFO ) ,
        CLUSCTL_RESOURCE_STORAGE_IS_PATH_VALID	= ( ( CLUS_OBJECT_RESOURCE << 24 )  | CLCTL_STORAGE_IS_PATH_VALID ) ,
        CLUSCTL_RESOURCE_QUERY_DELETE	= ( ( CLUS_OBJECT_RESOURCE << 24 )  | CLCTL_QUERY_DELETE ) ,
        CLUSCTL_RESOURCE_UPGRADE_DLL	= ( ( CLUS_OBJECT_RESOURCE << 24 )  | CLCTL_RESOURCE_UPGRADE_DLL ) ,
        CLUSCTL_RESOURCE_IPADDRESS_RENEW_LEASE	= ( ( CLUS_OBJECT_RESOURCE << 24 )  | CLCTL_IPADDRESS_RENEW_LEASE ) ,
        CLUSCTL_RESOURCE_IPADDRESS_RELEASE_LEASE	= ( ( CLUS_OBJECT_RESOURCE << 24 )  | CLCTL_IPADDRESS_RELEASE_LEASE ) ,
        CLUSCTL_RESOURCE_ADD_REGISTRY_CHECKPOINT_64BIT	= ( ( CLUS_OBJECT_RESOURCE << 24 )  | CLCTL_ADD_REGISTRY_CHECKPOINT_64BIT ) ,
        CLUSCTL_RESOURCE_ADD_REGISTRY_CHECKPOINT_32BIT	= ( ( CLUS_OBJECT_RESOURCE << 24 )  | CLCTL_ADD_REGISTRY_CHECKPOINT_32BIT ) ,
        CLUSCTL_RESOURCE_QUERY_MAINTENANCE_MODE	= ( ( CLUS_OBJECT_RESOURCE << 24 )  | CLCTL_QUERY_MAINTENANCE_MODE ) ,
        CLUSCTL_RESOURCE_SET_MAINTENANCE_MODE	= ( ( CLUS_OBJECT_RESOURCE << 24 )  | CLCTL_SET_MAINTENANCE_MODE ) ,
        CLUSCTL_RESOURCE_STORAGE_SET_DRIVELETTER	= ( ( CLUS_OBJECT_RESOURCE << 24 )  | CLCTL_STORAGE_SET_DRIVELETTER ) ,
        CLUSCTL_RESOURCE_STORAGE_GET_DISK_INFO_EX	= ( ( CLUS_OBJECT_RESOURCE << 24 )  | CLCTL_STORAGE_GET_DISK_INFO_EX ) ,
        CLUSCTL_RESOURCE_STORAGE_GET_DISK_INFO_EX2	= ( ( CLUS_OBJECT_RESOURCE << 24 )  | CLCTL_STORAGE_GET_DISK_INFO_EX2 ) ,
        CLUSCTL_RESOURCE_STORAGE_GET_MOUNTPOINTS	= ( ( CLUS_OBJECT_RESOURCE << 24 )  | CLCTL_STORAGE_GET_MOUNTPOINTS ) ,
        CLUSCTL_RESOURCE_STORAGE_GET_DIRTY	= ( ( CLUS_OBJECT_RESOURCE << 24 )  | CLCTL_STORAGE_GET_DIRTY ) ,
        CLUSCTL_RESOURCE_STORAGE_GET_SHARED_VOLUME_INFO	= ( ( CLUS_OBJECT_RESOURCE << 24 )  | CLCTL_STORAGE_GET_SHARED_VOLUME_INFO ) ,
        CLUSCTL_RESOURCE_SET_CSV_MAINTENANCE_MODE	= ( ( CLUS_OBJECT_RESOURCE << 24 )  | CLCTL_SET_CSV_MAINTENANCE_MODE ) ,
        CLUSCTL_RESOURCE_ENABLE_SHARED_VOLUME_DIRECTIO	= ( ( CLUS_OBJECT_RESOURCE << 24 )  | CLCTL_ENABLE_SHARED_VOLUME_DIRECTIO ) ,
        CLUSCTL_RESOURCE_DISABLE_SHARED_VOLUME_DIRECTIO	= ( ( CLUS_OBJECT_RESOURCE << 24 )  | CLCTL_DISABLE_SHARED_VOLUME_DIRECTIO ) ,
        CLUSCTL_RESOURCE_SET_SHARED_VOLUME_BACKUP_MODE	= ( ( CLUS_OBJECT_RESOURCE << 24 )  | CLCTL_SET_SHARED_VOLUME_BACKUP_MODE ) ,
        CLUSCTL_RESOURCE_STORAGE_GET_SHARED_VOLUME_PARTITION_NAMES	= ( ( CLUS_OBJECT_RESOURCE << 24 )  | CLCTL_STORAGE_GET_SHARED_VOLUME_PARTITION_NAMES ) ,
        CLUSCTL_RESOURCE_GET_FAILURE_INFO	= ( ( CLUS_OBJECT_RESOURCE << 24 )  | CLCTL_GET_FAILURE_INFO ) ,
        CLUSCTL_RESOURCE_STORAGE_GET_DISKID	= ( ( CLUS_OBJECT_RESOURCE << 24 )  | CLCTL_STORAGE_GET_DISKID ) ,
        CLUSCTL_RESOURCE_STORAGE_GET_SHARED_VOLUME_STATES	= ( ( CLUS_OBJECT_RESOURCE << 24 )  | CLCTL_STORAGE_GET_SHARED_VOLUME_STATES ) ,
        CLUSCTL_RESOURCE_STORAGE_IS_SHARED_VOLUME	= ( ( CLUS_OBJECT_RESOURCE << 24 )  | CLCTL_STORAGE_IS_SHARED_VOLUME ) ,
        CLUSCTL_RESOURCE_IS_QUORUM_BLOCKED	= ( ( CLUS_OBJECT_RESOURCE << 24 )  | CLCTL_IS_QUORUM_BLOCKED ) ,
        CLUSCTL_RESOURCE_POOL_GET_DRIVE_INFO	= ( ( CLUS_OBJECT_RESOURCE << 24 )  | CLCTL_POOL_GET_DRIVE_INFO ) ,
        CLUSCTL_RESOURCE_RLUA_GET_VIRTUAL_SERVER_TOKEN	= ( ( CLUS_OBJECT_RESOURCE << 24 )  | CLCTL_NETNAME_GET_VIRTUAL_SERVER_TOKEN ) ,
        CLUSCTL_RESOURCE_RLUA_SET_PWD_INFO	= ( ( CLUS_OBJECT_RESOURCE << 24 )  | CLCTL_NETNAME_SET_PWD_INFO ) ,
        CLUSCTL_RESOURCE_RLUA_SET_PWD_INFOEX	= ( ( CLUS_OBJECT_RESOURCE << 24 )  | CLCTL_NETNAME_SET_PWD_INFOEX ) ,
        CLUSCTL_RESOURCE_DELETE	= ( ( CLUS_OBJECT_RESOURCE << 24 )  | CLCTL_DELETE ) ,
        CLUSCTL_RESOURCE_UNDELETE	= ( ( CLUS_OBJECT_RESOURCE << 24 )  | CLCTL_UNDELETE ) ,
        CLUSCTL_RESOURCE_INSTALL_NODE	= ( ( CLUS_OBJECT_RESOURCE << 24 )  | CLCTL_INSTALL_NODE ) ,
        CLUSCTL_RESOURCE_EVICT_NODE	= ( ( CLUS_OBJECT_RESOURCE << 24 )  | CLCTL_EVICT_NODE ) ,
        CLUSCTL_RESOURCE_ADD_DEPENDENCY	= ( ( CLUS_OBJECT_RESOURCE << 24 )  | CLCTL_ADD_DEPENDENCY ) ,
        CLUSCTL_RESOURCE_REMOVE_DEPENDENCY	= ( ( CLUS_OBJECT_RESOURCE << 24 )  | CLCTL_REMOVE_DEPENDENCY ) ,
        CLUSCTL_RESOURCE_ADD_OWNER	= ( ( CLUS_OBJECT_RESOURCE << 24 )  | CLCTL_ADD_OWNER ) ,
        CLUSCTL_RESOURCE_REMOVE_OWNER	= ( ( CLUS_OBJECT_RESOURCE << 24 )  | CLCTL_REMOVE_OWNER ) ,
        CLUSCTL_RESOURCE_SET_NAME	= ( ( CLUS_OBJECT_RESOURCE << 24 )  | CLCTL_SET_NAME ) ,
        CLUSCTL_RESOURCE_CLUSTER_NAME_CHANGED	= ( ( CLUS_OBJECT_RESOURCE << 24 )  | CLCTL_CLUSTER_NAME_CHANGED ) ,
        CLUSCTL_RESOURCE_CLUSTER_VERSION_CHANGED	= ( ( CLUS_OBJECT_RESOURCE << 24 )  | CLCTL_CLUSTER_VERSION_CHANGED ) ,
        CLUSCTL_RESOURCE_FORCE_QUORUM	= ( ( CLUS_OBJECT_RESOURCE << 24 )  | CLCTL_FORCE_QUORUM ) ,
        CLUSCTL_RESOURCE_INITIALIZE	= ( ( CLUS_OBJECT_RESOURCE << 24 )  | CLCTL_INITIALIZE ) ,
        CLUSCTL_RESOURCE_STATE_CHANGE_REASON	= ( ( CLUS_OBJECT_RESOURCE << 24 )  | CLCTL_STATE_CHANGE_REASON ) ,
        CLUSCTL_RESOURCE_PROVIDER_STATE_CHANGE	= ( ( CLUS_OBJECT_RESOURCE << 24 )  | CLCTL_PROVIDER_STATE_CHANGE ) ,
        CLUSCTL_RESOURCE_LEAVING_GROUP	= ( ( CLUS_OBJECT_RESOURCE << 24 )  | CLCTL_LEAVING_GROUP ) ,
        CLUSCTL_RESOURCE_JOINING_GROUP	= ( ( CLUS_OBJECT_RESOURCE << 24 )  | CLCTL_JOINING_GROUP ) ,
        CLUSCTL_RESOURCE_FSWITNESS_GET_EPOCH_INFO	= ( ( CLUS_OBJECT_RESOURCE << 24 )  | CLCTL_FSWITNESS_GET_EPOCH_INFO ) ,
        CLUSCTL_RESOURCE_FSWITNESS_SET_EPOCH_INFO	= ( ( CLUS_OBJECT_RESOURCE << 24 )  | CLCTL_FSWITNESS_SET_EPOCH_INFO ) ,
        CLUSCTL_RESOURCE_FSWITNESS_RELEASE_LOCK	= ( ( CLUS_OBJECT_RESOURCE << 24 )  | CLCTL_FSWITNESS_RELEASE_LOCK ) ,
        CLUSCTL_RESOURCE_NETNAME_CREDS_NOTIFYCAM	= ( ( CLUS_OBJECT_RESOURCE << 24 )  | CLCTL_NETNAME_CREDS_NOTIFYCAM ) ,
        CLUSCTL_RESOURCE_GET_OPERATION_CONTEXT	= ( ( CLUS_OBJECT_RESOURCE << 24 )  | CLCTL_GET_OPERATION_CONTEXT ) ,
        CLUSCTL_RESOURCE_RW_MODIFY_NOOP	= ( ( CLUS_OBJECT_RESOURCE << 24 )  | CLCTL_RW_MODIFY_NOOP ) ,
        CLUSCTL_RESOURCE_NOTIFY_QUORUM_STATUS	= ( ( CLUS_OBJECT_RESOURCE << 24 )  | CLCTL_NOTIFY_QUORUM_STATUS ) ,
        CLUSCTL_RESOURCE_NOTIFY_OWNER_CHANGE	= ( ( CLUS_OBJECT_RESOURCE << 24 )  | CLCTL_NOTIFY_OWNER_CHANGE ) ,
        CLUSCTL_RESOURCE_VALIDATE_CHANGE_GROUP	= ( ( CLUS_OBJECT_RESOURCE << 24 )  | CLCTL_VALIDATE_CHANGE_GROUP ) ,
        CLUSCTL_RESOURCE_STORAGE_RENAME_SHARED_VOLUME	= ( ( CLUS_OBJECT_RESOURCE << 24 )  | CLCTL_STORAGE_RENAME_SHARED_VOLUME ) ,
        CLUSCTL_RESOURCE_STORAGE_RENAME_SHARED_VOLUME_GUID	= ( ( CLUS_OBJECT_RESOURCE << 24 )  | CLCTL_STORAGE_RENAME_SHARED_VOLUME_GUID ) ,
        CLUSCTL_CLOUD_WITNESS_RESOURCE_UPDATE_TOKEN	= ( ( CLUS_OBJECT_RESOURCE << 24 )  | CLCTL_CLOUD_WITNESS_RESOURCE_UPDATE_TOKEN ) ,
        CLUSCTL_CLOUD_WITNESS_RESOURCE_UPDATE_KEY	= ( ( CLUS_OBJECT_RESOURCE << 24 )  | CLCTL_CLOUD_WITNESS_RESOURCE_UPDATE_KEY ) ,
        CLUSCTL_RESOURCE_PREPARE_UPGRADE	= ( ( CLUS_OBJECT_RESOURCE << 24 )  | CLCTL_RESOURCE_PREPARE_UPGRADE ) ,
        CLUSCTL_RESOURCE_UPGRADE_COMPLETED	= ( ( CLUS_OBJECT_RESOURCE << 24 )  | CLCTL_RESOURCE_UPGRADE_COMPLETED ) ,
        CLUSCTL_RESOURCE_GET_STATE_CHANGE_TIME	= ( ( CLUS_OBJECT_RESOURCE << 24 )  | CLCTL_GET_STATE_CHANGE_TIME ) ,
        CLUSCTL_RESOURCE_GET_INFRASTRUCTURE_SOFS_BUFFER	= ( ( CLUS_OBJECT_RESOURCE << 24 )  | CLCTL_GET_INFRASTRUCTURE_SOFS_BUFFER ) ,
        CLUSCTL_RESOURCE_SET_INFRASTRUCTURE_SOFS_BUFFER	= ( ( CLUS_OBJECT_RESOURCE << 24 )  | CLCTL_SET_INFRASTRUCTURE_SOFS_BUFFER ) ,
        CLUSCTL_RESOURCE_SCALEOUT_COMMAND	= ( ( CLUS_OBJECT_RESOURCE << 24 )  | CLCTL_SCALEOUT_COMMAND ) ,
        CLUSCTL_RESOURCE_SCALEOUT_CONTROL	= ( ( CLUS_OBJECT_RESOURCE << 24 )  | CLCTL_SCALEOUT_CONTROL ) ,
        CLUSCTL_RESOURCE_SCALEOUT_GET_CLUSTERS	= ( ( CLUS_OBJECT_RESOURCE << 24 )  | CLCTL_SCALEOUT_GET_CLUSTERS ) ,
        CLUSCTL_RESOURCE_CHECK_DRAIN_VETO	= ( ( CLUS_OBJECT_RESOURCE << 24 )  | CLCTL_CHECK_DRAIN_VETO ) ,
        CLUSCTL_RESOURCE_NOTIFY_DRAIN_COMPLETE	= ( ( CLUS_OBJECT_RESOURCE << 24 )  | CLCTL_NOTIFY_DRAIN_COMPLETE ) ,
        CLUSCTL_RESOURCE_GET_NODES_IN_FD	= ( ( CLUS_OBJECT_RESOURCE << 24 )  | CLCTL_GET_NODES_IN_FD ) 
    } 	CLUSCTL_RESOURCE_CODES;

typedef 
enum CLUSCTL_RESOURCE_TYPE_CODES
    {
        CLUSCTL_RESOURCE_TYPE_UNKNOWN	= ( ( CLUS_OBJECT_RESOURCE_TYPE << 24 )  | CLCTL_UNKNOWN ) ,
        CLUSCTL_RESOURCE_TYPE_GET_CHARACTERISTICS	= ( ( CLUS_OBJECT_RESOURCE_TYPE << 24 )  | CLCTL_GET_CHARACTERISTICS ) ,
        CLUSCTL_RESOURCE_TYPE_GET_FLAGS	= ( ( CLUS_OBJECT_RESOURCE_TYPE << 24 )  | CLCTL_GET_FLAGS ) ,
        CLUSCTL_RESOURCE_TYPE_GET_CLASS_INFO	= ( ( CLUS_OBJECT_RESOURCE_TYPE << 24 )  | CLCTL_GET_CLASS_INFO ) ,
        CLUSCTL_RESOURCE_TYPE_GET_REQUIRED_DEPENDENCIES	= ( ( CLUS_OBJECT_RESOURCE_TYPE << 24 )  | CLCTL_GET_REQUIRED_DEPENDENCIES ) ,
        CLUSCTL_RESOURCE_TYPE_GET_ARB_TIMEOUT	= ( ( CLUS_OBJECT_RESOURCE_TYPE << 24 )  | CLCTL_GET_ARB_TIMEOUT ) ,
        CLUSCTL_RESOURCE_TYPE_ENUM_COMMON_PROPERTIES	= ( ( CLUS_OBJECT_RESOURCE_TYPE << 24 )  | CLCTL_ENUM_COMMON_PROPERTIES ) ,
        CLUSCTL_RESOURCE_TYPE_GET_RO_COMMON_PROPERTIES	= ( ( CLUS_OBJECT_RESOURCE_TYPE << 24 )  | CLCTL_GET_RO_COMMON_PROPERTIES ) ,
        CLUSCTL_RESOURCE_TYPE_GET_COMMON_PROPERTIES	= ( ( CLUS_OBJECT_RESOURCE_TYPE << 24 )  | CLCTL_GET_COMMON_PROPERTIES ) ,
        CLUSCTL_RESOURCE_TYPE_VALIDATE_COMMON_PROPERTIES	= ( ( CLUS_OBJECT_RESOURCE_TYPE << 24 )  | CLCTL_VALIDATE_COMMON_PROPERTIES ) ,
        CLUSCTL_RESOURCE_TYPE_SET_COMMON_PROPERTIES	= ( ( CLUS_OBJECT_RESOURCE_TYPE << 24 )  | CLCTL_SET_COMMON_PROPERTIES ) ,
        CLUSCTL_RESOURCE_TYPE_GET_COMMON_PROPERTY_FMTS	= ( ( CLUS_OBJECT_RESOURCE_TYPE << 24 )  | CLCTL_GET_COMMON_PROPERTY_FMTS ) ,
        CLUSCTL_RESOURCE_TYPE_GET_COMMON_RESOURCE_PROPERTY_FMTS	= ( ( CLUS_OBJECT_RESOURCE_TYPE << 24 )  | CLCTL_GET_COMMON_RESOURCE_PROPERTY_FMTS ) ,
        CLUSCTL_RESOURCE_TYPE_ENUM_PRIVATE_PROPERTIES	= ( ( CLUS_OBJECT_RESOURCE_TYPE << 24 )  | CLCTL_ENUM_PRIVATE_PROPERTIES ) ,
        CLUSCTL_RESOURCE_TYPE_GET_RO_PRIVATE_PROPERTIES	= ( ( CLUS_OBJECT_RESOURCE_TYPE << 24 )  | CLCTL_GET_RO_PRIVATE_PROPERTIES ) ,
        CLUSCTL_RESOURCE_TYPE_GET_PRIVATE_PROPERTIES	= ( ( CLUS_OBJECT_RESOURCE_TYPE << 24 )  | CLCTL_GET_PRIVATE_PROPERTIES ) ,
        CLUSCTL_RESOURCE_TYPE_SET_PRIVATE_PROPERTIES	= ( ( CLUS_OBJECT_RESOURCE_TYPE << 24 )  | CLCTL_SET_PRIVATE_PROPERTIES ) ,
        CLUSCTL_RESOURCE_TYPE_VALIDATE_PRIVATE_PROPERTIES	= ( ( CLUS_OBJECT_RESOURCE_TYPE << 24 )  | CLCTL_VALIDATE_PRIVATE_PROPERTIES ) ,
        CLUSCTL_RESOURCE_TYPE_GET_PRIVATE_PROPERTY_FMTS	= ( ( CLUS_OBJECT_RESOURCE_TYPE << 24 )  | CLCTL_GET_PRIVATE_PROPERTY_FMTS ) ,
        CLUSCTL_RESOURCE_TYPE_GET_PRIVATE_RESOURCE_PROPERTY_FMTS	= ( ( CLUS_OBJECT_RESOURCE_TYPE << 24 )  | CLCTL_GET_PRIVATE_RESOURCE_PROPERTY_FMTS ) ,
        CLUSCTL_RESOURCE_TYPE_GET_REGISTRY_CHECKPOINTS	= ( ( CLUS_OBJECT_RESOURCE_TYPE << 24 )  | CLCTL_GET_REGISTRY_CHECKPOINTS ) ,
        CLUSCTL_RESOURCE_TYPE_GET_CRYPTO_CHECKPOINTS	= ( ( CLUS_OBJECT_RESOURCE_TYPE << 24 )  | CLCTL_GET_CRYPTO_CHECKPOINTS ) ,
        CLUSCTL_RESOURCE_TYPE_STORAGE_GET_AVAILABLE_DISKS	= ( ( CLUS_OBJECT_RESOURCE_TYPE << 24 )  | CLCTL_STORAGE_GET_AVAILABLE_DISKS ) ,
        CLUSCTL_RESOURCE_TYPE_STORAGE_SYNC_CLUSDISK_DB	= ( ( CLUS_OBJECT_RESOURCE_TYPE << 24 )  | CLCTL_STORAGE_SYNC_CLUSDISK_DB ) ,
        CLUSCTL_RESOURCE_TYPE_NETNAME_VALIDATE_NETNAME	= ( ( CLUS_OBJECT_RESOURCE_TYPE << 24 )  | CLCTL_VALIDATE_NETNAME ) ,
        CLUSCTL_RESOURCE_TYPE_NETNAME_GET_OU_FOR_VCO	= ( ( CLUS_OBJECT_RESOURCE_TYPE << 24 )  | CLCTL_NETNAME_GET_OU_FOR_VCO ) ,
        CLUSCTL_RESOURCE_TYPE_GEN_APP_VALIDATE_PATH	= ( ( CLUS_OBJECT_RESOURCE_TYPE << 24 )  | CLCTL_VALIDATE_PATH ) ,
        CLUSCTL_RESOURCE_TYPE_GEN_APP_VALIDATE_DIRECTORY	= ( ( CLUS_OBJECT_RESOURCE_TYPE << 24 )  | CLCTL_VALIDATE_DIRECTORY ) ,
        CLUSCTL_RESOURCE_TYPE_GEN_SCRIPT_VALIDATE_PATH	= ( ( CLUS_OBJECT_RESOURCE_TYPE << 24 )  | CLCTL_VALIDATE_PATH ) ,
        CLUSCTL_RESOURCE_TYPE_QUERY_DELETE	= ( ( CLUS_OBJECT_RESOURCE_TYPE << 24 )  | CLCTL_QUERY_DELETE ) ,
        CLUSCTL_RESOURCE_TYPE_STORAGE_GET_DRIVELETTERS	= ( ( CLUS_OBJECT_RESOURCE_TYPE << 24 )  | CLCTL_STORAGE_GET_DRIVELETTERS ) ,
        CLUSCTL_RESOURCE_TYPE_STORAGE_GET_AVAILABLE_DISKS_EX	= ( ( CLUS_OBJECT_RESOURCE_TYPE << 24 )  | CLCTL_STORAGE_GET_AVAILABLE_DISKS_EX ) ,
        CLUSCTL_RESOURCE_TYPE_STORAGE_REMAP_DRIVELETTER	= ( ( CLUS_OBJECT_RESOURCE_TYPE << 24 )  | CLCTL_STORAGE_REMAP_DRIVELETTER ) ,
        CLUSCTL_RESOURCE_TYPE_STORAGE_GET_DISKID	= ( ( CLUS_OBJECT_RESOURCE_TYPE << 24 )  | CLCTL_STORAGE_GET_DISKID ) ,
        CLUSCTL_RESOURCE_TYPE_STORAGE_GET_RESOURCEID	= ( ( CLUS_OBJECT_RESOURCE_TYPE << 24 )  | CLCTL_STORAGE_GET_RESOURCEID ) ,
        CLUSCTL_RESOURCE_TYPE_STORAGE_IS_CLUSTERABLE	= ( ( CLUS_OBJECT_RESOURCE_TYPE << 24 )  | CLCTL_STORAGE_IS_CLUSTERABLE ) ,
        CLUSCTL_RESOURCE_TYPE_STORAGE_REMOVE_VM_OWNERSHIP	= ( ( CLUS_OBJECT_RESOURCE_TYPE << 24 )  | CLCTL_STORAGE_REMOVE_VM_OWNERSHIP ) ,
        CLUSCTL_RESOURCE_TYPE_STORAGE_IS_CSV_FILE	= ( ( CLUS_OBJECT_RESOURCE << 24 )  | CLCTL_STORAGE_IS_CSV_FILE ) ,
        CLUSCTL_RESOURCE_TYPE_WITNESS_VALIDATE_PATH	= ( ( CLUS_OBJECT_RESOURCE_TYPE << 24 )  | CLCTL_VALIDATE_PATH ) ,
        CLUSCTL_RESOURCE_TYPE_INSTALL_NODE	= ( ( CLUS_OBJECT_RESOURCE_TYPE << 24 )  | CLCTL_INSTALL_NODE ) ,
        CLUSCTL_RESOURCE_TYPE_EVICT_NODE	= ( ( CLUS_OBJECT_RESOURCE_TYPE << 24 )  | CLCTL_EVICT_NODE ) ,
        CLUSCTL_RESOURCE_TYPE_CLUSTER_VERSION_CHANGED	= ( ( CLUS_OBJECT_RESOURCE_TYPE << 24 )  | CLCTL_CLUSTER_VERSION_CHANGED ) ,
        CLUSCTL_RESOURCE_TYPE_FIXUP_ON_UPGRADE	= ( ( CLUS_OBJECT_RESOURCE_TYPE << 24 )  | CLCTL_FIXUP_ON_UPGRADE ) ,
        CLUSCTL_RESOURCE_TYPE_STARTING_PHASE1	= ( ( CLUS_OBJECT_RESOURCE_TYPE << 24 )  | CLCTL_STARTING_PHASE1 ) ,
        CLUSCTL_RESOURCE_TYPE_STARTING_PHASE2	= ( ( CLUS_OBJECT_RESOURCE_TYPE << 24 )  | CLCTL_STARTING_PHASE2 ) ,
        CLUSCTL_RESOURCE_TYPE_HOLD_IO	= ( ( CLUS_OBJECT_RESOURCE_TYPE << 24 )  | CLCTL_HOLD_IO ) ,
        CLUSCTL_RESOURCE_TYPE_RESUME_IO	= ( ( CLUS_OBJECT_RESOURCE_TYPE << 24 )  | CLCTL_RESUME_IO ) ,
        CLUSCTL_RESOURCE_TYPE_STORAGE_GET_AVAILABLE_DISKS_EX2_INT	= ( ( CLUS_OBJECT_RESOURCE_TYPE << 24 )  | CLCTL_STORAGE_GET_AVAILABLE_DISKS_EX2_INT ) ,
        CLUSCTL_RESOURCE_TYPE_REPLICATION_GET_ELIGIBLE_LOGDISKS	= ( ( CLUS_OBJECT_RESOURCE_TYPE << 24 )  | CLCTL_REPLICATION_GET_ELIGIBLE_LOGDISKS ) ,
        CLUSCTL_RESOURCE_TYPE_REPLICATION_GET_ELIGIBLE_TARGET_DATADISKS	= ( ( CLUS_OBJECT_RESOURCE_TYPE << 24 )  | CLCTL_REPLICATION_GET_ELIGIBLE_TARGET_DATADISKS ) ,
        CLUSCTL_RESOURCE_TYPE_REPLICATION_GET_ELIGIBLE_SOURCE_DATADISKS	= ( ( CLUS_OBJECT_RESOURCE_TYPE << 24 )  | CLCTL_REPLICATION_GET_ELIGIBLE_SOURCE_DATADISKS ) ,
        CLUSCTL_RESOURCE_TYPE_REPLICATION_GET_REPLICATED_DISKS	= ( ( CLUS_OBJECT_RESOURCE_TYPE << 24 )  | CLCTL_REPLICATION_GET_REPLICATED_DISKS ) ,
        CLUSCTL_RESOURCE_TYPE_REPLICATION_GET_REPLICA_VOLUMES	= ( ( CLUS_OBJECT_RESOURCE_TYPE << 24 )  | CLCTL_REPLICATION_GET_REPLICA_VOLUMES ) ,
        CLUSCTL_RESOURCE_TYPE_REPLICATION_GET_LOG_VOLUME	= ( ( CLUS_OBJECT_RESOURCE_TYPE << 24 )  | CLCTL_REPLICATION_GET_LOG_VOLUME ) ,
        CLUSCTL_RESOURCE_TYPE_REPLICATION_GET_RESOURCE_GROUP	= ( ( CLUS_OBJECT_RESOURCE_TYPE << 24 )  | CLCTL_REPLICATION_GET_RESOURCE_GROUP ) ,
        CLUSCTL_RESOURCE_TYPE_REPLICATION_GET_REPLICATED_PARTITION_INFO	= ( ( CLUS_OBJECT_RESOURCE_TYPE << 24 )  | CLCTL_REPLICATION_GET_REPLICATED_PARTITION_INFO ) ,
        CLUSCTL_RESOURCE_TYPE_REPLICATION_GET_LOG_INFO	= ( ( CLUS_OBJECT_RESOURCE_TYPE << 24 )  | CLCTL_REPLICATION_GET_LOG_INFO ) ,
        CLUSCTL_RESOURCE_TYPE_REPLICATION_ADD_REPLICATION_GROUP	= ( ( CLUS_OBJECT_RESOURCE_TYPE << 24 )  | CLCTL_REPLICATION_ADD_REPLICATION_GROUP ) ,
        CLUSCTL_CLOUD_WITNESS_RESOURCE_TYPE_VALIDATE_CREDENTIALS	= ( ( CLUS_OBJECT_RESOURCE_TYPE << 24 )  | CLCTL_CLOUD_WITNESS_RESOURCE_TYPE_VALIDATE_CREDENTIALS ) ,
        CLUSCTL_CLOUD_WITNESS_RESOURCE_TYPE_VALIDATE_CREDENTIALS_WITH_KEY	= ( ( CLUS_OBJECT_RESOURCE_TYPE << 24 )  | CLCTL_CLOUD_WITNESS_RESOURCE_TYPE_VALIDATE_CREDENTIALS_WITH_KEY ) ,
        CLUSCTL_RESOURCE_TYPE_PREPARE_UPGRADE	= ( ( CLUS_OBJECT_RESOURCE_TYPE << 24 )  | CLCTL_RESOURCE_PREPARE_UPGRADE ) ,
        CLUSCTL_RESOURCE_TYPE_UPGRADE_COMPLETED	= ( ( CLUS_OBJECT_RESOURCE_TYPE << 24 )  | CLCTL_RESOURCE_UPGRADE_COMPLETED ) ,
        CLUSCTL_RESOURCE_TYPE_NOTIFY_MONITOR_SHUTTING_DOWN	= ( ( CLUS_OBJECT_RESOURCE_TYPE << 24 )  | CLCTL_NOTIFY_MONITOR_SHUTTING_DOWN ) ,
        CLUSCTL_RESOURCE_TYPE_CHECK_DRAIN_VETO	= ( ( CLUS_OBJECT_RESOURCE_TYPE << 24 )  | CLCTL_CHECK_DRAIN_VETO ) ,
        CLUSCTL_RESOURCE_TYPE_NOTIFY_DRAIN_COMPLETE	= ( ( CLUS_OBJECT_RESOURCE_TYPE << 24 )  | CLCTL_NOTIFY_DRAIN_COMPLETE ) 
    } 	CLUSCTL_RESOURCE_TYPE_CODES;

typedef struct NodeSriovInfo
    {
    DWORD VFTotal;
    DWORD VFUsed;
    DWORD QPTotal;
    DWORD QPUsed;
    } 	NodeSriovInfo;

typedef 
enum CLUSCTL_GROUP_CODES
    {
        CLUSCTL_GROUP_UNKNOWN	= ( ( CLUS_OBJECT_GROUP << 24 )  | CLCTL_UNKNOWN ) ,
        CLUSCTL_GROUP_GET_CHARACTERISTICS	= ( ( CLUS_OBJECT_GROUP << 24 )  | CLCTL_GET_CHARACTERISTICS ) ,
        CLUSCTL_GROUP_GET_FLAGS	= ( ( CLUS_OBJECT_GROUP << 24 )  | CLCTL_GET_FLAGS ) ,
        CLUSCTL_GROUP_GET_NAME	= ( ( CLUS_OBJECT_GROUP << 24 )  | CLCTL_GET_NAME ) ,
        CLUSCTL_GROUP_GET_ID	= ( ( CLUS_OBJECT_GROUP << 24 )  | CLCTL_GET_ID ) ,
        CLUSCTL_GROUP_ENUM_COMMON_PROPERTIES	= ( ( CLUS_OBJECT_GROUP << 24 )  | CLCTL_ENUM_COMMON_PROPERTIES ) ,
        CLUSCTL_GROUP_GET_RO_COMMON_PROPERTIES	= ( ( CLUS_OBJECT_GROUP << 24 )  | CLCTL_GET_RO_COMMON_PROPERTIES ) ,
        CLUSCTL_GROUP_GET_COMMON_PROPERTIES	= ( ( CLUS_OBJECT_GROUP << 24 )  | CLCTL_GET_COMMON_PROPERTIES ) ,
        CLUSCTL_GROUP_SET_COMMON_PROPERTIES	= ( ( CLUS_OBJECT_GROUP << 24 )  | CLCTL_SET_COMMON_PROPERTIES ) ,
        CLUSCTL_GROUP_VALIDATE_COMMON_PROPERTIES	= ( ( CLUS_OBJECT_GROUP << 24 )  | CLCTL_VALIDATE_COMMON_PROPERTIES ) ,
        CLUSCTL_GROUP_ENUM_PRIVATE_PROPERTIES	= ( ( CLUS_OBJECT_GROUP << 24 )  | CLCTL_ENUM_PRIVATE_PROPERTIES ) ,
        CLUSCTL_GROUP_GET_RO_PRIVATE_PROPERTIES	= ( ( CLUS_OBJECT_GROUP << 24 )  | CLCTL_GET_RO_PRIVATE_PROPERTIES ) ,
        CLUSCTL_GROUP_GET_PRIVATE_PROPERTIES	= ( ( CLUS_OBJECT_GROUP << 24 )  | CLCTL_GET_PRIVATE_PROPERTIES ) ,
        CLUSCTL_GROUP_SET_PRIVATE_PROPERTIES	= ( ( CLUS_OBJECT_GROUP << 24 )  | CLCTL_SET_PRIVATE_PROPERTIES ) ,
        CLUSCTL_GROUP_VALIDATE_PRIVATE_PROPERTIES	= ( ( CLUS_OBJECT_GROUP << 24 )  | CLCTL_VALIDATE_PRIVATE_PROPERTIES ) ,
        CLUSCTL_GROUP_QUERY_DELETE	= ( ( CLUS_OBJECT_GROUP << 24 )  | CLCTL_QUERY_DELETE ) ,
        CLUSCTL_GROUP_GET_COMMON_PROPERTY_FMTS	= ( ( CLUS_OBJECT_GROUP << 24 )  | CLCTL_GET_COMMON_PROPERTY_FMTS ) ,
        CLUSCTL_GROUP_GET_PRIVATE_PROPERTY_FMTS	= ( ( CLUS_OBJECT_GROUP << 24 )  | CLCTL_GET_PRIVATE_PROPERTY_FMTS ) ,
        CLUSCTL_GROUP_GET_FAILURE_INFO	= ( ( CLUS_OBJECT_GROUP << 24 )  | CLCTL_GET_FAILURE_INFO ) ,
        CLUSCTL_GROUP_GET_LAST_MOVE_TIME	= ( ( CLUS_OBJECT_GROUP << 24 )  | CLCTL_GROUP_GET_LAST_MOVE_TIME ) ,
        CLUSCTL_GROUP_SET_CCF_FROM_MASTER	= ( ( CLUS_OBJECT_GROUP << 24 )  | CLCTL_GROUP_SET_CCF_FROM_MASTER ) 
    } 	CLUSCTL_GROUP_CODES;

typedef 
enum CLUSCTL_NODE_CODES
    {
        CLUSCTL_NODE_UNKNOWN	= ( ( CLUS_OBJECT_NODE << 24 )  | CLCTL_UNKNOWN ) ,
        CLUSCTL_NODE_GET_CHARACTERISTICS	= ( ( CLUS_OBJECT_NODE << 24 )  | CLCTL_GET_CHARACTERISTICS ) ,
        CLUSCTL_NODE_GET_FLAGS	= ( ( CLUS_OBJECT_NODE << 24 )  | CLCTL_GET_FLAGS ) ,
        CLUSCTL_NODE_GET_NAME	= ( ( CLUS_OBJECT_NODE << 24 )  | CLCTL_GET_NAME ) ,
        CLUSCTL_NODE_GET_ID	= ( ( CLUS_OBJECT_NODE << 24 )  | CLCTL_GET_ID ) ,
        CLUSCTL_NODE_ENUM_COMMON_PROPERTIES	= ( ( CLUS_OBJECT_NODE << 24 )  | CLCTL_ENUM_COMMON_PROPERTIES ) ,
        CLUSCTL_NODE_GET_RO_COMMON_PROPERTIES	= ( ( CLUS_OBJECT_NODE << 24 )  | CLCTL_GET_RO_COMMON_PROPERTIES ) ,
        CLUSCTL_NODE_GET_COMMON_PROPERTIES	= ( ( CLUS_OBJECT_NODE << 24 )  | CLCTL_GET_COMMON_PROPERTIES ) ,
        CLUSCTL_NODE_SET_COMMON_PROPERTIES	= ( ( CLUS_OBJECT_NODE << 24 )  | CLCTL_SET_COMMON_PROPERTIES ) ,
        CLUSCTL_NODE_VALIDATE_COMMON_PROPERTIES	= ( ( CLUS_OBJECT_NODE << 24 )  | CLCTL_VALIDATE_COMMON_PROPERTIES ) ,
        CLUSCTL_NODE_ENUM_PRIVATE_PROPERTIES	= ( ( CLUS_OBJECT_NODE << 24 )  | CLCTL_ENUM_PRIVATE_PROPERTIES ) ,
        CLUSCTL_NODE_GET_RO_PRIVATE_PROPERTIES	= ( ( CLUS_OBJECT_NODE << 24 )  | CLCTL_GET_RO_PRIVATE_PROPERTIES ) ,
        CLUSCTL_NODE_GET_PRIVATE_PROPERTIES	= ( ( CLUS_OBJECT_NODE << 24 )  | CLCTL_GET_PRIVATE_PROPERTIES ) ,
        CLUSCTL_NODE_SET_PRIVATE_PROPERTIES	= ( ( CLUS_OBJECT_NODE << 24 )  | CLCTL_SET_PRIVATE_PROPERTIES ) ,
        CLUSCTL_NODE_VALIDATE_PRIVATE_PROPERTIES	= ( ( CLUS_OBJECT_NODE << 24 )  | CLCTL_VALIDATE_PRIVATE_PROPERTIES ) ,
        CLUSCTL_NODE_GET_COMMON_PROPERTY_FMTS	= ( ( CLUS_OBJECT_NODE << 24 )  | CLCTL_GET_COMMON_PROPERTY_FMTS ) ,
        CLUSCTL_NODE_GET_PRIVATE_PROPERTY_FMTS	= ( ( CLUS_OBJECT_NODE << 24 )  | CLCTL_GET_PRIVATE_PROPERTY_FMTS ) ,
        CLUSCTL_NODE_GET_CLUSTER_SERVICE_ACCOUNT_NAME	= ( ( CLUS_OBJECT_NODE << 24 )  | CLCTL_GET_CLUSTER_SERVICE_ACCOUNT_NAME ) ,
        CLUSCTL_NODE_GET_STUCK_NODES	= ( ( CLUS_OBJECT_NODE << 24 )  | CLCTL_GET_STUCK_NODES ) ,
        CLUSCTL_NODE_INJECT_GEM_FAULT	= ( ( CLUS_OBJECT_NODE << 24 )  | CLCTL_INJECT_GEM_FAULT ) ,
        CLUSCTL_NODE_INTRODUCE_GEM_REPAIR_DELAY	= ( ( CLUS_OBJECT_NODE << 24 )  | CLCTL_INTRODUCE_GEM_REPAIR_DELAY ) ,
        CLUSCTL_NODE_SEND_DUMMY_GEM_MESSAGES	= ( ( CLUS_OBJECT_NODE << 24 )  | CLCTL_SEND_DUMMY_GEM_MESSAGES ) ,
        CLUSCTL_NODE_BLOCK_GEM_SEND_RECV	= ( ( CLUS_OBJECT_NODE << 24 )  | CLCTL_BLOCK_GEM_SEND_RECV ) ,
        CLUSCTL_NODE_GET_GEMID_VECTOR	= ( ( CLUS_OBJECT_NODE << 24 )  | CLCTL_GET_GEMID_VECTOR ) 
    } 	CLUSCTL_NODE_CODES;

typedef 
enum CLUSCTL_NETWORK_CODES
    {
        CLUSCTL_NETWORK_UNKNOWN	= ( ( CLUS_OBJECT_NETWORK << 24 )  | CLCTL_UNKNOWN ) ,
        CLUSCTL_NETWORK_GET_CHARACTERISTICS	= ( ( CLUS_OBJECT_NETWORK << 24 )  | CLCTL_GET_CHARACTERISTICS ) ,
        CLUSCTL_NETWORK_GET_FLAGS	= ( ( CLUS_OBJECT_NETWORK << 24 )  | CLCTL_GET_FLAGS ) ,
        CLUSCTL_NETWORK_GET_NAME	= ( ( CLUS_OBJECT_NETWORK << 24 )  | CLCTL_GET_NAME ) ,
        CLUSCTL_NETWORK_GET_ID	= ( ( CLUS_OBJECT_NETWORK << 24 )  | CLCTL_GET_ID ) ,
        CLUSCTL_NETWORK_ENUM_COMMON_PROPERTIES	= ( ( CLUS_OBJECT_NETWORK << 24 )  | CLCTL_ENUM_COMMON_PROPERTIES ) ,
        CLUSCTL_NETWORK_GET_RO_COMMON_PROPERTIES	= ( ( CLUS_OBJECT_NETWORK << 24 )  | CLCTL_GET_RO_COMMON_PROPERTIES ) ,
        CLUSCTL_NETWORK_GET_COMMON_PROPERTIES	= ( ( CLUS_OBJECT_NETWORK << 24 )  | CLCTL_GET_COMMON_PROPERTIES ) ,
        CLUSCTL_NETWORK_SET_COMMON_PROPERTIES	= ( ( CLUS_OBJECT_NETWORK << 24 )  | CLCTL_SET_COMMON_PROPERTIES ) ,
        CLUSCTL_NETWORK_VALIDATE_COMMON_PROPERTIES	= ( ( CLUS_OBJECT_NETWORK << 24 )  | CLCTL_VALIDATE_COMMON_PROPERTIES ) ,
        CLUSCTL_NETWORK_ENUM_PRIVATE_PROPERTIES	= ( ( CLUS_OBJECT_NETWORK << 24 )  | CLCTL_ENUM_PRIVATE_PROPERTIES ) ,
        CLUSCTL_NETWORK_GET_RO_PRIVATE_PROPERTIES	= ( ( CLUS_OBJECT_NETWORK << 24 )  | CLCTL_GET_RO_PRIVATE_PROPERTIES ) ,
        CLUSCTL_NETWORK_GET_PRIVATE_PROPERTIES	= ( ( CLUS_OBJECT_NETWORK << 24 )  | CLCTL_GET_PRIVATE_PROPERTIES ) ,
        CLUSCTL_NETWORK_SET_PRIVATE_PROPERTIES	= ( ( CLUS_OBJECT_NETWORK << 24 )  | CLCTL_SET_PRIVATE_PROPERTIES ) ,
        CLUSCTL_NETWORK_VALIDATE_PRIVATE_PROPERTIES	= ( ( CLUS_OBJECT_NETWORK << 24 )  | CLCTL_VALIDATE_PRIVATE_PROPERTIES ) ,
        CLUSCTL_NETWORK_GET_COMMON_PROPERTY_FMTS	= ( ( CLUS_OBJECT_NETWORK << 24 )  | CLCTL_GET_COMMON_PROPERTY_FMTS ) ,
        CLUSCTL_NETWORK_GET_PRIVATE_PROPERTY_FMTS	= ( ( CLUS_OBJECT_NETWORK << 24 )  | CLCTL_GET_PRIVATE_PROPERTY_FMTS ) 
    } 	CLUSCTL_NETWORK_CODES;

typedef 
enum CLUSCTL_NETINTERFACE_CODES
    {
        CLUSCTL_NETINTERFACE_UNKNOWN	= ( ( CLUS_OBJECT_NETINTERFACE << 24 )  | CLCTL_UNKNOWN ) ,
        CLUSCTL_NETINTERFACE_GET_CHARACTERISTICS	= ( ( CLUS_OBJECT_NETINTERFACE << 24 )  | CLCTL_GET_CHARACTERISTICS ) ,
        CLUSCTL_NETINTERFACE_GET_FLAGS	= ( ( CLUS_OBJECT_NETINTERFACE << 24 )  | CLCTL_GET_FLAGS ) ,
        CLUSCTL_NETINTERFACE_GET_NAME	= ( ( CLUS_OBJECT_NETINTERFACE << 24 )  | CLCTL_GET_NAME ) ,
        CLUSCTL_NETINTERFACE_GET_ID	= ( ( CLUS_OBJECT_NETINTERFACE << 24 )  | CLCTL_GET_ID ) ,
        CLUSCTL_NETINTERFACE_GET_NODE	= ( ( CLUS_OBJECT_NETINTERFACE << 24 )  | CLCTL_GET_NODE ) ,
        CLUSCTL_NETINTERFACE_GET_NETWORK	= ( ( CLUS_OBJECT_NETINTERFACE << 24 )  | CLCTL_GET_NETWORK ) ,
        CLUSCTL_NETINTERFACE_ENUM_COMMON_PROPERTIES	= ( ( CLUS_OBJECT_NETINTERFACE << 24 )  | CLCTL_ENUM_COMMON_PROPERTIES ) ,
        CLUSCTL_NETINTERFACE_GET_RO_COMMON_PROPERTIES	= ( ( CLUS_OBJECT_NETINTERFACE << 24 )  | CLCTL_GET_RO_COMMON_PROPERTIES ) ,
        CLUSCTL_NETINTERFACE_GET_COMMON_PROPERTIES	= ( ( CLUS_OBJECT_NETINTERFACE << 24 )  | CLCTL_GET_COMMON_PROPERTIES ) ,
        CLUSCTL_NETINTERFACE_SET_COMMON_PROPERTIES	= ( ( CLUS_OBJECT_NETINTERFACE << 24 )  | CLCTL_SET_COMMON_PROPERTIES ) ,
        CLUSCTL_NETINTERFACE_VALIDATE_COMMON_PROPERTIES	= ( ( CLUS_OBJECT_NETINTERFACE << 24 )  | CLCTL_VALIDATE_COMMON_PROPERTIES ) ,
        CLUSCTL_NETINTERFACE_ENUM_PRIVATE_PROPERTIES	= ( ( CLUS_OBJECT_NETINTERFACE << 24 )  | CLCTL_ENUM_PRIVATE_PROPERTIES ) ,
        CLUSCTL_NETINTERFACE_GET_RO_PRIVATE_PROPERTIES	= ( ( CLUS_OBJECT_NETINTERFACE << 24 )  | CLCTL_GET_RO_PRIVATE_PROPERTIES ) ,
        CLUSCTL_NETINTERFACE_GET_PRIVATE_PROPERTIES	= ( ( CLUS_OBJECT_NETINTERFACE << 24 )  | CLCTL_GET_PRIVATE_PROPERTIES ) ,
        CLUSCTL_NETINTERFACE_SET_PRIVATE_PROPERTIES	= ( ( CLUS_OBJECT_NETINTERFACE << 24 )  | CLCTL_SET_PRIVATE_PROPERTIES ) ,
        CLUSCTL_NETINTERFACE_VALIDATE_PRIVATE_PROPERTIES	= ( ( CLUS_OBJECT_NETINTERFACE << 24 )  | CLCTL_VALIDATE_PRIVATE_PROPERTIES ) ,
        CLUSCTL_NETINTERFACE_GET_COMMON_PROPERTY_FMTS	= ( ( CLUS_OBJECT_NETINTERFACE << 24 )  | CLCTL_GET_COMMON_PROPERTY_FMTS ) ,
        CLUSCTL_NETINTERFACE_GET_PRIVATE_PROPERTY_FMTS	= ( ( CLUS_OBJECT_NETINTERFACE << 24 )  | CLCTL_GET_PRIVATE_PROPERTY_FMTS ) 
    } 	CLUSCTL_NETINTERFACE_CODES;

typedef 
enum CLUSCTL_CLUSTER_CODES
    {
        CLUSCTL_CLUSTER_UNKNOWN	= ( ( CLUS_OBJECT_CLUSTER << 24 )  | CLCTL_UNKNOWN ) ,
        CLUSCTL_CLUSTER_GET_FQDN	= ( ( CLUS_OBJECT_CLUSTER << 24 )  | CLCTL_GET_FQDN ) ,
        CLUSCTL_CLUSTER_SET_STORAGE_CONFIGURATION	= ( ( CLUS_OBJECT_CLUSTER << 24 )  | CLCTL_SET_STORAGE_CONFIGURATION ) ,
        CLUSCTL_CLUSTER_GET_STORAGE_CONFIGURATION	= ( ( CLUS_OBJECT_CLUSTER << 24 )  | CLCTL_GET_STORAGE_CONFIGURATION ) ,
        CLUSCTL_CLUSTER_GET_STORAGE_CONFIG_ATTRIBUTES	= ( ( CLUS_OBJECT_CLUSTER << 24 )  | CLCTL_GET_STORAGE_CONFIG_ATTRIBUTES ) ,
        CLUSCTL_CLUSTER_ENUM_COMMON_PROPERTIES	= ( ( CLUS_OBJECT_CLUSTER << 24 )  | CLCTL_ENUM_COMMON_PROPERTIES ) ,
        CLUSCTL_CLUSTER_GET_RO_COMMON_PROPERTIES	= ( ( CLUS_OBJECT_CLUSTER << 24 )  | CLCTL_GET_RO_COMMON_PROPERTIES ) ,
        CLUSCTL_CLUSTER_GET_COMMON_PROPERTIES	= ( ( CLUS_OBJECT_CLUSTER << 24 )  | CLCTL_GET_COMMON_PROPERTIES ) ,
        CLUSCTL_CLUSTER_SET_COMMON_PROPERTIES	= ( ( CLUS_OBJECT_CLUSTER << 24 )  | CLCTL_SET_COMMON_PROPERTIES ) ,
        CLUSCTL_CLUSTER_VALIDATE_COMMON_PROPERTIES	= ( ( CLUS_OBJECT_CLUSTER << 24 )  | CLCTL_VALIDATE_COMMON_PROPERTIES ) ,
        CLUSCTL_CLUSTER_ENUM_PRIVATE_PROPERTIES	= ( ( CLUS_OBJECT_CLUSTER << 24 )  | CLCTL_ENUM_PRIVATE_PROPERTIES ) ,
        CLUSCTL_CLUSTER_GET_RO_PRIVATE_PROPERTIES	= ( ( CLUS_OBJECT_CLUSTER << 24 )  | CLCTL_GET_RO_PRIVATE_PROPERTIES ) ,
        CLUSCTL_CLUSTER_GET_PRIVATE_PROPERTIES	= ( ( CLUS_OBJECT_CLUSTER << 24 )  | CLCTL_GET_PRIVATE_PROPERTIES ) ,
        CLUSCTL_CLUSTER_SET_PRIVATE_PROPERTIES	= ( ( CLUS_OBJECT_CLUSTER << 24 )  | CLCTL_SET_PRIVATE_PROPERTIES ) ,
        CLUSCTL_CLUSTER_VALIDATE_PRIVATE_PROPERTIES	= ( ( CLUS_OBJECT_CLUSTER << 24 )  | CLCTL_VALIDATE_PRIVATE_PROPERTIES ) ,
        CLUSCTL_CLUSTER_GET_COMMON_PROPERTY_FMTS	= ( ( CLUS_OBJECT_CLUSTER << 24 )  | CLCTL_GET_COMMON_PROPERTY_FMTS ) ,
        CLUSCTL_CLUSTER_GET_PRIVATE_PROPERTY_FMTS	= ( ( CLUS_OBJECT_CLUSTER << 24 )  | CLCTL_GET_PRIVATE_PROPERTY_FMTS ) ,
        CLUSCTL_CLUSTER_CHECK_VOTER_EVICT	= ( ( CLUS_OBJECT_CLUSTER << 24 )  | CLCTL_CHECK_VOTER_EVICT ) ,
        CLUSCTL_CLUSTER_CHECK_VOTER_DOWN	= ( ( CLUS_OBJECT_CLUSTER << 24 )  | CLCTL_CHECK_VOTER_DOWN ) ,
        CLUSCTL_CLUSTER_SHUTDOWN	= ( ( CLUS_OBJECT_CLUSTER << 24 )  | CLCTL_SHUTDOWN ) ,
        CLUSCTL_CLUSTER_BATCH_BLOCK_KEY	= ( ( CLUS_OBJECT_CLUSTER << 24 )  | CLCTL_BATCH_BLOCK_KEY ) ,
        CLUSCTL_CLUSTER_BATCH_UNBLOCK_KEY	= ( ( CLUS_OBJECT_CLUSTER << 24 )  | CLCTL_BATCH_UNBLOCK_KEY ) ,
        CLUSCTL_CLUSTER_GET_SHARED_VOLUME_ID	= ( ( CLUS_OBJECT_CLUSTER << 24 )  | CLCTL_GET_SHARED_VOLUME_ID ) ,
        CLUSCTL_CLUSTER_GET_CLUSDB_TIMESTAMP	= ( ( CLUS_OBJECT_CLUSTER << 24 )  | CLCTL_GET_CLUSDB_TIMESTAMP ) ,
        CLUSCTL_CLUSTER_GET_GUM_LOCK_OWNER	= ( ( CLUS_OBJECT_CLUSTER << 24 )  | CLCTL_GET_GUM_LOCK_OWNER ) ,
        CLUSCTL_CLUSTER_REMOVE_NODE	= ( ( CLUS_OBJECT_CLUSTER << 24 )  | CLCTL_REMOVE_NODE ) ,
        CLUSCTL_CLUSTER_SET_ACCOUNT_ACCESS	= ( ( CLUS_OBJECT_CLUSTER << 24 )  | CLCTL_SET_ACCOUNT_ACCESS ) ,
        CLUSCTL_CLUSTER_CLEAR_NODE_CONNECTION_INFO	= ( ( CLUS_OBJECT_CLUSTER << 24 )  | CLCTL_CLEAR_NODE_CONNECTION_INFO ) ,
        CLUSCTL_CLUSTER_SET_DNS_DOMAIN	= ( ( CLUS_OBJECT_CLUSTER << 24 )  | CLCTL_SET_DNS_DOMAIN ) ,
        CLUSCTL_CLUSTER_SET_CLUSTER_S2D_ENABLED	= ( ( CLUS_OBJECT_CLUSTER << 24 )  | CLCTL_SET_CLUSTER_S2D_ENABLED ) ,
        CLUSCTL_CLUSTER_SET_CLUSTER_S2D_CACHE_METADATA_RESERVE_BYTES	= ( ( CLUS_OBJECT_CLUSTER << 24 )  | CLCTL_SET_CLUSTER_S2D_CACHE_METADATA_RESERVE_BYTES ) ,
        CLUSCTL_CLUSTER_STORAGE_RENAME_SHARED_VOLUME	= ( ( CLUS_OBJECT_CLUSTER << 24 )  | CLCTL_STORAGE_RENAME_SHARED_VOLUME ) ,
        CLUSCTL_CLUSTER_STORAGE_RENAME_SHARED_VOLUME_GUID	= ( ( CLUS_OBJECT_CLUSTER << 24 )  | CLCTL_STORAGE_RENAME_SHARED_VOLUME_GUID ) ,
        CLUSCTL_CLUSTER_RELOAD_AUTOLOGGER_CONFIG	= ( ( CLUS_OBJECT_CLUSTER << 24 )  | CLCTL_RELOAD_AUTOLOGGER_CONFIG ) ,
        CLUSCTL_CLUSTER_ENUM_AFFINITY_RULE_NAMES	= ( ( CLUS_OBJECT_CLUSTER << 24 )  | CLCTL_ENUM_AFFINITY_RULE_NAMES ) ,
        CLUSCTL_CLUSTER_GET_NODES_IN_FD	= ( ( CLUS_OBJECT_CLUSTER << 24 )  | CLCTL_GET_NODES_IN_FD ) ,
        CLUSCTL_CLUSTER_FORCE_FLUSH_DB	= ( ( CLUS_OBJECT_CLUSTER << 24 )  | CLCTL_FORCE_DB_FLUSH ) ,
        CLUSCTL_CLUSTER_GET_CLMUSR_TOKEN	= ( ( CLUS_OBJECT_CLUSTER << 24 )  | CLCTL_NETNAME_GET_VIRTUAL_SERVER_TOKEN ) ,
        CLUSCTL_CLUSTER_CHECK_VOTER_EVICT_WITNESS	= ( ( CLUS_OBJECT_CLUSTER << 24 )  | CLCTL_CHECK_VOTER_EVICT_WITNESS ) ,
        CLUSCTL_CLUSTER_CHECK_VOTER_DOWN_WITNESS	= ( ( CLUS_OBJECT_CLUSTER << 24 )  | CLCTL_CHECK_VOTER_DOWN_WITNESS ) 
    } 	CLUSCTL_CLUSTER_CODES;

typedef 
enum CLUSCTL_GROUPSET_CODES
    {
        CLUSCTL_GROUPSET_GET_COMMON_PROPERTIES	= ( ( CLUS_OBJECT_GROUPSET << 24 )  | CLCTL_GET_COMMON_PROPERTIES ) ,
        CLUSCTL_GROUPSET_GET_RO_COMMON_PROPERTIES	= ( ( CLUS_OBJECT_GROUPSET << 24 )  | CLCTL_GET_RO_COMMON_PROPERTIES ) ,
        CLUSCTL_GROUPSET_SET_COMMON_PROPERTIES	= ( ( CLUS_OBJECT_GROUPSET << 24 )  | CLCTL_SET_COMMON_PROPERTIES ) ,
        CLUSCTL_GROUPSET_GET_GROUPS	= ( ( CLUS_OBJECT_GROUPSET << 24 )  | CLCTL_GROUPSET_GET_GROUPS ) ,
        CLUSCTL_GROUPSET_GET_PROVIDER_GROUPS	= ( ( CLUS_OBJECT_GROUPSET << 24 )  | CLCTL_GROUPSET_GET_PROVIDER_GROUPS ) ,
        CLUSCTL_GROUPSET_GET_PROVIDER_GROUPSETS	= ( ( CLUS_OBJECT_GROUPSET << 24 )  | CLCTL_GROUPSET_GET_PROVIDER_GROUPSETS ) ,
        CLUSCTL_GROUP_GET_PROVIDER_GROUPS	= ( ( CLUS_OBJECT_GROUPSET << 24 )  | CLCTL_GROUP_GET_PROVIDER_GROUPS ) ,
        CLUSCTL_GROUP_GET_PROVIDER_GROUPSETS	= ( ( CLUS_OBJECT_GROUPSET << 24 )  | CLCTL_GROUP_GET_PROVIDER_GROUPSETS ) ,
        CLUSCTL_GROUPSET_GET_ID	= ( ( CLUS_OBJECT_GROUPSET << 24 )  | CLCTL_GET_ID ) 
    } 	CLUSCTL_GROUPSET_CODES;

typedef 
enum CLUSCTL_AFFINITYRULE_CODES
    {
        CLUSCTL_AFFINITYRULE_GET_COMMON_PROPERTIES	= ( ( CLUS_OBJECT_AFFINITYRULE << 24 )  | CLCTL_GET_COMMON_PROPERTIES ) ,
        CLUSCTL_AFFINITYRULE_GET_RO_COMMON_PROPERTIES	= ( ( CLUS_OBJECT_AFFINITYRULE << 24 )  | CLCTL_GET_RO_COMMON_PROPERTIES ) ,
        CLUSCTL_AFFINITYRULE_SET_COMMON_PROPERTIES	= ( ( CLUS_OBJECT_AFFINITYRULE << 24 )  | CLCTL_SET_COMMON_PROPERTIES ) ,
        CLUSCTL_AFFINITYRULE_GET_ID	= ( ( CLUS_OBJECT_AFFINITYRULE << 24 )  | CLCTL_GET_ID ) ,
        CLUSCTL_AFFINITYRULE_GET_GROUPNAMES	= ( ( CLUS_OBJECT_AFFINITYRULE << 24 )  | CLCTL_GROUPSET_GET_GROUPS ) 
    } 	CLUSCTL_AFFINITYRULE_CODES;

typedef 
enum CLUSTER_RESOURCE_CLASS
    {
        CLUS_RESCLASS_UNKNOWN	= 0,
        CLUS_RESCLASS_STORAGE	= ( CLUS_RESCLASS_UNKNOWN + 1 ) ,
        CLUS_RESCLASS_NETWORK	= ( CLUS_RESCLASS_STORAGE + 1 ) ,
        CLUS_RESCLASS_USER	= 32768
    } 	CLUSTER_RESOURCE_CLASS;

typedef 
enum CLUS_RESSUBCLASS
    {
        CLUS_RESSUBCLASS_SHARED	= 0x80000000
    } 	CLUS_RESSUBCLASS;

typedef 
enum CLUS_RESSUBCLASS_STORAGE
    {
        CLUS_RESSUBCLASS_STORAGE_SHARED_BUS	= 0x80000000,
        CLUS_RESSUBCLASS_STORAGE_DISK	= 0x40000000,
        CLUS_RESSUBCLASS_STORAGE_REPLICATION	= 0x10000000
    } 	CLUS_RESSUBCLASS_STORAGE;

typedef 
enum CLUS_RESSUBCLASS_NETWORK
    {
        CLUS_RESSUBCLASS_NETWORK_INTERNET_PROTOCOL	= 0x80000000
    } 	CLUS_RESSUBCLASS_NETWORK;

typedef 
enum CLUS_CHARACTERISTICS
    {
        CLUS_CHAR_UNKNOWN	= 0,
        CLUS_CHAR_QUORUM	= 0x1,
        CLUS_CHAR_DELETE_REQUIRES_ALL_NODES	= 0x2,
        CLUS_CHAR_LOCAL_QUORUM	= 0x4,
        CLUS_CHAR_LOCAL_QUORUM_DEBUG	= 0x8,
        CLUS_CHAR_REQUIRES_STATE_CHANGE_REASON	= 0x10,
        CLUS_CHAR_BROADCAST_DELETE	= 0x20,
        CLUS_CHAR_SINGLE_CLUSTER_INSTANCE	= 0x40,
        CLUS_CHAR_SINGLE_GROUP_INSTANCE	= 0x80,
        CLUS_CHAR_COEXIST_IN_SHARED_VOLUME_GROUP	= 0x100,
        CLUS_CHAR_PLACEMENT_DATA	= 0x200,
        CLUS_CHAR_MONITOR_DETACH	= 0x400,
        CLUS_CHAR_MONITOR_REATTACH	= 0x800,
        CLUS_CHAR_OPERATION_CONTEXT	= 0x1000,
        CLUS_CHAR_CLONES	= 0x2000,
        CLUS_CHAR_NOT_PREEMPTABLE	= 0x4000,
        CLUS_CHAR_NOTIFY_NEW_OWNER	= 0x8000,
        CLUS_CHAR_SUPPORTS_UNMONITORED_STATE	= 0x10000,
        CLUS_CHAR_INFRASTRUCTURE	= 0x20000,
        CLUS_CHAR_VETO_DRAIN	= 0x40000,
        CLUS_CHAR_DRAIN_LOCAL_OFFLINE	= 0x80000
    } 	CLUS_CHARACTERISTICS;

typedef 
enum CLUS_FLAGS
    {
        CLUS_FLAG_CORE	= 0x1
    } 	CLUS_FLAGS;

typedef 
enum CLUSPROP_PIFLAGS
    {
        CLUSPROP_PIFLAG_STICKY	= 0x1,
        CLUSPROP_PIFLAG_REMOVABLE	= 0x2,
        CLUSPROP_PIFLAG_USABLE	= 0x4,
        CLUSPROP_PIFLAG_DEFAULT_QUORUM	= 0x8,
        CLUSPROP_PIFLAG_USABLE_FOR_CSV	= 0x10,
        CLUSPROP_PIFLAG_ENCRYPTION_ENABLED	= 0x20,
        CLUSPROP_PIFLAG_RAW	= 0x40,
        CLUSPROP_PIFLAG_UNKNOWN	= 0x80000000
    } 	CLUSPROP_PIFLAGS;

typedef 
enum CLUSTER_RESOURCE_ENUM
    {
        CLUSTER_RESOURCE_ENUM_DEPENDS	= 0x1,
        CLUSTER_RESOURCE_ENUM_PROVIDES	= 0x2,
        CLUSTER_RESOURCE_ENUM_NODES	= 0x4,
        CLUSTER_RESOURCE_ENUM_ALL	= ( ( CLUSTER_RESOURCE_ENUM_DEPENDS | CLUSTER_RESOURCE_ENUM_PROVIDES )  | CLUSTER_RESOURCE_ENUM_NODES ) 
    } 	CLUSTER_RESOURCE_ENUM;

typedef 
enum CLUSTER_RESOURCE_TYPE_ENUM
    {
        CLUSTER_RESOURCE_TYPE_ENUM_NODES	= 0x1,
        CLUSTER_RESOURCE_TYPE_ENUM_RESOURCES	= 0x2,
        CLUSTER_RESOURCE_TYPE_ENUM_ALL	= ( CLUSTER_RESOURCE_TYPE_ENUM_NODES | CLUSTER_RESOURCE_TYPE_ENUM_RESOURCES ) 
    } 	CLUSTER_RESOURCE_TYPE_ENUM;

typedef 
enum CLUSTER_NETWORK_ENUM
    {
        CLUSTER_NETWORK_ENUM_NETINTERFACES	= 0x1,
        CLUSTER_NETWORK_ENUM_ALL	= CLUSTER_NETWORK_ENUM_NETINTERFACES
    } 	CLUSTER_NETWORK_ENUM;

typedef 
enum CLUSTER_NETWORK_STATE
    {
        ClusterNetworkStateUnknown	= -1,
        ClusterNetworkUnavailable	= ( ClusterNetworkStateUnknown + 1 ) ,
        ClusterNetworkDown	= ( ClusterNetworkUnavailable + 1 ) ,
        ClusterNetworkPartitioned	= ( ClusterNetworkDown + 1 ) ,
        ClusterNetworkUp	= ( ClusterNetworkPartitioned + 1 ) 
    } 	CLUSTER_NETWORK_STATE;

typedef 
enum CLUSTER_NETWORK_ROLE
    {
        ClusterNetworkRoleNone	= 0,
        ClusterNetworkRoleInternalUse	= 0x1,
        ClusterNetworkRoleClientAccess	= 0x2,
        ClusterNetworkRoleInternalAndClient	= 0x3
    } 	CLUSTER_NETWORK_ROLE;

typedef 
enum CLUSTER_NETINTERFACE_STATE
    {
        ClusterNetInterfaceStateUnknown	= -1,
        ClusterNetInterfaceUnavailable	= ( ClusterNetInterfaceStateUnknown + 1 ) ,
        ClusterNetInterfaceFailed	= ( ClusterNetInterfaceUnavailable + 1 ) ,
        ClusterNetInterfaceUnreachable	= ( ClusterNetInterfaceFailed + 1 ) ,
        ClusterNetInterfaceUp	= ( ClusterNetInterfaceUnreachable + 1 ) 
    } 	CLUSTER_NETINTERFACE_STATE;

#pragma endregion
#endif // _CLUSTER_API_TYPES_











































extern RPC_IF_HANDLE __MIDL_itf_msclus_0000_0000_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_msclus_0000_0000_v0_0_s_ifspec;


#ifndef __MSClusterLib_LIBRARY_DEFINED__
#define __MSClusterLib_LIBRARY_DEFINED__

/* library MSClusterLib */
/* [helpstring][version][uuid] */ 

typedef CLUSTER_QUORUM_TYPE _CLUSTER_QUORUM_TYPE;

typedef NODE_CLUSTER_STATE _NODE_CLUSTER_STATE;

typedef CLUSTER_RESOURCE_STATE_CHANGE_REASON _CLUSTER_RESOURCE_STATE_CHANGE_REASON;

typedef CLUSTER_CHANGE _CLUSTER_CHANGE;

typedef CLUSTER_ENUM _CLUSTER_ENUM;

typedef CLUSTER_NODE_ENUM _CLUSTER_NODE_ENUM;

typedef CLUSTER_NODE_STATE _CLUSTER_NODE_STATE;

typedef CLUSTER_GROUP_ENUM _CLUSTER_GROUP_ENUM;

typedef CLUSTER_GROUP_STATE _CLUSTER_GROUP_STATE;

typedef CLUSTER_GROUP_AUTOFAILBACK_TYPE _CLUSTER_GROUP_AUTOFAILBACK_TYPE;

typedef CLUSTER_RESOURCE_STATE _CLUSTER_RESOURCE_STATE;

typedef CLUSTER_RESOURCE_RESTART_ACTION _CLUSTER_RESOURCE_RESTART_ACTION;

typedef CLUSTER_RESOURCE_CREATE_FLAGS _CLUSTER_RESOURCE_CREATE_FLAGS;

typedef CLUSTER_PROPERTY_TYPE _CLUSTER_PROPERTY_TYPE;

typedef CLUSTER_PROPERTY_FORMAT _CLUSTER_PROPERTY_FORMAT;

typedef CLUSTER_PROPERTY_SYNTAX _CLUSTER_PROPERTY_SYNTAX;

typedef CLUSTER_CONTROL_OBJECT _CLUSTER_CONTROL_OBJECT;

typedef CLCTL_CODES _CLCTL_CODES;

typedef CLUSCTL_RESOURCE_CODES _CLUSCTL_RESOURCE_CODES;

typedef CLUSCTL_RESOURCE_TYPE_CODES _CLUSCTL_RESOURCE_TYPE_CODES;

typedef CLUSCTL_GROUP_CODES _CLUSCTL_GROUP_CODES;

typedef CLUSCTL_NODE_CODES _CLUSCTL_NODE_CODES;

typedef CLUSCTL_NETWORK_CODES _CLUSCTL_NETWORK_CODES;

typedef CLUSCTL_NETINTERFACE_CODES _CLUSCTL_NETINTERFACE_CODES;

typedef CLUSCTL_CLUSTER_CODES _CLUSCTL_CLUSTER_CODES;

typedef CLUSTER_RESOURCE_CLASS _CLUSTER_RESOURCE_CLASS;

typedef CLUS_RESSUBCLASS _CLUS_RESSUBCLASS;

typedef CLUS_CHARACTERISTICS _CLUS_CHARACTERISTICS;

typedef CLUS_FLAGS _CLUS_FLAGS;

typedef CLUSPROP_PIFLAGS _CLUSPROP_PIFLAGS;

typedef CLUSTER_RESOURCE_ENUM _CLUSTER_RESOURCE_ENUM;

typedef CLUSTER_RESOURCE_TYPE_ENUM _CLUSTER_RESOURCE_TYPE_ENUM;

typedef CLUSTER_NETWORK_ENUM _CLUSTER_NETWORK_ENUM;

typedef CLUSTER_NETWORK_STATE _CLUSTER_NETWORK_STATE;

typedef CLUSTER_NETWORK_ROLE _CLUSTER_NETWORK_ROLE;

typedef CLUSTER_NETINTERFACE_STATE _CLUSTER_NETINTERFACE_STATE;


EXTERN_C const IID LIBID_MSClusterLib;

EXTERN_C const CLSID CLSID_ClusApplication;

#ifdef __cplusplus

class DECLSPEC_UUID("f2e606e5-2631-11d1-89f1-00a0c90d061e")
ClusApplication;
#endif

EXTERN_C const CLSID CLSID_Cluster;

#ifdef __cplusplus

class DECLSPEC_UUID("f2e606e3-2631-11d1-89f1-00a0c90d061e")
Cluster;
#endif

EXTERN_C const CLSID CLSID_ClusVersion;

#ifdef __cplusplus

class DECLSPEC_UUID("f2e60715-2631-11d1-89f1-00a0c90d061e")
ClusVersion;
#endif

EXTERN_C const CLSID CLSID_ClusResType;

#ifdef __cplusplus

class DECLSPEC_UUID("f2e6070f-2631-11d1-89f1-00a0c90d061e")
ClusResType;
#endif

EXTERN_C const CLSID CLSID_ClusProperty;

#ifdef __cplusplus

class DECLSPEC_UUID("f2e606fd-2631-11d1-89f1-00a0c90d061e")
ClusProperty;
#endif

EXTERN_C const CLSID CLSID_ClusProperties;

#ifdef __cplusplus

class DECLSPEC_UUID("f2e606ff-2631-11d1-89f1-00a0c90d061e")
ClusProperties;
#endif

EXTERN_C const CLSID CLSID_DomainNames;

#ifdef __cplusplus

class DECLSPEC_UUID("f2e606e1-2631-11d1-89f1-00a0c90d061e")
DomainNames;
#endif

EXTERN_C const CLSID CLSID_ClusNetwork;

#ifdef __cplusplus

class DECLSPEC_UUID("f2e606f1-2631-11d1-89f1-00a0c90d061e")
ClusNetwork;
#endif

EXTERN_C const CLSID CLSID_ClusNetInterface;

#ifdef __cplusplus

class DECLSPEC_UUID("f2e606ed-2631-11d1-89f1-00a0c90d061e")
ClusNetInterface;
#endif

EXTERN_C const CLSID CLSID_ClusNetInterfaces;

#ifdef __cplusplus

class DECLSPEC_UUID("f2e606ef-2631-11d1-89f1-00a0c90d061e")
ClusNetInterfaces;
#endif

EXTERN_C const CLSID CLSID_ClusResDependencies;

#ifdef __cplusplus

class DECLSPEC_UUID("f2e60703-2631-11d1-89f1-00a0c90d061e")
ClusResDependencies;
#endif

EXTERN_C const CLSID CLSID_ClusResGroupResources;

#ifdef __cplusplus

class DECLSPEC_UUID("f2e606e9-2631-11d1-89f1-00a0c90d061e")
ClusResGroupResources;
#endif

EXTERN_C const CLSID CLSID_ClusResTypeResources;

#ifdef __cplusplus

class DECLSPEC_UUID("f2e60713-2631-11d1-89f1-00a0c90d061e")
ClusResTypeResources;
#endif

EXTERN_C const CLSID CLSID_ClusResGroupPreferredOwnerNodes;

#ifdef __cplusplus

class DECLSPEC_UUID("f2e606e7-2631-11d1-89f1-00a0c90d061e")
ClusResGroupPreferredOwnerNodes;
#endif

EXTERN_C const CLSID CLSID_ClusResPossibleOwnerNodes;

#ifdef __cplusplus

class DECLSPEC_UUID("f2e6070d-2631-11d1-89f1-00a0c90d061e")
ClusResPossibleOwnerNodes;
#endif

EXTERN_C const CLSID CLSID_ClusNetworks;

#ifdef __cplusplus

class DECLSPEC_UUID("f2e606f3-2631-11d1-89f1-00a0c90d061e")
ClusNetworks;
#endif

EXTERN_C const CLSID CLSID_ClusNetworkNetInterfaces;

#ifdef __cplusplus

class DECLSPEC_UUID("f2e606f5-2631-11d1-89f1-00a0c90d061e")
ClusNetworkNetInterfaces;
#endif

EXTERN_C const CLSID CLSID_ClusNodeNetInterfaces;

#ifdef __cplusplus

class DECLSPEC_UUID("f2e606fb-2631-11d1-89f1-00a0c90d061e")
ClusNodeNetInterfaces;
#endif

EXTERN_C const CLSID CLSID_ClusRefObject;

#ifdef __cplusplus

class DECLSPEC_UUID("f2e60701-2631-11d1-89f1-00a0c90d061e")
ClusRefObject;
#endif

EXTERN_C const CLSID CLSID_ClusterNames;

#ifdef __cplusplus

class DECLSPEC_UUID("f2e606eb-2631-11d1-89f1-00a0c90d061e")
ClusterNames;
#endif

EXTERN_C const CLSID CLSID_ClusNode;

#ifdef __cplusplus

class DECLSPEC_UUID("f2e606f7-2631-11d1-89f1-00a0c90d061e")
ClusNode;
#endif

EXTERN_C const CLSID CLSID_ClusNodes;

#ifdef __cplusplus

class DECLSPEC_UUID("f2e606f9-2631-11d1-89f1-00a0c90d061e")
ClusNodes;
#endif

EXTERN_C const CLSID CLSID_ClusResGroup;

#ifdef __cplusplus

class DECLSPEC_UUID("f2e60705-2631-11d1-89f1-00a0c90d061e")
ClusResGroup;
#endif

EXTERN_C const CLSID CLSID_ClusResGroups;

#ifdef __cplusplus

class DECLSPEC_UUID("f2e60707-2631-11d1-89f1-00a0c90d061e")
ClusResGroups;
#endif

EXTERN_C const CLSID CLSID_ClusResource;

#ifdef __cplusplus

class DECLSPEC_UUID("f2e60709-2631-11d1-89f1-00a0c90d061e")
ClusResource;
#endif

EXTERN_C const CLSID CLSID_ClusResources;

#ifdef __cplusplus

class DECLSPEC_UUID("f2e6070b-2631-11d1-89f1-00a0c90d061e")
ClusResources;
#endif

EXTERN_C const CLSID CLSID_ClusResTypes;

#ifdef __cplusplus

class DECLSPEC_UUID("f2e60711-2631-11d1-89f1-00a0c90d061e")
ClusResTypes;
#endif

EXTERN_C const CLSID CLSID_ClusResTypePossibleOwnerNodes;

#ifdef __cplusplus

class DECLSPEC_UUID("f2e60717-2631-11d1-89f1-00a0c90d061e")
ClusResTypePossibleOwnerNodes;
#endif

EXTERN_C const CLSID CLSID_ClusPropertyValue;

#ifdef __cplusplus

class DECLSPEC_UUID("f2e60719-2631-11d1-89f1-00a0c90d061e")
ClusPropertyValue;
#endif

EXTERN_C const CLSID CLSID_ClusPropertyValues;

#ifdef __cplusplus

class DECLSPEC_UUID("f2e6071b-2631-11d1-89f1-00a0c90d061e")
ClusPropertyValues;
#endif

EXTERN_C const CLSID CLSID_ClusPropertyValueData;

#ifdef __cplusplus

class DECLSPEC_UUID("f2e6071d-2631-11d1-89f1-00a0c90d061e")
ClusPropertyValueData;
#endif

EXTERN_C const CLSID CLSID_ClusPartition;

#ifdef __cplusplus

class DECLSPEC_UUID("f2e6071f-2631-11d1-89f1-00a0c90d061e")
ClusPartition;
#endif

EXTERN_C const CLSID CLSID_ClusPartitionEx;

#ifdef __cplusplus

class DECLSPEC_UUID("53d51d26-b51b-4a79-b2c3-5048d93a98fc")
ClusPartitionEx;
#endif

EXTERN_C const CLSID CLSID_ClusPartitions;

#ifdef __cplusplus

class DECLSPEC_UUID("f2e60721-2631-11d1-89f1-00a0c90d061e")
ClusPartitions;
#endif

EXTERN_C const CLSID CLSID_ClusDisk;

#ifdef __cplusplus

class DECLSPEC_UUID("f2e60723-2631-11d1-89f1-00a0c90d061e")
ClusDisk;
#endif

EXTERN_C const CLSID CLSID_ClusDisks;

#ifdef __cplusplus

class DECLSPEC_UUID("f2e60725-2631-11d1-89f1-00a0c90d061e")
ClusDisks;
#endif

EXTERN_C const CLSID CLSID_ClusScsiAddress;

#ifdef __cplusplus

class DECLSPEC_UUID("f2e60727-2631-11d1-89f1-00a0c90d061e")
ClusScsiAddress;
#endif

EXTERN_C const CLSID CLSID_ClusRegistryKeys;

#ifdef __cplusplus

class DECLSPEC_UUID("f2e60729-2631-11d1-89f1-00a0c90d061e")
ClusRegistryKeys;
#endif

EXTERN_C const CLSID CLSID_ClusCryptoKeys;

#ifdef __cplusplus

class DECLSPEC_UUID("f2e6072b-2631-11d1-89f1-00a0c90d061e")
ClusCryptoKeys;
#endif

EXTERN_C const CLSID CLSID_ClusResDependents;

#ifdef __cplusplus

class DECLSPEC_UUID("f2e6072d-2631-11d1-89f1-00a0c90d061e")
ClusResDependents;
#endif
#endif /* __MSClusterLib_LIBRARY_DEFINED__ */

#ifndef __ISClusApplication_INTERFACE_DEFINED__
#define __ISClusApplication_INTERFACE_DEFINED__

/* interface ISClusApplication */
/* [unique][helpstring][dual][uuid][object][nonextensible][hidden][oleautomation] */ 


EXTERN_C const IID IID_ISClusApplication;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("f2e606e6-2631-11d1-89f1-00a0c90d061e")
    ISClusApplication : public IDispatch
    {
    public:
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_DomainNames( 
            /* [retval][out] */ __RPC__deref_out_opt ISDomainNames **ppDomains) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_ClusterNames( 
            /* [in] */ __RPC__in BSTR bstrDomainName,
            /* [retval][out] */ __RPC__deref_out_opt ISClusterNames **ppClusters) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE OpenCluster( 
            /* [in] */ __RPC__in BSTR bstrClusterName,
            /* [retval][out] */ __RPC__deref_out_opt ISCluster **pCluster) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ISClusApplicationVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in ISClusApplication * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in ISClusApplication * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in ISClusApplication * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in ISClusApplication * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in ISClusApplication * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in ISClusApplication * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            ISClusApplication * This,
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
        
        DECLSPEC_XFGVIRT(ISClusApplication, get_DomainNames)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_DomainNames )( 
            __RPC__in ISClusApplication * This,
            /* [retval][out] */ __RPC__deref_out_opt ISDomainNames **ppDomains);
        
        DECLSPEC_XFGVIRT(ISClusApplication, get_ClusterNames)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_ClusterNames )( 
            __RPC__in ISClusApplication * This,
            /* [in] */ __RPC__in BSTR bstrDomainName,
            /* [retval][out] */ __RPC__deref_out_opt ISClusterNames **ppClusters);
        
        DECLSPEC_XFGVIRT(ISClusApplication, OpenCluster)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *OpenCluster )( 
            __RPC__in ISClusApplication * This,
            /* [in] */ __RPC__in BSTR bstrClusterName,
            /* [retval][out] */ __RPC__deref_out_opt ISCluster **pCluster);
        
        END_INTERFACE
    } ISClusApplicationVtbl;

    interface ISClusApplication
    {
        CONST_VTBL struct ISClusApplicationVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ISClusApplication_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ISClusApplication_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ISClusApplication_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ISClusApplication_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define ISClusApplication_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define ISClusApplication_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define ISClusApplication_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define ISClusApplication_get_DomainNames(This,ppDomains)	\
    ( (This)->lpVtbl -> get_DomainNames(This,ppDomains) ) 

#define ISClusApplication_get_ClusterNames(This,bstrDomainName,ppClusters)	\
    ( (This)->lpVtbl -> get_ClusterNames(This,bstrDomainName,ppClusters) ) 

#define ISClusApplication_OpenCluster(This,bstrClusterName,pCluster)	\
    ( (This)->lpVtbl -> OpenCluster(This,bstrClusterName,pCluster) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ISClusApplication_INTERFACE_DEFINED__ */


#ifndef __ISDomainNames_INTERFACE_DEFINED__
#define __ISDomainNames_INTERFACE_DEFINED__

/* interface ISDomainNames */
/* [unique][helpstring][dual][uuid][object][nonextensible][hidden][oleautomation] */ 


EXTERN_C const IID IID_ISDomainNames;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("f2e606e2-2631-11d1-89f1-00a0c90d061e")
    ISDomainNames : public IDispatch
    {
    public:
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Count( 
            /* [retval][out] */ __RPC__out long *plCount) = 0;
        
        virtual /* [helpstring][id][restricted][propget] */ HRESULT STDMETHODCALLTYPE get__NewEnum( 
            /* [retval][out] */ __RPC__deref_out_opt IUnknown **retval) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE Refresh( void) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Item( 
            /* [in] */ VARIANT varIndex,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrDomainName) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ISDomainNamesVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in ISDomainNames * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in ISDomainNames * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in ISDomainNames * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in ISDomainNames * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in ISDomainNames * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in ISDomainNames * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            ISDomainNames * This,
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
        
        DECLSPEC_XFGVIRT(ISDomainNames, get_Count)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Count )( 
            __RPC__in ISDomainNames * This,
            /* [retval][out] */ __RPC__out long *plCount);
        
        DECLSPEC_XFGVIRT(ISDomainNames, get__NewEnum)
        /* [helpstring][id][restricted][propget] */ HRESULT ( STDMETHODCALLTYPE *get__NewEnum )( 
            __RPC__in ISDomainNames * This,
            /* [retval][out] */ __RPC__deref_out_opt IUnknown **retval);
        
        DECLSPEC_XFGVIRT(ISDomainNames, Refresh)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *Refresh )( 
            __RPC__in ISDomainNames * This);
        
        DECLSPEC_XFGVIRT(ISDomainNames, get_Item)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Item )( 
            __RPC__in ISDomainNames * This,
            /* [in] */ VARIANT varIndex,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrDomainName);
        
        END_INTERFACE
    } ISDomainNamesVtbl;

    interface ISDomainNames
    {
        CONST_VTBL struct ISDomainNamesVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ISDomainNames_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ISDomainNames_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ISDomainNames_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ISDomainNames_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define ISDomainNames_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define ISDomainNames_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define ISDomainNames_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define ISDomainNames_get_Count(This,plCount)	\
    ( (This)->lpVtbl -> get_Count(This,plCount) ) 

#define ISDomainNames_get__NewEnum(This,retval)	\
    ( (This)->lpVtbl -> get__NewEnum(This,retval) ) 

#define ISDomainNames_Refresh(This)	\
    ( (This)->lpVtbl -> Refresh(This) ) 

#define ISDomainNames_get_Item(This,varIndex,pbstrDomainName)	\
    ( (This)->lpVtbl -> get_Item(This,varIndex,pbstrDomainName) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ISDomainNames_INTERFACE_DEFINED__ */


#ifndef __ISClusterNames_INTERFACE_DEFINED__
#define __ISClusterNames_INTERFACE_DEFINED__

/* interface ISClusterNames */
/* [unique][helpstring][dual][uuid][object][nonextensible][hidden][oleautomation] */ 


EXTERN_C const IID IID_ISClusterNames;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("f2e606ec-2631-11d1-89f1-00a0c90d061e")
    ISClusterNames : public IDispatch
    {
    public:
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Count( 
            /* [retval][out] */ __RPC__out long *plCount) = 0;
        
        virtual /* [helpstring][id][restricted][propget] */ HRESULT STDMETHODCALLTYPE get__NewEnum( 
            /* [retval][out] */ __RPC__deref_out_opt IUnknown **retval) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE Refresh( void) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Item( 
            /* [in] */ VARIANT varIndex,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrClusterName) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_DomainName( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrDomainName) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ISClusterNamesVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in ISClusterNames * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in ISClusterNames * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in ISClusterNames * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in ISClusterNames * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in ISClusterNames * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in ISClusterNames * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            ISClusterNames * This,
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
        
        DECLSPEC_XFGVIRT(ISClusterNames, get_Count)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Count )( 
            __RPC__in ISClusterNames * This,
            /* [retval][out] */ __RPC__out long *plCount);
        
        DECLSPEC_XFGVIRT(ISClusterNames, get__NewEnum)
        /* [helpstring][id][restricted][propget] */ HRESULT ( STDMETHODCALLTYPE *get__NewEnum )( 
            __RPC__in ISClusterNames * This,
            /* [retval][out] */ __RPC__deref_out_opt IUnknown **retval);
        
        DECLSPEC_XFGVIRT(ISClusterNames, Refresh)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *Refresh )( 
            __RPC__in ISClusterNames * This);
        
        DECLSPEC_XFGVIRT(ISClusterNames, get_Item)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Item )( 
            __RPC__in ISClusterNames * This,
            /* [in] */ VARIANT varIndex,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrClusterName);
        
        DECLSPEC_XFGVIRT(ISClusterNames, get_DomainName)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_DomainName )( 
            __RPC__in ISClusterNames * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrDomainName);
        
        END_INTERFACE
    } ISClusterNamesVtbl;

    interface ISClusterNames
    {
        CONST_VTBL struct ISClusterNamesVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ISClusterNames_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ISClusterNames_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ISClusterNames_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ISClusterNames_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define ISClusterNames_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define ISClusterNames_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define ISClusterNames_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define ISClusterNames_get_Count(This,plCount)	\
    ( (This)->lpVtbl -> get_Count(This,plCount) ) 

#define ISClusterNames_get__NewEnum(This,retval)	\
    ( (This)->lpVtbl -> get__NewEnum(This,retval) ) 

#define ISClusterNames_Refresh(This)	\
    ( (This)->lpVtbl -> Refresh(This) ) 

#define ISClusterNames_get_Item(This,varIndex,pbstrClusterName)	\
    ( (This)->lpVtbl -> get_Item(This,varIndex,pbstrClusterName) ) 

#define ISClusterNames_get_DomainName(This,pbstrDomainName)	\
    ( (This)->lpVtbl -> get_DomainName(This,pbstrDomainName) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ISClusterNames_INTERFACE_DEFINED__ */


#ifndef __ISClusRefObject_INTERFACE_DEFINED__
#define __ISClusRefObject_INTERFACE_DEFINED__

/* interface ISClusRefObject */
/* [unique][helpstring][dual][uuid][object][nonextensible][hidden][oleautomation] */ 


EXTERN_C const IID IID_ISClusRefObject;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("f2e60702-2631-11d1-89f1-00a0c90d061e")
    ISClusRefObject : public IDispatch
    {
    public:
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Handle( 
            /* [retval][out] */ __RPC__out ULONG_PTR *phandle) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ISClusRefObjectVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in ISClusRefObject * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in ISClusRefObject * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in ISClusRefObject * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in ISClusRefObject * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in ISClusRefObject * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in ISClusRefObject * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            ISClusRefObject * This,
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
        
        DECLSPEC_XFGVIRT(ISClusRefObject, get_Handle)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Handle )( 
            __RPC__in ISClusRefObject * This,
            /* [retval][out] */ __RPC__out ULONG_PTR *phandle);
        
        END_INTERFACE
    } ISClusRefObjectVtbl;

    interface ISClusRefObject
    {
        CONST_VTBL struct ISClusRefObjectVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ISClusRefObject_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ISClusRefObject_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ISClusRefObject_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ISClusRefObject_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define ISClusRefObject_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define ISClusRefObject_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define ISClusRefObject_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define ISClusRefObject_get_Handle(This,phandle)	\
    ( (This)->lpVtbl -> get_Handle(This,phandle) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ISClusRefObject_INTERFACE_DEFINED__ */


#ifndef __ISClusVersion_INTERFACE_DEFINED__
#define __ISClusVersion_INTERFACE_DEFINED__

/* interface ISClusVersion */
/* [unique][helpstring][dual][uuid][object][nonextensible][hidden][oleautomation] */ 


EXTERN_C const IID IID_ISClusVersion;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("f2e60716-2631-11d1-89f1-00a0c90d061e")
    ISClusVersion : public IDispatch
    {
    public:
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Name( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrClusterName) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_MajorVersion( 
            /* [retval][out] */ __RPC__out long *pnMajorVersion) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_MinorVersion( 
            /* [retval][out] */ __RPC__out long *pnMinorVersion) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_BuildNumber( 
            /* [retval][out] */ __RPC__out SHORT *pnBuildNumber) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_VendorId( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrVendorId) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_CSDVersion( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrCSDVersion) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_ClusterHighestVersion( 
            /* [retval][out] */ __RPC__out long *pnClusterHighestVersion) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_ClusterLowestVersion( 
            /* [retval][out] */ __RPC__out long *pnClusterLowestVersion) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Flags( 
            /* [retval][out] */ __RPC__out long *pnFlags) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_MixedVersion( 
            /* [retval][out] */ __RPC__out VARIANT *pvarMixedVersion) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ISClusVersionVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in ISClusVersion * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in ISClusVersion * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in ISClusVersion * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in ISClusVersion * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in ISClusVersion * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in ISClusVersion * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            ISClusVersion * This,
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
        
        DECLSPEC_XFGVIRT(ISClusVersion, get_Name)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Name )( 
            __RPC__in ISClusVersion * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrClusterName);
        
        DECLSPEC_XFGVIRT(ISClusVersion, get_MajorVersion)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_MajorVersion )( 
            __RPC__in ISClusVersion * This,
            /* [retval][out] */ __RPC__out long *pnMajorVersion);
        
        DECLSPEC_XFGVIRT(ISClusVersion, get_MinorVersion)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_MinorVersion )( 
            __RPC__in ISClusVersion * This,
            /* [retval][out] */ __RPC__out long *pnMinorVersion);
        
        DECLSPEC_XFGVIRT(ISClusVersion, get_BuildNumber)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_BuildNumber )( 
            __RPC__in ISClusVersion * This,
            /* [retval][out] */ __RPC__out SHORT *pnBuildNumber);
        
        DECLSPEC_XFGVIRT(ISClusVersion, get_VendorId)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_VendorId )( 
            __RPC__in ISClusVersion * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrVendorId);
        
        DECLSPEC_XFGVIRT(ISClusVersion, get_CSDVersion)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_CSDVersion )( 
            __RPC__in ISClusVersion * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrCSDVersion);
        
        DECLSPEC_XFGVIRT(ISClusVersion, get_ClusterHighestVersion)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_ClusterHighestVersion )( 
            __RPC__in ISClusVersion * This,
            /* [retval][out] */ __RPC__out long *pnClusterHighestVersion);
        
        DECLSPEC_XFGVIRT(ISClusVersion, get_ClusterLowestVersion)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_ClusterLowestVersion )( 
            __RPC__in ISClusVersion * This,
            /* [retval][out] */ __RPC__out long *pnClusterLowestVersion);
        
        DECLSPEC_XFGVIRT(ISClusVersion, get_Flags)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Flags )( 
            __RPC__in ISClusVersion * This,
            /* [retval][out] */ __RPC__out long *pnFlags);
        
        DECLSPEC_XFGVIRT(ISClusVersion, get_MixedVersion)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_MixedVersion )( 
            __RPC__in ISClusVersion * This,
            /* [retval][out] */ __RPC__out VARIANT *pvarMixedVersion);
        
        END_INTERFACE
    } ISClusVersionVtbl;

    interface ISClusVersion
    {
        CONST_VTBL struct ISClusVersionVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ISClusVersion_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ISClusVersion_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ISClusVersion_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ISClusVersion_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define ISClusVersion_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define ISClusVersion_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define ISClusVersion_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define ISClusVersion_get_Name(This,pbstrClusterName)	\
    ( (This)->lpVtbl -> get_Name(This,pbstrClusterName) ) 

#define ISClusVersion_get_MajorVersion(This,pnMajorVersion)	\
    ( (This)->lpVtbl -> get_MajorVersion(This,pnMajorVersion) ) 

#define ISClusVersion_get_MinorVersion(This,pnMinorVersion)	\
    ( (This)->lpVtbl -> get_MinorVersion(This,pnMinorVersion) ) 

#define ISClusVersion_get_BuildNumber(This,pnBuildNumber)	\
    ( (This)->lpVtbl -> get_BuildNumber(This,pnBuildNumber) ) 

#define ISClusVersion_get_VendorId(This,pbstrVendorId)	\
    ( (This)->lpVtbl -> get_VendorId(This,pbstrVendorId) ) 

#define ISClusVersion_get_CSDVersion(This,pbstrCSDVersion)	\
    ( (This)->lpVtbl -> get_CSDVersion(This,pbstrCSDVersion) ) 

#define ISClusVersion_get_ClusterHighestVersion(This,pnClusterHighestVersion)	\
    ( (This)->lpVtbl -> get_ClusterHighestVersion(This,pnClusterHighestVersion) ) 

#define ISClusVersion_get_ClusterLowestVersion(This,pnClusterLowestVersion)	\
    ( (This)->lpVtbl -> get_ClusterLowestVersion(This,pnClusterLowestVersion) ) 

#define ISClusVersion_get_Flags(This,pnFlags)	\
    ( (This)->lpVtbl -> get_Flags(This,pnFlags) ) 

#define ISClusVersion_get_MixedVersion(This,pvarMixedVersion)	\
    ( (This)->lpVtbl -> get_MixedVersion(This,pvarMixedVersion) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ISClusVersion_INTERFACE_DEFINED__ */


#ifndef __ISCluster_INTERFACE_DEFINED__
#define __ISCluster_INTERFACE_DEFINED__

/* interface ISCluster */
/* [unique][helpstring][dual][uuid][object][nonextensible][hidden][oleautomation] */ 


EXTERN_C const IID IID_ISCluster;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("f2e606e4-2631-11d1-89f1-00a0c90d061e")
    ISCluster : public IDispatch
    {
    public:
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_CommonProperties( 
            /* [retval][out] */ __RPC__deref_out_opt ISClusProperties **ppProperties) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_PrivateProperties( 
            /* [retval][out] */ __RPC__deref_out_opt ISClusProperties **ppProperties) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_CommonROProperties( 
            /* [retval][out] */ __RPC__deref_out_opt ISClusProperties **ppProperties) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_PrivateROProperties( 
            /* [retval][out] */ __RPC__deref_out_opt ISClusProperties **ppProperties) = 0;
        
        virtual /* [helpstring][id][hidden][propget] */ HRESULT STDMETHODCALLTYPE get_Handle( 
            /* [retval][out] */ __RPC__out ULONG_PTR *phandle) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE Open( 
            /* [in] */ __RPC__in BSTR bstrClusterName) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Name( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrName) = 0;
        
        virtual /* [helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_Name( 
            /* [in] */ __RPC__in BSTR bstrClusterName) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Version( 
            /* [retval][out] */ __RPC__deref_out_opt ISClusVersion **ppClusVersion) = 0;
        
        virtual /* [helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_QuorumResource( 
            /* [in] */ __RPC__in_opt ISClusResource *pClusterResource) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_QuorumResource( 
            /* [retval][out] */ __RPC__deref_out_opt ISClusResource **pClusterResource) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_QuorumLogSize( 
            /* [retval][out] */ __RPC__out long *pnLogSize) = 0;
        
        virtual /* [helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_QuorumLogSize( 
            /* [in] */ long nLogSize) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_QuorumPath( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *ppPath) = 0;
        
        virtual /* [helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_QuorumPath( 
            __RPC__in BSTR pPath) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Nodes( 
            /* [retval][out] */ __RPC__deref_out_opt ISClusNodes **ppNodes) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_ResourceGroups( 
            /* [retval][out] */ __RPC__deref_out_opt ISClusResGroups **ppClusterResourceGroups) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Resources( 
            /* [retval][out] */ __RPC__deref_out_opt ISClusResources **ppClusterResources) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_ResourceTypes( 
            /* [retval][out] */ __RPC__deref_out_opt ISClusResTypes **ppResourceTypes) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Networks( 
            /* [retval][out] */ __RPC__deref_out_opt ISClusNetworks **ppNetworks) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_NetInterfaces( 
            /* [retval][out] */ __RPC__deref_out_opt ISClusNetInterfaces **ppNetInterfaces) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ISClusterVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in ISCluster * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in ISCluster * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in ISCluster * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in ISCluster * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in ISCluster * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in ISCluster * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            ISCluster * This,
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
        
        DECLSPEC_XFGVIRT(ISCluster, get_CommonProperties)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_CommonProperties )( 
            __RPC__in ISCluster * This,
            /* [retval][out] */ __RPC__deref_out_opt ISClusProperties **ppProperties);
        
        DECLSPEC_XFGVIRT(ISCluster, get_PrivateProperties)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_PrivateProperties )( 
            __RPC__in ISCluster * This,
            /* [retval][out] */ __RPC__deref_out_opt ISClusProperties **ppProperties);
        
        DECLSPEC_XFGVIRT(ISCluster, get_CommonROProperties)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_CommonROProperties )( 
            __RPC__in ISCluster * This,
            /* [retval][out] */ __RPC__deref_out_opt ISClusProperties **ppProperties);
        
        DECLSPEC_XFGVIRT(ISCluster, get_PrivateROProperties)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_PrivateROProperties )( 
            __RPC__in ISCluster * This,
            /* [retval][out] */ __RPC__deref_out_opt ISClusProperties **ppProperties);
        
        DECLSPEC_XFGVIRT(ISCluster, get_Handle)
        /* [helpstring][id][hidden][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Handle )( 
            __RPC__in ISCluster * This,
            /* [retval][out] */ __RPC__out ULONG_PTR *phandle);
        
        DECLSPEC_XFGVIRT(ISCluster, Open)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *Open )( 
            __RPC__in ISCluster * This,
            /* [in] */ __RPC__in BSTR bstrClusterName);
        
        DECLSPEC_XFGVIRT(ISCluster, get_Name)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Name )( 
            __RPC__in ISCluster * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrName);
        
        DECLSPEC_XFGVIRT(ISCluster, put_Name)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_Name )( 
            __RPC__in ISCluster * This,
            /* [in] */ __RPC__in BSTR bstrClusterName);
        
        DECLSPEC_XFGVIRT(ISCluster, get_Version)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Version )( 
            __RPC__in ISCluster * This,
            /* [retval][out] */ __RPC__deref_out_opt ISClusVersion **ppClusVersion);
        
        DECLSPEC_XFGVIRT(ISCluster, put_QuorumResource)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_QuorumResource )( 
            __RPC__in ISCluster * This,
            /* [in] */ __RPC__in_opt ISClusResource *pClusterResource);
        
        DECLSPEC_XFGVIRT(ISCluster, get_QuorumResource)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_QuorumResource )( 
            __RPC__in ISCluster * This,
            /* [retval][out] */ __RPC__deref_out_opt ISClusResource **pClusterResource);
        
        DECLSPEC_XFGVIRT(ISCluster, get_QuorumLogSize)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_QuorumLogSize )( 
            __RPC__in ISCluster * This,
            /* [retval][out] */ __RPC__out long *pnLogSize);
        
        DECLSPEC_XFGVIRT(ISCluster, put_QuorumLogSize)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_QuorumLogSize )( 
            __RPC__in ISCluster * This,
            /* [in] */ long nLogSize);
        
        DECLSPEC_XFGVIRT(ISCluster, get_QuorumPath)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_QuorumPath )( 
            __RPC__in ISCluster * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *ppPath);
        
        DECLSPEC_XFGVIRT(ISCluster, put_QuorumPath)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_QuorumPath )( 
            __RPC__in ISCluster * This,
            __RPC__in BSTR pPath);
        
        DECLSPEC_XFGVIRT(ISCluster, get_Nodes)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Nodes )( 
            __RPC__in ISCluster * This,
            /* [retval][out] */ __RPC__deref_out_opt ISClusNodes **ppNodes);
        
        DECLSPEC_XFGVIRT(ISCluster, get_ResourceGroups)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_ResourceGroups )( 
            __RPC__in ISCluster * This,
            /* [retval][out] */ __RPC__deref_out_opt ISClusResGroups **ppClusterResourceGroups);
        
        DECLSPEC_XFGVIRT(ISCluster, get_Resources)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Resources )( 
            __RPC__in ISCluster * This,
            /* [retval][out] */ __RPC__deref_out_opt ISClusResources **ppClusterResources);
        
        DECLSPEC_XFGVIRT(ISCluster, get_ResourceTypes)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_ResourceTypes )( 
            __RPC__in ISCluster * This,
            /* [retval][out] */ __RPC__deref_out_opt ISClusResTypes **ppResourceTypes);
        
        DECLSPEC_XFGVIRT(ISCluster, get_Networks)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Networks )( 
            __RPC__in ISCluster * This,
            /* [retval][out] */ __RPC__deref_out_opt ISClusNetworks **ppNetworks);
        
        DECLSPEC_XFGVIRT(ISCluster, get_NetInterfaces)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_NetInterfaces )( 
            __RPC__in ISCluster * This,
            /* [retval][out] */ __RPC__deref_out_opt ISClusNetInterfaces **ppNetInterfaces);
        
        END_INTERFACE
    } ISClusterVtbl;

    interface ISCluster
    {
        CONST_VTBL struct ISClusterVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ISCluster_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ISCluster_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ISCluster_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ISCluster_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define ISCluster_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define ISCluster_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define ISCluster_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define ISCluster_get_CommonProperties(This,ppProperties)	\
    ( (This)->lpVtbl -> get_CommonProperties(This,ppProperties) ) 

#define ISCluster_get_PrivateProperties(This,ppProperties)	\
    ( (This)->lpVtbl -> get_PrivateProperties(This,ppProperties) ) 

#define ISCluster_get_CommonROProperties(This,ppProperties)	\
    ( (This)->lpVtbl -> get_CommonROProperties(This,ppProperties) ) 

#define ISCluster_get_PrivateROProperties(This,ppProperties)	\
    ( (This)->lpVtbl -> get_PrivateROProperties(This,ppProperties) ) 

#define ISCluster_get_Handle(This,phandle)	\
    ( (This)->lpVtbl -> get_Handle(This,phandle) ) 

#define ISCluster_Open(This,bstrClusterName)	\
    ( (This)->lpVtbl -> Open(This,bstrClusterName) ) 

#define ISCluster_get_Name(This,pbstrName)	\
    ( (This)->lpVtbl -> get_Name(This,pbstrName) ) 

#define ISCluster_put_Name(This,bstrClusterName)	\
    ( (This)->lpVtbl -> put_Name(This,bstrClusterName) ) 

#define ISCluster_get_Version(This,ppClusVersion)	\
    ( (This)->lpVtbl -> get_Version(This,ppClusVersion) ) 

#define ISCluster_put_QuorumResource(This,pClusterResource)	\
    ( (This)->lpVtbl -> put_QuorumResource(This,pClusterResource) ) 

#define ISCluster_get_QuorumResource(This,pClusterResource)	\
    ( (This)->lpVtbl -> get_QuorumResource(This,pClusterResource) ) 

#define ISCluster_get_QuorumLogSize(This,pnLogSize)	\
    ( (This)->lpVtbl -> get_QuorumLogSize(This,pnLogSize) ) 

#define ISCluster_put_QuorumLogSize(This,nLogSize)	\
    ( (This)->lpVtbl -> put_QuorumLogSize(This,nLogSize) ) 

#define ISCluster_get_QuorumPath(This,ppPath)	\
    ( (This)->lpVtbl -> get_QuorumPath(This,ppPath) ) 

#define ISCluster_put_QuorumPath(This,pPath)	\
    ( (This)->lpVtbl -> put_QuorumPath(This,pPath) ) 

#define ISCluster_get_Nodes(This,ppNodes)	\
    ( (This)->lpVtbl -> get_Nodes(This,ppNodes) ) 

#define ISCluster_get_ResourceGroups(This,ppClusterResourceGroups)	\
    ( (This)->lpVtbl -> get_ResourceGroups(This,ppClusterResourceGroups) ) 

#define ISCluster_get_Resources(This,ppClusterResources)	\
    ( (This)->lpVtbl -> get_Resources(This,ppClusterResources) ) 

#define ISCluster_get_ResourceTypes(This,ppResourceTypes)	\
    ( (This)->lpVtbl -> get_ResourceTypes(This,ppResourceTypes) ) 

#define ISCluster_get_Networks(This,ppNetworks)	\
    ( (This)->lpVtbl -> get_Networks(This,ppNetworks) ) 

#define ISCluster_get_NetInterfaces(This,ppNetInterfaces)	\
    ( (This)->lpVtbl -> get_NetInterfaces(This,ppNetInterfaces) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ISCluster_INTERFACE_DEFINED__ */


#ifndef __ISClusNode_INTERFACE_DEFINED__
#define __ISClusNode_INTERFACE_DEFINED__

/* interface ISClusNode */
/* [unique][helpstring][dual][uuid][object][nonextensible][hidden][oleautomation] */ 


EXTERN_C const IID IID_ISClusNode;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("f2e606f8-2631-11d1-89f1-00a0c90d061e")
    ISClusNode : public IDispatch
    {
    public:
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_CommonProperties( 
            /* [retval][out] */ __RPC__deref_out_opt ISClusProperties **ppProperties) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_PrivateProperties( 
            /* [retval][out] */ __RPC__deref_out_opt ISClusProperties **ppProperties) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_CommonROProperties( 
            /* [retval][out] */ __RPC__deref_out_opt ISClusProperties **ppProperties) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_PrivateROProperties( 
            /* [retval][out] */ __RPC__deref_out_opt ISClusProperties **ppProperties) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Name( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrName) = 0;
        
        virtual /* [helpstring][id][hidden][propget] */ HRESULT STDMETHODCALLTYPE get_Handle( 
            /* [retval][out] */ __RPC__out ULONG_PTR *phandle) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_NodeID( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrNodeID) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_State( 
            /* [retval][out] */ __RPC__out CLUSTER_NODE_STATE *dwState) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE Pause( void) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE Resume( void) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE Evict( void) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_ResourceGroups( 
            /* [retval][out] */ __RPC__deref_out_opt ISClusResGroups **ppResourceGroups) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Cluster( 
            /* [retval][out] */ __RPC__deref_out_opt ISCluster **ppCluster) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_NetInterfaces( 
            /* [retval][out] */ __RPC__deref_out_opt ISClusNodeNetInterfaces **ppClusNetInterfaces) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ISClusNodeVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in ISClusNode * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in ISClusNode * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in ISClusNode * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in ISClusNode * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in ISClusNode * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in ISClusNode * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            ISClusNode * This,
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
        
        DECLSPEC_XFGVIRT(ISClusNode, get_CommonProperties)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_CommonProperties )( 
            __RPC__in ISClusNode * This,
            /* [retval][out] */ __RPC__deref_out_opt ISClusProperties **ppProperties);
        
        DECLSPEC_XFGVIRT(ISClusNode, get_PrivateProperties)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_PrivateProperties )( 
            __RPC__in ISClusNode * This,
            /* [retval][out] */ __RPC__deref_out_opt ISClusProperties **ppProperties);
        
        DECLSPEC_XFGVIRT(ISClusNode, get_CommonROProperties)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_CommonROProperties )( 
            __RPC__in ISClusNode * This,
            /* [retval][out] */ __RPC__deref_out_opt ISClusProperties **ppProperties);
        
        DECLSPEC_XFGVIRT(ISClusNode, get_PrivateROProperties)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_PrivateROProperties )( 
            __RPC__in ISClusNode * This,
            /* [retval][out] */ __RPC__deref_out_opt ISClusProperties **ppProperties);
        
        DECLSPEC_XFGVIRT(ISClusNode, get_Name)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Name )( 
            __RPC__in ISClusNode * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrName);
        
        DECLSPEC_XFGVIRT(ISClusNode, get_Handle)
        /* [helpstring][id][hidden][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Handle )( 
            __RPC__in ISClusNode * This,
            /* [retval][out] */ __RPC__out ULONG_PTR *phandle);
        
        DECLSPEC_XFGVIRT(ISClusNode, get_NodeID)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_NodeID )( 
            __RPC__in ISClusNode * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrNodeID);
        
        DECLSPEC_XFGVIRT(ISClusNode, get_State)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_State )( 
            __RPC__in ISClusNode * This,
            /* [retval][out] */ __RPC__out CLUSTER_NODE_STATE *dwState);
        
        DECLSPEC_XFGVIRT(ISClusNode, Pause)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *Pause )( 
            __RPC__in ISClusNode * This);
        
        DECLSPEC_XFGVIRT(ISClusNode, Resume)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *Resume )( 
            __RPC__in ISClusNode * This);
        
        DECLSPEC_XFGVIRT(ISClusNode, Evict)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *Evict )( 
            __RPC__in ISClusNode * This);
        
        DECLSPEC_XFGVIRT(ISClusNode, get_ResourceGroups)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_ResourceGroups )( 
            __RPC__in ISClusNode * This,
            /* [retval][out] */ __RPC__deref_out_opt ISClusResGroups **ppResourceGroups);
        
        DECLSPEC_XFGVIRT(ISClusNode, get_Cluster)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Cluster )( 
            __RPC__in ISClusNode * This,
            /* [retval][out] */ __RPC__deref_out_opt ISCluster **ppCluster);
        
        DECLSPEC_XFGVIRT(ISClusNode, get_NetInterfaces)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_NetInterfaces )( 
            __RPC__in ISClusNode * This,
            /* [retval][out] */ __RPC__deref_out_opt ISClusNodeNetInterfaces **ppClusNetInterfaces);
        
        END_INTERFACE
    } ISClusNodeVtbl;

    interface ISClusNode
    {
        CONST_VTBL struct ISClusNodeVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ISClusNode_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ISClusNode_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ISClusNode_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ISClusNode_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define ISClusNode_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define ISClusNode_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define ISClusNode_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define ISClusNode_get_CommonProperties(This,ppProperties)	\
    ( (This)->lpVtbl -> get_CommonProperties(This,ppProperties) ) 

#define ISClusNode_get_PrivateProperties(This,ppProperties)	\
    ( (This)->lpVtbl -> get_PrivateProperties(This,ppProperties) ) 

#define ISClusNode_get_CommonROProperties(This,ppProperties)	\
    ( (This)->lpVtbl -> get_CommonROProperties(This,ppProperties) ) 

#define ISClusNode_get_PrivateROProperties(This,ppProperties)	\
    ( (This)->lpVtbl -> get_PrivateROProperties(This,ppProperties) ) 

#define ISClusNode_get_Name(This,pbstrName)	\
    ( (This)->lpVtbl -> get_Name(This,pbstrName) ) 

#define ISClusNode_get_Handle(This,phandle)	\
    ( (This)->lpVtbl -> get_Handle(This,phandle) ) 

#define ISClusNode_get_NodeID(This,pbstrNodeID)	\
    ( (This)->lpVtbl -> get_NodeID(This,pbstrNodeID) ) 

#define ISClusNode_get_State(This,dwState)	\
    ( (This)->lpVtbl -> get_State(This,dwState) ) 

#define ISClusNode_Pause(This)	\
    ( (This)->lpVtbl -> Pause(This) ) 

#define ISClusNode_Resume(This)	\
    ( (This)->lpVtbl -> Resume(This) ) 

#define ISClusNode_Evict(This)	\
    ( (This)->lpVtbl -> Evict(This) ) 

#define ISClusNode_get_ResourceGroups(This,ppResourceGroups)	\
    ( (This)->lpVtbl -> get_ResourceGroups(This,ppResourceGroups) ) 

#define ISClusNode_get_Cluster(This,ppCluster)	\
    ( (This)->lpVtbl -> get_Cluster(This,ppCluster) ) 

#define ISClusNode_get_NetInterfaces(This,ppClusNetInterfaces)	\
    ( (This)->lpVtbl -> get_NetInterfaces(This,ppClusNetInterfaces) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ISClusNode_INTERFACE_DEFINED__ */


#ifndef __ISClusNodes_INTERFACE_DEFINED__
#define __ISClusNodes_INTERFACE_DEFINED__

/* interface ISClusNodes */
/* [unique][helpstring][dual][uuid][object][nonextensible][hidden][oleautomation] */ 


EXTERN_C const IID IID_ISClusNodes;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("f2e606fa-2631-11d1-89f1-00a0c90d061e")
    ISClusNodes : public IDispatch
    {
    public:
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Count( 
            /* [retval][out] */ __RPC__out long *plCount) = 0;
        
        virtual /* [helpstring][id][restricted][propget] */ HRESULT STDMETHODCALLTYPE get__NewEnum( 
            /* [retval][out] */ __RPC__deref_out_opt IUnknown **retval) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE Refresh( void) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Item( 
            /* [in] */ VARIANT varIndex,
            /* [retval][out] */ __RPC__deref_out_opt ISClusNode **ppNode) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ISClusNodesVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in ISClusNodes * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in ISClusNodes * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in ISClusNodes * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in ISClusNodes * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in ISClusNodes * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in ISClusNodes * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            ISClusNodes * This,
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
        
        DECLSPEC_XFGVIRT(ISClusNodes, get_Count)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Count )( 
            __RPC__in ISClusNodes * This,
            /* [retval][out] */ __RPC__out long *plCount);
        
        DECLSPEC_XFGVIRT(ISClusNodes, get__NewEnum)
        /* [helpstring][id][restricted][propget] */ HRESULT ( STDMETHODCALLTYPE *get__NewEnum )( 
            __RPC__in ISClusNodes * This,
            /* [retval][out] */ __RPC__deref_out_opt IUnknown **retval);
        
        DECLSPEC_XFGVIRT(ISClusNodes, Refresh)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *Refresh )( 
            __RPC__in ISClusNodes * This);
        
        DECLSPEC_XFGVIRT(ISClusNodes, get_Item)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Item )( 
            __RPC__in ISClusNodes * This,
            /* [in] */ VARIANT varIndex,
            /* [retval][out] */ __RPC__deref_out_opt ISClusNode **ppNode);
        
        END_INTERFACE
    } ISClusNodesVtbl;

    interface ISClusNodes
    {
        CONST_VTBL struct ISClusNodesVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ISClusNodes_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ISClusNodes_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ISClusNodes_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ISClusNodes_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define ISClusNodes_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define ISClusNodes_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define ISClusNodes_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define ISClusNodes_get_Count(This,plCount)	\
    ( (This)->lpVtbl -> get_Count(This,plCount) ) 

#define ISClusNodes_get__NewEnum(This,retval)	\
    ( (This)->lpVtbl -> get__NewEnum(This,retval) ) 

#define ISClusNodes_Refresh(This)	\
    ( (This)->lpVtbl -> Refresh(This) ) 

#define ISClusNodes_get_Item(This,varIndex,ppNode)	\
    ( (This)->lpVtbl -> get_Item(This,varIndex,ppNode) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ISClusNodes_INTERFACE_DEFINED__ */


#ifndef __ISClusNetwork_INTERFACE_DEFINED__
#define __ISClusNetwork_INTERFACE_DEFINED__

/* interface ISClusNetwork */
/* [unique][helpstring][dual][uuid][object][nonextensible][hidden][oleautomation] */ 


EXTERN_C const IID IID_ISClusNetwork;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("f2e606f2-2631-11d1-89f1-00a0c90d061e")
    ISClusNetwork : public IDispatch
    {
    public:
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_CommonProperties( 
            /* [retval][out] */ __RPC__deref_out_opt ISClusProperties **ppProperties) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_PrivateProperties( 
            /* [retval][out] */ __RPC__deref_out_opt ISClusProperties **ppProperties) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_CommonROProperties( 
            /* [retval][out] */ __RPC__deref_out_opt ISClusProperties **ppProperties) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_PrivateROProperties( 
            /* [retval][out] */ __RPC__deref_out_opt ISClusProperties **ppProperties) = 0;
        
        virtual /* [helpstring][id][hidden][propget] */ HRESULT STDMETHODCALLTYPE get_Handle( 
            /* [retval][out] */ __RPC__out ULONG_PTR *phandle) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Name( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrName) = 0;
        
        virtual /* [helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_Name( 
            /* [in] */ __RPC__in BSTR bstrNetworkName) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_NetworkID( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrNetworkID) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_State( 
            /* [retval][out] */ __RPC__out CLUSTER_NETWORK_STATE *dwState) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_NetInterfaces( 
            /* [retval][out] */ __RPC__deref_out_opt ISClusNetworkNetInterfaces **ppClusNetInterfaces) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Cluster( 
            /* [retval][out] */ __RPC__deref_out_opt ISCluster **ppCluster) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ISClusNetworkVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in ISClusNetwork * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in ISClusNetwork * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in ISClusNetwork * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in ISClusNetwork * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in ISClusNetwork * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in ISClusNetwork * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            ISClusNetwork * This,
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
        
        DECLSPEC_XFGVIRT(ISClusNetwork, get_CommonProperties)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_CommonProperties )( 
            __RPC__in ISClusNetwork * This,
            /* [retval][out] */ __RPC__deref_out_opt ISClusProperties **ppProperties);
        
        DECLSPEC_XFGVIRT(ISClusNetwork, get_PrivateProperties)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_PrivateProperties )( 
            __RPC__in ISClusNetwork * This,
            /* [retval][out] */ __RPC__deref_out_opt ISClusProperties **ppProperties);
        
        DECLSPEC_XFGVIRT(ISClusNetwork, get_CommonROProperties)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_CommonROProperties )( 
            __RPC__in ISClusNetwork * This,
            /* [retval][out] */ __RPC__deref_out_opt ISClusProperties **ppProperties);
        
        DECLSPEC_XFGVIRT(ISClusNetwork, get_PrivateROProperties)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_PrivateROProperties )( 
            __RPC__in ISClusNetwork * This,
            /* [retval][out] */ __RPC__deref_out_opt ISClusProperties **ppProperties);
        
        DECLSPEC_XFGVIRT(ISClusNetwork, get_Handle)
        /* [helpstring][id][hidden][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Handle )( 
            __RPC__in ISClusNetwork * This,
            /* [retval][out] */ __RPC__out ULONG_PTR *phandle);
        
        DECLSPEC_XFGVIRT(ISClusNetwork, get_Name)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Name )( 
            __RPC__in ISClusNetwork * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrName);
        
        DECLSPEC_XFGVIRT(ISClusNetwork, put_Name)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_Name )( 
            __RPC__in ISClusNetwork * This,
            /* [in] */ __RPC__in BSTR bstrNetworkName);
        
        DECLSPEC_XFGVIRT(ISClusNetwork, get_NetworkID)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_NetworkID )( 
            __RPC__in ISClusNetwork * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrNetworkID);
        
        DECLSPEC_XFGVIRT(ISClusNetwork, get_State)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_State )( 
            __RPC__in ISClusNetwork * This,
            /* [retval][out] */ __RPC__out CLUSTER_NETWORK_STATE *dwState);
        
        DECLSPEC_XFGVIRT(ISClusNetwork, get_NetInterfaces)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_NetInterfaces )( 
            __RPC__in ISClusNetwork * This,
            /* [retval][out] */ __RPC__deref_out_opt ISClusNetworkNetInterfaces **ppClusNetInterfaces);
        
        DECLSPEC_XFGVIRT(ISClusNetwork, get_Cluster)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Cluster )( 
            __RPC__in ISClusNetwork * This,
            /* [retval][out] */ __RPC__deref_out_opt ISCluster **ppCluster);
        
        END_INTERFACE
    } ISClusNetworkVtbl;

    interface ISClusNetwork
    {
        CONST_VTBL struct ISClusNetworkVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ISClusNetwork_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ISClusNetwork_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ISClusNetwork_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ISClusNetwork_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define ISClusNetwork_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define ISClusNetwork_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define ISClusNetwork_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define ISClusNetwork_get_CommonProperties(This,ppProperties)	\
    ( (This)->lpVtbl -> get_CommonProperties(This,ppProperties) ) 

#define ISClusNetwork_get_PrivateProperties(This,ppProperties)	\
    ( (This)->lpVtbl -> get_PrivateProperties(This,ppProperties) ) 

#define ISClusNetwork_get_CommonROProperties(This,ppProperties)	\
    ( (This)->lpVtbl -> get_CommonROProperties(This,ppProperties) ) 

#define ISClusNetwork_get_PrivateROProperties(This,ppProperties)	\
    ( (This)->lpVtbl -> get_PrivateROProperties(This,ppProperties) ) 

#define ISClusNetwork_get_Handle(This,phandle)	\
    ( (This)->lpVtbl -> get_Handle(This,phandle) ) 

#define ISClusNetwork_get_Name(This,pbstrName)	\
    ( (This)->lpVtbl -> get_Name(This,pbstrName) ) 

#define ISClusNetwork_put_Name(This,bstrNetworkName)	\
    ( (This)->lpVtbl -> put_Name(This,bstrNetworkName) ) 

#define ISClusNetwork_get_NetworkID(This,pbstrNetworkID)	\
    ( (This)->lpVtbl -> get_NetworkID(This,pbstrNetworkID) ) 

#define ISClusNetwork_get_State(This,dwState)	\
    ( (This)->lpVtbl -> get_State(This,dwState) ) 

#define ISClusNetwork_get_NetInterfaces(This,ppClusNetInterfaces)	\
    ( (This)->lpVtbl -> get_NetInterfaces(This,ppClusNetInterfaces) ) 

#define ISClusNetwork_get_Cluster(This,ppCluster)	\
    ( (This)->lpVtbl -> get_Cluster(This,ppCluster) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ISClusNetwork_INTERFACE_DEFINED__ */


#ifndef __ISClusNetworks_INTERFACE_DEFINED__
#define __ISClusNetworks_INTERFACE_DEFINED__

/* interface ISClusNetworks */
/* [unique][helpstring][dual][uuid][object][nonextensible][hidden][oleautomation] */ 


EXTERN_C const IID IID_ISClusNetworks;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("f2e606f4-2631-11d1-89f1-00a0c90d061e")
    ISClusNetworks : public IDispatch
    {
    public:
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Count( 
            /* [retval][out] */ __RPC__out long *plCount) = 0;
        
        virtual /* [helpstring][id][restricted][propget] */ HRESULT STDMETHODCALLTYPE get__NewEnum( 
            /* [retval][out] */ __RPC__deref_out_opt IUnknown **retval) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE Refresh( void) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Item( 
            /* [in] */ VARIANT varIndex,
            /* [retval][out] */ __RPC__deref_out_opt ISClusNetwork **ppClusNetwork) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ISClusNetworksVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in ISClusNetworks * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in ISClusNetworks * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in ISClusNetworks * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in ISClusNetworks * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in ISClusNetworks * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in ISClusNetworks * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            ISClusNetworks * This,
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
        
        DECLSPEC_XFGVIRT(ISClusNetworks, get_Count)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Count )( 
            __RPC__in ISClusNetworks * This,
            /* [retval][out] */ __RPC__out long *plCount);
        
        DECLSPEC_XFGVIRT(ISClusNetworks, get__NewEnum)
        /* [helpstring][id][restricted][propget] */ HRESULT ( STDMETHODCALLTYPE *get__NewEnum )( 
            __RPC__in ISClusNetworks * This,
            /* [retval][out] */ __RPC__deref_out_opt IUnknown **retval);
        
        DECLSPEC_XFGVIRT(ISClusNetworks, Refresh)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *Refresh )( 
            __RPC__in ISClusNetworks * This);
        
        DECLSPEC_XFGVIRT(ISClusNetworks, get_Item)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Item )( 
            __RPC__in ISClusNetworks * This,
            /* [in] */ VARIANT varIndex,
            /* [retval][out] */ __RPC__deref_out_opt ISClusNetwork **ppClusNetwork);
        
        END_INTERFACE
    } ISClusNetworksVtbl;

    interface ISClusNetworks
    {
        CONST_VTBL struct ISClusNetworksVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ISClusNetworks_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ISClusNetworks_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ISClusNetworks_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ISClusNetworks_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define ISClusNetworks_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define ISClusNetworks_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define ISClusNetworks_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define ISClusNetworks_get_Count(This,plCount)	\
    ( (This)->lpVtbl -> get_Count(This,plCount) ) 

#define ISClusNetworks_get__NewEnum(This,retval)	\
    ( (This)->lpVtbl -> get__NewEnum(This,retval) ) 

#define ISClusNetworks_Refresh(This)	\
    ( (This)->lpVtbl -> Refresh(This) ) 

#define ISClusNetworks_get_Item(This,varIndex,ppClusNetwork)	\
    ( (This)->lpVtbl -> get_Item(This,varIndex,ppClusNetwork) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ISClusNetworks_INTERFACE_DEFINED__ */


#ifndef __ISClusNetInterface_INTERFACE_DEFINED__
#define __ISClusNetInterface_INTERFACE_DEFINED__

/* interface ISClusNetInterface */
/* [unique][helpstring][dual][uuid][object][nonextensible][hidden][oleautomation] */ 


EXTERN_C const IID IID_ISClusNetInterface;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("f2e606ee-2631-11d1-89f1-00a0c90d061e")
    ISClusNetInterface : public IDispatch
    {
    public:
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_CommonProperties( 
            /* [retval][out] */ __RPC__deref_out_opt ISClusProperties **ppProperties) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_PrivateProperties( 
            /* [retval][out] */ __RPC__deref_out_opt ISClusProperties **ppProperties) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_CommonROProperties( 
            /* [retval][out] */ __RPC__deref_out_opt ISClusProperties **ppProperties) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_PrivateROProperties( 
            /* [retval][out] */ __RPC__deref_out_opt ISClusProperties **ppProperties) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Name( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrName) = 0;
        
        virtual /* [helpstring][id][hidden][propget] */ HRESULT STDMETHODCALLTYPE get_Handle( 
            /* [retval][out] */ __RPC__out ULONG_PTR *phandle) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_State( 
            /* [retval][out] */ __RPC__out CLUSTER_NETINTERFACE_STATE *dwState) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Cluster( 
            /* [retval][out] */ __RPC__deref_out_opt ISCluster **ppCluster) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ISClusNetInterfaceVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in ISClusNetInterface * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in ISClusNetInterface * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in ISClusNetInterface * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in ISClusNetInterface * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in ISClusNetInterface * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in ISClusNetInterface * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            ISClusNetInterface * This,
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
        
        DECLSPEC_XFGVIRT(ISClusNetInterface, get_CommonProperties)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_CommonProperties )( 
            __RPC__in ISClusNetInterface * This,
            /* [retval][out] */ __RPC__deref_out_opt ISClusProperties **ppProperties);
        
        DECLSPEC_XFGVIRT(ISClusNetInterface, get_PrivateProperties)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_PrivateProperties )( 
            __RPC__in ISClusNetInterface * This,
            /* [retval][out] */ __RPC__deref_out_opt ISClusProperties **ppProperties);
        
        DECLSPEC_XFGVIRT(ISClusNetInterface, get_CommonROProperties)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_CommonROProperties )( 
            __RPC__in ISClusNetInterface * This,
            /* [retval][out] */ __RPC__deref_out_opt ISClusProperties **ppProperties);
        
        DECLSPEC_XFGVIRT(ISClusNetInterface, get_PrivateROProperties)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_PrivateROProperties )( 
            __RPC__in ISClusNetInterface * This,
            /* [retval][out] */ __RPC__deref_out_opt ISClusProperties **ppProperties);
        
        DECLSPEC_XFGVIRT(ISClusNetInterface, get_Name)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Name )( 
            __RPC__in ISClusNetInterface * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrName);
        
        DECLSPEC_XFGVIRT(ISClusNetInterface, get_Handle)
        /* [helpstring][id][hidden][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Handle )( 
            __RPC__in ISClusNetInterface * This,
            /* [retval][out] */ __RPC__out ULONG_PTR *phandle);
        
        DECLSPEC_XFGVIRT(ISClusNetInterface, get_State)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_State )( 
            __RPC__in ISClusNetInterface * This,
            /* [retval][out] */ __RPC__out CLUSTER_NETINTERFACE_STATE *dwState);
        
        DECLSPEC_XFGVIRT(ISClusNetInterface, get_Cluster)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Cluster )( 
            __RPC__in ISClusNetInterface * This,
            /* [retval][out] */ __RPC__deref_out_opt ISCluster **ppCluster);
        
        END_INTERFACE
    } ISClusNetInterfaceVtbl;

    interface ISClusNetInterface
    {
        CONST_VTBL struct ISClusNetInterfaceVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ISClusNetInterface_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ISClusNetInterface_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ISClusNetInterface_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ISClusNetInterface_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define ISClusNetInterface_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define ISClusNetInterface_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define ISClusNetInterface_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define ISClusNetInterface_get_CommonProperties(This,ppProperties)	\
    ( (This)->lpVtbl -> get_CommonProperties(This,ppProperties) ) 

#define ISClusNetInterface_get_PrivateProperties(This,ppProperties)	\
    ( (This)->lpVtbl -> get_PrivateProperties(This,ppProperties) ) 

#define ISClusNetInterface_get_CommonROProperties(This,ppProperties)	\
    ( (This)->lpVtbl -> get_CommonROProperties(This,ppProperties) ) 

#define ISClusNetInterface_get_PrivateROProperties(This,ppProperties)	\
    ( (This)->lpVtbl -> get_PrivateROProperties(This,ppProperties) ) 

#define ISClusNetInterface_get_Name(This,pbstrName)	\
    ( (This)->lpVtbl -> get_Name(This,pbstrName) ) 

#define ISClusNetInterface_get_Handle(This,phandle)	\
    ( (This)->lpVtbl -> get_Handle(This,phandle) ) 

#define ISClusNetInterface_get_State(This,dwState)	\
    ( (This)->lpVtbl -> get_State(This,dwState) ) 

#define ISClusNetInterface_get_Cluster(This,ppCluster)	\
    ( (This)->lpVtbl -> get_Cluster(This,ppCluster) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ISClusNetInterface_INTERFACE_DEFINED__ */


#ifndef __ISClusNetInterfaces_INTERFACE_DEFINED__
#define __ISClusNetInterfaces_INTERFACE_DEFINED__

/* interface ISClusNetInterfaces */
/* [unique][helpstring][dual][uuid][object][nonextensible][hidden][oleautomation] */ 


EXTERN_C const IID IID_ISClusNetInterfaces;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("f2e606f0-2631-11d1-89f1-00a0c90d061e")
    ISClusNetInterfaces : public IDispatch
    {
    public:
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Count( 
            /* [retval][out] */ __RPC__out long *plCount) = 0;
        
        virtual /* [helpstring][id][restricted][propget] */ HRESULT STDMETHODCALLTYPE get__NewEnum( 
            /* [retval][out] */ __RPC__deref_out_opt IUnknown **retval) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE Refresh( void) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Item( 
            /* [in] */ VARIANT varIndex,
            /* [retval][out] */ __RPC__deref_out_opt ISClusNetInterface **ppClusNetInterface) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ISClusNetInterfacesVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in ISClusNetInterfaces * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in ISClusNetInterfaces * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in ISClusNetInterfaces * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in ISClusNetInterfaces * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in ISClusNetInterfaces * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in ISClusNetInterfaces * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            ISClusNetInterfaces * This,
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
        
        DECLSPEC_XFGVIRT(ISClusNetInterfaces, get_Count)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Count )( 
            __RPC__in ISClusNetInterfaces * This,
            /* [retval][out] */ __RPC__out long *plCount);
        
        DECLSPEC_XFGVIRT(ISClusNetInterfaces, get__NewEnum)
        /* [helpstring][id][restricted][propget] */ HRESULT ( STDMETHODCALLTYPE *get__NewEnum )( 
            __RPC__in ISClusNetInterfaces * This,
            /* [retval][out] */ __RPC__deref_out_opt IUnknown **retval);
        
        DECLSPEC_XFGVIRT(ISClusNetInterfaces, Refresh)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *Refresh )( 
            __RPC__in ISClusNetInterfaces * This);
        
        DECLSPEC_XFGVIRT(ISClusNetInterfaces, get_Item)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Item )( 
            __RPC__in ISClusNetInterfaces * This,
            /* [in] */ VARIANT varIndex,
            /* [retval][out] */ __RPC__deref_out_opt ISClusNetInterface **ppClusNetInterface);
        
        END_INTERFACE
    } ISClusNetInterfacesVtbl;

    interface ISClusNetInterfaces
    {
        CONST_VTBL struct ISClusNetInterfacesVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ISClusNetInterfaces_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ISClusNetInterfaces_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ISClusNetInterfaces_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ISClusNetInterfaces_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define ISClusNetInterfaces_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define ISClusNetInterfaces_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define ISClusNetInterfaces_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define ISClusNetInterfaces_get_Count(This,plCount)	\
    ( (This)->lpVtbl -> get_Count(This,plCount) ) 

#define ISClusNetInterfaces_get__NewEnum(This,retval)	\
    ( (This)->lpVtbl -> get__NewEnum(This,retval) ) 

#define ISClusNetInterfaces_Refresh(This)	\
    ( (This)->lpVtbl -> Refresh(This) ) 

#define ISClusNetInterfaces_get_Item(This,varIndex,ppClusNetInterface)	\
    ( (This)->lpVtbl -> get_Item(This,varIndex,ppClusNetInterface) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ISClusNetInterfaces_INTERFACE_DEFINED__ */


#ifndef __ISClusNodeNetInterfaces_INTERFACE_DEFINED__
#define __ISClusNodeNetInterfaces_INTERFACE_DEFINED__

/* interface ISClusNodeNetInterfaces */
/* [unique][helpstring][dual][uuid][object][nonextensible][hidden][oleautomation] */ 


EXTERN_C const IID IID_ISClusNodeNetInterfaces;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("f2e606fc-2631-11d1-89f1-00a0c90d061e")
    ISClusNodeNetInterfaces : public IDispatch
    {
    public:
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Count( 
            /* [retval][out] */ __RPC__out long *plCount) = 0;
        
        virtual /* [helpstring][id][restricted][propget] */ HRESULT STDMETHODCALLTYPE get__NewEnum( 
            /* [retval][out] */ __RPC__deref_out_opt IUnknown **retval) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE Refresh( void) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Item( 
            /* [in] */ VARIANT varIndex,
            /* [retval][out] */ __RPC__deref_out_opt ISClusNetInterface **ppClusNetInterface) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ISClusNodeNetInterfacesVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in ISClusNodeNetInterfaces * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in ISClusNodeNetInterfaces * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in ISClusNodeNetInterfaces * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in ISClusNodeNetInterfaces * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in ISClusNodeNetInterfaces * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in ISClusNodeNetInterfaces * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            ISClusNodeNetInterfaces * This,
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
        
        DECLSPEC_XFGVIRT(ISClusNodeNetInterfaces, get_Count)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Count )( 
            __RPC__in ISClusNodeNetInterfaces * This,
            /* [retval][out] */ __RPC__out long *plCount);
        
        DECLSPEC_XFGVIRT(ISClusNodeNetInterfaces, get__NewEnum)
        /* [helpstring][id][restricted][propget] */ HRESULT ( STDMETHODCALLTYPE *get__NewEnum )( 
            __RPC__in ISClusNodeNetInterfaces * This,
            /* [retval][out] */ __RPC__deref_out_opt IUnknown **retval);
        
        DECLSPEC_XFGVIRT(ISClusNodeNetInterfaces, Refresh)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *Refresh )( 
            __RPC__in ISClusNodeNetInterfaces * This);
        
        DECLSPEC_XFGVIRT(ISClusNodeNetInterfaces, get_Item)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Item )( 
            __RPC__in ISClusNodeNetInterfaces * This,
            /* [in] */ VARIANT varIndex,
            /* [retval][out] */ __RPC__deref_out_opt ISClusNetInterface **ppClusNetInterface);
        
        END_INTERFACE
    } ISClusNodeNetInterfacesVtbl;

    interface ISClusNodeNetInterfaces
    {
        CONST_VTBL struct ISClusNodeNetInterfacesVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ISClusNodeNetInterfaces_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ISClusNodeNetInterfaces_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ISClusNodeNetInterfaces_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ISClusNodeNetInterfaces_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define ISClusNodeNetInterfaces_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define ISClusNodeNetInterfaces_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define ISClusNodeNetInterfaces_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define ISClusNodeNetInterfaces_get_Count(This,plCount)	\
    ( (This)->lpVtbl -> get_Count(This,plCount) ) 

#define ISClusNodeNetInterfaces_get__NewEnum(This,retval)	\
    ( (This)->lpVtbl -> get__NewEnum(This,retval) ) 

#define ISClusNodeNetInterfaces_Refresh(This)	\
    ( (This)->lpVtbl -> Refresh(This) ) 

#define ISClusNodeNetInterfaces_get_Item(This,varIndex,ppClusNetInterface)	\
    ( (This)->lpVtbl -> get_Item(This,varIndex,ppClusNetInterface) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ISClusNodeNetInterfaces_INTERFACE_DEFINED__ */


#ifndef __ISClusNetworkNetInterfaces_INTERFACE_DEFINED__
#define __ISClusNetworkNetInterfaces_INTERFACE_DEFINED__

/* interface ISClusNetworkNetInterfaces */
/* [unique][helpstring][dual][uuid][object][nonextensible][hidden][oleautomation] */ 


EXTERN_C const IID IID_ISClusNetworkNetInterfaces;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("f2e606f6-2631-11d1-89f1-00a0c90d061e")
    ISClusNetworkNetInterfaces : public IDispatch
    {
    public:
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Count( 
            /* [retval][out] */ __RPC__out long *plCount) = 0;
        
        virtual /* [helpstring][id][restricted][propget] */ HRESULT STDMETHODCALLTYPE get__NewEnum( 
            /* [retval][out] */ __RPC__deref_out_opt IUnknown **retval) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE Refresh( void) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Item( 
            /* [in] */ VARIANT varIndex,
            /* [retval][out] */ __RPC__deref_out_opt ISClusNetInterface **ppClusNetInterface) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ISClusNetworkNetInterfacesVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in ISClusNetworkNetInterfaces * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in ISClusNetworkNetInterfaces * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in ISClusNetworkNetInterfaces * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in ISClusNetworkNetInterfaces * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in ISClusNetworkNetInterfaces * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in ISClusNetworkNetInterfaces * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            ISClusNetworkNetInterfaces * This,
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
        
        DECLSPEC_XFGVIRT(ISClusNetworkNetInterfaces, get_Count)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Count )( 
            __RPC__in ISClusNetworkNetInterfaces * This,
            /* [retval][out] */ __RPC__out long *plCount);
        
        DECLSPEC_XFGVIRT(ISClusNetworkNetInterfaces, get__NewEnum)
        /* [helpstring][id][restricted][propget] */ HRESULT ( STDMETHODCALLTYPE *get__NewEnum )( 
            __RPC__in ISClusNetworkNetInterfaces * This,
            /* [retval][out] */ __RPC__deref_out_opt IUnknown **retval);
        
        DECLSPEC_XFGVIRT(ISClusNetworkNetInterfaces, Refresh)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *Refresh )( 
            __RPC__in ISClusNetworkNetInterfaces * This);
        
        DECLSPEC_XFGVIRT(ISClusNetworkNetInterfaces, get_Item)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Item )( 
            __RPC__in ISClusNetworkNetInterfaces * This,
            /* [in] */ VARIANT varIndex,
            /* [retval][out] */ __RPC__deref_out_opt ISClusNetInterface **ppClusNetInterface);
        
        END_INTERFACE
    } ISClusNetworkNetInterfacesVtbl;

    interface ISClusNetworkNetInterfaces
    {
        CONST_VTBL struct ISClusNetworkNetInterfacesVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ISClusNetworkNetInterfaces_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ISClusNetworkNetInterfaces_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ISClusNetworkNetInterfaces_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ISClusNetworkNetInterfaces_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define ISClusNetworkNetInterfaces_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define ISClusNetworkNetInterfaces_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define ISClusNetworkNetInterfaces_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define ISClusNetworkNetInterfaces_get_Count(This,plCount)	\
    ( (This)->lpVtbl -> get_Count(This,plCount) ) 

#define ISClusNetworkNetInterfaces_get__NewEnum(This,retval)	\
    ( (This)->lpVtbl -> get__NewEnum(This,retval) ) 

#define ISClusNetworkNetInterfaces_Refresh(This)	\
    ( (This)->lpVtbl -> Refresh(This) ) 

#define ISClusNetworkNetInterfaces_get_Item(This,varIndex,ppClusNetInterface)	\
    ( (This)->lpVtbl -> get_Item(This,varIndex,ppClusNetInterface) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ISClusNetworkNetInterfaces_INTERFACE_DEFINED__ */


#ifndef __ISClusResGroup_INTERFACE_DEFINED__
#define __ISClusResGroup_INTERFACE_DEFINED__

/* interface ISClusResGroup */
/* [unique][helpstring][dual][uuid][object][nonextensible][hidden][oleautomation] */ 


EXTERN_C const IID IID_ISClusResGroup;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("f2e60706-2631-11d1-89f1-00a0c90d061e")
    ISClusResGroup : public IDispatch
    {
    public:
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_CommonProperties( 
            /* [retval][out] */ __RPC__deref_out_opt ISClusProperties **ppProperties) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_PrivateProperties( 
            /* [retval][out] */ __RPC__deref_out_opt ISClusProperties **ppProperties) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_CommonROProperties( 
            /* [retval][out] */ __RPC__deref_out_opt ISClusProperties **ppProperties) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_PrivateROProperties( 
            /* [retval][out] */ __RPC__deref_out_opt ISClusProperties **ppProperties) = 0;
        
        virtual /* [helpstring][id][hidden][propget] */ HRESULT STDMETHODCALLTYPE get_Handle( 
            /* [retval][out] */ __RPC__out ULONG_PTR *phandle) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Name( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrName) = 0;
        
        virtual /* [helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_Name( 
            /* [in] */ __RPC__in BSTR bstrGroupName) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_State( 
            /* [retval][out] */ __RPC__out CLUSTER_GROUP_STATE *dwState) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_OwnerNode( 
            /* [retval][out] */ __RPC__deref_out_opt ISClusNode **ppOwnerNode) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Resources( 
            /* [retval][out] */ __RPC__deref_out_opt ISClusResGroupResources **ppClusterGroupResources) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_PreferredOwnerNodes( 
            /* [retval][out] */ __RPC__deref_out_opt ISClusResGroupPreferredOwnerNodes **ppOwnerNodes) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE Delete( void) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE Online( 
            /* [in] */ VARIANT varTimeout,
            /* [optional][in] */ VARIANT varNode,
            /* [retval][out] */ __RPC__out VARIANT *pvarPending) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE Move( 
            /* [in] */ VARIANT varTimeout,
            /* [optional][in] */ VARIANT varNode,
            /* [retval][out] */ __RPC__out VARIANT *pvarPending) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE Offline( 
            /* [in] */ VARIANT varTimeout,
            /* [retval][out] */ __RPC__out VARIANT *pvarPending) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Cluster( 
            /* [retval][out] */ __RPC__deref_out_opt ISCluster **ppCluster) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ISClusResGroupVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in ISClusResGroup * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in ISClusResGroup * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in ISClusResGroup * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in ISClusResGroup * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in ISClusResGroup * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in ISClusResGroup * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            ISClusResGroup * This,
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
        
        DECLSPEC_XFGVIRT(ISClusResGroup, get_CommonProperties)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_CommonProperties )( 
            __RPC__in ISClusResGroup * This,
            /* [retval][out] */ __RPC__deref_out_opt ISClusProperties **ppProperties);
        
        DECLSPEC_XFGVIRT(ISClusResGroup, get_PrivateProperties)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_PrivateProperties )( 
            __RPC__in ISClusResGroup * This,
            /* [retval][out] */ __RPC__deref_out_opt ISClusProperties **ppProperties);
        
        DECLSPEC_XFGVIRT(ISClusResGroup, get_CommonROProperties)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_CommonROProperties )( 
            __RPC__in ISClusResGroup * This,
            /* [retval][out] */ __RPC__deref_out_opt ISClusProperties **ppProperties);
        
        DECLSPEC_XFGVIRT(ISClusResGroup, get_PrivateROProperties)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_PrivateROProperties )( 
            __RPC__in ISClusResGroup * This,
            /* [retval][out] */ __RPC__deref_out_opt ISClusProperties **ppProperties);
        
        DECLSPEC_XFGVIRT(ISClusResGroup, get_Handle)
        /* [helpstring][id][hidden][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Handle )( 
            __RPC__in ISClusResGroup * This,
            /* [retval][out] */ __RPC__out ULONG_PTR *phandle);
        
        DECLSPEC_XFGVIRT(ISClusResGroup, get_Name)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Name )( 
            __RPC__in ISClusResGroup * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrName);
        
        DECLSPEC_XFGVIRT(ISClusResGroup, put_Name)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_Name )( 
            __RPC__in ISClusResGroup * This,
            /* [in] */ __RPC__in BSTR bstrGroupName);
        
        DECLSPEC_XFGVIRT(ISClusResGroup, get_State)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_State )( 
            __RPC__in ISClusResGroup * This,
            /* [retval][out] */ __RPC__out CLUSTER_GROUP_STATE *dwState);
        
        DECLSPEC_XFGVIRT(ISClusResGroup, get_OwnerNode)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_OwnerNode )( 
            __RPC__in ISClusResGroup * This,
            /* [retval][out] */ __RPC__deref_out_opt ISClusNode **ppOwnerNode);
        
        DECLSPEC_XFGVIRT(ISClusResGroup, get_Resources)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Resources )( 
            __RPC__in ISClusResGroup * This,
            /* [retval][out] */ __RPC__deref_out_opt ISClusResGroupResources **ppClusterGroupResources);
        
        DECLSPEC_XFGVIRT(ISClusResGroup, get_PreferredOwnerNodes)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_PreferredOwnerNodes )( 
            __RPC__in ISClusResGroup * This,
            /* [retval][out] */ __RPC__deref_out_opt ISClusResGroupPreferredOwnerNodes **ppOwnerNodes);
        
        DECLSPEC_XFGVIRT(ISClusResGroup, Delete)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *Delete )( 
            __RPC__in ISClusResGroup * This);
        
        DECLSPEC_XFGVIRT(ISClusResGroup, Online)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *Online )( 
            __RPC__in ISClusResGroup * This,
            /* [in] */ VARIANT varTimeout,
            /* [optional][in] */ VARIANT varNode,
            /* [retval][out] */ __RPC__out VARIANT *pvarPending);
        
        DECLSPEC_XFGVIRT(ISClusResGroup, Move)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *Move )( 
            __RPC__in ISClusResGroup * This,
            /* [in] */ VARIANT varTimeout,
            /* [optional][in] */ VARIANT varNode,
            /* [retval][out] */ __RPC__out VARIANT *pvarPending);
        
        DECLSPEC_XFGVIRT(ISClusResGroup, Offline)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *Offline )( 
            __RPC__in ISClusResGroup * This,
            /* [in] */ VARIANT varTimeout,
            /* [retval][out] */ __RPC__out VARIANT *pvarPending);
        
        DECLSPEC_XFGVIRT(ISClusResGroup, get_Cluster)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Cluster )( 
            __RPC__in ISClusResGroup * This,
            /* [retval][out] */ __RPC__deref_out_opt ISCluster **ppCluster);
        
        END_INTERFACE
    } ISClusResGroupVtbl;

    interface ISClusResGroup
    {
        CONST_VTBL struct ISClusResGroupVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ISClusResGroup_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ISClusResGroup_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ISClusResGroup_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ISClusResGroup_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define ISClusResGroup_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define ISClusResGroup_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define ISClusResGroup_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define ISClusResGroup_get_CommonProperties(This,ppProperties)	\
    ( (This)->lpVtbl -> get_CommonProperties(This,ppProperties) ) 

#define ISClusResGroup_get_PrivateProperties(This,ppProperties)	\
    ( (This)->lpVtbl -> get_PrivateProperties(This,ppProperties) ) 

#define ISClusResGroup_get_CommonROProperties(This,ppProperties)	\
    ( (This)->lpVtbl -> get_CommonROProperties(This,ppProperties) ) 

#define ISClusResGroup_get_PrivateROProperties(This,ppProperties)	\
    ( (This)->lpVtbl -> get_PrivateROProperties(This,ppProperties) ) 

#define ISClusResGroup_get_Handle(This,phandle)	\
    ( (This)->lpVtbl -> get_Handle(This,phandle) ) 

#define ISClusResGroup_get_Name(This,pbstrName)	\
    ( (This)->lpVtbl -> get_Name(This,pbstrName) ) 

#define ISClusResGroup_put_Name(This,bstrGroupName)	\
    ( (This)->lpVtbl -> put_Name(This,bstrGroupName) ) 

#define ISClusResGroup_get_State(This,dwState)	\
    ( (This)->lpVtbl -> get_State(This,dwState) ) 

#define ISClusResGroup_get_OwnerNode(This,ppOwnerNode)	\
    ( (This)->lpVtbl -> get_OwnerNode(This,ppOwnerNode) ) 

#define ISClusResGroup_get_Resources(This,ppClusterGroupResources)	\
    ( (This)->lpVtbl -> get_Resources(This,ppClusterGroupResources) ) 

#define ISClusResGroup_get_PreferredOwnerNodes(This,ppOwnerNodes)	\
    ( (This)->lpVtbl -> get_PreferredOwnerNodes(This,ppOwnerNodes) ) 

#define ISClusResGroup_Delete(This)	\
    ( (This)->lpVtbl -> Delete(This) ) 

#define ISClusResGroup_Online(This,varTimeout,varNode,pvarPending)	\
    ( (This)->lpVtbl -> Online(This,varTimeout,varNode,pvarPending) ) 

#define ISClusResGroup_Move(This,varTimeout,varNode,pvarPending)	\
    ( (This)->lpVtbl -> Move(This,varTimeout,varNode,pvarPending) ) 

#define ISClusResGroup_Offline(This,varTimeout,pvarPending)	\
    ( (This)->lpVtbl -> Offline(This,varTimeout,pvarPending) ) 

#define ISClusResGroup_get_Cluster(This,ppCluster)	\
    ( (This)->lpVtbl -> get_Cluster(This,ppCluster) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ISClusResGroup_INTERFACE_DEFINED__ */


#ifndef __ISClusResGroups_INTERFACE_DEFINED__
#define __ISClusResGroups_INTERFACE_DEFINED__

/* interface ISClusResGroups */
/* [unique][helpstring][dual][uuid][object][nonextensible][hidden][oleautomation] */ 


EXTERN_C const IID IID_ISClusResGroups;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("f2e60708-2631-11d1-89f1-00a0c90d061e")
    ISClusResGroups : public IDispatch
    {
    public:
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Count( 
            /* [retval][out] */ __RPC__out long *plCount) = 0;
        
        virtual /* [helpstring][id][restricted][propget] */ HRESULT STDMETHODCALLTYPE get__NewEnum( 
            /* [retval][out] */ __RPC__deref_out_opt IUnknown **retval) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE Refresh( void) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Item( 
            /* [in] */ VARIANT varIndex,
            /* [retval][out] */ __RPC__deref_out_opt ISClusResGroup **ppClusResGroup) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE CreateItem( 
            /* [in] */ __RPC__in BSTR bstrResourceGroupName,
            /* [retval][out] */ __RPC__deref_out_opt ISClusResGroup **ppResourceGroup) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE DeleteItem( 
            /* [in] */ VARIANT varIndex) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ISClusResGroupsVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in ISClusResGroups * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in ISClusResGroups * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in ISClusResGroups * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in ISClusResGroups * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in ISClusResGroups * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in ISClusResGroups * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            ISClusResGroups * This,
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
        
        DECLSPEC_XFGVIRT(ISClusResGroups, get_Count)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Count )( 
            __RPC__in ISClusResGroups * This,
            /* [retval][out] */ __RPC__out long *plCount);
        
        DECLSPEC_XFGVIRT(ISClusResGroups, get__NewEnum)
        /* [helpstring][id][restricted][propget] */ HRESULT ( STDMETHODCALLTYPE *get__NewEnum )( 
            __RPC__in ISClusResGroups * This,
            /* [retval][out] */ __RPC__deref_out_opt IUnknown **retval);
        
        DECLSPEC_XFGVIRT(ISClusResGroups, Refresh)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *Refresh )( 
            __RPC__in ISClusResGroups * This);
        
        DECLSPEC_XFGVIRT(ISClusResGroups, get_Item)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Item )( 
            __RPC__in ISClusResGroups * This,
            /* [in] */ VARIANT varIndex,
            /* [retval][out] */ __RPC__deref_out_opt ISClusResGroup **ppClusResGroup);
        
        DECLSPEC_XFGVIRT(ISClusResGroups, CreateItem)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *CreateItem )( 
            __RPC__in ISClusResGroups * This,
            /* [in] */ __RPC__in BSTR bstrResourceGroupName,
            /* [retval][out] */ __RPC__deref_out_opt ISClusResGroup **ppResourceGroup);
        
        DECLSPEC_XFGVIRT(ISClusResGroups, DeleteItem)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *DeleteItem )( 
            __RPC__in ISClusResGroups * This,
            /* [in] */ VARIANT varIndex);
        
        END_INTERFACE
    } ISClusResGroupsVtbl;

    interface ISClusResGroups
    {
        CONST_VTBL struct ISClusResGroupsVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ISClusResGroups_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ISClusResGroups_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ISClusResGroups_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ISClusResGroups_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define ISClusResGroups_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define ISClusResGroups_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define ISClusResGroups_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define ISClusResGroups_get_Count(This,plCount)	\
    ( (This)->lpVtbl -> get_Count(This,plCount) ) 

#define ISClusResGroups_get__NewEnum(This,retval)	\
    ( (This)->lpVtbl -> get__NewEnum(This,retval) ) 

#define ISClusResGroups_Refresh(This)	\
    ( (This)->lpVtbl -> Refresh(This) ) 

#define ISClusResGroups_get_Item(This,varIndex,ppClusResGroup)	\
    ( (This)->lpVtbl -> get_Item(This,varIndex,ppClusResGroup) ) 

#define ISClusResGroups_CreateItem(This,bstrResourceGroupName,ppResourceGroup)	\
    ( (This)->lpVtbl -> CreateItem(This,bstrResourceGroupName,ppResourceGroup) ) 

#define ISClusResGroups_DeleteItem(This,varIndex)	\
    ( (This)->lpVtbl -> DeleteItem(This,varIndex) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ISClusResGroups_INTERFACE_DEFINED__ */


#ifndef __ISClusResource_INTERFACE_DEFINED__
#define __ISClusResource_INTERFACE_DEFINED__

/* interface ISClusResource */
/* [unique][helpstring][dual][uuid][object][nonextensible][hidden][oleautomation] */ 


EXTERN_C const IID IID_ISClusResource;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("f2e6070a-2631-11d1-89f1-00a0c90d061e")
    ISClusResource : public IDispatch
    {
    public:
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_CommonProperties( 
            /* [retval][out] */ __RPC__deref_out_opt ISClusProperties **ppProperties) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_PrivateProperties( 
            /* [retval][out] */ __RPC__deref_out_opt ISClusProperties **ppProperties) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_CommonROProperties( 
            /* [retval][out] */ __RPC__deref_out_opt ISClusProperties **ppProperties) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_PrivateROProperties( 
            /* [retval][out] */ __RPC__deref_out_opt ISClusProperties **ppProperties) = 0;
        
        virtual /* [helpstring][id][hidden][propget] */ HRESULT STDMETHODCALLTYPE get_Handle( 
            /* [retval][out] */ __RPC__out ULONG_PTR *phandle) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Name( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrName) = 0;
        
        virtual /* [helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_Name( 
            /* [in] */ __RPC__in BSTR bstrResourceName) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_State( 
            /* [retval][out] */ __RPC__out CLUSTER_RESOURCE_STATE *dwState) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_CoreFlag( 
            /* [retval][out] */ __RPC__out CLUS_FLAGS *dwCoreFlag) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE BecomeQuorumResource( 
            /* [in] */ __RPC__in BSTR bstrDevicePath,
            /* [in] */ long lMaxLogSize) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE Delete( void) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE Fail( void) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE Online( 
            /* [in] */ long nTimeout,
            /* [retval][out] */ __RPC__out VARIANT *pvarPending) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE Offline( 
            /* [in] */ long nTimeout,
            /* [retval][out] */ __RPC__out VARIANT *pvarPending) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE ChangeResourceGroup( 
            /* [in] */ __RPC__in_opt ISClusResGroup *pResourceGroup) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE AddResourceNode( 
            /* [in] */ __RPC__in_opt ISClusNode *pNode) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE RemoveResourceNode( 
            /* [in] */ __RPC__in_opt ISClusNode *pNode) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE CanResourceBeDependent( 
            /* [in] */ __RPC__in_opt ISClusResource *pResource,
            /* [retval][out] */ __RPC__out VARIANT *pvarDependent) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_PossibleOwnerNodes( 
            /* [retval][out] */ __RPC__deref_out_opt ISClusResPossibleOwnerNodes **ppOwnerNodes) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Dependencies( 
            /* [retval][out] */ __RPC__deref_out_opt ISClusResDependencies **ppResDependencies) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Dependents( 
            /* [retval][out] */ __RPC__deref_out_opt ISClusResDependents **ppResDependents) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Group( 
            /* [retval][out] */ __RPC__deref_out_opt ISClusResGroup **ppResGroup) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_OwnerNode( 
            /* [retval][out] */ __RPC__deref_out_opt ISClusNode **ppOwnerNode) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Cluster( 
            /* [retval][out] */ __RPC__deref_out_opt ISCluster **ppCluster) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_ClassInfo( 
            /* [retval][out] */ __RPC__out CLUSTER_RESOURCE_CLASS *prcClassInfo) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Disk( 
            /* [retval][out] */ __RPC__deref_out_opt ISClusDisk **ppDisk) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_RegistryKeys( 
            /* [retval][out] */ __RPC__deref_out_opt ISClusRegistryKeys **ppRegistryKeys) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_CryptoKeys( 
            /* [retval][out] */ __RPC__deref_out_opt ISClusCryptoKeys **ppCryptoKeys) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_TypeName( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrTypeName) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Type( 
            /* [retval][out] */ __RPC__deref_out_opt ISClusResType **ppResourceType) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_MaintenanceMode( 
            /* [retval][out] */ __RPC__out BOOL *pbMaintenanceMode) = 0;
        
        virtual /* [helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_MaintenanceMode( 
            /* [in] */ BOOL bMaintenanceMode) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ISClusResourceVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in ISClusResource * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in ISClusResource * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in ISClusResource * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in ISClusResource * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in ISClusResource * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in ISClusResource * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            ISClusResource * This,
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
        
        DECLSPEC_XFGVIRT(ISClusResource, get_CommonProperties)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_CommonProperties )( 
            __RPC__in ISClusResource * This,
            /* [retval][out] */ __RPC__deref_out_opt ISClusProperties **ppProperties);
        
        DECLSPEC_XFGVIRT(ISClusResource, get_PrivateProperties)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_PrivateProperties )( 
            __RPC__in ISClusResource * This,
            /* [retval][out] */ __RPC__deref_out_opt ISClusProperties **ppProperties);
        
        DECLSPEC_XFGVIRT(ISClusResource, get_CommonROProperties)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_CommonROProperties )( 
            __RPC__in ISClusResource * This,
            /* [retval][out] */ __RPC__deref_out_opt ISClusProperties **ppProperties);
        
        DECLSPEC_XFGVIRT(ISClusResource, get_PrivateROProperties)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_PrivateROProperties )( 
            __RPC__in ISClusResource * This,
            /* [retval][out] */ __RPC__deref_out_opt ISClusProperties **ppProperties);
        
        DECLSPEC_XFGVIRT(ISClusResource, get_Handle)
        /* [helpstring][id][hidden][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Handle )( 
            __RPC__in ISClusResource * This,
            /* [retval][out] */ __RPC__out ULONG_PTR *phandle);
        
        DECLSPEC_XFGVIRT(ISClusResource, get_Name)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Name )( 
            __RPC__in ISClusResource * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrName);
        
        DECLSPEC_XFGVIRT(ISClusResource, put_Name)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_Name )( 
            __RPC__in ISClusResource * This,
            /* [in] */ __RPC__in BSTR bstrResourceName);
        
        DECLSPEC_XFGVIRT(ISClusResource, get_State)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_State )( 
            __RPC__in ISClusResource * This,
            /* [retval][out] */ __RPC__out CLUSTER_RESOURCE_STATE *dwState);
        
        DECLSPEC_XFGVIRT(ISClusResource, get_CoreFlag)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_CoreFlag )( 
            __RPC__in ISClusResource * This,
            /* [retval][out] */ __RPC__out CLUS_FLAGS *dwCoreFlag);
        
        DECLSPEC_XFGVIRT(ISClusResource, BecomeQuorumResource)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *BecomeQuorumResource )( 
            __RPC__in ISClusResource * This,
            /* [in] */ __RPC__in BSTR bstrDevicePath,
            /* [in] */ long lMaxLogSize);
        
        DECLSPEC_XFGVIRT(ISClusResource, Delete)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *Delete )( 
            __RPC__in ISClusResource * This);
        
        DECLSPEC_XFGVIRT(ISClusResource, Fail)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *Fail )( 
            __RPC__in ISClusResource * This);
        
        DECLSPEC_XFGVIRT(ISClusResource, Online)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *Online )( 
            __RPC__in ISClusResource * This,
            /* [in] */ long nTimeout,
            /* [retval][out] */ __RPC__out VARIANT *pvarPending);
        
        DECLSPEC_XFGVIRT(ISClusResource, Offline)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *Offline )( 
            __RPC__in ISClusResource * This,
            /* [in] */ long nTimeout,
            /* [retval][out] */ __RPC__out VARIANT *pvarPending);
        
        DECLSPEC_XFGVIRT(ISClusResource, ChangeResourceGroup)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *ChangeResourceGroup )( 
            __RPC__in ISClusResource * This,
            /* [in] */ __RPC__in_opt ISClusResGroup *pResourceGroup);
        
        DECLSPEC_XFGVIRT(ISClusResource, AddResourceNode)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *AddResourceNode )( 
            __RPC__in ISClusResource * This,
            /* [in] */ __RPC__in_opt ISClusNode *pNode);
        
        DECLSPEC_XFGVIRT(ISClusResource, RemoveResourceNode)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *RemoveResourceNode )( 
            __RPC__in ISClusResource * This,
            /* [in] */ __RPC__in_opt ISClusNode *pNode);
        
        DECLSPEC_XFGVIRT(ISClusResource, CanResourceBeDependent)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *CanResourceBeDependent )( 
            __RPC__in ISClusResource * This,
            /* [in] */ __RPC__in_opt ISClusResource *pResource,
            /* [retval][out] */ __RPC__out VARIANT *pvarDependent);
        
        DECLSPEC_XFGVIRT(ISClusResource, get_PossibleOwnerNodes)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_PossibleOwnerNodes )( 
            __RPC__in ISClusResource * This,
            /* [retval][out] */ __RPC__deref_out_opt ISClusResPossibleOwnerNodes **ppOwnerNodes);
        
        DECLSPEC_XFGVIRT(ISClusResource, get_Dependencies)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Dependencies )( 
            __RPC__in ISClusResource * This,
            /* [retval][out] */ __RPC__deref_out_opt ISClusResDependencies **ppResDependencies);
        
        DECLSPEC_XFGVIRT(ISClusResource, get_Dependents)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Dependents )( 
            __RPC__in ISClusResource * This,
            /* [retval][out] */ __RPC__deref_out_opt ISClusResDependents **ppResDependents);
        
        DECLSPEC_XFGVIRT(ISClusResource, get_Group)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Group )( 
            __RPC__in ISClusResource * This,
            /* [retval][out] */ __RPC__deref_out_opt ISClusResGroup **ppResGroup);
        
        DECLSPEC_XFGVIRT(ISClusResource, get_OwnerNode)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_OwnerNode )( 
            __RPC__in ISClusResource * This,
            /* [retval][out] */ __RPC__deref_out_opt ISClusNode **ppOwnerNode);
        
        DECLSPEC_XFGVIRT(ISClusResource, get_Cluster)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Cluster )( 
            __RPC__in ISClusResource * This,
            /* [retval][out] */ __RPC__deref_out_opt ISCluster **ppCluster);
        
        DECLSPEC_XFGVIRT(ISClusResource, get_ClassInfo)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_ClassInfo )( 
            __RPC__in ISClusResource * This,
            /* [retval][out] */ __RPC__out CLUSTER_RESOURCE_CLASS *prcClassInfo);
        
        DECLSPEC_XFGVIRT(ISClusResource, get_Disk)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Disk )( 
            __RPC__in ISClusResource * This,
            /* [retval][out] */ __RPC__deref_out_opt ISClusDisk **ppDisk);
        
        DECLSPEC_XFGVIRT(ISClusResource, get_RegistryKeys)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_RegistryKeys )( 
            __RPC__in ISClusResource * This,
            /* [retval][out] */ __RPC__deref_out_opt ISClusRegistryKeys **ppRegistryKeys);
        
        DECLSPEC_XFGVIRT(ISClusResource, get_CryptoKeys)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_CryptoKeys )( 
            __RPC__in ISClusResource * This,
            /* [retval][out] */ __RPC__deref_out_opt ISClusCryptoKeys **ppCryptoKeys);
        
        DECLSPEC_XFGVIRT(ISClusResource, get_TypeName)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_TypeName )( 
            __RPC__in ISClusResource * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrTypeName);
        
        DECLSPEC_XFGVIRT(ISClusResource, get_Type)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Type )( 
            __RPC__in ISClusResource * This,
            /* [retval][out] */ __RPC__deref_out_opt ISClusResType **ppResourceType);
        
        DECLSPEC_XFGVIRT(ISClusResource, get_MaintenanceMode)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_MaintenanceMode )( 
            __RPC__in ISClusResource * This,
            /* [retval][out] */ __RPC__out BOOL *pbMaintenanceMode);
        
        DECLSPEC_XFGVIRT(ISClusResource, put_MaintenanceMode)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_MaintenanceMode )( 
            __RPC__in ISClusResource * This,
            /* [in] */ BOOL bMaintenanceMode);
        
        END_INTERFACE
    } ISClusResourceVtbl;

    interface ISClusResource
    {
        CONST_VTBL struct ISClusResourceVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ISClusResource_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ISClusResource_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ISClusResource_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ISClusResource_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define ISClusResource_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define ISClusResource_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define ISClusResource_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define ISClusResource_get_CommonProperties(This,ppProperties)	\
    ( (This)->lpVtbl -> get_CommonProperties(This,ppProperties) ) 

#define ISClusResource_get_PrivateProperties(This,ppProperties)	\
    ( (This)->lpVtbl -> get_PrivateProperties(This,ppProperties) ) 

#define ISClusResource_get_CommonROProperties(This,ppProperties)	\
    ( (This)->lpVtbl -> get_CommonROProperties(This,ppProperties) ) 

#define ISClusResource_get_PrivateROProperties(This,ppProperties)	\
    ( (This)->lpVtbl -> get_PrivateROProperties(This,ppProperties) ) 

#define ISClusResource_get_Handle(This,phandle)	\
    ( (This)->lpVtbl -> get_Handle(This,phandle) ) 

#define ISClusResource_get_Name(This,pbstrName)	\
    ( (This)->lpVtbl -> get_Name(This,pbstrName) ) 

#define ISClusResource_put_Name(This,bstrResourceName)	\
    ( (This)->lpVtbl -> put_Name(This,bstrResourceName) ) 

#define ISClusResource_get_State(This,dwState)	\
    ( (This)->lpVtbl -> get_State(This,dwState) ) 

#define ISClusResource_get_CoreFlag(This,dwCoreFlag)	\
    ( (This)->lpVtbl -> get_CoreFlag(This,dwCoreFlag) ) 

#define ISClusResource_BecomeQuorumResource(This,bstrDevicePath,lMaxLogSize)	\
    ( (This)->lpVtbl -> BecomeQuorumResource(This,bstrDevicePath,lMaxLogSize) ) 

#define ISClusResource_Delete(This)	\
    ( (This)->lpVtbl -> Delete(This) ) 

#define ISClusResource_Fail(This)	\
    ( (This)->lpVtbl -> Fail(This) ) 

#define ISClusResource_Online(This,nTimeout,pvarPending)	\
    ( (This)->lpVtbl -> Online(This,nTimeout,pvarPending) ) 

#define ISClusResource_Offline(This,nTimeout,pvarPending)	\
    ( (This)->lpVtbl -> Offline(This,nTimeout,pvarPending) ) 

#define ISClusResource_ChangeResourceGroup(This,pResourceGroup)	\
    ( (This)->lpVtbl -> ChangeResourceGroup(This,pResourceGroup) ) 

#define ISClusResource_AddResourceNode(This,pNode)	\
    ( (This)->lpVtbl -> AddResourceNode(This,pNode) ) 

#define ISClusResource_RemoveResourceNode(This,pNode)	\
    ( (This)->lpVtbl -> RemoveResourceNode(This,pNode) ) 

#define ISClusResource_CanResourceBeDependent(This,pResource,pvarDependent)	\
    ( (This)->lpVtbl -> CanResourceBeDependent(This,pResource,pvarDependent) ) 

#define ISClusResource_get_PossibleOwnerNodes(This,ppOwnerNodes)	\
    ( (This)->lpVtbl -> get_PossibleOwnerNodes(This,ppOwnerNodes) ) 

#define ISClusResource_get_Dependencies(This,ppResDependencies)	\
    ( (This)->lpVtbl -> get_Dependencies(This,ppResDependencies) ) 

#define ISClusResource_get_Dependents(This,ppResDependents)	\
    ( (This)->lpVtbl -> get_Dependents(This,ppResDependents) ) 

#define ISClusResource_get_Group(This,ppResGroup)	\
    ( (This)->lpVtbl -> get_Group(This,ppResGroup) ) 

#define ISClusResource_get_OwnerNode(This,ppOwnerNode)	\
    ( (This)->lpVtbl -> get_OwnerNode(This,ppOwnerNode) ) 

#define ISClusResource_get_Cluster(This,ppCluster)	\
    ( (This)->lpVtbl -> get_Cluster(This,ppCluster) ) 

#define ISClusResource_get_ClassInfo(This,prcClassInfo)	\
    ( (This)->lpVtbl -> get_ClassInfo(This,prcClassInfo) ) 

#define ISClusResource_get_Disk(This,ppDisk)	\
    ( (This)->lpVtbl -> get_Disk(This,ppDisk) ) 

#define ISClusResource_get_RegistryKeys(This,ppRegistryKeys)	\
    ( (This)->lpVtbl -> get_RegistryKeys(This,ppRegistryKeys) ) 

#define ISClusResource_get_CryptoKeys(This,ppCryptoKeys)	\
    ( (This)->lpVtbl -> get_CryptoKeys(This,ppCryptoKeys) ) 

#define ISClusResource_get_TypeName(This,pbstrTypeName)	\
    ( (This)->lpVtbl -> get_TypeName(This,pbstrTypeName) ) 

#define ISClusResource_get_Type(This,ppResourceType)	\
    ( (This)->lpVtbl -> get_Type(This,ppResourceType) ) 

#define ISClusResource_get_MaintenanceMode(This,pbMaintenanceMode)	\
    ( (This)->lpVtbl -> get_MaintenanceMode(This,pbMaintenanceMode) ) 

#define ISClusResource_put_MaintenanceMode(This,bMaintenanceMode)	\
    ( (This)->lpVtbl -> put_MaintenanceMode(This,bMaintenanceMode) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ISClusResource_INTERFACE_DEFINED__ */


#ifndef __ISClusResDependencies_INTERFACE_DEFINED__
#define __ISClusResDependencies_INTERFACE_DEFINED__

/* interface ISClusResDependencies */
/* [unique][helpstring][dual][uuid][object][nonextensible][hidden][oleautomation] */ 


EXTERN_C const IID IID_ISClusResDependencies;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("f2e60704-2631-11d1-89f1-00a0c90d061e")
    ISClusResDependencies : public IDispatch
    {
    public:
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Count( 
            /* [retval][out] */ __RPC__out long *plCount) = 0;
        
        virtual /* [helpstring][id][restricted][propget] */ HRESULT STDMETHODCALLTYPE get__NewEnum( 
            /* [retval][out] */ __RPC__deref_out_opt IUnknown **retval) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE Refresh( void) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Item( 
            /* [in] */ VARIANT varIndex,
            /* [retval][out] */ __RPC__deref_out_opt ISClusResource **ppClusResource) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE CreateItem( 
            /* [in] */ __RPC__in BSTR bstrResourceName,
            /* [in] */ __RPC__in BSTR bstrResourceType,
            /* [in] */ CLUSTER_RESOURCE_CREATE_FLAGS dwFlags,
            /* [retval][out] */ __RPC__deref_out_opt ISClusResource **ppClusterResource) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE DeleteItem( 
            /* [in] */ VARIANT varIndex) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE AddItem( 
            /* [in] */ __RPC__in_opt ISClusResource *pResource) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE RemoveItem( 
            /* [in] */ VARIANT varIndex) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ISClusResDependenciesVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in ISClusResDependencies * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in ISClusResDependencies * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in ISClusResDependencies * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in ISClusResDependencies * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in ISClusResDependencies * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in ISClusResDependencies * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            ISClusResDependencies * This,
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
        
        DECLSPEC_XFGVIRT(ISClusResDependencies, get_Count)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Count )( 
            __RPC__in ISClusResDependencies * This,
            /* [retval][out] */ __RPC__out long *plCount);
        
        DECLSPEC_XFGVIRT(ISClusResDependencies, get__NewEnum)
        /* [helpstring][id][restricted][propget] */ HRESULT ( STDMETHODCALLTYPE *get__NewEnum )( 
            __RPC__in ISClusResDependencies * This,
            /* [retval][out] */ __RPC__deref_out_opt IUnknown **retval);
        
        DECLSPEC_XFGVIRT(ISClusResDependencies, Refresh)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *Refresh )( 
            __RPC__in ISClusResDependencies * This);
        
        DECLSPEC_XFGVIRT(ISClusResDependencies, get_Item)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Item )( 
            __RPC__in ISClusResDependencies * This,
            /* [in] */ VARIANT varIndex,
            /* [retval][out] */ __RPC__deref_out_opt ISClusResource **ppClusResource);
        
        DECLSPEC_XFGVIRT(ISClusResDependencies, CreateItem)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *CreateItem )( 
            __RPC__in ISClusResDependencies * This,
            /* [in] */ __RPC__in BSTR bstrResourceName,
            /* [in] */ __RPC__in BSTR bstrResourceType,
            /* [in] */ CLUSTER_RESOURCE_CREATE_FLAGS dwFlags,
            /* [retval][out] */ __RPC__deref_out_opt ISClusResource **ppClusterResource);
        
        DECLSPEC_XFGVIRT(ISClusResDependencies, DeleteItem)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *DeleteItem )( 
            __RPC__in ISClusResDependencies * This,
            /* [in] */ VARIANT varIndex);
        
        DECLSPEC_XFGVIRT(ISClusResDependencies, AddItem)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *AddItem )( 
            __RPC__in ISClusResDependencies * This,
            /* [in] */ __RPC__in_opt ISClusResource *pResource);
        
        DECLSPEC_XFGVIRT(ISClusResDependencies, RemoveItem)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *RemoveItem )( 
            __RPC__in ISClusResDependencies * This,
            /* [in] */ VARIANT varIndex);
        
        END_INTERFACE
    } ISClusResDependenciesVtbl;

    interface ISClusResDependencies
    {
        CONST_VTBL struct ISClusResDependenciesVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ISClusResDependencies_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ISClusResDependencies_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ISClusResDependencies_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ISClusResDependencies_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define ISClusResDependencies_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define ISClusResDependencies_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define ISClusResDependencies_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define ISClusResDependencies_get_Count(This,plCount)	\
    ( (This)->lpVtbl -> get_Count(This,plCount) ) 

#define ISClusResDependencies_get__NewEnum(This,retval)	\
    ( (This)->lpVtbl -> get__NewEnum(This,retval) ) 

#define ISClusResDependencies_Refresh(This)	\
    ( (This)->lpVtbl -> Refresh(This) ) 

#define ISClusResDependencies_get_Item(This,varIndex,ppClusResource)	\
    ( (This)->lpVtbl -> get_Item(This,varIndex,ppClusResource) ) 

#define ISClusResDependencies_CreateItem(This,bstrResourceName,bstrResourceType,dwFlags,ppClusterResource)	\
    ( (This)->lpVtbl -> CreateItem(This,bstrResourceName,bstrResourceType,dwFlags,ppClusterResource) ) 

#define ISClusResDependencies_DeleteItem(This,varIndex)	\
    ( (This)->lpVtbl -> DeleteItem(This,varIndex) ) 

#define ISClusResDependencies_AddItem(This,pResource)	\
    ( (This)->lpVtbl -> AddItem(This,pResource) ) 

#define ISClusResDependencies_RemoveItem(This,varIndex)	\
    ( (This)->lpVtbl -> RemoveItem(This,varIndex) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ISClusResDependencies_INTERFACE_DEFINED__ */


#ifndef __ISClusResGroupResources_INTERFACE_DEFINED__
#define __ISClusResGroupResources_INTERFACE_DEFINED__

/* interface ISClusResGroupResources */
/* [unique][helpstring][dual][uuid][object][nonextensible][hidden][oleautomation] */ 


EXTERN_C const IID IID_ISClusResGroupResources;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("f2e606ea-2631-11d1-89f1-00a0c90d061e")
    ISClusResGroupResources : public IDispatch
    {
    public:
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Count( 
            /* [retval][out] */ __RPC__out long *plCount) = 0;
        
        virtual /* [helpstring][id][restricted][propget] */ HRESULT STDMETHODCALLTYPE get__NewEnum( 
            /* [retval][out] */ __RPC__deref_out_opt IUnknown **retval) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE Refresh( void) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Item( 
            /* [in] */ VARIANT varIndex,
            /* [retval][out] */ __RPC__deref_out_opt ISClusResource **ppClusResource) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE CreateItem( 
            /* [in] */ __RPC__in BSTR bstrResourceName,
            /* [in] */ __RPC__in BSTR bstrResourceType,
            /* [in] */ CLUSTER_RESOURCE_CREATE_FLAGS dwFlags,
            /* [retval][out] */ __RPC__deref_out_opt ISClusResource **ppClusterResource) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE DeleteItem( 
            /* [in] */ VARIANT varIndex) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ISClusResGroupResourcesVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in ISClusResGroupResources * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in ISClusResGroupResources * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in ISClusResGroupResources * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in ISClusResGroupResources * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in ISClusResGroupResources * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in ISClusResGroupResources * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            ISClusResGroupResources * This,
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
        
        DECLSPEC_XFGVIRT(ISClusResGroupResources, get_Count)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Count )( 
            __RPC__in ISClusResGroupResources * This,
            /* [retval][out] */ __RPC__out long *plCount);
        
        DECLSPEC_XFGVIRT(ISClusResGroupResources, get__NewEnum)
        /* [helpstring][id][restricted][propget] */ HRESULT ( STDMETHODCALLTYPE *get__NewEnum )( 
            __RPC__in ISClusResGroupResources * This,
            /* [retval][out] */ __RPC__deref_out_opt IUnknown **retval);
        
        DECLSPEC_XFGVIRT(ISClusResGroupResources, Refresh)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *Refresh )( 
            __RPC__in ISClusResGroupResources * This);
        
        DECLSPEC_XFGVIRT(ISClusResGroupResources, get_Item)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Item )( 
            __RPC__in ISClusResGroupResources * This,
            /* [in] */ VARIANT varIndex,
            /* [retval][out] */ __RPC__deref_out_opt ISClusResource **ppClusResource);
        
        DECLSPEC_XFGVIRT(ISClusResGroupResources, CreateItem)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *CreateItem )( 
            __RPC__in ISClusResGroupResources * This,
            /* [in] */ __RPC__in BSTR bstrResourceName,
            /* [in] */ __RPC__in BSTR bstrResourceType,
            /* [in] */ CLUSTER_RESOURCE_CREATE_FLAGS dwFlags,
            /* [retval][out] */ __RPC__deref_out_opt ISClusResource **ppClusterResource);
        
        DECLSPEC_XFGVIRT(ISClusResGroupResources, DeleteItem)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *DeleteItem )( 
            __RPC__in ISClusResGroupResources * This,
            /* [in] */ VARIANT varIndex);
        
        END_INTERFACE
    } ISClusResGroupResourcesVtbl;

    interface ISClusResGroupResources
    {
        CONST_VTBL struct ISClusResGroupResourcesVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ISClusResGroupResources_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ISClusResGroupResources_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ISClusResGroupResources_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ISClusResGroupResources_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define ISClusResGroupResources_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define ISClusResGroupResources_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define ISClusResGroupResources_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define ISClusResGroupResources_get_Count(This,plCount)	\
    ( (This)->lpVtbl -> get_Count(This,plCount) ) 

#define ISClusResGroupResources_get__NewEnum(This,retval)	\
    ( (This)->lpVtbl -> get__NewEnum(This,retval) ) 

#define ISClusResGroupResources_Refresh(This)	\
    ( (This)->lpVtbl -> Refresh(This) ) 

#define ISClusResGroupResources_get_Item(This,varIndex,ppClusResource)	\
    ( (This)->lpVtbl -> get_Item(This,varIndex,ppClusResource) ) 

#define ISClusResGroupResources_CreateItem(This,bstrResourceName,bstrResourceType,dwFlags,ppClusterResource)	\
    ( (This)->lpVtbl -> CreateItem(This,bstrResourceName,bstrResourceType,dwFlags,ppClusterResource) ) 

#define ISClusResGroupResources_DeleteItem(This,varIndex)	\
    ( (This)->lpVtbl -> DeleteItem(This,varIndex) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ISClusResGroupResources_INTERFACE_DEFINED__ */


#ifndef __ISClusResTypeResources_INTERFACE_DEFINED__
#define __ISClusResTypeResources_INTERFACE_DEFINED__

/* interface ISClusResTypeResources */
/* [unique][helpstring][dual][uuid][object][nonextensible][hidden][oleautomation] */ 


EXTERN_C const IID IID_ISClusResTypeResources;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("f2e60714-2631-11d1-89f1-00a0c90d061e")
    ISClusResTypeResources : public IDispatch
    {
    public:
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Count( 
            /* [retval][out] */ __RPC__out long *plCount) = 0;
        
        virtual /* [helpstring][id][restricted][propget] */ HRESULT STDMETHODCALLTYPE get__NewEnum( 
            /* [retval][out] */ __RPC__deref_out_opt IUnknown **retval) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE Refresh( void) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Item( 
            /* [in] */ VARIANT varIndex,
            /* [retval][out] */ __RPC__deref_out_opt ISClusResource **ppClusResource) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE CreateItem( 
            /* [in] */ __RPC__in BSTR bstrResourceName,
            /* [in] */ __RPC__in BSTR bstrGroupName,
            /* [in] */ CLUSTER_RESOURCE_CREATE_FLAGS dwFlags,
            /* [retval][out] */ __RPC__deref_out_opt ISClusResource **ppClusterResource) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE DeleteItem( 
            /* [in] */ VARIANT varIndex) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ISClusResTypeResourcesVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in ISClusResTypeResources * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in ISClusResTypeResources * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in ISClusResTypeResources * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in ISClusResTypeResources * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in ISClusResTypeResources * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in ISClusResTypeResources * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            ISClusResTypeResources * This,
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
        
        DECLSPEC_XFGVIRT(ISClusResTypeResources, get_Count)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Count )( 
            __RPC__in ISClusResTypeResources * This,
            /* [retval][out] */ __RPC__out long *plCount);
        
        DECLSPEC_XFGVIRT(ISClusResTypeResources, get__NewEnum)
        /* [helpstring][id][restricted][propget] */ HRESULT ( STDMETHODCALLTYPE *get__NewEnum )( 
            __RPC__in ISClusResTypeResources * This,
            /* [retval][out] */ __RPC__deref_out_opt IUnknown **retval);
        
        DECLSPEC_XFGVIRT(ISClusResTypeResources, Refresh)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *Refresh )( 
            __RPC__in ISClusResTypeResources * This);
        
        DECLSPEC_XFGVIRT(ISClusResTypeResources, get_Item)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Item )( 
            __RPC__in ISClusResTypeResources * This,
            /* [in] */ VARIANT varIndex,
            /* [retval][out] */ __RPC__deref_out_opt ISClusResource **ppClusResource);
        
        DECLSPEC_XFGVIRT(ISClusResTypeResources, CreateItem)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *CreateItem )( 
            __RPC__in ISClusResTypeResources * This,
            /* [in] */ __RPC__in BSTR bstrResourceName,
            /* [in] */ __RPC__in BSTR bstrGroupName,
            /* [in] */ CLUSTER_RESOURCE_CREATE_FLAGS dwFlags,
            /* [retval][out] */ __RPC__deref_out_opt ISClusResource **ppClusterResource);
        
        DECLSPEC_XFGVIRT(ISClusResTypeResources, DeleteItem)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *DeleteItem )( 
            __RPC__in ISClusResTypeResources * This,
            /* [in] */ VARIANT varIndex);
        
        END_INTERFACE
    } ISClusResTypeResourcesVtbl;

    interface ISClusResTypeResources
    {
        CONST_VTBL struct ISClusResTypeResourcesVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ISClusResTypeResources_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ISClusResTypeResources_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ISClusResTypeResources_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ISClusResTypeResources_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define ISClusResTypeResources_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define ISClusResTypeResources_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define ISClusResTypeResources_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define ISClusResTypeResources_get_Count(This,plCount)	\
    ( (This)->lpVtbl -> get_Count(This,plCount) ) 

#define ISClusResTypeResources_get__NewEnum(This,retval)	\
    ( (This)->lpVtbl -> get__NewEnum(This,retval) ) 

#define ISClusResTypeResources_Refresh(This)	\
    ( (This)->lpVtbl -> Refresh(This) ) 

#define ISClusResTypeResources_get_Item(This,varIndex,ppClusResource)	\
    ( (This)->lpVtbl -> get_Item(This,varIndex,ppClusResource) ) 

#define ISClusResTypeResources_CreateItem(This,bstrResourceName,bstrGroupName,dwFlags,ppClusterResource)	\
    ( (This)->lpVtbl -> CreateItem(This,bstrResourceName,bstrGroupName,dwFlags,ppClusterResource) ) 

#define ISClusResTypeResources_DeleteItem(This,varIndex)	\
    ( (This)->lpVtbl -> DeleteItem(This,varIndex) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ISClusResTypeResources_INTERFACE_DEFINED__ */


#ifndef __ISClusResources_INTERFACE_DEFINED__
#define __ISClusResources_INTERFACE_DEFINED__

/* interface ISClusResources */
/* [unique][helpstring][dual][uuid][object][nonextensible][hidden][oleautomation] */ 


EXTERN_C const IID IID_ISClusResources;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("f2e6070c-2631-11d1-89f1-00a0c90d061e")
    ISClusResources : public IDispatch
    {
    public:
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Count( 
            /* [retval][out] */ __RPC__out long *plCount) = 0;
        
        virtual /* [helpstring][id][restricted][propget] */ HRESULT STDMETHODCALLTYPE get__NewEnum( 
            /* [retval][out] */ __RPC__deref_out_opt IUnknown **retval) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE Refresh( void) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Item( 
            /* [in] */ VARIANT varIndex,
            /* [retval][out] */ __RPC__deref_out_opt ISClusResource **ppClusResource) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE CreateItem( 
            /* [in] */ __RPC__in BSTR bstrResourceName,
            /* [in] */ __RPC__in BSTR bstrResourceType,
            /* [in] */ __RPC__in BSTR bstrGroupName,
            /* [in] */ CLUSTER_RESOURCE_CREATE_FLAGS dwFlags,
            /* [retval][out] */ __RPC__deref_out_opt ISClusResource **ppClusterResource) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE DeleteItem( 
            /* [in] */ VARIANT varIndex) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ISClusResourcesVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in ISClusResources * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in ISClusResources * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in ISClusResources * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in ISClusResources * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in ISClusResources * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in ISClusResources * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            ISClusResources * This,
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
        
        DECLSPEC_XFGVIRT(ISClusResources, get_Count)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Count )( 
            __RPC__in ISClusResources * This,
            /* [retval][out] */ __RPC__out long *plCount);
        
        DECLSPEC_XFGVIRT(ISClusResources, get__NewEnum)
        /* [helpstring][id][restricted][propget] */ HRESULT ( STDMETHODCALLTYPE *get__NewEnum )( 
            __RPC__in ISClusResources * This,
            /* [retval][out] */ __RPC__deref_out_opt IUnknown **retval);
        
        DECLSPEC_XFGVIRT(ISClusResources, Refresh)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *Refresh )( 
            __RPC__in ISClusResources * This);
        
        DECLSPEC_XFGVIRT(ISClusResources, get_Item)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Item )( 
            __RPC__in ISClusResources * This,
            /* [in] */ VARIANT varIndex,
            /* [retval][out] */ __RPC__deref_out_opt ISClusResource **ppClusResource);
        
        DECLSPEC_XFGVIRT(ISClusResources, CreateItem)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *CreateItem )( 
            __RPC__in ISClusResources * This,
            /* [in] */ __RPC__in BSTR bstrResourceName,
            /* [in] */ __RPC__in BSTR bstrResourceType,
            /* [in] */ __RPC__in BSTR bstrGroupName,
            /* [in] */ CLUSTER_RESOURCE_CREATE_FLAGS dwFlags,
            /* [retval][out] */ __RPC__deref_out_opt ISClusResource **ppClusterResource);
        
        DECLSPEC_XFGVIRT(ISClusResources, DeleteItem)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *DeleteItem )( 
            __RPC__in ISClusResources * This,
            /* [in] */ VARIANT varIndex);
        
        END_INTERFACE
    } ISClusResourcesVtbl;

    interface ISClusResources
    {
        CONST_VTBL struct ISClusResourcesVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ISClusResources_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ISClusResources_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ISClusResources_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ISClusResources_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define ISClusResources_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define ISClusResources_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define ISClusResources_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define ISClusResources_get_Count(This,plCount)	\
    ( (This)->lpVtbl -> get_Count(This,plCount) ) 

#define ISClusResources_get__NewEnum(This,retval)	\
    ( (This)->lpVtbl -> get__NewEnum(This,retval) ) 

#define ISClusResources_Refresh(This)	\
    ( (This)->lpVtbl -> Refresh(This) ) 

#define ISClusResources_get_Item(This,varIndex,ppClusResource)	\
    ( (This)->lpVtbl -> get_Item(This,varIndex,ppClusResource) ) 

#define ISClusResources_CreateItem(This,bstrResourceName,bstrResourceType,bstrGroupName,dwFlags,ppClusterResource)	\
    ( (This)->lpVtbl -> CreateItem(This,bstrResourceName,bstrResourceType,bstrGroupName,dwFlags,ppClusterResource) ) 

#define ISClusResources_DeleteItem(This,varIndex)	\
    ( (This)->lpVtbl -> DeleteItem(This,varIndex) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ISClusResources_INTERFACE_DEFINED__ */


#ifndef __ISClusResGroupPreferredOwnerNodes_INTERFACE_DEFINED__
#define __ISClusResGroupPreferredOwnerNodes_INTERFACE_DEFINED__

/* interface ISClusResGroupPreferredOwnerNodes */
/* [unique][helpstring][dual][uuid][object][nonextensible][hidden][oleautomation] */ 


EXTERN_C const IID IID_ISClusResGroupPreferredOwnerNodes;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("f2e606e8-2631-11d1-89f1-00a0c90d061e")
    ISClusResGroupPreferredOwnerNodes : public IDispatch
    {
    public:
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Count( 
            /* [retval][out] */ __RPC__out long *plCount) = 0;
        
        virtual /* [helpstring][id][restricted][propget] */ HRESULT STDMETHODCALLTYPE get__NewEnum( 
            /* [retval][out] */ __RPC__deref_out_opt IUnknown **retval) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE Refresh( void) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Item( 
            /* [in] */ VARIANT varIndex,
            /* [retval][out] */ __RPC__deref_out_opt ISClusNode **ppNode) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE InsertItem( 
            /* [in] */ __RPC__in_opt ISClusNode *pNode,
            /* [in] */ long nPosition) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE RemoveItem( 
            /* [in] */ VARIANT varIndex) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Modified( 
            /* [retval][out] */ __RPC__out VARIANT *pvarModified) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE SaveChanges( void) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE AddItem( 
            /* [in] */ __RPC__in_opt ISClusNode *pNode) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ISClusResGroupPreferredOwnerNodesVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in ISClusResGroupPreferredOwnerNodes * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in ISClusResGroupPreferredOwnerNodes * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in ISClusResGroupPreferredOwnerNodes * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in ISClusResGroupPreferredOwnerNodes * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in ISClusResGroupPreferredOwnerNodes * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in ISClusResGroupPreferredOwnerNodes * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            ISClusResGroupPreferredOwnerNodes * This,
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
        
        DECLSPEC_XFGVIRT(ISClusResGroupPreferredOwnerNodes, get_Count)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Count )( 
            __RPC__in ISClusResGroupPreferredOwnerNodes * This,
            /* [retval][out] */ __RPC__out long *plCount);
        
        DECLSPEC_XFGVIRT(ISClusResGroupPreferredOwnerNodes, get__NewEnum)
        /* [helpstring][id][restricted][propget] */ HRESULT ( STDMETHODCALLTYPE *get__NewEnum )( 
            __RPC__in ISClusResGroupPreferredOwnerNodes * This,
            /* [retval][out] */ __RPC__deref_out_opt IUnknown **retval);
        
        DECLSPEC_XFGVIRT(ISClusResGroupPreferredOwnerNodes, Refresh)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *Refresh )( 
            __RPC__in ISClusResGroupPreferredOwnerNodes * This);
        
        DECLSPEC_XFGVIRT(ISClusResGroupPreferredOwnerNodes, get_Item)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Item )( 
            __RPC__in ISClusResGroupPreferredOwnerNodes * This,
            /* [in] */ VARIANT varIndex,
            /* [retval][out] */ __RPC__deref_out_opt ISClusNode **ppNode);
        
        DECLSPEC_XFGVIRT(ISClusResGroupPreferredOwnerNodes, InsertItem)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *InsertItem )( 
            __RPC__in ISClusResGroupPreferredOwnerNodes * This,
            /* [in] */ __RPC__in_opt ISClusNode *pNode,
            /* [in] */ long nPosition);
        
        DECLSPEC_XFGVIRT(ISClusResGroupPreferredOwnerNodes, RemoveItem)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *RemoveItem )( 
            __RPC__in ISClusResGroupPreferredOwnerNodes * This,
            /* [in] */ VARIANT varIndex);
        
        DECLSPEC_XFGVIRT(ISClusResGroupPreferredOwnerNodes, get_Modified)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Modified )( 
            __RPC__in ISClusResGroupPreferredOwnerNodes * This,
            /* [retval][out] */ __RPC__out VARIANT *pvarModified);
        
        DECLSPEC_XFGVIRT(ISClusResGroupPreferredOwnerNodes, SaveChanges)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *SaveChanges )( 
            __RPC__in ISClusResGroupPreferredOwnerNodes * This);
        
        DECLSPEC_XFGVIRT(ISClusResGroupPreferredOwnerNodes, AddItem)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *AddItem )( 
            __RPC__in ISClusResGroupPreferredOwnerNodes * This,
            /* [in] */ __RPC__in_opt ISClusNode *pNode);
        
        END_INTERFACE
    } ISClusResGroupPreferredOwnerNodesVtbl;

    interface ISClusResGroupPreferredOwnerNodes
    {
        CONST_VTBL struct ISClusResGroupPreferredOwnerNodesVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ISClusResGroupPreferredOwnerNodes_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ISClusResGroupPreferredOwnerNodes_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ISClusResGroupPreferredOwnerNodes_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ISClusResGroupPreferredOwnerNodes_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define ISClusResGroupPreferredOwnerNodes_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define ISClusResGroupPreferredOwnerNodes_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define ISClusResGroupPreferredOwnerNodes_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define ISClusResGroupPreferredOwnerNodes_get_Count(This,plCount)	\
    ( (This)->lpVtbl -> get_Count(This,plCount) ) 

#define ISClusResGroupPreferredOwnerNodes_get__NewEnum(This,retval)	\
    ( (This)->lpVtbl -> get__NewEnum(This,retval) ) 

#define ISClusResGroupPreferredOwnerNodes_Refresh(This)	\
    ( (This)->lpVtbl -> Refresh(This) ) 

#define ISClusResGroupPreferredOwnerNodes_get_Item(This,varIndex,ppNode)	\
    ( (This)->lpVtbl -> get_Item(This,varIndex,ppNode) ) 

#define ISClusResGroupPreferredOwnerNodes_InsertItem(This,pNode,nPosition)	\
    ( (This)->lpVtbl -> InsertItem(This,pNode,nPosition) ) 

#define ISClusResGroupPreferredOwnerNodes_RemoveItem(This,varIndex)	\
    ( (This)->lpVtbl -> RemoveItem(This,varIndex) ) 

#define ISClusResGroupPreferredOwnerNodes_get_Modified(This,pvarModified)	\
    ( (This)->lpVtbl -> get_Modified(This,pvarModified) ) 

#define ISClusResGroupPreferredOwnerNodes_SaveChanges(This)	\
    ( (This)->lpVtbl -> SaveChanges(This) ) 

#define ISClusResGroupPreferredOwnerNodes_AddItem(This,pNode)	\
    ( (This)->lpVtbl -> AddItem(This,pNode) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ISClusResGroupPreferredOwnerNodes_INTERFACE_DEFINED__ */


#ifndef __ISClusResPossibleOwnerNodes_INTERFACE_DEFINED__
#define __ISClusResPossibleOwnerNodes_INTERFACE_DEFINED__

/* interface ISClusResPossibleOwnerNodes */
/* [unique][helpstring][dual][uuid][object][nonextensible][hidden][oleautomation] */ 


EXTERN_C const IID IID_ISClusResPossibleOwnerNodes;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("f2e6070e-2631-11d1-89f1-00a0c90d061e")
    ISClusResPossibleOwnerNodes : public IDispatch
    {
    public:
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Count( 
            /* [retval][out] */ __RPC__out long *plCount) = 0;
        
        virtual /* [helpstring][id][restricted][propget] */ HRESULT STDMETHODCALLTYPE get__NewEnum( 
            /* [retval][out] */ __RPC__deref_out_opt IUnknown **retval) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE Refresh( void) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Item( 
            /* [in] */ VARIANT varIndex,
            /* [retval][out] */ __RPC__deref_out_opt ISClusNode **ppNode) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE AddItem( 
            /* [in] */ __RPC__in_opt ISClusNode *pNode) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE RemoveItem( 
            /* [in] */ VARIANT varIndex) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Modified( 
            /* [retval][out] */ __RPC__out VARIANT *pvarModified) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ISClusResPossibleOwnerNodesVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in ISClusResPossibleOwnerNodes * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in ISClusResPossibleOwnerNodes * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in ISClusResPossibleOwnerNodes * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in ISClusResPossibleOwnerNodes * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in ISClusResPossibleOwnerNodes * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in ISClusResPossibleOwnerNodes * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            ISClusResPossibleOwnerNodes * This,
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
        
        DECLSPEC_XFGVIRT(ISClusResPossibleOwnerNodes, get_Count)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Count )( 
            __RPC__in ISClusResPossibleOwnerNodes * This,
            /* [retval][out] */ __RPC__out long *plCount);
        
        DECLSPEC_XFGVIRT(ISClusResPossibleOwnerNodes, get__NewEnum)
        /* [helpstring][id][restricted][propget] */ HRESULT ( STDMETHODCALLTYPE *get__NewEnum )( 
            __RPC__in ISClusResPossibleOwnerNodes * This,
            /* [retval][out] */ __RPC__deref_out_opt IUnknown **retval);
        
        DECLSPEC_XFGVIRT(ISClusResPossibleOwnerNodes, Refresh)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *Refresh )( 
            __RPC__in ISClusResPossibleOwnerNodes * This);
        
        DECLSPEC_XFGVIRT(ISClusResPossibleOwnerNodes, get_Item)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Item )( 
            __RPC__in ISClusResPossibleOwnerNodes * This,
            /* [in] */ VARIANT varIndex,
            /* [retval][out] */ __RPC__deref_out_opt ISClusNode **ppNode);
        
        DECLSPEC_XFGVIRT(ISClusResPossibleOwnerNodes, AddItem)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *AddItem )( 
            __RPC__in ISClusResPossibleOwnerNodes * This,
            /* [in] */ __RPC__in_opt ISClusNode *pNode);
        
        DECLSPEC_XFGVIRT(ISClusResPossibleOwnerNodes, RemoveItem)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *RemoveItem )( 
            __RPC__in ISClusResPossibleOwnerNodes * This,
            /* [in] */ VARIANT varIndex);
        
        DECLSPEC_XFGVIRT(ISClusResPossibleOwnerNodes, get_Modified)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Modified )( 
            __RPC__in ISClusResPossibleOwnerNodes * This,
            /* [retval][out] */ __RPC__out VARIANT *pvarModified);
        
        END_INTERFACE
    } ISClusResPossibleOwnerNodesVtbl;

    interface ISClusResPossibleOwnerNodes
    {
        CONST_VTBL struct ISClusResPossibleOwnerNodesVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ISClusResPossibleOwnerNodes_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ISClusResPossibleOwnerNodes_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ISClusResPossibleOwnerNodes_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ISClusResPossibleOwnerNodes_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define ISClusResPossibleOwnerNodes_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define ISClusResPossibleOwnerNodes_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define ISClusResPossibleOwnerNodes_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define ISClusResPossibleOwnerNodes_get_Count(This,plCount)	\
    ( (This)->lpVtbl -> get_Count(This,plCount) ) 

#define ISClusResPossibleOwnerNodes_get__NewEnum(This,retval)	\
    ( (This)->lpVtbl -> get__NewEnum(This,retval) ) 

#define ISClusResPossibleOwnerNodes_Refresh(This)	\
    ( (This)->lpVtbl -> Refresh(This) ) 

#define ISClusResPossibleOwnerNodes_get_Item(This,varIndex,ppNode)	\
    ( (This)->lpVtbl -> get_Item(This,varIndex,ppNode) ) 

#define ISClusResPossibleOwnerNodes_AddItem(This,pNode)	\
    ( (This)->lpVtbl -> AddItem(This,pNode) ) 

#define ISClusResPossibleOwnerNodes_RemoveItem(This,varIndex)	\
    ( (This)->lpVtbl -> RemoveItem(This,varIndex) ) 

#define ISClusResPossibleOwnerNodes_get_Modified(This,pvarModified)	\
    ( (This)->lpVtbl -> get_Modified(This,pvarModified) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ISClusResPossibleOwnerNodes_INTERFACE_DEFINED__ */


#ifndef __ISClusResTypePossibleOwnerNodes_INTERFACE_DEFINED__
#define __ISClusResTypePossibleOwnerNodes_INTERFACE_DEFINED__

/* interface ISClusResTypePossibleOwnerNodes */
/* [unique][helpstring][dual][uuid][object][nonextensible][hidden][oleautomation] */ 


EXTERN_C const IID IID_ISClusResTypePossibleOwnerNodes;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("f2e60718-2631-11d1-89f1-00a0c90d061e")
    ISClusResTypePossibleOwnerNodes : public IDispatch
    {
    public:
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Count( 
            /* [retval][out] */ __RPC__out long *plCount) = 0;
        
        virtual /* [helpstring][id][restricted][propget] */ HRESULT STDMETHODCALLTYPE get__NewEnum( 
            /* [retval][out] */ __RPC__deref_out_opt IUnknown **retval) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE Refresh( void) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Item( 
            /* [in] */ VARIANT varIndex,
            /* [retval][out] */ __RPC__deref_out_opt ISClusNode **ppNode) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ISClusResTypePossibleOwnerNodesVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in ISClusResTypePossibleOwnerNodes * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in ISClusResTypePossibleOwnerNodes * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in ISClusResTypePossibleOwnerNodes * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in ISClusResTypePossibleOwnerNodes * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in ISClusResTypePossibleOwnerNodes * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in ISClusResTypePossibleOwnerNodes * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            ISClusResTypePossibleOwnerNodes * This,
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
        
        DECLSPEC_XFGVIRT(ISClusResTypePossibleOwnerNodes, get_Count)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Count )( 
            __RPC__in ISClusResTypePossibleOwnerNodes * This,
            /* [retval][out] */ __RPC__out long *plCount);
        
        DECLSPEC_XFGVIRT(ISClusResTypePossibleOwnerNodes, get__NewEnum)
        /* [helpstring][id][restricted][propget] */ HRESULT ( STDMETHODCALLTYPE *get__NewEnum )( 
            __RPC__in ISClusResTypePossibleOwnerNodes * This,
            /* [retval][out] */ __RPC__deref_out_opt IUnknown **retval);
        
        DECLSPEC_XFGVIRT(ISClusResTypePossibleOwnerNodes, Refresh)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *Refresh )( 
            __RPC__in ISClusResTypePossibleOwnerNodes * This);
        
        DECLSPEC_XFGVIRT(ISClusResTypePossibleOwnerNodes, get_Item)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Item )( 
            __RPC__in ISClusResTypePossibleOwnerNodes * This,
            /* [in] */ VARIANT varIndex,
            /* [retval][out] */ __RPC__deref_out_opt ISClusNode **ppNode);
        
        END_INTERFACE
    } ISClusResTypePossibleOwnerNodesVtbl;

    interface ISClusResTypePossibleOwnerNodes
    {
        CONST_VTBL struct ISClusResTypePossibleOwnerNodesVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ISClusResTypePossibleOwnerNodes_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ISClusResTypePossibleOwnerNodes_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ISClusResTypePossibleOwnerNodes_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ISClusResTypePossibleOwnerNodes_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define ISClusResTypePossibleOwnerNodes_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define ISClusResTypePossibleOwnerNodes_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define ISClusResTypePossibleOwnerNodes_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define ISClusResTypePossibleOwnerNodes_get_Count(This,plCount)	\
    ( (This)->lpVtbl -> get_Count(This,plCount) ) 

#define ISClusResTypePossibleOwnerNodes_get__NewEnum(This,retval)	\
    ( (This)->lpVtbl -> get__NewEnum(This,retval) ) 

#define ISClusResTypePossibleOwnerNodes_Refresh(This)	\
    ( (This)->lpVtbl -> Refresh(This) ) 

#define ISClusResTypePossibleOwnerNodes_get_Item(This,varIndex,ppNode)	\
    ( (This)->lpVtbl -> get_Item(This,varIndex,ppNode) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ISClusResTypePossibleOwnerNodes_INTERFACE_DEFINED__ */


#ifndef __ISClusResType_INTERFACE_DEFINED__
#define __ISClusResType_INTERFACE_DEFINED__

/* interface ISClusResType */
/* [unique][helpstring][dual][uuid][object][nonextensible][hidden][oleautomation] */ 


EXTERN_C const IID IID_ISClusResType;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("f2e60710-2631-11d1-89f1-00a0c90d061e")
    ISClusResType : public IDispatch
    {
    public:
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_CommonProperties( 
            /* [retval][out] */ __RPC__deref_out_opt ISClusProperties **ppProperties) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_PrivateProperties( 
            /* [retval][out] */ __RPC__deref_out_opt ISClusProperties **ppProperties) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_CommonROProperties( 
            /* [retval][out] */ __RPC__deref_out_opt ISClusProperties **ppProperties) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_PrivateROProperties( 
            /* [retval][out] */ __RPC__deref_out_opt ISClusProperties **ppProperties) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Name( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrName) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE Delete( void) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Cluster( 
            /* [retval][out] */ __RPC__deref_out_opt ISCluster **ppCluster) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Resources( 
            /* [retval][out] */ __RPC__deref_out_opt ISClusResTypeResources **ppClusterResTypeResources) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_PossibleOwnerNodes( 
            /* [retval][out] */ __RPC__deref_out_opt ISClusResTypePossibleOwnerNodes **ppOwnerNodes) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_AvailableDisks( 
            /* [retval][out] */ __RPC__deref_out_opt ISClusDisks **ppAvailableDisks) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ISClusResTypeVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in ISClusResType * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in ISClusResType * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in ISClusResType * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in ISClusResType * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in ISClusResType * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in ISClusResType * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            ISClusResType * This,
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
        
        DECLSPEC_XFGVIRT(ISClusResType, get_CommonProperties)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_CommonProperties )( 
            __RPC__in ISClusResType * This,
            /* [retval][out] */ __RPC__deref_out_opt ISClusProperties **ppProperties);
        
        DECLSPEC_XFGVIRT(ISClusResType, get_PrivateProperties)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_PrivateProperties )( 
            __RPC__in ISClusResType * This,
            /* [retval][out] */ __RPC__deref_out_opt ISClusProperties **ppProperties);
        
        DECLSPEC_XFGVIRT(ISClusResType, get_CommonROProperties)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_CommonROProperties )( 
            __RPC__in ISClusResType * This,
            /* [retval][out] */ __RPC__deref_out_opt ISClusProperties **ppProperties);
        
        DECLSPEC_XFGVIRT(ISClusResType, get_PrivateROProperties)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_PrivateROProperties )( 
            __RPC__in ISClusResType * This,
            /* [retval][out] */ __RPC__deref_out_opt ISClusProperties **ppProperties);
        
        DECLSPEC_XFGVIRT(ISClusResType, get_Name)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Name )( 
            __RPC__in ISClusResType * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrName);
        
        DECLSPEC_XFGVIRT(ISClusResType, Delete)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *Delete )( 
            __RPC__in ISClusResType * This);
        
        DECLSPEC_XFGVIRT(ISClusResType, get_Cluster)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Cluster )( 
            __RPC__in ISClusResType * This,
            /* [retval][out] */ __RPC__deref_out_opt ISCluster **ppCluster);
        
        DECLSPEC_XFGVIRT(ISClusResType, get_Resources)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Resources )( 
            __RPC__in ISClusResType * This,
            /* [retval][out] */ __RPC__deref_out_opt ISClusResTypeResources **ppClusterResTypeResources);
        
        DECLSPEC_XFGVIRT(ISClusResType, get_PossibleOwnerNodes)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_PossibleOwnerNodes )( 
            __RPC__in ISClusResType * This,
            /* [retval][out] */ __RPC__deref_out_opt ISClusResTypePossibleOwnerNodes **ppOwnerNodes);
        
        DECLSPEC_XFGVIRT(ISClusResType, get_AvailableDisks)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_AvailableDisks )( 
            __RPC__in ISClusResType * This,
            /* [retval][out] */ __RPC__deref_out_opt ISClusDisks **ppAvailableDisks);
        
        END_INTERFACE
    } ISClusResTypeVtbl;

    interface ISClusResType
    {
        CONST_VTBL struct ISClusResTypeVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ISClusResType_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ISClusResType_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ISClusResType_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ISClusResType_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define ISClusResType_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define ISClusResType_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define ISClusResType_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define ISClusResType_get_CommonProperties(This,ppProperties)	\
    ( (This)->lpVtbl -> get_CommonProperties(This,ppProperties) ) 

#define ISClusResType_get_PrivateProperties(This,ppProperties)	\
    ( (This)->lpVtbl -> get_PrivateProperties(This,ppProperties) ) 

#define ISClusResType_get_CommonROProperties(This,ppProperties)	\
    ( (This)->lpVtbl -> get_CommonROProperties(This,ppProperties) ) 

#define ISClusResType_get_PrivateROProperties(This,ppProperties)	\
    ( (This)->lpVtbl -> get_PrivateROProperties(This,ppProperties) ) 

#define ISClusResType_get_Name(This,pbstrName)	\
    ( (This)->lpVtbl -> get_Name(This,pbstrName) ) 

#define ISClusResType_Delete(This)	\
    ( (This)->lpVtbl -> Delete(This) ) 

#define ISClusResType_get_Cluster(This,ppCluster)	\
    ( (This)->lpVtbl -> get_Cluster(This,ppCluster) ) 

#define ISClusResType_get_Resources(This,ppClusterResTypeResources)	\
    ( (This)->lpVtbl -> get_Resources(This,ppClusterResTypeResources) ) 

#define ISClusResType_get_PossibleOwnerNodes(This,ppOwnerNodes)	\
    ( (This)->lpVtbl -> get_PossibleOwnerNodes(This,ppOwnerNodes) ) 

#define ISClusResType_get_AvailableDisks(This,ppAvailableDisks)	\
    ( (This)->lpVtbl -> get_AvailableDisks(This,ppAvailableDisks) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ISClusResType_INTERFACE_DEFINED__ */


#ifndef __ISClusResTypes_INTERFACE_DEFINED__
#define __ISClusResTypes_INTERFACE_DEFINED__

/* interface ISClusResTypes */
/* [unique][helpstring][dual][uuid][object][nonextensible][hidden][oleautomation] */ 


EXTERN_C const IID IID_ISClusResTypes;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("f2e60712-2631-11d1-89f1-00a0c90d061e")
    ISClusResTypes : public IDispatch
    {
    public:
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Count( 
            /* [retval][out] */ __RPC__out long *plCount) = 0;
        
        virtual /* [helpstring][id][restricted][propget] */ HRESULT STDMETHODCALLTYPE get__NewEnum( 
            /* [retval][out] */ __RPC__deref_out_opt IUnknown **retval) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE Refresh( void) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Item( 
            /* [in] */ VARIANT varIndex,
            /* [retval][out] */ __RPC__deref_out_opt ISClusResType **ppClusResType) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE CreateItem( 
            /* [in] */ __RPC__in BSTR bstrResourceTypeName,
            /* [in] */ __RPC__in BSTR bstrDisplayName,
            /* [in] */ __RPC__in BSTR bstrResourceTypeDll,
            /* [in] */ long dwLooksAlivePollInterval,
            /* [in] */ long dwIsAlivePollInterval,
            /* [retval][out] */ __RPC__deref_out_opt ISClusResType **ppResourceType) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE DeleteItem( 
            /* [in] */ VARIANT varIndex) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ISClusResTypesVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in ISClusResTypes * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in ISClusResTypes * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in ISClusResTypes * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in ISClusResTypes * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in ISClusResTypes * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in ISClusResTypes * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            ISClusResTypes * This,
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
        
        DECLSPEC_XFGVIRT(ISClusResTypes, get_Count)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Count )( 
            __RPC__in ISClusResTypes * This,
            /* [retval][out] */ __RPC__out long *plCount);
        
        DECLSPEC_XFGVIRT(ISClusResTypes, get__NewEnum)
        /* [helpstring][id][restricted][propget] */ HRESULT ( STDMETHODCALLTYPE *get__NewEnum )( 
            __RPC__in ISClusResTypes * This,
            /* [retval][out] */ __RPC__deref_out_opt IUnknown **retval);
        
        DECLSPEC_XFGVIRT(ISClusResTypes, Refresh)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *Refresh )( 
            __RPC__in ISClusResTypes * This);
        
        DECLSPEC_XFGVIRT(ISClusResTypes, get_Item)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Item )( 
            __RPC__in ISClusResTypes * This,
            /* [in] */ VARIANT varIndex,
            /* [retval][out] */ __RPC__deref_out_opt ISClusResType **ppClusResType);
        
        DECLSPEC_XFGVIRT(ISClusResTypes, CreateItem)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *CreateItem )( 
            __RPC__in ISClusResTypes * This,
            /* [in] */ __RPC__in BSTR bstrResourceTypeName,
            /* [in] */ __RPC__in BSTR bstrDisplayName,
            /* [in] */ __RPC__in BSTR bstrResourceTypeDll,
            /* [in] */ long dwLooksAlivePollInterval,
            /* [in] */ long dwIsAlivePollInterval,
            /* [retval][out] */ __RPC__deref_out_opt ISClusResType **ppResourceType);
        
        DECLSPEC_XFGVIRT(ISClusResTypes, DeleteItem)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *DeleteItem )( 
            __RPC__in ISClusResTypes * This,
            /* [in] */ VARIANT varIndex);
        
        END_INTERFACE
    } ISClusResTypesVtbl;

    interface ISClusResTypes
    {
        CONST_VTBL struct ISClusResTypesVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ISClusResTypes_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ISClusResTypes_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ISClusResTypes_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ISClusResTypes_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define ISClusResTypes_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define ISClusResTypes_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define ISClusResTypes_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define ISClusResTypes_get_Count(This,plCount)	\
    ( (This)->lpVtbl -> get_Count(This,plCount) ) 

#define ISClusResTypes_get__NewEnum(This,retval)	\
    ( (This)->lpVtbl -> get__NewEnum(This,retval) ) 

#define ISClusResTypes_Refresh(This)	\
    ( (This)->lpVtbl -> Refresh(This) ) 

#define ISClusResTypes_get_Item(This,varIndex,ppClusResType)	\
    ( (This)->lpVtbl -> get_Item(This,varIndex,ppClusResType) ) 

#define ISClusResTypes_CreateItem(This,bstrResourceTypeName,bstrDisplayName,bstrResourceTypeDll,dwLooksAlivePollInterval,dwIsAlivePollInterval,ppResourceType)	\
    ( (This)->lpVtbl -> CreateItem(This,bstrResourceTypeName,bstrDisplayName,bstrResourceTypeDll,dwLooksAlivePollInterval,dwIsAlivePollInterval,ppResourceType) ) 

#define ISClusResTypes_DeleteItem(This,varIndex)	\
    ( (This)->lpVtbl -> DeleteItem(This,varIndex) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ISClusResTypes_INTERFACE_DEFINED__ */


#ifndef __ISClusProperty_INTERFACE_DEFINED__
#define __ISClusProperty_INTERFACE_DEFINED__

/* interface ISClusProperty */
/* [unique][helpstring][dual][uuid][object][nonextensible][hidden][oleautomation] */ 


EXTERN_C const IID IID_ISClusProperty;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("f2e606fe-2631-11d1-89f1-00a0c90d061e")
    ISClusProperty : public IDispatch
    {
    public:
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Name( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrName) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Length( 
            /* [retval][out] */ __RPC__out long *pLength) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_ValueCount( 
            /* [retval][out] */ __RPC__out long *pCount) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Values( 
            /* [retval][out] */ __RPC__deref_out_opt ISClusPropertyValues **ppClusterPropertyValues) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Value( 
            /* [retval][out] */ __RPC__out VARIANT *pvarValue) = 0;
        
        virtual /* [helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_Value( 
            /* [in] */ VARIANT varValue) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Type( 
            /* [retval][out] */ __RPC__out CLUSTER_PROPERTY_TYPE *pType) = 0;
        
        virtual /* [helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_Type( 
            /* [in] */ CLUSTER_PROPERTY_TYPE Type) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Format( 
            /* [retval][out] */ __RPC__out CLUSTER_PROPERTY_FORMAT *pFormat) = 0;
        
        virtual /* [helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_Format( 
            /* [in] */ CLUSTER_PROPERTY_FORMAT Format) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_ReadOnly( 
            /* [retval][out] */ __RPC__out VARIANT *pvarReadOnly) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Private( 
            /* [retval][out] */ __RPC__out VARIANT *pvarPrivate) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Common( 
            /* [retval][out] */ __RPC__out VARIANT *pvarCommon) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Modified( 
            /* [retval][out] */ __RPC__out VARIANT *pvarModified) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE UseDefaultValue( void) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ISClusPropertyVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in ISClusProperty * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in ISClusProperty * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in ISClusProperty * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in ISClusProperty * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in ISClusProperty * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in ISClusProperty * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            ISClusProperty * This,
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
        
        DECLSPEC_XFGVIRT(ISClusProperty, get_Name)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Name )( 
            __RPC__in ISClusProperty * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrName);
        
        DECLSPEC_XFGVIRT(ISClusProperty, get_Length)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Length )( 
            __RPC__in ISClusProperty * This,
            /* [retval][out] */ __RPC__out long *pLength);
        
        DECLSPEC_XFGVIRT(ISClusProperty, get_ValueCount)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_ValueCount )( 
            __RPC__in ISClusProperty * This,
            /* [retval][out] */ __RPC__out long *pCount);
        
        DECLSPEC_XFGVIRT(ISClusProperty, get_Values)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Values )( 
            __RPC__in ISClusProperty * This,
            /* [retval][out] */ __RPC__deref_out_opt ISClusPropertyValues **ppClusterPropertyValues);
        
        DECLSPEC_XFGVIRT(ISClusProperty, get_Value)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Value )( 
            __RPC__in ISClusProperty * This,
            /* [retval][out] */ __RPC__out VARIANT *pvarValue);
        
        DECLSPEC_XFGVIRT(ISClusProperty, put_Value)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_Value )( 
            __RPC__in ISClusProperty * This,
            /* [in] */ VARIANT varValue);
        
        DECLSPEC_XFGVIRT(ISClusProperty, get_Type)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Type )( 
            __RPC__in ISClusProperty * This,
            /* [retval][out] */ __RPC__out CLUSTER_PROPERTY_TYPE *pType);
        
        DECLSPEC_XFGVIRT(ISClusProperty, put_Type)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_Type )( 
            __RPC__in ISClusProperty * This,
            /* [in] */ CLUSTER_PROPERTY_TYPE Type);
        
        DECLSPEC_XFGVIRT(ISClusProperty, get_Format)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Format )( 
            __RPC__in ISClusProperty * This,
            /* [retval][out] */ __RPC__out CLUSTER_PROPERTY_FORMAT *pFormat);
        
        DECLSPEC_XFGVIRT(ISClusProperty, put_Format)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_Format )( 
            __RPC__in ISClusProperty * This,
            /* [in] */ CLUSTER_PROPERTY_FORMAT Format);
        
        DECLSPEC_XFGVIRT(ISClusProperty, get_ReadOnly)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_ReadOnly )( 
            __RPC__in ISClusProperty * This,
            /* [retval][out] */ __RPC__out VARIANT *pvarReadOnly);
        
        DECLSPEC_XFGVIRT(ISClusProperty, get_Private)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Private )( 
            __RPC__in ISClusProperty * This,
            /* [retval][out] */ __RPC__out VARIANT *pvarPrivate);
        
        DECLSPEC_XFGVIRT(ISClusProperty, get_Common)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Common )( 
            __RPC__in ISClusProperty * This,
            /* [retval][out] */ __RPC__out VARIANT *pvarCommon);
        
        DECLSPEC_XFGVIRT(ISClusProperty, get_Modified)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Modified )( 
            __RPC__in ISClusProperty * This,
            /* [retval][out] */ __RPC__out VARIANT *pvarModified);
        
        DECLSPEC_XFGVIRT(ISClusProperty, UseDefaultValue)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *UseDefaultValue )( 
            __RPC__in ISClusProperty * This);
        
        END_INTERFACE
    } ISClusPropertyVtbl;

    interface ISClusProperty
    {
        CONST_VTBL struct ISClusPropertyVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ISClusProperty_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ISClusProperty_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ISClusProperty_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ISClusProperty_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define ISClusProperty_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define ISClusProperty_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define ISClusProperty_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define ISClusProperty_get_Name(This,pbstrName)	\
    ( (This)->lpVtbl -> get_Name(This,pbstrName) ) 

#define ISClusProperty_get_Length(This,pLength)	\
    ( (This)->lpVtbl -> get_Length(This,pLength) ) 

#define ISClusProperty_get_ValueCount(This,pCount)	\
    ( (This)->lpVtbl -> get_ValueCount(This,pCount) ) 

#define ISClusProperty_get_Values(This,ppClusterPropertyValues)	\
    ( (This)->lpVtbl -> get_Values(This,ppClusterPropertyValues) ) 

#define ISClusProperty_get_Value(This,pvarValue)	\
    ( (This)->lpVtbl -> get_Value(This,pvarValue) ) 

#define ISClusProperty_put_Value(This,varValue)	\
    ( (This)->lpVtbl -> put_Value(This,varValue) ) 

#define ISClusProperty_get_Type(This,pType)	\
    ( (This)->lpVtbl -> get_Type(This,pType) ) 

#define ISClusProperty_put_Type(This,Type)	\
    ( (This)->lpVtbl -> put_Type(This,Type) ) 

#define ISClusProperty_get_Format(This,pFormat)	\
    ( (This)->lpVtbl -> get_Format(This,pFormat) ) 

#define ISClusProperty_put_Format(This,Format)	\
    ( (This)->lpVtbl -> put_Format(This,Format) ) 

#define ISClusProperty_get_ReadOnly(This,pvarReadOnly)	\
    ( (This)->lpVtbl -> get_ReadOnly(This,pvarReadOnly) ) 

#define ISClusProperty_get_Private(This,pvarPrivate)	\
    ( (This)->lpVtbl -> get_Private(This,pvarPrivate) ) 

#define ISClusProperty_get_Common(This,pvarCommon)	\
    ( (This)->lpVtbl -> get_Common(This,pvarCommon) ) 

#define ISClusProperty_get_Modified(This,pvarModified)	\
    ( (This)->lpVtbl -> get_Modified(This,pvarModified) ) 

#define ISClusProperty_UseDefaultValue(This)	\
    ( (This)->lpVtbl -> UseDefaultValue(This) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ISClusProperty_INTERFACE_DEFINED__ */


#ifndef __ISClusPropertyValue_INTERFACE_DEFINED__
#define __ISClusPropertyValue_INTERFACE_DEFINED__

/* interface ISClusPropertyValue */
/* [unique][helpstring][dual][uuid][object][nonextensible][hidden][oleautomation] */ 


EXTERN_C const IID IID_ISClusPropertyValue;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("f2e6071a-2631-11d1-89f1-00a0c90d061e")
    ISClusPropertyValue : public IDispatch
    {
    public:
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Value( 
            /* [retval][out] */ __RPC__out VARIANT *pvarValue) = 0;
        
        virtual /* [helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_Value( 
            /* [in] */ VARIANT varValue) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Type( 
            /* [retval][out] */ __RPC__out CLUSTER_PROPERTY_TYPE *pType) = 0;
        
        virtual /* [helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_Type( 
            /* [in] */ CLUSTER_PROPERTY_TYPE Type) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Format( 
            /* [retval][out] */ __RPC__out CLUSTER_PROPERTY_FORMAT *pFormat) = 0;
        
        virtual /* [helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_Format( 
            /* [in] */ CLUSTER_PROPERTY_FORMAT Format) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Length( 
            /* [retval][out] */ __RPC__out long *pLength) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_DataCount( 
            /* [retval][out] */ __RPC__out long *pCount) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Data( 
            /* [retval][out] */ __RPC__deref_out_opt ISClusPropertyValueData **ppClusterPropertyValueData) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ISClusPropertyValueVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in ISClusPropertyValue * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in ISClusPropertyValue * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in ISClusPropertyValue * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in ISClusPropertyValue * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in ISClusPropertyValue * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in ISClusPropertyValue * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            ISClusPropertyValue * This,
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
        
        DECLSPEC_XFGVIRT(ISClusPropertyValue, get_Value)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Value )( 
            __RPC__in ISClusPropertyValue * This,
            /* [retval][out] */ __RPC__out VARIANT *pvarValue);
        
        DECLSPEC_XFGVIRT(ISClusPropertyValue, put_Value)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_Value )( 
            __RPC__in ISClusPropertyValue * This,
            /* [in] */ VARIANT varValue);
        
        DECLSPEC_XFGVIRT(ISClusPropertyValue, get_Type)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Type )( 
            __RPC__in ISClusPropertyValue * This,
            /* [retval][out] */ __RPC__out CLUSTER_PROPERTY_TYPE *pType);
        
        DECLSPEC_XFGVIRT(ISClusPropertyValue, put_Type)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_Type )( 
            __RPC__in ISClusPropertyValue * This,
            /* [in] */ CLUSTER_PROPERTY_TYPE Type);
        
        DECLSPEC_XFGVIRT(ISClusPropertyValue, get_Format)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Format )( 
            __RPC__in ISClusPropertyValue * This,
            /* [retval][out] */ __RPC__out CLUSTER_PROPERTY_FORMAT *pFormat);
        
        DECLSPEC_XFGVIRT(ISClusPropertyValue, put_Format)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_Format )( 
            __RPC__in ISClusPropertyValue * This,
            /* [in] */ CLUSTER_PROPERTY_FORMAT Format);
        
        DECLSPEC_XFGVIRT(ISClusPropertyValue, get_Length)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Length )( 
            __RPC__in ISClusPropertyValue * This,
            /* [retval][out] */ __RPC__out long *pLength);
        
        DECLSPEC_XFGVIRT(ISClusPropertyValue, get_DataCount)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_DataCount )( 
            __RPC__in ISClusPropertyValue * This,
            /* [retval][out] */ __RPC__out long *pCount);
        
        DECLSPEC_XFGVIRT(ISClusPropertyValue, get_Data)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Data )( 
            __RPC__in ISClusPropertyValue * This,
            /* [retval][out] */ __RPC__deref_out_opt ISClusPropertyValueData **ppClusterPropertyValueData);
        
        END_INTERFACE
    } ISClusPropertyValueVtbl;

    interface ISClusPropertyValue
    {
        CONST_VTBL struct ISClusPropertyValueVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ISClusPropertyValue_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ISClusPropertyValue_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ISClusPropertyValue_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ISClusPropertyValue_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define ISClusPropertyValue_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define ISClusPropertyValue_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define ISClusPropertyValue_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define ISClusPropertyValue_get_Value(This,pvarValue)	\
    ( (This)->lpVtbl -> get_Value(This,pvarValue) ) 

#define ISClusPropertyValue_put_Value(This,varValue)	\
    ( (This)->lpVtbl -> put_Value(This,varValue) ) 

#define ISClusPropertyValue_get_Type(This,pType)	\
    ( (This)->lpVtbl -> get_Type(This,pType) ) 

#define ISClusPropertyValue_put_Type(This,Type)	\
    ( (This)->lpVtbl -> put_Type(This,Type) ) 

#define ISClusPropertyValue_get_Format(This,pFormat)	\
    ( (This)->lpVtbl -> get_Format(This,pFormat) ) 

#define ISClusPropertyValue_put_Format(This,Format)	\
    ( (This)->lpVtbl -> put_Format(This,Format) ) 

#define ISClusPropertyValue_get_Length(This,pLength)	\
    ( (This)->lpVtbl -> get_Length(This,pLength) ) 

#define ISClusPropertyValue_get_DataCount(This,pCount)	\
    ( (This)->lpVtbl -> get_DataCount(This,pCount) ) 

#define ISClusPropertyValue_get_Data(This,ppClusterPropertyValueData)	\
    ( (This)->lpVtbl -> get_Data(This,ppClusterPropertyValueData) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ISClusPropertyValue_INTERFACE_DEFINED__ */


#ifndef __ISClusPropertyValues_INTERFACE_DEFINED__
#define __ISClusPropertyValues_INTERFACE_DEFINED__

/* interface ISClusPropertyValues */
/* [unique][helpstring][dual][uuid][object][nonextensible][hidden][oleautomation] */ 


EXTERN_C const IID IID_ISClusPropertyValues;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("f2e6071c-2631-11d1-89f1-00a0c90d061e")
    ISClusPropertyValues : public IDispatch
    {
    public:
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Count( 
            /* [retval][out] */ __RPC__out long *plCount) = 0;
        
        virtual /* [helpstring][id][restricted][propget] */ HRESULT STDMETHODCALLTYPE get__NewEnum( 
            /* [retval][out] */ __RPC__deref_out_opt IUnknown **retval) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Item( 
            /* [in] */ VARIANT varIndex,
            /* [retval][out] */ __RPC__deref_out_opt ISClusPropertyValue **ppPropertyValue) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE CreateItem( 
            /* [in] */ __RPC__in BSTR bstrName,
            /* [in] */ VARIANT varValue,
            /* [retval][out] */ __RPC__deref_out_opt ISClusPropertyValue **ppPropertyValue) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE RemoveItem( 
            /* [in] */ VARIANT varIndex) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ISClusPropertyValuesVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in ISClusPropertyValues * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in ISClusPropertyValues * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in ISClusPropertyValues * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in ISClusPropertyValues * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in ISClusPropertyValues * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in ISClusPropertyValues * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            ISClusPropertyValues * This,
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
        
        DECLSPEC_XFGVIRT(ISClusPropertyValues, get_Count)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Count )( 
            __RPC__in ISClusPropertyValues * This,
            /* [retval][out] */ __RPC__out long *plCount);
        
        DECLSPEC_XFGVIRT(ISClusPropertyValues, get__NewEnum)
        /* [helpstring][id][restricted][propget] */ HRESULT ( STDMETHODCALLTYPE *get__NewEnum )( 
            __RPC__in ISClusPropertyValues * This,
            /* [retval][out] */ __RPC__deref_out_opt IUnknown **retval);
        
        DECLSPEC_XFGVIRT(ISClusPropertyValues, get_Item)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Item )( 
            __RPC__in ISClusPropertyValues * This,
            /* [in] */ VARIANT varIndex,
            /* [retval][out] */ __RPC__deref_out_opt ISClusPropertyValue **ppPropertyValue);
        
        DECLSPEC_XFGVIRT(ISClusPropertyValues, CreateItem)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *CreateItem )( 
            __RPC__in ISClusPropertyValues * This,
            /* [in] */ __RPC__in BSTR bstrName,
            /* [in] */ VARIANT varValue,
            /* [retval][out] */ __RPC__deref_out_opt ISClusPropertyValue **ppPropertyValue);
        
        DECLSPEC_XFGVIRT(ISClusPropertyValues, RemoveItem)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *RemoveItem )( 
            __RPC__in ISClusPropertyValues * This,
            /* [in] */ VARIANT varIndex);
        
        END_INTERFACE
    } ISClusPropertyValuesVtbl;

    interface ISClusPropertyValues
    {
        CONST_VTBL struct ISClusPropertyValuesVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ISClusPropertyValues_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ISClusPropertyValues_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ISClusPropertyValues_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ISClusPropertyValues_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define ISClusPropertyValues_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define ISClusPropertyValues_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define ISClusPropertyValues_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define ISClusPropertyValues_get_Count(This,plCount)	\
    ( (This)->lpVtbl -> get_Count(This,plCount) ) 

#define ISClusPropertyValues_get__NewEnum(This,retval)	\
    ( (This)->lpVtbl -> get__NewEnum(This,retval) ) 

#define ISClusPropertyValues_get_Item(This,varIndex,ppPropertyValue)	\
    ( (This)->lpVtbl -> get_Item(This,varIndex,ppPropertyValue) ) 

#define ISClusPropertyValues_CreateItem(This,bstrName,varValue,ppPropertyValue)	\
    ( (This)->lpVtbl -> CreateItem(This,bstrName,varValue,ppPropertyValue) ) 

#define ISClusPropertyValues_RemoveItem(This,varIndex)	\
    ( (This)->lpVtbl -> RemoveItem(This,varIndex) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ISClusPropertyValues_INTERFACE_DEFINED__ */


#ifndef __ISClusProperties_INTERFACE_DEFINED__
#define __ISClusProperties_INTERFACE_DEFINED__

/* interface ISClusProperties */
/* [unique][helpstring][dual][uuid][object][nonextensible][hidden][oleautomation] */ 


EXTERN_C const IID IID_ISClusProperties;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("f2e60700-2631-11d1-89f1-00a0c90d061e")
    ISClusProperties : public IDispatch
    {
    public:
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Count( 
            /* [retval][out] */ __RPC__out long *plCount) = 0;
        
        virtual /* [helpstring][id][restricted][propget] */ HRESULT STDMETHODCALLTYPE get__NewEnum( 
            /* [retval][out] */ __RPC__deref_out_opt IUnknown **retval) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE Refresh( void) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Item( 
            /* [in] */ VARIANT varIndex,
            /* [retval][out] */ __RPC__deref_out_opt ISClusProperty **ppClusProperty) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE CreateItem( 
            /* [in] */ __RPC__in BSTR bstrName,
            /* [in] */ VARIANT varValue,
            /* [retval][out] */ __RPC__deref_out_opt ISClusProperty **pProperty) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE UseDefaultValue( 
            /* [in] */ VARIANT varIndex) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE SaveChanges( 
            /* [defaultvalue][out] */ __RPC__out VARIANT *pvarStatusCode = 0) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_ReadOnly( 
            /* [retval][out] */ __RPC__out VARIANT *pvarReadOnly) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Private( 
            /* [retval][out] */ __RPC__out VARIANT *pvarPrivate) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Common( 
            /* [retval][out] */ __RPC__out VARIANT *pvarCommon) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Modified( 
            /* [retval][out] */ __RPC__out VARIANT *pvarModified) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ISClusPropertiesVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in ISClusProperties * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in ISClusProperties * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in ISClusProperties * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in ISClusProperties * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in ISClusProperties * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in ISClusProperties * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            ISClusProperties * This,
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
        
        DECLSPEC_XFGVIRT(ISClusProperties, get_Count)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Count )( 
            __RPC__in ISClusProperties * This,
            /* [retval][out] */ __RPC__out long *plCount);
        
        DECLSPEC_XFGVIRT(ISClusProperties, get__NewEnum)
        /* [helpstring][id][restricted][propget] */ HRESULT ( STDMETHODCALLTYPE *get__NewEnum )( 
            __RPC__in ISClusProperties * This,
            /* [retval][out] */ __RPC__deref_out_opt IUnknown **retval);
        
        DECLSPEC_XFGVIRT(ISClusProperties, Refresh)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *Refresh )( 
            __RPC__in ISClusProperties * This);
        
        DECLSPEC_XFGVIRT(ISClusProperties, get_Item)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Item )( 
            __RPC__in ISClusProperties * This,
            /* [in] */ VARIANT varIndex,
            /* [retval][out] */ __RPC__deref_out_opt ISClusProperty **ppClusProperty);
        
        DECLSPEC_XFGVIRT(ISClusProperties, CreateItem)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *CreateItem )( 
            __RPC__in ISClusProperties * This,
            /* [in] */ __RPC__in BSTR bstrName,
            /* [in] */ VARIANT varValue,
            /* [retval][out] */ __RPC__deref_out_opt ISClusProperty **pProperty);
        
        DECLSPEC_XFGVIRT(ISClusProperties, UseDefaultValue)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *UseDefaultValue )( 
            __RPC__in ISClusProperties * This,
            /* [in] */ VARIANT varIndex);
        
        DECLSPEC_XFGVIRT(ISClusProperties, SaveChanges)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *SaveChanges )( 
            __RPC__in ISClusProperties * This,
            /* [defaultvalue][out] */ __RPC__out VARIANT *pvarStatusCode);
        
        DECLSPEC_XFGVIRT(ISClusProperties, get_ReadOnly)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_ReadOnly )( 
            __RPC__in ISClusProperties * This,
            /* [retval][out] */ __RPC__out VARIANT *pvarReadOnly);
        
        DECLSPEC_XFGVIRT(ISClusProperties, get_Private)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Private )( 
            __RPC__in ISClusProperties * This,
            /* [retval][out] */ __RPC__out VARIANT *pvarPrivate);
        
        DECLSPEC_XFGVIRT(ISClusProperties, get_Common)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Common )( 
            __RPC__in ISClusProperties * This,
            /* [retval][out] */ __RPC__out VARIANT *pvarCommon);
        
        DECLSPEC_XFGVIRT(ISClusProperties, get_Modified)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Modified )( 
            __RPC__in ISClusProperties * This,
            /* [retval][out] */ __RPC__out VARIANT *pvarModified);
        
        END_INTERFACE
    } ISClusPropertiesVtbl;

    interface ISClusProperties
    {
        CONST_VTBL struct ISClusPropertiesVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ISClusProperties_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ISClusProperties_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ISClusProperties_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ISClusProperties_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define ISClusProperties_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define ISClusProperties_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define ISClusProperties_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define ISClusProperties_get_Count(This,plCount)	\
    ( (This)->lpVtbl -> get_Count(This,plCount) ) 

#define ISClusProperties_get__NewEnum(This,retval)	\
    ( (This)->lpVtbl -> get__NewEnum(This,retval) ) 

#define ISClusProperties_Refresh(This)	\
    ( (This)->lpVtbl -> Refresh(This) ) 

#define ISClusProperties_get_Item(This,varIndex,ppClusProperty)	\
    ( (This)->lpVtbl -> get_Item(This,varIndex,ppClusProperty) ) 

#define ISClusProperties_CreateItem(This,bstrName,varValue,pProperty)	\
    ( (This)->lpVtbl -> CreateItem(This,bstrName,varValue,pProperty) ) 

#define ISClusProperties_UseDefaultValue(This,varIndex)	\
    ( (This)->lpVtbl -> UseDefaultValue(This,varIndex) ) 

#define ISClusProperties_SaveChanges(This,pvarStatusCode)	\
    ( (This)->lpVtbl -> SaveChanges(This,pvarStatusCode) ) 

#define ISClusProperties_get_ReadOnly(This,pvarReadOnly)	\
    ( (This)->lpVtbl -> get_ReadOnly(This,pvarReadOnly) ) 

#define ISClusProperties_get_Private(This,pvarPrivate)	\
    ( (This)->lpVtbl -> get_Private(This,pvarPrivate) ) 

#define ISClusProperties_get_Common(This,pvarCommon)	\
    ( (This)->lpVtbl -> get_Common(This,pvarCommon) ) 

#define ISClusProperties_get_Modified(This,pvarModified)	\
    ( (This)->lpVtbl -> get_Modified(This,pvarModified) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ISClusProperties_INTERFACE_DEFINED__ */


#ifndef __ISClusPropertyValueData_INTERFACE_DEFINED__
#define __ISClusPropertyValueData_INTERFACE_DEFINED__

/* interface ISClusPropertyValueData */
/* [unique][helpstring][dual][uuid][object][nonextensible][hidden][oleautomation] */ 


EXTERN_C const IID IID_ISClusPropertyValueData;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("f2e6071e-2631-11d1-89f1-00a0c90d061e")
    ISClusPropertyValueData : public IDispatch
    {
    public:
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Count( 
            /* [retval][out] */ __RPC__out long *plCount) = 0;
        
        virtual /* [helpstring][id][restricted][propget] */ HRESULT STDMETHODCALLTYPE get__NewEnum( 
            /* [retval][out] */ __RPC__deref_out_opt IUnknown **retval) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Item( 
            /* [in] */ VARIANT varIndex,
            /* [retval][out] */ __RPC__out VARIANT *pvarValue) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE CreateItem( 
            /* [in] */ VARIANT varValue,
            /* [retval][out] */ __RPC__out VARIANT *pvarData) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE RemoveItem( 
            /* [in] */ VARIANT varIndex) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ISClusPropertyValueDataVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in ISClusPropertyValueData * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in ISClusPropertyValueData * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in ISClusPropertyValueData * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in ISClusPropertyValueData * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in ISClusPropertyValueData * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in ISClusPropertyValueData * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            ISClusPropertyValueData * This,
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
        
        DECLSPEC_XFGVIRT(ISClusPropertyValueData, get_Count)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Count )( 
            __RPC__in ISClusPropertyValueData * This,
            /* [retval][out] */ __RPC__out long *plCount);
        
        DECLSPEC_XFGVIRT(ISClusPropertyValueData, get__NewEnum)
        /* [helpstring][id][restricted][propget] */ HRESULT ( STDMETHODCALLTYPE *get__NewEnum )( 
            __RPC__in ISClusPropertyValueData * This,
            /* [retval][out] */ __RPC__deref_out_opt IUnknown **retval);
        
        DECLSPEC_XFGVIRT(ISClusPropertyValueData, get_Item)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Item )( 
            __RPC__in ISClusPropertyValueData * This,
            /* [in] */ VARIANT varIndex,
            /* [retval][out] */ __RPC__out VARIANT *pvarValue);
        
        DECLSPEC_XFGVIRT(ISClusPropertyValueData, CreateItem)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *CreateItem )( 
            __RPC__in ISClusPropertyValueData * This,
            /* [in] */ VARIANT varValue,
            /* [retval][out] */ __RPC__out VARIANT *pvarData);
        
        DECLSPEC_XFGVIRT(ISClusPropertyValueData, RemoveItem)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *RemoveItem )( 
            __RPC__in ISClusPropertyValueData * This,
            /* [in] */ VARIANT varIndex);
        
        END_INTERFACE
    } ISClusPropertyValueDataVtbl;

    interface ISClusPropertyValueData
    {
        CONST_VTBL struct ISClusPropertyValueDataVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ISClusPropertyValueData_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ISClusPropertyValueData_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ISClusPropertyValueData_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ISClusPropertyValueData_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define ISClusPropertyValueData_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define ISClusPropertyValueData_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define ISClusPropertyValueData_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define ISClusPropertyValueData_get_Count(This,plCount)	\
    ( (This)->lpVtbl -> get_Count(This,plCount) ) 

#define ISClusPropertyValueData_get__NewEnum(This,retval)	\
    ( (This)->lpVtbl -> get__NewEnum(This,retval) ) 

#define ISClusPropertyValueData_get_Item(This,varIndex,pvarValue)	\
    ( (This)->lpVtbl -> get_Item(This,varIndex,pvarValue) ) 

#define ISClusPropertyValueData_CreateItem(This,varValue,pvarData)	\
    ( (This)->lpVtbl -> CreateItem(This,varValue,pvarData) ) 

#define ISClusPropertyValueData_RemoveItem(This,varIndex)	\
    ( (This)->lpVtbl -> RemoveItem(This,varIndex) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ISClusPropertyValueData_INTERFACE_DEFINED__ */


#ifndef __ISClusPartition_INTERFACE_DEFINED__
#define __ISClusPartition_INTERFACE_DEFINED__

/* interface ISClusPartition */
/* [unique][helpstring][dual][uuid][object][nonextensible][hidden][oleautomation] */ 


EXTERN_C const IID IID_ISClusPartition;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("f2e60720-2631-11d1-89f1-00a0c90d061e")
    ISClusPartition : public IDispatch
    {
    public:
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Flags( 
            /* [retval][out] */ __RPC__out long *plFlags) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_DeviceName( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrDeviceName) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_VolumeLabel( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrVolumeLabel) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_SerialNumber( 
            /* [retval][out] */ __RPC__out long *plSerialNumber) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_MaximumComponentLength( 
            /* [retval][out] */ __RPC__out long *plMaximumComponentLength) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_FileSystemFlags( 
            /* [retval][out] */ __RPC__out long *plFileSystemFlags) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_FileSystem( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrFileSystem) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ISClusPartitionVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in ISClusPartition * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in ISClusPartition * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in ISClusPartition * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in ISClusPartition * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in ISClusPartition * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in ISClusPartition * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            ISClusPartition * This,
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
        
        DECLSPEC_XFGVIRT(ISClusPartition, get_Flags)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Flags )( 
            __RPC__in ISClusPartition * This,
            /* [retval][out] */ __RPC__out long *plFlags);
        
        DECLSPEC_XFGVIRT(ISClusPartition, get_DeviceName)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_DeviceName )( 
            __RPC__in ISClusPartition * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrDeviceName);
        
        DECLSPEC_XFGVIRT(ISClusPartition, get_VolumeLabel)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_VolumeLabel )( 
            __RPC__in ISClusPartition * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrVolumeLabel);
        
        DECLSPEC_XFGVIRT(ISClusPartition, get_SerialNumber)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_SerialNumber )( 
            __RPC__in ISClusPartition * This,
            /* [retval][out] */ __RPC__out long *plSerialNumber);
        
        DECLSPEC_XFGVIRT(ISClusPartition, get_MaximumComponentLength)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_MaximumComponentLength )( 
            __RPC__in ISClusPartition * This,
            /* [retval][out] */ __RPC__out long *plMaximumComponentLength);
        
        DECLSPEC_XFGVIRT(ISClusPartition, get_FileSystemFlags)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_FileSystemFlags )( 
            __RPC__in ISClusPartition * This,
            /* [retval][out] */ __RPC__out long *plFileSystemFlags);
        
        DECLSPEC_XFGVIRT(ISClusPartition, get_FileSystem)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_FileSystem )( 
            __RPC__in ISClusPartition * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrFileSystem);
        
        END_INTERFACE
    } ISClusPartitionVtbl;

    interface ISClusPartition
    {
        CONST_VTBL struct ISClusPartitionVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ISClusPartition_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ISClusPartition_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ISClusPartition_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ISClusPartition_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define ISClusPartition_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define ISClusPartition_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define ISClusPartition_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define ISClusPartition_get_Flags(This,plFlags)	\
    ( (This)->lpVtbl -> get_Flags(This,plFlags) ) 

#define ISClusPartition_get_DeviceName(This,pbstrDeviceName)	\
    ( (This)->lpVtbl -> get_DeviceName(This,pbstrDeviceName) ) 

#define ISClusPartition_get_VolumeLabel(This,pbstrVolumeLabel)	\
    ( (This)->lpVtbl -> get_VolumeLabel(This,pbstrVolumeLabel) ) 

#define ISClusPartition_get_SerialNumber(This,plSerialNumber)	\
    ( (This)->lpVtbl -> get_SerialNumber(This,plSerialNumber) ) 

#define ISClusPartition_get_MaximumComponentLength(This,plMaximumComponentLength)	\
    ( (This)->lpVtbl -> get_MaximumComponentLength(This,plMaximumComponentLength) ) 

#define ISClusPartition_get_FileSystemFlags(This,plFileSystemFlags)	\
    ( (This)->lpVtbl -> get_FileSystemFlags(This,plFileSystemFlags) ) 

#define ISClusPartition_get_FileSystem(This,pbstrFileSystem)	\
    ( (This)->lpVtbl -> get_FileSystem(This,pbstrFileSystem) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ISClusPartition_INTERFACE_DEFINED__ */


#ifndef __ISClusPartitionEx_INTERFACE_DEFINED__
#define __ISClusPartitionEx_INTERFACE_DEFINED__

/* interface ISClusPartitionEx */
/* [unique][helpstring][dual][uuid][object][nonextensible][hidden][oleautomation] */ 


EXTERN_C const IID IID_ISClusPartitionEx;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("8802d4fe-b32e-4ad1-9dbd-64f18e1166ce")
    ISClusPartitionEx : public ISClusPartition
    {
    public:
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_TotalSize( 
            /* [retval][out] */ __RPC__out long *plTotalSize) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_FreeSpace( 
            /* [retval][out] */ __RPC__out long *plFreeSpace) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_DeviceNumber( 
            /* [retval][out] */ __RPC__out long *plDeviceNumber) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_PartitionNumber( 
            /* [retval][out] */ __RPC__out long *plPartitionNumber) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_VolumeGuid( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrVolumeGuid) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ISClusPartitionExVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in ISClusPartitionEx * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in ISClusPartitionEx * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in ISClusPartitionEx * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in ISClusPartitionEx * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in ISClusPartitionEx * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in ISClusPartitionEx * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            ISClusPartitionEx * This,
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
        
        DECLSPEC_XFGVIRT(ISClusPartition, get_Flags)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Flags )( 
            __RPC__in ISClusPartitionEx * This,
            /* [retval][out] */ __RPC__out long *plFlags);
        
        DECLSPEC_XFGVIRT(ISClusPartition, get_DeviceName)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_DeviceName )( 
            __RPC__in ISClusPartitionEx * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrDeviceName);
        
        DECLSPEC_XFGVIRT(ISClusPartition, get_VolumeLabel)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_VolumeLabel )( 
            __RPC__in ISClusPartitionEx * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrVolumeLabel);
        
        DECLSPEC_XFGVIRT(ISClusPartition, get_SerialNumber)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_SerialNumber )( 
            __RPC__in ISClusPartitionEx * This,
            /* [retval][out] */ __RPC__out long *plSerialNumber);
        
        DECLSPEC_XFGVIRT(ISClusPartition, get_MaximumComponentLength)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_MaximumComponentLength )( 
            __RPC__in ISClusPartitionEx * This,
            /* [retval][out] */ __RPC__out long *plMaximumComponentLength);
        
        DECLSPEC_XFGVIRT(ISClusPartition, get_FileSystemFlags)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_FileSystemFlags )( 
            __RPC__in ISClusPartitionEx * This,
            /* [retval][out] */ __RPC__out long *plFileSystemFlags);
        
        DECLSPEC_XFGVIRT(ISClusPartition, get_FileSystem)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_FileSystem )( 
            __RPC__in ISClusPartitionEx * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrFileSystem);
        
        DECLSPEC_XFGVIRT(ISClusPartitionEx, get_TotalSize)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_TotalSize )( 
            __RPC__in ISClusPartitionEx * This,
            /* [retval][out] */ __RPC__out long *plTotalSize);
        
        DECLSPEC_XFGVIRT(ISClusPartitionEx, get_FreeSpace)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_FreeSpace )( 
            __RPC__in ISClusPartitionEx * This,
            /* [retval][out] */ __RPC__out long *plFreeSpace);
        
        DECLSPEC_XFGVIRT(ISClusPartitionEx, get_DeviceNumber)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_DeviceNumber )( 
            __RPC__in ISClusPartitionEx * This,
            /* [retval][out] */ __RPC__out long *plDeviceNumber);
        
        DECLSPEC_XFGVIRT(ISClusPartitionEx, get_PartitionNumber)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_PartitionNumber )( 
            __RPC__in ISClusPartitionEx * This,
            /* [retval][out] */ __RPC__out long *plPartitionNumber);
        
        DECLSPEC_XFGVIRT(ISClusPartitionEx, get_VolumeGuid)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_VolumeGuid )( 
            __RPC__in ISClusPartitionEx * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrVolumeGuid);
        
        END_INTERFACE
    } ISClusPartitionExVtbl;

    interface ISClusPartitionEx
    {
        CONST_VTBL struct ISClusPartitionExVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ISClusPartitionEx_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ISClusPartitionEx_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ISClusPartitionEx_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ISClusPartitionEx_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define ISClusPartitionEx_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define ISClusPartitionEx_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define ISClusPartitionEx_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define ISClusPartitionEx_get_Flags(This,plFlags)	\
    ( (This)->lpVtbl -> get_Flags(This,plFlags) ) 

#define ISClusPartitionEx_get_DeviceName(This,pbstrDeviceName)	\
    ( (This)->lpVtbl -> get_DeviceName(This,pbstrDeviceName) ) 

#define ISClusPartitionEx_get_VolumeLabel(This,pbstrVolumeLabel)	\
    ( (This)->lpVtbl -> get_VolumeLabel(This,pbstrVolumeLabel) ) 

#define ISClusPartitionEx_get_SerialNumber(This,plSerialNumber)	\
    ( (This)->lpVtbl -> get_SerialNumber(This,plSerialNumber) ) 

#define ISClusPartitionEx_get_MaximumComponentLength(This,plMaximumComponentLength)	\
    ( (This)->lpVtbl -> get_MaximumComponentLength(This,plMaximumComponentLength) ) 

#define ISClusPartitionEx_get_FileSystemFlags(This,plFileSystemFlags)	\
    ( (This)->lpVtbl -> get_FileSystemFlags(This,plFileSystemFlags) ) 

#define ISClusPartitionEx_get_FileSystem(This,pbstrFileSystem)	\
    ( (This)->lpVtbl -> get_FileSystem(This,pbstrFileSystem) ) 


#define ISClusPartitionEx_get_TotalSize(This,plTotalSize)	\
    ( (This)->lpVtbl -> get_TotalSize(This,plTotalSize) ) 

#define ISClusPartitionEx_get_FreeSpace(This,plFreeSpace)	\
    ( (This)->lpVtbl -> get_FreeSpace(This,plFreeSpace) ) 

#define ISClusPartitionEx_get_DeviceNumber(This,plDeviceNumber)	\
    ( (This)->lpVtbl -> get_DeviceNumber(This,plDeviceNumber) ) 

#define ISClusPartitionEx_get_PartitionNumber(This,plPartitionNumber)	\
    ( (This)->lpVtbl -> get_PartitionNumber(This,plPartitionNumber) ) 

#define ISClusPartitionEx_get_VolumeGuid(This,pbstrVolumeGuid)	\
    ( (This)->lpVtbl -> get_VolumeGuid(This,pbstrVolumeGuid) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ISClusPartitionEx_INTERFACE_DEFINED__ */


#ifndef __ISClusPartitions_INTERFACE_DEFINED__
#define __ISClusPartitions_INTERFACE_DEFINED__

/* interface ISClusPartitions */
/* [unique][helpstring][dual][uuid][object][nonextensible][hidden][oleautomation] */ 


EXTERN_C const IID IID_ISClusPartitions;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("f2e60722-2631-11d1-89f1-00a0c90d061e")
    ISClusPartitions : public IDispatch
    {
    public:
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Count( 
            /* [retval][out] */ __RPC__out long *plCount) = 0;
        
        virtual /* [helpstring][id][restricted][propget] */ HRESULT STDMETHODCALLTYPE get__NewEnum( 
            /* [retval][out] */ __RPC__deref_out_opt IUnknown **retval) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Item( 
            /* [in] */ VARIANT varIndex,
            /* [retval][out] */ __RPC__deref_out_opt ISClusPartition **ppPartition) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ISClusPartitionsVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in ISClusPartitions * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in ISClusPartitions * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in ISClusPartitions * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in ISClusPartitions * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in ISClusPartitions * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in ISClusPartitions * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            ISClusPartitions * This,
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
        
        DECLSPEC_XFGVIRT(ISClusPartitions, get_Count)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Count )( 
            __RPC__in ISClusPartitions * This,
            /* [retval][out] */ __RPC__out long *plCount);
        
        DECLSPEC_XFGVIRT(ISClusPartitions, get__NewEnum)
        /* [helpstring][id][restricted][propget] */ HRESULT ( STDMETHODCALLTYPE *get__NewEnum )( 
            __RPC__in ISClusPartitions * This,
            /* [retval][out] */ __RPC__deref_out_opt IUnknown **retval);
        
        DECLSPEC_XFGVIRT(ISClusPartitions, get_Item)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Item )( 
            __RPC__in ISClusPartitions * This,
            /* [in] */ VARIANT varIndex,
            /* [retval][out] */ __RPC__deref_out_opt ISClusPartition **ppPartition);
        
        END_INTERFACE
    } ISClusPartitionsVtbl;

    interface ISClusPartitions
    {
        CONST_VTBL struct ISClusPartitionsVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ISClusPartitions_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ISClusPartitions_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ISClusPartitions_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ISClusPartitions_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define ISClusPartitions_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define ISClusPartitions_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define ISClusPartitions_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define ISClusPartitions_get_Count(This,plCount)	\
    ( (This)->lpVtbl -> get_Count(This,plCount) ) 

#define ISClusPartitions_get__NewEnum(This,retval)	\
    ( (This)->lpVtbl -> get__NewEnum(This,retval) ) 

#define ISClusPartitions_get_Item(This,varIndex,ppPartition)	\
    ( (This)->lpVtbl -> get_Item(This,varIndex,ppPartition) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ISClusPartitions_INTERFACE_DEFINED__ */


#ifndef __ISClusDisk_INTERFACE_DEFINED__
#define __ISClusDisk_INTERFACE_DEFINED__

/* interface ISClusDisk */
/* [unique][helpstring][dual][uuid][object][nonextensible][hidden][oleautomation] */ 


EXTERN_C const IID IID_ISClusDisk;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("f2e60724-2631-11d1-89f1-00a0c90d061e")
    ISClusDisk : public IDispatch
    {
    public:
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Signature( 
            /* [retval][out] */ __RPC__out long *plSignature) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_ScsiAddress( 
            /* [retval][out] */ __RPC__deref_out_opt ISClusScsiAddress **ppScsiAddress) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_DiskNumber( 
            /* [retval][out] */ __RPC__out long *plDiskNumber) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Partitions( 
            /* [retval][out] */ __RPC__deref_out_opt ISClusPartitions **ppPartitions) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ISClusDiskVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in ISClusDisk * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in ISClusDisk * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in ISClusDisk * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in ISClusDisk * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in ISClusDisk * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in ISClusDisk * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            ISClusDisk * This,
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
        
        DECLSPEC_XFGVIRT(ISClusDisk, get_Signature)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Signature )( 
            __RPC__in ISClusDisk * This,
            /* [retval][out] */ __RPC__out long *plSignature);
        
        DECLSPEC_XFGVIRT(ISClusDisk, get_ScsiAddress)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_ScsiAddress )( 
            __RPC__in ISClusDisk * This,
            /* [retval][out] */ __RPC__deref_out_opt ISClusScsiAddress **ppScsiAddress);
        
        DECLSPEC_XFGVIRT(ISClusDisk, get_DiskNumber)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_DiskNumber )( 
            __RPC__in ISClusDisk * This,
            /* [retval][out] */ __RPC__out long *plDiskNumber);
        
        DECLSPEC_XFGVIRT(ISClusDisk, get_Partitions)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Partitions )( 
            __RPC__in ISClusDisk * This,
            /* [retval][out] */ __RPC__deref_out_opt ISClusPartitions **ppPartitions);
        
        END_INTERFACE
    } ISClusDiskVtbl;

    interface ISClusDisk
    {
        CONST_VTBL struct ISClusDiskVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ISClusDisk_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ISClusDisk_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ISClusDisk_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ISClusDisk_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define ISClusDisk_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define ISClusDisk_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define ISClusDisk_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define ISClusDisk_get_Signature(This,plSignature)	\
    ( (This)->lpVtbl -> get_Signature(This,plSignature) ) 

#define ISClusDisk_get_ScsiAddress(This,ppScsiAddress)	\
    ( (This)->lpVtbl -> get_ScsiAddress(This,ppScsiAddress) ) 

#define ISClusDisk_get_DiskNumber(This,plDiskNumber)	\
    ( (This)->lpVtbl -> get_DiskNumber(This,plDiskNumber) ) 

#define ISClusDisk_get_Partitions(This,ppPartitions)	\
    ( (This)->lpVtbl -> get_Partitions(This,ppPartitions) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ISClusDisk_INTERFACE_DEFINED__ */


#ifndef __ISClusDisks_INTERFACE_DEFINED__
#define __ISClusDisks_INTERFACE_DEFINED__

/* interface ISClusDisks */
/* [unique][helpstring][dual][uuid][object][nonextensible][hidden][oleautomation] */ 


EXTERN_C const IID IID_ISClusDisks;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("f2e60726-2631-11d1-89f1-00a0c90d061e")
    ISClusDisks : public IDispatch
    {
    public:
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Count( 
            /* [retval][out] */ __RPC__out long *plCount) = 0;
        
        virtual /* [helpstring][id][restricted][propget] */ HRESULT STDMETHODCALLTYPE get__NewEnum( 
            /* [retval][out] */ __RPC__deref_out_opt IUnknown **retval) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Item( 
            /* [in] */ VARIANT varIndex,
            /* [retval][out] */ __RPC__deref_out_opt ISClusDisk **ppDisk) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ISClusDisksVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in ISClusDisks * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in ISClusDisks * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in ISClusDisks * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in ISClusDisks * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in ISClusDisks * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in ISClusDisks * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            ISClusDisks * This,
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
        
        DECLSPEC_XFGVIRT(ISClusDisks, get_Count)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Count )( 
            __RPC__in ISClusDisks * This,
            /* [retval][out] */ __RPC__out long *plCount);
        
        DECLSPEC_XFGVIRT(ISClusDisks, get__NewEnum)
        /* [helpstring][id][restricted][propget] */ HRESULT ( STDMETHODCALLTYPE *get__NewEnum )( 
            __RPC__in ISClusDisks * This,
            /* [retval][out] */ __RPC__deref_out_opt IUnknown **retval);
        
        DECLSPEC_XFGVIRT(ISClusDisks, get_Item)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Item )( 
            __RPC__in ISClusDisks * This,
            /* [in] */ VARIANT varIndex,
            /* [retval][out] */ __RPC__deref_out_opt ISClusDisk **ppDisk);
        
        END_INTERFACE
    } ISClusDisksVtbl;

    interface ISClusDisks
    {
        CONST_VTBL struct ISClusDisksVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ISClusDisks_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ISClusDisks_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ISClusDisks_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ISClusDisks_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define ISClusDisks_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define ISClusDisks_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define ISClusDisks_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define ISClusDisks_get_Count(This,plCount)	\
    ( (This)->lpVtbl -> get_Count(This,plCount) ) 

#define ISClusDisks_get__NewEnum(This,retval)	\
    ( (This)->lpVtbl -> get__NewEnum(This,retval) ) 

#define ISClusDisks_get_Item(This,varIndex,ppDisk)	\
    ( (This)->lpVtbl -> get_Item(This,varIndex,ppDisk) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ISClusDisks_INTERFACE_DEFINED__ */


#ifndef __ISClusScsiAddress_INTERFACE_DEFINED__
#define __ISClusScsiAddress_INTERFACE_DEFINED__

/* interface ISClusScsiAddress */
/* [unique][helpstring][dual][uuid][object][nonextensible][hidden][oleautomation] */ 


EXTERN_C const IID IID_ISClusScsiAddress;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("f2e60728-2631-11d1-89f1-00a0c90d061e")
    ISClusScsiAddress : public IDispatch
    {
    public:
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_PortNumber( 
            /* [retval][out] */ __RPC__out VARIANT *pvarPortNumber) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_PathId( 
            /* [retval][out] */ __RPC__out VARIANT *pvarPathId) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_TargetId( 
            /* [retval][out] */ __RPC__out VARIANT *pvarTargetId) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Lun( 
            /* [retval][out] */ __RPC__out VARIANT *pvarLun) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ISClusScsiAddressVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in ISClusScsiAddress * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in ISClusScsiAddress * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in ISClusScsiAddress * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in ISClusScsiAddress * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in ISClusScsiAddress * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in ISClusScsiAddress * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            ISClusScsiAddress * This,
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
        
        DECLSPEC_XFGVIRT(ISClusScsiAddress, get_PortNumber)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_PortNumber )( 
            __RPC__in ISClusScsiAddress * This,
            /* [retval][out] */ __RPC__out VARIANT *pvarPortNumber);
        
        DECLSPEC_XFGVIRT(ISClusScsiAddress, get_PathId)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_PathId )( 
            __RPC__in ISClusScsiAddress * This,
            /* [retval][out] */ __RPC__out VARIANT *pvarPathId);
        
        DECLSPEC_XFGVIRT(ISClusScsiAddress, get_TargetId)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_TargetId )( 
            __RPC__in ISClusScsiAddress * This,
            /* [retval][out] */ __RPC__out VARIANT *pvarTargetId);
        
        DECLSPEC_XFGVIRT(ISClusScsiAddress, get_Lun)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Lun )( 
            __RPC__in ISClusScsiAddress * This,
            /* [retval][out] */ __RPC__out VARIANT *pvarLun);
        
        END_INTERFACE
    } ISClusScsiAddressVtbl;

    interface ISClusScsiAddress
    {
        CONST_VTBL struct ISClusScsiAddressVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ISClusScsiAddress_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ISClusScsiAddress_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ISClusScsiAddress_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ISClusScsiAddress_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define ISClusScsiAddress_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define ISClusScsiAddress_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define ISClusScsiAddress_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define ISClusScsiAddress_get_PortNumber(This,pvarPortNumber)	\
    ( (This)->lpVtbl -> get_PortNumber(This,pvarPortNumber) ) 

#define ISClusScsiAddress_get_PathId(This,pvarPathId)	\
    ( (This)->lpVtbl -> get_PathId(This,pvarPathId) ) 

#define ISClusScsiAddress_get_TargetId(This,pvarTargetId)	\
    ( (This)->lpVtbl -> get_TargetId(This,pvarTargetId) ) 

#define ISClusScsiAddress_get_Lun(This,pvarLun)	\
    ( (This)->lpVtbl -> get_Lun(This,pvarLun) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ISClusScsiAddress_INTERFACE_DEFINED__ */


#ifndef __ISClusRegistryKeys_INTERFACE_DEFINED__
#define __ISClusRegistryKeys_INTERFACE_DEFINED__

/* interface ISClusRegistryKeys */
/* [unique][helpstring][dual][uuid][object][nonextensible][hidden][oleautomation] */ 


EXTERN_C const IID IID_ISClusRegistryKeys;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("f2e6072a-2631-11d1-89f1-00a0c90d061e")
    ISClusRegistryKeys : public IDispatch
    {
    public:
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Count( 
            /* [retval][out] */ __RPC__out long *plCount) = 0;
        
        virtual /* [helpstring][id][restricted][propget] */ HRESULT STDMETHODCALLTYPE get__NewEnum( 
            /* [retval][out] */ __RPC__deref_out_opt IUnknown **retval) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE Refresh( void) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Item( 
            /* [in] */ VARIANT varIndex,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrRegistryKey) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE AddItem( 
            /* [in] */ __RPC__in BSTR bstrRegistryKey) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE RemoveItem( 
            /* [in] */ VARIANT varIndex) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ISClusRegistryKeysVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in ISClusRegistryKeys * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in ISClusRegistryKeys * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in ISClusRegistryKeys * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in ISClusRegistryKeys * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in ISClusRegistryKeys * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in ISClusRegistryKeys * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            ISClusRegistryKeys * This,
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
        
        DECLSPEC_XFGVIRT(ISClusRegistryKeys, get_Count)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Count )( 
            __RPC__in ISClusRegistryKeys * This,
            /* [retval][out] */ __RPC__out long *plCount);
        
        DECLSPEC_XFGVIRT(ISClusRegistryKeys, get__NewEnum)
        /* [helpstring][id][restricted][propget] */ HRESULT ( STDMETHODCALLTYPE *get__NewEnum )( 
            __RPC__in ISClusRegistryKeys * This,
            /* [retval][out] */ __RPC__deref_out_opt IUnknown **retval);
        
        DECLSPEC_XFGVIRT(ISClusRegistryKeys, Refresh)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *Refresh )( 
            __RPC__in ISClusRegistryKeys * This);
        
        DECLSPEC_XFGVIRT(ISClusRegistryKeys, get_Item)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Item )( 
            __RPC__in ISClusRegistryKeys * This,
            /* [in] */ VARIANT varIndex,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrRegistryKey);
        
        DECLSPEC_XFGVIRT(ISClusRegistryKeys, AddItem)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *AddItem )( 
            __RPC__in ISClusRegistryKeys * This,
            /* [in] */ __RPC__in BSTR bstrRegistryKey);
        
        DECLSPEC_XFGVIRT(ISClusRegistryKeys, RemoveItem)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *RemoveItem )( 
            __RPC__in ISClusRegistryKeys * This,
            /* [in] */ VARIANT varIndex);
        
        END_INTERFACE
    } ISClusRegistryKeysVtbl;

    interface ISClusRegistryKeys
    {
        CONST_VTBL struct ISClusRegistryKeysVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ISClusRegistryKeys_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ISClusRegistryKeys_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ISClusRegistryKeys_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ISClusRegistryKeys_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define ISClusRegistryKeys_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define ISClusRegistryKeys_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define ISClusRegistryKeys_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define ISClusRegistryKeys_get_Count(This,plCount)	\
    ( (This)->lpVtbl -> get_Count(This,plCount) ) 

#define ISClusRegistryKeys_get__NewEnum(This,retval)	\
    ( (This)->lpVtbl -> get__NewEnum(This,retval) ) 

#define ISClusRegistryKeys_Refresh(This)	\
    ( (This)->lpVtbl -> Refresh(This) ) 

#define ISClusRegistryKeys_get_Item(This,varIndex,pbstrRegistryKey)	\
    ( (This)->lpVtbl -> get_Item(This,varIndex,pbstrRegistryKey) ) 

#define ISClusRegistryKeys_AddItem(This,bstrRegistryKey)	\
    ( (This)->lpVtbl -> AddItem(This,bstrRegistryKey) ) 

#define ISClusRegistryKeys_RemoveItem(This,varIndex)	\
    ( (This)->lpVtbl -> RemoveItem(This,varIndex) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ISClusRegistryKeys_INTERFACE_DEFINED__ */


#ifndef __ISClusCryptoKeys_INTERFACE_DEFINED__
#define __ISClusCryptoKeys_INTERFACE_DEFINED__

/* interface ISClusCryptoKeys */
/* [unique][helpstring][dual][uuid][object][nonextensible][hidden][oleautomation] */ 


EXTERN_C const IID IID_ISClusCryptoKeys;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("f2e6072c-2631-11d1-89f1-00a0c90d061e")
    ISClusCryptoKeys : public IDispatch
    {
    public:
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Count( 
            /* [retval][out] */ __RPC__out long *plCount) = 0;
        
        virtual /* [helpstring][id][restricted][propget] */ HRESULT STDMETHODCALLTYPE get__NewEnum( 
            /* [retval][out] */ __RPC__deref_out_opt IUnknown **retval) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE Refresh( void) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Item( 
            /* [in] */ VARIANT varIndex,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrCyrptoKey) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE AddItem( 
            /* [in] */ __RPC__in BSTR bstrCryptoKey) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE RemoveItem( 
            /* [in] */ VARIANT varIndex) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ISClusCryptoKeysVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in ISClusCryptoKeys * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in ISClusCryptoKeys * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in ISClusCryptoKeys * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in ISClusCryptoKeys * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in ISClusCryptoKeys * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in ISClusCryptoKeys * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            ISClusCryptoKeys * This,
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
        
        DECLSPEC_XFGVIRT(ISClusCryptoKeys, get_Count)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Count )( 
            __RPC__in ISClusCryptoKeys * This,
            /* [retval][out] */ __RPC__out long *plCount);
        
        DECLSPEC_XFGVIRT(ISClusCryptoKeys, get__NewEnum)
        /* [helpstring][id][restricted][propget] */ HRESULT ( STDMETHODCALLTYPE *get__NewEnum )( 
            __RPC__in ISClusCryptoKeys * This,
            /* [retval][out] */ __RPC__deref_out_opt IUnknown **retval);
        
        DECLSPEC_XFGVIRT(ISClusCryptoKeys, Refresh)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *Refresh )( 
            __RPC__in ISClusCryptoKeys * This);
        
        DECLSPEC_XFGVIRT(ISClusCryptoKeys, get_Item)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Item )( 
            __RPC__in ISClusCryptoKeys * This,
            /* [in] */ VARIANT varIndex,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrCyrptoKey);
        
        DECLSPEC_XFGVIRT(ISClusCryptoKeys, AddItem)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *AddItem )( 
            __RPC__in ISClusCryptoKeys * This,
            /* [in] */ __RPC__in BSTR bstrCryptoKey);
        
        DECLSPEC_XFGVIRT(ISClusCryptoKeys, RemoveItem)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *RemoveItem )( 
            __RPC__in ISClusCryptoKeys * This,
            /* [in] */ VARIANT varIndex);
        
        END_INTERFACE
    } ISClusCryptoKeysVtbl;

    interface ISClusCryptoKeys
    {
        CONST_VTBL struct ISClusCryptoKeysVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ISClusCryptoKeys_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ISClusCryptoKeys_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ISClusCryptoKeys_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ISClusCryptoKeys_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define ISClusCryptoKeys_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define ISClusCryptoKeys_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define ISClusCryptoKeys_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define ISClusCryptoKeys_get_Count(This,plCount)	\
    ( (This)->lpVtbl -> get_Count(This,plCount) ) 

#define ISClusCryptoKeys_get__NewEnum(This,retval)	\
    ( (This)->lpVtbl -> get__NewEnum(This,retval) ) 

#define ISClusCryptoKeys_Refresh(This)	\
    ( (This)->lpVtbl -> Refresh(This) ) 

#define ISClusCryptoKeys_get_Item(This,varIndex,pbstrCyrptoKey)	\
    ( (This)->lpVtbl -> get_Item(This,varIndex,pbstrCyrptoKey) ) 

#define ISClusCryptoKeys_AddItem(This,bstrCryptoKey)	\
    ( (This)->lpVtbl -> AddItem(This,bstrCryptoKey) ) 

#define ISClusCryptoKeys_RemoveItem(This,varIndex)	\
    ( (This)->lpVtbl -> RemoveItem(This,varIndex) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ISClusCryptoKeys_INTERFACE_DEFINED__ */


#ifndef __ISClusResDependents_INTERFACE_DEFINED__
#define __ISClusResDependents_INTERFACE_DEFINED__

/* interface ISClusResDependents */
/* [unique][helpstring][dual][uuid][object][nonextensible][hidden][oleautomation] */ 


EXTERN_C const IID IID_ISClusResDependents;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("f2e6072e-2631-11d1-89f1-00a0c90d061e")
    ISClusResDependents : public IDispatch
    {
    public:
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Count( 
            /* [retval][out] */ __RPC__out long *plCount) = 0;
        
        virtual /* [helpstring][id][restricted][propget] */ HRESULT STDMETHODCALLTYPE get__NewEnum( 
            /* [retval][out] */ __RPC__deref_out_opt IUnknown **retval) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE Refresh( void) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Item( 
            /* [in] */ VARIANT varIndex,
            /* [retval][out] */ __RPC__deref_out_opt ISClusResource **ppClusResource) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE CreateItem( 
            /* [in] */ __RPC__in BSTR bstrResourceName,
            /* [in] */ __RPC__in BSTR bstrResourceType,
            /* [in] */ CLUSTER_RESOURCE_CREATE_FLAGS dwFlags,
            /* [retval][out] */ __RPC__deref_out_opt ISClusResource **ppClusterResource) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE DeleteItem( 
            /* [in] */ VARIANT varIndex) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE AddItem( 
            /* [in] */ __RPC__in_opt ISClusResource *pResource) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE RemoveItem( 
            /* [in] */ VARIANT varIndex) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ISClusResDependentsVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in ISClusResDependents * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in ISClusResDependents * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in ISClusResDependents * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in ISClusResDependents * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in ISClusResDependents * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in ISClusResDependents * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            ISClusResDependents * This,
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
        
        DECLSPEC_XFGVIRT(ISClusResDependents, get_Count)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Count )( 
            __RPC__in ISClusResDependents * This,
            /* [retval][out] */ __RPC__out long *plCount);
        
        DECLSPEC_XFGVIRT(ISClusResDependents, get__NewEnum)
        /* [helpstring][id][restricted][propget] */ HRESULT ( STDMETHODCALLTYPE *get__NewEnum )( 
            __RPC__in ISClusResDependents * This,
            /* [retval][out] */ __RPC__deref_out_opt IUnknown **retval);
        
        DECLSPEC_XFGVIRT(ISClusResDependents, Refresh)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *Refresh )( 
            __RPC__in ISClusResDependents * This);
        
        DECLSPEC_XFGVIRT(ISClusResDependents, get_Item)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Item )( 
            __RPC__in ISClusResDependents * This,
            /* [in] */ VARIANT varIndex,
            /* [retval][out] */ __RPC__deref_out_opt ISClusResource **ppClusResource);
        
        DECLSPEC_XFGVIRT(ISClusResDependents, CreateItem)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *CreateItem )( 
            __RPC__in ISClusResDependents * This,
            /* [in] */ __RPC__in BSTR bstrResourceName,
            /* [in] */ __RPC__in BSTR bstrResourceType,
            /* [in] */ CLUSTER_RESOURCE_CREATE_FLAGS dwFlags,
            /* [retval][out] */ __RPC__deref_out_opt ISClusResource **ppClusterResource);
        
        DECLSPEC_XFGVIRT(ISClusResDependents, DeleteItem)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *DeleteItem )( 
            __RPC__in ISClusResDependents * This,
            /* [in] */ VARIANT varIndex);
        
        DECLSPEC_XFGVIRT(ISClusResDependents, AddItem)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *AddItem )( 
            __RPC__in ISClusResDependents * This,
            /* [in] */ __RPC__in_opt ISClusResource *pResource);
        
        DECLSPEC_XFGVIRT(ISClusResDependents, RemoveItem)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *RemoveItem )( 
            __RPC__in ISClusResDependents * This,
            /* [in] */ VARIANT varIndex);
        
        END_INTERFACE
    } ISClusResDependentsVtbl;

    interface ISClusResDependents
    {
        CONST_VTBL struct ISClusResDependentsVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ISClusResDependents_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ISClusResDependents_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ISClusResDependents_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ISClusResDependents_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define ISClusResDependents_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define ISClusResDependents_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define ISClusResDependents_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define ISClusResDependents_get_Count(This,plCount)	\
    ( (This)->lpVtbl -> get_Count(This,plCount) ) 

#define ISClusResDependents_get__NewEnum(This,retval)	\
    ( (This)->lpVtbl -> get__NewEnum(This,retval) ) 

#define ISClusResDependents_Refresh(This)	\
    ( (This)->lpVtbl -> Refresh(This) ) 

#define ISClusResDependents_get_Item(This,varIndex,ppClusResource)	\
    ( (This)->lpVtbl -> get_Item(This,varIndex,ppClusResource) ) 

#define ISClusResDependents_CreateItem(This,bstrResourceName,bstrResourceType,dwFlags,ppClusterResource)	\
    ( (This)->lpVtbl -> CreateItem(This,bstrResourceName,bstrResourceType,dwFlags,ppClusterResource) ) 

#define ISClusResDependents_DeleteItem(This,varIndex)	\
    ( (This)->lpVtbl -> DeleteItem(This,varIndex) ) 

#define ISClusResDependents_AddItem(This,pResource)	\
    ( (This)->lpVtbl -> AddItem(This,pResource) ) 

#define ISClusResDependents_RemoveItem(This,varIndex)	\
    ( (This)->lpVtbl -> RemoveItem(This,varIndex) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ISClusResDependents_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_msclus_0000_0041 */
/* [local] */ 

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion


extern RPC_IF_HANDLE __MIDL_itf_msclus_0000_0041_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_msclus_0000_0041_v0_0_s_ifspec;

/* Additional Prototypes for ALL interfaces */

unsigned long             __RPC_USER  BSTR_UserSize(     __RPC__in unsigned long *, unsigned long            , __RPC__in BSTR * ); 
unsigned char * __RPC_USER  BSTR_UserMarshal(  __RPC__in unsigned long *, __RPC__inout_xcount(0) unsigned char *, __RPC__in BSTR * ); 
unsigned char * __RPC_USER  BSTR_UserUnmarshal(__RPC__in unsigned long *, __RPC__in_xcount(0) unsigned char *, __RPC__out BSTR * ); 
void                      __RPC_USER  BSTR_UserFree(     __RPC__in unsigned long *, __RPC__in BSTR * ); 

unsigned long             __RPC_USER  VARIANT_UserSize(     __RPC__in unsigned long *, unsigned long            , __RPC__in VARIANT * ); 
unsigned char * __RPC_USER  VARIANT_UserMarshal(  __RPC__in unsigned long *, __RPC__inout_xcount(0) unsigned char *, __RPC__in VARIANT * ); 
unsigned char * __RPC_USER  VARIANT_UserUnmarshal(__RPC__in unsigned long *, __RPC__in_xcount(0) unsigned char *, __RPC__out VARIANT * ); 
void                      __RPC_USER  VARIANT_UserFree(     __RPC__in unsigned long *, __RPC__in VARIANT * ); 

unsigned long             __RPC_USER  BSTR_UserSize64(     __RPC__in unsigned long *, unsigned long            , __RPC__in BSTR * ); 
unsigned char * __RPC_USER  BSTR_UserMarshal64(  __RPC__in unsigned long *, __RPC__inout_xcount(0) unsigned char *, __RPC__in BSTR * ); 
unsigned char * __RPC_USER  BSTR_UserUnmarshal64(__RPC__in unsigned long *, __RPC__in_xcount(0) unsigned char *, __RPC__out BSTR * ); 
void                      __RPC_USER  BSTR_UserFree64(     __RPC__in unsigned long *, __RPC__in BSTR * ); 

unsigned long             __RPC_USER  VARIANT_UserSize64(     __RPC__in unsigned long *, unsigned long            , __RPC__in VARIANT * ); 
unsigned char * __RPC_USER  VARIANT_UserMarshal64(  __RPC__in unsigned long *, __RPC__inout_xcount(0) unsigned char *, __RPC__in VARIANT * ); 
unsigned char * __RPC_USER  VARIANT_UserUnmarshal64(__RPC__in unsigned long *, __RPC__in_xcount(0) unsigned char *, __RPC__out VARIANT * ); 
void                      __RPC_USER  VARIANT_UserFree64(     __RPC__in unsigned long *, __RPC__in VARIANT * ); 

/* end of Additional Prototypes */

#ifdef __cplusplus
}
#endif

#endif


