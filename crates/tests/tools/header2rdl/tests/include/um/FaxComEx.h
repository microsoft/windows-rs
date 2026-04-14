

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

#ifndef __faxcomex_h__
#define __faxcomex_h__

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

#ifndef __IFaxJobStatus_FWD_DEFINED__
#define __IFaxJobStatus_FWD_DEFINED__
typedef interface IFaxJobStatus IFaxJobStatus;

#endif 	/* __IFaxJobStatus_FWD_DEFINED__ */


#ifndef __IFaxServer_FWD_DEFINED__
#define __IFaxServer_FWD_DEFINED__
typedef interface IFaxServer IFaxServer;

#endif 	/* __IFaxServer_FWD_DEFINED__ */


#ifndef __IFaxDeviceProviders_FWD_DEFINED__
#define __IFaxDeviceProviders_FWD_DEFINED__
typedef interface IFaxDeviceProviders IFaxDeviceProviders;

#endif 	/* __IFaxDeviceProviders_FWD_DEFINED__ */


#ifndef __IFaxDevices_FWD_DEFINED__
#define __IFaxDevices_FWD_DEFINED__
typedef interface IFaxDevices IFaxDevices;

#endif 	/* __IFaxDevices_FWD_DEFINED__ */


#ifndef __IFaxInboundRouting_FWD_DEFINED__
#define __IFaxInboundRouting_FWD_DEFINED__
typedef interface IFaxInboundRouting IFaxInboundRouting;

#endif 	/* __IFaxInboundRouting_FWD_DEFINED__ */


#ifndef __IFaxFolders_FWD_DEFINED__
#define __IFaxFolders_FWD_DEFINED__
typedef interface IFaxFolders IFaxFolders;

#endif 	/* __IFaxFolders_FWD_DEFINED__ */


#ifndef __IFaxLoggingOptions_FWD_DEFINED__
#define __IFaxLoggingOptions_FWD_DEFINED__
typedef interface IFaxLoggingOptions IFaxLoggingOptions;

#endif 	/* __IFaxLoggingOptions_FWD_DEFINED__ */


#ifndef __IFaxActivity_FWD_DEFINED__
#define __IFaxActivity_FWD_DEFINED__
typedef interface IFaxActivity IFaxActivity;

#endif 	/* __IFaxActivity_FWD_DEFINED__ */


#ifndef __IFaxOutboundRouting_FWD_DEFINED__
#define __IFaxOutboundRouting_FWD_DEFINED__
typedef interface IFaxOutboundRouting IFaxOutboundRouting;

#endif 	/* __IFaxOutboundRouting_FWD_DEFINED__ */


#ifndef __IFaxReceiptOptions_FWD_DEFINED__
#define __IFaxReceiptOptions_FWD_DEFINED__
typedef interface IFaxReceiptOptions IFaxReceiptOptions;

#endif 	/* __IFaxReceiptOptions_FWD_DEFINED__ */


#ifndef __IFaxSecurity_FWD_DEFINED__
#define __IFaxSecurity_FWD_DEFINED__
typedef interface IFaxSecurity IFaxSecurity;

#endif 	/* __IFaxSecurity_FWD_DEFINED__ */


#ifndef __IFaxDocument_FWD_DEFINED__
#define __IFaxDocument_FWD_DEFINED__
typedef interface IFaxDocument IFaxDocument;

#endif 	/* __IFaxDocument_FWD_DEFINED__ */


#ifndef __IFaxSender_FWD_DEFINED__
#define __IFaxSender_FWD_DEFINED__
typedef interface IFaxSender IFaxSender;

#endif 	/* __IFaxSender_FWD_DEFINED__ */


#ifndef __IFaxRecipient_FWD_DEFINED__
#define __IFaxRecipient_FWD_DEFINED__
typedef interface IFaxRecipient IFaxRecipient;

#endif 	/* __IFaxRecipient_FWD_DEFINED__ */


#ifndef __IFaxRecipients_FWD_DEFINED__
#define __IFaxRecipients_FWD_DEFINED__
typedef interface IFaxRecipients IFaxRecipients;

#endif 	/* __IFaxRecipients_FWD_DEFINED__ */


#ifndef __IFaxIncomingArchive_FWD_DEFINED__
#define __IFaxIncomingArchive_FWD_DEFINED__
typedef interface IFaxIncomingArchive IFaxIncomingArchive;

#endif 	/* __IFaxIncomingArchive_FWD_DEFINED__ */


#ifndef __IFaxIncomingQueue_FWD_DEFINED__
#define __IFaxIncomingQueue_FWD_DEFINED__
typedef interface IFaxIncomingQueue IFaxIncomingQueue;

#endif 	/* __IFaxIncomingQueue_FWD_DEFINED__ */


#ifndef __IFaxOutgoingArchive_FWD_DEFINED__
#define __IFaxOutgoingArchive_FWD_DEFINED__
typedef interface IFaxOutgoingArchive IFaxOutgoingArchive;

#endif 	/* __IFaxOutgoingArchive_FWD_DEFINED__ */


#ifndef __IFaxOutgoingQueue_FWD_DEFINED__
#define __IFaxOutgoingQueue_FWD_DEFINED__
typedef interface IFaxOutgoingQueue IFaxOutgoingQueue;

#endif 	/* __IFaxOutgoingQueue_FWD_DEFINED__ */


#ifndef __IFaxIncomingMessageIterator_FWD_DEFINED__
#define __IFaxIncomingMessageIterator_FWD_DEFINED__
typedef interface IFaxIncomingMessageIterator IFaxIncomingMessageIterator;

#endif 	/* __IFaxIncomingMessageIterator_FWD_DEFINED__ */


#ifndef __IFaxIncomingMessage_FWD_DEFINED__
#define __IFaxIncomingMessage_FWD_DEFINED__
typedef interface IFaxIncomingMessage IFaxIncomingMessage;

#endif 	/* __IFaxIncomingMessage_FWD_DEFINED__ */


#ifndef __IFaxOutgoingJobs_FWD_DEFINED__
#define __IFaxOutgoingJobs_FWD_DEFINED__
typedef interface IFaxOutgoingJobs IFaxOutgoingJobs;

#endif 	/* __IFaxOutgoingJobs_FWD_DEFINED__ */


#ifndef __IFaxOutgoingJob_FWD_DEFINED__
#define __IFaxOutgoingJob_FWD_DEFINED__
typedef interface IFaxOutgoingJob IFaxOutgoingJob;

#endif 	/* __IFaxOutgoingJob_FWD_DEFINED__ */


#ifndef __IFaxOutgoingMessageIterator_FWD_DEFINED__
#define __IFaxOutgoingMessageIterator_FWD_DEFINED__
typedef interface IFaxOutgoingMessageIterator IFaxOutgoingMessageIterator;

#endif 	/* __IFaxOutgoingMessageIterator_FWD_DEFINED__ */


#ifndef __IFaxOutgoingMessage_FWD_DEFINED__
#define __IFaxOutgoingMessage_FWD_DEFINED__
typedef interface IFaxOutgoingMessage IFaxOutgoingMessage;

#endif 	/* __IFaxOutgoingMessage_FWD_DEFINED__ */


#ifndef __IFaxIncomingJobs_FWD_DEFINED__
#define __IFaxIncomingJobs_FWD_DEFINED__
typedef interface IFaxIncomingJobs IFaxIncomingJobs;

#endif 	/* __IFaxIncomingJobs_FWD_DEFINED__ */


#ifndef __IFaxIncomingJob_FWD_DEFINED__
#define __IFaxIncomingJob_FWD_DEFINED__
typedef interface IFaxIncomingJob IFaxIncomingJob;

#endif 	/* __IFaxIncomingJob_FWD_DEFINED__ */


#ifndef __IFaxDeviceProvider_FWD_DEFINED__
#define __IFaxDeviceProvider_FWD_DEFINED__
typedef interface IFaxDeviceProvider IFaxDeviceProvider;

#endif 	/* __IFaxDeviceProvider_FWD_DEFINED__ */


#ifndef __IFaxDevice_FWD_DEFINED__
#define __IFaxDevice_FWD_DEFINED__
typedef interface IFaxDevice IFaxDevice;

#endif 	/* __IFaxDevice_FWD_DEFINED__ */


#ifndef __IFaxActivityLogging_FWD_DEFINED__
#define __IFaxActivityLogging_FWD_DEFINED__
typedef interface IFaxActivityLogging IFaxActivityLogging;

#endif 	/* __IFaxActivityLogging_FWD_DEFINED__ */


#ifndef __IFaxEventLogging_FWD_DEFINED__
#define __IFaxEventLogging_FWD_DEFINED__
typedef interface IFaxEventLogging IFaxEventLogging;

#endif 	/* __IFaxEventLogging_FWD_DEFINED__ */


#ifndef __IFaxOutboundRoutingGroups_FWD_DEFINED__
#define __IFaxOutboundRoutingGroups_FWD_DEFINED__
typedef interface IFaxOutboundRoutingGroups IFaxOutboundRoutingGroups;

#endif 	/* __IFaxOutboundRoutingGroups_FWD_DEFINED__ */


#ifndef __IFaxOutboundRoutingGroup_FWD_DEFINED__
#define __IFaxOutboundRoutingGroup_FWD_DEFINED__
typedef interface IFaxOutboundRoutingGroup IFaxOutboundRoutingGroup;

#endif 	/* __IFaxOutboundRoutingGroup_FWD_DEFINED__ */


#ifndef __IFaxDeviceIds_FWD_DEFINED__
#define __IFaxDeviceIds_FWD_DEFINED__
typedef interface IFaxDeviceIds IFaxDeviceIds;

#endif 	/* __IFaxDeviceIds_FWD_DEFINED__ */


#ifndef __IFaxOutboundRoutingRules_FWD_DEFINED__
#define __IFaxOutboundRoutingRules_FWD_DEFINED__
typedef interface IFaxOutboundRoutingRules IFaxOutboundRoutingRules;

#endif 	/* __IFaxOutboundRoutingRules_FWD_DEFINED__ */


#ifndef __IFaxOutboundRoutingRule_FWD_DEFINED__
#define __IFaxOutboundRoutingRule_FWD_DEFINED__
typedef interface IFaxOutboundRoutingRule IFaxOutboundRoutingRule;

#endif 	/* __IFaxOutboundRoutingRule_FWD_DEFINED__ */


#ifndef __IFaxInboundRoutingExtensions_FWD_DEFINED__
#define __IFaxInboundRoutingExtensions_FWD_DEFINED__
typedef interface IFaxInboundRoutingExtensions IFaxInboundRoutingExtensions;

#endif 	/* __IFaxInboundRoutingExtensions_FWD_DEFINED__ */


#ifndef __IFaxInboundRoutingExtension_FWD_DEFINED__
#define __IFaxInboundRoutingExtension_FWD_DEFINED__
typedef interface IFaxInboundRoutingExtension IFaxInboundRoutingExtension;

#endif 	/* __IFaxInboundRoutingExtension_FWD_DEFINED__ */


#ifndef __IFaxInboundRoutingMethods_FWD_DEFINED__
#define __IFaxInboundRoutingMethods_FWD_DEFINED__
typedef interface IFaxInboundRoutingMethods IFaxInboundRoutingMethods;

#endif 	/* __IFaxInboundRoutingMethods_FWD_DEFINED__ */


#ifndef __IFaxInboundRoutingMethod_FWD_DEFINED__
#define __IFaxInboundRoutingMethod_FWD_DEFINED__
typedef interface IFaxInboundRoutingMethod IFaxInboundRoutingMethod;

#endif 	/* __IFaxInboundRoutingMethod_FWD_DEFINED__ */


#ifndef __IFaxDocument2_FWD_DEFINED__
#define __IFaxDocument2_FWD_DEFINED__
typedef interface IFaxDocument2 IFaxDocument2;

#endif 	/* __IFaxDocument2_FWD_DEFINED__ */


#ifndef __IFaxConfiguration_FWD_DEFINED__
#define __IFaxConfiguration_FWD_DEFINED__
typedef interface IFaxConfiguration IFaxConfiguration;

#endif 	/* __IFaxConfiguration_FWD_DEFINED__ */


#ifndef __IFaxServer2_FWD_DEFINED__
#define __IFaxServer2_FWD_DEFINED__
typedef interface IFaxServer2 IFaxServer2;

#endif 	/* __IFaxServer2_FWD_DEFINED__ */


#ifndef __IFaxAccountSet_FWD_DEFINED__
#define __IFaxAccountSet_FWD_DEFINED__
typedef interface IFaxAccountSet IFaxAccountSet;

#endif 	/* __IFaxAccountSet_FWD_DEFINED__ */


#ifndef __IFaxAccounts_FWD_DEFINED__
#define __IFaxAccounts_FWD_DEFINED__
typedef interface IFaxAccounts IFaxAccounts;

#endif 	/* __IFaxAccounts_FWD_DEFINED__ */


#ifndef __IFaxAccount_FWD_DEFINED__
#define __IFaxAccount_FWD_DEFINED__
typedef interface IFaxAccount IFaxAccount;

#endif 	/* __IFaxAccount_FWD_DEFINED__ */


#ifndef __IFaxOutgoingJob2_FWD_DEFINED__
#define __IFaxOutgoingJob2_FWD_DEFINED__
typedef interface IFaxOutgoingJob2 IFaxOutgoingJob2;

#endif 	/* __IFaxOutgoingJob2_FWD_DEFINED__ */


#ifndef __IFaxAccountFolders_FWD_DEFINED__
#define __IFaxAccountFolders_FWD_DEFINED__
typedef interface IFaxAccountFolders IFaxAccountFolders;

#endif 	/* __IFaxAccountFolders_FWD_DEFINED__ */


#ifndef __IFaxAccountIncomingQueue_FWD_DEFINED__
#define __IFaxAccountIncomingQueue_FWD_DEFINED__
typedef interface IFaxAccountIncomingQueue IFaxAccountIncomingQueue;

#endif 	/* __IFaxAccountIncomingQueue_FWD_DEFINED__ */


#ifndef __IFaxAccountOutgoingQueue_FWD_DEFINED__
#define __IFaxAccountOutgoingQueue_FWD_DEFINED__
typedef interface IFaxAccountOutgoingQueue IFaxAccountOutgoingQueue;

#endif 	/* __IFaxAccountOutgoingQueue_FWD_DEFINED__ */


#ifndef __IFaxOutgoingMessage2_FWD_DEFINED__
#define __IFaxOutgoingMessage2_FWD_DEFINED__
typedef interface IFaxOutgoingMessage2 IFaxOutgoingMessage2;

#endif 	/* __IFaxOutgoingMessage2_FWD_DEFINED__ */


#ifndef __IFaxAccountIncomingArchive_FWD_DEFINED__
#define __IFaxAccountIncomingArchive_FWD_DEFINED__
typedef interface IFaxAccountIncomingArchive IFaxAccountIncomingArchive;

#endif 	/* __IFaxAccountIncomingArchive_FWD_DEFINED__ */


#ifndef __IFaxAccountOutgoingArchive_FWD_DEFINED__
#define __IFaxAccountOutgoingArchive_FWD_DEFINED__
typedef interface IFaxAccountOutgoingArchive IFaxAccountOutgoingArchive;

#endif 	/* __IFaxAccountOutgoingArchive_FWD_DEFINED__ */


#ifndef __IFaxSecurity2_FWD_DEFINED__
#define __IFaxSecurity2_FWD_DEFINED__
typedef interface IFaxSecurity2 IFaxSecurity2;

#endif 	/* __IFaxSecurity2_FWD_DEFINED__ */


#ifndef __IFaxIncomingMessage2_FWD_DEFINED__
#define __IFaxIncomingMessage2_FWD_DEFINED__
typedef interface IFaxIncomingMessage2 IFaxIncomingMessage2;

#endif 	/* __IFaxIncomingMessage2_FWD_DEFINED__ */


#ifndef __IFaxServerNotify_FWD_DEFINED__
#define __IFaxServerNotify_FWD_DEFINED__
typedef interface IFaxServerNotify IFaxServerNotify;

#endif 	/* __IFaxServerNotify_FWD_DEFINED__ */


#ifndef ___IFaxServerNotify2_FWD_DEFINED__
#define ___IFaxServerNotify2_FWD_DEFINED__
typedef interface _IFaxServerNotify2 _IFaxServerNotify2;

#endif 	/* ___IFaxServerNotify2_FWD_DEFINED__ */


#ifndef __IFaxServerNotify2_FWD_DEFINED__
#define __IFaxServerNotify2_FWD_DEFINED__
typedef interface IFaxServerNotify2 IFaxServerNotify2;

#endif 	/* __IFaxServerNotify2_FWD_DEFINED__ */


#ifndef ___IFaxAccountNotify_FWD_DEFINED__
#define ___IFaxAccountNotify_FWD_DEFINED__
typedef interface _IFaxAccountNotify _IFaxAccountNotify;

#endif 	/* ___IFaxAccountNotify_FWD_DEFINED__ */


#ifndef __IFaxAccountNotify_FWD_DEFINED__
#define __IFaxAccountNotify_FWD_DEFINED__
typedef interface IFaxAccountNotify IFaxAccountNotify;

#endif 	/* __IFaxAccountNotify_FWD_DEFINED__ */


#ifndef __FaxServer_FWD_DEFINED__
#define __FaxServer_FWD_DEFINED__

#ifdef __cplusplus
typedef class FaxServer FaxServer;
#else
typedef struct FaxServer FaxServer;
#endif /* __cplusplus */

#endif 	/* __FaxServer_FWD_DEFINED__ */


#ifndef __FaxDeviceProviders_FWD_DEFINED__
#define __FaxDeviceProviders_FWD_DEFINED__

#ifdef __cplusplus
typedef class FaxDeviceProviders FaxDeviceProviders;
#else
typedef struct FaxDeviceProviders FaxDeviceProviders;
#endif /* __cplusplus */

#endif 	/* __FaxDeviceProviders_FWD_DEFINED__ */


#ifndef __FaxDevices_FWD_DEFINED__
#define __FaxDevices_FWD_DEFINED__

#ifdef __cplusplus
typedef class FaxDevices FaxDevices;
#else
typedef struct FaxDevices FaxDevices;
#endif /* __cplusplus */

#endif 	/* __FaxDevices_FWD_DEFINED__ */


#ifndef __FaxInboundRouting_FWD_DEFINED__
#define __FaxInboundRouting_FWD_DEFINED__

#ifdef __cplusplus
typedef class FaxInboundRouting FaxInboundRouting;
#else
typedef struct FaxInboundRouting FaxInboundRouting;
#endif /* __cplusplus */

#endif 	/* __FaxInboundRouting_FWD_DEFINED__ */


#ifndef __FaxFolders_FWD_DEFINED__
#define __FaxFolders_FWD_DEFINED__

#ifdef __cplusplus
typedef class FaxFolders FaxFolders;
#else
typedef struct FaxFolders FaxFolders;
#endif /* __cplusplus */

#endif 	/* __FaxFolders_FWD_DEFINED__ */


#ifndef __FaxLoggingOptions_FWD_DEFINED__
#define __FaxLoggingOptions_FWD_DEFINED__

#ifdef __cplusplus
typedef class FaxLoggingOptions FaxLoggingOptions;
#else
typedef struct FaxLoggingOptions FaxLoggingOptions;
#endif /* __cplusplus */

#endif 	/* __FaxLoggingOptions_FWD_DEFINED__ */


#ifndef __FaxActivity_FWD_DEFINED__
#define __FaxActivity_FWD_DEFINED__

#ifdef __cplusplus
typedef class FaxActivity FaxActivity;
#else
typedef struct FaxActivity FaxActivity;
#endif /* __cplusplus */

#endif 	/* __FaxActivity_FWD_DEFINED__ */


#ifndef __FaxOutboundRouting_FWD_DEFINED__
#define __FaxOutboundRouting_FWD_DEFINED__

#ifdef __cplusplus
typedef class FaxOutboundRouting FaxOutboundRouting;
#else
typedef struct FaxOutboundRouting FaxOutboundRouting;
#endif /* __cplusplus */

#endif 	/* __FaxOutboundRouting_FWD_DEFINED__ */


#ifndef __FaxReceiptOptions_FWD_DEFINED__
#define __FaxReceiptOptions_FWD_DEFINED__

#ifdef __cplusplus
typedef class FaxReceiptOptions FaxReceiptOptions;
#else
typedef struct FaxReceiptOptions FaxReceiptOptions;
#endif /* __cplusplus */

#endif 	/* __FaxReceiptOptions_FWD_DEFINED__ */


#ifndef __FaxSecurity_FWD_DEFINED__
#define __FaxSecurity_FWD_DEFINED__

#ifdef __cplusplus
typedef class FaxSecurity FaxSecurity;
#else
typedef struct FaxSecurity FaxSecurity;
#endif /* __cplusplus */

#endif 	/* __FaxSecurity_FWD_DEFINED__ */


#ifndef __FaxDocument_FWD_DEFINED__
#define __FaxDocument_FWD_DEFINED__

#ifdef __cplusplus
typedef class FaxDocument FaxDocument;
#else
typedef struct FaxDocument FaxDocument;
#endif /* __cplusplus */

#endif 	/* __FaxDocument_FWD_DEFINED__ */


#ifndef __FaxSender_FWD_DEFINED__
#define __FaxSender_FWD_DEFINED__

#ifdef __cplusplus
typedef class FaxSender FaxSender;
#else
typedef struct FaxSender FaxSender;
#endif /* __cplusplus */

#endif 	/* __FaxSender_FWD_DEFINED__ */


#ifndef __FaxRecipients_FWD_DEFINED__
#define __FaxRecipients_FWD_DEFINED__

#ifdef __cplusplus
typedef class FaxRecipients FaxRecipients;
#else
typedef struct FaxRecipients FaxRecipients;
#endif /* __cplusplus */

#endif 	/* __FaxRecipients_FWD_DEFINED__ */


#ifndef __FaxIncomingArchive_FWD_DEFINED__
#define __FaxIncomingArchive_FWD_DEFINED__

#ifdef __cplusplus
typedef class FaxIncomingArchive FaxIncomingArchive;
#else
typedef struct FaxIncomingArchive FaxIncomingArchive;
#endif /* __cplusplus */

#endif 	/* __FaxIncomingArchive_FWD_DEFINED__ */


#ifndef __FaxIncomingQueue_FWD_DEFINED__
#define __FaxIncomingQueue_FWD_DEFINED__

#ifdef __cplusplus
typedef class FaxIncomingQueue FaxIncomingQueue;
#else
typedef struct FaxIncomingQueue FaxIncomingQueue;
#endif /* __cplusplus */

#endif 	/* __FaxIncomingQueue_FWD_DEFINED__ */


#ifndef __FaxOutgoingArchive_FWD_DEFINED__
#define __FaxOutgoingArchive_FWD_DEFINED__

#ifdef __cplusplus
typedef class FaxOutgoingArchive FaxOutgoingArchive;
#else
typedef struct FaxOutgoingArchive FaxOutgoingArchive;
#endif /* __cplusplus */

#endif 	/* __FaxOutgoingArchive_FWD_DEFINED__ */


#ifndef __FaxOutgoingQueue_FWD_DEFINED__
#define __FaxOutgoingQueue_FWD_DEFINED__

#ifdef __cplusplus
typedef class FaxOutgoingQueue FaxOutgoingQueue;
#else
typedef struct FaxOutgoingQueue FaxOutgoingQueue;
#endif /* __cplusplus */

#endif 	/* __FaxOutgoingQueue_FWD_DEFINED__ */


#ifndef __FaxIncomingMessageIterator_FWD_DEFINED__
#define __FaxIncomingMessageIterator_FWD_DEFINED__

#ifdef __cplusplus
typedef class FaxIncomingMessageIterator FaxIncomingMessageIterator;
#else
typedef struct FaxIncomingMessageIterator FaxIncomingMessageIterator;
#endif /* __cplusplus */

#endif 	/* __FaxIncomingMessageIterator_FWD_DEFINED__ */


#ifndef __FaxIncomingMessage_FWD_DEFINED__
#define __FaxIncomingMessage_FWD_DEFINED__

#ifdef __cplusplus
typedef class FaxIncomingMessage FaxIncomingMessage;
#else
typedef struct FaxIncomingMessage FaxIncomingMessage;
#endif /* __cplusplus */

#endif 	/* __FaxIncomingMessage_FWD_DEFINED__ */


#ifndef __FaxOutgoingJobs_FWD_DEFINED__
#define __FaxOutgoingJobs_FWD_DEFINED__

#ifdef __cplusplus
typedef class FaxOutgoingJobs FaxOutgoingJobs;
#else
typedef struct FaxOutgoingJobs FaxOutgoingJobs;
#endif /* __cplusplus */

#endif 	/* __FaxOutgoingJobs_FWD_DEFINED__ */


#ifndef __FaxOutgoingJob_FWD_DEFINED__
#define __FaxOutgoingJob_FWD_DEFINED__

#ifdef __cplusplus
typedef class FaxOutgoingJob FaxOutgoingJob;
#else
typedef struct FaxOutgoingJob FaxOutgoingJob;
#endif /* __cplusplus */

#endif 	/* __FaxOutgoingJob_FWD_DEFINED__ */


#ifndef __FaxOutgoingMessageIterator_FWD_DEFINED__
#define __FaxOutgoingMessageIterator_FWD_DEFINED__

#ifdef __cplusplus
typedef class FaxOutgoingMessageIterator FaxOutgoingMessageIterator;
#else
typedef struct FaxOutgoingMessageIterator FaxOutgoingMessageIterator;
#endif /* __cplusplus */

#endif 	/* __FaxOutgoingMessageIterator_FWD_DEFINED__ */


#ifndef __FaxOutgoingMessage_FWD_DEFINED__
#define __FaxOutgoingMessage_FWD_DEFINED__

#ifdef __cplusplus
typedef class FaxOutgoingMessage FaxOutgoingMessage;
#else
typedef struct FaxOutgoingMessage FaxOutgoingMessage;
#endif /* __cplusplus */

#endif 	/* __FaxOutgoingMessage_FWD_DEFINED__ */


#ifndef __FaxIncomingJobs_FWD_DEFINED__
#define __FaxIncomingJobs_FWD_DEFINED__

#ifdef __cplusplus
typedef class FaxIncomingJobs FaxIncomingJobs;
#else
typedef struct FaxIncomingJobs FaxIncomingJobs;
#endif /* __cplusplus */

#endif 	/* __FaxIncomingJobs_FWD_DEFINED__ */


#ifndef __FaxIncomingJob_FWD_DEFINED__
#define __FaxIncomingJob_FWD_DEFINED__

#ifdef __cplusplus
typedef class FaxIncomingJob FaxIncomingJob;
#else
typedef struct FaxIncomingJob FaxIncomingJob;
#endif /* __cplusplus */

#endif 	/* __FaxIncomingJob_FWD_DEFINED__ */


#ifndef __FaxDeviceProvider_FWD_DEFINED__
#define __FaxDeviceProvider_FWD_DEFINED__

#ifdef __cplusplus
typedef class FaxDeviceProvider FaxDeviceProvider;
#else
typedef struct FaxDeviceProvider FaxDeviceProvider;
#endif /* __cplusplus */

#endif 	/* __FaxDeviceProvider_FWD_DEFINED__ */


#ifndef __FaxDevice_FWD_DEFINED__
#define __FaxDevice_FWD_DEFINED__

#ifdef __cplusplus
typedef class FaxDevice FaxDevice;
#else
typedef struct FaxDevice FaxDevice;
#endif /* __cplusplus */

#endif 	/* __FaxDevice_FWD_DEFINED__ */


#ifndef __FaxActivityLogging_FWD_DEFINED__
#define __FaxActivityLogging_FWD_DEFINED__

#ifdef __cplusplus
typedef class FaxActivityLogging FaxActivityLogging;
#else
typedef struct FaxActivityLogging FaxActivityLogging;
#endif /* __cplusplus */

#endif 	/* __FaxActivityLogging_FWD_DEFINED__ */


#ifndef __FaxEventLogging_FWD_DEFINED__
#define __FaxEventLogging_FWD_DEFINED__

#ifdef __cplusplus
typedef class FaxEventLogging FaxEventLogging;
#else
typedef struct FaxEventLogging FaxEventLogging;
#endif /* __cplusplus */

#endif 	/* __FaxEventLogging_FWD_DEFINED__ */


#ifndef __FaxOutboundRoutingGroups_FWD_DEFINED__
#define __FaxOutboundRoutingGroups_FWD_DEFINED__

#ifdef __cplusplus
typedef class FaxOutboundRoutingGroups FaxOutboundRoutingGroups;
#else
typedef struct FaxOutboundRoutingGroups FaxOutboundRoutingGroups;
#endif /* __cplusplus */

#endif 	/* __FaxOutboundRoutingGroups_FWD_DEFINED__ */


#ifndef __FaxOutboundRoutingGroup_FWD_DEFINED__
#define __FaxOutboundRoutingGroup_FWD_DEFINED__

#ifdef __cplusplus
typedef class FaxOutboundRoutingGroup FaxOutboundRoutingGroup;
#else
typedef struct FaxOutboundRoutingGroup FaxOutboundRoutingGroup;
#endif /* __cplusplus */

#endif 	/* __FaxOutboundRoutingGroup_FWD_DEFINED__ */


#ifndef __FaxDeviceIds_FWD_DEFINED__
#define __FaxDeviceIds_FWD_DEFINED__

#ifdef __cplusplus
typedef class FaxDeviceIds FaxDeviceIds;
#else
typedef struct FaxDeviceIds FaxDeviceIds;
#endif /* __cplusplus */

#endif 	/* __FaxDeviceIds_FWD_DEFINED__ */


#ifndef __FaxOutboundRoutingRules_FWD_DEFINED__
#define __FaxOutboundRoutingRules_FWD_DEFINED__

#ifdef __cplusplus
typedef class FaxOutboundRoutingRules FaxOutboundRoutingRules;
#else
typedef struct FaxOutboundRoutingRules FaxOutboundRoutingRules;
#endif /* __cplusplus */

#endif 	/* __FaxOutboundRoutingRules_FWD_DEFINED__ */


#ifndef __FaxOutboundRoutingRule_FWD_DEFINED__
#define __FaxOutboundRoutingRule_FWD_DEFINED__

#ifdef __cplusplus
typedef class FaxOutboundRoutingRule FaxOutboundRoutingRule;
#else
typedef struct FaxOutboundRoutingRule FaxOutboundRoutingRule;
#endif /* __cplusplus */

#endif 	/* __FaxOutboundRoutingRule_FWD_DEFINED__ */


#ifndef __FaxInboundRoutingExtensions_FWD_DEFINED__
#define __FaxInboundRoutingExtensions_FWD_DEFINED__

#ifdef __cplusplus
typedef class FaxInboundRoutingExtensions FaxInboundRoutingExtensions;
#else
typedef struct FaxInboundRoutingExtensions FaxInboundRoutingExtensions;
#endif /* __cplusplus */

#endif 	/* __FaxInboundRoutingExtensions_FWD_DEFINED__ */


#ifndef __FaxInboundRoutingExtension_FWD_DEFINED__
#define __FaxInboundRoutingExtension_FWD_DEFINED__

#ifdef __cplusplus
typedef class FaxInboundRoutingExtension FaxInboundRoutingExtension;
#else
typedef struct FaxInboundRoutingExtension FaxInboundRoutingExtension;
#endif /* __cplusplus */

#endif 	/* __FaxInboundRoutingExtension_FWD_DEFINED__ */


#ifndef __FaxInboundRoutingMethods_FWD_DEFINED__
#define __FaxInboundRoutingMethods_FWD_DEFINED__

#ifdef __cplusplus
typedef class FaxInboundRoutingMethods FaxInboundRoutingMethods;
#else
typedef struct FaxInboundRoutingMethods FaxInboundRoutingMethods;
#endif /* __cplusplus */

#endif 	/* __FaxInboundRoutingMethods_FWD_DEFINED__ */


#ifndef __FaxInboundRoutingMethod_FWD_DEFINED__
#define __FaxInboundRoutingMethod_FWD_DEFINED__

#ifdef __cplusplus
typedef class FaxInboundRoutingMethod FaxInboundRoutingMethod;
#else
typedef struct FaxInboundRoutingMethod FaxInboundRoutingMethod;
#endif /* __cplusplus */

#endif 	/* __FaxInboundRoutingMethod_FWD_DEFINED__ */


#ifndef __FaxJobStatus_FWD_DEFINED__
#define __FaxJobStatus_FWD_DEFINED__

#ifdef __cplusplus
typedef class FaxJobStatus FaxJobStatus;
#else
typedef struct FaxJobStatus FaxJobStatus;
#endif /* __cplusplus */

#endif 	/* __FaxJobStatus_FWD_DEFINED__ */


#ifndef __FaxRecipient_FWD_DEFINED__
#define __FaxRecipient_FWD_DEFINED__

#ifdef __cplusplus
typedef class FaxRecipient FaxRecipient;
#else
typedef struct FaxRecipient FaxRecipient;
#endif /* __cplusplus */

#endif 	/* __FaxRecipient_FWD_DEFINED__ */


#ifndef __FaxConfiguration_FWD_DEFINED__
#define __FaxConfiguration_FWD_DEFINED__

#ifdef __cplusplus
typedef class FaxConfiguration FaxConfiguration;
#else
typedef struct FaxConfiguration FaxConfiguration;
#endif /* __cplusplus */

#endif 	/* __FaxConfiguration_FWD_DEFINED__ */


#ifndef __FaxAccountSet_FWD_DEFINED__
#define __FaxAccountSet_FWD_DEFINED__

#ifdef __cplusplus
typedef class FaxAccountSet FaxAccountSet;
#else
typedef struct FaxAccountSet FaxAccountSet;
#endif /* __cplusplus */

#endif 	/* __FaxAccountSet_FWD_DEFINED__ */


#ifndef __FaxAccounts_FWD_DEFINED__
#define __FaxAccounts_FWD_DEFINED__

#ifdef __cplusplus
typedef class FaxAccounts FaxAccounts;
#else
typedef struct FaxAccounts FaxAccounts;
#endif /* __cplusplus */

#endif 	/* __FaxAccounts_FWD_DEFINED__ */


#ifndef __FaxAccount_FWD_DEFINED__
#define __FaxAccount_FWD_DEFINED__

#ifdef __cplusplus
typedef class FaxAccount FaxAccount;
#else
typedef struct FaxAccount FaxAccount;
#endif /* __cplusplus */

#endif 	/* __FaxAccount_FWD_DEFINED__ */


#ifndef __FaxAccountFolders_FWD_DEFINED__
#define __FaxAccountFolders_FWD_DEFINED__

#ifdef __cplusplus
typedef class FaxAccountFolders FaxAccountFolders;
#else
typedef struct FaxAccountFolders FaxAccountFolders;
#endif /* __cplusplus */

#endif 	/* __FaxAccountFolders_FWD_DEFINED__ */


#ifndef __FaxAccountIncomingQueue_FWD_DEFINED__
#define __FaxAccountIncomingQueue_FWD_DEFINED__

#ifdef __cplusplus
typedef class FaxAccountIncomingQueue FaxAccountIncomingQueue;
#else
typedef struct FaxAccountIncomingQueue FaxAccountIncomingQueue;
#endif /* __cplusplus */

#endif 	/* __FaxAccountIncomingQueue_FWD_DEFINED__ */


#ifndef __FaxAccountOutgoingQueue_FWD_DEFINED__
#define __FaxAccountOutgoingQueue_FWD_DEFINED__

#ifdef __cplusplus
typedef class FaxAccountOutgoingQueue FaxAccountOutgoingQueue;
#else
typedef struct FaxAccountOutgoingQueue FaxAccountOutgoingQueue;
#endif /* __cplusplus */

#endif 	/* __FaxAccountOutgoingQueue_FWD_DEFINED__ */


#ifndef __FaxAccountIncomingArchive_FWD_DEFINED__
#define __FaxAccountIncomingArchive_FWD_DEFINED__

#ifdef __cplusplus
typedef class FaxAccountIncomingArchive FaxAccountIncomingArchive;
#else
typedef struct FaxAccountIncomingArchive FaxAccountIncomingArchive;
#endif /* __cplusplus */

#endif 	/* __FaxAccountIncomingArchive_FWD_DEFINED__ */


#ifndef __FaxAccountOutgoingArchive_FWD_DEFINED__
#define __FaxAccountOutgoingArchive_FWD_DEFINED__

#ifdef __cplusplus
typedef class FaxAccountOutgoingArchive FaxAccountOutgoingArchive;
#else
typedef struct FaxAccountOutgoingArchive FaxAccountOutgoingArchive;
#endif /* __cplusplus */

#endif 	/* __FaxAccountOutgoingArchive_FWD_DEFINED__ */


#ifndef __FaxSecurity2_FWD_DEFINED__
#define __FaxSecurity2_FWD_DEFINED__

#ifdef __cplusplus
typedef class FaxSecurity2 FaxSecurity2;
#else
typedef struct FaxSecurity2 FaxSecurity2;
#endif /* __cplusplus */

#endif 	/* __FaxSecurity2_FWD_DEFINED__ */


/* header files for imported files */
#include "oaidl.h"
#include "ocidl.h"

#ifdef __cplusplus
extern "C"{
#endif 


/* interface __MIDL_itf_faxcomex_0000_0000 */
/* [local] */ 

#include <winapifamily.h>
#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)



















































#define	prv_DEFAULT_PREFETCH_SIZE	( 100 )



extern RPC_IF_HANDLE __MIDL_itf_faxcomex_0000_0000_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_faxcomex_0000_0000_v0_0_s_ifspec;

#ifndef __IFaxJobStatus_INTERFACE_DEFINED__
#define __IFaxJobStatus_INTERFACE_DEFINED__

/* interface IFaxJobStatus */
/* [unique][helpstring][dual][uuid][object] */ 

typedef 
enum FAX_JOB_STATUS_ENUM
    {
        fjsPENDING	= 0x1,
        fjsINPROGRESS	= 0x2,
        fjsFAILED	= 0x8,
        fjsPAUSED	= 0x10,
        fjsNOLINE	= 0x20,
        fjsRETRYING	= 0x40,
        fjsRETRIES_EXCEEDED	= 0x80,
        fjsCOMPLETED	= 0x100,
        fjsCANCELED	= 0x200,
        fjsCANCELING	= 0x400,
        fjsROUTING	= 0x800
    } 	FAX_JOB_STATUS_ENUM;

typedef 
enum FAX_JOB_EXTENDED_STATUS_ENUM
    {
        fjesNONE	= 0,
        fjesDISCONNECTED	= ( fjesNONE + 1 ) ,
        fjesINITIALIZING	= ( fjesDISCONNECTED + 1 ) ,
        fjesDIALING	= ( fjesINITIALIZING + 1 ) ,
        fjesTRANSMITTING	= ( fjesDIALING + 1 ) ,
        fjesANSWERED	= ( fjesTRANSMITTING + 1 ) ,
        fjesRECEIVING	= ( fjesANSWERED + 1 ) ,
        fjesLINE_UNAVAILABLE	= ( fjesRECEIVING + 1 ) ,
        fjesBUSY	= ( fjesLINE_UNAVAILABLE + 1 ) ,
        fjesNO_ANSWER	= ( fjesBUSY + 1 ) ,
        fjesBAD_ADDRESS	= ( fjesNO_ANSWER + 1 ) ,
        fjesNO_DIAL_TONE	= ( fjesBAD_ADDRESS + 1 ) ,
        fjesFATAL_ERROR	= ( fjesNO_DIAL_TONE + 1 ) ,
        fjesCALL_DELAYED	= ( fjesFATAL_ERROR + 1 ) ,
        fjesCALL_BLACKLISTED	= ( fjesCALL_DELAYED + 1 ) ,
        fjesNOT_FAX_CALL	= ( fjesCALL_BLACKLISTED + 1 ) ,
        fjesPARTIALLY_RECEIVED	= ( fjesNOT_FAX_CALL + 1 ) ,
        fjesHANDLED	= ( fjesPARTIALLY_RECEIVED + 1 ) ,
        fjesCALL_COMPLETED	= ( fjesHANDLED + 1 ) ,
        fjesCALL_ABORTED	= ( fjesCALL_COMPLETED + 1 ) ,
        fjesPROPRIETARY	= 0x1000000
    } 	FAX_JOB_EXTENDED_STATUS_ENUM;

typedef 
enum FAX_JOB_OPERATIONS_ENUM
    {
        fjoVIEW	= 0x1,
        fjoPAUSE	= 0x2,
        fjoRESUME	= 0x4,
        fjoRESTART	= 0x8,
        fjoDELETE	= 0x10,
        fjoRECIPIENT_INFO	= 0x20,
        fjoSENDER_INFO	= 0x40
    } 	FAX_JOB_OPERATIONS_ENUM;

typedef 
enum FAX_JOB_TYPE_ENUM
    {
        fjtSEND	= 0,
        fjtRECEIVE	= ( fjtSEND + 1 ) ,
        fjtROUTING	= ( fjtRECEIVE + 1 ) 
    } 	FAX_JOB_TYPE_ENUM;


EXTERN_C const IID IID_IFaxJobStatus;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("8B86F485-FD7F-4824-886B-40C5CAA617CC")
    IFaxJobStatus : public IDispatch
    {
    public:
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Status( 
            /* [retval][out] */ __RPC__out FAX_JOB_STATUS_ENUM *pStatus) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Pages( 
            /* [retval][out] */ __RPC__out long *plPages) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Size( 
            /* [retval][out] */ __RPC__out long *plSize) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_CurrentPage( 
            /* [retval][out] */ __RPC__out long *plCurrentPage) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_DeviceId( 
            /* [retval][out] */ __RPC__out long *plDeviceId) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_CSID( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrCSID) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_TSID( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrTSID) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_ExtendedStatusCode( 
            /* [retval][out] */ __RPC__out FAX_JOB_EXTENDED_STATUS_ENUM *pExtendedStatusCode) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_ExtendedStatus( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrExtendedStatus) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_AvailableOperations( 
            /* [retval][out] */ __RPC__out FAX_JOB_OPERATIONS_ENUM *pAvailableOperations) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Retries( 
            /* [retval][out] */ __RPC__out long *plRetries) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_JobType( 
            /* [retval][out] */ __RPC__out FAX_JOB_TYPE_ENUM *pJobType) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_ScheduledTime( 
            /* [retval][out] */ __RPC__out DATE *pdateScheduledTime) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_TransmissionStart( 
            /* [retval][out] */ __RPC__out DATE *pdateTransmissionStart) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_TransmissionEnd( 
            /* [retval][out] */ __RPC__out DATE *pdateTransmissionEnd) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_CallerId( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrCallerId) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_RoutingInformation( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrRoutingInformation) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IFaxJobStatusVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IFaxJobStatus * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IFaxJobStatus * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IFaxJobStatus * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IFaxJobStatus * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IFaxJobStatus * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IFaxJobStatus * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IFaxJobStatus * This,
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
        
        DECLSPEC_XFGVIRT(IFaxJobStatus, get_Status)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Status )( 
            __RPC__in IFaxJobStatus * This,
            /* [retval][out] */ __RPC__out FAX_JOB_STATUS_ENUM *pStatus);
        
        DECLSPEC_XFGVIRT(IFaxJobStatus, get_Pages)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Pages )( 
            __RPC__in IFaxJobStatus * This,
            /* [retval][out] */ __RPC__out long *plPages);
        
        DECLSPEC_XFGVIRT(IFaxJobStatus, get_Size)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Size )( 
            __RPC__in IFaxJobStatus * This,
            /* [retval][out] */ __RPC__out long *plSize);
        
        DECLSPEC_XFGVIRT(IFaxJobStatus, get_CurrentPage)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_CurrentPage )( 
            __RPC__in IFaxJobStatus * This,
            /* [retval][out] */ __RPC__out long *plCurrentPage);
        
        DECLSPEC_XFGVIRT(IFaxJobStatus, get_DeviceId)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_DeviceId )( 
            __RPC__in IFaxJobStatus * This,
            /* [retval][out] */ __RPC__out long *plDeviceId);
        
        DECLSPEC_XFGVIRT(IFaxJobStatus, get_CSID)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_CSID )( 
            __RPC__in IFaxJobStatus * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrCSID);
        
        DECLSPEC_XFGVIRT(IFaxJobStatus, get_TSID)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_TSID )( 
            __RPC__in IFaxJobStatus * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrTSID);
        
        DECLSPEC_XFGVIRT(IFaxJobStatus, get_ExtendedStatusCode)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_ExtendedStatusCode )( 
            __RPC__in IFaxJobStatus * This,
            /* [retval][out] */ __RPC__out FAX_JOB_EXTENDED_STATUS_ENUM *pExtendedStatusCode);
        
        DECLSPEC_XFGVIRT(IFaxJobStatus, get_ExtendedStatus)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_ExtendedStatus )( 
            __RPC__in IFaxJobStatus * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrExtendedStatus);
        
        DECLSPEC_XFGVIRT(IFaxJobStatus, get_AvailableOperations)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_AvailableOperations )( 
            __RPC__in IFaxJobStatus * This,
            /* [retval][out] */ __RPC__out FAX_JOB_OPERATIONS_ENUM *pAvailableOperations);
        
        DECLSPEC_XFGVIRT(IFaxJobStatus, get_Retries)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Retries )( 
            __RPC__in IFaxJobStatus * This,
            /* [retval][out] */ __RPC__out long *plRetries);
        
        DECLSPEC_XFGVIRT(IFaxJobStatus, get_JobType)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_JobType )( 
            __RPC__in IFaxJobStatus * This,
            /* [retval][out] */ __RPC__out FAX_JOB_TYPE_ENUM *pJobType);
        
        DECLSPEC_XFGVIRT(IFaxJobStatus, get_ScheduledTime)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_ScheduledTime )( 
            __RPC__in IFaxJobStatus * This,
            /* [retval][out] */ __RPC__out DATE *pdateScheduledTime);
        
        DECLSPEC_XFGVIRT(IFaxJobStatus, get_TransmissionStart)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_TransmissionStart )( 
            __RPC__in IFaxJobStatus * This,
            /* [retval][out] */ __RPC__out DATE *pdateTransmissionStart);
        
        DECLSPEC_XFGVIRT(IFaxJobStatus, get_TransmissionEnd)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_TransmissionEnd )( 
            __RPC__in IFaxJobStatus * This,
            /* [retval][out] */ __RPC__out DATE *pdateTransmissionEnd);
        
        DECLSPEC_XFGVIRT(IFaxJobStatus, get_CallerId)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_CallerId )( 
            __RPC__in IFaxJobStatus * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrCallerId);
        
        DECLSPEC_XFGVIRT(IFaxJobStatus, get_RoutingInformation)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_RoutingInformation )( 
            __RPC__in IFaxJobStatus * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrRoutingInformation);
        
        END_INTERFACE
    } IFaxJobStatusVtbl;

    interface IFaxJobStatus
    {
        CONST_VTBL struct IFaxJobStatusVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IFaxJobStatus_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IFaxJobStatus_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IFaxJobStatus_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IFaxJobStatus_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IFaxJobStatus_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IFaxJobStatus_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IFaxJobStatus_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IFaxJobStatus_get_Status(This,pStatus)	\
    ( (This)->lpVtbl -> get_Status(This,pStatus) ) 

#define IFaxJobStatus_get_Pages(This,plPages)	\
    ( (This)->lpVtbl -> get_Pages(This,plPages) ) 

#define IFaxJobStatus_get_Size(This,plSize)	\
    ( (This)->lpVtbl -> get_Size(This,plSize) ) 

#define IFaxJobStatus_get_CurrentPage(This,plCurrentPage)	\
    ( (This)->lpVtbl -> get_CurrentPage(This,plCurrentPage) ) 

#define IFaxJobStatus_get_DeviceId(This,plDeviceId)	\
    ( (This)->lpVtbl -> get_DeviceId(This,plDeviceId) ) 

#define IFaxJobStatus_get_CSID(This,pbstrCSID)	\
    ( (This)->lpVtbl -> get_CSID(This,pbstrCSID) ) 

#define IFaxJobStatus_get_TSID(This,pbstrTSID)	\
    ( (This)->lpVtbl -> get_TSID(This,pbstrTSID) ) 

#define IFaxJobStatus_get_ExtendedStatusCode(This,pExtendedStatusCode)	\
    ( (This)->lpVtbl -> get_ExtendedStatusCode(This,pExtendedStatusCode) ) 

#define IFaxJobStatus_get_ExtendedStatus(This,pbstrExtendedStatus)	\
    ( (This)->lpVtbl -> get_ExtendedStatus(This,pbstrExtendedStatus) ) 

#define IFaxJobStatus_get_AvailableOperations(This,pAvailableOperations)	\
    ( (This)->lpVtbl -> get_AvailableOperations(This,pAvailableOperations) ) 

#define IFaxJobStatus_get_Retries(This,plRetries)	\
    ( (This)->lpVtbl -> get_Retries(This,plRetries) ) 

#define IFaxJobStatus_get_JobType(This,pJobType)	\
    ( (This)->lpVtbl -> get_JobType(This,pJobType) ) 

#define IFaxJobStatus_get_ScheduledTime(This,pdateScheduledTime)	\
    ( (This)->lpVtbl -> get_ScheduledTime(This,pdateScheduledTime) ) 

#define IFaxJobStatus_get_TransmissionStart(This,pdateTransmissionStart)	\
    ( (This)->lpVtbl -> get_TransmissionStart(This,pdateTransmissionStart) ) 

#define IFaxJobStatus_get_TransmissionEnd(This,pdateTransmissionEnd)	\
    ( (This)->lpVtbl -> get_TransmissionEnd(This,pdateTransmissionEnd) ) 

#define IFaxJobStatus_get_CallerId(This,pbstrCallerId)	\
    ( (This)->lpVtbl -> get_CallerId(This,pbstrCallerId) ) 

#define IFaxJobStatus_get_RoutingInformation(This,pbstrRoutingInformation)	\
    ( (This)->lpVtbl -> get_RoutingInformation(This,pbstrRoutingInformation) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IFaxJobStatus_INTERFACE_DEFINED__ */


#ifndef __IFaxServer_INTERFACE_DEFINED__
#define __IFaxServer_INTERFACE_DEFINED__

/* interface IFaxServer */
/* [nonextensible][unique][helpstring][dual][uuid][object] */ 

typedef 
enum FAX_SERVER_EVENTS_TYPE_ENUM
    {
        fsetNONE	= 0,
        fsetIN_QUEUE	= 0x1,
        fsetOUT_QUEUE	= 0x2,
        fsetCONFIG	= 0x4,
        fsetACTIVITY	= 0x8,
        fsetQUEUE_STATE	= 0x10,
        fsetIN_ARCHIVE	= 0x20,
        fsetOUT_ARCHIVE	= 0x40,
        fsetFXSSVC_ENDED	= 0x80,
        fsetDEVICE_STATUS	= 0x100,
        fsetINCOMING_CALL	= 0x200
    } 	FAX_SERVER_EVENTS_TYPE_ENUM;

typedef 
enum FAX_SERVER_APIVERSION_ENUM
    {
        fsAPI_VERSION_0	= 0,
        fsAPI_VERSION_1	= 0x10000,
        fsAPI_VERSION_2	= 0x20000,
        fsAPI_VERSION_3	= 0x30000
    } 	FAX_SERVER_APIVERSION_ENUM;


EXTERN_C const IID IID_IFaxServer;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("475B6469-90A5-4878-A577-17A86E8E3462")
    IFaxServer : public IDispatch
    {
    public:
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE Connect( 
            /* [in] */ __RPC__in BSTR bstrServerName) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_ServerName( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrServerName) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE GetDeviceProviders( 
            /* [retval][out] */ __RPC__deref_out_opt IFaxDeviceProviders **ppFaxDeviceProviders) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE GetDevices( 
            /* [retval][out] */ __RPC__deref_out_opt IFaxDevices **ppFaxDevices) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_InboundRouting( 
            /* [retval][out] */ __RPC__deref_out_opt IFaxInboundRouting **ppFaxInboundRouting) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Folders( 
            /* [retval][out] */ __RPC__deref_out_opt IFaxFolders **pFaxFolders) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_LoggingOptions( 
            /* [retval][out] */ __RPC__deref_out_opt IFaxLoggingOptions **ppFaxLoggingOptions) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_MajorVersion( 
            /* [retval][out] */ __RPC__out long *plMajorVersion) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_MinorVersion( 
            /* [retval][out] */ __RPC__out long *plMinorVersion) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_MajorBuild( 
            /* [retval][out] */ __RPC__out long *plMajorBuild) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_MinorBuild( 
            /* [retval][out] */ __RPC__out long *plMinorBuild) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Debug( 
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pbDebug) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Activity( 
            /* [retval][out] */ __RPC__deref_out_opt IFaxActivity **ppFaxActivity) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_OutboundRouting( 
            /* [retval][out] */ __RPC__deref_out_opt IFaxOutboundRouting **ppFaxOutboundRouting) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_ReceiptOptions( 
            /* [retval][out] */ __RPC__deref_out_opt IFaxReceiptOptions **ppFaxReceiptOptions) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Security( 
            /* [retval][out] */ __RPC__deref_out_opt IFaxSecurity **ppFaxSecurity) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE Disconnect( void) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE GetExtensionProperty( 
            /* [in] */ __RPC__in BSTR bstrGUID,
            /* [retval][out] */ __RPC__out VARIANT *pvProperty) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE SetExtensionProperty( 
            /* [in] */ __RPC__in BSTR bstrGUID,
            /* [in] */ VARIANT vProperty) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE ListenToServerEvents( 
            /* [in] */ FAX_SERVER_EVENTS_TYPE_ENUM EventTypes) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE RegisterDeviceProvider( 
            /* [in] */ __RPC__in BSTR bstrGUID,
            /* [in] */ __RPC__in BSTR bstrFriendlyName,
            /* [in] */ __RPC__in BSTR bstrImageName,
            /* [in] */ __RPC__in BSTR TspName,
            /* [in] */ long lFSPIVersion) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE UnregisterDeviceProvider( 
            /* [in] */ __RPC__in BSTR bstrUniqueName) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE RegisterInboundRoutingExtension( 
            /* [in] */ __RPC__in BSTR bstrExtensionName,
            /* [in] */ __RPC__in BSTR bstrFriendlyName,
            /* [in] */ __RPC__in BSTR bstrImageName,
            /* [in] */ VARIANT vMethods) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE UnregisterInboundRoutingExtension( 
            /* [in] */ __RPC__in BSTR bstrExtensionUniqueName) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_RegisteredEvents( 
            /* [retval][out] */ __RPC__out FAX_SERVER_EVENTS_TYPE_ENUM *pEventTypes) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_APIVersion( 
            /* [retval][out] */ __RPC__out FAX_SERVER_APIVERSION_ENUM *pAPIVersion) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IFaxServerVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IFaxServer * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IFaxServer * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IFaxServer * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IFaxServer * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IFaxServer * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IFaxServer * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IFaxServer * This,
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
        
        DECLSPEC_XFGVIRT(IFaxServer, Connect)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *Connect )( 
            __RPC__in IFaxServer * This,
            /* [in] */ __RPC__in BSTR bstrServerName);
        
        DECLSPEC_XFGVIRT(IFaxServer, get_ServerName)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_ServerName )( 
            __RPC__in IFaxServer * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrServerName);
        
        DECLSPEC_XFGVIRT(IFaxServer, GetDeviceProviders)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *GetDeviceProviders )( 
            __RPC__in IFaxServer * This,
            /* [retval][out] */ __RPC__deref_out_opt IFaxDeviceProviders **ppFaxDeviceProviders);
        
        DECLSPEC_XFGVIRT(IFaxServer, GetDevices)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *GetDevices )( 
            __RPC__in IFaxServer * This,
            /* [retval][out] */ __RPC__deref_out_opt IFaxDevices **ppFaxDevices);
        
        DECLSPEC_XFGVIRT(IFaxServer, get_InboundRouting)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_InboundRouting )( 
            __RPC__in IFaxServer * This,
            /* [retval][out] */ __RPC__deref_out_opt IFaxInboundRouting **ppFaxInboundRouting);
        
        DECLSPEC_XFGVIRT(IFaxServer, get_Folders)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Folders )( 
            __RPC__in IFaxServer * This,
            /* [retval][out] */ __RPC__deref_out_opt IFaxFolders **pFaxFolders);
        
        DECLSPEC_XFGVIRT(IFaxServer, get_LoggingOptions)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_LoggingOptions )( 
            __RPC__in IFaxServer * This,
            /* [retval][out] */ __RPC__deref_out_opt IFaxLoggingOptions **ppFaxLoggingOptions);
        
        DECLSPEC_XFGVIRT(IFaxServer, get_MajorVersion)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_MajorVersion )( 
            __RPC__in IFaxServer * This,
            /* [retval][out] */ __RPC__out long *plMajorVersion);
        
        DECLSPEC_XFGVIRT(IFaxServer, get_MinorVersion)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_MinorVersion )( 
            __RPC__in IFaxServer * This,
            /* [retval][out] */ __RPC__out long *plMinorVersion);
        
        DECLSPEC_XFGVIRT(IFaxServer, get_MajorBuild)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_MajorBuild )( 
            __RPC__in IFaxServer * This,
            /* [retval][out] */ __RPC__out long *plMajorBuild);
        
        DECLSPEC_XFGVIRT(IFaxServer, get_MinorBuild)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_MinorBuild )( 
            __RPC__in IFaxServer * This,
            /* [retval][out] */ __RPC__out long *plMinorBuild);
        
        DECLSPEC_XFGVIRT(IFaxServer, get_Debug)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Debug )( 
            __RPC__in IFaxServer * This,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pbDebug);
        
        DECLSPEC_XFGVIRT(IFaxServer, get_Activity)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Activity )( 
            __RPC__in IFaxServer * This,
            /* [retval][out] */ __RPC__deref_out_opt IFaxActivity **ppFaxActivity);
        
        DECLSPEC_XFGVIRT(IFaxServer, get_OutboundRouting)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_OutboundRouting )( 
            __RPC__in IFaxServer * This,
            /* [retval][out] */ __RPC__deref_out_opt IFaxOutboundRouting **ppFaxOutboundRouting);
        
        DECLSPEC_XFGVIRT(IFaxServer, get_ReceiptOptions)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_ReceiptOptions )( 
            __RPC__in IFaxServer * This,
            /* [retval][out] */ __RPC__deref_out_opt IFaxReceiptOptions **ppFaxReceiptOptions);
        
        DECLSPEC_XFGVIRT(IFaxServer, get_Security)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Security )( 
            __RPC__in IFaxServer * This,
            /* [retval][out] */ __RPC__deref_out_opt IFaxSecurity **ppFaxSecurity);
        
        DECLSPEC_XFGVIRT(IFaxServer, Disconnect)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *Disconnect )( 
            __RPC__in IFaxServer * This);
        
        DECLSPEC_XFGVIRT(IFaxServer, GetExtensionProperty)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *GetExtensionProperty )( 
            __RPC__in IFaxServer * This,
            /* [in] */ __RPC__in BSTR bstrGUID,
            /* [retval][out] */ __RPC__out VARIANT *pvProperty);
        
        DECLSPEC_XFGVIRT(IFaxServer, SetExtensionProperty)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *SetExtensionProperty )( 
            __RPC__in IFaxServer * This,
            /* [in] */ __RPC__in BSTR bstrGUID,
            /* [in] */ VARIANT vProperty);
        
        DECLSPEC_XFGVIRT(IFaxServer, ListenToServerEvents)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *ListenToServerEvents )( 
            __RPC__in IFaxServer * This,
            /* [in] */ FAX_SERVER_EVENTS_TYPE_ENUM EventTypes);
        
        DECLSPEC_XFGVIRT(IFaxServer, RegisterDeviceProvider)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *RegisterDeviceProvider )( 
            __RPC__in IFaxServer * This,
            /* [in] */ __RPC__in BSTR bstrGUID,
            /* [in] */ __RPC__in BSTR bstrFriendlyName,
            /* [in] */ __RPC__in BSTR bstrImageName,
            /* [in] */ __RPC__in BSTR TspName,
            /* [in] */ long lFSPIVersion);
        
        DECLSPEC_XFGVIRT(IFaxServer, UnregisterDeviceProvider)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *UnregisterDeviceProvider )( 
            __RPC__in IFaxServer * This,
            /* [in] */ __RPC__in BSTR bstrUniqueName);
        
        DECLSPEC_XFGVIRT(IFaxServer, RegisterInboundRoutingExtension)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *RegisterInboundRoutingExtension )( 
            __RPC__in IFaxServer * This,
            /* [in] */ __RPC__in BSTR bstrExtensionName,
            /* [in] */ __RPC__in BSTR bstrFriendlyName,
            /* [in] */ __RPC__in BSTR bstrImageName,
            /* [in] */ VARIANT vMethods);
        
        DECLSPEC_XFGVIRT(IFaxServer, UnregisterInboundRoutingExtension)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *UnregisterInboundRoutingExtension )( 
            __RPC__in IFaxServer * This,
            /* [in] */ __RPC__in BSTR bstrExtensionUniqueName);
        
        DECLSPEC_XFGVIRT(IFaxServer, get_RegisteredEvents)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_RegisteredEvents )( 
            __RPC__in IFaxServer * This,
            /* [retval][out] */ __RPC__out FAX_SERVER_EVENTS_TYPE_ENUM *pEventTypes);
        
        DECLSPEC_XFGVIRT(IFaxServer, get_APIVersion)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_APIVersion )( 
            __RPC__in IFaxServer * This,
            /* [retval][out] */ __RPC__out FAX_SERVER_APIVERSION_ENUM *pAPIVersion);
        
        END_INTERFACE
    } IFaxServerVtbl;

    interface IFaxServer
    {
        CONST_VTBL struct IFaxServerVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IFaxServer_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IFaxServer_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IFaxServer_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IFaxServer_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IFaxServer_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IFaxServer_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IFaxServer_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IFaxServer_Connect(This,bstrServerName)	\
    ( (This)->lpVtbl -> Connect(This,bstrServerName) ) 

#define IFaxServer_get_ServerName(This,pbstrServerName)	\
    ( (This)->lpVtbl -> get_ServerName(This,pbstrServerName) ) 

#define IFaxServer_GetDeviceProviders(This,ppFaxDeviceProviders)	\
    ( (This)->lpVtbl -> GetDeviceProviders(This,ppFaxDeviceProviders) ) 

#define IFaxServer_GetDevices(This,ppFaxDevices)	\
    ( (This)->lpVtbl -> GetDevices(This,ppFaxDevices) ) 

#define IFaxServer_get_InboundRouting(This,ppFaxInboundRouting)	\
    ( (This)->lpVtbl -> get_InboundRouting(This,ppFaxInboundRouting) ) 

#define IFaxServer_get_Folders(This,pFaxFolders)	\
    ( (This)->lpVtbl -> get_Folders(This,pFaxFolders) ) 

#define IFaxServer_get_LoggingOptions(This,ppFaxLoggingOptions)	\
    ( (This)->lpVtbl -> get_LoggingOptions(This,ppFaxLoggingOptions) ) 

#define IFaxServer_get_MajorVersion(This,plMajorVersion)	\
    ( (This)->lpVtbl -> get_MajorVersion(This,plMajorVersion) ) 

#define IFaxServer_get_MinorVersion(This,plMinorVersion)	\
    ( (This)->lpVtbl -> get_MinorVersion(This,plMinorVersion) ) 

#define IFaxServer_get_MajorBuild(This,plMajorBuild)	\
    ( (This)->lpVtbl -> get_MajorBuild(This,plMajorBuild) ) 

#define IFaxServer_get_MinorBuild(This,plMinorBuild)	\
    ( (This)->lpVtbl -> get_MinorBuild(This,plMinorBuild) ) 

#define IFaxServer_get_Debug(This,pbDebug)	\
    ( (This)->lpVtbl -> get_Debug(This,pbDebug) ) 

#define IFaxServer_get_Activity(This,ppFaxActivity)	\
    ( (This)->lpVtbl -> get_Activity(This,ppFaxActivity) ) 

#define IFaxServer_get_OutboundRouting(This,ppFaxOutboundRouting)	\
    ( (This)->lpVtbl -> get_OutboundRouting(This,ppFaxOutboundRouting) ) 

#define IFaxServer_get_ReceiptOptions(This,ppFaxReceiptOptions)	\
    ( (This)->lpVtbl -> get_ReceiptOptions(This,ppFaxReceiptOptions) ) 

#define IFaxServer_get_Security(This,ppFaxSecurity)	\
    ( (This)->lpVtbl -> get_Security(This,ppFaxSecurity) ) 

#define IFaxServer_Disconnect(This)	\
    ( (This)->lpVtbl -> Disconnect(This) ) 

#define IFaxServer_GetExtensionProperty(This,bstrGUID,pvProperty)	\
    ( (This)->lpVtbl -> GetExtensionProperty(This,bstrGUID,pvProperty) ) 

#define IFaxServer_SetExtensionProperty(This,bstrGUID,vProperty)	\
    ( (This)->lpVtbl -> SetExtensionProperty(This,bstrGUID,vProperty) ) 

#define IFaxServer_ListenToServerEvents(This,EventTypes)	\
    ( (This)->lpVtbl -> ListenToServerEvents(This,EventTypes) ) 

#define IFaxServer_RegisterDeviceProvider(This,bstrGUID,bstrFriendlyName,bstrImageName,TspName,lFSPIVersion)	\
    ( (This)->lpVtbl -> RegisterDeviceProvider(This,bstrGUID,bstrFriendlyName,bstrImageName,TspName,lFSPIVersion) ) 

#define IFaxServer_UnregisterDeviceProvider(This,bstrUniqueName)	\
    ( (This)->lpVtbl -> UnregisterDeviceProvider(This,bstrUniqueName) ) 

#define IFaxServer_RegisterInboundRoutingExtension(This,bstrExtensionName,bstrFriendlyName,bstrImageName,vMethods)	\
    ( (This)->lpVtbl -> RegisterInboundRoutingExtension(This,bstrExtensionName,bstrFriendlyName,bstrImageName,vMethods) ) 

#define IFaxServer_UnregisterInboundRoutingExtension(This,bstrExtensionUniqueName)	\
    ( (This)->lpVtbl -> UnregisterInboundRoutingExtension(This,bstrExtensionUniqueName) ) 

#define IFaxServer_get_RegisteredEvents(This,pEventTypes)	\
    ( (This)->lpVtbl -> get_RegisteredEvents(This,pEventTypes) ) 

#define IFaxServer_get_APIVersion(This,pAPIVersion)	\
    ( (This)->lpVtbl -> get_APIVersion(This,pAPIVersion) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IFaxServer_INTERFACE_DEFINED__ */


#ifndef __IFaxDeviceProviders_INTERFACE_DEFINED__
#define __IFaxDeviceProviders_INTERFACE_DEFINED__

/* interface IFaxDeviceProviders */
/* [nonextensible][unique][helpstring][dual][uuid][object] */ 


EXTERN_C const IID IID_IFaxDeviceProviders;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("9FB76F62-4C7E-43A5-B6FD-502893F7E13E")
    IFaxDeviceProviders : public IDispatch
    {
    public:
        virtual /* [propget][id] */ HRESULT STDMETHODCALLTYPE get__NewEnum( 
            /* [retval][out] */ __RPC__deref_out_opt IUnknown **ppUnk) = 0;
        
        virtual /* [propget][id] */ HRESULT STDMETHODCALLTYPE get_Item( 
            /* [in] */ VARIANT vIndex,
            /* [retval][out] */ __RPC__deref_out_opt IFaxDeviceProvider **pFaxDeviceProvider) = 0;
        
        virtual /* [propget][id] */ HRESULT STDMETHODCALLTYPE get_Count( 
            /* [retval][out] */ __RPC__out long *plCount) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IFaxDeviceProvidersVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IFaxDeviceProviders * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IFaxDeviceProviders * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IFaxDeviceProviders * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IFaxDeviceProviders * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IFaxDeviceProviders * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IFaxDeviceProviders * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IFaxDeviceProviders * This,
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
        
        DECLSPEC_XFGVIRT(IFaxDeviceProviders, get__NewEnum)
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *get__NewEnum )( 
            __RPC__in IFaxDeviceProviders * This,
            /* [retval][out] */ __RPC__deref_out_opt IUnknown **ppUnk);
        
        DECLSPEC_XFGVIRT(IFaxDeviceProviders, get_Item)
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_Item )( 
            __RPC__in IFaxDeviceProviders * This,
            /* [in] */ VARIANT vIndex,
            /* [retval][out] */ __RPC__deref_out_opt IFaxDeviceProvider **pFaxDeviceProvider);
        
        DECLSPEC_XFGVIRT(IFaxDeviceProviders, get_Count)
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_Count )( 
            __RPC__in IFaxDeviceProviders * This,
            /* [retval][out] */ __RPC__out long *plCount);
        
        END_INTERFACE
    } IFaxDeviceProvidersVtbl;

    interface IFaxDeviceProviders
    {
        CONST_VTBL struct IFaxDeviceProvidersVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IFaxDeviceProviders_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IFaxDeviceProviders_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IFaxDeviceProviders_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IFaxDeviceProviders_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IFaxDeviceProviders_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IFaxDeviceProviders_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IFaxDeviceProviders_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IFaxDeviceProviders_get__NewEnum(This,ppUnk)	\
    ( (This)->lpVtbl -> get__NewEnum(This,ppUnk) ) 

#define IFaxDeviceProviders_get_Item(This,vIndex,pFaxDeviceProvider)	\
    ( (This)->lpVtbl -> get_Item(This,vIndex,pFaxDeviceProvider) ) 

#define IFaxDeviceProviders_get_Count(This,plCount)	\
    ( (This)->lpVtbl -> get_Count(This,plCount) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IFaxDeviceProviders_INTERFACE_DEFINED__ */


#ifndef __IFaxDevices_INTERFACE_DEFINED__
#define __IFaxDevices_INTERFACE_DEFINED__

/* interface IFaxDevices */
/* [nonextensible][unique][helpstring][dual][uuid][object] */ 


EXTERN_C const IID IID_IFaxDevices;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("9E46783E-F34F-482E-A360-0416BECBBD96")
    IFaxDevices : public IDispatch
    {
    public:
        virtual /* [propget][id] */ HRESULT STDMETHODCALLTYPE get__NewEnum( 
            /* [retval][out] */ __RPC__deref_out_opt IUnknown **ppUnk) = 0;
        
        virtual /* [propget][id] */ HRESULT STDMETHODCALLTYPE get_Item( 
            /* [in] */ VARIANT vIndex,
            /* [retval][out] */ __RPC__deref_out_opt IFaxDevice **pFaxDevice) = 0;
        
        virtual /* [propget][id] */ HRESULT STDMETHODCALLTYPE get_Count( 
            /* [retval][out] */ __RPC__out long *plCount) = 0;
        
        virtual /* [propget][helpstring][id] */ HRESULT STDMETHODCALLTYPE get_ItemById( 
            /* [in] */ long lId,
            /* [retval][out] */ __RPC__deref_out_opt IFaxDevice **ppFaxDevice) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IFaxDevicesVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IFaxDevices * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IFaxDevices * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IFaxDevices * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IFaxDevices * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IFaxDevices * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IFaxDevices * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IFaxDevices * This,
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
        
        DECLSPEC_XFGVIRT(IFaxDevices, get__NewEnum)
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *get__NewEnum )( 
            __RPC__in IFaxDevices * This,
            /* [retval][out] */ __RPC__deref_out_opt IUnknown **ppUnk);
        
        DECLSPEC_XFGVIRT(IFaxDevices, get_Item)
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_Item )( 
            __RPC__in IFaxDevices * This,
            /* [in] */ VARIANT vIndex,
            /* [retval][out] */ __RPC__deref_out_opt IFaxDevice **pFaxDevice);
        
        DECLSPEC_XFGVIRT(IFaxDevices, get_Count)
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_Count )( 
            __RPC__in IFaxDevices * This,
            /* [retval][out] */ __RPC__out long *plCount);
        
        DECLSPEC_XFGVIRT(IFaxDevices, get_ItemById)
        /* [propget][helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *get_ItemById )( 
            __RPC__in IFaxDevices * This,
            /* [in] */ long lId,
            /* [retval][out] */ __RPC__deref_out_opt IFaxDevice **ppFaxDevice);
        
        END_INTERFACE
    } IFaxDevicesVtbl;

    interface IFaxDevices
    {
        CONST_VTBL struct IFaxDevicesVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IFaxDevices_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IFaxDevices_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IFaxDevices_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IFaxDevices_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IFaxDevices_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IFaxDevices_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IFaxDevices_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IFaxDevices_get__NewEnum(This,ppUnk)	\
    ( (This)->lpVtbl -> get__NewEnum(This,ppUnk) ) 

#define IFaxDevices_get_Item(This,vIndex,pFaxDevice)	\
    ( (This)->lpVtbl -> get_Item(This,vIndex,pFaxDevice) ) 

#define IFaxDevices_get_Count(This,plCount)	\
    ( (This)->lpVtbl -> get_Count(This,plCount) ) 

#define IFaxDevices_get_ItemById(This,lId,ppFaxDevice)	\
    ( (This)->lpVtbl -> get_ItemById(This,lId,ppFaxDevice) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IFaxDevices_INTERFACE_DEFINED__ */


#ifndef __IFaxInboundRouting_INTERFACE_DEFINED__
#define __IFaxInboundRouting_INTERFACE_DEFINED__

/* interface IFaxInboundRouting */
/* [nonextensible][unique][helpstring][dual][uuid][object] */ 


EXTERN_C const IID IID_IFaxInboundRouting;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("8148C20F-9D52-45B1-BF96-38FC12713527")
    IFaxInboundRouting : public IDispatch
    {
    public:
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE GetExtensions( 
            /* [retval][out] */ __RPC__deref_out_opt IFaxInboundRoutingExtensions **pFaxInboundRoutingExtensions) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE GetMethods( 
            /* [retval][out] */ __RPC__deref_out_opt IFaxInboundRoutingMethods **pFaxInboundRoutingMethods) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IFaxInboundRoutingVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IFaxInboundRouting * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IFaxInboundRouting * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IFaxInboundRouting * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IFaxInboundRouting * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IFaxInboundRouting * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IFaxInboundRouting * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IFaxInboundRouting * This,
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
        
        DECLSPEC_XFGVIRT(IFaxInboundRouting, GetExtensions)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *GetExtensions )( 
            __RPC__in IFaxInboundRouting * This,
            /* [retval][out] */ __RPC__deref_out_opt IFaxInboundRoutingExtensions **pFaxInboundRoutingExtensions);
        
        DECLSPEC_XFGVIRT(IFaxInboundRouting, GetMethods)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *GetMethods )( 
            __RPC__in IFaxInboundRouting * This,
            /* [retval][out] */ __RPC__deref_out_opt IFaxInboundRoutingMethods **pFaxInboundRoutingMethods);
        
        END_INTERFACE
    } IFaxInboundRoutingVtbl;

    interface IFaxInboundRouting
    {
        CONST_VTBL struct IFaxInboundRoutingVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IFaxInboundRouting_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IFaxInboundRouting_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IFaxInboundRouting_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IFaxInboundRouting_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IFaxInboundRouting_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IFaxInboundRouting_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IFaxInboundRouting_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IFaxInboundRouting_GetExtensions(This,pFaxInboundRoutingExtensions)	\
    ( (This)->lpVtbl -> GetExtensions(This,pFaxInboundRoutingExtensions) ) 

#define IFaxInboundRouting_GetMethods(This,pFaxInboundRoutingMethods)	\
    ( (This)->lpVtbl -> GetMethods(This,pFaxInboundRoutingMethods) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IFaxInboundRouting_INTERFACE_DEFINED__ */


#ifndef __IFaxFolders_INTERFACE_DEFINED__
#define __IFaxFolders_INTERFACE_DEFINED__

/* interface IFaxFolders */
/* [nonextensible][unique][helpstring][dual][uuid][object] */ 


EXTERN_C const IID IID_IFaxFolders;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("DCE3B2A8-A7AB-42BC-9D0A-3149457261A0")
    IFaxFolders : public IDispatch
    {
    public:
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_OutgoingQueue( 
            /* [retval][out] */ __RPC__deref_out_opt IFaxOutgoingQueue **pFaxOutgoingQueue) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_IncomingQueue( 
            /* [retval][out] */ __RPC__deref_out_opt IFaxIncomingQueue **pFaxIncomingQueue) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_IncomingArchive( 
            /* [retval][out] */ __RPC__deref_out_opt IFaxIncomingArchive **pFaxIncomingArchive) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_OutgoingArchive( 
            /* [retval][out] */ __RPC__deref_out_opt IFaxOutgoingArchive **pFaxOutgoingArchive) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IFaxFoldersVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IFaxFolders * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IFaxFolders * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IFaxFolders * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IFaxFolders * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IFaxFolders * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IFaxFolders * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IFaxFolders * This,
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
        
        DECLSPEC_XFGVIRT(IFaxFolders, get_OutgoingQueue)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_OutgoingQueue )( 
            __RPC__in IFaxFolders * This,
            /* [retval][out] */ __RPC__deref_out_opt IFaxOutgoingQueue **pFaxOutgoingQueue);
        
        DECLSPEC_XFGVIRT(IFaxFolders, get_IncomingQueue)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_IncomingQueue )( 
            __RPC__in IFaxFolders * This,
            /* [retval][out] */ __RPC__deref_out_opt IFaxIncomingQueue **pFaxIncomingQueue);
        
        DECLSPEC_XFGVIRT(IFaxFolders, get_IncomingArchive)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_IncomingArchive )( 
            __RPC__in IFaxFolders * This,
            /* [retval][out] */ __RPC__deref_out_opt IFaxIncomingArchive **pFaxIncomingArchive);
        
        DECLSPEC_XFGVIRT(IFaxFolders, get_OutgoingArchive)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_OutgoingArchive )( 
            __RPC__in IFaxFolders * This,
            /* [retval][out] */ __RPC__deref_out_opt IFaxOutgoingArchive **pFaxOutgoingArchive);
        
        END_INTERFACE
    } IFaxFoldersVtbl;

    interface IFaxFolders
    {
        CONST_VTBL struct IFaxFoldersVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IFaxFolders_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IFaxFolders_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IFaxFolders_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IFaxFolders_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IFaxFolders_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IFaxFolders_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IFaxFolders_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IFaxFolders_get_OutgoingQueue(This,pFaxOutgoingQueue)	\
    ( (This)->lpVtbl -> get_OutgoingQueue(This,pFaxOutgoingQueue) ) 

#define IFaxFolders_get_IncomingQueue(This,pFaxIncomingQueue)	\
    ( (This)->lpVtbl -> get_IncomingQueue(This,pFaxIncomingQueue) ) 

#define IFaxFolders_get_IncomingArchive(This,pFaxIncomingArchive)	\
    ( (This)->lpVtbl -> get_IncomingArchive(This,pFaxIncomingArchive) ) 

#define IFaxFolders_get_OutgoingArchive(This,pFaxOutgoingArchive)	\
    ( (This)->lpVtbl -> get_OutgoingArchive(This,pFaxOutgoingArchive) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IFaxFolders_INTERFACE_DEFINED__ */


#ifndef __IFaxLoggingOptions_INTERFACE_DEFINED__
#define __IFaxLoggingOptions_INTERFACE_DEFINED__

/* interface IFaxLoggingOptions */
/* [nonextensible][unique][helpstring][dual][uuid][object] */ 


EXTERN_C const IID IID_IFaxLoggingOptions;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("34E64FB9-6B31-4D32-8B27-D286C0C33606")
    IFaxLoggingOptions : public IDispatch
    {
    public:
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_EventLogging( 
            /* [retval][out] */ __RPC__deref_out_opt IFaxEventLogging **pFaxEventLogging) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_ActivityLogging( 
            /* [retval][out] */ __RPC__deref_out_opt IFaxActivityLogging **pFaxActivityLogging) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IFaxLoggingOptionsVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IFaxLoggingOptions * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IFaxLoggingOptions * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IFaxLoggingOptions * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IFaxLoggingOptions * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IFaxLoggingOptions * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IFaxLoggingOptions * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IFaxLoggingOptions * This,
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
        
        DECLSPEC_XFGVIRT(IFaxLoggingOptions, get_EventLogging)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_EventLogging )( 
            __RPC__in IFaxLoggingOptions * This,
            /* [retval][out] */ __RPC__deref_out_opt IFaxEventLogging **pFaxEventLogging);
        
        DECLSPEC_XFGVIRT(IFaxLoggingOptions, get_ActivityLogging)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_ActivityLogging )( 
            __RPC__in IFaxLoggingOptions * This,
            /* [retval][out] */ __RPC__deref_out_opt IFaxActivityLogging **pFaxActivityLogging);
        
        END_INTERFACE
    } IFaxLoggingOptionsVtbl;

    interface IFaxLoggingOptions
    {
        CONST_VTBL struct IFaxLoggingOptionsVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IFaxLoggingOptions_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IFaxLoggingOptions_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IFaxLoggingOptions_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IFaxLoggingOptions_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IFaxLoggingOptions_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IFaxLoggingOptions_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IFaxLoggingOptions_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IFaxLoggingOptions_get_EventLogging(This,pFaxEventLogging)	\
    ( (This)->lpVtbl -> get_EventLogging(This,pFaxEventLogging) ) 

#define IFaxLoggingOptions_get_ActivityLogging(This,pFaxActivityLogging)	\
    ( (This)->lpVtbl -> get_ActivityLogging(This,pFaxActivityLogging) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IFaxLoggingOptions_INTERFACE_DEFINED__ */


#ifndef __IFaxActivity_INTERFACE_DEFINED__
#define __IFaxActivity_INTERFACE_DEFINED__

/* interface IFaxActivity */
/* [nonextensible][unique][helpstring][dual][uuid][object] */ 


EXTERN_C const IID IID_IFaxActivity;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("4B106F97-3DF5-40F2-BC3C-44CB8115EBDF")
    IFaxActivity : public IDispatch
    {
    public:
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_IncomingMessages( 
            /* [retval][out] */ __RPC__out long *plIncomingMessages) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_RoutingMessages( 
            /* [retval][out] */ __RPC__out long *plRoutingMessages) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_OutgoingMessages( 
            /* [retval][out] */ __RPC__out long *plOutgoingMessages) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_QueuedMessages( 
            /* [retval][out] */ __RPC__out long *plQueuedMessages) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE Refresh( void) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IFaxActivityVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IFaxActivity * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IFaxActivity * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IFaxActivity * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IFaxActivity * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IFaxActivity * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IFaxActivity * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IFaxActivity * This,
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
        
        DECLSPEC_XFGVIRT(IFaxActivity, get_IncomingMessages)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_IncomingMessages )( 
            __RPC__in IFaxActivity * This,
            /* [retval][out] */ __RPC__out long *plIncomingMessages);
        
        DECLSPEC_XFGVIRT(IFaxActivity, get_RoutingMessages)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_RoutingMessages )( 
            __RPC__in IFaxActivity * This,
            /* [retval][out] */ __RPC__out long *plRoutingMessages);
        
        DECLSPEC_XFGVIRT(IFaxActivity, get_OutgoingMessages)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_OutgoingMessages )( 
            __RPC__in IFaxActivity * This,
            /* [retval][out] */ __RPC__out long *plOutgoingMessages);
        
        DECLSPEC_XFGVIRT(IFaxActivity, get_QueuedMessages)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_QueuedMessages )( 
            __RPC__in IFaxActivity * This,
            /* [retval][out] */ __RPC__out long *plQueuedMessages);
        
        DECLSPEC_XFGVIRT(IFaxActivity, Refresh)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *Refresh )( 
            __RPC__in IFaxActivity * This);
        
        END_INTERFACE
    } IFaxActivityVtbl;

    interface IFaxActivity
    {
        CONST_VTBL struct IFaxActivityVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IFaxActivity_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IFaxActivity_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IFaxActivity_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IFaxActivity_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IFaxActivity_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IFaxActivity_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IFaxActivity_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IFaxActivity_get_IncomingMessages(This,plIncomingMessages)	\
    ( (This)->lpVtbl -> get_IncomingMessages(This,plIncomingMessages) ) 

#define IFaxActivity_get_RoutingMessages(This,plRoutingMessages)	\
    ( (This)->lpVtbl -> get_RoutingMessages(This,plRoutingMessages) ) 

#define IFaxActivity_get_OutgoingMessages(This,plOutgoingMessages)	\
    ( (This)->lpVtbl -> get_OutgoingMessages(This,plOutgoingMessages) ) 

#define IFaxActivity_get_QueuedMessages(This,plQueuedMessages)	\
    ( (This)->lpVtbl -> get_QueuedMessages(This,plQueuedMessages) ) 

#define IFaxActivity_Refresh(This)	\
    ( (This)->lpVtbl -> Refresh(This) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IFaxActivity_INTERFACE_DEFINED__ */


#ifndef __IFaxOutboundRouting_INTERFACE_DEFINED__
#define __IFaxOutboundRouting_INTERFACE_DEFINED__

/* interface IFaxOutboundRouting */
/* [nonextensible][unique][helpstring][dual][uuid][object] */ 


EXTERN_C const IID IID_IFaxOutboundRouting;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("25DC05A4-9909-41BD-A95B-7E5D1DEC1D43")
    IFaxOutboundRouting : public IDispatch
    {
    public:
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE GetGroups( 
            /* [retval][out] */ __RPC__deref_out_opt IFaxOutboundRoutingGroups **pFaxOutboundRoutingGroups) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE GetRules( 
            /* [retval][out] */ __RPC__deref_out_opt IFaxOutboundRoutingRules **pFaxOutboundRoutingRules) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IFaxOutboundRoutingVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IFaxOutboundRouting * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IFaxOutboundRouting * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IFaxOutboundRouting * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IFaxOutboundRouting * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IFaxOutboundRouting * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IFaxOutboundRouting * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IFaxOutboundRouting * This,
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
        
        DECLSPEC_XFGVIRT(IFaxOutboundRouting, GetGroups)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *GetGroups )( 
            __RPC__in IFaxOutboundRouting * This,
            /* [retval][out] */ __RPC__deref_out_opt IFaxOutboundRoutingGroups **pFaxOutboundRoutingGroups);
        
        DECLSPEC_XFGVIRT(IFaxOutboundRouting, GetRules)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *GetRules )( 
            __RPC__in IFaxOutboundRouting * This,
            /* [retval][out] */ __RPC__deref_out_opt IFaxOutboundRoutingRules **pFaxOutboundRoutingRules);
        
        END_INTERFACE
    } IFaxOutboundRoutingVtbl;

    interface IFaxOutboundRouting
    {
        CONST_VTBL struct IFaxOutboundRoutingVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IFaxOutboundRouting_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IFaxOutboundRouting_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IFaxOutboundRouting_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IFaxOutboundRouting_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IFaxOutboundRouting_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IFaxOutboundRouting_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IFaxOutboundRouting_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IFaxOutboundRouting_GetGroups(This,pFaxOutboundRoutingGroups)	\
    ( (This)->lpVtbl -> GetGroups(This,pFaxOutboundRoutingGroups) ) 

#define IFaxOutboundRouting_GetRules(This,pFaxOutboundRoutingRules)	\
    ( (This)->lpVtbl -> GetRules(This,pFaxOutboundRoutingRules) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IFaxOutboundRouting_INTERFACE_DEFINED__ */


#ifndef __IFaxReceiptOptions_INTERFACE_DEFINED__
#define __IFaxReceiptOptions_INTERFACE_DEFINED__

/* interface IFaxReceiptOptions */
/* [nonextensible][unique][helpstring][dual][uuid][object] */ 

typedef 
enum FAX_SMTP_AUTHENTICATION_TYPE_ENUM
    {
        fsatANONYMOUS	= 0,
        fsatBASIC	= ( fsatANONYMOUS + 1 ) ,
        fsatNTLM	= ( fsatBASIC + 1 ) 
    } 	FAX_SMTP_AUTHENTICATION_TYPE_ENUM;

typedef 
enum FAX_RECEIPT_TYPE_ENUM
    {
        frtNONE	= 0,
        frtMAIL	= 0x1,
        frtMSGBOX	= 0x4
    } 	FAX_RECEIPT_TYPE_ENUM;


EXTERN_C const IID IID_IFaxReceiptOptions;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("378EFAEB-5FCB-4AFB-B2EE-E16E80614487")
    IFaxReceiptOptions : public IDispatch
    {
    public:
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_AuthenticationType( 
            /* [retval][out] */ __RPC__out FAX_SMTP_AUTHENTICATION_TYPE_ENUM *pType) = 0;
        
        virtual /* [helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_AuthenticationType( 
            /* [in] */ FAX_SMTP_AUTHENTICATION_TYPE_ENUM Type) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_SMTPServer( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrSMTPServer) = 0;
        
        virtual /* [helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_SMTPServer( 
            /* [in] */ __RPC__in BSTR bstrSMTPServer) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_SMTPPort( 
            /* [retval][out] */ __RPC__out long *plSMTPPort) = 0;
        
        virtual /* [helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_SMTPPort( 
            /* [in] */ long lSMTPPort) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_SMTPSender( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrSMTPSender) = 0;
        
        virtual /* [helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_SMTPSender( 
            /* [in] */ __RPC__in BSTR bstrSMTPSender) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_SMTPUser( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrSMTPUser) = 0;
        
        virtual /* [helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_SMTPUser( 
            /* [in] */ __RPC__in BSTR bstrSMTPUser) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_AllowedReceipts( 
            /* [retval][out] */ __RPC__out FAX_RECEIPT_TYPE_ENUM *pAllowedReceipts) = 0;
        
        virtual /* [helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_AllowedReceipts( 
            /* [in] */ FAX_RECEIPT_TYPE_ENUM AllowedReceipts) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_SMTPPassword( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrSMTPPassword) = 0;
        
        virtual /* [helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_SMTPPassword( 
            /* [in] */ __RPC__in BSTR bstrSMTPPassword) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE Refresh( void) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE Save( void) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_UseForInboundRouting( 
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pbUseForInboundRouting) = 0;
        
        virtual /* [helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_UseForInboundRouting( 
            /* [in] */ VARIANT_BOOL bUseForInboundRouting) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IFaxReceiptOptionsVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IFaxReceiptOptions * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IFaxReceiptOptions * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IFaxReceiptOptions * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IFaxReceiptOptions * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IFaxReceiptOptions * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IFaxReceiptOptions * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IFaxReceiptOptions * This,
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
        
        DECLSPEC_XFGVIRT(IFaxReceiptOptions, get_AuthenticationType)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_AuthenticationType )( 
            __RPC__in IFaxReceiptOptions * This,
            /* [retval][out] */ __RPC__out FAX_SMTP_AUTHENTICATION_TYPE_ENUM *pType);
        
        DECLSPEC_XFGVIRT(IFaxReceiptOptions, put_AuthenticationType)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_AuthenticationType )( 
            __RPC__in IFaxReceiptOptions * This,
            /* [in] */ FAX_SMTP_AUTHENTICATION_TYPE_ENUM Type);
        
        DECLSPEC_XFGVIRT(IFaxReceiptOptions, get_SMTPServer)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_SMTPServer )( 
            __RPC__in IFaxReceiptOptions * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrSMTPServer);
        
        DECLSPEC_XFGVIRT(IFaxReceiptOptions, put_SMTPServer)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_SMTPServer )( 
            __RPC__in IFaxReceiptOptions * This,
            /* [in] */ __RPC__in BSTR bstrSMTPServer);
        
        DECLSPEC_XFGVIRT(IFaxReceiptOptions, get_SMTPPort)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_SMTPPort )( 
            __RPC__in IFaxReceiptOptions * This,
            /* [retval][out] */ __RPC__out long *plSMTPPort);
        
        DECLSPEC_XFGVIRT(IFaxReceiptOptions, put_SMTPPort)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_SMTPPort )( 
            __RPC__in IFaxReceiptOptions * This,
            /* [in] */ long lSMTPPort);
        
        DECLSPEC_XFGVIRT(IFaxReceiptOptions, get_SMTPSender)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_SMTPSender )( 
            __RPC__in IFaxReceiptOptions * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrSMTPSender);
        
        DECLSPEC_XFGVIRT(IFaxReceiptOptions, put_SMTPSender)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_SMTPSender )( 
            __RPC__in IFaxReceiptOptions * This,
            /* [in] */ __RPC__in BSTR bstrSMTPSender);
        
        DECLSPEC_XFGVIRT(IFaxReceiptOptions, get_SMTPUser)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_SMTPUser )( 
            __RPC__in IFaxReceiptOptions * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrSMTPUser);
        
        DECLSPEC_XFGVIRT(IFaxReceiptOptions, put_SMTPUser)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_SMTPUser )( 
            __RPC__in IFaxReceiptOptions * This,
            /* [in] */ __RPC__in BSTR bstrSMTPUser);
        
        DECLSPEC_XFGVIRT(IFaxReceiptOptions, get_AllowedReceipts)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_AllowedReceipts )( 
            __RPC__in IFaxReceiptOptions * This,
            /* [retval][out] */ __RPC__out FAX_RECEIPT_TYPE_ENUM *pAllowedReceipts);
        
        DECLSPEC_XFGVIRT(IFaxReceiptOptions, put_AllowedReceipts)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_AllowedReceipts )( 
            __RPC__in IFaxReceiptOptions * This,
            /* [in] */ FAX_RECEIPT_TYPE_ENUM AllowedReceipts);
        
        DECLSPEC_XFGVIRT(IFaxReceiptOptions, get_SMTPPassword)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_SMTPPassword )( 
            __RPC__in IFaxReceiptOptions * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrSMTPPassword);
        
        DECLSPEC_XFGVIRT(IFaxReceiptOptions, put_SMTPPassword)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_SMTPPassword )( 
            __RPC__in IFaxReceiptOptions * This,
            /* [in] */ __RPC__in BSTR bstrSMTPPassword);
        
        DECLSPEC_XFGVIRT(IFaxReceiptOptions, Refresh)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *Refresh )( 
            __RPC__in IFaxReceiptOptions * This);
        
        DECLSPEC_XFGVIRT(IFaxReceiptOptions, Save)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *Save )( 
            __RPC__in IFaxReceiptOptions * This);
        
        DECLSPEC_XFGVIRT(IFaxReceiptOptions, get_UseForInboundRouting)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_UseForInboundRouting )( 
            __RPC__in IFaxReceiptOptions * This,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pbUseForInboundRouting);
        
        DECLSPEC_XFGVIRT(IFaxReceiptOptions, put_UseForInboundRouting)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_UseForInboundRouting )( 
            __RPC__in IFaxReceiptOptions * This,
            /* [in] */ VARIANT_BOOL bUseForInboundRouting);
        
        END_INTERFACE
    } IFaxReceiptOptionsVtbl;

    interface IFaxReceiptOptions
    {
        CONST_VTBL struct IFaxReceiptOptionsVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IFaxReceiptOptions_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IFaxReceiptOptions_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IFaxReceiptOptions_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IFaxReceiptOptions_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IFaxReceiptOptions_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IFaxReceiptOptions_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IFaxReceiptOptions_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IFaxReceiptOptions_get_AuthenticationType(This,pType)	\
    ( (This)->lpVtbl -> get_AuthenticationType(This,pType) ) 

#define IFaxReceiptOptions_put_AuthenticationType(This,Type)	\
    ( (This)->lpVtbl -> put_AuthenticationType(This,Type) ) 

#define IFaxReceiptOptions_get_SMTPServer(This,pbstrSMTPServer)	\
    ( (This)->lpVtbl -> get_SMTPServer(This,pbstrSMTPServer) ) 

#define IFaxReceiptOptions_put_SMTPServer(This,bstrSMTPServer)	\
    ( (This)->lpVtbl -> put_SMTPServer(This,bstrSMTPServer) ) 

#define IFaxReceiptOptions_get_SMTPPort(This,plSMTPPort)	\
    ( (This)->lpVtbl -> get_SMTPPort(This,plSMTPPort) ) 

#define IFaxReceiptOptions_put_SMTPPort(This,lSMTPPort)	\
    ( (This)->lpVtbl -> put_SMTPPort(This,lSMTPPort) ) 

#define IFaxReceiptOptions_get_SMTPSender(This,pbstrSMTPSender)	\
    ( (This)->lpVtbl -> get_SMTPSender(This,pbstrSMTPSender) ) 

#define IFaxReceiptOptions_put_SMTPSender(This,bstrSMTPSender)	\
    ( (This)->lpVtbl -> put_SMTPSender(This,bstrSMTPSender) ) 

#define IFaxReceiptOptions_get_SMTPUser(This,pbstrSMTPUser)	\
    ( (This)->lpVtbl -> get_SMTPUser(This,pbstrSMTPUser) ) 

#define IFaxReceiptOptions_put_SMTPUser(This,bstrSMTPUser)	\
    ( (This)->lpVtbl -> put_SMTPUser(This,bstrSMTPUser) ) 

#define IFaxReceiptOptions_get_AllowedReceipts(This,pAllowedReceipts)	\
    ( (This)->lpVtbl -> get_AllowedReceipts(This,pAllowedReceipts) ) 

#define IFaxReceiptOptions_put_AllowedReceipts(This,AllowedReceipts)	\
    ( (This)->lpVtbl -> put_AllowedReceipts(This,AllowedReceipts) ) 

#define IFaxReceiptOptions_get_SMTPPassword(This,pbstrSMTPPassword)	\
    ( (This)->lpVtbl -> get_SMTPPassword(This,pbstrSMTPPassword) ) 

#define IFaxReceiptOptions_put_SMTPPassword(This,bstrSMTPPassword)	\
    ( (This)->lpVtbl -> put_SMTPPassword(This,bstrSMTPPassword) ) 

#define IFaxReceiptOptions_Refresh(This)	\
    ( (This)->lpVtbl -> Refresh(This) ) 

#define IFaxReceiptOptions_Save(This)	\
    ( (This)->lpVtbl -> Save(This) ) 

#define IFaxReceiptOptions_get_UseForInboundRouting(This,pbUseForInboundRouting)	\
    ( (This)->lpVtbl -> get_UseForInboundRouting(This,pbUseForInboundRouting) ) 

#define IFaxReceiptOptions_put_UseForInboundRouting(This,bUseForInboundRouting)	\
    ( (This)->lpVtbl -> put_UseForInboundRouting(This,bUseForInboundRouting) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IFaxReceiptOptions_INTERFACE_DEFINED__ */


#ifndef __IFaxSecurity_INTERFACE_DEFINED__
#define __IFaxSecurity_INTERFACE_DEFINED__

/* interface IFaxSecurity */
/* [nonextensible][unique][helpstring][dual][uuid][object] */ 

typedef 
enum FAX_ACCESS_RIGHTS_ENUM
    {
        farSUBMIT_LOW	= 0x1,
        farSUBMIT_NORMAL	= 0x2,
        farSUBMIT_HIGH	= 0x4,
        farQUERY_JOBS	= 0x8,
        farMANAGE_JOBS	= 0x10,
        farQUERY_CONFIG	= 0x20,
        farMANAGE_CONFIG	= 0x40,
        farQUERY_IN_ARCHIVE	= 0x80,
        farMANAGE_IN_ARCHIVE	= 0x100,
        farQUERY_OUT_ARCHIVE	= 0x200,
        farMANAGE_OUT_ARCHIVE	= 0x400
    } 	FAX_ACCESS_RIGHTS_ENUM;


EXTERN_C const IID IID_IFaxSecurity;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("77B508C1-09C0-47A2-91EB-FCE7FDF2690E")
    IFaxSecurity : public IDispatch
    {
    public:
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Descriptor( 
            /* [retval][out] */ __RPC__out VARIANT *pvDescriptor) = 0;
        
        virtual /* [helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_Descriptor( 
            /* [in] */ VARIANT vDescriptor) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_GrantedRights( 
            /* [retval][out] */ __RPC__out FAX_ACCESS_RIGHTS_ENUM *pGrantedRights) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE Refresh( void) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE Save( void) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_InformationType( 
            /* [retval][out] */ __RPC__out long *plInformationType) = 0;
        
        virtual /* [helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_InformationType( 
            /* [in] */ long lInformationType) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IFaxSecurityVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IFaxSecurity * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IFaxSecurity * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IFaxSecurity * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IFaxSecurity * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IFaxSecurity * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IFaxSecurity * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IFaxSecurity * This,
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
        
        DECLSPEC_XFGVIRT(IFaxSecurity, get_Descriptor)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Descriptor )( 
            __RPC__in IFaxSecurity * This,
            /* [retval][out] */ __RPC__out VARIANT *pvDescriptor);
        
        DECLSPEC_XFGVIRT(IFaxSecurity, put_Descriptor)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_Descriptor )( 
            __RPC__in IFaxSecurity * This,
            /* [in] */ VARIANT vDescriptor);
        
        DECLSPEC_XFGVIRT(IFaxSecurity, get_GrantedRights)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_GrantedRights )( 
            __RPC__in IFaxSecurity * This,
            /* [retval][out] */ __RPC__out FAX_ACCESS_RIGHTS_ENUM *pGrantedRights);
        
        DECLSPEC_XFGVIRT(IFaxSecurity, Refresh)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *Refresh )( 
            __RPC__in IFaxSecurity * This);
        
        DECLSPEC_XFGVIRT(IFaxSecurity, Save)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *Save )( 
            __RPC__in IFaxSecurity * This);
        
        DECLSPEC_XFGVIRT(IFaxSecurity, get_InformationType)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_InformationType )( 
            __RPC__in IFaxSecurity * This,
            /* [retval][out] */ __RPC__out long *plInformationType);
        
        DECLSPEC_XFGVIRT(IFaxSecurity, put_InformationType)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_InformationType )( 
            __RPC__in IFaxSecurity * This,
            /* [in] */ long lInformationType);
        
        END_INTERFACE
    } IFaxSecurityVtbl;

    interface IFaxSecurity
    {
        CONST_VTBL struct IFaxSecurityVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IFaxSecurity_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IFaxSecurity_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IFaxSecurity_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IFaxSecurity_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IFaxSecurity_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IFaxSecurity_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IFaxSecurity_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IFaxSecurity_get_Descriptor(This,pvDescriptor)	\
    ( (This)->lpVtbl -> get_Descriptor(This,pvDescriptor) ) 

#define IFaxSecurity_put_Descriptor(This,vDescriptor)	\
    ( (This)->lpVtbl -> put_Descriptor(This,vDescriptor) ) 

#define IFaxSecurity_get_GrantedRights(This,pGrantedRights)	\
    ( (This)->lpVtbl -> get_GrantedRights(This,pGrantedRights) ) 

#define IFaxSecurity_Refresh(This)	\
    ( (This)->lpVtbl -> Refresh(This) ) 

#define IFaxSecurity_Save(This)	\
    ( (This)->lpVtbl -> Save(This) ) 

#define IFaxSecurity_get_InformationType(This,plInformationType)	\
    ( (This)->lpVtbl -> get_InformationType(This,plInformationType) ) 

#define IFaxSecurity_put_InformationType(This,lInformationType)	\
    ( (This)->lpVtbl -> put_InformationType(This,lInformationType) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IFaxSecurity_INTERFACE_DEFINED__ */


#ifndef __IFaxDocument_INTERFACE_DEFINED__
#define __IFaxDocument_INTERFACE_DEFINED__

/* interface IFaxDocument */
/* [nonextensible][unique][helpstring][dual][uuid][object] */ 

typedef 
enum FAX_PRIORITY_TYPE_ENUM
    {
        fptLOW	= 0,
        fptNORMAL	= ( fptLOW + 1 ) ,
        fptHIGH	= ( fptNORMAL + 1 ) 
    } 	FAX_PRIORITY_TYPE_ENUM;

typedef 
enum FAX_COVERPAGE_TYPE_ENUM
    {
        fcptNONE	= 0,
        fcptLOCAL	= ( fcptNONE + 1 ) ,
        fcptSERVER	= ( fcptLOCAL + 1 ) 
    } 	FAX_COVERPAGE_TYPE_ENUM;

typedef 
enum FAX_SCHEDULE_TYPE_ENUM
    {
        fstNOW	= 0,
        fstSPECIFIC_TIME	= ( fstNOW + 1 ) ,
        fstDISCOUNT_PERIOD	= ( fstSPECIFIC_TIME + 1 ) 
    } 	FAX_SCHEDULE_TYPE_ENUM;


EXTERN_C const IID IID_IFaxDocument;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("B207A246-09E3-4A4E-A7DC-FEA31D29458F")
    IFaxDocument : public IDispatch
    {
    public:
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Body( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrBody) = 0;
        
        virtual /* [helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_Body( 
            /* [in] */ __RPC__in BSTR bstrBody) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Sender( 
            /* [retval][out] */ __RPC__deref_out_opt IFaxSender **ppFaxSender) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Recipients( 
            /* [retval][out] */ __RPC__deref_out_opt IFaxRecipients **ppFaxRecipients) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_CoverPage( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrCoverPage) = 0;
        
        virtual /* [helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_CoverPage( 
            /* [in] */ __RPC__in BSTR bstrCoverPage) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Subject( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrSubject) = 0;
        
        virtual /* [helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_Subject( 
            /* [in] */ __RPC__in BSTR bstrSubject) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Note( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrNote) = 0;
        
        virtual /* [helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_Note( 
            /* [in] */ __RPC__in BSTR bstrNote) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_ScheduleTime( 
            /* [retval][out] */ __RPC__out DATE *pdateScheduleTime) = 0;
        
        virtual /* [helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_ScheduleTime( 
            /* [in] */ DATE dateScheduleTime) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_ReceiptAddress( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrReceiptAddress) = 0;
        
        virtual /* [helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_ReceiptAddress( 
            /* [in] */ __RPC__in BSTR bstrReceiptAddress) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_DocumentName( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrDocumentName) = 0;
        
        virtual /* [helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_DocumentName( 
            /* [in] */ __RPC__in BSTR bstrDocumentName) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_CallHandle( 
            /* [retval][out] */ __RPC__out long *plCallHandle) = 0;
        
        virtual /* [helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_CallHandle( 
            /* [in] */ long lCallHandle) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_CoverPageType( 
            /* [retval][out] */ __RPC__out FAX_COVERPAGE_TYPE_ENUM *pCoverPageType) = 0;
        
        virtual /* [helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_CoverPageType( 
            /* [in] */ FAX_COVERPAGE_TYPE_ENUM CoverPageType) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_ScheduleType( 
            /* [retval][out] */ __RPC__out FAX_SCHEDULE_TYPE_ENUM *pScheduleType) = 0;
        
        virtual /* [helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_ScheduleType( 
            /* [in] */ FAX_SCHEDULE_TYPE_ENUM ScheduleType) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_ReceiptType( 
            /* [retval][out] */ __RPC__out FAX_RECEIPT_TYPE_ENUM *pReceiptType) = 0;
        
        virtual /* [helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_ReceiptType( 
            /* [in] */ FAX_RECEIPT_TYPE_ENUM ReceiptType) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_GroupBroadcastReceipts( 
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pbUseGrouping) = 0;
        
        virtual /* [helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_GroupBroadcastReceipts( 
            /* [in] */ VARIANT_BOOL bUseGrouping) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Priority( 
            /* [retval][out] */ __RPC__out FAX_PRIORITY_TYPE_ENUM *pPriority) = 0;
        
        virtual /* [helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_Priority( 
            /* [in] */ FAX_PRIORITY_TYPE_ENUM Priority) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_TapiConnection( 
            /* [retval][out] */ __RPC__deref_out_opt IDispatch **ppTapiConnection) = 0;
        
        virtual /* [helpstring][id][propputref] */ HRESULT STDMETHODCALLTYPE putref_TapiConnection( 
            /* [in] */ __RPC__in_opt IDispatch *pTapiConnection) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE Submit( 
            /* [in] */ __RPC__in BSTR bstrFaxServerName,
            /* [retval][out] */ __RPC__out VARIANT *pvFaxOutgoingJobIDs) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE ConnectedSubmit( 
            /* [in] */ __RPC__in_opt IFaxServer *pFaxServer,
            /* [retval][out] */ __RPC__out VARIANT *pvFaxOutgoingJobIDs) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_AttachFaxToReceipt( 
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pbAttachFax) = 0;
        
        virtual /* [helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_AttachFaxToReceipt( 
            /* [in] */ VARIANT_BOOL bAttachFax) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IFaxDocumentVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IFaxDocument * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IFaxDocument * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IFaxDocument * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IFaxDocument * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IFaxDocument * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IFaxDocument * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IFaxDocument * This,
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
        
        DECLSPEC_XFGVIRT(IFaxDocument, get_Body)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Body )( 
            __RPC__in IFaxDocument * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrBody);
        
        DECLSPEC_XFGVIRT(IFaxDocument, put_Body)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_Body )( 
            __RPC__in IFaxDocument * This,
            /* [in] */ __RPC__in BSTR bstrBody);
        
        DECLSPEC_XFGVIRT(IFaxDocument, get_Sender)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Sender )( 
            __RPC__in IFaxDocument * This,
            /* [retval][out] */ __RPC__deref_out_opt IFaxSender **ppFaxSender);
        
        DECLSPEC_XFGVIRT(IFaxDocument, get_Recipients)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Recipients )( 
            __RPC__in IFaxDocument * This,
            /* [retval][out] */ __RPC__deref_out_opt IFaxRecipients **ppFaxRecipients);
        
        DECLSPEC_XFGVIRT(IFaxDocument, get_CoverPage)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_CoverPage )( 
            __RPC__in IFaxDocument * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrCoverPage);
        
        DECLSPEC_XFGVIRT(IFaxDocument, put_CoverPage)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_CoverPage )( 
            __RPC__in IFaxDocument * This,
            /* [in] */ __RPC__in BSTR bstrCoverPage);
        
        DECLSPEC_XFGVIRT(IFaxDocument, get_Subject)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Subject )( 
            __RPC__in IFaxDocument * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrSubject);
        
        DECLSPEC_XFGVIRT(IFaxDocument, put_Subject)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_Subject )( 
            __RPC__in IFaxDocument * This,
            /* [in] */ __RPC__in BSTR bstrSubject);
        
        DECLSPEC_XFGVIRT(IFaxDocument, get_Note)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Note )( 
            __RPC__in IFaxDocument * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrNote);
        
        DECLSPEC_XFGVIRT(IFaxDocument, put_Note)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_Note )( 
            __RPC__in IFaxDocument * This,
            /* [in] */ __RPC__in BSTR bstrNote);
        
        DECLSPEC_XFGVIRT(IFaxDocument, get_ScheduleTime)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_ScheduleTime )( 
            __RPC__in IFaxDocument * This,
            /* [retval][out] */ __RPC__out DATE *pdateScheduleTime);
        
        DECLSPEC_XFGVIRT(IFaxDocument, put_ScheduleTime)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_ScheduleTime )( 
            __RPC__in IFaxDocument * This,
            /* [in] */ DATE dateScheduleTime);
        
        DECLSPEC_XFGVIRT(IFaxDocument, get_ReceiptAddress)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_ReceiptAddress )( 
            __RPC__in IFaxDocument * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrReceiptAddress);
        
        DECLSPEC_XFGVIRT(IFaxDocument, put_ReceiptAddress)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_ReceiptAddress )( 
            __RPC__in IFaxDocument * This,
            /* [in] */ __RPC__in BSTR bstrReceiptAddress);
        
        DECLSPEC_XFGVIRT(IFaxDocument, get_DocumentName)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_DocumentName )( 
            __RPC__in IFaxDocument * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrDocumentName);
        
        DECLSPEC_XFGVIRT(IFaxDocument, put_DocumentName)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_DocumentName )( 
            __RPC__in IFaxDocument * This,
            /* [in] */ __RPC__in BSTR bstrDocumentName);
        
        DECLSPEC_XFGVIRT(IFaxDocument, get_CallHandle)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_CallHandle )( 
            __RPC__in IFaxDocument * This,
            /* [retval][out] */ __RPC__out long *plCallHandle);
        
        DECLSPEC_XFGVIRT(IFaxDocument, put_CallHandle)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_CallHandle )( 
            __RPC__in IFaxDocument * This,
            /* [in] */ long lCallHandle);
        
        DECLSPEC_XFGVIRT(IFaxDocument, get_CoverPageType)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_CoverPageType )( 
            __RPC__in IFaxDocument * This,
            /* [retval][out] */ __RPC__out FAX_COVERPAGE_TYPE_ENUM *pCoverPageType);
        
        DECLSPEC_XFGVIRT(IFaxDocument, put_CoverPageType)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_CoverPageType )( 
            __RPC__in IFaxDocument * This,
            /* [in] */ FAX_COVERPAGE_TYPE_ENUM CoverPageType);
        
        DECLSPEC_XFGVIRT(IFaxDocument, get_ScheduleType)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_ScheduleType )( 
            __RPC__in IFaxDocument * This,
            /* [retval][out] */ __RPC__out FAX_SCHEDULE_TYPE_ENUM *pScheduleType);
        
        DECLSPEC_XFGVIRT(IFaxDocument, put_ScheduleType)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_ScheduleType )( 
            __RPC__in IFaxDocument * This,
            /* [in] */ FAX_SCHEDULE_TYPE_ENUM ScheduleType);
        
        DECLSPEC_XFGVIRT(IFaxDocument, get_ReceiptType)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_ReceiptType )( 
            __RPC__in IFaxDocument * This,
            /* [retval][out] */ __RPC__out FAX_RECEIPT_TYPE_ENUM *pReceiptType);
        
        DECLSPEC_XFGVIRT(IFaxDocument, put_ReceiptType)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_ReceiptType )( 
            __RPC__in IFaxDocument * This,
            /* [in] */ FAX_RECEIPT_TYPE_ENUM ReceiptType);
        
        DECLSPEC_XFGVIRT(IFaxDocument, get_GroupBroadcastReceipts)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_GroupBroadcastReceipts )( 
            __RPC__in IFaxDocument * This,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pbUseGrouping);
        
        DECLSPEC_XFGVIRT(IFaxDocument, put_GroupBroadcastReceipts)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_GroupBroadcastReceipts )( 
            __RPC__in IFaxDocument * This,
            /* [in] */ VARIANT_BOOL bUseGrouping);
        
        DECLSPEC_XFGVIRT(IFaxDocument, get_Priority)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Priority )( 
            __RPC__in IFaxDocument * This,
            /* [retval][out] */ __RPC__out FAX_PRIORITY_TYPE_ENUM *pPriority);
        
        DECLSPEC_XFGVIRT(IFaxDocument, put_Priority)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_Priority )( 
            __RPC__in IFaxDocument * This,
            /* [in] */ FAX_PRIORITY_TYPE_ENUM Priority);
        
        DECLSPEC_XFGVIRT(IFaxDocument, get_TapiConnection)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_TapiConnection )( 
            __RPC__in IFaxDocument * This,
            /* [retval][out] */ __RPC__deref_out_opt IDispatch **ppTapiConnection);
        
        DECLSPEC_XFGVIRT(IFaxDocument, putref_TapiConnection)
        /* [helpstring][id][propputref] */ HRESULT ( STDMETHODCALLTYPE *putref_TapiConnection )( 
            __RPC__in IFaxDocument * This,
            /* [in] */ __RPC__in_opt IDispatch *pTapiConnection);
        
        DECLSPEC_XFGVIRT(IFaxDocument, Submit)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *Submit )( 
            __RPC__in IFaxDocument * This,
            /* [in] */ __RPC__in BSTR bstrFaxServerName,
            /* [retval][out] */ __RPC__out VARIANT *pvFaxOutgoingJobIDs);
        
        DECLSPEC_XFGVIRT(IFaxDocument, ConnectedSubmit)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *ConnectedSubmit )( 
            __RPC__in IFaxDocument * This,
            /* [in] */ __RPC__in_opt IFaxServer *pFaxServer,
            /* [retval][out] */ __RPC__out VARIANT *pvFaxOutgoingJobIDs);
        
        DECLSPEC_XFGVIRT(IFaxDocument, get_AttachFaxToReceipt)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_AttachFaxToReceipt )( 
            __RPC__in IFaxDocument * This,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pbAttachFax);
        
        DECLSPEC_XFGVIRT(IFaxDocument, put_AttachFaxToReceipt)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_AttachFaxToReceipt )( 
            __RPC__in IFaxDocument * This,
            /* [in] */ VARIANT_BOOL bAttachFax);
        
        END_INTERFACE
    } IFaxDocumentVtbl;

    interface IFaxDocument
    {
        CONST_VTBL struct IFaxDocumentVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IFaxDocument_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IFaxDocument_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IFaxDocument_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IFaxDocument_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IFaxDocument_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IFaxDocument_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IFaxDocument_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IFaxDocument_get_Body(This,pbstrBody)	\
    ( (This)->lpVtbl -> get_Body(This,pbstrBody) ) 

#define IFaxDocument_put_Body(This,bstrBody)	\
    ( (This)->lpVtbl -> put_Body(This,bstrBody) ) 

#define IFaxDocument_get_Sender(This,ppFaxSender)	\
    ( (This)->lpVtbl -> get_Sender(This,ppFaxSender) ) 

#define IFaxDocument_get_Recipients(This,ppFaxRecipients)	\
    ( (This)->lpVtbl -> get_Recipients(This,ppFaxRecipients) ) 

#define IFaxDocument_get_CoverPage(This,pbstrCoverPage)	\
    ( (This)->lpVtbl -> get_CoverPage(This,pbstrCoverPage) ) 

#define IFaxDocument_put_CoverPage(This,bstrCoverPage)	\
    ( (This)->lpVtbl -> put_CoverPage(This,bstrCoverPage) ) 

#define IFaxDocument_get_Subject(This,pbstrSubject)	\
    ( (This)->lpVtbl -> get_Subject(This,pbstrSubject) ) 

#define IFaxDocument_put_Subject(This,bstrSubject)	\
    ( (This)->lpVtbl -> put_Subject(This,bstrSubject) ) 

#define IFaxDocument_get_Note(This,pbstrNote)	\
    ( (This)->lpVtbl -> get_Note(This,pbstrNote) ) 

#define IFaxDocument_put_Note(This,bstrNote)	\
    ( (This)->lpVtbl -> put_Note(This,bstrNote) ) 

#define IFaxDocument_get_ScheduleTime(This,pdateScheduleTime)	\
    ( (This)->lpVtbl -> get_ScheduleTime(This,pdateScheduleTime) ) 

#define IFaxDocument_put_ScheduleTime(This,dateScheduleTime)	\
    ( (This)->lpVtbl -> put_ScheduleTime(This,dateScheduleTime) ) 

#define IFaxDocument_get_ReceiptAddress(This,pbstrReceiptAddress)	\
    ( (This)->lpVtbl -> get_ReceiptAddress(This,pbstrReceiptAddress) ) 

#define IFaxDocument_put_ReceiptAddress(This,bstrReceiptAddress)	\
    ( (This)->lpVtbl -> put_ReceiptAddress(This,bstrReceiptAddress) ) 

#define IFaxDocument_get_DocumentName(This,pbstrDocumentName)	\
    ( (This)->lpVtbl -> get_DocumentName(This,pbstrDocumentName) ) 

#define IFaxDocument_put_DocumentName(This,bstrDocumentName)	\
    ( (This)->lpVtbl -> put_DocumentName(This,bstrDocumentName) ) 

#define IFaxDocument_get_CallHandle(This,plCallHandle)	\
    ( (This)->lpVtbl -> get_CallHandle(This,plCallHandle) ) 

#define IFaxDocument_put_CallHandle(This,lCallHandle)	\
    ( (This)->lpVtbl -> put_CallHandle(This,lCallHandle) ) 

#define IFaxDocument_get_CoverPageType(This,pCoverPageType)	\
    ( (This)->lpVtbl -> get_CoverPageType(This,pCoverPageType) ) 

#define IFaxDocument_put_CoverPageType(This,CoverPageType)	\
    ( (This)->lpVtbl -> put_CoverPageType(This,CoverPageType) ) 

#define IFaxDocument_get_ScheduleType(This,pScheduleType)	\
    ( (This)->lpVtbl -> get_ScheduleType(This,pScheduleType) ) 

#define IFaxDocument_put_ScheduleType(This,ScheduleType)	\
    ( (This)->lpVtbl -> put_ScheduleType(This,ScheduleType) ) 

#define IFaxDocument_get_ReceiptType(This,pReceiptType)	\
    ( (This)->lpVtbl -> get_ReceiptType(This,pReceiptType) ) 

#define IFaxDocument_put_ReceiptType(This,ReceiptType)	\
    ( (This)->lpVtbl -> put_ReceiptType(This,ReceiptType) ) 

#define IFaxDocument_get_GroupBroadcastReceipts(This,pbUseGrouping)	\
    ( (This)->lpVtbl -> get_GroupBroadcastReceipts(This,pbUseGrouping) ) 

#define IFaxDocument_put_GroupBroadcastReceipts(This,bUseGrouping)	\
    ( (This)->lpVtbl -> put_GroupBroadcastReceipts(This,bUseGrouping) ) 

#define IFaxDocument_get_Priority(This,pPriority)	\
    ( (This)->lpVtbl -> get_Priority(This,pPriority) ) 

#define IFaxDocument_put_Priority(This,Priority)	\
    ( (This)->lpVtbl -> put_Priority(This,Priority) ) 

#define IFaxDocument_get_TapiConnection(This,ppTapiConnection)	\
    ( (This)->lpVtbl -> get_TapiConnection(This,ppTapiConnection) ) 

#define IFaxDocument_putref_TapiConnection(This,pTapiConnection)	\
    ( (This)->lpVtbl -> putref_TapiConnection(This,pTapiConnection) ) 

#define IFaxDocument_Submit(This,bstrFaxServerName,pvFaxOutgoingJobIDs)	\
    ( (This)->lpVtbl -> Submit(This,bstrFaxServerName,pvFaxOutgoingJobIDs) ) 

#define IFaxDocument_ConnectedSubmit(This,pFaxServer,pvFaxOutgoingJobIDs)	\
    ( (This)->lpVtbl -> ConnectedSubmit(This,pFaxServer,pvFaxOutgoingJobIDs) ) 

#define IFaxDocument_get_AttachFaxToReceipt(This,pbAttachFax)	\
    ( (This)->lpVtbl -> get_AttachFaxToReceipt(This,pbAttachFax) ) 

#define IFaxDocument_put_AttachFaxToReceipt(This,bAttachFax)	\
    ( (This)->lpVtbl -> put_AttachFaxToReceipt(This,bAttachFax) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IFaxDocument_INTERFACE_DEFINED__ */


#ifndef __IFaxSender_INTERFACE_DEFINED__
#define __IFaxSender_INTERFACE_DEFINED__

/* interface IFaxSender */
/* [nonextensible][unique][helpstring][dual][uuid][object] */ 


EXTERN_C const IID IID_IFaxSender;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("0D879D7D-F57A-4CC6-A6F9-3EE5D527B46A")
    IFaxSender : public IDispatch
    {
    public:
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_BillingCode( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrBillingCode) = 0;
        
        virtual /* [helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_BillingCode( 
            /* [in] */ __RPC__in BSTR bstrBillingCode) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_City( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrCity) = 0;
        
        virtual /* [helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_City( 
            /* [in] */ __RPC__in BSTR bstrCity) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Company( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrCompany) = 0;
        
        virtual /* [helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_Company( 
            /* [in] */ __RPC__in BSTR bstrCompany) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Country( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrCountry) = 0;
        
        virtual /* [helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_Country( 
            /* [in] */ __RPC__in BSTR bstrCountry) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Department( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrDepartment) = 0;
        
        virtual /* [helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_Department( 
            /* [in] */ __RPC__in BSTR bstrDepartment) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Email( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrEmail) = 0;
        
        virtual /* [helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_Email( 
            /* [in] */ __RPC__in BSTR bstrEmail) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_FaxNumber( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrFaxNumber) = 0;
        
        virtual /* [helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_FaxNumber( 
            /* [in] */ __RPC__in BSTR bstrFaxNumber) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_HomePhone( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrHomePhone) = 0;
        
        virtual /* [helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_HomePhone( 
            /* [in] */ __RPC__in BSTR bstrHomePhone) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Name( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrName) = 0;
        
        virtual /* [helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_Name( 
            /* [in] */ __RPC__in BSTR bstrName) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_TSID( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrTSID) = 0;
        
        virtual /* [helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_TSID( 
            /* [in] */ __RPC__in BSTR bstrTSID) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_OfficePhone( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrOfficePhone) = 0;
        
        virtual /* [helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_OfficePhone( 
            /* [in] */ __RPC__in BSTR bstrOfficePhone) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_OfficeLocation( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrOfficeLocation) = 0;
        
        virtual /* [helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_OfficeLocation( 
            /* [in] */ __RPC__in BSTR bstrOfficeLocation) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_State( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrState) = 0;
        
        virtual /* [helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_State( 
            /* [in] */ __RPC__in BSTR bstrState) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_StreetAddress( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrStreetAddress) = 0;
        
        virtual /* [helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_StreetAddress( 
            /* [in] */ __RPC__in BSTR bstrStreetAddress) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Title( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrTitle) = 0;
        
        virtual /* [helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_Title( 
            /* [in] */ __RPC__in BSTR bstrTitle) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_ZipCode( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrZipCode) = 0;
        
        virtual /* [helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_ZipCode( 
            /* [in] */ __RPC__in BSTR bstrZipCode) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE LoadDefaultSender( void) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE SaveDefaultSender( void) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IFaxSenderVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IFaxSender * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IFaxSender * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IFaxSender * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IFaxSender * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IFaxSender * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IFaxSender * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IFaxSender * This,
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
        
        DECLSPEC_XFGVIRT(IFaxSender, get_BillingCode)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_BillingCode )( 
            __RPC__in IFaxSender * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrBillingCode);
        
        DECLSPEC_XFGVIRT(IFaxSender, put_BillingCode)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_BillingCode )( 
            __RPC__in IFaxSender * This,
            /* [in] */ __RPC__in BSTR bstrBillingCode);
        
        DECLSPEC_XFGVIRT(IFaxSender, get_City)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_City )( 
            __RPC__in IFaxSender * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrCity);
        
        DECLSPEC_XFGVIRT(IFaxSender, put_City)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_City )( 
            __RPC__in IFaxSender * This,
            /* [in] */ __RPC__in BSTR bstrCity);
        
        DECLSPEC_XFGVIRT(IFaxSender, get_Company)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Company )( 
            __RPC__in IFaxSender * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrCompany);
        
        DECLSPEC_XFGVIRT(IFaxSender, put_Company)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_Company )( 
            __RPC__in IFaxSender * This,
            /* [in] */ __RPC__in BSTR bstrCompany);
        
        DECLSPEC_XFGVIRT(IFaxSender, get_Country)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Country )( 
            __RPC__in IFaxSender * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrCountry);
        
        DECLSPEC_XFGVIRT(IFaxSender, put_Country)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_Country )( 
            __RPC__in IFaxSender * This,
            /* [in] */ __RPC__in BSTR bstrCountry);
        
        DECLSPEC_XFGVIRT(IFaxSender, get_Department)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Department )( 
            __RPC__in IFaxSender * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrDepartment);
        
        DECLSPEC_XFGVIRT(IFaxSender, put_Department)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_Department )( 
            __RPC__in IFaxSender * This,
            /* [in] */ __RPC__in BSTR bstrDepartment);
        
        DECLSPEC_XFGVIRT(IFaxSender, get_Email)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Email )( 
            __RPC__in IFaxSender * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrEmail);
        
        DECLSPEC_XFGVIRT(IFaxSender, put_Email)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_Email )( 
            __RPC__in IFaxSender * This,
            /* [in] */ __RPC__in BSTR bstrEmail);
        
        DECLSPEC_XFGVIRT(IFaxSender, get_FaxNumber)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_FaxNumber )( 
            __RPC__in IFaxSender * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrFaxNumber);
        
        DECLSPEC_XFGVIRT(IFaxSender, put_FaxNumber)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_FaxNumber )( 
            __RPC__in IFaxSender * This,
            /* [in] */ __RPC__in BSTR bstrFaxNumber);
        
        DECLSPEC_XFGVIRT(IFaxSender, get_HomePhone)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_HomePhone )( 
            __RPC__in IFaxSender * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrHomePhone);
        
        DECLSPEC_XFGVIRT(IFaxSender, put_HomePhone)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_HomePhone )( 
            __RPC__in IFaxSender * This,
            /* [in] */ __RPC__in BSTR bstrHomePhone);
        
        DECLSPEC_XFGVIRT(IFaxSender, get_Name)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Name )( 
            __RPC__in IFaxSender * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrName);
        
        DECLSPEC_XFGVIRT(IFaxSender, put_Name)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_Name )( 
            __RPC__in IFaxSender * This,
            /* [in] */ __RPC__in BSTR bstrName);
        
        DECLSPEC_XFGVIRT(IFaxSender, get_TSID)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_TSID )( 
            __RPC__in IFaxSender * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrTSID);
        
        DECLSPEC_XFGVIRT(IFaxSender, put_TSID)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_TSID )( 
            __RPC__in IFaxSender * This,
            /* [in] */ __RPC__in BSTR bstrTSID);
        
        DECLSPEC_XFGVIRT(IFaxSender, get_OfficePhone)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_OfficePhone )( 
            __RPC__in IFaxSender * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrOfficePhone);
        
        DECLSPEC_XFGVIRT(IFaxSender, put_OfficePhone)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_OfficePhone )( 
            __RPC__in IFaxSender * This,
            /* [in] */ __RPC__in BSTR bstrOfficePhone);
        
        DECLSPEC_XFGVIRT(IFaxSender, get_OfficeLocation)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_OfficeLocation )( 
            __RPC__in IFaxSender * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrOfficeLocation);
        
        DECLSPEC_XFGVIRT(IFaxSender, put_OfficeLocation)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_OfficeLocation )( 
            __RPC__in IFaxSender * This,
            /* [in] */ __RPC__in BSTR bstrOfficeLocation);
        
        DECLSPEC_XFGVIRT(IFaxSender, get_State)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_State )( 
            __RPC__in IFaxSender * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrState);
        
        DECLSPEC_XFGVIRT(IFaxSender, put_State)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_State )( 
            __RPC__in IFaxSender * This,
            /* [in] */ __RPC__in BSTR bstrState);
        
        DECLSPEC_XFGVIRT(IFaxSender, get_StreetAddress)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_StreetAddress )( 
            __RPC__in IFaxSender * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrStreetAddress);
        
        DECLSPEC_XFGVIRT(IFaxSender, put_StreetAddress)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_StreetAddress )( 
            __RPC__in IFaxSender * This,
            /* [in] */ __RPC__in BSTR bstrStreetAddress);
        
        DECLSPEC_XFGVIRT(IFaxSender, get_Title)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Title )( 
            __RPC__in IFaxSender * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrTitle);
        
        DECLSPEC_XFGVIRT(IFaxSender, put_Title)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_Title )( 
            __RPC__in IFaxSender * This,
            /* [in] */ __RPC__in BSTR bstrTitle);
        
        DECLSPEC_XFGVIRT(IFaxSender, get_ZipCode)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_ZipCode )( 
            __RPC__in IFaxSender * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrZipCode);
        
        DECLSPEC_XFGVIRT(IFaxSender, put_ZipCode)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_ZipCode )( 
            __RPC__in IFaxSender * This,
            /* [in] */ __RPC__in BSTR bstrZipCode);
        
        DECLSPEC_XFGVIRT(IFaxSender, LoadDefaultSender)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *LoadDefaultSender )( 
            __RPC__in IFaxSender * This);
        
        DECLSPEC_XFGVIRT(IFaxSender, SaveDefaultSender)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *SaveDefaultSender )( 
            __RPC__in IFaxSender * This);
        
        END_INTERFACE
    } IFaxSenderVtbl;

    interface IFaxSender
    {
        CONST_VTBL struct IFaxSenderVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IFaxSender_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IFaxSender_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IFaxSender_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IFaxSender_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IFaxSender_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IFaxSender_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IFaxSender_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IFaxSender_get_BillingCode(This,pbstrBillingCode)	\
    ( (This)->lpVtbl -> get_BillingCode(This,pbstrBillingCode) ) 

#define IFaxSender_put_BillingCode(This,bstrBillingCode)	\
    ( (This)->lpVtbl -> put_BillingCode(This,bstrBillingCode) ) 

#define IFaxSender_get_City(This,pbstrCity)	\
    ( (This)->lpVtbl -> get_City(This,pbstrCity) ) 

#define IFaxSender_put_City(This,bstrCity)	\
    ( (This)->lpVtbl -> put_City(This,bstrCity) ) 

#define IFaxSender_get_Company(This,pbstrCompany)	\
    ( (This)->lpVtbl -> get_Company(This,pbstrCompany) ) 

#define IFaxSender_put_Company(This,bstrCompany)	\
    ( (This)->lpVtbl -> put_Company(This,bstrCompany) ) 

#define IFaxSender_get_Country(This,pbstrCountry)	\
    ( (This)->lpVtbl -> get_Country(This,pbstrCountry) ) 

#define IFaxSender_put_Country(This,bstrCountry)	\
    ( (This)->lpVtbl -> put_Country(This,bstrCountry) ) 

#define IFaxSender_get_Department(This,pbstrDepartment)	\
    ( (This)->lpVtbl -> get_Department(This,pbstrDepartment) ) 

#define IFaxSender_put_Department(This,bstrDepartment)	\
    ( (This)->lpVtbl -> put_Department(This,bstrDepartment) ) 

#define IFaxSender_get_Email(This,pbstrEmail)	\
    ( (This)->lpVtbl -> get_Email(This,pbstrEmail) ) 

#define IFaxSender_put_Email(This,bstrEmail)	\
    ( (This)->lpVtbl -> put_Email(This,bstrEmail) ) 

#define IFaxSender_get_FaxNumber(This,pbstrFaxNumber)	\
    ( (This)->lpVtbl -> get_FaxNumber(This,pbstrFaxNumber) ) 

#define IFaxSender_put_FaxNumber(This,bstrFaxNumber)	\
    ( (This)->lpVtbl -> put_FaxNumber(This,bstrFaxNumber) ) 

#define IFaxSender_get_HomePhone(This,pbstrHomePhone)	\
    ( (This)->lpVtbl -> get_HomePhone(This,pbstrHomePhone) ) 

#define IFaxSender_put_HomePhone(This,bstrHomePhone)	\
    ( (This)->lpVtbl -> put_HomePhone(This,bstrHomePhone) ) 

#define IFaxSender_get_Name(This,pbstrName)	\
    ( (This)->lpVtbl -> get_Name(This,pbstrName) ) 

#define IFaxSender_put_Name(This,bstrName)	\
    ( (This)->lpVtbl -> put_Name(This,bstrName) ) 

#define IFaxSender_get_TSID(This,pbstrTSID)	\
    ( (This)->lpVtbl -> get_TSID(This,pbstrTSID) ) 

#define IFaxSender_put_TSID(This,bstrTSID)	\
    ( (This)->lpVtbl -> put_TSID(This,bstrTSID) ) 

#define IFaxSender_get_OfficePhone(This,pbstrOfficePhone)	\
    ( (This)->lpVtbl -> get_OfficePhone(This,pbstrOfficePhone) ) 

#define IFaxSender_put_OfficePhone(This,bstrOfficePhone)	\
    ( (This)->lpVtbl -> put_OfficePhone(This,bstrOfficePhone) ) 

#define IFaxSender_get_OfficeLocation(This,pbstrOfficeLocation)	\
    ( (This)->lpVtbl -> get_OfficeLocation(This,pbstrOfficeLocation) ) 

#define IFaxSender_put_OfficeLocation(This,bstrOfficeLocation)	\
    ( (This)->lpVtbl -> put_OfficeLocation(This,bstrOfficeLocation) ) 

#define IFaxSender_get_State(This,pbstrState)	\
    ( (This)->lpVtbl -> get_State(This,pbstrState) ) 

#define IFaxSender_put_State(This,bstrState)	\
    ( (This)->lpVtbl -> put_State(This,bstrState) ) 

#define IFaxSender_get_StreetAddress(This,pbstrStreetAddress)	\
    ( (This)->lpVtbl -> get_StreetAddress(This,pbstrStreetAddress) ) 

#define IFaxSender_put_StreetAddress(This,bstrStreetAddress)	\
    ( (This)->lpVtbl -> put_StreetAddress(This,bstrStreetAddress) ) 

#define IFaxSender_get_Title(This,pbstrTitle)	\
    ( (This)->lpVtbl -> get_Title(This,pbstrTitle) ) 

#define IFaxSender_put_Title(This,bstrTitle)	\
    ( (This)->lpVtbl -> put_Title(This,bstrTitle) ) 

#define IFaxSender_get_ZipCode(This,pbstrZipCode)	\
    ( (This)->lpVtbl -> get_ZipCode(This,pbstrZipCode) ) 

#define IFaxSender_put_ZipCode(This,bstrZipCode)	\
    ( (This)->lpVtbl -> put_ZipCode(This,bstrZipCode) ) 

#define IFaxSender_LoadDefaultSender(This)	\
    ( (This)->lpVtbl -> LoadDefaultSender(This) ) 

#define IFaxSender_SaveDefaultSender(This)	\
    ( (This)->lpVtbl -> SaveDefaultSender(This) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IFaxSender_INTERFACE_DEFINED__ */


#ifndef __IFaxRecipient_INTERFACE_DEFINED__
#define __IFaxRecipient_INTERFACE_DEFINED__

/* interface IFaxRecipient */
/* [nonextensible][unique][helpstring][dual][uuid][object] */ 


EXTERN_C const IID IID_IFaxRecipient;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("9A3DA3A0-538D-42b6-9444-AAA57D0CE2BC")
    IFaxRecipient : public IDispatch
    {
    public:
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_FaxNumber( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrFaxNumber) = 0;
        
        virtual /* [helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_FaxNumber( 
            /* [in] */ __RPC__in BSTR bstrFaxNumber) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Name( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrName) = 0;
        
        virtual /* [helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_Name( 
            /* [in] */ __RPC__in BSTR bstrName) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IFaxRecipientVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IFaxRecipient * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IFaxRecipient * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IFaxRecipient * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IFaxRecipient * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IFaxRecipient * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IFaxRecipient * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IFaxRecipient * This,
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
        
        DECLSPEC_XFGVIRT(IFaxRecipient, get_FaxNumber)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_FaxNumber )( 
            __RPC__in IFaxRecipient * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrFaxNumber);
        
        DECLSPEC_XFGVIRT(IFaxRecipient, put_FaxNumber)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_FaxNumber )( 
            __RPC__in IFaxRecipient * This,
            /* [in] */ __RPC__in BSTR bstrFaxNumber);
        
        DECLSPEC_XFGVIRT(IFaxRecipient, get_Name)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Name )( 
            __RPC__in IFaxRecipient * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrName);
        
        DECLSPEC_XFGVIRT(IFaxRecipient, put_Name)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_Name )( 
            __RPC__in IFaxRecipient * This,
            /* [in] */ __RPC__in BSTR bstrName);
        
        END_INTERFACE
    } IFaxRecipientVtbl;

    interface IFaxRecipient
    {
        CONST_VTBL struct IFaxRecipientVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IFaxRecipient_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IFaxRecipient_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IFaxRecipient_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IFaxRecipient_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IFaxRecipient_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IFaxRecipient_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IFaxRecipient_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IFaxRecipient_get_FaxNumber(This,pbstrFaxNumber)	\
    ( (This)->lpVtbl -> get_FaxNumber(This,pbstrFaxNumber) ) 

#define IFaxRecipient_put_FaxNumber(This,bstrFaxNumber)	\
    ( (This)->lpVtbl -> put_FaxNumber(This,bstrFaxNumber) ) 

#define IFaxRecipient_get_Name(This,pbstrName)	\
    ( (This)->lpVtbl -> get_Name(This,pbstrName) ) 

#define IFaxRecipient_put_Name(This,bstrName)	\
    ( (This)->lpVtbl -> put_Name(This,bstrName) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IFaxRecipient_INTERFACE_DEFINED__ */


#ifndef __IFaxRecipients_INTERFACE_DEFINED__
#define __IFaxRecipients_INTERFACE_DEFINED__

/* interface IFaxRecipients */
/* [nonextensible][unique][helpstring][dual][uuid][object] */ 


EXTERN_C const IID IID_IFaxRecipients;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("B9C9DE5A-894E-4492-9FA3-08C627C11D5D")
    IFaxRecipients : public IDispatch
    {
    public:
        virtual /* [propget][id] */ HRESULT STDMETHODCALLTYPE get__NewEnum( 
            /* [retval][out] */ __RPC__deref_out_opt IUnknown **ppUnk) = 0;
        
        virtual /* [propget][id] */ HRESULT STDMETHODCALLTYPE get_Item( 
            /* [in] */ long lIndex,
            /* [retval][out] */ __RPC__deref_out_opt IFaxRecipient **ppFaxRecipient) = 0;
        
        virtual /* [propget][id] */ HRESULT STDMETHODCALLTYPE get_Count( 
            /* [retval][out] */ __RPC__out long *plCount) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE Add( 
            /* [in] */ __RPC__in BSTR bstrFaxNumber,
            /* [defaultvalue][in] */ __RPC__in BSTR bstrRecipientName,
            /* [retval][out] */ __RPC__deref_out_opt IFaxRecipient **ppFaxRecipient) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE Remove( 
            /* [in] */ long lIndex) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IFaxRecipientsVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IFaxRecipients * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IFaxRecipients * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IFaxRecipients * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IFaxRecipients * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IFaxRecipients * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IFaxRecipients * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IFaxRecipients * This,
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
        
        DECLSPEC_XFGVIRT(IFaxRecipients, get__NewEnum)
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *get__NewEnum )( 
            __RPC__in IFaxRecipients * This,
            /* [retval][out] */ __RPC__deref_out_opt IUnknown **ppUnk);
        
        DECLSPEC_XFGVIRT(IFaxRecipients, get_Item)
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_Item )( 
            __RPC__in IFaxRecipients * This,
            /* [in] */ long lIndex,
            /* [retval][out] */ __RPC__deref_out_opt IFaxRecipient **ppFaxRecipient);
        
        DECLSPEC_XFGVIRT(IFaxRecipients, get_Count)
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_Count )( 
            __RPC__in IFaxRecipients * This,
            /* [retval][out] */ __RPC__out long *plCount);
        
        DECLSPEC_XFGVIRT(IFaxRecipients, Add)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *Add )( 
            __RPC__in IFaxRecipients * This,
            /* [in] */ __RPC__in BSTR bstrFaxNumber,
            /* [defaultvalue][in] */ __RPC__in BSTR bstrRecipientName,
            /* [retval][out] */ __RPC__deref_out_opt IFaxRecipient **ppFaxRecipient);
        
        DECLSPEC_XFGVIRT(IFaxRecipients, Remove)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *Remove )( 
            __RPC__in IFaxRecipients * This,
            /* [in] */ long lIndex);
        
        END_INTERFACE
    } IFaxRecipientsVtbl;

    interface IFaxRecipients
    {
        CONST_VTBL struct IFaxRecipientsVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IFaxRecipients_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IFaxRecipients_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IFaxRecipients_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IFaxRecipients_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IFaxRecipients_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IFaxRecipients_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IFaxRecipients_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IFaxRecipients_get__NewEnum(This,ppUnk)	\
    ( (This)->lpVtbl -> get__NewEnum(This,ppUnk) ) 

#define IFaxRecipients_get_Item(This,lIndex,ppFaxRecipient)	\
    ( (This)->lpVtbl -> get_Item(This,lIndex,ppFaxRecipient) ) 

#define IFaxRecipients_get_Count(This,plCount)	\
    ( (This)->lpVtbl -> get_Count(This,plCount) ) 

#define IFaxRecipients_Add(This,bstrFaxNumber,bstrRecipientName,ppFaxRecipient)	\
    ( (This)->lpVtbl -> Add(This,bstrFaxNumber,bstrRecipientName,ppFaxRecipient) ) 

#define IFaxRecipients_Remove(This,lIndex)	\
    ( (This)->lpVtbl -> Remove(This,lIndex) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IFaxRecipients_INTERFACE_DEFINED__ */


#ifndef __IFaxIncomingArchive_INTERFACE_DEFINED__
#define __IFaxIncomingArchive_INTERFACE_DEFINED__

/* interface IFaxIncomingArchive */
/* [unique][helpstring][dual][uuid][object] */ 


EXTERN_C const IID IID_IFaxIncomingArchive;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("76062CC7-F714-4FBD-AA06-ED6E4A4B70F3")
    IFaxIncomingArchive : public IDispatch
    {
    public:
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_UseArchive( 
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pbUseArchive) = 0;
        
        virtual /* [helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_UseArchive( 
            /* [in] */ VARIANT_BOOL bUseArchive) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_ArchiveFolder( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrArchiveFolder) = 0;
        
        virtual /* [helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_ArchiveFolder( 
            /* [in] */ __RPC__in BSTR bstrArchiveFolder) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_SizeQuotaWarning( 
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pbSizeQuotaWarning) = 0;
        
        virtual /* [helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_SizeQuotaWarning( 
            /* [in] */ VARIANT_BOOL bSizeQuotaWarning) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_HighQuotaWaterMark( 
            /* [retval][out] */ __RPC__out long *plHighQuotaWaterMark) = 0;
        
        virtual /* [helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_HighQuotaWaterMark( 
            /* [in] */ long lHighQuotaWaterMark) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_LowQuotaWaterMark( 
            /* [retval][out] */ __RPC__out long *plLowQuotaWaterMark) = 0;
        
        virtual /* [helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_LowQuotaWaterMark( 
            /* [in] */ long lLowQuotaWaterMark) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_AgeLimit( 
            /* [retval][out] */ __RPC__out long *plAgeLimit) = 0;
        
        virtual /* [helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_AgeLimit( 
            /* [in] */ long lAgeLimit) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_SizeLow( 
            /* [retval][out] */ __RPC__out long *plSizeLow) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_SizeHigh( 
            /* [retval][out] */ __RPC__out long *plSizeHigh) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE Refresh( void) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE Save( void) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE GetMessages( 
            /* [defaultvalue][in] */ long lPrefetchSize,
            /* [retval][out] */ __RPC__deref_out_opt IFaxIncomingMessageIterator **pFaxIncomingMessageIterator) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE GetMessage( 
            /* [in] */ __RPC__in BSTR bstrMessageId,
            /* [retval][out] */ __RPC__deref_out_opt IFaxIncomingMessage **pFaxIncomingMessage) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IFaxIncomingArchiveVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IFaxIncomingArchive * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IFaxIncomingArchive * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IFaxIncomingArchive * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IFaxIncomingArchive * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IFaxIncomingArchive * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IFaxIncomingArchive * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IFaxIncomingArchive * This,
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
        
        DECLSPEC_XFGVIRT(IFaxIncomingArchive, get_UseArchive)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_UseArchive )( 
            __RPC__in IFaxIncomingArchive * This,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pbUseArchive);
        
        DECLSPEC_XFGVIRT(IFaxIncomingArchive, put_UseArchive)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_UseArchive )( 
            __RPC__in IFaxIncomingArchive * This,
            /* [in] */ VARIANT_BOOL bUseArchive);
        
        DECLSPEC_XFGVIRT(IFaxIncomingArchive, get_ArchiveFolder)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_ArchiveFolder )( 
            __RPC__in IFaxIncomingArchive * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrArchiveFolder);
        
        DECLSPEC_XFGVIRT(IFaxIncomingArchive, put_ArchiveFolder)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_ArchiveFolder )( 
            __RPC__in IFaxIncomingArchive * This,
            /* [in] */ __RPC__in BSTR bstrArchiveFolder);
        
        DECLSPEC_XFGVIRT(IFaxIncomingArchive, get_SizeQuotaWarning)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_SizeQuotaWarning )( 
            __RPC__in IFaxIncomingArchive * This,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pbSizeQuotaWarning);
        
        DECLSPEC_XFGVIRT(IFaxIncomingArchive, put_SizeQuotaWarning)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_SizeQuotaWarning )( 
            __RPC__in IFaxIncomingArchive * This,
            /* [in] */ VARIANT_BOOL bSizeQuotaWarning);
        
        DECLSPEC_XFGVIRT(IFaxIncomingArchive, get_HighQuotaWaterMark)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_HighQuotaWaterMark )( 
            __RPC__in IFaxIncomingArchive * This,
            /* [retval][out] */ __RPC__out long *plHighQuotaWaterMark);
        
        DECLSPEC_XFGVIRT(IFaxIncomingArchive, put_HighQuotaWaterMark)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_HighQuotaWaterMark )( 
            __RPC__in IFaxIncomingArchive * This,
            /* [in] */ long lHighQuotaWaterMark);
        
        DECLSPEC_XFGVIRT(IFaxIncomingArchive, get_LowQuotaWaterMark)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_LowQuotaWaterMark )( 
            __RPC__in IFaxIncomingArchive * This,
            /* [retval][out] */ __RPC__out long *plLowQuotaWaterMark);
        
        DECLSPEC_XFGVIRT(IFaxIncomingArchive, put_LowQuotaWaterMark)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_LowQuotaWaterMark )( 
            __RPC__in IFaxIncomingArchive * This,
            /* [in] */ long lLowQuotaWaterMark);
        
        DECLSPEC_XFGVIRT(IFaxIncomingArchive, get_AgeLimit)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_AgeLimit )( 
            __RPC__in IFaxIncomingArchive * This,
            /* [retval][out] */ __RPC__out long *plAgeLimit);
        
        DECLSPEC_XFGVIRT(IFaxIncomingArchive, put_AgeLimit)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_AgeLimit )( 
            __RPC__in IFaxIncomingArchive * This,
            /* [in] */ long lAgeLimit);
        
        DECLSPEC_XFGVIRT(IFaxIncomingArchive, get_SizeLow)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_SizeLow )( 
            __RPC__in IFaxIncomingArchive * This,
            /* [retval][out] */ __RPC__out long *plSizeLow);
        
        DECLSPEC_XFGVIRT(IFaxIncomingArchive, get_SizeHigh)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_SizeHigh )( 
            __RPC__in IFaxIncomingArchive * This,
            /* [retval][out] */ __RPC__out long *plSizeHigh);
        
        DECLSPEC_XFGVIRT(IFaxIncomingArchive, Refresh)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *Refresh )( 
            __RPC__in IFaxIncomingArchive * This);
        
        DECLSPEC_XFGVIRT(IFaxIncomingArchive, Save)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *Save )( 
            __RPC__in IFaxIncomingArchive * This);
        
        DECLSPEC_XFGVIRT(IFaxIncomingArchive, GetMessages)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *GetMessages )( 
            __RPC__in IFaxIncomingArchive * This,
            /* [defaultvalue][in] */ long lPrefetchSize,
            /* [retval][out] */ __RPC__deref_out_opt IFaxIncomingMessageIterator **pFaxIncomingMessageIterator);
        
        DECLSPEC_XFGVIRT(IFaxIncomingArchive, GetMessage)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *GetMessage )( 
            __RPC__in IFaxIncomingArchive * This,
            /* [in] */ __RPC__in BSTR bstrMessageId,
            /* [retval][out] */ __RPC__deref_out_opt IFaxIncomingMessage **pFaxIncomingMessage);
        
        END_INTERFACE
    } IFaxIncomingArchiveVtbl;

    interface IFaxIncomingArchive
    {
        CONST_VTBL struct IFaxIncomingArchiveVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IFaxIncomingArchive_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IFaxIncomingArchive_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IFaxIncomingArchive_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IFaxIncomingArchive_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IFaxIncomingArchive_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IFaxIncomingArchive_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IFaxIncomingArchive_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IFaxIncomingArchive_get_UseArchive(This,pbUseArchive)	\
    ( (This)->lpVtbl -> get_UseArchive(This,pbUseArchive) ) 

#define IFaxIncomingArchive_put_UseArchive(This,bUseArchive)	\
    ( (This)->lpVtbl -> put_UseArchive(This,bUseArchive) ) 

#define IFaxIncomingArchive_get_ArchiveFolder(This,pbstrArchiveFolder)	\
    ( (This)->lpVtbl -> get_ArchiveFolder(This,pbstrArchiveFolder) ) 

#define IFaxIncomingArchive_put_ArchiveFolder(This,bstrArchiveFolder)	\
    ( (This)->lpVtbl -> put_ArchiveFolder(This,bstrArchiveFolder) ) 

#define IFaxIncomingArchive_get_SizeQuotaWarning(This,pbSizeQuotaWarning)	\
    ( (This)->lpVtbl -> get_SizeQuotaWarning(This,pbSizeQuotaWarning) ) 

#define IFaxIncomingArchive_put_SizeQuotaWarning(This,bSizeQuotaWarning)	\
    ( (This)->lpVtbl -> put_SizeQuotaWarning(This,bSizeQuotaWarning) ) 

#define IFaxIncomingArchive_get_HighQuotaWaterMark(This,plHighQuotaWaterMark)	\
    ( (This)->lpVtbl -> get_HighQuotaWaterMark(This,plHighQuotaWaterMark) ) 

#define IFaxIncomingArchive_put_HighQuotaWaterMark(This,lHighQuotaWaterMark)	\
    ( (This)->lpVtbl -> put_HighQuotaWaterMark(This,lHighQuotaWaterMark) ) 

#define IFaxIncomingArchive_get_LowQuotaWaterMark(This,plLowQuotaWaterMark)	\
    ( (This)->lpVtbl -> get_LowQuotaWaterMark(This,plLowQuotaWaterMark) ) 

#define IFaxIncomingArchive_put_LowQuotaWaterMark(This,lLowQuotaWaterMark)	\
    ( (This)->lpVtbl -> put_LowQuotaWaterMark(This,lLowQuotaWaterMark) ) 

#define IFaxIncomingArchive_get_AgeLimit(This,plAgeLimit)	\
    ( (This)->lpVtbl -> get_AgeLimit(This,plAgeLimit) ) 

#define IFaxIncomingArchive_put_AgeLimit(This,lAgeLimit)	\
    ( (This)->lpVtbl -> put_AgeLimit(This,lAgeLimit) ) 

#define IFaxIncomingArchive_get_SizeLow(This,plSizeLow)	\
    ( (This)->lpVtbl -> get_SizeLow(This,plSizeLow) ) 

#define IFaxIncomingArchive_get_SizeHigh(This,plSizeHigh)	\
    ( (This)->lpVtbl -> get_SizeHigh(This,plSizeHigh) ) 

#define IFaxIncomingArchive_Refresh(This)	\
    ( (This)->lpVtbl -> Refresh(This) ) 

#define IFaxIncomingArchive_Save(This)	\
    ( (This)->lpVtbl -> Save(This) ) 

#define IFaxIncomingArchive_GetMessages(This,lPrefetchSize,pFaxIncomingMessageIterator)	\
    ( (This)->lpVtbl -> GetMessages(This,lPrefetchSize,pFaxIncomingMessageIterator) ) 

#define IFaxIncomingArchive_GetMessage(This,bstrMessageId,pFaxIncomingMessage)	\
    ( (This)->lpVtbl -> GetMessage(This,bstrMessageId,pFaxIncomingMessage) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IFaxIncomingArchive_INTERFACE_DEFINED__ */


#ifndef __IFaxIncomingQueue_INTERFACE_DEFINED__
#define __IFaxIncomingQueue_INTERFACE_DEFINED__

/* interface IFaxIncomingQueue */
/* [unique][helpstring][dual][uuid][object] */ 


EXTERN_C const IID IID_IFaxIncomingQueue;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("902E64EF-8FD8-4B75-9725-6014DF161545")
    IFaxIncomingQueue : public IDispatch
    {
    public:
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Blocked( 
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pbBlocked) = 0;
        
        virtual /* [helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_Blocked( 
            /* [in] */ VARIANT_BOOL bBlocked) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE Refresh( void) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE Save( void) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE GetJobs( 
            /* [retval][out] */ __RPC__deref_out_opt IFaxIncomingJobs **pFaxIncomingJobs) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE GetJob( 
            /* [in] */ __RPC__in BSTR bstrJobId,
            /* [retval][out] */ __RPC__deref_out_opt IFaxIncomingJob **pFaxIncomingJob) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IFaxIncomingQueueVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IFaxIncomingQueue * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IFaxIncomingQueue * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IFaxIncomingQueue * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IFaxIncomingQueue * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IFaxIncomingQueue * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IFaxIncomingQueue * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IFaxIncomingQueue * This,
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
        
        DECLSPEC_XFGVIRT(IFaxIncomingQueue, get_Blocked)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Blocked )( 
            __RPC__in IFaxIncomingQueue * This,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pbBlocked);
        
        DECLSPEC_XFGVIRT(IFaxIncomingQueue, put_Blocked)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_Blocked )( 
            __RPC__in IFaxIncomingQueue * This,
            /* [in] */ VARIANT_BOOL bBlocked);
        
        DECLSPEC_XFGVIRT(IFaxIncomingQueue, Refresh)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *Refresh )( 
            __RPC__in IFaxIncomingQueue * This);
        
        DECLSPEC_XFGVIRT(IFaxIncomingQueue, Save)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *Save )( 
            __RPC__in IFaxIncomingQueue * This);
        
        DECLSPEC_XFGVIRT(IFaxIncomingQueue, GetJobs)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *GetJobs )( 
            __RPC__in IFaxIncomingQueue * This,
            /* [retval][out] */ __RPC__deref_out_opt IFaxIncomingJobs **pFaxIncomingJobs);
        
        DECLSPEC_XFGVIRT(IFaxIncomingQueue, GetJob)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *GetJob )( 
            __RPC__in IFaxIncomingQueue * This,
            /* [in] */ __RPC__in BSTR bstrJobId,
            /* [retval][out] */ __RPC__deref_out_opt IFaxIncomingJob **pFaxIncomingJob);
        
        END_INTERFACE
    } IFaxIncomingQueueVtbl;

    interface IFaxIncomingQueue
    {
        CONST_VTBL struct IFaxIncomingQueueVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IFaxIncomingQueue_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IFaxIncomingQueue_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IFaxIncomingQueue_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IFaxIncomingQueue_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IFaxIncomingQueue_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IFaxIncomingQueue_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IFaxIncomingQueue_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IFaxIncomingQueue_get_Blocked(This,pbBlocked)	\
    ( (This)->lpVtbl -> get_Blocked(This,pbBlocked) ) 

#define IFaxIncomingQueue_put_Blocked(This,bBlocked)	\
    ( (This)->lpVtbl -> put_Blocked(This,bBlocked) ) 

#define IFaxIncomingQueue_Refresh(This)	\
    ( (This)->lpVtbl -> Refresh(This) ) 

#define IFaxIncomingQueue_Save(This)	\
    ( (This)->lpVtbl -> Save(This) ) 

#define IFaxIncomingQueue_GetJobs(This,pFaxIncomingJobs)	\
    ( (This)->lpVtbl -> GetJobs(This,pFaxIncomingJobs) ) 

#define IFaxIncomingQueue_GetJob(This,bstrJobId,pFaxIncomingJob)	\
    ( (This)->lpVtbl -> GetJob(This,bstrJobId,pFaxIncomingJob) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IFaxIncomingQueue_INTERFACE_DEFINED__ */


#ifndef __IFaxOutgoingArchive_INTERFACE_DEFINED__
#define __IFaxOutgoingArchive_INTERFACE_DEFINED__

/* interface IFaxOutgoingArchive */
/* [unique][helpstring][dual][uuid][object] */ 


EXTERN_C const IID IID_IFaxOutgoingArchive;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("C9C28F40-8D80-4E53-810F-9A79919B49FD")
    IFaxOutgoingArchive : public IDispatch
    {
    public:
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_UseArchive( 
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pbUseArchive) = 0;
        
        virtual /* [helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_UseArchive( 
            /* [in] */ VARIANT_BOOL bUseArchive) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_ArchiveFolder( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrArchiveFolder) = 0;
        
        virtual /* [helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_ArchiveFolder( 
            /* [in] */ __RPC__in BSTR bstrArchiveFolder) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_SizeQuotaWarning( 
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pbSizeQuotaWarning) = 0;
        
        virtual /* [helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_SizeQuotaWarning( 
            /* [in] */ VARIANT_BOOL bSizeQuotaWarning) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_HighQuotaWaterMark( 
            /* [retval][out] */ __RPC__out long *plHighQuotaWaterMark) = 0;
        
        virtual /* [helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_HighQuotaWaterMark( 
            /* [in] */ long lHighQuotaWaterMark) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_LowQuotaWaterMark( 
            /* [retval][out] */ __RPC__out long *plLowQuotaWaterMark) = 0;
        
        virtual /* [helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_LowQuotaWaterMark( 
            /* [in] */ long lLowQuotaWaterMark) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_AgeLimit( 
            /* [retval][out] */ __RPC__out long *plAgeLimit) = 0;
        
        virtual /* [helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_AgeLimit( 
            /* [in] */ long lAgeLimit) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_SizeLow( 
            /* [retval][out] */ __RPC__out long *plSizeLow) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_SizeHigh( 
            /* [retval][out] */ __RPC__out long *plSizeHigh) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE Refresh( void) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE Save( void) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE GetMessages( 
            /* [defaultvalue][in] */ long lPrefetchSize,
            /* [retval][out] */ __RPC__deref_out_opt IFaxOutgoingMessageIterator **pFaxOutgoingMessageIterator) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE GetMessage( 
            /* [in] */ __RPC__in BSTR bstrMessageId,
            /* [retval][out] */ __RPC__deref_out_opt IFaxOutgoingMessage **pFaxOutgoingMessage) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IFaxOutgoingArchiveVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IFaxOutgoingArchive * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IFaxOutgoingArchive * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IFaxOutgoingArchive * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IFaxOutgoingArchive * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IFaxOutgoingArchive * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IFaxOutgoingArchive * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IFaxOutgoingArchive * This,
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
        
        DECLSPEC_XFGVIRT(IFaxOutgoingArchive, get_UseArchive)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_UseArchive )( 
            __RPC__in IFaxOutgoingArchive * This,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pbUseArchive);
        
        DECLSPEC_XFGVIRT(IFaxOutgoingArchive, put_UseArchive)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_UseArchive )( 
            __RPC__in IFaxOutgoingArchive * This,
            /* [in] */ VARIANT_BOOL bUseArchive);
        
        DECLSPEC_XFGVIRT(IFaxOutgoingArchive, get_ArchiveFolder)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_ArchiveFolder )( 
            __RPC__in IFaxOutgoingArchive * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrArchiveFolder);
        
        DECLSPEC_XFGVIRT(IFaxOutgoingArchive, put_ArchiveFolder)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_ArchiveFolder )( 
            __RPC__in IFaxOutgoingArchive * This,
            /* [in] */ __RPC__in BSTR bstrArchiveFolder);
        
        DECLSPEC_XFGVIRT(IFaxOutgoingArchive, get_SizeQuotaWarning)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_SizeQuotaWarning )( 
            __RPC__in IFaxOutgoingArchive * This,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pbSizeQuotaWarning);
        
        DECLSPEC_XFGVIRT(IFaxOutgoingArchive, put_SizeQuotaWarning)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_SizeQuotaWarning )( 
            __RPC__in IFaxOutgoingArchive * This,
            /* [in] */ VARIANT_BOOL bSizeQuotaWarning);
        
        DECLSPEC_XFGVIRT(IFaxOutgoingArchive, get_HighQuotaWaterMark)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_HighQuotaWaterMark )( 
            __RPC__in IFaxOutgoingArchive * This,
            /* [retval][out] */ __RPC__out long *plHighQuotaWaterMark);
        
        DECLSPEC_XFGVIRT(IFaxOutgoingArchive, put_HighQuotaWaterMark)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_HighQuotaWaterMark )( 
            __RPC__in IFaxOutgoingArchive * This,
            /* [in] */ long lHighQuotaWaterMark);
        
        DECLSPEC_XFGVIRT(IFaxOutgoingArchive, get_LowQuotaWaterMark)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_LowQuotaWaterMark )( 
            __RPC__in IFaxOutgoingArchive * This,
            /* [retval][out] */ __RPC__out long *plLowQuotaWaterMark);
        
        DECLSPEC_XFGVIRT(IFaxOutgoingArchive, put_LowQuotaWaterMark)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_LowQuotaWaterMark )( 
            __RPC__in IFaxOutgoingArchive * This,
            /* [in] */ long lLowQuotaWaterMark);
        
        DECLSPEC_XFGVIRT(IFaxOutgoingArchive, get_AgeLimit)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_AgeLimit )( 
            __RPC__in IFaxOutgoingArchive * This,
            /* [retval][out] */ __RPC__out long *plAgeLimit);
        
        DECLSPEC_XFGVIRT(IFaxOutgoingArchive, put_AgeLimit)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_AgeLimit )( 
            __RPC__in IFaxOutgoingArchive * This,
            /* [in] */ long lAgeLimit);
        
        DECLSPEC_XFGVIRT(IFaxOutgoingArchive, get_SizeLow)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_SizeLow )( 
            __RPC__in IFaxOutgoingArchive * This,
            /* [retval][out] */ __RPC__out long *plSizeLow);
        
        DECLSPEC_XFGVIRT(IFaxOutgoingArchive, get_SizeHigh)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_SizeHigh )( 
            __RPC__in IFaxOutgoingArchive * This,
            /* [retval][out] */ __RPC__out long *plSizeHigh);
        
        DECLSPEC_XFGVIRT(IFaxOutgoingArchive, Refresh)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *Refresh )( 
            __RPC__in IFaxOutgoingArchive * This);
        
        DECLSPEC_XFGVIRT(IFaxOutgoingArchive, Save)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *Save )( 
            __RPC__in IFaxOutgoingArchive * This);
        
        DECLSPEC_XFGVIRT(IFaxOutgoingArchive, GetMessages)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *GetMessages )( 
            __RPC__in IFaxOutgoingArchive * This,
            /* [defaultvalue][in] */ long lPrefetchSize,
            /* [retval][out] */ __RPC__deref_out_opt IFaxOutgoingMessageIterator **pFaxOutgoingMessageIterator);
        
        DECLSPEC_XFGVIRT(IFaxOutgoingArchive, GetMessage)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *GetMessage )( 
            __RPC__in IFaxOutgoingArchive * This,
            /* [in] */ __RPC__in BSTR bstrMessageId,
            /* [retval][out] */ __RPC__deref_out_opt IFaxOutgoingMessage **pFaxOutgoingMessage);
        
        END_INTERFACE
    } IFaxOutgoingArchiveVtbl;

    interface IFaxOutgoingArchive
    {
        CONST_VTBL struct IFaxOutgoingArchiveVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IFaxOutgoingArchive_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IFaxOutgoingArchive_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IFaxOutgoingArchive_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IFaxOutgoingArchive_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IFaxOutgoingArchive_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IFaxOutgoingArchive_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IFaxOutgoingArchive_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IFaxOutgoingArchive_get_UseArchive(This,pbUseArchive)	\
    ( (This)->lpVtbl -> get_UseArchive(This,pbUseArchive) ) 

#define IFaxOutgoingArchive_put_UseArchive(This,bUseArchive)	\
    ( (This)->lpVtbl -> put_UseArchive(This,bUseArchive) ) 

#define IFaxOutgoingArchive_get_ArchiveFolder(This,pbstrArchiveFolder)	\
    ( (This)->lpVtbl -> get_ArchiveFolder(This,pbstrArchiveFolder) ) 

#define IFaxOutgoingArchive_put_ArchiveFolder(This,bstrArchiveFolder)	\
    ( (This)->lpVtbl -> put_ArchiveFolder(This,bstrArchiveFolder) ) 

#define IFaxOutgoingArchive_get_SizeQuotaWarning(This,pbSizeQuotaWarning)	\
    ( (This)->lpVtbl -> get_SizeQuotaWarning(This,pbSizeQuotaWarning) ) 

#define IFaxOutgoingArchive_put_SizeQuotaWarning(This,bSizeQuotaWarning)	\
    ( (This)->lpVtbl -> put_SizeQuotaWarning(This,bSizeQuotaWarning) ) 

#define IFaxOutgoingArchive_get_HighQuotaWaterMark(This,plHighQuotaWaterMark)	\
    ( (This)->lpVtbl -> get_HighQuotaWaterMark(This,plHighQuotaWaterMark) ) 

#define IFaxOutgoingArchive_put_HighQuotaWaterMark(This,lHighQuotaWaterMark)	\
    ( (This)->lpVtbl -> put_HighQuotaWaterMark(This,lHighQuotaWaterMark) ) 

#define IFaxOutgoingArchive_get_LowQuotaWaterMark(This,plLowQuotaWaterMark)	\
    ( (This)->lpVtbl -> get_LowQuotaWaterMark(This,plLowQuotaWaterMark) ) 

#define IFaxOutgoingArchive_put_LowQuotaWaterMark(This,lLowQuotaWaterMark)	\
    ( (This)->lpVtbl -> put_LowQuotaWaterMark(This,lLowQuotaWaterMark) ) 

#define IFaxOutgoingArchive_get_AgeLimit(This,plAgeLimit)	\
    ( (This)->lpVtbl -> get_AgeLimit(This,plAgeLimit) ) 

#define IFaxOutgoingArchive_put_AgeLimit(This,lAgeLimit)	\
    ( (This)->lpVtbl -> put_AgeLimit(This,lAgeLimit) ) 

#define IFaxOutgoingArchive_get_SizeLow(This,plSizeLow)	\
    ( (This)->lpVtbl -> get_SizeLow(This,plSizeLow) ) 

#define IFaxOutgoingArchive_get_SizeHigh(This,plSizeHigh)	\
    ( (This)->lpVtbl -> get_SizeHigh(This,plSizeHigh) ) 

#define IFaxOutgoingArchive_Refresh(This)	\
    ( (This)->lpVtbl -> Refresh(This) ) 

#define IFaxOutgoingArchive_Save(This)	\
    ( (This)->lpVtbl -> Save(This) ) 

#define IFaxOutgoingArchive_GetMessages(This,lPrefetchSize,pFaxOutgoingMessageIterator)	\
    ( (This)->lpVtbl -> GetMessages(This,lPrefetchSize,pFaxOutgoingMessageIterator) ) 

#define IFaxOutgoingArchive_GetMessage(This,bstrMessageId,pFaxOutgoingMessage)	\
    ( (This)->lpVtbl -> GetMessage(This,bstrMessageId,pFaxOutgoingMessage) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IFaxOutgoingArchive_INTERFACE_DEFINED__ */


#ifndef __IFaxOutgoingQueue_INTERFACE_DEFINED__
#define __IFaxOutgoingQueue_INTERFACE_DEFINED__

/* interface IFaxOutgoingQueue */
/* [unique][helpstring][dual][uuid][object] */ 


EXTERN_C const IID IID_IFaxOutgoingQueue;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("80B1DF24-D9AC-4333-B373-487CEDC80CE5")
    IFaxOutgoingQueue : public IDispatch
    {
    public:
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Blocked( 
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pbBlocked) = 0;
        
        virtual /* [helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_Blocked( 
            /* [in] */ VARIANT_BOOL bBlocked) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Paused( 
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pbPaused) = 0;
        
        virtual /* [helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_Paused( 
            /* [in] */ VARIANT_BOOL bPaused) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_AllowPersonalCoverPages( 
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pbAllowPersonalCoverPages) = 0;
        
        virtual /* [helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_AllowPersonalCoverPages( 
            /* [in] */ VARIANT_BOOL bAllowPersonalCoverPages) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_UseDeviceTSID( 
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pbUseDeviceTSID) = 0;
        
        virtual /* [helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_UseDeviceTSID( 
            /* [in] */ VARIANT_BOOL bUseDeviceTSID) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Retries( 
            /* [retval][out] */ __RPC__out long *plRetries) = 0;
        
        virtual /* [helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_Retries( 
            /* [in] */ long lRetries) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_RetryDelay( 
            /* [retval][out] */ __RPC__out long *plRetryDelay) = 0;
        
        virtual /* [helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_RetryDelay( 
            /* [in] */ long lRetryDelay) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_DiscountRateStart( 
            /* [retval][out] */ __RPC__out DATE *pdateDiscountRateStart) = 0;
        
        virtual /* [helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_DiscountRateStart( 
            /* [in] */ DATE dateDiscountRateStart) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_DiscountRateEnd( 
            /* [retval][out] */ __RPC__out DATE *pdateDiscountRateEnd) = 0;
        
        virtual /* [helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_DiscountRateEnd( 
            /* [in] */ DATE dateDiscountRateEnd) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_AgeLimit( 
            /* [retval][out] */ __RPC__out long *plAgeLimit) = 0;
        
        virtual /* [helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_AgeLimit( 
            /* [in] */ long lAgeLimit) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Branding( 
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pbBranding) = 0;
        
        virtual /* [helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_Branding( 
            /* [in] */ VARIANT_BOOL bBranding) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE Refresh( void) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE Save( void) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE GetJobs( 
            /* [retval][out] */ __RPC__deref_out_opt IFaxOutgoingJobs **pFaxOutgoingJobs) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE GetJob( 
            /* [in] */ __RPC__in BSTR bstrJobId,
            /* [retval][out] */ __RPC__deref_out_opt IFaxOutgoingJob **pFaxOutgoingJob) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IFaxOutgoingQueueVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IFaxOutgoingQueue * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IFaxOutgoingQueue * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IFaxOutgoingQueue * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IFaxOutgoingQueue * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IFaxOutgoingQueue * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IFaxOutgoingQueue * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IFaxOutgoingQueue * This,
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
        
        DECLSPEC_XFGVIRT(IFaxOutgoingQueue, get_Blocked)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Blocked )( 
            __RPC__in IFaxOutgoingQueue * This,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pbBlocked);
        
        DECLSPEC_XFGVIRT(IFaxOutgoingQueue, put_Blocked)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_Blocked )( 
            __RPC__in IFaxOutgoingQueue * This,
            /* [in] */ VARIANT_BOOL bBlocked);
        
        DECLSPEC_XFGVIRT(IFaxOutgoingQueue, get_Paused)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Paused )( 
            __RPC__in IFaxOutgoingQueue * This,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pbPaused);
        
        DECLSPEC_XFGVIRT(IFaxOutgoingQueue, put_Paused)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_Paused )( 
            __RPC__in IFaxOutgoingQueue * This,
            /* [in] */ VARIANT_BOOL bPaused);
        
        DECLSPEC_XFGVIRT(IFaxOutgoingQueue, get_AllowPersonalCoverPages)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_AllowPersonalCoverPages )( 
            __RPC__in IFaxOutgoingQueue * This,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pbAllowPersonalCoverPages);
        
        DECLSPEC_XFGVIRT(IFaxOutgoingQueue, put_AllowPersonalCoverPages)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_AllowPersonalCoverPages )( 
            __RPC__in IFaxOutgoingQueue * This,
            /* [in] */ VARIANT_BOOL bAllowPersonalCoverPages);
        
        DECLSPEC_XFGVIRT(IFaxOutgoingQueue, get_UseDeviceTSID)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_UseDeviceTSID )( 
            __RPC__in IFaxOutgoingQueue * This,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pbUseDeviceTSID);
        
        DECLSPEC_XFGVIRT(IFaxOutgoingQueue, put_UseDeviceTSID)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_UseDeviceTSID )( 
            __RPC__in IFaxOutgoingQueue * This,
            /* [in] */ VARIANT_BOOL bUseDeviceTSID);
        
        DECLSPEC_XFGVIRT(IFaxOutgoingQueue, get_Retries)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Retries )( 
            __RPC__in IFaxOutgoingQueue * This,
            /* [retval][out] */ __RPC__out long *plRetries);
        
        DECLSPEC_XFGVIRT(IFaxOutgoingQueue, put_Retries)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_Retries )( 
            __RPC__in IFaxOutgoingQueue * This,
            /* [in] */ long lRetries);
        
        DECLSPEC_XFGVIRT(IFaxOutgoingQueue, get_RetryDelay)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_RetryDelay )( 
            __RPC__in IFaxOutgoingQueue * This,
            /* [retval][out] */ __RPC__out long *plRetryDelay);
        
        DECLSPEC_XFGVIRT(IFaxOutgoingQueue, put_RetryDelay)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_RetryDelay )( 
            __RPC__in IFaxOutgoingQueue * This,
            /* [in] */ long lRetryDelay);
        
        DECLSPEC_XFGVIRT(IFaxOutgoingQueue, get_DiscountRateStart)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_DiscountRateStart )( 
            __RPC__in IFaxOutgoingQueue * This,
            /* [retval][out] */ __RPC__out DATE *pdateDiscountRateStart);
        
        DECLSPEC_XFGVIRT(IFaxOutgoingQueue, put_DiscountRateStart)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_DiscountRateStart )( 
            __RPC__in IFaxOutgoingQueue * This,
            /* [in] */ DATE dateDiscountRateStart);
        
        DECLSPEC_XFGVIRT(IFaxOutgoingQueue, get_DiscountRateEnd)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_DiscountRateEnd )( 
            __RPC__in IFaxOutgoingQueue * This,
            /* [retval][out] */ __RPC__out DATE *pdateDiscountRateEnd);
        
        DECLSPEC_XFGVIRT(IFaxOutgoingQueue, put_DiscountRateEnd)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_DiscountRateEnd )( 
            __RPC__in IFaxOutgoingQueue * This,
            /* [in] */ DATE dateDiscountRateEnd);
        
        DECLSPEC_XFGVIRT(IFaxOutgoingQueue, get_AgeLimit)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_AgeLimit )( 
            __RPC__in IFaxOutgoingQueue * This,
            /* [retval][out] */ __RPC__out long *plAgeLimit);
        
        DECLSPEC_XFGVIRT(IFaxOutgoingQueue, put_AgeLimit)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_AgeLimit )( 
            __RPC__in IFaxOutgoingQueue * This,
            /* [in] */ long lAgeLimit);
        
        DECLSPEC_XFGVIRT(IFaxOutgoingQueue, get_Branding)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Branding )( 
            __RPC__in IFaxOutgoingQueue * This,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pbBranding);
        
        DECLSPEC_XFGVIRT(IFaxOutgoingQueue, put_Branding)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_Branding )( 
            __RPC__in IFaxOutgoingQueue * This,
            /* [in] */ VARIANT_BOOL bBranding);
        
        DECLSPEC_XFGVIRT(IFaxOutgoingQueue, Refresh)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *Refresh )( 
            __RPC__in IFaxOutgoingQueue * This);
        
        DECLSPEC_XFGVIRT(IFaxOutgoingQueue, Save)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *Save )( 
            __RPC__in IFaxOutgoingQueue * This);
        
        DECLSPEC_XFGVIRT(IFaxOutgoingQueue, GetJobs)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *GetJobs )( 
            __RPC__in IFaxOutgoingQueue * This,
            /* [retval][out] */ __RPC__deref_out_opt IFaxOutgoingJobs **pFaxOutgoingJobs);
        
        DECLSPEC_XFGVIRT(IFaxOutgoingQueue, GetJob)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *GetJob )( 
            __RPC__in IFaxOutgoingQueue * This,
            /* [in] */ __RPC__in BSTR bstrJobId,
            /* [retval][out] */ __RPC__deref_out_opt IFaxOutgoingJob **pFaxOutgoingJob);
        
        END_INTERFACE
    } IFaxOutgoingQueueVtbl;

    interface IFaxOutgoingQueue
    {
        CONST_VTBL struct IFaxOutgoingQueueVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IFaxOutgoingQueue_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IFaxOutgoingQueue_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IFaxOutgoingQueue_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IFaxOutgoingQueue_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IFaxOutgoingQueue_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IFaxOutgoingQueue_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IFaxOutgoingQueue_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IFaxOutgoingQueue_get_Blocked(This,pbBlocked)	\
    ( (This)->lpVtbl -> get_Blocked(This,pbBlocked) ) 

#define IFaxOutgoingQueue_put_Blocked(This,bBlocked)	\
    ( (This)->lpVtbl -> put_Blocked(This,bBlocked) ) 

#define IFaxOutgoingQueue_get_Paused(This,pbPaused)	\
    ( (This)->lpVtbl -> get_Paused(This,pbPaused) ) 

#define IFaxOutgoingQueue_put_Paused(This,bPaused)	\
    ( (This)->lpVtbl -> put_Paused(This,bPaused) ) 

#define IFaxOutgoingQueue_get_AllowPersonalCoverPages(This,pbAllowPersonalCoverPages)	\
    ( (This)->lpVtbl -> get_AllowPersonalCoverPages(This,pbAllowPersonalCoverPages) ) 

#define IFaxOutgoingQueue_put_AllowPersonalCoverPages(This,bAllowPersonalCoverPages)	\
    ( (This)->lpVtbl -> put_AllowPersonalCoverPages(This,bAllowPersonalCoverPages) ) 

#define IFaxOutgoingQueue_get_UseDeviceTSID(This,pbUseDeviceTSID)	\
    ( (This)->lpVtbl -> get_UseDeviceTSID(This,pbUseDeviceTSID) ) 

#define IFaxOutgoingQueue_put_UseDeviceTSID(This,bUseDeviceTSID)	\
    ( (This)->lpVtbl -> put_UseDeviceTSID(This,bUseDeviceTSID) ) 

#define IFaxOutgoingQueue_get_Retries(This,plRetries)	\
    ( (This)->lpVtbl -> get_Retries(This,plRetries) ) 

#define IFaxOutgoingQueue_put_Retries(This,lRetries)	\
    ( (This)->lpVtbl -> put_Retries(This,lRetries) ) 

#define IFaxOutgoingQueue_get_RetryDelay(This,plRetryDelay)	\
    ( (This)->lpVtbl -> get_RetryDelay(This,plRetryDelay) ) 

#define IFaxOutgoingQueue_put_RetryDelay(This,lRetryDelay)	\
    ( (This)->lpVtbl -> put_RetryDelay(This,lRetryDelay) ) 

#define IFaxOutgoingQueue_get_DiscountRateStart(This,pdateDiscountRateStart)	\
    ( (This)->lpVtbl -> get_DiscountRateStart(This,pdateDiscountRateStart) ) 

#define IFaxOutgoingQueue_put_DiscountRateStart(This,dateDiscountRateStart)	\
    ( (This)->lpVtbl -> put_DiscountRateStart(This,dateDiscountRateStart) ) 

#define IFaxOutgoingQueue_get_DiscountRateEnd(This,pdateDiscountRateEnd)	\
    ( (This)->lpVtbl -> get_DiscountRateEnd(This,pdateDiscountRateEnd) ) 

#define IFaxOutgoingQueue_put_DiscountRateEnd(This,dateDiscountRateEnd)	\
    ( (This)->lpVtbl -> put_DiscountRateEnd(This,dateDiscountRateEnd) ) 

#define IFaxOutgoingQueue_get_AgeLimit(This,plAgeLimit)	\
    ( (This)->lpVtbl -> get_AgeLimit(This,plAgeLimit) ) 

#define IFaxOutgoingQueue_put_AgeLimit(This,lAgeLimit)	\
    ( (This)->lpVtbl -> put_AgeLimit(This,lAgeLimit) ) 

#define IFaxOutgoingQueue_get_Branding(This,pbBranding)	\
    ( (This)->lpVtbl -> get_Branding(This,pbBranding) ) 

#define IFaxOutgoingQueue_put_Branding(This,bBranding)	\
    ( (This)->lpVtbl -> put_Branding(This,bBranding) ) 

#define IFaxOutgoingQueue_Refresh(This)	\
    ( (This)->lpVtbl -> Refresh(This) ) 

#define IFaxOutgoingQueue_Save(This)	\
    ( (This)->lpVtbl -> Save(This) ) 

#define IFaxOutgoingQueue_GetJobs(This,pFaxOutgoingJobs)	\
    ( (This)->lpVtbl -> GetJobs(This,pFaxOutgoingJobs) ) 

#define IFaxOutgoingQueue_GetJob(This,bstrJobId,pFaxOutgoingJob)	\
    ( (This)->lpVtbl -> GetJob(This,bstrJobId,pFaxOutgoingJob) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IFaxOutgoingQueue_INTERFACE_DEFINED__ */


#ifndef __IFaxIncomingMessageIterator_INTERFACE_DEFINED__
#define __IFaxIncomingMessageIterator_INTERFACE_DEFINED__

/* interface IFaxIncomingMessageIterator */
/* [unique][helpstring][dual][uuid][object] */ 


EXTERN_C const IID IID_IFaxIncomingMessageIterator;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("FD73ECC4-6F06-4F52-82A8-F7BA06AE3108")
    IFaxIncomingMessageIterator : public IDispatch
    {
    public:
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Message( 
            /* [retval][out] */ __RPC__deref_out_opt IFaxIncomingMessage **pFaxIncomingMessage) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_PrefetchSize( 
            /* [retval][out] */ __RPC__out long *plPrefetchSize) = 0;
        
        virtual /* [helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_PrefetchSize( 
            /* [in] */ long lPrefetchSize) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_AtEOF( 
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pbEOF) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE MoveFirst( void) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE MoveNext( void) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IFaxIncomingMessageIteratorVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IFaxIncomingMessageIterator * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IFaxIncomingMessageIterator * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IFaxIncomingMessageIterator * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IFaxIncomingMessageIterator * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IFaxIncomingMessageIterator * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IFaxIncomingMessageIterator * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IFaxIncomingMessageIterator * This,
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
        
        DECLSPEC_XFGVIRT(IFaxIncomingMessageIterator, get_Message)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Message )( 
            __RPC__in IFaxIncomingMessageIterator * This,
            /* [retval][out] */ __RPC__deref_out_opt IFaxIncomingMessage **pFaxIncomingMessage);
        
        DECLSPEC_XFGVIRT(IFaxIncomingMessageIterator, get_PrefetchSize)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_PrefetchSize )( 
            __RPC__in IFaxIncomingMessageIterator * This,
            /* [retval][out] */ __RPC__out long *plPrefetchSize);
        
        DECLSPEC_XFGVIRT(IFaxIncomingMessageIterator, put_PrefetchSize)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_PrefetchSize )( 
            __RPC__in IFaxIncomingMessageIterator * This,
            /* [in] */ long lPrefetchSize);
        
        DECLSPEC_XFGVIRT(IFaxIncomingMessageIterator, get_AtEOF)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_AtEOF )( 
            __RPC__in IFaxIncomingMessageIterator * This,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pbEOF);
        
        DECLSPEC_XFGVIRT(IFaxIncomingMessageIterator, MoveFirst)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *MoveFirst )( 
            __RPC__in IFaxIncomingMessageIterator * This);
        
        DECLSPEC_XFGVIRT(IFaxIncomingMessageIterator, MoveNext)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *MoveNext )( 
            __RPC__in IFaxIncomingMessageIterator * This);
        
        END_INTERFACE
    } IFaxIncomingMessageIteratorVtbl;

    interface IFaxIncomingMessageIterator
    {
        CONST_VTBL struct IFaxIncomingMessageIteratorVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IFaxIncomingMessageIterator_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IFaxIncomingMessageIterator_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IFaxIncomingMessageIterator_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IFaxIncomingMessageIterator_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IFaxIncomingMessageIterator_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IFaxIncomingMessageIterator_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IFaxIncomingMessageIterator_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IFaxIncomingMessageIterator_get_Message(This,pFaxIncomingMessage)	\
    ( (This)->lpVtbl -> get_Message(This,pFaxIncomingMessage) ) 

#define IFaxIncomingMessageIterator_get_PrefetchSize(This,plPrefetchSize)	\
    ( (This)->lpVtbl -> get_PrefetchSize(This,plPrefetchSize) ) 

#define IFaxIncomingMessageIterator_put_PrefetchSize(This,lPrefetchSize)	\
    ( (This)->lpVtbl -> put_PrefetchSize(This,lPrefetchSize) ) 

#define IFaxIncomingMessageIterator_get_AtEOF(This,pbEOF)	\
    ( (This)->lpVtbl -> get_AtEOF(This,pbEOF) ) 

#define IFaxIncomingMessageIterator_MoveFirst(This)	\
    ( (This)->lpVtbl -> MoveFirst(This) ) 

#define IFaxIncomingMessageIterator_MoveNext(This)	\
    ( (This)->lpVtbl -> MoveNext(This) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IFaxIncomingMessageIterator_INTERFACE_DEFINED__ */


#ifndef __IFaxIncomingMessage_INTERFACE_DEFINED__
#define __IFaxIncomingMessage_INTERFACE_DEFINED__

/* interface IFaxIncomingMessage */
/* [unique][helpstring][dual][uuid][object] */ 


EXTERN_C const IID IID_IFaxIncomingMessage;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("7CAB88FA-2EF9-4851-B2F3-1D148FED8447")
    IFaxIncomingMessage : public IDispatch
    {
    public:
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Id( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrId) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Pages( 
            /* [retval][out] */ __RPC__out long *plPages) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Size( 
            /* [retval][out] */ __RPC__out long *plSize) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_DeviceName( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrDeviceName) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Retries( 
            /* [retval][out] */ __RPC__out long *plRetries) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_TransmissionStart( 
            /* [retval][out] */ __RPC__out DATE *pdateTransmissionStart) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_TransmissionEnd( 
            /* [retval][out] */ __RPC__out DATE *pdateTransmissionEnd) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_CSID( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrCSID) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_TSID( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrTSID) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_CallerId( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrCallerId) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_RoutingInformation( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrRoutingInformation) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE CopyTiff( 
            /* [in] */ __RPC__in BSTR bstrTiffPath) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE Delete( void) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IFaxIncomingMessageVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IFaxIncomingMessage * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IFaxIncomingMessage * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IFaxIncomingMessage * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IFaxIncomingMessage * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IFaxIncomingMessage * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IFaxIncomingMessage * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IFaxIncomingMessage * This,
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
        
        DECLSPEC_XFGVIRT(IFaxIncomingMessage, get_Id)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Id )( 
            __RPC__in IFaxIncomingMessage * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrId);
        
        DECLSPEC_XFGVIRT(IFaxIncomingMessage, get_Pages)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Pages )( 
            __RPC__in IFaxIncomingMessage * This,
            /* [retval][out] */ __RPC__out long *plPages);
        
        DECLSPEC_XFGVIRT(IFaxIncomingMessage, get_Size)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Size )( 
            __RPC__in IFaxIncomingMessage * This,
            /* [retval][out] */ __RPC__out long *plSize);
        
        DECLSPEC_XFGVIRT(IFaxIncomingMessage, get_DeviceName)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_DeviceName )( 
            __RPC__in IFaxIncomingMessage * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrDeviceName);
        
        DECLSPEC_XFGVIRT(IFaxIncomingMessage, get_Retries)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Retries )( 
            __RPC__in IFaxIncomingMessage * This,
            /* [retval][out] */ __RPC__out long *plRetries);
        
        DECLSPEC_XFGVIRT(IFaxIncomingMessage, get_TransmissionStart)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_TransmissionStart )( 
            __RPC__in IFaxIncomingMessage * This,
            /* [retval][out] */ __RPC__out DATE *pdateTransmissionStart);
        
        DECLSPEC_XFGVIRT(IFaxIncomingMessage, get_TransmissionEnd)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_TransmissionEnd )( 
            __RPC__in IFaxIncomingMessage * This,
            /* [retval][out] */ __RPC__out DATE *pdateTransmissionEnd);
        
        DECLSPEC_XFGVIRT(IFaxIncomingMessage, get_CSID)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_CSID )( 
            __RPC__in IFaxIncomingMessage * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrCSID);
        
        DECLSPEC_XFGVIRT(IFaxIncomingMessage, get_TSID)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_TSID )( 
            __RPC__in IFaxIncomingMessage * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrTSID);
        
        DECLSPEC_XFGVIRT(IFaxIncomingMessage, get_CallerId)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_CallerId )( 
            __RPC__in IFaxIncomingMessage * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrCallerId);
        
        DECLSPEC_XFGVIRT(IFaxIncomingMessage, get_RoutingInformation)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_RoutingInformation )( 
            __RPC__in IFaxIncomingMessage * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrRoutingInformation);
        
        DECLSPEC_XFGVIRT(IFaxIncomingMessage, CopyTiff)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *CopyTiff )( 
            __RPC__in IFaxIncomingMessage * This,
            /* [in] */ __RPC__in BSTR bstrTiffPath);
        
        DECLSPEC_XFGVIRT(IFaxIncomingMessage, Delete)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *Delete )( 
            __RPC__in IFaxIncomingMessage * This);
        
        END_INTERFACE
    } IFaxIncomingMessageVtbl;

    interface IFaxIncomingMessage
    {
        CONST_VTBL struct IFaxIncomingMessageVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IFaxIncomingMessage_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IFaxIncomingMessage_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IFaxIncomingMessage_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IFaxIncomingMessage_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IFaxIncomingMessage_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IFaxIncomingMessage_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IFaxIncomingMessage_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IFaxIncomingMessage_get_Id(This,pbstrId)	\
    ( (This)->lpVtbl -> get_Id(This,pbstrId) ) 

#define IFaxIncomingMessage_get_Pages(This,plPages)	\
    ( (This)->lpVtbl -> get_Pages(This,plPages) ) 

#define IFaxIncomingMessage_get_Size(This,plSize)	\
    ( (This)->lpVtbl -> get_Size(This,plSize) ) 

#define IFaxIncomingMessage_get_DeviceName(This,pbstrDeviceName)	\
    ( (This)->lpVtbl -> get_DeviceName(This,pbstrDeviceName) ) 

#define IFaxIncomingMessage_get_Retries(This,plRetries)	\
    ( (This)->lpVtbl -> get_Retries(This,plRetries) ) 

#define IFaxIncomingMessage_get_TransmissionStart(This,pdateTransmissionStart)	\
    ( (This)->lpVtbl -> get_TransmissionStart(This,pdateTransmissionStart) ) 

#define IFaxIncomingMessage_get_TransmissionEnd(This,pdateTransmissionEnd)	\
    ( (This)->lpVtbl -> get_TransmissionEnd(This,pdateTransmissionEnd) ) 

#define IFaxIncomingMessage_get_CSID(This,pbstrCSID)	\
    ( (This)->lpVtbl -> get_CSID(This,pbstrCSID) ) 

#define IFaxIncomingMessage_get_TSID(This,pbstrTSID)	\
    ( (This)->lpVtbl -> get_TSID(This,pbstrTSID) ) 

#define IFaxIncomingMessage_get_CallerId(This,pbstrCallerId)	\
    ( (This)->lpVtbl -> get_CallerId(This,pbstrCallerId) ) 

#define IFaxIncomingMessage_get_RoutingInformation(This,pbstrRoutingInformation)	\
    ( (This)->lpVtbl -> get_RoutingInformation(This,pbstrRoutingInformation) ) 

#define IFaxIncomingMessage_CopyTiff(This,bstrTiffPath)	\
    ( (This)->lpVtbl -> CopyTiff(This,bstrTiffPath) ) 

#define IFaxIncomingMessage_Delete(This)	\
    ( (This)->lpVtbl -> Delete(This) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IFaxIncomingMessage_INTERFACE_DEFINED__ */


#ifndef __IFaxOutgoingJobs_INTERFACE_DEFINED__
#define __IFaxOutgoingJobs_INTERFACE_DEFINED__

/* interface IFaxOutgoingJobs */
/* [unique][helpstring][dual][uuid][object] */ 


EXTERN_C const IID IID_IFaxOutgoingJobs;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("2C56D8E6-8C2F-4573-944C-E505F8F5AEED")
    IFaxOutgoingJobs : public IDispatch
    {
    public:
        virtual /* [propget][id] */ HRESULT STDMETHODCALLTYPE get__NewEnum( 
            /* [retval][out] */ __RPC__deref_out_opt IUnknown **ppUnk) = 0;
        
        virtual /* [propget][id] */ HRESULT STDMETHODCALLTYPE get_Item( 
            /* [in] */ VARIANT vIndex,
            /* [retval][out] */ __RPC__deref_out_opt IFaxOutgoingJob **pFaxOutgoingJob) = 0;
        
        virtual /* [propget][id] */ HRESULT STDMETHODCALLTYPE get_Count( 
            /* [retval][out] */ __RPC__out long *plCount) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IFaxOutgoingJobsVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IFaxOutgoingJobs * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IFaxOutgoingJobs * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IFaxOutgoingJobs * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IFaxOutgoingJobs * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IFaxOutgoingJobs * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IFaxOutgoingJobs * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IFaxOutgoingJobs * This,
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
        
        DECLSPEC_XFGVIRT(IFaxOutgoingJobs, get__NewEnum)
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *get__NewEnum )( 
            __RPC__in IFaxOutgoingJobs * This,
            /* [retval][out] */ __RPC__deref_out_opt IUnknown **ppUnk);
        
        DECLSPEC_XFGVIRT(IFaxOutgoingJobs, get_Item)
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_Item )( 
            __RPC__in IFaxOutgoingJobs * This,
            /* [in] */ VARIANT vIndex,
            /* [retval][out] */ __RPC__deref_out_opt IFaxOutgoingJob **pFaxOutgoingJob);
        
        DECLSPEC_XFGVIRT(IFaxOutgoingJobs, get_Count)
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_Count )( 
            __RPC__in IFaxOutgoingJobs * This,
            /* [retval][out] */ __RPC__out long *plCount);
        
        END_INTERFACE
    } IFaxOutgoingJobsVtbl;

    interface IFaxOutgoingJobs
    {
        CONST_VTBL struct IFaxOutgoingJobsVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IFaxOutgoingJobs_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IFaxOutgoingJobs_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IFaxOutgoingJobs_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IFaxOutgoingJobs_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IFaxOutgoingJobs_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IFaxOutgoingJobs_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IFaxOutgoingJobs_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IFaxOutgoingJobs_get__NewEnum(This,ppUnk)	\
    ( (This)->lpVtbl -> get__NewEnum(This,ppUnk) ) 

#define IFaxOutgoingJobs_get_Item(This,vIndex,pFaxOutgoingJob)	\
    ( (This)->lpVtbl -> get_Item(This,vIndex,pFaxOutgoingJob) ) 

#define IFaxOutgoingJobs_get_Count(This,plCount)	\
    ( (This)->lpVtbl -> get_Count(This,plCount) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IFaxOutgoingJobs_INTERFACE_DEFINED__ */


#ifndef __IFaxOutgoingJob_INTERFACE_DEFINED__
#define __IFaxOutgoingJob_INTERFACE_DEFINED__

/* interface IFaxOutgoingJob */
/* [unique][helpstring][dual][uuid][object] */ 


EXTERN_C const IID IID_IFaxOutgoingJob;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("6356DAAD-6614-4583-BF7A-3AD67BBFC71C")
    IFaxOutgoingJob : public IDispatch
    {
    public:
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Subject( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrSubject) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_DocumentName( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrDocumentName) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Pages( 
            /* [retval][out] */ __RPC__out long *plPages) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Size( 
            /* [retval][out] */ __RPC__out long *plSize) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_SubmissionId( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrSubmissionId) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Id( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrId) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_OriginalScheduledTime( 
            /* [retval][out] */ __RPC__out DATE *pdateOriginalScheduledTime) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_SubmissionTime( 
            /* [retval][out] */ __RPC__out DATE *pdateSubmissionTime) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_ReceiptType( 
            /* [retval][out] */ __RPC__out FAX_RECEIPT_TYPE_ENUM *pReceiptType) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Priority( 
            /* [retval][out] */ __RPC__out FAX_PRIORITY_TYPE_ENUM *pPriority) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Sender( 
            /* [retval][out] */ __RPC__deref_out_opt IFaxSender **ppFaxSender) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Recipient( 
            /* [retval][out] */ __RPC__deref_out_opt IFaxRecipient **ppFaxRecipient) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_CurrentPage( 
            /* [retval][out] */ __RPC__out long *plCurrentPage) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_DeviceId( 
            /* [retval][out] */ __RPC__out long *plDeviceId) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Status( 
            /* [retval][out] */ __RPC__out FAX_JOB_STATUS_ENUM *pStatus) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_ExtendedStatusCode( 
            /* [retval][out] */ __RPC__out FAX_JOB_EXTENDED_STATUS_ENUM *pExtendedStatusCode) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_ExtendedStatus( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrExtendedStatus) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_AvailableOperations( 
            /* [retval][out] */ __RPC__out FAX_JOB_OPERATIONS_ENUM *pAvailableOperations) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Retries( 
            /* [retval][out] */ __RPC__out long *plRetries) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_ScheduledTime( 
            /* [retval][out] */ __RPC__out DATE *pdateScheduledTime) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_TransmissionStart( 
            /* [retval][out] */ __RPC__out DATE *pdateTransmissionStart) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_TransmissionEnd( 
            /* [retval][out] */ __RPC__out DATE *pdateTransmissionEnd) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_CSID( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrCSID) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_TSID( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrTSID) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_GroupBroadcastReceipts( 
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pbGroupBroadcastReceipts) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE Pause( void) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE Resume( void) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE Restart( void) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE CopyTiff( 
            /* [in] */ __RPC__in BSTR bstrTiffPath) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE Refresh( void) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE Cancel( void) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IFaxOutgoingJobVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IFaxOutgoingJob * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IFaxOutgoingJob * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IFaxOutgoingJob * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IFaxOutgoingJob * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IFaxOutgoingJob * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IFaxOutgoingJob * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IFaxOutgoingJob * This,
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
        
        DECLSPEC_XFGVIRT(IFaxOutgoingJob, get_Subject)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Subject )( 
            __RPC__in IFaxOutgoingJob * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrSubject);
        
        DECLSPEC_XFGVIRT(IFaxOutgoingJob, get_DocumentName)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_DocumentName )( 
            __RPC__in IFaxOutgoingJob * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrDocumentName);
        
        DECLSPEC_XFGVIRT(IFaxOutgoingJob, get_Pages)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Pages )( 
            __RPC__in IFaxOutgoingJob * This,
            /* [retval][out] */ __RPC__out long *plPages);
        
        DECLSPEC_XFGVIRT(IFaxOutgoingJob, get_Size)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Size )( 
            __RPC__in IFaxOutgoingJob * This,
            /* [retval][out] */ __RPC__out long *plSize);
        
        DECLSPEC_XFGVIRT(IFaxOutgoingJob, get_SubmissionId)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_SubmissionId )( 
            __RPC__in IFaxOutgoingJob * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrSubmissionId);
        
        DECLSPEC_XFGVIRT(IFaxOutgoingJob, get_Id)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Id )( 
            __RPC__in IFaxOutgoingJob * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrId);
        
        DECLSPEC_XFGVIRT(IFaxOutgoingJob, get_OriginalScheduledTime)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_OriginalScheduledTime )( 
            __RPC__in IFaxOutgoingJob * This,
            /* [retval][out] */ __RPC__out DATE *pdateOriginalScheduledTime);
        
        DECLSPEC_XFGVIRT(IFaxOutgoingJob, get_SubmissionTime)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_SubmissionTime )( 
            __RPC__in IFaxOutgoingJob * This,
            /* [retval][out] */ __RPC__out DATE *pdateSubmissionTime);
        
        DECLSPEC_XFGVIRT(IFaxOutgoingJob, get_ReceiptType)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_ReceiptType )( 
            __RPC__in IFaxOutgoingJob * This,
            /* [retval][out] */ __RPC__out FAX_RECEIPT_TYPE_ENUM *pReceiptType);
        
        DECLSPEC_XFGVIRT(IFaxOutgoingJob, get_Priority)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Priority )( 
            __RPC__in IFaxOutgoingJob * This,
            /* [retval][out] */ __RPC__out FAX_PRIORITY_TYPE_ENUM *pPriority);
        
        DECLSPEC_XFGVIRT(IFaxOutgoingJob, get_Sender)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Sender )( 
            __RPC__in IFaxOutgoingJob * This,
            /* [retval][out] */ __RPC__deref_out_opt IFaxSender **ppFaxSender);
        
        DECLSPEC_XFGVIRT(IFaxOutgoingJob, get_Recipient)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Recipient )( 
            __RPC__in IFaxOutgoingJob * This,
            /* [retval][out] */ __RPC__deref_out_opt IFaxRecipient **ppFaxRecipient);
        
        DECLSPEC_XFGVIRT(IFaxOutgoingJob, get_CurrentPage)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_CurrentPage )( 
            __RPC__in IFaxOutgoingJob * This,
            /* [retval][out] */ __RPC__out long *plCurrentPage);
        
        DECLSPEC_XFGVIRT(IFaxOutgoingJob, get_DeviceId)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_DeviceId )( 
            __RPC__in IFaxOutgoingJob * This,
            /* [retval][out] */ __RPC__out long *plDeviceId);
        
        DECLSPEC_XFGVIRT(IFaxOutgoingJob, get_Status)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Status )( 
            __RPC__in IFaxOutgoingJob * This,
            /* [retval][out] */ __RPC__out FAX_JOB_STATUS_ENUM *pStatus);
        
        DECLSPEC_XFGVIRT(IFaxOutgoingJob, get_ExtendedStatusCode)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_ExtendedStatusCode )( 
            __RPC__in IFaxOutgoingJob * This,
            /* [retval][out] */ __RPC__out FAX_JOB_EXTENDED_STATUS_ENUM *pExtendedStatusCode);
        
        DECLSPEC_XFGVIRT(IFaxOutgoingJob, get_ExtendedStatus)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_ExtendedStatus )( 
            __RPC__in IFaxOutgoingJob * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrExtendedStatus);
        
        DECLSPEC_XFGVIRT(IFaxOutgoingJob, get_AvailableOperations)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_AvailableOperations )( 
            __RPC__in IFaxOutgoingJob * This,
            /* [retval][out] */ __RPC__out FAX_JOB_OPERATIONS_ENUM *pAvailableOperations);
        
        DECLSPEC_XFGVIRT(IFaxOutgoingJob, get_Retries)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Retries )( 
            __RPC__in IFaxOutgoingJob * This,
            /* [retval][out] */ __RPC__out long *plRetries);
        
        DECLSPEC_XFGVIRT(IFaxOutgoingJob, get_ScheduledTime)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_ScheduledTime )( 
            __RPC__in IFaxOutgoingJob * This,
            /* [retval][out] */ __RPC__out DATE *pdateScheduledTime);
        
        DECLSPEC_XFGVIRT(IFaxOutgoingJob, get_TransmissionStart)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_TransmissionStart )( 
            __RPC__in IFaxOutgoingJob * This,
            /* [retval][out] */ __RPC__out DATE *pdateTransmissionStart);
        
        DECLSPEC_XFGVIRT(IFaxOutgoingJob, get_TransmissionEnd)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_TransmissionEnd )( 
            __RPC__in IFaxOutgoingJob * This,
            /* [retval][out] */ __RPC__out DATE *pdateTransmissionEnd);
        
        DECLSPEC_XFGVIRT(IFaxOutgoingJob, get_CSID)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_CSID )( 
            __RPC__in IFaxOutgoingJob * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrCSID);
        
        DECLSPEC_XFGVIRT(IFaxOutgoingJob, get_TSID)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_TSID )( 
            __RPC__in IFaxOutgoingJob * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrTSID);
        
        DECLSPEC_XFGVIRT(IFaxOutgoingJob, get_GroupBroadcastReceipts)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_GroupBroadcastReceipts )( 
            __RPC__in IFaxOutgoingJob * This,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pbGroupBroadcastReceipts);
        
        DECLSPEC_XFGVIRT(IFaxOutgoingJob, Pause)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *Pause )( 
            __RPC__in IFaxOutgoingJob * This);
        
        DECLSPEC_XFGVIRT(IFaxOutgoingJob, Resume)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *Resume )( 
            __RPC__in IFaxOutgoingJob * This);
        
        DECLSPEC_XFGVIRT(IFaxOutgoingJob, Restart)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *Restart )( 
            __RPC__in IFaxOutgoingJob * This);
        
        DECLSPEC_XFGVIRT(IFaxOutgoingJob, CopyTiff)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *CopyTiff )( 
            __RPC__in IFaxOutgoingJob * This,
            /* [in] */ __RPC__in BSTR bstrTiffPath);
        
        DECLSPEC_XFGVIRT(IFaxOutgoingJob, Refresh)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *Refresh )( 
            __RPC__in IFaxOutgoingJob * This);
        
        DECLSPEC_XFGVIRT(IFaxOutgoingJob, Cancel)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *Cancel )( 
            __RPC__in IFaxOutgoingJob * This);
        
        END_INTERFACE
    } IFaxOutgoingJobVtbl;

    interface IFaxOutgoingJob
    {
        CONST_VTBL struct IFaxOutgoingJobVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IFaxOutgoingJob_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IFaxOutgoingJob_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IFaxOutgoingJob_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IFaxOutgoingJob_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IFaxOutgoingJob_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IFaxOutgoingJob_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IFaxOutgoingJob_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IFaxOutgoingJob_get_Subject(This,pbstrSubject)	\
    ( (This)->lpVtbl -> get_Subject(This,pbstrSubject) ) 

#define IFaxOutgoingJob_get_DocumentName(This,pbstrDocumentName)	\
    ( (This)->lpVtbl -> get_DocumentName(This,pbstrDocumentName) ) 

#define IFaxOutgoingJob_get_Pages(This,plPages)	\
    ( (This)->lpVtbl -> get_Pages(This,plPages) ) 

#define IFaxOutgoingJob_get_Size(This,plSize)	\
    ( (This)->lpVtbl -> get_Size(This,plSize) ) 

#define IFaxOutgoingJob_get_SubmissionId(This,pbstrSubmissionId)	\
    ( (This)->lpVtbl -> get_SubmissionId(This,pbstrSubmissionId) ) 

#define IFaxOutgoingJob_get_Id(This,pbstrId)	\
    ( (This)->lpVtbl -> get_Id(This,pbstrId) ) 

#define IFaxOutgoingJob_get_OriginalScheduledTime(This,pdateOriginalScheduledTime)	\
    ( (This)->lpVtbl -> get_OriginalScheduledTime(This,pdateOriginalScheduledTime) ) 

#define IFaxOutgoingJob_get_SubmissionTime(This,pdateSubmissionTime)	\
    ( (This)->lpVtbl -> get_SubmissionTime(This,pdateSubmissionTime) ) 

#define IFaxOutgoingJob_get_ReceiptType(This,pReceiptType)	\
    ( (This)->lpVtbl -> get_ReceiptType(This,pReceiptType) ) 

#define IFaxOutgoingJob_get_Priority(This,pPriority)	\
    ( (This)->lpVtbl -> get_Priority(This,pPriority) ) 

#define IFaxOutgoingJob_get_Sender(This,ppFaxSender)	\
    ( (This)->lpVtbl -> get_Sender(This,ppFaxSender) ) 

#define IFaxOutgoingJob_get_Recipient(This,ppFaxRecipient)	\
    ( (This)->lpVtbl -> get_Recipient(This,ppFaxRecipient) ) 

#define IFaxOutgoingJob_get_CurrentPage(This,plCurrentPage)	\
    ( (This)->lpVtbl -> get_CurrentPage(This,plCurrentPage) ) 

#define IFaxOutgoingJob_get_DeviceId(This,plDeviceId)	\
    ( (This)->lpVtbl -> get_DeviceId(This,plDeviceId) ) 

#define IFaxOutgoingJob_get_Status(This,pStatus)	\
    ( (This)->lpVtbl -> get_Status(This,pStatus) ) 

#define IFaxOutgoingJob_get_ExtendedStatusCode(This,pExtendedStatusCode)	\
    ( (This)->lpVtbl -> get_ExtendedStatusCode(This,pExtendedStatusCode) ) 

#define IFaxOutgoingJob_get_ExtendedStatus(This,pbstrExtendedStatus)	\
    ( (This)->lpVtbl -> get_ExtendedStatus(This,pbstrExtendedStatus) ) 

#define IFaxOutgoingJob_get_AvailableOperations(This,pAvailableOperations)	\
    ( (This)->lpVtbl -> get_AvailableOperations(This,pAvailableOperations) ) 

#define IFaxOutgoingJob_get_Retries(This,plRetries)	\
    ( (This)->lpVtbl -> get_Retries(This,plRetries) ) 

#define IFaxOutgoingJob_get_ScheduledTime(This,pdateScheduledTime)	\
    ( (This)->lpVtbl -> get_ScheduledTime(This,pdateScheduledTime) ) 

#define IFaxOutgoingJob_get_TransmissionStart(This,pdateTransmissionStart)	\
    ( (This)->lpVtbl -> get_TransmissionStart(This,pdateTransmissionStart) ) 

#define IFaxOutgoingJob_get_TransmissionEnd(This,pdateTransmissionEnd)	\
    ( (This)->lpVtbl -> get_TransmissionEnd(This,pdateTransmissionEnd) ) 

#define IFaxOutgoingJob_get_CSID(This,pbstrCSID)	\
    ( (This)->lpVtbl -> get_CSID(This,pbstrCSID) ) 

#define IFaxOutgoingJob_get_TSID(This,pbstrTSID)	\
    ( (This)->lpVtbl -> get_TSID(This,pbstrTSID) ) 

#define IFaxOutgoingJob_get_GroupBroadcastReceipts(This,pbGroupBroadcastReceipts)	\
    ( (This)->lpVtbl -> get_GroupBroadcastReceipts(This,pbGroupBroadcastReceipts) ) 

#define IFaxOutgoingJob_Pause(This)	\
    ( (This)->lpVtbl -> Pause(This) ) 

#define IFaxOutgoingJob_Resume(This)	\
    ( (This)->lpVtbl -> Resume(This) ) 

#define IFaxOutgoingJob_Restart(This)	\
    ( (This)->lpVtbl -> Restart(This) ) 

#define IFaxOutgoingJob_CopyTiff(This,bstrTiffPath)	\
    ( (This)->lpVtbl -> CopyTiff(This,bstrTiffPath) ) 

#define IFaxOutgoingJob_Refresh(This)	\
    ( (This)->lpVtbl -> Refresh(This) ) 

#define IFaxOutgoingJob_Cancel(This)	\
    ( (This)->lpVtbl -> Cancel(This) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IFaxOutgoingJob_INTERFACE_DEFINED__ */


#ifndef __IFaxOutgoingMessageIterator_INTERFACE_DEFINED__
#define __IFaxOutgoingMessageIterator_INTERFACE_DEFINED__

/* interface IFaxOutgoingMessageIterator */
/* [unique][helpstring][dual][uuid][object] */ 


EXTERN_C const IID IID_IFaxOutgoingMessageIterator;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("F5EC5D4F-B840-432F-9980-112FE42A9B7A")
    IFaxOutgoingMessageIterator : public IDispatch
    {
    public:
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Message( 
            /* [retval][out] */ __RPC__deref_out_opt IFaxOutgoingMessage **pFaxOutgoingMessage) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_AtEOF( 
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pbEOF) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_PrefetchSize( 
            /* [retval][out] */ __RPC__out long *plPrefetchSize) = 0;
        
        virtual /* [helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_PrefetchSize( 
            /* [in] */ long lPrefetchSize) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE MoveFirst( void) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE MoveNext( void) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IFaxOutgoingMessageIteratorVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IFaxOutgoingMessageIterator * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IFaxOutgoingMessageIterator * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IFaxOutgoingMessageIterator * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IFaxOutgoingMessageIterator * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IFaxOutgoingMessageIterator * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IFaxOutgoingMessageIterator * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IFaxOutgoingMessageIterator * This,
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
        
        DECLSPEC_XFGVIRT(IFaxOutgoingMessageIterator, get_Message)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Message )( 
            __RPC__in IFaxOutgoingMessageIterator * This,
            /* [retval][out] */ __RPC__deref_out_opt IFaxOutgoingMessage **pFaxOutgoingMessage);
        
        DECLSPEC_XFGVIRT(IFaxOutgoingMessageIterator, get_AtEOF)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_AtEOF )( 
            __RPC__in IFaxOutgoingMessageIterator * This,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pbEOF);
        
        DECLSPEC_XFGVIRT(IFaxOutgoingMessageIterator, get_PrefetchSize)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_PrefetchSize )( 
            __RPC__in IFaxOutgoingMessageIterator * This,
            /* [retval][out] */ __RPC__out long *plPrefetchSize);
        
        DECLSPEC_XFGVIRT(IFaxOutgoingMessageIterator, put_PrefetchSize)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_PrefetchSize )( 
            __RPC__in IFaxOutgoingMessageIterator * This,
            /* [in] */ long lPrefetchSize);
        
        DECLSPEC_XFGVIRT(IFaxOutgoingMessageIterator, MoveFirst)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *MoveFirst )( 
            __RPC__in IFaxOutgoingMessageIterator * This);
        
        DECLSPEC_XFGVIRT(IFaxOutgoingMessageIterator, MoveNext)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *MoveNext )( 
            __RPC__in IFaxOutgoingMessageIterator * This);
        
        END_INTERFACE
    } IFaxOutgoingMessageIteratorVtbl;

    interface IFaxOutgoingMessageIterator
    {
        CONST_VTBL struct IFaxOutgoingMessageIteratorVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IFaxOutgoingMessageIterator_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IFaxOutgoingMessageIterator_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IFaxOutgoingMessageIterator_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IFaxOutgoingMessageIterator_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IFaxOutgoingMessageIterator_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IFaxOutgoingMessageIterator_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IFaxOutgoingMessageIterator_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IFaxOutgoingMessageIterator_get_Message(This,pFaxOutgoingMessage)	\
    ( (This)->lpVtbl -> get_Message(This,pFaxOutgoingMessage) ) 

#define IFaxOutgoingMessageIterator_get_AtEOF(This,pbEOF)	\
    ( (This)->lpVtbl -> get_AtEOF(This,pbEOF) ) 

#define IFaxOutgoingMessageIterator_get_PrefetchSize(This,plPrefetchSize)	\
    ( (This)->lpVtbl -> get_PrefetchSize(This,plPrefetchSize) ) 

#define IFaxOutgoingMessageIterator_put_PrefetchSize(This,lPrefetchSize)	\
    ( (This)->lpVtbl -> put_PrefetchSize(This,lPrefetchSize) ) 

#define IFaxOutgoingMessageIterator_MoveFirst(This)	\
    ( (This)->lpVtbl -> MoveFirst(This) ) 

#define IFaxOutgoingMessageIterator_MoveNext(This)	\
    ( (This)->lpVtbl -> MoveNext(This) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IFaxOutgoingMessageIterator_INTERFACE_DEFINED__ */


#ifndef __IFaxOutgoingMessage_INTERFACE_DEFINED__
#define __IFaxOutgoingMessage_INTERFACE_DEFINED__

/* interface IFaxOutgoingMessage */
/* [unique][helpstring][dual][uuid][object] */ 


EXTERN_C const IID IID_IFaxOutgoingMessage;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("F0EA35DE-CAA5-4A7C-82C7-2B60BA5F2BE2")
    IFaxOutgoingMessage : public IDispatch
    {
    public:
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_SubmissionId( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrSubmissionId) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Id( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrId) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Subject( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrSubject) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_DocumentName( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrDocumentName) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Retries( 
            /* [retval][out] */ __RPC__out long *plRetries) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Pages( 
            /* [retval][out] */ __RPC__out long *plPages) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Size( 
            /* [retval][out] */ __RPC__out long *plSize) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_OriginalScheduledTime( 
            /* [retval][out] */ __RPC__out DATE *pdateOriginalScheduledTime) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_SubmissionTime( 
            /* [retval][out] */ __RPC__out DATE *pdateSubmissionTime) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Priority( 
            /* [retval][out] */ __RPC__out FAX_PRIORITY_TYPE_ENUM *pPriority) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Sender( 
            /* [retval][out] */ __RPC__deref_out_opt IFaxSender **ppFaxSender) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Recipient( 
            /* [retval][out] */ __RPC__deref_out_opt IFaxRecipient **ppFaxRecipient) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_DeviceName( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrDeviceName) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_TransmissionStart( 
            /* [retval][out] */ __RPC__out DATE *pdateTransmissionStart) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_TransmissionEnd( 
            /* [retval][out] */ __RPC__out DATE *pdateTransmissionEnd) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_CSID( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrCSID) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_TSID( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrTSID) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE CopyTiff( 
            /* [in] */ __RPC__in BSTR bstrTiffPath) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE Delete( void) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IFaxOutgoingMessageVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IFaxOutgoingMessage * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IFaxOutgoingMessage * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IFaxOutgoingMessage * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IFaxOutgoingMessage * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IFaxOutgoingMessage * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IFaxOutgoingMessage * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IFaxOutgoingMessage * This,
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
        
        DECLSPEC_XFGVIRT(IFaxOutgoingMessage, get_SubmissionId)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_SubmissionId )( 
            __RPC__in IFaxOutgoingMessage * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrSubmissionId);
        
        DECLSPEC_XFGVIRT(IFaxOutgoingMessage, get_Id)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Id )( 
            __RPC__in IFaxOutgoingMessage * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrId);
        
        DECLSPEC_XFGVIRT(IFaxOutgoingMessage, get_Subject)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Subject )( 
            __RPC__in IFaxOutgoingMessage * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrSubject);
        
        DECLSPEC_XFGVIRT(IFaxOutgoingMessage, get_DocumentName)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_DocumentName )( 
            __RPC__in IFaxOutgoingMessage * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrDocumentName);
        
        DECLSPEC_XFGVIRT(IFaxOutgoingMessage, get_Retries)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Retries )( 
            __RPC__in IFaxOutgoingMessage * This,
            /* [retval][out] */ __RPC__out long *plRetries);
        
        DECLSPEC_XFGVIRT(IFaxOutgoingMessage, get_Pages)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Pages )( 
            __RPC__in IFaxOutgoingMessage * This,
            /* [retval][out] */ __RPC__out long *plPages);
        
        DECLSPEC_XFGVIRT(IFaxOutgoingMessage, get_Size)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Size )( 
            __RPC__in IFaxOutgoingMessage * This,
            /* [retval][out] */ __RPC__out long *plSize);
        
        DECLSPEC_XFGVIRT(IFaxOutgoingMessage, get_OriginalScheduledTime)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_OriginalScheduledTime )( 
            __RPC__in IFaxOutgoingMessage * This,
            /* [retval][out] */ __RPC__out DATE *pdateOriginalScheduledTime);
        
        DECLSPEC_XFGVIRT(IFaxOutgoingMessage, get_SubmissionTime)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_SubmissionTime )( 
            __RPC__in IFaxOutgoingMessage * This,
            /* [retval][out] */ __RPC__out DATE *pdateSubmissionTime);
        
        DECLSPEC_XFGVIRT(IFaxOutgoingMessage, get_Priority)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Priority )( 
            __RPC__in IFaxOutgoingMessage * This,
            /* [retval][out] */ __RPC__out FAX_PRIORITY_TYPE_ENUM *pPriority);
        
        DECLSPEC_XFGVIRT(IFaxOutgoingMessage, get_Sender)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Sender )( 
            __RPC__in IFaxOutgoingMessage * This,
            /* [retval][out] */ __RPC__deref_out_opt IFaxSender **ppFaxSender);
        
        DECLSPEC_XFGVIRT(IFaxOutgoingMessage, get_Recipient)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Recipient )( 
            __RPC__in IFaxOutgoingMessage * This,
            /* [retval][out] */ __RPC__deref_out_opt IFaxRecipient **ppFaxRecipient);
        
        DECLSPEC_XFGVIRT(IFaxOutgoingMessage, get_DeviceName)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_DeviceName )( 
            __RPC__in IFaxOutgoingMessage * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrDeviceName);
        
        DECLSPEC_XFGVIRT(IFaxOutgoingMessage, get_TransmissionStart)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_TransmissionStart )( 
            __RPC__in IFaxOutgoingMessage * This,
            /* [retval][out] */ __RPC__out DATE *pdateTransmissionStart);
        
        DECLSPEC_XFGVIRT(IFaxOutgoingMessage, get_TransmissionEnd)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_TransmissionEnd )( 
            __RPC__in IFaxOutgoingMessage * This,
            /* [retval][out] */ __RPC__out DATE *pdateTransmissionEnd);
        
        DECLSPEC_XFGVIRT(IFaxOutgoingMessage, get_CSID)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_CSID )( 
            __RPC__in IFaxOutgoingMessage * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrCSID);
        
        DECLSPEC_XFGVIRT(IFaxOutgoingMessage, get_TSID)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_TSID )( 
            __RPC__in IFaxOutgoingMessage * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrTSID);
        
        DECLSPEC_XFGVIRT(IFaxOutgoingMessage, CopyTiff)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *CopyTiff )( 
            __RPC__in IFaxOutgoingMessage * This,
            /* [in] */ __RPC__in BSTR bstrTiffPath);
        
        DECLSPEC_XFGVIRT(IFaxOutgoingMessage, Delete)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *Delete )( 
            __RPC__in IFaxOutgoingMessage * This);
        
        END_INTERFACE
    } IFaxOutgoingMessageVtbl;

    interface IFaxOutgoingMessage
    {
        CONST_VTBL struct IFaxOutgoingMessageVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IFaxOutgoingMessage_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IFaxOutgoingMessage_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IFaxOutgoingMessage_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IFaxOutgoingMessage_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IFaxOutgoingMessage_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IFaxOutgoingMessage_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IFaxOutgoingMessage_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IFaxOutgoingMessage_get_SubmissionId(This,pbstrSubmissionId)	\
    ( (This)->lpVtbl -> get_SubmissionId(This,pbstrSubmissionId) ) 

#define IFaxOutgoingMessage_get_Id(This,pbstrId)	\
    ( (This)->lpVtbl -> get_Id(This,pbstrId) ) 

#define IFaxOutgoingMessage_get_Subject(This,pbstrSubject)	\
    ( (This)->lpVtbl -> get_Subject(This,pbstrSubject) ) 

#define IFaxOutgoingMessage_get_DocumentName(This,pbstrDocumentName)	\
    ( (This)->lpVtbl -> get_DocumentName(This,pbstrDocumentName) ) 

#define IFaxOutgoingMessage_get_Retries(This,plRetries)	\
    ( (This)->lpVtbl -> get_Retries(This,plRetries) ) 

#define IFaxOutgoingMessage_get_Pages(This,plPages)	\
    ( (This)->lpVtbl -> get_Pages(This,plPages) ) 

#define IFaxOutgoingMessage_get_Size(This,plSize)	\
    ( (This)->lpVtbl -> get_Size(This,plSize) ) 

#define IFaxOutgoingMessage_get_OriginalScheduledTime(This,pdateOriginalScheduledTime)	\
    ( (This)->lpVtbl -> get_OriginalScheduledTime(This,pdateOriginalScheduledTime) ) 

#define IFaxOutgoingMessage_get_SubmissionTime(This,pdateSubmissionTime)	\
    ( (This)->lpVtbl -> get_SubmissionTime(This,pdateSubmissionTime) ) 

#define IFaxOutgoingMessage_get_Priority(This,pPriority)	\
    ( (This)->lpVtbl -> get_Priority(This,pPriority) ) 

#define IFaxOutgoingMessage_get_Sender(This,ppFaxSender)	\
    ( (This)->lpVtbl -> get_Sender(This,ppFaxSender) ) 

#define IFaxOutgoingMessage_get_Recipient(This,ppFaxRecipient)	\
    ( (This)->lpVtbl -> get_Recipient(This,ppFaxRecipient) ) 

#define IFaxOutgoingMessage_get_DeviceName(This,pbstrDeviceName)	\
    ( (This)->lpVtbl -> get_DeviceName(This,pbstrDeviceName) ) 

#define IFaxOutgoingMessage_get_TransmissionStart(This,pdateTransmissionStart)	\
    ( (This)->lpVtbl -> get_TransmissionStart(This,pdateTransmissionStart) ) 

#define IFaxOutgoingMessage_get_TransmissionEnd(This,pdateTransmissionEnd)	\
    ( (This)->lpVtbl -> get_TransmissionEnd(This,pdateTransmissionEnd) ) 

#define IFaxOutgoingMessage_get_CSID(This,pbstrCSID)	\
    ( (This)->lpVtbl -> get_CSID(This,pbstrCSID) ) 

#define IFaxOutgoingMessage_get_TSID(This,pbstrTSID)	\
    ( (This)->lpVtbl -> get_TSID(This,pbstrTSID) ) 

#define IFaxOutgoingMessage_CopyTiff(This,bstrTiffPath)	\
    ( (This)->lpVtbl -> CopyTiff(This,bstrTiffPath) ) 

#define IFaxOutgoingMessage_Delete(This)	\
    ( (This)->lpVtbl -> Delete(This) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IFaxOutgoingMessage_INTERFACE_DEFINED__ */


#ifndef __IFaxIncomingJobs_INTERFACE_DEFINED__
#define __IFaxIncomingJobs_INTERFACE_DEFINED__

/* interface IFaxIncomingJobs */
/* [unique][helpstring][dual][uuid][object] */ 


EXTERN_C const IID IID_IFaxIncomingJobs;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("011F04E9-4FD6-4C23-9513-B6B66BB26BE9")
    IFaxIncomingJobs : public IDispatch
    {
    public:
        virtual /* [propget][id] */ HRESULT STDMETHODCALLTYPE get__NewEnum( 
            /* [retval][out] */ __RPC__deref_out_opt IUnknown **ppUnk) = 0;
        
        virtual /* [propget][id] */ HRESULT STDMETHODCALLTYPE get_Item( 
            /* [in] */ VARIANT vIndex,
            /* [retval][out] */ __RPC__deref_out_opt IFaxIncomingJob **pFaxIncomingJob) = 0;
        
        virtual /* [propget][id] */ HRESULT STDMETHODCALLTYPE get_Count( 
            /* [retval][out] */ __RPC__out long *plCount) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IFaxIncomingJobsVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IFaxIncomingJobs * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IFaxIncomingJobs * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IFaxIncomingJobs * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IFaxIncomingJobs * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IFaxIncomingJobs * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IFaxIncomingJobs * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IFaxIncomingJobs * This,
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
        
        DECLSPEC_XFGVIRT(IFaxIncomingJobs, get__NewEnum)
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *get__NewEnum )( 
            __RPC__in IFaxIncomingJobs * This,
            /* [retval][out] */ __RPC__deref_out_opt IUnknown **ppUnk);
        
        DECLSPEC_XFGVIRT(IFaxIncomingJobs, get_Item)
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_Item )( 
            __RPC__in IFaxIncomingJobs * This,
            /* [in] */ VARIANT vIndex,
            /* [retval][out] */ __RPC__deref_out_opt IFaxIncomingJob **pFaxIncomingJob);
        
        DECLSPEC_XFGVIRT(IFaxIncomingJobs, get_Count)
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_Count )( 
            __RPC__in IFaxIncomingJobs * This,
            /* [retval][out] */ __RPC__out long *plCount);
        
        END_INTERFACE
    } IFaxIncomingJobsVtbl;

    interface IFaxIncomingJobs
    {
        CONST_VTBL struct IFaxIncomingJobsVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IFaxIncomingJobs_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IFaxIncomingJobs_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IFaxIncomingJobs_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IFaxIncomingJobs_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IFaxIncomingJobs_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IFaxIncomingJobs_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IFaxIncomingJobs_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IFaxIncomingJobs_get__NewEnum(This,ppUnk)	\
    ( (This)->lpVtbl -> get__NewEnum(This,ppUnk) ) 

#define IFaxIncomingJobs_get_Item(This,vIndex,pFaxIncomingJob)	\
    ( (This)->lpVtbl -> get_Item(This,vIndex,pFaxIncomingJob) ) 

#define IFaxIncomingJobs_get_Count(This,plCount)	\
    ( (This)->lpVtbl -> get_Count(This,plCount) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IFaxIncomingJobs_INTERFACE_DEFINED__ */


#ifndef __IFaxIncomingJob_INTERFACE_DEFINED__
#define __IFaxIncomingJob_INTERFACE_DEFINED__

/* interface IFaxIncomingJob */
/* [unique][helpstring][dual][uuid][object] */ 


EXTERN_C const IID IID_IFaxIncomingJob;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("207529E6-654A-4916-9F88-4D232EE8A107")
    IFaxIncomingJob : public IDispatch
    {
    public:
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Size( 
            /* [retval][out] */ __RPC__out long *plSize) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Id( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrId) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_CurrentPage( 
            /* [retval][out] */ __RPC__out long *plCurrentPage) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_DeviceId( 
            /* [retval][out] */ __RPC__out long *plDeviceId) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Status( 
            /* [retval][out] */ __RPC__out FAX_JOB_STATUS_ENUM *pStatus) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_ExtendedStatusCode( 
            /* [retval][out] */ __RPC__out FAX_JOB_EXTENDED_STATUS_ENUM *pExtendedStatusCode) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_ExtendedStatus( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrExtendedStatus) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_AvailableOperations( 
            /* [retval][out] */ __RPC__out FAX_JOB_OPERATIONS_ENUM *pAvailableOperations) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Retries( 
            /* [retval][out] */ __RPC__out long *plRetries) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_TransmissionStart( 
            /* [retval][out] */ __RPC__out DATE *pdateTransmissionStart) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_TransmissionEnd( 
            /* [retval][out] */ __RPC__out DATE *pdateTransmissionEnd) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_CSID( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrCSID) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_TSID( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrTSID) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_CallerId( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrCallerId) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_RoutingInformation( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrRoutingInformation) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_JobType( 
            /* [retval][out] */ __RPC__out FAX_JOB_TYPE_ENUM *pJobType) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE Cancel( void) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE Refresh( void) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE CopyTiff( 
            /* [in] */ __RPC__in BSTR bstrTiffPath) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IFaxIncomingJobVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IFaxIncomingJob * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IFaxIncomingJob * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IFaxIncomingJob * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IFaxIncomingJob * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IFaxIncomingJob * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IFaxIncomingJob * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IFaxIncomingJob * This,
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
        
        DECLSPEC_XFGVIRT(IFaxIncomingJob, get_Size)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Size )( 
            __RPC__in IFaxIncomingJob * This,
            /* [retval][out] */ __RPC__out long *plSize);
        
        DECLSPEC_XFGVIRT(IFaxIncomingJob, get_Id)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Id )( 
            __RPC__in IFaxIncomingJob * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrId);
        
        DECLSPEC_XFGVIRT(IFaxIncomingJob, get_CurrentPage)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_CurrentPage )( 
            __RPC__in IFaxIncomingJob * This,
            /* [retval][out] */ __RPC__out long *plCurrentPage);
        
        DECLSPEC_XFGVIRT(IFaxIncomingJob, get_DeviceId)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_DeviceId )( 
            __RPC__in IFaxIncomingJob * This,
            /* [retval][out] */ __RPC__out long *plDeviceId);
        
        DECLSPEC_XFGVIRT(IFaxIncomingJob, get_Status)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Status )( 
            __RPC__in IFaxIncomingJob * This,
            /* [retval][out] */ __RPC__out FAX_JOB_STATUS_ENUM *pStatus);
        
        DECLSPEC_XFGVIRT(IFaxIncomingJob, get_ExtendedStatusCode)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_ExtendedStatusCode )( 
            __RPC__in IFaxIncomingJob * This,
            /* [retval][out] */ __RPC__out FAX_JOB_EXTENDED_STATUS_ENUM *pExtendedStatusCode);
        
        DECLSPEC_XFGVIRT(IFaxIncomingJob, get_ExtendedStatus)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_ExtendedStatus )( 
            __RPC__in IFaxIncomingJob * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrExtendedStatus);
        
        DECLSPEC_XFGVIRT(IFaxIncomingJob, get_AvailableOperations)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_AvailableOperations )( 
            __RPC__in IFaxIncomingJob * This,
            /* [retval][out] */ __RPC__out FAX_JOB_OPERATIONS_ENUM *pAvailableOperations);
        
        DECLSPEC_XFGVIRT(IFaxIncomingJob, get_Retries)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Retries )( 
            __RPC__in IFaxIncomingJob * This,
            /* [retval][out] */ __RPC__out long *plRetries);
        
        DECLSPEC_XFGVIRT(IFaxIncomingJob, get_TransmissionStart)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_TransmissionStart )( 
            __RPC__in IFaxIncomingJob * This,
            /* [retval][out] */ __RPC__out DATE *pdateTransmissionStart);
        
        DECLSPEC_XFGVIRT(IFaxIncomingJob, get_TransmissionEnd)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_TransmissionEnd )( 
            __RPC__in IFaxIncomingJob * This,
            /* [retval][out] */ __RPC__out DATE *pdateTransmissionEnd);
        
        DECLSPEC_XFGVIRT(IFaxIncomingJob, get_CSID)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_CSID )( 
            __RPC__in IFaxIncomingJob * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrCSID);
        
        DECLSPEC_XFGVIRT(IFaxIncomingJob, get_TSID)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_TSID )( 
            __RPC__in IFaxIncomingJob * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrTSID);
        
        DECLSPEC_XFGVIRT(IFaxIncomingJob, get_CallerId)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_CallerId )( 
            __RPC__in IFaxIncomingJob * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrCallerId);
        
        DECLSPEC_XFGVIRT(IFaxIncomingJob, get_RoutingInformation)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_RoutingInformation )( 
            __RPC__in IFaxIncomingJob * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrRoutingInformation);
        
        DECLSPEC_XFGVIRT(IFaxIncomingJob, get_JobType)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_JobType )( 
            __RPC__in IFaxIncomingJob * This,
            /* [retval][out] */ __RPC__out FAX_JOB_TYPE_ENUM *pJobType);
        
        DECLSPEC_XFGVIRT(IFaxIncomingJob, Cancel)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *Cancel )( 
            __RPC__in IFaxIncomingJob * This);
        
        DECLSPEC_XFGVIRT(IFaxIncomingJob, Refresh)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *Refresh )( 
            __RPC__in IFaxIncomingJob * This);
        
        DECLSPEC_XFGVIRT(IFaxIncomingJob, CopyTiff)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *CopyTiff )( 
            __RPC__in IFaxIncomingJob * This,
            /* [in] */ __RPC__in BSTR bstrTiffPath);
        
        END_INTERFACE
    } IFaxIncomingJobVtbl;

    interface IFaxIncomingJob
    {
        CONST_VTBL struct IFaxIncomingJobVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IFaxIncomingJob_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IFaxIncomingJob_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IFaxIncomingJob_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IFaxIncomingJob_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IFaxIncomingJob_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IFaxIncomingJob_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IFaxIncomingJob_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IFaxIncomingJob_get_Size(This,plSize)	\
    ( (This)->lpVtbl -> get_Size(This,plSize) ) 

#define IFaxIncomingJob_get_Id(This,pbstrId)	\
    ( (This)->lpVtbl -> get_Id(This,pbstrId) ) 

#define IFaxIncomingJob_get_CurrentPage(This,plCurrentPage)	\
    ( (This)->lpVtbl -> get_CurrentPage(This,plCurrentPage) ) 

#define IFaxIncomingJob_get_DeviceId(This,plDeviceId)	\
    ( (This)->lpVtbl -> get_DeviceId(This,plDeviceId) ) 

#define IFaxIncomingJob_get_Status(This,pStatus)	\
    ( (This)->lpVtbl -> get_Status(This,pStatus) ) 

#define IFaxIncomingJob_get_ExtendedStatusCode(This,pExtendedStatusCode)	\
    ( (This)->lpVtbl -> get_ExtendedStatusCode(This,pExtendedStatusCode) ) 

#define IFaxIncomingJob_get_ExtendedStatus(This,pbstrExtendedStatus)	\
    ( (This)->lpVtbl -> get_ExtendedStatus(This,pbstrExtendedStatus) ) 

#define IFaxIncomingJob_get_AvailableOperations(This,pAvailableOperations)	\
    ( (This)->lpVtbl -> get_AvailableOperations(This,pAvailableOperations) ) 

#define IFaxIncomingJob_get_Retries(This,plRetries)	\
    ( (This)->lpVtbl -> get_Retries(This,plRetries) ) 

#define IFaxIncomingJob_get_TransmissionStart(This,pdateTransmissionStart)	\
    ( (This)->lpVtbl -> get_TransmissionStart(This,pdateTransmissionStart) ) 

#define IFaxIncomingJob_get_TransmissionEnd(This,pdateTransmissionEnd)	\
    ( (This)->lpVtbl -> get_TransmissionEnd(This,pdateTransmissionEnd) ) 

#define IFaxIncomingJob_get_CSID(This,pbstrCSID)	\
    ( (This)->lpVtbl -> get_CSID(This,pbstrCSID) ) 

#define IFaxIncomingJob_get_TSID(This,pbstrTSID)	\
    ( (This)->lpVtbl -> get_TSID(This,pbstrTSID) ) 

#define IFaxIncomingJob_get_CallerId(This,pbstrCallerId)	\
    ( (This)->lpVtbl -> get_CallerId(This,pbstrCallerId) ) 

#define IFaxIncomingJob_get_RoutingInformation(This,pbstrRoutingInformation)	\
    ( (This)->lpVtbl -> get_RoutingInformation(This,pbstrRoutingInformation) ) 

#define IFaxIncomingJob_get_JobType(This,pJobType)	\
    ( (This)->lpVtbl -> get_JobType(This,pJobType) ) 

#define IFaxIncomingJob_Cancel(This)	\
    ( (This)->lpVtbl -> Cancel(This) ) 

#define IFaxIncomingJob_Refresh(This)	\
    ( (This)->lpVtbl -> Refresh(This) ) 

#define IFaxIncomingJob_CopyTiff(This,bstrTiffPath)	\
    ( (This)->lpVtbl -> CopyTiff(This,bstrTiffPath) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IFaxIncomingJob_INTERFACE_DEFINED__ */


#ifndef __IFaxDeviceProvider_INTERFACE_DEFINED__
#define __IFaxDeviceProvider_INTERFACE_DEFINED__

/* interface IFaxDeviceProvider */
/* [unique][helpstring][dual][uuid][object] */ 

typedef 
enum FAX_PROVIDER_STATUS_ENUM
    {
        fpsSUCCESS	= 0,
        fpsSERVER_ERROR	= ( fpsSUCCESS + 1 ) ,
        fpsBAD_GUID	= ( fpsSERVER_ERROR + 1 ) ,
        fpsBAD_VERSION	= ( fpsBAD_GUID + 1 ) ,
        fpsCANT_LOAD	= ( fpsBAD_VERSION + 1 ) ,
        fpsCANT_LINK	= ( fpsCANT_LOAD + 1 ) ,
        fpsCANT_INIT	= ( fpsCANT_LINK + 1 ) 
    } 	FAX_PROVIDER_STATUS_ENUM;


EXTERN_C const IID IID_IFaxDeviceProvider;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("290EAC63-83EC-449C-8417-F148DF8C682A")
    IFaxDeviceProvider : public IDispatch
    {
    public:
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_FriendlyName( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrFriendlyName) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_ImageName( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrImageName) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_UniqueName( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrUniqueName) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_TapiProviderName( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrTapiProviderName) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_MajorVersion( 
            /* [retval][out] */ __RPC__out long *plMajorVersion) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_MinorVersion( 
            /* [retval][out] */ __RPC__out long *plMinorVersion) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_MajorBuild( 
            /* [retval][out] */ __RPC__out long *plMajorBuild) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_MinorBuild( 
            /* [retval][out] */ __RPC__out long *plMinorBuild) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Debug( 
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pbDebug) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Status( 
            /* [retval][out] */ __RPC__out FAX_PROVIDER_STATUS_ENUM *pStatus) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_InitErrorCode( 
            /* [retval][out] */ __RPC__out long *plInitErrorCode) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_DeviceIds( 
            /* [retval][out] */ __RPC__out VARIANT *pvDeviceIds) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IFaxDeviceProviderVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IFaxDeviceProvider * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IFaxDeviceProvider * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IFaxDeviceProvider * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IFaxDeviceProvider * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IFaxDeviceProvider * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IFaxDeviceProvider * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IFaxDeviceProvider * This,
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
        
        DECLSPEC_XFGVIRT(IFaxDeviceProvider, get_FriendlyName)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_FriendlyName )( 
            __RPC__in IFaxDeviceProvider * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrFriendlyName);
        
        DECLSPEC_XFGVIRT(IFaxDeviceProvider, get_ImageName)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_ImageName )( 
            __RPC__in IFaxDeviceProvider * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrImageName);
        
        DECLSPEC_XFGVIRT(IFaxDeviceProvider, get_UniqueName)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_UniqueName )( 
            __RPC__in IFaxDeviceProvider * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrUniqueName);
        
        DECLSPEC_XFGVIRT(IFaxDeviceProvider, get_TapiProviderName)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_TapiProviderName )( 
            __RPC__in IFaxDeviceProvider * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrTapiProviderName);
        
        DECLSPEC_XFGVIRT(IFaxDeviceProvider, get_MajorVersion)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_MajorVersion )( 
            __RPC__in IFaxDeviceProvider * This,
            /* [retval][out] */ __RPC__out long *plMajorVersion);
        
        DECLSPEC_XFGVIRT(IFaxDeviceProvider, get_MinorVersion)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_MinorVersion )( 
            __RPC__in IFaxDeviceProvider * This,
            /* [retval][out] */ __RPC__out long *plMinorVersion);
        
        DECLSPEC_XFGVIRT(IFaxDeviceProvider, get_MajorBuild)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_MajorBuild )( 
            __RPC__in IFaxDeviceProvider * This,
            /* [retval][out] */ __RPC__out long *plMajorBuild);
        
        DECLSPEC_XFGVIRT(IFaxDeviceProvider, get_MinorBuild)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_MinorBuild )( 
            __RPC__in IFaxDeviceProvider * This,
            /* [retval][out] */ __RPC__out long *plMinorBuild);
        
        DECLSPEC_XFGVIRT(IFaxDeviceProvider, get_Debug)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Debug )( 
            __RPC__in IFaxDeviceProvider * This,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pbDebug);
        
        DECLSPEC_XFGVIRT(IFaxDeviceProvider, get_Status)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Status )( 
            __RPC__in IFaxDeviceProvider * This,
            /* [retval][out] */ __RPC__out FAX_PROVIDER_STATUS_ENUM *pStatus);
        
        DECLSPEC_XFGVIRT(IFaxDeviceProvider, get_InitErrorCode)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_InitErrorCode )( 
            __RPC__in IFaxDeviceProvider * This,
            /* [retval][out] */ __RPC__out long *plInitErrorCode);
        
        DECLSPEC_XFGVIRT(IFaxDeviceProvider, get_DeviceIds)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_DeviceIds )( 
            __RPC__in IFaxDeviceProvider * This,
            /* [retval][out] */ __RPC__out VARIANT *pvDeviceIds);
        
        END_INTERFACE
    } IFaxDeviceProviderVtbl;

    interface IFaxDeviceProvider
    {
        CONST_VTBL struct IFaxDeviceProviderVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IFaxDeviceProvider_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IFaxDeviceProvider_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IFaxDeviceProvider_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IFaxDeviceProvider_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IFaxDeviceProvider_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IFaxDeviceProvider_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IFaxDeviceProvider_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IFaxDeviceProvider_get_FriendlyName(This,pbstrFriendlyName)	\
    ( (This)->lpVtbl -> get_FriendlyName(This,pbstrFriendlyName) ) 

#define IFaxDeviceProvider_get_ImageName(This,pbstrImageName)	\
    ( (This)->lpVtbl -> get_ImageName(This,pbstrImageName) ) 

#define IFaxDeviceProvider_get_UniqueName(This,pbstrUniqueName)	\
    ( (This)->lpVtbl -> get_UniqueName(This,pbstrUniqueName) ) 

#define IFaxDeviceProvider_get_TapiProviderName(This,pbstrTapiProviderName)	\
    ( (This)->lpVtbl -> get_TapiProviderName(This,pbstrTapiProviderName) ) 

#define IFaxDeviceProvider_get_MajorVersion(This,plMajorVersion)	\
    ( (This)->lpVtbl -> get_MajorVersion(This,plMajorVersion) ) 

#define IFaxDeviceProvider_get_MinorVersion(This,plMinorVersion)	\
    ( (This)->lpVtbl -> get_MinorVersion(This,plMinorVersion) ) 

#define IFaxDeviceProvider_get_MajorBuild(This,plMajorBuild)	\
    ( (This)->lpVtbl -> get_MajorBuild(This,plMajorBuild) ) 

#define IFaxDeviceProvider_get_MinorBuild(This,plMinorBuild)	\
    ( (This)->lpVtbl -> get_MinorBuild(This,plMinorBuild) ) 

#define IFaxDeviceProvider_get_Debug(This,pbDebug)	\
    ( (This)->lpVtbl -> get_Debug(This,pbDebug) ) 

#define IFaxDeviceProvider_get_Status(This,pStatus)	\
    ( (This)->lpVtbl -> get_Status(This,pStatus) ) 

#define IFaxDeviceProvider_get_InitErrorCode(This,plInitErrorCode)	\
    ( (This)->lpVtbl -> get_InitErrorCode(This,plInitErrorCode) ) 

#define IFaxDeviceProvider_get_DeviceIds(This,pvDeviceIds)	\
    ( (This)->lpVtbl -> get_DeviceIds(This,pvDeviceIds) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IFaxDeviceProvider_INTERFACE_DEFINED__ */


#ifndef __IFaxDevice_INTERFACE_DEFINED__
#define __IFaxDevice_INTERFACE_DEFINED__

/* interface IFaxDevice */
/* [unique][helpstring][dual][uuid][object] */ 

typedef 
enum FAX_DEVICE_RECEIVE_MODE_ENUM
    {
        fdrmNO_ANSWER	= 0,
        fdrmAUTO_ANSWER	= ( fdrmNO_ANSWER + 1 ) ,
        fdrmMANUAL_ANSWER	= ( fdrmAUTO_ANSWER + 1 ) 
    } 	FAX_DEVICE_RECEIVE_MODE_ENUM;


EXTERN_C const IID IID_IFaxDevice;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("49306C59-B52E-4867-9DF4-CA5841C956D0")
    IFaxDevice : public IDispatch
    {
    public:
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Id( 
            /* [retval][out] */ __RPC__out long *plId) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_DeviceName( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrDeviceName) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_ProviderUniqueName( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrProviderUniqueName) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_PoweredOff( 
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pbPoweredOff) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_ReceivingNow( 
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pbReceivingNow) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_SendingNow( 
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pbSendingNow) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_UsedRoutingMethods( 
            /* [retval][out] */ __RPC__out VARIANT *pvUsedRoutingMethods) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Description( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrDescription) = 0;
        
        virtual /* [helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_Description( 
            /* [in] */ __RPC__in BSTR bstrDescription) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_SendEnabled( 
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pbSendEnabled) = 0;
        
        virtual /* [helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_SendEnabled( 
            /* [in] */ VARIANT_BOOL bSendEnabled) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_ReceiveMode( 
            /* [retval][out] */ __RPC__out FAX_DEVICE_RECEIVE_MODE_ENUM *pReceiveMode) = 0;
        
        virtual /* [helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_ReceiveMode( 
            /* [in] */ FAX_DEVICE_RECEIVE_MODE_ENUM ReceiveMode) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_RingsBeforeAnswer( 
            /* [retval][out] */ __RPC__out long *plRingsBeforeAnswer) = 0;
        
        virtual /* [helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_RingsBeforeAnswer( 
            /* [in] */ long lRingsBeforeAnswer) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_CSID( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrCSID) = 0;
        
        virtual /* [helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_CSID( 
            /* [in] */ __RPC__in BSTR bstrCSID) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_TSID( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrTSID) = 0;
        
        virtual /* [helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_TSID( 
            /* [in] */ __RPC__in BSTR bstrTSID) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE Refresh( void) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE Save( void) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE GetExtensionProperty( 
            /* [in] */ __RPC__in BSTR bstrGUID,
            /* [retval][out] */ __RPC__out VARIANT *pvProperty) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE SetExtensionProperty( 
            /* [in] */ __RPC__in BSTR bstrGUID,
            /* [in] */ VARIANT vProperty) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE UseRoutingMethod( 
            /* [in] */ __RPC__in BSTR bstrMethodGUID,
            /* [in] */ VARIANT_BOOL bUse) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_RingingNow( 
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pbRingingNow) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE AnswerCall( void) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IFaxDeviceVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IFaxDevice * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IFaxDevice * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IFaxDevice * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IFaxDevice * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IFaxDevice * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IFaxDevice * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IFaxDevice * This,
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
        
        DECLSPEC_XFGVIRT(IFaxDevice, get_Id)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Id )( 
            __RPC__in IFaxDevice * This,
            /* [retval][out] */ __RPC__out long *plId);
        
        DECLSPEC_XFGVIRT(IFaxDevice, get_DeviceName)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_DeviceName )( 
            __RPC__in IFaxDevice * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrDeviceName);
        
        DECLSPEC_XFGVIRT(IFaxDevice, get_ProviderUniqueName)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_ProviderUniqueName )( 
            __RPC__in IFaxDevice * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrProviderUniqueName);
        
        DECLSPEC_XFGVIRT(IFaxDevice, get_PoweredOff)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_PoweredOff )( 
            __RPC__in IFaxDevice * This,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pbPoweredOff);
        
        DECLSPEC_XFGVIRT(IFaxDevice, get_ReceivingNow)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_ReceivingNow )( 
            __RPC__in IFaxDevice * This,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pbReceivingNow);
        
        DECLSPEC_XFGVIRT(IFaxDevice, get_SendingNow)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_SendingNow )( 
            __RPC__in IFaxDevice * This,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pbSendingNow);
        
        DECLSPEC_XFGVIRT(IFaxDevice, get_UsedRoutingMethods)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_UsedRoutingMethods )( 
            __RPC__in IFaxDevice * This,
            /* [retval][out] */ __RPC__out VARIANT *pvUsedRoutingMethods);
        
        DECLSPEC_XFGVIRT(IFaxDevice, get_Description)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Description )( 
            __RPC__in IFaxDevice * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrDescription);
        
        DECLSPEC_XFGVIRT(IFaxDevice, put_Description)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_Description )( 
            __RPC__in IFaxDevice * This,
            /* [in] */ __RPC__in BSTR bstrDescription);
        
        DECLSPEC_XFGVIRT(IFaxDevice, get_SendEnabled)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_SendEnabled )( 
            __RPC__in IFaxDevice * This,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pbSendEnabled);
        
        DECLSPEC_XFGVIRT(IFaxDevice, put_SendEnabled)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_SendEnabled )( 
            __RPC__in IFaxDevice * This,
            /* [in] */ VARIANT_BOOL bSendEnabled);
        
        DECLSPEC_XFGVIRT(IFaxDevice, get_ReceiveMode)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_ReceiveMode )( 
            __RPC__in IFaxDevice * This,
            /* [retval][out] */ __RPC__out FAX_DEVICE_RECEIVE_MODE_ENUM *pReceiveMode);
        
        DECLSPEC_XFGVIRT(IFaxDevice, put_ReceiveMode)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_ReceiveMode )( 
            __RPC__in IFaxDevice * This,
            /* [in] */ FAX_DEVICE_RECEIVE_MODE_ENUM ReceiveMode);
        
        DECLSPEC_XFGVIRT(IFaxDevice, get_RingsBeforeAnswer)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_RingsBeforeAnswer )( 
            __RPC__in IFaxDevice * This,
            /* [retval][out] */ __RPC__out long *plRingsBeforeAnswer);
        
        DECLSPEC_XFGVIRT(IFaxDevice, put_RingsBeforeAnswer)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_RingsBeforeAnswer )( 
            __RPC__in IFaxDevice * This,
            /* [in] */ long lRingsBeforeAnswer);
        
        DECLSPEC_XFGVIRT(IFaxDevice, get_CSID)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_CSID )( 
            __RPC__in IFaxDevice * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrCSID);
        
        DECLSPEC_XFGVIRT(IFaxDevice, put_CSID)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_CSID )( 
            __RPC__in IFaxDevice * This,
            /* [in] */ __RPC__in BSTR bstrCSID);
        
        DECLSPEC_XFGVIRT(IFaxDevice, get_TSID)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_TSID )( 
            __RPC__in IFaxDevice * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrTSID);
        
        DECLSPEC_XFGVIRT(IFaxDevice, put_TSID)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_TSID )( 
            __RPC__in IFaxDevice * This,
            /* [in] */ __RPC__in BSTR bstrTSID);
        
        DECLSPEC_XFGVIRT(IFaxDevice, Refresh)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *Refresh )( 
            __RPC__in IFaxDevice * This);
        
        DECLSPEC_XFGVIRT(IFaxDevice, Save)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *Save )( 
            __RPC__in IFaxDevice * This);
        
        DECLSPEC_XFGVIRT(IFaxDevice, GetExtensionProperty)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *GetExtensionProperty )( 
            __RPC__in IFaxDevice * This,
            /* [in] */ __RPC__in BSTR bstrGUID,
            /* [retval][out] */ __RPC__out VARIANT *pvProperty);
        
        DECLSPEC_XFGVIRT(IFaxDevice, SetExtensionProperty)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *SetExtensionProperty )( 
            __RPC__in IFaxDevice * This,
            /* [in] */ __RPC__in BSTR bstrGUID,
            /* [in] */ VARIANT vProperty);
        
        DECLSPEC_XFGVIRT(IFaxDevice, UseRoutingMethod)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *UseRoutingMethod )( 
            __RPC__in IFaxDevice * This,
            /* [in] */ __RPC__in BSTR bstrMethodGUID,
            /* [in] */ VARIANT_BOOL bUse);
        
        DECLSPEC_XFGVIRT(IFaxDevice, get_RingingNow)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_RingingNow )( 
            __RPC__in IFaxDevice * This,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pbRingingNow);
        
        DECLSPEC_XFGVIRT(IFaxDevice, AnswerCall)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *AnswerCall )( 
            __RPC__in IFaxDevice * This);
        
        END_INTERFACE
    } IFaxDeviceVtbl;

    interface IFaxDevice
    {
        CONST_VTBL struct IFaxDeviceVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IFaxDevice_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IFaxDevice_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IFaxDevice_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IFaxDevice_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IFaxDevice_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IFaxDevice_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IFaxDevice_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IFaxDevice_get_Id(This,plId)	\
    ( (This)->lpVtbl -> get_Id(This,plId) ) 

#define IFaxDevice_get_DeviceName(This,pbstrDeviceName)	\
    ( (This)->lpVtbl -> get_DeviceName(This,pbstrDeviceName) ) 

#define IFaxDevice_get_ProviderUniqueName(This,pbstrProviderUniqueName)	\
    ( (This)->lpVtbl -> get_ProviderUniqueName(This,pbstrProviderUniqueName) ) 

#define IFaxDevice_get_PoweredOff(This,pbPoweredOff)	\
    ( (This)->lpVtbl -> get_PoweredOff(This,pbPoweredOff) ) 

#define IFaxDevice_get_ReceivingNow(This,pbReceivingNow)	\
    ( (This)->lpVtbl -> get_ReceivingNow(This,pbReceivingNow) ) 

#define IFaxDevice_get_SendingNow(This,pbSendingNow)	\
    ( (This)->lpVtbl -> get_SendingNow(This,pbSendingNow) ) 

#define IFaxDevice_get_UsedRoutingMethods(This,pvUsedRoutingMethods)	\
    ( (This)->lpVtbl -> get_UsedRoutingMethods(This,pvUsedRoutingMethods) ) 

#define IFaxDevice_get_Description(This,pbstrDescription)	\
    ( (This)->lpVtbl -> get_Description(This,pbstrDescription) ) 

#define IFaxDevice_put_Description(This,bstrDescription)	\
    ( (This)->lpVtbl -> put_Description(This,bstrDescription) ) 

#define IFaxDevice_get_SendEnabled(This,pbSendEnabled)	\
    ( (This)->lpVtbl -> get_SendEnabled(This,pbSendEnabled) ) 

#define IFaxDevice_put_SendEnabled(This,bSendEnabled)	\
    ( (This)->lpVtbl -> put_SendEnabled(This,bSendEnabled) ) 

#define IFaxDevice_get_ReceiveMode(This,pReceiveMode)	\
    ( (This)->lpVtbl -> get_ReceiveMode(This,pReceiveMode) ) 

#define IFaxDevice_put_ReceiveMode(This,ReceiveMode)	\
    ( (This)->lpVtbl -> put_ReceiveMode(This,ReceiveMode) ) 

#define IFaxDevice_get_RingsBeforeAnswer(This,plRingsBeforeAnswer)	\
    ( (This)->lpVtbl -> get_RingsBeforeAnswer(This,plRingsBeforeAnswer) ) 

#define IFaxDevice_put_RingsBeforeAnswer(This,lRingsBeforeAnswer)	\
    ( (This)->lpVtbl -> put_RingsBeforeAnswer(This,lRingsBeforeAnswer) ) 

#define IFaxDevice_get_CSID(This,pbstrCSID)	\
    ( (This)->lpVtbl -> get_CSID(This,pbstrCSID) ) 

#define IFaxDevice_put_CSID(This,bstrCSID)	\
    ( (This)->lpVtbl -> put_CSID(This,bstrCSID) ) 

#define IFaxDevice_get_TSID(This,pbstrTSID)	\
    ( (This)->lpVtbl -> get_TSID(This,pbstrTSID) ) 

#define IFaxDevice_put_TSID(This,bstrTSID)	\
    ( (This)->lpVtbl -> put_TSID(This,bstrTSID) ) 

#define IFaxDevice_Refresh(This)	\
    ( (This)->lpVtbl -> Refresh(This) ) 

#define IFaxDevice_Save(This)	\
    ( (This)->lpVtbl -> Save(This) ) 

#define IFaxDevice_GetExtensionProperty(This,bstrGUID,pvProperty)	\
    ( (This)->lpVtbl -> GetExtensionProperty(This,bstrGUID,pvProperty) ) 

#define IFaxDevice_SetExtensionProperty(This,bstrGUID,vProperty)	\
    ( (This)->lpVtbl -> SetExtensionProperty(This,bstrGUID,vProperty) ) 

#define IFaxDevice_UseRoutingMethod(This,bstrMethodGUID,bUse)	\
    ( (This)->lpVtbl -> UseRoutingMethod(This,bstrMethodGUID,bUse) ) 

#define IFaxDevice_get_RingingNow(This,pbRingingNow)	\
    ( (This)->lpVtbl -> get_RingingNow(This,pbRingingNow) ) 

#define IFaxDevice_AnswerCall(This)	\
    ( (This)->lpVtbl -> AnswerCall(This) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IFaxDevice_INTERFACE_DEFINED__ */


#ifndef __IFaxActivityLogging_INTERFACE_DEFINED__
#define __IFaxActivityLogging_INTERFACE_DEFINED__

/* interface IFaxActivityLogging */
/* [unique][helpstring][dual][uuid][object] */ 


EXTERN_C const IID IID_IFaxActivityLogging;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("1E29078B-5A69-497B-9592-49B7E7FADDB5")
    IFaxActivityLogging : public IDispatch
    {
    public:
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_LogIncoming( 
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pbLogIncoming) = 0;
        
        virtual /* [helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_LogIncoming( 
            /* [in] */ VARIANT_BOOL bLogIncoming) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_LogOutgoing( 
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pbLogOutgoing) = 0;
        
        virtual /* [helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_LogOutgoing( 
            /* [in] */ VARIANT_BOOL bLogOutgoing) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_DatabasePath( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrDatabasePath) = 0;
        
        virtual /* [helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_DatabasePath( 
            /* [in] */ __RPC__in BSTR bstrDatabasePath) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE Refresh( void) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE Save( void) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IFaxActivityLoggingVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IFaxActivityLogging * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IFaxActivityLogging * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IFaxActivityLogging * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IFaxActivityLogging * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IFaxActivityLogging * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IFaxActivityLogging * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IFaxActivityLogging * This,
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
        
        DECLSPEC_XFGVIRT(IFaxActivityLogging, get_LogIncoming)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_LogIncoming )( 
            __RPC__in IFaxActivityLogging * This,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pbLogIncoming);
        
        DECLSPEC_XFGVIRT(IFaxActivityLogging, put_LogIncoming)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_LogIncoming )( 
            __RPC__in IFaxActivityLogging * This,
            /* [in] */ VARIANT_BOOL bLogIncoming);
        
        DECLSPEC_XFGVIRT(IFaxActivityLogging, get_LogOutgoing)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_LogOutgoing )( 
            __RPC__in IFaxActivityLogging * This,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pbLogOutgoing);
        
        DECLSPEC_XFGVIRT(IFaxActivityLogging, put_LogOutgoing)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_LogOutgoing )( 
            __RPC__in IFaxActivityLogging * This,
            /* [in] */ VARIANT_BOOL bLogOutgoing);
        
        DECLSPEC_XFGVIRT(IFaxActivityLogging, get_DatabasePath)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_DatabasePath )( 
            __RPC__in IFaxActivityLogging * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrDatabasePath);
        
        DECLSPEC_XFGVIRT(IFaxActivityLogging, put_DatabasePath)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_DatabasePath )( 
            __RPC__in IFaxActivityLogging * This,
            /* [in] */ __RPC__in BSTR bstrDatabasePath);
        
        DECLSPEC_XFGVIRT(IFaxActivityLogging, Refresh)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *Refresh )( 
            __RPC__in IFaxActivityLogging * This);
        
        DECLSPEC_XFGVIRT(IFaxActivityLogging, Save)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *Save )( 
            __RPC__in IFaxActivityLogging * This);
        
        END_INTERFACE
    } IFaxActivityLoggingVtbl;

    interface IFaxActivityLogging
    {
        CONST_VTBL struct IFaxActivityLoggingVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IFaxActivityLogging_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IFaxActivityLogging_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IFaxActivityLogging_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IFaxActivityLogging_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IFaxActivityLogging_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IFaxActivityLogging_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IFaxActivityLogging_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IFaxActivityLogging_get_LogIncoming(This,pbLogIncoming)	\
    ( (This)->lpVtbl -> get_LogIncoming(This,pbLogIncoming) ) 

#define IFaxActivityLogging_put_LogIncoming(This,bLogIncoming)	\
    ( (This)->lpVtbl -> put_LogIncoming(This,bLogIncoming) ) 

#define IFaxActivityLogging_get_LogOutgoing(This,pbLogOutgoing)	\
    ( (This)->lpVtbl -> get_LogOutgoing(This,pbLogOutgoing) ) 

#define IFaxActivityLogging_put_LogOutgoing(This,bLogOutgoing)	\
    ( (This)->lpVtbl -> put_LogOutgoing(This,bLogOutgoing) ) 

#define IFaxActivityLogging_get_DatabasePath(This,pbstrDatabasePath)	\
    ( (This)->lpVtbl -> get_DatabasePath(This,pbstrDatabasePath) ) 

#define IFaxActivityLogging_put_DatabasePath(This,bstrDatabasePath)	\
    ( (This)->lpVtbl -> put_DatabasePath(This,bstrDatabasePath) ) 

#define IFaxActivityLogging_Refresh(This)	\
    ( (This)->lpVtbl -> Refresh(This) ) 

#define IFaxActivityLogging_Save(This)	\
    ( (This)->lpVtbl -> Save(This) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IFaxActivityLogging_INTERFACE_DEFINED__ */


#ifndef __IFaxEventLogging_INTERFACE_DEFINED__
#define __IFaxEventLogging_INTERFACE_DEFINED__

/* interface IFaxEventLogging */
/* [unique][helpstring][dual][uuid][object] */ 

typedef 
enum FAX_LOG_LEVEL_ENUM
    {
        fllNONE	= 0,
        fllMIN	= ( fllNONE + 1 ) ,
        fllMED	= ( fllMIN + 1 ) ,
        fllMAX	= ( fllMED + 1 ) 
    } 	FAX_LOG_LEVEL_ENUM;


EXTERN_C const IID IID_IFaxEventLogging;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("0880D965-20E8-42E4-8E17-944F192CAAD4")
    IFaxEventLogging : public IDispatch
    {
    public:
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_InitEventsLevel( 
            /* [retval][out] */ __RPC__out FAX_LOG_LEVEL_ENUM *pInitEventLevel) = 0;
        
        virtual /* [helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_InitEventsLevel( 
            /* [in] */ FAX_LOG_LEVEL_ENUM InitEventLevel) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_InboundEventsLevel( 
            /* [retval][out] */ __RPC__out FAX_LOG_LEVEL_ENUM *pInboundEventLevel) = 0;
        
        virtual /* [helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_InboundEventsLevel( 
            /* [in] */ FAX_LOG_LEVEL_ENUM InboundEventLevel) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_OutboundEventsLevel( 
            /* [retval][out] */ __RPC__out FAX_LOG_LEVEL_ENUM *pOutboundEventLevel) = 0;
        
        virtual /* [helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_OutboundEventsLevel( 
            /* [in] */ FAX_LOG_LEVEL_ENUM OutboundEventLevel) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_GeneralEventsLevel( 
            /* [retval][out] */ __RPC__out FAX_LOG_LEVEL_ENUM *pGeneralEventLevel) = 0;
        
        virtual /* [helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_GeneralEventsLevel( 
            /* [in] */ FAX_LOG_LEVEL_ENUM GeneralEventLevel) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE Refresh( void) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE Save( void) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IFaxEventLoggingVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IFaxEventLogging * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IFaxEventLogging * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IFaxEventLogging * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IFaxEventLogging * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IFaxEventLogging * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IFaxEventLogging * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IFaxEventLogging * This,
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
        
        DECLSPEC_XFGVIRT(IFaxEventLogging, get_InitEventsLevel)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_InitEventsLevel )( 
            __RPC__in IFaxEventLogging * This,
            /* [retval][out] */ __RPC__out FAX_LOG_LEVEL_ENUM *pInitEventLevel);
        
        DECLSPEC_XFGVIRT(IFaxEventLogging, put_InitEventsLevel)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_InitEventsLevel )( 
            __RPC__in IFaxEventLogging * This,
            /* [in] */ FAX_LOG_LEVEL_ENUM InitEventLevel);
        
        DECLSPEC_XFGVIRT(IFaxEventLogging, get_InboundEventsLevel)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_InboundEventsLevel )( 
            __RPC__in IFaxEventLogging * This,
            /* [retval][out] */ __RPC__out FAX_LOG_LEVEL_ENUM *pInboundEventLevel);
        
        DECLSPEC_XFGVIRT(IFaxEventLogging, put_InboundEventsLevel)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_InboundEventsLevel )( 
            __RPC__in IFaxEventLogging * This,
            /* [in] */ FAX_LOG_LEVEL_ENUM InboundEventLevel);
        
        DECLSPEC_XFGVIRT(IFaxEventLogging, get_OutboundEventsLevel)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_OutboundEventsLevel )( 
            __RPC__in IFaxEventLogging * This,
            /* [retval][out] */ __RPC__out FAX_LOG_LEVEL_ENUM *pOutboundEventLevel);
        
        DECLSPEC_XFGVIRT(IFaxEventLogging, put_OutboundEventsLevel)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_OutboundEventsLevel )( 
            __RPC__in IFaxEventLogging * This,
            /* [in] */ FAX_LOG_LEVEL_ENUM OutboundEventLevel);
        
        DECLSPEC_XFGVIRT(IFaxEventLogging, get_GeneralEventsLevel)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_GeneralEventsLevel )( 
            __RPC__in IFaxEventLogging * This,
            /* [retval][out] */ __RPC__out FAX_LOG_LEVEL_ENUM *pGeneralEventLevel);
        
        DECLSPEC_XFGVIRT(IFaxEventLogging, put_GeneralEventsLevel)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_GeneralEventsLevel )( 
            __RPC__in IFaxEventLogging * This,
            /* [in] */ FAX_LOG_LEVEL_ENUM GeneralEventLevel);
        
        DECLSPEC_XFGVIRT(IFaxEventLogging, Refresh)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *Refresh )( 
            __RPC__in IFaxEventLogging * This);
        
        DECLSPEC_XFGVIRT(IFaxEventLogging, Save)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *Save )( 
            __RPC__in IFaxEventLogging * This);
        
        END_INTERFACE
    } IFaxEventLoggingVtbl;

    interface IFaxEventLogging
    {
        CONST_VTBL struct IFaxEventLoggingVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IFaxEventLogging_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IFaxEventLogging_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IFaxEventLogging_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IFaxEventLogging_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IFaxEventLogging_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IFaxEventLogging_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IFaxEventLogging_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IFaxEventLogging_get_InitEventsLevel(This,pInitEventLevel)	\
    ( (This)->lpVtbl -> get_InitEventsLevel(This,pInitEventLevel) ) 

#define IFaxEventLogging_put_InitEventsLevel(This,InitEventLevel)	\
    ( (This)->lpVtbl -> put_InitEventsLevel(This,InitEventLevel) ) 

#define IFaxEventLogging_get_InboundEventsLevel(This,pInboundEventLevel)	\
    ( (This)->lpVtbl -> get_InboundEventsLevel(This,pInboundEventLevel) ) 

#define IFaxEventLogging_put_InboundEventsLevel(This,InboundEventLevel)	\
    ( (This)->lpVtbl -> put_InboundEventsLevel(This,InboundEventLevel) ) 

#define IFaxEventLogging_get_OutboundEventsLevel(This,pOutboundEventLevel)	\
    ( (This)->lpVtbl -> get_OutboundEventsLevel(This,pOutboundEventLevel) ) 

#define IFaxEventLogging_put_OutboundEventsLevel(This,OutboundEventLevel)	\
    ( (This)->lpVtbl -> put_OutboundEventsLevel(This,OutboundEventLevel) ) 

#define IFaxEventLogging_get_GeneralEventsLevel(This,pGeneralEventLevel)	\
    ( (This)->lpVtbl -> get_GeneralEventsLevel(This,pGeneralEventLevel) ) 

#define IFaxEventLogging_put_GeneralEventsLevel(This,GeneralEventLevel)	\
    ( (This)->lpVtbl -> put_GeneralEventsLevel(This,GeneralEventLevel) ) 

#define IFaxEventLogging_Refresh(This)	\
    ( (This)->lpVtbl -> Refresh(This) ) 

#define IFaxEventLogging_Save(This)	\
    ( (This)->lpVtbl -> Save(This) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IFaxEventLogging_INTERFACE_DEFINED__ */


#ifndef __IFaxOutboundRoutingGroups_INTERFACE_DEFINED__
#define __IFaxOutboundRoutingGroups_INTERFACE_DEFINED__

/* interface IFaxOutboundRoutingGroups */
/* [unique][helpstring][dual][uuid][object] */ 


EXTERN_C const IID IID_IFaxOutboundRoutingGroups;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("235CBEF7-C2DE-4BFD-B8DA-75097C82C87F")
    IFaxOutboundRoutingGroups : public IDispatch
    {
    public:
        virtual /* [propget][id] */ HRESULT STDMETHODCALLTYPE get__NewEnum( 
            /* [retval][out] */ __RPC__deref_out_opt IUnknown **ppUnk) = 0;
        
        virtual /* [propget][id] */ HRESULT STDMETHODCALLTYPE get_Item( 
            /* [in] */ VARIANT vIndex,
            /* [retval][out] */ __RPC__deref_out_opt IFaxOutboundRoutingGroup **pFaxOutboundRoutingGroup) = 0;
        
        virtual /* [propget][id] */ HRESULT STDMETHODCALLTYPE get_Count( 
            /* [retval][out] */ __RPC__out long *plCount) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE Add( 
            /* [in] */ __RPC__in BSTR bstrName,
            /* [retval][out] */ __RPC__deref_out_opt IFaxOutboundRoutingGroup **pFaxOutboundRoutingGroup) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE Remove( 
            /* [in] */ VARIANT vIndex) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IFaxOutboundRoutingGroupsVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IFaxOutboundRoutingGroups * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IFaxOutboundRoutingGroups * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IFaxOutboundRoutingGroups * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IFaxOutboundRoutingGroups * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IFaxOutboundRoutingGroups * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IFaxOutboundRoutingGroups * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IFaxOutboundRoutingGroups * This,
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
        
        DECLSPEC_XFGVIRT(IFaxOutboundRoutingGroups, get__NewEnum)
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *get__NewEnum )( 
            __RPC__in IFaxOutboundRoutingGroups * This,
            /* [retval][out] */ __RPC__deref_out_opt IUnknown **ppUnk);
        
        DECLSPEC_XFGVIRT(IFaxOutboundRoutingGroups, get_Item)
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_Item )( 
            __RPC__in IFaxOutboundRoutingGroups * This,
            /* [in] */ VARIANT vIndex,
            /* [retval][out] */ __RPC__deref_out_opt IFaxOutboundRoutingGroup **pFaxOutboundRoutingGroup);
        
        DECLSPEC_XFGVIRT(IFaxOutboundRoutingGroups, get_Count)
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_Count )( 
            __RPC__in IFaxOutboundRoutingGroups * This,
            /* [retval][out] */ __RPC__out long *plCount);
        
        DECLSPEC_XFGVIRT(IFaxOutboundRoutingGroups, Add)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *Add )( 
            __RPC__in IFaxOutboundRoutingGroups * This,
            /* [in] */ __RPC__in BSTR bstrName,
            /* [retval][out] */ __RPC__deref_out_opt IFaxOutboundRoutingGroup **pFaxOutboundRoutingGroup);
        
        DECLSPEC_XFGVIRT(IFaxOutboundRoutingGroups, Remove)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *Remove )( 
            __RPC__in IFaxOutboundRoutingGroups * This,
            /* [in] */ VARIANT vIndex);
        
        END_INTERFACE
    } IFaxOutboundRoutingGroupsVtbl;

    interface IFaxOutboundRoutingGroups
    {
        CONST_VTBL struct IFaxOutboundRoutingGroupsVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IFaxOutboundRoutingGroups_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IFaxOutboundRoutingGroups_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IFaxOutboundRoutingGroups_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IFaxOutboundRoutingGroups_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IFaxOutboundRoutingGroups_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IFaxOutboundRoutingGroups_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IFaxOutboundRoutingGroups_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IFaxOutboundRoutingGroups_get__NewEnum(This,ppUnk)	\
    ( (This)->lpVtbl -> get__NewEnum(This,ppUnk) ) 

#define IFaxOutboundRoutingGroups_get_Item(This,vIndex,pFaxOutboundRoutingGroup)	\
    ( (This)->lpVtbl -> get_Item(This,vIndex,pFaxOutboundRoutingGroup) ) 

#define IFaxOutboundRoutingGroups_get_Count(This,plCount)	\
    ( (This)->lpVtbl -> get_Count(This,plCount) ) 

#define IFaxOutboundRoutingGroups_Add(This,bstrName,pFaxOutboundRoutingGroup)	\
    ( (This)->lpVtbl -> Add(This,bstrName,pFaxOutboundRoutingGroup) ) 

#define IFaxOutboundRoutingGroups_Remove(This,vIndex)	\
    ( (This)->lpVtbl -> Remove(This,vIndex) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IFaxOutboundRoutingGroups_INTERFACE_DEFINED__ */


#ifndef __IFaxOutboundRoutingGroup_INTERFACE_DEFINED__
#define __IFaxOutboundRoutingGroup_INTERFACE_DEFINED__

/* interface IFaxOutboundRoutingGroup */
/* [unique][helpstring][dual][uuid][object] */ 

typedef 
enum FAX_GROUP_STATUS_ENUM
    {
        fgsALL_DEV_VALID	= 0,
        fgsEMPTY	= ( fgsALL_DEV_VALID + 1 ) ,
        fgsALL_DEV_NOT_VALID	= ( fgsEMPTY + 1 ) ,
        fgsSOME_DEV_NOT_VALID	= ( fgsALL_DEV_NOT_VALID + 1 ) 
    } 	FAX_GROUP_STATUS_ENUM;


EXTERN_C const IID IID_IFaxOutboundRoutingGroup;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("CA6289A1-7E25-4F87-9A0B-93365734962C")
    IFaxOutboundRoutingGroup : public IDispatch
    {
    public:
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Name( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrName) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Status( 
            /* [retval][out] */ __RPC__out FAX_GROUP_STATUS_ENUM *pStatus) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_DeviceIds( 
            /* [retval][out] */ __RPC__deref_out_opt IFaxDeviceIds **pFaxDeviceIds) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IFaxOutboundRoutingGroupVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IFaxOutboundRoutingGroup * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IFaxOutboundRoutingGroup * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IFaxOutboundRoutingGroup * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IFaxOutboundRoutingGroup * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IFaxOutboundRoutingGroup * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IFaxOutboundRoutingGroup * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IFaxOutboundRoutingGroup * This,
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
        
        DECLSPEC_XFGVIRT(IFaxOutboundRoutingGroup, get_Name)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Name )( 
            __RPC__in IFaxOutboundRoutingGroup * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrName);
        
        DECLSPEC_XFGVIRT(IFaxOutboundRoutingGroup, get_Status)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Status )( 
            __RPC__in IFaxOutboundRoutingGroup * This,
            /* [retval][out] */ __RPC__out FAX_GROUP_STATUS_ENUM *pStatus);
        
        DECLSPEC_XFGVIRT(IFaxOutboundRoutingGroup, get_DeviceIds)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_DeviceIds )( 
            __RPC__in IFaxOutboundRoutingGroup * This,
            /* [retval][out] */ __RPC__deref_out_opt IFaxDeviceIds **pFaxDeviceIds);
        
        END_INTERFACE
    } IFaxOutboundRoutingGroupVtbl;

    interface IFaxOutboundRoutingGroup
    {
        CONST_VTBL struct IFaxOutboundRoutingGroupVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IFaxOutboundRoutingGroup_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IFaxOutboundRoutingGroup_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IFaxOutboundRoutingGroup_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IFaxOutboundRoutingGroup_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IFaxOutboundRoutingGroup_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IFaxOutboundRoutingGroup_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IFaxOutboundRoutingGroup_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IFaxOutboundRoutingGroup_get_Name(This,pbstrName)	\
    ( (This)->lpVtbl -> get_Name(This,pbstrName) ) 

#define IFaxOutboundRoutingGroup_get_Status(This,pStatus)	\
    ( (This)->lpVtbl -> get_Status(This,pStatus) ) 

#define IFaxOutboundRoutingGroup_get_DeviceIds(This,pFaxDeviceIds)	\
    ( (This)->lpVtbl -> get_DeviceIds(This,pFaxDeviceIds) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IFaxOutboundRoutingGroup_INTERFACE_DEFINED__ */


#ifndef __IFaxDeviceIds_INTERFACE_DEFINED__
#define __IFaxDeviceIds_INTERFACE_DEFINED__

/* interface IFaxDeviceIds */
/* [unique][helpstring][dual][uuid][object] */ 


EXTERN_C const IID IID_IFaxDeviceIds;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("2F0F813F-4CE9-443E-8CA1-738CFAEEE149")
    IFaxDeviceIds : public IDispatch
    {
    public:
        virtual /* [propget][id] */ HRESULT STDMETHODCALLTYPE get__NewEnum( 
            /* [retval][out] */ __RPC__deref_out_opt IUnknown **ppUnk) = 0;
        
        virtual /* [propget][id] */ HRESULT STDMETHODCALLTYPE get_Item( 
            /* [in] */ long lIndex,
            /* [retval][out] */ __RPC__out long *plDeviceId) = 0;
        
        virtual /* [propget][id] */ HRESULT STDMETHODCALLTYPE get_Count( 
            /* [retval][out] */ __RPC__out long *plCount) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE Add( 
            /* [in] */ long lDeviceId) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE Remove( 
            /* [in] */ long lIndex) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE SetOrder( 
            /* [in] */ long lDeviceId,
            /* [in] */ long lNewOrder) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IFaxDeviceIdsVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IFaxDeviceIds * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IFaxDeviceIds * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IFaxDeviceIds * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IFaxDeviceIds * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IFaxDeviceIds * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IFaxDeviceIds * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IFaxDeviceIds * This,
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
        
        DECLSPEC_XFGVIRT(IFaxDeviceIds, get__NewEnum)
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *get__NewEnum )( 
            __RPC__in IFaxDeviceIds * This,
            /* [retval][out] */ __RPC__deref_out_opt IUnknown **ppUnk);
        
        DECLSPEC_XFGVIRT(IFaxDeviceIds, get_Item)
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_Item )( 
            __RPC__in IFaxDeviceIds * This,
            /* [in] */ long lIndex,
            /* [retval][out] */ __RPC__out long *plDeviceId);
        
        DECLSPEC_XFGVIRT(IFaxDeviceIds, get_Count)
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_Count )( 
            __RPC__in IFaxDeviceIds * This,
            /* [retval][out] */ __RPC__out long *plCount);
        
        DECLSPEC_XFGVIRT(IFaxDeviceIds, Add)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *Add )( 
            __RPC__in IFaxDeviceIds * This,
            /* [in] */ long lDeviceId);
        
        DECLSPEC_XFGVIRT(IFaxDeviceIds, Remove)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *Remove )( 
            __RPC__in IFaxDeviceIds * This,
            /* [in] */ long lIndex);
        
        DECLSPEC_XFGVIRT(IFaxDeviceIds, SetOrder)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *SetOrder )( 
            __RPC__in IFaxDeviceIds * This,
            /* [in] */ long lDeviceId,
            /* [in] */ long lNewOrder);
        
        END_INTERFACE
    } IFaxDeviceIdsVtbl;

    interface IFaxDeviceIds
    {
        CONST_VTBL struct IFaxDeviceIdsVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IFaxDeviceIds_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IFaxDeviceIds_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IFaxDeviceIds_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IFaxDeviceIds_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IFaxDeviceIds_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IFaxDeviceIds_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IFaxDeviceIds_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IFaxDeviceIds_get__NewEnum(This,ppUnk)	\
    ( (This)->lpVtbl -> get__NewEnum(This,ppUnk) ) 

#define IFaxDeviceIds_get_Item(This,lIndex,plDeviceId)	\
    ( (This)->lpVtbl -> get_Item(This,lIndex,plDeviceId) ) 

#define IFaxDeviceIds_get_Count(This,plCount)	\
    ( (This)->lpVtbl -> get_Count(This,plCount) ) 

#define IFaxDeviceIds_Add(This,lDeviceId)	\
    ( (This)->lpVtbl -> Add(This,lDeviceId) ) 

#define IFaxDeviceIds_Remove(This,lIndex)	\
    ( (This)->lpVtbl -> Remove(This,lIndex) ) 

#define IFaxDeviceIds_SetOrder(This,lDeviceId,lNewOrder)	\
    ( (This)->lpVtbl -> SetOrder(This,lDeviceId,lNewOrder) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IFaxDeviceIds_INTERFACE_DEFINED__ */


#ifndef __IFaxOutboundRoutingRules_INTERFACE_DEFINED__
#define __IFaxOutboundRoutingRules_INTERFACE_DEFINED__

/* interface IFaxOutboundRoutingRules */
/* [unique][helpstring][dual][uuid][object] */ 


EXTERN_C const IID IID_IFaxOutboundRoutingRules;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("DCEFA1E7-AE7D-4ED6-8521-369EDCCA5120")
    IFaxOutboundRoutingRules : public IDispatch
    {
    public:
        virtual /* [propget][id] */ HRESULT STDMETHODCALLTYPE get__NewEnum( 
            /* [retval][out] */ __RPC__deref_out_opt IUnknown **ppUnk) = 0;
        
        virtual /* [propget][id] */ HRESULT STDMETHODCALLTYPE get_Item( 
            /* [in] */ long lIndex,
            /* [retval][out] */ __RPC__deref_out_opt IFaxOutboundRoutingRule **pFaxOutboundRoutingRule) = 0;
        
        virtual /* [propget][id] */ HRESULT STDMETHODCALLTYPE get_Count( 
            /* [retval][out] */ __RPC__out long *plCount) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE ItemByCountryAndArea( 
            /* [in] */ long lCountryCode,
            /* [in] */ long lAreaCode,
            /* [retval][out] */ __RPC__deref_out_opt IFaxOutboundRoutingRule **pFaxOutboundRoutingRule) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE RemoveByCountryAndArea( 
            /* [in] */ long lCountryCode,
            /* [in] */ long lAreaCode) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE Remove( 
            /* [in] */ long lIndex) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE Add( 
            /* [in] */ long lCountryCode,
            /* [in] */ long lAreaCode,
            /* [in] */ VARIANT_BOOL bUseDevice,
            /* [in] */ __RPC__in BSTR bstrGroupName,
            /* [in] */ long lDeviceId,
            /* [retval][out] */ __RPC__deref_out_opt IFaxOutboundRoutingRule **pFaxOutboundRoutingRule) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IFaxOutboundRoutingRulesVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IFaxOutboundRoutingRules * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IFaxOutboundRoutingRules * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IFaxOutboundRoutingRules * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IFaxOutboundRoutingRules * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IFaxOutboundRoutingRules * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IFaxOutboundRoutingRules * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IFaxOutboundRoutingRules * This,
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
        
        DECLSPEC_XFGVIRT(IFaxOutboundRoutingRules, get__NewEnum)
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *get__NewEnum )( 
            __RPC__in IFaxOutboundRoutingRules * This,
            /* [retval][out] */ __RPC__deref_out_opt IUnknown **ppUnk);
        
        DECLSPEC_XFGVIRT(IFaxOutboundRoutingRules, get_Item)
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_Item )( 
            __RPC__in IFaxOutboundRoutingRules * This,
            /* [in] */ long lIndex,
            /* [retval][out] */ __RPC__deref_out_opt IFaxOutboundRoutingRule **pFaxOutboundRoutingRule);
        
        DECLSPEC_XFGVIRT(IFaxOutboundRoutingRules, get_Count)
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_Count )( 
            __RPC__in IFaxOutboundRoutingRules * This,
            /* [retval][out] */ __RPC__out long *plCount);
        
        DECLSPEC_XFGVIRT(IFaxOutboundRoutingRules, ItemByCountryAndArea)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *ItemByCountryAndArea )( 
            __RPC__in IFaxOutboundRoutingRules * This,
            /* [in] */ long lCountryCode,
            /* [in] */ long lAreaCode,
            /* [retval][out] */ __RPC__deref_out_opt IFaxOutboundRoutingRule **pFaxOutboundRoutingRule);
        
        DECLSPEC_XFGVIRT(IFaxOutboundRoutingRules, RemoveByCountryAndArea)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *RemoveByCountryAndArea )( 
            __RPC__in IFaxOutboundRoutingRules * This,
            /* [in] */ long lCountryCode,
            /* [in] */ long lAreaCode);
        
        DECLSPEC_XFGVIRT(IFaxOutboundRoutingRules, Remove)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *Remove )( 
            __RPC__in IFaxOutboundRoutingRules * This,
            /* [in] */ long lIndex);
        
        DECLSPEC_XFGVIRT(IFaxOutboundRoutingRules, Add)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *Add )( 
            __RPC__in IFaxOutboundRoutingRules * This,
            /* [in] */ long lCountryCode,
            /* [in] */ long lAreaCode,
            /* [in] */ VARIANT_BOOL bUseDevice,
            /* [in] */ __RPC__in BSTR bstrGroupName,
            /* [in] */ long lDeviceId,
            /* [retval][out] */ __RPC__deref_out_opt IFaxOutboundRoutingRule **pFaxOutboundRoutingRule);
        
        END_INTERFACE
    } IFaxOutboundRoutingRulesVtbl;

    interface IFaxOutboundRoutingRules
    {
        CONST_VTBL struct IFaxOutboundRoutingRulesVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IFaxOutboundRoutingRules_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IFaxOutboundRoutingRules_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IFaxOutboundRoutingRules_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IFaxOutboundRoutingRules_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IFaxOutboundRoutingRules_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IFaxOutboundRoutingRules_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IFaxOutboundRoutingRules_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IFaxOutboundRoutingRules_get__NewEnum(This,ppUnk)	\
    ( (This)->lpVtbl -> get__NewEnum(This,ppUnk) ) 

#define IFaxOutboundRoutingRules_get_Item(This,lIndex,pFaxOutboundRoutingRule)	\
    ( (This)->lpVtbl -> get_Item(This,lIndex,pFaxOutboundRoutingRule) ) 

#define IFaxOutboundRoutingRules_get_Count(This,plCount)	\
    ( (This)->lpVtbl -> get_Count(This,plCount) ) 

#define IFaxOutboundRoutingRules_ItemByCountryAndArea(This,lCountryCode,lAreaCode,pFaxOutboundRoutingRule)	\
    ( (This)->lpVtbl -> ItemByCountryAndArea(This,lCountryCode,lAreaCode,pFaxOutboundRoutingRule) ) 

#define IFaxOutboundRoutingRules_RemoveByCountryAndArea(This,lCountryCode,lAreaCode)	\
    ( (This)->lpVtbl -> RemoveByCountryAndArea(This,lCountryCode,lAreaCode) ) 

#define IFaxOutboundRoutingRules_Remove(This,lIndex)	\
    ( (This)->lpVtbl -> Remove(This,lIndex) ) 

#define IFaxOutboundRoutingRules_Add(This,lCountryCode,lAreaCode,bUseDevice,bstrGroupName,lDeviceId,pFaxOutboundRoutingRule)	\
    ( (This)->lpVtbl -> Add(This,lCountryCode,lAreaCode,bUseDevice,bstrGroupName,lDeviceId,pFaxOutboundRoutingRule) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IFaxOutboundRoutingRules_INTERFACE_DEFINED__ */


#ifndef __IFaxOutboundRoutingRule_INTERFACE_DEFINED__
#define __IFaxOutboundRoutingRule_INTERFACE_DEFINED__

/* interface IFaxOutboundRoutingRule */
/* [unique][helpstring][dual][uuid][object] */ 

typedef 
enum FAX_RULE_STATUS_ENUM
    {
        frsVALID	= 0,
        frsEMPTY_GROUP	= ( frsVALID + 1 ) ,
        frsALL_GROUP_DEV_NOT_VALID	= ( frsEMPTY_GROUP + 1 ) ,
        frsSOME_GROUP_DEV_NOT_VALID	= ( frsALL_GROUP_DEV_NOT_VALID + 1 ) ,
        frsBAD_DEVICE	= ( frsSOME_GROUP_DEV_NOT_VALID + 1 ) 
    } 	FAX_RULE_STATUS_ENUM;


EXTERN_C const IID IID_IFaxOutboundRoutingRule;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("E1F795D5-07C2-469F-B027-ACACC23219DA")
    IFaxOutboundRoutingRule : public IDispatch
    {
    public:
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_CountryCode( 
            /* [retval][out] */ __RPC__out long *plCountryCode) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_AreaCode( 
            /* [retval][out] */ __RPC__out long *plAreaCode) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Status( 
            /* [retval][out] */ __RPC__out FAX_RULE_STATUS_ENUM *pStatus) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_UseDevice( 
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pbUseDevice) = 0;
        
        virtual /* [helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_UseDevice( 
            /* [in] */ VARIANT_BOOL bUseDevice) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_DeviceId( 
            /* [retval][out] */ __RPC__out long *plDeviceId) = 0;
        
        virtual /* [helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_DeviceId( 
            /* [in] */ long DeviceId) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_GroupName( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrGroupName) = 0;
        
        virtual /* [helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_GroupName( 
            /* [in] */ __RPC__in BSTR bstrGroupName) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE Refresh( void) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE Save( void) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IFaxOutboundRoutingRuleVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IFaxOutboundRoutingRule * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IFaxOutboundRoutingRule * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IFaxOutboundRoutingRule * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IFaxOutboundRoutingRule * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IFaxOutboundRoutingRule * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IFaxOutboundRoutingRule * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IFaxOutboundRoutingRule * This,
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
        
        DECLSPEC_XFGVIRT(IFaxOutboundRoutingRule, get_CountryCode)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_CountryCode )( 
            __RPC__in IFaxOutboundRoutingRule * This,
            /* [retval][out] */ __RPC__out long *plCountryCode);
        
        DECLSPEC_XFGVIRT(IFaxOutboundRoutingRule, get_AreaCode)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_AreaCode )( 
            __RPC__in IFaxOutboundRoutingRule * This,
            /* [retval][out] */ __RPC__out long *plAreaCode);
        
        DECLSPEC_XFGVIRT(IFaxOutboundRoutingRule, get_Status)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Status )( 
            __RPC__in IFaxOutboundRoutingRule * This,
            /* [retval][out] */ __RPC__out FAX_RULE_STATUS_ENUM *pStatus);
        
        DECLSPEC_XFGVIRT(IFaxOutboundRoutingRule, get_UseDevice)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_UseDevice )( 
            __RPC__in IFaxOutboundRoutingRule * This,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pbUseDevice);
        
        DECLSPEC_XFGVIRT(IFaxOutboundRoutingRule, put_UseDevice)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_UseDevice )( 
            __RPC__in IFaxOutboundRoutingRule * This,
            /* [in] */ VARIANT_BOOL bUseDevice);
        
        DECLSPEC_XFGVIRT(IFaxOutboundRoutingRule, get_DeviceId)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_DeviceId )( 
            __RPC__in IFaxOutboundRoutingRule * This,
            /* [retval][out] */ __RPC__out long *plDeviceId);
        
        DECLSPEC_XFGVIRT(IFaxOutboundRoutingRule, put_DeviceId)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_DeviceId )( 
            __RPC__in IFaxOutboundRoutingRule * This,
            /* [in] */ long DeviceId);
        
        DECLSPEC_XFGVIRT(IFaxOutboundRoutingRule, get_GroupName)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_GroupName )( 
            __RPC__in IFaxOutboundRoutingRule * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrGroupName);
        
        DECLSPEC_XFGVIRT(IFaxOutboundRoutingRule, put_GroupName)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_GroupName )( 
            __RPC__in IFaxOutboundRoutingRule * This,
            /* [in] */ __RPC__in BSTR bstrGroupName);
        
        DECLSPEC_XFGVIRT(IFaxOutboundRoutingRule, Refresh)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *Refresh )( 
            __RPC__in IFaxOutboundRoutingRule * This);
        
        DECLSPEC_XFGVIRT(IFaxOutboundRoutingRule, Save)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *Save )( 
            __RPC__in IFaxOutboundRoutingRule * This);
        
        END_INTERFACE
    } IFaxOutboundRoutingRuleVtbl;

    interface IFaxOutboundRoutingRule
    {
        CONST_VTBL struct IFaxOutboundRoutingRuleVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IFaxOutboundRoutingRule_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IFaxOutboundRoutingRule_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IFaxOutboundRoutingRule_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IFaxOutboundRoutingRule_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IFaxOutboundRoutingRule_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IFaxOutboundRoutingRule_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IFaxOutboundRoutingRule_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IFaxOutboundRoutingRule_get_CountryCode(This,plCountryCode)	\
    ( (This)->lpVtbl -> get_CountryCode(This,plCountryCode) ) 

#define IFaxOutboundRoutingRule_get_AreaCode(This,plAreaCode)	\
    ( (This)->lpVtbl -> get_AreaCode(This,plAreaCode) ) 

#define IFaxOutboundRoutingRule_get_Status(This,pStatus)	\
    ( (This)->lpVtbl -> get_Status(This,pStatus) ) 

#define IFaxOutboundRoutingRule_get_UseDevice(This,pbUseDevice)	\
    ( (This)->lpVtbl -> get_UseDevice(This,pbUseDevice) ) 

#define IFaxOutboundRoutingRule_put_UseDevice(This,bUseDevice)	\
    ( (This)->lpVtbl -> put_UseDevice(This,bUseDevice) ) 

#define IFaxOutboundRoutingRule_get_DeviceId(This,plDeviceId)	\
    ( (This)->lpVtbl -> get_DeviceId(This,plDeviceId) ) 

#define IFaxOutboundRoutingRule_put_DeviceId(This,DeviceId)	\
    ( (This)->lpVtbl -> put_DeviceId(This,DeviceId) ) 

#define IFaxOutboundRoutingRule_get_GroupName(This,pbstrGroupName)	\
    ( (This)->lpVtbl -> get_GroupName(This,pbstrGroupName) ) 

#define IFaxOutboundRoutingRule_put_GroupName(This,bstrGroupName)	\
    ( (This)->lpVtbl -> put_GroupName(This,bstrGroupName) ) 

#define IFaxOutboundRoutingRule_Refresh(This)	\
    ( (This)->lpVtbl -> Refresh(This) ) 

#define IFaxOutboundRoutingRule_Save(This)	\
    ( (This)->lpVtbl -> Save(This) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IFaxOutboundRoutingRule_INTERFACE_DEFINED__ */


#ifndef __IFaxInboundRoutingExtensions_INTERFACE_DEFINED__
#define __IFaxInboundRoutingExtensions_INTERFACE_DEFINED__

/* interface IFaxInboundRoutingExtensions */
/* [unique][helpstring][dual][uuid][object] */ 


EXTERN_C const IID IID_IFaxInboundRoutingExtensions;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("2F6C9673-7B26-42DE-8EB0-915DCD2A4F4C")
    IFaxInboundRoutingExtensions : public IDispatch
    {
    public:
        virtual /* [propget][id] */ HRESULT STDMETHODCALLTYPE get__NewEnum( 
            /* [retval][out] */ __RPC__deref_out_opt IUnknown **ppUnk) = 0;
        
        virtual /* [propget][id] */ HRESULT STDMETHODCALLTYPE get_Item( 
            /* [in] */ VARIANT vIndex,
            /* [retval][out] */ __RPC__deref_out_opt IFaxInboundRoutingExtension **pFaxInboundRoutingExtension) = 0;
        
        virtual /* [propget][id] */ HRESULT STDMETHODCALLTYPE get_Count( 
            /* [retval][out] */ __RPC__out long *plCount) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IFaxInboundRoutingExtensionsVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IFaxInboundRoutingExtensions * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IFaxInboundRoutingExtensions * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IFaxInboundRoutingExtensions * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IFaxInboundRoutingExtensions * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IFaxInboundRoutingExtensions * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IFaxInboundRoutingExtensions * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IFaxInboundRoutingExtensions * This,
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
        
        DECLSPEC_XFGVIRT(IFaxInboundRoutingExtensions, get__NewEnum)
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *get__NewEnum )( 
            __RPC__in IFaxInboundRoutingExtensions * This,
            /* [retval][out] */ __RPC__deref_out_opt IUnknown **ppUnk);
        
        DECLSPEC_XFGVIRT(IFaxInboundRoutingExtensions, get_Item)
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_Item )( 
            __RPC__in IFaxInboundRoutingExtensions * This,
            /* [in] */ VARIANT vIndex,
            /* [retval][out] */ __RPC__deref_out_opt IFaxInboundRoutingExtension **pFaxInboundRoutingExtension);
        
        DECLSPEC_XFGVIRT(IFaxInboundRoutingExtensions, get_Count)
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_Count )( 
            __RPC__in IFaxInboundRoutingExtensions * This,
            /* [retval][out] */ __RPC__out long *plCount);
        
        END_INTERFACE
    } IFaxInboundRoutingExtensionsVtbl;

    interface IFaxInboundRoutingExtensions
    {
        CONST_VTBL struct IFaxInboundRoutingExtensionsVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IFaxInboundRoutingExtensions_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IFaxInboundRoutingExtensions_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IFaxInboundRoutingExtensions_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IFaxInboundRoutingExtensions_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IFaxInboundRoutingExtensions_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IFaxInboundRoutingExtensions_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IFaxInboundRoutingExtensions_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IFaxInboundRoutingExtensions_get__NewEnum(This,ppUnk)	\
    ( (This)->lpVtbl -> get__NewEnum(This,ppUnk) ) 

#define IFaxInboundRoutingExtensions_get_Item(This,vIndex,pFaxInboundRoutingExtension)	\
    ( (This)->lpVtbl -> get_Item(This,vIndex,pFaxInboundRoutingExtension) ) 

#define IFaxInboundRoutingExtensions_get_Count(This,plCount)	\
    ( (This)->lpVtbl -> get_Count(This,plCount) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IFaxInboundRoutingExtensions_INTERFACE_DEFINED__ */


#ifndef __IFaxInboundRoutingExtension_INTERFACE_DEFINED__
#define __IFaxInboundRoutingExtension_INTERFACE_DEFINED__

/* interface IFaxInboundRoutingExtension */
/* [unique][helpstring][dual][uuid][object] */ 


EXTERN_C const IID IID_IFaxInboundRoutingExtension;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("885B5E08-C26C-4EF9-AF83-51580A750BE1")
    IFaxInboundRoutingExtension : public IDispatch
    {
    public:
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_FriendlyName( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrFriendlyName) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_ImageName( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrImageName) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_UniqueName( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrUniqueName) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_MajorVersion( 
            /* [retval][out] */ __RPC__out long *plMajorVersion) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_MinorVersion( 
            /* [retval][out] */ __RPC__out long *plMinorVersion) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_MajorBuild( 
            /* [retval][out] */ __RPC__out long *plMajorBuild) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_MinorBuild( 
            /* [retval][out] */ __RPC__out long *plMinorBuild) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Debug( 
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pbDebug) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Status( 
            /* [retval][out] */ __RPC__out FAX_PROVIDER_STATUS_ENUM *pStatus) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_InitErrorCode( 
            /* [retval][out] */ __RPC__out long *plInitErrorCode) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Methods( 
            /* [retval][out] */ __RPC__out VARIANT *pvMethods) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IFaxInboundRoutingExtensionVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IFaxInboundRoutingExtension * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IFaxInboundRoutingExtension * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IFaxInboundRoutingExtension * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IFaxInboundRoutingExtension * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IFaxInboundRoutingExtension * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IFaxInboundRoutingExtension * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IFaxInboundRoutingExtension * This,
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
        
        DECLSPEC_XFGVIRT(IFaxInboundRoutingExtension, get_FriendlyName)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_FriendlyName )( 
            __RPC__in IFaxInboundRoutingExtension * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrFriendlyName);
        
        DECLSPEC_XFGVIRT(IFaxInboundRoutingExtension, get_ImageName)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_ImageName )( 
            __RPC__in IFaxInboundRoutingExtension * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrImageName);
        
        DECLSPEC_XFGVIRT(IFaxInboundRoutingExtension, get_UniqueName)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_UniqueName )( 
            __RPC__in IFaxInboundRoutingExtension * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrUniqueName);
        
        DECLSPEC_XFGVIRT(IFaxInboundRoutingExtension, get_MajorVersion)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_MajorVersion )( 
            __RPC__in IFaxInboundRoutingExtension * This,
            /* [retval][out] */ __RPC__out long *plMajorVersion);
        
        DECLSPEC_XFGVIRT(IFaxInboundRoutingExtension, get_MinorVersion)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_MinorVersion )( 
            __RPC__in IFaxInboundRoutingExtension * This,
            /* [retval][out] */ __RPC__out long *plMinorVersion);
        
        DECLSPEC_XFGVIRT(IFaxInboundRoutingExtension, get_MajorBuild)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_MajorBuild )( 
            __RPC__in IFaxInboundRoutingExtension * This,
            /* [retval][out] */ __RPC__out long *plMajorBuild);
        
        DECLSPEC_XFGVIRT(IFaxInboundRoutingExtension, get_MinorBuild)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_MinorBuild )( 
            __RPC__in IFaxInboundRoutingExtension * This,
            /* [retval][out] */ __RPC__out long *plMinorBuild);
        
        DECLSPEC_XFGVIRT(IFaxInboundRoutingExtension, get_Debug)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Debug )( 
            __RPC__in IFaxInboundRoutingExtension * This,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pbDebug);
        
        DECLSPEC_XFGVIRT(IFaxInboundRoutingExtension, get_Status)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Status )( 
            __RPC__in IFaxInboundRoutingExtension * This,
            /* [retval][out] */ __RPC__out FAX_PROVIDER_STATUS_ENUM *pStatus);
        
        DECLSPEC_XFGVIRT(IFaxInboundRoutingExtension, get_InitErrorCode)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_InitErrorCode )( 
            __RPC__in IFaxInboundRoutingExtension * This,
            /* [retval][out] */ __RPC__out long *plInitErrorCode);
        
        DECLSPEC_XFGVIRT(IFaxInboundRoutingExtension, get_Methods)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Methods )( 
            __RPC__in IFaxInboundRoutingExtension * This,
            /* [retval][out] */ __RPC__out VARIANT *pvMethods);
        
        END_INTERFACE
    } IFaxInboundRoutingExtensionVtbl;

    interface IFaxInboundRoutingExtension
    {
        CONST_VTBL struct IFaxInboundRoutingExtensionVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IFaxInboundRoutingExtension_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IFaxInboundRoutingExtension_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IFaxInboundRoutingExtension_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IFaxInboundRoutingExtension_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IFaxInboundRoutingExtension_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IFaxInboundRoutingExtension_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IFaxInboundRoutingExtension_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IFaxInboundRoutingExtension_get_FriendlyName(This,pbstrFriendlyName)	\
    ( (This)->lpVtbl -> get_FriendlyName(This,pbstrFriendlyName) ) 

#define IFaxInboundRoutingExtension_get_ImageName(This,pbstrImageName)	\
    ( (This)->lpVtbl -> get_ImageName(This,pbstrImageName) ) 

#define IFaxInboundRoutingExtension_get_UniqueName(This,pbstrUniqueName)	\
    ( (This)->lpVtbl -> get_UniqueName(This,pbstrUniqueName) ) 

#define IFaxInboundRoutingExtension_get_MajorVersion(This,plMajorVersion)	\
    ( (This)->lpVtbl -> get_MajorVersion(This,plMajorVersion) ) 

#define IFaxInboundRoutingExtension_get_MinorVersion(This,plMinorVersion)	\
    ( (This)->lpVtbl -> get_MinorVersion(This,plMinorVersion) ) 

#define IFaxInboundRoutingExtension_get_MajorBuild(This,plMajorBuild)	\
    ( (This)->lpVtbl -> get_MajorBuild(This,plMajorBuild) ) 

#define IFaxInboundRoutingExtension_get_MinorBuild(This,plMinorBuild)	\
    ( (This)->lpVtbl -> get_MinorBuild(This,plMinorBuild) ) 

#define IFaxInboundRoutingExtension_get_Debug(This,pbDebug)	\
    ( (This)->lpVtbl -> get_Debug(This,pbDebug) ) 

#define IFaxInboundRoutingExtension_get_Status(This,pStatus)	\
    ( (This)->lpVtbl -> get_Status(This,pStatus) ) 

#define IFaxInboundRoutingExtension_get_InitErrorCode(This,plInitErrorCode)	\
    ( (This)->lpVtbl -> get_InitErrorCode(This,plInitErrorCode) ) 

#define IFaxInboundRoutingExtension_get_Methods(This,pvMethods)	\
    ( (This)->lpVtbl -> get_Methods(This,pvMethods) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IFaxInboundRoutingExtension_INTERFACE_DEFINED__ */


#ifndef __IFaxInboundRoutingMethods_INTERFACE_DEFINED__
#define __IFaxInboundRoutingMethods_INTERFACE_DEFINED__

/* interface IFaxInboundRoutingMethods */
/* [unique][helpstring][dual][uuid][object] */ 


EXTERN_C const IID IID_IFaxInboundRoutingMethods;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("783FCA10-8908-4473-9D69-F67FBEA0C6B9")
    IFaxInboundRoutingMethods : public IDispatch
    {
    public:
        virtual /* [propget][id] */ HRESULT STDMETHODCALLTYPE get__NewEnum( 
            /* [retval][out] */ __RPC__deref_out_opt IUnknown **ppUnk) = 0;
        
        virtual /* [propget][id] */ HRESULT STDMETHODCALLTYPE get_Item( 
            /* [in] */ VARIANT vIndex,
            /* [retval][out] */ __RPC__deref_out_opt IFaxInboundRoutingMethod **pFaxInboundRoutingMethod) = 0;
        
        virtual /* [propget][id] */ HRESULT STDMETHODCALLTYPE get_Count( 
            /* [retval][out] */ __RPC__out long *plCount) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IFaxInboundRoutingMethodsVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IFaxInboundRoutingMethods * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IFaxInboundRoutingMethods * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IFaxInboundRoutingMethods * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IFaxInboundRoutingMethods * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IFaxInboundRoutingMethods * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IFaxInboundRoutingMethods * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IFaxInboundRoutingMethods * This,
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
        
        DECLSPEC_XFGVIRT(IFaxInboundRoutingMethods, get__NewEnum)
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *get__NewEnum )( 
            __RPC__in IFaxInboundRoutingMethods * This,
            /* [retval][out] */ __RPC__deref_out_opt IUnknown **ppUnk);
        
        DECLSPEC_XFGVIRT(IFaxInboundRoutingMethods, get_Item)
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_Item )( 
            __RPC__in IFaxInboundRoutingMethods * This,
            /* [in] */ VARIANT vIndex,
            /* [retval][out] */ __RPC__deref_out_opt IFaxInboundRoutingMethod **pFaxInboundRoutingMethod);
        
        DECLSPEC_XFGVIRT(IFaxInboundRoutingMethods, get_Count)
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_Count )( 
            __RPC__in IFaxInboundRoutingMethods * This,
            /* [retval][out] */ __RPC__out long *plCount);
        
        END_INTERFACE
    } IFaxInboundRoutingMethodsVtbl;

    interface IFaxInboundRoutingMethods
    {
        CONST_VTBL struct IFaxInboundRoutingMethodsVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IFaxInboundRoutingMethods_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IFaxInboundRoutingMethods_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IFaxInboundRoutingMethods_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IFaxInboundRoutingMethods_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IFaxInboundRoutingMethods_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IFaxInboundRoutingMethods_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IFaxInboundRoutingMethods_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IFaxInboundRoutingMethods_get__NewEnum(This,ppUnk)	\
    ( (This)->lpVtbl -> get__NewEnum(This,ppUnk) ) 

#define IFaxInboundRoutingMethods_get_Item(This,vIndex,pFaxInboundRoutingMethod)	\
    ( (This)->lpVtbl -> get_Item(This,vIndex,pFaxInboundRoutingMethod) ) 

#define IFaxInboundRoutingMethods_get_Count(This,plCount)	\
    ( (This)->lpVtbl -> get_Count(This,plCount) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IFaxInboundRoutingMethods_INTERFACE_DEFINED__ */


#ifndef __IFaxInboundRoutingMethod_INTERFACE_DEFINED__
#define __IFaxInboundRoutingMethod_INTERFACE_DEFINED__

/* interface IFaxInboundRoutingMethod */
/* [unique][helpstring][dual][uuid][object] */ 


EXTERN_C const IID IID_IFaxInboundRoutingMethod;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("45700061-AD9D-4776-A8C4-64065492CF4B")
    IFaxInboundRoutingMethod : public IDispatch
    {
    public:
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Name( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrName) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_GUID( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrGUID) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_FunctionName( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrFunctionName) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_ExtensionFriendlyName( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrExtensionFriendlyName) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_ExtensionImageName( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrExtensionImageName) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Priority( 
            /* [retval][out] */ __RPC__out long *plPriority) = 0;
        
        virtual /* [helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_Priority( 
            /* [in] */ long lPriority) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE Refresh( void) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE Save( void) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IFaxInboundRoutingMethodVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IFaxInboundRoutingMethod * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IFaxInboundRoutingMethod * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IFaxInboundRoutingMethod * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IFaxInboundRoutingMethod * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IFaxInboundRoutingMethod * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IFaxInboundRoutingMethod * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IFaxInboundRoutingMethod * This,
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
        
        DECLSPEC_XFGVIRT(IFaxInboundRoutingMethod, get_Name)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Name )( 
            __RPC__in IFaxInboundRoutingMethod * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrName);
        
        DECLSPEC_XFGVIRT(IFaxInboundRoutingMethod, get_GUID)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_GUID )( 
            __RPC__in IFaxInboundRoutingMethod * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrGUID);
        
        DECLSPEC_XFGVIRT(IFaxInboundRoutingMethod, get_FunctionName)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_FunctionName )( 
            __RPC__in IFaxInboundRoutingMethod * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrFunctionName);
        
        DECLSPEC_XFGVIRT(IFaxInboundRoutingMethod, get_ExtensionFriendlyName)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_ExtensionFriendlyName )( 
            __RPC__in IFaxInboundRoutingMethod * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrExtensionFriendlyName);
        
        DECLSPEC_XFGVIRT(IFaxInboundRoutingMethod, get_ExtensionImageName)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_ExtensionImageName )( 
            __RPC__in IFaxInboundRoutingMethod * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrExtensionImageName);
        
        DECLSPEC_XFGVIRT(IFaxInboundRoutingMethod, get_Priority)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Priority )( 
            __RPC__in IFaxInboundRoutingMethod * This,
            /* [retval][out] */ __RPC__out long *plPriority);
        
        DECLSPEC_XFGVIRT(IFaxInboundRoutingMethod, put_Priority)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_Priority )( 
            __RPC__in IFaxInboundRoutingMethod * This,
            /* [in] */ long lPriority);
        
        DECLSPEC_XFGVIRT(IFaxInboundRoutingMethod, Refresh)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *Refresh )( 
            __RPC__in IFaxInboundRoutingMethod * This);
        
        DECLSPEC_XFGVIRT(IFaxInboundRoutingMethod, Save)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *Save )( 
            __RPC__in IFaxInboundRoutingMethod * This);
        
        END_INTERFACE
    } IFaxInboundRoutingMethodVtbl;

    interface IFaxInboundRoutingMethod
    {
        CONST_VTBL struct IFaxInboundRoutingMethodVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IFaxInboundRoutingMethod_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IFaxInboundRoutingMethod_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IFaxInboundRoutingMethod_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IFaxInboundRoutingMethod_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IFaxInboundRoutingMethod_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IFaxInboundRoutingMethod_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IFaxInboundRoutingMethod_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IFaxInboundRoutingMethod_get_Name(This,pbstrName)	\
    ( (This)->lpVtbl -> get_Name(This,pbstrName) ) 

#define IFaxInboundRoutingMethod_get_GUID(This,pbstrGUID)	\
    ( (This)->lpVtbl -> get_GUID(This,pbstrGUID) ) 

#define IFaxInboundRoutingMethod_get_FunctionName(This,pbstrFunctionName)	\
    ( (This)->lpVtbl -> get_FunctionName(This,pbstrFunctionName) ) 

#define IFaxInboundRoutingMethod_get_ExtensionFriendlyName(This,pbstrExtensionFriendlyName)	\
    ( (This)->lpVtbl -> get_ExtensionFriendlyName(This,pbstrExtensionFriendlyName) ) 

#define IFaxInboundRoutingMethod_get_ExtensionImageName(This,pbstrExtensionImageName)	\
    ( (This)->lpVtbl -> get_ExtensionImageName(This,pbstrExtensionImageName) ) 

#define IFaxInboundRoutingMethod_get_Priority(This,plPriority)	\
    ( (This)->lpVtbl -> get_Priority(This,plPriority) ) 

#define IFaxInboundRoutingMethod_put_Priority(This,lPriority)	\
    ( (This)->lpVtbl -> put_Priority(This,lPriority) ) 

#define IFaxInboundRoutingMethod_Refresh(This)	\
    ( (This)->lpVtbl -> Refresh(This) ) 

#define IFaxInboundRoutingMethod_Save(This)	\
    ( (This)->lpVtbl -> Save(This) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IFaxInboundRoutingMethod_INTERFACE_DEFINED__ */


#ifndef __IFaxDocument2_INTERFACE_DEFINED__
#define __IFaxDocument2_INTERFACE_DEFINED__

/* interface IFaxDocument2 */
/* [nonextensible][unique][helpstring][dual][uuid][object] */ 


EXTERN_C const IID IID_IFaxDocument2;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("e1347661-f9ef-4d6d-b4a5-c0a068b65cff")
    IFaxDocument2 : public IFaxDocument
    {
    public:
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_SubmissionId( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrSubmissionId) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Bodies( 
            /* [retval][out] */ __RPC__out VARIANT *pvBodies) = 0;
        
        virtual /* [helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_Bodies( 
            /* [in] */ VARIANT vBodies) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE Submit2( 
            /* [in] */ __RPC__in BSTR bstrFaxServerName,
            /* [out] */ __RPC__out VARIANT *pvFaxOutgoingJobIDs,
            /* [retval][out] */ __RPC__out long *plErrorBodyFile) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE ConnectedSubmit2( 
            /* [in] */ __RPC__in_opt IFaxServer *pFaxServer,
            /* [out] */ __RPC__out VARIANT *pvFaxOutgoingJobIDs,
            /* [retval][out] */ __RPC__out long *plErrorBodyFile) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IFaxDocument2Vtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IFaxDocument2 * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IFaxDocument2 * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IFaxDocument2 * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IFaxDocument2 * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IFaxDocument2 * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IFaxDocument2 * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IFaxDocument2 * This,
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
        
        DECLSPEC_XFGVIRT(IFaxDocument, get_Body)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Body )( 
            __RPC__in IFaxDocument2 * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrBody);
        
        DECLSPEC_XFGVIRT(IFaxDocument, put_Body)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_Body )( 
            __RPC__in IFaxDocument2 * This,
            /* [in] */ __RPC__in BSTR bstrBody);
        
        DECLSPEC_XFGVIRT(IFaxDocument, get_Sender)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Sender )( 
            __RPC__in IFaxDocument2 * This,
            /* [retval][out] */ __RPC__deref_out_opt IFaxSender **ppFaxSender);
        
        DECLSPEC_XFGVIRT(IFaxDocument, get_Recipients)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Recipients )( 
            __RPC__in IFaxDocument2 * This,
            /* [retval][out] */ __RPC__deref_out_opt IFaxRecipients **ppFaxRecipients);
        
        DECLSPEC_XFGVIRT(IFaxDocument, get_CoverPage)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_CoverPage )( 
            __RPC__in IFaxDocument2 * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrCoverPage);
        
        DECLSPEC_XFGVIRT(IFaxDocument, put_CoverPage)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_CoverPage )( 
            __RPC__in IFaxDocument2 * This,
            /* [in] */ __RPC__in BSTR bstrCoverPage);
        
        DECLSPEC_XFGVIRT(IFaxDocument, get_Subject)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Subject )( 
            __RPC__in IFaxDocument2 * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrSubject);
        
        DECLSPEC_XFGVIRT(IFaxDocument, put_Subject)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_Subject )( 
            __RPC__in IFaxDocument2 * This,
            /* [in] */ __RPC__in BSTR bstrSubject);
        
        DECLSPEC_XFGVIRT(IFaxDocument, get_Note)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Note )( 
            __RPC__in IFaxDocument2 * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrNote);
        
        DECLSPEC_XFGVIRT(IFaxDocument, put_Note)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_Note )( 
            __RPC__in IFaxDocument2 * This,
            /* [in] */ __RPC__in BSTR bstrNote);
        
        DECLSPEC_XFGVIRT(IFaxDocument, get_ScheduleTime)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_ScheduleTime )( 
            __RPC__in IFaxDocument2 * This,
            /* [retval][out] */ __RPC__out DATE *pdateScheduleTime);
        
        DECLSPEC_XFGVIRT(IFaxDocument, put_ScheduleTime)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_ScheduleTime )( 
            __RPC__in IFaxDocument2 * This,
            /* [in] */ DATE dateScheduleTime);
        
        DECLSPEC_XFGVIRT(IFaxDocument, get_ReceiptAddress)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_ReceiptAddress )( 
            __RPC__in IFaxDocument2 * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrReceiptAddress);
        
        DECLSPEC_XFGVIRT(IFaxDocument, put_ReceiptAddress)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_ReceiptAddress )( 
            __RPC__in IFaxDocument2 * This,
            /* [in] */ __RPC__in BSTR bstrReceiptAddress);
        
        DECLSPEC_XFGVIRT(IFaxDocument, get_DocumentName)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_DocumentName )( 
            __RPC__in IFaxDocument2 * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrDocumentName);
        
        DECLSPEC_XFGVIRT(IFaxDocument, put_DocumentName)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_DocumentName )( 
            __RPC__in IFaxDocument2 * This,
            /* [in] */ __RPC__in BSTR bstrDocumentName);
        
        DECLSPEC_XFGVIRT(IFaxDocument, get_CallHandle)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_CallHandle )( 
            __RPC__in IFaxDocument2 * This,
            /* [retval][out] */ __RPC__out long *plCallHandle);
        
        DECLSPEC_XFGVIRT(IFaxDocument, put_CallHandle)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_CallHandle )( 
            __RPC__in IFaxDocument2 * This,
            /* [in] */ long lCallHandle);
        
        DECLSPEC_XFGVIRT(IFaxDocument, get_CoverPageType)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_CoverPageType )( 
            __RPC__in IFaxDocument2 * This,
            /* [retval][out] */ __RPC__out FAX_COVERPAGE_TYPE_ENUM *pCoverPageType);
        
        DECLSPEC_XFGVIRT(IFaxDocument, put_CoverPageType)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_CoverPageType )( 
            __RPC__in IFaxDocument2 * This,
            /* [in] */ FAX_COVERPAGE_TYPE_ENUM CoverPageType);
        
        DECLSPEC_XFGVIRT(IFaxDocument, get_ScheduleType)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_ScheduleType )( 
            __RPC__in IFaxDocument2 * This,
            /* [retval][out] */ __RPC__out FAX_SCHEDULE_TYPE_ENUM *pScheduleType);
        
        DECLSPEC_XFGVIRT(IFaxDocument, put_ScheduleType)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_ScheduleType )( 
            __RPC__in IFaxDocument2 * This,
            /* [in] */ FAX_SCHEDULE_TYPE_ENUM ScheduleType);
        
        DECLSPEC_XFGVIRT(IFaxDocument, get_ReceiptType)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_ReceiptType )( 
            __RPC__in IFaxDocument2 * This,
            /* [retval][out] */ __RPC__out FAX_RECEIPT_TYPE_ENUM *pReceiptType);
        
        DECLSPEC_XFGVIRT(IFaxDocument, put_ReceiptType)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_ReceiptType )( 
            __RPC__in IFaxDocument2 * This,
            /* [in] */ FAX_RECEIPT_TYPE_ENUM ReceiptType);
        
        DECLSPEC_XFGVIRT(IFaxDocument, get_GroupBroadcastReceipts)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_GroupBroadcastReceipts )( 
            __RPC__in IFaxDocument2 * This,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pbUseGrouping);
        
        DECLSPEC_XFGVIRT(IFaxDocument, put_GroupBroadcastReceipts)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_GroupBroadcastReceipts )( 
            __RPC__in IFaxDocument2 * This,
            /* [in] */ VARIANT_BOOL bUseGrouping);
        
        DECLSPEC_XFGVIRT(IFaxDocument, get_Priority)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Priority )( 
            __RPC__in IFaxDocument2 * This,
            /* [retval][out] */ __RPC__out FAX_PRIORITY_TYPE_ENUM *pPriority);
        
        DECLSPEC_XFGVIRT(IFaxDocument, put_Priority)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_Priority )( 
            __RPC__in IFaxDocument2 * This,
            /* [in] */ FAX_PRIORITY_TYPE_ENUM Priority);
        
        DECLSPEC_XFGVIRT(IFaxDocument, get_TapiConnection)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_TapiConnection )( 
            __RPC__in IFaxDocument2 * This,
            /* [retval][out] */ __RPC__deref_out_opt IDispatch **ppTapiConnection);
        
        DECLSPEC_XFGVIRT(IFaxDocument, putref_TapiConnection)
        /* [helpstring][id][propputref] */ HRESULT ( STDMETHODCALLTYPE *putref_TapiConnection )( 
            __RPC__in IFaxDocument2 * This,
            /* [in] */ __RPC__in_opt IDispatch *pTapiConnection);
        
        DECLSPEC_XFGVIRT(IFaxDocument, Submit)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *Submit )( 
            __RPC__in IFaxDocument2 * This,
            /* [in] */ __RPC__in BSTR bstrFaxServerName,
            /* [retval][out] */ __RPC__out VARIANT *pvFaxOutgoingJobIDs);
        
        DECLSPEC_XFGVIRT(IFaxDocument, ConnectedSubmit)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *ConnectedSubmit )( 
            __RPC__in IFaxDocument2 * This,
            /* [in] */ __RPC__in_opt IFaxServer *pFaxServer,
            /* [retval][out] */ __RPC__out VARIANT *pvFaxOutgoingJobIDs);
        
        DECLSPEC_XFGVIRT(IFaxDocument, get_AttachFaxToReceipt)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_AttachFaxToReceipt )( 
            __RPC__in IFaxDocument2 * This,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pbAttachFax);
        
        DECLSPEC_XFGVIRT(IFaxDocument, put_AttachFaxToReceipt)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_AttachFaxToReceipt )( 
            __RPC__in IFaxDocument2 * This,
            /* [in] */ VARIANT_BOOL bAttachFax);
        
        DECLSPEC_XFGVIRT(IFaxDocument2, get_SubmissionId)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_SubmissionId )( 
            __RPC__in IFaxDocument2 * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrSubmissionId);
        
        DECLSPEC_XFGVIRT(IFaxDocument2, get_Bodies)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Bodies )( 
            __RPC__in IFaxDocument2 * This,
            /* [retval][out] */ __RPC__out VARIANT *pvBodies);
        
        DECLSPEC_XFGVIRT(IFaxDocument2, put_Bodies)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_Bodies )( 
            __RPC__in IFaxDocument2 * This,
            /* [in] */ VARIANT vBodies);
        
        DECLSPEC_XFGVIRT(IFaxDocument2, Submit2)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *Submit2 )( 
            __RPC__in IFaxDocument2 * This,
            /* [in] */ __RPC__in BSTR bstrFaxServerName,
            /* [out] */ __RPC__out VARIANT *pvFaxOutgoingJobIDs,
            /* [retval][out] */ __RPC__out long *plErrorBodyFile);
        
        DECLSPEC_XFGVIRT(IFaxDocument2, ConnectedSubmit2)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *ConnectedSubmit2 )( 
            __RPC__in IFaxDocument2 * This,
            /* [in] */ __RPC__in_opt IFaxServer *pFaxServer,
            /* [out] */ __RPC__out VARIANT *pvFaxOutgoingJobIDs,
            /* [retval][out] */ __RPC__out long *plErrorBodyFile);
        
        END_INTERFACE
    } IFaxDocument2Vtbl;

    interface IFaxDocument2
    {
        CONST_VTBL struct IFaxDocument2Vtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IFaxDocument2_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IFaxDocument2_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IFaxDocument2_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IFaxDocument2_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IFaxDocument2_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IFaxDocument2_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IFaxDocument2_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IFaxDocument2_get_Body(This,pbstrBody)	\
    ( (This)->lpVtbl -> get_Body(This,pbstrBody) ) 

#define IFaxDocument2_put_Body(This,bstrBody)	\
    ( (This)->lpVtbl -> put_Body(This,bstrBody) ) 

#define IFaxDocument2_get_Sender(This,ppFaxSender)	\
    ( (This)->lpVtbl -> get_Sender(This,ppFaxSender) ) 

#define IFaxDocument2_get_Recipients(This,ppFaxRecipients)	\
    ( (This)->lpVtbl -> get_Recipients(This,ppFaxRecipients) ) 

#define IFaxDocument2_get_CoverPage(This,pbstrCoverPage)	\
    ( (This)->lpVtbl -> get_CoverPage(This,pbstrCoverPage) ) 

#define IFaxDocument2_put_CoverPage(This,bstrCoverPage)	\
    ( (This)->lpVtbl -> put_CoverPage(This,bstrCoverPage) ) 

#define IFaxDocument2_get_Subject(This,pbstrSubject)	\
    ( (This)->lpVtbl -> get_Subject(This,pbstrSubject) ) 

#define IFaxDocument2_put_Subject(This,bstrSubject)	\
    ( (This)->lpVtbl -> put_Subject(This,bstrSubject) ) 

#define IFaxDocument2_get_Note(This,pbstrNote)	\
    ( (This)->lpVtbl -> get_Note(This,pbstrNote) ) 

#define IFaxDocument2_put_Note(This,bstrNote)	\
    ( (This)->lpVtbl -> put_Note(This,bstrNote) ) 

#define IFaxDocument2_get_ScheduleTime(This,pdateScheduleTime)	\
    ( (This)->lpVtbl -> get_ScheduleTime(This,pdateScheduleTime) ) 

#define IFaxDocument2_put_ScheduleTime(This,dateScheduleTime)	\
    ( (This)->lpVtbl -> put_ScheduleTime(This,dateScheduleTime) ) 

#define IFaxDocument2_get_ReceiptAddress(This,pbstrReceiptAddress)	\
    ( (This)->lpVtbl -> get_ReceiptAddress(This,pbstrReceiptAddress) ) 

#define IFaxDocument2_put_ReceiptAddress(This,bstrReceiptAddress)	\
    ( (This)->lpVtbl -> put_ReceiptAddress(This,bstrReceiptAddress) ) 

#define IFaxDocument2_get_DocumentName(This,pbstrDocumentName)	\
    ( (This)->lpVtbl -> get_DocumentName(This,pbstrDocumentName) ) 

#define IFaxDocument2_put_DocumentName(This,bstrDocumentName)	\
    ( (This)->lpVtbl -> put_DocumentName(This,bstrDocumentName) ) 

#define IFaxDocument2_get_CallHandle(This,plCallHandle)	\
    ( (This)->lpVtbl -> get_CallHandle(This,plCallHandle) ) 

#define IFaxDocument2_put_CallHandle(This,lCallHandle)	\
    ( (This)->lpVtbl -> put_CallHandle(This,lCallHandle) ) 

#define IFaxDocument2_get_CoverPageType(This,pCoverPageType)	\
    ( (This)->lpVtbl -> get_CoverPageType(This,pCoverPageType) ) 

#define IFaxDocument2_put_CoverPageType(This,CoverPageType)	\
    ( (This)->lpVtbl -> put_CoverPageType(This,CoverPageType) ) 

#define IFaxDocument2_get_ScheduleType(This,pScheduleType)	\
    ( (This)->lpVtbl -> get_ScheduleType(This,pScheduleType) ) 

#define IFaxDocument2_put_ScheduleType(This,ScheduleType)	\
    ( (This)->lpVtbl -> put_ScheduleType(This,ScheduleType) ) 

#define IFaxDocument2_get_ReceiptType(This,pReceiptType)	\
    ( (This)->lpVtbl -> get_ReceiptType(This,pReceiptType) ) 

#define IFaxDocument2_put_ReceiptType(This,ReceiptType)	\
    ( (This)->lpVtbl -> put_ReceiptType(This,ReceiptType) ) 

#define IFaxDocument2_get_GroupBroadcastReceipts(This,pbUseGrouping)	\
    ( (This)->lpVtbl -> get_GroupBroadcastReceipts(This,pbUseGrouping) ) 

#define IFaxDocument2_put_GroupBroadcastReceipts(This,bUseGrouping)	\
    ( (This)->lpVtbl -> put_GroupBroadcastReceipts(This,bUseGrouping) ) 

#define IFaxDocument2_get_Priority(This,pPriority)	\
    ( (This)->lpVtbl -> get_Priority(This,pPriority) ) 

#define IFaxDocument2_put_Priority(This,Priority)	\
    ( (This)->lpVtbl -> put_Priority(This,Priority) ) 

#define IFaxDocument2_get_TapiConnection(This,ppTapiConnection)	\
    ( (This)->lpVtbl -> get_TapiConnection(This,ppTapiConnection) ) 

#define IFaxDocument2_putref_TapiConnection(This,pTapiConnection)	\
    ( (This)->lpVtbl -> putref_TapiConnection(This,pTapiConnection) ) 

#define IFaxDocument2_Submit(This,bstrFaxServerName,pvFaxOutgoingJobIDs)	\
    ( (This)->lpVtbl -> Submit(This,bstrFaxServerName,pvFaxOutgoingJobIDs) ) 

#define IFaxDocument2_ConnectedSubmit(This,pFaxServer,pvFaxOutgoingJobIDs)	\
    ( (This)->lpVtbl -> ConnectedSubmit(This,pFaxServer,pvFaxOutgoingJobIDs) ) 

#define IFaxDocument2_get_AttachFaxToReceipt(This,pbAttachFax)	\
    ( (This)->lpVtbl -> get_AttachFaxToReceipt(This,pbAttachFax) ) 

#define IFaxDocument2_put_AttachFaxToReceipt(This,bAttachFax)	\
    ( (This)->lpVtbl -> put_AttachFaxToReceipt(This,bAttachFax) ) 


#define IFaxDocument2_get_SubmissionId(This,pbstrSubmissionId)	\
    ( (This)->lpVtbl -> get_SubmissionId(This,pbstrSubmissionId) ) 

#define IFaxDocument2_get_Bodies(This,pvBodies)	\
    ( (This)->lpVtbl -> get_Bodies(This,pvBodies) ) 

#define IFaxDocument2_put_Bodies(This,vBodies)	\
    ( (This)->lpVtbl -> put_Bodies(This,vBodies) ) 

#define IFaxDocument2_Submit2(This,bstrFaxServerName,pvFaxOutgoingJobIDs,plErrorBodyFile)	\
    ( (This)->lpVtbl -> Submit2(This,bstrFaxServerName,pvFaxOutgoingJobIDs,plErrorBodyFile) ) 

#define IFaxDocument2_ConnectedSubmit2(This,pFaxServer,pvFaxOutgoingJobIDs,plErrorBodyFile)	\
    ( (This)->lpVtbl -> ConnectedSubmit2(This,pFaxServer,pvFaxOutgoingJobIDs,plErrorBodyFile) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IFaxDocument2_INTERFACE_DEFINED__ */


#ifndef __IFaxConfiguration_INTERFACE_DEFINED__
#define __IFaxConfiguration_INTERFACE_DEFINED__

/* interface IFaxConfiguration */
/* [unique][helpstring][dual][uuid][object] */ 


EXTERN_C const IID IID_IFaxConfiguration;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("10f4d0f7-0994-4543-ab6e-506949128c40")
    IFaxConfiguration : public IDispatch
    {
    public:
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_UseArchive( 
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pbUseArchive) = 0;
        
        virtual /* [helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_UseArchive( 
            /* [in] */ VARIANT_BOOL bUseArchive) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_ArchiveLocation( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrArchiveLocation) = 0;
        
        virtual /* [helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_ArchiveLocation( 
            /* [in] */ __RPC__in BSTR bstrArchiveLocation) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_SizeQuotaWarning( 
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pbSizeQuotaWarning) = 0;
        
        virtual /* [helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_SizeQuotaWarning( 
            /* [in] */ VARIANT_BOOL bSizeQuotaWarning) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_HighQuotaWaterMark( 
            /* [retval][out] */ __RPC__out long *plHighQuotaWaterMark) = 0;
        
        virtual /* [helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_HighQuotaWaterMark( 
            /* [in] */ long lHighQuotaWaterMark) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_LowQuotaWaterMark( 
            /* [retval][out] */ __RPC__out long *plLowQuotaWaterMark) = 0;
        
        virtual /* [helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_LowQuotaWaterMark( 
            /* [in] */ long lLowQuotaWaterMark) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_ArchiveAgeLimit( 
            /* [retval][out] */ __RPC__out long *plArchiveAgeLimit) = 0;
        
        virtual /* [helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_ArchiveAgeLimit( 
            /* [in] */ long lArchiveAgeLimit) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_ArchiveSizeLow( 
            /* [retval][out] */ __RPC__out long *plSizeLow) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_ArchiveSizeHigh( 
            /* [retval][out] */ __RPC__out long *plSizeHigh) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_OutgoingQueueBlocked( 
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pbOutgoingBlocked) = 0;
        
        virtual /* [helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_OutgoingQueueBlocked( 
            /* [in] */ VARIANT_BOOL bOutgoingBlocked) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_OutgoingQueuePaused( 
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pbOutgoingPaused) = 0;
        
        virtual /* [helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_OutgoingQueuePaused( 
            /* [in] */ VARIANT_BOOL bOutgoingPaused) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_AllowPersonalCoverPages( 
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pbAllowPersonalCoverPages) = 0;
        
        virtual /* [helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_AllowPersonalCoverPages( 
            /* [in] */ VARIANT_BOOL bAllowPersonalCoverPages) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_UseDeviceTSID( 
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pbUseDeviceTSID) = 0;
        
        virtual /* [helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_UseDeviceTSID( 
            /* [in] */ VARIANT_BOOL bUseDeviceTSID) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Retries( 
            /* [retval][out] */ __RPC__out long *plRetries) = 0;
        
        virtual /* [helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_Retries( 
            /* [in] */ long lRetries) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_RetryDelay( 
            /* [retval][out] */ __RPC__out long *plRetryDelay) = 0;
        
        virtual /* [helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_RetryDelay( 
            /* [in] */ long lRetryDelay) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_DiscountRateStart( 
            /* [retval][out] */ __RPC__out DATE *pdateDiscountRateStart) = 0;
        
        virtual /* [helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_DiscountRateStart( 
            /* [in] */ DATE dateDiscountRateStart) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_DiscountRateEnd( 
            /* [retval][out] */ __RPC__out DATE *pdateDiscountRateEnd) = 0;
        
        virtual /* [helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_DiscountRateEnd( 
            /* [in] */ DATE dateDiscountRateEnd) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_OutgoingQueueAgeLimit( 
            /* [retval][out] */ __RPC__out long *plOutgoingQueueAgeLimit) = 0;
        
        virtual /* [helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_OutgoingQueueAgeLimit( 
            /* [in] */ long lOutgoingQueueAgeLimit) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Branding( 
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pbBranding) = 0;
        
        virtual /* [helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_Branding( 
            /* [in] */ VARIANT_BOOL bBranding) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_IncomingQueueBlocked( 
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pbIncomingBlocked) = 0;
        
        virtual /* [helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_IncomingQueueBlocked( 
            /* [in] */ VARIANT_BOOL bIncomingBlocked) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_AutoCreateAccountOnConnect( 
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pbAutoCreateAccountOnConnect) = 0;
        
        virtual /* [helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_AutoCreateAccountOnConnect( 
            /* [in] */ VARIANT_BOOL bAutoCreateAccountOnConnect) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_IncomingFaxesArePublic( 
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pbIncomingFaxesArePublic) = 0;
        
        virtual /* [helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_IncomingFaxesArePublic( 
            /* [in] */ VARIANT_BOOL bIncomingFaxesArePublic) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE Refresh( void) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE Save( void) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IFaxConfigurationVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IFaxConfiguration * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IFaxConfiguration * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IFaxConfiguration * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IFaxConfiguration * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IFaxConfiguration * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IFaxConfiguration * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IFaxConfiguration * This,
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
        
        DECLSPEC_XFGVIRT(IFaxConfiguration, get_UseArchive)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_UseArchive )( 
            __RPC__in IFaxConfiguration * This,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pbUseArchive);
        
        DECLSPEC_XFGVIRT(IFaxConfiguration, put_UseArchive)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_UseArchive )( 
            __RPC__in IFaxConfiguration * This,
            /* [in] */ VARIANT_BOOL bUseArchive);
        
        DECLSPEC_XFGVIRT(IFaxConfiguration, get_ArchiveLocation)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_ArchiveLocation )( 
            __RPC__in IFaxConfiguration * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrArchiveLocation);
        
        DECLSPEC_XFGVIRT(IFaxConfiguration, put_ArchiveLocation)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_ArchiveLocation )( 
            __RPC__in IFaxConfiguration * This,
            /* [in] */ __RPC__in BSTR bstrArchiveLocation);
        
        DECLSPEC_XFGVIRT(IFaxConfiguration, get_SizeQuotaWarning)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_SizeQuotaWarning )( 
            __RPC__in IFaxConfiguration * This,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pbSizeQuotaWarning);
        
        DECLSPEC_XFGVIRT(IFaxConfiguration, put_SizeQuotaWarning)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_SizeQuotaWarning )( 
            __RPC__in IFaxConfiguration * This,
            /* [in] */ VARIANT_BOOL bSizeQuotaWarning);
        
        DECLSPEC_XFGVIRT(IFaxConfiguration, get_HighQuotaWaterMark)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_HighQuotaWaterMark )( 
            __RPC__in IFaxConfiguration * This,
            /* [retval][out] */ __RPC__out long *plHighQuotaWaterMark);
        
        DECLSPEC_XFGVIRT(IFaxConfiguration, put_HighQuotaWaterMark)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_HighQuotaWaterMark )( 
            __RPC__in IFaxConfiguration * This,
            /* [in] */ long lHighQuotaWaterMark);
        
        DECLSPEC_XFGVIRT(IFaxConfiguration, get_LowQuotaWaterMark)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_LowQuotaWaterMark )( 
            __RPC__in IFaxConfiguration * This,
            /* [retval][out] */ __RPC__out long *plLowQuotaWaterMark);
        
        DECLSPEC_XFGVIRT(IFaxConfiguration, put_LowQuotaWaterMark)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_LowQuotaWaterMark )( 
            __RPC__in IFaxConfiguration * This,
            /* [in] */ long lLowQuotaWaterMark);
        
        DECLSPEC_XFGVIRT(IFaxConfiguration, get_ArchiveAgeLimit)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_ArchiveAgeLimit )( 
            __RPC__in IFaxConfiguration * This,
            /* [retval][out] */ __RPC__out long *plArchiveAgeLimit);
        
        DECLSPEC_XFGVIRT(IFaxConfiguration, put_ArchiveAgeLimit)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_ArchiveAgeLimit )( 
            __RPC__in IFaxConfiguration * This,
            /* [in] */ long lArchiveAgeLimit);
        
        DECLSPEC_XFGVIRT(IFaxConfiguration, get_ArchiveSizeLow)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_ArchiveSizeLow )( 
            __RPC__in IFaxConfiguration * This,
            /* [retval][out] */ __RPC__out long *plSizeLow);
        
        DECLSPEC_XFGVIRT(IFaxConfiguration, get_ArchiveSizeHigh)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_ArchiveSizeHigh )( 
            __RPC__in IFaxConfiguration * This,
            /* [retval][out] */ __RPC__out long *plSizeHigh);
        
        DECLSPEC_XFGVIRT(IFaxConfiguration, get_OutgoingQueueBlocked)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_OutgoingQueueBlocked )( 
            __RPC__in IFaxConfiguration * This,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pbOutgoingBlocked);
        
        DECLSPEC_XFGVIRT(IFaxConfiguration, put_OutgoingQueueBlocked)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_OutgoingQueueBlocked )( 
            __RPC__in IFaxConfiguration * This,
            /* [in] */ VARIANT_BOOL bOutgoingBlocked);
        
        DECLSPEC_XFGVIRT(IFaxConfiguration, get_OutgoingQueuePaused)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_OutgoingQueuePaused )( 
            __RPC__in IFaxConfiguration * This,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pbOutgoingPaused);
        
        DECLSPEC_XFGVIRT(IFaxConfiguration, put_OutgoingQueuePaused)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_OutgoingQueuePaused )( 
            __RPC__in IFaxConfiguration * This,
            /* [in] */ VARIANT_BOOL bOutgoingPaused);
        
        DECLSPEC_XFGVIRT(IFaxConfiguration, get_AllowPersonalCoverPages)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_AllowPersonalCoverPages )( 
            __RPC__in IFaxConfiguration * This,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pbAllowPersonalCoverPages);
        
        DECLSPEC_XFGVIRT(IFaxConfiguration, put_AllowPersonalCoverPages)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_AllowPersonalCoverPages )( 
            __RPC__in IFaxConfiguration * This,
            /* [in] */ VARIANT_BOOL bAllowPersonalCoverPages);
        
        DECLSPEC_XFGVIRT(IFaxConfiguration, get_UseDeviceTSID)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_UseDeviceTSID )( 
            __RPC__in IFaxConfiguration * This,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pbUseDeviceTSID);
        
        DECLSPEC_XFGVIRT(IFaxConfiguration, put_UseDeviceTSID)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_UseDeviceTSID )( 
            __RPC__in IFaxConfiguration * This,
            /* [in] */ VARIANT_BOOL bUseDeviceTSID);
        
        DECLSPEC_XFGVIRT(IFaxConfiguration, get_Retries)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Retries )( 
            __RPC__in IFaxConfiguration * This,
            /* [retval][out] */ __RPC__out long *plRetries);
        
        DECLSPEC_XFGVIRT(IFaxConfiguration, put_Retries)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_Retries )( 
            __RPC__in IFaxConfiguration * This,
            /* [in] */ long lRetries);
        
        DECLSPEC_XFGVIRT(IFaxConfiguration, get_RetryDelay)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_RetryDelay )( 
            __RPC__in IFaxConfiguration * This,
            /* [retval][out] */ __RPC__out long *plRetryDelay);
        
        DECLSPEC_XFGVIRT(IFaxConfiguration, put_RetryDelay)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_RetryDelay )( 
            __RPC__in IFaxConfiguration * This,
            /* [in] */ long lRetryDelay);
        
        DECLSPEC_XFGVIRT(IFaxConfiguration, get_DiscountRateStart)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_DiscountRateStart )( 
            __RPC__in IFaxConfiguration * This,
            /* [retval][out] */ __RPC__out DATE *pdateDiscountRateStart);
        
        DECLSPEC_XFGVIRT(IFaxConfiguration, put_DiscountRateStart)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_DiscountRateStart )( 
            __RPC__in IFaxConfiguration * This,
            /* [in] */ DATE dateDiscountRateStart);
        
        DECLSPEC_XFGVIRT(IFaxConfiguration, get_DiscountRateEnd)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_DiscountRateEnd )( 
            __RPC__in IFaxConfiguration * This,
            /* [retval][out] */ __RPC__out DATE *pdateDiscountRateEnd);
        
        DECLSPEC_XFGVIRT(IFaxConfiguration, put_DiscountRateEnd)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_DiscountRateEnd )( 
            __RPC__in IFaxConfiguration * This,
            /* [in] */ DATE dateDiscountRateEnd);
        
        DECLSPEC_XFGVIRT(IFaxConfiguration, get_OutgoingQueueAgeLimit)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_OutgoingQueueAgeLimit )( 
            __RPC__in IFaxConfiguration * This,
            /* [retval][out] */ __RPC__out long *plOutgoingQueueAgeLimit);
        
        DECLSPEC_XFGVIRT(IFaxConfiguration, put_OutgoingQueueAgeLimit)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_OutgoingQueueAgeLimit )( 
            __RPC__in IFaxConfiguration * This,
            /* [in] */ long lOutgoingQueueAgeLimit);
        
        DECLSPEC_XFGVIRT(IFaxConfiguration, get_Branding)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Branding )( 
            __RPC__in IFaxConfiguration * This,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pbBranding);
        
        DECLSPEC_XFGVIRT(IFaxConfiguration, put_Branding)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_Branding )( 
            __RPC__in IFaxConfiguration * This,
            /* [in] */ VARIANT_BOOL bBranding);
        
        DECLSPEC_XFGVIRT(IFaxConfiguration, get_IncomingQueueBlocked)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_IncomingQueueBlocked )( 
            __RPC__in IFaxConfiguration * This,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pbIncomingBlocked);
        
        DECLSPEC_XFGVIRT(IFaxConfiguration, put_IncomingQueueBlocked)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_IncomingQueueBlocked )( 
            __RPC__in IFaxConfiguration * This,
            /* [in] */ VARIANT_BOOL bIncomingBlocked);
        
        DECLSPEC_XFGVIRT(IFaxConfiguration, get_AutoCreateAccountOnConnect)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_AutoCreateAccountOnConnect )( 
            __RPC__in IFaxConfiguration * This,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pbAutoCreateAccountOnConnect);
        
        DECLSPEC_XFGVIRT(IFaxConfiguration, put_AutoCreateAccountOnConnect)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_AutoCreateAccountOnConnect )( 
            __RPC__in IFaxConfiguration * This,
            /* [in] */ VARIANT_BOOL bAutoCreateAccountOnConnect);
        
        DECLSPEC_XFGVIRT(IFaxConfiguration, get_IncomingFaxesArePublic)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_IncomingFaxesArePublic )( 
            __RPC__in IFaxConfiguration * This,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pbIncomingFaxesArePublic);
        
        DECLSPEC_XFGVIRT(IFaxConfiguration, put_IncomingFaxesArePublic)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_IncomingFaxesArePublic )( 
            __RPC__in IFaxConfiguration * This,
            /* [in] */ VARIANT_BOOL bIncomingFaxesArePublic);
        
        DECLSPEC_XFGVIRT(IFaxConfiguration, Refresh)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *Refresh )( 
            __RPC__in IFaxConfiguration * This);
        
        DECLSPEC_XFGVIRT(IFaxConfiguration, Save)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *Save )( 
            __RPC__in IFaxConfiguration * This);
        
        END_INTERFACE
    } IFaxConfigurationVtbl;

    interface IFaxConfiguration
    {
        CONST_VTBL struct IFaxConfigurationVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IFaxConfiguration_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IFaxConfiguration_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IFaxConfiguration_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IFaxConfiguration_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IFaxConfiguration_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IFaxConfiguration_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IFaxConfiguration_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IFaxConfiguration_get_UseArchive(This,pbUseArchive)	\
    ( (This)->lpVtbl -> get_UseArchive(This,pbUseArchive) ) 

#define IFaxConfiguration_put_UseArchive(This,bUseArchive)	\
    ( (This)->lpVtbl -> put_UseArchive(This,bUseArchive) ) 

#define IFaxConfiguration_get_ArchiveLocation(This,pbstrArchiveLocation)	\
    ( (This)->lpVtbl -> get_ArchiveLocation(This,pbstrArchiveLocation) ) 

#define IFaxConfiguration_put_ArchiveLocation(This,bstrArchiveLocation)	\
    ( (This)->lpVtbl -> put_ArchiveLocation(This,bstrArchiveLocation) ) 

#define IFaxConfiguration_get_SizeQuotaWarning(This,pbSizeQuotaWarning)	\
    ( (This)->lpVtbl -> get_SizeQuotaWarning(This,pbSizeQuotaWarning) ) 

#define IFaxConfiguration_put_SizeQuotaWarning(This,bSizeQuotaWarning)	\
    ( (This)->lpVtbl -> put_SizeQuotaWarning(This,bSizeQuotaWarning) ) 

#define IFaxConfiguration_get_HighQuotaWaterMark(This,plHighQuotaWaterMark)	\
    ( (This)->lpVtbl -> get_HighQuotaWaterMark(This,plHighQuotaWaterMark) ) 

#define IFaxConfiguration_put_HighQuotaWaterMark(This,lHighQuotaWaterMark)	\
    ( (This)->lpVtbl -> put_HighQuotaWaterMark(This,lHighQuotaWaterMark) ) 

#define IFaxConfiguration_get_LowQuotaWaterMark(This,plLowQuotaWaterMark)	\
    ( (This)->lpVtbl -> get_LowQuotaWaterMark(This,plLowQuotaWaterMark) ) 

#define IFaxConfiguration_put_LowQuotaWaterMark(This,lLowQuotaWaterMark)	\
    ( (This)->lpVtbl -> put_LowQuotaWaterMark(This,lLowQuotaWaterMark) ) 

#define IFaxConfiguration_get_ArchiveAgeLimit(This,plArchiveAgeLimit)	\
    ( (This)->lpVtbl -> get_ArchiveAgeLimit(This,plArchiveAgeLimit) ) 

#define IFaxConfiguration_put_ArchiveAgeLimit(This,lArchiveAgeLimit)	\
    ( (This)->lpVtbl -> put_ArchiveAgeLimit(This,lArchiveAgeLimit) ) 

#define IFaxConfiguration_get_ArchiveSizeLow(This,plSizeLow)	\
    ( (This)->lpVtbl -> get_ArchiveSizeLow(This,plSizeLow) ) 

#define IFaxConfiguration_get_ArchiveSizeHigh(This,plSizeHigh)	\
    ( (This)->lpVtbl -> get_ArchiveSizeHigh(This,plSizeHigh) ) 

#define IFaxConfiguration_get_OutgoingQueueBlocked(This,pbOutgoingBlocked)	\
    ( (This)->lpVtbl -> get_OutgoingQueueBlocked(This,pbOutgoingBlocked) ) 

#define IFaxConfiguration_put_OutgoingQueueBlocked(This,bOutgoingBlocked)	\
    ( (This)->lpVtbl -> put_OutgoingQueueBlocked(This,bOutgoingBlocked) ) 

#define IFaxConfiguration_get_OutgoingQueuePaused(This,pbOutgoingPaused)	\
    ( (This)->lpVtbl -> get_OutgoingQueuePaused(This,pbOutgoingPaused) ) 

#define IFaxConfiguration_put_OutgoingQueuePaused(This,bOutgoingPaused)	\
    ( (This)->lpVtbl -> put_OutgoingQueuePaused(This,bOutgoingPaused) ) 

#define IFaxConfiguration_get_AllowPersonalCoverPages(This,pbAllowPersonalCoverPages)	\
    ( (This)->lpVtbl -> get_AllowPersonalCoverPages(This,pbAllowPersonalCoverPages) ) 

#define IFaxConfiguration_put_AllowPersonalCoverPages(This,bAllowPersonalCoverPages)	\
    ( (This)->lpVtbl -> put_AllowPersonalCoverPages(This,bAllowPersonalCoverPages) ) 

#define IFaxConfiguration_get_UseDeviceTSID(This,pbUseDeviceTSID)	\
    ( (This)->lpVtbl -> get_UseDeviceTSID(This,pbUseDeviceTSID) ) 

#define IFaxConfiguration_put_UseDeviceTSID(This,bUseDeviceTSID)	\
    ( (This)->lpVtbl -> put_UseDeviceTSID(This,bUseDeviceTSID) ) 

#define IFaxConfiguration_get_Retries(This,plRetries)	\
    ( (This)->lpVtbl -> get_Retries(This,plRetries) ) 

#define IFaxConfiguration_put_Retries(This,lRetries)	\
    ( (This)->lpVtbl -> put_Retries(This,lRetries) ) 

#define IFaxConfiguration_get_RetryDelay(This,plRetryDelay)	\
    ( (This)->lpVtbl -> get_RetryDelay(This,plRetryDelay) ) 

#define IFaxConfiguration_put_RetryDelay(This,lRetryDelay)	\
    ( (This)->lpVtbl -> put_RetryDelay(This,lRetryDelay) ) 

#define IFaxConfiguration_get_DiscountRateStart(This,pdateDiscountRateStart)	\
    ( (This)->lpVtbl -> get_DiscountRateStart(This,pdateDiscountRateStart) ) 

#define IFaxConfiguration_put_DiscountRateStart(This,dateDiscountRateStart)	\
    ( (This)->lpVtbl -> put_DiscountRateStart(This,dateDiscountRateStart) ) 

#define IFaxConfiguration_get_DiscountRateEnd(This,pdateDiscountRateEnd)	\
    ( (This)->lpVtbl -> get_DiscountRateEnd(This,pdateDiscountRateEnd) ) 

#define IFaxConfiguration_put_DiscountRateEnd(This,dateDiscountRateEnd)	\
    ( (This)->lpVtbl -> put_DiscountRateEnd(This,dateDiscountRateEnd) ) 

#define IFaxConfiguration_get_OutgoingQueueAgeLimit(This,plOutgoingQueueAgeLimit)	\
    ( (This)->lpVtbl -> get_OutgoingQueueAgeLimit(This,plOutgoingQueueAgeLimit) ) 

#define IFaxConfiguration_put_OutgoingQueueAgeLimit(This,lOutgoingQueueAgeLimit)	\
    ( (This)->lpVtbl -> put_OutgoingQueueAgeLimit(This,lOutgoingQueueAgeLimit) ) 

#define IFaxConfiguration_get_Branding(This,pbBranding)	\
    ( (This)->lpVtbl -> get_Branding(This,pbBranding) ) 

#define IFaxConfiguration_put_Branding(This,bBranding)	\
    ( (This)->lpVtbl -> put_Branding(This,bBranding) ) 

#define IFaxConfiguration_get_IncomingQueueBlocked(This,pbIncomingBlocked)	\
    ( (This)->lpVtbl -> get_IncomingQueueBlocked(This,pbIncomingBlocked) ) 

#define IFaxConfiguration_put_IncomingQueueBlocked(This,bIncomingBlocked)	\
    ( (This)->lpVtbl -> put_IncomingQueueBlocked(This,bIncomingBlocked) ) 

#define IFaxConfiguration_get_AutoCreateAccountOnConnect(This,pbAutoCreateAccountOnConnect)	\
    ( (This)->lpVtbl -> get_AutoCreateAccountOnConnect(This,pbAutoCreateAccountOnConnect) ) 

#define IFaxConfiguration_put_AutoCreateAccountOnConnect(This,bAutoCreateAccountOnConnect)	\
    ( (This)->lpVtbl -> put_AutoCreateAccountOnConnect(This,bAutoCreateAccountOnConnect) ) 

#define IFaxConfiguration_get_IncomingFaxesArePublic(This,pbIncomingFaxesArePublic)	\
    ( (This)->lpVtbl -> get_IncomingFaxesArePublic(This,pbIncomingFaxesArePublic) ) 

#define IFaxConfiguration_put_IncomingFaxesArePublic(This,bIncomingFaxesArePublic)	\
    ( (This)->lpVtbl -> put_IncomingFaxesArePublic(This,bIncomingFaxesArePublic) ) 

#define IFaxConfiguration_Refresh(This)	\
    ( (This)->lpVtbl -> Refresh(This) ) 

#define IFaxConfiguration_Save(This)	\
    ( (This)->lpVtbl -> Save(This) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IFaxConfiguration_INTERFACE_DEFINED__ */


#ifndef __IFaxServer2_INTERFACE_DEFINED__
#define __IFaxServer2_INTERFACE_DEFINED__

/* interface IFaxServer2 */
/* [nonextensible][unique][helpstring][dual][uuid][object] */ 


EXTERN_C const IID IID_IFaxServer2;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("571ced0f-5609-4f40-9176-547e3a72ca7c")
    IFaxServer2 : public IFaxServer
    {
    public:
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Configuration( 
            /* [retval][out] */ __RPC__deref_out_opt IFaxConfiguration **ppFaxConfiguration) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_CurrentAccount( 
            /* [retval][out] */ __RPC__deref_out_opt IFaxAccount **ppCurrentAccount) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_FaxAccountSet( 
            /* [retval][out] */ __RPC__deref_out_opt IFaxAccountSet **ppFaxAccountSet) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Security2( 
            /* [retval][out] */ __RPC__deref_out_opt IFaxSecurity2 **ppFaxSecurity2) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IFaxServer2Vtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IFaxServer2 * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IFaxServer2 * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IFaxServer2 * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IFaxServer2 * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IFaxServer2 * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IFaxServer2 * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IFaxServer2 * This,
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
        
        DECLSPEC_XFGVIRT(IFaxServer, Connect)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *Connect )( 
            __RPC__in IFaxServer2 * This,
            /* [in] */ __RPC__in BSTR bstrServerName);
        
        DECLSPEC_XFGVIRT(IFaxServer, get_ServerName)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_ServerName )( 
            __RPC__in IFaxServer2 * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrServerName);
        
        DECLSPEC_XFGVIRT(IFaxServer, GetDeviceProviders)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *GetDeviceProviders )( 
            __RPC__in IFaxServer2 * This,
            /* [retval][out] */ __RPC__deref_out_opt IFaxDeviceProviders **ppFaxDeviceProviders);
        
        DECLSPEC_XFGVIRT(IFaxServer, GetDevices)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *GetDevices )( 
            __RPC__in IFaxServer2 * This,
            /* [retval][out] */ __RPC__deref_out_opt IFaxDevices **ppFaxDevices);
        
        DECLSPEC_XFGVIRT(IFaxServer, get_InboundRouting)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_InboundRouting )( 
            __RPC__in IFaxServer2 * This,
            /* [retval][out] */ __RPC__deref_out_opt IFaxInboundRouting **ppFaxInboundRouting);
        
        DECLSPEC_XFGVIRT(IFaxServer, get_Folders)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Folders )( 
            __RPC__in IFaxServer2 * This,
            /* [retval][out] */ __RPC__deref_out_opt IFaxFolders **pFaxFolders);
        
        DECLSPEC_XFGVIRT(IFaxServer, get_LoggingOptions)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_LoggingOptions )( 
            __RPC__in IFaxServer2 * This,
            /* [retval][out] */ __RPC__deref_out_opt IFaxLoggingOptions **ppFaxLoggingOptions);
        
        DECLSPEC_XFGVIRT(IFaxServer, get_MajorVersion)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_MajorVersion )( 
            __RPC__in IFaxServer2 * This,
            /* [retval][out] */ __RPC__out long *plMajorVersion);
        
        DECLSPEC_XFGVIRT(IFaxServer, get_MinorVersion)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_MinorVersion )( 
            __RPC__in IFaxServer2 * This,
            /* [retval][out] */ __RPC__out long *plMinorVersion);
        
        DECLSPEC_XFGVIRT(IFaxServer, get_MajorBuild)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_MajorBuild )( 
            __RPC__in IFaxServer2 * This,
            /* [retval][out] */ __RPC__out long *plMajorBuild);
        
        DECLSPEC_XFGVIRT(IFaxServer, get_MinorBuild)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_MinorBuild )( 
            __RPC__in IFaxServer2 * This,
            /* [retval][out] */ __RPC__out long *plMinorBuild);
        
        DECLSPEC_XFGVIRT(IFaxServer, get_Debug)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Debug )( 
            __RPC__in IFaxServer2 * This,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pbDebug);
        
        DECLSPEC_XFGVIRT(IFaxServer, get_Activity)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Activity )( 
            __RPC__in IFaxServer2 * This,
            /* [retval][out] */ __RPC__deref_out_opt IFaxActivity **ppFaxActivity);
        
        DECLSPEC_XFGVIRT(IFaxServer, get_OutboundRouting)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_OutboundRouting )( 
            __RPC__in IFaxServer2 * This,
            /* [retval][out] */ __RPC__deref_out_opt IFaxOutboundRouting **ppFaxOutboundRouting);
        
        DECLSPEC_XFGVIRT(IFaxServer, get_ReceiptOptions)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_ReceiptOptions )( 
            __RPC__in IFaxServer2 * This,
            /* [retval][out] */ __RPC__deref_out_opt IFaxReceiptOptions **ppFaxReceiptOptions);
        
        DECLSPEC_XFGVIRT(IFaxServer, get_Security)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Security )( 
            __RPC__in IFaxServer2 * This,
            /* [retval][out] */ __RPC__deref_out_opt IFaxSecurity **ppFaxSecurity);
        
        DECLSPEC_XFGVIRT(IFaxServer, Disconnect)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *Disconnect )( 
            __RPC__in IFaxServer2 * This);
        
        DECLSPEC_XFGVIRT(IFaxServer, GetExtensionProperty)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *GetExtensionProperty )( 
            __RPC__in IFaxServer2 * This,
            /* [in] */ __RPC__in BSTR bstrGUID,
            /* [retval][out] */ __RPC__out VARIANT *pvProperty);
        
        DECLSPEC_XFGVIRT(IFaxServer, SetExtensionProperty)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *SetExtensionProperty )( 
            __RPC__in IFaxServer2 * This,
            /* [in] */ __RPC__in BSTR bstrGUID,
            /* [in] */ VARIANT vProperty);
        
        DECLSPEC_XFGVIRT(IFaxServer, ListenToServerEvents)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *ListenToServerEvents )( 
            __RPC__in IFaxServer2 * This,
            /* [in] */ FAX_SERVER_EVENTS_TYPE_ENUM EventTypes);
        
        DECLSPEC_XFGVIRT(IFaxServer, RegisterDeviceProvider)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *RegisterDeviceProvider )( 
            __RPC__in IFaxServer2 * This,
            /* [in] */ __RPC__in BSTR bstrGUID,
            /* [in] */ __RPC__in BSTR bstrFriendlyName,
            /* [in] */ __RPC__in BSTR bstrImageName,
            /* [in] */ __RPC__in BSTR TspName,
            /* [in] */ long lFSPIVersion);
        
        DECLSPEC_XFGVIRT(IFaxServer, UnregisterDeviceProvider)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *UnregisterDeviceProvider )( 
            __RPC__in IFaxServer2 * This,
            /* [in] */ __RPC__in BSTR bstrUniqueName);
        
        DECLSPEC_XFGVIRT(IFaxServer, RegisterInboundRoutingExtension)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *RegisterInboundRoutingExtension )( 
            __RPC__in IFaxServer2 * This,
            /* [in] */ __RPC__in BSTR bstrExtensionName,
            /* [in] */ __RPC__in BSTR bstrFriendlyName,
            /* [in] */ __RPC__in BSTR bstrImageName,
            /* [in] */ VARIANT vMethods);
        
        DECLSPEC_XFGVIRT(IFaxServer, UnregisterInboundRoutingExtension)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *UnregisterInboundRoutingExtension )( 
            __RPC__in IFaxServer2 * This,
            /* [in] */ __RPC__in BSTR bstrExtensionUniqueName);
        
        DECLSPEC_XFGVIRT(IFaxServer, get_RegisteredEvents)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_RegisteredEvents )( 
            __RPC__in IFaxServer2 * This,
            /* [retval][out] */ __RPC__out FAX_SERVER_EVENTS_TYPE_ENUM *pEventTypes);
        
        DECLSPEC_XFGVIRT(IFaxServer, get_APIVersion)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_APIVersion )( 
            __RPC__in IFaxServer2 * This,
            /* [retval][out] */ __RPC__out FAX_SERVER_APIVERSION_ENUM *pAPIVersion);
        
        DECLSPEC_XFGVIRT(IFaxServer2, get_Configuration)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Configuration )( 
            __RPC__in IFaxServer2 * This,
            /* [retval][out] */ __RPC__deref_out_opt IFaxConfiguration **ppFaxConfiguration);
        
        DECLSPEC_XFGVIRT(IFaxServer2, get_CurrentAccount)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_CurrentAccount )( 
            __RPC__in IFaxServer2 * This,
            /* [retval][out] */ __RPC__deref_out_opt IFaxAccount **ppCurrentAccount);
        
        DECLSPEC_XFGVIRT(IFaxServer2, get_FaxAccountSet)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_FaxAccountSet )( 
            __RPC__in IFaxServer2 * This,
            /* [retval][out] */ __RPC__deref_out_opt IFaxAccountSet **ppFaxAccountSet);
        
        DECLSPEC_XFGVIRT(IFaxServer2, get_Security2)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Security2 )( 
            __RPC__in IFaxServer2 * This,
            /* [retval][out] */ __RPC__deref_out_opt IFaxSecurity2 **ppFaxSecurity2);
        
        END_INTERFACE
    } IFaxServer2Vtbl;

    interface IFaxServer2
    {
        CONST_VTBL struct IFaxServer2Vtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IFaxServer2_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IFaxServer2_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IFaxServer2_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IFaxServer2_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IFaxServer2_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IFaxServer2_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IFaxServer2_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IFaxServer2_Connect(This,bstrServerName)	\
    ( (This)->lpVtbl -> Connect(This,bstrServerName) ) 

#define IFaxServer2_get_ServerName(This,pbstrServerName)	\
    ( (This)->lpVtbl -> get_ServerName(This,pbstrServerName) ) 

#define IFaxServer2_GetDeviceProviders(This,ppFaxDeviceProviders)	\
    ( (This)->lpVtbl -> GetDeviceProviders(This,ppFaxDeviceProviders) ) 

#define IFaxServer2_GetDevices(This,ppFaxDevices)	\
    ( (This)->lpVtbl -> GetDevices(This,ppFaxDevices) ) 

#define IFaxServer2_get_InboundRouting(This,ppFaxInboundRouting)	\
    ( (This)->lpVtbl -> get_InboundRouting(This,ppFaxInboundRouting) ) 

#define IFaxServer2_get_Folders(This,pFaxFolders)	\
    ( (This)->lpVtbl -> get_Folders(This,pFaxFolders) ) 

#define IFaxServer2_get_LoggingOptions(This,ppFaxLoggingOptions)	\
    ( (This)->lpVtbl -> get_LoggingOptions(This,ppFaxLoggingOptions) ) 

#define IFaxServer2_get_MajorVersion(This,plMajorVersion)	\
    ( (This)->lpVtbl -> get_MajorVersion(This,plMajorVersion) ) 

#define IFaxServer2_get_MinorVersion(This,plMinorVersion)	\
    ( (This)->lpVtbl -> get_MinorVersion(This,plMinorVersion) ) 

#define IFaxServer2_get_MajorBuild(This,plMajorBuild)	\
    ( (This)->lpVtbl -> get_MajorBuild(This,plMajorBuild) ) 

#define IFaxServer2_get_MinorBuild(This,plMinorBuild)	\
    ( (This)->lpVtbl -> get_MinorBuild(This,plMinorBuild) ) 

#define IFaxServer2_get_Debug(This,pbDebug)	\
    ( (This)->lpVtbl -> get_Debug(This,pbDebug) ) 

#define IFaxServer2_get_Activity(This,ppFaxActivity)	\
    ( (This)->lpVtbl -> get_Activity(This,ppFaxActivity) ) 

#define IFaxServer2_get_OutboundRouting(This,ppFaxOutboundRouting)	\
    ( (This)->lpVtbl -> get_OutboundRouting(This,ppFaxOutboundRouting) ) 

#define IFaxServer2_get_ReceiptOptions(This,ppFaxReceiptOptions)	\
    ( (This)->lpVtbl -> get_ReceiptOptions(This,ppFaxReceiptOptions) ) 

#define IFaxServer2_get_Security(This,ppFaxSecurity)	\
    ( (This)->lpVtbl -> get_Security(This,ppFaxSecurity) ) 

#define IFaxServer2_Disconnect(This)	\
    ( (This)->lpVtbl -> Disconnect(This) ) 

#define IFaxServer2_GetExtensionProperty(This,bstrGUID,pvProperty)	\
    ( (This)->lpVtbl -> GetExtensionProperty(This,bstrGUID,pvProperty) ) 

#define IFaxServer2_SetExtensionProperty(This,bstrGUID,vProperty)	\
    ( (This)->lpVtbl -> SetExtensionProperty(This,bstrGUID,vProperty) ) 

#define IFaxServer2_ListenToServerEvents(This,EventTypes)	\
    ( (This)->lpVtbl -> ListenToServerEvents(This,EventTypes) ) 

#define IFaxServer2_RegisterDeviceProvider(This,bstrGUID,bstrFriendlyName,bstrImageName,TspName,lFSPIVersion)	\
    ( (This)->lpVtbl -> RegisterDeviceProvider(This,bstrGUID,bstrFriendlyName,bstrImageName,TspName,lFSPIVersion) ) 

#define IFaxServer2_UnregisterDeviceProvider(This,bstrUniqueName)	\
    ( (This)->lpVtbl -> UnregisterDeviceProvider(This,bstrUniqueName) ) 

#define IFaxServer2_RegisterInboundRoutingExtension(This,bstrExtensionName,bstrFriendlyName,bstrImageName,vMethods)	\
    ( (This)->lpVtbl -> RegisterInboundRoutingExtension(This,bstrExtensionName,bstrFriendlyName,bstrImageName,vMethods) ) 

#define IFaxServer2_UnregisterInboundRoutingExtension(This,bstrExtensionUniqueName)	\
    ( (This)->lpVtbl -> UnregisterInboundRoutingExtension(This,bstrExtensionUniqueName) ) 

#define IFaxServer2_get_RegisteredEvents(This,pEventTypes)	\
    ( (This)->lpVtbl -> get_RegisteredEvents(This,pEventTypes) ) 

#define IFaxServer2_get_APIVersion(This,pAPIVersion)	\
    ( (This)->lpVtbl -> get_APIVersion(This,pAPIVersion) ) 


#define IFaxServer2_get_Configuration(This,ppFaxConfiguration)	\
    ( (This)->lpVtbl -> get_Configuration(This,ppFaxConfiguration) ) 

#define IFaxServer2_get_CurrentAccount(This,ppCurrentAccount)	\
    ( (This)->lpVtbl -> get_CurrentAccount(This,ppCurrentAccount) ) 

#define IFaxServer2_get_FaxAccountSet(This,ppFaxAccountSet)	\
    ( (This)->lpVtbl -> get_FaxAccountSet(This,ppFaxAccountSet) ) 

#define IFaxServer2_get_Security2(This,ppFaxSecurity2)	\
    ( (This)->lpVtbl -> get_Security2(This,ppFaxSecurity2) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IFaxServer2_INTERFACE_DEFINED__ */


#ifndef __IFaxAccountSet_INTERFACE_DEFINED__
#define __IFaxAccountSet_INTERFACE_DEFINED__

/* interface IFaxAccountSet */
/* [nonextensible][unique][helpstring][dual][uuid][object] */ 


EXTERN_C const IID IID_IFaxAccountSet;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("7428fbae-841e-47b8-86f4-2288946dca1b")
    IFaxAccountSet : public IDispatch
    {
    public:
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE GetAccounts( 
            /* [retval][out] */ __RPC__deref_out_opt IFaxAccounts **ppFaxAccounts) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE GetAccount( 
            /* [in] */ __RPC__in BSTR bstrAccountName,
            /* [retval][out] */ __RPC__deref_out_opt IFaxAccount **pFaxAccount) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE AddAccount( 
            /* [in] */ __RPC__in BSTR bstrAccountName,
            /* [retval][out] */ __RPC__deref_out_opt IFaxAccount **pFaxAccount) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE RemoveAccount( 
            /* [in] */ __RPC__in BSTR bstrAccountName) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IFaxAccountSetVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IFaxAccountSet * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IFaxAccountSet * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IFaxAccountSet * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IFaxAccountSet * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IFaxAccountSet * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IFaxAccountSet * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IFaxAccountSet * This,
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
        
        DECLSPEC_XFGVIRT(IFaxAccountSet, GetAccounts)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *GetAccounts )( 
            __RPC__in IFaxAccountSet * This,
            /* [retval][out] */ __RPC__deref_out_opt IFaxAccounts **ppFaxAccounts);
        
        DECLSPEC_XFGVIRT(IFaxAccountSet, GetAccount)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *GetAccount )( 
            __RPC__in IFaxAccountSet * This,
            /* [in] */ __RPC__in BSTR bstrAccountName,
            /* [retval][out] */ __RPC__deref_out_opt IFaxAccount **pFaxAccount);
        
        DECLSPEC_XFGVIRT(IFaxAccountSet, AddAccount)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *AddAccount )( 
            __RPC__in IFaxAccountSet * This,
            /* [in] */ __RPC__in BSTR bstrAccountName,
            /* [retval][out] */ __RPC__deref_out_opt IFaxAccount **pFaxAccount);
        
        DECLSPEC_XFGVIRT(IFaxAccountSet, RemoveAccount)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *RemoveAccount )( 
            __RPC__in IFaxAccountSet * This,
            /* [in] */ __RPC__in BSTR bstrAccountName);
        
        END_INTERFACE
    } IFaxAccountSetVtbl;

    interface IFaxAccountSet
    {
        CONST_VTBL struct IFaxAccountSetVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IFaxAccountSet_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IFaxAccountSet_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IFaxAccountSet_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IFaxAccountSet_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IFaxAccountSet_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IFaxAccountSet_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IFaxAccountSet_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IFaxAccountSet_GetAccounts(This,ppFaxAccounts)	\
    ( (This)->lpVtbl -> GetAccounts(This,ppFaxAccounts) ) 

#define IFaxAccountSet_GetAccount(This,bstrAccountName,pFaxAccount)	\
    ( (This)->lpVtbl -> GetAccount(This,bstrAccountName,pFaxAccount) ) 

#define IFaxAccountSet_AddAccount(This,bstrAccountName,pFaxAccount)	\
    ( (This)->lpVtbl -> AddAccount(This,bstrAccountName,pFaxAccount) ) 

#define IFaxAccountSet_RemoveAccount(This,bstrAccountName)	\
    ( (This)->lpVtbl -> RemoveAccount(This,bstrAccountName) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IFaxAccountSet_INTERFACE_DEFINED__ */


#ifndef __IFaxAccounts_INTERFACE_DEFINED__
#define __IFaxAccounts_INTERFACE_DEFINED__

/* interface IFaxAccounts */
/* [unique][helpstring][dual][uuid][object] */ 


EXTERN_C const IID IID_IFaxAccounts;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("93ea8162-8be7-42d1-ae7b-ec74e2d989da")
    IFaxAccounts : public IDispatch
    {
    public:
        virtual /* [propget][id] */ HRESULT STDMETHODCALLTYPE get__NewEnum( 
            /* [retval][out] */ __RPC__deref_out_opt IUnknown **ppUnk) = 0;
        
        virtual /* [propget][id] */ HRESULT STDMETHODCALLTYPE get_Item( 
            /* [in] */ VARIANT vIndex,
            /* [retval][out] */ __RPC__deref_out_opt IFaxAccount **pFaxAccount) = 0;
        
        virtual /* [propget][id] */ HRESULT STDMETHODCALLTYPE get_Count( 
            /* [retval][out] */ __RPC__out long *plCount) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IFaxAccountsVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IFaxAccounts * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IFaxAccounts * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IFaxAccounts * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IFaxAccounts * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IFaxAccounts * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IFaxAccounts * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IFaxAccounts * This,
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
        
        DECLSPEC_XFGVIRT(IFaxAccounts, get__NewEnum)
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *get__NewEnum )( 
            __RPC__in IFaxAccounts * This,
            /* [retval][out] */ __RPC__deref_out_opt IUnknown **ppUnk);
        
        DECLSPEC_XFGVIRT(IFaxAccounts, get_Item)
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_Item )( 
            __RPC__in IFaxAccounts * This,
            /* [in] */ VARIANT vIndex,
            /* [retval][out] */ __RPC__deref_out_opt IFaxAccount **pFaxAccount);
        
        DECLSPEC_XFGVIRT(IFaxAccounts, get_Count)
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_Count )( 
            __RPC__in IFaxAccounts * This,
            /* [retval][out] */ __RPC__out long *plCount);
        
        END_INTERFACE
    } IFaxAccountsVtbl;

    interface IFaxAccounts
    {
        CONST_VTBL struct IFaxAccountsVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IFaxAccounts_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IFaxAccounts_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IFaxAccounts_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IFaxAccounts_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IFaxAccounts_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IFaxAccounts_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IFaxAccounts_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IFaxAccounts_get__NewEnum(This,ppUnk)	\
    ( (This)->lpVtbl -> get__NewEnum(This,ppUnk) ) 

#define IFaxAccounts_get_Item(This,vIndex,pFaxAccount)	\
    ( (This)->lpVtbl -> get_Item(This,vIndex,pFaxAccount) ) 

#define IFaxAccounts_get_Count(This,plCount)	\
    ( (This)->lpVtbl -> get_Count(This,plCount) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IFaxAccounts_INTERFACE_DEFINED__ */


#ifndef __IFaxAccount_INTERFACE_DEFINED__
#define __IFaxAccount_INTERFACE_DEFINED__

/* interface IFaxAccount */
/* [unique][helpstring][dual][uuid][object] */ 

typedef 
enum FAX_ACCOUNT_EVENTS_TYPE_ENUM
    {
        faetNONE	= 0,
        faetIN_QUEUE	= 0x1,
        faetOUT_QUEUE	= 0x2,
        faetIN_ARCHIVE	= 0x4,
        faetOUT_ARCHIVE	= 0x8,
        faetFXSSVC_ENDED	= 0x10
    } 	FAX_ACCOUNT_EVENTS_TYPE_ENUM;


EXTERN_C const IID IID_IFaxAccount;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("68535b33-5dc4-4086-be26-b76f9b711006")
    IFaxAccount : public IDispatch
    {
    public:
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_AccountName( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrAccountName) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Folders( 
            /* [retval][out] */ __RPC__deref_out_opt IFaxAccountFolders **ppFolders) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE ListenToAccountEvents( 
            /* [in] */ FAX_ACCOUNT_EVENTS_TYPE_ENUM EventTypes) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_RegisteredEvents( 
            /* [retval][out] */ __RPC__out FAX_ACCOUNT_EVENTS_TYPE_ENUM *pRegisteredEvents) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IFaxAccountVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IFaxAccount * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IFaxAccount * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IFaxAccount * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IFaxAccount * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IFaxAccount * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IFaxAccount * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IFaxAccount * This,
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
        
        DECLSPEC_XFGVIRT(IFaxAccount, get_AccountName)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_AccountName )( 
            __RPC__in IFaxAccount * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrAccountName);
        
        DECLSPEC_XFGVIRT(IFaxAccount, get_Folders)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Folders )( 
            __RPC__in IFaxAccount * This,
            /* [retval][out] */ __RPC__deref_out_opt IFaxAccountFolders **ppFolders);
        
        DECLSPEC_XFGVIRT(IFaxAccount, ListenToAccountEvents)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *ListenToAccountEvents )( 
            __RPC__in IFaxAccount * This,
            /* [in] */ FAX_ACCOUNT_EVENTS_TYPE_ENUM EventTypes);
        
        DECLSPEC_XFGVIRT(IFaxAccount, get_RegisteredEvents)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_RegisteredEvents )( 
            __RPC__in IFaxAccount * This,
            /* [retval][out] */ __RPC__out FAX_ACCOUNT_EVENTS_TYPE_ENUM *pRegisteredEvents);
        
        END_INTERFACE
    } IFaxAccountVtbl;

    interface IFaxAccount
    {
        CONST_VTBL struct IFaxAccountVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IFaxAccount_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IFaxAccount_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IFaxAccount_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IFaxAccount_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IFaxAccount_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IFaxAccount_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IFaxAccount_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IFaxAccount_get_AccountName(This,pbstrAccountName)	\
    ( (This)->lpVtbl -> get_AccountName(This,pbstrAccountName) ) 

#define IFaxAccount_get_Folders(This,ppFolders)	\
    ( (This)->lpVtbl -> get_Folders(This,ppFolders) ) 

#define IFaxAccount_ListenToAccountEvents(This,EventTypes)	\
    ( (This)->lpVtbl -> ListenToAccountEvents(This,EventTypes) ) 

#define IFaxAccount_get_RegisteredEvents(This,pRegisteredEvents)	\
    ( (This)->lpVtbl -> get_RegisteredEvents(This,pRegisteredEvents) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IFaxAccount_INTERFACE_DEFINED__ */


#ifndef __IFaxOutgoingJob2_INTERFACE_DEFINED__
#define __IFaxOutgoingJob2_INTERFACE_DEFINED__

/* interface IFaxOutgoingJob2 */
/* [unique][helpstring][dual][uuid][object] */ 


EXTERN_C const IID IID_IFaxOutgoingJob2;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("418a8d96-59a0-4789-b176-edf3dc8fa8f7")
    IFaxOutgoingJob2 : public IFaxOutgoingJob
    {
    public:
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_HasCoverPage( 
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pbHasCoverPage) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_ReceiptAddress( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrReceiptAddress) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_ScheduleType( 
            /* [retval][out] */ __RPC__out FAX_SCHEDULE_TYPE_ENUM *pScheduleType) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IFaxOutgoingJob2Vtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IFaxOutgoingJob2 * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IFaxOutgoingJob2 * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IFaxOutgoingJob2 * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IFaxOutgoingJob2 * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IFaxOutgoingJob2 * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IFaxOutgoingJob2 * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IFaxOutgoingJob2 * This,
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
        
        DECLSPEC_XFGVIRT(IFaxOutgoingJob, get_Subject)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Subject )( 
            __RPC__in IFaxOutgoingJob2 * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrSubject);
        
        DECLSPEC_XFGVIRT(IFaxOutgoingJob, get_DocumentName)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_DocumentName )( 
            __RPC__in IFaxOutgoingJob2 * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrDocumentName);
        
        DECLSPEC_XFGVIRT(IFaxOutgoingJob, get_Pages)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Pages )( 
            __RPC__in IFaxOutgoingJob2 * This,
            /* [retval][out] */ __RPC__out long *plPages);
        
        DECLSPEC_XFGVIRT(IFaxOutgoingJob, get_Size)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Size )( 
            __RPC__in IFaxOutgoingJob2 * This,
            /* [retval][out] */ __RPC__out long *plSize);
        
        DECLSPEC_XFGVIRT(IFaxOutgoingJob, get_SubmissionId)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_SubmissionId )( 
            __RPC__in IFaxOutgoingJob2 * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrSubmissionId);
        
        DECLSPEC_XFGVIRT(IFaxOutgoingJob, get_Id)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Id )( 
            __RPC__in IFaxOutgoingJob2 * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrId);
        
        DECLSPEC_XFGVIRT(IFaxOutgoingJob, get_OriginalScheduledTime)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_OriginalScheduledTime )( 
            __RPC__in IFaxOutgoingJob2 * This,
            /* [retval][out] */ __RPC__out DATE *pdateOriginalScheduledTime);
        
        DECLSPEC_XFGVIRT(IFaxOutgoingJob, get_SubmissionTime)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_SubmissionTime )( 
            __RPC__in IFaxOutgoingJob2 * This,
            /* [retval][out] */ __RPC__out DATE *pdateSubmissionTime);
        
        DECLSPEC_XFGVIRT(IFaxOutgoingJob, get_ReceiptType)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_ReceiptType )( 
            __RPC__in IFaxOutgoingJob2 * This,
            /* [retval][out] */ __RPC__out FAX_RECEIPT_TYPE_ENUM *pReceiptType);
        
        DECLSPEC_XFGVIRT(IFaxOutgoingJob, get_Priority)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Priority )( 
            __RPC__in IFaxOutgoingJob2 * This,
            /* [retval][out] */ __RPC__out FAX_PRIORITY_TYPE_ENUM *pPriority);
        
        DECLSPEC_XFGVIRT(IFaxOutgoingJob, get_Sender)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Sender )( 
            __RPC__in IFaxOutgoingJob2 * This,
            /* [retval][out] */ __RPC__deref_out_opt IFaxSender **ppFaxSender);
        
        DECLSPEC_XFGVIRT(IFaxOutgoingJob, get_Recipient)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Recipient )( 
            __RPC__in IFaxOutgoingJob2 * This,
            /* [retval][out] */ __RPC__deref_out_opt IFaxRecipient **ppFaxRecipient);
        
        DECLSPEC_XFGVIRT(IFaxOutgoingJob, get_CurrentPage)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_CurrentPage )( 
            __RPC__in IFaxOutgoingJob2 * This,
            /* [retval][out] */ __RPC__out long *plCurrentPage);
        
        DECLSPEC_XFGVIRT(IFaxOutgoingJob, get_DeviceId)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_DeviceId )( 
            __RPC__in IFaxOutgoingJob2 * This,
            /* [retval][out] */ __RPC__out long *plDeviceId);
        
        DECLSPEC_XFGVIRT(IFaxOutgoingJob, get_Status)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Status )( 
            __RPC__in IFaxOutgoingJob2 * This,
            /* [retval][out] */ __RPC__out FAX_JOB_STATUS_ENUM *pStatus);
        
        DECLSPEC_XFGVIRT(IFaxOutgoingJob, get_ExtendedStatusCode)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_ExtendedStatusCode )( 
            __RPC__in IFaxOutgoingJob2 * This,
            /* [retval][out] */ __RPC__out FAX_JOB_EXTENDED_STATUS_ENUM *pExtendedStatusCode);
        
        DECLSPEC_XFGVIRT(IFaxOutgoingJob, get_ExtendedStatus)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_ExtendedStatus )( 
            __RPC__in IFaxOutgoingJob2 * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrExtendedStatus);
        
        DECLSPEC_XFGVIRT(IFaxOutgoingJob, get_AvailableOperations)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_AvailableOperations )( 
            __RPC__in IFaxOutgoingJob2 * This,
            /* [retval][out] */ __RPC__out FAX_JOB_OPERATIONS_ENUM *pAvailableOperations);
        
        DECLSPEC_XFGVIRT(IFaxOutgoingJob, get_Retries)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Retries )( 
            __RPC__in IFaxOutgoingJob2 * This,
            /* [retval][out] */ __RPC__out long *plRetries);
        
        DECLSPEC_XFGVIRT(IFaxOutgoingJob, get_ScheduledTime)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_ScheduledTime )( 
            __RPC__in IFaxOutgoingJob2 * This,
            /* [retval][out] */ __RPC__out DATE *pdateScheduledTime);
        
        DECLSPEC_XFGVIRT(IFaxOutgoingJob, get_TransmissionStart)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_TransmissionStart )( 
            __RPC__in IFaxOutgoingJob2 * This,
            /* [retval][out] */ __RPC__out DATE *pdateTransmissionStart);
        
        DECLSPEC_XFGVIRT(IFaxOutgoingJob, get_TransmissionEnd)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_TransmissionEnd )( 
            __RPC__in IFaxOutgoingJob2 * This,
            /* [retval][out] */ __RPC__out DATE *pdateTransmissionEnd);
        
        DECLSPEC_XFGVIRT(IFaxOutgoingJob, get_CSID)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_CSID )( 
            __RPC__in IFaxOutgoingJob2 * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrCSID);
        
        DECLSPEC_XFGVIRT(IFaxOutgoingJob, get_TSID)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_TSID )( 
            __RPC__in IFaxOutgoingJob2 * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrTSID);
        
        DECLSPEC_XFGVIRT(IFaxOutgoingJob, get_GroupBroadcastReceipts)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_GroupBroadcastReceipts )( 
            __RPC__in IFaxOutgoingJob2 * This,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pbGroupBroadcastReceipts);
        
        DECLSPEC_XFGVIRT(IFaxOutgoingJob, Pause)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *Pause )( 
            __RPC__in IFaxOutgoingJob2 * This);
        
        DECLSPEC_XFGVIRT(IFaxOutgoingJob, Resume)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *Resume )( 
            __RPC__in IFaxOutgoingJob2 * This);
        
        DECLSPEC_XFGVIRT(IFaxOutgoingJob, Restart)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *Restart )( 
            __RPC__in IFaxOutgoingJob2 * This);
        
        DECLSPEC_XFGVIRT(IFaxOutgoingJob, CopyTiff)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *CopyTiff )( 
            __RPC__in IFaxOutgoingJob2 * This,
            /* [in] */ __RPC__in BSTR bstrTiffPath);
        
        DECLSPEC_XFGVIRT(IFaxOutgoingJob, Refresh)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *Refresh )( 
            __RPC__in IFaxOutgoingJob2 * This);
        
        DECLSPEC_XFGVIRT(IFaxOutgoingJob, Cancel)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *Cancel )( 
            __RPC__in IFaxOutgoingJob2 * This);
        
        DECLSPEC_XFGVIRT(IFaxOutgoingJob2, get_HasCoverPage)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_HasCoverPage )( 
            __RPC__in IFaxOutgoingJob2 * This,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pbHasCoverPage);
        
        DECLSPEC_XFGVIRT(IFaxOutgoingJob2, get_ReceiptAddress)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_ReceiptAddress )( 
            __RPC__in IFaxOutgoingJob2 * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrReceiptAddress);
        
        DECLSPEC_XFGVIRT(IFaxOutgoingJob2, get_ScheduleType)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_ScheduleType )( 
            __RPC__in IFaxOutgoingJob2 * This,
            /* [retval][out] */ __RPC__out FAX_SCHEDULE_TYPE_ENUM *pScheduleType);
        
        END_INTERFACE
    } IFaxOutgoingJob2Vtbl;

    interface IFaxOutgoingJob2
    {
        CONST_VTBL struct IFaxOutgoingJob2Vtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IFaxOutgoingJob2_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IFaxOutgoingJob2_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IFaxOutgoingJob2_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IFaxOutgoingJob2_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IFaxOutgoingJob2_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IFaxOutgoingJob2_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IFaxOutgoingJob2_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IFaxOutgoingJob2_get_Subject(This,pbstrSubject)	\
    ( (This)->lpVtbl -> get_Subject(This,pbstrSubject) ) 

#define IFaxOutgoingJob2_get_DocumentName(This,pbstrDocumentName)	\
    ( (This)->lpVtbl -> get_DocumentName(This,pbstrDocumentName) ) 

#define IFaxOutgoingJob2_get_Pages(This,plPages)	\
    ( (This)->lpVtbl -> get_Pages(This,plPages) ) 

#define IFaxOutgoingJob2_get_Size(This,plSize)	\
    ( (This)->lpVtbl -> get_Size(This,plSize) ) 

#define IFaxOutgoingJob2_get_SubmissionId(This,pbstrSubmissionId)	\
    ( (This)->lpVtbl -> get_SubmissionId(This,pbstrSubmissionId) ) 

#define IFaxOutgoingJob2_get_Id(This,pbstrId)	\
    ( (This)->lpVtbl -> get_Id(This,pbstrId) ) 

#define IFaxOutgoingJob2_get_OriginalScheduledTime(This,pdateOriginalScheduledTime)	\
    ( (This)->lpVtbl -> get_OriginalScheduledTime(This,pdateOriginalScheduledTime) ) 

#define IFaxOutgoingJob2_get_SubmissionTime(This,pdateSubmissionTime)	\
    ( (This)->lpVtbl -> get_SubmissionTime(This,pdateSubmissionTime) ) 

#define IFaxOutgoingJob2_get_ReceiptType(This,pReceiptType)	\
    ( (This)->lpVtbl -> get_ReceiptType(This,pReceiptType) ) 

#define IFaxOutgoingJob2_get_Priority(This,pPriority)	\
    ( (This)->lpVtbl -> get_Priority(This,pPriority) ) 

#define IFaxOutgoingJob2_get_Sender(This,ppFaxSender)	\
    ( (This)->lpVtbl -> get_Sender(This,ppFaxSender) ) 

#define IFaxOutgoingJob2_get_Recipient(This,ppFaxRecipient)	\
    ( (This)->lpVtbl -> get_Recipient(This,ppFaxRecipient) ) 

#define IFaxOutgoingJob2_get_CurrentPage(This,plCurrentPage)	\
    ( (This)->lpVtbl -> get_CurrentPage(This,plCurrentPage) ) 

#define IFaxOutgoingJob2_get_DeviceId(This,plDeviceId)	\
    ( (This)->lpVtbl -> get_DeviceId(This,plDeviceId) ) 

#define IFaxOutgoingJob2_get_Status(This,pStatus)	\
    ( (This)->lpVtbl -> get_Status(This,pStatus) ) 

#define IFaxOutgoingJob2_get_ExtendedStatusCode(This,pExtendedStatusCode)	\
    ( (This)->lpVtbl -> get_ExtendedStatusCode(This,pExtendedStatusCode) ) 

#define IFaxOutgoingJob2_get_ExtendedStatus(This,pbstrExtendedStatus)	\
    ( (This)->lpVtbl -> get_ExtendedStatus(This,pbstrExtendedStatus) ) 

#define IFaxOutgoingJob2_get_AvailableOperations(This,pAvailableOperations)	\
    ( (This)->lpVtbl -> get_AvailableOperations(This,pAvailableOperations) ) 

#define IFaxOutgoingJob2_get_Retries(This,plRetries)	\
    ( (This)->lpVtbl -> get_Retries(This,plRetries) ) 

#define IFaxOutgoingJob2_get_ScheduledTime(This,pdateScheduledTime)	\
    ( (This)->lpVtbl -> get_ScheduledTime(This,pdateScheduledTime) ) 

#define IFaxOutgoingJob2_get_TransmissionStart(This,pdateTransmissionStart)	\
    ( (This)->lpVtbl -> get_TransmissionStart(This,pdateTransmissionStart) ) 

#define IFaxOutgoingJob2_get_TransmissionEnd(This,pdateTransmissionEnd)	\
    ( (This)->lpVtbl -> get_TransmissionEnd(This,pdateTransmissionEnd) ) 

#define IFaxOutgoingJob2_get_CSID(This,pbstrCSID)	\
    ( (This)->lpVtbl -> get_CSID(This,pbstrCSID) ) 

#define IFaxOutgoingJob2_get_TSID(This,pbstrTSID)	\
    ( (This)->lpVtbl -> get_TSID(This,pbstrTSID) ) 

#define IFaxOutgoingJob2_get_GroupBroadcastReceipts(This,pbGroupBroadcastReceipts)	\
    ( (This)->lpVtbl -> get_GroupBroadcastReceipts(This,pbGroupBroadcastReceipts) ) 

#define IFaxOutgoingJob2_Pause(This)	\
    ( (This)->lpVtbl -> Pause(This) ) 

#define IFaxOutgoingJob2_Resume(This)	\
    ( (This)->lpVtbl -> Resume(This) ) 

#define IFaxOutgoingJob2_Restart(This)	\
    ( (This)->lpVtbl -> Restart(This) ) 

#define IFaxOutgoingJob2_CopyTiff(This,bstrTiffPath)	\
    ( (This)->lpVtbl -> CopyTiff(This,bstrTiffPath) ) 

#define IFaxOutgoingJob2_Refresh(This)	\
    ( (This)->lpVtbl -> Refresh(This) ) 

#define IFaxOutgoingJob2_Cancel(This)	\
    ( (This)->lpVtbl -> Cancel(This) ) 


#define IFaxOutgoingJob2_get_HasCoverPage(This,pbHasCoverPage)	\
    ( (This)->lpVtbl -> get_HasCoverPage(This,pbHasCoverPage) ) 

#define IFaxOutgoingJob2_get_ReceiptAddress(This,pbstrReceiptAddress)	\
    ( (This)->lpVtbl -> get_ReceiptAddress(This,pbstrReceiptAddress) ) 

#define IFaxOutgoingJob2_get_ScheduleType(This,pScheduleType)	\
    ( (This)->lpVtbl -> get_ScheduleType(This,pScheduleType) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IFaxOutgoingJob2_INTERFACE_DEFINED__ */


#ifndef __IFaxAccountFolders_INTERFACE_DEFINED__
#define __IFaxAccountFolders_INTERFACE_DEFINED__

/* interface IFaxAccountFolders */
/* [nonextensible][unique][helpstring][dual][uuid][object] */ 


EXTERN_C const IID IID_IFaxAccountFolders;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("6463f89d-23d8-46a9-8f86-c47b77ca7926")
    IFaxAccountFolders : public IDispatch
    {
    public:
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_OutgoingQueue( 
            /* [retval][out] */ __RPC__deref_out_opt IFaxAccountOutgoingQueue **pFaxOutgoingQueue) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_IncomingQueue( 
            /* [retval][out] */ __RPC__deref_out_opt IFaxAccountIncomingQueue **pFaxIncomingQueue) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_IncomingArchive( 
            /* [retval][out] */ __RPC__deref_out_opt IFaxAccountIncomingArchive **pFaxIncomingArchive) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_OutgoingArchive( 
            /* [retval][out] */ __RPC__deref_out_opt IFaxAccountOutgoingArchive **pFaxOutgoingArchive) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IFaxAccountFoldersVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IFaxAccountFolders * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IFaxAccountFolders * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IFaxAccountFolders * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IFaxAccountFolders * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IFaxAccountFolders * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IFaxAccountFolders * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IFaxAccountFolders * This,
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
        
        DECLSPEC_XFGVIRT(IFaxAccountFolders, get_OutgoingQueue)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_OutgoingQueue )( 
            __RPC__in IFaxAccountFolders * This,
            /* [retval][out] */ __RPC__deref_out_opt IFaxAccountOutgoingQueue **pFaxOutgoingQueue);
        
        DECLSPEC_XFGVIRT(IFaxAccountFolders, get_IncomingQueue)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_IncomingQueue )( 
            __RPC__in IFaxAccountFolders * This,
            /* [retval][out] */ __RPC__deref_out_opt IFaxAccountIncomingQueue **pFaxIncomingQueue);
        
        DECLSPEC_XFGVIRT(IFaxAccountFolders, get_IncomingArchive)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_IncomingArchive )( 
            __RPC__in IFaxAccountFolders * This,
            /* [retval][out] */ __RPC__deref_out_opt IFaxAccountIncomingArchive **pFaxIncomingArchive);
        
        DECLSPEC_XFGVIRT(IFaxAccountFolders, get_OutgoingArchive)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_OutgoingArchive )( 
            __RPC__in IFaxAccountFolders * This,
            /* [retval][out] */ __RPC__deref_out_opt IFaxAccountOutgoingArchive **pFaxOutgoingArchive);
        
        END_INTERFACE
    } IFaxAccountFoldersVtbl;

    interface IFaxAccountFolders
    {
        CONST_VTBL struct IFaxAccountFoldersVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IFaxAccountFolders_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IFaxAccountFolders_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IFaxAccountFolders_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IFaxAccountFolders_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IFaxAccountFolders_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IFaxAccountFolders_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IFaxAccountFolders_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IFaxAccountFolders_get_OutgoingQueue(This,pFaxOutgoingQueue)	\
    ( (This)->lpVtbl -> get_OutgoingQueue(This,pFaxOutgoingQueue) ) 

#define IFaxAccountFolders_get_IncomingQueue(This,pFaxIncomingQueue)	\
    ( (This)->lpVtbl -> get_IncomingQueue(This,pFaxIncomingQueue) ) 

#define IFaxAccountFolders_get_IncomingArchive(This,pFaxIncomingArchive)	\
    ( (This)->lpVtbl -> get_IncomingArchive(This,pFaxIncomingArchive) ) 

#define IFaxAccountFolders_get_OutgoingArchive(This,pFaxOutgoingArchive)	\
    ( (This)->lpVtbl -> get_OutgoingArchive(This,pFaxOutgoingArchive) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IFaxAccountFolders_INTERFACE_DEFINED__ */


#ifndef __IFaxAccountIncomingQueue_INTERFACE_DEFINED__
#define __IFaxAccountIncomingQueue_INTERFACE_DEFINED__

/* interface IFaxAccountIncomingQueue */
/* [unique][helpstring][dual][uuid][object] */ 


EXTERN_C const IID IID_IFaxAccountIncomingQueue;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("dd142d92-0186-4a95-a090-cbc3eadba6b4")
    IFaxAccountIncomingQueue : public IDispatch
    {
    public:
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE GetJobs( 
            /* [retval][out] */ __RPC__deref_out_opt IFaxIncomingJobs **pFaxIncomingJobs) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE GetJob( 
            /* [in] */ __RPC__in BSTR bstrJobId,
            /* [retval][out] */ __RPC__deref_out_opt IFaxIncomingJob **pFaxIncomingJob) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IFaxAccountIncomingQueueVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IFaxAccountIncomingQueue * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IFaxAccountIncomingQueue * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IFaxAccountIncomingQueue * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IFaxAccountIncomingQueue * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IFaxAccountIncomingQueue * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IFaxAccountIncomingQueue * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IFaxAccountIncomingQueue * This,
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
        
        DECLSPEC_XFGVIRT(IFaxAccountIncomingQueue, GetJobs)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *GetJobs )( 
            __RPC__in IFaxAccountIncomingQueue * This,
            /* [retval][out] */ __RPC__deref_out_opt IFaxIncomingJobs **pFaxIncomingJobs);
        
        DECLSPEC_XFGVIRT(IFaxAccountIncomingQueue, GetJob)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *GetJob )( 
            __RPC__in IFaxAccountIncomingQueue * This,
            /* [in] */ __RPC__in BSTR bstrJobId,
            /* [retval][out] */ __RPC__deref_out_opt IFaxIncomingJob **pFaxIncomingJob);
        
        END_INTERFACE
    } IFaxAccountIncomingQueueVtbl;

    interface IFaxAccountIncomingQueue
    {
        CONST_VTBL struct IFaxAccountIncomingQueueVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IFaxAccountIncomingQueue_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IFaxAccountIncomingQueue_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IFaxAccountIncomingQueue_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IFaxAccountIncomingQueue_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IFaxAccountIncomingQueue_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IFaxAccountIncomingQueue_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IFaxAccountIncomingQueue_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IFaxAccountIncomingQueue_GetJobs(This,pFaxIncomingJobs)	\
    ( (This)->lpVtbl -> GetJobs(This,pFaxIncomingJobs) ) 

#define IFaxAccountIncomingQueue_GetJob(This,bstrJobId,pFaxIncomingJob)	\
    ( (This)->lpVtbl -> GetJob(This,bstrJobId,pFaxIncomingJob) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IFaxAccountIncomingQueue_INTERFACE_DEFINED__ */


#ifndef __IFaxAccountOutgoingQueue_INTERFACE_DEFINED__
#define __IFaxAccountOutgoingQueue_INTERFACE_DEFINED__

/* interface IFaxAccountOutgoingQueue */
/* [unique][helpstring][dual][uuid][object] */ 


EXTERN_C const IID IID_IFaxAccountOutgoingQueue;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("0f1424e9-f22d-4553-b7a5-0d24bd0d7e46")
    IFaxAccountOutgoingQueue : public IDispatch
    {
    public:
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE GetJobs( 
            /* [retval][out] */ __RPC__deref_out_opt IFaxOutgoingJobs **pFaxOutgoingJobs) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE GetJob( 
            /* [in] */ __RPC__in BSTR bstrJobId,
            /* [retval][out] */ __RPC__deref_out_opt IFaxOutgoingJob **pFaxOutgoingJob) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IFaxAccountOutgoingQueueVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IFaxAccountOutgoingQueue * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IFaxAccountOutgoingQueue * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IFaxAccountOutgoingQueue * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IFaxAccountOutgoingQueue * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IFaxAccountOutgoingQueue * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IFaxAccountOutgoingQueue * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IFaxAccountOutgoingQueue * This,
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
        
        DECLSPEC_XFGVIRT(IFaxAccountOutgoingQueue, GetJobs)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *GetJobs )( 
            __RPC__in IFaxAccountOutgoingQueue * This,
            /* [retval][out] */ __RPC__deref_out_opt IFaxOutgoingJobs **pFaxOutgoingJobs);
        
        DECLSPEC_XFGVIRT(IFaxAccountOutgoingQueue, GetJob)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *GetJob )( 
            __RPC__in IFaxAccountOutgoingQueue * This,
            /* [in] */ __RPC__in BSTR bstrJobId,
            /* [retval][out] */ __RPC__deref_out_opt IFaxOutgoingJob **pFaxOutgoingJob);
        
        END_INTERFACE
    } IFaxAccountOutgoingQueueVtbl;

    interface IFaxAccountOutgoingQueue
    {
        CONST_VTBL struct IFaxAccountOutgoingQueueVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IFaxAccountOutgoingQueue_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IFaxAccountOutgoingQueue_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IFaxAccountOutgoingQueue_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IFaxAccountOutgoingQueue_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IFaxAccountOutgoingQueue_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IFaxAccountOutgoingQueue_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IFaxAccountOutgoingQueue_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IFaxAccountOutgoingQueue_GetJobs(This,pFaxOutgoingJobs)	\
    ( (This)->lpVtbl -> GetJobs(This,pFaxOutgoingJobs) ) 

#define IFaxAccountOutgoingQueue_GetJob(This,bstrJobId,pFaxOutgoingJob)	\
    ( (This)->lpVtbl -> GetJob(This,bstrJobId,pFaxOutgoingJob) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IFaxAccountOutgoingQueue_INTERFACE_DEFINED__ */


#ifndef __IFaxOutgoingMessage2_INTERFACE_DEFINED__
#define __IFaxOutgoingMessage2_INTERFACE_DEFINED__

/* interface IFaxOutgoingMessage2 */
/* [unique][helpstring][dual][uuid][object] */ 


EXTERN_C const IID IID_IFaxOutgoingMessage2;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("b37df687-bc88-4b46-b3be-b458b3ea9e7f")
    IFaxOutgoingMessage2 : public IFaxOutgoingMessage
    {
    public:
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_HasCoverPage( 
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pbHasCoverPage) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_ReceiptType( 
            /* [retval][out] */ __RPC__out FAX_RECEIPT_TYPE_ENUM *pReceiptType) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_ReceiptAddress( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrReceiptAddress) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Read( 
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pbRead) = 0;
        
        virtual /* [helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_Read( 
            /* [in] */ VARIANT_BOOL bRead) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE Save( void) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE Refresh( void) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IFaxOutgoingMessage2Vtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IFaxOutgoingMessage2 * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IFaxOutgoingMessage2 * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IFaxOutgoingMessage2 * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IFaxOutgoingMessage2 * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IFaxOutgoingMessage2 * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IFaxOutgoingMessage2 * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IFaxOutgoingMessage2 * This,
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
        
        DECLSPEC_XFGVIRT(IFaxOutgoingMessage, get_SubmissionId)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_SubmissionId )( 
            __RPC__in IFaxOutgoingMessage2 * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrSubmissionId);
        
        DECLSPEC_XFGVIRT(IFaxOutgoingMessage, get_Id)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Id )( 
            __RPC__in IFaxOutgoingMessage2 * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrId);
        
        DECLSPEC_XFGVIRT(IFaxOutgoingMessage, get_Subject)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Subject )( 
            __RPC__in IFaxOutgoingMessage2 * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrSubject);
        
        DECLSPEC_XFGVIRT(IFaxOutgoingMessage, get_DocumentName)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_DocumentName )( 
            __RPC__in IFaxOutgoingMessage2 * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrDocumentName);
        
        DECLSPEC_XFGVIRT(IFaxOutgoingMessage, get_Retries)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Retries )( 
            __RPC__in IFaxOutgoingMessage2 * This,
            /* [retval][out] */ __RPC__out long *plRetries);
        
        DECLSPEC_XFGVIRT(IFaxOutgoingMessage, get_Pages)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Pages )( 
            __RPC__in IFaxOutgoingMessage2 * This,
            /* [retval][out] */ __RPC__out long *plPages);
        
        DECLSPEC_XFGVIRT(IFaxOutgoingMessage, get_Size)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Size )( 
            __RPC__in IFaxOutgoingMessage2 * This,
            /* [retval][out] */ __RPC__out long *plSize);
        
        DECLSPEC_XFGVIRT(IFaxOutgoingMessage, get_OriginalScheduledTime)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_OriginalScheduledTime )( 
            __RPC__in IFaxOutgoingMessage2 * This,
            /* [retval][out] */ __RPC__out DATE *pdateOriginalScheduledTime);
        
        DECLSPEC_XFGVIRT(IFaxOutgoingMessage, get_SubmissionTime)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_SubmissionTime )( 
            __RPC__in IFaxOutgoingMessage2 * This,
            /* [retval][out] */ __RPC__out DATE *pdateSubmissionTime);
        
        DECLSPEC_XFGVIRT(IFaxOutgoingMessage, get_Priority)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Priority )( 
            __RPC__in IFaxOutgoingMessage2 * This,
            /* [retval][out] */ __RPC__out FAX_PRIORITY_TYPE_ENUM *pPriority);
        
        DECLSPEC_XFGVIRT(IFaxOutgoingMessage, get_Sender)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Sender )( 
            __RPC__in IFaxOutgoingMessage2 * This,
            /* [retval][out] */ __RPC__deref_out_opt IFaxSender **ppFaxSender);
        
        DECLSPEC_XFGVIRT(IFaxOutgoingMessage, get_Recipient)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Recipient )( 
            __RPC__in IFaxOutgoingMessage2 * This,
            /* [retval][out] */ __RPC__deref_out_opt IFaxRecipient **ppFaxRecipient);
        
        DECLSPEC_XFGVIRT(IFaxOutgoingMessage, get_DeviceName)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_DeviceName )( 
            __RPC__in IFaxOutgoingMessage2 * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrDeviceName);
        
        DECLSPEC_XFGVIRT(IFaxOutgoingMessage, get_TransmissionStart)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_TransmissionStart )( 
            __RPC__in IFaxOutgoingMessage2 * This,
            /* [retval][out] */ __RPC__out DATE *pdateTransmissionStart);
        
        DECLSPEC_XFGVIRT(IFaxOutgoingMessage, get_TransmissionEnd)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_TransmissionEnd )( 
            __RPC__in IFaxOutgoingMessage2 * This,
            /* [retval][out] */ __RPC__out DATE *pdateTransmissionEnd);
        
        DECLSPEC_XFGVIRT(IFaxOutgoingMessage, get_CSID)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_CSID )( 
            __RPC__in IFaxOutgoingMessage2 * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrCSID);
        
        DECLSPEC_XFGVIRT(IFaxOutgoingMessage, get_TSID)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_TSID )( 
            __RPC__in IFaxOutgoingMessage2 * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrTSID);
        
        DECLSPEC_XFGVIRT(IFaxOutgoingMessage, CopyTiff)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *CopyTiff )( 
            __RPC__in IFaxOutgoingMessage2 * This,
            /* [in] */ __RPC__in BSTR bstrTiffPath);
        
        DECLSPEC_XFGVIRT(IFaxOutgoingMessage, Delete)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *Delete )( 
            __RPC__in IFaxOutgoingMessage2 * This);
        
        DECLSPEC_XFGVIRT(IFaxOutgoingMessage2, get_HasCoverPage)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_HasCoverPage )( 
            __RPC__in IFaxOutgoingMessage2 * This,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pbHasCoverPage);
        
        DECLSPEC_XFGVIRT(IFaxOutgoingMessage2, get_ReceiptType)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_ReceiptType )( 
            __RPC__in IFaxOutgoingMessage2 * This,
            /* [retval][out] */ __RPC__out FAX_RECEIPT_TYPE_ENUM *pReceiptType);
        
        DECLSPEC_XFGVIRT(IFaxOutgoingMessage2, get_ReceiptAddress)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_ReceiptAddress )( 
            __RPC__in IFaxOutgoingMessage2 * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrReceiptAddress);
        
        DECLSPEC_XFGVIRT(IFaxOutgoingMessage2, get_Read)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Read )( 
            __RPC__in IFaxOutgoingMessage2 * This,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pbRead);
        
        DECLSPEC_XFGVIRT(IFaxOutgoingMessage2, put_Read)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_Read )( 
            __RPC__in IFaxOutgoingMessage2 * This,
            /* [in] */ VARIANT_BOOL bRead);
        
        DECLSPEC_XFGVIRT(IFaxOutgoingMessage2, Save)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *Save )( 
            __RPC__in IFaxOutgoingMessage2 * This);
        
        DECLSPEC_XFGVIRT(IFaxOutgoingMessage2, Refresh)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *Refresh )( 
            __RPC__in IFaxOutgoingMessage2 * This);
        
        END_INTERFACE
    } IFaxOutgoingMessage2Vtbl;

    interface IFaxOutgoingMessage2
    {
        CONST_VTBL struct IFaxOutgoingMessage2Vtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IFaxOutgoingMessage2_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IFaxOutgoingMessage2_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IFaxOutgoingMessage2_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IFaxOutgoingMessage2_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IFaxOutgoingMessage2_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IFaxOutgoingMessage2_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IFaxOutgoingMessage2_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IFaxOutgoingMessage2_get_SubmissionId(This,pbstrSubmissionId)	\
    ( (This)->lpVtbl -> get_SubmissionId(This,pbstrSubmissionId) ) 

#define IFaxOutgoingMessage2_get_Id(This,pbstrId)	\
    ( (This)->lpVtbl -> get_Id(This,pbstrId) ) 

#define IFaxOutgoingMessage2_get_Subject(This,pbstrSubject)	\
    ( (This)->lpVtbl -> get_Subject(This,pbstrSubject) ) 

#define IFaxOutgoingMessage2_get_DocumentName(This,pbstrDocumentName)	\
    ( (This)->lpVtbl -> get_DocumentName(This,pbstrDocumentName) ) 

#define IFaxOutgoingMessage2_get_Retries(This,plRetries)	\
    ( (This)->lpVtbl -> get_Retries(This,plRetries) ) 

#define IFaxOutgoingMessage2_get_Pages(This,plPages)	\
    ( (This)->lpVtbl -> get_Pages(This,plPages) ) 

#define IFaxOutgoingMessage2_get_Size(This,plSize)	\
    ( (This)->lpVtbl -> get_Size(This,plSize) ) 

#define IFaxOutgoingMessage2_get_OriginalScheduledTime(This,pdateOriginalScheduledTime)	\
    ( (This)->lpVtbl -> get_OriginalScheduledTime(This,pdateOriginalScheduledTime) ) 

#define IFaxOutgoingMessage2_get_SubmissionTime(This,pdateSubmissionTime)	\
    ( (This)->lpVtbl -> get_SubmissionTime(This,pdateSubmissionTime) ) 

#define IFaxOutgoingMessage2_get_Priority(This,pPriority)	\
    ( (This)->lpVtbl -> get_Priority(This,pPriority) ) 

#define IFaxOutgoingMessage2_get_Sender(This,ppFaxSender)	\
    ( (This)->lpVtbl -> get_Sender(This,ppFaxSender) ) 

#define IFaxOutgoingMessage2_get_Recipient(This,ppFaxRecipient)	\
    ( (This)->lpVtbl -> get_Recipient(This,ppFaxRecipient) ) 

#define IFaxOutgoingMessage2_get_DeviceName(This,pbstrDeviceName)	\
    ( (This)->lpVtbl -> get_DeviceName(This,pbstrDeviceName) ) 

#define IFaxOutgoingMessage2_get_TransmissionStart(This,pdateTransmissionStart)	\
    ( (This)->lpVtbl -> get_TransmissionStart(This,pdateTransmissionStart) ) 

#define IFaxOutgoingMessage2_get_TransmissionEnd(This,pdateTransmissionEnd)	\
    ( (This)->lpVtbl -> get_TransmissionEnd(This,pdateTransmissionEnd) ) 

#define IFaxOutgoingMessage2_get_CSID(This,pbstrCSID)	\
    ( (This)->lpVtbl -> get_CSID(This,pbstrCSID) ) 

#define IFaxOutgoingMessage2_get_TSID(This,pbstrTSID)	\
    ( (This)->lpVtbl -> get_TSID(This,pbstrTSID) ) 

#define IFaxOutgoingMessage2_CopyTiff(This,bstrTiffPath)	\
    ( (This)->lpVtbl -> CopyTiff(This,bstrTiffPath) ) 

#define IFaxOutgoingMessage2_Delete(This)	\
    ( (This)->lpVtbl -> Delete(This) ) 


#define IFaxOutgoingMessage2_get_HasCoverPage(This,pbHasCoverPage)	\
    ( (This)->lpVtbl -> get_HasCoverPage(This,pbHasCoverPage) ) 

#define IFaxOutgoingMessage2_get_ReceiptType(This,pReceiptType)	\
    ( (This)->lpVtbl -> get_ReceiptType(This,pReceiptType) ) 

#define IFaxOutgoingMessage2_get_ReceiptAddress(This,pbstrReceiptAddress)	\
    ( (This)->lpVtbl -> get_ReceiptAddress(This,pbstrReceiptAddress) ) 

#define IFaxOutgoingMessage2_get_Read(This,pbRead)	\
    ( (This)->lpVtbl -> get_Read(This,pbRead) ) 

#define IFaxOutgoingMessage2_put_Read(This,bRead)	\
    ( (This)->lpVtbl -> put_Read(This,bRead) ) 

#define IFaxOutgoingMessage2_Save(This)	\
    ( (This)->lpVtbl -> Save(This) ) 

#define IFaxOutgoingMessage2_Refresh(This)	\
    ( (This)->lpVtbl -> Refresh(This) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IFaxOutgoingMessage2_INTERFACE_DEFINED__ */


#ifndef __IFaxAccountIncomingArchive_INTERFACE_DEFINED__
#define __IFaxAccountIncomingArchive_INTERFACE_DEFINED__

/* interface IFaxAccountIncomingArchive */
/* [unique][helpstring][dual][uuid][object] */ 


EXTERN_C const IID IID_IFaxAccountIncomingArchive;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("a8a5b6ef-e0d6-4aee-955c-91625bec9db4")
    IFaxAccountIncomingArchive : public IDispatch
    {
    public:
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_SizeLow( 
            /* [retval][out] */ __RPC__out long *plSizeLow) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_SizeHigh( 
            /* [retval][out] */ __RPC__out long *plSizeHigh) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE Refresh( void) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE GetMessages( 
            /* [defaultvalue][in] */ long lPrefetchSize,
            /* [retval][out] */ __RPC__deref_out_opt IFaxIncomingMessageIterator **pFaxIncomingMessageIterator) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE GetMessage( 
            /* [in] */ __RPC__in BSTR bstrMessageId,
            /* [retval][out] */ __RPC__deref_out_opt IFaxIncomingMessage **pFaxIncomingMessage) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IFaxAccountIncomingArchiveVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IFaxAccountIncomingArchive * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IFaxAccountIncomingArchive * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IFaxAccountIncomingArchive * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IFaxAccountIncomingArchive * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IFaxAccountIncomingArchive * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IFaxAccountIncomingArchive * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IFaxAccountIncomingArchive * This,
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
        
        DECLSPEC_XFGVIRT(IFaxAccountIncomingArchive, get_SizeLow)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_SizeLow )( 
            __RPC__in IFaxAccountIncomingArchive * This,
            /* [retval][out] */ __RPC__out long *plSizeLow);
        
        DECLSPEC_XFGVIRT(IFaxAccountIncomingArchive, get_SizeHigh)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_SizeHigh )( 
            __RPC__in IFaxAccountIncomingArchive * This,
            /* [retval][out] */ __RPC__out long *plSizeHigh);
        
        DECLSPEC_XFGVIRT(IFaxAccountIncomingArchive, Refresh)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *Refresh )( 
            __RPC__in IFaxAccountIncomingArchive * This);
        
        DECLSPEC_XFGVIRT(IFaxAccountIncomingArchive, GetMessages)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *GetMessages )( 
            __RPC__in IFaxAccountIncomingArchive * This,
            /* [defaultvalue][in] */ long lPrefetchSize,
            /* [retval][out] */ __RPC__deref_out_opt IFaxIncomingMessageIterator **pFaxIncomingMessageIterator);
        
        DECLSPEC_XFGVIRT(IFaxAccountIncomingArchive, GetMessage)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *GetMessage )( 
            __RPC__in IFaxAccountIncomingArchive * This,
            /* [in] */ __RPC__in BSTR bstrMessageId,
            /* [retval][out] */ __RPC__deref_out_opt IFaxIncomingMessage **pFaxIncomingMessage);
        
        END_INTERFACE
    } IFaxAccountIncomingArchiveVtbl;

    interface IFaxAccountIncomingArchive
    {
        CONST_VTBL struct IFaxAccountIncomingArchiveVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IFaxAccountIncomingArchive_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IFaxAccountIncomingArchive_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IFaxAccountIncomingArchive_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IFaxAccountIncomingArchive_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IFaxAccountIncomingArchive_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IFaxAccountIncomingArchive_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IFaxAccountIncomingArchive_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IFaxAccountIncomingArchive_get_SizeLow(This,plSizeLow)	\
    ( (This)->lpVtbl -> get_SizeLow(This,plSizeLow) ) 

#define IFaxAccountIncomingArchive_get_SizeHigh(This,plSizeHigh)	\
    ( (This)->lpVtbl -> get_SizeHigh(This,plSizeHigh) ) 

#define IFaxAccountIncomingArchive_Refresh(This)	\
    ( (This)->lpVtbl -> Refresh(This) ) 

#define IFaxAccountIncomingArchive_GetMessages(This,lPrefetchSize,pFaxIncomingMessageIterator)	\
    ( (This)->lpVtbl -> GetMessages(This,lPrefetchSize,pFaxIncomingMessageIterator) ) 

#define IFaxAccountIncomingArchive_GetMessage(This,bstrMessageId,pFaxIncomingMessage)	\
    ( (This)->lpVtbl -> GetMessage(This,bstrMessageId,pFaxIncomingMessage) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IFaxAccountIncomingArchive_INTERFACE_DEFINED__ */


#ifndef __IFaxAccountOutgoingArchive_INTERFACE_DEFINED__
#define __IFaxAccountOutgoingArchive_INTERFACE_DEFINED__

/* interface IFaxAccountOutgoingArchive */
/* [unique][helpstring][dual][uuid][object] */ 


EXTERN_C const IID IID_IFaxAccountOutgoingArchive;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("5463076d-ec14-491f-926e-b3ceda5e5662")
    IFaxAccountOutgoingArchive : public IDispatch
    {
    public:
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_SizeLow( 
            /* [retval][out] */ __RPC__out long *plSizeLow) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_SizeHigh( 
            /* [retval][out] */ __RPC__out long *plSizeHigh) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE Refresh( void) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE GetMessages( 
            /* [defaultvalue][in] */ long lPrefetchSize,
            /* [retval][out] */ __RPC__deref_out_opt IFaxOutgoingMessageIterator **pFaxOutgoingMessageIterator) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE GetMessage( 
            /* [in] */ __RPC__in BSTR bstrMessageId,
            /* [retval][out] */ __RPC__deref_out_opt IFaxOutgoingMessage **pFaxOutgoingMessage) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IFaxAccountOutgoingArchiveVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IFaxAccountOutgoingArchive * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IFaxAccountOutgoingArchive * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IFaxAccountOutgoingArchive * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IFaxAccountOutgoingArchive * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IFaxAccountOutgoingArchive * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IFaxAccountOutgoingArchive * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IFaxAccountOutgoingArchive * This,
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
        
        DECLSPEC_XFGVIRT(IFaxAccountOutgoingArchive, get_SizeLow)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_SizeLow )( 
            __RPC__in IFaxAccountOutgoingArchive * This,
            /* [retval][out] */ __RPC__out long *plSizeLow);
        
        DECLSPEC_XFGVIRT(IFaxAccountOutgoingArchive, get_SizeHigh)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_SizeHigh )( 
            __RPC__in IFaxAccountOutgoingArchive * This,
            /* [retval][out] */ __RPC__out long *plSizeHigh);
        
        DECLSPEC_XFGVIRT(IFaxAccountOutgoingArchive, Refresh)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *Refresh )( 
            __RPC__in IFaxAccountOutgoingArchive * This);
        
        DECLSPEC_XFGVIRT(IFaxAccountOutgoingArchive, GetMessages)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *GetMessages )( 
            __RPC__in IFaxAccountOutgoingArchive * This,
            /* [defaultvalue][in] */ long lPrefetchSize,
            /* [retval][out] */ __RPC__deref_out_opt IFaxOutgoingMessageIterator **pFaxOutgoingMessageIterator);
        
        DECLSPEC_XFGVIRT(IFaxAccountOutgoingArchive, GetMessage)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *GetMessage )( 
            __RPC__in IFaxAccountOutgoingArchive * This,
            /* [in] */ __RPC__in BSTR bstrMessageId,
            /* [retval][out] */ __RPC__deref_out_opt IFaxOutgoingMessage **pFaxOutgoingMessage);
        
        END_INTERFACE
    } IFaxAccountOutgoingArchiveVtbl;

    interface IFaxAccountOutgoingArchive
    {
        CONST_VTBL struct IFaxAccountOutgoingArchiveVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IFaxAccountOutgoingArchive_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IFaxAccountOutgoingArchive_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IFaxAccountOutgoingArchive_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IFaxAccountOutgoingArchive_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IFaxAccountOutgoingArchive_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IFaxAccountOutgoingArchive_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IFaxAccountOutgoingArchive_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IFaxAccountOutgoingArchive_get_SizeLow(This,plSizeLow)	\
    ( (This)->lpVtbl -> get_SizeLow(This,plSizeLow) ) 

#define IFaxAccountOutgoingArchive_get_SizeHigh(This,plSizeHigh)	\
    ( (This)->lpVtbl -> get_SizeHigh(This,plSizeHigh) ) 

#define IFaxAccountOutgoingArchive_Refresh(This)	\
    ( (This)->lpVtbl -> Refresh(This) ) 

#define IFaxAccountOutgoingArchive_GetMessages(This,lPrefetchSize,pFaxOutgoingMessageIterator)	\
    ( (This)->lpVtbl -> GetMessages(This,lPrefetchSize,pFaxOutgoingMessageIterator) ) 

#define IFaxAccountOutgoingArchive_GetMessage(This,bstrMessageId,pFaxOutgoingMessage)	\
    ( (This)->lpVtbl -> GetMessage(This,bstrMessageId,pFaxOutgoingMessage) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IFaxAccountOutgoingArchive_INTERFACE_DEFINED__ */


#ifndef __IFaxSecurity2_INTERFACE_DEFINED__
#define __IFaxSecurity2_INTERFACE_DEFINED__

/* interface IFaxSecurity2 */
/* [nonextensible][unique][helpstring][dual][uuid][object] */ 

typedef 
enum FAX_ACCESS_RIGHTS_ENUM_2
    {
        far2SUBMIT_LOW	= 0x1,
        far2SUBMIT_NORMAL	= 0x2,
        far2SUBMIT_HIGH	= 0x4,
        far2QUERY_OUT_JOBS	= 0x8,
        far2MANAGE_OUT_JOBS	= 0x10,
        far2QUERY_CONFIG	= 0x20,
        far2MANAGE_CONFIG	= 0x40,
        far2QUERY_ARCHIVES	= 0x80,
        far2MANAGE_ARCHIVES	= 0x100,
        far2MANAGE_RECEIVE_FOLDER	= 0x200
    } 	FAX_ACCESS_RIGHTS_ENUM_2;


EXTERN_C const IID IID_IFaxSecurity2;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("17d851f4-d09b-48fc-99c9-8f24c4db9ab1")
    IFaxSecurity2 : public IDispatch
    {
    public:
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Descriptor( 
            /* [retval][out] */ __RPC__out VARIANT *pvDescriptor) = 0;
        
        virtual /* [helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_Descriptor( 
            /* [in] */ VARIANT vDescriptor) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_GrantedRights( 
            /* [retval][out] */ __RPC__out FAX_ACCESS_RIGHTS_ENUM_2 *pGrantedRights) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE Refresh( void) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE Save( void) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_InformationType( 
            /* [retval][out] */ __RPC__out long *plInformationType) = 0;
        
        virtual /* [helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_InformationType( 
            /* [in] */ long lInformationType) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IFaxSecurity2Vtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IFaxSecurity2 * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IFaxSecurity2 * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IFaxSecurity2 * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IFaxSecurity2 * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IFaxSecurity2 * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IFaxSecurity2 * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IFaxSecurity2 * This,
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
        
        DECLSPEC_XFGVIRT(IFaxSecurity2, get_Descriptor)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Descriptor )( 
            __RPC__in IFaxSecurity2 * This,
            /* [retval][out] */ __RPC__out VARIANT *pvDescriptor);
        
        DECLSPEC_XFGVIRT(IFaxSecurity2, put_Descriptor)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_Descriptor )( 
            __RPC__in IFaxSecurity2 * This,
            /* [in] */ VARIANT vDescriptor);
        
        DECLSPEC_XFGVIRT(IFaxSecurity2, get_GrantedRights)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_GrantedRights )( 
            __RPC__in IFaxSecurity2 * This,
            /* [retval][out] */ __RPC__out FAX_ACCESS_RIGHTS_ENUM_2 *pGrantedRights);
        
        DECLSPEC_XFGVIRT(IFaxSecurity2, Refresh)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *Refresh )( 
            __RPC__in IFaxSecurity2 * This);
        
        DECLSPEC_XFGVIRT(IFaxSecurity2, Save)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *Save )( 
            __RPC__in IFaxSecurity2 * This);
        
        DECLSPEC_XFGVIRT(IFaxSecurity2, get_InformationType)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_InformationType )( 
            __RPC__in IFaxSecurity2 * This,
            /* [retval][out] */ __RPC__out long *plInformationType);
        
        DECLSPEC_XFGVIRT(IFaxSecurity2, put_InformationType)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_InformationType )( 
            __RPC__in IFaxSecurity2 * This,
            /* [in] */ long lInformationType);
        
        END_INTERFACE
    } IFaxSecurity2Vtbl;

    interface IFaxSecurity2
    {
        CONST_VTBL struct IFaxSecurity2Vtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IFaxSecurity2_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IFaxSecurity2_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IFaxSecurity2_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IFaxSecurity2_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IFaxSecurity2_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IFaxSecurity2_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IFaxSecurity2_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IFaxSecurity2_get_Descriptor(This,pvDescriptor)	\
    ( (This)->lpVtbl -> get_Descriptor(This,pvDescriptor) ) 

#define IFaxSecurity2_put_Descriptor(This,vDescriptor)	\
    ( (This)->lpVtbl -> put_Descriptor(This,vDescriptor) ) 

#define IFaxSecurity2_get_GrantedRights(This,pGrantedRights)	\
    ( (This)->lpVtbl -> get_GrantedRights(This,pGrantedRights) ) 

#define IFaxSecurity2_Refresh(This)	\
    ( (This)->lpVtbl -> Refresh(This) ) 

#define IFaxSecurity2_Save(This)	\
    ( (This)->lpVtbl -> Save(This) ) 

#define IFaxSecurity2_get_InformationType(This,plInformationType)	\
    ( (This)->lpVtbl -> get_InformationType(This,plInformationType) ) 

#define IFaxSecurity2_put_InformationType(This,lInformationType)	\
    ( (This)->lpVtbl -> put_InformationType(This,lInformationType) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IFaxSecurity2_INTERFACE_DEFINED__ */


#ifndef __IFaxIncomingMessage2_INTERFACE_DEFINED__
#define __IFaxIncomingMessage2_INTERFACE_DEFINED__

/* interface IFaxIncomingMessage2 */
/* [unique][helpstring][dual][uuid][object] */ 


EXTERN_C const IID IID_IFaxIncomingMessage2;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("f9208503-e2bc-48f3-9ec0-e6236f9b509a")
    IFaxIncomingMessage2 : public IFaxIncomingMessage
    {
    public:
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Subject( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrSubject) = 0;
        
        virtual /* [helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_Subject( 
            /* [in] */ __RPC__in BSTR bstrSubject) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_SenderName( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrSenderName) = 0;
        
        virtual /* [helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_SenderName( 
            /* [in] */ __RPC__in BSTR bstrSenderName) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_SenderFaxNumber( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrSenderFaxNumber) = 0;
        
        virtual /* [helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_SenderFaxNumber( 
            /* [in] */ __RPC__in BSTR bstrSenderFaxNumber) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_HasCoverPage( 
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pbHasCoverPage) = 0;
        
        virtual /* [helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_HasCoverPage( 
            /* [in] */ VARIANT_BOOL bHasCoverPage) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Recipients( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrRecipients) = 0;
        
        virtual /* [helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_Recipients( 
            /* [in] */ __RPC__in BSTR bstrRecipients) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_WasReAssigned( 
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pbWasReAssigned) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Read( 
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pbRead) = 0;
        
        virtual /* [helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_Read( 
            /* [in] */ VARIANT_BOOL bRead) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE ReAssign( void) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE Save( void) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE Refresh( void) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IFaxIncomingMessage2Vtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IFaxIncomingMessage2 * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IFaxIncomingMessage2 * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IFaxIncomingMessage2 * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IFaxIncomingMessage2 * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IFaxIncomingMessage2 * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IFaxIncomingMessage2 * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IFaxIncomingMessage2 * This,
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
        
        DECLSPEC_XFGVIRT(IFaxIncomingMessage, get_Id)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Id )( 
            __RPC__in IFaxIncomingMessage2 * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrId);
        
        DECLSPEC_XFGVIRT(IFaxIncomingMessage, get_Pages)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Pages )( 
            __RPC__in IFaxIncomingMessage2 * This,
            /* [retval][out] */ __RPC__out long *plPages);
        
        DECLSPEC_XFGVIRT(IFaxIncomingMessage, get_Size)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Size )( 
            __RPC__in IFaxIncomingMessage2 * This,
            /* [retval][out] */ __RPC__out long *plSize);
        
        DECLSPEC_XFGVIRT(IFaxIncomingMessage, get_DeviceName)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_DeviceName )( 
            __RPC__in IFaxIncomingMessage2 * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrDeviceName);
        
        DECLSPEC_XFGVIRT(IFaxIncomingMessage, get_Retries)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Retries )( 
            __RPC__in IFaxIncomingMessage2 * This,
            /* [retval][out] */ __RPC__out long *plRetries);
        
        DECLSPEC_XFGVIRT(IFaxIncomingMessage, get_TransmissionStart)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_TransmissionStart )( 
            __RPC__in IFaxIncomingMessage2 * This,
            /* [retval][out] */ __RPC__out DATE *pdateTransmissionStart);
        
        DECLSPEC_XFGVIRT(IFaxIncomingMessage, get_TransmissionEnd)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_TransmissionEnd )( 
            __RPC__in IFaxIncomingMessage2 * This,
            /* [retval][out] */ __RPC__out DATE *pdateTransmissionEnd);
        
        DECLSPEC_XFGVIRT(IFaxIncomingMessage, get_CSID)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_CSID )( 
            __RPC__in IFaxIncomingMessage2 * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrCSID);
        
        DECLSPEC_XFGVIRT(IFaxIncomingMessage, get_TSID)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_TSID )( 
            __RPC__in IFaxIncomingMessage2 * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrTSID);
        
        DECLSPEC_XFGVIRT(IFaxIncomingMessage, get_CallerId)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_CallerId )( 
            __RPC__in IFaxIncomingMessage2 * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrCallerId);
        
        DECLSPEC_XFGVIRT(IFaxIncomingMessage, get_RoutingInformation)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_RoutingInformation )( 
            __RPC__in IFaxIncomingMessage2 * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrRoutingInformation);
        
        DECLSPEC_XFGVIRT(IFaxIncomingMessage, CopyTiff)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *CopyTiff )( 
            __RPC__in IFaxIncomingMessage2 * This,
            /* [in] */ __RPC__in BSTR bstrTiffPath);
        
        DECLSPEC_XFGVIRT(IFaxIncomingMessage, Delete)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *Delete )( 
            __RPC__in IFaxIncomingMessage2 * This);
        
        DECLSPEC_XFGVIRT(IFaxIncomingMessage2, get_Subject)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Subject )( 
            __RPC__in IFaxIncomingMessage2 * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrSubject);
        
        DECLSPEC_XFGVIRT(IFaxIncomingMessage2, put_Subject)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_Subject )( 
            __RPC__in IFaxIncomingMessage2 * This,
            /* [in] */ __RPC__in BSTR bstrSubject);
        
        DECLSPEC_XFGVIRT(IFaxIncomingMessage2, get_SenderName)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_SenderName )( 
            __RPC__in IFaxIncomingMessage2 * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrSenderName);
        
        DECLSPEC_XFGVIRT(IFaxIncomingMessage2, put_SenderName)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_SenderName )( 
            __RPC__in IFaxIncomingMessage2 * This,
            /* [in] */ __RPC__in BSTR bstrSenderName);
        
        DECLSPEC_XFGVIRT(IFaxIncomingMessage2, get_SenderFaxNumber)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_SenderFaxNumber )( 
            __RPC__in IFaxIncomingMessage2 * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrSenderFaxNumber);
        
        DECLSPEC_XFGVIRT(IFaxIncomingMessage2, put_SenderFaxNumber)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_SenderFaxNumber )( 
            __RPC__in IFaxIncomingMessage2 * This,
            /* [in] */ __RPC__in BSTR bstrSenderFaxNumber);
        
        DECLSPEC_XFGVIRT(IFaxIncomingMessage2, get_HasCoverPage)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_HasCoverPage )( 
            __RPC__in IFaxIncomingMessage2 * This,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pbHasCoverPage);
        
        DECLSPEC_XFGVIRT(IFaxIncomingMessage2, put_HasCoverPage)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_HasCoverPage )( 
            __RPC__in IFaxIncomingMessage2 * This,
            /* [in] */ VARIANT_BOOL bHasCoverPage);
        
        DECLSPEC_XFGVIRT(IFaxIncomingMessage2, get_Recipients)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Recipients )( 
            __RPC__in IFaxIncomingMessage2 * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrRecipients);
        
        DECLSPEC_XFGVIRT(IFaxIncomingMessage2, put_Recipients)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_Recipients )( 
            __RPC__in IFaxIncomingMessage2 * This,
            /* [in] */ __RPC__in BSTR bstrRecipients);
        
        DECLSPEC_XFGVIRT(IFaxIncomingMessage2, get_WasReAssigned)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_WasReAssigned )( 
            __RPC__in IFaxIncomingMessage2 * This,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pbWasReAssigned);
        
        DECLSPEC_XFGVIRT(IFaxIncomingMessage2, get_Read)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Read )( 
            __RPC__in IFaxIncomingMessage2 * This,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pbRead);
        
        DECLSPEC_XFGVIRT(IFaxIncomingMessage2, put_Read)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_Read )( 
            __RPC__in IFaxIncomingMessage2 * This,
            /* [in] */ VARIANT_BOOL bRead);
        
        DECLSPEC_XFGVIRT(IFaxIncomingMessage2, ReAssign)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *ReAssign )( 
            __RPC__in IFaxIncomingMessage2 * This);
        
        DECLSPEC_XFGVIRT(IFaxIncomingMessage2, Save)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *Save )( 
            __RPC__in IFaxIncomingMessage2 * This);
        
        DECLSPEC_XFGVIRT(IFaxIncomingMessage2, Refresh)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *Refresh )( 
            __RPC__in IFaxIncomingMessage2 * This);
        
        END_INTERFACE
    } IFaxIncomingMessage2Vtbl;

    interface IFaxIncomingMessage2
    {
        CONST_VTBL struct IFaxIncomingMessage2Vtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IFaxIncomingMessage2_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IFaxIncomingMessage2_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IFaxIncomingMessage2_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IFaxIncomingMessage2_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IFaxIncomingMessage2_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IFaxIncomingMessage2_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IFaxIncomingMessage2_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IFaxIncomingMessage2_get_Id(This,pbstrId)	\
    ( (This)->lpVtbl -> get_Id(This,pbstrId) ) 

#define IFaxIncomingMessage2_get_Pages(This,plPages)	\
    ( (This)->lpVtbl -> get_Pages(This,plPages) ) 

#define IFaxIncomingMessage2_get_Size(This,plSize)	\
    ( (This)->lpVtbl -> get_Size(This,plSize) ) 

#define IFaxIncomingMessage2_get_DeviceName(This,pbstrDeviceName)	\
    ( (This)->lpVtbl -> get_DeviceName(This,pbstrDeviceName) ) 

#define IFaxIncomingMessage2_get_Retries(This,plRetries)	\
    ( (This)->lpVtbl -> get_Retries(This,plRetries) ) 

#define IFaxIncomingMessage2_get_TransmissionStart(This,pdateTransmissionStart)	\
    ( (This)->lpVtbl -> get_TransmissionStart(This,pdateTransmissionStart) ) 

#define IFaxIncomingMessage2_get_TransmissionEnd(This,pdateTransmissionEnd)	\
    ( (This)->lpVtbl -> get_TransmissionEnd(This,pdateTransmissionEnd) ) 

#define IFaxIncomingMessage2_get_CSID(This,pbstrCSID)	\
    ( (This)->lpVtbl -> get_CSID(This,pbstrCSID) ) 

#define IFaxIncomingMessage2_get_TSID(This,pbstrTSID)	\
    ( (This)->lpVtbl -> get_TSID(This,pbstrTSID) ) 

#define IFaxIncomingMessage2_get_CallerId(This,pbstrCallerId)	\
    ( (This)->lpVtbl -> get_CallerId(This,pbstrCallerId) ) 

#define IFaxIncomingMessage2_get_RoutingInformation(This,pbstrRoutingInformation)	\
    ( (This)->lpVtbl -> get_RoutingInformation(This,pbstrRoutingInformation) ) 

#define IFaxIncomingMessage2_CopyTiff(This,bstrTiffPath)	\
    ( (This)->lpVtbl -> CopyTiff(This,bstrTiffPath) ) 

#define IFaxIncomingMessage2_Delete(This)	\
    ( (This)->lpVtbl -> Delete(This) ) 


#define IFaxIncomingMessage2_get_Subject(This,pbstrSubject)	\
    ( (This)->lpVtbl -> get_Subject(This,pbstrSubject) ) 

#define IFaxIncomingMessage2_put_Subject(This,bstrSubject)	\
    ( (This)->lpVtbl -> put_Subject(This,bstrSubject) ) 

#define IFaxIncomingMessage2_get_SenderName(This,pbstrSenderName)	\
    ( (This)->lpVtbl -> get_SenderName(This,pbstrSenderName) ) 

#define IFaxIncomingMessage2_put_SenderName(This,bstrSenderName)	\
    ( (This)->lpVtbl -> put_SenderName(This,bstrSenderName) ) 

#define IFaxIncomingMessage2_get_SenderFaxNumber(This,pbstrSenderFaxNumber)	\
    ( (This)->lpVtbl -> get_SenderFaxNumber(This,pbstrSenderFaxNumber) ) 

#define IFaxIncomingMessage2_put_SenderFaxNumber(This,bstrSenderFaxNumber)	\
    ( (This)->lpVtbl -> put_SenderFaxNumber(This,bstrSenderFaxNumber) ) 

#define IFaxIncomingMessage2_get_HasCoverPage(This,pbHasCoverPage)	\
    ( (This)->lpVtbl -> get_HasCoverPage(This,pbHasCoverPage) ) 

#define IFaxIncomingMessage2_put_HasCoverPage(This,bHasCoverPage)	\
    ( (This)->lpVtbl -> put_HasCoverPage(This,bHasCoverPage) ) 

#define IFaxIncomingMessage2_get_Recipients(This,pbstrRecipients)	\
    ( (This)->lpVtbl -> get_Recipients(This,pbstrRecipients) ) 

#define IFaxIncomingMessage2_put_Recipients(This,bstrRecipients)	\
    ( (This)->lpVtbl -> put_Recipients(This,bstrRecipients) ) 

#define IFaxIncomingMessage2_get_WasReAssigned(This,pbWasReAssigned)	\
    ( (This)->lpVtbl -> get_WasReAssigned(This,pbWasReAssigned) ) 

#define IFaxIncomingMessage2_get_Read(This,pbRead)	\
    ( (This)->lpVtbl -> get_Read(This,pbRead) ) 

#define IFaxIncomingMessage2_put_Read(This,bRead)	\
    ( (This)->lpVtbl -> put_Read(This,bRead) ) 

#define IFaxIncomingMessage2_ReAssign(This)	\
    ( (This)->lpVtbl -> ReAssign(This) ) 

#define IFaxIncomingMessage2_Save(This)	\
    ( (This)->lpVtbl -> Save(This) ) 

#define IFaxIncomingMessage2_Refresh(This)	\
    ( (This)->lpVtbl -> Refresh(This) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IFaxIncomingMessage2_INTERFACE_DEFINED__ */



#ifndef __FAXCOMEXLib_LIBRARY_DEFINED__
#define __FAXCOMEXLib_LIBRARY_DEFINED__

/* library FAXCOMEXLib */
/* [helpstring][version][uuid] */ 

typedef 
enum FAX_ROUTING_RULE_CODE_ENUM
    {
        frrcANY_CODE	= 0
    } 	FAX_ROUTING_RULE_CODE_ENUM;


EXTERN_C const IID LIBID_FAXCOMEXLib;

#ifndef __IFaxServerNotify_DISPINTERFACE_DEFINED__
#define __IFaxServerNotify_DISPINTERFACE_DEFINED__

/* dispinterface IFaxServerNotify */
/* [helpstring][uuid] */ 


EXTERN_C const IID DIID_IFaxServerNotify;

#if defined(__cplusplus) && !defined(CINTERFACE)

    MIDL_INTERFACE("2E037B27-CF8A-4abd-B1E0-5704943BEA6F")
    IFaxServerNotify : public IDispatch
    {
    };
    
#else 	/* C style interface */

    typedef struct IFaxServerNotifyVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IFaxServerNotify * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IFaxServerNotify * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IFaxServerNotify * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IFaxServerNotify * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IFaxServerNotify * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IFaxServerNotify * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IFaxServerNotify * This,
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
    } IFaxServerNotifyVtbl;

    interface IFaxServerNotify
    {
        CONST_VTBL struct IFaxServerNotifyVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IFaxServerNotify_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IFaxServerNotify_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IFaxServerNotify_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IFaxServerNotify_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IFaxServerNotify_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IFaxServerNotify_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IFaxServerNotify_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */


#endif 	/* __IFaxServerNotify_DISPINTERFACE_DEFINED__ */


#ifndef ___IFaxServerNotify2_INTERFACE_DEFINED__
#define ___IFaxServerNotify2_INTERFACE_DEFINED__

/* interface _IFaxServerNotify2 */
/* [unique][uuid][object] */ 


EXTERN_C const IID IID__IFaxServerNotify2;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("ec9c69b9-5fe7-4805-9467-82fcd96af903")
    _IFaxServerNotify2 : public IDispatch
    {
    public:
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE OnIncomingJobAdded( 
            /* [in] */ __RPC__in_opt IFaxServer2 *pFaxServer,
            /* [in] */ __RPC__in BSTR bstrJobId) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE OnIncomingJobRemoved( 
            /* [in] */ __RPC__in_opt IFaxServer2 *pFaxServer,
            /* [in] */ __RPC__in BSTR bstrJobId) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE OnIncomingJobChanged( 
            /* [in] */ __RPC__in_opt IFaxServer2 *pFaxServer,
            /* [in] */ __RPC__in BSTR bstrJobId,
            /* [in] */ __RPC__in_opt IFaxJobStatus *pJobStatus) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE OnOutgoingJobAdded( 
            /* [in] */ __RPC__in_opt IFaxServer2 *pFaxServer,
            /* [in] */ __RPC__in BSTR bstrJobId) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE OnOutgoingJobRemoved( 
            /* [in] */ __RPC__in_opt IFaxServer2 *pFaxServer,
            /* [in] */ __RPC__in BSTR bstrJobId) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE OnOutgoingJobChanged( 
            /* [in] */ __RPC__in_opt IFaxServer2 *pFaxServer,
            /* [in] */ __RPC__in BSTR bstrJobId,
            /* [in] */ __RPC__in_opt IFaxJobStatus *pJobStatus) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE OnIncomingMessageAdded( 
            /* [in] */ __RPC__in_opt IFaxServer2 *pFaxServer,
            /* [in] */ __RPC__in BSTR bstrMessageId) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE OnIncomingMessageRemoved( 
            /* [in] */ __RPC__in_opt IFaxServer2 *pFaxServer,
            /* [in] */ __RPC__in BSTR bstrMessageId) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE OnOutgoingMessageAdded( 
            /* [in] */ __RPC__in_opt IFaxServer2 *pFaxServer,
            /* [in] */ __RPC__in BSTR bstrMessageId) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE OnOutgoingMessageRemoved( 
            /* [in] */ __RPC__in_opt IFaxServer2 *pFaxServer,
            /* [in] */ __RPC__in BSTR bstrMessageId) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE OnReceiptOptionsChange( 
            /* [in] */ __RPC__in_opt IFaxServer2 *pFaxServer) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE OnActivityLoggingConfigChange( 
            /* [in] */ __RPC__in_opt IFaxServer2 *pFaxServer) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE OnSecurityConfigChange( 
            /* [in] */ __RPC__in_opt IFaxServer2 *pFaxServer) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE OnEventLoggingConfigChange( 
            /* [in] */ __RPC__in_opt IFaxServer2 *pFaxServer) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE OnOutgoingQueueConfigChange( 
            /* [in] */ __RPC__in_opt IFaxServer2 *pFaxServer) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE OnOutgoingArchiveConfigChange( 
            /* [in] */ __RPC__in_opt IFaxServer2 *pFaxServer) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE OnIncomingArchiveConfigChange( 
            /* [in] */ __RPC__in_opt IFaxServer2 *pFaxServer) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE OnDevicesConfigChange( 
            /* [in] */ __RPC__in_opt IFaxServer2 *pFaxServer) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE OnOutboundRoutingGroupsConfigChange( 
            /* [in] */ __RPC__in_opt IFaxServer2 *pFaxServer) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE OnOutboundRoutingRulesConfigChange( 
            /* [in] */ __RPC__in_opt IFaxServer2 *pFaxServer) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE OnServerActivityChange( 
            /* [in] */ __RPC__in_opt IFaxServer2 *pFaxServer,
            /* [in] */ long lIncomingMessages,
            /* [in] */ long lRoutingMessages,
            /* [in] */ long lOutgoingMessages,
            /* [in] */ long lQueuedMessages) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE OnQueuesStatusChange( 
            /* [in] */ __RPC__in_opt IFaxServer2 *pFaxServer,
            /* [in] */ VARIANT_BOOL bOutgoingQueueBlocked,
            /* [in] */ VARIANT_BOOL bOutgoingQueuePaused,
            /* [in] */ VARIANT_BOOL bIncomingQueueBlocked) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE OnNewCall( 
            /* [in] */ __RPC__in_opt IFaxServer2 *pFaxServer,
            /* [in] */ long lCallId,
            /* [in] */ long lDeviceId,
            /* [in] */ __RPC__in BSTR bstrCallerId) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE OnServerShutDown( 
            /* [in] */ __RPC__in_opt IFaxServer2 *pFaxServer) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE OnDeviceStatusChange( 
            /* [in] */ __RPC__in_opt IFaxServer2 *pFaxServer,
            /* [in] */ long lDeviceId,
            /* [in] */ VARIANT_BOOL bPoweredOff,
            /* [in] */ VARIANT_BOOL bSending,
            /* [in] */ VARIANT_BOOL bReceiving,
            /* [in] */ VARIANT_BOOL bRinging) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE OnGeneralServerConfigChanged( 
            /* [in] */ __RPC__in_opt IFaxServer2 *pFaxServer) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct _IFaxServerNotify2Vtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in _IFaxServerNotify2 * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in _IFaxServerNotify2 * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in _IFaxServerNotify2 * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in _IFaxServerNotify2 * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in _IFaxServerNotify2 * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in _IFaxServerNotify2 * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            _IFaxServerNotify2 * This,
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
        
        DECLSPEC_XFGVIRT(_IFaxServerNotify2, OnIncomingJobAdded)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *OnIncomingJobAdded )( 
            __RPC__in _IFaxServerNotify2 * This,
            /* [in] */ __RPC__in_opt IFaxServer2 *pFaxServer,
            /* [in] */ __RPC__in BSTR bstrJobId);
        
        DECLSPEC_XFGVIRT(_IFaxServerNotify2, OnIncomingJobRemoved)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *OnIncomingJobRemoved )( 
            __RPC__in _IFaxServerNotify2 * This,
            /* [in] */ __RPC__in_opt IFaxServer2 *pFaxServer,
            /* [in] */ __RPC__in BSTR bstrJobId);
        
        DECLSPEC_XFGVIRT(_IFaxServerNotify2, OnIncomingJobChanged)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *OnIncomingJobChanged )( 
            __RPC__in _IFaxServerNotify2 * This,
            /* [in] */ __RPC__in_opt IFaxServer2 *pFaxServer,
            /* [in] */ __RPC__in BSTR bstrJobId,
            /* [in] */ __RPC__in_opt IFaxJobStatus *pJobStatus);
        
        DECLSPEC_XFGVIRT(_IFaxServerNotify2, OnOutgoingJobAdded)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *OnOutgoingJobAdded )( 
            __RPC__in _IFaxServerNotify2 * This,
            /* [in] */ __RPC__in_opt IFaxServer2 *pFaxServer,
            /* [in] */ __RPC__in BSTR bstrJobId);
        
        DECLSPEC_XFGVIRT(_IFaxServerNotify2, OnOutgoingJobRemoved)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *OnOutgoingJobRemoved )( 
            __RPC__in _IFaxServerNotify2 * This,
            /* [in] */ __RPC__in_opt IFaxServer2 *pFaxServer,
            /* [in] */ __RPC__in BSTR bstrJobId);
        
        DECLSPEC_XFGVIRT(_IFaxServerNotify2, OnOutgoingJobChanged)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *OnOutgoingJobChanged )( 
            __RPC__in _IFaxServerNotify2 * This,
            /* [in] */ __RPC__in_opt IFaxServer2 *pFaxServer,
            /* [in] */ __RPC__in BSTR bstrJobId,
            /* [in] */ __RPC__in_opt IFaxJobStatus *pJobStatus);
        
        DECLSPEC_XFGVIRT(_IFaxServerNotify2, OnIncomingMessageAdded)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *OnIncomingMessageAdded )( 
            __RPC__in _IFaxServerNotify2 * This,
            /* [in] */ __RPC__in_opt IFaxServer2 *pFaxServer,
            /* [in] */ __RPC__in BSTR bstrMessageId);
        
        DECLSPEC_XFGVIRT(_IFaxServerNotify2, OnIncomingMessageRemoved)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *OnIncomingMessageRemoved )( 
            __RPC__in _IFaxServerNotify2 * This,
            /* [in] */ __RPC__in_opt IFaxServer2 *pFaxServer,
            /* [in] */ __RPC__in BSTR bstrMessageId);
        
        DECLSPEC_XFGVIRT(_IFaxServerNotify2, OnOutgoingMessageAdded)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *OnOutgoingMessageAdded )( 
            __RPC__in _IFaxServerNotify2 * This,
            /* [in] */ __RPC__in_opt IFaxServer2 *pFaxServer,
            /* [in] */ __RPC__in BSTR bstrMessageId);
        
        DECLSPEC_XFGVIRT(_IFaxServerNotify2, OnOutgoingMessageRemoved)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *OnOutgoingMessageRemoved )( 
            __RPC__in _IFaxServerNotify2 * This,
            /* [in] */ __RPC__in_opt IFaxServer2 *pFaxServer,
            /* [in] */ __RPC__in BSTR bstrMessageId);
        
        DECLSPEC_XFGVIRT(_IFaxServerNotify2, OnReceiptOptionsChange)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *OnReceiptOptionsChange )( 
            __RPC__in _IFaxServerNotify2 * This,
            /* [in] */ __RPC__in_opt IFaxServer2 *pFaxServer);
        
        DECLSPEC_XFGVIRT(_IFaxServerNotify2, OnActivityLoggingConfigChange)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *OnActivityLoggingConfigChange )( 
            __RPC__in _IFaxServerNotify2 * This,
            /* [in] */ __RPC__in_opt IFaxServer2 *pFaxServer);
        
        DECLSPEC_XFGVIRT(_IFaxServerNotify2, OnSecurityConfigChange)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *OnSecurityConfigChange )( 
            __RPC__in _IFaxServerNotify2 * This,
            /* [in] */ __RPC__in_opt IFaxServer2 *pFaxServer);
        
        DECLSPEC_XFGVIRT(_IFaxServerNotify2, OnEventLoggingConfigChange)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *OnEventLoggingConfigChange )( 
            __RPC__in _IFaxServerNotify2 * This,
            /* [in] */ __RPC__in_opt IFaxServer2 *pFaxServer);
        
        DECLSPEC_XFGVIRT(_IFaxServerNotify2, OnOutgoingQueueConfigChange)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *OnOutgoingQueueConfigChange )( 
            __RPC__in _IFaxServerNotify2 * This,
            /* [in] */ __RPC__in_opt IFaxServer2 *pFaxServer);
        
        DECLSPEC_XFGVIRT(_IFaxServerNotify2, OnOutgoingArchiveConfigChange)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *OnOutgoingArchiveConfigChange )( 
            __RPC__in _IFaxServerNotify2 * This,
            /* [in] */ __RPC__in_opt IFaxServer2 *pFaxServer);
        
        DECLSPEC_XFGVIRT(_IFaxServerNotify2, OnIncomingArchiveConfigChange)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *OnIncomingArchiveConfigChange )( 
            __RPC__in _IFaxServerNotify2 * This,
            /* [in] */ __RPC__in_opt IFaxServer2 *pFaxServer);
        
        DECLSPEC_XFGVIRT(_IFaxServerNotify2, OnDevicesConfigChange)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *OnDevicesConfigChange )( 
            __RPC__in _IFaxServerNotify2 * This,
            /* [in] */ __RPC__in_opt IFaxServer2 *pFaxServer);
        
        DECLSPEC_XFGVIRT(_IFaxServerNotify2, OnOutboundRoutingGroupsConfigChange)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *OnOutboundRoutingGroupsConfigChange )( 
            __RPC__in _IFaxServerNotify2 * This,
            /* [in] */ __RPC__in_opt IFaxServer2 *pFaxServer);
        
        DECLSPEC_XFGVIRT(_IFaxServerNotify2, OnOutboundRoutingRulesConfigChange)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *OnOutboundRoutingRulesConfigChange )( 
            __RPC__in _IFaxServerNotify2 * This,
            /* [in] */ __RPC__in_opt IFaxServer2 *pFaxServer);
        
        DECLSPEC_XFGVIRT(_IFaxServerNotify2, OnServerActivityChange)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *OnServerActivityChange )( 
            __RPC__in _IFaxServerNotify2 * This,
            /* [in] */ __RPC__in_opt IFaxServer2 *pFaxServer,
            /* [in] */ long lIncomingMessages,
            /* [in] */ long lRoutingMessages,
            /* [in] */ long lOutgoingMessages,
            /* [in] */ long lQueuedMessages);
        
        DECLSPEC_XFGVIRT(_IFaxServerNotify2, OnQueuesStatusChange)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *OnQueuesStatusChange )( 
            __RPC__in _IFaxServerNotify2 * This,
            /* [in] */ __RPC__in_opt IFaxServer2 *pFaxServer,
            /* [in] */ VARIANT_BOOL bOutgoingQueueBlocked,
            /* [in] */ VARIANT_BOOL bOutgoingQueuePaused,
            /* [in] */ VARIANT_BOOL bIncomingQueueBlocked);
        
        DECLSPEC_XFGVIRT(_IFaxServerNotify2, OnNewCall)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *OnNewCall )( 
            __RPC__in _IFaxServerNotify2 * This,
            /* [in] */ __RPC__in_opt IFaxServer2 *pFaxServer,
            /* [in] */ long lCallId,
            /* [in] */ long lDeviceId,
            /* [in] */ __RPC__in BSTR bstrCallerId);
        
        DECLSPEC_XFGVIRT(_IFaxServerNotify2, OnServerShutDown)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *OnServerShutDown )( 
            __RPC__in _IFaxServerNotify2 * This,
            /* [in] */ __RPC__in_opt IFaxServer2 *pFaxServer);
        
        DECLSPEC_XFGVIRT(_IFaxServerNotify2, OnDeviceStatusChange)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *OnDeviceStatusChange )( 
            __RPC__in _IFaxServerNotify2 * This,
            /* [in] */ __RPC__in_opt IFaxServer2 *pFaxServer,
            /* [in] */ long lDeviceId,
            /* [in] */ VARIANT_BOOL bPoweredOff,
            /* [in] */ VARIANT_BOOL bSending,
            /* [in] */ VARIANT_BOOL bReceiving,
            /* [in] */ VARIANT_BOOL bRinging);
        
        DECLSPEC_XFGVIRT(_IFaxServerNotify2, OnGeneralServerConfigChanged)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *OnGeneralServerConfigChanged )( 
            __RPC__in _IFaxServerNotify2 * This,
            /* [in] */ __RPC__in_opt IFaxServer2 *pFaxServer);
        
        END_INTERFACE
    } _IFaxServerNotify2Vtbl;

    interface _IFaxServerNotify2
    {
        CONST_VTBL struct _IFaxServerNotify2Vtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define _IFaxServerNotify2_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define _IFaxServerNotify2_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define _IFaxServerNotify2_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define _IFaxServerNotify2_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define _IFaxServerNotify2_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define _IFaxServerNotify2_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define _IFaxServerNotify2_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define _IFaxServerNotify2_OnIncomingJobAdded(This,pFaxServer,bstrJobId)	\
    ( (This)->lpVtbl -> OnIncomingJobAdded(This,pFaxServer,bstrJobId) ) 

#define _IFaxServerNotify2_OnIncomingJobRemoved(This,pFaxServer,bstrJobId)	\
    ( (This)->lpVtbl -> OnIncomingJobRemoved(This,pFaxServer,bstrJobId) ) 

#define _IFaxServerNotify2_OnIncomingJobChanged(This,pFaxServer,bstrJobId,pJobStatus)	\
    ( (This)->lpVtbl -> OnIncomingJobChanged(This,pFaxServer,bstrJobId,pJobStatus) ) 

#define _IFaxServerNotify2_OnOutgoingJobAdded(This,pFaxServer,bstrJobId)	\
    ( (This)->lpVtbl -> OnOutgoingJobAdded(This,pFaxServer,bstrJobId) ) 

#define _IFaxServerNotify2_OnOutgoingJobRemoved(This,pFaxServer,bstrJobId)	\
    ( (This)->lpVtbl -> OnOutgoingJobRemoved(This,pFaxServer,bstrJobId) ) 

#define _IFaxServerNotify2_OnOutgoingJobChanged(This,pFaxServer,bstrJobId,pJobStatus)	\
    ( (This)->lpVtbl -> OnOutgoingJobChanged(This,pFaxServer,bstrJobId,pJobStatus) ) 

#define _IFaxServerNotify2_OnIncomingMessageAdded(This,pFaxServer,bstrMessageId)	\
    ( (This)->lpVtbl -> OnIncomingMessageAdded(This,pFaxServer,bstrMessageId) ) 

#define _IFaxServerNotify2_OnIncomingMessageRemoved(This,pFaxServer,bstrMessageId)	\
    ( (This)->lpVtbl -> OnIncomingMessageRemoved(This,pFaxServer,bstrMessageId) ) 

#define _IFaxServerNotify2_OnOutgoingMessageAdded(This,pFaxServer,bstrMessageId)	\
    ( (This)->lpVtbl -> OnOutgoingMessageAdded(This,pFaxServer,bstrMessageId) ) 

#define _IFaxServerNotify2_OnOutgoingMessageRemoved(This,pFaxServer,bstrMessageId)	\
    ( (This)->lpVtbl -> OnOutgoingMessageRemoved(This,pFaxServer,bstrMessageId) ) 

#define _IFaxServerNotify2_OnReceiptOptionsChange(This,pFaxServer)	\
    ( (This)->lpVtbl -> OnReceiptOptionsChange(This,pFaxServer) ) 

#define _IFaxServerNotify2_OnActivityLoggingConfigChange(This,pFaxServer)	\
    ( (This)->lpVtbl -> OnActivityLoggingConfigChange(This,pFaxServer) ) 

#define _IFaxServerNotify2_OnSecurityConfigChange(This,pFaxServer)	\
    ( (This)->lpVtbl -> OnSecurityConfigChange(This,pFaxServer) ) 

#define _IFaxServerNotify2_OnEventLoggingConfigChange(This,pFaxServer)	\
    ( (This)->lpVtbl -> OnEventLoggingConfigChange(This,pFaxServer) ) 

#define _IFaxServerNotify2_OnOutgoingQueueConfigChange(This,pFaxServer)	\
    ( (This)->lpVtbl -> OnOutgoingQueueConfigChange(This,pFaxServer) ) 

#define _IFaxServerNotify2_OnOutgoingArchiveConfigChange(This,pFaxServer)	\
    ( (This)->lpVtbl -> OnOutgoingArchiveConfigChange(This,pFaxServer) ) 

#define _IFaxServerNotify2_OnIncomingArchiveConfigChange(This,pFaxServer)	\
    ( (This)->lpVtbl -> OnIncomingArchiveConfigChange(This,pFaxServer) ) 

#define _IFaxServerNotify2_OnDevicesConfigChange(This,pFaxServer)	\
    ( (This)->lpVtbl -> OnDevicesConfigChange(This,pFaxServer) ) 

#define _IFaxServerNotify2_OnOutboundRoutingGroupsConfigChange(This,pFaxServer)	\
    ( (This)->lpVtbl -> OnOutboundRoutingGroupsConfigChange(This,pFaxServer) ) 

#define _IFaxServerNotify2_OnOutboundRoutingRulesConfigChange(This,pFaxServer)	\
    ( (This)->lpVtbl -> OnOutboundRoutingRulesConfigChange(This,pFaxServer) ) 

#define _IFaxServerNotify2_OnServerActivityChange(This,pFaxServer,lIncomingMessages,lRoutingMessages,lOutgoingMessages,lQueuedMessages)	\
    ( (This)->lpVtbl -> OnServerActivityChange(This,pFaxServer,lIncomingMessages,lRoutingMessages,lOutgoingMessages,lQueuedMessages) ) 

#define _IFaxServerNotify2_OnQueuesStatusChange(This,pFaxServer,bOutgoingQueueBlocked,bOutgoingQueuePaused,bIncomingQueueBlocked)	\
    ( (This)->lpVtbl -> OnQueuesStatusChange(This,pFaxServer,bOutgoingQueueBlocked,bOutgoingQueuePaused,bIncomingQueueBlocked) ) 

#define _IFaxServerNotify2_OnNewCall(This,pFaxServer,lCallId,lDeviceId,bstrCallerId)	\
    ( (This)->lpVtbl -> OnNewCall(This,pFaxServer,lCallId,lDeviceId,bstrCallerId) ) 

#define _IFaxServerNotify2_OnServerShutDown(This,pFaxServer)	\
    ( (This)->lpVtbl -> OnServerShutDown(This,pFaxServer) ) 

#define _IFaxServerNotify2_OnDeviceStatusChange(This,pFaxServer,lDeviceId,bPoweredOff,bSending,bReceiving,bRinging)	\
    ( (This)->lpVtbl -> OnDeviceStatusChange(This,pFaxServer,lDeviceId,bPoweredOff,bSending,bReceiving,bRinging) ) 

#define _IFaxServerNotify2_OnGeneralServerConfigChanged(This,pFaxServer)	\
    ( (This)->lpVtbl -> OnGeneralServerConfigChanged(This,pFaxServer) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* ___IFaxServerNotify2_INTERFACE_DEFINED__ */


#ifndef __IFaxServerNotify2_DISPINTERFACE_DEFINED__
#define __IFaxServerNotify2_DISPINTERFACE_DEFINED__

/* dispinterface IFaxServerNotify2 */
/* [helpstring][uuid] */ 


EXTERN_C const IID DIID_IFaxServerNotify2;

#if defined(__cplusplus) && !defined(CINTERFACE)

    MIDL_INTERFACE("616ca8d6-a77a-4062-abfd-0e471241c7aa")
    IFaxServerNotify2 : public IDispatch
    {
    };
    
#else 	/* C style interface */

    typedef struct IFaxServerNotify2Vtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IFaxServerNotify2 * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IFaxServerNotify2 * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IFaxServerNotify2 * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IFaxServerNotify2 * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IFaxServerNotify2 * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IFaxServerNotify2 * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IFaxServerNotify2 * This,
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
    } IFaxServerNotify2Vtbl;

    interface IFaxServerNotify2
    {
        CONST_VTBL struct IFaxServerNotify2Vtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IFaxServerNotify2_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IFaxServerNotify2_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IFaxServerNotify2_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IFaxServerNotify2_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IFaxServerNotify2_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IFaxServerNotify2_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IFaxServerNotify2_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */


#endif 	/* __IFaxServerNotify2_DISPINTERFACE_DEFINED__ */


#ifndef ___IFaxAccountNotify_INTERFACE_DEFINED__
#define ___IFaxAccountNotify_INTERFACE_DEFINED__

/* interface _IFaxAccountNotify */
/* [unique][uuid][object] */ 


EXTERN_C const IID IID__IFaxAccountNotify;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("b9b3bc81-ac1b-46f3-b39d-0adc30e1b788")
    _IFaxAccountNotify : public IDispatch
    {
    public:
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE OnIncomingJobAdded( 
            /* [in] */ __RPC__in_opt IFaxAccount *pFaxAccount,
            /* [in] */ __RPC__in BSTR bstrJobId) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE OnIncomingJobRemoved( 
            /* [in] */ __RPC__in_opt IFaxAccount *pFaxAccount,
            /* [in] */ __RPC__in BSTR bstrJobId) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE OnIncomingJobChanged( 
            /* [in] */ __RPC__in_opt IFaxAccount *pFaxAccount,
            /* [in] */ __RPC__in BSTR bstrJobId,
            /* [in] */ __RPC__in_opt IFaxJobStatus *pJobStatus) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE OnOutgoingJobAdded( 
            /* [in] */ __RPC__in_opt IFaxAccount *pFaxAccount,
            /* [in] */ __RPC__in BSTR bstrJobId) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE OnOutgoingJobRemoved( 
            /* [in] */ __RPC__in_opt IFaxAccount *pFaxAccount,
            /* [in] */ __RPC__in BSTR bstrJobId) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE OnOutgoingJobChanged( 
            /* [in] */ __RPC__in_opt IFaxAccount *pFaxAccount,
            /* [in] */ __RPC__in BSTR bstrJobId,
            /* [in] */ __RPC__in_opt IFaxJobStatus *pJobStatus) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE OnIncomingMessageAdded( 
            /* [in] */ __RPC__in_opt IFaxAccount *pFaxAccount,
            /* [in] */ __RPC__in BSTR bstrMessageId,
            /* [in] */ VARIANT_BOOL fAddedToReceiveFolder) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE OnIncomingMessageRemoved( 
            /* [in] */ __RPC__in_opt IFaxAccount *pFaxAccount,
            /* [in] */ __RPC__in BSTR bstrMessageId,
            /* [in] */ VARIANT_BOOL fRemovedFromReceiveFolder) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE OnOutgoingMessageAdded( 
            /* [in] */ __RPC__in_opt IFaxAccount *pFaxAccount,
            /* [in] */ __RPC__in BSTR bstrMessageId) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE OnOutgoingMessageRemoved( 
            /* [in] */ __RPC__in_opt IFaxAccount *pFaxAccount,
            /* [in] */ __RPC__in BSTR bstrMessageId) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE OnServerShutDown( 
            /* [in] */ __RPC__in_opt IFaxServer2 *pFaxServer) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct _IFaxAccountNotifyVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in _IFaxAccountNotify * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in _IFaxAccountNotify * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in _IFaxAccountNotify * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in _IFaxAccountNotify * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in _IFaxAccountNotify * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in _IFaxAccountNotify * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            _IFaxAccountNotify * This,
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
        
        DECLSPEC_XFGVIRT(_IFaxAccountNotify, OnIncomingJobAdded)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *OnIncomingJobAdded )( 
            __RPC__in _IFaxAccountNotify * This,
            /* [in] */ __RPC__in_opt IFaxAccount *pFaxAccount,
            /* [in] */ __RPC__in BSTR bstrJobId);
        
        DECLSPEC_XFGVIRT(_IFaxAccountNotify, OnIncomingJobRemoved)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *OnIncomingJobRemoved )( 
            __RPC__in _IFaxAccountNotify * This,
            /* [in] */ __RPC__in_opt IFaxAccount *pFaxAccount,
            /* [in] */ __RPC__in BSTR bstrJobId);
        
        DECLSPEC_XFGVIRT(_IFaxAccountNotify, OnIncomingJobChanged)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *OnIncomingJobChanged )( 
            __RPC__in _IFaxAccountNotify * This,
            /* [in] */ __RPC__in_opt IFaxAccount *pFaxAccount,
            /* [in] */ __RPC__in BSTR bstrJobId,
            /* [in] */ __RPC__in_opt IFaxJobStatus *pJobStatus);
        
        DECLSPEC_XFGVIRT(_IFaxAccountNotify, OnOutgoingJobAdded)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *OnOutgoingJobAdded )( 
            __RPC__in _IFaxAccountNotify * This,
            /* [in] */ __RPC__in_opt IFaxAccount *pFaxAccount,
            /* [in] */ __RPC__in BSTR bstrJobId);
        
        DECLSPEC_XFGVIRT(_IFaxAccountNotify, OnOutgoingJobRemoved)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *OnOutgoingJobRemoved )( 
            __RPC__in _IFaxAccountNotify * This,
            /* [in] */ __RPC__in_opt IFaxAccount *pFaxAccount,
            /* [in] */ __RPC__in BSTR bstrJobId);
        
        DECLSPEC_XFGVIRT(_IFaxAccountNotify, OnOutgoingJobChanged)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *OnOutgoingJobChanged )( 
            __RPC__in _IFaxAccountNotify * This,
            /* [in] */ __RPC__in_opt IFaxAccount *pFaxAccount,
            /* [in] */ __RPC__in BSTR bstrJobId,
            /* [in] */ __RPC__in_opt IFaxJobStatus *pJobStatus);
        
        DECLSPEC_XFGVIRT(_IFaxAccountNotify, OnIncomingMessageAdded)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *OnIncomingMessageAdded )( 
            __RPC__in _IFaxAccountNotify * This,
            /* [in] */ __RPC__in_opt IFaxAccount *pFaxAccount,
            /* [in] */ __RPC__in BSTR bstrMessageId,
            /* [in] */ VARIANT_BOOL fAddedToReceiveFolder);
        
        DECLSPEC_XFGVIRT(_IFaxAccountNotify, OnIncomingMessageRemoved)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *OnIncomingMessageRemoved )( 
            __RPC__in _IFaxAccountNotify * This,
            /* [in] */ __RPC__in_opt IFaxAccount *pFaxAccount,
            /* [in] */ __RPC__in BSTR bstrMessageId,
            /* [in] */ VARIANT_BOOL fRemovedFromReceiveFolder);
        
        DECLSPEC_XFGVIRT(_IFaxAccountNotify, OnOutgoingMessageAdded)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *OnOutgoingMessageAdded )( 
            __RPC__in _IFaxAccountNotify * This,
            /* [in] */ __RPC__in_opt IFaxAccount *pFaxAccount,
            /* [in] */ __RPC__in BSTR bstrMessageId);
        
        DECLSPEC_XFGVIRT(_IFaxAccountNotify, OnOutgoingMessageRemoved)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *OnOutgoingMessageRemoved )( 
            __RPC__in _IFaxAccountNotify * This,
            /* [in] */ __RPC__in_opt IFaxAccount *pFaxAccount,
            /* [in] */ __RPC__in BSTR bstrMessageId);
        
        DECLSPEC_XFGVIRT(_IFaxAccountNotify, OnServerShutDown)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *OnServerShutDown )( 
            __RPC__in _IFaxAccountNotify * This,
            /* [in] */ __RPC__in_opt IFaxServer2 *pFaxServer);
        
        END_INTERFACE
    } _IFaxAccountNotifyVtbl;

    interface _IFaxAccountNotify
    {
        CONST_VTBL struct _IFaxAccountNotifyVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define _IFaxAccountNotify_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define _IFaxAccountNotify_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define _IFaxAccountNotify_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define _IFaxAccountNotify_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define _IFaxAccountNotify_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define _IFaxAccountNotify_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define _IFaxAccountNotify_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define _IFaxAccountNotify_OnIncomingJobAdded(This,pFaxAccount,bstrJobId)	\
    ( (This)->lpVtbl -> OnIncomingJobAdded(This,pFaxAccount,bstrJobId) ) 

#define _IFaxAccountNotify_OnIncomingJobRemoved(This,pFaxAccount,bstrJobId)	\
    ( (This)->lpVtbl -> OnIncomingJobRemoved(This,pFaxAccount,bstrJobId) ) 

#define _IFaxAccountNotify_OnIncomingJobChanged(This,pFaxAccount,bstrJobId,pJobStatus)	\
    ( (This)->lpVtbl -> OnIncomingJobChanged(This,pFaxAccount,bstrJobId,pJobStatus) ) 

#define _IFaxAccountNotify_OnOutgoingJobAdded(This,pFaxAccount,bstrJobId)	\
    ( (This)->lpVtbl -> OnOutgoingJobAdded(This,pFaxAccount,bstrJobId) ) 

#define _IFaxAccountNotify_OnOutgoingJobRemoved(This,pFaxAccount,bstrJobId)	\
    ( (This)->lpVtbl -> OnOutgoingJobRemoved(This,pFaxAccount,bstrJobId) ) 

#define _IFaxAccountNotify_OnOutgoingJobChanged(This,pFaxAccount,bstrJobId,pJobStatus)	\
    ( (This)->lpVtbl -> OnOutgoingJobChanged(This,pFaxAccount,bstrJobId,pJobStatus) ) 

#define _IFaxAccountNotify_OnIncomingMessageAdded(This,pFaxAccount,bstrMessageId,fAddedToReceiveFolder)	\
    ( (This)->lpVtbl -> OnIncomingMessageAdded(This,pFaxAccount,bstrMessageId,fAddedToReceiveFolder) ) 

#define _IFaxAccountNotify_OnIncomingMessageRemoved(This,pFaxAccount,bstrMessageId,fRemovedFromReceiveFolder)	\
    ( (This)->lpVtbl -> OnIncomingMessageRemoved(This,pFaxAccount,bstrMessageId,fRemovedFromReceiveFolder) ) 

#define _IFaxAccountNotify_OnOutgoingMessageAdded(This,pFaxAccount,bstrMessageId)	\
    ( (This)->lpVtbl -> OnOutgoingMessageAdded(This,pFaxAccount,bstrMessageId) ) 

#define _IFaxAccountNotify_OnOutgoingMessageRemoved(This,pFaxAccount,bstrMessageId)	\
    ( (This)->lpVtbl -> OnOutgoingMessageRemoved(This,pFaxAccount,bstrMessageId) ) 

#define _IFaxAccountNotify_OnServerShutDown(This,pFaxServer)	\
    ( (This)->lpVtbl -> OnServerShutDown(This,pFaxServer) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* ___IFaxAccountNotify_INTERFACE_DEFINED__ */


#ifndef __IFaxAccountNotify_DISPINTERFACE_DEFINED__
#define __IFaxAccountNotify_DISPINTERFACE_DEFINED__

/* dispinterface IFaxAccountNotify */
/* [helpstring][uuid] */ 


EXTERN_C const IID DIID_IFaxAccountNotify;

#if defined(__cplusplus) && !defined(CINTERFACE)

    MIDL_INTERFACE("0b5e5bd1-b8a9-47a0-a323-ef4a293ba06a")
    IFaxAccountNotify : public IDispatch
    {
    };
    
#else 	/* C style interface */

    typedef struct IFaxAccountNotifyVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IFaxAccountNotify * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IFaxAccountNotify * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IFaxAccountNotify * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IFaxAccountNotify * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IFaxAccountNotify * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IFaxAccountNotify * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IFaxAccountNotify * This,
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
    } IFaxAccountNotifyVtbl;

    interface IFaxAccountNotify
    {
        CONST_VTBL struct IFaxAccountNotifyVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IFaxAccountNotify_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IFaxAccountNotify_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IFaxAccountNotify_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IFaxAccountNotify_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IFaxAccountNotify_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IFaxAccountNotify_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IFaxAccountNotify_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */


#endif 	/* __IFaxAccountNotify_DISPINTERFACE_DEFINED__ */


EXTERN_C const CLSID CLSID_FaxServer;

#ifdef __cplusplus

class DECLSPEC_UUID("CDA8ACB0-8CF5-4F6C-9BA2-5931D40C8CAE")
FaxServer;
#endif

EXTERN_C const CLSID CLSID_FaxDeviceProviders;

#ifdef __cplusplus

class DECLSPEC_UUID("EB8FE768-875A-4F5F-82C5-03F23AAC1BD7")
FaxDeviceProviders;
#endif

EXTERN_C const CLSID CLSID_FaxDevices;

#ifdef __cplusplus

class DECLSPEC_UUID("5589E28E-23CB-4919-8808-E6101846E80D")
FaxDevices;
#endif

EXTERN_C const CLSID CLSID_FaxInboundRouting;

#ifdef __cplusplus

class DECLSPEC_UUID("E80248ED-AD65-4218-8108-991924D4E7ED")
FaxInboundRouting;
#endif

EXTERN_C const CLSID CLSID_FaxFolders;

#ifdef __cplusplus

class DECLSPEC_UUID("C35211D7-5776-48CB-AF44-C31BE3B2CFE5")
FaxFolders;
#endif

EXTERN_C const CLSID CLSID_FaxLoggingOptions;

#ifdef __cplusplus

class DECLSPEC_UUID("1BF9EEA6-ECE0-4785-A18B-DE56E9EEF96A")
FaxLoggingOptions;
#endif

EXTERN_C const CLSID CLSID_FaxActivity;

#ifdef __cplusplus

class DECLSPEC_UUID("CFEF5D0E-E84D-462E-AABB-87D31EB04FEF")
FaxActivity;
#endif

EXTERN_C const CLSID CLSID_FaxOutboundRouting;

#ifdef __cplusplus

class DECLSPEC_UUID("C81B385E-B869-4AFD-86C0-616498ED9BE2")
FaxOutboundRouting;
#endif

EXTERN_C const CLSID CLSID_FaxReceiptOptions;

#ifdef __cplusplus

class DECLSPEC_UUID("6982487B-227B-4C96-A61C-248348B05AB6")
FaxReceiptOptions;
#endif

EXTERN_C const CLSID CLSID_FaxSecurity;

#ifdef __cplusplus

class DECLSPEC_UUID("10C4DDDE-ABF0-43DF-964F-7F3AC21A4C7B")
FaxSecurity;
#endif

EXTERN_C const CLSID CLSID_FaxDocument;

#ifdef __cplusplus

class DECLSPEC_UUID("0F3F9F91-C838-415E-A4F3-3E828CA445E0")
FaxDocument;
#endif

EXTERN_C const CLSID CLSID_FaxSender;

#ifdef __cplusplus

class DECLSPEC_UUID("265D84D0-1850-4360-B7C8-758BBB5F0B96")
FaxSender;
#endif

EXTERN_C const CLSID CLSID_FaxRecipients;

#ifdef __cplusplus

class DECLSPEC_UUID("EA9BDF53-10A9-4D4F-A067-63C8F84F01B0")
FaxRecipients;
#endif

EXTERN_C const CLSID CLSID_FaxIncomingArchive;

#ifdef __cplusplus

class DECLSPEC_UUID("8426C56A-35A1-4C6F-AF93-FC952422E2C2")
FaxIncomingArchive;
#endif

EXTERN_C const CLSID CLSID_FaxIncomingQueue;

#ifdef __cplusplus

class DECLSPEC_UUID("69131717-F3F1-40E3-809D-A6CBF7BD85E5")
FaxIncomingQueue;
#endif

EXTERN_C const CLSID CLSID_FaxOutgoingArchive;

#ifdef __cplusplus

class DECLSPEC_UUID("43C28403-E04F-474D-990C-B94669148F59")
FaxOutgoingArchive;
#endif

EXTERN_C const CLSID CLSID_FaxOutgoingQueue;

#ifdef __cplusplus

class DECLSPEC_UUID("7421169E-8C43-4B0D-BB16-645C8FA40357")
FaxOutgoingQueue;
#endif

EXTERN_C const CLSID CLSID_FaxIncomingMessageIterator;

#ifdef __cplusplus

class DECLSPEC_UUID("6088E1D8-3FC8-45C2-87B1-909A29607EA9")
FaxIncomingMessageIterator;
#endif

EXTERN_C const CLSID CLSID_FaxIncomingMessage;

#ifdef __cplusplus

class DECLSPEC_UUID("1932FCF7-9D43-4D5A-89FF-03861B321736")
FaxIncomingMessage;
#endif

EXTERN_C const CLSID CLSID_FaxOutgoingJobs;

#ifdef __cplusplus

class DECLSPEC_UUID("92BF2A6C-37BE-43FA-A37D-CB0E5F753B35")
FaxOutgoingJobs;
#endif

EXTERN_C const CLSID CLSID_FaxOutgoingJob;

#ifdef __cplusplus

class DECLSPEC_UUID("71BB429C-0EF9-4915-BEC5-A5D897A3E924")
FaxOutgoingJob;
#endif

EXTERN_C const CLSID CLSID_FaxOutgoingMessageIterator;

#ifdef __cplusplus

class DECLSPEC_UUID("8A3224D0-D30B-49DE-9813-CB385790FBBB")
FaxOutgoingMessageIterator;
#endif

EXTERN_C const CLSID CLSID_FaxOutgoingMessage;

#ifdef __cplusplus

class DECLSPEC_UUID("91B4A378-4AD8-4AEF-A4DC-97D96E939A3A")
FaxOutgoingMessage;
#endif

EXTERN_C const CLSID CLSID_FaxIncomingJobs;

#ifdef __cplusplus

class DECLSPEC_UUID("A1BB8A43-8866-4FB7-A15D-6266C875A5CC")
FaxIncomingJobs;
#endif

EXTERN_C const CLSID CLSID_FaxIncomingJob;

#ifdef __cplusplus

class DECLSPEC_UUID("C47311EC-AE32-41B8-AE4B-3EAE0629D0C9")
FaxIncomingJob;
#endif

EXTERN_C const CLSID CLSID_FaxDeviceProvider;

#ifdef __cplusplus

class DECLSPEC_UUID("17CF1AA3-F5EB-484A-9C9A-4440A5BAABFC")
FaxDeviceProvider;
#endif

EXTERN_C const CLSID CLSID_FaxDevice;

#ifdef __cplusplus

class DECLSPEC_UUID("59E3A5B2-D676-484B-A6DE-720BFA89B5AF")
FaxDevice;
#endif

EXTERN_C const CLSID CLSID_FaxActivityLogging;

#ifdef __cplusplus

class DECLSPEC_UUID("F0A0294E-3BBD-48B8-8F13-8C591A55BDBC")
FaxActivityLogging;
#endif

EXTERN_C const CLSID CLSID_FaxEventLogging;

#ifdef __cplusplus

class DECLSPEC_UUID("A6850930-A0F6-4A6F-95B7-DB2EBF3D02E3")
FaxEventLogging;
#endif

EXTERN_C const CLSID CLSID_FaxOutboundRoutingGroups;

#ifdef __cplusplus

class DECLSPEC_UUID("CCBEA1A5-E2B4-4B57-9421-B04B6289464B")
FaxOutboundRoutingGroups;
#endif

EXTERN_C const CLSID CLSID_FaxOutboundRoutingGroup;

#ifdef __cplusplus

class DECLSPEC_UUID("0213F3E0-6791-4D77-A271-04D2357C50D6")
FaxOutboundRoutingGroup;
#endif

EXTERN_C const CLSID CLSID_FaxDeviceIds;

#ifdef __cplusplus

class DECLSPEC_UUID("CDC539EA-7277-460E-8DE0-48A0A5760D1F")
FaxDeviceIds;
#endif

EXTERN_C const CLSID CLSID_FaxOutboundRoutingRules;

#ifdef __cplusplus

class DECLSPEC_UUID("D385BECA-E624-4473-BFAA-9F4000831F54")
FaxOutboundRoutingRules;
#endif

EXTERN_C const CLSID CLSID_FaxOutboundRoutingRule;

#ifdef __cplusplus

class DECLSPEC_UUID("6549EEBF-08D1-475A-828B-3BF105952FA0")
FaxOutboundRoutingRule;
#endif

EXTERN_C const CLSID CLSID_FaxInboundRoutingExtensions;

#ifdef __cplusplus

class DECLSPEC_UUID("189A48ED-623C-4C0D-80F2-D66C7B9EFEC2")
FaxInboundRoutingExtensions;
#endif

EXTERN_C const CLSID CLSID_FaxInboundRoutingExtension;

#ifdef __cplusplus

class DECLSPEC_UUID("1D7DFB51-7207-4436-A0D9-24E32EE56988")
FaxInboundRoutingExtension;
#endif

EXTERN_C const CLSID CLSID_FaxInboundRoutingMethods;

#ifdef __cplusplus

class DECLSPEC_UUID("25FCB76A-B750-4B82-9266-FBBBAE8922BA")
FaxInboundRoutingMethods;
#endif

EXTERN_C const CLSID CLSID_FaxInboundRoutingMethod;

#ifdef __cplusplus

class DECLSPEC_UUID("4B9FD75C-0194-4B72-9CE5-02A8205AC7D4")
FaxInboundRoutingMethod;
#endif

EXTERN_C const CLSID CLSID_FaxJobStatus;

#ifdef __cplusplus

class DECLSPEC_UUID("7BF222F4-BE8D-442f-841D-6132742423BB")
FaxJobStatus;
#endif

EXTERN_C const CLSID CLSID_FaxRecipient;

#ifdef __cplusplus

class DECLSPEC_UUID("60BF3301-7DF8-4bd8-9148-7B5801F9EFDF")
FaxRecipient;
#endif

EXTERN_C const CLSID CLSID_FaxConfiguration;

#ifdef __cplusplus

class DECLSPEC_UUID("5857326f-e7b3-41a7-9c19-a91b463e2d56")
FaxConfiguration;
#endif

EXTERN_C const CLSID CLSID_FaxAccountSet;

#ifdef __cplusplus

class DECLSPEC_UUID("fbc23c4b-79e0-4291-bc56-c12e253bbf3a")
FaxAccountSet;
#endif

EXTERN_C const CLSID CLSID_FaxAccounts;

#ifdef __cplusplus

class DECLSPEC_UUID("da1f94aa-ee2c-47c0-8f4f-2a217075b76e")
FaxAccounts;
#endif

EXTERN_C const CLSID CLSID_FaxAccount;

#ifdef __cplusplus

class DECLSPEC_UUID("a7e0647f-4524-4464-a56d-b9fe666f715e")
FaxAccount;
#endif

EXTERN_C const CLSID CLSID_FaxAccountFolders;

#ifdef __cplusplus

class DECLSPEC_UUID("85398f49-c034-4a3f-821c-db7d685e8129")
FaxAccountFolders;
#endif

EXTERN_C const CLSID CLSID_FaxAccountIncomingQueue;

#ifdef __cplusplus

class DECLSPEC_UUID("9bcf6094-b4da-45f4-b8d6-ddeb2186652c")
FaxAccountIncomingQueue;
#endif

EXTERN_C const CLSID CLSID_FaxAccountOutgoingQueue;

#ifdef __cplusplus

class DECLSPEC_UUID("feeceefb-c149-48ba-bab8-b791e101f62f")
FaxAccountOutgoingQueue;
#endif

EXTERN_C const CLSID CLSID_FaxAccountIncomingArchive;

#ifdef __cplusplus

class DECLSPEC_UUID("14b33db5-4c40-4ecf-9ef8-a360cbe809ed")
FaxAccountIncomingArchive;
#endif

EXTERN_C const CLSID CLSID_FaxAccountOutgoingArchive;

#ifdef __cplusplus

class DECLSPEC_UUID("851e7af5-433a-4739-a2df-ad245c2cb98e")
FaxAccountOutgoingArchive;
#endif

EXTERN_C const CLSID CLSID_FaxSecurity2;

#ifdef __cplusplus

class DECLSPEC_UUID("735c1248-ec89-4c30-a127-656e92e3c4ea")
FaxSecurity2;
#endif


#ifndef __FaxConstants_MODULE_DEFINED__
#define __FaxConstants_MODULE_DEFINED__


/* module FaxConstants */
/* [dllname] */ 

/* [helpstring] */ const long lDEFAULT_PREFETCH_SIZE	=	prv_DEFAULT_PREFETCH_SIZE;

/* [helpstring] */ const BSTR bstrGROUPNAME_ALLDEVICES	=	( BSTR  )L"<All Devices>";

/* [helpstring] */ const wchar_t wcharREASSIGN_RECIPIENTS_DELIMITER	=	L';';

#endif /* __FaxConstants_MODULE_DEFINED__ */
#endif /* __FAXCOMEXLib_LIBRARY_DEFINED__ */

/* interface __MIDL_itf_faxcomex_0000_0056 */
/* [local] */ 

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion


extern RPC_IF_HANDLE __MIDL_itf_faxcomex_0000_0056_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_faxcomex_0000_0056_v0_0_s_ifspec;

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


