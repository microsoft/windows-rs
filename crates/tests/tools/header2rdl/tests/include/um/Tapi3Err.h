/*****************************************************************************
*
* Copyright (c) Microsoft Corporation. All rights reserved.
*
* Module Name:
*
*    tapi3err.h
*
* Abstract:
*
*    Error Notifications for TAPI 3.0
*
*****************************************************************************/

#if _MSC_VER >= 1000
#pragma once
#endif // _MSC_VER >= 1000

#ifndef __TAPI3ERR_H__
#define __TAPI3ERR_H__

//--------------------------------------------------------------------------
//     Core TAPI Error messages
//--------------------------------------------------------------------------

//
//  Values are 32 bit values laid out as follows:
//
//   3 3 2 2 2 2 2 2 2 2 2 2 1 1 1 1 1 1 1 1 1 1
//   1 0 9 8 7 6 5 4 3 2 1 0 9 8 7 6 5 4 3 2 1 0 9 8 7 6 5 4 3 2 1 0
//  +---+-+-+-----------------------+-------------------------------+
//  |Sev|C|R|     Facility          |               Code            |
//  +---+-+-+-----------------------+-------------------------------+
//
//  where
//
//      Sev - is the severity code
//
//          00 - Success
//          01 - Informational
//          10 - Warning
//          11 - Error
//
//      C - is the Customer code flag
//
//      R - is a reserved bit
//
//      Facility - is the facility code
//
//      Code - is the facility's status code
//
//
// Define the facility codes
//


//
// Define the severity codes
//


//
// MessageId: TAPI_E_NOTENOUGHMEMORY
//
// MessageText:
//
// The buffer passed in to this method was not big enough.
//
#define TAPI_E_NOTENOUGHMEMORY           ((HRESULT)0x80040001L)

//
// MessageId: TAPI_E_NOITEMS
//
// MessageText:
//
// No items exist that match the request.
//
#define TAPI_E_NOITEMS                   ((HRESULT)0x80040002L)

//
// MessageId: TAPI_E_NOTSUPPORTED
//
// MessageText:
//
// This method is not supported.
//
#define TAPI_E_NOTSUPPORTED              ((HRESULT)0x80040003L)

//
// MessageId: TAPI_E_INVALIDMEDIATYPE
//
// MessageText:
//
// The MEDIATYPE passed in to this method was invalid.
//
#define TAPI_E_INVALIDMEDIATYPE          ((HRESULT)0x80040004L)

//
// MessageId: TAPI_E_OPERATIONFAILED
//
// MessageText:
//
// The operation failed for an unspecified reason.
//
#define TAPI_E_OPERATIONFAILED           ((HRESULT)0x80040005L)

//
// MessageId: TAPI_E_ALLOCATED
//
// MessageText:
//
// The device is already in use.
//
#define TAPI_E_ALLOCATED                 ((HRESULT)0x80040006L)

//
// MessageId: TAPI_E_CALLUNAVAIL
//
// MessageText:
//
// No call appearance available.
//
#define TAPI_E_CALLUNAVAIL               ((HRESULT)0x80040007L)

//
// MessageId: TAPI_E_COMPLETIONOVERRUN
//
// MessageText:
//
// Too many call completions outstanding.
//
#define TAPI_E_COMPLETIONOVERRUN         ((HRESULT)0x80040008L)

//
// MessageId: TAPI_E_CONFERENCEFULL
//
// MessageText:
//
// The conference is full.
//
#define TAPI_E_CONFERENCEFULL            ((HRESULT)0x80040009L)

//
// MessageId: TAPI_E_DIALMODIFIERNOTSUPPORTED
//
// MessageText:
//
// The dial modifier is not supported.
//
#define TAPI_E_DIALMODIFIERNOTSUPPORTED  ((HRESULT)0x8004000AL)

//
// MessageId: TAPI_E_INUSE
//
// MessageText:
//
// The device is already in use.
//
#define TAPI_E_INUSE                     ((HRESULT)0x8004000BL)

//
// MessageId: TAPI_E_INVALADDRESS
//
// MessageText:
//
// The phone number is invalid or not properly formatted.
//
#define TAPI_E_INVALADDRESS              ((HRESULT)0x8004000CL)

//
// MessageId: TAPI_E_INVALADDRESSSTATE
//
// MessageText:
//
// Operation not permitted in current address state.
//
#define TAPI_E_INVALADDRESSSTATE         ((HRESULT)0x8004000DL)

//
// MessageId: TAPI_E_INVALCALLPARAMS
//
// MessageText:
//
// Invalid LINECALLPARAMS structure.
//
#define TAPI_E_INVALCALLPARAMS           ((HRESULT)0x8004000EL)

//
// MessageId: TAPI_E_INVALCALLPRIVILEGE
//
// MessageText:
//
// Invalid call privilege.
//
#define TAPI_E_INVALCALLPRIVILEGE        ((HRESULT)0x8004000FL)

//
// MessageId: TAPI_E_INVALCALLSTATE
//
// MessageText:
//
// Operation not permitted in current call state.
//
#define TAPI_E_INVALCALLSTATE            ((HRESULT)0x80040010L)

//
// MessageId: TAPI_E_INVALCARD
//
// MessageText:
//
// Invalid calling card.
//
#define TAPI_E_INVALCARD                 ((HRESULT)0x80040011L)

//
// MessageId: TAPI_E_INVALCOMPLETIONID
//
// MessageText:
//
// Invalid call completion ID.
//
#define TAPI_E_INVALCOMPLETIONID         ((HRESULT)0x80040012L)

//
// MessageId: TAPI_E_INVALCOUNTRYCODE
//
// MessageText:
//
// Invalid country code.
//
#define TAPI_E_INVALCOUNTRYCODE          ((HRESULT)0x80040013L)

//
// MessageId: TAPI_E_INVALDEVICECLASS
//
// MessageText:
//
// Invalid device class identifier
//
#define TAPI_E_INVALDEVICECLASS          ((HRESULT)0x80040014L)

//
// MessageId: TAPI_E_INVALDIALPARAMS
//
// MessageText:
//
// Invalid dialing parameters
//
#define TAPI_E_INVALDIALPARAMS           ((HRESULT)0x80040015L)

//
// MessageId: TAPI_E_INVALDIGITS
//
// MessageText:
//
// Invalid digits.
//
#define TAPI_E_INVALDIGITS               ((HRESULT)0x80040016L)

//
// MessageId: TAPI_E_INVALGROUPID
//
// MessageText:
//
// Invalid group pickup ID.
//
#define TAPI_E_INVALGROUPID              ((HRESULT)0x80040017L)

//
// MessageId: TAPI_E_INVALLOCATION
//
// MessageText:
//
// Invalid location ID.
//
#define TAPI_E_INVALLOCATION             ((HRESULT)0x80040018L)

//
// MessageId: TAPI_E_INVALMESSAGEID
//
// MessageText:
//
// Invalid message ID.
//
#define TAPI_E_INVALMESSAGEID            ((HRESULT)0x80040019L)

//
// MessageId: TAPI_E_INVALPARKID
//
// MessageText:
//
// Invalid park ID.
//
#define TAPI_E_INVALPARKID               ((HRESULT)0x8004001AL)

//
// MessageId: TAPI_E_INVALRATE
//
// MessageText:
//
// Invalid rate.
//
#define TAPI_E_INVALRATE                 ((HRESULT)0x8004001BL)

//
// MessageId: TAPI_E_INVALTIMEOUT
//
// MessageText:
//
// Invalid timeout value.
//
#define TAPI_E_INVALTIMEOUT              ((HRESULT)0x8004001CL)

//
// MessageId: TAPI_E_INVALTONE
//
// MessageText:
//
// Invalid tone.
//
#define TAPI_E_INVALTONE                 ((HRESULT)0x8004001DL)

//
// MessageId: TAPI_E_INVALLIST
//
// MessageText:
//
// Invalid list passed as a parameter
//
#define TAPI_E_INVALLIST                 ((HRESULT)0x8004001EL)

//
// MessageId: TAPI_E_INVALMODE
//
// MessageText:
//
// Invalid mode passed as a parameter
//
#define TAPI_E_INVALMODE                 ((HRESULT)0x8004001FL)

//
// MessageId: TAPI_E_NOCONFERENCE
//
// MessageText:
//
// The call is not part of a conference.
//
#define TAPI_E_NOCONFERENCE              ((HRESULT)0x80040020L)

//
// MessageId: TAPI_E_NODEVICE
//
// MessageText:
//
// The device was removed, or the device class is not recognized.
//
#define TAPI_E_NODEVICE                  ((HRESULT)0x80040021L)

//
// MessageId: TAPI_E_NOREQUEST
//
// MessageText:
//
// No Assisted Telephony requests are pending.
//
#define TAPI_E_NOREQUEST                 ((HRESULT)0x80040022L)

//
// MessageId: TAPI_E_NOTOWNER
//
// MessageText:
//
// The application is does not have OWNER privilege on the call.
//
#define TAPI_E_NOTOWNER                  ((HRESULT)0x80040023L)

//
// MessageId: TAPI_E_NOTREGISTERED
//
// MessageText:
//
// The application is not registered to handle requests.
//
#define TAPI_E_NOTREGISTERED             ((HRESULT)0x80040024L)

//
// MessageId: TAPI_E_REQUESTOVERRUN
//
// MessageText:
//
// The request queue is already full.
//
#define TAPI_E_REQUESTOVERRUN            ((HRESULT)0x80040025L)

//
// MessageId: TAPI_E_TARGETNOTFOUND
//
// MessageText:
//
// The call handoff failed because the specified target was not found.
//
#define TAPI_E_TARGETNOTFOUND            ((HRESULT)0x80040026L)

//
// MessageId: TAPI_E_TARGETSELF
//
// MessageText:
//
// No higher priority target exists for the call handoff.
//
#define TAPI_E_TARGETSELF                ((HRESULT)0x80040027L)

//
// MessageId: TAPI_E_USERUSERINFOTOOBIG
//
// MessageText:
//
// The amount of user-user info exceeds the maximum permitted.
//
#define TAPI_E_USERUSERINFOTOOBIG        ((HRESULT)0x80040028L)

//
// MessageId: TAPI_E_REINIT
//
// MessageText:
//
// The operation cannot be completed until all TAPI applications shutdown and reinitialize. 
//
#define TAPI_E_REINIT                    ((HRESULT)0x80040029L)

//
// MessageId: TAPI_E_ADDRESSBLOCKED
//
// MessageText:
//
// You are not permitted to call this number.
//
#define TAPI_E_ADDRESSBLOCKED            ((HRESULT)0x8004002AL)

//
// MessageId: TAPI_E_BILLINGREJECTED
//
// MessageText:
//
// The calling card number or other billing information was rejected.
//
#define TAPI_E_BILLINGREJECTED           ((HRESULT)0x8004002BL)

//
// MessageId: TAPI_E_INVALFEATURE
//
// MessageText:
//
// Invalid device-specific feature.
//
#define TAPI_E_INVALFEATURE              ((HRESULT)0x8004002CL)

//
// MessageId: TAPI_E_INVALBUTTONLAMPID
//
// MessageText:
//
// Invalid button or lamp ID.
//
#define TAPI_E_INVALBUTTONLAMPID         ((HRESULT)0x8004002DL)

//
// MessageId: TAPI_E_INVALBUTTONSTATE
//
// MessageText:
//
// Invalid button state.
//
#define TAPI_E_INVALBUTTONSTATE          ((HRESULT)0x8004002EL)

//
// MessageId: TAPI_E_INVALDATAID
//
// MessageText:
//
// Invalid data segment ID.
//
#define TAPI_E_INVALDATAID               ((HRESULT)0x8004002FL)

//
// MessageId: TAPI_E_INVALHOOKSWITCHDEV
//
// MessageText:
//
// Invalid hookswitch device ID.
//
#define TAPI_E_INVALHOOKSWITCHDEV        ((HRESULT)0x80040030L)

//
// MessageId: TAPI_E_DROPPED
//
// MessageText:
//
// The call was disconnected.
//
#define TAPI_E_DROPPED                   ((HRESULT)0x80040031L)

//
// MessageId: TAPI_E_NOREQUESTRECIPIENT
//
// MessageText:
//
// No program is available to handle the request.
//
#define TAPI_E_NOREQUESTRECIPIENT        ((HRESULT)0x80040032L)

//
// MessageId: TAPI_E_REQUESTQUEUEFULL
//
// MessageText:
//
// The queue of call requests is full.
//
#define TAPI_E_REQUESTQUEUEFULL          ((HRESULT)0x80040033L)

//
// MessageId: TAPI_E_DESTBUSY
//
// MessageText:
//
// The called number is busy.
//
#define TAPI_E_DESTBUSY                  ((HRESULT)0x80040034L)

//
// MessageId: TAPI_E_DESTNOANSWER
//
// MessageText:
//
// The called party does not answer.
//
#define TAPI_E_DESTNOANSWER              ((HRESULT)0x80040035L)

//
// MessageId: TAPI_E_DESTUNAVAIL
//
// MessageText:
//
// The called number could not be reached
//
#define TAPI_E_DESTUNAVAIL               ((HRESULT)0x80040036L)

//
// MessageId: TAPI_E_REQUESTFAILED
//
// MessageText:
//
// The request failed for unspecified reasons.
//
#define TAPI_E_REQUESTFAILED             ((HRESULT)0x80040037L)

//
// MessageId: TAPI_E_REQUESTCANCELLED
//
// MessageText:
//
// The request was cancelled.
//
#define TAPI_E_REQUESTCANCELLED          ((HRESULT)0x80040038L)

//
// MessageId: TAPI_E_INVALPRIVILEGE
//
// MessageText:
//
// Invalid privilege.
//
#define TAPI_E_INVALPRIVILEGE            ((HRESULT)0x80040039L)

//
// MessageId: TAPI_E_INVALIDDIRECTION
//
// MessageText:
//
// The TERMINAL_DIRECTION passed in was invalid.
//
#define TAPI_E_INVALIDDIRECTION          ((HRESULT)0x8004003AL)

//
// MessageId: TAPI_E_INVALIDTERMINAL
//
// MessageText:
//
// The Terminal passed in was invalid for this operation.
//
#define TAPI_E_INVALIDTERMINAL           ((HRESULT)0x8004003BL)

//
// MessageId: TAPI_E_INVALIDTERMINALCLASS
//
// MessageText:
//
// The Terminal Class is invalid.
//
#define TAPI_E_INVALIDTERMINALCLASS      ((HRESULT)0x8004003CL)

//
// MessageId: TAPI_E_NODRIVER
//
// MessageText:
//
// The service provider was removed.
//
#define TAPI_E_NODRIVER                  ((HRESULT)0x8004003DL)

//
// MessageId: TAPI_E_MAXSTREAMS
//
// MessageText:
//
// The maximum number of streams was reached.
//
#define TAPI_E_MAXSTREAMS                ((HRESULT)0x8004003EL)

//
// MessageId: TAPI_E_NOTERMINALSELECTED
//
// MessageText:
//
// The operation could not be performed because it requires terminals to be selected.
//
#define TAPI_E_NOTERMINALSELECTED        ((HRESULT)0x8004003FL)

//
// MessageId: TAPI_E_TERMINALINUSE
//
// MessageText:
//
// The operation could not be performed because the terminal is in use.
//
#define TAPI_E_TERMINALINUSE             ((HRESULT)0x80040040L)

//
// MessageId: TAPI_E_NOTSTOPPED
//
// MessageText:
//
// The operation could not be performed because it requires the stream to be stopped.
//
#define TAPI_E_NOTSTOPPED                ((HRESULT)0x80040041L)

//
// MessageId: TAPI_E_MAXTERMINALS
//
// MessageText:
//
// The maximum number of terminals has been reached.
//
#define TAPI_E_MAXTERMINALS              ((HRESULT)0x80040042L)

//
// MessageId: TAPI_E_INVALIDSTREAM
//
// MessageText:
//
// The Stream passed in was invalid for this operation.
//
#define TAPI_E_INVALIDSTREAM             ((HRESULT)0x80040043L)

//
// MessageId: TAPI_E_TIMEOUT
//
// MessageText:
//
// The call failed due to a timeout.
//
#define TAPI_E_TIMEOUT                   ((HRESULT)0x80040044L)

//--------------------------------------------------------------------------
//     Call Center Error messages
//--------------------------------------------------------------------------

//
// MessageId: TAPI_E_CALLCENTER_GROUP_REMOVED
//
// MessageText:
//
// The ACD Proxy has removed this Group. Operations on this object are invalid.
//
#define TAPI_E_CALLCENTER_GROUP_REMOVED  ((HRESULT)0x80040045L)

//
// MessageId: TAPI_E_CALLCENTER_QUEUE_REMOVED
//
// MessageText:
//
// The ACD Proxy has removed this Queue. Operations on this object are invalid.
//
#define TAPI_E_CALLCENTER_QUEUE_REMOVED  ((HRESULT)0x80040046L)

//
// MessageId: TAPI_E_CALLCENTER_NO_AGENT_ID
//
// MessageText:
//
// The Agent object was created with CreateAgent. It does not have an ID, use CreateAgentWithID.
//
#define TAPI_E_CALLCENTER_NO_AGENT_ID    ((HRESULT)0x80040047L)

//
// MessageId: TAPI_E_CALLCENTER_INVALAGENTID
//
// MessageText:
//
// Invalid agent ID.
//
#define TAPI_E_CALLCENTER_INVALAGENTID   ((HRESULT)0x80040048L)

//
// MessageId: TAPI_E_CALLCENTER_INVALAGENTGROUP
//
// MessageText:
//
// Invalid agent group.
//
#define TAPI_E_CALLCENTER_INVALAGENTGROUP ((HRESULT)0x80040049L)

//
// MessageId: TAPI_E_CALLCENTER_INVALPASSWORD
//
// MessageText:
//
// Invalid agent password.
//
#define TAPI_E_CALLCENTER_INVALPASSWORD  ((HRESULT)0x8004004AL)

//
// MessageId: TAPI_E_CALLCENTER_INVALAGENTSTATE
//
// MessageText:
//
// Invalid agent state
//
#define TAPI_E_CALLCENTER_INVALAGENTSTATE ((HRESULT)0x8004004BL)

//
// MessageId: TAPI_E_CALLCENTER_INVALAGENTACTIVITY
//
// MessageText:
//
// Invalid agent activity.
//
#define TAPI_E_CALLCENTER_INVALAGENTACTIVITY ((HRESULT)0x8004004CL)

//
// MessageId: TAPI_E_REGISTRY_SETTING_CORRUPT
//
// MessageText:
//
// Registry Setting is Corrupt.
//
#define TAPI_E_REGISTRY_SETTING_CORRUPT  ((HRESULT)0x8004004DL)

//--------------------------------------------------------------------------
//     Terminal Specific Error messages
//--------------------------------------------------------------------------

//
// MessageId: TAPI_E_TERMINAL_PEER
//
// MessageText:
//
// The peer for one of these bridge terminals has already been assigned.
//
#define TAPI_E_TERMINAL_PEER             ((HRESULT)0x8004004EL)

//
// MessageId: TAPI_E_PEER_NOT_SET
//
// MessageText:
//
// The peer for this bridge terminal must be set to complete this operation.
//
#define TAPI_E_PEER_NOT_SET              ((HRESULT)0x8004004FL)


//--------------------------------------------------------------------------
//     Media Service Provider Error messages
//--------------------------------------------------------------------------
//
// MessageId: TAPI_E_NOEVENT
//
// MessageText:
//
// There is no event in the MSP's event queue.
//
#define TAPI_E_NOEVENT                   ((HRESULT)0x80040050L)

//--------------------------------------------------------------------------
//     Core TAPI Error messages
//--------------------------------------------------------------------------

//
// MessageId: TAPI_E_INVALADDRESSTYPE
//
// MessageText:
//
// The specified address type is not supported by this address.
//
#define TAPI_E_INVALADDRESSTYPE          ((HRESULT)0x80040051L)

//
// MessageId: TAPI_E_RESOURCEUNAVAIL
//
// MessageText:
//
// A resource needed to fulfill the request is not available.
//
#define TAPI_E_RESOURCEUNAVAIL           ((HRESULT)0x80040052L)

//
// MessageId: TAPI_E_PHONENOTOPEN
//
// MessageText:
//
// The phone is not open.
//
#define TAPI_E_PHONENOTOPEN              ((HRESULT)0x80040053L)

//
// MessageId: TAPI_E_CALLNOTSELECTED
//
// MessageText:
//
// The specified call is not currently selected.
//
#define TAPI_E_CALLNOTSELECTED           ((HRESULT)0x80040054L)

//
// MessageId: TAPI_E_WRONGEVENT
//
// MessageText:
//
// This information is not available for this type of event.
//
#define TAPI_E_WRONGEVENT                ((HRESULT)0x80040055L)

//
// MessageId: TAPI_E_NOFORMAT
//
// MessageText:
//
// The format is unknown
//
#define TAPI_E_NOFORMAT                  ((HRESULT)0x80040056L)

//
// MessageId: TAPI_E_INVALIDSTREAMSTATE
//
// MessageText:
//
// The operation is not permitted in current stream state.
//
#define TAPI_E_INVALIDSTREAMSTATE        ((HRESULT)0x80040057L)

//
// MessageId: TAPI_E_WRONG_STATE
//
// MessageText:
//
// The operation requested is not permitted for the current state.
//
#define TAPI_E_WRONG_STATE               ((HRESULT)0x80040058L)

//
// MessageId: TAPI_E_NOT_INITIALIZED
//
// MessageText:
//
// The object has not been initialized.
//
#define TAPI_E_NOT_INITIALIZED           ((HRESULT)0x80040059L)

//
// MessageId: TAPI_E_SERVICE_NOT_RUNNING
//
// MessageText:
//
// The Telephony Service could not be contacted.
//
#define TAPI_E_SERVICE_NOT_RUNNING       ((HRESULT)0x8004005AL)

#endif // #ifndef __TAPI3ERR_H__
