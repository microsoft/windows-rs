

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

#ifndef __rtccore_h__
#define __rtccore_h__

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

#ifndef __IRTCClient_FWD_DEFINED__
#define __IRTCClient_FWD_DEFINED__
typedef interface IRTCClient IRTCClient;

#endif 	/* __IRTCClient_FWD_DEFINED__ */


#ifndef __IRTCClient2_FWD_DEFINED__
#define __IRTCClient2_FWD_DEFINED__
typedef interface IRTCClient2 IRTCClient2;

#endif 	/* __IRTCClient2_FWD_DEFINED__ */


#ifndef __IRTCClientPresence_FWD_DEFINED__
#define __IRTCClientPresence_FWD_DEFINED__
typedef interface IRTCClientPresence IRTCClientPresence;

#endif 	/* __IRTCClientPresence_FWD_DEFINED__ */


#ifndef __IRTCClientPresence2_FWD_DEFINED__
#define __IRTCClientPresence2_FWD_DEFINED__
typedef interface IRTCClientPresence2 IRTCClientPresence2;

#endif 	/* __IRTCClientPresence2_FWD_DEFINED__ */


#ifndef __IRTCClientProvisioning_FWD_DEFINED__
#define __IRTCClientProvisioning_FWD_DEFINED__
typedef interface IRTCClientProvisioning IRTCClientProvisioning;

#endif 	/* __IRTCClientProvisioning_FWD_DEFINED__ */


#ifndef __IRTCClientProvisioning2_FWD_DEFINED__
#define __IRTCClientProvisioning2_FWD_DEFINED__
typedef interface IRTCClientProvisioning2 IRTCClientProvisioning2;

#endif 	/* __IRTCClientProvisioning2_FWD_DEFINED__ */


#ifndef __IRTCProfile_FWD_DEFINED__
#define __IRTCProfile_FWD_DEFINED__
typedef interface IRTCProfile IRTCProfile;

#endif 	/* __IRTCProfile_FWD_DEFINED__ */


#ifndef __IRTCProfile2_FWD_DEFINED__
#define __IRTCProfile2_FWD_DEFINED__
typedef interface IRTCProfile2 IRTCProfile2;

#endif 	/* __IRTCProfile2_FWD_DEFINED__ */


#ifndef __IRTCSession_FWD_DEFINED__
#define __IRTCSession_FWD_DEFINED__
typedef interface IRTCSession IRTCSession;

#endif 	/* __IRTCSession_FWD_DEFINED__ */


#ifndef __IRTCSession2_FWD_DEFINED__
#define __IRTCSession2_FWD_DEFINED__
typedef interface IRTCSession2 IRTCSession2;

#endif 	/* __IRTCSession2_FWD_DEFINED__ */


#ifndef __IRTCSessionCallControl_FWD_DEFINED__
#define __IRTCSessionCallControl_FWD_DEFINED__
typedef interface IRTCSessionCallControl IRTCSessionCallControl;

#endif 	/* __IRTCSessionCallControl_FWD_DEFINED__ */


#ifndef __IRTCParticipant_FWD_DEFINED__
#define __IRTCParticipant_FWD_DEFINED__
typedef interface IRTCParticipant IRTCParticipant;

#endif 	/* __IRTCParticipant_FWD_DEFINED__ */


#ifndef __IRTCRoamingEvent_FWD_DEFINED__
#define __IRTCRoamingEvent_FWD_DEFINED__
typedef interface IRTCRoamingEvent IRTCRoamingEvent;

#endif 	/* __IRTCRoamingEvent_FWD_DEFINED__ */


#ifndef __IRTCProfileEvent_FWD_DEFINED__
#define __IRTCProfileEvent_FWD_DEFINED__
typedef interface IRTCProfileEvent IRTCProfileEvent;

#endif 	/* __IRTCProfileEvent_FWD_DEFINED__ */


#ifndef __IRTCProfileEvent2_FWD_DEFINED__
#define __IRTCProfileEvent2_FWD_DEFINED__
typedef interface IRTCProfileEvent2 IRTCProfileEvent2;

#endif 	/* __IRTCProfileEvent2_FWD_DEFINED__ */


#ifndef __IRTCClientEvent_FWD_DEFINED__
#define __IRTCClientEvent_FWD_DEFINED__
typedef interface IRTCClientEvent IRTCClientEvent;

#endif 	/* __IRTCClientEvent_FWD_DEFINED__ */


#ifndef __IRTCRegistrationStateChangeEvent_FWD_DEFINED__
#define __IRTCRegistrationStateChangeEvent_FWD_DEFINED__
typedef interface IRTCRegistrationStateChangeEvent IRTCRegistrationStateChangeEvent;

#endif 	/* __IRTCRegistrationStateChangeEvent_FWD_DEFINED__ */


#ifndef __IRTCSessionStateChangeEvent_FWD_DEFINED__
#define __IRTCSessionStateChangeEvent_FWD_DEFINED__
typedef interface IRTCSessionStateChangeEvent IRTCSessionStateChangeEvent;

#endif 	/* __IRTCSessionStateChangeEvent_FWD_DEFINED__ */


#ifndef __IRTCSessionStateChangeEvent2_FWD_DEFINED__
#define __IRTCSessionStateChangeEvent2_FWD_DEFINED__
typedef interface IRTCSessionStateChangeEvent2 IRTCSessionStateChangeEvent2;

#endif 	/* __IRTCSessionStateChangeEvent2_FWD_DEFINED__ */


#ifndef __IRTCSessionOperationCompleteEvent_FWD_DEFINED__
#define __IRTCSessionOperationCompleteEvent_FWD_DEFINED__
typedef interface IRTCSessionOperationCompleteEvent IRTCSessionOperationCompleteEvent;

#endif 	/* __IRTCSessionOperationCompleteEvent_FWD_DEFINED__ */


#ifndef __IRTCSessionOperationCompleteEvent2_FWD_DEFINED__
#define __IRTCSessionOperationCompleteEvent2_FWD_DEFINED__
typedef interface IRTCSessionOperationCompleteEvent2 IRTCSessionOperationCompleteEvent2;

#endif 	/* __IRTCSessionOperationCompleteEvent2_FWD_DEFINED__ */


#ifndef __IRTCParticipantStateChangeEvent_FWD_DEFINED__
#define __IRTCParticipantStateChangeEvent_FWD_DEFINED__
typedef interface IRTCParticipantStateChangeEvent IRTCParticipantStateChangeEvent;

#endif 	/* __IRTCParticipantStateChangeEvent_FWD_DEFINED__ */


#ifndef __IRTCMediaEvent_FWD_DEFINED__
#define __IRTCMediaEvent_FWD_DEFINED__
typedef interface IRTCMediaEvent IRTCMediaEvent;

#endif 	/* __IRTCMediaEvent_FWD_DEFINED__ */


#ifndef __IRTCIntensityEvent_FWD_DEFINED__
#define __IRTCIntensityEvent_FWD_DEFINED__
typedef interface IRTCIntensityEvent IRTCIntensityEvent;

#endif 	/* __IRTCIntensityEvent_FWD_DEFINED__ */


#ifndef __IRTCMessagingEvent_FWD_DEFINED__
#define __IRTCMessagingEvent_FWD_DEFINED__
typedef interface IRTCMessagingEvent IRTCMessagingEvent;

#endif 	/* __IRTCMessagingEvent_FWD_DEFINED__ */


#ifndef __IRTCBuddyEvent_FWD_DEFINED__
#define __IRTCBuddyEvent_FWD_DEFINED__
typedef interface IRTCBuddyEvent IRTCBuddyEvent;

#endif 	/* __IRTCBuddyEvent_FWD_DEFINED__ */


#ifndef __IRTCBuddyEvent2_FWD_DEFINED__
#define __IRTCBuddyEvent2_FWD_DEFINED__
typedef interface IRTCBuddyEvent2 IRTCBuddyEvent2;

#endif 	/* __IRTCBuddyEvent2_FWD_DEFINED__ */


#ifndef __IRTCWatcherEvent_FWD_DEFINED__
#define __IRTCWatcherEvent_FWD_DEFINED__
typedef interface IRTCWatcherEvent IRTCWatcherEvent;

#endif 	/* __IRTCWatcherEvent_FWD_DEFINED__ */


#ifndef __IRTCWatcherEvent2_FWD_DEFINED__
#define __IRTCWatcherEvent2_FWD_DEFINED__
typedef interface IRTCWatcherEvent2 IRTCWatcherEvent2;

#endif 	/* __IRTCWatcherEvent2_FWD_DEFINED__ */


#ifndef __IRTCBuddyGroupEvent_FWD_DEFINED__
#define __IRTCBuddyGroupEvent_FWD_DEFINED__
typedef interface IRTCBuddyGroupEvent IRTCBuddyGroupEvent;

#endif 	/* __IRTCBuddyGroupEvent_FWD_DEFINED__ */


#ifndef __IRTCInfoEvent_FWD_DEFINED__
#define __IRTCInfoEvent_FWD_DEFINED__
typedef interface IRTCInfoEvent IRTCInfoEvent;

#endif 	/* __IRTCInfoEvent_FWD_DEFINED__ */


#ifndef __IRTCMediaRequestEvent_FWD_DEFINED__
#define __IRTCMediaRequestEvent_FWD_DEFINED__
typedef interface IRTCMediaRequestEvent IRTCMediaRequestEvent;

#endif 	/* __IRTCMediaRequestEvent_FWD_DEFINED__ */


#ifndef __IRTCReInviteEvent_FWD_DEFINED__
#define __IRTCReInviteEvent_FWD_DEFINED__
typedef interface IRTCReInviteEvent IRTCReInviteEvent;

#endif 	/* __IRTCReInviteEvent_FWD_DEFINED__ */


#ifndef __IRTCPresencePropertyEvent_FWD_DEFINED__
#define __IRTCPresencePropertyEvent_FWD_DEFINED__
typedef interface IRTCPresencePropertyEvent IRTCPresencePropertyEvent;

#endif 	/* __IRTCPresencePropertyEvent_FWD_DEFINED__ */


#ifndef __IRTCPresenceDataEvent_FWD_DEFINED__
#define __IRTCPresenceDataEvent_FWD_DEFINED__
typedef interface IRTCPresenceDataEvent IRTCPresenceDataEvent;

#endif 	/* __IRTCPresenceDataEvent_FWD_DEFINED__ */


#ifndef __IRTCPresenceStatusEvent_FWD_DEFINED__
#define __IRTCPresenceStatusEvent_FWD_DEFINED__
typedef interface IRTCPresenceStatusEvent IRTCPresenceStatusEvent;

#endif 	/* __IRTCPresenceStatusEvent_FWD_DEFINED__ */


#ifndef __IRTCCollection_FWD_DEFINED__
#define __IRTCCollection_FWD_DEFINED__
typedef interface IRTCCollection IRTCCollection;

#endif 	/* __IRTCCollection_FWD_DEFINED__ */


#ifndef __IRTCEnumParticipants_FWD_DEFINED__
#define __IRTCEnumParticipants_FWD_DEFINED__
typedef interface IRTCEnumParticipants IRTCEnumParticipants;

#endif 	/* __IRTCEnumParticipants_FWD_DEFINED__ */


#ifndef __IRTCEnumProfiles_FWD_DEFINED__
#define __IRTCEnumProfiles_FWD_DEFINED__
typedef interface IRTCEnumProfiles IRTCEnumProfiles;

#endif 	/* __IRTCEnumProfiles_FWD_DEFINED__ */


#ifndef __IRTCEnumBuddies_FWD_DEFINED__
#define __IRTCEnumBuddies_FWD_DEFINED__
typedef interface IRTCEnumBuddies IRTCEnumBuddies;

#endif 	/* __IRTCEnumBuddies_FWD_DEFINED__ */


#ifndef __IRTCEnumWatchers_FWD_DEFINED__
#define __IRTCEnumWatchers_FWD_DEFINED__
typedef interface IRTCEnumWatchers IRTCEnumWatchers;

#endif 	/* __IRTCEnumWatchers_FWD_DEFINED__ */


#ifndef __IRTCEnumGroups_FWD_DEFINED__
#define __IRTCEnumGroups_FWD_DEFINED__
typedef interface IRTCEnumGroups IRTCEnumGroups;

#endif 	/* __IRTCEnumGroups_FWD_DEFINED__ */


#ifndef __IRTCPresenceContact_FWD_DEFINED__
#define __IRTCPresenceContact_FWD_DEFINED__
typedef interface IRTCPresenceContact IRTCPresenceContact;

#endif 	/* __IRTCPresenceContact_FWD_DEFINED__ */


#ifndef __IRTCBuddy_FWD_DEFINED__
#define __IRTCBuddy_FWD_DEFINED__
typedef interface IRTCBuddy IRTCBuddy;

#endif 	/* __IRTCBuddy_FWD_DEFINED__ */


#ifndef __IRTCBuddy2_FWD_DEFINED__
#define __IRTCBuddy2_FWD_DEFINED__
typedef interface IRTCBuddy2 IRTCBuddy2;

#endif 	/* __IRTCBuddy2_FWD_DEFINED__ */


#ifndef __IRTCWatcher_FWD_DEFINED__
#define __IRTCWatcher_FWD_DEFINED__
typedef interface IRTCWatcher IRTCWatcher;

#endif 	/* __IRTCWatcher_FWD_DEFINED__ */


#ifndef __IRTCWatcher2_FWD_DEFINED__
#define __IRTCWatcher2_FWD_DEFINED__
typedef interface IRTCWatcher2 IRTCWatcher2;

#endif 	/* __IRTCWatcher2_FWD_DEFINED__ */


#ifndef __IRTCBuddyGroup_FWD_DEFINED__
#define __IRTCBuddyGroup_FWD_DEFINED__
typedef interface IRTCBuddyGroup IRTCBuddyGroup;

#endif 	/* __IRTCBuddyGroup_FWD_DEFINED__ */


#ifndef __IRTCEventNotification_FWD_DEFINED__
#define __IRTCEventNotification_FWD_DEFINED__
typedef interface IRTCEventNotification IRTCEventNotification;

#endif 	/* __IRTCEventNotification_FWD_DEFINED__ */


#ifndef __IRTCDispatchEventNotification_FWD_DEFINED__
#define __IRTCDispatchEventNotification_FWD_DEFINED__
typedef interface IRTCDispatchEventNotification IRTCDispatchEventNotification;

#endif 	/* __IRTCDispatchEventNotification_FWD_DEFINED__ */


#ifndef __IRTCPortManager_FWD_DEFINED__
#define __IRTCPortManager_FWD_DEFINED__
typedef interface IRTCPortManager IRTCPortManager;

#endif 	/* __IRTCPortManager_FWD_DEFINED__ */


#ifndef __IRTCSessionPortManagement_FWD_DEFINED__
#define __IRTCSessionPortManagement_FWD_DEFINED__
typedef interface IRTCSessionPortManagement IRTCSessionPortManagement;

#endif 	/* __IRTCSessionPortManagement_FWD_DEFINED__ */


#ifndef __IRTCClientPortManagement_FWD_DEFINED__
#define __IRTCClientPortManagement_FWD_DEFINED__
typedef interface IRTCClientPortManagement IRTCClientPortManagement;

#endif 	/* __IRTCClientPortManagement_FWD_DEFINED__ */


#ifndef __IRTCUserSearch_FWD_DEFINED__
#define __IRTCUserSearch_FWD_DEFINED__
typedef interface IRTCUserSearch IRTCUserSearch;

#endif 	/* __IRTCUserSearch_FWD_DEFINED__ */


#ifndef __IRTCUserSearchQuery_FWD_DEFINED__
#define __IRTCUserSearchQuery_FWD_DEFINED__
typedef interface IRTCUserSearchQuery IRTCUserSearchQuery;

#endif 	/* __IRTCUserSearchQuery_FWD_DEFINED__ */


#ifndef __IRTCUserSearchResult_FWD_DEFINED__
#define __IRTCUserSearchResult_FWD_DEFINED__
typedef interface IRTCUserSearchResult IRTCUserSearchResult;

#endif 	/* __IRTCUserSearchResult_FWD_DEFINED__ */


#ifndef __IRTCEnumUserSearchResults_FWD_DEFINED__
#define __IRTCEnumUserSearchResults_FWD_DEFINED__
typedef interface IRTCEnumUserSearchResults IRTCEnumUserSearchResults;

#endif 	/* __IRTCEnumUserSearchResults_FWD_DEFINED__ */


#ifndef __IRTCUserSearchResultsEvent_FWD_DEFINED__
#define __IRTCUserSearchResultsEvent_FWD_DEFINED__
typedef interface IRTCUserSearchResultsEvent IRTCUserSearchResultsEvent;

#endif 	/* __IRTCUserSearchResultsEvent_FWD_DEFINED__ */


#ifndef __IRTCSessionReferStatusEvent_FWD_DEFINED__
#define __IRTCSessionReferStatusEvent_FWD_DEFINED__
typedef interface IRTCSessionReferStatusEvent IRTCSessionReferStatusEvent;

#endif 	/* __IRTCSessionReferStatusEvent_FWD_DEFINED__ */


#ifndef __IRTCSessionReferredEvent_FWD_DEFINED__
#define __IRTCSessionReferredEvent_FWD_DEFINED__
typedef interface IRTCSessionReferredEvent IRTCSessionReferredEvent;

#endif 	/* __IRTCSessionReferredEvent_FWD_DEFINED__ */


#ifndef __IRTCSessionDescriptionManager_FWD_DEFINED__
#define __IRTCSessionDescriptionManager_FWD_DEFINED__
typedef interface IRTCSessionDescriptionManager IRTCSessionDescriptionManager;

#endif 	/* __IRTCSessionDescriptionManager_FWD_DEFINED__ */


#ifndef __IRTCEnumPresenceDevices_FWD_DEFINED__
#define __IRTCEnumPresenceDevices_FWD_DEFINED__
typedef interface IRTCEnumPresenceDevices IRTCEnumPresenceDevices;

#endif 	/* __IRTCEnumPresenceDevices_FWD_DEFINED__ */


#ifndef __IRTCPresenceDevice_FWD_DEFINED__
#define __IRTCPresenceDevice_FWD_DEFINED__
typedef interface IRTCPresenceDevice IRTCPresenceDevice;

#endif 	/* __IRTCPresenceDevice_FWD_DEFINED__ */


#ifndef __IRTCProfile_FWD_DEFINED__
#define __IRTCProfile_FWD_DEFINED__
typedef interface IRTCProfile IRTCProfile;

#endif 	/* __IRTCProfile_FWD_DEFINED__ */


#ifndef __IRTCProfile2_FWD_DEFINED__
#define __IRTCProfile2_FWD_DEFINED__
typedef interface IRTCProfile2 IRTCProfile2;

#endif 	/* __IRTCProfile2_FWD_DEFINED__ */


#ifndef __IRTCEnumProfiles_FWD_DEFINED__
#define __IRTCEnumProfiles_FWD_DEFINED__
typedef interface IRTCEnumProfiles IRTCEnumProfiles;

#endif 	/* __IRTCEnumProfiles_FWD_DEFINED__ */


#ifndef __IRTCSession_FWD_DEFINED__
#define __IRTCSession_FWD_DEFINED__
typedef interface IRTCSession IRTCSession;

#endif 	/* __IRTCSession_FWD_DEFINED__ */


#ifndef __IRTCSession2_FWD_DEFINED__
#define __IRTCSession2_FWD_DEFINED__
typedef interface IRTCSession2 IRTCSession2;

#endif 	/* __IRTCSession2_FWD_DEFINED__ */


#ifndef __IRTCSessionCallControl_FWD_DEFINED__
#define __IRTCSessionCallControl_FWD_DEFINED__
typedef interface IRTCSessionCallControl IRTCSessionCallControl;

#endif 	/* __IRTCSessionCallControl_FWD_DEFINED__ */


#ifndef __IRTCParticipant_FWD_DEFINED__
#define __IRTCParticipant_FWD_DEFINED__
typedef interface IRTCParticipant IRTCParticipant;

#endif 	/* __IRTCParticipant_FWD_DEFINED__ */


#ifndef __IRTCEnumParticipants_FWD_DEFINED__
#define __IRTCEnumParticipants_FWD_DEFINED__
typedef interface IRTCEnumParticipants IRTCEnumParticipants;

#endif 	/* __IRTCEnumParticipants_FWD_DEFINED__ */


#ifndef __IRTCCollection_FWD_DEFINED__
#define __IRTCCollection_FWD_DEFINED__
typedef interface IRTCCollection IRTCCollection;

#endif 	/* __IRTCCollection_FWD_DEFINED__ */


#ifndef __IRTCPresenceContact_FWD_DEFINED__
#define __IRTCPresenceContact_FWD_DEFINED__
typedef interface IRTCPresenceContact IRTCPresenceContact;

#endif 	/* __IRTCPresenceContact_FWD_DEFINED__ */


#ifndef __IRTCBuddy_FWD_DEFINED__
#define __IRTCBuddy_FWD_DEFINED__
typedef interface IRTCBuddy IRTCBuddy;

#endif 	/* __IRTCBuddy_FWD_DEFINED__ */


#ifndef __IRTCBuddy2_FWD_DEFINED__
#define __IRTCBuddy2_FWD_DEFINED__
typedef interface IRTCBuddy2 IRTCBuddy2;

#endif 	/* __IRTCBuddy2_FWD_DEFINED__ */


#ifndef __IRTCEnumBuddies_FWD_DEFINED__
#define __IRTCEnumBuddies_FWD_DEFINED__
typedef interface IRTCEnumBuddies IRTCEnumBuddies;

#endif 	/* __IRTCEnumBuddies_FWD_DEFINED__ */


#ifndef __IRTCWatcher_FWD_DEFINED__
#define __IRTCWatcher_FWD_DEFINED__
typedef interface IRTCWatcher IRTCWatcher;

#endif 	/* __IRTCWatcher_FWD_DEFINED__ */


#ifndef __IRTCWatcher2_FWD_DEFINED__
#define __IRTCWatcher2_FWD_DEFINED__
typedef interface IRTCWatcher2 IRTCWatcher2;

#endif 	/* __IRTCWatcher2_FWD_DEFINED__ */


#ifndef __IRTCEnumWatchers_FWD_DEFINED__
#define __IRTCEnumWatchers_FWD_DEFINED__
typedef interface IRTCEnumWatchers IRTCEnumWatchers;

#endif 	/* __IRTCEnumWatchers_FWD_DEFINED__ */


#ifndef __IRTCBuddyGroup_FWD_DEFINED__
#define __IRTCBuddyGroup_FWD_DEFINED__
typedef interface IRTCBuddyGroup IRTCBuddyGroup;

#endif 	/* __IRTCBuddyGroup_FWD_DEFINED__ */


#ifndef __IRTCEnumGroups_FWD_DEFINED__
#define __IRTCEnumGroups_FWD_DEFINED__
typedef interface IRTCEnumGroups IRTCEnumGroups;

#endif 	/* __IRTCEnumGroups_FWD_DEFINED__ */


#ifndef __IRTCUserSearchQuery_FWD_DEFINED__
#define __IRTCUserSearchQuery_FWD_DEFINED__
typedef interface IRTCUserSearchQuery IRTCUserSearchQuery;

#endif 	/* __IRTCUserSearchQuery_FWD_DEFINED__ */


#ifndef __IRTCUserSearchResult_FWD_DEFINED__
#define __IRTCUserSearchResult_FWD_DEFINED__
typedef interface IRTCUserSearchResult IRTCUserSearchResult;

#endif 	/* __IRTCUserSearchResult_FWD_DEFINED__ */


#ifndef __IRTCEnumUserSearchResults_FWD_DEFINED__
#define __IRTCEnumUserSearchResults_FWD_DEFINED__
typedef interface IRTCEnumUserSearchResults IRTCEnumUserSearchResults;

#endif 	/* __IRTCEnumUserSearchResults_FWD_DEFINED__ */


#ifndef __IRTCEventNotification_FWD_DEFINED__
#define __IRTCEventNotification_FWD_DEFINED__
typedef interface IRTCEventNotification IRTCEventNotification;

#endif 	/* __IRTCEventNotification_FWD_DEFINED__ */


#ifndef __IRTCClientEvent_FWD_DEFINED__
#define __IRTCClientEvent_FWD_DEFINED__
typedef interface IRTCClientEvent IRTCClientEvent;

#endif 	/* __IRTCClientEvent_FWD_DEFINED__ */


#ifndef __IRTCRegistrationStateChangeEvent_FWD_DEFINED__
#define __IRTCRegistrationStateChangeEvent_FWD_DEFINED__
typedef interface IRTCRegistrationStateChangeEvent IRTCRegistrationStateChangeEvent;

#endif 	/* __IRTCRegistrationStateChangeEvent_FWD_DEFINED__ */


#ifndef __IRTCSessionStateChangeEvent_FWD_DEFINED__
#define __IRTCSessionStateChangeEvent_FWD_DEFINED__
typedef interface IRTCSessionStateChangeEvent IRTCSessionStateChangeEvent;

#endif 	/* __IRTCSessionStateChangeEvent_FWD_DEFINED__ */


#ifndef __IRTCSessionStateChangeEvent2_FWD_DEFINED__
#define __IRTCSessionStateChangeEvent2_FWD_DEFINED__
typedef interface IRTCSessionStateChangeEvent2 IRTCSessionStateChangeEvent2;

#endif 	/* __IRTCSessionStateChangeEvent2_FWD_DEFINED__ */


#ifndef __IRTCSessionOperationCompleteEvent_FWD_DEFINED__
#define __IRTCSessionOperationCompleteEvent_FWD_DEFINED__
typedef interface IRTCSessionOperationCompleteEvent IRTCSessionOperationCompleteEvent;

#endif 	/* __IRTCSessionOperationCompleteEvent_FWD_DEFINED__ */


#ifndef __IRTCSessionOperationCompleteEvent2_FWD_DEFINED__
#define __IRTCSessionOperationCompleteEvent2_FWD_DEFINED__
typedef interface IRTCSessionOperationCompleteEvent2 IRTCSessionOperationCompleteEvent2;

#endif 	/* __IRTCSessionOperationCompleteEvent2_FWD_DEFINED__ */


#ifndef __IRTCParticipantStateChangeEvent_FWD_DEFINED__
#define __IRTCParticipantStateChangeEvent_FWD_DEFINED__
typedef interface IRTCParticipantStateChangeEvent IRTCParticipantStateChangeEvent;

#endif 	/* __IRTCParticipantStateChangeEvent_FWD_DEFINED__ */


#ifndef __IRTCMediaEvent_FWD_DEFINED__
#define __IRTCMediaEvent_FWD_DEFINED__
typedef interface IRTCMediaEvent IRTCMediaEvent;

#endif 	/* __IRTCMediaEvent_FWD_DEFINED__ */


#ifndef __IRTCIntensityEvent_FWD_DEFINED__
#define __IRTCIntensityEvent_FWD_DEFINED__
typedef interface IRTCIntensityEvent IRTCIntensityEvent;

#endif 	/* __IRTCIntensityEvent_FWD_DEFINED__ */


#ifndef __IRTCMessagingEvent_FWD_DEFINED__
#define __IRTCMessagingEvent_FWD_DEFINED__
typedef interface IRTCMessagingEvent IRTCMessagingEvent;

#endif 	/* __IRTCMessagingEvent_FWD_DEFINED__ */


#ifndef __IRTCBuddyEvent_FWD_DEFINED__
#define __IRTCBuddyEvent_FWD_DEFINED__
typedef interface IRTCBuddyEvent IRTCBuddyEvent;

#endif 	/* __IRTCBuddyEvent_FWD_DEFINED__ */


#ifndef __IRTCBuddyEvent2_FWD_DEFINED__
#define __IRTCBuddyEvent2_FWD_DEFINED__
typedef interface IRTCBuddyEvent2 IRTCBuddyEvent2;

#endif 	/* __IRTCBuddyEvent2_FWD_DEFINED__ */


#ifndef __IRTCWatcherEvent_FWD_DEFINED__
#define __IRTCWatcherEvent_FWD_DEFINED__
typedef interface IRTCWatcherEvent IRTCWatcherEvent;

#endif 	/* __IRTCWatcherEvent_FWD_DEFINED__ */


#ifndef __IRTCPortManager_FWD_DEFINED__
#define __IRTCPortManager_FWD_DEFINED__
typedef interface IRTCPortManager IRTCPortManager;

#endif 	/* __IRTCPortManager_FWD_DEFINED__ */


#ifndef __IRTCSessionPortManagement_FWD_DEFINED__
#define __IRTCSessionPortManagement_FWD_DEFINED__
typedef interface IRTCSessionPortManagement IRTCSessionPortManagement;

#endif 	/* __IRTCSessionPortManagement_FWD_DEFINED__ */


#ifndef __IRTCClientPortManagement_FWD_DEFINED__
#define __IRTCClientPortManagement_FWD_DEFINED__
typedef interface IRTCClientPortManagement IRTCClientPortManagement;

#endif 	/* __IRTCClientPortManagement_FWD_DEFINED__ */


#ifndef __IRTCWatcherEvent2_FWD_DEFINED__
#define __IRTCWatcherEvent2_FWD_DEFINED__
typedef interface IRTCWatcherEvent2 IRTCWatcherEvent2;

#endif 	/* __IRTCWatcherEvent2_FWD_DEFINED__ */


#ifndef __IRTCBuddyGroupEvent_FWD_DEFINED__
#define __IRTCBuddyGroupEvent_FWD_DEFINED__
typedef interface IRTCBuddyGroupEvent IRTCBuddyGroupEvent;

#endif 	/* __IRTCBuddyGroupEvent_FWD_DEFINED__ */


#ifndef __IRTCProfileEvent_FWD_DEFINED__
#define __IRTCProfileEvent_FWD_DEFINED__
typedef interface IRTCProfileEvent IRTCProfileEvent;

#endif 	/* __IRTCProfileEvent_FWD_DEFINED__ */


#ifndef __IRTCProfileEvent2_FWD_DEFINED__
#define __IRTCProfileEvent2_FWD_DEFINED__
typedef interface IRTCProfileEvent2 IRTCProfileEvent2;

#endif 	/* __IRTCProfileEvent2_FWD_DEFINED__ */


#ifndef __IRTCUserSearchResultsEvent_FWD_DEFINED__
#define __IRTCUserSearchResultsEvent_FWD_DEFINED__
typedef interface IRTCUserSearchResultsEvent IRTCUserSearchResultsEvent;

#endif 	/* __IRTCUserSearchResultsEvent_FWD_DEFINED__ */


#ifndef __IRTCInfoEvent_FWD_DEFINED__
#define __IRTCInfoEvent_FWD_DEFINED__
typedef interface IRTCInfoEvent IRTCInfoEvent;

#endif 	/* __IRTCInfoEvent_FWD_DEFINED__ */


#ifndef __IRTCRoamingEvent_FWD_DEFINED__
#define __IRTCRoamingEvent_FWD_DEFINED__
typedef interface IRTCRoamingEvent IRTCRoamingEvent;

#endif 	/* __IRTCRoamingEvent_FWD_DEFINED__ */


#ifndef __IRTCMediaRequestEvent_FWD_DEFINED__
#define __IRTCMediaRequestEvent_FWD_DEFINED__
typedef interface IRTCMediaRequestEvent IRTCMediaRequestEvent;

#endif 	/* __IRTCMediaRequestEvent_FWD_DEFINED__ */


#ifndef __IRTCReInviteEvent_FWD_DEFINED__
#define __IRTCReInviteEvent_FWD_DEFINED__
typedef interface IRTCReInviteEvent IRTCReInviteEvent;

#endif 	/* __IRTCReInviteEvent_FWD_DEFINED__ */


#ifndef __IRTCPresencePropertyEvent_FWD_DEFINED__
#define __IRTCPresencePropertyEvent_FWD_DEFINED__
typedef interface IRTCPresencePropertyEvent IRTCPresencePropertyEvent;

#endif 	/* __IRTCPresencePropertyEvent_FWD_DEFINED__ */


#ifndef __IRTCPresenceDataEvent_FWD_DEFINED__
#define __IRTCPresenceDataEvent_FWD_DEFINED__
typedef interface IRTCPresenceDataEvent IRTCPresenceDataEvent;

#endif 	/* __IRTCPresenceDataEvent_FWD_DEFINED__ */


#ifndef __IRTCPresenceStatusEvent_FWD_DEFINED__
#define __IRTCPresenceStatusEvent_FWD_DEFINED__
typedef interface IRTCPresenceStatusEvent IRTCPresenceStatusEvent;

#endif 	/* __IRTCPresenceStatusEvent_FWD_DEFINED__ */


#ifndef __IRTCSessionReferStatusEvent_FWD_DEFINED__
#define __IRTCSessionReferStatusEvent_FWD_DEFINED__
typedef interface IRTCSessionReferStatusEvent IRTCSessionReferStatusEvent;

#endif 	/* __IRTCSessionReferStatusEvent_FWD_DEFINED__ */


#ifndef __IRTCSessionReferredEvent_FWD_DEFINED__
#define __IRTCSessionReferredEvent_FWD_DEFINED__
typedef interface IRTCSessionReferredEvent IRTCSessionReferredEvent;

#endif 	/* __IRTCSessionReferredEvent_FWD_DEFINED__ */


#ifndef __IRTCSessionDescriptionManager_FWD_DEFINED__
#define __IRTCSessionDescriptionManager_FWD_DEFINED__
typedef interface IRTCSessionDescriptionManager IRTCSessionDescriptionManager;

#endif 	/* __IRTCSessionDescriptionManager_FWD_DEFINED__ */


#ifndef __IRTCEnumPresenceDevices_FWD_DEFINED__
#define __IRTCEnumPresenceDevices_FWD_DEFINED__
typedef interface IRTCEnumPresenceDevices IRTCEnumPresenceDevices;

#endif 	/* __IRTCEnumPresenceDevices_FWD_DEFINED__ */


#ifndef __IRTCPresenceDevice_FWD_DEFINED__
#define __IRTCPresenceDevice_FWD_DEFINED__
typedef interface IRTCPresenceDevice IRTCPresenceDevice;

#endif 	/* __IRTCPresenceDevice_FWD_DEFINED__ */


#ifndef __IRTCDispatchEventNotification_FWD_DEFINED__
#define __IRTCDispatchEventNotification_FWD_DEFINED__
typedef interface IRTCDispatchEventNotification IRTCDispatchEventNotification;

#endif 	/* __IRTCDispatchEventNotification_FWD_DEFINED__ */


#ifndef __RTCClient_FWD_DEFINED__
#define __RTCClient_FWD_DEFINED__

#ifdef __cplusplus
typedef class RTCClient RTCClient;
#else
typedef struct RTCClient RTCClient;
#endif /* __cplusplus */

#endif 	/* __RTCClient_FWD_DEFINED__ */


/* header files for imported files */
#include "oaidl.h"
#include "ocidl.h"
#include "control.h"

#ifdef __cplusplus
extern "C"{
#endif 


/* interface __MIDL_itf_rtccore_0000_0000 */
/* [local] */ 

//  Copyright (c) Microsoft Corporation.  All rights reserved.
#include <winapifamily.h>
#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)
typedef 
enum RTC_AUDIO_DEVICE
    {
        RTCAD_SPEAKER	= 0,
        RTCAD_MICROPHONE	= ( RTCAD_SPEAKER + 1 ) 
    } 	RTC_AUDIO_DEVICE;

typedef 
enum RTC_VIDEO_DEVICE
    {
        RTCVD_RECEIVE	= 0,
        RTCVD_PREVIEW	= ( RTCVD_RECEIVE + 1 ) 
    } 	RTC_VIDEO_DEVICE;

typedef 
enum RTC_EVENT
    {
        RTCE_CLIENT	= 0,
        RTCE_REGISTRATION_STATE_CHANGE	= ( RTCE_CLIENT + 1 ) ,
        RTCE_SESSION_STATE_CHANGE	= ( RTCE_REGISTRATION_STATE_CHANGE + 1 ) ,
        RTCE_SESSION_OPERATION_COMPLETE	= ( RTCE_SESSION_STATE_CHANGE + 1 ) ,
        RTCE_PARTICIPANT_STATE_CHANGE	= ( RTCE_SESSION_OPERATION_COMPLETE + 1 ) ,
        RTCE_MEDIA	= ( RTCE_PARTICIPANT_STATE_CHANGE + 1 ) ,
        RTCE_INTENSITY	= ( RTCE_MEDIA + 1 ) ,
        RTCE_MESSAGING	= ( RTCE_INTENSITY + 1 ) ,
        RTCE_BUDDY	= ( RTCE_MESSAGING + 1 ) ,
        RTCE_WATCHER	= ( RTCE_BUDDY + 1 ) ,
        RTCE_PROFILE	= ( RTCE_WATCHER + 1 ) ,
        RTCE_USERSEARCH	= ( RTCE_PROFILE + 1 ) ,
        RTCE_INFO	= ( RTCE_USERSEARCH + 1 ) ,
        RTCE_GROUP	= ( RTCE_INFO + 1 ) ,
        RTCE_MEDIA_REQUEST	= ( RTCE_GROUP + 1 ) ,
        RTCE_ROAMING	= ( RTCE_MEDIA_REQUEST + 1 ) ,
        RTCE_PRESENCE_PROPERTY	= ( RTCE_ROAMING + 1 ) ,
        RTCE_PRESENCE_DATA	= ( RTCE_PRESENCE_PROPERTY + 1 ) ,
        RTCE_PRESENCE_STATUS	= ( RTCE_PRESENCE_DATA + 1 ) ,
        RTCE_SESSION_REFER_STATUS	= ( RTCE_PRESENCE_STATUS + 1 ) ,
        RTCE_SESSION_REFERRED	= ( RTCE_SESSION_REFER_STATUS + 1 ) ,
        RTCE_REINVITE	= ( RTCE_SESSION_REFERRED + 1 ) 
    } 	RTC_EVENT;

typedef 
enum RTC_LISTEN_MODE
    {
        RTCLM_NONE	= 0,
        RTCLM_DYNAMIC	= ( RTCLM_NONE + 1 ) ,
        RTCLM_BOTH	= ( RTCLM_DYNAMIC + 1 ) 
    } 	RTC_LISTEN_MODE;

typedef 
enum RTC_CLIENT_EVENT_TYPE
    {
        RTCCET_VOLUME_CHANGE	= 0,
        RTCCET_DEVICE_CHANGE	= ( RTCCET_VOLUME_CHANGE + 1 ) ,
        RTCCET_NETWORK_QUALITY_CHANGE	= ( RTCCET_DEVICE_CHANGE + 1 ) ,
        RTCCET_ASYNC_CLEANUP_DONE	= ( RTCCET_NETWORK_QUALITY_CHANGE + 1 ) 
    } 	RTC_CLIENT_EVENT_TYPE;

typedef 
enum RTC_BUDDY_EVENT_TYPE
    {
        RTCBET_BUDDY_ADD	= 0,
        RTCBET_BUDDY_REMOVE	= ( RTCBET_BUDDY_ADD + 1 ) ,
        RTCBET_BUDDY_UPDATE	= ( RTCBET_BUDDY_REMOVE + 1 ) ,
        RTCBET_BUDDY_STATE_CHANGE	= ( RTCBET_BUDDY_UPDATE + 1 ) ,
        RTCBET_BUDDY_ROAMED	= ( RTCBET_BUDDY_STATE_CHANGE + 1 ) ,
        RTCBET_BUDDY_SUBSCRIBED	= ( RTCBET_BUDDY_ROAMED + 1 ) 
    } 	RTC_BUDDY_EVENT_TYPE;

typedef 
enum RTC_WATCHER_EVENT_TYPE
    {
        RTCWET_WATCHER_ADD	= 0,
        RTCWET_WATCHER_REMOVE	= ( RTCWET_WATCHER_ADD + 1 ) ,
        RTCWET_WATCHER_UPDATE	= ( RTCWET_WATCHER_REMOVE + 1 ) ,
        RTCWET_WATCHER_OFFERING	= ( RTCWET_WATCHER_UPDATE + 1 ) ,
        RTCWET_WATCHER_ROAMED	= ( RTCWET_WATCHER_OFFERING + 1 ) 
    } 	RTC_WATCHER_EVENT_TYPE;

typedef 
enum RTC_GROUP_EVENT_TYPE
    {
        RTCGET_GROUP_ADD	= 0,
        RTCGET_GROUP_REMOVE	= ( RTCGET_GROUP_ADD + 1 ) ,
        RTCGET_GROUP_UPDATE	= ( RTCGET_GROUP_REMOVE + 1 ) ,
        RTCGET_GROUP_BUDDY_ADD	= ( RTCGET_GROUP_UPDATE + 1 ) ,
        RTCGET_GROUP_BUDDY_REMOVE	= ( RTCGET_GROUP_BUDDY_ADD + 1 ) ,
        RTCGET_GROUP_ROAMED	= ( RTCGET_GROUP_BUDDY_REMOVE + 1 ) 
    } 	RTC_GROUP_EVENT_TYPE;

typedef 
enum RTC_TERMINATE_REASON
    {
        RTCTR_NORMAL	= 0,
        RTCTR_DND	= ( RTCTR_NORMAL + 1 ) ,
        RTCTR_BUSY	= ( RTCTR_DND + 1 ) ,
        RTCTR_REJECT	= ( RTCTR_BUSY + 1 ) ,
        RTCTR_TIMEOUT	= ( RTCTR_REJECT + 1 ) ,
        RTCTR_SHUTDOWN	= ( RTCTR_TIMEOUT + 1 ) ,
        RTCTR_INSUFFICIENT_SECURITY_LEVEL	= ( RTCTR_SHUTDOWN + 1 ) ,
        RTCTR_NOT_SUPPORTED	= ( RTCTR_INSUFFICIENT_SECURITY_LEVEL + 1 ) 
    } 	RTC_TERMINATE_REASON;

typedef 
enum RTC_REGISTRATION_STATE
    {
        RTCRS_NOT_REGISTERED	= 0,
        RTCRS_REGISTERING	= ( RTCRS_NOT_REGISTERED + 1 ) ,
        RTCRS_REGISTERED	= ( RTCRS_REGISTERING + 1 ) ,
        RTCRS_REJECTED	= ( RTCRS_REGISTERED + 1 ) ,
        RTCRS_UNREGISTERING	= ( RTCRS_REJECTED + 1 ) ,
        RTCRS_ERROR	= ( RTCRS_UNREGISTERING + 1 ) ,
        RTCRS_LOGGED_OFF	= ( RTCRS_ERROR + 1 ) ,
        RTCRS_LOCAL_PA_LOGGED_OFF	= ( RTCRS_LOGGED_OFF + 1 ) ,
        RTCRS_REMOTE_PA_LOGGED_OFF	= ( RTCRS_LOCAL_PA_LOGGED_OFF + 1 ) 
    } 	RTC_REGISTRATION_STATE;

typedef 
enum RTC_SESSION_STATE
    {
        RTCSS_IDLE	= 0,
        RTCSS_INCOMING	= ( RTCSS_IDLE + 1 ) ,
        RTCSS_ANSWERING	= ( RTCSS_INCOMING + 1 ) ,
        RTCSS_INPROGRESS	= ( RTCSS_ANSWERING + 1 ) ,
        RTCSS_CONNECTED	= ( RTCSS_INPROGRESS + 1 ) ,
        RTCSS_DISCONNECTED	= ( RTCSS_CONNECTED + 1 ) ,
        RTCSS_HOLD	= ( RTCSS_DISCONNECTED + 1 ) ,
        RTCSS_REFER	= ( RTCSS_HOLD + 1 ) 
    } 	RTC_SESSION_STATE;

typedef 
enum RTC_PARTICIPANT_STATE
    {
        RTCPS_IDLE	= 0,
        RTCPS_PENDING	= ( RTCPS_IDLE + 1 ) ,
        RTCPS_INCOMING	= ( RTCPS_PENDING + 1 ) ,
        RTCPS_ANSWERING	= ( RTCPS_INCOMING + 1 ) ,
        RTCPS_INPROGRESS	= ( RTCPS_ANSWERING + 1 ) ,
        RTCPS_ALERTING	= ( RTCPS_INPROGRESS + 1 ) ,
        RTCPS_CONNECTED	= ( RTCPS_ALERTING + 1 ) ,
        RTCPS_DISCONNECTING	= ( RTCPS_CONNECTED + 1 ) ,
        RTCPS_DISCONNECTED	= ( RTCPS_DISCONNECTING + 1 ) 
    } 	RTC_PARTICIPANT_STATE;

typedef 
enum RTC_WATCHER_STATE
    {
        RTCWS_UNKNOWN	= 0,
        RTCWS_OFFERING	= ( RTCWS_UNKNOWN + 1 ) ,
        RTCWS_ALLOWED	= ( RTCWS_OFFERING + 1 ) ,
        RTCWS_BLOCKED	= ( RTCWS_ALLOWED + 1 ) ,
        RTCWS_DENIED	= ( RTCWS_BLOCKED + 1 ) ,
        RTCWS_PROMPT	= ( RTCWS_DENIED + 1 ) 
    } 	RTC_WATCHER_STATE;

typedef 
enum RTC_ACE_SCOPE
    {
        RTCAS_SCOPE_USER	= 0,
        RTCAS_SCOPE_DOMAIN	= ( RTCAS_SCOPE_USER + 1 ) ,
        RTCAS_SCOPE_ALL	= ( RTCAS_SCOPE_DOMAIN + 1 ) 
    } 	RTC_ACE_SCOPE;

typedef 
enum RTC_OFFER_WATCHER_MODE
    {
        RTCOWM_OFFER_WATCHER_EVENT	= 0,
        RTCOWM_AUTOMATICALLY_ADD_WATCHER	= ( RTCOWM_OFFER_WATCHER_EVENT + 1 ) 
    } 	RTC_OFFER_WATCHER_MODE;

typedef 
enum RTC_WATCHER_MATCH_MODE
    {
        RTCWMM_EXACT_MATCH	= 0,
        RTCWMM_BEST_ACE_MATCH	= ( RTCWMM_EXACT_MATCH + 1 ) 
    } 	RTC_WATCHER_MATCH_MODE;

typedef 
enum RTC_PRIVACY_MODE
    {
        RTCPM_BLOCK_LIST_EXCLUDED	= 0,
        RTCPM_ALLOW_LIST_ONLY	= ( RTCPM_BLOCK_LIST_EXCLUDED + 1 ) 
    } 	RTC_PRIVACY_MODE;

typedef 
enum RTC_SESSION_TYPE
    {
        RTCST_PC_TO_PC	= 0,
        RTCST_PC_TO_PHONE	= ( RTCST_PC_TO_PC + 1 ) ,
        RTCST_PHONE_TO_PHONE	= ( RTCST_PC_TO_PHONE + 1 ) ,
        RTCST_IM	= ( RTCST_PHONE_TO_PHONE + 1 ) ,
        RTCST_MULTIPARTY_IM	= ( RTCST_IM + 1 ) ,
        RTCST_APPLICATION	= ( RTCST_MULTIPARTY_IM + 1 ) 
    } 	RTC_SESSION_TYPE;

typedef 
enum RTC_PRESENCE_STATUS
    {
        RTCXS_PRESENCE_OFFLINE	= 0,
        RTCXS_PRESENCE_ONLINE	= ( RTCXS_PRESENCE_OFFLINE + 1 ) ,
        RTCXS_PRESENCE_AWAY	= ( RTCXS_PRESENCE_ONLINE + 1 ) ,
        RTCXS_PRESENCE_IDLE	= ( RTCXS_PRESENCE_AWAY + 1 ) ,
        RTCXS_PRESENCE_BUSY	= ( RTCXS_PRESENCE_IDLE + 1 ) ,
        RTCXS_PRESENCE_BE_RIGHT_BACK	= ( RTCXS_PRESENCE_BUSY + 1 ) ,
        RTCXS_PRESENCE_ON_THE_PHONE	= ( RTCXS_PRESENCE_BE_RIGHT_BACK + 1 ) ,
        RTCXS_PRESENCE_OUT_TO_LUNCH	= ( RTCXS_PRESENCE_ON_THE_PHONE + 1 ) 
    } 	RTC_PRESENCE_STATUS;

typedef 
enum RTC_BUDDY_SUBSCRIPTION_TYPE
    {
        RTCBT_SUBSCRIBED	= 0,
        RTCBT_ALWAYS_OFFLINE	= ( RTCBT_SUBSCRIBED + 1 ) ,
        RTCBT_ALWAYS_ONLINE	= ( RTCBT_ALWAYS_OFFLINE + 1 ) ,
        RTCBT_POLL	= ( RTCBT_ALWAYS_ONLINE + 1 ) 
    } 	RTC_BUDDY_SUBSCRIPTION_TYPE;

typedef 
enum RTC_MEDIA_EVENT_TYPE
    {
        RTCMET_STOPPED	= 0,
        RTCMET_STARTED	= ( RTCMET_STOPPED + 1 ) ,
        RTCMET_FAILED	= ( RTCMET_STARTED + 1 ) 
    } 	RTC_MEDIA_EVENT_TYPE;

typedef 
enum RTC_MEDIA_EVENT_REASON
    {
        RTCMER_NORMAL	= 0,
        RTCMER_HOLD	= ( RTCMER_NORMAL + 1 ) ,
        RTCMER_TIMEOUT	= ( RTCMER_HOLD + 1 ) ,
        RTCMER_BAD_DEVICE	= ( RTCMER_TIMEOUT + 1 ) ,
        RTCMER_NO_PORT	= ( RTCMER_BAD_DEVICE + 1 ) ,
        RTCMER_PORT_MAPPING_FAILED	= ( RTCMER_NO_PORT + 1 ) ,
        RTCMER_REMOTE_REQUEST	= ( RTCMER_PORT_MAPPING_FAILED + 1 ) 
    } 	RTC_MEDIA_EVENT_REASON;

typedef 
enum RTC_MESSAGING_EVENT_TYPE
    {
        RTCMSET_MESSAGE	= 0,
        RTCMSET_STATUS	= ( RTCMSET_MESSAGE + 1 ) 
    } 	RTC_MESSAGING_EVENT_TYPE;

typedef 
enum RTC_MESSAGING_USER_STATUS
    {
        RTCMUS_IDLE	= 0,
        RTCMUS_TYPING	= ( RTCMUS_IDLE + 1 ) 
    } 	RTC_MESSAGING_USER_STATUS;

typedef 
enum RTC_DTMF
    {
        RTC_DTMF_0	= 0,
        RTC_DTMF_1	= ( RTC_DTMF_0 + 1 ) ,
        RTC_DTMF_2	= ( RTC_DTMF_1 + 1 ) ,
        RTC_DTMF_3	= ( RTC_DTMF_2 + 1 ) ,
        RTC_DTMF_4	= ( RTC_DTMF_3 + 1 ) ,
        RTC_DTMF_5	= ( RTC_DTMF_4 + 1 ) ,
        RTC_DTMF_6	= ( RTC_DTMF_5 + 1 ) ,
        RTC_DTMF_7	= ( RTC_DTMF_6 + 1 ) ,
        RTC_DTMF_8	= ( RTC_DTMF_7 + 1 ) ,
        RTC_DTMF_9	= ( RTC_DTMF_8 + 1 ) ,
        RTC_DTMF_STAR	= ( RTC_DTMF_9 + 1 ) ,
        RTC_DTMF_POUND	= ( RTC_DTMF_STAR + 1 ) ,
        RTC_DTMF_A	= ( RTC_DTMF_POUND + 1 ) ,
        RTC_DTMF_B	= ( RTC_DTMF_A + 1 ) ,
        RTC_DTMF_C	= ( RTC_DTMF_B + 1 ) ,
        RTC_DTMF_D	= ( RTC_DTMF_C + 1 ) ,
        RTC_DTMF_FLASH	= ( RTC_DTMF_D + 1 ) 
    } 	RTC_DTMF;

typedef 
enum RTC_PROVIDER_URI
    {
        RTCPU_URIHOMEPAGE	= 0,
        RTCPU_URIHELPDESK	= ( RTCPU_URIHOMEPAGE + 1 ) ,
        RTCPU_URIPERSONALACCOUNT	= ( RTCPU_URIHELPDESK + 1 ) ,
        RTCPU_URIDISPLAYDURINGCALL	= ( RTCPU_URIPERSONALACCOUNT + 1 ) ,
        RTCPU_URIDISPLAYDURINGIDLE	= ( RTCPU_URIDISPLAYDURINGCALL + 1 ) 
    } 	RTC_PROVIDER_URI;

typedef 
enum RTC_RING_TYPE
    {
        RTCRT_PHONE	= 0,
        RTCRT_MESSAGE	= ( RTCRT_PHONE + 1 ) ,
        RTCRT_RINGBACK	= ( RTCRT_MESSAGE + 1 ) 
    } 	RTC_RING_TYPE;

typedef 
enum RTC_T120_APPLET
    {
        RTCTA_WHITEBOARD	= 0,
        RTCTA_APPSHARING	= ( RTCTA_WHITEBOARD + 1 ) 
    } 	RTC_T120_APPLET;

typedef 
enum RTC_PORT_TYPE
    {
        RTCPT_AUDIO_RTP	= 0,
        RTCPT_AUDIO_RTCP	= ( RTCPT_AUDIO_RTP + 1 ) ,
        RTCPT_VIDEO_RTP	= ( RTCPT_AUDIO_RTCP + 1 ) ,
        RTCPT_VIDEO_RTCP	= ( RTCPT_VIDEO_RTP + 1 ) ,
        RTCPT_SIP	= ( RTCPT_VIDEO_RTCP + 1 ) 
    } 	RTC_PORT_TYPE;

typedef 
enum RTC_USER_SEARCH_COLUMN
    {
        RTCUSC_URI	= 0,
        RTCUSC_DISPLAYNAME	= ( RTCUSC_URI + 1 ) ,
        RTCUSC_TITLE	= ( RTCUSC_DISPLAYNAME + 1 ) ,
        RTCUSC_OFFICE	= ( RTCUSC_TITLE + 1 ) ,
        RTCUSC_PHONE	= ( RTCUSC_OFFICE + 1 ) ,
        RTCUSC_COMPANY	= ( RTCUSC_PHONE + 1 ) ,
        RTCUSC_CITY	= ( RTCUSC_COMPANY + 1 ) ,
        RTCUSC_STATE	= ( RTCUSC_CITY + 1 ) ,
        RTCUSC_COUNTRY	= ( RTCUSC_STATE + 1 ) ,
        RTCUSC_EMAIL	= ( RTCUSC_COUNTRY + 1 ) 
    } 	RTC_USER_SEARCH_COLUMN;

typedef 
enum RTC_USER_SEARCH_PREFERENCE
    {
        RTCUSP_MAX_MATCHES	= 0,
        RTCUSP_TIME_LIMIT	= ( RTCUSP_MAX_MATCHES + 1 ) 
    } 	RTC_USER_SEARCH_PREFERENCE;

typedef 
enum RTC_ROAMING_EVENT_TYPE
    {
        RTCRET_BUDDY_ROAMING	= 0,
        RTCRET_WATCHER_ROAMING	= ( RTCRET_BUDDY_ROAMING + 1 ) ,
        RTCRET_PRESENCE_ROAMING	= ( RTCRET_WATCHER_ROAMING + 1 ) ,
        RTCRET_PROFILE_ROAMING	= ( RTCRET_PRESENCE_ROAMING + 1 ) ,
        RTCRET_WPENDING_ROAMING	= ( RTCRET_PROFILE_ROAMING + 1 ) 
    } 	RTC_ROAMING_EVENT_TYPE;

typedef 
enum RTC_PROFILE_EVENT_TYPE
    {
        RTCPFET_PROFILE_GET	= 0,
        RTCPFET_PROFILE_UPDATE	= ( RTCPFET_PROFILE_GET + 1 ) 
    } 	RTC_PROFILE_EVENT_TYPE;

typedef 
enum RTC_ANSWER_MODE
    {
        RTCAM_OFFER_SESSION_EVENT	= 0,
        RTCAM_AUTOMATICALLY_ACCEPT	= ( RTCAM_OFFER_SESSION_EVENT + 1 ) ,
        RTCAM_AUTOMATICALLY_REJECT	= ( RTCAM_AUTOMATICALLY_ACCEPT + 1 ) ,
        RTCAM_NOT_SUPPORTED	= ( RTCAM_AUTOMATICALLY_REJECT + 1 ) 
    } 	RTC_ANSWER_MODE;

typedef 
enum RTC_SESSION_REFER_STATUS
    {
        RTCSRS_REFERRING	= 0,
        RTCSRS_ACCEPTED	= ( RTCSRS_REFERRING + 1 ) ,
        RTCSRS_ERROR	= ( RTCSRS_ACCEPTED + 1 ) ,
        RTCSRS_REJECTED	= ( RTCSRS_ERROR + 1 ) ,
        RTCSRS_DROPPED	= ( RTCSRS_REJECTED + 1 ) ,
        RTCSRS_DONE	= ( RTCSRS_DROPPED + 1 ) 
    } 	RTC_SESSION_REFER_STATUS;

typedef 
enum RTC_PRESENCE_PROPERTY
    {
        RTCPP_PHONENUMBER	= 0,
        RTCPP_DISPLAYNAME	= ( RTCPP_PHONENUMBER + 1 ) ,
        RTCPP_EMAIL	= ( RTCPP_DISPLAYNAME + 1 ) ,
        RTCPP_DEVICE_NAME	= ( RTCPP_EMAIL + 1 ) ,
        RTCPP_MULTIPLE	= ( RTCPP_DEVICE_NAME + 1 ) 
    } 	RTC_PRESENCE_PROPERTY;

typedef 
enum RTC_SECURITY_TYPE
    {
        RTCSECT_AUDIO_VIDEO_MEDIA_ENCRYPTION	= 0,
        RTCSECT_T120_MEDIA_ENCRYPTION	= ( RTCSECT_AUDIO_VIDEO_MEDIA_ENCRYPTION + 1 ) 
    } 	RTC_SECURITY_TYPE;

typedef 
enum RTC_SECURITY_LEVEL
    {
        RTCSECL_UNSUPPORTED	= 1,
        RTCSECL_SUPPORTED	= ( RTCSECL_UNSUPPORTED + 1 ) ,
        RTCSECL_REQUIRED	= ( RTCSECL_SUPPORTED + 1 ) 
    } 	RTC_SECURITY_LEVEL;

typedef 
enum RTC_REINVITE_STATE
    {
        RTCRIN_INCOMING	= 0,
        RTCRIN_SUCCEEDED	= ( RTCRIN_INCOMING + 1 ) ,
        RTCRIN_FAIL	= ( RTCRIN_SUCCEEDED + 1 ) 
    } 	RTC_REINVITE_STATE;

#define RTCCS_FORCE_PROFILE          0x00000001
#define RTCCS_FAIL_ON_REDIRECT       0x00000002
#define RTCMT_AUDIO_SEND     0x00000001
#define RTCMT_AUDIO_RECEIVE  0x00000002
#define RTCMT_VIDEO_SEND     0x00000004
#define RTCMT_VIDEO_RECEIVE  0x00000008
#define RTCMT_T120_SENDRECV  0x00000010
#define RTCMT_ALL_RTP   (       \
         RTCMT_AUDIO_SEND    |  \
         RTCMT_AUDIO_RECEIVE  | \
         RTCMT_VIDEO_SEND  |    \
         RTCMT_VIDEO_RECEIVE )    
#define RTCMT_ALL       (       \
         RTCMT_ALL_RTP    |     \
         RTCMT_T120_SENDRECV  )   
#define RTCSI_PC_TO_PC       0x00000001
#define RTCSI_PC_TO_PHONE    0x00000002
#define RTCSI_PHONE_TO_PHONE 0x00000004
#define RTCSI_IM             0x00000008
#define RTCSI_MULTIPARTY_IM  0x00000010
#define RTCSI_APPLICATION	0x00000020
#define RTCTR_UDP            0x00000001
#define RTCTR_TCP            0x00000002
#define RTCTR_TLS            0x00000004
#define RTCAU_BASIC                  0x00000001
#define RTCAU_DIGEST                 0x00000002
#define RTCAU_NTLM                   0x00000004
#define RTCAU_KERBEROS               0x00000008
#define RTCAU_USE_LOGON_CRED         0x00010000
#define RTCRF_REGISTER_INVITE_SESSIONS   0x00000001
#define RTCRF_REGISTER_MESSAGE_SESSIONS  0x00000002
#define RTCRF_REGISTER_PRESENCE          0x00000004
#define RTCRF_REGISTER_NOTIFY            0x00000008
#define RTCRF_REGISTER_ALL               0x0000000F
#define RTCRMF_BUDDY_ROAMING              0x00000001
#define RTCRMF_WATCHER_ROAMING            0x00000002
#define RTCRMF_PRESENCE_ROAMING           0x00000004
#define RTCRMF_PROFILE_ROAMING            0x00000008
#define RTCRMF_ALL_ROAMING                0x0000000F
#define RTCEF_CLIENT                     0x00000001
#define RTCEF_REGISTRATION_STATE_CHANGE  0x00000002
#define RTCEF_SESSION_STATE_CHANGE       0x00000004
#define RTCEF_SESSION_OPERATION_COMPLETE 0x00000008
#define RTCEF_PARTICIPANT_STATE_CHANGE   0x00000010
#define RTCEF_MEDIA                      0x00000020
#define RTCEF_INTENSITY                  0x00000040
#define RTCEF_MESSAGING                  0x00000080
#define RTCEF_BUDDY                      0x00000100
#define RTCEF_WATCHER                    0x00000200
#define RTCEF_PROFILE                    0x00000400
#define RTCEF_USERSEARCH                 0x00000800
#define RTCEF_INFO                       0x00001000
#define RTCEF_GROUP                      0x00002000
#define RTCEF_MEDIA_REQUEST              0x00004000
#define RTCEF_ROAMING                    0x00010000
#define RTCEF_PRESENCE_PROPERTY          0x00020000
#define RTCEF_BUDDY2                     0x00040000
#define RTCEF_WATCHER2                   0x00080000
#define RTCEF_SESSION_REFER_STATUS       0x00100000
#define RTCEF_SESSION_REFERRED           0x00200000
#define RTCEF_REINVITE                   0x00400000
#define RTCEF_PRESENCE_DATA              0x00800000
#define RTCEF_PRESENCE_STATUS            0x01000000
#define RTCEF_ALL                        0x01FFFFFF
#define RTCIF_DISABLE_MEDIA                          0x00000001
#define RTCIF_DISABLE_UPNP                           0x00000002
#define RTCIF_ENABLE_SERVER_CLASS                    0x00000004
#define RTCIF_DISABLE_STRICT_DNS                     0x00000008




































extern RPC_IF_HANDLE __MIDL_itf_rtccore_0000_0000_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_rtccore_0000_0000_v0_0_s_ifspec;

#ifndef __IRTCClient_INTERFACE_DEFINED__
#define __IRTCClient_INTERFACE_DEFINED__

/* interface IRTCClient */
/* [unique][helpstring][uuid][object] */ 


EXTERN_C const IID IID_IRTCClient;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("07829e45-9a34-408e-a011-bddf13487cd1")
    IRTCClient : public IUnknown
    {
    public:
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE Initialize( void) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE Shutdown( void) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE PrepareForShutdown( void) = 0;
        
        virtual /* [helpstring][propput] */ HRESULT STDMETHODCALLTYPE put_EventFilter( 
            /* [in] */ long lFilter) = 0;
        
        virtual /* [helpstring][propget] */ HRESULT STDMETHODCALLTYPE get_EventFilter( 
            /* [retval][out] */ __RPC__out long *plFilter) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE SetPreferredMediaTypes( 
            /* [in] */ long lMediaTypes,
            /* [in] */ VARIANT_BOOL fPersistent) = 0;
        
        virtual /* [helpstring][propget] */ HRESULT STDMETHODCALLTYPE get_PreferredMediaTypes( 
            /* [retval][out] */ __RPC__out long *plMediaTypes) = 0;
        
        virtual /* [helpstring][propget] */ HRESULT STDMETHODCALLTYPE get_MediaCapabilities( 
            /* [retval][out] */ __RPC__out long *plMediaTypes) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE CreateSession( 
            /* [in] */ RTC_SESSION_TYPE enType,
            /* [in] */ __RPC__in BSTR bstrLocalPhoneURI,
            /* [in] */ __RPC__in_opt IRTCProfile *pProfile,
            /* [in] */ long lFlags,
            /* [retval][out] */ __RPC__deref_out_opt IRTCSession **ppSession) = 0;
        
        virtual /* [helpstring][propput] */ HRESULT STDMETHODCALLTYPE put_ListenForIncomingSessions( 
            /* [in] */ RTC_LISTEN_MODE enListen) = 0;
        
        virtual /* [helpstring][propget] */ HRESULT STDMETHODCALLTYPE get_ListenForIncomingSessions( 
            /* [retval][out] */ __RPC__out RTC_LISTEN_MODE *penListen) = 0;
        
        virtual /* [helpstring][propget] */ HRESULT STDMETHODCALLTYPE get_NetworkAddresses( 
            /* [in] */ VARIANT_BOOL fTCP,
            /* [in] */ VARIANT_BOOL fExternal,
            /* [retval][out] */ __RPC__out VARIANT *pvAddresses) = 0;
        
        virtual /* [helpstring][propput] */ HRESULT STDMETHODCALLTYPE put_Volume( 
            /* [in] */ RTC_AUDIO_DEVICE enDevice,
            /* [in] */ long lVolume) = 0;
        
        virtual /* [helpstring][propget] */ HRESULT STDMETHODCALLTYPE get_Volume( 
            /* [in] */ RTC_AUDIO_DEVICE enDevice,
            /* [retval][out] */ __RPC__out long *plVolume) = 0;
        
        virtual /* [helpstring][propput] */ HRESULT STDMETHODCALLTYPE put_AudioMuted( 
            /* [in] */ RTC_AUDIO_DEVICE enDevice,
            /* [in] */ VARIANT_BOOL fMuted) = 0;
        
        virtual /* [helpstring][propget] */ HRESULT STDMETHODCALLTYPE get_AudioMuted( 
            /* [in] */ RTC_AUDIO_DEVICE enDevice,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pfMuted) = 0;
        
        virtual /* [helpstring][propget] */ HRESULT STDMETHODCALLTYPE get_IVideoWindow( 
            /* [in] */ RTC_VIDEO_DEVICE enDevice,
            /* [retval][out] */ __RPC__deref_out_opt IVideoWindow **ppIVideoWindow) = 0;
        
        virtual /* [helpstring][propput] */ HRESULT STDMETHODCALLTYPE put_PreferredAudioDevice( 
            /* [in] */ RTC_AUDIO_DEVICE enDevice,
            /* [in] */ __RPC__in BSTR bstrDeviceName) = 0;
        
        virtual /* [helpstring][propget] */ HRESULT STDMETHODCALLTYPE get_PreferredAudioDevice( 
            /* [in] */ RTC_AUDIO_DEVICE enDevice,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrDeviceName) = 0;
        
        virtual /* [helpstring][propput] */ HRESULT STDMETHODCALLTYPE put_PreferredVolume( 
            /* [in] */ RTC_AUDIO_DEVICE enDevice,
            /* [in] */ long lVolume) = 0;
        
        virtual /* [helpstring][propget] */ HRESULT STDMETHODCALLTYPE get_PreferredVolume( 
            /* [in] */ RTC_AUDIO_DEVICE enDevice,
            /* [retval][out] */ __RPC__out long *plVolume) = 0;
        
        virtual /* [helpstring][propput] */ HRESULT STDMETHODCALLTYPE put_PreferredAEC( 
            /* [in] */ VARIANT_BOOL bEnable) = 0;
        
        virtual /* [helpstring][propget] */ HRESULT STDMETHODCALLTYPE get_PreferredAEC( 
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pbEnabled) = 0;
        
        virtual /* [helpstring][propput] */ HRESULT STDMETHODCALLTYPE put_PreferredVideoDevice( 
            /* [in] */ __RPC__in BSTR bstrDeviceName) = 0;
        
        virtual /* [helpstring][propget] */ HRESULT STDMETHODCALLTYPE get_PreferredVideoDevice( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrDeviceName) = 0;
        
        virtual /* [helpstring][propget] */ HRESULT STDMETHODCALLTYPE get_ActiveMedia( 
            /* [retval][out] */ __RPC__out long *plMediaType) = 0;
        
        virtual /* [helpstring][propput] */ HRESULT STDMETHODCALLTYPE put_MaxBitrate( 
            /* [in] */ long lMaxBitrate) = 0;
        
        virtual /* [helpstring][propget] */ HRESULT STDMETHODCALLTYPE get_MaxBitrate( 
            /* [retval][out] */ __RPC__out long *plMaxBitrate) = 0;
        
        virtual /* [helpstring][propput] */ HRESULT STDMETHODCALLTYPE put_TemporalSpatialTradeOff( 
            /* [in] */ long lValue) = 0;
        
        virtual /* [helpstring][propget] */ HRESULT STDMETHODCALLTYPE get_TemporalSpatialTradeOff( 
            /* [retval][out] */ __RPC__out long *plValue) = 0;
        
        virtual /* [helpstring][propget] */ HRESULT STDMETHODCALLTYPE get_NetworkQuality( 
            /* [retval][out] */ __RPC__out long *plNetworkQuality) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE StartT120Applet( 
            /* [in] */ RTC_T120_APPLET enApplet) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE StopT120Applets( void) = 0;
        
        virtual /* [helpstring][propget] */ HRESULT STDMETHODCALLTYPE get_IsT120AppletRunning( 
            /* [in] */ RTC_T120_APPLET enApplet,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pfRunning) = 0;
        
        virtual /* [helpstring][propget] */ HRESULT STDMETHODCALLTYPE get_LocalUserURI( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrUserURI) = 0;
        
        virtual /* [helpstring][propput] */ HRESULT STDMETHODCALLTYPE put_LocalUserURI( 
            /* [in] */ __RPC__in BSTR bstrUserURI) = 0;
        
        virtual /* [helpstring][propget] */ HRESULT STDMETHODCALLTYPE get_LocalUserName( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrUserName) = 0;
        
        virtual /* [helpstring][propput] */ HRESULT STDMETHODCALLTYPE put_LocalUserName( 
            /* [in] */ __RPC__in BSTR bstrUserName) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE PlayRing( 
            /* [in] */ RTC_RING_TYPE enType,
            /* [in] */ VARIANT_BOOL bPlay) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE SendDTMF( 
            /* [in] */ RTC_DTMF enDTMF) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE InvokeTuningWizard( 
            /* [in] */ OAHWND hwndParent) = 0;
        
        virtual /* [helpstring][propget] */ HRESULT STDMETHODCALLTYPE get_IsTuned( 
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pfTuned) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IRTCClientVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IRTCClient * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IRTCClient * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IRTCClient * This);
        
        DECLSPEC_XFGVIRT(IRTCClient, Initialize)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *Initialize )( 
            __RPC__in IRTCClient * This);
        
        DECLSPEC_XFGVIRT(IRTCClient, Shutdown)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *Shutdown )( 
            __RPC__in IRTCClient * This);
        
        DECLSPEC_XFGVIRT(IRTCClient, PrepareForShutdown)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *PrepareForShutdown )( 
            __RPC__in IRTCClient * This);
        
        DECLSPEC_XFGVIRT(IRTCClient, put_EventFilter)
        /* [helpstring][propput] */ HRESULT ( STDMETHODCALLTYPE *put_EventFilter )( 
            __RPC__in IRTCClient * This,
            /* [in] */ long lFilter);
        
        DECLSPEC_XFGVIRT(IRTCClient, get_EventFilter)
        /* [helpstring][propget] */ HRESULT ( STDMETHODCALLTYPE *get_EventFilter )( 
            __RPC__in IRTCClient * This,
            /* [retval][out] */ __RPC__out long *plFilter);
        
        DECLSPEC_XFGVIRT(IRTCClient, SetPreferredMediaTypes)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *SetPreferredMediaTypes )( 
            __RPC__in IRTCClient * This,
            /* [in] */ long lMediaTypes,
            /* [in] */ VARIANT_BOOL fPersistent);
        
        DECLSPEC_XFGVIRT(IRTCClient, get_PreferredMediaTypes)
        /* [helpstring][propget] */ HRESULT ( STDMETHODCALLTYPE *get_PreferredMediaTypes )( 
            __RPC__in IRTCClient * This,
            /* [retval][out] */ __RPC__out long *plMediaTypes);
        
        DECLSPEC_XFGVIRT(IRTCClient, get_MediaCapabilities)
        /* [helpstring][propget] */ HRESULT ( STDMETHODCALLTYPE *get_MediaCapabilities )( 
            __RPC__in IRTCClient * This,
            /* [retval][out] */ __RPC__out long *plMediaTypes);
        
        DECLSPEC_XFGVIRT(IRTCClient, CreateSession)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *CreateSession )( 
            __RPC__in IRTCClient * This,
            /* [in] */ RTC_SESSION_TYPE enType,
            /* [in] */ __RPC__in BSTR bstrLocalPhoneURI,
            /* [in] */ __RPC__in_opt IRTCProfile *pProfile,
            /* [in] */ long lFlags,
            /* [retval][out] */ __RPC__deref_out_opt IRTCSession **ppSession);
        
        DECLSPEC_XFGVIRT(IRTCClient, put_ListenForIncomingSessions)
        /* [helpstring][propput] */ HRESULT ( STDMETHODCALLTYPE *put_ListenForIncomingSessions )( 
            __RPC__in IRTCClient * This,
            /* [in] */ RTC_LISTEN_MODE enListen);
        
        DECLSPEC_XFGVIRT(IRTCClient, get_ListenForIncomingSessions)
        /* [helpstring][propget] */ HRESULT ( STDMETHODCALLTYPE *get_ListenForIncomingSessions )( 
            __RPC__in IRTCClient * This,
            /* [retval][out] */ __RPC__out RTC_LISTEN_MODE *penListen);
        
        DECLSPEC_XFGVIRT(IRTCClient, get_NetworkAddresses)
        /* [helpstring][propget] */ HRESULT ( STDMETHODCALLTYPE *get_NetworkAddresses )( 
            __RPC__in IRTCClient * This,
            /* [in] */ VARIANT_BOOL fTCP,
            /* [in] */ VARIANT_BOOL fExternal,
            /* [retval][out] */ __RPC__out VARIANT *pvAddresses);
        
        DECLSPEC_XFGVIRT(IRTCClient, put_Volume)
        /* [helpstring][propput] */ HRESULT ( STDMETHODCALLTYPE *put_Volume )( 
            __RPC__in IRTCClient * This,
            /* [in] */ RTC_AUDIO_DEVICE enDevice,
            /* [in] */ long lVolume);
        
        DECLSPEC_XFGVIRT(IRTCClient, get_Volume)
        /* [helpstring][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Volume )( 
            __RPC__in IRTCClient * This,
            /* [in] */ RTC_AUDIO_DEVICE enDevice,
            /* [retval][out] */ __RPC__out long *plVolume);
        
        DECLSPEC_XFGVIRT(IRTCClient, put_AudioMuted)
        /* [helpstring][propput] */ HRESULT ( STDMETHODCALLTYPE *put_AudioMuted )( 
            __RPC__in IRTCClient * This,
            /* [in] */ RTC_AUDIO_DEVICE enDevice,
            /* [in] */ VARIANT_BOOL fMuted);
        
        DECLSPEC_XFGVIRT(IRTCClient, get_AudioMuted)
        /* [helpstring][propget] */ HRESULT ( STDMETHODCALLTYPE *get_AudioMuted )( 
            __RPC__in IRTCClient * This,
            /* [in] */ RTC_AUDIO_DEVICE enDevice,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pfMuted);
        
        DECLSPEC_XFGVIRT(IRTCClient, get_IVideoWindow)
        /* [helpstring][propget] */ HRESULT ( STDMETHODCALLTYPE *get_IVideoWindow )( 
            __RPC__in IRTCClient * This,
            /* [in] */ RTC_VIDEO_DEVICE enDevice,
            /* [retval][out] */ __RPC__deref_out_opt IVideoWindow **ppIVideoWindow);
        
        DECLSPEC_XFGVIRT(IRTCClient, put_PreferredAudioDevice)
        /* [helpstring][propput] */ HRESULT ( STDMETHODCALLTYPE *put_PreferredAudioDevice )( 
            __RPC__in IRTCClient * This,
            /* [in] */ RTC_AUDIO_DEVICE enDevice,
            /* [in] */ __RPC__in BSTR bstrDeviceName);
        
        DECLSPEC_XFGVIRT(IRTCClient, get_PreferredAudioDevice)
        /* [helpstring][propget] */ HRESULT ( STDMETHODCALLTYPE *get_PreferredAudioDevice )( 
            __RPC__in IRTCClient * This,
            /* [in] */ RTC_AUDIO_DEVICE enDevice,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrDeviceName);
        
        DECLSPEC_XFGVIRT(IRTCClient, put_PreferredVolume)
        /* [helpstring][propput] */ HRESULT ( STDMETHODCALLTYPE *put_PreferredVolume )( 
            __RPC__in IRTCClient * This,
            /* [in] */ RTC_AUDIO_DEVICE enDevice,
            /* [in] */ long lVolume);
        
        DECLSPEC_XFGVIRT(IRTCClient, get_PreferredVolume)
        /* [helpstring][propget] */ HRESULT ( STDMETHODCALLTYPE *get_PreferredVolume )( 
            __RPC__in IRTCClient * This,
            /* [in] */ RTC_AUDIO_DEVICE enDevice,
            /* [retval][out] */ __RPC__out long *plVolume);
        
        DECLSPEC_XFGVIRT(IRTCClient, put_PreferredAEC)
        /* [helpstring][propput] */ HRESULT ( STDMETHODCALLTYPE *put_PreferredAEC )( 
            __RPC__in IRTCClient * This,
            /* [in] */ VARIANT_BOOL bEnable);
        
        DECLSPEC_XFGVIRT(IRTCClient, get_PreferredAEC)
        /* [helpstring][propget] */ HRESULT ( STDMETHODCALLTYPE *get_PreferredAEC )( 
            __RPC__in IRTCClient * This,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pbEnabled);
        
        DECLSPEC_XFGVIRT(IRTCClient, put_PreferredVideoDevice)
        /* [helpstring][propput] */ HRESULT ( STDMETHODCALLTYPE *put_PreferredVideoDevice )( 
            __RPC__in IRTCClient * This,
            /* [in] */ __RPC__in BSTR bstrDeviceName);
        
        DECLSPEC_XFGVIRT(IRTCClient, get_PreferredVideoDevice)
        /* [helpstring][propget] */ HRESULT ( STDMETHODCALLTYPE *get_PreferredVideoDevice )( 
            __RPC__in IRTCClient * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrDeviceName);
        
        DECLSPEC_XFGVIRT(IRTCClient, get_ActiveMedia)
        /* [helpstring][propget] */ HRESULT ( STDMETHODCALLTYPE *get_ActiveMedia )( 
            __RPC__in IRTCClient * This,
            /* [retval][out] */ __RPC__out long *plMediaType);
        
        DECLSPEC_XFGVIRT(IRTCClient, put_MaxBitrate)
        /* [helpstring][propput] */ HRESULT ( STDMETHODCALLTYPE *put_MaxBitrate )( 
            __RPC__in IRTCClient * This,
            /* [in] */ long lMaxBitrate);
        
        DECLSPEC_XFGVIRT(IRTCClient, get_MaxBitrate)
        /* [helpstring][propget] */ HRESULT ( STDMETHODCALLTYPE *get_MaxBitrate )( 
            __RPC__in IRTCClient * This,
            /* [retval][out] */ __RPC__out long *plMaxBitrate);
        
        DECLSPEC_XFGVIRT(IRTCClient, put_TemporalSpatialTradeOff)
        /* [helpstring][propput] */ HRESULT ( STDMETHODCALLTYPE *put_TemporalSpatialTradeOff )( 
            __RPC__in IRTCClient * This,
            /* [in] */ long lValue);
        
        DECLSPEC_XFGVIRT(IRTCClient, get_TemporalSpatialTradeOff)
        /* [helpstring][propget] */ HRESULT ( STDMETHODCALLTYPE *get_TemporalSpatialTradeOff )( 
            __RPC__in IRTCClient * This,
            /* [retval][out] */ __RPC__out long *plValue);
        
        DECLSPEC_XFGVIRT(IRTCClient, get_NetworkQuality)
        /* [helpstring][propget] */ HRESULT ( STDMETHODCALLTYPE *get_NetworkQuality )( 
            __RPC__in IRTCClient * This,
            /* [retval][out] */ __RPC__out long *plNetworkQuality);
        
        DECLSPEC_XFGVIRT(IRTCClient, StartT120Applet)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *StartT120Applet )( 
            __RPC__in IRTCClient * This,
            /* [in] */ RTC_T120_APPLET enApplet);
        
        DECLSPEC_XFGVIRT(IRTCClient, StopT120Applets)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *StopT120Applets )( 
            __RPC__in IRTCClient * This);
        
        DECLSPEC_XFGVIRT(IRTCClient, get_IsT120AppletRunning)
        /* [helpstring][propget] */ HRESULT ( STDMETHODCALLTYPE *get_IsT120AppletRunning )( 
            __RPC__in IRTCClient * This,
            /* [in] */ RTC_T120_APPLET enApplet,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pfRunning);
        
        DECLSPEC_XFGVIRT(IRTCClient, get_LocalUserURI)
        /* [helpstring][propget] */ HRESULT ( STDMETHODCALLTYPE *get_LocalUserURI )( 
            __RPC__in IRTCClient * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrUserURI);
        
        DECLSPEC_XFGVIRT(IRTCClient, put_LocalUserURI)
        /* [helpstring][propput] */ HRESULT ( STDMETHODCALLTYPE *put_LocalUserURI )( 
            __RPC__in IRTCClient * This,
            /* [in] */ __RPC__in BSTR bstrUserURI);
        
        DECLSPEC_XFGVIRT(IRTCClient, get_LocalUserName)
        /* [helpstring][propget] */ HRESULT ( STDMETHODCALLTYPE *get_LocalUserName )( 
            __RPC__in IRTCClient * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrUserName);
        
        DECLSPEC_XFGVIRT(IRTCClient, put_LocalUserName)
        /* [helpstring][propput] */ HRESULT ( STDMETHODCALLTYPE *put_LocalUserName )( 
            __RPC__in IRTCClient * This,
            /* [in] */ __RPC__in BSTR bstrUserName);
        
        DECLSPEC_XFGVIRT(IRTCClient, PlayRing)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *PlayRing )( 
            __RPC__in IRTCClient * This,
            /* [in] */ RTC_RING_TYPE enType,
            /* [in] */ VARIANT_BOOL bPlay);
        
        DECLSPEC_XFGVIRT(IRTCClient, SendDTMF)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *SendDTMF )( 
            __RPC__in IRTCClient * This,
            /* [in] */ RTC_DTMF enDTMF);
        
        DECLSPEC_XFGVIRT(IRTCClient, InvokeTuningWizard)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *InvokeTuningWizard )( 
            __RPC__in IRTCClient * This,
            /* [in] */ OAHWND hwndParent);
        
        DECLSPEC_XFGVIRT(IRTCClient, get_IsTuned)
        /* [helpstring][propget] */ HRESULT ( STDMETHODCALLTYPE *get_IsTuned )( 
            __RPC__in IRTCClient * This,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pfTuned);
        
        END_INTERFACE
    } IRTCClientVtbl;

    interface IRTCClient
    {
        CONST_VTBL struct IRTCClientVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IRTCClient_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IRTCClient_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IRTCClient_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IRTCClient_Initialize(This)	\
    ( (This)->lpVtbl -> Initialize(This) ) 

#define IRTCClient_Shutdown(This)	\
    ( (This)->lpVtbl -> Shutdown(This) ) 

#define IRTCClient_PrepareForShutdown(This)	\
    ( (This)->lpVtbl -> PrepareForShutdown(This) ) 

#define IRTCClient_put_EventFilter(This,lFilter)	\
    ( (This)->lpVtbl -> put_EventFilter(This,lFilter) ) 

#define IRTCClient_get_EventFilter(This,plFilter)	\
    ( (This)->lpVtbl -> get_EventFilter(This,plFilter) ) 

#define IRTCClient_SetPreferredMediaTypes(This,lMediaTypes,fPersistent)	\
    ( (This)->lpVtbl -> SetPreferredMediaTypes(This,lMediaTypes,fPersistent) ) 

#define IRTCClient_get_PreferredMediaTypes(This,plMediaTypes)	\
    ( (This)->lpVtbl -> get_PreferredMediaTypes(This,plMediaTypes) ) 

#define IRTCClient_get_MediaCapabilities(This,plMediaTypes)	\
    ( (This)->lpVtbl -> get_MediaCapabilities(This,plMediaTypes) ) 

#define IRTCClient_CreateSession(This,enType,bstrLocalPhoneURI,pProfile,lFlags,ppSession)	\
    ( (This)->lpVtbl -> CreateSession(This,enType,bstrLocalPhoneURI,pProfile,lFlags,ppSession) ) 

#define IRTCClient_put_ListenForIncomingSessions(This,enListen)	\
    ( (This)->lpVtbl -> put_ListenForIncomingSessions(This,enListen) ) 

#define IRTCClient_get_ListenForIncomingSessions(This,penListen)	\
    ( (This)->lpVtbl -> get_ListenForIncomingSessions(This,penListen) ) 

#define IRTCClient_get_NetworkAddresses(This,fTCP,fExternal,pvAddresses)	\
    ( (This)->lpVtbl -> get_NetworkAddresses(This,fTCP,fExternal,pvAddresses) ) 

#define IRTCClient_put_Volume(This,enDevice,lVolume)	\
    ( (This)->lpVtbl -> put_Volume(This,enDevice,lVolume) ) 

#define IRTCClient_get_Volume(This,enDevice,plVolume)	\
    ( (This)->lpVtbl -> get_Volume(This,enDevice,plVolume) ) 

#define IRTCClient_put_AudioMuted(This,enDevice,fMuted)	\
    ( (This)->lpVtbl -> put_AudioMuted(This,enDevice,fMuted) ) 

#define IRTCClient_get_AudioMuted(This,enDevice,pfMuted)	\
    ( (This)->lpVtbl -> get_AudioMuted(This,enDevice,pfMuted) ) 

#define IRTCClient_get_IVideoWindow(This,enDevice,ppIVideoWindow)	\
    ( (This)->lpVtbl -> get_IVideoWindow(This,enDevice,ppIVideoWindow) ) 

#define IRTCClient_put_PreferredAudioDevice(This,enDevice,bstrDeviceName)	\
    ( (This)->lpVtbl -> put_PreferredAudioDevice(This,enDevice,bstrDeviceName) ) 

#define IRTCClient_get_PreferredAudioDevice(This,enDevice,pbstrDeviceName)	\
    ( (This)->lpVtbl -> get_PreferredAudioDevice(This,enDevice,pbstrDeviceName) ) 

#define IRTCClient_put_PreferredVolume(This,enDevice,lVolume)	\
    ( (This)->lpVtbl -> put_PreferredVolume(This,enDevice,lVolume) ) 

#define IRTCClient_get_PreferredVolume(This,enDevice,plVolume)	\
    ( (This)->lpVtbl -> get_PreferredVolume(This,enDevice,plVolume) ) 

#define IRTCClient_put_PreferredAEC(This,bEnable)	\
    ( (This)->lpVtbl -> put_PreferredAEC(This,bEnable) ) 

#define IRTCClient_get_PreferredAEC(This,pbEnabled)	\
    ( (This)->lpVtbl -> get_PreferredAEC(This,pbEnabled) ) 

#define IRTCClient_put_PreferredVideoDevice(This,bstrDeviceName)	\
    ( (This)->lpVtbl -> put_PreferredVideoDevice(This,bstrDeviceName) ) 

#define IRTCClient_get_PreferredVideoDevice(This,pbstrDeviceName)	\
    ( (This)->lpVtbl -> get_PreferredVideoDevice(This,pbstrDeviceName) ) 

#define IRTCClient_get_ActiveMedia(This,plMediaType)	\
    ( (This)->lpVtbl -> get_ActiveMedia(This,plMediaType) ) 

#define IRTCClient_put_MaxBitrate(This,lMaxBitrate)	\
    ( (This)->lpVtbl -> put_MaxBitrate(This,lMaxBitrate) ) 

#define IRTCClient_get_MaxBitrate(This,plMaxBitrate)	\
    ( (This)->lpVtbl -> get_MaxBitrate(This,plMaxBitrate) ) 

#define IRTCClient_put_TemporalSpatialTradeOff(This,lValue)	\
    ( (This)->lpVtbl -> put_TemporalSpatialTradeOff(This,lValue) ) 

#define IRTCClient_get_TemporalSpatialTradeOff(This,plValue)	\
    ( (This)->lpVtbl -> get_TemporalSpatialTradeOff(This,plValue) ) 

#define IRTCClient_get_NetworkQuality(This,plNetworkQuality)	\
    ( (This)->lpVtbl -> get_NetworkQuality(This,plNetworkQuality) ) 

#define IRTCClient_StartT120Applet(This,enApplet)	\
    ( (This)->lpVtbl -> StartT120Applet(This,enApplet) ) 

#define IRTCClient_StopT120Applets(This)	\
    ( (This)->lpVtbl -> StopT120Applets(This) ) 

#define IRTCClient_get_IsT120AppletRunning(This,enApplet,pfRunning)	\
    ( (This)->lpVtbl -> get_IsT120AppletRunning(This,enApplet,pfRunning) ) 

#define IRTCClient_get_LocalUserURI(This,pbstrUserURI)	\
    ( (This)->lpVtbl -> get_LocalUserURI(This,pbstrUserURI) ) 

#define IRTCClient_put_LocalUserURI(This,bstrUserURI)	\
    ( (This)->lpVtbl -> put_LocalUserURI(This,bstrUserURI) ) 

#define IRTCClient_get_LocalUserName(This,pbstrUserName)	\
    ( (This)->lpVtbl -> get_LocalUserName(This,pbstrUserName) ) 

#define IRTCClient_put_LocalUserName(This,bstrUserName)	\
    ( (This)->lpVtbl -> put_LocalUserName(This,bstrUserName) ) 

#define IRTCClient_PlayRing(This,enType,bPlay)	\
    ( (This)->lpVtbl -> PlayRing(This,enType,bPlay) ) 

#define IRTCClient_SendDTMF(This,enDTMF)	\
    ( (This)->lpVtbl -> SendDTMF(This,enDTMF) ) 

#define IRTCClient_InvokeTuningWizard(This,hwndParent)	\
    ( (This)->lpVtbl -> InvokeTuningWizard(This,hwndParent) ) 

#define IRTCClient_get_IsTuned(This,pfTuned)	\
    ( (This)->lpVtbl -> get_IsTuned(This,pfTuned) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IRTCClient_INTERFACE_DEFINED__ */


#ifndef __IRTCClient2_INTERFACE_DEFINED__
#define __IRTCClient2_INTERFACE_DEFINED__

/* interface IRTCClient2 */
/* [unique][helpstring][uuid][object] */ 


EXTERN_C const IID IID_IRTCClient2;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("0C91D71D-1064-42da-BFA5-572BEB8EEA84")
    IRTCClient2 : public IRTCClient
    {
    public:
        virtual /* [helpstring][propput] */ HRESULT STDMETHODCALLTYPE put_AnswerMode( 
            /* [in] */ RTC_SESSION_TYPE enType,
            /* [in] */ RTC_ANSWER_MODE enMode) = 0;
        
        virtual /* [helpstring][propget] */ HRESULT STDMETHODCALLTYPE get_AnswerMode( 
            /* [in] */ RTC_SESSION_TYPE enType,
            /* [retval][out] */ __RPC__out RTC_ANSWER_MODE *penMode) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE InvokeTuningWizardEx( 
            /* [in] */ OAHWND hwndParent,
            /* [in] */ VARIANT_BOOL fAllowAudio,
            /* [in] */ VARIANT_BOOL fAllowVideo) = 0;
        
        virtual /* [helpstring][propget] */ HRESULT STDMETHODCALLTYPE get_Version( 
            /* [retval][out] */ __RPC__out long *plVersion) = 0;
        
        virtual /* [helpstring][propput] */ HRESULT STDMETHODCALLTYPE put_ClientName( 
            /* [in] */ __RPC__in BSTR bstrClientName) = 0;
        
        virtual /* [helpstring][propput] */ HRESULT STDMETHODCALLTYPE put_ClientCurVer( 
            /* [in] */ __RPC__in BSTR bstrClientCurVer) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE InitializeEx( 
            /* [in] */ long lFlags) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE CreateSessionWithDescription( 
            /* [in] */ __RPC__in BSTR bstrContentType,
            /* [in] */ __RPC__in BSTR bstrSessionDescription,
            /* [in] */ __RPC__in_opt IRTCProfile *pProfile,
            /* [in] */ long lFlags,
            /* [retval][out] */ __RPC__deref_out_opt IRTCSession2 **ppSession2) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE SetSessionDescriptionManager( 
            /* [in] */ __RPC__in_opt IRTCSessionDescriptionManager *pSessionDescriptionManager) = 0;
        
        virtual /* [helpstring][propput] */ HRESULT STDMETHODCALLTYPE put_PreferredSecurityLevel( 
            /* [in] */ RTC_SECURITY_TYPE enSecurityType,
            /* [in] */ RTC_SECURITY_LEVEL enSecurityLevel) = 0;
        
        virtual /* [helpstring][propget] */ HRESULT STDMETHODCALLTYPE get_PreferredSecurityLevel( 
            /* [in] */ RTC_SECURITY_TYPE enSecurityType,
            /* [retval][out] */ __RPC__out RTC_SECURITY_LEVEL *penSecurityLevel) = 0;
        
        virtual /* [helpstring][propput] */ HRESULT STDMETHODCALLTYPE put_AllowedPorts( 
            /* [in] */ long lTransport,
            /* [in] */ RTC_LISTEN_MODE enListenMode) = 0;
        
        virtual /* [helpstring][propget] */ HRESULT STDMETHODCALLTYPE get_AllowedPorts( 
            /* [in] */ long lTransport,
            /* [retval][out] */ __RPC__out RTC_LISTEN_MODE *penListenMode) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IRTCClient2Vtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IRTCClient2 * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IRTCClient2 * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IRTCClient2 * This);
        
        DECLSPEC_XFGVIRT(IRTCClient, Initialize)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *Initialize )( 
            __RPC__in IRTCClient2 * This);
        
        DECLSPEC_XFGVIRT(IRTCClient, Shutdown)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *Shutdown )( 
            __RPC__in IRTCClient2 * This);
        
        DECLSPEC_XFGVIRT(IRTCClient, PrepareForShutdown)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *PrepareForShutdown )( 
            __RPC__in IRTCClient2 * This);
        
        DECLSPEC_XFGVIRT(IRTCClient, put_EventFilter)
        /* [helpstring][propput] */ HRESULT ( STDMETHODCALLTYPE *put_EventFilter )( 
            __RPC__in IRTCClient2 * This,
            /* [in] */ long lFilter);
        
        DECLSPEC_XFGVIRT(IRTCClient, get_EventFilter)
        /* [helpstring][propget] */ HRESULT ( STDMETHODCALLTYPE *get_EventFilter )( 
            __RPC__in IRTCClient2 * This,
            /* [retval][out] */ __RPC__out long *plFilter);
        
        DECLSPEC_XFGVIRT(IRTCClient, SetPreferredMediaTypes)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *SetPreferredMediaTypes )( 
            __RPC__in IRTCClient2 * This,
            /* [in] */ long lMediaTypes,
            /* [in] */ VARIANT_BOOL fPersistent);
        
        DECLSPEC_XFGVIRT(IRTCClient, get_PreferredMediaTypes)
        /* [helpstring][propget] */ HRESULT ( STDMETHODCALLTYPE *get_PreferredMediaTypes )( 
            __RPC__in IRTCClient2 * This,
            /* [retval][out] */ __RPC__out long *plMediaTypes);
        
        DECLSPEC_XFGVIRT(IRTCClient, get_MediaCapabilities)
        /* [helpstring][propget] */ HRESULT ( STDMETHODCALLTYPE *get_MediaCapabilities )( 
            __RPC__in IRTCClient2 * This,
            /* [retval][out] */ __RPC__out long *plMediaTypes);
        
        DECLSPEC_XFGVIRT(IRTCClient, CreateSession)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *CreateSession )( 
            __RPC__in IRTCClient2 * This,
            /* [in] */ RTC_SESSION_TYPE enType,
            /* [in] */ __RPC__in BSTR bstrLocalPhoneURI,
            /* [in] */ __RPC__in_opt IRTCProfile *pProfile,
            /* [in] */ long lFlags,
            /* [retval][out] */ __RPC__deref_out_opt IRTCSession **ppSession);
        
        DECLSPEC_XFGVIRT(IRTCClient, put_ListenForIncomingSessions)
        /* [helpstring][propput] */ HRESULT ( STDMETHODCALLTYPE *put_ListenForIncomingSessions )( 
            __RPC__in IRTCClient2 * This,
            /* [in] */ RTC_LISTEN_MODE enListen);
        
        DECLSPEC_XFGVIRT(IRTCClient, get_ListenForIncomingSessions)
        /* [helpstring][propget] */ HRESULT ( STDMETHODCALLTYPE *get_ListenForIncomingSessions )( 
            __RPC__in IRTCClient2 * This,
            /* [retval][out] */ __RPC__out RTC_LISTEN_MODE *penListen);
        
        DECLSPEC_XFGVIRT(IRTCClient, get_NetworkAddresses)
        /* [helpstring][propget] */ HRESULT ( STDMETHODCALLTYPE *get_NetworkAddresses )( 
            __RPC__in IRTCClient2 * This,
            /* [in] */ VARIANT_BOOL fTCP,
            /* [in] */ VARIANT_BOOL fExternal,
            /* [retval][out] */ __RPC__out VARIANT *pvAddresses);
        
        DECLSPEC_XFGVIRT(IRTCClient, put_Volume)
        /* [helpstring][propput] */ HRESULT ( STDMETHODCALLTYPE *put_Volume )( 
            __RPC__in IRTCClient2 * This,
            /* [in] */ RTC_AUDIO_DEVICE enDevice,
            /* [in] */ long lVolume);
        
        DECLSPEC_XFGVIRT(IRTCClient, get_Volume)
        /* [helpstring][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Volume )( 
            __RPC__in IRTCClient2 * This,
            /* [in] */ RTC_AUDIO_DEVICE enDevice,
            /* [retval][out] */ __RPC__out long *plVolume);
        
        DECLSPEC_XFGVIRT(IRTCClient, put_AudioMuted)
        /* [helpstring][propput] */ HRESULT ( STDMETHODCALLTYPE *put_AudioMuted )( 
            __RPC__in IRTCClient2 * This,
            /* [in] */ RTC_AUDIO_DEVICE enDevice,
            /* [in] */ VARIANT_BOOL fMuted);
        
        DECLSPEC_XFGVIRT(IRTCClient, get_AudioMuted)
        /* [helpstring][propget] */ HRESULT ( STDMETHODCALLTYPE *get_AudioMuted )( 
            __RPC__in IRTCClient2 * This,
            /* [in] */ RTC_AUDIO_DEVICE enDevice,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pfMuted);
        
        DECLSPEC_XFGVIRT(IRTCClient, get_IVideoWindow)
        /* [helpstring][propget] */ HRESULT ( STDMETHODCALLTYPE *get_IVideoWindow )( 
            __RPC__in IRTCClient2 * This,
            /* [in] */ RTC_VIDEO_DEVICE enDevice,
            /* [retval][out] */ __RPC__deref_out_opt IVideoWindow **ppIVideoWindow);
        
        DECLSPEC_XFGVIRT(IRTCClient, put_PreferredAudioDevice)
        /* [helpstring][propput] */ HRESULT ( STDMETHODCALLTYPE *put_PreferredAudioDevice )( 
            __RPC__in IRTCClient2 * This,
            /* [in] */ RTC_AUDIO_DEVICE enDevice,
            /* [in] */ __RPC__in BSTR bstrDeviceName);
        
        DECLSPEC_XFGVIRT(IRTCClient, get_PreferredAudioDevice)
        /* [helpstring][propget] */ HRESULT ( STDMETHODCALLTYPE *get_PreferredAudioDevice )( 
            __RPC__in IRTCClient2 * This,
            /* [in] */ RTC_AUDIO_DEVICE enDevice,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrDeviceName);
        
        DECLSPEC_XFGVIRT(IRTCClient, put_PreferredVolume)
        /* [helpstring][propput] */ HRESULT ( STDMETHODCALLTYPE *put_PreferredVolume )( 
            __RPC__in IRTCClient2 * This,
            /* [in] */ RTC_AUDIO_DEVICE enDevice,
            /* [in] */ long lVolume);
        
        DECLSPEC_XFGVIRT(IRTCClient, get_PreferredVolume)
        /* [helpstring][propget] */ HRESULT ( STDMETHODCALLTYPE *get_PreferredVolume )( 
            __RPC__in IRTCClient2 * This,
            /* [in] */ RTC_AUDIO_DEVICE enDevice,
            /* [retval][out] */ __RPC__out long *plVolume);
        
        DECLSPEC_XFGVIRT(IRTCClient, put_PreferredAEC)
        /* [helpstring][propput] */ HRESULT ( STDMETHODCALLTYPE *put_PreferredAEC )( 
            __RPC__in IRTCClient2 * This,
            /* [in] */ VARIANT_BOOL bEnable);
        
        DECLSPEC_XFGVIRT(IRTCClient, get_PreferredAEC)
        /* [helpstring][propget] */ HRESULT ( STDMETHODCALLTYPE *get_PreferredAEC )( 
            __RPC__in IRTCClient2 * This,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pbEnabled);
        
        DECLSPEC_XFGVIRT(IRTCClient, put_PreferredVideoDevice)
        /* [helpstring][propput] */ HRESULT ( STDMETHODCALLTYPE *put_PreferredVideoDevice )( 
            __RPC__in IRTCClient2 * This,
            /* [in] */ __RPC__in BSTR bstrDeviceName);
        
        DECLSPEC_XFGVIRT(IRTCClient, get_PreferredVideoDevice)
        /* [helpstring][propget] */ HRESULT ( STDMETHODCALLTYPE *get_PreferredVideoDevice )( 
            __RPC__in IRTCClient2 * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrDeviceName);
        
        DECLSPEC_XFGVIRT(IRTCClient, get_ActiveMedia)
        /* [helpstring][propget] */ HRESULT ( STDMETHODCALLTYPE *get_ActiveMedia )( 
            __RPC__in IRTCClient2 * This,
            /* [retval][out] */ __RPC__out long *plMediaType);
        
        DECLSPEC_XFGVIRT(IRTCClient, put_MaxBitrate)
        /* [helpstring][propput] */ HRESULT ( STDMETHODCALLTYPE *put_MaxBitrate )( 
            __RPC__in IRTCClient2 * This,
            /* [in] */ long lMaxBitrate);
        
        DECLSPEC_XFGVIRT(IRTCClient, get_MaxBitrate)
        /* [helpstring][propget] */ HRESULT ( STDMETHODCALLTYPE *get_MaxBitrate )( 
            __RPC__in IRTCClient2 * This,
            /* [retval][out] */ __RPC__out long *plMaxBitrate);
        
        DECLSPEC_XFGVIRT(IRTCClient, put_TemporalSpatialTradeOff)
        /* [helpstring][propput] */ HRESULT ( STDMETHODCALLTYPE *put_TemporalSpatialTradeOff )( 
            __RPC__in IRTCClient2 * This,
            /* [in] */ long lValue);
        
        DECLSPEC_XFGVIRT(IRTCClient, get_TemporalSpatialTradeOff)
        /* [helpstring][propget] */ HRESULT ( STDMETHODCALLTYPE *get_TemporalSpatialTradeOff )( 
            __RPC__in IRTCClient2 * This,
            /* [retval][out] */ __RPC__out long *plValue);
        
        DECLSPEC_XFGVIRT(IRTCClient, get_NetworkQuality)
        /* [helpstring][propget] */ HRESULT ( STDMETHODCALLTYPE *get_NetworkQuality )( 
            __RPC__in IRTCClient2 * This,
            /* [retval][out] */ __RPC__out long *plNetworkQuality);
        
        DECLSPEC_XFGVIRT(IRTCClient, StartT120Applet)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *StartT120Applet )( 
            __RPC__in IRTCClient2 * This,
            /* [in] */ RTC_T120_APPLET enApplet);
        
        DECLSPEC_XFGVIRT(IRTCClient, StopT120Applets)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *StopT120Applets )( 
            __RPC__in IRTCClient2 * This);
        
        DECLSPEC_XFGVIRT(IRTCClient, get_IsT120AppletRunning)
        /* [helpstring][propget] */ HRESULT ( STDMETHODCALLTYPE *get_IsT120AppletRunning )( 
            __RPC__in IRTCClient2 * This,
            /* [in] */ RTC_T120_APPLET enApplet,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pfRunning);
        
        DECLSPEC_XFGVIRT(IRTCClient, get_LocalUserURI)
        /* [helpstring][propget] */ HRESULT ( STDMETHODCALLTYPE *get_LocalUserURI )( 
            __RPC__in IRTCClient2 * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrUserURI);
        
        DECLSPEC_XFGVIRT(IRTCClient, put_LocalUserURI)
        /* [helpstring][propput] */ HRESULT ( STDMETHODCALLTYPE *put_LocalUserURI )( 
            __RPC__in IRTCClient2 * This,
            /* [in] */ __RPC__in BSTR bstrUserURI);
        
        DECLSPEC_XFGVIRT(IRTCClient, get_LocalUserName)
        /* [helpstring][propget] */ HRESULT ( STDMETHODCALLTYPE *get_LocalUserName )( 
            __RPC__in IRTCClient2 * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrUserName);
        
        DECLSPEC_XFGVIRT(IRTCClient, put_LocalUserName)
        /* [helpstring][propput] */ HRESULT ( STDMETHODCALLTYPE *put_LocalUserName )( 
            __RPC__in IRTCClient2 * This,
            /* [in] */ __RPC__in BSTR bstrUserName);
        
        DECLSPEC_XFGVIRT(IRTCClient, PlayRing)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *PlayRing )( 
            __RPC__in IRTCClient2 * This,
            /* [in] */ RTC_RING_TYPE enType,
            /* [in] */ VARIANT_BOOL bPlay);
        
        DECLSPEC_XFGVIRT(IRTCClient, SendDTMF)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *SendDTMF )( 
            __RPC__in IRTCClient2 * This,
            /* [in] */ RTC_DTMF enDTMF);
        
        DECLSPEC_XFGVIRT(IRTCClient, InvokeTuningWizard)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *InvokeTuningWizard )( 
            __RPC__in IRTCClient2 * This,
            /* [in] */ OAHWND hwndParent);
        
        DECLSPEC_XFGVIRT(IRTCClient, get_IsTuned)
        /* [helpstring][propget] */ HRESULT ( STDMETHODCALLTYPE *get_IsTuned )( 
            __RPC__in IRTCClient2 * This,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pfTuned);
        
        DECLSPEC_XFGVIRT(IRTCClient2, put_AnswerMode)
        /* [helpstring][propput] */ HRESULT ( STDMETHODCALLTYPE *put_AnswerMode )( 
            __RPC__in IRTCClient2 * This,
            /* [in] */ RTC_SESSION_TYPE enType,
            /* [in] */ RTC_ANSWER_MODE enMode);
        
        DECLSPEC_XFGVIRT(IRTCClient2, get_AnswerMode)
        /* [helpstring][propget] */ HRESULT ( STDMETHODCALLTYPE *get_AnswerMode )( 
            __RPC__in IRTCClient2 * This,
            /* [in] */ RTC_SESSION_TYPE enType,
            /* [retval][out] */ __RPC__out RTC_ANSWER_MODE *penMode);
        
        DECLSPEC_XFGVIRT(IRTCClient2, InvokeTuningWizardEx)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *InvokeTuningWizardEx )( 
            __RPC__in IRTCClient2 * This,
            /* [in] */ OAHWND hwndParent,
            /* [in] */ VARIANT_BOOL fAllowAudio,
            /* [in] */ VARIANT_BOOL fAllowVideo);
        
        DECLSPEC_XFGVIRT(IRTCClient2, get_Version)
        /* [helpstring][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Version )( 
            __RPC__in IRTCClient2 * This,
            /* [retval][out] */ __RPC__out long *plVersion);
        
        DECLSPEC_XFGVIRT(IRTCClient2, put_ClientName)
        /* [helpstring][propput] */ HRESULT ( STDMETHODCALLTYPE *put_ClientName )( 
            __RPC__in IRTCClient2 * This,
            /* [in] */ __RPC__in BSTR bstrClientName);
        
        DECLSPEC_XFGVIRT(IRTCClient2, put_ClientCurVer)
        /* [helpstring][propput] */ HRESULT ( STDMETHODCALLTYPE *put_ClientCurVer )( 
            __RPC__in IRTCClient2 * This,
            /* [in] */ __RPC__in BSTR bstrClientCurVer);
        
        DECLSPEC_XFGVIRT(IRTCClient2, InitializeEx)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *InitializeEx )( 
            __RPC__in IRTCClient2 * This,
            /* [in] */ long lFlags);
        
        DECLSPEC_XFGVIRT(IRTCClient2, CreateSessionWithDescription)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *CreateSessionWithDescription )( 
            __RPC__in IRTCClient2 * This,
            /* [in] */ __RPC__in BSTR bstrContentType,
            /* [in] */ __RPC__in BSTR bstrSessionDescription,
            /* [in] */ __RPC__in_opt IRTCProfile *pProfile,
            /* [in] */ long lFlags,
            /* [retval][out] */ __RPC__deref_out_opt IRTCSession2 **ppSession2);
        
        DECLSPEC_XFGVIRT(IRTCClient2, SetSessionDescriptionManager)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *SetSessionDescriptionManager )( 
            __RPC__in IRTCClient2 * This,
            /* [in] */ __RPC__in_opt IRTCSessionDescriptionManager *pSessionDescriptionManager);
        
        DECLSPEC_XFGVIRT(IRTCClient2, put_PreferredSecurityLevel)
        /* [helpstring][propput] */ HRESULT ( STDMETHODCALLTYPE *put_PreferredSecurityLevel )( 
            __RPC__in IRTCClient2 * This,
            /* [in] */ RTC_SECURITY_TYPE enSecurityType,
            /* [in] */ RTC_SECURITY_LEVEL enSecurityLevel);
        
        DECLSPEC_XFGVIRT(IRTCClient2, get_PreferredSecurityLevel)
        /* [helpstring][propget] */ HRESULT ( STDMETHODCALLTYPE *get_PreferredSecurityLevel )( 
            __RPC__in IRTCClient2 * This,
            /* [in] */ RTC_SECURITY_TYPE enSecurityType,
            /* [retval][out] */ __RPC__out RTC_SECURITY_LEVEL *penSecurityLevel);
        
        DECLSPEC_XFGVIRT(IRTCClient2, put_AllowedPorts)
        /* [helpstring][propput] */ HRESULT ( STDMETHODCALLTYPE *put_AllowedPorts )( 
            __RPC__in IRTCClient2 * This,
            /* [in] */ long lTransport,
            /* [in] */ RTC_LISTEN_MODE enListenMode);
        
        DECLSPEC_XFGVIRT(IRTCClient2, get_AllowedPorts)
        /* [helpstring][propget] */ HRESULT ( STDMETHODCALLTYPE *get_AllowedPorts )( 
            __RPC__in IRTCClient2 * This,
            /* [in] */ long lTransport,
            /* [retval][out] */ __RPC__out RTC_LISTEN_MODE *penListenMode);
        
        END_INTERFACE
    } IRTCClient2Vtbl;

    interface IRTCClient2
    {
        CONST_VTBL struct IRTCClient2Vtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IRTCClient2_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IRTCClient2_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IRTCClient2_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IRTCClient2_Initialize(This)	\
    ( (This)->lpVtbl -> Initialize(This) ) 

#define IRTCClient2_Shutdown(This)	\
    ( (This)->lpVtbl -> Shutdown(This) ) 

#define IRTCClient2_PrepareForShutdown(This)	\
    ( (This)->lpVtbl -> PrepareForShutdown(This) ) 

#define IRTCClient2_put_EventFilter(This,lFilter)	\
    ( (This)->lpVtbl -> put_EventFilter(This,lFilter) ) 

#define IRTCClient2_get_EventFilter(This,plFilter)	\
    ( (This)->lpVtbl -> get_EventFilter(This,plFilter) ) 

#define IRTCClient2_SetPreferredMediaTypes(This,lMediaTypes,fPersistent)	\
    ( (This)->lpVtbl -> SetPreferredMediaTypes(This,lMediaTypes,fPersistent) ) 

#define IRTCClient2_get_PreferredMediaTypes(This,plMediaTypes)	\
    ( (This)->lpVtbl -> get_PreferredMediaTypes(This,plMediaTypes) ) 

#define IRTCClient2_get_MediaCapabilities(This,plMediaTypes)	\
    ( (This)->lpVtbl -> get_MediaCapabilities(This,plMediaTypes) ) 

#define IRTCClient2_CreateSession(This,enType,bstrLocalPhoneURI,pProfile,lFlags,ppSession)	\
    ( (This)->lpVtbl -> CreateSession(This,enType,bstrLocalPhoneURI,pProfile,lFlags,ppSession) ) 

#define IRTCClient2_put_ListenForIncomingSessions(This,enListen)	\
    ( (This)->lpVtbl -> put_ListenForIncomingSessions(This,enListen) ) 

#define IRTCClient2_get_ListenForIncomingSessions(This,penListen)	\
    ( (This)->lpVtbl -> get_ListenForIncomingSessions(This,penListen) ) 

#define IRTCClient2_get_NetworkAddresses(This,fTCP,fExternal,pvAddresses)	\
    ( (This)->lpVtbl -> get_NetworkAddresses(This,fTCP,fExternal,pvAddresses) ) 

#define IRTCClient2_put_Volume(This,enDevice,lVolume)	\
    ( (This)->lpVtbl -> put_Volume(This,enDevice,lVolume) ) 

#define IRTCClient2_get_Volume(This,enDevice,plVolume)	\
    ( (This)->lpVtbl -> get_Volume(This,enDevice,plVolume) ) 

#define IRTCClient2_put_AudioMuted(This,enDevice,fMuted)	\
    ( (This)->lpVtbl -> put_AudioMuted(This,enDevice,fMuted) ) 

#define IRTCClient2_get_AudioMuted(This,enDevice,pfMuted)	\
    ( (This)->lpVtbl -> get_AudioMuted(This,enDevice,pfMuted) ) 

#define IRTCClient2_get_IVideoWindow(This,enDevice,ppIVideoWindow)	\
    ( (This)->lpVtbl -> get_IVideoWindow(This,enDevice,ppIVideoWindow) ) 

#define IRTCClient2_put_PreferredAudioDevice(This,enDevice,bstrDeviceName)	\
    ( (This)->lpVtbl -> put_PreferredAudioDevice(This,enDevice,bstrDeviceName) ) 

#define IRTCClient2_get_PreferredAudioDevice(This,enDevice,pbstrDeviceName)	\
    ( (This)->lpVtbl -> get_PreferredAudioDevice(This,enDevice,pbstrDeviceName) ) 

#define IRTCClient2_put_PreferredVolume(This,enDevice,lVolume)	\
    ( (This)->lpVtbl -> put_PreferredVolume(This,enDevice,lVolume) ) 

#define IRTCClient2_get_PreferredVolume(This,enDevice,plVolume)	\
    ( (This)->lpVtbl -> get_PreferredVolume(This,enDevice,plVolume) ) 

#define IRTCClient2_put_PreferredAEC(This,bEnable)	\
    ( (This)->lpVtbl -> put_PreferredAEC(This,bEnable) ) 

#define IRTCClient2_get_PreferredAEC(This,pbEnabled)	\
    ( (This)->lpVtbl -> get_PreferredAEC(This,pbEnabled) ) 

#define IRTCClient2_put_PreferredVideoDevice(This,bstrDeviceName)	\
    ( (This)->lpVtbl -> put_PreferredVideoDevice(This,bstrDeviceName) ) 

#define IRTCClient2_get_PreferredVideoDevice(This,pbstrDeviceName)	\
    ( (This)->lpVtbl -> get_PreferredVideoDevice(This,pbstrDeviceName) ) 

#define IRTCClient2_get_ActiveMedia(This,plMediaType)	\
    ( (This)->lpVtbl -> get_ActiveMedia(This,plMediaType) ) 

#define IRTCClient2_put_MaxBitrate(This,lMaxBitrate)	\
    ( (This)->lpVtbl -> put_MaxBitrate(This,lMaxBitrate) ) 

#define IRTCClient2_get_MaxBitrate(This,plMaxBitrate)	\
    ( (This)->lpVtbl -> get_MaxBitrate(This,plMaxBitrate) ) 

#define IRTCClient2_put_TemporalSpatialTradeOff(This,lValue)	\
    ( (This)->lpVtbl -> put_TemporalSpatialTradeOff(This,lValue) ) 

#define IRTCClient2_get_TemporalSpatialTradeOff(This,plValue)	\
    ( (This)->lpVtbl -> get_TemporalSpatialTradeOff(This,plValue) ) 

#define IRTCClient2_get_NetworkQuality(This,plNetworkQuality)	\
    ( (This)->lpVtbl -> get_NetworkQuality(This,plNetworkQuality) ) 

#define IRTCClient2_StartT120Applet(This,enApplet)	\
    ( (This)->lpVtbl -> StartT120Applet(This,enApplet) ) 

#define IRTCClient2_StopT120Applets(This)	\
    ( (This)->lpVtbl -> StopT120Applets(This) ) 

#define IRTCClient2_get_IsT120AppletRunning(This,enApplet,pfRunning)	\
    ( (This)->lpVtbl -> get_IsT120AppletRunning(This,enApplet,pfRunning) ) 

#define IRTCClient2_get_LocalUserURI(This,pbstrUserURI)	\
    ( (This)->lpVtbl -> get_LocalUserURI(This,pbstrUserURI) ) 

#define IRTCClient2_put_LocalUserURI(This,bstrUserURI)	\
    ( (This)->lpVtbl -> put_LocalUserURI(This,bstrUserURI) ) 

#define IRTCClient2_get_LocalUserName(This,pbstrUserName)	\
    ( (This)->lpVtbl -> get_LocalUserName(This,pbstrUserName) ) 

#define IRTCClient2_put_LocalUserName(This,bstrUserName)	\
    ( (This)->lpVtbl -> put_LocalUserName(This,bstrUserName) ) 

#define IRTCClient2_PlayRing(This,enType,bPlay)	\
    ( (This)->lpVtbl -> PlayRing(This,enType,bPlay) ) 

#define IRTCClient2_SendDTMF(This,enDTMF)	\
    ( (This)->lpVtbl -> SendDTMF(This,enDTMF) ) 

#define IRTCClient2_InvokeTuningWizard(This,hwndParent)	\
    ( (This)->lpVtbl -> InvokeTuningWizard(This,hwndParent) ) 

#define IRTCClient2_get_IsTuned(This,pfTuned)	\
    ( (This)->lpVtbl -> get_IsTuned(This,pfTuned) ) 


#define IRTCClient2_put_AnswerMode(This,enType,enMode)	\
    ( (This)->lpVtbl -> put_AnswerMode(This,enType,enMode) ) 

#define IRTCClient2_get_AnswerMode(This,enType,penMode)	\
    ( (This)->lpVtbl -> get_AnswerMode(This,enType,penMode) ) 

#define IRTCClient2_InvokeTuningWizardEx(This,hwndParent,fAllowAudio,fAllowVideo)	\
    ( (This)->lpVtbl -> InvokeTuningWizardEx(This,hwndParent,fAllowAudio,fAllowVideo) ) 

#define IRTCClient2_get_Version(This,plVersion)	\
    ( (This)->lpVtbl -> get_Version(This,plVersion) ) 

#define IRTCClient2_put_ClientName(This,bstrClientName)	\
    ( (This)->lpVtbl -> put_ClientName(This,bstrClientName) ) 

#define IRTCClient2_put_ClientCurVer(This,bstrClientCurVer)	\
    ( (This)->lpVtbl -> put_ClientCurVer(This,bstrClientCurVer) ) 

#define IRTCClient2_InitializeEx(This,lFlags)	\
    ( (This)->lpVtbl -> InitializeEx(This,lFlags) ) 

#define IRTCClient2_CreateSessionWithDescription(This,bstrContentType,bstrSessionDescription,pProfile,lFlags,ppSession2)	\
    ( (This)->lpVtbl -> CreateSessionWithDescription(This,bstrContentType,bstrSessionDescription,pProfile,lFlags,ppSession2) ) 

#define IRTCClient2_SetSessionDescriptionManager(This,pSessionDescriptionManager)	\
    ( (This)->lpVtbl -> SetSessionDescriptionManager(This,pSessionDescriptionManager) ) 

#define IRTCClient2_put_PreferredSecurityLevel(This,enSecurityType,enSecurityLevel)	\
    ( (This)->lpVtbl -> put_PreferredSecurityLevel(This,enSecurityType,enSecurityLevel) ) 

#define IRTCClient2_get_PreferredSecurityLevel(This,enSecurityType,penSecurityLevel)	\
    ( (This)->lpVtbl -> get_PreferredSecurityLevel(This,enSecurityType,penSecurityLevel) ) 

#define IRTCClient2_put_AllowedPorts(This,lTransport,enListenMode)	\
    ( (This)->lpVtbl -> put_AllowedPorts(This,lTransport,enListenMode) ) 

#define IRTCClient2_get_AllowedPorts(This,lTransport,penListenMode)	\
    ( (This)->lpVtbl -> get_AllowedPorts(This,lTransport,penListenMode) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IRTCClient2_INTERFACE_DEFINED__ */


#ifndef __IRTCClientPresence_INTERFACE_DEFINED__
#define __IRTCClientPresence_INTERFACE_DEFINED__

/* interface IRTCClientPresence */
/* [unique][helpstring][uuid][object] */ 


EXTERN_C const IID IID_IRTCClientPresence;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("11c3cbcc-0744-42d1-968a-51aa1bb274c6")
    IRTCClientPresence : public IUnknown
    {
    public:
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE EnablePresence( 
            /* [in] */ VARIANT_BOOL fUseStorage,
            /* [in] */ VARIANT varStorage) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE Export( 
            /* [in] */ VARIANT varStorage) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE Import( 
            /* [in] */ VARIANT varStorage,
            /* [in] */ VARIANT_BOOL fReplaceAll) = 0;
        
        virtual /* [helpstring][hidden] */ HRESULT STDMETHODCALLTYPE EnumerateBuddies( 
            /* [retval][out] */ __RPC__deref_out_opt IRTCEnumBuddies **ppEnum) = 0;
        
        virtual /* [helpstring][propget] */ HRESULT STDMETHODCALLTYPE get_Buddies( 
            /* [retval][out] */ __RPC__deref_out_opt IRTCCollection **ppCollection) = 0;
        
        virtual /* [helpstring][propget] */ HRESULT STDMETHODCALLTYPE get_Buddy( 
            /* [in] */ __RPC__in BSTR bstrPresentityURI,
            /* [retval][out] */ __RPC__deref_out_opt IRTCBuddy **ppBuddy) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE AddBuddy( 
            /* [in] */ __RPC__in BSTR bstrPresentityURI,
            /* [in] */ __RPC__in BSTR bstrUserName,
            /* [in] */ __RPC__in BSTR bstrData,
            /* [in] */ VARIANT_BOOL fPersistent,
            /* [in] */ __RPC__in_opt IRTCProfile *pProfile,
            /* [in] */ long lFlags,
            /* [retval][out] */ __RPC__deref_out_opt IRTCBuddy **ppBuddy) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE RemoveBuddy( 
            /* [in] */ __RPC__in_opt IRTCBuddy *pBuddy) = 0;
        
        virtual /* [helpstring][hidden] */ HRESULT STDMETHODCALLTYPE EnumerateWatchers( 
            /* [retval][out] */ __RPC__deref_out_opt IRTCEnumWatchers **ppEnum) = 0;
        
        virtual /* [helpstring][propget] */ HRESULT STDMETHODCALLTYPE get_Watchers( 
            /* [retval][out] */ __RPC__deref_out_opt IRTCCollection **ppCollection) = 0;
        
        virtual /* [helpstring][propget] */ HRESULT STDMETHODCALLTYPE get_Watcher( 
            /* [in] */ __RPC__in BSTR bstrPresentityURI,
            /* [retval][out] */ __RPC__deref_out_opt IRTCWatcher **ppWatcher) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE AddWatcher( 
            /* [in] */ __RPC__in BSTR bstrPresentityURI,
            /* [in] */ __RPC__in BSTR bstrUserName,
            /* [in] */ __RPC__in BSTR bstrData,
            /* [in] */ VARIANT_BOOL fBlocked,
            /* [in] */ VARIANT_BOOL fPersistent,
            /* [retval][out] */ __RPC__deref_out_opt IRTCWatcher **ppWatcher) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE RemoveWatcher( 
            /* [in] */ __RPC__in_opt IRTCWatcher *pWatcher) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE SetLocalPresenceInfo( 
            /* [in] */ RTC_PRESENCE_STATUS enStatus,
            /* [in] */ __RPC__in BSTR bstrNotes) = 0;
        
        virtual /* [helpstring][propget] */ HRESULT STDMETHODCALLTYPE get_OfferWatcherMode( 
            /* [retval][out] */ __RPC__out RTC_OFFER_WATCHER_MODE *penMode) = 0;
        
        virtual /* [helpstring][propput] */ HRESULT STDMETHODCALLTYPE put_OfferWatcherMode( 
            /* [in] */ RTC_OFFER_WATCHER_MODE enMode) = 0;
        
        virtual /* [helpstring][propget] */ HRESULT STDMETHODCALLTYPE get_PrivacyMode( 
            /* [retval][out] */ __RPC__out RTC_PRIVACY_MODE *penMode) = 0;
        
        virtual /* [helpstring][propput] */ HRESULT STDMETHODCALLTYPE put_PrivacyMode( 
            /* [in] */ RTC_PRIVACY_MODE enMode) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IRTCClientPresenceVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IRTCClientPresence * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IRTCClientPresence * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IRTCClientPresence * This);
        
        DECLSPEC_XFGVIRT(IRTCClientPresence, EnablePresence)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *EnablePresence )( 
            __RPC__in IRTCClientPresence * This,
            /* [in] */ VARIANT_BOOL fUseStorage,
            /* [in] */ VARIANT varStorage);
        
        DECLSPEC_XFGVIRT(IRTCClientPresence, Export)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *Export )( 
            __RPC__in IRTCClientPresence * This,
            /* [in] */ VARIANT varStorage);
        
        DECLSPEC_XFGVIRT(IRTCClientPresence, Import)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *Import )( 
            __RPC__in IRTCClientPresence * This,
            /* [in] */ VARIANT varStorage,
            /* [in] */ VARIANT_BOOL fReplaceAll);
        
        DECLSPEC_XFGVIRT(IRTCClientPresence, EnumerateBuddies)
        /* [helpstring][hidden] */ HRESULT ( STDMETHODCALLTYPE *EnumerateBuddies )( 
            __RPC__in IRTCClientPresence * This,
            /* [retval][out] */ __RPC__deref_out_opt IRTCEnumBuddies **ppEnum);
        
        DECLSPEC_XFGVIRT(IRTCClientPresence, get_Buddies)
        /* [helpstring][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Buddies )( 
            __RPC__in IRTCClientPresence * This,
            /* [retval][out] */ __RPC__deref_out_opt IRTCCollection **ppCollection);
        
        DECLSPEC_XFGVIRT(IRTCClientPresence, get_Buddy)
        /* [helpstring][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Buddy )( 
            __RPC__in IRTCClientPresence * This,
            /* [in] */ __RPC__in BSTR bstrPresentityURI,
            /* [retval][out] */ __RPC__deref_out_opt IRTCBuddy **ppBuddy);
        
        DECLSPEC_XFGVIRT(IRTCClientPresence, AddBuddy)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *AddBuddy )( 
            __RPC__in IRTCClientPresence * This,
            /* [in] */ __RPC__in BSTR bstrPresentityURI,
            /* [in] */ __RPC__in BSTR bstrUserName,
            /* [in] */ __RPC__in BSTR bstrData,
            /* [in] */ VARIANT_BOOL fPersistent,
            /* [in] */ __RPC__in_opt IRTCProfile *pProfile,
            /* [in] */ long lFlags,
            /* [retval][out] */ __RPC__deref_out_opt IRTCBuddy **ppBuddy);
        
        DECLSPEC_XFGVIRT(IRTCClientPresence, RemoveBuddy)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *RemoveBuddy )( 
            __RPC__in IRTCClientPresence * This,
            /* [in] */ __RPC__in_opt IRTCBuddy *pBuddy);
        
        DECLSPEC_XFGVIRT(IRTCClientPresence, EnumerateWatchers)
        /* [helpstring][hidden] */ HRESULT ( STDMETHODCALLTYPE *EnumerateWatchers )( 
            __RPC__in IRTCClientPresence * This,
            /* [retval][out] */ __RPC__deref_out_opt IRTCEnumWatchers **ppEnum);
        
        DECLSPEC_XFGVIRT(IRTCClientPresence, get_Watchers)
        /* [helpstring][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Watchers )( 
            __RPC__in IRTCClientPresence * This,
            /* [retval][out] */ __RPC__deref_out_opt IRTCCollection **ppCollection);
        
        DECLSPEC_XFGVIRT(IRTCClientPresence, get_Watcher)
        /* [helpstring][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Watcher )( 
            __RPC__in IRTCClientPresence * This,
            /* [in] */ __RPC__in BSTR bstrPresentityURI,
            /* [retval][out] */ __RPC__deref_out_opt IRTCWatcher **ppWatcher);
        
        DECLSPEC_XFGVIRT(IRTCClientPresence, AddWatcher)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *AddWatcher )( 
            __RPC__in IRTCClientPresence * This,
            /* [in] */ __RPC__in BSTR bstrPresentityURI,
            /* [in] */ __RPC__in BSTR bstrUserName,
            /* [in] */ __RPC__in BSTR bstrData,
            /* [in] */ VARIANT_BOOL fBlocked,
            /* [in] */ VARIANT_BOOL fPersistent,
            /* [retval][out] */ __RPC__deref_out_opt IRTCWatcher **ppWatcher);
        
        DECLSPEC_XFGVIRT(IRTCClientPresence, RemoveWatcher)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *RemoveWatcher )( 
            __RPC__in IRTCClientPresence * This,
            /* [in] */ __RPC__in_opt IRTCWatcher *pWatcher);
        
        DECLSPEC_XFGVIRT(IRTCClientPresence, SetLocalPresenceInfo)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *SetLocalPresenceInfo )( 
            __RPC__in IRTCClientPresence * This,
            /* [in] */ RTC_PRESENCE_STATUS enStatus,
            /* [in] */ __RPC__in BSTR bstrNotes);
        
        DECLSPEC_XFGVIRT(IRTCClientPresence, get_OfferWatcherMode)
        /* [helpstring][propget] */ HRESULT ( STDMETHODCALLTYPE *get_OfferWatcherMode )( 
            __RPC__in IRTCClientPresence * This,
            /* [retval][out] */ __RPC__out RTC_OFFER_WATCHER_MODE *penMode);
        
        DECLSPEC_XFGVIRT(IRTCClientPresence, put_OfferWatcherMode)
        /* [helpstring][propput] */ HRESULT ( STDMETHODCALLTYPE *put_OfferWatcherMode )( 
            __RPC__in IRTCClientPresence * This,
            /* [in] */ RTC_OFFER_WATCHER_MODE enMode);
        
        DECLSPEC_XFGVIRT(IRTCClientPresence, get_PrivacyMode)
        /* [helpstring][propget] */ HRESULT ( STDMETHODCALLTYPE *get_PrivacyMode )( 
            __RPC__in IRTCClientPresence * This,
            /* [retval][out] */ __RPC__out RTC_PRIVACY_MODE *penMode);
        
        DECLSPEC_XFGVIRT(IRTCClientPresence, put_PrivacyMode)
        /* [helpstring][propput] */ HRESULT ( STDMETHODCALLTYPE *put_PrivacyMode )( 
            __RPC__in IRTCClientPresence * This,
            /* [in] */ RTC_PRIVACY_MODE enMode);
        
        END_INTERFACE
    } IRTCClientPresenceVtbl;

    interface IRTCClientPresence
    {
        CONST_VTBL struct IRTCClientPresenceVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IRTCClientPresence_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IRTCClientPresence_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IRTCClientPresence_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IRTCClientPresence_EnablePresence(This,fUseStorage,varStorage)	\
    ( (This)->lpVtbl -> EnablePresence(This,fUseStorage,varStorage) ) 

#define IRTCClientPresence_Export(This,varStorage)	\
    ( (This)->lpVtbl -> Export(This,varStorage) ) 

#define IRTCClientPresence_Import(This,varStorage,fReplaceAll)	\
    ( (This)->lpVtbl -> Import(This,varStorage,fReplaceAll) ) 

#define IRTCClientPresence_EnumerateBuddies(This,ppEnum)	\
    ( (This)->lpVtbl -> EnumerateBuddies(This,ppEnum) ) 

#define IRTCClientPresence_get_Buddies(This,ppCollection)	\
    ( (This)->lpVtbl -> get_Buddies(This,ppCollection) ) 

#define IRTCClientPresence_get_Buddy(This,bstrPresentityURI,ppBuddy)	\
    ( (This)->lpVtbl -> get_Buddy(This,bstrPresentityURI,ppBuddy) ) 

#define IRTCClientPresence_AddBuddy(This,bstrPresentityURI,bstrUserName,bstrData,fPersistent,pProfile,lFlags,ppBuddy)	\
    ( (This)->lpVtbl -> AddBuddy(This,bstrPresentityURI,bstrUserName,bstrData,fPersistent,pProfile,lFlags,ppBuddy) ) 

#define IRTCClientPresence_RemoveBuddy(This,pBuddy)	\
    ( (This)->lpVtbl -> RemoveBuddy(This,pBuddy) ) 

#define IRTCClientPresence_EnumerateWatchers(This,ppEnum)	\
    ( (This)->lpVtbl -> EnumerateWatchers(This,ppEnum) ) 

#define IRTCClientPresence_get_Watchers(This,ppCollection)	\
    ( (This)->lpVtbl -> get_Watchers(This,ppCollection) ) 

#define IRTCClientPresence_get_Watcher(This,bstrPresentityURI,ppWatcher)	\
    ( (This)->lpVtbl -> get_Watcher(This,bstrPresentityURI,ppWatcher) ) 

#define IRTCClientPresence_AddWatcher(This,bstrPresentityURI,bstrUserName,bstrData,fBlocked,fPersistent,ppWatcher)	\
    ( (This)->lpVtbl -> AddWatcher(This,bstrPresentityURI,bstrUserName,bstrData,fBlocked,fPersistent,ppWatcher) ) 

#define IRTCClientPresence_RemoveWatcher(This,pWatcher)	\
    ( (This)->lpVtbl -> RemoveWatcher(This,pWatcher) ) 

#define IRTCClientPresence_SetLocalPresenceInfo(This,enStatus,bstrNotes)	\
    ( (This)->lpVtbl -> SetLocalPresenceInfo(This,enStatus,bstrNotes) ) 

#define IRTCClientPresence_get_OfferWatcherMode(This,penMode)	\
    ( (This)->lpVtbl -> get_OfferWatcherMode(This,penMode) ) 

#define IRTCClientPresence_put_OfferWatcherMode(This,enMode)	\
    ( (This)->lpVtbl -> put_OfferWatcherMode(This,enMode) ) 

#define IRTCClientPresence_get_PrivacyMode(This,penMode)	\
    ( (This)->lpVtbl -> get_PrivacyMode(This,penMode) ) 

#define IRTCClientPresence_put_PrivacyMode(This,enMode)	\
    ( (This)->lpVtbl -> put_PrivacyMode(This,enMode) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IRTCClientPresence_INTERFACE_DEFINED__ */


#ifndef __IRTCClientPresence2_INTERFACE_DEFINED__
#define __IRTCClientPresence2_INTERFACE_DEFINED__

/* interface IRTCClientPresence2 */
/* [unique][helpstring][uuid][object] */ 


EXTERN_C const IID IID_IRTCClientPresence2;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("AD1809E8-62F7-4783-909A-29C9D2CB1D34")
    IRTCClientPresence2 : public IRTCClientPresence
    {
    public:
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE EnablePresenceEx( 
            /* [in] */ __RPC__in_opt IRTCProfile *pProfile,
            /* [in] */ VARIANT varStorage,
            /* [in] */ long lFlags) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE DisablePresence( void) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE AddGroup( 
            /* [in] */ __RPC__in BSTR bstrGroupName,
            /* [in] */ __RPC__in BSTR bstrData,
            /* [in] */ __RPC__in_opt IRTCProfile *pProfile,
            /* [in] */ long lFlags,
            /* [retval][out] */ __RPC__deref_out_opt IRTCBuddyGroup **ppGroup) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE RemoveGroup( 
            /* [in] */ __RPC__in_opt IRTCBuddyGroup *pGroup) = 0;
        
        virtual /* [helpstring][hidden] */ HRESULT STDMETHODCALLTYPE EnumerateGroups( 
            /* [retval][out] */ __RPC__deref_out_opt IRTCEnumGroups **ppEnum) = 0;
        
        virtual /* [helpstring][propget] */ HRESULT STDMETHODCALLTYPE get_Groups( 
            /* [retval][out] */ __RPC__deref_out_opt IRTCCollection **ppCollection) = 0;
        
        virtual /* [helpstring][propget] */ HRESULT STDMETHODCALLTYPE get_Group( 
            /* [in] */ __RPC__in BSTR bstrGroupName,
            /* [retval][out] */ __RPC__deref_out_opt IRTCBuddyGroup **ppGroup) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE AddWatcherEx( 
            /* [in] */ __RPC__in BSTR bstrPresentityURI,
            /* [in] */ __RPC__in BSTR bstrUserName,
            /* [in] */ __RPC__in BSTR bstrData,
            /* [in] */ RTC_WATCHER_STATE enState,
            /* [in] */ VARIANT_BOOL fPersistent,
            /* [in] */ RTC_ACE_SCOPE enScope,
            /* [in] */ __RPC__in_opt IRTCProfile *pProfile,
            /* [in] */ long lFlags,
            /* [retval][out] */ __RPC__deref_out_opt IRTCWatcher2 **ppWatcher) = 0;
        
        virtual /* [helpstring][propget] */ HRESULT STDMETHODCALLTYPE get_WatcherEx( 
            /* [in] */ RTC_WATCHER_MATCH_MODE enMode,
            /* [in] */ __RPC__in BSTR bstrPresentityURI,
            /* [retval][out] */ __RPC__deref_out_opt IRTCWatcher2 **ppWatcher) = 0;
        
        virtual /* [helpstring][propput] */ HRESULT STDMETHODCALLTYPE put_PresenceProperty( 
            /* [in] */ RTC_PRESENCE_PROPERTY enProperty,
            /* [in] */ __RPC__in BSTR bstrProperty) = 0;
        
        virtual /* [helpstring][propget] */ HRESULT STDMETHODCALLTYPE get_PresenceProperty( 
            /* [in] */ RTC_PRESENCE_PROPERTY enProperty,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrProperty) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE SetPresenceData( 
            /* [in] */ __RPC__in BSTR bstrNamespace,
            /* [in] */ __RPC__in BSTR bstrData) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE GetPresenceData( 
            /* [out] */ __RPC__deref_out_opt BSTR *pbstrNamespace,
            /* [out] */ __RPC__deref_out_opt BSTR *pbstrData) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE GetLocalPresenceInfo( 
            /* [out] */ __RPC__out RTC_PRESENCE_STATUS *penStatus,
            /* [out] */ __RPC__deref_out_opt BSTR *pbstrNotes) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE AddBuddyEx( 
            /* [in] */ __RPC__in BSTR bstrPresentityURI,
            /* [in] */ __RPC__in BSTR bstrUserName,
            /* [in] */ __RPC__in BSTR bstrData,
            /* [in] */ VARIANT_BOOL fPersistent,
            /* [in] */ RTC_BUDDY_SUBSCRIPTION_TYPE enSubscriptionType,
            /* [in] */ __RPC__in_opt IRTCProfile *pProfile,
            /* [in] */ long lFlags,
            /* [retval][out] */ __RPC__deref_out_opt IRTCBuddy2 **ppBuddy) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IRTCClientPresence2Vtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IRTCClientPresence2 * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IRTCClientPresence2 * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IRTCClientPresence2 * This);
        
        DECLSPEC_XFGVIRT(IRTCClientPresence, EnablePresence)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *EnablePresence )( 
            __RPC__in IRTCClientPresence2 * This,
            /* [in] */ VARIANT_BOOL fUseStorage,
            /* [in] */ VARIANT varStorage);
        
        DECLSPEC_XFGVIRT(IRTCClientPresence, Export)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *Export )( 
            __RPC__in IRTCClientPresence2 * This,
            /* [in] */ VARIANT varStorage);
        
        DECLSPEC_XFGVIRT(IRTCClientPresence, Import)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *Import )( 
            __RPC__in IRTCClientPresence2 * This,
            /* [in] */ VARIANT varStorage,
            /* [in] */ VARIANT_BOOL fReplaceAll);
        
        DECLSPEC_XFGVIRT(IRTCClientPresence, EnumerateBuddies)
        /* [helpstring][hidden] */ HRESULT ( STDMETHODCALLTYPE *EnumerateBuddies )( 
            __RPC__in IRTCClientPresence2 * This,
            /* [retval][out] */ __RPC__deref_out_opt IRTCEnumBuddies **ppEnum);
        
        DECLSPEC_XFGVIRT(IRTCClientPresence, get_Buddies)
        /* [helpstring][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Buddies )( 
            __RPC__in IRTCClientPresence2 * This,
            /* [retval][out] */ __RPC__deref_out_opt IRTCCollection **ppCollection);
        
        DECLSPEC_XFGVIRT(IRTCClientPresence, get_Buddy)
        /* [helpstring][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Buddy )( 
            __RPC__in IRTCClientPresence2 * This,
            /* [in] */ __RPC__in BSTR bstrPresentityURI,
            /* [retval][out] */ __RPC__deref_out_opt IRTCBuddy **ppBuddy);
        
        DECLSPEC_XFGVIRT(IRTCClientPresence, AddBuddy)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *AddBuddy )( 
            __RPC__in IRTCClientPresence2 * This,
            /* [in] */ __RPC__in BSTR bstrPresentityURI,
            /* [in] */ __RPC__in BSTR bstrUserName,
            /* [in] */ __RPC__in BSTR bstrData,
            /* [in] */ VARIANT_BOOL fPersistent,
            /* [in] */ __RPC__in_opt IRTCProfile *pProfile,
            /* [in] */ long lFlags,
            /* [retval][out] */ __RPC__deref_out_opt IRTCBuddy **ppBuddy);
        
        DECLSPEC_XFGVIRT(IRTCClientPresence, RemoveBuddy)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *RemoveBuddy )( 
            __RPC__in IRTCClientPresence2 * This,
            /* [in] */ __RPC__in_opt IRTCBuddy *pBuddy);
        
        DECLSPEC_XFGVIRT(IRTCClientPresence, EnumerateWatchers)
        /* [helpstring][hidden] */ HRESULT ( STDMETHODCALLTYPE *EnumerateWatchers )( 
            __RPC__in IRTCClientPresence2 * This,
            /* [retval][out] */ __RPC__deref_out_opt IRTCEnumWatchers **ppEnum);
        
        DECLSPEC_XFGVIRT(IRTCClientPresence, get_Watchers)
        /* [helpstring][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Watchers )( 
            __RPC__in IRTCClientPresence2 * This,
            /* [retval][out] */ __RPC__deref_out_opt IRTCCollection **ppCollection);
        
        DECLSPEC_XFGVIRT(IRTCClientPresence, get_Watcher)
        /* [helpstring][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Watcher )( 
            __RPC__in IRTCClientPresence2 * This,
            /* [in] */ __RPC__in BSTR bstrPresentityURI,
            /* [retval][out] */ __RPC__deref_out_opt IRTCWatcher **ppWatcher);
        
        DECLSPEC_XFGVIRT(IRTCClientPresence, AddWatcher)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *AddWatcher )( 
            __RPC__in IRTCClientPresence2 * This,
            /* [in] */ __RPC__in BSTR bstrPresentityURI,
            /* [in] */ __RPC__in BSTR bstrUserName,
            /* [in] */ __RPC__in BSTR bstrData,
            /* [in] */ VARIANT_BOOL fBlocked,
            /* [in] */ VARIANT_BOOL fPersistent,
            /* [retval][out] */ __RPC__deref_out_opt IRTCWatcher **ppWatcher);
        
        DECLSPEC_XFGVIRT(IRTCClientPresence, RemoveWatcher)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *RemoveWatcher )( 
            __RPC__in IRTCClientPresence2 * This,
            /* [in] */ __RPC__in_opt IRTCWatcher *pWatcher);
        
        DECLSPEC_XFGVIRT(IRTCClientPresence, SetLocalPresenceInfo)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *SetLocalPresenceInfo )( 
            __RPC__in IRTCClientPresence2 * This,
            /* [in] */ RTC_PRESENCE_STATUS enStatus,
            /* [in] */ __RPC__in BSTR bstrNotes);
        
        DECLSPEC_XFGVIRT(IRTCClientPresence, get_OfferWatcherMode)
        /* [helpstring][propget] */ HRESULT ( STDMETHODCALLTYPE *get_OfferWatcherMode )( 
            __RPC__in IRTCClientPresence2 * This,
            /* [retval][out] */ __RPC__out RTC_OFFER_WATCHER_MODE *penMode);
        
        DECLSPEC_XFGVIRT(IRTCClientPresence, put_OfferWatcherMode)
        /* [helpstring][propput] */ HRESULT ( STDMETHODCALLTYPE *put_OfferWatcherMode )( 
            __RPC__in IRTCClientPresence2 * This,
            /* [in] */ RTC_OFFER_WATCHER_MODE enMode);
        
        DECLSPEC_XFGVIRT(IRTCClientPresence, get_PrivacyMode)
        /* [helpstring][propget] */ HRESULT ( STDMETHODCALLTYPE *get_PrivacyMode )( 
            __RPC__in IRTCClientPresence2 * This,
            /* [retval][out] */ __RPC__out RTC_PRIVACY_MODE *penMode);
        
        DECLSPEC_XFGVIRT(IRTCClientPresence, put_PrivacyMode)
        /* [helpstring][propput] */ HRESULT ( STDMETHODCALLTYPE *put_PrivacyMode )( 
            __RPC__in IRTCClientPresence2 * This,
            /* [in] */ RTC_PRIVACY_MODE enMode);
        
        DECLSPEC_XFGVIRT(IRTCClientPresence2, EnablePresenceEx)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *EnablePresenceEx )( 
            __RPC__in IRTCClientPresence2 * This,
            /* [in] */ __RPC__in_opt IRTCProfile *pProfile,
            /* [in] */ VARIANT varStorage,
            /* [in] */ long lFlags);
        
        DECLSPEC_XFGVIRT(IRTCClientPresence2, DisablePresence)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *DisablePresence )( 
            __RPC__in IRTCClientPresence2 * This);
        
        DECLSPEC_XFGVIRT(IRTCClientPresence2, AddGroup)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *AddGroup )( 
            __RPC__in IRTCClientPresence2 * This,
            /* [in] */ __RPC__in BSTR bstrGroupName,
            /* [in] */ __RPC__in BSTR bstrData,
            /* [in] */ __RPC__in_opt IRTCProfile *pProfile,
            /* [in] */ long lFlags,
            /* [retval][out] */ __RPC__deref_out_opt IRTCBuddyGroup **ppGroup);
        
        DECLSPEC_XFGVIRT(IRTCClientPresence2, RemoveGroup)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *RemoveGroup )( 
            __RPC__in IRTCClientPresence2 * This,
            /* [in] */ __RPC__in_opt IRTCBuddyGroup *pGroup);
        
        DECLSPEC_XFGVIRT(IRTCClientPresence2, EnumerateGroups)
        /* [helpstring][hidden] */ HRESULT ( STDMETHODCALLTYPE *EnumerateGroups )( 
            __RPC__in IRTCClientPresence2 * This,
            /* [retval][out] */ __RPC__deref_out_opt IRTCEnumGroups **ppEnum);
        
        DECLSPEC_XFGVIRT(IRTCClientPresence2, get_Groups)
        /* [helpstring][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Groups )( 
            __RPC__in IRTCClientPresence2 * This,
            /* [retval][out] */ __RPC__deref_out_opt IRTCCollection **ppCollection);
        
        DECLSPEC_XFGVIRT(IRTCClientPresence2, get_Group)
        /* [helpstring][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Group )( 
            __RPC__in IRTCClientPresence2 * This,
            /* [in] */ __RPC__in BSTR bstrGroupName,
            /* [retval][out] */ __RPC__deref_out_opt IRTCBuddyGroup **ppGroup);
        
        DECLSPEC_XFGVIRT(IRTCClientPresence2, AddWatcherEx)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *AddWatcherEx )( 
            __RPC__in IRTCClientPresence2 * This,
            /* [in] */ __RPC__in BSTR bstrPresentityURI,
            /* [in] */ __RPC__in BSTR bstrUserName,
            /* [in] */ __RPC__in BSTR bstrData,
            /* [in] */ RTC_WATCHER_STATE enState,
            /* [in] */ VARIANT_BOOL fPersistent,
            /* [in] */ RTC_ACE_SCOPE enScope,
            /* [in] */ __RPC__in_opt IRTCProfile *pProfile,
            /* [in] */ long lFlags,
            /* [retval][out] */ __RPC__deref_out_opt IRTCWatcher2 **ppWatcher);
        
        DECLSPEC_XFGVIRT(IRTCClientPresence2, get_WatcherEx)
        /* [helpstring][propget] */ HRESULT ( STDMETHODCALLTYPE *get_WatcherEx )( 
            __RPC__in IRTCClientPresence2 * This,
            /* [in] */ RTC_WATCHER_MATCH_MODE enMode,
            /* [in] */ __RPC__in BSTR bstrPresentityURI,
            /* [retval][out] */ __RPC__deref_out_opt IRTCWatcher2 **ppWatcher);
        
        DECLSPEC_XFGVIRT(IRTCClientPresence2, put_PresenceProperty)
        /* [helpstring][propput] */ HRESULT ( STDMETHODCALLTYPE *put_PresenceProperty )( 
            __RPC__in IRTCClientPresence2 * This,
            /* [in] */ RTC_PRESENCE_PROPERTY enProperty,
            /* [in] */ __RPC__in BSTR bstrProperty);
        
        DECLSPEC_XFGVIRT(IRTCClientPresence2, get_PresenceProperty)
        /* [helpstring][propget] */ HRESULT ( STDMETHODCALLTYPE *get_PresenceProperty )( 
            __RPC__in IRTCClientPresence2 * This,
            /* [in] */ RTC_PRESENCE_PROPERTY enProperty,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrProperty);
        
        DECLSPEC_XFGVIRT(IRTCClientPresence2, SetPresenceData)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *SetPresenceData )( 
            __RPC__in IRTCClientPresence2 * This,
            /* [in] */ __RPC__in BSTR bstrNamespace,
            /* [in] */ __RPC__in BSTR bstrData);
        
        DECLSPEC_XFGVIRT(IRTCClientPresence2, GetPresenceData)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *GetPresenceData )( 
            __RPC__in IRTCClientPresence2 * This,
            /* [out] */ __RPC__deref_out_opt BSTR *pbstrNamespace,
            /* [out] */ __RPC__deref_out_opt BSTR *pbstrData);
        
        DECLSPEC_XFGVIRT(IRTCClientPresence2, GetLocalPresenceInfo)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *GetLocalPresenceInfo )( 
            __RPC__in IRTCClientPresence2 * This,
            /* [out] */ __RPC__out RTC_PRESENCE_STATUS *penStatus,
            /* [out] */ __RPC__deref_out_opt BSTR *pbstrNotes);
        
        DECLSPEC_XFGVIRT(IRTCClientPresence2, AddBuddyEx)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *AddBuddyEx )( 
            __RPC__in IRTCClientPresence2 * This,
            /* [in] */ __RPC__in BSTR bstrPresentityURI,
            /* [in] */ __RPC__in BSTR bstrUserName,
            /* [in] */ __RPC__in BSTR bstrData,
            /* [in] */ VARIANT_BOOL fPersistent,
            /* [in] */ RTC_BUDDY_SUBSCRIPTION_TYPE enSubscriptionType,
            /* [in] */ __RPC__in_opt IRTCProfile *pProfile,
            /* [in] */ long lFlags,
            /* [retval][out] */ __RPC__deref_out_opt IRTCBuddy2 **ppBuddy);
        
        END_INTERFACE
    } IRTCClientPresence2Vtbl;

    interface IRTCClientPresence2
    {
        CONST_VTBL struct IRTCClientPresence2Vtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IRTCClientPresence2_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IRTCClientPresence2_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IRTCClientPresence2_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IRTCClientPresence2_EnablePresence(This,fUseStorage,varStorage)	\
    ( (This)->lpVtbl -> EnablePresence(This,fUseStorage,varStorage) ) 

#define IRTCClientPresence2_Export(This,varStorage)	\
    ( (This)->lpVtbl -> Export(This,varStorage) ) 

#define IRTCClientPresence2_Import(This,varStorage,fReplaceAll)	\
    ( (This)->lpVtbl -> Import(This,varStorage,fReplaceAll) ) 

#define IRTCClientPresence2_EnumerateBuddies(This,ppEnum)	\
    ( (This)->lpVtbl -> EnumerateBuddies(This,ppEnum) ) 

#define IRTCClientPresence2_get_Buddies(This,ppCollection)	\
    ( (This)->lpVtbl -> get_Buddies(This,ppCollection) ) 

#define IRTCClientPresence2_get_Buddy(This,bstrPresentityURI,ppBuddy)	\
    ( (This)->lpVtbl -> get_Buddy(This,bstrPresentityURI,ppBuddy) ) 

#define IRTCClientPresence2_AddBuddy(This,bstrPresentityURI,bstrUserName,bstrData,fPersistent,pProfile,lFlags,ppBuddy)	\
    ( (This)->lpVtbl -> AddBuddy(This,bstrPresentityURI,bstrUserName,bstrData,fPersistent,pProfile,lFlags,ppBuddy) ) 

#define IRTCClientPresence2_RemoveBuddy(This,pBuddy)	\
    ( (This)->lpVtbl -> RemoveBuddy(This,pBuddy) ) 

#define IRTCClientPresence2_EnumerateWatchers(This,ppEnum)	\
    ( (This)->lpVtbl -> EnumerateWatchers(This,ppEnum) ) 

#define IRTCClientPresence2_get_Watchers(This,ppCollection)	\
    ( (This)->lpVtbl -> get_Watchers(This,ppCollection) ) 

#define IRTCClientPresence2_get_Watcher(This,bstrPresentityURI,ppWatcher)	\
    ( (This)->lpVtbl -> get_Watcher(This,bstrPresentityURI,ppWatcher) ) 

#define IRTCClientPresence2_AddWatcher(This,bstrPresentityURI,bstrUserName,bstrData,fBlocked,fPersistent,ppWatcher)	\
    ( (This)->lpVtbl -> AddWatcher(This,bstrPresentityURI,bstrUserName,bstrData,fBlocked,fPersistent,ppWatcher) ) 

#define IRTCClientPresence2_RemoveWatcher(This,pWatcher)	\
    ( (This)->lpVtbl -> RemoveWatcher(This,pWatcher) ) 

#define IRTCClientPresence2_SetLocalPresenceInfo(This,enStatus,bstrNotes)	\
    ( (This)->lpVtbl -> SetLocalPresenceInfo(This,enStatus,bstrNotes) ) 

#define IRTCClientPresence2_get_OfferWatcherMode(This,penMode)	\
    ( (This)->lpVtbl -> get_OfferWatcherMode(This,penMode) ) 

#define IRTCClientPresence2_put_OfferWatcherMode(This,enMode)	\
    ( (This)->lpVtbl -> put_OfferWatcherMode(This,enMode) ) 

#define IRTCClientPresence2_get_PrivacyMode(This,penMode)	\
    ( (This)->lpVtbl -> get_PrivacyMode(This,penMode) ) 

#define IRTCClientPresence2_put_PrivacyMode(This,enMode)	\
    ( (This)->lpVtbl -> put_PrivacyMode(This,enMode) ) 


#define IRTCClientPresence2_EnablePresenceEx(This,pProfile,varStorage,lFlags)	\
    ( (This)->lpVtbl -> EnablePresenceEx(This,pProfile,varStorage,lFlags) ) 

#define IRTCClientPresence2_DisablePresence(This)	\
    ( (This)->lpVtbl -> DisablePresence(This) ) 

#define IRTCClientPresence2_AddGroup(This,bstrGroupName,bstrData,pProfile,lFlags,ppGroup)	\
    ( (This)->lpVtbl -> AddGroup(This,bstrGroupName,bstrData,pProfile,lFlags,ppGroup) ) 

#define IRTCClientPresence2_RemoveGroup(This,pGroup)	\
    ( (This)->lpVtbl -> RemoveGroup(This,pGroup) ) 

#define IRTCClientPresence2_EnumerateGroups(This,ppEnum)	\
    ( (This)->lpVtbl -> EnumerateGroups(This,ppEnum) ) 

#define IRTCClientPresence2_get_Groups(This,ppCollection)	\
    ( (This)->lpVtbl -> get_Groups(This,ppCollection) ) 

#define IRTCClientPresence2_get_Group(This,bstrGroupName,ppGroup)	\
    ( (This)->lpVtbl -> get_Group(This,bstrGroupName,ppGroup) ) 

#define IRTCClientPresence2_AddWatcherEx(This,bstrPresentityURI,bstrUserName,bstrData,enState,fPersistent,enScope,pProfile,lFlags,ppWatcher)	\
    ( (This)->lpVtbl -> AddWatcherEx(This,bstrPresentityURI,bstrUserName,bstrData,enState,fPersistent,enScope,pProfile,lFlags,ppWatcher) ) 

#define IRTCClientPresence2_get_WatcherEx(This,enMode,bstrPresentityURI,ppWatcher)	\
    ( (This)->lpVtbl -> get_WatcherEx(This,enMode,bstrPresentityURI,ppWatcher) ) 

#define IRTCClientPresence2_put_PresenceProperty(This,enProperty,bstrProperty)	\
    ( (This)->lpVtbl -> put_PresenceProperty(This,enProperty,bstrProperty) ) 

#define IRTCClientPresence2_get_PresenceProperty(This,enProperty,pbstrProperty)	\
    ( (This)->lpVtbl -> get_PresenceProperty(This,enProperty,pbstrProperty) ) 

#define IRTCClientPresence2_SetPresenceData(This,bstrNamespace,bstrData)	\
    ( (This)->lpVtbl -> SetPresenceData(This,bstrNamespace,bstrData) ) 

#define IRTCClientPresence2_GetPresenceData(This,pbstrNamespace,pbstrData)	\
    ( (This)->lpVtbl -> GetPresenceData(This,pbstrNamespace,pbstrData) ) 

#define IRTCClientPresence2_GetLocalPresenceInfo(This,penStatus,pbstrNotes)	\
    ( (This)->lpVtbl -> GetLocalPresenceInfo(This,penStatus,pbstrNotes) ) 

#define IRTCClientPresence2_AddBuddyEx(This,bstrPresentityURI,bstrUserName,bstrData,fPersistent,enSubscriptionType,pProfile,lFlags,ppBuddy)	\
    ( (This)->lpVtbl -> AddBuddyEx(This,bstrPresentityURI,bstrUserName,bstrData,fPersistent,enSubscriptionType,pProfile,lFlags,ppBuddy) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IRTCClientPresence2_INTERFACE_DEFINED__ */


#ifndef __IRTCClientProvisioning_INTERFACE_DEFINED__
#define __IRTCClientProvisioning_INTERFACE_DEFINED__

/* interface IRTCClientProvisioning */
/* [unique][helpstring][uuid][object] */ 


EXTERN_C const IID IID_IRTCClientProvisioning;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("B9F5CF06-65B9-4a80-A0E6-73CAE3EF3822")
    IRTCClientProvisioning : public IUnknown
    {
    public:
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE CreateProfile( 
            /* [in] */ __RPC__in BSTR bstrProfileXML,
            /* [retval][out] */ __RPC__deref_out_opt IRTCProfile **ppProfile) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE EnableProfile( 
            /* [in] */ __RPC__in_opt IRTCProfile *pProfile,
            /* [in] */ long lRegisterFlags) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE DisableProfile( 
            /* [in] */ __RPC__in_opt IRTCProfile *pProfile) = 0;
        
        virtual /* [helpstring][hidden] */ HRESULT STDMETHODCALLTYPE EnumerateProfiles( 
            /* [retval][out] */ __RPC__deref_out_opt IRTCEnumProfiles **ppEnum) = 0;
        
        virtual /* [helpstring][propget] */ HRESULT STDMETHODCALLTYPE get_Profiles( 
            /* [retval][out] */ __RPC__deref_out_opt IRTCCollection **ppCollection) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE GetProfile( 
            /* [in] */ __RPC__in BSTR bstrUserAccount,
            /* [in] */ __RPC__in BSTR bstrUserPassword,
            /* [in] */ __RPC__in BSTR bstrUserURI,
            /* [in] */ __RPC__in BSTR bstrServer,
            /* [in] */ long lTransport,
            /* [in] */ LONG_PTR lCookie) = 0;
        
        virtual /* [helpstring][propget] */ HRESULT STDMETHODCALLTYPE get_SessionCapabilities( 
            /* [retval][out] */ __RPC__out long *plSupportedSessions) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IRTCClientProvisioningVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IRTCClientProvisioning * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IRTCClientProvisioning * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IRTCClientProvisioning * This);
        
        DECLSPEC_XFGVIRT(IRTCClientProvisioning, CreateProfile)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *CreateProfile )( 
            __RPC__in IRTCClientProvisioning * This,
            /* [in] */ __RPC__in BSTR bstrProfileXML,
            /* [retval][out] */ __RPC__deref_out_opt IRTCProfile **ppProfile);
        
        DECLSPEC_XFGVIRT(IRTCClientProvisioning, EnableProfile)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *EnableProfile )( 
            __RPC__in IRTCClientProvisioning * This,
            /* [in] */ __RPC__in_opt IRTCProfile *pProfile,
            /* [in] */ long lRegisterFlags);
        
        DECLSPEC_XFGVIRT(IRTCClientProvisioning, DisableProfile)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *DisableProfile )( 
            __RPC__in IRTCClientProvisioning * This,
            /* [in] */ __RPC__in_opt IRTCProfile *pProfile);
        
        DECLSPEC_XFGVIRT(IRTCClientProvisioning, EnumerateProfiles)
        /* [helpstring][hidden] */ HRESULT ( STDMETHODCALLTYPE *EnumerateProfiles )( 
            __RPC__in IRTCClientProvisioning * This,
            /* [retval][out] */ __RPC__deref_out_opt IRTCEnumProfiles **ppEnum);
        
        DECLSPEC_XFGVIRT(IRTCClientProvisioning, get_Profiles)
        /* [helpstring][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Profiles )( 
            __RPC__in IRTCClientProvisioning * This,
            /* [retval][out] */ __RPC__deref_out_opt IRTCCollection **ppCollection);
        
        DECLSPEC_XFGVIRT(IRTCClientProvisioning, GetProfile)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *GetProfile )( 
            __RPC__in IRTCClientProvisioning * This,
            /* [in] */ __RPC__in BSTR bstrUserAccount,
            /* [in] */ __RPC__in BSTR bstrUserPassword,
            /* [in] */ __RPC__in BSTR bstrUserURI,
            /* [in] */ __RPC__in BSTR bstrServer,
            /* [in] */ long lTransport,
            /* [in] */ LONG_PTR lCookie);
        
        DECLSPEC_XFGVIRT(IRTCClientProvisioning, get_SessionCapabilities)
        /* [helpstring][propget] */ HRESULT ( STDMETHODCALLTYPE *get_SessionCapabilities )( 
            __RPC__in IRTCClientProvisioning * This,
            /* [retval][out] */ __RPC__out long *plSupportedSessions);
        
        END_INTERFACE
    } IRTCClientProvisioningVtbl;

    interface IRTCClientProvisioning
    {
        CONST_VTBL struct IRTCClientProvisioningVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IRTCClientProvisioning_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IRTCClientProvisioning_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IRTCClientProvisioning_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IRTCClientProvisioning_CreateProfile(This,bstrProfileXML,ppProfile)	\
    ( (This)->lpVtbl -> CreateProfile(This,bstrProfileXML,ppProfile) ) 

#define IRTCClientProvisioning_EnableProfile(This,pProfile,lRegisterFlags)	\
    ( (This)->lpVtbl -> EnableProfile(This,pProfile,lRegisterFlags) ) 

#define IRTCClientProvisioning_DisableProfile(This,pProfile)	\
    ( (This)->lpVtbl -> DisableProfile(This,pProfile) ) 

#define IRTCClientProvisioning_EnumerateProfiles(This,ppEnum)	\
    ( (This)->lpVtbl -> EnumerateProfiles(This,ppEnum) ) 

#define IRTCClientProvisioning_get_Profiles(This,ppCollection)	\
    ( (This)->lpVtbl -> get_Profiles(This,ppCollection) ) 

#define IRTCClientProvisioning_GetProfile(This,bstrUserAccount,bstrUserPassword,bstrUserURI,bstrServer,lTransport,lCookie)	\
    ( (This)->lpVtbl -> GetProfile(This,bstrUserAccount,bstrUserPassword,bstrUserURI,bstrServer,lTransport,lCookie) ) 

#define IRTCClientProvisioning_get_SessionCapabilities(This,plSupportedSessions)	\
    ( (This)->lpVtbl -> get_SessionCapabilities(This,plSupportedSessions) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IRTCClientProvisioning_INTERFACE_DEFINED__ */


#ifndef __IRTCClientProvisioning2_INTERFACE_DEFINED__
#define __IRTCClientProvisioning2_INTERFACE_DEFINED__

/* interface IRTCClientProvisioning2 */
/* [unique][helpstring][uuid][object] */ 


EXTERN_C const IID IID_IRTCClientProvisioning2;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("A70909B5-F40E-4587-BB75-E6BC0845023E")
    IRTCClientProvisioning2 : public IRTCClientProvisioning
    {
    public:
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE EnableProfileEx( 
            /* [in] */ __RPC__in_opt IRTCProfile *pProfile,
            /* [in] */ long lRegisterFlags,
            /* [in] */ long lRoamingFlags) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IRTCClientProvisioning2Vtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IRTCClientProvisioning2 * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IRTCClientProvisioning2 * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IRTCClientProvisioning2 * This);
        
        DECLSPEC_XFGVIRT(IRTCClientProvisioning, CreateProfile)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *CreateProfile )( 
            __RPC__in IRTCClientProvisioning2 * This,
            /* [in] */ __RPC__in BSTR bstrProfileXML,
            /* [retval][out] */ __RPC__deref_out_opt IRTCProfile **ppProfile);
        
        DECLSPEC_XFGVIRT(IRTCClientProvisioning, EnableProfile)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *EnableProfile )( 
            __RPC__in IRTCClientProvisioning2 * This,
            /* [in] */ __RPC__in_opt IRTCProfile *pProfile,
            /* [in] */ long lRegisterFlags);
        
        DECLSPEC_XFGVIRT(IRTCClientProvisioning, DisableProfile)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *DisableProfile )( 
            __RPC__in IRTCClientProvisioning2 * This,
            /* [in] */ __RPC__in_opt IRTCProfile *pProfile);
        
        DECLSPEC_XFGVIRT(IRTCClientProvisioning, EnumerateProfiles)
        /* [helpstring][hidden] */ HRESULT ( STDMETHODCALLTYPE *EnumerateProfiles )( 
            __RPC__in IRTCClientProvisioning2 * This,
            /* [retval][out] */ __RPC__deref_out_opt IRTCEnumProfiles **ppEnum);
        
        DECLSPEC_XFGVIRT(IRTCClientProvisioning, get_Profiles)
        /* [helpstring][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Profiles )( 
            __RPC__in IRTCClientProvisioning2 * This,
            /* [retval][out] */ __RPC__deref_out_opt IRTCCollection **ppCollection);
        
        DECLSPEC_XFGVIRT(IRTCClientProvisioning, GetProfile)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *GetProfile )( 
            __RPC__in IRTCClientProvisioning2 * This,
            /* [in] */ __RPC__in BSTR bstrUserAccount,
            /* [in] */ __RPC__in BSTR bstrUserPassword,
            /* [in] */ __RPC__in BSTR bstrUserURI,
            /* [in] */ __RPC__in BSTR bstrServer,
            /* [in] */ long lTransport,
            /* [in] */ LONG_PTR lCookie);
        
        DECLSPEC_XFGVIRT(IRTCClientProvisioning, get_SessionCapabilities)
        /* [helpstring][propget] */ HRESULT ( STDMETHODCALLTYPE *get_SessionCapabilities )( 
            __RPC__in IRTCClientProvisioning2 * This,
            /* [retval][out] */ __RPC__out long *plSupportedSessions);
        
        DECLSPEC_XFGVIRT(IRTCClientProvisioning2, EnableProfileEx)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *EnableProfileEx )( 
            __RPC__in IRTCClientProvisioning2 * This,
            /* [in] */ __RPC__in_opt IRTCProfile *pProfile,
            /* [in] */ long lRegisterFlags,
            /* [in] */ long lRoamingFlags);
        
        END_INTERFACE
    } IRTCClientProvisioning2Vtbl;

    interface IRTCClientProvisioning2
    {
        CONST_VTBL struct IRTCClientProvisioning2Vtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IRTCClientProvisioning2_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IRTCClientProvisioning2_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IRTCClientProvisioning2_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IRTCClientProvisioning2_CreateProfile(This,bstrProfileXML,ppProfile)	\
    ( (This)->lpVtbl -> CreateProfile(This,bstrProfileXML,ppProfile) ) 

#define IRTCClientProvisioning2_EnableProfile(This,pProfile,lRegisterFlags)	\
    ( (This)->lpVtbl -> EnableProfile(This,pProfile,lRegisterFlags) ) 

#define IRTCClientProvisioning2_DisableProfile(This,pProfile)	\
    ( (This)->lpVtbl -> DisableProfile(This,pProfile) ) 

#define IRTCClientProvisioning2_EnumerateProfiles(This,ppEnum)	\
    ( (This)->lpVtbl -> EnumerateProfiles(This,ppEnum) ) 

#define IRTCClientProvisioning2_get_Profiles(This,ppCollection)	\
    ( (This)->lpVtbl -> get_Profiles(This,ppCollection) ) 

#define IRTCClientProvisioning2_GetProfile(This,bstrUserAccount,bstrUserPassword,bstrUserURI,bstrServer,lTransport,lCookie)	\
    ( (This)->lpVtbl -> GetProfile(This,bstrUserAccount,bstrUserPassword,bstrUserURI,bstrServer,lTransport,lCookie) ) 

#define IRTCClientProvisioning2_get_SessionCapabilities(This,plSupportedSessions)	\
    ( (This)->lpVtbl -> get_SessionCapabilities(This,plSupportedSessions) ) 


#define IRTCClientProvisioning2_EnableProfileEx(This,pProfile,lRegisterFlags,lRoamingFlags)	\
    ( (This)->lpVtbl -> EnableProfileEx(This,pProfile,lRegisterFlags,lRoamingFlags) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IRTCClientProvisioning2_INTERFACE_DEFINED__ */


#ifndef __IRTCProfile_INTERFACE_DEFINED__
#define __IRTCProfile_INTERFACE_DEFINED__

/* interface IRTCProfile */
/* [unique][helpstring][uuid][object] */ 


EXTERN_C const IID IID_IRTCProfile;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("d07eca9e-4062-4dd4-9e7d-722a49ba7303")
    IRTCProfile : public IUnknown
    {
    public:
        virtual /* [helpstring][propget] */ HRESULT STDMETHODCALLTYPE get_Key( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrKey) = 0;
        
        virtual /* [helpstring][propget] */ HRESULT STDMETHODCALLTYPE get_Name( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrName) = 0;
        
        virtual /* [helpstring][propget] */ HRESULT STDMETHODCALLTYPE get_XML( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrXML) = 0;
        
        virtual /* [helpstring][propget] */ HRESULT STDMETHODCALLTYPE get_ProviderName( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrName) = 0;
        
        virtual /* [helpstring][propget] */ HRESULT STDMETHODCALLTYPE get_ProviderURI( 
            /* [in] */ RTC_PROVIDER_URI enURI,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrURI) = 0;
        
        virtual /* [helpstring][propget] */ HRESULT STDMETHODCALLTYPE get_ProviderData( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrData) = 0;
        
        virtual /* [helpstring][propget] */ HRESULT STDMETHODCALLTYPE get_ClientName( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrName) = 0;
        
        virtual /* [helpstring][propget] */ HRESULT STDMETHODCALLTYPE get_ClientBanner( 
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pfBanner) = 0;
        
        virtual /* [helpstring][propget] */ HRESULT STDMETHODCALLTYPE get_ClientMinVer( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrMinVer) = 0;
        
        virtual /* [helpstring][propget] */ HRESULT STDMETHODCALLTYPE get_ClientCurVer( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrCurVer) = 0;
        
        virtual /* [helpstring][propget] */ HRESULT STDMETHODCALLTYPE get_ClientUpdateURI( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrUpdateURI) = 0;
        
        virtual /* [helpstring][propget] */ HRESULT STDMETHODCALLTYPE get_ClientData( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrData) = 0;
        
        virtual /* [helpstring][propget] */ HRESULT STDMETHODCALLTYPE get_UserURI( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrUserURI) = 0;
        
        virtual /* [helpstring][propget] */ HRESULT STDMETHODCALLTYPE get_UserName( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrUserName) = 0;
        
        virtual /* [helpstring][propget] */ HRESULT STDMETHODCALLTYPE get_UserAccount( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrUserAccount) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE SetCredentials( 
            /* [in] */ __RPC__in BSTR bstrUserURI,
            /* [in] */ __RPC__in BSTR bstrUserAccount,
            /* [in] */ __RPC__in BSTR bstrPassword) = 0;
        
        virtual /* [helpstring][propget] */ HRESULT STDMETHODCALLTYPE get_SessionCapabilities( 
            /* [retval][out] */ __RPC__out long *plSupportedSessions) = 0;
        
        virtual /* [helpstring][propget] */ HRESULT STDMETHODCALLTYPE get_State( 
            /* [retval][out] */ __RPC__out RTC_REGISTRATION_STATE *penState) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IRTCProfileVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IRTCProfile * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IRTCProfile * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IRTCProfile * This);
        
        DECLSPEC_XFGVIRT(IRTCProfile, get_Key)
        /* [helpstring][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Key )( 
            __RPC__in IRTCProfile * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrKey);
        
        DECLSPEC_XFGVIRT(IRTCProfile, get_Name)
        /* [helpstring][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Name )( 
            __RPC__in IRTCProfile * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrName);
        
        DECLSPEC_XFGVIRT(IRTCProfile, get_XML)
        /* [helpstring][propget] */ HRESULT ( STDMETHODCALLTYPE *get_XML )( 
            __RPC__in IRTCProfile * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrXML);
        
        DECLSPEC_XFGVIRT(IRTCProfile, get_ProviderName)
        /* [helpstring][propget] */ HRESULT ( STDMETHODCALLTYPE *get_ProviderName )( 
            __RPC__in IRTCProfile * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrName);
        
        DECLSPEC_XFGVIRT(IRTCProfile, get_ProviderURI)
        /* [helpstring][propget] */ HRESULT ( STDMETHODCALLTYPE *get_ProviderURI )( 
            __RPC__in IRTCProfile * This,
            /* [in] */ RTC_PROVIDER_URI enURI,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrURI);
        
        DECLSPEC_XFGVIRT(IRTCProfile, get_ProviderData)
        /* [helpstring][propget] */ HRESULT ( STDMETHODCALLTYPE *get_ProviderData )( 
            __RPC__in IRTCProfile * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrData);
        
        DECLSPEC_XFGVIRT(IRTCProfile, get_ClientName)
        /* [helpstring][propget] */ HRESULT ( STDMETHODCALLTYPE *get_ClientName )( 
            __RPC__in IRTCProfile * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrName);
        
        DECLSPEC_XFGVIRT(IRTCProfile, get_ClientBanner)
        /* [helpstring][propget] */ HRESULT ( STDMETHODCALLTYPE *get_ClientBanner )( 
            __RPC__in IRTCProfile * This,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pfBanner);
        
        DECLSPEC_XFGVIRT(IRTCProfile, get_ClientMinVer)
        /* [helpstring][propget] */ HRESULT ( STDMETHODCALLTYPE *get_ClientMinVer )( 
            __RPC__in IRTCProfile * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrMinVer);
        
        DECLSPEC_XFGVIRT(IRTCProfile, get_ClientCurVer)
        /* [helpstring][propget] */ HRESULT ( STDMETHODCALLTYPE *get_ClientCurVer )( 
            __RPC__in IRTCProfile * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrCurVer);
        
        DECLSPEC_XFGVIRT(IRTCProfile, get_ClientUpdateURI)
        /* [helpstring][propget] */ HRESULT ( STDMETHODCALLTYPE *get_ClientUpdateURI )( 
            __RPC__in IRTCProfile * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrUpdateURI);
        
        DECLSPEC_XFGVIRT(IRTCProfile, get_ClientData)
        /* [helpstring][propget] */ HRESULT ( STDMETHODCALLTYPE *get_ClientData )( 
            __RPC__in IRTCProfile * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrData);
        
        DECLSPEC_XFGVIRT(IRTCProfile, get_UserURI)
        /* [helpstring][propget] */ HRESULT ( STDMETHODCALLTYPE *get_UserURI )( 
            __RPC__in IRTCProfile * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrUserURI);
        
        DECLSPEC_XFGVIRT(IRTCProfile, get_UserName)
        /* [helpstring][propget] */ HRESULT ( STDMETHODCALLTYPE *get_UserName )( 
            __RPC__in IRTCProfile * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrUserName);
        
        DECLSPEC_XFGVIRT(IRTCProfile, get_UserAccount)
        /* [helpstring][propget] */ HRESULT ( STDMETHODCALLTYPE *get_UserAccount )( 
            __RPC__in IRTCProfile * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrUserAccount);
        
        DECLSPEC_XFGVIRT(IRTCProfile, SetCredentials)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *SetCredentials )( 
            __RPC__in IRTCProfile * This,
            /* [in] */ __RPC__in BSTR bstrUserURI,
            /* [in] */ __RPC__in BSTR bstrUserAccount,
            /* [in] */ __RPC__in BSTR bstrPassword);
        
        DECLSPEC_XFGVIRT(IRTCProfile, get_SessionCapabilities)
        /* [helpstring][propget] */ HRESULT ( STDMETHODCALLTYPE *get_SessionCapabilities )( 
            __RPC__in IRTCProfile * This,
            /* [retval][out] */ __RPC__out long *plSupportedSessions);
        
        DECLSPEC_XFGVIRT(IRTCProfile, get_State)
        /* [helpstring][propget] */ HRESULT ( STDMETHODCALLTYPE *get_State )( 
            __RPC__in IRTCProfile * This,
            /* [retval][out] */ __RPC__out RTC_REGISTRATION_STATE *penState);
        
        END_INTERFACE
    } IRTCProfileVtbl;

    interface IRTCProfile
    {
        CONST_VTBL struct IRTCProfileVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IRTCProfile_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IRTCProfile_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IRTCProfile_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IRTCProfile_get_Key(This,pbstrKey)	\
    ( (This)->lpVtbl -> get_Key(This,pbstrKey) ) 

#define IRTCProfile_get_Name(This,pbstrName)	\
    ( (This)->lpVtbl -> get_Name(This,pbstrName) ) 

#define IRTCProfile_get_XML(This,pbstrXML)	\
    ( (This)->lpVtbl -> get_XML(This,pbstrXML) ) 

#define IRTCProfile_get_ProviderName(This,pbstrName)	\
    ( (This)->lpVtbl -> get_ProviderName(This,pbstrName) ) 

#define IRTCProfile_get_ProviderURI(This,enURI,pbstrURI)	\
    ( (This)->lpVtbl -> get_ProviderURI(This,enURI,pbstrURI) ) 

#define IRTCProfile_get_ProviderData(This,pbstrData)	\
    ( (This)->lpVtbl -> get_ProviderData(This,pbstrData) ) 

#define IRTCProfile_get_ClientName(This,pbstrName)	\
    ( (This)->lpVtbl -> get_ClientName(This,pbstrName) ) 

#define IRTCProfile_get_ClientBanner(This,pfBanner)	\
    ( (This)->lpVtbl -> get_ClientBanner(This,pfBanner) ) 

#define IRTCProfile_get_ClientMinVer(This,pbstrMinVer)	\
    ( (This)->lpVtbl -> get_ClientMinVer(This,pbstrMinVer) ) 

#define IRTCProfile_get_ClientCurVer(This,pbstrCurVer)	\
    ( (This)->lpVtbl -> get_ClientCurVer(This,pbstrCurVer) ) 

#define IRTCProfile_get_ClientUpdateURI(This,pbstrUpdateURI)	\
    ( (This)->lpVtbl -> get_ClientUpdateURI(This,pbstrUpdateURI) ) 

#define IRTCProfile_get_ClientData(This,pbstrData)	\
    ( (This)->lpVtbl -> get_ClientData(This,pbstrData) ) 

#define IRTCProfile_get_UserURI(This,pbstrUserURI)	\
    ( (This)->lpVtbl -> get_UserURI(This,pbstrUserURI) ) 

#define IRTCProfile_get_UserName(This,pbstrUserName)	\
    ( (This)->lpVtbl -> get_UserName(This,pbstrUserName) ) 

#define IRTCProfile_get_UserAccount(This,pbstrUserAccount)	\
    ( (This)->lpVtbl -> get_UserAccount(This,pbstrUserAccount) ) 

#define IRTCProfile_SetCredentials(This,bstrUserURI,bstrUserAccount,bstrPassword)	\
    ( (This)->lpVtbl -> SetCredentials(This,bstrUserURI,bstrUserAccount,bstrPassword) ) 

#define IRTCProfile_get_SessionCapabilities(This,plSupportedSessions)	\
    ( (This)->lpVtbl -> get_SessionCapabilities(This,plSupportedSessions) ) 

#define IRTCProfile_get_State(This,penState)	\
    ( (This)->lpVtbl -> get_State(This,penState) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IRTCProfile_INTERFACE_DEFINED__ */


#ifndef __IRTCProfile2_INTERFACE_DEFINED__
#define __IRTCProfile2_INTERFACE_DEFINED__

/* interface IRTCProfile2 */
/* [unique][helpstring][uuid][object] */ 


EXTERN_C const IID IID_IRTCProfile2;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("4B81F84E-BDC7-4184-9154-3CB2DD7917FB")
    IRTCProfile2 : public IRTCProfile
    {
    public:
        virtual /* [helpstring][propget] */ HRESULT STDMETHODCALLTYPE get_Realm( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrRealm) = 0;
        
        virtual /* [helpstring][propput] */ HRESULT STDMETHODCALLTYPE put_Realm( 
            /* [in] */ __RPC__in BSTR bstrRealm) = 0;
        
        virtual /* [helpstring][propget] */ HRESULT STDMETHODCALLTYPE get_AllowedAuth( 
            /* [retval][out] */ __RPC__out long *plAllowedAuth) = 0;
        
        virtual /* [helpstring][propput] */ HRESULT STDMETHODCALLTYPE put_AllowedAuth( 
            /* [in] */ long lAllowedAuth) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IRTCProfile2Vtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IRTCProfile2 * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IRTCProfile2 * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IRTCProfile2 * This);
        
        DECLSPEC_XFGVIRT(IRTCProfile, get_Key)
        /* [helpstring][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Key )( 
            __RPC__in IRTCProfile2 * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrKey);
        
        DECLSPEC_XFGVIRT(IRTCProfile, get_Name)
        /* [helpstring][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Name )( 
            __RPC__in IRTCProfile2 * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrName);
        
        DECLSPEC_XFGVIRT(IRTCProfile, get_XML)
        /* [helpstring][propget] */ HRESULT ( STDMETHODCALLTYPE *get_XML )( 
            __RPC__in IRTCProfile2 * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrXML);
        
        DECLSPEC_XFGVIRT(IRTCProfile, get_ProviderName)
        /* [helpstring][propget] */ HRESULT ( STDMETHODCALLTYPE *get_ProviderName )( 
            __RPC__in IRTCProfile2 * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrName);
        
        DECLSPEC_XFGVIRT(IRTCProfile, get_ProviderURI)
        /* [helpstring][propget] */ HRESULT ( STDMETHODCALLTYPE *get_ProviderURI )( 
            __RPC__in IRTCProfile2 * This,
            /* [in] */ RTC_PROVIDER_URI enURI,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrURI);
        
        DECLSPEC_XFGVIRT(IRTCProfile, get_ProviderData)
        /* [helpstring][propget] */ HRESULT ( STDMETHODCALLTYPE *get_ProviderData )( 
            __RPC__in IRTCProfile2 * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrData);
        
        DECLSPEC_XFGVIRT(IRTCProfile, get_ClientName)
        /* [helpstring][propget] */ HRESULT ( STDMETHODCALLTYPE *get_ClientName )( 
            __RPC__in IRTCProfile2 * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrName);
        
        DECLSPEC_XFGVIRT(IRTCProfile, get_ClientBanner)
        /* [helpstring][propget] */ HRESULT ( STDMETHODCALLTYPE *get_ClientBanner )( 
            __RPC__in IRTCProfile2 * This,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pfBanner);
        
        DECLSPEC_XFGVIRT(IRTCProfile, get_ClientMinVer)
        /* [helpstring][propget] */ HRESULT ( STDMETHODCALLTYPE *get_ClientMinVer )( 
            __RPC__in IRTCProfile2 * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrMinVer);
        
        DECLSPEC_XFGVIRT(IRTCProfile, get_ClientCurVer)
        /* [helpstring][propget] */ HRESULT ( STDMETHODCALLTYPE *get_ClientCurVer )( 
            __RPC__in IRTCProfile2 * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrCurVer);
        
        DECLSPEC_XFGVIRT(IRTCProfile, get_ClientUpdateURI)
        /* [helpstring][propget] */ HRESULT ( STDMETHODCALLTYPE *get_ClientUpdateURI )( 
            __RPC__in IRTCProfile2 * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrUpdateURI);
        
        DECLSPEC_XFGVIRT(IRTCProfile, get_ClientData)
        /* [helpstring][propget] */ HRESULT ( STDMETHODCALLTYPE *get_ClientData )( 
            __RPC__in IRTCProfile2 * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrData);
        
        DECLSPEC_XFGVIRT(IRTCProfile, get_UserURI)
        /* [helpstring][propget] */ HRESULT ( STDMETHODCALLTYPE *get_UserURI )( 
            __RPC__in IRTCProfile2 * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrUserURI);
        
        DECLSPEC_XFGVIRT(IRTCProfile, get_UserName)
        /* [helpstring][propget] */ HRESULT ( STDMETHODCALLTYPE *get_UserName )( 
            __RPC__in IRTCProfile2 * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrUserName);
        
        DECLSPEC_XFGVIRT(IRTCProfile, get_UserAccount)
        /* [helpstring][propget] */ HRESULT ( STDMETHODCALLTYPE *get_UserAccount )( 
            __RPC__in IRTCProfile2 * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrUserAccount);
        
        DECLSPEC_XFGVIRT(IRTCProfile, SetCredentials)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *SetCredentials )( 
            __RPC__in IRTCProfile2 * This,
            /* [in] */ __RPC__in BSTR bstrUserURI,
            /* [in] */ __RPC__in BSTR bstrUserAccount,
            /* [in] */ __RPC__in BSTR bstrPassword);
        
        DECLSPEC_XFGVIRT(IRTCProfile, get_SessionCapabilities)
        /* [helpstring][propget] */ HRESULT ( STDMETHODCALLTYPE *get_SessionCapabilities )( 
            __RPC__in IRTCProfile2 * This,
            /* [retval][out] */ __RPC__out long *plSupportedSessions);
        
        DECLSPEC_XFGVIRT(IRTCProfile, get_State)
        /* [helpstring][propget] */ HRESULT ( STDMETHODCALLTYPE *get_State )( 
            __RPC__in IRTCProfile2 * This,
            /* [retval][out] */ __RPC__out RTC_REGISTRATION_STATE *penState);
        
        DECLSPEC_XFGVIRT(IRTCProfile2, get_Realm)
        /* [helpstring][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Realm )( 
            __RPC__in IRTCProfile2 * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrRealm);
        
        DECLSPEC_XFGVIRT(IRTCProfile2, put_Realm)
        /* [helpstring][propput] */ HRESULT ( STDMETHODCALLTYPE *put_Realm )( 
            __RPC__in IRTCProfile2 * This,
            /* [in] */ __RPC__in BSTR bstrRealm);
        
        DECLSPEC_XFGVIRT(IRTCProfile2, get_AllowedAuth)
        /* [helpstring][propget] */ HRESULT ( STDMETHODCALLTYPE *get_AllowedAuth )( 
            __RPC__in IRTCProfile2 * This,
            /* [retval][out] */ __RPC__out long *plAllowedAuth);
        
        DECLSPEC_XFGVIRT(IRTCProfile2, put_AllowedAuth)
        /* [helpstring][propput] */ HRESULT ( STDMETHODCALLTYPE *put_AllowedAuth )( 
            __RPC__in IRTCProfile2 * This,
            /* [in] */ long lAllowedAuth);
        
        END_INTERFACE
    } IRTCProfile2Vtbl;

    interface IRTCProfile2
    {
        CONST_VTBL struct IRTCProfile2Vtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IRTCProfile2_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IRTCProfile2_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IRTCProfile2_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IRTCProfile2_get_Key(This,pbstrKey)	\
    ( (This)->lpVtbl -> get_Key(This,pbstrKey) ) 

#define IRTCProfile2_get_Name(This,pbstrName)	\
    ( (This)->lpVtbl -> get_Name(This,pbstrName) ) 

#define IRTCProfile2_get_XML(This,pbstrXML)	\
    ( (This)->lpVtbl -> get_XML(This,pbstrXML) ) 

#define IRTCProfile2_get_ProviderName(This,pbstrName)	\
    ( (This)->lpVtbl -> get_ProviderName(This,pbstrName) ) 

#define IRTCProfile2_get_ProviderURI(This,enURI,pbstrURI)	\
    ( (This)->lpVtbl -> get_ProviderURI(This,enURI,pbstrURI) ) 

#define IRTCProfile2_get_ProviderData(This,pbstrData)	\
    ( (This)->lpVtbl -> get_ProviderData(This,pbstrData) ) 

#define IRTCProfile2_get_ClientName(This,pbstrName)	\
    ( (This)->lpVtbl -> get_ClientName(This,pbstrName) ) 

#define IRTCProfile2_get_ClientBanner(This,pfBanner)	\
    ( (This)->lpVtbl -> get_ClientBanner(This,pfBanner) ) 

#define IRTCProfile2_get_ClientMinVer(This,pbstrMinVer)	\
    ( (This)->lpVtbl -> get_ClientMinVer(This,pbstrMinVer) ) 

#define IRTCProfile2_get_ClientCurVer(This,pbstrCurVer)	\
    ( (This)->lpVtbl -> get_ClientCurVer(This,pbstrCurVer) ) 

#define IRTCProfile2_get_ClientUpdateURI(This,pbstrUpdateURI)	\
    ( (This)->lpVtbl -> get_ClientUpdateURI(This,pbstrUpdateURI) ) 

#define IRTCProfile2_get_ClientData(This,pbstrData)	\
    ( (This)->lpVtbl -> get_ClientData(This,pbstrData) ) 

#define IRTCProfile2_get_UserURI(This,pbstrUserURI)	\
    ( (This)->lpVtbl -> get_UserURI(This,pbstrUserURI) ) 

#define IRTCProfile2_get_UserName(This,pbstrUserName)	\
    ( (This)->lpVtbl -> get_UserName(This,pbstrUserName) ) 

#define IRTCProfile2_get_UserAccount(This,pbstrUserAccount)	\
    ( (This)->lpVtbl -> get_UserAccount(This,pbstrUserAccount) ) 

#define IRTCProfile2_SetCredentials(This,bstrUserURI,bstrUserAccount,bstrPassword)	\
    ( (This)->lpVtbl -> SetCredentials(This,bstrUserURI,bstrUserAccount,bstrPassword) ) 

#define IRTCProfile2_get_SessionCapabilities(This,plSupportedSessions)	\
    ( (This)->lpVtbl -> get_SessionCapabilities(This,plSupportedSessions) ) 

#define IRTCProfile2_get_State(This,penState)	\
    ( (This)->lpVtbl -> get_State(This,penState) ) 


#define IRTCProfile2_get_Realm(This,pbstrRealm)	\
    ( (This)->lpVtbl -> get_Realm(This,pbstrRealm) ) 

#define IRTCProfile2_put_Realm(This,bstrRealm)	\
    ( (This)->lpVtbl -> put_Realm(This,bstrRealm) ) 

#define IRTCProfile2_get_AllowedAuth(This,plAllowedAuth)	\
    ( (This)->lpVtbl -> get_AllowedAuth(This,plAllowedAuth) ) 

#define IRTCProfile2_put_AllowedAuth(This,lAllowedAuth)	\
    ( (This)->lpVtbl -> put_AllowedAuth(This,lAllowedAuth) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IRTCProfile2_INTERFACE_DEFINED__ */


#ifndef __IRTCSession_INTERFACE_DEFINED__
#define __IRTCSession_INTERFACE_DEFINED__

/* interface IRTCSession */
/* [unique][helpstring][uuid][object] */ 


EXTERN_C const IID IID_IRTCSession;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("387c8086-99be-42fb-9973-7c0fc0ca9fa8")
    IRTCSession : public IUnknown
    {
    public:
        virtual /* [helpstring][propget] */ HRESULT STDMETHODCALLTYPE get_Client( 
            /* [retval][out] */ __RPC__deref_out_opt IRTCClient **ppClient) = 0;
        
        virtual /* [helpstring][propget] */ HRESULT STDMETHODCALLTYPE get_State( 
            /* [retval][out] */ __RPC__out RTC_SESSION_STATE *penState) = 0;
        
        virtual /* [helpstring][propget] */ HRESULT STDMETHODCALLTYPE get_Type( 
            /* [retval][out] */ __RPC__out RTC_SESSION_TYPE *penType) = 0;
        
        virtual /* [helpstring][propget] */ HRESULT STDMETHODCALLTYPE get_Profile( 
            /* [retval][out] */ __RPC__deref_out_opt IRTCProfile **ppProfile) = 0;
        
        virtual /* [helpstring][propget] */ HRESULT STDMETHODCALLTYPE get_Participants( 
            /* [retval][out] */ __RPC__deref_out_opt IRTCCollection **ppCollection) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE Answer( void) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE Terminate( 
            /* [in] */ RTC_TERMINATE_REASON enReason) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE Redirect( 
            /* [in] */ RTC_SESSION_TYPE enType,
            /* [in] */ __RPC__in BSTR bstrLocalPhoneURI,
            /* [in] */ __RPC__in_opt IRTCProfile *pProfile,
            /* [in] */ long lFlags) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE AddParticipant( 
            /* [in] */ __RPC__in BSTR bstrAddress,
            /* [in] */ __RPC__in BSTR bstrName,
            /* [retval][out] */ __RPC__deref_out_opt IRTCParticipant **ppParticipant) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE RemoveParticipant( 
            /* [in] */ __RPC__in_opt IRTCParticipant *pParticipant) = 0;
        
        virtual /* [helpstring][hidden] */ HRESULT STDMETHODCALLTYPE EnumerateParticipants( 
            /* [retval][out] */ __RPC__deref_out_opt IRTCEnumParticipants **ppEnum) = 0;
        
        virtual /* [helpstring][propget] */ HRESULT STDMETHODCALLTYPE get_CanAddParticipants( 
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pfCanAdd) = 0;
        
        virtual /* [helpstring][propget] */ HRESULT STDMETHODCALLTYPE get_RedirectedUserURI( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrUserURI) = 0;
        
        virtual /* [helpstring][propget] */ HRESULT STDMETHODCALLTYPE get_RedirectedUserName( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrUserName) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE NextRedirectedUser( void) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE SendMessage( 
            /* [in] */ __RPC__in BSTR bstrMessageHeader,
            /* [in] */ __RPC__in BSTR bstrMessage,
            /* [in] */ LONG_PTR lCookie) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE SendMessageStatus( 
            /* [in] */ RTC_MESSAGING_USER_STATUS enUserStatus,
            /* [in] */ LONG_PTR lCookie) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE AddStream( 
            /* [in] */ long lMediaType,
            /* [in] */ LONG_PTR lCookie) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE RemoveStream( 
            /* [in] */ long lMediaType,
            /* [in] */ LONG_PTR lCookie) = 0;
        
        virtual /* [helpstring][propput] */ HRESULT STDMETHODCALLTYPE put_EncryptionKey( 
            /* [in] */ long lMediaType,
            /* [in] */ __RPC__in BSTR EncryptionKey) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IRTCSessionVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IRTCSession * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IRTCSession * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IRTCSession * This);
        
        DECLSPEC_XFGVIRT(IRTCSession, get_Client)
        /* [helpstring][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Client )( 
            __RPC__in IRTCSession * This,
            /* [retval][out] */ __RPC__deref_out_opt IRTCClient **ppClient);
        
        DECLSPEC_XFGVIRT(IRTCSession, get_State)
        /* [helpstring][propget] */ HRESULT ( STDMETHODCALLTYPE *get_State )( 
            __RPC__in IRTCSession * This,
            /* [retval][out] */ __RPC__out RTC_SESSION_STATE *penState);
        
        DECLSPEC_XFGVIRT(IRTCSession, get_Type)
        /* [helpstring][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Type )( 
            __RPC__in IRTCSession * This,
            /* [retval][out] */ __RPC__out RTC_SESSION_TYPE *penType);
        
        DECLSPEC_XFGVIRT(IRTCSession, get_Profile)
        /* [helpstring][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Profile )( 
            __RPC__in IRTCSession * This,
            /* [retval][out] */ __RPC__deref_out_opt IRTCProfile **ppProfile);
        
        DECLSPEC_XFGVIRT(IRTCSession, get_Participants)
        /* [helpstring][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Participants )( 
            __RPC__in IRTCSession * This,
            /* [retval][out] */ __RPC__deref_out_opt IRTCCollection **ppCollection);
        
        DECLSPEC_XFGVIRT(IRTCSession, Answer)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *Answer )( 
            __RPC__in IRTCSession * This);
        
        DECLSPEC_XFGVIRT(IRTCSession, Terminate)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *Terminate )( 
            __RPC__in IRTCSession * This,
            /* [in] */ RTC_TERMINATE_REASON enReason);
        
        DECLSPEC_XFGVIRT(IRTCSession, Redirect)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *Redirect )( 
            __RPC__in IRTCSession * This,
            /* [in] */ RTC_SESSION_TYPE enType,
            /* [in] */ __RPC__in BSTR bstrLocalPhoneURI,
            /* [in] */ __RPC__in_opt IRTCProfile *pProfile,
            /* [in] */ long lFlags);
        
        DECLSPEC_XFGVIRT(IRTCSession, AddParticipant)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *AddParticipant )( 
            __RPC__in IRTCSession * This,
            /* [in] */ __RPC__in BSTR bstrAddress,
            /* [in] */ __RPC__in BSTR bstrName,
            /* [retval][out] */ __RPC__deref_out_opt IRTCParticipant **ppParticipant);
        
        DECLSPEC_XFGVIRT(IRTCSession, RemoveParticipant)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *RemoveParticipant )( 
            __RPC__in IRTCSession * This,
            /* [in] */ __RPC__in_opt IRTCParticipant *pParticipant);
        
        DECLSPEC_XFGVIRT(IRTCSession, EnumerateParticipants)
        /* [helpstring][hidden] */ HRESULT ( STDMETHODCALLTYPE *EnumerateParticipants )( 
            __RPC__in IRTCSession * This,
            /* [retval][out] */ __RPC__deref_out_opt IRTCEnumParticipants **ppEnum);
        
        DECLSPEC_XFGVIRT(IRTCSession, get_CanAddParticipants)
        /* [helpstring][propget] */ HRESULT ( STDMETHODCALLTYPE *get_CanAddParticipants )( 
            __RPC__in IRTCSession * This,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pfCanAdd);
        
        DECLSPEC_XFGVIRT(IRTCSession, get_RedirectedUserURI)
        /* [helpstring][propget] */ HRESULT ( STDMETHODCALLTYPE *get_RedirectedUserURI )( 
            __RPC__in IRTCSession * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrUserURI);
        
        DECLSPEC_XFGVIRT(IRTCSession, get_RedirectedUserName)
        /* [helpstring][propget] */ HRESULT ( STDMETHODCALLTYPE *get_RedirectedUserName )( 
            __RPC__in IRTCSession * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrUserName);
        
        DECLSPEC_XFGVIRT(IRTCSession, NextRedirectedUser)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *NextRedirectedUser )( 
            __RPC__in IRTCSession * This);
        
        DECLSPEC_XFGVIRT(IRTCSession, SendMessage)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *SendMessage )( 
            __RPC__in IRTCSession * This,
            /* [in] */ __RPC__in BSTR bstrMessageHeader,
            /* [in] */ __RPC__in BSTR bstrMessage,
            /* [in] */ LONG_PTR lCookie);
        
        DECLSPEC_XFGVIRT(IRTCSession, SendMessageStatus)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *SendMessageStatus )( 
            __RPC__in IRTCSession * This,
            /* [in] */ RTC_MESSAGING_USER_STATUS enUserStatus,
            /* [in] */ LONG_PTR lCookie);
        
        DECLSPEC_XFGVIRT(IRTCSession, AddStream)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *AddStream )( 
            __RPC__in IRTCSession * This,
            /* [in] */ long lMediaType,
            /* [in] */ LONG_PTR lCookie);
        
        DECLSPEC_XFGVIRT(IRTCSession, RemoveStream)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *RemoveStream )( 
            __RPC__in IRTCSession * This,
            /* [in] */ long lMediaType,
            /* [in] */ LONG_PTR lCookie);
        
        DECLSPEC_XFGVIRT(IRTCSession, put_EncryptionKey)
        /* [helpstring][propput] */ HRESULT ( STDMETHODCALLTYPE *put_EncryptionKey )( 
            __RPC__in IRTCSession * This,
            /* [in] */ long lMediaType,
            /* [in] */ __RPC__in BSTR EncryptionKey);
        
        END_INTERFACE
    } IRTCSessionVtbl;

    interface IRTCSession
    {
        CONST_VTBL struct IRTCSessionVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IRTCSession_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IRTCSession_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IRTCSession_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IRTCSession_get_Client(This,ppClient)	\
    ( (This)->lpVtbl -> get_Client(This,ppClient) ) 

#define IRTCSession_get_State(This,penState)	\
    ( (This)->lpVtbl -> get_State(This,penState) ) 

#define IRTCSession_get_Type(This,penType)	\
    ( (This)->lpVtbl -> get_Type(This,penType) ) 

#define IRTCSession_get_Profile(This,ppProfile)	\
    ( (This)->lpVtbl -> get_Profile(This,ppProfile) ) 

#define IRTCSession_get_Participants(This,ppCollection)	\
    ( (This)->lpVtbl -> get_Participants(This,ppCollection) ) 

#define IRTCSession_Answer(This)	\
    ( (This)->lpVtbl -> Answer(This) ) 

#define IRTCSession_Terminate(This,enReason)	\
    ( (This)->lpVtbl -> Terminate(This,enReason) ) 

#define IRTCSession_Redirect(This,enType,bstrLocalPhoneURI,pProfile,lFlags)	\
    ( (This)->lpVtbl -> Redirect(This,enType,bstrLocalPhoneURI,pProfile,lFlags) ) 

#define IRTCSession_AddParticipant(This,bstrAddress,bstrName,ppParticipant)	\
    ( (This)->lpVtbl -> AddParticipant(This,bstrAddress,bstrName,ppParticipant) ) 

#define IRTCSession_RemoveParticipant(This,pParticipant)	\
    ( (This)->lpVtbl -> RemoveParticipant(This,pParticipant) ) 

#define IRTCSession_EnumerateParticipants(This,ppEnum)	\
    ( (This)->lpVtbl -> EnumerateParticipants(This,ppEnum) ) 

#define IRTCSession_get_CanAddParticipants(This,pfCanAdd)	\
    ( (This)->lpVtbl -> get_CanAddParticipants(This,pfCanAdd) ) 

#define IRTCSession_get_RedirectedUserURI(This,pbstrUserURI)	\
    ( (This)->lpVtbl -> get_RedirectedUserURI(This,pbstrUserURI) ) 

#define IRTCSession_get_RedirectedUserName(This,pbstrUserName)	\
    ( (This)->lpVtbl -> get_RedirectedUserName(This,pbstrUserName) ) 

#define IRTCSession_NextRedirectedUser(This)	\
    ( (This)->lpVtbl -> NextRedirectedUser(This) ) 

#define IRTCSession_SendMessage(This,bstrMessageHeader,bstrMessage,lCookie)	\
    ( (This)->lpVtbl -> SendMessage(This,bstrMessageHeader,bstrMessage,lCookie) ) 

#define IRTCSession_SendMessageStatus(This,enUserStatus,lCookie)	\
    ( (This)->lpVtbl -> SendMessageStatus(This,enUserStatus,lCookie) ) 

#define IRTCSession_AddStream(This,lMediaType,lCookie)	\
    ( (This)->lpVtbl -> AddStream(This,lMediaType,lCookie) ) 

#define IRTCSession_RemoveStream(This,lMediaType,lCookie)	\
    ( (This)->lpVtbl -> RemoveStream(This,lMediaType,lCookie) ) 

#define IRTCSession_put_EncryptionKey(This,lMediaType,EncryptionKey)	\
    ( (This)->lpVtbl -> put_EncryptionKey(This,lMediaType,EncryptionKey) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IRTCSession_INTERFACE_DEFINED__ */


#ifndef __IRTCSession2_INTERFACE_DEFINED__
#define __IRTCSession2_INTERFACE_DEFINED__

/* interface IRTCSession2 */
/* [unique][helpstring][uuid][object] */ 


EXTERN_C const IID IID_IRTCSession2;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("17D7CDFC-B007-484c-99D2-86A8A820991D")
    IRTCSession2 : public IRTCSession
    {
    public:
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE SendInfo( 
            /* [in] */ __RPC__in BSTR bstrInfoHeader,
            /* [in] */ __RPC__in BSTR bstrInfo,
            /* [in] */ LONG_PTR lCookie) = 0;
        
        virtual /* [helpstring][propput] */ HRESULT STDMETHODCALLTYPE put_PreferredSecurityLevel( 
            /* [in] */ RTC_SECURITY_TYPE enSecurityType,
            /* [in] */ RTC_SECURITY_LEVEL enSecurityLevel) = 0;
        
        virtual /* [helpstring][propget] */ HRESULT STDMETHODCALLTYPE get_PreferredSecurityLevel( 
            /* [in] */ RTC_SECURITY_TYPE enSecurityType,
            /* [retval][out] */ __RPC__out RTC_SECURITY_LEVEL *penSecurityLevel) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE IsSecurityEnabled( 
            /* [in] */ RTC_SECURITY_TYPE enSecurityType,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pfSecurityEnabled) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE AnswerWithSessionDescription( 
            /* [in] */ __RPC__in BSTR bstrContentType,
            /* [in] */ __RPC__in BSTR bstrSessionDescription) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE ReInviteWithSessionDescription( 
            /* [in] */ __RPC__in BSTR bstrContentType,
            /* [in] */ __RPC__in BSTR bstrSessionDescription,
            /* [in] */ LONG_PTR lCookie) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IRTCSession2Vtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IRTCSession2 * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IRTCSession2 * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IRTCSession2 * This);
        
        DECLSPEC_XFGVIRT(IRTCSession, get_Client)
        /* [helpstring][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Client )( 
            __RPC__in IRTCSession2 * This,
            /* [retval][out] */ __RPC__deref_out_opt IRTCClient **ppClient);
        
        DECLSPEC_XFGVIRT(IRTCSession, get_State)
        /* [helpstring][propget] */ HRESULT ( STDMETHODCALLTYPE *get_State )( 
            __RPC__in IRTCSession2 * This,
            /* [retval][out] */ __RPC__out RTC_SESSION_STATE *penState);
        
        DECLSPEC_XFGVIRT(IRTCSession, get_Type)
        /* [helpstring][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Type )( 
            __RPC__in IRTCSession2 * This,
            /* [retval][out] */ __RPC__out RTC_SESSION_TYPE *penType);
        
        DECLSPEC_XFGVIRT(IRTCSession, get_Profile)
        /* [helpstring][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Profile )( 
            __RPC__in IRTCSession2 * This,
            /* [retval][out] */ __RPC__deref_out_opt IRTCProfile **ppProfile);
        
        DECLSPEC_XFGVIRT(IRTCSession, get_Participants)
        /* [helpstring][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Participants )( 
            __RPC__in IRTCSession2 * This,
            /* [retval][out] */ __RPC__deref_out_opt IRTCCollection **ppCollection);
        
        DECLSPEC_XFGVIRT(IRTCSession, Answer)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *Answer )( 
            __RPC__in IRTCSession2 * This);
        
        DECLSPEC_XFGVIRT(IRTCSession, Terminate)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *Terminate )( 
            __RPC__in IRTCSession2 * This,
            /* [in] */ RTC_TERMINATE_REASON enReason);
        
        DECLSPEC_XFGVIRT(IRTCSession, Redirect)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *Redirect )( 
            __RPC__in IRTCSession2 * This,
            /* [in] */ RTC_SESSION_TYPE enType,
            /* [in] */ __RPC__in BSTR bstrLocalPhoneURI,
            /* [in] */ __RPC__in_opt IRTCProfile *pProfile,
            /* [in] */ long lFlags);
        
        DECLSPEC_XFGVIRT(IRTCSession, AddParticipant)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *AddParticipant )( 
            __RPC__in IRTCSession2 * This,
            /* [in] */ __RPC__in BSTR bstrAddress,
            /* [in] */ __RPC__in BSTR bstrName,
            /* [retval][out] */ __RPC__deref_out_opt IRTCParticipant **ppParticipant);
        
        DECLSPEC_XFGVIRT(IRTCSession, RemoveParticipant)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *RemoveParticipant )( 
            __RPC__in IRTCSession2 * This,
            /* [in] */ __RPC__in_opt IRTCParticipant *pParticipant);
        
        DECLSPEC_XFGVIRT(IRTCSession, EnumerateParticipants)
        /* [helpstring][hidden] */ HRESULT ( STDMETHODCALLTYPE *EnumerateParticipants )( 
            __RPC__in IRTCSession2 * This,
            /* [retval][out] */ __RPC__deref_out_opt IRTCEnumParticipants **ppEnum);
        
        DECLSPEC_XFGVIRT(IRTCSession, get_CanAddParticipants)
        /* [helpstring][propget] */ HRESULT ( STDMETHODCALLTYPE *get_CanAddParticipants )( 
            __RPC__in IRTCSession2 * This,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pfCanAdd);
        
        DECLSPEC_XFGVIRT(IRTCSession, get_RedirectedUserURI)
        /* [helpstring][propget] */ HRESULT ( STDMETHODCALLTYPE *get_RedirectedUserURI )( 
            __RPC__in IRTCSession2 * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrUserURI);
        
        DECLSPEC_XFGVIRT(IRTCSession, get_RedirectedUserName)
        /* [helpstring][propget] */ HRESULT ( STDMETHODCALLTYPE *get_RedirectedUserName )( 
            __RPC__in IRTCSession2 * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrUserName);
        
        DECLSPEC_XFGVIRT(IRTCSession, NextRedirectedUser)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *NextRedirectedUser )( 
            __RPC__in IRTCSession2 * This);
        
        DECLSPEC_XFGVIRT(IRTCSession, SendMessage)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *SendMessage )( 
            __RPC__in IRTCSession2 * This,
            /* [in] */ __RPC__in BSTR bstrMessageHeader,
            /* [in] */ __RPC__in BSTR bstrMessage,
            /* [in] */ LONG_PTR lCookie);
        
        DECLSPEC_XFGVIRT(IRTCSession, SendMessageStatus)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *SendMessageStatus )( 
            __RPC__in IRTCSession2 * This,
            /* [in] */ RTC_MESSAGING_USER_STATUS enUserStatus,
            /* [in] */ LONG_PTR lCookie);
        
        DECLSPEC_XFGVIRT(IRTCSession, AddStream)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *AddStream )( 
            __RPC__in IRTCSession2 * This,
            /* [in] */ long lMediaType,
            /* [in] */ LONG_PTR lCookie);
        
        DECLSPEC_XFGVIRT(IRTCSession, RemoveStream)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *RemoveStream )( 
            __RPC__in IRTCSession2 * This,
            /* [in] */ long lMediaType,
            /* [in] */ LONG_PTR lCookie);
        
        DECLSPEC_XFGVIRT(IRTCSession, put_EncryptionKey)
        /* [helpstring][propput] */ HRESULT ( STDMETHODCALLTYPE *put_EncryptionKey )( 
            __RPC__in IRTCSession2 * This,
            /* [in] */ long lMediaType,
            /* [in] */ __RPC__in BSTR EncryptionKey);
        
        DECLSPEC_XFGVIRT(IRTCSession2, SendInfo)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *SendInfo )( 
            __RPC__in IRTCSession2 * This,
            /* [in] */ __RPC__in BSTR bstrInfoHeader,
            /* [in] */ __RPC__in BSTR bstrInfo,
            /* [in] */ LONG_PTR lCookie);
        
        DECLSPEC_XFGVIRT(IRTCSession2, put_PreferredSecurityLevel)
        /* [helpstring][propput] */ HRESULT ( STDMETHODCALLTYPE *put_PreferredSecurityLevel )( 
            __RPC__in IRTCSession2 * This,
            /* [in] */ RTC_SECURITY_TYPE enSecurityType,
            /* [in] */ RTC_SECURITY_LEVEL enSecurityLevel);
        
        DECLSPEC_XFGVIRT(IRTCSession2, get_PreferredSecurityLevel)
        /* [helpstring][propget] */ HRESULT ( STDMETHODCALLTYPE *get_PreferredSecurityLevel )( 
            __RPC__in IRTCSession2 * This,
            /* [in] */ RTC_SECURITY_TYPE enSecurityType,
            /* [retval][out] */ __RPC__out RTC_SECURITY_LEVEL *penSecurityLevel);
        
        DECLSPEC_XFGVIRT(IRTCSession2, IsSecurityEnabled)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *IsSecurityEnabled )( 
            __RPC__in IRTCSession2 * This,
            /* [in] */ RTC_SECURITY_TYPE enSecurityType,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pfSecurityEnabled);
        
        DECLSPEC_XFGVIRT(IRTCSession2, AnswerWithSessionDescription)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *AnswerWithSessionDescription )( 
            __RPC__in IRTCSession2 * This,
            /* [in] */ __RPC__in BSTR bstrContentType,
            /* [in] */ __RPC__in BSTR bstrSessionDescription);
        
        DECLSPEC_XFGVIRT(IRTCSession2, ReInviteWithSessionDescription)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *ReInviteWithSessionDescription )( 
            __RPC__in IRTCSession2 * This,
            /* [in] */ __RPC__in BSTR bstrContentType,
            /* [in] */ __RPC__in BSTR bstrSessionDescription,
            /* [in] */ LONG_PTR lCookie);
        
        END_INTERFACE
    } IRTCSession2Vtbl;

    interface IRTCSession2
    {
        CONST_VTBL struct IRTCSession2Vtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IRTCSession2_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IRTCSession2_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IRTCSession2_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IRTCSession2_get_Client(This,ppClient)	\
    ( (This)->lpVtbl -> get_Client(This,ppClient) ) 

#define IRTCSession2_get_State(This,penState)	\
    ( (This)->lpVtbl -> get_State(This,penState) ) 

#define IRTCSession2_get_Type(This,penType)	\
    ( (This)->lpVtbl -> get_Type(This,penType) ) 

#define IRTCSession2_get_Profile(This,ppProfile)	\
    ( (This)->lpVtbl -> get_Profile(This,ppProfile) ) 

#define IRTCSession2_get_Participants(This,ppCollection)	\
    ( (This)->lpVtbl -> get_Participants(This,ppCollection) ) 

#define IRTCSession2_Answer(This)	\
    ( (This)->lpVtbl -> Answer(This) ) 

#define IRTCSession2_Terminate(This,enReason)	\
    ( (This)->lpVtbl -> Terminate(This,enReason) ) 

#define IRTCSession2_Redirect(This,enType,bstrLocalPhoneURI,pProfile,lFlags)	\
    ( (This)->lpVtbl -> Redirect(This,enType,bstrLocalPhoneURI,pProfile,lFlags) ) 

#define IRTCSession2_AddParticipant(This,bstrAddress,bstrName,ppParticipant)	\
    ( (This)->lpVtbl -> AddParticipant(This,bstrAddress,bstrName,ppParticipant) ) 

#define IRTCSession2_RemoveParticipant(This,pParticipant)	\
    ( (This)->lpVtbl -> RemoveParticipant(This,pParticipant) ) 

#define IRTCSession2_EnumerateParticipants(This,ppEnum)	\
    ( (This)->lpVtbl -> EnumerateParticipants(This,ppEnum) ) 

#define IRTCSession2_get_CanAddParticipants(This,pfCanAdd)	\
    ( (This)->lpVtbl -> get_CanAddParticipants(This,pfCanAdd) ) 

#define IRTCSession2_get_RedirectedUserURI(This,pbstrUserURI)	\
    ( (This)->lpVtbl -> get_RedirectedUserURI(This,pbstrUserURI) ) 

#define IRTCSession2_get_RedirectedUserName(This,pbstrUserName)	\
    ( (This)->lpVtbl -> get_RedirectedUserName(This,pbstrUserName) ) 

#define IRTCSession2_NextRedirectedUser(This)	\
    ( (This)->lpVtbl -> NextRedirectedUser(This) ) 

#define IRTCSession2_SendMessage(This,bstrMessageHeader,bstrMessage,lCookie)	\
    ( (This)->lpVtbl -> SendMessage(This,bstrMessageHeader,bstrMessage,lCookie) ) 

#define IRTCSession2_SendMessageStatus(This,enUserStatus,lCookie)	\
    ( (This)->lpVtbl -> SendMessageStatus(This,enUserStatus,lCookie) ) 

#define IRTCSession2_AddStream(This,lMediaType,lCookie)	\
    ( (This)->lpVtbl -> AddStream(This,lMediaType,lCookie) ) 

#define IRTCSession2_RemoveStream(This,lMediaType,lCookie)	\
    ( (This)->lpVtbl -> RemoveStream(This,lMediaType,lCookie) ) 

#define IRTCSession2_put_EncryptionKey(This,lMediaType,EncryptionKey)	\
    ( (This)->lpVtbl -> put_EncryptionKey(This,lMediaType,EncryptionKey) ) 


#define IRTCSession2_SendInfo(This,bstrInfoHeader,bstrInfo,lCookie)	\
    ( (This)->lpVtbl -> SendInfo(This,bstrInfoHeader,bstrInfo,lCookie) ) 

#define IRTCSession2_put_PreferredSecurityLevel(This,enSecurityType,enSecurityLevel)	\
    ( (This)->lpVtbl -> put_PreferredSecurityLevel(This,enSecurityType,enSecurityLevel) ) 

#define IRTCSession2_get_PreferredSecurityLevel(This,enSecurityType,penSecurityLevel)	\
    ( (This)->lpVtbl -> get_PreferredSecurityLevel(This,enSecurityType,penSecurityLevel) ) 

#define IRTCSession2_IsSecurityEnabled(This,enSecurityType,pfSecurityEnabled)	\
    ( (This)->lpVtbl -> IsSecurityEnabled(This,enSecurityType,pfSecurityEnabled) ) 

#define IRTCSession2_AnswerWithSessionDescription(This,bstrContentType,bstrSessionDescription)	\
    ( (This)->lpVtbl -> AnswerWithSessionDescription(This,bstrContentType,bstrSessionDescription) ) 

#define IRTCSession2_ReInviteWithSessionDescription(This,bstrContentType,bstrSessionDescription,lCookie)	\
    ( (This)->lpVtbl -> ReInviteWithSessionDescription(This,bstrContentType,bstrSessionDescription,lCookie) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IRTCSession2_INTERFACE_DEFINED__ */


#ifndef __IRTCSessionCallControl_INTERFACE_DEFINED__
#define __IRTCSessionCallControl_INTERFACE_DEFINED__

/* interface IRTCSessionCallControl */
/* [unique][helpstring][uuid][object] */ 


EXTERN_C const IID IID_IRTCSessionCallControl;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("E9A50D94-190B-4F82-9530-3B8EBF60758A")
    IRTCSessionCallControl : public IUnknown
    {
    public:
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE Hold( 
            /* [in] */ LONG_PTR lCookie) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE UnHold( 
            /* [in] */ LONG_PTR lCookie) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE Forward( 
            /* [in] */ __RPC__in BSTR bstrForwardToURI) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE Refer( 
            /* [in] */ __RPC__in BSTR bstrReferToURI,
            /* [in] */ __RPC__in BSTR bstrReferCookie) = 0;
        
        virtual /* [helpstring][propput] */ HRESULT STDMETHODCALLTYPE put_ReferredByURI( 
            /* [in] */ __RPC__in BSTR bstrReferredByURI) = 0;
        
        virtual /* [helpstring][propget] */ HRESULT STDMETHODCALLTYPE get_ReferredByURI( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrReferredByURI) = 0;
        
        virtual /* [helpstring][propput] */ HRESULT STDMETHODCALLTYPE put_ReferCookie( 
            /* [in] */ __RPC__in BSTR bstrReferCookie) = 0;
        
        virtual /* [helpstring][propget] */ HRESULT STDMETHODCALLTYPE get_ReferCookie( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrReferCookie) = 0;
        
        virtual /* [helpstring][propget] */ HRESULT STDMETHODCALLTYPE get_IsReferred( 
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pfIsReferred) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IRTCSessionCallControlVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IRTCSessionCallControl * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IRTCSessionCallControl * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IRTCSessionCallControl * This);
        
        DECLSPEC_XFGVIRT(IRTCSessionCallControl, Hold)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *Hold )( 
            __RPC__in IRTCSessionCallControl * This,
            /* [in] */ LONG_PTR lCookie);
        
        DECLSPEC_XFGVIRT(IRTCSessionCallControl, UnHold)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *UnHold )( 
            __RPC__in IRTCSessionCallControl * This,
            /* [in] */ LONG_PTR lCookie);
        
        DECLSPEC_XFGVIRT(IRTCSessionCallControl, Forward)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *Forward )( 
            __RPC__in IRTCSessionCallControl * This,
            /* [in] */ __RPC__in BSTR bstrForwardToURI);
        
        DECLSPEC_XFGVIRT(IRTCSessionCallControl, Refer)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *Refer )( 
            __RPC__in IRTCSessionCallControl * This,
            /* [in] */ __RPC__in BSTR bstrReferToURI,
            /* [in] */ __RPC__in BSTR bstrReferCookie);
        
        DECLSPEC_XFGVIRT(IRTCSessionCallControl, put_ReferredByURI)
        /* [helpstring][propput] */ HRESULT ( STDMETHODCALLTYPE *put_ReferredByURI )( 
            __RPC__in IRTCSessionCallControl * This,
            /* [in] */ __RPC__in BSTR bstrReferredByURI);
        
        DECLSPEC_XFGVIRT(IRTCSessionCallControl, get_ReferredByURI)
        /* [helpstring][propget] */ HRESULT ( STDMETHODCALLTYPE *get_ReferredByURI )( 
            __RPC__in IRTCSessionCallControl * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrReferredByURI);
        
        DECLSPEC_XFGVIRT(IRTCSessionCallControl, put_ReferCookie)
        /* [helpstring][propput] */ HRESULT ( STDMETHODCALLTYPE *put_ReferCookie )( 
            __RPC__in IRTCSessionCallControl * This,
            /* [in] */ __RPC__in BSTR bstrReferCookie);
        
        DECLSPEC_XFGVIRT(IRTCSessionCallControl, get_ReferCookie)
        /* [helpstring][propget] */ HRESULT ( STDMETHODCALLTYPE *get_ReferCookie )( 
            __RPC__in IRTCSessionCallControl * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrReferCookie);
        
        DECLSPEC_XFGVIRT(IRTCSessionCallControl, get_IsReferred)
        /* [helpstring][propget] */ HRESULT ( STDMETHODCALLTYPE *get_IsReferred )( 
            __RPC__in IRTCSessionCallControl * This,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pfIsReferred);
        
        END_INTERFACE
    } IRTCSessionCallControlVtbl;

    interface IRTCSessionCallControl
    {
        CONST_VTBL struct IRTCSessionCallControlVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IRTCSessionCallControl_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IRTCSessionCallControl_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IRTCSessionCallControl_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IRTCSessionCallControl_Hold(This,lCookie)	\
    ( (This)->lpVtbl -> Hold(This,lCookie) ) 

#define IRTCSessionCallControl_UnHold(This,lCookie)	\
    ( (This)->lpVtbl -> UnHold(This,lCookie) ) 

#define IRTCSessionCallControl_Forward(This,bstrForwardToURI)	\
    ( (This)->lpVtbl -> Forward(This,bstrForwardToURI) ) 

#define IRTCSessionCallControl_Refer(This,bstrReferToURI,bstrReferCookie)	\
    ( (This)->lpVtbl -> Refer(This,bstrReferToURI,bstrReferCookie) ) 

#define IRTCSessionCallControl_put_ReferredByURI(This,bstrReferredByURI)	\
    ( (This)->lpVtbl -> put_ReferredByURI(This,bstrReferredByURI) ) 

#define IRTCSessionCallControl_get_ReferredByURI(This,pbstrReferredByURI)	\
    ( (This)->lpVtbl -> get_ReferredByURI(This,pbstrReferredByURI) ) 

#define IRTCSessionCallControl_put_ReferCookie(This,bstrReferCookie)	\
    ( (This)->lpVtbl -> put_ReferCookie(This,bstrReferCookie) ) 

#define IRTCSessionCallControl_get_ReferCookie(This,pbstrReferCookie)	\
    ( (This)->lpVtbl -> get_ReferCookie(This,pbstrReferCookie) ) 

#define IRTCSessionCallControl_get_IsReferred(This,pfIsReferred)	\
    ( (This)->lpVtbl -> get_IsReferred(This,pfIsReferred) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IRTCSessionCallControl_INTERFACE_DEFINED__ */


#ifndef __IRTCParticipant_INTERFACE_DEFINED__
#define __IRTCParticipant_INTERFACE_DEFINED__

/* interface IRTCParticipant */
/* [unique][helpstring][uuid][object] */ 


EXTERN_C const IID IID_IRTCParticipant;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("ae86add5-26b1-4414-af1d-b94cd938d739")
    IRTCParticipant : public IUnknown
    {
    public:
        virtual /* [helpstring][propget] */ HRESULT STDMETHODCALLTYPE get_UserURI( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrUserURI) = 0;
        
        virtual /* [helpstring][propget] */ HRESULT STDMETHODCALLTYPE get_Name( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrName) = 0;
        
        virtual /* [helpstring][propget] */ HRESULT STDMETHODCALLTYPE get_Removable( 
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pfRemovable) = 0;
        
        virtual /* [helpstring][propget] */ HRESULT STDMETHODCALLTYPE get_State( 
            /* [retval][out] */ __RPC__out RTC_PARTICIPANT_STATE *penState) = 0;
        
        virtual /* [helpstring][propget] */ HRESULT STDMETHODCALLTYPE get_Session( 
            /* [retval][out] */ __RPC__deref_out_opt IRTCSession **ppSession) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IRTCParticipantVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IRTCParticipant * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IRTCParticipant * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IRTCParticipant * This);
        
        DECLSPEC_XFGVIRT(IRTCParticipant, get_UserURI)
        /* [helpstring][propget] */ HRESULT ( STDMETHODCALLTYPE *get_UserURI )( 
            __RPC__in IRTCParticipant * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrUserURI);
        
        DECLSPEC_XFGVIRT(IRTCParticipant, get_Name)
        /* [helpstring][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Name )( 
            __RPC__in IRTCParticipant * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrName);
        
        DECLSPEC_XFGVIRT(IRTCParticipant, get_Removable)
        /* [helpstring][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Removable )( 
            __RPC__in IRTCParticipant * This,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pfRemovable);
        
        DECLSPEC_XFGVIRT(IRTCParticipant, get_State)
        /* [helpstring][propget] */ HRESULT ( STDMETHODCALLTYPE *get_State )( 
            __RPC__in IRTCParticipant * This,
            /* [retval][out] */ __RPC__out RTC_PARTICIPANT_STATE *penState);
        
        DECLSPEC_XFGVIRT(IRTCParticipant, get_Session)
        /* [helpstring][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Session )( 
            __RPC__in IRTCParticipant * This,
            /* [retval][out] */ __RPC__deref_out_opt IRTCSession **ppSession);
        
        END_INTERFACE
    } IRTCParticipantVtbl;

    interface IRTCParticipant
    {
        CONST_VTBL struct IRTCParticipantVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IRTCParticipant_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IRTCParticipant_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IRTCParticipant_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IRTCParticipant_get_UserURI(This,pbstrUserURI)	\
    ( (This)->lpVtbl -> get_UserURI(This,pbstrUserURI) ) 

#define IRTCParticipant_get_Name(This,pbstrName)	\
    ( (This)->lpVtbl -> get_Name(This,pbstrName) ) 

#define IRTCParticipant_get_Removable(This,pfRemovable)	\
    ( (This)->lpVtbl -> get_Removable(This,pfRemovable) ) 

#define IRTCParticipant_get_State(This,penState)	\
    ( (This)->lpVtbl -> get_State(This,penState) ) 

#define IRTCParticipant_get_Session(This,ppSession)	\
    ( (This)->lpVtbl -> get_Session(This,ppSession) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IRTCParticipant_INTERFACE_DEFINED__ */


#ifndef __IRTCRoamingEvent_INTERFACE_DEFINED__
#define __IRTCRoamingEvent_INTERFACE_DEFINED__

/* interface IRTCRoamingEvent */
/* [unique][helpstring][dual][uuid][object] */ 


EXTERN_C const IID IID_IRTCRoamingEvent;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("79960a6b-0cb1-4dc8-a805-7318e99902e8")
    IRTCRoamingEvent : public IDispatch
    {
    public:
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_EventType( 
            /* [retval][out] */ __RPC__out RTC_ROAMING_EVENT_TYPE *pEventType) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Profile( 
            /* [retval][out] */ __RPC__deref_out_opt IRTCProfile2 **ppProfile) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_StatusCode( 
            /* [retval][out] */ __RPC__out long *plStatusCode) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_StatusText( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrStatusText) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IRTCRoamingEventVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IRTCRoamingEvent * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IRTCRoamingEvent * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IRTCRoamingEvent * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IRTCRoamingEvent * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IRTCRoamingEvent * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IRTCRoamingEvent * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IRTCRoamingEvent * This,
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
        
        DECLSPEC_XFGVIRT(IRTCRoamingEvent, get_EventType)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_EventType )( 
            __RPC__in IRTCRoamingEvent * This,
            /* [retval][out] */ __RPC__out RTC_ROAMING_EVENT_TYPE *pEventType);
        
        DECLSPEC_XFGVIRT(IRTCRoamingEvent, get_Profile)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Profile )( 
            __RPC__in IRTCRoamingEvent * This,
            /* [retval][out] */ __RPC__deref_out_opt IRTCProfile2 **ppProfile);
        
        DECLSPEC_XFGVIRT(IRTCRoamingEvent, get_StatusCode)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_StatusCode )( 
            __RPC__in IRTCRoamingEvent * This,
            /* [retval][out] */ __RPC__out long *plStatusCode);
        
        DECLSPEC_XFGVIRT(IRTCRoamingEvent, get_StatusText)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_StatusText )( 
            __RPC__in IRTCRoamingEvent * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrStatusText);
        
        END_INTERFACE
    } IRTCRoamingEventVtbl;

    interface IRTCRoamingEvent
    {
        CONST_VTBL struct IRTCRoamingEventVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IRTCRoamingEvent_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IRTCRoamingEvent_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IRTCRoamingEvent_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IRTCRoamingEvent_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IRTCRoamingEvent_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IRTCRoamingEvent_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IRTCRoamingEvent_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IRTCRoamingEvent_get_EventType(This,pEventType)	\
    ( (This)->lpVtbl -> get_EventType(This,pEventType) ) 

#define IRTCRoamingEvent_get_Profile(This,ppProfile)	\
    ( (This)->lpVtbl -> get_Profile(This,ppProfile) ) 

#define IRTCRoamingEvent_get_StatusCode(This,plStatusCode)	\
    ( (This)->lpVtbl -> get_StatusCode(This,plStatusCode) ) 

#define IRTCRoamingEvent_get_StatusText(This,pbstrStatusText)	\
    ( (This)->lpVtbl -> get_StatusText(This,pbstrStatusText) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IRTCRoamingEvent_INTERFACE_DEFINED__ */


#ifndef __IRTCProfileEvent_INTERFACE_DEFINED__
#define __IRTCProfileEvent_INTERFACE_DEFINED__

/* interface IRTCProfileEvent */
/* [unique][helpstring][dual][uuid][object] */ 


EXTERN_C const IID IID_IRTCProfileEvent;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("D6D5AB3B-770E-43e8-800A-79B062395FCA")
    IRTCProfileEvent : public IDispatch
    {
    public:
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Profile( 
            /* [retval][out] */ __RPC__deref_out_opt IRTCProfile **ppProfile) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Cookie( 
            /* [retval][out] */ __RPC__out LONG_PTR *plCookie) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_StatusCode( 
            /* [retval][out] */ __RPC__out long *plStatusCode) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IRTCProfileEventVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IRTCProfileEvent * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IRTCProfileEvent * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IRTCProfileEvent * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IRTCProfileEvent * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IRTCProfileEvent * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IRTCProfileEvent * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IRTCProfileEvent * This,
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
        
        DECLSPEC_XFGVIRT(IRTCProfileEvent, get_Profile)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Profile )( 
            __RPC__in IRTCProfileEvent * This,
            /* [retval][out] */ __RPC__deref_out_opt IRTCProfile **ppProfile);
        
        DECLSPEC_XFGVIRT(IRTCProfileEvent, get_Cookie)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Cookie )( 
            __RPC__in IRTCProfileEvent * This,
            /* [retval][out] */ __RPC__out LONG_PTR *plCookie);
        
        DECLSPEC_XFGVIRT(IRTCProfileEvent, get_StatusCode)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_StatusCode )( 
            __RPC__in IRTCProfileEvent * This,
            /* [retval][out] */ __RPC__out long *plStatusCode);
        
        END_INTERFACE
    } IRTCProfileEventVtbl;

    interface IRTCProfileEvent
    {
        CONST_VTBL struct IRTCProfileEventVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IRTCProfileEvent_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IRTCProfileEvent_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IRTCProfileEvent_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IRTCProfileEvent_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IRTCProfileEvent_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IRTCProfileEvent_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IRTCProfileEvent_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IRTCProfileEvent_get_Profile(This,ppProfile)	\
    ( (This)->lpVtbl -> get_Profile(This,ppProfile) ) 

#define IRTCProfileEvent_get_Cookie(This,plCookie)	\
    ( (This)->lpVtbl -> get_Cookie(This,plCookie) ) 

#define IRTCProfileEvent_get_StatusCode(This,plStatusCode)	\
    ( (This)->lpVtbl -> get_StatusCode(This,plStatusCode) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IRTCProfileEvent_INTERFACE_DEFINED__ */


#ifndef __IRTCProfileEvent2_INTERFACE_DEFINED__
#define __IRTCProfileEvent2_INTERFACE_DEFINED__

/* interface IRTCProfileEvent2 */
/* [unique][helpstring][dual][uuid][object] */ 


EXTERN_C const IID IID_IRTCProfileEvent2;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("62E56EDC-03FA-4121-94FB-23493FD0AE64")
    IRTCProfileEvent2 : public IRTCProfileEvent
    {
    public:
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_EventType( 
            /* [retval][out] */ __RPC__out RTC_PROFILE_EVENT_TYPE *pEventType) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IRTCProfileEvent2Vtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IRTCProfileEvent2 * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IRTCProfileEvent2 * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IRTCProfileEvent2 * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IRTCProfileEvent2 * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IRTCProfileEvent2 * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IRTCProfileEvent2 * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IRTCProfileEvent2 * This,
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
        
        DECLSPEC_XFGVIRT(IRTCProfileEvent, get_Profile)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Profile )( 
            __RPC__in IRTCProfileEvent2 * This,
            /* [retval][out] */ __RPC__deref_out_opt IRTCProfile **ppProfile);
        
        DECLSPEC_XFGVIRT(IRTCProfileEvent, get_Cookie)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Cookie )( 
            __RPC__in IRTCProfileEvent2 * This,
            /* [retval][out] */ __RPC__out LONG_PTR *plCookie);
        
        DECLSPEC_XFGVIRT(IRTCProfileEvent, get_StatusCode)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_StatusCode )( 
            __RPC__in IRTCProfileEvent2 * This,
            /* [retval][out] */ __RPC__out long *plStatusCode);
        
        DECLSPEC_XFGVIRT(IRTCProfileEvent2, get_EventType)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_EventType )( 
            __RPC__in IRTCProfileEvent2 * This,
            /* [retval][out] */ __RPC__out RTC_PROFILE_EVENT_TYPE *pEventType);
        
        END_INTERFACE
    } IRTCProfileEvent2Vtbl;

    interface IRTCProfileEvent2
    {
        CONST_VTBL struct IRTCProfileEvent2Vtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IRTCProfileEvent2_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IRTCProfileEvent2_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IRTCProfileEvent2_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IRTCProfileEvent2_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IRTCProfileEvent2_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IRTCProfileEvent2_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IRTCProfileEvent2_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IRTCProfileEvent2_get_Profile(This,ppProfile)	\
    ( (This)->lpVtbl -> get_Profile(This,ppProfile) ) 

#define IRTCProfileEvent2_get_Cookie(This,plCookie)	\
    ( (This)->lpVtbl -> get_Cookie(This,plCookie) ) 

#define IRTCProfileEvent2_get_StatusCode(This,plStatusCode)	\
    ( (This)->lpVtbl -> get_StatusCode(This,plStatusCode) ) 


#define IRTCProfileEvent2_get_EventType(This,pEventType)	\
    ( (This)->lpVtbl -> get_EventType(This,pEventType) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IRTCProfileEvent2_INTERFACE_DEFINED__ */


#ifndef __IRTCClientEvent_INTERFACE_DEFINED__
#define __IRTCClientEvent_INTERFACE_DEFINED__

/* interface IRTCClientEvent */
/* [unique][helpstring][dual][uuid][object] */ 


EXTERN_C const IID IID_IRTCClientEvent;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("2b493b7a-3cba-4170-9c8b-76a9dacdd644")
    IRTCClientEvent : public IDispatch
    {
    public:
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_EventType( 
            /* [retval][out] */ __RPC__out RTC_CLIENT_EVENT_TYPE *penEventType) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Client( 
            /* [retval][out] */ __RPC__deref_out_opt IRTCClient **ppClient) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IRTCClientEventVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IRTCClientEvent * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IRTCClientEvent * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IRTCClientEvent * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IRTCClientEvent * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IRTCClientEvent * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IRTCClientEvent * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IRTCClientEvent * This,
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
        
        DECLSPEC_XFGVIRT(IRTCClientEvent, get_EventType)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_EventType )( 
            __RPC__in IRTCClientEvent * This,
            /* [retval][out] */ __RPC__out RTC_CLIENT_EVENT_TYPE *penEventType);
        
        DECLSPEC_XFGVIRT(IRTCClientEvent, get_Client)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Client )( 
            __RPC__in IRTCClientEvent * This,
            /* [retval][out] */ __RPC__deref_out_opt IRTCClient **ppClient);
        
        END_INTERFACE
    } IRTCClientEventVtbl;

    interface IRTCClientEvent
    {
        CONST_VTBL struct IRTCClientEventVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IRTCClientEvent_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IRTCClientEvent_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IRTCClientEvent_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IRTCClientEvent_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IRTCClientEvent_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IRTCClientEvent_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IRTCClientEvent_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IRTCClientEvent_get_EventType(This,penEventType)	\
    ( (This)->lpVtbl -> get_EventType(This,penEventType) ) 

#define IRTCClientEvent_get_Client(This,ppClient)	\
    ( (This)->lpVtbl -> get_Client(This,ppClient) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IRTCClientEvent_INTERFACE_DEFINED__ */


#ifndef __IRTCRegistrationStateChangeEvent_INTERFACE_DEFINED__
#define __IRTCRegistrationStateChangeEvent_INTERFACE_DEFINED__

/* interface IRTCRegistrationStateChangeEvent */
/* [unique][helpstring][dual][uuid][object] */ 


EXTERN_C const IID IID_IRTCRegistrationStateChangeEvent;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("62d0991b-50ab-4f02-b948-ca94f26f8f95")
    IRTCRegistrationStateChangeEvent : public IDispatch
    {
    public:
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Profile( 
            /* [retval][out] */ __RPC__deref_out_opt IRTCProfile **ppProfile) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_State( 
            /* [retval][out] */ __RPC__out RTC_REGISTRATION_STATE *penState) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_StatusCode( 
            /* [retval][out] */ __RPC__out long *plStatusCode) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_StatusText( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrStatusText) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IRTCRegistrationStateChangeEventVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IRTCRegistrationStateChangeEvent * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IRTCRegistrationStateChangeEvent * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IRTCRegistrationStateChangeEvent * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IRTCRegistrationStateChangeEvent * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IRTCRegistrationStateChangeEvent * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IRTCRegistrationStateChangeEvent * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IRTCRegistrationStateChangeEvent * This,
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
        
        DECLSPEC_XFGVIRT(IRTCRegistrationStateChangeEvent, get_Profile)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Profile )( 
            __RPC__in IRTCRegistrationStateChangeEvent * This,
            /* [retval][out] */ __RPC__deref_out_opt IRTCProfile **ppProfile);
        
        DECLSPEC_XFGVIRT(IRTCRegistrationStateChangeEvent, get_State)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_State )( 
            __RPC__in IRTCRegistrationStateChangeEvent * This,
            /* [retval][out] */ __RPC__out RTC_REGISTRATION_STATE *penState);
        
        DECLSPEC_XFGVIRT(IRTCRegistrationStateChangeEvent, get_StatusCode)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_StatusCode )( 
            __RPC__in IRTCRegistrationStateChangeEvent * This,
            /* [retval][out] */ __RPC__out long *plStatusCode);
        
        DECLSPEC_XFGVIRT(IRTCRegistrationStateChangeEvent, get_StatusText)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_StatusText )( 
            __RPC__in IRTCRegistrationStateChangeEvent * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrStatusText);
        
        END_INTERFACE
    } IRTCRegistrationStateChangeEventVtbl;

    interface IRTCRegistrationStateChangeEvent
    {
        CONST_VTBL struct IRTCRegistrationStateChangeEventVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IRTCRegistrationStateChangeEvent_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IRTCRegistrationStateChangeEvent_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IRTCRegistrationStateChangeEvent_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IRTCRegistrationStateChangeEvent_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IRTCRegistrationStateChangeEvent_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IRTCRegistrationStateChangeEvent_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IRTCRegistrationStateChangeEvent_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IRTCRegistrationStateChangeEvent_get_Profile(This,ppProfile)	\
    ( (This)->lpVtbl -> get_Profile(This,ppProfile) ) 

#define IRTCRegistrationStateChangeEvent_get_State(This,penState)	\
    ( (This)->lpVtbl -> get_State(This,penState) ) 

#define IRTCRegistrationStateChangeEvent_get_StatusCode(This,plStatusCode)	\
    ( (This)->lpVtbl -> get_StatusCode(This,plStatusCode) ) 

#define IRTCRegistrationStateChangeEvent_get_StatusText(This,pbstrStatusText)	\
    ( (This)->lpVtbl -> get_StatusText(This,pbstrStatusText) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IRTCRegistrationStateChangeEvent_INTERFACE_DEFINED__ */


#ifndef __IRTCSessionStateChangeEvent_INTERFACE_DEFINED__
#define __IRTCSessionStateChangeEvent_INTERFACE_DEFINED__

/* interface IRTCSessionStateChangeEvent */
/* [unique][helpstring][dual][uuid][object] */ 


EXTERN_C const IID IID_IRTCSessionStateChangeEvent;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("b5bad703-5952-48b3-9321-7f4500521506")
    IRTCSessionStateChangeEvent : public IDispatch
    {
    public:
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Session( 
            /* [retval][out] */ __RPC__deref_out_opt IRTCSession **ppSession) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_State( 
            /* [retval][out] */ __RPC__out RTC_SESSION_STATE *penState) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_StatusCode( 
            /* [retval][out] */ __RPC__out long *plStatusCode) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_StatusText( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrStatusText) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IRTCSessionStateChangeEventVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IRTCSessionStateChangeEvent * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IRTCSessionStateChangeEvent * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IRTCSessionStateChangeEvent * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IRTCSessionStateChangeEvent * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IRTCSessionStateChangeEvent * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IRTCSessionStateChangeEvent * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IRTCSessionStateChangeEvent * This,
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
        
        DECLSPEC_XFGVIRT(IRTCSessionStateChangeEvent, get_Session)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Session )( 
            __RPC__in IRTCSessionStateChangeEvent * This,
            /* [retval][out] */ __RPC__deref_out_opt IRTCSession **ppSession);
        
        DECLSPEC_XFGVIRT(IRTCSessionStateChangeEvent, get_State)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_State )( 
            __RPC__in IRTCSessionStateChangeEvent * This,
            /* [retval][out] */ __RPC__out RTC_SESSION_STATE *penState);
        
        DECLSPEC_XFGVIRT(IRTCSessionStateChangeEvent, get_StatusCode)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_StatusCode )( 
            __RPC__in IRTCSessionStateChangeEvent * This,
            /* [retval][out] */ __RPC__out long *plStatusCode);
        
        DECLSPEC_XFGVIRT(IRTCSessionStateChangeEvent, get_StatusText)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_StatusText )( 
            __RPC__in IRTCSessionStateChangeEvent * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrStatusText);
        
        END_INTERFACE
    } IRTCSessionStateChangeEventVtbl;

    interface IRTCSessionStateChangeEvent
    {
        CONST_VTBL struct IRTCSessionStateChangeEventVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IRTCSessionStateChangeEvent_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IRTCSessionStateChangeEvent_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IRTCSessionStateChangeEvent_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IRTCSessionStateChangeEvent_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IRTCSessionStateChangeEvent_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IRTCSessionStateChangeEvent_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IRTCSessionStateChangeEvent_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IRTCSessionStateChangeEvent_get_Session(This,ppSession)	\
    ( (This)->lpVtbl -> get_Session(This,ppSession) ) 

#define IRTCSessionStateChangeEvent_get_State(This,penState)	\
    ( (This)->lpVtbl -> get_State(This,penState) ) 

#define IRTCSessionStateChangeEvent_get_StatusCode(This,plStatusCode)	\
    ( (This)->lpVtbl -> get_StatusCode(This,plStatusCode) ) 

#define IRTCSessionStateChangeEvent_get_StatusText(This,pbstrStatusText)	\
    ( (This)->lpVtbl -> get_StatusText(This,pbstrStatusText) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IRTCSessionStateChangeEvent_INTERFACE_DEFINED__ */


#ifndef __IRTCSessionStateChangeEvent2_INTERFACE_DEFINED__
#define __IRTCSessionStateChangeEvent2_INTERFACE_DEFINED__

/* interface IRTCSessionStateChangeEvent2 */
/* [unique][helpstring][dual][uuid][object] */ 


EXTERN_C const IID IID_IRTCSessionStateChangeEvent2;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("4F933171-6F95-4880-80D9-2EC8D495D261")
    IRTCSessionStateChangeEvent2 : public IRTCSessionStateChangeEvent
    {
    public:
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_MediaTypes( 
            /* [retval][out] */ __RPC__out long *pMediaTypes) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_RemotePreferredSecurityLevel( 
            /* [in] */ RTC_SECURITY_TYPE enSecurityType,
            /* [retval][out] */ __RPC__out RTC_SECURITY_LEVEL *penSecurityLevel) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_IsForked( 
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pfIsForked) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE GetRemoteSessionDescription( 
            /* [out] */ __RPC__deref_out_opt BSTR *pbstrContentType,
            /* [out] */ __RPC__deref_out_opt BSTR *pbstrSessionDescription) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IRTCSessionStateChangeEvent2Vtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IRTCSessionStateChangeEvent2 * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IRTCSessionStateChangeEvent2 * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IRTCSessionStateChangeEvent2 * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IRTCSessionStateChangeEvent2 * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IRTCSessionStateChangeEvent2 * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IRTCSessionStateChangeEvent2 * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IRTCSessionStateChangeEvent2 * This,
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
        
        DECLSPEC_XFGVIRT(IRTCSessionStateChangeEvent, get_Session)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Session )( 
            __RPC__in IRTCSessionStateChangeEvent2 * This,
            /* [retval][out] */ __RPC__deref_out_opt IRTCSession **ppSession);
        
        DECLSPEC_XFGVIRT(IRTCSessionStateChangeEvent, get_State)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_State )( 
            __RPC__in IRTCSessionStateChangeEvent2 * This,
            /* [retval][out] */ __RPC__out RTC_SESSION_STATE *penState);
        
        DECLSPEC_XFGVIRT(IRTCSessionStateChangeEvent, get_StatusCode)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_StatusCode )( 
            __RPC__in IRTCSessionStateChangeEvent2 * This,
            /* [retval][out] */ __RPC__out long *plStatusCode);
        
        DECLSPEC_XFGVIRT(IRTCSessionStateChangeEvent, get_StatusText)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_StatusText )( 
            __RPC__in IRTCSessionStateChangeEvent2 * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrStatusText);
        
        DECLSPEC_XFGVIRT(IRTCSessionStateChangeEvent2, get_MediaTypes)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_MediaTypes )( 
            __RPC__in IRTCSessionStateChangeEvent2 * This,
            /* [retval][out] */ __RPC__out long *pMediaTypes);
        
        DECLSPEC_XFGVIRT(IRTCSessionStateChangeEvent2, get_RemotePreferredSecurityLevel)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_RemotePreferredSecurityLevel )( 
            __RPC__in IRTCSessionStateChangeEvent2 * This,
            /* [in] */ RTC_SECURITY_TYPE enSecurityType,
            /* [retval][out] */ __RPC__out RTC_SECURITY_LEVEL *penSecurityLevel);
        
        DECLSPEC_XFGVIRT(IRTCSessionStateChangeEvent2, get_IsForked)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_IsForked )( 
            __RPC__in IRTCSessionStateChangeEvent2 * This,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pfIsForked);
        
        DECLSPEC_XFGVIRT(IRTCSessionStateChangeEvent2, GetRemoteSessionDescription)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *GetRemoteSessionDescription )( 
            __RPC__in IRTCSessionStateChangeEvent2 * This,
            /* [out] */ __RPC__deref_out_opt BSTR *pbstrContentType,
            /* [out] */ __RPC__deref_out_opt BSTR *pbstrSessionDescription);
        
        END_INTERFACE
    } IRTCSessionStateChangeEvent2Vtbl;

    interface IRTCSessionStateChangeEvent2
    {
        CONST_VTBL struct IRTCSessionStateChangeEvent2Vtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IRTCSessionStateChangeEvent2_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IRTCSessionStateChangeEvent2_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IRTCSessionStateChangeEvent2_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IRTCSessionStateChangeEvent2_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IRTCSessionStateChangeEvent2_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IRTCSessionStateChangeEvent2_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IRTCSessionStateChangeEvent2_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IRTCSessionStateChangeEvent2_get_Session(This,ppSession)	\
    ( (This)->lpVtbl -> get_Session(This,ppSession) ) 

#define IRTCSessionStateChangeEvent2_get_State(This,penState)	\
    ( (This)->lpVtbl -> get_State(This,penState) ) 

#define IRTCSessionStateChangeEvent2_get_StatusCode(This,plStatusCode)	\
    ( (This)->lpVtbl -> get_StatusCode(This,plStatusCode) ) 

#define IRTCSessionStateChangeEvent2_get_StatusText(This,pbstrStatusText)	\
    ( (This)->lpVtbl -> get_StatusText(This,pbstrStatusText) ) 


#define IRTCSessionStateChangeEvent2_get_MediaTypes(This,pMediaTypes)	\
    ( (This)->lpVtbl -> get_MediaTypes(This,pMediaTypes) ) 

#define IRTCSessionStateChangeEvent2_get_RemotePreferredSecurityLevel(This,enSecurityType,penSecurityLevel)	\
    ( (This)->lpVtbl -> get_RemotePreferredSecurityLevel(This,enSecurityType,penSecurityLevel) ) 

#define IRTCSessionStateChangeEvent2_get_IsForked(This,pfIsForked)	\
    ( (This)->lpVtbl -> get_IsForked(This,pfIsForked) ) 

#define IRTCSessionStateChangeEvent2_GetRemoteSessionDescription(This,pbstrContentType,pbstrSessionDescription)	\
    ( (This)->lpVtbl -> GetRemoteSessionDescription(This,pbstrContentType,pbstrSessionDescription) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IRTCSessionStateChangeEvent2_INTERFACE_DEFINED__ */


#ifndef __IRTCSessionOperationCompleteEvent_INTERFACE_DEFINED__
#define __IRTCSessionOperationCompleteEvent_INTERFACE_DEFINED__

/* interface IRTCSessionOperationCompleteEvent */
/* [unique][helpstring][dual][uuid][object] */ 


EXTERN_C const IID IID_IRTCSessionOperationCompleteEvent;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("a6bff4c0-f7c8-4d3c-9a41-3550f78a95b0")
    IRTCSessionOperationCompleteEvent : public IDispatch
    {
    public:
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Session( 
            /* [retval][out] */ __RPC__deref_out_opt IRTCSession **ppSession) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Cookie( 
            /* [retval][out] */ __RPC__out LONG_PTR *plCookie) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_StatusCode( 
            /* [retval][out] */ __RPC__out long *plStatusCode) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_StatusText( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrStatusText) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IRTCSessionOperationCompleteEventVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IRTCSessionOperationCompleteEvent * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IRTCSessionOperationCompleteEvent * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IRTCSessionOperationCompleteEvent * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IRTCSessionOperationCompleteEvent * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IRTCSessionOperationCompleteEvent * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IRTCSessionOperationCompleteEvent * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IRTCSessionOperationCompleteEvent * This,
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
        
        DECLSPEC_XFGVIRT(IRTCSessionOperationCompleteEvent, get_Session)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Session )( 
            __RPC__in IRTCSessionOperationCompleteEvent * This,
            /* [retval][out] */ __RPC__deref_out_opt IRTCSession **ppSession);
        
        DECLSPEC_XFGVIRT(IRTCSessionOperationCompleteEvent, get_Cookie)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Cookie )( 
            __RPC__in IRTCSessionOperationCompleteEvent * This,
            /* [retval][out] */ __RPC__out LONG_PTR *plCookie);
        
        DECLSPEC_XFGVIRT(IRTCSessionOperationCompleteEvent, get_StatusCode)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_StatusCode )( 
            __RPC__in IRTCSessionOperationCompleteEvent * This,
            /* [retval][out] */ __RPC__out long *plStatusCode);
        
        DECLSPEC_XFGVIRT(IRTCSessionOperationCompleteEvent, get_StatusText)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_StatusText )( 
            __RPC__in IRTCSessionOperationCompleteEvent * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrStatusText);
        
        END_INTERFACE
    } IRTCSessionOperationCompleteEventVtbl;

    interface IRTCSessionOperationCompleteEvent
    {
        CONST_VTBL struct IRTCSessionOperationCompleteEventVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IRTCSessionOperationCompleteEvent_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IRTCSessionOperationCompleteEvent_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IRTCSessionOperationCompleteEvent_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IRTCSessionOperationCompleteEvent_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IRTCSessionOperationCompleteEvent_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IRTCSessionOperationCompleteEvent_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IRTCSessionOperationCompleteEvent_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IRTCSessionOperationCompleteEvent_get_Session(This,ppSession)	\
    ( (This)->lpVtbl -> get_Session(This,ppSession) ) 

#define IRTCSessionOperationCompleteEvent_get_Cookie(This,plCookie)	\
    ( (This)->lpVtbl -> get_Cookie(This,plCookie) ) 

#define IRTCSessionOperationCompleteEvent_get_StatusCode(This,plStatusCode)	\
    ( (This)->lpVtbl -> get_StatusCode(This,plStatusCode) ) 

#define IRTCSessionOperationCompleteEvent_get_StatusText(This,pbstrStatusText)	\
    ( (This)->lpVtbl -> get_StatusText(This,pbstrStatusText) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IRTCSessionOperationCompleteEvent_INTERFACE_DEFINED__ */


#ifndef __IRTCSessionOperationCompleteEvent2_INTERFACE_DEFINED__
#define __IRTCSessionOperationCompleteEvent2_INTERFACE_DEFINED__

/* interface IRTCSessionOperationCompleteEvent2 */
/* [unique][helpstring][dual][uuid][object] */ 


EXTERN_C const IID IID_IRTCSessionOperationCompleteEvent2;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("f6fc2a9b-d5bc-4241-b436-1b8460c13832")
    IRTCSessionOperationCompleteEvent2 : public IRTCSessionOperationCompleteEvent
    {
    public:
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Participant( 
            /* [retval][out] */ __RPC__deref_out_opt IRTCParticipant **ppParticipant) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE GetRemoteSessionDescription( 
            /* [out] */ __RPC__deref_out_opt BSTR *pbstrContentType,
            /* [out] */ __RPC__deref_out_opt BSTR *pbstrSessionDescription) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IRTCSessionOperationCompleteEvent2Vtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IRTCSessionOperationCompleteEvent2 * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IRTCSessionOperationCompleteEvent2 * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IRTCSessionOperationCompleteEvent2 * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IRTCSessionOperationCompleteEvent2 * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IRTCSessionOperationCompleteEvent2 * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IRTCSessionOperationCompleteEvent2 * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IRTCSessionOperationCompleteEvent2 * This,
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
        
        DECLSPEC_XFGVIRT(IRTCSessionOperationCompleteEvent, get_Session)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Session )( 
            __RPC__in IRTCSessionOperationCompleteEvent2 * This,
            /* [retval][out] */ __RPC__deref_out_opt IRTCSession **ppSession);
        
        DECLSPEC_XFGVIRT(IRTCSessionOperationCompleteEvent, get_Cookie)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Cookie )( 
            __RPC__in IRTCSessionOperationCompleteEvent2 * This,
            /* [retval][out] */ __RPC__out LONG_PTR *plCookie);
        
        DECLSPEC_XFGVIRT(IRTCSessionOperationCompleteEvent, get_StatusCode)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_StatusCode )( 
            __RPC__in IRTCSessionOperationCompleteEvent2 * This,
            /* [retval][out] */ __RPC__out long *plStatusCode);
        
        DECLSPEC_XFGVIRT(IRTCSessionOperationCompleteEvent, get_StatusText)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_StatusText )( 
            __RPC__in IRTCSessionOperationCompleteEvent2 * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrStatusText);
        
        DECLSPEC_XFGVIRT(IRTCSessionOperationCompleteEvent2, get_Participant)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Participant )( 
            __RPC__in IRTCSessionOperationCompleteEvent2 * This,
            /* [retval][out] */ __RPC__deref_out_opt IRTCParticipant **ppParticipant);
        
        DECLSPEC_XFGVIRT(IRTCSessionOperationCompleteEvent2, GetRemoteSessionDescription)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *GetRemoteSessionDescription )( 
            __RPC__in IRTCSessionOperationCompleteEvent2 * This,
            /* [out] */ __RPC__deref_out_opt BSTR *pbstrContentType,
            /* [out] */ __RPC__deref_out_opt BSTR *pbstrSessionDescription);
        
        END_INTERFACE
    } IRTCSessionOperationCompleteEvent2Vtbl;

    interface IRTCSessionOperationCompleteEvent2
    {
        CONST_VTBL struct IRTCSessionOperationCompleteEvent2Vtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IRTCSessionOperationCompleteEvent2_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IRTCSessionOperationCompleteEvent2_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IRTCSessionOperationCompleteEvent2_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IRTCSessionOperationCompleteEvent2_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IRTCSessionOperationCompleteEvent2_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IRTCSessionOperationCompleteEvent2_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IRTCSessionOperationCompleteEvent2_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IRTCSessionOperationCompleteEvent2_get_Session(This,ppSession)	\
    ( (This)->lpVtbl -> get_Session(This,ppSession) ) 

#define IRTCSessionOperationCompleteEvent2_get_Cookie(This,plCookie)	\
    ( (This)->lpVtbl -> get_Cookie(This,plCookie) ) 

#define IRTCSessionOperationCompleteEvent2_get_StatusCode(This,plStatusCode)	\
    ( (This)->lpVtbl -> get_StatusCode(This,plStatusCode) ) 

#define IRTCSessionOperationCompleteEvent2_get_StatusText(This,pbstrStatusText)	\
    ( (This)->lpVtbl -> get_StatusText(This,pbstrStatusText) ) 


#define IRTCSessionOperationCompleteEvent2_get_Participant(This,ppParticipant)	\
    ( (This)->lpVtbl -> get_Participant(This,ppParticipant) ) 

#define IRTCSessionOperationCompleteEvent2_GetRemoteSessionDescription(This,pbstrContentType,pbstrSessionDescription)	\
    ( (This)->lpVtbl -> GetRemoteSessionDescription(This,pbstrContentType,pbstrSessionDescription) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IRTCSessionOperationCompleteEvent2_INTERFACE_DEFINED__ */


#ifndef __IRTCParticipantStateChangeEvent_INTERFACE_DEFINED__
#define __IRTCParticipantStateChangeEvent_INTERFACE_DEFINED__

/* interface IRTCParticipantStateChangeEvent */
/* [unique][helpstring][dual][uuid][object] */ 


EXTERN_C const IID IID_IRTCParticipantStateChangeEvent;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("09bcb597-f0fa-48f9-b420-468cea7fde04")
    IRTCParticipantStateChangeEvent : public IDispatch
    {
    public:
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Participant( 
            /* [retval][out] */ __RPC__deref_out_opt IRTCParticipant **ppParticipant) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_State( 
            /* [retval][out] */ __RPC__out RTC_PARTICIPANT_STATE *penState) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_StatusCode( 
            /* [retval][out] */ __RPC__out long *plStatusCode) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IRTCParticipantStateChangeEventVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IRTCParticipantStateChangeEvent * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IRTCParticipantStateChangeEvent * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IRTCParticipantStateChangeEvent * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IRTCParticipantStateChangeEvent * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IRTCParticipantStateChangeEvent * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IRTCParticipantStateChangeEvent * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IRTCParticipantStateChangeEvent * This,
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
        
        DECLSPEC_XFGVIRT(IRTCParticipantStateChangeEvent, get_Participant)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Participant )( 
            __RPC__in IRTCParticipantStateChangeEvent * This,
            /* [retval][out] */ __RPC__deref_out_opt IRTCParticipant **ppParticipant);
        
        DECLSPEC_XFGVIRT(IRTCParticipantStateChangeEvent, get_State)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_State )( 
            __RPC__in IRTCParticipantStateChangeEvent * This,
            /* [retval][out] */ __RPC__out RTC_PARTICIPANT_STATE *penState);
        
        DECLSPEC_XFGVIRT(IRTCParticipantStateChangeEvent, get_StatusCode)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_StatusCode )( 
            __RPC__in IRTCParticipantStateChangeEvent * This,
            /* [retval][out] */ __RPC__out long *plStatusCode);
        
        END_INTERFACE
    } IRTCParticipantStateChangeEventVtbl;

    interface IRTCParticipantStateChangeEvent
    {
        CONST_VTBL struct IRTCParticipantStateChangeEventVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IRTCParticipantStateChangeEvent_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IRTCParticipantStateChangeEvent_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IRTCParticipantStateChangeEvent_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IRTCParticipantStateChangeEvent_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IRTCParticipantStateChangeEvent_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IRTCParticipantStateChangeEvent_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IRTCParticipantStateChangeEvent_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IRTCParticipantStateChangeEvent_get_Participant(This,ppParticipant)	\
    ( (This)->lpVtbl -> get_Participant(This,ppParticipant) ) 

#define IRTCParticipantStateChangeEvent_get_State(This,penState)	\
    ( (This)->lpVtbl -> get_State(This,penState) ) 

#define IRTCParticipantStateChangeEvent_get_StatusCode(This,plStatusCode)	\
    ( (This)->lpVtbl -> get_StatusCode(This,plStatusCode) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IRTCParticipantStateChangeEvent_INTERFACE_DEFINED__ */


#ifndef __IRTCMediaEvent_INTERFACE_DEFINED__
#define __IRTCMediaEvent_INTERFACE_DEFINED__

/* interface IRTCMediaEvent */
/* [unique][helpstring][dual][uuid][object] */ 


EXTERN_C const IID IID_IRTCMediaEvent;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("099944fb-bcda-453e-8c41-e13da2adf7f3")
    IRTCMediaEvent : public IDispatch
    {
    public:
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_MediaType( 
            /* [retval][out] */ __RPC__out long *pMediaType) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_EventType( 
            /* [retval][out] */ __RPC__out RTC_MEDIA_EVENT_TYPE *penEventType) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_EventReason( 
            /* [retval][out] */ __RPC__out RTC_MEDIA_EVENT_REASON *penEventReason) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IRTCMediaEventVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IRTCMediaEvent * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IRTCMediaEvent * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IRTCMediaEvent * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IRTCMediaEvent * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IRTCMediaEvent * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IRTCMediaEvent * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IRTCMediaEvent * This,
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
        
        DECLSPEC_XFGVIRT(IRTCMediaEvent, get_MediaType)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_MediaType )( 
            __RPC__in IRTCMediaEvent * This,
            /* [retval][out] */ __RPC__out long *pMediaType);
        
        DECLSPEC_XFGVIRT(IRTCMediaEvent, get_EventType)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_EventType )( 
            __RPC__in IRTCMediaEvent * This,
            /* [retval][out] */ __RPC__out RTC_MEDIA_EVENT_TYPE *penEventType);
        
        DECLSPEC_XFGVIRT(IRTCMediaEvent, get_EventReason)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_EventReason )( 
            __RPC__in IRTCMediaEvent * This,
            /* [retval][out] */ __RPC__out RTC_MEDIA_EVENT_REASON *penEventReason);
        
        END_INTERFACE
    } IRTCMediaEventVtbl;

    interface IRTCMediaEvent
    {
        CONST_VTBL struct IRTCMediaEventVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IRTCMediaEvent_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IRTCMediaEvent_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IRTCMediaEvent_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IRTCMediaEvent_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IRTCMediaEvent_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IRTCMediaEvent_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IRTCMediaEvent_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IRTCMediaEvent_get_MediaType(This,pMediaType)	\
    ( (This)->lpVtbl -> get_MediaType(This,pMediaType) ) 

#define IRTCMediaEvent_get_EventType(This,penEventType)	\
    ( (This)->lpVtbl -> get_EventType(This,penEventType) ) 

#define IRTCMediaEvent_get_EventReason(This,penEventReason)	\
    ( (This)->lpVtbl -> get_EventReason(This,penEventReason) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IRTCMediaEvent_INTERFACE_DEFINED__ */


#ifndef __IRTCIntensityEvent_INTERFACE_DEFINED__
#define __IRTCIntensityEvent_INTERFACE_DEFINED__

/* interface IRTCIntensityEvent */
/* [unique][helpstring][dual][uuid][object] */ 


EXTERN_C const IID IID_IRTCIntensityEvent;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("4c23bf51-390c-4992-a41d-41eec05b2a4b")
    IRTCIntensityEvent : public IDispatch
    {
    public:
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Level( 
            /* [retval][out] */ __RPC__out long *plLevel) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Min( 
            /* [retval][out] */ __RPC__out long *plMin) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Max( 
            /* [retval][out] */ __RPC__out long *plMax) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Direction( 
            /* [retval][out] */ __RPC__out RTC_AUDIO_DEVICE *penDirection) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IRTCIntensityEventVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IRTCIntensityEvent * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IRTCIntensityEvent * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IRTCIntensityEvent * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IRTCIntensityEvent * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IRTCIntensityEvent * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IRTCIntensityEvent * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IRTCIntensityEvent * This,
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
        
        DECLSPEC_XFGVIRT(IRTCIntensityEvent, get_Level)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Level )( 
            __RPC__in IRTCIntensityEvent * This,
            /* [retval][out] */ __RPC__out long *plLevel);
        
        DECLSPEC_XFGVIRT(IRTCIntensityEvent, get_Min)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Min )( 
            __RPC__in IRTCIntensityEvent * This,
            /* [retval][out] */ __RPC__out long *plMin);
        
        DECLSPEC_XFGVIRT(IRTCIntensityEvent, get_Max)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Max )( 
            __RPC__in IRTCIntensityEvent * This,
            /* [retval][out] */ __RPC__out long *plMax);
        
        DECLSPEC_XFGVIRT(IRTCIntensityEvent, get_Direction)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Direction )( 
            __RPC__in IRTCIntensityEvent * This,
            /* [retval][out] */ __RPC__out RTC_AUDIO_DEVICE *penDirection);
        
        END_INTERFACE
    } IRTCIntensityEventVtbl;

    interface IRTCIntensityEvent
    {
        CONST_VTBL struct IRTCIntensityEventVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IRTCIntensityEvent_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IRTCIntensityEvent_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IRTCIntensityEvent_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IRTCIntensityEvent_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IRTCIntensityEvent_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IRTCIntensityEvent_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IRTCIntensityEvent_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IRTCIntensityEvent_get_Level(This,plLevel)	\
    ( (This)->lpVtbl -> get_Level(This,plLevel) ) 

#define IRTCIntensityEvent_get_Min(This,plMin)	\
    ( (This)->lpVtbl -> get_Min(This,plMin) ) 

#define IRTCIntensityEvent_get_Max(This,plMax)	\
    ( (This)->lpVtbl -> get_Max(This,plMax) ) 

#define IRTCIntensityEvent_get_Direction(This,penDirection)	\
    ( (This)->lpVtbl -> get_Direction(This,penDirection) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IRTCIntensityEvent_INTERFACE_DEFINED__ */


#ifndef __IRTCMessagingEvent_INTERFACE_DEFINED__
#define __IRTCMessagingEvent_INTERFACE_DEFINED__

/* interface IRTCMessagingEvent */
/* [unique][helpstring][dual][uuid][object] */ 


EXTERN_C const IID IID_IRTCMessagingEvent;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("d3609541-1b29-4de5-a4ad-5aebaf319512")
    IRTCMessagingEvent : public IDispatch
    {
    public:
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Session( 
            /* [retval][out] */ __RPC__deref_out_opt IRTCSession **ppSession) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Participant( 
            /* [retval][out] */ __RPC__deref_out_opt IRTCParticipant **ppParticipant) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_EventType( 
            /* [retval][out] */ __RPC__out RTC_MESSAGING_EVENT_TYPE *penEventType) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Message( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrMessage) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_MessageHeader( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrMessageHeader) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_UserStatus( 
            /* [retval][out] */ __RPC__out RTC_MESSAGING_USER_STATUS *penUserStatus) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IRTCMessagingEventVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IRTCMessagingEvent * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IRTCMessagingEvent * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IRTCMessagingEvent * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IRTCMessagingEvent * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IRTCMessagingEvent * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IRTCMessagingEvent * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IRTCMessagingEvent * This,
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
        
        DECLSPEC_XFGVIRT(IRTCMessagingEvent, get_Session)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Session )( 
            __RPC__in IRTCMessagingEvent * This,
            /* [retval][out] */ __RPC__deref_out_opt IRTCSession **ppSession);
        
        DECLSPEC_XFGVIRT(IRTCMessagingEvent, get_Participant)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Participant )( 
            __RPC__in IRTCMessagingEvent * This,
            /* [retval][out] */ __RPC__deref_out_opt IRTCParticipant **ppParticipant);
        
        DECLSPEC_XFGVIRT(IRTCMessagingEvent, get_EventType)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_EventType )( 
            __RPC__in IRTCMessagingEvent * This,
            /* [retval][out] */ __RPC__out RTC_MESSAGING_EVENT_TYPE *penEventType);
        
        DECLSPEC_XFGVIRT(IRTCMessagingEvent, get_Message)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Message )( 
            __RPC__in IRTCMessagingEvent * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrMessage);
        
        DECLSPEC_XFGVIRT(IRTCMessagingEvent, get_MessageHeader)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_MessageHeader )( 
            __RPC__in IRTCMessagingEvent * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrMessageHeader);
        
        DECLSPEC_XFGVIRT(IRTCMessagingEvent, get_UserStatus)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_UserStatus )( 
            __RPC__in IRTCMessagingEvent * This,
            /* [retval][out] */ __RPC__out RTC_MESSAGING_USER_STATUS *penUserStatus);
        
        END_INTERFACE
    } IRTCMessagingEventVtbl;

    interface IRTCMessagingEvent
    {
        CONST_VTBL struct IRTCMessagingEventVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IRTCMessagingEvent_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IRTCMessagingEvent_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IRTCMessagingEvent_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IRTCMessagingEvent_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IRTCMessagingEvent_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IRTCMessagingEvent_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IRTCMessagingEvent_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IRTCMessagingEvent_get_Session(This,ppSession)	\
    ( (This)->lpVtbl -> get_Session(This,ppSession) ) 

#define IRTCMessagingEvent_get_Participant(This,ppParticipant)	\
    ( (This)->lpVtbl -> get_Participant(This,ppParticipant) ) 

#define IRTCMessagingEvent_get_EventType(This,penEventType)	\
    ( (This)->lpVtbl -> get_EventType(This,penEventType) ) 

#define IRTCMessagingEvent_get_Message(This,pbstrMessage)	\
    ( (This)->lpVtbl -> get_Message(This,pbstrMessage) ) 

#define IRTCMessagingEvent_get_MessageHeader(This,pbstrMessageHeader)	\
    ( (This)->lpVtbl -> get_MessageHeader(This,pbstrMessageHeader) ) 

#define IRTCMessagingEvent_get_UserStatus(This,penUserStatus)	\
    ( (This)->lpVtbl -> get_UserStatus(This,penUserStatus) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IRTCMessagingEvent_INTERFACE_DEFINED__ */


#ifndef __IRTCBuddyEvent_INTERFACE_DEFINED__
#define __IRTCBuddyEvent_INTERFACE_DEFINED__

/* interface IRTCBuddyEvent */
/* [unique][helpstring][dual][uuid][object] */ 


EXTERN_C const IID IID_IRTCBuddyEvent;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("f36d755d-17e6-404e-954f-0fc07574c78d")
    IRTCBuddyEvent : public IDispatch
    {
    public:
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Buddy( 
            /* [retval][out] */ __RPC__deref_out_opt IRTCBuddy **ppBuddy) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IRTCBuddyEventVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IRTCBuddyEvent * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IRTCBuddyEvent * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IRTCBuddyEvent * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IRTCBuddyEvent * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IRTCBuddyEvent * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IRTCBuddyEvent * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IRTCBuddyEvent * This,
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
        
        DECLSPEC_XFGVIRT(IRTCBuddyEvent, get_Buddy)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Buddy )( 
            __RPC__in IRTCBuddyEvent * This,
            /* [retval][out] */ __RPC__deref_out_opt IRTCBuddy **ppBuddy);
        
        END_INTERFACE
    } IRTCBuddyEventVtbl;

    interface IRTCBuddyEvent
    {
        CONST_VTBL struct IRTCBuddyEventVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IRTCBuddyEvent_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IRTCBuddyEvent_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IRTCBuddyEvent_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IRTCBuddyEvent_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IRTCBuddyEvent_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IRTCBuddyEvent_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IRTCBuddyEvent_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IRTCBuddyEvent_get_Buddy(This,ppBuddy)	\
    ( (This)->lpVtbl -> get_Buddy(This,ppBuddy) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IRTCBuddyEvent_INTERFACE_DEFINED__ */


#ifndef __IRTCBuddyEvent2_INTERFACE_DEFINED__
#define __IRTCBuddyEvent2_INTERFACE_DEFINED__

/* interface IRTCBuddyEvent2 */
/* [unique][helpstring][dual][uuid][object] */ 


EXTERN_C const IID IID_IRTCBuddyEvent2;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("484A7F1E-73F0-4990-BFC2-60BC3978A720")
    IRTCBuddyEvent2 : public IRTCBuddyEvent
    {
    public:
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_EventType( 
            /* [retval][out] */ __RPC__out RTC_BUDDY_EVENT_TYPE *pEventType) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_StatusCode( 
            /* [retval][out] */ __RPC__out long *plStatusCode) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_StatusText( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrStatusText) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IRTCBuddyEvent2Vtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IRTCBuddyEvent2 * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IRTCBuddyEvent2 * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IRTCBuddyEvent2 * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IRTCBuddyEvent2 * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IRTCBuddyEvent2 * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IRTCBuddyEvent2 * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IRTCBuddyEvent2 * This,
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
        
        DECLSPEC_XFGVIRT(IRTCBuddyEvent, get_Buddy)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Buddy )( 
            __RPC__in IRTCBuddyEvent2 * This,
            /* [retval][out] */ __RPC__deref_out_opt IRTCBuddy **ppBuddy);
        
        DECLSPEC_XFGVIRT(IRTCBuddyEvent2, get_EventType)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_EventType )( 
            __RPC__in IRTCBuddyEvent2 * This,
            /* [retval][out] */ __RPC__out RTC_BUDDY_EVENT_TYPE *pEventType);
        
        DECLSPEC_XFGVIRT(IRTCBuddyEvent2, get_StatusCode)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_StatusCode )( 
            __RPC__in IRTCBuddyEvent2 * This,
            /* [retval][out] */ __RPC__out long *plStatusCode);
        
        DECLSPEC_XFGVIRT(IRTCBuddyEvent2, get_StatusText)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_StatusText )( 
            __RPC__in IRTCBuddyEvent2 * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrStatusText);
        
        END_INTERFACE
    } IRTCBuddyEvent2Vtbl;

    interface IRTCBuddyEvent2
    {
        CONST_VTBL struct IRTCBuddyEvent2Vtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IRTCBuddyEvent2_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IRTCBuddyEvent2_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IRTCBuddyEvent2_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IRTCBuddyEvent2_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IRTCBuddyEvent2_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IRTCBuddyEvent2_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IRTCBuddyEvent2_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IRTCBuddyEvent2_get_Buddy(This,ppBuddy)	\
    ( (This)->lpVtbl -> get_Buddy(This,ppBuddy) ) 


#define IRTCBuddyEvent2_get_EventType(This,pEventType)	\
    ( (This)->lpVtbl -> get_EventType(This,pEventType) ) 

#define IRTCBuddyEvent2_get_StatusCode(This,plStatusCode)	\
    ( (This)->lpVtbl -> get_StatusCode(This,plStatusCode) ) 

#define IRTCBuddyEvent2_get_StatusText(This,pbstrStatusText)	\
    ( (This)->lpVtbl -> get_StatusText(This,pbstrStatusText) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IRTCBuddyEvent2_INTERFACE_DEFINED__ */


#ifndef __IRTCWatcherEvent_INTERFACE_DEFINED__
#define __IRTCWatcherEvent_INTERFACE_DEFINED__

/* interface IRTCWatcherEvent */
/* [unique][helpstring][dual][uuid][object] */ 


EXTERN_C const IID IID_IRTCWatcherEvent;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("f30d7261-587a-424f-822c-312788f43548")
    IRTCWatcherEvent : public IDispatch
    {
    public:
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Watcher( 
            /* [retval][out] */ __RPC__deref_out_opt IRTCWatcher **ppWatcher) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IRTCWatcherEventVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IRTCWatcherEvent * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IRTCWatcherEvent * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IRTCWatcherEvent * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IRTCWatcherEvent * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IRTCWatcherEvent * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IRTCWatcherEvent * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IRTCWatcherEvent * This,
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
        
        DECLSPEC_XFGVIRT(IRTCWatcherEvent, get_Watcher)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Watcher )( 
            __RPC__in IRTCWatcherEvent * This,
            /* [retval][out] */ __RPC__deref_out_opt IRTCWatcher **ppWatcher);
        
        END_INTERFACE
    } IRTCWatcherEventVtbl;

    interface IRTCWatcherEvent
    {
        CONST_VTBL struct IRTCWatcherEventVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IRTCWatcherEvent_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IRTCWatcherEvent_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IRTCWatcherEvent_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IRTCWatcherEvent_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IRTCWatcherEvent_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IRTCWatcherEvent_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IRTCWatcherEvent_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IRTCWatcherEvent_get_Watcher(This,ppWatcher)	\
    ( (This)->lpVtbl -> get_Watcher(This,ppWatcher) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IRTCWatcherEvent_INTERFACE_DEFINED__ */


#ifndef __IRTCWatcherEvent2_INTERFACE_DEFINED__
#define __IRTCWatcherEvent2_INTERFACE_DEFINED__

/* interface IRTCWatcherEvent2 */
/* [unique][helpstring][dual][uuid][object] */ 


EXTERN_C const IID IID_IRTCWatcherEvent2;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("E52891E8-188C-49AF-B005-98ED13F83F9C")
    IRTCWatcherEvent2 : public IRTCWatcherEvent
    {
    public:
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_EventType( 
            /* [retval][out] */ __RPC__out RTC_WATCHER_EVENT_TYPE *pEventType) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_StatusCode( 
            /* [retval][out] */ __RPC__out long *plStatusCode) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IRTCWatcherEvent2Vtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IRTCWatcherEvent2 * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IRTCWatcherEvent2 * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IRTCWatcherEvent2 * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IRTCWatcherEvent2 * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IRTCWatcherEvent2 * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IRTCWatcherEvent2 * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IRTCWatcherEvent2 * This,
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
        
        DECLSPEC_XFGVIRT(IRTCWatcherEvent, get_Watcher)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Watcher )( 
            __RPC__in IRTCWatcherEvent2 * This,
            /* [retval][out] */ __RPC__deref_out_opt IRTCWatcher **ppWatcher);
        
        DECLSPEC_XFGVIRT(IRTCWatcherEvent2, get_EventType)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_EventType )( 
            __RPC__in IRTCWatcherEvent2 * This,
            /* [retval][out] */ __RPC__out RTC_WATCHER_EVENT_TYPE *pEventType);
        
        DECLSPEC_XFGVIRT(IRTCWatcherEvent2, get_StatusCode)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_StatusCode )( 
            __RPC__in IRTCWatcherEvent2 * This,
            /* [retval][out] */ __RPC__out long *plStatusCode);
        
        END_INTERFACE
    } IRTCWatcherEvent2Vtbl;

    interface IRTCWatcherEvent2
    {
        CONST_VTBL struct IRTCWatcherEvent2Vtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IRTCWatcherEvent2_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IRTCWatcherEvent2_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IRTCWatcherEvent2_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IRTCWatcherEvent2_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IRTCWatcherEvent2_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IRTCWatcherEvent2_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IRTCWatcherEvent2_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IRTCWatcherEvent2_get_Watcher(This,ppWatcher)	\
    ( (This)->lpVtbl -> get_Watcher(This,ppWatcher) ) 


#define IRTCWatcherEvent2_get_EventType(This,pEventType)	\
    ( (This)->lpVtbl -> get_EventType(This,pEventType) ) 

#define IRTCWatcherEvent2_get_StatusCode(This,plStatusCode)	\
    ( (This)->lpVtbl -> get_StatusCode(This,plStatusCode) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IRTCWatcherEvent2_INTERFACE_DEFINED__ */


#ifndef __IRTCBuddyGroupEvent_INTERFACE_DEFINED__
#define __IRTCBuddyGroupEvent_INTERFACE_DEFINED__

/* interface IRTCBuddyGroupEvent */
/* [unique][helpstring][dual][uuid][object] */ 


EXTERN_C const IID IID_IRTCBuddyGroupEvent;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("3A79E1D1-B736-4414-96F8-BBC7F08863E4")
    IRTCBuddyGroupEvent : public IDispatch
    {
    public:
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_EventType( 
            /* [retval][out] */ __RPC__out RTC_GROUP_EVENT_TYPE *pEventType) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Group( 
            /* [retval][out] */ __RPC__deref_out_opt IRTCBuddyGroup **ppGroup) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Buddy( 
            /* [retval][out] */ __RPC__deref_out_opt IRTCBuddy2 **ppBuddy) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_StatusCode( 
            /* [retval][out] */ __RPC__out long *plStatusCode) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IRTCBuddyGroupEventVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IRTCBuddyGroupEvent * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IRTCBuddyGroupEvent * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IRTCBuddyGroupEvent * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IRTCBuddyGroupEvent * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IRTCBuddyGroupEvent * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IRTCBuddyGroupEvent * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IRTCBuddyGroupEvent * This,
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
        
        DECLSPEC_XFGVIRT(IRTCBuddyGroupEvent, get_EventType)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_EventType )( 
            __RPC__in IRTCBuddyGroupEvent * This,
            /* [retval][out] */ __RPC__out RTC_GROUP_EVENT_TYPE *pEventType);
        
        DECLSPEC_XFGVIRT(IRTCBuddyGroupEvent, get_Group)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Group )( 
            __RPC__in IRTCBuddyGroupEvent * This,
            /* [retval][out] */ __RPC__deref_out_opt IRTCBuddyGroup **ppGroup);
        
        DECLSPEC_XFGVIRT(IRTCBuddyGroupEvent, get_Buddy)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Buddy )( 
            __RPC__in IRTCBuddyGroupEvent * This,
            /* [retval][out] */ __RPC__deref_out_opt IRTCBuddy2 **ppBuddy);
        
        DECLSPEC_XFGVIRT(IRTCBuddyGroupEvent, get_StatusCode)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_StatusCode )( 
            __RPC__in IRTCBuddyGroupEvent * This,
            /* [retval][out] */ __RPC__out long *plStatusCode);
        
        END_INTERFACE
    } IRTCBuddyGroupEventVtbl;

    interface IRTCBuddyGroupEvent
    {
        CONST_VTBL struct IRTCBuddyGroupEventVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IRTCBuddyGroupEvent_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IRTCBuddyGroupEvent_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IRTCBuddyGroupEvent_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IRTCBuddyGroupEvent_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IRTCBuddyGroupEvent_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IRTCBuddyGroupEvent_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IRTCBuddyGroupEvent_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IRTCBuddyGroupEvent_get_EventType(This,pEventType)	\
    ( (This)->lpVtbl -> get_EventType(This,pEventType) ) 

#define IRTCBuddyGroupEvent_get_Group(This,ppGroup)	\
    ( (This)->lpVtbl -> get_Group(This,ppGroup) ) 

#define IRTCBuddyGroupEvent_get_Buddy(This,ppBuddy)	\
    ( (This)->lpVtbl -> get_Buddy(This,ppBuddy) ) 

#define IRTCBuddyGroupEvent_get_StatusCode(This,plStatusCode)	\
    ( (This)->lpVtbl -> get_StatusCode(This,plStatusCode) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IRTCBuddyGroupEvent_INTERFACE_DEFINED__ */


#ifndef __IRTCInfoEvent_INTERFACE_DEFINED__
#define __IRTCInfoEvent_INTERFACE_DEFINED__

/* interface IRTCInfoEvent */
/* [unique][helpstring][dual][uuid][object] */ 


EXTERN_C const IID IID_IRTCInfoEvent;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("4E1D68AE-1912-4f49-B2C3-594FADFD425F")
    IRTCInfoEvent : public IDispatch
    {
    public:
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Session( 
            /* [retval][out] */ __RPC__deref_out_opt IRTCSession2 **ppSession) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Participant( 
            /* [retval][out] */ __RPC__deref_out_opt IRTCParticipant **ppParticipant) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Info( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrInfo) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_InfoHeader( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrInfoHeader) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IRTCInfoEventVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IRTCInfoEvent * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IRTCInfoEvent * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IRTCInfoEvent * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IRTCInfoEvent * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IRTCInfoEvent * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IRTCInfoEvent * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IRTCInfoEvent * This,
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
        
        DECLSPEC_XFGVIRT(IRTCInfoEvent, get_Session)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Session )( 
            __RPC__in IRTCInfoEvent * This,
            /* [retval][out] */ __RPC__deref_out_opt IRTCSession2 **ppSession);
        
        DECLSPEC_XFGVIRT(IRTCInfoEvent, get_Participant)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Participant )( 
            __RPC__in IRTCInfoEvent * This,
            /* [retval][out] */ __RPC__deref_out_opt IRTCParticipant **ppParticipant);
        
        DECLSPEC_XFGVIRT(IRTCInfoEvent, get_Info)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Info )( 
            __RPC__in IRTCInfoEvent * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrInfo);
        
        DECLSPEC_XFGVIRT(IRTCInfoEvent, get_InfoHeader)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_InfoHeader )( 
            __RPC__in IRTCInfoEvent * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrInfoHeader);
        
        END_INTERFACE
    } IRTCInfoEventVtbl;

    interface IRTCInfoEvent
    {
        CONST_VTBL struct IRTCInfoEventVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IRTCInfoEvent_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IRTCInfoEvent_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IRTCInfoEvent_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IRTCInfoEvent_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IRTCInfoEvent_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IRTCInfoEvent_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IRTCInfoEvent_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IRTCInfoEvent_get_Session(This,ppSession)	\
    ( (This)->lpVtbl -> get_Session(This,ppSession) ) 

#define IRTCInfoEvent_get_Participant(This,ppParticipant)	\
    ( (This)->lpVtbl -> get_Participant(This,ppParticipant) ) 

#define IRTCInfoEvent_get_Info(This,pbstrInfo)	\
    ( (This)->lpVtbl -> get_Info(This,pbstrInfo) ) 

#define IRTCInfoEvent_get_InfoHeader(This,pbstrInfoHeader)	\
    ( (This)->lpVtbl -> get_InfoHeader(This,pbstrInfoHeader) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IRTCInfoEvent_INTERFACE_DEFINED__ */


#ifndef __IRTCMediaRequestEvent_INTERFACE_DEFINED__
#define __IRTCMediaRequestEvent_INTERFACE_DEFINED__

/* interface IRTCMediaRequestEvent */
/* [unique][helpstring][dual][uuid][object] */ 


EXTERN_C const IID IID_IRTCMediaRequestEvent;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("52572D15-148C-4d97-A36C-2DA55C289D63")
    IRTCMediaRequestEvent : public IDispatch
    {
    public:
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Session( 
            /* [retval][out] */ __RPC__deref_out_opt IRTCSession2 **ppSession) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_ProposedMedia( 
            /* [retval][out] */ __RPC__out long *plMediaTypes) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_CurrentMedia( 
            /* [retval][out] */ __RPC__out long *plMediaTypes) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE Accept( 
            /* [in] */ long lMediaTypes) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_RemotePreferredSecurityLevel( 
            /* [in] */ RTC_SECURITY_TYPE enSecurityType,
            /* [retval][out] */ __RPC__out RTC_SECURITY_LEVEL *penSecurityLevel) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE Reject( void) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_State( 
            /* [retval][out] */ __RPC__out RTC_REINVITE_STATE *pState) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IRTCMediaRequestEventVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IRTCMediaRequestEvent * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IRTCMediaRequestEvent * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IRTCMediaRequestEvent * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IRTCMediaRequestEvent * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IRTCMediaRequestEvent * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IRTCMediaRequestEvent * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IRTCMediaRequestEvent * This,
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
        
        DECLSPEC_XFGVIRT(IRTCMediaRequestEvent, get_Session)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Session )( 
            __RPC__in IRTCMediaRequestEvent * This,
            /* [retval][out] */ __RPC__deref_out_opt IRTCSession2 **ppSession);
        
        DECLSPEC_XFGVIRT(IRTCMediaRequestEvent, get_ProposedMedia)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_ProposedMedia )( 
            __RPC__in IRTCMediaRequestEvent * This,
            /* [retval][out] */ __RPC__out long *plMediaTypes);
        
        DECLSPEC_XFGVIRT(IRTCMediaRequestEvent, get_CurrentMedia)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_CurrentMedia )( 
            __RPC__in IRTCMediaRequestEvent * This,
            /* [retval][out] */ __RPC__out long *plMediaTypes);
        
        DECLSPEC_XFGVIRT(IRTCMediaRequestEvent, Accept)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *Accept )( 
            __RPC__in IRTCMediaRequestEvent * This,
            /* [in] */ long lMediaTypes);
        
        DECLSPEC_XFGVIRT(IRTCMediaRequestEvent, get_RemotePreferredSecurityLevel)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_RemotePreferredSecurityLevel )( 
            __RPC__in IRTCMediaRequestEvent * This,
            /* [in] */ RTC_SECURITY_TYPE enSecurityType,
            /* [retval][out] */ __RPC__out RTC_SECURITY_LEVEL *penSecurityLevel);
        
        DECLSPEC_XFGVIRT(IRTCMediaRequestEvent, Reject)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *Reject )( 
            __RPC__in IRTCMediaRequestEvent * This);
        
        DECLSPEC_XFGVIRT(IRTCMediaRequestEvent, get_State)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_State )( 
            __RPC__in IRTCMediaRequestEvent * This,
            /* [retval][out] */ __RPC__out RTC_REINVITE_STATE *pState);
        
        END_INTERFACE
    } IRTCMediaRequestEventVtbl;

    interface IRTCMediaRequestEvent
    {
        CONST_VTBL struct IRTCMediaRequestEventVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IRTCMediaRequestEvent_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IRTCMediaRequestEvent_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IRTCMediaRequestEvent_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IRTCMediaRequestEvent_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IRTCMediaRequestEvent_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IRTCMediaRequestEvent_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IRTCMediaRequestEvent_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IRTCMediaRequestEvent_get_Session(This,ppSession)	\
    ( (This)->lpVtbl -> get_Session(This,ppSession) ) 

#define IRTCMediaRequestEvent_get_ProposedMedia(This,plMediaTypes)	\
    ( (This)->lpVtbl -> get_ProposedMedia(This,plMediaTypes) ) 

#define IRTCMediaRequestEvent_get_CurrentMedia(This,plMediaTypes)	\
    ( (This)->lpVtbl -> get_CurrentMedia(This,plMediaTypes) ) 

#define IRTCMediaRequestEvent_Accept(This,lMediaTypes)	\
    ( (This)->lpVtbl -> Accept(This,lMediaTypes) ) 

#define IRTCMediaRequestEvent_get_RemotePreferredSecurityLevel(This,enSecurityType,penSecurityLevel)	\
    ( (This)->lpVtbl -> get_RemotePreferredSecurityLevel(This,enSecurityType,penSecurityLevel) ) 

#define IRTCMediaRequestEvent_Reject(This)	\
    ( (This)->lpVtbl -> Reject(This) ) 

#define IRTCMediaRequestEvent_get_State(This,pState)	\
    ( (This)->lpVtbl -> get_State(This,pState) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IRTCMediaRequestEvent_INTERFACE_DEFINED__ */


#ifndef __IRTCReInviteEvent_INTERFACE_DEFINED__
#define __IRTCReInviteEvent_INTERFACE_DEFINED__

/* interface IRTCReInviteEvent */
/* [unique][helpstring][dual][uuid][object] */ 


EXTERN_C const IID IID_IRTCReInviteEvent;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("11558D84-204C-43e7-99B0-2034E9417F7D")
    IRTCReInviteEvent : public IDispatch
    {
    public:
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Session( 
            /* [retval][out] */ __RPC__deref_out_opt IRTCSession2 **ppSession2) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE Accept( 
            /* [in] */ __RPC__in BSTR bstrContentType,
            /* [in] */ __RPC__in BSTR bstrSessionDescription) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE Reject( void) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_State( 
            /* [retval][out] */ __RPC__out RTC_REINVITE_STATE *pState) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE GetRemoteSessionDescription( 
            /* [out] */ __RPC__deref_out_opt BSTR *pbstrContentType,
            /* [out] */ __RPC__deref_out_opt BSTR *pbstrSessionDescription) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IRTCReInviteEventVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IRTCReInviteEvent * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IRTCReInviteEvent * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IRTCReInviteEvent * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IRTCReInviteEvent * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IRTCReInviteEvent * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IRTCReInviteEvent * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IRTCReInviteEvent * This,
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
        
        DECLSPEC_XFGVIRT(IRTCReInviteEvent, get_Session)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Session )( 
            __RPC__in IRTCReInviteEvent * This,
            /* [retval][out] */ __RPC__deref_out_opt IRTCSession2 **ppSession2);
        
        DECLSPEC_XFGVIRT(IRTCReInviteEvent, Accept)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *Accept )( 
            __RPC__in IRTCReInviteEvent * This,
            /* [in] */ __RPC__in BSTR bstrContentType,
            /* [in] */ __RPC__in BSTR bstrSessionDescription);
        
        DECLSPEC_XFGVIRT(IRTCReInviteEvent, Reject)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *Reject )( 
            __RPC__in IRTCReInviteEvent * This);
        
        DECLSPEC_XFGVIRT(IRTCReInviteEvent, get_State)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_State )( 
            __RPC__in IRTCReInviteEvent * This,
            /* [retval][out] */ __RPC__out RTC_REINVITE_STATE *pState);
        
        DECLSPEC_XFGVIRT(IRTCReInviteEvent, GetRemoteSessionDescription)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *GetRemoteSessionDescription )( 
            __RPC__in IRTCReInviteEvent * This,
            /* [out] */ __RPC__deref_out_opt BSTR *pbstrContentType,
            /* [out] */ __RPC__deref_out_opt BSTR *pbstrSessionDescription);
        
        END_INTERFACE
    } IRTCReInviteEventVtbl;

    interface IRTCReInviteEvent
    {
        CONST_VTBL struct IRTCReInviteEventVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IRTCReInviteEvent_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IRTCReInviteEvent_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IRTCReInviteEvent_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IRTCReInviteEvent_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IRTCReInviteEvent_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IRTCReInviteEvent_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IRTCReInviteEvent_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IRTCReInviteEvent_get_Session(This,ppSession2)	\
    ( (This)->lpVtbl -> get_Session(This,ppSession2) ) 

#define IRTCReInviteEvent_Accept(This,bstrContentType,bstrSessionDescription)	\
    ( (This)->lpVtbl -> Accept(This,bstrContentType,bstrSessionDescription) ) 

#define IRTCReInviteEvent_Reject(This)	\
    ( (This)->lpVtbl -> Reject(This) ) 

#define IRTCReInviteEvent_get_State(This,pState)	\
    ( (This)->lpVtbl -> get_State(This,pState) ) 

#define IRTCReInviteEvent_GetRemoteSessionDescription(This,pbstrContentType,pbstrSessionDescription)	\
    ( (This)->lpVtbl -> GetRemoteSessionDescription(This,pbstrContentType,pbstrSessionDescription) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IRTCReInviteEvent_INTERFACE_DEFINED__ */


#ifndef __IRTCPresencePropertyEvent_INTERFACE_DEFINED__
#define __IRTCPresencePropertyEvent_INTERFACE_DEFINED__

/* interface IRTCPresencePropertyEvent */
/* [unique][helpstring][dual][uuid][object] */ 


EXTERN_C const IID IID_IRTCPresencePropertyEvent;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("F777F570-A820-49d5-86BD-E099493F1518")
    IRTCPresencePropertyEvent : public IDispatch
    {
    public:
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_StatusCode( 
            /* [retval][out] */ __RPC__out long *plStatusCode) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_StatusText( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrStatusText) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_PresenceProperty( 
            /* [retval][out] */ __RPC__out RTC_PRESENCE_PROPERTY *penPresProp) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Value( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrValue) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IRTCPresencePropertyEventVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IRTCPresencePropertyEvent * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IRTCPresencePropertyEvent * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IRTCPresencePropertyEvent * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IRTCPresencePropertyEvent * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IRTCPresencePropertyEvent * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IRTCPresencePropertyEvent * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IRTCPresencePropertyEvent * This,
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
        
        DECLSPEC_XFGVIRT(IRTCPresencePropertyEvent, get_StatusCode)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_StatusCode )( 
            __RPC__in IRTCPresencePropertyEvent * This,
            /* [retval][out] */ __RPC__out long *plStatusCode);
        
        DECLSPEC_XFGVIRT(IRTCPresencePropertyEvent, get_StatusText)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_StatusText )( 
            __RPC__in IRTCPresencePropertyEvent * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrStatusText);
        
        DECLSPEC_XFGVIRT(IRTCPresencePropertyEvent, get_PresenceProperty)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_PresenceProperty )( 
            __RPC__in IRTCPresencePropertyEvent * This,
            /* [retval][out] */ __RPC__out RTC_PRESENCE_PROPERTY *penPresProp);
        
        DECLSPEC_XFGVIRT(IRTCPresencePropertyEvent, get_Value)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Value )( 
            __RPC__in IRTCPresencePropertyEvent * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrValue);
        
        END_INTERFACE
    } IRTCPresencePropertyEventVtbl;

    interface IRTCPresencePropertyEvent
    {
        CONST_VTBL struct IRTCPresencePropertyEventVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IRTCPresencePropertyEvent_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IRTCPresencePropertyEvent_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IRTCPresencePropertyEvent_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IRTCPresencePropertyEvent_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IRTCPresencePropertyEvent_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IRTCPresencePropertyEvent_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IRTCPresencePropertyEvent_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IRTCPresencePropertyEvent_get_StatusCode(This,plStatusCode)	\
    ( (This)->lpVtbl -> get_StatusCode(This,plStatusCode) ) 

#define IRTCPresencePropertyEvent_get_StatusText(This,pbstrStatusText)	\
    ( (This)->lpVtbl -> get_StatusText(This,pbstrStatusText) ) 

#define IRTCPresencePropertyEvent_get_PresenceProperty(This,penPresProp)	\
    ( (This)->lpVtbl -> get_PresenceProperty(This,penPresProp) ) 

#define IRTCPresencePropertyEvent_get_Value(This,pbstrValue)	\
    ( (This)->lpVtbl -> get_Value(This,pbstrValue) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IRTCPresencePropertyEvent_INTERFACE_DEFINED__ */


#ifndef __IRTCPresenceDataEvent_INTERFACE_DEFINED__
#define __IRTCPresenceDataEvent_INTERFACE_DEFINED__

/* interface IRTCPresenceDataEvent */
/* [unique][helpstring][dual][uuid][object] */ 


EXTERN_C const IID IID_IRTCPresenceDataEvent;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("38F0E78C-8B87-4c04-A82D-AEDD83C909BB")
    IRTCPresenceDataEvent : public IDispatch
    {
    public:
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_StatusCode( 
            /* [retval][out] */ __RPC__out long *plStatusCode) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_StatusText( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrStatusText) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE GetPresenceData( 
            /* [out] */ __RPC__deref_out_opt BSTR *pbstrNamespace,
            /* [out] */ __RPC__deref_out_opt BSTR *pbstrData) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IRTCPresenceDataEventVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IRTCPresenceDataEvent * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IRTCPresenceDataEvent * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IRTCPresenceDataEvent * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IRTCPresenceDataEvent * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IRTCPresenceDataEvent * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IRTCPresenceDataEvent * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IRTCPresenceDataEvent * This,
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
        
        DECLSPEC_XFGVIRT(IRTCPresenceDataEvent, get_StatusCode)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_StatusCode )( 
            __RPC__in IRTCPresenceDataEvent * This,
            /* [retval][out] */ __RPC__out long *plStatusCode);
        
        DECLSPEC_XFGVIRT(IRTCPresenceDataEvent, get_StatusText)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_StatusText )( 
            __RPC__in IRTCPresenceDataEvent * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrStatusText);
        
        DECLSPEC_XFGVIRT(IRTCPresenceDataEvent, GetPresenceData)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *GetPresenceData )( 
            __RPC__in IRTCPresenceDataEvent * This,
            /* [out] */ __RPC__deref_out_opt BSTR *pbstrNamespace,
            /* [out] */ __RPC__deref_out_opt BSTR *pbstrData);
        
        END_INTERFACE
    } IRTCPresenceDataEventVtbl;

    interface IRTCPresenceDataEvent
    {
        CONST_VTBL struct IRTCPresenceDataEventVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IRTCPresenceDataEvent_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IRTCPresenceDataEvent_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IRTCPresenceDataEvent_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IRTCPresenceDataEvent_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IRTCPresenceDataEvent_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IRTCPresenceDataEvent_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IRTCPresenceDataEvent_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IRTCPresenceDataEvent_get_StatusCode(This,plStatusCode)	\
    ( (This)->lpVtbl -> get_StatusCode(This,plStatusCode) ) 

#define IRTCPresenceDataEvent_get_StatusText(This,pbstrStatusText)	\
    ( (This)->lpVtbl -> get_StatusText(This,pbstrStatusText) ) 

#define IRTCPresenceDataEvent_GetPresenceData(This,pbstrNamespace,pbstrData)	\
    ( (This)->lpVtbl -> GetPresenceData(This,pbstrNamespace,pbstrData) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IRTCPresenceDataEvent_INTERFACE_DEFINED__ */


#ifndef __IRTCPresenceStatusEvent_INTERFACE_DEFINED__
#define __IRTCPresenceStatusEvent_INTERFACE_DEFINED__

/* interface IRTCPresenceStatusEvent */
/* [unique][helpstring][dual][uuid][object] */ 


EXTERN_C const IID IID_IRTCPresenceStatusEvent;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("78673F32-4A0F-462c-89AA-EE7706707678")
    IRTCPresenceStatusEvent : public IDispatch
    {
    public:
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_StatusCode( 
            /* [retval][out] */ __RPC__out long *plStatusCode) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_StatusText( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrStatusText) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE GetLocalPresenceInfo( 
            /* [out] */ __RPC__out RTC_PRESENCE_STATUS *penStatus,
            /* [out] */ __RPC__deref_out_opt BSTR *pbstrNotes) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IRTCPresenceStatusEventVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IRTCPresenceStatusEvent * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IRTCPresenceStatusEvent * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IRTCPresenceStatusEvent * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IRTCPresenceStatusEvent * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IRTCPresenceStatusEvent * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IRTCPresenceStatusEvent * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IRTCPresenceStatusEvent * This,
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
        
        DECLSPEC_XFGVIRT(IRTCPresenceStatusEvent, get_StatusCode)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_StatusCode )( 
            __RPC__in IRTCPresenceStatusEvent * This,
            /* [retval][out] */ __RPC__out long *plStatusCode);
        
        DECLSPEC_XFGVIRT(IRTCPresenceStatusEvent, get_StatusText)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_StatusText )( 
            __RPC__in IRTCPresenceStatusEvent * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrStatusText);
        
        DECLSPEC_XFGVIRT(IRTCPresenceStatusEvent, GetLocalPresenceInfo)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *GetLocalPresenceInfo )( 
            __RPC__in IRTCPresenceStatusEvent * This,
            /* [out] */ __RPC__out RTC_PRESENCE_STATUS *penStatus,
            /* [out] */ __RPC__deref_out_opt BSTR *pbstrNotes);
        
        END_INTERFACE
    } IRTCPresenceStatusEventVtbl;

    interface IRTCPresenceStatusEvent
    {
        CONST_VTBL struct IRTCPresenceStatusEventVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IRTCPresenceStatusEvent_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IRTCPresenceStatusEvent_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IRTCPresenceStatusEvent_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IRTCPresenceStatusEvent_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IRTCPresenceStatusEvent_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IRTCPresenceStatusEvent_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IRTCPresenceStatusEvent_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IRTCPresenceStatusEvent_get_StatusCode(This,plStatusCode)	\
    ( (This)->lpVtbl -> get_StatusCode(This,plStatusCode) ) 

#define IRTCPresenceStatusEvent_get_StatusText(This,pbstrStatusText)	\
    ( (This)->lpVtbl -> get_StatusText(This,pbstrStatusText) ) 

#define IRTCPresenceStatusEvent_GetLocalPresenceInfo(This,penStatus,pbstrNotes)	\
    ( (This)->lpVtbl -> GetLocalPresenceInfo(This,penStatus,pbstrNotes) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IRTCPresenceStatusEvent_INTERFACE_DEFINED__ */


#ifndef __IRTCCollection_INTERFACE_DEFINED__
#define __IRTCCollection_INTERFACE_DEFINED__

/* interface IRTCCollection */
/* [dual][helpstring][uuid][object] */ 


EXTERN_C const IID IID_IRTCCollection;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("EC7C8096-B918-4044-94F1-E4FBA0361D5C")
    IRTCCollection : public IDispatch
    {
    public:
        virtual /* [helpstring][propget] */ HRESULT STDMETHODCALLTYPE get_Count( 
            /* [retval][out] */ __RPC__out long *lCount) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Item( 
            /* [in] */ long Index,
            /* [retval][out] */ __RPC__out VARIANT *pVariant) = 0;
        
        virtual /* [helpstring][id][restricted][propget] */ HRESULT STDMETHODCALLTYPE get__NewEnum( 
            /* [retval][out] */ __RPC__deref_out_opt IUnknown **ppNewEnum) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IRTCCollectionVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IRTCCollection * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IRTCCollection * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IRTCCollection * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IRTCCollection * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IRTCCollection * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IRTCCollection * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IRTCCollection * This,
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
        
        DECLSPEC_XFGVIRT(IRTCCollection, get_Count)
        /* [helpstring][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Count )( 
            __RPC__in IRTCCollection * This,
            /* [retval][out] */ __RPC__out long *lCount);
        
        DECLSPEC_XFGVIRT(IRTCCollection, get_Item)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Item )( 
            __RPC__in IRTCCollection * This,
            /* [in] */ long Index,
            /* [retval][out] */ __RPC__out VARIANT *pVariant);
        
        DECLSPEC_XFGVIRT(IRTCCollection, get__NewEnum)
        /* [helpstring][id][restricted][propget] */ HRESULT ( STDMETHODCALLTYPE *get__NewEnum )( 
            __RPC__in IRTCCollection * This,
            /* [retval][out] */ __RPC__deref_out_opt IUnknown **ppNewEnum);
        
        END_INTERFACE
    } IRTCCollectionVtbl;

    interface IRTCCollection
    {
        CONST_VTBL struct IRTCCollectionVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IRTCCollection_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IRTCCollection_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IRTCCollection_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IRTCCollection_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IRTCCollection_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IRTCCollection_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IRTCCollection_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IRTCCollection_get_Count(This,lCount)	\
    ( (This)->lpVtbl -> get_Count(This,lCount) ) 

#define IRTCCollection_get_Item(This,Index,pVariant)	\
    ( (This)->lpVtbl -> get_Item(This,Index,pVariant) ) 

#define IRTCCollection_get__NewEnum(This,ppNewEnum)	\
    ( (This)->lpVtbl -> get__NewEnum(This,ppNewEnum) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IRTCCollection_INTERFACE_DEFINED__ */


#ifndef __IRTCEnumParticipants_INTERFACE_DEFINED__
#define __IRTCEnumParticipants_INTERFACE_DEFINED__

/* interface IRTCEnumParticipants */
/* [unique][helpstring][hidden][uuid][object] */ 


EXTERN_C const IID IID_IRTCEnumParticipants;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("fcd56f29-4a4f-41b2-ba5c-f5bccc060bf6")
    IRTCEnumParticipants : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE Next( 
            /* [in] */ ULONG celt,
            /* [size_is][out] */ __RPC__out_ecount_full(celt) IRTCParticipant **ppElements,
            /* [full][out][in] */ __RPC__inout_opt ULONG *pceltFetched) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Reset( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Skip( 
            /* [in] */ ULONG celt) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Clone( 
            /* [retval][out] */ __RPC__deref_out_opt IRTCEnumParticipants **ppEnum) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IRTCEnumParticipantsVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IRTCEnumParticipants * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IRTCEnumParticipants * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IRTCEnumParticipants * This);
        
        DECLSPEC_XFGVIRT(IRTCEnumParticipants, Next)
        HRESULT ( STDMETHODCALLTYPE *Next )( 
            __RPC__in IRTCEnumParticipants * This,
            /* [in] */ ULONG celt,
            /* [size_is][out] */ __RPC__out_ecount_full(celt) IRTCParticipant **ppElements,
            /* [full][out][in] */ __RPC__inout_opt ULONG *pceltFetched);
        
        DECLSPEC_XFGVIRT(IRTCEnumParticipants, Reset)
        HRESULT ( STDMETHODCALLTYPE *Reset )( 
            __RPC__in IRTCEnumParticipants * This);
        
        DECLSPEC_XFGVIRT(IRTCEnumParticipants, Skip)
        HRESULT ( STDMETHODCALLTYPE *Skip )( 
            __RPC__in IRTCEnumParticipants * This,
            /* [in] */ ULONG celt);
        
        DECLSPEC_XFGVIRT(IRTCEnumParticipants, Clone)
        HRESULT ( STDMETHODCALLTYPE *Clone )( 
            __RPC__in IRTCEnumParticipants * This,
            /* [retval][out] */ __RPC__deref_out_opt IRTCEnumParticipants **ppEnum);
        
        END_INTERFACE
    } IRTCEnumParticipantsVtbl;

    interface IRTCEnumParticipants
    {
        CONST_VTBL struct IRTCEnumParticipantsVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IRTCEnumParticipants_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IRTCEnumParticipants_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IRTCEnumParticipants_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IRTCEnumParticipants_Next(This,celt,ppElements,pceltFetched)	\
    ( (This)->lpVtbl -> Next(This,celt,ppElements,pceltFetched) ) 

#define IRTCEnumParticipants_Reset(This)	\
    ( (This)->lpVtbl -> Reset(This) ) 

#define IRTCEnumParticipants_Skip(This,celt)	\
    ( (This)->lpVtbl -> Skip(This,celt) ) 

#define IRTCEnumParticipants_Clone(This,ppEnum)	\
    ( (This)->lpVtbl -> Clone(This,ppEnum) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IRTCEnumParticipants_INTERFACE_DEFINED__ */


#ifndef __IRTCEnumProfiles_INTERFACE_DEFINED__
#define __IRTCEnumProfiles_INTERFACE_DEFINED__

/* interface IRTCEnumProfiles */
/* [unique][helpstring][hidden][uuid][object] */ 


EXTERN_C const IID IID_IRTCEnumProfiles;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("29b7c41c-ed82-4bca-84ad-39d5101b58e3")
    IRTCEnumProfiles : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE Next( 
            /* [in] */ ULONG celt,
            /* [size_is][out] */ __RPC__out_ecount_full(celt) IRTCProfile **ppElements,
            /* [full][out][in] */ __RPC__inout_opt ULONG *pceltFetched) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Reset( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Skip( 
            /* [in] */ ULONG celt) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Clone( 
            /* [retval][out] */ __RPC__deref_out_opt IRTCEnumProfiles **ppEnum) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IRTCEnumProfilesVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IRTCEnumProfiles * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IRTCEnumProfiles * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IRTCEnumProfiles * This);
        
        DECLSPEC_XFGVIRT(IRTCEnumProfiles, Next)
        HRESULT ( STDMETHODCALLTYPE *Next )( 
            __RPC__in IRTCEnumProfiles * This,
            /* [in] */ ULONG celt,
            /* [size_is][out] */ __RPC__out_ecount_full(celt) IRTCProfile **ppElements,
            /* [full][out][in] */ __RPC__inout_opt ULONG *pceltFetched);
        
        DECLSPEC_XFGVIRT(IRTCEnumProfiles, Reset)
        HRESULT ( STDMETHODCALLTYPE *Reset )( 
            __RPC__in IRTCEnumProfiles * This);
        
        DECLSPEC_XFGVIRT(IRTCEnumProfiles, Skip)
        HRESULT ( STDMETHODCALLTYPE *Skip )( 
            __RPC__in IRTCEnumProfiles * This,
            /* [in] */ ULONG celt);
        
        DECLSPEC_XFGVIRT(IRTCEnumProfiles, Clone)
        HRESULT ( STDMETHODCALLTYPE *Clone )( 
            __RPC__in IRTCEnumProfiles * This,
            /* [retval][out] */ __RPC__deref_out_opt IRTCEnumProfiles **ppEnum);
        
        END_INTERFACE
    } IRTCEnumProfilesVtbl;

    interface IRTCEnumProfiles
    {
        CONST_VTBL struct IRTCEnumProfilesVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IRTCEnumProfiles_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IRTCEnumProfiles_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IRTCEnumProfiles_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IRTCEnumProfiles_Next(This,celt,ppElements,pceltFetched)	\
    ( (This)->lpVtbl -> Next(This,celt,ppElements,pceltFetched) ) 

#define IRTCEnumProfiles_Reset(This)	\
    ( (This)->lpVtbl -> Reset(This) ) 

#define IRTCEnumProfiles_Skip(This,celt)	\
    ( (This)->lpVtbl -> Skip(This,celt) ) 

#define IRTCEnumProfiles_Clone(This,ppEnum)	\
    ( (This)->lpVtbl -> Clone(This,ppEnum) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IRTCEnumProfiles_INTERFACE_DEFINED__ */


#ifndef __IRTCEnumBuddies_INTERFACE_DEFINED__
#define __IRTCEnumBuddies_INTERFACE_DEFINED__

/* interface IRTCEnumBuddies */
/* [unique][helpstring][hidden][uuid][object] */ 


EXTERN_C const IID IID_IRTCEnumBuddies;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("f7296917-5569-4b3b-b3af-98d1144b2b87")
    IRTCEnumBuddies : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE Next( 
            /* [in] */ ULONG celt,
            /* [size_is][out] */ __RPC__out_ecount_full(celt) IRTCBuddy **ppElements,
            /* [full][out][in] */ __RPC__inout_opt ULONG *pceltFetched) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Reset( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Skip( 
            /* [in] */ ULONG celt) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Clone( 
            /* [retval][out] */ __RPC__deref_out_opt IRTCEnumBuddies **ppEnum) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IRTCEnumBuddiesVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IRTCEnumBuddies * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IRTCEnumBuddies * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IRTCEnumBuddies * This);
        
        DECLSPEC_XFGVIRT(IRTCEnumBuddies, Next)
        HRESULT ( STDMETHODCALLTYPE *Next )( 
            __RPC__in IRTCEnumBuddies * This,
            /* [in] */ ULONG celt,
            /* [size_is][out] */ __RPC__out_ecount_full(celt) IRTCBuddy **ppElements,
            /* [full][out][in] */ __RPC__inout_opt ULONG *pceltFetched);
        
        DECLSPEC_XFGVIRT(IRTCEnumBuddies, Reset)
        HRESULT ( STDMETHODCALLTYPE *Reset )( 
            __RPC__in IRTCEnumBuddies * This);
        
        DECLSPEC_XFGVIRT(IRTCEnumBuddies, Skip)
        HRESULT ( STDMETHODCALLTYPE *Skip )( 
            __RPC__in IRTCEnumBuddies * This,
            /* [in] */ ULONG celt);
        
        DECLSPEC_XFGVIRT(IRTCEnumBuddies, Clone)
        HRESULT ( STDMETHODCALLTYPE *Clone )( 
            __RPC__in IRTCEnumBuddies * This,
            /* [retval][out] */ __RPC__deref_out_opt IRTCEnumBuddies **ppEnum);
        
        END_INTERFACE
    } IRTCEnumBuddiesVtbl;

    interface IRTCEnumBuddies
    {
        CONST_VTBL struct IRTCEnumBuddiesVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IRTCEnumBuddies_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IRTCEnumBuddies_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IRTCEnumBuddies_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IRTCEnumBuddies_Next(This,celt,ppElements,pceltFetched)	\
    ( (This)->lpVtbl -> Next(This,celt,ppElements,pceltFetched) ) 

#define IRTCEnumBuddies_Reset(This)	\
    ( (This)->lpVtbl -> Reset(This) ) 

#define IRTCEnumBuddies_Skip(This,celt)	\
    ( (This)->lpVtbl -> Skip(This,celt) ) 

#define IRTCEnumBuddies_Clone(This,ppEnum)	\
    ( (This)->lpVtbl -> Clone(This,ppEnum) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IRTCEnumBuddies_INTERFACE_DEFINED__ */


#ifndef __IRTCEnumWatchers_INTERFACE_DEFINED__
#define __IRTCEnumWatchers_INTERFACE_DEFINED__

/* interface IRTCEnumWatchers */
/* [unique][helpstring][hidden][uuid][object] */ 


EXTERN_C const IID IID_IRTCEnumWatchers;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("a87d55d7-db74-4ed1-9ca4-77a0e41b413e")
    IRTCEnumWatchers : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE Next( 
            /* [in] */ ULONG celt,
            /* [size_is][out] */ __RPC__out_ecount_full(celt) IRTCWatcher **ppElements,
            /* [full][out][in] */ __RPC__inout_opt ULONG *pceltFetched) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Reset( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Skip( 
            /* [in] */ ULONG celt) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Clone( 
            /* [retval][out] */ __RPC__deref_out_opt IRTCEnumWatchers **ppEnum) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IRTCEnumWatchersVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IRTCEnumWatchers * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IRTCEnumWatchers * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IRTCEnumWatchers * This);
        
        DECLSPEC_XFGVIRT(IRTCEnumWatchers, Next)
        HRESULT ( STDMETHODCALLTYPE *Next )( 
            __RPC__in IRTCEnumWatchers * This,
            /* [in] */ ULONG celt,
            /* [size_is][out] */ __RPC__out_ecount_full(celt) IRTCWatcher **ppElements,
            /* [full][out][in] */ __RPC__inout_opt ULONG *pceltFetched);
        
        DECLSPEC_XFGVIRT(IRTCEnumWatchers, Reset)
        HRESULT ( STDMETHODCALLTYPE *Reset )( 
            __RPC__in IRTCEnumWatchers * This);
        
        DECLSPEC_XFGVIRT(IRTCEnumWatchers, Skip)
        HRESULT ( STDMETHODCALLTYPE *Skip )( 
            __RPC__in IRTCEnumWatchers * This,
            /* [in] */ ULONG celt);
        
        DECLSPEC_XFGVIRT(IRTCEnumWatchers, Clone)
        HRESULT ( STDMETHODCALLTYPE *Clone )( 
            __RPC__in IRTCEnumWatchers * This,
            /* [retval][out] */ __RPC__deref_out_opt IRTCEnumWatchers **ppEnum);
        
        END_INTERFACE
    } IRTCEnumWatchersVtbl;

    interface IRTCEnumWatchers
    {
        CONST_VTBL struct IRTCEnumWatchersVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IRTCEnumWatchers_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IRTCEnumWatchers_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IRTCEnumWatchers_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IRTCEnumWatchers_Next(This,celt,ppElements,pceltFetched)	\
    ( (This)->lpVtbl -> Next(This,celt,ppElements,pceltFetched) ) 

#define IRTCEnumWatchers_Reset(This)	\
    ( (This)->lpVtbl -> Reset(This) ) 

#define IRTCEnumWatchers_Skip(This,celt)	\
    ( (This)->lpVtbl -> Skip(This,celt) ) 

#define IRTCEnumWatchers_Clone(This,ppEnum)	\
    ( (This)->lpVtbl -> Clone(This,ppEnum) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IRTCEnumWatchers_INTERFACE_DEFINED__ */


#ifndef __IRTCEnumGroups_INTERFACE_DEFINED__
#define __IRTCEnumGroups_INTERFACE_DEFINED__

/* interface IRTCEnumGroups */
/* [unique][helpstring][hidden][uuid][object] */ 


EXTERN_C const IID IID_IRTCEnumGroups;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("742378D6-A141-4415-8F27-35D99076CF5D")
    IRTCEnumGroups : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE Next( 
            /* [in] */ ULONG celt,
            /* [size_is][out] */ __RPC__out_ecount_full(celt) IRTCBuddyGroup **ppElements,
            /* [full][out][in] */ __RPC__inout_opt ULONG *pceltFetched) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Reset( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Skip( 
            /* [in] */ ULONG celt) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Clone( 
            /* [retval][out] */ __RPC__deref_out_opt IRTCEnumGroups **ppEnum) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IRTCEnumGroupsVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IRTCEnumGroups * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IRTCEnumGroups * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IRTCEnumGroups * This);
        
        DECLSPEC_XFGVIRT(IRTCEnumGroups, Next)
        HRESULT ( STDMETHODCALLTYPE *Next )( 
            __RPC__in IRTCEnumGroups * This,
            /* [in] */ ULONG celt,
            /* [size_is][out] */ __RPC__out_ecount_full(celt) IRTCBuddyGroup **ppElements,
            /* [full][out][in] */ __RPC__inout_opt ULONG *pceltFetched);
        
        DECLSPEC_XFGVIRT(IRTCEnumGroups, Reset)
        HRESULT ( STDMETHODCALLTYPE *Reset )( 
            __RPC__in IRTCEnumGroups * This);
        
        DECLSPEC_XFGVIRT(IRTCEnumGroups, Skip)
        HRESULT ( STDMETHODCALLTYPE *Skip )( 
            __RPC__in IRTCEnumGroups * This,
            /* [in] */ ULONG celt);
        
        DECLSPEC_XFGVIRT(IRTCEnumGroups, Clone)
        HRESULT ( STDMETHODCALLTYPE *Clone )( 
            __RPC__in IRTCEnumGroups * This,
            /* [retval][out] */ __RPC__deref_out_opt IRTCEnumGroups **ppEnum);
        
        END_INTERFACE
    } IRTCEnumGroupsVtbl;

    interface IRTCEnumGroups
    {
        CONST_VTBL struct IRTCEnumGroupsVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IRTCEnumGroups_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IRTCEnumGroups_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IRTCEnumGroups_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IRTCEnumGroups_Next(This,celt,ppElements,pceltFetched)	\
    ( (This)->lpVtbl -> Next(This,celt,ppElements,pceltFetched) ) 

#define IRTCEnumGroups_Reset(This)	\
    ( (This)->lpVtbl -> Reset(This) ) 

#define IRTCEnumGroups_Skip(This,celt)	\
    ( (This)->lpVtbl -> Skip(This,celt) ) 

#define IRTCEnumGroups_Clone(This,ppEnum)	\
    ( (This)->lpVtbl -> Clone(This,ppEnum) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IRTCEnumGroups_INTERFACE_DEFINED__ */


#ifndef __IRTCPresenceContact_INTERFACE_DEFINED__
#define __IRTCPresenceContact_INTERFACE_DEFINED__

/* interface IRTCPresenceContact */
/* [unique][helpstring][uuid][object] */ 


EXTERN_C const IID IID_IRTCPresenceContact;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("8b22f92c-cd90-42db-a733-212205c3e3df")
    IRTCPresenceContact : public IUnknown
    {
    public:
        virtual /* [helpstring][propget] */ HRESULT STDMETHODCALLTYPE get_PresentityURI( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrPresentityURI) = 0;
        
        virtual /* [helpstring][propput] */ HRESULT STDMETHODCALLTYPE put_PresentityURI( 
            /* [in] */ __RPC__in BSTR bstrPresentityURI) = 0;
        
        virtual /* [helpstring][propget] */ HRESULT STDMETHODCALLTYPE get_Name( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrName) = 0;
        
        virtual /* [helpstring][propput] */ HRESULT STDMETHODCALLTYPE put_Name( 
            /* [in] */ __RPC__in BSTR bstrName) = 0;
        
        virtual /* [helpstring][propget] */ HRESULT STDMETHODCALLTYPE get_Data( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrData) = 0;
        
        virtual /* [helpstring][propput] */ HRESULT STDMETHODCALLTYPE put_Data( 
            /* [in] */ __RPC__in BSTR bstrData) = 0;
        
        virtual /* [helpstring][propget] */ HRESULT STDMETHODCALLTYPE get_Persistent( 
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pfPersistent) = 0;
        
        virtual /* [helpstring][propput] */ HRESULT STDMETHODCALLTYPE put_Persistent( 
            /* [in] */ VARIANT_BOOL fPersistent) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IRTCPresenceContactVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IRTCPresenceContact * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IRTCPresenceContact * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IRTCPresenceContact * This);
        
        DECLSPEC_XFGVIRT(IRTCPresenceContact, get_PresentityURI)
        /* [helpstring][propget] */ HRESULT ( STDMETHODCALLTYPE *get_PresentityURI )( 
            __RPC__in IRTCPresenceContact * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrPresentityURI);
        
        DECLSPEC_XFGVIRT(IRTCPresenceContact, put_PresentityURI)
        /* [helpstring][propput] */ HRESULT ( STDMETHODCALLTYPE *put_PresentityURI )( 
            __RPC__in IRTCPresenceContact * This,
            /* [in] */ __RPC__in BSTR bstrPresentityURI);
        
        DECLSPEC_XFGVIRT(IRTCPresenceContact, get_Name)
        /* [helpstring][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Name )( 
            __RPC__in IRTCPresenceContact * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrName);
        
        DECLSPEC_XFGVIRT(IRTCPresenceContact, put_Name)
        /* [helpstring][propput] */ HRESULT ( STDMETHODCALLTYPE *put_Name )( 
            __RPC__in IRTCPresenceContact * This,
            /* [in] */ __RPC__in BSTR bstrName);
        
        DECLSPEC_XFGVIRT(IRTCPresenceContact, get_Data)
        /* [helpstring][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Data )( 
            __RPC__in IRTCPresenceContact * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrData);
        
        DECLSPEC_XFGVIRT(IRTCPresenceContact, put_Data)
        /* [helpstring][propput] */ HRESULT ( STDMETHODCALLTYPE *put_Data )( 
            __RPC__in IRTCPresenceContact * This,
            /* [in] */ __RPC__in BSTR bstrData);
        
        DECLSPEC_XFGVIRT(IRTCPresenceContact, get_Persistent)
        /* [helpstring][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Persistent )( 
            __RPC__in IRTCPresenceContact * This,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pfPersistent);
        
        DECLSPEC_XFGVIRT(IRTCPresenceContact, put_Persistent)
        /* [helpstring][propput] */ HRESULT ( STDMETHODCALLTYPE *put_Persistent )( 
            __RPC__in IRTCPresenceContact * This,
            /* [in] */ VARIANT_BOOL fPersistent);
        
        END_INTERFACE
    } IRTCPresenceContactVtbl;

    interface IRTCPresenceContact
    {
        CONST_VTBL struct IRTCPresenceContactVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IRTCPresenceContact_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IRTCPresenceContact_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IRTCPresenceContact_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IRTCPresenceContact_get_PresentityURI(This,pbstrPresentityURI)	\
    ( (This)->lpVtbl -> get_PresentityURI(This,pbstrPresentityURI) ) 

#define IRTCPresenceContact_put_PresentityURI(This,bstrPresentityURI)	\
    ( (This)->lpVtbl -> put_PresentityURI(This,bstrPresentityURI) ) 

#define IRTCPresenceContact_get_Name(This,pbstrName)	\
    ( (This)->lpVtbl -> get_Name(This,pbstrName) ) 

#define IRTCPresenceContact_put_Name(This,bstrName)	\
    ( (This)->lpVtbl -> put_Name(This,bstrName) ) 

#define IRTCPresenceContact_get_Data(This,pbstrData)	\
    ( (This)->lpVtbl -> get_Data(This,pbstrData) ) 

#define IRTCPresenceContact_put_Data(This,bstrData)	\
    ( (This)->lpVtbl -> put_Data(This,bstrData) ) 

#define IRTCPresenceContact_get_Persistent(This,pfPersistent)	\
    ( (This)->lpVtbl -> get_Persistent(This,pfPersistent) ) 

#define IRTCPresenceContact_put_Persistent(This,fPersistent)	\
    ( (This)->lpVtbl -> put_Persistent(This,fPersistent) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IRTCPresenceContact_INTERFACE_DEFINED__ */


#ifndef __IRTCBuddy_INTERFACE_DEFINED__
#define __IRTCBuddy_INTERFACE_DEFINED__

/* interface IRTCBuddy */
/* [unique][helpstring][uuid][object] */ 


EXTERN_C const IID IID_IRTCBuddy;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("fcb136c8-7b90-4e0c-befe-56edf0ba6f1c")
    IRTCBuddy : public IRTCPresenceContact
    {
    public:
        virtual /* [helpstring][propget] */ HRESULT STDMETHODCALLTYPE get_Status( 
            /* [retval][out] */ __RPC__out RTC_PRESENCE_STATUS *penStatus) = 0;
        
        virtual /* [helpstring][propget] */ HRESULT STDMETHODCALLTYPE get_Notes( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrNotes) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IRTCBuddyVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IRTCBuddy * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IRTCBuddy * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IRTCBuddy * This);
        
        DECLSPEC_XFGVIRT(IRTCPresenceContact, get_PresentityURI)
        /* [helpstring][propget] */ HRESULT ( STDMETHODCALLTYPE *get_PresentityURI )( 
            __RPC__in IRTCBuddy * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrPresentityURI);
        
        DECLSPEC_XFGVIRT(IRTCPresenceContact, put_PresentityURI)
        /* [helpstring][propput] */ HRESULT ( STDMETHODCALLTYPE *put_PresentityURI )( 
            __RPC__in IRTCBuddy * This,
            /* [in] */ __RPC__in BSTR bstrPresentityURI);
        
        DECLSPEC_XFGVIRT(IRTCPresenceContact, get_Name)
        /* [helpstring][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Name )( 
            __RPC__in IRTCBuddy * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrName);
        
        DECLSPEC_XFGVIRT(IRTCPresenceContact, put_Name)
        /* [helpstring][propput] */ HRESULT ( STDMETHODCALLTYPE *put_Name )( 
            __RPC__in IRTCBuddy * This,
            /* [in] */ __RPC__in BSTR bstrName);
        
        DECLSPEC_XFGVIRT(IRTCPresenceContact, get_Data)
        /* [helpstring][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Data )( 
            __RPC__in IRTCBuddy * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrData);
        
        DECLSPEC_XFGVIRT(IRTCPresenceContact, put_Data)
        /* [helpstring][propput] */ HRESULT ( STDMETHODCALLTYPE *put_Data )( 
            __RPC__in IRTCBuddy * This,
            /* [in] */ __RPC__in BSTR bstrData);
        
        DECLSPEC_XFGVIRT(IRTCPresenceContact, get_Persistent)
        /* [helpstring][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Persistent )( 
            __RPC__in IRTCBuddy * This,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pfPersistent);
        
        DECLSPEC_XFGVIRT(IRTCPresenceContact, put_Persistent)
        /* [helpstring][propput] */ HRESULT ( STDMETHODCALLTYPE *put_Persistent )( 
            __RPC__in IRTCBuddy * This,
            /* [in] */ VARIANT_BOOL fPersistent);
        
        DECLSPEC_XFGVIRT(IRTCBuddy, get_Status)
        /* [helpstring][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Status )( 
            __RPC__in IRTCBuddy * This,
            /* [retval][out] */ __RPC__out RTC_PRESENCE_STATUS *penStatus);
        
        DECLSPEC_XFGVIRT(IRTCBuddy, get_Notes)
        /* [helpstring][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Notes )( 
            __RPC__in IRTCBuddy * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrNotes);
        
        END_INTERFACE
    } IRTCBuddyVtbl;

    interface IRTCBuddy
    {
        CONST_VTBL struct IRTCBuddyVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IRTCBuddy_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IRTCBuddy_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IRTCBuddy_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IRTCBuddy_get_PresentityURI(This,pbstrPresentityURI)	\
    ( (This)->lpVtbl -> get_PresentityURI(This,pbstrPresentityURI) ) 

#define IRTCBuddy_put_PresentityURI(This,bstrPresentityURI)	\
    ( (This)->lpVtbl -> put_PresentityURI(This,bstrPresentityURI) ) 

#define IRTCBuddy_get_Name(This,pbstrName)	\
    ( (This)->lpVtbl -> get_Name(This,pbstrName) ) 

#define IRTCBuddy_put_Name(This,bstrName)	\
    ( (This)->lpVtbl -> put_Name(This,bstrName) ) 

#define IRTCBuddy_get_Data(This,pbstrData)	\
    ( (This)->lpVtbl -> get_Data(This,pbstrData) ) 

#define IRTCBuddy_put_Data(This,bstrData)	\
    ( (This)->lpVtbl -> put_Data(This,bstrData) ) 

#define IRTCBuddy_get_Persistent(This,pfPersistent)	\
    ( (This)->lpVtbl -> get_Persistent(This,pfPersistent) ) 

#define IRTCBuddy_put_Persistent(This,fPersistent)	\
    ( (This)->lpVtbl -> put_Persistent(This,fPersistent) ) 


#define IRTCBuddy_get_Status(This,penStatus)	\
    ( (This)->lpVtbl -> get_Status(This,penStatus) ) 

#define IRTCBuddy_get_Notes(This,pbstrNotes)	\
    ( (This)->lpVtbl -> get_Notes(This,pbstrNotes) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IRTCBuddy_INTERFACE_DEFINED__ */


#ifndef __IRTCBuddy2_INTERFACE_DEFINED__
#define __IRTCBuddy2_INTERFACE_DEFINED__

/* interface IRTCBuddy2 */
/* [unique][helpstring][uuid][object] */ 


EXTERN_C const IID IID_IRTCBuddy2;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("102F9588-23E7-40e3-954D-CD7A1D5C0361")
    IRTCBuddy2 : public IRTCBuddy
    {
    public:
        virtual /* [helpstring][propget] */ HRESULT STDMETHODCALLTYPE get_Profile( 
            /* [retval][out] */ __RPC__deref_out_opt IRTCProfile2 **ppProfile) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE Refresh( void) = 0;
        
        virtual /* [helpstring][hidden] */ HRESULT STDMETHODCALLTYPE EnumerateGroups( 
            /* [retval][out] */ __RPC__deref_out_opt IRTCEnumGroups **ppEnum) = 0;
        
        virtual /* [helpstring][propget] */ HRESULT STDMETHODCALLTYPE get_Groups( 
            /* [retval][out] */ __RPC__deref_out_opt IRTCCollection **ppCollection) = 0;
        
        virtual /* [helpstring][propget] */ HRESULT STDMETHODCALLTYPE get_PresenceProperty( 
            /* [in] */ RTC_PRESENCE_PROPERTY enProperty,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrProperty) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE EnumeratePresenceDevices( 
            /* [retval][out] */ __RPC__deref_out_opt IRTCEnumPresenceDevices **ppEnumDevices) = 0;
        
        virtual /* [helpstring][propget] */ HRESULT STDMETHODCALLTYPE get_PresenceDevices( 
            /* [retval][out] */ __RPC__deref_out_opt IRTCCollection **ppDevicesCollection) = 0;
        
        virtual /* [helpstring][propget] */ HRESULT STDMETHODCALLTYPE get_SubscriptionType( 
            /* [retval][out] */ __RPC__out RTC_BUDDY_SUBSCRIPTION_TYPE *penSubscriptionType) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IRTCBuddy2Vtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IRTCBuddy2 * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IRTCBuddy2 * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IRTCBuddy2 * This);
        
        DECLSPEC_XFGVIRT(IRTCPresenceContact, get_PresentityURI)
        /* [helpstring][propget] */ HRESULT ( STDMETHODCALLTYPE *get_PresentityURI )( 
            __RPC__in IRTCBuddy2 * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrPresentityURI);
        
        DECLSPEC_XFGVIRT(IRTCPresenceContact, put_PresentityURI)
        /* [helpstring][propput] */ HRESULT ( STDMETHODCALLTYPE *put_PresentityURI )( 
            __RPC__in IRTCBuddy2 * This,
            /* [in] */ __RPC__in BSTR bstrPresentityURI);
        
        DECLSPEC_XFGVIRT(IRTCPresenceContact, get_Name)
        /* [helpstring][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Name )( 
            __RPC__in IRTCBuddy2 * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrName);
        
        DECLSPEC_XFGVIRT(IRTCPresenceContact, put_Name)
        /* [helpstring][propput] */ HRESULT ( STDMETHODCALLTYPE *put_Name )( 
            __RPC__in IRTCBuddy2 * This,
            /* [in] */ __RPC__in BSTR bstrName);
        
        DECLSPEC_XFGVIRT(IRTCPresenceContact, get_Data)
        /* [helpstring][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Data )( 
            __RPC__in IRTCBuddy2 * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrData);
        
        DECLSPEC_XFGVIRT(IRTCPresenceContact, put_Data)
        /* [helpstring][propput] */ HRESULT ( STDMETHODCALLTYPE *put_Data )( 
            __RPC__in IRTCBuddy2 * This,
            /* [in] */ __RPC__in BSTR bstrData);
        
        DECLSPEC_XFGVIRT(IRTCPresenceContact, get_Persistent)
        /* [helpstring][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Persistent )( 
            __RPC__in IRTCBuddy2 * This,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pfPersistent);
        
        DECLSPEC_XFGVIRT(IRTCPresenceContact, put_Persistent)
        /* [helpstring][propput] */ HRESULT ( STDMETHODCALLTYPE *put_Persistent )( 
            __RPC__in IRTCBuddy2 * This,
            /* [in] */ VARIANT_BOOL fPersistent);
        
        DECLSPEC_XFGVIRT(IRTCBuddy, get_Status)
        /* [helpstring][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Status )( 
            __RPC__in IRTCBuddy2 * This,
            /* [retval][out] */ __RPC__out RTC_PRESENCE_STATUS *penStatus);
        
        DECLSPEC_XFGVIRT(IRTCBuddy, get_Notes)
        /* [helpstring][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Notes )( 
            __RPC__in IRTCBuddy2 * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrNotes);
        
        DECLSPEC_XFGVIRT(IRTCBuddy2, get_Profile)
        /* [helpstring][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Profile )( 
            __RPC__in IRTCBuddy2 * This,
            /* [retval][out] */ __RPC__deref_out_opt IRTCProfile2 **ppProfile);
        
        DECLSPEC_XFGVIRT(IRTCBuddy2, Refresh)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *Refresh )( 
            __RPC__in IRTCBuddy2 * This);
        
        DECLSPEC_XFGVIRT(IRTCBuddy2, EnumerateGroups)
        /* [helpstring][hidden] */ HRESULT ( STDMETHODCALLTYPE *EnumerateGroups )( 
            __RPC__in IRTCBuddy2 * This,
            /* [retval][out] */ __RPC__deref_out_opt IRTCEnumGroups **ppEnum);
        
        DECLSPEC_XFGVIRT(IRTCBuddy2, get_Groups)
        /* [helpstring][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Groups )( 
            __RPC__in IRTCBuddy2 * This,
            /* [retval][out] */ __RPC__deref_out_opt IRTCCollection **ppCollection);
        
        DECLSPEC_XFGVIRT(IRTCBuddy2, get_PresenceProperty)
        /* [helpstring][propget] */ HRESULT ( STDMETHODCALLTYPE *get_PresenceProperty )( 
            __RPC__in IRTCBuddy2 * This,
            /* [in] */ RTC_PRESENCE_PROPERTY enProperty,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrProperty);
        
        DECLSPEC_XFGVIRT(IRTCBuddy2, EnumeratePresenceDevices)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *EnumeratePresenceDevices )( 
            __RPC__in IRTCBuddy2 * This,
            /* [retval][out] */ __RPC__deref_out_opt IRTCEnumPresenceDevices **ppEnumDevices);
        
        DECLSPEC_XFGVIRT(IRTCBuddy2, get_PresenceDevices)
        /* [helpstring][propget] */ HRESULT ( STDMETHODCALLTYPE *get_PresenceDevices )( 
            __RPC__in IRTCBuddy2 * This,
            /* [retval][out] */ __RPC__deref_out_opt IRTCCollection **ppDevicesCollection);
        
        DECLSPEC_XFGVIRT(IRTCBuddy2, get_SubscriptionType)
        /* [helpstring][propget] */ HRESULT ( STDMETHODCALLTYPE *get_SubscriptionType )( 
            __RPC__in IRTCBuddy2 * This,
            /* [retval][out] */ __RPC__out RTC_BUDDY_SUBSCRIPTION_TYPE *penSubscriptionType);
        
        END_INTERFACE
    } IRTCBuddy2Vtbl;

    interface IRTCBuddy2
    {
        CONST_VTBL struct IRTCBuddy2Vtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IRTCBuddy2_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IRTCBuddy2_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IRTCBuddy2_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IRTCBuddy2_get_PresentityURI(This,pbstrPresentityURI)	\
    ( (This)->lpVtbl -> get_PresentityURI(This,pbstrPresentityURI) ) 

#define IRTCBuddy2_put_PresentityURI(This,bstrPresentityURI)	\
    ( (This)->lpVtbl -> put_PresentityURI(This,bstrPresentityURI) ) 

#define IRTCBuddy2_get_Name(This,pbstrName)	\
    ( (This)->lpVtbl -> get_Name(This,pbstrName) ) 

#define IRTCBuddy2_put_Name(This,bstrName)	\
    ( (This)->lpVtbl -> put_Name(This,bstrName) ) 

#define IRTCBuddy2_get_Data(This,pbstrData)	\
    ( (This)->lpVtbl -> get_Data(This,pbstrData) ) 

#define IRTCBuddy2_put_Data(This,bstrData)	\
    ( (This)->lpVtbl -> put_Data(This,bstrData) ) 

#define IRTCBuddy2_get_Persistent(This,pfPersistent)	\
    ( (This)->lpVtbl -> get_Persistent(This,pfPersistent) ) 

#define IRTCBuddy2_put_Persistent(This,fPersistent)	\
    ( (This)->lpVtbl -> put_Persistent(This,fPersistent) ) 


#define IRTCBuddy2_get_Status(This,penStatus)	\
    ( (This)->lpVtbl -> get_Status(This,penStatus) ) 

#define IRTCBuddy2_get_Notes(This,pbstrNotes)	\
    ( (This)->lpVtbl -> get_Notes(This,pbstrNotes) ) 


#define IRTCBuddy2_get_Profile(This,ppProfile)	\
    ( (This)->lpVtbl -> get_Profile(This,ppProfile) ) 

#define IRTCBuddy2_Refresh(This)	\
    ( (This)->lpVtbl -> Refresh(This) ) 

#define IRTCBuddy2_EnumerateGroups(This,ppEnum)	\
    ( (This)->lpVtbl -> EnumerateGroups(This,ppEnum) ) 

#define IRTCBuddy2_get_Groups(This,ppCollection)	\
    ( (This)->lpVtbl -> get_Groups(This,ppCollection) ) 

#define IRTCBuddy2_get_PresenceProperty(This,enProperty,pbstrProperty)	\
    ( (This)->lpVtbl -> get_PresenceProperty(This,enProperty,pbstrProperty) ) 

#define IRTCBuddy2_EnumeratePresenceDevices(This,ppEnumDevices)	\
    ( (This)->lpVtbl -> EnumeratePresenceDevices(This,ppEnumDevices) ) 

#define IRTCBuddy2_get_PresenceDevices(This,ppDevicesCollection)	\
    ( (This)->lpVtbl -> get_PresenceDevices(This,ppDevicesCollection) ) 

#define IRTCBuddy2_get_SubscriptionType(This,penSubscriptionType)	\
    ( (This)->lpVtbl -> get_SubscriptionType(This,penSubscriptionType) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IRTCBuddy2_INTERFACE_DEFINED__ */


#ifndef __IRTCWatcher_INTERFACE_DEFINED__
#define __IRTCWatcher_INTERFACE_DEFINED__

/* interface IRTCWatcher */
/* [unique][helpstring][uuid][object] */ 


EXTERN_C const IID IID_IRTCWatcher;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("c7cedad8-346b-4d1b-ac02-a2088df9be4f")
    IRTCWatcher : public IRTCPresenceContact
    {
    public:
        virtual /* [helpstring][propget] */ HRESULT STDMETHODCALLTYPE get_State( 
            /* [retval][out] */ __RPC__out RTC_WATCHER_STATE *penState) = 0;
        
        virtual /* [helpstring][propput] */ HRESULT STDMETHODCALLTYPE put_State( 
            /* [in] */ RTC_WATCHER_STATE enState) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IRTCWatcherVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IRTCWatcher * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IRTCWatcher * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IRTCWatcher * This);
        
        DECLSPEC_XFGVIRT(IRTCPresenceContact, get_PresentityURI)
        /* [helpstring][propget] */ HRESULT ( STDMETHODCALLTYPE *get_PresentityURI )( 
            __RPC__in IRTCWatcher * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrPresentityURI);
        
        DECLSPEC_XFGVIRT(IRTCPresenceContact, put_PresentityURI)
        /* [helpstring][propput] */ HRESULT ( STDMETHODCALLTYPE *put_PresentityURI )( 
            __RPC__in IRTCWatcher * This,
            /* [in] */ __RPC__in BSTR bstrPresentityURI);
        
        DECLSPEC_XFGVIRT(IRTCPresenceContact, get_Name)
        /* [helpstring][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Name )( 
            __RPC__in IRTCWatcher * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrName);
        
        DECLSPEC_XFGVIRT(IRTCPresenceContact, put_Name)
        /* [helpstring][propput] */ HRESULT ( STDMETHODCALLTYPE *put_Name )( 
            __RPC__in IRTCWatcher * This,
            /* [in] */ __RPC__in BSTR bstrName);
        
        DECLSPEC_XFGVIRT(IRTCPresenceContact, get_Data)
        /* [helpstring][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Data )( 
            __RPC__in IRTCWatcher * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrData);
        
        DECLSPEC_XFGVIRT(IRTCPresenceContact, put_Data)
        /* [helpstring][propput] */ HRESULT ( STDMETHODCALLTYPE *put_Data )( 
            __RPC__in IRTCWatcher * This,
            /* [in] */ __RPC__in BSTR bstrData);
        
        DECLSPEC_XFGVIRT(IRTCPresenceContact, get_Persistent)
        /* [helpstring][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Persistent )( 
            __RPC__in IRTCWatcher * This,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pfPersistent);
        
        DECLSPEC_XFGVIRT(IRTCPresenceContact, put_Persistent)
        /* [helpstring][propput] */ HRESULT ( STDMETHODCALLTYPE *put_Persistent )( 
            __RPC__in IRTCWatcher * This,
            /* [in] */ VARIANT_BOOL fPersistent);
        
        DECLSPEC_XFGVIRT(IRTCWatcher, get_State)
        /* [helpstring][propget] */ HRESULT ( STDMETHODCALLTYPE *get_State )( 
            __RPC__in IRTCWatcher * This,
            /* [retval][out] */ __RPC__out RTC_WATCHER_STATE *penState);
        
        DECLSPEC_XFGVIRT(IRTCWatcher, put_State)
        /* [helpstring][propput] */ HRESULT ( STDMETHODCALLTYPE *put_State )( 
            __RPC__in IRTCWatcher * This,
            /* [in] */ RTC_WATCHER_STATE enState);
        
        END_INTERFACE
    } IRTCWatcherVtbl;

    interface IRTCWatcher
    {
        CONST_VTBL struct IRTCWatcherVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IRTCWatcher_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IRTCWatcher_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IRTCWatcher_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IRTCWatcher_get_PresentityURI(This,pbstrPresentityURI)	\
    ( (This)->lpVtbl -> get_PresentityURI(This,pbstrPresentityURI) ) 

#define IRTCWatcher_put_PresentityURI(This,bstrPresentityURI)	\
    ( (This)->lpVtbl -> put_PresentityURI(This,bstrPresentityURI) ) 

#define IRTCWatcher_get_Name(This,pbstrName)	\
    ( (This)->lpVtbl -> get_Name(This,pbstrName) ) 

#define IRTCWatcher_put_Name(This,bstrName)	\
    ( (This)->lpVtbl -> put_Name(This,bstrName) ) 

#define IRTCWatcher_get_Data(This,pbstrData)	\
    ( (This)->lpVtbl -> get_Data(This,pbstrData) ) 

#define IRTCWatcher_put_Data(This,bstrData)	\
    ( (This)->lpVtbl -> put_Data(This,bstrData) ) 

#define IRTCWatcher_get_Persistent(This,pfPersistent)	\
    ( (This)->lpVtbl -> get_Persistent(This,pfPersistent) ) 

#define IRTCWatcher_put_Persistent(This,fPersistent)	\
    ( (This)->lpVtbl -> put_Persistent(This,fPersistent) ) 


#define IRTCWatcher_get_State(This,penState)	\
    ( (This)->lpVtbl -> get_State(This,penState) ) 

#define IRTCWatcher_put_State(This,enState)	\
    ( (This)->lpVtbl -> put_State(This,enState) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IRTCWatcher_INTERFACE_DEFINED__ */


#ifndef __IRTCWatcher2_INTERFACE_DEFINED__
#define __IRTCWatcher2_INTERFACE_DEFINED__

/* interface IRTCWatcher2 */
/* [unique][helpstring][uuid][object] */ 


EXTERN_C const IID IID_IRTCWatcher2;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("D4D9967F-D011-4b1d-91E3-ABA78F96393D")
    IRTCWatcher2 : public IRTCWatcher
    {
    public:
        virtual /* [helpstring][propget] */ HRESULT STDMETHODCALLTYPE get_Profile( 
            /* [retval][out] */ __RPC__deref_out_opt IRTCProfile2 **ppProfile) = 0;
        
        virtual /* [helpstring][propget] */ HRESULT STDMETHODCALLTYPE get_Scope( 
            /* [retval][out] */ __RPC__out RTC_ACE_SCOPE *penScope) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IRTCWatcher2Vtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IRTCWatcher2 * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IRTCWatcher2 * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IRTCWatcher2 * This);
        
        DECLSPEC_XFGVIRT(IRTCPresenceContact, get_PresentityURI)
        /* [helpstring][propget] */ HRESULT ( STDMETHODCALLTYPE *get_PresentityURI )( 
            __RPC__in IRTCWatcher2 * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrPresentityURI);
        
        DECLSPEC_XFGVIRT(IRTCPresenceContact, put_PresentityURI)
        /* [helpstring][propput] */ HRESULT ( STDMETHODCALLTYPE *put_PresentityURI )( 
            __RPC__in IRTCWatcher2 * This,
            /* [in] */ __RPC__in BSTR bstrPresentityURI);
        
        DECLSPEC_XFGVIRT(IRTCPresenceContact, get_Name)
        /* [helpstring][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Name )( 
            __RPC__in IRTCWatcher2 * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrName);
        
        DECLSPEC_XFGVIRT(IRTCPresenceContact, put_Name)
        /* [helpstring][propput] */ HRESULT ( STDMETHODCALLTYPE *put_Name )( 
            __RPC__in IRTCWatcher2 * This,
            /* [in] */ __RPC__in BSTR bstrName);
        
        DECLSPEC_XFGVIRT(IRTCPresenceContact, get_Data)
        /* [helpstring][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Data )( 
            __RPC__in IRTCWatcher2 * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrData);
        
        DECLSPEC_XFGVIRT(IRTCPresenceContact, put_Data)
        /* [helpstring][propput] */ HRESULT ( STDMETHODCALLTYPE *put_Data )( 
            __RPC__in IRTCWatcher2 * This,
            /* [in] */ __RPC__in BSTR bstrData);
        
        DECLSPEC_XFGVIRT(IRTCPresenceContact, get_Persistent)
        /* [helpstring][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Persistent )( 
            __RPC__in IRTCWatcher2 * This,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pfPersistent);
        
        DECLSPEC_XFGVIRT(IRTCPresenceContact, put_Persistent)
        /* [helpstring][propput] */ HRESULT ( STDMETHODCALLTYPE *put_Persistent )( 
            __RPC__in IRTCWatcher2 * This,
            /* [in] */ VARIANT_BOOL fPersistent);
        
        DECLSPEC_XFGVIRT(IRTCWatcher, get_State)
        /* [helpstring][propget] */ HRESULT ( STDMETHODCALLTYPE *get_State )( 
            __RPC__in IRTCWatcher2 * This,
            /* [retval][out] */ __RPC__out RTC_WATCHER_STATE *penState);
        
        DECLSPEC_XFGVIRT(IRTCWatcher, put_State)
        /* [helpstring][propput] */ HRESULT ( STDMETHODCALLTYPE *put_State )( 
            __RPC__in IRTCWatcher2 * This,
            /* [in] */ RTC_WATCHER_STATE enState);
        
        DECLSPEC_XFGVIRT(IRTCWatcher2, get_Profile)
        /* [helpstring][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Profile )( 
            __RPC__in IRTCWatcher2 * This,
            /* [retval][out] */ __RPC__deref_out_opt IRTCProfile2 **ppProfile);
        
        DECLSPEC_XFGVIRT(IRTCWatcher2, get_Scope)
        /* [helpstring][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Scope )( 
            __RPC__in IRTCWatcher2 * This,
            /* [retval][out] */ __RPC__out RTC_ACE_SCOPE *penScope);
        
        END_INTERFACE
    } IRTCWatcher2Vtbl;

    interface IRTCWatcher2
    {
        CONST_VTBL struct IRTCWatcher2Vtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IRTCWatcher2_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IRTCWatcher2_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IRTCWatcher2_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IRTCWatcher2_get_PresentityURI(This,pbstrPresentityURI)	\
    ( (This)->lpVtbl -> get_PresentityURI(This,pbstrPresentityURI) ) 

#define IRTCWatcher2_put_PresentityURI(This,bstrPresentityURI)	\
    ( (This)->lpVtbl -> put_PresentityURI(This,bstrPresentityURI) ) 

#define IRTCWatcher2_get_Name(This,pbstrName)	\
    ( (This)->lpVtbl -> get_Name(This,pbstrName) ) 

#define IRTCWatcher2_put_Name(This,bstrName)	\
    ( (This)->lpVtbl -> put_Name(This,bstrName) ) 

#define IRTCWatcher2_get_Data(This,pbstrData)	\
    ( (This)->lpVtbl -> get_Data(This,pbstrData) ) 

#define IRTCWatcher2_put_Data(This,bstrData)	\
    ( (This)->lpVtbl -> put_Data(This,bstrData) ) 

#define IRTCWatcher2_get_Persistent(This,pfPersistent)	\
    ( (This)->lpVtbl -> get_Persistent(This,pfPersistent) ) 

#define IRTCWatcher2_put_Persistent(This,fPersistent)	\
    ( (This)->lpVtbl -> put_Persistent(This,fPersistent) ) 


#define IRTCWatcher2_get_State(This,penState)	\
    ( (This)->lpVtbl -> get_State(This,penState) ) 

#define IRTCWatcher2_put_State(This,enState)	\
    ( (This)->lpVtbl -> put_State(This,enState) ) 


#define IRTCWatcher2_get_Profile(This,ppProfile)	\
    ( (This)->lpVtbl -> get_Profile(This,ppProfile) ) 

#define IRTCWatcher2_get_Scope(This,penScope)	\
    ( (This)->lpVtbl -> get_Scope(This,penScope) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IRTCWatcher2_INTERFACE_DEFINED__ */


#ifndef __IRTCBuddyGroup_INTERFACE_DEFINED__
#define __IRTCBuddyGroup_INTERFACE_DEFINED__

/* interface IRTCBuddyGroup */
/* [unique][helpstring][uuid][object] */ 


EXTERN_C const IID IID_IRTCBuddyGroup;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("60361E68-9164-4389-A4C6-D0B3925BDA5E")
    IRTCBuddyGroup : public IUnknown
    {
    public:
        virtual /* [helpstring][propget] */ HRESULT STDMETHODCALLTYPE get_Name( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrGroupName) = 0;
        
        virtual /* [helpstring][propput] */ HRESULT STDMETHODCALLTYPE put_Name( 
            /* [in] */ __RPC__in BSTR bstrGroupName) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE AddBuddy( 
            /* [in] */ __RPC__in_opt IRTCBuddy *pBuddy) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE RemoveBuddy( 
            /* [in] */ __RPC__in_opt IRTCBuddy *pBuddy) = 0;
        
        virtual /* [helpstring][hidden] */ HRESULT STDMETHODCALLTYPE EnumerateBuddies( 
            /* [retval][out] */ __RPC__deref_out_opt IRTCEnumBuddies **ppEnum) = 0;
        
        virtual /* [helpstring][propget] */ HRESULT STDMETHODCALLTYPE get_Buddies( 
            /* [retval][out] */ __RPC__deref_out_opt IRTCCollection **ppCollection) = 0;
        
        virtual /* [helpstring][propget] */ HRESULT STDMETHODCALLTYPE get_Data( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrData) = 0;
        
        virtual /* [helpstring][propput] */ HRESULT STDMETHODCALLTYPE put_Data( 
            /* [in] */ __RPC__in BSTR bstrData) = 0;
        
        virtual /* [helpstring][propget] */ HRESULT STDMETHODCALLTYPE get_Profile( 
            /* [retval][out] */ __RPC__deref_out_opt IRTCProfile2 **ppProfile) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IRTCBuddyGroupVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IRTCBuddyGroup * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IRTCBuddyGroup * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IRTCBuddyGroup * This);
        
        DECLSPEC_XFGVIRT(IRTCBuddyGroup, get_Name)
        /* [helpstring][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Name )( 
            __RPC__in IRTCBuddyGroup * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrGroupName);
        
        DECLSPEC_XFGVIRT(IRTCBuddyGroup, put_Name)
        /* [helpstring][propput] */ HRESULT ( STDMETHODCALLTYPE *put_Name )( 
            __RPC__in IRTCBuddyGroup * This,
            /* [in] */ __RPC__in BSTR bstrGroupName);
        
        DECLSPEC_XFGVIRT(IRTCBuddyGroup, AddBuddy)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *AddBuddy )( 
            __RPC__in IRTCBuddyGroup * This,
            /* [in] */ __RPC__in_opt IRTCBuddy *pBuddy);
        
        DECLSPEC_XFGVIRT(IRTCBuddyGroup, RemoveBuddy)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *RemoveBuddy )( 
            __RPC__in IRTCBuddyGroup * This,
            /* [in] */ __RPC__in_opt IRTCBuddy *pBuddy);
        
        DECLSPEC_XFGVIRT(IRTCBuddyGroup, EnumerateBuddies)
        /* [helpstring][hidden] */ HRESULT ( STDMETHODCALLTYPE *EnumerateBuddies )( 
            __RPC__in IRTCBuddyGroup * This,
            /* [retval][out] */ __RPC__deref_out_opt IRTCEnumBuddies **ppEnum);
        
        DECLSPEC_XFGVIRT(IRTCBuddyGroup, get_Buddies)
        /* [helpstring][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Buddies )( 
            __RPC__in IRTCBuddyGroup * This,
            /* [retval][out] */ __RPC__deref_out_opt IRTCCollection **ppCollection);
        
        DECLSPEC_XFGVIRT(IRTCBuddyGroup, get_Data)
        /* [helpstring][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Data )( 
            __RPC__in IRTCBuddyGroup * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrData);
        
        DECLSPEC_XFGVIRT(IRTCBuddyGroup, put_Data)
        /* [helpstring][propput] */ HRESULT ( STDMETHODCALLTYPE *put_Data )( 
            __RPC__in IRTCBuddyGroup * This,
            /* [in] */ __RPC__in BSTR bstrData);
        
        DECLSPEC_XFGVIRT(IRTCBuddyGroup, get_Profile)
        /* [helpstring][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Profile )( 
            __RPC__in IRTCBuddyGroup * This,
            /* [retval][out] */ __RPC__deref_out_opt IRTCProfile2 **ppProfile);
        
        END_INTERFACE
    } IRTCBuddyGroupVtbl;

    interface IRTCBuddyGroup
    {
        CONST_VTBL struct IRTCBuddyGroupVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IRTCBuddyGroup_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IRTCBuddyGroup_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IRTCBuddyGroup_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IRTCBuddyGroup_get_Name(This,pbstrGroupName)	\
    ( (This)->lpVtbl -> get_Name(This,pbstrGroupName) ) 

#define IRTCBuddyGroup_put_Name(This,bstrGroupName)	\
    ( (This)->lpVtbl -> put_Name(This,bstrGroupName) ) 

#define IRTCBuddyGroup_AddBuddy(This,pBuddy)	\
    ( (This)->lpVtbl -> AddBuddy(This,pBuddy) ) 

#define IRTCBuddyGroup_RemoveBuddy(This,pBuddy)	\
    ( (This)->lpVtbl -> RemoveBuddy(This,pBuddy) ) 

#define IRTCBuddyGroup_EnumerateBuddies(This,ppEnum)	\
    ( (This)->lpVtbl -> EnumerateBuddies(This,ppEnum) ) 

#define IRTCBuddyGroup_get_Buddies(This,ppCollection)	\
    ( (This)->lpVtbl -> get_Buddies(This,ppCollection) ) 

#define IRTCBuddyGroup_get_Data(This,pbstrData)	\
    ( (This)->lpVtbl -> get_Data(This,pbstrData) ) 

#define IRTCBuddyGroup_put_Data(This,bstrData)	\
    ( (This)->lpVtbl -> put_Data(This,bstrData) ) 

#define IRTCBuddyGroup_get_Profile(This,ppProfile)	\
    ( (This)->lpVtbl -> get_Profile(This,ppProfile) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IRTCBuddyGroup_INTERFACE_DEFINED__ */


#ifndef __IRTCEventNotification_INTERFACE_DEFINED__
#define __IRTCEventNotification_INTERFACE_DEFINED__

/* interface IRTCEventNotification */
/* [unique][oleautomation][helpstring][uuid][object] */ 


EXTERN_C const IID IID_IRTCEventNotification;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("13fa24c7-5748-4b21-91f5-7397609ce747")
    IRTCEventNotification : public IUnknown
    {
    public:
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE Event( 
            /* [in] */ RTC_EVENT RTCEvent,
            /* [in] */ __RPC__in_opt IDispatch *pEvent) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IRTCEventNotificationVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IRTCEventNotification * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IRTCEventNotification * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IRTCEventNotification * This);
        
        DECLSPEC_XFGVIRT(IRTCEventNotification, Event)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *Event )( 
            __RPC__in IRTCEventNotification * This,
            /* [in] */ RTC_EVENT RTCEvent,
            /* [in] */ __RPC__in_opt IDispatch *pEvent);
        
        END_INTERFACE
    } IRTCEventNotificationVtbl;

    interface IRTCEventNotification
    {
        CONST_VTBL struct IRTCEventNotificationVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IRTCEventNotification_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IRTCEventNotification_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IRTCEventNotification_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IRTCEventNotification_Event(This,RTCEvent,pEvent)	\
    ( (This)->lpVtbl -> Event(This,RTCEvent,pEvent) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IRTCEventNotification_INTERFACE_DEFINED__ */


#ifndef __IRTCPortManager_INTERFACE_DEFINED__
#define __IRTCPortManager_INTERFACE_DEFINED__

/* interface IRTCPortManager */
/* [unique][oleautomation][helpstring][uuid][object] */ 


EXTERN_C const IID IID_IRTCPortManager;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("DA77C14B-6208-43ca-8DDF-5B60A0A69FAC")
    IRTCPortManager : public IUnknown
    {
    public:
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE GetMapping( 
            /* [in] */ __RPC__in BSTR bstrRemoteAddress,
            /* [in] */ RTC_PORT_TYPE enPortType,
            /* [out][in] */ __RPC__deref_inout_opt BSTR *pbstrInternalLocalAddress,
            /* [out][in] */ __RPC__inout long *plInternalLocalPort,
            /* [out][in] */ __RPC__deref_inout_opt BSTR *pbstrExternalLocalAddress,
            /* [out][in] */ __RPC__inout long *plExternalLocalPort) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE UpdateRemoteAddress( 
            /* [in] */ __RPC__in BSTR bstrRemoteAddress,
            /* [in] */ __RPC__in BSTR bstrInternalLocalAddress,
            /* [in] */ long lInternalLocalPort,
            /* [in] */ __RPC__in BSTR bstrExternalLocalAddress,
            /* [in] */ long lExternalLocalPort) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE ReleaseMapping( 
            /* [in] */ __RPC__in BSTR bstrInternalLocalAddress,
            /* [in] */ long lInternalLocalPort,
            /* [in] */ __RPC__in BSTR bstrExternalLocalAddress,
            /* [in] */ long lExternalLocalAddress) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IRTCPortManagerVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IRTCPortManager * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IRTCPortManager * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IRTCPortManager * This);
        
        DECLSPEC_XFGVIRT(IRTCPortManager, GetMapping)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *GetMapping )( 
            __RPC__in IRTCPortManager * This,
            /* [in] */ __RPC__in BSTR bstrRemoteAddress,
            /* [in] */ RTC_PORT_TYPE enPortType,
            /* [out][in] */ __RPC__deref_inout_opt BSTR *pbstrInternalLocalAddress,
            /* [out][in] */ __RPC__inout long *plInternalLocalPort,
            /* [out][in] */ __RPC__deref_inout_opt BSTR *pbstrExternalLocalAddress,
            /* [out][in] */ __RPC__inout long *plExternalLocalPort);
        
        DECLSPEC_XFGVIRT(IRTCPortManager, UpdateRemoteAddress)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *UpdateRemoteAddress )( 
            __RPC__in IRTCPortManager * This,
            /* [in] */ __RPC__in BSTR bstrRemoteAddress,
            /* [in] */ __RPC__in BSTR bstrInternalLocalAddress,
            /* [in] */ long lInternalLocalPort,
            /* [in] */ __RPC__in BSTR bstrExternalLocalAddress,
            /* [in] */ long lExternalLocalPort);
        
        DECLSPEC_XFGVIRT(IRTCPortManager, ReleaseMapping)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *ReleaseMapping )( 
            __RPC__in IRTCPortManager * This,
            /* [in] */ __RPC__in BSTR bstrInternalLocalAddress,
            /* [in] */ long lInternalLocalPort,
            /* [in] */ __RPC__in BSTR bstrExternalLocalAddress,
            /* [in] */ long lExternalLocalAddress);
        
        END_INTERFACE
    } IRTCPortManagerVtbl;

    interface IRTCPortManager
    {
        CONST_VTBL struct IRTCPortManagerVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IRTCPortManager_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IRTCPortManager_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IRTCPortManager_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IRTCPortManager_GetMapping(This,bstrRemoteAddress,enPortType,pbstrInternalLocalAddress,plInternalLocalPort,pbstrExternalLocalAddress,plExternalLocalPort)	\
    ( (This)->lpVtbl -> GetMapping(This,bstrRemoteAddress,enPortType,pbstrInternalLocalAddress,plInternalLocalPort,pbstrExternalLocalAddress,plExternalLocalPort) ) 

#define IRTCPortManager_UpdateRemoteAddress(This,bstrRemoteAddress,bstrInternalLocalAddress,lInternalLocalPort,bstrExternalLocalAddress,lExternalLocalPort)	\
    ( (This)->lpVtbl -> UpdateRemoteAddress(This,bstrRemoteAddress,bstrInternalLocalAddress,lInternalLocalPort,bstrExternalLocalAddress,lExternalLocalPort) ) 

#define IRTCPortManager_ReleaseMapping(This,bstrInternalLocalAddress,lInternalLocalPort,bstrExternalLocalAddress,lExternalLocalAddress)	\
    ( (This)->lpVtbl -> ReleaseMapping(This,bstrInternalLocalAddress,lInternalLocalPort,bstrExternalLocalAddress,lExternalLocalAddress) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IRTCPortManager_INTERFACE_DEFINED__ */


#ifndef __IRTCSessionPortManagement_INTERFACE_DEFINED__
#define __IRTCSessionPortManagement_INTERFACE_DEFINED__

/* interface IRTCSessionPortManagement */
/* [unique][helpstring][uuid][object] */ 


EXTERN_C const IID IID_IRTCSessionPortManagement;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("a072f1d6-0286-4e1f-85f2-17a2948456ec")
    IRTCSessionPortManagement : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE SetPortManager( 
            /* [in] */ __RPC__in_opt IRTCPortManager *pPortManager) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IRTCSessionPortManagementVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IRTCSessionPortManagement * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IRTCSessionPortManagement * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IRTCSessionPortManagement * This);
        
        DECLSPEC_XFGVIRT(IRTCSessionPortManagement, SetPortManager)
        HRESULT ( STDMETHODCALLTYPE *SetPortManager )( 
            __RPC__in IRTCSessionPortManagement * This,
            /* [in] */ __RPC__in_opt IRTCPortManager *pPortManager);
        
        END_INTERFACE
    } IRTCSessionPortManagementVtbl;

    interface IRTCSessionPortManagement
    {
        CONST_VTBL struct IRTCSessionPortManagementVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IRTCSessionPortManagement_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IRTCSessionPortManagement_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IRTCSessionPortManagement_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IRTCSessionPortManagement_SetPortManager(This,pPortManager)	\
    ( (This)->lpVtbl -> SetPortManager(This,pPortManager) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IRTCSessionPortManagement_INTERFACE_DEFINED__ */


#ifndef __IRTCClientPortManagement_INTERFACE_DEFINED__
#define __IRTCClientPortManagement_INTERFACE_DEFINED__

/* interface IRTCClientPortManagement */
/* [unique][helpstring][uuid][object] */ 


EXTERN_C const IID IID_IRTCClientPortManagement;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("d5df3f03-4bde-4417-aefe-71177bdaea66")
    IRTCClientPortManagement : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE StartListenAddressAndPort( 
            /* [in] */ __RPC__in BSTR bstrInternalLocalAddress,
            /* [in] */ long lInternalLocalPort) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE StopListenAddressAndPort( 
            /* [in] */ __RPC__in BSTR bstrInternalLocalAddress,
            /* [in] */ long lInternalLocalPort) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE GetPortRange( 
            /* [in] */ RTC_PORT_TYPE enPortType,
            /* [out] */ __RPC__out long *plMinValue,
            /* [out] */ __RPC__out long *plMaxValue) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IRTCClientPortManagementVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IRTCClientPortManagement * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IRTCClientPortManagement * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IRTCClientPortManagement * This);
        
        DECLSPEC_XFGVIRT(IRTCClientPortManagement, StartListenAddressAndPort)
        HRESULT ( STDMETHODCALLTYPE *StartListenAddressAndPort )( 
            __RPC__in IRTCClientPortManagement * This,
            /* [in] */ __RPC__in BSTR bstrInternalLocalAddress,
            /* [in] */ long lInternalLocalPort);
        
        DECLSPEC_XFGVIRT(IRTCClientPortManagement, StopListenAddressAndPort)
        HRESULT ( STDMETHODCALLTYPE *StopListenAddressAndPort )( 
            __RPC__in IRTCClientPortManagement * This,
            /* [in] */ __RPC__in BSTR bstrInternalLocalAddress,
            /* [in] */ long lInternalLocalPort);
        
        DECLSPEC_XFGVIRT(IRTCClientPortManagement, GetPortRange)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *GetPortRange )( 
            __RPC__in IRTCClientPortManagement * This,
            /* [in] */ RTC_PORT_TYPE enPortType,
            /* [out] */ __RPC__out long *plMinValue,
            /* [out] */ __RPC__out long *plMaxValue);
        
        END_INTERFACE
    } IRTCClientPortManagementVtbl;

    interface IRTCClientPortManagement
    {
        CONST_VTBL struct IRTCClientPortManagementVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IRTCClientPortManagement_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IRTCClientPortManagement_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IRTCClientPortManagement_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IRTCClientPortManagement_StartListenAddressAndPort(This,bstrInternalLocalAddress,lInternalLocalPort)	\
    ( (This)->lpVtbl -> StartListenAddressAndPort(This,bstrInternalLocalAddress,lInternalLocalPort) ) 

#define IRTCClientPortManagement_StopListenAddressAndPort(This,bstrInternalLocalAddress,lInternalLocalPort)	\
    ( (This)->lpVtbl -> StopListenAddressAndPort(This,bstrInternalLocalAddress,lInternalLocalPort) ) 

#define IRTCClientPortManagement_GetPortRange(This,enPortType,plMinValue,plMaxValue)	\
    ( (This)->lpVtbl -> GetPortRange(This,enPortType,plMinValue,plMaxValue) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IRTCClientPortManagement_INTERFACE_DEFINED__ */


#ifndef __IRTCUserSearch_INTERFACE_DEFINED__
#define __IRTCUserSearch_INTERFACE_DEFINED__

/* interface IRTCUserSearch */
/* [unique][helpstring][uuid][object] */ 


EXTERN_C const IID IID_IRTCUserSearch;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("B619882B-860C-4db4-BE1B-693B6505BBE5")
    IRTCUserSearch : public IUnknown
    {
    public:
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE CreateQuery( 
            /* [retval][out] */ __RPC__deref_out_opt IRTCUserSearchQuery **ppQuery) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE ExecuteSearch( 
            /* [in] */ __RPC__in_opt IRTCUserSearchQuery *pQuery,
            /* [in] */ __RPC__in_opt IRTCProfile *pProfile,
            /* [in] */ LONG_PTR lCookie) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IRTCUserSearchVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IRTCUserSearch * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IRTCUserSearch * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IRTCUserSearch * This);
        
        DECLSPEC_XFGVIRT(IRTCUserSearch, CreateQuery)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *CreateQuery )( 
            __RPC__in IRTCUserSearch * This,
            /* [retval][out] */ __RPC__deref_out_opt IRTCUserSearchQuery **ppQuery);
        
        DECLSPEC_XFGVIRT(IRTCUserSearch, ExecuteSearch)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *ExecuteSearch )( 
            __RPC__in IRTCUserSearch * This,
            /* [in] */ __RPC__in_opt IRTCUserSearchQuery *pQuery,
            /* [in] */ __RPC__in_opt IRTCProfile *pProfile,
            /* [in] */ LONG_PTR lCookie);
        
        END_INTERFACE
    } IRTCUserSearchVtbl;

    interface IRTCUserSearch
    {
        CONST_VTBL struct IRTCUserSearchVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IRTCUserSearch_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IRTCUserSearch_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IRTCUserSearch_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IRTCUserSearch_CreateQuery(This,ppQuery)	\
    ( (This)->lpVtbl -> CreateQuery(This,ppQuery) ) 

#define IRTCUserSearch_ExecuteSearch(This,pQuery,pProfile,lCookie)	\
    ( (This)->lpVtbl -> ExecuteSearch(This,pQuery,pProfile,lCookie) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IRTCUserSearch_INTERFACE_DEFINED__ */


#ifndef __IRTCUserSearchQuery_INTERFACE_DEFINED__
#define __IRTCUserSearchQuery_INTERFACE_DEFINED__

/* interface IRTCUserSearchQuery */
/* [unique][helpstring][uuid][object] */ 


EXTERN_C const IID IID_IRTCUserSearchQuery;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("288300F5-D23A-4365-9A73-9985C98C2881")
    IRTCUserSearchQuery : public IUnknown
    {
    public:
        virtual /* [helpstring][propput] */ HRESULT STDMETHODCALLTYPE put_SearchTerm( 
            /* [in] */ __RPC__in BSTR bstrName,
            /* [in] */ __RPC__in BSTR bstrValue) = 0;
        
        virtual /* [helpstring][propget] */ HRESULT STDMETHODCALLTYPE get_SearchTerm( 
            /* [in] */ __RPC__in BSTR bstrName,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrValue) = 0;
        
        virtual /* [helpstring][propget] */ HRESULT STDMETHODCALLTYPE get_SearchTerms( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrNames) = 0;
        
        virtual /* [helpstring][propput] */ HRESULT STDMETHODCALLTYPE put_SearchPreference( 
            /* [in] */ RTC_USER_SEARCH_PREFERENCE enPreference,
            /* [in] */ long lValue) = 0;
        
        virtual /* [helpstring][propget] */ HRESULT STDMETHODCALLTYPE get_SearchPreference( 
            /* [in] */ RTC_USER_SEARCH_PREFERENCE enPreference,
            /* [retval][out] */ __RPC__out long *plValue) = 0;
        
        virtual /* [helpstring][propput] */ HRESULT STDMETHODCALLTYPE put_SearchDomain( 
            /* [in] */ __RPC__in BSTR bstrDomain) = 0;
        
        virtual /* [helpstring][propget] */ HRESULT STDMETHODCALLTYPE get_SearchDomain( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrDomain) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IRTCUserSearchQueryVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IRTCUserSearchQuery * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IRTCUserSearchQuery * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IRTCUserSearchQuery * This);
        
        DECLSPEC_XFGVIRT(IRTCUserSearchQuery, put_SearchTerm)
        /* [helpstring][propput] */ HRESULT ( STDMETHODCALLTYPE *put_SearchTerm )( 
            __RPC__in IRTCUserSearchQuery * This,
            /* [in] */ __RPC__in BSTR bstrName,
            /* [in] */ __RPC__in BSTR bstrValue);
        
        DECLSPEC_XFGVIRT(IRTCUserSearchQuery, get_SearchTerm)
        /* [helpstring][propget] */ HRESULT ( STDMETHODCALLTYPE *get_SearchTerm )( 
            __RPC__in IRTCUserSearchQuery * This,
            /* [in] */ __RPC__in BSTR bstrName,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrValue);
        
        DECLSPEC_XFGVIRT(IRTCUserSearchQuery, get_SearchTerms)
        /* [helpstring][propget] */ HRESULT ( STDMETHODCALLTYPE *get_SearchTerms )( 
            __RPC__in IRTCUserSearchQuery * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrNames);
        
        DECLSPEC_XFGVIRT(IRTCUserSearchQuery, put_SearchPreference)
        /* [helpstring][propput] */ HRESULT ( STDMETHODCALLTYPE *put_SearchPreference )( 
            __RPC__in IRTCUserSearchQuery * This,
            /* [in] */ RTC_USER_SEARCH_PREFERENCE enPreference,
            /* [in] */ long lValue);
        
        DECLSPEC_XFGVIRT(IRTCUserSearchQuery, get_SearchPreference)
        /* [helpstring][propget] */ HRESULT ( STDMETHODCALLTYPE *get_SearchPreference )( 
            __RPC__in IRTCUserSearchQuery * This,
            /* [in] */ RTC_USER_SEARCH_PREFERENCE enPreference,
            /* [retval][out] */ __RPC__out long *plValue);
        
        DECLSPEC_XFGVIRT(IRTCUserSearchQuery, put_SearchDomain)
        /* [helpstring][propput] */ HRESULT ( STDMETHODCALLTYPE *put_SearchDomain )( 
            __RPC__in IRTCUserSearchQuery * This,
            /* [in] */ __RPC__in BSTR bstrDomain);
        
        DECLSPEC_XFGVIRT(IRTCUserSearchQuery, get_SearchDomain)
        /* [helpstring][propget] */ HRESULT ( STDMETHODCALLTYPE *get_SearchDomain )( 
            __RPC__in IRTCUserSearchQuery * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrDomain);
        
        END_INTERFACE
    } IRTCUserSearchQueryVtbl;

    interface IRTCUserSearchQuery
    {
        CONST_VTBL struct IRTCUserSearchQueryVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IRTCUserSearchQuery_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IRTCUserSearchQuery_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IRTCUserSearchQuery_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IRTCUserSearchQuery_put_SearchTerm(This,bstrName,bstrValue)	\
    ( (This)->lpVtbl -> put_SearchTerm(This,bstrName,bstrValue) ) 

#define IRTCUserSearchQuery_get_SearchTerm(This,bstrName,pbstrValue)	\
    ( (This)->lpVtbl -> get_SearchTerm(This,bstrName,pbstrValue) ) 

#define IRTCUserSearchQuery_get_SearchTerms(This,pbstrNames)	\
    ( (This)->lpVtbl -> get_SearchTerms(This,pbstrNames) ) 

#define IRTCUserSearchQuery_put_SearchPreference(This,enPreference,lValue)	\
    ( (This)->lpVtbl -> put_SearchPreference(This,enPreference,lValue) ) 

#define IRTCUserSearchQuery_get_SearchPreference(This,enPreference,plValue)	\
    ( (This)->lpVtbl -> get_SearchPreference(This,enPreference,plValue) ) 

#define IRTCUserSearchQuery_put_SearchDomain(This,bstrDomain)	\
    ( (This)->lpVtbl -> put_SearchDomain(This,bstrDomain) ) 

#define IRTCUserSearchQuery_get_SearchDomain(This,pbstrDomain)	\
    ( (This)->lpVtbl -> get_SearchDomain(This,pbstrDomain) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IRTCUserSearchQuery_INTERFACE_DEFINED__ */


#ifndef __IRTCUserSearchResult_INTERFACE_DEFINED__
#define __IRTCUserSearchResult_INTERFACE_DEFINED__

/* interface IRTCUserSearchResult */
/* [unique][helpstring][uuid][object] */ 


EXTERN_C const IID IID_IRTCUserSearchResult;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("851278B2-9592-480f-8DB5-2DE86B26B54D")
    IRTCUserSearchResult : public IUnknown
    {
    public:
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Value( 
            /* [in] */ RTC_USER_SEARCH_COLUMN enColumn,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrValue) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IRTCUserSearchResultVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IRTCUserSearchResult * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IRTCUserSearchResult * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IRTCUserSearchResult * This);
        
        DECLSPEC_XFGVIRT(IRTCUserSearchResult, get_Value)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Value )( 
            __RPC__in IRTCUserSearchResult * This,
            /* [in] */ RTC_USER_SEARCH_COLUMN enColumn,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrValue);
        
        END_INTERFACE
    } IRTCUserSearchResultVtbl;

    interface IRTCUserSearchResult
    {
        CONST_VTBL struct IRTCUserSearchResultVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IRTCUserSearchResult_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IRTCUserSearchResult_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IRTCUserSearchResult_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IRTCUserSearchResult_get_Value(This,enColumn,pbstrValue)	\
    ( (This)->lpVtbl -> get_Value(This,enColumn,pbstrValue) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IRTCUserSearchResult_INTERFACE_DEFINED__ */


#ifndef __IRTCEnumUserSearchResults_INTERFACE_DEFINED__
#define __IRTCEnumUserSearchResults_INTERFACE_DEFINED__

/* interface IRTCEnumUserSearchResults */
/* [unique][helpstring][uuid][object] */ 


EXTERN_C const IID IID_IRTCEnumUserSearchResults;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("83D4D877-AA5D-4a5b-8D0E-002A8067E0E8")
    IRTCEnumUserSearchResults : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE Next( 
            /* [in] */ ULONG celt,
            /* [size_is][out] */ __RPC__out_ecount_full(celt) IRTCUserSearchResult **ppElements,
            /* [full][out][in] */ __RPC__inout_opt ULONG *pceltFetched) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Reset( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Skip( 
            /* [in] */ ULONG celt) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Clone( 
            /* [retval][out] */ __RPC__deref_out_opt IRTCEnumUserSearchResults **ppEnum) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IRTCEnumUserSearchResultsVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IRTCEnumUserSearchResults * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IRTCEnumUserSearchResults * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IRTCEnumUserSearchResults * This);
        
        DECLSPEC_XFGVIRT(IRTCEnumUserSearchResults, Next)
        HRESULT ( STDMETHODCALLTYPE *Next )( 
            __RPC__in IRTCEnumUserSearchResults * This,
            /* [in] */ ULONG celt,
            /* [size_is][out] */ __RPC__out_ecount_full(celt) IRTCUserSearchResult **ppElements,
            /* [full][out][in] */ __RPC__inout_opt ULONG *pceltFetched);
        
        DECLSPEC_XFGVIRT(IRTCEnumUserSearchResults, Reset)
        HRESULT ( STDMETHODCALLTYPE *Reset )( 
            __RPC__in IRTCEnumUserSearchResults * This);
        
        DECLSPEC_XFGVIRT(IRTCEnumUserSearchResults, Skip)
        HRESULT ( STDMETHODCALLTYPE *Skip )( 
            __RPC__in IRTCEnumUserSearchResults * This,
            /* [in] */ ULONG celt);
        
        DECLSPEC_XFGVIRT(IRTCEnumUserSearchResults, Clone)
        HRESULT ( STDMETHODCALLTYPE *Clone )( 
            __RPC__in IRTCEnumUserSearchResults * This,
            /* [retval][out] */ __RPC__deref_out_opt IRTCEnumUserSearchResults **ppEnum);
        
        END_INTERFACE
    } IRTCEnumUserSearchResultsVtbl;

    interface IRTCEnumUserSearchResults
    {
        CONST_VTBL struct IRTCEnumUserSearchResultsVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IRTCEnumUserSearchResults_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IRTCEnumUserSearchResults_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IRTCEnumUserSearchResults_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IRTCEnumUserSearchResults_Next(This,celt,ppElements,pceltFetched)	\
    ( (This)->lpVtbl -> Next(This,celt,ppElements,pceltFetched) ) 

#define IRTCEnumUserSearchResults_Reset(This)	\
    ( (This)->lpVtbl -> Reset(This) ) 

#define IRTCEnumUserSearchResults_Skip(This,celt)	\
    ( (This)->lpVtbl -> Skip(This,celt) ) 

#define IRTCEnumUserSearchResults_Clone(This,ppEnum)	\
    ( (This)->lpVtbl -> Clone(This,ppEnum) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IRTCEnumUserSearchResults_INTERFACE_DEFINED__ */


#ifndef __IRTCUserSearchResultsEvent_INTERFACE_DEFINED__
#define __IRTCUserSearchResultsEvent_INTERFACE_DEFINED__

/* interface IRTCUserSearchResultsEvent */
/* [unique][helpstring][dual][uuid][object] */ 


EXTERN_C const IID IID_IRTCUserSearchResultsEvent;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("D8C8C3CD-7FAC-4088-81C5-C24CBC0938E3")
    IRTCUserSearchResultsEvent : public IDispatch
    {
    public:
        virtual /* [helpstring][id][hidden] */ HRESULT STDMETHODCALLTYPE EnumerateResults( 
            /* [retval][out] */ __RPC__deref_out_opt IRTCEnumUserSearchResults **ppEnum) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Results( 
            /* [retval][out] */ __RPC__deref_out_opt IRTCCollection **ppCollection) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Profile( 
            /* [retval][out] */ __RPC__deref_out_opt IRTCProfile2 **ppProfile) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Query( 
            /* [retval][out] */ __RPC__deref_out_opt IRTCUserSearchQuery **ppQuery) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Cookie( 
            /* [retval][out] */ __RPC__out LONG_PTR *plCookie) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_StatusCode( 
            /* [retval][out] */ __RPC__out long *plStatusCode) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_MoreAvailable( 
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pfMoreAvailable) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IRTCUserSearchResultsEventVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IRTCUserSearchResultsEvent * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IRTCUserSearchResultsEvent * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IRTCUserSearchResultsEvent * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IRTCUserSearchResultsEvent * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IRTCUserSearchResultsEvent * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IRTCUserSearchResultsEvent * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IRTCUserSearchResultsEvent * This,
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
        
        DECLSPEC_XFGVIRT(IRTCUserSearchResultsEvent, EnumerateResults)
        /* [helpstring][id][hidden] */ HRESULT ( STDMETHODCALLTYPE *EnumerateResults )( 
            __RPC__in IRTCUserSearchResultsEvent * This,
            /* [retval][out] */ __RPC__deref_out_opt IRTCEnumUserSearchResults **ppEnum);
        
        DECLSPEC_XFGVIRT(IRTCUserSearchResultsEvent, get_Results)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Results )( 
            __RPC__in IRTCUserSearchResultsEvent * This,
            /* [retval][out] */ __RPC__deref_out_opt IRTCCollection **ppCollection);
        
        DECLSPEC_XFGVIRT(IRTCUserSearchResultsEvent, get_Profile)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Profile )( 
            __RPC__in IRTCUserSearchResultsEvent * This,
            /* [retval][out] */ __RPC__deref_out_opt IRTCProfile2 **ppProfile);
        
        DECLSPEC_XFGVIRT(IRTCUserSearchResultsEvent, get_Query)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Query )( 
            __RPC__in IRTCUserSearchResultsEvent * This,
            /* [retval][out] */ __RPC__deref_out_opt IRTCUserSearchQuery **ppQuery);
        
        DECLSPEC_XFGVIRT(IRTCUserSearchResultsEvent, get_Cookie)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Cookie )( 
            __RPC__in IRTCUserSearchResultsEvent * This,
            /* [retval][out] */ __RPC__out LONG_PTR *plCookie);
        
        DECLSPEC_XFGVIRT(IRTCUserSearchResultsEvent, get_StatusCode)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_StatusCode )( 
            __RPC__in IRTCUserSearchResultsEvent * This,
            /* [retval][out] */ __RPC__out long *plStatusCode);
        
        DECLSPEC_XFGVIRT(IRTCUserSearchResultsEvent, get_MoreAvailable)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_MoreAvailable )( 
            __RPC__in IRTCUserSearchResultsEvent * This,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pfMoreAvailable);
        
        END_INTERFACE
    } IRTCUserSearchResultsEventVtbl;

    interface IRTCUserSearchResultsEvent
    {
        CONST_VTBL struct IRTCUserSearchResultsEventVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IRTCUserSearchResultsEvent_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IRTCUserSearchResultsEvent_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IRTCUserSearchResultsEvent_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IRTCUserSearchResultsEvent_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IRTCUserSearchResultsEvent_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IRTCUserSearchResultsEvent_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IRTCUserSearchResultsEvent_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IRTCUserSearchResultsEvent_EnumerateResults(This,ppEnum)	\
    ( (This)->lpVtbl -> EnumerateResults(This,ppEnum) ) 

#define IRTCUserSearchResultsEvent_get_Results(This,ppCollection)	\
    ( (This)->lpVtbl -> get_Results(This,ppCollection) ) 

#define IRTCUserSearchResultsEvent_get_Profile(This,ppProfile)	\
    ( (This)->lpVtbl -> get_Profile(This,ppProfile) ) 

#define IRTCUserSearchResultsEvent_get_Query(This,ppQuery)	\
    ( (This)->lpVtbl -> get_Query(This,ppQuery) ) 

#define IRTCUserSearchResultsEvent_get_Cookie(This,plCookie)	\
    ( (This)->lpVtbl -> get_Cookie(This,plCookie) ) 

#define IRTCUserSearchResultsEvent_get_StatusCode(This,plStatusCode)	\
    ( (This)->lpVtbl -> get_StatusCode(This,plStatusCode) ) 

#define IRTCUserSearchResultsEvent_get_MoreAvailable(This,pfMoreAvailable)	\
    ( (This)->lpVtbl -> get_MoreAvailable(This,pfMoreAvailable) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IRTCUserSearchResultsEvent_INTERFACE_DEFINED__ */


#ifndef __IRTCSessionReferStatusEvent_INTERFACE_DEFINED__
#define __IRTCSessionReferStatusEvent_INTERFACE_DEFINED__

/* interface IRTCSessionReferStatusEvent */
/* [unique][helpstring][dual][uuid][object] */ 


EXTERN_C const IID IID_IRTCSessionReferStatusEvent;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("3D8FC2CD-5D76-44AB-BB68-2A80353B34A2")
    IRTCSessionReferStatusEvent : public IDispatch
    {
    public:
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Session( 
            /* [retval][out] */ __RPC__deref_out_opt IRTCSession2 **ppSession) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_ReferStatus( 
            /* [retval][out] */ __RPC__out RTC_SESSION_REFER_STATUS *penReferStatus) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_StatusCode( 
            /* [retval][out] */ __RPC__out long *plStatusCode) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_StatusText( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrStatusText) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IRTCSessionReferStatusEventVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IRTCSessionReferStatusEvent * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IRTCSessionReferStatusEvent * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IRTCSessionReferStatusEvent * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IRTCSessionReferStatusEvent * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IRTCSessionReferStatusEvent * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IRTCSessionReferStatusEvent * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IRTCSessionReferStatusEvent * This,
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
        
        DECLSPEC_XFGVIRT(IRTCSessionReferStatusEvent, get_Session)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Session )( 
            __RPC__in IRTCSessionReferStatusEvent * This,
            /* [retval][out] */ __RPC__deref_out_opt IRTCSession2 **ppSession);
        
        DECLSPEC_XFGVIRT(IRTCSessionReferStatusEvent, get_ReferStatus)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_ReferStatus )( 
            __RPC__in IRTCSessionReferStatusEvent * This,
            /* [retval][out] */ __RPC__out RTC_SESSION_REFER_STATUS *penReferStatus);
        
        DECLSPEC_XFGVIRT(IRTCSessionReferStatusEvent, get_StatusCode)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_StatusCode )( 
            __RPC__in IRTCSessionReferStatusEvent * This,
            /* [retval][out] */ __RPC__out long *plStatusCode);
        
        DECLSPEC_XFGVIRT(IRTCSessionReferStatusEvent, get_StatusText)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_StatusText )( 
            __RPC__in IRTCSessionReferStatusEvent * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrStatusText);
        
        END_INTERFACE
    } IRTCSessionReferStatusEventVtbl;

    interface IRTCSessionReferStatusEvent
    {
        CONST_VTBL struct IRTCSessionReferStatusEventVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IRTCSessionReferStatusEvent_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IRTCSessionReferStatusEvent_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IRTCSessionReferStatusEvent_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IRTCSessionReferStatusEvent_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IRTCSessionReferStatusEvent_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IRTCSessionReferStatusEvent_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IRTCSessionReferStatusEvent_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IRTCSessionReferStatusEvent_get_Session(This,ppSession)	\
    ( (This)->lpVtbl -> get_Session(This,ppSession) ) 

#define IRTCSessionReferStatusEvent_get_ReferStatus(This,penReferStatus)	\
    ( (This)->lpVtbl -> get_ReferStatus(This,penReferStatus) ) 

#define IRTCSessionReferStatusEvent_get_StatusCode(This,plStatusCode)	\
    ( (This)->lpVtbl -> get_StatusCode(This,plStatusCode) ) 

#define IRTCSessionReferStatusEvent_get_StatusText(This,pbstrStatusText)	\
    ( (This)->lpVtbl -> get_StatusText(This,pbstrStatusText) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IRTCSessionReferStatusEvent_INTERFACE_DEFINED__ */


#ifndef __IRTCSessionReferredEvent_INTERFACE_DEFINED__
#define __IRTCSessionReferredEvent_INTERFACE_DEFINED__

/* interface IRTCSessionReferredEvent */
/* [unique][helpstring][dual][uuid][object] */ 


EXTERN_C const IID IID_IRTCSessionReferredEvent;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("176A6828-4FCC-4F28-A862-04597A6CF1C4")
    IRTCSessionReferredEvent : public IDispatch
    {
    public:
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Session( 
            /* [retval][out] */ __RPC__deref_out_opt IRTCSession2 **ppSession) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_ReferredByURI( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrReferredByURI) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_ReferToURI( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrReferoURI) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_ReferCookie( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrReferCookie) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE Accept( void) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE Reject( void) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE SetReferredSessionState( 
            /* [in] */ RTC_SESSION_STATE enState) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IRTCSessionReferredEventVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IRTCSessionReferredEvent * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IRTCSessionReferredEvent * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IRTCSessionReferredEvent * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IRTCSessionReferredEvent * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IRTCSessionReferredEvent * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IRTCSessionReferredEvent * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IRTCSessionReferredEvent * This,
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
        
        DECLSPEC_XFGVIRT(IRTCSessionReferredEvent, get_Session)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Session )( 
            __RPC__in IRTCSessionReferredEvent * This,
            /* [retval][out] */ __RPC__deref_out_opt IRTCSession2 **ppSession);
        
        DECLSPEC_XFGVIRT(IRTCSessionReferredEvent, get_ReferredByURI)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_ReferredByURI )( 
            __RPC__in IRTCSessionReferredEvent * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrReferredByURI);
        
        DECLSPEC_XFGVIRT(IRTCSessionReferredEvent, get_ReferToURI)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_ReferToURI )( 
            __RPC__in IRTCSessionReferredEvent * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrReferoURI);
        
        DECLSPEC_XFGVIRT(IRTCSessionReferredEvent, get_ReferCookie)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_ReferCookie )( 
            __RPC__in IRTCSessionReferredEvent * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrReferCookie);
        
        DECLSPEC_XFGVIRT(IRTCSessionReferredEvent, Accept)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *Accept )( 
            __RPC__in IRTCSessionReferredEvent * This);
        
        DECLSPEC_XFGVIRT(IRTCSessionReferredEvent, Reject)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *Reject )( 
            __RPC__in IRTCSessionReferredEvent * This);
        
        DECLSPEC_XFGVIRT(IRTCSessionReferredEvent, SetReferredSessionState)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *SetReferredSessionState )( 
            __RPC__in IRTCSessionReferredEvent * This,
            /* [in] */ RTC_SESSION_STATE enState);
        
        END_INTERFACE
    } IRTCSessionReferredEventVtbl;

    interface IRTCSessionReferredEvent
    {
        CONST_VTBL struct IRTCSessionReferredEventVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IRTCSessionReferredEvent_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IRTCSessionReferredEvent_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IRTCSessionReferredEvent_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IRTCSessionReferredEvent_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IRTCSessionReferredEvent_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IRTCSessionReferredEvent_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IRTCSessionReferredEvent_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IRTCSessionReferredEvent_get_Session(This,ppSession)	\
    ( (This)->lpVtbl -> get_Session(This,ppSession) ) 

#define IRTCSessionReferredEvent_get_ReferredByURI(This,pbstrReferredByURI)	\
    ( (This)->lpVtbl -> get_ReferredByURI(This,pbstrReferredByURI) ) 

#define IRTCSessionReferredEvent_get_ReferToURI(This,pbstrReferoURI)	\
    ( (This)->lpVtbl -> get_ReferToURI(This,pbstrReferoURI) ) 

#define IRTCSessionReferredEvent_get_ReferCookie(This,pbstrReferCookie)	\
    ( (This)->lpVtbl -> get_ReferCookie(This,pbstrReferCookie) ) 

#define IRTCSessionReferredEvent_Accept(This)	\
    ( (This)->lpVtbl -> Accept(This) ) 

#define IRTCSessionReferredEvent_Reject(This)	\
    ( (This)->lpVtbl -> Reject(This) ) 

#define IRTCSessionReferredEvent_SetReferredSessionState(This,enState)	\
    ( (This)->lpVtbl -> SetReferredSessionState(This,enState) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IRTCSessionReferredEvent_INTERFACE_DEFINED__ */


#ifndef __IRTCSessionDescriptionManager_INTERFACE_DEFINED__
#define __IRTCSessionDescriptionManager_INTERFACE_DEFINED__

/* interface IRTCSessionDescriptionManager */
/* [unique][oleautomation][helpstring][uuid][object] */ 


EXTERN_C const IID IID_IRTCSessionDescriptionManager;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("BA7F518E-D336-4070-93A6-865395C843F9")
    IRTCSessionDescriptionManager : public IUnknown
    {
    public:
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE EvaluateSessionDescription( 
            /* [in] */ __RPC__in BSTR bstrContentType,
            /* [in] */ __RPC__in BSTR bstrSessionDescription,
            /* [out][in] */ __RPC__inout VARIANT_BOOL *pfApplicationSession) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IRTCSessionDescriptionManagerVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IRTCSessionDescriptionManager * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IRTCSessionDescriptionManager * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IRTCSessionDescriptionManager * This);
        
        DECLSPEC_XFGVIRT(IRTCSessionDescriptionManager, EvaluateSessionDescription)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *EvaluateSessionDescription )( 
            __RPC__in IRTCSessionDescriptionManager * This,
            /* [in] */ __RPC__in BSTR bstrContentType,
            /* [in] */ __RPC__in BSTR bstrSessionDescription,
            /* [out][in] */ __RPC__inout VARIANT_BOOL *pfApplicationSession);
        
        END_INTERFACE
    } IRTCSessionDescriptionManagerVtbl;

    interface IRTCSessionDescriptionManager
    {
        CONST_VTBL struct IRTCSessionDescriptionManagerVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IRTCSessionDescriptionManager_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IRTCSessionDescriptionManager_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IRTCSessionDescriptionManager_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IRTCSessionDescriptionManager_EvaluateSessionDescription(This,bstrContentType,bstrSessionDescription,pfApplicationSession)	\
    ( (This)->lpVtbl -> EvaluateSessionDescription(This,bstrContentType,bstrSessionDescription,pfApplicationSession) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IRTCSessionDescriptionManager_INTERFACE_DEFINED__ */


#ifndef __IRTCEnumPresenceDevices_INTERFACE_DEFINED__
#define __IRTCEnumPresenceDevices_INTERFACE_DEFINED__

/* interface IRTCEnumPresenceDevices */
/* [unique][helpstring][hidden][uuid][object] */ 


EXTERN_C const IID IID_IRTCEnumPresenceDevices;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("708C2AB7-8BF8-42f8-8C7D-635197AD5539")
    IRTCEnumPresenceDevices : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE Next( 
            /* [in] */ ULONG celt,
            /* [size_is][out] */ __RPC__out_ecount_full(celt) IRTCPresenceDevice **ppElements,
            /* [full][out][in] */ __RPC__inout_opt ULONG *pceltFetched) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Reset( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Skip( 
            /* [in] */ ULONG celt) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Clone( 
            /* [retval][out] */ __RPC__deref_out_opt IRTCEnumPresenceDevices **ppEnum) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IRTCEnumPresenceDevicesVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IRTCEnumPresenceDevices * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IRTCEnumPresenceDevices * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IRTCEnumPresenceDevices * This);
        
        DECLSPEC_XFGVIRT(IRTCEnumPresenceDevices, Next)
        HRESULT ( STDMETHODCALLTYPE *Next )( 
            __RPC__in IRTCEnumPresenceDevices * This,
            /* [in] */ ULONG celt,
            /* [size_is][out] */ __RPC__out_ecount_full(celt) IRTCPresenceDevice **ppElements,
            /* [full][out][in] */ __RPC__inout_opt ULONG *pceltFetched);
        
        DECLSPEC_XFGVIRT(IRTCEnumPresenceDevices, Reset)
        HRESULT ( STDMETHODCALLTYPE *Reset )( 
            __RPC__in IRTCEnumPresenceDevices * This);
        
        DECLSPEC_XFGVIRT(IRTCEnumPresenceDevices, Skip)
        HRESULT ( STDMETHODCALLTYPE *Skip )( 
            __RPC__in IRTCEnumPresenceDevices * This,
            /* [in] */ ULONG celt);
        
        DECLSPEC_XFGVIRT(IRTCEnumPresenceDevices, Clone)
        HRESULT ( STDMETHODCALLTYPE *Clone )( 
            __RPC__in IRTCEnumPresenceDevices * This,
            /* [retval][out] */ __RPC__deref_out_opt IRTCEnumPresenceDevices **ppEnum);
        
        END_INTERFACE
    } IRTCEnumPresenceDevicesVtbl;

    interface IRTCEnumPresenceDevices
    {
        CONST_VTBL struct IRTCEnumPresenceDevicesVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IRTCEnumPresenceDevices_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IRTCEnumPresenceDevices_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IRTCEnumPresenceDevices_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IRTCEnumPresenceDevices_Next(This,celt,ppElements,pceltFetched)	\
    ( (This)->lpVtbl -> Next(This,celt,ppElements,pceltFetched) ) 

#define IRTCEnumPresenceDevices_Reset(This)	\
    ( (This)->lpVtbl -> Reset(This) ) 

#define IRTCEnumPresenceDevices_Skip(This,celt)	\
    ( (This)->lpVtbl -> Skip(This,celt) ) 

#define IRTCEnumPresenceDevices_Clone(This,ppEnum)	\
    ( (This)->lpVtbl -> Clone(This,ppEnum) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IRTCEnumPresenceDevices_INTERFACE_DEFINED__ */


#ifndef __IRTCPresenceDevice_INTERFACE_DEFINED__
#define __IRTCPresenceDevice_INTERFACE_DEFINED__

/* interface IRTCPresenceDevice */
/* [unique][helpstring][uuid][object] */ 


EXTERN_C const IID IID_IRTCPresenceDevice;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("BC6A90DD-AD9A-48da-9B0C-2515E38521AD")
    IRTCPresenceDevice : public IUnknown
    {
    public:
        virtual /* [helpstring][propget] */ HRESULT STDMETHODCALLTYPE get_Status( 
            /* [retval][out] */ __RPC__out RTC_PRESENCE_STATUS *penStatus) = 0;
        
        virtual /* [helpstring][propget] */ HRESULT STDMETHODCALLTYPE get_Notes( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrNotes) = 0;
        
        virtual /* [helpstring][propget] */ HRESULT STDMETHODCALLTYPE get_PresenceProperty( 
            /* [in] */ RTC_PRESENCE_PROPERTY enProperty,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrProperty) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE GetPresenceData( 
            /* [out] */ __RPC__deref_out_opt BSTR *pbstrNamespace,
            /* [out] */ __RPC__deref_out_opt BSTR *pbstrData) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IRTCPresenceDeviceVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IRTCPresenceDevice * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IRTCPresenceDevice * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IRTCPresenceDevice * This);
        
        DECLSPEC_XFGVIRT(IRTCPresenceDevice, get_Status)
        /* [helpstring][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Status )( 
            __RPC__in IRTCPresenceDevice * This,
            /* [retval][out] */ __RPC__out RTC_PRESENCE_STATUS *penStatus);
        
        DECLSPEC_XFGVIRT(IRTCPresenceDevice, get_Notes)
        /* [helpstring][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Notes )( 
            __RPC__in IRTCPresenceDevice * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrNotes);
        
        DECLSPEC_XFGVIRT(IRTCPresenceDevice, get_PresenceProperty)
        /* [helpstring][propget] */ HRESULT ( STDMETHODCALLTYPE *get_PresenceProperty )( 
            __RPC__in IRTCPresenceDevice * This,
            /* [in] */ RTC_PRESENCE_PROPERTY enProperty,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrProperty);
        
        DECLSPEC_XFGVIRT(IRTCPresenceDevice, GetPresenceData)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *GetPresenceData )( 
            __RPC__in IRTCPresenceDevice * This,
            /* [out] */ __RPC__deref_out_opt BSTR *pbstrNamespace,
            /* [out] */ __RPC__deref_out_opt BSTR *pbstrData);
        
        END_INTERFACE
    } IRTCPresenceDeviceVtbl;

    interface IRTCPresenceDevice
    {
        CONST_VTBL struct IRTCPresenceDeviceVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IRTCPresenceDevice_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IRTCPresenceDevice_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IRTCPresenceDevice_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IRTCPresenceDevice_get_Status(This,penStatus)	\
    ( (This)->lpVtbl -> get_Status(This,penStatus) ) 

#define IRTCPresenceDevice_get_Notes(This,pbstrNotes)	\
    ( (This)->lpVtbl -> get_Notes(This,pbstrNotes) ) 

#define IRTCPresenceDevice_get_PresenceProperty(This,enProperty,pbstrProperty)	\
    ( (This)->lpVtbl -> get_PresenceProperty(This,enProperty,pbstrProperty) ) 

#define IRTCPresenceDevice_GetPresenceData(This,pbstrNamespace,pbstrData)	\
    ( (This)->lpVtbl -> GetPresenceData(This,pbstrNamespace,pbstrData) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IRTCPresenceDevice_INTERFACE_DEFINED__ */



#ifndef __RTCCORELib_LIBRARY_DEFINED__
#define __RTCCORELib_LIBRARY_DEFINED__

/* library RTCCORELib */
/* [helpstring][version][uuid] */ 

























































EXTERN_C const IID LIBID_RTCCORELib;

#ifndef __IRTCDispatchEventNotification_DISPINTERFACE_DEFINED__
#define __IRTCDispatchEventNotification_DISPINTERFACE_DEFINED__

/* dispinterface IRTCDispatchEventNotification */
/* [helpstring][uuid] */ 


EXTERN_C const IID DIID_IRTCDispatchEventNotification;

#if defined(__cplusplus) && !defined(CINTERFACE)

    MIDL_INTERFACE("176ddfbe-fec0-4d55-bc87-84cff1ef7f91")
    IRTCDispatchEventNotification : public IDispatch
    {
    };
    
#else 	/* C style interface */

    typedef struct IRTCDispatchEventNotificationVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IRTCDispatchEventNotification * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IRTCDispatchEventNotification * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IRTCDispatchEventNotification * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IRTCDispatchEventNotification * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IRTCDispatchEventNotification * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IRTCDispatchEventNotification * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IRTCDispatchEventNotification * This,
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
        
        END_INTERFACE
    } IRTCDispatchEventNotificationVtbl;

    interface IRTCDispatchEventNotification
    {
        CONST_VTBL struct IRTCDispatchEventNotificationVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IRTCDispatchEventNotification_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IRTCDispatchEventNotification_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IRTCDispatchEventNotification_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IRTCDispatchEventNotification_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IRTCDispatchEventNotification_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IRTCDispatchEventNotification_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IRTCDispatchEventNotification_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */


#endif 	/* __IRTCDispatchEventNotification_DISPINTERFACE_DEFINED__ */


EXTERN_C const CLSID CLSID_RTCClient;

#ifdef __cplusplus

class DECLSPEC_UUID("7a42ea29-a2b7-40c4-b091-f6f024aa89be")
RTCClient;
#endif
#endif /* __RTCCORELib_LIBRARY_DEFINED__ */

/* interface __MIDL_itf_rtccore_0000_0064 */
/* [local] */ 

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion


extern RPC_IF_HANDLE __MIDL_itf_rtccore_0000_0064_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_rtccore_0000_0064_v0_0_s_ifspec;

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


