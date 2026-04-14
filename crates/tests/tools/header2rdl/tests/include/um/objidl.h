

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

#ifndef __objidl_h__
#define __objidl_h__

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

#ifndef __IMarshal_FWD_DEFINED__
#define __IMarshal_FWD_DEFINED__
typedef interface IMarshal IMarshal;

#endif 	/* __IMarshal_FWD_DEFINED__ */


#ifndef __INoMarshal_FWD_DEFINED__
#define __INoMarshal_FWD_DEFINED__
typedef interface INoMarshal INoMarshal;

#endif 	/* __INoMarshal_FWD_DEFINED__ */


#ifndef __IAgileObject_FWD_DEFINED__
#define __IAgileObject_FWD_DEFINED__
typedef interface IAgileObject IAgileObject;

#endif 	/* __IAgileObject_FWD_DEFINED__ */


#ifndef __IActivationFilter_FWD_DEFINED__
#define __IActivationFilter_FWD_DEFINED__
typedef interface IActivationFilter IActivationFilter;

#endif 	/* __IActivationFilter_FWD_DEFINED__ */


#ifndef __IMarshal2_FWD_DEFINED__
#define __IMarshal2_FWD_DEFINED__
typedef interface IMarshal2 IMarshal2;

#endif 	/* __IMarshal2_FWD_DEFINED__ */


#ifndef __IMalloc_FWD_DEFINED__
#define __IMalloc_FWD_DEFINED__
typedef interface IMalloc IMalloc;

#endif 	/* __IMalloc_FWD_DEFINED__ */


#ifndef __IStdMarshalInfo_FWD_DEFINED__
#define __IStdMarshalInfo_FWD_DEFINED__
typedef interface IStdMarshalInfo IStdMarshalInfo;

#endif 	/* __IStdMarshalInfo_FWD_DEFINED__ */


#ifndef __IExternalConnection_FWD_DEFINED__
#define __IExternalConnection_FWD_DEFINED__
typedef interface IExternalConnection IExternalConnection;

#endif 	/* __IExternalConnection_FWD_DEFINED__ */


#ifndef __IMultiQI_FWD_DEFINED__
#define __IMultiQI_FWD_DEFINED__
typedef interface IMultiQI IMultiQI;

#endif 	/* __IMultiQI_FWD_DEFINED__ */


#ifndef __AsyncIMultiQI_FWD_DEFINED__
#define __AsyncIMultiQI_FWD_DEFINED__
typedef interface AsyncIMultiQI AsyncIMultiQI;

#endif 	/* __AsyncIMultiQI_FWD_DEFINED__ */


#ifndef __IInternalUnknown_FWD_DEFINED__
#define __IInternalUnknown_FWD_DEFINED__
typedef interface IInternalUnknown IInternalUnknown;

#endif 	/* __IInternalUnknown_FWD_DEFINED__ */


#ifndef __IEnumUnknown_FWD_DEFINED__
#define __IEnumUnknown_FWD_DEFINED__
typedef interface IEnumUnknown IEnumUnknown;

#endif 	/* __IEnumUnknown_FWD_DEFINED__ */


#ifndef __IEnumString_FWD_DEFINED__
#define __IEnumString_FWD_DEFINED__
typedef interface IEnumString IEnumString;

#endif 	/* __IEnumString_FWD_DEFINED__ */


#ifndef __ISequentialStream_FWD_DEFINED__
#define __ISequentialStream_FWD_DEFINED__
typedef interface ISequentialStream ISequentialStream;

#endif 	/* __ISequentialStream_FWD_DEFINED__ */


#ifndef __IStream_FWD_DEFINED__
#define __IStream_FWD_DEFINED__
typedef interface IStream IStream;

#endif 	/* __IStream_FWD_DEFINED__ */


#ifndef __IRpcChannelBuffer_FWD_DEFINED__
#define __IRpcChannelBuffer_FWD_DEFINED__
typedef interface IRpcChannelBuffer IRpcChannelBuffer;

#endif 	/* __IRpcChannelBuffer_FWD_DEFINED__ */


#ifndef __IRpcChannelBuffer2_FWD_DEFINED__
#define __IRpcChannelBuffer2_FWD_DEFINED__
typedef interface IRpcChannelBuffer2 IRpcChannelBuffer2;

#endif 	/* __IRpcChannelBuffer2_FWD_DEFINED__ */


#ifndef __IAsyncRpcChannelBuffer_FWD_DEFINED__
#define __IAsyncRpcChannelBuffer_FWD_DEFINED__
typedef interface IAsyncRpcChannelBuffer IAsyncRpcChannelBuffer;

#endif 	/* __IAsyncRpcChannelBuffer_FWD_DEFINED__ */


#ifndef __IRpcChannelBuffer3_FWD_DEFINED__
#define __IRpcChannelBuffer3_FWD_DEFINED__
typedef interface IRpcChannelBuffer3 IRpcChannelBuffer3;

#endif 	/* __IRpcChannelBuffer3_FWD_DEFINED__ */


#ifndef __IRpcSyntaxNegotiate_FWD_DEFINED__
#define __IRpcSyntaxNegotiate_FWD_DEFINED__
typedef interface IRpcSyntaxNegotiate IRpcSyntaxNegotiate;

#endif 	/* __IRpcSyntaxNegotiate_FWD_DEFINED__ */


#ifndef __IRpcProxyBuffer_FWD_DEFINED__
#define __IRpcProxyBuffer_FWD_DEFINED__
typedef interface IRpcProxyBuffer IRpcProxyBuffer;

#endif 	/* __IRpcProxyBuffer_FWD_DEFINED__ */


#ifndef __IRpcStubBuffer_FWD_DEFINED__
#define __IRpcStubBuffer_FWD_DEFINED__
typedef interface IRpcStubBuffer IRpcStubBuffer;

#endif 	/* __IRpcStubBuffer_FWD_DEFINED__ */


#ifndef __IPSFactoryBuffer_FWD_DEFINED__
#define __IPSFactoryBuffer_FWD_DEFINED__
typedef interface IPSFactoryBuffer IPSFactoryBuffer;

#endif 	/* __IPSFactoryBuffer_FWD_DEFINED__ */


#ifndef __IChannelHook_FWD_DEFINED__
#define __IChannelHook_FWD_DEFINED__
typedef interface IChannelHook IChannelHook;

#endif 	/* __IChannelHook_FWD_DEFINED__ */


#ifndef __IClientSecurity_FWD_DEFINED__
#define __IClientSecurity_FWD_DEFINED__
typedef interface IClientSecurity IClientSecurity;

#endif 	/* __IClientSecurity_FWD_DEFINED__ */


#ifndef __IServerSecurity_FWD_DEFINED__
#define __IServerSecurity_FWD_DEFINED__
typedef interface IServerSecurity IServerSecurity;

#endif 	/* __IServerSecurity_FWD_DEFINED__ */


#ifndef __IRpcOptions_FWD_DEFINED__
#define __IRpcOptions_FWD_DEFINED__
typedef interface IRpcOptions IRpcOptions;

#endif 	/* __IRpcOptions_FWD_DEFINED__ */


#ifndef __IGlobalOptions_FWD_DEFINED__
#define __IGlobalOptions_FWD_DEFINED__
typedef interface IGlobalOptions IGlobalOptions;

#endif 	/* __IGlobalOptions_FWD_DEFINED__ */


#ifndef __ISurrogate_FWD_DEFINED__
#define __ISurrogate_FWD_DEFINED__
typedef interface ISurrogate ISurrogate;

#endif 	/* __ISurrogate_FWD_DEFINED__ */


#ifndef __IGlobalInterfaceTable_FWD_DEFINED__
#define __IGlobalInterfaceTable_FWD_DEFINED__
typedef interface IGlobalInterfaceTable IGlobalInterfaceTable;

#endif 	/* __IGlobalInterfaceTable_FWD_DEFINED__ */


#ifndef __ISynchronize_FWD_DEFINED__
#define __ISynchronize_FWD_DEFINED__
typedef interface ISynchronize ISynchronize;

#endif 	/* __ISynchronize_FWD_DEFINED__ */


#ifndef __ISynchronizeHandle_FWD_DEFINED__
#define __ISynchronizeHandle_FWD_DEFINED__
typedef interface ISynchronizeHandle ISynchronizeHandle;

#endif 	/* __ISynchronizeHandle_FWD_DEFINED__ */


#ifndef __ISynchronizeEvent_FWD_DEFINED__
#define __ISynchronizeEvent_FWD_DEFINED__
typedef interface ISynchronizeEvent ISynchronizeEvent;

#endif 	/* __ISynchronizeEvent_FWD_DEFINED__ */


#ifndef __ISynchronizeContainer_FWD_DEFINED__
#define __ISynchronizeContainer_FWD_DEFINED__
typedef interface ISynchronizeContainer ISynchronizeContainer;

#endif 	/* __ISynchronizeContainer_FWD_DEFINED__ */


#ifndef __ISynchronizeMutex_FWD_DEFINED__
#define __ISynchronizeMutex_FWD_DEFINED__
typedef interface ISynchronizeMutex ISynchronizeMutex;

#endif 	/* __ISynchronizeMutex_FWD_DEFINED__ */


#ifndef __ICancelMethodCalls_FWD_DEFINED__
#define __ICancelMethodCalls_FWD_DEFINED__
typedef interface ICancelMethodCalls ICancelMethodCalls;

#endif 	/* __ICancelMethodCalls_FWD_DEFINED__ */


#ifndef __IAsyncManager_FWD_DEFINED__
#define __IAsyncManager_FWD_DEFINED__
typedef interface IAsyncManager IAsyncManager;

#endif 	/* __IAsyncManager_FWD_DEFINED__ */


#ifndef __ICallFactory_FWD_DEFINED__
#define __ICallFactory_FWD_DEFINED__
typedef interface ICallFactory ICallFactory;

#endif 	/* __ICallFactory_FWD_DEFINED__ */


#ifndef __IRpcHelper_FWD_DEFINED__
#define __IRpcHelper_FWD_DEFINED__
typedef interface IRpcHelper IRpcHelper;

#endif 	/* __IRpcHelper_FWD_DEFINED__ */


#ifndef __IReleaseMarshalBuffers_FWD_DEFINED__
#define __IReleaseMarshalBuffers_FWD_DEFINED__
typedef interface IReleaseMarshalBuffers IReleaseMarshalBuffers;

#endif 	/* __IReleaseMarshalBuffers_FWD_DEFINED__ */


#ifndef __IWaitMultiple_FWD_DEFINED__
#define __IWaitMultiple_FWD_DEFINED__
typedef interface IWaitMultiple IWaitMultiple;

#endif 	/* __IWaitMultiple_FWD_DEFINED__ */


#ifndef __IAddrTrackingControl_FWD_DEFINED__
#define __IAddrTrackingControl_FWD_DEFINED__
typedef interface IAddrTrackingControl IAddrTrackingControl;

#endif 	/* __IAddrTrackingControl_FWD_DEFINED__ */


#ifndef __IAddrExclusionControl_FWD_DEFINED__
#define __IAddrExclusionControl_FWD_DEFINED__
typedef interface IAddrExclusionControl IAddrExclusionControl;

#endif 	/* __IAddrExclusionControl_FWD_DEFINED__ */


#ifndef __IPipeByte_FWD_DEFINED__
#define __IPipeByte_FWD_DEFINED__
typedef interface IPipeByte IPipeByte;

#endif 	/* __IPipeByte_FWD_DEFINED__ */


#ifndef __AsyncIPipeByte_FWD_DEFINED__
#define __AsyncIPipeByte_FWD_DEFINED__
typedef interface AsyncIPipeByte AsyncIPipeByte;

#endif 	/* __AsyncIPipeByte_FWD_DEFINED__ */


#ifndef __IPipeLong_FWD_DEFINED__
#define __IPipeLong_FWD_DEFINED__
typedef interface IPipeLong IPipeLong;

#endif 	/* __IPipeLong_FWD_DEFINED__ */


#ifndef __AsyncIPipeLong_FWD_DEFINED__
#define __AsyncIPipeLong_FWD_DEFINED__
typedef interface AsyncIPipeLong AsyncIPipeLong;

#endif 	/* __AsyncIPipeLong_FWD_DEFINED__ */


#ifndef __IPipeDouble_FWD_DEFINED__
#define __IPipeDouble_FWD_DEFINED__
typedef interface IPipeDouble IPipeDouble;

#endif 	/* __IPipeDouble_FWD_DEFINED__ */


#ifndef __AsyncIPipeDouble_FWD_DEFINED__
#define __AsyncIPipeDouble_FWD_DEFINED__
typedef interface AsyncIPipeDouble AsyncIPipeDouble;

#endif 	/* __AsyncIPipeDouble_FWD_DEFINED__ */


#ifndef __IEnumContextProps_FWD_DEFINED__
#define __IEnumContextProps_FWD_DEFINED__
typedef interface IEnumContextProps IEnumContextProps;

#endif 	/* __IEnumContextProps_FWD_DEFINED__ */


#ifndef __IContext_FWD_DEFINED__
#define __IContext_FWD_DEFINED__
typedef interface IContext IContext;

#endif 	/* __IContext_FWD_DEFINED__ */


#ifndef __IObjContext_FWD_DEFINED__
#define __IObjContext_FWD_DEFINED__
typedef interface IObjContext IObjContext;

#endif 	/* __IObjContext_FWD_DEFINED__ */


#ifndef __IComThreadingInfo_FWD_DEFINED__
#define __IComThreadingInfo_FWD_DEFINED__
typedef interface IComThreadingInfo IComThreadingInfo;

#endif 	/* __IComThreadingInfo_FWD_DEFINED__ */


#ifndef __IProcessInitControl_FWD_DEFINED__
#define __IProcessInitControl_FWD_DEFINED__
typedef interface IProcessInitControl IProcessInitControl;

#endif 	/* __IProcessInitControl_FWD_DEFINED__ */


#ifndef __IFastRundown_FWD_DEFINED__
#define __IFastRundown_FWD_DEFINED__
typedef interface IFastRundown IFastRundown;

#endif 	/* __IFastRundown_FWD_DEFINED__ */


#ifndef __IMarshalingStream_FWD_DEFINED__
#define __IMarshalingStream_FWD_DEFINED__
typedef interface IMarshalingStream IMarshalingStream;

#endif 	/* __IMarshalingStream_FWD_DEFINED__ */


#ifndef __IAgileReference_FWD_DEFINED__
#define __IAgileReference_FWD_DEFINED__
typedef interface IAgileReference IAgileReference;

#endif 	/* __IAgileReference_FWD_DEFINED__ */


#ifndef __IMachineGlobalObjectTable_FWD_DEFINED__
#define __IMachineGlobalObjectTable_FWD_DEFINED__
typedef interface IMachineGlobalObjectTable IMachineGlobalObjectTable;

#endif 	/* __IMachineGlobalObjectTable_FWD_DEFINED__ */


#ifndef __ISupportAllowLowerTrustActivation_FWD_DEFINED__
#define __ISupportAllowLowerTrustActivation_FWD_DEFINED__
typedef interface ISupportAllowLowerTrustActivation ISupportAllowLowerTrustActivation;

#endif 	/* __ISupportAllowLowerTrustActivation_FWD_DEFINED__ */


#ifndef __ISupportActivationFromPackage_FWD_DEFINED__
#define __ISupportActivationFromPackage_FWD_DEFINED__
typedef interface ISupportActivationFromPackage ISupportActivationFromPackage;

#endif 	/* __ISupportActivationFromPackage_FWD_DEFINED__ */


#ifndef __ISupportCoAddComDependencyOnPackage_FWD_DEFINED__
#define __ISupportCoAddComDependencyOnPackage_FWD_DEFINED__
typedef interface ISupportCoAddComDependencyOnPackage ISupportCoAddComDependencyOnPackage;

#endif 	/* __ISupportCoAddComDependencyOnPackage_FWD_DEFINED__ */


#ifndef __ISupportServerMustBeEqualOrGreaterPrivilegeActivation_FWD_DEFINED__
#define __ISupportServerMustBeEqualOrGreaterPrivilegeActivation_FWD_DEFINED__
typedef interface ISupportServerMustBeEqualOrGreaterPrivilegeActivation ISupportServerMustBeEqualOrGreaterPrivilegeActivation;

#endif 	/* __ISupportServerMustBeEqualOrGreaterPrivilegeActivation_FWD_DEFINED__ */


#ifndef __ISupportDoNotElevateServerActivation_FWD_DEFINED__
#define __ISupportDoNotElevateServerActivation_FWD_DEFINED__
typedef interface ISupportDoNotElevateServerActivation ISupportDoNotElevateServerActivation;

#endif 	/* __ISupportDoNotElevateServerActivation_FWD_DEFINED__ */


#ifndef __ISupportActivateAsActivatorPackaged_FWD_DEFINED__
#define __ISupportActivateAsActivatorPackaged_FWD_DEFINED__
typedef interface ISupportActivateAsActivatorPackaged ISupportActivateAsActivatorPackaged;

#endif 	/* __ISupportActivateAsActivatorPackaged_FWD_DEFINED__ */


#ifndef __ISupportPackagedComRegistrationVisibility_FWD_DEFINED__
#define __ISupportPackagedComRegistrationVisibility_FWD_DEFINED__
typedef interface ISupportPackagedComRegistrationVisibility ISupportPackagedComRegistrationVisibility;

#endif 	/* __ISupportPackagedComRegistrationVisibility_FWD_DEFINED__ */


#ifndef __ISupportPackagedComElevationEnabledClasses_FWD_DEFINED__
#define __ISupportPackagedComElevationEnabledClasses_FWD_DEFINED__
typedef interface ISupportPackagedComElevationEnabledClasses ISupportPackagedComElevationEnabledClasses;

#endif 	/* __ISupportPackagedComElevationEnabledClasses_FWD_DEFINED__ */


#ifndef __IPackagedComSyntaxSupport_FWD_DEFINED__
#define __IPackagedComSyntaxSupport_FWD_DEFINED__
typedef interface IPackagedComSyntaxSupport IPackagedComSyntaxSupport;

#endif 	/* __IPackagedComSyntaxSupport_FWD_DEFINED__ */


#ifndef __IMallocSpy_FWD_DEFINED__
#define __IMallocSpy_FWD_DEFINED__
typedef interface IMallocSpy IMallocSpy;

#endif 	/* __IMallocSpy_FWD_DEFINED__ */


#ifndef __IBindCtx_FWD_DEFINED__
#define __IBindCtx_FWD_DEFINED__
typedef interface IBindCtx IBindCtx;

#endif 	/* __IBindCtx_FWD_DEFINED__ */


#ifndef __IEnumMoniker_FWD_DEFINED__
#define __IEnumMoniker_FWD_DEFINED__
typedef interface IEnumMoniker IEnumMoniker;

#endif 	/* __IEnumMoniker_FWD_DEFINED__ */


#ifndef __IRunnableObject_FWD_DEFINED__
#define __IRunnableObject_FWD_DEFINED__
typedef interface IRunnableObject IRunnableObject;

#endif 	/* __IRunnableObject_FWD_DEFINED__ */


#ifndef __IRunningObjectTable_FWD_DEFINED__
#define __IRunningObjectTable_FWD_DEFINED__
typedef interface IRunningObjectTable IRunningObjectTable;

#endif 	/* __IRunningObjectTable_FWD_DEFINED__ */


#ifndef __IPersist_FWD_DEFINED__
#define __IPersist_FWD_DEFINED__
typedef interface IPersist IPersist;

#endif 	/* __IPersist_FWD_DEFINED__ */


#ifndef __IPersistStream_FWD_DEFINED__
#define __IPersistStream_FWD_DEFINED__
typedef interface IPersistStream IPersistStream;

#endif 	/* __IPersistStream_FWD_DEFINED__ */


#ifndef __IMoniker_FWD_DEFINED__
#define __IMoniker_FWD_DEFINED__
typedef interface IMoniker IMoniker;

#endif 	/* __IMoniker_FWD_DEFINED__ */


#ifndef __IROTData_FWD_DEFINED__
#define __IROTData_FWD_DEFINED__
typedef interface IROTData IROTData;

#endif 	/* __IROTData_FWD_DEFINED__ */


#ifndef __IEnumSTATSTG_FWD_DEFINED__
#define __IEnumSTATSTG_FWD_DEFINED__
typedef interface IEnumSTATSTG IEnumSTATSTG;

#endif 	/* __IEnumSTATSTG_FWD_DEFINED__ */


#ifndef __IStorage_FWD_DEFINED__
#define __IStorage_FWD_DEFINED__
typedef interface IStorage IStorage;

#endif 	/* __IStorage_FWD_DEFINED__ */


#ifndef __IPersistFile_FWD_DEFINED__
#define __IPersistFile_FWD_DEFINED__
typedef interface IPersistFile IPersistFile;

#endif 	/* __IPersistFile_FWD_DEFINED__ */


#ifndef __IPersistStorage_FWD_DEFINED__
#define __IPersistStorage_FWD_DEFINED__
typedef interface IPersistStorage IPersistStorage;

#endif 	/* __IPersistStorage_FWD_DEFINED__ */


#ifndef __ILockBytes_FWD_DEFINED__
#define __ILockBytes_FWD_DEFINED__
typedef interface ILockBytes ILockBytes;

#endif 	/* __ILockBytes_FWD_DEFINED__ */


#ifndef __IEnumFORMATETC_FWD_DEFINED__
#define __IEnumFORMATETC_FWD_DEFINED__
typedef interface IEnumFORMATETC IEnumFORMATETC;

#endif 	/* __IEnumFORMATETC_FWD_DEFINED__ */


#ifndef __IEnumSTATDATA_FWD_DEFINED__
#define __IEnumSTATDATA_FWD_DEFINED__
typedef interface IEnumSTATDATA IEnumSTATDATA;

#endif 	/* __IEnumSTATDATA_FWD_DEFINED__ */


#ifndef __IRootStorage_FWD_DEFINED__
#define __IRootStorage_FWD_DEFINED__
typedef interface IRootStorage IRootStorage;

#endif 	/* __IRootStorage_FWD_DEFINED__ */


#ifndef __IAdviseSink_FWD_DEFINED__
#define __IAdviseSink_FWD_DEFINED__
typedef interface IAdviseSink IAdviseSink;

#endif 	/* __IAdviseSink_FWD_DEFINED__ */


#ifndef __AsyncIAdviseSink_FWD_DEFINED__
#define __AsyncIAdviseSink_FWD_DEFINED__
typedef interface AsyncIAdviseSink AsyncIAdviseSink;

#endif 	/* __AsyncIAdviseSink_FWD_DEFINED__ */


#ifndef __IAdviseSink2_FWD_DEFINED__
#define __IAdviseSink2_FWD_DEFINED__
typedef interface IAdviseSink2 IAdviseSink2;

#endif 	/* __IAdviseSink2_FWD_DEFINED__ */


#ifndef __AsyncIAdviseSink2_FWD_DEFINED__
#define __AsyncIAdviseSink2_FWD_DEFINED__
typedef interface AsyncIAdviseSink2 AsyncIAdviseSink2;

#endif 	/* __AsyncIAdviseSink2_FWD_DEFINED__ */


#ifndef __IDataObject_FWD_DEFINED__
#define __IDataObject_FWD_DEFINED__
typedef interface IDataObject IDataObject;

#endif 	/* __IDataObject_FWD_DEFINED__ */


#ifndef __IDataAdviseHolder_FWD_DEFINED__
#define __IDataAdviseHolder_FWD_DEFINED__
typedef interface IDataAdviseHolder IDataAdviseHolder;

#endif 	/* __IDataAdviseHolder_FWD_DEFINED__ */


#ifndef __IMessageFilter_FWD_DEFINED__
#define __IMessageFilter_FWD_DEFINED__
typedef interface IMessageFilter IMessageFilter;

#endif 	/* __IMessageFilter_FWD_DEFINED__ */


#ifndef __IClassActivator_FWD_DEFINED__
#define __IClassActivator_FWD_DEFINED__
typedef interface IClassActivator IClassActivator;

#endif 	/* __IClassActivator_FWD_DEFINED__ */


#ifndef __IFillLockBytes_FWD_DEFINED__
#define __IFillLockBytes_FWD_DEFINED__
typedef interface IFillLockBytes IFillLockBytes;

#endif 	/* __IFillLockBytes_FWD_DEFINED__ */


#ifndef __IProgressNotify_FWD_DEFINED__
#define __IProgressNotify_FWD_DEFINED__
typedef interface IProgressNotify IProgressNotify;

#endif 	/* __IProgressNotify_FWD_DEFINED__ */


#ifndef __ILayoutStorage_FWD_DEFINED__
#define __ILayoutStorage_FWD_DEFINED__
typedef interface ILayoutStorage ILayoutStorage;

#endif 	/* __ILayoutStorage_FWD_DEFINED__ */


#ifndef __IBlockingLock_FWD_DEFINED__
#define __IBlockingLock_FWD_DEFINED__
typedef interface IBlockingLock IBlockingLock;

#endif 	/* __IBlockingLock_FWD_DEFINED__ */


#ifndef __ITimeAndNoticeControl_FWD_DEFINED__
#define __ITimeAndNoticeControl_FWD_DEFINED__
typedef interface ITimeAndNoticeControl ITimeAndNoticeControl;

#endif 	/* __ITimeAndNoticeControl_FWD_DEFINED__ */


#ifndef __IOplockStorage_FWD_DEFINED__
#define __IOplockStorage_FWD_DEFINED__
typedef interface IOplockStorage IOplockStorage;

#endif 	/* __IOplockStorage_FWD_DEFINED__ */


#ifndef __IDirectWriterLock_FWD_DEFINED__
#define __IDirectWriterLock_FWD_DEFINED__
typedef interface IDirectWriterLock IDirectWriterLock;

#endif 	/* __IDirectWriterLock_FWD_DEFINED__ */


#ifndef __IUrlMon_FWD_DEFINED__
#define __IUrlMon_FWD_DEFINED__
typedef interface IUrlMon IUrlMon;

#endif 	/* __IUrlMon_FWD_DEFINED__ */


#ifndef __IForegroundTransfer_FWD_DEFINED__
#define __IForegroundTransfer_FWD_DEFINED__
typedef interface IForegroundTransfer IForegroundTransfer;

#endif 	/* __IForegroundTransfer_FWD_DEFINED__ */


#ifndef __IThumbnailExtractor_FWD_DEFINED__
#define __IThumbnailExtractor_FWD_DEFINED__
typedef interface IThumbnailExtractor IThumbnailExtractor;

#endif 	/* __IThumbnailExtractor_FWD_DEFINED__ */


#ifndef __IDummyHICONIncluder_FWD_DEFINED__
#define __IDummyHICONIncluder_FWD_DEFINED__
typedef interface IDummyHICONIncluder IDummyHICONIncluder;

#endif 	/* __IDummyHICONIncluder_FWD_DEFINED__ */


#ifndef __IProcessLock_FWD_DEFINED__
#define __IProcessLock_FWD_DEFINED__
typedef interface IProcessLock IProcessLock;

#endif 	/* __IProcessLock_FWD_DEFINED__ */


#ifndef __ISurrogateService_FWD_DEFINED__
#define __ISurrogateService_FWD_DEFINED__
typedef interface ISurrogateService ISurrogateService;

#endif 	/* __ISurrogateService_FWD_DEFINED__ */


#ifndef __IInitializeSpy_FWD_DEFINED__
#define __IInitializeSpy_FWD_DEFINED__
typedef interface IInitializeSpy IInitializeSpy;

#endif 	/* __IInitializeSpy_FWD_DEFINED__ */


#ifndef __IApartmentShutdown_FWD_DEFINED__
#define __IApartmentShutdown_FWD_DEFINED__
typedef interface IApartmentShutdown IApartmentShutdown;

#endif 	/* __IApartmentShutdown_FWD_DEFINED__ */


/* header files for imported files */
#include "unknwn.h"

#ifdef __cplusplus
extern "C"{
#endif 


/* interface __MIDL_itf_objidl_0000_0000 */
/* [local] */ 

#include <winapifamily.h>
//+-------------------------------------------------------------------------
//+-------------------------------------------------------------------------
//
//  Microsoft Windows
//  Copyright (c) Microsoft Corporation. All rights reserved.
//
//--------------------------------------------------------------------------
#if(NTDDI_VERSION >= NTDDI_VISTA && !defined(_WIN32_WINNT))
#define _WIN32_WINNT 0x0600
#endif
#if(NTDDI_VERSION >= NTDDI_WS03 && !defined(_WIN32_WINNT))
#define _WIN32_WINNT 0x0502
#endif
#if(NTDDI_VERSION >= NTDDI_WINXP && !defined(_WIN32_WINNT))
#define _WIN32_WINNT 0x0501
#endif
#if(NTDDI_VERSION >= NTDDI_WIN2K && !defined(_WIN32_WINNT))
#define _WIN32_WINNT 0x0500
#endif



#if ( _MSC_VER >= 800 )
#if _MSC_VER >= 1200
#pragma warning(push)
#pragma warning(disable:4820) /* padding added after data member */
#endif
#pragma warning(disable:4201)
#endif
#if ( _MSC_VER >= 1020 )
#pragma once
#endif
#include <winapifamily.h>
//+-------------------------------------------------------------------------
//+-------------------------------------------------------------------------
//
//  Microsoft Windows
//  Copyright (c) Microsoft Corporation. All rights reserved.
//
//--------------------------------------------------------------------------
#if(NTDDI_VERSION >= NTDDI_VISTA && !defined(_WIN32_WINNT))
#define _WIN32_WINNT 0x0600
#endif
#if(NTDDI_VERSION >= NTDDI_WS03 && !defined(_WIN32_WINNT))
#define _WIN32_WINNT 0x0502
#endif
#if(NTDDI_VERSION >= NTDDI_WINXP && !defined(_WIN32_WINNT))
#define _WIN32_WINNT 0x0501
#endif
#if(NTDDI_VERSION >= NTDDI_WIN2K && !defined(_WIN32_WINNT))
#define _WIN32_WINNT 0x0500
#endif



#if ( _MSC_VER >= 800 )
#if _MSC_VER >= 1200
#pragma warning(push)
#ifndef _MSC_EXTENSIONS
#pragma warning(disable:4309) /* truncation of constant value */
#endif
#pragma warning(disable:4820) /* padding added after data member */
#endif
#pragma warning(disable:4201)
#endif
#if ( _MSC_VER >= 1020 )
#pragma once
#endif
#include <limits.h>
#ifndef _OBJIDLBASE_
#pragma region Application Family or OneCore Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES)


#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES) */
#pragma endregion
#pragma region Desktop Family or OneCore Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES)



#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES) */
#pragma endregion
#pragma region Application Family or OneCore Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES)

typedef struct _COSERVERINFO
    {
    DWORD dwReserved1;
    LPWSTR pwszName;
    COAUTHINFO *pAuthInfo;
    DWORD dwReserved2;
    } 	COSERVERINFO;




extern RPC_IF_HANDLE __MIDL_itf_objidl_0000_0000_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_objidl_0000_0000_v0_0_s_ifspec;

#ifndef __IMarshal_INTERFACE_DEFINED__
#define __IMarshal_INTERFACE_DEFINED__

/* interface IMarshal */
/* [uuid][object][local] */ 

typedef /* [unique] */ IMarshal *LPMARSHAL;


EXTERN_C const IID IID_IMarshal;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("00000003-0000-0000-C000-000000000046")
    IMarshal : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetUnmarshalClass( 
            /* [annotation][in] */ 
            _In_  REFIID riid,
            /* [annotation][unique][in] */ 
            _In_opt_  void *pv,
            /* [annotation][in] */ 
            _In_  DWORD dwDestContext,
            /* [annotation][unique][in] */ 
            _Reserved_  void *pvDestContext,
            /* [annotation][in] */ 
            _In_  DWORD mshlflags,
            /* [annotation][out] */ 
            _Out_  CLSID *pCid) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetMarshalSizeMax( 
            /* [annotation][in] */ 
            _In_  REFIID riid,
            /* [annotation][unique][in] */ 
            _In_opt_  void *pv,
            /* [annotation][in] */ 
            _In_  DWORD dwDestContext,
            /* [annotation][unique][in] */ 
            _Reserved_  void *pvDestContext,
            /* [annotation][in] */ 
            _In_  DWORD mshlflags,
            /* [annotation][out] */ 
            _Out_  DWORD *pSize) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE MarshalInterface( 
            /* [annotation][unique][in] */ 
            _In_  IStream *pStm,
            /* [annotation][in] */ 
            _In_  REFIID riid,
            /* [annotation][unique][in] */ 
            _In_opt_  void *pv,
            /* [annotation][in] */ 
            _In_  DWORD dwDestContext,
            /* [annotation][unique][in] */ 
            _Reserved_  void *pvDestContext,
            /* [annotation][in] */ 
            _In_  DWORD mshlflags) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE UnmarshalInterface( 
            /* [annotation][unique][in] */ 
            _In_  IStream *pStm,
            /* [annotation][in] */ 
            _In_  REFIID riid,
            /* [annotation][out] */ 
            _Outptr_  void **ppv) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE ReleaseMarshalData( 
            /* [annotation][unique][in] */ 
            _In_  IStream *pStm) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE DisconnectObject( 
            /* [annotation][in] */ 
            _In_  DWORD dwReserved) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IMarshalVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IMarshal * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IMarshal * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IMarshal * This);
        
        DECLSPEC_XFGVIRT(IMarshal, GetUnmarshalClass)
        HRESULT ( STDMETHODCALLTYPE *GetUnmarshalClass )( 
            IMarshal * This,
            /* [annotation][in] */ 
            _In_  REFIID riid,
            /* [annotation][unique][in] */ 
            _In_opt_  void *pv,
            /* [annotation][in] */ 
            _In_  DWORD dwDestContext,
            /* [annotation][unique][in] */ 
            _Reserved_  void *pvDestContext,
            /* [annotation][in] */ 
            _In_  DWORD mshlflags,
            /* [annotation][out] */ 
            _Out_  CLSID *pCid);
        
        DECLSPEC_XFGVIRT(IMarshal, GetMarshalSizeMax)
        HRESULT ( STDMETHODCALLTYPE *GetMarshalSizeMax )( 
            IMarshal * This,
            /* [annotation][in] */ 
            _In_  REFIID riid,
            /* [annotation][unique][in] */ 
            _In_opt_  void *pv,
            /* [annotation][in] */ 
            _In_  DWORD dwDestContext,
            /* [annotation][unique][in] */ 
            _Reserved_  void *pvDestContext,
            /* [annotation][in] */ 
            _In_  DWORD mshlflags,
            /* [annotation][out] */ 
            _Out_  DWORD *pSize);
        
        DECLSPEC_XFGVIRT(IMarshal, MarshalInterface)
        HRESULT ( STDMETHODCALLTYPE *MarshalInterface )( 
            IMarshal * This,
            /* [annotation][unique][in] */ 
            _In_  IStream *pStm,
            /* [annotation][in] */ 
            _In_  REFIID riid,
            /* [annotation][unique][in] */ 
            _In_opt_  void *pv,
            /* [annotation][in] */ 
            _In_  DWORD dwDestContext,
            /* [annotation][unique][in] */ 
            _Reserved_  void *pvDestContext,
            /* [annotation][in] */ 
            _In_  DWORD mshlflags);
        
        DECLSPEC_XFGVIRT(IMarshal, UnmarshalInterface)
        HRESULT ( STDMETHODCALLTYPE *UnmarshalInterface )( 
            IMarshal * This,
            /* [annotation][unique][in] */ 
            _In_  IStream *pStm,
            /* [annotation][in] */ 
            _In_  REFIID riid,
            /* [annotation][out] */ 
            _Outptr_  void **ppv);
        
        DECLSPEC_XFGVIRT(IMarshal, ReleaseMarshalData)
        HRESULT ( STDMETHODCALLTYPE *ReleaseMarshalData )( 
            IMarshal * This,
            /* [annotation][unique][in] */ 
            _In_  IStream *pStm);
        
        DECLSPEC_XFGVIRT(IMarshal, DisconnectObject)
        HRESULT ( STDMETHODCALLTYPE *DisconnectObject )( 
            IMarshal * This,
            /* [annotation][in] */ 
            _In_  DWORD dwReserved);
        
        END_INTERFACE
    } IMarshalVtbl;

    interface IMarshal
    {
        CONST_VTBL struct IMarshalVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IMarshal_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IMarshal_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IMarshal_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IMarshal_GetUnmarshalClass(This,riid,pv,dwDestContext,pvDestContext,mshlflags,pCid)	\
    ( (This)->lpVtbl -> GetUnmarshalClass(This,riid,pv,dwDestContext,pvDestContext,mshlflags,pCid) ) 

#define IMarshal_GetMarshalSizeMax(This,riid,pv,dwDestContext,pvDestContext,mshlflags,pSize)	\
    ( (This)->lpVtbl -> GetMarshalSizeMax(This,riid,pv,dwDestContext,pvDestContext,mshlflags,pSize) ) 

#define IMarshal_MarshalInterface(This,pStm,riid,pv,dwDestContext,pvDestContext,mshlflags)	\
    ( (This)->lpVtbl -> MarshalInterface(This,pStm,riid,pv,dwDestContext,pvDestContext,mshlflags) ) 

#define IMarshal_UnmarshalInterface(This,pStm,riid,ppv)	\
    ( (This)->lpVtbl -> UnmarshalInterface(This,pStm,riid,ppv) ) 

#define IMarshal_ReleaseMarshalData(This,pStm)	\
    ( (This)->lpVtbl -> ReleaseMarshalData(This,pStm) ) 

#define IMarshal_DisconnectObject(This,dwReserved)	\
    ( (This)->lpVtbl -> DisconnectObject(This,dwReserved) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IMarshal_INTERFACE_DEFINED__ */


#ifndef __INoMarshal_INTERFACE_DEFINED__
#define __INoMarshal_INTERFACE_DEFINED__

/* interface INoMarshal */
/* [uuid][object][local] */ 


EXTERN_C const IID IID_INoMarshal;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("ecc8691b-c1db-4dc0-855e-65f6c551af49")
    INoMarshal : public IUnknown
    {
    public:
    };
    
    
#else 	/* C style interface */

    typedef struct INoMarshalVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            INoMarshal * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            INoMarshal * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            INoMarshal * This);
        
        END_INTERFACE
    } INoMarshalVtbl;

    interface INoMarshal
    {
        CONST_VTBL struct INoMarshalVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define INoMarshal_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define INoMarshal_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define INoMarshal_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __INoMarshal_INTERFACE_DEFINED__ */


#ifndef __IAgileObject_INTERFACE_DEFINED__
#define __IAgileObject_INTERFACE_DEFINED__

/* interface IAgileObject */
/* [uuid][object][local] */ 


EXTERN_C const IID IID_IAgileObject;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("94ea2b94-e9cc-49e0-c0ff-ee64ca8f5b90")
    IAgileObject : public IUnknown
    {
    public:
    };
    
    
#else 	/* C style interface */

    typedef struct IAgileObjectVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IAgileObject * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IAgileObject * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IAgileObject * This);
        
        END_INTERFACE
    } IAgileObjectVtbl;

    interface IAgileObject
    {
        CONST_VTBL struct IAgileObjectVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IAgileObject_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IAgileObject_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IAgileObject_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IAgileObject_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_objidl_0000_0003 */
/* [local] */ 

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES) */
#pragma endregion
#pragma region Desktop Family or OneCore Family or OneCore Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES)


extern RPC_IF_HANDLE __MIDL_itf_objidl_0000_0003_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_objidl_0000_0003_v0_0_s_ifspec;

#ifndef __IActivationFilter_INTERFACE_DEFINED__
#define __IActivationFilter_INTERFACE_DEFINED__

/* interface IActivationFilter */
/* [uuid][object][local] */ 

typedef 
enum tagACTIVATIONTYPE
    {
        ACTIVATIONTYPE_UNCATEGORIZED	= 0,
        ACTIVATIONTYPE_FROM_MONIKER	= 0x1,
        ACTIVATIONTYPE_FROM_DATA	= 0x2,
        ACTIVATIONTYPE_FROM_STORAGE	= 0x4,
        ACTIVATIONTYPE_FROM_STREAM	= 0x8,
        ACTIVATIONTYPE_FROM_FILE	= 0x10
    } 	ACTIVATIONTYPE;


EXTERN_C const IID IID_IActivationFilter;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("00000017-0000-0000-C000-000000000046")
    IActivationFilter : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE HandleActivation( 
            /* [in] */ DWORD dwActivationType,
            /* [in] */ REFCLSID rclsid,
            /* [out] */ CLSID *pReplacementClsId) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IActivationFilterVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IActivationFilter * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IActivationFilter * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IActivationFilter * This);
        
        DECLSPEC_XFGVIRT(IActivationFilter, HandleActivation)
        HRESULT ( STDMETHODCALLTYPE *HandleActivation )( 
            IActivationFilter * This,
            /* [in] */ DWORD dwActivationType,
            /* [in] */ REFCLSID rclsid,
            /* [out] */ CLSID *pReplacementClsId);
        
        END_INTERFACE
    } IActivationFilterVtbl;

    interface IActivationFilter
    {
        CONST_VTBL struct IActivationFilterVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IActivationFilter_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IActivationFilter_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IActivationFilter_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IActivationFilter_HandleActivation(This,dwActivationType,rclsid,pReplacementClsId)	\
    ( (This)->lpVtbl -> HandleActivation(This,dwActivationType,rclsid,pReplacementClsId) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IActivationFilter_INTERFACE_DEFINED__ */


#ifndef __IMarshal2_INTERFACE_DEFINED__
#define __IMarshal2_INTERFACE_DEFINED__

/* interface IMarshal2 */
/* [uuid][object][local] */ 

typedef /* [unique] */ IMarshal2 *LPMARSHAL2;


EXTERN_C const IID IID_IMarshal2;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("000001cf-0000-0000-C000-000000000046")
    IMarshal2 : public IMarshal
    {
    public:
    };
    
    
#else 	/* C style interface */

    typedef struct IMarshal2Vtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IMarshal2 * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IMarshal2 * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IMarshal2 * This);
        
        DECLSPEC_XFGVIRT(IMarshal, GetUnmarshalClass)
        HRESULT ( STDMETHODCALLTYPE *GetUnmarshalClass )( 
            IMarshal2 * This,
            /* [annotation][in] */ 
            _In_  REFIID riid,
            /* [annotation][unique][in] */ 
            _In_opt_  void *pv,
            /* [annotation][in] */ 
            _In_  DWORD dwDestContext,
            /* [annotation][unique][in] */ 
            _Reserved_  void *pvDestContext,
            /* [annotation][in] */ 
            _In_  DWORD mshlflags,
            /* [annotation][out] */ 
            _Out_  CLSID *pCid);
        
        DECLSPEC_XFGVIRT(IMarshal, GetMarshalSizeMax)
        HRESULT ( STDMETHODCALLTYPE *GetMarshalSizeMax )( 
            IMarshal2 * This,
            /* [annotation][in] */ 
            _In_  REFIID riid,
            /* [annotation][unique][in] */ 
            _In_opt_  void *pv,
            /* [annotation][in] */ 
            _In_  DWORD dwDestContext,
            /* [annotation][unique][in] */ 
            _Reserved_  void *pvDestContext,
            /* [annotation][in] */ 
            _In_  DWORD mshlflags,
            /* [annotation][out] */ 
            _Out_  DWORD *pSize);
        
        DECLSPEC_XFGVIRT(IMarshal, MarshalInterface)
        HRESULT ( STDMETHODCALLTYPE *MarshalInterface )( 
            IMarshal2 * This,
            /* [annotation][unique][in] */ 
            _In_  IStream *pStm,
            /* [annotation][in] */ 
            _In_  REFIID riid,
            /* [annotation][unique][in] */ 
            _In_opt_  void *pv,
            /* [annotation][in] */ 
            _In_  DWORD dwDestContext,
            /* [annotation][unique][in] */ 
            _Reserved_  void *pvDestContext,
            /* [annotation][in] */ 
            _In_  DWORD mshlflags);
        
        DECLSPEC_XFGVIRT(IMarshal, UnmarshalInterface)
        HRESULT ( STDMETHODCALLTYPE *UnmarshalInterface )( 
            IMarshal2 * This,
            /* [annotation][unique][in] */ 
            _In_  IStream *pStm,
            /* [annotation][in] */ 
            _In_  REFIID riid,
            /* [annotation][out] */ 
            _Outptr_  void **ppv);
        
        DECLSPEC_XFGVIRT(IMarshal, ReleaseMarshalData)
        HRESULT ( STDMETHODCALLTYPE *ReleaseMarshalData )( 
            IMarshal2 * This,
            /* [annotation][unique][in] */ 
            _In_  IStream *pStm);
        
        DECLSPEC_XFGVIRT(IMarshal, DisconnectObject)
        HRESULT ( STDMETHODCALLTYPE *DisconnectObject )( 
            IMarshal2 * This,
            /* [annotation][in] */ 
            _In_  DWORD dwReserved);
        
        END_INTERFACE
    } IMarshal2Vtbl;

    interface IMarshal2
    {
        CONST_VTBL struct IMarshal2Vtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IMarshal2_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IMarshal2_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IMarshal2_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IMarshal2_GetUnmarshalClass(This,riid,pv,dwDestContext,pvDestContext,mshlflags,pCid)	\
    ( (This)->lpVtbl -> GetUnmarshalClass(This,riid,pv,dwDestContext,pvDestContext,mshlflags,pCid) ) 

#define IMarshal2_GetMarshalSizeMax(This,riid,pv,dwDestContext,pvDestContext,mshlflags,pSize)	\
    ( (This)->lpVtbl -> GetMarshalSizeMax(This,riid,pv,dwDestContext,pvDestContext,mshlflags,pSize) ) 

#define IMarshal2_MarshalInterface(This,pStm,riid,pv,dwDestContext,pvDestContext,mshlflags)	\
    ( (This)->lpVtbl -> MarshalInterface(This,pStm,riid,pv,dwDestContext,pvDestContext,mshlflags) ) 

#define IMarshal2_UnmarshalInterface(This,pStm,riid,ppv)	\
    ( (This)->lpVtbl -> UnmarshalInterface(This,pStm,riid,ppv) ) 

#define IMarshal2_ReleaseMarshalData(This,pStm)	\
    ( (This)->lpVtbl -> ReleaseMarshalData(This,pStm) ) 

#define IMarshal2_DisconnectObject(This,dwReserved)	\
    ( (This)->lpVtbl -> DisconnectObject(This,dwReserved) ) 


#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IMarshal2_INTERFACE_DEFINED__ */


#ifndef __IMalloc_INTERFACE_DEFINED__
#define __IMalloc_INTERFACE_DEFINED__

/* interface IMalloc */
/* [uuid][object][local] */ 

typedef /* [unique] */ IMalloc *LPMALLOC;


EXTERN_C const IID IID_IMalloc;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("00000002-0000-0000-C000-000000000046")
    IMalloc : public IUnknown
    {
    public:
        virtual void *STDMETHODCALLTYPE Alloc( 
            /* [annotation][in] */ 
            _In_  SIZE_T cb) = 0;
        
        virtual void *STDMETHODCALLTYPE Realloc( 
            /* [annotation][in] */ 
            _In_opt_  void *pv,
            /* [annotation][in] */ 
            _In_  SIZE_T cb) = 0;
        
        virtual void STDMETHODCALLTYPE Free( 
            /* [annotation][in] */ 
            _In_opt_  void *pv) = 0;
        
        virtual SIZE_T STDMETHODCALLTYPE GetSize( 
            /* [annotation][in] */ 
            _In_opt_ _Post_writable_byte_size_(return)  void *pv) = 0;
        
        virtual int STDMETHODCALLTYPE DidAlloc( 
            /* [annotation][in] */ 
            _In_opt_  void *pv) = 0;
        
        virtual void STDMETHODCALLTYPE HeapMinimize( void) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IMallocVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IMalloc * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IMalloc * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IMalloc * This);
        
        DECLSPEC_XFGVIRT(IMalloc, Alloc)
        void *( STDMETHODCALLTYPE *Alloc )( 
            IMalloc * This,
            /* [annotation][in] */ 
            _In_  SIZE_T cb);
        
        DECLSPEC_XFGVIRT(IMalloc, Realloc)
        void *( STDMETHODCALLTYPE *Realloc )( 
            IMalloc * This,
            /* [annotation][in] */ 
            _In_opt_  void *pv,
            /* [annotation][in] */ 
            _In_  SIZE_T cb);
        
        DECLSPEC_XFGVIRT(IMalloc, Free)
        void ( STDMETHODCALLTYPE *Free )( 
            IMalloc * This,
            /* [annotation][in] */ 
            _In_opt_  void *pv);
        
        DECLSPEC_XFGVIRT(IMalloc, GetSize)
        SIZE_T ( STDMETHODCALLTYPE *GetSize )( 
            IMalloc * This,
            /* [annotation][in] */ 
            _In_opt_ _Post_writable_byte_size_(return)  void *pv);
        
        DECLSPEC_XFGVIRT(IMalloc, DidAlloc)
        int ( STDMETHODCALLTYPE *DidAlloc )( 
            IMalloc * This,
            /* [annotation][in] */ 
            _In_opt_  void *pv);
        
        DECLSPEC_XFGVIRT(IMalloc, HeapMinimize)
        void ( STDMETHODCALLTYPE *HeapMinimize )( 
            IMalloc * This);
        
        END_INTERFACE
    } IMallocVtbl;

    interface IMalloc
    {
        CONST_VTBL struct IMallocVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IMalloc_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IMalloc_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IMalloc_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IMalloc_Alloc(This,cb)	\
    ( (This)->lpVtbl -> Alloc(This,cb) ) 

#define IMalloc_Realloc(This,pv,cb)	\
    ( (This)->lpVtbl -> Realloc(This,pv,cb) ) 

#define IMalloc_Free(This,pv)	\
    ( (This)->lpVtbl -> Free(This,pv) ) 

#define IMalloc_GetSize(This,pv)	\
    ( (This)->lpVtbl -> GetSize(This,pv) ) 

#define IMalloc_DidAlloc(This,pv)	\
    ( (This)->lpVtbl -> DidAlloc(This,pv) ) 

#define IMalloc_HeapMinimize(This)	\
    ( (This)->lpVtbl -> HeapMinimize(This) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IMalloc_INTERFACE_DEFINED__ */


#ifndef __IStdMarshalInfo_INTERFACE_DEFINED__
#define __IStdMarshalInfo_INTERFACE_DEFINED__

/* interface IStdMarshalInfo */
/* [uuid][object][local] */ 

typedef /* [unique] */ IStdMarshalInfo *LPSTDMARSHALINFO;


EXTERN_C const IID IID_IStdMarshalInfo;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("00000018-0000-0000-C000-000000000046")
    IStdMarshalInfo : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetClassForHandler( 
            /* [annotation][in] */ 
            _In_  DWORD dwDestContext,
            /* [annotation][unique][in] */ 
            _Reserved_  void *pvDestContext,
            /* [annotation][out] */ 
            _Out_  CLSID *pClsid) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IStdMarshalInfoVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IStdMarshalInfo * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IStdMarshalInfo * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IStdMarshalInfo * This);
        
        DECLSPEC_XFGVIRT(IStdMarshalInfo, GetClassForHandler)
        HRESULT ( STDMETHODCALLTYPE *GetClassForHandler )( 
            IStdMarshalInfo * This,
            /* [annotation][in] */ 
            _In_  DWORD dwDestContext,
            /* [annotation][unique][in] */ 
            _Reserved_  void *pvDestContext,
            /* [annotation][out] */ 
            _Out_  CLSID *pClsid);
        
        END_INTERFACE
    } IStdMarshalInfoVtbl;

    interface IStdMarshalInfo
    {
        CONST_VTBL struct IStdMarshalInfoVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IStdMarshalInfo_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IStdMarshalInfo_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IStdMarshalInfo_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IStdMarshalInfo_GetClassForHandler(This,dwDestContext,pvDestContext,pClsid)	\
    ( (This)->lpVtbl -> GetClassForHandler(This,dwDestContext,pvDestContext,pClsid) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IStdMarshalInfo_INTERFACE_DEFINED__ */


#ifndef __IExternalConnection_INTERFACE_DEFINED__
#define __IExternalConnection_INTERFACE_DEFINED__

/* interface IExternalConnection */
/* [uuid][local][object] */ 

typedef /* [unique] */ IExternalConnection *LPEXTERNALCONNECTION;

typedef 
enum tagEXTCONN
    {
        EXTCONN_STRONG	= 0x1,
        EXTCONN_WEAK	= 0x2,
        EXTCONN_CALLABLE	= 0x4
    } 	EXTCONN;


EXTERN_C const IID IID_IExternalConnection;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("00000019-0000-0000-C000-000000000046")
    IExternalConnection : public IUnknown
    {
    public:
        virtual DWORD STDMETHODCALLTYPE AddConnection( 
            /* [annotation][in] */ 
            _In_  DWORD extconn,
            /* [annotation][in] */ 
            _In_  DWORD reserved) = 0;
        
        virtual DWORD STDMETHODCALLTYPE ReleaseConnection( 
            /* [annotation][in] */ 
            _In_  DWORD extconn,
            /* [annotation][in] */ 
            _In_  DWORD reserved,
            /* [annotation][in] */ 
            _In_  BOOL fLastReleaseCloses) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IExternalConnectionVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IExternalConnection * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IExternalConnection * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IExternalConnection * This);
        
        DECLSPEC_XFGVIRT(IExternalConnection, AddConnection)
        DWORD ( STDMETHODCALLTYPE *AddConnection )( 
            IExternalConnection * This,
            /* [annotation][in] */ 
            _In_  DWORD extconn,
            /* [annotation][in] */ 
            _In_  DWORD reserved);
        
        DECLSPEC_XFGVIRT(IExternalConnection, ReleaseConnection)
        DWORD ( STDMETHODCALLTYPE *ReleaseConnection )( 
            IExternalConnection * This,
            /* [annotation][in] */ 
            _In_  DWORD extconn,
            /* [annotation][in] */ 
            _In_  DWORD reserved,
            /* [annotation][in] */ 
            _In_  BOOL fLastReleaseCloses);
        
        END_INTERFACE
    } IExternalConnectionVtbl;

    interface IExternalConnection
    {
        CONST_VTBL struct IExternalConnectionVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IExternalConnection_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IExternalConnection_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IExternalConnection_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IExternalConnection_AddConnection(This,extconn,reserved)	\
    ( (This)->lpVtbl -> AddConnection(This,extconn,reserved) ) 

#define IExternalConnection_ReleaseConnection(This,extconn,reserved,fLastReleaseCloses)	\
    ( (This)->lpVtbl -> ReleaseConnection(This,extconn,reserved,fLastReleaseCloses) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IExternalConnection_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_objidl_0000_0008 */
/* [local] */ 

typedef /* [unique] */  __RPC_unique_pointer IMultiQI *LPMULTIQI;

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES) */
#pragma endregion
#pragma region Application Family or OneCore Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES)
typedef struct tagMULTI_QI
    {
    const IID *pIID;
    IUnknown *pItf;
    HRESULT hr;
    } 	MULTI_QI;



extern RPC_IF_HANDLE __MIDL_itf_objidl_0000_0008_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_objidl_0000_0008_v0_0_s_ifspec;

#ifndef __IMultiQI_INTERFACE_DEFINED__
#define __IMultiQI_INTERFACE_DEFINED__

/* interface IMultiQI */
/* [async_uuid][uuid][local][object] */ 


EXTERN_C const IID IID_IMultiQI;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("00000020-0000-0000-C000-000000000046")
    IMultiQI : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE QueryMultipleInterfaces( 
            /* [annotation][in] */ 
            _In_  ULONG cMQIs,
            /* [annotation][out][in] */ 
            _Inout_updates_(cMQIs)  MULTI_QI *pMQIs) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IMultiQIVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IMultiQI * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IMultiQI * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IMultiQI * This);
        
        DECLSPEC_XFGVIRT(IMultiQI, QueryMultipleInterfaces)
        HRESULT ( STDMETHODCALLTYPE *QueryMultipleInterfaces )( 
            IMultiQI * This,
            /* [annotation][in] */ 
            _In_  ULONG cMQIs,
            /* [annotation][out][in] */ 
            _Inout_updates_(cMQIs)  MULTI_QI *pMQIs);
        
        END_INTERFACE
    } IMultiQIVtbl;

    interface IMultiQI
    {
        CONST_VTBL struct IMultiQIVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IMultiQI_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IMultiQI_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IMultiQI_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IMultiQI_QueryMultipleInterfaces(This,cMQIs,pMQIs)	\
    ( (This)->lpVtbl -> QueryMultipleInterfaces(This,cMQIs,pMQIs) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IMultiQI_INTERFACE_DEFINED__ */


#ifndef __AsyncIMultiQI_INTERFACE_DEFINED__
#define __AsyncIMultiQI_INTERFACE_DEFINED__

/* interface AsyncIMultiQI */
/* [uuid][local][object] */ 


EXTERN_C const IID IID_AsyncIMultiQI;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("000e0020-0000-0000-C000-000000000046")
    AsyncIMultiQI : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE Begin_QueryMultipleInterfaces( 
            /* [annotation][in] */ 
            _In_  ULONG cMQIs,
            /* [annotation][out][in] */ 
            _Inout_updates_(cMQIs)  MULTI_QI *pMQIs) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Finish_QueryMultipleInterfaces( 
            /* [annotation][out][in] */ 
            _Inout_updates_(cMQIs)  MULTI_QI *pMQIs) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct AsyncIMultiQIVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            AsyncIMultiQI * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            AsyncIMultiQI * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            AsyncIMultiQI * This);
        
        DECLSPEC_XFGVIRT(AsyncIMultiQI, Begin_QueryMultipleInterfaces)
        HRESULT ( STDMETHODCALLTYPE *Begin_QueryMultipleInterfaces )( 
            AsyncIMultiQI * This,
            /* [annotation][in] */ 
            _In_  ULONG cMQIs,
            /* [annotation][out][in] */ 
            _Inout_updates_(cMQIs)  MULTI_QI *pMQIs);
        
        DECLSPEC_XFGVIRT(AsyncIMultiQI, Finish_QueryMultipleInterfaces)
        HRESULT ( STDMETHODCALLTYPE *Finish_QueryMultipleInterfaces )( 
            AsyncIMultiQI * This,
            /* [annotation][out][in] */ 
            _Inout_updates_(cMQIs)  MULTI_QI *pMQIs);
        
        END_INTERFACE
    } AsyncIMultiQIVtbl;

    interface AsyncIMultiQI
    {
        CONST_VTBL struct AsyncIMultiQIVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define AsyncIMultiQI_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define AsyncIMultiQI_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define AsyncIMultiQI_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define AsyncIMultiQI_Begin_QueryMultipleInterfaces(This,cMQIs,pMQIs)	\
    ( (This)->lpVtbl -> Begin_QueryMultipleInterfaces(This,cMQIs,pMQIs) ) 

#define AsyncIMultiQI_Finish_QueryMultipleInterfaces(This,pMQIs)	\
    ( (This)->lpVtbl -> Finish_QueryMultipleInterfaces(This,pMQIs) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __AsyncIMultiQI_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_objidl_0000_0009 */
/* [local] */ 

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES) */
#pragma endregion
#pragma region Desktop Family or OneCore Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES)


extern RPC_IF_HANDLE __MIDL_itf_objidl_0000_0009_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_objidl_0000_0009_v0_0_s_ifspec;

#ifndef __IInternalUnknown_INTERFACE_DEFINED__
#define __IInternalUnknown_INTERFACE_DEFINED__

/* interface IInternalUnknown */
/* [uuid][local][object] */ 


EXTERN_C const IID IID_IInternalUnknown;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("00000021-0000-0000-C000-000000000046")
    IInternalUnknown : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE QueryInternalInterface( 
            /* [annotation][in] */ 
            _In_  REFIID riid,
            /* [annotation][out] */ 
            _Outptr_  void **ppv) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IInternalUnknownVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IInternalUnknown * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IInternalUnknown * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IInternalUnknown * This);
        
        DECLSPEC_XFGVIRT(IInternalUnknown, QueryInternalInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInternalInterface )( 
            IInternalUnknown * This,
            /* [annotation][in] */ 
            _In_  REFIID riid,
            /* [annotation][out] */ 
            _Outptr_  void **ppv);
        
        END_INTERFACE
    } IInternalUnknownVtbl;

    interface IInternalUnknown
    {
        CONST_VTBL struct IInternalUnknownVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IInternalUnknown_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IInternalUnknown_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IInternalUnknown_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IInternalUnknown_QueryInternalInterface(This,riid,ppv)	\
    ( (This)->lpVtbl -> QueryInternalInterface(This,riid,ppv) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IInternalUnknown_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_objidl_0000_0010 */
/* [local] */ 

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES) */
#pragma endregion
#pragma region Application Family or OneCore Family or OneCore Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES)


extern RPC_IF_HANDLE __MIDL_itf_objidl_0000_0010_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_objidl_0000_0010_v0_0_s_ifspec;

#ifndef __IEnumUnknown_INTERFACE_DEFINED__
#define __IEnumUnknown_INTERFACE_DEFINED__

/* interface IEnumUnknown */
/* [unique][uuid][object] */ 

typedef /* [unique] */  __RPC_unique_pointer IEnumUnknown *LPENUMUNKNOWN;


EXTERN_C const IID IID_IEnumUnknown;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("00000100-0000-0000-C000-000000000046")
    IEnumUnknown : public IUnknown
    {
    public:
        virtual /* [local] */ HRESULT STDMETHODCALLTYPE Next( 
            /* [annotation][in] */ 
            _In_  ULONG celt,
            /* [annotation][out] */ 
            _Out_writes_to_(celt,*pceltFetched)  IUnknown **rgelt,
            /* [annotation][out] */ 
            _Out_opt_  ULONG *pceltFetched) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Skip( 
            /* [in] */ ULONG celt) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Reset( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Clone( 
            /* [out] */ __RPC__deref_out_opt IEnumUnknown **ppenum) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IEnumUnknownVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IEnumUnknown * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IEnumUnknown * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IEnumUnknown * This);
        
        DECLSPEC_XFGVIRT(IEnumUnknown, Next)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Next )( 
            IEnumUnknown * This,
            /* [annotation][in] */ 
            _In_  ULONG celt,
            /* [annotation][out] */ 
            _Out_writes_to_(celt,*pceltFetched)  IUnknown **rgelt,
            /* [annotation][out] */ 
            _Out_opt_  ULONG *pceltFetched);
        
        DECLSPEC_XFGVIRT(IEnumUnknown, Skip)
        HRESULT ( STDMETHODCALLTYPE *Skip )( 
            __RPC__in IEnumUnknown * This,
            /* [in] */ ULONG celt);
        
        DECLSPEC_XFGVIRT(IEnumUnknown, Reset)
        HRESULT ( STDMETHODCALLTYPE *Reset )( 
            __RPC__in IEnumUnknown * This);
        
        DECLSPEC_XFGVIRT(IEnumUnknown, Clone)
        HRESULT ( STDMETHODCALLTYPE *Clone )( 
            __RPC__in IEnumUnknown * This,
            /* [out] */ __RPC__deref_out_opt IEnumUnknown **ppenum);
        
        END_INTERFACE
    } IEnumUnknownVtbl;

    interface IEnumUnknown
    {
        CONST_VTBL struct IEnumUnknownVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IEnumUnknown_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IEnumUnknown_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IEnumUnknown_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IEnumUnknown_Next(This,celt,rgelt,pceltFetched)	\
    ( (This)->lpVtbl -> Next(This,celt,rgelt,pceltFetched) ) 

#define IEnumUnknown_Skip(This,celt)	\
    ( (This)->lpVtbl -> Skip(This,celt) ) 

#define IEnumUnknown_Reset(This)	\
    ( (This)->lpVtbl -> Reset(This) ) 

#define IEnumUnknown_Clone(This,ppenum)	\
    ( (This)->lpVtbl -> Clone(This,ppenum) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */



/* [call_as] */ HRESULT STDMETHODCALLTYPE IEnumUnknown_RemoteNext_Proxy( 
    __RPC__in IEnumUnknown * This,
    /* [in] */ ULONG celt,
    /* [length_is][size_is][out] */ __RPC__out_ecount_part(celt, *pceltFetched) IUnknown **rgelt,
    /* [out] */ __RPC__out ULONG *pceltFetched);


void __RPC_STUB IEnumUnknown_RemoteNext_Stub(
    IRpcStubBuffer *This,
    IRpcChannelBuffer *_pRpcChannelBuffer,
    PRPC_MESSAGE _pRpcMessage,
    DWORD *_pdwStubPhase);



#endif 	/* __IEnumUnknown_INTERFACE_DEFINED__ */


#ifndef __IEnumString_INTERFACE_DEFINED__
#define __IEnumString_INTERFACE_DEFINED__

/* interface IEnumString */
/* [unique][uuid][object] */ 

typedef /* [unique] */  __RPC_unique_pointer IEnumString *LPENUMSTRING;


EXTERN_C const IID IID_IEnumString;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("00000101-0000-0000-C000-000000000046")
    IEnumString : public IUnknown
    {
    public:
        virtual /* [local] */ HRESULT STDMETHODCALLTYPE Next( 
            /* [in] */ ULONG celt,
            /* [annotation] */ 
            _Out_writes_to_(celt,*pceltFetched)  LPOLESTR *rgelt,
            /* [annotation] */ 
            _Out_opt_  ULONG *pceltFetched) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Skip( 
            /* [in] */ ULONG celt) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Reset( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Clone( 
            /* [out] */ __RPC__deref_out_opt IEnumString **ppenum) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IEnumStringVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IEnumString * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IEnumString * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IEnumString * This);
        
        DECLSPEC_XFGVIRT(IEnumString, Next)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Next )( 
            IEnumString * This,
            /* [in] */ ULONG celt,
            /* [annotation] */ 
            _Out_writes_to_(celt,*pceltFetched)  LPOLESTR *rgelt,
            /* [annotation] */ 
            _Out_opt_  ULONG *pceltFetched);
        
        DECLSPEC_XFGVIRT(IEnumString, Skip)
        HRESULT ( STDMETHODCALLTYPE *Skip )( 
            __RPC__in IEnumString * This,
            /* [in] */ ULONG celt);
        
        DECLSPEC_XFGVIRT(IEnumString, Reset)
        HRESULT ( STDMETHODCALLTYPE *Reset )( 
            __RPC__in IEnumString * This);
        
        DECLSPEC_XFGVIRT(IEnumString, Clone)
        HRESULT ( STDMETHODCALLTYPE *Clone )( 
            __RPC__in IEnumString * This,
            /* [out] */ __RPC__deref_out_opt IEnumString **ppenum);
        
        END_INTERFACE
    } IEnumStringVtbl;

    interface IEnumString
    {
        CONST_VTBL struct IEnumStringVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IEnumString_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IEnumString_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IEnumString_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IEnumString_Next(This,celt,rgelt,pceltFetched)	\
    ( (This)->lpVtbl -> Next(This,celt,rgelt,pceltFetched) ) 

#define IEnumString_Skip(This,celt)	\
    ( (This)->lpVtbl -> Skip(This,celt) ) 

#define IEnumString_Reset(This)	\
    ( (This)->lpVtbl -> Reset(This) ) 

#define IEnumString_Clone(This,ppenum)	\
    ( (This)->lpVtbl -> Clone(This,ppenum) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */



/* [call_as] */ HRESULT STDMETHODCALLTYPE IEnumString_RemoteNext_Proxy( 
    __RPC__in IEnumString * This,
    /* [in] */ ULONG celt,
    /* [length_is][size_is][out] */ __RPC__out_ecount_part(celt, *pceltFetched) LPOLESTR *rgelt,
    /* [out] */ __RPC__out ULONG *pceltFetched);


void __RPC_STUB IEnumString_RemoteNext_Stub(
    IRpcStubBuffer *This,
    IRpcChannelBuffer *_pRpcChannelBuffer,
    PRPC_MESSAGE _pRpcMessage,
    DWORD *_pdwStubPhase);



#endif 	/* __IEnumString_INTERFACE_DEFINED__ */


#ifndef __ISequentialStream_INTERFACE_DEFINED__
#define __ISequentialStream_INTERFACE_DEFINED__

/* interface ISequentialStream */
/* [unique][uuid][object] */ 


EXTERN_C const IID IID_ISequentialStream;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("0c733a30-2a1c-11ce-ade5-00aa0044773d")
    ISequentialStream : public IUnknown
    {
    public:
        virtual /* [local] */ HRESULT STDMETHODCALLTYPE Read( 
            /* [annotation] */ 
            _Out_writes_bytes_to_(cb, *pcbRead)  void *pv,
            /* [annotation][in] */ 
            _In_  ULONG cb,
            /* [annotation] */ 
            _Out_opt_  ULONG *pcbRead) = 0;
        
        virtual /* [local] */ HRESULT STDMETHODCALLTYPE Write( 
            /* [annotation] */ 
            _In_reads_bytes_(cb)  const void *pv,
            /* [annotation][in] */ 
            _In_  ULONG cb,
            /* [annotation] */ 
            _Out_opt_  ULONG *pcbWritten) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ISequentialStreamVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in ISequentialStream * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in ISequentialStream * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in ISequentialStream * This);
        
        DECLSPEC_XFGVIRT(ISequentialStream, Read)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Read )( 
            ISequentialStream * This,
            /* [annotation] */ 
            _Out_writes_bytes_to_(cb, *pcbRead)  void *pv,
            /* [annotation][in] */ 
            _In_  ULONG cb,
            /* [annotation] */ 
            _Out_opt_  ULONG *pcbRead);
        
        DECLSPEC_XFGVIRT(ISequentialStream, Write)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Write )( 
            ISequentialStream * This,
            /* [annotation] */ 
            _In_reads_bytes_(cb)  const void *pv,
            /* [annotation][in] */ 
            _In_  ULONG cb,
            /* [annotation] */ 
            _Out_opt_  ULONG *pcbWritten);
        
        END_INTERFACE
    } ISequentialStreamVtbl;

    interface ISequentialStream
    {
        CONST_VTBL struct ISequentialStreamVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ISequentialStream_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ISequentialStream_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ISequentialStream_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ISequentialStream_Read(This,pv,cb,pcbRead)	\
    ( (This)->lpVtbl -> Read(This,pv,cb,pcbRead) ) 

#define ISequentialStream_Write(This,pv,cb,pcbWritten)	\
    ( (This)->lpVtbl -> Write(This,pv,cb,pcbWritten) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */



/* [call_as] */ HRESULT STDMETHODCALLTYPE ISequentialStream_RemoteRead_Proxy( 
    __RPC__in ISequentialStream * This,
    /* [length_is][size_is][out] */ __RPC__out_ecount_part(cb, *pcbRead) byte *pv,
    /* [in] */ ULONG cb,
    /* [out] */ __RPC__out ULONG *pcbRead);


void __RPC_STUB ISequentialStream_RemoteRead_Stub(
    IRpcStubBuffer *This,
    IRpcChannelBuffer *_pRpcChannelBuffer,
    PRPC_MESSAGE _pRpcMessage,
    DWORD *_pdwStubPhase);


/* [call_as] */ HRESULT STDMETHODCALLTYPE ISequentialStream_RemoteWrite_Proxy( 
    __RPC__in ISequentialStream * This,
    /* [size_is][in] */ __RPC__in_ecount_full(cb) const byte *pv,
    /* [in] */ ULONG cb,
    /* [out] */ __RPC__out ULONG *pcbWritten);


void __RPC_STUB ISequentialStream_RemoteWrite_Stub(
    IRpcStubBuffer *This,
    IRpcChannelBuffer *_pRpcChannelBuffer,
    PRPC_MESSAGE _pRpcMessage,
    DWORD *_pdwStubPhase);



#endif 	/* __ISequentialStream_INTERFACE_DEFINED__ */


#ifndef __IStream_INTERFACE_DEFINED__
#define __IStream_INTERFACE_DEFINED__

/* interface IStream */
/* [unique][uuid][object] */ 

typedef /* [unique] */  __RPC_unique_pointer IStream *LPSTREAM;

typedef struct tagSTATSTG
    {
    LPOLESTR pwcsName;
    DWORD type;
    ULARGE_INTEGER cbSize;
    FILETIME mtime;
    FILETIME ctime;
    FILETIME atime;
    DWORD grfMode;
    DWORD grfLocksSupported;
    CLSID clsid;
    DWORD grfStateBits;
    DWORD reserved;
    } 	STATSTG;

typedef 
enum tagSTGTY
    {
        STGTY_STORAGE	= 1,
        STGTY_STREAM	= 2,
        STGTY_LOCKBYTES	= 3,
        STGTY_PROPERTY	= 4
    } 	STGTY;

typedef 
enum tagSTREAM_SEEK
    {
        STREAM_SEEK_SET	= 0,
        STREAM_SEEK_CUR	= 1,
        STREAM_SEEK_END	= 2
    } 	STREAM_SEEK;

typedef 
enum tagLOCKTYPE
    {
        LOCK_WRITE	= 1,
        LOCK_EXCLUSIVE	= 2,
        LOCK_ONLYONCE	= 4
    } 	LOCKTYPE;


EXTERN_C const IID IID_IStream;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("0000000c-0000-0000-C000-000000000046")
    IStream : public ISequentialStream
    {
    public:
        virtual /* [local] */ HRESULT STDMETHODCALLTYPE Seek( 
            /* [in] */ LARGE_INTEGER dlibMove,
            /* [in] */ DWORD dwOrigin,
            /* [annotation] */ 
            _Out_opt_  ULARGE_INTEGER *plibNewPosition) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetSize( 
            /* [in] */ ULARGE_INTEGER libNewSize) = 0;
        
        virtual /* [local] */ HRESULT STDMETHODCALLTYPE CopyTo( 
            /* [annotation][unique][in] */ 
            _In_  IStream *pstm,
            /* [in] */ ULARGE_INTEGER cb,
            /* [annotation] */ 
            _Out_opt_  ULARGE_INTEGER *pcbRead,
            /* [annotation] */ 
            _Out_opt_  ULARGE_INTEGER *pcbWritten) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Commit( 
            /* [in] */ DWORD grfCommitFlags) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Revert( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE LockRegion( 
            /* [in] */ ULARGE_INTEGER libOffset,
            /* [in] */ ULARGE_INTEGER cb,
            /* [in] */ DWORD dwLockType) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE UnlockRegion( 
            /* [in] */ ULARGE_INTEGER libOffset,
            /* [in] */ ULARGE_INTEGER cb,
            /* [in] */ DWORD dwLockType) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Stat( 
            /* [out] */ __RPC__out STATSTG *pstatstg,
            /* [in] */ DWORD grfStatFlag) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Clone( 
            /* [out] */ __RPC__deref_out_opt IStream **ppstm) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IStreamVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IStream * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IStream * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IStream * This);
        
        DECLSPEC_XFGVIRT(ISequentialStream, Read)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Read )( 
            IStream * This,
            /* [annotation] */ 
            _Out_writes_bytes_to_(cb, *pcbRead)  void *pv,
            /* [annotation][in] */ 
            _In_  ULONG cb,
            /* [annotation] */ 
            _Out_opt_  ULONG *pcbRead);
        
        DECLSPEC_XFGVIRT(ISequentialStream, Write)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Write )( 
            IStream * This,
            /* [annotation] */ 
            _In_reads_bytes_(cb)  const void *pv,
            /* [annotation][in] */ 
            _In_  ULONG cb,
            /* [annotation] */ 
            _Out_opt_  ULONG *pcbWritten);
        
        DECLSPEC_XFGVIRT(IStream, Seek)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Seek )( 
            IStream * This,
            /* [in] */ LARGE_INTEGER dlibMove,
            /* [in] */ DWORD dwOrigin,
            /* [annotation] */ 
            _Out_opt_  ULARGE_INTEGER *plibNewPosition);
        
        DECLSPEC_XFGVIRT(IStream, SetSize)
        HRESULT ( STDMETHODCALLTYPE *SetSize )( 
            __RPC__in IStream * This,
            /* [in] */ ULARGE_INTEGER libNewSize);
        
        DECLSPEC_XFGVIRT(IStream, CopyTo)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *CopyTo )( 
            IStream * This,
            /* [annotation][unique][in] */ 
            _In_  IStream *pstm,
            /* [in] */ ULARGE_INTEGER cb,
            /* [annotation] */ 
            _Out_opt_  ULARGE_INTEGER *pcbRead,
            /* [annotation] */ 
            _Out_opt_  ULARGE_INTEGER *pcbWritten);
        
        DECLSPEC_XFGVIRT(IStream, Commit)
        HRESULT ( STDMETHODCALLTYPE *Commit )( 
            __RPC__in IStream * This,
            /* [in] */ DWORD grfCommitFlags);
        
        DECLSPEC_XFGVIRT(IStream, Revert)
        HRESULT ( STDMETHODCALLTYPE *Revert )( 
            __RPC__in IStream * This);
        
        DECLSPEC_XFGVIRT(IStream, LockRegion)
        HRESULT ( STDMETHODCALLTYPE *LockRegion )( 
            __RPC__in IStream * This,
            /* [in] */ ULARGE_INTEGER libOffset,
            /* [in] */ ULARGE_INTEGER cb,
            /* [in] */ DWORD dwLockType);
        
        DECLSPEC_XFGVIRT(IStream, UnlockRegion)
        HRESULT ( STDMETHODCALLTYPE *UnlockRegion )( 
            __RPC__in IStream * This,
            /* [in] */ ULARGE_INTEGER libOffset,
            /* [in] */ ULARGE_INTEGER cb,
            /* [in] */ DWORD dwLockType);
        
        DECLSPEC_XFGVIRT(IStream, Stat)
        HRESULT ( STDMETHODCALLTYPE *Stat )( 
            __RPC__in IStream * This,
            /* [out] */ __RPC__out STATSTG *pstatstg,
            /* [in] */ DWORD grfStatFlag);
        
        DECLSPEC_XFGVIRT(IStream, Clone)
        HRESULT ( STDMETHODCALLTYPE *Clone )( 
            __RPC__in IStream * This,
            /* [out] */ __RPC__deref_out_opt IStream **ppstm);
        
        END_INTERFACE
    } IStreamVtbl;

    interface IStream
    {
        CONST_VTBL struct IStreamVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IStream_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IStream_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IStream_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IStream_Read(This,pv,cb,pcbRead)	\
    ( (This)->lpVtbl -> Read(This,pv,cb,pcbRead) ) 

#define IStream_Write(This,pv,cb,pcbWritten)	\
    ( (This)->lpVtbl -> Write(This,pv,cb,pcbWritten) ) 


#define IStream_Seek(This,dlibMove,dwOrigin,plibNewPosition)	\
    ( (This)->lpVtbl -> Seek(This,dlibMove,dwOrigin,plibNewPosition) ) 

#define IStream_SetSize(This,libNewSize)	\
    ( (This)->lpVtbl -> SetSize(This,libNewSize) ) 

#define IStream_CopyTo(This,pstm,cb,pcbRead,pcbWritten)	\
    ( (This)->lpVtbl -> CopyTo(This,pstm,cb,pcbRead,pcbWritten) ) 

#define IStream_Commit(This,grfCommitFlags)	\
    ( (This)->lpVtbl -> Commit(This,grfCommitFlags) ) 

#define IStream_Revert(This)	\
    ( (This)->lpVtbl -> Revert(This) ) 

#define IStream_LockRegion(This,libOffset,cb,dwLockType)	\
    ( (This)->lpVtbl -> LockRegion(This,libOffset,cb,dwLockType) ) 

#define IStream_UnlockRegion(This,libOffset,cb,dwLockType)	\
    ( (This)->lpVtbl -> UnlockRegion(This,libOffset,cb,dwLockType) ) 

#define IStream_Stat(This,pstatstg,grfStatFlag)	\
    ( (This)->lpVtbl -> Stat(This,pstatstg,grfStatFlag) ) 

#define IStream_Clone(This,ppstm)	\
    ( (This)->lpVtbl -> Clone(This,ppstm) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */



/* [call_as] */ HRESULT STDMETHODCALLTYPE IStream_RemoteSeek_Proxy( 
    __RPC__in IStream * This,
    /* [in] */ LARGE_INTEGER dlibMove,
    /* [in] */ DWORD dwOrigin,
    /* [out] */ __RPC__out ULARGE_INTEGER *plibNewPosition);


void __RPC_STUB IStream_RemoteSeek_Stub(
    IRpcStubBuffer *This,
    IRpcChannelBuffer *_pRpcChannelBuffer,
    PRPC_MESSAGE _pRpcMessage,
    DWORD *_pdwStubPhase);


/* [call_as] */ HRESULT STDMETHODCALLTYPE IStream_RemoteCopyTo_Proxy( 
    __RPC__in IStream * This,
    /* [unique][in] */ __RPC__in_opt IStream *pstm,
    /* [in] */ ULARGE_INTEGER cb,
    /* [out] */ __RPC__out ULARGE_INTEGER *pcbRead,
    /* [out] */ __RPC__out ULARGE_INTEGER *pcbWritten);


void __RPC_STUB IStream_RemoteCopyTo_Stub(
    IRpcStubBuffer *This,
    IRpcChannelBuffer *_pRpcChannelBuffer,
    PRPC_MESSAGE _pRpcMessage,
    DWORD *_pdwStubPhase);



#endif 	/* __IStream_INTERFACE_DEFINED__ */


#ifndef __IRpcChannelBuffer_INTERFACE_DEFINED__
#define __IRpcChannelBuffer_INTERFACE_DEFINED__

/* interface IRpcChannelBuffer */
/* [uuid][object][local] */ 

typedef ULONG RPCOLEDATAREP;

typedef struct tagRPCOLEMESSAGE
    {
    void *reserved1;
    RPCOLEDATAREP dataRepresentation;
    void *Buffer;
    ULONG cbBuffer;
    ULONG iMethod;
    void *reserved2[ 5 ];
    ULONG rpcFlags;
    } 	RPCOLEMESSAGE;

typedef RPCOLEMESSAGE *PRPCOLEMESSAGE;


EXTERN_C const IID IID_IRpcChannelBuffer;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("D5F56B60-593B-101A-B569-08002B2DBF7A")
    IRpcChannelBuffer : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetBuffer( 
            /* [annotation][out][in] */ 
            _Inout_  RPCOLEMESSAGE *pMessage,
            /* [annotation][in] */ 
            _In_  REFIID riid) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SendReceive( 
            /* [annotation][out][in] */ 
            _Inout_  RPCOLEMESSAGE *pMessage,
            /* [annotation][out] */ 
            _Out_opt_  ULONG *pStatus) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE FreeBuffer( 
            /* [annotation][out][in] */ 
            _Inout_  RPCOLEMESSAGE *pMessage) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetDestCtx( 
            /* [annotation][out] */ 
            _Out_  DWORD *pdwDestContext,
            /* [annotation][out] */ 
            _Outptr_result_maybenull_  void **ppvDestContext) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE IsConnected( void) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IRpcChannelBufferVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IRpcChannelBuffer * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IRpcChannelBuffer * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IRpcChannelBuffer * This);
        
        DECLSPEC_XFGVIRT(IRpcChannelBuffer, GetBuffer)
        HRESULT ( STDMETHODCALLTYPE *GetBuffer )( 
            IRpcChannelBuffer * This,
            /* [annotation][out][in] */ 
            _Inout_  RPCOLEMESSAGE *pMessage,
            /* [annotation][in] */ 
            _In_  REFIID riid);
        
        DECLSPEC_XFGVIRT(IRpcChannelBuffer, SendReceive)
        HRESULT ( STDMETHODCALLTYPE *SendReceive )( 
            IRpcChannelBuffer * This,
            /* [annotation][out][in] */ 
            _Inout_  RPCOLEMESSAGE *pMessage,
            /* [annotation][out] */ 
            _Out_opt_  ULONG *pStatus);
        
        DECLSPEC_XFGVIRT(IRpcChannelBuffer, FreeBuffer)
        HRESULT ( STDMETHODCALLTYPE *FreeBuffer )( 
            IRpcChannelBuffer * This,
            /* [annotation][out][in] */ 
            _Inout_  RPCOLEMESSAGE *pMessage);
        
        DECLSPEC_XFGVIRT(IRpcChannelBuffer, GetDestCtx)
        HRESULT ( STDMETHODCALLTYPE *GetDestCtx )( 
            IRpcChannelBuffer * This,
            /* [annotation][out] */ 
            _Out_  DWORD *pdwDestContext,
            /* [annotation][out] */ 
            _Outptr_result_maybenull_  void **ppvDestContext);
        
        DECLSPEC_XFGVIRT(IRpcChannelBuffer, IsConnected)
        HRESULT ( STDMETHODCALLTYPE *IsConnected )( 
            IRpcChannelBuffer * This);
        
        END_INTERFACE
    } IRpcChannelBufferVtbl;

    interface IRpcChannelBuffer
    {
        CONST_VTBL struct IRpcChannelBufferVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IRpcChannelBuffer_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IRpcChannelBuffer_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IRpcChannelBuffer_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IRpcChannelBuffer_GetBuffer(This,pMessage,riid)	\
    ( (This)->lpVtbl -> GetBuffer(This,pMessage,riid) ) 

#define IRpcChannelBuffer_SendReceive(This,pMessage,pStatus)	\
    ( (This)->lpVtbl -> SendReceive(This,pMessage,pStatus) ) 

#define IRpcChannelBuffer_FreeBuffer(This,pMessage)	\
    ( (This)->lpVtbl -> FreeBuffer(This,pMessage) ) 

#define IRpcChannelBuffer_GetDestCtx(This,pdwDestContext,ppvDestContext)	\
    ( (This)->lpVtbl -> GetDestCtx(This,pdwDestContext,ppvDestContext) ) 

#define IRpcChannelBuffer_IsConnected(This)	\
    ( (This)->lpVtbl -> IsConnected(This) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IRpcChannelBuffer_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_objidl_0000_0015 */
/* [local] */ 

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES) */
#pragma endregion
#pragma region Desktop Family or OneCore Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES)


extern RPC_IF_HANDLE __MIDL_itf_objidl_0000_0015_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_objidl_0000_0015_v0_0_s_ifspec;

#ifndef __IRpcChannelBuffer2_INTERFACE_DEFINED__
#define __IRpcChannelBuffer2_INTERFACE_DEFINED__

/* interface IRpcChannelBuffer2 */
/* [uuid][object][local] */ 


EXTERN_C const IID IID_IRpcChannelBuffer2;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("594f31d0-7f19-11d0-b194-00a0c90dc8bf")
    IRpcChannelBuffer2 : public IRpcChannelBuffer
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetProtocolVersion( 
            /* [annotation][out] */ 
            _Out_  DWORD *pdwVersion) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IRpcChannelBuffer2Vtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IRpcChannelBuffer2 * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IRpcChannelBuffer2 * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IRpcChannelBuffer2 * This);
        
        DECLSPEC_XFGVIRT(IRpcChannelBuffer, GetBuffer)
        HRESULT ( STDMETHODCALLTYPE *GetBuffer )( 
            IRpcChannelBuffer2 * This,
            /* [annotation][out][in] */ 
            _Inout_  RPCOLEMESSAGE *pMessage,
            /* [annotation][in] */ 
            _In_  REFIID riid);
        
        DECLSPEC_XFGVIRT(IRpcChannelBuffer, SendReceive)
        HRESULT ( STDMETHODCALLTYPE *SendReceive )( 
            IRpcChannelBuffer2 * This,
            /* [annotation][out][in] */ 
            _Inout_  RPCOLEMESSAGE *pMessage,
            /* [annotation][out] */ 
            _Out_opt_  ULONG *pStatus);
        
        DECLSPEC_XFGVIRT(IRpcChannelBuffer, FreeBuffer)
        HRESULT ( STDMETHODCALLTYPE *FreeBuffer )( 
            IRpcChannelBuffer2 * This,
            /* [annotation][out][in] */ 
            _Inout_  RPCOLEMESSAGE *pMessage);
        
        DECLSPEC_XFGVIRT(IRpcChannelBuffer, GetDestCtx)
        HRESULT ( STDMETHODCALLTYPE *GetDestCtx )( 
            IRpcChannelBuffer2 * This,
            /* [annotation][out] */ 
            _Out_  DWORD *pdwDestContext,
            /* [annotation][out] */ 
            _Outptr_result_maybenull_  void **ppvDestContext);
        
        DECLSPEC_XFGVIRT(IRpcChannelBuffer, IsConnected)
        HRESULT ( STDMETHODCALLTYPE *IsConnected )( 
            IRpcChannelBuffer2 * This);
        
        DECLSPEC_XFGVIRT(IRpcChannelBuffer2, GetProtocolVersion)
        HRESULT ( STDMETHODCALLTYPE *GetProtocolVersion )( 
            IRpcChannelBuffer2 * This,
            /* [annotation][out] */ 
            _Out_  DWORD *pdwVersion);
        
        END_INTERFACE
    } IRpcChannelBuffer2Vtbl;

    interface IRpcChannelBuffer2
    {
        CONST_VTBL struct IRpcChannelBuffer2Vtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IRpcChannelBuffer2_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IRpcChannelBuffer2_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IRpcChannelBuffer2_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IRpcChannelBuffer2_GetBuffer(This,pMessage,riid)	\
    ( (This)->lpVtbl -> GetBuffer(This,pMessage,riid) ) 

#define IRpcChannelBuffer2_SendReceive(This,pMessage,pStatus)	\
    ( (This)->lpVtbl -> SendReceive(This,pMessage,pStatus) ) 

#define IRpcChannelBuffer2_FreeBuffer(This,pMessage)	\
    ( (This)->lpVtbl -> FreeBuffer(This,pMessage) ) 

#define IRpcChannelBuffer2_GetDestCtx(This,pdwDestContext,ppvDestContext)	\
    ( (This)->lpVtbl -> GetDestCtx(This,pdwDestContext,ppvDestContext) ) 

#define IRpcChannelBuffer2_IsConnected(This)	\
    ( (This)->lpVtbl -> IsConnected(This) ) 


#define IRpcChannelBuffer2_GetProtocolVersion(This,pdwVersion)	\
    ( (This)->lpVtbl -> GetProtocolVersion(This,pdwVersion) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IRpcChannelBuffer2_INTERFACE_DEFINED__ */


#ifndef __IAsyncRpcChannelBuffer_INTERFACE_DEFINED__
#define __IAsyncRpcChannelBuffer_INTERFACE_DEFINED__

/* interface IAsyncRpcChannelBuffer */
/* [unique][uuid][object][local] */ 


EXTERN_C const IID IID_IAsyncRpcChannelBuffer;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("a5029fb6-3c34-11d1-9c99-00c04fb998aa")
    IAsyncRpcChannelBuffer : public IRpcChannelBuffer2
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE Send( 
            /* [annotation][out][in] */ 
            _Inout_  RPCOLEMESSAGE *pMsg,
            /* [annotation][in] */ 
            _In_  ISynchronize *pSync,
            /* [annotation][out] */ 
            _Out_  ULONG *pulStatus) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Receive( 
            /* [annotation][out][in] */ 
            _Inout_  RPCOLEMESSAGE *pMsg,
            /* [annotation][out] */ 
            _Out_  ULONG *pulStatus) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetDestCtxEx( 
            /* [annotation][in] */ 
            _In_  RPCOLEMESSAGE *pMsg,
            /* [annotation][out] */ 
            _Out_  DWORD *pdwDestContext,
            /* [annotation][out] */ 
            _Outptr_opt_result_maybenull_  void **ppvDestContext) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IAsyncRpcChannelBufferVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IAsyncRpcChannelBuffer * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IAsyncRpcChannelBuffer * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IAsyncRpcChannelBuffer * This);
        
        DECLSPEC_XFGVIRT(IRpcChannelBuffer, GetBuffer)
        HRESULT ( STDMETHODCALLTYPE *GetBuffer )( 
            IAsyncRpcChannelBuffer * This,
            /* [annotation][out][in] */ 
            _Inout_  RPCOLEMESSAGE *pMessage,
            /* [annotation][in] */ 
            _In_  REFIID riid);
        
        DECLSPEC_XFGVIRT(IRpcChannelBuffer, SendReceive)
        HRESULT ( STDMETHODCALLTYPE *SendReceive )( 
            IAsyncRpcChannelBuffer * This,
            /* [annotation][out][in] */ 
            _Inout_  RPCOLEMESSAGE *pMessage,
            /* [annotation][out] */ 
            _Out_opt_  ULONG *pStatus);
        
        DECLSPEC_XFGVIRT(IRpcChannelBuffer, FreeBuffer)
        HRESULT ( STDMETHODCALLTYPE *FreeBuffer )( 
            IAsyncRpcChannelBuffer * This,
            /* [annotation][out][in] */ 
            _Inout_  RPCOLEMESSAGE *pMessage);
        
        DECLSPEC_XFGVIRT(IRpcChannelBuffer, GetDestCtx)
        HRESULT ( STDMETHODCALLTYPE *GetDestCtx )( 
            IAsyncRpcChannelBuffer * This,
            /* [annotation][out] */ 
            _Out_  DWORD *pdwDestContext,
            /* [annotation][out] */ 
            _Outptr_result_maybenull_  void **ppvDestContext);
        
        DECLSPEC_XFGVIRT(IRpcChannelBuffer, IsConnected)
        HRESULT ( STDMETHODCALLTYPE *IsConnected )( 
            IAsyncRpcChannelBuffer * This);
        
        DECLSPEC_XFGVIRT(IRpcChannelBuffer2, GetProtocolVersion)
        HRESULT ( STDMETHODCALLTYPE *GetProtocolVersion )( 
            IAsyncRpcChannelBuffer * This,
            /* [annotation][out] */ 
            _Out_  DWORD *pdwVersion);
        
        DECLSPEC_XFGVIRT(IAsyncRpcChannelBuffer, Send)
        HRESULT ( STDMETHODCALLTYPE *Send )( 
            IAsyncRpcChannelBuffer * This,
            /* [annotation][out][in] */ 
            _Inout_  RPCOLEMESSAGE *pMsg,
            /* [annotation][in] */ 
            _In_  ISynchronize *pSync,
            /* [annotation][out] */ 
            _Out_  ULONG *pulStatus);
        
        DECLSPEC_XFGVIRT(IAsyncRpcChannelBuffer, Receive)
        HRESULT ( STDMETHODCALLTYPE *Receive )( 
            IAsyncRpcChannelBuffer * This,
            /* [annotation][out][in] */ 
            _Inout_  RPCOLEMESSAGE *pMsg,
            /* [annotation][out] */ 
            _Out_  ULONG *pulStatus);
        
        DECLSPEC_XFGVIRT(IAsyncRpcChannelBuffer, GetDestCtxEx)
        HRESULT ( STDMETHODCALLTYPE *GetDestCtxEx )( 
            IAsyncRpcChannelBuffer * This,
            /* [annotation][in] */ 
            _In_  RPCOLEMESSAGE *pMsg,
            /* [annotation][out] */ 
            _Out_  DWORD *pdwDestContext,
            /* [annotation][out] */ 
            _Outptr_opt_result_maybenull_  void **ppvDestContext);
        
        END_INTERFACE
    } IAsyncRpcChannelBufferVtbl;

    interface IAsyncRpcChannelBuffer
    {
        CONST_VTBL struct IAsyncRpcChannelBufferVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IAsyncRpcChannelBuffer_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IAsyncRpcChannelBuffer_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IAsyncRpcChannelBuffer_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IAsyncRpcChannelBuffer_GetBuffer(This,pMessage,riid)	\
    ( (This)->lpVtbl -> GetBuffer(This,pMessage,riid) ) 

#define IAsyncRpcChannelBuffer_SendReceive(This,pMessage,pStatus)	\
    ( (This)->lpVtbl -> SendReceive(This,pMessage,pStatus) ) 

#define IAsyncRpcChannelBuffer_FreeBuffer(This,pMessage)	\
    ( (This)->lpVtbl -> FreeBuffer(This,pMessage) ) 

#define IAsyncRpcChannelBuffer_GetDestCtx(This,pdwDestContext,ppvDestContext)	\
    ( (This)->lpVtbl -> GetDestCtx(This,pdwDestContext,ppvDestContext) ) 

#define IAsyncRpcChannelBuffer_IsConnected(This)	\
    ( (This)->lpVtbl -> IsConnected(This) ) 


#define IAsyncRpcChannelBuffer_GetProtocolVersion(This,pdwVersion)	\
    ( (This)->lpVtbl -> GetProtocolVersion(This,pdwVersion) ) 


#define IAsyncRpcChannelBuffer_Send(This,pMsg,pSync,pulStatus)	\
    ( (This)->lpVtbl -> Send(This,pMsg,pSync,pulStatus) ) 

#define IAsyncRpcChannelBuffer_Receive(This,pMsg,pulStatus)	\
    ( (This)->lpVtbl -> Receive(This,pMsg,pulStatus) ) 

#define IAsyncRpcChannelBuffer_GetDestCtxEx(This,pMsg,pdwDestContext,ppvDestContext)	\
    ( (This)->lpVtbl -> GetDestCtxEx(This,pMsg,pdwDestContext,ppvDestContext) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IAsyncRpcChannelBuffer_INTERFACE_DEFINED__ */


#ifndef __IRpcChannelBuffer3_INTERFACE_DEFINED__
#define __IRpcChannelBuffer3_INTERFACE_DEFINED__

/* interface IRpcChannelBuffer3 */
/* [uuid][object][local] */ 


EXTERN_C const IID IID_IRpcChannelBuffer3;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("25B15600-0115-11d0-BF0D-00AA00B8DFD2")
    IRpcChannelBuffer3 : public IRpcChannelBuffer2
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE Send( 
            /* [annotation][out][in] */ 
            _Inout_  RPCOLEMESSAGE *pMsg,
            /* [annotation][out] */ 
            _Out_  ULONG *pulStatus) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Receive( 
            /* [annotation][out][in] */ 
            _Inout_  RPCOLEMESSAGE *pMsg,
            /* [annotation][in] */ 
            _In_  ULONG ulSize,
            /* [annotation][out] */ 
            _Out_  ULONG *pulStatus) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Cancel( 
            /* [annotation][out][in] */ 
            _Inout_  RPCOLEMESSAGE *pMsg) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetCallContext( 
            /* [annotation][in] */ 
            _In_  RPCOLEMESSAGE *pMsg,
            /* [annotation][in] */ 
            _In_  REFIID riid,
            /* [annotation][out] */ 
            _Outptr_  void **pInterface) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetDestCtxEx( 
            /* [annotation][in] */ 
            _In_  RPCOLEMESSAGE *pMsg,
            /* [annotation][out] */ 
            _Out_  DWORD *pdwDestContext,
            /* [annotation][out] */ 
            _Outptr_opt_result_maybenull_  void **ppvDestContext) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetState( 
            /* [annotation][in] */ 
            _In_  RPCOLEMESSAGE *pMsg,
            /* [annotation][out] */ 
            _Out_  DWORD *pState) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE RegisterAsync( 
            /* [annotation][out][in] */ 
            _Inout_  RPCOLEMESSAGE *pMsg,
            /* [annotation][in] */ 
            _In_  IAsyncManager *pAsyncMgr) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IRpcChannelBuffer3Vtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IRpcChannelBuffer3 * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IRpcChannelBuffer3 * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IRpcChannelBuffer3 * This);
        
        DECLSPEC_XFGVIRT(IRpcChannelBuffer, GetBuffer)
        HRESULT ( STDMETHODCALLTYPE *GetBuffer )( 
            IRpcChannelBuffer3 * This,
            /* [annotation][out][in] */ 
            _Inout_  RPCOLEMESSAGE *pMessage,
            /* [annotation][in] */ 
            _In_  REFIID riid);
        
        DECLSPEC_XFGVIRT(IRpcChannelBuffer, SendReceive)
        HRESULT ( STDMETHODCALLTYPE *SendReceive )( 
            IRpcChannelBuffer3 * This,
            /* [annotation][out][in] */ 
            _Inout_  RPCOLEMESSAGE *pMessage,
            /* [annotation][out] */ 
            _Out_opt_  ULONG *pStatus);
        
        DECLSPEC_XFGVIRT(IRpcChannelBuffer, FreeBuffer)
        HRESULT ( STDMETHODCALLTYPE *FreeBuffer )( 
            IRpcChannelBuffer3 * This,
            /* [annotation][out][in] */ 
            _Inout_  RPCOLEMESSAGE *pMessage);
        
        DECLSPEC_XFGVIRT(IRpcChannelBuffer, GetDestCtx)
        HRESULT ( STDMETHODCALLTYPE *GetDestCtx )( 
            IRpcChannelBuffer3 * This,
            /* [annotation][out] */ 
            _Out_  DWORD *pdwDestContext,
            /* [annotation][out] */ 
            _Outptr_result_maybenull_  void **ppvDestContext);
        
        DECLSPEC_XFGVIRT(IRpcChannelBuffer, IsConnected)
        HRESULT ( STDMETHODCALLTYPE *IsConnected )( 
            IRpcChannelBuffer3 * This);
        
        DECLSPEC_XFGVIRT(IRpcChannelBuffer2, GetProtocolVersion)
        HRESULT ( STDMETHODCALLTYPE *GetProtocolVersion )( 
            IRpcChannelBuffer3 * This,
            /* [annotation][out] */ 
            _Out_  DWORD *pdwVersion);
        
        DECLSPEC_XFGVIRT(IRpcChannelBuffer3, Send)
        HRESULT ( STDMETHODCALLTYPE *Send )( 
            IRpcChannelBuffer3 * This,
            /* [annotation][out][in] */ 
            _Inout_  RPCOLEMESSAGE *pMsg,
            /* [annotation][out] */ 
            _Out_  ULONG *pulStatus);
        
        DECLSPEC_XFGVIRT(IRpcChannelBuffer3, Receive)
        HRESULT ( STDMETHODCALLTYPE *Receive )( 
            IRpcChannelBuffer3 * This,
            /* [annotation][out][in] */ 
            _Inout_  RPCOLEMESSAGE *pMsg,
            /* [annotation][in] */ 
            _In_  ULONG ulSize,
            /* [annotation][out] */ 
            _Out_  ULONG *pulStatus);
        
        DECLSPEC_XFGVIRT(IRpcChannelBuffer3, Cancel)
        HRESULT ( STDMETHODCALLTYPE *Cancel )( 
            IRpcChannelBuffer3 * This,
            /* [annotation][out][in] */ 
            _Inout_  RPCOLEMESSAGE *pMsg);
        
        DECLSPEC_XFGVIRT(IRpcChannelBuffer3, GetCallContext)
        HRESULT ( STDMETHODCALLTYPE *GetCallContext )( 
            IRpcChannelBuffer3 * This,
            /* [annotation][in] */ 
            _In_  RPCOLEMESSAGE *pMsg,
            /* [annotation][in] */ 
            _In_  REFIID riid,
            /* [annotation][out] */ 
            _Outptr_  void **pInterface);
        
        DECLSPEC_XFGVIRT(IRpcChannelBuffer3, GetDestCtxEx)
        HRESULT ( STDMETHODCALLTYPE *GetDestCtxEx )( 
            IRpcChannelBuffer3 * This,
            /* [annotation][in] */ 
            _In_  RPCOLEMESSAGE *pMsg,
            /* [annotation][out] */ 
            _Out_  DWORD *pdwDestContext,
            /* [annotation][out] */ 
            _Outptr_opt_result_maybenull_  void **ppvDestContext);
        
        DECLSPEC_XFGVIRT(IRpcChannelBuffer3, GetState)
        HRESULT ( STDMETHODCALLTYPE *GetState )( 
            IRpcChannelBuffer3 * This,
            /* [annotation][in] */ 
            _In_  RPCOLEMESSAGE *pMsg,
            /* [annotation][out] */ 
            _Out_  DWORD *pState);
        
        DECLSPEC_XFGVIRT(IRpcChannelBuffer3, RegisterAsync)
        HRESULT ( STDMETHODCALLTYPE *RegisterAsync )( 
            IRpcChannelBuffer3 * This,
            /* [annotation][out][in] */ 
            _Inout_  RPCOLEMESSAGE *pMsg,
            /* [annotation][in] */ 
            _In_  IAsyncManager *pAsyncMgr);
        
        END_INTERFACE
    } IRpcChannelBuffer3Vtbl;

    interface IRpcChannelBuffer3
    {
        CONST_VTBL struct IRpcChannelBuffer3Vtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IRpcChannelBuffer3_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IRpcChannelBuffer3_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IRpcChannelBuffer3_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IRpcChannelBuffer3_GetBuffer(This,pMessage,riid)	\
    ( (This)->lpVtbl -> GetBuffer(This,pMessage,riid) ) 

#define IRpcChannelBuffer3_SendReceive(This,pMessage,pStatus)	\
    ( (This)->lpVtbl -> SendReceive(This,pMessage,pStatus) ) 

#define IRpcChannelBuffer3_FreeBuffer(This,pMessage)	\
    ( (This)->lpVtbl -> FreeBuffer(This,pMessage) ) 

#define IRpcChannelBuffer3_GetDestCtx(This,pdwDestContext,ppvDestContext)	\
    ( (This)->lpVtbl -> GetDestCtx(This,pdwDestContext,ppvDestContext) ) 

#define IRpcChannelBuffer3_IsConnected(This)	\
    ( (This)->lpVtbl -> IsConnected(This) ) 


#define IRpcChannelBuffer3_GetProtocolVersion(This,pdwVersion)	\
    ( (This)->lpVtbl -> GetProtocolVersion(This,pdwVersion) ) 


#define IRpcChannelBuffer3_Send(This,pMsg,pulStatus)	\
    ( (This)->lpVtbl -> Send(This,pMsg,pulStatus) ) 

#define IRpcChannelBuffer3_Receive(This,pMsg,ulSize,pulStatus)	\
    ( (This)->lpVtbl -> Receive(This,pMsg,ulSize,pulStatus) ) 

#define IRpcChannelBuffer3_Cancel(This,pMsg)	\
    ( (This)->lpVtbl -> Cancel(This,pMsg) ) 

#define IRpcChannelBuffer3_GetCallContext(This,pMsg,riid,pInterface)	\
    ( (This)->lpVtbl -> GetCallContext(This,pMsg,riid,pInterface) ) 

#define IRpcChannelBuffer3_GetDestCtxEx(This,pMsg,pdwDestContext,ppvDestContext)	\
    ( (This)->lpVtbl -> GetDestCtxEx(This,pMsg,pdwDestContext,ppvDestContext) ) 

#define IRpcChannelBuffer3_GetState(This,pMsg,pState)	\
    ( (This)->lpVtbl -> GetState(This,pMsg,pState) ) 

#define IRpcChannelBuffer3_RegisterAsync(This,pMsg,pAsyncMgr)	\
    ( (This)->lpVtbl -> RegisterAsync(This,pMsg,pAsyncMgr) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IRpcChannelBuffer3_INTERFACE_DEFINED__ */


#ifndef __IRpcSyntaxNegotiate_INTERFACE_DEFINED__
#define __IRpcSyntaxNegotiate_INTERFACE_DEFINED__

/* interface IRpcSyntaxNegotiate */
/* [uuid][object][local] */ 


EXTERN_C const IID IID_IRpcSyntaxNegotiate;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("58a08519-24c8-4935-b482-3fd823333a4f")
    IRpcSyntaxNegotiate : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE NegotiateSyntax( 
            /* [annotation][out][in] */ 
            _Inout_  RPCOLEMESSAGE *pMsg) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IRpcSyntaxNegotiateVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IRpcSyntaxNegotiate * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IRpcSyntaxNegotiate * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IRpcSyntaxNegotiate * This);
        
        DECLSPEC_XFGVIRT(IRpcSyntaxNegotiate, NegotiateSyntax)
        HRESULT ( STDMETHODCALLTYPE *NegotiateSyntax )( 
            IRpcSyntaxNegotiate * This,
            /* [annotation][out][in] */ 
            _Inout_  RPCOLEMESSAGE *pMsg);
        
        END_INTERFACE
    } IRpcSyntaxNegotiateVtbl;

    interface IRpcSyntaxNegotiate
    {
        CONST_VTBL struct IRpcSyntaxNegotiateVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IRpcSyntaxNegotiate_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IRpcSyntaxNegotiate_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IRpcSyntaxNegotiate_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IRpcSyntaxNegotiate_NegotiateSyntax(This,pMsg)	\
    ( (This)->lpVtbl -> NegotiateSyntax(This,pMsg) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IRpcSyntaxNegotiate_INTERFACE_DEFINED__ */


#ifndef __IRpcProxyBuffer_INTERFACE_DEFINED__
#define __IRpcProxyBuffer_INTERFACE_DEFINED__

/* interface IRpcProxyBuffer */
/* [uuid][object][local] */ 


EXTERN_C const IID IID_IRpcProxyBuffer;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("D5F56A34-593B-101A-B569-08002B2DBF7A")
    IRpcProxyBuffer : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE Connect( 
            /* [annotation][unique][in] */ 
            _In_  IRpcChannelBuffer *pRpcChannelBuffer) = 0;
        
        virtual void STDMETHODCALLTYPE Disconnect( void) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IRpcProxyBufferVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IRpcProxyBuffer * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IRpcProxyBuffer * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IRpcProxyBuffer * This);
        
        DECLSPEC_XFGVIRT(IRpcProxyBuffer, Connect)
        HRESULT ( STDMETHODCALLTYPE *Connect )( 
            IRpcProxyBuffer * This,
            /* [annotation][unique][in] */ 
            _In_  IRpcChannelBuffer *pRpcChannelBuffer);
        
        DECLSPEC_XFGVIRT(IRpcProxyBuffer, Disconnect)
        void ( STDMETHODCALLTYPE *Disconnect )( 
            IRpcProxyBuffer * This);
        
        END_INTERFACE
    } IRpcProxyBufferVtbl;

    interface IRpcProxyBuffer
    {
        CONST_VTBL struct IRpcProxyBufferVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IRpcProxyBuffer_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IRpcProxyBuffer_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IRpcProxyBuffer_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IRpcProxyBuffer_Connect(This,pRpcChannelBuffer)	\
    ( (This)->lpVtbl -> Connect(This,pRpcChannelBuffer) ) 

#define IRpcProxyBuffer_Disconnect(This)	\
    ( (This)->lpVtbl -> Disconnect(This) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IRpcProxyBuffer_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_objidl_0000_0020 */
/* [local] */ 

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES) */
#pragma endregion
#pragma region Application Family or OneCore Family or OneCore Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES)


extern RPC_IF_HANDLE __MIDL_itf_objidl_0000_0020_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_objidl_0000_0020_v0_0_s_ifspec;

#ifndef __IRpcStubBuffer_INTERFACE_DEFINED__
#define __IRpcStubBuffer_INTERFACE_DEFINED__

/* interface IRpcStubBuffer */
/* [uuid][object][local] */ 


EXTERN_C const IID IID_IRpcStubBuffer;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("D5F56AFC-593B-101A-B569-08002B2DBF7A")
    IRpcStubBuffer : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE Connect( 
            /* [annotation][in] */ 
            _In_  IUnknown *pUnkServer) = 0;
        
        virtual void STDMETHODCALLTYPE Disconnect( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Invoke( 
            /* [annotation][out][in] */ 
            _Inout_  RPCOLEMESSAGE *_prpcmsg,
            /* [annotation][in] */ 
            _In_  IRpcChannelBuffer *_pRpcChannelBuffer) = 0;
        
        virtual IRpcStubBuffer *STDMETHODCALLTYPE IsIIDSupported( 
            /* [annotation][in] */ 
            _In_  REFIID riid) = 0;
        
        virtual ULONG STDMETHODCALLTYPE CountRefs( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE DebugServerQueryInterface( 
            /* [annotation][out] */ 
            _Outptr_  void **ppv) = 0;
        
        virtual void STDMETHODCALLTYPE DebugServerRelease( 
            /* [annotation][in] */ 
            _In_  void *pv) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IRpcStubBufferVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IRpcStubBuffer * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IRpcStubBuffer * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IRpcStubBuffer * This);
        
        DECLSPEC_XFGVIRT(IRpcStubBuffer, Connect)
        HRESULT ( STDMETHODCALLTYPE *Connect )( 
            IRpcStubBuffer * This,
            /* [annotation][in] */ 
            _In_  IUnknown *pUnkServer);
        
        DECLSPEC_XFGVIRT(IRpcStubBuffer, Disconnect)
        void ( STDMETHODCALLTYPE *Disconnect )( 
            IRpcStubBuffer * This);
        
        DECLSPEC_XFGVIRT(IRpcStubBuffer, Invoke)
        HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IRpcStubBuffer * This,
            /* [annotation][out][in] */ 
            _Inout_  RPCOLEMESSAGE *_prpcmsg,
            /* [annotation][in] */ 
            _In_  IRpcChannelBuffer *_pRpcChannelBuffer);
        
        DECLSPEC_XFGVIRT(IRpcStubBuffer, IsIIDSupported)
        IRpcStubBuffer *( STDMETHODCALLTYPE *IsIIDSupported )( 
            IRpcStubBuffer * This,
            /* [annotation][in] */ 
            _In_  REFIID riid);
        
        DECLSPEC_XFGVIRT(IRpcStubBuffer, CountRefs)
        ULONG ( STDMETHODCALLTYPE *CountRefs )( 
            IRpcStubBuffer * This);
        
        DECLSPEC_XFGVIRT(IRpcStubBuffer, DebugServerQueryInterface)
        HRESULT ( STDMETHODCALLTYPE *DebugServerQueryInterface )( 
            IRpcStubBuffer * This,
            /* [annotation][out] */ 
            _Outptr_  void **ppv);
        
        DECLSPEC_XFGVIRT(IRpcStubBuffer, DebugServerRelease)
        void ( STDMETHODCALLTYPE *DebugServerRelease )( 
            IRpcStubBuffer * This,
            /* [annotation][in] */ 
            _In_  void *pv);
        
        END_INTERFACE
    } IRpcStubBufferVtbl;

    interface IRpcStubBuffer
    {
        CONST_VTBL struct IRpcStubBufferVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IRpcStubBuffer_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IRpcStubBuffer_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IRpcStubBuffer_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IRpcStubBuffer_Connect(This,pUnkServer)	\
    ( (This)->lpVtbl -> Connect(This,pUnkServer) ) 

#define IRpcStubBuffer_Disconnect(This)	\
    ( (This)->lpVtbl -> Disconnect(This) ) 

#define IRpcStubBuffer_Invoke(This,_prpcmsg,_pRpcChannelBuffer)	\
    ( (This)->lpVtbl -> Invoke(This,_prpcmsg,_pRpcChannelBuffer) ) 

#define IRpcStubBuffer_IsIIDSupported(This,riid)	\
    ( (This)->lpVtbl -> IsIIDSupported(This,riid) ) 

#define IRpcStubBuffer_CountRefs(This)	\
    ( (This)->lpVtbl -> CountRefs(This) ) 

#define IRpcStubBuffer_DebugServerQueryInterface(This,ppv)	\
    ( (This)->lpVtbl -> DebugServerQueryInterface(This,ppv) ) 

#define IRpcStubBuffer_DebugServerRelease(This,pv)	\
    ( (This)->lpVtbl -> DebugServerRelease(This,pv) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IRpcStubBuffer_INTERFACE_DEFINED__ */


#ifndef __IPSFactoryBuffer_INTERFACE_DEFINED__
#define __IPSFactoryBuffer_INTERFACE_DEFINED__

/* interface IPSFactoryBuffer */
/* [uuid][object][local] */ 


EXTERN_C const IID IID_IPSFactoryBuffer;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("D5F569D0-593B-101A-B569-08002B2DBF7A")
    IPSFactoryBuffer : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE CreateProxy( 
            /* [annotation][in] */ 
            _In_  IUnknown *pUnkOuter,
            /* [annotation][in] */ 
            _In_  REFIID riid,
            /* [annotation][out] */ 
            _Outptr_  IRpcProxyBuffer **ppProxy,
            /* [annotation][out] */ 
            _Outptr_  void **ppv) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE CreateStub( 
            /* [annotation][in] */ 
            _In_  REFIID riid,
            /* [annotation][unique][in] */ 
            _In_opt_  IUnknown *pUnkServer,
            /* [annotation][out] */ 
            _Outptr_  IRpcStubBuffer **ppStub) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IPSFactoryBufferVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IPSFactoryBuffer * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IPSFactoryBuffer * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IPSFactoryBuffer * This);
        
        DECLSPEC_XFGVIRT(IPSFactoryBuffer, CreateProxy)
        HRESULT ( STDMETHODCALLTYPE *CreateProxy )( 
            IPSFactoryBuffer * This,
            /* [annotation][in] */ 
            _In_  IUnknown *pUnkOuter,
            /* [annotation][in] */ 
            _In_  REFIID riid,
            /* [annotation][out] */ 
            _Outptr_  IRpcProxyBuffer **ppProxy,
            /* [annotation][out] */ 
            _Outptr_  void **ppv);
        
        DECLSPEC_XFGVIRT(IPSFactoryBuffer, CreateStub)
        HRESULT ( STDMETHODCALLTYPE *CreateStub )( 
            IPSFactoryBuffer * This,
            /* [annotation][in] */ 
            _In_  REFIID riid,
            /* [annotation][unique][in] */ 
            _In_opt_  IUnknown *pUnkServer,
            /* [annotation][out] */ 
            _Outptr_  IRpcStubBuffer **ppStub);
        
        END_INTERFACE
    } IPSFactoryBufferVtbl;

    interface IPSFactoryBuffer
    {
        CONST_VTBL struct IPSFactoryBufferVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IPSFactoryBuffer_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IPSFactoryBuffer_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IPSFactoryBuffer_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IPSFactoryBuffer_CreateProxy(This,pUnkOuter,riid,ppProxy,ppv)	\
    ( (This)->lpVtbl -> CreateProxy(This,pUnkOuter,riid,ppProxy,ppv) ) 

#define IPSFactoryBuffer_CreateStub(This,riid,pUnkServer,ppStub)	\
    ( (This)->lpVtbl -> CreateStub(This,riid,pUnkServer,ppStub) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IPSFactoryBuffer_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_objidl_0000_0022 */
/* [local] */ 

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES) */
#pragma endregion
#pragma region Desktop Family or OneCore Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES)
#if  (_WIN32_WINNT >= 0x0400 ) || defined(_WIN32_DCOM) // DCOM
// This interface is only valid on Windows NT 4.0
typedef struct SChannelHookCallInfo
    {
    IID iid;
    DWORD cbSize;
    GUID uCausality;
    DWORD dwServerPid;
    DWORD iMethod;
    void *pObject;
    } 	SChannelHookCallInfo;



extern RPC_IF_HANDLE __MIDL_itf_objidl_0000_0022_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_objidl_0000_0022_v0_0_s_ifspec;

#ifndef __IChannelHook_INTERFACE_DEFINED__
#define __IChannelHook_INTERFACE_DEFINED__

/* interface IChannelHook */
/* [uuid][object][local] */ 


EXTERN_C const IID IID_IChannelHook;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("1008c4a0-7613-11cf-9af1-0020af6e72f4")
    IChannelHook : public IUnknown
    {
    public:
        virtual void STDMETHODCALLTYPE ClientGetSize( 
            /* [annotation][in] */ 
            _In_  REFGUID uExtent,
            /* [annotation][in] */ 
            _In_  REFIID riid,
            /* [annotation][out] */ 
            _Out_  ULONG *pDataSize) = 0;
        
        virtual void STDMETHODCALLTYPE ClientFillBuffer( 
            /* [annotation][in] */ 
            _In_  REFGUID uExtent,
            /* [annotation][in] */ 
            _In_  REFIID riid,
            /* [annotation][out][in] */ 
            _Inout_  ULONG *pDataSize,
            /* [annotation][in] */ 
            _In_  void *pDataBuffer) = 0;
        
        virtual void STDMETHODCALLTYPE ClientNotify( 
            /* [annotation][in] */ 
            _In_  REFGUID uExtent,
            /* [annotation][in] */ 
            _In_  REFIID riid,
            /* [annotation][in] */ 
            _In_  ULONG cbDataSize,
            /* [annotation][in] */ 
            _In_  void *pDataBuffer,
            /* [annotation][in] */ 
            _In_  DWORD lDataRep,
            /* [annotation][in] */ 
            _In_  HRESULT hrFault) = 0;
        
        virtual void STDMETHODCALLTYPE ServerNotify( 
            /* [annotation][in] */ 
            _In_  REFGUID uExtent,
            /* [annotation][in] */ 
            _In_  REFIID riid,
            /* [annotation][in] */ 
            _In_  ULONG cbDataSize,
            /* [annotation][in] */ 
            _In_  void *pDataBuffer,
            /* [annotation][in] */ 
            _In_  DWORD lDataRep) = 0;
        
        virtual void STDMETHODCALLTYPE ServerGetSize( 
            /* [annotation][in] */ 
            _In_  REFGUID uExtent,
            /* [annotation][in] */ 
            _In_  REFIID riid,
            /* [annotation][in] */ 
            _In_  HRESULT hrFault,
            /* [annotation][out] */ 
            _Out_  ULONG *pDataSize) = 0;
        
        virtual void STDMETHODCALLTYPE ServerFillBuffer( 
            /* [annotation][in] */ 
            _In_  REFGUID uExtent,
            /* [annotation][in] */ 
            _In_  REFIID riid,
            /* [annotation][out][in] */ 
            _Inout_  ULONG *pDataSize,
            /* [annotation][in] */ 
            _In_  void *pDataBuffer,
            /* [annotation][in] */ 
            _In_  HRESULT hrFault) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IChannelHookVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IChannelHook * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IChannelHook * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IChannelHook * This);
        
        DECLSPEC_XFGVIRT(IChannelHook, ClientGetSize)
        void ( STDMETHODCALLTYPE *ClientGetSize )( 
            IChannelHook * This,
            /* [annotation][in] */ 
            _In_  REFGUID uExtent,
            /* [annotation][in] */ 
            _In_  REFIID riid,
            /* [annotation][out] */ 
            _Out_  ULONG *pDataSize);
        
        DECLSPEC_XFGVIRT(IChannelHook, ClientFillBuffer)
        void ( STDMETHODCALLTYPE *ClientFillBuffer )( 
            IChannelHook * This,
            /* [annotation][in] */ 
            _In_  REFGUID uExtent,
            /* [annotation][in] */ 
            _In_  REFIID riid,
            /* [annotation][out][in] */ 
            _Inout_  ULONG *pDataSize,
            /* [annotation][in] */ 
            _In_  void *pDataBuffer);
        
        DECLSPEC_XFGVIRT(IChannelHook, ClientNotify)
        void ( STDMETHODCALLTYPE *ClientNotify )( 
            IChannelHook * This,
            /* [annotation][in] */ 
            _In_  REFGUID uExtent,
            /* [annotation][in] */ 
            _In_  REFIID riid,
            /* [annotation][in] */ 
            _In_  ULONG cbDataSize,
            /* [annotation][in] */ 
            _In_  void *pDataBuffer,
            /* [annotation][in] */ 
            _In_  DWORD lDataRep,
            /* [annotation][in] */ 
            _In_  HRESULT hrFault);
        
        DECLSPEC_XFGVIRT(IChannelHook, ServerNotify)
        void ( STDMETHODCALLTYPE *ServerNotify )( 
            IChannelHook * This,
            /* [annotation][in] */ 
            _In_  REFGUID uExtent,
            /* [annotation][in] */ 
            _In_  REFIID riid,
            /* [annotation][in] */ 
            _In_  ULONG cbDataSize,
            /* [annotation][in] */ 
            _In_  void *pDataBuffer,
            /* [annotation][in] */ 
            _In_  DWORD lDataRep);
        
        DECLSPEC_XFGVIRT(IChannelHook, ServerGetSize)
        void ( STDMETHODCALLTYPE *ServerGetSize )( 
            IChannelHook * This,
            /* [annotation][in] */ 
            _In_  REFGUID uExtent,
            /* [annotation][in] */ 
            _In_  REFIID riid,
            /* [annotation][in] */ 
            _In_  HRESULT hrFault,
            /* [annotation][out] */ 
            _Out_  ULONG *pDataSize);
        
        DECLSPEC_XFGVIRT(IChannelHook, ServerFillBuffer)
        void ( STDMETHODCALLTYPE *ServerFillBuffer )( 
            IChannelHook * This,
            /* [annotation][in] */ 
            _In_  REFGUID uExtent,
            /* [annotation][in] */ 
            _In_  REFIID riid,
            /* [annotation][out][in] */ 
            _Inout_  ULONG *pDataSize,
            /* [annotation][in] */ 
            _In_  void *pDataBuffer,
            /* [annotation][in] */ 
            _In_  HRESULT hrFault);
        
        END_INTERFACE
    } IChannelHookVtbl;

    interface IChannelHook
    {
        CONST_VTBL struct IChannelHookVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IChannelHook_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IChannelHook_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IChannelHook_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IChannelHook_ClientGetSize(This,uExtent,riid,pDataSize)	\
    ( (This)->lpVtbl -> ClientGetSize(This,uExtent,riid,pDataSize) ) 

#define IChannelHook_ClientFillBuffer(This,uExtent,riid,pDataSize,pDataBuffer)	\
    ( (This)->lpVtbl -> ClientFillBuffer(This,uExtent,riid,pDataSize,pDataBuffer) ) 

#define IChannelHook_ClientNotify(This,uExtent,riid,cbDataSize,pDataBuffer,lDataRep,hrFault)	\
    ( (This)->lpVtbl -> ClientNotify(This,uExtent,riid,cbDataSize,pDataBuffer,lDataRep,hrFault) ) 

#define IChannelHook_ServerNotify(This,uExtent,riid,cbDataSize,pDataBuffer,lDataRep)	\
    ( (This)->lpVtbl -> ServerNotify(This,uExtent,riid,cbDataSize,pDataBuffer,lDataRep) ) 

#define IChannelHook_ServerGetSize(This,uExtent,riid,hrFault,pDataSize)	\
    ( (This)->lpVtbl -> ServerGetSize(This,uExtent,riid,hrFault,pDataSize) ) 

#define IChannelHook_ServerFillBuffer(This,uExtent,riid,pDataSize,pDataBuffer,hrFault)	\
    ( (This)->lpVtbl -> ServerFillBuffer(This,uExtent,riid,pDataSize,pDataBuffer,hrFault) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IChannelHook_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_objidl_0000_0023 */
/* [local] */ 

#endif //DCOM
#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES) */
#pragma endregion
#if  (_WIN32_WINNT >= 0x0400 ) || defined(_WIN32_DCOM) // DCOM
// This interface is only valid on Windows NT 4.0
#pragma region Application Family or OneCore Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES)


extern RPC_IF_HANDLE __MIDL_itf_objidl_0000_0023_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_objidl_0000_0023_v0_0_s_ifspec;

#ifndef __IClientSecurity_INTERFACE_DEFINED__
#define __IClientSecurity_INTERFACE_DEFINED__

/* interface IClientSecurity */
/* [uuid][object][local] */ 

typedef struct tagSOLE_AUTHENTICATION_SERVICE
    {
    DWORD dwAuthnSvc;
    DWORD dwAuthzSvc;
    OLECHAR *pPrincipalName;
    HRESULT hr;
    } 	SOLE_AUTHENTICATION_SERVICE;

typedef SOLE_AUTHENTICATION_SERVICE *PSOLE_AUTHENTICATION_SERVICE;

typedef 
enum tagEOLE_AUTHENTICATION_CAPABILITIES
    {
        EOAC_NONE	= 0,
        EOAC_MUTUAL_AUTH	= 0x1,
        EOAC_STATIC_CLOAKING	= 0x20,
        EOAC_DYNAMIC_CLOAKING	= 0x40,
        EOAC_ANY_AUTHORITY	= 0x80,
        EOAC_MAKE_FULLSIC	= 0x100,
        EOAC_DEFAULT	= 0x800,
        EOAC_SECURE_REFS	= 0x2,
        EOAC_ACCESS_CONTROL	= 0x4,
        EOAC_APPID	= 0x8,
        EOAC_DYNAMIC	= 0x10,
        EOAC_REQUIRE_FULLSIC	= 0x200,
        EOAC_AUTO_IMPERSONATE	= 0x400,
        EOAC_DISABLE_AAA	= 0x1000,
        EOAC_NO_CUSTOM_MARSHAL	= 0x2000,
        EOAC_RESERVED1	= 0x4000
    } 	EOLE_AUTHENTICATION_CAPABILITIES;

#define	COLE_DEFAULT_PRINCIPAL	( ( OLECHAR * )( INT_PTR  )-1 )

#define	COLE_DEFAULT_AUTHINFO	( ( void * )( INT_PTR  )-1 )

typedef struct tagSOLE_AUTHENTICATION_INFO
    {
    DWORD dwAuthnSvc;
    DWORD dwAuthzSvc;
    void *pAuthInfo;
    } 	SOLE_AUTHENTICATION_INFO;

typedef struct tagSOLE_AUTHENTICATION_INFO *PSOLE_AUTHENTICATION_INFO;

typedef struct tagSOLE_AUTHENTICATION_LIST
    {
    DWORD cAuthInfo;
    SOLE_AUTHENTICATION_INFO *aAuthInfo;
    } 	SOLE_AUTHENTICATION_LIST;

typedef struct tagSOLE_AUTHENTICATION_LIST *PSOLE_AUTHENTICATION_LIST;


EXTERN_C const IID IID_IClientSecurity;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("0000013D-0000-0000-C000-000000000046")
    IClientSecurity : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE QueryBlanket( 
            /* [annotation][in] */ 
            _In_  IUnknown *pProxy,
            /* [annotation][out] */ 
            _Out_  DWORD *pAuthnSvc,
            /* [annotation][out] */ 
            _Out_opt_  DWORD *pAuthzSvc,
            /* [annotation][out] */ 
            __RPC__deref_out_opt  OLECHAR **pServerPrincName,
            /* [annotation][out] */ 
            _Out_opt_  DWORD *pAuthnLevel,
            /* [annotation][out] */ 
            _Out_opt_  DWORD *pImpLevel,
            /* [annotation][out] */ 
            _Outptr_result_maybenull_  void **pAuthInfo,
            /* [annotation][out] */ 
            _Out_opt_  DWORD *pCapabilites) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetBlanket( 
            /* [annotation][in] */ 
            _In_  IUnknown *pProxy,
            /* [annotation][in] */ 
            _In_  DWORD dwAuthnSvc,
            /* [annotation][in] */ 
            _In_  DWORD dwAuthzSvc,
            /* [annotation][in] */ 
            __RPC__in_opt  OLECHAR *pServerPrincName,
            /* [annotation][in] */ 
            _In_  DWORD dwAuthnLevel,
            /* [annotation][in] */ 
            _In_  DWORD dwImpLevel,
            /* [annotation][in] */ 
            _In_opt_  void *pAuthInfo,
            /* [annotation][in] */ 
            _In_  DWORD dwCapabilities) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE CopyProxy( 
            /* [annotation][in] */ 
            _In_  IUnknown *pProxy,
            /* [annotation][out] */ 
            _Outptr_  IUnknown **ppCopy) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IClientSecurityVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IClientSecurity * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IClientSecurity * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IClientSecurity * This);
        
        DECLSPEC_XFGVIRT(IClientSecurity, QueryBlanket)
        HRESULT ( STDMETHODCALLTYPE *QueryBlanket )( 
            IClientSecurity * This,
            /* [annotation][in] */ 
            _In_  IUnknown *pProxy,
            /* [annotation][out] */ 
            _Out_  DWORD *pAuthnSvc,
            /* [annotation][out] */ 
            _Out_opt_  DWORD *pAuthzSvc,
            /* [annotation][out] */ 
            __RPC__deref_out_opt  OLECHAR **pServerPrincName,
            /* [annotation][out] */ 
            _Out_opt_  DWORD *pAuthnLevel,
            /* [annotation][out] */ 
            _Out_opt_  DWORD *pImpLevel,
            /* [annotation][out] */ 
            _Outptr_result_maybenull_  void **pAuthInfo,
            /* [annotation][out] */ 
            _Out_opt_  DWORD *pCapabilites);
        
        DECLSPEC_XFGVIRT(IClientSecurity, SetBlanket)
        HRESULT ( STDMETHODCALLTYPE *SetBlanket )( 
            IClientSecurity * This,
            /* [annotation][in] */ 
            _In_  IUnknown *pProxy,
            /* [annotation][in] */ 
            _In_  DWORD dwAuthnSvc,
            /* [annotation][in] */ 
            _In_  DWORD dwAuthzSvc,
            /* [annotation][in] */ 
            __RPC__in_opt  OLECHAR *pServerPrincName,
            /* [annotation][in] */ 
            _In_  DWORD dwAuthnLevel,
            /* [annotation][in] */ 
            _In_  DWORD dwImpLevel,
            /* [annotation][in] */ 
            _In_opt_  void *pAuthInfo,
            /* [annotation][in] */ 
            _In_  DWORD dwCapabilities);
        
        DECLSPEC_XFGVIRT(IClientSecurity, CopyProxy)
        HRESULT ( STDMETHODCALLTYPE *CopyProxy )( 
            IClientSecurity * This,
            /* [annotation][in] */ 
            _In_  IUnknown *pProxy,
            /* [annotation][out] */ 
            _Outptr_  IUnknown **ppCopy);
        
        END_INTERFACE
    } IClientSecurityVtbl;

    interface IClientSecurity
    {
        CONST_VTBL struct IClientSecurityVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IClientSecurity_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IClientSecurity_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IClientSecurity_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IClientSecurity_QueryBlanket(This,pProxy,pAuthnSvc,pAuthzSvc,pServerPrincName,pAuthnLevel,pImpLevel,pAuthInfo,pCapabilites)	\
    ( (This)->lpVtbl -> QueryBlanket(This,pProxy,pAuthnSvc,pAuthzSvc,pServerPrincName,pAuthnLevel,pImpLevel,pAuthInfo,pCapabilites) ) 

#define IClientSecurity_SetBlanket(This,pProxy,dwAuthnSvc,dwAuthzSvc,pServerPrincName,dwAuthnLevel,dwImpLevel,pAuthInfo,dwCapabilities)	\
    ( (This)->lpVtbl -> SetBlanket(This,pProxy,dwAuthnSvc,dwAuthzSvc,pServerPrincName,dwAuthnLevel,dwImpLevel,pAuthInfo,dwCapabilities) ) 

#define IClientSecurity_CopyProxy(This,pProxy,ppCopy)	\
    ( (This)->lpVtbl -> CopyProxy(This,pProxy,ppCopy) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IClientSecurity_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_objidl_0000_0024 */
/* [local] */ 

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES) */
#pragma endregion
#pragma region Desktop Family or OneCore Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES)


extern RPC_IF_HANDLE __MIDL_itf_objidl_0000_0024_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_objidl_0000_0024_v0_0_s_ifspec;

#ifndef __IServerSecurity_INTERFACE_DEFINED__
#define __IServerSecurity_INTERFACE_DEFINED__

/* interface IServerSecurity */
/* [uuid][object][local] */ 


EXTERN_C const IID IID_IServerSecurity;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("0000013E-0000-0000-C000-000000000046")
    IServerSecurity : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE QueryBlanket( 
            /* [annotation][out] */ 
            _Out_opt_  DWORD *pAuthnSvc,
            /* [annotation][out] */ 
            _Out_opt_  DWORD *pAuthzSvc,
            /* [annotation][out] */ 
            __RPC__deref_out_opt  OLECHAR **pServerPrincName,
            /* [annotation][out] */ 
            _Out_opt_  DWORD *pAuthnLevel,
            /* [annotation][out] */ 
            _Out_opt_  DWORD *pImpLevel,
            /* [annotation][out] */ 
            _Outptr_result_maybenull_  void **pPrivs,
            /* [annotation][out][in] */ 
            _Inout_opt_  DWORD *pCapabilities) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE ImpersonateClient( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE RevertToSelf( void) = 0;
        
        virtual BOOL STDMETHODCALLTYPE IsImpersonating( void) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IServerSecurityVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IServerSecurity * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IServerSecurity * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IServerSecurity * This);
        
        DECLSPEC_XFGVIRT(IServerSecurity, QueryBlanket)
        HRESULT ( STDMETHODCALLTYPE *QueryBlanket )( 
            IServerSecurity * This,
            /* [annotation][out] */ 
            _Out_opt_  DWORD *pAuthnSvc,
            /* [annotation][out] */ 
            _Out_opt_  DWORD *pAuthzSvc,
            /* [annotation][out] */ 
            __RPC__deref_out_opt  OLECHAR **pServerPrincName,
            /* [annotation][out] */ 
            _Out_opt_  DWORD *pAuthnLevel,
            /* [annotation][out] */ 
            _Out_opt_  DWORD *pImpLevel,
            /* [annotation][out] */ 
            _Outptr_result_maybenull_  void **pPrivs,
            /* [annotation][out][in] */ 
            _Inout_opt_  DWORD *pCapabilities);
        
        DECLSPEC_XFGVIRT(IServerSecurity, ImpersonateClient)
        HRESULT ( STDMETHODCALLTYPE *ImpersonateClient )( 
            IServerSecurity * This);
        
        DECLSPEC_XFGVIRT(IServerSecurity, RevertToSelf)
        HRESULT ( STDMETHODCALLTYPE *RevertToSelf )( 
            IServerSecurity * This);
        
        DECLSPEC_XFGVIRT(IServerSecurity, IsImpersonating)
        BOOL ( STDMETHODCALLTYPE *IsImpersonating )( 
            IServerSecurity * This);
        
        END_INTERFACE
    } IServerSecurityVtbl;

    interface IServerSecurity
    {
        CONST_VTBL struct IServerSecurityVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IServerSecurity_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IServerSecurity_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IServerSecurity_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IServerSecurity_QueryBlanket(This,pAuthnSvc,pAuthzSvc,pServerPrincName,pAuthnLevel,pImpLevel,pPrivs,pCapabilities)	\
    ( (This)->lpVtbl -> QueryBlanket(This,pAuthnSvc,pAuthzSvc,pServerPrincName,pAuthnLevel,pImpLevel,pPrivs,pCapabilities) ) 

#define IServerSecurity_ImpersonateClient(This)	\
    ( (This)->lpVtbl -> ImpersonateClient(This) ) 

#define IServerSecurity_RevertToSelf(This)	\
    ( (This)->lpVtbl -> RevertToSelf(This) ) 

#define IServerSecurity_IsImpersonating(This)	\
    ( (This)->lpVtbl -> IsImpersonating(This) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IServerSecurity_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_objidl_0000_0025 */
/* [local] */ 

typedef 
enum tagRPCOPT_PROPERTIES
    {
        COMBND_RPCTIMEOUT	= 0x1,
        COMBND_SERVER_LOCALITY	= 0x2,
        COMBND_RESERVED1	= 0x4,
        COMBND_RESERVED2	= 0x5,
        COMBND_RESERVED3	= 0x8,
        COMBND_RESERVED4	= 0x10
    } 	RPCOPT_PROPERTIES;

typedef 
enum tagRPCOPT_SERVER_LOCALITY_VALUES
    {
        SERVER_LOCALITY_PROCESS_LOCAL	= 0,
        SERVER_LOCALITY_MACHINE_LOCAL	= 1,
        SERVER_LOCALITY_REMOTE	= 2
    } 	RPCOPT_SERVER_LOCALITY_VALUES;



extern RPC_IF_HANDLE __MIDL_itf_objidl_0000_0025_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_objidl_0000_0025_v0_0_s_ifspec;

#ifndef __IRpcOptions_INTERFACE_DEFINED__
#define __IRpcOptions_INTERFACE_DEFINED__

/* interface IRpcOptions */
/* [uuid][local][object] */ 


EXTERN_C const IID IID_IRpcOptions;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("00000144-0000-0000-C000-000000000046")
    IRpcOptions : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE Set( 
            /* [annotation][in] */ 
            _In_  IUnknown *pPrx,
            /* [annotation][in] */ 
            _In_  RPCOPT_PROPERTIES dwProperty,
            /* [annotation][in] */ 
            _In_  ULONG_PTR dwValue) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Query( 
            /* [annotation][in] */ 
            _In_  IUnknown *pPrx,
            /* [annotation][in] */ 
            _In_  RPCOPT_PROPERTIES dwProperty,
            /* [annotation][out] */ 
            _Out_  ULONG_PTR *pdwValue) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IRpcOptionsVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IRpcOptions * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IRpcOptions * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IRpcOptions * This);
        
        DECLSPEC_XFGVIRT(IRpcOptions, Set)
        HRESULT ( STDMETHODCALLTYPE *Set )( 
            IRpcOptions * This,
            /* [annotation][in] */ 
            _In_  IUnknown *pPrx,
            /* [annotation][in] */ 
            _In_  RPCOPT_PROPERTIES dwProperty,
            /* [annotation][in] */ 
            _In_  ULONG_PTR dwValue);
        
        DECLSPEC_XFGVIRT(IRpcOptions, Query)
        HRESULT ( STDMETHODCALLTYPE *Query )( 
            IRpcOptions * This,
            /* [annotation][in] */ 
            _In_  IUnknown *pPrx,
            /* [annotation][in] */ 
            _In_  RPCOPT_PROPERTIES dwProperty,
            /* [annotation][out] */ 
            _Out_  ULONG_PTR *pdwValue);
        
        END_INTERFACE
    } IRpcOptionsVtbl;

    interface IRpcOptions
    {
        CONST_VTBL struct IRpcOptionsVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IRpcOptions_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IRpcOptions_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IRpcOptions_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IRpcOptions_Set(This,pPrx,dwProperty,dwValue)	\
    ( (This)->lpVtbl -> Set(This,pPrx,dwProperty,dwValue) ) 

#define IRpcOptions_Query(This,pPrx,dwProperty,pdwValue)	\
    ( (This)->lpVtbl -> Query(This,pPrx,dwProperty,pdwValue) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IRpcOptions_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_objidl_0000_0026 */
/* [local] */ 

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES) */
#pragma endregion
#pragma region Application Family or OneCore Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES)
typedef 
enum tagGLOBALOPT_PROPERTIES
    {
        COMGLB_EXCEPTION_HANDLING	= 1,
        COMGLB_APPID	= 2,
        COMGLB_RPC_THREADPOOL_SETTING	= 3,
        COMGLB_RO_SETTINGS	= 4,
        COMGLB_UNMARSHALING_POLICY	= 5,
        COMGLB_PROPERTIES_RESERVED1	= 6,
        COMGLB_PROPERTIES_RESERVED2	= 7,
        COMGLB_PROPERTIES_RESERVED3	= 8
    } 	GLOBALOPT_PROPERTIES;

typedef 
enum tagGLOBALOPT_EH_VALUES
    {
        COMGLB_EXCEPTION_HANDLE	= 0,
        COMGLB_EXCEPTION_DONOT_HANDLE_FATAL	= 1,
        COMGLB_EXCEPTION_DONOT_HANDLE	= COMGLB_EXCEPTION_DONOT_HANDLE_FATAL,
        COMGLB_EXCEPTION_DONOT_HANDLE_ANY	= 2
    } 	GLOBALOPT_EH_VALUES;

typedef 
enum tagGLOBALOPT_RPCTP_VALUES
    {
        COMGLB_RPC_THREADPOOL_SETTING_DEFAULT_POOL	= 0,
        COMGLB_RPC_THREADPOOL_SETTING_PRIVATE_POOL	= 1
    } 	GLOBALOPT_RPCTP_VALUES;

typedef 
enum tagGLOBALOPT_RO_FLAGS
    {
        COMGLB_STA_MODALLOOP_REMOVE_TOUCH_MESSAGES	= 0x1,
        COMGLB_STA_MODALLOOP_SHARED_QUEUE_REMOVE_INPUT_MESSAGES	= 0x2,
        COMGLB_STA_MODALLOOP_SHARED_QUEUE_DONOT_REMOVE_INPUT_MESSAGES	= 0x4,
        COMGLB_FAST_RUNDOWN	= 0x8,
        COMGLB_RESERVED1	= 0x10,
        COMGLB_RESERVED2	= 0x20,
        COMGLB_RESERVED3	= 0x40,
        COMGLB_STA_MODALLOOP_SHARED_QUEUE_REORDER_POINTER_MESSAGES	= 0x80,
        COMGLB_RESERVED4	= 0x100,
        COMGLB_RESERVED5	= 0x200,
        COMGLB_RESERVED6	= 0x400
    } 	GLOBALOPT_RO_FLAGS;

typedef 
enum tagGLOBALOPT_UNMARSHALING_POLICY_VALUES
    {
        COMGLB_UNMARSHALING_POLICY_NORMAL	= 0,
        COMGLB_UNMARSHALING_POLICY_STRONG	= 1,
        COMGLB_UNMARSHALING_POLICY_HYBRID	= 2
    } 	GLOBALOPT_UNMARSHALING_POLICY_VALUES;



extern RPC_IF_HANDLE __MIDL_itf_objidl_0000_0026_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_objidl_0000_0026_v0_0_s_ifspec;

#ifndef __IGlobalOptions_INTERFACE_DEFINED__
#define __IGlobalOptions_INTERFACE_DEFINED__

/* interface IGlobalOptions */
/* [uuid][unique][local][object] */ 


EXTERN_C const IID IID_IGlobalOptions;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("0000015B-0000-0000-C000-000000000046")
    IGlobalOptions : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE Set( 
            /* [annotation][in] */ 
            _In_  GLOBALOPT_PROPERTIES dwProperty,
            /* [annotation][in] */ 
            _In_  ULONG_PTR dwValue) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Query( 
            /* [annotation][in] */ 
            _In_  GLOBALOPT_PROPERTIES dwProperty,
            /* [annotation][out] */ 
            _Out_  ULONG_PTR *pdwValue) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IGlobalOptionsVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IGlobalOptions * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IGlobalOptions * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IGlobalOptions * This);
        
        DECLSPEC_XFGVIRT(IGlobalOptions, Set)
        HRESULT ( STDMETHODCALLTYPE *Set )( 
            IGlobalOptions * This,
            /* [annotation][in] */ 
            _In_  GLOBALOPT_PROPERTIES dwProperty,
            /* [annotation][in] */ 
            _In_  ULONG_PTR dwValue);
        
        DECLSPEC_XFGVIRT(IGlobalOptions, Query)
        HRESULT ( STDMETHODCALLTYPE *Query )( 
            IGlobalOptions * This,
            /* [annotation][in] */ 
            _In_  GLOBALOPT_PROPERTIES dwProperty,
            /* [annotation][out] */ 
            _Out_  ULONG_PTR *pdwValue);
        
        END_INTERFACE
    } IGlobalOptionsVtbl;

    interface IGlobalOptions
    {
        CONST_VTBL struct IGlobalOptionsVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IGlobalOptions_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IGlobalOptions_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IGlobalOptions_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IGlobalOptions_Set(This,dwProperty,dwValue)	\
    ( (This)->lpVtbl -> Set(This,dwProperty,dwValue) ) 

#define IGlobalOptions_Query(This,dwProperty,pdwValue)	\
    ( (This)->lpVtbl -> Query(This,dwProperty,pdwValue) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IGlobalOptions_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_objidl_0000_0027 */
/* [local] */ 

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES) */
#pragma endregion
#endif //DCOM
#pragma region Application Family or OneCore Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES)


extern RPC_IF_HANDLE __MIDL_itf_objidl_0000_0027_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_objidl_0000_0027_v0_0_s_ifspec;

#ifndef __ISurrogate_INTERFACE_DEFINED__
#define __ISurrogate_INTERFACE_DEFINED__

/* interface ISurrogate */
/* [object][unique][version][uuid] */ 

typedef /* [unique] */  __RPC_unique_pointer ISurrogate *LPSURROGATE;


EXTERN_C const IID IID_ISurrogate;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("00000022-0000-0000-C000-000000000046")
    ISurrogate : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE LoadDllServer( 
            /* [in] */ __RPC__in REFCLSID Clsid) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE FreeSurrogate( void) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ISurrogateVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in ISurrogate * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in ISurrogate * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in ISurrogate * This);
        
        DECLSPEC_XFGVIRT(ISurrogate, LoadDllServer)
        HRESULT ( STDMETHODCALLTYPE *LoadDllServer )( 
            __RPC__in ISurrogate * This,
            /* [in] */ __RPC__in REFCLSID Clsid);
        
        DECLSPEC_XFGVIRT(ISurrogate, FreeSurrogate)
        HRESULT ( STDMETHODCALLTYPE *FreeSurrogate )( 
            __RPC__in ISurrogate * This);
        
        END_INTERFACE
    } ISurrogateVtbl;

    interface ISurrogate
    {
        CONST_VTBL struct ISurrogateVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ISurrogate_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ISurrogate_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ISurrogate_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ISurrogate_LoadDllServer(This,Clsid)	\
    ( (This)->lpVtbl -> LoadDllServer(This,Clsid) ) 

#define ISurrogate_FreeSurrogate(This)	\
    ( (This)->lpVtbl -> FreeSurrogate(This) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ISurrogate_INTERFACE_DEFINED__ */


#ifndef __IGlobalInterfaceTable_INTERFACE_DEFINED__
#define __IGlobalInterfaceTable_INTERFACE_DEFINED__

/* interface IGlobalInterfaceTable */
/* [uuid][object][local] */ 

typedef /* [unique] */ IGlobalInterfaceTable *LPGLOBALINTERFACETABLE;


EXTERN_C const IID IID_IGlobalInterfaceTable;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("00000146-0000-0000-C000-000000000046")
    IGlobalInterfaceTable : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE RegisterInterfaceInGlobal( 
            /* [annotation][in] */ 
            _In_  IUnknown *pUnk,
            /* [annotation][in] */ 
            _In_  REFIID riid,
            /* [annotation][out] */ 
            _Out_  DWORD *pdwCookie) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE RevokeInterfaceFromGlobal( 
            /* [annotation][in] */ 
            _In_  DWORD dwCookie) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetInterfaceFromGlobal( 
            /* [annotation][in] */ 
            _In_  DWORD dwCookie,
            /* [annotation][in] */ 
            _In_  REFIID riid,
            /* [annotation][iid_is][out] */ 
            _Outptr_  void **ppv) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IGlobalInterfaceTableVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IGlobalInterfaceTable * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IGlobalInterfaceTable * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IGlobalInterfaceTable * This);
        
        DECLSPEC_XFGVIRT(IGlobalInterfaceTable, RegisterInterfaceInGlobal)
        HRESULT ( STDMETHODCALLTYPE *RegisterInterfaceInGlobal )( 
            IGlobalInterfaceTable * This,
            /* [annotation][in] */ 
            _In_  IUnknown *pUnk,
            /* [annotation][in] */ 
            _In_  REFIID riid,
            /* [annotation][out] */ 
            _Out_  DWORD *pdwCookie);
        
        DECLSPEC_XFGVIRT(IGlobalInterfaceTable, RevokeInterfaceFromGlobal)
        HRESULT ( STDMETHODCALLTYPE *RevokeInterfaceFromGlobal )( 
            IGlobalInterfaceTable * This,
            /* [annotation][in] */ 
            _In_  DWORD dwCookie);
        
        DECLSPEC_XFGVIRT(IGlobalInterfaceTable, GetInterfaceFromGlobal)
        HRESULT ( STDMETHODCALLTYPE *GetInterfaceFromGlobal )( 
            IGlobalInterfaceTable * This,
            /* [annotation][in] */ 
            _In_  DWORD dwCookie,
            /* [annotation][in] */ 
            _In_  REFIID riid,
            /* [annotation][iid_is][out] */ 
            _Outptr_  void **ppv);
        
        END_INTERFACE
    } IGlobalInterfaceTableVtbl;

    interface IGlobalInterfaceTable
    {
        CONST_VTBL struct IGlobalInterfaceTableVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IGlobalInterfaceTable_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IGlobalInterfaceTable_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IGlobalInterfaceTable_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IGlobalInterfaceTable_RegisterInterfaceInGlobal(This,pUnk,riid,pdwCookie)	\
    ( (This)->lpVtbl -> RegisterInterfaceInGlobal(This,pUnk,riid,pdwCookie) ) 

#define IGlobalInterfaceTable_RevokeInterfaceFromGlobal(This,dwCookie)	\
    ( (This)->lpVtbl -> RevokeInterfaceFromGlobal(This,dwCookie) ) 

#define IGlobalInterfaceTable_GetInterfaceFromGlobal(This,dwCookie,riid,ppv)	\
    ( (This)->lpVtbl -> GetInterfaceFromGlobal(This,dwCookie,riid,ppv) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IGlobalInterfaceTable_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_objidl_0000_0029 */
/* [local] */ 

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES) */
#pragma endregion
#pragma region Desktop Family or OneCore Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES)


extern RPC_IF_HANDLE __MIDL_itf_objidl_0000_0029_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_objidl_0000_0029_v0_0_s_ifspec;

#ifndef __ISynchronize_INTERFACE_DEFINED__
#define __ISynchronize_INTERFACE_DEFINED__

/* interface ISynchronize */
/* [uuid][object] */ 


EXTERN_C const IID IID_ISynchronize;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("00000030-0000-0000-C000-000000000046")
    ISynchronize : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE Wait( 
            /* [in] */ DWORD dwFlags,
            /* [in] */ DWORD dwMilliseconds) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Signal( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Reset( void) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ISynchronizeVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in ISynchronize * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in ISynchronize * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in ISynchronize * This);
        
        DECLSPEC_XFGVIRT(ISynchronize, Wait)
        HRESULT ( STDMETHODCALLTYPE *Wait )( 
            __RPC__in ISynchronize * This,
            /* [in] */ DWORD dwFlags,
            /* [in] */ DWORD dwMilliseconds);
        
        DECLSPEC_XFGVIRT(ISynchronize, Signal)
        HRESULT ( STDMETHODCALLTYPE *Signal )( 
            __RPC__in ISynchronize * This);
        
        DECLSPEC_XFGVIRT(ISynchronize, Reset)
        HRESULT ( STDMETHODCALLTYPE *Reset )( 
            __RPC__in ISynchronize * This);
        
        END_INTERFACE
    } ISynchronizeVtbl;

    interface ISynchronize
    {
        CONST_VTBL struct ISynchronizeVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ISynchronize_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ISynchronize_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ISynchronize_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ISynchronize_Wait(This,dwFlags,dwMilliseconds)	\
    ( (This)->lpVtbl -> Wait(This,dwFlags,dwMilliseconds) ) 

#define ISynchronize_Signal(This)	\
    ( (This)->lpVtbl -> Signal(This) ) 

#define ISynchronize_Reset(This)	\
    ( (This)->lpVtbl -> Reset(This) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ISynchronize_INTERFACE_DEFINED__ */


#ifndef __ISynchronizeHandle_INTERFACE_DEFINED__
#define __ISynchronizeHandle_INTERFACE_DEFINED__

/* interface ISynchronizeHandle */
/* [uuid][object][local] */ 


EXTERN_C const IID IID_ISynchronizeHandle;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("00000031-0000-0000-C000-000000000046")
    ISynchronizeHandle : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetHandle( 
            /* [annotation][out] */ 
            _Out_  HANDLE *ph) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ISynchronizeHandleVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            ISynchronizeHandle * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            ISynchronizeHandle * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            ISynchronizeHandle * This);
        
        DECLSPEC_XFGVIRT(ISynchronizeHandle, GetHandle)
        HRESULT ( STDMETHODCALLTYPE *GetHandle )( 
            ISynchronizeHandle * This,
            /* [annotation][out] */ 
            _Out_  HANDLE *ph);
        
        END_INTERFACE
    } ISynchronizeHandleVtbl;

    interface ISynchronizeHandle
    {
        CONST_VTBL struct ISynchronizeHandleVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ISynchronizeHandle_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ISynchronizeHandle_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ISynchronizeHandle_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ISynchronizeHandle_GetHandle(This,ph)	\
    ( (This)->lpVtbl -> GetHandle(This,ph) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ISynchronizeHandle_INTERFACE_DEFINED__ */


#ifndef __ISynchronizeEvent_INTERFACE_DEFINED__
#define __ISynchronizeEvent_INTERFACE_DEFINED__

/* interface ISynchronizeEvent */
/* [uuid][object][local] */ 


EXTERN_C const IID IID_ISynchronizeEvent;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("00000032-0000-0000-C000-000000000046")
    ISynchronizeEvent : public ISynchronizeHandle
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE SetEventHandle( 
            /* [annotation][in] */ 
            _In_  HANDLE *ph) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ISynchronizeEventVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            ISynchronizeEvent * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            ISynchronizeEvent * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            ISynchronizeEvent * This);
        
        DECLSPEC_XFGVIRT(ISynchronizeHandle, GetHandle)
        HRESULT ( STDMETHODCALLTYPE *GetHandle )( 
            ISynchronizeEvent * This,
            /* [annotation][out] */ 
            _Out_  HANDLE *ph);
        
        DECLSPEC_XFGVIRT(ISynchronizeEvent, SetEventHandle)
        HRESULT ( STDMETHODCALLTYPE *SetEventHandle )( 
            ISynchronizeEvent * This,
            /* [annotation][in] */ 
            _In_  HANDLE *ph);
        
        END_INTERFACE
    } ISynchronizeEventVtbl;

    interface ISynchronizeEvent
    {
        CONST_VTBL struct ISynchronizeEventVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ISynchronizeEvent_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ISynchronizeEvent_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ISynchronizeEvent_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ISynchronizeEvent_GetHandle(This,ph)	\
    ( (This)->lpVtbl -> GetHandle(This,ph) ) 


#define ISynchronizeEvent_SetEventHandle(This,ph)	\
    ( (This)->lpVtbl -> SetEventHandle(This,ph) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ISynchronizeEvent_INTERFACE_DEFINED__ */


#ifndef __ISynchronizeContainer_INTERFACE_DEFINED__
#define __ISynchronizeContainer_INTERFACE_DEFINED__

/* interface ISynchronizeContainer */
/* [uuid][object][local] */ 


EXTERN_C const IID IID_ISynchronizeContainer;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("00000033-0000-0000-C000-000000000046")
    ISynchronizeContainer : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE AddSynchronize( 
            /* [annotation][in] */ 
            _In_  ISynchronize *pSync) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE WaitMultiple( 
            /* [annotation][in] */ 
            _In_  DWORD dwFlags,
            /* [annotation][in] */ 
            _In_  DWORD dwTimeOut,
            /* [annotation][out] */ 
            _Outptr_  ISynchronize **ppSync) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ISynchronizeContainerVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            ISynchronizeContainer * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            ISynchronizeContainer * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            ISynchronizeContainer * This);
        
        DECLSPEC_XFGVIRT(ISynchronizeContainer, AddSynchronize)
        HRESULT ( STDMETHODCALLTYPE *AddSynchronize )( 
            ISynchronizeContainer * This,
            /* [annotation][in] */ 
            _In_  ISynchronize *pSync);
        
        DECLSPEC_XFGVIRT(ISynchronizeContainer, WaitMultiple)
        HRESULT ( STDMETHODCALLTYPE *WaitMultiple )( 
            ISynchronizeContainer * This,
            /* [annotation][in] */ 
            _In_  DWORD dwFlags,
            /* [annotation][in] */ 
            _In_  DWORD dwTimeOut,
            /* [annotation][out] */ 
            _Outptr_  ISynchronize **ppSync);
        
        END_INTERFACE
    } ISynchronizeContainerVtbl;

    interface ISynchronizeContainer
    {
        CONST_VTBL struct ISynchronizeContainerVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ISynchronizeContainer_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ISynchronizeContainer_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ISynchronizeContainer_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ISynchronizeContainer_AddSynchronize(This,pSync)	\
    ( (This)->lpVtbl -> AddSynchronize(This,pSync) ) 

#define ISynchronizeContainer_WaitMultiple(This,dwFlags,dwTimeOut,ppSync)	\
    ( (This)->lpVtbl -> WaitMultiple(This,dwFlags,dwTimeOut,ppSync) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ISynchronizeContainer_INTERFACE_DEFINED__ */


#ifndef __ISynchronizeMutex_INTERFACE_DEFINED__
#define __ISynchronizeMutex_INTERFACE_DEFINED__

/* interface ISynchronizeMutex */
/* [uuid][object][local] */ 


EXTERN_C const IID IID_ISynchronizeMutex;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("00000025-0000-0000-C000-000000000046")
    ISynchronizeMutex : public ISynchronize
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE ReleaseMutex( void) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ISynchronizeMutexVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            ISynchronizeMutex * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            ISynchronizeMutex * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            ISynchronizeMutex * This);
        
        DECLSPEC_XFGVIRT(ISynchronize, Wait)
        HRESULT ( STDMETHODCALLTYPE *Wait )( 
            ISynchronizeMutex * This,
            /* [in] */ DWORD dwFlags,
            /* [in] */ DWORD dwMilliseconds);
        
        DECLSPEC_XFGVIRT(ISynchronize, Signal)
        HRESULT ( STDMETHODCALLTYPE *Signal )( 
            ISynchronizeMutex * This);
        
        DECLSPEC_XFGVIRT(ISynchronize, Reset)
        HRESULT ( STDMETHODCALLTYPE *Reset )( 
            ISynchronizeMutex * This);
        
        DECLSPEC_XFGVIRT(ISynchronizeMutex, ReleaseMutex)
        HRESULT ( STDMETHODCALLTYPE *ReleaseMutex )( 
            ISynchronizeMutex * This);
        
        END_INTERFACE
    } ISynchronizeMutexVtbl;

    interface ISynchronizeMutex
    {
        CONST_VTBL struct ISynchronizeMutexVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ISynchronizeMutex_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ISynchronizeMutex_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ISynchronizeMutex_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ISynchronizeMutex_Wait(This,dwFlags,dwMilliseconds)	\
    ( (This)->lpVtbl -> Wait(This,dwFlags,dwMilliseconds) ) 

#define ISynchronizeMutex_Signal(This)	\
    ( (This)->lpVtbl -> Signal(This) ) 

#define ISynchronizeMutex_Reset(This)	\
    ( (This)->lpVtbl -> Reset(This) ) 


#define ISynchronizeMutex_ReleaseMutex(This)	\
    ( (This)->lpVtbl -> ReleaseMutex(This) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ISynchronizeMutex_INTERFACE_DEFINED__ */


#ifndef __ICancelMethodCalls_INTERFACE_DEFINED__
#define __ICancelMethodCalls_INTERFACE_DEFINED__

/* interface ICancelMethodCalls */
/* [uuid][object][local] */ 

typedef /* [unique] */ ICancelMethodCalls *LPCANCELMETHODCALLS;


EXTERN_C const IID IID_ICancelMethodCalls;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("00000029-0000-0000-C000-000000000046")
    ICancelMethodCalls : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE Cancel( 
            /* [annotation][in] */ 
            _In_  ULONG ulSeconds) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE TestCancel( void) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ICancelMethodCallsVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            ICancelMethodCalls * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            ICancelMethodCalls * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            ICancelMethodCalls * This);
        
        DECLSPEC_XFGVIRT(ICancelMethodCalls, Cancel)
        HRESULT ( STDMETHODCALLTYPE *Cancel )( 
            ICancelMethodCalls * This,
            /* [annotation][in] */ 
            _In_  ULONG ulSeconds);
        
        DECLSPEC_XFGVIRT(ICancelMethodCalls, TestCancel)
        HRESULT ( STDMETHODCALLTYPE *TestCancel )( 
            ICancelMethodCalls * This);
        
        END_INTERFACE
    } ICancelMethodCallsVtbl;

    interface ICancelMethodCalls
    {
        CONST_VTBL struct ICancelMethodCallsVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ICancelMethodCalls_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ICancelMethodCalls_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ICancelMethodCalls_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ICancelMethodCalls_Cancel(This,ulSeconds)	\
    ( (This)->lpVtbl -> Cancel(This,ulSeconds) ) 

#define ICancelMethodCalls_TestCancel(This)	\
    ( (This)->lpVtbl -> TestCancel(This) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ICancelMethodCalls_INTERFACE_DEFINED__ */


#ifndef __IAsyncManager_INTERFACE_DEFINED__
#define __IAsyncManager_INTERFACE_DEFINED__

/* interface IAsyncManager */
/* [uuid][object][local] */ 

typedef 
enum tagDCOM_CALL_STATE
    {
        DCOM_NONE	= 0,
        DCOM_CALL_COMPLETE	= 0x1,
        DCOM_CALL_CANCELED	= 0x2
    } 	DCOM_CALL_STATE;


EXTERN_C const IID IID_IAsyncManager;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("0000002A-0000-0000-C000-000000000046")
    IAsyncManager : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE CompleteCall( 
            /* [annotation][in] */ 
            _In_  HRESULT Result) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetCallContext( 
            /* [annotation][in] */ 
            _In_  REFIID riid,
            /* [annotation][out] */ 
            _Outptr_  void **pInterface) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetState( 
            /* [annotation][out] */ 
            _Out_  ULONG *pulStateFlags) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IAsyncManagerVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IAsyncManager * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IAsyncManager * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IAsyncManager * This);
        
        DECLSPEC_XFGVIRT(IAsyncManager, CompleteCall)
        HRESULT ( STDMETHODCALLTYPE *CompleteCall )( 
            IAsyncManager * This,
            /* [annotation][in] */ 
            _In_  HRESULT Result);
        
        DECLSPEC_XFGVIRT(IAsyncManager, GetCallContext)
        HRESULT ( STDMETHODCALLTYPE *GetCallContext )( 
            IAsyncManager * This,
            /* [annotation][in] */ 
            _In_  REFIID riid,
            /* [annotation][out] */ 
            _Outptr_  void **pInterface);
        
        DECLSPEC_XFGVIRT(IAsyncManager, GetState)
        HRESULT ( STDMETHODCALLTYPE *GetState )( 
            IAsyncManager * This,
            /* [annotation][out] */ 
            _Out_  ULONG *pulStateFlags);
        
        END_INTERFACE
    } IAsyncManagerVtbl;

    interface IAsyncManager
    {
        CONST_VTBL struct IAsyncManagerVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IAsyncManager_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IAsyncManager_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IAsyncManager_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IAsyncManager_CompleteCall(This,Result)	\
    ( (This)->lpVtbl -> CompleteCall(This,Result) ) 

#define IAsyncManager_GetCallContext(This,riid,pInterface)	\
    ( (This)->lpVtbl -> GetCallContext(This,riid,pInterface) ) 

#define IAsyncManager_GetState(This,pulStateFlags)	\
    ( (This)->lpVtbl -> GetState(This,pulStateFlags) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IAsyncManager_INTERFACE_DEFINED__ */


#ifndef __ICallFactory_INTERFACE_DEFINED__
#define __ICallFactory_INTERFACE_DEFINED__

/* interface ICallFactory */
/* [unique][uuid][object][local] */ 


EXTERN_C const IID IID_ICallFactory;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("1c733a30-2a1c-11ce-ade5-00aa0044773d")
    ICallFactory : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE CreateCall( 
            /* [annotation][in] */ 
            _In_  REFIID riid,
            /* [annotation][in] */ 
            _In_opt_  IUnknown *pCtrlUnk,
            /* [annotation][in] */ 
            _In_  REFIID riid2,
            /* [annotation][iid_is][out] */ 
            _Outptr_  IUnknown **ppv) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ICallFactoryVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            ICallFactory * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            ICallFactory * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            ICallFactory * This);
        
        DECLSPEC_XFGVIRT(ICallFactory, CreateCall)
        HRESULT ( STDMETHODCALLTYPE *CreateCall )( 
            ICallFactory * This,
            /* [annotation][in] */ 
            _In_  REFIID riid,
            /* [annotation][in] */ 
            _In_opt_  IUnknown *pCtrlUnk,
            /* [annotation][in] */ 
            _In_  REFIID riid2,
            /* [annotation][iid_is][out] */ 
            _Outptr_  IUnknown **ppv);
        
        END_INTERFACE
    } ICallFactoryVtbl;

    interface ICallFactory
    {
        CONST_VTBL struct ICallFactoryVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ICallFactory_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ICallFactory_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ICallFactory_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ICallFactory_CreateCall(This,riid,pCtrlUnk,riid2,ppv)	\
    ( (This)->lpVtbl -> CreateCall(This,riid,pCtrlUnk,riid2,ppv) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ICallFactory_INTERFACE_DEFINED__ */


#ifndef __IRpcHelper_INTERFACE_DEFINED__
#define __IRpcHelper_INTERFACE_DEFINED__

/* interface IRpcHelper */
/* [object][local][unique][version][uuid] */ 


EXTERN_C const IID IID_IRpcHelper;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("00000149-0000-0000-C000-000000000046")
    IRpcHelper : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetDCOMProtocolVersion( 
            /* [annotation][out] */ 
            _Out_  DWORD *pComVersion) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetIIDFromOBJREF( 
            /* [annotation][in] */ 
            _In_  void *pObjRef,
            /* [annotation][out] */ 
            _Outptr_  IID **piid) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IRpcHelperVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IRpcHelper * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IRpcHelper * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IRpcHelper * This);
        
        DECLSPEC_XFGVIRT(IRpcHelper, GetDCOMProtocolVersion)
        HRESULT ( STDMETHODCALLTYPE *GetDCOMProtocolVersion )( 
            IRpcHelper * This,
            /* [annotation][out] */ 
            _Out_  DWORD *pComVersion);
        
        DECLSPEC_XFGVIRT(IRpcHelper, GetIIDFromOBJREF)
        HRESULT ( STDMETHODCALLTYPE *GetIIDFromOBJREF )( 
            IRpcHelper * This,
            /* [annotation][in] */ 
            _In_  void *pObjRef,
            /* [annotation][out] */ 
            _Outptr_  IID **piid);
        
        END_INTERFACE
    } IRpcHelperVtbl;

    interface IRpcHelper
    {
        CONST_VTBL struct IRpcHelperVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IRpcHelper_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IRpcHelper_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IRpcHelper_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IRpcHelper_GetDCOMProtocolVersion(This,pComVersion)	\
    ( (This)->lpVtbl -> GetDCOMProtocolVersion(This,pComVersion) ) 

#define IRpcHelper_GetIIDFromOBJREF(This,pObjRef,piid)	\
    ( (This)->lpVtbl -> GetIIDFromOBJREF(This,pObjRef,piid) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IRpcHelper_INTERFACE_DEFINED__ */


#ifndef __IReleaseMarshalBuffers_INTERFACE_DEFINED__
#define __IReleaseMarshalBuffers_INTERFACE_DEFINED__

/* interface IReleaseMarshalBuffers */
/* [uuid][object][local] */ 


EXTERN_C const IID IID_IReleaseMarshalBuffers;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("eb0cb9e8-7996-11d2-872e-0000f8080859")
    IReleaseMarshalBuffers : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE ReleaseMarshalBuffer( 
            /* [annotation][out][in] */ 
            _Inout_  RPCOLEMESSAGE *pMsg,
            /* [annotation][in] */ 
            _In_  DWORD dwFlags,
            /* [annotation][unique][in] */ 
            _In_opt_  IUnknown *pChnl) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IReleaseMarshalBuffersVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IReleaseMarshalBuffers * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IReleaseMarshalBuffers * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IReleaseMarshalBuffers * This);
        
        DECLSPEC_XFGVIRT(IReleaseMarshalBuffers, ReleaseMarshalBuffer)
        HRESULT ( STDMETHODCALLTYPE *ReleaseMarshalBuffer )( 
            IReleaseMarshalBuffers * This,
            /* [annotation][out][in] */ 
            _Inout_  RPCOLEMESSAGE *pMsg,
            /* [annotation][in] */ 
            _In_  DWORD dwFlags,
            /* [annotation][unique][in] */ 
            _In_opt_  IUnknown *pChnl);
        
        END_INTERFACE
    } IReleaseMarshalBuffersVtbl;

    interface IReleaseMarshalBuffers
    {
        CONST_VTBL struct IReleaseMarshalBuffersVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IReleaseMarshalBuffers_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IReleaseMarshalBuffers_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IReleaseMarshalBuffers_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IReleaseMarshalBuffers_ReleaseMarshalBuffer(This,pMsg,dwFlags,pChnl)	\
    ( (This)->lpVtbl -> ReleaseMarshalBuffer(This,pMsg,dwFlags,pChnl) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IReleaseMarshalBuffers_INTERFACE_DEFINED__ */


#ifndef __IWaitMultiple_INTERFACE_DEFINED__
#define __IWaitMultiple_INTERFACE_DEFINED__

/* interface IWaitMultiple */
/* [uuid][object][local] */ 


EXTERN_C const IID IID_IWaitMultiple;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("0000002B-0000-0000-C000-000000000046")
    IWaitMultiple : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE WaitMultiple( 
            /* [annotation][in] */ 
            _In_  DWORD timeout,
            /* [annotation][out] */ 
            _Outptr_  ISynchronize **pSync) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE AddSynchronize( 
            /* [annotation][in] */ 
            _In_  ISynchronize *pSync) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IWaitMultipleVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IWaitMultiple * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IWaitMultiple * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IWaitMultiple * This);
        
        DECLSPEC_XFGVIRT(IWaitMultiple, WaitMultiple)
        HRESULT ( STDMETHODCALLTYPE *WaitMultiple )( 
            IWaitMultiple * This,
            /* [annotation][in] */ 
            _In_  DWORD timeout,
            /* [annotation][out] */ 
            _Outptr_  ISynchronize **pSync);
        
        DECLSPEC_XFGVIRT(IWaitMultiple, AddSynchronize)
        HRESULT ( STDMETHODCALLTYPE *AddSynchronize )( 
            IWaitMultiple * This,
            /* [annotation][in] */ 
            _In_  ISynchronize *pSync);
        
        END_INTERFACE
    } IWaitMultipleVtbl;

    interface IWaitMultiple
    {
        CONST_VTBL struct IWaitMultipleVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IWaitMultiple_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IWaitMultiple_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IWaitMultiple_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IWaitMultiple_WaitMultiple(This,timeout,pSync)	\
    ( (This)->lpVtbl -> WaitMultiple(This,timeout,pSync) ) 

#define IWaitMultiple_AddSynchronize(This,pSync)	\
    ( (This)->lpVtbl -> AddSynchronize(This,pSync) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IWaitMultiple_INTERFACE_DEFINED__ */


#ifndef __IAddrTrackingControl_INTERFACE_DEFINED__
#define __IAddrTrackingControl_INTERFACE_DEFINED__

/* interface IAddrTrackingControl */
/* [uuid][object][local] */ 

typedef /* [unique] */ IAddrTrackingControl *LPADDRTRACKINGCONTROL;


EXTERN_C const IID IID_IAddrTrackingControl;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("00000147-0000-0000-C000-000000000046")
    IAddrTrackingControl : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE EnableCOMDynamicAddrTracking( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE DisableCOMDynamicAddrTracking( void) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IAddrTrackingControlVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IAddrTrackingControl * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IAddrTrackingControl * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IAddrTrackingControl * This);
        
        DECLSPEC_XFGVIRT(IAddrTrackingControl, EnableCOMDynamicAddrTracking)
        HRESULT ( STDMETHODCALLTYPE *EnableCOMDynamicAddrTracking )( 
            IAddrTrackingControl * This);
        
        DECLSPEC_XFGVIRT(IAddrTrackingControl, DisableCOMDynamicAddrTracking)
        HRESULT ( STDMETHODCALLTYPE *DisableCOMDynamicAddrTracking )( 
            IAddrTrackingControl * This);
        
        END_INTERFACE
    } IAddrTrackingControlVtbl;

    interface IAddrTrackingControl
    {
        CONST_VTBL struct IAddrTrackingControlVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IAddrTrackingControl_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IAddrTrackingControl_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IAddrTrackingControl_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IAddrTrackingControl_EnableCOMDynamicAddrTracking(This)	\
    ( (This)->lpVtbl -> EnableCOMDynamicAddrTracking(This) ) 

#define IAddrTrackingControl_DisableCOMDynamicAddrTracking(This)	\
    ( (This)->lpVtbl -> DisableCOMDynamicAddrTracking(This) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IAddrTrackingControl_INTERFACE_DEFINED__ */


#ifndef __IAddrExclusionControl_INTERFACE_DEFINED__
#define __IAddrExclusionControl_INTERFACE_DEFINED__

/* interface IAddrExclusionControl */
/* [uuid][object][local] */ 

typedef /* [unique] */ IAddrExclusionControl *LPADDREXCLUSIONCONTROL;


EXTERN_C const IID IID_IAddrExclusionControl;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("00000148-0000-0000-C000-000000000046")
    IAddrExclusionControl : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetCurrentAddrExclusionList( 
            /* [annotation][in] */ 
            _In_  REFIID riid,
            /* [annotation][iid_is][out] */ 
            _Outptr_  void **ppEnumerator) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE UpdateAddrExclusionList( 
            /* [annotation][in] */ 
            _In_  IUnknown *pEnumerator) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IAddrExclusionControlVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IAddrExclusionControl * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IAddrExclusionControl * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IAddrExclusionControl * This);
        
        DECLSPEC_XFGVIRT(IAddrExclusionControl, GetCurrentAddrExclusionList)
        HRESULT ( STDMETHODCALLTYPE *GetCurrentAddrExclusionList )( 
            IAddrExclusionControl * This,
            /* [annotation][in] */ 
            _In_  REFIID riid,
            /* [annotation][iid_is][out] */ 
            _Outptr_  void **ppEnumerator);
        
        DECLSPEC_XFGVIRT(IAddrExclusionControl, UpdateAddrExclusionList)
        HRESULT ( STDMETHODCALLTYPE *UpdateAddrExclusionList )( 
            IAddrExclusionControl * This,
            /* [annotation][in] */ 
            _In_  IUnknown *pEnumerator);
        
        END_INTERFACE
    } IAddrExclusionControlVtbl;

    interface IAddrExclusionControl
    {
        CONST_VTBL struct IAddrExclusionControlVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IAddrExclusionControl_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IAddrExclusionControl_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IAddrExclusionControl_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IAddrExclusionControl_GetCurrentAddrExclusionList(This,riid,ppEnumerator)	\
    ( (This)->lpVtbl -> GetCurrentAddrExclusionList(This,riid,ppEnumerator) ) 

#define IAddrExclusionControl_UpdateAddrExclusionList(This,pEnumerator)	\
    ( (This)->lpVtbl -> UpdateAddrExclusionList(This,pEnumerator) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IAddrExclusionControl_INTERFACE_DEFINED__ */


#ifndef __IPipeByte_INTERFACE_DEFINED__
#define __IPipeByte_INTERFACE_DEFINED__

/* interface IPipeByte */
/* [unique][async_uuid][uuid][object] */ 


EXTERN_C const IID IID_IPipeByte;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("DB2F3ACA-2F86-11d1-8E04-00C04FB9989A")
    IPipeByte : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE Pull( 
            /* [length_is][size_is][out] */ __RPC__out_ecount_part(cRequest, *pcReturned) BYTE *buf,
            /* [in] */ ULONG cRequest,
            /* [out] */ __RPC__out ULONG *pcReturned) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Push( 
            /* [size_is][in] */ __RPC__in_ecount_full(cSent) BYTE *buf,
            /* [in] */ ULONG cSent) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IPipeByteVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IPipeByte * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IPipeByte * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IPipeByte * This);
        
        DECLSPEC_XFGVIRT(IPipeByte, Pull)
        HRESULT ( STDMETHODCALLTYPE *Pull )( 
            __RPC__in IPipeByte * This,
            /* [length_is][size_is][out] */ __RPC__out_ecount_part(cRequest, *pcReturned) BYTE *buf,
            /* [in] */ ULONG cRequest,
            /* [out] */ __RPC__out ULONG *pcReturned);
        
        DECLSPEC_XFGVIRT(IPipeByte, Push)
        HRESULT ( STDMETHODCALLTYPE *Push )( 
            __RPC__in IPipeByte * This,
            /* [size_is][in] */ __RPC__in_ecount_full(cSent) BYTE *buf,
            /* [in] */ ULONG cSent);
        
        END_INTERFACE
    } IPipeByteVtbl;

    interface IPipeByte
    {
        CONST_VTBL struct IPipeByteVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IPipeByte_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IPipeByte_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IPipeByte_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IPipeByte_Pull(This,buf,cRequest,pcReturned)	\
    ( (This)->lpVtbl -> Pull(This,buf,cRequest,pcReturned) ) 

#define IPipeByte_Push(This,buf,cSent)	\
    ( (This)->lpVtbl -> Push(This,buf,cSent) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IPipeByte_INTERFACE_DEFINED__ */


#ifndef __AsyncIPipeByte_INTERFACE_DEFINED__
#define __AsyncIPipeByte_INTERFACE_DEFINED__

/* interface AsyncIPipeByte */
/* [uuid][unique][object] */ 


EXTERN_C const IID IID_AsyncIPipeByte;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("DB2F3ACB-2F86-11d1-8E04-00C04FB9989A")
    AsyncIPipeByte : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE Begin_Pull( 
            /* [in] */ ULONG cRequest) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Finish_Pull( 
            /* [length_is][size_is][out] */ __RPC__out_xcount_part(cRequest, *pcReturned) BYTE *buf,
            /* [out] */ __RPC__out ULONG *pcReturned) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Begin_Push( 
            /* [size_is][in] */ __RPC__in_xcount_full(cSent) BYTE *buf,
            /* [in] */ ULONG cSent) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Finish_Push( void) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct AsyncIPipeByteVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in AsyncIPipeByte * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in AsyncIPipeByte * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in AsyncIPipeByte * This);
        
        DECLSPEC_XFGVIRT(AsyncIPipeByte, Begin_Pull)
        HRESULT ( STDMETHODCALLTYPE *Begin_Pull )( 
            __RPC__in AsyncIPipeByte * This,
            /* [in] */ ULONG cRequest);
        
        DECLSPEC_XFGVIRT(AsyncIPipeByte, Finish_Pull)
        HRESULT ( STDMETHODCALLTYPE *Finish_Pull )( 
            __RPC__in AsyncIPipeByte * This,
            /* [length_is][size_is][out] */ __RPC__out_xcount_part(cRequest, *pcReturned) BYTE *buf,
            /* [out] */ __RPC__out ULONG *pcReturned);
        
        DECLSPEC_XFGVIRT(AsyncIPipeByte, Begin_Push)
        HRESULT ( STDMETHODCALLTYPE *Begin_Push )( 
            __RPC__in AsyncIPipeByte * This,
            /* [size_is][in] */ __RPC__in_xcount_full(cSent) BYTE *buf,
            /* [in] */ ULONG cSent);
        
        DECLSPEC_XFGVIRT(AsyncIPipeByte, Finish_Push)
        HRESULT ( STDMETHODCALLTYPE *Finish_Push )( 
            __RPC__in AsyncIPipeByte * This);
        
        END_INTERFACE
    } AsyncIPipeByteVtbl;

    interface AsyncIPipeByte
    {
        CONST_VTBL struct AsyncIPipeByteVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define AsyncIPipeByte_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define AsyncIPipeByte_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define AsyncIPipeByte_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define AsyncIPipeByte_Begin_Pull(This,cRequest)	\
    ( (This)->lpVtbl -> Begin_Pull(This,cRequest) ) 

#define AsyncIPipeByte_Finish_Pull(This,buf,pcReturned)	\
    ( (This)->lpVtbl -> Finish_Pull(This,buf,pcReturned) ) 

#define AsyncIPipeByte_Begin_Push(This,buf,cSent)	\
    ( (This)->lpVtbl -> Begin_Push(This,buf,cSent) ) 

#define AsyncIPipeByte_Finish_Push(This)	\
    ( (This)->lpVtbl -> Finish_Push(This) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __AsyncIPipeByte_INTERFACE_DEFINED__ */


#ifndef __IPipeLong_INTERFACE_DEFINED__
#define __IPipeLong_INTERFACE_DEFINED__

/* interface IPipeLong */
/* [unique][async_uuid][uuid][object] */ 


EXTERN_C const IID IID_IPipeLong;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("DB2F3ACC-2F86-11d1-8E04-00C04FB9989A")
    IPipeLong : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE Pull( 
            /* [length_is][size_is][out] */ __RPC__out_ecount_part(cRequest, *pcReturned) LONG *buf,
            /* [in] */ ULONG cRequest,
            /* [out] */ __RPC__out ULONG *pcReturned) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Push( 
            /* [size_is][in] */ __RPC__in_ecount_full(cSent) LONG *buf,
            /* [in] */ ULONG cSent) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IPipeLongVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IPipeLong * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IPipeLong * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IPipeLong * This);
        
        DECLSPEC_XFGVIRT(IPipeLong, Pull)
        HRESULT ( STDMETHODCALLTYPE *Pull )( 
            __RPC__in IPipeLong * This,
            /* [length_is][size_is][out] */ __RPC__out_ecount_part(cRequest, *pcReturned) LONG *buf,
            /* [in] */ ULONG cRequest,
            /* [out] */ __RPC__out ULONG *pcReturned);
        
        DECLSPEC_XFGVIRT(IPipeLong, Push)
        HRESULT ( STDMETHODCALLTYPE *Push )( 
            __RPC__in IPipeLong * This,
            /* [size_is][in] */ __RPC__in_ecount_full(cSent) LONG *buf,
            /* [in] */ ULONG cSent);
        
        END_INTERFACE
    } IPipeLongVtbl;

    interface IPipeLong
    {
        CONST_VTBL struct IPipeLongVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IPipeLong_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IPipeLong_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IPipeLong_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IPipeLong_Pull(This,buf,cRequest,pcReturned)	\
    ( (This)->lpVtbl -> Pull(This,buf,cRequest,pcReturned) ) 

#define IPipeLong_Push(This,buf,cSent)	\
    ( (This)->lpVtbl -> Push(This,buf,cSent) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IPipeLong_INTERFACE_DEFINED__ */


#ifndef __AsyncIPipeLong_INTERFACE_DEFINED__
#define __AsyncIPipeLong_INTERFACE_DEFINED__

/* interface AsyncIPipeLong */
/* [uuid][unique][object] */ 


EXTERN_C const IID IID_AsyncIPipeLong;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("DB2F3ACD-2F86-11d1-8E04-00C04FB9989A")
    AsyncIPipeLong : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE Begin_Pull( 
            /* [in] */ ULONG cRequest) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Finish_Pull( 
            /* [length_is][size_is][out] */ __RPC__out_xcount_part(cRequest, *pcReturned) LONG *buf,
            /* [out] */ __RPC__out ULONG *pcReturned) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Begin_Push( 
            /* [size_is][in] */ __RPC__in_xcount_full(cSent) LONG *buf,
            /* [in] */ ULONG cSent) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Finish_Push( void) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct AsyncIPipeLongVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in AsyncIPipeLong * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in AsyncIPipeLong * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in AsyncIPipeLong * This);
        
        DECLSPEC_XFGVIRT(AsyncIPipeLong, Begin_Pull)
        HRESULT ( STDMETHODCALLTYPE *Begin_Pull )( 
            __RPC__in AsyncIPipeLong * This,
            /* [in] */ ULONG cRequest);
        
        DECLSPEC_XFGVIRT(AsyncIPipeLong, Finish_Pull)
        HRESULT ( STDMETHODCALLTYPE *Finish_Pull )( 
            __RPC__in AsyncIPipeLong * This,
            /* [length_is][size_is][out] */ __RPC__out_xcount_part(cRequest, *pcReturned) LONG *buf,
            /* [out] */ __RPC__out ULONG *pcReturned);
        
        DECLSPEC_XFGVIRT(AsyncIPipeLong, Begin_Push)
        HRESULT ( STDMETHODCALLTYPE *Begin_Push )( 
            __RPC__in AsyncIPipeLong * This,
            /* [size_is][in] */ __RPC__in_xcount_full(cSent) LONG *buf,
            /* [in] */ ULONG cSent);
        
        DECLSPEC_XFGVIRT(AsyncIPipeLong, Finish_Push)
        HRESULT ( STDMETHODCALLTYPE *Finish_Push )( 
            __RPC__in AsyncIPipeLong * This);
        
        END_INTERFACE
    } AsyncIPipeLongVtbl;

    interface AsyncIPipeLong
    {
        CONST_VTBL struct AsyncIPipeLongVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define AsyncIPipeLong_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define AsyncIPipeLong_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define AsyncIPipeLong_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define AsyncIPipeLong_Begin_Pull(This,cRequest)	\
    ( (This)->lpVtbl -> Begin_Pull(This,cRequest) ) 

#define AsyncIPipeLong_Finish_Pull(This,buf,pcReturned)	\
    ( (This)->lpVtbl -> Finish_Pull(This,buf,pcReturned) ) 

#define AsyncIPipeLong_Begin_Push(This,buf,cSent)	\
    ( (This)->lpVtbl -> Begin_Push(This,buf,cSent) ) 

#define AsyncIPipeLong_Finish_Push(This)	\
    ( (This)->lpVtbl -> Finish_Push(This) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __AsyncIPipeLong_INTERFACE_DEFINED__ */


#ifndef __IPipeDouble_INTERFACE_DEFINED__
#define __IPipeDouble_INTERFACE_DEFINED__

/* interface IPipeDouble */
/* [unique][async_uuid][uuid][object] */ 


EXTERN_C const IID IID_IPipeDouble;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("DB2F3ACE-2F86-11d1-8E04-00C04FB9989A")
    IPipeDouble : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE Pull( 
            /* [length_is][size_is][out] */ __RPC__out_ecount_part(cRequest, *pcReturned) DOUBLE *buf,
            /* [in] */ ULONG cRequest,
            /* [out] */ __RPC__out ULONG *pcReturned) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Push( 
            /* [size_is][in] */ __RPC__in_ecount_full(cSent) DOUBLE *buf,
            /* [in] */ ULONG cSent) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IPipeDoubleVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IPipeDouble * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IPipeDouble * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IPipeDouble * This);
        
        DECLSPEC_XFGVIRT(IPipeDouble, Pull)
        HRESULT ( STDMETHODCALLTYPE *Pull )( 
            __RPC__in IPipeDouble * This,
            /* [length_is][size_is][out] */ __RPC__out_ecount_part(cRequest, *pcReturned) DOUBLE *buf,
            /* [in] */ ULONG cRequest,
            /* [out] */ __RPC__out ULONG *pcReturned);
        
        DECLSPEC_XFGVIRT(IPipeDouble, Push)
        HRESULT ( STDMETHODCALLTYPE *Push )( 
            __RPC__in IPipeDouble * This,
            /* [size_is][in] */ __RPC__in_ecount_full(cSent) DOUBLE *buf,
            /* [in] */ ULONG cSent);
        
        END_INTERFACE
    } IPipeDoubleVtbl;

    interface IPipeDouble
    {
        CONST_VTBL struct IPipeDoubleVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IPipeDouble_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IPipeDouble_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IPipeDouble_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IPipeDouble_Pull(This,buf,cRequest,pcReturned)	\
    ( (This)->lpVtbl -> Pull(This,buf,cRequest,pcReturned) ) 

#define IPipeDouble_Push(This,buf,cSent)	\
    ( (This)->lpVtbl -> Push(This,buf,cSent) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IPipeDouble_INTERFACE_DEFINED__ */


#ifndef __AsyncIPipeDouble_INTERFACE_DEFINED__
#define __AsyncIPipeDouble_INTERFACE_DEFINED__

/* interface AsyncIPipeDouble */
/* [uuid][unique][object] */ 


EXTERN_C const IID IID_AsyncIPipeDouble;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("DB2F3ACF-2F86-11d1-8E04-00C04FB9989A")
    AsyncIPipeDouble : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE Begin_Pull( 
            /* [in] */ ULONG cRequest) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Finish_Pull( 
            /* [length_is][size_is][out] */ __RPC__out_xcount_part(cRequest, *pcReturned) DOUBLE *buf,
            /* [out] */ __RPC__out ULONG *pcReturned) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Begin_Push( 
            /* [size_is][in] */ __RPC__in_xcount_full(cSent) DOUBLE *buf,
            /* [in] */ ULONG cSent) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Finish_Push( void) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct AsyncIPipeDoubleVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in AsyncIPipeDouble * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in AsyncIPipeDouble * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in AsyncIPipeDouble * This);
        
        DECLSPEC_XFGVIRT(AsyncIPipeDouble, Begin_Pull)
        HRESULT ( STDMETHODCALLTYPE *Begin_Pull )( 
            __RPC__in AsyncIPipeDouble * This,
            /* [in] */ ULONG cRequest);
        
        DECLSPEC_XFGVIRT(AsyncIPipeDouble, Finish_Pull)
        HRESULT ( STDMETHODCALLTYPE *Finish_Pull )( 
            __RPC__in AsyncIPipeDouble * This,
            /* [length_is][size_is][out] */ __RPC__out_xcount_part(cRequest, *pcReturned) DOUBLE *buf,
            /* [out] */ __RPC__out ULONG *pcReturned);
        
        DECLSPEC_XFGVIRT(AsyncIPipeDouble, Begin_Push)
        HRESULT ( STDMETHODCALLTYPE *Begin_Push )( 
            __RPC__in AsyncIPipeDouble * This,
            /* [size_is][in] */ __RPC__in_xcount_full(cSent) DOUBLE *buf,
            /* [in] */ ULONG cSent);
        
        DECLSPEC_XFGVIRT(AsyncIPipeDouble, Finish_Push)
        HRESULT ( STDMETHODCALLTYPE *Finish_Push )( 
            __RPC__in AsyncIPipeDouble * This);
        
        END_INTERFACE
    } AsyncIPipeDoubleVtbl;

    interface AsyncIPipeDouble
    {
        CONST_VTBL struct AsyncIPipeDoubleVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define AsyncIPipeDouble_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define AsyncIPipeDouble_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define AsyncIPipeDouble_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define AsyncIPipeDouble_Begin_Pull(This,cRequest)	\
    ( (This)->lpVtbl -> Begin_Pull(This,cRequest) ) 

#define AsyncIPipeDouble_Finish_Pull(This,buf,pcReturned)	\
    ( (This)->lpVtbl -> Finish_Pull(This,buf,pcReturned) ) 

#define AsyncIPipeDouble_Begin_Push(This,buf,cSent)	\
    ( (This)->lpVtbl -> Begin_Push(This,buf,cSent) ) 

#define AsyncIPipeDouble_Finish_Push(This)	\
    ( (This)->lpVtbl -> Finish_Push(This) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __AsyncIPipeDouble_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_objidl_0000_0045 */
/* [local] */ 

#if defined USE_COM_CONTEXT_DEF || defined BUILDTYPE_COMSVCS || defined _COMBASEAPI_ || defined _OLE32_
typedef DWORD CPFLAGS;

typedef struct tagContextProperty
    {
    GUID policyId;
    CPFLAGS flags;
    /* [unique] */ IUnknown *pUnk;
    } 	ContextProperty;



extern RPC_IF_HANDLE __MIDL_itf_objidl_0000_0045_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_objidl_0000_0045_v0_0_s_ifspec;

#ifndef __IEnumContextProps_INTERFACE_DEFINED__
#define __IEnumContextProps_INTERFACE_DEFINED__

/* interface IEnumContextProps */
/* [unique][uuid][object][local] */ 

typedef /* [unique] */ IEnumContextProps *LPENUMCONTEXTPROPS;


EXTERN_C const IID IID_IEnumContextProps;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("000001c1-0000-0000-C000-000000000046")
    IEnumContextProps : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE Next( 
            /* [annotation][in] */ 
            _In_  ULONG celt,
            /* [annotation][length_is][size_is][out] */ 
            _Out_writes_to_(celt, *pceltFetched)  ContextProperty *pContextProperties,
            /* [annotation][out] */ 
            _Out_  ULONG *pceltFetched) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Skip( 
            /* [annotation][in] */ 
            _In_  ULONG celt) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Reset( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Clone( 
            /* [annotation][out] */ 
            _Outptr_  IEnumContextProps **ppEnumContextProps) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Count( 
            /* [annotation][out] */ 
            _Out_  ULONG *pcelt) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IEnumContextPropsVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IEnumContextProps * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IEnumContextProps * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IEnumContextProps * This);
        
        DECLSPEC_XFGVIRT(IEnumContextProps, Next)
        HRESULT ( STDMETHODCALLTYPE *Next )( 
            IEnumContextProps * This,
            /* [annotation][in] */ 
            _In_  ULONG celt,
            /* [annotation][length_is][size_is][out] */ 
            _Out_writes_to_(celt, *pceltFetched)  ContextProperty *pContextProperties,
            /* [annotation][out] */ 
            _Out_  ULONG *pceltFetched);
        
        DECLSPEC_XFGVIRT(IEnumContextProps, Skip)
        HRESULT ( STDMETHODCALLTYPE *Skip )( 
            IEnumContextProps * This,
            /* [annotation][in] */ 
            _In_  ULONG celt);
        
        DECLSPEC_XFGVIRT(IEnumContextProps, Reset)
        HRESULT ( STDMETHODCALLTYPE *Reset )( 
            IEnumContextProps * This);
        
        DECLSPEC_XFGVIRT(IEnumContextProps, Clone)
        HRESULT ( STDMETHODCALLTYPE *Clone )( 
            IEnumContextProps * This,
            /* [annotation][out] */ 
            _Outptr_  IEnumContextProps **ppEnumContextProps);
        
        DECLSPEC_XFGVIRT(IEnumContextProps, Count)
        HRESULT ( STDMETHODCALLTYPE *Count )( 
            IEnumContextProps * This,
            /* [annotation][out] */ 
            _Out_  ULONG *pcelt);
        
        END_INTERFACE
    } IEnumContextPropsVtbl;

    interface IEnumContextProps
    {
        CONST_VTBL struct IEnumContextPropsVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IEnumContextProps_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IEnumContextProps_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IEnumContextProps_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IEnumContextProps_Next(This,celt,pContextProperties,pceltFetched)	\
    ( (This)->lpVtbl -> Next(This,celt,pContextProperties,pceltFetched) ) 

#define IEnumContextProps_Skip(This,celt)	\
    ( (This)->lpVtbl -> Skip(This,celt) ) 

#define IEnumContextProps_Reset(This)	\
    ( (This)->lpVtbl -> Reset(This) ) 

#define IEnumContextProps_Clone(This,ppEnumContextProps)	\
    ( (This)->lpVtbl -> Clone(This,ppEnumContextProps) ) 

#define IEnumContextProps_Count(This,pcelt)	\
    ( (This)->lpVtbl -> Count(This,pcelt) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IEnumContextProps_INTERFACE_DEFINED__ */


#ifndef __IContext_INTERFACE_DEFINED__
#define __IContext_INTERFACE_DEFINED__

/* interface IContext */
/* [unique][uuid][object][local] */ 


EXTERN_C const IID IID_IContext;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("000001c0-0000-0000-C000-000000000046")
    IContext : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE SetProperty( 
            /* [annotation][in] */ 
            _In_  REFGUID rpolicyId,
            /* [annotation][in] */ 
            _In_  CPFLAGS flags,
            /* [annotation][in] */ 
            _In_  IUnknown *pUnk) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE RemoveProperty( 
            /* [annotation][in] */ 
            _In_  REFGUID rPolicyId) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetProperty( 
            /* [annotation][in] */ 
            _In_  REFGUID rGuid,
            /* [annotation][out] */ 
            _Out_  CPFLAGS *pFlags,
            /* [annotation][out] */ 
            _Outptr_  IUnknown **ppUnk) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE EnumContextProps( 
            /* [annotation][out] */ 
            _Outptr_  IEnumContextProps **ppEnumContextProps) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IContextVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IContext * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IContext * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IContext * This);
        
        DECLSPEC_XFGVIRT(IContext, SetProperty)
        HRESULT ( STDMETHODCALLTYPE *SetProperty )( 
            IContext * This,
            /* [annotation][in] */ 
            _In_  REFGUID rpolicyId,
            /* [annotation][in] */ 
            _In_  CPFLAGS flags,
            /* [annotation][in] */ 
            _In_  IUnknown *pUnk);
        
        DECLSPEC_XFGVIRT(IContext, RemoveProperty)
        HRESULT ( STDMETHODCALLTYPE *RemoveProperty )( 
            IContext * This,
            /* [annotation][in] */ 
            _In_  REFGUID rPolicyId);
        
        DECLSPEC_XFGVIRT(IContext, GetProperty)
        HRESULT ( STDMETHODCALLTYPE *GetProperty )( 
            IContext * This,
            /* [annotation][in] */ 
            _In_  REFGUID rGuid,
            /* [annotation][out] */ 
            _Out_  CPFLAGS *pFlags,
            /* [annotation][out] */ 
            _Outptr_  IUnknown **ppUnk);
        
        DECLSPEC_XFGVIRT(IContext, EnumContextProps)
        HRESULT ( STDMETHODCALLTYPE *EnumContextProps )( 
            IContext * This,
            /* [annotation][out] */ 
            _Outptr_  IEnumContextProps **ppEnumContextProps);
        
        END_INTERFACE
    } IContextVtbl;

    interface IContext
    {
        CONST_VTBL struct IContextVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IContext_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IContext_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IContext_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IContext_SetProperty(This,rpolicyId,flags,pUnk)	\
    ( (This)->lpVtbl -> SetProperty(This,rpolicyId,flags,pUnk) ) 

#define IContext_RemoveProperty(This,rPolicyId)	\
    ( (This)->lpVtbl -> RemoveProperty(This,rPolicyId) ) 

#define IContext_GetProperty(This,rGuid,pFlags,ppUnk)	\
    ( (This)->lpVtbl -> GetProperty(This,rGuid,pFlags,ppUnk) ) 

#define IContext_EnumContextProps(This,ppEnumContextProps)	\
    ( (This)->lpVtbl -> EnumContextProps(This,ppEnumContextProps) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IContext_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_objidl_0000_0047 */
/* [local] */ 

#if !defined BUILDTYPE_COMSVCS && ! (defined _COMBASEAPI_ || defined _OLE32_)


extern RPC_IF_HANDLE __MIDL_itf_objidl_0000_0047_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_objidl_0000_0047_v0_0_s_ifspec;

#ifndef __IObjContext_INTERFACE_DEFINED__
#define __IObjContext_INTERFACE_DEFINED__

/* interface IObjContext */
/* [unique][uuid][object][local] */ 


EXTERN_C const IID IID_IObjContext;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("000001c6-0000-0000-C000-000000000046")
    IObjContext : public IContext
    {
    public:
        virtual void STDMETHODCALLTYPE Reserved1( void) = 0;
        
        virtual void STDMETHODCALLTYPE Reserved2( void) = 0;
        
        virtual void STDMETHODCALLTYPE Reserved3( void) = 0;
        
        virtual void STDMETHODCALLTYPE Reserved4( void) = 0;
        
        virtual void STDMETHODCALLTYPE Reserved5( void) = 0;
        
        virtual void STDMETHODCALLTYPE Reserved6( void) = 0;
        
        virtual void STDMETHODCALLTYPE Reserved7( void) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IObjContextVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IObjContext * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IObjContext * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IObjContext * This);
        
        DECLSPEC_XFGVIRT(IContext, SetProperty)
        HRESULT ( STDMETHODCALLTYPE *SetProperty )( 
            IObjContext * This,
            /* [annotation][in] */ 
            _In_  REFGUID rpolicyId,
            /* [annotation][in] */ 
            _In_  CPFLAGS flags,
            /* [annotation][in] */ 
            _In_  IUnknown *pUnk);
        
        DECLSPEC_XFGVIRT(IContext, RemoveProperty)
        HRESULT ( STDMETHODCALLTYPE *RemoveProperty )( 
            IObjContext * This,
            /* [annotation][in] */ 
            _In_  REFGUID rPolicyId);
        
        DECLSPEC_XFGVIRT(IContext, GetProperty)
        HRESULT ( STDMETHODCALLTYPE *GetProperty )( 
            IObjContext * This,
            /* [annotation][in] */ 
            _In_  REFGUID rGuid,
            /* [annotation][out] */ 
            _Out_  CPFLAGS *pFlags,
            /* [annotation][out] */ 
            _Outptr_  IUnknown **ppUnk);
        
        DECLSPEC_XFGVIRT(IContext, EnumContextProps)
        HRESULT ( STDMETHODCALLTYPE *EnumContextProps )( 
            IObjContext * This,
            /* [annotation][out] */ 
            _Outptr_  IEnumContextProps **ppEnumContextProps);
        
        DECLSPEC_XFGVIRT(IObjContext, Reserved1)
        void ( STDMETHODCALLTYPE *Reserved1 )( 
            IObjContext * This);
        
        DECLSPEC_XFGVIRT(IObjContext, Reserved2)
        void ( STDMETHODCALLTYPE *Reserved2 )( 
            IObjContext * This);
        
        DECLSPEC_XFGVIRT(IObjContext, Reserved3)
        void ( STDMETHODCALLTYPE *Reserved3 )( 
            IObjContext * This);
        
        DECLSPEC_XFGVIRT(IObjContext, Reserved4)
        void ( STDMETHODCALLTYPE *Reserved4 )( 
            IObjContext * This);
        
        DECLSPEC_XFGVIRT(IObjContext, Reserved5)
        void ( STDMETHODCALLTYPE *Reserved5 )( 
            IObjContext * This);
        
        DECLSPEC_XFGVIRT(IObjContext, Reserved6)
        void ( STDMETHODCALLTYPE *Reserved6 )( 
            IObjContext * This);
        
        DECLSPEC_XFGVIRT(IObjContext, Reserved7)
        void ( STDMETHODCALLTYPE *Reserved7 )( 
            IObjContext * This);
        
        END_INTERFACE
    } IObjContextVtbl;

    interface IObjContext
    {
        CONST_VTBL struct IObjContextVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IObjContext_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IObjContext_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IObjContext_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IObjContext_SetProperty(This,rpolicyId,flags,pUnk)	\
    ( (This)->lpVtbl -> SetProperty(This,rpolicyId,flags,pUnk) ) 

#define IObjContext_RemoveProperty(This,rPolicyId)	\
    ( (This)->lpVtbl -> RemoveProperty(This,rPolicyId) ) 

#define IObjContext_GetProperty(This,rGuid,pFlags,ppUnk)	\
    ( (This)->lpVtbl -> GetProperty(This,rGuid,pFlags,ppUnk) ) 

#define IObjContext_EnumContextProps(This,ppEnumContextProps)	\
    ( (This)->lpVtbl -> EnumContextProps(This,ppEnumContextProps) ) 


#define IObjContext_Reserved1(This)	\
    ( (This)->lpVtbl -> Reserved1(This) ) 

#define IObjContext_Reserved2(This)	\
    ( (This)->lpVtbl -> Reserved2(This) ) 

#define IObjContext_Reserved3(This)	\
    ( (This)->lpVtbl -> Reserved3(This) ) 

#define IObjContext_Reserved4(This)	\
    ( (This)->lpVtbl -> Reserved4(This) ) 

#define IObjContext_Reserved5(This)	\
    ( (This)->lpVtbl -> Reserved5(This) ) 

#define IObjContext_Reserved6(This)	\
    ( (This)->lpVtbl -> Reserved6(This) ) 

#define IObjContext_Reserved7(This)	\
    ( (This)->lpVtbl -> Reserved7(This) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IObjContext_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_objidl_0000_0048 */
/* [local] */ 

#endif
#endif
#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES) */
#pragma endregion
#pragma region Application Family or OneCore Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES)
typedef 
enum _APTTYPEQUALIFIER
    {
        APTTYPEQUALIFIER_NONE	= 0,
        APTTYPEQUALIFIER_IMPLICIT_MTA	= 1,
        APTTYPEQUALIFIER_NA_ON_MTA	= 2,
        APTTYPEQUALIFIER_NA_ON_STA	= 3,
        APTTYPEQUALIFIER_NA_ON_IMPLICIT_MTA	= 4,
        APTTYPEQUALIFIER_NA_ON_MAINSTA	= 5,
        APTTYPEQUALIFIER_APPLICATION_STA	= 6,
        APTTYPEQUALIFIER_RESERVED_1	= 7
    } 	APTTYPEQUALIFIER;

typedef 
enum _APTTYPE
    {
        APTTYPE_CURRENT	= -1,
        APTTYPE_STA	= 0,
        APTTYPE_MTA	= 1,
        APTTYPE_NA	= 2,
        APTTYPE_MAINSTA	= 3
    } 	APTTYPE;

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES) */
#pragma endregion
#pragma region Desktop Family or OneCore Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES)
typedef 
enum _THDTYPE
    {
        THDTYPE_BLOCKMESSAGES	= 0,
        THDTYPE_PROCESSMESSAGES	= 1
    } 	THDTYPE;

typedef DWORD APARTMENTID;



extern RPC_IF_HANDLE __MIDL_itf_objidl_0000_0048_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_objidl_0000_0048_v0_0_s_ifspec;

#ifndef __IComThreadingInfo_INTERFACE_DEFINED__
#define __IComThreadingInfo_INTERFACE_DEFINED__

/* interface IComThreadingInfo */
/* [unique][uuid][object][local] */ 


EXTERN_C const IID IID_IComThreadingInfo;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("000001ce-0000-0000-C000-000000000046")
    IComThreadingInfo : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetCurrentApartmentType( 
            /* [annotation][out] */ 
            _Out_  APTTYPE *pAptType) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetCurrentThreadType( 
            /* [annotation][out] */ 
            _Out_  THDTYPE *pThreadType) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetCurrentLogicalThreadId( 
            /* [annotation][out] */ 
            _Out_  GUID *pguidLogicalThreadId) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetCurrentLogicalThreadId( 
            /* [annotation][in] */ 
            _In_  REFGUID rguid) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IComThreadingInfoVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IComThreadingInfo * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IComThreadingInfo * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IComThreadingInfo * This);
        
        DECLSPEC_XFGVIRT(IComThreadingInfo, GetCurrentApartmentType)
        HRESULT ( STDMETHODCALLTYPE *GetCurrentApartmentType )( 
            IComThreadingInfo * This,
            /* [annotation][out] */ 
            _Out_  APTTYPE *pAptType);
        
        DECLSPEC_XFGVIRT(IComThreadingInfo, GetCurrentThreadType)
        HRESULT ( STDMETHODCALLTYPE *GetCurrentThreadType )( 
            IComThreadingInfo * This,
            /* [annotation][out] */ 
            _Out_  THDTYPE *pThreadType);
        
        DECLSPEC_XFGVIRT(IComThreadingInfo, GetCurrentLogicalThreadId)
        HRESULT ( STDMETHODCALLTYPE *GetCurrentLogicalThreadId )( 
            IComThreadingInfo * This,
            /* [annotation][out] */ 
            _Out_  GUID *pguidLogicalThreadId);
        
        DECLSPEC_XFGVIRT(IComThreadingInfo, SetCurrentLogicalThreadId)
        HRESULT ( STDMETHODCALLTYPE *SetCurrentLogicalThreadId )( 
            IComThreadingInfo * This,
            /* [annotation][in] */ 
            _In_  REFGUID rguid);
        
        END_INTERFACE
    } IComThreadingInfoVtbl;

    interface IComThreadingInfo
    {
        CONST_VTBL struct IComThreadingInfoVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IComThreadingInfo_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IComThreadingInfo_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IComThreadingInfo_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IComThreadingInfo_GetCurrentApartmentType(This,pAptType)	\
    ( (This)->lpVtbl -> GetCurrentApartmentType(This,pAptType) ) 

#define IComThreadingInfo_GetCurrentThreadType(This,pThreadType)	\
    ( (This)->lpVtbl -> GetCurrentThreadType(This,pThreadType) ) 

#define IComThreadingInfo_GetCurrentLogicalThreadId(This,pguidLogicalThreadId)	\
    ( (This)->lpVtbl -> GetCurrentLogicalThreadId(This,pguidLogicalThreadId) ) 

#define IComThreadingInfo_SetCurrentLogicalThreadId(This,rguid)	\
    ( (This)->lpVtbl -> SetCurrentLogicalThreadId(This,rguid) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IComThreadingInfo_INTERFACE_DEFINED__ */


#ifndef __IProcessInitControl_INTERFACE_DEFINED__
#define __IProcessInitControl_INTERFACE_DEFINED__

/* interface IProcessInitControl */
/* [uuid][unique][object] */ 


EXTERN_C const IID IID_IProcessInitControl;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("72380d55-8d2b-43a3-8513-2b6ef31434e9")
    IProcessInitControl : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE ResetInitializerTimeout( 
            /* [in] */ DWORD dwSecondsRemaining) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IProcessInitControlVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IProcessInitControl * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IProcessInitControl * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IProcessInitControl * This);
        
        DECLSPEC_XFGVIRT(IProcessInitControl, ResetInitializerTimeout)
        HRESULT ( STDMETHODCALLTYPE *ResetInitializerTimeout )( 
            __RPC__in IProcessInitControl * This,
            /* [in] */ DWORD dwSecondsRemaining);
        
        END_INTERFACE
    } IProcessInitControlVtbl;

    interface IProcessInitControl
    {
        CONST_VTBL struct IProcessInitControlVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IProcessInitControl_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IProcessInitControl_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IProcessInitControl_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IProcessInitControl_ResetInitializerTimeout(This,dwSecondsRemaining)	\
    ( (This)->lpVtbl -> ResetInitializerTimeout(This,dwSecondsRemaining) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IProcessInitControl_INTERFACE_DEFINED__ */


#ifndef __IFastRundown_INTERFACE_DEFINED__
#define __IFastRundown_INTERFACE_DEFINED__

/* interface IFastRundown */
/* [uuid][unique][local][object] */ 


EXTERN_C const IID IID_IFastRundown;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("00000040-0000-0000-C000-000000000046")
    IFastRundown : public IUnknown
    {
    public:
    };
    
    
#else 	/* C style interface */

    typedef struct IFastRundownVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IFastRundown * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IFastRundown * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IFastRundown * This);
        
        END_INTERFACE
    } IFastRundownVtbl;

    interface IFastRundown
    {
        CONST_VTBL struct IFastRundownVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IFastRundown_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IFastRundown_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IFastRundown_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IFastRundown_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_objidl_0000_0051 */
/* [local] */ 

typedef 
enum CO_MARSHALING_CONTEXT_ATTRIBUTES
    {
        CO_MARSHALING_SOURCE_IS_APP_CONTAINER	= 0,
        CO_MARSHALING_CONTEXT_ATTRIBUTE_RESERVED_1	= 0x80000000,
        CO_MARSHALING_CONTEXT_ATTRIBUTE_RESERVED_2	= 0x80000001,
        CO_MARSHALING_CONTEXT_ATTRIBUTE_RESERVED_3	= 0x80000002,
        CO_MARSHALING_CONTEXT_ATTRIBUTE_RESERVED_4	= 0x80000003,
        CO_MARSHALING_CONTEXT_ATTRIBUTE_RESERVED_5	= 0x80000004,
        CO_MARSHALING_CONTEXT_ATTRIBUTE_RESERVED_6	= 0x80000005,
        CO_MARSHALING_CONTEXT_ATTRIBUTE_RESERVED_7	= 0x80000006,
        CO_MARSHALING_CONTEXT_ATTRIBUTE_RESERVED_8	= 0x80000007,
        CO_MARSHALING_CONTEXT_ATTRIBUTE_RESERVED_9	= 0x80000008,
        CO_MARSHALING_CONTEXT_ATTRIBUTE_RESERVED_10	= 0x80000009,
        CO_MARSHALING_CONTEXT_ATTRIBUTE_RESERVED_11	= 0x8000000a,
        CO_MARSHALING_CONTEXT_ATTRIBUTE_RESERVED_12	= 0x8000000b,
        CO_MARSHALING_CONTEXT_ATTRIBUTE_RESERVED_13	= 0x8000000c,
        CO_MARSHALING_CONTEXT_ATTRIBUTE_RESERVED_14	= 0x8000000d,
        CO_MARSHALING_CONTEXT_ATTRIBUTE_RESERVED_15	= 0x8000000e,
        CO_MARSHALING_CONTEXT_ATTRIBUTE_RESERVED_16	= 0x8000000f,
        CO_MARSHALING_CONTEXT_ATTRIBUTE_RESERVED_17	= 0x80000010,
        CO_MARSHALING_CONTEXT_ATTRIBUTE_RESERVED_18	= 0x80000011
    } 	CO_MARSHALING_CONTEXT_ATTRIBUTES;



extern RPC_IF_HANDLE __MIDL_itf_objidl_0000_0051_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_objidl_0000_0051_v0_0_s_ifspec;

#ifndef __IMarshalingStream_INTERFACE_DEFINED__
#define __IMarshalingStream_INTERFACE_DEFINED__

/* interface IMarshalingStream */
/* [unique][uuid][object][local] */ 


EXTERN_C const IID IID_IMarshalingStream;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("D8F2F5E6-6102-4863-9F26-389A4676EFDE")
    IMarshalingStream : public IStream
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetMarshalingContextAttribute( 
            /* [in] */ CO_MARSHALING_CONTEXT_ATTRIBUTES attribute,
            /* [out] */ ULONG_PTR *pAttributeValue) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IMarshalingStreamVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IMarshalingStream * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IMarshalingStream * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IMarshalingStream * This);
        
        DECLSPEC_XFGVIRT(ISequentialStream, Read)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Read )( 
            IMarshalingStream * This,
            /* [annotation] */ 
            _Out_writes_bytes_to_(cb, *pcbRead)  void *pv,
            /* [annotation][in] */ 
            _In_  ULONG cb,
            /* [annotation] */ 
            _Out_opt_  ULONG *pcbRead);
        
        DECLSPEC_XFGVIRT(ISequentialStream, Write)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Write )( 
            IMarshalingStream * This,
            /* [annotation] */ 
            _In_reads_bytes_(cb)  const void *pv,
            /* [annotation][in] */ 
            _In_  ULONG cb,
            /* [annotation] */ 
            _Out_opt_  ULONG *pcbWritten);
        
        DECLSPEC_XFGVIRT(IStream, Seek)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Seek )( 
            IMarshalingStream * This,
            /* [in] */ LARGE_INTEGER dlibMove,
            /* [in] */ DWORD dwOrigin,
            /* [annotation] */ 
            _Out_opt_  ULARGE_INTEGER *plibNewPosition);
        
        DECLSPEC_XFGVIRT(IStream, SetSize)
        HRESULT ( STDMETHODCALLTYPE *SetSize )( 
            IMarshalingStream * This,
            /* [in] */ ULARGE_INTEGER libNewSize);
        
        DECLSPEC_XFGVIRT(IStream, CopyTo)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *CopyTo )( 
            IMarshalingStream * This,
            /* [annotation][unique][in] */ 
            _In_  IStream *pstm,
            /* [in] */ ULARGE_INTEGER cb,
            /* [annotation] */ 
            _Out_opt_  ULARGE_INTEGER *pcbRead,
            /* [annotation] */ 
            _Out_opt_  ULARGE_INTEGER *pcbWritten);
        
        DECLSPEC_XFGVIRT(IStream, Commit)
        HRESULT ( STDMETHODCALLTYPE *Commit )( 
            IMarshalingStream * This,
            /* [in] */ DWORD grfCommitFlags);
        
        DECLSPEC_XFGVIRT(IStream, Revert)
        HRESULT ( STDMETHODCALLTYPE *Revert )( 
            IMarshalingStream * This);
        
        DECLSPEC_XFGVIRT(IStream, LockRegion)
        HRESULT ( STDMETHODCALLTYPE *LockRegion )( 
            IMarshalingStream * This,
            /* [in] */ ULARGE_INTEGER libOffset,
            /* [in] */ ULARGE_INTEGER cb,
            /* [in] */ DWORD dwLockType);
        
        DECLSPEC_XFGVIRT(IStream, UnlockRegion)
        HRESULT ( STDMETHODCALLTYPE *UnlockRegion )( 
            IMarshalingStream * This,
            /* [in] */ ULARGE_INTEGER libOffset,
            /* [in] */ ULARGE_INTEGER cb,
            /* [in] */ DWORD dwLockType);
        
        DECLSPEC_XFGVIRT(IStream, Stat)
        HRESULT ( STDMETHODCALLTYPE *Stat )( 
            IMarshalingStream * This,
            /* [out] */ STATSTG *pstatstg,
            /* [in] */ DWORD grfStatFlag);
        
        DECLSPEC_XFGVIRT(IStream, Clone)
        HRESULT ( STDMETHODCALLTYPE *Clone )( 
            IMarshalingStream * This,
            /* [out] */ IStream **ppstm);
        
        DECLSPEC_XFGVIRT(IMarshalingStream, GetMarshalingContextAttribute)
        HRESULT ( STDMETHODCALLTYPE *GetMarshalingContextAttribute )( 
            IMarshalingStream * This,
            /* [in] */ CO_MARSHALING_CONTEXT_ATTRIBUTES attribute,
            /* [out] */ ULONG_PTR *pAttributeValue);
        
        END_INTERFACE
    } IMarshalingStreamVtbl;

    interface IMarshalingStream
    {
        CONST_VTBL struct IMarshalingStreamVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IMarshalingStream_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IMarshalingStream_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IMarshalingStream_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IMarshalingStream_Read(This,pv,cb,pcbRead)	\
    ( (This)->lpVtbl -> Read(This,pv,cb,pcbRead) ) 

#define IMarshalingStream_Write(This,pv,cb,pcbWritten)	\
    ( (This)->lpVtbl -> Write(This,pv,cb,pcbWritten) ) 


#define IMarshalingStream_Seek(This,dlibMove,dwOrigin,plibNewPosition)	\
    ( (This)->lpVtbl -> Seek(This,dlibMove,dwOrigin,plibNewPosition) ) 

#define IMarshalingStream_SetSize(This,libNewSize)	\
    ( (This)->lpVtbl -> SetSize(This,libNewSize) ) 

#define IMarshalingStream_CopyTo(This,pstm,cb,pcbRead,pcbWritten)	\
    ( (This)->lpVtbl -> CopyTo(This,pstm,cb,pcbRead,pcbWritten) ) 

#define IMarshalingStream_Commit(This,grfCommitFlags)	\
    ( (This)->lpVtbl -> Commit(This,grfCommitFlags) ) 

#define IMarshalingStream_Revert(This)	\
    ( (This)->lpVtbl -> Revert(This) ) 

#define IMarshalingStream_LockRegion(This,libOffset,cb,dwLockType)	\
    ( (This)->lpVtbl -> LockRegion(This,libOffset,cb,dwLockType) ) 

#define IMarshalingStream_UnlockRegion(This,libOffset,cb,dwLockType)	\
    ( (This)->lpVtbl -> UnlockRegion(This,libOffset,cb,dwLockType) ) 

#define IMarshalingStream_Stat(This,pstatstg,grfStatFlag)	\
    ( (This)->lpVtbl -> Stat(This,pstatstg,grfStatFlag) ) 

#define IMarshalingStream_Clone(This,ppstm)	\
    ( (This)->lpVtbl -> Clone(This,ppstm) ) 


#define IMarshalingStream_GetMarshalingContextAttribute(This,attribute,pAttributeValue)	\
    ( (This)->lpVtbl -> GetMarshalingContextAttribute(This,attribute,pAttributeValue) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IMarshalingStream_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_objidl_0000_0052 */
/* [local] */ 

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES) */
#pragma endregion
#pragma region Application Family or OneCore Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES)


extern RPC_IF_HANDLE __MIDL_itf_objidl_0000_0052_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_objidl_0000_0052_v0_0_s_ifspec;

#ifndef __IAgileReference_INTERFACE_DEFINED__
#define __IAgileReference_INTERFACE_DEFINED__

/* interface IAgileReference */
/* [unique][uuid][object][local] */ 

#if defined(__cplusplus) && !defined(CINTERFACE)
    EXTERN_C const IID IID_IAgileReference;
    extern "C++"
    {
        MIDL_INTERFACE("C03F6A43-65A4-9818-987E-E0B810D2A6F2")
        IAgileReference : public IUnknown
        {
        public:
            virtual HRESULT STDMETHODCALLTYPE Resolve( 
                /* [in] */ REFIID riid,
                /* [iid_is][retval][out] */ void **ppvObjectReference) = 0;

            template<class Q>
            HRESULT
#ifdef _M_CEE_PURE
            __clrcall
#else
            STDMETHODCALLTYPE
#endif
            Resolve(_COM_Outptr_ Q** pp)
            {
                return Resolve(__uuidof(Q), (void **)pp);
            }

        };
    } // extern C++
#else

EXTERN_C const IID IID_IAgileReference;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("C03F6A43-65A4-9818-987E-E0B810D2A6F2")
    IAgileReference : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE Resolve( 
            /* [in] */ REFIID riid,
            /* [iid_is][retval][out] */ void **ppvObjectReference) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IAgileReferenceVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IAgileReference * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IAgileReference * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IAgileReference * This);
        
        DECLSPEC_XFGVIRT(IAgileReference, Resolve)
        HRESULT ( STDMETHODCALLTYPE *Resolve )( 
            IAgileReference * This,
            /* [in] */ REFIID riid,
            /* [iid_is][retval][out] */ void **ppvObjectReference);
        
        END_INTERFACE
    } IAgileReferenceVtbl;

    interface IAgileReference
    {
        CONST_VTBL struct IAgileReferenceVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IAgileReference_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IAgileReference_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IAgileReference_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IAgileReference_Resolve(This,riid,ppvObjectReference)	\
    ( (This)->lpVtbl -> Resolve(This,riid,ppvObjectReference) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IAgileReference_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_objidl_0000_0053 */
/* [local] */ 

#endif
#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES) */
#pragma endregion
#pragma region Application Family or OneCore Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES)
EXTERN_C const GUID  IID_ICallbackWithNoReentrancyToApplicationSTA;
#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES) */
#pragma endregion
#pragma region Application Family or OneCore Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES)
typedef struct MachineGlobalObjectTableRegistrationToken__
    {
    int unused;
    } 	*MachineGlobalObjectTableRegistrationToken;



extern RPC_IF_HANDLE __MIDL_itf_objidl_0000_0053_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_objidl_0000_0053_v0_0_s_ifspec;

#ifndef __IMachineGlobalObjectTable_INTERFACE_DEFINED__
#define __IMachineGlobalObjectTable_INTERFACE_DEFINED__

/* interface IMachineGlobalObjectTable */
/* [unique][uuid][object][local] */ 


EXTERN_C const IID IID_IMachineGlobalObjectTable;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("26d709ac-f70b-4421-a96f-d2878fafb00d")
    IMachineGlobalObjectTable : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE RegisterObject( 
            /* [in] */ REFCLSID clsid,
            /* [in] */ LPCWSTR identifier,
            /* [in] */ IUnknown *object,
            /* [out] */ MachineGlobalObjectTableRegistrationToken *token) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetObject( 
            /* [in] */ REFCLSID clsid,
            /* [in] */ LPCWSTR identifier,
            /* [in] */ REFIID riid,
            /* [iid_is][out] */ void **ppv) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE RevokeObject( 
            /* [in] */ MachineGlobalObjectTableRegistrationToken token) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IMachineGlobalObjectTableVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IMachineGlobalObjectTable * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IMachineGlobalObjectTable * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IMachineGlobalObjectTable * This);
        
        DECLSPEC_XFGVIRT(IMachineGlobalObjectTable, RegisterObject)
        HRESULT ( STDMETHODCALLTYPE *RegisterObject )( 
            IMachineGlobalObjectTable * This,
            /* [in] */ REFCLSID clsid,
            /* [in] */ LPCWSTR identifier,
            /* [in] */ IUnknown *object,
            /* [out] */ MachineGlobalObjectTableRegistrationToken *token);
        
        DECLSPEC_XFGVIRT(IMachineGlobalObjectTable, GetObject)
        HRESULT ( STDMETHODCALLTYPE *GetObject )( 
            IMachineGlobalObjectTable * This,
            /* [in] */ REFCLSID clsid,
            /* [in] */ LPCWSTR identifier,
            /* [in] */ REFIID riid,
            /* [iid_is][out] */ void **ppv);
        
        DECLSPEC_XFGVIRT(IMachineGlobalObjectTable, RevokeObject)
        HRESULT ( STDMETHODCALLTYPE *RevokeObject )( 
            IMachineGlobalObjectTable * This,
            /* [in] */ MachineGlobalObjectTableRegistrationToken token);
        
        END_INTERFACE
    } IMachineGlobalObjectTableVtbl;

    interface IMachineGlobalObjectTable
    {
        CONST_VTBL struct IMachineGlobalObjectTableVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IMachineGlobalObjectTable_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IMachineGlobalObjectTable_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IMachineGlobalObjectTable_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IMachineGlobalObjectTable_RegisterObject(This,clsid,identifier,object,token)	\
    ( (This)->lpVtbl -> RegisterObject(This,clsid,identifier,object,token) ) 

#define IMachineGlobalObjectTable_GetObject(This,clsid,identifier,riid,ppv)	\
    ( (This)->lpVtbl -> GetObject(This,clsid,identifier,riid,ppv) ) 

#define IMachineGlobalObjectTable_RevokeObject(This,token)	\
    ( (This)->lpVtbl -> RevokeObject(This,token) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IMachineGlobalObjectTable_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_objidl_0000_0054 */
/* [local] */ 

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES) */
#pragma endregion
#pragma region Application Family or OneCore Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES)


extern RPC_IF_HANDLE __MIDL_itf_objidl_0000_0054_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_objidl_0000_0054_v0_0_s_ifspec;

#ifndef __ISupportAllowLowerTrustActivation_INTERFACE_DEFINED__
#define __ISupportAllowLowerTrustActivation_INTERFACE_DEFINED__

/* interface ISupportAllowLowerTrustActivation */
/* [uuid][object][local] */ 


EXTERN_C const IID IID_ISupportAllowLowerTrustActivation;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("e9956ef2-3828-4b4b-8fa9-7db61dee4954")
    ISupportAllowLowerTrustActivation : public IUnknown
    {
    public:
    };
    
    
#else 	/* C style interface */

    typedef struct ISupportAllowLowerTrustActivationVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            ISupportAllowLowerTrustActivation * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            ISupportAllowLowerTrustActivation * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            ISupportAllowLowerTrustActivation * This);
        
        END_INTERFACE
    } ISupportAllowLowerTrustActivationVtbl;

    interface ISupportAllowLowerTrustActivation
    {
        CONST_VTBL struct ISupportAllowLowerTrustActivationVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ISupportAllowLowerTrustActivation_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ISupportAllowLowerTrustActivation_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ISupportAllowLowerTrustActivation_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ISupportAllowLowerTrustActivation_INTERFACE_DEFINED__ */


#ifndef __ISupportActivationFromPackage_INTERFACE_DEFINED__
#define __ISupportActivationFromPackage_INTERFACE_DEFINED__

/* interface ISupportActivationFromPackage */
/* [uuid][object][local] */ 


EXTERN_C const IID IID_ISupportActivationFromPackage;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("0a18aae5-5caa-48c5-a9f4-6e46dcd58ad5")
    ISupportActivationFromPackage : public IUnknown
    {
    public:
    };
    
    
#else 	/* C style interface */

    typedef struct ISupportActivationFromPackageVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            ISupportActivationFromPackage * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            ISupportActivationFromPackage * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            ISupportActivationFromPackage * This);
        
        END_INTERFACE
    } ISupportActivationFromPackageVtbl;

    interface ISupportActivationFromPackage
    {
        CONST_VTBL struct ISupportActivationFromPackageVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ISupportActivationFromPackage_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ISupportActivationFromPackage_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ISupportActivationFromPackage_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ISupportActivationFromPackage_INTERFACE_DEFINED__ */


#ifndef __ISupportCoAddComDependencyOnPackage_INTERFACE_DEFINED__
#define __ISupportCoAddComDependencyOnPackage_INTERFACE_DEFINED__

/* interface ISupportCoAddComDependencyOnPackage */
/* [uuid][object][local] */ 


EXTERN_C const IID IID_ISupportCoAddComDependencyOnPackage;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("c8059efc-4e98-4fd0-bfc6-44190b80b823")
    ISupportCoAddComDependencyOnPackage : public IUnknown
    {
    public:
    };
    
    
#else 	/* C style interface */

    typedef struct ISupportCoAddComDependencyOnPackageVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            ISupportCoAddComDependencyOnPackage * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            ISupportCoAddComDependencyOnPackage * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            ISupportCoAddComDependencyOnPackage * This);
        
        END_INTERFACE
    } ISupportCoAddComDependencyOnPackageVtbl;

    interface ISupportCoAddComDependencyOnPackage
    {
        CONST_VTBL struct ISupportCoAddComDependencyOnPackageVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ISupportCoAddComDependencyOnPackage_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ISupportCoAddComDependencyOnPackage_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ISupportCoAddComDependencyOnPackage_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ISupportCoAddComDependencyOnPackage_INTERFACE_DEFINED__ */


#ifndef __ISupportServerMustBeEqualOrGreaterPrivilegeActivation_INTERFACE_DEFINED__
#define __ISupportServerMustBeEqualOrGreaterPrivilegeActivation_INTERFACE_DEFINED__

/* interface ISupportServerMustBeEqualOrGreaterPrivilegeActivation */
/* [uuid][object][local] */ 


EXTERN_C const IID IID_ISupportServerMustBeEqualOrGreaterPrivilegeActivation;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("5bdb3ee2-46bc-4313-b5fb-801c360ba5f9")
    ISupportServerMustBeEqualOrGreaterPrivilegeActivation : public IUnknown
    {
    public:
    };
    
    
#else 	/* C style interface */

    typedef struct ISupportServerMustBeEqualOrGreaterPrivilegeActivationVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            ISupportServerMustBeEqualOrGreaterPrivilegeActivation * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            ISupportServerMustBeEqualOrGreaterPrivilegeActivation * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            ISupportServerMustBeEqualOrGreaterPrivilegeActivation * This);
        
        END_INTERFACE
    } ISupportServerMustBeEqualOrGreaterPrivilegeActivationVtbl;

    interface ISupportServerMustBeEqualOrGreaterPrivilegeActivation
    {
        CONST_VTBL struct ISupportServerMustBeEqualOrGreaterPrivilegeActivationVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ISupportServerMustBeEqualOrGreaterPrivilegeActivation_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ISupportServerMustBeEqualOrGreaterPrivilegeActivation_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ISupportServerMustBeEqualOrGreaterPrivilegeActivation_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ISupportServerMustBeEqualOrGreaterPrivilegeActivation_INTERFACE_DEFINED__ */


#ifndef __ISupportDoNotElevateServerActivation_INTERFACE_DEFINED__
#define __ISupportDoNotElevateServerActivation_INTERFACE_DEFINED__

/* interface ISupportDoNotElevateServerActivation */
/* [uuid][object][local] */ 


EXTERN_C const IID IID_ISupportDoNotElevateServerActivation;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("40aefe22-3ff6-43dc-8108-c8c402d57b5c")
    ISupportDoNotElevateServerActivation : public IUnknown
    {
    public:
    };
    
    
#else 	/* C style interface */

    typedef struct ISupportDoNotElevateServerActivationVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            ISupportDoNotElevateServerActivation * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            ISupportDoNotElevateServerActivation * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            ISupportDoNotElevateServerActivation * This);
        
        END_INTERFACE
    } ISupportDoNotElevateServerActivationVtbl;

    interface ISupportDoNotElevateServerActivation
    {
        CONST_VTBL struct ISupportDoNotElevateServerActivationVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ISupportDoNotElevateServerActivation_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ISupportDoNotElevateServerActivation_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ISupportDoNotElevateServerActivation_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ISupportDoNotElevateServerActivation_INTERFACE_DEFINED__ */


#ifndef __ISupportActivateAsActivatorPackaged_INTERFACE_DEFINED__
#define __ISupportActivateAsActivatorPackaged_INTERFACE_DEFINED__

/* interface ISupportActivateAsActivatorPackaged */
/* [uuid][object][local] */ 


EXTERN_C const IID IID_ISupportActivateAsActivatorPackaged;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("765d1df2-f0af-4ef8-aa50-84789ca330ed")
    ISupportActivateAsActivatorPackaged : public IUnknown
    {
    public:
    };
    
    
#else 	/* C style interface */

    typedef struct ISupportActivateAsActivatorPackagedVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            ISupportActivateAsActivatorPackaged * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            ISupportActivateAsActivatorPackaged * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            ISupportActivateAsActivatorPackaged * This);
        
        END_INTERFACE
    } ISupportActivateAsActivatorPackagedVtbl;

    interface ISupportActivateAsActivatorPackaged
    {
        CONST_VTBL struct ISupportActivateAsActivatorPackagedVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ISupportActivateAsActivatorPackaged_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ISupportActivateAsActivatorPackaged_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ISupportActivateAsActivatorPackaged_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ISupportActivateAsActivatorPackaged_INTERFACE_DEFINED__ */


#ifndef __ISupportPackagedComRegistrationVisibility_INTERFACE_DEFINED__
#define __ISupportPackagedComRegistrationVisibility_INTERFACE_DEFINED__

/* interface ISupportPackagedComRegistrationVisibility */
/* [uuid][object][local] */ 


EXTERN_C const IID IID_ISupportPackagedComRegistrationVisibility;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("8dc3444e-c7ee-449a-9fb8-b9173988d66a")
    ISupportPackagedComRegistrationVisibility : public IUnknown
    {
    public:
    };
    
    
#else 	/* C style interface */

    typedef struct ISupportPackagedComRegistrationVisibilityVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            ISupportPackagedComRegistrationVisibility * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            ISupportPackagedComRegistrationVisibility * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            ISupportPackagedComRegistrationVisibility * This);
        
        END_INTERFACE
    } ISupportPackagedComRegistrationVisibilityVtbl;

    interface ISupportPackagedComRegistrationVisibility
    {
        CONST_VTBL struct ISupportPackagedComRegistrationVisibilityVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ISupportPackagedComRegistrationVisibility_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ISupportPackagedComRegistrationVisibility_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ISupportPackagedComRegistrationVisibility_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ISupportPackagedComRegistrationVisibility_INTERFACE_DEFINED__ */


#ifndef __ISupportPackagedComElevationEnabledClasses_INTERFACE_DEFINED__
#define __ISupportPackagedComElevationEnabledClasses_INTERFACE_DEFINED__

/* interface ISupportPackagedComElevationEnabledClasses */
/* [uuid][object][local] */ 


EXTERN_C const IID IID_ISupportPackagedComElevationEnabledClasses;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("b4219019-f712-4d4f-ade7-f468276af0b8")
    ISupportPackagedComElevationEnabledClasses : public IUnknown
    {
    public:
    };
    
    
#else 	/* C style interface */

    typedef struct ISupportPackagedComElevationEnabledClassesVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            ISupportPackagedComElevationEnabledClasses * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            ISupportPackagedComElevationEnabledClasses * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            ISupportPackagedComElevationEnabledClasses * This);
        
        END_INTERFACE
    } ISupportPackagedComElevationEnabledClassesVtbl;

    interface ISupportPackagedComElevationEnabledClasses
    {
        CONST_VTBL struct ISupportPackagedComElevationEnabledClassesVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ISupportPackagedComElevationEnabledClasses_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ISupportPackagedComElevationEnabledClasses_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ISupportPackagedComElevationEnabledClasses_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ISupportPackagedComElevationEnabledClasses_INTERFACE_DEFINED__ */


#ifndef __IPackagedComSyntaxSupport_INTERFACE_DEFINED__
#define __IPackagedComSyntaxSupport_INTERFACE_DEFINED__

/* interface IPackagedComSyntaxSupport */
/* [uuid][object][local] */ 


EXTERN_C const IID IID_IPackagedComSyntaxSupport;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("8f146474-b228-48fb-a58c-105ebb273abc")
    IPackagedComSyntaxSupport : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetSupportedVersion( 
            /* [out] */ UINT32 *supportedVersion) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IPackagedComSyntaxSupportVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IPackagedComSyntaxSupport * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IPackagedComSyntaxSupport * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IPackagedComSyntaxSupport * This);
        
        DECLSPEC_XFGVIRT(IPackagedComSyntaxSupport, GetSupportedVersion)
        HRESULT ( STDMETHODCALLTYPE *GetSupportedVersion )( 
            IPackagedComSyntaxSupport * This,
            /* [out] */ UINT32 *supportedVersion);
        
        END_INTERFACE
    } IPackagedComSyntaxSupportVtbl;

    interface IPackagedComSyntaxSupport
    {
        CONST_VTBL struct IPackagedComSyntaxSupportVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IPackagedComSyntaxSupport_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IPackagedComSyntaxSupport_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IPackagedComSyntaxSupport_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IPackagedComSyntaxSupport_GetSupportedVersion(This,supportedVersion)	\
    ( (This)->lpVtbl -> GetSupportedVersion(This,supportedVersion) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IPackagedComSyntaxSupport_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_objidl_0000_0063 */
/* [local] */ 

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES) */
#pragma endregion
#define _OBJIDLBASE_
#endif
#if ( _MSC_VER >= 800 )
#if _MSC_VER >= 1200
#pragma warning(pop)
#else
#pragma warning(default:4201)
#endif
#endif









#pragma region Desktop Family or OneCore Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM)


extern RPC_IF_HANDLE __MIDL_itf_objidl_0000_0063_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_objidl_0000_0063_v0_0_s_ifspec;

#ifndef __IMallocSpy_INTERFACE_DEFINED__
#define __IMallocSpy_INTERFACE_DEFINED__

/* interface IMallocSpy */
/* [uuid][object][local] */ 

typedef /* [unique] */ IMallocSpy *LPMALLOCSPY;


EXTERN_C const IID IID_IMallocSpy;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("0000001d-0000-0000-C000-000000000046")
    IMallocSpy : public IUnknown
    {
    public:
        virtual SIZE_T STDMETHODCALLTYPE PreAlloc( 
            /* [annotation][in] */ 
            _In_  SIZE_T cbRequest) = 0;
        
        virtual void *STDMETHODCALLTYPE PostAlloc( 
            /* [annotation][in] */ 
            _In_  void *pActual) = 0;
        
        virtual void *STDMETHODCALLTYPE PreFree( 
            /* [annotation][in] */ 
            _In_  void *pRequest,
            /* [annotation][in] */ 
            _In_  BOOL fSpyed) = 0;
        
        virtual void STDMETHODCALLTYPE PostFree( 
            /* [annotation][in] */ 
            _In_  BOOL fSpyed) = 0;
        
        virtual SIZE_T STDMETHODCALLTYPE PreRealloc( 
            /* [annotation][in] */ 
            _In_  void *pRequest,
            /* [annotation][in] */ 
            _In_  SIZE_T cbRequest,
            /* [annotation][out] */ 
            _Outptr_  void **ppNewRequest,
            /* [annotation][in] */ 
            _In_  BOOL fSpyed) = 0;
        
        virtual void *STDMETHODCALLTYPE PostRealloc( 
            /* [annotation][in] */ 
            _In_  void *pActual,
            /* [annotation][in] */ 
            _In_  BOOL fSpyed) = 0;
        
        virtual void *STDMETHODCALLTYPE PreGetSize( 
            /* [annotation][in] */ 
            _In_  void *pRequest,
            /* [annotation][in] */ 
            _In_  BOOL fSpyed) = 0;
        
        virtual SIZE_T STDMETHODCALLTYPE PostGetSize( 
            /* [annotation][in] */ 
            _In_  SIZE_T cbActual,
            /* [annotation][in] */ 
            _In_  BOOL fSpyed) = 0;
        
        virtual void *STDMETHODCALLTYPE PreDidAlloc( 
            /* [annotation][in] */ 
            _In_  void *pRequest,
            /* [annotation][in] */ 
            _In_  BOOL fSpyed) = 0;
        
        virtual int STDMETHODCALLTYPE PostDidAlloc( 
            /* [annotation][in] */ 
            _In_  void *pRequest,
            /* [annotation][in] */ 
            _In_  BOOL fSpyed,
            /* [annotation][in] */ 
            _In_  int fActual) = 0;
        
        virtual void STDMETHODCALLTYPE PreHeapMinimize( void) = 0;
        
        virtual void STDMETHODCALLTYPE PostHeapMinimize( void) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IMallocSpyVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IMallocSpy * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IMallocSpy * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IMallocSpy * This);
        
        DECLSPEC_XFGVIRT(IMallocSpy, PreAlloc)
        SIZE_T ( STDMETHODCALLTYPE *PreAlloc )( 
            IMallocSpy * This,
            /* [annotation][in] */ 
            _In_  SIZE_T cbRequest);
        
        DECLSPEC_XFGVIRT(IMallocSpy, PostAlloc)
        void *( STDMETHODCALLTYPE *PostAlloc )( 
            IMallocSpy * This,
            /* [annotation][in] */ 
            _In_  void *pActual);
        
        DECLSPEC_XFGVIRT(IMallocSpy, PreFree)
        void *( STDMETHODCALLTYPE *PreFree )( 
            IMallocSpy * This,
            /* [annotation][in] */ 
            _In_  void *pRequest,
            /* [annotation][in] */ 
            _In_  BOOL fSpyed);
        
        DECLSPEC_XFGVIRT(IMallocSpy, PostFree)
        void ( STDMETHODCALLTYPE *PostFree )( 
            IMallocSpy * This,
            /* [annotation][in] */ 
            _In_  BOOL fSpyed);
        
        DECLSPEC_XFGVIRT(IMallocSpy, PreRealloc)
        SIZE_T ( STDMETHODCALLTYPE *PreRealloc )( 
            IMallocSpy * This,
            /* [annotation][in] */ 
            _In_  void *pRequest,
            /* [annotation][in] */ 
            _In_  SIZE_T cbRequest,
            /* [annotation][out] */ 
            _Outptr_  void **ppNewRequest,
            /* [annotation][in] */ 
            _In_  BOOL fSpyed);
        
        DECLSPEC_XFGVIRT(IMallocSpy, PostRealloc)
        void *( STDMETHODCALLTYPE *PostRealloc )( 
            IMallocSpy * This,
            /* [annotation][in] */ 
            _In_  void *pActual,
            /* [annotation][in] */ 
            _In_  BOOL fSpyed);
        
        DECLSPEC_XFGVIRT(IMallocSpy, PreGetSize)
        void *( STDMETHODCALLTYPE *PreGetSize )( 
            IMallocSpy * This,
            /* [annotation][in] */ 
            _In_  void *pRequest,
            /* [annotation][in] */ 
            _In_  BOOL fSpyed);
        
        DECLSPEC_XFGVIRT(IMallocSpy, PostGetSize)
        SIZE_T ( STDMETHODCALLTYPE *PostGetSize )( 
            IMallocSpy * This,
            /* [annotation][in] */ 
            _In_  SIZE_T cbActual,
            /* [annotation][in] */ 
            _In_  BOOL fSpyed);
        
        DECLSPEC_XFGVIRT(IMallocSpy, PreDidAlloc)
        void *( STDMETHODCALLTYPE *PreDidAlloc )( 
            IMallocSpy * This,
            /* [annotation][in] */ 
            _In_  void *pRequest,
            /* [annotation][in] */ 
            _In_  BOOL fSpyed);
        
        DECLSPEC_XFGVIRT(IMallocSpy, PostDidAlloc)
        int ( STDMETHODCALLTYPE *PostDidAlloc )( 
            IMallocSpy * This,
            /* [annotation][in] */ 
            _In_  void *pRequest,
            /* [annotation][in] */ 
            _In_  BOOL fSpyed,
            /* [annotation][in] */ 
            _In_  int fActual);
        
        DECLSPEC_XFGVIRT(IMallocSpy, PreHeapMinimize)
        void ( STDMETHODCALLTYPE *PreHeapMinimize )( 
            IMallocSpy * This);
        
        DECLSPEC_XFGVIRT(IMallocSpy, PostHeapMinimize)
        void ( STDMETHODCALLTYPE *PostHeapMinimize )( 
            IMallocSpy * This);
        
        END_INTERFACE
    } IMallocSpyVtbl;

    interface IMallocSpy
    {
        CONST_VTBL struct IMallocSpyVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IMallocSpy_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IMallocSpy_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IMallocSpy_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IMallocSpy_PreAlloc(This,cbRequest)	\
    ( (This)->lpVtbl -> PreAlloc(This,cbRequest) ) 

#define IMallocSpy_PostAlloc(This,pActual)	\
    ( (This)->lpVtbl -> PostAlloc(This,pActual) ) 

#define IMallocSpy_PreFree(This,pRequest,fSpyed)	\
    ( (This)->lpVtbl -> PreFree(This,pRequest,fSpyed) ) 

#define IMallocSpy_PostFree(This,fSpyed)	\
    ( (This)->lpVtbl -> PostFree(This,fSpyed) ) 

#define IMallocSpy_PreRealloc(This,pRequest,cbRequest,ppNewRequest,fSpyed)	\
    ( (This)->lpVtbl -> PreRealloc(This,pRequest,cbRequest,ppNewRequest,fSpyed) ) 

#define IMallocSpy_PostRealloc(This,pActual,fSpyed)	\
    ( (This)->lpVtbl -> PostRealloc(This,pActual,fSpyed) ) 

#define IMallocSpy_PreGetSize(This,pRequest,fSpyed)	\
    ( (This)->lpVtbl -> PreGetSize(This,pRequest,fSpyed) ) 

#define IMallocSpy_PostGetSize(This,cbActual,fSpyed)	\
    ( (This)->lpVtbl -> PostGetSize(This,cbActual,fSpyed) ) 

#define IMallocSpy_PreDidAlloc(This,pRequest,fSpyed)	\
    ( (This)->lpVtbl -> PreDidAlloc(This,pRequest,fSpyed) ) 

#define IMallocSpy_PostDidAlloc(This,pRequest,fSpyed,fActual)	\
    ( (This)->lpVtbl -> PostDidAlloc(This,pRequest,fSpyed,fActual) ) 

#define IMallocSpy_PreHeapMinimize(This)	\
    ( (This)->lpVtbl -> PreHeapMinimize(This) ) 

#define IMallocSpy_PostHeapMinimize(This)	\
    ( (This)->lpVtbl -> PostHeapMinimize(This) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IMallocSpy_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_objidl_0000_0064 */
/* [local] */ 

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM) */
#pragma endregion
#pragma region Application Family or OneCore Family or Games Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES)


extern RPC_IF_HANDLE __MIDL_itf_objidl_0000_0064_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_objidl_0000_0064_v0_0_s_ifspec;

#ifndef __IBindCtx_INTERFACE_DEFINED__
#define __IBindCtx_INTERFACE_DEFINED__

/* interface IBindCtx */
/* [unique][uuid][object] */ 

typedef /* [unique] */  __RPC_unique_pointer IBindCtx *LPBC;

typedef /* [unique] */  __RPC_unique_pointer IBindCtx *LPBINDCTX;

#if defined(__cplusplus)
    typedef _Struct_size_bytes_(cbStruct) struct tagBIND_OPTS {
    DWORD           cbStruct;
    DWORD           grfFlags;
    DWORD           grfMode;
    DWORD           dwTickCountDeadline;
    } BIND_OPTS, * LPBIND_OPTS;
#else
typedef struct tagBIND_OPTS
    {
    DWORD cbStruct;
    DWORD grfFlags;
    DWORD grfMode;
    DWORD dwTickCountDeadline;
    } 	BIND_OPTS;

typedef struct tagBIND_OPTS *LPBIND_OPTS;

#endif
#if defined(__cplusplus)
    typedef struct tagBIND_OPTS2 : tagBIND_OPTS {
    DWORD           dwTrackFlags;
    DWORD           dwClassContext;
    LCID            locale;
    COSERVERINFO *  pServerInfo;
    } BIND_OPTS2, * LPBIND_OPTS2;
#else

typedef struct tagBIND_OPTS2
    {
    DWORD cbStruct;
    DWORD grfFlags;
    DWORD grfMode;
    DWORD dwTickCountDeadline;
    DWORD dwTrackFlags;
    DWORD dwClassContext;
    LCID locale;
    COSERVERINFO *pServerInfo;
    } 	BIND_OPTS2;

typedef struct tagBIND_OPTS2 *LPBIND_OPTS2;

#endif
#if defined(__cplusplus)
    typedef struct tagBIND_OPTS3 : tagBIND_OPTS2 {
   HWND           hwnd;
    } BIND_OPTS3, * LPBIND_OPTS3;
#else
typedef struct tagBIND_OPTS3
    {
    DWORD cbStruct;
    DWORD grfFlags;
    DWORD grfMode;
    DWORD dwTickCountDeadline;
    DWORD dwTrackFlags;
    DWORD dwClassContext;
    LCID locale;
    COSERVERINFO *pServerInfo;
    HWND hwnd;
    } 	BIND_OPTS3;

typedef struct tagBIND_OPTS3 *LPBIND_OPTS3;

#endif
typedef 
enum tagBIND_FLAGS
    {
        BIND_MAYBOTHERUSER	= 1,
        BIND_JUSTTESTEXISTENCE	= 2
    } 	BIND_FLAGS;


EXTERN_C const IID IID_IBindCtx;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("0000000e-0000-0000-C000-000000000046")
    IBindCtx : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE RegisterObjectBound( 
            /* [unique][in] */ __RPC__in_opt IUnknown *punk) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE RevokeObjectBound( 
            /* [unique][in] */ __RPC__in_opt IUnknown *punk) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE ReleaseBoundObjects( void) = 0;
        
        virtual /* [local] */ HRESULT STDMETHODCALLTYPE SetBindOptions( 
            /* [annotation][in] */ 
            _In_  BIND_OPTS *pbindopts) = 0;
        
        virtual /* [local] */ HRESULT STDMETHODCALLTYPE GetBindOptions( 
            /* [annotation][out][in] */ 
            _Inout_  BIND_OPTS *pbindopts) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetRunningObjectTable( 
            /* [out] */ __RPC__deref_out_opt IRunningObjectTable **pprot) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE RegisterObjectParam( 
            /* [in] */ __RPC__in LPOLESTR pszKey,
            /* [unique][in] */ __RPC__in_opt IUnknown *punk) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetObjectParam( 
            /* [in] */ __RPC__in LPOLESTR pszKey,
            /* [out] */ __RPC__deref_out_opt IUnknown **ppunk) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE EnumObjectParam( 
            /* [out] */ __RPC__deref_out_opt IEnumString **ppenum) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE RevokeObjectParam( 
            /* [in] */ __RPC__in LPOLESTR pszKey) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IBindCtxVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IBindCtx * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IBindCtx * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IBindCtx * This);
        
        DECLSPEC_XFGVIRT(IBindCtx, RegisterObjectBound)
        HRESULT ( STDMETHODCALLTYPE *RegisterObjectBound )( 
            __RPC__in IBindCtx * This,
            /* [unique][in] */ __RPC__in_opt IUnknown *punk);
        
        DECLSPEC_XFGVIRT(IBindCtx, RevokeObjectBound)
        HRESULT ( STDMETHODCALLTYPE *RevokeObjectBound )( 
            __RPC__in IBindCtx * This,
            /* [unique][in] */ __RPC__in_opt IUnknown *punk);
        
        DECLSPEC_XFGVIRT(IBindCtx, ReleaseBoundObjects)
        HRESULT ( STDMETHODCALLTYPE *ReleaseBoundObjects )( 
            __RPC__in IBindCtx * This);
        
        DECLSPEC_XFGVIRT(IBindCtx, SetBindOptions)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *SetBindOptions )( 
            IBindCtx * This,
            /* [annotation][in] */ 
            _In_  BIND_OPTS *pbindopts);
        
        DECLSPEC_XFGVIRT(IBindCtx, GetBindOptions)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *GetBindOptions )( 
            IBindCtx * This,
            /* [annotation][out][in] */ 
            _Inout_  BIND_OPTS *pbindopts);
        
        DECLSPEC_XFGVIRT(IBindCtx, GetRunningObjectTable)
        HRESULT ( STDMETHODCALLTYPE *GetRunningObjectTable )( 
            __RPC__in IBindCtx * This,
            /* [out] */ __RPC__deref_out_opt IRunningObjectTable **pprot);
        
        DECLSPEC_XFGVIRT(IBindCtx, RegisterObjectParam)
        HRESULT ( STDMETHODCALLTYPE *RegisterObjectParam )( 
            __RPC__in IBindCtx * This,
            /* [in] */ __RPC__in LPOLESTR pszKey,
            /* [unique][in] */ __RPC__in_opt IUnknown *punk);
        
        DECLSPEC_XFGVIRT(IBindCtx, GetObjectParam)
        HRESULT ( STDMETHODCALLTYPE *GetObjectParam )( 
            __RPC__in IBindCtx * This,
            /* [in] */ __RPC__in LPOLESTR pszKey,
            /* [out] */ __RPC__deref_out_opt IUnknown **ppunk);
        
        DECLSPEC_XFGVIRT(IBindCtx, EnumObjectParam)
        HRESULT ( STDMETHODCALLTYPE *EnumObjectParam )( 
            __RPC__in IBindCtx * This,
            /* [out] */ __RPC__deref_out_opt IEnumString **ppenum);
        
        DECLSPEC_XFGVIRT(IBindCtx, RevokeObjectParam)
        HRESULT ( STDMETHODCALLTYPE *RevokeObjectParam )( 
            __RPC__in IBindCtx * This,
            /* [in] */ __RPC__in LPOLESTR pszKey);
        
        END_INTERFACE
    } IBindCtxVtbl;

    interface IBindCtx
    {
        CONST_VTBL struct IBindCtxVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IBindCtx_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IBindCtx_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IBindCtx_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IBindCtx_RegisterObjectBound(This,punk)	\
    ( (This)->lpVtbl -> RegisterObjectBound(This,punk) ) 

#define IBindCtx_RevokeObjectBound(This,punk)	\
    ( (This)->lpVtbl -> RevokeObjectBound(This,punk) ) 

#define IBindCtx_ReleaseBoundObjects(This)	\
    ( (This)->lpVtbl -> ReleaseBoundObjects(This) ) 

#define IBindCtx_SetBindOptions(This,pbindopts)	\
    ( (This)->lpVtbl -> SetBindOptions(This,pbindopts) ) 

#define IBindCtx_GetBindOptions(This,pbindopts)	\
    ( (This)->lpVtbl -> GetBindOptions(This,pbindopts) ) 

#define IBindCtx_GetRunningObjectTable(This,pprot)	\
    ( (This)->lpVtbl -> GetRunningObjectTable(This,pprot) ) 

#define IBindCtx_RegisterObjectParam(This,pszKey,punk)	\
    ( (This)->lpVtbl -> RegisterObjectParam(This,pszKey,punk) ) 

#define IBindCtx_GetObjectParam(This,pszKey,ppunk)	\
    ( (This)->lpVtbl -> GetObjectParam(This,pszKey,ppunk) ) 

#define IBindCtx_EnumObjectParam(This,ppenum)	\
    ( (This)->lpVtbl -> EnumObjectParam(This,ppenum) ) 

#define IBindCtx_RevokeObjectParam(This,pszKey)	\
    ( (This)->lpVtbl -> RevokeObjectParam(This,pszKey) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */



/* [call_as] */ HRESULT STDMETHODCALLTYPE IBindCtx_RemoteSetBindOptions_Proxy( 
    __RPC__in IBindCtx * This,
    /* [in] */ __RPC__in BIND_OPTS2 *pbindopts);


void __RPC_STUB IBindCtx_RemoteSetBindOptions_Stub(
    IRpcStubBuffer *This,
    IRpcChannelBuffer *_pRpcChannelBuffer,
    PRPC_MESSAGE _pRpcMessage,
    DWORD *_pdwStubPhase);


/* [call_as] */ HRESULT STDMETHODCALLTYPE IBindCtx_RemoteGetBindOptions_Proxy( 
    __RPC__in IBindCtx * This,
    /* [out][in] */ __RPC__inout BIND_OPTS2 *pbindopts);


void __RPC_STUB IBindCtx_RemoteGetBindOptions_Stub(
    IRpcStubBuffer *This,
    IRpcChannelBuffer *_pRpcChannelBuffer,
    PRPC_MESSAGE _pRpcMessage,
    DWORD *_pdwStubPhase);



#endif 	/* __IBindCtx_INTERFACE_DEFINED__ */


#ifndef __IEnumMoniker_INTERFACE_DEFINED__
#define __IEnumMoniker_INTERFACE_DEFINED__

/* interface IEnumMoniker */
/* [unique][uuid][object] */ 

typedef /* [unique] */  __RPC_unique_pointer IEnumMoniker *LPENUMMONIKER;


EXTERN_C const IID IID_IEnumMoniker;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("00000102-0000-0000-C000-000000000046")
    IEnumMoniker : public IUnknown
    {
    public:
        virtual /* [local] */ HRESULT STDMETHODCALLTYPE Next( 
            /* [in] */ ULONG celt,
            /* [annotation] */ 
            _Out_writes_to_(celt,*pceltFetched)  IMoniker **rgelt,
            /* [annotation] */ 
            _Out_opt_  ULONG *pceltFetched) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Skip( 
            /* [in] */ ULONG celt) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Reset( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Clone( 
            /* [out] */ __RPC__deref_out_opt IEnumMoniker **ppenum) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IEnumMonikerVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IEnumMoniker * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IEnumMoniker * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IEnumMoniker * This);
        
        DECLSPEC_XFGVIRT(IEnumMoniker, Next)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Next )( 
            IEnumMoniker * This,
            /* [in] */ ULONG celt,
            /* [annotation] */ 
            _Out_writes_to_(celt,*pceltFetched)  IMoniker **rgelt,
            /* [annotation] */ 
            _Out_opt_  ULONG *pceltFetched);
        
        DECLSPEC_XFGVIRT(IEnumMoniker, Skip)
        HRESULT ( STDMETHODCALLTYPE *Skip )( 
            __RPC__in IEnumMoniker * This,
            /* [in] */ ULONG celt);
        
        DECLSPEC_XFGVIRT(IEnumMoniker, Reset)
        HRESULT ( STDMETHODCALLTYPE *Reset )( 
            __RPC__in IEnumMoniker * This);
        
        DECLSPEC_XFGVIRT(IEnumMoniker, Clone)
        HRESULT ( STDMETHODCALLTYPE *Clone )( 
            __RPC__in IEnumMoniker * This,
            /* [out] */ __RPC__deref_out_opt IEnumMoniker **ppenum);
        
        END_INTERFACE
    } IEnumMonikerVtbl;

    interface IEnumMoniker
    {
        CONST_VTBL struct IEnumMonikerVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IEnumMoniker_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IEnumMoniker_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IEnumMoniker_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IEnumMoniker_Next(This,celt,rgelt,pceltFetched)	\
    ( (This)->lpVtbl -> Next(This,celt,rgelt,pceltFetched) ) 

#define IEnumMoniker_Skip(This,celt)	\
    ( (This)->lpVtbl -> Skip(This,celt) ) 

#define IEnumMoniker_Reset(This)	\
    ( (This)->lpVtbl -> Reset(This) ) 

#define IEnumMoniker_Clone(This,ppenum)	\
    ( (This)->lpVtbl -> Clone(This,ppenum) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */



/* [call_as] */ HRESULT STDMETHODCALLTYPE IEnumMoniker_RemoteNext_Proxy( 
    __RPC__in IEnumMoniker * This,
    /* [in] */ ULONG celt,
    /* [length_is][size_is][out] */ __RPC__out_ecount_part(celt, *pceltFetched) IMoniker **rgelt,
    /* [out] */ __RPC__out ULONG *pceltFetched);


void __RPC_STUB IEnumMoniker_RemoteNext_Stub(
    IRpcStubBuffer *This,
    IRpcChannelBuffer *_pRpcChannelBuffer,
    PRPC_MESSAGE _pRpcMessage,
    DWORD *_pdwStubPhase);



#endif 	/* __IEnumMoniker_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_objidl_0000_0066 */
/* [local] */ 

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES) */
#pragma endregion
#pragma region Desktop Family or OneCore Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM)


extern RPC_IF_HANDLE __MIDL_itf_objidl_0000_0066_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_objidl_0000_0066_v0_0_s_ifspec;

#ifndef __IRunnableObject_INTERFACE_DEFINED__
#define __IRunnableObject_INTERFACE_DEFINED__

/* interface IRunnableObject */
/* [uuid][object] */ 

typedef /* [unique] */  __RPC_unique_pointer IRunnableObject *LPRUNNABLEOBJECT;


EXTERN_C const IID IID_IRunnableObject;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("00000126-0000-0000-C000-000000000046")
    IRunnableObject : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetRunningClass( 
            /* [out] */ __RPC__out LPCLSID lpClsid) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Run( 
            /* [in] */ __RPC__in_opt LPBINDCTX pbc) = 0;
        
        virtual /* [local] */ BOOL STDMETHODCALLTYPE IsRunning( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE LockRunning( 
            /* [in] */ BOOL fLock,
            /* [in] */ BOOL fLastUnlockCloses) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetContainedObject( 
            /* [in] */ BOOL fContained) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IRunnableObjectVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IRunnableObject * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IRunnableObject * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IRunnableObject * This);
        
        DECLSPEC_XFGVIRT(IRunnableObject, GetRunningClass)
        HRESULT ( STDMETHODCALLTYPE *GetRunningClass )( 
            __RPC__in IRunnableObject * This,
            /* [out] */ __RPC__out LPCLSID lpClsid);
        
        DECLSPEC_XFGVIRT(IRunnableObject, Run)
        HRESULT ( STDMETHODCALLTYPE *Run )( 
            __RPC__in IRunnableObject * This,
            /* [in] */ __RPC__in_opt LPBINDCTX pbc);
        
        DECLSPEC_XFGVIRT(IRunnableObject, IsRunning)
        /* [local] */ BOOL ( STDMETHODCALLTYPE *IsRunning )( 
            IRunnableObject * This);
        
        DECLSPEC_XFGVIRT(IRunnableObject, LockRunning)
        HRESULT ( STDMETHODCALLTYPE *LockRunning )( 
            __RPC__in IRunnableObject * This,
            /* [in] */ BOOL fLock,
            /* [in] */ BOOL fLastUnlockCloses);
        
        DECLSPEC_XFGVIRT(IRunnableObject, SetContainedObject)
        HRESULT ( STDMETHODCALLTYPE *SetContainedObject )( 
            __RPC__in IRunnableObject * This,
            /* [in] */ BOOL fContained);
        
        END_INTERFACE
    } IRunnableObjectVtbl;

    interface IRunnableObject
    {
        CONST_VTBL struct IRunnableObjectVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IRunnableObject_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IRunnableObject_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IRunnableObject_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IRunnableObject_GetRunningClass(This,lpClsid)	\
    ( (This)->lpVtbl -> GetRunningClass(This,lpClsid) ) 

#define IRunnableObject_Run(This,pbc)	\
    ( (This)->lpVtbl -> Run(This,pbc) ) 

#define IRunnableObject_IsRunning(This)	\
    ( (This)->lpVtbl -> IsRunning(This) ) 

#define IRunnableObject_LockRunning(This,fLock,fLastUnlockCloses)	\
    ( (This)->lpVtbl -> LockRunning(This,fLock,fLastUnlockCloses) ) 

#define IRunnableObject_SetContainedObject(This,fContained)	\
    ( (This)->lpVtbl -> SetContainedObject(This,fContained) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */



/* [call_as] */ HRESULT STDMETHODCALLTYPE IRunnableObject_RemoteIsRunning_Proxy( 
    __RPC__in IRunnableObject * This);


void __RPC_STUB IRunnableObject_RemoteIsRunning_Stub(
    IRpcStubBuffer *This,
    IRpcChannelBuffer *_pRpcChannelBuffer,
    PRPC_MESSAGE _pRpcMessage,
    DWORD *_pdwStubPhase);



#endif 	/* __IRunnableObject_INTERFACE_DEFINED__ */


#ifndef __IRunningObjectTable_INTERFACE_DEFINED__
#define __IRunningObjectTable_INTERFACE_DEFINED__

/* interface IRunningObjectTable */
/* [uuid][object] */ 

typedef /* [unique] */  __RPC_unique_pointer IRunningObjectTable *LPRUNNINGOBJECTTABLE;


EXTERN_C const IID IID_IRunningObjectTable;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("00000010-0000-0000-C000-000000000046")
    IRunningObjectTable : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE Register( 
            /* [in] */ DWORD grfFlags,
            /* [unique][in] */ __RPC__in_opt IUnknown *punkObject,
            /* [unique][in] */ __RPC__in_opt IMoniker *pmkObjectName,
            /* [out] */ __RPC__out DWORD *pdwRegister) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Revoke( 
            /* [in] */ DWORD dwRegister) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE IsRunning( 
            /* [unique][in] */ __RPC__in_opt IMoniker *pmkObjectName) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetObject( 
            /* [unique][in] */ __RPC__in_opt IMoniker *pmkObjectName,
            /* [out] */ __RPC__deref_out_opt IUnknown **ppunkObject) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE NoteChangeTime( 
            /* [in] */ DWORD dwRegister,
            /* [in] */ __RPC__in FILETIME *pfiletime) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetTimeOfLastChange( 
            /* [unique][in] */ __RPC__in_opt IMoniker *pmkObjectName,
            /* [out] */ __RPC__out FILETIME *pfiletime) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE EnumRunning( 
            /* [out] */ __RPC__deref_out_opt IEnumMoniker **ppenumMoniker) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IRunningObjectTableVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IRunningObjectTable * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IRunningObjectTable * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IRunningObjectTable * This);
        
        DECLSPEC_XFGVIRT(IRunningObjectTable, Register)
        HRESULT ( STDMETHODCALLTYPE *Register )( 
            __RPC__in IRunningObjectTable * This,
            /* [in] */ DWORD grfFlags,
            /* [unique][in] */ __RPC__in_opt IUnknown *punkObject,
            /* [unique][in] */ __RPC__in_opt IMoniker *pmkObjectName,
            /* [out] */ __RPC__out DWORD *pdwRegister);
        
        DECLSPEC_XFGVIRT(IRunningObjectTable, Revoke)
        HRESULT ( STDMETHODCALLTYPE *Revoke )( 
            __RPC__in IRunningObjectTable * This,
            /* [in] */ DWORD dwRegister);
        
        DECLSPEC_XFGVIRT(IRunningObjectTable, IsRunning)
        HRESULT ( STDMETHODCALLTYPE *IsRunning )( 
            __RPC__in IRunningObjectTable * This,
            /* [unique][in] */ __RPC__in_opt IMoniker *pmkObjectName);
        
        DECLSPEC_XFGVIRT(IRunningObjectTable, GetObject)
        HRESULT ( STDMETHODCALLTYPE *GetObject )( 
            __RPC__in IRunningObjectTable * This,
            /* [unique][in] */ __RPC__in_opt IMoniker *pmkObjectName,
            /* [out] */ __RPC__deref_out_opt IUnknown **ppunkObject);
        
        DECLSPEC_XFGVIRT(IRunningObjectTable, NoteChangeTime)
        HRESULT ( STDMETHODCALLTYPE *NoteChangeTime )( 
            __RPC__in IRunningObjectTable * This,
            /* [in] */ DWORD dwRegister,
            /* [in] */ __RPC__in FILETIME *pfiletime);
        
        DECLSPEC_XFGVIRT(IRunningObjectTable, GetTimeOfLastChange)
        HRESULT ( STDMETHODCALLTYPE *GetTimeOfLastChange )( 
            __RPC__in IRunningObjectTable * This,
            /* [unique][in] */ __RPC__in_opt IMoniker *pmkObjectName,
            /* [out] */ __RPC__out FILETIME *pfiletime);
        
        DECLSPEC_XFGVIRT(IRunningObjectTable, EnumRunning)
        HRESULT ( STDMETHODCALLTYPE *EnumRunning )( 
            __RPC__in IRunningObjectTable * This,
            /* [out] */ __RPC__deref_out_opt IEnumMoniker **ppenumMoniker);
        
        END_INTERFACE
    } IRunningObjectTableVtbl;

    interface IRunningObjectTable
    {
        CONST_VTBL struct IRunningObjectTableVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IRunningObjectTable_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IRunningObjectTable_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IRunningObjectTable_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IRunningObjectTable_Register(This,grfFlags,punkObject,pmkObjectName,pdwRegister)	\
    ( (This)->lpVtbl -> Register(This,grfFlags,punkObject,pmkObjectName,pdwRegister) ) 

#define IRunningObjectTable_Revoke(This,dwRegister)	\
    ( (This)->lpVtbl -> Revoke(This,dwRegister) ) 

#define IRunningObjectTable_IsRunning(This,pmkObjectName)	\
    ( (This)->lpVtbl -> IsRunning(This,pmkObjectName) ) 

#define IRunningObjectTable_GetObject(This,pmkObjectName,ppunkObject)	\
    ( (This)->lpVtbl -> GetObject(This,pmkObjectName,ppunkObject) ) 

#define IRunningObjectTable_NoteChangeTime(This,dwRegister,pfiletime)	\
    ( (This)->lpVtbl -> NoteChangeTime(This,dwRegister,pfiletime) ) 

#define IRunningObjectTable_GetTimeOfLastChange(This,pmkObjectName,pfiletime)	\
    ( (This)->lpVtbl -> GetTimeOfLastChange(This,pmkObjectName,pfiletime) ) 

#define IRunningObjectTable_EnumRunning(This,ppenumMoniker)	\
    ( (This)->lpVtbl -> EnumRunning(This,ppenumMoniker) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IRunningObjectTable_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_objidl_0000_0068 */
/* [local] */ 

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM) */
#pragma endregion
#pragma region Application Family or OneCore Family or Games Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES)


extern RPC_IF_HANDLE __MIDL_itf_objidl_0000_0068_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_objidl_0000_0068_v0_0_s_ifspec;

#ifndef __IPersist_INTERFACE_DEFINED__
#define __IPersist_INTERFACE_DEFINED__

/* interface IPersist */
/* [uuid][object] */ 

typedef /* [unique] */  __RPC_unique_pointer IPersist *LPPERSIST;


EXTERN_C const IID IID_IPersist;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("0000010c-0000-0000-C000-000000000046")
    IPersist : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetClassID( 
            /* [out] */ __RPC__out CLSID *pClassID) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IPersistVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IPersist * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IPersist * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IPersist * This);
        
        DECLSPEC_XFGVIRT(IPersist, GetClassID)
        HRESULT ( STDMETHODCALLTYPE *GetClassID )( 
            __RPC__in IPersist * This,
            /* [out] */ __RPC__out CLSID *pClassID);
        
        END_INTERFACE
    } IPersistVtbl;

    interface IPersist
    {
        CONST_VTBL struct IPersistVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IPersist_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IPersist_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IPersist_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IPersist_GetClassID(This,pClassID)	\
    ( (This)->lpVtbl -> GetClassID(This,pClassID) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IPersist_INTERFACE_DEFINED__ */


#ifndef __IPersistStream_INTERFACE_DEFINED__
#define __IPersistStream_INTERFACE_DEFINED__

/* interface IPersistStream */
/* [unique][uuid][object] */ 

typedef /* [unique] */  __RPC_unique_pointer IPersistStream *LPPERSISTSTREAM;


EXTERN_C const IID IID_IPersistStream;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("00000109-0000-0000-C000-000000000046")
    IPersistStream : public IPersist
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE IsDirty( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Load( 
            /* [unique][in] */ __RPC__in_opt IStream *pStm) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Save( 
            /* [unique][in] */ __RPC__in_opt IStream *pStm,
            /* [in] */ BOOL fClearDirty) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetSizeMax( 
            /* [out] */ __RPC__out ULARGE_INTEGER *pcbSize) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IPersistStreamVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IPersistStream * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IPersistStream * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IPersistStream * This);
        
        DECLSPEC_XFGVIRT(IPersist, GetClassID)
        HRESULT ( STDMETHODCALLTYPE *GetClassID )( 
            __RPC__in IPersistStream * This,
            /* [out] */ __RPC__out CLSID *pClassID);
        
        DECLSPEC_XFGVIRT(IPersistStream, IsDirty)
        HRESULT ( STDMETHODCALLTYPE *IsDirty )( 
            __RPC__in IPersistStream * This);
        
        DECLSPEC_XFGVIRT(IPersistStream, Load)
        HRESULT ( STDMETHODCALLTYPE *Load )( 
            __RPC__in IPersistStream * This,
            /* [unique][in] */ __RPC__in_opt IStream *pStm);
        
        DECLSPEC_XFGVIRT(IPersistStream, Save)
        HRESULT ( STDMETHODCALLTYPE *Save )( 
            __RPC__in IPersistStream * This,
            /* [unique][in] */ __RPC__in_opt IStream *pStm,
            /* [in] */ BOOL fClearDirty);
        
        DECLSPEC_XFGVIRT(IPersistStream, GetSizeMax)
        HRESULT ( STDMETHODCALLTYPE *GetSizeMax )( 
            __RPC__in IPersistStream * This,
            /* [out] */ __RPC__out ULARGE_INTEGER *pcbSize);
        
        END_INTERFACE
    } IPersistStreamVtbl;

    interface IPersistStream
    {
        CONST_VTBL struct IPersistStreamVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IPersistStream_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IPersistStream_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IPersistStream_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IPersistStream_GetClassID(This,pClassID)	\
    ( (This)->lpVtbl -> GetClassID(This,pClassID) ) 


#define IPersistStream_IsDirty(This)	\
    ( (This)->lpVtbl -> IsDirty(This) ) 

#define IPersistStream_Load(This,pStm)	\
    ( (This)->lpVtbl -> Load(This,pStm) ) 

#define IPersistStream_Save(This,pStm,fClearDirty)	\
    ( (This)->lpVtbl -> Save(This,pStm,fClearDirty) ) 

#define IPersistStream_GetSizeMax(This,pcbSize)	\
    ( (This)->lpVtbl -> GetSizeMax(This,pcbSize) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IPersistStream_INTERFACE_DEFINED__ */


#ifndef __IMoniker_INTERFACE_DEFINED__
#define __IMoniker_INTERFACE_DEFINED__

/* interface IMoniker */
/* [unique][uuid][object] */ 

typedef /* [unique] */  __RPC_unique_pointer IMoniker *LPMONIKER;

typedef 
enum tagMKSYS
    {
        MKSYS_NONE	= 0,
        MKSYS_GENERICCOMPOSITE	= 1,
        MKSYS_FILEMONIKER	= 2,
        MKSYS_ANTIMONIKER	= 3,
        MKSYS_ITEMMONIKER	= 4,
        MKSYS_POINTERMONIKER	= 5,
        MKSYS_CLASSMONIKER	= 7,
        MKSYS_OBJREFMONIKER	= 8,
        MKSYS_SESSIONMONIKER	= 9,
        MKSYS_LUAMONIKER	= 10
    } 	MKSYS;

typedef /* [v1_enum] */ 
enum tagMKREDUCE
    {
        MKRREDUCE_ONE	= ( 3 << 16 ) ,
        MKRREDUCE_TOUSER	= ( 2 << 16 ) ,
        MKRREDUCE_THROUGHUSER	= ( 1 << 16 ) ,
        MKRREDUCE_ALL	= 0
    } 	MKRREDUCE;


EXTERN_C const IID IID_IMoniker;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("0000000f-0000-0000-C000-000000000046")
    IMoniker : public IPersistStream
    {
    public:
        virtual /* [local] */ HRESULT STDMETHODCALLTYPE BindToObject( 
            /* [annotation][unique][in] */ 
            _In_  IBindCtx *pbc,
            /* [annotation][unique][in] */ 
            _In_opt_  IMoniker *pmkToLeft,
            /* [annotation][in] */ 
            _In_  REFIID riidResult,
            /* [annotation][iid_is][out] */ 
            _Outptr_  void **ppvResult) = 0;
        
        virtual /* [local] */ HRESULT STDMETHODCALLTYPE BindToStorage( 
            /* [annotation][unique][in] */ 
            _In_  IBindCtx *pbc,
            /* [annotation][unique][in] */ 
            _In_opt_  IMoniker *pmkToLeft,
            /* [annotation][in] */ 
            _In_  REFIID riid,
            /* [annotation][iid_is][out] */ 
            _Outptr_  void **ppvObj) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Reduce( 
            /* [unique][in] */ __RPC__in_opt IBindCtx *pbc,
            /* [in] */ DWORD dwReduceHowFar,
            /* [unique][out][in] */ __RPC__deref_opt_inout_opt IMoniker **ppmkToLeft,
            /* [out] */ __RPC__deref_out_opt IMoniker **ppmkReduced) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE ComposeWith( 
            /* [unique][in] */ __RPC__in_opt IMoniker *pmkRight,
            /* [in] */ BOOL fOnlyIfNotGeneric,
            /* [out] */ __RPC__deref_out_opt IMoniker **ppmkComposite) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Enum( 
            /* [in] */ BOOL fForward,
            /* [out] */ __RPC__deref_out_opt IEnumMoniker **ppenumMoniker) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE IsEqual( 
            /* [unique][in] */ __RPC__in_opt IMoniker *pmkOtherMoniker) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Hash( 
            /* [out] */ __RPC__out DWORD *pdwHash) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE IsRunning( 
            /* [unique][in] */ __RPC__in_opt IBindCtx *pbc,
            /* [unique][in] */ __RPC__in_opt IMoniker *pmkToLeft,
            /* [unique][in] */ __RPC__in_opt IMoniker *pmkNewlyRunning) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetTimeOfLastChange( 
            /* [unique][in] */ __RPC__in_opt IBindCtx *pbc,
            /* [unique][in] */ __RPC__in_opt IMoniker *pmkToLeft,
            /* [out] */ __RPC__out FILETIME *pFileTime) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Inverse( 
            /* [out] */ __RPC__deref_out_opt IMoniker **ppmk) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE CommonPrefixWith( 
            /* [unique][in] */ __RPC__in_opt IMoniker *pmkOther,
            /* [out] */ __RPC__deref_out_opt IMoniker **ppmkPrefix) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE RelativePathTo( 
            /* [unique][in] */ __RPC__in_opt IMoniker *pmkOther,
            /* [out] */ __RPC__deref_out_opt IMoniker **ppmkRelPath) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetDisplayName( 
            /* [unique][in] */ __RPC__in_opt IBindCtx *pbc,
            /* [unique][in] */ __RPC__in_opt IMoniker *pmkToLeft,
            /* [out] */ __RPC__deref_out_opt LPOLESTR *ppszDisplayName) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE ParseDisplayName( 
            /* [unique][in] */ __RPC__in_opt IBindCtx *pbc,
            /* [unique][in] */ __RPC__in_opt IMoniker *pmkToLeft,
            /* [in] */ __RPC__in LPOLESTR pszDisplayName,
            /* [out] */ __RPC__out ULONG *pchEaten,
            /* [out] */ __RPC__deref_out_opt IMoniker **ppmkOut) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE IsSystemMoniker( 
            /* [out] */ __RPC__out DWORD *pdwMksys) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IMonikerVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IMoniker * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IMoniker * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IMoniker * This);
        
        DECLSPEC_XFGVIRT(IPersist, GetClassID)
        HRESULT ( STDMETHODCALLTYPE *GetClassID )( 
            __RPC__in IMoniker * This,
            /* [out] */ __RPC__out CLSID *pClassID);
        
        DECLSPEC_XFGVIRT(IPersistStream, IsDirty)
        HRESULT ( STDMETHODCALLTYPE *IsDirty )( 
            __RPC__in IMoniker * This);
        
        DECLSPEC_XFGVIRT(IPersistStream, Load)
        HRESULT ( STDMETHODCALLTYPE *Load )( 
            __RPC__in IMoniker * This,
            /* [unique][in] */ __RPC__in_opt IStream *pStm);
        
        DECLSPEC_XFGVIRT(IPersistStream, Save)
        HRESULT ( STDMETHODCALLTYPE *Save )( 
            __RPC__in IMoniker * This,
            /* [unique][in] */ __RPC__in_opt IStream *pStm,
            /* [in] */ BOOL fClearDirty);
        
        DECLSPEC_XFGVIRT(IPersistStream, GetSizeMax)
        HRESULT ( STDMETHODCALLTYPE *GetSizeMax )( 
            __RPC__in IMoniker * This,
            /* [out] */ __RPC__out ULARGE_INTEGER *pcbSize);
        
        DECLSPEC_XFGVIRT(IMoniker, BindToObject)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *BindToObject )( 
            IMoniker * This,
            /* [annotation][unique][in] */ 
            _In_  IBindCtx *pbc,
            /* [annotation][unique][in] */ 
            _In_opt_  IMoniker *pmkToLeft,
            /* [annotation][in] */ 
            _In_  REFIID riidResult,
            /* [annotation][iid_is][out] */ 
            _Outptr_  void **ppvResult);
        
        DECLSPEC_XFGVIRT(IMoniker, BindToStorage)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *BindToStorage )( 
            IMoniker * This,
            /* [annotation][unique][in] */ 
            _In_  IBindCtx *pbc,
            /* [annotation][unique][in] */ 
            _In_opt_  IMoniker *pmkToLeft,
            /* [annotation][in] */ 
            _In_  REFIID riid,
            /* [annotation][iid_is][out] */ 
            _Outptr_  void **ppvObj);
        
        DECLSPEC_XFGVIRT(IMoniker, Reduce)
        HRESULT ( STDMETHODCALLTYPE *Reduce )( 
            __RPC__in IMoniker * This,
            /* [unique][in] */ __RPC__in_opt IBindCtx *pbc,
            /* [in] */ DWORD dwReduceHowFar,
            /* [unique][out][in] */ __RPC__deref_opt_inout_opt IMoniker **ppmkToLeft,
            /* [out] */ __RPC__deref_out_opt IMoniker **ppmkReduced);
        
        DECLSPEC_XFGVIRT(IMoniker, ComposeWith)
        HRESULT ( STDMETHODCALLTYPE *ComposeWith )( 
            __RPC__in IMoniker * This,
            /* [unique][in] */ __RPC__in_opt IMoniker *pmkRight,
            /* [in] */ BOOL fOnlyIfNotGeneric,
            /* [out] */ __RPC__deref_out_opt IMoniker **ppmkComposite);
        
        DECLSPEC_XFGVIRT(IMoniker, Enum)
        HRESULT ( STDMETHODCALLTYPE *Enum )( 
            __RPC__in IMoniker * This,
            /* [in] */ BOOL fForward,
            /* [out] */ __RPC__deref_out_opt IEnumMoniker **ppenumMoniker);
        
        DECLSPEC_XFGVIRT(IMoniker, IsEqual)
        HRESULT ( STDMETHODCALLTYPE *IsEqual )( 
            __RPC__in IMoniker * This,
            /* [unique][in] */ __RPC__in_opt IMoniker *pmkOtherMoniker);
        
        DECLSPEC_XFGVIRT(IMoniker, Hash)
        HRESULT ( STDMETHODCALLTYPE *Hash )( 
            __RPC__in IMoniker * This,
            /* [out] */ __RPC__out DWORD *pdwHash);
        
        DECLSPEC_XFGVIRT(IMoniker, IsRunning)
        HRESULT ( STDMETHODCALLTYPE *IsRunning )( 
            __RPC__in IMoniker * This,
            /* [unique][in] */ __RPC__in_opt IBindCtx *pbc,
            /* [unique][in] */ __RPC__in_opt IMoniker *pmkToLeft,
            /* [unique][in] */ __RPC__in_opt IMoniker *pmkNewlyRunning);
        
        DECLSPEC_XFGVIRT(IMoniker, GetTimeOfLastChange)
        HRESULT ( STDMETHODCALLTYPE *GetTimeOfLastChange )( 
            __RPC__in IMoniker * This,
            /* [unique][in] */ __RPC__in_opt IBindCtx *pbc,
            /* [unique][in] */ __RPC__in_opt IMoniker *pmkToLeft,
            /* [out] */ __RPC__out FILETIME *pFileTime);
        
        DECLSPEC_XFGVIRT(IMoniker, Inverse)
        HRESULT ( STDMETHODCALLTYPE *Inverse )( 
            __RPC__in IMoniker * This,
            /* [out] */ __RPC__deref_out_opt IMoniker **ppmk);
        
        DECLSPEC_XFGVIRT(IMoniker, CommonPrefixWith)
        HRESULT ( STDMETHODCALLTYPE *CommonPrefixWith )( 
            __RPC__in IMoniker * This,
            /* [unique][in] */ __RPC__in_opt IMoniker *pmkOther,
            /* [out] */ __RPC__deref_out_opt IMoniker **ppmkPrefix);
        
        DECLSPEC_XFGVIRT(IMoniker, RelativePathTo)
        HRESULT ( STDMETHODCALLTYPE *RelativePathTo )( 
            __RPC__in IMoniker * This,
            /* [unique][in] */ __RPC__in_opt IMoniker *pmkOther,
            /* [out] */ __RPC__deref_out_opt IMoniker **ppmkRelPath);
        
        DECLSPEC_XFGVIRT(IMoniker, GetDisplayName)
        HRESULT ( STDMETHODCALLTYPE *GetDisplayName )( 
            __RPC__in IMoniker * This,
            /* [unique][in] */ __RPC__in_opt IBindCtx *pbc,
            /* [unique][in] */ __RPC__in_opt IMoniker *pmkToLeft,
            /* [out] */ __RPC__deref_out_opt LPOLESTR *ppszDisplayName);
        
        DECLSPEC_XFGVIRT(IMoniker, ParseDisplayName)
        HRESULT ( STDMETHODCALLTYPE *ParseDisplayName )( 
            __RPC__in IMoniker * This,
            /* [unique][in] */ __RPC__in_opt IBindCtx *pbc,
            /* [unique][in] */ __RPC__in_opt IMoniker *pmkToLeft,
            /* [in] */ __RPC__in LPOLESTR pszDisplayName,
            /* [out] */ __RPC__out ULONG *pchEaten,
            /* [out] */ __RPC__deref_out_opt IMoniker **ppmkOut);
        
        DECLSPEC_XFGVIRT(IMoniker, IsSystemMoniker)
        HRESULT ( STDMETHODCALLTYPE *IsSystemMoniker )( 
            __RPC__in IMoniker * This,
            /* [out] */ __RPC__out DWORD *pdwMksys);
        
        END_INTERFACE
    } IMonikerVtbl;

    interface IMoniker
    {
        CONST_VTBL struct IMonikerVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IMoniker_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IMoniker_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IMoniker_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IMoniker_GetClassID(This,pClassID)	\
    ( (This)->lpVtbl -> GetClassID(This,pClassID) ) 


#define IMoniker_IsDirty(This)	\
    ( (This)->lpVtbl -> IsDirty(This) ) 

#define IMoniker_Load(This,pStm)	\
    ( (This)->lpVtbl -> Load(This,pStm) ) 

#define IMoniker_Save(This,pStm,fClearDirty)	\
    ( (This)->lpVtbl -> Save(This,pStm,fClearDirty) ) 

#define IMoniker_GetSizeMax(This,pcbSize)	\
    ( (This)->lpVtbl -> GetSizeMax(This,pcbSize) ) 


#define IMoniker_BindToObject(This,pbc,pmkToLeft,riidResult,ppvResult)	\
    ( (This)->lpVtbl -> BindToObject(This,pbc,pmkToLeft,riidResult,ppvResult) ) 

#define IMoniker_BindToStorage(This,pbc,pmkToLeft,riid,ppvObj)	\
    ( (This)->lpVtbl -> BindToStorage(This,pbc,pmkToLeft,riid,ppvObj) ) 

#define IMoniker_Reduce(This,pbc,dwReduceHowFar,ppmkToLeft,ppmkReduced)	\
    ( (This)->lpVtbl -> Reduce(This,pbc,dwReduceHowFar,ppmkToLeft,ppmkReduced) ) 

#define IMoniker_ComposeWith(This,pmkRight,fOnlyIfNotGeneric,ppmkComposite)	\
    ( (This)->lpVtbl -> ComposeWith(This,pmkRight,fOnlyIfNotGeneric,ppmkComposite) ) 

#define IMoniker_Enum(This,fForward,ppenumMoniker)	\
    ( (This)->lpVtbl -> Enum(This,fForward,ppenumMoniker) ) 

#define IMoniker_IsEqual(This,pmkOtherMoniker)	\
    ( (This)->lpVtbl -> IsEqual(This,pmkOtherMoniker) ) 

#define IMoniker_Hash(This,pdwHash)	\
    ( (This)->lpVtbl -> Hash(This,pdwHash) ) 

#define IMoniker_IsRunning(This,pbc,pmkToLeft,pmkNewlyRunning)	\
    ( (This)->lpVtbl -> IsRunning(This,pbc,pmkToLeft,pmkNewlyRunning) ) 

#define IMoniker_GetTimeOfLastChange(This,pbc,pmkToLeft,pFileTime)	\
    ( (This)->lpVtbl -> GetTimeOfLastChange(This,pbc,pmkToLeft,pFileTime) ) 

#define IMoniker_Inverse(This,ppmk)	\
    ( (This)->lpVtbl -> Inverse(This,ppmk) ) 

#define IMoniker_CommonPrefixWith(This,pmkOther,ppmkPrefix)	\
    ( (This)->lpVtbl -> CommonPrefixWith(This,pmkOther,ppmkPrefix) ) 

#define IMoniker_RelativePathTo(This,pmkOther,ppmkRelPath)	\
    ( (This)->lpVtbl -> RelativePathTo(This,pmkOther,ppmkRelPath) ) 

#define IMoniker_GetDisplayName(This,pbc,pmkToLeft,ppszDisplayName)	\
    ( (This)->lpVtbl -> GetDisplayName(This,pbc,pmkToLeft,ppszDisplayName) ) 

#define IMoniker_ParseDisplayName(This,pbc,pmkToLeft,pszDisplayName,pchEaten,ppmkOut)	\
    ( (This)->lpVtbl -> ParseDisplayName(This,pbc,pmkToLeft,pszDisplayName,pchEaten,ppmkOut) ) 

#define IMoniker_IsSystemMoniker(This,pdwMksys)	\
    ( (This)->lpVtbl -> IsSystemMoniker(This,pdwMksys) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */



/* [call_as] */ HRESULT STDMETHODCALLTYPE IMoniker_RemoteBindToObject_Proxy( 
    __RPC__in IMoniker * This,
    /* [unique][in] */ __RPC__in_opt IBindCtx *pbc,
    /* [unique][in] */ __RPC__in_opt IMoniker *pmkToLeft,
    /* [in] */ __RPC__in REFIID riidResult,
    /* [iid_is][out] */ __RPC__deref_out_opt IUnknown **ppvResult);


void __RPC_STUB IMoniker_RemoteBindToObject_Stub(
    IRpcStubBuffer *This,
    IRpcChannelBuffer *_pRpcChannelBuffer,
    PRPC_MESSAGE _pRpcMessage,
    DWORD *_pdwStubPhase);


/* [call_as] */ HRESULT STDMETHODCALLTYPE IMoniker_RemoteBindToStorage_Proxy( 
    __RPC__in IMoniker * This,
    /* [unique][in] */ __RPC__in_opt IBindCtx *pbc,
    /* [unique][in] */ __RPC__in_opt IMoniker *pmkToLeft,
    /* [in] */ __RPC__in REFIID riid,
    /* [iid_is][out] */ __RPC__deref_out_opt IUnknown **ppvObj);


void __RPC_STUB IMoniker_RemoteBindToStorage_Stub(
    IRpcStubBuffer *This,
    IRpcChannelBuffer *_pRpcChannelBuffer,
    PRPC_MESSAGE _pRpcMessage,
    DWORD *_pdwStubPhase);



#endif 	/* __IMoniker_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_objidl_0000_0071 */
/* [local] */ 

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES) */
#pragma endregion
#pragma region Desktop Family or OneCore Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM)


extern RPC_IF_HANDLE __MIDL_itf_objidl_0000_0071_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_objidl_0000_0071_v0_0_s_ifspec;

#ifndef __IROTData_INTERFACE_DEFINED__
#define __IROTData_INTERFACE_DEFINED__

/* interface IROTData */
/* [unique][uuid][object] */ 


EXTERN_C const IID IID_IROTData;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("f29f6bc0-5021-11ce-aa15-00006901293f")
    IROTData : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetComparisonData( 
            /* [size_is][out] */ __RPC__out_ecount_full(cbMax) byte *pbData,
            /* [in] */ ULONG cbMax,
            /* [out] */ __RPC__out ULONG *pcbData) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IROTDataVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IROTData * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IROTData * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IROTData * This);
        
        DECLSPEC_XFGVIRT(IROTData, GetComparisonData)
        HRESULT ( STDMETHODCALLTYPE *GetComparisonData )( 
            __RPC__in IROTData * This,
            /* [size_is][out] */ __RPC__out_ecount_full(cbMax) byte *pbData,
            /* [in] */ ULONG cbMax,
            /* [out] */ __RPC__out ULONG *pcbData);
        
        END_INTERFACE
    } IROTDataVtbl;

    interface IROTData
    {
        CONST_VTBL struct IROTDataVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IROTData_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IROTData_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IROTData_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IROTData_GetComparisonData(This,pbData,cbMax,pcbData)	\
    ( (This)->lpVtbl -> GetComparisonData(This,pbData,cbMax,pcbData) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IROTData_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_objidl_0000_0072 */
/* [local] */ 

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM) */
#pragma endregion
#pragma region Application Family or OneCore Family or Games Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES)


extern RPC_IF_HANDLE __MIDL_itf_objidl_0000_0072_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_objidl_0000_0072_v0_0_s_ifspec;

#ifndef __IEnumSTATSTG_INTERFACE_DEFINED__
#define __IEnumSTATSTG_INTERFACE_DEFINED__

/* interface IEnumSTATSTG */
/* [unique][uuid][object] */ 

typedef /* [unique] */  __RPC_unique_pointer IEnumSTATSTG *LPENUMSTATSTG;


EXTERN_C const IID IID_IEnumSTATSTG;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("0000000d-0000-0000-C000-000000000046")
    IEnumSTATSTG : public IUnknown
    {
    public:
        virtual /* [local] */ HRESULT STDMETHODCALLTYPE Next( 
            /* [in] */ ULONG celt,
            /* [annotation] */ 
            _Out_writes_to_(celt,*pceltFetched)  STATSTG *rgelt,
            /* [annotation] */ 
            _Out_opt_  ULONG *pceltFetched) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Skip( 
            /* [in] */ ULONG celt) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Reset( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Clone( 
            /* [out] */ __RPC__deref_out_opt IEnumSTATSTG **ppenum) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IEnumSTATSTGVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IEnumSTATSTG * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IEnumSTATSTG * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IEnumSTATSTG * This);
        
        DECLSPEC_XFGVIRT(IEnumSTATSTG, Next)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Next )( 
            IEnumSTATSTG * This,
            /* [in] */ ULONG celt,
            /* [annotation] */ 
            _Out_writes_to_(celt,*pceltFetched)  STATSTG *rgelt,
            /* [annotation] */ 
            _Out_opt_  ULONG *pceltFetched);
        
        DECLSPEC_XFGVIRT(IEnumSTATSTG, Skip)
        HRESULT ( STDMETHODCALLTYPE *Skip )( 
            __RPC__in IEnumSTATSTG * This,
            /* [in] */ ULONG celt);
        
        DECLSPEC_XFGVIRT(IEnumSTATSTG, Reset)
        HRESULT ( STDMETHODCALLTYPE *Reset )( 
            __RPC__in IEnumSTATSTG * This);
        
        DECLSPEC_XFGVIRT(IEnumSTATSTG, Clone)
        HRESULT ( STDMETHODCALLTYPE *Clone )( 
            __RPC__in IEnumSTATSTG * This,
            /* [out] */ __RPC__deref_out_opt IEnumSTATSTG **ppenum);
        
        END_INTERFACE
    } IEnumSTATSTGVtbl;

    interface IEnumSTATSTG
    {
        CONST_VTBL struct IEnumSTATSTGVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IEnumSTATSTG_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IEnumSTATSTG_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IEnumSTATSTG_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IEnumSTATSTG_Next(This,celt,rgelt,pceltFetched)	\
    ( (This)->lpVtbl -> Next(This,celt,rgelt,pceltFetched) ) 

#define IEnumSTATSTG_Skip(This,celt)	\
    ( (This)->lpVtbl -> Skip(This,celt) ) 

#define IEnumSTATSTG_Reset(This)	\
    ( (This)->lpVtbl -> Reset(This) ) 

#define IEnumSTATSTG_Clone(This,ppenum)	\
    ( (This)->lpVtbl -> Clone(This,ppenum) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */



/* [call_as] */ HRESULT STDMETHODCALLTYPE IEnumSTATSTG_RemoteNext_Proxy( 
    __RPC__in IEnumSTATSTG * This,
    /* [in] */ ULONG celt,
    /* [length_is][size_is][out] */ __RPC__out_ecount_part(celt, *pceltFetched) STATSTG *rgelt,
    /* [out] */ __RPC__out ULONG *pceltFetched);


void __RPC_STUB IEnumSTATSTG_RemoteNext_Stub(
    IRpcStubBuffer *This,
    IRpcChannelBuffer *_pRpcChannelBuffer,
    PRPC_MESSAGE _pRpcMessage,
    DWORD *_pdwStubPhase);



#endif 	/* __IEnumSTATSTG_INTERFACE_DEFINED__ */


#ifndef __IStorage_INTERFACE_DEFINED__
#define __IStorage_INTERFACE_DEFINED__

/* interface IStorage */
/* [unique][uuid][object] */ 

typedef /* [unique] */  __RPC_unique_pointer IStorage *LPSTORAGE;

typedef struct tagRemSNB
    {
    ULONG ulCntStr;
    ULONG ulCntChar;
    /* [size_is] */ OLECHAR rgString[ 1 ];
    } 	RemSNB;

typedef /* [unique] */  __RPC_unique_pointer RemSNB *wireSNB;


typedef /* [annotation][wire_marshal] */ _Null_terminated_  LPOLESTR *SNB;



EXTERN_C const IID IID_IStorage;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("0000000b-0000-0000-C000-000000000046")
    IStorage : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE CreateStream( 
            /* [string][in] */ __RPC__in_string const OLECHAR *pwcsName,
            /* [in] */ DWORD grfMode,
            /* [in] */ DWORD reserved1,
            /* [in] */ DWORD reserved2,
            /* [out] */ __RPC__deref_out_opt IStream **ppstm) = 0;
        
        virtual /* [local] */ HRESULT STDMETHODCALLTYPE OpenStream( 
            /* [annotation][string][in] */ 
            _In_z_  const OLECHAR *pwcsName,
            /* [annotation][unique][in] */ 
            _Reserved_  void *reserved1,
            /* [in] */ DWORD grfMode,
            /* [in] */ DWORD reserved2,
            /* [annotation][out] */ 
            _Outptr_  IStream **ppstm) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE CreateStorage( 
            /* [string][in] */ __RPC__in_string const OLECHAR *pwcsName,
            /* [in] */ DWORD grfMode,
            /* [in] */ DWORD reserved1,
            /* [in] */ DWORD reserved2,
            /* [out] */ __RPC__deref_out_opt IStorage **ppstg) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE OpenStorage( 
            /* [string][unique][in] */ __RPC__in_opt_string const OLECHAR *pwcsName,
            /* [unique][in] */ __RPC__in_opt IStorage *pstgPriority,
            /* [in] */ DWORD grfMode,
            /* [unique][in] */ __RPC__deref_opt_in_opt SNB snbExclude,
            /* [in] */ DWORD reserved,
            /* [out] */ __RPC__deref_out_opt IStorage **ppstg) = 0;
        
        virtual /* [local] */ HRESULT STDMETHODCALLTYPE CopyTo( 
            /* [in] */ DWORD ciidExclude,
            /* [annotation][size_is][unique][in] */ 
            _In_reads_opt_(ciidExclude)  const IID *rgiidExclude,
            /* [annotation][unique][in] */ 
            _In_opt_  SNB snbExclude,
            /* [annotation][unique][in] */ 
            _In_  IStorage *pstgDest) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE MoveElementTo( 
            /* [string][in] */ __RPC__in_string const OLECHAR *pwcsName,
            /* [unique][in] */ __RPC__in_opt IStorage *pstgDest,
            /* [string][in] */ __RPC__in_string const OLECHAR *pwcsNewName,
            /* [in] */ DWORD grfFlags) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Commit( 
            /* [in] */ DWORD grfCommitFlags) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Revert( void) = 0;
        
        virtual /* [local] */ HRESULT STDMETHODCALLTYPE EnumElements( 
            /* [annotation][in] */ 
            _Reserved_  DWORD reserved1,
            /* [annotation][size_is][unique][in] */ 
            _Reserved_  void *reserved2,
            /* [annotation][in] */ 
            _Reserved_  DWORD reserved3,
            /* [annotation][out] */ 
            _Outptr_  IEnumSTATSTG **ppenum) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE DestroyElement( 
            /* [string][in] */ __RPC__in_string const OLECHAR *pwcsName) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE RenameElement( 
            /* [string][in] */ __RPC__in_string const OLECHAR *pwcsOldName,
            /* [string][in] */ __RPC__in_string const OLECHAR *pwcsNewName) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetElementTimes( 
            /* [string][unique][in] */ __RPC__in_opt_string const OLECHAR *pwcsName,
            /* [unique][in] */ __RPC__in_opt const FILETIME *pctime,
            /* [unique][in] */ __RPC__in_opt const FILETIME *patime,
            /* [unique][in] */ __RPC__in_opt const FILETIME *pmtime) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetClass( 
            /* [in] */ __RPC__in REFCLSID clsid) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetStateBits( 
            /* [in] */ DWORD grfStateBits,
            /* [in] */ DWORD grfMask) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Stat( 
            /* [out] */ __RPC__out STATSTG *pstatstg,
            /* [in] */ DWORD grfStatFlag) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IStorageVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IStorage * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IStorage * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IStorage * This);
        
        DECLSPEC_XFGVIRT(IStorage, CreateStream)
        HRESULT ( STDMETHODCALLTYPE *CreateStream )( 
            __RPC__in IStorage * This,
            /* [string][in] */ __RPC__in_string const OLECHAR *pwcsName,
            /* [in] */ DWORD grfMode,
            /* [in] */ DWORD reserved1,
            /* [in] */ DWORD reserved2,
            /* [out] */ __RPC__deref_out_opt IStream **ppstm);
        
        DECLSPEC_XFGVIRT(IStorage, OpenStream)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *OpenStream )( 
            IStorage * This,
            /* [annotation][string][in] */ 
            _In_z_  const OLECHAR *pwcsName,
            /* [annotation][unique][in] */ 
            _Reserved_  void *reserved1,
            /* [in] */ DWORD grfMode,
            /* [in] */ DWORD reserved2,
            /* [annotation][out] */ 
            _Outptr_  IStream **ppstm);
        
        DECLSPEC_XFGVIRT(IStorage, CreateStorage)
        HRESULT ( STDMETHODCALLTYPE *CreateStorage )( 
            __RPC__in IStorage * This,
            /* [string][in] */ __RPC__in_string const OLECHAR *pwcsName,
            /* [in] */ DWORD grfMode,
            /* [in] */ DWORD reserved1,
            /* [in] */ DWORD reserved2,
            /* [out] */ __RPC__deref_out_opt IStorage **ppstg);
        
        DECLSPEC_XFGVIRT(IStorage, OpenStorage)
        HRESULT ( STDMETHODCALLTYPE *OpenStorage )( 
            __RPC__in IStorage * This,
            /* [string][unique][in] */ __RPC__in_opt_string const OLECHAR *pwcsName,
            /* [unique][in] */ __RPC__in_opt IStorage *pstgPriority,
            /* [in] */ DWORD grfMode,
            /* [unique][in] */ __RPC__deref_opt_in_opt SNB snbExclude,
            /* [in] */ DWORD reserved,
            /* [out] */ __RPC__deref_out_opt IStorage **ppstg);
        
        DECLSPEC_XFGVIRT(IStorage, CopyTo)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *CopyTo )( 
            IStorage * This,
            /* [in] */ DWORD ciidExclude,
            /* [annotation][size_is][unique][in] */ 
            _In_reads_opt_(ciidExclude)  const IID *rgiidExclude,
            /* [annotation][unique][in] */ 
            _In_opt_  SNB snbExclude,
            /* [annotation][unique][in] */ 
            _In_  IStorage *pstgDest);
        
        DECLSPEC_XFGVIRT(IStorage, MoveElementTo)
        HRESULT ( STDMETHODCALLTYPE *MoveElementTo )( 
            __RPC__in IStorage * This,
            /* [string][in] */ __RPC__in_string const OLECHAR *pwcsName,
            /* [unique][in] */ __RPC__in_opt IStorage *pstgDest,
            /* [string][in] */ __RPC__in_string const OLECHAR *pwcsNewName,
            /* [in] */ DWORD grfFlags);
        
        DECLSPEC_XFGVIRT(IStorage, Commit)
        HRESULT ( STDMETHODCALLTYPE *Commit )( 
            __RPC__in IStorage * This,
            /* [in] */ DWORD grfCommitFlags);
        
        DECLSPEC_XFGVIRT(IStorage, Revert)
        HRESULT ( STDMETHODCALLTYPE *Revert )( 
            __RPC__in IStorage * This);
        
        DECLSPEC_XFGVIRT(IStorage, EnumElements)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *EnumElements )( 
            IStorage * This,
            /* [annotation][in] */ 
            _Reserved_  DWORD reserved1,
            /* [annotation][size_is][unique][in] */ 
            _Reserved_  void *reserved2,
            /* [annotation][in] */ 
            _Reserved_  DWORD reserved3,
            /* [annotation][out] */ 
            _Outptr_  IEnumSTATSTG **ppenum);
        
        DECLSPEC_XFGVIRT(IStorage, DestroyElement)
        HRESULT ( STDMETHODCALLTYPE *DestroyElement )( 
            __RPC__in IStorage * This,
            /* [string][in] */ __RPC__in_string const OLECHAR *pwcsName);
        
        DECLSPEC_XFGVIRT(IStorage, RenameElement)
        HRESULT ( STDMETHODCALLTYPE *RenameElement )( 
            __RPC__in IStorage * This,
            /* [string][in] */ __RPC__in_string const OLECHAR *pwcsOldName,
            /* [string][in] */ __RPC__in_string const OLECHAR *pwcsNewName);
        
        DECLSPEC_XFGVIRT(IStorage, SetElementTimes)
        HRESULT ( STDMETHODCALLTYPE *SetElementTimes )( 
            __RPC__in IStorage * This,
            /* [string][unique][in] */ __RPC__in_opt_string const OLECHAR *pwcsName,
            /* [unique][in] */ __RPC__in_opt const FILETIME *pctime,
            /* [unique][in] */ __RPC__in_opt const FILETIME *patime,
            /* [unique][in] */ __RPC__in_opt const FILETIME *pmtime);
        
        DECLSPEC_XFGVIRT(IStorage, SetClass)
        HRESULT ( STDMETHODCALLTYPE *SetClass )( 
            __RPC__in IStorage * This,
            /* [in] */ __RPC__in REFCLSID clsid);
        
        DECLSPEC_XFGVIRT(IStorage, SetStateBits)
        HRESULT ( STDMETHODCALLTYPE *SetStateBits )( 
            __RPC__in IStorage * This,
            /* [in] */ DWORD grfStateBits,
            /* [in] */ DWORD grfMask);
        
        DECLSPEC_XFGVIRT(IStorage, Stat)
        HRESULT ( STDMETHODCALLTYPE *Stat )( 
            __RPC__in IStorage * This,
            /* [out] */ __RPC__out STATSTG *pstatstg,
            /* [in] */ DWORD grfStatFlag);
        
        END_INTERFACE
    } IStorageVtbl;

    interface IStorage
    {
        CONST_VTBL struct IStorageVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IStorage_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IStorage_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IStorage_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IStorage_CreateStream(This,pwcsName,grfMode,reserved1,reserved2,ppstm)	\
    ( (This)->lpVtbl -> CreateStream(This,pwcsName,grfMode,reserved1,reserved2,ppstm) ) 

#define IStorage_OpenStream(This,pwcsName,reserved1,grfMode,reserved2,ppstm)	\
    ( (This)->lpVtbl -> OpenStream(This,pwcsName,reserved1,grfMode,reserved2,ppstm) ) 

#define IStorage_CreateStorage(This,pwcsName,grfMode,reserved1,reserved2,ppstg)	\
    ( (This)->lpVtbl -> CreateStorage(This,pwcsName,grfMode,reserved1,reserved2,ppstg) ) 

#define IStorage_OpenStorage(This,pwcsName,pstgPriority,grfMode,snbExclude,reserved,ppstg)	\
    ( (This)->lpVtbl -> OpenStorage(This,pwcsName,pstgPriority,grfMode,snbExclude,reserved,ppstg) ) 

#define IStorage_CopyTo(This,ciidExclude,rgiidExclude,snbExclude,pstgDest)	\
    ( (This)->lpVtbl -> CopyTo(This,ciidExclude,rgiidExclude,snbExclude,pstgDest) ) 

#define IStorage_MoveElementTo(This,pwcsName,pstgDest,pwcsNewName,grfFlags)	\
    ( (This)->lpVtbl -> MoveElementTo(This,pwcsName,pstgDest,pwcsNewName,grfFlags) ) 

#define IStorage_Commit(This,grfCommitFlags)	\
    ( (This)->lpVtbl -> Commit(This,grfCommitFlags) ) 

#define IStorage_Revert(This)	\
    ( (This)->lpVtbl -> Revert(This) ) 

#define IStorage_EnumElements(This,reserved1,reserved2,reserved3,ppenum)	\
    ( (This)->lpVtbl -> EnumElements(This,reserved1,reserved2,reserved3,ppenum) ) 

#define IStorage_DestroyElement(This,pwcsName)	\
    ( (This)->lpVtbl -> DestroyElement(This,pwcsName) ) 

#define IStorage_RenameElement(This,pwcsOldName,pwcsNewName)	\
    ( (This)->lpVtbl -> RenameElement(This,pwcsOldName,pwcsNewName) ) 

#define IStorage_SetElementTimes(This,pwcsName,pctime,patime,pmtime)	\
    ( (This)->lpVtbl -> SetElementTimes(This,pwcsName,pctime,patime,pmtime) ) 

#define IStorage_SetClass(This,clsid)	\
    ( (This)->lpVtbl -> SetClass(This,clsid) ) 

#define IStorage_SetStateBits(This,grfStateBits,grfMask)	\
    ( (This)->lpVtbl -> SetStateBits(This,grfStateBits,grfMask) ) 

#define IStorage_Stat(This,pstatstg,grfStatFlag)	\
    ( (This)->lpVtbl -> Stat(This,pstatstg,grfStatFlag) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */



/* [call_as] */ HRESULT STDMETHODCALLTYPE IStorage_RemoteOpenStream_Proxy( 
    __RPC__in IStorage * This,
    /* [string][in] */ __RPC__in_string const OLECHAR *pwcsName,
    /* [in] */ ULONG cbReserved1,
    /* [size_is][unique][in] */ __RPC__in_ecount_full_opt(cbReserved1) byte *reserved1,
    /* [in] */ DWORD grfMode,
    /* [in] */ DWORD reserved2,
    /* [out] */ __RPC__deref_out_opt IStream **ppstm);


void __RPC_STUB IStorage_RemoteOpenStream_Stub(
    IRpcStubBuffer *This,
    IRpcChannelBuffer *_pRpcChannelBuffer,
    PRPC_MESSAGE _pRpcMessage,
    DWORD *_pdwStubPhase);


/* [call_as] */ HRESULT STDMETHODCALLTYPE IStorage_RemoteCopyTo_Proxy( 
    __RPC__in IStorage * This,
    /* [in] */ DWORD ciidExclude,
    /* [size_is][unique][in] */ __RPC__in_ecount_full_opt(ciidExclude) const IID *rgiidExclude,
    /* [unique][in] */ __RPC__deref_opt_in_opt SNB snbExclude,
    /* [unique][in] */ __RPC__in_opt IStorage *pstgDest);


void __RPC_STUB IStorage_RemoteCopyTo_Stub(
    IRpcStubBuffer *This,
    IRpcChannelBuffer *_pRpcChannelBuffer,
    PRPC_MESSAGE _pRpcMessage,
    DWORD *_pdwStubPhase);


/* [call_as] */ HRESULT STDMETHODCALLTYPE IStorage_RemoteEnumElements_Proxy( 
    __RPC__in IStorage * This,
    /* [in] */ DWORD reserved1,
    /* [in] */ ULONG cbReserved2,
    /* [size_is][unique][in] */ __RPC__in_ecount_full_opt(cbReserved2) byte *reserved2,
    /* [in] */ DWORD reserved3,
    /* [out] */ __RPC__deref_out_opt IEnumSTATSTG **ppenum);


void __RPC_STUB IStorage_RemoteEnumElements_Stub(
    IRpcStubBuffer *This,
    IRpcChannelBuffer *_pRpcChannelBuffer,
    PRPC_MESSAGE _pRpcMessage,
    DWORD *_pdwStubPhase);



#endif 	/* __IStorage_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_objidl_0000_0074 */
/* [local] */ 

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES) */
#pragma endregion
#pragma region Desktop Family or OneCore Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM)


extern RPC_IF_HANDLE __MIDL_itf_objidl_0000_0074_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_objidl_0000_0074_v0_0_s_ifspec;

#ifndef __IPersistFile_INTERFACE_DEFINED__
#define __IPersistFile_INTERFACE_DEFINED__

/* interface IPersistFile */
/* [unique][uuid][object] */ 

typedef /* [unique] */  __RPC_unique_pointer IPersistFile *LPPERSISTFILE;


EXTERN_C const IID IID_IPersistFile;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("0000010b-0000-0000-C000-000000000046")
    IPersistFile : public IPersist
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE IsDirty( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Load( 
            /* [in] */ __RPC__in LPCOLESTR pszFileName,
            /* [in] */ DWORD dwMode) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Save( 
            /* [unique][in] */ __RPC__in_opt LPCOLESTR pszFileName,
            /* [in] */ BOOL fRemember) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SaveCompleted( 
            /* [unique][in] */ __RPC__in_opt LPCOLESTR pszFileName) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetCurFile( 
            /* [out] */ __RPC__deref_out_opt LPOLESTR *ppszFileName) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IPersistFileVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IPersistFile * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IPersistFile * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IPersistFile * This);
        
        DECLSPEC_XFGVIRT(IPersist, GetClassID)
        HRESULT ( STDMETHODCALLTYPE *GetClassID )( 
            __RPC__in IPersistFile * This,
            /* [out] */ __RPC__out CLSID *pClassID);
        
        DECLSPEC_XFGVIRT(IPersistFile, IsDirty)
        HRESULT ( STDMETHODCALLTYPE *IsDirty )( 
            __RPC__in IPersistFile * This);
        
        DECLSPEC_XFGVIRT(IPersistFile, Load)
        HRESULT ( STDMETHODCALLTYPE *Load )( 
            __RPC__in IPersistFile * This,
            /* [in] */ __RPC__in LPCOLESTR pszFileName,
            /* [in] */ DWORD dwMode);
        
        DECLSPEC_XFGVIRT(IPersistFile, Save)
        HRESULT ( STDMETHODCALLTYPE *Save )( 
            __RPC__in IPersistFile * This,
            /* [unique][in] */ __RPC__in_opt LPCOLESTR pszFileName,
            /* [in] */ BOOL fRemember);
        
        DECLSPEC_XFGVIRT(IPersistFile, SaveCompleted)
        HRESULT ( STDMETHODCALLTYPE *SaveCompleted )( 
            __RPC__in IPersistFile * This,
            /* [unique][in] */ __RPC__in_opt LPCOLESTR pszFileName);
        
        DECLSPEC_XFGVIRT(IPersistFile, GetCurFile)
        HRESULT ( STDMETHODCALLTYPE *GetCurFile )( 
            __RPC__in IPersistFile * This,
            /* [out] */ __RPC__deref_out_opt LPOLESTR *ppszFileName);
        
        END_INTERFACE
    } IPersistFileVtbl;

    interface IPersistFile
    {
        CONST_VTBL struct IPersistFileVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IPersistFile_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IPersistFile_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IPersistFile_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IPersistFile_GetClassID(This,pClassID)	\
    ( (This)->lpVtbl -> GetClassID(This,pClassID) ) 


#define IPersistFile_IsDirty(This)	\
    ( (This)->lpVtbl -> IsDirty(This) ) 

#define IPersistFile_Load(This,pszFileName,dwMode)	\
    ( (This)->lpVtbl -> Load(This,pszFileName,dwMode) ) 

#define IPersistFile_Save(This,pszFileName,fRemember)	\
    ( (This)->lpVtbl -> Save(This,pszFileName,fRemember) ) 

#define IPersistFile_SaveCompleted(This,pszFileName)	\
    ( (This)->lpVtbl -> SaveCompleted(This,pszFileName) ) 

#define IPersistFile_GetCurFile(This,ppszFileName)	\
    ( (This)->lpVtbl -> GetCurFile(This,ppszFileName) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IPersistFile_INTERFACE_DEFINED__ */


#ifndef __IPersistStorage_INTERFACE_DEFINED__
#define __IPersistStorage_INTERFACE_DEFINED__

/* interface IPersistStorage */
/* [unique][uuid][object] */ 

typedef /* [unique] */  __RPC_unique_pointer IPersistStorage *LPPERSISTSTORAGE;


EXTERN_C const IID IID_IPersistStorage;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("0000010a-0000-0000-C000-000000000046")
    IPersistStorage : public IPersist
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE IsDirty( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE InitNew( 
            /* [unique][in] */ __RPC__in_opt IStorage *pStg) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Load( 
            /* [unique][in] */ __RPC__in_opt IStorage *pStg) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Save( 
            /* [unique][in] */ __RPC__in_opt IStorage *pStgSave,
            /* [in] */ BOOL fSameAsLoad) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SaveCompleted( 
            /* [unique][in] */ __RPC__in_opt IStorage *pStgNew) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE HandsOffStorage( void) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IPersistStorageVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IPersistStorage * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IPersistStorage * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IPersistStorage * This);
        
        DECLSPEC_XFGVIRT(IPersist, GetClassID)
        HRESULT ( STDMETHODCALLTYPE *GetClassID )( 
            __RPC__in IPersistStorage * This,
            /* [out] */ __RPC__out CLSID *pClassID);
        
        DECLSPEC_XFGVIRT(IPersistStorage, IsDirty)
        HRESULT ( STDMETHODCALLTYPE *IsDirty )( 
            __RPC__in IPersistStorage * This);
        
        DECLSPEC_XFGVIRT(IPersistStorage, InitNew)
        HRESULT ( STDMETHODCALLTYPE *InitNew )( 
            __RPC__in IPersistStorage * This,
            /* [unique][in] */ __RPC__in_opt IStorage *pStg);
        
        DECLSPEC_XFGVIRT(IPersistStorage, Load)
        HRESULT ( STDMETHODCALLTYPE *Load )( 
            __RPC__in IPersistStorage * This,
            /* [unique][in] */ __RPC__in_opt IStorage *pStg);
        
        DECLSPEC_XFGVIRT(IPersistStorage, Save)
        HRESULT ( STDMETHODCALLTYPE *Save )( 
            __RPC__in IPersistStorage * This,
            /* [unique][in] */ __RPC__in_opt IStorage *pStgSave,
            /* [in] */ BOOL fSameAsLoad);
        
        DECLSPEC_XFGVIRT(IPersistStorage, SaveCompleted)
        HRESULT ( STDMETHODCALLTYPE *SaveCompleted )( 
            __RPC__in IPersistStorage * This,
            /* [unique][in] */ __RPC__in_opt IStorage *pStgNew);
        
        DECLSPEC_XFGVIRT(IPersistStorage, HandsOffStorage)
        HRESULT ( STDMETHODCALLTYPE *HandsOffStorage )( 
            __RPC__in IPersistStorage * This);
        
        END_INTERFACE
    } IPersistStorageVtbl;

    interface IPersistStorage
    {
        CONST_VTBL struct IPersistStorageVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IPersistStorage_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IPersistStorage_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IPersistStorage_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IPersistStorage_GetClassID(This,pClassID)	\
    ( (This)->lpVtbl -> GetClassID(This,pClassID) ) 


#define IPersistStorage_IsDirty(This)	\
    ( (This)->lpVtbl -> IsDirty(This) ) 

#define IPersistStorage_InitNew(This,pStg)	\
    ( (This)->lpVtbl -> InitNew(This,pStg) ) 

#define IPersistStorage_Load(This,pStg)	\
    ( (This)->lpVtbl -> Load(This,pStg) ) 

#define IPersistStorage_Save(This,pStgSave,fSameAsLoad)	\
    ( (This)->lpVtbl -> Save(This,pStgSave,fSameAsLoad) ) 

#define IPersistStorage_SaveCompleted(This,pStgNew)	\
    ( (This)->lpVtbl -> SaveCompleted(This,pStgNew) ) 

#define IPersistStorage_HandsOffStorage(This)	\
    ( (This)->lpVtbl -> HandsOffStorage(This) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IPersistStorage_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_objidl_0000_0076 */
/* [local] */ 

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM) */
#pragma endregion
#pragma region Application Family or OneCore Family or Games Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES)


extern RPC_IF_HANDLE __MIDL_itf_objidl_0000_0076_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_objidl_0000_0076_v0_0_s_ifspec;

#ifndef __ILockBytes_INTERFACE_DEFINED__
#define __ILockBytes_INTERFACE_DEFINED__

/* interface ILockBytes */
/* [unique][uuid][object] */ 

typedef /* [unique] */  __RPC_unique_pointer ILockBytes *LPLOCKBYTES;


EXTERN_C const IID IID_ILockBytes;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("0000000a-0000-0000-C000-000000000046")
    ILockBytes : public IUnknown
    {
    public:
        virtual /* [local] */ HRESULT STDMETHODCALLTYPE ReadAt( 
            /* [in] */ ULARGE_INTEGER ulOffset,
            /* [annotation][length_is][size_is][out] */ 
            _Out_writes_bytes_to_(cb, *pcbRead)  void *pv,
            /* [in] */ ULONG cb,
            /* [annotation][out] */ 
            _Out_opt_  ULONG *pcbRead) = 0;
        
        virtual /* [local] */ HRESULT STDMETHODCALLTYPE WriteAt( 
            /* [in] */ ULARGE_INTEGER ulOffset,
            /* [annotation][size_is][in] */ 
            _In_reads_bytes_(cb)  const void *pv,
            /* [in] */ ULONG cb,
            /* [annotation][out] */ 
            _Out_opt_  ULONG *pcbWritten) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Flush( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetSize( 
            /* [in] */ ULARGE_INTEGER cb) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE LockRegion( 
            /* [in] */ ULARGE_INTEGER libOffset,
            /* [in] */ ULARGE_INTEGER cb,
            /* [in] */ DWORD dwLockType) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE UnlockRegion( 
            /* [in] */ ULARGE_INTEGER libOffset,
            /* [in] */ ULARGE_INTEGER cb,
            /* [in] */ DWORD dwLockType) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Stat( 
            /* [out] */ __RPC__out STATSTG *pstatstg,
            /* [in] */ DWORD grfStatFlag) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ILockBytesVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in ILockBytes * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in ILockBytes * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in ILockBytes * This);
        
        DECLSPEC_XFGVIRT(ILockBytes, ReadAt)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *ReadAt )( 
            ILockBytes * This,
            /* [in] */ ULARGE_INTEGER ulOffset,
            /* [annotation][length_is][size_is][out] */ 
            _Out_writes_bytes_to_(cb, *pcbRead)  void *pv,
            /* [in] */ ULONG cb,
            /* [annotation][out] */ 
            _Out_opt_  ULONG *pcbRead);
        
        DECLSPEC_XFGVIRT(ILockBytes, WriteAt)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *WriteAt )( 
            ILockBytes * This,
            /* [in] */ ULARGE_INTEGER ulOffset,
            /* [annotation][size_is][in] */ 
            _In_reads_bytes_(cb)  const void *pv,
            /* [in] */ ULONG cb,
            /* [annotation][out] */ 
            _Out_opt_  ULONG *pcbWritten);
        
        DECLSPEC_XFGVIRT(ILockBytes, Flush)
        HRESULT ( STDMETHODCALLTYPE *Flush )( 
            __RPC__in ILockBytes * This);
        
        DECLSPEC_XFGVIRT(ILockBytes, SetSize)
        HRESULT ( STDMETHODCALLTYPE *SetSize )( 
            __RPC__in ILockBytes * This,
            /* [in] */ ULARGE_INTEGER cb);
        
        DECLSPEC_XFGVIRT(ILockBytes, LockRegion)
        HRESULT ( STDMETHODCALLTYPE *LockRegion )( 
            __RPC__in ILockBytes * This,
            /* [in] */ ULARGE_INTEGER libOffset,
            /* [in] */ ULARGE_INTEGER cb,
            /* [in] */ DWORD dwLockType);
        
        DECLSPEC_XFGVIRT(ILockBytes, UnlockRegion)
        HRESULT ( STDMETHODCALLTYPE *UnlockRegion )( 
            __RPC__in ILockBytes * This,
            /* [in] */ ULARGE_INTEGER libOffset,
            /* [in] */ ULARGE_INTEGER cb,
            /* [in] */ DWORD dwLockType);
        
        DECLSPEC_XFGVIRT(ILockBytes, Stat)
        HRESULT ( STDMETHODCALLTYPE *Stat )( 
            __RPC__in ILockBytes * This,
            /* [out] */ __RPC__out STATSTG *pstatstg,
            /* [in] */ DWORD grfStatFlag);
        
        END_INTERFACE
    } ILockBytesVtbl;

    interface ILockBytes
    {
        CONST_VTBL struct ILockBytesVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ILockBytes_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ILockBytes_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ILockBytes_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ILockBytes_ReadAt(This,ulOffset,pv,cb,pcbRead)	\
    ( (This)->lpVtbl -> ReadAt(This,ulOffset,pv,cb,pcbRead) ) 

#define ILockBytes_WriteAt(This,ulOffset,pv,cb,pcbWritten)	\
    ( (This)->lpVtbl -> WriteAt(This,ulOffset,pv,cb,pcbWritten) ) 

#define ILockBytes_Flush(This)	\
    ( (This)->lpVtbl -> Flush(This) ) 

#define ILockBytes_SetSize(This,cb)	\
    ( (This)->lpVtbl -> SetSize(This,cb) ) 

#define ILockBytes_LockRegion(This,libOffset,cb,dwLockType)	\
    ( (This)->lpVtbl -> LockRegion(This,libOffset,cb,dwLockType) ) 

#define ILockBytes_UnlockRegion(This,libOffset,cb,dwLockType)	\
    ( (This)->lpVtbl -> UnlockRegion(This,libOffset,cb,dwLockType) ) 

#define ILockBytes_Stat(This,pstatstg,grfStatFlag)	\
    ( (This)->lpVtbl -> Stat(This,pstatstg,grfStatFlag) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */



/* [call_as] */ HRESULT __stdcall ILockBytes_RemoteReadAt_Proxy( 
    __RPC__in ILockBytes * This,
    /* [in] */ ULARGE_INTEGER ulOffset,
    /* [length_is][size_is][out] */ __RPC__out_ecount_part(cb, *pcbRead) byte *pv,
    /* [in] */ ULONG cb,
    /* [out] */ __RPC__out ULONG *pcbRead);


void __RPC_STUB ILockBytes_RemoteReadAt_Stub(
    IRpcStubBuffer *This,
    IRpcChannelBuffer *_pRpcChannelBuffer,
    PRPC_MESSAGE _pRpcMessage,
    DWORD *_pdwStubPhase);


/* [call_as] */ HRESULT STDMETHODCALLTYPE ILockBytes_RemoteWriteAt_Proxy( 
    __RPC__in ILockBytes * This,
    /* [in] */ ULARGE_INTEGER ulOffset,
    /* [size_is][in] */ __RPC__in_ecount_full(cb) const byte *pv,
    /* [in] */ ULONG cb,
    /* [out] */ __RPC__out ULONG *pcbWritten);


void __RPC_STUB ILockBytes_RemoteWriteAt_Stub(
    IRpcStubBuffer *This,
    IRpcChannelBuffer *_pRpcChannelBuffer,
    PRPC_MESSAGE _pRpcMessage,
    DWORD *_pdwStubPhase);



#endif 	/* __ILockBytes_INTERFACE_DEFINED__ */


#ifndef __IEnumFORMATETC_INTERFACE_DEFINED__
#define __IEnumFORMATETC_INTERFACE_DEFINED__

/* interface IEnumFORMATETC */
/* [unique][uuid][object] */ 

typedef /* [unique] */  __RPC_unique_pointer IEnumFORMATETC *LPENUMFORMATETC;


typedef struct tagDVTARGETDEVICE
    {
    DWORD tdSize;
    WORD tdDriverNameOffset;
    WORD tdDeviceNameOffset;
    WORD tdPortNameOffset;
    WORD tdExtDevmodeOffset;
    /* [size_is] */ BYTE tdData[ 1 ];
    } 	DVTARGETDEVICE;


typedef CLIPFORMAT *LPCLIPFORMAT;

typedef struct tagFORMATETC
    {
    CLIPFORMAT cfFormat;
    /* [unique] */ DVTARGETDEVICE *ptd;
    DWORD dwAspect;
    LONG lindex;
    DWORD tymed;
    } 	FORMATETC;

typedef struct tagFORMATETC *LPFORMATETC;


EXTERN_C const IID IID_IEnumFORMATETC;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("00000103-0000-0000-C000-000000000046")
    IEnumFORMATETC : public IUnknown
    {
    public:
        virtual /* [local] */ HRESULT STDMETHODCALLTYPE Next( 
            /* [in] */ ULONG celt,
            /* [annotation] */ 
            _Out_writes_to_(celt,*pceltFetched)  FORMATETC *rgelt,
            /* [annotation] */ 
            _Out_opt_  ULONG *pceltFetched) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Skip( 
            /* [in] */ ULONG celt) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Reset( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Clone( 
            /* [out] */ __RPC__deref_out_opt IEnumFORMATETC **ppenum) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IEnumFORMATETCVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IEnumFORMATETC * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IEnumFORMATETC * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IEnumFORMATETC * This);
        
        DECLSPEC_XFGVIRT(IEnumFORMATETC, Next)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Next )( 
            IEnumFORMATETC * This,
            /* [in] */ ULONG celt,
            /* [annotation] */ 
            _Out_writes_to_(celt,*pceltFetched)  FORMATETC *rgelt,
            /* [annotation] */ 
            _Out_opt_  ULONG *pceltFetched);
        
        DECLSPEC_XFGVIRT(IEnumFORMATETC, Skip)
        HRESULT ( STDMETHODCALLTYPE *Skip )( 
            __RPC__in IEnumFORMATETC * This,
            /* [in] */ ULONG celt);
        
        DECLSPEC_XFGVIRT(IEnumFORMATETC, Reset)
        HRESULT ( STDMETHODCALLTYPE *Reset )( 
            __RPC__in IEnumFORMATETC * This);
        
        DECLSPEC_XFGVIRT(IEnumFORMATETC, Clone)
        HRESULT ( STDMETHODCALLTYPE *Clone )( 
            __RPC__in IEnumFORMATETC * This,
            /* [out] */ __RPC__deref_out_opt IEnumFORMATETC **ppenum);
        
        END_INTERFACE
    } IEnumFORMATETCVtbl;

    interface IEnumFORMATETC
    {
        CONST_VTBL struct IEnumFORMATETCVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IEnumFORMATETC_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IEnumFORMATETC_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IEnumFORMATETC_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IEnumFORMATETC_Next(This,celt,rgelt,pceltFetched)	\
    ( (This)->lpVtbl -> Next(This,celt,rgelt,pceltFetched) ) 

#define IEnumFORMATETC_Skip(This,celt)	\
    ( (This)->lpVtbl -> Skip(This,celt) ) 

#define IEnumFORMATETC_Reset(This)	\
    ( (This)->lpVtbl -> Reset(This) ) 

#define IEnumFORMATETC_Clone(This,ppenum)	\
    ( (This)->lpVtbl -> Clone(This,ppenum) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */



/* [call_as] */ HRESULT STDMETHODCALLTYPE IEnumFORMATETC_RemoteNext_Proxy( 
    __RPC__in IEnumFORMATETC * This,
    /* [in] */ ULONG celt,
    /* [length_is][size_is][out] */ __RPC__out_ecount_part(celt, *pceltFetched) FORMATETC *rgelt,
    /* [out] */ __RPC__out ULONG *pceltFetched);


void __RPC_STUB IEnumFORMATETC_RemoteNext_Stub(
    IRpcStubBuffer *This,
    IRpcChannelBuffer *_pRpcChannelBuffer,
    PRPC_MESSAGE _pRpcMessage,
    DWORD *_pdwStubPhase);



#endif 	/* __IEnumFORMATETC_INTERFACE_DEFINED__ */


#ifndef __IEnumSTATDATA_INTERFACE_DEFINED__
#define __IEnumSTATDATA_INTERFACE_DEFINED__

/* interface IEnumSTATDATA */
/* [unique][uuid][object] */ 

typedef /* [unique] */  __RPC_unique_pointer IEnumSTATDATA *LPENUMSTATDATA;

typedef 
enum tagADVF
    {
        ADVF_NODATA	= 1,
        ADVF_PRIMEFIRST	= 2,
        ADVF_ONLYONCE	= 4,
        ADVF_DATAONSTOP	= 64,
        ADVFCACHE_NOHANDLER	= 8,
        ADVFCACHE_FORCEBUILTIN	= 16,
        ADVFCACHE_ONSAVE	= 32
    } 	ADVF;

typedef struct tagSTATDATA
    {
    FORMATETC formatetc;
    DWORD advf;
    /* [unique] */ IAdviseSink *pAdvSink;
    DWORD dwConnection;
    } 	STATDATA;

typedef STATDATA *LPSTATDATA;


EXTERN_C const IID IID_IEnumSTATDATA;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("00000105-0000-0000-C000-000000000046")
    IEnumSTATDATA : public IUnknown
    {
    public:
        virtual /* [local] */ HRESULT STDMETHODCALLTYPE Next( 
            /* [in] */ ULONG celt,
            /* [annotation] */ 
            _Out_writes_to_(celt,*pceltFetched)  STATDATA *rgelt,
            /* [annotation] */ 
            _Out_opt_  ULONG *pceltFetched) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Skip( 
            /* [in] */ ULONG celt) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Reset( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Clone( 
            /* [out] */ __RPC__deref_out_opt IEnumSTATDATA **ppenum) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IEnumSTATDATAVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IEnumSTATDATA * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IEnumSTATDATA * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IEnumSTATDATA * This);
        
        DECLSPEC_XFGVIRT(IEnumSTATDATA, Next)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Next )( 
            IEnumSTATDATA * This,
            /* [in] */ ULONG celt,
            /* [annotation] */ 
            _Out_writes_to_(celt,*pceltFetched)  STATDATA *rgelt,
            /* [annotation] */ 
            _Out_opt_  ULONG *pceltFetched);
        
        DECLSPEC_XFGVIRT(IEnumSTATDATA, Skip)
        HRESULT ( STDMETHODCALLTYPE *Skip )( 
            __RPC__in IEnumSTATDATA * This,
            /* [in] */ ULONG celt);
        
        DECLSPEC_XFGVIRT(IEnumSTATDATA, Reset)
        HRESULT ( STDMETHODCALLTYPE *Reset )( 
            __RPC__in IEnumSTATDATA * This);
        
        DECLSPEC_XFGVIRT(IEnumSTATDATA, Clone)
        HRESULT ( STDMETHODCALLTYPE *Clone )( 
            __RPC__in IEnumSTATDATA * This,
            /* [out] */ __RPC__deref_out_opt IEnumSTATDATA **ppenum);
        
        END_INTERFACE
    } IEnumSTATDATAVtbl;

    interface IEnumSTATDATA
    {
        CONST_VTBL struct IEnumSTATDATAVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IEnumSTATDATA_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IEnumSTATDATA_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IEnumSTATDATA_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IEnumSTATDATA_Next(This,celt,rgelt,pceltFetched)	\
    ( (This)->lpVtbl -> Next(This,celt,rgelt,pceltFetched) ) 

#define IEnumSTATDATA_Skip(This,celt)	\
    ( (This)->lpVtbl -> Skip(This,celt) ) 

#define IEnumSTATDATA_Reset(This)	\
    ( (This)->lpVtbl -> Reset(This) ) 

#define IEnumSTATDATA_Clone(This,ppenum)	\
    ( (This)->lpVtbl -> Clone(This,ppenum) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */



/* [call_as] */ HRESULT STDMETHODCALLTYPE IEnumSTATDATA_RemoteNext_Proxy( 
    __RPC__in IEnumSTATDATA * This,
    /* [in] */ ULONG celt,
    /* [length_is][size_is][out] */ __RPC__out_ecount_part(celt, *pceltFetched) STATDATA *rgelt,
    /* [out] */ __RPC__out ULONG *pceltFetched);


void __RPC_STUB IEnumSTATDATA_RemoteNext_Stub(
    IRpcStubBuffer *This,
    IRpcChannelBuffer *_pRpcChannelBuffer,
    PRPC_MESSAGE _pRpcMessage,
    DWORD *_pdwStubPhase);



#endif 	/* __IEnumSTATDATA_INTERFACE_DEFINED__ */


#ifndef __IRootStorage_INTERFACE_DEFINED__
#define __IRootStorage_INTERFACE_DEFINED__

/* interface IRootStorage */
/* [unique][uuid][object] */ 

typedef /* [unique] */  __RPC_unique_pointer IRootStorage *LPROOTSTORAGE;


EXTERN_C const IID IID_IRootStorage;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("00000012-0000-0000-C000-000000000046")
    IRootStorage : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE SwitchToFile( 
            /* [in] */ __RPC__in LPOLESTR pszFile) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IRootStorageVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IRootStorage * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IRootStorage * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IRootStorage * This);
        
        DECLSPEC_XFGVIRT(IRootStorage, SwitchToFile)
        HRESULT ( STDMETHODCALLTYPE *SwitchToFile )( 
            __RPC__in IRootStorage * This,
            /* [in] */ __RPC__in LPOLESTR pszFile);
        
        END_INTERFACE
    } IRootStorageVtbl;

    interface IRootStorage
    {
        CONST_VTBL struct IRootStorageVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IRootStorage_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IRootStorage_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IRootStorage_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IRootStorage_SwitchToFile(This,pszFile)	\
    ( (This)->lpVtbl -> SwitchToFile(This,pszFile) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IRootStorage_INTERFACE_DEFINED__ */


#ifndef __IAdviseSink_INTERFACE_DEFINED__
#define __IAdviseSink_INTERFACE_DEFINED__

/* interface IAdviseSink */
/* [unique][async_uuid][uuid][object] */ 

typedef IAdviseSink *LPADVISESINK;

typedef /* [v1_enum] */ 
enum tagTYMED
    {
        TYMED_HGLOBAL	= 1,
        TYMED_FILE	= 2,
        TYMED_ISTREAM	= 4,
        TYMED_ISTORAGE	= 8,
        TYMED_GDI	= 16,
        TYMED_MFPICT	= 32,
        TYMED_ENHMF	= 64,
        TYMED_NULL	= 0
    } 	TYMED;

#ifndef RC_INVOKED
#if _MSC_VER >= 1200
#pragma warning(push)
#endif
#pragma warning(disable:4200)
#endif
typedef struct tagRemSTGMEDIUM
    {
    DWORD tymed;
    DWORD dwHandleType;
    ULONG pData;
    ULONG pUnkForRelease;
    ULONG cbData;
    /* [size_is] */ byte data[ 1 ];
    } 	RemSTGMEDIUM;

#ifndef RC_INVOKED
#if _MSC_VER >= 1200
#pragma warning(pop)
#else
#pragma warning(default:4200)
#endif
#endif
#ifdef NONAMELESSUNION
typedef struct tagSTGMEDIUM {
    DWORD tymed;
    union {
        HBITMAP hBitmap;
        HMETAFILEPICT hMetaFilePict;
        HENHMETAFILE hEnhMetaFile;
        HGLOBAL hGlobal;
        LPOLESTR lpszFileName;
        IStream *pstm;
        IStorage *pstg;
        } u;
    IUnknown *pUnkForRelease;
}uSTGMEDIUM;
#else
typedef struct tagSTGMEDIUM
    {
    DWORD tymed;
    /* [switch_is][switch_type] */ union 
        {
        /* [case()] */ HBITMAP hBitmap;
        /* [case()] */ HMETAFILEPICT hMetaFilePict;
        /* [case()] */ HENHMETAFILE hEnhMetaFile;
        /* [case()] */ HGLOBAL hGlobal;
        /* [case()] */ LPOLESTR lpszFileName;
        /* [case()] */ IStream *pstm;
        /* [case()] */ IStorage *pstg;
        /* [default] */  /* Empty union arm */ 
        } 	DUMMYUNIONNAME;
    /* [unique] */ IUnknown *pUnkForRelease;
    } 	uSTGMEDIUM;

#endif /* !NONAMELESSUNION */
typedef struct _GDI_OBJECT
    {
    DWORD ObjectType;
    /* [switch_is] */ /* [switch_type] */ union __MIDL_IAdviseSink_0002
        {
        /* [case()] */ wireHBITMAP hBitmap;
        /* [case()] */ wireHPALETTE hPalette;
        /* [default] */ wireHGLOBAL hGeneric;
        } 	u;
    } 	GDI_OBJECT;

typedef struct _userSTGMEDIUM
    {
    struct _STGMEDIUM_UNION
        {
        DWORD tymed;
        /* [switch_is] */ /* [switch_type] */ union __MIDL_IAdviseSink_0003
            {
            /* [case()] */  /* Empty union arm */ 
            /* [case()] */ wireHMETAFILEPICT hMetaFilePict;
            /* [case()] */ wireHENHMETAFILE hHEnhMetaFile;
            /* [case()] */ GDI_OBJECT *hGdiHandle;
            /* [case()] */ wireHGLOBAL hGlobal;
            /* [case()] */ LPOLESTR lpszFileName;
            /* [case()] */ BYTE_BLOB *pstm;
            /* [case()] */ BYTE_BLOB *pstg;
            } 	u;
        } 	DUMMYUNIONNAME;
    IUnknown *pUnkForRelease;
    } 	userSTGMEDIUM;

typedef /* [unique] */  __RPC_unique_pointer userSTGMEDIUM *wireSTGMEDIUM;

typedef /* [wire_marshal] */ uSTGMEDIUM STGMEDIUM;

typedef /* [unique] */  __RPC_unique_pointer userSTGMEDIUM *wireASYNC_STGMEDIUM;

typedef /* [wire_marshal] */ STGMEDIUM ASYNC_STGMEDIUM;

typedef STGMEDIUM *LPSTGMEDIUM;

typedef struct _userFLAG_STGMEDIUM
    {
    LONG ContextFlags;
    LONG fPassOwnership;
    userSTGMEDIUM Stgmed;
    } 	userFLAG_STGMEDIUM;

typedef /* [unique] */  __RPC_unique_pointer userFLAG_STGMEDIUM *wireFLAG_STGMEDIUM;

typedef /* [wire_marshal] */ struct _FLAG_STGMEDIUM
    {
    LONG ContextFlags;
    LONG fPassOwnership;
    STGMEDIUM Stgmed;
    } 	FLAG_STGMEDIUM;


EXTERN_C const IID IID_IAdviseSink;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("0000010f-0000-0000-C000-000000000046")
    IAdviseSink : public IUnknown
    {
    public:
        virtual /* [local] */ void STDMETHODCALLTYPE OnDataChange( 
            /* [annotation][unique][in] */ 
            _In_  FORMATETC *pFormatetc,
            /* [annotation][unique][in] */ 
            _In_  STGMEDIUM *pStgmed) = 0;
        
        virtual /* [local] */ void STDMETHODCALLTYPE OnViewChange( 
            /* [in] */ DWORD dwAspect,
            /* [in] */ LONG lindex) = 0;
        
        virtual /* [local] */ void STDMETHODCALLTYPE OnRename( 
            /* [annotation][in] */ 
            _In_  IMoniker *pmk) = 0;
        
        virtual /* [local] */ void STDMETHODCALLTYPE OnSave( void) = 0;
        
        virtual /* [local] */ void STDMETHODCALLTYPE OnClose( void) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IAdviseSinkVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IAdviseSink * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IAdviseSink * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IAdviseSink * This);
        
        DECLSPEC_XFGVIRT(IAdviseSink, OnDataChange)
        /* [local] */ void ( STDMETHODCALLTYPE *OnDataChange )( 
            IAdviseSink * This,
            /* [annotation][unique][in] */ 
            _In_  FORMATETC *pFormatetc,
            /* [annotation][unique][in] */ 
            _In_  STGMEDIUM *pStgmed);
        
        DECLSPEC_XFGVIRT(IAdviseSink, OnViewChange)
        /* [local] */ void ( STDMETHODCALLTYPE *OnViewChange )( 
            IAdviseSink * This,
            /* [in] */ DWORD dwAspect,
            /* [in] */ LONG lindex);
        
        DECLSPEC_XFGVIRT(IAdviseSink, OnRename)
        /* [local] */ void ( STDMETHODCALLTYPE *OnRename )( 
            IAdviseSink * This,
            /* [annotation][in] */ 
            _In_  IMoniker *pmk);
        
        DECLSPEC_XFGVIRT(IAdviseSink, OnSave)
        /* [local] */ void ( STDMETHODCALLTYPE *OnSave )( 
            IAdviseSink * This);
        
        DECLSPEC_XFGVIRT(IAdviseSink, OnClose)
        /* [local] */ void ( STDMETHODCALLTYPE *OnClose )( 
            IAdviseSink * This);
        
        END_INTERFACE
    } IAdviseSinkVtbl;

    interface IAdviseSink
    {
        CONST_VTBL struct IAdviseSinkVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IAdviseSink_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IAdviseSink_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IAdviseSink_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IAdviseSink_OnDataChange(This,pFormatetc,pStgmed)	\
    ( (This)->lpVtbl -> OnDataChange(This,pFormatetc,pStgmed) ) 

#define IAdviseSink_OnViewChange(This,dwAspect,lindex)	\
    ( (This)->lpVtbl -> OnViewChange(This,dwAspect,lindex) ) 

#define IAdviseSink_OnRename(This,pmk)	\
    ( (This)->lpVtbl -> OnRename(This,pmk) ) 

#define IAdviseSink_OnSave(This)	\
    ( (This)->lpVtbl -> OnSave(This) ) 

#define IAdviseSink_OnClose(This)	\
    ( (This)->lpVtbl -> OnClose(This) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */



/* [call_as] */ HRESULT STDMETHODCALLTYPE IAdviseSink_RemoteOnDataChange_Proxy( 
    __RPC__in IAdviseSink * This,
    /* [unique][in] */ __RPC__in_opt FORMATETC *pFormatetc,
    /* [unique][in] */ __RPC__in_opt ASYNC_STGMEDIUM *pStgmed);


void __RPC_STUB IAdviseSink_RemoteOnDataChange_Stub(
    IRpcStubBuffer *This,
    IRpcChannelBuffer *_pRpcChannelBuffer,
    PRPC_MESSAGE _pRpcMessage,
    DWORD *_pdwStubPhase);


/* [call_as] */ HRESULT STDMETHODCALLTYPE IAdviseSink_RemoteOnViewChange_Proxy( 
    __RPC__in IAdviseSink * This,
    /* [in] */ DWORD dwAspect,
    /* [in] */ LONG lindex);


void __RPC_STUB IAdviseSink_RemoteOnViewChange_Stub(
    IRpcStubBuffer *This,
    IRpcChannelBuffer *_pRpcChannelBuffer,
    PRPC_MESSAGE _pRpcMessage,
    DWORD *_pdwStubPhase);


/* [call_as] */ HRESULT STDMETHODCALLTYPE IAdviseSink_RemoteOnRename_Proxy( 
    __RPC__in IAdviseSink * This,
    /* [in] */ __RPC__in_opt IMoniker *pmk);


void __RPC_STUB IAdviseSink_RemoteOnRename_Stub(
    IRpcStubBuffer *This,
    IRpcChannelBuffer *_pRpcChannelBuffer,
    PRPC_MESSAGE _pRpcMessage,
    DWORD *_pdwStubPhase);


/* [call_as] */ HRESULT STDMETHODCALLTYPE IAdviseSink_RemoteOnSave_Proxy( 
    __RPC__in IAdviseSink * This);


void __RPC_STUB IAdviseSink_RemoteOnSave_Stub(
    IRpcStubBuffer *This,
    IRpcChannelBuffer *_pRpcChannelBuffer,
    PRPC_MESSAGE _pRpcMessage,
    DWORD *_pdwStubPhase);


/* [call_as] */ HRESULT STDMETHODCALLTYPE IAdviseSink_RemoteOnClose_Proxy( 
    __RPC__in IAdviseSink * This);


void __RPC_STUB IAdviseSink_RemoteOnClose_Stub(
    IRpcStubBuffer *This,
    IRpcChannelBuffer *_pRpcChannelBuffer,
    PRPC_MESSAGE _pRpcMessage,
    DWORD *_pdwStubPhase);



#endif 	/* __IAdviseSink_INTERFACE_DEFINED__ */


#ifndef __AsyncIAdviseSink_INTERFACE_DEFINED__
#define __AsyncIAdviseSink_INTERFACE_DEFINED__

/* interface AsyncIAdviseSink */
/* [uuid][unique][object] */ 


EXTERN_C const IID IID_AsyncIAdviseSink;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("00000150-0000-0000-C000-000000000046")
    AsyncIAdviseSink : public IUnknown
    {
    public:
        virtual /* [local] */ void STDMETHODCALLTYPE Begin_OnDataChange( 
            /* [annotation][unique][in] */ 
            _In_  FORMATETC *pFormatetc,
            /* [annotation][unique][in] */ 
            _In_  STGMEDIUM *pStgmed) = 0;
        
        virtual /* [local] */ void STDMETHODCALLTYPE Finish_OnDataChange( void) = 0;
        
        virtual /* [local] */ void STDMETHODCALLTYPE Begin_OnViewChange( 
            /* [in] */ DWORD dwAspect,
            /* [in] */ LONG lindex) = 0;
        
        virtual /* [local] */ void STDMETHODCALLTYPE Finish_OnViewChange( void) = 0;
        
        virtual /* [local] */ void STDMETHODCALLTYPE Begin_OnRename( 
            /* [annotation][in] */ 
            _In_  IMoniker *pmk) = 0;
        
        virtual /* [local] */ void STDMETHODCALLTYPE Finish_OnRename( void) = 0;
        
        virtual /* [local] */ void STDMETHODCALLTYPE Begin_OnSave( void) = 0;
        
        virtual /* [local] */ void STDMETHODCALLTYPE Finish_OnSave( void) = 0;
        
        virtual /* [local] */ void STDMETHODCALLTYPE Begin_OnClose( void) = 0;
        
        virtual /* [local] */ void STDMETHODCALLTYPE Finish_OnClose( void) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct AsyncIAdviseSinkVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in AsyncIAdviseSink * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in AsyncIAdviseSink * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in AsyncIAdviseSink * This);
        
        DECLSPEC_XFGVIRT(AsyncIAdviseSink, Begin_OnDataChange)
        /* [local] */ void ( STDMETHODCALLTYPE *Begin_OnDataChange )( 
            AsyncIAdviseSink * This,
            /* [annotation][unique][in] */ 
            _In_  FORMATETC *pFormatetc,
            /* [annotation][unique][in] */ 
            _In_  STGMEDIUM *pStgmed);
        
        DECLSPEC_XFGVIRT(AsyncIAdviseSink, Finish_OnDataChange)
        /* [local] */ void ( STDMETHODCALLTYPE *Finish_OnDataChange )( 
            AsyncIAdviseSink * This);
        
        DECLSPEC_XFGVIRT(AsyncIAdviseSink, Begin_OnViewChange)
        /* [local] */ void ( STDMETHODCALLTYPE *Begin_OnViewChange )( 
            AsyncIAdviseSink * This,
            /* [in] */ DWORD dwAspect,
            /* [in] */ LONG lindex);
        
        DECLSPEC_XFGVIRT(AsyncIAdviseSink, Finish_OnViewChange)
        /* [local] */ void ( STDMETHODCALLTYPE *Finish_OnViewChange )( 
            AsyncIAdviseSink * This);
        
        DECLSPEC_XFGVIRT(AsyncIAdviseSink, Begin_OnRename)
        /* [local] */ void ( STDMETHODCALLTYPE *Begin_OnRename )( 
            AsyncIAdviseSink * This,
            /* [annotation][in] */ 
            _In_  IMoniker *pmk);
        
        DECLSPEC_XFGVIRT(AsyncIAdviseSink, Finish_OnRename)
        /* [local] */ void ( STDMETHODCALLTYPE *Finish_OnRename )( 
            AsyncIAdviseSink * This);
        
        DECLSPEC_XFGVIRT(AsyncIAdviseSink, Begin_OnSave)
        /* [local] */ void ( STDMETHODCALLTYPE *Begin_OnSave )( 
            AsyncIAdviseSink * This);
        
        DECLSPEC_XFGVIRT(AsyncIAdviseSink, Finish_OnSave)
        /* [local] */ void ( STDMETHODCALLTYPE *Finish_OnSave )( 
            AsyncIAdviseSink * This);
        
        DECLSPEC_XFGVIRT(AsyncIAdviseSink, Begin_OnClose)
        /* [local] */ void ( STDMETHODCALLTYPE *Begin_OnClose )( 
            AsyncIAdviseSink * This);
        
        DECLSPEC_XFGVIRT(AsyncIAdviseSink, Finish_OnClose)
        /* [local] */ void ( STDMETHODCALLTYPE *Finish_OnClose )( 
            AsyncIAdviseSink * This);
        
        END_INTERFACE
    } AsyncIAdviseSinkVtbl;

    interface AsyncIAdviseSink
    {
        CONST_VTBL struct AsyncIAdviseSinkVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define AsyncIAdviseSink_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define AsyncIAdviseSink_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define AsyncIAdviseSink_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define AsyncIAdviseSink_Begin_OnDataChange(This,pFormatetc,pStgmed)	\
    ( (This)->lpVtbl -> Begin_OnDataChange(This,pFormatetc,pStgmed) ) 

#define AsyncIAdviseSink_Finish_OnDataChange(This)	\
    ( (This)->lpVtbl -> Finish_OnDataChange(This) ) 

#define AsyncIAdviseSink_Begin_OnViewChange(This,dwAspect,lindex)	\
    ( (This)->lpVtbl -> Begin_OnViewChange(This,dwAspect,lindex) ) 

#define AsyncIAdviseSink_Finish_OnViewChange(This)	\
    ( (This)->lpVtbl -> Finish_OnViewChange(This) ) 

#define AsyncIAdviseSink_Begin_OnRename(This,pmk)	\
    ( (This)->lpVtbl -> Begin_OnRename(This,pmk) ) 

#define AsyncIAdviseSink_Finish_OnRename(This)	\
    ( (This)->lpVtbl -> Finish_OnRename(This) ) 

#define AsyncIAdviseSink_Begin_OnSave(This)	\
    ( (This)->lpVtbl -> Begin_OnSave(This) ) 

#define AsyncIAdviseSink_Finish_OnSave(This)	\
    ( (This)->lpVtbl -> Finish_OnSave(This) ) 

#define AsyncIAdviseSink_Begin_OnClose(This)	\
    ( (This)->lpVtbl -> Begin_OnClose(This) ) 

#define AsyncIAdviseSink_Finish_OnClose(This)	\
    ( (This)->lpVtbl -> Finish_OnClose(This) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */



/* [call_as] */ HRESULT STDMETHODCALLTYPE AsyncIAdviseSink_Begin_RemoteOnDataChange_Proxy( 
    __RPC__in AsyncIAdviseSink * This,
    /* [unique][in] */ __RPC__in_opt FORMATETC *pFormatetc,
    /* [unique][in] */ __RPC__in_opt ASYNC_STGMEDIUM *pStgmed);


void __RPC_STUB AsyncIAdviseSink_Begin_RemoteOnDataChange_Stub(
    IRpcStubBuffer *This,
    IRpcChannelBuffer *_pRpcChannelBuffer,
    PRPC_MESSAGE _pRpcMessage,
    DWORD *_pdwStubPhase);


/* [call_as] */ HRESULT STDMETHODCALLTYPE AsyncIAdviseSink_Finish_RemoteOnDataChange_Proxy( 
    __RPC__in AsyncIAdviseSink * This);


void __RPC_STUB AsyncIAdviseSink_Finish_RemoteOnDataChange_Stub(
    IRpcStubBuffer *This,
    IRpcChannelBuffer *_pRpcChannelBuffer,
    PRPC_MESSAGE _pRpcMessage,
    DWORD *_pdwStubPhase);


/* [call_as] */ HRESULT STDMETHODCALLTYPE AsyncIAdviseSink_Begin_RemoteOnViewChange_Proxy( 
    __RPC__in AsyncIAdviseSink * This,
    /* [in] */ DWORD dwAspect,
    /* [in] */ LONG lindex);


void __RPC_STUB AsyncIAdviseSink_Begin_RemoteOnViewChange_Stub(
    IRpcStubBuffer *This,
    IRpcChannelBuffer *_pRpcChannelBuffer,
    PRPC_MESSAGE _pRpcMessage,
    DWORD *_pdwStubPhase);


/* [call_as] */ HRESULT STDMETHODCALLTYPE AsyncIAdviseSink_Finish_RemoteOnViewChange_Proxy( 
    __RPC__in AsyncIAdviseSink * This);


void __RPC_STUB AsyncIAdviseSink_Finish_RemoteOnViewChange_Stub(
    IRpcStubBuffer *This,
    IRpcChannelBuffer *_pRpcChannelBuffer,
    PRPC_MESSAGE _pRpcMessage,
    DWORD *_pdwStubPhase);


/* [call_as] */ HRESULT STDMETHODCALLTYPE AsyncIAdviseSink_Begin_RemoteOnRename_Proxy( 
    __RPC__in AsyncIAdviseSink * This,
    /* [in] */ __RPC__in_opt IMoniker *pmk);


void __RPC_STUB AsyncIAdviseSink_Begin_RemoteOnRename_Stub(
    IRpcStubBuffer *This,
    IRpcChannelBuffer *_pRpcChannelBuffer,
    PRPC_MESSAGE _pRpcMessage,
    DWORD *_pdwStubPhase);


/* [call_as] */ HRESULT STDMETHODCALLTYPE AsyncIAdviseSink_Finish_RemoteOnRename_Proxy( 
    __RPC__in AsyncIAdviseSink * This);


void __RPC_STUB AsyncIAdviseSink_Finish_RemoteOnRename_Stub(
    IRpcStubBuffer *This,
    IRpcChannelBuffer *_pRpcChannelBuffer,
    PRPC_MESSAGE _pRpcMessage,
    DWORD *_pdwStubPhase);


/* [call_as] */ HRESULT STDMETHODCALLTYPE AsyncIAdviseSink_Begin_RemoteOnSave_Proxy( 
    __RPC__in AsyncIAdviseSink * This);


void __RPC_STUB AsyncIAdviseSink_Begin_RemoteOnSave_Stub(
    IRpcStubBuffer *This,
    IRpcChannelBuffer *_pRpcChannelBuffer,
    PRPC_MESSAGE _pRpcMessage,
    DWORD *_pdwStubPhase);


/* [call_as] */ HRESULT STDMETHODCALLTYPE AsyncIAdviseSink_Finish_RemoteOnSave_Proxy( 
    __RPC__in AsyncIAdviseSink * This);


void __RPC_STUB AsyncIAdviseSink_Finish_RemoteOnSave_Stub(
    IRpcStubBuffer *This,
    IRpcChannelBuffer *_pRpcChannelBuffer,
    PRPC_MESSAGE _pRpcMessage,
    DWORD *_pdwStubPhase);


/* [call_as] */ HRESULT STDMETHODCALLTYPE AsyncIAdviseSink_Begin_RemoteOnClose_Proxy( 
    __RPC__in AsyncIAdviseSink * This);


void __RPC_STUB AsyncIAdviseSink_Begin_RemoteOnClose_Stub(
    IRpcStubBuffer *This,
    IRpcChannelBuffer *_pRpcChannelBuffer,
    PRPC_MESSAGE _pRpcMessage,
    DWORD *_pdwStubPhase);


/* [call_as] */ HRESULT STDMETHODCALLTYPE AsyncIAdviseSink_Finish_RemoteOnClose_Proxy( 
    __RPC__in AsyncIAdviseSink * This);


void __RPC_STUB AsyncIAdviseSink_Finish_RemoteOnClose_Stub(
    IRpcStubBuffer *This,
    IRpcChannelBuffer *_pRpcChannelBuffer,
    PRPC_MESSAGE _pRpcMessage,
    DWORD *_pdwStubPhase);



#endif 	/* __AsyncIAdviseSink_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_objidl_0000_0081 */
/* [local] */ 

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM) */
#pragma endregion
#pragma region Desktop Family or OneCore Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM)


extern RPC_IF_HANDLE __MIDL_itf_objidl_0000_0081_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_objidl_0000_0081_v0_0_s_ifspec;

#ifndef __IAdviseSink2_INTERFACE_DEFINED__
#define __IAdviseSink2_INTERFACE_DEFINED__

/* interface IAdviseSink2 */
/* [unique][async_uuid][uuid][object] */ 

typedef /* [unique] */  __RPC_unique_pointer IAdviseSink2 *LPADVISESINK2;


EXTERN_C const IID IID_IAdviseSink2;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("00000125-0000-0000-C000-000000000046")
    IAdviseSink2 : public IAdviseSink
    {
    public:
        virtual /* [local] */ void STDMETHODCALLTYPE OnLinkSrcChange( 
            /* [annotation][unique][in] */ 
            _In_  IMoniker *pmk) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IAdviseSink2Vtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IAdviseSink2 * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IAdviseSink2 * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IAdviseSink2 * This);
        
        DECLSPEC_XFGVIRT(IAdviseSink, OnDataChange)
        /* [local] */ void ( STDMETHODCALLTYPE *OnDataChange )( 
            IAdviseSink2 * This,
            /* [annotation][unique][in] */ 
            _In_  FORMATETC *pFormatetc,
            /* [annotation][unique][in] */ 
            _In_  STGMEDIUM *pStgmed);
        
        DECLSPEC_XFGVIRT(IAdviseSink, OnViewChange)
        /* [local] */ void ( STDMETHODCALLTYPE *OnViewChange )( 
            IAdviseSink2 * This,
            /* [in] */ DWORD dwAspect,
            /* [in] */ LONG lindex);
        
        DECLSPEC_XFGVIRT(IAdviseSink, OnRename)
        /* [local] */ void ( STDMETHODCALLTYPE *OnRename )( 
            IAdviseSink2 * This,
            /* [annotation][in] */ 
            _In_  IMoniker *pmk);
        
        DECLSPEC_XFGVIRT(IAdviseSink, OnSave)
        /* [local] */ void ( STDMETHODCALLTYPE *OnSave )( 
            IAdviseSink2 * This);
        
        DECLSPEC_XFGVIRT(IAdviseSink, OnClose)
        /* [local] */ void ( STDMETHODCALLTYPE *OnClose )( 
            IAdviseSink2 * This);
        
        DECLSPEC_XFGVIRT(IAdviseSink2, OnLinkSrcChange)
        /* [local] */ void ( STDMETHODCALLTYPE *OnLinkSrcChange )( 
            IAdviseSink2 * This,
            /* [annotation][unique][in] */ 
            _In_  IMoniker *pmk);
        
        END_INTERFACE
    } IAdviseSink2Vtbl;

    interface IAdviseSink2
    {
        CONST_VTBL struct IAdviseSink2Vtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IAdviseSink2_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IAdviseSink2_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IAdviseSink2_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IAdviseSink2_OnDataChange(This,pFormatetc,pStgmed)	\
    ( (This)->lpVtbl -> OnDataChange(This,pFormatetc,pStgmed) ) 

#define IAdviseSink2_OnViewChange(This,dwAspect,lindex)	\
    ( (This)->lpVtbl -> OnViewChange(This,dwAspect,lindex) ) 

#define IAdviseSink2_OnRename(This,pmk)	\
    ( (This)->lpVtbl -> OnRename(This,pmk) ) 

#define IAdviseSink2_OnSave(This)	\
    ( (This)->lpVtbl -> OnSave(This) ) 

#define IAdviseSink2_OnClose(This)	\
    ( (This)->lpVtbl -> OnClose(This) ) 


#define IAdviseSink2_OnLinkSrcChange(This,pmk)	\
    ( (This)->lpVtbl -> OnLinkSrcChange(This,pmk) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */



/* [call_as] */ HRESULT STDMETHODCALLTYPE IAdviseSink2_RemoteOnLinkSrcChange_Proxy( 
    __RPC__in IAdviseSink2 * This,
    /* [unique][in] */ __RPC__in_opt IMoniker *pmk);


void __RPC_STUB IAdviseSink2_RemoteOnLinkSrcChange_Stub(
    IRpcStubBuffer *This,
    IRpcChannelBuffer *_pRpcChannelBuffer,
    PRPC_MESSAGE _pRpcMessage,
    DWORD *_pdwStubPhase);



#endif 	/* __IAdviseSink2_INTERFACE_DEFINED__ */


#ifndef __AsyncIAdviseSink2_INTERFACE_DEFINED__
#define __AsyncIAdviseSink2_INTERFACE_DEFINED__

/* interface AsyncIAdviseSink2 */
/* [uuid][unique][object] */ 


EXTERN_C const IID IID_AsyncIAdviseSink2;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("00000151-0000-0000-C000-000000000046")
    AsyncIAdviseSink2 : public AsyncIAdviseSink
    {
    public:
        virtual /* [local] */ void STDMETHODCALLTYPE Begin_OnLinkSrcChange( 
            /* [annotation][unique][in] */ 
            _In_  IMoniker *pmk) = 0;
        
        virtual /* [local] */ void STDMETHODCALLTYPE Finish_OnLinkSrcChange( void) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct AsyncIAdviseSink2Vtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in AsyncIAdviseSink2 * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in AsyncIAdviseSink2 * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in AsyncIAdviseSink2 * This);
        
        DECLSPEC_XFGVIRT(AsyncIAdviseSink, Begin_OnDataChange)
        /* [local] */ void ( STDMETHODCALLTYPE *Begin_OnDataChange )( 
            AsyncIAdviseSink2 * This,
            /* [annotation][unique][in] */ 
            _In_  FORMATETC *pFormatetc,
            /* [annotation][unique][in] */ 
            _In_  STGMEDIUM *pStgmed);
        
        DECLSPEC_XFGVIRT(AsyncIAdviseSink, Finish_OnDataChange)
        /* [local] */ void ( STDMETHODCALLTYPE *Finish_OnDataChange )( 
            AsyncIAdviseSink2 * This);
        
        DECLSPEC_XFGVIRT(AsyncIAdviseSink, Begin_OnViewChange)
        /* [local] */ void ( STDMETHODCALLTYPE *Begin_OnViewChange )( 
            AsyncIAdviseSink2 * This,
            /* [in] */ DWORD dwAspect,
            /* [in] */ LONG lindex);
        
        DECLSPEC_XFGVIRT(AsyncIAdviseSink, Finish_OnViewChange)
        /* [local] */ void ( STDMETHODCALLTYPE *Finish_OnViewChange )( 
            AsyncIAdviseSink2 * This);
        
        DECLSPEC_XFGVIRT(AsyncIAdviseSink, Begin_OnRename)
        /* [local] */ void ( STDMETHODCALLTYPE *Begin_OnRename )( 
            AsyncIAdviseSink2 * This,
            /* [annotation][in] */ 
            _In_  IMoniker *pmk);
        
        DECLSPEC_XFGVIRT(AsyncIAdviseSink, Finish_OnRename)
        /* [local] */ void ( STDMETHODCALLTYPE *Finish_OnRename )( 
            AsyncIAdviseSink2 * This);
        
        DECLSPEC_XFGVIRT(AsyncIAdviseSink, Begin_OnSave)
        /* [local] */ void ( STDMETHODCALLTYPE *Begin_OnSave )( 
            AsyncIAdviseSink2 * This);
        
        DECLSPEC_XFGVIRT(AsyncIAdviseSink, Finish_OnSave)
        /* [local] */ void ( STDMETHODCALLTYPE *Finish_OnSave )( 
            AsyncIAdviseSink2 * This);
        
        DECLSPEC_XFGVIRT(AsyncIAdviseSink, Begin_OnClose)
        /* [local] */ void ( STDMETHODCALLTYPE *Begin_OnClose )( 
            AsyncIAdviseSink2 * This);
        
        DECLSPEC_XFGVIRT(AsyncIAdviseSink, Finish_OnClose)
        /* [local] */ void ( STDMETHODCALLTYPE *Finish_OnClose )( 
            AsyncIAdviseSink2 * This);
        
        DECLSPEC_XFGVIRT(AsyncIAdviseSink2, Begin_OnLinkSrcChange)
        /* [local] */ void ( STDMETHODCALLTYPE *Begin_OnLinkSrcChange )( 
            AsyncIAdviseSink2 * This,
            /* [annotation][unique][in] */ 
            _In_  IMoniker *pmk);
        
        DECLSPEC_XFGVIRT(AsyncIAdviseSink2, Finish_OnLinkSrcChange)
        /* [local] */ void ( STDMETHODCALLTYPE *Finish_OnLinkSrcChange )( 
            AsyncIAdviseSink2 * This);
        
        END_INTERFACE
    } AsyncIAdviseSink2Vtbl;

    interface AsyncIAdviseSink2
    {
        CONST_VTBL struct AsyncIAdviseSink2Vtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define AsyncIAdviseSink2_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define AsyncIAdviseSink2_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define AsyncIAdviseSink2_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define AsyncIAdviseSink2_Begin_OnDataChange(This,pFormatetc,pStgmed)	\
    ( (This)->lpVtbl -> Begin_OnDataChange(This,pFormatetc,pStgmed) ) 

#define AsyncIAdviseSink2_Finish_OnDataChange(This)	\
    ( (This)->lpVtbl -> Finish_OnDataChange(This) ) 

#define AsyncIAdviseSink2_Begin_OnViewChange(This,dwAspect,lindex)	\
    ( (This)->lpVtbl -> Begin_OnViewChange(This,dwAspect,lindex) ) 

#define AsyncIAdviseSink2_Finish_OnViewChange(This)	\
    ( (This)->lpVtbl -> Finish_OnViewChange(This) ) 

#define AsyncIAdviseSink2_Begin_OnRename(This,pmk)	\
    ( (This)->lpVtbl -> Begin_OnRename(This,pmk) ) 

#define AsyncIAdviseSink2_Finish_OnRename(This)	\
    ( (This)->lpVtbl -> Finish_OnRename(This) ) 

#define AsyncIAdviseSink2_Begin_OnSave(This)	\
    ( (This)->lpVtbl -> Begin_OnSave(This) ) 

#define AsyncIAdviseSink2_Finish_OnSave(This)	\
    ( (This)->lpVtbl -> Finish_OnSave(This) ) 

#define AsyncIAdviseSink2_Begin_OnClose(This)	\
    ( (This)->lpVtbl -> Begin_OnClose(This) ) 

#define AsyncIAdviseSink2_Finish_OnClose(This)	\
    ( (This)->lpVtbl -> Finish_OnClose(This) ) 


#define AsyncIAdviseSink2_Begin_OnLinkSrcChange(This,pmk)	\
    ( (This)->lpVtbl -> Begin_OnLinkSrcChange(This,pmk) ) 

#define AsyncIAdviseSink2_Finish_OnLinkSrcChange(This)	\
    ( (This)->lpVtbl -> Finish_OnLinkSrcChange(This) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */



/* [call_as] */ HRESULT STDMETHODCALLTYPE AsyncIAdviseSink2_Begin_RemoteOnLinkSrcChange_Proxy( 
    __RPC__in AsyncIAdviseSink2 * This,
    /* [unique][in] */ __RPC__in_opt IMoniker *pmk);


void __RPC_STUB AsyncIAdviseSink2_Begin_RemoteOnLinkSrcChange_Stub(
    IRpcStubBuffer *This,
    IRpcChannelBuffer *_pRpcChannelBuffer,
    PRPC_MESSAGE _pRpcMessage,
    DWORD *_pdwStubPhase);


/* [call_as] */ HRESULT STDMETHODCALLTYPE AsyncIAdviseSink2_Finish_RemoteOnLinkSrcChange_Proxy( 
    __RPC__in AsyncIAdviseSink2 * This);


void __RPC_STUB AsyncIAdviseSink2_Finish_RemoteOnLinkSrcChange_Stub(
    IRpcStubBuffer *This,
    IRpcChannelBuffer *_pRpcChannelBuffer,
    PRPC_MESSAGE _pRpcMessage,
    DWORD *_pdwStubPhase);



#endif 	/* __AsyncIAdviseSink2_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_objidl_0000_0082 */
/* [local] */ 

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES) */
#pragma endregion
#pragma region Application Family or OneCore Family or Games Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES)


extern RPC_IF_HANDLE __MIDL_itf_objidl_0000_0082_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_objidl_0000_0082_v0_0_s_ifspec;

#ifndef __IDataObject_INTERFACE_DEFINED__
#define __IDataObject_INTERFACE_DEFINED__

/* interface IDataObject */
/* [unique][uuid][object] */ 

typedef /* [unique] */  __RPC_unique_pointer IDataObject *LPDATAOBJECT;

typedef 
enum tagDATADIR
    {
        DATADIR_GET	= 1,
        DATADIR_SET	= 2
    } 	DATADIR;


EXTERN_C const IID IID_IDataObject;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("0000010e-0000-0000-C000-000000000046")
    IDataObject : public IUnknown
    {
    public:
        virtual /* [local] */ HRESULT STDMETHODCALLTYPE GetData( 
            /* [annotation][unique][in] */ 
            _In_  FORMATETC *pformatetcIn,
            /* [annotation][out] */ 
            _Out_  STGMEDIUM *pmedium) = 0;
        
        virtual /* [local] */ HRESULT STDMETHODCALLTYPE GetDataHere( 
            /* [annotation][unique][in] */ 
            _In_  FORMATETC *pformatetc,
            /* [annotation][out][in] */ 
            _Inout_  STGMEDIUM *pmedium) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE QueryGetData( 
            /* [unique][in] */ __RPC__in_opt FORMATETC *pformatetc) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetCanonicalFormatEtc( 
            /* [unique][in] */ __RPC__in_opt FORMATETC *pformatectIn,
            /* [out] */ __RPC__out FORMATETC *pformatetcOut) = 0;
        
        virtual /* [local] */ HRESULT STDMETHODCALLTYPE SetData( 
            /* [annotation][unique][in] */ 
            _In_  FORMATETC *pformatetc,
            /* [annotation][unique][in] */ 
            _In_  STGMEDIUM *pmedium,
            /* [in] */ BOOL fRelease) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE EnumFormatEtc( 
            /* [in] */ DWORD dwDirection,
            /* [out] */ __RPC__deref_out_opt IEnumFORMATETC **ppenumFormatEtc) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE DAdvise( 
            /* [in] */ __RPC__in FORMATETC *pformatetc,
            /* [in] */ DWORD advf,
            /* [unique][in] */ __RPC__in_opt IAdviseSink *pAdvSink,
            /* [out] */ __RPC__out DWORD *pdwConnection) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE DUnadvise( 
            /* [in] */ DWORD dwConnection) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE EnumDAdvise( 
            /* [out] */ __RPC__deref_out_opt IEnumSTATDATA **ppenumAdvise) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IDataObjectVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IDataObject * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IDataObject * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IDataObject * This);
        
        DECLSPEC_XFGVIRT(IDataObject, GetData)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *GetData )( 
            IDataObject * This,
            /* [annotation][unique][in] */ 
            _In_  FORMATETC *pformatetcIn,
            /* [annotation][out] */ 
            _Out_  STGMEDIUM *pmedium);
        
        DECLSPEC_XFGVIRT(IDataObject, GetDataHere)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *GetDataHere )( 
            IDataObject * This,
            /* [annotation][unique][in] */ 
            _In_  FORMATETC *pformatetc,
            /* [annotation][out][in] */ 
            _Inout_  STGMEDIUM *pmedium);
        
        DECLSPEC_XFGVIRT(IDataObject, QueryGetData)
        HRESULT ( STDMETHODCALLTYPE *QueryGetData )( 
            __RPC__in IDataObject * This,
            /* [unique][in] */ __RPC__in_opt FORMATETC *pformatetc);
        
        DECLSPEC_XFGVIRT(IDataObject, GetCanonicalFormatEtc)
        HRESULT ( STDMETHODCALLTYPE *GetCanonicalFormatEtc )( 
            __RPC__in IDataObject * This,
            /* [unique][in] */ __RPC__in_opt FORMATETC *pformatectIn,
            /* [out] */ __RPC__out FORMATETC *pformatetcOut);
        
        DECLSPEC_XFGVIRT(IDataObject, SetData)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *SetData )( 
            IDataObject * This,
            /* [annotation][unique][in] */ 
            _In_  FORMATETC *pformatetc,
            /* [annotation][unique][in] */ 
            _In_  STGMEDIUM *pmedium,
            /* [in] */ BOOL fRelease);
        
        DECLSPEC_XFGVIRT(IDataObject, EnumFormatEtc)
        HRESULT ( STDMETHODCALLTYPE *EnumFormatEtc )( 
            __RPC__in IDataObject * This,
            /* [in] */ DWORD dwDirection,
            /* [out] */ __RPC__deref_out_opt IEnumFORMATETC **ppenumFormatEtc);
        
        DECLSPEC_XFGVIRT(IDataObject, DAdvise)
        HRESULT ( STDMETHODCALLTYPE *DAdvise )( 
            __RPC__in IDataObject * This,
            /* [in] */ __RPC__in FORMATETC *pformatetc,
            /* [in] */ DWORD advf,
            /* [unique][in] */ __RPC__in_opt IAdviseSink *pAdvSink,
            /* [out] */ __RPC__out DWORD *pdwConnection);
        
        DECLSPEC_XFGVIRT(IDataObject, DUnadvise)
        HRESULT ( STDMETHODCALLTYPE *DUnadvise )( 
            __RPC__in IDataObject * This,
            /* [in] */ DWORD dwConnection);
        
        DECLSPEC_XFGVIRT(IDataObject, EnumDAdvise)
        HRESULT ( STDMETHODCALLTYPE *EnumDAdvise )( 
            __RPC__in IDataObject * This,
            /* [out] */ __RPC__deref_out_opt IEnumSTATDATA **ppenumAdvise);
        
        END_INTERFACE
    } IDataObjectVtbl;

    interface IDataObject
    {
        CONST_VTBL struct IDataObjectVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IDataObject_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IDataObject_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IDataObject_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IDataObject_GetData(This,pformatetcIn,pmedium)	\
    ( (This)->lpVtbl -> GetData(This,pformatetcIn,pmedium) ) 

#define IDataObject_GetDataHere(This,pformatetc,pmedium)	\
    ( (This)->lpVtbl -> GetDataHere(This,pformatetc,pmedium) ) 

#define IDataObject_QueryGetData(This,pformatetc)	\
    ( (This)->lpVtbl -> QueryGetData(This,pformatetc) ) 

#define IDataObject_GetCanonicalFormatEtc(This,pformatectIn,pformatetcOut)	\
    ( (This)->lpVtbl -> GetCanonicalFormatEtc(This,pformatectIn,pformatetcOut) ) 

#define IDataObject_SetData(This,pformatetc,pmedium,fRelease)	\
    ( (This)->lpVtbl -> SetData(This,pformatetc,pmedium,fRelease) ) 

#define IDataObject_EnumFormatEtc(This,dwDirection,ppenumFormatEtc)	\
    ( (This)->lpVtbl -> EnumFormatEtc(This,dwDirection,ppenumFormatEtc) ) 

#define IDataObject_DAdvise(This,pformatetc,advf,pAdvSink,pdwConnection)	\
    ( (This)->lpVtbl -> DAdvise(This,pformatetc,advf,pAdvSink,pdwConnection) ) 

#define IDataObject_DUnadvise(This,dwConnection)	\
    ( (This)->lpVtbl -> DUnadvise(This,dwConnection) ) 

#define IDataObject_EnumDAdvise(This,ppenumAdvise)	\
    ( (This)->lpVtbl -> EnumDAdvise(This,ppenumAdvise) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */



/* [call_as] */ HRESULT STDMETHODCALLTYPE IDataObject_RemoteGetData_Proxy( 
    __RPC__in IDataObject * This,
    /* [unique][in] */ __RPC__in_opt FORMATETC *pformatetcIn,
    /* [out] */ __RPC__out STGMEDIUM *pRemoteMedium);


void __RPC_STUB IDataObject_RemoteGetData_Stub(
    IRpcStubBuffer *This,
    IRpcChannelBuffer *_pRpcChannelBuffer,
    PRPC_MESSAGE _pRpcMessage,
    DWORD *_pdwStubPhase);


/* [call_as] */ HRESULT STDMETHODCALLTYPE IDataObject_RemoteGetDataHere_Proxy( 
    __RPC__in IDataObject * This,
    /* [unique][in] */ __RPC__in_opt FORMATETC *pformatetc,
    /* [out][in] */ __RPC__inout STGMEDIUM *pRemoteMedium);


void __RPC_STUB IDataObject_RemoteGetDataHere_Stub(
    IRpcStubBuffer *This,
    IRpcChannelBuffer *_pRpcChannelBuffer,
    PRPC_MESSAGE _pRpcMessage,
    DWORD *_pdwStubPhase);


/* [call_as] */ HRESULT STDMETHODCALLTYPE IDataObject_RemoteSetData_Proxy( 
    __RPC__in IDataObject * This,
    /* [unique][in] */ __RPC__in_opt FORMATETC *pformatetc,
    /* [unique][in] */ __RPC__in_opt FLAG_STGMEDIUM *pmedium,
    /* [in] */ BOOL fRelease);


void __RPC_STUB IDataObject_RemoteSetData_Stub(
    IRpcStubBuffer *This,
    IRpcChannelBuffer *_pRpcChannelBuffer,
    PRPC_MESSAGE _pRpcMessage,
    DWORD *_pdwStubPhase);



#endif 	/* __IDataObject_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_objidl_0000_0083 */
/* [local] */ 

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES) */
#pragma endregion
#pragma region Desktop Family or OneCore Family or Games Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES)


extern RPC_IF_HANDLE __MIDL_itf_objidl_0000_0083_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_objidl_0000_0083_v0_0_s_ifspec;

#ifndef __IDataAdviseHolder_INTERFACE_DEFINED__
#define __IDataAdviseHolder_INTERFACE_DEFINED__

/* interface IDataAdviseHolder */
/* [uuid][object][local] */ 

typedef /* [unique] */ IDataAdviseHolder *LPDATAADVISEHOLDER;


EXTERN_C const IID IID_IDataAdviseHolder;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("00000110-0000-0000-C000-000000000046")
    IDataAdviseHolder : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE Advise( 
            /* [annotation][unique][in] */ 
            _In_opt_  IDataObject *pDataObject,
            /* [annotation][unique][in] */ 
            _In_  FORMATETC *pFetc,
            /* [annotation][in] */ 
            _In_  DWORD advf,
            /* [annotation][unique][in] */ 
            _In_  IAdviseSink *pAdvise,
            /* [annotation][out] */ 
            _Out_  DWORD *pdwConnection) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Unadvise( 
            /* [annotation][in] */ 
            _In_  DWORD dwConnection) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE EnumAdvise( 
            /* [annotation][out] */ 
            _Outptr_  IEnumSTATDATA **ppenumAdvise) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SendOnDataChange( 
            /* [annotation][unique][in] */ 
            _In_  IDataObject *pDataObject,
            /* [annotation][in] */ 
            _Reserved_  DWORD dwReserved,
            /* [annotation][in] */ 
            _In_  DWORD advf) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IDataAdviseHolderVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IDataAdviseHolder * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IDataAdviseHolder * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IDataAdviseHolder * This);
        
        DECLSPEC_XFGVIRT(IDataAdviseHolder, Advise)
        HRESULT ( STDMETHODCALLTYPE *Advise )( 
            IDataAdviseHolder * This,
            /* [annotation][unique][in] */ 
            _In_opt_  IDataObject *pDataObject,
            /* [annotation][unique][in] */ 
            _In_  FORMATETC *pFetc,
            /* [annotation][in] */ 
            _In_  DWORD advf,
            /* [annotation][unique][in] */ 
            _In_  IAdviseSink *pAdvise,
            /* [annotation][out] */ 
            _Out_  DWORD *pdwConnection);
        
        DECLSPEC_XFGVIRT(IDataAdviseHolder, Unadvise)
        HRESULT ( STDMETHODCALLTYPE *Unadvise )( 
            IDataAdviseHolder * This,
            /* [annotation][in] */ 
            _In_  DWORD dwConnection);
        
        DECLSPEC_XFGVIRT(IDataAdviseHolder, EnumAdvise)
        HRESULT ( STDMETHODCALLTYPE *EnumAdvise )( 
            IDataAdviseHolder * This,
            /* [annotation][out] */ 
            _Outptr_  IEnumSTATDATA **ppenumAdvise);
        
        DECLSPEC_XFGVIRT(IDataAdviseHolder, SendOnDataChange)
        HRESULT ( STDMETHODCALLTYPE *SendOnDataChange )( 
            IDataAdviseHolder * This,
            /* [annotation][unique][in] */ 
            _In_  IDataObject *pDataObject,
            /* [annotation][in] */ 
            _Reserved_  DWORD dwReserved,
            /* [annotation][in] */ 
            _In_  DWORD advf);
        
        END_INTERFACE
    } IDataAdviseHolderVtbl;

    interface IDataAdviseHolder
    {
        CONST_VTBL struct IDataAdviseHolderVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IDataAdviseHolder_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IDataAdviseHolder_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IDataAdviseHolder_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IDataAdviseHolder_Advise(This,pDataObject,pFetc,advf,pAdvise,pdwConnection)	\
    ( (This)->lpVtbl -> Advise(This,pDataObject,pFetc,advf,pAdvise,pdwConnection) ) 

#define IDataAdviseHolder_Unadvise(This,dwConnection)	\
    ( (This)->lpVtbl -> Unadvise(This,dwConnection) ) 

#define IDataAdviseHolder_EnumAdvise(This,ppenumAdvise)	\
    ( (This)->lpVtbl -> EnumAdvise(This,ppenumAdvise) ) 

#define IDataAdviseHolder_SendOnDataChange(This,pDataObject,dwReserved,advf)	\
    ( (This)->lpVtbl -> SendOnDataChange(This,pDataObject,dwReserved,advf) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IDataAdviseHolder_INTERFACE_DEFINED__ */


#ifndef __IMessageFilter_INTERFACE_DEFINED__
#define __IMessageFilter_INTERFACE_DEFINED__

/* interface IMessageFilter */
/* [uuid][object][local] */ 

typedef /* [unique] */ IMessageFilter *LPMESSAGEFILTER;

typedef 
enum tagCALLTYPE
    {
        CALLTYPE_TOPLEVEL	= 1,
        CALLTYPE_NESTED	= 2,
        CALLTYPE_ASYNC	= 3,
        CALLTYPE_TOPLEVEL_CALLPENDING	= 4,
        CALLTYPE_ASYNC_CALLPENDING	= 5
    } 	CALLTYPE;

typedef 
enum tagSERVERCALL
    {
        SERVERCALL_ISHANDLED	= 0,
        SERVERCALL_REJECTED	= 1,
        SERVERCALL_RETRYLATER	= 2
    } 	SERVERCALL;

typedef 
enum tagPENDINGTYPE
    {
        PENDINGTYPE_TOPLEVEL	= 1,
        PENDINGTYPE_NESTED	= 2
    } 	PENDINGTYPE;

typedef 
enum tagPENDINGMSG
    {
        PENDINGMSG_CANCELCALL	= 0,
        PENDINGMSG_WAITNOPROCESS	= 1,
        PENDINGMSG_WAITDEFPROCESS	= 2
    } 	PENDINGMSG;

typedef struct tagINTERFACEINFO
    {
    IUnknown *pUnk;
    IID iid;
    WORD wMethod;
    } 	INTERFACEINFO;

typedef struct tagINTERFACEINFO *LPINTERFACEINFO;


EXTERN_C const IID IID_IMessageFilter;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("00000016-0000-0000-C000-000000000046")
    IMessageFilter : public IUnknown
    {
    public:
        virtual DWORD STDMETHODCALLTYPE HandleInComingCall( 
            /* [annotation][in] */ 
            _In_  DWORD dwCallType,
            /* [annotation][in] */ 
            _In_  HTASK htaskCaller,
            /* [annotation][in] */ 
            _In_  DWORD dwTickCount,
            /* [annotation][in] */ 
            _In_opt_  LPINTERFACEINFO lpInterfaceInfo) = 0;
        
        virtual DWORD STDMETHODCALLTYPE RetryRejectedCall( 
            /* [annotation][in] */ 
            _In_  HTASK htaskCallee,
            /* [annotation][in] */ 
            _In_  DWORD dwTickCount,
            /* [annotation][in] */ 
            _In_  DWORD dwRejectType) = 0;
        
        virtual DWORD STDMETHODCALLTYPE MessagePending( 
            /* [annotation][in] */ 
            _In_  HTASK htaskCallee,
            /* [annotation][in] */ 
            _In_  DWORD dwTickCount,
            /* [annotation][in] */ 
            _In_  DWORD dwPendingType) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IMessageFilterVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IMessageFilter * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IMessageFilter * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IMessageFilter * This);
        
        DECLSPEC_XFGVIRT(IMessageFilter, HandleInComingCall)
        DWORD ( STDMETHODCALLTYPE *HandleInComingCall )( 
            IMessageFilter * This,
            /* [annotation][in] */ 
            _In_  DWORD dwCallType,
            /* [annotation][in] */ 
            _In_  HTASK htaskCaller,
            /* [annotation][in] */ 
            _In_  DWORD dwTickCount,
            /* [annotation][in] */ 
            _In_opt_  LPINTERFACEINFO lpInterfaceInfo);
        
        DECLSPEC_XFGVIRT(IMessageFilter, RetryRejectedCall)
        DWORD ( STDMETHODCALLTYPE *RetryRejectedCall )( 
            IMessageFilter * This,
            /* [annotation][in] */ 
            _In_  HTASK htaskCallee,
            /* [annotation][in] */ 
            _In_  DWORD dwTickCount,
            /* [annotation][in] */ 
            _In_  DWORD dwRejectType);
        
        DECLSPEC_XFGVIRT(IMessageFilter, MessagePending)
        DWORD ( STDMETHODCALLTYPE *MessagePending )( 
            IMessageFilter * This,
            /* [annotation][in] */ 
            _In_  HTASK htaskCallee,
            /* [annotation][in] */ 
            _In_  DWORD dwTickCount,
            /* [annotation][in] */ 
            _In_  DWORD dwPendingType);
        
        END_INTERFACE
    } IMessageFilterVtbl;

    interface IMessageFilter
    {
        CONST_VTBL struct IMessageFilterVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IMessageFilter_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IMessageFilter_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IMessageFilter_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IMessageFilter_HandleInComingCall(This,dwCallType,htaskCaller,dwTickCount,lpInterfaceInfo)	\
    ( (This)->lpVtbl -> HandleInComingCall(This,dwCallType,htaskCaller,dwTickCount,lpInterfaceInfo) ) 

#define IMessageFilter_RetryRejectedCall(This,htaskCallee,dwTickCount,dwRejectType)	\
    ( (This)->lpVtbl -> RetryRejectedCall(This,htaskCallee,dwTickCount,dwRejectType) ) 

#define IMessageFilter_MessagePending(This,htaskCallee,dwTickCount,dwPendingType)	\
    ( (This)->lpVtbl -> MessagePending(This,htaskCallee,dwTickCount,dwPendingType) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IMessageFilter_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_objidl_0000_0085 */
/* [local] */ 


// Well-known Property Set Format IDs
extern const FMTID FMTID_SummaryInformation;

extern const FMTID FMTID_DocSummaryInformation;

extern const FMTID FMTID_UserDefinedProperties;

extern const FMTID FMTID_DiscardableInformation;

extern const FMTID FMTID_ImageSummaryInformation;

extern const FMTID FMTID_AudioSummaryInformation;

extern const FMTID FMTID_VideoSummaryInformation;

extern const FMTID FMTID_MediaFileSummaryInformation;



extern RPC_IF_HANDLE __MIDL_itf_objidl_0000_0085_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_objidl_0000_0085_v0_0_s_ifspec;

#ifndef __IClassActivator_INTERFACE_DEFINED__
#define __IClassActivator_INTERFACE_DEFINED__

/* interface IClassActivator */
/* [uuid][object] */ 


EXTERN_C const IID IID_IClassActivator;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("00000140-0000-0000-C000-000000000046")
    IClassActivator : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetClassObject( 
            /* [in] */ __RPC__in REFCLSID rclsid,
            /* [in] */ DWORD dwClassContext,
            /* [in] */ LCID locale,
            /* [in] */ __RPC__in REFIID riid,
            /* [iid_is][out] */ __RPC__deref_out_opt void **ppv) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IClassActivatorVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IClassActivator * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IClassActivator * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IClassActivator * This);
        
        DECLSPEC_XFGVIRT(IClassActivator, GetClassObject)
        HRESULT ( STDMETHODCALLTYPE *GetClassObject )( 
            __RPC__in IClassActivator * This,
            /* [in] */ __RPC__in REFCLSID rclsid,
            /* [in] */ DWORD dwClassContext,
            /* [in] */ LCID locale,
            /* [in] */ __RPC__in REFIID riid,
            /* [iid_is][out] */ __RPC__deref_out_opt void **ppv);
        
        END_INTERFACE
    } IClassActivatorVtbl;

    interface IClassActivator
    {
        CONST_VTBL struct IClassActivatorVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IClassActivator_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IClassActivator_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IClassActivator_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IClassActivator_GetClassObject(This,rclsid,dwClassContext,locale,riid,ppv)	\
    ( (This)->lpVtbl -> GetClassObject(This,rclsid,dwClassContext,locale,riid,ppv) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IClassActivator_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_objidl_0000_0086 */
/* [local] */ 

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES) */
#pragma endregion
#pragma region Application Family or OneCore Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM)


extern RPC_IF_HANDLE __MIDL_itf_objidl_0000_0086_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_objidl_0000_0086_v0_0_s_ifspec;

#ifndef __IFillLockBytes_INTERFACE_DEFINED__
#define __IFillLockBytes_INTERFACE_DEFINED__

/* interface IFillLockBytes */
/* [unique][uuid][object] */ 


EXTERN_C const IID IID_IFillLockBytes;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("99caf010-415e-11cf-8814-00aa00b569f5")
    IFillLockBytes : public IUnknown
    {
    public:
        virtual /* [local] */ HRESULT STDMETHODCALLTYPE FillAppend( 
            /* [annotation][size_is][in] */ 
            _In_reads_bytes_(cb)  const void *pv,
            /* [annotation][in] */ 
            _In_  ULONG cb,
            /* [annotation][out] */ 
            _Out_  ULONG *pcbWritten) = 0;
        
        virtual /* [local] */ HRESULT STDMETHODCALLTYPE FillAt( 
            /* [annotation][in] */ 
            _In_  ULARGE_INTEGER ulOffset,
            /* [annotation][size_is][in] */ 
            _In_reads_bytes_(cb)  const void *pv,
            /* [annotation][in] */ 
            _In_  ULONG cb,
            /* [annotation][out] */ 
            _Out_  ULONG *pcbWritten) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetFillSize( 
            /* [in] */ ULARGE_INTEGER ulSize) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Terminate( 
            /* [in] */ BOOL bCanceled) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IFillLockBytesVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IFillLockBytes * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IFillLockBytes * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IFillLockBytes * This);
        
        DECLSPEC_XFGVIRT(IFillLockBytes, FillAppend)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *FillAppend )( 
            IFillLockBytes * This,
            /* [annotation][size_is][in] */ 
            _In_reads_bytes_(cb)  const void *pv,
            /* [annotation][in] */ 
            _In_  ULONG cb,
            /* [annotation][out] */ 
            _Out_  ULONG *pcbWritten);
        
        DECLSPEC_XFGVIRT(IFillLockBytes, FillAt)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *FillAt )( 
            IFillLockBytes * This,
            /* [annotation][in] */ 
            _In_  ULARGE_INTEGER ulOffset,
            /* [annotation][size_is][in] */ 
            _In_reads_bytes_(cb)  const void *pv,
            /* [annotation][in] */ 
            _In_  ULONG cb,
            /* [annotation][out] */ 
            _Out_  ULONG *pcbWritten);
        
        DECLSPEC_XFGVIRT(IFillLockBytes, SetFillSize)
        HRESULT ( STDMETHODCALLTYPE *SetFillSize )( 
            __RPC__in IFillLockBytes * This,
            /* [in] */ ULARGE_INTEGER ulSize);
        
        DECLSPEC_XFGVIRT(IFillLockBytes, Terminate)
        HRESULT ( STDMETHODCALLTYPE *Terminate )( 
            __RPC__in IFillLockBytes * This,
            /* [in] */ BOOL bCanceled);
        
        END_INTERFACE
    } IFillLockBytesVtbl;

    interface IFillLockBytes
    {
        CONST_VTBL struct IFillLockBytesVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IFillLockBytes_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IFillLockBytes_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IFillLockBytes_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IFillLockBytes_FillAppend(This,pv,cb,pcbWritten)	\
    ( (This)->lpVtbl -> FillAppend(This,pv,cb,pcbWritten) ) 

#define IFillLockBytes_FillAt(This,ulOffset,pv,cb,pcbWritten)	\
    ( (This)->lpVtbl -> FillAt(This,ulOffset,pv,cb,pcbWritten) ) 

#define IFillLockBytes_SetFillSize(This,ulSize)	\
    ( (This)->lpVtbl -> SetFillSize(This,ulSize) ) 

#define IFillLockBytes_Terminate(This,bCanceled)	\
    ( (This)->lpVtbl -> Terminate(This,bCanceled) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */



/* [call_as] */ HRESULT __stdcall IFillLockBytes_RemoteFillAppend_Proxy( 
    __RPC__in IFillLockBytes * This,
    /* [size_is][in] */ __RPC__in_ecount_full(cb) const byte *pv,
    /* [in] */ ULONG cb,
    /* [out] */ __RPC__out ULONG *pcbWritten);


void __RPC_STUB IFillLockBytes_RemoteFillAppend_Stub(
    IRpcStubBuffer *This,
    IRpcChannelBuffer *_pRpcChannelBuffer,
    PRPC_MESSAGE _pRpcMessage,
    DWORD *_pdwStubPhase);


/* [call_as] */ HRESULT __stdcall IFillLockBytes_RemoteFillAt_Proxy( 
    __RPC__in IFillLockBytes * This,
    /* [in] */ ULARGE_INTEGER ulOffset,
    /* [size_is][in] */ __RPC__in_ecount_full(cb) const byte *pv,
    /* [in] */ ULONG cb,
    /* [out] */ __RPC__out ULONG *pcbWritten);


void __RPC_STUB IFillLockBytes_RemoteFillAt_Stub(
    IRpcStubBuffer *This,
    IRpcChannelBuffer *_pRpcChannelBuffer,
    PRPC_MESSAGE _pRpcMessage,
    DWORD *_pdwStubPhase);



#endif 	/* __IFillLockBytes_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_objidl_0000_0087 */
/* [local] */ 

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM) */
#pragma endregion
#pragma region Desktop Family or OneCore Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM)


extern RPC_IF_HANDLE __MIDL_itf_objidl_0000_0087_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_objidl_0000_0087_v0_0_s_ifspec;

#ifndef __IProgressNotify_INTERFACE_DEFINED__
#define __IProgressNotify_INTERFACE_DEFINED__

/* interface IProgressNotify */
/* [unique][uuid][object] */ 


EXTERN_C const IID IID_IProgressNotify;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("a9d758a0-4617-11cf-95fc-00aa00680db4")
    IProgressNotify : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE OnProgress( 
            /* [in] */ DWORD dwProgressCurrent,
            /* [in] */ DWORD dwProgressMaximum,
            /* [in] */ BOOL fAccurate,
            /* [in] */ BOOL fOwner) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IProgressNotifyVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IProgressNotify * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IProgressNotify * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IProgressNotify * This);
        
        DECLSPEC_XFGVIRT(IProgressNotify, OnProgress)
        HRESULT ( STDMETHODCALLTYPE *OnProgress )( 
            __RPC__in IProgressNotify * This,
            /* [in] */ DWORD dwProgressCurrent,
            /* [in] */ DWORD dwProgressMaximum,
            /* [in] */ BOOL fAccurate,
            /* [in] */ BOOL fOwner);
        
        END_INTERFACE
    } IProgressNotifyVtbl;

    interface IProgressNotify
    {
        CONST_VTBL struct IProgressNotifyVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IProgressNotify_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IProgressNotify_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IProgressNotify_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IProgressNotify_OnProgress(This,dwProgressCurrent,dwProgressMaximum,fAccurate,fOwner)	\
    ( (This)->lpVtbl -> OnProgress(This,dwProgressCurrent,dwProgressMaximum,fAccurate,fOwner) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IProgressNotify_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_objidl_0000_0088 */
/* [local] */ 

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM) */
#pragma endregion
#pragma region Application Family or OneCore Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM)


extern RPC_IF_HANDLE __MIDL_itf_objidl_0000_0088_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_objidl_0000_0088_v0_0_s_ifspec;

#ifndef __ILayoutStorage_INTERFACE_DEFINED__
#define __ILayoutStorage_INTERFACE_DEFINED__

/* interface ILayoutStorage */
/* [unique][uuid][object][local] */ 

typedef struct tagStorageLayout
    {
    DWORD LayoutType;
    OLECHAR *pwcsElementName;
    LARGE_INTEGER cOffset;
    LARGE_INTEGER cBytes;
    } 	StorageLayout;


EXTERN_C const IID IID_ILayoutStorage;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("0e6d4d90-6738-11cf-9608-00aa00680db4")
    ILayoutStorage : public IUnknown
    {
    public:
        virtual HRESULT __stdcall LayoutScript( 
            /* [annotation][in] */ 
            _In_reads_(nEntries)  StorageLayout *pStorageLayout,
            /* [annotation][in] */ 
            _In_  DWORD nEntries,
            /* [annotation][in] */ 
            _Reserved_  DWORD glfInterleavedFlag) = 0;
        
        virtual HRESULT __stdcall BeginMonitor( void) = 0;
        
        virtual HRESULT __stdcall EndMonitor( void) = 0;
        
        virtual HRESULT __stdcall ReLayoutDocfile( 
            /* [annotation][in] */ 
            __RPC__in  OLECHAR *pwcsNewDfName) = 0;
        
        virtual HRESULT __stdcall ReLayoutDocfileOnILockBytes( 
            /* [annotation][in] */ 
            _In_  ILockBytes *pILockBytes) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ILayoutStorageVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            ILayoutStorage * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            ILayoutStorage * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            ILayoutStorage * This);
        
        DECLSPEC_XFGVIRT(ILayoutStorage, LayoutScript)
        HRESULT ( __stdcall *LayoutScript )( 
            ILayoutStorage * This,
            /* [annotation][in] */ 
            _In_reads_(nEntries)  StorageLayout *pStorageLayout,
            /* [annotation][in] */ 
            _In_  DWORD nEntries,
            /* [annotation][in] */ 
            _Reserved_  DWORD glfInterleavedFlag);
        
        DECLSPEC_XFGVIRT(ILayoutStorage, BeginMonitor)
        HRESULT ( __stdcall *BeginMonitor )( 
            ILayoutStorage * This);
        
        DECLSPEC_XFGVIRT(ILayoutStorage, EndMonitor)
        HRESULT ( __stdcall *EndMonitor )( 
            ILayoutStorage * This);
        
        DECLSPEC_XFGVIRT(ILayoutStorage, ReLayoutDocfile)
        HRESULT ( __stdcall *ReLayoutDocfile )( 
            ILayoutStorage * This,
            /* [annotation][in] */ 
            __RPC__in  OLECHAR *pwcsNewDfName);
        
        DECLSPEC_XFGVIRT(ILayoutStorage, ReLayoutDocfileOnILockBytes)
        HRESULT ( __stdcall *ReLayoutDocfileOnILockBytes )( 
            ILayoutStorage * This,
            /* [annotation][in] */ 
            _In_  ILockBytes *pILockBytes);
        
        END_INTERFACE
    } ILayoutStorageVtbl;

    interface ILayoutStorage
    {
        CONST_VTBL struct ILayoutStorageVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ILayoutStorage_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ILayoutStorage_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ILayoutStorage_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ILayoutStorage_LayoutScript(This,pStorageLayout,nEntries,glfInterleavedFlag)	\
    ( (This)->lpVtbl -> LayoutScript(This,pStorageLayout,nEntries,glfInterleavedFlag) ) 

#define ILayoutStorage_BeginMonitor(This)	\
    ( (This)->lpVtbl -> BeginMonitor(This) ) 

#define ILayoutStorage_EndMonitor(This)	\
    ( (This)->lpVtbl -> EndMonitor(This) ) 

#define ILayoutStorage_ReLayoutDocfile(This,pwcsNewDfName)	\
    ( (This)->lpVtbl -> ReLayoutDocfile(This,pwcsNewDfName) ) 

#define ILayoutStorage_ReLayoutDocfileOnILockBytes(This,pILockBytes)	\
    ( (This)->lpVtbl -> ReLayoutDocfileOnILockBytes(This,pILockBytes) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ILayoutStorage_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_objidl_0000_0089 */
/* [local] */ 

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM) */
#pragma endregion
#pragma region Desktop Family or OneCore Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM)


extern RPC_IF_HANDLE __MIDL_itf_objidl_0000_0089_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_objidl_0000_0089_v0_0_s_ifspec;

#ifndef __IBlockingLock_INTERFACE_DEFINED__
#define __IBlockingLock_INTERFACE_DEFINED__

/* interface IBlockingLock */
/* [uuid][object] */ 


EXTERN_C const IID IID_IBlockingLock;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("30f3d47a-6447-11d1-8e3c-00c04fb9386d")
    IBlockingLock : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE Lock( 
            /* [in] */ DWORD dwTimeout) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Unlock( void) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IBlockingLockVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IBlockingLock * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IBlockingLock * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IBlockingLock * This);
        
        DECLSPEC_XFGVIRT(IBlockingLock, Lock)
        HRESULT ( STDMETHODCALLTYPE *Lock )( 
            __RPC__in IBlockingLock * This,
            /* [in] */ DWORD dwTimeout);
        
        DECLSPEC_XFGVIRT(IBlockingLock, Unlock)
        HRESULT ( STDMETHODCALLTYPE *Unlock )( 
            __RPC__in IBlockingLock * This);
        
        END_INTERFACE
    } IBlockingLockVtbl;

    interface IBlockingLock
    {
        CONST_VTBL struct IBlockingLockVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IBlockingLock_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IBlockingLock_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IBlockingLock_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IBlockingLock_Lock(This,dwTimeout)	\
    ( (This)->lpVtbl -> Lock(This,dwTimeout) ) 

#define IBlockingLock_Unlock(This)	\
    ( (This)->lpVtbl -> Unlock(This) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IBlockingLock_INTERFACE_DEFINED__ */


#ifndef __ITimeAndNoticeControl_INTERFACE_DEFINED__
#define __ITimeAndNoticeControl_INTERFACE_DEFINED__

/* interface ITimeAndNoticeControl */
/* [uuid][object] */ 


EXTERN_C const IID IID_ITimeAndNoticeControl;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("bc0bf6ae-8878-11d1-83e9-00c04fc2c6d4")
    ITimeAndNoticeControl : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE SuppressChanges( 
            /* [in] */ DWORD res1,
            /* [in] */ DWORD res2) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ITimeAndNoticeControlVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in ITimeAndNoticeControl * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in ITimeAndNoticeControl * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in ITimeAndNoticeControl * This);
        
        DECLSPEC_XFGVIRT(ITimeAndNoticeControl, SuppressChanges)
        HRESULT ( STDMETHODCALLTYPE *SuppressChanges )( 
            __RPC__in ITimeAndNoticeControl * This,
            /* [in] */ DWORD res1,
            /* [in] */ DWORD res2);
        
        END_INTERFACE
    } ITimeAndNoticeControlVtbl;

    interface ITimeAndNoticeControl
    {
        CONST_VTBL struct ITimeAndNoticeControlVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ITimeAndNoticeControl_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ITimeAndNoticeControl_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ITimeAndNoticeControl_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ITimeAndNoticeControl_SuppressChanges(This,res1,res2)	\
    ( (This)->lpVtbl -> SuppressChanges(This,res1,res2) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ITimeAndNoticeControl_INTERFACE_DEFINED__ */


#ifndef __IOplockStorage_INTERFACE_DEFINED__
#define __IOplockStorage_INTERFACE_DEFINED__

/* interface IOplockStorage */
/* [uuid][object] */ 


EXTERN_C const IID IID_IOplockStorage;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("8d19c834-8879-11d1-83e9-00c04fc2c6d4")
    IOplockStorage : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE CreateStorageEx( 
            /* [in] */ __RPC__in LPCWSTR pwcsName,
            /* [in] */ DWORD grfMode,
            /* [in] */ DWORD stgfmt,
            /* [in] */ DWORD grfAttrs,
            /* [in] */ __RPC__in REFIID riid,
            /* [iid_is][out] */ __RPC__deref_out_opt void **ppstgOpen) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE OpenStorageEx( 
            /* [in] */ __RPC__in LPCWSTR pwcsName,
            /* [in] */ DWORD grfMode,
            /* [in] */ DWORD stgfmt,
            /* [in] */ DWORD grfAttrs,
            /* [in] */ __RPC__in REFIID riid,
            /* [iid_is][out] */ __RPC__deref_out_opt void **ppstgOpen) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IOplockStorageVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IOplockStorage * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IOplockStorage * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IOplockStorage * This);
        
        DECLSPEC_XFGVIRT(IOplockStorage, CreateStorageEx)
        HRESULT ( STDMETHODCALLTYPE *CreateStorageEx )( 
            __RPC__in IOplockStorage * This,
            /* [in] */ __RPC__in LPCWSTR pwcsName,
            /* [in] */ DWORD grfMode,
            /* [in] */ DWORD stgfmt,
            /* [in] */ DWORD grfAttrs,
            /* [in] */ __RPC__in REFIID riid,
            /* [iid_is][out] */ __RPC__deref_out_opt void **ppstgOpen);
        
        DECLSPEC_XFGVIRT(IOplockStorage, OpenStorageEx)
        HRESULT ( STDMETHODCALLTYPE *OpenStorageEx )( 
            __RPC__in IOplockStorage * This,
            /* [in] */ __RPC__in LPCWSTR pwcsName,
            /* [in] */ DWORD grfMode,
            /* [in] */ DWORD stgfmt,
            /* [in] */ DWORD grfAttrs,
            /* [in] */ __RPC__in REFIID riid,
            /* [iid_is][out] */ __RPC__deref_out_opt void **ppstgOpen);
        
        END_INTERFACE
    } IOplockStorageVtbl;

    interface IOplockStorage
    {
        CONST_VTBL struct IOplockStorageVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IOplockStorage_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IOplockStorage_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IOplockStorage_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IOplockStorage_CreateStorageEx(This,pwcsName,grfMode,stgfmt,grfAttrs,riid,ppstgOpen)	\
    ( (This)->lpVtbl -> CreateStorageEx(This,pwcsName,grfMode,stgfmt,grfAttrs,riid,ppstgOpen) ) 

#define IOplockStorage_OpenStorageEx(This,pwcsName,grfMode,stgfmt,grfAttrs,riid,ppstgOpen)	\
    ( (This)->lpVtbl -> OpenStorageEx(This,pwcsName,grfMode,stgfmt,grfAttrs,riid,ppstgOpen) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IOplockStorage_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_objidl_0000_0092 */
/* [local] */ 

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM) */
#pragma endregion
#pragma region Application Family or OneCore Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM)


extern RPC_IF_HANDLE __MIDL_itf_objidl_0000_0092_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_objidl_0000_0092_v0_0_s_ifspec;

#ifndef __IDirectWriterLock_INTERFACE_DEFINED__
#define __IDirectWriterLock_INTERFACE_DEFINED__

/* interface IDirectWriterLock */
/* [unique][uuid][object] */ 


EXTERN_C const IID IID_IDirectWriterLock;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("0e6d4d92-6738-11cf-9608-00aa00680db4")
    IDirectWriterLock : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE WaitForWriteAccess( 
            /* [in] */ DWORD dwTimeout) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE ReleaseWriteAccess( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE HaveWriteAccess( void) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IDirectWriterLockVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IDirectWriterLock * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IDirectWriterLock * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IDirectWriterLock * This);
        
        DECLSPEC_XFGVIRT(IDirectWriterLock, WaitForWriteAccess)
        HRESULT ( STDMETHODCALLTYPE *WaitForWriteAccess )( 
            __RPC__in IDirectWriterLock * This,
            /* [in] */ DWORD dwTimeout);
        
        DECLSPEC_XFGVIRT(IDirectWriterLock, ReleaseWriteAccess)
        HRESULT ( STDMETHODCALLTYPE *ReleaseWriteAccess )( 
            __RPC__in IDirectWriterLock * This);
        
        DECLSPEC_XFGVIRT(IDirectWriterLock, HaveWriteAccess)
        HRESULT ( STDMETHODCALLTYPE *HaveWriteAccess )( 
            __RPC__in IDirectWriterLock * This);
        
        END_INTERFACE
    } IDirectWriterLockVtbl;

    interface IDirectWriterLock
    {
        CONST_VTBL struct IDirectWriterLockVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IDirectWriterLock_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IDirectWriterLock_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IDirectWriterLock_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IDirectWriterLock_WaitForWriteAccess(This,dwTimeout)	\
    ( (This)->lpVtbl -> WaitForWriteAccess(This,dwTimeout) ) 

#define IDirectWriterLock_ReleaseWriteAccess(This)	\
    ( (This)->lpVtbl -> ReleaseWriteAccess(This) ) 

#define IDirectWriterLock_HaveWriteAccess(This)	\
    ( (This)->lpVtbl -> HaveWriteAccess(This) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IDirectWriterLock_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_objidl_0000_0093 */
/* [local] */ 

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM) */
#pragma endregion
#pragma region Desktop Family or OneCore Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM)


extern RPC_IF_HANDLE __MIDL_itf_objidl_0000_0093_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_objidl_0000_0093_v0_0_s_ifspec;

#ifndef __IUrlMon_INTERFACE_DEFINED__
#define __IUrlMon_INTERFACE_DEFINED__

/* interface IUrlMon */
/* [uuid][object] */ 


EXTERN_C const IID IID_IUrlMon;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("00000026-0000-0000-C000-000000000046")
    IUrlMon : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE AsyncGetClassBits( 
            /* [in] */ __RPC__in REFCLSID rclsid,
            /* [unique][in] */ __RPC__in_opt LPCWSTR pszTYPE,
            /* [unique][in] */ __RPC__in_opt LPCWSTR pszExt,
            /* [in] */ DWORD dwFileVersionMS,
            /* [in] */ DWORD dwFileVersionLS,
            /* [unique][in] */ __RPC__in_opt LPCWSTR pszCodeBase,
            /* [in] */ __RPC__in_opt IBindCtx *pbc,
            /* [in] */ DWORD dwClassContext,
            /* [in] */ __RPC__in REFIID riid,
            /* [in] */ DWORD flags) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IUrlMonVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IUrlMon * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IUrlMon * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IUrlMon * This);
        
        DECLSPEC_XFGVIRT(IUrlMon, AsyncGetClassBits)
        HRESULT ( STDMETHODCALLTYPE *AsyncGetClassBits )( 
            __RPC__in IUrlMon * This,
            /* [in] */ __RPC__in REFCLSID rclsid,
            /* [unique][in] */ __RPC__in_opt LPCWSTR pszTYPE,
            /* [unique][in] */ __RPC__in_opt LPCWSTR pszExt,
            /* [in] */ DWORD dwFileVersionMS,
            /* [in] */ DWORD dwFileVersionLS,
            /* [unique][in] */ __RPC__in_opt LPCWSTR pszCodeBase,
            /* [in] */ __RPC__in_opt IBindCtx *pbc,
            /* [in] */ DWORD dwClassContext,
            /* [in] */ __RPC__in REFIID riid,
            /* [in] */ DWORD flags);
        
        END_INTERFACE
    } IUrlMonVtbl;

    interface IUrlMon
    {
        CONST_VTBL struct IUrlMonVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IUrlMon_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IUrlMon_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IUrlMon_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IUrlMon_AsyncGetClassBits(This,rclsid,pszTYPE,pszExt,dwFileVersionMS,dwFileVersionLS,pszCodeBase,pbc,dwClassContext,riid,flags)	\
    ( (This)->lpVtbl -> AsyncGetClassBits(This,rclsid,pszTYPE,pszExt,dwFileVersionMS,dwFileVersionLS,pszCodeBase,pbc,dwClassContext,riid,flags) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IUrlMon_INTERFACE_DEFINED__ */


#ifndef __IForegroundTransfer_INTERFACE_DEFINED__
#define __IForegroundTransfer_INTERFACE_DEFINED__

/* interface IForegroundTransfer */
/* [uuid][object][local] */ 


EXTERN_C const IID IID_IForegroundTransfer;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("00000145-0000-0000-C000-000000000046")
    IForegroundTransfer : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE AllowForegroundTransfer( 
            /* [annotation][in] */ 
            _Reserved_  void *lpvReserved) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IForegroundTransferVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IForegroundTransfer * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IForegroundTransfer * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IForegroundTransfer * This);
        
        DECLSPEC_XFGVIRT(IForegroundTransfer, AllowForegroundTransfer)
        HRESULT ( STDMETHODCALLTYPE *AllowForegroundTransfer )( 
            IForegroundTransfer * This,
            /* [annotation][in] */ 
            _Reserved_  void *lpvReserved);
        
        END_INTERFACE
    } IForegroundTransferVtbl;

    interface IForegroundTransfer
    {
        CONST_VTBL struct IForegroundTransferVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IForegroundTransfer_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IForegroundTransfer_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IForegroundTransfer_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IForegroundTransfer_AllowForegroundTransfer(This,lpvReserved)	\
    ( (This)->lpVtbl -> AllowForegroundTransfer(This,lpvReserved) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IForegroundTransfer_INTERFACE_DEFINED__ */


#ifndef __IThumbnailExtractor_INTERFACE_DEFINED__
#define __IThumbnailExtractor_INTERFACE_DEFINED__

/* interface IThumbnailExtractor */
/* [object][uuid] */ 


EXTERN_C const IID IID_IThumbnailExtractor;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("969dc708-5c76-11d1-8d86-0000f804b057")
    IThumbnailExtractor : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE ExtractThumbnail( 
            /* [in] */ __RPC__in_opt IStorage *pStg,
            /* [in] */ ULONG ulLength,
            /* [in] */ ULONG ulHeight,
            /* [out] */ __RPC__out ULONG *pulOutputLength,
            /* [out] */ __RPC__out ULONG *pulOutputHeight,
            /* [out] */ __RPC__deref_out_opt HBITMAP *phOutputBitmap) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE OnFileUpdated( 
            /* [in] */ __RPC__in_opt IStorage *pStg) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IThumbnailExtractorVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IThumbnailExtractor * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IThumbnailExtractor * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IThumbnailExtractor * This);
        
        DECLSPEC_XFGVIRT(IThumbnailExtractor, ExtractThumbnail)
        HRESULT ( STDMETHODCALLTYPE *ExtractThumbnail )( 
            __RPC__in IThumbnailExtractor * This,
            /* [in] */ __RPC__in_opt IStorage *pStg,
            /* [in] */ ULONG ulLength,
            /* [in] */ ULONG ulHeight,
            /* [out] */ __RPC__out ULONG *pulOutputLength,
            /* [out] */ __RPC__out ULONG *pulOutputHeight,
            /* [out] */ __RPC__deref_out_opt HBITMAP *phOutputBitmap);
        
        DECLSPEC_XFGVIRT(IThumbnailExtractor, OnFileUpdated)
        HRESULT ( STDMETHODCALLTYPE *OnFileUpdated )( 
            __RPC__in IThumbnailExtractor * This,
            /* [in] */ __RPC__in_opt IStorage *pStg);
        
        END_INTERFACE
    } IThumbnailExtractorVtbl;

    interface IThumbnailExtractor
    {
        CONST_VTBL struct IThumbnailExtractorVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IThumbnailExtractor_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IThumbnailExtractor_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IThumbnailExtractor_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IThumbnailExtractor_ExtractThumbnail(This,pStg,ulLength,ulHeight,pulOutputLength,pulOutputHeight,phOutputBitmap)	\
    ( (This)->lpVtbl -> ExtractThumbnail(This,pStg,ulLength,ulHeight,pulOutputLength,pulOutputHeight,phOutputBitmap) ) 

#define IThumbnailExtractor_OnFileUpdated(This,pStg)	\
    ( (This)->lpVtbl -> OnFileUpdated(This,pStg) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IThumbnailExtractor_INTERFACE_DEFINED__ */


#ifndef __IDummyHICONIncluder_INTERFACE_DEFINED__
#define __IDummyHICONIncluder_INTERFACE_DEFINED__

/* interface IDummyHICONIncluder */
/* [uuid][unique][object] */ 


EXTERN_C const IID IID_IDummyHICONIncluder;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("947990de-cc28-11d2-a0f7-00805f858fb1")
    IDummyHICONIncluder : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE Dummy( 
            /* [in] */ __RPC__in HICON h1,
            /* [in] */ __RPC__in HDC h2) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IDummyHICONIncluderVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IDummyHICONIncluder * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IDummyHICONIncluder * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IDummyHICONIncluder * This);
        
        DECLSPEC_XFGVIRT(IDummyHICONIncluder, Dummy)
        HRESULT ( STDMETHODCALLTYPE *Dummy )( 
            __RPC__in IDummyHICONIncluder * This,
            /* [in] */ __RPC__in HICON h1,
            /* [in] */ __RPC__in HDC h2);
        
        END_INTERFACE
    } IDummyHICONIncluderVtbl;

    interface IDummyHICONIncluder
    {
        CONST_VTBL struct IDummyHICONIncluderVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IDummyHICONIncluder_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IDummyHICONIncluder_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IDummyHICONIncluder_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IDummyHICONIncluder_Dummy(This,h1,h2)	\
    ( (This)->lpVtbl -> Dummy(This,h1,h2) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IDummyHICONIncluder_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_objidl_0000_0097 */
/* [local] */ 

typedef 
enum tagApplicationType
    {
        ServerApplication	= 0,
        LibraryApplication	= ( ServerApplication + 1 ) 
    } 	ApplicationType;

typedef 
enum tagShutdownType
    {
        IdleShutdown	= 0,
        ForcedShutdown	= ( IdleShutdown + 1 ) 
    } 	ShutdownType;



extern RPC_IF_HANDLE __MIDL_itf_objidl_0000_0097_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_objidl_0000_0097_v0_0_s_ifspec;

#ifndef __IProcessLock_INTERFACE_DEFINED__
#define __IProcessLock_INTERFACE_DEFINED__

/* interface IProcessLock */
/* [unique][uuid][local][object] */ 


EXTERN_C const IID IID_IProcessLock;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("000001d5-0000-0000-C000-000000000046")
    IProcessLock : public IUnknown
    {
    public:
        virtual ULONG STDMETHODCALLTYPE AddRefOnProcess( void) = 0;
        
        virtual ULONG STDMETHODCALLTYPE ReleaseRefOnProcess( void) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IProcessLockVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IProcessLock * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IProcessLock * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IProcessLock * This);
        
        DECLSPEC_XFGVIRT(IProcessLock, AddRefOnProcess)
        ULONG ( STDMETHODCALLTYPE *AddRefOnProcess )( 
            IProcessLock * This);
        
        DECLSPEC_XFGVIRT(IProcessLock, ReleaseRefOnProcess)
        ULONG ( STDMETHODCALLTYPE *ReleaseRefOnProcess )( 
            IProcessLock * This);
        
        END_INTERFACE
    } IProcessLockVtbl;

    interface IProcessLock
    {
        CONST_VTBL struct IProcessLockVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IProcessLock_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IProcessLock_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IProcessLock_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IProcessLock_AddRefOnProcess(This)	\
    ( (This)->lpVtbl -> AddRefOnProcess(This) ) 

#define IProcessLock_ReleaseRefOnProcess(This)	\
    ( (This)->lpVtbl -> ReleaseRefOnProcess(This) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IProcessLock_INTERFACE_DEFINED__ */


#ifndef __ISurrogateService_INTERFACE_DEFINED__
#define __ISurrogateService_INTERFACE_DEFINED__

/* interface ISurrogateService */
/* [unique][uuid][local][object] */ 


EXTERN_C const IID IID_ISurrogateService;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("000001d4-0000-0000-C000-000000000046")
    ISurrogateService : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE Init( 
            /* [annotation][in] */ 
            _In_  REFGUID rguidProcessID,
            /* [annotation][in] */ 
            _In_  IProcessLock *pProcessLock,
            /* [annotation][out] */ 
            _Out_  BOOL *pfApplicationAware) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE ApplicationLaunch( 
            /* [annotation][in] */ 
            _In_  REFGUID rguidApplID,
            /* [annotation][in] */ 
            _In_  ApplicationType appType) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE ApplicationFree( 
            /* [annotation][in] */ 
            _In_  REFGUID rguidApplID) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE CatalogRefresh( 
            /* [annotation][in] */ 
            _Reserved_  ULONG ulReserved) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE ProcessShutdown( 
            /* [annotation][in] */ 
            _In_  ShutdownType shutdownType) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ISurrogateServiceVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            ISurrogateService * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            ISurrogateService * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            ISurrogateService * This);
        
        DECLSPEC_XFGVIRT(ISurrogateService, Init)
        HRESULT ( STDMETHODCALLTYPE *Init )( 
            ISurrogateService * This,
            /* [annotation][in] */ 
            _In_  REFGUID rguidProcessID,
            /* [annotation][in] */ 
            _In_  IProcessLock *pProcessLock,
            /* [annotation][out] */ 
            _Out_  BOOL *pfApplicationAware);
        
        DECLSPEC_XFGVIRT(ISurrogateService, ApplicationLaunch)
        HRESULT ( STDMETHODCALLTYPE *ApplicationLaunch )( 
            ISurrogateService * This,
            /* [annotation][in] */ 
            _In_  REFGUID rguidApplID,
            /* [annotation][in] */ 
            _In_  ApplicationType appType);
        
        DECLSPEC_XFGVIRT(ISurrogateService, ApplicationFree)
        HRESULT ( STDMETHODCALLTYPE *ApplicationFree )( 
            ISurrogateService * This,
            /* [annotation][in] */ 
            _In_  REFGUID rguidApplID);
        
        DECLSPEC_XFGVIRT(ISurrogateService, CatalogRefresh)
        HRESULT ( STDMETHODCALLTYPE *CatalogRefresh )( 
            ISurrogateService * This,
            /* [annotation][in] */ 
            _Reserved_  ULONG ulReserved);
        
        DECLSPEC_XFGVIRT(ISurrogateService, ProcessShutdown)
        HRESULT ( STDMETHODCALLTYPE *ProcessShutdown )( 
            ISurrogateService * This,
            /* [annotation][in] */ 
            _In_  ShutdownType shutdownType);
        
        END_INTERFACE
    } ISurrogateServiceVtbl;

    interface ISurrogateService
    {
        CONST_VTBL struct ISurrogateServiceVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ISurrogateService_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ISurrogateService_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ISurrogateService_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ISurrogateService_Init(This,rguidProcessID,pProcessLock,pfApplicationAware)	\
    ( (This)->lpVtbl -> Init(This,rguidProcessID,pProcessLock,pfApplicationAware) ) 

#define ISurrogateService_ApplicationLaunch(This,rguidApplID,appType)	\
    ( (This)->lpVtbl -> ApplicationLaunch(This,rguidApplID,appType) ) 

#define ISurrogateService_ApplicationFree(This,rguidApplID)	\
    ( (This)->lpVtbl -> ApplicationFree(This,rguidApplID) ) 

#define ISurrogateService_CatalogRefresh(This,ulReserved)	\
    ( (This)->lpVtbl -> CatalogRefresh(This,ulReserved) ) 

#define ISurrogateService_ProcessShutdown(This,shutdownType)	\
    ( (This)->lpVtbl -> ProcessShutdown(This,shutdownType) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ISurrogateService_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_objidl_0000_0099 */
/* [local] */ 

#if  (_WIN32_WINNT >= 0x0501 )


extern RPC_IF_HANDLE __MIDL_itf_objidl_0000_0099_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_objidl_0000_0099_v0_0_s_ifspec;

#ifndef __IInitializeSpy_INTERFACE_DEFINED__
#define __IInitializeSpy_INTERFACE_DEFINED__

/* interface IInitializeSpy */
/* [unique][uuid][object][local] */ 

typedef /* [unique] */ IInitializeSpy *LPINITIALIZESPY;


EXTERN_C const IID IID_IInitializeSpy;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("00000034-0000-0000-C000-000000000046")
    IInitializeSpy : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE PreInitialize( 
            /* [annotation][in] */ 
            _In_  DWORD dwCoInit,
            /* [annotation][in] */ 
            _In_  DWORD dwCurThreadAptRefs) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE PostInitialize( 
            /* [annotation][in] */ 
            _In_  HRESULT hrCoInit,
            /* [annotation][in] */ 
            _In_  DWORD dwCoInit,
            /* [annotation][in] */ 
            _In_  DWORD dwNewThreadAptRefs) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE PreUninitialize( 
            /* [annotation][in] */ 
            _In_  DWORD dwCurThreadAptRefs) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE PostUninitialize( 
            /* [annotation][in] */ 
            _In_  DWORD dwNewThreadAptRefs) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IInitializeSpyVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IInitializeSpy * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IInitializeSpy * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IInitializeSpy * This);
        
        DECLSPEC_XFGVIRT(IInitializeSpy, PreInitialize)
        HRESULT ( STDMETHODCALLTYPE *PreInitialize )( 
            IInitializeSpy * This,
            /* [annotation][in] */ 
            _In_  DWORD dwCoInit,
            /* [annotation][in] */ 
            _In_  DWORD dwCurThreadAptRefs);
        
        DECLSPEC_XFGVIRT(IInitializeSpy, PostInitialize)
        HRESULT ( STDMETHODCALLTYPE *PostInitialize )( 
            IInitializeSpy * This,
            /* [annotation][in] */ 
            _In_  HRESULT hrCoInit,
            /* [annotation][in] */ 
            _In_  DWORD dwCoInit,
            /* [annotation][in] */ 
            _In_  DWORD dwNewThreadAptRefs);
        
        DECLSPEC_XFGVIRT(IInitializeSpy, PreUninitialize)
        HRESULT ( STDMETHODCALLTYPE *PreUninitialize )( 
            IInitializeSpy * This,
            /* [annotation][in] */ 
            _In_  DWORD dwCurThreadAptRefs);
        
        DECLSPEC_XFGVIRT(IInitializeSpy, PostUninitialize)
        HRESULT ( STDMETHODCALLTYPE *PostUninitialize )( 
            IInitializeSpy * This,
            /* [annotation][in] */ 
            _In_  DWORD dwNewThreadAptRefs);
        
        END_INTERFACE
    } IInitializeSpyVtbl;

    interface IInitializeSpy
    {
        CONST_VTBL struct IInitializeSpyVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IInitializeSpy_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IInitializeSpy_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IInitializeSpy_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IInitializeSpy_PreInitialize(This,dwCoInit,dwCurThreadAptRefs)	\
    ( (This)->lpVtbl -> PreInitialize(This,dwCoInit,dwCurThreadAptRefs) ) 

#define IInitializeSpy_PostInitialize(This,hrCoInit,dwCoInit,dwNewThreadAptRefs)	\
    ( (This)->lpVtbl -> PostInitialize(This,hrCoInit,dwCoInit,dwNewThreadAptRefs) ) 

#define IInitializeSpy_PreUninitialize(This,dwCurThreadAptRefs)	\
    ( (This)->lpVtbl -> PreUninitialize(This,dwCurThreadAptRefs) ) 

#define IInitializeSpy_PostUninitialize(This,dwNewThreadAptRefs)	\
    ( (This)->lpVtbl -> PostUninitialize(This,dwNewThreadAptRefs) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IInitializeSpy_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_objidl_0000_0100 */
/* [local] */ 

#endif
#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM) */
#pragma endregion
#pragma region Application Family or OneCore Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM)


extern RPC_IF_HANDLE __MIDL_itf_objidl_0000_0100_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_objidl_0000_0100_v0_0_s_ifspec;

#ifndef __IApartmentShutdown_INTERFACE_DEFINED__
#define __IApartmentShutdown_INTERFACE_DEFINED__

/* interface IApartmentShutdown */
/* [unique][uuid][object][local] */ 


EXTERN_C const IID IID_IApartmentShutdown;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("A2F05A09-27A2-42B5-BC0E-AC163EF49D9B")
    IApartmentShutdown : public IUnknown
    {
    public:
        virtual void STDMETHODCALLTYPE OnUninitialize( 
            /* [annotation][in] */ 
            _In_  UINT64 ui64ApartmentIdentifier) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IApartmentShutdownVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IApartmentShutdown * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IApartmentShutdown * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IApartmentShutdown * This);
        
        DECLSPEC_XFGVIRT(IApartmentShutdown, OnUninitialize)
        void ( STDMETHODCALLTYPE *OnUninitialize )( 
            IApartmentShutdown * This,
            /* [annotation][in] */ 
            _In_  UINT64 ui64ApartmentIdentifier);
        
        END_INTERFACE
    } IApartmentShutdownVtbl;

    interface IApartmentShutdown
    {
        CONST_VTBL struct IApartmentShutdownVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IApartmentShutdown_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IApartmentShutdown_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IApartmentShutdown_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IApartmentShutdown_OnUninitialize(This,ui64ApartmentIdentifier)	\
    ( (This)->lpVtbl -> OnUninitialize(This,ui64ApartmentIdentifier) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IApartmentShutdown_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_objidl_0000_0101 */
/* [local] */ 

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM) */
#pragma endregion
#if ( _MSC_VER >= 800 )
#if _MSC_VER >= 1200
#pragma warning(pop)
#else
#pragma warning(default:4201)
#endif
#endif


extern RPC_IF_HANDLE __MIDL_itf_objidl_0000_0101_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_objidl_0000_0101_v0_0_s_ifspec;

/* Additional Prototypes for ALL interfaces */

unsigned long             __RPC_USER  ASYNC_STGMEDIUM_UserSize(     __RPC__in unsigned long *, unsigned long            , __RPC__in ASYNC_STGMEDIUM * ); 
unsigned char * __RPC_USER  ASYNC_STGMEDIUM_UserMarshal(  __RPC__in unsigned long *, __RPC__inout_xcount(0) unsigned char *, __RPC__in ASYNC_STGMEDIUM * ); 
unsigned char * __RPC_USER  ASYNC_STGMEDIUM_UserUnmarshal(__RPC__in unsigned long *, __RPC__in_xcount(0) unsigned char *, __RPC__out ASYNC_STGMEDIUM * ); 
void                      __RPC_USER  ASYNC_STGMEDIUM_UserFree(     __RPC__in unsigned long *, __RPC__in ASYNC_STGMEDIUM * ); 

unsigned long             __RPC_USER  CLIPFORMAT_UserSize(     __RPC__in unsigned long *, unsigned long            , __RPC__in CLIPFORMAT * ); 
unsigned char * __RPC_USER  CLIPFORMAT_UserMarshal(  __RPC__in unsigned long *, __RPC__inout_xcount(0) unsigned char *, __RPC__in CLIPFORMAT * ); 
unsigned char * __RPC_USER  CLIPFORMAT_UserUnmarshal(__RPC__in unsigned long *, __RPC__in_xcount(0) unsigned char *, __RPC__out CLIPFORMAT * ); 
void                      __RPC_USER  CLIPFORMAT_UserFree(     __RPC__in unsigned long *, __RPC__in CLIPFORMAT * ); 

unsigned long             __RPC_USER  FLAG_STGMEDIUM_UserSize(     __RPC__in unsigned long *, unsigned long            , __RPC__in FLAG_STGMEDIUM * ); 
unsigned char * __RPC_USER  FLAG_STGMEDIUM_UserMarshal(  __RPC__in unsigned long *, __RPC__inout_xcount(0) unsigned char *, __RPC__in FLAG_STGMEDIUM * ); 
unsigned char * __RPC_USER  FLAG_STGMEDIUM_UserUnmarshal(__RPC__in unsigned long *, __RPC__in_xcount(0) unsigned char *, __RPC__out FLAG_STGMEDIUM * ); 
void                      __RPC_USER  FLAG_STGMEDIUM_UserFree(     __RPC__in unsigned long *, __RPC__in FLAG_STGMEDIUM * ); 

unsigned long             __RPC_USER  HBITMAP_UserSize(     __RPC__in unsigned long *, unsigned long            , __RPC__in HBITMAP * ); 
unsigned char * __RPC_USER  HBITMAP_UserMarshal(  __RPC__in unsigned long *, __RPC__inout_xcount(0) unsigned char *, __RPC__in HBITMAP * ); 
unsigned char * __RPC_USER  HBITMAP_UserUnmarshal(__RPC__in unsigned long *, __RPC__in_xcount(0) unsigned char *, __RPC__out HBITMAP * ); 
void                      __RPC_USER  HBITMAP_UserFree(     __RPC__in unsigned long *, __RPC__in HBITMAP * ); 

unsigned long             __RPC_USER  HDC_UserSize(     __RPC__in unsigned long *, unsigned long            , __RPC__in HDC * ); 
unsigned char * __RPC_USER  HDC_UserMarshal(  __RPC__in unsigned long *, __RPC__inout_xcount(0) unsigned char *, __RPC__in HDC * ); 
unsigned char * __RPC_USER  HDC_UserUnmarshal(__RPC__in unsigned long *, __RPC__in_xcount(0) unsigned char *, __RPC__out HDC * ); 
void                      __RPC_USER  HDC_UserFree(     __RPC__in unsigned long *, __RPC__in HDC * ); 

unsigned long             __RPC_USER  HICON_UserSize(     __RPC__in unsigned long *, unsigned long            , __RPC__in HICON * ); 
unsigned char * __RPC_USER  HICON_UserMarshal(  __RPC__in unsigned long *, __RPC__inout_xcount(0) unsigned char *, __RPC__in HICON * ); 
unsigned char * __RPC_USER  HICON_UserUnmarshal(__RPC__in unsigned long *, __RPC__in_xcount(0) unsigned char *, __RPC__out HICON * ); 
void                      __RPC_USER  HICON_UserFree(     __RPC__in unsigned long *, __RPC__in HICON * ); 

unsigned long             __RPC_USER  SNB_UserSize(     __RPC__in unsigned long *, unsigned long            , __RPC__in SNB * ); 
unsigned char * __RPC_USER  SNB_UserMarshal(  __RPC__in unsigned long *, __RPC__inout_xcount(0) unsigned char *, __RPC__in SNB * ); 
unsigned char * __RPC_USER  SNB_UserUnmarshal(__RPC__in unsigned long *, __RPC__in_xcount(0) unsigned char *, __RPC__out SNB * ); 
void                      __RPC_USER  SNB_UserFree(     __RPC__in unsigned long *, __RPC__in SNB * ); 

unsigned long             __RPC_USER  STGMEDIUM_UserSize(     __RPC__in unsigned long *, unsigned long            , __RPC__in STGMEDIUM * ); 
unsigned char * __RPC_USER  STGMEDIUM_UserMarshal(  __RPC__in unsigned long *, __RPC__inout_xcount(0) unsigned char *, __RPC__in STGMEDIUM * ); 
unsigned char * __RPC_USER  STGMEDIUM_UserUnmarshal(__RPC__in unsigned long *, __RPC__in_xcount(0) unsigned char *, __RPC__out STGMEDIUM * ); 
void                      __RPC_USER  STGMEDIUM_UserFree(     __RPC__in unsigned long *, __RPC__in STGMEDIUM * ); 

unsigned long             __RPC_USER  ASYNC_STGMEDIUM_UserSize64(     __RPC__in unsigned long *, unsigned long            , __RPC__in ASYNC_STGMEDIUM * ); 
unsigned char * __RPC_USER  ASYNC_STGMEDIUM_UserMarshal64(  __RPC__in unsigned long *, __RPC__inout_xcount(0) unsigned char *, __RPC__in ASYNC_STGMEDIUM * ); 
unsigned char * __RPC_USER  ASYNC_STGMEDIUM_UserUnmarshal64(__RPC__in unsigned long *, __RPC__in_xcount(0) unsigned char *, __RPC__out ASYNC_STGMEDIUM * ); 
void                      __RPC_USER  ASYNC_STGMEDIUM_UserFree64(     __RPC__in unsigned long *, __RPC__in ASYNC_STGMEDIUM * ); 

unsigned long             __RPC_USER  CLIPFORMAT_UserSize64(     __RPC__in unsigned long *, unsigned long            , __RPC__in CLIPFORMAT * ); 
unsigned char * __RPC_USER  CLIPFORMAT_UserMarshal64(  __RPC__in unsigned long *, __RPC__inout_xcount(0) unsigned char *, __RPC__in CLIPFORMAT * ); 
unsigned char * __RPC_USER  CLIPFORMAT_UserUnmarshal64(__RPC__in unsigned long *, __RPC__in_xcount(0) unsigned char *, __RPC__out CLIPFORMAT * ); 
void                      __RPC_USER  CLIPFORMAT_UserFree64(     __RPC__in unsigned long *, __RPC__in CLIPFORMAT * ); 

unsigned long             __RPC_USER  FLAG_STGMEDIUM_UserSize64(     __RPC__in unsigned long *, unsigned long            , __RPC__in FLAG_STGMEDIUM * ); 
unsigned char * __RPC_USER  FLAG_STGMEDIUM_UserMarshal64(  __RPC__in unsigned long *, __RPC__inout_xcount(0) unsigned char *, __RPC__in FLAG_STGMEDIUM * ); 
unsigned char * __RPC_USER  FLAG_STGMEDIUM_UserUnmarshal64(__RPC__in unsigned long *, __RPC__in_xcount(0) unsigned char *, __RPC__out FLAG_STGMEDIUM * ); 
void                      __RPC_USER  FLAG_STGMEDIUM_UserFree64(     __RPC__in unsigned long *, __RPC__in FLAG_STGMEDIUM * ); 

unsigned long             __RPC_USER  HBITMAP_UserSize64(     __RPC__in unsigned long *, unsigned long            , __RPC__in HBITMAP * ); 
unsigned char * __RPC_USER  HBITMAP_UserMarshal64(  __RPC__in unsigned long *, __RPC__inout_xcount(0) unsigned char *, __RPC__in HBITMAP * ); 
unsigned char * __RPC_USER  HBITMAP_UserUnmarshal64(__RPC__in unsigned long *, __RPC__in_xcount(0) unsigned char *, __RPC__out HBITMAP * ); 
void                      __RPC_USER  HBITMAP_UserFree64(     __RPC__in unsigned long *, __RPC__in HBITMAP * ); 

unsigned long             __RPC_USER  HDC_UserSize64(     __RPC__in unsigned long *, unsigned long            , __RPC__in HDC * ); 
unsigned char * __RPC_USER  HDC_UserMarshal64(  __RPC__in unsigned long *, __RPC__inout_xcount(0) unsigned char *, __RPC__in HDC * ); 
unsigned char * __RPC_USER  HDC_UserUnmarshal64(__RPC__in unsigned long *, __RPC__in_xcount(0) unsigned char *, __RPC__out HDC * ); 
void                      __RPC_USER  HDC_UserFree64(     __RPC__in unsigned long *, __RPC__in HDC * ); 

unsigned long             __RPC_USER  HICON_UserSize64(     __RPC__in unsigned long *, unsigned long            , __RPC__in HICON * ); 
unsigned char * __RPC_USER  HICON_UserMarshal64(  __RPC__in unsigned long *, __RPC__inout_xcount(0) unsigned char *, __RPC__in HICON * ); 
unsigned char * __RPC_USER  HICON_UserUnmarshal64(__RPC__in unsigned long *, __RPC__in_xcount(0) unsigned char *, __RPC__out HICON * ); 
void                      __RPC_USER  HICON_UserFree64(     __RPC__in unsigned long *, __RPC__in HICON * ); 

unsigned long             __RPC_USER  SNB_UserSize64(     __RPC__in unsigned long *, unsigned long            , __RPC__in SNB * ); 
unsigned char * __RPC_USER  SNB_UserMarshal64(  __RPC__in unsigned long *, __RPC__inout_xcount(0) unsigned char *, __RPC__in SNB * ); 
unsigned char * __RPC_USER  SNB_UserUnmarshal64(__RPC__in unsigned long *, __RPC__in_xcount(0) unsigned char *, __RPC__out SNB * ); 
void                      __RPC_USER  SNB_UserFree64(     __RPC__in unsigned long *, __RPC__in SNB * ); 

unsigned long             __RPC_USER  STGMEDIUM_UserSize64(     __RPC__in unsigned long *, unsigned long            , __RPC__in STGMEDIUM * ); 
unsigned char * __RPC_USER  STGMEDIUM_UserMarshal64(  __RPC__in unsigned long *, __RPC__inout_xcount(0) unsigned char *, __RPC__in STGMEDIUM * ); 
unsigned char * __RPC_USER  STGMEDIUM_UserUnmarshal64(__RPC__in unsigned long *, __RPC__in_xcount(0) unsigned char *, __RPC__out STGMEDIUM * ); 
void                      __RPC_USER  STGMEDIUM_UserFree64(     __RPC__in unsigned long *, __RPC__in STGMEDIUM * ); 

/* [local] */ HRESULT STDMETHODCALLTYPE IEnumUnknown_Next_Proxy( 
    IEnumUnknown * This,
    /* [annotation][in] */ 
    _In_  ULONG celt,
    /* [annotation][out] */ 
    _Out_writes_to_(celt,*pceltFetched)  IUnknown **rgelt,
    /* [annotation][out] */ 
    _Out_opt_  ULONG *pceltFetched);


/* [call_as] */ HRESULT STDMETHODCALLTYPE IEnumUnknown_Next_Stub( 
    __RPC__in IEnumUnknown * This,
    /* [in] */ ULONG celt,
    /* [length_is][size_is][out] */ __RPC__out_ecount_part(celt, *pceltFetched) IUnknown **rgelt,
    /* [out] */ __RPC__out ULONG *pceltFetched);

/* [local] */ HRESULT STDMETHODCALLTYPE IEnumString_Next_Proxy( 
    IEnumString * This,
    /* [in] */ ULONG celt,
    /* [annotation] */ 
    _Out_writes_to_(celt,*pceltFetched)  LPOLESTR *rgelt,
    /* [annotation] */ 
    _Out_opt_  ULONG *pceltFetched);


/* [call_as] */ HRESULT STDMETHODCALLTYPE IEnumString_Next_Stub( 
    __RPC__in IEnumString * This,
    /* [in] */ ULONG celt,
    /* [length_is][size_is][out] */ __RPC__out_ecount_part(celt, *pceltFetched) LPOLESTR *rgelt,
    /* [out] */ __RPC__out ULONG *pceltFetched);

/* [local] */ HRESULT STDMETHODCALLTYPE ISequentialStream_Read_Proxy( 
    ISequentialStream * This,
    /* [annotation] */ 
    _Out_writes_bytes_to_(cb, *pcbRead)  void *pv,
    /* [annotation][in] */ 
    _In_  ULONG cb,
    /* [annotation] */ 
    _Out_opt_  ULONG *pcbRead);


/* [call_as] */ HRESULT STDMETHODCALLTYPE ISequentialStream_Read_Stub( 
    __RPC__in ISequentialStream * This,
    /* [length_is][size_is][out] */ __RPC__out_ecount_part(cb, *pcbRead) byte *pv,
    /* [in] */ ULONG cb,
    /* [out] */ __RPC__out ULONG *pcbRead);

/* [local] */ HRESULT STDMETHODCALLTYPE ISequentialStream_Write_Proxy( 
    ISequentialStream * This,
    /* [annotation] */ 
    _In_reads_bytes_(cb)  const void *pv,
    /* [annotation][in] */ 
    _In_  ULONG cb,
    /* [annotation] */ 
    _Out_opt_  ULONG *pcbWritten);


/* [call_as] */ HRESULT STDMETHODCALLTYPE ISequentialStream_Write_Stub( 
    __RPC__in ISequentialStream * This,
    /* [size_is][in] */ __RPC__in_ecount_full(cb) const byte *pv,
    /* [in] */ ULONG cb,
    /* [out] */ __RPC__out ULONG *pcbWritten);

/* [local] */ HRESULT STDMETHODCALLTYPE IStream_Seek_Proxy( 
    IStream * This,
    /* [in] */ LARGE_INTEGER dlibMove,
    /* [in] */ DWORD dwOrigin,
    /* [annotation] */ 
    _Out_opt_  ULARGE_INTEGER *plibNewPosition);


/* [call_as] */ HRESULT STDMETHODCALLTYPE IStream_Seek_Stub( 
    __RPC__in IStream * This,
    /* [in] */ LARGE_INTEGER dlibMove,
    /* [in] */ DWORD dwOrigin,
    /* [out] */ __RPC__out ULARGE_INTEGER *plibNewPosition);

/* [local] */ HRESULT STDMETHODCALLTYPE IStream_CopyTo_Proxy( 
    IStream * This,
    /* [annotation][unique][in] */ 
    _In_  IStream *pstm,
    /* [in] */ ULARGE_INTEGER cb,
    /* [annotation] */ 
    _Out_opt_  ULARGE_INTEGER *pcbRead,
    /* [annotation] */ 
    _Out_opt_  ULARGE_INTEGER *pcbWritten);


/* [call_as] */ HRESULT STDMETHODCALLTYPE IStream_CopyTo_Stub( 
    __RPC__in IStream * This,
    /* [unique][in] */ __RPC__in_opt IStream *pstm,
    /* [in] */ ULARGE_INTEGER cb,
    /* [out] */ __RPC__out ULARGE_INTEGER *pcbRead,
    /* [out] */ __RPC__out ULARGE_INTEGER *pcbWritten);

/* [local] */ HRESULT STDMETHODCALLTYPE IBindCtx_SetBindOptions_Proxy( 
    IBindCtx * This,
    /* [annotation][in] */ 
    _In_  BIND_OPTS *pbindopts);


/* [call_as] */ HRESULT STDMETHODCALLTYPE IBindCtx_SetBindOptions_Stub( 
    __RPC__in IBindCtx * This,
    /* [in] */ __RPC__in BIND_OPTS2 *pbindopts);

/* [local] */ HRESULT STDMETHODCALLTYPE IBindCtx_GetBindOptions_Proxy( 
    IBindCtx * This,
    /* [annotation][out][in] */ 
    _Inout_  BIND_OPTS *pbindopts);


/* [call_as] */ HRESULT STDMETHODCALLTYPE IBindCtx_GetBindOptions_Stub( 
    __RPC__in IBindCtx * This,
    /* [out][in] */ __RPC__inout BIND_OPTS2 *pbindopts);

/* [local] */ HRESULT STDMETHODCALLTYPE IEnumMoniker_Next_Proxy( 
    IEnumMoniker * This,
    /* [in] */ ULONG celt,
    /* [annotation] */ 
    _Out_writes_to_(celt,*pceltFetched)  IMoniker **rgelt,
    /* [annotation] */ 
    _Out_opt_  ULONG *pceltFetched);


/* [call_as] */ HRESULT STDMETHODCALLTYPE IEnumMoniker_Next_Stub( 
    __RPC__in IEnumMoniker * This,
    /* [in] */ ULONG celt,
    /* [length_is][size_is][out] */ __RPC__out_ecount_part(celt, *pceltFetched) IMoniker **rgelt,
    /* [out] */ __RPC__out ULONG *pceltFetched);

/* [local] */ BOOL STDMETHODCALLTYPE IRunnableObject_IsRunning_Proxy( 
    IRunnableObject * This);


/* [call_as] */ HRESULT STDMETHODCALLTYPE IRunnableObject_IsRunning_Stub( 
    __RPC__in IRunnableObject * This);

/* [local] */ HRESULT STDMETHODCALLTYPE IMoniker_BindToObject_Proxy( 
    IMoniker * This,
    /* [annotation][unique][in] */ 
    _In_  IBindCtx *pbc,
    /* [annotation][unique][in] */ 
    _In_opt_  IMoniker *pmkToLeft,
    /* [annotation][in] */ 
    _In_  REFIID riidResult,
    /* [annotation][iid_is][out] */ 
    _Outptr_  void **ppvResult);


/* [call_as] */ HRESULT STDMETHODCALLTYPE IMoniker_BindToObject_Stub( 
    __RPC__in IMoniker * This,
    /* [unique][in] */ __RPC__in_opt IBindCtx *pbc,
    /* [unique][in] */ __RPC__in_opt IMoniker *pmkToLeft,
    /* [in] */ __RPC__in REFIID riidResult,
    /* [iid_is][out] */ __RPC__deref_out_opt IUnknown **ppvResult);

/* [local] */ HRESULT STDMETHODCALLTYPE IMoniker_BindToStorage_Proxy( 
    IMoniker * This,
    /* [annotation][unique][in] */ 
    _In_  IBindCtx *pbc,
    /* [annotation][unique][in] */ 
    _In_opt_  IMoniker *pmkToLeft,
    /* [annotation][in] */ 
    _In_  REFIID riid,
    /* [annotation][iid_is][out] */ 
    _Outptr_  void **ppvObj);


/* [call_as] */ HRESULT STDMETHODCALLTYPE IMoniker_BindToStorage_Stub( 
    __RPC__in IMoniker * This,
    /* [unique][in] */ __RPC__in_opt IBindCtx *pbc,
    /* [unique][in] */ __RPC__in_opt IMoniker *pmkToLeft,
    /* [in] */ __RPC__in REFIID riid,
    /* [iid_is][out] */ __RPC__deref_out_opt IUnknown **ppvObj);

/* [local] */ HRESULT STDMETHODCALLTYPE IEnumSTATSTG_Next_Proxy( 
    IEnumSTATSTG * This,
    /* [in] */ ULONG celt,
    /* [annotation] */ 
    _Out_writes_to_(celt,*pceltFetched)  STATSTG *rgelt,
    /* [annotation] */ 
    _Out_opt_  ULONG *pceltFetched);


/* [call_as] */ HRESULT STDMETHODCALLTYPE IEnumSTATSTG_Next_Stub( 
    __RPC__in IEnumSTATSTG * This,
    /* [in] */ ULONG celt,
    /* [length_is][size_is][out] */ __RPC__out_ecount_part(celt, *pceltFetched) STATSTG *rgelt,
    /* [out] */ __RPC__out ULONG *pceltFetched);

/* [local] */ HRESULT STDMETHODCALLTYPE IStorage_OpenStream_Proxy( 
    IStorage * This,
    /* [annotation][string][in] */ 
    _In_z_  const OLECHAR *pwcsName,
    /* [annotation][unique][in] */ 
    _Reserved_  void *reserved1,
    /* [in] */ DWORD grfMode,
    /* [in] */ DWORD reserved2,
    /* [annotation][out] */ 
    _Outptr_  IStream **ppstm);


/* [call_as] */ HRESULT STDMETHODCALLTYPE IStorage_OpenStream_Stub( 
    __RPC__in IStorage * This,
    /* [string][in] */ __RPC__in_string const OLECHAR *pwcsName,
    /* [in] */ ULONG cbReserved1,
    /* [size_is][unique][in] */ __RPC__in_ecount_full_opt(cbReserved1) byte *reserved1,
    /* [in] */ DWORD grfMode,
    /* [in] */ DWORD reserved2,
    /* [out] */ __RPC__deref_out_opt IStream **ppstm);

/* [local] */ HRESULT STDMETHODCALLTYPE IStorage_CopyTo_Proxy( 
    IStorage * This,
    /* [in] */ DWORD ciidExclude,
    /* [annotation][size_is][unique][in] */ 
    _In_reads_opt_(ciidExclude)  const IID *rgiidExclude,
    /* [annotation][unique][in] */ 
    _In_opt_  SNB snbExclude,
    /* [annotation][unique][in] */ 
    _In_  IStorage *pstgDest);


/* [call_as] */ HRESULT STDMETHODCALLTYPE IStorage_CopyTo_Stub( 
    __RPC__in IStorage * This,
    /* [in] */ DWORD ciidExclude,
    /* [size_is][unique][in] */ __RPC__in_ecount_full_opt(ciidExclude) const IID *rgiidExclude,
    /* [unique][in] */ __RPC__deref_opt_in_opt SNB snbExclude,
    /* [unique][in] */ __RPC__in_opt IStorage *pstgDest);

/* [local] */ HRESULT STDMETHODCALLTYPE IStorage_EnumElements_Proxy( 
    IStorage * This,
    /* [annotation][in] */ 
    _Reserved_  DWORD reserved1,
    /* [annotation][size_is][unique][in] */ 
    _Reserved_  void *reserved2,
    /* [annotation][in] */ 
    _Reserved_  DWORD reserved3,
    /* [annotation][out] */ 
    _Outptr_  IEnumSTATSTG **ppenum);


/* [call_as] */ HRESULT STDMETHODCALLTYPE IStorage_EnumElements_Stub( 
    __RPC__in IStorage * This,
    /* [in] */ DWORD reserved1,
    /* [in] */ ULONG cbReserved2,
    /* [size_is][unique][in] */ __RPC__in_ecount_full_opt(cbReserved2) byte *reserved2,
    /* [in] */ DWORD reserved3,
    /* [out] */ __RPC__deref_out_opt IEnumSTATSTG **ppenum);

/* [local] */ HRESULT STDMETHODCALLTYPE ILockBytes_ReadAt_Proxy( 
    ILockBytes * This,
    /* [in] */ ULARGE_INTEGER ulOffset,
    /* [annotation][length_is][size_is][out] */ 
    _Out_writes_bytes_to_(cb, *pcbRead)  void *pv,
    /* [in] */ ULONG cb,
    /* [annotation][out] */ 
    _Out_opt_  ULONG *pcbRead);


/* [call_as] */ HRESULT __stdcall ILockBytes_ReadAt_Stub( 
    __RPC__in ILockBytes * This,
    /* [in] */ ULARGE_INTEGER ulOffset,
    /* [length_is][size_is][out] */ __RPC__out_ecount_part(cb, *pcbRead) byte *pv,
    /* [in] */ ULONG cb,
    /* [out] */ __RPC__out ULONG *pcbRead);

/* [local] */ HRESULT STDMETHODCALLTYPE ILockBytes_WriteAt_Proxy( 
    ILockBytes * This,
    /* [in] */ ULARGE_INTEGER ulOffset,
    /* [annotation][size_is][in] */ 
    _In_reads_bytes_(cb)  const void *pv,
    /* [in] */ ULONG cb,
    /* [annotation][out] */ 
    _Out_opt_  ULONG *pcbWritten);


/* [call_as] */ HRESULT STDMETHODCALLTYPE ILockBytes_WriteAt_Stub( 
    __RPC__in ILockBytes * This,
    /* [in] */ ULARGE_INTEGER ulOffset,
    /* [size_is][in] */ __RPC__in_ecount_full(cb) const byte *pv,
    /* [in] */ ULONG cb,
    /* [out] */ __RPC__out ULONG *pcbWritten);

/* [local] */ HRESULT STDMETHODCALLTYPE IEnumFORMATETC_Next_Proxy( 
    IEnumFORMATETC * This,
    /* [in] */ ULONG celt,
    /* [annotation] */ 
    _Out_writes_to_(celt,*pceltFetched)  FORMATETC *rgelt,
    /* [annotation] */ 
    _Out_opt_  ULONG *pceltFetched);


/* [call_as] */ HRESULT STDMETHODCALLTYPE IEnumFORMATETC_Next_Stub( 
    __RPC__in IEnumFORMATETC * This,
    /* [in] */ ULONG celt,
    /* [length_is][size_is][out] */ __RPC__out_ecount_part(celt, *pceltFetched) FORMATETC *rgelt,
    /* [out] */ __RPC__out ULONG *pceltFetched);

/* [local] */ HRESULT STDMETHODCALLTYPE IEnumSTATDATA_Next_Proxy( 
    IEnumSTATDATA * This,
    /* [in] */ ULONG celt,
    /* [annotation] */ 
    _Out_writes_to_(celt,*pceltFetched)  STATDATA *rgelt,
    /* [annotation] */ 
    _Out_opt_  ULONG *pceltFetched);


/* [call_as] */ HRESULT STDMETHODCALLTYPE IEnumSTATDATA_Next_Stub( 
    __RPC__in IEnumSTATDATA * This,
    /* [in] */ ULONG celt,
    /* [length_is][size_is][out] */ __RPC__out_ecount_part(celt, *pceltFetched) STATDATA *rgelt,
    /* [out] */ __RPC__out ULONG *pceltFetched);

/* [local] */ void STDMETHODCALLTYPE IAdviseSink_OnDataChange_Proxy( 
    IAdviseSink * This,
    /* [annotation][unique][in] */ 
    _In_  FORMATETC *pFormatetc,
    /* [annotation][unique][in] */ 
    _In_  STGMEDIUM *pStgmed);


/* [call_as] */ HRESULT STDMETHODCALLTYPE IAdviseSink_OnDataChange_Stub( 
    __RPC__in IAdviseSink * This,
    /* [unique][in] */ __RPC__in_opt FORMATETC *pFormatetc,
    /* [unique][in] */ __RPC__in_opt ASYNC_STGMEDIUM *pStgmed);

/* [local] */ void STDMETHODCALLTYPE IAdviseSink_OnViewChange_Proxy( 
    IAdviseSink * This,
    /* [in] */ DWORD dwAspect,
    /* [in] */ LONG lindex);


/* [call_as] */ HRESULT STDMETHODCALLTYPE IAdviseSink_OnViewChange_Stub( 
    __RPC__in IAdviseSink * This,
    /* [in] */ DWORD dwAspect,
    /* [in] */ LONG lindex);

/* [local] */ void STDMETHODCALLTYPE IAdviseSink_OnRename_Proxy( 
    IAdviseSink * This,
    /* [annotation][in] */ 
    _In_  IMoniker *pmk);


/* [call_as] */ HRESULT STDMETHODCALLTYPE IAdviseSink_OnRename_Stub( 
    __RPC__in IAdviseSink * This,
    /* [in] */ __RPC__in_opt IMoniker *pmk);

/* [local] */ void STDMETHODCALLTYPE IAdviseSink_OnSave_Proxy( 
    IAdviseSink * This);


/* [call_as] */ HRESULT STDMETHODCALLTYPE IAdviseSink_OnSave_Stub( 
    __RPC__in IAdviseSink * This);

/* [local] */ void STDMETHODCALLTYPE IAdviseSink_OnClose_Proxy( 
    IAdviseSink * This);


/* [call_as] */ HRESULT STDMETHODCALLTYPE IAdviseSink_OnClose_Stub( 
    __RPC__in IAdviseSink * This);

/* [local] */ void STDMETHODCALLTYPE AsyncIAdviseSink_Begin_OnDataChange_Proxy( 
    AsyncIAdviseSink * This,
    /* [annotation][unique][in] */ 
    _In_  FORMATETC *pFormatetc,
    /* [annotation][unique][in] */ 
    _In_  STGMEDIUM *pStgmed);


/* [call_as] */ HRESULT STDMETHODCALLTYPE AsyncIAdviseSink_Begin_OnDataChange_Stub( 
    __RPC__in AsyncIAdviseSink * This,
    /* [unique][in] */ __RPC__in_opt FORMATETC *pFormatetc,
    /* [unique][in] */ __RPC__in_opt ASYNC_STGMEDIUM *pStgmed);

/* [local] */ void STDMETHODCALLTYPE AsyncIAdviseSink_Finish_OnDataChange_Proxy( 
    AsyncIAdviseSink * This);


/* [call_as] */ HRESULT STDMETHODCALLTYPE AsyncIAdviseSink_Finish_OnDataChange_Stub( 
    __RPC__in AsyncIAdviseSink * This);

/* [local] */ void STDMETHODCALLTYPE AsyncIAdviseSink_Begin_OnViewChange_Proxy( 
    AsyncIAdviseSink * This,
    /* [in] */ DWORD dwAspect,
    /* [in] */ LONG lindex);


/* [call_as] */ HRESULT STDMETHODCALLTYPE AsyncIAdviseSink_Begin_OnViewChange_Stub( 
    __RPC__in AsyncIAdviseSink * This,
    /* [in] */ DWORD dwAspect,
    /* [in] */ LONG lindex);

/* [local] */ void STDMETHODCALLTYPE AsyncIAdviseSink_Finish_OnViewChange_Proxy( 
    AsyncIAdviseSink * This);


/* [call_as] */ HRESULT STDMETHODCALLTYPE AsyncIAdviseSink_Finish_OnViewChange_Stub( 
    __RPC__in AsyncIAdviseSink * This);

/* [local] */ void STDMETHODCALLTYPE AsyncIAdviseSink_Begin_OnRename_Proxy( 
    AsyncIAdviseSink * This,
    /* [annotation][in] */ 
    _In_  IMoniker *pmk);


/* [call_as] */ HRESULT STDMETHODCALLTYPE AsyncIAdviseSink_Begin_OnRename_Stub( 
    __RPC__in AsyncIAdviseSink * This,
    /* [in] */ __RPC__in_opt IMoniker *pmk);

/* [local] */ void STDMETHODCALLTYPE AsyncIAdviseSink_Finish_OnRename_Proxy( 
    AsyncIAdviseSink * This);


/* [call_as] */ HRESULT STDMETHODCALLTYPE AsyncIAdviseSink_Finish_OnRename_Stub( 
    __RPC__in AsyncIAdviseSink * This);

/* [local] */ void STDMETHODCALLTYPE AsyncIAdviseSink_Begin_OnSave_Proxy( 
    AsyncIAdviseSink * This);


/* [call_as] */ HRESULT STDMETHODCALLTYPE AsyncIAdviseSink_Begin_OnSave_Stub( 
    __RPC__in AsyncIAdviseSink * This);

/* [local] */ void STDMETHODCALLTYPE AsyncIAdviseSink_Finish_OnSave_Proxy( 
    AsyncIAdviseSink * This);


/* [call_as] */ HRESULT STDMETHODCALLTYPE AsyncIAdviseSink_Finish_OnSave_Stub( 
    __RPC__in AsyncIAdviseSink * This);

/* [local] */ void STDMETHODCALLTYPE AsyncIAdviseSink_Begin_OnClose_Proxy( 
    AsyncIAdviseSink * This);


/* [call_as] */ HRESULT STDMETHODCALLTYPE AsyncIAdviseSink_Begin_OnClose_Stub( 
    __RPC__in AsyncIAdviseSink * This);

/* [local] */ void STDMETHODCALLTYPE AsyncIAdviseSink_Finish_OnClose_Proxy( 
    AsyncIAdviseSink * This);


/* [call_as] */ HRESULT STDMETHODCALLTYPE AsyncIAdviseSink_Finish_OnClose_Stub( 
    __RPC__in AsyncIAdviseSink * This);

/* [local] */ void STDMETHODCALLTYPE IAdviseSink2_OnLinkSrcChange_Proxy( 
    IAdviseSink2 * This,
    /* [annotation][unique][in] */ 
    _In_  IMoniker *pmk);


/* [call_as] */ HRESULT STDMETHODCALLTYPE IAdviseSink2_OnLinkSrcChange_Stub( 
    __RPC__in IAdviseSink2 * This,
    /* [unique][in] */ __RPC__in_opt IMoniker *pmk);

/* [local] */ void STDMETHODCALLTYPE AsyncIAdviseSink2_Begin_OnLinkSrcChange_Proxy( 
    AsyncIAdviseSink2 * This,
    /* [annotation][unique][in] */ 
    _In_  IMoniker *pmk);


/* [call_as] */ HRESULT STDMETHODCALLTYPE AsyncIAdviseSink2_Begin_OnLinkSrcChange_Stub( 
    __RPC__in AsyncIAdviseSink2 * This,
    /* [unique][in] */ __RPC__in_opt IMoniker *pmk);

/* [local] */ void STDMETHODCALLTYPE AsyncIAdviseSink2_Finish_OnLinkSrcChange_Proxy( 
    AsyncIAdviseSink2 * This);


/* [call_as] */ HRESULT STDMETHODCALLTYPE AsyncIAdviseSink2_Finish_OnLinkSrcChange_Stub( 
    __RPC__in AsyncIAdviseSink2 * This);

/* [local] */ HRESULT STDMETHODCALLTYPE IDataObject_GetData_Proxy( 
    IDataObject * This,
    /* [annotation][unique][in] */ 
    _In_  FORMATETC *pformatetcIn,
    /* [annotation][out] */ 
    _Out_  STGMEDIUM *pmedium);


/* [call_as] */ HRESULT STDMETHODCALLTYPE IDataObject_GetData_Stub( 
    __RPC__in IDataObject * This,
    /* [unique][in] */ __RPC__in_opt FORMATETC *pformatetcIn,
    /* [out] */ __RPC__out STGMEDIUM *pRemoteMedium);

/* [local] */ HRESULT STDMETHODCALLTYPE IDataObject_GetDataHere_Proxy( 
    IDataObject * This,
    /* [annotation][unique][in] */ 
    _In_  FORMATETC *pformatetc,
    /* [annotation][out][in] */ 
    _Inout_  STGMEDIUM *pmedium);


/* [call_as] */ HRESULT STDMETHODCALLTYPE IDataObject_GetDataHere_Stub( 
    __RPC__in IDataObject * This,
    /* [unique][in] */ __RPC__in_opt FORMATETC *pformatetc,
    /* [out][in] */ __RPC__inout STGMEDIUM *pRemoteMedium);

/* [local] */ HRESULT STDMETHODCALLTYPE IDataObject_SetData_Proxy( 
    IDataObject * This,
    /* [annotation][unique][in] */ 
    _In_  FORMATETC *pformatetc,
    /* [annotation][unique][in] */ 
    _In_  STGMEDIUM *pmedium,
    /* [in] */ BOOL fRelease);


/* [call_as] */ HRESULT STDMETHODCALLTYPE IDataObject_SetData_Stub( 
    __RPC__in IDataObject * This,
    /* [unique][in] */ __RPC__in_opt FORMATETC *pformatetc,
    /* [unique][in] */ __RPC__in_opt FLAG_STGMEDIUM *pmedium,
    /* [in] */ BOOL fRelease);

/* [local] */ HRESULT STDMETHODCALLTYPE IFillLockBytes_FillAppend_Proxy( 
    IFillLockBytes * This,
    /* [annotation][size_is][in] */ 
    _In_reads_bytes_(cb)  const void *pv,
    /* [annotation][in] */ 
    _In_  ULONG cb,
    /* [annotation][out] */ 
    _Out_  ULONG *pcbWritten);


/* [call_as] */ HRESULT __stdcall IFillLockBytes_FillAppend_Stub( 
    __RPC__in IFillLockBytes * This,
    /* [size_is][in] */ __RPC__in_ecount_full(cb) const byte *pv,
    /* [in] */ ULONG cb,
    /* [out] */ __RPC__out ULONG *pcbWritten);

/* [local] */ HRESULT STDMETHODCALLTYPE IFillLockBytes_FillAt_Proxy( 
    IFillLockBytes * This,
    /* [annotation][in] */ 
    _In_  ULARGE_INTEGER ulOffset,
    /* [annotation][size_is][in] */ 
    _In_reads_bytes_(cb)  const void *pv,
    /* [annotation][in] */ 
    _In_  ULONG cb,
    /* [annotation][out] */ 
    _Out_  ULONG *pcbWritten);


/* [call_as] */ HRESULT __stdcall IFillLockBytes_FillAt_Stub( 
    __RPC__in IFillLockBytes * This,
    /* [in] */ ULARGE_INTEGER ulOffset,
    /* [size_is][in] */ __RPC__in_ecount_full(cb) const byte *pv,
    /* [in] */ ULONG cb,
    /* [out] */ __RPC__out ULONG *pcbWritten);



/* end of Additional Prototypes */

#ifdef __cplusplus
}
#endif

#endif


