/*++

Copyright (c) Microsoft Corporation. All rights reserved.

Module Name:

    mpradmin.h

Abstract:

    This file contains the structures, defines and function prototypes for the
    following APIs:

        MprAdminIsServiceRunning
        MprAdminServerConnect
        MprAdminServerDisconnect
        MprAdminBufferFree
        MprAdminPortEnum
        MprAdminConnectionEnum
        MprAdminPortGetInfo
        MprAdminConnectionGetInfo
        MprAdminPortClearStats
        MprAdminPortReset
        MprAdminConnectionClearStats
        MprAdminPortDisconnect
        MprAdminGetErrorString

        MprAdminAcceptNewConnection
        MprAdminAcceptNewLink
        MprAdminConnectionHangupNotification
        MprAdminLinkHangupNotification
        MprAdminGetIpAddressForUser
        MprAdminReleaseIpAddress
        MprAdminInitializeDll
        MprAdminTerminateDll
        MprAdminAcceptNewConnection2
        MprAdminConnectionHangupNotification2
        MprAdminAcceptReauthentication

        MprAdminUserGetInfo
        MprAdminUserSetInfo
        MprAdminSendUserMessage
        MprAdminGetPDCServer

        MprAdminRegisterConnectionNotification
        MprAdminDeregisterConnectionNotification

        MprAdminIsServiceRunning
        MprAdminServerConnect
        MprAdminServerDisconnect
        MprAdminBufferFree
        MprAdminServerSetInfo
        MprAdminServerGetInfo        
        MprAdminGetErrorString
        MprAdminTransportCreate
        MprAdminTransportSetInfo
        MprAdminTransportGetInfo
        MprAdminInterfaceCreate
        MprAdminInterfaceDelete
        MprAdminInterfaceGetInfo
        MprAdminInterfaceTransportAdd
        MprAdminInterfaceTransportGetInfo
        MprAdminInterfaceTransportSetInfo
        MprAdminInterfaceTransportRemove
        MprAdminInterfaceGetHandle
        MprAdminInterfaceSetCredentials
        MprAdminInterfaceGetCredentials
        MprAdminInterfaceEnum
        MprAdminInterfaceConnect
        MprAdminInterfaceDisconnect
        MprAdminInterfaceUpdateRoutes
        MprAdminInterfaceQueryUpdateResult
        MprAdminInterfaceUpdatePhonebookInfo
        MprAdminEstablishDomainRasServer
        MprAdminIsDomainRasServer

        MprAdminMIBServerConnect
        MprAdminMIBServerDisconnect
        MprAdminMIBBufferFree
        MprAdminMIBEntryCreate
        MprAdminMIBEntryDelete
        MprAdminMIBEntryGet
        MprAdminMIBEntrySet
        MprAdminMIBEntryGetFirst
        MprAdminMIBEntryGetNext
        MprAdminMIBSetTrapInfo
        MprAdminMIBGetTrapInfo

    All MIB APIs operate with the conceptual MIB row.

        MprConfigServerConnect
        MprConfigServerDisconnect
        MprConfigBufferFree
        MprConfigServerSetInfo
        MprConfigServerGetInfo
        MprConfigServerBackup
        MprConfigServerRestore
        MprConfigTransportCreate
        MprConfigTransportDelete
        MprConfigTransportGetHandle
        MprConfigTransportSetInfo
        MprConfigTransportGetInfo
        MprConfigTransportEnum
        MprConfigInterfaceCreate
        MprConfigInterfaceDelete
        MprConfigInterfaceGetHandle
        MprConfigInterfaceGetInfo
        MprConfigInterfaceEnum
        MprConfigInterfaceTransportAdd
        MprConfigInterfaceTransportRemove
        MprConfigInterfaceTransportGetHandle
        MprConfigInterfaceTransportGetInfo
        MprConfigInterfaceTransportSetInfo
        MprConfigInterfaceTransportEnum
        MprConfigFilterGetInfo
        MprConfigFilterSetInfo
        
        MprSetupIpInIpInterfaceFriendlyNameEnum
        MprSetupIpInIpInterfaceFriendlyNameFree
        MprSetupIpInIpInterfaceFriendlyNameCreate
        MprSetupIpInIpInterfaceFriendlyNameDelete

--*/


#ifndef __ROUTING_MPRADMIN_H__
#define __ROUTING_MPRADMIN_H__

#if _MSC_VER > 1000
#pragma once
#endif
#include <winapifamily.h>

#pragma region Desktop Family or CmdLineTools Package
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_PKG_CMDTOOLS)


#include <lmcons.h>
#include <ras.h>
#include <in6addr.h>
#include <wincrypt.h>

#ifdef __cplusplus
extern "C" {
#endif

//
// Name of the Routing and RemoteAccess Service
//

#define RRAS_SERVICE_NAME       TEXT("RemoteAccess")

//
// Protocol IDs
//

#define PID_IPX                 0x0000002B
#define PID_IP                  0x00000021
#define PID_IPV6                0x00000057
#define PID_NBF                 0x0000003F
#define PID_ATALK               0x00000029

#include <mprapidef.h>

//
// MPR Interface structures and definitions.
//

//
// MPR Interface types
//

typedef enum _ROUTER_INTERFACE_TYPE
{
    ROUTER_IF_TYPE_CLIENT,
    ROUTER_IF_TYPE_HOME_ROUTER,
    ROUTER_IF_TYPE_FULL_ROUTER,
    ROUTER_IF_TYPE_DEDICATED,
    ROUTER_IF_TYPE_INTERNAL,
    ROUTER_IF_TYPE_LOOPBACK,
    ROUTER_IF_TYPE_TUNNEL1,
    ROUTER_IF_TYPE_DIALOUT,
    ROUTER_IF_TYPE_MAX // do not use.Illegal
}
ROUTER_INTERFACE_TYPE;

typedef enum _ROUTER_CONNECTION_STATE
{
    ROUTER_IF_STATE_UNREACHABLE,
    ROUTER_IF_STATE_DISCONNECTED,
    ROUTER_IF_STATE_CONNECTING,
    ROUTER_IF_STATE_CONNECTED
}
ROUTER_CONNECTION_STATE;

#define MPR_INTERFACE_OUT_OF_RESOURCES              0x00000001
#define MPR_INTERFACE_ADMIN_DISABLED                0x00000002
#define MPR_INTERFACE_CONNECTION_FAILURE            0x00000004
#define MPR_INTERFACE_SERVICE_PAUSED                0x00000008
#define MPR_INTERFACE_DIALOUT_HOURS_RESTRICTION     0x00000010
#define MPR_INTERFACE_NO_MEDIA_SENSE                0x00000020
#define MPR_INTERFACE_NO_DEVICE                     0x00000040

typedef struct _MPR_INTERFACE_0
{
    IN OUT _Field_z_  WCHAR                   wszInterfaceName[MAX_INTERFACE_NAME_LEN+1];
    OUT     HANDLE                  hInterface;
    IN OUT  BOOL                    fEnabled;
    IN OUT  ROUTER_INTERFACE_TYPE   dwIfType;
    OUT     ROUTER_CONNECTION_STATE dwConnectionState;
    OUT     DWORD                   fUnReachabilityReasons;
    OUT     DWORD                   dwLastError;

}
MPR_INTERFACE_0, *PMPR_INTERFACE_0;

typedef struct _MPR_IPINIP_INTERFACE_0
{
    WCHAR   wszFriendlyName[MAX_INTERFACE_NAME_LEN+1];

    GUID    Guid;

}MPR_IPINIP_INTERFACE_0, *PMPR_IPINIP_INTERFACE_0;

#if(WINVER >= 0x0500)

typedef struct _MPR_INTERFACE_1
{
    IN OUT  WCHAR                   wszInterfaceName[MAX_INTERFACE_NAME_LEN+1];
    OUT     HANDLE                  hInterface;
    IN OUT  BOOL                    fEnabled;
    IN OUT  ROUTER_INTERFACE_TYPE   dwIfType;
    OUT     ROUTER_CONNECTION_STATE dwConnectionState;
    OUT     DWORD                   fUnReachabilityReasons;
    OUT     DWORD                   dwLastError;
    OUT     LPWSTR                  lpwsDialoutHoursRestriction;

}
MPR_INTERFACE_1, *PMPR_INTERFACE_1;

//
// MPR_INTERFACE_2 definitions
//

#define MPR_MaxDeviceType     RAS_MaxDeviceType
#define MPR_MaxPhoneNumber    RAS_MaxPhoneNumber
#define MPR_MaxIpAddress      RAS_MaxIpAddress
#define MPR_MaxIpxAddress     RAS_MaxIpxAddress

#define MPR_MaxEntryName      RAS_MaxEntryName
#define MPR_MaxDeviceName     RAS_MaxDeviceName
#define MPR_MaxCallbackNumber RAS_MaxCallbackNumber

#define MPR_MaxAreaCode       RAS_MaxAreaCode
#define MPR_MaxPadType        RAS_MaxPadType
#define MPR_MaxX25Address     RAS_MaxX25Address
#define MPR_MaxFacilities     RAS_MaxFacilities
#define MPR_MaxUserData       RAS_MaxUserData

//
// MPR_INTERFACE_2 'dwfOptions' bit flags.
//

#define MPRIO_SpecificIpAddr            RASEO_SpecificIpAddr
#define MPRIO_SpecificNameServers       RASEO_SpecificNameServers
#define MPRIO_IpHeaderCompression       RASEO_IpHeaderCompression
#define MPRIO_RemoteDefaultGateway      RASEO_RemoteDefaultGateway
#define MPRIO_DisableLcpExtensions      RASEO_DisableLcpExtensions
#define MPRIO_SwCompression             RASEO_SwCompression
#define MPRIO_RequireEncryptedPw        RASEO_RequireEncryptedPw
#define MPRIO_RequireMsEncryptedPw      RASEO_RequireMsEncryptedPw
#define MPRIO_RequireDataEncryption     RASEO_RequireDataEncryption
#define MPRIO_NetworkLogon              RASEO_NetworkLogon
#define MPRIO_PromoteAlternates         RASEO_PromoteAlternates
#define MPRIO_SecureLocalFiles          RASEO_SecureLocalFiles
#define MPRIO_RequireEAP                RASEO_RequireEAP
#define MPRIO_RequirePAP                RASEO_RequirePAP
#define MPRIO_RequireSPAP               RASEO_RequireSPAP
#define MPRIO_SharedPhoneNumbers        RASEO_SharedPhoneNumbers
#define MPRIO_RequireCHAP               RASEO_RequireCHAP
#define MPRIO_RequireMsCHAP             RASEO_RequireMsCHAP
#define MPRIO_RequireMsCHAP2            RASEO_RequireMsCHAP2

#if (WINVER >= 0x501)
#define MPRIO_IpSecPreSharedKey         0x80000000
#endif

#if (WINVER >= 0x602)
#define MPRIO_RequireMachineCertificates              0x01000000
#define MPRIO_UsePreSharedKeyForIkev2Initiator        0x02000000
#define MPRIO_UsePreSharedKeyForIkev2Responder        0x04000000
#endif

//
// MPR_INTERFACE_2 'dwProtocols' bit flags.
//

#define MPRNP_Ipx                       RASNP_Ipx
#define MPRNP_Ip                        RASNP_Ip
#if (WINVER >= 0x600)
#define MPRNP_Ipv6                      RASNP_Ipv6
#endif

//
// MPR_INTERFACE_2 'szDeviceType' default strings.
//

#define MPRDT_Modem                     RASDT_Modem
#define MPRDT_Isdn                      RASDT_Isdn
#define MPRDT_X25                       RASDT_X25
#define MPRDT_Vpn                       RASDT_Vpn
#define MPRDT_Pad                       RASDT_Pad
#define MPRDT_Generic                   RASDT_Generic
#define MPRDT_Serial                    RASDT_Serial        			
#define MPRDT_FrameRelay                RASDT_FrameRelay
#define MPRDT_Atm                       RASDT_Atm
#define MPRDT_Sonet                     RASDT_Sonet
#define MPRDT_SW56                      RASDT_SW56
#define MPRDT_Irda                      RASDT_Irda
#define MPRDT_Parallel                  RASDT_Parallel

//
// MPR_INTERFACE_2 'dwType' settings
//

#define MPRET_Phone    RASET_Phone
#define MPRET_Vpn      RASET_Vpn
#define MPRET_Direct   RASET_Direct

//
// MPR_INTERFACE_2 'dwDialMode' values.
//

#define MPRDM_DialFirst                0
#define MPRDM_DialAll                  RASEDM_DialAll
#define MPRDM_DialAsNeeded             RASEDM_DialAsNeeded

//
// MPR_INTERFACE_2 'dwIdleDisconnectSeconds' constants.
//

#define MPRIDS_Disabled                 RASIDS_Disabled
#define MPRIDS_UseGlobalValue           RASIDS_UseGlobalValue

//
// MPR_INTERFACE_2 encryption types.
//

#define MPR_ET_None         ET_None         
#define MPR_ET_Require      ET_Require      
#define MPR_ET_RequireMax   ET_RequireMax   
#define MPR_ET_Optional     ET_Optional     

//
// MPR_INTERFACE_2 Vpn strategies
//

#define MPR_VS_Default	        VS_Default		
#define MPR_VS_PptpOnly	        VS_PptpOnly	
#define MPR_VS_PptpFirst        VS_PptpFirst	
#define MPR_VS_L2tpOnly         VS_L2tpOnly 	
#define MPR_VS_L2tpFirst        VS_L2tpFirst	
#define MPR_VS_Ikev2Only        VS_Ikev2Only
#define MPR_VS_Ikev2First       VS_Ikev2First

//
// Used to create/get/set a demand dial interface plus its
// ras configuration.
//

typedef struct _MPR_INTERFACE_2
{
    IN OUT  WCHAR                   wszInterfaceName[MAX_INTERFACE_NAME_LEN+1];
    OUT     HANDLE                  hInterface;
    IN OUT  BOOL                    fEnabled;
    IN OUT  ROUTER_INTERFACE_TYPE   dwIfType;
    OUT     ROUTER_CONNECTION_STATE dwConnectionState;
    OUT     DWORD                   fUnReachabilityReasons;
    OUT     DWORD                   dwLastError;

    //
    // Demand dial-specific properties
    //

    DWORD       dwfOptions;

    //
    // Location/phone number
    //

    WCHAR       szLocalPhoneNumber[ RAS_MaxPhoneNumber + 1 ];
    PWCHAR      szAlternates;

    //
    // PPP/Ip
    //

    DWORD       ipaddr;
    DWORD       ipaddrDns;
    DWORD       ipaddrDnsAlt;
    DWORD       ipaddrWins;
    DWORD       ipaddrWinsAlt;

    //
    // NetProtocols
    //

    DWORD       dwfNetProtocols;

    //
    // Device
    //

    WCHAR       szDeviceType[ MPR_MaxDeviceType + 1 ];
    WCHAR       szDeviceName[ MPR_MaxDeviceName + 1 ];

    //
    // X.25
    //

    WCHAR       szX25PadType[ MPR_MaxPadType + 1 ];
    WCHAR       szX25Address[ MPR_MaxX25Address + 1 ];
    WCHAR       szX25Facilities[ MPR_MaxFacilities + 1 ];
    WCHAR       szX25UserData[ MPR_MaxUserData + 1 ];
    DWORD       dwChannels;

    //
    // Multilink
    //

    DWORD       dwSubEntries;
    DWORD       dwDialMode;
    DWORD       dwDialExtraPercent;
    DWORD       dwDialExtraSampleSeconds;
    DWORD       dwHangUpExtraPercent;
    DWORD       dwHangUpExtraSampleSeconds;

    //
    // Idle timeout
    //

    DWORD       dwIdleDisconnectSeconds;

    //
    // Entry Type
    //

    DWORD       dwType;

    //
    // EncryptionType
    //

    DWORD       dwEncryptionType;

    //
    // EAP information
    //

    DWORD       dwCustomAuthKey;
    DWORD       dwCustomAuthDataSize;
    LPBYTE      lpbCustomAuthData;

    //
    // Guid of the connection
    //

    GUID        guidId;

    //
    // Vpn Strategy
    //

    DWORD       dwVpnStrategy;

} MPR_INTERFACE_2, *PMPR_INTERFACE_2;

#if(WINVER >= 0x0600)

typedef struct _MPR_INTERFACE_3
{
    IN OUT  WCHAR                   wszInterfaceName[MAX_INTERFACE_NAME_LEN+1];
    OUT     HANDLE                  hInterface;
    IN OUT  BOOL                    fEnabled;
    IN OUT  ROUTER_INTERFACE_TYPE   dwIfType;
    OUT     ROUTER_CONNECTION_STATE dwConnectionState;
    OUT     DWORD                   fUnReachabilityReasons;
    OUT     DWORD                   dwLastError;

    //
    // Demand dial-specific properties
    //

    DWORD       dwfOptions;

    //
    // Location/phone number
    //

    WCHAR       szLocalPhoneNumber[ RAS_MaxPhoneNumber + 1 ];
    PWCHAR      szAlternates;

    //
    // PPP/Ip
    //

    DWORD       ipaddr;
    DWORD       ipaddrDns;
    DWORD       ipaddrDnsAlt;
    DWORD       ipaddrWins;
    DWORD       ipaddrWinsAlt;

   
    
   

    //
    // NetProtocols
    //

    DWORD       dwfNetProtocols;

    //
    // Device
    //

    WCHAR       szDeviceType[ MPR_MaxDeviceType + 1 ];
    WCHAR       szDeviceName[ MPR_MaxDeviceName + 1 ];

    //
    // X.25
    //

    WCHAR       szX25PadType[ MPR_MaxPadType + 1 ];
    WCHAR       szX25Address[ MPR_MaxX25Address + 1 ];
    WCHAR       szX25Facilities[ MPR_MaxFacilities + 1 ];
    WCHAR       szX25UserData[ MPR_MaxUserData + 1 ];
    DWORD       dwChannels;

    //
    // Multilink
    //

    DWORD       dwSubEntries;
    DWORD       dwDialMode;
    DWORD       dwDialExtraPercent;
    DWORD       dwDialExtraSampleSeconds;
    DWORD       dwHangUpExtraPercent;
    DWORD       dwHangUpExtraSampleSeconds;

    //
    // Idle timeout
    //

    DWORD       dwIdleDisconnectSeconds;

    //
    // Entry Type
    //

    DWORD       dwType;

    //
    // EncryptionType
    //

    DWORD       dwEncryptionType;

    //
    // EAP information
    //

    DWORD       dwCustomAuthKey;
    DWORD       dwCustomAuthDataSize;
    LPBYTE      lpbCustomAuthData;

    //
    // Guid of the connection
    //

    GUID        guidId;

    //
    // Vpn Strategy
    //

    DWORD       dwVpnStrategy;

    ULONG          AddressCount;
    IN6_ADDR      ipv6addrDns;
    IN6_ADDR       ipv6addrDnsAlt;
    IN6_ADDR       *ipv6addr; 

} MPR_INTERFACE_3, *PMPR_INTERFACE_3;

#endif /* WINVER >= 0x0600 */

//
// Used to set/get per-link information for multilinked demand
// dial interfaces.
//

typedef struct _MPR_DEVICE_0
{
    //
    // Device
    //

    WCHAR       szDeviceType[ MPR_MaxDeviceType + 1 ];
    WCHAR       szDeviceName[ MPR_MaxDeviceName + 1 ];

}
MPR_DEVICE_0, *PMPR_DEVICE_0;

typedef struct _MPR_DEVICE_1
{
    //
    // Device
    //

    WCHAR       szDeviceType[ MPR_MaxDeviceType + 1 ];
    WCHAR       szDeviceName[ MPR_MaxDeviceName + 1 ];

    //
    // Phone numbers
    //

    WCHAR       szLocalPhoneNumber[ MPR_MaxPhoneNumber + 1 ];
    PWCHAR      szAlternates;

}
MPR_DEVICE_1, *PMPR_DEVICE_1;

//
// Used to get/set extended credentials information such as
// eap credentials info.
//

typedef _Struct_size_bytes_(dwSize + sizeof(MPR_CREDENTIALSEX_0)) struct _MPR_CREDENTIALSEX_0
{
    DWORD  dwSize;
    LPBYTE lpbCredentialsInfo;
}
MPR_CREDENTIALSEX_0, *PMPR_CREDENTIALSEX_0;

typedef _Struct_size_bytes_(dwSize + sizeof(MPR_CREDENTIALSEX_1)) struct _MPR_CREDENTIALSEX_1
{
    DWORD  dwSize;
    LPBYTE lpbCredentialsInfo;
}
MPR_CREDENTIALSEX_1, *PMPR_CREDENTIALSEX_1;

#endif /* WINVER >= 0x0500 */

typedef struct _MPR_TRANSPORT_0
{
    OUT     DWORD                   dwTransportId;
    OUT     HANDLE                  hTransport;
    OUT     WCHAR                   wszTransportName[MAX_TRANSPORT_NAME_LEN+1];

}
MPR_TRANSPORT_0, *PMPR_TRANSPORT_0;

typedef struct _MPR_IFTRANSPORT_0
{
    OUT     DWORD                  dwTransportId;
    OUT     HANDLE                 hIfTransport;
    OUT     WCHAR                  wszIfTransportName[MAX_TRANSPORT_NAME_LEN+1];

}
MPR_IFTRANSPORT_0, *PMPR_IFTRANSPORT_0;

typedef struct _MPR_SERVER_0
{
    OUT BOOL                    fLanOnlyMode;
    OUT DWORD                   dwUpTime;           // In seconds
    OUT DWORD                   dwTotalPorts;
    OUT DWORD                   dwPortsInUse;

}
MPR_SERVER_0, *PMPR_SERVER_0;


#if (WINVER >= 0x501)

//
// values for dwFlags in MPR_SERVER_1. This enables the ports for Ras or Routing.
//

#define MPR_ENABLE_RAS_ON_DEVICE            0x00000001
#define MPR_ENABLE_ROUTING_ON_DEVICE        0x00000002

typedef struct _MPR_SERVER_1
{

    IN OUT DWORD                   dwNumPptpPorts;
    IN OUT DWORD                   dwPptpPortFlags;
    IN OUT DWORD                   dwNumL2tpPorts;
    IN OUT DWORD                   dwL2tpPortFlags;
}
MPR_SERVER_1, *PMPR_SERVER_1;

#endif


#if (WINVER >= 0x600)

//
// MPR_SERVER_2 structure.
//

typedef struct _MPR_SERVER_2
{
    IN OUT DWORD                   dwNumPptpPorts;
    IN OUT DWORD                   dwPptpPortFlags;
    IN OUT DWORD                   dwNumL2tpPorts;
    IN OUT DWORD                   dwL2tpPortFlags;
    IN OUT DWORD                   dwNumSstpPorts;
    IN OUT DWORD                   dwSstpPortFlags;
}
MPR_SERVER_2, *PMPR_SERVER_2;

#endif


//
// Port condition codes
//

typedef enum _RAS_PORT_CONDITION
{
    RAS_PORT_NON_OPERATIONAL,
    RAS_PORT_DISCONNECTED,	
    RAS_PORT_CALLING_BACK,
    RAS_PORT_LISTENING,
    RAS_PORT_AUTHENTICATING,
    RAS_PORT_AUTHENTICATED,	
    RAS_PORT_INITIALIZING

}
RAS_PORT_CONDITION;

//
// Hardware condition codes
//

typedef enum _RAS_HARDWARE_CONDITION
{
    RAS_HARDWARE_OPERATIONAL,
    RAS_HARDWARE_FAILURE

}
RAS_HARDWARE_CONDITION;

typedef struct _RAS_PORT_0
{
    OUT HANDLE                  hPort;
    OUT HANDLE                  hConnection;
    OUT RAS_PORT_CONDITION      dwPortCondition;
    OUT DWORD                   dwTotalNumberOfCalls;
    OUT DWORD                   dwConnectDuration;      // In seconds
    OUT WCHAR                   wszPortName[ MAX_PORT_NAME + 1 ];
    OUT WCHAR                   wszMediaName[ MAX_MEDIA_NAME + 1 ];
    OUT WCHAR                   wszDeviceName[ MAX_DEVICE_NAME + 1 ];
    OUT WCHAR                   wszDeviceType[ MAX_DEVICETYPE_NAME + 1 ];

}
RAS_PORT_0, *PRAS_PORT_0;

typedef struct _RAS_PORT_1
{
    OUT HANDLE                  hPort;
    OUT HANDLE                  hConnection;
    OUT RAS_HARDWARE_CONDITION  dwHardwareCondition;
    OUT DWORD                   dwLineSpeed;            // in bits/second
    OUT DWORD                   dwBytesXmited;
    OUT DWORD                   dwBytesRcved;
    OUT DWORD                   dwFramesXmited;
    OUT DWORD                   dwFramesRcved;
    OUT DWORD                   dwCrcErr;
    OUT DWORD                   dwTimeoutErr;
    OUT DWORD                   dwAlignmentErr;
    OUT DWORD                   dwHardwareOverrunErr;
    OUT DWORD                   dwFramingErr;
    OUT DWORD                   dwBufferOverrunErr;
    OUT DWORD                   dwCompressionRatioIn;
    OUT DWORD                   dwCompressionRatioOut;
}
RAS_PORT_1, *PRAS_PORT_1;

typedef struct _RAS_PORT_2
{

    OUT HANDLE                  hPort;
    OUT HANDLE                  hConnection;
    OUT DWORD                   dwConn_State;
    OUT WCHAR                   wszPortName[ MAX_PORT_NAME + 1 ];
    OUT WCHAR                   wszMediaName[ MAX_MEDIA_NAME + 1 ];
    OUT WCHAR                   wszDeviceName[ MAX_DEVICE_NAME + 1 ];
    OUT WCHAR                   wszDeviceType[ MAX_DEVICETYPE_NAME + 1 ];
    OUT RAS_HARDWARE_CONDITION  dwHardwareCondition;
    OUT DWORD                   dwLineSpeed;            // in bits/second
    OUT DWORD                   dwCrcErr;
    OUT DWORD                   dwSerialOverRunErrs;
    OUT DWORD                   dwTimeoutErr;
    OUT DWORD                   dwAlignmentErr;
    OUT DWORD                   dwHardwareOverrunErr;
    OUT DWORD                   dwFramingErr;
    OUT DWORD                   dwBufferOverrunErr;
    OUT DWORD                   dwCompressionRatioIn;
    OUT DWORD                   dwCompressionRatioOut;
    OUT DWORD                   dwTotalErrors;
    OUT ULONGLONG               ullBytesXmited;
    OUT ULONGLONG               ullBytesRcved;
    OUT ULONGLONG               ullFramesXmited;        // Making 64 bit for future
    OUT ULONGLONG               ullFramesRcved;         // Currently using 32 bits only
    OUT ULONGLONG               ullBytesTxUncompressed;
    OUT ULONGLONG               ullBytesTxCompressed;
    OUT ULONGLONG               ullBytesRcvUncompressed;
    OUT ULONGLONG               ullBytesRcvCompressed;
}
RAS_PORT_2, *PRAS_PORT_2;

//
// Maximum length of address string, e.g. "255.255.255.255" for IP.
//

#define IPADDRESSLEN  15
#define IPXADDRESSLEN 22
#define ATADDRESSLEN  32
#define MAXIPADRESSLEN 64


typedef struct _PPP_NBFCP_INFO
{
    OUT DWORD           dwError;
    OUT WCHAR           wszWksta[ NETBIOS_NAME_LEN + 1 ];
}
PPP_NBFCP_INFO;

typedef struct _PPP_IPCP_INFO
{
    OUT DWORD           dwError;
    OUT WCHAR           wszAddress[ IPADDRESSLEN + 1 ];
    OUT WCHAR           wszRemoteAddress[ IPADDRESSLEN + 1 ];
}
PPP_IPCP_INFO;

//
// PPP_IPCP_INFO2 dwOptions values.
//

#define PPP_IPCP_VJ             0x00000001

typedef struct _PPP_IPCP_INFO2
{
    OUT DWORD           dwError;
    OUT WCHAR           wszAddress[ IPADDRESSLEN + 1 ];
    OUT WCHAR           wszRemoteAddress[ IPADDRESSLEN + 1 ];
    OUT DWORD           dwOptions;
    OUT DWORD           dwRemoteOptions;
}
PPP_IPCP_INFO2;

typedef struct _PPP_IPXCP_INFO
{
    OUT DWORD           dwError;
    OUT WCHAR           wszAddress[ IPXADDRESSLEN + 1 ];
}
PPP_IPXCP_INFO;

typedef struct _PPP_ATCP_INFO
{
    OUT DWORD           dwError;
    OUT WCHAR           wszAddress[ ATADDRESSLEN + 1 ];
}
PPP_ATCP_INFO;

#if(WINVER >= 0x0600)

typedef struct _PPP_IPV6_CP_INFO
{
    OUT DWORD           dwVersion;
    OUT DWORD           dwSize;
    OUT DWORD           dwError;
    OUT BYTE            bInterfaceIdentifier[8];
    OUT BYTE            bRemoteInterfaceIdentifier[8];
    OUT DWORD           dwOptions;
    OUT DWORD           dwRemoteOptions;
    OUT BYTE		bPrefix[8];
    OUT DWORD		dwPrefixLength;
}
PPP_IPV6_CP_INFO;

#endif /* WINVER >= 0x0600 */

typedef struct _PPP_INFO
{
    OUT PPP_NBFCP_INFO  nbf;
    OUT PPP_IPCP_INFO   ip;
    OUT PPP_IPXCP_INFO  ipx;
    OUT PPP_ATCP_INFO   at;

} PPP_INFO;

#if(WINVER >= 0x0500)

//
// PPP_CCP dwCompressionAlgorithm values.
//

#define RASCCPCA_MPPC         0x00000006
#define RASCCPCA_STAC         0x00000005

//
// PPP_CCP dwOptions values.
//

#define PPP_CCP_COMPRESSION         0x00000001
#define PPP_CCP_ENCRYPTION40BITOLD  0x00000010
#define PPP_CCP_ENCRYPTION40BIT     0x00000020
#define PPP_CCP_ENCRYPTION128BIT    0x00000040
#define PPP_CCP_ENCRYPTION56BIT     0x00000080
#define PPP_CCP_HISTORYLESS         0x01000000

typedef struct _PPP_CCP_INFO
{
    OUT DWORD           dwError;
    OUT DWORD           dwCompressionAlgorithm;
    OUT DWORD           dwOptions;
    OUT DWORD           dwRemoteCompressionAlgorithm;
    OUT DWORD           dwRemoteOptions;
}
PPP_CCP_INFO;

//
// PPP_LCP dwAuthenticatonProtocol values.
//

#define PPP_LCP_PAP          0xC023
#define PPP_LCP_SPAP         0xC027
#define PPP_LCP_CHAP         0xC223
#define PPP_LCP_EAP          0xC227

//
// PPP_LCP dwAuthenticatonData values.
//

#define PPP_LCP_CHAP_MD5     0x05
#define PPP_LCP_CHAP_MS      0x80
#define PPP_LCP_CHAP_MSV2    0x81

//
// PPP_LCP dwOption values
//

#define PPP_LCP_MULTILINK_FRAMING   0x00000001
#define PPP_LCP_PFC                 0x00000002
#define PPP_LCP_ACFC                0x00000004
#define PPP_LCP_SSHF                0x00000008
#define PPP_LCP_DES_56              0x00000010
#define PPP_LCP_3_DES               0x00000020

#if(WINVER >= 0x0600)
#define PPP_LCP_AES_128             0x00000040
#define PPP_LCP_AES_256             0x00000080
#endif /* WINVER >= 0x0600 */

#if(WINVER >= 0x0603)
#define PPP_LCP_AES_192             0x00000100
#define PPP_LCP_GCM_AES_128         0x00000200
#define PPP_LCP_GCM_AES_192         0x00000400
#define PPP_LCP_GCM_AES_256         0x00000800
#endif /* WINVER >= 0x0603 */

typedef struct _PPP_LCP_INFO
{
    OUT DWORD dwError;
    OUT DWORD dwAuthenticationProtocol;
    OUT DWORD dwAuthenticationData;
    OUT DWORD dwRemoteAuthenticationProtocol;
    OUT DWORD dwRemoteAuthenticationData;
    OUT DWORD dwTerminateReason;
    OUT DWORD dwRemoteTerminateReason;
    OUT DWORD dwOptions;
    OUT DWORD dwRemoteOptions;
    OUT DWORD dwEapTypeId;
    OUT DWORD dwRemoteEapTypeId;
}
PPP_LCP_INFO;

typedef struct _PPP_INFO_2
{
    OUT PPP_NBFCP_INFO  nbf;
    OUT PPP_IPCP_INFO2  ip;
    OUT PPP_IPXCP_INFO  ipx;
    OUT PPP_ATCP_INFO   at;
    OUT PPP_CCP_INFO    ccp;
    OUT PPP_LCP_INFO    lcp;
}
PPP_INFO_2;
#endif /* WINVER >= 0x0500 */

#if(WINVER >= 0x0600)
typedef struct _PPP_INFO_3
{
    OUT PPP_NBFCP_INFO  nbf;
    OUT PPP_IPCP_INFO2  ip;
    OUT PPP_IPV6_CP_INFO  ipv6;
    OUT PPP_CCP_INFO    ccp;
    OUT PPP_LCP_INFO    lcp;
}
PPP_INFO_3;

#endif /* WINVER >= 0x0600 */

//
// Possible bits set in Connection Flags field
//

#define RAS_FLAGS_PPP_CONNECTION        0x00000001
#define RAS_FLAGS_MESSENGER_PRESENT     0x00000002

#if(WINVER < 0x0501)
#define RAS_FLAGS_RAS_CONNECTION        0x00000004
#endif

#define RAS_FLAGS_QUARANTINE_PRESENT    0x00000008

#if(WINVER >= 0x0601)
#define RAS_FLAGS_DORMANT               0x00000020
#endif

typedef struct _RAS_CONNECTION_0
{
    OUT HANDLE                  hConnection;
    OUT HANDLE                  hInterface;
    OUT DWORD                   dwConnectDuration;      // In seconds
    OUT ROUTER_INTERFACE_TYPE   dwInterfaceType;
    OUT DWORD                   dwConnectionFlags;
    OUT WCHAR                   wszInterfaceName[ MAX_INTERFACE_NAME_LEN + 1 ];
    OUT WCHAR                   wszUserName[ UNLEN + 1 ];
    OUT WCHAR                   wszLogonDomain[ DNLEN + 1 ];
    OUT WCHAR                   wszRemoteComputer[ NETBIOS_NAME_LEN + 1 ];
}
RAS_CONNECTION_0, *PRAS_CONNECTION_0;

typedef struct _RAS_CONNECTION_1
{
    OUT HANDLE                  hConnection;
    OUT HANDLE                  hInterface;
    OUT PPP_INFO                PppInfo;
    OUT DWORD                   dwBytesXmited;
    OUT DWORD                   dwBytesRcved;
    OUT DWORD                   dwFramesXmited;
    OUT DWORD                   dwFramesRcved;
    OUT DWORD                   dwCrcErr;
    OUT DWORD                   dwTimeoutErr;
    OUT DWORD                   dwAlignmentErr;
    OUT DWORD                   dwHardwareOverrunErr;
    OUT DWORD                   dwFramingErr;
    OUT DWORD                   dwBufferOverrunErr;
    OUT DWORD                   dwCompressionRatioIn;
    OUT DWORD                   dwCompressionRatioOut;
}
RAS_CONNECTION_1, *PRAS_CONNECTION_1;

#if(WINVER >= 0x0500)

typedef struct _RAS_CONNECTION_2
{
    OUT HANDLE                  hConnection;
    OUT WCHAR                   wszUserName[ UNLEN + 1 ];
    OUT ROUTER_INTERFACE_TYPE   dwInterfaceType;
    OUT GUID                    guid;
    OUT PPP_INFO_2              PppInfo2;
}
RAS_CONNECTION_2, *PRAS_CONNECTION_2;

//indicates quarantine state of a client connection
typedef enum _RAS_QUARANTINE_STATE
{
      RAS_QUAR_STATE_NORMAL     = 0,
      RAS_QUAR_STATE_QUARANTINE = 1,
      RAS_QUAR_STATE_PROBATION  = 2,
      RAS_QUAR_STATE_NOT_CAPABLE    = 3
}RAS_QUARANTINE_STATE;
#endif /* WINVER >= 0x0500 */

#if(WINVER >= 0x0600)
typedef struct _RAS_CONNECTION_3
{
       OUT DWORD     dwVersion;
       OUT DWORD     dwSize;
	OUT HANDLE     hConnection;   //connection handle
	WCHAR                   wszUserName[ UNLEN + 1 ];
       ROUTER_INTERFACE_TYPE   dwInterfaceType;
       GUID                    guid;
       PPP_INFO_3           PppInfo3;
	OUT RAS_QUARANTINE_STATE    rasQuarState;  //Quarantine state of the connection
	OUT FILETIME                timer;         //Probation timer in UTC
}RAS_CONNECTION_3, *PRAS_CONNECTION_3;

#endif /* WINVER >= 0x0600 */

//
// Structures used by the MPRADMIN USER APIs. Use level 0 to get/set this
// structure.
//
//
// Bits indicating user's Remote Access privileges and mask to isolate
// call back privilege.
//
// Note: Bit 0 MUST represent NoCallback due to a quirk of the "userparms"
//       storage method.  When a new LAN Manager user is created, bit 0 of the
//       userparms field is set to 1 and all other bits are 0.  These bits are
//       arranged so this "no Dial-In info" state maps to the "default Dial-In
//       privilege" state.

#define RASPRIV_NoCallback        0x01
#define RASPRIV_AdminSetCallback  0x02
#define RASPRIV_CallerSetCallback 0x04
#define RASPRIV_DialinPrivilege   0x08

// 
// The following are flags for the bfPrivilege2 member of RAS_USER_1
// structure
//
#define RASPRIV2_DialinPolicy      0x1  

#define RASPRIV_CallbackType (RASPRIV_AdminSetCallback \
                              | RASPRIV_CallerSetCallback \
                              | RASPRIV_NoCallback)

typedef struct _RAS_USER_0
{
    OUT BYTE                    bfPrivilege;
    OUT WCHAR                   wszPhoneNumber[ MAX_PHONE_NUMBER_LEN + 1];
}
RAS_USER_0, *PRAS_USER_0;

typedef struct _RAS_USER_1
{
    OUT BYTE                    bfPrivilege;
    OUT WCHAR                   wszPhoneNumber[ MAX_PHONE_NUMBER_LEN + 1];
    OUT BYTE                    bfPrivilege2;
} 
RAS_USER_1, *PRAS_USER_1;

#if(WINVER >= 0x0600)
typedef struct _MPR_FILTER_0
{
    IN BOOL fEnable;
}
MPR_FILTER_0, *PMPR_FILTER_0;

#endif /* WINVER >= 0x0600 */

//
// Used as RPC binding handle to server
//

typedef HANDLE RAS_SERVER_HANDLE;
typedef HANDLE MPR_SERVER_HANDLE;
typedef HANDLE MIB_SERVER_HANDLE;

#if(WINVER >= 0x0601)

typedef struct _MPRAPI_OBJECT_HEADER {
    UCHAR                       revision;
    UCHAR                       type;
    USHORT                      size;
} MPRAPI_OBJECT_HEADER, *PMPRAPI_OBJECT_HEADER ;


typedef enum _MPRAPI_OBJECT_TYPE
{
    MPRAPI_OBJECT_TYPE_RAS_CONNECTION_OBJECT               = 1,
    MPRAPI_OBJECT_TYPE_MPR_SERVER_OBJECT                   = 2,
    MPRAPI_OBJECT_TYPE_MPR_SERVER_SET_CONFIG_OBJECT        = 3,
    MPRAPI_OBJECT_TYPE_AUTH_VALIDATION_OBJECT              = 4,
    MPRAPI_OBJECT_TYPE_UPDATE_CONNECTION_OBJECT            = 5,
    MPRAPI_OBJECT_TYPE_IF_CUSTOM_CONFIG_OBJECT            = 6
}MPRAPI_OBJECT_TYPE, *PMPRAPI_OBJECT_TYPE;


typedef struct _PPP_PROJECTION_INFO {

    // IPv4 Projection Parameters
    DWORD                       dwIPv4NegotiationError;
    WCHAR                       wszAddress[IPADDRESSLEN + 1];
    WCHAR                       wszRemoteAddress[IPADDRESSLEN + 1];
    DWORD                       dwIPv4Options;
    DWORD                       dwIPv4RemoteOptions; 
    ULONG64                     IPv4SubInterfaceIndex;

    // IPv6 Projection Parameters
    DWORD                       dwIPv6NegotiationError;
    BYTE                        bInterfaceIdentifier[8];
    BYTE                        bRemoteInterfaceIdentifier[8];
    BYTE                        bPrefix[8];
    DWORD                       dwPrefixLength; 
    ULONG64                     IPv6SubInterfaceIndex;
    // LCP Options
    DWORD                       dwLcpError;
    DWORD                       dwAuthenticationProtocol;  
    DWORD                       dwAuthenticationData;  
    DWORD                       dwRemoteAuthenticationProtocol;  
    DWORD                       dwRemoteAuthenticationData;  
    DWORD                       dwLcpTerminateReason;  
    DWORD                       dwLcpRemoteTerminateReason;  
    DWORD                       dwLcpOptions;  
    DWORD                       dwLcpRemoteOptions;  
    DWORD                       dwEapTypeId;  
    DWORD                       dwRemoteEapTypeId;
    
    // CCP options:
    DWORD                       dwCcpError;  
    DWORD                       dwCompressionAlgorithm;  
    DWORD                       dwCcpOptions;  
    DWORD                       dwRemoteCompressionAlgorithm;  
    DWORD                       dwCcpRemoteOptions;

}PPP_PROJECTION_INFO, *PPPP_PROJECTION_INFO;

typedef struct _PPP_PROJECTION_INFO2 {

    // IPv4 Projection Parameters
    DWORD                       dwIPv4NegotiationError;
    WCHAR                       wszAddress[IPADDRESSLEN + 1];
    WCHAR                       wszRemoteAddress[IPADDRESSLEN + 1];
    DWORD                       dwIPv4Options;
    DWORD                       dwIPv4RemoteOptions; 
    ULONG64                     IPv4SubInterfaceIndex;

    // IPv6 Projection Parameters
    DWORD                       dwIPv6NegotiationError;
    BYTE                        bInterfaceIdentifier[8];
    BYTE                        bRemoteInterfaceIdentifier[8];
    BYTE                        bPrefix[8];
    DWORD                       dwPrefixLength; 
    ULONG64                     IPv6SubInterfaceIndex;
    // LCP Options
    DWORD                       dwLcpError;
    DWORD                       dwAuthenticationProtocol;  
    DWORD                       dwAuthenticationData;  
    DWORD                       dwRemoteAuthenticationProtocol;  
    DWORD                       dwRemoteAuthenticationData;  
    DWORD                       dwLcpTerminateReason;  
    DWORD                       dwLcpRemoteTerminateReason;  
    DWORD                       dwLcpOptions;  
    DWORD                       dwLcpRemoteOptions;  
    DWORD                       dwEapTypeId;  
    DWORD                       dwEmbeddedEAPTypeId;
    DWORD                       dwRemoteEapTypeId;
    
    // CCP options:
    DWORD                       dwCcpError;  
    DWORD                       dwCompressionAlgorithm;  
    DWORD                       dwCcpOptions;  
    DWORD                       dwRemoteCompressionAlgorithm;  
    DWORD                       dwCcpRemoteOptions;

}PPP_PROJECTION_INFO2, *PPPP_PROJECTION_INFO2;

#define MPRAPI_IKEV2_AUTH_USING_CERT                1
#define MPRAPI_IKEV2_AUTH_USING_EAP                 2

typedef struct _IKEV2_PROJECTION_INFO {

    DWORD                       dwIPv4NegotiationError;
    WCHAR                       wszAddress[IPADDRESSLEN + 1];
    WCHAR                       wszRemoteAddress[IPADDRESSLEN + 1];
    ULONG64                     IPv4SubInterfaceIndex;

    DWORD                       dwIPv6NegotiationError;
    BYTE                        bInterfaceIdentifier[8];
    BYTE                        bRemoteInterfaceIdentifier[8];
    BYTE                        bPrefix[8];
    DWORD                       dwPrefixLength; 
    ULONG64                     IPv6SubInterfaceIndex;

    DWORD                       dwOptions;

    DWORD                       dwAuthenticationProtocol;
    DWORD                       dwEapTypeId;
    DWORD                       dwCompressionAlgorithm;
    DWORD		                dwEncryptionMethod;
    
}IKEV2_PROJECTION_INFO, *PIKEV2_PROJECTION_INFO;

typedef struct _IKEV2_PROJECTION_INFO2 {

    DWORD                       dwIPv4NegotiationError;
    WCHAR                       wszAddress[IPADDRESSLEN + 1];
    WCHAR                       wszRemoteAddress[IPADDRESSLEN + 1];
    ULONG64                     IPv4SubInterfaceIndex;

    DWORD                       dwIPv6NegotiationError;
    BYTE                        bInterfaceIdentifier[8];
    BYTE                        bRemoteInterfaceIdentifier[8];
    BYTE                        bPrefix[8];
    DWORD                       dwPrefixLength; 
    ULONG64                     IPv6SubInterfaceIndex;

    DWORD                       dwOptions;

    DWORD                       dwAuthenticationProtocol;
    DWORD                       dwEapTypeId;
    DWORD                       dwEmbeddedEAPTypeId;
    DWORD                       dwCompressionAlgorithm;
    DWORD		                dwEncryptionMethod;
    
}IKEV2_PROJECTION_INFO2, *PIKEV2_PROJECTION_INFO2;

#define    MPRAPI_PPP_PROJECTION_INFO_TYPE          1
#define    MPRAPI_IKEV2_PROJECTION_INFO_TYPE        2

typedef struct _PROJECTION_INFO { 
    UCHAR     projectionInfoType;
    // Based on the connectionFlags, it should use appropriate projection info 
    union {
        PPP_PROJECTION_INFO     PppProjectionInfo;
        IKEV2_PROJECTION_INFO   Ikev2ProjectionInfo;
    };
}PROJECTION_INFO, *PPROJECTION_INFO;

typedef struct _PROJECTION_INFO2 { 
    UCHAR     projectionInfoType;
    // Based on the connectionFlags, it should use appropriate projection info 
    union {
        PPP_PROJECTION_INFO2     PppProjectionInfo;
        IKEV2_PROJECTION_INFO2   Ikev2ProjectionInfo;
    };
}PROJECTION_INFO2, *PPROJECTION_INFO2;

#define MPRAPI_RAS_CONNECTION_OBJECT_REVISION_1     0x1 

typedef struct _RAS_CONNECTION_EX {
    MPRAPI_OBJECT_HEADER        Header;
    DWORD                       dwConnectDuration;
    ROUTER_INTERFACE_TYPE       dwInterfaceType;
    DWORD                       dwConnectionFlags;
    WCHAR                       wszInterfaceName[MAX_INTERFACE_NAME_LEN + 1];
    WCHAR                       wszUserName[UNLEN + 1];
    WCHAR                       wszLogonDomain[DNLEN + 1];
    WCHAR                       wszRemoteComputer[NETBIOS_NAME_LEN + 1]; 
    GUID                        guid;
    RAS_QUARANTINE_STATE        rasQuarState;
    FILETIME                    probationTime; 

    // Statistics: 
    DWORD                       dwBytesXmited;
    DWORD                       dwBytesRcved;
    DWORD                       dwFramesXmited;
    DWORD                       dwFramesRcved;
    DWORD                       dwCrcErr;
    DWORD                       dwTimeoutErr;
    DWORD                       dwAlignmentErr;
    DWORD                       dwHardwareOverrunErr;
    DWORD                       dwFramingErr;
    DWORD                       dwBufferOverrunErr;
    DWORD                       dwCompressionRatioIn;
    DWORD                       dwCompressionRatioOut; 

    // Currently valid only for IKEV2:
    DWORD                       dwNumSwitchOvers;

    // Endpoint Information:
    WCHAR                       wszRemoteEndpointAddress[MAXIPADRESSLEN+1];
    WCHAR                       wszLocalEndpointAddress[MAXIPADRESSLEN+1];


    //Projection result: 
    PROJECTION_INFO             ProjectionInfo;

    HANDLE                      hConnection;
    HANDLE                      hInterface;

}RAS_CONNECTION_EX, *PRAS_CONNECTION_EX;

#if (NTDDI_VERSION >= NTDDI_WIN8)
typedef struct _RAS_CONNECTION_4 {
    DWORD                       dwConnectDuration;
    ROUTER_INTERFACE_TYPE       dwInterfaceType;
    DWORD                       dwConnectionFlags;
    WCHAR                       wszInterfaceName[MAX_INTERFACE_NAME_LEN + 1];
    WCHAR                       wszUserName[UNLEN + 1];
    WCHAR                       wszLogonDomain[DNLEN + 1];
    WCHAR                       wszRemoteComputer[NETBIOS_NAME_LEN + 1]; 
    GUID                        guid;
    RAS_QUARANTINE_STATE        rasQuarState;
    FILETIME                    probationTime; 
    FILETIME                    connectionStartTime; 

    // Statistics: 
    ULONGLONG                 ullBytesXmited;
    ULONGLONG                 ullBytesRcved;
    DWORD                       dwFramesXmited;
    DWORD                       dwFramesRcved;
    DWORD                       dwCrcErr;
    DWORD                       dwTimeoutErr;
    DWORD                       dwAlignmentErr;
    DWORD                       dwHardwareOverrunErr;
    DWORD                       dwFramingErr;
    DWORD                       dwBufferOverrunErr;
    DWORD                       dwCompressionRatioIn;
    DWORD                       dwCompressionRatioOut; 

    // Currently valid only for IKEV2:
    DWORD                       dwNumSwitchOvers;

    // Endpoint Information:
    WCHAR                       wszRemoteEndpointAddress[MAXIPADRESSLEN+1];
    WCHAR                       wszLocalEndpointAddress[MAXIPADRESSLEN+1];


    //Projection result: 
    PROJECTION_INFO2             ProjectionInfo;

    HANDLE                      hConnection;
    HANDLE                      hInterface;

    // VPN Device type 
    DWORD                      dwDeviceType;

}RAS_CONNECTION_4, *PRAS_CONNECTION_4;
#endif // NTDDI_VERSION >= NTDDI_WIN8

#if (NTDDI_VERSION >= NTDDI_WIN8)
// ------------ IKEv2 custom policy object (supported Win8 onwards)

typedef struct _ROUTER_CUSTOM_IKEv2_POLICY0
{
    //    Integrity method plumbed in IKE policy
    DWORD dwIntegrityMethod;

     //    Encryption Method  Plumbed in IKE  policy
   DWORD dwEncryptionMethod;

    //    Cipher plumbed in Ipsec policy
    DWORD dwCipherTransformConstant;

    //    AH Auth transform plumbed in Ipsec policy
    DWORD dwAuthTransformConstant;

    //    PFS Group plumbed in Ipsec policy
    DWORD dwPfsGroup;

    //    DH Group Plumbed in IKE policy
    DWORD dwDhGroup; 
}ROUTER_CUSTOM_IKEv2_POLICY0, *PROUTER_CUSTOM_IKEv2_POLICY0, ROUTER_CUSTOM_L2TP_POLICY0, *PROUTER_CUSTOM_L2TP_POLICY0;

typedef struct _ROUTER_IKEv2_IF_CUSTOM_CONFIG0
{
    //    Lifetime of a security association (SA) in seconds, 
    //    after which the SA is no longer valid [RFC 4306].
    DWORD dwSaLifeTime;

    //    Number of Kilobytes that are allowed to transfer using a SA. 
    //    After that the SA will be renegotiated [RFC 4306].
    DWORD dwSaDataSize;

    //    SubjecName of the certificate to be used in default store 
    //    for machine certificate authentication. 
   CERT_NAME_BLOB certificateName;   

    //    Custom IKEv2 Policy 
    ROUTER_CUSTOM_IKEv2_POLICY0* customPolicy; 
}ROUTER_IKEv2_IF_CUSTOM_CONFIG0, *PROUTER_IKEv2_IF_CUSTOM_CONFIG0;

#define MPRAPI_MPR_IF_CUSTOM_CONFIG_OBJECT_REVISION_1      0x1 
#define MPRAPI_IF_CUSTOM_CONFIG_FOR_IKEV2                            0x1

typedef struct _MPR_IF_CUSTOMINFOEX0 {
    MPRAPI_OBJECT_HEADER        Header;
    DWORD                                  dwFlags;
    ROUTER_IKEv2_IF_CUSTOM_CONFIG0 customIkev2Config;
}MPR_IF_CUSTOMINFOEX0, *PMPR_IF_CUSTOMINFOEX0;

#endif // NTDDI_VERSION >= NTDDI_WIN8

#if (NTDDI_VERSION >= NTDDI_WINBLUE)
typedef  struct  _MPR_CERT_EKU
{
    DWORD dwSize;
    BOOL  IsEKUOID;
    WCHAR *pwszEKU;
    
}MPR_CERT_EKU,*PMPR_CERT_EKU;
#endif

typedef struct _VPN_TS_IP_ADDRESS
{
    USHORT Type;
    union
    {
        IN_ADDR v4;           // The v4 address, if the Type == AF_INET .
        IN6_ADDR v6;           // The v6 address, if the Type == AF_INET6.
    };
}VPN_TS_IP_ADDRESS, *PVPN_TS_IP_ADDRESS;

typedef enum _MPR_VPN_TS_TYPE
{
    MPR_VPN_TS_IPv4_ADDR_RANGE = 7,
    MPR_VPN_TS_IPv6_ADDR_RANGE = 8
}MPR_VPN_TS_TYPE;

typedef struct _MPR_VPN_SELECTOR
{
    MPR_VPN_TS_TYPE                            type;
    UINT8                                      protocolId;
    UINT16                                     portStart;
    UINT16                                     portEnd;
    UINT16                                     tsPayloadId;
    VPN_TS_IP_ADDRESS                          addrStart;
    VPN_TS_IP_ADDRESS                          addrEnd;
}MPR_VPN_TRAFFIC_SELECTOR, *PMPR_VPN_TRAFFIC_SELECTOR;

typedef struct _MPR_VPN_TRAFFIC_SELECTORS
{
    DWORD                                                                    numTsi;
    DWORD                                                                    numTsr;
    PMPR_VPN_TRAFFIC_SELECTOR                                                tsI;
    PMPR_VPN_TRAFFIC_SELECTOR                                                tsR;
}MPR_VPN_TRAFFIC_SELECTORS, *PMPR_VPN_TRAFFIC_SELECTORS;

#if (NTDDI_VERSION >= NTDDI_WINTHRESHOLD)
typedef struct _ROUTER_IKEv2_IF_CUSTOM_CONFIG2
{
    //    Lifetime of a security association (SA) in seconds, 
    //    after which the SA is no longer valid [RFC 4306].
    DWORD dwSaLifeTime;

    //    Number of Kilobytes that are allowed to transfer using a SA. 
    //    After that the SA will be renegotiated [RFC 4306].
    DWORD dwSaDataSize;

    //    SubjecName of the certificate to be used in default store 
    //    for machine certificate authentication. 
   CERT_NAME_BLOB certificateName;   

    //    Custom IKEv2 Policy 
    ROUTER_CUSTOM_IKEv2_POLICY0* customPolicy; 
    
    //    Thumbprint of the certificate to be used in default store 
    //    for machine certificate authentication. 
    CRYPT_HASH_BLOB certificateHash;

    //    Lifetime of main mode security association (SA) in seconds, 
    //    after which the MM SA is no longer valid.
    DWORD dwMmSaLifeTime;

    //    Traffic Selectors For IPSec SA Negotiation
    //    TrafficSelector Negotitated Traffic is allowed to go through and received
    MPR_VPN_TRAFFIC_SELECTORS vpnTrafficSelectors;
}ROUTER_IKEv2_IF_CUSTOM_CONFIG2, *PROUTER_IKEv2_IF_CUSTOM_CONFIG2;

#define MPRAPI_MPR_IF_CUSTOM_CONFIG_OBJECT_REVISION_3      0x3

typedef struct _MPR_IF_CUSTOMINFOEX2 {
    MPRAPI_OBJECT_HEADER        Header;
    DWORD                                  dwFlags;
    ROUTER_IKEv2_IF_CUSTOM_CONFIG2 customIkev2Config;
}MPR_IF_CUSTOMINFOEX2, *PMPR_IF_CUSTOMINFOEX2;

typedef struct _IKEV2_TUNNEL_CONFIG_PARAMS4 {
    DWORD                       dwIdleTimeout;
    DWORD                       dwNetworkBlackoutTime;
    DWORD                       dwSaLifeTime;
    DWORD                       dwSaDataSizeForRenegotiation;
    DWORD                       dwConfigOptions;
    DWORD                       dwTotalCertificates;
    CERT_NAME_BLOB*      certificateNames;
    
    //    SubjecName of the certificate to be used in default store 
    //    for machine certificate authentication. 
    CERT_NAME_BLOB      machineCertificateName;
    
    // encryption type to be used for IKEv2
    DWORD                       dwEncryptionType;

    PROUTER_CUSTOM_IKEv2_POLICY0 customPolicy;

    // Number of Ekus specified
    DWORD          dwTotalEkus;
    PMPR_CERT_EKU  certificateEKUs;

    //   Thumbprint of the certificate to be used in default store 
    //   for machine certificate authentication. 
    CRYPT_HASH_BLOB     machineCertificateHash;

    DWORD                       dwMmSaLifeTime;

}IKEV2_TUNNEL_CONFIG_PARAMS4, *PIKEV2_TUNNEL_CONFIG_PARAMS4;
#endif

#if (NTDDI_VERSION >= NTDDI_WINBLUE)
typedef struct _ROUTER_IKEv2_IF_CUSTOM_CONFIG1
{
    //    Lifetime of a security association (SA) in seconds, 
    //    after which the SA is no longer valid [RFC 4306].
    DWORD dwSaLifeTime;

    //    Number of Kilobytes that are allowed to transfer using a SA. 
    //    After that the SA will be renegotiated [RFC 4306].
    DWORD dwSaDataSize;

    //    SubjecName of the certificate to be used in default store 
    //    for machine certificate authentication. 
   CERT_NAME_BLOB certificateName;   

    //    Custom IKEv2 Policy 
    ROUTER_CUSTOM_IKEv2_POLICY0* customPolicy; 
    
    //    Thumbprint of the certificate to be used in default store 
    //    for machine certificate authentication. 
    CRYPT_HASH_BLOB certificateHash;
}ROUTER_IKEv2_IF_CUSTOM_CONFIG1, *PROUTER_IKEv2_IF_CUSTOM_CONFIG1;

#define MPRAPI_MPR_IF_CUSTOM_CONFIG_OBJECT_REVISION_2      0x2

typedef struct _MPR_IF_CUSTOMINFOEX1 {
    MPRAPI_OBJECT_HEADER        Header;
    DWORD                                  dwFlags;
    ROUTER_IKEv2_IF_CUSTOM_CONFIG1 customIkev2Config;
}MPR_IF_CUSTOMINFOEX1, *PMPR_IF_CUSTOMINFOEX1;

typedef struct _IKEV2_TUNNEL_CONFIG_PARAMS3 {
    DWORD                       dwIdleTimeout;
    DWORD                       dwNetworkBlackoutTime;
    DWORD                       dwSaLifeTime;
    DWORD                       dwSaDataSizeForRenegotiation;
    DWORD                       dwConfigOptions;
    DWORD                       dwTotalCertificates;
    CERT_NAME_BLOB*      certificateNames;
    
    //    SubjecName of the certificate to be used in default store 
    //    for machine certificate authentication. 
    CERT_NAME_BLOB      machineCertificateName;
    
    // encryption type to be used for IKEv2
    DWORD                       dwEncryptionType;

    PROUTER_CUSTOM_IKEv2_POLICY0 customPolicy;

    // Number of Ekus specified
    DWORD          dwTotalEkus;
    PMPR_CERT_EKU  certificateEKUs;

    //   Thumbprint of the certificate to be used in default store 
    //   for machine certificate authentication. 
    CRYPT_HASH_BLOB     machineCertificateHash;

}IKEV2_TUNNEL_CONFIG_PARAMS3, *PIKEV2_TUNNEL_CONFIG_PARAMS3;
#endif // NTDDI_VERSION >= NTDDI_WINBLUE

#if (NTDDI_VERSION >= NTDDI_WIN8)
typedef struct _IKEV2_TUNNEL_CONFIG_PARAMS2 {
    DWORD                       dwIdleTimeout;
    DWORD                       dwNetworkBlackoutTime;
    DWORD                       dwSaLifeTime;
    DWORD                       dwSaDataSizeForRenegotiation;
    DWORD                       dwConfigOptions;
    DWORD                       dwTotalCertificates;
    CERT_NAME_BLOB*      certificateNames;
    
    //    SubjecName of the certificate to be used in default store 
    //    for machine certificate authentication. 
    CERT_NAME_BLOB      machineCertificateName;
    
    // encryption type to be used for IKEv2
    DWORD                       dwEncryptionType;

    PROUTER_CUSTOM_IKEv2_POLICY0 customPolicy;
}IKEV2_TUNNEL_CONFIG_PARAMS2, *PIKEV2_TUNNEL_CONFIG_PARAMS2;
#else
typedef struct _IKEV2_TUNNEL_CONFIG_PARAMS1 {
    DWORD                       dwIdleTimeout;
    DWORD                       dwNetworkBlackoutTime;
    DWORD                       dwSaLifeTime;
    DWORD                       dwSaDataSizeForRenegotiation;
    DWORD                       dwConfigOptions;
    DWORD                       dwTotalCertificates;
    CERT_NAME_BLOB *     certificateNames;
}IKEV2_TUNNEL_CONFIG_PARAMS1, *PIKEV2_TUNNEL_CONFIG_PARAMS1;
#endif // NTDDI_VERSION >= NTDDI_WIN8

#if (NTDDI_VERSION >= NTDDI_WINTHRESHOLD)
typedef struct _L2TP_TUNNEL_CONFIG_PARAMS2 {

    DWORD                       dwIdleTimeout;
    // encryption type to be used for L2TP
    DWORD                       dwEncryptionType;
    DWORD                       dwSaLifeTime;
    DWORD                       dwSaDataSizeForRenegotiation;

    PROUTER_CUSTOM_L2TP_POLICY0 customPolicy;
    DWORD                       dwMmSaLifeTime;
}L2TP_TUNNEL_CONFIG_PARAMS2, *PL2TP_TUNNEL_CONFIG_PARAMS2;
#endif

#if (NTDDI_VERSION >= NTDDI_WINBLUE)

typedef struct _L2TP_TUNNEL_CONFIG_PARAMS1 {

    DWORD                       dwIdleTimeout;
    // encryption type to be used for L2TP
    DWORD                       dwEncryptionType;
    DWORD                       dwSaLifeTime;
    DWORD                       dwSaDataSizeForRenegotiation;

    PROUTER_CUSTOM_L2TP_POLICY0 customPolicy;
}L2TP_TUNNEL_CONFIG_PARAMS1, *PL2TP_TUNNEL_CONFIG_PARAMS1;

#endif


#if (NTDDI_VERSION >= NTDDI_WIN8)

#define ROUTER_CUSTOM_IKEv2_POLICY  ROUTER_CUSTOM_IKEv2_POLICY0
#define PROUTER_CUSTOM_IKEv2_POLICY  PROUTER_CUSTOM_IKEv2_POLICY0

#endif // NTDDI_VERSION >= NTDDI_WIN8


#if (NTDDI_VERSION >= NTDDI_WINTHRESHOLD)

#define IKEV2_TUNNEL_CONFIG_PARAMS  IKEV2_TUNNEL_CONFIG_PARAMS4
#define PIKEV2_TUNNEL_CONFIG_PARAMS PIKEV2_TUNNEL_CONFIG_PARAMS4
#define L2TP_TUNNEL_CONFIG_PARAMS   L2TP_TUNNEL_CONFIG_PARAMS2
#define PL2TP_TUNNEL_CONFIG_PARAMS  PL2TP_TUNNEL_CONFIG_PARAMS2
#define ROUTER_IKEv2_IF_CUSTOM_CONFIG  ROUTER_IKEv2_IF_CUSTOM_CONFIG2
#define PROUTER_IKEv2_IF_CUSTOM_CONFIG  PROUTER_IKEv2_IF_CUSTOM_CONFIG2
#define MPR_IF_CUSTOMINFOEX      MPR_IF_CUSTOMINFOEX2
#define PMPR_IF_CUSTOMINFOEX    PMPR_IF_CUSTOMINFOEX2


#elif (NTDDI_VERSION >= NTDDI_WINBLUE)

#define IKEV2_TUNNEL_CONFIG_PARAMS  IKEV2_TUNNEL_CONFIG_PARAMS3
#define PIKEV2_TUNNEL_CONFIG_PARAMS PIKEV2_TUNNEL_CONFIG_PARAMS3
#define L2TP_TUNNEL_CONFIG_PARAMS   L2TP_TUNNEL_CONFIG_PARAMS1
#define PL2TP_TUNNEL_CONFIG_PARAMS  PL2TP_TUNNEL_CONFIG_PARAMS1
#define ROUTER_IKEv2_IF_CUSTOM_CONFIG  ROUTER_IKEv2_IF_CUSTOM_CONFIG1
#define PROUTER_IKEv2_IF_CUSTOM_CONFIG  PROUTER_IKEv2_IF_CUSTOM_CONFIG1
#define MPR_IF_CUSTOMINFOEX      MPR_IF_CUSTOMINFOEX1
#define PMPR_IF_CUSTOMINFOEX    PMPR_IF_CUSTOMINFOEX1


#elif (NTDDI_VERSION >= NTDDI_WIN8)

#define ROUTER_IKEv2_IF_CUSTOM_CONFIG  ROUTER_IKEv2_IF_CUSTOM_CONFIG0
#define PROUTER_IKEv2_IF_CUSTOM_CONFIG  PROUTER_IKEv2_IF_CUSTOM_CONFIG0
#define IKEV2_TUNNEL_CONFIG_PARAMS  IKEV2_TUNNEL_CONFIG_PARAMS2
#define PIKEV2_TUNNEL_CONFIG_PARAMS  PIKEV2_TUNNEL_CONFIG_PARAMS2

#define MPR_IF_CUSTOMINFOEX      MPR_IF_CUSTOMINFOEX0
#define PMPR_IF_CUSTOMINFOEX    PMPR_IF_CUSTOMINFOEX0

#else

#define IKEV2_TUNNEL_CONFIG_PARAMS  IKEV2_TUNNEL_CONFIG_PARAMS1
#define PIKEV2_TUNNEL_CONFIG_PARAMS  PIKEV2_TUNNEL_CONFIG_PARAMS1
#endif 


#if (NTDDI_VERSION >= NTDDI_WINBLUE)
#define ROUTER_CUSTOM_L2TP_POLICY  ROUTER_CUSTOM_L2TP_POLICY0
#define PROUTER_CUSTOM_L2TP_POLICY  PROUTER_CUSTOM_L2TP_POLICY0
#define L2TP_CONFIG_PARAMS         L2TP_CONFIG_PARAMS1
#define PL2TP_CONFIG_PARAMS        PL2TP_CONFIG_PARAMS1
#else
#define L2TP_CONFIG_PARAMS         L2TP_CONFIG_PARAMS0
#define PL2TP_CONFIG_PARAMS        PL2TP_CONFIG_PARAMS0

#endif 


//----------- MPR_SERVER_EX object 

#define MPRAPI_IKEV2_SET_TUNNEL_CONFIG_PARAMS   0x1
#define MPRAPI_L2TP_SET_TUNNEL_CONFIG_PARAMS   0x1
typedef struct _IKEV2_CONFIG_PARAMS {

    DWORD                       dwNumPorts;
    DWORD                       dwPortFlags;
    DWORD                       dwTunnelConfigParamFlags;
    IKEV2_TUNNEL_CONFIG_PARAMS  TunnelConfigParams;
 
}IKEV2_CONFIG_PARAMS, *PIKEV2_CONFIG_PARAMS;


typedef struct _PPTP_CONFIG_PARAMS {

    DWORD                       dwNumPorts;
    DWORD                       dwPortFlags;
    

}PPTP_CONFIG_PARAMS, *PPPTP_CONFIG_PARAMS;

#if (NTDDI_VERSION >= NTDDI_WINBLUE)

typedef struct _L2TP_CONFIG_PARAMS1 {

    DWORD                       dwNumPorts;
    DWORD                       dwPortFlags; 
    DWORD                       dwTunnelConfigParamFlags;
    L2TP_TUNNEL_CONFIG_PARAMS   TunnelConfigParams;
    
}L2TP_CONFIG_PARAMS1, *PL2TP_CONFIG_PARAMS1;

typedef struct _GRE_CONFIG_PARAMS0 {

    DWORD                       dwNumPorts;
    DWORD                       dwPortFlags;  

}GRE_CONFIG_PARAMS0, *PGRE_CONFIG_PARAMS0;

#define GRE_CONFIG_PARAMS GRE_CONFIG_PARAMS0
#define PGRE_CONFIG_PARAMS PGRE_CONFIG_PARAMS0
#endif

typedef struct _L2TP_CONFIG_PARAMS0 {

    DWORD                       dwNumPorts;
    DWORD                       dwPortFlags;
    
}L2TP_CONFIG_PARAMS0, *PL2TP_CONFIG_PARAMS0;

#define MAX_SSTP_HASH_SIZE       32 // SHA-256 Certificate hash size

typedef struct _SSTP_CERT_INFO
{
    // Whether it is the default node [no cert config]
    BOOL                isDefault;
    CRYPT_HASH_BLOB      certBlob;
}SSTP_CERT_INFO, *PSSTP_CERT_INFO;

typedef struct _SSTP_CONFIG_PARAMS {

    DWORD                       dwNumPorts;
    DWORD                       dwPortFlags;
    BOOL                        isUseHttps;
    DWORD                       certAlgorithm; // Should always be CALG_SHA_256
    SSTP_CERT_INFO              sstpCertDetails;

}SSTP_CONFIG_PARAMS, *PSSTP_CONFIG_PARAMS;


typedef struct _MPRAPI_TUNNEL_CONFIG_PARAMS0 {

    IKEV2_CONFIG_PARAMS         IkeConfigParams;

    PPTP_CONFIG_PARAMS          PptpConfigParams;

    L2TP_CONFIG_PARAMS	        L2tpConfigParams;

    SSTP_CONFIG_PARAMS          SstpConfigParams;

}MPRAPI_TUNNEL_CONFIG_PARAMS0, *PMPRAPI_TUNNEL_CONFIG_PARAMS0;

#if (NTDDI_VERSION >= NTDDI_WINBLUE)
typedef struct _MPRAPI_TUNNEL_CONFIG_PARAMS1 {

    IKEV2_CONFIG_PARAMS         IkeConfigParams;

    PPTP_CONFIG_PARAMS          PptpConfigParams;

    L2TP_CONFIG_PARAMS	        L2tpConfigParams;

    SSTP_CONFIG_PARAMS          SstpConfigParams;

    GRE_CONFIG_PARAMS           GREConfigParams;

}MPRAPI_TUNNEL_CONFIG_PARAMS1, *PMPRAPI_TUNNEL_CONFIG_PARAMS1;
#endif

#if (NTDDI_VERSION >= NTDDI_WINTHRESHOLD)
#define MPRAPI_TUNNEL_CONFIG_PARAMS MPRAPI_TUNNEL_CONFIG_PARAMS1
#define PMPRAPI_TUNNEL_CONFIG_PARAMS PMPRAPI_TUNNEL_CONFIG_PARAMS1
#else
#define MPRAPI_TUNNEL_CONFIG_PARAMS MPRAPI_TUNNEL_CONFIG_PARAMS0
#define PMPRAPI_TUNNEL_CONFIG_PARAMS PMPRAPI_TUNNEL_CONFIG_PARAMS0
#endif

#define MPRAPI_MPR_SERVER_OBJECT_REVISION_1     0x1 
#define MPRAPI_MPR_SERVER_OBJECT_REVISION_2     0x2 
#if (NTDDI_VERSION >= NTDDI_WINBLUE)
#define MPRAPI_MPR_SERVER_OBJECT_REVISION_3     0x3 
#define MPRAPI_MPR_SERVER_OBJECT_REVISION_4     0x4
#endif
#if (NTDDI_VERSION >= NTDDI_WINTHRESHOLD)
#define MPRAPI_MPR_SERVER_OBJECT_REVISION_5     0x5
#endif


typedef struct _MPR_SERVER_EX0 {
    MPRAPI_OBJECT_HEADER        Header;
    DWORD                       fLanOnlyMode;
    DWORD                       dwUpTime;
    DWORD                       dwTotalPorts;
    DWORD                       dwPortsInUse; 
    DWORD                       Reserved; // Added so that the structure is 8 byte aligned
    MPRAPI_TUNNEL_CONFIG_PARAMS0 ConfigParams;
}MPR_SERVER_EX0, *PMPR_SERVER_EX0;

#if (NTDDI_VERSION >= NTDDI_WINBLUE)
typedef struct _MPR_SERVER_EX1 {
    MPRAPI_OBJECT_HEADER        Header;
    DWORD                       fLanOnlyMode;
    DWORD                       dwUpTime;
    DWORD                       dwTotalPorts;
    DWORD                       dwPortsInUse; 
    DWORD                       Reserved; // Added so that the structure is 8 byte aligned
    MPRAPI_TUNNEL_CONFIG_PARAMS1 ConfigParams;
}MPR_SERVER_EX1, *PMPR_SERVER_EX1;
#endif

#if (NTDDI_VERSION >= NTDDI_WINTHRESHOLD)
#define MPR_SERVER_EX MPR_SERVER_EX1
#define PMPR_SERVER_EX PMPR_SERVER_EX1
#else
#define MPR_SERVER_EX MPR_SERVER_EX0
#define PMPR_SERVER_EX PMPR_SERVER_EX0
#endif

#define MPRAPI_MPR_SERVER_SET_CONFIG_OBJECT_REVISION_1      0x1 

#if (NTDDI_VERSION >= NTDDI_WIN8)
#define MPRAPI_MPR_SERVER_SET_CONFIG_OBJECT_REVISION_2      0x2 
#endif

#if (NTDDI_VERSION >= NTDDI_WINBLUE)
#define MPRAPI_MPR_SERVER_SET_CONFIG_OBJECT_REVISION_3      0x3 
#define MPRAPI_MPR_SERVER_SET_CONFIG_OBJECT_REVISION_4      0x4
#endif

#if (NTDDI_VERSION >= NTDDI_WINTHRESHOLD)
#define MPRAPI_MPR_SERVER_SET_CONFIG_OBJECT_REVISION_5      0x5
#endif

#define MPRAPI_SET_CONFIG_PROTOCOL_FOR_PPTP                 0x1
#define MPRAPI_SET_CONFIG_PROTOCOL_FOR_L2TP                 0x2
#define MPRAPI_SET_CONFIG_PROTOCOL_FOR_SSTP                 0x4
#define MPRAPI_SET_CONFIG_PROTOCOL_FOR_IKEV2                0x8
#define MPRAPI_SET_CONFIG_PROTOCOL_FOR_GRE                  0x10


typedef struct _MPR_SERVER_SET_CONFIG_EX0 {
    MPRAPI_OBJECT_HEADER        Header;
    DWORD                       setConfigForProtocols;
    MPRAPI_TUNNEL_CONFIG_PARAMS0 ConfigParams;
}MPR_SERVER_SET_CONFIG_EX0, *PMPR_SERVER_SET_CONFIG_EX0;

#if (NTDDI_VERSION >= NTDDI_WINBLUE)
typedef struct _MPR_SERVER_SET_CONFIG_EX1 {
    MPRAPI_OBJECT_HEADER        Header;
    DWORD                       setConfigForProtocols;
    MPRAPI_TUNNEL_CONFIG_PARAMS1 ConfigParams;
}MPR_SERVER_SET_CONFIG_EX1, *PMPR_SERVER_SET_CONFIG_EX1;
#endif


#if (NTDDI_VERSION >= NTDDI_WINTHRESHOLD)
#define MPR_SERVER_SET_CONFIG_EX MPR_SERVER_SET_CONFIG_EX1
#define PMPR_SERVER_SET_CONFIG_EX PMPR_SERVER_SET_CONFIG_EX1
#else
#define MPR_SERVER_SET_CONFIG_EX MPR_SERVER_SET_CONFIG_EX0
#define PMPR_SERVER_SET_CONFIG_EX PMPR_SERVER_SET_CONFIG_EX0
#endif

#define ALLOW_NO_AUTH         1
#define DO_NOT_ALLOW_NO_AUTH  0

typedef struct _AUTH_VALIDATION_EX {
    MPRAPI_OBJECT_HEADER        Header;
    HANDLE                      hRasConnection;
    WCHAR                       wszUserName[UNLEN + 1 ];
    WCHAR                       wszLogonDomain[DNLEN + 1 ];
    DWORD                       AuthInfoSize;
    BYTE                        AuthInfo[1];
    
} AUTH_VALIDATION_EX;

#define MPRAPI_RAS_UPDATE_CONNECTION_OBJECT_REVISION_1     0x1
typedef struct RAS_UPDATE_CONNECTION_
{
    MPRAPI_OBJECT_HEADER    Header;
    DWORD                   dwIfIndex;
    WCHAR                   wszLocalEndpointAddress[MAXIPADRESSLEN+1];
    WCHAR                   wszRemoteEndpointAddress[MAXIPADRESSLEN+1];
} 
RAS_UPDATE_CONNECTION,*PRAS_UPDATE_CONNECTION;

// RAS Admin Functions:
DWORD APIENTRY
MprAdminConnectionEnumEx(
     _In_      RAS_SERVER_HANDLE       hRasServer,
     _In_      PMPRAPI_OBJECT_HEADER   pObjectHeader,
     _In_      DWORD                   dwPreferedMaxLen,
     _Out_     LPDWORD                 lpdwEntriesRead,
     _Out_     LPDWORD                 lpdwTotalEntries,
     _Out_     PRAS_CONNECTION_EX      *ppRasConn,
     _In_      LPDWORD                 lpdwResumeHandle);


DWORD APIENTRY MprAdminConnectionGetInfoEx(
   _In_       RAS_SERVER_HANDLE       hRasServer,
   _In_       HANDLE                  hRasConnection,
   _Out_      PRAS_CONNECTION_EX      pRasConnection
); // Use MprAdminBufferFree for freeing pRASCONN

// Router Admin Functions:
DWORD APIENTRY MprAdminServerGetInfoEx(
    _In_      MPR_SERVER_HANDLE       hMprServer,
    _Out_     MPR_SERVER_EX*          pServerInfo);


DWORD APIENTRY MprAdminServerSetInfoEx(
    _In_      MPR_SERVER_HANDLE           hMprServer,
    _In_      MPR_SERVER_SET_CONFIG_EX*   pServerInfo);


// Router Config Functions:

DWORD APIENTRY MprConfigServerGetInfoEx(
    _In_      HANDLE                  hMprConfig,
    _Out_     MPR_SERVER_EX*          pServerInfo);


DWORD APIENTRY MprConfigServerSetInfoEx(
    _In_      HANDLE                      hMprConfig,
    _In_      MPR_SERVER_SET_CONFIG_EX*   pSetServerConfig);

DWORD APIENTRY MprAdminUpdateConnection(
    _In_      RAS_SERVER_HANDLE       hRasServer,
    _In_      HANDLE                  hRasConnection,
    _In_      PRAS_UPDATE_CONNECTION  pRasUpdateConnection);

// Admin DLL's:
#define MPRAPI_ADMIN_DLL_VERSION_1          0x1 
#define MPRAPI_ADMIN_DLL_VERSION_2          0x2 // If the version is 2: it can use RAS_CONNECTION_EX supported in Windows 7.0

typedef DWORD (APIENTRY * PMPRADMINGETIPADDRESSFORUSER)(WCHAR *, WCHAR *, DWORD *, BOOL *);

typedef VOID  (APIENTRY * PMPRADMINRELEASEIPADRESS)(WCHAR *, WCHAR *, DWORD *);

typedef DWORD (APIENTRY * PMPRADMINGETIPV6ADDRESSFORUSER)(WCHAR *, WCHAR *, IN6_ADDR *, BOOL *);

typedef VOID  (APIENTRY * PMPRADMINRELEASEIPV6ADDRESSFORUSER)(WCHAR *, WCHAR *, IN6_ADDR *);

typedef BOOL  (APIENTRY * PMPRADMINACCEPTNEWCONNECTION)(RAS_CONNECTION_0 *, RAS_CONNECTION_1 *);

typedef BOOL  (APIENTRY * PMPRADMINACCEPTNEWCONNECTION2)(RAS_CONNECTION_0 *, RAS_CONNECTION_1 *, RAS_CONNECTION_2 *);

typedef BOOL  (APIENTRY * PMPRADMINACCEPTNEWCONNECTION3)(RAS_CONNECTION_0 *, RAS_CONNECTION_1 *, RAS_CONNECTION_2 *, RAS_CONNECTION_3 *);

typedef BOOL  (APIENTRY * PMPRADMINACCEPTNEWLINK)(RAS_PORT_0 *, RAS_PORT_1 *);

typedef VOID  (APIENTRY * PMPRADMINCONNECTIONHANGUPNOTIFICATION)(RAS_CONNECTION_0 *, RAS_CONNECTION_1 *);

typedef VOID  (APIENTRY * PMPRADMINCONNECTIONHANGUPNOTIFICATION2)(RAS_CONNECTION_0 *, RAS_CONNECTION_1 *, RAS_CONNECTION_2 *);

typedef VOID  (APIENTRY * PMPRADMINCONNECTIONHANGUPNOTIFICATION3)(RAS_CONNECTION_0 *, RAS_CONNECTION_1 *, RAS_CONNECTION_2 *, RAS_CONNECTION_3);

typedef VOID  (APIENTRY * PMPRADMINLINKHANGUPNOTIFICATION)(RAS_PORT_0 *, RAS_PORT_1 *);

typedef DWORD (APIENTRY * PMPRADMINTERMINATEDLL)();

typedef BOOL  (APIENTRY * PMPRADMINACCEPTREAUTHENTICATION)(RAS_CONNECTION_0 *, RAS_CONNECTION_1 *, RAS_CONNECTION_2 *, RAS_CONNECTION_3 *);


typedef BOOL  (APIENTRY *PMPRADMINACCEPTNEWCONNECTIONEX)(RAS_CONNECTION_EX*);

typedef BOOL  (APIENTRY *PMPRADMINACCEPTREAUTHENTICATIONEX)(RAS_CONNECTION_EX*);

typedef BOOL  (APIENTRY *PMPRADMINACCEPTTUNNELENDPOINTCHANGEEX)(RAS_CONNECTION_EX*); 

typedef VOID  (APIENTRY *PMPRADMINCONNECTIONHANGUPNOTIFICATIONEX)(RAS_CONNECTION_EX*);

typedef DWORD (APIENTRY *PMPRADMINRASVALIDATEPREAUTHENTICATEDCONNECTIONEX) (AUTH_VALIDATION_EX *);


typedef struct _MPRAPI_ADMIN_DLL_CALLBACKS
{

    UCHAR                                           revision;

    PMPRADMINGETIPADDRESSFORUSER                    lpfnMprAdminGetIpAddressForUser;

    PMPRADMINRELEASEIPADRESS                        lpfnMprAdminReleaseIpAddress;

    PMPRADMINGETIPV6ADDRESSFORUSER                  lpfnMprAdminGetIpv6AddressForUser;

    PMPRADMINRELEASEIPV6ADDRESSFORUSER              lpfnMprAdminReleaseIpV6AddressForUser;

    PMPRADMINACCEPTNEWLINK                          lpfnRasAdminAcceptNewLink;

    PMPRADMINLINKHANGUPNOTIFICATION                 lpfnRasAdminLinkHangupNotification;

    PMPRADMINTERMINATEDLL                           lpfnRasAdminTerminateDll;

    PMPRADMINACCEPTNEWCONNECTIONEX                  lpfnRasAdminAcceptNewConnectionEx;

    PMPRADMINACCEPTTUNNELENDPOINTCHANGEEX           lpfnRasAdminAcceptEndpointChangeEx;

    PMPRADMINACCEPTREAUTHENTICATIONEX               lpfnRasAdminAcceptReauthenticationEx;

    PMPRADMINCONNECTIONHANGUPNOTIFICATIONEX         lpfnRasAdminConnectionHangupNotificationEx;
    
    PMPRADMINRASVALIDATEPREAUTHENTICATEDCONNECTIONEX lpfnRASValidatePreAuthenticatedConnectionEx;
    
} MPRAPI_ADMIN_DLL_CALLBACKS, *PMPRAPI_ADMIN_DLL_CALLBACKS;


BOOL APIENTRY MprAdminAcceptNewConnectionEx(
    RAS_CONNECTION_EX*                    pRasConn
);

BOOL APIENTRY MprAdminAcceptReauthenticationEx(
    RAS_CONNECTION_EX*                    pRasConn
);

void APIENTRY MprAdminConnectionHangupNotificationEx(
    RAS_CONNECTION_EX*                    pRasConn
);

DWORD APIENTRY MprAdminInitializeDllEx(
    PMPRAPI_ADMIN_DLL_CALLBACKS           pAdminCallbacks
);

DWORD APIENTRY MprAdminIsServiceInitialized(
    _In_ LPWSTR  lpwsServerName,
    _In_ BOOL    *fIsServiceInitialized
);

#if(NTDDI_VERSION >= NTDDI_WIN8)

// Admin APIs
DWORD APIENTRY
MprAdminInterfaceSetCustomInfoEx(
    _In_      MPR_SERVER_HANDLE       hMprServer,
    _In_      HANDLE                            hInterface,
    _In_      PMPR_IF_CUSTOMINFOEX  pCustomInfo
);

DWORD APIENTRY
MprAdminInterfaceGetCustomInfoEx(
    _In_      MPR_SERVER_HANDLE       hMprServer,
    _In_      HANDLE                            hInterface,
    _Out_    PMPR_IF_CUSTOMINFOEX  pCustomInfo
);

// Config APIs
DWORD APIENTRY
MprConfigInterfaceGetCustomInfoEx(
    _In_      HANDLE                            hMprConfig,
    _In_      HANDLE                            hRouterInterface,
    _Out_    PMPR_IF_CUSTOMINFOEX  pCustomInfo
);

DWORD APIENTRY
MprConfigInterfaceSetCustomInfoEx(
    _In_      HANDLE                            hMprConfig,
    _In_      HANDLE                            hRouterInterface,
    _In_      PMPR_IF_CUSTOMINFOEX  pCustomInfo
);
#endif /* NTDDI_VERSION >= NTDDI_WIN8 */

#endif /* WINVER >= 0x0601 */

//
//  RAS ADMIN APIs
//

DWORD APIENTRY
MprAdminConnectionEnum(
    _In_      RAS_SERVER_HANDLE       hRasServer,
    _In_      DWORD                   dwLevel,
    _Outptr_result_bytebuffer_((*lpdwEntriesRead)*(dwLevel==0?
    sizeof(RAS_CONNECTION_0):(dwLevel==1?
    sizeof(RAS_CONNECTION_1):(dwLevel==2?
    sizeof(RAS_CONNECTION_2):sizeof(RAS_CONNECTION_3))))) LPBYTE *  lplpbBuffer,
    _In_      DWORD                   dwPrefMaxLen,
    _Out_     LPDWORD                 lpdwEntriesRead,
    _Out_     LPDWORD                 lpdwTotalEntries,
    _In_opt_  LPDWORD                 lpdwResumeHandle
);

DWORD APIENTRY
MprAdminPortEnum(
    _In_      RAS_SERVER_HANDLE       hRasServer,
    _In_      DWORD                   dwLevel,
    _In_      HANDLE                  hRasConnection,
    _Outptr_result_bytebuffer_((*lpdwEntriesRead)*sizeof(RAS_PORT_0)) LPBYTE *  lplpbBuffer,
    _In_      DWORD                   dwPrefMaxLen,
    _Out_     LPDWORD                 lpdwEntriesRead,
    _Out_     LPDWORD                 lpdwTotalEntries,
    _In_opt_  LPDWORD                 lpdwResumeHandle
);

DWORD APIENTRY
MprAdminConnectionGetInfo(
    _In_      RAS_SERVER_HANDLE       hRasServer,
    _In_      DWORD                   dwLevel,
    _In_      HANDLE                  hRasConnection,
    _Out_     LPBYTE *                lplpbBuffer
);

DWORD APIENTRY
MprAdminPortGetInfo(
    _In_      RAS_SERVER_HANDLE       hRasServer,
    _In_      DWORD                   dwLevel,
    _In_      HANDLE                  hPort,
    _Out_     LPBYTE *                lplpbBuffer
);

DWORD APIENTRY
MprAdminConnectionClearStats(
    _In_      RAS_SERVER_HANDLE       hRasServer,
    _In_      HANDLE                  hRasConnection
);

DWORD APIENTRY
MprAdminPortClearStats(
    _In_      RAS_SERVER_HANDLE       hRasServer,
    _In_      HANDLE                  hPort
);

DWORD APIENTRY
MprAdminPortReset(
    _In_      RAS_SERVER_HANDLE       hRasServer,
    _In_      HANDLE                  hPort
);

DWORD APIENTRY
MprAdminPortDisconnect(
    _In_      RAS_SERVER_HANDLE       hRasServer,
    _In_      HANDLE                  hPort
);

BOOL APIENTRY
MprAdminAcceptNewConnection(
    _In_      RAS_CONNECTION_0 *      pRasConnection0,
    _In_      RAS_CONNECTION_1 *      pRasConnection1
    );

#if(WINVER >= 0x0500)

BOOL APIENTRY
MprAdminAcceptNewConnection2(
    _In_      RAS_CONNECTION_0 *      pRasConnection0,
    _In_      RAS_CONNECTION_1 *      pRasConnection1,
    _In_      RAS_CONNECTION_2 *      pRasConnection2
    );

#endif /* WINVER >= 0x0500 */

#if(WINVER >= 0x0600)

BOOL APIENTRY
MprAdminAcceptNewConnection3(
    _In_      RAS_CONNECTION_0 *      pRasConnection0,
    _In_      RAS_CONNECTION_1 *      pRasConnection1,
    _In_      RAS_CONNECTION_2 *      pRasConnection2,
    _In_      RAS_CONNECTION_3 *      pRasConnection3
    );

BOOL APIENTRY
MprAdminAcceptReauthentication(
    _In_      RAS_CONNECTION_0 *      pRasConnection0,
    _In_      RAS_CONNECTION_1 *      pRasConnection1,
    _In_      RAS_CONNECTION_2 *      pRasConnection2,
    _In_      RAS_CONNECTION_3 *      pRasConnection3
    );
#endif /* WINVER >= 0x0600 */

BOOL APIENTRY
MprAdminAcceptNewLink (
    _In_      RAS_PORT_0 *            pRasPort0,
    _In_      RAS_PORT_1 *            pRasPort1
    );

VOID APIENTRY
MprAdminConnectionHangupNotification(
    _In_      RAS_CONNECTION_0 *      pRasConnection0,
    _In_      RAS_CONNECTION_1 *      pRasConnection1
    );

#if(WINVER >= 0x0500)

VOID APIENTRY
MprAdminConnectionHangupNotification2(
    _In_      RAS_CONNECTION_0 *      pRasConnection0,
    _In_      RAS_CONNECTION_1 *      pRasConnection1,
    _In_      RAS_CONNECTION_2 *      pRasConnection2
    );

#endif /* WINVER >= 0x0500 */

#if(WINVER >= 0x0600)

VOID APIENTRY
MprAdminConnectionHangupNotification3(
    _In_      RAS_CONNECTION_0 *      pRasConnection0,
    _In_      RAS_CONNECTION_1 *      pRasConnection1,
    _In_      RAS_CONNECTION_2 *      pRasConnection2,
    _In_      RAS_CONNECTION_3 *      pRasConnection3
    );

#endif /* WINVER >= 0x0600 */

#if (WINVER >= 0x501)

DWORD APIENTRY
MprAdminConnectionRemoveQuarantine(
    _In_      HANDLE          hRasServer,    
    _In_      HANDLE          hRasConnection,
    _In_      BOOL            fIsIpAddress
    );

#endif

VOID APIENTRY
MprAdminLinkHangupNotification (
    _In_      RAS_PORT_0 *            pRasPort0,
    _In_      RAS_PORT_1 *            pRasPort1
    );

DWORD APIENTRY
MprAdminGetIpAddressForUser (
    _In_      WCHAR *                 lpwszUserName,
    _In_      WCHAR *                 lpwszPortName,
    _Inout_  DWORD *                  lpdwIpAddress,
    _Out_     BOOL *                  bNotifyRelease
    );


    
VOID APIENTRY
MprAdminReleaseIpAddress (
    _In_      WCHAR *                 lpszUserName,
    _In_      WCHAR *                 lpszPortName,
    _In_      DWORD *                 lpdwIpAddress
    );


#if(WINVER >= 0x0600)

DWORD APIENTRY 
MprAdminGetIpv6AddressForUser (
    _In_      WCHAR *                 lpwszUserName,
    _In_      WCHAR *                 lpwszPortName,
    _Inout_  IN6_ADDR *               lpdwIpv6Address, /* Currently Only 64 Bit Identifier is supported */
    _Out_     BOOL *                  bNotifyRelease
    );

VOID MprAdminReleaseIpv6AddressForUser (
    _In_      WCHAR *                 lpszUserName,
    _In_      WCHAR *                 lpszPortName,
    _In_      IN6_ADDR *              lpdwIpv6Address 
    );

#endif /* WINVER >= 0x0600 */

#if(WINVER >= 0x0500)

DWORD APIENTRY
MprAdminInitializeDll(
    VOID
);

DWORD APIENTRY
MprAdminTerminateDll(
    VOID
);

#endif

//
// MprAdminUser APIs
//

DWORD APIENTRY
MprAdminUserGetInfo(
    _In_      LPCWSTR           lpszServer,
    _In_      LPCWSTR           lpszUser,
    _In_      DWORD                   dwLevel,
    _Out_writes_bytes_(dwLevel==1?sizeof(RAS_USER_1):sizeof(RAS_USER_0))     LPBYTE                  lpbBuffer
);


DWORD APIENTRY
MprAdminUserSetInfo(
    _In_      LPCWSTR           lpszServer,
    _In_      LPCWSTR           lpszUser,
    _In_      DWORD                   dwLevel,
    _In_reads_bytes_(dwLevel==1?sizeof(RAS_USER_1):sizeof(RAS_USER_0))      const LPBYTE            lpbBuffer
);


#if(WINVER >= 0x0500)

DWORD APIENTRY
MprAdminSendUserMessage(
    _In_      MPR_SERVER_HANDLE       hMprServer,
    _In_      HANDLE                  hConnection,
    _In_      LPWSTR                  lpwszMessage
);

#endif /* WINVER >= 0x0500 */

DWORD APIENTRY
MprAdminGetPDCServer(
    _In_       LPCWSTR          lpszDomain,
    _In_       LPCWSTR          lpszServer,
    _Out_writes_(UNCLEN+1) LPWSTR     lpszPDCServer
);

//
// Router APIs
//

BOOL APIENTRY
MprAdminIsServiceRunning(
    _In_  LPWSTR  lpwsServerName
);

DWORD APIENTRY
MprAdminServerConnect(
    _In_opt_  LPWSTR                  lpwsServerName,
    _Out_     MPR_SERVER_HANDLE *     phMprServer
);

VOID APIENTRY
MprAdminServerDisconnect(
    _In_      MPR_SERVER_HANDLE       hMprServer
);

#if (WINVER >= 0x501)
DWORD APIENTRY
MprAdminServerGetCredentials(
    _In_  MPR_SERVER_HANDLE       hMprServer,
    _In_  DWORD                   dwLevel,
    _In_  LPBYTE *                lplpbBuffer
);

DWORD APIENTRY
MprAdminServerSetCredentials(
    _In_      MPR_SERVER_HANDLE       hMprServer,
    _In_      DWORD                   dwLevel,
    _In_      LPBYTE                  lpbBuffer
);


#endif

DWORD APIENTRY
MprAdminBufferFree(
    _In_      LPVOID                  pBuffer
);

DWORD APIENTRY
MprAdminGetErrorString(
    _In_      DWORD                   dwError,
    _Out_     LPWSTR *                lplpwsErrorString
);

DWORD APIENTRY
MprAdminServerGetInfo(
    _In_      MPR_SERVER_HANDLE       hMprServer,
    _In_      DWORD                   dwLevel,
    _Out_     LPBYTE *                lplpbBuffer
);

#if (WINVER >= 0x501)
DWORD APIENTRY
MprAdminServerSetInfo(
    _In_      MPR_SERVER_HANDLE       hMprServer,
    _In_      DWORD                   dwLevel,
    _In_reads_bytes_(dwLevel==1?sizeof(MPR_SERVER_1):sizeof(MPR_SERVER_2))      LPBYTE                  lpbBuffer
);


DWORD  APIENTRY
MprAdminEstablishDomainRasServer (
    _In_      LPWSTR                 pszDomain,
    _In_      LPWSTR                 pszMachine,
    _In_      BOOL                   bEnable
);

DWORD  APIENTRY
MprAdminIsDomainRasServer (
    _In_     LPWSTR                  pszDomain,
    _In_     LPWSTR                  pszMachine,
    _Out_    PBOOL                   pbIsRasServer
);

#endif

#if(WINVER >= 0x0500)

DWORD APIENTRY
MprAdminTransportCreate(
    _In_      MPR_SERVER_HANDLE       hMprServer,
    _In_      DWORD                   dwTransportId,
    _In_opt_  LPWSTR                  lpwsTransportName,
    _In_      LPBYTE                  pGlobalInfo,
    _In_      DWORD                   dwGlobalInfoSize,
    _In_opt_  LPBYTE                  pClientInterfaceInfo,
    _In_opt_  DWORD                   dwClientInterfaceInfoSize,
    _In_      LPWSTR                  lpwsDLLPath
);

#endif /* WINVER >= 0x0500 */

DWORD APIENTRY
MprAdminTransportSetInfo(
    _In_      MPR_SERVER_HANDLE       hMprServer,
    _In_      DWORD                   dwTransportId,
    _In_opt_  LPBYTE                  pGlobalInfo,
    _In_      DWORD                   dwGlobalInfoSize,
    _In_opt_  LPBYTE                  pClientInterfaceInfo,
    _In_      DWORD                   dwClientInterfaceInfoSize
);

DWORD APIENTRY
MprAdminTransportGetInfo(
    _In_      MPR_SERVER_HANDLE       hMprServer,
    _In_      DWORD                   dwTransportId,
    _Out_opt_ LPBYTE *                ppGlobalInfo,
    _Out_opt_ LPDWORD                 lpdwGlobalInfoSize,
    _Out_opt_ LPBYTE *                ppClientInterfaceInfo,
    _Out_opt_ LPDWORD                 lpdwClientInterfaceInfoSize
);

#if(WINVER >= 0x0500)

DWORD APIENTRY
MprAdminDeviceEnum(
    _In_      MPR_SERVER_HANDLE       hMprServer,
    _In_      DWORD                   dwLevel,
    _Out_     LPBYTE*                 lplpbBuffer,
    _Out_     LPDWORD                 lpdwTotalEntries);

#endif /* WINVER >= 0x0500 */

DWORD APIENTRY
MprAdminInterfaceGetHandle(
    _In_      MPR_SERVER_HANDLE       hMprServer,
    _In_      LPWSTR                  lpwsInterfaceName,
    _Out_     HANDLE *                phInterface,
    _In_      BOOL                    fIncludeClientInterfaces
);

DWORD APIENTRY
MprAdminInterfaceCreate(
    _In_      MPR_SERVER_HANDLE       hMprServer,
    _In_      DWORD                   dwLevel,
    _In_      LPBYTE                  lpbBuffer,
    _Out_     HANDLE *                phInterface
);

DWORD APIENTRY
MprAdminInterfaceGetInfo(
    _In_      MPR_SERVER_HANDLE       hMprServer,
    _In_      HANDLE                  hInterface,
    _In_      DWORD                   dwLevel,
    _In_      LPBYTE *                lplpbBuffer
);

DWORD APIENTRY
MprAdminInterfaceSetInfo(
    _In_      MPR_SERVER_HANDLE       hMprServer,
    _In_      HANDLE                  hInterface,
    _In_      DWORD                   dwLevel,
    _In_      LPBYTE                  lpbBuffer
);

DWORD APIENTRY
MprAdminInterfaceDelete(
    _In_      MPR_SERVER_HANDLE       hMprServer,
    _In_      HANDLE                  hInterface
);

#if(WINVER >= 0x0500)

DWORD APIENTRY
MprAdminInterfaceDeviceGetInfo(
    _In_      MPR_SERVER_HANDLE       hMprServer,
    _In_      HANDLE                  hInterface,
    _In_      DWORD                   dwIndex,
    _In_      DWORD                   dwLevel,
    _Out_     LPBYTE*                 lplpBuffer
);

DWORD APIENTRY
MprAdminInterfaceDeviceSetInfo(
    _In_      MPR_SERVER_HANDLE       hMprServer,
    _In_      HANDLE                  hInterface,
    _In_      DWORD                   dwIndex,
    _In_      DWORD                   dwLevel,
    _In_reads_bytes_(dwLevel==1?sizeof(MPR_DEVICE_1):sizeof(MPR_DEVICE_0)) LPBYTE lpbBuffer
);

#endif /* WINVER >= 0x0500 */

DWORD APIENTRY
MprAdminInterfaceTransportRemove(
    _In_      MPR_SERVER_HANDLE       hMprServer,
    _In_      HANDLE                  hInterface,
    _In_      DWORD                   dwTransportId
);

DWORD APIENTRY
MprAdminInterfaceTransportAdd(
    _In_      MPR_SERVER_HANDLE       hMprServer,
    _In_      HANDLE                  hInterface,
    _In_      DWORD                   dwTransportId,
    _In_      LPBYTE                  pInterfaceInfo,
    _In_      DWORD                   dwInterfaceInfoSize
);

DWORD APIENTRY
MprAdminInterfaceTransportGetInfo(
    _In_      MPR_SERVER_HANDLE       hMprServer,
    _In_      HANDLE                  hInterface,
    _In_      DWORD                   dwTransportId,
    _Out_     LPBYTE *                ppInterfaceInfo,
    _Out_opt_ LPDWORD                 lpdwInterfaceInfoSize
);

DWORD APIENTRY
MprAdminInterfaceTransportSetInfo(
    _In_      MPR_SERVER_HANDLE       hMprServer,
    _In_      HANDLE                  hInterface,
    _In_      DWORD                   dwTransportId,
    _In_      LPBYTE                  pInterfaceInfo,
    _In_      DWORD                   dwInterfaceInfoSize
);

DWORD APIENTRY
MprAdminInterfaceEnum(
    _In_      MPR_SERVER_HANDLE       hMprServer,
    _In_      DWORD                   dwLevel,
    _Outptr_result_bytebuffer_(dwLevel==1?(sizeof(MPR_INTERFACE_1) * (*lpdwEntriesRead)):(sizeof(MPR_INTERFACE_0) * (*lpdwEntriesRead))) LPBYTE *  lplpbBuffer,
    _In_      DWORD                   dwPrefMaxLen,
    _Out_     LPDWORD                 lpdwEntriesRead,
    _Out_     LPDWORD                 lpdwTotalEntries,
    _In_opt_  LPDWORD                 lpdwResumeHandle
);

DWORD APIENTRY
MprSetupIpInIpInterfaceFriendlyNameEnum(
    _In_ PWCHAR  pwszMachineName,
    _Out_ LPBYTE* lplpBuffer,
    _Out_ LPDWORD lpdwEntriesRead
    );

DWORD APIENTRY
MprSetupIpInIpInterfaceFriendlyNameFree(
    _In_  LPVOID  lpBuffer
    );

DWORD APIENTRY
MprSetupIpInIpInterfaceFriendlyNameCreate(
    _In_ PWCHAR                  pwszMachineName,
    _In_ PMPR_IPINIP_INTERFACE_0 pNameInformation
    );

DWORD APIENTRY
MprSetupIpInIpInterfaceFriendlyNameDelete(
    _In_  PWCHAR  pwszMachineName,
    _In_  GUID    *pGuid
    );

DWORD APIENTRY
MprAdminInterfaceSetCredentials(
    _In_opt_       LPWSTR                  lpwsServer,
    _In_           LPWSTR                  lpwsInterfaceName,
    _In_opt_       LPWSTR                  lpwsUserName,
    _In_opt_       LPWSTR                  lpwsDomainName,
    _In_opt_       LPWSTR                  lpwsPassword
);

DWORD APIENTRY
MprAdminInterfaceGetCredentials(
    _In_opt_         LPWSTR                  lpwsServer,
    _In_             LPWSTR                  lpwsInterfaceName,
    _Out_writes_opt_(UNLEN+1)       LPWSTR   lpwsUserName,
    _Out_writes_opt_(PWLEN+1)       LPWSTR   lpwsPassword,
    _Out_writes_opt_(DNLEN+1)       LPWSTR   lpwsDomainName

);

#if(WINVER >= 0x0500)

DWORD APIENTRY
MprAdminInterfaceSetCredentialsEx(
    _In_      MPR_SERVER_HANDLE       hMprServer,
    _In_      HANDLE                  hInterface,
    _In_      DWORD                   dwLevel,
    _In_      LPBYTE                  lpbBuffer
);

DWORD APIENTRY
MprAdminInterfaceGetCredentialsEx(
    _In_      MPR_SERVER_HANDLE       hMprServer,
    _In_      HANDLE                  hInterface,
    _In_      DWORD                   dwLevel,
    _Out_     LPBYTE *                lplpbBuffer
);

#endif /* WINVER >= 0x0500 */

DWORD APIENTRY
MprAdminInterfaceConnect(
    _In_      MPR_SERVER_HANDLE       hMprServer,
    _In_      HANDLE                  hInterface,
    _In_      HANDLE                  hEvent,
    _In_      BOOL                    fSynchronous
);

DWORD APIENTRY
MprAdminInterfaceDisconnect(
    _In_      MPR_SERVER_HANDLE       hMprServer,
    _In_      HANDLE                  hInterface
);

DWORD APIENTRY
MprAdminInterfaceUpdateRoutes(
    _In_      MPR_SERVER_HANDLE       hMprServer,
    _In_      HANDLE                  hInterface,
    _In_      DWORD                   dwProtocolId,
    _In_      HANDLE                  hEvent
);

DWORD APIENTRY
MprAdminInterfaceQueryUpdateResult(
    _In_      MPR_SERVER_HANDLE       hMprServer,
    _In_      HANDLE                  hInterface,
    _In_      DWORD                   dwProtocolId,
    _Out_     LPDWORD                 lpdwUpdateResult
);

DWORD APIENTRY
MprAdminInterfaceUpdatePhonebookInfo(
    _In_      MPR_SERVER_HANDLE       hMprServer,
    _In_      HANDLE                  hInterface
);

#if(WINVER >= 0x0500)

DWORD APIENTRY
MprAdminRegisterConnectionNotification(
    _In_      MPR_SERVER_HANDLE       hMprServer,
    _In_      HANDLE                  hEventNotification
);

DWORD APIENTRY
MprAdminDeregisterConnectionNotification(
    _In_      MPR_SERVER_HANDLE       hMprServer,
    _In_      HANDLE                  hEventNotification
);

#endif /* WINVER >= 0x0500 */

//
// MIB APIs
//

DWORD APIENTRY
MprAdminMIBServerConnect(
    _In_opt_  LPWSTR                  lpwsServerName,
    _Out_     MIB_SERVER_HANDLE *     phMibServer
);

VOID APIENTRY
MprAdminMIBServerDisconnect(
    _In_      MIB_SERVER_HANDLE       hMibServer
);

DWORD APIENTRY
MprAdminMIBEntryCreate(
    _In_      MIB_SERVER_HANDLE       hMibServer,
    _In_      DWORD                   dwPid,
    _In_      DWORD                   dwRoutingPid,
    _In_      LPVOID                  lpEntry,
    _In_      DWORD                   dwEntrySize
);

DWORD APIENTRY
MprAdminMIBEntryDelete(
    _In_      MIB_SERVER_HANDLE       hMibServer,
    _In_      DWORD                   dwProtocolId,
    _In_      DWORD                   dwRoutingPid,
    _In_      LPVOID                  lpEntry,
    _In_      DWORD                   dwEntrySize
);

DWORD APIENTRY
MprAdminMIBEntrySet(
    _In_      MIB_SERVER_HANDLE       hMibServer,
    _In_      DWORD                   dwProtocolId,
    _In_      DWORD                   dwRoutingPid,
    _In_      LPVOID                  lpEntry,
    _In_      DWORD                   dwEntrySize
);

DWORD APIENTRY
MprAdminMIBEntryGet(
    _In_      MIB_SERVER_HANDLE       hMibServer,
    _In_      DWORD                   dwProtocolId,
    _In_      DWORD                   dwRoutingPid,
    _In_      LPVOID                  lpInEntry,
    _In_      DWORD                   dwInEntrySize,
    _Out_     LPVOID*                 lplpOutEntry,
    _Out_     LPDWORD                 lpOutEntrySize
);

DWORD APIENTRY
MprAdminMIBEntryGetFirst(
    _In_      MIB_SERVER_HANDLE       hMibServer,
    _In_      DWORD                   dwProtocolId,
    _In_      DWORD                   dwRoutingPid,
    _In_      LPVOID                  lpInEntry,
    _In_      DWORD                   dwInEntrySize,
    _Out_     LPVOID*                 lplpOutEntry,
    _Out_     LPDWORD                 lpOutEntrySize
);

DWORD APIENTRY
MprAdminMIBEntryGetNext(
    _In_      MIB_SERVER_HANDLE       hMibServer,
    _In_      DWORD                   dwProtocolId,
    _In_      DWORD                   dwRoutingPid,
    _In_      LPVOID                  lpInEntry,
    _In_      DWORD                   dwInEntrySize,
    _Out_     LPVOID*                 lplpOutEntry,
    _Out_     LPDWORD                 lpOutEntrySize
);

DWORD APIENTRY
MprAdminMIBGetTrapInfo(
    _In_      MIB_SERVER_HANDLE       hMibServer,
    _In_      DWORD                   dwProtocolId,
    _In_      DWORD                   dwRoutingPid,
    _In_      LPVOID                  lpInData,
    _In_      DWORD                   dwInDataSize,
    _Out_     LPVOID*                 lplpOutData,
    _Inout_  LPDWORD                 lpOutDataSize
);

DWORD APIENTRY
MprAdminMIBSetTrapInfo(
    _In_      DWORD                   dwProtocolId,
    _In_      DWORD                   dwRoutingPid,
    _In_      HANDLE                  hEvent,
    _In_      LPVOID                  lpInData,
    _In_      DWORD                   dwInDataSize,
    _Out_     LPVOID*                 lplpOutData,
    _Inout_  LPDWORD                 lpOutDataSize
);

DWORD APIENTRY
MprAdminMIBBufferFree(
    _In_      LPVOID                  pBuffer
);

//
// Configuration APIs
//

DWORD APIENTRY
MprConfigServerInstall(
    _In_      DWORD                   dwLevel,
    _In_      PVOID                   pBuffer
);

DWORD APIENTRY
MprConfigServerConnect(
    _In_opt_  LPWSTR                  lpwsServerName,
    _Out_     HANDLE*                 phMprConfig
);

VOID APIENTRY
MprConfigServerDisconnect(
    _In_      HANDLE                  hMprConfig
);

DWORD APIENTRY
MprConfigServerRefresh(
    _In_      HANDLE                  hMprConfig
    );

DWORD APIENTRY
MprConfigBufferFree(
    _In_      LPVOID                  pBuffer
);

DWORD APIENTRY
MprConfigServerGetInfo(
    _In_      HANDLE                  hMprConfig,
    _In_      DWORD                   dwLevel,
    _Out_     LPBYTE *                lplpbBuffer
);

#if (WINVER >= 0x501)
DWORD APIENTRY
MprConfigServerSetInfo(
    _In_      MPR_SERVER_HANDLE       hMprServer,
    _In_      DWORD                   dwLevel,
    _In_reads_bytes_(dwLevel==1?sizeof(MPR_SERVER_1):sizeof(MPR_SERVER_2)) LPBYTE lpbBuffer
);
#endif

DWORD APIENTRY
MprConfigServerBackup(
    _In_      HANDLE                  hMprConfig,
    _In_      LPWSTR                  lpwsPath
);

DWORD APIENTRY
MprConfigServerRestore(
    _In_      HANDLE                  hMprConfig,
    _In_ IN      LPWSTR                  lpwsPath
);

DWORD APIENTRY
MprConfigTransportCreate(
    _In_      HANDLE                  hMprConfig,
    _In_      DWORD                   dwTransportId,
    _In_opt_  LPWSTR                  lpwsTransportName,
    _In_reads_bytes_(dwGlobalInfoSize)               LPBYTE                  pGlobalInfo,
    _In_      DWORD                   dwGlobalInfoSize,
    _In_reads_bytes_opt_(dwClientInterfaceInfoSize)  LPBYTE                  pClientInterfaceInfo,
    _In_opt_  DWORD                   dwClientInterfaceInfoSize,
    _In_opt_  LPWSTR                  lpwsDLLPath,
    _Out_     HANDLE*                 phRouterTransport
);

DWORD APIENTRY
MprConfigTransportDelete(
    _In_      HANDLE                  hMprConfig,
    _In_      HANDLE                  hRouterTransport
);

DWORD APIENTRY
MprConfigTransportGetHandle(
    _In_      HANDLE                  hMprConfig,
    _In_      DWORD                   dwTransportId,
    _Out_     HANDLE*                 phRouterTransport
);

DWORD APIENTRY
MprConfigTransportSetInfo(
    _In_      HANDLE                  hMprConfig,
    _In_      HANDLE                  hRouterTransport,
    _In_reads_bytes_opt_(dwGlobalInfoSize)          LPBYTE                pGlobalInfo,
    _In_opt_      DWORD               dwGlobalInfoSize,
    _In_reads_bytes_opt_(dwClientInterfaceInfoSize) LPBYTE                pClientInterfaceInfo,
    _In_opt_      DWORD               dwClientInterfaceInfoSize,
    _In_opt_      LPWSTR              lpwsDLLPath
);

DWORD APIENTRY
MprConfigTransportGetInfo(
    _In_          HANDLE                 hMprConfig,
    _In_          HANDLE                 hRouterTransport,
    _Inout_opt_   LPBYTE*                ppGlobalInfo,
    _Out_opt_     LPDWORD                lpdwGlobalInfoSize,
    _Inout_opt_   LPBYTE*                ppClientInterfaceInfo,
    _Out_opt_     LPDWORD                lpdwClientInterfaceInfoSize,
    _Outptr_opt_ _Inout_ LPWSTR*      lplpwsDLLPath 
);

DWORD APIENTRY
MprConfigTransportEnum(
    _In_      HANDLE                  hMprConfig,
    _In_      DWORD                   dwLevel,
    _Outptr_result_bytebuffer_maybenull_((*lpdwEntriesRead)*sizeof(MPR_TRANSPORT_0)) LPBYTE*  lplpBuffer,
    _In_      DWORD                   dwPrefMaxLen,
    _Out_     LPDWORD                 lpdwEntriesRead,
    _Out_     LPDWORD                 lpdwTotalEntries,
    _Inout_opt_ LPDWORD               lpdwResumeHandle
);

DWORD APIENTRY
MprConfigInterfaceCreate(
    _In_      HANDLE                  hMprConfig,
    _In_      DWORD                   dwLevel,
    _In_reads_bytes_(dwLevel==1?sizeof(MPR_INTERFACE_1):sizeof(MPR_INTERFACE_0)) LPBYTE lpbBuffer,
    _Out_     HANDLE*                 phRouterInterface
);

DWORD APIENTRY
MprConfigInterfaceDelete(
    _In_      HANDLE                  hMprConfig,
    _In_      HANDLE                  hRouterInterface
);

DWORD APIENTRY
MprConfigInterfaceGetHandle(
    _In_      HANDLE                  hMprConfig,
    _In_      LPWSTR                  lpwsInterfaceName,
    _Out_     HANDLE*                 phRouterInterface
);

DWORD APIENTRY
MprConfigInterfaceGetInfo(
    _In_      HANDLE                  hMprConfig,
    _In_      HANDLE                  hRouterInterface,
    _In_      DWORD                   dwLevel,
    _Inout_   LPBYTE*                 lplpBuffer,
    _Out_     LPDWORD                 lpdwBufferSize
);

DWORD APIENTRY
MprConfigInterfaceSetInfo(
    _In_      HANDLE                  hMprConfig,
    _In_      HANDLE                  hRouterInterface,
    _In_      DWORD                   dwLevel,
    _In_reads_bytes_(dwLevel==1?sizeof(MPR_INTERFACE_1):sizeof(MPR_INTERFACE_0))      LPBYTE                  lpbBuffer
);

DWORD APIENTRY
MprConfigInterfaceEnum(
    _In_      HANDLE                  hMprConfig,
    _In_      DWORD                   dwLevel,
    _Outptr_result_bytebuffer_maybenull_((*lpdwEntriesRead)*sizeof( MPR_INTERFACE_0)) LPBYTE*  lplpBuffer,
    _In_      DWORD                   dwPrefMaxLen,
    _Out_     LPDWORD                 lpdwEntriesRead,
    _Out_     LPDWORD                 lpdwTotalEntries,
    _Inout_opt_ LPDWORD               lpdwResumeHandle
);

DWORD APIENTRY
MprConfigInterfaceTransportAdd(
    _In_      HANDLE                  hMprConfig,
    _In_      HANDLE                  hRouterInterface,
    _In_      DWORD                   dwTransportId,
    _In_opt_  LPWSTR                  lpwsTransportName,
    _In_reads_bytes_(dwInterfaceInfoSize)      LPBYTE                  pInterfaceInfo,
    _In_      DWORD                   dwInterfaceInfoSize,
    _Out_     HANDLE*                 phRouterIfTransport
);

DWORD APIENTRY
MprConfigInterfaceTransportRemove(
    _In_      HANDLE                  hMprConfig,
    _In_      HANDLE                  hRouterInterface,
    _In_      HANDLE                  hRouterIfTransport
);

DWORD APIENTRY
MprConfigInterfaceTransportGetHandle(
    _In_      HANDLE                  hMprConfig,
    _In_      HANDLE                  hRouterInterface,
    _In_      DWORD                   dwTransportId,
    _Out_     HANDLE*                 phRouterIfTransport
);

DWORD APIENTRY
MprConfigInterfaceTransportGetInfo(
    _In_      HANDLE                  hMprConfig,
    _In_      HANDLE                  hRouterInterface,
    _In_      HANDLE                  hRouterIfTransport,
    _Outptr_opt_result_bytebuffer_(*lpdwInterfaceInfoSize)   LPBYTE*         ppInterfaceInfo,
    _Out_opt_ LPDWORD     lpdwInterfaceInfoSize
);

DWORD APIENTRY
MprConfigInterfaceTransportSetInfo(
    _In_      HANDLE                  hMprConfig,
    _In_      HANDLE                  hRouterInterface,
    _In_      HANDLE                  hRouterIfTransport,
    _In_reads_bytes_opt_(dwInterfaceInfoSize)  LPBYTE                  pInterfaceInfo,         
   _In_       DWORD                  dwInterfaceInfoSize         
);

DWORD APIENTRY
MprConfigInterfaceTransportEnum(
    _In_      HANDLE                  hMprConfig,
    _In_      HANDLE                  hRouterInterface,
    _In_      DWORD                   dwLevel,
    _Outptr_result_bytebuffer_maybenull_((*lpdwEntriesRead)*sizeof(MPR_IFTRANSPORT_0)) LPBYTE*  lplpBuffer, // MPR_IFTRANSPORT_0
    _In_      DWORD                   dwPrefMaxLen,
    _Out_     LPDWORD                 lpdwEntriesRead,
    _Out_     LPDWORD                 lpdwTotalEntries,
    _Inout_opt_ LPDWORD               lpdwResumeHandle            
);

DWORD APIENTRY
MprConfigGetFriendlyName(
    _In_      HANDLE                  hMprConfig,
    _In_      PWSTR                   pszGuidName,
    _Out_writes_bytes_(dwBufferSize) PWCHAR pszBuffer,
    _In_      DWORD                   dwBufferSize);

DWORD APIENTRY
MprConfigGetGuidName(
    _In_      HANDLE                  hMprConfig,
    _In_      PWSTR                   pszFriendlyName,
    _Out_writes_bytes_(dwBufferSize) PWCHAR pszBuffer,
    _In_      DWORD                   dwBufferSize);

#if(WINVER >= 0x0600)
DWORD APIENTRY
MprConfigFilterGetInfo(
    _In_      HANDLE                  hMprConfig,
    _In_      DWORD                   dwLevel,
    _In_      DWORD                   dwTransportId, 
    _Inout_updates_bytes_(sizeof(MPR_FILTER_0))     LPBYTE                  lpBuffer);

DWORD APIENTRY
MprConfigFilterSetInfo(
    _In_      HANDLE                  hMprConfig,
    _In_      DWORD                   dwLevel,
    _In_      DWORD                   dwTransportId, 
    _In_      LPBYTE                  lpBuffer);
#endif /* WINVER >= 0x0600 */

//
// Information block APIs
//

DWORD APIENTRY
MprInfoCreate(
    _In_      DWORD                   dwVersion,
    _Out_     LPVOID*                 lplpNewHeader
);

DWORD APIENTRY
MprInfoDelete(
   _In_     LPVOID                    lpHeader
);

DWORD APIENTRY
MprInfoRemoveAll(
    _In_      LPVOID                  lpHeader,
    _Out_     LPVOID*                 lplpNewHeader
);

DWORD APIENTRY
MprInfoDuplicate(
    _In_      LPVOID                  lpHeader,
    _Out_     LPVOID*                 lplpNewHeader
);

DWORD APIENTRY
MprInfoBlockAdd(
    _In_      LPVOID                  lpHeader,
    _In_      DWORD                   dwInfoType,
    _In_      DWORD                   dwItemSize,
    _In_      DWORD                   dwItemCount,
    _In_reads_bytes_(dwItemSize*dwItemCount)      LPBYTE                  lpItemData,
    _Out_     LPVOID*                 lplpNewHeader
);

DWORD APIENTRY
MprInfoBlockRemove(
    _In_      LPVOID                  lpHeader,
    _In_      DWORD                   dwInfoType,
    _Out_     LPVOID*                 lplpNewHeader
);

DWORD APIENTRY
MprInfoBlockSet(
    _In_      LPVOID                  lpHeader,
    _In_      DWORD                   dwInfoType,
    _In_      DWORD                   dwItemSize,
    _In_      DWORD                   dwItemCount,
    _In_reads_bytes_(dwItemSize*dwItemCount)      LPBYTE                  lpItemData,
    _Out_     LPVOID*                 lplpNewHeader
);

DWORD APIENTRY
MprInfoBlockFind(
    _In_      LPVOID                  lpHeader,
    _In_      DWORD                   dwInfoType,
    _Out_     LPDWORD                 lpdwItemSize,       // OPTIONAL
    _Out_     LPDWORD                 lpdwItemCount,      // OPTIONAL
    _Outptr_result_bytebuffer_((*lpdwItemSize)*(*lpdwItemCount))     LPBYTE*                 lplpItemData        // OPTIONAL
);

DWORD APIENTRY
MprInfoBlockQuerySize(
    _In_      LPVOID                  lpHeader
);

//
// BOOL APIENTRY
// MprInfoBlockExists(
//     _In_      LPVOID                  lpHeader,
//     _In_      DWORD                   dwInfoType
// );
//


#define MprInfoBlockExists(h,t) \
    (MprInfoBlockFind((h),(t),NULL,NULL,NULL) == NO_ERROR)

#ifdef __cplusplus
}
#endif


#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_PKG_CMDTOOLS) */
#pragma endregion

#endif // __ROUTING_MPRADMIN_H__

