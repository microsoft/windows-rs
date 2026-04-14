

/* this ALWAYS GENERATED file contains the definitions for the interfaces */


 /* File created by MIDL compiler version 8.01.0628 */
/* @@MIDL_FILE_HEADING(  ) */



/* verify that the <rpcndr.h> version is high enough to compile this file*/
#ifndef __REQUIRED_RPCNDR_H_VERSION__
#define __REQUIRED_RPCNDR_H_VERSION__ 500
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

#ifndef __activdbg_h__
#define __activdbg_h__

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

#ifndef __IActiveScriptDebug32_FWD_DEFINED__
#define __IActiveScriptDebug32_FWD_DEFINED__
typedef interface IActiveScriptDebug32 IActiveScriptDebug32;

#endif 	/* __IActiveScriptDebug32_FWD_DEFINED__ */


#ifndef __IActiveScriptDebug64_FWD_DEFINED__
#define __IActiveScriptDebug64_FWD_DEFINED__
typedef interface IActiveScriptDebug64 IActiveScriptDebug64;

#endif 	/* __IActiveScriptDebug64_FWD_DEFINED__ */


#ifndef __IActiveScriptSiteDebug32_FWD_DEFINED__
#define __IActiveScriptSiteDebug32_FWD_DEFINED__
typedef interface IActiveScriptSiteDebug32 IActiveScriptSiteDebug32;

#endif 	/* __IActiveScriptSiteDebug32_FWD_DEFINED__ */


#ifndef __IActiveScriptSiteDebug64_FWD_DEFINED__
#define __IActiveScriptSiteDebug64_FWD_DEFINED__
typedef interface IActiveScriptSiteDebug64 IActiveScriptSiteDebug64;

#endif 	/* __IActiveScriptSiteDebug64_FWD_DEFINED__ */


#ifndef __IActiveScriptSiteDebugEx_FWD_DEFINED__
#define __IActiveScriptSiteDebugEx_FWD_DEFINED__
typedef interface IActiveScriptSiteDebugEx IActiveScriptSiteDebugEx;

#endif 	/* __IActiveScriptSiteDebugEx_FWD_DEFINED__ */


#ifndef __IActiveScriptErrorDebug_FWD_DEFINED__
#define __IActiveScriptErrorDebug_FWD_DEFINED__
typedef interface IActiveScriptErrorDebug IActiveScriptErrorDebug;

#endif 	/* __IActiveScriptErrorDebug_FWD_DEFINED__ */


#ifndef __IDebugCodeContext_FWD_DEFINED__
#define __IDebugCodeContext_FWD_DEFINED__
typedef interface IDebugCodeContext IDebugCodeContext;

#endif 	/* __IDebugCodeContext_FWD_DEFINED__ */


#ifndef __IDebugExpression_FWD_DEFINED__
#define __IDebugExpression_FWD_DEFINED__
typedef interface IDebugExpression IDebugExpression;

#endif 	/* __IDebugExpression_FWD_DEFINED__ */


#ifndef __IDebugExpressionContext_FWD_DEFINED__
#define __IDebugExpressionContext_FWD_DEFINED__
typedef interface IDebugExpressionContext IDebugExpressionContext;

#endif 	/* __IDebugExpressionContext_FWD_DEFINED__ */


#ifndef __IDebugExpressionCallBack_FWD_DEFINED__
#define __IDebugExpressionCallBack_FWD_DEFINED__
typedef interface IDebugExpressionCallBack IDebugExpressionCallBack;

#endif 	/* __IDebugExpressionCallBack_FWD_DEFINED__ */


#ifndef __IDebugStackFrame_FWD_DEFINED__
#define __IDebugStackFrame_FWD_DEFINED__
typedef interface IDebugStackFrame IDebugStackFrame;

#endif 	/* __IDebugStackFrame_FWD_DEFINED__ */


#ifndef __IDebugStackFrameSniffer_FWD_DEFINED__
#define __IDebugStackFrameSniffer_FWD_DEFINED__
typedef interface IDebugStackFrameSniffer IDebugStackFrameSniffer;

#endif 	/* __IDebugStackFrameSniffer_FWD_DEFINED__ */


#ifndef __IDebugStackFrameSnifferEx32_FWD_DEFINED__
#define __IDebugStackFrameSnifferEx32_FWD_DEFINED__
typedef interface IDebugStackFrameSnifferEx32 IDebugStackFrameSnifferEx32;

#endif 	/* __IDebugStackFrameSnifferEx32_FWD_DEFINED__ */


#ifndef __IDebugStackFrameSnifferEx64_FWD_DEFINED__
#define __IDebugStackFrameSnifferEx64_FWD_DEFINED__
typedef interface IDebugStackFrameSnifferEx64 IDebugStackFrameSnifferEx64;

#endif 	/* __IDebugStackFrameSnifferEx64_FWD_DEFINED__ */


#ifndef __IDebugSyncOperation_FWD_DEFINED__
#define __IDebugSyncOperation_FWD_DEFINED__
typedef interface IDebugSyncOperation IDebugSyncOperation;

#endif 	/* __IDebugSyncOperation_FWD_DEFINED__ */


#ifndef __IDebugAsyncOperation_FWD_DEFINED__
#define __IDebugAsyncOperation_FWD_DEFINED__
typedef interface IDebugAsyncOperation IDebugAsyncOperation;

#endif 	/* __IDebugAsyncOperation_FWD_DEFINED__ */


#ifndef __IDebugAsyncOperationCallBack_FWD_DEFINED__
#define __IDebugAsyncOperationCallBack_FWD_DEFINED__
typedef interface IDebugAsyncOperationCallBack IDebugAsyncOperationCallBack;

#endif 	/* __IDebugAsyncOperationCallBack_FWD_DEFINED__ */


#ifndef __IEnumDebugCodeContexts_FWD_DEFINED__
#define __IEnumDebugCodeContexts_FWD_DEFINED__
typedef interface IEnumDebugCodeContexts IEnumDebugCodeContexts;

#endif 	/* __IEnumDebugCodeContexts_FWD_DEFINED__ */


#ifndef __IEnumDebugStackFrames_FWD_DEFINED__
#define __IEnumDebugStackFrames_FWD_DEFINED__
typedef interface IEnumDebugStackFrames IEnumDebugStackFrames;

#endif 	/* __IEnumDebugStackFrames_FWD_DEFINED__ */


#ifndef __IEnumDebugStackFrames64_FWD_DEFINED__
#define __IEnumDebugStackFrames64_FWD_DEFINED__
typedef interface IEnumDebugStackFrames64 IEnumDebugStackFrames64;

#endif 	/* __IEnumDebugStackFrames64_FWD_DEFINED__ */


#ifndef __IDebugDocumentInfo_FWD_DEFINED__
#define __IDebugDocumentInfo_FWD_DEFINED__
typedef interface IDebugDocumentInfo IDebugDocumentInfo;

#endif 	/* __IDebugDocumentInfo_FWD_DEFINED__ */


#ifndef __IDebugDocumentProvider_FWD_DEFINED__
#define __IDebugDocumentProvider_FWD_DEFINED__
typedef interface IDebugDocumentProvider IDebugDocumentProvider;

#endif 	/* __IDebugDocumentProvider_FWD_DEFINED__ */


#ifndef __IDebugDocument_FWD_DEFINED__
#define __IDebugDocument_FWD_DEFINED__
typedef interface IDebugDocument IDebugDocument;

#endif 	/* __IDebugDocument_FWD_DEFINED__ */


#ifndef __IDebugDocumentText_FWD_DEFINED__
#define __IDebugDocumentText_FWD_DEFINED__
typedef interface IDebugDocumentText IDebugDocumentText;

#endif 	/* __IDebugDocumentText_FWD_DEFINED__ */


#ifndef __IDebugDocumentTextEvents_FWD_DEFINED__
#define __IDebugDocumentTextEvents_FWD_DEFINED__
typedef interface IDebugDocumentTextEvents IDebugDocumentTextEvents;

#endif 	/* __IDebugDocumentTextEvents_FWD_DEFINED__ */


#ifndef __IDebugDocumentTextAuthor_FWD_DEFINED__
#define __IDebugDocumentTextAuthor_FWD_DEFINED__
typedef interface IDebugDocumentTextAuthor IDebugDocumentTextAuthor;

#endif 	/* __IDebugDocumentTextAuthor_FWD_DEFINED__ */


#ifndef __IDebugDocumentTextExternalAuthor_FWD_DEFINED__
#define __IDebugDocumentTextExternalAuthor_FWD_DEFINED__
typedef interface IDebugDocumentTextExternalAuthor IDebugDocumentTextExternalAuthor;

#endif 	/* __IDebugDocumentTextExternalAuthor_FWD_DEFINED__ */


#ifndef __IDebugDocumentHelper32_FWD_DEFINED__
#define __IDebugDocumentHelper32_FWD_DEFINED__
typedef interface IDebugDocumentHelper32 IDebugDocumentHelper32;

#endif 	/* __IDebugDocumentHelper32_FWD_DEFINED__ */


#ifndef __IDebugDocumentHelper64_FWD_DEFINED__
#define __IDebugDocumentHelper64_FWD_DEFINED__
typedef interface IDebugDocumentHelper64 IDebugDocumentHelper64;

#endif 	/* __IDebugDocumentHelper64_FWD_DEFINED__ */


#ifndef __IDebugDocumentHost_FWD_DEFINED__
#define __IDebugDocumentHost_FWD_DEFINED__
typedef interface IDebugDocumentHost IDebugDocumentHost;

#endif 	/* __IDebugDocumentHost_FWD_DEFINED__ */


#ifndef __IDebugDocumentContext_FWD_DEFINED__
#define __IDebugDocumentContext_FWD_DEFINED__
typedef interface IDebugDocumentContext IDebugDocumentContext;

#endif 	/* __IDebugDocumentContext_FWD_DEFINED__ */


#ifndef __IDebugSessionProvider_FWD_DEFINED__
#define __IDebugSessionProvider_FWD_DEFINED__
typedef interface IDebugSessionProvider IDebugSessionProvider;

#endif 	/* __IDebugSessionProvider_FWD_DEFINED__ */


#ifndef __IApplicationDebugger_FWD_DEFINED__
#define __IApplicationDebugger_FWD_DEFINED__
typedef interface IApplicationDebugger IApplicationDebugger;

#endif 	/* __IApplicationDebugger_FWD_DEFINED__ */


#ifndef __IApplicationDebuggerUI_FWD_DEFINED__
#define __IApplicationDebuggerUI_FWD_DEFINED__
typedef interface IApplicationDebuggerUI IApplicationDebuggerUI;

#endif 	/* __IApplicationDebuggerUI_FWD_DEFINED__ */


#ifndef __IMachineDebugManager_FWD_DEFINED__
#define __IMachineDebugManager_FWD_DEFINED__
typedef interface IMachineDebugManager IMachineDebugManager;

#endif 	/* __IMachineDebugManager_FWD_DEFINED__ */


#ifndef __IMachineDebugManagerCookie_FWD_DEFINED__
#define __IMachineDebugManagerCookie_FWD_DEFINED__
typedef interface IMachineDebugManagerCookie IMachineDebugManagerCookie;

#endif 	/* __IMachineDebugManagerCookie_FWD_DEFINED__ */


#ifndef __IMachineDebugManagerEvents_FWD_DEFINED__
#define __IMachineDebugManagerEvents_FWD_DEFINED__
typedef interface IMachineDebugManagerEvents IMachineDebugManagerEvents;

#endif 	/* __IMachineDebugManagerEvents_FWD_DEFINED__ */


#ifndef __IProcessDebugManager32_FWD_DEFINED__
#define __IProcessDebugManager32_FWD_DEFINED__
typedef interface IProcessDebugManager32 IProcessDebugManager32;

#endif 	/* __IProcessDebugManager32_FWD_DEFINED__ */


#ifndef __IProcessDebugManager64_FWD_DEFINED__
#define __IProcessDebugManager64_FWD_DEFINED__
typedef interface IProcessDebugManager64 IProcessDebugManager64;

#endif 	/* __IProcessDebugManager64_FWD_DEFINED__ */


#ifndef __IRemoteDebugApplication_FWD_DEFINED__
#define __IRemoteDebugApplication_FWD_DEFINED__
typedef interface IRemoteDebugApplication IRemoteDebugApplication;

#endif 	/* __IRemoteDebugApplication_FWD_DEFINED__ */


#ifndef __IDebugApplication32_FWD_DEFINED__
#define __IDebugApplication32_FWD_DEFINED__
typedef interface IDebugApplication32 IDebugApplication32;

#endif 	/* __IDebugApplication32_FWD_DEFINED__ */


#ifndef __IDebugApplication64_FWD_DEFINED__
#define __IDebugApplication64_FWD_DEFINED__
typedef interface IDebugApplication64 IDebugApplication64;

#endif 	/* __IDebugApplication64_FWD_DEFINED__ */


#ifndef __IRemoteDebugApplicationEvents_FWD_DEFINED__
#define __IRemoteDebugApplicationEvents_FWD_DEFINED__
typedef interface IRemoteDebugApplicationEvents IRemoteDebugApplicationEvents;

#endif 	/* __IRemoteDebugApplicationEvents_FWD_DEFINED__ */


#ifndef __IDebugApplicationNode_FWD_DEFINED__
#define __IDebugApplicationNode_FWD_DEFINED__
typedef interface IDebugApplicationNode IDebugApplicationNode;

#endif 	/* __IDebugApplicationNode_FWD_DEFINED__ */


#ifndef __IDebugApplicationNodeEvents_FWD_DEFINED__
#define __IDebugApplicationNodeEvents_FWD_DEFINED__
typedef interface IDebugApplicationNodeEvents IDebugApplicationNodeEvents;

#endif 	/* __IDebugApplicationNodeEvents_FWD_DEFINED__ */


#ifndef __AsyncIDebugApplicationNodeEvents_FWD_DEFINED__
#define __AsyncIDebugApplicationNodeEvents_FWD_DEFINED__
typedef interface AsyncIDebugApplicationNodeEvents AsyncIDebugApplicationNodeEvents;

#endif 	/* __AsyncIDebugApplicationNodeEvents_FWD_DEFINED__ */


#ifndef __IDebugThreadCall32_FWD_DEFINED__
#define __IDebugThreadCall32_FWD_DEFINED__
typedef interface IDebugThreadCall32 IDebugThreadCall32;

#endif 	/* __IDebugThreadCall32_FWD_DEFINED__ */


#ifndef __IDebugThreadCall64_FWD_DEFINED__
#define __IDebugThreadCall64_FWD_DEFINED__
typedef interface IDebugThreadCall64 IDebugThreadCall64;

#endif 	/* __IDebugThreadCall64_FWD_DEFINED__ */


#ifndef __IRemoteDebugApplicationThread_FWD_DEFINED__
#define __IRemoteDebugApplicationThread_FWD_DEFINED__
typedef interface IRemoteDebugApplicationThread IRemoteDebugApplicationThread;

#endif 	/* __IRemoteDebugApplicationThread_FWD_DEFINED__ */


#ifndef __IDebugApplicationThread_FWD_DEFINED__
#define __IDebugApplicationThread_FWD_DEFINED__
typedef interface IDebugApplicationThread IDebugApplicationThread;

#endif 	/* __IDebugApplicationThread_FWD_DEFINED__ */


#ifndef __IDebugApplicationThread64_FWD_DEFINED__
#define __IDebugApplicationThread64_FWD_DEFINED__
typedef interface IDebugApplicationThread64 IDebugApplicationThread64;

#endif 	/* __IDebugApplicationThread64_FWD_DEFINED__ */


#ifndef __IDebugCookie_FWD_DEFINED__
#define __IDebugCookie_FWD_DEFINED__
typedef interface IDebugCookie IDebugCookie;

#endif 	/* __IDebugCookie_FWD_DEFINED__ */


#ifndef __IEnumDebugApplicationNodes_FWD_DEFINED__
#define __IEnumDebugApplicationNodes_FWD_DEFINED__
typedef interface IEnumDebugApplicationNodes IEnumDebugApplicationNodes;

#endif 	/* __IEnumDebugApplicationNodes_FWD_DEFINED__ */


#ifndef __IEnumRemoteDebugApplications_FWD_DEFINED__
#define __IEnumRemoteDebugApplications_FWD_DEFINED__
typedef interface IEnumRemoteDebugApplications IEnumRemoteDebugApplications;

#endif 	/* __IEnumRemoteDebugApplications_FWD_DEFINED__ */


#ifndef __IEnumRemoteDebugApplicationThreads_FWD_DEFINED__
#define __IEnumRemoteDebugApplicationThreads_FWD_DEFINED__
typedef interface IEnumRemoteDebugApplicationThreads IEnumRemoteDebugApplicationThreads;

#endif 	/* __IEnumRemoteDebugApplicationThreads_FWD_DEFINED__ */


#ifndef __IDebugFormatter_FWD_DEFINED__
#define __IDebugFormatter_FWD_DEFINED__
typedef interface IDebugFormatter IDebugFormatter;

#endif 	/* __IDebugFormatter_FWD_DEFINED__ */


#ifndef __ISimpleConnectionPoint_FWD_DEFINED__
#define __ISimpleConnectionPoint_FWD_DEFINED__
typedef interface ISimpleConnectionPoint ISimpleConnectionPoint;

#endif 	/* __ISimpleConnectionPoint_FWD_DEFINED__ */


#ifndef __IDebugHelper_FWD_DEFINED__
#define __IDebugHelper_FWD_DEFINED__
typedef interface IDebugHelper IDebugHelper;

#endif 	/* __IDebugHelper_FWD_DEFINED__ */


#ifndef __IEnumDebugExpressionContexts_FWD_DEFINED__
#define __IEnumDebugExpressionContexts_FWD_DEFINED__
typedef interface IEnumDebugExpressionContexts IEnumDebugExpressionContexts;

#endif 	/* __IEnumDebugExpressionContexts_FWD_DEFINED__ */


#ifndef __IProvideExpressionContexts_FWD_DEFINED__
#define __IProvideExpressionContexts_FWD_DEFINED__
typedef interface IProvideExpressionContexts IProvideExpressionContexts;

#endif 	/* __IProvideExpressionContexts_FWD_DEFINED__ */


#ifndef __IActiveScriptDebug32_FWD_DEFINED__
#define __IActiveScriptDebug32_FWD_DEFINED__
typedef interface IActiveScriptDebug32 IActiveScriptDebug32;

#endif 	/* __IActiveScriptDebug32_FWD_DEFINED__ */


#ifndef __IActiveScriptDebug64_FWD_DEFINED__
#define __IActiveScriptDebug64_FWD_DEFINED__
typedef interface IActiveScriptDebug64 IActiveScriptDebug64;

#endif 	/* __IActiveScriptDebug64_FWD_DEFINED__ */


#ifndef __IActiveScriptErrorDebug_FWD_DEFINED__
#define __IActiveScriptErrorDebug_FWD_DEFINED__
typedef interface IActiveScriptErrorDebug IActiveScriptErrorDebug;

#endif 	/* __IActiveScriptErrorDebug_FWD_DEFINED__ */


#ifndef __IActiveScriptSiteDebug32_FWD_DEFINED__
#define __IActiveScriptSiteDebug32_FWD_DEFINED__
typedef interface IActiveScriptSiteDebug32 IActiveScriptSiteDebug32;

#endif 	/* __IActiveScriptSiteDebug32_FWD_DEFINED__ */


#ifndef __IActiveScriptSiteDebug64_FWD_DEFINED__
#define __IActiveScriptSiteDebug64_FWD_DEFINED__
typedef interface IActiveScriptSiteDebug64 IActiveScriptSiteDebug64;

#endif 	/* __IActiveScriptSiteDebug64_FWD_DEFINED__ */


#ifndef __IActiveScriptSiteDebugEx_FWD_DEFINED__
#define __IActiveScriptSiteDebugEx_FWD_DEFINED__
typedef interface IActiveScriptSiteDebugEx IActiveScriptSiteDebugEx;

#endif 	/* __IActiveScriptSiteDebugEx_FWD_DEFINED__ */


#ifndef __IApplicationDebugger_FWD_DEFINED__
#define __IApplicationDebugger_FWD_DEFINED__
typedef interface IApplicationDebugger IApplicationDebugger;

#endif 	/* __IApplicationDebugger_FWD_DEFINED__ */


#ifndef __IApplicationDebuggerUI_FWD_DEFINED__
#define __IApplicationDebuggerUI_FWD_DEFINED__
typedef interface IApplicationDebuggerUI IApplicationDebuggerUI;

#endif 	/* __IApplicationDebuggerUI_FWD_DEFINED__ */


#ifndef __IRemoteDebugApplication_FWD_DEFINED__
#define __IRemoteDebugApplication_FWD_DEFINED__
typedef interface IRemoteDebugApplication IRemoteDebugApplication;

#endif 	/* __IRemoteDebugApplication_FWD_DEFINED__ */


#ifndef __IDebugApplication32_FWD_DEFINED__
#define __IDebugApplication32_FWD_DEFINED__
typedef interface IDebugApplication32 IDebugApplication32;

#endif 	/* __IDebugApplication32_FWD_DEFINED__ */


#ifndef __IDebugApplication64_FWD_DEFINED__
#define __IDebugApplication64_FWD_DEFINED__
typedef interface IDebugApplication64 IDebugApplication64;

#endif 	/* __IDebugApplication64_FWD_DEFINED__ */


#ifndef __IDebugDocumentInfo_FWD_DEFINED__
#define __IDebugDocumentInfo_FWD_DEFINED__
typedef interface IDebugDocumentInfo IDebugDocumentInfo;

#endif 	/* __IDebugDocumentInfo_FWD_DEFINED__ */


#ifndef __IDebugDocumentProvider_FWD_DEFINED__
#define __IDebugDocumentProvider_FWD_DEFINED__
typedef interface IDebugDocumentProvider IDebugDocumentProvider;

#endif 	/* __IDebugDocumentProvider_FWD_DEFINED__ */


#ifndef __IDebugApplicationNode_FWD_DEFINED__
#define __IDebugApplicationNode_FWD_DEFINED__
typedef interface IDebugApplicationNode IDebugApplicationNode;

#endif 	/* __IDebugApplicationNode_FWD_DEFINED__ */


#ifndef __IDebugApplicationNodeEvents_FWD_DEFINED__
#define __IDebugApplicationNodeEvents_FWD_DEFINED__
typedef interface IDebugApplicationNodeEvents IDebugApplicationNodeEvents;

#endif 	/* __IDebugApplicationNodeEvents_FWD_DEFINED__ */


#ifndef __IRemoteDebugApplicationThread_FWD_DEFINED__
#define __IRemoteDebugApplicationThread_FWD_DEFINED__
typedef interface IRemoteDebugApplicationThread IRemoteDebugApplicationThread;

#endif 	/* __IRemoteDebugApplicationThread_FWD_DEFINED__ */


#ifndef __IDebugApplicationThread_FWD_DEFINED__
#define __IDebugApplicationThread_FWD_DEFINED__
typedef interface IDebugApplicationThread IDebugApplicationThread;

#endif 	/* __IDebugApplicationThread_FWD_DEFINED__ */


#ifndef __IDebugAsyncOperation_FWD_DEFINED__
#define __IDebugAsyncOperation_FWD_DEFINED__
typedef interface IDebugAsyncOperation IDebugAsyncOperation;

#endif 	/* __IDebugAsyncOperation_FWD_DEFINED__ */


#ifndef __IDebugAsyncOperationCallBack_FWD_DEFINED__
#define __IDebugAsyncOperationCallBack_FWD_DEFINED__
typedef interface IDebugAsyncOperationCallBack IDebugAsyncOperationCallBack;

#endif 	/* __IDebugAsyncOperationCallBack_FWD_DEFINED__ */


#ifndef __IDebugCodeContext_FWD_DEFINED__
#define __IDebugCodeContext_FWD_DEFINED__
typedef interface IDebugCodeContext IDebugCodeContext;

#endif 	/* __IDebugCodeContext_FWD_DEFINED__ */


#ifndef __IDebugCookie_FWD_DEFINED__
#define __IDebugCookie_FWD_DEFINED__
typedef interface IDebugCookie IDebugCookie;

#endif 	/* __IDebugCookie_FWD_DEFINED__ */


#ifndef __IDebugDocument_FWD_DEFINED__
#define __IDebugDocument_FWD_DEFINED__
typedef interface IDebugDocument IDebugDocument;

#endif 	/* __IDebugDocument_FWD_DEFINED__ */


#ifndef __IDebugDocumentContext_FWD_DEFINED__
#define __IDebugDocumentContext_FWD_DEFINED__
typedef interface IDebugDocumentContext IDebugDocumentContext;

#endif 	/* __IDebugDocumentContext_FWD_DEFINED__ */


#ifndef __IDebugDocumentHelper32_FWD_DEFINED__
#define __IDebugDocumentHelper32_FWD_DEFINED__
typedef interface IDebugDocumentHelper32 IDebugDocumentHelper32;

#endif 	/* __IDebugDocumentHelper32_FWD_DEFINED__ */


#ifndef __IDebugDocumentHelper64_FWD_DEFINED__
#define __IDebugDocumentHelper64_FWD_DEFINED__
typedef interface IDebugDocumentHelper64 IDebugDocumentHelper64;

#endif 	/* __IDebugDocumentHelper64_FWD_DEFINED__ */


#ifndef __IDebugDocumentHost_FWD_DEFINED__
#define __IDebugDocumentHost_FWD_DEFINED__
typedef interface IDebugDocumentHost IDebugDocumentHost;

#endif 	/* __IDebugDocumentHost_FWD_DEFINED__ */


#ifndef __IDebugDocumentText_FWD_DEFINED__
#define __IDebugDocumentText_FWD_DEFINED__
typedef interface IDebugDocumentText IDebugDocumentText;

#endif 	/* __IDebugDocumentText_FWD_DEFINED__ */


#ifndef __IDebugDocumentTextAuthor_FWD_DEFINED__
#define __IDebugDocumentTextAuthor_FWD_DEFINED__
typedef interface IDebugDocumentTextAuthor IDebugDocumentTextAuthor;

#endif 	/* __IDebugDocumentTextAuthor_FWD_DEFINED__ */


#ifndef __IDebugDocumentTextEvents_FWD_DEFINED__
#define __IDebugDocumentTextEvents_FWD_DEFINED__
typedef interface IDebugDocumentTextEvents IDebugDocumentTextEvents;

#endif 	/* __IDebugDocumentTextEvents_FWD_DEFINED__ */


#ifndef __IDebugDocumentTextExternalAuthor_FWD_DEFINED__
#define __IDebugDocumentTextExternalAuthor_FWD_DEFINED__
typedef interface IDebugDocumentTextExternalAuthor IDebugDocumentTextExternalAuthor;

#endif 	/* __IDebugDocumentTextExternalAuthor_FWD_DEFINED__ */


#ifndef __IDebugExpression_FWD_DEFINED__
#define __IDebugExpression_FWD_DEFINED__
typedef interface IDebugExpression IDebugExpression;

#endif 	/* __IDebugExpression_FWD_DEFINED__ */


#ifndef __IDebugExpressionCallBack_FWD_DEFINED__
#define __IDebugExpressionCallBack_FWD_DEFINED__
typedef interface IDebugExpressionCallBack IDebugExpressionCallBack;

#endif 	/* __IDebugExpressionCallBack_FWD_DEFINED__ */


#ifndef __IDebugExpressionContext_FWD_DEFINED__
#define __IDebugExpressionContext_FWD_DEFINED__
typedef interface IDebugExpressionContext IDebugExpressionContext;

#endif 	/* __IDebugExpressionContext_FWD_DEFINED__ */


#ifndef __IDebugFormatter_FWD_DEFINED__
#define __IDebugFormatter_FWD_DEFINED__
typedef interface IDebugFormatter IDebugFormatter;

#endif 	/* __IDebugFormatter_FWD_DEFINED__ */


#ifndef __IDebugHelper_FWD_DEFINED__
#define __IDebugHelper_FWD_DEFINED__
typedef interface IDebugHelper IDebugHelper;

#endif 	/* __IDebugHelper_FWD_DEFINED__ */


#ifndef __IDebugSessionProvider_FWD_DEFINED__
#define __IDebugSessionProvider_FWD_DEFINED__
typedef interface IDebugSessionProvider IDebugSessionProvider;

#endif 	/* __IDebugSessionProvider_FWD_DEFINED__ */


#ifndef __IDebugStackFrame_FWD_DEFINED__
#define __IDebugStackFrame_FWD_DEFINED__
typedef interface IDebugStackFrame IDebugStackFrame;

#endif 	/* __IDebugStackFrame_FWD_DEFINED__ */


#ifndef __IDebugStackFrameSniffer_FWD_DEFINED__
#define __IDebugStackFrameSniffer_FWD_DEFINED__
typedef interface IDebugStackFrameSniffer IDebugStackFrameSniffer;

#endif 	/* __IDebugStackFrameSniffer_FWD_DEFINED__ */


#ifndef __IDebugStackFrameSnifferEx32_FWD_DEFINED__
#define __IDebugStackFrameSnifferEx32_FWD_DEFINED__
typedef interface IDebugStackFrameSnifferEx32 IDebugStackFrameSnifferEx32;

#endif 	/* __IDebugStackFrameSnifferEx32_FWD_DEFINED__ */


#ifndef __IDebugStackFrameSnifferEx64_FWD_DEFINED__
#define __IDebugStackFrameSnifferEx64_FWD_DEFINED__
typedef interface IDebugStackFrameSnifferEx64 IDebugStackFrameSnifferEx64;

#endif 	/* __IDebugStackFrameSnifferEx64_FWD_DEFINED__ */


#ifndef __IDebugSyncOperation_FWD_DEFINED__
#define __IDebugSyncOperation_FWD_DEFINED__
typedef interface IDebugSyncOperation IDebugSyncOperation;

#endif 	/* __IDebugSyncOperation_FWD_DEFINED__ */


#ifndef __IDebugThreadCall32_FWD_DEFINED__
#define __IDebugThreadCall32_FWD_DEFINED__
typedef interface IDebugThreadCall32 IDebugThreadCall32;

#endif 	/* __IDebugThreadCall32_FWD_DEFINED__ */


#ifndef __IDebugThreadCall64_FWD_DEFINED__
#define __IDebugThreadCall64_FWD_DEFINED__
typedef interface IDebugThreadCall64 IDebugThreadCall64;

#endif 	/* __IDebugThreadCall64_FWD_DEFINED__ */


#ifndef __IEnumDebugApplicationNodes_FWD_DEFINED__
#define __IEnumDebugApplicationNodes_FWD_DEFINED__
typedef interface IEnumDebugApplicationNodes IEnumDebugApplicationNodes;

#endif 	/* __IEnumDebugApplicationNodes_FWD_DEFINED__ */


#ifndef __IEnumDebugCodeContexts_FWD_DEFINED__
#define __IEnumDebugCodeContexts_FWD_DEFINED__
typedef interface IEnumDebugCodeContexts IEnumDebugCodeContexts;

#endif 	/* __IEnumDebugCodeContexts_FWD_DEFINED__ */


#ifndef __IEnumDebugExpressionContexts_FWD_DEFINED__
#define __IEnumDebugExpressionContexts_FWD_DEFINED__
typedef interface IEnumDebugExpressionContexts IEnumDebugExpressionContexts;

#endif 	/* __IEnumDebugExpressionContexts_FWD_DEFINED__ */


#ifndef __IEnumDebugStackFrames_FWD_DEFINED__
#define __IEnumDebugStackFrames_FWD_DEFINED__
typedef interface IEnumDebugStackFrames IEnumDebugStackFrames;

#endif 	/* __IEnumDebugStackFrames_FWD_DEFINED__ */


#ifndef __IEnumDebugStackFrames64_FWD_DEFINED__
#define __IEnumDebugStackFrames64_FWD_DEFINED__
typedef interface IEnumDebugStackFrames64 IEnumDebugStackFrames64;

#endif 	/* __IEnumDebugStackFrames64_FWD_DEFINED__ */


#ifndef __IEnumRemoteDebugApplications_FWD_DEFINED__
#define __IEnumRemoteDebugApplications_FWD_DEFINED__
typedef interface IEnumRemoteDebugApplications IEnumRemoteDebugApplications;

#endif 	/* __IEnumRemoteDebugApplications_FWD_DEFINED__ */


#ifndef __IEnumRemoteDebugApplicationThreads_FWD_DEFINED__
#define __IEnumRemoteDebugApplicationThreads_FWD_DEFINED__
typedef interface IEnumRemoteDebugApplicationThreads IEnumRemoteDebugApplicationThreads;

#endif 	/* __IEnumRemoteDebugApplicationThreads_FWD_DEFINED__ */


#ifndef __IProcessDebugManager32_FWD_DEFINED__
#define __IProcessDebugManager32_FWD_DEFINED__
typedef interface IProcessDebugManager32 IProcessDebugManager32;

#endif 	/* __IProcessDebugManager32_FWD_DEFINED__ */


#ifndef __IProcessDebugManager64_FWD_DEFINED__
#define __IProcessDebugManager64_FWD_DEFINED__
typedef interface IProcessDebugManager64 IProcessDebugManager64;

#endif 	/* __IProcessDebugManager64_FWD_DEFINED__ */


#ifndef __IProvideExpressionContexts_FWD_DEFINED__
#define __IProvideExpressionContexts_FWD_DEFINED__
typedef interface IProvideExpressionContexts IProvideExpressionContexts;

#endif 	/* __IProvideExpressionContexts_FWD_DEFINED__ */


#ifndef __IMachineDebugManager_FWD_DEFINED__
#define __IMachineDebugManager_FWD_DEFINED__
typedef interface IMachineDebugManager IMachineDebugManager;

#endif 	/* __IMachineDebugManager_FWD_DEFINED__ */


#ifndef __IMachineDebugManagerCookie_FWD_DEFINED__
#define __IMachineDebugManagerCookie_FWD_DEFINED__
typedef interface IMachineDebugManagerCookie IMachineDebugManagerCookie;

#endif 	/* __IMachineDebugManagerCookie_FWD_DEFINED__ */


#ifndef __IMachineDebugManagerEvents_FWD_DEFINED__
#define __IMachineDebugManagerEvents_FWD_DEFINED__
typedef interface IMachineDebugManagerEvents IMachineDebugManagerEvents;

#endif 	/* __IMachineDebugManagerEvents_FWD_DEFINED__ */


#ifndef __IRemoteDebugApplicationEvents_FWD_DEFINED__
#define __IRemoteDebugApplicationEvents_FWD_DEFINED__
typedef interface IRemoteDebugApplicationEvents IRemoteDebugApplicationEvents;

#endif 	/* __IRemoteDebugApplicationEvents_FWD_DEFINED__ */


#ifndef __ISimpleConnectionPoint_FWD_DEFINED__
#define __ISimpleConnectionPoint_FWD_DEFINED__
typedef interface ISimpleConnectionPoint ISimpleConnectionPoint;

#endif 	/* __ISimpleConnectionPoint_FWD_DEFINED__ */


#ifndef __ProcessDebugManager_FWD_DEFINED__
#define __ProcessDebugManager_FWD_DEFINED__

#ifdef __cplusplus
typedef class ProcessDebugManager ProcessDebugManager;
#else
typedef struct ProcessDebugManager ProcessDebugManager;
#endif /* __cplusplus */

#endif 	/* __ProcessDebugManager_FWD_DEFINED__ */


#ifndef __DebugHelper_FWD_DEFINED__
#define __DebugHelper_FWD_DEFINED__

#ifdef __cplusplus
typedef class DebugHelper DebugHelper;
#else
typedef struct DebugHelper DebugHelper;
#endif /* __cplusplus */

#endif 	/* __DebugHelper_FWD_DEFINED__ */


#ifndef __CDebugDocumentHelper_FWD_DEFINED__
#define __CDebugDocumentHelper_FWD_DEFINED__

#ifdef __cplusplus
typedef class CDebugDocumentHelper CDebugDocumentHelper;
#else
typedef struct CDebugDocumentHelper CDebugDocumentHelper;
#endif /* __cplusplus */

#endif 	/* __CDebugDocumentHelper_FWD_DEFINED__ */


#ifndef __MachineDebugManager_RETAIL_FWD_DEFINED__
#define __MachineDebugManager_RETAIL_FWD_DEFINED__

#ifdef __cplusplus
typedef class MachineDebugManager_RETAIL MachineDebugManager_RETAIL;
#else
typedef struct MachineDebugManager_RETAIL MachineDebugManager_RETAIL;
#endif /* __cplusplus */

#endif 	/* __MachineDebugManager_RETAIL_FWD_DEFINED__ */


#ifndef __MachineDebugManager_DEBUG_FWD_DEFINED__
#define __MachineDebugManager_DEBUG_FWD_DEFINED__

#ifdef __cplusplus
typedef class MachineDebugManager_DEBUG MachineDebugManager_DEBUG;
#else
typedef struct MachineDebugManager_DEBUG MachineDebugManager_DEBUG;
#endif /* __cplusplus */

#endif 	/* __MachineDebugManager_DEBUG_FWD_DEFINED__ */


#ifndef __DefaultDebugSessionProvider_FWD_DEFINED__
#define __DefaultDebugSessionProvider_FWD_DEFINED__

#ifdef __cplusplus
typedef class DefaultDebugSessionProvider DefaultDebugSessionProvider;
#else
typedef struct DefaultDebugSessionProvider DefaultDebugSessionProvider;
#endif /* __cplusplus */

#endif 	/* __DefaultDebugSessionProvider_FWD_DEFINED__ */


/* header files for imported files */
#include "ocidl.h"
#include "activscp.h"
#include "dbgprop.h"

#ifdef __cplusplus
extern "C"{
#endif 


/* interface __MIDL_itf_activdbg_0000_0000 */
/* [local] */ 

//=--------------------------------------------------------------------------=
// ActivDbg.h
//=--------------------------------------------------------------------------=
// (C) Copyright 2000 Microsoft Corporation.  All Rights Reserved.
//
// THIS CODE AND INFORMATION IS PROVIDED "AS IS" WITHOUT WARRANTY OF
// ANY KIND, EITHER EXPRESSED OR IMPLIED, INCLUDING BUT NOT LIMITED TO
// THE IMPLIED WARRANTIES OF MERCHANTABILITY AND/OR FITNESS FOR A
// PARTICULAR PURPOSE.
//=--------------------------------------------------------------------------=
//
#include <winapifamily.h>
#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)
#pragma comment(lib,"uuid.lib")
//
// Declarations for ActiveX Scripting authoring/Debugging.
//

#ifndef __ActivDbg_h
#define __ActivDbg_h
























































typedef 
enum tagBREAKPOINT_STATE
    {
        BREAKPOINT_DELETED	= 0,
        BREAKPOINT_DISABLED	= 1,
        BREAKPOINT_ENABLED	= 2
    } 	BREAKPOINT_STATE;

typedef DWORD APPBREAKFLAGS;

#define	APPBREAKFLAG_DEBUGGER_BLOCK	( 0x1 )

#define	APPBREAKFLAG_DEBUGGER_HALT	( 0x2 )

#define	APPBREAKFLAG_STEP	( 0x10000 )

#define	APPBREAKFLAG_NESTED	( 0x20000 )

#define	APPBREAKFLAG_STEPTYPE_SOURCE	( 0 )

#define	APPBREAKFLAG_STEPTYPE_BYTECODE	( 0x100000 )

#define	APPBREAKFLAG_STEPTYPE_MACHINE	( 0x200000 )

#define	APPBREAKFLAG_STEPTYPE_MASK	( 0xf00000 )

#define	APPBREAKFLAG_IN_BREAKPOINT	( 0x80000000 )

typedef 
enum tagBREAKREASON
    {
        BREAKREASON_STEP	= 0,
        BREAKREASON_BREAKPOINT	= ( BREAKREASON_STEP + 1 ) ,
        BREAKREASON_DEBUGGER_BLOCK	= ( BREAKREASON_BREAKPOINT + 1 ) ,
        BREAKREASON_HOST_INITIATED	= ( BREAKREASON_DEBUGGER_BLOCK + 1 ) ,
        BREAKREASON_LANGUAGE_INITIATED	= ( BREAKREASON_HOST_INITIATED + 1 ) ,
        BREAKREASON_DEBUGGER_HALT	= ( BREAKREASON_LANGUAGE_INITIATED + 1 ) ,
        BREAKREASON_ERROR	= ( BREAKREASON_DEBUGGER_HALT + 1 ) ,
        BREAKREASON_JIT	= ( BREAKREASON_ERROR + 1 ) ,
        BREAKREASON_MUTATION_BREAKPOINT	= ( BREAKREASON_JIT + 1 ) 
    } 	BREAKREASON;

typedef 
enum tagBREAKRESUME_ACTION
    {
        BREAKRESUMEACTION_ABORT	= 0,
        BREAKRESUMEACTION_CONTINUE	= ( BREAKRESUMEACTION_ABORT + 1 ) ,
        BREAKRESUMEACTION_STEP_INTO	= ( BREAKRESUMEACTION_CONTINUE + 1 ) ,
        BREAKRESUMEACTION_STEP_OVER	= ( BREAKRESUMEACTION_STEP_INTO + 1 ) ,
        BREAKRESUMEACTION_STEP_OUT	= ( BREAKRESUMEACTION_STEP_OVER + 1 ) ,
        BREAKRESUMEACTION_IGNORE	= ( BREAKRESUMEACTION_STEP_OUT + 1 ) ,
        BREAKRESUMEACTION_STEP_DOCUMENT	= ( BREAKRESUMEACTION_IGNORE + 1 ) 
    } 	BREAKRESUMEACTION;

typedef 
enum tagERRORRESUMEACTION
    {
        ERRORRESUMEACTION_ReexecuteErrorStatement	= 0,
        ERRORRESUMEACTION_AbortCallAndReturnErrorToCaller	= ( ERRORRESUMEACTION_ReexecuteErrorStatement + 1 ) ,
        ERRORRESUMEACTION_SkipErrorStatement	= ( ERRORRESUMEACTION_AbortCallAndReturnErrorToCaller + 1 ) 
    } 	ERRORRESUMEACTION;

typedef 
enum tagDOCUMENTNAMETYPE
    {
        DOCUMENTNAMETYPE_APPNODE	= 0,
        DOCUMENTNAMETYPE_TITLE	= ( DOCUMENTNAMETYPE_APPNODE + 1 ) ,
        DOCUMENTNAMETYPE_FILE_TAIL	= ( DOCUMENTNAMETYPE_TITLE + 1 ) ,
        DOCUMENTNAMETYPE_URL	= ( DOCUMENTNAMETYPE_FILE_TAIL + 1 ) ,
        DOCUMENTNAMETYPE_UNIQUE_TITLE	= ( DOCUMENTNAMETYPE_URL + 1 ) ,
        DOCUMENTNAMETYPE_SOURCE_MAP_URL	= ( DOCUMENTNAMETYPE_UNIQUE_TITLE + 1 ) 
    } 	DOCUMENTNAMETYPE;

typedef WORD SOURCE_TEXT_ATTR;

#define	SOURCETEXT_ATTR_KEYWORD	( 0x1 )

#define	SOURCETEXT_ATTR_COMMENT	( 0x2 )

#define	SOURCETEXT_ATTR_NONSOURCE	( 0x4 )

#define	SOURCETEXT_ATTR_OPERATOR	( 0x8 )

#define	SOURCETEXT_ATTR_NUMBER	( 0x10 )

#define	SOURCETEXT_ATTR_STRING	( 0x20 )

#define	SOURCETEXT_ATTR_FUNCTION_START	( 0x40 )

typedef DWORD TEXT_DOC_ATTR;

#define	TEXT_DOC_ATTR_READONLY	( 0x1 )

#define	TEXT_DOC_ATTR_TYPE_PRIMARY	( 0x2 )

#define	TEXT_DOC_ATTR_TYPE_WORKER	( 0x4 )

#define	TEXT_DOC_ATTR_TYPE_SCRIPT	( 0x8 )

#define	DEBUG_TEXT_ISEXPRESSION	( 0x1 )

#define	DEBUG_TEXT_RETURNVALUE	( 0x2 )

#define	DEBUG_TEXT_NOSIDEEFFECTS	( 0x4 )

#define	DEBUG_TEXT_ALLOWBREAKPOINTS	( 0x8 )

#define	DEBUG_TEXT_ALLOWERRORREPORT	( 0x10 )

#define	DEBUG_TEXT_EVALUATETOCODECONTEXT	( 0x20 )

#define	DEBUG_TEXT_ISNONUSERCODE	( 0x40 )

#ifndef DISABLE_ACTIVDBG_INTERFACE_WRAPPERS
#ifdef _WIN64
#define IActiveScriptDebug IActiveScriptDebug64
#define IID_IActiveScriptDebug IID_IActiveScriptDebug64
#else
#define IActiveScriptDebug IActiveScriptDebug32
#define IID_IActiveScriptDebug IID_IActiveScriptDebug32
#endif
#endif


extern RPC_IF_HANDLE __MIDL_itf_activdbg_0000_0000_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_activdbg_0000_0000_v0_0_s_ifspec;

#ifndef __IActiveScriptDebug32_INTERFACE_DEFINED__
#define __IActiveScriptDebug32_INTERFACE_DEFINED__

/* interface IActiveScriptDebug32 */
/* [unique][uuid][object] */ 


EXTERN_C const IID IID_IActiveScriptDebug32;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("51973C10-CB0C-11d0-B5C9-00A0244A0E7A")
    IActiveScriptDebug32 : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetScriptTextAttributes( 
            /* [size_is][in] */ __RPC__in_ecount_full(uNumCodeChars) LPCOLESTR pstrCode,
            /* [in] */ ULONG uNumCodeChars,
            /* [in] */ __RPC__in LPCOLESTR pstrDelimiter,
            /* [in] */ DWORD dwFlags,
            /* [size_is][out][in] */ __RPC__inout_ecount_full(uNumCodeChars) SOURCE_TEXT_ATTR *pattr) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetScriptletTextAttributes( 
            /* [size_is][in] */ __RPC__in_ecount_full(uNumCodeChars) LPCOLESTR pstrCode,
            /* [in] */ ULONG uNumCodeChars,
            /* [in] */ __RPC__in LPCOLESTR pstrDelimiter,
            /* [in] */ DWORD dwFlags,
            /* [size_is][out][in] */ __RPC__inout_ecount_full(uNumCodeChars) SOURCE_TEXT_ATTR *pattr) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE EnumCodeContextsOfPosition( 
            /* [in] */ DWORD dwSourceContext,
            /* [in] */ ULONG uCharacterOffset,
            /* [in] */ ULONG uNumChars,
            /* [out] */ __RPC__deref_out_opt IEnumDebugCodeContexts **ppescc) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IActiveScriptDebug32Vtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IActiveScriptDebug32 * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IActiveScriptDebug32 * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IActiveScriptDebug32 * This);
        
        DECLSPEC_XFGVIRT(IActiveScriptDebug32, GetScriptTextAttributes)
        HRESULT ( STDMETHODCALLTYPE *GetScriptTextAttributes )( 
            __RPC__in IActiveScriptDebug32 * This,
            /* [size_is][in] */ __RPC__in_ecount_full(uNumCodeChars) LPCOLESTR pstrCode,
            /* [in] */ ULONG uNumCodeChars,
            /* [in] */ __RPC__in LPCOLESTR pstrDelimiter,
            /* [in] */ DWORD dwFlags,
            /* [size_is][out][in] */ __RPC__inout_ecount_full(uNumCodeChars) SOURCE_TEXT_ATTR *pattr);
        
        DECLSPEC_XFGVIRT(IActiveScriptDebug32, GetScriptletTextAttributes)
        HRESULT ( STDMETHODCALLTYPE *GetScriptletTextAttributes )( 
            __RPC__in IActiveScriptDebug32 * This,
            /* [size_is][in] */ __RPC__in_ecount_full(uNumCodeChars) LPCOLESTR pstrCode,
            /* [in] */ ULONG uNumCodeChars,
            /* [in] */ __RPC__in LPCOLESTR pstrDelimiter,
            /* [in] */ DWORD dwFlags,
            /* [size_is][out][in] */ __RPC__inout_ecount_full(uNumCodeChars) SOURCE_TEXT_ATTR *pattr);
        
        DECLSPEC_XFGVIRT(IActiveScriptDebug32, EnumCodeContextsOfPosition)
        HRESULT ( STDMETHODCALLTYPE *EnumCodeContextsOfPosition )( 
            __RPC__in IActiveScriptDebug32 * This,
            /* [in] */ DWORD dwSourceContext,
            /* [in] */ ULONG uCharacterOffset,
            /* [in] */ ULONG uNumChars,
            /* [out] */ __RPC__deref_out_opt IEnumDebugCodeContexts **ppescc);
        
        END_INTERFACE
    } IActiveScriptDebug32Vtbl;

    interface IActiveScriptDebug32
    {
        CONST_VTBL struct IActiveScriptDebug32Vtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IActiveScriptDebug32_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IActiveScriptDebug32_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IActiveScriptDebug32_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IActiveScriptDebug32_GetScriptTextAttributes(This,pstrCode,uNumCodeChars,pstrDelimiter,dwFlags,pattr)	\
    ( (This)->lpVtbl -> GetScriptTextAttributes(This,pstrCode,uNumCodeChars,pstrDelimiter,dwFlags,pattr) ) 

#define IActiveScriptDebug32_GetScriptletTextAttributes(This,pstrCode,uNumCodeChars,pstrDelimiter,dwFlags,pattr)	\
    ( (This)->lpVtbl -> GetScriptletTextAttributes(This,pstrCode,uNumCodeChars,pstrDelimiter,dwFlags,pattr) ) 

#define IActiveScriptDebug32_EnumCodeContextsOfPosition(This,dwSourceContext,uCharacterOffset,uNumChars,ppescc)	\
    ( (This)->lpVtbl -> EnumCodeContextsOfPosition(This,dwSourceContext,uCharacterOffset,uNumChars,ppescc) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IActiveScriptDebug32_INTERFACE_DEFINED__ */


#ifndef __IActiveScriptDebug64_INTERFACE_DEFINED__
#define __IActiveScriptDebug64_INTERFACE_DEFINED__

/* interface IActiveScriptDebug64 */
/* [unique][uuid][object] */ 


EXTERN_C const IID IID_IActiveScriptDebug64;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("bc437e23-f5b8-47f4-bb79-7d1ce5483b86")
    IActiveScriptDebug64 : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetScriptTextAttributes( 
            /* [size_is][in] */ __RPC__in_ecount_full(uNumCodeChars) LPCOLESTR pstrCode,
            /* [in] */ ULONG uNumCodeChars,
            /* [in] */ __RPC__in LPCOLESTR pstrDelimiter,
            /* [in] */ DWORD dwFlags,
            /* [size_is][out][in] */ __RPC__inout_ecount_full(uNumCodeChars) SOURCE_TEXT_ATTR *pattr) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetScriptletTextAttributes( 
            /* [size_is][in] */ __RPC__in_ecount_full(uNumCodeChars) LPCOLESTR pstrCode,
            /* [in] */ ULONG uNumCodeChars,
            /* [in] */ __RPC__in LPCOLESTR pstrDelimiter,
            /* [in] */ DWORD dwFlags,
            /* [size_is][out][in] */ __RPC__inout_ecount_full(uNumCodeChars) SOURCE_TEXT_ATTR *pattr) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE EnumCodeContextsOfPosition( 
            /* [in] */ DWORDLONG dwSourceContext,
            /* [in] */ ULONG uCharacterOffset,
            /* [in] */ ULONG uNumChars,
            /* [out] */ __RPC__deref_out_opt IEnumDebugCodeContexts **ppescc) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IActiveScriptDebug64Vtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IActiveScriptDebug64 * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IActiveScriptDebug64 * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IActiveScriptDebug64 * This);
        
        DECLSPEC_XFGVIRT(IActiveScriptDebug64, GetScriptTextAttributes)
        HRESULT ( STDMETHODCALLTYPE *GetScriptTextAttributes )( 
            __RPC__in IActiveScriptDebug64 * This,
            /* [size_is][in] */ __RPC__in_ecount_full(uNumCodeChars) LPCOLESTR pstrCode,
            /* [in] */ ULONG uNumCodeChars,
            /* [in] */ __RPC__in LPCOLESTR pstrDelimiter,
            /* [in] */ DWORD dwFlags,
            /* [size_is][out][in] */ __RPC__inout_ecount_full(uNumCodeChars) SOURCE_TEXT_ATTR *pattr);
        
        DECLSPEC_XFGVIRT(IActiveScriptDebug64, GetScriptletTextAttributes)
        HRESULT ( STDMETHODCALLTYPE *GetScriptletTextAttributes )( 
            __RPC__in IActiveScriptDebug64 * This,
            /* [size_is][in] */ __RPC__in_ecount_full(uNumCodeChars) LPCOLESTR pstrCode,
            /* [in] */ ULONG uNumCodeChars,
            /* [in] */ __RPC__in LPCOLESTR pstrDelimiter,
            /* [in] */ DWORD dwFlags,
            /* [size_is][out][in] */ __RPC__inout_ecount_full(uNumCodeChars) SOURCE_TEXT_ATTR *pattr);
        
        DECLSPEC_XFGVIRT(IActiveScriptDebug64, EnumCodeContextsOfPosition)
        HRESULT ( STDMETHODCALLTYPE *EnumCodeContextsOfPosition )( 
            __RPC__in IActiveScriptDebug64 * This,
            /* [in] */ DWORDLONG dwSourceContext,
            /* [in] */ ULONG uCharacterOffset,
            /* [in] */ ULONG uNumChars,
            /* [out] */ __RPC__deref_out_opt IEnumDebugCodeContexts **ppescc);
        
        END_INTERFACE
    } IActiveScriptDebug64Vtbl;

    interface IActiveScriptDebug64
    {
        CONST_VTBL struct IActiveScriptDebug64Vtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IActiveScriptDebug64_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IActiveScriptDebug64_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IActiveScriptDebug64_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IActiveScriptDebug64_GetScriptTextAttributes(This,pstrCode,uNumCodeChars,pstrDelimiter,dwFlags,pattr)	\
    ( (This)->lpVtbl -> GetScriptTextAttributes(This,pstrCode,uNumCodeChars,pstrDelimiter,dwFlags,pattr) ) 

#define IActiveScriptDebug64_GetScriptletTextAttributes(This,pstrCode,uNumCodeChars,pstrDelimiter,dwFlags,pattr)	\
    ( (This)->lpVtbl -> GetScriptletTextAttributes(This,pstrCode,uNumCodeChars,pstrDelimiter,dwFlags,pattr) ) 

#define IActiveScriptDebug64_EnumCodeContextsOfPosition(This,dwSourceContext,uCharacterOffset,uNumChars,ppescc)	\
    ( (This)->lpVtbl -> EnumCodeContextsOfPosition(This,dwSourceContext,uCharacterOffset,uNumChars,ppescc) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IActiveScriptDebug64_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_activdbg_0000_0002 */
/* [local] */ 

#ifndef DISABLE_ACTIVDBG_INTERFACE_WRAPPERS
#ifdef _WIN64
#define IActiveScriptSiteDebug IActiveScriptSiteDebug64
#define IID_IActiveScriptSiteDebug IID_IActiveScriptSiteDebug64
#else
#define IActiveScriptSiteDebug IActiveScriptSiteDebug32
#define IID_IActiveScriptSiteDebug IID_IActiveScriptSiteDebug32
#endif
#endif


extern RPC_IF_HANDLE __MIDL_itf_activdbg_0000_0002_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_activdbg_0000_0002_v0_0_s_ifspec;

#ifndef __IActiveScriptSiteDebug32_INTERFACE_DEFINED__
#define __IActiveScriptSiteDebug32_INTERFACE_DEFINED__

/* interface IActiveScriptSiteDebug32 */
/* [local][unique][uuid][object] */ 


EXTERN_C const IID IID_IActiveScriptSiteDebug32;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("51973C11-CB0C-11d0-B5C9-00A0244A0E7A")
    IActiveScriptSiteDebug32 : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetDocumentContextFromPosition( 
            /* [in] */ DWORD dwSourceContext,
            /* [in] */ ULONG uCharacterOffset,
            /* [in] */ ULONG uNumChars,
            /* [out] */ IDebugDocumentContext **ppsc) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetApplication( 
            /* [out] */ IDebugApplication32 **ppda) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetRootApplicationNode( 
            /* [out] */ IDebugApplicationNode **ppdanRoot) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE OnScriptErrorDebug( 
            /* [in] */ IActiveScriptErrorDebug *pErrorDebug,
            /* [out] */ BOOL *pfEnterDebugger,
            /* [out] */ BOOL *pfCallOnScriptErrorWhenContinuing) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IActiveScriptSiteDebug32Vtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IActiveScriptSiteDebug32 * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IActiveScriptSiteDebug32 * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IActiveScriptSiteDebug32 * This);
        
        DECLSPEC_XFGVIRT(IActiveScriptSiteDebug32, GetDocumentContextFromPosition)
        HRESULT ( STDMETHODCALLTYPE *GetDocumentContextFromPosition )( 
            IActiveScriptSiteDebug32 * This,
            /* [in] */ DWORD dwSourceContext,
            /* [in] */ ULONG uCharacterOffset,
            /* [in] */ ULONG uNumChars,
            /* [out] */ IDebugDocumentContext **ppsc);
        
        DECLSPEC_XFGVIRT(IActiveScriptSiteDebug32, GetApplication)
        HRESULT ( STDMETHODCALLTYPE *GetApplication )( 
            IActiveScriptSiteDebug32 * This,
            /* [out] */ IDebugApplication32 **ppda);
        
        DECLSPEC_XFGVIRT(IActiveScriptSiteDebug32, GetRootApplicationNode)
        HRESULT ( STDMETHODCALLTYPE *GetRootApplicationNode )( 
            IActiveScriptSiteDebug32 * This,
            /* [out] */ IDebugApplicationNode **ppdanRoot);
        
        DECLSPEC_XFGVIRT(IActiveScriptSiteDebug32, OnScriptErrorDebug)
        HRESULT ( STDMETHODCALLTYPE *OnScriptErrorDebug )( 
            IActiveScriptSiteDebug32 * This,
            /* [in] */ IActiveScriptErrorDebug *pErrorDebug,
            /* [out] */ BOOL *pfEnterDebugger,
            /* [out] */ BOOL *pfCallOnScriptErrorWhenContinuing);
        
        END_INTERFACE
    } IActiveScriptSiteDebug32Vtbl;

    interface IActiveScriptSiteDebug32
    {
        CONST_VTBL struct IActiveScriptSiteDebug32Vtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IActiveScriptSiteDebug32_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IActiveScriptSiteDebug32_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IActiveScriptSiteDebug32_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IActiveScriptSiteDebug32_GetDocumentContextFromPosition(This,dwSourceContext,uCharacterOffset,uNumChars,ppsc)	\
    ( (This)->lpVtbl -> GetDocumentContextFromPosition(This,dwSourceContext,uCharacterOffset,uNumChars,ppsc) ) 

#define IActiveScriptSiteDebug32_GetApplication(This,ppda)	\
    ( (This)->lpVtbl -> GetApplication(This,ppda) ) 

#define IActiveScriptSiteDebug32_GetRootApplicationNode(This,ppdanRoot)	\
    ( (This)->lpVtbl -> GetRootApplicationNode(This,ppdanRoot) ) 

#define IActiveScriptSiteDebug32_OnScriptErrorDebug(This,pErrorDebug,pfEnterDebugger,pfCallOnScriptErrorWhenContinuing)	\
    ( (This)->lpVtbl -> OnScriptErrorDebug(This,pErrorDebug,pfEnterDebugger,pfCallOnScriptErrorWhenContinuing) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IActiveScriptSiteDebug32_INTERFACE_DEFINED__ */


#ifndef __IActiveScriptSiteDebug64_INTERFACE_DEFINED__
#define __IActiveScriptSiteDebug64_INTERFACE_DEFINED__

/* interface IActiveScriptSiteDebug64 */
/* [local][unique][uuid][object] */ 


EXTERN_C const IID IID_IActiveScriptSiteDebug64;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("d6b96b0a-7463-402c-92ac-89984226942f")
    IActiveScriptSiteDebug64 : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetDocumentContextFromPosition( 
            /* [in] */ DWORDLONG dwSourceContext,
            /* [in] */ ULONG uCharacterOffset,
            /* [in] */ ULONG uNumChars,
            /* [out] */ IDebugDocumentContext **ppsc) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetApplication( 
            /* [out] */ IDebugApplication64 **ppda) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetRootApplicationNode( 
            /* [out] */ IDebugApplicationNode **ppdanRoot) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE OnScriptErrorDebug( 
            /* [in] */ IActiveScriptErrorDebug *pErrorDebug,
            /* [out] */ BOOL *pfEnterDebugger,
            /* [out] */ BOOL *pfCallOnScriptErrorWhenContinuing) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IActiveScriptSiteDebug64Vtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IActiveScriptSiteDebug64 * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IActiveScriptSiteDebug64 * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IActiveScriptSiteDebug64 * This);
        
        DECLSPEC_XFGVIRT(IActiveScriptSiteDebug64, GetDocumentContextFromPosition)
        HRESULT ( STDMETHODCALLTYPE *GetDocumentContextFromPosition )( 
            IActiveScriptSiteDebug64 * This,
            /* [in] */ DWORDLONG dwSourceContext,
            /* [in] */ ULONG uCharacterOffset,
            /* [in] */ ULONG uNumChars,
            /* [out] */ IDebugDocumentContext **ppsc);
        
        DECLSPEC_XFGVIRT(IActiveScriptSiteDebug64, GetApplication)
        HRESULT ( STDMETHODCALLTYPE *GetApplication )( 
            IActiveScriptSiteDebug64 * This,
            /* [out] */ IDebugApplication64 **ppda);
        
        DECLSPEC_XFGVIRT(IActiveScriptSiteDebug64, GetRootApplicationNode)
        HRESULT ( STDMETHODCALLTYPE *GetRootApplicationNode )( 
            IActiveScriptSiteDebug64 * This,
            /* [out] */ IDebugApplicationNode **ppdanRoot);
        
        DECLSPEC_XFGVIRT(IActiveScriptSiteDebug64, OnScriptErrorDebug)
        HRESULT ( STDMETHODCALLTYPE *OnScriptErrorDebug )( 
            IActiveScriptSiteDebug64 * This,
            /* [in] */ IActiveScriptErrorDebug *pErrorDebug,
            /* [out] */ BOOL *pfEnterDebugger,
            /* [out] */ BOOL *pfCallOnScriptErrorWhenContinuing);
        
        END_INTERFACE
    } IActiveScriptSiteDebug64Vtbl;

    interface IActiveScriptSiteDebug64
    {
        CONST_VTBL struct IActiveScriptSiteDebug64Vtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IActiveScriptSiteDebug64_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IActiveScriptSiteDebug64_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IActiveScriptSiteDebug64_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IActiveScriptSiteDebug64_GetDocumentContextFromPosition(This,dwSourceContext,uCharacterOffset,uNumChars,ppsc)	\
    ( (This)->lpVtbl -> GetDocumentContextFromPosition(This,dwSourceContext,uCharacterOffset,uNumChars,ppsc) ) 

#define IActiveScriptSiteDebug64_GetApplication(This,ppda)	\
    ( (This)->lpVtbl -> GetApplication(This,ppda) ) 

#define IActiveScriptSiteDebug64_GetRootApplicationNode(This,ppdanRoot)	\
    ( (This)->lpVtbl -> GetRootApplicationNode(This,ppdanRoot) ) 

#define IActiveScriptSiteDebug64_OnScriptErrorDebug(This,pErrorDebug,pfEnterDebugger,pfCallOnScriptErrorWhenContinuing)	\
    ( (This)->lpVtbl -> OnScriptErrorDebug(This,pErrorDebug,pfEnterDebugger,pfCallOnScriptErrorWhenContinuing) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IActiveScriptSiteDebug64_INTERFACE_DEFINED__ */


#ifndef __IActiveScriptSiteDebugEx_INTERFACE_DEFINED__
#define __IActiveScriptSiteDebugEx_INTERFACE_DEFINED__

/* interface IActiveScriptSiteDebugEx */
/* [local][unique][uuid][object] */ 


EXTERN_C const IID IID_IActiveScriptSiteDebugEx;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("BB722CCB-6AD2-41c6-B780-AF9C03EE69F5")
    IActiveScriptSiteDebugEx : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE OnCanNotJITScriptErrorDebug( 
            /* [in] */ IActiveScriptErrorDebug *pErrorDebug,
            /* [out] */ BOOL *pfCallOnScriptErrorWhenContinuing) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IActiveScriptSiteDebugExVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IActiveScriptSiteDebugEx * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IActiveScriptSiteDebugEx * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IActiveScriptSiteDebugEx * This);
        
        DECLSPEC_XFGVIRT(IActiveScriptSiteDebugEx, OnCanNotJITScriptErrorDebug)
        HRESULT ( STDMETHODCALLTYPE *OnCanNotJITScriptErrorDebug )( 
            IActiveScriptSiteDebugEx * This,
            /* [in] */ IActiveScriptErrorDebug *pErrorDebug,
            /* [out] */ BOOL *pfCallOnScriptErrorWhenContinuing);
        
        END_INTERFACE
    } IActiveScriptSiteDebugExVtbl;

    interface IActiveScriptSiteDebugEx
    {
        CONST_VTBL struct IActiveScriptSiteDebugExVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IActiveScriptSiteDebugEx_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IActiveScriptSiteDebugEx_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IActiveScriptSiteDebugEx_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IActiveScriptSiteDebugEx_OnCanNotJITScriptErrorDebug(This,pErrorDebug,pfCallOnScriptErrorWhenContinuing)	\
    ( (This)->lpVtbl -> OnCanNotJITScriptErrorDebug(This,pErrorDebug,pfCallOnScriptErrorWhenContinuing) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IActiveScriptSiteDebugEx_INTERFACE_DEFINED__ */


#ifndef __IActiveScriptErrorDebug_INTERFACE_DEFINED__
#define __IActiveScriptErrorDebug_INTERFACE_DEFINED__

/* interface IActiveScriptErrorDebug */
/* [unique][uuid][object] */ 


EXTERN_C const IID IID_IActiveScriptErrorDebug;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("51973C12-CB0C-11d0-B5C9-00A0244A0E7A")
    IActiveScriptErrorDebug : public IActiveScriptError
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetDocumentContext( 
            /* [out] */ __RPC__deref_out_opt IDebugDocumentContext **ppssc) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetStackFrame( 
            /* [out] */ __RPC__deref_out_opt IDebugStackFrame **ppdsf) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IActiveScriptErrorDebugVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IActiveScriptErrorDebug * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IActiveScriptErrorDebug * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IActiveScriptErrorDebug * This);
        
        DECLSPEC_XFGVIRT(IActiveScriptError, GetExceptionInfo)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *GetExceptionInfo )( 
            IActiveScriptErrorDebug * This,
            /* [out] */ EXCEPINFO *pexcepinfo);
        
        DECLSPEC_XFGVIRT(IActiveScriptError, GetSourcePosition)
        HRESULT ( STDMETHODCALLTYPE *GetSourcePosition )( 
            __RPC__in IActiveScriptErrorDebug * This,
            /* [out] */ __RPC__out DWORD *pdwSourceContext,
            /* [out] */ __RPC__out ULONG *pulLineNumber,
            /* [out] */ __RPC__out LONG *plCharacterPosition);
        
        DECLSPEC_XFGVIRT(IActiveScriptError, GetSourceLineText)
        HRESULT ( STDMETHODCALLTYPE *GetSourceLineText )( 
            __RPC__in IActiveScriptErrorDebug * This,
            /* [out] */ __RPC__deref_out_opt BSTR *pbstrSourceLine);
        
        DECLSPEC_XFGVIRT(IActiveScriptErrorDebug, GetDocumentContext)
        HRESULT ( STDMETHODCALLTYPE *GetDocumentContext )( 
            __RPC__in IActiveScriptErrorDebug * This,
            /* [out] */ __RPC__deref_out_opt IDebugDocumentContext **ppssc);
        
        DECLSPEC_XFGVIRT(IActiveScriptErrorDebug, GetStackFrame)
        HRESULT ( STDMETHODCALLTYPE *GetStackFrame )( 
            __RPC__in IActiveScriptErrorDebug * This,
            /* [out] */ __RPC__deref_out_opt IDebugStackFrame **ppdsf);
        
        END_INTERFACE
    } IActiveScriptErrorDebugVtbl;

    interface IActiveScriptErrorDebug
    {
        CONST_VTBL struct IActiveScriptErrorDebugVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IActiveScriptErrorDebug_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IActiveScriptErrorDebug_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IActiveScriptErrorDebug_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IActiveScriptErrorDebug_GetExceptionInfo(This,pexcepinfo)	\
    ( (This)->lpVtbl -> GetExceptionInfo(This,pexcepinfo) ) 

#define IActiveScriptErrorDebug_GetSourcePosition(This,pdwSourceContext,pulLineNumber,plCharacterPosition)	\
    ( (This)->lpVtbl -> GetSourcePosition(This,pdwSourceContext,pulLineNumber,plCharacterPosition) ) 

#define IActiveScriptErrorDebug_GetSourceLineText(This,pbstrSourceLine)	\
    ( (This)->lpVtbl -> GetSourceLineText(This,pbstrSourceLine) ) 


#define IActiveScriptErrorDebug_GetDocumentContext(This,ppssc)	\
    ( (This)->lpVtbl -> GetDocumentContext(This,ppssc) ) 

#define IActiveScriptErrorDebug_GetStackFrame(This,ppdsf)	\
    ( (This)->lpVtbl -> GetStackFrame(This,ppdsf) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IActiveScriptErrorDebug_INTERFACE_DEFINED__ */


#ifndef __IDebugCodeContext_INTERFACE_DEFINED__
#define __IDebugCodeContext_INTERFACE_DEFINED__

/* interface IDebugCodeContext */
/* [unique][uuid][object] */ 


EXTERN_C const IID IID_IDebugCodeContext;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("51973C13-CB0C-11d0-B5C9-00A0244A0E7A")
    IDebugCodeContext : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetDocumentContext( 
            /* [out] */ __RPC__deref_out_opt IDebugDocumentContext **ppsc) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetBreakPoint( 
            /* [in] */ BREAKPOINT_STATE bps) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IDebugCodeContextVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IDebugCodeContext * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IDebugCodeContext * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IDebugCodeContext * This);
        
        DECLSPEC_XFGVIRT(IDebugCodeContext, GetDocumentContext)
        HRESULT ( STDMETHODCALLTYPE *GetDocumentContext )( 
            __RPC__in IDebugCodeContext * This,
            /* [out] */ __RPC__deref_out_opt IDebugDocumentContext **ppsc);
        
        DECLSPEC_XFGVIRT(IDebugCodeContext, SetBreakPoint)
        HRESULT ( STDMETHODCALLTYPE *SetBreakPoint )( 
            __RPC__in IDebugCodeContext * This,
            /* [in] */ BREAKPOINT_STATE bps);
        
        END_INTERFACE
    } IDebugCodeContextVtbl;

    interface IDebugCodeContext
    {
        CONST_VTBL struct IDebugCodeContextVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IDebugCodeContext_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IDebugCodeContext_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IDebugCodeContext_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IDebugCodeContext_GetDocumentContext(This,ppsc)	\
    ( (This)->lpVtbl -> GetDocumentContext(This,ppsc) ) 

#define IDebugCodeContext_SetBreakPoint(This,bps)	\
    ( (This)->lpVtbl -> SetBreakPoint(This,bps) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IDebugCodeContext_INTERFACE_DEFINED__ */


#ifndef __IDebugExpression_INTERFACE_DEFINED__
#define __IDebugExpression_INTERFACE_DEFINED__

/* interface IDebugExpression */
/* [unique][uuid][object] */ 


EXTERN_C const IID IID_IDebugExpression;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("51973C14-CB0C-11d0-B5C9-00A0244A0E7A")
    IDebugExpression : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE Start( 
            /* [in] */ __RPC__in_opt IDebugExpressionCallBack *pdecb) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Abort( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE QueryIsComplete( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetResultAsString( 
            /* [out] */ __RPC__out HRESULT *phrResult,
            /* [out] */ __RPC__deref_out_opt BSTR *pbstrResult) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetResultAsDebugProperty( 
            /* [out] */ __RPC__out HRESULT *phrResult,
            /* [out] */ __RPC__deref_out_opt IDebugProperty **ppdp) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IDebugExpressionVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IDebugExpression * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IDebugExpression * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IDebugExpression * This);
        
        DECLSPEC_XFGVIRT(IDebugExpression, Start)
        HRESULT ( STDMETHODCALLTYPE *Start )( 
            __RPC__in IDebugExpression * This,
            /* [in] */ __RPC__in_opt IDebugExpressionCallBack *pdecb);
        
        DECLSPEC_XFGVIRT(IDebugExpression, Abort)
        HRESULT ( STDMETHODCALLTYPE *Abort )( 
            __RPC__in IDebugExpression * This);
        
        DECLSPEC_XFGVIRT(IDebugExpression, QueryIsComplete)
        HRESULT ( STDMETHODCALLTYPE *QueryIsComplete )( 
            __RPC__in IDebugExpression * This);
        
        DECLSPEC_XFGVIRT(IDebugExpression, GetResultAsString)
        HRESULT ( STDMETHODCALLTYPE *GetResultAsString )( 
            __RPC__in IDebugExpression * This,
            /* [out] */ __RPC__out HRESULT *phrResult,
            /* [out] */ __RPC__deref_out_opt BSTR *pbstrResult);
        
        DECLSPEC_XFGVIRT(IDebugExpression, GetResultAsDebugProperty)
        HRESULT ( STDMETHODCALLTYPE *GetResultAsDebugProperty )( 
            __RPC__in IDebugExpression * This,
            /* [out] */ __RPC__out HRESULT *phrResult,
            /* [out] */ __RPC__deref_out_opt IDebugProperty **ppdp);
        
        END_INTERFACE
    } IDebugExpressionVtbl;

    interface IDebugExpression
    {
        CONST_VTBL struct IDebugExpressionVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IDebugExpression_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IDebugExpression_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IDebugExpression_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IDebugExpression_Start(This,pdecb)	\
    ( (This)->lpVtbl -> Start(This,pdecb) ) 

#define IDebugExpression_Abort(This)	\
    ( (This)->lpVtbl -> Abort(This) ) 

#define IDebugExpression_QueryIsComplete(This)	\
    ( (This)->lpVtbl -> QueryIsComplete(This) ) 

#define IDebugExpression_GetResultAsString(This,phrResult,pbstrResult)	\
    ( (This)->lpVtbl -> GetResultAsString(This,phrResult,pbstrResult) ) 

#define IDebugExpression_GetResultAsDebugProperty(This,phrResult,ppdp)	\
    ( (This)->lpVtbl -> GetResultAsDebugProperty(This,phrResult,ppdp) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IDebugExpression_INTERFACE_DEFINED__ */


#ifndef __IDebugExpressionContext_INTERFACE_DEFINED__
#define __IDebugExpressionContext_INTERFACE_DEFINED__

/* interface IDebugExpressionContext */
/* [unique][helpstring][uuid][object] */ 


EXTERN_C const IID IID_IDebugExpressionContext;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("51973C15-CB0C-11d0-B5C9-00A0244A0E7A")
    IDebugExpressionContext : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE ParseLanguageText( 
            /* [in] */ __RPC__in LPCOLESTR pstrCode,
            /* [in] */ UINT nRadix,
            /* [in] */ __RPC__in LPCOLESTR pstrDelimiter,
            /* [in] */ DWORD dwFlags,
            /* [out] */ __RPC__deref_out_opt IDebugExpression **ppe) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetLanguageInfo( 
            /* [out] */ __RPC__deref_out_opt BSTR *pbstrLanguageName,
            /* [out] */ __RPC__out GUID *pLanguageID) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IDebugExpressionContextVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IDebugExpressionContext * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IDebugExpressionContext * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IDebugExpressionContext * This);
        
        DECLSPEC_XFGVIRT(IDebugExpressionContext, ParseLanguageText)
        HRESULT ( STDMETHODCALLTYPE *ParseLanguageText )( 
            __RPC__in IDebugExpressionContext * This,
            /* [in] */ __RPC__in LPCOLESTR pstrCode,
            /* [in] */ UINT nRadix,
            /* [in] */ __RPC__in LPCOLESTR pstrDelimiter,
            /* [in] */ DWORD dwFlags,
            /* [out] */ __RPC__deref_out_opt IDebugExpression **ppe);
        
        DECLSPEC_XFGVIRT(IDebugExpressionContext, GetLanguageInfo)
        HRESULT ( STDMETHODCALLTYPE *GetLanguageInfo )( 
            __RPC__in IDebugExpressionContext * This,
            /* [out] */ __RPC__deref_out_opt BSTR *pbstrLanguageName,
            /* [out] */ __RPC__out GUID *pLanguageID);
        
        END_INTERFACE
    } IDebugExpressionContextVtbl;

    interface IDebugExpressionContext
    {
        CONST_VTBL struct IDebugExpressionContextVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IDebugExpressionContext_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IDebugExpressionContext_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IDebugExpressionContext_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IDebugExpressionContext_ParseLanguageText(This,pstrCode,nRadix,pstrDelimiter,dwFlags,ppe)	\
    ( (This)->lpVtbl -> ParseLanguageText(This,pstrCode,nRadix,pstrDelimiter,dwFlags,ppe) ) 

#define IDebugExpressionContext_GetLanguageInfo(This,pbstrLanguageName,pLanguageID)	\
    ( (This)->lpVtbl -> GetLanguageInfo(This,pbstrLanguageName,pLanguageID) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IDebugExpressionContext_INTERFACE_DEFINED__ */


#ifndef __IDebugExpressionCallBack_INTERFACE_DEFINED__
#define __IDebugExpressionCallBack_INTERFACE_DEFINED__

/* interface IDebugExpressionCallBack */
/* [unique][uuid][object] */ 


EXTERN_C const IID IID_IDebugExpressionCallBack;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("51973C16-CB0C-11d0-B5C9-00A0244A0E7A")
    IDebugExpressionCallBack : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE onComplete( void) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IDebugExpressionCallBackVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IDebugExpressionCallBack * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IDebugExpressionCallBack * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IDebugExpressionCallBack * This);
        
        DECLSPEC_XFGVIRT(IDebugExpressionCallBack, onComplete)
        HRESULT ( STDMETHODCALLTYPE *onComplete )( 
            __RPC__in IDebugExpressionCallBack * This);
        
        END_INTERFACE
    } IDebugExpressionCallBackVtbl;

    interface IDebugExpressionCallBack
    {
        CONST_VTBL struct IDebugExpressionCallBackVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IDebugExpressionCallBack_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IDebugExpressionCallBack_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IDebugExpressionCallBack_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IDebugExpressionCallBack_onComplete(This)	\
    ( (This)->lpVtbl -> onComplete(This) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IDebugExpressionCallBack_INTERFACE_DEFINED__ */


#ifndef __IDebugStackFrame_INTERFACE_DEFINED__
#define __IDebugStackFrame_INTERFACE_DEFINED__

/* interface IDebugStackFrame */
/* [unique][uuid][object] */ 


EXTERN_C const IID IID_IDebugStackFrame;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("51973C17-CB0C-11d0-B5C9-00A0244A0E7A")
    IDebugStackFrame : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetCodeContext( 
            /* [out] */ __RPC__deref_out_opt IDebugCodeContext **ppcc) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetDescriptionString( 
            /* [in] */ BOOL fLong,
            /* [out] */ __RPC__deref_out_opt BSTR *pbstrDescription) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetLanguageString( 
            /* [in] */ BOOL fLong,
            /* [out] */ __RPC__deref_out_opt BSTR *pbstrLanguage) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetThread( 
            /* [out] */ __RPC__deref_out_opt IDebugApplicationThread **ppat) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetDebugProperty( 
            /* [out] */ __RPC__deref_out_opt IDebugProperty **ppDebugProp) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IDebugStackFrameVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IDebugStackFrame * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IDebugStackFrame * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IDebugStackFrame * This);
        
        DECLSPEC_XFGVIRT(IDebugStackFrame, GetCodeContext)
        HRESULT ( STDMETHODCALLTYPE *GetCodeContext )( 
            __RPC__in IDebugStackFrame * This,
            /* [out] */ __RPC__deref_out_opt IDebugCodeContext **ppcc);
        
        DECLSPEC_XFGVIRT(IDebugStackFrame, GetDescriptionString)
        HRESULT ( STDMETHODCALLTYPE *GetDescriptionString )( 
            __RPC__in IDebugStackFrame * This,
            /* [in] */ BOOL fLong,
            /* [out] */ __RPC__deref_out_opt BSTR *pbstrDescription);
        
        DECLSPEC_XFGVIRT(IDebugStackFrame, GetLanguageString)
        HRESULT ( STDMETHODCALLTYPE *GetLanguageString )( 
            __RPC__in IDebugStackFrame * This,
            /* [in] */ BOOL fLong,
            /* [out] */ __RPC__deref_out_opt BSTR *pbstrLanguage);
        
        DECLSPEC_XFGVIRT(IDebugStackFrame, GetThread)
        HRESULT ( STDMETHODCALLTYPE *GetThread )( 
            __RPC__in IDebugStackFrame * This,
            /* [out] */ __RPC__deref_out_opt IDebugApplicationThread **ppat);
        
        DECLSPEC_XFGVIRT(IDebugStackFrame, GetDebugProperty)
        HRESULT ( STDMETHODCALLTYPE *GetDebugProperty )( 
            __RPC__in IDebugStackFrame * This,
            /* [out] */ __RPC__deref_out_opt IDebugProperty **ppDebugProp);
        
        END_INTERFACE
    } IDebugStackFrameVtbl;

    interface IDebugStackFrame
    {
        CONST_VTBL struct IDebugStackFrameVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IDebugStackFrame_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IDebugStackFrame_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IDebugStackFrame_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IDebugStackFrame_GetCodeContext(This,ppcc)	\
    ( (This)->lpVtbl -> GetCodeContext(This,ppcc) ) 

#define IDebugStackFrame_GetDescriptionString(This,fLong,pbstrDescription)	\
    ( (This)->lpVtbl -> GetDescriptionString(This,fLong,pbstrDescription) ) 

#define IDebugStackFrame_GetLanguageString(This,fLong,pbstrLanguage)	\
    ( (This)->lpVtbl -> GetLanguageString(This,fLong,pbstrLanguage) ) 

#define IDebugStackFrame_GetThread(This,ppat)	\
    ( (This)->lpVtbl -> GetThread(This,ppat) ) 

#define IDebugStackFrame_GetDebugProperty(This,ppDebugProp)	\
    ( (This)->lpVtbl -> GetDebugProperty(This,ppDebugProp) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IDebugStackFrame_INTERFACE_DEFINED__ */


#ifndef __IDebugStackFrameSniffer_INTERFACE_DEFINED__
#define __IDebugStackFrameSniffer_INTERFACE_DEFINED__

/* interface IDebugStackFrameSniffer */
/* [unique][uuid][object] */ 


EXTERN_C const IID IID_IDebugStackFrameSniffer;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("51973C18-CB0C-11d0-B5C9-00A0244A0E7A")
    IDebugStackFrameSniffer : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE EnumStackFrames( 
            /* [out] */ __RPC__deref_out_opt IEnumDebugStackFrames **ppedsf) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IDebugStackFrameSnifferVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IDebugStackFrameSniffer * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IDebugStackFrameSniffer * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IDebugStackFrameSniffer * This);
        
        DECLSPEC_XFGVIRT(IDebugStackFrameSniffer, EnumStackFrames)
        HRESULT ( STDMETHODCALLTYPE *EnumStackFrames )( 
            __RPC__in IDebugStackFrameSniffer * This,
            /* [out] */ __RPC__deref_out_opt IEnumDebugStackFrames **ppedsf);
        
        END_INTERFACE
    } IDebugStackFrameSnifferVtbl;

    interface IDebugStackFrameSniffer
    {
        CONST_VTBL struct IDebugStackFrameSnifferVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IDebugStackFrameSniffer_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IDebugStackFrameSniffer_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IDebugStackFrameSniffer_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IDebugStackFrameSniffer_EnumStackFrames(This,ppedsf)	\
    ( (This)->lpVtbl -> EnumStackFrames(This,ppedsf) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IDebugStackFrameSniffer_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_activdbg_0000_0012 */
/* [local] */ 

#ifndef DISABLE_ACTIVDBG_INTERFACE_WRAPPERS
#ifdef _WIN64
#define IDebugStackFrameSnifferEx IDebugStackFrameSnifferEx64
#define IID_IDebugStackFrameSnifferEx IID_IDebugStackFrameSnifferEx64
#define EnumStackFramesEx EnumStackFramesEx64
#else
#define IDebugStackFrameSnifferEx IDebugStackFrameSnifferEx32
#define IID_IDebugStackFrameSnifferEx IID_IDebugStackFrameSnifferEx32
#define EnumStackFramesEx EnumStackFramesEx32
#endif
#endif


extern RPC_IF_HANDLE __MIDL_itf_activdbg_0000_0012_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_activdbg_0000_0012_v0_0_s_ifspec;

#ifndef __IDebugStackFrameSnifferEx32_INTERFACE_DEFINED__
#define __IDebugStackFrameSnifferEx32_INTERFACE_DEFINED__

/* interface IDebugStackFrameSnifferEx32 */
/* [unique][uuid][object] */ 


EXTERN_C const IID IID_IDebugStackFrameSnifferEx32;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("51973C19-CB0C-11d0-B5C9-00A0244A0E7A")
    IDebugStackFrameSnifferEx32 : public IDebugStackFrameSniffer
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE EnumStackFramesEx32( 
            /* [in] */ DWORD dwSpMin,
            /* [out] */ __RPC__deref_out_opt IEnumDebugStackFrames **ppedsf) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IDebugStackFrameSnifferEx32Vtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IDebugStackFrameSnifferEx32 * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IDebugStackFrameSnifferEx32 * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IDebugStackFrameSnifferEx32 * This);
        
        DECLSPEC_XFGVIRT(IDebugStackFrameSniffer, EnumStackFrames)
        HRESULT ( STDMETHODCALLTYPE *EnumStackFrames )( 
            __RPC__in IDebugStackFrameSnifferEx32 * This,
            /* [out] */ __RPC__deref_out_opt IEnumDebugStackFrames **ppedsf);
        
        DECLSPEC_XFGVIRT(IDebugStackFrameSnifferEx32, EnumStackFramesEx32)
        HRESULT ( STDMETHODCALLTYPE *EnumStackFramesEx32 )( 
            __RPC__in IDebugStackFrameSnifferEx32 * This,
            /* [in] */ DWORD dwSpMin,
            /* [out] */ __RPC__deref_out_opt IEnumDebugStackFrames **ppedsf);
        
        END_INTERFACE
    } IDebugStackFrameSnifferEx32Vtbl;

    interface IDebugStackFrameSnifferEx32
    {
        CONST_VTBL struct IDebugStackFrameSnifferEx32Vtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IDebugStackFrameSnifferEx32_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IDebugStackFrameSnifferEx32_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IDebugStackFrameSnifferEx32_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IDebugStackFrameSnifferEx32_EnumStackFrames(This,ppedsf)	\
    ( (This)->lpVtbl -> EnumStackFrames(This,ppedsf) ) 


#define IDebugStackFrameSnifferEx32_EnumStackFramesEx32(This,dwSpMin,ppedsf)	\
    ( (This)->lpVtbl -> EnumStackFramesEx32(This,dwSpMin,ppedsf) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IDebugStackFrameSnifferEx32_INTERFACE_DEFINED__ */


#ifndef __IDebugStackFrameSnifferEx64_INTERFACE_DEFINED__
#define __IDebugStackFrameSnifferEx64_INTERFACE_DEFINED__

/* interface IDebugStackFrameSnifferEx64 */
/* [unique][uuid][object] */ 


EXTERN_C const IID IID_IDebugStackFrameSnifferEx64;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("8cd12af4-49c1-4d52-8d8a-c146f47581aa")
    IDebugStackFrameSnifferEx64 : public IDebugStackFrameSniffer
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE EnumStackFramesEx64( 
            /* [in] */ DWORDLONG dwSpMin,
            /* [out] */ __RPC__deref_out_opt IEnumDebugStackFrames64 **ppedsf) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IDebugStackFrameSnifferEx64Vtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IDebugStackFrameSnifferEx64 * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IDebugStackFrameSnifferEx64 * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IDebugStackFrameSnifferEx64 * This);
        
        DECLSPEC_XFGVIRT(IDebugStackFrameSniffer, EnumStackFrames)
        HRESULT ( STDMETHODCALLTYPE *EnumStackFrames )( 
            __RPC__in IDebugStackFrameSnifferEx64 * This,
            /* [out] */ __RPC__deref_out_opt IEnumDebugStackFrames **ppedsf);
        
        DECLSPEC_XFGVIRT(IDebugStackFrameSnifferEx64, EnumStackFramesEx64)
        HRESULT ( STDMETHODCALLTYPE *EnumStackFramesEx64 )( 
            __RPC__in IDebugStackFrameSnifferEx64 * This,
            /* [in] */ DWORDLONG dwSpMin,
            /* [out] */ __RPC__deref_out_opt IEnumDebugStackFrames64 **ppedsf);
        
        END_INTERFACE
    } IDebugStackFrameSnifferEx64Vtbl;

    interface IDebugStackFrameSnifferEx64
    {
        CONST_VTBL struct IDebugStackFrameSnifferEx64Vtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IDebugStackFrameSnifferEx64_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IDebugStackFrameSnifferEx64_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IDebugStackFrameSnifferEx64_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IDebugStackFrameSnifferEx64_EnumStackFrames(This,ppedsf)	\
    ( (This)->lpVtbl -> EnumStackFrames(This,ppedsf) ) 


#define IDebugStackFrameSnifferEx64_EnumStackFramesEx64(This,dwSpMin,ppedsf)	\
    ( (This)->lpVtbl -> EnumStackFramesEx64(This,dwSpMin,ppedsf) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IDebugStackFrameSnifferEx64_INTERFACE_DEFINED__ */


#ifndef __IDebugSyncOperation_INTERFACE_DEFINED__
#define __IDebugSyncOperation_INTERFACE_DEFINED__

/* interface IDebugSyncOperation */
/* [local][unique][uuid][object] */ 


EXTERN_C const IID IID_IDebugSyncOperation;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("51973C1a-CB0C-11d0-B5C9-00A0244A0E7A")
    IDebugSyncOperation : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetTargetThread( 
            /* [out] */ IDebugApplicationThread **ppatTarget) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Execute( 
            /* [out] */ IUnknown **ppunkResult) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE InProgressAbort( void) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IDebugSyncOperationVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IDebugSyncOperation * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IDebugSyncOperation * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IDebugSyncOperation * This);
        
        DECLSPEC_XFGVIRT(IDebugSyncOperation, GetTargetThread)
        HRESULT ( STDMETHODCALLTYPE *GetTargetThread )( 
            IDebugSyncOperation * This,
            /* [out] */ IDebugApplicationThread **ppatTarget);
        
        DECLSPEC_XFGVIRT(IDebugSyncOperation, Execute)
        HRESULT ( STDMETHODCALLTYPE *Execute )( 
            IDebugSyncOperation * This,
            /* [out] */ IUnknown **ppunkResult);
        
        DECLSPEC_XFGVIRT(IDebugSyncOperation, InProgressAbort)
        HRESULT ( STDMETHODCALLTYPE *InProgressAbort )( 
            IDebugSyncOperation * This);
        
        END_INTERFACE
    } IDebugSyncOperationVtbl;

    interface IDebugSyncOperation
    {
        CONST_VTBL struct IDebugSyncOperationVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IDebugSyncOperation_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IDebugSyncOperation_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IDebugSyncOperation_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IDebugSyncOperation_GetTargetThread(This,ppatTarget)	\
    ( (This)->lpVtbl -> GetTargetThread(This,ppatTarget) ) 

#define IDebugSyncOperation_Execute(This,ppunkResult)	\
    ( (This)->lpVtbl -> Execute(This,ppunkResult) ) 

#define IDebugSyncOperation_InProgressAbort(This)	\
    ( (This)->lpVtbl -> InProgressAbort(This) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IDebugSyncOperation_INTERFACE_DEFINED__ */


#ifndef __IDebugAsyncOperation_INTERFACE_DEFINED__
#define __IDebugAsyncOperation_INTERFACE_DEFINED__

/* interface IDebugAsyncOperation */
/* [local][unique][uuid][object] */ 


EXTERN_C const IID IID_IDebugAsyncOperation;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("51973C1b-CB0C-11d0-B5C9-00A0244A0E7A")
    IDebugAsyncOperation : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetSyncDebugOperation( 
            /* [out] */ IDebugSyncOperation **ppsdo) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Start( 
            IDebugAsyncOperationCallBack *padocb) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Abort( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE QueryIsComplete( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetResult( 
            /* [out] */ HRESULT *phrResult,
            /* [out] */ IUnknown **ppunkResult) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IDebugAsyncOperationVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IDebugAsyncOperation * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IDebugAsyncOperation * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IDebugAsyncOperation * This);
        
        DECLSPEC_XFGVIRT(IDebugAsyncOperation, GetSyncDebugOperation)
        HRESULT ( STDMETHODCALLTYPE *GetSyncDebugOperation )( 
            IDebugAsyncOperation * This,
            /* [out] */ IDebugSyncOperation **ppsdo);
        
        DECLSPEC_XFGVIRT(IDebugAsyncOperation, Start)
        HRESULT ( STDMETHODCALLTYPE *Start )( 
            IDebugAsyncOperation * This,
            IDebugAsyncOperationCallBack *padocb);
        
        DECLSPEC_XFGVIRT(IDebugAsyncOperation, Abort)
        HRESULT ( STDMETHODCALLTYPE *Abort )( 
            IDebugAsyncOperation * This);
        
        DECLSPEC_XFGVIRT(IDebugAsyncOperation, QueryIsComplete)
        HRESULT ( STDMETHODCALLTYPE *QueryIsComplete )( 
            IDebugAsyncOperation * This);
        
        DECLSPEC_XFGVIRT(IDebugAsyncOperation, GetResult)
        HRESULT ( STDMETHODCALLTYPE *GetResult )( 
            IDebugAsyncOperation * This,
            /* [out] */ HRESULT *phrResult,
            /* [out] */ IUnknown **ppunkResult);
        
        END_INTERFACE
    } IDebugAsyncOperationVtbl;

    interface IDebugAsyncOperation
    {
        CONST_VTBL struct IDebugAsyncOperationVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IDebugAsyncOperation_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IDebugAsyncOperation_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IDebugAsyncOperation_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IDebugAsyncOperation_GetSyncDebugOperation(This,ppsdo)	\
    ( (This)->lpVtbl -> GetSyncDebugOperation(This,ppsdo) ) 

#define IDebugAsyncOperation_Start(This,padocb)	\
    ( (This)->lpVtbl -> Start(This,padocb) ) 

#define IDebugAsyncOperation_Abort(This)	\
    ( (This)->lpVtbl -> Abort(This) ) 

#define IDebugAsyncOperation_QueryIsComplete(This)	\
    ( (This)->lpVtbl -> QueryIsComplete(This) ) 

#define IDebugAsyncOperation_GetResult(This,phrResult,ppunkResult)	\
    ( (This)->lpVtbl -> GetResult(This,phrResult,ppunkResult) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IDebugAsyncOperation_INTERFACE_DEFINED__ */


#ifndef __IDebugAsyncOperationCallBack_INTERFACE_DEFINED__
#define __IDebugAsyncOperationCallBack_INTERFACE_DEFINED__

/* interface IDebugAsyncOperationCallBack */
/* [local][unique][uuid][object] */ 


EXTERN_C const IID IID_IDebugAsyncOperationCallBack;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("51973C1c-CB0C-11d0-B5C9-00A0244A0E7A")
    IDebugAsyncOperationCallBack : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE onComplete( void) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IDebugAsyncOperationCallBackVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IDebugAsyncOperationCallBack * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IDebugAsyncOperationCallBack * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IDebugAsyncOperationCallBack * This);
        
        DECLSPEC_XFGVIRT(IDebugAsyncOperationCallBack, onComplete)
        HRESULT ( STDMETHODCALLTYPE *onComplete )( 
            IDebugAsyncOperationCallBack * This);
        
        END_INTERFACE
    } IDebugAsyncOperationCallBackVtbl;

    interface IDebugAsyncOperationCallBack
    {
        CONST_VTBL struct IDebugAsyncOperationCallBackVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IDebugAsyncOperationCallBack_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IDebugAsyncOperationCallBack_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IDebugAsyncOperationCallBack_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IDebugAsyncOperationCallBack_onComplete(This)	\
    ( (This)->lpVtbl -> onComplete(This) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IDebugAsyncOperationCallBack_INTERFACE_DEFINED__ */


#ifndef __IEnumDebugCodeContexts_INTERFACE_DEFINED__
#define __IEnumDebugCodeContexts_INTERFACE_DEFINED__

/* interface IEnumDebugCodeContexts */
/* [unique][helpstring][uuid][object] */ 


EXTERN_C const IID IID_IEnumDebugCodeContexts;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("51973C1d-CB0C-11d0-B5C9-00A0244A0E7A")
    IEnumDebugCodeContexts : public IUnknown
    {
    public:
        virtual /* [local] */ HRESULT __stdcall Next( 
            /* [in] */ ULONG celt,
            /* [out] */ IDebugCodeContext **pscc,
            /* [out] */ ULONG *pceltFetched) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Skip( 
            /* [in] */ ULONG celt) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Reset( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Clone( 
            /* [out] */ __RPC__deref_out_opt IEnumDebugCodeContexts **ppescc) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IEnumDebugCodeContextsVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IEnumDebugCodeContexts * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IEnumDebugCodeContexts * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IEnumDebugCodeContexts * This);
        
        DECLSPEC_XFGVIRT(IEnumDebugCodeContexts, Next)
        /* [local] */ HRESULT ( __stdcall *Next )( 
            IEnumDebugCodeContexts * This,
            /* [in] */ ULONG celt,
            /* [out] */ IDebugCodeContext **pscc,
            /* [out] */ ULONG *pceltFetched);
        
        DECLSPEC_XFGVIRT(IEnumDebugCodeContexts, Skip)
        HRESULT ( STDMETHODCALLTYPE *Skip )( 
            __RPC__in IEnumDebugCodeContexts * This,
            /* [in] */ ULONG celt);
        
        DECLSPEC_XFGVIRT(IEnumDebugCodeContexts, Reset)
        HRESULT ( STDMETHODCALLTYPE *Reset )( 
            __RPC__in IEnumDebugCodeContexts * This);
        
        DECLSPEC_XFGVIRT(IEnumDebugCodeContexts, Clone)
        HRESULT ( STDMETHODCALLTYPE *Clone )( 
            __RPC__in IEnumDebugCodeContexts * This,
            /* [out] */ __RPC__deref_out_opt IEnumDebugCodeContexts **ppescc);
        
        END_INTERFACE
    } IEnumDebugCodeContextsVtbl;

    interface IEnumDebugCodeContexts
    {
        CONST_VTBL struct IEnumDebugCodeContextsVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IEnumDebugCodeContexts_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IEnumDebugCodeContexts_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IEnumDebugCodeContexts_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IEnumDebugCodeContexts_Next(This,celt,pscc,pceltFetched)	\
    ( (This)->lpVtbl -> Next(This,celt,pscc,pceltFetched) ) 

#define IEnumDebugCodeContexts_Skip(This,celt)	\
    ( (This)->lpVtbl -> Skip(This,celt) ) 

#define IEnumDebugCodeContexts_Reset(This)	\
    ( (This)->lpVtbl -> Reset(This) ) 

#define IEnumDebugCodeContexts_Clone(This,ppescc)	\
    ( (This)->lpVtbl -> Clone(This,ppescc) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */



/* [call_as] */ HRESULT __stdcall IEnumDebugCodeContexts_RemoteNext_Proxy( 
    __RPC__in IEnumDebugCodeContexts * This,
    /* [in] */ ULONG celt,
    /* [length_is][size_is][out] */ __RPC__out_ecount_part(celt, *pceltFetched) IDebugCodeContext **pscc,
    /* [out] */ __RPC__out ULONG *pceltFetched);


void __RPC_STUB IEnumDebugCodeContexts_RemoteNext_Stub(
    IRpcStubBuffer *This,
    IRpcChannelBuffer *_pRpcChannelBuffer,
    PRPC_MESSAGE _pRpcMessage,
    DWORD *_pdwStubPhase);



#endif 	/* __IEnumDebugCodeContexts_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_activdbg_0000_0018 */
/* [local] */ 

typedef struct tagDebugStackFrameDescriptor
    {
    IDebugStackFrame *pdsf;
    DWORD dwMin;
    DWORD dwLim;
    BOOL fFinal;
    IUnknown *punkFinal;
    } 	DebugStackFrameDescriptor;

typedef struct tagDebugStackFrameDescriptor64
    {
    IDebugStackFrame *pdsf;
    DWORDLONG dwMin;
    DWORDLONG dwLim;
    BOOL fFinal;
    IUnknown *punkFinal;
    } 	DebugStackFrameDescriptor64;



extern RPC_IF_HANDLE __MIDL_itf_activdbg_0000_0018_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_activdbg_0000_0018_v0_0_s_ifspec;

#ifndef __IEnumDebugStackFrames_INTERFACE_DEFINED__
#define __IEnumDebugStackFrames_INTERFACE_DEFINED__

/* interface IEnumDebugStackFrames */
/* [unique][helpstring][uuid][object] */ 


EXTERN_C const IID IID_IEnumDebugStackFrames;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("51973C1e-CB0C-11d0-B5C9-00A0244A0E7A")
    IEnumDebugStackFrames : public IUnknown
    {
    public:
        virtual /* [local] */ HRESULT __stdcall Next( 
            /* [in] */ ULONG celt,
            /* [out] */ DebugStackFrameDescriptor *prgdsfd,
            /* [out] */ ULONG *pceltFetched) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Skip( 
            /* [in] */ ULONG celt) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Reset( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Clone( 
            /* [out] */ __RPC__deref_out_opt IEnumDebugStackFrames **ppedsf) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IEnumDebugStackFramesVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IEnumDebugStackFrames * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IEnumDebugStackFrames * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IEnumDebugStackFrames * This);
        
        DECLSPEC_XFGVIRT(IEnumDebugStackFrames, Next)
        /* [local] */ HRESULT ( __stdcall *Next )( 
            IEnumDebugStackFrames * This,
            /* [in] */ ULONG celt,
            /* [out] */ DebugStackFrameDescriptor *prgdsfd,
            /* [out] */ ULONG *pceltFetched);
        
        DECLSPEC_XFGVIRT(IEnumDebugStackFrames, Skip)
        HRESULT ( STDMETHODCALLTYPE *Skip )( 
            __RPC__in IEnumDebugStackFrames * This,
            /* [in] */ ULONG celt);
        
        DECLSPEC_XFGVIRT(IEnumDebugStackFrames, Reset)
        HRESULT ( STDMETHODCALLTYPE *Reset )( 
            __RPC__in IEnumDebugStackFrames * This);
        
        DECLSPEC_XFGVIRT(IEnumDebugStackFrames, Clone)
        HRESULT ( STDMETHODCALLTYPE *Clone )( 
            __RPC__in IEnumDebugStackFrames * This,
            /* [out] */ __RPC__deref_out_opt IEnumDebugStackFrames **ppedsf);
        
        END_INTERFACE
    } IEnumDebugStackFramesVtbl;

    interface IEnumDebugStackFrames
    {
        CONST_VTBL struct IEnumDebugStackFramesVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IEnumDebugStackFrames_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IEnumDebugStackFrames_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IEnumDebugStackFrames_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IEnumDebugStackFrames_Next(This,celt,prgdsfd,pceltFetched)	\
    ( (This)->lpVtbl -> Next(This,celt,prgdsfd,pceltFetched) ) 

#define IEnumDebugStackFrames_Skip(This,celt)	\
    ( (This)->lpVtbl -> Skip(This,celt) ) 

#define IEnumDebugStackFrames_Reset(This)	\
    ( (This)->lpVtbl -> Reset(This) ) 

#define IEnumDebugStackFrames_Clone(This,ppedsf)	\
    ( (This)->lpVtbl -> Clone(This,ppedsf) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */



/* [call_as] */ HRESULT __stdcall IEnumDebugStackFrames_RemoteNext_Proxy( 
    __RPC__in IEnumDebugStackFrames * This,
    /* [in] */ ULONG celt,
    /* [length_is][size_is][out] */ __RPC__out_ecount_part(celt, *pceltFetched) DebugStackFrameDescriptor *prgdsfd,
    /* [out] */ __RPC__out ULONG *pceltFetched);


void __RPC_STUB IEnumDebugStackFrames_RemoteNext_Stub(
    IRpcStubBuffer *This,
    IRpcChannelBuffer *_pRpcChannelBuffer,
    PRPC_MESSAGE _pRpcMessage,
    DWORD *_pdwStubPhase);



#endif 	/* __IEnumDebugStackFrames_INTERFACE_DEFINED__ */


#ifndef __IEnumDebugStackFrames64_INTERFACE_DEFINED__
#define __IEnumDebugStackFrames64_INTERFACE_DEFINED__

/* interface IEnumDebugStackFrames64 */
/* [unique][helpstring][uuid][object] */ 


EXTERN_C const IID IID_IEnumDebugStackFrames64;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("0dc38853-c1b0-4176-a984-b298361027af")
    IEnumDebugStackFrames64 : public IEnumDebugStackFrames
    {
    public:
        virtual /* [local] */ HRESULT __stdcall Next64( 
            /* [in] */ ULONG celt,
            /* [out] */ DebugStackFrameDescriptor64 *prgdsfd,
            /* [out] */ ULONG *pceltFetched) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IEnumDebugStackFrames64Vtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IEnumDebugStackFrames64 * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IEnumDebugStackFrames64 * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IEnumDebugStackFrames64 * This);
        
        DECLSPEC_XFGVIRT(IEnumDebugStackFrames, Next)
        /* [local] */ HRESULT ( __stdcall *Next )( 
            IEnumDebugStackFrames64 * This,
            /* [in] */ ULONG celt,
            /* [out] */ DebugStackFrameDescriptor *prgdsfd,
            /* [out] */ ULONG *pceltFetched);
        
        DECLSPEC_XFGVIRT(IEnumDebugStackFrames, Skip)
        HRESULT ( STDMETHODCALLTYPE *Skip )( 
            __RPC__in IEnumDebugStackFrames64 * This,
            /* [in] */ ULONG celt);
        
        DECLSPEC_XFGVIRT(IEnumDebugStackFrames, Reset)
        HRESULT ( STDMETHODCALLTYPE *Reset )( 
            __RPC__in IEnumDebugStackFrames64 * This);
        
        DECLSPEC_XFGVIRT(IEnumDebugStackFrames, Clone)
        HRESULT ( STDMETHODCALLTYPE *Clone )( 
            __RPC__in IEnumDebugStackFrames64 * This,
            /* [out] */ __RPC__deref_out_opt IEnumDebugStackFrames **ppedsf);
        
        DECLSPEC_XFGVIRT(IEnumDebugStackFrames64, Next64)
        /* [local] */ HRESULT ( __stdcall *Next64 )( 
            IEnumDebugStackFrames64 * This,
            /* [in] */ ULONG celt,
            /* [out] */ DebugStackFrameDescriptor64 *prgdsfd,
            /* [out] */ ULONG *pceltFetched);
        
        END_INTERFACE
    } IEnumDebugStackFrames64Vtbl;

    interface IEnumDebugStackFrames64
    {
        CONST_VTBL struct IEnumDebugStackFrames64Vtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IEnumDebugStackFrames64_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IEnumDebugStackFrames64_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IEnumDebugStackFrames64_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IEnumDebugStackFrames64_Next(This,celt,prgdsfd,pceltFetched)	\
    ( (This)->lpVtbl -> Next(This,celt,prgdsfd,pceltFetched) ) 

#define IEnumDebugStackFrames64_Skip(This,celt)	\
    ( (This)->lpVtbl -> Skip(This,celt) ) 

#define IEnumDebugStackFrames64_Reset(This)	\
    ( (This)->lpVtbl -> Reset(This) ) 

#define IEnumDebugStackFrames64_Clone(This,ppedsf)	\
    ( (This)->lpVtbl -> Clone(This,ppedsf) ) 


#define IEnumDebugStackFrames64_Next64(This,celt,prgdsfd,pceltFetched)	\
    ( (This)->lpVtbl -> Next64(This,celt,prgdsfd,pceltFetched) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */



/* [call_as] */ HRESULT __stdcall IEnumDebugStackFrames64_RemoteNext64_Proxy( 
    __RPC__in IEnumDebugStackFrames64 * This,
    /* [in] */ ULONG celt,
    /* [length_is][size_is][out] */ __RPC__out_ecount_part(celt, *pceltFetched) DebugStackFrameDescriptor64 *prgdsfd,
    /* [out] */ __RPC__out ULONG *pceltFetched);


void __RPC_STUB IEnumDebugStackFrames64_RemoteNext64_Stub(
    IRpcStubBuffer *This,
    IRpcChannelBuffer *_pRpcChannelBuffer,
    PRPC_MESSAGE _pRpcMessage,
    DWORD *_pdwStubPhase);



#endif 	/* __IEnumDebugStackFrames64_INTERFACE_DEFINED__ */


#ifndef __IDebugDocumentInfo_INTERFACE_DEFINED__
#define __IDebugDocumentInfo_INTERFACE_DEFINED__

/* interface IDebugDocumentInfo */
/* [unique][helpstring][uuid][object] */ 


EXTERN_C const IID IID_IDebugDocumentInfo;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("51973C1f-CB0C-11d0-B5C9-00A0244A0E7A")
    IDebugDocumentInfo : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetName( 
            /* [in] */ DOCUMENTNAMETYPE dnt,
            /* [out] */ __RPC__deref_out_opt BSTR *pbstrName) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetDocumentClassId( 
            /* [out] */ __RPC__out CLSID *pclsidDocument) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IDebugDocumentInfoVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IDebugDocumentInfo * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IDebugDocumentInfo * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IDebugDocumentInfo * This);
        
        DECLSPEC_XFGVIRT(IDebugDocumentInfo, GetName)
        HRESULT ( STDMETHODCALLTYPE *GetName )( 
            __RPC__in IDebugDocumentInfo * This,
            /* [in] */ DOCUMENTNAMETYPE dnt,
            /* [out] */ __RPC__deref_out_opt BSTR *pbstrName);
        
        DECLSPEC_XFGVIRT(IDebugDocumentInfo, GetDocumentClassId)
        HRESULT ( STDMETHODCALLTYPE *GetDocumentClassId )( 
            __RPC__in IDebugDocumentInfo * This,
            /* [out] */ __RPC__out CLSID *pclsidDocument);
        
        END_INTERFACE
    } IDebugDocumentInfoVtbl;

    interface IDebugDocumentInfo
    {
        CONST_VTBL struct IDebugDocumentInfoVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IDebugDocumentInfo_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IDebugDocumentInfo_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IDebugDocumentInfo_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IDebugDocumentInfo_GetName(This,dnt,pbstrName)	\
    ( (This)->lpVtbl -> GetName(This,dnt,pbstrName) ) 

#define IDebugDocumentInfo_GetDocumentClassId(This,pclsidDocument)	\
    ( (This)->lpVtbl -> GetDocumentClassId(This,pclsidDocument) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IDebugDocumentInfo_INTERFACE_DEFINED__ */


#ifndef __IDebugDocumentProvider_INTERFACE_DEFINED__
#define __IDebugDocumentProvider_INTERFACE_DEFINED__

/* interface IDebugDocumentProvider */
/* [unique][helpstring][uuid][object] */ 


EXTERN_C const IID IID_IDebugDocumentProvider;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("51973C20-CB0C-11d0-B5C9-00A0244A0E7A")
    IDebugDocumentProvider : public IDebugDocumentInfo
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetDocument( 
            /* [out] */ __RPC__deref_out_opt IDebugDocument **ppssd) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IDebugDocumentProviderVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IDebugDocumentProvider * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IDebugDocumentProvider * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IDebugDocumentProvider * This);
        
        DECLSPEC_XFGVIRT(IDebugDocumentInfo, GetName)
        HRESULT ( STDMETHODCALLTYPE *GetName )( 
            __RPC__in IDebugDocumentProvider * This,
            /* [in] */ DOCUMENTNAMETYPE dnt,
            /* [out] */ __RPC__deref_out_opt BSTR *pbstrName);
        
        DECLSPEC_XFGVIRT(IDebugDocumentInfo, GetDocumentClassId)
        HRESULT ( STDMETHODCALLTYPE *GetDocumentClassId )( 
            __RPC__in IDebugDocumentProvider * This,
            /* [out] */ __RPC__out CLSID *pclsidDocument);
        
        DECLSPEC_XFGVIRT(IDebugDocumentProvider, GetDocument)
        HRESULT ( STDMETHODCALLTYPE *GetDocument )( 
            __RPC__in IDebugDocumentProvider * This,
            /* [out] */ __RPC__deref_out_opt IDebugDocument **ppssd);
        
        END_INTERFACE
    } IDebugDocumentProviderVtbl;

    interface IDebugDocumentProvider
    {
        CONST_VTBL struct IDebugDocumentProviderVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IDebugDocumentProvider_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IDebugDocumentProvider_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IDebugDocumentProvider_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IDebugDocumentProvider_GetName(This,dnt,pbstrName)	\
    ( (This)->lpVtbl -> GetName(This,dnt,pbstrName) ) 

#define IDebugDocumentProvider_GetDocumentClassId(This,pclsidDocument)	\
    ( (This)->lpVtbl -> GetDocumentClassId(This,pclsidDocument) ) 


#define IDebugDocumentProvider_GetDocument(This,ppssd)	\
    ( (This)->lpVtbl -> GetDocument(This,ppssd) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IDebugDocumentProvider_INTERFACE_DEFINED__ */


#ifndef __IDebugDocument_INTERFACE_DEFINED__
#define __IDebugDocument_INTERFACE_DEFINED__

/* interface IDebugDocument */
/* [unique][uuid][object] */ 


EXTERN_C const IID IID_IDebugDocument;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("51973C21-CB0C-11d0-B5C9-00A0244A0E7A")
    IDebugDocument : public IDebugDocumentInfo
    {
    public:
    };
    
    
#else 	/* C style interface */

    typedef struct IDebugDocumentVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IDebugDocument * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IDebugDocument * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IDebugDocument * This);
        
        DECLSPEC_XFGVIRT(IDebugDocumentInfo, GetName)
        HRESULT ( STDMETHODCALLTYPE *GetName )( 
            __RPC__in IDebugDocument * This,
            /* [in] */ DOCUMENTNAMETYPE dnt,
            /* [out] */ __RPC__deref_out_opt BSTR *pbstrName);
        
        DECLSPEC_XFGVIRT(IDebugDocumentInfo, GetDocumentClassId)
        HRESULT ( STDMETHODCALLTYPE *GetDocumentClassId )( 
            __RPC__in IDebugDocument * This,
            /* [out] */ __RPC__out CLSID *pclsidDocument);
        
        END_INTERFACE
    } IDebugDocumentVtbl;

    interface IDebugDocument
    {
        CONST_VTBL struct IDebugDocumentVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IDebugDocument_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IDebugDocument_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IDebugDocument_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IDebugDocument_GetName(This,dnt,pbstrName)	\
    ( (This)->lpVtbl -> GetName(This,dnt,pbstrName) ) 

#define IDebugDocument_GetDocumentClassId(This,pclsidDocument)	\
    ( (This)->lpVtbl -> GetDocumentClassId(This,pclsidDocument) ) 


#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IDebugDocument_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_activdbg_0000_0023 */
/* [local] */ 

#pragma warning(push)
#pragma warning(disable:28718)	


extern RPC_IF_HANDLE __MIDL_itf_activdbg_0000_0023_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_activdbg_0000_0023_v0_0_s_ifspec;

#ifndef __IDebugDocumentText_INTERFACE_DEFINED__
#define __IDebugDocumentText_INTERFACE_DEFINED__

/* interface IDebugDocumentText */
/* [unique][uuid][object] */ 


EXTERN_C const IID IID_IDebugDocumentText;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("51973C22-CB0C-11d0-B5C9-00A0244A0E7A")
    IDebugDocumentText : public IDebugDocument
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetDocumentAttributes( 
            /* [out] */ __RPC__out TEXT_DOC_ATTR *ptextdocattr) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetSize( 
            /* [out] */ __RPC__out ULONG *pcNumLines,
            /* [out] */ __RPC__out ULONG *pcNumChars) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetPositionOfLine( 
            /* [in] */ ULONG cLineNumber,
            /* [out] */ __RPC__out ULONG *pcCharacterPosition) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetLineOfPosition( 
            /* [in] */ ULONG cCharacterPosition,
            /* [out] */ __RPC__out ULONG *pcLineNumber,
            /* [out] */ __RPC__out ULONG *pcCharacterOffsetInLine) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetText( 
            /* [in] */ ULONG cCharacterPosition,
            /* [size_is][length_is][out][in] */ __RPC__inout_ecount_part(cMaxChars, *pcNumChars) WCHAR *pcharText,
            /* [full][size_is][length_is][out][in] */ __RPC__inout_ecount_part_opt(cMaxChars, *pcNumChars) SOURCE_TEXT_ATTR *pstaTextAttr,
            /* [out][in] */ __RPC__inout ULONG *pcNumChars,
            /* [in] */ ULONG cMaxChars) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetPositionOfContext( 
            /* [in] */ __RPC__in_opt IDebugDocumentContext *psc,
            /* [out] */ __RPC__out ULONG *pcCharacterPosition,
            /* [out] */ __RPC__out ULONG *cNumChars) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetContextOfPosition( 
            /* [in] */ ULONG cCharacterPosition,
            /* [in] */ ULONG cNumChars,
            /* [out] */ __RPC__deref_out_opt IDebugDocumentContext **ppsc) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IDebugDocumentTextVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IDebugDocumentText * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IDebugDocumentText * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IDebugDocumentText * This);
        
        DECLSPEC_XFGVIRT(IDebugDocumentInfo, GetName)
        HRESULT ( STDMETHODCALLTYPE *GetName )( 
            __RPC__in IDebugDocumentText * This,
            /* [in] */ DOCUMENTNAMETYPE dnt,
            /* [out] */ __RPC__deref_out_opt BSTR *pbstrName);
        
        DECLSPEC_XFGVIRT(IDebugDocumentInfo, GetDocumentClassId)
        HRESULT ( STDMETHODCALLTYPE *GetDocumentClassId )( 
            __RPC__in IDebugDocumentText * This,
            /* [out] */ __RPC__out CLSID *pclsidDocument);
        
        DECLSPEC_XFGVIRT(IDebugDocumentText, GetDocumentAttributes)
        HRESULT ( STDMETHODCALLTYPE *GetDocumentAttributes )( 
            __RPC__in IDebugDocumentText * This,
            /* [out] */ __RPC__out TEXT_DOC_ATTR *ptextdocattr);
        
        DECLSPEC_XFGVIRT(IDebugDocumentText, GetSize)
        HRESULT ( STDMETHODCALLTYPE *GetSize )( 
            __RPC__in IDebugDocumentText * This,
            /* [out] */ __RPC__out ULONG *pcNumLines,
            /* [out] */ __RPC__out ULONG *pcNumChars);
        
        DECLSPEC_XFGVIRT(IDebugDocumentText, GetPositionOfLine)
        HRESULT ( STDMETHODCALLTYPE *GetPositionOfLine )( 
            __RPC__in IDebugDocumentText * This,
            /* [in] */ ULONG cLineNumber,
            /* [out] */ __RPC__out ULONG *pcCharacterPosition);
        
        DECLSPEC_XFGVIRT(IDebugDocumentText, GetLineOfPosition)
        HRESULT ( STDMETHODCALLTYPE *GetLineOfPosition )( 
            __RPC__in IDebugDocumentText * This,
            /* [in] */ ULONG cCharacterPosition,
            /* [out] */ __RPC__out ULONG *pcLineNumber,
            /* [out] */ __RPC__out ULONG *pcCharacterOffsetInLine);
        
        DECLSPEC_XFGVIRT(IDebugDocumentText, GetText)
        HRESULT ( STDMETHODCALLTYPE *GetText )( 
            __RPC__in IDebugDocumentText * This,
            /* [in] */ ULONG cCharacterPosition,
            /* [size_is][length_is][out][in] */ __RPC__inout_ecount_part(cMaxChars, *pcNumChars) WCHAR *pcharText,
            /* [full][size_is][length_is][out][in] */ __RPC__inout_ecount_part_opt(cMaxChars, *pcNumChars) SOURCE_TEXT_ATTR *pstaTextAttr,
            /* [out][in] */ __RPC__inout ULONG *pcNumChars,
            /* [in] */ ULONG cMaxChars);
        
        DECLSPEC_XFGVIRT(IDebugDocumentText, GetPositionOfContext)
        HRESULT ( STDMETHODCALLTYPE *GetPositionOfContext )( 
            __RPC__in IDebugDocumentText * This,
            /* [in] */ __RPC__in_opt IDebugDocumentContext *psc,
            /* [out] */ __RPC__out ULONG *pcCharacterPosition,
            /* [out] */ __RPC__out ULONG *cNumChars);
        
        DECLSPEC_XFGVIRT(IDebugDocumentText, GetContextOfPosition)
        HRESULT ( STDMETHODCALLTYPE *GetContextOfPosition )( 
            __RPC__in IDebugDocumentText * This,
            /* [in] */ ULONG cCharacterPosition,
            /* [in] */ ULONG cNumChars,
            /* [out] */ __RPC__deref_out_opt IDebugDocumentContext **ppsc);
        
        END_INTERFACE
    } IDebugDocumentTextVtbl;

    interface IDebugDocumentText
    {
        CONST_VTBL struct IDebugDocumentTextVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IDebugDocumentText_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IDebugDocumentText_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IDebugDocumentText_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IDebugDocumentText_GetName(This,dnt,pbstrName)	\
    ( (This)->lpVtbl -> GetName(This,dnt,pbstrName) ) 

#define IDebugDocumentText_GetDocumentClassId(This,pclsidDocument)	\
    ( (This)->lpVtbl -> GetDocumentClassId(This,pclsidDocument) ) 



#define IDebugDocumentText_GetDocumentAttributes(This,ptextdocattr)	\
    ( (This)->lpVtbl -> GetDocumentAttributes(This,ptextdocattr) ) 

#define IDebugDocumentText_GetSize(This,pcNumLines,pcNumChars)	\
    ( (This)->lpVtbl -> GetSize(This,pcNumLines,pcNumChars) ) 

#define IDebugDocumentText_GetPositionOfLine(This,cLineNumber,pcCharacterPosition)	\
    ( (This)->lpVtbl -> GetPositionOfLine(This,cLineNumber,pcCharacterPosition) ) 

#define IDebugDocumentText_GetLineOfPosition(This,cCharacterPosition,pcLineNumber,pcCharacterOffsetInLine)	\
    ( (This)->lpVtbl -> GetLineOfPosition(This,cCharacterPosition,pcLineNumber,pcCharacterOffsetInLine) ) 

#define IDebugDocumentText_GetText(This,cCharacterPosition,pcharText,pstaTextAttr,pcNumChars,cMaxChars)	\
    ( (This)->lpVtbl -> GetText(This,cCharacterPosition,pcharText,pstaTextAttr,pcNumChars,cMaxChars) ) 

#define IDebugDocumentText_GetPositionOfContext(This,psc,pcCharacterPosition,cNumChars)	\
    ( (This)->lpVtbl -> GetPositionOfContext(This,psc,pcCharacterPosition,cNumChars) ) 

#define IDebugDocumentText_GetContextOfPosition(This,cCharacterPosition,cNumChars,ppsc)	\
    ( (This)->lpVtbl -> GetContextOfPosition(This,cCharacterPosition,cNumChars,ppsc) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IDebugDocumentText_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_activdbg_0000_0024 */
/* [local] */ 

#pragma warning(pop)


extern RPC_IF_HANDLE __MIDL_itf_activdbg_0000_0024_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_activdbg_0000_0024_v0_0_s_ifspec;

#ifndef __IDebugDocumentTextEvents_INTERFACE_DEFINED__
#define __IDebugDocumentTextEvents_INTERFACE_DEFINED__

/* interface IDebugDocumentTextEvents */
/* [unique][uuid][object] */ 


EXTERN_C const IID IID_IDebugDocumentTextEvents;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("51973C23-CB0C-11d0-B5C9-00A0244A0E7A")
    IDebugDocumentTextEvents : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE onDestroy( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE onInsertText( 
            /* [in] */ ULONG cCharacterPosition,
            /* [in] */ ULONG cNumToInsert) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE onRemoveText( 
            /* [in] */ ULONG cCharacterPosition,
            /* [in] */ ULONG cNumToRemove) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE onReplaceText( 
            /* [in] */ ULONG cCharacterPosition,
            /* [in] */ ULONG cNumToReplace) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE onUpdateTextAttributes( 
            /* [in] */ ULONG cCharacterPosition,
            /* [in] */ ULONG cNumToUpdate) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE onUpdateDocumentAttributes( 
            /* [in] */ TEXT_DOC_ATTR textdocattr) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IDebugDocumentTextEventsVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IDebugDocumentTextEvents * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IDebugDocumentTextEvents * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IDebugDocumentTextEvents * This);
        
        DECLSPEC_XFGVIRT(IDebugDocumentTextEvents, onDestroy)
        HRESULT ( STDMETHODCALLTYPE *onDestroy )( 
            __RPC__in IDebugDocumentTextEvents * This);
        
        DECLSPEC_XFGVIRT(IDebugDocumentTextEvents, onInsertText)
        HRESULT ( STDMETHODCALLTYPE *onInsertText )( 
            __RPC__in IDebugDocumentTextEvents * This,
            /* [in] */ ULONG cCharacterPosition,
            /* [in] */ ULONG cNumToInsert);
        
        DECLSPEC_XFGVIRT(IDebugDocumentTextEvents, onRemoveText)
        HRESULT ( STDMETHODCALLTYPE *onRemoveText )( 
            __RPC__in IDebugDocumentTextEvents * This,
            /* [in] */ ULONG cCharacterPosition,
            /* [in] */ ULONG cNumToRemove);
        
        DECLSPEC_XFGVIRT(IDebugDocumentTextEvents, onReplaceText)
        HRESULT ( STDMETHODCALLTYPE *onReplaceText )( 
            __RPC__in IDebugDocumentTextEvents * This,
            /* [in] */ ULONG cCharacterPosition,
            /* [in] */ ULONG cNumToReplace);
        
        DECLSPEC_XFGVIRT(IDebugDocumentTextEvents, onUpdateTextAttributes)
        HRESULT ( STDMETHODCALLTYPE *onUpdateTextAttributes )( 
            __RPC__in IDebugDocumentTextEvents * This,
            /* [in] */ ULONG cCharacterPosition,
            /* [in] */ ULONG cNumToUpdate);
        
        DECLSPEC_XFGVIRT(IDebugDocumentTextEvents, onUpdateDocumentAttributes)
        HRESULT ( STDMETHODCALLTYPE *onUpdateDocumentAttributes )( 
            __RPC__in IDebugDocumentTextEvents * This,
            /* [in] */ TEXT_DOC_ATTR textdocattr);
        
        END_INTERFACE
    } IDebugDocumentTextEventsVtbl;

    interface IDebugDocumentTextEvents
    {
        CONST_VTBL struct IDebugDocumentTextEventsVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IDebugDocumentTextEvents_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IDebugDocumentTextEvents_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IDebugDocumentTextEvents_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IDebugDocumentTextEvents_onDestroy(This)	\
    ( (This)->lpVtbl -> onDestroy(This) ) 

#define IDebugDocumentTextEvents_onInsertText(This,cCharacterPosition,cNumToInsert)	\
    ( (This)->lpVtbl -> onInsertText(This,cCharacterPosition,cNumToInsert) ) 

#define IDebugDocumentTextEvents_onRemoveText(This,cCharacterPosition,cNumToRemove)	\
    ( (This)->lpVtbl -> onRemoveText(This,cCharacterPosition,cNumToRemove) ) 

#define IDebugDocumentTextEvents_onReplaceText(This,cCharacterPosition,cNumToReplace)	\
    ( (This)->lpVtbl -> onReplaceText(This,cCharacterPosition,cNumToReplace) ) 

#define IDebugDocumentTextEvents_onUpdateTextAttributes(This,cCharacterPosition,cNumToUpdate)	\
    ( (This)->lpVtbl -> onUpdateTextAttributes(This,cCharacterPosition,cNumToUpdate) ) 

#define IDebugDocumentTextEvents_onUpdateDocumentAttributes(This,textdocattr)	\
    ( (This)->lpVtbl -> onUpdateDocumentAttributes(This,textdocattr) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IDebugDocumentTextEvents_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_activdbg_0000_0025 */
/* [local] */ 

#pragma warning(push)
#pragma warning(disable:28718)	


extern RPC_IF_HANDLE __MIDL_itf_activdbg_0000_0025_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_activdbg_0000_0025_v0_0_s_ifspec;

#ifndef __IDebugDocumentTextAuthor_INTERFACE_DEFINED__
#define __IDebugDocumentTextAuthor_INTERFACE_DEFINED__

/* interface IDebugDocumentTextAuthor */
/* [unique][uuid][object] */ 


EXTERN_C const IID IID_IDebugDocumentTextAuthor;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("51973C24-CB0C-11d0-B5C9-00A0244A0E7A")
    IDebugDocumentTextAuthor : public IDebugDocumentText
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE InsertText( 
            /* [in] */ ULONG cCharacterPosition,
            /* [in] */ ULONG cNumToInsert,
            /* [size_is][in] */ __RPC__in_ecount_full(cNumToInsert) OLECHAR pcharText[  ]) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE RemoveText( 
            /* [in] */ ULONG cCharacterPosition,
            /* [in] */ ULONG cNumToRemove) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE ReplaceText( 
            /* [in] */ ULONG cCharacterPosition,
            /* [in] */ ULONG cNumToReplace,
            /* [size_is][in] */ __RPC__in_ecount_full(cNumToReplace) OLECHAR pcharText[  ]) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IDebugDocumentTextAuthorVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IDebugDocumentTextAuthor * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IDebugDocumentTextAuthor * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IDebugDocumentTextAuthor * This);
        
        DECLSPEC_XFGVIRT(IDebugDocumentInfo, GetName)
        HRESULT ( STDMETHODCALLTYPE *GetName )( 
            __RPC__in IDebugDocumentTextAuthor * This,
            /* [in] */ DOCUMENTNAMETYPE dnt,
            /* [out] */ __RPC__deref_out_opt BSTR *pbstrName);
        
        DECLSPEC_XFGVIRT(IDebugDocumentInfo, GetDocumentClassId)
        HRESULT ( STDMETHODCALLTYPE *GetDocumentClassId )( 
            __RPC__in IDebugDocumentTextAuthor * This,
            /* [out] */ __RPC__out CLSID *pclsidDocument);
        
        DECLSPEC_XFGVIRT(IDebugDocumentText, GetDocumentAttributes)
        HRESULT ( STDMETHODCALLTYPE *GetDocumentAttributes )( 
            __RPC__in IDebugDocumentTextAuthor * This,
            /* [out] */ __RPC__out TEXT_DOC_ATTR *ptextdocattr);
        
        DECLSPEC_XFGVIRT(IDebugDocumentText, GetSize)
        HRESULT ( STDMETHODCALLTYPE *GetSize )( 
            __RPC__in IDebugDocumentTextAuthor * This,
            /* [out] */ __RPC__out ULONG *pcNumLines,
            /* [out] */ __RPC__out ULONG *pcNumChars);
        
        DECLSPEC_XFGVIRT(IDebugDocumentText, GetPositionOfLine)
        HRESULT ( STDMETHODCALLTYPE *GetPositionOfLine )( 
            __RPC__in IDebugDocumentTextAuthor * This,
            /* [in] */ ULONG cLineNumber,
            /* [out] */ __RPC__out ULONG *pcCharacterPosition);
        
        DECLSPEC_XFGVIRT(IDebugDocumentText, GetLineOfPosition)
        HRESULT ( STDMETHODCALLTYPE *GetLineOfPosition )( 
            __RPC__in IDebugDocumentTextAuthor * This,
            /* [in] */ ULONG cCharacterPosition,
            /* [out] */ __RPC__out ULONG *pcLineNumber,
            /* [out] */ __RPC__out ULONG *pcCharacterOffsetInLine);
        
        DECLSPEC_XFGVIRT(IDebugDocumentText, GetText)
        HRESULT ( STDMETHODCALLTYPE *GetText )( 
            __RPC__in IDebugDocumentTextAuthor * This,
            /* [in] */ ULONG cCharacterPosition,
            /* [size_is][length_is][out][in] */ __RPC__inout_ecount_part(cMaxChars, *pcNumChars) WCHAR *pcharText,
            /* [full][size_is][length_is][out][in] */ __RPC__inout_ecount_part_opt(cMaxChars, *pcNumChars) SOURCE_TEXT_ATTR *pstaTextAttr,
            /* [out][in] */ __RPC__inout ULONG *pcNumChars,
            /* [in] */ ULONG cMaxChars);
        
        DECLSPEC_XFGVIRT(IDebugDocumentText, GetPositionOfContext)
        HRESULT ( STDMETHODCALLTYPE *GetPositionOfContext )( 
            __RPC__in IDebugDocumentTextAuthor * This,
            /* [in] */ __RPC__in_opt IDebugDocumentContext *psc,
            /* [out] */ __RPC__out ULONG *pcCharacterPosition,
            /* [out] */ __RPC__out ULONG *cNumChars);
        
        DECLSPEC_XFGVIRT(IDebugDocumentText, GetContextOfPosition)
        HRESULT ( STDMETHODCALLTYPE *GetContextOfPosition )( 
            __RPC__in IDebugDocumentTextAuthor * This,
            /* [in] */ ULONG cCharacterPosition,
            /* [in] */ ULONG cNumChars,
            /* [out] */ __RPC__deref_out_opt IDebugDocumentContext **ppsc);
        
        DECLSPEC_XFGVIRT(IDebugDocumentTextAuthor, InsertText)
        HRESULT ( STDMETHODCALLTYPE *InsertText )( 
            __RPC__in IDebugDocumentTextAuthor * This,
            /* [in] */ ULONG cCharacterPosition,
            /* [in] */ ULONG cNumToInsert,
            /* [size_is][in] */ __RPC__in_ecount_full(cNumToInsert) OLECHAR pcharText[  ]);
        
        DECLSPEC_XFGVIRT(IDebugDocumentTextAuthor, RemoveText)
        HRESULT ( STDMETHODCALLTYPE *RemoveText )( 
            __RPC__in IDebugDocumentTextAuthor * This,
            /* [in] */ ULONG cCharacterPosition,
            /* [in] */ ULONG cNumToRemove);
        
        DECLSPEC_XFGVIRT(IDebugDocumentTextAuthor, ReplaceText)
        HRESULT ( STDMETHODCALLTYPE *ReplaceText )( 
            __RPC__in IDebugDocumentTextAuthor * This,
            /* [in] */ ULONG cCharacterPosition,
            /* [in] */ ULONG cNumToReplace,
            /* [size_is][in] */ __RPC__in_ecount_full(cNumToReplace) OLECHAR pcharText[  ]);
        
        END_INTERFACE
    } IDebugDocumentTextAuthorVtbl;

    interface IDebugDocumentTextAuthor
    {
        CONST_VTBL struct IDebugDocumentTextAuthorVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IDebugDocumentTextAuthor_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IDebugDocumentTextAuthor_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IDebugDocumentTextAuthor_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IDebugDocumentTextAuthor_GetName(This,dnt,pbstrName)	\
    ( (This)->lpVtbl -> GetName(This,dnt,pbstrName) ) 

#define IDebugDocumentTextAuthor_GetDocumentClassId(This,pclsidDocument)	\
    ( (This)->lpVtbl -> GetDocumentClassId(This,pclsidDocument) ) 



#define IDebugDocumentTextAuthor_GetDocumentAttributes(This,ptextdocattr)	\
    ( (This)->lpVtbl -> GetDocumentAttributes(This,ptextdocattr) ) 

#define IDebugDocumentTextAuthor_GetSize(This,pcNumLines,pcNumChars)	\
    ( (This)->lpVtbl -> GetSize(This,pcNumLines,pcNumChars) ) 

#define IDebugDocumentTextAuthor_GetPositionOfLine(This,cLineNumber,pcCharacterPosition)	\
    ( (This)->lpVtbl -> GetPositionOfLine(This,cLineNumber,pcCharacterPosition) ) 

#define IDebugDocumentTextAuthor_GetLineOfPosition(This,cCharacterPosition,pcLineNumber,pcCharacterOffsetInLine)	\
    ( (This)->lpVtbl -> GetLineOfPosition(This,cCharacterPosition,pcLineNumber,pcCharacterOffsetInLine) ) 

#define IDebugDocumentTextAuthor_GetText(This,cCharacterPosition,pcharText,pstaTextAttr,pcNumChars,cMaxChars)	\
    ( (This)->lpVtbl -> GetText(This,cCharacterPosition,pcharText,pstaTextAttr,pcNumChars,cMaxChars) ) 

#define IDebugDocumentTextAuthor_GetPositionOfContext(This,psc,pcCharacterPosition,cNumChars)	\
    ( (This)->lpVtbl -> GetPositionOfContext(This,psc,pcCharacterPosition,cNumChars) ) 

#define IDebugDocumentTextAuthor_GetContextOfPosition(This,cCharacterPosition,cNumChars,ppsc)	\
    ( (This)->lpVtbl -> GetContextOfPosition(This,cCharacterPosition,cNumChars,ppsc) ) 


#define IDebugDocumentTextAuthor_InsertText(This,cCharacterPosition,cNumToInsert,pcharText)	\
    ( (This)->lpVtbl -> InsertText(This,cCharacterPosition,cNumToInsert,pcharText) ) 

#define IDebugDocumentTextAuthor_RemoveText(This,cCharacterPosition,cNumToRemove)	\
    ( (This)->lpVtbl -> RemoveText(This,cCharacterPosition,cNumToRemove) ) 

#define IDebugDocumentTextAuthor_ReplaceText(This,cCharacterPosition,cNumToReplace,pcharText)	\
    ( (This)->lpVtbl -> ReplaceText(This,cCharacterPosition,cNumToReplace,pcharText) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IDebugDocumentTextAuthor_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_activdbg_0000_0026 */
/* [local] */ 

#pragma warning(pop)


extern RPC_IF_HANDLE __MIDL_itf_activdbg_0000_0026_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_activdbg_0000_0026_v0_0_s_ifspec;

#ifndef __IDebugDocumentTextExternalAuthor_INTERFACE_DEFINED__
#define __IDebugDocumentTextExternalAuthor_INTERFACE_DEFINED__

/* interface IDebugDocumentTextExternalAuthor */
/* [unique][uuid][object] */ 


EXTERN_C const IID IID_IDebugDocumentTextExternalAuthor;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("51973C25-CB0C-11d0-B5C9-00A0244A0E7A")
    IDebugDocumentTextExternalAuthor : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetPathName( 
            /* [out] */ __RPC__deref_out_opt BSTR *pbstrLongName,
            /* [out] */ __RPC__out BOOL *pfIsOriginalFile) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetFileName( 
            /* [out] */ __RPC__deref_out_opt BSTR *pbstrShortName) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE NotifyChanged( void) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IDebugDocumentTextExternalAuthorVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IDebugDocumentTextExternalAuthor * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IDebugDocumentTextExternalAuthor * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IDebugDocumentTextExternalAuthor * This);
        
        DECLSPEC_XFGVIRT(IDebugDocumentTextExternalAuthor, GetPathName)
        HRESULT ( STDMETHODCALLTYPE *GetPathName )( 
            __RPC__in IDebugDocumentTextExternalAuthor * This,
            /* [out] */ __RPC__deref_out_opt BSTR *pbstrLongName,
            /* [out] */ __RPC__out BOOL *pfIsOriginalFile);
        
        DECLSPEC_XFGVIRT(IDebugDocumentTextExternalAuthor, GetFileName)
        HRESULT ( STDMETHODCALLTYPE *GetFileName )( 
            __RPC__in IDebugDocumentTextExternalAuthor * This,
            /* [out] */ __RPC__deref_out_opt BSTR *pbstrShortName);
        
        DECLSPEC_XFGVIRT(IDebugDocumentTextExternalAuthor, NotifyChanged)
        HRESULT ( STDMETHODCALLTYPE *NotifyChanged )( 
            __RPC__in IDebugDocumentTextExternalAuthor * This);
        
        END_INTERFACE
    } IDebugDocumentTextExternalAuthorVtbl;

    interface IDebugDocumentTextExternalAuthor
    {
        CONST_VTBL struct IDebugDocumentTextExternalAuthorVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IDebugDocumentTextExternalAuthor_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IDebugDocumentTextExternalAuthor_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IDebugDocumentTextExternalAuthor_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IDebugDocumentTextExternalAuthor_GetPathName(This,pbstrLongName,pfIsOriginalFile)	\
    ( (This)->lpVtbl -> GetPathName(This,pbstrLongName,pfIsOriginalFile) ) 

#define IDebugDocumentTextExternalAuthor_GetFileName(This,pbstrShortName)	\
    ( (This)->lpVtbl -> GetFileName(This,pbstrShortName) ) 

#define IDebugDocumentTextExternalAuthor_NotifyChanged(This)	\
    ( (This)->lpVtbl -> NotifyChanged(This) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IDebugDocumentTextExternalAuthor_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_activdbg_0000_0027 */
/* [local] */ 

#ifndef DISABLE_ACTIVDBG_INTERFACE_WRAPPERS
#ifdef _WIN64
#define IDebugDocumentHelper IDebugDocumentHelper64
#define IID_IDebugDocumentHelper IID_IDebugDocumentHelper64
#else
#define IDebugDocumentHelper IDebugDocumentHelper32
#define IID_IDebugDocumentHelper IID_IDebugDocumentHelper32
#endif
#endif


extern RPC_IF_HANDLE __MIDL_itf_activdbg_0000_0027_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_activdbg_0000_0027_v0_0_s_ifspec;

#ifndef __IDebugDocumentHelper32_INTERFACE_DEFINED__
#define __IDebugDocumentHelper32_INTERFACE_DEFINED__

/* interface IDebugDocumentHelper32 */
/* [unique][helpstring][uuid][object] */ 


EXTERN_C const IID IID_IDebugDocumentHelper32;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("51973C26-CB0C-11d0-B5C9-00A0244A0E7A")
    IDebugDocumentHelper32 : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE Init( 
            /* [in] */ __RPC__in_opt IDebugApplication32 *pda,
            /* [string][in] */ __RPC__in_string LPCOLESTR pszShortName,
            /* [string][in] */ __RPC__in_string LPCOLESTR pszLongName,
            /* [in] */ TEXT_DOC_ATTR docAttr) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Attach( 
            /* [in] */ __RPC__in_opt IDebugDocumentHelper32 *pddhParent) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Detach( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE AddUnicodeText( 
            /* [string][in] */ __RPC__in_string LPCOLESTR pszText) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE AddDBCSText( 
            /* [string][in] */ __RPC__in_string LPCSTR pszText) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetDebugDocumentHost( 
            /* [in] */ __RPC__in_opt IDebugDocumentHost *pddh) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE AddDeferredText( 
            /* [in] */ ULONG cChars,
            /* [in] */ DWORD dwTextStartCookie) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE DefineScriptBlock( 
            /* [in] */ ULONG ulCharOffset,
            /* [in] */ ULONG cChars,
            /* [in] */ __RPC__in_opt IActiveScript *pas,
            /* [in] */ BOOL fScriptlet,
            /* [out] */ __RPC__out DWORD *pdwSourceContext) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetDefaultTextAttr( 
            SOURCE_TEXT_ATTR staTextAttr) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetTextAttributes( 
            /* [in] */ ULONG ulCharOffset,
            /* [in] */ ULONG cChars,
            /* [size_is][length_is][in] */ __RPC__in_ecount_part(cChars, cChars) SOURCE_TEXT_ATTR *pstaTextAttr) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetLongName( 
            /* [string][in] */ __RPC__in_string LPCOLESTR pszLongName) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetShortName( 
            /* [string][in] */ __RPC__in_string LPCOLESTR pszShortName) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetDocumentAttr( 
            /* [in] */ TEXT_DOC_ATTR pszAttributes) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetDebugApplicationNode( 
            /* [out] */ __RPC__deref_out_opt IDebugApplicationNode **ppdan) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetScriptBlockInfo( 
            /* [in] */ DWORD dwSourceContext,
            /* [out] */ __RPC__deref_out_opt IActiveScript **ppasd,
            /* [out] */ __RPC__out ULONG *piCharPos,
            /* [out] */ __RPC__out ULONG *pcChars) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE CreateDebugDocumentContext( 
            /* [in] */ ULONG iCharPos,
            /* [in] */ ULONG cChars,
            /* [out] */ __RPC__deref_out_opt IDebugDocumentContext **ppddc) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE BringDocumentToTop( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE BringDocumentContextToTop( 
            __RPC__in_opt IDebugDocumentContext *pddc) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IDebugDocumentHelper32Vtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IDebugDocumentHelper32 * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IDebugDocumentHelper32 * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IDebugDocumentHelper32 * This);
        
        DECLSPEC_XFGVIRT(IDebugDocumentHelper32, Init)
        HRESULT ( STDMETHODCALLTYPE *Init )( 
            __RPC__in IDebugDocumentHelper32 * This,
            /* [in] */ __RPC__in_opt IDebugApplication32 *pda,
            /* [string][in] */ __RPC__in_string LPCOLESTR pszShortName,
            /* [string][in] */ __RPC__in_string LPCOLESTR pszLongName,
            /* [in] */ TEXT_DOC_ATTR docAttr);
        
        DECLSPEC_XFGVIRT(IDebugDocumentHelper32, Attach)
        HRESULT ( STDMETHODCALLTYPE *Attach )( 
            __RPC__in IDebugDocumentHelper32 * This,
            /* [in] */ __RPC__in_opt IDebugDocumentHelper32 *pddhParent);
        
        DECLSPEC_XFGVIRT(IDebugDocumentHelper32, Detach)
        HRESULT ( STDMETHODCALLTYPE *Detach )( 
            __RPC__in IDebugDocumentHelper32 * This);
        
        DECLSPEC_XFGVIRT(IDebugDocumentHelper32, AddUnicodeText)
        HRESULT ( STDMETHODCALLTYPE *AddUnicodeText )( 
            __RPC__in IDebugDocumentHelper32 * This,
            /* [string][in] */ __RPC__in_string LPCOLESTR pszText);
        
        DECLSPEC_XFGVIRT(IDebugDocumentHelper32, AddDBCSText)
        HRESULT ( STDMETHODCALLTYPE *AddDBCSText )( 
            __RPC__in IDebugDocumentHelper32 * This,
            /* [string][in] */ __RPC__in_string LPCSTR pszText);
        
        DECLSPEC_XFGVIRT(IDebugDocumentHelper32, SetDebugDocumentHost)
        HRESULT ( STDMETHODCALLTYPE *SetDebugDocumentHost )( 
            __RPC__in IDebugDocumentHelper32 * This,
            /* [in] */ __RPC__in_opt IDebugDocumentHost *pddh);
        
        DECLSPEC_XFGVIRT(IDebugDocumentHelper32, AddDeferredText)
        HRESULT ( STDMETHODCALLTYPE *AddDeferredText )( 
            __RPC__in IDebugDocumentHelper32 * This,
            /* [in] */ ULONG cChars,
            /* [in] */ DWORD dwTextStartCookie);
        
        DECLSPEC_XFGVIRT(IDebugDocumentHelper32, DefineScriptBlock)
        HRESULT ( STDMETHODCALLTYPE *DefineScriptBlock )( 
            __RPC__in IDebugDocumentHelper32 * This,
            /* [in] */ ULONG ulCharOffset,
            /* [in] */ ULONG cChars,
            /* [in] */ __RPC__in_opt IActiveScript *pas,
            /* [in] */ BOOL fScriptlet,
            /* [out] */ __RPC__out DWORD *pdwSourceContext);
        
        DECLSPEC_XFGVIRT(IDebugDocumentHelper32, SetDefaultTextAttr)
        HRESULT ( STDMETHODCALLTYPE *SetDefaultTextAttr )( 
            __RPC__in IDebugDocumentHelper32 * This,
            SOURCE_TEXT_ATTR staTextAttr);
        
        DECLSPEC_XFGVIRT(IDebugDocumentHelper32, SetTextAttributes)
        HRESULT ( STDMETHODCALLTYPE *SetTextAttributes )( 
            __RPC__in IDebugDocumentHelper32 * This,
            /* [in] */ ULONG ulCharOffset,
            /* [in] */ ULONG cChars,
            /* [size_is][length_is][in] */ __RPC__in_ecount_part(cChars, cChars) SOURCE_TEXT_ATTR *pstaTextAttr);
        
        DECLSPEC_XFGVIRT(IDebugDocumentHelper32, SetLongName)
        HRESULT ( STDMETHODCALLTYPE *SetLongName )( 
            __RPC__in IDebugDocumentHelper32 * This,
            /* [string][in] */ __RPC__in_string LPCOLESTR pszLongName);
        
        DECLSPEC_XFGVIRT(IDebugDocumentHelper32, SetShortName)
        HRESULT ( STDMETHODCALLTYPE *SetShortName )( 
            __RPC__in IDebugDocumentHelper32 * This,
            /* [string][in] */ __RPC__in_string LPCOLESTR pszShortName);
        
        DECLSPEC_XFGVIRT(IDebugDocumentHelper32, SetDocumentAttr)
        HRESULT ( STDMETHODCALLTYPE *SetDocumentAttr )( 
            __RPC__in IDebugDocumentHelper32 * This,
            /* [in] */ TEXT_DOC_ATTR pszAttributes);
        
        DECLSPEC_XFGVIRT(IDebugDocumentHelper32, GetDebugApplicationNode)
        HRESULT ( STDMETHODCALLTYPE *GetDebugApplicationNode )( 
            __RPC__in IDebugDocumentHelper32 * This,
            /* [out] */ __RPC__deref_out_opt IDebugApplicationNode **ppdan);
        
        DECLSPEC_XFGVIRT(IDebugDocumentHelper32, GetScriptBlockInfo)
        HRESULT ( STDMETHODCALLTYPE *GetScriptBlockInfo )( 
            __RPC__in IDebugDocumentHelper32 * This,
            /* [in] */ DWORD dwSourceContext,
            /* [out] */ __RPC__deref_out_opt IActiveScript **ppasd,
            /* [out] */ __RPC__out ULONG *piCharPos,
            /* [out] */ __RPC__out ULONG *pcChars);
        
        DECLSPEC_XFGVIRT(IDebugDocumentHelper32, CreateDebugDocumentContext)
        HRESULT ( STDMETHODCALLTYPE *CreateDebugDocumentContext )( 
            __RPC__in IDebugDocumentHelper32 * This,
            /* [in] */ ULONG iCharPos,
            /* [in] */ ULONG cChars,
            /* [out] */ __RPC__deref_out_opt IDebugDocumentContext **ppddc);
        
        DECLSPEC_XFGVIRT(IDebugDocumentHelper32, BringDocumentToTop)
        HRESULT ( STDMETHODCALLTYPE *BringDocumentToTop )( 
            __RPC__in IDebugDocumentHelper32 * This);
        
        DECLSPEC_XFGVIRT(IDebugDocumentHelper32, BringDocumentContextToTop)
        HRESULT ( STDMETHODCALLTYPE *BringDocumentContextToTop )( 
            __RPC__in IDebugDocumentHelper32 * This,
            __RPC__in_opt IDebugDocumentContext *pddc);
        
        END_INTERFACE
    } IDebugDocumentHelper32Vtbl;

    interface IDebugDocumentHelper32
    {
        CONST_VTBL struct IDebugDocumentHelper32Vtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IDebugDocumentHelper32_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IDebugDocumentHelper32_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IDebugDocumentHelper32_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IDebugDocumentHelper32_Init(This,pda,pszShortName,pszLongName,docAttr)	\
    ( (This)->lpVtbl -> Init(This,pda,pszShortName,pszLongName,docAttr) ) 

#define IDebugDocumentHelper32_Attach(This,pddhParent)	\
    ( (This)->lpVtbl -> Attach(This,pddhParent) ) 

#define IDebugDocumentHelper32_Detach(This)	\
    ( (This)->lpVtbl -> Detach(This) ) 

#define IDebugDocumentHelper32_AddUnicodeText(This,pszText)	\
    ( (This)->lpVtbl -> AddUnicodeText(This,pszText) ) 

#define IDebugDocumentHelper32_AddDBCSText(This,pszText)	\
    ( (This)->lpVtbl -> AddDBCSText(This,pszText) ) 

#define IDebugDocumentHelper32_SetDebugDocumentHost(This,pddh)	\
    ( (This)->lpVtbl -> SetDebugDocumentHost(This,pddh) ) 

#define IDebugDocumentHelper32_AddDeferredText(This,cChars,dwTextStartCookie)	\
    ( (This)->lpVtbl -> AddDeferredText(This,cChars,dwTextStartCookie) ) 

#define IDebugDocumentHelper32_DefineScriptBlock(This,ulCharOffset,cChars,pas,fScriptlet,pdwSourceContext)	\
    ( (This)->lpVtbl -> DefineScriptBlock(This,ulCharOffset,cChars,pas,fScriptlet,pdwSourceContext) ) 

#define IDebugDocumentHelper32_SetDefaultTextAttr(This,staTextAttr)	\
    ( (This)->lpVtbl -> SetDefaultTextAttr(This,staTextAttr) ) 

#define IDebugDocumentHelper32_SetTextAttributes(This,ulCharOffset,cChars,pstaTextAttr)	\
    ( (This)->lpVtbl -> SetTextAttributes(This,ulCharOffset,cChars,pstaTextAttr) ) 

#define IDebugDocumentHelper32_SetLongName(This,pszLongName)	\
    ( (This)->lpVtbl -> SetLongName(This,pszLongName) ) 

#define IDebugDocumentHelper32_SetShortName(This,pszShortName)	\
    ( (This)->lpVtbl -> SetShortName(This,pszShortName) ) 

#define IDebugDocumentHelper32_SetDocumentAttr(This,pszAttributes)	\
    ( (This)->lpVtbl -> SetDocumentAttr(This,pszAttributes) ) 

#define IDebugDocumentHelper32_GetDebugApplicationNode(This,ppdan)	\
    ( (This)->lpVtbl -> GetDebugApplicationNode(This,ppdan) ) 

#define IDebugDocumentHelper32_GetScriptBlockInfo(This,dwSourceContext,ppasd,piCharPos,pcChars)	\
    ( (This)->lpVtbl -> GetScriptBlockInfo(This,dwSourceContext,ppasd,piCharPos,pcChars) ) 

#define IDebugDocumentHelper32_CreateDebugDocumentContext(This,iCharPos,cChars,ppddc)	\
    ( (This)->lpVtbl -> CreateDebugDocumentContext(This,iCharPos,cChars,ppddc) ) 

#define IDebugDocumentHelper32_BringDocumentToTop(This)	\
    ( (This)->lpVtbl -> BringDocumentToTop(This) ) 

#define IDebugDocumentHelper32_BringDocumentContextToTop(This,pddc)	\
    ( (This)->lpVtbl -> BringDocumentContextToTop(This,pddc) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IDebugDocumentHelper32_INTERFACE_DEFINED__ */


#ifndef __IDebugDocumentHelper64_INTERFACE_DEFINED__
#define __IDebugDocumentHelper64_INTERFACE_DEFINED__

/* interface IDebugDocumentHelper64 */
/* [unique][helpstring][uuid][object] */ 


EXTERN_C const IID IID_IDebugDocumentHelper64;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("c4c7363c-20fd-47f9-bd82-4855e0150871")
    IDebugDocumentHelper64 : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE Init( 
            /* [in] */ __RPC__in_opt IDebugApplication64 *pda,
            /* [string][in] */ __RPC__in_string LPCOLESTR pszShortName,
            /* [string][in] */ __RPC__in_string LPCOLESTR pszLongName,
            /* [in] */ TEXT_DOC_ATTR docAttr) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Attach( 
            /* [in] */ __RPC__in_opt IDebugDocumentHelper64 *pddhParent) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Detach( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE AddUnicodeText( 
            /* [string][in] */ __RPC__in_string LPCOLESTR pszText) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE AddDBCSText( 
            /* [string][in] */ __RPC__in_string LPCSTR pszText) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetDebugDocumentHost( 
            /* [in] */ __RPC__in_opt IDebugDocumentHost *pddh) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE AddDeferredText( 
            /* [in] */ ULONG cChars,
            /* [in] */ DWORD dwTextStartCookie) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE DefineScriptBlock( 
            /* [in] */ ULONG ulCharOffset,
            /* [in] */ ULONG cChars,
            /* [in] */ __RPC__in_opt IActiveScript *pas,
            /* [in] */ BOOL fScriptlet,
            /* [out] */ __RPC__out DWORDLONG *pdwSourceContext) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetDefaultTextAttr( 
            SOURCE_TEXT_ATTR staTextAttr) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetTextAttributes( 
            /* [in] */ ULONG ulCharOffset,
            /* [in] */ ULONG cChars,
            /* [size_is][length_is][in] */ __RPC__in_ecount_part(cChars, cChars) SOURCE_TEXT_ATTR *pstaTextAttr) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetLongName( 
            /* [string][in] */ __RPC__in_string LPCOLESTR pszLongName) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetShortName( 
            /* [string][in] */ __RPC__in_string LPCOLESTR pszShortName) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetDocumentAttr( 
            /* [in] */ TEXT_DOC_ATTR pszAttributes) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetDebugApplicationNode( 
            /* [out] */ __RPC__deref_out_opt IDebugApplicationNode **ppdan) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetScriptBlockInfo( 
            /* [in] */ DWORDLONG dwSourceContext,
            /* [out] */ __RPC__deref_out_opt IActiveScript **ppasd,
            /* [out] */ __RPC__out ULONG *piCharPos,
            /* [out] */ __RPC__out ULONG *pcChars) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE CreateDebugDocumentContext( 
            /* [in] */ ULONG iCharPos,
            /* [in] */ ULONG cChars,
            /* [out] */ __RPC__deref_out_opt IDebugDocumentContext **ppddc) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE BringDocumentToTop( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE BringDocumentContextToTop( 
            __RPC__in_opt IDebugDocumentContext *pddc) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IDebugDocumentHelper64Vtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IDebugDocumentHelper64 * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IDebugDocumentHelper64 * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IDebugDocumentHelper64 * This);
        
        DECLSPEC_XFGVIRT(IDebugDocumentHelper64, Init)
        HRESULT ( STDMETHODCALLTYPE *Init )( 
            __RPC__in IDebugDocumentHelper64 * This,
            /* [in] */ __RPC__in_opt IDebugApplication64 *pda,
            /* [string][in] */ __RPC__in_string LPCOLESTR pszShortName,
            /* [string][in] */ __RPC__in_string LPCOLESTR pszLongName,
            /* [in] */ TEXT_DOC_ATTR docAttr);
        
        DECLSPEC_XFGVIRT(IDebugDocumentHelper64, Attach)
        HRESULT ( STDMETHODCALLTYPE *Attach )( 
            __RPC__in IDebugDocumentHelper64 * This,
            /* [in] */ __RPC__in_opt IDebugDocumentHelper64 *pddhParent);
        
        DECLSPEC_XFGVIRT(IDebugDocumentHelper64, Detach)
        HRESULT ( STDMETHODCALLTYPE *Detach )( 
            __RPC__in IDebugDocumentHelper64 * This);
        
        DECLSPEC_XFGVIRT(IDebugDocumentHelper64, AddUnicodeText)
        HRESULT ( STDMETHODCALLTYPE *AddUnicodeText )( 
            __RPC__in IDebugDocumentHelper64 * This,
            /* [string][in] */ __RPC__in_string LPCOLESTR pszText);
        
        DECLSPEC_XFGVIRT(IDebugDocumentHelper64, AddDBCSText)
        HRESULT ( STDMETHODCALLTYPE *AddDBCSText )( 
            __RPC__in IDebugDocumentHelper64 * This,
            /* [string][in] */ __RPC__in_string LPCSTR pszText);
        
        DECLSPEC_XFGVIRT(IDebugDocumentHelper64, SetDebugDocumentHost)
        HRESULT ( STDMETHODCALLTYPE *SetDebugDocumentHost )( 
            __RPC__in IDebugDocumentHelper64 * This,
            /* [in] */ __RPC__in_opt IDebugDocumentHost *pddh);
        
        DECLSPEC_XFGVIRT(IDebugDocumentHelper64, AddDeferredText)
        HRESULT ( STDMETHODCALLTYPE *AddDeferredText )( 
            __RPC__in IDebugDocumentHelper64 * This,
            /* [in] */ ULONG cChars,
            /* [in] */ DWORD dwTextStartCookie);
        
        DECLSPEC_XFGVIRT(IDebugDocumentHelper64, DefineScriptBlock)
        HRESULT ( STDMETHODCALLTYPE *DefineScriptBlock )( 
            __RPC__in IDebugDocumentHelper64 * This,
            /* [in] */ ULONG ulCharOffset,
            /* [in] */ ULONG cChars,
            /* [in] */ __RPC__in_opt IActiveScript *pas,
            /* [in] */ BOOL fScriptlet,
            /* [out] */ __RPC__out DWORDLONG *pdwSourceContext);
        
        DECLSPEC_XFGVIRT(IDebugDocumentHelper64, SetDefaultTextAttr)
        HRESULT ( STDMETHODCALLTYPE *SetDefaultTextAttr )( 
            __RPC__in IDebugDocumentHelper64 * This,
            SOURCE_TEXT_ATTR staTextAttr);
        
        DECLSPEC_XFGVIRT(IDebugDocumentHelper64, SetTextAttributes)
        HRESULT ( STDMETHODCALLTYPE *SetTextAttributes )( 
            __RPC__in IDebugDocumentHelper64 * This,
            /* [in] */ ULONG ulCharOffset,
            /* [in] */ ULONG cChars,
            /* [size_is][length_is][in] */ __RPC__in_ecount_part(cChars, cChars) SOURCE_TEXT_ATTR *pstaTextAttr);
        
        DECLSPEC_XFGVIRT(IDebugDocumentHelper64, SetLongName)
        HRESULT ( STDMETHODCALLTYPE *SetLongName )( 
            __RPC__in IDebugDocumentHelper64 * This,
            /* [string][in] */ __RPC__in_string LPCOLESTR pszLongName);
        
        DECLSPEC_XFGVIRT(IDebugDocumentHelper64, SetShortName)
        HRESULT ( STDMETHODCALLTYPE *SetShortName )( 
            __RPC__in IDebugDocumentHelper64 * This,
            /* [string][in] */ __RPC__in_string LPCOLESTR pszShortName);
        
        DECLSPEC_XFGVIRT(IDebugDocumentHelper64, SetDocumentAttr)
        HRESULT ( STDMETHODCALLTYPE *SetDocumentAttr )( 
            __RPC__in IDebugDocumentHelper64 * This,
            /* [in] */ TEXT_DOC_ATTR pszAttributes);
        
        DECLSPEC_XFGVIRT(IDebugDocumentHelper64, GetDebugApplicationNode)
        HRESULT ( STDMETHODCALLTYPE *GetDebugApplicationNode )( 
            __RPC__in IDebugDocumentHelper64 * This,
            /* [out] */ __RPC__deref_out_opt IDebugApplicationNode **ppdan);
        
        DECLSPEC_XFGVIRT(IDebugDocumentHelper64, GetScriptBlockInfo)
        HRESULT ( STDMETHODCALLTYPE *GetScriptBlockInfo )( 
            __RPC__in IDebugDocumentHelper64 * This,
            /* [in] */ DWORDLONG dwSourceContext,
            /* [out] */ __RPC__deref_out_opt IActiveScript **ppasd,
            /* [out] */ __RPC__out ULONG *piCharPos,
            /* [out] */ __RPC__out ULONG *pcChars);
        
        DECLSPEC_XFGVIRT(IDebugDocumentHelper64, CreateDebugDocumentContext)
        HRESULT ( STDMETHODCALLTYPE *CreateDebugDocumentContext )( 
            __RPC__in IDebugDocumentHelper64 * This,
            /* [in] */ ULONG iCharPos,
            /* [in] */ ULONG cChars,
            /* [out] */ __RPC__deref_out_opt IDebugDocumentContext **ppddc);
        
        DECLSPEC_XFGVIRT(IDebugDocumentHelper64, BringDocumentToTop)
        HRESULT ( STDMETHODCALLTYPE *BringDocumentToTop )( 
            __RPC__in IDebugDocumentHelper64 * This);
        
        DECLSPEC_XFGVIRT(IDebugDocumentHelper64, BringDocumentContextToTop)
        HRESULT ( STDMETHODCALLTYPE *BringDocumentContextToTop )( 
            __RPC__in IDebugDocumentHelper64 * This,
            __RPC__in_opt IDebugDocumentContext *pddc);
        
        END_INTERFACE
    } IDebugDocumentHelper64Vtbl;

    interface IDebugDocumentHelper64
    {
        CONST_VTBL struct IDebugDocumentHelper64Vtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IDebugDocumentHelper64_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IDebugDocumentHelper64_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IDebugDocumentHelper64_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IDebugDocumentHelper64_Init(This,pda,pszShortName,pszLongName,docAttr)	\
    ( (This)->lpVtbl -> Init(This,pda,pszShortName,pszLongName,docAttr) ) 

#define IDebugDocumentHelper64_Attach(This,pddhParent)	\
    ( (This)->lpVtbl -> Attach(This,pddhParent) ) 

#define IDebugDocumentHelper64_Detach(This)	\
    ( (This)->lpVtbl -> Detach(This) ) 

#define IDebugDocumentHelper64_AddUnicodeText(This,pszText)	\
    ( (This)->lpVtbl -> AddUnicodeText(This,pszText) ) 

#define IDebugDocumentHelper64_AddDBCSText(This,pszText)	\
    ( (This)->lpVtbl -> AddDBCSText(This,pszText) ) 

#define IDebugDocumentHelper64_SetDebugDocumentHost(This,pddh)	\
    ( (This)->lpVtbl -> SetDebugDocumentHost(This,pddh) ) 

#define IDebugDocumentHelper64_AddDeferredText(This,cChars,dwTextStartCookie)	\
    ( (This)->lpVtbl -> AddDeferredText(This,cChars,dwTextStartCookie) ) 

#define IDebugDocumentHelper64_DefineScriptBlock(This,ulCharOffset,cChars,pas,fScriptlet,pdwSourceContext)	\
    ( (This)->lpVtbl -> DefineScriptBlock(This,ulCharOffset,cChars,pas,fScriptlet,pdwSourceContext) ) 

#define IDebugDocumentHelper64_SetDefaultTextAttr(This,staTextAttr)	\
    ( (This)->lpVtbl -> SetDefaultTextAttr(This,staTextAttr) ) 

#define IDebugDocumentHelper64_SetTextAttributes(This,ulCharOffset,cChars,pstaTextAttr)	\
    ( (This)->lpVtbl -> SetTextAttributes(This,ulCharOffset,cChars,pstaTextAttr) ) 

#define IDebugDocumentHelper64_SetLongName(This,pszLongName)	\
    ( (This)->lpVtbl -> SetLongName(This,pszLongName) ) 

#define IDebugDocumentHelper64_SetShortName(This,pszShortName)	\
    ( (This)->lpVtbl -> SetShortName(This,pszShortName) ) 

#define IDebugDocumentHelper64_SetDocumentAttr(This,pszAttributes)	\
    ( (This)->lpVtbl -> SetDocumentAttr(This,pszAttributes) ) 

#define IDebugDocumentHelper64_GetDebugApplicationNode(This,ppdan)	\
    ( (This)->lpVtbl -> GetDebugApplicationNode(This,ppdan) ) 

#define IDebugDocumentHelper64_GetScriptBlockInfo(This,dwSourceContext,ppasd,piCharPos,pcChars)	\
    ( (This)->lpVtbl -> GetScriptBlockInfo(This,dwSourceContext,ppasd,piCharPos,pcChars) ) 

#define IDebugDocumentHelper64_CreateDebugDocumentContext(This,iCharPos,cChars,ppddc)	\
    ( (This)->lpVtbl -> CreateDebugDocumentContext(This,iCharPos,cChars,ppddc) ) 

#define IDebugDocumentHelper64_BringDocumentToTop(This)	\
    ( (This)->lpVtbl -> BringDocumentToTop(This) ) 

#define IDebugDocumentHelper64_BringDocumentContextToTop(This,pddc)	\
    ( (This)->lpVtbl -> BringDocumentContextToTop(This,pddc) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IDebugDocumentHelper64_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_activdbg_0000_0029 */
/* [local] */ 

#pragma warning(push)
#pragma warning(disable:28718)


extern RPC_IF_HANDLE __MIDL_itf_activdbg_0000_0029_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_activdbg_0000_0029_v0_0_s_ifspec;

#ifndef __IDebugDocumentHost_INTERFACE_DEFINED__
#define __IDebugDocumentHost_INTERFACE_DEFINED__

/* interface IDebugDocumentHost */
/* [unique][helpstring][uuid][object] */ 


EXTERN_C const IID IID_IDebugDocumentHost;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("51973C27-CB0C-11d0-B5C9-00A0244A0E7A")
    IDebugDocumentHost : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetDeferredText( 
            /* [in] */ DWORD dwTextStartCookie,
            /* [size_is][length_is][out][in] */ __RPC__inout_ecount_part(cMaxChars, *pcNumChars) WCHAR *pcharText,
            /* [size_is][length_is][out][in] */ __RPC__inout_ecount_part(cMaxChars, *pcNumChars) SOURCE_TEXT_ATTR *pstaTextAttr,
            /* [out][in] */ __RPC__inout ULONG *pcNumChars,
            /* [in] */ ULONG cMaxChars) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetScriptTextAttributes( 
            /* [size_is][in] */ __RPC__in_ecount_full(uNumCodeChars) LPCOLESTR pstrCode,
            /* [in] */ ULONG uNumCodeChars,
            /* [in] */ __RPC__in LPCOLESTR pstrDelimiter,
            /* [in] */ DWORD dwFlags,
            /* [size_is][out][in] */ __RPC__inout_ecount_full(uNumCodeChars) SOURCE_TEXT_ATTR *pattr) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE OnCreateDocumentContext( 
            /* [out] */ __RPC__deref_out_opt IUnknown **ppunkOuter) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetPathName( 
            /* [out] */ __RPC__deref_out_opt BSTR *pbstrLongName,
            /* [out] */ __RPC__out BOOL *pfIsOriginalFile) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetFileName( 
            /* [out] */ __RPC__deref_out_opt BSTR *pbstrShortName) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE NotifyChanged( void) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IDebugDocumentHostVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IDebugDocumentHost * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IDebugDocumentHost * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IDebugDocumentHost * This);
        
        DECLSPEC_XFGVIRT(IDebugDocumentHost, GetDeferredText)
        HRESULT ( STDMETHODCALLTYPE *GetDeferredText )( 
            __RPC__in IDebugDocumentHost * This,
            /* [in] */ DWORD dwTextStartCookie,
            /* [size_is][length_is][out][in] */ __RPC__inout_ecount_part(cMaxChars, *pcNumChars) WCHAR *pcharText,
            /* [size_is][length_is][out][in] */ __RPC__inout_ecount_part(cMaxChars, *pcNumChars) SOURCE_TEXT_ATTR *pstaTextAttr,
            /* [out][in] */ __RPC__inout ULONG *pcNumChars,
            /* [in] */ ULONG cMaxChars);
        
        DECLSPEC_XFGVIRT(IDebugDocumentHost, GetScriptTextAttributes)
        HRESULT ( STDMETHODCALLTYPE *GetScriptTextAttributes )( 
            __RPC__in IDebugDocumentHost * This,
            /* [size_is][in] */ __RPC__in_ecount_full(uNumCodeChars) LPCOLESTR pstrCode,
            /* [in] */ ULONG uNumCodeChars,
            /* [in] */ __RPC__in LPCOLESTR pstrDelimiter,
            /* [in] */ DWORD dwFlags,
            /* [size_is][out][in] */ __RPC__inout_ecount_full(uNumCodeChars) SOURCE_TEXT_ATTR *pattr);
        
        DECLSPEC_XFGVIRT(IDebugDocumentHost, OnCreateDocumentContext)
        HRESULT ( STDMETHODCALLTYPE *OnCreateDocumentContext )( 
            __RPC__in IDebugDocumentHost * This,
            /* [out] */ __RPC__deref_out_opt IUnknown **ppunkOuter);
        
        DECLSPEC_XFGVIRT(IDebugDocumentHost, GetPathName)
        HRESULT ( STDMETHODCALLTYPE *GetPathName )( 
            __RPC__in IDebugDocumentHost * This,
            /* [out] */ __RPC__deref_out_opt BSTR *pbstrLongName,
            /* [out] */ __RPC__out BOOL *pfIsOriginalFile);
        
        DECLSPEC_XFGVIRT(IDebugDocumentHost, GetFileName)
        HRESULT ( STDMETHODCALLTYPE *GetFileName )( 
            __RPC__in IDebugDocumentHost * This,
            /* [out] */ __RPC__deref_out_opt BSTR *pbstrShortName);
        
        DECLSPEC_XFGVIRT(IDebugDocumentHost, NotifyChanged)
        HRESULT ( STDMETHODCALLTYPE *NotifyChanged )( 
            __RPC__in IDebugDocumentHost * This);
        
        END_INTERFACE
    } IDebugDocumentHostVtbl;

    interface IDebugDocumentHost
    {
        CONST_VTBL struct IDebugDocumentHostVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IDebugDocumentHost_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IDebugDocumentHost_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IDebugDocumentHost_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IDebugDocumentHost_GetDeferredText(This,dwTextStartCookie,pcharText,pstaTextAttr,pcNumChars,cMaxChars)	\
    ( (This)->lpVtbl -> GetDeferredText(This,dwTextStartCookie,pcharText,pstaTextAttr,pcNumChars,cMaxChars) ) 

#define IDebugDocumentHost_GetScriptTextAttributes(This,pstrCode,uNumCodeChars,pstrDelimiter,dwFlags,pattr)	\
    ( (This)->lpVtbl -> GetScriptTextAttributes(This,pstrCode,uNumCodeChars,pstrDelimiter,dwFlags,pattr) ) 

#define IDebugDocumentHost_OnCreateDocumentContext(This,ppunkOuter)	\
    ( (This)->lpVtbl -> OnCreateDocumentContext(This,ppunkOuter) ) 

#define IDebugDocumentHost_GetPathName(This,pbstrLongName,pfIsOriginalFile)	\
    ( (This)->lpVtbl -> GetPathName(This,pbstrLongName,pfIsOriginalFile) ) 

#define IDebugDocumentHost_GetFileName(This,pbstrShortName)	\
    ( (This)->lpVtbl -> GetFileName(This,pbstrShortName) ) 

#define IDebugDocumentHost_NotifyChanged(This)	\
    ( (This)->lpVtbl -> NotifyChanged(This) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IDebugDocumentHost_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_activdbg_0000_0030 */
/* [local] */ 

#pragma warning(pop)


extern RPC_IF_HANDLE __MIDL_itf_activdbg_0000_0030_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_activdbg_0000_0030_v0_0_s_ifspec;

#ifndef __IDebugDocumentContext_INTERFACE_DEFINED__
#define __IDebugDocumentContext_INTERFACE_DEFINED__

/* interface IDebugDocumentContext */
/* [unique][uuid][object] */ 


EXTERN_C const IID IID_IDebugDocumentContext;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("51973C28-CB0C-11d0-B5C9-00A0244A0E7A")
    IDebugDocumentContext : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetDocument( 
            /* [out] */ __RPC__deref_out_opt IDebugDocument **ppsd) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE EnumCodeContexts( 
            /* [out] */ __RPC__deref_out_opt IEnumDebugCodeContexts **ppescc) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IDebugDocumentContextVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IDebugDocumentContext * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IDebugDocumentContext * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IDebugDocumentContext * This);
        
        DECLSPEC_XFGVIRT(IDebugDocumentContext, GetDocument)
        HRESULT ( STDMETHODCALLTYPE *GetDocument )( 
            __RPC__in IDebugDocumentContext * This,
            /* [out] */ __RPC__deref_out_opt IDebugDocument **ppsd);
        
        DECLSPEC_XFGVIRT(IDebugDocumentContext, EnumCodeContexts)
        HRESULT ( STDMETHODCALLTYPE *EnumCodeContexts )( 
            __RPC__in IDebugDocumentContext * This,
            /* [out] */ __RPC__deref_out_opt IEnumDebugCodeContexts **ppescc);
        
        END_INTERFACE
    } IDebugDocumentContextVtbl;

    interface IDebugDocumentContext
    {
        CONST_VTBL struct IDebugDocumentContextVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IDebugDocumentContext_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IDebugDocumentContext_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IDebugDocumentContext_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IDebugDocumentContext_GetDocument(This,ppsd)	\
    ( (This)->lpVtbl -> GetDocument(This,ppsd) ) 

#define IDebugDocumentContext_EnumCodeContexts(This,ppescc)	\
    ( (This)->lpVtbl -> EnumCodeContexts(This,ppescc) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IDebugDocumentContext_INTERFACE_DEFINED__ */


#ifndef __IDebugSessionProvider_INTERFACE_DEFINED__
#define __IDebugSessionProvider_INTERFACE_DEFINED__

/* interface IDebugSessionProvider */
/* [unique][helpstring][uuid][object] */ 


EXTERN_C const IID IID_IDebugSessionProvider;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("51973C29-CB0C-11d0-B5C9-00A0244A0E7A")
    IDebugSessionProvider : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE StartDebugSession( 
            /* [in] */ __RPC__in_opt IRemoteDebugApplication *pda) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IDebugSessionProviderVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IDebugSessionProvider * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IDebugSessionProvider * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IDebugSessionProvider * This);
        
        DECLSPEC_XFGVIRT(IDebugSessionProvider, StartDebugSession)
        HRESULT ( STDMETHODCALLTYPE *StartDebugSession )( 
            __RPC__in IDebugSessionProvider * This,
            /* [in] */ __RPC__in_opt IRemoteDebugApplication *pda);
        
        END_INTERFACE
    } IDebugSessionProviderVtbl;

    interface IDebugSessionProvider
    {
        CONST_VTBL struct IDebugSessionProviderVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IDebugSessionProvider_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IDebugSessionProvider_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IDebugSessionProvider_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IDebugSessionProvider_StartDebugSession(This,pda)	\
    ( (This)->lpVtbl -> StartDebugSession(This,pda) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IDebugSessionProvider_INTERFACE_DEFINED__ */


#ifndef __IApplicationDebugger_INTERFACE_DEFINED__
#define __IApplicationDebugger_INTERFACE_DEFINED__

/* interface IApplicationDebugger */
/* [unique][helpstring][uuid][object] */ 


EXTERN_C const IID IID_IApplicationDebugger;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("51973C2a-CB0C-11d0-B5C9-00A0244A0E7A")
    IApplicationDebugger : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE QueryAlive( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE CreateInstanceAtDebugger( 
            /* [in] */ __RPC__in REFCLSID rclsid,
            /* [in] */ __RPC__in_opt IUnknown *pUnkOuter,
            /* [in] */ DWORD dwClsContext,
            /* [in] */ __RPC__in REFIID riid,
            /* [iid_is][out] */ __RPC__deref_out_opt IUnknown **ppvObject) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE onDebugOutput( 
            /* [in] */ __RPC__in LPCOLESTR pstr) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE onHandleBreakPoint( 
            /* [in] */ __RPC__in_opt IRemoteDebugApplicationThread *prpt,
            /* [in] */ BREAKREASON br,
            /* [in] */ __RPC__in_opt IActiveScriptErrorDebug *pError) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE onClose( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE onDebuggerEvent( 
            /* [in] */ __RPC__in REFIID riid,
            /* [in] */ __RPC__in_opt IUnknown *punk) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IApplicationDebuggerVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IApplicationDebugger * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IApplicationDebugger * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IApplicationDebugger * This);
        
        DECLSPEC_XFGVIRT(IApplicationDebugger, QueryAlive)
        HRESULT ( STDMETHODCALLTYPE *QueryAlive )( 
            __RPC__in IApplicationDebugger * This);
        
        DECLSPEC_XFGVIRT(IApplicationDebugger, CreateInstanceAtDebugger)
        HRESULT ( STDMETHODCALLTYPE *CreateInstanceAtDebugger )( 
            __RPC__in IApplicationDebugger * This,
            /* [in] */ __RPC__in REFCLSID rclsid,
            /* [in] */ __RPC__in_opt IUnknown *pUnkOuter,
            /* [in] */ DWORD dwClsContext,
            /* [in] */ __RPC__in REFIID riid,
            /* [iid_is][out] */ __RPC__deref_out_opt IUnknown **ppvObject);
        
        DECLSPEC_XFGVIRT(IApplicationDebugger, onDebugOutput)
        HRESULT ( STDMETHODCALLTYPE *onDebugOutput )( 
            __RPC__in IApplicationDebugger * This,
            /* [in] */ __RPC__in LPCOLESTR pstr);
        
        DECLSPEC_XFGVIRT(IApplicationDebugger, onHandleBreakPoint)
        HRESULT ( STDMETHODCALLTYPE *onHandleBreakPoint )( 
            __RPC__in IApplicationDebugger * This,
            /* [in] */ __RPC__in_opt IRemoteDebugApplicationThread *prpt,
            /* [in] */ BREAKREASON br,
            /* [in] */ __RPC__in_opt IActiveScriptErrorDebug *pError);
        
        DECLSPEC_XFGVIRT(IApplicationDebugger, onClose)
        HRESULT ( STDMETHODCALLTYPE *onClose )( 
            __RPC__in IApplicationDebugger * This);
        
        DECLSPEC_XFGVIRT(IApplicationDebugger, onDebuggerEvent)
        HRESULT ( STDMETHODCALLTYPE *onDebuggerEvent )( 
            __RPC__in IApplicationDebugger * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [in] */ __RPC__in_opt IUnknown *punk);
        
        END_INTERFACE
    } IApplicationDebuggerVtbl;

    interface IApplicationDebugger
    {
        CONST_VTBL struct IApplicationDebuggerVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IApplicationDebugger_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IApplicationDebugger_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IApplicationDebugger_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IApplicationDebugger_QueryAlive(This)	\
    ( (This)->lpVtbl -> QueryAlive(This) ) 

#define IApplicationDebugger_CreateInstanceAtDebugger(This,rclsid,pUnkOuter,dwClsContext,riid,ppvObject)	\
    ( (This)->lpVtbl -> CreateInstanceAtDebugger(This,rclsid,pUnkOuter,dwClsContext,riid,ppvObject) ) 

#define IApplicationDebugger_onDebugOutput(This,pstr)	\
    ( (This)->lpVtbl -> onDebugOutput(This,pstr) ) 

#define IApplicationDebugger_onHandleBreakPoint(This,prpt,br,pError)	\
    ( (This)->lpVtbl -> onHandleBreakPoint(This,prpt,br,pError) ) 

#define IApplicationDebugger_onClose(This)	\
    ( (This)->lpVtbl -> onClose(This) ) 

#define IApplicationDebugger_onDebuggerEvent(This,riid,punk)	\
    ( (This)->lpVtbl -> onDebuggerEvent(This,riid,punk) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IApplicationDebugger_INTERFACE_DEFINED__ */


#ifndef __IApplicationDebuggerUI_INTERFACE_DEFINED__
#define __IApplicationDebuggerUI_INTERFACE_DEFINED__

/* interface IApplicationDebuggerUI */
/* [unique][helpstring][uuid][object] */ 


EXTERN_C const IID IID_IApplicationDebuggerUI;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("51973C2b-CB0C-11d0-B5C9-00A0244A0E7A")
    IApplicationDebuggerUI : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE BringDocumentToTop( 
            /* [in] */ __RPC__in_opt IDebugDocumentText *pddt) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE BringDocumentContextToTop( 
            /* [in] */ __RPC__in_opt IDebugDocumentContext *pddc) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IApplicationDebuggerUIVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IApplicationDebuggerUI * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IApplicationDebuggerUI * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IApplicationDebuggerUI * This);
        
        DECLSPEC_XFGVIRT(IApplicationDebuggerUI, BringDocumentToTop)
        HRESULT ( STDMETHODCALLTYPE *BringDocumentToTop )( 
            __RPC__in IApplicationDebuggerUI * This,
            /* [in] */ __RPC__in_opt IDebugDocumentText *pddt);
        
        DECLSPEC_XFGVIRT(IApplicationDebuggerUI, BringDocumentContextToTop)
        HRESULT ( STDMETHODCALLTYPE *BringDocumentContextToTop )( 
            __RPC__in IApplicationDebuggerUI * This,
            /* [in] */ __RPC__in_opt IDebugDocumentContext *pddc);
        
        END_INTERFACE
    } IApplicationDebuggerUIVtbl;

    interface IApplicationDebuggerUI
    {
        CONST_VTBL struct IApplicationDebuggerUIVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IApplicationDebuggerUI_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IApplicationDebuggerUI_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IApplicationDebuggerUI_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IApplicationDebuggerUI_BringDocumentToTop(This,pddt)	\
    ( (This)->lpVtbl -> BringDocumentToTop(This,pddt) ) 

#define IApplicationDebuggerUI_BringDocumentContextToTop(This,pddc)	\
    ( (This)->lpVtbl -> BringDocumentContextToTop(This,pddc) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IApplicationDebuggerUI_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_activdbg_0000_0034 */
/* [local] */ 

EXTERN_C const CLSID CLSID_MachineDebugManager;


extern RPC_IF_HANDLE __MIDL_itf_activdbg_0000_0034_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_activdbg_0000_0034_v0_0_s_ifspec;

#ifndef __IMachineDebugManager_INTERFACE_DEFINED__
#define __IMachineDebugManager_INTERFACE_DEFINED__

/* interface IMachineDebugManager */
/* [unique][helpstring][uuid][object] */ 


EXTERN_C const IID IID_IMachineDebugManager;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("51973C2c-CB0C-11d0-B5C9-00A0244A0E7A")
    IMachineDebugManager : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE AddApplication( 
            /* [in] */ __RPC__in_opt IRemoteDebugApplication *pda,
            /* [out] */ __RPC__out DWORD *pdwAppCookie) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE RemoveApplication( 
            /* [in] */ DWORD dwAppCookie) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE EnumApplications( 
            /* [out] */ __RPC__deref_out_opt IEnumRemoteDebugApplications **ppeda) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IMachineDebugManagerVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IMachineDebugManager * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IMachineDebugManager * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IMachineDebugManager * This);
        
        DECLSPEC_XFGVIRT(IMachineDebugManager, AddApplication)
        HRESULT ( STDMETHODCALLTYPE *AddApplication )( 
            __RPC__in IMachineDebugManager * This,
            /* [in] */ __RPC__in_opt IRemoteDebugApplication *pda,
            /* [out] */ __RPC__out DWORD *pdwAppCookie);
        
        DECLSPEC_XFGVIRT(IMachineDebugManager, RemoveApplication)
        HRESULT ( STDMETHODCALLTYPE *RemoveApplication )( 
            __RPC__in IMachineDebugManager * This,
            /* [in] */ DWORD dwAppCookie);
        
        DECLSPEC_XFGVIRT(IMachineDebugManager, EnumApplications)
        HRESULT ( STDMETHODCALLTYPE *EnumApplications )( 
            __RPC__in IMachineDebugManager * This,
            /* [out] */ __RPC__deref_out_opt IEnumRemoteDebugApplications **ppeda);
        
        END_INTERFACE
    } IMachineDebugManagerVtbl;

    interface IMachineDebugManager
    {
        CONST_VTBL struct IMachineDebugManagerVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IMachineDebugManager_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IMachineDebugManager_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IMachineDebugManager_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IMachineDebugManager_AddApplication(This,pda,pdwAppCookie)	\
    ( (This)->lpVtbl -> AddApplication(This,pda,pdwAppCookie) ) 

#define IMachineDebugManager_RemoveApplication(This,dwAppCookie)	\
    ( (This)->lpVtbl -> RemoveApplication(This,dwAppCookie) ) 

#define IMachineDebugManager_EnumApplications(This,ppeda)	\
    ( (This)->lpVtbl -> EnumApplications(This,ppeda) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IMachineDebugManager_INTERFACE_DEFINED__ */


#ifndef __IMachineDebugManagerCookie_INTERFACE_DEFINED__
#define __IMachineDebugManagerCookie_INTERFACE_DEFINED__

/* interface IMachineDebugManagerCookie */
/* [unique][helpstring][uuid][object] */ 


EXTERN_C const IID IID_IMachineDebugManagerCookie;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("51973C2d-CB0C-11d0-B5C9-00A0244A0E7A")
    IMachineDebugManagerCookie : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE AddApplication( 
            /* [in] */ __RPC__in_opt IRemoteDebugApplication *pda,
            /* [in] */ DWORD dwDebugAppCookie,
            /* [out] */ __RPC__out DWORD *pdwAppCookie) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE RemoveApplication( 
            /* [in] */ DWORD dwDebugAppCookie,
            /* [in] */ DWORD dwAppCookie) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE EnumApplications( 
            /* [out] */ __RPC__deref_out_opt IEnumRemoteDebugApplications **ppeda) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IMachineDebugManagerCookieVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IMachineDebugManagerCookie * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IMachineDebugManagerCookie * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IMachineDebugManagerCookie * This);
        
        DECLSPEC_XFGVIRT(IMachineDebugManagerCookie, AddApplication)
        HRESULT ( STDMETHODCALLTYPE *AddApplication )( 
            __RPC__in IMachineDebugManagerCookie * This,
            /* [in] */ __RPC__in_opt IRemoteDebugApplication *pda,
            /* [in] */ DWORD dwDebugAppCookie,
            /* [out] */ __RPC__out DWORD *pdwAppCookie);
        
        DECLSPEC_XFGVIRT(IMachineDebugManagerCookie, RemoveApplication)
        HRESULT ( STDMETHODCALLTYPE *RemoveApplication )( 
            __RPC__in IMachineDebugManagerCookie * This,
            /* [in] */ DWORD dwDebugAppCookie,
            /* [in] */ DWORD dwAppCookie);
        
        DECLSPEC_XFGVIRT(IMachineDebugManagerCookie, EnumApplications)
        HRESULT ( STDMETHODCALLTYPE *EnumApplications )( 
            __RPC__in IMachineDebugManagerCookie * This,
            /* [out] */ __RPC__deref_out_opt IEnumRemoteDebugApplications **ppeda);
        
        END_INTERFACE
    } IMachineDebugManagerCookieVtbl;

    interface IMachineDebugManagerCookie
    {
        CONST_VTBL struct IMachineDebugManagerCookieVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IMachineDebugManagerCookie_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IMachineDebugManagerCookie_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IMachineDebugManagerCookie_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IMachineDebugManagerCookie_AddApplication(This,pda,dwDebugAppCookie,pdwAppCookie)	\
    ( (This)->lpVtbl -> AddApplication(This,pda,dwDebugAppCookie,pdwAppCookie) ) 

#define IMachineDebugManagerCookie_RemoveApplication(This,dwDebugAppCookie,dwAppCookie)	\
    ( (This)->lpVtbl -> RemoveApplication(This,dwDebugAppCookie,dwAppCookie) ) 

#define IMachineDebugManagerCookie_EnumApplications(This,ppeda)	\
    ( (This)->lpVtbl -> EnumApplications(This,ppeda) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IMachineDebugManagerCookie_INTERFACE_DEFINED__ */


#ifndef __IMachineDebugManagerEvents_INTERFACE_DEFINED__
#define __IMachineDebugManagerEvents_INTERFACE_DEFINED__

/* interface IMachineDebugManagerEvents */
/* [unique][helpstring][uuid][object] */ 


EXTERN_C const IID IID_IMachineDebugManagerEvents;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("51973C2e-CB0C-11d0-B5C9-00A0244A0E7A")
    IMachineDebugManagerEvents : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE onAddApplication( 
            /* [in] */ __RPC__in_opt IRemoteDebugApplication *pda,
            /* [in] */ DWORD dwAppCookie) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE onRemoveApplication( 
            /* [in] */ __RPC__in_opt IRemoteDebugApplication *pda,
            /* [in] */ DWORD dwAppCookie) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IMachineDebugManagerEventsVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IMachineDebugManagerEvents * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IMachineDebugManagerEvents * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IMachineDebugManagerEvents * This);
        
        DECLSPEC_XFGVIRT(IMachineDebugManagerEvents, onAddApplication)
        HRESULT ( STDMETHODCALLTYPE *onAddApplication )( 
            __RPC__in IMachineDebugManagerEvents * This,
            /* [in] */ __RPC__in_opt IRemoteDebugApplication *pda,
            /* [in] */ DWORD dwAppCookie);
        
        DECLSPEC_XFGVIRT(IMachineDebugManagerEvents, onRemoveApplication)
        HRESULT ( STDMETHODCALLTYPE *onRemoveApplication )( 
            __RPC__in IMachineDebugManagerEvents * This,
            /* [in] */ __RPC__in_opt IRemoteDebugApplication *pda,
            /* [in] */ DWORD dwAppCookie);
        
        END_INTERFACE
    } IMachineDebugManagerEventsVtbl;

    interface IMachineDebugManagerEvents
    {
        CONST_VTBL struct IMachineDebugManagerEventsVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IMachineDebugManagerEvents_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IMachineDebugManagerEvents_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IMachineDebugManagerEvents_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IMachineDebugManagerEvents_onAddApplication(This,pda,dwAppCookie)	\
    ( (This)->lpVtbl -> onAddApplication(This,pda,dwAppCookie) ) 

#define IMachineDebugManagerEvents_onRemoveApplication(This,pda,dwAppCookie)	\
    ( (This)->lpVtbl -> onRemoveApplication(This,pda,dwAppCookie) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IMachineDebugManagerEvents_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_activdbg_0000_0037 */
/* [local] */ 

EXTERN_C const CLSID CLSID_ProcessDebugManager;
#ifndef DISABLE_ACTIVDBG_INTERFACE_WRAPPERS
#ifdef _WIN64
#define IProcessDebugManager IProcessDebugManager64
#define IID_IProcessDebugManager IID_IProcessDebugManager64
#else
#define IProcessDebugManager IProcessDebugManager32
#define IID_IProcessDebugManager IID_IProcessDebugManager32
#endif
#endif


extern RPC_IF_HANDLE __MIDL_itf_activdbg_0000_0037_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_activdbg_0000_0037_v0_0_s_ifspec;

#ifndef __IProcessDebugManager32_INTERFACE_DEFINED__
#define __IProcessDebugManager32_INTERFACE_DEFINED__

/* interface IProcessDebugManager32 */
/* [local][unique][helpstring][uuid][object] */ 


EXTERN_C const IID IID_IProcessDebugManager32;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("51973C2f-CB0C-11d0-B5C9-00A0244A0E7A")
    IProcessDebugManager32 : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE CreateApplication( 
            /* [out] */ IDebugApplication32 **ppda) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetDefaultApplication( 
            /* [out] */ IDebugApplication32 **ppda) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE AddApplication( 
            /* [in] */ IDebugApplication32 *pda,
            /* [out] */ DWORD *pdwAppCookie) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE RemoveApplication( 
            /* [in] */ DWORD dwAppCookie) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE CreateDebugDocumentHelper( 
            /* [in] */ IUnknown *punkOuter,
            /* [out] */ IDebugDocumentHelper32 **pddh) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IProcessDebugManager32Vtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IProcessDebugManager32 * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IProcessDebugManager32 * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IProcessDebugManager32 * This);
        
        DECLSPEC_XFGVIRT(IProcessDebugManager32, CreateApplication)
        HRESULT ( STDMETHODCALLTYPE *CreateApplication )( 
            IProcessDebugManager32 * This,
            /* [out] */ IDebugApplication32 **ppda);
        
        DECLSPEC_XFGVIRT(IProcessDebugManager32, GetDefaultApplication)
        HRESULT ( STDMETHODCALLTYPE *GetDefaultApplication )( 
            IProcessDebugManager32 * This,
            /* [out] */ IDebugApplication32 **ppda);
        
        DECLSPEC_XFGVIRT(IProcessDebugManager32, AddApplication)
        HRESULT ( STDMETHODCALLTYPE *AddApplication )( 
            IProcessDebugManager32 * This,
            /* [in] */ IDebugApplication32 *pda,
            /* [out] */ DWORD *pdwAppCookie);
        
        DECLSPEC_XFGVIRT(IProcessDebugManager32, RemoveApplication)
        HRESULT ( STDMETHODCALLTYPE *RemoveApplication )( 
            IProcessDebugManager32 * This,
            /* [in] */ DWORD dwAppCookie);
        
        DECLSPEC_XFGVIRT(IProcessDebugManager32, CreateDebugDocumentHelper)
        HRESULT ( STDMETHODCALLTYPE *CreateDebugDocumentHelper )( 
            IProcessDebugManager32 * This,
            /* [in] */ IUnknown *punkOuter,
            /* [out] */ IDebugDocumentHelper32 **pddh);
        
        END_INTERFACE
    } IProcessDebugManager32Vtbl;

    interface IProcessDebugManager32
    {
        CONST_VTBL struct IProcessDebugManager32Vtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IProcessDebugManager32_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IProcessDebugManager32_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IProcessDebugManager32_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IProcessDebugManager32_CreateApplication(This,ppda)	\
    ( (This)->lpVtbl -> CreateApplication(This,ppda) ) 

#define IProcessDebugManager32_GetDefaultApplication(This,ppda)	\
    ( (This)->lpVtbl -> GetDefaultApplication(This,ppda) ) 

#define IProcessDebugManager32_AddApplication(This,pda,pdwAppCookie)	\
    ( (This)->lpVtbl -> AddApplication(This,pda,pdwAppCookie) ) 

#define IProcessDebugManager32_RemoveApplication(This,dwAppCookie)	\
    ( (This)->lpVtbl -> RemoveApplication(This,dwAppCookie) ) 

#define IProcessDebugManager32_CreateDebugDocumentHelper(This,punkOuter,pddh)	\
    ( (This)->lpVtbl -> CreateDebugDocumentHelper(This,punkOuter,pddh) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IProcessDebugManager32_INTERFACE_DEFINED__ */


#ifndef __IProcessDebugManager64_INTERFACE_DEFINED__
#define __IProcessDebugManager64_INTERFACE_DEFINED__

/* interface IProcessDebugManager64 */
/* [local][unique][helpstring][uuid][object] */ 


EXTERN_C const IID IID_IProcessDebugManager64;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("56b9fc1c-63a9-4cc1-ac21-087d69a17fab")
    IProcessDebugManager64 : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE CreateApplication( 
            /* [out] */ IDebugApplication64 **ppda) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetDefaultApplication( 
            /* [out] */ IDebugApplication64 **ppda) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE AddApplication( 
            /* [in] */ IDebugApplication64 *pda,
            /* [out] */ DWORD *pdwAppCookie) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE RemoveApplication( 
            /* [in] */ DWORD dwAppCookie) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE CreateDebugDocumentHelper( 
            /* [in] */ IUnknown *punkOuter,
            /* [out] */ IDebugDocumentHelper64 **pddh) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IProcessDebugManager64Vtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IProcessDebugManager64 * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IProcessDebugManager64 * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IProcessDebugManager64 * This);
        
        DECLSPEC_XFGVIRT(IProcessDebugManager64, CreateApplication)
        HRESULT ( STDMETHODCALLTYPE *CreateApplication )( 
            IProcessDebugManager64 * This,
            /* [out] */ IDebugApplication64 **ppda);
        
        DECLSPEC_XFGVIRT(IProcessDebugManager64, GetDefaultApplication)
        HRESULT ( STDMETHODCALLTYPE *GetDefaultApplication )( 
            IProcessDebugManager64 * This,
            /* [out] */ IDebugApplication64 **ppda);
        
        DECLSPEC_XFGVIRT(IProcessDebugManager64, AddApplication)
        HRESULT ( STDMETHODCALLTYPE *AddApplication )( 
            IProcessDebugManager64 * This,
            /* [in] */ IDebugApplication64 *pda,
            /* [out] */ DWORD *pdwAppCookie);
        
        DECLSPEC_XFGVIRT(IProcessDebugManager64, RemoveApplication)
        HRESULT ( STDMETHODCALLTYPE *RemoveApplication )( 
            IProcessDebugManager64 * This,
            /* [in] */ DWORD dwAppCookie);
        
        DECLSPEC_XFGVIRT(IProcessDebugManager64, CreateDebugDocumentHelper)
        HRESULT ( STDMETHODCALLTYPE *CreateDebugDocumentHelper )( 
            IProcessDebugManager64 * This,
            /* [in] */ IUnknown *punkOuter,
            /* [out] */ IDebugDocumentHelper64 **pddh);
        
        END_INTERFACE
    } IProcessDebugManager64Vtbl;

    interface IProcessDebugManager64
    {
        CONST_VTBL struct IProcessDebugManager64Vtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IProcessDebugManager64_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IProcessDebugManager64_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IProcessDebugManager64_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IProcessDebugManager64_CreateApplication(This,ppda)	\
    ( (This)->lpVtbl -> CreateApplication(This,ppda) ) 

#define IProcessDebugManager64_GetDefaultApplication(This,ppda)	\
    ( (This)->lpVtbl -> GetDefaultApplication(This,ppda) ) 

#define IProcessDebugManager64_AddApplication(This,pda,pdwAppCookie)	\
    ( (This)->lpVtbl -> AddApplication(This,pda,pdwAppCookie) ) 

#define IProcessDebugManager64_RemoveApplication(This,dwAppCookie)	\
    ( (This)->lpVtbl -> RemoveApplication(This,dwAppCookie) ) 

#define IProcessDebugManager64_CreateDebugDocumentHelper(This,punkOuter,pddh)	\
    ( (This)->lpVtbl -> CreateDebugDocumentHelper(This,punkOuter,pddh) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IProcessDebugManager64_INTERFACE_DEFINED__ */


#ifndef __IRemoteDebugApplication_INTERFACE_DEFINED__
#define __IRemoteDebugApplication_INTERFACE_DEFINED__

/* interface IRemoteDebugApplication */
/* [unique][helpstring][uuid][object] */ 


EXTERN_C const IID IID_IRemoteDebugApplication;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("51973C30-CB0C-11d0-B5C9-00A0244A0E7A")
    IRemoteDebugApplication : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE ResumeFromBreakPoint( 
            /* [in] */ __RPC__in_opt IRemoteDebugApplicationThread *prptFocus,
            /* [in] */ BREAKRESUMEACTION bra,
            /* [in] */ ERRORRESUMEACTION era) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE CauseBreak( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE ConnectDebugger( 
            /* [in] */ __RPC__in_opt IApplicationDebugger *pad) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE DisconnectDebugger( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetDebugger( 
            /* [out] */ __RPC__deref_out_opt IApplicationDebugger **pad) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE CreateInstanceAtApplication( 
            /* [in] */ __RPC__in REFCLSID rclsid,
            /* [in] */ __RPC__in_opt IUnknown *pUnkOuter,
            /* [in] */ DWORD dwClsContext,
            /* [in] */ __RPC__in REFIID riid,
            /* [iid_is][out] */ __RPC__deref_out_opt IUnknown **ppvObject) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE QueryAlive( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE EnumThreads( 
            /* [out] */ __RPC__deref_out_opt IEnumRemoteDebugApplicationThreads **pperdat) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetName( 
            /* [out] */ __RPC__deref_out_opt BSTR *pbstrName) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetRootNode( 
            /* [out] */ __RPC__deref_out_opt IDebugApplicationNode **ppdanRoot) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE EnumGlobalExpressionContexts( 
            /* [out] */ __RPC__deref_out_opt IEnumDebugExpressionContexts **ppedec) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IRemoteDebugApplicationVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IRemoteDebugApplication * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IRemoteDebugApplication * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IRemoteDebugApplication * This);
        
        DECLSPEC_XFGVIRT(IRemoteDebugApplication, ResumeFromBreakPoint)
        HRESULT ( STDMETHODCALLTYPE *ResumeFromBreakPoint )( 
            __RPC__in IRemoteDebugApplication * This,
            /* [in] */ __RPC__in_opt IRemoteDebugApplicationThread *prptFocus,
            /* [in] */ BREAKRESUMEACTION bra,
            /* [in] */ ERRORRESUMEACTION era);
        
        DECLSPEC_XFGVIRT(IRemoteDebugApplication, CauseBreak)
        HRESULT ( STDMETHODCALLTYPE *CauseBreak )( 
            __RPC__in IRemoteDebugApplication * This);
        
        DECLSPEC_XFGVIRT(IRemoteDebugApplication, ConnectDebugger)
        HRESULT ( STDMETHODCALLTYPE *ConnectDebugger )( 
            __RPC__in IRemoteDebugApplication * This,
            /* [in] */ __RPC__in_opt IApplicationDebugger *pad);
        
        DECLSPEC_XFGVIRT(IRemoteDebugApplication, DisconnectDebugger)
        HRESULT ( STDMETHODCALLTYPE *DisconnectDebugger )( 
            __RPC__in IRemoteDebugApplication * This);
        
        DECLSPEC_XFGVIRT(IRemoteDebugApplication, GetDebugger)
        HRESULT ( STDMETHODCALLTYPE *GetDebugger )( 
            __RPC__in IRemoteDebugApplication * This,
            /* [out] */ __RPC__deref_out_opt IApplicationDebugger **pad);
        
        DECLSPEC_XFGVIRT(IRemoteDebugApplication, CreateInstanceAtApplication)
        HRESULT ( STDMETHODCALLTYPE *CreateInstanceAtApplication )( 
            __RPC__in IRemoteDebugApplication * This,
            /* [in] */ __RPC__in REFCLSID rclsid,
            /* [in] */ __RPC__in_opt IUnknown *pUnkOuter,
            /* [in] */ DWORD dwClsContext,
            /* [in] */ __RPC__in REFIID riid,
            /* [iid_is][out] */ __RPC__deref_out_opt IUnknown **ppvObject);
        
        DECLSPEC_XFGVIRT(IRemoteDebugApplication, QueryAlive)
        HRESULT ( STDMETHODCALLTYPE *QueryAlive )( 
            __RPC__in IRemoteDebugApplication * This);
        
        DECLSPEC_XFGVIRT(IRemoteDebugApplication, EnumThreads)
        HRESULT ( STDMETHODCALLTYPE *EnumThreads )( 
            __RPC__in IRemoteDebugApplication * This,
            /* [out] */ __RPC__deref_out_opt IEnumRemoteDebugApplicationThreads **pperdat);
        
        DECLSPEC_XFGVIRT(IRemoteDebugApplication, GetName)
        HRESULT ( STDMETHODCALLTYPE *GetName )( 
            __RPC__in IRemoteDebugApplication * This,
            /* [out] */ __RPC__deref_out_opt BSTR *pbstrName);
        
        DECLSPEC_XFGVIRT(IRemoteDebugApplication, GetRootNode)
        HRESULT ( STDMETHODCALLTYPE *GetRootNode )( 
            __RPC__in IRemoteDebugApplication * This,
            /* [out] */ __RPC__deref_out_opt IDebugApplicationNode **ppdanRoot);
        
        DECLSPEC_XFGVIRT(IRemoteDebugApplication, EnumGlobalExpressionContexts)
        HRESULT ( STDMETHODCALLTYPE *EnumGlobalExpressionContexts )( 
            __RPC__in IRemoteDebugApplication * This,
            /* [out] */ __RPC__deref_out_opt IEnumDebugExpressionContexts **ppedec);
        
        END_INTERFACE
    } IRemoteDebugApplicationVtbl;

    interface IRemoteDebugApplication
    {
        CONST_VTBL struct IRemoteDebugApplicationVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IRemoteDebugApplication_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IRemoteDebugApplication_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IRemoteDebugApplication_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IRemoteDebugApplication_ResumeFromBreakPoint(This,prptFocus,bra,era)	\
    ( (This)->lpVtbl -> ResumeFromBreakPoint(This,prptFocus,bra,era) ) 

#define IRemoteDebugApplication_CauseBreak(This)	\
    ( (This)->lpVtbl -> CauseBreak(This) ) 

#define IRemoteDebugApplication_ConnectDebugger(This,pad)	\
    ( (This)->lpVtbl -> ConnectDebugger(This,pad) ) 

#define IRemoteDebugApplication_DisconnectDebugger(This)	\
    ( (This)->lpVtbl -> DisconnectDebugger(This) ) 

#define IRemoteDebugApplication_GetDebugger(This,pad)	\
    ( (This)->lpVtbl -> GetDebugger(This,pad) ) 

#define IRemoteDebugApplication_CreateInstanceAtApplication(This,rclsid,pUnkOuter,dwClsContext,riid,ppvObject)	\
    ( (This)->lpVtbl -> CreateInstanceAtApplication(This,rclsid,pUnkOuter,dwClsContext,riid,ppvObject) ) 

#define IRemoteDebugApplication_QueryAlive(This)	\
    ( (This)->lpVtbl -> QueryAlive(This) ) 

#define IRemoteDebugApplication_EnumThreads(This,pperdat)	\
    ( (This)->lpVtbl -> EnumThreads(This,pperdat) ) 

#define IRemoteDebugApplication_GetName(This,pbstrName)	\
    ( (This)->lpVtbl -> GetName(This,pbstrName) ) 

#define IRemoteDebugApplication_GetRootNode(This,ppdanRoot)	\
    ( (This)->lpVtbl -> GetRootNode(This,ppdanRoot) ) 

#define IRemoteDebugApplication_EnumGlobalExpressionContexts(This,ppedec)	\
    ( (This)->lpVtbl -> EnumGlobalExpressionContexts(This,ppedec) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IRemoteDebugApplication_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_activdbg_0000_0040 */
/* [local] */ 

#ifndef DISABLE_ACTIVDBG_INTERFACE_WRAPPERS
#ifdef _WIN64
#define IDebugApplication IDebugApplication64
#define IID_IDebugApplication IID_IDebugApplication64
#else
#define IDebugApplication IDebugApplication32
#define IID_IDebugApplication IID_IDebugApplication32
#endif
#endif


extern RPC_IF_HANDLE __MIDL_itf_activdbg_0000_0040_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_activdbg_0000_0040_v0_0_s_ifspec;

#ifndef __IDebugApplication32_INTERFACE_DEFINED__
#define __IDebugApplication32_INTERFACE_DEFINED__

/* interface IDebugApplication32 */
/* [local][unique][helpstring][uuid][object] */ 


EXTERN_C const IID IID_IDebugApplication32;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("51973C32-CB0C-11d0-B5C9-00A0244A0E7A")
    IDebugApplication32 : public IRemoteDebugApplication
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE SetName( 
            /* [in] */ LPCOLESTR pstrName) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE StepOutComplete( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE DebugOutput( 
            /* [in] */ LPCOLESTR pstr) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE StartDebugSession( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE HandleBreakPoint( 
            /* [in] */ BREAKREASON br,
            /* [out] */ BREAKRESUMEACTION *pbra) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Close( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetBreakFlags( 
            /* [out] */ APPBREAKFLAGS *pabf,
            /* [out] */ IRemoteDebugApplicationThread **pprdatSteppingThread) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetCurrentThread( 
            /* [out] */ IDebugApplicationThread **pat) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE CreateAsyncDebugOperation( 
            /* [in] */ IDebugSyncOperation *psdo,
            /* [out] */ IDebugAsyncOperation **ppado) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE AddStackFrameSniffer( 
            /* [in] */ IDebugStackFrameSniffer *pdsfs,
            /* [out] */ DWORD *pdwCookie) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE RemoveStackFrameSniffer( 
            /* [in] */ DWORD dwCookie) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE QueryCurrentThreadIsDebuggerThread( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SynchronousCallInDebuggerThread( 
            /* [in] */ IDebugThreadCall32 *pptc,
            /* [in] */ DWORD dwParam1,
            /* [in] */ DWORD dwParam2,
            /* [in] */ DWORD dwParam3) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE CreateApplicationNode( 
            /* [out] */ IDebugApplicationNode **ppdanNew) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE FireDebuggerEvent( 
            /* [in] */ REFGUID riid,
            /* [in] */ IUnknown *punk) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE HandleRuntimeError( 
            /* [in] */ IActiveScriptErrorDebug *pErrorDebug,
            /* [in] */ IActiveScriptSite *pScriptSite,
            /* [out] */ BREAKRESUMEACTION *pbra,
            /* [out] */ ERRORRESUMEACTION *perra,
            /* [out] */ BOOL *pfCallOnScriptError) = 0;
        
        virtual BOOL STDMETHODCALLTYPE FCanJitDebug( void) = 0;
        
        virtual BOOL STDMETHODCALLTYPE FIsAutoJitDebugEnabled( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE AddGlobalExpressionContextProvider( 
            /* [in] */ IProvideExpressionContexts *pdsfs,
            /* [out] */ DWORD *pdwCookie) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE RemoveGlobalExpressionContextProvider( 
            /* [in] */ DWORD dwCookie) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IDebugApplication32Vtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IDebugApplication32 * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IDebugApplication32 * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IDebugApplication32 * This);
        
        DECLSPEC_XFGVIRT(IRemoteDebugApplication, ResumeFromBreakPoint)
        HRESULT ( STDMETHODCALLTYPE *ResumeFromBreakPoint )( 
            IDebugApplication32 * This,
            /* [in] */ IRemoteDebugApplicationThread *prptFocus,
            /* [in] */ BREAKRESUMEACTION bra,
            /* [in] */ ERRORRESUMEACTION era);
        
        DECLSPEC_XFGVIRT(IRemoteDebugApplication, CauseBreak)
        HRESULT ( STDMETHODCALLTYPE *CauseBreak )( 
            IDebugApplication32 * This);
        
        DECLSPEC_XFGVIRT(IRemoteDebugApplication, ConnectDebugger)
        HRESULT ( STDMETHODCALLTYPE *ConnectDebugger )( 
            IDebugApplication32 * This,
            /* [in] */ IApplicationDebugger *pad);
        
        DECLSPEC_XFGVIRT(IRemoteDebugApplication, DisconnectDebugger)
        HRESULT ( STDMETHODCALLTYPE *DisconnectDebugger )( 
            IDebugApplication32 * This);
        
        DECLSPEC_XFGVIRT(IRemoteDebugApplication, GetDebugger)
        HRESULT ( STDMETHODCALLTYPE *GetDebugger )( 
            IDebugApplication32 * This,
            /* [out] */ IApplicationDebugger **pad);
        
        DECLSPEC_XFGVIRT(IRemoteDebugApplication, CreateInstanceAtApplication)
        HRESULT ( STDMETHODCALLTYPE *CreateInstanceAtApplication )( 
            IDebugApplication32 * This,
            /* [in] */ REFCLSID rclsid,
            /* [in] */ IUnknown *pUnkOuter,
            /* [in] */ DWORD dwClsContext,
            /* [in] */ REFIID riid,
            /* [iid_is][out] */ IUnknown **ppvObject);
        
        DECLSPEC_XFGVIRT(IRemoteDebugApplication, QueryAlive)
        HRESULT ( STDMETHODCALLTYPE *QueryAlive )( 
            IDebugApplication32 * This);
        
        DECLSPEC_XFGVIRT(IRemoteDebugApplication, EnumThreads)
        HRESULT ( STDMETHODCALLTYPE *EnumThreads )( 
            IDebugApplication32 * This,
            /* [out] */ IEnumRemoteDebugApplicationThreads **pperdat);
        
        DECLSPEC_XFGVIRT(IRemoteDebugApplication, GetName)
        HRESULT ( STDMETHODCALLTYPE *GetName )( 
            IDebugApplication32 * This,
            /* [out] */ BSTR *pbstrName);
        
        DECLSPEC_XFGVIRT(IRemoteDebugApplication, GetRootNode)
        HRESULT ( STDMETHODCALLTYPE *GetRootNode )( 
            IDebugApplication32 * This,
            /* [out] */ IDebugApplicationNode **ppdanRoot);
        
        DECLSPEC_XFGVIRT(IRemoteDebugApplication, EnumGlobalExpressionContexts)
        HRESULT ( STDMETHODCALLTYPE *EnumGlobalExpressionContexts )( 
            IDebugApplication32 * This,
            /* [out] */ IEnumDebugExpressionContexts **ppedec);
        
        DECLSPEC_XFGVIRT(IDebugApplication32, SetName)
        HRESULT ( STDMETHODCALLTYPE *SetName )( 
            IDebugApplication32 * This,
            /* [in] */ LPCOLESTR pstrName);
        
        DECLSPEC_XFGVIRT(IDebugApplication32, StepOutComplete)
        HRESULT ( STDMETHODCALLTYPE *StepOutComplete )( 
            IDebugApplication32 * This);
        
        DECLSPEC_XFGVIRT(IDebugApplication32, DebugOutput)
        HRESULT ( STDMETHODCALLTYPE *DebugOutput )( 
            IDebugApplication32 * This,
            /* [in] */ LPCOLESTR pstr);
        
        DECLSPEC_XFGVIRT(IDebugApplication32, StartDebugSession)
        HRESULT ( STDMETHODCALLTYPE *StartDebugSession )( 
            IDebugApplication32 * This);
        
        DECLSPEC_XFGVIRT(IDebugApplication32, HandleBreakPoint)
        HRESULT ( STDMETHODCALLTYPE *HandleBreakPoint )( 
            IDebugApplication32 * This,
            /* [in] */ BREAKREASON br,
            /* [out] */ BREAKRESUMEACTION *pbra);
        
        DECLSPEC_XFGVIRT(IDebugApplication32, Close)
        HRESULT ( STDMETHODCALLTYPE *Close )( 
            IDebugApplication32 * This);
        
        DECLSPEC_XFGVIRT(IDebugApplication32, GetBreakFlags)
        HRESULT ( STDMETHODCALLTYPE *GetBreakFlags )( 
            IDebugApplication32 * This,
            /* [out] */ APPBREAKFLAGS *pabf,
            /* [out] */ IRemoteDebugApplicationThread **pprdatSteppingThread);
        
        DECLSPEC_XFGVIRT(IDebugApplication32, GetCurrentThread)
        HRESULT ( STDMETHODCALLTYPE *GetCurrentThread )( 
            IDebugApplication32 * This,
            /* [out] */ IDebugApplicationThread **pat);
        
        DECLSPEC_XFGVIRT(IDebugApplication32, CreateAsyncDebugOperation)
        HRESULT ( STDMETHODCALLTYPE *CreateAsyncDebugOperation )( 
            IDebugApplication32 * This,
            /* [in] */ IDebugSyncOperation *psdo,
            /* [out] */ IDebugAsyncOperation **ppado);
        
        DECLSPEC_XFGVIRT(IDebugApplication32, AddStackFrameSniffer)
        HRESULT ( STDMETHODCALLTYPE *AddStackFrameSniffer )( 
            IDebugApplication32 * This,
            /* [in] */ IDebugStackFrameSniffer *pdsfs,
            /* [out] */ DWORD *pdwCookie);
        
        DECLSPEC_XFGVIRT(IDebugApplication32, RemoveStackFrameSniffer)
        HRESULT ( STDMETHODCALLTYPE *RemoveStackFrameSniffer )( 
            IDebugApplication32 * This,
            /* [in] */ DWORD dwCookie);
        
        DECLSPEC_XFGVIRT(IDebugApplication32, QueryCurrentThreadIsDebuggerThread)
        HRESULT ( STDMETHODCALLTYPE *QueryCurrentThreadIsDebuggerThread )( 
            IDebugApplication32 * This);
        
        DECLSPEC_XFGVIRT(IDebugApplication32, SynchronousCallInDebuggerThread)
        HRESULT ( STDMETHODCALLTYPE *SynchronousCallInDebuggerThread )( 
            IDebugApplication32 * This,
            /* [in] */ IDebugThreadCall32 *pptc,
            /* [in] */ DWORD dwParam1,
            /* [in] */ DWORD dwParam2,
            /* [in] */ DWORD dwParam3);
        
        DECLSPEC_XFGVIRT(IDebugApplication32, CreateApplicationNode)
        HRESULT ( STDMETHODCALLTYPE *CreateApplicationNode )( 
            IDebugApplication32 * This,
            /* [out] */ IDebugApplicationNode **ppdanNew);
        
        DECLSPEC_XFGVIRT(IDebugApplication32, FireDebuggerEvent)
        HRESULT ( STDMETHODCALLTYPE *FireDebuggerEvent )( 
            IDebugApplication32 * This,
            /* [in] */ REFGUID riid,
            /* [in] */ IUnknown *punk);
        
        DECLSPEC_XFGVIRT(IDebugApplication32, HandleRuntimeError)
        HRESULT ( STDMETHODCALLTYPE *HandleRuntimeError )( 
            IDebugApplication32 * This,
            /* [in] */ IActiveScriptErrorDebug *pErrorDebug,
            /* [in] */ IActiveScriptSite *pScriptSite,
            /* [out] */ BREAKRESUMEACTION *pbra,
            /* [out] */ ERRORRESUMEACTION *perra,
            /* [out] */ BOOL *pfCallOnScriptError);
        
        DECLSPEC_XFGVIRT(IDebugApplication32, FCanJitDebug)
        BOOL ( STDMETHODCALLTYPE *FCanJitDebug )( 
            IDebugApplication32 * This);
        
        DECLSPEC_XFGVIRT(IDebugApplication32, FIsAutoJitDebugEnabled)
        BOOL ( STDMETHODCALLTYPE *FIsAutoJitDebugEnabled )( 
            IDebugApplication32 * This);
        
        DECLSPEC_XFGVIRT(IDebugApplication32, AddGlobalExpressionContextProvider)
        HRESULT ( STDMETHODCALLTYPE *AddGlobalExpressionContextProvider )( 
            IDebugApplication32 * This,
            /* [in] */ IProvideExpressionContexts *pdsfs,
            /* [out] */ DWORD *pdwCookie);
        
        DECLSPEC_XFGVIRT(IDebugApplication32, RemoveGlobalExpressionContextProvider)
        HRESULT ( STDMETHODCALLTYPE *RemoveGlobalExpressionContextProvider )( 
            IDebugApplication32 * This,
            /* [in] */ DWORD dwCookie);
        
        END_INTERFACE
    } IDebugApplication32Vtbl;

    interface IDebugApplication32
    {
        CONST_VTBL struct IDebugApplication32Vtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IDebugApplication32_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IDebugApplication32_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IDebugApplication32_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IDebugApplication32_ResumeFromBreakPoint(This,prptFocus,bra,era)	\
    ( (This)->lpVtbl -> ResumeFromBreakPoint(This,prptFocus,bra,era) ) 

#define IDebugApplication32_CauseBreak(This)	\
    ( (This)->lpVtbl -> CauseBreak(This) ) 

#define IDebugApplication32_ConnectDebugger(This,pad)	\
    ( (This)->lpVtbl -> ConnectDebugger(This,pad) ) 

#define IDebugApplication32_DisconnectDebugger(This)	\
    ( (This)->lpVtbl -> DisconnectDebugger(This) ) 

#define IDebugApplication32_GetDebugger(This,pad)	\
    ( (This)->lpVtbl -> GetDebugger(This,pad) ) 

#define IDebugApplication32_CreateInstanceAtApplication(This,rclsid,pUnkOuter,dwClsContext,riid,ppvObject)	\
    ( (This)->lpVtbl -> CreateInstanceAtApplication(This,rclsid,pUnkOuter,dwClsContext,riid,ppvObject) ) 

#define IDebugApplication32_QueryAlive(This)	\
    ( (This)->lpVtbl -> QueryAlive(This) ) 

#define IDebugApplication32_EnumThreads(This,pperdat)	\
    ( (This)->lpVtbl -> EnumThreads(This,pperdat) ) 

#define IDebugApplication32_GetName(This,pbstrName)	\
    ( (This)->lpVtbl -> GetName(This,pbstrName) ) 

#define IDebugApplication32_GetRootNode(This,ppdanRoot)	\
    ( (This)->lpVtbl -> GetRootNode(This,ppdanRoot) ) 

#define IDebugApplication32_EnumGlobalExpressionContexts(This,ppedec)	\
    ( (This)->lpVtbl -> EnumGlobalExpressionContexts(This,ppedec) ) 


#define IDebugApplication32_SetName(This,pstrName)	\
    ( (This)->lpVtbl -> SetName(This,pstrName) ) 

#define IDebugApplication32_StepOutComplete(This)	\
    ( (This)->lpVtbl -> StepOutComplete(This) ) 

#define IDebugApplication32_DebugOutput(This,pstr)	\
    ( (This)->lpVtbl -> DebugOutput(This,pstr) ) 

#define IDebugApplication32_StartDebugSession(This)	\
    ( (This)->lpVtbl -> StartDebugSession(This) ) 

#define IDebugApplication32_HandleBreakPoint(This,br,pbra)	\
    ( (This)->lpVtbl -> HandleBreakPoint(This,br,pbra) ) 

#define IDebugApplication32_Close(This)	\
    ( (This)->lpVtbl -> Close(This) ) 

#define IDebugApplication32_GetBreakFlags(This,pabf,pprdatSteppingThread)	\
    ( (This)->lpVtbl -> GetBreakFlags(This,pabf,pprdatSteppingThread) ) 

#define IDebugApplication32_GetCurrentThread(This,pat)	\
    ( (This)->lpVtbl -> GetCurrentThread(This,pat) ) 

#define IDebugApplication32_CreateAsyncDebugOperation(This,psdo,ppado)	\
    ( (This)->lpVtbl -> CreateAsyncDebugOperation(This,psdo,ppado) ) 

#define IDebugApplication32_AddStackFrameSniffer(This,pdsfs,pdwCookie)	\
    ( (This)->lpVtbl -> AddStackFrameSniffer(This,pdsfs,pdwCookie) ) 

#define IDebugApplication32_RemoveStackFrameSniffer(This,dwCookie)	\
    ( (This)->lpVtbl -> RemoveStackFrameSniffer(This,dwCookie) ) 

#define IDebugApplication32_QueryCurrentThreadIsDebuggerThread(This)	\
    ( (This)->lpVtbl -> QueryCurrentThreadIsDebuggerThread(This) ) 

#define IDebugApplication32_SynchronousCallInDebuggerThread(This,pptc,dwParam1,dwParam2,dwParam3)	\
    ( (This)->lpVtbl -> SynchronousCallInDebuggerThread(This,pptc,dwParam1,dwParam2,dwParam3) ) 

#define IDebugApplication32_CreateApplicationNode(This,ppdanNew)	\
    ( (This)->lpVtbl -> CreateApplicationNode(This,ppdanNew) ) 

#define IDebugApplication32_FireDebuggerEvent(This,riid,punk)	\
    ( (This)->lpVtbl -> FireDebuggerEvent(This,riid,punk) ) 

#define IDebugApplication32_HandleRuntimeError(This,pErrorDebug,pScriptSite,pbra,perra,pfCallOnScriptError)	\
    ( (This)->lpVtbl -> HandleRuntimeError(This,pErrorDebug,pScriptSite,pbra,perra,pfCallOnScriptError) ) 

#define IDebugApplication32_FCanJitDebug(This)	\
    ( (This)->lpVtbl -> FCanJitDebug(This) ) 

#define IDebugApplication32_FIsAutoJitDebugEnabled(This)	\
    ( (This)->lpVtbl -> FIsAutoJitDebugEnabled(This) ) 

#define IDebugApplication32_AddGlobalExpressionContextProvider(This,pdsfs,pdwCookie)	\
    ( (This)->lpVtbl -> AddGlobalExpressionContextProvider(This,pdsfs,pdwCookie) ) 

#define IDebugApplication32_RemoveGlobalExpressionContextProvider(This,dwCookie)	\
    ( (This)->lpVtbl -> RemoveGlobalExpressionContextProvider(This,dwCookie) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IDebugApplication32_INTERFACE_DEFINED__ */


#ifndef __IDebugApplication64_INTERFACE_DEFINED__
#define __IDebugApplication64_INTERFACE_DEFINED__

/* interface IDebugApplication64 */
/* [local][unique][helpstring][uuid][object] */ 


EXTERN_C const IID IID_IDebugApplication64;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("4dedc754-04c7-4f10-9e60-16a390fe6e62")
    IDebugApplication64 : public IRemoteDebugApplication
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE SetName( 
            /* [in] */ LPCOLESTR pstrName) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE StepOutComplete( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE DebugOutput( 
            /* [in] */ LPCOLESTR pstr) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE StartDebugSession( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE HandleBreakPoint( 
            /* [in] */ BREAKREASON br,
            /* [out] */ BREAKRESUMEACTION *pbra) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Close( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetBreakFlags( 
            /* [out] */ APPBREAKFLAGS *pabf,
            /* [out] */ IRemoteDebugApplicationThread **pprdatSteppingThread) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetCurrentThread( 
            /* [out] */ IDebugApplicationThread **pat) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE CreateAsyncDebugOperation( 
            /* [in] */ IDebugSyncOperation *psdo,
            /* [out] */ IDebugAsyncOperation **ppado) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE AddStackFrameSniffer( 
            /* [in] */ IDebugStackFrameSniffer *pdsfs,
            /* [out] */ DWORD *pdwCookie) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE RemoveStackFrameSniffer( 
            /* [in] */ DWORD dwCookie) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE QueryCurrentThreadIsDebuggerThread( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SynchronousCallInDebuggerThread( 
            /* [in] */ IDebugThreadCall64 *pptc,
            /* [in] */ DWORDLONG dwParam1,
            /* [in] */ DWORDLONG dwParam2,
            /* [in] */ DWORDLONG dwParam3) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE CreateApplicationNode( 
            /* [out] */ IDebugApplicationNode **ppdanNew) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE FireDebuggerEvent( 
            /* [in] */ REFGUID riid,
            /* [in] */ IUnknown *punk) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE HandleRuntimeError( 
            /* [in] */ IActiveScriptErrorDebug *pErrorDebug,
            /* [in] */ IActiveScriptSite *pScriptSite,
            /* [out] */ BREAKRESUMEACTION *pbra,
            /* [out] */ ERRORRESUMEACTION *perra,
            /* [out] */ BOOL *pfCallOnScriptError) = 0;
        
        virtual BOOL STDMETHODCALLTYPE FCanJitDebug( void) = 0;
        
        virtual BOOL STDMETHODCALLTYPE FIsAutoJitDebugEnabled( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE AddGlobalExpressionContextProvider( 
            /* [in] */ IProvideExpressionContexts *pdsfs,
            /* [out] */ DWORDLONG *pdwCookie) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE RemoveGlobalExpressionContextProvider( 
            /* [in] */ DWORDLONG dwCookie) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IDebugApplication64Vtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IDebugApplication64 * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IDebugApplication64 * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IDebugApplication64 * This);
        
        DECLSPEC_XFGVIRT(IRemoteDebugApplication, ResumeFromBreakPoint)
        HRESULT ( STDMETHODCALLTYPE *ResumeFromBreakPoint )( 
            IDebugApplication64 * This,
            /* [in] */ IRemoteDebugApplicationThread *prptFocus,
            /* [in] */ BREAKRESUMEACTION bra,
            /* [in] */ ERRORRESUMEACTION era);
        
        DECLSPEC_XFGVIRT(IRemoteDebugApplication, CauseBreak)
        HRESULT ( STDMETHODCALLTYPE *CauseBreak )( 
            IDebugApplication64 * This);
        
        DECLSPEC_XFGVIRT(IRemoteDebugApplication, ConnectDebugger)
        HRESULT ( STDMETHODCALLTYPE *ConnectDebugger )( 
            IDebugApplication64 * This,
            /* [in] */ IApplicationDebugger *pad);
        
        DECLSPEC_XFGVIRT(IRemoteDebugApplication, DisconnectDebugger)
        HRESULT ( STDMETHODCALLTYPE *DisconnectDebugger )( 
            IDebugApplication64 * This);
        
        DECLSPEC_XFGVIRT(IRemoteDebugApplication, GetDebugger)
        HRESULT ( STDMETHODCALLTYPE *GetDebugger )( 
            IDebugApplication64 * This,
            /* [out] */ IApplicationDebugger **pad);
        
        DECLSPEC_XFGVIRT(IRemoteDebugApplication, CreateInstanceAtApplication)
        HRESULT ( STDMETHODCALLTYPE *CreateInstanceAtApplication )( 
            IDebugApplication64 * This,
            /* [in] */ REFCLSID rclsid,
            /* [in] */ IUnknown *pUnkOuter,
            /* [in] */ DWORD dwClsContext,
            /* [in] */ REFIID riid,
            /* [iid_is][out] */ IUnknown **ppvObject);
        
        DECLSPEC_XFGVIRT(IRemoteDebugApplication, QueryAlive)
        HRESULT ( STDMETHODCALLTYPE *QueryAlive )( 
            IDebugApplication64 * This);
        
        DECLSPEC_XFGVIRT(IRemoteDebugApplication, EnumThreads)
        HRESULT ( STDMETHODCALLTYPE *EnumThreads )( 
            IDebugApplication64 * This,
            /* [out] */ IEnumRemoteDebugApplicationThreads **pperdat);
        
        DECLSPEC_XFGVIRT(IRemoteDebugApplication, GetName)
        HRESULT ( STDMETHODCALLTYPE *GetName )( 
            IDebugApplication64 * This,
            /* [out] */ BSTR *pbstrName);
        
        DECLSPEC_XFGVIRT(IRemoteDebugApplication, GetRootNode)
        HRESULT ( STDMETHODCALLTYPE *GetRootNode )( 
            IDebugApplication64 * This,
            /* [out] */ IDebugApplicationNode **ppdanRoot);
        
        DECLSPEC_XFGVIRT(IRemoteDebugApplication, EnumGlobalExpressionContexts)
        HRESULT ( STDMETHODCALLTYPE *EnumGlobalExpressionContexts )( 
            IDebugApplication64 * This,
            /* [out] */ IEnumDebugExpressionContexts **ppedec);
        
        DECLSPEC_XFGVIRT(IDebugApplication64, SetName)
        HRESULT ( STDMETHODCALLTYPE *SetName )( 
            IDebugApplication64 * This,
            /* [in] */ LPCOLESTR pstrName);
        
        DECLSPEC_XFGVIRT(IDebugApplication64, StepOutComplete)
        HRESULT ( STDMETHODCALLTYPE *StepOutComplete )( 
            IDebugApplication64 * This);
        
        DECLSPEC_XFGVIRT(IDebugApplication64, DebugOutput)
        HRESULT ( STDMETHODCALLTYPE *DebugOutput )( 
            IDebugApplication64 * This,
            /* [in] */ LPCOLESTR pstr);
        
        DECLSPEC_XFGVIRT(IDebugApplication64, StartDebugSession)
        HRESULT ( STDMETHODCALLTYPE *StartDebugSession )( 
            IDebugApplication64 * This);
        
        DECLSPEC_XFGVIRT(IDebugApplication64, HandleBreakPoint)
        HRESULT ( STDMETHODCALLTYPE *HandleBreakPoint )( 
            IDebugApplication64 * This,
            /* [in] */ BREAKREASON br,
            /* [out] */ BREAKRESUMEACTION *pbra);
        
        DECLSPEC_XFGVIRT(IDebugApplication64, Close)
        HRESULT ( STDMETHODCALLTYPE *Close )( 
            IDebugApplication64 * This);
        
        DECLSPEC_XFGVIRT(IDebugApplication64, GetBreakFlags)
        HRESULT ( STDMETHODCALLTYPE *GetBreakFlags )( 
            IDebugApplication64 * This,
            /* [out] */ APPBREAKFLAGS *pabf,
            /* [out] */ IRemoteDebugApplicationThread **pprdatSteppingThread);
        
        DECLSPEC_XFGVIRT(IDebugApplication64, GetCurrentThread)
        HRESULT ( STDMETHODCALLTYPE *GetCurrentThread )( 
            IDebugApplication64 * This,
            /* [out] */ IDebugApplicationThread **pat);
        
        DECLSPEC_XFGVIRT(IDebugApplication64, CreateAsyncDebugOperation)
        HRESULT ( STDMETHODCALLTYPE *CreateAsyncDebugOperation )( 
            IDebugApplication64 * This,
            /* [in] */ IDebugSyncOperation *psdo,
            /* [out] */ IDebugAsyncOperation **ppado);
        
        DECLSPEC_XFGVIRT(IDebugApplication64, AddStackFrameSniffer)
        HRESULT ( STDMETHODCALLTYPE *AddStackFrameSniffer )( 
            IDebugApplication64 * This,
            /* [in] */ IDebugStackFrameSniffer *pdsfs,
            /* [out] */ DWORD *pdwCookie);
        
        DECLSPEC_XFGVIRT(IDebugApplication64, RemoveStackFrameSniffer)
        HRESULT ( STDMETHODCALLTYPE *RemoveStackFrameSniffer )( 
            IDebugApplication64 * This,
            /* [in] */ DWORD dwCookie);
        
        DECLSPEC_XFGVIRT(IDebugApplication64, QueryCurrentThreadIsDebuggerThread)
        HRESULT ( STDMETHODCALLTYPE *QueryCurrentThreadIsDebuggerThread )( 
            IDebugApplication64 * This);
        
        DECLSPEC_XFGVIRT(IDebugApplication64, SynchronousCallInDebuggerThread)
        HRESULT ( STDMETHODCALLTYPE *SynchronousCallInDebuggerThread )( 
            IDebugApplication64 * This,
            /* [in] */ IDebugThreadCall64 *pptc,
            /* [in] */ DWORDLONG dwParam1,
            /* [in] */ DWORDLONG dwParam2,
            /* [in] */ DWORDLONG dwParam3);
        
        DECLSPEC_XFGVIRT(IDebugApplication64, CreateApplicationNode)
        HRESULT ( STDMETHODCALLTYPE *CreateApplicationNode )( 
            IDebugApplication64 * This,
            /* [out] */ IDebugApplicationNode **ppdanNew);
        
        DECLSPEC_XFGVIRT(IDebugApplication64, FireDebuggerEvent)
        HRESULT ( STDMETHODCALLTYPE *FireDebuggerEvent )( 
            IDebugApplication64 * This,
            /* [in] */ REFGUID riid,
            /* [in] */ IUnknown *punk);
        
        DECLSPEC_XFGVIRT(IDebugApplication64, HandleRuntimeError)
        HRESULT ( STDMETHODCALLTYPE *HandleRuntimeError )( 
            IDebugApplication64 * This,
            /* [in] */ IActiveScriptErrorDebug *pErrorDebug,
            /* [in] */ IActiveScriptSite *pScriptSite,
            /* [out] */ BREAKRESUMEACTION *pbra,
            /* [out] */ ERRORRESUMEACTION *perra,
            /* [out] */ BOOL *pfCallOnScriptError);
        
        DECLSPEC_XFGVIRT(IDebugApplication64, FCanJitDebug)
        BOOL ( STDMETHODCALLTYPE *FCanJitDebug )( 
            IDebugApplication64 * This);
        
        DECLSPEC_XFGVIRT(IDebugApplication64, FIsAutoJitDebugEnabled)
        BOOL ( STDMETHODCALLTYPE *FIsAutoJitDebugEnabled )( 
            IDebugApplication64 * This);
        
        DECLSPEC_XFGVIRT(IDebugApplication64, AddGlobalExpressionContextProvider)
        HRESULT ( STDMETHODCALLTYPE *AddGlobalExpressionContextProvider )( 
            IDebugApplication64 * This,
            /* [in] */ IProvideExpressionContexts *pdsfs,
            /* [out] */ DWORDLONG *pdwCookie);
        
        DECLSPEC_XFGVIRT(IDebugApplication64, RemoveGlobalExpressionContextProvider)
        HRESULT ( STDMETHODCALLTYPE *RemoveGlobalExpressionContextProvider )( 
            IDebugApplication64 * This,
            /* [in] */ DWORDLONG dwCookie);
        
        END_INTERFACE
    } IDebugApplication64Vtbl;

    interface IDebugApplication64
    {
        CONST_VTBL struct IDebugApplication64Vtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IDebugApplication64_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IDebugApplication64_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IDebugApplication64_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IDebugApplication64_ResumeFromBreakPoint(This,prptFocus,bra,era)	\
    ( (This)->lpVtbl -> ResumeFromBreakPoint(This,prptFocus,bra,era) ) 

#define IDebugApplication64_CauseBreak(This)	\
    ( (This)->lpVtbl -> CauseBreak(This) ) 

#define IDebugApplication64_ConnectDebugger(This,pad)	\
    ( (This)->lpVtbl -> ConnectDebugger(This,pad) ) 

#define IDebugApplication64_DisconnectDebugger(This)	\
    ( (This)->lpVtbl -> DisconnectDebugger(This) ) 

#define IDebugApplication64_GetDebugger(This,pad)	\
    ( (This)->lpVtbl -> GetDebugger(This,pad) ) 

#define IDebugApplication64_CreateInstanceAtApplication(This,rclsid,pUnkOuter,dwClsContext,riid,ppvObject)	\
    ( (This)->lpVtbl -> CreateInstanceAtApplication(This,rclsid,pUnkOuter,dwClsContext,riid,ppvObject) ) 

#define IDebugApplication64_QueryAlive(This)	\
    ( (This)->lpVtbl -> QueryAlive(This) ) 

#define IDebugApplication64_EnumThreads(This,pperdat)	\
    ( (This)->lpVtbl -> EnumThreads(This,pperdat) ) 

#define IDebugApplication64_GetName(This,pbstrName)	\
    ( (This)->lpVtbl -> GetName(This,pbstrName) ) 

#define IDebugApplication64_GetRootNode(This,ppdanRoot)	\
    ( (This)->lpVtbl -> GetRootNode(This,ppdanRoot) ) 

#define IDebugApplication64_EnumGlobalExpressionContexts(This,ppedec)	\
    ( (This)->lpVtbl -> EnumGlobalExpressionContexts(This,ppedec) ) 


#define IDebugApplication64_SetName(This,pstrName)	\
    ( (This)->lpVtbl -> SetName(This,pstrName) ) 

#define IDebugApplication64_StepOutComplete(This)	\
    ( (This)->lpVtbl -> StepOutComplete(This) ) 

#define IDebugApplication64_DebugOutput(This,pstr)	\
    ( (This)->lpVtbl -> DebugOutput(This,pstr) ) 

#define IDebugApplication64_StartDebugSession(This)	\
    ( (This)->lpVtbl -> StartDebugSession(This) ) 

#define IDebugApplication64_HandleBreakPoint(This,br,pbra)	\
    ( (This)->lpVtbl -> HandleBreakPoint(This,br,pbra) ) 

#define IDebugApplication64_Close(This)	\
    ( (This)->lpVtbl -> Close(This) ) 

#define IDebugApplication64_GetBreakFlags(This,pabf,pprdatSteppingThread)	\
    ( (This)->lpVtbl -> GetBreakFlags(This,pabf,pprdatSteppingThread) ) 

#define IDebugApplication64_GetCurrentThread(This,pat)	\
    ( (This)->lpVtbl -> GetCurrentThread(This,pat) ) 

#define IDebugApplication64_CreateAsyncDebugOperation(This,psdo,ppado)	\
    ( (This)->lpVtbl -> CreateAsyncDebugOperation(This,psdo,ppado) ) 

#define IDebugApplication64_AddStackFrameSniffer(This,pdsfs,pdwCookie)	\
    ( (This)->lpVtbl -> AddStackFrameSniffer(This,pdsfs,pdwCookie) ) 

#define IDebugApplication64_RemoveStackFrameSniffer(This,dwCookie)	\
    ( (This)->lpVtbl -> RemoveStackFrameSniffer(This,dwCookie) ) 

#define IDebugApplication64_QueryCurrentThreadIsDebuggerThread(This)	\
    ( (This)->lpVtbl -> QueryCurrentThreadIsDebuggerThread(This) ) 

#define IDebugApplication64_SynchronousCallInDebuggerThread(This,pptc,dwParam1,dwParam2,dwParam3)	\
    ( (This)->lpVtbl -> SynchronousCallInDebuggerThread(This,pptc,dwParam1,dwParam2,dwParam3) ) 

#define IDebugApplication64_CreateApplicationNode(This,ppdanNew)	\
    ( (This)->lpVtbl -> CreateApplicationNode(This,ppdanNew) ) 

#define IDebugApplication64_FireDebuggerEvent(This,riid,punk)	\
    ( (This)->lpVtbl -> FireDebuggerEvent(This,riid,punk) ) 

#define IDebugApplication64_HandleRuntimeError(This,pErrorDebug,pScriptSite,pbra,perra,pfCallOnScriptError)	\
    ( (This)->lpVtbl -> HandleRuntimeError(This,pErrorDebug,pScriptSite,pbra,perra,pfCallOnScriptError) ) 

#define IDebugApplication64_FCanJitDebug(This)	\
    ( (This)->lpVtbl -> FCanJitDebug(This) ) 

#define IDebugApplication64_FIsAutoJitDebugEnabled(This)	\
    ( (This)->lpVtbl -> FIsAutoJitDebugEnabled(This) ) 

#define IDebugApplication64_AddGlobalExpressionContextProvider(This,pdsfs,pdwCookie)	\
    ( (This)->lpVtbl -> AddGlobalExpressionContextProvider(This,pdsfs,pdwCookie) ) 

#define IDebugApplication64_RemoveGlobalExpressionContextProvider(This,dwCookie)	\
    ( (This)->lpVtbl -> RemoveGlobalExpressionContextProvider(This,dwCookie) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IDebugApplication64_INTERFACE_DEFINED__ */


#ifndef __IRemoteDebugApplicationEvents_INTERFACE_DEFINED__
#define __IRemoteDebugApplicationEvents_INTERFACE_DEFINED__

/* interface IRemoteDebugApplicationEvents */
/* [unique][helpstring][uuid][object] */ 


EXTERN_C const IID IID_IRemoteDebugApplicationEvents;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("51973C33-CB0C-11d0-B5C9-00A0244A0E7A")
    IRemoteDebugApplicationEvents : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE OnConnectDebugger( 
            /* [in] */ __RPC__in_opt IApplicationDebugger *pad) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE OnDisconnectDebugger( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE OnSetName( 
            /* [in] */ __RPC__in LPCOLESTR pstrName) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE OnDebugOutput( 
            /* [in] */ __RPC__in LPCOLESTR pstr) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE OnClose( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE OnEnterBreakPoint( 
            /* [in] */ __RPC__in_opt IRemoteDebugApplicationThread *prdat) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE OnLeaveBreakPoint( 
            /* [in] */ __RPC__in_opt IRemoteDebugApplicationThread *prdat) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE OnCreateThread( 
            /* [in] */ __RPC__in_opt IRemoteDebugApplicationThread *prdat) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE OnDestroyThread( 
            /* [in] */ __RPC__in_opt IRemoteDebugApplicationThread *prdat) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE OnBreakFlagChange( 
            /* [in] */ APPBREAKFLAGS abf,
            /* [in] */ __RPC__in_opt IRemoteDebugApplicationThread *prdatSteppingThread) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IRemoteDebugApplicationEventsVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IRemoteDebugApplicationEvents * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IRemoteDebugApplicationEvents * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IRemoteDebugApplicationEvents * This);
        
        DECLSPEC_XFGVIRT(IRemoteDebugApplicationEvents, OnConnectDebugger)
        HRESULT ( STDMETHODCALLTYPE *OnConnectDebugger )( 
            __RPC__in IRemoteDebugApplicationEvents * This,
            /* [in] */ __RPC__in_opt IApplicationDebugger *pad);
        
        DECLSPEC_XFGVIRT(IRemoteDebugApplicationEvents, OnDisconnectDebugger)
        HRESULT ( STDMETHODCALLTYPE *OnDisconnectDebugger )( 
            __RPC__in IRemoteDebugApplicationEvents * This);
        
        DECLSPEC_XFGVIRT(IRemoteDebugApplicationEvents, OnSetName)
        HRESULT ( STDMETHODCALLTYPE *OnSetName )( 
            __RPC__in IRemoteDebugApplicationEvents * This,
            /* [in] */ __RPC__in LPCOLESTR pstrName);
        
        DECLSPEC_XFGVIRT(IRemoteDebugApplicationEvents, OnDebugOutput)
        HRESULT ( STDMETHODCALLTYPE *OnDebugOutput )( 
            __RPC__in IRemoteDebugApplicationEvents * This,
            /* [in] */ __RPC__in LPCOLESTR pstr);
        
        DECLSPEC_XFGVIRT(IRemoteDebugApplicationEvents, OnClose)
        HRESULT ( STDMETHODCALLTYPE *OnClose )( 
            __RPC__in IRemoteDebugApplicationEvents * This);
        
        DECLSPEC_XFGVIRT(IRemoteDebugApplicationEvents, OnEnterBreakPoint)
        HRESULT ( STDMETHODCALLTYPE *OnEnterBreakPoint )( 
            __RPC__in IRemoteDebugApplicationEvents * This,
            /* [in] */ __RPC__in_opt IRemoteDebugApplicationThread *prdat);
        
        DECLSPEC_XFGVIRT(IRemoteDebugApplicationEvents, OnLeaveBreakPoint)
        HRESULT ( STDMETHODCALLTYPE *OnLeaveBreakPoint )( 
            __RPC__in IRemoteDebugApplicationEvents * This,
            /* [in] */ __RPC__in_opt IRemoteDebugApplicationThread *prdat);
        
        DECLSPEC_XFGVIRT(IRemoteDebugApplicationEvents, OnCreateThread)
        HRESULT ( STDMETHODCALLTYPE *OnCreateThread )( 
            __RPC__in IRemoteDebugApplicationEvents * This,
            /* [in] */ __RPC__in_opt IRemoteDebugApplicationThread *prdat);
        
        DECLSPEC_XFGVIRT(IRemoteDebugApplicationEvents, OnDestroyThread)
        HRESULT ( STDMETHODCALLTYPE *OnDestroyThread )( 
            __RPC__in IRemoteDebugApplicationEvents * This,
            /* [in] */ __RPC__in_opt IRemoteDebugApplicationThread *prdat);
        
        DECLSPEC_XFGVIRT(IRemoteDebugApplicationEvents, OnBreakFlagChange)
        HRESULT ( STDMETHODCALLTYPE *OnBreakFlagChange )( 
            __RPC__in IRemoteDebugApplicationEvents * This,
            /* [in] */ APPBREAKFLAGS abf,
            /* [in] */ __RPC__in_opt IRemoteDebugApplicationThread *prdatSteppingThread);
        
        END_INTERFACE
    } IRemoteDebugApplicationEventsVtbl;

    interface IRemoteDebugApplicationEvents
    {
        CONST_VTBL struct IRemoteDebugApplicationEventsVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IRemoteDebugApplicationEvents_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IRemoteDebugApplicationEvents_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IRemoteDebugApplicationEvents_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IRemoteDebugApplicationEvents_OnConnectDebugger(This,pad)	\
    ( (This)->lpVtbl -> OnConnectDebugger(This,pad) ) 

#define IRemoteDebugApplicationEvents_OnDisconnectDebugger(This)	\
    ( (This)->lpVtbl -> OnDisconnectDebugger(This) ) 

#define IRemoteDebugApplicationEvents_OnSetName(This,pstrName)	\
    ( (This)->lpVtbl -> OnSetName(This,pstrName) ) 

#define IRemoteDebugApplicationEvents_OnDebugOutput(This,pstr)	\
    ( (This)->lpVtbl -> OnDebugOutput(This,pstr) ) 

#define IRemoteDebugApplicationEvents_OnClose(This)	\
    ( (This)->lpVtbl -> OnClose(This) ) 

#define IRemoteDebugApplicationEvents_OnEnterBreakPoint(This,prdat)	\
    ( (This)->lpVtbl -> OnEnterBreakPoint(This,prdat) ) 

#define IRemoteDebugApplicationEvents_OnLeaveBreakPoint(This,prdat)	\
    ( (This)->lpVtbl -> OnLeaveBreakPoint(This,prdat) ) 

#define IRemoteDebugApplicationEvents_OnCreateThread(This,prdat)	\
    ( (This)->lpVtbl -> OnCreateThread(This,prdat) ) 

#define IRemoteDebugApplicationEvents_OnDestroyThread(This,prdat)	\
    ( (This)->lpVtbl -> OnDestroyThread(This,prdat) ) 

#define IRemoteDebugApplicationEvents_OnBreakFlagChange(This,abf,prdatSteppingThread)	\
    ( (This)->lpVtbl -> OnBreakFlagChange(This,abf,prdatSteppingThread) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IRemoteDebugApplicationEvents_INTERFACE_DEFINED__ */


#ifndef __IDebugApplicationNode_INTERFACE_DEFINED__
#define __IDebugApplicationNode_INTERFACE_DEFINED__

/* interface IDebugApplicationNode */
/* [unique][uuid][object] */ 


EXTERN_C const IID IID_IDebugApplicationNode;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("51973C34-CB0C-11d0-B5C9-00A0244A0E7A")
    IDebugApplicationNode : public IDebugDocumentProvider
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE EnumChildren( 
            /* [out] */ __RPC__deref_out_opt IEnumDebugApplicationNodes **pperddp) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetParent( 
            /* [out] */ __RPC__deref_out_opt IDebugApplicationNode **pprddp) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetDocumentProvider( 
            /* [in] */ __RPC__in_opt IDebugDocumentProvider *pddp) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Close( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Attach( 
            /* [in] */ __RPC__in_opt IDebugApplicationNode *pdanParent) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Detach( void) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IDebugApplicationNodeVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IDebugApplicationNode * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IDebugApplicationNode * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IDebugApplicationNode * This);
        
        DECLSPEC_XFGVIRT(IDebugDocumentInfo, GetName)
        HRESULT ( STDMETHODCALLTYPE *GetName )( 
            __RPC__in IDebugApplicationNode * This,
            /* [in] */ DOCUMENTNAMETYPE dnt,
            /* [out] */ __RPC__deref_out_opt BSTR *pbstrName);
        
        DECLSPEC_XFGVIRT(IDebugDocumentInfo, GetDocumentClassId)
        HRESULT ( STDMETHODCALLTYPE *GetDocumentClassId )( 
            __RPC__in IDebugApplicationNode * This,
            /* [out] */ __RPC__out CLSID *pclsidDocument);
        
        DECLSPEC_XFGVIRT(IDebugDocumentProvider, GetDocument)
        HRESULT ( STDMETHODCALLTYPE *GetDocument )( 
            __RPC__in IDebugApplicationNode * This,
            /* [out] */ __RPC__deref_out_opt IDebugDocument **ppssd);
        
        DECLSPEC_XFGVIRT(IDebugApplicationNode, EnumChildren)
        HRESULT ( STDMETHODCALLTYPE *EnumChildren )( 
            __RPC__in IDebugApplicationNode * This,
            /* [out] */ __RPC__deref_out_opt IEnumDebugApplicationNodes **pperddp);
        
        DECLSPEC_XFGVIRT(IDebugApplicationNode, GetParent)
        HRESULT ( STDMETHODCALLTYPE *GetParent )( 
            __RPC__in IDebugApplicationNode * This,
            /* [out] */ __RPC__deref_out_opt IDebugApplicationNode **pprddp);
        
        DECLSPEC_XFGVIRT(IDebugApplicationNode, SetDocumentProvider)
        HRESULT ( STDMETHODCALLTYPE *SetDocumentProvider )( 
            __RPC__in IDebugApplicationNode * This,
            /* [in] */ __RPC__in_opt IDebugDocumentProvider *pddp);
        
        DECLSPEC_XFGVIRT(IDebugApplicationNode, Close)
        HRESULT ( STDMETHODCALLTYPE *Close )( 
            __RPC__in IDebugApplicationNode * This);
        
        DECLSPEC_XFGVIRT(IDebugApplicationNode, Attach)
        HRESULT ( STDMETHODCALLTYPE *Attach )( 
            __RPC__in IDebugApplicationNode * This,
            /* [in] */ __RPC__in_opt IDebugApplicationNode *pdanParent);
        
        DECLSPEC_XFGVIRT(IDebugApplicationNode, Detach)
        HRESULT ( STDMETHODCALLTYPE *Detach )( 
            __RPC__in IDebugApplicationNode * This);
        
        END_INTERFACE
    } IDebugApplicationNodeVtbl;

    interface IDebugApplicationNode
    {
        CONST_VTBL struct IDebugApplicationNodeVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IDebugApplicationNode_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IDebugApplicationNode_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IDebugApplicationNode_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IDebugApplicationNode_GetName(This,dnt,pbstrName)	\
    ( (This)->lpVtbl -> GetName(This,dnt,pbstrName) ) 

#define IDebugApplicationNode_GetDocumentClassId(This,pclsidDocument)	\
    ( (This)->lpVtbl -> GetDocumentClassId(This,pclsidDocument) ) 


#define IDebugApplicationNode_GetDocument(This,ppssd)	\
    ( (This)->lpVtbl -> GetDocument(This,ppssd) ) 


#define IDebugApplicationNode_EnumChildren(This,pperddp)	\
    ( (This)->lpVtbl -> EnumChildren(This,pperddp) ) 

#define IDebugApplicationNode_GetParent(This,pprddp)	\
    ( (This)->lpVtbl -> GetParent(This,pprddp) ) 

#define IDebugApplicationNode_SetDocumentProvider(This,pddp)	\
    ( (This)->lpVtbl -> SetDocumentProvider(This,pddp) ) 

#define IDebugApplicationNode_Close(This)	\
    ( (This)->lpVtbl -> Close(This) ) 

#define IDebugApplicationNode_Attach(This,pdanParent)	\
    ( (This)->lpVtbl -> Attach(This,pdanParent) ) 

#define IDebugApplicationNode_Detach(This)	\
    ( (This)->lpVtbl -> Detach(This) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IDebugApplicationNode_INTERFACE_DEFINED__ */


#ifndef __IDebugApplicationNodeEvents_INTERFACE_DEFINED__
#define __IDebugApplicationNodeEvents_INTERFACE_DEFINED__

/* interface IDebugApplicationNodeEvents */
/* [unique][async_uuid][uuid][object] */ 


EXTERN_C const IID IID_IDebugApplicationNodeEvents;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("51973C35-CB0C-11d0-B5C9-00A0244A0E7A")
    IDebugApplicationNodeEvents : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE onAddChild( 
            /* [in] */ __RPC__in_opt IDebugApplicationNode *prddpChild) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE onRemoveChild( 
            /* [in] */ __RPC__in_opt IDebugApplicationNode *prddpChild) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE onDetach( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE onAttach( 
            /* [in] */ __RPC__in_opt IDebugApplicationNode *prddpParent) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IDebugApplicationNodeEventsVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IDebugApplicationNodeEvents * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IDebugApplicationNodeEvents * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IDebugApplicationNodeEvents * This);
        
        DECLSPEC_XFGVIRT(IDebugApplicationNodeEvents, onAddChild)
        HRESULT ( STDMETHODCALLTYPE *onAddChild )( 
            __RPC__in IDebugApplicationNodeEvents * This,
            /* [in] */ __RPC__in_opt IDebugApplicationNode *prddpChild);
        
        DECLSPEC_XFGVIRT(IDebugApplicationNodeEvents, onRemoveChild)
        HRESULT ( STDMETHODCALLTYPE *onRemoveChild )( 
            __RPC__in IDebugApplicationNodeEvents * This,
            /* [in] */ __RPC__in_opt IDebugApplicationNode *prddpChild);
        
        DECLSPEC_XFGVIRT(IDebugApplicationNodeEvents, onDetach)
        HRESULT ( STDMETHODCALLTYPE *onDetach )( 
            __RPC__in IDebugApplicationNodeEvents * This);
        
        DECLSPEC_XFGVIRT(IDebugApplicationNodeEvents, onAttach)
        HRESULT ( STDMETHODCALLTYPE *onAttach )( 
            __RPC__in IDebugApplicationNodeEvents * This,
            /* [in] */ __RPC__in_opt IDebugApplicationNode *prddpParent);
        
        END_INTERFACE
    } IDebugApplicationNodeEventsVtbl;

    interface IDebugApplicationNodeEvents
    {
        CONST_VTBL struct IDebugApplicationNodeEventsVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IDebugApplicationNodeEvents_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IDebugApplicationNodeEvents_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IDebugApplicationNodeEvents_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IDebugApplicationNodeEvents_onAddChild(This,prddpChild)	\
    ( (This)->lpVtbl -> onAddChild(This,prddpChild) ) 

#define IDebugApplicationNodeEvents_onRemoveChild(This,prddpChild)	\
    ( (This)->lpVtbl -> onRemoveChild(This,prddpChild) ) 

#define IDebugApplicationNodeEvents_onDetach(This)	\
    ( (This)->lpVtbl -> onDetach(This) ) 

#define IDebugApplicationNodeEvents_onAttach(This,prddpParent)	\
    ( (This)->lpVtbl -> onAttach(This,prddpParent) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IDebugApplicationNodeEvents_INTERFACE_DEFINED__ */


#ifndef __AsyncIDebugApplicationNodeEvents_INTERFACE_DEFINED__
#define __AsyncIDebugApplicationNodeEvents_INTERFACE_DEFINED__

/* interface AsyncIDebugApplicationNodeEvents */
/* [uuid][unique][object] */ 


EXTERN_C const IID IID_AsyncIDebugApplicationNodeEvents;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("a2e3aa3b-aa8d-4ebf-84cd-648b737b8c13")
    AsyncIDebugApplicationNodeEvents : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE Begin_onAddChild( 
            /* [in] */ __RPC__in_opt IDebugApplicationNode *prddpChild) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Finish_onAddChild( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Begin_onRemoveChild( 
            /* [in] */ __RPC__in_opt IDebugApplicationNode *prddpChild) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Finish_onRemoveChild( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Begin_onDetach( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Finish_onDetach( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Begin_onAttach( 
            /* [in] */ __RPC__in_opt IDebugApplicationNode *prddpParent) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Finish_onAttach( void) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct AsyncIDebugApplicationNodeEventsVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in AsyncIDebugApplicationNodeEvents * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in AsyncIDebugApplicationNodeEvents * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in AsyncIDebugApplicationNodeEvents * This);
        
        DECLSPEC_XFGVIRT(AsyncIDebugApplicationNodeEvents, Begin_onAddChild)
        HRESULT ( STDMETHODCALLTYPE *Begin_onAddChild )( 
            __RPC__in AsyncIDebugApplicationNodeEvents * This,
            /* [in] */ __RPC__in_opt IDebugApplicationNode *prddpChild);
        
        DECLSPEC_XFGVIRT(AsyncIDebugApplicationNodeEvents, Finish_onAddChild)
        HRESULT ( STDMETHODCALLTYPE *Finish_onAddChild )( 
            __RPC__in AsyncIDebugApplicationNodeEvents * This);
        
        DECLSPEC_XFGVIRT(AsyncIDebugApplicationNodeEvents, Begin_onRemoveChild)
        HRESULT ( STDMETHODCALLTYPE *Begin_onRemoveChild )( 
            __RPC__in AsyncIDebugApplicationNodeEvents * This,
            /* [in] */ __RPC__in_opt IDebugApplicationNode *prddpChild);
        
        DECLSPEC_XFGVIRT(AsyncIDebugApplicationNodeEvents, Finish_onRemoveChild)
        HRESULT ( STDMETHODCALLTYPE *Finish_onRemoveChild )( 
            __RPC__in AsyncIDebugApplicationNodeEvents * This);
        
        DECLSPEC_XFGVIRT(AsyncIDebugApplicationNodeEvents, Begin_onDetach)
        HRESULT ( STDMETHODCALLTYPE *Begin_onDetach )( 
            __RPC__in AsyncIDebugApplicationNodeEvents * This);
        
        DECLSPEC_XFGVIRT(AsyncIDebugApplicationNodeEvents, Finish_onDetach)
        HRESULT ( STDMETHODCALLTYPE *Finish_onDetach )( 
            __RPC__in AsyncIDebugApplicationNodeEvents * This);
        
        DECLSPEC_XFGVIRT(AsyncIDebugApplicationNodeEvents, Begin_onAttach)
        HRESULT ( STDMETHODCALLTYPE *Begin_onAttach )( 
            __RPC__in AsyncIDebugApplicationNodeEvents * This,
            /* [in] */ __RPC__in_opt IDebugApplicationNode *prddpParent);
        
        DECLSPEC_XFGVIRT(AsyncIDebugApplicationNodeEvents, Finish_onAttach)
        HRESULT ( STDMETHODCALLTYPE *Finish_onAttach )( 
            __RPC__in AsyncIDebugApplicationNodeEvents * This);
        
        END_INTERFACE
    } AsyncIDebugApplicationNodeEventsVtbl;

    interface AsyncIDebugApplicationNodeEvents
    {
        CONST_VTBL struct AsyncIDebugApplicationNodeEventsVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define AsyncIDebugApplicationNodeEvents_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define AsyncIDebugApplicationNodeEvents_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define AsyncIDebugApplicationNodeEvents_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define AsyncIDebugApplicationNodeEvents_Begin_onAddChild(This,prddpChild)	\
    ( (This)->lpVtbl -> Begin_onAddChild(This,prddpChild) ) 

#define AsyncIDebugApplicationNodeEvents_Finish_onAddChild(This)	\
    ( (This)->lpVtbl -> Finish_onAddChild(This) ) 

#define AsyncIDebugApplicationNodeEvents_Begin_onRemoveChild(This,prddpChild)	\
    ( (This)->lpVtbl -> Begin_onRemoveChild(This,prddpChild) ) 

#define AsyncIDebugApplicationNodeEvents_Finish_onRemoveChild(This)	\
    ( (This)->lpVtbl -> Finish_onRemoveChild(This) ) 

#define AsyncIDebugApplicationNodeEvents_Begin_onDetach(This)	\
    ( (This)->lpVtbl -> Begin_onDetach(This) ) 

#define AsyncIDebugApplicationNodeEvents_Finish_onDetach(This)	\
    ( (This)->lpVtbl -> Finish_onDetach(This) ) 

#define AsyncIDebugApplicationNodeEvents_Begin_onAttach(This,prddpParent)	\
    ( (This)->lpVtbl -> Begin_onAttach(This,prddpParent) ) 

#define AsyncIDebugApplicationNodeEvents_Finish_onAttach(This)	\
    ( (This)->lpVtbl -> Finish_onAttach(This) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __AsyncIDebugApplicationNodeEvents_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_activdbg_0000_0045 */
/* [local] */ 

#ifndef DISABLE_ACTIVDBG_INTERFACE_WRAPPERS
#ifdef _WIN64
#define IDebugThreadCall IDebugThreadCall64
#define IID_IDebugThreadCall IID_IDebugThreadCall64
#else
#define IDebugThreadCall IDebugThreadCall32
#define IID_IDebugThreadCall IID_IDebugThreadCall32
#endif
#endif


extern RPC_IF_HANDLE __MIDL_itf_activdbg_0000_0045_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_activdbg_0000_0045_v0_0_s_ifspec;

#ifndef __IDebugThreadCall32_INTERFACE_DEFINED__
#define __IDebugThreadCall32_INTERFACE_DEFINED__

/* interface IDebugThreadCall32 */
/* [local][unique][uuid][object] */ 


EXTERN_C const IID IID_IDebugThreadCall32;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("51973C36-CB0C-11d0-B5C9-00A0244A0E7A")
    IDebugThreadCall32 : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE ThreadCallHandler( 
            /* [in] */ DWORD dwParam1,
            /* [in] */ DWORD dwParam2,
            /* [in] */ DWORD dwParam3) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IDebugThreadCall32Vtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IDebugThreadCall32 * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IDebugThreadCall32 * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IDebugThreadCall32 * This);
        
        DECLSPEC_XFGVIRT(IDebugThreadCall32, ThreadCallHandler)
        HRESULT ( STDMETHODCALLTYPE *ThreadCallHandler )( 
            IDebugThreadCall32 * This,
            /* [in] */ DWORD dwParam1,
            /* [in] */ DWORD dwParam2,
            /* [in] */ DWORD dwParam3);
        
        END_INTERFACE
    } IDebugThreadCall32Vtbl;

    interface IDebugThreadCall32
    {
        CONST_VTBL struct IDebugThreadCall32Vtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IDebugThreadCall32_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IDebugThreadCall32_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IDebugThreadCall32_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IDebugThreadCall32_ThreadCallHandler(This,dwParam1,dwParam2,dwParam3)	\
    ( (This)->lpVtbl -> ThreadCallHandler(This,dwParam1,dwParam2,dwParam3) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IDebugThreadCall32_INTERFACE_DEFINED__ */


#ifndef __IDebugThreadCall64_INTERFACE_DEFINED__
#define __IDebugThreadCall64_INTERFACE_DEFINED__

/* interface IDebugThreadCall64 */
/* [local][unique][uuid][object] */ 


EXTERN_C const IID IID_IDebugThreadCall64;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("cb3fa335-e979-42fd-9fcf-a7546a0f3905")
    IDebugThreadCall64 : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE ThreadCallHandler( 
            /* [in] */ DWORDLONG dwParam1,
            /* [in] */ DWORDLONG dwParam2,
            /* [in] */ DWORDLONG dwParam3) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IDebugThreadCall64Vtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IDebugThreadCall64 * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IDebugThreadCall64 * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IDebugThreadCall64 * This);
        
        DECLSPEC_XFGVIRT(IDebugThreadCall64, ThreadCallHandler)
        HRESULT ( STDMETHODCALLTYPE *ThreadCallHandler )( 
            IDebugThreadCall64 * This,
            /* [in] */ DWORDLONG dwParam1,
            /* [in] */ DWORDLONG dwParam2,
            /* [in] */ DWORDLONG dwParam3);
        
        END_INTERFACE
    } IDebugThreadCall64Vtbl;

    interface IDebugThreadCall64
    {
        CONST_VTBL struct IDebugThreadCall64Vtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IDebugThreadCall64_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IDebugThreadCall64_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IDebugThreadCall64_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IDebugThreadCall64_ThreadCallHandler(This,dwParam1,dwParam2,dwParam3)	\
    ( (This)->lpVtbl -> ThreadCallHandler(This,dwParam1,dwParam2,dwParam3) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IDebugThreadCall64_INTERFACE_DEFINED__ */


#ifndef __IRemoteDebugApplicationThread_INTERFACE_DEFINED__
#define __IRemoteDebugApplicationThread_INTERFACE_DEFINED__

/* interface IRemoteDebugApplicationThread */
/* [unique][uuid][object] */ 

typedef DWORD THREAD_STATE;

#define	THREAD_STATE_RUNNING	( 0x1 )

#define	THREAD_STATE_SUSPENDED	( 0x2 )

#define	THREAD_BLOCKED	( 0x4 )

#define	THREAD_OUT_OF_CONTEXT	( 0x8 )


EXTERN_C const IID IID_IRemoteDebugApplicationThread;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("51973C37-CB0C-11d0-B5C9-00A0244A0E7A")
    IRemoteDebugApplicationThread : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetSystemThreadId( 
            /* [out] */ __RPC__out DWORD *dwThreadId) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetApplication( 
            /* [out] */ __RPC__deref_out_opt IRemoteDebugApplication **pprda) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE EnumStackFrames( 
            /* [out] */ __RPC__deref_out_opt IEnumDebugStackFrames **ppedsf) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetDescription( 
            /* [out] */ __RPC__deref_out_opt BSTR *pbstrDescription,
            /* [out] */ __RPC__deref_out_opt BSTR *pbstrState) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetNextStatement( 
            /* [in] */ __RPC__in_opt IDebugStackFrame *pStackFrame,
            /* [in] */ __RPC__in_opt IDebugCodeContext *pCodeContext) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetState( 
            /* [out] */ __RPC__out DWORD *pState) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Suspend( 
            /* [out] */ __RPC__out DWORD *pdwCount) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Resume( 
            /* [out] */ __RPC__out DWORD *pdwCount) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetSuspendCount( 
            /* [out] */ __RPC__out DWORD *pdwCount) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IRemoteDebugApplicationThreadVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IRemoteDebugApplicationThread * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IRemoteDebugApplicationThread * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IRemoteDebugApplicationThread * This);
        
        DECLSPEC_XFGVIRT(IRemoteDebugApplicationThread, GetSystemThreadId)
        HRESULT ( STDMETHODCALLTYPE *GetSystemThreadId )( 
            __RPC__in IRemoteDebugApplicationThread * This,
            /* [out] */ __RPC__out DWORD *dwThreadId);
        
        DECLSPEC_XFGVIRT(IRemoteDebugApplicationThread, GetApplication)
        HRESULT ( STDMETHODCALLTYPE *GetApplication )( 
            __RPC__in IRemoteDebugApplicationThread * This,
            /* [out] */ __RPC__deref_out_opt IRemoteDebugApplication **pprda);
        
        DECLSPEC_XFGVIRT(IRemoteDebugApplicationThread, EnumStackFrames)
        HRESULT ( STDMETHODCALLTYPE *EnumStackFrames )( 
            __RPC__in IRemoteDebugApplicationThread * This,
            /* [out] */ __RPC__deref_out_opt IEnumDebugStackFrames **ppedsf);
        
        DECLSPEC_XFGVIRT(IRemoteDebugApplicationThread, GetDescription)
        HRESULT ( STDMETHODCALLTYPE *GetDescription )( 
            __RPC__in IRemoteDebugApplicationThread * This,
            /* [out] */ __RPC__deref_out_opt BSTR *pbstrDescription,
            /* [out] */ __RPC__deref_out_opt BSTR *pbstrState);
        
        DECLSPEC_XFGVIRT(IRemoteDebugApplicationThread, SetNextStatement)
        HRESULT ( STDMETHODCALLTYPE *SetNextStatement )( 
            __RPC__in IRemoteDebugApplicationThread * This,
            /* [in] */ __RPC__in_opt IDebugStackFrame *pStackFrame,
            /* [in] */ __RPC__in_opt IDebugCodeContext *pCodeContext);
        
        DECLSPEC_XFGVIRT(IRemoteDebugApplicationThread, GetState)
        HRESULT ( STDMETHODCALLTYPE *GetState )( 
            __RPC__in IRemoteDebugApplicationThread * This,
            /* [out] */ __RPC__out DWORD *pState);
        
        DECLSPEC_XFGVIRT(IRemoteDebugApplicationThread, Suspend)
        HRESULT ( STDMETHODCALLTYPE *Suspend )( 
            __RPC__in IRemoteDebugApplicationThread * This,
            /* [out] */ __RPC__out DWORD *pdwCount);
        
        DECLSPEC_XFGVIRT(IRemoteDebugApplicationThread, Resume)
        HRESULT ( STDMETHODCALLTYPE *Resume )( 
            __RPC__in IRemoteDebugApplicationThread * This,
            /* [out] */ __RPC__out DWORD *pdwCount);
        
        DECLSPEC_XFGVIRT(IRemoteDebugApplicationThread, GetSuspendCount)
        HRESULT ( STDMETHODCALLTYPE *GetSuspendCount )( 
            __RPC__in IRemoteDebugApplicationThread * This,
            /* [out] */ __RPC__out DWORD *pdwCount);
        
        END_INTERFACE
    } IRemoteDebugApplicationThreadVtbl;

    interface IRemoteDebugApplicationThread
    {
        CONST_VTBL struct IRemoteDebugApplicationThreadVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IRemoteDebugApplicationThread_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IRemoteDebugApplicationThread_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IRemoteDebugApplicationThread_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IRemoteDebugApplicationThread_GetSystemThreadId(This,dwThreadId)	\
    ( (This)->lpVtbl -> GetSystemThreadId(This,dwThreadId) ) 

#define IRemoteDebugApplicationThread_GetApplication(This,pprda)	\
    ( (This)->lpVtbl -> GetApplication(This,pprda) ) 

#define IRemoteDebugApplicationThread_EnumStackFrames(This,ppedsf)	\
    ( (This)->lpVtbl -> EnumStackFrames(This,ppedsf) ) 

#define IRemoteDebugApplicationThread_GetDescription(This,pbstrDescription,pbstrState)	\
    ( (This)->lpVtbl -> GetDescription(This,pbstrDescription,pbstrState) ) 

#define IRemoteDebugApplicationThread_SetNextStatement(This,pStackFrame,pCodeContext)	\
    ( (This)->lpVtbl -> SetNextStatement(This,pStackFrame,pCodeContext) ) 

#define IRemoteDebugApplicationThread_GetState(This,pState)	\
    ( (This)->lpVtbl -> GetState(This,pState) ) 

#define IRemoteDebugApplicationThread_Suspend(This,pdwCount)	\
    ( (This)->lpVtbl -> Suspend(This,pdwCount) ) 

#define IRemoteDebugApplicationThread_Resume(This,pdwCount)	\
    ( (This)->lpVtbl -> Resume(This,pdwCount) ) 

#define IRemoteDebugApplicationThread_GetSuspendCount(This,pdwCount)	\
    ( (This)->lpVtbl -> GetSuspendCount(This,pdwCount) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IRemoteDebugApplicationThread_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_activdbg_0000_0048 */
/* [local] */ 

#ifndef DISABLE_ACTIVDBG_INTERFACE_WRAPPERS
#ifdef _WIN64
#define SynchronousCallIntoThread SynchronousCallIntoThread64
#else
#define SynchronousCallIntoThread SynchronousCallIntoThread32
#endif
#endif


extern RPC_IF_HANDLE __MIDL_itf_activdbg_0000_0048_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_activdbg_0000_0048_v0_0_s_ifspec;

#ifndef __IDebugApplicationThread_INTERFACE_DEFINED__
#define __IDebugApplicationThread_INTERFACE_DEFINED__

/* interface IDebugApplicationThread */
/* [local][unique][uuid][object] */ 


EXTERN_C const IID IID_IDebugApplicationThread;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("51973C38-CB0C-11d0-B5C9-00A0244A0E7A")
    IDebugApplicationThread : public IRemoteDebugApplicationThread
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE SynchronousCallIntoThread32( 
            /* [in] */ IDebugThreadCall32 *pstcb,
            /* [in] */ DWORD dwParam1,
            /* [in] */ DWORD dwParam2,
            /* [in] */ DWORD dwParam3) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE QueryIsCurrentThread( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE QueryIsDebuggerThread( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetDescription( 
            /* [in] */ LPCOLESTR pstrDescription) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetStateString( 
            /* [in] */ LPCOLESTR pstrState) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IDebugApplicationThreadVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IDebugApplicationThread * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IDebugApplicationThread * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IDebugApplicationThread * This);
        
        DECLSPEC_XFGVIRT(IRemoteDebugApplicationThread, GetSystemThreadId)
        HRESULT ( STDMETHODCALLTYPE *GetSystemThreadId )( 
            IDebugApplicationThread * This,
            /* [out] */ DWORD *dwThreadId);
        
        DECLSPEC_XFGVIRT(IRemoteDebugApplicationThread, GetApplication)
        HRESULT ( STDMETHODCALLTYPE *GetApplication )( 
            IDebugApplicationThread * This,
            /* [out] */ IRemoteDebugApplication **pprda);
        
        DECLSPEC_XFGVIRT(IRemoteDebugApplicationThread, EnumStackFrames)
        HRESULT ( STDMETHODCALLTYPE *EnumStackFrames )( 
            IDebugApplicationThread * This,
            /* [out] */ IEnumDebugStackFrames **ppedsf);
        
        DECLSPEC_XFGVIRT(IRemoteDebugApplicationThread, GetDescription)
        HRESULT ( STDMETHODCALLTYPE *GetDescription )( 
            IDebugApplicationThread * This,
            /* [out] */ BSTR *pbstrDescription,
            /* [out] */ BSTR *pbstrState);
        
        DECLSPEC_XFGVIRT(IRemoteDebugApplicationThread, SetNextStatement)
        HRESULT ( STDMETHODCALLTYPE *SetNextStatement )( 
            IDebugApplicationThread * This,
            /* [in] */ IDebugStackFrame *pStackFrame,
            /* [in] */ IDebugCodeContext *pCodeContext);
        
        DECLSPEC_XFGVIRT(IRemoteDebugApplicationThread, GetState)
        HRESULT ( STDMETHODCALLTYPE *GetState )( 
            IDebugApplicationThread * This,
            /* [out] */ DWORD *pState);
        
        DECLSPEC_XFGVIRT(IRemoteDebugApplicationThread, Suspend)
        HRESULT ( STDMETHODCALLTYPE *Suspend )( 
            IDebugApplicationThread * This,
            /* [out] */ DWORD *pdwCount);
        
        DECLSPEC_XFGVIRT(IRemoteDebugApplicationThread, Resume)
        HRESULT ( STDMETHODCALLTYPE *Resume )( 
            IDebugApplicationThread * This,
            /* [out] */ DWORD *pdwCount);
        
        DECLSPEC_XFGVIRT(IRemoteDebugApplicationThread, GetSuspendCount)
        HRESULT ( STDMETHODCALLTYPE *GetSuspendCount )( 
            IDebugApplicationThread * This,
            /* [out] */ DWORD *pdwCount);
        
        DECLSPEC_XFGVIRT(IDebugApplicationThread, SynchronousCallIntoThread32)
        HRESULT ( STDMETHODCALLTYPE *SynchronousCallIntoThread32 )( 
            IDebugApplicationThread * This,
            /* [in] */ IDebugThreadCall32 *pstcb,
            /* [in] */ DWORD dwParam1,
            /* [in] */ DWORD dwParam2,
            /* [in] */ DWORD dwParam3);
        
        DECLSPEC_XFGVIRT(IDebugApplicationThread, QueryIsCurrentThread)
        HRESULT ( STDMETHODCALLTYPE *QueryIsCurrentThread )( 
            IDebugApplicationThread * This);
        
        DECLSPEC_XFGVIRT(IDebugApplicationThread, QueryIsDebuggerThread)
        HRESULT ( STDMETHODCALLTYPE *QueryIsDebuggerThread )( 
            IDebugApplicationThread * This);
        
        DECLSPEC_XFGVIRT(IDebugApplicationThread, SetDescription)
        HRESULT ( STDMETHODCALLTYPE *SetDescription )( 
            IDebugApplicationThread * This,
            /* [in] */ LPCOLESTR pstrDescription);
        
        DECLSPEC_XFGVIRT(IDebugApplicationThread, SetStateString)
        HRESULT ( STDMETHODCALLTYPE *SetStateString )( 
            IDebugApplicationThread * This,
            /* [in] */ LPCOLESTR pstrState);
        
        END_INTERFACE
    } IDebugApplicationThreadVtbl;

    interface IDebugApplicationThread
    {
        CONST_VTBL struct IDebugApplicationThreadVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IDebugApplicationThread_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IDebugApplicationThread_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IDebugApplicationThread_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IDebugApplicationThread_GetSystemThreadId(This,dwThreadId)	\
    ( (This)->lpVtbl -> GetSystemThreadId(This,dwThreadId) ) 

#define IDebugApplicationThread_GetApplication(This,pprda)	\
    ( (This)->lpVtbl -> GetApplication(This,pprda) ) 

#define IDebugApplicationThread_EnumStackFrames(This,ppedsf)	\
    ( (This)->lpVtbl -> EnumStackFrames(This,ppedsf) ) 

#define IDebugApplicationThread_GetDescription(This,pbstrDescription,pbstrState)	\
    ( (This)->lpVtbl -> GetDescription(This,pbstrDescription,pbstrState) ) 

#define IDebugApplicationThread_SetNextStatement(This,pStackFrame,pCodeContext)	\
    ( (This)->lpVtbl -> SetNextStatement(This,pStackFrame,pCodeContext) ) 

#define IDebugApplicationThread_GetState(This,pState)	\
    ( (This)->lpVtbl -> GetState(This,pState) ) 

#define IDebugApplicationThread_Suspend(This,pdwCount)	\
    ( (This)->lpVtbl -> Suspend(This,pdwCount) ) 

#define IDebugApplicationThread_Resume(This,pdwCount)	\
    ( (This)->lpVtbl -> Resume(This,pdwCount) ) 

#define IDebugApplicationThread_GetSuspendCount(This,pdwCount)	\
    ( (This)->lpVtbl -> GetSuspendCount(This,pdwCount) ) 


#define IDebugApplicationThread_SynchronousCallIntoThread32(This,pstcb,dwParam1,dwParam2,dwParam3)	\
    ( (This)->lpVtbl -> SynchronousCallIntoThread32(This,pstcb,dwParam1,dwParam2,dwParam3) ) 

#define IDebugApplicationThread_QueryIsCurrentThread(This)	\
    ( (This)->lpVtbl -> QueryIsCurrentThread(This) ) 

#define IDebugApplicationThread_QueryIsDebuggerThread(This)	\
    ( (This)->lpVtbl -> QueryIsDebuggerThread(This) ) 

#define IDebugApplicationThread_SetDescription(This,pstrDescription)	\
    ( (This)->lpVtbl -> SetDescription(This,pstrDescription) ) 

#define IDebugApplicationThread_SetStateString(This,pstrState)	\
    ( (This)->lpVtbl -> SetStateString(This,pstrState) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IDebugApplicationThread_INTERFACE_DEFINED__ */


#ifndef __IDebugApplicationThread64_INTERFACE_DEFINED__
#define __IDebugApplicationThread64_INTERFACE_DEFINED__

/* interface IDebugApplicationThread64 */
/* [local][unique][uuid][object] */ 


EXTERN_C const IID IID_IDebugApplicationThread64;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("9dac5886-dbad-456d-9dee-5dec39ab3dda")
    IDebugApplicationThread64 : public IDebugApplicationThread
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE SynchronousCallIntoThread64( 
            /* [in] */ IDebugThreadCall64 *pstcb,
            /* [in] */ DWORDLONG dwParam1,
            /* [in] */ DWORDLONG dwParam2,
            /* [in] */ DWORDLONG dwParam3) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IDebugApplicationThread64Vtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IDebugApplicationThread64 * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IDebugApplicationThread64 * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IDebugApplicationThread64 * This);
        
        DECLSPEC_XFGVIRT(IRemoteDebugApplicationThread, GetSystemThreadId)
        HRESULT ( STDMETHODCALLTYPE *GetSystemThreadId )( 
            IDebugApplicationThread64 * This,
            /* [out] */ DWORD *dwThreadId);
        
        DECLSPEC_XFGVIRT(IRemoteDebugApplicationThread, GetApplication)
        HRESULT ( STDMETHODCALLTYPE *GetApplication )( 
            IDebugApplicationThread64 * This,
            /* [out] */ IRemoteDebugApplication **pprda);
        
        DECLSPEC_XFGVIRT(IRemoteDebugApplicationThread, EnumStackFrames)
        HRESULT ( STDMETHODCALLTYPE *EnumStackFrames )( 
            IDebugApplicationThread64 * This,
            /* [out] */ IEnumDebugStackFrames **ppedsf);
        
        DECLSPEC_XFGVIRT(IRemoteDebugApplicationThread, GetDescription)
        HRESULT ( STDMETHODCALLTYPE *GetDescription )( 
            IDebugApplicationThread64 * This,
            /* [out] */ BSTR *pbstrDescription,
            /* [out] */ BSTR *pbstrState);
        
        DECLSPEC_XFGVIRT(IRemoteDebugApplicationThread, SetNextStatement)
        HRESULT ( STDMETHODCALLTYPE *SetNextStatement )( 
            IDebugApplicationThread64 * This,
            /* [in] */ IDebugStackFrame *pStackFrame,
            /* [in] */ IDebugCodeContext *pCodeContext);
        
        DECLSPEC_XFGVIRT(IRemoteDebugApplicationThread, GetState)
        HRESULT ( STDMETHODCALLTYPE *GetState )( 
            IDebugApplicationThread64 * This,
            /* [out] */ DWORD *pState);
        
        DECLSPEC_XFGVIRT(IRemoteDebugApplicationThread, Suspend)
        HRESULT ( STDMETHODCALLTYPE *Suspend )( 
            IDebugApplicationThread64 * This,
            /* [out] */ DWORD *pdwCount);
        
        DECLSPEC_XFGVIRT(IRemoteDebugApplicationThread, Resume)
        HRESULT ( STDMETHODCALLTYPE *Resume )( 
            IDebugApplicationThread64 * This,
            /* [out] */ DWORD *pdwCount);
        
        DECLSPEC_XFGVIRT(IRemoteDebugApplicationThread, GetSuspendCount)
        HRESULT ( STDMETHODCALLTYPE *GetSuspendCount )( 
            IDebugApplicationThread64 * This,
            /* [out] */ DWORD *pdwCount);
        
        DECLSPEC_XFGVIRT(IDebugApplicationThread, SynchronousCallIntoThread32)
        HRESULT ( STDMETHODCALLTYPE *SynchronousCallIntoThread32 )( 
            IDebugApplicationThread64 * This,
            /* [in] */ IDebugThreadCall32 *pstcb,
            /* [in] */ DWORD dwParam1,
            /* [in] */ DWORD dwParam2,
            /* [in] */ DWORD dwParam3);
        
        DECLSPEC_XFGVIRT(IDebugApplicationThread, QueryIsCurrentThread)
        HRESULT ( STDMETHODCALLTYPE *QueryIsCurrentThread )( 
            IDebugApplicationThread64 * This);
        
        DECLSPEC_XFGVIRT(IDebugApplicationThread, QueryIsDebuggerThread)
        HRESULT ( STDMETHODCALLTYPE *QueryIsDebuggerThread )( 
            IDebugApplicationThread64 * This);
        
        DECLSPEC_XFGVIRT(IDebugApplicationThread, SetDescription)
        HRESULT ( STDMETHODCALLTYPE *SetDescription )( 
            IDebugApplicationThread64 * This,
            /* [in] */ LPCOLESTR pstrDescription);
        
        DECLSPEC_XFGVIRT(IDebugApplicationThread, SetStateString)
        HRESULT ( STDMETHODCALLTYPE *SetStateString )( 
            IDebugApplicationThread64 * This,
            /* [in] */ LPCOLESTR pstrState);
        
        DECLSPEC_XFGVIRT(IDebugApplicationThread64, SynchronousCallIntoThread64)
        HRESULT ( STDMETHODCALLTYPE *SynchronousCallIntoThread64 )( 
            IDebugApplicationThread64 * This,
            /* [in] */ IDebugThreadCall64 *pstcb,
            /* [in] */ DWORDLONG dwParam1,
            /* [in] */ DWORDLONG dwParam2,
            /* [in] */ DWORDLONG dwParam3);
        
        END_INTERFACE
    } IDebugApplicationThread64Vtbl;

    interface IDebugApplicationThread64
    {
        CONST_VTBL struct IDebugApplicationThread64Vtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IDebugApplicationThread64_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IDebugApplicationThread64_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IDebugApplicationThread64_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IDebugApplicationThread64_GetSystemThreadId(This,dwThreadId)	\
    ( (This)->lpVtbl -> GetSystemThreadId(This,dwThreadId) ) 

#define IDebugApplicationThread64_GetApplication(This,pprda)	\
    ( (This)->lpVtbl -> GetApplication(This,pprda) ) 

#define IDebugApplicationThread64_EnumStackFrames(This,ppedsf)	\
    ( (This)->lpVtbl -> EnumStackFrames(This,ppedsf) ) 

#define IDebugApplicationThread64_GetDescription(This,pbstrDescription,pbstrState)	\
    ( (This)->lpVtbl -> GetDescription(This,pbstrDescription,pbstrState) ) 

#define IDebugApplicationThread64_SetNextStatement(This,pStackFrame,pCodeContext)	\
    ( (This)->lpVtbl -> SetNextStatement(This,pStackFrame,pCodeContext) ) 

#define IDebugApplicationThread64_GetState(This,pState)	\
    ( (This)->lpVtbl -> GetState(This,pState) ) 

#define IDebugApplicationThread64_Suspend(This,pdwCount)	\
    ( (This)->lpVtbl -> Suspend(This,pdwCount) ) 

#define IDebugApplicationThread64_Resume(This,pdwCount)	\
    ( (This)->lpVtbl -> Resume(This,pdwCount) ) 

#define IDebugApplicationThread64_GetSuspendCount(This,pdwCount)	\
    ( (This)->lpVtbl -> GetSuspendCount(This,pdwCount) ) 


#define IDebugApplicationThread64_SynchronousCallIntoThread32(This,pstcb,dwParam1,dwParam2,dwParam3)	\
    ( (This)->lpVtbl -> SynchronousCallIntoThread32(This,pstcb,dwParam1,dwParam2,dwParam3) ) 

#define IDebugApplicationThread64_QueryIsCurrentThread(This)	\
    ( (This)->lpVtbl -> QueryIsCurrentThread(This) ) 

#define IDebugApplicationThread64_QueryIsDebuggerThread(This)	\
    ( (This)->lpVtbl -> QueryIsDebuggerThread(This) ) 

#define IDebugApplicationThread64_SetDescription(This,pstrDescription)	\
    ( (This)->lpVtbl -> SetDescription(This,pstrDescription) ) 

#define IDebugApplicationThread64_SetStateString(This,pstrState)	\
    ( (This)->lpVtbl -> SetStateString(This,pstrState) ) 


#define IDebugApplicationThread64_SynchronousCallIntoThread64(This,pstcb,dwParam1,dwParam2,dwParam3)	\
    ( (This)->lpVtbl -> SynchronousCallIntoThread64(This,pstcb,dwParam1,dwParam2,dwParam3) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IDebugApplicationThread64_INTERFACE_DEFINED__ */


#ifndef __IDebugCookie_INTERFACE_DEFINED__
#define __IDebugCookie_INTERFACE_DEFINED__

/* interface IDebugCookie */
/* [unique][helpstring][uuid][local][object] */ 


EXTERN_C const IID IID_IDebugCookie;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("51973C39-CB0C-11d0-B5C9-00A0244A0E7A")
    IDebugCookie : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE SetDebugCookie( 
            /* [in] */ DWORD dwDebugAppCookie) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IDebugCookieVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IDebugCookie * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IDebugCookie * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IDebugCookie * This);
        
        DECLSPEC_XFGVIRT(IDebugCookie, SetDebugCookie)
        HRESULT ( STDMETHODCALLTYPE *SetDebugCookie )( 
            IDebugCookie * This,
            /* [in] */ DWORD dwDebugAppCookie);
        
        END_INTERFACE
    } IDebugCookieVtbl;

    interface IDebugCookie
    {
        CONST_VTBL struct IDebugCookieVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IDebugCookie_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IDebugCookie_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IDebugCookie_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IDebugCookie_SetDebugCookie(This,dwDebugAppCookie)	\
    ( (This)->lpVtbl -> SetDebugCookie(This,dwDebugAppCookie) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IDebugCookie_INTERFACE_DEFINED__ */


#ifndef __IEnumDebugApplicationNodes_INTERFACE_DEFINED__
#define __IEnumDebugApplicationNodes_INTERFACE_DEFINED__

/* interface IEnumDebugApplicationNodes */
/* [unique][helpstring][uuid][object] */ 


EXTERN_C const IID IID_IEnumDebugApplicationNodes;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("51973C3a-CB0C-11d0-B5C9-00A0244A0E7A")
    IEnumDebugApplicationNodes : public IUnknown
    {
    public:
        virtual /* [local] */ HRESULT __stdcall Next( 
            /* [in] */ ULONG celt,
            /* [out] */ IDebugApplicationNode **pprddp,
            /* [out] */ ULONG *pceltFetched) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Skip( 
            /* [in] */ ULONG celt) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Reset( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Clone( 
            /* [out] */ __RPC__deref_out_opt IEnumDebugApplicationNodes **pperddp) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IEnumDebugApplicationNodesVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IEnumDebugApplicationNodes * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IEnumDebugApplicationNodes * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IEnumDebugApplicationNodes * This);
        
        DECLSPEC_XFGVIRT(IEnumDebugApplicationNodes, Next)
        /* [local] */ HRESULT ( __stdcall *Next )( 
            IEnumDebugApplicationNodes * This,
            /* [in] */ ULONG celt,
            /* [out] */ IDebugApplicationNode **pprddp,
            /* [out] */ ULONG *pceltFetched);
        
        DECLSPEC_XFGVIRT(IEnumDebugApplicationNodes, Skip)
        HRESULT ( STDMETHODCALLTYPE *Skip )( 
            __RPC__in IEnumDebugApplicationNodes * This,
            /* [in] */ ULONG celt);
        
        DECLSPEC_XFGVIRT(IEnumDebugApplicationNodes, Reset)
        HRESULT ( STDMETHODCALLTYPE *Reset )( 
            __RPC__in IEnumDebugApplicationNodes * This);
        
        DECLSPEC_XFGVIRT(IEnumDebugApplicationNodes, Clone)
        HRESULT ( STDMETHODCALLTYPE *Clone )( 
            __RPC__in IEnumDebugApplicationNodes * This,
            /* [out] */ __RPC__deref_out_opt IEnumDebugApplicationNodes **pperddp);
        
        END_INTERFACE
    } IEnumDebugApplicationNodesVtbl;

    interface IEnumDebugApplicationNodes
    {
        CONST_VTBL struct IEnumDebugApplicationNodesVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IEnumDebugApplicationNodes_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IEnumDebugApplicationNodes_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IEnumDebugApplicationNodes_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IEnumDebugApplicationNodes_Next(This,celt,pprddp,pceltFetched)	\
    ( (This)->lpVtbl -> Next(This,celt,pprddp,pceltFetched) ) 

#define IEnumDebugApplicationNodes_Skip(This,celt)	\
    ( (This)->lpVtbl -> Skip(This,celt) ) 

#define IEnumDebugApplicationNodes_Reset(This)	\
    ( (This)->lpVtbl -> Reset(This) ) 

#define IEnumDebugApplicationNodes_Clone(This,pperddp)	\
    ( (This)->lpVtbl -> Clone(This,pperddp) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */



/* [call_as] */ HRESULT __stdcall IEnumDebugApplicationNodes_RemoteNext_Proxy( 
    __RPC__in IEnumDebugApplicationNodes * This,
    /* [in] */ ULONG celt,
    /* [length_is][size_is][out] */ __RPC__out_ecount_part(celt, *pceltFetched) IDebugApplicationNode **pprddp,
    /* [out] */ __RPC__out ULONG *pceltFetched);


void __RPC_STUB IEnumDebugApplicationNodes_RemoteNext_Stub(
    IRpcStubBuffer *This,
    IRpcChannelBuffer *_pRpcChannelBuffer,
    PRPC_MESSAGE _pRpcMessage,
    DWORD *_pdwStubPhase);



#endif 	/* __IEnumDebugApplicationNodes_INTERFACE_DEFINED__ */


#ifndef __IEnumRemoteDebugApplications_INTERFACE_DEFINED__
#define __IEnumRemoteDebugApplications_INTERFACE_DEFINED__

/* interface IEnumRemoteDebugApplications */
/* [unique][helpstring][uuid][object] */ 


EXTERN_C const IID IID_IEnumRemoteDebugApplications;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("51973C3b-CB0C-11d0-B5C9-00A0244A0E7A")
    IEnumRemoteDebugApplications : public IUnknown
    {
    public:
        virtual /* [local] */ HRESULT __stdcall Next( 
            /* [in] */ ULONG celt,
            /* [out] */ IRemoteDebugApplication **ppda,
            /* [out] */ ULONG *pceltFetched) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Skip( 
            /* [in] */ ULONG celt) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Reset( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Clone( 
            /* [out] */ __RPC__deref_out_opt IEnumRemoteDebugApplications **ppessd) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IEnumRemoteDebugApplicationsVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IEnumRemoteDebugApplications * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IEnumRemoteDebugApplications * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IEnumRemoteDebugApplications * This);
        
        DECLSPEC_XFGVIRT(IEnumRemoteDebugApplications, Next)
        /* [local] */ HRESULT ( __stdcall *Next )( 
            IEnumRemoteDebugApplications * This,
            /* [in] */ ULONG celt,
            /* [out] */ IRemoteDebugApplication **ppda,
            /* [out] */ ULONG *pceltFetched);
        
        DECLSPEC_XFGVIRT(IEnumRemoteDebugApplications, Skip)
        HRESULT ( STDMETHODCALLTYPE *Skip )( 
            __RPC__in IEnumRemoteDebugApplications * This,
            /* [in] */ ULONG celt);
        
        DECLSPEC_XFGVIRT(IEnumRemoteDebugApplications, Reset)
        HRESULT ( STDMETHODCALLTYPE *Reset )( 
            __RPC__in IEnumRemoteDebugApplications * This);
        
        DECLSPEC_XFGVIRT(IEnumRemoteDebugApplications, Clone)
        HRESULT ( STDMETHODCALLTYPE *Clone )( 
            __RPC__in IEnumRemoteDebugApplications * This,
            /* [out] */ __RPC__deref_out_opt IEnumRemoteDebugApplications **ppessd);
        
        END_INTERFACE
    } IEnumRemoteDebugApplicationsVtbl;

    interface IEnumRemoteDebugApplications
    {
        CONST_VTBL struct IEnumRemoteDebugApplicationsVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IEnumRemoteDebugApplications_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IEnumRemoteDebugApplications_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IEnumRemoteDebugApplications_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IEnumRemoteDebugApplications_Next(This,celt,ppda,pceltFetched)	\
    ( (This)->lpVtbl -> Next(This,celt,ppda,pceltFetched) ) 

#define IEnumRemoteDebugApplications_Skip(This,celt)	\
    ( (This)->lpVtbl -> Skip(This,celt) ) 

#define IEnumRemoteDebugApplications_Reset(This)	\
    ( (This)->lpVtbl -> Reset(This) ) 

#define IEnumRemoteDebugApplications_Clone(This,ppessd)	\
    ( (This)->lpVtbl -> Clone(This,ppessd) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */



/* [call_as] */ HRESULT __stdcall IEnumRemoteDebugApplications_RemoteNext_Proxy( 
    __RPC__in IEnumRemoteDebugApplications * This,
    /* [in] */ ULONG celt,
    /* [length_is][size_is][out] */ __RPC__out_ecount_part(celt, *pceltFetched) IRemoteDebugApplication **ppda,
    /* [out] */ __RPC__out ULONG *pceltFetched);


void __RPC_STUB IEnumRemoteDebugApplications_RemoteNext_Stub(
    IRpcStubBuffer *This,
    IRpcChannelBuffer *_pRpcChannelBuffer,
    PRPC_MESSAGE _pRpcMessage,
    DWORD *_pdwStubPhase);



#endif 	/* __IEnumRemoteDebugApplications_INTERFACE_DEFINED__ */


#ifndef __IEnumRemoteDebugApplicationThreads_INTERFACE_DEFINED__
#define __IEnumRemoteDebugApplicationThreads_INTERFACE_DEFINED__

/* interface IEnumRemoteDebugApplicationThreads */
/* [unique][helpstring][uuid][object] */ 


EXTERN_C const IID IID_IEnumRemoteDebugApplicationThreads;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("51973C3c-CB0C-11d0-B5C9-00A0244A0E7A")
    IEnumRemoteDebugApplicationThreads : public IUnknown
    {
    public:
        virtual /* [local] */ HRESULT __stdcall Next( 
            /* [in] */ ULONG celt,
            /* [out] */ IRemoteDebugApplicationThread **pprdat,
            /* [out] */ ULONG *pceltFetched) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Skip( 
            /* [in] */ ULONG celt) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Reset( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Clone( 
            /* [out] */ __RPC__deref_out_opt IEnumRemoteDebugApplicationThreads **pperdat) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IEnumRemoteDebugApplicationThreadsVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IEnumRemoteDebugApplicationThreads * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IEnumRemoteDebugApplicationThreads * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IEnumRemoteDebugApplicationThreads * This);
        
        DECLSPEC_XFGVIRT(IEnumRemoteDebugApplicationThreads, Next)
        /* [local] */ HRESULT ( __stdcall *Next )( 
            IEnumRemoteDebugApplicationThreads * This,
            /* [in] */ ULONG celt,
            /* [out] */ IRemoteDebugApplicationThread **pprdat,
            /* [out] */ ULONG *pceltFetched);
        
        DECLSPEC_XFGVIRT(IEnumRemoteDebugApplicationThreads, Skip)
        HRESULT ( STDMETHODCALLTYPE *Skip )( 
            __RPC__in IEnumRemoteDebugApplicationThreads * This,
            /* [in] */ ULONG celt);
        
        DECLSPEC_XFGVIRT(IEnumRemoteDebugApplicationThreads, Reset)
        HRESULT ( STDMETHODCALLTYPE *Reset )( 
            __RPC__in IEnumRemoteDebugApplicationThreads * This);
        
        DECLSPEC_XFGVIRT(IEnumRemoteDebugApplicationThreads, Clone)
        HRESULT ( STDMETHODCALLTYPE *Clone )( 
            __RPC__in IEnumRemoteDebugApplicationThreads * This,
            /* [out] */ __RPC__deref_out_opt IEnumRemoteDebugApplicationThreads **pperdat);
        
        END_INTERFACE
    } IEnumRemoteDebugApplicationThreadsVtbl;

    interface IEnumRemoteDebugApplicationThreads
    {
        CONST_VTBL struct IEnumRemoteDebugApplicationThreadsVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IEnumRemoteDebugApplicationThreads_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IEnumRemoteDebugApplicationThreads_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IEnumRemoteDebugApplicationThreads_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IEnumRemoteDebugApplicationThreads_Next(This,celt,pprdat,pceltFetched)	\
    ( (This)->lpVtbl -> Next(This,celt,pprdat,pceltFetched) ) 

#define IEnumRemoteDebugApplicationThreads_Skip(This,celt)	\
    ( (This)->lpVtbl -> Skip(This,celt) ) 

#define IEnumRemoteDebugApplicationThreads_Reset(This)	\
    ( (This)->lpVtbl -> Reset(This) ) 

#define IEnumRemoteDebugApplicationThreads_Clone(This,pperdat)	\
    ( (This)->lpVtbl -> Clone(This,pperdat) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */



/* [call_as] */ HRESULT __stdcall IEnumRemoteDebugApplicationThreads_RemoteNext_Proxy( 
    __RPC__in IEnumRemoteDebugApplicationThreads * This,
    /* [in] */ ULONG celt,
    /* [length_is][size_is][out] */ __RPC__out_ecount_part(celt, *pceltFetched) IRemoteDebugApplicationThread **ppdat,
    /* [out] */ __RPC__out ULONG *pceltFetched);


void __RPC_STUB IEnumRemoteDebugApplicationThreads_RemoteNext_Stub(
    IRpcStubBuffer *This,
    IRpcChannelBuffer *_pRpcChannelBuffer,
    PRPC_MESSAGE _pRpcMessage,
    DWORD *_pdwStubPhase);



#endif 	/* __IEnumRemoteDebugApplicationThreads_INTERFACE_DEFINED__ */


#ifndef __IDebugFormatter_INTERFACE_DEFINED__
#define __IDebugFormatter_INTERFACE_DEFINED__

/* interface IDebugFormatter */
/* [local][unique][helpstring][uuid][object] */ 


EXTERN_C const IID IID_IDebugFormatter;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("51973C05-CB0C-11d0-B5C9-00A0244A0E7A")
    IDebugFormatter : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetStringForVariant( 
            /* [in] */ VARIANT *pvar,
            /* [in] */ ULONG nRadix,
            /* [out] */ BSTR *pbstrValue) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetVariantForString( 
            /* [in] */ LPCOLESTR pwstrValue,
            /* [out] */ VARIANT *pvar) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetStringForVarType( 
            /* [in] */ VARTYPE vt,
            /* [in] */ TYPEDESC *ptdescArrayType,
            /* [out] */ BSTR *pbstr) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IDebugFormatterVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IDebugFormatter * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IDebugFormatter * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IDebugFormatter * This);
        
        DECLSPEC_XFGVIRT(IDebugFormatter, GetStringForVariant)
        HRESULT ( STDMETHODCALLTYPE *GetStringForVariant )( 
            IDebugFormatter * This,
            /* [in] */ VARIANT *pvar,
            /* [in] */ ULONG nRadix,
            /* [out] */ BSTR *pbstrValue);
        
        DECLSPEC_XFGVIRT(IDebugFormatter, GetVariantForString)
        HRESULT ( STDMETHODCALLTYPE *GetVariantForString )( 
            IDebugFormatter * This,
            /* [in] */ LPCOLESTR pwstrValue,
            /* [out] */ VARIANT *pvar);
        
        DECLSPEC_XFGVIRT(IDebugFormatter, GetStringForVarType)
        HRESULT ( STDMETHODCALLTYPE *GetStringForVarType )( 
            IDebugFormatter * This,
            /* [in] */ VARTYPE vt,
            /* [in] */ TYPEDESC *ptdescArrayType,
            /* [out] */ BSTR *pbstr);
        
        END_INTERFACE
    } IDebugFormatterVtbl;

    interface IDebugFormatter
    {
        CONST_VTBL struct IDebugFormatterVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IDebugFormatter_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IDebugFormatter_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IDebugFormatter_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IDebugFormatter_GetStringForVariant(This,pvar,nRadix,pbstrValue)	\
    ( (This)->lpVtbl -> GetStringForVariant(This,pvar,nRadix,pbstrValue) ) 

#define IDebugFormatter_GetVariantForString(This,pwstrValue,pvar)	\
    ( (This)->lpVtbl -> GetVariantForString(This,pwstrValue,pvar) ) 

#define IDebugFormatter_GetStringForVarType(This,vt,ptdescArrayType,pbstr)	\
    ( (This)->lpVtbl -> GetStringForVarType(This,vt,ptdescArrayType,pbstr) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IDebugFormatter_INTERFACE_DEFINED__ */


#ifndef __ISimpleConnectionPoint_INTERFACE_DEFINED__
#define __ISimpleConnectionPoint_INTERFACE_DEFINED__

/* interface ISimpleConnectionPoint */
/* [local][unique][helpstring][uuid][object] */ 


EXTERN_C const IID IID_ISimpleConnectionPoint;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("51973C3e-CB0C-11d0-B5C9-00A0244A0E7A")
    ISimpleConnectionPoint : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetEventCount( 
            /* [out] */ ULONG *pulCount) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE DescribeEvents( 
            /* [in] */ ULONG iEvent,
            /* [in] */ ULONG cEvents,
            /* [length_is][size_is][out] */ DISPID *prgid,
            /* [length_is][size_is][out] */ BSTR *prgbstr,
            /* [out] */ ULONG *pcEventsFetched) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Advise( 
            /* [in] */ IDispatch *pdisp,
            /* [out] */ DWORD *pdwCookie) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Unadvise( 
            /* [in] */ DWORD dwCookie) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ISimpleConnectionPointVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            ISimpleConnectionPoint * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            ISimpleConnectionPoint * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            ISimpleConnectionPoint * This);
        
        DECLSPEC_XFGVIRT(ISimpleConnectionPoint, GetEventCount)
        HRESULT ( STDMETHODCALLTYPE *GetEventCount )( 
            ISimpleConnectionPoint * This,
            /* [out] */ ULONG *pulCount);
        
        DECLSPEC_XFGVIRT(ISimpleConnectionPoint, DescribeEvents)
        HRESULT ( STDMETHODCALLTYPE *DescribeEvents )( 
            ISimpleConnectionPoint * This,
            /* [in] */ ULONG iEvent,
            /* [in] */ ULONG cEvents,
            /* [length_is][size_is][out] */ DISPID *prgid,
            /* [length_is][size_is][out] */ BSTR *prgbstr,
            /* [out] */ ULONG *pcEventsFetched);
        
        DECLSPEC_XFGVIRT(ISimpleConnectionPoint, Advise)
        HRESULT ( STDMETHODCALLTYPE *Advise )( 
            ISimpleConnectionPoint * This,
            /* [in] */ IDispatch *pdisp,
            /* [out] */ DWORD *pdwCookie);
        
        DECLSPEC_XFGVIRT(ISimpleConnectionPoint, Unadvise)
        HRESULT ( STDMETHODCALLTYPE *Unadvise )( 
            ISimpleConnectionPoint * This,
            /* [in] */ DWORD dwCookie);
        
        END_INTERFACE
    } ISimpleConnectionPointVtbl;

    interface ISimpleConnectionPoint
    {
        CONST_VTBL struct ISimpleConnectionPointVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ISimpleConnectionPoint_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ISimpleConnectionPoint_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ISimpleConnectionPoint_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ISimpleConnectionPoint_GetEventCount(This,pulCount)	\
    ( (This)->lpVtbl -> GetEventCount(This,pulCount) ) 

#define ISimpleConnectionPoint_DescribeEvents(This,iEvent,cEvents,prgid,prgbstr,pcEventsFetched)	\
    ( (This)->lpVtbl -> DescribeEvents(This,iEvent,cEvents,prgid,prgbstr,pcEventsFetched) ) 

#define ISimpleConnectionPoint_Advise(This,pdisp,pdwCookie)	\
    ( (This)->lpVtbl -> Advise(This,pdisp,pdwCookie) ) 

#define ISimpleConnectionPoint_Unadvise(This,dwCookie)	\
    ( (This)->lpVtbl -> Unadvise(This,dwCookie) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ISimpleConnectionPoint_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_activdbg_0000_0056 */
/* [local] */ 

EXTERN_C const CLSID CLSID_DebugHelper;


extern RPC_IF_HANDLE __MIDL_itf_activdbg_0000_0056_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_activdbg_0000_0056_v0_0_s_ifspec;

#ifndef __IDebugHelper_INTERFACE_DEFINED__
#define __IDebugHelper_INTERFACE_DEFINED__

/* interface IDebugHelper */
/* [local][unique][helpstring][uuid][object] */ 


EXTERN_C const IID IID_IDebugHelper;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("51973C3f-CB0C-11d0-B5C9-00A0244A0E7A")
    IDebugHelper : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE CreatePropertyBrowser( 
            /* [in] */ VARIANT *pvar,
            /* [in] */ LPCOLESTR bstrName,
            /* [in] */ IDebugApplicationThread *pdat,
            /* [out] */ IDebugProperty **ppdob) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE CreatePropertyBrowserEx( 
            /* [in] */ VARIANT *pvar,
            /* [in] */ LPCOLESTR bstrName,
            /* [in] */ IDebugApplicationThread *pdat,
            /* [in] */ IDebugFormatter *pdf,
            /* [out] */ IDebugProperty **ppdob) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE CreateSimpleConnectionPoint( 
            /* [in] */ IDispatch *pdisp,
            /* [out] */ ISimpleConnectionPoint **ppscp) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IDebugHelperVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IDebugHelper * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IDebugHelper * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IDebugHelper * This);
        
        DECLSPEC_XFGVIRT(IDebugHelper, CreatePropertyBrowser)
        HRESULT ( STDMETHODCALLTYPE *CreatePropertyBrowser )( 
            IDebugHelper * This,
            /* [in] */ VARIANT *pvar,
            /* [in] */ LPCOLESTR bstrName,
            /* [in] */ IDebugApplicationThread *pdat,
            /* [out] */ IDebugProperty **ppdob);
        
        DECLSPEC_XFGVIRT(IDebugHelper, CreatePropertyBrowserEx)
        HRESULT ( STDMETHODCALLTYPE *CreatePropertyBrowserEx )( 
            IDebugHelper * This,
            /* [in] */ VARIANT *pvar,
            /* [in] */ LPCOLESTR bstrName,
            /* [in] */ IDebugApplicationThread *pdat,
            /* [in] */ IDebugFormatter *pdf,
            /* [out] */ IDebugProperty **ppdob);
        
        DECLSPEC_XFGVIRT(IDebugHelper, CreateSimpleConnectionPoint)
        HRESULT ( STDMETHODCALLTYPE *CreateSimpleConnectionPoint )( 
            IDebugHelper * This,
            /* [in] */ IDispatch *pdisp,
            /* [out] */ ISimpleConnectionPoint **ppscp);
        
        END_INTERFACE
    } IDebugHelperVtbl;

    interface IDebugHelper
    {
        CONST_VTBL struct IDebugHelperVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IDebugHelper_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IDebugHelper_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IDebugHelper_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IDebugHelper_CreatePropertyBrowser(This,pvar,bstrName,pdat,ppdob)	\
    ( (This)->lpVtbl -> CreatePropertyBrowser(This,pvar,bstrName,pdat,ppdob) ) 

#define IDebugHelper_CreatePropertyBrowserEx(This,pvar,bstrName,pdat,pdf,ppdob)	\
    ( (This)->lpVtbl -> CreatePropertyBrowserEx(This,pvar,bstrName,pdat,pdf,ppdob) ) 

#define IDebugHelper_CreateSimpleConnectionPoint(This,pdisp,ppscp)	\
    ( (This)->lpVtbl -> CreateSimpleConnectionPoint(This,pdisp,ppscp) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IDebugHelper_INTERFACE_DEFINED__ */


#ifndef __IEnumDebugExpressionContexts_INTERFACE_DEFINED__
#define __IEnumDebugExpressionContexts_INTERFACE_DEFINED__

/* interface IEnumDebugExpressionContexts */
/* [unique][helpstring][uuid][object] */ 


EXTERN_C const IID IID_IEnumDebugExpressionContexts;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("51973C40-CB0C-11d0-B5C9-00A0244A0E7A")
    IEnumDebugExpressionContexts : public IUnknown
    {
    public:
        virtual /* [local] */ HRESULT __stdcall Next( 
            /* [in] */ ULONG celt,
            /* [out] */ IDebugExpressionContext **ppdec,
            /* [out] */ ULONG *pceltFetched) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Skip( 
            /* [in] */ ULONG celt) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Reset( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Clone( 
            /* [out] */ __RPC__deref_out_opt IEnumDebugExpressionContexts **ppedec) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IEnumDebugExpressionContextsVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IEnumDebugExpressionContexts * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IEnumDebugExpressionContexts * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IEnumDebugExpressionContexts * This);
        
        DECLSPEC_XFGVIRT(IEnumDebugExpressionContexts, Next)
        /* [local] */ HRESULT ( __stdcall *Next )( 
            IEnumDebugExpressionContexts * This,
            /* [in] */ ULONG celt,
            /* [out] */ IDebugExpressionContext **ppdec,
            /* [out] */ ULONG *pceltFetched);
        
        DECLSPEC_XFGVIRT(IEnumDebugExpressionContexts, Skip)
        HRESULT ( STDMETHODCALLTYPE *Skip )( 
            __RPC__in IEnumDebugExpressionContexts * This,
            /* [in] */ ULONG celt);
        
        DECLSPEC_XFGVIRT(IEnumDebugExpressionContexts, Reset)
        HRESULT ( STDMETHODCALLTYPE *Reset )( 
            __RPC__in IEnumDebugExpressionContexts * This);
        
        DECLSPEC_XFGVIRT(IEnumDebugExpressionContexts, Clone)
        HRESULT ( STDMETHODCALLTYPE *Clone )( 
            __RPC__in IEnumDebugExpressionContexts * This,
            /* [out] */ __RPC__deref_out_opt IEnumDebugExpressionContexts **ppedec);
        
        END_INTERFACE
    } IEnumDebugExpressionContextsVtbl;

    interface IEnumDebugExpressionContexts
    {
        CONST_VTBL struct IEnumDebugExpressionContextsVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IEnumDebugExpressionContexts_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IEnumDebugExpressionContexts_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IEnumDebugExpressionContexts_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IEnumDebugExpressionContexts_Next(This,celt,ppdec,pceltFetched)	\
    ( (This)->lpVtbl -> Next(This,celt,ppdec,pceltFetched) ) 

#define IEnumDebugExpressionContexts_Skip(This,celt)	\
    ( (This)->lpVtbl -> Skip(This,celt) ) 

#define IEnumDebugExpressionContexts_Reset(This)	\
    ( (This)->lpVtbl -> Reset(This) ) 

#define IEnumDebugExpressionContexts_Clone(This,ppedec)	\
    ( (This)->lpVtbl -> Clone(This,ppedec) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */



/* [call_as] */ HRESULT __stdcall IEnumDebugExpressionContexts_RemoteNext_Proxy( 
    __RPC__in IEnumDebugExpressionContexts * This,
    /* [in] */ ULONG celt,
    /* [length_is][size_is][out] */ __RPC__out_ecount_part(celt, *pceltFetched) IDebugExpressionContext **pprgdec,
    /* [out] */ __RPC__out ULONG *pceltFetched);


void __RPC_STUB IEnumDebugExpressionContexts_RemoteNext_Stub(
    IRpcStubBuffer *This,
    IRpcChannelBuffer *_pRpcChannelBuffer,
    PRPC_MESSAGE _pRpcMessage,
    DWORD *_pdwStubPhase);



#endif 	/* __IEnumDebugExpressionContexts_INTERFACE_DEFINED__ */


#ifndef __IProvideExpressionContexts_INTERFACE_DEFINED__
#define __IProvideExpressionContexts_INTERFACE_DEFINED__

/* interface IProvideExpressionContexts */
/* [unique][uuid][object] */ 


EXTERN_C const IID IID_IProvideExpressionContexts;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("51973C41-CB0C-11d0-B5C9-00A0244A0E7A")
    IProvideExpressionContexts : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE EnumExpressionContexts( 
            /* [out] */ __RPC__deref_out_opt IEnumDebugExpressionContexts **ppedec) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IProvideExpressionContextsVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IProvideExpressionContexts * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IProvideExpressionContexts * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IProvideExpressionContexts * This);
        
        DECLSPEC_XFGVIRT(IProvideExpressionContexts, EnumExpressionContexts)
        HRESULT ( STDMETHODCALLTYPE *EnumExpressionContexts )( 
            __RPC__in IProvideExpressionContexts * This,
            /* [out] */ __RPC__deref_out_opt IEnumDebugExpressionContexts **ppedec);
        
        END_INTERFACE
    } IProvideExpressionContextsVtbl;

    interface IProvideExpressionContexts
    {
        CONST_VTBL struct IProvideExpressionContextsVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IProvideExpressionContexts_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IProvideExpressionContexts_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IProvideExpressionContexts_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IProvideExpressionContexts_EnumExpressionContexts(This,ppedec)	\
    ( (This)->lpVtbl -> EnumExpressionContexts(This,ppedec) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IProvideExpressionContexts_INTERFACE_DEFINED__ */



#ifndef __ProcessDebugManagerLib_LIBRARY_DEFINED__
#define __ProcessDebugManagerLib_LIBRARY_DEFINED__

/* library ProcessDebugManagerLib */
/* [helpstring][version][uuid] */ 



























































EXTERN_C const CLSID CLSID_CDebugDocumentHelper;
#ifdef DEBUG
#define MachineDebugManger MachineDebugManager_DEBUG
#define CLSID_MachineDebugManager CLSID_MachineDebugManager_DEBUG
#else
#define MachineDebugManger MachineDebugManager_RETAIL
#define CLSID_MachineDebugManager CLSID_MachineDebugManager_RETAIL
#endif

EXTERN_C const IID LIBID_ProcessDebugManagerLib;

EXTERN_C const CLSID CLSID_ProcessDebugManager;

#ifdef __cplusplus

class DECLSPEC_UUID("78a51822-51f4-11d0-8f20-00805f2cd064")
ProcessDebugManager;
#endif

EXTERN_C const CLSID CLSID_DebugHelper;

#ifdef __cplusplus

class DECLSPEC_UUID("0BFCC060-8C1D-11d0-ACCD-00AA0060275C")
DebugHelper;
#endif

EXTERN_C const CLSID CLSID_CDebugDocumentHelper;

#ifdef __cplusplus

class DECLSPEC_UUID("83B8BCA6-687C-11D0-A405-00AA0060275C")
CDebugDocumentHelper;
#endif

EXTERN_C const CLSID CLSID_MachineDebugManager_RETAIL;

#ifdef __cplusplus

class DECLSPEC_UUID("0C0A3666-30C9-11D0-8F20-00805F2CD064")
MachineDebugManager_RETAIL;
#endif

EXTERN_C const CLSID CLSID_MachineDebugManager_DEBUG;

#ifdef __cplusplus

class DECLSPEC_UUID("49769CEC-3A55-4bb0-B697-88FEDE77E8EA")
MachineDebugManager_DEBUG;
#endif

EXTERN_C const CLSID CLSID_DefaultDebugSessionProvider;

#ifdef __cplusplus

class DECLSPEC_UUID("834128a2-51f4-11d0-8f20-00805f2cd064")
DefaultDebugSessionProvider;
#endif
#endif /* __ProcessDebugManagerLib_LIBRARY_DEFINED__ */

/* interface __MIDL_itf_activdbg_0000_0060 */
/* [local] */ 


#endif  // __ActivDbg_h

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion


extern RPC_IF_HANDLE __MIDL_itf_activdbg_0000_0060_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_activdbg_0000_0060_v0_0_s_ifspec;

/* Additional Prototypes for ALL interfaces */

unsigned long             __RPC_USER  BSTR_UserSize(     __RPC__in unsigned long *, unsigned long            , __RPC__in BSTR * ); 
unsigned char * __RPC_USER  BSTR_UserMarshal(  __RPC__in unsigned long *, __RPC__inout_xcount(0) unsigned char *, __RPC__in BSTR * ); 
unsigned char * __RPC_USER  BSTR_UserUnmarshal(__RPC__in unsigned long *, __RPC__in_xcount(0) unsigned char *, __RPC__out BSTR * ); 
void                      __RPC_USER  BSTR_UserFree(     __RPC__in unsigned long *, __RPC__in BSTR * ); 

unsigned long             __RPC_USER  BSTR_UserSize64(     __RPC__in unsigned long *, unsigned long            , __RPC__in BSTR * ); 
unsigned char * __RPC_USER  BSTR_UserMarshal64(  __RPC__in unsigned long *, __RPC__inout_xcount(0) unsigned char *, __RPC__in BSTR * ); 
unsigned char * __RPC_USER  BSTR_UserUnmarshal64(__RPC__in unsigned long *, __RPC__in_xcount(0) unsigned char *, __RPC__out BSTR * ); 
void                      __RPC_USER  BSTR_UserFree64(     __RPC__in unsigned long *, __RPC__in BSTR * ); 

/* [local] */ HRESULT __stdcall IEnumDebugCodeContexts_Next_Proxy( 
    IEnumDebugCodeContexts * This,
    /* [in] */ ULONG celt,
    /* [out] */ IDebugCodeContext **pscc,
    /* [out] */ ULONG *pceltFetched);


/* [call_as] */ HRESULT __stdcall IEnumDebugCodeContexts_Next_Stub( 
    __RPC__in IEnumDebugCodeContexts * This,
    /* [in] */ ULONG celt,
    /* [length_is][size_is][out] */ __RPC__out_ecount_part(celt, *pceltFetched) IDebugCodeContext **pscc,
    /* [out] */ __RPC__out ULONG *pceltFetched);

/* [local] */ HRESULT __stdcall IEnumDebugStackFrames_Next_Proxy( 
    IEnumDebugStackFrames * This,
    /* [in] */ ULONG celt,
    /* [out] */ DebugStackFrameDescriptor *prgdsfd,
    /* [out] */ ULONG *pceltFetched);


/* [call_as] */ HRESULT __stdcall IEnumDebugStackFrames_Next_Stub( 
    __RPC__in IEnumDebugStackFrames * This,
    /* [in] */ ULONG celt,
    /* [length_is][size_is][out] */ __RPC__out_ecount_part(celt, *pceltFetched) DebugStackFrameDescriptor *prgdsfd,
    /* [out] */ __RPC__out ULONG *pceltFetched);

/* [local] */ HRESULT __stdcall IEnumDebugStackFrames64_Next64_Proxy( 
    IEnumDebugStackFrames64 * This,
    /* [in] */ ULONG celt,
    /* [out] */ DebugStackFrameDescriptor64 *prgdsfd,
    /* [out] */ ULONG *pceltFetched);


/* [call_as] */ HRESULT __stdcall IEnumDebugStackFrames64_Next64_Stub( 
    __RPC__in IEnumDebugStackFrames64 * This,
    /* [in] */ ULONG celt,
    /* [length_is][size_is][out] */ __RPC__out_ecount_part(celt, *pceltFetched) DebugStackFrameDescriptor64 *prgdsfd,
    /* [out] */ __RPC__out ULONG *pceltFetched);

/* [local] */ HRESULT __stdcall IEnumDebugApplicationNodes_Next_Proxy( 
    IEnumDebugApplicationNodes * This,
    /* [in] */ ULONG celt,
    /* [out] */ IDebugApplicationNode **pprddp,
    /* [out] */ ULONG *pceltFetched);


/* [call_as] */ HRESULT __stdcall IEnumDebugApplicationNodes_Next_Stub( 
    __RPC__in IEnumDebugApplicationNodes * This,
    /* [in] */ ULONG celt,
    /* [length_is][size_is][out] */ __RPC__out_ecount_part(celt, *pceltFetched) IDebugApplicationNode **pprddp,
    /* [out] */ __RPC__out ULONG *pceltFetched);

/* [local] */ HRESULT __stdcall IEnumRemoteDebugApplications_Next_Proxy( 
    IEnumRemoteDebugApplications * This,
    /* [in] */ ULONG celt,
    /* [out] */ IRemoteDebugApplication **ppda,
    /* [out] */ ULONG *pceltFetched);


/* [call_as] */ HRESULT __stdcall IEnumRemoteDebugApplications_Next_Stub( 
    __RPC__in IEnumRemoteDebugApplications * This,
    /* [in] */ ULONG celt,
    /* [length_is][size_is][out] */ __RPC__out_ecount_part(celt, *pceltFetched) IRemoteDebugApplication **ppda,
    /* [out] */ __RPC__out ULONG *pceltFetched);

/* [local] */ HRESULT __stdcall IEnumRemoteDebugApplicationThreads_Next_Proxy( 
    IEnumRemoteDebugApplicationThreads * This,
    /* [in] */ ULONG celt,
    /* [out] */ IRemoteDebugApplicationThread **pprdat,
    /* [out] */ ULONG *pceltFetched);


/* [call_as] */ HRESULT __stdcall IEnumRemoteDebugApplicationThreads_Next_Stub( 
    __RPC__in IEnumRemoteDebugApplicationThreads * This,
    /* [in] */ ULONG celt,
    /* [length_is][size_is][out] */ __RPC__out_ecount_part(celt, *pceltFetched) IRemoteDebugApplicationThread **ppdat,
    /* [out] */ __RPC__out ULONG *pceltFetched);

/* [local] */ HRESULT __stdcall IEnumDebugExpressionContexts_Next_Proxy( 
    IEnumDebugExpressionContexts * This,
    /* [in] */ ULONG celt,
    /* [out] */ IDebugExpressionContext **ppdec,
    /* [out] */ ULONG *pceltFetched);


/* [call_as] */ HRESULT __stdcall IEnumDebugExpressionContexts_Next_Stub( 
    __RPC__in IEnumDebugExpressionContexts * This,
    /* [in] */ ULONG celt,
    /* [length_is][size_is][out] */ __RPC__out_ecount_part(celt, *pceltFetched) IDebugExpressionContext **pprgdec,
    /* [out] */ __RPC__out ULONG *pceltFetched);



/* end of Additional Prototypes */

#ifdef __cplusplus
}
#endif

#endif


