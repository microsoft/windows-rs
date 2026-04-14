/*++

Copyright (c) Microsoft Corporation. All rights reserved.

Module Name:

    raserror.h

Abstract:

    RAS specific error codes

--*/

#ifndef _RASERROR_H_
#define _RASERROR_H_

#if _MSC_VER > 1000
#pragma once
#endif
#include <winapifamily.h>

#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)


#define RASBASE 600
#define SUCCESS 0


#define PENDING                              (RASBASE+0)
/*
 * An operation is pending.%0
 */

#define ERROR_INVALID_PORT_HANDLE            (RASBASE+1)
/*
 * An invalid port handle was detected.%0
 */

#define ERROR_PORT_ALREADY_OPEN              (RASBASE+2)
/*
 * The specified port is already open.%0
 */

#define ERROR_BUFFER_TOO_SMALL               (RASBASE+3)
/*
 * The caller's buffer is too small.%0
 */

#define ERROR_WRONG_INFO_SPECIFIED           (RASBASE+4)
/*
 * Incorrect information was specified.%0
 */

#if (WINVER < 0x600)
//
// The port information cannot be set.
//
// This error was obsoleted in the Windows Vista  release.
//
#define ERROR_CANNOT_SET_PORT_INFO           (RASBASE+5)
/*
 * %0
 */
#endif

#define ERROR_PORT_NOT_CONNECTED             (RASBASE+6)
/*
 * The specified port is not connected.%0
 */

#if (WINVER < 0x600)
//
// An invalid event was detected.
//
// This error was obsoleted in the Windows Vista  release.
//
#define ERROR_EVENT_INVALID                  (RASBASE+7)
/*
 * %0
 */
#endif

#define ERROR_DEVICE_DOES_NOT_EXIST          (RASBASE+8)
/*
 * A device was specified that does not exist.%0
 */

#define ERROR_DEVICETYPE_DOES_NOT_EXIST      (RASBASE+9)
/*
 * A device type was specified that does not exist.%0
 */

#define ERROR_BUFFER_INVALID                 (RASBASE+10)
/*
 * An invalid buffer was specified.%0
 */

#if (WINVER < 0x600)
//
// A route was specified that is not available.
//
// This error was obsoleted in the Windows Vista  release.
//
#define ERROR_ROUTE_NOT_AVAILABLE            (RASBASE+11)
/*
 * %0
 */
#endif

#define ERROR_ROUTE_NOT_ALLOCATED            (RASBASE+12)
/*
 * A route was specified that is not allocated.%0
 */

#if (WINVER < 0x600)
//
// An invalid compression was specified.
//
// This error was obsoleted in the Windows Vista  release.
//
#define ERROR_INVALID_COMPRESSION_SPECIFIED  (RASBASE+13)
/*
 * %0
 */

//
// There were insufficient buffers available.
//
// This error was obsoleted in the Windows Vista  release.
//
#define ERROR_OUT_OF_BUFFERS                 (RASBASE+14)
/*
 * %0
 */
#endif

#define ERROR_PORT_NOT_FOUND                 (RASBASE+15)
/*
 * The specified port was not found.%0
 */

#define ERROR_ASYNC_REQUEST_PENDING          (RASBASE+16)
/*
 * An asynchronous request is pending.%0
 */

#define ERROR_ALREADY_DISCONNECTING          (RASBASE+17)
/*
 * The modem (or other connecting device) is already disconnecting.%0
 */

#define ERROR_PORT_NOT_OPEN                  (RASBASE+18)
/*
 * The specified port is not open.%0
 */

#define ERROR_PORT_DISCONNECTED              (RASBASE+19)
/*
 * A connection to the remote computer could not be established, so the port used for this connection was closed.%0
 */

#if (WINVER < 0x600)
//
//  No endpoints could be determined.
//
// This error was obsoleted in the Windows Vista  release.
//
#define ERROR_NO_ENDPOINTS                   (RASBASE+20)
/*
 * %0
 */
#endif

#define ERROR_CANNOT_OPEN_PHONEBOOK          (RASBASE+21)
/*
 * The system could not open the phone book file.%0
 */

#define ERROR_CANNOT_LOAD_PHONEBOOK          (RASBASE+22)
/*
 * The system could not load the phone book file.%0
 */

#define ERROR_CANNOT_FIND_PHONEBOOK_ENTRY    (RASBASE+23)
/*
 * The system could not find the phone book entry for this connection.%0
 */

#define ERROR_CANNOT_WRITE_PHONEBOOK         (RASBASE+24)
/*
 * The system could not update the phone book file.%0
 */

#define ERROR_CORRUPT_PHONEBOOK              (RASBASE+25)
/*
 * The system found invalid information in the phone book file.%0
 */

#if (WINVER < 0x600)
//
// A string could not be loaded.
//
// This error was obsoleted in the Windows Vista  release.
//
#define ERROR_CANNOT_LOAD_STRING             (RASBASE+26)
/*
 * %0
 */
#endif

#define ERROR_KEY_NOT_FOUND                  (RASBASE+27)
/*
 * A key could not be found.%0
 */

#define ERROR_DISCONNECTION                  (RASBASE+28)
/*
 * The connection was terminated by the remote computer before it could be completed.%0
 */

#define ERROR_REMOTE_DISCONNECTION           (RASBASE+29)
/*
 * The connection was closed by the remote computer.%0
 */

#define ERROR_HARDWARE_FAILURE               (RASBASE+30)
/*
 * The modem (or other connecting device) was disconnected due to hardware failure.%0
 */

#define ERROR_USER_DISCONNECTION             (RASBASE+31)
/*
 * The user disconnected the modem (or other connecting device).%0
 */

#define ERROR_INVALID_SIZE                   (RASBASE+32)
/*
 * An incorrect structure size was detected.%0
 */

#define ERROR_PORT_NOT_AVAILABLE             (RASBASE+33)
/*
 * The modem (or other connecting device) is already in use or is not configured properly.%0
 */

#if (WINVER < 0x600)
//
// Your computer could not be registered on the remote network.
//
// This error was obsoleted in the Windows Vista  release.
//
#define ERROR_CANNOT_PROJECT_CLIENT          (RASBASE+34)
/*
 * %0
 */
#endif

#define ERROR_UNKNOWN                        (RASBASE+35)
/*
 * There was an unknown error.%0
 */

#define ERROR_WRONG_DEVICE_ATTACHED          (RASBASE+36)
/*
 * The device attached to the port is not the one expected.%0
 */

#if (WINVER < 0x600)
//
// A string was detected that could not be converted.
//
// This error was obsoleted in the Windows Vista  release.
//
#define ERROR_BAD_STRING                     (RASBASE+37)
/*
 * %0
 */
#endif

#define ERROR_REQUEST_TIMEOUT                (RASBASE+38)
/*
 * The remote server is not responding in a timely fashion.%0
 */

#if (WINVER < 0x600)
//
// No asynchronous net is available.
//
// This error was obsoleted in the Windows Vista  release.
//
#define ERROR_CANNOT_GET_LANA                (RASBASE+39)
/*
 * %0
 */

//
// An error has occurred involving NetBIOS.
//
// This error was obsoleted in the Windows Vista  release.
//
#define ERROR_NETBIOS_ERROR                  (RASBASE+40)
/*
 * %0
 */

//
// The server cannot allocate NetBIOS resources needed to support the client.
//
// This error was obsoleted in the Windows Vista  release.
//
#define ERROR_SERVER_OUT_OF_RESOURCES        (RASBASE+41)
/*
 * %0
 */

//
// One of your computer's NetBIOS names is already registered on the remote network.
//
// This error was obsoleted in the Windows Vista  release.
//
#define ERROR_NAME_EXISTS_ON_NET             (RASBASE+42)
/*
 * %0
 */

//
// A network adapter at the server failed.
//
// This error was obsoleted in the Windows Vista  release.
//
#define ERROR_SERVER_GENERAL_NET_FAILURE     (RASBASE+43)
/*
 * %0
 */

//
// You will not receive network message popups.
//
// This error was obsoleted in the Windows Vista  release.
//
#define WARNING_MSG_ALIAS_NOT_ADDED          (RASBASE+44)
/*
 * %0
 */
#endif

#define ERROR_AUTH_INTERNAL                  (RASBASE+45)
/*
 * There was an internal authentication error.%0
 */

#define ERROR_RESTRICTED_LOGON_HOURS         (RASBASE+46)
/*
 * The account is not permitted to log on at this time of day.%0
 */

#define ERROR_ACCT_DISABLED                  (RASBASE+47)
/*
 * The account is disabled.%0
 */

#define ERROR_PASSWD_EXPIRED                 (RASBASE+48)
/*
 * The password for this account has expired.%0
 */

#define ERROR_NO_DIALIN_PERMISSION           (RASBASE+49)
/*
 * The account does not have permission to dial in.%0
 */

#if (WINVER < 0x600)
//
// The remote access server is not responding.
//
// This error was obsoleted in the Windows Vista  release.
//
#define ERROR_SERVER_NOT_RESPONDING          (RASBASE+50)
/*
 * %0
 */
#endif

#define ERROR_FROM_DEVICE                    (RASBASE+51)
/*
 * The modem (or other connecting device) has reported an error.%0
 */

#define ERROR_UNRECOGNIZED_RESPONSE          (RASBASE+52)
/*
 * There was an unrecognized response from the modem (or other connecting device).%0
 */

#define ERROR_MACRO_NOT_FOUND                (RASBASE+53)
/*
 * A macro required by the modem (or other connecting device) was not found in the device.INF file.%0
 */

#define ERROR_MACRO_NOT_DEFINED              (RASBASE+54)
/*
 * A command or response in the device.INF file section refers to an undefined macro.%0
 */

#define ERROR_MESSAGE_MACRO_NOT_FOUND        (RASBASE+55)
/*
 * The <message> macro was not found in the device.INF file section.%0
 */

#define ERROR_DEFAULTOFF_MACRO_NOT_FOUND     (RASBASE+56)
/*
 * The <defaultoff> macro in the device.INF file section contains an undefined macro.%0
 */

#define ERROR_FILE_COULD_NOT_BE_OPENED       (RASBASE+57)
/*
 * The device.INF file could not be opened.%0
 */

#define ERROR_DEVICENAME_TOO_LONG            (RASBASE+58)
/*
 * The device name in the device.INF or media.INI file is too long.%0
 */

#define ERROR_DEVICENAME_NOT_FOUND           (RASBASE+59)
/*
 * The media.INI file refers to an unknown device name.%0
 */

#define ERROR_NO_RESPONSES                   (RASBASE+60)
/*
 * The device.INF file contains no responses for the command.%0
 */

#define ERROR_NO_COMMAND_FOUND               (RASBASE+61)
/*
 * The device.INF file is missing a command.%0
 */

#define ERROR_WRONG_KEY_SPECIFIED            (RASBASE+62)
/*
 * There was an attempt to set a macro not listed in device.INF file section.%0
 */

#define ERROR_UNKNOWN_DEVICE_TYPE            (RASBASE+63)
/*
 * The media.INI file refers to an unknown device type.%0
 */

#define ERROR_ALLOCATING_MEMORY              (RASBASE+64)
/*
 * The system has run out of memory.%0
 */

#define ERROR_PORT_NOT_CONFIGURED            (RASBASE+65)
/*
 * The modem (or other connecting device) is not properly configured.%0
 */

#define ERROR_DEVICE_NOT_READY               (RASBASE+66)
/*
 * The modem (or other connecting device) is not functioning.%0
 */

#define ERROR_READING_INI_FILE               (RASBASE+67)
/*
 * The system was unable to read the media.INI file.%0
 */

#define ERROR_NO_CONNECTION                  (RASBASE+68)
/*
 * The connection was terminated.%0
 */

#define ERROR_BAD_USAGE_IN_INI_FILE          (RASBASE+69)
/*
 * The usage parameter in the media.INI file is invalid.%0
 */

#define ERROR_READING_SECTIONNAME            (RASBASE+70)
/*
 * The system was unable to read the section name from the media.INI file.%0
 */

#define ERROR_READING_DEVICETYPE             (RASBASE+71)
/*
 * The system was unable to read the device type from the media.INI file.%0
 */

#define ERROR_READING_DEVICENAME             (RASBASE+72)
/*
 * The system was unable to read the device name from the media.INI file.%0
 */

#define ERROR_READING_USAGE                  (RASBASE+73)
/*
 * The system was unable to read the usage from the media.INI file.%0
 */

#if (WINVER < 0x600)
//
// The system was unable to read the maximum connection BPS rate from the media.INI file.
//
// This error was obsoleted in the Windows Vista  release.
//
#define ERROR_READING_MAXCONNECTBPS          (RASBASE+74)
/*
 * %0
 */

//
// The system was unable to read the maximum carrier connection speed from the media.INI file.
//
// This error was obsoleted in the Windows Vista  release.
//
#define ERROR_READING_MAXCARRIERBPS          (RASBASE+75)
/*
 * %0
 */
#endif

#define ERROR_LINE_BUSY                      (RASBASE+76)
/*
 * The phone line is busy.%0
 */

#define ERROR_VOICE_ANSWER                   (RASBASE+77)
/*
 * A person answered instead of a modem (or other connecting device).%0
 */

#define ERROR_NO_ANSWER                      (RASBASE+78)
/*
 * The remote computer did not respond. To make sure that the server can be reached, ping the remote computer.%0
 */

#define ERROR_NO_CARRIER                     (RASBASE+79)
/*
 * The system could not detect the carrier.%0
 */

#define ERROR_NO_DIALTONE                    (RASBASE+80)
/*
 * There was no dial tone.%0
 */

#if (WINVER < 0x600)
//
// The modem (or other connecting device) reported a general error.
//
// This error was obsoleted in the Windows Vista  release.
//
#define ERROR_IN_COMMAND                     (RASBASE+81)
/*
 * %0
 */

//
// There was an error in writing the section name.
//
// This error was obsoleted in the Windows Vista  release.
//
#define ERROR_WRITING_SECTIONNAME            (RASBASE+82)
/*
 * %0
 */

//
// There was an error in writing the device type.
//
// This error was obsoleted in the Windows Vista  release.
//
#define ERROR_WRITING_DEVICETYPE             (RASBASE+83)
/*
 * %0
 */

//
// There was an error in writing the device name.
//
// This error was obsoleted in the Windows Vista  release.
//
#define ERROR_WRITING_DEVICENAME             (RASBASE+84)
/*
 * %0
 */

//
// There was an error in writing the maximum connection speed.
//
// This error was obsoleted in the Windows Vista  release.
//
#define ERROR_WRITING_MAXCONNECTBPS          (RASBASE+85)
/*
 * %0
 */

//
// There was an error in writing the maximum carrier speed.
//
// This error was obsoleted in the Windows Vista  release.
//
#define ERROR_WRITING_MAXCARRIERBPS          (RASBASE+86)
/*
 * %0
 */

//
// There was an error in writing the usage.
//
// This error was obsoleted in the Windows Vista  release.
//
#define ERROR_WRITING_USAGE                  (RASBASE+87)
/*
 * %0
 */

//
// There was an error in writing the default-off.
//
// This error was obsoleted in the Windows Vista  release.
//
#define ERROR_WRITING_DEFAULTOFF             (RASBASE+88)
/*
 * %0
 */

//
// There was an error in reading the default-off.
//
// This error was obsoleted in the Windows Vista  release.
//
#define ERROR_READING_DEFAULTOFF             (RASBASE+89)
/*
 * %0
 */

//
// ERROR_EMPTY_INI_FILE
//
// This error was obsoleted in the Windows Vista  release.
//
#define ERROR_EMPTY_INI_FILE                 (RASBASE+90)
/*
 * %0
 */
#endif

#define ERROR_AUTHENTICATION_FAILURE         (RASBASE+91)
/*
 * The remote connection was denied because the user name and password combination you provided is not recognized, or the selected authentication protocol is not permitted on the remote access server.%0
 */

#define ERROR_PORT_OR_DEVICE                 (RASBASE+92)
/*
 * There was a hardware failure in the modem (or other connecting device).%0
 */

#define ERROR_NOT_BINARY_MACRO               (RASBASE+93)
/*
 * ERROR_NOT_BINARY_MACRO%0
 */

#define ERROR_DCB_NOT_FOUND                  (RASBASE+94)
/*
 * ERROR_DCB_NOT_FOUND%0
 */

#define ERROR_STATE_MACHINES_NOT_STARTED     (RASBASE+95)
/*
 * The state machines are not started.%0
 */

#define ERROR_STATE_MACHINES_ALREADY_STARTED (RASBASE+96)
/*
 * The state machines are already started.%0
 */

#define ERROR_PARTIAL_RESPONSE_LOOPING       (RASBASE+97)
/*
 * The response looping did not complete.%0
 */

#define ERROR_UNKNOWN_RESPONSE_KEY           (RASBASE+98)
/*
 * A response keyname in the device.INF file is not in the expected format.%0
 */

#define ERROR_RECV_BUF_FULL                  (RASBASE+99)
/*
 * The modem (or other connecting device) response caused a buffer overflow.%0
 */

#define ERROR_CMD_TOO_LONG                   (RASBASE+100)
/*
 * The expanded command in the device.INF file is too long.%0
 */

#define ERROR_UNSUPPORTED_BPS                (RASBASE+101)
/*
 * The modem moved to a connection speed not supported by the COM driver.%0
 */

#define ERROR_UNEXPECTED_RESPONSE            (RASBASE+102)
/*
 * Device response received when none expected.%0
 */

#define ERROR_INTERACTIVE_MODE               (RASBASE+103)
/*
 * The connection needs information from you, but the application does not allow user interaction.%0
 */

#define ERROR_BAD_CALLBACK_NUMBER            (RASBASE+104)
/*
 * The callback number is invalid.%0
 */

#define ERROR_INVALID_AUTH_STATE             (RASBASE+105)
/*
 * The authorization state is invalid.%0
 */

#if (WINVER < 0x600)
//
// ERROR_WRITING_INITBPS
//
// This error was obsoleted in the Windows Vista  release.
//
#define ERROR_WRITING_INITBPS                (RASBASE+106)
/*
 * %0
 */
#endif

#define ERROR_X25_DIAGNOSTIC                 (RASBASE+107)
/*
 * There was an error related to the X.25 protocol.%0
 */

#define ERROR_ACCT_EXPIRED                   (RASBASE+108)
/*
 * The account has expired.%0
 */

#define ERROR_CHANGING_PASSWORD              (RASBASE+109)
/*
 * There was an error changing the password on the domain.  The password might have been too short or might have matched a previously used password.%0
 */

#define ERROR_OVERRUN                        (RASBASE+110)
/*
 * Serial overrun errors were detected while communicating with the modem.%0
 */

#define ERROR_RASMAN_CANNOT_INITIALIZE	     (RASBASE+111)
/*
 * The operation could not finish because it could not start the Remote Access Connection Manager service in time. Please try the operation again.%0
 */

#if (WINVER < 0x600)
//
// The two-way port is initializing.  Wait a few seconds and redial.
//
// This error was obsoleted in the Windows Vista  release.
//
#define ERROR_BIPLEX_PORT_NOT_AVAILABLE      (RASBASE+112)
/*
 * %0
 */
#endif

#define ERROR_NO_ACTIVE_ISDN_LINES           (RASBASE+113)
/*
 * No active ISDN lines are available.%0
 */

#if (WINVER < 0x600)
//
// No ISDN channels are available to make the call.
//
// This error was obsoleted in the Windows Vista  release.
//
#define ERROR_NO_ISDN_CHANNELS_AVAILABLE     (RASBASE+114)
/*
 * %0
 */

//
// Too many errors occurred because of poor phone line quality.
//
// This error was obsoleted in the Windows Vista  release.
//
#define ERROR_TOO_MANY_LINE_ERRORS           (RASBASE+115)
/*
 * %0
 */
#endif

#define ERROR_IP_CONFIGURATION               (RASBASE+116)
/*
 * The Remote Access Service IP configuration is unusable.%0
 */

#define ERROR_NO_IP_ADDRESSES                (RASBASE+117)
/*
 * No IP addresses are available in the static pool of Remote Access Service IP addresses.%0
 */

#define ERROR_PPP_TIMEOUT                    (RASBASE+118)
/*
 * The connection was terminated because the remote computer did not respond in a timely manner.%0
 */

#if (WINVER < 0x600)
//
// The connection was terminated by the remote computer.
//
// This error was obsoleted in the Windows Vista  release.
//
#define ERROR_PPP_REMOTE_TERMINATED          (RASBASE+119)
/*
 * %0
 */
#endif

#define ERROR_PPP_NO_PROTOCOLS_CONFIGURED    (RASBASE+120)
/*
 * A connection to the remote computer could not be established.  You might need to change the network settings for this connection.%0
 */

#define ERROR_PPP_NO_RESPONSE                (RASBASE+121)
/*
 * A connection to the remote access server was not made because the remote access server did not respond.%0
 */

#define ERROR_PPP_INVALID_PACKET             (RASBASE+122)
/*
 * Invalid data was received from the remote computer.  This data was ignored.%0
 */

#define ERROR_PHONE_NUMBER_TOO_LONG          (RASBASE+123)
/*
 * The phone number, including prefix and suffix, is too long.%0
 */

#if (WINVER < 0x600)
//
// The IPX protocol cannot dial out on the modem (or other connecting device) because this computer is not configured for dialing out (it is an IPX router).
//
// This error was obsoleted in the Windows Vista  release.
//
#define ERROR_IPXCP_NO_DIALOUT_CONFIGURED    (RASBASE+124)
/*
 * %0
 */

//
// The IPX protocol cannot dial in on the modem (or other connecting device) because this computer is not configured for dialing in (the IPX router is not installed).
//
// This error was obsoleted in the Windows Vista  release.
//
#define ERROR_IPXCP_NO_DIALIN_CONFIGURED     (RASBASE+125)
/*
 * %0
 */
#endif

#define ERROR_IPXCP_DIALOUT_ALREADY_ACTIVE   (RASBASE+126)
/*
 * The IPX protocol cannot be used for dialing out on more than one modem (or other connecting device) at a time.%0
 */

#if (WINVER < 0x600)
//
// Cannot access TCPCFG.DLL.
//
// This error was obsoleted in the Windows Vista  release.
//
#define ERROR_ACCESSING_TCPCFGDLL            (RASBASE+127)
/*
 * %0
 */
#endif

#define ERROR_NO_IP_RAS_ADAPTER              (RASBASE+128)
/*
 * The system cannot find an IP adapter.%0
 */

#define ERROR_SLIP_REQUIRES_IP               (RASBASE+129)
/*
 * SLIP cannot be used unless the IP protocol is installed.%0
 */

#if (WINVER < 0x600)
//
// Computer registration is not complete.
//
// This error was obsoleted in the Windows Vista  release.
//
#define ERROR_PROJECTION_NOT_COMPLETE        (RASBASE+130)
/*
 * %0
 */
#endif

#define ERROR_PROTOCOL_NOT_CONFIGURED        (RASBASE+131)
/*
 * The protocol is not configured.%0
 */

#define ERROR_PPP_NOT_CONVERGING             (RASBASE+132)
/*
 *Your computer and the remote computer could not agree on PPP control protocols.%0
 */

#define ERROR_PPP_CP_REJECTED                (RASBASE+133)
/*
 * A connection to the remote computer could not be completed.  You might need to adjust the protocols on this computer.%0
 */

#define ERROR_PPP_LCP_TERMINATED             (RASBASE+134)
/*
 * The PPP link control protocol was terminated.%0
 */

#define ERROR_PPP_REQUIRED_ADDRESS_REJECTED  (RASBASE+135)
/*
 * The requested address was rejected by the server.%0
 */

#define ERROR_PPP_NCP_TERMINATED             (RASBASE+136)
/*
 * The remote computer terminated the control protocol.%0
 */

#define ERROR_PPP_LOOPBACK_DETECTED          (RASBASE+137)
/*
 * Loopback was detected.%0
 */

#define ERROR_PPP_NO_ADDRESS_ASSIGNED        (RASBASE+138)
/*
 * The remote connection was not made because the remote access server did not assign an IP address.%0
 */

#define ERROR_CANNOT_USE_LOGON_CREDENTIALS   (RASBASE+139)
/*
 * The authentication protocol required by the remote server cannot use the stored password.  Redial, entering the password explicitly.%0
 */

#define ERROR_TAPI_CONFIGURATION             (RASBASE+140)
/*
 * An invalid dialing rule was detected.%0
 */

#define ERROR_NO_LOCAL_ENCRYPTION            (RASBASE+141)
/*
 * The local computer does not support the required data encryption type.%0
 */

#define ERROR_NO_REMOTE_ENCRYPTION           (RASBASE+142)
/*
 * The remote computer does not support the required data encryption type.%0
 */

#if (WINVER < 0x600)
//
// The remote computer requires data encryption.
//
// This error was obsoleted in the Windows Vista  release.
//
#define ERROR_REMOTE_REQUIRES_ENCRYPTION     (RASBASE+143)
/*
 * %0
 */

//
// The system cannot use the IPX network number assigned by the remote computer.  Additional information is provided in the event log.
//
// This error was obsoleted in the Windows Vista  release.
//
#define ERROR_IPXCP_NET_NUMBER_CONFLICT      (RASBASE+144)
/*
 * %0
 */

//
// ERROR_INVALID_SMM
//
// This error was obsoleted in the Windows Vista  release.
//
#define ERROR_INVALID_SMM                    (RASBASE+145)
/*
 * %0
 */

//
// ERROR_SMM_UNINITIALIZED
//
// This error was obsoleted in the Windows Vista  release.
//
#define ERROR_SMM_UNINITIALIZED              (RASBASE+146)
/*
 * %0
 */

//
// ERROR_NO_MAC_FOR_PORT
//
// This error was obsoleted in the Windows Vista  release.
//
#define ERROR_NO_MAC_FOR_PORT                (RASBASE+147)
/*
 * %0
 */

//
// ERROR_SMM_TIMEOUT
//
// This error was obsoleted in the Windows Vista  release.
//
#define ERROR_SMM_TIMEOUT                    (RASBASE+148)
/*
 * %0
 */
#endif


#define ERROR_BAD_PHONE_NUMBER               (RASBASE+149)
/*
 * The destination address or phone number is either invalid or not present.%0
 */


#if (WINVER < 0x600)

//
// ERROR_WRONG_MODULE
//
// This error was obsoleted in the Windows Vista  release.
//
#define ERROR_WRONG_MODULE                   (RASBASE+150)
/*
 * %0
 */

//
// The callback number contains an invalid character.  Only the following 18 characters are allowed:  0 to 9, T, P, W, (, ), -, @, and space.
//
// This error was obsoleted in the Windows Vista  release.
//
#define ERROR_INVALID_CALLBACK_NUMBER        (RASBASE+151)
/*
 * %0
 */
#endif

#define ERROR_SCRIPT_SYNTAX                  (RASBASE+152)
/*
 * A syntax error was encountered while processing a script.%0
 */

#define ERROR_HANGUP_FAILED                  (RASBASE+153)
/*
 * The connection could not be disconnected because the user does not have the required permission to disconnect.%0
 */

#define ERROR_BUNDLE_NOT_FOUND               (RASBASE+154)
/*
 * The system could not find the multi-link bundle.%0
 */

#define ERROR_CANNOT_DO_CUSTOMDIAL           (RASBASE+155)
/*
 * The system cannot perform automated dial because this connection has a custom dialer specified.%0
 */

#define ERROR_DIAL_ALREADY_IN_PROGRESS       (RASBASE+156)
/*
 * This connection is already being dialed.%0
 */

#define ERROR_RASAUTO_CANNOT_INITIALIZE      (RASBASE+157)
/*
 * Remote Access Services could not be started automatically.  Additional information is provided in the event log.%0
 */

#if (WINVER < 0x600)
//
// Internet Connection Sharing is already enabled on the connection.
//
// This error was obsoleted in the Windows Vista  release.
//
#define ERROR_CONNECTION_ALREADY_SHARED      (RASBASE+158)
/*
 * %0
 */

//
// An error occurred while the existing Internet Connection Sharing settings were being changed.
//
// This error was obsoleted in the Windows Vista  release.
//
#define ERROR_SHARING_CHANGE_FAILED          (RASBASE+159)
/*
 * %0
 */

//
// An error occurred while routing capabilities were being enabled.
//
// This error was obsoleted in the Windows Vista  release.
//
#define ERROR_SHARING_ROUTER_INSTALL         (RASBASE+160)
/*
 * %0
 */

//
// An error occurred while Internet Connection Sharing was being enabled for the connection.
//
// This error was obsoleted in the Windows Vista  release.
//
#define ERROR_SHARE_CONNECTION_FAILED        (RASBASE+161)
/*
 * %0
 */

//
// An error occurred while the local network was being configured for sharing.
//
// This error was obsoleted in the Windows Vista  release.
//
#define ERROR_SHARING_PRIVATE_INSTALL        (RASBASE+162)
/*
 * %0
 */

//
// Internet Connection Sharing cannot be enabled.  There is more than one LAN connection other than the connection to be shared.
//
// This error was obsoleted in the Windows Vista  release.
//
#define ERROR_CANNOT_SHARE_CONNECTION        (RASBASE+163)
/*
 * %0
 */
#endif

#define ERROR_NO_SMART_CARD_READER           (RASBASE+164)
/*
 * No smart card reader is installed.%0
 */

#define ERROR_SHARING_ADDRESS_EXISTS         (RASBASE+165)
/*
 * Internet Connection Sharing cannot be enabled.  A LAN connection is already configured with the IP address that is required for automatic IP addressing.%0
 */

#define ERROR_NO_CERTIFICATE                 (RASBASE+166)
/*
 * A certificate could not be found.  Connections that use the L2TP protocol over IPSec require the installation of a machine certificate, also known as a computer certificate.%0
 */

#define ERROR_SHARING_MULTIPLE_ADDRESSES     (RASBASE+167)
/*
 * Internet Connection Sharing cannot be enabled.  The LAN connection selected as the private network has more than one IP address configured.  Please reconfigure the LAN connection with a single IP address before enabling Internet Connection Sharing.%0
 */

#define ERROR_FAILED_TO_ENCRYPT              (RASBASE+168)
/*
 * The connection attempt failed because of failure to encrypt data.%0
 */

#define ERROR_BAD_ADDRESS_SPECIFIED          (RASBASE+169)
/*
 * The specified destination is not reachable.%0
 */

#define ERROR_CONNECTION_REJECT              (RASBASE+170)
/*
 * The remote computer rejected the connection attempt.%0
 */

#define ERROR_CONGESTION                     (RASBASE+171)
/*
 * The connection attempt failed because the network is busy.%0
 */

#define ERROR_INCOMPATIBLE                   (RASBASE+172)
/*
 * The remote computer's network hardware is incompatible with the type of call requested.%0
 */

#define ERROR_NUMBERCHANGED                  (RASBASE+173)
/*
 * The connection attempt failed because the destination number has changed.%0
 */

#define ERROR_TEMPFAILURE                    (RASBASE+174)
/*
 * The connection attempt failed because of a temporary failure.  Try connecting again.%0
 */

#define ERROR_BLOCKED                        (RASBASE+175)
/*
 * The call was blocked by the remote computer.%0
 */

#define ERROR_DONOTDISTURB                   (RASBASE+176)
/*
 * The call could not be connected because the remote computer has invoked the Do Not Disturb feature.%0
 */

#define ERROR_OUTOFORDER                     (RASBASE+177)
/*
 * The connection attempt failed because the modem (or other connecting device) on the remote computer is out of order.%0
 */

#define ERROR_UNABLE_TO_AUTHENTICATE_SERVER  (RASBASE+178)
/*
 * It was not possible to verify the identity of the server.%0
 */

#if (WINVER < 0x600)
//
// To dial out using this connection you must use a smart card.
//
// This error was obsoleted in the Windows Vista  release.
//
#define ERROR_SMART_CARD_REQUIRED            (RASBASE+179)
/*
 * %0
 */
#endif

#define ERROR_INVALID_FUNCTION_FOR_ENTRY     (RASBASE+180)
/*
 * An attempted function is not valid for this connection.%0
 */

#if (WINVER < 0x600)
//
// The connection requires a certificate, and no valid certificate was found.  For further assistance, click More Info or search Help and Support Center for this error number.
//
// This error was obsoleted in the Windows Vista  release.
//
#define ERROR_CERT_FOR_ENCRYPTION_NOT_FOUND  (RASBASE+181)
/*
 * %0
 */
#endif

#define ERROR_SHARING_RRAS_CONFLICT          (RASBASE+182)
/*
 * Internet Connection Sharing (ICS) and Internet Connection Firewall (ICF) cannot be enabled because Routing and Remote Access has been enabled on this computer.  To enable ICS or ICF, first disable Routing and Remote Access.  For more information about Routing and Remote Access, ICS, or ICF, see Help and Support.%0
 */

#define ERROR_SHARING_NO_PRIVATE_LAN         (RASBASE+183)
/*
 * Internet Connection Sharing cannot be enabled.  The LAN connection selected as the private network is either not present, or is disconnected from the network.  Please ensure that the LAN adapter is connected before enabling Internet Connection Sharing.%0
 */

#define ERROR_NO_DIFF_USER_AT_LOGON          (RASBASE+184)
/*
 * You cannot dial using this connection at logon time, because it is configured to use a user name different than the one on the smart card.  If you want to use it at logon time, you must configure it to use the user name on the smart card.%0
 */

#define ERROR_NO_REG_CERT_AT_LOGON           (RASBASE+185)
/*
 * You cannot dial using this connection at logon time, because it is not configured to use a smart card.  If you want to use it at logon time, you must edit the properties of this connection so that it uses a smart card.%0
 */

#define ERROR_OAKLEY_NO_CERT                 (RASBASE+186)
/*
 * The connection attempt failed because there is no valid machine certificate on your computer for security authentication.%0
 */

#define ERROR_OAKLEY_AUTH_FAIL               (RASBASE+187)
/*
 * The L2TP connection attempt failed because the security layer could not authenticate the remote computer.%0
 */

#define ERROR_OAKLEY_ATTRIB_FAIL             (RASBASE+188)
/*
 * The L2TP connection attempt failed because the security layer could not negotiate compatible parameters with the remote computer.%0
 */

#define ERROR_OAKLEY_GENERAL_PROCESSING      (RASBASE+189)
/*
 * The L2TP connection attempt failed because the security layer encountered a processing error during initial negotiations with the remote computer.%0
 */

#define ERROR_OAKLEY_NO_PEER_CERT            (RASBASE+190)
/*
 * The L2TP connection attempt failed because certificate validation of the remote computer failed.%0
 */

#define ERROR_OAKLEY_NO_POLICY               (RASBASE+191)
/*
 * The L2TP connection attempt failed because security policy for the connection was not found.%0
 */

#define ERROR_OAKLEY_TIMED_OUT               (RASBASE+192)
/*
 * The L2TP connection attempt failed because security negotiation timed out.%0
 */

#define ERROR_OAKLEY_ERROR                   (RASBASE+193)
/*
 * The L2TP connection attempt failed because an error occurred while negotiating security.%0
 */

#define ERROR_UNKNOWN_FRAMED_PROTOCOL        (RASBASE+194)
/*
 * The Framed Protocol RADIUS attribute for this user is not PPP.%0
 */

#define ERROR_WRONG_TUNNEL_TYPE              (RASBASE+195)
/*
 * The remote connection request was denied because the VPN tunnel type being used is not allowed.%0
 */

#define ERROR_UNKNOWN_SERVICE_TYPE           (RASBASE+196)
/*
 * The Service Type RADIUS attribute for this user is neither Framed nor Callback Framed.%0
 */

#define ERROR_CONNECTING_DEVICE_NOT_FOUND    (RASBASE+197)
/*
 * A connection to the remote access server was not made because the modem was not found.%0
 */

#define ERROR_NO_EAPTLS_CERTIFICATE          (RASBASE+198)
/*
 * A certificate could not be found that can be used with this Extensible Authentication Protocol.%0
 */

#define ERROR_SHARING_HOST_ADDRESS_CONFLICT  (RASBASE+199)
/*
 * Internet Connection Sharing (ICS) cannot be enabled due to an IP address conflict on the network.  ICS requires the host be configured to use 192.168.137.1.  Please ensure that no other client on the network is configured to use 192.168.137.1.%0
 */

#define ERROR_AUTOMATIC_VPN_FAILED           (RASBASE+200)
/*
 * The remote connection was not made because the attempted VPN tunnels failed. The VPN server might be unreachable. If this connection is attempting to use an L2TP/IPsec tunnel, the security parameters required for IPsec negotiation might not be configured properly.%0
 */

#define ERROR_VALIDATING_SERVER_CERT         (RASBASE+201)
/*
 * This connection is configured to validate the identity of the access server, but Windows cannot verify the digital certificate sent by the server.%0
 */

#define ERROR_READING_SCARD                  (RASBASE+202)
/*
 * The card supplied was not recognized.  Please check that the card is inserted correctly, and fits tightly.%0
 */

#define ERROR_INVALID_PEAP_COOKIE_CONFIG     (RASBASE+203)
/*
 * The PEAP configuration stored in the session cookie does not match the current session configuration.%0
 */

#define ERROR_INVALID_PEAP_COOKIE_USER       (RASBASE+204)
/*
 * The PEAP identity stored in the session cookie does not match the current identity.%0
 */

#define ERROR_INVALID_MSCHAPV2_CONFIG        (RASBASE+205)
/*
 * You cannot dial using this connection at logon time, because it is configured to use logged on user's credentials.%0
 */

//
// New Errors for Windows Vista
//
#define ERROR_VPN_GRE_BLOCKED                 (RASBASE+206)
/*
 * The VPN connection between your computer and the VPN server could not be completed. The most common cause for this failure is that at least one Internet device (for example, a firewall or a router) between your computer and the VPN server is not configured to allow Generic Routing Encapsulation (GRE) protocol packets. If the problem persists, contact your network administrator or Internet Service Provider.%0
 */

#define ERROR_VPN_DISCONNECT                 (RASBASE+207)
/*
 * The network connection between your computer and the VPN server was interrupted.  This can be caused by a problem in the VPN transmission and is commonly the result of internet latency or simply that your VPN server has reached capacity.  Please try to reconnect to the VPN server.  If this problem persists, contact the VPN administrator and analyze quality of network connectivity.%0
 */

#define ERROR_VPN_REFUSED                    (RASBASE+208)
/*
 * The network connection between your computer and the VPN server could not be established because the remote server refused the connection. This is typically caused by a mismatch between the server's configuration and your connection settings. Please contact the remote server's Administrator to verify the server configuration and your connection settings.%0
 */

#define ERROR_VPN_TIMEOUT                    (RASBASE+209)
/*
 * The network connection between your computer and the VPN server could not be established because the remote server is not responding. This could be because one of the network devices (e.g, firewalls, NAT, routers, etc) between your computer and the remote server is not configured to allow VPN connections. Please contact your Administrator or your service provider to determine which device may be causing the problem.%0
 */

#define ERROR_VPN_BAD_CERT                   (RASBASE+210)
/*
 * A network connection between your computer and the VPN server was started, but the VPN connection was not completed. This is typically caused by the use of an incorrect or expired certificate for authentication between the client and the server. Please contact your Administrator to ensure that the certificate being used for authentication is valid.%0
 */

#define ERROR_VPN_BAD_PSK                    (RASBASE+211)
/*
 * The network conection between your computer and the VPN server could not be established because the remote server is not responding.  This is typically caused by a pre-shared key problem between the client and server.  A pre-shared key is used to guarantee you are who you say you are in an IP Security (IPSec) communication cycle.  Please get the assistance of your administrator to determine where the pre-shared key problem is originating.%0
 */

#define ERROR_SERVER_POLICY                  (RASBASE+212)
/*
 *The connection was prevented because of a policy configured on your RAS/VPN server. Specifically, the authentication method used by the server to verify your username and password may not match the authentication method configured in your connection profile. Please contact the Administrator of the RAS server and notify them of this error.%0
 */

#define ERROR_BROADBAND_ACTIVE               (RASBASE+213)
/*
 * You have attempted to establish a second broadband connection while a previous broadband connection is already established using the same device or port. Please disconnect the earlier connection and then reestablish the connection.%0
 */

#define ERROR_BROADBAND_NO_NIC               (RASBASE+214)
/*
 * The underlying Ethernet connectivity required for the broadband connection was not found. Please install and enable the Ethernet adapter on your computer via the Network Connections folder before attempting this connection.%0
 */

#define ERROR_BROADBAND_TIMEOUT              (RASBASE+215)
/*
 * The broadband network conection could not be established on your computer because the remote server is not responding. This could be caused by an invalid value for the 'Service Name' field for this connection. Please contact your Internet Service Provider and inquire about the correct value for this field and update it in the Connection Properties.%0
 */

#define ERROR_FEATURE_DEPRECATED             (RASBASE+216)
/*
 * A feature or setting you have tried to enable is no longer supported by the remote access service.%0
 */

#define ERROR_CANNOT_DELETE                  (RASBASE+217)
/*
 * Cannot delete a connection while it is connected.%0
 */

#if (WINVER <= 0x600)

#define ERROR_RASQEC_RESOURCE_CREATION_FAILED (RASBASE+218)
/*
* The Network Access Protection (NAP) enforcement client could not create system resources for remote access connections. Some network services or resources might not be available. If the problem persists, disconnect and retry the remote access connection or contact the administrator for the remote access server.
*/

#define ERROR_RASQEC_NAPAGENT_NOT_ENABLED    (RASBASE+219)
/*
* The Network Access Protection Agent (NAPAgent) service has been disabled or is not installed on this computer. Some network services or resources might not be available. If the problem persists, disconnect and retry the remote access connection or contact the administrator for the remote access server.
*/

#define ERROR_RASQEC_NAPAGENT_NOT_CONNECTED  (RASBASE+220)
/*
* The Network Access Protection (NAP) enforcement client failed to register with the Network Access Protection Agent (NAPAgent) service. Some network services or resources might not be available. If the problem persists, disconnect and retry the remote access connection or contact the administrator for the remote access server.
*/

#define ERROR_RASQEC_CONN_DOESNOTEXIST       (RASBASE+221)
/*
* The Network Access Protection (NAP) enforcement client was unable to process the request because the remote access connection does not exist. Retry the remote access connection. If the problem persists, make sure that you can connect to the Internet, and then contact the administrator for the remote access server.
*/

#define ERROR_RASQEC_TIMEOUT                 (RASBASE+222)
/*
* The Network Access Protection (NAP) enforcement client did not respond. Some network services or resources might not be available. If the problem persists, disconnect and retry the remote access connection or contact the administrator for the remote access server.
*/

#endif

#define ERROR_PEAP_CRYPTOBINDING_INVALID    (RASBASE+223)
/*
* Received Crypto-Binding TLV is invalid.%0
*/

#define ERROR_PEAP_CRYPTOBINDING_NOTRECEIVED   (RASBASE+224)
/*
* Crypto-Binding TLV is not received.%0
*/

#if (WINVER >= 0x600)

#define ERROR_INVALID_VPNSTRATEGY   (RASBASE+225)
/*
* The remote connection was not made because Point-to-Point Tunneling Protocol (PPTP) is incompatible with IPv6. Use any other tunneling protocol. %0
*/

#endif

#define ERROR_EAPTLS_CACHE_CREDENTIALS_INVALID   (RASBASE+226)
/*
* EAPTLS validation of the cached credentials failed. Please discard
* cached credentials.
*/

#define ERROR_IPSEC_SERVICE_STOPPED                 (RASBASE+227)
/*
* The VPN connection cannot be completed because the 'IKE and AuthIP IPSec Keying Modules' service and/or the 'Base Filtering Engine' service is not running. These services are required to establish the connection. Please ensure that these services have been started before dialing the connection.
*/

#if (WINVER >= 0x600)

#define ERROR_IDLE_TIMEOUT               (RASBASE+228)
/*
 * The connection was terminated because of idle timeout.%0
 */

#define ERROR_LINK_FAILURE               (RASBASE+229)
/*
 * The modem (or other connecting device) was disconnected due to link failure.%0
 */

#define ERROR_USER_LOGOFF                (RASBASE+230)
/*
 * The connection was terminated because user logged off.%0
 */

#define ERROR_FAST_USER_SWITCH           (RASBASE+231)
/*
 * The connection was terminated because user switch happened.%0
 */

#define ERROR_HIBERNATION                 (RASBASE+232)
/*
 * The connection was terminated because of hibernation.%0
 */

#define ERROR_SYSTEM_SUSPENDED            (RASBASE+233)
/*
 * The connection was terminated because the system got suspended.%0
 */

#define ERROR_RASMAN_SERVICE_STOPPED      (RASBASE+234)
/*
 * The connection was terminated because Remote Access Connection manager stopped.%0
 */

#define ERROR_INVALID_SERVER_CERT         (RASBASE+235)
/*
* The L2TP connection attempt failed because the security layer could not authenticate the remote computer. This could be because one or more fields of the certificate presented by the remote server could not be validated as belonging to the target destination.
*/

#define ERROR_NOT_NAP_CAPABLE             (RASBASE+236)
/*
* The Network Access Protection (NAP) health state of the computer cannot be determined. Contact your administrator to verify that the NAP enforcement client is enabled, the NAP Agent service is running, and NAP is enforced in the Protected Extensible Authentication Protocol (PEAP) properties of the remote access connection.
*/

#endif

#if (WINVER >= 0x601)

#define ERROR_INVALID_TUNNELID           (RASBASE+237)
/*
* Invalid Tunnel ID.%0
*/

#define ERROR_UPDATECONNECTION_REQUEST_IN_PROCESS    (RASBASE+238)
/*
* Another Update connection request is in progress. RAS allows only one Update Connection request at a time.%0
*/

#define ERROR_PROTOCOL_ENGINE_DISABLED    (RASBASE+239)
/*
* Negotiating using configured protocol is disable. Edit connection properties and select different protocol for negotiation and try again.%0
*/

#define ERROR_INTERNAL_ADDRESS_FAILURE    (RASBASE+240)
/*
 *  Internal address negotiation failed.%0
 */

#define ERROR_FAILED_CP_REQUIRED          (RASBASE+241)
/*
 * Client has to request a Internal IPv4 or IPv6 address.%0
 */

#define ERROR_TS_UNACCEPTABLE             (RASBASE+242)
/*
 *  Traffic Selectors negotiation failed.%0
 */

#define ERROR_MOBIKE_DISABLED             (RASBASE+243)
/*
* Mobility is disabled for this connection.
*/

#define ERROR_CANNOT_INITIATE_MOBIKE_UPDATE  (RASBASE+244)
/*
* The VPN Connection is still connecting or reauthenticating because of Quarantine state change. Initiate mobike update only when connection state is 'Connected'.
*/

#define ERROR_PEAP_SERVER_REJECTED_CLIENT_TLV           (RASBASE+245)
/*
* Server rejected client authentication, due unexpected TLV or value mismatch for a TLV.
*/

#define ERROR_INVALID_PREFERENCES           (RASBASE+246)
/*
* Either VPN Destination preference is not selected by the user or it is no longer valid.
*/

#define ERROR_EAPTLS_SCARD_CACHE_CREDENTIALS_INVALID (RASBASE+247)
/*
* Cached smart card credential is invalid.
*/

#define ERROR_SSTP_COOKIE_SET_FAILURE             (RASBASE + 248)
/*
* VPN connection attempt failed due to internal error occurred while adding cookies to the Secure Socket Tunneling Protocol (SSTP). Please see the System Event Log for the detailed information.
*/
#define ERROR_INVALID_PEAP_COOKIE_ATTRIBUTES (RASBASE+249)
/*
* The PEAP inner method attributes stored in the cookie is invalid
*/

#define ERROR_EAP_METHOD_NOT_INSTALLED            (RASBASE+250)
/*
* The Extensible Authentication Protocol type required for authentication of the remote access connection is not installed on your computer.
*/

#define ERROR_EAP_METHOD_DOES_NOT_SUPPORT_SSO     (RASBASE+251)
/*
* The Extensible Authentication Protocol type configured on the remote access connection does not support single sign-on.
*/

#define ERROR_EAP_METHOD_OPERATION_NOT_SUPPORTED  (RASBASE+252)
/*
* The Extensible Authentication Protocol type configured on the remote access connection does not support the requested operation.
*/

#define ERROR_EAP_USER_CERT_INVALID               (RASBASE+253)
/*
* The remote access connection completed, but authentication failed because the certificate that authenticates the client to the server is not valid. Ensure that the certificate used for authentication is valid.
*/

#define ERROR_EAP_USER_CERT_EXPIRED               (RASBASE+254)
/*
* The remote access connection completed, but authentication failed because the certificate that authenticates the client to the server is expired. Renew the certificate.
*/

#define ERROR_EAP_USER_CERT_REVOKED               (RASBASE+255)
/*
* The remote access connection completed, but authentication failed because the certificate that authenticates the client to the server is revoked.
*/

#define ERROR_EAP_USER_CERT_OTHER_ERROR           (RASBASE+256)
/*
* The remote access connection completed, but authentication failed because of an error in the certificate that authenticates the client to the server.
*/

#define ERROR_EAP_SERVER_CERT_INVALID             (RASBASE+257)
/*
* The remote access connection completed, but authentication failed because the certificate that the client uses to authenticate the server is not valid.
*/

#define ERROR_EAP_SERVER_CERT_EXPIRED             (RASBASE+258)
/*
* The remote access connection completed, but authentication failed because the certificate that the client uses to authenticate the server is expired.
*/

#define ERROR_EAP_SERVER_CERT_REVOKED             (RASBASE+259)
/*
* The remote access connection completed, but authentication failed because the certificate that the client uses to authenticate the server is revoked.
*/

#define ERROR_EAP_SERVER_CERT_OTHER_ERROR         (RASBASE+260)
/*
* The remote access connection completed, but authentication failed because of an error in the certificate that the client uses to authenticate the server.
*/

#define ERROR_EAP_USER_ROOT_CERT_NOT_FOUND        (RASBASE+261)
/*
* The remote access connection completed, but authentication failed because a trusted root certificate that validates the user certificate was not found in the Trusted Root Certification Authorities certificate store.
*/

#define ERROR_EAP_USER_ROOT_CERT_INVALID          (RASBASE+262)
/*
* The remote access connection completed, but authentication failed because the trusted root certificate that is used to validate the user certificate is not valid.
*/

#define ERROR_EAP_USER_ROOT_CERT_EXPIRED          (RASBASE+263)
/*
* The remote access connection completed, but authentication failed because the certificate in the Trusted Root Certification Authorities certificate store that authenticates the user certificate is expired. Renew the certificate.
*/

#define ERROR_EAP_SERVER_ROOT_CERT_NOT_FOUND      (RASBASE+264)
/*
* The remote access connection completed, but authentication failed because a certificate that validates the server certificate was not found in the Trusted Root Certification Authorities certificate store.
*/

#define ERROR_EAP_SERVER_ROOT_CERT_INVALID        (RASBASE+265)
/*
* The remote zccess connection completed, but authentication failed because the certificate in the Trusted Root Certification Authorities certificate store that validates the server certificate is not valid.
*/

#define ERROR_EAP_SERVER_ROOT_CERT_NAME_REQUIRED  (RASBASE+266)
/*
* The remote access connection completed, but authentication failed because the certificate on the server computer does not have a server name specified.
*/

#define ERROR_PEAP_IDENTITY_MISMATCH               (RASBASE+267)
/*
*    The PEAP outer identity is not same as the inner identity when identity privacy is turned OFF.
*/

#define ERROR_DNSNAME_NOT_RESOLVABLE               (RASBASE+268)
/*
*    The remote connection was not made because the name of the remote access server did not resolve.
*/

#define ERROR_EAPTLS_PASSWD_INVALID                (RASBASE+269)
/*
* Password provided for certificate is not valid.
*/

#define ERROR_IKEV2_PSK_INTERFACE_ALREADY_EXISTS   (RASBASE+270)
/*
* The Interface  could not be enabled as more than one interface with same destination has been created with authentication method as Pre Shared Key. Change the destination/auth method and enable the interface.
*/

#define ERROR_INVALID_DESTINATION_IP               (RASBASE+271)
/*
* The Interface could not be enabled as no valid destination is found for it. Ensure that a valid/resolvable destination IP is configured and try again.
*/

#endif

#if (WINVER >= 0x603)

#define ERROR_INVALID_INTERFACE_CONFIG             (RASBASE+272)
/*
* The configuration for the interface is not valid
*/

#endif

#define ERROR_VPN_PLUGIN_GENERIC                   (RASBASE+273)
/*
* Couldn't connect to the VPN.
*/

#define ERROR_SSO_CERT_MISSING                     (RASBASE+274)
/*
* Couldn't connect because we couldn't find a certificate for single sign-on. Please contact your support person.
*/

#define ERROR_DEVICE_COMPLIANCE                    (RASBASE+275)
/*
* Couldn't connect because there was a problem with device compliance.
*/

#define ERROR_PLUGIN_NOT_INSTALLED                 (RASBASE+276)
/*
* The app %1!ws! is required for this VPN connection. Please download it in the Microsoft Store or contact your IT administrators.
*/

#define ERROR_ACTION_REQUIRED                      (RASBASE+277)
/*
* The connection requires an input state from the user.
*/

#define ERROR_WINHTTP_AUTO_PROXY_SERVICE           (RASBASE+278)
/*
* Couldn't connect because the call to the WinHttp Auto Proxy Service failed. This is usually the result of Web Proxy Auto-Discovery (WPAD) being disabled. Please enable it to allow the VPN connection to proceed, or contact your network administrator.
*/

#define RASBASEEND                                 (RASBASE+278)

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion

#endif // _RASERROR_H_
