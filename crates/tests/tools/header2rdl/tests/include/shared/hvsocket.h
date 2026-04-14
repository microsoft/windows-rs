/*++

Copyright (c) Microsoft Corporation

Module Name:

    HvSocket.h

Abstract:

    This file contains the core definitions for the Hyper-V
    socket address family that can be used by both user-mode and
    kernel mode modules.

    Reference AF_HYPERV in ws2def.h

--*/

#ifdef _MSC_VER
#pragma once
#endif //_MSC_VER

#include <initguid.h>
#include <ws2def.h>

//
// Hyper-v Socket options.
//

//
// Following options are available for Socket option level
// HV_PROTOCOL_RAW:
//

//
// HVSOCKET_CONNECT_TIMEOUT:
// Input:
// Type: ULONG.
// Description:
// The timeout in milliseconds.
//
#define HVSOCKET_CONNECT_TIMEOUT        0x01

//
// The maximum connect timeout is 5 minutes.
//
#define HVSOCKET_CONNECT_TIMEOUT_MAX   300000

//
// HVSOCKET_CONNECTED_SUSPEND:
// Input:
// Type: ULONG.
// Description:
// Set socket connected suspend flag. Non-zero value indicates the socket will not disconnect
// on VM pause.
//
#define HVSOCKET_CONNECTED_SUSPEND     0x04

//
// HVSOCKET_HIGH_VTL:
// Input:
// Type: ULONG.
// Description:
// Set socket high VTL flag. Non-zero value set on a host socket indicates that the socket will
// connect to guest VTL 2.
//
#define HVSOCKET_HIGH_VTL     0x08

//
// Well-known GUIDs.
//
DEFINE_GUID(HV_GUID_ZERO,  0x00000000, 0x0000, 0x0000, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00);
DEFINE_GUID(HV_GUID_BROADCAST, 0xFFFFFFFF, 0xFFFF, 0xFFFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF);

//
// HV_GUID_WILDCARD:
//
// Wildcard address. Listeners should bind to this VmId to accept
// connection from all partitions.
//
// 00000000-0000-0000-0000-000000000000
//
#define HV_GUID_WILDCARD HV_GUID_ZERO

//
// HV_GUID_CHILDREN:
//
// Wildcard address for children. Listeners should bind to this VmId
// to accept connection from its children.
//
// 90db8b89-0d35-4f79-8ce9-49ea0ac8b7cd
//

DEFINE_GUID(HV_GUID_CHILDREN, 0x90db8b89, 0x0d35, 0x4f79, 0x8c, 0xe9, 0x49, 0xea, 0x0a, 0xc8, 0xb7, 0xcd);

//
// HV_GUID_LOOPBACK:
//
// Loopback address. Using this VmId connects to the same partition as the connector.
//
// e0e16197-dd56-4a10-9195-5ee7a155a838
//
DEFINE_GUID(HV_GUID_LOOPBACK, 0xe0e16197, 0xdd56, 0x4a10, 0x91, 0x95, 0x5e, 0xe7, 0xa1, 0x55, 0xa8, 0x38);

//
// HV_GUID_PARENT:
//
// Parent address. Using this VmId connects to the parent partition of the connector.
// The parent of a virtual machine is its host.
// The parent of a hosted silo is the VM's host. (passthru)
//
// Listening on this VmId accepts connection from:
// Inside silos: silo host partition. 
// Inside hosted silo: host of the VM.
// Inside VM: VM host.
// Physical host: Not supported.
//
// a42e7cda-d03f-480c-9cc2-a4de20abb878
//
DEFINE_GUID(HV_GUID_PARENT, 0xa42e7cda, 0xd03f, 0x480c, 0x9c, 0xc2, 0xa4, 0xde, 0x20, 0xab, 0xb8, 0x78);

//
// HV_GUID_SILOHOST:
//
// Address of a silo's host partition. 
//
// The silo host of a hosted silo is the utility VM.
// The silo host of a silo on a physical host is the physical host.
//
// 36bd0c5c-7276-4223-88ba-7d03b654c568
//
DEFINE_GUID(HV_GUID_SILOHOST, 0x36bd0c5c, 0x7276, 0x4223, 0x88, 0xba, 0x7d, 0x03, 0xb6, 0x54, 0xc5, 0x68);

//
// HV_GUID_VSOCK_TEMPLATE:
// 
// Template to facilitate mapping between AF_HYPERV.ServiceId (GUID) and AF_VSOCK.port (uint32) 
// Services offered to Linux guests must use a ServiceId that matches this template.
//
// Linux guests use AF_VSOCK to communicate to a Hyper-V host using Hyper-V Sockets.
// The following mappings from cid to VmId are included in the Linux Hyper-V Socket driver:
//      VMADDR_CID_HOST (2)                 HV_GUID_PARENT 
//      VMADDR_CID_ANY (0xffffffff)         HV_GUID_WILDCARD 
//
// The Linux Hyper-V Socket driver will map the AF_VSOCK port to the Data1 value of a GUID where Data2, Data3, 
// and Data4 match HV_GUID_VSOCK_TEMPLATE (i.e. <port>-facb-11e6-bd58-64006a7986d3)
// Valid port mappings are 0-0x7FFFFFFF
// Here is a table of possible mappings from port to GUID: 
//              0                       00000000-facb-11e6-bd58-64006a7986d3
//              1                       00000001-facb-11e6-bd58-64006a7986d3
//              50                      00000032-facb-11e6-bd58-64006a7986d3
//              2147483647              7fffffff-facb-11e6-bd58-64006a7986d3  
//              
// 00000000-facb-11e6-bd58-64006a7986d3
//
DEFINE_GUID(HV_GUID_VSOCK_TEMPLATE, 0x00000000, 0xfacb, 0x11e6, 0xbd, 0x58, 0x64, 0x00, 0x6a, 0x79, 0x86, 0xd3);




#define HV_PROTOCOL_RAW 1

typedef struct _SOCKADDR_HV
{
     ADDRESS_FAMILY Family;
     USHORT Reserved;
     GUID VmId;
     GUID ServiceId;
}SOCKADDR_HV, *PSOCKADDR_HV;


#define HVSOCKET_ADDRESS_FLAG_PASSTHRU 0x00000001

typedef struct _HVSOCKET_ADDRESS_INFO
{
    GUID SystemId;
    GUID VirtualMachineId;
    GUID SiloId;
    ULONG Flags;

} HVSOCKET_ADDRESS_INFO, *PHVSOCKET_ADDRESS_INFO;

