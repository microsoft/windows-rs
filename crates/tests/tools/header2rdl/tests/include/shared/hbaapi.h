//***************************************************************************
//
//  hbaapi.h
// 
//  Module: Windows HBA API implmentation
//
//      This header is consistent with www.t11.org SM-HBA Draft July 2006
//
//  Purpose: Contains HBA api header
//
//  Copyright (c) Microsoft Corporation
//
//***************************************************************************

#ifndef HBAAPI_H
#define HBAAPI_H

#if (NTDDI_VERSION >= NTDDI_VISTA)

#include <winapifamily.h>

#pragma region Desktop Family or OneCore Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM)

#ifdef __cplusplus
extern "C" {
#endif

#define SM_HBA_API   // include SM-HBA specific functions

#include <time.h>

#ifdef _HBAAPIP_
#define HBA_API __cdecl
#else
#define HBA_API DECLSPEC_IMPORT __cdecl
#endif

#define HBA_VERSION   2

typedef ULONGLONG   HBA_UINT64;
typedef LONGLONG    HBA_INT64;
typedef ULONG       HBA_UINT32;
typedef USHORT      HBA_UINT16;
typedef UCHAR       HBA_UINT8;
typedef signed char HBA_INT8;

typedef HBA_UINT32 HBA_HANDLE;

typedef HBA_UINT32 HBA_STATUS;

#define HBA_STATUS_OK                        0

#define HBA_STATUS_ERROR                     1   /* Error */
#define HBA_STATUS_ERROR_NOT_SUPPORTED       2   /* Function not supported.*/
#define HBA_STATUS_ERROR_INVALID_HANDLE      3   /* invalid handle */
#define HBA_STATUS_ERROR_ARG                 4   /* Bad argument */
#define HBA_STATUS_ERROR_ILLEGAL_WWN         5   /* WWN not recognized */
#define HBA_STATUS_ERROR_ILLEGAL_INDEX       6   /* Index not recognized */
#define HBA_STATUS_ERROR_MORE_DATA           7   /* Larger buffer required */

/* Information has changed since the last call to HBA_RefreshInformation */
#define HBA_STATUS_ERROR_STALE_DATA          8  

/* SCSI Check Condition reported*/
#define HBA_STATUS_SCSI_CHECK_CONDITION      9

/* Adapter busy or reserved, retry may be effective*/
#define HBA_STATUS_ERROR_BUSY               10 

/* Request timed out, retry may be effective */
#define HBA_STATUS_ERROR_TRY_AGAIN          11 

/* Referenced HBA has been removed or deactivated */
#define HBA_STATUS_ERROR_UNAVAILABLE        12   

/* The requested ELS was rejected  by the local adapter */
#define HBA_STATUS_ERROR_ELS_REJECT         13   

/* The specified LUN is not provided  by the specified adapter */
#define HBA_STATUS_ERROR_INVALID_LUN        14   

/* An incompatibility has been detected among the library and driver modules */
/* invoked which will cause one or more functions in the highest version */ 
/* that all support to operate incorrectly.  */
/* The differing function sets of software modules implementing different */
/* versions of the HBA API specification does not in itself constitute an */
/* incompatibility. */
/* Known interoperability bugs among supposedly compatible versions */
/* should be reported as incompatibilities, */
/* but not all such interoperability bugs may be known. */
/* This value may be returned by any function which calls a */
/* Vendor Specific Library,  and by HBA_LoadLibrary and HBA_GetAdapterName. */

#define HBA_STATUS_ERROR_INCOMPATIBLE       15   


/* Multiple adapters have a matching WWN. */
/* This could occur if the NodeWWN of multiple adapters is identical. */
#define HBA_STATUS_ERROR_AMBIGUOUS_WWN      16   

/* A persistent binding request included a bad local SCSI bus number */
#define HBA_STATUS_ERROR_LOCAL_BUS          17   

/* A persistent binding request included a bad local SCSI target number */
#define HBA_STATUS_ERROR_LOCAL_TARGET       18   

/* A persistent binding request included a bad local SCSI logical unit number */
#define HBA_STATUS_ERROR_LOCAL_LUN          19   

/* A persistent binding set request included */
/* a local SCSI ID that was already bound */
#define HBA_STATUS_ERROR_LOCAL_SCSIID_BOUND 20   

/* A persistent binding request included a bad or unlocatable FCP Target FCID */
#define HBA_STATUS_ERROR_TARGET_FCID        21   

/* A persistent binding request included a bad FCP Target Node WWN */
#define HBA_STATUS_ERROR_TARGET_NODE_WWN    22   

/* A persistent binding request included a bad FCP Target Port WWN */
#define HBA_STATUS_ERROR_TARGET_PORT_WWN    23   

/* A persistent binding request included */
/* an FCP Logical Unit Number not defined by the identified Target*/
#define HBA_STATUS_ERROR_TARGET_LUN         24   

/* A persistent binding request included */
/* an undefined or otherwise inaccessible Logical Unit Unique Identifier */
#define HBA_STATUS_ERROR_TARGET_LUID        25   

/* A persistent binding remove request included */
/* a binding which did not match a binding established by the specified port */
#define HBA_STATUS_ERROR_NO_SUCH_BINDING    26 

/* A SCSI command was requested to an Nx_Port that was not a SCSI Target Port */
#define HBA_STATUS_ERROR_NOT_A_TARGET       27   

/* A request was made concerning an unsupported FC-4 protocol */
#define HBA_STATUS_ERROR_UNSUPPORTED_FC4    28   

/* A request was made to enable unimplemented capabilities for a port */
#define HBA_STATUS_ERROR_INCAPABLE          29 

/* A SCSI function was rejected to prevent causing */
/* a SCSI overlapped command condition (see SAM-3) */
#define HBA_STATUS_ERROR_TARGET_BUSY        30  

/* A call was made to HBA_FreeLibrary when no library was loaded */
#define HBA_STATUS_ERROR_NOT_LOADED         31   

/* A call was made to HBA_LoadLibrary when a library was already loaded */
#define HBA_STATUS_ERROR_ALREADY_LOADED     32

/* The Address Identifier specified in a call to HBA_SendRNIDV2 */
/* violates access control rules for that call */
#define HBA_STATUS_ERROR_ILLEGAL_FCID       33

#define HBA_STATUS_ERROR_NOT_ASCSIDEVICE    34

#define HBA_STATUS_ERROR_INVALID_PROTOCOL_TYPE 35

#define HBA_STATUS_ERROR_BAD_EVENT_TYPE     36


typedef HBA_UINT8  HBA_BOOLEAN;     /* Use this for a single true/false flag */

typedef HBA_UINT32 HBA_PORTTYPE;

#define HBA_PORTTYPE_UNKNOWN      1 /* Unknown */
#define HBA_PORTTYPE_OTHER        2 /* Other */
#define HBA_PORTTYPE_NOTPRESENT   3 /* Not present */
#define HBA_PORTTYPE_NPORT        5 /* Fabric */
#define HBA_PORTTYPE_NLPORT       6 /* Public Loop */
#define HBA_PORTTYPE_FLPORT       7 /* Fabric on a Loop */
#define HBA_PORTTYPE_FPORT        8 /* Fabric Port */
#define HBA_PORTTYPE_EPORT        9 /* Fabric expansion port */ // obsolete?
#define HBA_PORTTYPE_GPORT       10 /* Generic Fabric Port */   // obsolete?
#define HBA_PORTTYPE_LPORT       20 /* Private Loop */
#define HBA_PORTTYPE_PTP         21 /* Point to Point */
#define HBA_PORTTYPE_SASDEVICE   30 /* SAS (SSP or STP) */
#define HBA_PORTTYPE_SATADEVICE  31 /* SATA Device, i.e. Direct Attach SATA */
#define HBA_PORTTYPE_SASEXPANDER 32 /* SAS Expander */

typedef HBA_UINT32 HBA_PORTSTATE;
#define HBA_PORTSTATE_UNKNOWN     1 /* Unknown */
#define HBA_PORTSTATE_ONLINE      2 /* Operational */
#define HBA_PORTSTATE_OFFLINE     3 /* User Offline */
#define HBA_PORTSTATE_BYPASSED    4 /* Bypassed */
#define HBA_PORTSTATE_DIAGNOSTICS 5 /* In diagnostics mode */
#define HBA_PORTSTATE_LINKDOWN    6 /* Link Down */
#define HBA_PORTSTATE_ERROR       7 /* Port Error */
#define HBA_PORTSTATE_LOOPBACK    8 /* Loopback */
#define HBA_PORTSTATE_DEGRADED    9 /* Degraded, but Operational mode */

typedef HBA_UINT32 HBA_PORTSPEED;
typedef HBA_UINT32 HBA_FCPHYSPEED, HBA_SASPHYSPEED, HBA_PHYSPEED;

#define HBA_PORTSPEED_UNKNOWN     0    /* Unknown - transceiver incapable */
                                       /* of reporting */
#define HBA_PORTSPEED_1GBIT       1    /* 1 GBit/sec */
#define HBA_PORTSPEED_2GBIT       2    /* 2 GBit/sec */
#define HBA_PORTSPEED_10GBIT      4    /* 10 GBit/sec */
#define HBA_PORTSPEED_4GBIT       8    /* 4 GBit/sec */


#define HBA_FCSPEED_UNKNOWN       0    /* Unknown - transceiver incapable */
                                       /* of reporting */
#define HBA_FCPHYSPEED_8GBIT     16    /* 8 GBit/sec */
#define HBA_FCPHYSPEED_16GBIT    32    /* 16 GBit/sec */

#define HBA_PORTSPEED_NOT_NEGOTIATED (1 << 15) /* Speed not established */


typedef HBA_UINT8 HBA_FCPHYTYPE;

#define HBA_FCPHYTYPE_UNKNOWN     1    /* Unknown Phy type */
#define HBA_FCPHYTYPE_OPTICAL     2    /* Optical Phy */
#define HBA_FCPHYTYPE_COPPER      4    /* Copper Phy */


#define HBA_SASSTATE_UNKNOWN     0x00  /* Phy is enabled. Speed is unknown   */
#define HBA_SASSTATE_DISABLED    0x01  /* Phy is disabled. */

#define HBA_SASSTATE_FAILED      0x02  /* Phy is enabled. But failed speed   */
                                       /* negotiation. */

#define HBA_SASSTATE_SATASPINUP  0x03  /* Phy is enabled. Detected a SATA    */
                                       /* device and entered the SATA Spinup */
                                       /* hold state */

#define HBA_SASSTATE_SATAPORTSEL 0x04  /* The phy is attached to a */
                                       /* Port Selector (see SATA-2.5). */

#define HBA_SASSPEED_1_5GBIT     0x08  /* 1.5 GBit/sec */
#define HBA_SASSPEED_3GBIT       0x09  /* 3 GBit/sec   */


typedef struct HBA_wwn {
    HBA_UINT8 wwn[8];
} HBA_WWN, *PHBA_WWN;


typedef struct SMHBA_FC_Phy {
    HBA_FCPHYSPEED  PhySupportSpeed;   /* PhySupportedSpeed */
    HBA_FCPHYSPEED  PhySpeed;          /* PhySpeed */
    HBA_FCPHYTYPE   PhyType;          
    HBA_UINT32      MaxFrameSize;      /* MaxFrameSize */
} SMHBA_FC_PHY, *PSMHBA_FC_PHY;

typedef struct SMHBA_SAS_Phy {
    HBA_UINT8        PhyIdentifier;
    HBA_SASPHYSPEED  NegotiatedLinkRate;
    HBA_SASPHYSPEED  ProgrammedMinLinkRate;
    HBA_SASPHYSPEED  HardwareMinLinkRate;
    HBA_SASPHYSPEED  ProgrammedMaxLinkRate;
    HBA_SASPHYSPEED  HardwareMaxLinkRate;
    HBA_WWN          domainPortWWN;
} SMHBA_SAS_PHY, *PSMHBA_SAS_PHY;



typedef HBA_UINT32 HBA_COS;

typedef struct HBA_fc4types {
    HBA_UINT8 bits[32];     /* See FC-4 TYPEs - Format in FC-GS-4 */
} HBA_FC4TYPES, *PHBA_FC4TYPES;

typedef struct HBA_ipaddress {
    int ipversion; // see enumerations in RNID
    union
    {
        unsigned char ipv4address[4];
        unsigned char ipv6address[16];
    } ipaddress;
} HBA_IPADDRESS, *PHBA_IPADDRESS;

typedef struct HBA_AdapterAttributes {
    char       Manufacturer[64];      
    char       SerialNumber[64];     
    char       Model[256];          
    char       ModelDescription[256]; 
    HBA_WWN    NodeWWN;
    char       NodeSymbolicName[256]; /* From GS-2 */
    char       HardwareVersion[256];  /* Vendor use */
    char       DriverVersion[256];    /* Vendor use */
    char       OptionROMVersion[256]; /* Vendor use - i.e. hardware boot ROM*/
    char       FirmwareVersion[256];  /* Vendor use */
    HBA_UINT32 VendorSpecificID;      /* Vendor specific */
    HBA_UINT32 NumberOfPorts;
    char       DriverName[256];       /* Binary path and/or name of driver file. */
} HBA_ADAPTERATTRIBUTES, *PHBA_ADAPTERATTRIBUTES;


typedef struct SMHBA_AdapterAttributes {
    char       Manufacturer[64];      
    char       SerialNumber[64];      
    char       Model[256];            
    char       ModelDescription[256]; 
    char       HardwareVersion[256];  
    char       DriverVersion[256];    
    char       OptionROMVersion[256]; 
    char       FirmwareVersion[256];  
    HBA_UINT32 VendorSpecificID;      
    char       DriverName[256];      
    char       HBASymbolicName[256];
    char       RedundantOptionROMVersion[256]; 
    char       RedundantFirmwareVersion[256];  
} SMHBA_ADAPTERATTRIBUTES, *PSMHBA_ADAPTERATTRIBUTES;


typedef struct HBA_PortAttributes {
    HBA_WWN       NodeWWN;
    HBA_WWN       PortWWN;
    HBA_UINT32    PortFcId;
    HBA_PORTTYPE  PortType;            /*PTP, Fabric, etc. */
    HBA_PORTSTATE PortState;
    HBA_COS       PortSupportedClassofService;
    HBA_FC4TYPES  PortSupportedFc4Types;
    HBA_FC4TYPES  PortActiveFc4Types;
    char          PortSymbolicName[256];
    char          OSDeviceName[256];   
    HBA_PORTSPEED PortSupportedSpeed;
    HBA_PORTSPEED PortSpeed;
    HBA_UINT32    PortMaxFrameSize;
    HBA_WWN       FabricName;
    HBA_UINT32    NumberofDiscoveredPorts;
} HBA_PORTATTRIBUTES, *PHBA_PORTATTRIBUTES;


typedef HBA_UINT32 HBA_SASPORTPROTOCOL;

#define HBA_SASPORTPROTOCOL_SSP  1  /* Serial Attached SCSI Port */
#define HBA_SASPORTPROTOCOL_STP  2  /* Serial ATA Tunneling Protocol Port */
#define HBA_SASPORTPROTOCOL_SMP  4  /* Serial Management Protocol Port */
#define HBA_SASPORTPROTOCOL_SATA 8  /* SATA Device, Direct Attached or */
                                    /* anywhere in the domain */


typedef struct SMHBA_FC_Port {
    HBA_WWN       NodeWWN;
    HBA_WWN       PortWWN;
    HBA_UINT32    FcId;
    HBA_COS       PortSupportedClassofService;
    HBA_FC4TYPES  PortSupportedFc4Types;
    HBA_FC4TYPES  PortActiveFc4Types;
    HBA_WWN       FabricName;
    char          PortSymbolicName[256];
    HBA_UINT32    NumberofDiscoveredPorts;
    HBA_UINT8     NumberofPhys;
} SMHBA_FC_PORT, *PSMHBA_FC_PORT;

typedef struct SMHBA_SAS_Port {
    HBA_SASPORTPROTOCOL  PortProtocol;
    HBA_WWN              LocalSASAddress;
    HBA_WWN              AttachedSASAddress;
    HBA_UINT32           NumberofDiscoveredPorts;
    HBA_UINT32           NumberofPhys;
} SMHBA_SAS_PORT, *PSMHBA_SAS_PORT;

typedef union SMHBA_Port {
    SMHBA_FC_PORT  *FCPort;
    SMHBA_SAS_PORT *SASPort;
} SMHBA_PORT, *PSMHBA_PORT;

typedef struct SMHBA_PortAttributes {
    HBA_PORTTYPE  PortType;  
    HBA_PORTSTATE PortState;
    char          OSDeviceName[256];   
    SMHBA_PORT    PortSpecificAttribute;
} SMHBA_PORTATTRIBUTES, *PSMHBA_PORTATTRIBUTES;


/* Statistical counters for FC-0, FC-1 and FC-2 */

typedef struct HBA_PortStatistics {
    HBA_INT64 SecondsSinceLastReset;
    HBA_INT64 TxFrames;
    HBA_INT64 TxWords;
    HBA_INT64 RxFrames;
    HBA_INT64 RxWords;
    HBA_INT64 LIPCount;
    HBA_INT64 NOSCount;
    HBA_INT64 ErrorFrames;
    HBA_INT64 DumpedFrames;
    HBA_INT64 LinkFailureCount;
    HBA_INT64 LossOfSyncCount;
    HBA_INT64 LossOfSignalCount;
    HBA_INT64 PrimitiveSeqProtocolErrCount;
    HBA_INT64 InvalidTxWordCount;
    HBA_INT64 InvalidCRCCount;
} HBA_PORTSTATISTICS, *PHBA_PORTSTATISTICS;


/* Statistical counters for FC-4 protocols */

typedef struct HBA_FC4Statistics {
    HBA_INT64 InputRequests;
    HBA_INT64 OutputRequests;
    HBA_INT64 ControlRequests;
    HBA_INT64 InputMegabytes;
    HBA_INT64 OutputMegabytes;
} HBA_FC4STATISTICS, *PHBA_FC4STATISTICS;


/* Statistical counters for FC-4, SSP, STP, SMP protocols */

typedef struct SMHBA_ProtocolStatistics {
    HBA_INT64 SecondsSinceLastReset;
    HBA_INT64 InputRequests;
    HBA_INT64 OutputRequests;
    HBA_INT64 ControlRequests;
    HBA_INT64 InputMegabytes;
    HBA_INT64 OutputMegabytes;
} SMHBA_PROTOCOLSTATISTICS, *PSMHBA_PROTOCOLSTATISTICS;


typedef struct SMHBA_SASPhyStatistics {
    HBA_INT64 SecondsSinceLastReset;
    HBA_INT64 TxFrames;
    HBA_INT64 TxWords;
    HBA_INT64 RxFrames;
    HBA_INT64 RxWords;
    HBA_INT64 InvalidDwordCount;
    HBA_INT64 RunningDisparityErrorCount;
    HBA_INT64 LossofDwordSyncCount;
    HBA_INT64 PhyResetProblemCount;
} SMHBA_SASPHYSTATISTICS, *PSMHBA_SASPHYSTATISTICS;


/* Statistical counters for FC-0, FC-1, and FC-2 */

typedef HBA_PORTSTATISTICS SMHBA_FCPHYSTATISTICS, *PSMHBA_FCPHYSTATISTICS;

typedef union SMHBA_PhyStatistics {
    SMHBA_SASPHYSTATISTICS *SASPhyStatistics;
    SMHBA_FCPHYSTATISTICS  *FCPhyStatistics;
} SMHBA_PHYSTATISTICS, *PSMHBA_PHYSTATISTICS;



/* HBA_FCPBINDINGTYPE was used in Rev 1.0.  Add TO_OTHER for older calls to 
   indicate other binding types for HBA_GetPersistentBinding.  To support 
   multiple types a new flag has been created to allow for multiple bindings
   supported */

typedef enum HBA_fcpbindingtype { TO_D_ID, TO_WWN, TO_OTHER } HBA_FCPBINDINGTYPE;


/* A bit mask of Rev 2.0 persistent binding capabilities */

typedef HBA_UINT32 HBA_BIND_CAPABILITY;   

/* The following are bit flags indicating persistent binding capabilities */

#define HBA_CAN_BIND_TO_D_ID     0x0001
#define HBA_CAN_BIND_TO_WWPN     0x0002
#define HBA_CAN_BIND_TO_WWNN     0x0004
#define HBA_CAN_BIND_TO_LUID     0x0008
#define HBA_CAN_BIND_ANY_LUNS    0x0400
#define HBA_CAN_BIND_TARGETS     0x0800
#define HBA_CAN_BIND_AUTOMAP     0x1000
#define HBA_CAN_BIND_CONFIGURED  0x2000

/* A bit mask of Rev 2.0 persistent binding setting types */

typedef HBA_UINT32 HBA_BIND_TYPE;

/* The following are bit flags indicating persistent binding setting types */

#define HBA_BIND_TO_D_ID     0x0001
#define HBA_BIND_TO_WWPN     0x0002
#define HBA_BIND_TO_WWNN     0x0004
#define HBA_BIND_TO_LUID     0x0008
#define HBA_BIND_TARGETS     0x0800

typedef struct HBA_LUID {
    char  buffer[256];
} HBA_LUID, *PHBA_LUID;

typedef struct HBA_ScsiId {
    char       OSDeviceName[256]; /* \device\ScsiPort3 */
    HBA_UINT32 ScsiBusNumber;     /* Bus on the HBA */
    HBA_UINT32 ScsiTargetNumber;  /* SCSI Target ID to OS */
    HBA_UINT32 ScsiOSLun;
} HBA_SCSIID, *PHBA_SCSIID;

typedef struct HBA_FcpId {
    HBA_UINT32 FcId;
    HBA_WWN    NodeWWN;
    HBA_WWN    PortWWN;
    HBA_UINT64 FcpLun;
} HBA_FCPID, *PHBA_FCPID;

typedef struct HBA_FcpScsiEntry {
    HBA_SCSIID ScsiId;
    HBA_FCPID  FcpId;
} HBA_FCPSCSIENTRY, *PHBA_FCPSCSIENTRY;

typedef struct HBA_FcpScsiEntryV2 {
    HBA_SCSIID ScsiId;
    HBA_FCPID  FcpId;
    HBA_LUID   LUID;
} HBA_FCPSCSIENTRYV2, *PHBA_FCPSCSIENTRYV2;

typedef struct HBA_FCPTargetMapping {
    HBA_UINT32       NumberOfEntries;
    HBA_FCPSCSIENTRY entry[1];  /* Variable length array containing mappings*/
} HBA_FCPTARGETMAPPING, *PHBA_FCPTARGETMAPPING;

typedef struct HBA_FCPTargetMappingV2 {
    HBA_UINT32         NumberOfEntries;
    HBA_FCPSCSIENTRYV2 entry[1]; /* Variable length array containing mappings*/
} HBA_FCPTARGETMAPPINGV2, *PHBA_FCPTARGETMAPPINGV2;


typedef struct HBA_FCPBindingEntry {
    HBA_FCPBINDINGTYPE type;
    HBA_SCSIID         ScsiId;
    HBA_FCPID          FcpId;
    HBA_UINT32         FcId;
} HBA_FCPBINDINGENTRY, *PHBA_FCPBINDINGENTRY;

typedef struct HBA_FCPBinding {
    HBA_UINT32          NumberOfEntries;
    HBA_FCPBINDINGENTRY entry[1]; /* Variable length array */
} HBA_FCPBINDING, *PHBA_FCPBINDING;

typedef struct HBA_FCPBindingEntry2 {
    HBA_BIND_TYPE  type;
    HBA_SCSIID     ScsiId;
    HBA_FCPID      FcpId;
    HBA_LUID       LUID;
    HBA_STATUS     Status;
} HBA_FCPBINDINGENTRY2, *PHBA_FCPBINDINGENTRY2;

typedef struct HBA_FCPBinding2 {
    HBA_UINT32           NumberOfEntries;
    HBA_FCPBINDINGENTRY2 entry[1]; /* Variable length array */
} HBA_FCPBINDING2, *PHBA_FCPBINDING2;


typedef HBA_UINT32 SMHBA_BIND_CAPABILITY;

#define SMHBA_CAN_BIND_TO_WWPN  0x0001
#define SMHBA_CAN_BIND_TO_LUID  0x0002
#define SMHBA_CAN_BIND_ANY_LUNS 0x0400
#define SMHBA_CAN_BIND_AUTOMAP  0x0800

typedef HBA_UINT32 SMHBA_BIND_TYPE;

#define SMHBA_BIND_TO_WWPN 0x0001
#define SMHBA_BIND_TO_LUID 0x0002

typedef struct SMHBA_ScsiId {
    char       OSDeviceName[256];
    HBA_UINT32 ScsiBusNumber;
    HBA_UINT32 ScsiTargetNumber;
    HBA_UINT32 ScsiOSLun;
} SMHBA_SCSIID, *PSMHBA_SCSIID;

typedef struct SMHBA_LUID {
    char buffer[256];
} SMHBA_LUID, *PSMHBA_LUID;

typedef HBA_UINT64 HBA_SCSILUN;

typedef struct SMHBA_SCSILUN {
    HBA_UINT8 lun[8];
} SMHBA_SCSILUN;

typedef struct SMHBA_PORTLUN {
    HBA_WWN     PortWWN;
    HBA_WWN     domainPortWWN;
    HBA_SCSILUN TargetLun;
} SMHBA_PORTLUN, *PSMHBA_PORTLUN;

typedef struct SMHBA_ScsiEntry {
    SMHBA_SCSIID  ScsiId;
    SMHBA_PORTLUN PortLun;
    SMHBA_LUID    LUID;
} SMHBA_SCSIENTRY, *PSMHBA_SCSIENTRY;

typedef struct SMHBA_TargetMapping {
    HBA_UINT32      NumberOfEntries;
    SMHBA_SCSIENTRY entry[1]; /* Variable length array containing mappings*/
} SMHBA_TARGETMAPPING, *PSMHBA_TARGETMAPPING;

typedef struct SMHBA_BindingEntry {
    SMHBA_BIND_TYPE type;
    SMHBA_SCSIID    ScsiId;
    SMHBA_PORTLUN   PortLun;
    SMHBA_LUID      LUID;
    HBA_STATUS      Status;
} SMHBA_BINDINGENTRY, *PSMHBA_BINDINGENTRY;

typedef struct SMHBA_Binding {
    HBA_UINT32         NumberOfEntries;
    SMHBA_BINDINGENTRY entry[1]; /* Variable length array */
} SMHBA_BINDING, *PSMHBA_BINDING;


typedef enum HBA_wwntype { NODE_WWN, PORT_WWN } HBA_WWNTYPE;

typedef struct HBA_MgmtInfo {
    HBA_WWN    wwn;
    HBA_UINT32 unittype;
    HBA_UINT32 PortId;
    HBA_UINT32 NumberOfAttachedNodes;
    HBA_UINT16 IPVersion;
    HBA_UINT16 UDPPort;
    HBA_UINT8  IPAddress[16];
    HBA_UINT16 reserved;
    HBA_UINT16 TopologyDiscoveryFlags;
} HBA_MGMTINFO, *PHBA_MGMTINFO;

#define HBA_EVENT_LIP_OCCURRED       1
#define HBA_EVENT_LINK_UP            2
#define HBA_EVENT_LINK_DOWN          3
#define HBA_EVENT_LIP_RESET_OCCURRED 4
#define HBA_EVENT_RSCN               5
#define HBA_EVENT_PROPRIETARY        0xFFFF

typedef struct HBA_Link_EventInfo {
    HBA_UINT32 PortFcId;   /* Port which this event occurred */
    HBA_UINT32 Reserved[3];
} HBA_LINK_EVENTINFO, *PHBA_LINK_EVENTINFO;

typedef struct HBA_RSCN_EventInfo {
    HBA_UINT32 PortFcId;  /* Port which this event occurred */
    HBA_UINT32 NPortPage; /* Reference FC-FS for RSCN ELS "Affected N-Port Pages"*/
    HBA_UINT32 Reserved[2];
} HBA_RSCN_EVENTINFO, *PHBA_RSCN_EVENTINFO;

typedef struct HBA_Pty_EventInfo {
    HBA_UINT32 PtyData[4]; /* Proprietary data */
} HBA_PTY_EVENTINFO, *PHBA_PTY_EVENTINFO;

typedef struct HBA_EventInfo {
    HBA_UINT32 EventCode;
    union {
        HBA_LINK_EVENTINFO Link_EventInfo;
        HBA_RSCN_EVENTINFO RSCN_EventInfo;
        HBA_PTY_EVENTINFO  Pty_EventInfo;
    } Event;
} HBA_EVENTINFO, *PHBA_EVENTINFO;

typedef PVOID PHBA_ENTRYPOINTS;
typedef PVOID PHBA_ENTRYPOINTSV2;
typedef PVOID PSMHBA_ENTRYPOINTS;

HBA_STATUS HBA_API HBA_RegisterLibrary(PHBA_ENTRYPOINTS entrypoints);
HBA_STATUS HBA_API HBA_RegisterLibraryV2(PHBA_ENTRYPOINTSV2 entrypoints);


HBA_UINT32 HBA_API HBA_GetVersion();
HBA_STATUS HBA_API HBA_LoadLibrary();
HBA_STATUS HBA_API HBA_FreeLibrary();


HBA_UINT32 HBA_API HBA_GetNumberOfAdapters();

HBA_STATUS HBA_API 
HBA_GetAdapterName(
    _In_                      HBA_UINT32 AdapterIndex, 
    _Inout_updates_(MAX_NAME)  PSTR AdapterName
    );

HBA_HANDLE HBA_API 
HBA_OpenAdapter(
    _In_ PSTR AdapterName
    );

void HBA_API 
HBA_CloseAdapter(
    IN HBA_HANDLE handle
    );

HBA_STATUS HBA_API 
HBA_GetAdapterAttributes(
    IN  HBA_HANDLE             Handle,
    OUT HBA_ADAPTERATTRIBUTES *HbaAttributes
    );

HBA_STATUS HBA_API 
HBA_GetAdapterPortAttributes(
    IN  HBA_HANDLE          Handle,
    IN  HBA_UINT32          PortIndex,
    OUT HBA_PORTATTRIBUTES *PortAttributes
    );


HBA_STATUS HBA_API 
HBA_GetPortStatistics(
    IN  HBA_HANDLE          Handle,
    IN  HBA_UINT32          PortIndex,
    OUT HBA_PORTSTATISTICS *PortStatistics
    );

HBA_STATUS HBA_API 
HBA_GetDiscoveredPortAttributes(
    IN  HBA_HANDLE          Handle,
    IN  HBA_UINT32          PortIndex,
    IN  HBA_UINT32          DiscoveredPortIndex,
    OUT HBA_PORTATTRIBUTES *PortAttributes
    );

HBA_STATUS HBA_API 
HBA_GetPortAttributesByWWN(
    IN  HBA_HANDLE          Handle,
    IN  HBA_WWN             PortWWN,
    OUT HBA_PORTATTRIBUTES *PortAttributes
    );


HBA_STATUS HBA_API 
HBA_SendCTPassThru(
    IN  HBA_HANDLE Handle,
    IN  void      *pReqBuffer,
    IN  HBA_UINT32 ReqBufferSize,
    OUT void      *pRspBuffer,
    IN  HBA_UINT32 RspBufferSize
    );

HBA_STATUS HBA_API 
HBA_GetEventBuffer(
    IN     HBA_HANDLE      Handle,
    OUT    PHBA_EVENTINFO  EventBuffer,
    IN OUT HBA_UINT32     *EventCount
    );

HBA_STATUS HBA_API 
HBA_SetRNIDMgmtInfo(
    IN HBA_HANDLE    Handle,
    IN HBA_MGMTINFO *pInfo
    );

HBA_STATUS HBA_API 
HBA_GetRNIDMgmtInfo(
    IN  HBA_HANDLE    Handle,
    OUT HBA_MGMTINFO *pInfo
    );

HBA_STATUS HBA_API 
HBA_SendRNID(
    IN     HBA_HANDLE  Handle,
    IN     HBA_WWN     Wwn,
    IN     HBA_WWNTYPE WnnType,
    OUT    void       *pRspBuffer,
    IN OUT HBA_UINT32 *RspBufferSize
    );

HBA_STATUS HBA_API 
HBA_GetFcpTargetMapping(
    IN     HBA_HANDLE            Handle,
    IN OUT PHBA_FCPTARGETMAPPING Mapping
    );

HBA_STATUS HBA_API 
HBA_GetFcpPersistentBinding(
    IN     HBA_HANDLE      Handle,
    IN OUT PHBA_FCPBINDING Binding
    );

HBA_STATUS HBA_API 
HBA_SendScsiInquiry(
    IN  HBA_HANDLE  Handle,
    IN  HBA_WWN     PortWWN,
    IN  HBA_UINT64  FcLUN,
    IN  HBA_UINT8   EVPD,
    IN  HBA_UINT32  PageCode,
    OUT void       *pRspBuffer,
    IN  HBA_UINT32  RspBufferSize,
    OUT void       *pSenseBuffer,
    IN  HBA_UINT32  SenseBufferSize
    );

HBA_STATUS HBA_API 
HBA_SendReportLUNs(
    IN  HBA_HANDLE  Handle,
    IN  HBA_WWN     PortWWN,
    OUT void       *pRspBuffer,
    IN  HBA_UINT32  RspBufferSize,
    OUT void       *pSenseBuffer,
    IN  HBA_UINT32  SenseBufferSize
    );

HBA_STATUS HBA_API 
HBA_SendReadCapacity(
    IN  HBA_HANDLE  Handle,
    IN  HBA_WWN     PortWWN,
    IN  HBA_UINT64  FcLUN,
    OUT void       *pRspBuffer,
    IN  HBA_UINT32  RspBufferSize,
    OUT void       *pSenseBuffer,
    IN  HBA_UINT32  SenseBufferSize
    );

void HBA_API 
HBA_RefreshInformation(
    IN HBA_HANDLE Handle
    );

void HBA_API
HBA_ResetStatistics(
    IN HBA_HANDLE Handle,
    IN HBA_UINT32 PortIndex
    );


typedef void *HBA_CALLBACKHANDLE;

typedef HBA_CALLBACKHANDLE *PHBA_CALLBACKHANDLE;

/* Adapter Level Events */
#define HBA_EVENT_ADAPTER_UNKNOWN    0x100
#define HBA_EVENT_ADAPTER_ADD        0x101
#define HBA_EVENT_ADAPTER_REMOVE     0x102
#define HBA_EVENT_ADAPTER_CHANGE     0x103

/* Port Level Events */
#define HBA_EVENT_PORT_UNKNOWN          0x200
#define HBA_EVENT_PORT_OFFLINE          0x201
#define HBA_EVENT_PORT_ONLINE           0x202
#define HBA_EVENT_PORT_NEW_TARGETS      0x203
#define HBA_EVENT_PORT_FABRIC           0x204
#define HBA_EVENT_PORT_BROADCAST_CHANGE 0x205
#define HBA_EVENT_PORT_BROADCAST_D24_0  0x206
#define HBA_EVENT_PORT_BROADCAST_D27_4  0x207
#define HBA_EVENT_PORT_BROADCAST_SES    0x208
#define HBA_EVENT_PORT_BROADCAST_D01_4  0x209
#define HBA_EVENT_PORT_BROADCAST_D04_7  0x20a
#define HBA_EVENT_PORT_BROADCAST_D16_7  0x20b
#define HBA_EVENT_PORT_BROADCAST_D29_7  0x20c
#define HBA_EVENT_PORT_ALL              0x2ff

/* Port Statistics Events */
#define HBA_EVENT_PORT_STAT_THRESHOLD   0x301
#define HBA_EVENT_PORT_STAT_GROWTH      0x302

/* Phy Statistics Events */
#define HBA_EVENT_PHY_STAT_THRESHOLD    0x351
#define HBA_EVENT_PHY_STAT_GROWTH       0x352

/* Target Level Events */
#define HBA_EVENT_TARGET_UNKNOWN    0x400
#define HBA_EVENT_TARGET_OFFLINE    0x401
#define HBA_EVENT_TARGET_ONLINE     0x402
#define HBA_EVENT_TARGET_REMOVED    0x403

/* Fabric Link  Events */
#define HBA_EVENT_LINK_UNKNOWN      0x500
#define HBA_EVENT_LINK_INCIDENT     0x501

HBA_STATUS HBA_API
HBA_RemoveCallback(
    IN HBA_CALLBACKHANDLE callbackHandle
    );

HBA_STATUS HBA_API
HBA_RegisterForAdapterAddEvents(
    IN  void (*callback) (void *pData, HBA_WWN PortWWN, HBA_UINT32 eventType), 
    IN  void               *pUserData,
    OUT HBA_CALLBACKHANDLE *pCallbackHandle
    );

HBA_STATUS HBA_API
HBA_RegisterForAdapterEvents(
    IN  void (*callback) (void *pData, HBA_WWN PortWWN, HBA_UINT32 eventType),
    IN  void               *pUserData, 
    IN  HBA_HANDLE          Handle,
    OUT HBA_CALLBACKHANDLE *pCallbackHandle
    );

HBA_STATUS HBA_API
HBA_RegisterForAdapterPortEvents(
    IN  void (*callback) (void      *pData, 
                          HBA_WWN    PortWWN,
                          HBA_UINT32 eventType, 
                          HBA_UINT32 fabricPortID),
    IN  void               *UserData,
    IN  HBA_HANDLE          Handle,
    IN  HBA_WWN             PortWWN,
    OUT HBA_CALLBACKHANDLE *pCallbackHandle
    );

HBA_STATUS HBA_API
HBA_RegisterForAdapterPortStatEvents(
    IN  void (*callback)(void      *pData, 
                         HBA_WWN    PortWWN, 
                         HBA_UINT32 eventType),
    IN  void               *pUserData,
    IN  HBA_HANDLE          Handle,
    IN  HBA_WWN             PortWWN,
    IN  HBA_PORTSTATISTICS  stats,
    IN  HBA_UINT32          statType,
    OUT HBA_CALLBACKHANDLE *pCallbackHandle
    );

HBA_STATUS HBA_API
HBA_RegisterForTargetEvents(
    IN  void (*callback)(void      *pData, 
                         HBA_WWN    hbaPortWWN,
                         HBA_WWN    discoveredPortWWN, 
                         HBA_UINT32 eventType),
    IN  void               *pUserData,
    IN  HBA_HANDLE          Handle, 
    IN  HBA_WWN             HbaPortWWN,
    IN  HBA_WWN             DiscoveredPortWWN,
    OUT HBA_CALLBACKHANDLE *pCallbackHandle,
    IN  HBA_UINT32          AllTargets
    );

HBA_STATUS HBA_API
HBA_RegisterForLinkEvents(
    IN  void (*callback)(void *data, HBA_WWN adapterWWN, HBA_UINT32 eventType,
                     void *pRLIRBuffer, HBA_UINT32 RLIRBufferSize),
    IN  void               *userData,
    IN  void               *pRLIRBuffer,
    IN  HBA_UINT32          RLIRBufferSize,
    IN  HBA_HANDLE          Handle, 
    OUT HBA_CALLBACKHANDLE *pCallbackHandle
    );


HBA_STATUS HBA_API 
HBA_OpenAdapterByWWN(
    OUT HBA_HANDLE *HbaHandle,
    IN  HBA_WWN     Wwn
    );

void HBA_API 
HBA_RefreshAdapterConfiguration(
    );

HBA_STATUS HBA_API 
HBA_SendCTPassThruV2(
    IN     HBA_HANDLE  Handle,
    IN     HBA_WWN     HbaPortWWN,
    IN     void       *pReqBuffer,
    IN     HBA_UINT32  ReqBufferSize,
    OUT    void       *pRspBuffer,
    IN OUT HBA_UINT32 *pRspBufferSize
    );

HBA_STATUS HBA_API 
HBA_SendRNIDV2(
    IN     HBA_HANDLE  Handle,
    IN     HBA_WWN     HbaPortWWN,
    IN     HBA_WWN     DestWWN,
    IN     HBA_UINT32  DestFCID,
    IN     HBA_UINT32  NodeIdDataFormat,
    OUT    void       *pRspBuffer,
    IN OUT HBA_UINT32 *pRspBufferSize
    );

HBA_STATUS HBA_API 
HBA_SendRPL(
    IN     HBA_HANDLE  Handle,
    IN     HBA_WWN     HbaPortWWN,
    IN     HBA_WWN     Agent_wwn,
    IN     HBA_UINT32  Agent_domain,
    IN     HBA_UINT32  PortIndex,
    OUT    void       *pRspBuffer,
    IN OUT HBA_UINT32 *pRspBufferSize
    );

HBA_STATUS HBA_API 
HBA_SendRPS(
    IN     HBA_HANDLE  Handle,
    IN     HBA_WWN     HbaPortWWN,
    IN     HBA_WWN     Agent_wwn,
    IN     HBA_UINT32  Agent_domain,
    IN     HBA_WWN     Object_wwn,
    IN     HBA_UINT32  Object_port_number,
    OUT    void       *pRspBuffer,
    IN OUT HBA_UINT32 *pRspBufferSize
    );

HBA_STATUS HBA_API 
HBA_SendSRL(
    IN     HBA_HANDLE  Handle,
    IN     HBA_WWN     HbaPortWWN,
    IN     HBA_WWN     Wwn,
    IN     HBA_UINT32  Domain,
    OUT    void       *pRspBuffer,
    IN OUT HBA_UINT32 *pRspBufferSize
    );

HBA_STATUS HBA_API 
HBA_SendLIRR(
    IN     HBA_HANDLE  Handle,
    IN     HBA_WWN     SourceWWN,
    IN     HBA_WWN     DestWWN,
    IN     HBA_UINT8   Function,
    IN     HBA_UINT8   Type,
    OUT    void       *pRspBuffer,
    IN OUT HBA_UINT32 *pRspBufferSize
    );

HBA_STATUS HBA_API
HBA_SendRLS(
    IN     HBA_HANDLE  HbaHandle,
    IN     HBA_WWN     HbaPortWWN,
    IN     HBA_WWN     DestWWN,
    OUT    void       *pRspBuffer,
    IN OUT HBA_UINT32 *pRspBufferSize
    );


HBA_STATUS HBA_API 
HBA_GetFC4Statistics(
    IN  HBA_HANDLE         Handle,
    IN  HBA_WWN            PortWWN,
    IN  HBA_UINT8          FC4type,
    OUT HBA_FC4STATISTICS *Statistics
    );

HBA_STATUS HBA_API
HBA_GetFCPStatistics(
    IN  HBA_HANDLE         Handle,
    IN  const HBA_SCSIID  *Lunit,
    OUT HBA_FC4STATISTICS *Statistics
    );



typedef struct HBA_LibraryAttributes {
    HBA_BOOLEAN final;
    char        LibPath[256];
    char        VName[256];
    char        VVersion[256];
    struct tm   build_date;
} HBA_LIBRARYATTRIBUTES, *PHBA_LIBRARYATTRIBUTES;

typedef struct SMHBA_LibraryAttributes {
    char        LibPath[256];
    char        VName[256];
    char        VVersion[256];
    struct {
        int tm_mday;  /* day of the month - [1,31] */
        int tm_mon;   /* months since January - [0,11] */
        int tm_year;  /* years since 1900 */
    } build_date;
} SMHBA_LIBRARYATTRIBUTES, *PSMHBA_LIBRARYATTRIBUTES;


HBA_UINT32 HBA_API 
HBA_GetWrapperLibraryAttributes(
    OUT HBA_LIBRARYATTRIBUTES *Attributes
    );

HBA_UINT32 HBA_API 
HBA_GetVendorLibraryAttributes(
    IN  HBA_UINT32             AdapterIndex,
    OUT HBA_LIBRARYATTRIBUTES *Attributes
    );


HBA_STATUS HBA_API
HBA_ScsiReadCapacityV2(
    IN     HBA_HANDLE  HbaHandle,
    IN     HBA_WWN     HbaPortWWN,
    IN     HBA_WWN     DiscoveredPortWWN,
    IN     HBA_UINT64  FcLUN,
    OUT    void       *pRespBuffer,
    IN OUT HBA_UINT32 *pRespBufferSize,
    OUT    HBA_UINT8  *pScsiStatus,
    OUT    void       *pSenseBuffer,
    IN OUT HBA_UINT32 *pSenseBufferSize
    );

HBA_STATUS HBA_API
HBA_ScsiReportLUNsV2(
    IN     HBA_HANDLE  Hbahandle,
    IN     HBA_WWN     HbaPortWWN,
    IN     HBA_WWN     DiscoveredPortWWN,
    OUT    void       *pRespBuffer,
    IN OUT HBA_UINT32 *pRespBufferSize,
    OUT    HBA_UINT8  *pScsiStatus,
    OUT    void       *pSenseBuffer,
    IN OUT HBA_UINT32 *pSenseBufferSize
    );

HBA_STATUS HBA_API 
HBA_ScsiInquiryV2 (
    IN     HBA_HANDLE  HbaHandle,
    IN     HBA_WWN     HbaPortWWN,
    IN     HBA_WWN     DiscoveredPortWWN,
    IN     HBA_UINT64  FcLUN,
    IN     HBA_UINT8   CDB_Byte1,
    IN     HBA_UINT8   CDB_Byte2,
    OUT    void       *pRespBuffer,
    IN OUT HBA_UINT32 *pRespBufferSize,
    OUT    HBA_UINT8  *pScsiStatus,
    OUT    void       *pSenseBuffer,
    IN OUT HBA_UINT32 *pSenseBufferSize
    );


HBA_STATUS HBA_API 
HBA_GetFcpTargetMappingV2(
    IN     HBA_HANDLE              HbaHandle,
    IN     HBA_WWN                 HbaPortWWN,
    IN OUT HBA_FCPTARGETMAPPINGV2 *Mapping
    );

HBA_STATUS HBA_API
HBA_GetBindingCapability(
    IN  HBA_HANDLE           Handle,
    IN  HBA_WWN              HbaPortWWN,
    OUT HBA_BIND_CAPABILITY *Flags
    );

HBA_STATUS HBA_API
HBA_GetBindingSupport(
    IN  HBA_HANDLE           Handle,
    IN  HBA_WWN              HbaPortWWN,
    OUT HBA_BIND_CAPABILITY *Flags
    );

HBA_STATUS HBA_API
HBA_SetBindingSupport(
    IN HBA_HANDLE          Handle,
    IN HBA_WWN             HbaPortWWN,
    IN HBA_BIND_CAPABILITY Flags
    );


HBA_STATUS HBA_API
HBA_GetPersistentBindingV2(
    IN     HBA_HANDLE       Handle,
    IN     HBA_WWN          HbaPortWWN,
    IN OUT PHBA_FCPBINDING2 Binding
    );

HBA_STATUS HBA_API
HBA_SetPersistentBindingV2(
    IN HBA_HANDLE       Handle,
    IN HBA_WWN          HbaPortWWN,
    IN PHBA_FCPBINDING2 Binding
    );

HBA_STATUS HBA_API
HBA_RemovePersistentBinding(
    IN HBA_HANDLE       Handle,
    IN HBA_WWN          HbaPortWWN,
    IN PHBA_FCPBINDING2 Binding
    );

HBA_STATUS HBA_API
HBA_RemoveAllPersistentBindings(
    IN HBA_HANDLE  Handle,
    IN HBA_WWN     HbaPortWWN
    );


#if 0

//
// stubs for unsupported SB functions
//

typedef PVOID PHBA_SBTARGETMAPPING;

typedef struct HBA_SBDevId {
    char  OSDeviceName[256];
} HBA_SBDEVID, *PHBA_SBDEVID;


typedef struct HBA_Ned {
    HBA_UINT32  NEDWord0;
    HBA_UINT32  NelId[7];
} HBA_NED, *PHBA_NED;

typedef struct HBA_DeviceSelfDesc {
    HBA_NED  TokenNED;
    HBA_NED  DeviceNED;
} HBA_DEVICESELFDESC, *PHBA_DEVICESELFDESC;

typedef PVOID PHBA_SBSTATISTICS;

typedef struct HBA_SBDskCapacity {
    HBA_INT32  SCSIFormatLBA;          /* SCSI Read Capacity Format */
    HBA_INT32  SCSIFormatBlkLen;       /* SCSI Read Capacity Format */
    HBA_INT32  SBDskNumberOfCylinders; /* cyls */
    HBA_INT32  SBDskTracksPerCylinder; /* tracks per cyl */
    HBA_INT32  SBDskMaxUsableTrackLen; /* usable track capcacity */
} HBA_SBDSKCAPACITY, *PHBA_SBDSKCAPACITY;
   

HBA_STATUS HBA_API
HBA_GetSBTargetMapping(
    IN  HBA_HANDLE           HbaHandle,
    IN  HBA_WWN              HbaPortWWN,
    OUT PHBA_SBTARGETMAPPING Mapping
    );

HBA_STATUS HBA_API
HBA_GetSBStatistics(
    IN  HBA_HANDLE         HbaHandle,
    IN  const HBA_SBDEVID *Device,
    OUT PHBA_SBSTATISTICS  Statistics
    );

HBA_STATUS HBA_API
HBA_SBDskGetCapacity(
    IN  HBA_DEVICESELFDESC  DeviceSelfDesc,
    OUT PHBA_SBDSKCAPACITY  PSbDskCapacity
    );

#endif

#ifdef SM_HBA_API

//
// SM-HBA Library Control Functions
//

#define SMHBA_VERSION 1

HBA_UINT32 HBA_API SMHBA_GetVersion();
HBA_STATUS HBA_API SMHBA_RegisterLibrary(PSMHBA_ENTRYPOINTS entrypoints);

HBA_UINT32 HBA_API
SMHBA_GetWrapperLibraryAttributes(
    OUT SMHBA_LIBRARYATTRIBUTES *Attributes
    );

HBA_UINT32 HBA_API
SMHBA_GetVendorLibraryAttributes(
    IN  HBA_UINT32                AdapterIndex,
    OUT SMHBA_LIBRARYATTRIBUTES  *Attributes
    );


//
// SM-HBA Adapter, FC_Port and SAS_Port Information Functions
//

HBA_STATUS HBA_API 
SMHBA_GetProtocolStatistics(
    IN  HBA_HANDLE                Handle,
    IN  HBA_UINT32                PortIndex,
    IN  HBA_UINT32                ProtocolType,
    OUT SMHBA_PROTOCOLSTATISTICS *ProtocolStatistics
    );

HBA_STATUS HBA_API
SMHBA_GetNumberOfPorts(
    IN  HBA_HANDLE  Handle,
    OUT HBA_UINT32 *NumberOfPorts
    );

HBA_STATUS HBA_API 
SMHBA_GetAdapterAttributes(
    IN  HBA_HANDLE               Handle,
    OUT SMHBA_ADAPTERATTRIBUTES *AdapterAttributes
    );

HBA_STATUS HBA_API 
SMHBA_GetPhyStatistics(
    IN  HBA_HANDLE           Handle,
    IN  HBA_UINT32           PortIndex,
    IN  HBA_UINT32           PhyIndex,
    OUT SMHBA_PHYSTATISTICS *PhyStatistics
    );

HBA_STATUS HBA_API 
SMHBA_GetDiscoveredPortAttributes(
    IN  HBA_HANDLE            Handle,
    IN  HBA_UINT32            PortIndex,
    IN  HBA_UINT32            DiscoveredPortIndex,
    OUT SMHBA_PORTATTRIBUTES *PortAttributes
    );

HBA_STATUS HBA_API 
SMHBA_GetPortAttributesByWWN(
    IN  HBA_HANDLE            Handle,
    IN  HBA_WWN               PortWWN,
    IN  HBA_WWN               DomainPortWWN,
    OUT SMHBA_PORTATTRIBUTES *PortAttributes
    );

HBA_STATUS HBA_API
SMHBA_GetPortType(
    IN  HBA_HANDLE    Handle,
    IN  HBA_UINT32    PortIndex,
    OUT HBA_PORTTYPE *PortType
    );

HBA_STATUS HBA_API 
SMHBA_GetAdapterPortAttributes(
    IN  HBA_HANDLE            Handle,
    IN  HBA_UINT32            PortIndex,
    OUT SMHBA_PORTATTRIBUTES *PortAttributes
    );

HBA_STATUS HBA_API 
SMHBA_GetFCPhyAttributes(
    IN  HBA_HANDLE    Handle,
    IN  HBA_UINT32    PortIndex,
    IN  HBA_UINT32    PhyIndex,
    OUT SMHBA_FC_PHY *PhyType
    );

HBA_STATUS HBA_API 
SMHBA_GetSASPhyAttributes(
    IN  HBA_HANDLE     Handle,
    IN  HBA_UINT32     PortIndex,
    IN  HBA_UINT32     PhyIndex,
    OUT SMHBA_SAS_PHY *PhyType
    );

//
// SM-HBA Target Information Functions
//

HBA_STATUS HBA_API
SMHBA_GetLUNStatistics(
    IN  HBA_HANDLE                Handle,
    IN  const HBA_SCSIID         *Lunit,
    OUT SMHBA_PROTOCOLSTATISTICS *ProtocolStatistics
    );

HBA_STATUS HBA_API 
SMHBA_GetTargetMapping(
    IN     HBA_HANDLE           HbaHandle,
    IN     HBA_WWN              HbaPortWWN,
    IN     HBA_WWN              DomainPortWWN,
    IN OUT SMHBA_TARGETMAPPING *Mapping
    );

HBA_STATUS HBA_API
SMHBA_GetBindingCapability(
    IN  HBA_HANDLE             Handle,
    IN  HBA_WWN                HbaPortWWN,
    IN  HBA_WWN                DomainPortWWN,
    OUT SMHBA_BIND_CAPABILITY *Flags
    );

HBA_STATUS HBA_API
SMHBA_GetBindingSupport(
    IN  HBA_HANDLE             Handle,
    IN  HBA_WWN                HbaPortWWN,
    IN  HBA_WWN                DomainPortWWN,
    OUT SMHBA_BIND_CAPABILITY *Flags
    );

HBA_STATUS HBA_API
SMHBA_SetBindingSupport(
    IN HBA_HANDLE            Handle,
    IN HBA_WWN               HbaPortWWN,
    IN HBA_WWN               DomainPortWWN,
    IN SMHBA_BIND_CAPABILITY Flags
    );

HBA_STATUS HBA_API
SMHBA_GetPersistentBinding(
    IN     HBA_HANDLE     Handle,
    IN     HBA_WWN        HbaPortWWN,
    IN     HBA_WWN        DomainPortWWN,
    IN OUT SMHBA_BINDING *Binding
    );

HBA_STATUS HBA_API
SMHBA_SetPersistentBinding(
    IN HBA_HANDLE           Handle,
    IN HBA_WWN              HbaPortWWN,
    IN HBA_WWN              DomainPortWWN,
    IN OUT SMHBA_BINDING   *Binding
    );

HBA_STATUS HBA_API
SMHBA_RemovePersistentBinding(
    IN HBA_HANDLE           Handle,
    IN HBA_WWN              HbaPortWWN,
    IN HBA_WWN              DomainPortWWN,
    IN const SMHBA_BINDING *Binding
    );

HBA_STATUS HBA_API
SMHBA_RemoveAllPersistentBindings(
    IN HBA_HANDLE  Handle,
    IN HBA_WWN     HbaPortWWN,
    IN HBA_WWN     DomainPortWWN
    );

//
// SCSI Information Functions
//

HBA_STATUS HBA_API 
SMHBA_ScsiInquiry(
    IN     HBA_HANDLE  HbaHandle,
    IN     HBA_WWN     HbaPortWWN,
    IN     HBA_WWN     DiscoveredPortWWN,
    IN     HBA_WWN     DomainPortWWN,
    IN     HBA_SCSILUN SmhbaLUN,
    IN     HBA_UINT8   CDB_Byte1,
    IN     HBA_UINT8   CDB_Byte2,
    OUT    void       *pRespBuffer,
    IN OUT HBA_UINT32 *pRespBufferSize,
    OUT    HBA_UINT8  *pScsiStatus,
    OUT    void       *pSenseBuffer,
    IN OUT HBA_UINT32 *pSenseBufferSize
    );

HBA_STATUS HBA_API
SMHBA_ScsiReportLuns(
    IN     HBA_HANDLE  Hbahandle,
    IN     HBA_WWN     HbaPortWWN,
    IN     HBA_WWN     DiscoveredPortWWN,
    IN     HBA_WWN     DomainPortWWN,
    OUT    void       *pRespBuffer,
    IN OUT HBA_UINT32 *pRespBufferSize,
    OUT    HBA_UINT8  *pScsiStatus,
    OUT    void       *pSenseBuffer,
    IN OUT HBA_UINT32 *pSenseBufferSize
    );

HBA_STATUS HBA_API
SMHBA_ScsiReadCapacity(
    IN     HBA_HANDLE  HbaHandle,
    IN     HBA_WWN     HbaPortWWN,
    IN     HBA_WWN     DiscoveredPortWWN,
    IN     HBA_WWN     DomainPortWWN,
    IN     HBA_SCSILUN SmhbaLUN,
    OUT    void       *pRespBuffer,
    IN OUT HBA_UINT32 *pRespBufferSize,
    OUT    HBA_UINT8  *pScsiStatus,
    OUT    void       *pSenseBuffer,
    IN OUT HBA_UINT32 *pSenseBufferSize
    );

//
// Fabric and Domain Management Functions
//

// new FC specific

HBA_STATUS HBA_API
SMHBA_SendTEST(
    IN HBA_HANDLE  Handle,
    IN HBA_WWN     HbaPortWWN,
    IN HBA_WWN     DestWWN,
    IN HBA_UINT32  DestFCID,
    IN void       *ReqBuffer,
    IN HBA_UINT32  ReqBufferSize
    );

// new FC specific

HBA_STATUS HBA_API
SMHBA_SendECHO(
    IN HBA_HANDLE      Handle,
    IN HBA_WWN         HbaPortWWN,
    IN HBA_WWN         DestWWN,
    IN HBA_UINT32      DestFCID,
    IN void           *ReqBuffer,
    IN HBA_UINT32      ReqBufferSize,
    OUT    void       *RspBuffer,
    IN OUT HBA_UINT32 *RspBufferSize
    );

// new SAS specific

HBA_STATUS HBA_API
SMHBA_SendSMPPassThru(
    IN HBA_HANDLE      Handle,
    IN HBA_WWN         HbaPortWWN,
    IN HBA_WWN         DestPortWWN,
    IN HBA_WWN         DomainPortWWN,
    IN void           *ReqBuffer,
    IN HBA_UINT32      ReqBufferSize,
    OUT    void       *RspBuffer,
    IN OUT HBA_UINT32 *RspBufferSize
    );


//
// Event Handling Functions
//


HBA_STATUS HBA_API
SMHBA_RegisterForAdapterAddEvents(
    IN  void (*callback)(void *pData, HBA_WWN PortWWN, HBA_UINT32 eventType), 
    IN  void               *pUserData,
    OUT HBA_CALLBACKHANDLE *pCallbackHandle
    );

HBA_STATUS HBA_API
SMHBA_RegisterForAdapterEvents(
    IN  void (*callback)(void *pData, HBA_WWN PortWWN, HBA_UINT32 eventType),
    IN  void               *pUserData, 
    IN  HBA_HANDLE          Handle,
    OUT HBA_CALLBACKHANDLE *pCallbackHandle
    );

HBA_STATUS HBA_API
SMHBA_RegisterForAdapterPortEvents(
    IN  void (*callback)(void      *pData, 
                         HBA_WWN    PortWWN,
                         HBA_UINT32 eventType, 
                         HBA_UINT32 fabricPortID),
    IN  void               *pUserData,
    IN  HBA_HANDLE          Handle,
    IN  HBA_WWN             PortWWN,
    IN  HBA_UINT32          SpecificEventType,
    OUT HBA_CALLBACKHANDLE *pCallbackHandle
    );

HBA_STATUS HBA_API
SMHBA_RegisterForAdapterPortStatEvents(
    IN  void (*callback)(void      *pData, 
                         HBA_WWN    PortWWN, 
                         HBA_UINT32 ProtocolType,
                         HBA_UINT32 EventType),
    IN  void                     *pUserData,
    IN  HBA_HANDLE                Handle,
    IN  HBA_WWN                   PortWWN,
    IN  HBA_UINT32                ProtocolType,
    IN  SMHBA_PROTOCOLSTATISTICS  Stats,
    IN  HBA_UINT32                StatType,
    OUT HBA_CALLBACKHANDLE       *pCallbackHandle
    );

HBA_STATUS HBA_API
SMHBA_RegisterForAdapterPhyStatEvents(
    IN  void (*callback)(void      *pData, 
                         HBA_WWN    PortWWN,
                         HBA_UINT32 PhyIndex,
                         HBA_UINT32 EventType),
    IN  void                *pUserData,
    IN  HBA_HANDLE           Handle, 
    IN  HBA_WWN              PortWWN,
    IN  HBA_UINT32           PhyIndex,
    IN  SMHBA_PHYSTATISTICS  Stats,
    IN  HBA_UINT32           StatType,
    OUT HBA_CALLBACKHANDLE  *pCallbackHandle
    );

HBA_STATUS HBA_API
SMHBA_RegisterForTargetEvents(
    IN  void (*callback)(void      *pData, 
                         HBA_WWN    HbaPortWWN, 
                         HBA_WWN    DiscoveredPortWWN, 
                         HBA_WWN    DomainPortWWN, 
                         HBA_UINT32 EventType),
    IN  void               *pUserData,
    IN  HBA_HANDLE          Handle, 
    IN  HBA_WWN             HbaPortWWN, 
    IN  HBA_WWN             DiscoveredPortWWN, 
    IN  HBA_WWN             DomainPortWWN, 
    OUT HBA_CALLBACKHANDLE *pCallbackHandle,
    IN  HBA_UINT32          AllTargets
    );

#endif // _SM_HBA_API_

#ifdef __cplusplus
};
#endif

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM) */
#pragma endregion

#endif //NTDDI_VERSION

#endif // HBAAPI_H

