/***
* ==++==
*
* Copyright (c) Microsoft Corporation.  All rights reserved.
*
* ==--==
* =+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+
*
* concrtrm.h
*
* Main public header file for ConcRT's Resource Manager. This is the only header file a client
* must include to build atop the resource manager.
*
* The core runtime, the Agents and Message Blocks Library, and the Parallel Patterns Library (PPL)
* are defined in different header files.
* =-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-
****/

#pragma once

#include <crtdefs.h>

#if !(defined (_M_X64) || defined (_M_IX86) || defined (_M_ARM) || defined (_M_ARM64))
    #error ERROR: Concurrency Runtime is supported only on X64, X86, ARM, and ARM64 architectures.
#endif  /* !(defined (_M_X64) || defined (_M_IX86) || defined (_M_ARM) || defined (_M_ARM64)) */

#if defined (_M_CEE)
    #error ERROR: Concurrency Runtime is not supported when compiling /clr.
#endif  /* defined (_M_CEE) */

#ifndef __cplusplus
    #error ERROR: Concurrency Runtime is supported only for C++.
#endif  /* __cplusplus */

#pragma pack(push,_CRT_PACKING)

/// <summary>
///     The <c>Concurrency</c> namespace provides classes and functions that give you access to the Concurrency Runtime,
///     a concurrent programming framework for C++. For more information, see <see cref="Concurrency Runtime"/>.
/// </summary>
/**/
namespace Concurrency
{
#ifdef _CRT_USE_WINAPI_FAMILY_DESKTOP_APP
    //
    // Forward Declarations:
    //
    struct IScheduler;
    struct IThreadProxy;
    class SchedulerPolicy;

    /// <summary>
    ///     Used to denote the state a thread proxy is in, when it is executing a cooperative context switch to a different thread
    ///     proxy.
    /// </summary>
    /// <remarks>
    ///     A parameter of type <c>SwitchingProxyState</c> is passed in to the method <c>IThreadProxy::SwitchTo</c> to
    ///     instruct the Resource Manager how to treat the thread proxy that is making the call.
    ///     <para>For more information on how this type is used, see <see cref="IThreadProxy::SwitchTo Method">IThreadProxy::SwitchTo
    ///     </see>.</para>
    /// </remarks>
    /**/
    enum SwitchingProxyState
    {
        /// <summary>
        ///     Indicates that the calling thread is no longer needed by the scheduler and is being returned to the Resource Manager.  The
        ///     context which was being dispatched is no longer able to be utilized by the Resource Manager.
        /// </summary>
        /**/
        Idle,

        /// <summary>
        ///     Indicates that the calling thread is cooperatively blocking and should be exclusively owned by the caller until subsequently
        ///     running again and performing other action.
        /// </summary>
        /**/
        Blocking,

        /// <summary>
        ///     Indicates that the calling thread is nesting a child scheduler and is needed by the caller, in order to attach to a
        ///     different scheduler.
        /// </summary>
        /**/
        Nesting
    };

    /// <summary>
    ///     The <c>DispatchState</c> structure is used to transfer state to the <c>IExecutionContext::Dispatch</c> method. It describes
    ///     the circumstances under which the <c>Dispatch</c> method is invoked on an <c>IExecutionContext</c> interface.
    /// </summary>
    /// <seealso cref="IExecutionContext::Dispatch Method"/>
    /**/
    struct DispatchState
    {
        /// <summary>
        ///     Constructs a new <c>DispatchState</c> object.
        /// </summary>
        /**/
        DispatchState() : m_dispatchStateSize(sizeof(DispatchState)), m_fIsPreviousContextAsynchronouslyBlocked(0), m_reserved(0)
        {
        }

        /// <summary>
        ///     Size of this structure, which is used for versioning.
        /// </summary>
        /**/
        unsigned long m_dispatchStateSize;

        /// <summary>
        ///     Tells whether this context has entered the <c>Dispatch</c> method because the previous context asynchronously blocked.
        ///     This is set to the value <c>0</c> for all execution contexts.
        /// </summary>
        /// <seealso cref="IExecutionContext::Dispatch Method"/>
        /**/
        unsigned int  m_fIsPreviousContextAsynchronouslyBlocked : 1;

        /// <summary>
        ///     Bits reserved for future information passing.
        /// </summary>
        /// <seealso cref="IExecutionContext::Dispatch Method"/>
        /**/
        unsigned int  m_reserved : 31;
    };

    /// <summary>
    ///     An interface to an execution context which can run on a given virtual processor and be cooperatively context switched.
    /// </summary>
    /// <remarks>
    ///     If you are implementing a custom scheduler that interfaces with the Concurrency Runtime's Resource Manager, you will need
    ///     to implement the <c>IExecutionContext</c> interface. The threads created by the Resource Manager perform work on behalf
    ///     of your scheduler by executing the <c>IExecutionContext::Dispatch</c> method.
    /// </remarks>
    /// <seealso cref="IScheduler Structure"/>
    /// <seealso cref="IThreadProxy Structure"/>
    /**/
    struct IExecutionContext
    {
        /// <summary>
        ///     Returns a unique identifier for the execution context.
        /// </summary>
        /// <returns>
        ///     A unique integer identifier.
        /// </returns>
        /// <remarks>
        ///     You should use the method <c>GetExecutionContextId</c> to obtain a unique identifier for the object that implements the
        ///     <c>IExecutionContext</c> interface, before you use the interface as a parameter to methods supplied by the Resource Manager.
        ///     You are expected to return the same identifier when the <c>GetId</c> function is invoked. <para> An identifier obtained from a different
        ///     source could result in undefined behavior.</para>
        /// </remarks>
        /// <seealso cref="GetExecutionContextId Function"/>
        /**/
        virtual unsigned int GetId() const =0;

        /// <summary>
        ///     Returns an interface to the scheduler this execution context belongs to.
        /// </summary>
        /// <returns>
        ///     An <c>IScheduler</c> interface.
        /// </returns>
        /// <remarks>
        ///     You are required to initialize the execution context with a valid <c>IScheduler</c> interface before you use it as a parameter to
        ///     methods supplied by the Resource Manager.
        /// </remarks>
        /**/
        virtual IScheduler * GetScheduler() =0;

        /// <summary>
        ///     Returns an interface to the thread proxy that is executing this context.
        /// </summary>
        /// <returns>
        ///     An <c>IThreadProxy</c> interface. If the execution context's thread proxy has not been initialized with a call to <c>SetProxy</c>,
        ///     the function must return <c>NULL</c>.
        /// </returns>
        /// <remarks>
        ///     The Resource Manager will invoke the <c>SetProxy</c> method on an execution context, with an <c>IThreadProxy</c> interface
        ///     as a parameter, prior to entering the <c>Dispatch</c> method on the on the context. You are expected to store this argument and return it
        ///     on calls to <c>GetProxy()</c>.
        /// </remarks>
        /// <seealso cref="IExecutionContext::SetProxy Method"/>
        /**/
        virtual IThreadProxy * GetProxy() =0;

        /// <summary>
        ///     Associates a thread proxy with this execution context. The associated thread proxy invokes this method right before it starts
        ///     executing the context's <c>Dispatch</c> method.
        /// </summary>
        /// <param name="pThreadProxy">
        ///     An interface to the thread proxy that is about to enter the <c>Dispatch</c> method on this execution context.
        /// </param>
        /// <remarks>
        ///     You are expected to save the parameter <paramref name="pThreadProxy"/> and return it on a call to the <c>GetProxy</c> method.
        ///     The Resource Manager guarantees that the thread proxy associated with the execution context will not change while the
        ///     thread proxy is executing the <c>Dispatch</c> method.
        /// </remarks>
        /// <seealso cref="IExecutionContext::GetProxy Method"/>
        /**/
        virtual void SetProxy(_Inout_ IThreadProxy * pThreadProxy) =0;

        /// <summary>
        ///     The method that is called when a thread proxy starts executing a particular execution context. This should be the main worker
        ///     routine for your scheduler.
        /// </summary>
        /// <param name="pDispatchState">
        ///     A pointer to the state under which this execution context is being dispatched. For more information on dispatch state, see
        ///     <see cref="DispatchState Structure">DispatchState</see>.
        /// </param>
        /**/
        virtual void Dispatch(_Inout_ DispatchState * pDispatchState) =0;
    };

    /// <summary>
    ///     An abstraction for a thread of execution.
    ///     The Resource Manager will grant you a thread proxy that is backed by a regular Win32 thread.
    /// </summary>
    /// <remarks>
    ///     Thread proxies are coupled to execution contexts represented by the interface <c>IExecutionContext</c> as a means of dispatching work.
    /// </remarks>
    /// <seealso cref="IExecutionContext Structure"/>
    /// <seealso cref="IScheduler Structure"/>
    /// <seealso cref="IVirtualProcessorRoot Structure"/>
    /**/
    struct IThreadProxy
    {
        /// <summary>
        ///     Returns a unique identifier for the thread proxy.
        /// </summary>
        /// <returns>
        ///     A unique integer identifier.
        /// </returns>
        /**/
        virtual unsigned int GetId() const =0;

        /// <summary>
        ///     Performs a cooperative context switch from the currently executing context to a different one.
        /// </summary>
        /// <param name="pContext">
        ///     The execution context to cooperatively switch to.
        /// </param>
        /// <param name="switchState">
        ///     Indicates the state of the thread proxy that is executing the switch. The parameter is of type <typeparamref name="SwitchingProxyState"/>.
        /// </param>
        /// <remarks>
        ///     Use this method to switch from one execution context to another, from the <see cref="IExecutionContext::Dispatch Method">
        ///     IExecutionContext::Dispatch </see> method of the first execution context.
        ///     The method associates the execution context <paramref name="pContext"/> with a thread proxy if it is not already associated
        ///     with one. The ownership of the current thread proxy is determined by the value you specify for the <paramref name="switchState"/>
        ///     argument.
        ///
        ///     <para> Use the value <c>Idle</c> when you want to return the currently executing thread proxy to the Resource Manager.
        ///     Calling <c>SwitchTo</c> with the parameter <paramref name="switchState"/> set to <c>Idle</c> will cause
        ///     the execution context <paramref name="pContext"/> to start executing on the underlying execution resource. Ownership of
        ///     this thread proxy is transferred to the Resource Manager, and you are expected to return from the execution context's
        ///     <c>Dispatch</c> method soon after <c>SwitchTo</c> returns, in order to complete the transfer. The execution context that the
        ///     thread proxy was dispatching is disassociated from the thread proxy, and the scheduler is free to reuse it or destroy it
        ///     as it sees fit.</para>
        ///
        ///     <para> Use the value <c>Blocking</c> when you want this thread proxy to enter a blocked state. Calling
        ///     <c>SwitchTo</c> with the parameter <paramref name="switchState"/> set to <c>Blocking</c> will cause the execution context
        ///     <paramref name="pContext"/> to start executing, and block the current thread proxy until it is resumed. The scheduler retains
        ///     ownership of the thread proxy when the thread proxy is in the <c>Blocking</c> state. The blocking thread proxy
        ///     can be resumed by calling the function <c>SwitchTo</c> to switch to this thread proxy's execution context. You can also
        ///     resume the thread proxy, by using its associated context to activate a virtual processor root. For more information on how
        ///     to do this, see <see cref="IVirtualProcessorRoot::Activate Method"> IVirtualProcessorRoot::Activate</see>.</para>
        ///
        ///     <para> Use the value <c>Nesting</c> when you want to temporarily detach this thread proxy from the virtual processor root
        ///     it is running on, and the scheduler it is dispatching work for. Calling <c>SwitchTo</c> with the parameter <paramref name="switchState"/>
        ///     set to <c>Nesting</c> will cause the execution context <paramref name="pContext"/> to start executing and the
        ///     current thread proxy also continues executing without the need for a virtual processor root. The thread proxy is considered
        ///     to have left the scheduler until it calls the <see cref="IThreadProxy::SwitchOut Method">IThreadProxy::SwitchOut</see>
        ///     method at a later point in time. The <c>IThreadProxy::SwitchOut</c> method could block the thread proxy until a virtual
        ///     processor root is available to reschedule it.</para>
        ///     <para><c>SwitchTo</c> must be called on the <c>IThreadProxy</c> interface that represents the currently executing thread
        ///     or the results are undefined. The function throws <c>invalid_argument</c> if the parameter <paramref name="pContext"/>
        ///     is set to <c>NULL</c>.</para>
        /// </remarks>
        /// <seealso cref="SwitchingProxyState Enumeration"/>
        /**/
        virtual void SwitchTo(_Inout_ IExecutionContext * pContext, SwitchingProxyState switchState) =0;

        /// <summary>
        ///     Disassociates the context from the underlying virtual processor root.
        /// </summary>
        /// <param name="switchState">
        ///     Indicates the state of the thread proxy that is executing the switch. The parameter is of type <typeparamref name="SwitchingProxyState"/>.
        /// </param>
        /// <remarks>
        ///     Use <c>SwitchOut</c> if you need to disassociate a context from the virtual processor root it is executing on, for any reason. Depending
        ///     on the value you pass in to the parameter <paramref name="switchState"/>, and whether or not it is executing on a virtual processor root,
        ///     the call will either return immediately or block the thread proxy associated with the context. It is an error to call <c>SwitchOut</c> with
        ///     the parameter set to <c>Idle</c>. Doing so will result in an <see cref="invalid_argument Class">invalid_argument</see> exception.
        ///
        ///     <para><c>SwitchOut</c> is useful when you want to reduce the number of virtual processor roots your scheduler has, either because the Resource
        ///     Manager has instructed you to do so, or because you requested a temporary oversubscribed virtual processor root, and are done with it.
        ///     In this case you should invoke the method <see cref="IVirtualProcessorRoot::Remove Method"/> on the virtual processor root, before making
        ///     a call to <c>SwitchOut</c> with the parameter <paramref name="switchState"/> set to <c>Blocking</c>. This will block the thread proxy and it
        ///     will resume execution when a different virtual processor root in the scheduler is available to execute it. The blocking thread proxy can be
        ///     resumed by calling the function <c>SwitchTo</c> to switch to this thread proxy's execution context. You can also resume the thread proxy,
        ///     by using its associated context to activate a virtual processor root. For more information on how to do this, see
        ///     <see cref="IVirtualProcessorRoot::Activate Method"> IVirtualProcessorRoot::Activate</see>.</para>
        ///
        ///     <para><c>SwitchOut</c> may also be used when you want reinitialize the virtual processor so it may be activated in the future while either
        ///     blocking the thread proxy or temporarily detaching it from the virtual processor root it is running on, and the scheduler it is dispatching
        ///     work for. Use <c>SwitchOut</c> with the parameter <paramref name="switchState"/> set to <c>Blocking</c> if you wish to block the thread proxy.
        ///     It can later be resumed using either <c>SwitchTo</c> or <c>IVirtualProcessorRoot::Activate</c> as noted above. Use <c>SwitchOut</c> with the
        ///     parameter set to <c>Nesting</c> when you want to temporarily detach this thread proxy from the virtual processor root it is running on,
        ///     and the scheduler the virtual processor is associated with. Calling <c>SwitchOut</c> with the parameter <paramref name="switchState"/>
        ///     set to <c>Nesting</c> while it is executing on a virtual processor root will cause the root to be reinitialized and the current thread proxy
        ///     to continue executing without the need for one. The thread proxy is considered to have left the scheduler until it calls the
        ///     <see cref="IThreadProxy::SwitchOut Method">IThreadProxy::SwitchOut</see> method with <c>Blocking</c> at a later point in time. The second
        ///     call to <c>SwitchOut</c> with the parameter set to <c>Blocking</c> is intended to return the context to a blocked state so that it can be
        ///     resumed by either <c>SwitchTo</c> or <c>IVirtualProcessorRoot::Activate</c> in the scheduler it detached from. Because it was not executing
        ///     on a virtual processor root, no reinitialization takes place.</para>
        ///
        ///     <para>A reinitialized virtual processor root is no different from a brand new virtual processor root your scheduler has been granted by the Resource
        ///     Manager. You can use it for execution by activating it with an execution context using <c>IVirtualProcessorRoot::Activate</c>.</para>
        ///
        ///     <para><c>SwitchOut</c> must be called on the <c>IThreadProxy</c> interface that represents the currently executing thread
        ///     or the results are undefined.</para>
        ///
        ///     <para>In the libraries and headers that shipped with Visual Studio 2010, this method did not take a parameter and did not reinitialize the
        ///     virtual processor root. To preserve old behavior when you upgrade to Visual Studio 2012, the default parameter value of <c>Blocking</c> is supplied.</para>
        /// </remarks>
        /**/
        virtual void SwitchOut(SwitchingProxyState switchState = Blocking) =0;

        /// <summary>
        ///     Causes the calling thread to yield execution to another thread that is ready to run on the current processor. The operating
        ///     system selects the next thread to be executed.
        /// </summary>
        /// <remarks>
        ///     When called by a thread proxy backed by a regular Windows thread, <c>YieldToSystem</c> behaves exactly like the Windows function
        ///     <c>SwitchToThread</c>.
        ///     <para><c>YieldToSystem</c> must be called on the <c>IThreadProxy</c> interface that represents the currently executing thread
        ///     or the results are undefined.</para>
        /// </remarks>
        /**/
        virtual void YieldToSystem() = 0;
    };

    /// <summary>
    ///    The type of critical region a context is inside.
    /// </summary>
    /**/
    enum CriticalRegionType
    {
        /// <summary>
        ///     Indicates that the context is outside any critical region.
        /// </summary>
        /**/
        OutsideCriticalRegion,

        /// <summary>
        ///     Indicates that the context is inside a critical region.  When inside a critical region, asynchronous suspensions are hidden from
        ///     the scheduler.  Should such a suspension happen, the Resource Manager will wait for the thread to become runnable and simply resume it instead
        ///     of invoking the scheduler again.  Any locks taken inside such a region must be taken with extreme care.
        /// </summary>
        /**/
        InsideCriticalRegion,

        /// <summary>
        ///     Indicates that the context is inside a hyper-critical region.  When inside a hyper-critical region, both synchronous and asynchronous
        ///     suspensions are hidden from the scheduler.  Should such a suspension or blocking happen, the resource manager will wait for the thread to
        ///     become runnable and simply resume it instead of invoking the scheduler again.  Locks taken inside such a region must never be shared with
        ///     code running outside such a region.  Doing so will cause unpredictable deadlock.
        /// </summary>
        /**/
        InsideHyperCriticalRegion
    };

    /// <summary>
    ///     An abstraction for a hardware thread.
    /// </summary>
    /// <remarks>
    ///     Execution resources can be standalone or associated with virtual processor roots. A standalone execution resource is created when
    ///     a thread in your application creates a thread subscription. The methods <see cref="ISchedulerProxy::SubscribeCurrentThread Method">
    ///     ISchedulerProxy::SubscribeThread</see> and <see cref="ISchedulerProxy::RequestInitialVirtualProcessors Method">
    ///     ISchedulerProxy::RequestInitialVirtualProcessors</see> create thread subscriptions, and return an <c>IExecutionResource</c> interface
    ///     representing the subscription. Creating a thread subscription is a way to inform the Resource Manager that a given thread will participate
    ///     in the work queued to a scheduler, along with the virtual processor roots Resource Manager assigns to the scheduler.
    ///     The Resource Manager uses the information to avoid oversubscribing hardware threads where it can.
    /// </remarks>
    /// <seealso cref="IVirtualProcessorRoot Structure"/>
    /// <seealso cref="ISchedulerProxy::SubscribeCurrentThread Method"/>
    /// <seealso cref="ISchedulerProxy::RequestInitialVirtualProcessors Method"/>
    /**/
    struct IExecutionResource
    {
        /// <summary>
        ///     Returns a unique identifier for the processor node that this execution resource belongs to.
        /// </summary>
        /// <returns>
        ///     A unique identifier for a processor node.
        /// </returns>
        /// <remarks>
        ///     The Concurrency Runtime represents hardware threads on the system in groups of processor nodes. Nodes are usually derived from
        ///     the hardware topology of the system. For example, all processors on a specific socket or a specific NUMA node may belong to the
        ///     same processor node. The Resource Manager assigns unique identifiers to these nodes starting with <c>0</c> up to and including
        ///     <c>nodeCount - 1</c>, where <c>nodeCount</c> represents the total number of processor nodes on the system.
        ///     <para>The count of nodes can be obtained from the function <see cref="GetProcessorNodeCount Function">GetProcessorNodeCount</see>.</para>
        /// </remarks>
        /**/
        virtual unsigned int GetNodeId() const =0;

        /// <summary>
        ///     Returns a unique identifier for the hardware thread that this execution resource represents.
        /// </summary>
        /// <returns>
        ///     A unique identifier for the hardware thread underlying this execution resource.
        /// </returns>
        /// <remarks>
        ///     Each hardware thread is assigned a unique identifier by the Concurrency Runtime. If multiple execution resources are associated
        ///     hardware thread, they will all have the same execution resource identifier.
        /// </remarks>
        /**/
        virtual unsigned int GetExecutionResourceId() const =0;

        /// <summary>
        ///     Returns this execution resource to the Resource Manager.
        /// </summary>
        /// <param name="pScheduler">
        ///     An interface to the scheduler making the request to remove this execution resource.
        /// </param>
        /// <remarks>
        ///     Use this method to return standalone execution resources as well as execution resources associated with virtual processor roots to
        ///     the Resource Manager.
        ///     <para>If this is a standalone execution resource you received from either of the methods <see cref="ISchedulerProxy::SubscribeCurrentThread Method">
        ///     ISchedulerProxy::SubscribeCurrentThread</see> or <see cref="ISchedulerProxy::RequestInitialVirtualProcessors Method">
        ///     ISchedulerProxy::RequestInitialVirtualProcessors</see>, calling the method <c>Remove</c> will end the thread subscription that the
        ///     resource was created to represent. You are required to end all thread subscriptions before shutting down a scheduler proxy, and must
        ///     call <c>Remove</c> from the thread that created the subscription.</para>
        ///     <para>Virtual processor roots, too, can be returned to the Resource Manager by invoking the <c>Remove</c> method, because the interface
        ///     <c>IVirtualProcessorRoot</c> inherits from the <c>IExecutionResource</c> interface. You may need to return a virtual processor root either
        ///     in response to a call to the <see cref="IScheduler::RemoveVirtualProcessors Method">IScheduler::RemoveVirtualProcessors</see>
        ///     method, or when you are done with an oversubscribed virtual processor root you obtained from the <see cref="ISchedulerProxy::CreateOversubscriber Method">
        ///     ISchedulerProxy::CreateOversubscriber</see> method. For virtual processor roots, there are no restrictions on which thread can invoke
        ///     the <c>Remove</c> method.</para>
        ///     <para><c>invalid_argument</c> is thrown if the parameter <paramref name="pScheduler"/> is set to <c>NULL</c>.</para>
        ///     <para><c>invalid_operation</c> is thrown if the parameter <paramref name="pScheduler"/> is different from the scheduler that this
        ///     execution resource was created for, or, with a standalone execution resource, if the current thread is different from the
        ///     thread that created the thread subscription.</para>
        /// </remarks>
        /// <seealso cref="invalid_argument Class"/>
        /// <seealso cref="invalid_operation Class"/>
        /**/
        virtual void Remove(_Inout_ IScheduler * pScheduler) =0;

        /// <summary>
        ///     Returns the number of activated virtual processor roots and subscribed external threads currently associated with the underlying
        ///     hardware thread this execution resource represents.
        /// </summary>
        /// <returns>
        ///     The current subscription level.
        /// </returns>
        /// <remarks>
        ///     The subscription level tells you how many running threads are associated with the hardware thread. This only includes threads
        ///     the Resource Manager is aware of in the form of subscribed threads, and virtual processor roots that are actively executing
        ///     thread proxies.
        ///     <para>Calling the method <see cref="ISchedulerProxy::SubscribeCurrentThread Method">ISchedulerProxy::SubscribeCurrentThread</see>,
        ///     or the method <see cref="ISchedulerProxy::RequestInitialVirtualProcessors Method">ISchedulerProxy::RequestInitialVirtualProcessors
        ///     </see> with the parameter <paramref name="doSubscribeCurrentThread"/> set to the value <c>true</c> increments the subscription
        ///     level of a hardware thread by one. They also return an <c>IExecutionResource</c> interface representing the subscription.  A
        ///     corresponding call to the <see cref="IExecutionResource::Remove Method"> IExecutionResource::Remove</see> decrements the
        ///     hardware thread's subscription level by one.</para>
        ///     <para>The act of activating a virtual processor root using the method <see cref="IVirtualProcessorRoot::Activate Method">
        ///     IVirtualProcessorRoot::Activate</see> increments the subscription level of a hardware thread by one. The methods
        ///     <see cref="IVirtualProcessorRoot::Deactivate Method">IVirtualProcessorRoot::Deactivate</see>, or
        ///     <see cref="IExecutionResource::Remove Method">IExecutionResource::Remove</see> decrement the subscription level by one
        ///     when invoked on an activated virtual processor root.</para>
        ///     <para>The Resource Manager uses subscription level information as one of the ways in which to determine when to move resources
        ///     between schedulers.</para>
        /// </remarks>
        /**/
        virtual unsigned int CurrentSubscriptionLevel() const =0;
    };

    /// <summary>
    ///     An abstraction for a hardware thread on which a thread proxy can execute.
    /// </summary>
    /// <remarks>
    ///     Every virtual processor root has an associated execution resource. The <c>IVirtualProcessorRoot</c> interface inherits from the
    ///     <see cref="IExecutionResource Structure">IExecutionResource</see> interface. Multiple virtual processor roots may correspond to the same
    ///     underlying hardware thread.
    ///     <para>The Resource Manager grants virtual processor roots to schedulers in response to requests for resources. A scheduler can use
    ///     a virtual processor root to perform work by activating it with an execution context.</para>
    /// </remarks>
    /**/
    struct IVirtualProcessorRoot : public IExecutionResource
    {
        /// <summary>
        ///     Returns a unique identifier for the virtual processor root.
        /// </summary>
        /// <returns>
        ///     An integer identifier.
        /// </returns>
        /**/
        virtual unsigned int GetId() const =0;

        /// <summary>
        ///     Causes the thread proxy associated with the execution context interface <paramref name="pContext"/> to start executing on this
        ///     virtual processor root.
        /// </summary>
        /// <param name="pContext">
        ///     An interface to the execution context that will be dispatched on this virtual processor root.
        /// </param>
        /// <remarks>
        ///     The Resource Manager will supply a thread proxy if one is not associated with the execution context interface <paramref name="pContext"/>
        ///     <para>The <c>Activate</c> method can be used to start executing work on a new virtual processor root returned by the Resource Manager, or to resume
        ///     the thread proxy on a virtual processor root that has deactivated or is about to deactivate. See <see cref="IVirtualProcessorRoot::Deactivate Method">
        ///     IVirtualProcessorRoot::Deactivate</see> for more information on deactivation. When you are resuming a deactivated virtual processor
        ///     root, the parameter <paramref name="pContext"/> must be the same as the parameter used to deactivate the virtual processor root.</para>
        ///     <para> Once a virtual processor root has been activated for the first time, subsequent pairs of calls to <c>Deactivate</c> and
        ///     <c>Activate</c> may race with each other. This means it is acceptable for the Resource Manager to receive a call to <c>Activate</c>
        ///     before it receives the <c>Deactivate</c> call it was meant for.</para>
        ///     <para>When you activate a virtual processor root, you signal to the Resource Manager that this virtual processor root is currently
        ///     busy with work. If your scheduler cannot find any work to execute on this root, it is expected to invoke the <c>Deactivate</c> method
        ///     informing the Resource Manager that the virtual processor root is idle. The Resource Manager uses this data to
        ///     load balance the system.</para>
        ///     <para><c>invalid_argument</c> is thrown if the argument <paramref name="pContext"/> has the value <c>NULL</c>.</para>
        ///     <para><c>invalid_operation</c> is thrown if the argument <paramref name="pContext"/> does not represent the execution context that
        ///     was most recently dispatched by this virtual processor root.</para>
        ///     <para>The act of activating a virtual processor root increases the subscription level of the underlying hardware thread by one. For more
        ///     information on subscription levels, see <see cref="IExecutionResource::CurrentSubscriptionLevel Method">
        ///     IExecutionResource::CurrentSubscriptionLevel</see>.</para>
        /// </remarks>
        /// <seealso cref="IVirtualProcessorRoot::Deactivate Method"/>
        /// <seealso cref="IExecutionResource::CurrentSubscriptionLevel Method"/>
        /**/
        virtual void Activate(_Inout_ IExecutionContext * pContext) =0;

        /// <summary>
        ///     Causes the thread proxy currently executing on this virtual processor root to stop dispatching the execution context. The thread proxy
        ///     will resume executing on a call to the <c>Activate</c> method.
        /// </summary>
        /// <param name="pContext">
        ///     The context which is currently being dispatched by this root.
        /// </param>
        /// <returns>
        ///     A boolean value. A value of <c>true</c> indicates that the thread proxy returned from the <c>Deactivate</c> method in response to
        ///     a call to the <c>Activate</c> method. A value of <c>false</c> indicates that the thread proxy returned from the method in response
        ///     to a notification event in the Resource Manager.
        /// </returns>
        /// <remarks>
        ///     Use this method to temporarily stop executing a virtual processor root when you cannot find any work in your scheduler.
        ///     A call to the <c>Deactivate</c> method must originate from within the <c>Dispatch</c> method of the execution context that
        ///     the virtual processor root was last activated with. In other words, the thread proxy invoking the <c>Deactivate</c> method
        ///     must be the one that is currently executing on the virtual processor root. Calling the method on a virtual processor
        ///     root you are not executing on could result in undefined behavior.
        ///     <para>A deactivated virtual processor root may be woken up with a call to the <c>Activate</c> method, with the same
        ///     argument that was passed in to the <c>Deactivate</c> method. The scheduler is responsible for ensuring that calls to the <c>Activate</c>
        ///     and <c>Deactivate</c> methods are paired, but they are not required to be received in a specific order. The Resource
        ///     Manager can handle receiving a call to the <c>Activate</c> method before it receives a call to the <c>Deactivate</c> method it was
        ///     meant for.</para>
        ///     <para><c>invalid_argument</c> is thrown if the argument <paramref name="pContext"/> has the value <c>NULL</c>.</para>
        ///     <para><c>invalid_operation</c> is thrown if the virtual processor root has never been activated, or the argument <paramref name="pContext"/>
        ///     does not represent the execution context that was most recently dispatched by this virtual processor root.</para>
        ///     <para>The act of deactivating a virtual processor root decreases the subscription level of the underlying hardware thread by one.  For
        ///     more information on subscription levels, see <see cref="IExecutionResource::CurrentSubscriptionLevel Method">
        ///     IExecutionResource::CurrentSubscriptionLevel</see>.</para>
        /// </remarks>
        /// <seealso cref="IVirtualProcessorRoot::Activate Method"/>
        /// <seealso cref="IExecutionResource::CurrentSubscriptionLevel Method"/>
        /**/
        virtual bool Deactivate(_Inout_ IExecutionContext * pContext) =0;

        /// <summary>
        ///     Causes data stored in the memory hierarchy of individual processors to become visible to all processors on the system.
        ///     It ensures that a full memory fence has been executed on all processors before the method returns.
        /// </summary>
        /// <param name="pContext">
        ///     The context which is currently being dispatched by this virtual processor root.
        /// </param>
        /// <remarks>
        ///     You may find this method useful when you want to synchronize deactivation of a virtual processor root with the addition of new work into
        ///     the scheduler. For performance reasons, you may decide to add work items to your scheduler without executing a memory barrier, which
        ///     means work items added by a thread executing on one processor are not immediately visible to all other processors. By using this method
        ///     in conjunction with the <c>Deactivate</c> method you can ensure that your scheduler does not deactivate all its virtual processor
        ///     roots while work items exist in your scheduler's collections.
        ///     <para> A call to the <c>EnsureAllTasksVisibleThe</c> method must originate from within the <c>Dispatch</c> method of the execution
        ///     context that the virtual processor root was last activated with. In other words, the thread proxy invoking the <c>EnsureAllTasksVisible</c>
        ///     method must be the one that is currently executing on the virtual processor root. Calling the method on a virtual processor
        ///     root you are not executing on could result in undefined behavior.</para>
        ///     <para><c>invalid_argument</c> is thrown if the argument <paramref name="pContext"/> has the value <c>NULL</c>.</para>
        ///     <para><c>invalid_operation</c> is thrown if the virtual processor root has never been activated, or the argument <paramref name="pContext"/>
        ///     does not represent the execution context that was most recently dispatched by this virtual processor root.</para>
        /// </remarks>
        /// <seealso cref="IVirtualProcessorRoot::Deactivate Method"/>
        /**/
        virtual void EnsureAllTasksVisible(_Inout_ IExecutionContext *pContext) =0;
    };

    /// <summary>
    ///     An interface to an abstraction of a work scheduler. The Concurrency Runtime's Resource Manager uses this interface to communicate with work
    ///     schedulers.
    /// </summary>
    /// <remarks>
    ///     If you are implementing a custom scheduler that communicates with the Resource Manager, you should provide an implementation of the
    ///     <c>IScheduler</c> interface. This interface is one end of a two-way channel of communication between a scheduler and the
    ///     Resource Manager.  The other end is represented by the <c>IResourceManager</c> and <c>ISchedulerProxy</c> interfaces which are
    ///     implemented by the Resource Manager.
    /// </remarks>
    /// <seealso cref="PolicyElementKey Enumeration"/>
    /// <seealso cref="SchedulerPolicy Class"/>
    /// <seealso cref="IExecutionContext Structure"/>
    /// <seealso cref="IThreadProxy Structure"/>
    /// <seealso cref="IVirtualProcessorRoot Structure"/>
    /// <seealso cref="IResourceManager Structure"/>
    /**/
    struct IScheduler
    {
        /// <summary>
        ///     Returns a unique identifier for the scheduler.
        /// </summary>
        /// <returns>
        ///     A unique integer identifier.
        /// </returns>
        /// <remarks>
        ///     You should use the <see cref="GetSchedulerId Function">GetSchedulerId</see> function to obtain a unique identifier for the object
        ///     that implements the <c>IScheduler</c> interface, before you use the interface as a parameter to methods supplied by the Resource Manager.
        ///     You are expected to return the same identifier when the <c>GetId</c> function is invoked. <para> An identifier obtained from a different
        ///     source could result in undefined behavior.</para>
        /// </remarks>
        /**/
        virtual unsigned int GetId() const =0;

        /// <summary>
        ///     Provides information related to task arrival and completion rates, and change in queue length for a scheduler.
        /// </summary>
        /// <param name="pTaskCompletionRate">
        ///     The number of tasks that have been completed by the scheduler since the last call to this method.
        /// </param>
        /// <param name="pTaskArrivalRate">
        ///     The number of tasks that have arrived in the scheduler since the last call to this method.
        /// </param>
        /// <param name="pNumberOfTasksEnqueued">
        ///     The total number of tasks in all scheduler queues.
        /// </param>
        /// <remarks>
        ///     This method is invoked by the Resource Manager in order to gather statistics for a scheduler.  The statistics gathered here
        ///     will be used to drive dynamic feedback algorithms to determine when it is appropriate to assign more resources to
        ///     the scheduler and when to take resources away.  The values provided by the scheduler can be optimistic and do not necessarily
        ///     have to reflect the current count accurately.
        ///     <para> You should implement this method if you want the Resource Manager to use feedback about such things as task arrival to determine
        ///     how to balance resource between your scheduler and other schedulers registered with the Resource Manager. If you choose not to
        ///     gather statistics, you can set the policy key <c>DynamicProgressFeedback</c> to the value <c>DynamicProgressFeedbackDisabled</c>
        ///     in your scheduler's policy, and the Resource Manager will not invoke this method on your scheduler.</para>
        ///     <para>In the absence of statistical information, the Resource Manager will use hardware thread subscription levels to make
        ///     resource allocation and migration decisions. For more information on subscription levels, see
        ///     <see cref="IExecutionResource::CurrentSubscriptionLevel Method"> IExecutionResource::CurrentSubscriptionLevel</see>.</para>
        /// </remarks>
        /// <seealso cref="PolicyElementKey Enumeration"/>
        /// <seealso cref="IExecutionResource::CurrentSubscriptionLevel Method"/>
        /**/
        virtual void Statistics(_Out_ unsigned int * pTaskCompletionRate, _Out_ unsigned int * pTaskArrivalRate, _Out_ unsigned int * pNumberOfTasksEnqueued) =0;

        /// <summary>
        ///     Returns a copy of the scheduler's policy. For more information on scheduler policies, see <see cref="SchedulerPolicy Class">
        ///     SchedulerPolicy</see>.
        /// </summary>
        /// <returns>
        ///     A copy of the scheduler's policy.
        /// </returns>
        /// <seealso cref="SchedulerPolicy Class"/>
        /**/
        virtual SchedulerPolicy GetPolicy() const =0;

        /// <summary>
        ///     Provides a scheduler with a set of virtual processor roots for its use. Each <c>IVirtualProcessorRoot</c> interface represents
        ///     the right to execute a single thread that can perform work on behalf of the scheduler.
        /// </summary>
        /// <param name="ppVirtualProcessorRoots">
        ///     An array of <c>IVirtualProcessorRoot</c> interfaces representing the virtual processor roots being added to the scheduler.
        /// </param>
        /// <param name="count">
        ///     The number of <c>IVirtualProcessorRoot</c> interfaces in the array.
        /// </param>
        /// <remarks>
        ///     The Resource Manager invokes the <c>AddVirtualProcessor</c> method to grant an initial set of virtual processor roots to
        ///     a scheduler. It could also invoke the method to add virtual processor roots to the scheduler when it rebalances resources
        ///     among schedulers.
        /// </remarks>
        /// <seealso cref="IVirtualProcessorRoot Structure"/>
        /// <seealso cref="IScheduler::RemoveVirtualProcessors Method"/>
        /**/
        virtual void AddVirtualProcessors(_In_reads_(count) IVirtualProcessorRoot ** ppVirtualProcessorRoots, unsigned int count) =0;

        /// <summary>
        ///     Initiates the removal of virtual processor roots that were previously allocated to this scheduler.
        /// </summary>
        /// <param name="ppVirtualProcessorRoots">
        ///     An array of <c>IVirtualProcessorRoot</c> interfaces representing the virtual processor roots to be removed.
        /// </param>
        /// <param name="count">
        ///     The number of <c>IVirtualProcessorRoot</c> interfaces in the array.
        /// </param>
        /// <remarks>
        ///     The Resource Manager invokes the <c>RemoveVirtualProcessors</c> method to take back a set of virtual processor roots from
        ///     a scheduler. The scheduler is expected to invoke the <see cref="IExecutionResource::Remove Method">Remove</see> method on each
        ///     interface when it is done with the virtual processor roots. Do not use an <c>IVirtualProcessorRoot</c> interface once you have
        ///     invoked the <c>Remove</c> method on it.
        ///     <para>The parameter <paramref name="ppVirtualProcessorRoots"/> points to an array of interfaces. Among the set of virtual processor
        ///     roots to be removed, the roots have never been activated can be returned immediately using the <c>Remove</c> method.
        ///     The roots that have been activated and are either executing work, or have been deactivated and are waiting for work to arrive, should be
        ///     returned asynchronously. The scheduler must make every attempt to remove the virtual processor root as quickly as possible.
        ///     Delaying removal of the virtual processor roots may result in unintentional oversubscription within the scheduler.</para>
        /// </remarks>
        /// <seealso cref="IVirtualProcessorRoot Structure"/>
        /// <seealso cref="IScheduler::RemoveVirtualProcessors Method"/>
        /**/
        virtual void RemoveVirtualProcessors(_In_reads_(count) IVirtualProcessorRoot ** ppVirtualProcessorRoots, unsigned int count) =0;

        /// <summary>
        ///     Notifies this scheduler that the hardware threads represented by the set of virtual processor roots in the array
        ///     <paramref name="ppVirtualProcessorRoots"/> are not being used by other schedulers.
        /// </summary>
        /// <param name="ppVirtualProcessorRoots">
        ///     An array of <c>IVirtualProcessorRoot</c> interfaces associated with hardware threads on which other schedulers have become idle.
        /// </param>
        /// <param name="count">
        ///     The number of <c>IVirtualProcessorRoot</c> interfaces in the array.
        /// </param>
        /// <remarks>
        ///     It is possible for a particular hardware thread to be assigned to multiple schedulers at the same time. One reason for this could be
        ///     that there are not enough hardware threads on the system to satisfy the minimum concurrency for all schedulers, without sharing resources.
        ///     Another possibility is that resources are temporarily assigned to other schedulers when the owning scheduler is not using them, by way of
        ///     all its virtual processor roots on that hardware thread being deactivated.
        ///     <para>The subscription level of a hardware thread is denoted by the number of subscribed threads and activated virtual processor roots associated
        ///     with that hardware thread. From a particular scheduler's point of view, the external subscription level of a hardware thread is the portion
        ///     of the subscription other schedulers contribute to. Notifications that resources are externally busy are sent to a scheduler when the external
        ///     subscription level for a hardware thread falls to zero from a previous positive value.</para>
        ///     <para>Notifications via this method are only sent to schedulers that have a policy where the value for the <c>MinConcurrency</c>
        ///     policy key is equal to the value for the <c>MaxConcurrency</c> policy key. For more information on scheduler policies,
        ///     see <see cref="SchedulerPolicy Class">SchedulerPolicy</see>.</para>
        ///     <para>A scheduler that qualifies for notifications gets a set of initial notifications when it is created, informing it whether the
        ///     resources it was just assigned are externally busy or idle.</para>
        /// </remarks>
        /// <seealso cref="IExecutionResource::CurrentSubscriptionLevel Method"/>
        /// <seealso cref="IScheduler::NotifyResourcesExternallyBusy Method"/>
        /**/
        virtual void NotifyResourcesExternallyIdle(_In_reads_(count) IVirtualProcessorRoot ** ppVirtualProcessorRoots, unsigned int count) =0;

        /// <summary>
        ///     Notifies this scheduler that the hardware threads represented by the set of virtual processor roots in the array
        ///     <paramref name="ppVirtualProcessorRoots"/> are now being used by other schedulers.
        /// </summary>
        /// <param name="ppVirtualProcessorRoots">
        ///     An array of <c>IVirtualProcessorRoot</c> interfaces associated with the hardware threads on which other schedulers have become busy.
        /// </param>
        /// <param name="count">
        ///     The number of <c>IVirtualProcessorRoot</c> interfaces in the array.
        /// </param>
        /// <remarks>
        ///     It is possible for a particular hardware thread to be assigned to multiple schedulers at the same time. One reason for this could be
        ///     that there are not enough hardware threads on the system to satisfy the minimum concurrency for all schedulers, without sharing resources.
        ///     Another possibility is that resources are temporarily assigned to other schedulers when the owning scheduler is not using them, by way of
        ///     all its virtual processor roots on that hardware thread being deactivated.
        ///     <para>The subscription level of a hardware thread is denoted by the number of subscribed threads and activated virtual processor roots associated
        ///     with that hardware thread. From a particular scheduler's point of view, the external subscription level of a hardware thread is the portion
        ///     of the subscription other schedulers contribute to. Notifications that resources are externally busy are sent to a scheduler when the external
        ///     subscription level for a hardware thread moves from zero into positive territory.</para>
        ///     <para>Notifications via this method are only sent to schedulers that have a policy where the value for the <c>MinConcurrency</c>
        ///     policy key is equal to the value for the <c>MaxConcurrency</c> policy key. For more information on scheduler policies,
        ///     see <see cref="SchedulerPolicy Class">SchedulerPolicy</see>.</para>
        ///     <para>A scheduler that qualifies for notifications gets a set of initial notifications when it is created, informing it whether the
        ///     resources it was just assigned are externally busy or idle.</para>
        /// </remarks>
        /// <seealso cref="IExecutionResource::CurrentSubscriptionLevel Method"/>
        /// <seealso cref="IScheduler::NotifyResourcesExternallyIdle Method"/>
        /**/
        virtual void NotifyResourcesExternallyBusy(_In_reads_(count) IVirtualProcessorRoot ** ppVirtualProcessorRoots, unsigned int count) =0;
    };

    /// <summary>
    ///     The interface by which schedulers communicate with the Concurrency Runtime's Resource Manager to negotiate resource allocation.
    /// </summary>
    /// <remarks>
    ///     The Resource Manager hands an <c>ISchedulerProxy</c> interface to every scheduler that registers with it using the
    ///     <see cref="IResourceManager::RegisterScheduler Method">IResourceManager::RegisterScheduler</see> method.
    /// </remarks>
    /// <seealso cref="IScheduler Structure"/>
    /// <seealso cref="IThreadProxy Structure"/>
    /// <seealso cref="IVirtualProcessorRoot Structure"/>
    /// <seealso cref="IResourceManager Structure"/>
    /**/
    struct ISchedulerProxy
    {
        /// <summary>
        ///     Requests an initial allocation of virtual processor roots. Every virtual processor root represents the ability to execute one thread
        ///     that can perform work for the scheduler.
        /// </summary>
        /// <param name="doSubscribeCurrentThread">
        ///     Whether to subscribe the current thread and account for it during resource allocation.
        /// </param>
        /// <returns>
        ///     The <c>IExecutionResource</c> interface for the current thread, if the parameter <paramref name="doSubscribeCurrentThread"/> has
        ///     the value <c>true</c>. If the value is <c>false</c>, the method returns <c>NULL</c>.
        /// </returns>
        /// <remarks>
        ///     Before a scheduler executes any work, it should use this method to request virtual processor roots from the Resource Manager. The Resource
        ///     Manager will access the scheduler's policy using <see cref="IScheduler::GetPolicy Method">IScheduler::GetPolicy</see> and use the
        ///     values for the policy keys <c>MinConcurrency</c>, <c>MaxConcurrency</c> and <c>TargetOversubscriptionFactor</c> to determine how many
        ///     hardware threads to assign to the scheduler initially and how many virtual processor roots to create for every hardware thread.
        ///     For more information on how scheduler policies are used to determine a scheduler's initial allocation, see <see cref="PolicyElementKey Enumeration">
        ///     PolicyElementKey</see>.
        ///     <para>The Resource Manager grants resources to a scheduler by calling the method <see cref="IScheduler::AddVirtualProcessors Method">
        ///     IScheduler::AddVirtualProcessors</see> with a list of virtual processor roots. The method is invoked as a callback into the scheduler
        ///     before this method returns.</para>
        ///     <para> If the scheduler requested subscription for the current thread by setting the parameter <paramref name="doSubscribeCurrentThread"/>
        ///     to <c>true</c>, the method returns an <c>IExecutionResource</c> interface. The subscription must be terminated at a later point by using
        ///     the <see cref="IExecutionResource::Remove Method">IExecutionResource::Remove</see> method.</para>
        ///     <para>When determining which hardware threads are selected, the Resource Manager will attempt to optimize for processor node affinity.
        ///     If subscription is requested for the current thread, it is an indication that the current thread intends to participate in the work assigned
        ///     to this scheduler. In such a case, the allocated virtual processors roots are located on the processor node the current thread is executing on,
        ///     if possible.</para>
        ///     <para>The act of subscribing a thread increases the subscription level of the underlying hardware thread by one. The subscription level is
        ///     reduced by one when the subscription is terminated. For more information on subscription levels, see
        ///     <see cref="IExecutionResource::CurrentSubscriptionLevel Method">IExecutionResource::CurrentSubscriptionLevel</see>.</para>
        /// </remarks>
        /**/
        virtual IExecutionResource * RequestInitialVirtualProcessors(bool doSubscribeCurrentThread) =0;

        /// <summary>
        ///     Notifies the Resource Manager that the scheduler is shutting down.  This will cause the Resource Manager to immediately reclaim
        ///     all resources granted to the scheduler.
        /// </summary>
        /// <remarks>
        ///     All <c>IExecutionContext</c> interfaces the scheduler received as a result of subscribing an external thread using the methods
        ///     <c>ISchedulerProxy::RequestInitialVirtualProcessors</c> or <c>ISchedulerProxy::SubscribeCurrentThread</c> must be returned to the Resource
        ///     Manager using <c>IExecutionResource::Remove</c> before a scheduler shuts itself down.
        ///     <para>If your scheduler had any deactivated virtual processor roots, you must activate them using <see cref="IVirtualProcessorRoot::Activate Method">
        ///     IVirtualProcessorRoot::Activate</see>, and have the thread proxies executing on them leave the <c>Dispatch</c> method of the execution contexts
        ///     they are dispatching before you invoke <c>Shutdown</c> on a scheduler proxy.</para>
        ///     <para>It is not necessary for the scheduler to individually return all of the virtual processor roots the Resource Manager granted to it via
        ///     calls to the <c>Remove</c> method because all virtual processors roots will be returned to the Resource Manager at shutdown.</para>
        /// </remarks>
        /// <seealso cref="ISchedulerProxy::RequestInitialVirtualProcessors Method"/>
        /// <seealso cref="ISchedulerProxy::SubscribeCurrentThread Method"/>
        /// <seealso cref="IExecutionResource::Remove Method"/>
        /**/
        virtual void Shutdown() =0;

        /// <summary>
        ///     Associates an execution context with a thread proxy, if it is not already associated with one.
        /// </summary>
        /// <param name="pContext">
        ///     An interface to the execution context to associate with a thread proxy.
        /// </param>
        /// <remarks>
        ///     Normally, the <see cref="IThreadProxy::SwitchTo Method">IThreadProxy::SwitchTo</see> method will bind a thread proxy to an
        ///     execution context on demand.  There are, however, circumstances where it is necessary to bind a context in advance
        ///     to ensure that the <c>SwitchTo</c> method switches to an already bound context.
        ///     <para><c>invalid_argument</c> is thrown if the parameter <paramref name="pContext"/> has the value <c>NULL</c>.</para>
        /// </remarks>
        /// <seealso cref="ISchedulerProxy::UnbindContext Method"/>
        /**/
        virtual void BindContext(_Inout_ IExecutionContext * pContext) =0;

        /// <summary>
        ///     Disassociates a thread proxy from the execution context specified by the <paramref name="pContext"/> parameter and returns it
        ///     to the thread proxy factory's free pool.  This method may only be called on an execution context which was bound via the
        ///     <see cref="ISchedulerProxy::BindContext Method">ISchedulerProxy::BindContext</see> method and has not yet been started via being
        ///     the <c>pContext</c> parameter of an <see cref="IThreadProxy::SwitchTo Method">IThreadProxy::SwitchTo</see> method call.
        /// </summary>
        /// <param name="pContext">
        ///     The execution context to disassociate from its thread proxy.
        /// </param>
        /**/
        virtual void UnbindContext(_Inout_ IExecutionContext * pContext) =0;

        /// <summary>
        ///     Registers the current thread with the Resource Manager, associating it with this scheduler.
        /// </summary>
        /// <returns>
        ///     The <c>IExecutionResource</c> interfacing representing the current thread in the runtime.
        /// </returns>
        /// <remarks>
        ///     Use this method if you want the Resource Manager to account for the current thread while allocating resources to your scheduler and other
        ///     schedulers. It is especially useful when the thread plans to participate in the work queued to your scheduler, along with the virtual
        ///     processor roots the scheduler receives from the Resource Manager. The Resource Manager uses information to prevent unnecessary oversubscription
        ///     of hardware threads on the system.
        ///     <para>The execution resource received via this method should be returned to the Resource Manager using the
        ///     <see cref="IExecutionResource::Remove Method">IExecutionResource::Remove</see> method. The thread that calls the <c>Remove</c> method must be
        ///     the same thread that previously called the <c>SubscribeCurrentThread</c> method.</para>
        ///     <para>The act of subscribing a thread increases the subscription level of the underlying hardware thread by one. The subscription level is
        ///     reduced by one when the subscription is terminated. For more information on subscription levels, see
        ///     <see cref="IExecutionResource::CurrentSubscriptionLevel Method">IExecutionResource::CurrentSubscriptionLevel</see>.</para>
        /// </remarks>
        /**/
        virtual IExecutionResource * SubscribeCurrentThread() =0;

        /// <summary>
        ///     Creates a new virtual processor root on the hardware thread associated with an existing execution resource.
        /// </summary>
        /// <param name="pExecutionResource">
        ///     An <c>IExecutionResource</c> interface that represents the hardware thread you want to oversubscribe.
        /// </param>
        /// <returns>
        ///     An <c>IVirtualProcessorRoot</c> interface.
        /// </returns>
        /// <remarks>
        ///     Use this method when your scheduler wants to oversubscribe a particular hardware thread for a limited amount of time. Once you are
        ///     done with the virtual processor root, you should return it to the resource manager by calling the
        ///     <see cref="IExecutionResource::Remove Method">Remove</see> method on the <c>IVirtualProcessorRoot</c> interface.
        ///     <para>You can even oversubscribe an existing virtual processor root, because the <c>IVirtualProcessorRoot</c> interface inherits from the
        ///     <c>IExecutionResource</c> interface.</para>
        /// </remarks>
        /**/
        virtual IVirtualProcessorRoot * CreateOversubscriber(_Inout_ IExecutionResource * pExecutionResource) =0;
    };

    /// <summary>
    ///     An interface to an execution resource as defined by the Resource Manager.
    /// </summary>
    /// <remarks>
    ///     This interface is typically utilized to walk the topology of the system as observed by the Resource Manager.
    /// </remarks>
    /**/
    struct ITopologyExecutionResource
    {
        /// <summary>
        ///     Returns an interface to the next execution resource in enumeration order.
        /// </summary>
        /// <returns>
        ///     An interface to the next execution resource in enumeration order.  If there are no more nodes in enumeration order of the node to which
        ///     this execution resource belongs, this method will return the value <c>NULL</c>.
        /// </returns>
        /// <seealso cref="ITopologyNode::GetFirstExecutionResource Method"/>
        /// <seealso cref="ITopologyNode Structure"/>
        /**/
        virtual ITopologyExecutionResource *GetNext() const =0;

        /// <summary>
        ///     Returns the Resource Manager's unique identifier for this execution resource.
        /// </summary>
        /// <returns>
        ///     The Resource Manager's unique identifier for this execution resource.
        /// </returns>
        /**/
        virtual unsigned int GetId() const =0;
    };

    /// <summary>
    ///     An interface to a topology node as defined by the Resource Manager. A node contains one or more execution resources.
    /// </summary>
    /// <remarks>
    ///     This interface is typically utilized to walk the topology of the system as observed by the Resource Manager.
    /// </remarks>
    /**/
    struct ITopologyNode
    {
        /// <summary>
        ///     Returns an interface to the next topology node in enumeration order.
        /// </summary>
        /// <returns>
        ///     An interface to the next node in enumeration order.  If there are no more nodes in enumeration order of the system topology, this method
        ///     will return the value <c>NULL</c>.
        /// </returns>
        /// <seealso cref="IResourceManager::GetFirstNode Method"/>
        /// <seealso cref="ITopologyExecutionResource Structure"/>
        /**/
        virtual ITopologyNode *GetNext() const =0;

        /// <summary>
        ///     Returns the Resource Manager's unique identifier for this node.
        /// </summary>
        /// <returns>
        ///     The Resource Manager's unique identifier for this node.
        /// </returns>
        /// <remarks>
        ///     The Concurrency Runtime represents hardware threads on the system in groups of processor nodes. Nodes are usually derived from
        ///     the hardware topology of the system. For example, all processors on a specific socket or a specific NUMA node may belong to the
        ///     same processor node. The Resource Manager assigns unique identifiers to these nodes starting with <c>0</c> up to and including
        ///     <c>nodeCount - 1</c>, where <c>nodeCount</c> represents the total number of processor nodes on the system.
        ///     <para>The count of nodes can be obtained from the function <see cref="GetProcessorNodeCount Function">GetProcessorNodeCount</see>.</para>
        /// </remarks>
        /**/
        virtual unsigned int GetId() const =0;

        /// <summary>
        ///     Returns the Windows assigned NUMA node number to which this Resource Maanger node belongs.
        /// </summary>
        /// <returns>
        ///     The Windows assigned NUMA node number to which this Resource Manager node belongs.
        /// </returns>
        /// <remarks>
        ///     A thread proxy running on a virtual processor root belonging to this node will have affinity to at least the NUMA node level for the NUMA
        ///     node returned by this method.
        /// </remarks>
        /**/
        virtual unsigned long GetNumaNode() const =0;

        /// <summary>
        ///     Returns the number of execution resources grouped together under this node.
        /// </summary>
        /// <returns>
        ///     The number of execution resources grouped together under this node.
        /// </returns>
        /// <seealso cref="ITopologyNode::GetFirstExecutionResource Method"/>
        /**/
        virtual unsigned int GetExecutionResourceCount() const =0;

        /// <summary>
        ///     Returns the first execution resource grouped under this node in enumeration order.
        /// </summary>
        /// <returns>
        ///     The first execution resource grouped under this node in enumeration order.
        /// </returns>
        /// <seealso cref="ITopologyNode::GetExecutionResourceCount Method"/>
        /**/
        virtual ITopologyExecutionResource *GetFirstExecutionResource() const =0;

    };

    /// <summary>
    ///     Indicates support of the Resource Manager interface defined in Visual Studio 2010.
    /// </summary>
    /**/
    const unsigned int CONCRT_RM_VERSION_1 = 0x00010000;

    /// <summary>
    ///     An interface to the Concurrency Runtime's Resource Manager. This is the interface by which schedulers communicate with the
    ///     Resource Manager.
    /// </summary>
    /// <remarks>
    ///     Use the <see cref="CreateResourceManager Function">CreateResourceManager</see> function to obtain an interface to the singleton Resource Manager
    ///     instance.  The method increments a reference count on the Resource Manager, and you should invoke the <see cref="IResourceManager::Release Method">
    ///     IResourceManager::Release</see> method to release the reference when you are done with Resource Manager. Typically, each scheduler
    ///     you create will invoke this method during creation, and release the reference to the Resource Manager after it shuts down.
    /// </remarks>
    /// <seealso cref="ISchedulerProxy Structure"/>
    /// <seealso cref="IScheduler Structure"/>
    /**/
    struct IResourceManager
    {
        /// <summary>
        ///     Increments the reference count on the Resource Manager instance.
        /// </summary>
        /// <returns>
        ///     The resulting reference count.
        /// </returns>
        /// <seealso cref="IResourceManager::Release Method"/>
        /**/
        virtual unsigned int Reference() =0;

        /// <summary>
        ///     Decrements the reference count on the Resource Manager instance. The Resource Manager is destroyed when its reference count goes to <c>0</c>.
        /// </summary>
        /// <returns>
        ///     The resulting reference count.
        /// </returns>
        /// <seealso cref="CreateResourceManager Function"/>
        /// <seealso cref="IResourceManager::Reference Method"/>
        /**/
        virtual unsigned int Release() =0;

        /// <summary>
        ///     Registers a scheduler with the Resource Manager. Once the scheduler is registered, it should communicate with the Resource Manager using the
        ///     <c>ISchedulerProxy</c> interface that is returned.
        /// </summary>
        /// <param name="pScheduler">
        ///     An <c>IScheduler</c> interface to the scheduler to be registered.
        /// </param>
        /// <param name="version">
        ///     The version of communication interface the scheduler is using to communicate with the Resource Manager. Using a version allows the Resource
        ///     Manager to evolve the communication interface while allowing schedulers to obtain access to older features. Schedulers that wish to use Resource
        ///     Manager features present in Visual Studio 2010 should use the version <c>CONCRT_RM_VERSION_1</c>.
        /// </param>
        /// <returns>
        ///     The <c>ISchedulerProxy</c> interface the Resource Manager has associated with your scheduler. Your scheduler should use this interface to
        ///     communicate with Resource Manager from this point on.
        /// </returns>
        /// <remarks>
        ///     Use this method to initiate communication with the Resource Manager.  The method associates the <c>IScheduler</c> interface for your scheduler
        ///     with an <c>ISchedulerProxy</c> interface and hands it back to you. You can use the returned interface to request execution resources for use
        ///     by your scheduler, or to subscribe threads with the Resource Manager. The Resource Manager will use policy elements from the scheduler policy
        ///     returned by the <see cref="IScheduler::GetPolicy Method">IScheduler::GetPolicy</see> method to determine what type of threads the scheduler will
        ///     need to execute work.
        ///     <para>The method throws an <c>invalid_argument</c> exception if the parameter <paramref name="pScheduler"/> has the value <c>NULL</c> or if the
        ///     parameter <paramref name="version"/> is not a valid version for the communication interface.</para>
        /// </remarks>
        /// <seealso cref="IScheduler Structure"/>
        /// <seealso cref="ISchedulerProxy Structure"/>
        /// <seealso cref="SchedulerPolicy Class"/>
        /// <seealso cref="PolicyElementKey Enumeration"/>
        /**/
        virtual ISchedulerProxy *RegisterScheduler(_Inout_ IScheduler * pScheduler, unsigned int version) =0;

        /// <summary>
        ///     Returns the number of nodes available to the Resource Manager.
        /// </summary>
        /// <returns>
        ///     The number of nodes available to the Resource Manager.
        /// </returns>
        /**/
        virtual unsigned int GetAvailableNodeCount() const =0;

        /// <summary>
        ///     Returns the first node in enumeration order as defined by the Resource Manager.
        /// </summary>
        /// <returns>
        ///     The first node in enumeration order as defined by the Resource Manager.
        /// </returns>
        /// <seealso cref="ITopologyNode::GetExecutionResourceCount Method"/>
        /**/
        virtual ITopologyNode* GetFirstNode() const =0;

        /// <summary>
        ///     Present only in debug builds of the runtime, this method is a test hook designed to facilitate testing of the Resource Manager on varying hardware
        ///     topologies, without requiring actual hardware matching the configuration. With retail builds of the runtime, this method will return without performing
        ///     any action.
        /// </summary>
        /// <param name="nodeCount">
        ///     The number of processor nodes being simulated.
        /// </param>
        /// <param name="pCoreCount">
        ///     An array that specifies the number of cores on each node.
        /// </param>
        /// <param name="pNodeDistance">
        ///     A matrix specifying the node distance between any two nodes. This parameter can have the value <c>NULL</c>.
        /// </param>
        /// <param name="pProcessorGroups">
        ///     An array that specifies the processor group each node belongs to.
        /// </param>
        /// <remarks>
        ///     <see cref="invalid_argument Class">invalid_argument</see> is thrown if the parameter <paramref name="nodeCount"/> has the value <c>0</c> was passed
        ///     in, or if the parameter <paramref name="pCoreCount"/> has the value <c>NULL</c>.
        ///     <para><see cref="invalid_operation Class">invalid_operation</see> is thrown if this method is called while other schedulers exist in the process.</para>
        /// </remarks>
        /**/
        virtual void CreateNodeTopology(unsigned int nodeCount, _In_reads_(nodeCount) unsigned int * pCoreCount, _In_reads_opt_(nodeCount) unsigned int ** pNodeDistance, _In_reads_(nodeCount) unsigned int * pProcessorGroups) =0;

        /// <summary>
        ///     An enumerated type that represents the operating system version.
        /// </summary>
        /**/
        enum OSVersion
        {
            /// <summary>
            /// An operating system prior to Windows XP.
            /// </summary>
            /**/
            UnsupportedOS,

            /// <summary>
            ///     The Windows XP operating system.
            /// </summary>
            /**/
            XP,

            /// <summary>
            ///     The Windows Server 2003 operating system.
            /// </summary>
            /**/
            Win2k3,

            /// <summary>
            ///     The Windows Vista and Windows Server 2008 operating systems.
            /// </summary>
            /**/
            Vista,

            /// <summary>
            ///     The Windows 7 and Windows Server 2008 R2 operating systems.
            /// </summary>
            /**/
            Win7OrLater,

            /// <summary>
            ///     This value is preserved for legacy reasons.  The Concurrency Runtime in Visual Studio 2012 does not support Windows user-mode schedulable
            ///     threads.
            /// </summary>
            /**/
            UmsThreadAwareOS,

            /// <summary>
            ///     Any operating system with version Windows 8 or higher.
            /// </summary>
            /**/
            Win8OrLater
        };
    };

    /// <summary>
    ///     Returns an interface that represents the singleton instance of the Concurrency Runtime's Resource Manager. The Resource Manager is responsible
    ///     for assigning resources to schedulers that want to cooperate with each other.
    /// </summary>
    /// <returns>
    ///     An <c>IResourceManager</c> interface.
    /// </returns>
    /// <remarks>
    ///     Multiple subsequent calls to this method will return the same instance of the Resource Manager. Each call to the method increments a reference
    ///     count on the Resource Manager, and must be matched with a call to the <see cref="IResourceManager::Release"> IResourceManager::Release</see>
    ///     method when your scheduler is done communicating with the Resource Manager.
    ///     <para><see cref="unsupported_os Class">unsupported_os</see> is thrown if the operating system is not supported by the Concurrency Runtime.</para>
    /// </remarks>
    /// <seealso cref="IResourceManager::OSVersion Enumeration"/>
    /**/
    _CONCRTIMP IResourceManager* __cdecl CreateResourceManager();

    /// <summary>
    ///     Returns the operating system version.
    /// </summary>
    /// <returns>
    ///     An enumerated value representing the operating system.
    /// </returns>
    /// <remarks>
    ///     <para><see cref="unsupported_os Class">unsupported_os</see> is thrown if the operating system is not supported by the Concurrency Runtime.</para>
    /// </remarks>
    /// <seealso cref="IResourceManager::OSVersion Enumeration"/>
    /**/
    _CONCRTIMP IResourceManager::OSVersion __cdecl GetOSVersion();

    /// <summary>
    ///     Returns a unique identifier that can be assigned to a scheduler that implements the <c>IScheduler</c> interface.
    /// </summary>
    /// <returns>
    ///     A unique identifier for a scheduler.
    /// </returns>
    /// <remarks>
    ///     Use this method to obtain an identifier for your scheduler before you pass an <c>IScheduler</c> interface as a parameter to any of the methods
    ///     offered by the Resource Manager.
    /// </remarks>
    /**/
    _CONCRTIMP unsigned int __cdecl GetSchedulerId();

    /// <summary>
    ///     Returns a unique identifier that can be assigned to an execution context that implements the <c>IExecutionContext</c> interface.
    /// </summary>
    /// <returns>
    ///     A unique identifier for an execution context.
    /// </returns>
    /// <remarks>
    ///     Use this method to obtain an identifier for your execution context before you pass an <c>IExecutionContext</c> interface as a parameter to any
    ///     of the methods offered by the Resource Manager.
    /// </remarks>
    /**/
    _CONCRTIMP unsigned int __cdecl GetExecutionContextId();

    /// <summary>
    ///     Returns the number of hardware threads on the underlying system.
    /// </summary>
    /// <returns>
    ///     The number of hardware threads.
    /// </returns>
    /// <remarks>
    ///     <para><see cref="unsupported_os Class">unsupported_os</see> is thrown if the operating system is not supported by the Concurrency Runtime.</para>
    /// </remarks>
    /// <seealso cref="IResourceManager::OSVersion Enumeration"/>
    /**/
    _CONCRTIMP unsigned int __cdecl GetProcessorCount();

    /// <summary>
    ///     Returns the number of NUMA nodes or processor packages on the underlying system.
    /// </summary>
    /// <returns>
    ///     The number of NUMA nodes or processor packages.
    /// </returns>
    /// <remarks>
    ///     If the system contains more NUMA nodes than processor packages, the number of NUMA nodes is returned, otherwise, the number of processor packages is returned.
    ///     <para><see cref="unsupported_os Class">unsupported_os</see> is thrown if the operating system is not supported by the Concurrency Runtime.</para>
    /// </remarks>
    /// <seealso cref="IResourceManager::OSVersion Enumeration"/>
    /**/
    _CONCRTIMP unsigned int __cdecl GetProcessorNodeCount();
#endif  /* _CRT_USE_WINAPI_FAMILY_DESKTOP_APP */
}

namespace concurrency = ::Concurrency;

#pragma pack(pop)
