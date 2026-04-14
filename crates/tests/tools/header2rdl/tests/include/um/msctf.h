

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

#ifndef __msctf_h__
#define __msctf_h__

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

#ifndef __ITfThreadMgr_FWD_DEFINED__
#define __ITfThreadMgr_FWD_DEFINED__
typedef interface ITfThreadMgr ITfThreadMgr;

#endif 	/* __ITfThreadMgr_FWD_DEFINED__ */


#ifndef __ITfThreadMgrEx_FWD_DEFINED__
#define __ITfThreadMgrEx_FWD_DEFINED__
typedef interface ITfThreadMgrEx ITfThreadMgrEx;

#endif 	/* __ITfThreadMgrEx_FWD_DEFINED__ */


#ifndef __ITfThreadMgr2_FWD_DEFINED__
#define __ITfThreadMgr2_FWD_DEFINED__
typedef interface ITfThreadMgr2 ITfThreadMgr2;

#endif 	/* __ITfThreadMgr2_FWD_DEFINED__ */


#ifndef __ITfThreadMgrEventSink_FWD_DEFINED__
#define __ITfThreadMgrEventSink_FWD_DEFINED__
typedef interface ITfThreadMgrEventSink ITfThreadMgrEventSink;

#endif 	/* __ITfThreadMgrEventSink_FWD_DEFINED__ */


#ifndef __ITfConfigureSystemKeystrokeFeed_FWD_DEFINED__
#define __ITfConfigureSystemKeystrokeFeed_FWD_DEFINED__
typedef interface ITfConfigureSystemKeystrokeFeed ITfConfigureSystemKeystrokeFeed;

#endif 	/* __ITfConfigureSystemKeystrokeFeed_FWD_DEFINED__ */


#ifndef __IEnumTfDocumentMgrs_FWD_DEFINED__
#define __IEnumTfDocumentMgrs_FWD_DEFINED__
typedef interface IEnumTfDocumentMgrs IEnumTfDocumentMgrs;

#endif 	/* __IEnumTfDocumentMgrs_FWD_DEFINED__ */


#ifndef __ITfDocumentMgr_FWD_DEFINED__
#define __ITfDocumentMgr_FWD_DEFINED__
typedef interface ITfDocumentMgr ITfDocumentMgr;

#endif 	/* __ITfDocumentMgr_FWD_DEFINED__ */


#ifndef __IEnumTfContexts_FWD_DEFINED__
#define __IEnumTfContexts_FWD_DEFINED__
typedef interface IEnumTfContexts IEnumTfContexts;

#endif 	/* __IEnumTfContexts_FWD_DEFINED__ */


#ifndef __ITfCompositionView_FWD_DEFINED__
#define __ITfCompositionView_FWD_DEFINED__
typedef interface ITfCompositionView ITfCompositionView;

#endif 	/* __ITfCompositionView_FWD_DEFINED__ */


#ifndef __IEnumITfCompositionView_FWD_DEFINED__
#define __IEnumITfCompositionView_FWD_DEFINED__
typedef interface IEnumITfCompositionView IEnumITfCompositionView;

#endif 	/* __IEnumITfCompositionView_FWD_DEFINED__ */


#ifndef __ITfComposition_FWD_DEFINED__
#define __ITfComposition_FWD_DEFINED__
typedef interface ITfComposition ITfComposition;

#endif 	/* __ITfComposition_FWD_DEFINED__ */


#ifndef __ITfCompositionSink_FWD_DEFINED__
#define __ITfCompositionSink_FWD_DEFINED__
typedef interface ITfCompositionSink ITfCompositionSink;

#endif 	/* __ITfCompositionSink_FWD_DEFINED__ */


#ifndef __ITfContextComposition_FWD_DEFINED__
#define __ITfContextComposition_FWD_DEFINED__
typedef interface ITfContextComposition ITfContextComposition;

#endif 	/* __ITfContextComposition_FWD_DEFINED__ */


#ifndef __ITfContextOwnerCompositionServices_FWD_DEFINED__
#define __ITfContextOwnerCompositionServices_FWD_DEFINED__
typedef interface ITfContextOwnerCompositionServices ITfContextOwnerCompositionServices;

#endif 	/* __ITfContextOwnerCompositionServices_FWD_DEFINED__ */


#ifndef __ITfContextOwnerCompositionSink_FWD_DEFINED__
#define __ITfContextOwnerCompositionSink_FWD_DEFINED__
typedef interface ITfContextOwnerCompositionSink ITfContextOwnerCompositionSink;

#endif 	/* __ITfContextOwnerCompositionSink_FWD_DEFINED__ */


#ifndef __ITfContextView_FWD_DEFINED__
#define __ITfContextView_FWD_DEFINED__
typedef interface ITfContextView ITfContextView;

#endif 	/* __ITfContextView_FWD_DEFINED__ */


#ifndef __IEnumTfContextViews_FWD_DEFINED__
#define __IEnumTfContextViews_FWD_DEFINED__
typedef interface IEnumTfContextViews IEnumTfContextViews;

#endif 	/* __IEnumTfContextViews_FWD_DEFINED__ */


#ifndef __ITfContext_FWD_DEFINED__
#define __ITfContext_FWD_DEFINED__
typedef interface ITfContext ITfContext;

#endif 	/* __ITfContext_FWD_DEFINED__ */


#ifndef __ITfQueryEmbedded_FWD_DEFINED__
#define __ITfQueryEmbedded_FWD_DEFINED__
typedef interface ITfQueryEmbedded ITfQueryEmbedded;

#endif 	/* __ITfQueryEmbedded_FWD_DEFINED__ */


#ifndef __ITfInsertAtSelection_FWD_DEFINED__
#define __ITfInsertAtSelection_FWD_DEFINED__
typedef interface ITfInsertAtSelection ITfInsertAtSelection;

#endif 	/* __ITfInsertAtSelection_FWD_DEFINED__ */


#ifndef __ITfCleanupContextSink_FWD_DEFINED__
#define __ITfCleanupContextSink_FWD_DEFINED__
typedef interface ITfCleanupContextSink ITfCleanupContextSink;

#endif 	/* __ITfCleanupContextSink_FWD_DEFINED__ */


#ifndef __ITfCleanupContextDurationSink_FWD_DEFINED__
#define __ITfCleanupContextDurationSink_FWD_DEFINED__
typedef interface ITfCleanupContextDurationSink ITfCleanupContextDurationSink;

#endif 	/* __ITfCleanupContextDurationSink_FWD_DEFINED__ */


#ifndef __ITfReadOnlyProperty_FWD_DEFINED__
#define __ITfReadOnlyProperty_FWD_DEFINED__
typedef interface ITfReadOnlyProperty ITfReadOnlyProperty;

#endif 	/* __ITfReadOnlyProperty_FWD_DEFINED__ */


#ifndef __IEnumTfPropertyValue_FWD_DEFINED__
#define __IEnumTfPropertyValue_FWD_DEFINED__
typedef interface IEnumTfPropertyValue IEnumTfPropertyValue;

#endif 	/* __IEnumTfPropertyValue_FWD_DEFINED__ */


#ifndef __ITfMouseTracker_FWD_DEFINED__
#define __ITfMouseTracker_FWD_DEFINED__
typedef interface ITfMouseTracker ITfMouseTracker;

#endif 	/* __ITfMouseTracker_FWD_DEFINED__ */


#ifndef __ITfMouseTrackerACP_FWD_DEFINED__
#define __ITfMouseTrackerACP_FWD_DEFINED__
typedef interface ITfMouseTrackerACP ITfMouseTrackerACP;

#endif 	/* __ITfMouseTrackerACP_FWD_DEFINED__ */


#ifndef __ITfMouseSink_FWD_DEFINED__
#define __ITfMouseSink_FWD_DEFINED__
typedef interface ITfMouseSink ITfMouseSink;

#endif 	/* __ITfMouseSink_FWD_DEFINED__ */


#ifndef __ITfEditRecord_FWD_DEFINED__
#define __ITfEditRecord_FWD_DEFINED__
typedef interface ITfEditRecord ITfEditRecord;

#endif 	/* __ITfEditRecord_FWD_DEFINED__ */


#ifndef __ITfTextEditSink_FWD_DEFINED__
#define __ITfTextEditSink_FWD_DEFINED__
typedef interface ITfTextEditSink ITfTextEditSink;

#endif 	/* __ITfTextEditSink_FWD_DEFINED__ */


#ifndef __ITfTextLayoutSink_FWD_DEFINED__
#define __ITfTextLayoutSink_FWD_DEFINED__
typedef interface ITfTextLayoutSink ITfTextLayoutSink;

#endif 	/* __ITfTextLayoutSink_FWD_DEFINED__ */


#ifndef __ITfStatusSink_FWD_DEFINED__
#define __ITfStatusSink_FWD_DEFINED__
typedef interface ITfStatusSink ITfStatusSink;

#endif 	/* __ITfStatusSink_FWD_DEFINED__ */


#ifndef __ITfEditTransactionSink_FWD_DEFINED__
#define __ITfEditTransactionSink_FWD_DEFINED__
typedef interface ITfEditTransactionSink ITfEditTransactionSink;

#endif 	/* __ITfEditTransactionSink_FWD_DEFINED__ */


#ifndef __ITfContextOwner_FWD_DEFINED__
#define __ITfContextOwner_FWD_DEFINED__
typedef interface ITfContextOwner ITfContextOwner;

#endif 	/* __ITfContextOwner_FWD_DEFINED__ */


#ifndef __ITfContextOwnerServices_FWD_DEFINED__
#define __ITfContextOwnerServices_FWD_DEFINED__
typedef interface ITfContextOwnerServices ITfContextOwnerServices;

#endif 	/* __ITfContextOwnerServices_FWD_DEFINED__ */


#ifndef __ITfContextKeyEventSink_FWD_DEFINED__
#define __ITfContextKeyEventSink_FWD_DEFINED__
typedef interface ITfContextKeyEventSink ITfContextKeyEventSink;

#endif 	/* __ITfContextKeyEventSink_FWD_DEFINED__ */


#ifndef __ITfEditSession_FWD_DEFINED__
#define __ITfEditSession_FWD_DEFINED__
typedef interface ITfEditSession ITfEditSession;

#endif 	/* __ITfEditSession_FWD_DEFINED__ */


#ifndef __ITfRange_FWD_DEFINED__
#define __ITfRange_FWD_DEFINED__
typedef interface ITfRange ITfRange;

#endif 	/* __ITfRange_FWD_DEFINED__ */


#ifndef __ITfRangeACP_FWD_DEFINED__
#define __ITfRangeACP_FWD_DEFINED__
typedef interface ITfRangeACP ITfRangeACP;

#endif 	/* __ITfRangeACP_FWD_DEFINED__ */


#ifndef __ITextStoreACPServices_FWD_DEFINED__
#define __ITextStoreACPServices_FWD_DEFINED__
typedef interface ITextStoreACPServices ITextStoreACPServices;

#endif 	/* __ITextStoreACPServices_FWD_DEFINED__ */


#ifndef __ITfRangeBackup_FWD_DEFINED__
#define __ITfRangeBackup_FWD_DEFINED__
typedef interface ITfRangeBackup ITfRangeBackup;

#endif 	/* __ITfRangeBackup_FWD_DEFINED__ */


#ifndef __ITfPropertyStore_FWD_DEFINED__
#define __ITfPropertyStore_FWD_DEFINED__
typedef interface ITfPropertyStore ITfPropertyStore;

#endif 	/* __ITfPropertyStore_FWD_DEFINED__ */


#ifndef __IEnumTfRanges_FWD_DEFINED__
#define __IEnumTfRanges_FWD_DEFINED__
typedef interface IEnumTfRanges IEnumTfRanges;

#endif 	/* __IEnumTfRanges_FWD_DEFINED__ */


#ifndef __ITfCreatePropertyStore_FWD_DEFINED__
#define __ITfCreatePropertyStore_FWD_DEFINED__
typedef interface ITfCreatePropertyStore ITfCreatePropertyStore;

#endif 	/* __ITfCreatePropertyStore_FWD_DEFINED__ */


#ifndef __ITfPersistentPropertyLoaderACP_FWD_DEFINED__
#define __ITfPersistentPropertyLoaderACP_FWD_DEFINED__
typedef interface ITfPersistentPropertyLoaderACP ITfPersistentPropertyLoaderACP;

#endif 	/* __ITfPersistentPropertyLoaderACP_FWD_DEFINED__ */


#ifndef __ITfProperty_FWD_DEFINED__
#define __ITfProperty_FWD_DEFINED__
typedef interface ITfProperty ITfProperty;

#endif 	/* __ITfProperty_FWD_DEFINED__ */


#ifndef __IEnumTfProperties_FWD_DEFINED__
#define __IEnumTfProperties_FWD_DEFINED__
typedef interface IEnumTfProperties IEnumTfProperties;

#endif 	/* __IEnumTfProperties_FWD_DEFINED__ */


#ifndef __ITfCompartment_FWD_DEFINED__
#define __ITfCompartment_FWD_DEFINED__
typedef interface ITfCompartment ITfCompartment;

#endif 	/* __ITfCompartment_FWD_DEFINED__ */


#ifndef __ITfCompartmentEventSink_FWD_DEFINED__
#define __ITfCompartmentEventSink_FWD_DEFINED__
typedef interface ITfCompartmentEventSink ITfCompartmentEventSink;

#endif 	/* __ITfCompartmentEventSink_FWD_DEFINED__ */


#ifndef __ITfCompartmentMgr_FWD_DEFINED__
#define __ITfCompartmentMgr_FWD_DEFINED__
typedef interface ITfCompartmentMgr ITfCompartmentMgr;

#endif 	/* __ITfCompartmentMgr_FWD_DEFINED__ */


#ifndef __ITfFunction_FWD_DEFINED__
#define __ITfFunction_FWD_DEFINED__
typedef interface ITfFunction ITfFunction;

#endif 	/* __ITfFunction_FWD_DEFINED__ */


#ifndef __ITfFunctionProvider_FWD_DEFINED__
#define __ITfFunctionProvider_FWD_DEFINED__
typedef interface ITfFunctionProvider ITfFunctionProvider;

#endif 	/* __ITfFunctionProvider_FWD_DEFINED__ */


#ifndef __IEnumTfFunctionProviders_FWD_DEFINED__
#define __IEnumTfFunctionProviders_FWD_DEFINED__
typedef interface IEnumTfFunctionProviders IEnumTfFunctionProviders;

#endif 	/* __IEnumTfFunctionProviders_FWD_DEFINED__ */


#ifndef __ITfInputProcessorProfiles_FWD_DEFINED__
#define __ITfInputProcessorProfiles_FWD_DEFINED__
typedef interface ITfInputProcessorProfiles ITfInputProcessorProfiles;

#endif 	/* __ITfInputProcessorProfiles_FWD_DEFINED__ */


#ifndef __ITfInputProcessorProfilesEx_FWD_DEFINED__
#define __ITfInputProcessorProfilesEx_FWD_DEFINED__
typedef interface ITfInputProcessorProfilesEx ITfInputProcessorProfilesEx;

#endif 	/* __ITfInputProcessorProfilesEx_FWD_DEFINED__ */


#ifndef __ITfInputProcessorProfileSubstituteLayout_FWD_DEFINED__
#define __ITfInputProcessorProfileSubstituteLayout_FWD_DEFINED__
typedef interface ITfInputProcessorProfileSubstituteLayout ITfInputProcessorProfileSubstituteLayout;

#endif 	/* __ITfInputProcessorProfileSubstituteLayout_FWD_DEFINED__ */


#ifndef __ITfActiveLanguageProfileNotifySink_FWD_DEFINED__
#define __ITfActiveLanguageProfileNotifySink_FWD_DEFINED__
typedef interface ITfActiveLanguageProfileNotifySink ITfActiveLanguageProfileNotifySink;

#endif 	/* __ITfActiveLanguageProfileNotifySink_FWD_DEFINED__ */


#ifndef __IEnumTfLanguageProfiles_FWD_DEFINED__
#define __IEnumTfLanguageProfiles_FWD_DEFINED__
typedef interface IEnumTfLanguageProfiles IEnumTfLanguageProfiles;

#endif 	/* __IEnumTfLanguageProfiles_FWD_DEFINED__ */


#ifndef __ITfLanguageProfileNotifySink_FWD_DEFINED__
#define __ITfLanguageProfileNotifySink_FWD_DEFINED__
typedef interface ITfLanguageProfileNotifySink ITfLanguageProfileNotifySink;

#endif 	/* __ITfLanguageProfileNotifySink_FWD_DEFINED__ */


#ifndef __ITfInputProcessorProfileMgr_FWD_DEFINED__
#define __ITfInputProcessorProfileMgr_FWD_DEFINED__
typedef interface ITfInputProcessorProfileMgr ITfInputProcessorProfileMgr;

#endif 	/* __ITfInputProcessorProfileMgr_FWD_DEFINED__ */


#ifndef __IEnumTfInputProcessorProfiles_FWD_DEFINED__
#define __IEnumTfInputProcessorProfiles_FWD_DEFINED__
typedef interface IEnumTfInputProcessorProfiles IEnumTfInputProcessorProfiles;

#endif 	/* __IEnumTfInputProcessorProfiles_FWD_DEFINED__ */


#ifndef __ITfInputProcessorProfileActivationSink_FWD_DEFINED__
#define __ITfInputProcessorProfileActivationSink_FWD_DEFINED__
typedef interface ITfInputProcessorProfileActivationSink ITfInputProcessorProfileActivationSink;

#endif 	/* __ITfInputProcessorProfileActivationSink_FWD_DEFINED__ */


#ifndef __ITfKeystrokeMgr_FWD_DEFINED__
#define __ITfKeystrokeMgr_FWD_DEFINED__
typedef interface ITfKeystrokeMgr ITfKeystrokeMgr;

#endif 	/* __ITfKeystrokeMgr_FWD_DEFINED__ */


#ifndef __ITfKeyEventSink_FWD_DEFINED__
#define __ITfKeyEventSink_FWD_DEFINED__
typedef interface ITfKeyEventSink ITfKeyEventSink;

#endif 	/* __ITfKeyEventSink_FWD_DEFINED__ */


#ifndef __ITfKeyTraceEventSink_FWD_DEFINED__
#define __ITfKeyTraceEventSink_FWD_DEFINED__
typedef interface ITfKeyTraceEventSink ITfKeyTraceEventSink;

#endif 	/* __ITfKeyTraceEventSink_FWD_DEFINED__ */


#ifndef __ITfPreservedKeyNotifySink_FWD_DEFINED__
#define __ITfPreservedKeyNotifySink_FWD_DEFINED__
typedef interface ITfPreservedKeyNotifySink ITfPreservedKeyNotifySink;

#endif 	/* __ITfPreservedKeyNotifySink_FWD_DEFINED__ */


#ifndef __ITfMessagePump_FWD_DEFINED__
#define __ITfMessagePump_FWD_DEFINED__
typedef interface ITfMessagePump ITfMessagePump;

#endif 	/* __ITfMessagePump_FWD_DEFINED__ */


#ifndef __ITfThreadFocusSink_FWD_DEFINED__
#define __ITfThreadFocusSink_FWD_DEFINED__
typedef interface ITfThreadFocusSink ITfThreadFocusSink;

#endif 	/* __ITfThreadFocusSink_FWD_DEFINED__ */


#ifndef __ITfTextInputProcessor_FWD_DEFINED__
#define __ITfTextInputProcessor_FWD_DEFINED__
typedef interface ITfTextInputProcessor ITfTextInputProcessor;

#endif 	/* __ITfTextInputProcessor_FWD_DEFINED__ */


#ifndef __ITfTextInputProcessorEx_FWD_DEFINED__
#define __ITfTextInputProcessorEx_FWD_DEFINED__
typedef interface ITfTextInputProcessorEx ITfTextInputProcessorEx;

#endif 	/* __ITfTextInputProcessorEx_FWD_DEFINED__ */


#ifndef __ITfClientId_FWD_DEFINED__
#define __ITfClientId_FWD_DEFINED__
typedef interface ITfClientId ITfClientId;

#endif 	/* __ITfClientId_FWD_DEFINED__ */


#ifndef __ITfDisplayAttributeInfo_FWD_DEFINED__
#define __ITfDisplayAttributeInfo_FWD_DEFINED__
typedef interface ITfDisplayAttributeInfo ITfDisplayAttributeInfo;

#endif 	/* __ITfDisplayAttributeInfo_FWD_DEFINED__ */


#ifndef __IEnumTfDisplayAttributeInfo_FWD_DEFINED__
#define __IEnumTfDisplayAttributeInfo_FWD_DEFINED__
typedef interface IEnumTfDisplayAttributeInfo IEnumTfDisplayAttributeInfo;

#endif 	/* __IEnumTfDisplayAttributeInfo_FWD_DEFINED__ */


#ifndef __ITfDisplayAttributeProvider_FWD_DEFINED__
#define __ITfDisplayAttributeProvider_FWD_DEFINED__
typedef interface ITfDisplayAttributeProvider ITfDisplayAttributeProvider;

#endif 	/* __ITfDisplayAttributeProvider_FWD_DEFINED__ */


#ifndef __ITfDisplayAttributeMgr_FWD_DEFINED__
#define __ITfDisplayAttributeMgr_FWD_DEFINED__
typedef interface ITfDisplayAttributeMgr ITfDisplayAttributeMgr;

#endif 	/* __ITfDisplayAttributeMgr_FWD_DEFINED__ */


#ifndef __ITfDisplayAttributeNotifySink_FWD_DEFINED__
#define __ITfDisplayAttributeNotifySink_FWD_DEFINED__
typedef interface ITfDisplayAttributeNotifySink ITfDisplayAttributeNotifySink;

#endif 	/* __ITfDisplayAttributeNotifySink_FWD_DEFINED__ */


#ifndef __ITfCategoryMgr_FWD_DEFINED__
#define __ITfCategoryMgr_FWD_DEFINED__
typedef interface ITfCategoryMgr ITfCategoryMgr;

#endif 	/* __ITfCategoryMgr_FWD_DEFINED__ */


#ifndef __ITfSource_FWD_DEFINED__
#define __ITfSource_FWD_DEFINED__
typedef interface ITfSource ITfSource;

#endif 	/* __ITfSource_FWD_DEFINED__ */


#ifndef __ITfSourceSingle_FWD_DEFINED__
#define __ITfSourceSingle_FWD_DEFINED__
typedef interface ITfSourceSingle ITfSourceSingle;

#endif 	/* __ITfSourceSingle_FWD_DEFINED__ */


#ifndef __ITfUIElementMgr_FWD_DEFINED__
#define __ITfUIElementMgr_FWD_DEFINED__
typedef interface ITfUIElementMgr ITfUIElementMgr;

#endif 	/* __ITfUIElementMgr_FWD_DEFINED__ */


#ifndef __IEnumTfUIElements_FWD_DEFINED__
#define __IEnumTfUIElements_FWD_DEFINED__
typedef interface IEnumTfUIElements IEnumTfUIElements;

#endif 	/* __IEnumTfUIElements_FWD_DEFINED__ */


#ifndef __ITfUIElementSink_FWD_DEFINED__
#define __ITfUIElementSink_FWD_DEFINED__
typedef interface ITfUIElementSink ITfUIElementSink;

#endif 	/* __ITfUIElementSink_FWD_DEFINED__ */


#ifndef __ITfUIElement_FWD_DEFINED__
#define __ITfUIElement_FWD_DEFINED__
typedef interface ITfUIElement ITfUIElement;

#endif 	/* __ITfUIElement_FWD_DEFINED__ */


#ifndef __ITfCandidateListUIElement_FWD_DEFINED__
#define __ITfCandidateListUIElement_FWD_DEFINED__
typedef interface ITfCandidateListUIElement ITfCandidateListUIElement;

#endif 	/* __ITfCandidateListUIElement_FWD_DEFINED__ */


#ifndef __ITfCandidateListUIElementBehavior_FWD_DEFINED__
#define __ITfCandidateListUIElementBehavior_FWD_DEFINED__
typedef interface ITfCandidateListUIElementBehavior ITfCandidateListUIElementBehavior;

#endif 	/* __ITfCandidateListUIElementBehavior_FWD_DEFINED__ */


#ifndef __ITfReadingInformationUIElement_FWD_DEFINED__
#define __ITfReadingInformationUIElement_FWD_DEFINED__
typedef interface ITfReadingInformationUIElement ITfReadingInformationUIElement;

#endif 	/* __ITfReadingInformationUIElement_FWD_DEFINED__ */


#ifndef __ITfTransitoryExtensionUIElement_FWD_DEFINED__
#define __ITfTransitoryExtensionUIElement_FWD_DEFINED__
typedef interface ITfTransitoryExtensionUIElement ITfTransitoryExtensionUIElement;

#endif 	/* __ITfTransitoryExtensionUIElement_FWD_DEFINED__ */


#ifndef __ITfTransitoryExtensionSink_FWD_DEFINED__
#define __ITfTransitoryExtensionSink_FWD_DEFINED__
typedef interface ITfTransitoryExtensionSink ITfTransitoryExtensionSink;

#endif 	/* __ITfTransitoryExtensionSink_FWD_DEFINED__ */


#ifndef __ITfToolTipUIElement_FWD_DEFINED__
#define __ITfToolTipUIElement_FWD_DEFINED__
typedef interface ITfToolTipUIElement ITfToolTipUIElement;

#endif 	/* __ITfToolTipUIElement_FWD_DEFINED__ */


#ifndef __ITfReverseConversionList_FWD_DEFINED__
#define __ITfReverseConversionList_FWD_DEFINED__
typedef interface ITfReverseConversionList ITfReverseConversionList;

#endif 	/* __ITfReverseConversionList_FWD_DEFINED__ */


#ifndef __ITfReverseConversion_FWD_DEFINED__
#define __ITfReverseConversion_FWD_DEFINED__
typedef interface ITfReverseConversion ITfReverseConversion;

#endif 	/* __ITfReverseConversion_FWD_DEFINED__ */


#ifndef __ITfReverseConversionMgr_FWD_DEFINED__
#define __ITfReverseConversionMgr_FWD_DEFINED__
typedef interface ITfReverseConversionMgr ITfReverseConversionMgr;

#endif 	/* __ITfReverseConversionMgr_FWD_DEFINED__ */


/* header files for imported files */
#include "oaidl.h"
#include "comcat.h"
#include "textstor.h"
#include "ctfutb.h"

#ifdef __cplusplus
extern "C"{
#endif 


/* interface __MIDL_itf_msctf_0000_0000 */
/* [local] */ 

#include <winapifamily.h>
//=--------------------------------------------------------------------------=
// msctf.h


// Text Framework declarations.

//=--------------------------------------------------------------------------=
// (C) Copyright 1995-2001 Microsoft Corporation.  All Rights Reserved.
//
// THIS CODE AND INFORMATION IS PROVIDED "AS IS" WITHOUT WARRANTY OF
// ANY KIND, EITHER EXPRESSED OR IMPLIED, INCLUDING BUT NOT LIMITED TO
// THE IMPLIED WARRANTIES OF MERCHANTABILITY AND/OR FITNESS FOR A
// PARTICULAR PURPOSE.
//=--------------------------------------------------------------------------=

#ifndef MSCTF_DEFINED
#define MSCTF_DEFINED

#include <windows.h>

#ifdef __cplusplus
extern "C" {
#endif /* __cplusplus */

#pragma region Application Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP)
#define TF_E_LOCKED          MAKE_HRESULT(SEVERITY_ERROR, FACILITY_ITF, 0x0500)
#define TF_E_STACKFULL       MAKE_HRESULT(SEVERITY_ERROR, FACILITY_ITF, 0x0501)
#define TF_E_NOTOWNEDRANGE   MAKE_HRESULT(SEVERITY_ERROR, FACILITY_ITF, 0x0502)
#define TF_E_NOPROVIDER      MAKE_HRESULT(SEVERITY_ERROR, FACILITY_ITF, 0x0503)
#define TF_E_DISCONNECTED    MAKE_HRESULT(SEVERITY_ERROR, FACILITY_ITF, 0x0504)
#define TF_E_INVALIDVIEW     MAKE_HRESULT(SEVERITY_ERROR, FACILITY_ITF, 0x0505)
#define TF_E_ALREADY_EXISTS  MAKE_HRESULT(SEVERITY_ERROR, FACILITY_ITF, 0x0506)
#define TF_E_RANGE_NOT_COVERED MAKE_HRESULT(SEVERITY_ERROR, FACILITY_ITF, 0x0507)
#define TF_E_COMPOSITION_REJECTED MAKE_HRESULT(SEVERITY_ERROR, FACILITY_ITF, 0x0508)
#define TF_E_EMPTYCONTEXT    MAKE_HRESULT(SEVERITY_ERROR, FACILITY_ITF, 0x0509)
#define TF_E_INVALIDPOS      MAKE_HRESULT(SEVERITY_ERROR, FACILITY_ITF, 0x0200)
#define TF_E_NOLOCK          MAKE_HRESULT(SEVERITY_ERROR, FACILITY_ITF, 0x0201)
#define TF_E_NOOBJECT        MAKE_HRESULT(SEVERITY_ERROR, FACILITY_ITF, 0x0202)
#define TF_E_NOSERVICE       MAKE_HRESULT(SEVERITY_ERROR, FACILITY_ITF, 0x0203)
#define TF_E_NOINTERFACE     MAKE_HRESULT(SEVERITY_ERROR, FACILITY_ITF, 0x0204)
#define TF_E_NOSELECTION     MAKE_HRESULT(SEVERITY_ERROR, FACILITY_ITF, 0x0205)
#define TF_E_NOLAYOUT        MAKE_HRESULT(SEVERITY_ERROR, FACILITY_ITF, 0x0206)
#define TF_E_INVALIDPOINT    MAKE_HRESULT(SEVERITY_ERROR, FACILITY_ITF, 0x0207)
#define TF_E_SYNCHRONOUS     MAKE_HRESULT(SEVERITY_ERROR, FACILITY_ITF, 0x0208)
#define TF_E_READONLY        MAKE_HRESULT(SEVERITY_ERROR, FACILITY_ITF, 0x0209)
#define TF_E_FORMAT          MAKE_HRESULT(SEVERITY_ERROR, FACILITY_ITF, 0x020a)
#define TF_S_ASYNC           MAKE_HRESULT(SEVERITY_SUCCESS, FACILITY_ITF, 0x0300)
#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP) */
#pragma endregion

#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)
#define TF_RCM_COMLESS                          0x00000001
#define TF_RCM_VKEY                             0x00000002
#define TF_RCM_HINT_READING_LENGTH              0x00000004
#define TF_RCM_HINT_COLLISION                   0x00000008
HRESULT WINAPI TF_CreateThreadMgr(ITfThreadMgr **pptim);
HRESULT WINAPI TF_GetThreadMgr(ITfThreadMgr **pptim);
HRESULT WINAPI TF_CreateInputProcessorProfiles(ITfInputProcessorProfiles **ppipr);
HRESULT WINAPI TF_CreateDisplayAttributeMgr(ITfDisplayAttributeMgr **ppdam);
HRESULT WINAPI TF_CreateLangBarMgr(ITfLangBarMgr **pppbm);
HRESULT WINAPI TF_CreateLangBarItemMgr(ITfLangBarItemMgr **pplbim);
HRESULT WINAPI TF_CreateCategoryMgr(ITfCategoryMgr **ppcat);

EXTERN_C const GUID GUID_PROP_TEXTOWNER;
EXTERN_C const GUID GUID_PROP_ATTRIBUTE;
EXTERN_C const GUID GUID_PROP_LANGID;
EXTERN_C const GUID GUID_PROP_READING;
EXTERN_C const GUID GUID_PROP_COMPOSING;
EXTERN_C const GUID GUID_PROP_TKB_ALTERNATES;
EXTERN_C const GUID GUID_PROP_URL;

#define TKB_ALTERNATES_STANDARD               0x00000001
#define TKB_ALTERNATES_FOR_AUTOCORRECTION     0x00000002
#define TKB_ALTERNATES_FOR_PREDICTION         0x00000003
#define TKB_ALTERNATES_AUTOCORRECTION_APPLIED 0x00000004
#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion
#pragma region Application Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP)
EXTERN_C const CLSID CLSID_TF_ThreadMgr;
EXTERN_C const CLSID CLSID_TF_InputProcessorProfiles;
#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP) */
#pragma endregion
#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)
EXTERN_C const CLSID CLSID_TF_LangBarMgr;
#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion
#pragma region Application Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP)
EXTERN_C const CLSID CLSID_TF_DisplayAttributeMgr;
EXTERN_C const CLSID CLSID_TF_CategoryMgr;
#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP) */
#pragma endregion
#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)
EXTERN_C const CLSID CLSID_TF_LangBarItemMgr;
#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion
#pragma region Application Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP)
EXTERN_C const GUID GUID_SYSTEM_FUNCTIONPROVIDER;
EXTERN_C const GUID GUID_APP_FUNCTIONPROVIDER;


EXTERN_C const GUID GUID_COMPARTMENT_KEYBOARD_DISABLED;
EXTERN_C const GUID GUID_COMPARTMENT_KEYBOARD_OPENCLOSE;
EXTERN_C const GUID GUID_COMPARTMENT_HANDWRITING_OPENCLOSE;
EXTERN_C const GUID GUID_COMPARTMENT_SPEECH_DISABLED;
EXTERN_C const GUID GUID_COMPARTMENT_SPEECH_OPENCLOSE;
EXTERN_C const GUID GUID_COMPARTMENT_SPEECH_GLOBALSTATE;
EXTERN_C const GUID GUID_COMPARTMENT_PERSISTMENUENABLED;
EXTERN_C const GUID GUID_COMPARTMENT_EMPTYCONTEXT;
EXTERN_C const GUID GUID_COMPARTMENT_TIPUISTATUS;
EXTERN_C const GUID GUID_COMPARTMENT_KEYBOARD_INPUTMODE_CONVERSION;
EXTERN_C const GUID GUID_COMPARTMENT_KEYBOARD_INPUTMODE_SENTENCE;
EXTERN_C const GUID GUID_COMPARTMENT_TRANSITORYEXTENSION;
EXTERN_C const GUID GUID_COMPARTMENT_TRANSITORYEXTENSION_DOCUMENTMANAGER;
EXTERN_C const GUID GUID_COMPARTMENT_TRANSITORYEXTENSION_PARENT;
EXTERN_C const GUID GUID_COMPARTMENT_ENABLED_PROFILES_UPDATED;

EXTERN_C const GUID GUID_PROP_MODEBIAS;

EXTERN_C const GUID GUID_MODEBIAS_NONE;
EXTERN_C const GUID GUID_MODEBIAS_URLHISTORY;
EXTERN_C const GUID GUID_MODEBIAS_FILENAME;
EXTERN_C const GUID GUID_MODEBIAS_READING;
EXTERN_C const GUID GUID_MODEBIAS_DATETIME;
EXTERN_C const GUID GUID_MODEBIAS_NAME;
EXTERN_C const GUID GUID_MODEBIAS_CONVERSATION;
EXTERN_C const GUID GUID_MODEBIAS_NUMERIC;
EXTERN_C const GUID GUID_MODEBIAS_HIRAGANA;
EXTERN_C const GUID GUID_MODEBIAS_KATAKANA;
EXTERN_C const GUID GUID_MODEBIAS_HANGUL;
EXTERN_C const GUID GUID_MODEBIAS_CHINESE;
EXTERN_C const GUID GUID_MODEBIAS_HALFWIDTHKATAKANA;
EXTERN_C const GUID GUID_MODEBIAS_FULLWIDTHALPHANUMERIC;
EXTERN_C const GUID GUID_MODEBIAS_HALFWIDTHALPHANUMERIC;
EXTERN_C const GUID GUID_MODEBIAS_FULLWIDTHHANGUL;
EXTERN_C const GUID GUID_TFCAT_CATEGORY_OF_TIP;
EXTERN_C const GUID GUID_TFCAT_TIP_KEYBOARD;
EXTERN_C const GUID GUID_TFCAT_TIP_SPEECH;
EXTERN_C const GUID GUID_TFCAT_TIP_HANDWRITING;
EXTERN_C const GUID GUID_TFCAT_TIPCAP_SECUREMODE;
EXTERN_C const GUID GUID_TFCAT_TIPCAP_UIELEMENTENABLED;
EXTERN_C const GUID GUID_TFCAT_TIPCAP_INPUTMODECOMPARTMENT;
EXTERN_C const GUID GUID_TFCAT_TIPCAP_COMLESS;
EXTERN_C const GUID GUID_TFCAT_TIPCAP_WOW16;
EXTERN_C const GUID GUID_TFCAT_TIPCAP_IMMERSIVESUPPORT;
EXTERN_C const GUID GUID_TFCAT_TIPCAP_SYSTRAYSUPPORT;

EXTERN_C const GUID GUID_TFCAT_PROP_AUDIODATA;
EXTERN_C const GUID GUID_TFCAT_PROP_INKDATA;

EXTERN_C const GUID GUID_TFCAT_PROPSTYLE_CUSTOM;
EXTERN_C const GUID GUID_TFCAT_PROPSTYLE_STATIC;
EXTERN_C const GUID GUID_TFCAT_PROPSTYLE_STATICCOMPACT;

EXTERN_C const GUID GUID_TFCAT_DISPLAYATTRIBUTEPROVIDER;
EXTERN_C const GUID GUID_TFCAT_DISPLAYATTRIBUTEPROPERTY;

#define TF_INVALID_GUIDATOM ((TfGuidAtom)0)
#define TF_CLIENTID_NULL    ((TfClientId)0)

#define TF_TMAE_NOACTIVATETIP              0x00000001
#define TF_TMAE_SECUREMODE                 0x00000002
#define TF_TMAE_UIELEMENTENABLEDONLY       0x00000004
#define TF_TMAE_COMLESS                    0x00000008
#define TF_TMAE_WOW16                      0x00000010
#define TF_TMAE_NOACTIVATEKEYBOARDLAYOUT   0x00000020
#define TF_TMAE_CONSOLE                    0x00000040

#define TF_TMF_NOACTIVATETIP          TF_TMAE_NOACTIVATETIP
#define TF_TMF_SECUREMODE             TF_TMAE_SECUREMODE
#define TF_TMF_UIELEMENTENABLEDONLY   TF_TMAE_UIELEMENTENABLEDONLY
#define TF_TMF_COMLESS                TF_TMAE_COMLESS
#define TF_TMF_WOW16                  TF_TMAE_WOW16
#define TF_TMF_CONSOLE                TF_TMAE_CONSOLE
#define TF_TMF_IMMERSIVEMODE          0x40000000
#define TF_TMF_ACTIVATED              0x80000000

#define TF_MOD_ALT                         0x0001
#define TF_MOD_CONTROL                     0x0002
#define TF_MOD_SHIFT                       0x0004
#define TF_MOD_RALT                        0x0008
#define TF_MOD_RCONTROL                    0x0010
#define TF_MOD_RSHIFT                      0x0020
#define TF_MOD_LALT                        0x0040
#define TF_MOD_LCONTROL                    0x0080
#define TF_MOD_LSHIFT                      0x0100
#define TF_MOD_ON_KEYUP                    0x0200
#define TF_MOD_IGNORE_ALL_MODIFIER         0x0400

#define TF_US_HIDETIPUI         0x00000001

#define TF_DISABLE_SPEECH         0x00000001
#define TF_DISABLE_DICTATION      0x00000002
#define TF_DISABLE_COMMANDING     0x00000004

#define TF_PROCESS_ATOM             TEXT("_CTF_PROCESS_ATOM_")
#define TF_ENABLE_PROCESS_ATOM      TEXT("_CTF_ENABLE_PROCESS_ATOM_")
#define TF_INVALID_UIELEMENTID    DWORD(-1)
#define TF_CLUIE_DOCUMENTMGR      0x00000001
#define TF_CLUIE_COUNT            0x00000002
#define TF_CLUIE_SELECTION        0x00000004
#define TF_CLUIE_STRING           0x00000008
#define TF_CLUIE_PAGEINDEX        0x00000010
#define TF_CLUIE_CURRENTPAGE      0x00000020
#define TF_RIUIE_CONTEXT                 0x00000001
#define TF_RIUIE_STRING                  0x00000002
#define TF_RIUIE_MAXREADINGSTRINGLENGTH  0x00000004
#define TF_RIUIE_ERRORINDEX              0x00000008
#define TF_RIUIE_VERTICALORDER           0x00000010
#define TF_CONVERSIONMODE_ALPHANUMERIC        0x0000
#define TF_CONVERSIONMODE_NATIVE              0x0001
#define TF_CONVERSIONMODE_KATAKANA            0x0002
#define TF_CONVERSIONMODE_FULLSHAPE           0x0008
#define TF_CONVERSIONMODE_ROMAN               0x0010
#define TF_CONVERSIONMODE_CHARCODE            0x0020
#define TF_CONVERSIONMODE_SOFTKEYBOARD        0x0080
#define TF_CONVERSIONMODE_NOCONVERSION        0x0100
#define TF_CONVERSIONMODE_EUDC                0x0200
#define TF_CONVERSIONMODE_SYMBOL              0x0400
#define TF_CONVERSIONMODE_FIXED               0x0800
#define TF_SENTENCEMODE_NONE                  0x0000
#define TF_SENTENCEMODE_PLAURALCLAUSE         0x0001
#define TF_SENTENCEMODE_SINGLECONVERT         0x0002
#define TF_SENTENCEMODE_AUTOMATIC             0x0004
#define TF_SENTENCEMODE_PHRASEPREDICT         0x0008
#define TF_SENTENCEMODE_CONVERSATION          0x0010

#define TF_TRANSITORYEXTENSION_NONE           0x0000
#define TF_TRANSITORYEXTENSION_FLOATING       0x0001
#define TF_TRANSITORYEXTENSION_ATSELECTION    0x0002

#define TF_PROFILETYPE_INPUTPROCESSOR         0x0001
#define TF_PROFILETYPE_KEYBOARDLAYOUT         0x0002
#define TF_RIP_FLAG_FREEUNUSEDLIBRARIES         0x00000001
#define TF_IPP_FLAG_ACTIVE                      0x00000001
#define TF_IPP_FLAG_ENABLED                     0x00000002
#define TF_IPP_FLAG_SUBSTITUTEDBYINPUTPROCESSOR 0x00000004
#define TF_IPP_CAPS_DISABLEONTRANSITORY         0x00000001
#define TF_IPP_CAPS_SECUREMODESUPPORT           0x00000002
#define TF_IPP_CAPS_UIELEMENTENABLED            0x00000004
#define TF_IPP_CAPS_COMLESSSUPPORT              0x00000008
#define TF_IPP_CAPS_WOW16SUPPORT                0x00000010
#define TF_IPP_CAPS_IMMERSIVESUPPORT            0x00010000
#define TF_IPP_CAPS_SYSTRAYSUPPORT              0x00020000
#define TF_IPPMF_FORPROCESS                     0x10000000
#define TF_IPPMF_FORSESSION                     0x20000000
#define TF_IPPMF_FORSYSTEMALL                   0x40000000
#define TF_IPPMF_ENABLEPROFILE                  0x00000001
#define TF_IPPMF_DISABLEPROFILE                 0x00000002
#define TF_IPPMF_DONTCARECURRENTINPUTLANGUAGE   0x00000004
#define TF_RP_HIDDENINSETTINGUI                 0x00000002
#define TF_RP_LOCALPROCESS                      0x00000004
#define TF_RP_LOCALTHREAD                       0x00000008
#define TF_RP_SUBITEMINSETTINGUI                0x00000010
#define TF_URP_ALLPROFILES                      0x00000002
#define TF_URP_LOCALPROCESS                     0x00000004
#define TF_URP_LOCALTHREAD                      0x00000008
#define TF_IPSINK_FLAG_ACTIVE                   0x0001
#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP) */
#pragma endregion
#ifdef __cplusplus
}
#endif  /* __cplusplus */
#pragma region Application Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP)

#if 0
typedef /* [uuid] */  DECLSPEC_UUID("4f5d560f-5ab5-4dde-8c4d-404592857ab0") UINT_PTR HKL;

#endif





















#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP) */
#pragma endregion
#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion
#pragma region Application Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP)

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP) */
#pragma endregion
#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion
#pragma region Application Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP)



typedef /* [uuid] */  DECLSPEC_UUID("7213778c-7bb0-4270-b050-6189ee594e97") DWORD TfEditCookie;

#define	TF_INVALID_EDIT_COOKIE	( 0 )

typedef /* [uuid] */  DECLSPEC_UUID("88a9c478-f3ec-4763-8345-cd9250443f8d") DWORD TfGuidAtom;

typedef /* [uuid] */  DECLSPEC_UUID("de403c21-89fd-4f85-8b87-64584d063fbc") DWORD TfClientId;

typedef /* [uuid] */  DECLSPEC_UUID("e26d9e1d-691e-4f29-90d7-338dcf1f8cef") struct TF_PERSISTENT_PROPERTY_HEADER_ACP
    {
    GUID guidType;
    LONG ichStart;
    LONG cch;
    ULONG cb;
    DWORD dwPrivate;
    CLSID clsidTIP;
    } 	TF_PERSISTENT_PROPERTY_HEADER_ACP;

typedef /* [uuid] */  DECLSPEC_UUID("e1b5808d-1e46-4c19-84dc-68c5f5978cc8") struct TF_LANGUAGEPROFILE
    {
    CLSID clsid;
    LANGID langid;
    GUID catid;
    BOOL fActive;
    GUID guidProfile;
    } 	TF_LANGUAGEPROFILE;

typedef /* [public][public][public][public][public][public][public][public][public][public][public][uuid] */  DECLSPEC_UUID("5a886226-ae9a-489b-b991-2b1e25ee59a9") 
enum __MIDL___MIDL_itf_msctf_0000_0000_0001
    {
        TF_ANCHOR_START	= 0,
        TF_ANCHOR_END	= 1
    } 	TfAnchor;

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP) */
#pragma endregion
#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)


extern RPC_IF_HANDLE __MIDL_itf_msctf_0000_0000_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_msctf_0000_0000_v0_0_s_ifspec;

#ifndef __ITfThreadMgr_INTERFACE_DEFINED__
#define __ITfThreadMgr_INTERFACE_DEFINED__

/* interface ITfThreadMgr */
/* [unique][uuid][object] */ 


EXTERN_C const IID IID_ITfThreadMgr;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("aa80e801-2021-11d2-93e0-0060b067b86e")
    ITfThreadMgr : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE Activate( 
            /* [out] */ __RPC__out TfClientId *ptid) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Deactivate( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE CreateDocumentMgr( 
            /* [out] */ __RPC__deref_out_opt ITfDocumentMgr **ppdim) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE EnumDocumentMgrs( 
            /* [out] */ __RPC__deref_out_opt IEnumTfDocumentMgrs **ppEnum) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetFocus( 
            /* [out] */ __RPC__deref_out_opt ITfDocumentMgr **ppdimFocus) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetFocus( 
            /* [in] */ __RPC__in_opt ITfDocumentMgr *pdimFocus) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE AssociateFocus( 
            /* [in] */ __RPC__in HWND hwnd,
            /* [unique][in] */ __RPC__in_opt ITfDocumentMgr *pdimNew,
            /* [out] */ __RPC__deref_out_opt ITfDocumentMgr **ppdimPrev) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE IsThreadFocus( 
            /* [out] */ __RPC__out BOOL *pfThreadFocus) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetFunctionProvider( 
            /* [in] */ __RPC__in REFCLSID clsid,
            /* [out] */ __RPC__deref_out_opt ITfFunctionProvider **ppFuncProv) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE EnumFunctionProviders( 
            /* [out] */ __RPC__deref_out_opt IEnumTfFunctionProviders **ppEnum) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetGlobalCompartment( 
            /* [out] */ __RPC__deref_out_opt ITfCompartmentMgr **ppCompMgr) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ITfThreadMgrVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in ITfThreadMgr * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in ITfThreadMgr * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in ITfThreadMgr * This);
        
        DECLSPEC_XFGVIRT(ITfThreadMgr, Activate)
        HRESULT ( STDMETHODCALLTYPE *Activate )( 
            __RPC__in ITfThreadMgr * This,
            /* [out] */ __RPC__out TfClientId *ptid);
        
        DECLSPEC_XFGVIRT(ITfThreadMgr, Deactivate)
        HRESULT ( STDMETHODCALLTYPE *Deactivate )( 
            __RPC__in ITfThreadMgr * This);
        
        DECLSPEC_XFGVIRT(ITfThreadMgr, CreateDocumentMgr)
        HRESULT ( STDMETHODCALLTYPE *CreateDocumentMgr )( 
            __RPC__in ITfThreadMgr * This,
            /* [out] */ __RPC__deref_out_opt ITfDocumentMgr **ppdim);
        
        DECLSPEC_XFGVIRT(ITfThreadMgr, EnumDocumentMgrs)
        HRESULT ( STDMETHODCALLTYPE *EnumDocumentMgrs )( 
            __RPC__in ITfThreadMgr * This,
            /* [out] */ __RPC__deref_out_opt IEnumTfDocumentMgrs **ppEnum);
        
        DECLSPEC_XFGVIRT(ITfThreadMgr, GetFocus)
        HRESULT ( STDMETHODCALLTYPE *GetFocus )( 
            __RPC__in ITfThreadMgr * This,
            /* [out] */ __RPC__deref_out_opt ITfDocumentMgr **ppdimFocus);
        
        DECLSPEC_XFGVIRT(ITfThreadMgr, SetFocus)
        HRESULT ( STDMETHODCALLTYPE *SetFocus )( 
            __RPC__in ITfThreadMgr * This,
            /* [in] */ __RPC__in_opt ITfDocumentMgr *pdimFocus);
        
        DECLSPEC_XFGVIRT(ITfThreadMgr, AssociateFocus)
        HRESULT ( STDMETHODCALLTYPE *AssociateFocus )( 
            __RPC__in ITfThreadMgr * This,
            /* [in] */ __RPC__in HWND hwnd,
            /* [unique][in] */ __RPC__in_opt ITfDocumentMgr *pdimNew,
            /* [out] */ __RPC__deref_out_opt ITfDocumentMgr **ppdimPrev);
        
        DECLSPEC_XFGVIRT(ITfThreadMgr, IsThreadFocus)
        HRESULT ( STDMETHODCALLTYPE *IsThreadFocus )( 
            __RPC__in ITfThreadMgr * This,
            /* [out] */ __RPC__out BOOL *pfThreadFocus);
        
        DECLSPEC_XFGVIRT(ITfThreadMgr, GetFunctionProvider)
        HRESULT ( STDMETHODCALLTYPE *GetFunctionProvider )( 
            __RPC__in ITfThreadMgr * This,
            /* [in] */ __RPC__in REFCLSID clsid,
            /* [out] */ __RPC__deref_out_opt ITfFunctionProvider **ppFuncProv);
        
        DECLSPEC_XFGVIRT(ITfThreadMgr, EnumFunctionProviders)
        HRESULT ( STDMETHODCALLTYPE *EnumFunctionProviders )( 
            __RPC__in ITfThreadMgr * This,
            /* [out] */ __RPC__deref_out_opt IEnumTfFunctionProviders **ppEnum);
        
        DECLSPEC_XFGVIRT(ITfThreadMgr, GetGlobalCompartment)
        HRESULT ( STDMETHODCALLTYPE *GetGlobalCompartment )( 
            __RPC__in ITfThreadMgr * This,
            /* [out] */ __RPC__deref_out_opt ITfCompartmentMgr **ppCompMgr);
        
        END_INTERFACE
    } ITfThreadMgrVtbl;

    interface ITfThreadMgr
    {
        CONST_VTBL struct ITfThreadMgrVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ITfThreadMgr_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ITfThreadMgr_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ITfThreadMgr_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ITfThreadMgr_Activate(This,ptid)	\
    ( (This)->lpVtbl -> Activate(This,ptid) ) 

#define ITfThreadMgr_Deactivate(This)	\
    ( (This)->lpVtbl -> Deactivate(This) ) 

#define ITfThreadMgr_CreateDocumentMgr(This,ppdim)	\
    ( (This)->lpVtbl -> CreateDocumentMgr(This,ppdim) ) 

#define ITfThreadMgr_EnumDocumentMgrs(This,ppEnum)	\
    ( (This)->lpVtbl -> EnumDocumentMgrs(This,ppEnum) ) 

#define ITfThreadMgr_GetFocus(This,ppdimFocus)	\
    ( (This)->lpVtbl -> GetFocus(This,ppdimFocus) ) 

#define ITfThreadMgr_SetFocus(This,pdimFocus)	\
    ( (This)->lpVtbl -> SetFocus(This,pdimFocus) ) 

#define ITfThreadMgr_AssociateFocus(This,hwnd,pdimNew,ppdimPrev)	\
    ( (This)->lpVtbl -> AssociateFocus(This,hwnd,pdimNew,ppdimPrev) ) 

#define ITfThreadMgr_IsThreadFocus(This,pfThreadFocus)	\
    ( (This)->lpVtbl -> IsThreadFocus(This,pfThreadFocus) ) 

#define ITfThreadMgr_GetFunctionProvider(This,clsid,ppFuncProv)	\
    ( (This)->lpVtbl -> GetFunctionProvider(This,clsid,ppFuncProv) ) 

#define ITfThreadMgr_EnumFunctionProviders(This,ppEnum)	\
    ( (This)->lpVtbl -> EnumFunctionProviders(This,ppEnum) ) 

#define ITfThreadMgr_GetGlobalCompartment(This,ppCompMgr)	\
    ( (This)->lpVtbl -> GetGlobalCompartment(This,ppCompMgr) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ITfThreadMgr_INTERFACE_DEFINED__ */


#ifndef __ITfThreadMgrEx_INTERFACE_DEFINED__
#define __ITfThreadMgrEx_INTERFACE_DEFINED__

/* interface ITfThreadMgrEx */
/* [unique][uuid][object] */ 


EXTERN_C const IID IID_ITfThreadMgrEx;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("3e90ade3-7594-4cb0-bb58-69628f5f458c")
    ITfThreadMgrEx : public ITfThreadMgr
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE ActivateEx( 
            /* [out] */ __RPC__out TfClientId *ptid,
            /* [in] */ DWORD dwFlags) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetActiveFlags( 
            /* [out] */ __RPC__out DWORD *lpdwFlags) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ITfThreadMgrExVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in ITfThreadMgrEx * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in ITfThreadMgrEx * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in ITfThreadMgrEx * This);
        
        DECLSPEC_XFGVIRT(ITfThreadMgr, Activate)
        HRESULT ( STDMETHODCALLTYPE *Activate )( 
            __RPC__in ITfThreadMgrEx * This,
            /* [out] */ __RPC__out TfClientId *ptid);
        
        DECLSPEC_XFGVIRT(ITfThreadMgr, Deactivate)
        HRESULT ( STDMETHODCALLTYPE *Deactivate )( 
            __RPC__in ITfThreadMgrEx * This);
        
        DECLSPEC_XFGVIRT(ITfThreadMgr, CreateDocumentMgr)
        HRESULT ( STDMETHODCALLTYPE *CreateDocumentMgr )( 
            __RPC__in ITfThreadMgrEx * This,
            /* [out] */ __RPC__deref_out_opt ITfDocumentMgr **ppdim);
        
        DECLSPEC_XFGVIRT(ITfThreadMgr, EnumDocumentMgrs)
        HRESULT ( STDMETHODCALLTYPE *EnumDocumentMgrs )( 
            __RPC__in ITfThreadMgrEx * This,
            /* [out] */ __RPC__deref_out_opt IEnumTfDocumentMgrs **ppEnum);
        
        DECLSPEC_XFGVIRT(ITfThreadMgr, GetFocus)
        HRESULT ( STDMETHODCALLTYPE *GetFocus )( 
            __RPC__in ITfThreadMgrEx * This,
            /* [out] */ __RPC__deref_out_opt ITfDocumentMgr **ppdimFocus);
        
        DECLSPEC_XFGVIRT(ITfThreadMgr, SetFocus)
        HRESULT ( STDMETHODCALLTYPE *SetFocus )( 
            __RPC__in ITfThreadMgrEx * This,
            /* [in] */ __RPC__in_opt ITfDocumentMgr *pdimFocus);
        
        DECLSPEC_XFGVIRT(ITfThreadMgr, AssociateFocus)
        HRESULT ( STDMETHODCALLTYPE *AssociateFocus )( 
            __RPC__in ITfThreadMgrEx * This,
            /* [in] */ __RPC__in HWND hwnd,
            /* [unique][in] */ __RPC__in_opt ITfDocumentMgr *pdimNew,
            /* [out] */ __RPC__deref_out_opt ITfDocumentMgr **ppdimPrev);
        
        DECLSPEC_XFGVIRT(ITfThreadMgr, IsThreadFocus)
        HRESULT ( STDMETHODCALLTYPE *IsThreadFocus )( 
            __RPC__in ITfThreadMgrEx * This,
            /* [out] */ __RPC__out BOOL *pfThreadFocus);
        
        DECLSPEC_XFGVIRT(ITfThreadMgr, GetFunctionProvider)
        HRESULT ( STDMETHODCALLTYPE *GetFunctionProvider )( 
            __RPC__in ITfThreadMgrEx * This,
            /* [in] */ __RPC__in REFCLSID clsid,
            /* [out] */ __RPC__deref_out_opt ITfFunctionProvider **ppFuncProv);
        
        DECLSPEC_XFGVIRT(ITfThreadMgr, EnumFunctionProviders)
        HRESULT ( STDMETHODCALLTYPE *EnumFunctionProviders )( 
            __RPC__in ITfThreadMgrEx * This,
            /* [out] */ __RPC__deref_out_opt IEnumTfFunctionProviders **ppEnum);
        
        DECLSPEC_XFGVIRT(ITfThreadMgr, GetGlobalCompartment)
        HRESULT ( STDMETHODCALLTYPE *GetGlobalCompartment )( 
            __RPC__in ITfThreadMgrEx * This,
            /* [out] */ __RPC__deref_out_opt ITfCompartmentMgr **ppCompMgr);
        
        DECLSPEC_XFGVIRT(ITfThreadMgrEx, ActivateEx)
        HRESULT ( STDMETHODCALLTYPE *ActivateEx )( 
            __RPC__in ITfThreadMgrEx * This,
            /* [out] */ __RPC__out TfClientId *ptid,
            /* [in] */ DWORD dwFlags);
        
        DECLSPEC_XFGVIRT(ITfThreadMgrEx, GetActiveFlags)
        HRESULT ( STDMETHODCALLTYPE *GetActiveFlags )( 
            __RPC__in ITfThreadMgrEx * This,
            /* [out] */ __RPC__out DWORD *lpdwFlags);
        
        END_INTERFACE
    } ITfThreadMgrExVtbl;

    interface ITfThreadMgrEx
    {
        CONST_VTBL struct ITfThreadMgrExVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ITfThreadMgrEx_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ITfThreadMgrEx_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ITfThreadMgrEx_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ITfThreadMgrEx_Activate(This,ptid)	\
    ( (This)->lpVtbl -> Activate(This,ptid) ) 

#define ITfThreadMgrEx_Deactivate(This)	\
    ( (This)->lpVtbl -> Deactivate(This) ) 

#define ITfThreadMgrEx_CreateDocumentMgr(This,ppdim)	\
    ( (This)->lpVtbl -> CreateDocumentMgr(This,ppdim) ) 

#define ITfThreadMgrEx_EnumDocumentMgrs(This,ppEnum)	\
    ( (This)->lpVtbl -> EnumDocumentMgrs(This,ppEnum) ) 

#define ITfThreadMgrEx_GetFocus(This,ppdimFocus)	\
    ( (This)->lpVtbl -> GetFocus(This,ppdimFocus) ) 

#define ITfThreadMgrEx_SetFocus(This,pdimFocus)	\
    ( (This)->lpVtbl -> SetFocus(This,pdimFocus) ) 

#define ITfThreadMgrEx_AssociateFocus(This,hwnd,pdimNew,ppdimPrev)	\
    ( (This)->lpVtbl -> AssociateFocus(This,hwnd,pdimNew,ppdimPrev) ) 

#define ITfThreadMgrEx_IsThreadFocus(This,pfThreadFocus)	\
    ( (This)->lpVtbl -> IsThreadFocus(This,pfThreadFocus) ) 

#define ITfThreadMgrEx_GetFunctionProvider(This,clsid,ppFuncProv)	\
    ( (This)->lpVtbl -> GetFunctionProvider(This,clsid,ppFuncProv) ) 

#define ITfThreadMgrEx_EnumFunctionProviders(This,ppEnum)	\
    ( (This)->lpVtbl -> EnumFunctionProviders(This,ppEnum) ) 

#define ITfThreadMgrEx_GetGlobalCompartment(This,ppCompMgr)	\
    ( (This)->lpVtbl -> GetGlobalCompartment(This,ppCompMgr) ) 


#define ITfThreadMgrEx_ActivateEx(This,ptid,dwFlags)	\
    ( (This)->lpVtbl -> ActivateEx(This,ptid,dwFlags) ) 

#define ITfThreadMgrEx_GetActiveFlags(This,lpdwFlags)	\
    ( (This)->lpVtbl -> GetActiveFlags(This,lpdwFlags) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ITfThreadMgrEx_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_msctf_0000_0002 */
/* [local] */ 

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion
#pragma region Application Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP)


extern RPC_IF_HANDLE __MIDL_itf_msctf_0000_0002_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_msctf_0000_0002_v0_0_s_ifspec;

#ifndef __ITfThreadMgr2_INTERFACE_DEFINED__
#define __ITfThreadMgr2_INTERFACE_DEFINED__

/* interface ITfThreadMgr2 */
/* [unique][uuid][object] */ 


EXTERN_C const IID IID_ITfThreadMgr2;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("0ab198ef-6477-4ee8-8812-6780edb82d5e")
    ITfThreadMgr2 : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE Activate( 
            /* [out] */ __RPC__out TfClientId *ptid) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Deactivate( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE CreateDocumentMgr( 
            /* [out] */ __RPC__deref_out_opt ITfDocumentMgr **ppdim) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE EnumDocumentMgrs( 
            /* [out] */ __RPC__deref_out_opt IEnumTfDocumentMgrs **ppEnum) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetFocus( 
            /* [out] */ __RPC__deref_out_opt ITfDocumentMgr **ppdimFocus) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetFocus( 
            /* [in] */ __RPC__in_opt ITfDocumentMgr *pdimFocus) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE IsThreadFocus( 
            /* [out] */ __RPC__out BOOL *pfThreadFocus) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetFunctionProvider( 
            /* [in] */ __RPC__in REFCLSID clsid,
            /* [out] */ __RPC__deref_out_opt ITfFunctionProvider **ppFuncProv) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE EnumFunctionProviders( 
            /* [out] */ __RPC__deref_out_opt IEnumTfFunctionProviders **ppEnum) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetGlobalCompartment( 
            /* [out] */ __RPC__deref_out_opt ITfCompartmentMgr **ppCompMgr) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE ActivateEx( 
            /* [out] */ __RPC__out TfClientId *ptid,
            /* [in] */ DWORD dwFlags) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetActiveFlags( 
            /* [out] */ __RPC__out DWORD *lpdwFlags) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SuspendKeystrokeHandling( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE ResumeKeystrokeHandling( void) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ITfThreadMgr2Vtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in ITfThreadMgr2 * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in ITfThreadMgr2 * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in ITfThreadMgr2 * This);
        
        DECLSPEC_XFGVIRT(ITfThreadMgr2, Activate)
        HRESULT ( STDMETHODCALLTYPE *Activate )( 
            __RPC__in ITfThreadMgr2 * This,
            /* [out] */ __RPC__out TfClientId *ptid);
        
        DECLSPEC_XFGVIRT(ITfThreadMgr2, Deactivate)
        HRESULT ( STDMETHODCALLTYPE *Deactivate )( 
            __RPC__in ITfThreadMgr2 * This);
        
        DECLSPEC_XFGVIRT(ITfThreadMgr2, CreateDocumentMgr)
        HRESULT ( STDMETHODCALLTYPE *CreateDocumentMgr )( 
            __RPC__in ITfThreadMgr2 * This,
            /* [out] */ __RPC__deref_out_opt ITfDocumentMgr **ppdim);
        
        DECLSPEC_XFGVIRT(ITfThreadMgr2, EnumDocumentMgrs)
        HRESULT ( STDMETHODCALLTYPE *EnumDocumentMgrs )( 
            __RPC__in ITfThreadMgr2 * This,
            /* [out] */ __RPC__deref_out_opt IEnumTfDocumentMgrs **ppEnum);
        
        DECLSPEC_XFGVIRT(ITfThreadMgr2, GetFocus)
        HRESULT ( STDMETHODCALLTYPE *GetFocus )( 
            __RPC__in ITfThreadMgr2 * This,
            /* [out] */ __RPC__deref_out_opt ITfDocumentMgr **ppdimFocus);
        
        DECLSPEC_XFGVIRT(ITfThreadMgr2, SetFocus)
        HRESULT ( STDMETHODCALLTYPE *SetFocus )( 
            __RPC__in ITfThreadMgr2 * This,
            /* [in] */ __RPC__in_opt ITfDocumentMgr *pdimFocus);
        
        DECLSPEC_XFGVIRT(ITfThreadMgr2, IsThreadFocus)
        HRESULT ( STDMETHODCALLTYPE *IsThreadFocus )( 
            __RPC__in ITfThreadMgr2 * This,
            /* [out] */ __RPC__out BOOL *pfThreadFocus);
        
        DECLSPEC_XFGVIRT(ITfThreadMgr2, GetFunctionProvider)
        HRESULT ( STDMETHODCALLTYPE *GetFunctionProvider )( 
            __RPC__in ITfThreadMgr2 * This,
            /* [in] */ __RPC__in REFCLSID clsid,
            /* [out] */ __RPC__deref_out_opt ITfFunctionProvider **ppFuncProv);
        
        DECLSPEC_XFGVIRT(ITfThreadMgr2, EnumFunctionProviders)
        HRESULT ( STDMETHODCALLTYPE *EnumFunctionProviders )( 
            __RPC__in ITfThreadMgr2 * This,
            /* [out] */ __RPC__deref_out_opt IEnumTfFunctionProviders **ppEnum);
        
        DECLSPEC_XFGVIRT(ITfThreadMgr2, GetGlobalCompartment)
        HRESULT ( STDMETHODCALLTYPE *GetGlobalCompartment )( 
            __RPC__in ITfThreadMgr2 * This,
            /* [out] */ __RPC__deref_out_opt ITfCompartmentMgr **ppCompMgr);
        
        DECLSPEC_XFGVIRT(ITfThreadMgr2, ActivateEx)
        HRESULT ( STDMETHODCALLTYPE *ActivateEx )( 
            __RPC__in ITfThreadMgr2 * This,
            /* [out] */ __RPC__out TfClientId *ptid,
            /* [in] */ DWORD dwFlags);
        
        DECLSPEC_XFGVIRT(ITfThreadMgr2, GetActiveFlags)
        HRESULT ( STDMETHODCALLTYPE *GetActiveFlags )( 
            __RPC__in ITfThreadMgr2 * This,
            /* [out] */ __RPC__out DWORD *lpdwFlags);
        
        DECLSPEC_XFGVIRT(ITfThreadMgr2, SuspendKeystrokeHandling)
        HRESULT ( STDMETHODCALLTYPE *SuspendKeystrokeHandling )( 
            __RPC__in ITfThreadMgr2 * This);
        
        DECLSPEC_XFGVIRT(ITfThreadMgr2, ResumeKeystrokeHandling)
        HRESULT ( STDMETHODCALLTYPE *ResumeKeystrokeHandling )( 
            __RPC__in ITfThreadMgr2 * This);
        
        END_INTERFACE
    } ITfThreadMgr2Vtbl;

    interface ITfThreadMgr2
    {
        CONST_VTBL struct ITfThreadMgr2Vtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ITfThreadMgr2_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ITfThreadMgr2_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ITfThreadMgr2_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ITfThreadMgr2_Activate(This,ptid)	\
    ( (This)->lpVtbl -> Activate(This,ptid) ) 

#define ITfThreadMgr2_Deactivate(This)	\
    ( (This)->lpVtbl -> Deactivate(This) ) 

#define ITfThreadMgr2_CreateDocumentMgr(This,ppdim)	\
    ( (This)->lpVtbl -> CreateDocumentMgr(This,ppdim) ) 

#define ITfThreadMgr2_EnumDocumentMgrs(This,ppEnum)	\
    ( (This)->lpVtbl -> EnumDocumentMgrs(This,ppEnum) ) 

#define ITfThreadMgr2_GetFocus(This,ppdimFocus)	\
    ( (This)->lpVtbl -> GetFocus(This,ppdimFocus) ) 

#define ITfThreadMgr2_SetFocus(This,pdimFocus)	\
    ( (This)->lpVtbl -> SetFocus(This,pdimFocus) ) 

#define ITfThreadMgr2_IsThreadFocus(This,pfThreadFocus)	\
    ( (This)->lpVtbl -> IsThreadFocus(This,pfThreadFocus) ) 

#define ITfThreadMgr2_GetFunctionProvider(This,clsid,ppFuncProv)	\
    ( (This)->lpVtbl -> GetFunctionProvider(This,clsid,ppFuncProv) ) 

#define ITfThreadMgr2_EnumFunctionProviders(This,ppEnum)	\
    ( (This)->lpVtbl -> EnumFunctionProviders(This,ppEnum) ) 

#define ITfThreadMgr2_GetGlobalCompartment(This,ppCompMgr)	\
    ( (This)->lpVtbl -> GetGlobalCompartment(This,ppCompMgr) ) 

#define ITfThreadMgr2_ActivateEx(This,ptid,dwFlags)	\
    ( (This)->lpVtbl -> ActivateEx(This,ptid,dwFlags) ) 

#define ITfThreadMgr2_GetActiveFlags(This,lpdwFlags)	\
    ( (This)->lpVtbl -> GetActiveFlags(This,lpdwFlags) ) 

#define ITfThreadMgr2_SuspendKeystrokeHandling(This)	\
    ( (This)->lpVtbl -> SuspendKeystrokeHandling(This) ) 

#define ITfThreadMgr2_ResumeKeystrokeHandling(This)	\
    ( (This)->lpVtbl -> ResumeKeystrokeHandling(This) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ITfThreadMgr2_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_msctf_0000_0003 */
/* [local] */ 

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP) */
#pragma endregion
#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)


extern RPC_IF_HANDLE __MIDL_itf_msctf_0000_0003_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_msctf_0000_0003_v0_0_s_ifspec;

#ifndef __ITfThreadMgrEventSink_INTERFACE_DEFINED__
#define __ITfThreadMgrEventSink_INTERFACE_DEFINED__

/* interface ITfThreadMgrEventSink */
/* [unique][uuid][object] */ 


EXTERN_C const IID IID_ITfThreadMgrEventSink;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("aa80e80e-2021-11d2-93e0-0060b067b86e")
    ITfThreadMgrEventSink : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE OnInitDocumentMgr( 
            /* [in] */ __RPC__in_opt ITfDocumentMgr *pdim) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE OnUninitDocumentMgr( 
            /* [in] */ __RPC__in_opt ITfDocumentMgr *pdim) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE OnSetFocus( 
            /* [in] */ __RPC__in_opt ITfDocumentMgr *pdimFocus,
            /* [in] */ __RPC__in_opt ITfDocumentMgr *pdimPrevFocus) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE OnPushContext( 
            /* [in] */ __RPC__in_opt ITfContext *pic) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE OnPopContext( 
            /* [in] */ __RPC__in_opt ITfContext *pic) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ITfThreadMgrEventSinkVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in ITfThreadMgrEventSink * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in ITfThreadMgrEventSink * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in ITfThreadMgrEventSink * This);
        
        DECLSPEC_XFGVIRT(ITfThreadMgrEventSink, OnInitDocumentMgr)
        HRESULT ( STDMETHODCALLTYPE *OnInitDocumentMgr )( 
            __RPC__in ITfThreadMgrEventSink * This,
            /* [in] */ __RPC__in_opt ITfDocumentMgr *pdim);
        
        DECLSPEC_XFGVIRT(ITfThreadMgrEventSink, OnUninitDocumentMgr)
        HRESULT ( STDMETHODCALLTYPE *OnUninitDocumentMgr )( 
            __RPC__in ITfThreadMgrEventSink * This,
            /* [in] */ __RPC__in_opt ITfDocumentMgr *pdim);
        
        DECLSPEC_XFGVIRT(ITfThreadMgrEventSink, OnSetFocus)
        HRESULT ( STDMETHODCALLTYPE *OnSetFocus )( 
            __RPC__in ITfThreadMgrEventSink * This,
            /* [in] */ __RPC__in_opt ITfDocumentMgr *pdimFocus,
            /* [in] */ __RPC__in_opt ITfDocumentMgr *pdimPrevFocus);
        
        DECLSPEC_XFGVIRT(ITfThreadMgrEventSink, OnPushContext)
        HRESULT ( STDMETHODCALLTYPE *OnPushContext )( 
            __RPC__in ITfThreadMgrEventSink * This,
            /* [in] */ __RPC__in_opt ITfContext *pic);
        
        DECLSPEC_XFGVIRT(ITfThreadMgrEventSink, OnPopContext)
        HRESULT ( STDMETHODCALLTYPE *OnPopContext )( 
            __RPC__in ITfThreadMgrEventSink * This,
            /* [in] */ __RPC__in_opt ITfContext *pic);
        
        END_INTERFACE
    } ITfThreadMgrEventSinkVtbl;

    interface ITfThreadMgrEventSink
    {
        CONST_VTBL struct ITfThreadMgrEventSinkVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ITfThreadMgrEventSink_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ITfThreadMgrEventSink_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ITfThreadMgrEventSink_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ITfThreadMgrEventSink_OnInitDocumentMgr(This,pdim)	\
    ( (This)->lpVtbl -> OnInitDocumentMgr(This,pdim) ) 

#define ITfThreadMgrEventSink_OnUninitDocumentMgr(This,pdim)	\
    ( (This)->lpVtbl -> OnUninitDocumentMgr(This,pdim) ) 

#define ITfThreadMgrEventSink_OnSetFocus(This,pdimFocus,pdimPrevFocus)	\
    ( (This)->lpVtbl -> OnSetFocus(This,pdimFocus,pdimPrevFocus) ) 

#define ITfThreadMgrEventSink_OnPushContext(This,pic)	\
    ( (This)->lpVtbl -> OnPushContext(This,pic) ) 

#define ITfThreadMgrEventSink_OnPopContext(This,pic)	\
    ( (This)->lpVtbl -> OnPopContext(This,pic) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ITfThreadMgrEventSink_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_msctf_0000_0004 */
/* [local] */ 

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion
#pragma region Application Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP)


extern RPC_IF_HANDLE __MIDL_itf_msctf_0000_0004_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_msctf_0000_0004_v0_0_s_ifspec;

#ifndef __ITfConfigureSystemKeystrokeFeed_INTERFACE_DEFINED__
#define __ITfConfigureSystemKeystrokeFeed_INTERFACE_DEFINED__

/* interface ITfConfigureSystemKeystrokeFeed */
/* [unique][uuid][local][object] */ 


EXTERN_C const IID IID_ITfConfigureSystemKeystrokeFeed;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("0d2c969a-bc9c-437c-84ee-951c49b1a764")
    ITfConfigureSystemKeystrokeFeed : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE DisableSystemKeystrokeFeed( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE EnableSystemKeystrokeFeed( void) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ITfConfigureSystemKeystrokeFeedVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            ITfConfigureSystemKeystrokeFeed * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            ITfConfigureSystemKeystrokeFeed * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            ITfConfigureSystemKeystrokeFeed * This);
        
        DECLSPEC_XFGVIRT(ITfConfigureSystemKeystrokeFeed, DisableSystemKeystrokeFeed)
        HRESULT ( STDMETHODCALLTYPE *DisableSystemKeystrokeFeed )( 
            ITfConfigureSystemKeystrokeFeed * This);
        
        DECLSPEC_XFGVIRT(ITfConfigureSystemKeystrokeFeed, EnableSystemKeystrokeFeed)
        HRESULT ( STDMETHODCALLTYPE *EnableSystemKeystrokeFeed )( 
            ITfConfigureSystemKeystrokeFeed * This);
        
        END_INTERFACE
    } ITfConfigureSystemKeystrokeFeedVtbl;

    interface ITfConfigureSystemKeystrokeFeed
    {
        CONST_VTBL struct ITfConfigureSystemKeystrokeFeedVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ITfConfigureSystemKeystrokeFeed_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ITfConfigureSystemKeystrokeFeed_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ITfConfigureSystemKeystrokeFeed_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ITfConfigureSystemKeystrokeFeed_DisableSystemKeystrokeFeed(This)	\
    ( (This)->lpVtbl -> DisableSystemKeystrokeFeed(This) ) 

#define ITfConfigureSystemKeystrokeFeed_EnableSystemKeystrokeFeed(This)	\
    ( (This)->lpVtbl -> EnableSystemKeystrokeFeed(This) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ITfConfigureSystemKeystrokeFeed_INTERFACE_DEFINED__ */


#ifndef __IEnumTfDocumentMgrs_INTERFACE_DEFINED__
#define __IEnumTfDocumentMgrs_INTERFACE_DEFINED__

/* interface IEnumTfDocumentMgrs */
/* [unique][uuid][object] */ 


EXTERN_C const IID IID_IEnumTfDocumentMgrs;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("aa80e808-2021-11d2-93e0-0060b067b86e")
    IEnumTfDocumentMgrs : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE Clone( 
            /* [out] */ __RPC__deref_out_opt IEnumTfDocumentMgrs **ppEnum) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Next( 
            /* [in] */ ULONG ulCount,
            /* [length_is][size_is][out] */ __RPC__out_ecount_part(ulCount, *pcFetched) ITfDocumentMgr **rgDocumentMgr,
            /* [out] */ __RPC__out ULONG *pcFetched) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Reset( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Skip( 
            /* [in] */ ULONG ulCount) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IEnumTfDocumentMgrsVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IEnumTfDocumentMgrs * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IEnumTfDocumentMgrs * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IEnumTfDocumentMgrs * This);
        
        DECLSPEC_XFGVIRT(IEnumTfDocumentMgrs, Clone)
        HRESULT ( STDMETHODCALLTYPE *Clone )( 
            __RPC__in IEnumTfDocumentMgrs * This,
            /* [out] */ __RPC__deref_out_opt IEnumTfDocumentMgrs **ppEnum);
        
        DECLSPEC_XFGVIRT(IEnumTfDocumentMgrs, Next)
        HRESULT ( STDMETHODCALLTYPE *Next )( 
            __RPC__in IEnumTfDocumentMgrs * This,
            /* [in] */ ULONG ulCount,
            /* [length_is][size_is][out] */ __RPC__out_ecount_part(ulCount, *pcFetched) ITfDocumentMgr **rgDocumentMgr,
            /* [out] */ __RPC__out ULONG *pcFetched);
        
        DECLSPEC_XFGVIRT(IEnumTfDocumentMgrs, Reset)
        HRESULT ( STDMETHODCALLTYPE *Reset )( 
            __RPC__in IEnumTfDocumentMgrs * This);
        
        DECLSPEC_XFGVIRT(IEnumTfDocumentMgrs, Skip)
        HRESULT ( STDMETHODCALLTYPE *Skip )( 
            __RPC__in IEnumTfDocumentMgrs * This,
            /* [in] */ ULONG ulCount);
        
        END_INTERFACE
    } IEnumTfDocumentMgrsVtbl;

    interface IEnumTfDocumentMgrs
    {
        CONST_VTBL struct IEnumTfDocumentMgrsVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IEnumTfDocumentMgrs_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IEnumTfDocumentMgrs_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IEnumTfDocumentMgrs_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IEnumTfDocumentMgrs_Clone(This,ppEnum)	\
    ( (This)->lpVtbl -> Clone(This,ppEnum) ) 

#define IEnumTfDocumentMgrs_Next(This,ulCount,rgDocumentMgr,pcFetched)	\
    ( (This)->lpVtbl -> Next(This,ulCount,rgDocumentMgr,pcFetched) ) 

#define IEnumTfDocumentMgrs_Reset(This)	\
    ( (This)->lpVtbl -> Reset(This) ) 

#define IEnumTfDocumentMgrs_Skip(This,ulCount)	\
    ( (This)->lpVtbl -> Skip(This,ulCount) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IEnumTfDocumentMgrs_INTERFACE_DEFINED__ */


#ifndef __ITfDocumentMgr_INTERFACE_DEFINED__
#define __ITfDocumentMgr_INTERFACE_DEFINED__

/* interface ITfDocumentMgr */
/* [unique][uuid][object] */ 

#define	TF_POPF_ALL	( 0x1 )


EXTERN_C const IID IID_ITfDocumentMgr;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("aa80e7f4-2021-11d2-93e0-0060b067b86e")
    ITfDocumentMgr : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE CreateContext( 
            /* [in] */ TfClientId tidOwner,
            /* [in] */ DWORD dwFlags,
            /* [unique][in] */ __RPC__in_opt IUnknown *punk,
            /* [out] */ __RPC__deref_out_opt ITfContext **ppic,
            /* [out] */ __RPC__out TfEditCookie *pecTextStore) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Push( 
            /* [in] */ __RPC__in_opt ITfContext *pic) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Pop( 
            /* [in] */ DWORD dwFlags) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetTop( 
            /* [out] */ __RPC__deref_out_opt ITfContext **ppic) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetBase( 
            /* [out] */ __RPC__deref_out_opt ITfContext **ppic) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE EnumContexts( 
            /* [out] */ __RPC__deref_out_opt IEnumTfContexts **ppEnum) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ITfDocumentMgrVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in ITfDocumentMgr * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in ITfDocumentMgr * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in ITfDocumentMgr * This);
        
        DECLSPEC_XFGVIRT(ITfDocumentMgr, CreateContext)
        HRESULT ( STDMETHODCALLTYPE *CreateContext )( 
            __RPC__in ITfDocumentMgr * This,
            /* [in] */ TfClientId tidOwner,
            /* [in] */ DWORD dwFlags,
            /* [unique][in] */ __RPC__in_opt IUnknown *punk,
            /* [out] */ __RPC__deref_out_opt ITfContext **ppic,
            /* [out] */ __RPC__out TfEditCookie *pecTextStore);
        
        DECLSPEC_XFGVIRT(ITfDocumentMgr, Push)
        HRESULT ( STDMETHODCALLTYPE *Push )( 
            __RPC__in ITfDocumentMgr * This,
            /* [in] */ __RPC__in_opt ITfContext *pic);
        
        DECLSPEC_XFGVIRT(ITfDocumentMgr, Pop)
        HRESULT ( STDMETHODCALLTYPE *Pop )( 
            __RPC__in ITfDocumentMgr * This,
            /* [in] */ DWORD dwFlags);
        
        DECLSPEC_XFGVIRT(ITfDocumentMgr, GetTop)
        HRESULT ( STDMETHODCALLTYPE *GetTop )( 
            __RPC__in ITfDocumentMgr * This,
            /* [out] */ __RPC__deref_out_opt ITfContext **ppic);
        
        DECLSPEC_XFGVIRT(ITfDocumentMgr, GetBase)
        HRESULT ( STDMETHODCALLTYPE *GetBase )( 
            __RPC__in ITfDocumentMgr * This,
            /* [out] */ __RPC__deref_out_opt ITfContext **ppic);
        
        DECLSPEC_XFGVIRT(ITfDocumentMgr, EnumContexts)
        HRESULT ( STDMETHODCALLTYPE *EnumContexts )( 
            __RPC__in ITfDocumentMgr * This,
            /* [out] */ __RPC__deref_out_opt IEnumTfContexts **ppEnum);
        
        END_INTERFACE
    } ITfDocumentMgrVtbl;

    interface ITfDocumentMgr
    {
        CONST_VTBL struct ITfDocumentMgrVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ITfDocumentMgr_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ITfDocumentMgr_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ITfDocumentMgr_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ITfDocumentMgr_CreateContext(This,tidOwner,dwFlags,punk,ppic,pecTextStore)	\
    ( (This)->lpVtbl -> CreateContext(This,tidOwner,dwFlags,punk,ppic,pecTextStore) ) 

#define ITfDocumentMgr_Push(This,pic)	\
    ( (This)->lpVtbl -> Push(This,pic) ) 

#define ITfDocumentMgr_Pop(This,dwFlags)	\
    ( (This)->lpVtbl -> Pop(This,dwFlags) ) 

#define ITfDocumentMgr_GetTop(This,ppic)	\
    ( (This)->lpVtbl -> GetTop(This,ppic) ) 

#define ITfDocumentMgr_GetBase(This,ppic)	\
    ( (This)->lpVtbl -> GetBase(This,ppic) ) 

#define ITfDocumentMgr_EnumContexts(This,ppEnum)	\
    ( (This)->lpVtbl -> EnumContexts(This,ppEnum) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ITfDocumentMgr_INTERFACE_DEFINED__ */


#ifndef __IEnumTfContexts_INTERFACE_DEFINED__
#define __IEnumTfContexts_INTERFACE_DEFINED__

/* interface IEnumTfContexts */
/* [unique][uuid][object] */ 


EXTERN_C const IID IID_IEnumTfContexts;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("8f1a7ea6-1654-4502-a86e-b2902344d507")
    IEnumTfContexts : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE Clone( 
            /* [out] */ __RPC__deref_out_opt IEnumTfContexts **ppEnum) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Next( 
            /* [in] */ ULONG ulCount,
            /* [length_is][size_is][out] */ __RPC__out_ecount_part(ulCount, *pcFetched) ITfContext **rgContext,
            /* [out] */ __RPC__out ULONG *pcFetched) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Reset( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Skip( 
            /* [in] */ ULONG ulCount) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IEnumTfContextsVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IEnumTfContexts * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IEnumTfContexts * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IEnumTfContexts * This);
        
        DECLSPEC_XFGVIRT(IEnumTfContexts, Clone)
        HRESULT ( STDMETHODCALLTYPE *Clone )( 
            __RPC__in IEnumTfContexts * This,
            /* [out] */ __RPC__deref_out_opt IEnumTfContexts **ppEnum);
        
        DECLSPEC_XFGVIRT(IEnumTfContexts, Next)
        HRESULT ( STDMETHODCALLTYPE *Next )( 
            __RPC__in IEnumTfContexts * This,
            /* [in] */ ULONG ulCount,
            /* [length_is][size_is][out] */ __RPC__out_ecount_part(ulCount, *pcFetched) ITfContext **rgContext,
            /* [out] */ __RPC__out ULONG *pcFetched);
        
        DECLSPEC_XFGVIRT(IEnumTfContexts, Reset)
        HRESULT ( STDMETHODCALLTYPE *Reset )( 
            __RPC__in IEnumTfContexts * This);
        
        DECLSPEC_XFGVIRT(IEnumTfContexts, Skip)
        HRESULT ( STDMETHODCALLTYPE *Skip )( 
            __RPC__in IEnumTfContexts * This,
            /* [in] */ ULONG ulCount);
        
        END_INTERFACE
    } IEnumTfContextsVtbl;

    interface IEnumTfContexts
    {
        CONST_VTBL struct IEnumTfContextsVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IEnumTfContexts_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IEnumTfContexts_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IEnumTfContexts_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IEnumTfContexts_Clone(This,ppEnum)	\
    ( (This)->lpVtbl -> Clone(This,ppEnum) ) 

#define IEnumTfContexts_Next(This,ulCount,rgContext,pcFetched)	\
    ( (This)->lpVtbl -> Next(This,ulCount,rgContext,pcFetched) ) 

#define IEnumTfContexts_Reset(This)	\
    ( (This)->lpVtbl -> Reset(This) ) 

#define IEnumTfContexts_Skip(This,ulCount)	\
    ( (This)->lpVtbl -> Skip(This,ulCount) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IEnumTfContexts_INTERFACE_DEFINED__ */


#ifndef __ITfCompositionView_INTERFACE_DEFINED__
#define __ITfCompositionView_INTERFACE_DEFINED__

/* interface ITfCompositionView */
/* [unique][uuid][object] */ 


EXTERN_C const IID IID_ITfCompositionView;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("D7540241-F9A1-4364-BEFC-DBCD2C4395B7")
    ITfCompositionView : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetOwnerClsid( 
            /* [out] */ __RPC__out CLSID *pclsid) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetRange( 
            /* [out] */ __RPC__deref_out_opt ITfRange **ppRange) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ITfCompositionViewVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in ITfCompositionView * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in ITfCompositionView * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in ITfCompositionView * This);
        
        DECLSPEC_XFGVIRT(ITfCompositionView, GetOwnerClsid)
        HRESULT ( STDMETHODCALLTYPE *GetOwnerClsid )( 
            __RPC__in ITfCompositionView * This,
            /* [out] */ __RPC__out CLSID *pclsid);
        
        DECLSPEC_XFGVIRT(ITfCompositionView, GetRange)
        HRESULT ( STDMETHODCALLTYPE *GetRange )( 
            __RPC__in ITfCompositionView * This,
            /* [out] */ __RPC__deref_out_opt ITfRange **ppRange);
        
        END_INTERFACE
    } ITfCompositionViewVtbl;

    interface ITfCompositionView
    {
        CONST_VTBL struct ITfCompositionViewVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ITfCompositionView_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ITfCompositionView_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ITfCompositionView_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ITfCompositionView_GetOwnerClsid(This,pclsid)	\
    ( (This)->lpVtbl -> GetOwnerClsid(This,pclsid) ) 

#define ITfCompositionView_GetRange(This,ppRange)	\
    ( (This)->lpVtbl -> GetRange(This,ppRange) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ITfCompositionView_INTERFACE_DEFINED__ */


#ifndef __IEnumITfCompositionView_INTERFACE_DEFINED__
#define __IEnumITfCompositionView_INTERFACE_DEFINED__

/* interface IEnumITfCompositionView */
/* [unique][uuid][object] */ 


EXTERN_C const IID IID_IEnumITfCompositionView;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("5EFD22BA-7838-46CB-88E2-CADB14124F8F")
    IEnumITfCompositionView : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE Clone( 
            /* [out] */ __RPC__deref_out_opt IEnumITfCompositionView **ppEnum) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Next( 
            /* [in] */ ULONG ulCount,
            /* [length_is][size_is][out] */ __RPC__out_ecount_part(ulCount, *pcFetched) ITfCompositionView **rgCompositionView,
            /* [out] */ __RPC__out ULONG *pcFetched) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Reset( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Skip( 
            /* [in] */ ULONG ulCount) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IEnumITfCompositionViewVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IEnumITfCompositionView * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IEnumITfCompositionView * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IEnumITfCompositionView * This);
        
        DECLSPEC_XFGVIRT(IEnumITfCompositionView, Clone)
        HRESULT ( STDMETHODCALLTYPE *Clone )( 
            __RPC__in IEnumITfCompositionView * This,
            /* [out] */ __RPC__deref_out_opt IEnumITfCompositionView **ppEnum);
        
        DECLSPEC_XFGVIRT(IEnumITfCompositionView, Next)
        HRESULT ( STDMETHODCALLTYPE *Next )( 
            __RPC__in IEnumITfCompositionView * This,
            /* [in] */ ULONG ulCount,
            /* [length_is][size_is][out] */ __RPC__out_ecount_part(ulCount, *pcFetched) ITfCompositionView **rgCompositionView,
            /* [out] */ __RPC__out ULONG *pcFetched);
        
        DECLSPEC_XFGVIRT(IEnumITfCompositionView, Reset)
        HRESULT ( STDMETHODCALLTYPE *Reset )( 
            __RPC__in IEnumITfCompositionView * This);
        
        DECLSPEC_XFGVIRT(IEnumITfCompositionView, Skip)
        HRESULT ( STDMETHODCALLTYPE *Skip )( 
            __RPC__in IEnumITfCompositionView * This,
            /* [in] */ ULONG ulCount);
        
        END_INTERFACE
    } IEnumITfCompositionViewVtbl;

    interface IEnumITfCompositionView
    {
        CONST_VTBL struct IEnumITfCompositionViewVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IEnumITfCompositionView_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IEnumITfCompositionView_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IEnumITfCompositionView_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IEnumITfCompositionView_Clone(This,ppEnum)	\
    ( (This)->lpVtbl -> Clone(This,ppEnum) ) 

#define IEnumITfCompositionView_Next(This,ulCount,rgCompositionView,pcFetched)	\
    ( (This)->lpVtbl -> Next(This,ulCount,rgCompositionView,pcFetched) ) 

#define IEnumITfCompositionView_Reset(This)	\
    ( (This)->lpVtbl -> Reset(This) ) 

#define IEnumITfCompositionView_Skip(This,ulCount)	\
    ( (This)->lpVtbl -> Skip(This,ulCount) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IEnumITfCompositionView_INTERFACE_DEFINED__ */


#ifndef __ITfComposition_INTERFACE_DEFINED__
#define __ITfComposition_INTERFACE_DEFINED__

/* interface ITfComposition */
/* [unique][uuid][object] */ 


EXTERN_C const IID IID_ITfComposition;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("20168D64-5A8F-4A5A-B7BD-CFA29F4D0FD9")
    ITfComposition : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetRange( 
            /* [out] */ __RPC__deref_out_opt ITfRange **ppRange) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE ShiftStart( 
            /* [in] */ TfEditCookie ecWrite,
            /* [in] */ __RPC__in_opt ITfRange *pNewStart) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE ShiftEnd( 
            /* [in] */ TfEditCookie ecWrite,
            /* [in] */ __RPC__in_opt ITfRange *pNewEnd) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE EndComposition( 
            /* [in] */ TfEditCookie ecWrite) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ITfCompositionVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in ITfComposition * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in ITfComposition * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in ITfComposition * This);
        
        DECLSPEC_XFGVIRT(ITfComposition, GetRange)
        HRESULT ( STDMETHODCALLTYPE *GetRange )( 
            __RPC__in ITfComposition * This,
            /* [out] */ __RPC__deref_out_opt ITfRange **ppRange);
        
        DECLSPEC_XFGVIRT(ITfComposition, ShiftStart)
        HRESULT ( STDMETHODCALLTYPE *ShiftStart )( 
            __RPC__in ITfComposition * This,
            /* [in] */ TfEditCookie ecWrite,
            /* [in] */ __RPC__in_opt ITfRange *pNewStart);
        
        DECLSPEC_XFGVIRT(ITfComposition, ShiftEnd)
        HRESULT ( STDMETHODCALLTYPE *ShiftEnd )( 
            __RPC__in ITfComposition * This,
            /* [in] */ TfEditCookie ecWrite,
            /* [in] */ __RPC__in_opt ITfRange *pNewEnd);
        
        DECLSPEC_XFGVIRT(ITfComposition, EndComposition)
        HRESULT ( STDMETHODCALLTYPE *EndComposition )( 
            __RPC__in ITfComposition * This,
            /* [in] */ TfEditCookie ecWrite);
        
        END_INTERFACE
    } ITfCompositionVtbl;

    interface ITfComposition
    {
        CONST_VTBL struct ITfCompositionVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ITfComposition_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ITfComposition_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ITfComposition_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ITfComposition_GetRange(This,ppRange)	\
    ( (This)->lpVtbl -> GetRange(This,ppRange) ) 

#define ITfComposition_ShiftStart(This,ecWrite,pNewStart)	\
    ( (This)->lpVtbl -> ShiftStart(This,ecWrite,pNewStart) ) 

#define ITfComposition_ShiftEnd(This,ecWrite,pNewEnd)	\
    ( (This)->lpVtbl -> ShiftEnd(This,ecWrite,pNewEnd) ) 

#define ITfComposition_EndComposition(This,ecWrite)	\
    ( (This)->lpVtbl -> EndComposition(This,ecWrite) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ITfComposition_INTERFACE_DEFINED__ */


#ifndef __ITfCompositionSink_INTERFACE_DEFINED__
#define __ITfCompositionSink_INTERFACE_DEFINED__

/* interface ITfCompositionSink */
/* [unique][uuid][object] */ 


EXTERN_C const IID IID_ITfCompositionSink;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("A781718C-579A-4B15-A280-32B8577ACC5E")
    ITfCompositionSink : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE OnCompositionTerminated( 
            /* [in] */ TfEditCookie ecWrite,
            /* [in] */ __RPC__in_opt ITfComposition *pComposition) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ITfCompositionSinkVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in ITfCompositionSink * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in ITfCompositionSink * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in ITfCompositionSink * This);
        
        DECLSPEC_XFGVIRT(ITfCompositionSink, OnCompositionTerminated)
        HRESULT ( STDMETHODCALLTYPE *OnCompositionTerminated )( 
            __RPC__in ITfCompositionSink * This,
            /* [in] */ TfEditCookie ecWrite,
            /* [in] */ __RPC__in_opt ITfComposition *pComposition);
        
        END_INTERFACE
    } ITfCompositionSinkVtbl;

    interface ITfCompositionSink
    {
        CONST_VTBL struct ITfCompositionSinkVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ITfCompositionSink_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ITfCompositionSink_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ITfCompositionSink_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ITfCompositionSink_OnCompositionTerminated(This,ecWrite,pComposition)	\
    ( (This)->lpVtbl -> OnCompositionTerminated(This,ecWrite,pComposition) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ITfCompositionSink_INTERFACE_DEFINED__ */


#ifndef __ITfContextComposition_INTERFACE_DEFINED__
#define __ITfContextComposition_INTERFACE_DEFINED__

/* interface ITfContextComposition */
/* [unique][uuid][object] */ 


EXTERN_C const IID IID_ITfContextComposition;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("D40C8AAE-AC92-4FC7-9A11-0EE0E23AA39B")
    ITfContextComposition : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE StartComposition( 
            /* [in] */ TfEditCookie ecWrite,
            /* [in] */ __RPC__in_opt ITfRange *pCompositionRange,
            /* [in] */ __RPC__in_opt ITfCompositionSink *pSink,
            /* [out] */ __RPC__deref_out_opt ITfComposition **ppComposition) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE EnumCompositions( 
            /* [out] */ __RPC__deref_out_opt IEnumITfCompositionView **ppEnum) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE FindComposition( 
            /* [in] */ TfEditCookie ecRead,
            /* [in] */ __RPC__in_opt ITfRange *pTestRange,
            /* [out] */ __RPC__deref_out_opt IEnumITfCompositionView **ppEnum) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE TakeOwnership( 
            /* [in] */ TfEditCookie ecWrite,
            /* [in] */ __RPC__in_opt ITfCompositionView *pComposition,
            /* [in] */ __RPC__in_opt ITfCompositionSink *pSink,
            /* [out] */ __RPC__deref_out_opt ITfComposition **ppComposition) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ITfContextCompositionVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in ITfContextComposition * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in ITfContextComposition * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in ITfContextComposition * This);
        
        DECLSPEC_XFGVIRT(ITfContextComposition, StartComposition)
        HRESULT ( STDMETHODCALLTYPE *StartComposition )( 
            __RPC__in ITfContextComposition * This,
            /* [in] */ TfEditCookie ecWrite,
            /* [in] */ __RPC__in_opt ITfRange *pCompositionRange,
            /* [in] */ __RPC__in_opt ITfCompositionSink *pSink,
            /* [out] */ __RPC__deref_out_opt ITfComposition **ppComposition);
        
        DECLSPEC_XFGVIRT(ITfContextComposition, EnumCompositions)
        HRESULT ( STDMETHODCALLTYPE *EnumCompositions )( 
            __RPC__in ITfContextComposition * This,
            /* [out] */ __RPC__deref_out_opt IEnumITfCompositionView **ppEnum);
        
        DECLSPEC_XFGVIRT(ITfContextComposition, FindComposition)
        HRESULT ( STDMETHODCALLTYPE *FindComposition )( 
            __RPC__in ITfContextComposition * This,
            /* [in] */ TfEditCookie ecRead,
            /* [in] */ __RPC__in_opt ITfRange *pTestRange,
            /* [out] */ __RPC__deref_out_opt IEnumITfCompositionView **ppEnum);
        
        DECLSPEC_XFGVIRT(ITfContextComposition, TakeOwnership)
        HRESULT ( STDMETHODCALLTYPE *TakeOwnership )( 
            __RPC__in ITfContextComposition * This,
            /* [in] */ TfEditCookie ecWrite,
            /* [in] */ __RPC__in_opt ITfCompositionView *pComposition,
            /* [in] */ __RPC__in_opt ITfCompositionSink *pSink,
            /* [out] */ __RPC__deref_out_opt ITfComposition **ppComposition);
        
        END_INTERFACE
    } ITfContextCompositionVtbl;

    interface ITfContextComposition
    {
        CONST_VTBL struct ITfContextCompositionVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ITfContextComposition_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ITfContextComposition_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ITfContextComposition_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ITfContextComposition_StartComposition(This,ecWrite,pCompositionRange,pSink,ppComposition)	\
    ( (This)->lpVtbl -> StartComposition(This,ecWrite,pCompositionRange,pSink,ppComposition) ) 

#define ITfContextComposition_EnumCompositions(This,ppEnum)	\
    ( (This)->lpVtbl -> EnumCompositions(This,ppEnum) ) 

#define ITfContextComposition_FindComposition(This,ecRead,pTestRange,ppEnum)	\
    ( (This)->lpVtbl -> FindComposition(This,ecRead,pTestRange,ppEnum) ) 

#define ITfContextComposition_TakeOwnership(This,ecWrite,pComposition,pSink,ppComposition)	\
    ( (This)->lpVtbl -> TakeOwnership(This,ecWrite,pComposition,pSink,ppComposition) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ITfContextComposition_INTERFACE_DEFINED__ */


#ifndef __ITfContextOwnerCompositionServices_INTERFACE_DEFINED__
#define __ITfContextOwnerCompositionServices_INTERFACE_DEFINED__

/* interface ITfContextOwnerCompositionServices */
/* [unique][uuid][object] */ 


EXTERN_C const IID IID_ITfContextOwnerCompositionServices;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("86462810-593B-4916-9764-19C08E9CE110")
    ITfContextOwnerCompositionServices : public ITfContextComposition
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE TerminateComposition( 
            /* [in] */ __RPC__in_opt ITfCompositionView *pComposition) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ITfContextOwnerCompositionServicesVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in ITfContextOwnerCompositionServices * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in ITfContextOwnerCompositionServices * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in ITfContextOwnerCompositionServices * This);
        
        DECLSPEC_XFGVIRT(ITfContextComposition, StartComposition)
        HRESULT ( STDMETHODCALLTYPE *StartComposition )( 
            __RPC__in ITfContextOwnerCompositionServices * This,
            /* [in] */ TfEditCookie ecWrite,
            /* [in] */ __RPC__in_opt ITfRange *pCompositionRange,
            /* [in] */ __RPC__in_opt ITfCompositionSink *pSink,
            /* [out] */ __RPC__deref_out_opt ITfComposition **ppComposition);
        
        DECLSPEC_XFGVIRT(ITfContextComposition, EnumCompositions)
        HRESULT ( STDMETHODCALLTYPE *EnumCompositions )( 
            __RPC__in ITfContextOwnerCompositionServices * This,
            /* [out] */ __RPC__deref_out_opt IEnumITfCompositionView **ppEnum);
        
        DECLSPEC_XFGVIRT(ITfContextComposition, FindComposition)
        HRESULT ( STDMETHODCALLTYPE *FindComposition )( 
            __RPC__in ITfContextOwnerCompositionServices * This,
            /* [in] */ TfEditCookie ecRead,
            /* [in] */ __RPC__in_opt ITfRange *pTestRange,
            /* [out] */ __RPC__deref_out_opt IEnumITfCompositionView **ppEnum);
        
        DECLSPEC_XFGVIRT(ITfContextComposition, TakeOwnership)
        HRESULT ( STDMETHODCALLTYPE *TakeOwnership )( 
            __RPC__in ITfContextOwnerCompositionServices * This,
            /* [in] */ TfEditCookie ecWrite,
            /* [in] */ __RPC__in_opt ITfCompositionView *pComposition,
            /* [in] */ __RPC__in_opt ITfCompositionSink *pSink,
            /* [out] */ __RPC__deref_out_opt ITfComposition **ppComposition);
        
        DECLSPEC_XFGVIRT(ITfContextOwnerCompositionServices, TerminateComposition)
        HRESULT ( STDMETHODCALLTYPE *TerminateComposition )( 
            __RPC__in ITfContextOwnerCompositionServices * This,
            /* [in] */ __RPC__in_opt ITfCompositionView *pComposition);
        
        END_INTERFACE
    } ITfContextOwnerCompositionServicesVtbl;

    interface ITfContextOwnerCompositionServices
    {
        CONST_VTBL struct ITfContextOwnerCompositionServicesVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ITfContextOwnerCompositionServices_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ITfContextOwnerCompositionServices_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ITfContextOwnerCompositionServices_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ITfContextOwnerCompositionServices_StartComposition(This,ecWrite,pCompositionRange,pSink,ppComposition)	\
    ( (This)->lpVtbl -> StartComposition(This,ecWrite,pCompositionRange,pSink,ppComposition) ) 

#define ITfContextOwnerCompositionServices_EnumCompositions(This,ppEnum)	\
    ( (This)->lpVtbl -> EnumCompositions(This,ppEnum) ) 

#define ITfContextOwnerCompositionServices_FindComposition(This,ecRead,pTestRange,ppEnum)	\
    ( (This)->lpVtbl -> FindComposition(This,ecRead,pTestRange,ppEnum) ) 

#define ITfContextOwnerCompositionServices_TakeOwnership(This,ecWrite,pComposition,pSink,ppComposition)	\
    ( (This)->lpVtbl -> TakeOwnership(This,ecWrite,pComposition,pSink,ppComposition) ) 


#define ITfContextOwnerCompositionServices_TerminateComposition(This,pComposition)	\
    ( (This)->lpVtbl -> TerminateComposition(This,pComposition) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ITfContextOwnerCompositionServices_INTERFACE_DEFINED__ */


#ifndef __ITfContextOwnerCompositionSink_INTERFACE_DEFINED__
#define __ITfContextOwnerCompositionSink_INTERFACE_DEFINED__

/* interface ITfContextOwnerCompositionSink */
/* [unique][uuid][object] */ 


EXTERN_C const IID IID_ITfContextOwnerCompositionSink;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("5F20AA40-B57A-4F34-96AB-3576F377CC79")
    ITfContextOwnerCompositionSink : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE OnStartComposition( 
            /* [in] */ __RPC__in_opt ITfCompositionView *pComposition,
            /* [out] */ __RPC__out BOOL *pfOk) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE OnUpdateComposition( 
            /* [in] */ __RPC__in_opt ITfCompositionView *pComposition,
            /* [in] */ __RPC__in_opt ITfRange *pRangeNew) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE OnEndComposition( 
            /* [in] */ __RPC__in_opt ITfCompositionView *pComposition) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ITfContextOwnerCompositionSinkVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in ITfContextOwnerCompositionSink * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in ITfContextOwnerCompositionSink * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in ITfContextOwnerCompositionSink * This);
        
        DECLSPEC_XFGVIRT(ITfContextOwnerCompositionSink, OnStartComposition)
        HRESULT ( STDMETHODCALLTYPE *OnStartComposition )( 
            __RPC__in ITfContextOwnerCompositionSink * This,
            /* [in] */ __RPC__in_opt ITfCompositionView *pComposition,
            /* [out] */ __RPC__out BOOL *pfOk);
        
        DECLSPEC_XFGVIRT(ITfContextOwnerCompositionSink, OnUpdateComposition)
        HRESULT ( STDMETHODCALLTYPE *OnUpdateComposition )( 
            __RPC__in ITfContextOwnerCompositionSink * This,
            /* [in] */ __RPC__in_opt ITfCompositionView *pComposition,
            /* [in] */ __RPC__in_opt ITfRange *pRangeNew);
        
        DECLSPEC_XFGVIRT(ITfContextOwnerCompositionSink, OnEndComposition)
        HRESULT ( STDMETHODCALLTYPE *OnEndComposition )( 
            __RPC__in ITfContextOwnerCompositionSink * This,
            /* [in] */ __RPC__in_opt ITfCompositionView *pComposition);
        
        END_INTERFACE
    } ITfContextOwnerCompositionSinkVtbl;

    interface ITfContextOwnerCompositionSink
    {
        CONST_VTBL struct ITfContextOwnerCompositionSinkVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ITfContextOwnerCompositionSink_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ITfContextOwnerCompositionSink_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ITfContextOwnerCompositionSink_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ITfContextOwnerCompositionSink_OnStartComposition(This,pComposition,pfOk)	\
    ( (This)->lpVtbl -> OnStartComposition(This,pComposition,pfOk) ) 

#define ITfContextOwnerCompositionSink_OnUpdateComposition(This,pComposition,pRangeNew)	\
    ( (This)->lpVtbl -> OnUpdateComposition(This,pComposition,pRangeNew) ) 

#define ITfContextOwnerCompositionSink_OnEndComposition(This,pComposition)	\
    ( (This)->lpVtbl -> OnEndComposition(This,pComposition) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ITfContextOwnerCompositionSink_INTERFACE_DEFINED__ */


#ifndef __ITfContextView_INTERFACE_DEFINED__
#define __ITfContextView_INTERFACE_DEFINED__

/* interface ITfContextView */
/* [unique][uuid][object] */ 


EXTERN_C const IID IID_ITfContextView;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("2433bf8e-0f9b-435c-ba2c-180611978c30")
    ITfContextView : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetRangeFromPoint( 
            /* [in] */ TfEditCookie ec,
            /* [in] */ __RPC__in const POINT *ppt,
            /* [in] */ DWORD dwFlags,
            /* [out] */ __RPC__deref_out_opt ITfRange **ppRange) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetTextExt( 
            /* [in] */ TfEditCookie ec,
            /* [in] */ __RPC__in_opt ITfRange *pRange,
            /* [out] */ __RPC__out RECT *prc,
            /* [out] */ __RPC__out BOOL *pfClipped) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetScreenExt( 
            /* [out] */ __RPC__out RECT *prc) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetWnd( 
            /* [out] */ __RPC__deref_out_opt HWND *phwnd) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ITfContextViewVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in ITfContextView * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in ITfContextView * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in ITfContextView * This);
        
        DECLSPEC_XFGVIRT(ITfContextView, GetRangeFromPoint)
        HRESULT ( STDMETHODCALLTYPE *GetRangeFromPoint )( 
            __RPC__in ITfContextView * This,
            /* [in] */ TfEditCookie ec,
            /* [in] */ __RPC__in const POINT *ppt,
            /* [in] */ DWORD dwFlags,
            /* [out] */ __RPC__deref_out_opt ITfRange **ppRange);
        
        DECLSPEC_XFGVIRT(ITfContextView, GetTextExt)
        HRESULT ( STDMETHODCALLTYPE *GetTextExt )( 
            __RPC__in ITfContextView * This,
            /* [in] */ TfEditCookie ec,
            /* [in] */ __RPC__in_opt ITfRange *pRange,
            /* [out] */ __RPC__out RECT *prc,
            /* [out] */ __RPC__out BOOL *pfClipped);
        
        DECLSPEC_XFGVIRT(ITfContextView, GetScreenExt)
        HRESULT ( STDMETHODCALLTYPE *GetScreenExt )( 
            __RPC__in ITfContextView * This,
            /* [out] */ __RPC__out RECT *prc);
        
        DECLSPEC_XFGVIRT(ITfContextView, GetWnd)
        HRESULT ( STDMETHODCALLTYPE *GetWnd )( 
            __RPC__in ITfContextView * This,
            /* [out] */ __RPC__deref_out_opt HWND *phwnd);
        
        END_INTERFACE
    } ITfContextViewVtbl;

    interface ITfContextView
    {
        CONST_VTBL struct ITfContextViewVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ITfContextView_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ITfContextView_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ITfContextView_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ITfContextView_GetRangeFromPoint(This,ec,ppt,dwFlags,ppRange)	\
    ( (This)->lpVtbl -> GetRangeFromPoint(This,ec,ppt,dwFlags,ppRange) ) 

#define ITfContextView_GetTextExt(This,ec,pRange,prc,pfClipped)	\
    ( (This)->lpVtbl -> GetTextExt(This,ec,pRange,prc,pfClipped) ) 

#define ITfContextView_GetScreenExt(This,prc)	\
    ( (This)->lpVtbl -> GetScreenExt(This,prc) ) 

#define ITfContextView_GetWnd(This,phwnd)	\
    ( (This)->lpVtbl -> GetWnd(This,phwnd) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ITfContextView_INTERFACE_DEFINED__ */


#ifndef __IEnumTfContextViews_INTERFACE_DEFINED__
#define __IEnumTfContextViews_INTERFACE_DEFINED__

/* interface IEnumTfContextViews */
/* [unique][uuid][object] */ 


EXTERN_C const IID IID_IEnumTfContextViews;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("F0C0F8DD-CF38-44E1-BB0F-68CF0D551C78")
    IEnumTfContextViews : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE Clone( 
            /* [out] */ __RPC__deref_out_opt IEnumTfContextViews **ppEnum) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Next( 
            /* [in] */ ULONG ulCount,
            /* [length_is][size_is][out] */ __RPC__out_ecount_part(ulCount, *pcFetched) ITfContextView **rgViews,
            /* [out] */ __RPC__out ULONG *pcFetched) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Reset( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Skip( 
            /* [in] */ ULONG ulCount) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IEnumTfContextViewsVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IEnumTfContextViews * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IEnumTfContextViews * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IEnumTfContextViews * This);
        
        DECLSPEC_XFGVIRT(IEnumTfContextViews, Clone)
        HRESULT ( STDMETHODCALLTYPE *Clone )( 
            __RPC__in IEnumTfContextViews * This,
            /* [out] */ __RPC__deref_out_opt IEnumTfContextViews **ppEnum);
        
        DECLSPEC_XFGVIRT(IEnumTfContextViews, Next)
        HRESULT ( STDMETHODCALLTYPE *Next )( 
            __RPC__in IEnumTfContextViews * This,
            /* [in] */ ULONG ulCount,
            /* [length_is][size_is][out] */ __RPC__out_ecount_part(ulCount, *pcFetched) ITfContextView **rgViews,
            /* [out] */ __RPC__out ULONG *pcFetched);
        
        DECLSPEC_XFGVIRT(IEnumTfContextViews, Reset)
        HRESULT ( STDMETHODCALLTYPE *Reset )( 
            __RPC__in IEnumTfContextViews * This);
        
        DECLSPEC_XFGVIRT(IEnumTfContextViews, Skip)
        HRESULT ( STDMETHODCALLTYPE *Skip )( 
            __RPC__in IEnumTfContextViews * This,
            /* [in] */ ULONG ulCount);
        
        END_INTERFACE
    } IEnumTfContextViewsVtbl;

    interface IEnumTfContextViews
    {
        CONST_VTBL struct IEnumTfContextViewsVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IEnumTfContextViews_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IEnumTfContextViews_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IEnumTfContextViews_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IEnumTfContextViews_Clone(This,ppEnum)	\
    ( (This)->lpVtbl -> Clone(This,ppEnum) ) 

#define IEnumTfContextViews_Next(This,ulCount,rgViews,pcFetched)	\
    ( (This)->lpVtbl -> Next(This,ulCount,rgViews,pcFetched) ) 

#define IEnumTfContextViews_Reset(This)	\
    ( (This)->lpVtbl -> Reset(This) ) 

#define IEnumTfContextViews_Skip(This,ulCount)	\
    ( (This)->lpVtbl -> Skip(This,ulCount) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IEnumTfContextViews_INTERFACE_DEFINED__ */


#ifndef __ITfContext_INTERFACE_DEFINED__
#define __ITfContext_INTERFACE_DEFINED__

/* interface ITfContext */
/* [unique][uuid][object] */ 

#define	TF_ES_ASYNCDONTCARE	( 0 )

#define	TF_ES_SYNC	( 0x1 )

#define	TF_ES_READ	( 0x2 )

#define	TF_ES_READWRITE	( 0x6 )

#define	TF_ES_ASYNC	( 0x8 )

typedef /* [public][public][public][public][public][uuid] */  DECLSPEC_UUID("1690be9b-d3e9-49f6-8d8b-51b905af4c43") 
enum __MIDL_ITfContext_0001
    {
        TF_AE_NONE	= 0,
        TF_AE_START	= 1,
        TF_AE_END	= 2
    } 	TfActiveSelEnd;

typedef /* [uuid] */  DECLSPEC_UUID("36ae42a4-6989-4bdc-b48a-6137b7bf2e42") struct TF_SELECTIONSTYLE
    {
    TfActiveSelEnd ase;
    BOOL fInterimChar;
    } 	TF_SELECTIONSTYLE;

typedef /* [uuid] */  DECLSPEC_UUID("75eb22f2-b0bf-46a8-8006-975a3b6efcf1") struct TF_SELECTION
    {
    ITfRange *range;
    TF_SELECTIONSTYLE style;
    } 	TF_SELECTION;

#define	TF_DEFAULT_SELECTION	( TS_DEFAULT_SELECTION )

#define	TF_SD_READONLY	( TS_SD_READONLY )

#define	TF_SD_LOADING	( TS_SD_LOADING )

#define	TF_SS_DISJOINTSEL	( TS_SS_DISJOINTSEL )

#define	TF_SS_REGIONS	( TS_SS_REGIONS )

#define	TF_SS_TRANSITORY	( TS_SS_TRANSITORY )

#define	TF_SS_TKBAUTOCORRECTENABLE	( TS_SS_TKBAUTOCORRECTENABLE )

#define	TF_SS_TKBPREDICTIONENABLE	( TS_SS_TKBPREDICTIONENABLE )

typedef /* [uuid] */  DECLSPEC_UUID("bc7d979a-846a-444d-afef-0a9bfa82b961") TS_STATUS TF_STATUS;


EXTERN_C const IID IID_ITfContext;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("aa80e7fd-2021-11d2-93e0-0060b067b86e")
    ITfContext : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE RequestEditSession( 
            /* [in] */ TfClientId tid,
            /* [in] */ __RPC__in_opt ITfEditSession *pes,
            /* [in] */ DWORD dwFlags,
            /* [out] */ __RPC__out HRESULT *phrSession) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE InWriteSession( 
            /* [in] */ TfClientId tid,
            /* [out] */ __RPC__out BOOL *pfWriteSession) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetSelection( 
            /* [in] */ TfEditCookie ec,
            /* [in] */ ULONG ulIndex,
            /* [in] */ ULONG ulCount,
            /* [length_is][size_is][out] */ __RPC__out_ecount_part(ulCount, *pcFetched) TF_SELECTION *pSelection,
            /* [out] */ __RPC__out ULONG *pcFetched) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetSelection( 
            /* [in] */ TfEditCookie ec,
            /* [in] */ ULONG ulCount,
            /* [size_is][in] */ __RPC__in_ecount_full(ulCount) const TF_SELECTION *pSelection) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetStart( 
            /* [in] */ TfEditCookie ec,
            /* [out] */ __RPC__deref_out_opt ITfRange **ppStart) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetEnd( 
            /* [in] */ TfEditCookie ec,
            /* [out] */ __RPC__deref_out_opt ITfRange **ppEnd) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetActiveView( 
            /* [out] */ __RPC__deref_out_opt ITfContextView **ppView) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE EnumViews( 
            /* [out] */ __RPC__deref_out_opt IEnumTfContextViews **ppEnum) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetStatus( 
            /* [out] */ __RPC__out TF_STATUS *pdcs) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetProperty( 
            /* [in] */ __RPC__in REFGUID guidProp,
            /* [out] */ __RPC__deref_out_opt ITfProperty **ppProp) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetAppProperty( 
            /* [in] */ __RPC__in REFGUID guidProp,
            /* [out] */ __RPC__deref_out_opt ITfReadOnlyProperty **ppProp) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE TrackProperties( 
            /* [size_is][in] */ __RPC__in_ecount_full(cProp) const GUID **prgProp,
            /* [in] */ ULONG cProp,
            /* [size_is][in] */ __RPC__in_ecount_full(cAppProp) const GUID **prgAppProp,
            /* [in] */ ULONG cAppProp,
            /* [out] */ __RPC__deref_out_opt ITfReadOnlyProperty **ppProperty) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE EnumProperties( 
            /* [out] */ __RPC__deref_out_opt IEnumTfProperties **ppEnum) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetDocumentMgr( 
            /* [out] */ __RPC__deref_out_opt ITfDocumentMgr **ppDm) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE CreateRangeBackup( 
            /* [in] */ TfEditCookie ec,
            /* [in] */ __RPC__in_opt ITfRange *pRange,
            /* [out] */ __RPC__deref_out_opt ITfRangeBackup **ppBackup) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ITfContextVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in ITfContext * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in ITfContext * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in ITfContext * This);
        
        DECLSPEC_XFGVIRT(ITfContext, RequestEditSession)
        HRESULT ( STDMETHODCALLTYPE *RequestEditSession )( 
            __RPC__in ITfContext * This,
            /* [in] */ TfClientId tid,
            /* [in] */ __RPC__in_opt ITfEditSession *pes,
            /* [in] */ DWORD dwFlags,
            /* [out] */ __RPC__out HRESULT *phrSession);
        
        DECLSPEC_XFGVIRT(ITfContext, InWriteSession)
        HRESULT ( STDMETHODCALLTYPE *InWriteSession )( 
            __RPC__in ITfContext * This,
            /* [in] */ TfClientId tid,
            /* [out] */ __RPC__out BOOL *pfWriteSession);
        
        DECLSPEC_XFGVIRT(ITfContext, GetSelection)
        HRESULT ( STDMETHODCALLTYPE *GetSelection )( 
            __RPC__in ITfContext * This,
            /* [in] */ TfEditCookie ec,
            /* [in] */ ULONG ulIndex,
            /* [in] */ ULONG ulCount,
            /* [length_is][size_is][out] */ __RPC__out_ecount_part(ulCount, *pcFetched) TF_SELECTION *pSelection,
            /* [out] */ __RPC__out ULONG *pcFetched);
        
        DECLSPEC_XFGVIRT(ITfContext, SetSelection)
        HRESULT ( STDMETHODCALLTYPE *SetSelection )( 
            __RPC__in ITfContext * This,
            /* [in] */ TfEditCookie ec,
            /* [in] */ ULONG ulCount,
            /* [size_is][in] */ __RPC__in_ecount_full(ulCount) const TF_SELECTION *pSelection);
        
        DECLSPEC_XFGVIRT(ITfContext, GetStart)
        HRESULT ( STDMETHODCALLTYPE *GetStart )( 
            __RPC__in ITfContext * This,
            /* [in] */ TfEditCookie ec,
            /* [out] */ __RPC__deref_out_opt ITfRange **ppStart);
        
        DECLSPEC_XFGVIRT(ITfContext, GetEnd)
        HRESULT ( STDMETHODCALLTYPE *GetEnd )( 
            __RPC__in ITfContext * This,
            /* [in] */ TfEditCookie ec,
            /* [out] */ __RPC__deref_out_opt ITfRange **ppEnd);
        
        DECLSPEC_XFGVIRT(ITfContext, GetActiveView)
        HRESULT ( STDMETHODCALLTYPE *GetActiveView )( 
            __RPC__in ITfContext * This,
            /* [out] */ __RPC__deref_out_opt ITfContextView **ppView);
        
        DECLSPEC_XFGVIRT(ITfContext, EnumViews)
        HRESULT ( STDMETHODCALLTYPE *EnumViews )( 
            __RPC__in ITfContext * This,
            /* [out] */ __RPC__deref_out_opt IEnumTfContextViews **ppEnum);
        
        DECLSPEC_XFGVIRT(ITfContext, GetStatus)
        HRESULT ( STDMETHODCALLTYPE *GetStatus )( 
            __RPC__in ITfContext * This,
            /* [out] */ __RPC__out TF_STATUS *pdcs);
        
        DECLSPEC_XFGVIRT(ITfContext, GetProperty)
        HRESULT ( STDMETHODCALLTYPE *GetProperty )( 
            __RPC__in ITfContext * This,
            /* [in] */ __RPC__in REFGUID guidProp,
            /* [out] */ __RPC__deref_out_opt ITfProperty **ppProp);
        
        DECLSPEC_XFGVIRT(ITfContext, GetAppProperty)
        HRESULT ( STDMETHODCALLTYPE *GetAppProperty )( 
            __RPC__in ITfContext * This,
            /* [in] */ __RPC__in REFGUID guidProp,
            /* [out] */ __RPC__deref_out_opt ITfReadOnlyProperty **ppProp);
        
        DECLSPEC_XFGVIRT(ITfContext, TrackProperties)
        HRESULT ( STDMETHODCALLTYPE *TrackProperties )( 
            __RPC__in ITfContext * This,
            /* [size_is][in] */ __RPC__in_ecount_full(cProp) const GUID **prgProp,
            /* [in] */ ULONG cProp,
            /* [size_is][in] */ __RPC__in_ecount_full(cAppProp) const GUID **prgAppProp,
            /* [in] */ ULONG cAppProp,
            /* [out] */ __RPC__deref_out_opt ITfReadOnlyProperty **ppProperty);
        
        DECLSPEC_XFGVIRT(ITfContext, EnumProperties)
        HRESULT ( STDMETHODCALLTYPE *EnumProperties )( 
            __RPC__in ITfContext * This,
            /* [out] */ __RPC__deref_out_opt IEnumTfProperties **ppEnum);
        
        DECLSPEC_XFGVIRT(ITfContext, GetDocumentMgr)
        HRESULT ( STDMETHODCALLTYPE *GetDocumentMgr )( 
            __RPC__in ITfContext * This,
            /* [out] */ __RPC__deref_out_opt ITfDocumentMgr **ppDm);
        
        DECLSPEC_XFGVIRT(ITfContext, CreateRangeBackup)
        HRESULT ( STDMETHODCALLTYPE *CreateRangeBackup )( 
            __RPC__in ITfContext * This,
            /* [in] */ TfEditCookie ec,
            /* [in] */ __RPC__in_opt ITfRange *pRange,
            /* [out] */ __RPC__deref_out_opt ITfRangeBackup **ppBackup);
        
        END_INTERFACE
    } ITfContextVtbl;

    interface ITfContext
    {
        CONST_VTBL struct ITfContextVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ITfContext_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ITfContext_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ITfContext_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ITfContext_RequestEditSession(This,tid,pes,dwFlags,phrSession)	\
    ( (This)->lpVtbl -> RequestEditSession(This,tid,pes,dwFlags,phrSession) ) 

#define ITfContext_InWriteSession(This,tid,pfWriteSession)	\
    ( (This)->lpVtbl -> InWriteSession(This,tid,pfWriteSession) ) 

#define ITfContext_GetSelection(This,ec,ulIndex,ulCount,pSelection,pcFetched)	\
    ( (This)->lpVtbl -> GetSelection(This,ec,ulIndex,ulCount,pSelection,pcFetched) ) 

#define ITfContext_SetSelection(This,ec,ulCount,pSelection)	\
    ( (This)->lpVtbl -> SetSelection(This,ec,ulCount,pSelection) ) 

#define ITfContext_GetStart(This,ec,ppStart)	\
    ( (This)->lpVtbl -> GetStart(This,ec,ppStart) ) 

#define ITfContext_GetEnd(This,ec,ppEnd)	\
    ( (This)->lpVtbl -> GetEnd(This,ec,ppEnd) ) 

#define ITfContext_GetActiveView(This,ppView)	\
    ( (This)->lpVtbl -> GetActiveView(This,ppView) ) 

#define ITfContext_EnumViews(This,ppEnum)	\
    ( (This)->lpVtbl -> EnumViews(This,ppEnum) ) 

#define ITfContext_GetStatus(This,pdcs)	\
    ( (This)->lpVtbl -> GetStatus(This,pdcs) ) 

#define ITfContext_GetProperty(This,guidProp,ppProp)	\
    ( (This)->lpVtbl -> GetProperty(This,guidProp,ppProp) ) 

#define ITfContext_GetAppProperty(This,guidProp,ppProp)	\
    ( (This)->lpVtbl -> GetAppProperty(This,guidProp,ppProp) ) 

#define ITfContext_TrackProperties(This,prgProp,cProp,prgAppProp,cAppProp,ppProperty)	\
    ( (This)->lpVtbl -> TrackProperties(This,prgProp,cProp,prgAppProp,cAppProp,ppProperty) ) 

#define ITfContext_EnumProperties(This,ppEnum)	\
    ( (This)->lpVtbl -> EnumProperties(This,ppEnum) ) 

#define ITfContext_GetDocumentMgr(This,ppDm)	\
    ( (This)->lpVtbl -> GetDocumentMgr(This,ppDm) ) 

#define ITfContext_CreateRangeBackup(This,ec,pRange,ppBackup)	\
    ( (This)->lpVtbl -> CreateRangeBackup(This,ec,pRange,ppBackup) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ITfContext_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_msctf_0000_0018 */
/* [local] */ 

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP) */
#pragma endregion
#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)


extern RPC_IF_HANDLE __MIDL_itf_msctf_0000_0018_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_msctf_0000_0018_v0_0_s_ifspec;

#ifndef __ITfQueryEmbedded_INTERFACE_DEFINED__
#define __ITfQueryEmbedded_INTERFACE_DEFINED__

/* interface ITfQueryEmbedded */
/* [unique][uuid][object] */ 


EXTERN_C const IID IID_ITfQueryEmbedded;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("0fab9bdb-d250-4169-84e5-6be118fdd7a8")
    ITfQueryEmbedded : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE QueryInsertEmbedded( 
            /* [in] */ __RPC__in const GUID *pguidService,
            /* [in] */ __RPC__in const FORMATETC *pFormatEtc,
            /* [out] */ __RPC__out BOOL *pfInsertable) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ITfQueryEmbeddedVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in ITfQueryEmbedded * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in ITfQueryEmbedded * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in ITfQueryEmbedded * This);
        
        DECLSPEC_XFGVIRT(ITfQueryEmbedded, QueryInsertEmbedded)
        HRESULT ( STDMETHODCALLTYPE *QueryInsertEmbedded )( 
            __RPC__in ITfQueryEmbedded * This,
            /* [in] */ __RPC__in const GUID *pguidService,
            /* [in] */ __RPC__in const FORMATETC *pFormatEtc,
            /* [out] */ __RPC__out BOOL *pfInsertable);
        
        END_INTERFACE
    } ITfQueryEmbeddedVtbl;

    interface ITfQueryEmbedded
    {
        CONST_VTBL struct ITfQueryEmbeddedVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ITfQueryEmbedded_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ITfQueryEmbedded_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ITfQueryEmbedded_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ITfQueryEmbedded_QueryInsertEmbedded(This,pguidService,pFormatEtc,pfInsertable)	\
    ( (This)->lpVtbl -> QueryInsertEmbedded(This,pguidService,pFormatEtc,pfInsertable) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ITfQueryEmbedded_INTERFACE_DEFINED__ */


#ifndef __ITfInsertAtSelection_INTERFACE_DEFINED__
#define __ITfInsertAtSelection_INTERFACE_DEFINED__

/* interface ITfInsertAtSelection */
/* [unique][uuid][object] */ 

#define	TF_IAS_NOQUERY	( 0x1 )

#define	TF_IAS_QUERYONLY	( 0x2 )

#define	TF_IAS_NO_DEFAULT_COMPOSITION	( 0x80000000 )


EXTERN_C const IID IID_ITfInsertAtSelection;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("55ce16ba-3014-41c1-9ceb-fade1446ac6c")
    ITfInsertAtSelection : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE InsertTextAtSelection( 
            /* [in] */ TfEditCookie ec,
            /* [in] */ DWORD dwFlags,
            /* [size_is][in] */ __RPC__in_ecount_full(cch) const WCHAR *pchText,
            /* [in] */ LONG cch,
            /* [out] */ __RPC__deref_out_opt ITfRange **ppRange) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE InsertEmbeddedAtSelection( 
            /* [in] */ TfEditCookie ec,
            /* [in] */ DWORD dwFlags,
            /* [in] */ __RPC__in_opt IDataObject *pDataObject,
            /* [out] */ __RPC__deref_out_opt ITfRange **ppRange) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ITfInsertAtSelectionVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in ITfInsertAtSelection * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in ITfInsertAtSelection * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in ITfInsertAtSelection * This);
        
        DECLSPEC_XFGVIRT(ITfInsertAtSelection, InsertTextAtSelection)
        HRESULT ( STDMETHODCALLTYPE *InsertTextAtSelection )( 
            __RPC__in ITfInsertAtSelection * This,
            /* [in] */ TfEditCookie ec,
            /* [in] */ DWORD dwFlags,
            /* [size_is][in] */ __RPC__in_ecount_full(cch) const WCHAR *pchText,
            /* [in] */ LONG cch,
            /* [out] */ __RPC__deref_out_opt ITfRange **ppRange);
        
        DECLSPEC_XFGVIRT(ITfInsertAtSelection, InsertEmbeddedAtSelection)
        HRESULT ( STDMETHODCALLTYPE *InsertEmbeddedAtSelection )( 
            __RPC__in ITfInsertAtSelection * This,
            /* [in] */ TfEditCookie ec,
            /* [in] */ DWORD dwFlags,
            /* [in] */ __RPC__in_opt IDataObject *pDataObject,
            /* [out] */ __RPC__deref_out_opt ITfRange **ppRange);
        
        END_INTERFACE
    } ITfInsertAtSelectionVtbl;

    interface ITfInsertAtSelection
    {
        CONST_VTBL struct ITfInsertAtSelectionVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ITfInsertAtSelection_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ITfInsertAtSelection_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ITfInsertAtSelection_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ITfInsertAtSelection_InsertTextAtSelection(This,ec,dwFlags,pchText,cch,ppRange)	\
    ( (This)->lpVtbl -> InsertTextAtSelection(This,ec,dwFlags,pchText,cch,ppRange) ) 

#define ITfInsertAtSelection_InsertEmbeddedAtSelection(This,ec,dwFlags,pDataObject,ppRange)	\
    ( (This)->lpVtbl -> InsertEmbeddedAtSelection(This,ec,dwFlags,pDataObject,ppRange) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ITfInsertAtSelection_INTERFACE_DEFINED__ */


#ifndef __ITfCleanupContextSink_INTERFACE_DEFINED__
#define __ITfCleanupContextSink_INTERFACE_DEFINED__

/* interface ITfCleanupContextSink */
/* [unique][uuid][object] */ 


EXTERN_C const IID IID_ITfCleanupContextSink;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("01689689-7acb-4e9b-ab7c-7ea46b12b522")
    ITfCleanupContextSink : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE OnCleanupContext( 
            /* [in] */ TfEditCookie ecWrite,
            /* [in] */ __RPC__in_opt ITfContext *pic) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ITfCleanupContextSinkVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in ITfCleanupContextSink * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in ITfCleanupContextSink * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in ITfCleanupContextSink * This);
        
        DECLSPEC_XFGVIRT(ITfCleanupContextSink, OnCleanupContext)
        HRESULT ( STDMETHODCALLTYPE *OnCleanupContext )( 
            __RPC__in ITfCleanupContextSink * This,
            /* [in] */ TfEditCookie ecWrite,
            /* [in] */ __RPC__in_opt ITfContext *pic);
        
        END_INTERFACE
    } ITfCleanupContextSinkVtbl;

    interface ITfCleanupContextSink
    {
        CONST_VTBL struct ITfCleanupContextSinkVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ITfCleanupContextSink_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ITfCleanupContextSink_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ITfCleanupContextSink_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ITfCleanupContextSink_OnCleanupContext(This,ecWrite,pic)	\
    ( (This)->lpVtbl -> OnCleanupContext(This,ecWrite,pic) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ITfCleanupContextSink_INTERFACE_DEFINED__ */


#ifndef __ITfCleanupContextDurationSink_INTERFACE_DEFINED__
#define __ITfCleanupContextDurationSink_INTERFACE_DEFINED__

/* interface ITfCleanupContextDurationSink */
/* [unique][uuid][object] */ 


EXTERN_C const IID IID_ITfCleanupContextDurationSink;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("45c35144-154e-4797-bed8-d33ae7bf8794")
    ITfCleanupContextDurationSink : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE OnStartCleanupContext( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE OnEndCleanupContext( void) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ITfCleanupContextDurationSinkVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in ITfCleanupContextDurationSink * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in ITfCleanupContextDurationSink * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in ITfCleanupContextDurationSink * This);
        
        DECLSPEC_XFGVIRT(ITfCleanupContextDurationSink, OnStartCleanupContext)
        HRESULT ( STDMETHODCALLTYPE *OnStartCleanupContext )( 
            __RPC__in ITfCleanupContextDurationSink * This);
        
        DECLSPEC_XFGVIRT(ITfCleanupContextDurationSink, OnEndCleanupContext)
        HRESULT ( STDMETHODCALLTYPE *OnEndCleanupContext )( 
            __RPC__in ITfCleanupContextDurationSink * This);
        
        END_INTERFACE
    } ITfCleanupContextDurationSinkVtbl;

    interface ITfCleanupContextDurationSink
    {
        CONST_VTBL struct ITfCleanupContextDurationSinkVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ITfCleanupContextDurationSink_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ITfCleanupContextDurationSink_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ITfCleanupContextDurationSink_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ITfCleanupContextDurationSink_OnStartCleanupContext(This)	\
    ( (This)->lpVtbl -> OnStartCleanupContext(This) ) 

#define ITfCleanupContextDurationSink_OnEndCleanupContext(This)	\
    ( (This)->lpVtbl -> OnEndCleanupContext(This) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ITfCleanupContextDurationSink_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_msctf_0000_0022 */
/* [local] */ 

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion
#pragma region Application Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP)


extern RPC_IF_HANDLE __MIDL_itf_msctf_0000_0022_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_msctf_0000_0022_v0_0_s_ifspec;

#ifndef __ITfReadOnlyProperty_INTERFACE_DEFINED__
#define __ITfReadOnlyProperty_INTERFACE_DEFINED__

/* interface ITfReadOnlyProperty */
/* [unique][uuid][object] */ 


EXTERN_C const IID IID_ITfReadOnlyProperty;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("17d49a3d-f8b8-4b2f-b254-52319dd64c53")
    ITfReadOnlyProperty : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetType( 
            /* [out] */ __RPC__out GUID *pguid) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE EnumRanges( 
            /* [in] */ TfEditCookie ec,
            /* [out] */ __RPC__deref_out_opt IEnumTfRanges **ppEnum,
            /* [in] */ __RPC__in_opt ITfRange *pTargetRange) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetValue( 
            /* [in] */ TfEditCookie ec,
            /* [in] */ __RPC__in_opt ITfRange *pRange,
            /* [out] */ __RPC__out VARIANT *pvarValue) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetContext( 
            /* [out] */ __RPC__deref_out_opt ITfContext **ppContext) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ITfReadOnlyPropertyVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in ITfReadOnlyProperty * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in ITfReadOnlyProperty * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in ITfReadOnlyProperty * This);
        
        DECLSPEC_XFGVIRT(ITfReadOnlyProperty, GetType)
        HRESULT ( STDMETHODCALLTYPE *GetType )( 
            __RPC__in ITfReadOnlyProperty * This,
            /* [out] */ __RPC__out GUID *pguid);
        
        DECLSPEC_XFGVIRT(ITfReadOnlyProperty, EnumRanges)
        HRESULT ( STDMETHODCALLTYPE *EnumRanges )( 
            __RPC__in ITfReadOnlyProperty * This,
            /* [in] */ TfEditCookie ec,
            /* [out] */ __RPC__deref_out_opt IEnumTfRanges **ppEnum,
            /* [in] */ __RPC__in_opt ITfRange *pTargetRange);
        
        DECLSPEC_XFGVIRT(ITfReadOnlyProperty, GetValue)
        HRESULT ( STDMETHODCALLTYPE *GetValue )( 
            __RPC__in ITfReadOnlyProperty * This,
            /* [in] */ TfEditCookie ec,
            /* [in] */ __RPC__in_opt ITfRange *pRange,
            /* [out] */ __RPC__out VARIANT *pvarValue);
        
        DECLSPEC_XFGVIRT(ITfReadOnlyProperty, GetContext)
        HRESULT ( STDMETHODCALLTYPE *GetContext )( 
            __RPC__in ITfReadOnlyProperty * This,
            /* [out] */ __RPC__deref_out_opt ITfContext **ppContext);
        
        END_INTERFACE
    } ITfReadOnlyPropertyVtbl;

    interface ITfReadOnlyProperty
    {
        CONST_VTBL struct ITfReadOnlyPropertyVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ITfReadOnlyProperty_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ITfReadOnlyProperty_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ITfReadOnlyProperty_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ITfReadOnlyProperty_GetType(This,pguid)	\
    ( (This)->lpVtbl -> GetType(This,pguid) ) 

#define ITfReadOnlyProperty_EnumRanges(This,ec,ppEnum,pTargetRange)	\
    ( (This)->lpVtbl -> EnumRanges(This,ec,ppEnum,pTargetRange) ) 

#define ITfReadOnlyProperty_GetValue(This,ec,pRange,pvarValue)	\
    ( (This)->lpVtbl -> GetValue(This,ec,pRange,pvarValue) ) 

#define ITfReadOnlyProperty_GetContext(This,ppContext)	\
    ( (This)->lpVtbl -> GetContext(This,ppContext) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ITfReadOnlyProperty_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_msctf_0000_0023 */
/* [local] */ 

typedef /* [uuid] */  DECLSPEC_UUID("d678c645-eb6a-45c9-b4ee-0f3e3a991348") struct TF_PROPERTYVAL
    {
    GUID guidId;
    VARIANT varValue;
    } 	TF_PROPERTYVAL;



extern RPC_IF_HANDLE __MIDL_itf_msctf_0000_0023_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_msctf_0000_0023_v0_0_s_ifspec;

#ifndef __IEnumTfPropertyValue_INTERFACE_DEFINED__
#define __IEnumTfPropertyValue_INTERFACE_DEFINED__

/* interface IEnumTfPropertyValue */
/* [unique][uuid][object] */ 


EXTERN_C const IID IID_IEnumTfPropertyValue;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("8ed8981b-7c10-4d7d-9fb3-ab72e9c75f72")
    IEnumTfPropertyValue : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE Clone( 
            /* [out] */ __RPC__deref_out_opt IEnumTfPropertyValue **ppEnum) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Next( 
            /* [in] */ ULONG ulCount,
            /* [length_is][size_is][out] */ __RPC__out_ecount_part(ulCount, *pcFetched) TF_PROPERTYVAL *rgValues,
            /* [out] */ __RPC__out ULONG *pcFetched) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Reset( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Skip( 
            /* [in] */ ULONG ulCount) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IEnumTfPropertyValueVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IEnumTfPropertyValue * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IEnumTfPropertyValue * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IEnumTfPropertyValue * This);
        
        DECLSPEC_XFGVIRT(IEnumTfPropertyValue, Clone)
        HRESULT ( STDMETHODCALLTYPE *Clone )( 
            __RPC__in IEnumTfPropertyValue * This,
            /* [out] */ __RPC__deref_out_opt IEnumTfPropertyValue **ppEnum);
        
        DECLSPEC_XFGVIRT(IEnumTfPropertyValue, Next)
        HRESULT ( STDMETHODCALLTYPE *Next )( 
            __RPC__in IEnumTfPropertyValue * This,
            /* [in] */ ULONG ulCount,
            /* [length_is][size_is][out] */ __RPC__out_ecount_part(ulCount, *pcFetched) TF_PROPERTYVAL *rgValues,
            /* [out] */ __RPC__out ULONG *pcFetched);
        
        DECLSPEC_XFGVIRT(IEnumTfPropertyValue, Reset)
        HRESULT ( STDMETHODCALLTYPE *Reset )( 
            __RPC__in IEnumTfPropertyValue * This);
        
        DECLSPEC_XFGVIRT(IEnumTfPropertyValue, Skip)
        HRESULT ( STDMETHODCALLTYPE *Skip )( 
            __RPC__in IEnumTfPropertyValue * This,
            /* [in] */ ULONG ulCount);
        
        END_INTERFACE
    } IEnumTfPropertyValueVtbl;

    interface IEnumTfPropertyValue
    {
        CONST_VTBL struct IEnumTfPropertyValueVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IEnumTfPropertyValue_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IEnumTfPropertyValue_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IEnumTfPropertyValue_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IEnumTfPropertyValue_Clone(This,ppEnum)	\
    ( (This)->lpVtbl -> Clone(This,ppEnum) ) 

#define IEnumTfPropertyValue_Next(This,ulCount,rgValues,pcFetched)	\
    ( (This)->lpVtbl -> Next(This,ulCount,rgValues,pcFetched) ) 

#define IEnumTfPropertyValue_Reset(This)	\
    ( (This)->lpVtbl -> Reset(This) ) 

#define IEnumTfPropertyValue_Skip(This,ulCount)	\
    ( (This)->lpVtbl -> Skip(This,ulCount) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IEnumTfPropertyValue_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_msctf_0000_0024 */
/* [local] */ 

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP) */
#pragma endregion
#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)


extern RPC_IF_HANDLE __MIDL_itf_msctf_0000_0024_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_msctf_0000_0024_v0_0_s_ifspec;

#ifndef __ITfMouseTracker_INTERFACE_DEFINED__
#define __ITfMouseTracker_INTERFACE_DEFINED__

/* interface ITfMouseTracker */
/* [unique][uuid][object] */ 


EXTERN_C const IID IID_ITfMouseTracker;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("09d146cd-a544-4132-925b-7afa8ef322d0")
    ITfMouseTracker : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE AdviseMouseSink( 
            /* [in] */ __RPC__in_opt ITfRange *range,
            /* [in] */ __RPC__in_opt ITfMouseSink *pSink,
            /* [out] */ __RPC__out DWORD *pdwCookie) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE UnadviseMouseSink( 
            /* [in] */ DWORD dwCookie) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ITfMouseTrackerVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in ITfMouseTracker * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in ITfMouseTracker * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in ITfMouseTracker * This);
        
        DECLSPEC_XFGVIRT(ITfMouseTracker, AdviseMouseSink)
        HRESULT ( STDMETHODCALLTYPE *AdviseMouseSink )( 
            __RPC__in ITfMouseTracker * This,
            /* [in] */ __RPC__in_opt ITfRange *range,
            /* [in] */ __RPC__in_opt ITfMouseSink *pSink,
            /* [out] */ __RPC__out DWORD *pdwCookie);
        
        DECLSPEC_XFGVIRT(ITfMouseTracker, UnadviseMouseSink)
        HRESULT ( STDMETHODCALLTYPE *UnadviseMouseSink )( 
            __RPC__in ITfMouseTracker * This,
            /* [in] */ DWORD dwCookie);
        
        END_INTERFACE
    } ITfMouseTrackerVtbl;

    interface ITfMouseTracker
    {
        CONST_VTBL struct ITfMouseTrackerVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ITfMouseTracker_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ITfMouseTracker_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ITfMouseTracker_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ITfMouseTracker_AdviseMouseSink(This,range,pSink,pdwCookie)	\
    ( (This)->lpVtbl -> AdviseMouseSink(This,range,pSink,pdwCookie) ) 

#define ITfMouseTracker_UnadviseMouseSink(This,dwCookie)	\
    ( (This)->lpVtbl -> UnadviseMouseSink(This,dwCookie) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ITfMouseTracker_INTERFACE_DEFINED__ */


#ifndef __ITfMouseTrackerACP_INTERFACE_DEFINED__
#define __ITfMouseTrackerACP_INTERFACE_DEFINED__

/* interface ITfMouseTrackerACP */
/* [unique][uuid][object] */ 


EXTERN_C const IID IID_ITfMouseTrackerACP;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("3bdd78e2-c16e-47fd-b883-ce6facc1a208")
    ITfMouseTrackerACP : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE AdviseMouseSink( 
            /* [in] */ __RPC__in_opt ITfRangeACP *range,
            /* [in] */ __RPC__in_opt ITfMouseSink *pSink,
            /* [out] */ __RPC__out DWORD *pdwCookie) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE UnadviseMouseSink( 
            /* [in] */ DWORD dwCookie) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ITfMouseTrackerACPVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in ITfMouseTrackerACP * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in ITfMouseTrackerACP * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in ITfMouseTrackerACP * This);
        
        DECLSPEC_XFGVIRT(ITfMouseTrackerACP, AdviseMouseSink)
        HRESULT ( STDMETHODCALLTYPE *AdviseMouseSink )( 
            __RPC__in ITfMouseTrackerACP * This,
            /* [in] */ __RPC__in_opt ITfRangeACP *range,
            /* [in] */ __RPC__in_opt ITfMouseSink *pSink,
            /* [out] */ __RPC__out DWORD *pdwCookie);
        
        DECLSPEC_XFGVIRT(ITfMouseTrackerACP, UnadviseMouseSink)
        HRESULT ( STDMETHODCALLTYPE *UnadviseMouseSink )( 
            __RPC__in ITfMouseTrackerACP * This,
            /* [in] */ DWORD dwCookie);
        
        END_INTERFACE
    } ITfMouseTrackerACPVtbl;

    interface ITfMouseTrackerACP
    {
        CONST_VTBL struct ITfMouseTrackerACPVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ITfMouseTrackerACP_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ITfMouseTrackerACP_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ITfMouseTrackerACP_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ITfMouseTrackerACP_AdviseMouseSink(This,range,pSink,pdwCookie)	\
    ( (This)->lpVtbl -> AdviseMouseSink(This,range,pSink,pdwCookie) ) 

#define ITfMouseTrackerACP_UnadviseMouseSink(This,dwCookie)	\
    ( (This)->lpVtbl -> UnadviseMouseSink(This,dwCookie) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ITfMouseTrackerACP_INTERFACE_DEFINED__ */


#ifndef __ITfMouseSink_INTERFACE_DEFINED__
#define __ITfMouseSink_INTERFACE_DEFINED__

/* interface ITfMouseSink */
/* [unique][uuid][local][object] */ 


EXTERN_C const IID IID_ITfMouseSink;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("a1adaaa2-3a24-449d-ac96-5183e7f5c217")
    ITfMouseSink : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE OnMouseEvent( 
            /* [in] */ ULONG uEdge,
            /* [in] */ ULONG uQuadrant,
            /* [in] */ DWORD dwBtnStatus,
            /* [out] */ BOOL *pfEaten) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ITfMouseSinkVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            ITfMouseSink * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            ITfMouseSink * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            ITfMouseSink * This);
        
        DECLSPEC_XFGVIRT(ITfMouseSink, OnMouseEvent)
        HRESULT ( STDMETHODCALLTYPE *OnMouseEvent )( 
            ITfMouseSink * This,
            /* [in] */ ULONG uEdge,
            /* [in] */ ULONG uQuadrant,
            /* [in] */ DWORD dwBtnStatus,
            /* [out] */ BOOL *pfEaten);
        
        END_INTERFACE
    } ITfMouseSinkVtbl;

    interface ITfMouseSink
    {
        CONST_VTBL struct ITfMouseSinkVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ITfMouseSink_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ITfMouseSink_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ITfMouseSink_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ITfMouseSink_OnMouseEvent(This,uEdge,uQuadrant,dwBtnStatus,pfEaten)	\
    ( (This)->lpVtbl -> OnMouseEvent(This,uEdge,uQuadrant,dwBtnStatus,pfEaten) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ITfMouseSink_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_msctf_0000_0027 */
/* [local] */ 

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion
#pragma region Application Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP)


extern RPC_IF_HANDLE __MIDL_itf_msctf_0000_0027_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_msctf_0000_0027_v0_0_s_ifspec;

#ifndef __ITfEditRecord_INTERFACE_DEFINED__
#define __ITfEditRecord_INTERFACE_DEFINED__

/* interface ITfEditRecord */
/* [unique][uuid][object] */ 

#define	TF_GTP_INCL_TEXT	( 0x1 )


EXTERN_C const IID IID_ITfEditRecord;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("42d4d099-7c1a-4a89-b836-6c6f22160df0")
    ITfEditRecord : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetSelectionStatus( 
            /* [out] */ __RPC__out BOOL *pfChanged) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetTextAndPropertyUpdates( 
            /* [in] */ DWORD dwFlags,
            /* [size_is][in] */ __RPC__in_ecount_full(cProperties) const GUID **prgProperties,
            /* [in] */ ULONG cProperties,
            /* [out] */ __RPC__deref_out_opt IEnumTfRanges **ppEnum) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ITfEditRecordVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in ITfEditRecord * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in ITfEditRecord * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in ITfEditRecord * This);
        
        DECLSPEC_XFGVIRT(ITfEditRecord, GetSelectionStatus)
        HRESULT ( STDMETHODCALLTYPE *GetSelectionStatus )( 
            __RPC__in ITfEditRecord * This,
            /* [out] */ __RPC__out BOOL *pfChanged);
        
        DECLSPEC_XFGVIRT(ITfEditRecord, GetTextAndPropertyUpdates)
        HRESULT ( STDMETHODCALLTYPE *GetTextAndPropertyUpdates )( 
            __RPC__in ITfEditRecord * This,
            /* [in] */ DWORD dwFlags,
            /* [size_is][in] */ __RPC__in_ecount_full(cProperties) const GUID **prgProperties,
            /* [in] */ ULONG cProperties,
            /* [out] */ __RPC__deref_out_opt IEnumTfRanges **ppEnum);
        
        END_INTERFACE
    } ITfEditRecordVtbl;

    interface ITfEditRecord
    {
        CONST_VTBL struct ITfEditRecordVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ITfEditRecord_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ITfEditRecord_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ITfEditRecord_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ITfEditRecord_GetSelectionStatus(This,pfChanged)	\
    ( (This)->lpVtbl -> GetSelectionStatus(This,pfChanged) ) 

#define ITfEditRecord_GetTextAndPropertyUpdates(This,dwFlags,prgProperties,cProperties,ppEnum)	\
    ( (This)->lpVtbl -> GetTextAndPropertyUpdates(This,dwFlags,prgProperties,cProperties,ppEnum) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ITfEditRecord_INTERFACE_DEFINED__ */


#ifndef __ITfTextEditSink_INTERFACE_DEFINED__
#define __ITfTextEditSink_INTERFACE_DEFINED__

/* interface ITfTextEditSink */
/* [unique][uuid][object] */ 


EXTERN_C const IID IID_ITfTextEditSink;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("8127d409-ccd3-4683-967a-b43d5b482bf7")
    ITfTextEditSink : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE OnEndEdit( 
            /* [in] */ __RPC__in_opt ITfContext *pic,
            /* [in] */ TfEditCookie ecReadOnly,
            /* [in] */ __RPC__in_opt ITfEditRecord *pEditRecord) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ITfTextEditSinkVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in ITfTextEditSink * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in ITfTextEditSink * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in ITfTextEditSink * This);
        
        DECLSPEC_XFGVIRT(ITfTextEditSink, OnEndEdit)
        HRESULT ( STDMETHODCALLTYPE *OnEndEdit )( 
            __RPC__in ITfTextEditSink * This,
            /* [in] */ __RPC__in_opt ITfContext *pic,
            /* [in] */ TfEditCookie ecReadOnly,
            /* [in] */ __RPC__in_opt ITfEditRecord *pEditRecord);
        
        END_INTERFACE
    } ITfTextEditSinkVtbl;

    interface ITfTextEditSink
    {
        CONST_VTBL struct ITfTextEditSinkVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ITfTextEditSink_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ITfTextEditSink_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ITfTextEditSink_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ITfTextEditSink_OnEndEdit(This,pic,ecReadOnly,pEditRecord)	\
    ( (This)->lpVtbl -> OnEndEdit(This,pic,ecReadOnly,pEditRecord) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ITfTextEditSink_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_msctf_0000_0029 */
/* [local] */ 

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP) */
#pragma endregion
#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)


extern RPC_IF_HANDLE __MIDL_itf_msctf_0000_0029_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_msctf_0000_0029_v0_0_s_ifspec;

#ifndef __ITfTextLayoutSink_INTERFACE_DEFINED__
#define __ITfTextLayoutSink_INTERFACE_DEFINED__

/* interface ITfTextLayoutSink */
/* [unique][uuid][object] */ 

typedef /* [public][public][uuid] */  DECLSPEC_UUID("603553cf-9edd-4cc1-9ecc-069e4a427734") 
enum __MIDL_ITfTextLayoutSink_0001
    {
        TF_LC_CREATE	= 0,
        TF_LC_CHANGE	= 1,
        TF_LC_DESTROY	= 2
    } 	TfLayoutCode;


EXTERN_C const IID IID_ITfTextLayoutSink;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("2af2d06a-dd5b-4927-a0b4-54f19c91fade")
    ITfTextLayoutSink : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE OnLayoutChange( 
            /* [in] */ __RPC__in_opt ITfContext *pic,
            /* [in] */ TfLayoutCode lcode,
            /* [in] */ __RPC__in_opt ITfContextView *pView) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ITfTextLayoutSinkVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in ITfTextLayoutSink * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in ITfTextLayoutSink * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in ITfTextLayoutSink * This);
        
        DECLSPEC_XFGVIRT(ITfTextLayoutSink, OnLayoutChange)
        HRESULT ( STDMETHODCALLTYPE *OnLayoutChange )( 
            __RPC__in ITfTextLayoutSink * This,
            /* [in] */ __RPC__in_opt ITfContext *pic,
            /* [in] */ TfLayoutCode lcode,
            /* [in] */ __RPC__in_opt ITfContextView *pView);
        
        END_INTERFACE
    } ITfTextLayoutSinkVtbl;

    interface ITfTextLayoutSink
    {
        CONST_VTBL struct ITfTextLayoutSinkVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ITfTextLayoutSink_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ITfTextLayoutSink_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ITfTextLayoutSink_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ITfTextLayoutSink_OnLayoutChange(This,pic,lcode,pView)	\
    ( (This)->lpVtbl -> OnLayoutChange(This,pic,lcode,pView) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ITfTextLayoutSink_INTERFACE_DEFINED__ */


#ifndef __ITfStatusSink_INTERFACE_DEFINED__
#define __ITfStatusSink_INTERFACE_DEFINED__

/* interface ITfStatusSink */
/* [unique][uuid][object] */ 


EXTERN_C const IID IID_ITfStatusSink;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("6b7d8d73-b267-4f69-b32e-1ca321ce4f45")
    ITfStatusSink : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE OnStatusChange( 
            /* [in] */ __RPC__in_opt ITfContext *pic,
            /* [in] */ DWORD dwFlags) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ITfStatusSinkVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in ITfStatusSink * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in ITfStatusSink * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in ITfStatusSink * This);
        
        DECLSPEC_XFGVIRT(ITfStatusSink, OnStatusChange)
        HRESULT ( STDMETHODCALLTYPE *OnStatusChange )( 
            __RPC__in ITfStatusSink * This,
            /* [in] */ __RPC__in_opt ITfContext *pic,
            /* [in] */ DWORD dwFlags);
        
        END_INTERFACE
    } ITfStatusSinkVtbl;

    interface ITfStatusSink
    {
        CONST_VTBL struct ITfStatusSinkVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ITfStatusSink_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ITfStatusSink_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ITfStatusSink_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ITfStatusSink_OnStatusChange(This,pic,dwFlags)	\
    ( (This)->lpVtbl -> OnStatusChange(This,pic,dwFlags) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ITfStatusSink_INTERFACE_DEFINED__ */


#ifndef __ITfEditTransactionSink_INTERFACE_DEFINED__
#define __ITfEditTransactionSink_INTERFACE_DEFINED__

/* interface ITfEditTransactionSink */
/* [unique][uuid][object] */ 


EXTERN_C const IID IID_ITfEditTransactionSink;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("708fbf70-b520-416b-b06c-2c41ab44f8ba")
    ITfEditTransactionSink : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE OnStartEditTransaction( 
            /* [in] */ __RPC__in_opt ITfContext *pic) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE OnEndEditTransaction( 
            /* [in] */ __RPC__in_opt ITfContext *pic) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ITfEditTransactionSinkVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in ITfEditTransactionSink * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in ITfEditTransactionSink * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in ITfEditTransactionSink * This);
        
        DECLSPEC_XFGVIRT(ITfEditTransactionSink, OnStartEditTransaction)
        HRESULT ( STDMETHODCALLTYPE *OnStartEditTransaction )( 
            __RPC__in ITfEditTransactionSink * This,
            /* [in] */ __RPC__in_opt ITfContext *pic);
        
        DECLSPEC_XFGVIRT(ITfEditTransactionSink, OnEndEditTransaction)
        HRESULT ( STDMETHODCALLTYPE *OnEndEditTransaction )( 
            __RPC__in ITfEditTransactionSink * This,
            /* [in] */ __RPC__in_opt ITfContext *pic);
        
        END_INTERFACE
    } ITfEditTransactionSinkVtbl;

    interface ITfEditTransactionSink
    {
        CONST_VTBL struct ITfEditTransactionSinkVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ITfEditTransactionSink_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ITfEditTransactionSink_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ITfEditTransactionSink_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ITfEditTransactionSink_OnStartEditTransaction(This,pic)	\
    ( (This)->lpVtbl -> OnStartEditTransaction(This,pic) ) 

#define ITfEditTransactionSink_OnEndEditTransaction(This,pic)	\
    ( (This)->lpVtbl -> OnEndEditTransaction(This,pic) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ITfEditTransactionSink_INTERFACE_DEFINED__ */


#ifndef __ITfContextOwner_INTERFACE_DEFINED__
#define __ITfContextOwner_INTERFACE_DEFINED__

/* interface ITfContextOwner */
/* [unique][uuid][object] */ 


EXTERN_C const IID IID_ITfContextOwner;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("aa80e80c-2021-11d2-93e0-0060b067b86e")
    ITfContextOwner : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetACPFromPoint( 
            /* [in] */ __RPC__in const POINT *ptScreen,
            /* [in] */ DWORD dwFlags,
            /* [out] */ __RPC__out LONG *pacp) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetTextExt( 
            /* [in] */ LONG acpStart,
            /* [in] */ LONG acpEnd,
            /* [out] */ __RPC__out RECT *prc,
            /* [out] */ __RPC__out BOOL *pfClipped) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetScreenExt( 
            /* [out] */ __RPC__out RECT *prc) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetStatus( 
            /* [out] */ __RPC__out TF_STATUS *pdcs) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetWnd( 
            /* [out] */ __RPC__deref_out_opt HWND *phwnd) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetAttribute( 
            /* [in] */ __RPC__in REFGUID rguidAttribute,
            /* [out] */ __RPC__out VARIANT *pvarValue) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ITfContextOwnerVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in ITfContextOwner * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in ITfContextOwner * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in ITfContextOwner * This);
        
        DECLSPEC_XFGVIRT(ITfContextOwner, GetACPFromPoint)
        HRESULT ( STDMETHODCALLTYPE *GetACPFromPoint )( 
            __RPC__in ITfContextOwner * This,
            /* [in] */ __RPC__in const POINT *ptScreen,
            /* [in] */ DWORD dwFlags,
            /* [out] */ __RPC__out LONG *pacp);
        
        DECLSPEC_XFGVIRT(ITfContextOwner, GetTextExt)
        HRESULT ( STDMETHODCALLTYPE *GetTextExt )( 
            __RPC__in ITfContextOwner * This,
            /* [in] */ LONG acpStart,
            /* [in] */ LONG acpEnd,
            /* [out] */ __RPC__out RECT *prc,
            /* [out] */ __RPC__out BOOL *pfClipped);
        
        DECLSPEC_XFGVIRT(ITfContextOwner, GetScreenExt)
        HRESULT ( STDMETHODCALLTYPE *GetScreenExt )( 
            __RPC__in ITfContextOwner * This,
            /* [out] */ __RPC__out RECT *prc);
        
        DECLSPEC_XFGVIRT(ITfContextOwner, GetStatus)
        HRESULT ( STDMETHODCALLTYPE *GetStatus )( 
            __RPC__in ITfContextOwner * This,
            /* [out] */ __RPC__out TF_STATUS *pdcs);
        
        DECLSPEC_XFGVIRT(ITfContextOwner, GetWnd)
        HRESULT ( STDMETHODCALLTYPE *GetWnd )( 
            __RPC__in ITfContextOwner * This,
            /* [out] */ __RPC__deref_out_opt HWND *phwnd);
        
        DECLSPEC_XFGVIRT(ITfContextOwner, GetAttribute)
        HRESULT ( STDMETHODCALLTYPE *GetAttribute )( 
            __RPC__in ITfContextOwner * This,
            /* [in] */ __RPC__in REFGUID rguidAttribute,
            /* [out] */ __RPC__out VARIANT *pvarValue);
        
        END_INTERFACE
    } ITfContextOwnerVtbl;

    interface ITfContextOwner
    {
        CONST_VTBL struct ITfContextOwnerVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ITfContextOwner_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ITfContextOwner_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ITfContextOwner_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ITfContextOwner_GetACPFromPoint(This,ptScreen,dwFlags,pacp)	\
    ( (This)->lpVtbl -> GetACPFromPoint(This,ptScreen,dwFlags,pacp) ) 

#define ITfContextOwner_GetTextExt(This,acpStart,acpEnd,prc,pfClipped)	\
    ( (This)->lpVtbl -> GetTextExt(This,acpStart,acpEnd,prc,pfClipped) ) 

#define ITfContextOwner_GetScreenExt(This,prc)	\
    ( (This)->lpVtbl -> GetScreenExt(This,prc) ) 

#define ITfContextOwner_GetStatus(This,pdcs)	\
    ( (This)->lpVtbl -> GetStatus(This,pdcs) ) 

#define ITfContextOwner_GetWnd(This,phwnd)	\
    ( (This)->lpVtbl -> GetWnd(This,phwnd) ) 

#define ITfContextOwner_GetAttribute(This,rguidAttribute,pvarValue)	\
    ( (This)->lpVtbl -> GetAttribute(This,rguidAttribute,pvarValue) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ITfContextOwner_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_msctf_0000_0033 */
/* [local] */ 

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion
#pragma region Application Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP)


extern RPC_IF_HANDLE __MIDL_itf_msctf_0000_0033_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_msctf_0000_0033_v0_0_s_ifspec;

#ifndef __ITfContextOwnerServices_INTERFACE_DEFINED__
#define __ITfContextOwnerServices_INTERFACE_DEFINED__

/* interface ITfContextOwnerServices */
/* [unique][uuid][object] */ 


EXTERN_C const IID IID_ITfContextOwnerServices;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("b23eb630-3e1c-11d3-a745-0050040ab407")
    ITfContextOwnerServices : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE OnLayoutChange( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE OnStatusChange( 
            /* [in] */ DWORD dwFlags) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE OnAttributeChange( 
            /* [in] */ __RPC__in REFGUID rguidAttribute) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Serialize( 
            /* [in] */ __RPC__in_opt ITfProperty *pProp,
            /* [in] */ __RPC__in_opt ITfRange *pRange,
            /* [out] */ __RPC__out TF_PERSISTENT_PROPERTY_HEADER_ACP *pHdr,
            /* [in] */ __RPC__in_opt IStream *pStream) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Unserialize( 
            /* [in] */ __RPC__in_opt ITfProperty *pProp,
            /* [in] */ __RPC__in const TF_PERSISTENT_PROPERTY_HEADER_ACP *pHdr,
            /* [in] */ __RPC__in_opt IStream *pStream,
            /* [in] */ __RPC__in_opt ITfPersistentPropertyLoaderACP *pLoader) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE ForceLoadProperty( 
            /* [in] */ __RPC__in_opt ITfProperty *pProp) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE CreateRange( 
            /* [in] */ LONG acpStart,
            /* [in] */ LONG acpEnd,
            /* [out] */ __RPC__deref_out_opt ITfRangeACP **ppRange) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ITfContextOwnerServicesVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in ITfContextOwnerServices * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in ITfContextOwnerServices * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in ITfContextOwnerServices * This);
        
        DECLSPEC_XFGVIRT(ITfContextOwnerServices, OnLayoutChange)
        HRESULT ( STDMETHODCALLTYPE *OnLayoutChange )( 
            __RPC__in ITfContextOwnerServices * This);
        
        DECLSPEC_XFGVIRT(ITfContextOwnerServices, OnStatusChange)
        HRESULT ( STDMETHODCALLTYPE *OnStatusChange )( 
            __RPC__in ITfContextOwnerServices * This,
            /* [in] */ DWORD dwFlags);
        
        DECLSPEC_XFGVIRT(ITfContextOwnerServices, OnAttributeChange)
        HRESULT ( STDMETHODCALLTYPE *OnAttributeChange )( 
            __RPC__in ITfContextOwnerServices * This,
            /* [in] */ __RPC__in REFGUID rguidAttribute);
        
        DECLSPEC_XFGVIRT(ITfContextOwnerServices, Serialize)
        HRESULT ( STDMETHODCALLTYPE *Serialize )( 
            __RPC__in ITfContextOwnerServices * This,
            /* [in] */ __RPC__in_opt ITfProperty *pProp,
            /* [in] */ __RPC__in_opt ITfRange *pRange,
            /* [out] */ __RPC__out TF_PERSISTENT_PROPERTY_HEADER_ACP *pHdr,
            /* [in] */ __RPC__in_opt IStream *pStream);
        
        DECLSPEC_XFGVIRT(ITfContextOwnerServices, Unserialize)
        HRESULT ( STDMETHODCALLTYPE *Unserialize )( 
            __RPC__in ITfContextOwnerServices * This,
            /* [in] */ __RPC__in_opt ITfProperty *pProp,
            /* [in] */ __RPC__in const TF_PERSISTENT_PROPERTY_HEADER_ACP *pHdr,
            /* [in] */ __RPC__in_opt IStream *pStream,
            /* [in] */ __RPC__in_opt ITfPersistentPropertyLoaderACP *pLoader);
        
        DECLSPEC_XFGVIRT(ITfContextOwnerServices, ForceLoadProperty)
        HRESULT ( STDMETHODCALLTYPE *ForceLoadProperty )( 
            __RPC__in ITfContextOwnerServices * This,
            /* [in] */ __RPC__in_opt ITfProperty *pProp);
        
        DECLSPEC_XFGVIRT(ITfContextOwnerServices, CreateRange)
        HRESULT ( STDMETHODCALLTYPE *CreateRange )( 
            __RPC__in ITfContextOwnerServices * This,
            /* [in] */ LONG acpStart,
            /* [in] */ LONG acpEnd,
            /* [out] */ __RPC__deref_out_opt ITfRangeACP **ppRange);
        
        END_INTERFACE
    } ITfContextOwnerServicesVtbl;

    interface ITfContextOwnerServices
    {
        CONST_VTBL struct ITfContextOwnerServicesVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ITfContextOwnerServices_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ITfContextOwnerServices_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ITfContextOwnerServices_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ITfContextOwnerServices_OnLayoutChange(This)	\
    ( (This)->lpVtbl -> OnLayoutChange(This) ) 

#define ITfContextOwnerServices_OnStatusChange(This,dwFlags)	\
    ( (This)->lpVtbl -> OnStatusChange(This,dwFlags) ) 

#define ITfContextOwnerServices_OnAttributeChange(This,rguidAttribute)	\
    ( (This)->lpVtbl -> OnAttributeChange(This,rguidAttribute) ) 

#define ITfContextOwnerServices_Serialize(This,pProp,pRange,pHdr,pStream)	\
    ( (This)->lpVtbl -> Serialize(This,pProp,pRange,pHdr,pStream) ) 

#define ITfContextOwnerServices_Unserialize(This,pProp,pHdr,pStream,pLoader)	\
    ( (This)->lpVtbl -> Unserialize(This,pProp,pHdr,pStream,pLoader) ) 

#define ITfContextOwnerServices_ForceLoadProperty(This,pProp)	\
    ( (This)->lpVtbl -> ForceLoadProperty(This,pProp) ) 

#define ITfContextOwnerServices_CreateRange(This,acpStart,acpEnd,ppRange)	\
    ( (This)->lpVtbl -> CreateRange(This,acpStart,acpEnd,ppRange) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ITfContextOwnerServices_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_msctf_0000_0034 */
/* [local] */ 

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP) */
#pragma endregion
#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)


extern RPC_IF_HANDLE __MIDL_itf_msctf_0000_0034_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_msctf_0000_0034_v0_0_s_ifspec;

#ifndef __ITfContextKeyEventSink_INTERFACE_DEFINED__
#define __ITfContextKeyEventSink_INTERFACE_DEFINED__

/* interface ITfContextKeyEventSink */
/* [unique][uuid][local][object] */ 


EXTERN_C const IID IID_ITfContextKeyEventSink;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("0552ba5d-c835-4934-bf50-846aaa67432f")
    ITfContextKeyEventSink : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE OnKeyDown( 
            /* [in] */ WPARAM wParam,
            /* [in] */ LPARAM lParam,
            /* [out] */ BOOL *pfEaten) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE OnKeyUp( 
            /* [in] */ WPARAM wParam,
            /* [in] */ LPARAM lParam,
            /* [out] */ BOOL *pfEaten) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE OnTestKeyDown( 
            /* [in] */ WPARAM wParam,
            /* [in] */ LPARAM lParam,
            /* [out] */ BOOL *pfEaten) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE OnTestKeyUp( 
            /* [in] */ WPARAM wParam,
            /* [in] */ LPARAM lParam,
            /* [out] */ BOOL *pfEaten) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ITfContextKeyEventSinkVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            ITfContextKeyEventSink * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            ITfContextKeyEventSink * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            ITfContextKeyEventSink * This);
        
        DECLSPEC_XFGVIRT(ITfContextKeyEventSink, OnKeyDown)
        HRESULT ( STDMETHODCALLTYPE *OnKeyDown )( 
            ITfContextKeyEventSink * This,
            /* [in] */ WPARAM wParam,
            /* [in] */ LPARAM lParam,
            /* [out] */ BOOL *pfEaten);
        
        DECLSPEC_XFGVIRT(ITfContextKeyEventSink, OnKeyUp)
        HRESULT ( STDMETHODCALLTYPE *OnKeyUp )( 
            ITfContextKeyEventSink * This,
            /* [in] */ WPARAM wParam,
            /* [in] */ LPARAM lParam,
            /* [out] */ BOOL *pfEaten);
        
        DECLSPEC_XFGVIRT(ITfContextKeyEventSink, OnTestKeyDown)
        HRESULT ( STDMETHODCALLTYPE *OnTestKeyDown )( 
            ITfContextKeyEventSink * This,
            /* [in] */ WPARAM wParam,
            /* [in] */ LPARAM lParam,
            /* [out] */ BOOL *pfEaten);
        
        DECLSPEC_XFGVIRT(ITfContextKeyEventSink, OnTestKeyUp)
        HRESULT ( STDMETHODCALLTYPE *OnTestKeyUp )( 
            ITfContextKeyEventSink * This,
            /* [in] */ WPARAM wParam,
            /* [in] */ LPARAM lParam,
            /* [out] */ BOOL *pfEaten);
        
        END_INTERFACE
    } ITfContextKeyEventSinkVtbl;

    interface ITfContextKeyEventSink
    {
        CONST_VTBL struct ITfContextKeyEventSinkVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ITfContextKeyEventSink_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ITfContextKeyEventSink_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ITfContextKeyEventSink_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ITfContextKeyEventSink_OnKeyDown(This,wParam,lParam,pfEaten)	\
    ( (This)->lpVtbl -> OnKeyDown(This,wParam,lParam,pfEaten) ) 

#define ITfContextKeyEventSink_OnKeyUp(This,wParam,lParam,pfEaten)	\
    ( (This)->lpVtbl -> OnKeyUp(This,wParam,lParam,pfEaten) ) 

#define ITfContextKeyEventSink_OnTestKeyDown(This,wParam,lParam,pfEaten)	\
    ( (This)->lpVtbl -> OnTestKeyDown(This,wParam,lParam,pfEaten) ) 

#define ITfContextKeyEventSink_OnTestKeyUp(This,wParam,lParam,pfEaten)	\
    ( (This)->lpVtbl -> OnTestKeyUp(This,wParam,lParam,pfEaten) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ITfContextKeyEventSink_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_msctf_0000_0035 */
/* [local] */ 

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion
#pragma region Application Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP)


extern RPC_IF_HANDLE __MIDL_itf_msctf_0000_0035_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_msctf_0000_0035_v0_0_s_ifspec;

#ifndef __ITfEditSession_INTERFACE_DEFINED__
#define __ITfEditSession_INTERFACE_DEFINED__

/* interface ITfEditSession */
/* [unique][uuid][object] */ 


EXTERN_C const IID IID_ITfEditSession;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("aa80e803-2021-11d2-93e0-0060b067b86e")
    ITfEditSession : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE DoEditSession( 
            /* [in] */ TfEditCookie ec) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ITfEditSessionVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in ITfEditSession * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in ITfEditSession * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in ITfEditSession * This);
        
        DECLSPEC_XFGVIRT(ITfEditSession, DoEditSession)
        HRESULT ( STDMETHODCALLTYPE *DoEditSession )( 
            __RPC__in ITfEditSession * This,
            /* [in] */ TfEditCookie ec);
        
        END_INTERFACE
    } ITfEditSessionVtbl;

    interface ITfEditSession
    {
        CONST_VTBL struct ITfEditSessionVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ITfEditSession_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ITfEditSession_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ITfEditSession_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ITfEditSession_DoEditSession(This,ec)	\
    ( (This)->lpVtbl -> DoEditSession(This,ec) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ITfEditSession_INTERFACE_DEFINED__ */


#ifndef __ITfRange_INTERFACE_DEFINED__
#define __ITfRange_INTERFACE_DEFINED__

/* interface ITfRange */
/* [unique][uuid][object] */ 

#define	TF_CHAR_EMBEDDED	( TS_CHAR_EMBEDDED )

typedef /* [public][public][public][public][public][uuid] */  DECLSPEC_UUID("cf610f06-2882-46f6-abe5-298568b664c4") 
enum __MIDL_ITfRange_0001
    {
        TF_GRAVITY_BACKWARD	= 0,
        TF_GRAVITY_FORWARD	= 1
    } 	TfGravity;

typedef /* [public][public][public][uuid] */  DECLSPEC_UUID("1e512533-bbdc-4530-9a8e-a1dc0af67468") 
enum __MIDL_ITfRange_0002
    {
        TF_SD_BACKWARD	= 0,
        TF_SD_FORWARD	= 1
    } 	TfShiftDir;

#define	TF_HF_OBJECT	( 1 )

#define	TF_TF_MOVESTART	( 1 )

#define	TF_TF_IGNOREEND	( 2 )

#define	TF_ST_CORRECTION	( 1 )

#define	TF_IE_CORRECTION	( 1 )

typedef /* [uuid] */  DECLSPEC_UUID("49930d51-7d93-448c-a48c-fea5dac192b1") struct TF_HALTCOND
    {
    ITfRange *pHaltRange;
    TfAnchor aHaltPos;
    DWORD dwFlags;
    } 	TF_HALTCOND;


EXTERN_C const IID IID_ITfRange;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("aa80e7ff-2021-11d2-93e0-0060b067b86e")
    ITfRange : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetText( 
            /* [in] */ TfEditCookie ec,
            /* [in] */ DWORD dwFlags,
            /* [length_is][size_is][out] */ __RPC__out_ecount_part(cchMax, *pcch) WCHAR *pchText,
            /* [in] */ ULONG cchMax,
            /* [out] */ __RPC__out ULONG *pcch) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetText( 
            /* [in] */ TfEditCookie ec,
            /* [in] */ DWORD dwFlags,
            /* [unique][size_is][in] */ __RPC__in_ecount_full_opt(cch) const WCHAR *pchText,
            /* [in] */ LONG cch) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetFormattedText( 
            /* [in] */ TfEditCookie ec,
            /* [out] */ __RPC__deref_out_opt IDataObject **ppDataObject) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetEmbedded( 
            /* [in] */ TfEditCookie ec,
            /* [in] */ __RPC__in REFGUID rguidService,
            /* [in] */ __RPC__in REFIID riid,
            /* [iid_is][out] */ __RPC__deref_out_opt IUnknown **ppunk) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE InsertEmbedded( 
            /* [in] */ TfEditCookie ec,
            /* [in] */ DWORD dwFlags,
            /* [in] */ __RPC__in_opt IDataObject *pDataObject) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE ShiftStart( 
            /* [in] */ TfEditCookie ec,
            /* [in] */ LONG cchReq,
            /* [out] */ __RPC__out LONG *pcch,
            /* [unique][in] */ __RPC__in_opt const TF_HALTCOND *pHalt) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE ShiftEnd( 
            /* [in] */ TfEditCookie ec,
            /* [in] */ LONG cchReq,
            /* [out] */ __RPC__out LONG *pcch,
            /* [unique][in] */ __RPC__in_opt const TF_HALTCOND *pHalt) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE ShiftStartToRange( 
            /* [in] */ TfEditCookie ec,
            /* [in] */ __RPC__in_opt ITfRange *pRange,
            /* [in] */ TfAnchor aPos) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE ShiftEndToRange( 
            /* [in] */ TfEditCookie ec,
            /* [in] */ __RPC__in_opt ITfRange *pRange,
            /* [in] */ TfAnchor aPos) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE ShiftStartRegion( 
            /* [in] */ TfEditCookie ec,
            /* [in] */ TfShiftDir dir,
            /* [out] */ __RPC__out BOOL *pfNoRegion) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE ShiftEndRegion( 
            /* [in] */ TfEditCookie ec,
            /* [in] */ TfShiftDir dir,
            /* [out] */ __RPC__out BOOL *pfNoRegion) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE IsEmpty( 
            /* [in] */ TfEditCookie ec,
            /* [out] */ __RPC__out BOOL *pfEmpty) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Collapse( 
            /* [in] */ TfEditCookie ec,
            /* [in] */ TfAnchor aPos) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE IsEqualStart( 
            /* [in] */ TfEditCookie ec,
            /* [in] */ __RPC__in_opt ITfRange *pWith,
            /* [in] */ TfAnchor aPos,
            /* [out] */ __RPC__out BOOL *pfEqual) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE IsEqualEnd( 
            /* [in] */ TfEditCookie ec,
            /* [in] */ __RPC__in_opt ITfRange *pWith,
            /* [in] */ TfAnchor aPos,
            /* [out] */ __RPC__out BOOL *pfEqual) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE CompareStart( 
            /* [in] */ TfEditCookie ec,
            /* [in] */ __RPC__in_opt ITfRange *pWith,
            /* [in] */ TfAnchor aPos,
            /* [out] */ __RPC__out LONG *plResult) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE CompareEnd( 
            /* [in] */ TfEditCookie ec,
            /* [in] */ __RPC__in_opt ITfRange *pWith,
            /* [in] */ TfAnchor aPos,
            /* [out] */ __RPC__out LONG *plResult) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE AdjustForInsert( 
            /* [in] */ TfEditCookie ec,
            /* [in] */ ULONG cchInsert,
            /* [out] */ __RPC__out BOOL *pfInsertOk) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetGravity( 
            /* [out] */ __RPC__out TfGravity *pgStart,
            /* [out] */ __RPC__out TfGravity *pgEnd) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetGravity( 
            /* [in] */ TfEditCookie ec,
            /* [in] */ TfGravity gStart,
            /* [in] */ TfGravity gEnd) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Clone( 
            /* [out] */ __RPC__deref_out_opt ITfRange **ppClone) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetContext( 
            /* [out] */ __RPC__deref_out_opt ITfContext **ppContext) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ITfRangeVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in ITfRange * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in ITfRange * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in ITfRange * This);
        
        DECLSPEC_XFGVIRT(ITfRange, GetText)
        HRESULT ( STDMETHODCALLTYPE *GetText )( 
            __RPC__in ITfRange * This,
            /* [in] */ TfEditCookie ec,
            /* [in] */ DWORD dwFlags,
            /* [length_is][size_is][out] */ __RPC__out_ecount_part(cchMax, *pcch) WCHAR *pchText,
            /* [in] */ ULONG cchMax,
            /* [out] */ __RPC__out ULONG *pcch);
        
        DECLSPEC_XFGVIRT(ITfRange, SetText)
        HRESULT ( STDMETHODCALLTYPE *SetText )( 
            __RPC__in ITfRange * This,
            /* [in] */ TfEditCookie ec,
            /* [in] */ DWORD dwFlags,
            /* [unique][size_is][in] */ __RPC__in_ecount_full_opt(cch) const WCHAR *pchText,
            /* [in] */ LONG cch);
        
        DECLSPEC_XFGVIRT(ITfRange, GetFormattedText)
        HRESULT ( STDMETHODCALLTYPE *GetFormattedText )( 
            __RPC__in ITfRange * This,
            /* [in] */ TfEditCookie ec,
            /* [out] */ __RPC__deref_out_opt IDataObject **ppDataObject);
        
        DECLSPEC_XFGVIRT(ITfRange, GetEmbedded)
        HRESULT ( STDMETHODCALLTYPE *GetEmbedded )( 
            __RPC__in ITfRange * This,
            /* [in] */ TfEditCookie ec,
            /* [in] */ __RPC__in REFGUID rguidService,
            /* [in] */ __RPC__in REFIID riid,
            /* [iid_is][out] */ __RPC__deref_out_opt IUnknown **ppunk);
        
        DECLSPEC_XFGVIRT(ITfRange, InsertEmbedded)
        HRESULT ( STDMETHODCALLTYPE *InsertEmbedded )( 
            __RPC__in ITfRange * This,
            /* [in] */ TfEditCookie ec,
            /* [in] */ DWORD dwFlags,
            /* [in] */ __RPC__in_opt IDataObject *pDataObject);
        
        DECLSPEC_XFGVIRT(ITfRange, ShiftStart)
        HRESULT ( STDMETHODCALLTYPE *ShiftStart )( 
            __RPC__in ITfRange * This,
            /* [in] */ TfEditCookie ec,
            /* [in] */ LONG cchReq,
            /* [out] */ __RPC__out LONG *pcch,
            /* [unique][in] */ __RPC__in_opt const TF_HALTCOND *pHalt);
        
        DECLSPEC_XFGVIRT(ITfRange, ShiftEnd)
        HRESULT ( STDMETHODCALLTYPE *ShiftEnd )( 
            __RPC__in ITfRange * This,
            /* [in] */ TfEditCookie ec,
            /* [in] */ LONG cchReq,
            /* [out] */ __RPC__out LONG *pcch,
            /* [unique][in] */ __RPC__in_opt const TF_HALTCOND *pHalt);
        
        DECLSPEC_XFGVIRT(ITfRange, ShiftStartToRange)
        HRESULT ( STDMETHODCALLTYPE *ShiftStartToRange )( 
            __RPC__in ITfRange * This,
            /* [in] */ TfEditCookie ec,
            /* [in] */ __RPC__in_opt ITfRange *pRange,
            /* [in] */ TfAnchor aPos);
        
        DECLSPEC_XFGVIRT(ITfRange, ShiftEndToRange)
        HRESULT ( STDMETHODCALLTYPE *ShiftEndToRange )( 
            __RPC__in ITfRange * This,
            /* [in] */ TfEditCookie ec,
            /* [in] */ __RPC__in_opt ITfRange *pRange,
            /* [in] */ TfAnchor aPos);
        
        DECLSPEC_XFGVIRT(ITfRange, ShiftStartRegion)
        HRESULT ( STDMETHODCALLTYPE *ShiftStartRegion )( 
            __RPC__in ITfRange * This,
            /* [in] */ TfEditCookie ec,
            /* [in] */ TfShiftDir dir,
            /* [out] */ __RPC__out BOOL *pfNoRegion);
        
        DECLSPEC_XFGVIRT(ITfRange, ShiftEndRegion)
        HRESULT ( STDMETHODCALLTYPE *ShiftEndRegion )( 
            __RPC__in ITfRange * This,
            /* [in] */ TfEditCookie ec,
            /* [in] */ TfShiftDir dir,
            /* [out] */ __RPC__out BOOL *pfNoRegion);
        
        DECLSPEC_XFGVIRT(ITfRange, IsEmpty)
        HRESULT ( STDMETHODCALLTYPE *IsEmpty )( 
            __RPC__in ITfRange * This,
            /* [in] */ TfEditCookie ec,
            /* [out] */ __RPC__out BOOL *pfEmpty);
        
        DECLSPEC_XFGVIRT(ITfRange, Collapse)
        HRESULT ( STDMETHODCALLTYPE *Collapse )( 
            __RPC__in ITfRange * This,
            /* [in] */ TfEditCookie ec,
            /* [in] */ TfAnchor aPos);
        
        DECLSPEC_XFGVIRT(ITfRange, IsEqualStart)
        HRESULT ( STDMETHODCALLTYPE *IsEqualStart )( 
            __RPC__in ITfRange * This,
            /* [in] */ TfEditCookie ec,
            /* [in] */ __RPC__in_opt ITfRange *pWith,
            /* [in] */ TfAnchor aPos,
            /* [out] */ __RPC__out BOOL *pfEqual);
        
        DECLSPEC_XFGVIRT(ITfRange, IsEqualEnd)
        HRESULT ( STDMETHODCALLTYPE *IsEqualEnd )( 
            __RPC__in ITfRange * This,
            /* [in] */ TfEditCookie ec,
            /* [in] */ __RPC__in_opt ITfRange *pWith,
            /* [in] */ TfAnchor aPos,
            /* [out] */ __RPC__out BOOL *pfEqual);
        
        DECLSPEC_XFGVIRT(ITfRange, CompareStart)
        HRESULT ( STDMETHODCALLTYPE *CompareStart )( 
            __RPC__in ITfRange * This,
            /* [in] */ TfEditCookie ec,
            /* [in] */ __RPC__in_opt ITfRange *pWith,
            /* [in] */ TfAnchor aPos,
            /* [out] */ __RPC__out LONG *plResult);
        
        DECLSPEC_XFGVIRT(ITfRange, CompareEnd)
        HRESULT ( STDMETHODCALLTYPE *CompareEnd )( 
            __RPC__in ITfRange * This,
            /* [in] */ TfEditCookie ec,
            /* [in] */ __RPC__in_opt ITfRange *pWith,
            /* [in] */ TfAnchor aPos,
            /* [out] */ __RPC__out LONG *plResult);
        
        DECLSPEC_XFGVIRT(ITfRange, AdjustForInsert)
        HRESULT ( STDMETHODCALLTYPE *AdjustForInsert )( 
            __RPC__in ITfRange * This,
            /* [in] */ TfEditCookie ec,
            /* [in] */ ULONG cchInsert,
            /* [out] */ __RPC__out BOOL *pfInsertOk);
        
        DECLSPEC_XFGVIRT(ITfRange, GetGravity)
        HRESULT ( STDMETHODCALLTYPE *GetGravity )( 
            __RPC__in ITfRange * This,
            /* [out] */ __RPC__out TfGravity *pgStart,
            /* [out] */ __RPC__out TfGravity *pgEnd);
        
        DECLSPEC_XFGVIRT(ITfRange, SetGravity)
        HRESULT ( STDMETHODCALLTYPE *SetGravity )( 
            __RPC__in ITfRange * This,
            /* [in] */ TfEditCookie ec,
            /* [in] */ TfGravity gStart,
            /* [in] */ TfGravity gEnd);
        
        DECLSPEC_XFGVIRT(ITfRange, Clone)
        HRESULT ( STDMETHODCALLTYPE *Clone )( 
            __RPC__in ITfRange * This,
            /* [out] */ __RPC__deref_out_opt ITfRange **ppClone);
        
        DECLSPEC_XFGVIRT(ITfRange, GetContext)
        HRESULT ( STDMETHODCALLTYPE *GetContext )( 
            __RPC__in ITfRange * This,
            /* [out] */ __RPC__deref_out_opt ITfContext **ppContext);
        
        END_INTERFACE
    } ITfRangeVtbl;

    interface ITfRange
    {
        CONST_VTBL struct ITfRangeVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ITfRange_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ITfRange_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ITfRange_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ITfRange_GetText(This,ec,dwFlags,pchText,cchMax,pcch)	\
    ( (This)->lpVtbl -> GetText(This,ec,dwFlags,pchText,cchMax,pcch) ) 

#define ITfRange_SetText(This,ec,dwFlags,pchText,cch)	\
    ( (This)->lpVtbl -> SetText(This,ec,dwFlags,pchText,cch) ) 

#define ITfRange_GetFormattedText(This,ec,ppDataObject)	\
    ( (This)->lpVtbl -> GetFormattedText(This,ec,ppDataObject) ) 

#define ITfRange_GetEmbedded(This,ec,rguidService,riid,ppunk)	\
    ( (This)->lpVtbl -> GetEmbedded(This,ec,rguidService,riid,ppunk) ) 

#define ITfRange_InsertEmbedded(This,ec,dwFlags,pDataObject)	\
    ( (This)->lpVtbl -> InsertEmbedded(This,ec,dwFlags,pDataObject) ) 

#define ITfRange_ShiftStart(This,ec,cchReq,pcch,pHalt)	\
    ( (This)->lpVtbl -> ShiftStart(This,ec,cchReq,pcch,pHalt) ) 

#define ITfRange_ShiftEnd(This,ec,cchReq,pcch,pHalt)	\
    ( (This)->lpVtbl -> ShiftEnd(This,ec,cchReq,pcch,pHalt) ) 

#define ITfRange_ShiftStartToRange(This,ec,pRange,aPos)	\
    ( (This)->lpVtbl -> ShiftStartToRange(This,ec,pRange,aPos) ) 

#define ITfRange_ShiftEndToRange(This,ec,pRange,aPos)	\
    ( (This)->lpVtbl -> ShiftEndToRange(This,ec,pRange,aPos) ) 

#define ITfRange_ShiftStartRegion(This,ec,dir,pfNoRegion)	\
    ( (This)->lpVtbl -> ShiftStartRegion(This,ec,dir,pfNoRegion) ) 

#define ITfRange_ShiftEndRegion(This,ec,dir,pfNoRegion)	\
    ( (This)->lpVtbl -> ShiftEndRegion(This,ec,dir,pfNoRegion) ) 

#define ITfRange_IsEmpty(This,ec,pfEmpty)	\
    ( (This)->lpVtbl -> IsEmpty(This,ec,pfEmpty) ) 

#define ITfRange_Collapse(This,ec,aPos)	\
    ( (This)->lpVtbl -> Collapse(This,ec,aPos) ) 

#define ITfRange_IsEqualStart(This,ec,pWith,aPos,pfEqual)	\
    ( (This)->lpVtbl -> IsEqualStart(This,ec,pWith,aPos,pfEqual) ) 

#define ITfRange_IsEqualEnd(This,ec,pWith,aPos,pfEqual)	\
    ( (This)->lpVtbl -> IsEqualEnd(This,ec,pWith,aPos,pfEqual) ) 

#define ITfRange_CompareStart(This,ec,pWith,aPos,plResult)	\
    ( (This)->lpVtbl -> CompareStart(This,ec,pWith,aPos,plResult) ) 

#define ITfRange_CompareEnd(This,ec,pWith,aPos,plResult)	\
    ( (This)->lpVtbl -> CompareEnd(This,ec,pWith,aPos,plResult) ) 

#define ITfRange_AdjustForInsert(This,ec,cchInsert,pfInsertOk)	\
    ( (This)->lpVtbl -> AdjustForInsert(This,ec,cchInsert,pfInsertOk) ) 

#define ITfRange_GetGravity(This,pgStart,pgEnd)	\
    ( (This)->lpVtbl -> GetGravity(This,pgStart,pgEnd) ) 

#define ITfRange_SetGravity(This,ec,gStart,gEnd)	\
    ( (This)->lpVtbl -> SetGravity(This,ec,gStart,gEnd) ) 

#define ITfRange_Clone(This,ppClone)	\
    ( (This)->lpVtbl -> Clone(This,ppClone) ) 

#define ITfRange_GetContext(This,ppContext)	\
    ( (This)->lpVtbl -> GetContext(This,ppContext) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ITfRange_INTERFACE_DEFINED__ */


#ifndef __ITfRangeACP_INTERFACE_DEFINED__
#define __ITfRangeACP_INTERFACE_DEFINED__

/* interface ITfRangeACP */
/* [unique][uuid][object] */ 


EXTERN_C const IID IID_ITfRangeACP;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("057a6296-029b-4154-b79a-0d461d4ea94c")
    ITfRangeACP : public ITfRange
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetExtent( 
            /* [out] */ __RPC__out LONG *pacpAnchor,
            /* [out] */ __RPC__out LONG *pcch) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetExtent( 
            /* [in] */ LONG acpAnchor,
            /* [in] */ LONG cch) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ITfRangeACPVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in ITfRangeACP * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in ITfRangeACP * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in ITfRangeACP * This);
        
        DECLSPEC_XFGVIRT(ITfRange, GetText)
        HRESULT ( STDMETHODCALLTYPE *GetText )( 
            __RPC__in ITfRangeACP * This,
            /* [in] */ TfEditCookie ec,
            /* [in] */ DWORD dwFlags,
            /* [length_is][size_is][out] */ __RPC__out_ecount_part(cchMax, *pcch) WCHAR *pchText,
            /* [in] */ ULONG cchMax,
            /* [out] */ __RPC__out ULONG *pcch);
        
        DECLSPEC_XFGVIRT(ITfRange, SetText)
        HRESULT ( STDMETHODCALLTYPE *SetText )( 
            __RPC__in ITfRangeACP * This,
            /* [in] */ TfEditCookie ec,
            /* [in] */ DWORD dwFlags,
            /* [unique][size_is][in] */ __RPC__in_ecount_full_opt(cch) const WCHAR *pchText,
            /* [in] */ LONG cch);
        
        DECLSPEC_XFGVIRT(ITfRange, GetFormattedText)
        HRESULT ( STDMETHODCALLTYPE *GetFormattedText )( 
            __RPC__in ITfRangeACP * This,
            /* [in] */ TfEditCookie ec,
            /* [out] */ __RPC__deref_out_opt IDataObject **ppDataObject);
        
        DECLSPEC_XFGVIRT(ITfRange, GetEmbedded)
        HRESULT ( STDMETHODCALLTYPE *GetEmbedded )( 
            __RPC__in ITfRangeACP * This,
            /* [in] */ TfEditCookie ec,
            /* [in] */ __RPC__in REFGUID rguidService,
            /* [in] */ __RPC__in REFIID riid,
            /* [iid_is][out] */ __RPC__deref_out_opt IUnknown **ppunk);
        
        DECLSPEC_XFGVIRT(ITfRange, InsertEmbedded)
        HRESULT ( STDMETHODCALLTYPE *InsertEmbedded )( 
            __RPC__in ITfRangeACP * This,
            /* [in] */ TfEditCookie ec,
            /* [in] */ DWORD dwFlags,
            /* [in] */ __RPC__in_opt IDataObject *pDataObject);
        
        DECLSPEC_XFGVIRT(ITfRange, ShiftStart)
        HRESULT ( STDMETHODCALLTYPE *ShiftStart )( 
            __RPC__in ITfRangeACP * This,
            /* [in] */ TfEditCookie ec,
            /* [in] */ LONG cchReq,
            /* [out] */ __RPC__out LONG *pcch,
            /* [unique][in] */ __RPC__in_opt const TF_HALTCOND *pHalt);
        
        DECLSPEC_XFGVIRT(ITfRange, ShiftEnd)
        HRESULT ( STDMETHODCALLTYPE *ShiftEnd )( 
            __RPC__in ITfRangeACP * This,
            /* [in] */ TfEditCookie ec,
            /* [in] */ LONG cchReq,
            /* [out] */ __RPC__out LONG *pcch,
            /* [unique][in] */ __RPC__in_opt const TF_HALTCOND *pHalt);
        
        DECLSPEC_XFGVIRT(ITfRange, ShiftStartToRange)
        HRESULT ( STDMETHODCALLTYPE *ShiftStartToRange )( 
            __RPC__in ITfRangeACP * This,
            /* [in] */ TfEditCookie ec,
            /* [in] */ __RPC__in_opt ITfRange *pRange,
            /* [in] */ TfAnchor aPos);
        
        DECLSPEC_XFGVIRT(ITfRange, ShiftEndToRange)
        HRESULT ( STDMETHODCALLTYPE *ShiftEndToRange )( 
            __RPC__in ITfRangeACP * This,
            /* [in] */ TfEditCookie ec,
            /* [in] */ __RPC__in_opt ITfRange *pRange,
            /* [in] */ TfAnchor aPos);
        
        DECLSPEC_XFGVIRT(ITfRange, ShiftStartRegion)
        HRESULT ( STDMETHODCALLTYPE *ShiftStartRegion )( 
            __RPC__in ITfRangeACP * This,
            /* [in] */ TfEditCookie ec,
            /* [in] */ TfShiftDir dir,
            /* [out] */ __RPC__out BOOL *pfNoRegion);
        
        DECLSPEC_XFGVIRT(ITfRange, ShiftEndRegion)
        HRESULT ( STDMETHODCALLTYPE *ShiftEndRegion )( 
            __RPC__in ITfRangeACP * This,
            /* [in] */ TfEditCookie ec,
            /* [in] */ TfShiftDir dir,
            /* [out] */ __RPC__out BOOL *pfNoRegion);
        
        DECLSPEC_XFGVIRT(ITfRange, IsEmpty)
        HRESULT ( STDMETHODCALLTYPE *IsEmpty )( 
            __RPC__in ITfRangeACP * This,
            /* [in] */ TfEditCookie ec,
            /* [out] */ __RPC__out BOOL *pfEmpty);
        
        DECLSPEC_XFGVIRT(ITfRange, Collapse)
        HRESULT ( STDMETHODCALLTYPE *Collapse )( 
            __RPC__in ITfRangeACP * This,
            /* [in] */ TfEditCookie ec,
            /* [in] */ TfAnchor aPos);
        
        DECLSPEC_XFGVIRT(ITfRange, IsEqualStart)
        HRESULT ( STDMETHODCALLTYPE *IsEqualStart )( 
            __RPC__in ITfRangeACP * This,
            /* [in] */ TfEditCookie ec,
            /* [in] */ __RPC__in_opt ITfRange *pWith,
            /* [in] */ TfAnchor aPos,
            /* [out] */ __RPC__out BOOL *pfEqual);
        
        DECLSPEC_XFGVIRT(ITfRange, IsEqualEnd)
        HRESULT ( STDMETHODCALLTYPE *IsEqualEnd )( 
            __RPC__in ITfRangeACP * This,
            /* [in] */ TfEditCookie ec,
            /* [in] */ __RPC__in_opt ITfRange *pWith,
            /* [in] */ TfAnchor aPos,
            /* [out] */ __RPC__out BOOL *pfEqual);
        
        DECLSPEC_XFGVIRT(ITfRange, CompareStart)
        HRESULT ( STDMETHODCALLTYPE *CompareStart )( 
            __RPC__in ITfRangeACP * This,
            /* [in] */ TfEditCookie ec,
            /* [in] */ __RPC__in_opt ITfRange *pWith,
            /* [in] */ TfAnchor aPos,
            /* [out] */ __RPC__out LONG *plResult);
        
        DECLSPEC_XFGVIRT(ITfRange, CompareEnd)
        HRESULT ( STDMETHODCALLTYPE *CompareEnd )( 
            __RPC__in ITfRangeACP * This,
            /* [in] */ TfEditCookie ec,
            /* [in] */ __RPC__in_opt ITfRange *pWith,
            /* [in] */ TfAnchor aPos,
            /* [out] */ __RPC__out LONG *plResult);
        
        DECLSPEC_XFGVIRT(ITfRange, AdjustForInsert)
        HRESULT ( STDMETHODCALLTYPE *AdjustForInsert )( 
            __RPC__in ITfRangeACP * This,
            /* [in] */ TfEditCookie ec,
            /* [in] */ ULONG cchInsert,
            /* [out] */ __RPC__out BOOL *pfInsertOk);
        
        DECLSPEC_XFGVIRT(ITfRange, GetGravity)
        HRESULT ( STDMETHODCALLTYPE *GetGravity )( 
            __RPC__in ITfRangeACP * This,
            /* [out] */ __RPC__out TfGravity *pgStart,
            /* [out] */ __RPC__out TfGravity *pgEnd);
        
        DECLSPEC_XFGVIRT(ITfRange, SetGravity)
        HRESULT ( STDMETHODCALLTYPE *SetGravity )( 
            __RPC__in ITfRangeACP * This,
            /* [in] */ TfEditCookie ec,
            /* [in] */ TfGravity gStart,
            /* [in] */ TfGravity gEnd);
        
        DECLSPEC_XFGVIRT(ITfRange, Clone)
        HRESULT ( STDMETHODCALLTYPE *Clone )( 
            __RPC__in ITfRangeACP * This,
            /* [out] */ __RPC__deref_out_opt ITfRange **ppClone);
        
        DECLSPEC_XFGVIRT(ITfRange, GetContext)
        HRESULT ( STDMETHODCALLTYPE *GetContext )( 
            __RPC__in ITfRangeACP * This,
            /* [out] */ __RPC__deref_out_opt ITfContext **ppContext);
        
        DECLSPEC_XFGVIRT(ITfRangeACP, GetExtent)
        HRESULT ( STDMETHODCALLTYPE *GetExtent )( 
            __RPC__in ITfRangeACP * This,
            /* [out] */ __RPC__out LONG *pacpAnchor,
            /* [out] */ __RPC__out LONG *pcch);
        
        DECLSPEC_XFGVIRT(ITfRangeACP, SetExtent)
        HRESULT ( STDMETHODCALLTYPE *SetExtent )( 
            __RPC__in ITfRangeACP * This,
            /* [in] */ LONG acpAnchor,
            /* [in] */ LONG cch);
        
        END_INTERFACE
    } ITfRangeACPVtbl;

    interface ITfRangeACP
    {
        CONST_VTBL struct ITfRangeACPVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ITfRangeACP_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ITfRangeACP_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ITfRangeACP_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ITfRangeACP_GetText(This,ec,dwFlags,pchText,cchMax,pcch)	\
    ( (This)->lpVtbl -> GetText(This,ec,dwFlags,pchText,cchMax,pcch) ) 

#define ITfRangeACP_SetText(This,ec,dwFlags,pchText,cch)	\
    ( (This)->lpVtbl -> SetText(This,ec,dwFlags,pchText,cch) ) 

#define ITfRangeACP_GetFormattedText(This,ec,ppDataObject)	\
    ( (This)->lpVtbl -> GetFormattedText(This,ec,ppDataObject) ) 

#define ITfRangeACP_GetEmbedded(This,ec,rguidService,riid,ppunk)	\
    ( (This)->lpVtbl -> GetEmbedded(This,ec,rguidService,riid,ppunk) ) 

#define ITfRangeACP_InsertEmbedded(This,ec,dwFlags,pDataObject)	\
    ( (This)->lpVtbl -> InsertEmbedded(This,ec,dwFlags,pDataObject) ) 

#define ITfRangeACP_ShiftStart(This,ec,cchReq,pcch,pHalt)	\
    ( (This)->lpVtbl -> ShiftStart(This,ec,cchReq,pcch,pHalt) ) 

#define ITfRangeACP_ShiftEnd(This,ec,cchReq,pcch,pHalt)	\
    ( (This)->lpVtbl -> ShiftEnd(This,ec,cchReq,pcch,pHalt) ) 

#define ITfRangeACP_ShiftStartToRange(This,ec,pRange,aPos)	\
    ( (This)->lpVtbl -> ShiftStartToRange(This,ec,pRange,aPos) ) 

#define ITfRangeACP_ShiftEndToRange(This,ec,pRange,aPos)	\
    ( (This)->lpVtbl -> ShiftEndToRange(This,ec,pRange,aPos) ) 

#define ITfRangeACP_ShiftStartRegion(This,ec,dir,pfNoRegion)	\
    ( (This)->lpVtbl -> ShiftStartRegion(This,ec,dir,pfNoRegion) ) 

#define ITfRangeACP_ShiftEndRegion(This,ec,dir,pfNoRegion)	\
    ( (This)->lpVtbl -> ShiftEndRegion(This,ec,dir,pfNoRegion) ) 

#define ITfRangeACP_IsEmpty(This,ec,pfEmpty)	\
    ( (This)->lpVtbl -> IsEmpty(This,ec,pfEmpty) ) 

#define ITfRangeACP_Collapse(This,ec,aPos)	\
    ( (This)->lpVtbl -> Collapse(This,ec,aPos) ) 

#define ITfRangeACP_IsEqualStart(This,ec,pWith,aPos,pfEqual)	\
    ( (This)->lpVtbl -> IsEqualStart(This,ec,pWith,aPos,pfEqual) ) 

#define ITfRangeACP_IsEqualEnd(This,ec,pWith,aPos,pfEqual)	\
    ( (This)->lpVtbl -> IsEqualEnd(This,ec,pWith,aPos,pfEqual) ) 

#define ITfRangeACP_CompareStart(This,ec,pWith,aPos,plResult)	\
    ( (This)->lpVtbl -> CompareStart(This,ec,pWith,aPos,plResult) ) 

#define ITfRangeACP_CompareEnd(This,ec,pWith,aPos,plResult)	\
    ( (This)->lpVtbl -> CompareEnd(This,ec,pWith,aPos,plResult) ) 

#define ITfRangeACP_AdjustForInsert(This,ec,cchInsert,pfInsertOk)	\
    ( (This)->lpVtbl -> AdjustForInsert(This,ec,cchInsert,pfInsertOk) ) 

#define ITfRangeACP_GetGravity(This,pgStart,pgEnd)	\
    ( (This)->lpVtbl -> GetGravity(This,pgStart,pgEnd) ) 

#define ITfRangeACP_SetGravity(This,ec,gStart,gEnd)	\
    ( (This)->lpVtbl -> SetGravity(This,ec,gStart,gEnd) ) 

#define ITfRangeACP_Clone(This,ppClone)	\
    ( (This)->lpVtbl -> Clone(This,ppClone) ) 

#define ITfRangeACP_GetContext(This,ppContext)	\
    ( (This)->lpVtbl -> GetContext(This,ppContext) ) 


#define ITfRangeACP_GetExtent(This,pacpAnchor,pcch)	\
    ( (This)->lpVtbl -> GetExtent(This,pacpAnchor,pcch) ) 

#define ITfRangeACP_SetExtent(This,acpAnchor,cch)	\
    ( (This)->lpVtbl -> SetExtent(This,acpAnchor,cch) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ITfRangeACP_INTERFACE_DEFINED__ */


#ifndef __ITextStoreACPServices_INTERFACE_DEFINED__
#define __ITextStoreACPServices_INTERFACE_DEFINED__

/* interface ITextStoreACPServices */
/* [unique][uuid][object] */ 


EXTERN_C const IID IID_ITextStoreACPServices;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("aa80e901-2021-11d2-93e0-0060b067b86e")
    ITextStoreACPServices : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE Serialize( 
            /* [in] */ __RPC__in_opt ITfProperty *pProp,
            /* [in] */ __RPC__in_opt ITfRange *pRange,
            /* [out] */ __RPC__out TF_PERSISTENT_PROPERTY_HEADER_ACP *pHdr,
            /* [in] */ __RPC__in_opt IStream *pStream) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Unserialize( 
            /* [in] */ __RPC__in_opt ITfProperty *pProp,
            /* [in] */ __RPC__in const TF_PERSISTENT_PROPERTY_HEADER_ACP *pHdr,
            /* [in] */ __RPC__in_opt IStream *pStream,
            /* [in] */ __RPC__in_opt ITfPersistentPropertyLoaderACP *pLoader) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE ForceLoadProperty( 
            /* [in] */ __RPC__in_opt ITfProperty *pProp) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE CreateRange( 
            /* [in] */ LONG acpStart,
            /* [in] */ LONG acpEnd,
            /* [out] */ __RPC__deref_out_opt ITfRangeACP **ppRange) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ITextStoreACPServicesVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in ITextStoreACPServices * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in ITextStoreACPServices * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in ITextStoreACPServices * This);
        
        DECLSPEC_XFGVIRT(ITextStoreACPServices, Serialize)
        HRESULT ( STDMETHODCALLTYPE *Serialize )( 
            __RPC__in ITextStoreACPServices * This,
            /* [in] */ __RPC__in_opt ITfProperty *pProp,
            /* [in] */ __RPC__in_opt ITfRange *pRange,
            /* [out] */ __RPC__out TF_PERSISTENT_PROPERTY_HEADER_ACP *pHdr,
            /* [in] */ __RPC__in_opt IStream *pStream);
        
        DECLSPEC_XFGVIRT(ITextStoreACPServices, Unserialize)
        HRESULT ( STDMETHODCALLTYPE *Unserialize )( 
            __RPC__in ITextStoreACPServices * This,
            /* [in] */ __RPC__in_opt ITfProperty *pProp,
            /* [in] */ __RPC__in const TF_PERSISTENT_PROPERTY_HEADER_ACP *pHdr,
            /* [in] */ __RPC__in_opt IStream *pStream,
            /* [in] */ __RPC__in_opt ITfPersistentPropertyLoaderACP *pLoader);
        
        DECLSPEC_XFGVIRT(ITextStoreACPServices, ForceLoadProperty)
        HRESULT ( STDMETHODCALLTYPE *ForceLoadProperty )( 
            __RPC__in ITextStoreACPServices * This,
            /* [in] */ __RPC__in_opt ITfProperty *pProp);
        
        DECLSPEC_XFGVIRT(ITextStoreACPServices, CreateRange)
        HRESULT ( STDMETHODCALLTYPE *CreateRange )( 
            __RPC__in ITextStoreACPServices * This,
            /* [in] */ LONG acpStart,
            /* [in] */ LONG acpEnd,
            /* [out] */ __RPC__deref_out_opt ITfRangeACP **ppRange);
        
        END_INTERFACE
    } ITextStoreACPServicesVtbl;

    interface ITextStoreACPServices
    {
        CONST_VTBL struct ITextStoreACPServicesVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ITextStoreACPServices_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ITextStoreACPServices_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ITextStoreACPServices_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ITextStoreACPServices_Serialize(This,pProp,pRange,pHdr,pStream)	\
    ( (This)->lpVtbl -> Serialize(This,pProp,pRange,pHdr,pStream) ) 

#define ITextStoreACPServices_Unserialize(This,pProp,pHdr,pStream,pLoader)	\
    ( (This)->lpVtbl -> Unserialize(This,pProp,pHdr,pStream,pLoader) ) 

#define ITextStoreACPServices_ForceLoadProperty(This,pProp)	\
    ( (This)->lpVtbl -> ForceLoadProperty(This,pProp) ) 

#define ITextStoreACPServices_CreateRange(This,acpStart,acpEnd,ppRange)	\
    ( (This)->lpVtbl -> CreateRange(This,acpStart,acpEnd,ppRange) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ITextStoreACPServices_INTERFACE_DEFINED__ */


#ifndef __ITfRangeBackup_INTERFACE_DEFINED__
#define __ITfRangeBackup_INTERFACE_DEFINED__

/* interface ITfRangeBackup */
/* [unique][uuid][object] */ 


EXTERN_C const IID IID_ITfRangeBackup;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("463a506d-6992-49d2-9b88-93d55e70bb16")
    ITfRangeBackup : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE Restore( 
            /* [in] */ TfEditCookie ec,
            /* [in] */ __RPC__in_opt ITfRange *pRange) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ITfRangeBackupVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in ITfRangeBackup * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in ITfRangeBackup * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in ITfRangeBackup * This);
        
        DECLSPEC_XFGVIRT(ITfRangeBackup, Restore)
        HRESULT ( STDMETHODCALLTYPE *Restore )( 
            __RPC__in ITfRangeBackup * This,
            /* [in] */ TfEditCookie ec,
            /* [in] */ __RPC__in_opt ITfRange *pRange);
        
        END_INTERFACE
    } ITfRangeBackupVtbl;

    interface ITfRangeBackup
    {
        CONST_VTBL struct ITfRangeBackupVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ITfRangeBackup_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ITfRangeBackup_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ITfRangeBackup_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ITfRangeBackup_Restore(This,ec,pRange)	\
    ( (This)->lpVtbl -> Restore(This,ec,pRange) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ITfRangeBackup_INTERFACE_DEFINED__ */


#ifndef __ITfPropertyStore_INTERFACE_DEFINED__
#define __ITfPropertyStore_INTERFACE_DEFINED__

/* interface ITfPropertyStore */
/* [unique][uuid][object] */ 

#define	TF_TU_CORRECTION	( 0x1 )


EXTERN_C const IID IID_ITfPropertyStore;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("6834b120-88cb-11d2-bf45-00105a2799b5")
    ITfPropertyStore : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetType( 
            /* [out] */ __RPC__out GUID *pguid) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetDataType( 
            /* [out] */ __RPC__out DWORD *pdwReserved) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetData( 
            /* [out] */ __RPC__out VARIANT *pvarValue) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE OnTextUpdated( 
            /* [in] */ DWORD dwFlags,
            /* [in] */ __RPC__in_opt ITfRange *pRangeNew,
            /* [out] */ __RPC__out BOOL *pfAccept) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Shrink( 
            /* [in] */ __RPC__in_opt ITfRange *pRangeNew,
            /* [out] */ __RPC__out BOOL *pfFree) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Divide( 
            /* [in] */ __RPC__in_opt ITfRange *pRangeThis,
            /* [in] */ __RPC__in_opt ITfRange *pRangeNew,
            /* [out] */ __RPC__deref_out_opt ITfPropertyStore **ppPropStore) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Clone( 
            /* [out] */ __RPC__deref_out_opt ITfPropertyStore **pPropStore) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetPropertyRangeCreator( 
            /* [out] */ __RPC__out CLSID *pclsid) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Serialize( 
            /* [in] */ __RPC__in_opt IStream *pStream,
            /* [out] */ __RPC__out ULONG *pcb) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ITfPropertyStoreVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in ITfPropertyStore * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in ITfPropertyStore * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in ITfPropertyStore * This);
        
        DECLSPEC_XFGVIRT(ITfPropertyStore, GetType)
        HRESULT ( STDMETHODCALLTYPE *GetType )( 
            __RPC__in ITfPropertyStore * This,
            /* [out] */ __RPC__out GUID *pguid);
        
        DECLSPEC_XFGVIRT(ITfPropertyStore, GetDataType)
        HRESULT ( STDMETHODCALLTYPE *GetDataType )( 
            __RPC__in ITfPropertyStore * This,
            /* [out] */ __RPC__out DWORD *pdwReserved);
        
        DECLSPEC_XFGVIRT(ITfPropertyStore, GetData)
        HRESULT ( STDMETHODCALLTYPE *GetData )( 
            __RPC__in ITfPropertyStore * This,
            /* [out] */ __RPC__out VARIANT *pvarValue);
        
        DECLSPEC_XFGVIRT(ITfPropertyStore, OnTextUpdated)
        HRESULT ( STDMETHODCALLTYPE *OnTextUpdated )( 
            __RPC__in ITfPropertyStore * This,
            /* [in] */ DWORD dwFlags,
            /* [in] */ __RPC__in_opt ITfRange *pRangeNew,
            /* [out] */ __RPC__out BOOL *pfAccept);
        
        DECLSPEC_XFGVIRT(ITfPropertyStore, Shrink)
        HRESULT ( STDMETHODCALLTYPE *Shrink )( 
            __RPC__in ITfPropertyStore * This,
            /* [in] */ __RPC__in_opt ITfRange *pRangeNew,
            /* [out] */ __RPC__out BOOL *pfFree);
        
        DECLSPEC_XFGVIRT(ITfPropertyStore, Divide)
        HRESULT ( STDMETHODCALLTYPE *Divide )( 
            __RPC__in ITfPropertyStore * This,
            /* [in] */ __RPC__in_opt ITfRange *pRangeThis,
            /* [in] */ __RPC__in_opt ITfRange *pRangeNew,
            /* [out] */ __RPC__deref_out_opt ITfPropertyStore **ppPropStore);
        
        DECLSPEC_XFGVIRT(ITfPropertyStore, Clone)
        HRESULT ( STDMETHODCALLTYPE *Clone )( 
            __RPC__in ITfPropertyStore * This,
            /* [out] */ __RPC__deref_out_opt ITfPropertyStore **pPropStore);
        
        DECLSPEC_XFGVIRT(ITfPropertyStore, GetPropertyRangeCreator)
        HRESULT ( STDMETHODCALLTYPE *GetPropertyRangeCreator )( 
            __RPC__in ITfPropertyStore * This,
            /* [out] */ __RPC__out CLSID *pclsid);
        
        DECLSPEC_XFGVIRT(ITfPropertyStore, Serialize)
        HRESULT ( STDMETHODCALLTYPE *Serialize )( 
            __RPC__in ITfPropertyStore * This,
            /* [in] */ __RPC__in_opt IStream *pStream,
            /* [out] */ __RPC__out ULONG *pcb);
        
        END_INTERFACE
    } ITfPropertyStoreVtbl;

    interface ITfPropertyStore
    {
        CONST_VTBL struct ITfPropertyStoreVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ITfPropertyStore_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ITfPropertyStore_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ITfPropertyStore_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ITfPropertyStore_GetType(This,pguid)	\
    ( (This)->lpVtbl -> GetType(This,pguid) ) 

#define ITfPropertyStore_GetDataType(This,pdwReserved)	\
    ( (This)->lpVtbl -> GetDataType(This,pdwReserved) ) 

#define ITfPropertyStore_GetData(This,pvarValue)	\
    ( (This)->lpVtbl -> GetData(This,pvarValue) ) 

#define ITfPropertyStore_OnTextUpdated(This,dwFlags,pRangeNew,pfAccept)	\
    ( (This)->lpVtbl -> OnTextUpdated(This,dwFlags,pRangeNew,pfAccept) ) 

#define ITfPropertyStore_Shrink(This,pRangeNew,pfFree)	\
    ( (This)->lpVtbl -> Shrink(This,pRangeNew,pfFree) ) 

#define ITfPropertyStore_Divide(This,pRangeThis,pRangeNew,ppPropStore)	\
    ( (This)->lpVtbl -> Divide(This,pRangeThis,pRangeNew,ppPropStore) ) 

#define ITfPropertyStore_Clone(This,pPropStore)	\
    ( (This)->lpVtbl -> Clone(This,pPropStore) ) 

#define ITfPropertyStore_GetPropertyRangeCreator(This,pclsid)	\
    ( (This)->lpVtbl -> GetPropertyRangeCreator(This,pclsid) ) 

#define ITfPropertyStore_Serialize(This,pStream,pcb)	\
    ( (This)->lpVtbl -> Serialize(This,pStream,pcb) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ITfPropertyStore_INTERFACE_DEFINED__ */


#ifndef __IEnumTfRanges_INTERFACE_DEFINED__
#define __IEnumTfRanges_INTERFACE_DEFINED__

/* interface IEnumTfRanges */
/* [unique][uuid][object] */ 


EXTERN_C const IID IID_IEnumTfRanges;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("f99d3f40-8e32-11d2-bf46-00105a2799b5")
    IEnumTfRanges : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE Clone( 
            /* [out] */ __RPC__deref_out_opt IEnumTfRanges **ppEnum) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Next( 
            /* [in] */ ULONG ulCount,
            /* [length_is][size_is][out] */ __RPC__out_ecount_part(ulCount, *pcFetched) ITfRange **ppRange,
            /* [out] */ __RPC__out ULONG *pcFetched) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Reset( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Skip( 
            ULONG ulCount) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IEnumTfRangesVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IEnumTfRanges * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IEnumTfRanges * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IEnumTfRanges * This);
        
        DECLSPEC_XFGVIRT(IEnumTfRanges, Clone)
        HRESULT ( STDMETHODCALLTYPE *Clone )( 
            __RPC__in IEnumTfRanges * This,
            /* [out] */ __RPC__deref_out_opt IEnumTfRanges **ppEnum);
        
        DECLSPEC_XFGVIRT(IEnumTfRanges, Next)
        HRESULT ( STDMETHODCALLTYPE *Next )( 
            __RPC__in IEnumTfRanges * This,
            /* [in] */ ULONG ulCount,
            /* [length_is][size_is][out] */ __RPC__out_ecount_part(ulCount, *pcFetched) ITfRange **ppRange,
            /* [out] */ __RPC__out ULONG *pcFetched);
        
        DECLSPEC_XFGVIRT(IEnumTfRanges, Reset)
        HRESULT ( STDMETHODCALLTYPE *Reset )( 
            __RPC__in IEnumTfRanges * This);
        
        DECLSPEC_XFGVIRT(IEnumTfRanges, Skip)
        HRESULT ( STDMETHODCALLTYPE *Skip )( 
            __RPC__in IEnumTfRanges * This,
            ULONG ulCount);
        
        END_INTERFACE
    } IEnumTfRangesVtbl;

    interface IEnumTfRanges
    {
        CONST_VTBL struct IEnumTfRangesVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IEnumTfRanges_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IEnumTfRanges_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IEnumTfRanges_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IEnumTfRanges_Clone(This,ppEnum)	\
    ( (This)->lpVtbl -> Clone(This,ppEnum) ) 

#define IEnumTfRanges_Next(This,ulCount,ppRange,pcFetched)	\
    ( (This)->lpVtbl -> Next(This,ulCount,ppRange,pcFetched) ) 

#define IEnumTfRanges_Reset(This)	\
    ( (This)->lpVtbl -> Reset(This) ) 

#define IEnumTfRanges_Skip(This,ulCount)	\
    ( (This)->lpVtbl -> Skip(This,ulCount) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IEnumTfRanges_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_msctf_0000_0042 */
/* [local] */ 

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP) */
#pragma endregion
#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)


extern RPC_IF_HANDLE __MIDL_itf_msctf_0000_0042_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_msctf_0000_0042_v0_0_s_ifspec;

#ifndef __ITfCreatePropertyStore_INTERFACE_DEFINED__
#define __ITfCreatePropertyStore_INTERFACE_DEFINED__

/* interface ITfCreatePropertyStore */
/* [unique][uuid][object] */ 


EXTERN_C const IID IID_ITfCreatePropertyStore;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("2463fbf0-b0af-11d2-afc5-00105a2799b5")
    ITfCreatePropertyStore : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE IsStoreSerializable( 
            /* [in] */ __RPC__in REFGUID guidProp,
            /* [in] */ __RPC__in_opt ITfRange *pRange,
            /* [in] */ __RPC__in_opt ITfPropertyStore *pPropStore,
            /* [out] */ __RPC__out BOOL *pfSerializable) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE CreatePropertyStore( 
            /* [in] */ __RPC__in REFGUID guidProp,
            /* [in] */ __RPC__in_opt ITfRange *pRange,
            /* [in] */ ULONG cb,
            /* [in] */ __RPC__in_opt IStream *pStream,
            /* [out] */ __RPC__deref_out_opt ITfPropertyStore **ppStore) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ITfCreatePropertyStoreVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in ITfCreatePropertyStore * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in ITfCreatePropertyStore * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in ITfCreatePropertyStore * This);
        
        DECLSPEC_XFGVIRT(ITfCreatePropertyStore, IsStoreSerializable)
        HRESULT ( STDMETHODCALLTYPE *IsStoreSerializable )( 
            __RPC__in ITfCreatePropertyStore * This,
            /* [in] */ __RPC__in REFGUID guidProp,
            /* [in] */ __RPC__in_opt ITfRange *pRange,
            /* [in] */ __RPC__in_opt ITfPropertyStore *pPropStore,
            /* [out] */ __RPC__out BOOL *pfSerializable);
        
        DECLSPEC_XFGVIRT(ITfCreatePropertyStore, CreatePropertyStore)
        HRESULT ( STDMETHODCALLTYPE *CreatePropertyStore )( 
            __RPC__in ITfCreatePropertyStore * This,
            /* [in] */ __RPC__in REFGUID guidProp,
            /* [in] */ __RPC__in_opt ITfRange *pRange,
            /* [in] */ ULONG cb,
            /* [in] */ __RPC__in_opt IStream *pStream,
            /* [out] */ __RPC__deref_out_opt ITfPropertyStore **ppStore);
        
        END_INTERFACE
    } ITfCreatePropertyStoreVtbl;

    interface ITfCreatePropertyStore
    {
        CONST_VTBL struct ITfCreatePropertyStoreVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ITfCreatePropertyStore_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ITfCreatePropertyStore_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ITfCreatePropertyStore_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ITfCreatePropertyStore_IsStoreSerializable(This,guidProp,pRange,pPropStore,pfSerializable)	\
    ( (This)->lpVtbl -> IsStoreSerializable(This,guidProp,pRange,pPropStore,pfSerializable) ) 

#define ITfCreatePropertyStore_CreatePropertyStore(This,guidProp,pRange,cb,pStream,ppStore)	\
    ( (This)->lpVtbl -> CreatePropertyStore(This,guidProp,pRange,cb,pStream,ppStore) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ITfCreatePropertyStore_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_msctf_0000_0043 */
/* [local] */ 

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion
#pragma region Application Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP)


extern RPC_IF_HANDLE __MIDL_itf_msctf_0000_0043_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_msctf_0000_0043_v0_0_s_ifspec;

#ifndef __ITfPersistentPropertyLoaderACP_INTERFACE_DEFINED__
#define __ITfPersistentPropertyLoaderACP_INTERFACE_DEFINED__

/* interface ITfPersistentPropertyLoaderACP */
/* [unique][uuid][object] */ 


EXTERN_C const IID IID_ITfPersistentPropertyLoaderACP;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("4ef89150-0807-11d3-8df0-00105a2799b5")
    ITfPersistentPropertyLoaderACP : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE LoadProperty( 
            /* [in] */ __RPC__in const TF_PERSISTENT_PROPERTY_HEADER_ACP *pHdr,
            /* [out] */ __RPC__deref_out_opt IStream **ppStream) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ITfPersistentPropertyLoaderACPVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in ITfPersistentPropertyLoaderACP * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in ITfPersistentPropertyLoaderACP * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in ITfPersistentPropertyLoaderACP * This);
        
        DECLSPEC_XFGVIRT(ITfPersistentPropertyLoaderACP, LoadProperty)
        HRESULT ( STDMETHODCALLTYPE *LoadProperty )( 
            __RPC__in ITfPersistentPropertyLoaderACP * This,
            /* [in] */ __RPC__in const TF_PERSISTENT_PROPERTY_HEADER_ACP *pHdr,
            /* [out] */ __RPC__deref_out_opt IStream **ppStream);
        
        END_INTERFACE
    } ITfPersistentPropertyLoaderACPVtbl;

    interface ITfPersistentPropertyLoaderACP
    {
        CONST_VTBL struct ITfPersistentPropertyLoaderACPVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ITfPersistentPropertyLoaderACP_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ITfPersistentPropertyLoaderACP_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ITfPersistentPropertyLoaderACP_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ITfPersistentPropertyLoaderACP_LoadProperty(This,pHdr,ppStream)	\
    ( (This)->lpVtbl -> LoadProperty(This,pHdr,ppStream) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ITfPersistentPropertyLoaderACP_INTERFACE_DEFINED__ */


#ifndef __ITfProperty_INTERFACE_DEFINED__
#define __ITfProperty_INTERFACE_DEFINED__

/* interface ITfProperty */
/* [unique][uuid][object] */ 


EXTERN_C const IID IID_ITfProperty;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("e2449660-9542-11d2-bf46-00105a2799b5")
    ITfProperty : public ITfReadOnlyProperty
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE FindRange( 
            /* [in] */ TfEditCookie ec,
            /* [in] */ __RPC__in_opt ITfRange *pRange,
            /* [out] */ __RPC__deref_out_opt ITfRange **ppRange,
            /* [in] */ TfAnchor aPos) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetValueStore( 
            /* [in] */ TfEditCookie ec,
            /* [in] */ __RPC__in_opt ITfRange *pRange,
            /* [in] */ __RPC__in_opt ITfPropertyStore *pPropStore) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetValue( 
            /* [in] */ TfEditCookie ec,
            /* [in] */ __RPC__in_opt ITfRange *pRange,
            /* [in] */ __RPC__in const VARIANT *pvarValue) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Clear( 
            /* [in] */ TfEditCookie ec,
            /* [in] */ __RPC__in_opt ITfRange *pRange) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ITfPropertyVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in ITfProperty * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in ITfProperty * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in ITfProperty * This);
        
        DECLSPEC_XFGVIRT(ITfReadOnlyProperty, GetType)
        HRESULT ( STDMETHODCALLTYPE *GetType )( 
            __RPC__in ITfProperty * This,
            /* [out] */ __RPC__out GUID *pguid);
        
        DECLSPEC_XFGVIRT(ITfReadOnlyProperty, EnumRanges)
        HRESULT ( STDMETHODCALLTYPE *EnumRanges )( 
            __RPC__in ITfProperty * This,
            /* [in] */ TfEditCookie ec,
            /* [out] */ __RPC__deref_out_opt IEnumTfRanges **ppEnum,
            /* [in] */ __RPC__in_opt ITfRange *pTargetRange);
        
        DECLSPEC_XFGVIRT(ITfReadOnlyProperty, GetValue)
        HRESULT ( STDMETHODCALLTYPE *GetValue )( 
            __RPC__in ITfProperty * This,
            /* [in] */ TfEditCookie ec,
            /* [in] */ __RPC__in_opt ITfRange *pRange,
            /* [out] */ __RPC__out VARIANT *pvarValue);
        
        DECLSPEC_XFGVIRT(ITfReadOnlyProperty, GetContext)
        HRESULT ( STDMETHODCALLTYPE *GetContext )( 
            __RPC__in ITfProperty * This,
            /* [out] */ __RPC__deref_out_opt ITfContext **ppContext);
        
        DECLSPEC_XFGVIRT(ITfProperty, FindRange)
        HRESULT ( STDMETHODCALLTYPE *FindRange )( 
            __RPC__in ITfProperty * This,
            /* [in] */ TfEditCookie ec,
            /* [in] */ __RPC__in_opt ITfRange *pRange,
            /* [out] */ __RPC__deref_out_opt ITfRange **ppRange,
            /* [in] */ TfAnchor aPos);
        
        DECLSPEC_XFGVIRT(ITfProperty, SetValueStore)
        HRESULT ( STDMETHODCALLTYPE *SetValueStore )( 
            __RPC__in ITfProperty * This,
            /* [in] */ TfEditCookie ec,
            /* [in] */ __RPC__in_opt ITfRange *pRange,
            /* [in] */ __RPC__in_opt ITfPropertyStore *pPropStore);
        
        DECLSPEC_XFGVIRT(ITfProperty, SetValue)
        HRESULT ( STDMETHODCALLTYPE *SetValue )( 
            __RPC__in ITfProperty * This,
            /* [in] */ TfEditCookie ec,
            /* [in] */ __RPC__in_opt ITfRange *pRange,
            /* [in] */ __RPC__in const VARIANT *pvarValue);
        
        DECLSPEC_XFGVIRT(ITfProperty, Clear)
        HRESULT ( STDMETHODCALLTYPE *Clear )( 
            __RPC__in ITfProperty * This,
            /* [in] */ TfEditCookie ec,
            /* [in] */ __RPC__in_opt ITfRange *pRange);
        
        END_INTERFACE
    } ITfPropertyVtbl;

    interface ITfProperty
    {
        CONST_VTBL struct ITfPropertyVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ITfProperty_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ITfProperty_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ITfProperty_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ITfProperty_GetType(This,pguid)	\
    ( (This)->lpVtbl -> GetType(This,pguid) ) 

#define ITfProperty_EnumRanges(This,ec,ppEnum,pTargetRange)	\
    ( (This)->lpVtbl -> EnumRanges(This,ec,ppEnum,pTargetRange) ) 

#define ITfProperty_GetValue(This,ec,pRange,pvarValue)	\
    ( (This)->lpVtbl -> GetValue(This,ec,pRange,pvarValue) ) 

#define ITfProperty_GetContext(This,ppContext)	\
    ( (This)->lpVtbl -> GetContext(This,ppContext) ) 


#define ITfProperty_FindRange(This,ec,pRange,ppRange,aPos)	\
    ( (This)->lpVtbl -> FindRange(This,ec,pRange,ppRange,aPos) ) 

#define ITfProperty_SetValueStore(This,ec,pRange,pPropStore)	\
    ( (This)->lpVtbl -> SetValueStore(This,ec,pRange,pPropStore) ) 

#define ITfProperty_SetValue(This,ec,pRange,pvarValue)	\
    ( (This)->lpVtbl -> SetValue(This,ec,pRange,pvarValue) ) 

#define ITfProperty_Clear(This,ec,pRange)	\
    ( (This)->lpVtbl -> Clear(This,ec,pRange) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ITfProperty_INTERFACE_DEFINED__ */


#ifndef __IEnumTfProperties_INTERFACE_DEFINED__
#define __IEnumTfProperties_INTERFACE_DEFINED__

/* interface IEnumTfProperties */
/* [unique][uuid][object] */ 


EXTERN_C const IID IID_IEnumTfProperties;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("19188cb0-aca9-11d2-afc5-00105a2799b5")
    IEnumTfProperties : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE Clone( 
            /* [out] */ __RPC__deref_out_opt IEnumTfProperties **ppEnum) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Next( 
            /* [in] */ ULONG ulCount,
            /* [length_is][size_is][out] */ __RPC__out_ecount_part(ulCount, *pcFetched) ITfProperty **ppProp,
            /* [out] */ __RPC__out ULONG *pcFetched) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Reset( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Skip( 
            /* [in] */ ULONG ulCount) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IEnumTfPropertiesVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IEnumTfProperties * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IEnumTfProperties * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IEnumTfProperties * This);
        
        DECLSPEC_XFGVIRT(IEnumTfProperties, Clone)
        HRESULT ( STDMETHODCALLTYPE *Clone )( 
            __RPC__in IEnumTfProperties * This,
            /* [out] */ __RPC__deref_out_opt IEnumTfProperties **ppEnum);
        
        DECLSPEC_XFGVIRT(IEnumTfProperties, Next)
        HRESULT ( STDMETHODCALLTYPE *Next )( 
            __RPC__in IEnumTfProperties * This,
            /* [in] */ ULONG ulCount,
            /* [length_is][size_is][out] */ __RPC__out_ecount_part(ulCount, *pcFetched) ITfProperty **ppProp,
            /* [out] */ __RPC__out ULONG *pcFetched);
        
        DECLSPEC_XFGVIRT(IEnumTfProperties, Reset)
        HRESULT ( STDMETHODCALLTYPE *Reset )( 
            __RPC__in IEnumTfProperties * This);
        
        DECLSPEC_XFGVIRT(IEnumTfProperties, Skip)
        HRESULT ( STDMETHODCALLTYPE *Skip )( 
            __RPC__in IEnumTfProperties * This,
            /* [in] */ ULONG ulCount);
        
        END_INTERFACE
    } IEnumTfPropertiesVtbl;

    interface IEnumTfProperties
    {
        CONST_VTBL struct IEnumTfPropertiesVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IEnumTfProperties_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IEnumTfProperties_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IEnumTfProperties_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IEnumTfProperties_Clone(This,ppEnum)	\
    ( (This)->lpVtbl -> Clone(This,ppEnum) ) 

#define IEnumTfProperties_Next(This,ulCount,ppProp,pcFetched)	\
    ( (This)->lpVtbl -> Next(This,ulCount,ppProp,pcFetched) ) 

#define IEnumTfProperties_Reset(This)	\
    ( (This)->lpVtbl -> Reset(This) ) 

#define IEnumTfProperties_Skip(This,ulCount)	\
    ( (This)->lpVtbl -> Skip(This,ulCount) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IEnumTfProperties_INTERFACE_DEFINED__ */


#ifndef __ITfCompartment_INTERFACE_DEFINED__
#define __ITfCompartment_INTERFACE_DEFINED__

/* interface ITfCompartment */
/* [unique][uuid][object] */ 


EXTERN_C const IID IID_ITfCompartment;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("bb08f7a9-607a-4384-8623-056892b64371")
    ITfCompartment : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE SetValue( 
            /* [in] */ TfClientId tid,
            /* [in] */ __RPC__in const VARIANT *pvarValue) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetValue( 
            /* [out] */ __RPC__out VARIANT *pvarValue) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ITfCompartmentVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in ITfCompartment * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in ITfCompartment * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in ITfCompartment * This);
        
        DECLSPEC_XFGVIRT(ITfCompartment, SetValue)
        HRESULT ( STDMETHODCALLTYPE *SetValue )( 
            __RPC__in ITfCompartment * This,
            /* [in] */ TfClientId tid,
            /* [in] */ __RPC__in const VARIANT *pvarValue);
        
        DECLSPEC_XFGVIRT(ITfCompartment, GetValue)
        HRESULT ( STDMETHODCALLTYPE *GetValue )( 
            __RPC__in ITfCompartment * This,
            /* [out] */ __RPC__out VARIANT *pvarValue);
        
        END_INTERFACE
    } ITfCompartmentVtbl;

    interface ITfCompartment
    {
        CONST_VTBL struct ITfCompartmentVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ITfCompartment_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ITfCompartment_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ITfCompartment_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ITfCompartment_SetValue(This,tid,pvarValue)	\
    ( (This)->lpVtbl -> SetValue(This,tid,pvarValue) ) 

#define ITfCompartment_GetValue(This,pvarValue)	\
    ( (This)->lpVtbl -> GetValue(This,pvarValue) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ITfCompartment_INTERFACE_DEFINED__ */


#ifndef __ITfCompartmentEventSink_INTERFACE_DEFINED__
#define __ITfCompartmentEventSink_INTERFACE_DEFINED__

/* interface ITfCompartmentEventSink */
/* [unique][uuid][object] */ 


EXTERN_C const IID IID_ITfCompartmentEventSink;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("743abd5f-f26d-48df-8cc5-238492419b64")
    ITfCompartmentEventSink : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE OnChange( 
            /* [in] */ __RPC__in REFGUID rguid) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ITfCompartmentEventSinkVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in ITfCompartmentEventSink * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in ITfCompartmentEventSink * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in ITfCompartmentEventSink * This);
        
        DECLSPEC_XFGVIRT(ITfCompartmentEventSink, OnChange)
        HRESULT ( STDMETHODCALLTYPE *OnChange )( 
            __RPC__in ITfCompartmentEventSink * This,
            /* [in] */ __RPC__in REFGUID rguid);
        
        END_INTERFACE
    } ITfCompartmentEventSinkVtbl;

    interface ITfCompartmentEventSink
    {
        CONST_VTBL struct ITfCompartmentEventSinkVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ITfCompartmentEventSink_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ITfCompartmentEventSink_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ITfCompartmentEventSink_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ITfCompartmentEventSink_OnChange(This,rguid)	\
    ( (This)->lpVtbl -> OnChange(This,rguid) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ITfCompartmentEventSink_INTERFACE_DEFINED__ */


#ifndef __ITfCompartmentMgr_INTERFACE_DEFINED__
#define __ITfCompartmentMgr_INTERFACE_DEFINED__

/* interface ITfCompartmentMgr */
/* [unique][uuid][object] */ 


EXTERN_C const IID IID_ITfCompartmentMgr;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("7dcf57ac-18ad-438b-824d-979bffb74b7c")
    ITfCompartmentMgr : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetCompartment( 
            /* [in] */ __RPC__in REFGUID rguid,
            /* [out] */ __RPC__deref_out_opt ITfCompartment **ppcomp) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE ClearCompartment( 
            /* [in] */ TfClientId tid,
            /* [in] */ __RPC__in REFGUID rguid) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE EnumCompartments( 
            /* [out] */ __RPC__deref_out_opt IEnumGUID **ppEnum) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ITfCompartmentMgrVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in ITfCompartmentMgr * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in ITfCompartmentMgr * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in ITfCompartmentMgr * This);
        
        DECLSPEC_XFGVIRT(ITfCompartmentMgr, GetCompartment)
        HRESULT ( STDMETHODCALLTYPE *GetCompartment )( 
            __RPC__in ITfCompartmentMgr * This,
            /* [in] */ __RPC__in REFGUID rguid,
            /* [out] */ __RPC__deref_out_opt ITfCompartment **ppcomp);
        
        DECLSPEC_XFGVIRT(ITfCompartmentMgr, ClearCompartment)
        HRESULT ( STDMETHODCALLTYPE *ClearCompartment )( 
            __RPC__in ITfCompartmentMgr * This,
            /* [in] */ TfClientId tid,
            /* [in] */ __RPC__in REFGUID rguid);
        
        DECLSPEC_XFGVIRT(ITfCompartmentMgr, EnumCompartments)
        HRESULT ( STDMETHODCALLTYPE *EnumCompartments )( 
            __RPC__in ITfCompartmentMgr * This,
            /* [out] */ __RPC__deref_out_opt IEnumGUID **ppEnum);
        
        END_INTERFACE
    } ITfCompartmentMgrVtbl;

    interface ITfCompartmentMgr
    {
        CONST_VTBL struct ITfCompartmentMgrVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ITfCompartmentMgr_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ITfCompartmentMgr_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ITfCompartmentMgr_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ITfCompartmentMgr_GetCompartment(This,rguid,ppcomp)	\
    ( (This)->lpVtbl -> GetCompartment(This,rguid,ppcomp) ) 

#define ITfCompartmentMgr_ClearCompartment(This,tid,rguid)	\
    ( (This)->lpVtbl -> ClearCompartment(This,tid,rguid) ) 

#define ITfCompartmentMgr_EnumCompartments(This,ppEnum)	\
    ( (This)->lpVtbl -> EnumCompartments(This,ppEnum) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ITfCompartmentMgr_INTERFACE_DEFINED__ */


#ifndef __ITfFunction_INTERFACE_DEFINED__
#define __ITfFunction_INTERFACE_DEFINED__

/* interface ITfFunction */
/* [unique][uuid][object] */ 


EXTERN_C const IID IID_ITfFunction;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("db593490-098f-11d3-8df0-00105a2799b5")
    ITfFunction : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetDisplayName( 
            /* [out] */ __RPC__deref_out_opt BSTR *pbstrName) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ITfFunctionVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in ITfFunction * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in ITfFunction * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in ITfFunction * This);
        
        DECLSPEC_XFGVIRT(ITfFunction, GetDisplayName)
        HRESULT ( STDMETHODCALLTYPE *GetDisplayName )( 
            __RPC__in ITfFunction * This,
            /* [out] */ __RPC__deref_out_opt BSTR *pbstrName);
        
        END_INTERFACE
    } ITfFunctionVtbl;

    interface ITfFunction
    {
        CONST_VTBL struct ITfFunctionVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ITfFunction_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ITfFunction_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ITfFunction_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ITfFunction_GetDisplayName(This,pbstrName)	\
    ( (This)->lpVtbl -> GetDisplayName(This,pbstrName) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ITfFunction_INTERFACE_DEFINED__ */


#ifndef __ITfFunctionProvider_INTERFACE_DEFINED__
#define __ITfFunctionProvider_INTERFACE_DEFINED__

/* interface ITfFunctionProvider */
/* [unique][uuid][object] */ 


EXTERN_C const IID IID_ITfFunctionProvider;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("101d6610-0990-11d3-8df0-00105a2799b5")
    ITfFunctionProvider : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetType( 
            /* [out] */ __RPC__out GUID *pguid) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetDescription( 
            /* [out] */ __RPC__deref_out_opt BSTR *pbstrDesc) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetFunction( 
            /* [in] */ __RPC__in REFGUID rguid,
            /* [in] */ __RPC__in REFIID riid,
            /* [iid_is][out] */ __RPC__deref_out_opt IUnknown **ppunk) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ITfFunctionProviderVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in ITfFunctionProvider * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in ITfFunctionProvider * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in ITfFunctionProvider * This);
        
        DECLSPEC_XFGVIRT(ITfFunctionProvider, GetType)
        HRESULT ( STDMETHODCALLTYPE *GetType )( 
            __RPC__in ITfFunctionProvider * This,
            /* [out] */ __RPC__out GUID *pguid);
        
        DECLSPEC_XFGVIRT(ITfFunctionProvider, GetDescription)
        HRESULT ( STDMETHODCALLTYPE *GetDescription )( 
            __RPC__in ITfFunctionProvider * This,
            /* [out] */ __RPC__deref_out_opt BSTR *pbstrDesc);
        
        DECLSPEC_XFGVIRT(ITfFunctionProvider, GetFunction)
        HRESULT ( STDMETHODCALLTYPE *GetFunction )( 
            __RPC__in ITfFunctionProvider * This,
            /* [in] */ __RPC__in REFGUID rguid,
            /* [in] */ __RPC__in REFIID riid,
            /* [iid_is][out] */ __RPC__deref_out_opt IUnknown **ppunk);
        
        END_INTERFACE
    } ITfFunctionProviderVtbl;

    interface ITfFunctionProvider
    {
        CONST_VTBL struct ITfFunctionProviderVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ITfFunctionProvider_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ITfFunctionProvider_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ITfFunctionProvider_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ITfFunctionProvider_GetType(This,pguid)	\
    ( (This)->lpVtbl -> GetType(This,pguid) ) 

#define ITfFunctionProvider_GetDescription(This,pbstrDesc)	\
    ( (This)->lpVtbl -> GetDescription(This,pbstrDesc) ) 

#define ITfFunctionProvider_GetFunction(This,rguid,riid,ppunk)	\
    ( (This)->lpVtbl -> GetFunction(This,rguid,riid,ppunk) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ITfFunctionProvider_INTERFACE_DEFINED__ */


#ifndef __IEnumTfFunctionProviders_INTERFACE_DEFINED__
#define __IEnumTfFunctionProviders_INTERFACE_DEFINED__

/* interface IEnumTfFunctionProviders */
/* [unique][uuid][object] */ 


EXTERN_C const IID IID_IEnumTfFunctionProviders;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("e4b24db0-0990-11d3-8df0-00105a2799b5")
    IEnumTfFunctionProviders : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE Clone( 
            /* [out] */ __RPC__deref_out_opt IEnumTfFunctionProviders **ppEnum) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Next( 
            /* [in] */ ULONG ulCount,
            /* [length_is][size_is][out] */ __RPC__out_ecount_part(ulCount, *pcFetch) ITfFunctionProvider **ppCmdobj,
            /* [out] */ __RPC__out ULONG *pcFetch) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Reset( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Skip( 
            /* [in] */ ULONG ulCount) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IEnumTfFunctionProvidersVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IEnumTfFunctionProviders * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IEnumTfFunctionProviders * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IEnumTfFunctionProviders * This);
        
        DECLSPEC_XFGVIRT(IEnumTfFunctionProviders, Clone)
        HRESULT ( STDMETHODCALLTYPE *Clone )( 
            __RPC__in IEnumTfFunctionProviders * This,
            /* [out] */ __RPC__deref_out_opt IEnumTfFunctionProviders **ppEnum);
        
        DECLSPEC_XFGVIRT(IEnumTfFunctionProviders, Next)
        HRESULT ( STDMETHODCALLTYPE *Next )( 
            __RPC__in IEnumTfFunctionProviders * This,
            /* [in] */ ULONG ulCount,
            /* [length_is][size_is][out] */ __RPC__out_ecount_part(ulCount, *pcFetch) ITfFunctionProvider **ppCmdobj,
            /* [out] */ __RPC__out ULONG *pcFetch);
        
        DECLSPEC_XFGVIRT(IEnumTfFunctionProviders, Reset)
        HRESULT ( STDMETHODCALLTYPE *Reset )( 
            __RPC__in IEnumTfFunctionProviders * This);
        
        DECLSPEC_XFGVIRT(IEnumTfFunctionProviders, Skip)
        HRESULT ( STDMETHODCALLTYPE *Skip )( 
            __RPC__in IEnumTfFunctionProviders * This,
            /* [in] */ ULONG ulCount);
        
        END_INTERFACE
    } IEnumTfFunctionProvidersVtbl;

    interface IEnumTfFunctionProviders
    {
        CONST_VTBL struct IEnumTfFunctionProvidersVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IEnumTfFunctionProviders_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IEnumTfFunctionProviders_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IEnumTfFunctionProviders_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IEnumTfFunctionProviders_Clone(This,ppEnum)	\
    ( (This)->lpVtbl -> Clone(This,ppEnum) ) 

#define IEnumTfFunctionProviders_Next(This,ulCount,ppCmdobj,pcFetch)	\
    ( (This)->lpVtbl -> Next(This,ulCount,ppCmdobj,pcFetch) ) 

#define IEnumTfFunctionProviders_Reset(This)	\
    ( (This)->lpVtbl -> Reset(This) ) 

#define IEnumTfFunctionProviders_Skip(This,ulCount)	\
    ( (This)->lpVtbl -> Skip(This,ulCount) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IEnumTfFunctionProviders_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_msctf_0000_0052 */
/* [local] */ 

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP) */
#pragma endregion
#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)


extern RPC_IF_HANDLE __MIDL_itf_msctf_0000_0052_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_msctf_0000_0052_v0_0_s_ifspec;

#ifndef __ITfInputProcessorProfiles_INTERFACE_DEFINED__
#define __ITfInputProcessorProfiles_INTERFACE_DEFINED__

/* interface ITfInputProcessorProfiles */
/* [unique][uuid][object] */ 


EXTERN_C const IID IID_ITfInputProcessorProfiles;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("1F02B6C5-7842-4EE6-8A0B-9A24183A95CA")
    ITfInputProcessorProfiles : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE Register( 
            /* [in] */ __RPC__in REFCLSID rclsid) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Unregister( 
            /* [in] */ __RPC__in REFCLSID rclsid) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE AddLanguageProfile( 
            /* [in] */ __RPC__in REFCLSID rclsid,
            /* [in] */ LANGID langid,
            /* [in] */ __RPC__in REFGUID guidProfile,
            /* [size_is][in] */ __RPC__in_ecount_full(cchDesc) const WCHAR *pchDesc,
            /* [in] */ ULONG cchDesc,
            /* [size_is][in] */ __RPC__in_ecount_full(cchFile) const WCHAR *pchIconFile,
            /* [in] */ ULONG cchFile,
            /* [in] */ ULONG uIconIndex) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE RemoveLanguageProfile( 
            /* [in] */ __RPC__in REFCLSID rclsid,
            /* [in] */ LANGID langid,
            /* [in] */ __RPC__in REFGUID guidProfile) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE EnumInputProcessorInfo( 
            /* [out] */ __RPC__deref_out_opt IEnumGUID **ppEnum) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetDefaultLanguageProfile( 
            /* [in] */ LANGID langid,
            /* [in] */ __RPC__in REFGUID catid,
            /* [out] */ __RPC__out CLSID *pclsid,
            /* [out] */ __RPC__out GUID *pguidProfile) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetDefaultLanguageProfile( 
            /* [in] */ LANGID langid,
            /* [in] */ __RPC__in REFCLSID rclsid,
            /* [in] */ __RPC__in REFGUID guidProfiles) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE ActivateLanguageProfile( 
            /* [in] */ __RPC__in REFCLSID rclsid,
            /* [in] */ LANGID langid,
            /* [in] */ __RPC__in REFGUID guidProfiles) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetActiveLanguageProfile( 
            /* [in] */ __RPC__in REFCLSID rclsid,
            /* [out] */ __RPC__out LANGID *plangid,
            /* [out] */ __RPC__out GUID *pguidProfile) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetLanguageProfileDescription( 
            /* [in] */ __RPC__in REFCLSID rclsid,
            /* [in] */ LANGID langid,
            /* [in] */ __RPC__in REFGUID guidProfile,
            /* [out] */ __RPC__deref_out_opt BSTR *pbstrProfile) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetCurrentLanguage( 
            /* [out] */ __RPC__out LANGID *plangid) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE ChangeCurrentLanguage( 
            /* [in] */ LANGID langid) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetLanguageList( 
            /* [size_is][size_is][out] */ __RPC__deref_out_ecount_full_opt(*pulCount) LANGID **ppLangId,
            /* [out] */ __RPC__out ULONG *pulCount) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE EnumLanguageProfiles( 
            /* [in] */ LANGID langid,
            /* [out] */ __RPC__deref_out_opt IEnumTfLanguageProfiles **ppEnum) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE EnableLanguageProfile( 
            /* [in] */ __RPC__in REFCLSID rclsid,
            /* [in] */ LANGID langid,
            /* [in] */ __RPC__in REFGUID guidProfile,
            /* [in] */ BOOL fEnable) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE IsEnabledLanguageProfile( 
            /* [in] */ __RPC__in REFCLSID rclsid,
            /* [in] */ LANGID langid,
            /* [in] */ __RPC__in REFGUID guidProfile,
            /* [out] */ __RPC__out BOOL *pfEnable) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE EnableLanguageProfileByDefault( 
            /* [in] */ __RPC__in REFCLSID rclsid,
            /* [in] */ LANGID langid,
            /* [in] */ __RPC__in REFGUID guidProfile,
            /* [in] */ BOOL fEnable) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SubstituteKeyboardLayout( 
            /* [in] */ __RPC__in REFCLSID rclsid,
            /* [in] */ LANGID langid,
            /* [in] */ __RPC__in REFGUID guidProfile,
            /* [in] */ HKL hKL) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ITfInputProcessorProfilesVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in ITfInputProcessorProfiles * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in ITfInputProcessorProfiles * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in ITfInputProcessorProfiles * This);
        
        DECLSPEC_XFGVIRT(ITfInputProcessorProfiles, Register)
        HRESULT ( STDMETHODCALLTYPE *Register )( 
            __RPC__in ITfInputProcessorProfiles * This,
            /* [in] */ __RPC__in REFCLSID rclsid);
        
        DECLSPEC_XFGVIRT(ITfInputProcessorProfiles, Unregister)
        HRESULT ( STDMETHODCALLTYPE *Unregister )( 
            __RPC__in ITfInputProcessorProfiles * This,
            /* [in] */ __RPC__in REFCLSID rclsid);
        
        DECLSPEC_XFGVIRT(ITfInputProcessorProfiles, AddLanguageProfile)
        HRESULT ( STDMETHODCALLTYPE *AddLanguageProfile )( 
            __RPC__in ITfInputProcessorProfiles * This,
            /* [in] */ __RPC__in REFCLSID rclsid,
            /* [in] */ LANGID langid,
            /* [in] */ __RPC__in REFGUID guidProfile,
            /* [size_is][in] */ __RPC__in_ecount_full(cchDesc) const WCHAR *pchDesc,
            /* [in] */ ULONG cchDesc,
            /* [size_is][in] */ __RPC__in_ecount_full(cchFile) const WCHAR *pchIconFile,
            /* [in] */ ULONG cchFile,
            /* [in] */ ULONG uIconIndex);
        
        DECLSPEC_XFGVIRT(ITfInputProcessorProfiles, RemoveLanguageProfile)
        HRESULT ( STDMETHODCALLTYPE *RemoveLanguageProfile )( 
            __RPC__in ITfInputProcessorProfiles * This,
            /* [in] */ __RPC__in REFCLSID rclsid,
            /* [in] */ LANGID langid,
            /* [in] */ __RPC__in REFGUID guidProfile);
        
        DECLSPEC_XFGVIRT(ITfInputProcessorProfiles, EnumInputProcessorInfo)
        HRESULT ( STDMETHODCALLTYPE *EnumInputProcessorInfo )( 
            __RPC__in ITfInputProcessorProfiles * This,
            /* [out] */ __RPC__deref_out_opt IEnumGUID **ppEnum);
        
        DECLSPEC_XFGVIRT(ITfInputProcessorProfiles, GetDefaultLanguageProfile)
        HRESULT ( STDMETHODCALLTYPE *GetDefaultLanguageProfile )( 
            __RPC__in ITfInputProcessorProfiles * This,
            /* [in] */ LANGID langid,
            /* [in] */ __RPC__in REFGUID catid,
            /* [out] */ __RPC__out CLSID *pclsid,
            /* [out] */ __RPC__out GUID *pguidProfile);
        
        DECLSPEC_XFGVIRT(ITfInputProcessorProfiles, SetDefaultLanguageProfile)
        HRESULT ( STDMETHODCALLTYPE *SetDefaultLanguageProfile )( 
            __RPC__in ITfInputProcessorProfiles * This,
            /* [in] */ LANGID langid,
            /* [in] */ __RPC__in REFCLSID rclsid,
            /* [in] */ __RPC__in REFGUID guidProfiles);
        
        DECLSPEC_XFGVIRT(ITfInputProcessorProfiles, ActivateLanguageProfile)
        HRESULT ( STDMETHODCALLTYPE *ActivateLanguageProfile )( 
            __RPC__in ITfInputProcessorProfiles * This,
            /* [in] */ __RPC__in REFCLSID rclsid,
            /* [in] */ LANGID langid,
            /* [in] */ __RPC__in REFGUID guidProfiles);
        
        DECLSPEC_XFGVIRT(ITfInputProcessorProfiles, GetActiveLanguageProfile)
        HRESULT ( STDMETHODCALLTYPE *GetActiveLanguageProfile )( 
            __RPC__in ITfInputProcessorProfiles * This,
            /* [in] */ __RPC__in REFCLSID rclsid,
            /* [out] */ __RPC__out LANGID *plangid,
            /* [out] */ __RPC__out GUID *pguidProfile);
        
        DECLSPEC_XFGVIRT(ITfInputProcessorProfiles, GetLanguageProfileDescription)
        HRESULT ( STDMETHODCALLTYPE *GetLanguageProfileDescription )( 
            __RPC__in ITfInputProcessorProfiles * This,
            /* [in] */ __RPC__in REFCLSID rclsid,
            /* [in] */ LANGID langid,
            /* [in] */ __RPC__in REFGUID guidProfile,
            /* [out] */ __RPC__deref_out_opt BSTR *pbstrProfile);
        
        DECLSPEC_XFGVIRT(ITfInputProcessorProfiles, GetCurrentLanguage)
        HRESULT ( STDMETHODCALLTYPE *GetCurrentLanguage )( 
            __RPC__in ITfInputProcessorProfiles * This,
            /* [out] */ __RPC__out LANGID *plangid);
        
        DECLSPEC_XFGVIRT(ITfInputProcessorProfiles, ChangeCurrentLanguage)
        HRESULT ( STDMETHODCALLTYPE *ChangeCurrentLanguage )( 
            __RPC__in ITfInputProcessorProfiles * This,
            /* [in] */ LANGID langid);
        
        DECLSPEC_XFGVIRT(ITfInputProcessorProfiles, GetLanguageList)
        HRESULT ( STDMETHODCALLTYPE *GetLanguageList )( 
            __RPC__in ITfInputProcessorProfiles * This,
            /* [size_is][size_is][out] */ __RPC__deref_out_ecount_full_opt(*pulCount) LANGID **ppLangId,
            /* [out] */ __RPC__out ULONG *pulCount);
        
        DECLSPEC_XFGVIRT(ITfInputProcessorProfiles, EnumLanguageProfiles)
        HRESULT ( STDMETHODCALLTYPE *EnumLanguageProfiles )( 
            __RPC__in ITfInputProcessorProfiles * This,
            /* [in] */ LANGID langid,
            /* [out] */ __RPC__deref_out_opt IEnumTfLanguageProfiles **ppEnum);
        
        DECLSPEC_XFGVIRT(ITfInputProcessorProfiles, EnableLanguageProfile)
        HRESULT ( STDMETHODCALLTYPE *EnableLanguageProfile )( 
            __RPC__in ITfInputProcessorProfiles * This,
            /* [in] */ __RPC__in REFCLSID rclsid,
            /* [in] */ LANGID langid,
            /* [in] */ __RPC__in REFGUID guidProfile,
            /* [in] */ BOOL fEnable);
        
        DECLSPEC_XFGVIRT(ITfInputProcessorProfiles, IsEnabledLanguageProfile)
        HRESULT ( STDMETHODCALLTYPE *IsEnabledLanguageProfile )( 
            __RPC__in ITfInputProcessorProfiles * This,
            /* [in] */ __RPC__in REFCLSID rclsid,
            /* [in] */ LANGID langid,
            /* [in] */ __RPC__in REFGUID guidProfile,
            /* [out] */ __RPC__out BOOL *pfEnable);
        
        DECLSPEC_XFGVIRT(ITfInputProcessorProfiles, EnableLanguageProfileByDefault)
        HRESULT ( STDMETHODCALLTYPE *EnableLanguageProfileByDefault )( 
            __RPC__in ITfInputProcessorProfiles * This,
            /* [in] */ __RPC__in REFCLSID rclsid,
            /* [in] */ LANGID langid,
            /* [in] */ __RPC__in REFGUID guidProfile,
            /* [in] */ BOOL fEnable);
        
        DECLSPEC_XFGVIRT(ITfInputProcessorProfiles, SubstituteKeyboardLayout)
        HRESULT ( STDMETHODCALLTYPE *SubstituteKeyboardLayout )( 
            __RPC__in ITfInputProcessorProfiles * This,
            /* [in] */ __RPC__in REFCLSID rclsid,
            /* [in] */ LANGID langid,
            /* [in] */ __RPC__in REFGUID guidProfile,
            /* [in] */ HKL hKL);
        
        END_INTERFACE
    } ITfInputProcessorProfilesVtbl;

    interface ITfInputProcessorProfiles
    {
        CONST_VTBL struct ITfInputProcessorProfilesVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ITfInputProcessorProfiles_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ITfInputProcessorProfiles_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ITfInputProcessorProfiles_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ITfInputProcessorProfiles_Register(This,rclsid)	\
    ( (This)->lpVtbl -> Register(This,rclsid) ) 

#define ITfInputProcessorProfiles_Unregister(This,rclsid)	\
    ( (This)->lpVtbl -> Unregister(This,rclsid) ) 

#define ITfInputProcessorProfiles_AddLanguageProfile(This,rclsid,langid,guidProfile,pchDesc,cchDesc,pchIconFile,cchFile,uIconIndex)	\
    ( (This)->lpVtbl -> AddLanguageProfile(This,rclsid,langid,guidProfile,pchDesc,cchDesc,pchIconFile,cchFile,uIconIndex) ) 

#define ITfInputProcessorProfiles_RemoveLanguageProfile(This,rclsid,langid,guidProfile)	\
    ( (This)->lpVtbl -> RemoveLanguageProfile(This,rclsid,langid,guidProfile) ) 

#define ITfInputProcessorProfiles_EnumInputProcessorInfo(This,ppEnum)	\
    ( (This)->lpVtbl -> EnumInputProcessorInfo(This,ppEnum) ) 

#define ITfInputProcessorProfiles_GetDefaultLanguageProfile(This,langid,catid,pclsid,pguidProfile)	\
    ( (This)->lpVtbl -> GetDefaultLanguageProfile(This,langid,catid,pclsid,pguidProfile) ) 

#define ITfInputProcessorProfiles_SetDefaultLanguageProfile(This,langid,rclsid,guidProfiles)	\
    ( (This)->lpVtbl -> SetDefaultLanguageProfile(This,langid,rclsid,guidProfiles) ) 

#define ITfInputProcessorProfiles_ActivateLanguageProfile(This,rclsid,langid,guidProfiles)	\
    ( (This)->lpVtbl -> ActivateLanguageProfile(This,rclsid,langid,guidProfiles) ) 

#define ITfInputProcessorProfiles_GetActiveLanguageProfile(This,rclsid,plangid,pguidProfile)	\
    ( (This)->lpVtbl -> GetActiveLanguageProfile(This,rclsid,plangid,pguidProfile) ) 

#define ITfInputProcessorProfiles_GetLanguageProfileDescription(This,rclsid,langid,guidProfile,pbstrProfile)	\
    ( (This)->lpVtbl -> GetLanguageProfileDescription(This,rclsid,langid,guidProfile,pbstrProfile) ) 

#define ITfInputProcessorProfiles_GetCurrentLanguage(This,plangid)	\
    ( (This)->lpVtbl -> GetCurrentLanguage(This,plangid) ) 

#define ITfInputProcessorProfiles_ChangeCurrentLanguage(This,langid)	\
    ( (This)->lpVtbl -> ChangeCurrentLanguage(This,langid) ) 

#define ITfInputProcessorProfiles_GetLanguageList(This,ppLangId,pulCount)	\
    ( (This)->lpVtbl -> GetLanguageList(This,ppLangId,pulCount) ) 

#define ITfInputProcessorProfiles_EnumLanguageProfiles(This,langid,ppEnum)	\
    ( (This)->lpVtbl -> EnumLanguageProfiles(This,langid,ppEnum) ) 

#define ITfInputProcessorProfiles_EnableLanguageProfile(This,rclsid,langid,guidProfile,fEnable)	\
    ( (This)->lpVtbl -> EnableLanguageProfile(This,rclsid,langid,guidProfile,fEnable) ) 

#define ITfInputProcessorProfiles_IsEnabledLanguageProfile(This,rclsid,langid,guidProfile,pfEnable)	\
    ( (This)->lpVtbl -> IsEnabledLanguageProfile(This,rclsid,langid,guidProfile,pfEnable) ) 

#define ITfInputProcessorProfiles_EnableLanguageProfileByDefault(This,rclsid,langid,guidProfile,fEnable)	\
    ( (This)->lpVtbl -> EnableLanguageProfileByDefault(This,rclsid,langid,guidProfile,fEnable) ) 

#define ITfInputProcessorProfiles_SubstituteKeyboardLayout(This,rclsid,langid,guidProfile,hKL)	\
    ( (This)->lpVtbl -> SubstituteKeyboardLayout(This,rclsid,langid,guidProfile,hKL) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ITfInputProcessorProfiles_INTERFACE_DEFINED__ */


#ifndef __ITfInputProcessorProfilesEx_INTERFACE_DEFINED__
#define __ITfInputProcessorProfilesEx_INTERFACE_DEFINED__

/* interface ITfInputProcessorProfilesEx */
/* [unique][uuid][object] */ 


EXTERN_C const IID IID_ITfInputProcessorProfilesEx;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("892f230f-fe00-4a41-a98e-fcd6de0d35ef")
    ITfInputProcessorProfilesEx : public ITfInputProcessorProfiles
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE SetLanguageProfileDisplayName( 
            /* [in] */ __RPC__in REFCLSID rclsid,
            /* [in] */ LANGID langid,
            /* [in] */ __RPC__in REFGUID guidProfile,
            /* [size_is][in] */ __RPC__in_ecount_full(cchFile) const WCHAR *pchFile,
            /* [in] */ ULONG cchFile,
            /* [in] */ ULONG uResId) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ITfInputProcessorProfilesExVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in ITfInputProcessorProfilesEx * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in ITfInputProcessorProfilesEx * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in ITfInputProcessorProfilesEx * This);
        
        DECLSPEC_XFGVIRT(ITfInputProcessorProfiles, Register)
        HRESULT ( STDMETHODCALLTYPE *Register )( 
            __RPC__in ITfInputProcessorProfilesEx * This,
            /* [in] */ __RPC__in REFCLSID rclsid);
        
        DECLSPEC_XFGVIRT(ITfInputProcessorProfiles, Unregister)
        HRESULT ( STDMETHODCALLTYPE *Unregister )( 
            __RPC__in ITfInputProcessorProfilesEx * This,
            /* [in] */ __RPC__in REFCLSID rclsid);
        
        DECLSPEC_XFGVIRT(ITfInputProcessorProfiles, AddLanguageProfile)
        HRESULT ( STDMETHODCALLTYPE *AddLanguageProfile )( 
            __RPC__in ITfInputProcessorProfilesEx * This,
            /* [in] */ __RPC__in REFCLSID rclsid,
            /* [in] */ LANGID langid,
            /* [in] */ __RPC__in REFGUID guidProfile,
            /* [size_is][in] */ __RPC__in_ecount_full(cchDesc) const WCHAR *pchDesc,
            /* [in] */ ULONG cchDesc,
            /* [size_is][in] */ __RPC__in_ecount_full(cchFile) const WCHAR *pchIconFile,
            /* [in] */ ULONG cchFile,
            /* [in] */ ULONG uIconIndex);
        
        DECLSPEC_XFGVIRT(ITfInputProcessorProfiles, RemoveLanguageProfile)
        HRESULT ( STDMETHODCALLTYPE *RemoveLanguageProfile )( 
            __RPC__in ITfInputProcessorProfilesEx * This,
            /* [in] */ __RPC__in REFCLSID rclsid,
            /* [in] */ LANGID langid,
            /* [in] */ __RPC__in REFGUID guidProfile);
        
        DECLSPEC_XFGVIRT(ITfInputProcessorProfiles, EnumInputProcessorInfo)
        HRESULT ( STDMETHODCALLTYPE *EnumInputProcessorInfo )( 
            __RPC__in ITfInputProcessorProfilesEx * This,
            /* [out] */ __RPC__deref_out_opt IEnumGUID **ppEnum);
        
        DECLSPEC_XFGVIRT(ITfInputProcessorProfiles, GetDefaultLanguageProfile)
        HRESULT ( STDMETHODCALLTYPE *GetDefaultLanguageProfile )( 
            __RPC__in ITfInputProcessorProfilesEx * This,
            /* [in] */ LANGID langid,
            /* [in] */ __RPC__in REFGUID catid,
            /* [out] */ __RPC__out CLSID *pclsid,
            /* [out] */ __RPC__out GUID *pguidProfile);
        
        DECLSPEC_XFGVIRT(ITfInputProcessorProfiles, SetDefaultLanguageProfile)
        HRESULT ( STDMETHODCALLTYPE *SetDefaultLanguageProfile )( 
            __RPC__in ITfInputProcessorProfilesEx * This,
            /* [in] */ LANGID langid,
            /* [in] */ __RPC__in REFCLSID rclsid,
            /* [in] */ __RPC__in REFGUID guidProfiles);
        
        DECLSPEC_XFGVIRT(ITfInputProcessorProfiles, ActivateLanguageProfile)
        HRESULT ( STDMETHODCALLTYPE *ActivateLanguageProfile )( 
            __RPC__in ITfInputProcessorProfilesEx * This,
            /* [in] */ __RPC__in REFCLSID rclsid,
            /* [in] */ LANGID langid,
            /* [in] */ __RPC__in REFGUID guidProfiles);
        
        DECLSPEC_XFGVIRT(ITfInputProcessorProfiles, GetActiveLanguageProfile)
        HRESULT ( STDMETHODCALLTYPE *GetActiveLanguageProfile )( 
            __RPC__in ITfInputProcessorProfilesEx * This,
            /* [in] */ __RPC__in REFCLSID rclsid,
            /* [out] */ __RPC__out LANGID *plangid,
            /* [out] */ __RPC__out GUID *pguidProfile);
        
        DECLSPEC_XFGVIRT(ITfInputProcessorProfiles, GetLanguageProfileDescription)
        HRESULT ( STDMETHODCALLTYPE *GetLanguageProfileDescription )( 
            __RPC__in ITfInputProcessorProfilesEx * This,
            /* [in] */ __RPC__in REFCLSID rclsid,
            /* [in] */ LANGID langid,
            /* [in] */ __RPC__in REFGUID guidProfile,
            /* [out] */ __RPC__deref_out_opt BSTR *pbstrProfile);
        
        DECLSPEC_XFGVIRT(ITfInputProcessorProfiles, GetCurrentLanguage)
        HRESULT ( STDMETHODCALLTYPE *GetCurrentLanguage )( 
            __RPC__in ITfInputProcessorProfilesEx * This,
            /* [out] */ __RPC__out LANGID *plangid);
        
        DECLSPEC_XFGVIRT(ITfInputProcessorProfiles, ChangeCurrentLanguage)
        HRESULT ( STDMETHODCALLTYPE *ChangeCurrentLanguage )( 
            __RPC__in ITfInputProcessorProfilesEx * This,
            /* [in] */ LANGID langid);
        
        DECLSPEC_XFGVIRT(ITfInputProcessorProfiles, GetLanguageList)
        HRESULT ( STDMETHODCALLTYPE *GetLanguageList )( 
            __RPC__in ITfInputProcessorProfilesEx * This,
            /* [size_is][size_is][out] */ __RPC__deref_out_ecount_full_opt(*pulCount) LANGID **ppLangId,
            /* [out] */ __RPC__out ULONG *pulCount);
        
        DECLSPEC_XFGVIRT(ITfInputProcessorProfiles, EnumLanguageProfiles)
        HRESULT ( STDMETHODCALLTYPE *EnumLanguageProfiles )( 
            __RPC__in ITfInputProcessorProfilesEx * This,
            /* [in] */ LANGID langid,
            /* [out] */ __RPC__deref_out_opt IEnumTfLanguageProfiles **ppEnum);
        
        DECLSPEC_XFGVIRT(ITfInputProcessorProfiles, EnableLanguageProfile)
        HRESULT ( STDMETHODCALLTYPE *EnableLanguageProfile )( 
            __RPC__in ITfInputProcessorProfilesEx * This,
            /* [in] */ __RPC__in REFCLSID rclsid,
            /* [in] */ LANGID langid,
            /* [in] */ __RPC__in REFGUID guidProfile,
            /* [in] */ BOOL fEnable);
        
        DECLSPEC_XFGVIRT(ITfInputProcessorProfiles, IsEnabledLanguageProfile)
        HRESULT ( STDMETHODCALLTYPE *IsEnabledLanguageProfile )( 
            __RPC__in ITfInputProcessorProfilesEx * This,
            /* [in] */ __RPC__in REFCLSID rclsid,
            /* [in] */ LANGID langid,
            /* [in] */ __RPC__in REFGUID guidProfile,
            /* [out] */ __RPC__out BOOL *pfEnable);
        
        DECLSPEC_XFGVIRT(ITfInputProcessorProfiles, EnableLanguageProfileByDefault)
        HRESULT ( STDMETHODCALLTYPE *EnableLanguageProfileByDefault )( 
            __RPC__in ITfInputProcessorProfilesEx * This,
            /* [in] */ __RPC__in REFCLSID rclsid,
            /* [in] */ LANGID langid,
            /* [in] */ __RPC__in REFGUID guidProfile,
            /* [in] */ BOOL fEnable);
        
        DECLSPEC_XFGVIRT(ITfInputProcessorProfiles, SubstituteKeyboardLayout)
        HRESULT ( STDMETHODCALLTYPE *SubstituteKeyboardLayout )( 
            __RPC__in ITfInputProcessorProfilesEx * This,
            /* [in] */ __RPC__in REFCLSID rclsid,
            /* [in] */ LANGID langid,
            /* [in] */ __RPC__in REFGUID guidProfile,
            /* [in] */ HKL hKL);
        
        DECLSPEC_XFGVIRT(ITfInputProcessorProfilesEx, SetLanguageProfileDisplayName)
        HRESULT ( STDMETHODCALLTYPE *SetLanguageProfileDisplayName )( 
            __RPC__in ITfInputProcessorProfilesEx * This,
            /* [in] */ __RPC__in REFCLSID rclsid,
            /* [in] */ LANGID langid,
            /* [in] */ __RPC__in REFGUID guidProfile,
            /* [size_is][in] */ __RPC__in_ecount_full(cchFile) const WCHAR *pchFile,
            /* [in] */ ULONG cchFile,
            /* [in] */ ULONG uResId);
        
        END_INTERFACE
    } ITfInputProcessorProfilesExVtbl;

    interface ITfInputProcessorProfilesEx
    {
        CONST_VTBL struct ITfInputProcessorProfilesExVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ITfInputProcessorProfilesEx_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ITfInputProcessorProfilesEx_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ITfInputProcessorProfilesEx_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ITfInputProcessorProfilesEx_Register(This,rclsid)	\
    ( (This)->lpVtbl -> Register(This,rclsid) ) 

#define ITfInputProcessorProfilesEx_Unregister(This,rclsid)	\
    ( (This)->lpVtbl -> Unregister(This,rclsid) ) 

#define ITfInputProcessorProfilesEx_AddLanguageProfile(This,rclsid,langid,guidProfile,pchDesc,cchDesc,pchIconFile,cchFile,uIconIndex)	\
    ( (This)->lpVtbl -> AddLanguageProfile(This,rclsid,langid,guidProfile,pchDesc,cchDesc,pchIconFile,cchFile,uIconIndex) ) 

#define ITfInputProcessorProfilesEx_RemoveLanguageProfile(This,rclsid,langid,guidProfile)	\
    ( (This)->lpVtbl -> RemoveLanguageProfile(This,rclsid,langid,guidProfile) ) 

#define ITfInputProcessorProfilesEx_EnumInputProcessorInfo(This,ppEnum)	\
    ( (This)->lpVtbl -> EnumInputProcessorInfo(This,ppEnum) ) 

#define ITfInputProcessorProfilesEx_GetDefaultLanguageProfile(This,langid,catid,pclsid,pguidProfile)	\
    ( (This)->lpVtbl -> GetDefaultLanguageProfile(This,langid,catid,pclsid,pguidProfile) ) 

#define ITfInputProcessorProfilesEx_SetDefaultLanguageProfile(This,langid,rclsid,guidProfiles)	\
    ( (This)->lpVtbl -> SetDefaultLanguageProfile(This,langid,rclsid,guidProfiles) ) 

#define ITfInputProcessorProfilesEx_ActivateLanguageProfile(This,rclsid,langid,guidProfiles)	\
    ( (This)->lpVtbl -> ActivateLanguageProfile(This,rclsid,langid,guidProfiles) ) 

#define ITfInputProcessorProfilesEx_GetActiveLanguageProfile(This,rclsid,plangid,pguidProfile)	\
    ( (This)->lpVtbl -> GetActiveLanguageProfile(This,rclsid,plangid,pguidProfile) ) 

#define ITfInputProcessorProfilesEx_GetLanguageProfileDescription(This,rclsid,langid,guidProfile,pbstrProfile)	\
    ( (This)->lpVtbl -> GetLanguageProfileDescription(This,rclsid,langid,guidProfile,pbstrProfile) ) 

#define ITfInputProcessorProfilesEx_GetCurrentLanguage(This,plangid)	\
    ( (This)->lpVtbl -> GetCurrentLanguage(This,plangid) ) 

#define ITfInputProcessorProfilesEx_ChangeCurrentLanguage(This,langid)	\
    ( (This)->lpVtbl -> ChangeCurrentLanguage(This,langid) ) 

#define ITfInputProcessorProfilesEx_GetLanguageList(This,ppLangId,pulCount)	\
    ( (This)->lpVtbl -> GetLanguageList(This,ppLangId,pulCount) ) 

#define ITfInputProcessorProfilesEx_EnumLanguageProfiles(This,langid,ppEnum)	\
    ( (This)->lpVtbl -> EnumLanguageProfiles(This,langid,ppEnum) ) 

#define ITfInputProcessorProfilesEx_EnableLanguageProfile(This,rclsid,langid,guidProfile,fEnable)	\
    ( (This)->lpVtbl -> EnableLanguageProfile(This,rclsid,langid,guidProfile,fEnable) ) 

#define ITfInputProcessorProfilesEx_IsEnabledLanguageProfile(This,rclsid,langid,guidProfile,pfEnable)	\
    ( (This)->lpVtbl -> IsEnabledLanguageProfile(This,rclsid,langid,guidProfile,pfEnable) ) 

#define ITfInputProcessorProfilesEx_EnableLanguageProfileByDefault(This,rclsid,langid,guidProfile,fEnable)	\
    ( (This)->lpVtbl -> EnableLanguageProfileByDefault(This,rclsid,langid,guidProfile,fEnable) ) 

#define ITfInputProcessorProfilesEx_SubstituteKeyboardLayout(This,rclsid,langid,guidProfile,hKL)	\
    ( (This)->lpVtbl -> SubstituteKeyboardLayout(This,rclsid,langid,guidProfile,hKL) ) 


#define ITfInputProcessorProfilesEx_SetLanguageProfileDisplayName(This,rclsid,langid,guidProfile,pchFile,cchFile,uResId)	\
    ( (This)->lpVtbl -> SetLanguageProfileDisplayName(This,rclsid,langid,guidProfile,pchFile,cchFile,uResId) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ITfInputProcessorProfilesEx_INTERFACE_DEFINED__ */


#ifndef __ITfInputProcessorProfileSubstituteLayout_INTERFACE_DEFINED__
#define __ITfInputProcessorProfileSubstituteLayout_INTERFACE_DEFINED__

/* interface ITfInputProcessorProfileSubstituteLayout */
/* [unique][uuid][object] */ 


EXTERN_C const IID IID_ITfInputProcessorProfileSubstituteLayout;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("4fd67194-1002-4513-bff2-c0ddf6258552")
    ITfInputProcessorProfileSubstituteLayout : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetSubstituteKeyboardLayout( 
            /* [in] */ __RPC__in REFCLSID rclsid,
            /* [in] */ LANGID langid,
            /* [in] */ __RPC__in REFGUID guidProfile,
            /* [out] */ __RPC__out HKL *phKL) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ITfInputProcessorProfileSubstituteLayoutVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in ITfInputProcessorProfileSubstituteLayout * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in ITfInputProcessorProfileSubstituteLayout * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in ITfInputProcessorProfileSubstituteLayout * This);
        
        DECLSPEC_XFGVIRT(ITfInputProcessorProfileSubstituteLayout, GetSubstituteKeyboardLayout)
        HRESULT ( STDMETHODCALLTYPE *GetSubstituteKeyboardLayout )( 
            __RPC__in ITfInputProcessorProfileSubstituteLayout * This,
            /* [in] */ __RPC__in REFCLSID rclsid,
            /* [in] */ LANGID langid,
            /* [in] */ __RPC__in REFGUID guidProfile,
            /* [out] */ __RPC__out HKL *phKL);
        
        END_INTERFACE
    } ITfInputProcessorProfileSubstituteLayoutVtbl;

    interface ITfInputProcessorProfileSubstituteLayout
    {
        CONST_VTBL struct ITfInputProcessorProfileSubstituteLayoutVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ITfInputProcessorProfileSubstituteLayout_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ITfInputProcessorProfileSubstituteLayout_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ITfInputProcessorProfileSubstituteLayout_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ITfInputProcessorProfileSubstituteLayout_GetSubstituteKeyboardLayout(This,rclsid,langid,guidProfile,phKL)	\
    ( (This)->lpVtbl -> GetSubstituteKeyboardLayout(This,rclsid,langid,guidProfile,phKL) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ITfInputProcessorProfileSubstituteLayout_INTERFACE_DEFINED__ */


#ifndef __ITfActiveLanguageProfileNotifySink_INTERFACE_DEFINED__
#define __ITfActiveLanguageProfileNotifySink_INTERFACE_DEFINED__

/* interface ITfActiveLanguageProfileNotifySink */
/* [unique][uuid][object] */ 


EXTERN_C const IID IID_ITfActiveLanguageProfileNotifySink;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("b246cb75-a93e-4652-bf8c-b3fe0cfd7e57")
    ITfActiveLanguageProfileNotifySink : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE OnActivated( 
            /* [in] */ __RPC__in REFCLSID clsid,
            /* [in] */ __RPC__in REFGUID guidProfile,
            /* [in] */ BOOL fActivated) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ITfActiveLanguageProfileNotifySinkVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in ITfActiveLanguageProfileNotifySink * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in ITfActiveLanguageProfileNotifySink * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in ITfActiveLanguageProfileNotifySink * This);
        
        DECLSPEC_XFGVIRT(ITfActiveLanguageProfileNotifySink, OnActivated)
        HRESULT ( STDMETHODCALLTYPE *OnActivated )( 
            __RPC__in ITfActiveLanguageProfileNotifySink * This,
            /* [in] */ __RPC__in REFCLSID clsid,
            /* [in] */ __RPC__in REFGUID guidProfile,
            /* [in] */ BOOL fActivated);
        
        END_INTERFACE
    } ITfActiveLanguageProfileNotifySinkVtbl;

    interface ITfActiveLanguageProfileNotifySink
    {
        CONST_VTBL struct ITfActiveLanguageProfileNotifySinkVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ITfActiveLanguageProfileNotifySink_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ITfActiveLanguageProfileNotifySink_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ITfActiveLanguageProfileNotifySink_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ITfActiveLanguageProfileNotifySink_OnActivated(This,clsid,guidProfile,fActivated)	\
    ( (This)->lpVtbl -> OnActivated(This,clsid,guidProfile,fActivated) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ITfActiveLanguageProfileNotifySink_INTERFACE_DEFINED__ */


#ifndef __IEnumTfLanguageProfiles_INTERFACE_DEFINED__
#define __IEnumTfLanguageProfiles_INTERFACE_DEFINED__

/* interface IEnumTfLanguageProfiles */
/* [unique][uuid][object] */ 


EXTERN_C const IID IID_IEnumTfLanguageProfiles;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("3d61bf11-ac5f-42c8-a4cb-931bcc28c744")
    IEnumTfLanguageProfiles : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE Clone( 
            /* [out] */ __RPC__deref_out_opt IEnumTfLanguageProfiles **ppEnum) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Next( 
            /* [in] */ ULONG ulCount,
            /* [length_is][size_is][out] */ __RPC__out_ecount_part(ulCount, *pcFetch) TF_LANGUAGEPROFILE *pProfile,
            /* [out] */ __RPC__out ULONG *pcFetch) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Reset( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Skip( 
            /* [in] */ ULONG ulCount) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IEnumTfLanguageProfilesVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IEnumTfLanguageProfiles * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IEnumTfLanguageProfiles * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IEnumTfLanguageProfiles * This);
        
        DECLSPEC_XFGVIRT(IEnumTfLanguageProfiles, Clone)
        HRESULT ( STDMETHODCALLTYPE *Clone )( 
            __RPC__in IEnumTfLanguageProfiles * This,
            /* [out] */ __RPC__deref_out_opt IEnumTfLanguageProfiles **ppEnum);
        
        DECLSPEC_XFGVIRT(IEnumTfLanguageProfiles, Next)
        HRESULT ( STDMETHODCALLTYPE *Next )( 
            __RPC__in IEnumTfLanguageProfiles * This,
            /* [in] */ ULONG ulCount,
            /* [length_is][size_is][out] */ __RPC__out_ecount_part(ulCount, *pcFetch) TF_LANGUAGEPROFILE *pProfile,
            /* [out] */ __RPC__out ULONG *pcFetch);
        
        DECLSPEC_XFGVIRT(IEnumTfLanguageProfiles, Reset)
        HRESULT ( STDMETHODCALLTYPE *Reset )( 
            __RPC__in IEnumTfLanguageProfiles * This);
        
        DECLSPEC_XFGVIRT(IEnumTfLanguageProfiles, Skip)
        HRESULT ( STDMETHODCALLTYPE *Skip )( 
            __RPC__in IEnumTfLanguageProfiles * This,
            /* [in] */ ULONG ulCount);
        
        END_INTERFACE
    } IEnumTfLanguageProfilesVtbl;

    interface IEnumTfLanguageProfiles
    {
        CONST_VTBL struct IEnumTfLanguageProfilesVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IEnumTfLanguageProfiles_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IEnumTfLanguageProfiles_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IEnumTfLanguageProfiles_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IEnumTfLanguageProfiles_Clone(This,ppEnum)	\
    ( (This)->lpVtbl -> Clone(This,ppEnum) ) 

#define IEnumTfLanguageProfiles_Next(This,ulCount,pProfile,pcFetch)	\
    ( (This)->lpVtbl -> Next(This,ulCount,pProfile,pcFetch) ) 

#define IEnumTfLanguageProfiles_Reset(This)	\
    ( (This)->lpVtbl -> Reset(This) ) 

#define IEnumTfLanguageProfiles_Skip(This,ulCount)	\
    ( (This)->lpVtbl -> Skip(This,ulCount) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IEnumTfLanguageProfiles_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_msctf_0000_0057 */
/* [local] */ 

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion
#pragma region Application Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP)


extern RPC_IF_HANDLE __MIDL_itf_msctf_0000_0057_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_msctf_0000_0057_v0_0_s_ifspec;

#ifndef __ITfLanguageProfileNotifySink_INTERFACE_DEFINED__
#define __ITfLanguageProfileNotifySink_INTERFACE_DEFINED__

/* interface ITfLanguageProfileNotifySink */
/* [unique][uuid][object] */ 


EXTERN_C const IID IID_ITfLanguageProfileNotifySink;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("43c9fe15-f494-4c17-9de2-b8a4ac350aa8")
    ITfLanguageProfileNotifySink : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE OnLanguageChange( 
            /* [in] */ LANGID langid,
            /* [out] */ __RPC__out BOOL *pfAccept) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE OnLanguageChanged( void) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ITfLanguageProfileNotifySinkVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in ITfLanguageProfileNotifySink * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in ITfLanguageProfileNotifySink * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in ITfLanguageProfileNotifySink * This);
        
        DECLSPEC_XFGVIRT(ITfLanguageProfileNotifySink, OnLanguageChange)
        HRESULT ( STDMETHODCALLTYPE *OnLanguageChange )( 
            __RPC__in ITfLanguageProfileNotifySink * This,
            /* [in] */ LANGID langid,
            /* [out] */ __RPC__out BOOL *pfAccept);
        
        DECLSPEC_XFGVIRT(ITfLanguageProfileNotifySink, OnLanguageChanged)
        HRESULT ( STDMETHODCALLTYPE *OnLanguageChanged )( 
            __RPC__in ITfLanguageProfileNotifySink * This);
        
        END_INTERFACE
    } ITfLanguageProfileNotifySinkVtbl;

    interface ITfLanguageProfileNotifySink
    {
        CONST_VTBL struct ITfLanguageProfileNotifySinkVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ITfLanguageProfileNotifySink_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ITfLanguageProfileNotifySink_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ITfLanguageProfileNotifySink_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ITfLanguageProfileNotifySink_OnLanguageChange(This,langid,pfAccept)	\
    ( (This)->lpVtbl -> OnLanguageChange(This,langid,pfAccept) ) 

#define ITfLanguageProfileNotifySink_OnLanguageChanged(This)	\
    ( (This)->lpVtbl -> OnLanguageChanged(This) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ITfLanguageProfileNotifySink_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_msctf_0000_0058 */
/* [local] */ 

typedef /* [uuid] */  DECLSPEC_UUID("44d2825a-10e5-43b2-877f-6cb2f43b7e7e") struct TF_INPUTPROCESSORPROFILE
    {
    DWORD dwProfileType;
    LANGID langid;
    CLSID clsid;
    GUID guidProfile;
    GUID catid;
    HKL hklSubstitute;
    DWORD dwCaps;
    HKL hkl;
    DWORD dwFlags;
    } 	TF_INPUTPROCESSORPROFILE;



extern RPC_IF_HANDLE __MIDL_itf_msctf_0000_0058_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_msctf_0000_0058_v0_0_s_ifspec;

#ifndef __ITfInputProcessorProfileMgr_INTERFACE_DEFINED__
#define __ITfInputProcessorProfileMgr_INTERFACE_DEFINED__

/* interface ITfInputProcessorProfileMgr */
/* [unique][uuid][object] */ 


EXTERN_C const IID IID_ITfInputProcessorProfileMgr;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("71c6e74c-0f28-11d8-a82a-00065b84435c")
    ITfInputProcessorProfileMgr : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE ActivateProfile( 
            /* [in] */ DWORD dwProfileType,
            /* [in] */ LANGID langid,
            /* [in] */ __RPC__in REFCLSID clsid,
            /* [in] */ __RPC__in REFGUID guidProfile,
            /* [in] */ HKL hkl,
            /* [in] */ DWORD dwFlags) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE DeactivateProfile( 
            /* [in] */ DWORD dwProfileType,
            /* [in] */ LANGID langid,
            /* [in] */ __RPC__in REFCLSID clsid,
            /* [in] */ __RPC__in REFGUID guidProfile,
            /* [in] */ HKL hkl,
            /* [in] */ DWORD dwFlags) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetProfile( 
            /* [in] */ DWORD dwProfileType,
            /* [in] */ LANGID langid,
            /* [in] */ __RPC__in REFCLSID clsid,
            /* [in] */ __RPC__in REFGUID guidProfile,
            /* [in] */ HKL hkl,
            /* [out] */ __RPC__out TF_INPUTPROCESSORPROFILE *pProfile) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE EnumProfiles( 
            /* [in] */ LANGID langid,
            /* [out] */ __RPC__deref_out_opt IEnumTfInputProcessorProfiles **ppEnum) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE ReleaseInputProcessor( 
            /* [in] */ __RPC__in REFCLSID rclsid,
            /* [in] */ DWORD dwFlags) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE RegisterProfile( 
            /* [in] */ __RPC__in REFCLSID rclsid,
            /* [in] */ LANGID langid,
            /* [in] */ __RPC__in REFGUID guidProfile,
            /* [size_is][in] */ __RPC__in_ecount_full(cchDesc) const WCHAR *pchDesc,
            /* [in] */ ULONG cchDesc,
            /* [size_is][in] */ __RPC__in_ecount_full(cchFile) const WCHAR *pchIconFile,
            /* [in] */ ULONG cchFile,
            /* [in] */ ULONG uIconIndex,
            /* [in] */ HKL hklsubstitute,
            /* [in] */ DWORD dwPreferredLayout,
            /* [in] */ BOOL bEnabledByDefault,
            /* [in] */ DWORD dwFlags) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE UnregisterProfile( 
            /* [in] */ __RPC__in REFCLSID rclsid,
            /* [in] */ LANGID langid,
            /* [in] */ __RPC__in REFGUID guidProfile,
            /* [in] */ DWORD dwFlags) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetActiveProfile( 
            /* [in] */ __RPC__in REFGUID catid,
            /* [out] */ __RPC__out TF_INPUTPROCESSORPROFILE *pProfile) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ITfInputProcessorProfileMgrVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in ITfInputProcessorProfileMgr * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in ITfInputProcessorProfileMgr * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in ITfInputProcessorProfileMgr * This);
        
        DECLSPEC_XFGVIRT(ITfInputProcessorProfileMgr, ActivateProfile)
        HRESULT ( STDMETHODCALLTYPE *ActivateProfile )( 
            __RPC__in ITfInputProcessorProfileMgr * This,
            /* [in] */ DWORD dwProfileType,
            /* [in] */ LANGID langid,
            /* [in] */ __RPC__in REFCLSID clsid,
            /* [in] */ __RPC__in REFGUID guidProfile,
            /* [in] */ HKL hkl,
            /* [in] */ DWORD dwFlags);
        
        DECLSPEC_XFGVIRT(ITfInputProcessorProfileMgr, DeactivateProfile)
        HRESULT ( STDMETHODCALLTYPE *DeactivateProfile )( 
            __RPC__in ITfInputProcessorProfileMgr * This,
            /* [in] */ DWORD dwProfileType,
            /* [in] */ LANGID langid,
            /* [in] */ __RPC__in REFCLSID clsid,
            /* [in] */ __RPC__in REFGUID guidProfile,
            /* [in] */ HKL hkl,
            /* [in] */ DWORD dwFlags);
        
        DECLSPEC_XFGVIRT(ITfInputProcessorProfileMgr, GetProfile)
        HRESULT ( STDMETHODCALLTYPE *GetProfile )( 
            __RPC__in ITfInputProcessorProfileMgr * This,
            /* [in] */ DWORD dwProfileType,
            /* [in] */ LANGID langid,
            /* [in] */ __RPC__in REFCLSID clsid,
            /* [in] */ __RPC__in REFGUID guidProfile,
            /* [in] */ HKL hkl,
            /* [out] */ __RPC__out TF_INPUTPROCESSORPROFILE *pProfile);
        
        DECLSPEC_XFGVIRT(ITfInputProcessorProfileMgr, EnumProfiles)
        HRESULT ( STDMETHODCALLTYPE *EnumProfiles )( 
            __RPC__in ITfInputProcessorProfileMgr * This,
            /* [in] */ LANGID langid,
            /* [out] */ __RPC__deref_out_opt IEnumTfInputProcessorProfiles **ppEnum);
        
        DECLSPEC_XFGVIRT(ITfInputProcessorProfileMgr, ReleaseInputProcessor)
        HRESULT ( STDMETHODCALLTYPE *ReleaseInputProcessor )( 
            __RPC__in ITfInputProcessorProfileMgr * This,
            /* [in] */ __RPC__in REFCLSID rclsid,
            /* [in] */ DWORD dwFlags);
        
        DECLSPEC_XFGVIRT(ITfInputProcessorProfileMgr, RegisterProfile)
        HRESULT ( STDMETHODCALLTYPE *RegisterProfile )( 
            __RPC__in ITfInputProcessorProfileMgr * This,
            /* [in] */ __RPC__in REFCLSID rclsid,
            /* [in] */ LANGID langid,
            /* [in] */ __RPC__in REFGUID guidProfile,
            /* [size_is][in] */ __RPC__in_ecount_full(cchDesc) const WCHAR *pchDesc,
            /* [in] */ ULONG cchDesc,
            /* [size_is][in] */ __RPC__in_ecount_full(cchFile) const WCHAR *pchIconFile,
            /* [in] */ ULONG cchFile,
            /* [in] */ ULONG uIconIndex,
            /* [in] */ HKL hklsubstitute,
            /* [in] */ DWORD dwPreferredLayout,
            /* [in] */ BOOL bEnabledByDefault,
            /* [in] */ DWORD dwFlags);
        
        DECLSPEC_XFGVIRT(ITfInputProcessorProfileMgr, UnregisterProfile)
        HRESULT ( STDMETHODCALLTYPE *UnregisterProfile )( 
            __RPC__in ITfInputProcessorProfileMgr * This,
            /* [in] */ __RPC__in REFCLSID rclsid,
            /* [in] */ LANGID langid,
            /* [in] */ __RPC__in REFGUID guidProfile,
            /* [in] */ DWORD dwFlags);
        
        DECLSPEC_XFGVIRT(ITfInputProcessorProfileMgr, GetActiveProfile)
        HRESULT ( STDMETHODCALLTYPE *GetActiveProfile )( 
            __RPC__in ITfInputProcessorProfileMgr * This,
            /* [in] */ __RPC__in REFGUID catid,
            /* [out] */ __RPC__out TF_INPUTPROCESSORPROFILE *pProfile);
        
        END_INTERFACE
    } ITfInputProcessorProfileMgrVtbl;

    interface ITfInputProcessorProfileMgr
    {
        CONST_VTBL struct ITfInputProcessorProfileMgrVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ITfInputProcessorProfileMgr_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ITfInputProcessorProfileMgr_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ITfInputProcessorProfileMgr_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ITfInputProcessorProfileMgr_ActivateProfile(This,dwProfileType,langid,clsid,guidProfile,hkl,dwFlags)	\
    ( (This)->lpVtbl -> ActivateProfile(This,dwProfileType,langid,clsid,guidProfile,hkl,dwFlags) ) 

#define ITfInputProcessorProfileMgr_DeactivateProfile(This,dwProfileType,langid,clsid,guidProfile,hkl,dwFlags)	\
    ( (This)->lpVtbl -> DeactivateProfile(This,dwProfileType,langid,clsid,guidProfile,hkl,dwFlags) ) 

#define ITfInputProcessorProfileMgr_GetProfile(This,dwProfileType,langid,clsid,guidProfile,hkl,pProfile)	\
    ( (This)->lpVtbl -> GetProfile(This,dwProfileType,langid,clsid,guidProfile,hkl,pProfile) ) 

#define ITfInputProcessorProfileMgr_EnumProfiles(This,langid,ppEnum)	\
    ( (This)->lpVtbl -> EnumProfiles(This,langid,ppEnum) ) 

#define ITfInputProcessorProfileMgr_ReleaseInputProcessor(This,rclsid,dwFlags)	\
    ( (This)->lpVtbl -> ReleaseInputProcessor(This,rclsid,dwFlags) ) 

#define ITfInputProcessorProfileMgr_RegisterProfile(This,rclsid,langid,guidProfile,pchDesc,cchDesc,pchIconFile,cchFile,uIconIndex,hklsubstitute,dwPreferredLayout,bEnabledByDefault,dwFlags)	\
    ( (This)->lpVtbl -> RegisterProfile(This,rclsid,langid,guidProfile,pchDesc,cchDesc,pchIconFile,cchFile,uIconIndex,hklsubstitute,dwPreferredLayout,bEnabledByDefault,dwFlags) ) 

#define ITfInputProcessorProfileMgr_UnregisterProfile(This,rclsid,langid,guidProfile,dwFlags)	\
    ( (This)->lpVtbl -> UnregisterProfile(This,rclsid,langid,guidProfile,dwFlags) ) 

#define ITfInputProcessorProfileMgr_GetActiveProfile(This,catid,pProfile)	\
    ( (This)->lpVtbl -> GetActiveProfile(This,catid,pProfile) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ITfInputProcessorProfileMgr_INTERFACE_DEFINED__ */


#ifndef __IEnumTfInputProcessorProfiles_INTERFACE_DEFINED__
#define __IEnumTfInputProcessorProfiles_INTERFACE_DEFINED__

/* interface IEnumTfInputProcessorProfiles */
/* [unique][uuid][object] */ 


EXTERN_C const IID IID_IEnumTfInputProcessorProfiles;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("71c6e74d-0f28-11d8-a82a-00065b84435c")
    IEnumTfInputProcessorProfiles : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE Clone( 
            /* [out] */ __RPC__deref_out_opt IEnumTfInputProcessorProfiles **ppEnum) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Next( 
            /* [in] */ ULONG ulCount,
            /* [length_is][size_is][out] */ __RPC__out_ecount_part(ulCount, *pcFetch) TF_INPUTPROCESSORPROFILE *pProfile,
            /* [out] */ __RPC__out ULONG *pcFetch) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Reset( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Skip( 
            /* [in] */ ULONG ulCount) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IEnumTfInputProcessorProfilesVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IEnumTfInputProcessorProfiles * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IEnumTfInputProcessorProfiles * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IEnumTfInputProcessorProfiles * This);
        
        DECLSPEC_XFGVIRT(IEnumTfInputProcessorProfiles, Clone)
        HRESULT ( STDMETHODCALLTYPE *Clone )( 
            __RPC__in IEnumTfInputProcessorProfiles * This,
            /* [out] */ __RPC__deref_out_opt IEnumTfInputProcessorProfiles **ppEnum);
        
        DECLSPEC_XFGVIRT(IEnumTfInputProcessorProfiles, Next)
        HRESULT ( STDMETHODCALLTYPE *Next )( 
            __RPC__in IEnumTfInputProcessorProfiles * This,
            /* [in] */ ULONG ulCount,
            /* [length_is][size_is][out] */ __RPC__out_ecount_part(ulCount, *pcFetch) TF_INPUTPROCESSORPROFILE *pProfile,
            /* [out] */ __RPC__out ULONG *pcFetch);
        
        DECLSPEC_XFGVIRT(IEnumTfInputProcessorProfiles, Reset)
        HRESULT ( STDMETHODCALLTYPE *Reset )( 
            __RPC__in IEnumTfInputProcessorProfiles * This);
        
        DECLSPEC_XFGVIRT(IEnumTfInputProcessorProfiles, Skip)
        HRESULT ( STDMETHODCALLTYPE *Skip )( 
            __RPC__in IEnumTfInputProcessorProfiles * This,
            /* [in] */ ULONG ulCount);
        
        END_INTERFACE
    } IEnumTfInputProcessorProfilesVtbl;

    interface IEnumTfInputProcessorProfiles
    {
        CONST_VTBL struct IEnumTfInputProcessorProfilesVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IEnumTfInputProcessorProfiles_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IEnumTfInputProcessorProfiles_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IEnumTfInputProcessorProfiles_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IEnumTfInputProcessorProfiles_Clone(This,ppEnum)	\
    ( (This)->lpVtbl -> Clone(This,ppEnum) ) 

#define IEnumTfInputProcessorProfiles_Next(This,ulCount,pProfile,pcFetch)	\
    ( (This)->lpVtbl -> Next(This,ulCount,pProfile,pcFetch) ) 

#define IEnumTfInputProcessorProfiles_Reset(This)	\
    ( (This)->lpVtbl -> Reset(This) ) 

#define IEnumTfInputProcessorProfiles_Skip(This,ulCount)	\
    ( (This)->lpVtbl -> Skip(This,ulCount) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IEnumTfInputProcessorProfiles_INTERFACE_DEFINED__ */


#ifndef __ITfInputProcessorProfileActivationSink_INTERFACE_DEFINED__
#define __ITfInputProcessorProfileActivationSink_INTERFACE_DEFINED__

/* interface ITfInputProcessorProfileActivationSink */
/* [unique][uuid][object] */ 


EXTERN_C const IID IID_ITfInputProcessorProfileActivationSink;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("71c6e74e-0f28-11d8-a82a-00065b84435c")
    ITfInputProcessorProfileActivationSink : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE OnActivated( 
            /* [in] */ DWORD dwProfileType,
            /* [in] */ LANGID langid,
            /* [in] */ __RPC__in REFCLSID clsid,
            /* [in] */ __RPC__in REFGUID catid,
            /* [in] */ __RPC__in REFGUID guidProfile,
            /* [in] */ HKL hkl,
            /* [in] */ DWORD dwFlags) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ITfInputProcessorProfileActivationSinkVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in ITfInputProcessorProfileActivationSink * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in ITfInputProcessorProfileActivationSink * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in ITfInputProcessorProfileActivationSink * This);
        
        DECLSPEC_XFGVIRT(ITfInputProcessorProfileActivationSink, OnActivated)
        HRESULT ( STDMETHODCALLTYPE *OnActivated )( 
            __RPC__in ITfInputProcessorProfileActivationSink * This,
            /* [in] */ DWORD dwProfileType,
            /* [in] */ LANGID langid,
            /* [in] */ __RPC__in REFCLSID clsid,
            /* [in] */ __RPC__in REFGUID catid,
            /* [in] */ __RPC__in REFGUID guidProfile,
            /* [in] */ HKL hkl,
            /* [in] */ DWORD dwFlags);
        
        END_INTERFACE
    } ITfInputProcessorProfileActivationSinkVtbl;

    interface ITfInputProcessorProfileActivationSink
    {
        CONST_VTBL struct ITfInputProcessorProfileActivationSinkVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ITfInputProcessorProfileActivationSink_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ITfInputProcessorProfileActivationSink_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ITfInputProcessorProfileActivationSink_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ITfInputProcessorProfileActivationSink_OnActivated(This,dwProfileType,langid,clsid,catid,guidProfile,hkl,dwFlags)	\
    ( (This)->lpVtbl -> OnActivated(This,dwProfileType,langid,clsid,catid,guidProfile,hkl,dwFlags) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ITfInputProcessorProfileActivationSink_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_msctf_0000_0061 */
/* [local] */ 

typedef /* [uuid] */  DECLSPEC_UUID("77c12f95-b783-450d-879f-1cd2362c6521") struct TF_PRESERVEDKEY
    {
    UINT uVKey;
    UINT uModifiers;
    } 	TF_PRESERVEDKEY;



extern RPC_IF_HANDLE __MIDL_itf_msctf_0000_0061_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_msctf_0000_0061_v0_0_s_ifspec;

#ifndef __ITfKeystrokeMgr_INTERFACE_DEFINED__
#define __ITfKeystrokeMgr_INTERFACE_DEFINED__

/* interface ITfKeystrokeMgr */
/* [unique][uuid][local][object] */ 


EXTERN_C const IID IID_ITfKeystrokeMgr;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("aa80e7f0-2021-11d2-93e0-0060b067b86e")
    ITfKeystrokeMgr : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE AdviseKeyEventSink( 
            /* [in] */ TfClientId tid,
            /* [in] */ ITfKeyEventSink *pSink,
            /* [in] */ BOOL fForeground) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE UnadviseKeyEventSink( 
            /* [in] */ TfClientId tid) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetForeground( 
            /* [out] */ CLSID *pclsid) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE TestKeyDown( 
            /* [in] */ WPARAM wParam,
            /* [in] */ LPARAM lParam,
            /* [out] */ BOOL *pfEaten) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE TestKeyUp( 
            /* [in] */ WPARAM wParam,
            /* [in] */ LPARAM lParam,
            /* [out] */ BOOL *pfEaten) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE KeyDown( 
            /* [in] */ WPARAM wParam,
            /* [in] */ LPARAM lParam,
            /* [out] */ BOOL *pfEaten) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE KeyUp( 
            /* [in] */ WPARAM wParam,
            /* [in] */ LPARAM lParam,
            /* [out] */ BOOL *pfEaten) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetPreservedKey( 
            /* [in] */ ITfContext *pic,
            /* [in] */ const TF_PRESERVEDKEY *pprekey,
            /* [out] */ GUID *pguid) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE IsPreservedKey( 
            /* [in] */ REFGUID rguid,
            /* [in] */ const TF_PRESERVEDKEY *pprekey,
            /* [out] */ BOOL *pfRegistered) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE PreserveKey( 
            /* [in] */ TfClientId tid,
            /* [in] */ REFGUID rguid,
            /* [in] */ const TF_PRESERVEDKEY *prekey,
            /* [size_is][in] */ const WCHAR *pchDesc,
            /* [in] */ ULONG cchDesc) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE UnpreserveKey( 
            /* [in] */ REFGUID rguid,
            /* [in] */ const TF_PRESERVEDKEY *pprekey) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetPreservedKeyDescription( 
            /* [in] */ REFGUID rguid,
            /* [size_is][in] */ const WCHAR *pchDesc,
            /* [in] */ ULONG cchDesc) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetPreservedKeyDescription( 
            /* [in] */ REFGUID rguid,
            /* [out] */ BSTR *pbstrDesc) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SimulatePreservedKey( 
            /* [in] */ ITfContext *pic,
            /* [in] */ REFGUID rguid,
            /* [out] */ BOOL *pfEaten) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ITfKeystrokeMgrVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            ITfKeystrokeMgr * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            ITfKeystrokeMgr * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            ITfKeystrokeMgr * This);
        
        DECLSPEC_XFGVIRT(ITfKeystrokeMgr, AdviseKeyEventSink)
        HRESULT ( STDMETHODCALLTYPE *AdviseKeyEventSink )( 
            ITfKeystrokeMgr * This,
            /* [in] */ TfClientId tid,
            /* [in] */ ITfKeyEventSink *pSink,
            /* [in] */ BOOL fForeground);
        
        DECLSPEC_XFGVIRT(ITfKeystrokeMgr, UnadviseKeyEventSink)
        HRESULT ( STDMETHODCALLTYPE *UnadviseKeyEventSink )( 
            ITfKeystrokeMgr * This,
            /* [in] */ TfClientId tid);
        
        DECLSPEC_XFGVIRT(ITfKeystrokeMgr, GetForeground)
        HRESULT ( STDMETHODCALLTYPE *GetForeground )( 
            ITfKeystrokeMgr * This,
            /* [out] */ CLSID *pclsid);
        
        DECLSPEC_XFGVIRT(ITfKeystrokeMgr, TestKeyDown)
        HRESULT ( STDMETHODCALLTYPE *TestKeyDown )( 
            ITfKeystrokeMgr * This,
            /* [in] */ WPARAM wParam,
            /* [in] */ LPARAM lParam,
            /* [out] */ BOOL *pfEaten);
        
        DECLSPEC_XFGVIRT(ITfKeystrokeMgr, TestKeyUp)
        HRESULT ( STDMETHODCALLTYPE *TestKeyUp )( 
            ITfKeystrokeMgr * This,
            /* [in] */ WPARAM wParam,
            /* [in] */ LPARAM lParam,
            /* [out] */ BOOL *pfEaten);
        
        DECLSPEC_XFGVIRT(ITfKeystrokeMgr, KeyDown)
        HRESULT ( STDMETHODCALLTYPE *KeyDown )( 
            ITfKeystrokeMgr * This,
            /* [in] */ WPARAM wParam,
            /* [in] */ LPARAM lParam,
            /* [out] */ BOOL *pfEaten);
        
        DECLSPEC_XFGVIRT(ITfKeystrokeMgr, KeyUp)
        HRESULT ( STDMETHODCALLTYPE *KeyUp )( 
            ITfKeystrokeMgr * This,
            /* [in] */ WPARAM wParam,
            /* [in] */ LPARAM lParam,
            /* [out] */ BOOL *pfEaten);
        
        DECLSPEC_XFGVIRT(ITfKeystrokeMgr, GetPreservedKey)
        HRESULT ( STDMETHODCALLTYPE *GetPreservedKey )( 
            ITfKeystrokeMgr * This,
            /* [in] */ ITfContext *pic,
            /* [in] */ const TF_PRESERVEDKEY *pprekey,
            /* [out] */ GUID *pguid);
        
        DECLSPEC_XFGVIRT(ITfKeystrokeMgr, IsPreservedKey)
        HRESULT ( STDMETHODCALLTYPE *IsPreservedKey )( 
            ITfKeystrokeMgr * This,
            /* [in] */ REFGUID rguid,
            /* [in] */ const TF_PRESERVEDKEY *pprekey,
            /* [out] */ BOOL *pfRegistered);
        
        DECLSPEC_XFGVIRT(ITfKeystrokeMgr, PreserveKey)
        HRESULT ( STDMETHODCALLTYPE *PreserveKey )( 
            ITfKeystrokeMgr * This,
            /* [in] */ TfClientId tid,
            /* [in] */ REFGUID rguid,
            /* [in] */ const TF_PRESERVEDKEY *prekey,
            /* [size_is][in] */ const WCHAR *pchDesc,
            /* [in] */ ULONG cchDesc);
        
        DECLSPEC_XFGVIRT(ITfKeystrokeMgr, UnpreserveKey)
        HRESULT ( STDMETHODCALLTYPE *UnpreserveKey )( 
            ITfKeystrokeMgr * This,
            /* [in] */ REFGUID rguid,
            /* [in] */ const TF_PRESERVEDKEY *pprekey);
        
        DECLSPEC_XFGVIRT(ITfKeystrokeMgr, SetPreservedKeyDescription)
        HRESULT ( STDMETHODCALLTYPE *SetPreservedKeyDescription )( 
            ITfKeystrokeMgr * This,
            /* [in] */ REFGUID rguid,
            /* [size_is][in] */ const WCHAR *pchDesc,
            /* [in] */ ULONG cchDesc);
        
        DECLSPEC_XFGVIRT(ITfKeystrokeMgr, GetPreservedKeyDescription)
        HRESULT ( STDMETHODCALLTYPE *GetPreservedKeyDescription )( 
            ITfKeystrokeMgr * This,
            /* [in] */ REFGUID rguid,
            /* [out] */ BSTR *pbstrDesc);
        
        DECLSPEC_XFGVIRT(ITfKeystrokeMgr, SimulatePreservedKey)
        HRESULT ( STDMETHODCALLTYPE *SimulatePreservedKey )( 
            ITfKeystrokeMgr * This,
            /* [in] */ ITfContext *pic,
            /* [in] */ REFGUID rguid,
            /* [out] */ BOOL *pfEaten);
        
        END_INTERFACE
    } ITfKeystrokeMgrVtbl;

    interface ITfKeystrokeMgr
    {
        CONST_VTBL struct ITfKeystrokeMgrVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ITfKeystrokeMgr_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ITfKeystrokeMgr_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ITfKeystrokeMgr_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ITfKeystrokeMgr_AdviseKeyEventSink(This,tid,pSink,fForeground)	\
    ( (This)->lpVtbl -> AdviseKeyEventSink(This,tid,pSink,fForeground) ) 

#define ITfKeystrokeMgr_UnadviseKeyEventSink(This,tid)	\
    ( (This)->lpVtbl -> UnadviseKeyEventSink(This,tid) ) 

#define ITfKeystrokeMgr_GetForeground(This,pclsid)	\
    ( (This)->lpVtbl -> GetForeground(This,pclsid) ) 

#define ITfKeystrokeMgr_TestKeyDown(This,wParam,lParam,pfEaten)	\
    ( (This)->lpVtbl -> TestKeyDown(This,wParam,lParam,pfEaten) ) 

#define ITfKeystrokeMgr_TestKeyUp(This,wParam,lParam,pfEaten)	\
    ( (This)->lpVtbl -> TestKeyUp(This,wParam,lParam,pfEaten) ) 

#define ITfKeystrokeMgr_KeyDown(This,wParam,lParam,pfEaten)	\
    ( (This)->lpVtbl -> KeyDown(This,wParam,lParam,pfEaten) ) 

#define ITfKeystrokeMgr_KeyUp(This,wParam,lParam,pfEaten)	\
    ( (This)->lpVtbl -> KeyUp(This,wParam,lParam,pfEaten) ) 

#define ITfKeystrokeMgr_GetPreservedKey(This,pic,pprekey,pguid)	\
    ( (This)->lpVtbl -> GetPreservedKey(This,pic,pprekey,pguid) ) 

#define ITfKeystrokeMgr_IsPreservedKey(This,rguid,pprekey,pfRegistered)	\
    ( (This)->lpVtbl -> IsPreservedKey(This,rguid,pprekey,pfRegistered) ) 

#define ITfKeystrokeMgr_PreserveKey(This,tid,rguid,prekey,pchDesc,cchDesc)	\
    ( (This)->lpVtbl -> PreserveKey(This,tid,rguid,prekey,pchDesc,cchDesc) ) 

#define ITfKeystrokeMgr_UnpreserveKey(This,rguid,pprekey)	\
    ( (This)->lpVtbl -> UnpreserveKey(This,rguid,pprekey) ) 

#define ITfKeystrokeMgr_SetPreservedKeyDescription(This,rguid,pchDesc,cchDesc)	\
    ( (This)->lpVtbl -> SetPreservedKeyDescription(This,rguid,pchDesc,cchDesc) ) 

#define ITfKeystrokeMgr_GetPreservedKeyDescription(This,rguid,pbstrDesc)	\
    ( (This)->lpVtbl -> GetPreservedKeyDescription(This,rguid,pbstrDesc) ) 

#define ITfKeystrokeMgr_SimulatePreservedKey(This,pic,rguid,pfEaten)	\
    ( (This)->lpVtbl -> SimulatePreservedKey(This,pic,rguid,pfEaten) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ITfKeystrokeMgr_INTERFACE_DEFINED__ */


#ifndef __ITfKeyEventSink_INTERFACE_DEFINED__
#define __ITfKeyEventSink_INTERFACE_DEFINED__

/* interface ITfKeyEventSink */
/* [unique][uuid][local][object] */ 


EXTERN_C const IID IID_ITfKeyEventSink;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("aa80e7f5-2021-11d2-93e0-0060b067b86e")
    ITfKeyEventSink : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE OnSetFocus( 
            /* [in] */ BOOL fForeground) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE OnTestKeyDown( 
            /* [in] */ ITfContext *pic,
            /* [in] */ WPARAM wParam,
            /* [in] */ LPARAM lParam,
            /* [out] */ BOOL *pfEaten) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE OnTestKeyUp( 
            /* [in] */ ITfContext *pic,
            /* [in] */ WPARAM wParam,
            /* [in] */ LPARAM lParam,
            /* [out] */ BOOL *pfEaten) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE OnKeyDown( 
            /* [in] */ ITfContext *pic,
            /* [in] */ WPARAM wParam,
            /* [in] */ LPARAM lParam,
            /* [out] */ BOOL *pfEaten) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE OnKeyUp( 
            /* [in] */ ITfContext *pic,
            /* [in] */ WPARAM wParam,
            /* [in] */ LPARAM lParam,
            /* [out] */ BOOL *pfEaten) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE OnPreservedKey( 
            /* [in] */ ITfContext *pic,
            /* [in] */ REFGUID rguid,
            /* [out] */ BOOL *pfEaten) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ITfKeyEventSinkVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            ITfKeyEventSink * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            ITfKeyEventSink * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            ITfKeyEventSink * This);
        
        DECLSPEC_XFGVIRT(ITfKeyEventSink, OnSetFocus)
        HRESULT ( STDMETHODCALLTYPE *OnSetFocus )( 
            ITfKeyEventSink * This,
            /* [in] */ BOOL fForeground);
        
        DECLSPEC_XFGVIRT(ITfKeyEventSink, OnTestKeyDown)
        HRESULT ( STDMETHODCALLTYPE *OnTestKeyDown )( 
            ITfKeyEventSink * This,
            /* [in] */ ITfContext *pic,
            /* [in] */ WPARAM wParam,
            /* [in] */ LPARAM lParam,
            /* [out] */ BOOL *pfEaten);
        
        DECLSPEC_XFGVIRT(ITfKeyEventSink, OnTestKeyUp)
        HRESULT ( STDMETHODCALLTYPE *OnTestKeyUp )( 
            ITfKeyEventSink * This,
            /* [in] */ ITfContext *pic,
            /* [in] */ WPARAM wParam,
            /* [in] */ LPARAM lParam,
            /* [out] */ BOOL *pfEaten);
        
        DECLSPEC_XFGVIRT(ITfKeyEventSink, OnKeyDown)
        HRESULT ( STDMETHODCALLTYPE *OnKeyDown )( 
            ITfKeyEventSink * This,
            /* [in] */ ITfContext *pic,
            /* [in] */ WPARAM wParam,
            /* [in] */ LPARAM lParam,
            /* [out] */ BOOL *pfEaten);
        
        DECLSPEC_XFGVIRT(ITfKeyEventSink, OnKeyUp)
        HRESULT ( STDMETHODCALLTYPE *OnKeyUp )( 
            ITfKeyEventSink * This,
            /* [in] */ ITfContext *pic,
            /* [in] */ WPARAM wParam,
            /* [in] */ LPARAM lParam,
            /* [out] */ BOOL *pfEaten);
        
        DECLSPEC_XFGVIRT(ITfKeyEventSink, OnPreservedKey)
        HRESULT ( STDMETHODCALLTYPE *OnPreservedKey )( 
            ITfKeyEventSink * This,
            /* [in] */ ITfContext *pic,
            /* [in] */ REFGUID rguid,
            /* [out] */ BOOL *pfEaten);
        
        END_INTERFACE
    } ITfKeyEventSinkVtbl;

    interface ITfKeyEventSink
    {
        CONST_VTBL struct ITfKeyEventSinkVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ITfKeyEventSink_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ITfKeyEventSink_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ITfKeyEventSink_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ITfKeyEventSink_OnSetFocus(This,fForeground)	\
    ( (This)->lpVtbl -> OnSetFocus(This,fForeground) ) 

#define ITfKeyEventSink_OnTestKeyDown(This,pic,wParam,lParam,pfEaten)	\
    ( (This)->lpVtbl -> OnTestKeyDown(This,pic,wParam,lParam,pfEaten) ) 

#define ITfKeyEventSink_OnTestKeyUp(This,pic,wParam,lParam,pfEaten)	\
    ( (This)->lpVtbl -> OnTestKeyUp(This,pic,wParam,lParam,pfEaten) ) 

#define ITfKeyEventSink_OnKeyDown(This,pic,wParam,lParam,pfEaten)	\
    ( (This)->lpVtbl -> OnKeyDown(This,pic,wParam,lParam,pfEaten) ) 

#define ITfKeyEventSink_OnKeyUp(This,pic,wParam,lParam,pfEaten)	\
    ( (This)->lpVtbl -> OnKeyUp(This,pic,wParam,lParam,pfEaten) ) 

#define ITfKeyEventSink_OnPreservedKey(This,pic,rguid,pfEaten)	\
    ( (This)->lpVtbl -> OnPreservedKey(This,pic,rguid,pfEaten) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ITfKeyEventSink_INTERFACE_DEFINED__ */


#ifndef __ITfKeyTraceEventSink_INTERFACE_DEFINED__
#define __ITfKeyTraceEventSink_INTERFACE_DEFINED__

/* interface ITfKeyTraceEventSink */
/* [unique][uuid][local][object] */ 


EXTERN_C const IID IID_ITfKeyTraceEventSink;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("1cd4c13b-1c36-4191-a70a-7f3e611f367d")
    ITfKeyTraceEventSink : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE OnKeyTraceDown( 
            /* [in] */ WPARAM wParam,
            /* [in] */ LPARAM lParam) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE OnKeyTraceUp( 
            /* [in] */ WPARAM wParam,
            /* [in] */ LPARAM lParam) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ITfKeyTraceEventSinkVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            ITfKeyTraceEventSink * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            ITfKeyTraceEventSink * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            ITfKeyTraceEventSink * This);
        
        DECLSPEC_XFGVIRT(ITfKeyTraceEventSink, OnKeyTraceDown)
        HRESULT ( STDMETHODCALLTYPE *OnKeyTraceDown )( 
            ITfKeyTraceEventSink * This,
            /* [in] */ WPARAM wParam,
            /* [in] */ LPARAM lParam);
        
        DECLSPEC_XFGVIRT(ITfKeyTraceEventSink, OnKeyTraceUp)
        HRESULT ( STDMETHODCALLTYPE *OnKeyTraceUp )( 
            ITfKeyTraceEventSink * This,
            /* [in] */ WPARAM wParam,
            /* [in] */ LPARAM lParam);
        
        END_INTERFACE
    } ITfKeyTraceEventSinkVtbl;

    interface ITfKeyTraceEventSink
    {
        CONST_VTBL struct ITfKeyTraceEventSinkVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ITfKeyTraceEventSink_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ITfKeyTraceEventSink_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ITfKeyTraceEventSink_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ITfKeyTraceEventSink_OnKeyTraceDown(This,wParam,lParam)	\
    ( (This)->lpVtbl -> OnKeyTraceDown(This,wParam,lParam) ) 

#define ITfKeyTraceEventSink_OnKeyTraceUp(This,wParam,lParam)	\
    ( (This)->lpVtbl -> OnKeyTraceUp(This,wParam,lParam) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ITfKeyTraceEventSink_INTERFACE_DEFINED__ */


#ifndef __ITfPreservedKeyNotifySink_INTERFACE_DEFINED__
#define __ITfPreservedKeyNotifySink_INTERFACE_DEFINED__

/* interface ITfPreservedKeyNotifySink */
/* [unique][uuid][object] */ 


EXTERN_C const IID IID_ITfPreservedKeyNotifySink;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("6f77c993-d2b1-446e-853e-5912efc8a286")
    ITfPreservedKeyNotifySink : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE OnUpdated( 
            /* [in] */ __RPC__in const TF_PRESERVEDKEY *pprekey) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ITfPreservedKeyNotifySinkVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in ITfPreservedKeyNotifySink * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in ITfPreservedKeyNotifySink * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in ITfPreservedKeyNotifySink * This);
        
        DECLSPEC_XFGVIRT(ITfPreservedKeyNotifySink, OnUpdated)
        HRESULT ( STDMETHODCALLTYPE *OnUpdated )( 
            __RPC__in ITfPreservedKeyNotifySink * This,
            /* [in] */ __RPC__in const TF_PRESERVEDKEY *pprekey);
        
        END_INTERFACE
    } ITfPreservedKeyNotifySinkVtbl;

    interface ITfPreservedKeyNotifySink
    {
        CONST_VTBL struct ITfPreservedKeyNotifySinkVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ITfPreservedKeyNotifySink_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ITfPreservedKeyNotifySink_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ITfPreservedKeyNotifySink_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ITfPreservedKeyNotifySink_OnUpdated(This,pprekey)	\
    ( (This)->lpVtbl -> OnUpdated(This,pprekey) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ITfPreservedKeyNotifySink_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_msctf_0000_0065 */
/* [local] */ 

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP) */
#pragma endregion
#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)


extern RPC_IF_HANDLE __MIDL_itf_msctf_0000_0065_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_msctf_0000_0065_v0_0_s_ifspec;

#ifndef __ITfMessagePump_INTERFACE_DEFINED__
#define __ITfMessagePump_INTERFACE_DEFINED__

/* interface ITfMessagePump */
/* [unique][uuid][local][object] */ 


EXTERN_C const IID IID_ITfMessagePump;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("8f1b8ad8-0b6b-4874-90c5-bd76011e8f7c")
    ITfMessagePump : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE PeekMessageA( 
            /* [out] */ LPMSG pMsg,
            /* [in] */ HWND hwnd,
            /* [in] */ UINT wMsgFilterMin,
            /* [in] */ UINT wMsgFilterMax,
            /* [in] */ UINT wRemoveMsg,
            /* [out] */ BOOL *pfResult) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetMessageA( 
            /* [out] */ LPMSG pMsg,
            /* [in] */ HWND hwnd,
            /* [in] */ UINT wMsgFilterMin,
            /* [in] */ UINT wMsgFilterMax,
            /* [out] */ BOOL *pfResult) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE PeekMessageW( 
            /* [out] */ LPMSG pMsg,
            /* [in] */ HWND hwnd,
            /* [in] */ UINT wMsgFilterMin,
            /* [in] */ UINT wMsgFilterMax,
            /* [in] */ UINT wRemoveMsg,
            /* [out] */ BOOL *pfResult) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetMessageW( 
            /* [out] */ LPMSG pMsg,
            /* [in] */ HWND hwnd,
            /* [in] */ UINT wMsgFilterMin,
            /* [in] */ UINT wMsgFilterMax,
            /* [out] */ BOOL *pfResult) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ITfMessagePumpVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            ITfMessagePump * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            ITfMessagePump * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            ITfMessagePump * This);
        
        DECLSPEC_XFGVIRT(ITfMessagePump, PeekMessageA)
        HRESULT ( STDMETHODCALLTYPE *PeekMessageA )( 
            ITfMessagePump * This,
            /* [out] */ LPMSG pMsg,
            /* [in] */ HWND hwnd,
            /* [in] */ UINT wMsgFilterMin,
            /* [in] */ UINT wMsgFilterMax,
            /* [in] */ UINT wRemoveMsg,
            /* [out] */ BOOL *pfResult);
        
        DECLSPEC_XFGVIRT(ITfMessagePump, GetMessageA)
        HRESULT ( STDMETHODCALLTYPE *GetMessageA )( 
            ITfMessagePump * This,
            /* [out] */ LPMSG pMsg,
            /* [in] */ HWND hwnd,
            /* [in] */ UINT wMsgFilterMin,
            /* [in] */ UINT wMsgFilterMax,
            /* [out] */ BOOL *pfResult);
        
        DECLSPEC_XFGVIRT(ITfMessagePump, PeekMessageW)
        HRESULT ( STDMETHODCALLTYPE *PeekMessageW )( 
            ITfMessagePump * This,
            /* [out] */ LPMSG pMsg,
            /* [in] */ HWND hwnd,
            /* [in] */ UINT wMsgFilterMin,
            /* [in] */ UINT wMsgFilterMax,
            /* [in] */ UINT wRemoveMsg,
            /* [out] */ BOOL *pfResult);
        
        DECLSPEC_XFGVIRT(ITfMessagePump, GetMessageW)
        HRESULT ( STDMETHODCALLTYPE *GetMessageW )( 
            ITfMessagePump * This,
            /* [out] */ LPMSG pMsg,
            /* [in] */ HWND hwnd,
            /* [in] */ UINT wMsgFilterMin,
            /* [in] */ UINT wMsgFilterMax,
            /* [out] */ BOOL *pfResult);
        
        END_INTERFACE
    } ITfMessagePumpVtbl;

    interface ITfMessagePump
    {
        CONST_VTBL struct ITfMessagePumpVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ITfMessagePump_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ITfMessagePump_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ITfMessagePump_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ITfMessagePump_PeekMessageA(This,pMsg,hwnd,wMsgFilterMin,wMsgFilterMax,wRemoveMsg,pfResult)	\
    ( (This)->lpVtbl -> PeekMessageA(This,pMsg,hwnd,wMsgFilterMin,wMsgFilterMax,wRemoveMsg,pfResult) ) 

#define ITfMessagePump_GetMessageA(This,pMsg,hwnd,wMsgFilterMin,wMsgFilterMax,pfResult)	\
    ( (This)->lpVtbl -> GetMessageA(This,pMsg,hwnd,wMsgFilterMin,wMsgFilterMax,pfResult) ) 

#define ITfMessagePump_PeekMessageW(This,pMsg,hwnd,wMsgFilterMin,wMsgFilterMax,wRemoveMsg,pfResult)	\
    ( (This)->lpVtbl -> PeekMessageW(This,pMsg,hwnd,wMsgFilterMin,wMsgFilterMax,wRemoveMsg,pfResult) ) 

#define ITfMessagePump_GetMessageW(This,pMsg,hwnd,wMsgFilterMin,wMsgFilterMax,pfResult)	\
    ( (This)->lpVtbl -> GetMessageW(This,pMsg,hwnd,wMsgFilterMin,wMsgFilterMax,pfResult) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ITfMessagePump_INTERFACE_DEFINED__ */


#ifndef __ITfThreadFocusSink_INTERFACE_DEFINED__
#define __ITfThreadFocusSink_INTERFACE_DEFINED__

/* interface ITfThreadFocusSink */
/* [unique][uuid][local][object] */ 


EXTERN_C const IID IID_ITfThreadFocusSink;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("c0f1db0c-3a20-405c-a303-96b6010a885f")
    ITfThreadFocusSink : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE OnSetThreadFocus( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE OnKillThreadFocus( void) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ITfThreadFocusSinkVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            ITfThreadFocusSink * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            ITfThreadFocusSink * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            ITfThreadFocusSink * This);
        
        DECLSPEC_XFGVIRT(ITfThreadFocusSink, OnSetThreadFocus)
        HRESULT ( STDMETHODCALLTYPE *OnSetThreadFocus )( 
            ITfThreadFocusSink * This);
        
        DECLSPEC_XFGVIRT(ITfThreadFocusSink, OnKillThreadFocus)
        HRESULT ( STDMETHODCALLTYPE *OnKillThreadFocus )( 
            ITfThreadFocusSink * This);
        
        END_INTERFACE
    } ITfThreadFocusSinkVtbl;

    interface ITfThreadFocusSink
    {
        CONST_VTBL struct ITfThreadFocusSinkVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ITfThreadFocusSink_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ITfThreadFocusSink_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ITfThreadFocusSink_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ITfThreadFocusSink_OnSetThreadFocus(This)	\
    ( (This)->lpVtbl -> OnSetThreadFocus(This) ) 

#define ITfThreadFocusSink_OnKillThreadFocus(This)	\
    ( (This)->lpVtbl -> OnKillThreadFocus(This) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ITfThreadFocusSink_INTERFACE_DEFINED__ */


#ifndef __ITfTextInputProcessor_INTERFACE_DEFINED__
#define __ITfTextInputProcessor_INTERFACE_DEFINED__

/* interface ITfTextInputProcessor */
/* [unique][uuid][local][object] */ 


EXTERN_C const IID IID_ITfTextInputProcessor;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("aa80e7f7-2021-11d2-93e0-0060b067b86e")
    ITfTextInputProcessor : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE Activate( 
            /* [in] */ ITfThreadMgr *ptim,
            /* [in] */ TfClientId tid) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Deactivate( void) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ITfTextInputProcessorVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            ITfTextInputProcessor * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            ITfTextInputProcessor * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            ITfTextInputProcessor * This);
        
        DECLSPEC_XFGVIRT(ITfTextInputProcessor, Activate)
        HRESULT ( STDMETHODCALLTYPE *Activate )( 
            ITfTextInputProcessor * This,
            /* [in] */ ITfThreadMgr *ptim,
            /* [in] */ TfClientId tid);
        
        DECLSPEC_XFGVIRT(ITfTextInputProcessor, Deactivate)
        HRESULT ( STDMETHODCALLTYPE *Deactivate )( 
            ITfTextInputProcessor * This);
        
        END_INTERFACE
    } ITfTextInputProcessorVtbl;

    interface ITfTextInputProcessor
    {
        CONST_VTBL struct ITfTextInputProcessorVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ITfTextInputProcessor_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ITfTextInputProcessor_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ITfTextInputProcessor_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ITfTextInputProcessor_Activate(This,ptim,tid)	\
    ( (This)->lpVtbl -> Activate(This,ptim,tid) ) 

#define ITfTextInputProcessor_Deactivate(This)	\
    ( (This)->lpVtbl -> Deactivate(This) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ITfTextInputProcessor_INTERFACE_DEFINED__ */


#ifndef __ITfTextInputProcessorEx_INTERFACE_DEFINED__
#define __ITfTextInputProcessorEx_INTERFACE_DEFINED__

/* interface ITfTextInputProcessorEx */
/* [unique][uuid][local][object] */ 


EXTERN_C const IID IID_ITfTextInputProcessorEx;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("6e4e2102-f9cd-433d-b496-303ce03a6507")
    ITfTextInputProcessorEx : public ITfTextInputProcessor
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE ActivateEx( 
            /* [in] */ ITfThreadMgr *ptim,
            /* [in] */ TfClientId tid,
            /* [in] */ DWORD dwFlags) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ITfTextInputProcessorExVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            ITfTextInputProcessorEx * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            ITfTextInputProcessorEx * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            ITfTextInputProcessorEx * This);
        
        DECLSPEC_XFGVIRT(ITfTextInputProcessor, Activate)
        HRESULT ( STDMETHODCALLTYPE *Activate )( 
            ITfTextInputProcessorEx * This,
            /* [in] */ ITfThreadMgr *ptim,
            /* [in] */ TfClientId tid);
        
        DECLSPEC_XFGVIRT(ITfTextInputProcessor, Deactivate)
        HRESULT ( STDMETHODCALLTYPE *Deactivate )( 
            ITfTextInputProcessorEx * This);
        
        DECLSPEC_XFGVIRT(ITfTextInputProcessorEx, ActivateEx)
        HRESULT ( STDMETHODCALLTYPE *ActivateEx )( 
            ITfTextInputProcessorEx * This,
            /* [in] */ ITfThreadMgr *ptim,
            /* [in] */ TfClientId tid,
            /* [in] */ DWORD dwFlags);
        
        END_INTERFACE
    } ITfTextInputProcessorExVtbl;

    interface ITfTextInputProcessorEx
    {
        CONST_VTBL struct ITfTextInputProcessorExVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ITfTextInputProcessorEx_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ITfTextInputProcessorEx_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ITfTextInputProcessorEx_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ITfTextInputProcessorEx_Activate(This,ptim,tid)	\
    ( (This)->lpVtbl -> Activate(This,ptim,tid) ) 

#define ITfTextInputProcessorEx_Deactivate(This)	\
    ( (This)->lpVtbl -> Deactivate(This) ) 


#define ITfTextInputProcessorEx_ActivateEx(This,ptim,tid,dwFlags)	\
    ( (This)->lpVtbl -> ActivateEx(This,ptim,tid,dwFlags) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ITfTextInputProcessorEx_INTERFACE_DEFINED__ */


#ifndef __ITfClientId_INTERFACE_DEFINED__
#define __ITfClientId_INTERFACE_DEFINED__

/* interface ITfClientId */
/* [unique][uuid][local][object] */ 


EXTERN_C const IID IID_ITfClientId;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("d60a7b49-1b9f-4be2-b702-47e9dc05dec3")
    ITfClientId : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetClientId( 
            /* [in] */ REFCLSID rclsid,
            /* [out] */ TfClientId *ptid) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ITfClientIdVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            ITfClientId * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            ITfClientId * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            ITfClientId * This);
        
        DECLSPEC_XFGVIRT(ITfClientId, GetClientId)
        HRESULT ( STDMETHODCALLTYPE *GetClientId )( 
            ITfClientId * This,
            /* [in] */ REFCLSID rclsid,
            /* [out] */ TfClientId *ptid);
        
        END_INTERFACE
    } ITfClientIdVtbl;

    interface ITfClientId
    {
        CONST_VTBL struct ITfClientIdVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ITfClientId_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ITfClientId_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ITfClientId_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ITfClientId_GetClientId(This,rclsid,ptid)	\
    ( (This)->lpVtbl -> GetClientId(This,rclsid,ptid) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ITfClientId_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_msctf_0000_0070 */
/* [local] */ 

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion
#pragma region Application Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP)
typedef /* [public][public][public][public][uuid] */  DECLSPEC_UUID("c4cc07f1-80cc-4a7b-bc54-98512782cbe3") 
enum __MIDL___MIDL_itf_msctf_0000_0070_0001
    {
        TF_LS_NONE	= 0,
        TF_LS_SOLID	= 1,
        TF_LS_DOT	= 2,
        TF_LS_DASH	= 3,
        TF_LS_SQUIGGLE	= 4
    } 	TF_DA_LINESTYLE;

typedef /* [public][public][public][public][public][uuid] */  DECLSPEC_UUID("d9b92e21-084a-401b-9c64-1e6dad91a1ab") 
enum __MIDL___MIDL_itf_msctf_0000_0070_0002
    {
        TF_CT_NONE	= 0,
        TF_CT_SYSCOLOR	= 1,
        TF_CT_COLORREF	= 2
    } 	TF_DA_COLORTYPE;

typedef /* [uuid] */  DECLSPEC_UUID("90d0cb5e-6520-4a0f-b47c-c39bd955f0d6") struct TF_DA_COLOR
    {
    TF_DA_COLORTYPE type;
    /* [switch_is][switch_type] */ union 
        {
        /* [case()] */ int nIndex;
        /* [case()] */ COLORREF cr;
        } 	;
    } 	TF_DA_COLOR;

typedef /* [public][public][public][public][uuid] */  DECLSPEC_UUID("33d2fe4b-6c24-4f67-8d75-3bc1819e4126") 
enum __MIDL___MIDL_itf_msctf_0000_0070_0004
    {
        TF_ATTR_INPUT	= 0,
        TF_ATTR_TARGET_CONVERTED	= 1,
        TF_ATTR_CONVERTED	= 2,
        TF_ATTR_TARGET_NOTCONVERTED	= 3,
        TF_ATTR_INPUT_ERROR	= 4,
        TF_ATTR_FIXEDCONVERTED	= 5,
        TF_ATTR_OTHER	= -1
    } 	TF_DA_ATTR_INFO;

typedef /* [uuid] */  DECLSPEC_UUID("1bf1c305-419b-4182-a4d2-9bfadc3f021f") struct TF_DISPLAYATTRIBUTE
    {
    TF_DA_COLOR crText;
    TF_DA_COLOR crBk;
    TF_DA_LINESTYLE lsStyle;
    BOOL fBoldLine;
    TF_DA_COLOR crLine;
    TF_DA_ATTR_INFO bAttr;
    } 	TF_DISPLAYATTRIBUTE;



extern RPC_IF_HANDLE __MIDL_itf_msctf_0000_0070_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_msctf_0000_0070_v0_0_s_ifspec;

#ifndef __ITfDisplayAttributeInfo_INTERFACE_DEFINED__
#define __ITfDisplayAttributeInfo_INTERFACE_DEFINED__

/* interface ITfDisplayAttributeInfo */
/* [unique][uuid][object] */ 


EXTERN_C const IID IID_ITfDisplayAttributeInfo;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("70528852-2f26-4aea-8c96-215150578932")
    ITfDisplayAttributeInfo : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetGUID( 
            /* [out] */ __RPC__out GUID *pguid) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetDescription( 
            /* [out] */ __RPC__deref_out_opt BSTR *pbstrDesc) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetAttributeInfo( 
            /* [out] */ __RPC__out TF_DISPLAYATTRIBUTE *pda) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetAttributeInfo( 
            /* [in] */ __RPC__in const TF_DISPLAYATTRIBUTE *pda) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Reset( void) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ITfDisplayAttributeInfoVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in ITfDisplayAttributeInfo * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in ITfDisplayAttributeInfo * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in ITfDisplayAttributeInfo * This);
        
        DECLSPEC_XFGVIRT(ITfDisplayAttributeInfo, GetGUID)
        HRESULT ( STDMETHODCALLTYPE *GetGUID )( 
            __RPC__in ITfDisplayAttributeInfo * This,
            /* [out] */ __RPC__out GUID *pguid);
        
        DECLSPEC_XFGVIRT(ITfDisplayAttributeInfo, GetDescription)
        HRESULT ( STDMETHODCALLTYPE *GetDescription )( 
            __RPC__in ITfDisplayAttributeInfo * This,
            /* [out] */ __RPC__deref_out_opt BSTR *pbstrDesc);
        
        DECLSPEC_XFGVIRT(ITfDisplayAttributeInfo, GetAttributeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetAttributeInfo )( 
            __RPC__in ITfDisplayAttributeInfo * This,
            /* [out] */ __RPC__out TF_DISPLAYATTRIBUTE *pda);
        
        DECLSPEC_XFGVIRT(ITfDisplayAttributeInfo, SetAttributeInfo)
        HRESULT ( STDMETHODCALLTYPE *SetAttributeInfo )( 
            __RPC__in ITfDisplayAttributeInfo * This,
            /* [in] */ __RPC__in const TF_DISPLAYATTRIBUTE *pda);
        
        DECLSPEC_XFGVIRT(ITfDisplayAttributeInfo, Reset)
        HRESULT ( STDMETHODCALLTYPE *Reset )( 
            __RPC__in ITfDisplayAttributeInfo * This);
        
        END_INTERFACE
    } ITfDisplayAttributeInfoVtbl;

    interface ITfDisplayAttributeInfo
    {
        CONST_VTBL struct ITfDisplayAttributeInfoVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ITfDisplayAttributeInfo_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ITfDisplayAttributeInfo_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ITfDisplayAttributeInfo_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ITfDisplayAttributeInfo_GetGUID(This,pguid)	\
    ( (This)->lpVtbl -> GetGUID(This,pguid) ) 

#define ITfDisplayAttributeInfo_GetDescription(This,pbstrDesc)	\
    ( (This)->lpVtbl -> GetDescription(This,pbstrDesc) ) 

#define ITfDisplayAttributeInfo_GetAttributeInfo(This,pda)	\
    ( (This)->lpVtbl -> GetAttributeInfo(This,pda) ) 

#define ITfDisplayAttributeInfo_SetAttributeInfo(This,pda)	\
    ( (This)->lpVtbl -> SetAttributeInfo(This,pda) ) 

#define ITfDisplayAttributeInfo_Reset(This)	\
    ( (This)->lpVtbl -> Reset(This) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ITfDisplayAttributeInfo_INTERFACE_DEFINED__ */


#ifndef __IEnumTfDisplayAttributeInfo_INTERFACE_DEFINED__
#define __IEnumTfDisplayAttributeInfo_INTERFACE_DEFINED__

/* interface IEnumTfDisplayAttributeInfo */
/* [unique][uuid][object] */ 


EXTERN_C const IID IID_IEnumTfDisplayAttributeInfo;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("7cef04d7-cb75-4e80-a7ab-5f5bc7d332de")
    IEnumTfDisplayAttributeInfo : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE Clone( 
            /* [out] */ __RPC__deref_out_opt IEnumTfDisplayAttributeInfo **ppEnum) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Next( 
            /* [in] */ ULONG ulCount,
            /* [length_is][size_is][out] */ __RPC__out_ecount_part(ulCount, *pcFetched) ITfDisplayAttributeInfo **rgInfo,
            /* [out] */ __RPC__out ULONG *pcFetched) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Reset( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Skip( 
            /* [in] */ ULONG ulCount) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IEnumTfDisplayAttributeInfoVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IEnumTfDisplayAttributeInfo * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IEnumTfDisplayAttributeInfo * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IEnumTfDisplayAttributeInfo * This);
        
        DECLSPEC_XFGVIRT(IEnumTfDisplayAttributeInfo, Clone)
        HRESULT ( STDMETHODCALLTYPE *Clone )( 
            __RPC__in IEnumTfDisplayAttributeInfo * This,
            /* [out] */ __RPC__deref_out_opt IEnumTfDisplayAttributeInfo **ppEnum);
        
        DECLSPEC_XFGVIRT(IEnumTfDisplayAttributeInfo, Next)
        HRESULT ( STDMETHODCALLTYPE *Next )( 
            __RPC__in IEnumTfDisplayAttributeInfo * This,
            /* [in] */ ULONG ulCount,
            /* [length_is][size_is][out] */ __RPC__out_ecount_part(ulCount, *pcFetched) ITfDisplayAttributeInfo **rgInfo,
            /* [out] */ __RPC__out ULONG *pcFetched);
        
        DECLSPEC_XFGVIRT(IEnumTfDisplayAttributeInfo, Reset)
        HRESULT ( STDMETHODCALLTYPE *Reset )( 
            __RPC__in IEnumTfDisplayAttributeInfo * This);
        
        DECLSPEC_XFGVIRT(IEnumTfDisplayAttributeInfo, Skip)
        HRESULT ( STDMETHODCALLTYPE *Skip )( 
            __RPC__in IEnumTfDisplayAttributeInfo * This,
            /* [in] */ ULONG ulCount);
        
        END_INTERFACE
    } IEnumTfDisplayAttributeInfoVtbl;

    interface IEnumTfDisplayAttributeInfo
    {
        CONST_VTBL struct IEnumTfDisplayAttributeInfoVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IEnumTfDisplayAttributeInfo_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IEnumTfDisplayAttributeInfo_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IEnumTfDisplayAttributeInfo_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IEnumTfDisplayAttributeInfo_Clone(This,ppEnum)	\
    ( (This)->lpVtbl -> Clone(This,ppEnum) ) 

#define IEnumTfDisplayAttributeInfo_Next(This,ulCount,rgInfo,pcFetched)	\
    ( (This)->lpVtbl -> Next(This,ulCount,rgInfo,pcFetched) ) 

#define IEnumTfDisplayAttributeInfo_Reset(This)	\
    ( (This)->lpVtbl -> Reset(This) ) 

#define IEnumTfDisplayAttributeInfo_Skip(This,ulCount)	\
    ( (This)->lpVtbl -> Skip(This,ulCount) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IEnumTfDisplayAttributeInfo_INTERFACE_DEFINED__ */


#ifndef __ITfDisplayAttributeProvider_INTERFACE_DEFINED__
#define __ITfDisplayAttributeProvider_INTERFACE_DEFINED__

/* interface ITfDisplayAttributeProvider */
/* [unique][uuid][object] */ 


EXTERN_C const IID IID_ITfDisplayAttributeProvider;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("fee47777-163c-4769-996a-6e9c50ad8f54")
    ITfDisplayAttributeProvider : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE EnumDisplayAttributeInfo( 
            /* [out] */ __RPC__deref_out_opt IEnumTfDisplayAttributeInfo **ppEnum) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetDisplayAttributeInfo( 
            /* [in] */ __RPC__in REFGUID guid,
            /* [out] */ __RPC__deref_out_opt ITfDisplayAttributeInfo **ppInfo) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ITfDisplayAttributeProviderVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in ITfDisplayAttributeProvider * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in ITfDisplayAttributeProvider * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in ITfDisplayAttributeProvider * This);
        
        DECLSPEC_XFGVIRT(ITfDisplayAttributeProvider, EnumDisplayAttributeInfo)
        HRESULT ( STDMETHODCALLTYPE *EnumDisplayAttributeInfo )( 
            __RPC__in ITfDisplayAttributeProvider * This,
            /* [out] */ __RPC__deref_out_opt IEnumTfDisplayAttributeInfo **ppEnum);
        
        DECLSPEC_XFGVIRT(ITfDisplayAttributeProvider, GetDisplayAttributeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetDisplayAttributeInfo )( 
            __RPC__in ITfDisplayAttributeProvider * This,
            /* [in] */ __RPC__in REFGUID guid,
            /* [out] */ __RPC__deref_out_opt ITfDisplayAttributeInfo **ppInfo);
        
        END_INTERFACE
    } ITfDisplayAttributeProviderVtbl;

    interface ITfDisplayAttributeProvider
    {
        CONST_VTBL struct ITfDisplayAttributeProviderVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ITfDisplayAttributeProvider_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ITfDisplayAttributeProvider_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ITfDisplayAttributeProvider_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ITfDisplayAttributeProvider_EnumDisplayAttributeInfo(This,ppEnum)	\
    ( (This)->lpVtbl -> EnumDisplayAttributeInfo(This,ppEnum) ) 

#define ITfDisplayAttributeProvider_GetDisplayAttributeInfo(This,guid,ppInfo)	\
    ( (This)->lpVtbl -> GetDisplayAttributeInfo(This,guid,ppInfo) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ITfDisplayAttributeProvider_INTERFACE_DEFINED__ */


#ifndef __ITfDisplayAttributeMgr_INTERFACE_DEFINED__
#define __ITfDisplayAttributeMgr_INTERFACE_DEFINED__

/* interface ITfDisplayAttributeMgr */
/* [unique][uuid][local][object] */ 


EXTERN_C const IID IID_ITfDisplayAttributeMgr;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("8ded7393-5db1-475c-9e71-a39111b0ff67")
    ITfDisplayAttributeMgr : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE OnUpdateInfo( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE EnumDisplayAttributeInfo( 
            /* [out] */ IEnumTfDisplayAttributeInfo **ppEnum) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetDisplayAttributeInfo( 
            /* [in] */ REFGUID guid,
            /* [out] */ ITfDisplayAttributeInfo **ppInfo,
            /* [out] */ CLSID *pclsidOwner) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ITfDisplayAttributeMgrVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            ITfDisplayAttributeMgr * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            ITfDisplayAttributeMgr * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            ITfDisplayAttributeMgr * This);
        
        DECLSPEC_XFGVIRT(ITfDisplayAttributeMgr, OnUpdateInfo)
        HRESULT ( STDMETHODCALLTYPE *OnUpdateInfo )( 
            ITfDisplayAttributeMgr * This);
        
        DECLSPEC_XFGVIRT(ITfDisplayAttributeMgr, EnumDisplayAttributeInfo)
        HRESULT ( STDMETHODCALLTYPE *EnumDisplayAttributeInfo )( 
            ITfDisplayAttributeMgr * This,
            /* [out] */ IEnumTfDisplayAttributeInfo **ppEnum);
        
        DECLSPEC_XFGVIRT(ITfDisplayAttributeMgr, GetDisplayAttributeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetDisplayAttributeInfo )( 
            ITfDisplayAttributeMgr * This,
            /* [in] */ REFGUID guid,
            /* [out] */ ITfDisplayAttributeInfo **ppInfo,
            /* [out] */ CLSID *pclsidOwner);
        
        END_INTERFACE
    } ITfDisplayAttributeMgrVtbl;

    interface ITfDisplayAttributeMgr
    {
        CONST_VTBL struct ITfDisplayAttributeMgrVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ITfDisplayAttributeMgr_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ITfDisplayAttributeMgr_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ITfDisplayAttributeMgr_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ITfDisplayAttributeMgr_OnUpdateInfo(This)	\
    ( (This)->lpVtbl -> OnUpdateInfo(This) ) 

#define ITfDisplayAttributeMgr_EnumDisplayAttributeInfo(This,ppEnum)	\
    ( (This)->lpVtbl -> EnumDisplayAttributeInfo(This,ppEnum) ) 

#define ITfDisplayAttributeMgr_GetDisplayAttributeInfo(This,guid,ppInfo,pclsidOwner)	\
    ( (This)->lpVtbl -> GetDisplayAttributeInfo(This,guid,ppInfo,pclsidOwner) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ITfDisplayAttributeMgr_INTERFACE_DEFINED__ */


#ifndef __ITfDisplayAttributeNotifySink_INTERFACE_DEFINED__
#define __ITfDisplayAttributeNotifySink_INTERFACE_DEFINED__

/* interface ITfDisplayAttributeNotifySink */
/* [unique][uuid][object] */ 


EXTERN_C const IID IID_ITfDisplayAttributeNotifySink;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("ad56f402-e162-4f25-908f-7d577cf9bda9")
    ITfDisplayAttributeNotifySink : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE OnUpdateInfo( void) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ITfDisplayAttributeNotifySinkVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in ITfDisplayAttributeNotifySink * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in ITfDisplayAttributeNotifySink * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in ITfDisplayAttributeNotifySink * This);
        
        DECLSPEC_XFGVIRT(ITfDisplayAttributeNotifySink, OnUpdateInfo)
        HRESULT ( STDMETHODCALLTYPE *OnUpdateInfo )( 
            __RPC__in ITfDisplayAttributeNotifySink * This);
        
        END_INTERFACE
    } ITfDisplayAttributeNotifySinkVtbl;

    interface ITfDisplayAttributeNotifySink
    {
        CONST_VTBL struct ITfDisplayAttributeNotifySinkVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ITfDisplayAttributeNotifySink_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ITfDisplayAttributeNotifySink_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ITfDisplayAttributeNotifySink_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ITfDisplayAttributeNotifySink_OnUpdateInfo(This)	\
    ( (This)->lpVtbl -> OnUpdateInfo(This) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ITfDisplayAttributeNotifySink_INTERFACE_DEFINED__ */


#ifndef __ITfCategoryMgr_INTERFACE_DEFINED__
#define __ITfCategoryMgr_INTERFACE_DEFINED__

/* interface ITfCategoryMgr */
/* [unique][uuid][local][object] */ 


EXTERN_C const IID IID_ITfCategoryMgr;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("c3acefb5-f69d-4905-938f-fcadcf4be830")
    ITfCategoryMgr : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE RegisterCategory( 
            /* [in] */ REFCLSID rclsid,
            /* [in] */ REFGUID rcatid,
            /* [in] */ REFGUID rguid) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE UnregisterCategory( 
            /* [in] */ REFCLSID rclsid,
            /* [in] */ REFGUID rcatid,
            /* [in] */ REFGUID rguid) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE EnumCategoriesInItem( 
            /* [in] */ REFGUID rguid,
            /* [out] */ IEnumGUID **ppEnum) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE EnumItemsInCategory( 
            /* [in] */ REFGUID rcatid,
            /* [out] */ IEnumGUID **ppEnum) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE FindClosestCategory( 
            /* [in] */ REFGUID rguid,
            /* [out] */ GUID *pcatid,
            /* [size_is][in] */ const GUID **ppcatidList,
            /* [in] */ ULONG ulCount) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE RegisterGUIDDescription( 
            /* [in] */ REFCLSID rclsid,
            /* [in] */ REFGUID rguid,
            /* [size_is][in] */ const WCHAR *pchDesc,
            /* [in] */ ULONG cch) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE UnregisterGUIDDescription( 
            /* [in] */ REFCLSID rclsid,
            /* [in] */ REFGUID rguid) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetGUIDDescription( 
            /* [in] */ REFGUID rguid,
            /* [out] */ BSTR *pbstrDesc) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE RegisterGUIDDWORD( 
            /* [in] */ REFCLSID rclsid,
            /* [in] */ REFGUID rguid,
            /* [in] */ DWORD dw) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE UnregisterGUIDDWORD( 
            /* [in] */ REFCLSID rclsid,
            /* [in] */ REFGUID rguid) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetGUIDDWORD( 
            /* [in] */ REFGUID rguid,
            /* [out] */ DWORD *pdw) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE RegisterGUID( 
            /* [in] */ REFGUID rguid,
            /* [out] */ TfGuidAtom *pguidatom) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetGUID( 
            /* [in] */ TfGuidAtom guidatom,
            /* [out] */ GUID *pguid) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE IsEqualTfGuidAtom( 
            /* [in] */ TfGuidAtom guidatom,
            /* [in] */ REFGUID rguid,
            /* [out] */ BOOL *pfEqual) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ITfCategoryMgrVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            ITfCategoryMgr * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            ITfCategoryMgr * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            ITfCategoryMgr * This);
        
        DECLSPEC_XFGVIRT(ITfCategoryMgr, RegisterCategory)
        HRESULT ( STDMETHODCALLTYPE *RegisterCategory )( 
            ITfCategoryMgr * This,
            /* [in] */ REFCLSID rclsid,
            /* [in] */ REFGUID rcatid,
            /* [in] */ REFGUID rguid);
        
        DECLSPEC_XFGVIRT(ITfCategoryMgr, UnregisterCategory)
        HRESULT ( STDMETHODCALLTYPE *UnregisterCategory )( 
            ITfCategoryMgr * This,
            /* [in] */ REFCLSID rclsid,
            /* [in] */ REFGUID rcatid,
            /* [in] */ REFGUID rguid);
        
        DECLSPEC_XFGVIRT(ITfCategoryMgr, EnumCategoriesInItem)
        HRESULT ( STDMETHODCALLTYPE *EnumCategoriesInItem )( 
            ITfCategoryMgr * This,
            /* [in] */ REFGUID rguid,
            /* [out] */ IEnumGUID **ppEnum);
        
        DECLSPEC_XFGVIRT(ITfCategoryMgr, EnumItemsInCategory)
        HRESULT ( STDMETHODCALLTYPE *EnumItemsInCategory )( 
            ITfCategoryMgr * This,
            /* [in] */ REFGUID rcatid,
            /* [out] */ IEnumGUID **ppEnum);
        
        DECLSPEC_XFGVIRT(ITfCategoryMgr, FindClosestCategory)
        HRESULT ( STDMETHODCALLTYPE *FindClosestCategory )( 
            ITfCategoryMgr * This,
            /* [in] */ REFGUID rguid,
            /* [out] */ GUID *pcatid,
            /* [size_is][in] */ const GUID **ppcatidList,
            /* [in] */ ULONG ulCount);
        
        DECLSPEC_XFGVIRT(ITfCategoryMgr, RegisterGUIDDescription)
        HRESULT ( STDMETHODCALLTYPE *RegisterGUIDDescription )( 
            ITfCategoryMgr * This,
            /* [in] */ REFCLSID rclsid,
            /* [in] */ REFGUID rguid,
            /* [size_is][in] */ const WCHAR *pchDesc,
            /* [in] */ ULONG cch);
        
        DECLSPEC_XFGVIRT(ITfCategoryMgr, UnregisterGUIDDescription)
        HRESULT ( STDMETHODCALLTYPE *UnregisterGUIDDescription )( 
            ITfCategoryMgr * This,
            /* [in] */ REFCLSID rclsid,
            /* [in] */ REFGUID rguid);
        
        DECLSPEC_XFGVIRT(ITfCategoryMgr, GetGUIDDescription)
        HRESULT ( STDMETHODCALLTYPE *GetGUIDDescription )( 
            ITfCategoryMgr * This,
            /* [in] */ REFGUID rguid,
            /* [out] */ BSTR *pbstrDesc);
        
        DECLSPEC_XFGVIRT(ITfCategoryMgr, RegisterGUIDDWORD)
        HRESULT ( STDMETHODCALLTYPE *RegisterGUIDDWORD )( 
            ITfCategoryMgr * This,
            /* [in] */ REFCLSID rclsid,
            /* [in] */ REFGUID rguid,
            /* [in] */ DWORD dw);
        
        DECLSPEC_XFGVIRT(ITfCategoryMgr, UnregisterGUIDDWORD)
        HRESULT ( STDMETHODCALLTYPE *UnregisterGUIDDWORD )( 
            ITfCategoryMgr * This,
            /* [in] */ REFCLSID rclsid,
            /* [in] */ REFGUID rguid);
        
        DECLSPEC_XFGVIRT(ITfCategoryMgr, GetGUIDDWORD)
        HRESULT ( STDMETHODCALLTYPE *GetGUIDDWORD )( 
            ITfCategoryMgr * This,
            /* [in] */ REFGUID rguid,
            /* [out] */ DWORD *pdw);
        
        DECLSPEC_XFGVIRT(ITfCategoryMgr, RegisterGUID)
        HRESULT ( STDMETHODCALLTYPE *RegisterGUID )( 
            ITfCategoryMgr * This,
            /* [in] */ REFGUID rguid,
            /* [out] */ TfGuidAtom *pguidatom);
        
        DECLSPEC_XFGVIRT(ITfCategoryMgr, GetGUID)
        HRESULT ( STDMETHODCALLTYPE *GetGUID )( 
            ITfCategoryMgr * This,
            /* [in] */ TfGuidAtom guidatom,
            /* [out] */ GUID *pguid);
        
        DECLSPEC_XFGVIRT(ITfCategoryMgr, IsEqualTfGuidAtom)
        HRESULT ( STDMETHODCALLTYPE *IsEqualTfGuidAtom )( 
            ITfCategoryMgr * This,
            /* [in] */ TfGuidAtom guidatom,
            /* [in] */ REFGUID rguid,
            /* [out] */ BOOL *pfEqual);
        
        END_INTERFACE
    } ITfCategoryMgrVtbl;

    interface ITfCategoryMgr
    {
        CONST_VTBL struct ITfCategoryMgrVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ITfCategoryMgr_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ITfCategoryMgr_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ITfCategoryMgr_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ITfCategoryMgr_RegisterCategory(This,rclsid,rcatid,rguid)	\
    ( (This)->lpVtbl -> RegisterCategory(This,rclsid,rcatid,rguid) ) 

#define ITfCategoryMgr_UnregisterCategory(This,rclsid,rcatid,rguid)	\
    ( (This)->lpVtbl -> UnregisterCategory(This,rclsid,rcatid,rguid) ) 

#define ITfCategoryMgr_EnumCategoriesInItem(This,rguid,ppEnum)	\
    ( (This)->lpVtbl -> EnumCategoriesInItem(This,rguid,ppEnum) ) 

#define ITfCategoryMgr_EnumItemsInCategory(This,rcatid,ppEnum)	\
    ( (This)->lpVtbl -> EnumItemsInCategory(This,rcatid,ppEnum) ) 

#define ITfCategoryMgr_FindClosestCategory(This,rguid,pcatid,ppcatidList,ulCount)	\
    ( (This)->lpVtbl -> FindClosestCategory(This,rguid,pcatid,ppcatidList,ulCount) ) 

#define ITfCategoryMgr_RegisterGUIDDescription(This,rclsid,rguid,pchDesc,cch)	\
    ( (This)->lpVtbl -> RegisterGUIDDescription(This,rclsid,rguid,pchDesc,cch) ) 

#define ITfCategoryMgr_UnregisterGUIDDescription(This,rclsid,rguid)	\
    ( (This)->lpVtbl -> UnregisterGUIDDescription(This,rclsid,rguid) ) 

#define ITfCategoryMgr_GetGUIDDescription(This,rguid,pbstrDesc)	\
    ( (This)->lpVtbl -> GetGUIDDescription(This,rguid,pbstrDesc) ) 

#define ITfCategoryMgr_RegisterGUIDDWORD(This,rclsid,rguid,dw)	\
    ( (This)->lpVtbl -> RegisterGUIDDWORD(This,rclsid,rguid,dw) ) 

#define ITfCategoryMgr_UnregisterGUIDDWORD(This,rclsid,rguid)	\
    ( (This)->lpVtbl -> UnregisterGUIDDWORD(This,rclsid,rguid) ) 

#define ITfCategoryMgr_GetGUIDDWORD(This,rguid,pdw)	\
    ( (This)->lpVtbl -> GetGUIDDWORD(This,rguid,pdw) ) 

#define ITfCategoryMgr_RegisterGUID(This,rguid,pguidatom)	\
    ( (This)->lpVtbl -> RegisterGUID(This,rguid,pguidatom) ) 

#define ITfCategoryMgr_GetGUID(This,guidatom,pguid)	\
    ( (This)->lpVtbl -> GetGUID(This,guidatom,pguid) ) 

#define ITfCategoryMgr_IsEqualTfGuidAtom(This,guidatom,rguid,pfEqual)	\
    ( (This)->lpVtbl -> IsEqualTfGuidAtom(This,guidatom,rguid,pfEqual) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ITfCategoryMgr_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_msctf_0000_0076 */
/* [local] */ 

#define	TF_INVALID_COOKIE	( 0xffffffff )



extern RPC_IF_HANDLE __MIDL_itf_msctf_0000_0076_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_msctf_0000_0076_v0_0_s_ifspec;

#ifndef __ITfSource_INTERFACE_DEFINED__
#define __ITfSource_INTERFACE_DEFINED__

/* interface ITfSource */
/* [unique][uuid][object] */ 


EXTERN_C const IID IID_ITfSource;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("4ea48a35-60ae-446f-8fd6-e6a8d82459f7")
    ITfSource : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE AdviseSink( 
            /* [in] */ __RPC__in REFIID riid,
            /* [iid_is][in] */ __RPC__in_opt IUnknown *punk,
            /* [out] */ __RPC__out DWORD *pdwCookie) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE UnadviseSink( 
            /* [in] */ DWORD dwCookie) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ITfSourceVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in ITfSource * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in ITfSource * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in ITfSource * This);
        
        DECLSPEC_XFGVIRT(ITfSource, AdviseSink)
        HRESULT ( STDMETHODCALLTYPE *AdviseSink )( 
            __RPC__in ITfSource * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [iid_is][in] */ __RPC__in_opt IUnknown *punk,
            /* [out] */ __RPC__out DWORD *pdwCookie);
        
        DECLSPEC_XFGVIRT(ITfSource, UnadviseSink)
        HRESULT ( STDMETHODCALLTYPE *UnadviseSink )( 
            __RPC__in ITfSource * This,
            /* [in] */ DWORD dwCookie);
        
        END_INTERFACE
    } ITfSourceVtbl;

    interface ITfSource
    {
        CONST_VTBL struct ITfSourceVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ITfSource_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ITfSource_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ITfSource_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ITfSource_AdviseSink(This,riid,punk,pdwCookie)	\
    ( (This)->lpVtbl -> AdviseSink(This,riid,punk,pdwCookie) ) 

#define ITfSource_UnadviseSink(This,dwCookie)	\
    ( (This)->lpVtbl -> UnadviseSink(This,dwCookie) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ITfSource_INTERFACE_DEFINED__ */


#ifndef __ITfSourceSingle_INTERFACE_DEFINED__
#define __ITfSourceSingle_INTERFACE_DEFINED__

/* interface ITfSourceSingle */
/* [unique][uuid][object] */ 


EXTERN_C const IID IID_ITfSourceSingle;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("73131f9c-56a9-49dd-b0ee-d046633f7528")
    ITfSourceSingle : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE AdviseSingleSink( 
            /* [in] */ TfClientId tid,
            /* [in] */ __RPC__in REFIID riid,
            /* [iid_is][in] */ __RPC__in_opt IUnknown *punk) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE UnadviseSingleSink( 
            /* [in] */ TfClientId tid,
            /* [in] */ __RPC__in REFIID riid) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ITfSourceSingleVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in ITfSourceSingle * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in ITfSourceSingle * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in ITfSourceSingle * This);
        
        DECLSPEC_XFGVIRT(ITfSourceSingle, AdviseSingleSink)
        HRESULT ( STDMETHODCALLTYPE *AdviseSingleSink )( 
            __RPC__in ITfSourceSingle * This,
            /* [in] */ TfClientId tid,
            /* [in] */ __RPC__in REFIID riid,
            /* [iid_is][in] */ __RPC__in_opt IUnknown *punk);
        
        DECLSPEC_XFGVIRT(ITfSourceSingle, UnadviseSingleSink)
        HRESULT ( STDMETHODCALLTYPE *UnadviseSingleSink )( 
            __RPC__in ITfSourceSingle * This,
            /* [in] */ TfClientId tid,
            /* [in] */ __RPC__in REFIID riid);
        
        END_INTERFACE
    } ITfSourceSingleVtbl;

    interface ITfSourceSingle
    {
        CONST_VTBL struct ITfSourceSingleVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ITfSourceSingle_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ITfSourceSingle_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ITfSourceSingle_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ITfSourceSingle_AdviseSingleSink(This,tid,riid,punk)	\
    ( (This)->lpVtbl -> AdviseSingleSink(This,tid,riid,punk) ) 

#define ITfSourceSingle_UnadviseSingleSink(This,tid,riid)	\
    ( (This)->lpVtbl -> UnadviseSingleSink(This,tid,riid) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ITfSourceSingle_INTERFACE_DEFINED__ */


#ifndef __ITfUIElementMgr_INTERFACE_DEFINED__
#define __ITfUIElementMgr_INTERFACE_DEFINED__

/* interface ITfUIElementMgr */
/* [unique][uuid][local][object] */ 


EXTERN_C const IID IID_ITfUIElementMgr;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("ea1ea135-19df-11d7-a6d2-00065b84435c")
    ITfUIElementMgr : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE BeginUIElement( 
            /* [in] */ ITfUIElement *pElement,
            /* [out][in] */ BOOL *pbShow,
            /* [out] */ DWORD *pdwUIElementId) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE UpdateUIElement( 
            /* [in] */ DWORD dwUIElementId) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE EndUIElement( 
            /* [in] */ DWORD dwUIElementId) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetUIElement( 
            /* [in] */ DWORD dwUIELementId,
            /* [out] */ ITfUIElement **ppElement) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE EnumUIElements( 
            /* [out] */ IEnumTfUIElements **ppEnum) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ITfUIElementMgrVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            ITfUIElementMgr * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            ITfUIElementMgr * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            ITfUIElementMgr * This);
        
        DECLSPEC_XFGVIRT(ITfUIElementMgr, BeginUIElement)
        HRESULT ( STDMETHODCALLTYPE *BeginUIElement )( 
            ITfUIElementMgr * This,
            /* [in] */ ITfUIElement *pElement,
            /* [out][in] */ BOOL *pbShow,
            /* [out] */ DWORD *pdwUIElementId);
        
        DECLSPEC_XFGVIRT(ITfUIElementMgr, UpdateUIElement)
        HRESULT ( STDMETHODCALLTYPE *UpdateUIElement )( 
            ITfUIElementMgr * This,
            /* [in] */ DWORD dwUIElementId);
        
        DECLSPEC_XFGVIRT(ITfUIElementMgr, EndUIElement)
        HRESULT ( STDMETHODCALLTYPE *EndUIElement )( 
            ITfUIElementMgr * This,
            /* [in] */ DWORD dwUIElementId);
        
        DECLSPEC_XFGVIRT(ITfUIElementMgr, GetUIElement)
        HRESULT ( STDMETHODCALLTYPE *GetUIElement )( 
            ITfUIElementMgr * This,
            /* [in] */ DWORD dwUIELementId,
            /* [out] */ ITfUIElement **ppElement);
        
        DECLSPEC_XFGVIRT(ITfUIElementMgr, EnumUIElements)
        HRESULT ( STDMETHODCALLTYPE *EnumUIElements )( 
            ITfUIElementMgr * This,
            /* [out] */ IEnumTfUIElements **ppEnum);
        
        END_INTERFACE
    } ITfUIElementMgrVtbl;

    interface ITfUIElementMgr
    {
        CONST_VTBL struct ITfUIElementMgrVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ITfUIElementMgr_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ITfUIElementMgr_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ITfUIElementMgr_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ITfUIElementMgr_BeginUIElement(This,pElement,pbShow,pdwUIElementId)	\
    ( (This)->lpVtbl -> BeginUIElement(This,pElement,pbShow,pdwUIElementId) ) 

#define ITfUIElementMgr_UpdateUIElement(This,dwUIElementId)	\
    ( (This)->lpVtbl -> UpdateUIElement(This,dwUIElementId) ) 

#define ITfUIElementMgr_EndUIElement(This,dwUIElementId)	\
    ( (This)->lpVtbl -> EndUIElement(This,dwUIElementId) ) 

#define ITfUIElementMgr_GetUIElement(This,dwUIELementId,ppElement)	\
    ( (This)->lpVtbl -> GetUIElement(This,dwUIELementId,ppElement) ) 

#define ITfUIElementMgr_EnumUIElements(This,ppEnum)	\
    ( (This)->lpVtbl -> EnumUIElements(This,ppEnum) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ITfUIElementMgr_INTERFACE_DEFINED__ */


#ifndef __IEnumTfUIElements_INTERFACE_DEFINED__
#define __IEnumTfUIElements_INTERFACE_DEFINED__

/* interface IEnumTfUIElements */
/* [unique][uuid][local][object] */ 


EXTERN_C const IID IID_IEnumTfUIElements;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("887aa91e-acba-4931-84da-3c5208cf543f")
    IEnumTfUIElements : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE Clone( 
            /* [out] */ IEnumTfUIElements **ppEnum) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Next( 
            /* [in] */ ULONG ulCount,
            /* [length_is][size_is][out] */ ITfUIElement **ppElement,
            /* [out] */ ULONG *pcFetched) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Reset( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Skip( 
            /* [in] */ ULONG ulCount) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IEnumTfUIElementsVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IEnumTfUIElements * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IEnumTfUIElements * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IEnumTfUIElements * This);
        
        DECLSPEC_XFGVIRT(IEnumTfUIElements, Clone)
        HRESULT ( STDMETHODCALLTYPE *Clone )( 
            IEnumTfUIElements * This,
            /* [out] */ IEnumTfUIElements **ppEnum);
        
        DECLSPEC_XFGVIRT(IEnumTfUIElements, Next)
        HRESULT ( STDMETHODCALLTYPE *Next )( 
            IEnumTfUIElements * This,
            /* [in] */ ULONG ulCount,
            /* [length_is][size_is][out] */ ITfUIElement **ppElement,
            /* [out] */ ULONG *pcFetched);
        
        DECLSPEC_XFGVIRT(IEnumTfUIElements, Reset)
        HRESULT ( STDMETHODCALLTYPE *Reset )( 
            IEnumTfUIElements * This);
        
        DECLSPEC_XFGVIRT(IEnumTfUIElements, Skip)
        HRESULT ( STDMETHODCALLTYPE *Skip )( 
            IEnumTfUIElements * This,
            /* [in] */ ULONG ulCount);
        
        END_INTERFACE
    } IEnumTfUIElementsVtbl;

    interface IEnumTfUIElements
    {
        CONST_VTBL struct IEnumTfUIElementsVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IEnumTfUIElements_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IEnumTfUIElements_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IEnumTfUIElements_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IEnumTfUIElements_Clone(This,ppEnum)	\
    ( (This)->lpVtbl -> Clone(This,ppEnum) ) 

#define IEnumTfUIElements_Next(This,ulCount,ppElement,pcFetched)	\
    ( (This)->lpVtbl -> Next(This,ulCount,ppElement,pcFetched) ) 

#define IEnumTfUIElements_Reset(This)	\
    ( (This)->lpVtbl -> Reset(This) ) 

#define IEnumTfUIElements_Skip(This,ulCount)	\
    ( (This)->lpVtbl -> Skip(This,ulCount) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IEnumTfUIElements_INTERFACE_DEFINED__ */


#ifndef __ITfUIElementSink_INTERFACE_DEFINED__
#define __ITfUIElementSink_INTERFACE_DEFINED__

/* interface ITfUIElementSink */
/* [unique][uuid][local][object] */ 


EXTERN_C const IID IID_ITfUIElementSink;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("ea1ea136-19df-11d7-a6d2-00065b84435c")
    ITfUIElementSink : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE BeginUIElement( 
            /* [in] */ DWORD dwUIElementId,
            /* [out][in] */ BOOL *pbShow) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE UpdateUIElement( 
            /* [in] */ DWORD dwUIElementId) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE EndUIElement( 
            /* [in] */ DWORD dwUIElementId) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ITfUIElementSinkVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            ITfUIElementSink * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            ITfUIElementSink * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            ITfUIElementSink * This);
        
        DECLSPEC_XFGVIRT(ITfUIElementSink, BeginUIElement)
        HRESULT ( STDMETHODCALLTYPE *BeginUIElement )( 
            ITfUIElementSink * This,
            /* [in] */ DWORD dwUIElementId,
            /* [out][in] */ BOOL *pbShow);
        
        DECLSPEC_XFGVIRT(ITfUIElementSink, UpdateUIElement)
        HRESULT ( STDMETHODCALLTYPE *UpdateUIElement )( 
            ITfUIElementSink * This,
            /* [in] */ DWORD dwUIElementId);
        
        DECLSPEC_XFGVIRT(ITfUIElementSink, EndUIElement)
        HRESULT ( STDMETHODCALLTYPE *EndUIElement )( 
            ITfUIElementSink * This,
            /* [in] */ DWORD dwUIElementId);
        
        END_INTERFACE
    } ITfUIElementSinkVtbl;

    interface ITfUIElementSink
    {
        CONST_VTBL struct ITfUIElementSinkVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ITfUIElementSink_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ITfUIElementSink_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ITfUIElementSink_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ITfUIElementSink_BeginUIElement(This,dwUIElementId,pbShow)	\
    ( (This)->lpVtbl -> BeginUIElement(This,dwUIElementId,pbShow) ) 

#define ITfUIElementSink_UpdateUIElement(This,dwUIElementId)	\
    ( (This)->lpVtbl -> UpdateUIElement(This,dwUIElementId) ) 

#define ITfUIElementSink_EndUIElement(This,dwUIElementId)	\
    ( (This)->lpVtbl -> EndUIElement(This,dwUIElementId) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ITfUIElementSink_INTERFACE_DEFINED__ */


#ifndef __ITfUIElement_INTERFACE_DEFINED__
#define __ITfUIElement_INTERFACE_DEFINED__

/* interface ITfUIElement */
/* [unique][uuid][local][object] */ 


EXTERN_C const IID IID_ITfUIElement;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("ea1ea137-19df-11d7-a6d2-00065b84435c")
    ITfUIElement : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetDescription( 
            /* [out] */ BSTR *pbstrDescription) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetGUID( 
            /* [out] */ GUID *pguid) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Show( 
            /* [in] */ BOOL bShow) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE IsShown( 
            /* [out] */ BOOL *pbShow) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ITfUIElementVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            ITfUIElement * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            ITfUIElement * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            ITfUIElement * This);
        
        DECLSPEC_XFGVIRT(ITfUIElement, GetDescription)
        HRESULT ( STDMETHODCALLTYPE *GetDescription )( 
            ITfUIElement * This,
            /* [out] */ BSTR *pbstrDescription);
        
        DECLSPEC_XFGVIRT(ITfUIElement, GetGUID)
        HRESULT ( STDMETHODCALLTYPE *GetGUID )( 
            ITfUIElement * This,
            /* [out] */ GUID *pguid);
        
        DECLSPEC_XFGVIRT(ITfUIElement, Show)
        HRESULT ( STDMETHODCALLTYPE *Show )( 
            ITfUIElement * This,
            /* [in] */ BOOL bShow);
        
        DECLSPEC_XFGVIRT(ITfUIElement, IsShown)
        HRESULT ( STDMETHODCALLTYPE *IsShown )( 
            ITfUIElement * This,
            /* [out] */ BOOL *pbShow);
        
        END_INTERFACE
    } ITfUIElementVtbl;

    interface ITfUIElement
    {
        CONST_VTBL struct ITfUIElementVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ITfUIElement_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ITfUIElement_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ITfUIElement_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ITfUIElement_GetDescription(This,pbstrDescription)	\
    ( (This)->lpVtbl -> GetDescription(This,pbstrDescription) ) 

#define ITfUIElement_GetGUID(This,pguid)	\
    ( (This)->lpVtbl -> GetGUID(This,pguid) ) 

#define ITfUIElement_Show(This,bShow)	\
    ( (This)->lpVtbl -> Show(This,bShow) ) 

#define ITfUIElement_IsShown(This,pbShow)	\
    ( (This)->lpVtbl -> IsShown(This,pbShow) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ITfUIElement_INTERFACE_DEFINED__ */


#ifndef __ITfCandidateListUIElement_INTERFACE_DEFINED__
#define __ITfCandidateListUIElement_INTERFACE_DEFINED__

/* interface ITfCandidateListUIElement */
/* [unique][uuid][local][object] */ 


EXTERN_C const IID IID_ITfCandidateListUIElement;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("ea1ea138-19df-11d7-a6d2-00065b84435c")
    ITfCandidateListUIElement : public ITfUIElement
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetUpdatedFlags( 
            /* [out] */ DWORD *pdwFlags) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetDocumentMgr( 
            /* [out] */ ITfDocumentMgr **ppdim) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetCount( 
            /* [out] */ UINT *puCount) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetSelection( 
            /* [out] */ UINT *puIndex) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetString( 
            /* [in] */ UINT uIndex,
            /* [out] */ BSTR *pstr) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetPageIndex( 
            /* [length_is][size_is][out] */ UINT *pIndex,
            /* [in] */ UINT uSize,
            /* [out] */ UINT *puPageCnt) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetPageIndex( 
            /* [size_is][in] */ UINT *pIndex,
            /* [in] */ UINT uPageCnt) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetCurrentPage( 
            /* [out] */ UINT *puPage) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ITfCandidateListUIElementVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            ITfCandidateListUIElement * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            ITfCandidateListUIElement * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            ITfCandidateListUIElement * This);
        
        DECLSPEC_XFGVIRT(ITfUIElement, GetDescription)
        HRESULT ( STDMETHODCALLTYPE *GetDescription )( 
            ITfCandidateListUIElement * This,
            /* [out] */ BSTR *pbstrDescription);
        
        DECLSPEC_XFGVIRT(ITfUIElement, GetGUID)
        HRESULT ( STDMETHODCALLTYPE *GetGUID )( 
            ITfCandidateListUIElement * This,
            /* [out] */ GUID *pguid);
        
        DECLSPEC_XFGVIRT(ITfUIElement, Show)
        HRESULT ( STDMETHODCALLTYPE *Show )( 
            ITfCandidateListUIElement * This,
            /* [in] */ BOOL bShow);
        
        DECLSPEC_XFGVIRT(ITfUIElement, IsShown)
        HRESULT ( STDMETHODCALLTYPE *IsShown )( 
            ITfCandidateListUIElement * This,
            /* [out] */ BOOL *pbShow);
        
        DECLSPEC_XFGVIRT(ITfCandidateListUIElement, GetUpdatedFlags)
        HRESULT ( STDMETHODCALLTYPE *GetUpdatedFlags )( 
            ITfCandidateListUIElement * This,
            /* [out] */ DWORD *pdwFlags);
        
        DECLSPEC_XFGVIRT(ITfCandidateListUIElement, GetDocumentMgr)
        HRESULT ( STDMETHODCALLTYPE *GetDocumentMgr )( 
            ITfCandidateListUIElement * This,
            /* [out] */ ITfDocumentMgr **ppdim);
        
        DECLSPEC_XFGVIRT(ITfCandidateListUIElement, GetCount)
        HRESULT ( STDMETHODCALLTYPE *GetCount )( 
            ITfCandidateListUIElement * This,
            /* [out] */ UINT *puCount);
        
        DECLSPEC_XFGVIRT(ITfCandidateListUIElement, GetSelection)
        HRESULT ( STDMETHODCALLTYPE *GetSelection )( 
            ITfCandidateListUIElement * This,
            /* [out] */ UINT *puIndex);
        
        DECLSPEC_XFGVIRT(ITfCandidateListUIElement, GetString)
        HRESULT ( STDMETHODCALLTYPE *GetString )( 
            ITfCandidateListUIElement * This,
            /* [in] */ UINT uIndex,
            /* [out] */ BSTR *pstr);
        
        DECLSPEC_XFGVIRT(ITfCandidateListUIElement, GetPageIndex)
        HRESULT ( STDMETHODCALLTYPE *GetPageIndex )( 
            ITfCandidateListUIElement * This,
            /* [length_is][size_is][out] */ UINT *pIndex,
            /* [in] */ UINT uSize,
            /* [out] */ UINT *puPageCnt);
        
        DECLSPEC_XFGVIRT(ITfCandidateListUIElement, SetPageIndex)
        HRESULT ( STDMETHODCALLTYPE *SetPageIndex )( 
            ITfCandidateListUIElement * This,
            /* [size_is][in] */ UINT *pIndex,
            /* [in] */ UINT uPageCnt);
        
        DECLSPEC_XFGVIRT(ITfCandidateListUIElement, GetCurrentPage)
        HRESULT ( STDMETHODCALLTYPE *GetCurrentPage )( 
            ITfCandidateListUIElement * This,
            /* [out] */ UINT *puPage);
        
        END_INTERFACE
    } ITfCandidateListUIElementVtbl;

    interface ITfCandidateListUIElement
    {
        CONST_VTBL struct ITfCandidateListUIElementVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ITfCandidateListUIElement_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ITfCandidateListUIElement_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ITfCandidateListUIElement_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ITfCandidateListUIElement_GetDescription(This,pbstrDescription)	\
    ( (This)->lpVtbl -> GetDescription(This,pbstrDescription) ) 

#define ITfCandidateListUIElement_GetGUID(This,pguid)	\
    ( (This)->lpVtbl -> GetGUID(This,pguid) ) 

#define ITfCandidateListUIElement_Show(This,bShow)	\
    ( (This)->lpVtbl -> Show(This,bShow) ) 

#define ITfCandidateListUIElement_IsShown(This,pbShow)	\
    ( (This)->lpVtbl -> IsShown(This,pbShow) ) 


#define ITfCandidateListUIElement_GetUpdatedFlags(This,pdwFlags)	\
    ( (This)->lpVtbl -> GetUpdatedFlags(This,pdwFlags) ) 

#define ITfCandidateListUIElement_GetDocumentMgr(This,ppdim)	\
    ( (This)->lpVtbl -> GetDocumentMgr(This,ppdim) ) 

#define ITfCandidateListUIElement_GetCount(This,puCount)	\
    ( (This)->lpVtbl -> GetCount(This,puCount) ) 

#define ITfCandidateListUIElement_GetSelection(This,puIndex)	\
    ( (This)->lpVtbl -> GetSelection(This,puIndex) ) 

#define ITfCandidateListUIElement_GetString(This,uIndex,pstr)	\
    ( (This)->lpVtbl -> GetString(This,uIndex,pstr) ) 

#define ITfCandidateListUIElement_GetPageIndex(This,pIndex,uSize,puPageCnt)	\
    ( (This)->lpVtbl -> GetPageIndex(This,pIndex,uSize,puPageCnt) ) 

#define ITfCandidateListUIElement_SetPageIndex(This,pIndex,uPageCnt)	\
    ( (This)->lpVtbl -> SetPageIndex(This,pIndex,uPageCnt) ) 

#define ITfCandidateListUIElement_GetCurrentPage(This,puPage)	\
    ( (This)->lpVtbl -> GetCurrentPage(This,puPage) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ITfCandidateListUIElement_INTERFACE_DEFINED__ */


#ifndef __ITfCandidateListUIElementBehavior_INTERFACE_DEFINED__
#define __ITfCandidateListUIElementBehavior_INTERFACE_DEFINED__

/* interface ITfCandidateListUIElementBehavior */
/* [unique][uuid][local][object] */ 


EXTERN_C const IID IID_ITfCandidateListUIElementBehavior;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("85fad185-58ce-497a-9460-355366b64b9a")
    ITfCandidateListUIElementBehavior : public ITfCandidateListUIElement
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE SetSelection( 
            /* [in] */ UINT nIndex) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Finalize( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Abort( void) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ITfCandidateListUIElementBehaviorVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            ITfCandidateListUIElementBehavior * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            ITfCandidateListUIElementBehavior * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            ITfCandidateListUIElementBehavior * This);
        
        DECLSPEC_XFGVIRT(ITfUIElement, GetDescription)
        HRESULT ( STDMETHODCALLTYPE *GetDescription )( 
            ITfCandidateListUIElementBehavior * This,
            /* [out] */ BSTR *pbstrDescription);
        
        DECLSPEC_XFGVIRT(ITfUIElement, GetGUID)
        HRESULT ( STDMETHODCALLTYPE *GetGUID )( 
            ITfCandidateListUIElementBehavior * This,
            /* [out] */ GUID *pguid);
        
        DECLSPEC_XFGVIRT(ITfUIElement, Show)
        HRESULT ( STDMETHODCALLTYPE *Show )( 
            ITfCandidateListUIElementBehavior * This,
            /* [in] */ BOOL bShow);
        
        DECLSPEC_XFGVIRT(ITfUIElement, IsShown)
        HRESULT ( STDMETHODCALLTYPE *IsShown )( 
            ITfCandidateListUIElementBehavior * This,
            /* [out] */ BOOL *pbShow);
        
        DECLSPEC_XFGVIRT(ITfCandidateListUIElement, GetUpdatedFlags)
        HRESULT ( STDMETHODCALLTYPE *GetUpdatedFlags )( 
            ITfCandidateListUIElementBehavior * This,
            /* [out] */ DWORD *pdwFlags);
        
        DECLSPEC_XFGVIRT(ITfCandidateListUIElement, GetDocumentMgr)
        HRESULT ( STDMETHODCALLTYPE *GetDocumentMgr )( 
            ITfCandidateListUIElementBehavior * This,
            /* [out] */ ITfDocumentMgr **ppdim);
        
        DECLSPEC_XFGVIRT(ITfCandidateListUIElement, GetCount)
        HRESULT ( STDMETHODCALLTYPE *GetCount )( 
            ITfCandidateListUIElementBehavior * This,
            /* [out] */ UINT *puCount);
        
        DECLSPEC_XFGVIRT(ITfCandidateListUIElement, GetSelection)
        HRESULT ( STDMETHODCALLTYPE *GetSelection )( 
            ITfCandidateListUIElementBehavior * This,
            /* [out] */ UINT *puIndex);
        
        DECLSPEC_XFGVIRT(ITfCandidateListUIElement, GetString)
        HRESULT ( STDMETHODCALLTYPE *GetString )( 
            ITfCandidateListUIElementBehavior * This,
            /* [in] */ UINT uIndex,
            /* [out] */ BSTR *pstr);
        
        DECLSPEC_XFGVIRT(ITfCandidateListUIElement, GetPageIndex)
        HRESULT ( STDMETHODCALLTYPE *GetPageIndex )( 
            ITfCandidateListUIElementBehavior * This,
            /* [length_is][size_is][out] */ UINT *pIndex,
            /* [in] */ UINT uSize,
            /* [out] */ UINT *puPageCnt);
        
        DECLSPEC_XFGVIRT(ITfCandidateListUIElement, SetPageIndex)
        HRESULT ( STDMETHODCALLTYPE *SetPageIndex )( 
            ITfCandidateListUIElementBehavior * This,
            /* [size_is][in] */ UINT *pIndex,
            /* [in] */ UINT uPageCnt);
        
        DECLSPEC_XFGVIRT(ITfCandidateListUIElement, GetCurrentPage)
        HRESULT ( STDMETHODCALLTYPE *GetCurrentPage )( 
            ITfCandidateListUIElementBehavior * This,
            /* [out] */ UINT *puPage);
        
        DECLSPEC_XFGVIRT(ITfCandidateListUIElementBehavior, SetSelection)
        HRESULT ( STDMETHODCALLTYPE *SetSelection )( 
            ITfCandidateListUIElementBehavior * This,
            /* [in] */ UINT nIndex);
        
        DECLSPEC_XFGVIRT(ITfCandidateListUIElementBehavior, Finalize)
        HRESULT ( STDMETHODCALLTYPE *Finalize )( 
            ITfCandidateListUIElementBehavior * This);
        
        DECLSPEC_XFGVIRT(ITfCandidateListUIElementBehavior, Abort)
        HRESULT ( STDMETHODCALLTYPE *Abort )( 
            ITfCandidateListUIElementBehavior * This);
        
        END_INTERFACE
    } ITfCandidateListUIElementBehaviorVtbl;

    interface ITfCandidateListUIElementBehavior
    {
        CONST_VTBL struct ITfCandidateListUIElementBehaviorVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ITfCandidateListUIElementBehavior_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ITfCandidateListUIElementBehavior_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ITfCandidateListUIElementBehavior_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ITfCandidateListUIElementBehavior_GetDescription(This,pbstrDescription)	\
    ( (This)->lpVtbl -> GetDescription(This,pbstrDescription) ) 

#define ITfCandidateListUIElementBehavior_GetGUID(This,pguid)	\
    ( (This)->lpVtbl -> GetGUID(This,pguid) ) 

#define ITfCandidateListUIElementBehavior_Show(This,bShow)	\
    ( (This)->lpVtbl -> Show(This,bShow) ) 

#define ITfCandidateListUIElementBehavior_IsShown(This,pbShow)	\
    ( (This)->lpVtbl -> IsShown(This,pbShow) ) 


#define ITfCandidateListUIElementBehavior_GetUpdatedFlags(This,pdwFlags)	\
    ( (This)->lpVtbl -> GetUpdatedFlags(This,pdwFlags) ) 

#define ITfCandidateListUIElementBehavior_GetDocumentMgr(This,ppdim)	\
    ( (This)->lpVtbl -> GetDocumentMgr(This,ppdim) ) 

#define ITfCandidateListUIElementBehavior_GetCount(This,puCount)	\
    ( (This)->lpVtbl -> GetCount(This,puCount) ) 

#define ITfCandidateListUIElementBehavior_GetSelection(This,puIndex)	\
    ( (This)->lpVtbl -> GetSelection(This,puIndex) ) 

#define ITfCandidateListUIElementBehavior_GetString(This,uIndex,pstr)	\
    ( (This)->lpVtbl -> GetString(This,uIndex,pstr) ) 

#define ITfCandidateListUIElementBehavior_GetPageIndex(This,pIndex,uSize,puPageCnt)	\
    ( (This)->lpVtbl -> GetPageIndex(This,pIndex,uSize,puPageCnt) ) 

#define ITfCandidateListUIElementBehavior_SetPageIndex(This,pIndex,uPageCnt)	\
    ( (This)->lpVtbl -> SetPageIndex(This,pIndex,uPageCnt) ) 

#define ITfCandidateListUIElementBehavior_GetCurrentPage(This,puPage)	\
    ( (This)->lpVtbl -> GetCurrentPage(This,puPage) ) 


#define ITfCandidateListUIElementBehavior_SetSelection(This,nIndex)	\
    ( (This)->lpVtbl -> SetSelection(This,nIndex) ) 

#define ITfCandidateListUIElementBehavior_Finalize(This)	\
    ( (This)->lpVtbl -> Finalize(This) ) 

#define ITfCandidateListUIElementBehavior_Abort(This)	\
    ( (This)->lpVtbl -> Abort(This) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ITfCandidateListUIElementBehavior_INTERFACE_DEFINED__ */


#ifndef __ITfReadingInformationUIElement_INTERFACE_DEFINED__
#define __ITfReadingInformationUIElement_INTERFACE_DEFINED__

/* interface ITfReadingInformationUIElement */
/* [unique][uuid][local][object] */ 


EXTERN_C const IID IID_ITfReadingInformationUIElement;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("ea1ea139-19df-11d7-a6d2-00065b84435c")
    ITfReadingInformationUIElement : public ITfUIElement
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetUpdatedFlags( 
            /* [out] */ DWORD *pdwFlags) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetContext( 
            /* [out] */ ITfContext **ppic) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetString( 
            /* [out] */ BSTR *pstr) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetMaxReadingStringLength( 
            /* [out] */ UINT *pcchMax) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetErrorIndex( 
            /* [out] */ UINT *pErrorIndex) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE IsVerticalOrderPreferred( 
            /* [out] */ BOOL *pfVertical) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ITfReadingInformationUIElementVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            ITfReadingInformationUIElement * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            ITfReadingInformationUIElement * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            ITfReadingInformationUIElement * This);
        
        DECLSPEC_XFGVIRT(ITfUIElement, GetDescription)
        HRESULT ( STDMETHODCALLTYPE *GetDescription )( 
            ITfReadingInformationUIElement * This,
            /* [out] */ BSTR *pbstrDescription);
        
        DECLSPEC_XFGVIRT(ITfUIElement, GetGUID)
        HRESULT ( STDMETHODCALLTYPE *GetGUID )( 
            ITfReadingInformationUIElement * This,
            /* [out] */ GUID *pguid);
        
        DECLSPEC_XFGVIRT(ITfUIElement, Show)
        HRESULT ( STDMETHODCALLTYPE *Show )( 
            ITfReadingInformationUIElement * This,
            /* [in] */ BOOL bShow);
        
        DECLSPEC_XFGVIRT(ITfUIElement, IsShown)
        HRESULT ( STDMETHODCALLTYPE *IsShown )( 
            ITfReadingInformationUIElement * This,
            /* [out] */ BOOL *pbShow);
        
        DECLSPEC_XFGVIRT(ITfReadingInformationUIElement, GetUpdatedFlags)
        HRESULT ( STDMETHODCALLTYPE *GetUpdatedFlags )( 
            ITfReadingInformationUIElement * This,
            /* [out] */ DWORD *pdwFlags);
        
        DECLSPEC_XFGVIRT(ITfReadingInformationUIElement, GetContext)
        HRESULT ( STDMETHODCALLTYPE *GetContext )( 
            ITfReadingInformationUIElement * This,
            /* [out] */ ITfContext **ppic);
        
        DECLSPEC_XFGVIRT(ITfReadingInformationUIElement, GetString)
        HRESULT ( STDMETHODCALLTYPE *GetString )( 
            ITfReadingInformationUIElement * This,
            /* [out] */ BSTR *pstr);
        
        DECLSPEC_XFGVIRT(ITfReadingInformationUIElement, GetMaxReadingStringLength)
        HRESULT ( STDMETHODCALLTYPE *GetMaxReadingStringLength )( 
            ITfReadingInformationUIElement * This,
            /* [out] */ UINT *pcchMax);
        
        DECLSPEC_XFGVIRT(ITfReadingInformationUIElement, GetErrorIndex)
        HRESULT ( STDMETHODCALLTYPE *GetErrorIndex )( 
            ITfReadingInformationUIElement * This,
            /* [out] */ UINT *pErrorIndex);
        
        DECLSPEC_XFGVIRT(ITfReadingInformationUIElement, IsVerticalOrderPreferred)
        HRESULT ( STDMETHODCALLTYPE *IsVerticalOrderPreferred )( 
            ITfReadingInformationUIElement * This,
            /* [out] */ BOOL *pfVertical);
        
        END_INTERFACE
    } ITfReadingInformationUIElementVtbl;

    interface ITfReadingInformationUIElement
    {
        CONST_VTBL struct ITfReadingInformationUIElementVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ITfReadingInformationUIElement_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ITfReadingInformationUIElement_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ITfReadingInformationUIElement_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ITfReadingInformationUIElement_GetDescription(This,pbstrDescription)	\
    ( (This)->lpVtbl -> GetDescription(This,pbstrDescription) ) 

#define ITfReadingInformationUIElement_GetGUID(This,pguid)	\
    ( (This)->lpVtbl -> GetGUID(This,pguid) ) 

#define ITfReadingInformationUIElement_Show(This,bShow)	\
    ( (This)->lpVtbl -> Show(This,bShow) ) 

#define ITfReadingInformationUIElement_IsShown(This,pbShow)	\
    ( (This)->lpVtbl -> IsShown(This,pbShow) ) 


#define ITfReadingInformationUIElement_GetUpdatedFlags(This,pdwFlags)	\
    ( (This)->lpVtbl -> GetUpdatedFlags(This,pdwFlags) ) 

#define ITfReadingInformationUIElement_GetContext(This,ppic)	\
    ( (This)->lpVtbl -> GetContext(This,ppic) ) 

#define ITfReadingInformationUIElement_GetString(This,pstr)	\
    ( (This)->lpVtbl -> GetString(This,pstr) ) 

#define ITfReadingInformationUIElement_GetMaxReadingStringLength(This,pcchMax)	\
    ( (This)->lpVtbl -> GetMaxReadingStringLength(This,pcchMax) ) 

#define ITfReadingInformationUIElement_GetErrorIndex(This,pErrorIndex)	\
    ( (This)->lpVtbl -> GetErrorIndex(This,pErrorIndex) ) 

#define ITfReadingInformationUIElement_IsVerticalOrderPreferred(This,pfVertical)	\
    ( (This)->lpVtbl -> IsVerticalOrderPreferred(This,pfVertical) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ITfReadingInformationUIElement_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_msctf_0000_0085 */
/* [local] */ 

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP) */
#pragma endregion
#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)


extern RPC_IF_HANDLE __MIDL_itf_msctf_0000_0085_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_msctf_0000_0085_v0_0_s_ifspec;

#ifndef __ITfTransitoryExtensionUIElement_INTERFACE_DEFINED__
#define __ITfTransitoryExtensionUIElement_INTERFACE_DEFINED__

/* interface ITfTransitoryExtensionUIElement */
/* [unique][uuid][local][object] */ 


EXTERN_C const IID IID_ITfTransitoryExtensionUIElement;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("858f956a-972f-42a2-a2f2-0321e1abe209")
    ITfTransitoryExtensionUIElement : public ITfUIElement
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetDocumentMgr( 
            /* [out] */ ITfDocumentMgr **ppdim) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ITfTransitoryExtensionUIElementVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            ITfTransitoryExtensionUIElement * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            ITfTransitoryExtensionUIElement * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            ITfTransitoryExtensionUIElement * This);
        
        DECLSPEC_XFGVIRT(ITfUIElement, GetDescription)
        HRESULT ( STDMETHODCALLTYPE *GetDescription )( 
            ITfTransitoryExtensionUIElement * This,
            /* [out] */ BSTR *pbstrDescription);
        
        DECLSPEC_XFGVIRT(ITfUIElement, GetGUID)
        HRESULT ( STDMETHODCALLTYPE *GetGUID )( 
            ITfTransitoryExtensionUIElement * This,
            /* [out] */ GUID *pguid);
        
        DECLSPEC_XFGVIRT(ITfUIElement, Show)
        HRESULT ( STDMETHODCALLTYPE *Show )( 
            ITfTransitoryExtensionUIElement * This,
            /* [in] */ BOOL bShow);
        
        DECLSPEC_XFGVIRT(ITfUIElement, IsShown)
        HRESULT ( STDMETHODCALLTYPE *IsShown )( 
            ITfTransitoryExtensionUIElement * This,
            /* [out] */ BOOL *pbShow);
        
        DECLSPEC_XFGVIRT(ITfTransitoryExtensionUIElement, GetDocumentMgr)
        HRESULT ( STDMETHODCALLTYPE *GetDocumentMgr )( 
            ITfTransitoryExtensionUIElement * This,
            /* [out] */ ITfDocumentMgr **ppdim);
        
        END_INTERFACE
    } ITfTransitoryExtensionUIElementVtbl;

    interface ITfTransitoryExtensionUIElement
    {
        CONST_VTBL struct ITfTransitoryExtensionUIElementVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ITfTransitoryExtensionUIElement_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ITfTransitoryExtensionUIElement_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ITfTransitoryExtensionUIElement_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ITfTransitoryExtensionUIElement_GetDescription(This,pbstrDescription)	\
    ( (This)->lpVtbl -> GetDescription(This,pbstrDescription) ) 

#define ITfTransitoryExtensionUIElement_GetGUID(This,pguid)	\
    ( (This)->lpVtbl -> GetGUID(This,pguid) ) 

#define ITfTransitoryExtensionUIElement_Show(This,bShow)	\
    ( (This)->lpVtbl -> Show(This,bShow) ) 

#define ITfTransitoryExtensionUIElement_IsShown(This,pbShow)	\
    ( (This)->lpVtbl -> IsShown(This,pbShow) ) 


#define ITfTransitoryExtensionUIElement_GetDocumentMgr(This,ppdim)	\
    ( (This)->lpVtbl -> GetDocumentMgr(This,ppdim) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ITfTransitoryExtensionUIElement_INTERFACE_DEFINED__ */


#ifndef __ITfTransitoryExtensionSink_INTERFACE_DEFINED__
#define __ITfTransitoryExtensionSink_INTERFACE_DEFINED__

/* interface ITfTransitoryExtensionSink */
/* [unique][uuid][local][object] */ 


EXTERN_C const IID IID_ITfTransitoryExtensionSink;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("a615096f-1c57-4813-8a15-55ee6e5a839c")
    ITfTransitoryExtensionSink : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE OnTransitoryExtensionUpdated( 
            /* [in] */ ITfContext *pic,
            /* [in] */ TfEditCookie ecReadOnly,
            /* [in] */ ITfRange *pResultRange,
            /* [in] */ ITfRange *pCompositionRange,
            /* [out] */ BOOL *pfDeleteResultRange) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ITfTransitoryExtensionSinkVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            ITfTransitoryExtensionSink * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            ITfTransitoryExtensionSink * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            ITfTransitoryExtensionSink * This);
        
        DECLSPEC_XFGVIRT(ITfTransitoryExtensionSink, OnTransitoryExtensionUpdated)
        HRESULT ( STDMETHODCALLTYPE *OnTransitoryExtensionUpdated )( 
            ITfTransitoryExtensionSink * This,
            /* [in] */ ITfContext *pic,
            /* [in] */ TfEditCookie ecReadOnly,
            /* [in] */ ITfRange *pResultRange,
            /* [in] */ ITfRange *pCompositionRange,
            /* [out] */ BOOL *pfDeleteResultRange);
        
        END_INTERFACE
    } ITfTransitoryExtensionSinkVtbl;

    interface ITfTransitoryExtensionSink
    {
        CONST_VTBL struct ITfTransitoryExtensionSinkVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ITfTransitoryExtensionSink_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ITfTransitoryExtensionSink_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ITfTransitoryExtensionSink_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ITfTransitoryExtensionSink_OnTransitoryExtensionUpdated(This,pic,ecReadOnly,pResultRange,pCompositionRange,pfDeleteResultRange)	\
    ( (This)->lpVtbl -> OnTransitoryExtensionUpdated(This,pic,ecReadOnly,pResultRange,pCompositionRange,pfDeleteResultRange) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ITfTransitoryExtensionSink_INTERFACE_DEFINED__ */


#ifndef __ITfToolTipUIElement_INTERFACE_DEFINED__
#define __ITfToolTipUIElement_INTERFACE_DEFINED__

/* interface ITfToolTipUIElement */
/* [unique][uuid][local][object] */ 


EXTERN_C const IID IID_ITfToolTipUIElement;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("52b18b5c-555d-46b2-b00a-fa680144fbdb")
    ITfToolTipUIElement : public ITfUIElement
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetString( 
            /* [out] */ BSTR *pstr) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ITfToolTipUIElementVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            ITfToolTipUIElement * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            ITfToolTipUIElement * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            ITfToolTipUIElement * This);
        
        DECLSPEC_XFGVIRT(ITfUIElement, GetDescription)
        HRESULT ( STDMETHODCALLTYPE *GetDescription )( 
            ITfToolTipUIElement * This,
            /* [out] */ BSTR *pbstrDescription);
        
        DECLSPEC_XFGVIRT(ITfUIElement, GetGUID)
        HRESULT ( STDMETHODCALLTYPE *GetGUID )( 
            ITfToolTipUIElement * This,
            /* [out] */ GUID *pguid);
        
        DECLSPEC_XFGVIRT(ITfUIElement, Show)
        HRESULT ( STDMETHODCALLTYPE *Show )( 
            ITfToolTipUIElement * This,
            /* [in] */ BOOL bShow);
        
        DECLSPEC_XFGVIRT(ITfUIElement, IsShown)
        HRESULT ( STDMETHODCALLTYPE *IsShown )( 
            ITfToolTipUIElement * This,
            /* [out] */ BOOL *pbShow);
        
        DECLSPEC_XFGVIRT(ITfToolTipUIElement, GetString)
        HRESULT ( STDMETHODCALLTYPE *GetString )( 
            ITfToolTipUIElement * This,
            /* [out] */ BSTR *pstr);
        
        END_INTERFACE
    } ITfToolTipUIElementVtbl;

    interface ITfToolTipUIElement
    {
        CONST_VTBL struct ITfToolTipUIElementVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ITfToolTipUIElement_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ITfToolTipUIElement_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ITfToolTipUIElement_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ITfToolTipUIElement_GetDescription(This,pbstrDescription)	\
    ( (This)->lpVtbl -> GetDescription(This,pbstrDescription) ) 

#define ITfToolTipUIElement_GetGUID(This,pguid)	\
    ( (This)->lpVtbl -> GetGUID(This,pguid) ) 

#define ITfToolTipUIElement_Show(This,bShow)	\
    ( (This)->lpVtbl -> Show(This,bShow) ) 

#define ITfToolTipUIElement_IsShown(This,pbShow)	\
    ( (This)->lpVtbl -> IsShown(This,pbShow) ) 


#define ITfToolTipUIElement_GetString(This,pstr)	\
    ( (This)->lpVtbl -> GetString(This,pstr) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ITfToolTipUIElement_INTERFACE_DEFINED__ */


#ifndef __ITfReverseConversionList_INTERFACE_DEFINED__
#define __ITfReverseConversionList_INTERFACE_DEFINED__

/* interface ITfReverseConversionList */
/* [unique][uuid][object] */ 


EXTERN_C const IID IID_ITfReverseConversionList;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("151d69f0-86f4-4674-b721-56911e797f47")
    ITfReverseConversionList : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetLength( 
            /* [out] */ __RPC__out UINT *puIndex) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetString( 
            /* [in] */ UINT uIndex,
            /* [out] */ __RPC__deref_out_opt BSTR *pbstr) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ITfReverseConversionListVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in ITfReverseConversionList * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in ITfReverseConversionList * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in ITfReverseConversionList * This);
        
        DECLSPEC_XFGVIRT(ITfReverseConversionList, GetLength)
        HRESULT ( STDMETHODCALLTYPE *GetLength )( 
            __RPC__in ITfReverseConversionList * This,
            /* [out] */ __RPC__out UINT *puIndex);
        
        DECLSPEC_XFGVIRT(ITfReverseConversionList, GetString)
        HRESULT ( STDMETHODCALLTYPE *GetString )( 
            __RPC__in ITfReverseConversionList * This,
            /* [in] */ UINT uIndex,
            /* [out] */ __RPC__deref_out_opt BSTR *pbstr);
        
        END_INTERFACE
    } ITfReverseConversionListVtbl;

    interface ITfReverseConversionList
    {
        CONST_VTBL struct ITfReverseConversionListVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ITfReverseConversionList_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ITfReverseConversionList_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ITfReverseConversionList_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ITfReverseConversionList_GetLength(This,puIndex)	\
    ( (This)->lpVtbl -> GetLength(This,puIndex) ) 

#define ITfReverseConversionList_GetString(This,uIndex,pbstr)	\
    ( (This)->lpVtbl -> GetString(This,uIndex,pbstr) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ITfReverseConversionList_INTERFACE_DEFINED__ */


#ifndef __ITfReverseConversion_INTERFACE_DEFINED__
#define __ITfReverseConversion_INTERFACE_DEFINED__

/* interface ITfReverseConversion */
/* [unique][uuid][object] */ 


EXTERN_C const IID IID_ITfReverseConversion;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("a415e162-157d-417d-8a8c-0ab26c7d2781")
    ITfReverseConversion : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE DoReverseConversion( 
            /* [in] */ __RPC__in LPCWSTR lpstr,
            /* [out] */ __RPC__deref_out_opt ITfReverseConversionList **ppList) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ITfReverseConversionVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in ITfReverseConversion * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in ITfReverseConversion * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in ITfReverseConversion * This);
        
        DECLSPEC_XFGVIRT(ITfReverseConversion, DoReverseConversion)
        HRESULT ( STDMETHODCALLTYPE *DoReverseConversion )( 
            __RPC__in ITfReverseConversion * This,
            /* [in] */ __RPC__in LPCWSTR lpstr,
            /* [out] */ __RPC__deref_out_opt ITfReverseConversionList **ppList);
        
        END_INTERFACE
    } ITfReverseConversionVtbl;

    interface ITfReverseConversion
    {
        CONST_VTBL struct ITfReverseConversionVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ITfReverseConversion_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ITfReverseConversion_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ITfReverseConversion_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ITfReverseConversion_DoReverseConversion(This,lpstr,ppList)	\
    ( (This)->lpVtbl -> DoReverseConversion(This,lpstr,ppList) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ITfReverseConversion_INTERFACE_DEFINED__ */


#ifndef __ITfReverseConversionMgr_INTERFACE_DEFINED__
#define __ITfReverseConversionMgr_INTERFACE_DEFINED__

/* interface ITfReverseConversionMgr */
/* [unique][uuid][object] */ 


EXTERN_C const IID IID_ITfReverseConversionMgr;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("b643c236-c493-41b6-abb3-692412775cc4")
    ITfReverseConversionMgr : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetReverseConversion( 
            /* [in] */ LANGID langid,
            /* [in] */ __RPC__in REFGUID guidProfile,
            /* [in] */ DWORD dwflag,
            /* [out] */ __RPC__deref_out_opt ITfReverseConversion **ppReverseConversion) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ITfReverseConversionMgrVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in ITfReverseConversionMgr * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in ITfReverseConversionMgr * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in ITfReverseConversionMgr * This);
        
        DECLSPEC_XFGVIRT(ITfReverseConversionMgr, GetReverseConversion)
        HRESULT ( STDMETHODCALLTYPE *GetReverseConversion )( 
            __RPC__in ITfReverseConversionMgr * This,
            /* [in] */ LANGID langid,
            /* [in] */ __RPC__in REFGUID guidProfile,
            /* [in] */ DWORD dwflag,
            /* [out] */ __RPC__deref_out_opt ITfReverseConversion **ppReverseConversion);
        
        END_INTERFACE
    } ITfReverseConversionMgrVtbl;

    interface ITfReverseConversionMgr
    {
        CONST_VTBL struct ITfReverseConversionMgrVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ITfReverseConversionMgr_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ITfReverseConversionMgr_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ITfReverseConversionMgr_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ITfReverseConversionMgr_GetReverseConversion(This,langid,guidProfile,dwflag,ppReverseConversion)	\
    ( (This)->lpVtbl -> GetReverseConversion(This,langid,guidProfile,dwflag,ppReverseConversion) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ITfReverseConversionMgr_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_msctf_0000_0091 */
/* [local] */ 


DEFINE_GUID( TF_PROFILE_NEWPHONETIC, 0xB2F9C502, 0x1742, 0x11D4, 0x97, 0x90, 0x00, 0x80, 0xC8, 0x82, 0x68, 0x7E);
DEFINE_GUID( TF_PROFILE_PHONETIC,    0x761309DE, 0x317A, 0x11D4, 0x9B, 0x5D, 0x00, 0x80, 0xC8, 0x82, 0x68, 0x7E);
DEFINE_GUID( TF_PROFILE_NEWCHANGJIE, 0xF3BA907A, 0x6C7E, 0x11D4, 0x97, 0xFA, 0x00, 0x80, 0xC8, 0x82, 0x68, 0x7E);
DEFINE_GUID( TF_PROFILE_CHANGJIE,    0x4BDF9F03, 0xC7D3, 0x11D4, 0xB2, 0xAB, 0x00, 0x80, 0xC8, 0x82, 0x68, 0x7E);
DEFINE_GUID( TF_PROFILE_NEWQUICK,    0x0B883BA0, 0xC1C7, 0x11D4, 0x87, 0xF9, 0x00, 0x80, 0xC8, 0x82, 0x68, 0x7E);
DEFINE_GUID( TF_PROFILE_QUICK,       0x6024B45F, 0x5C54, 0x11D4, 0xB9, 0x21, 0x00, 0x80, 0xC8, 0x82, 0x68, 0x7E);
DEFINE_GUID( TF_PROFILE_CANTONESE,   0x0AEC109C, 0x7E96, 0x11D4, 0xB2, 0xEF, 0x00, 0x80, 0xC8, 0x82, 0x68, 0x7E);
DEFINE_GUID( TF_PROFILE_PINYIN,      0xF3BA9077, 0x6C7E, 0x11D4, 0x97, 0xFA, 0x00, 0x80, 0xC8, 0x82, 0x68, 0x7E);
DEFINE_GUID( TF_PROFILE_SIMPLEFAST,  0xFA550B04, 0x5AD7, 0x411F, 0xA5, 0xAC, 0xCA, 0x03, 0x8E, 0xC5, 0x15, 0xD7);
DEFINE_GUID( TF_PROFILE_WUBI,        0x82590C13, 0xF4DD, 0x44F4, 0xBA, 0x1D, 0x86, 0x67, 0x24, 0x6F, 0xDF, 0x8E);
DEFINE_GUID( TF_PROFILE_DAYI,        0x037B2C25, 0x480C, 0x4D7F, 0xB0, 0x27, 0xD6, 0xCA, 0x6B, 0x69, 0x78, 0x8A);
DEFINE_GUID( TF_PROFILE_ARRAY,       0xD38EFF65, 0xAA46, 0x4FD5, 0x91, 0xA7, 0x67, 0x84, 0x5F, 0xB0, 0x2F, 0x5B);
DEFINE_GUID( TF_PROFILE_YI,          0x409C8376, 0x007B, 0x4357, 0xAE, 0x8E, 0x26, 0x31, 0x6E, 0xE3, 0xFB, 0x0D);
DEFINE_GUID( TF_PROFILE_TIGRINYA,    0x3CAB88B7, 0xCC3E, 0x46A6, 0x97, 0x65, 0xB7, 0x72, 0xAD, 0x77, 0x61, 0xFF);
#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion
#endif // MSCTF_DEFINED


extern RPC_IF_HANDLE __MIDL_itf_msctf_0000_0091_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_msctf_0000_0091_v0_0_s_ifspec;

/* Additional Prototypes for ALL interfaces */

unsigned long             __RPC_USER  BSTR_UserSize(     __RPC__in unsigned long *, unsigned long            , __RPC__in BSTR * ); 
unsigned char * __RPC_USER  BSTR_UserMarshal(  __RPC__in unsigned long *, __RPC__inout_xcount(0) unsigned char *, __RPC__in BSTR * ); 
unsigned char * __RPC_USER  BSTR_UserUnmarshal(__RPC__in unsigned long *, __RPC__in_xcount(0) unsigned char *, __RPC__out BSTR * ); 
void                      __RPC_USER  BSTR_UserFree(     __RPC__in unsigned long *, __RPC__in BSTR * ); 

unsigned long             __RPC_USER  CLIPFORMAT_UserSize(     __RPC__in unsigned long *, unsigned long            , __RPC__in CLIPFORMAT * ); 
unsigned char * __RPC_USER  CLIPFORMAT_UserMarshal(  __RPC__in unsigned long *, __RPC__inout_xcount(0) unsigned char *, __RPC__in CLIPFORMAT * ); 
unsigned char * __RPC_USER  CLIPFORMAT_UserUnmarshal(__RPC__in unsigned long *, __RPC__in_xcount(0) unsigned char *, __RPC__out CLIPFORMAT * ); 
void                      __RPC_USER  CLIPFORMAT_UserFree(     __RPC__in unsigned long *, __RPC__in CLIPFORMAT * ); 

unsigned long             __RPC_USER  HWND_UserSize(     __RPC__in unsigned long *, unsigned long            , __RPC__in HWND * ); 
unsigned char * __RPC_USER  HWND_UserMarshal(  __RPC__in unsigned long *, __RPC__inout_xcount(0) unsigned char *, __RPC__in HWND * ); 
unsigned char * __RPC_USER  HWND_UserUnmarshal(__RPC__in unsigned long *, __RPC__in_xcount(0) unsigned char *, __RPC__out HWND * ); 
void                      __RPC_USER  HWND_UserFree(     __RPC__in unsigned long *, __RPC__in HWND * ); 

unsigned long             __RPC_USER  VARIANT_UserSize(     __RPC__in unsigned long *, unsigned long            , __RPC__in VARIANT * ); 
unsigned char * __RPC_USER  VARIANT_UserMarshal(  __RPC__in unsigned long *, __RPC__inout_xcount(0) unsigned char *, __RPC__in VARIANT * ); 
unsigned char * __RPC_USER  VARIANT_UserUnmarshal(__RPC__in unsigned long *, __RPC__in_xcount(0) unsigned char *, __RPC__out VARIANT * ); 
void                      __RPC_USER  VARIANT_UserFree(     __RPC__in unsigned long *, __RPC__in VARIANT * ); 

unsigned long             __RPC_USER  BSTR_UserSize64(     __RPC__in unsigned long *, unsigned long            , __RPC__in BSTR * ); 
unsigned char * __RPC_USER  BSTR_UserMarshal64(  __RPC__in unsigned long *, __RPC__inout_xcount(0) unsigned char *, __RPC__in BSTR * ); 
unsigned char * __RPC_USER  BSTR_UserUnmarshal64(__RPC__in unsigned long *, __RPC__in_xcount(0) unsigned char *, __RPC__out BSTR * ); 
void                      __RPC_USER  BSTR_UserFree64(     __RPC__in unsigned long *, __RPC__in BSTR * ); 

unsigned long             __RPC_USER  CLIPFORMAT_UserSize64(     __RPC__in unsigned long *, unsigned long            , __RPC__in CLIPFORMAT * ); 
unsigned char * __RPC_USER  CLIPFORMAT_UserMarshal64(  __RPC__in unsigned long *, __RPC__inout_xcount(0) unsigned char *, __RPC__in CLIPFORMAT * ); 
unsigned char * __RPC_USER  CLIPFORMAT_UserUnmarshal64(__RPC__in unsigned long *, __RPC__in_xcount(0) unsigned char *, __RPC__out CLIPFORMAT * ); 
void                      __RPC_USER  CLIPFORMAT_UserFree64(     __RPC__in unsigned long *, __RPC__in CLIPFORMAT * ); 

unsigned long             __RPC_USER  HWND_UserSize64(     __RPC__in unsigned long *, unsigned long            , __RPC__in HWND * ); 
unsigned char * __RPC_USER  HWND_UserMarshal64(  __RPC__in unsigned long *, __RPC__inout_xcount(0) unsigned char *, __RPC__in HWND * ); 
unsigned char * __RPC_USER  HWND_UserUnmarshal64(__RPC__in unsigned long *, __RPC__in_xcount(0) unsigned char *, __RPC__out HWND * ); 
void                      __RPC_USER  HWND_UserFree64(     __RPC__in unsigned long *, __RPC__in HWND * ); 

unsigned long             __RPC_USER  VARIANT_UserSize64(     __RPC__in unsigned long *, unsigned long            , __RPC__in VARIANT * ); 
unsigned char * __RPC_USER  VARIANT_UserMarshal64(  __RPC__in unsigned long *, __RPC__inout_xcount(0) unsigned char *, __RPC__in VARIANT * ); 
unsigned char * __RPC_USER  VARIANT_UserUnmarshal64(__RPC__in unsigned long *, __RPC__in_xcount(0) unsigned char *, __RPC__out VARIANT * ); 
void                      __RPC_USER  VARIANT_UserFree64(     __RPC__in unsigned long *, __RPC__in VARIANT * ); 

/* end of Additional Prototypes */

#ifdef __cplusplus
}
#endif

#endif


