/*++

Copyright (c) Microsoft Corporation. All rights reserved.

Module Name:

    mprerror.h

Abstract:

    Router specific error codes
    
--*/

//
// Don't change the comments following the manifest constants without
// understanding how mapmsg works.
//

#ifndef _MPRERROR_H_
#define _MPRERROR_H_

#if _MSC_VER > 1000
#pragma once
#endif
#include <winapifamily.h>

#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)


#define ROUTEBASE 900
#define SUCCESS 0

#define ERROR_ROUTER_STOPPED                        (ROUTEBASE+0)
/*
 * The router is not running.%0
 */

#define ERROR_ALREADY_CONNECTED                     (ROUTEBASE+1)
/*
 * The interface is already connected.%0
 */

#define ERROR_UNKNOWN_PROTOCOL_ID                   (ROUTEBASE+2)
/*
 * The specified protocol identifier is not known to the router.%0
 */

#define ERROR_DDM_NOT_RUNNING                       (ROUTEBASE+3)
/*
 * The Demand-dial Interface Manager is not running.%0
 */

#define ERROR_INTERFACE_ALREADY_EXISTS              (ROUTEBASE+4)
/*
 * An interface with this name is already registered with the router.%0
 */

#define ERROR_NO_SUCH_INTERFACE                     (ROUTEBASE+5)
/*
 * An interface with this name is not registered with the router.%0
 */

#define ERROR_INTERFACE_NOT_CONNECTED               (ROUTEBASE+6)
/*
 * The interface is not connected.%0
 */

#define ERROR_PROTOCOL_STOP_PENDING                 (ROUTEBASE+7)
/*
 * The specified protocol is stopping.%0
 */

#define ERROR_INTERFACE_CONNECTED                   (ROUTEBASE+8)
/*
 * The interface is connected and hence cannot be deleted.%0
 */

#define ERROR_NO_INTERFACE_CREDENTIALS_SET          (ROUTEBASE+9)
/*
 * The interface credentials have not been set.%0
 */

#define ERROR_ALREADY_CONNECTING                    (ROUTEBASE+10)
/*
 * This interface is already in the process of connecting.%0
 */

#define ERROR_UPDATE_IN_PROGRESS                    (ROUTEBASE+11)
/*
 * An update of routing information on this interface is already in progress.%0
 */

#define ERROR_INTERFACE_CONFIGURATION               (ROUTEBASE+12)
/*
 * The interface confugration in invalid. There is already another interface that is connected to the same inteface on the remote router.%0
 */

#define ERROR_NOT_CLIENT_PORT                       (ROUTEBASE+13)
/*
 * A Remote Access Client attempted to connect over a port that was reserved for Routers only.%0
 */

#define ERROR_NOT_ROUTER_PORT                       (ROUTEBASE+14)
/*
 * A Demand Dial Router attempted to connect over a port that was reserved for Remote Access Clients only.%0
 */

#define ERROR_CLIENT_INTERFACE_ALREADY_EXISTS       (ROUTEBASE+15)
/*
 * The client interface with this name already exists and is currently connected.%0
 */

#define ERROR_INTERFACE_DISABLED                    (ROUTEBASE+16)
/*
 * The interface is in a disabled state.%0
 */

#define ERROR_AUTH_PROTOCOL_REJECTED                (ROUTEBASE+17)
/*
 * The authentication protocol was rejected by the remote peer.%0
 */

#define ERROR_NO_AUTH_PROTOCOL_AVAILABLE            (ROUTEBASE+18)
/*
 * There are no authentication protocols available for use.%0
 */

#define ERROR_PEER_REFUSED_AUTH                     (ROUTEBASE+19)
/*
 * The connection could not be established because the authentication protocol used by the RAS/VPN server to verify your username and password could not be matched with the settings in your connection profile.%0
 */

#define ERROR_REMOTE_NO_DIALIN_PERMISSION           (ROUTEBASE+20)
/*
 * The remote account does not have Remote Access permission.%0
 */

#define ERROR_REMOTE_PASSWD_EXPIRED                 (ROUTEBASE+21)
/*
 * The remote account has expired.%0
 */

#define ERROR_REMOTE_ACCT_DISABLED                  (ROUTEBASE+22)
/*
 * The remote account is disabled.%0
 */

#define ERROR_REMOTE_RESTRICTED_LOGON_HOURS         (ROUTEBASE+23)
/*
 * The remote account is not permitted to logon at this time of day.%0
 */

#define ERROR_REMOTE_AUTHENTICATION_FAILURE         (ROUTEBASE+24)
/*
 * Access was denied to the remote peer  because username and/or password is invalid on the domain.%0
 */

#define ERROR_INTERFACE_HAS_NO_DEVICES              (ROUTEBASE+25)
/*
 * There are no routing enabled ports available for use by this demand dial interface.%0
 */

#define ERROR_IDLE_DISCONNECTED                     (ROUTEBASE+26)
/*
 * The port has been disconnected due to inactivity.%0
 */

#define ERROR_INTERFACE_UNREACHABLE                 (ROUTEBASE+27)
/*
 * The interface is not reachable at this time.%0
 */

#define ERROR_SERVICE_IS_PAUSED                     (ROUTEBASE+28)
/*
 * The Demand Dial service is in a paused state.%0
 */

#define ERROR_INTERFACE_DISCONNECTED                (ROUTEBASE+29)
/*
 * The interface has been disconnected by the administrator.%0
 */

#define ERROR_AUTH_SERVER_TIMEOUT                   (ROUTEBASE+30)
/*
 * The authentication server did not respond to authentication requests in a timely fashion.%0
 */

#define ERROR_PORT_LIMIT_REACHED                    (ROUTEBASE+31)
/*
 * The maximum number of ports allowed for use in the multilinked connection has been reached.%0
 */

#define ERROR_PPP_SESSION_TIMEOUT                   (ROUTEBASE+32)
/*
 * The connection time limit for the user has been reached.%0
 */

#define ERROR_MAX_LAN_INTERFACE_LIMIT               (ROUTEBASE+33)
/*
 * The maximum limit on the number of LAN interfaces supported has been reached.%0
 */

#define ERROR_MAX_WAN_INTERFACE_LIMIT               (ROUTEBASE+34)
/*
 * The maximum limit on the number of Demand Dial interfaces supported has been reached.%0
 */

#define ERROR_MAX_CLIENT_INTERFACE_LIMIT            (ROUTEBASE+35)
/*
 * The maximum limit on the number of Remote Access clients supported has been reached.%0
 */

#define ERROR_BAP_DISCONNECTED                      (ROUTEBASE+36)
/*
 * The port has been disconnected due to the BAP policy.%0
 */

#define ERROR_USER_LIMIT                            (ROUTEBASE+37)
/*
 * Because another connection of your type is in use, the incoming connection cannot accept your connection request.%0
 */

#define ERROR_NO_RADIUS_SERVERS                     (ROUTEBASE+38)
/*
 * No RADIUS servers were located on the network.%0
 */

#define ERROR_INVALID_RADIUS_RESPONSE               (ROUTEBASE+39)
/*
 * An invalid response was received from the RADIUS authentication server.
 * Make sure that the case sensitive secret pasword for the RADIUS server is set correctly.%0
 */

#define ERROR_DIALIN_HOURS_RESTRICTION              (ROUTEBASE+40)
/*
 * You do not have permission to connect at this time.%0
 */

#define ERROR_ALLOWED_PORT_TYPE_RESTRICTION         (ROUTEBASE+41)
/*
 * You do not have permission to connect using the current device type.%0
 */

#define ERROR_AUTH_PROTOCOL_RESTRICTION             (ROUTEBASE+42)
/*
 * The connection could not be established because the authentication method used by your connection profile is not permitted for use by an access policy configured on the RAS/VPN server. Specifically, this could be due to configuration differences between the authentication method selected on the RAS/VPN server and the access policy configured for it.%0
 */

#define ERROR_BAP_REQUIRED                          (ROUTEBASE+43)
/*
 * BAP is required for this user.%0
 */

#define ERROR_DIALOUT_HOURS_RESTRICTION             (ROUTEBASE+44)
/*
 * The interface is not allowed to connect at this time.%0
 */

#define ERROR_ROUTER_CONFIG_INCOMPATIBLE            (ROUTEBASE+45)
/*
 * The saved router configuration is incompatible with the current router.%0
 */

#define WARNING_NO_MD5_MIGRATION                    (ROUTEBASE+46)
/*
 * RemoteAccess has detected older format user accounts that will not be
 * migrated automatically.  To migrate these manually, run XXXX.
 */

#define ERROR_PROTOCOL_ALREADY_INSTALLED            (ROUTEBASE+48)
/*
 * The transport is already installed with the router.%0
 */

#define ERROR_INVALID_SIGNATURE_LENGTH              (ROUTEBASE+49)
/*
 * Received invalid signature length in packet from RADIUS server.%0
 */

#define ERROR_INVALID_SIGNATURE                     (ROUTEBASE+50)
/*
 * Received invalid signature in packet from RADIUS server.%0
 */

#define ERROR_NO_SIGNATURE                          (ROUTEBASE+51)
/*
 * Did not receive signature along with EAPMessage from RADIUS server.%0
 */

#define ERROR_INVALID_PACKET_LENGTH_OR_ID           (ROUTEBASE+52)
/*
 * Received packet with invalid length or Id from RADIUS server.%0
 */

#define ERROR_INVALID_ATTRIBUTE_LENGTH              (ROUTEBASE+53)
/*
 * Received packet with attribute with invalid length from RADIUS server.%0
 */

#define ERROR_INVALID_PACKET                        (ROUTEBASE+54)
/*
 * Received invalid packet from RADIUS server.%0
 */

#define ERROR_AUTHENTICATOR_MISMATCH                (ROUTEBASE+55)
/*
 * Authenticator does not match in packet from RADIUS server.%0
 */

#define ERROR_REMOTEACCESS_NOT_CONFIGURED           (ROUTEBASE+56)
/*
 * Routing and Remote access server is either not configured or not running.%0
 */

#define ROUTEBASEEND                                (ROUTEBASE+57)


#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion

#endif // _MPRERROR_H_
