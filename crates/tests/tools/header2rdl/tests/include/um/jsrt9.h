// Copyright (C) Microsoft. All rights reserved.

/// \mainpage Chakra Hosting API Reference
///
/// Chakra is Microsoft's JavaScript engine. It is an integral part of Internet Explorer but can 
/// also be hosted independently by other applications. This reference describes the APIs available
/// to applications to host Chakra.

/// \file
/// \brief The base Chakra hosting API.
///
/// This file contains a flat C API layer. This is the API exported by jscript9.dll.

#ifdef _MSC_VER
#pragma once
#endif  // _MSC_VER

#ifndef _JSRT_
#error "You should include <jsrt.h> instead of <jsrt9.h> or <chakrart.h>."
#endif

#if defined(_CHAKRART_H_) || defined(USE_EDGEMODE_JSRT)
#error "It is invalid to include both jscript9-mode and edge-mode JsRT headers.  To target edge-mode, use #define USE_EDGEMODE_JSRT and then include jsrt.h, ensuring that you link against chakrart.lib.  To use jscript9 mode, simply include jsrt.h and link against jsrt.lib."
#endif

#ifndef _JSRT9_H_
#define _JSRT9_H_

#if NTDDI_VERSION >= NTDDI_WIN7

#include <activdbg.h>
#include <activprof.h>

#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)

    /// <summary>
    ///     An error code returned from a Chakra hosting API.
    /// </summary>
    typedef _Return_type_success_(return == 0) enum JsErrorCode : unsigned int
    {
        /// <summary>
        ///     Success error code.
        /// </summary>
        JsNoError = 0,

        /// <summary>
        ///     Category of errors that relates to incorrect usage of the API itself.
        /// </summary>
        JsErrorCategoryUsage = 0x10000,
        /// <summary>
        ///     An argument to a hosting API was invalid.
        /// </summary>
        JsErrorInvalidArgument,
        /// <summary>
        ///     An argument to a hosting API was null in a context where null is not allowed.
        /// </summary>
        JsErrorNullArgument,
        /// <summary>
        ///     The hosting API requires that a context be current, but there is no current context.
        /// </summary>
        JsErrorNoCurrentContext,
        /// <summary>
        ///     The engine is in an exception state and no APIs can be called until the exception is 
        ///     cleared.
        /// </summary>
        JsErrorInExceptionState,
        /// <summary>
        ///     A hosting API is not yet implemented.
        /// </summary>
        JsErrorNotImplemented,
        /// <summary>
        ///     A hosting API was called on the wrong thread.
        /// </summary>
        JsErrorWrongThread,
        /// <summary>
        ///     A runtime that is still in use cannot be disposed.
        /// </summary>
        JsErrorRuntimeInUse,
        /// <summary>
        ///     A bad serialized script was used, or the serialized script was serialized by a 
        ///     different version of the Chakra engine.
        /// </summary>
        JsErrorBadSerializedScript,
        /// <summary>
        ///     The runtime is in a disabled state.
        /// </summary>
        JsErrorInDisabledState,
        /// <summary>
        ///     Runtime does not support reliable script interruption.
        /// </summary>
        JsErrorCannotDisableExecution,
        /// <summary>
        ///     A heap enumeration is currently underway in the script context.
        /// </summary>
        JsErrorHeapEnumInProgress,
        /// <summary>
        ///     A hosting API that operates on object values was called with a non-object value.
        /// </summary>
        JsErrorArgumentNotObject,
        /// <summary>
        ///     A script context is in the middle of a profile callback.
        /// </summary>
        JsErrorInProfileCallback,
        /// <summary>
        ///     A thread service callback is currently underway.
        /// </summary>
        JsErrorInThreadServiceCallback,
        /// <summary>
        ///     Scripts cannot be serialized in debug contexts.
        /// </summary>
        JsErrorCannotSerializeDebugScript,
        /// <summary>
        ///     The context cannot be put into a debug state because it is already in a debug state.
        /// </summary>
        JsErrorAlreadyDebuggingContext,
        /// <summary>
        ///     The context cannot start profiling because it is already profiling.
        /// </summary>
        JsErrorAlreadyProfilingContext,
        /// <summary>
        ///     Idle notification given when the host did not enable idle processing.
        /// </summary>
        JsErrorIdleNotEnabled,

        /// <summary>
        ///     Category of errors that relates to errors occurring within the engine itself.
        /// </summary>
        JsErrorCategoryEngine = 0x20000,
        /// <summary>
        ///     The Chakra engine has run out of memory.
        /// </summary>
        JsErrorOutOfMemory,

        /// <summary>
        ///     Category of errors that relates to errors in a script.
        /// </summary>
        JsErrorCategoryScript = 0x30000,
        /// <summary>
        ///     A JavaScript exception occurred while running a script.
        /// </summary>
        JsErrorScriptException,
        /// <summary>
        ///     JavaScript failed to compile.
        /// </summary>
        JsErrorScriptCompile,
        /// <summary>
        ///     A script was terminated due to a request to suspend a runtime.
        /// </summary>
        JsErrorScriptTerminated,
        /// <summary>
        ///     A script was terminated because it tried to use <c>eval</c> or <c>function</c> and eval
        ///     was disabled.
        /// </summary>
        JsErrorScriptEvalDisabled,

        /// <summary>
        ///     Category of errors that are fatal and signify failure of the engine.
        /// </summary>
        JsErrorCategoryFatal = 0x40000,
        /// <summary>
        ///     A fatal error in the engine has occurred.
        /// </summary>
        JsErrorFatal,
    }JsErrorCode;

    /// <summary>
    ///     A handle to a Chakra runtime.
    /// </summary>
    /// <remarks>
    ///     <para>
    ///     Each Chakra runtime has its own independent execution engine, JIT compiler, and garbage 
    ///     collected heap. As such, each runtime is completely isolated from other runtimes.
    ///     </para>
    ///     <para>
    ///     Runtimes can be used on any thread, but only one thread can call into a runtime at any 
    ///     time.
    ///     </para>
    ///     <para>
    ///     NOTE: A <c>JsRuntimeHandle</c>, unlike other object references in the Chakra hosting API, 
    ///     is not garbage collected since it contains the garbage collected heap itself. A runtime 
    ///     will continue to exist until <c>JsDisposeRuntime</c> is called.
    ///     </para>
    /// </remarks>
    typedef void *JsRuntimeHandle;

    /// <summary>
    ///     An invalid runtime handle.
    /// </summary>
    const JsRuntimeHandle JS_INVALID_RUNTIME_HANDLE = NULL;

    /// <summary>
    ///     A reference to an object owned by the Chakra garbage collector.
    /// </summary>
    /// <remarks>
    ///     A Chakra runtime will automatically track <c>JsRef</c> references as long as they are
    ///     stored in local variables or in parameters (i.e. on the stack). Storing a <c>JsRef</c>
    ///     somewhere other than on the stack requires calling <c>JsAddRef</c> and <c>JsRelease</c> to
    ///     manage the lifetime of the object, otherwise the garbage collector may free the object 
    ///     while it is still in use.
    /// </remarks>
    typedef void *JsRef;

    /// <summary>
    ///     An invalid reference.
    /// </summary>
    const JsRef JS_INVALID_REFERENCE = NULL;

    /// <summary>
    ///     A reference to a script context.
    /// </summary>
    /// <remarks>
    ///     <para>
    ///     Each script context contains its own global object, distinct from the global object in 
    ///     other script contexts.
    ///     </para>
    ///     <para>
    ///     Many Chakra hosting APIs require an "active" script context, which can be set using 
    ///     <c>JsSetCurrentContext</c>. Chakra hosting APIs that require a current context to be set 
    ///     will note that explicitly in their documentation.
    ///     </para>
    /// </remarks>
    typedef JsRef JsContextRef;

    /// <summary>
    ///     A reference to a JavaScript value.
    /// </summary>
    /// <remarks>
    ///     A JavaScript value is one of the following types of values: undefined, null, Boolean, 
    ///     string, number, or object.
    /// </remarks>
    typedef JsRef JsValueRef;

    /// <summary>
    ///     A cookie that identifies a script for debugging purposes.
    /// </summary>
    typedef DWORD_PTR JsSourceContext;

    /// <summary>
    ///     An empty source context.
    /// </summary>
    const JsSourceContext JS_SOURCE_CONTEXT_NONE = (JsSourceContext)-1;

    /// <summary>
    ///     A property identifier.
    /// </summary>
    /// <remarks>
    ///     Property identifiers are used to refer to properties of JavaScript objects instead of using
    ///     strings.
    /// </remarks>
    typedef JsRef JsPropertyIdRef;

    /// <summary>
    ///     Attributes of a runtime.
    /// </summary>
    typedef enum JsRuntimeAttributes
    {
        /// <summary>
        ///     No special attributes.
        /// </summary>
        JsRuntimeAttributeNone = 0x00000000,
        /// <summary>
        ///     The runtime will not do any work (such as garbage collection) on background threads.
        /// </summary>
        JsRuntimeAttributeDisableBackgroundWork = 0x00000001,
        /// <summary>
        ///     The runtime should support reliable script interruption. This increases the number of
        ///     places where the runtime will check for a script interrupt request at the cost of a
        ///     small amount of runtime performance.
        /// </summary>
        JsRuntimeAttributeAllowScriptInterrupt = 0x00000002,
        /// <summary>
        ///     Host will call <c>JsIdle</c>, so enable idle processing. Otherwise, the runtime will 
        ///     manage memory slightly more aggressively.
        /// </summary>
        JsRuntimeAttributeEnableIdleProcessing = 0x00000004,
        /// <summary>
        ///     Runtime will not generate native code.
        /// </summary>
        JsRuntimeAttributeDisableNativeCodeGeneration = 0x00000008,
        /// <summary>
        ///     Using <c>eval</c> or <c>function</c> constructor will throw an exception.
        /// </summary>
        JsRuntimeAttributeDisableEval = 0x00000010,
    }JsRuntimeAttributes;

    /// <summary>
    ///     Allocation callback event type.
    /// </summary>
    typedef enum JsMemoryEventType
    {
        /// <summary>
        ///     Indicates a request for memory allocation.
        /// </summary>
        JsMemoryAllocate = 0,
        /// <summary>
        ///     Indicates a memory freeing event.
        /// </summary>
        JsMemoryFree = 1,
        /// <summary>
        ///     Indicates a failed allocation event.
        /// </summary>
        JsMemoryFailure = 2
    }JsMemoryEventType;

    /// <summary>
    ///     User implemented callback routine for memory allocation events
    /// </summary>
    /// <remarks>
    ///     Use <c>JsSetRuntimeMemoryAllocationCallback</c> to register this callback.
    /// </remarks>
    /// <param name="callbackState">
    ///     The state passed to <c>JsSetRuntimeMemoryAllocationCallback</c>.
    /// </param>
    /// <param name="allocationEvent">The type of type allocation event.</param>
    /// <param name="allocationSize">The size of the allocation.</param>
    /// <returns>
    ///     For the <c>JsMemoryAllocate</c> event, returning <c>true</c> allows the runtime to continue
    ///     with the allocation. Returning false indicates the allocation request is rejected. The 
    ///     return value is ignored for other allocation events.
    /// </returns>
    typedef bool (CALLBACK * JsMemoryAllocationCallback)(_In_opt_ void *callbackState, _In_ JsMemoryEventType allocationEvent, _In_ size_t allocationSize);

    /// <summary>
    ///     A callback called before collection.
    /// </summary>
    /// <remarks>
    ///     Use <c>JsSetBeforeCollectCallback</c> to register this callback.
    /// </remarks>
    /// <param name="callbackState">The state passed to <c>JsSetBeforeCollectCallback</c>.</param>
    typedef void (CALLBACK *JsBeforeCollectCallback)(_In_opt_ void *callbackState);

    /// <summary>
    ///     A background work item callback.
    /// </summary>
    /// <remarks>
    ///     This is passed to the host's thread service (if provided) to allow the host to 
    ///     invoke the work item callback on the background thread of its choice.
    /// </remarks>
    /// <param name="callbackState">Data argument passed to the thread service.</param>
    typedef void (CALLBACK *JsBackgroundWorkItemCallback)(_In_opt_ void *callbackState);

    /// <summary>
    ///     A thread service callback.
    /// </summary>
    /// <remarks>
    ///     The host can specify a background thread service when calling <c>JsCreateRuntime</c>. If 
    ///     specified, then background work items will be passed to the host using this callback. The
    ///     host is expected to either begin executing the background work item immediately and return
    ///     true or return false and the runtime will handle the work item in-thread.
    /// </remarks>
    /// <param name="callback">The callback for the background work item.</param>
    /// <param name="callbackState">The data argument to be passed to the callback.</param>
    typedef bool (CALLBACK *JsThreadServiceCallback)(_In_ JsBackgroundWorkItemCallback callback, _In_opt_ void *callbackState);

    /// <summary>
    ///     Version of the runtime.
    /// </summary>
    typedef enum JsRuntimeVersion
    {
        /// <summary>
        ///     Create runtime with IE10 version.
        /// </summary>
        JsRuntimeVersion10 = 0,
        /// <summary>
        ///     Create runtime with IE11 version.
        /// </summary>
        JsRuntimeVersion11 = 1,
    }JsRuntimeVersion;

    /// <summary>
    ///     Create runtime with highest version present on the machine at runtime.
    /// </summary>
    __declspec(deprecated("JsRuntimeVersionEdge is frozen at runtime version 11 when including the jscript9-mode JavaScript Runtime header.  To opt into true edge-mode, define USE_EDGEMODE_JSRT before including jsrt.h.  For more information, go to https://go.microsoft.com/fwlink/?LinkId=522493")) const JsRuntimeVersion JsRuntimeVersionEdge = (JsRuntimeVersion)-1;

    /// <summary>
    ///     Creates a new runtime.
    /// </summary>
    /// <param name="attributes">The attributes of the runtime to be created.</param>
    /// <param name="runtimeVersion">The version of the runtime to be created.</param>
    /// <param name="threadService">The thread service for the runtime. Can be null.</param>
    /// <param name="runtime">The runtime created.</param>
    /// <returns>
    ///     The code <c>JsNoError</c> if the operation succeeded, a failure code otherwise.
    /// </returns>
    STDAPI_(JsErrorCode)
        JsCreateRuntime(
        _In_ JsRuntimeAttributes attributes,
        _In_ JsRuntimeVersion runtimeVersion,
        _In_opt_ JsThreadServiceCallback threadService,
        _Out_ JsRuntimeHandle *runtime);

    /// <summary>
    ///     Performs a full garbage collection.
    /// </summary>
    /// <param name="runtime">The runtime in which the garbage collection will be performed.</param>
    /// <returns>
    ///     The code <c>JsNoError</c> if the operation succeeded, a failure code otherwise.
    /// </returns>
    STDAPI_(JsErrorCode)
        JsCollectGarbage(
        _In_ JsRuntimeHandle runtime);

    /// <summary>
    ///     Disposes a runtime.
    /// </summary>
    /// <remarks>
    ///     Once a runtime has been disposed, all resources owned by it are invalid and cannot be used.
    ///     If the runtime is active (i.e. it is set to be current on a particular thread), it cannot 
    ///     be disposed.
    /// </remarks>
    /// <param name="runtime">The runtime to dispose.</param>
    /// <returns>
    ///     The code <c>JsNoError</c> if the operation succeeded, a failure code otherwise.
    /// </returns>
    STDAPI_(JsErrorCode)
        JsDisposeRuntime(
        _In_ JsRuntimeHandle runtime);

    /// <summary>
    ///     Gets the current memory usage for a runtime.
    /// </summary>
    /// <remarks>
    ///     Memory usage can be always be retrieved, regardless of whether or not the runtime is active
    ///     on another thread.
    /// </remarks>
    /// <param name="runtime">The runtime whose memory usage is to be retrieved.</param>
    /// <param name="memoryUsage">The runtime's current memory usage, in bytes.</param>
    /// <returns>
    ///     The code <c>JsNoError</c> if the operation succeeded, a failure code otherwise.
    /// </returns>
    STDAPI_(JsErrorCode)
        JsGetRuntimeMemoryUsage(
        _In_ JsRuntimeHandle runtime,
        _Out_ size_t *memoryUsage);

    /// <summary>
    ///     Gets the current memory limit for a runtime.
    /// </summary>
    /// <remarks>
    ///     The memory limit of a runtime can be always be retrieved, regardless of whether or not the 
    ///     runtime is active on another thread.
    /// </remarks>
    /// <param name="runtime">The runtime whose memory limit is to be retrieved.</param>
    /// <param name="memoryLimit">
    ///     The runtime's current memory limit, in bytes, or -1 if no limit has been set.
    /// </param>
    /// <returns>
    ///     The code <c>JsNoError</c> if the operation succeeded, a failure code otherwise.
    /// </returns>
    STDAPI_(JsErrorCode)
        JsGetRuntimeMemoryLimit(
        _In_ JsRuntimeHandle runtime,
        _Out_ size_t *memoryLimit);

    /// <summary>
    ///     Sets the current memory limit for a runtime.
    /// </summary>
    /// <remarks>
    ///     <para>
    ///     A memory limit will cause any operation which exceeds the limit to fail with an "out of 
    ///     memory" error. Setting a runtime's memory limit to -1 means that the runtime has no memory 
    ///     limit. New runtimes  default to having no memory limit. If the new memory limit exceeds
    ///     current usage, the call will succeed and any future allocations in this runtime will fail
    ///     until the runtime's memory usage drops below the limit.
    ///     </para>
    ///     <para>
    ///     A runtime's memory limit can be always be set, regardless of whether or not the runtime is 
    ///     active on another thread.
    ///     </para>
    /// </remarks>
    /// <param name="runtime">The runtime whose memory limit is to be set.</param>
    /// <param name="memoryLimit">
    ///     The new runtime memory limit, in bytes, or -1 for no memory limit.
    /// </param>
    /// <returns>
    ///     The code <c>JsNoError</c> if the operation succeeded, a failure code otherwise.
    /// </returns>
    STDAPI_(JsErrorCode)
        JsSetRuntimeMemoryLimit(
        _In_ JsRuntimeHandle runtime,
        _In_ size_t memoryLimit);

    /// <summary>
    ///     Sets a memory allocation callback for specified runtime
    /// </summary>
    /// <remarks>
    ///     <para>
    ///     Registering a memory allocation callback will cause the runtime to call back to the host 
    ///     whenever it acquires memory from, or releases memory to, the OS. The callback routine is
    ///     called before the runtime memory manager allocates a block of memory. The allocation will
    ///     be rejected if the callback returns false. The runtime memory manager will also invoke the
    ///     callback routine after freeing a block of memory, as well as after allocation failures. 
    ///     </para>
    ///     <para>
    ///     The callback is invoked on the current runtime execution thread, therefore execution is 
    ///     blocked until the callback completes.
    ///     </para>
    ///     <para>
    ///     The return value of the callback is not stored; previously rejected allocations will not
    ///     prevent the runtime from invoking the callback again later for new memory allocations.
    ///     </para>
    /// </remarks>
    /// <param name="runtime">The runtime for which to register the allocation callback.</param>
    /// <param name="callbackState">
    ///     User provided state that will be passed back to the callback.
    /// </param>
    /// <param name="allocationCallback">
    ///     Memory allocation callback to be called for memory allocation events.
    /// </param>
    /// <returns>
    ///     The code <c>JsNoError</c> if the operation succeeded, a failure code otherwise.
    /// </returns>
    STDAPI_(JsErrorCode)
        JsSetRuntimeMemoryAllocationCallback(
        _In_ JsRuntimeHandle runtime,
        _In_opt_ void *callbackState,
        _In_ JsMemoryAllocationCallback allocationCallback);

    /// <summary>
    ///     Sets a callback function that is called by the runtime before garbage collection.
    /// </summary>
    /// <remarks>
    ///     <para>
    ///     The callback is invoked on the current runtime execution thread, therefore execution is 
    ///     blocked until the callback completes.
    ///     </para>
    ///     <para>
    ///     The callback can be used by hosts to prepare for garbage collection. For example, by 
    ///     releasing unnecessary references on Chakra objects.
    ///     </para>
    /// </remarks>
    /// <param name="runtime">The runtime for which to register the allocation callback.</param>
    /// <param name="callbackState">
    ///     User provided state that will be passed back to the callback.
    /// </param>
    /// <param name="beforeCollectCallback">The callback function being set.</param>
    /// <returns>
    ///     The code <c>JsNoError</c> if the operation succeeded, a failure code otherwise.
    /// </returns>
    STDAPI_(JsErrorCode)
        JsSetRuntimeBeforeCollectCallback(
        _In_ JsRuntimeHandle runtime,
        _In_opt_ void *callbackState,
        _In_ JsBeforeCollectCallback beforeCollectCallback);

    /// <summary>
    ///     Adds a reference to a garbage collected object.
    /// </summary>
    /// <remarks>
    ///     This only needs to be called on <c>JsRef</c> handles that are not going to be stored 
    ///     somewhere on the stack. Calling <c>JsAddRef</c> ensures that the object the <c>JsRef</c>
    ///     refers to will not be freed until <c>JsRelease</c> is called.
    /// </remarks>
    /// <param name="ref">The object to add a reference to.</param>
    /// <param name="count">The object's new reference count (can pass in null).</param>
    /// <returns>
    ///     The code <c>JsNoError</c> if the operation succeeded, a failure code otherwise.
    /// </returns>
    STDAPI_(JsErrorCode)
        JsAddRef(
        _In_ JsRef ref,
        _Out_opt_ unsigned int *count);

    /// <summary>
    ///     Releases a reference to a garbage collected object.
    /// </summary>
    /// <remarks>
    ///     Removes a reference to a <c>JsRef</c> handle that was created by <c>JsAddRef</c>.
    /// </remarks>
    /// <param name="ref">The object to add a reference to.</param>
    /// <param name="count">The object's new reference count (can pass in null).</param>
    /// <returns>
    ///     The code <c>JsNoError</c> if the operation succeeded, a failure code otherwise.
    /// </returns>
    STDAPI_(JsErrorCode)
        JsRelease(
        _In_ JsRef ref,
        _Out_opt_ unsigned int *count);

    /// <summary>
    ///     Creates a script context for running scripts.
    /// </summary>
    /// <remarks>
    ///     Each script context has its own global object that is isolated from all other script 
    ///     contexts.
    /// </remarks>
    /// <param name="runtime">The runtime the script context is being created in.</param>
    /// <param name="debugApplication">
    ///     The debug application to use for debugging. This parameter can be null, in which case 
    ///     debugging is not enabled for the context.
    /// </param>
    /// <param name="newContext">The created script context.</param>
    /// <returns>
    ///     The code <c>JsNoError</c> if the operation succeeded, a failure code otherwise.
    /// </returns>
    STDAPI_(JsErrorCode)
        JsCreateContext(
        _In_ JsRuntimeHandle runtime,
        _In_ IDebugApplication *debugApplication,
        _Out_ JsContextRef *newContext);

    /// <summary>
    ///     Gets the current script context on the thread.
    /// </summary>
    /// <param name="currentContext">
    ///     The current script context on the thread, null if there is no current script context.
    /// </param>
    /// <returns>
    ///     The code <c>JsNoError</c> if the operation succeeded, a failure code otherwise.
    /// </returns>
    STDAPI_(JsErrorCode)
        JsGetCurrentContext(
        _Out_ JsContextRef *currentContext);

    /// <summary>
    ///     Sets the current script context on the thread.
    /// </summary>
    /// <param name="context">The script context to make current.</param>
    /// <returns>
    ///     The code <c>JsNoError</c> if the operation succeeded, a failure code otherwise.
    /// </returns>
    STDAPI_(JsErrorCode)
        JsSetCurrentContext(
        _In_ JsContextRef context);

    /// <summary>
    ///     Gets the runtime that the context belongs to.
    /// </summary>
    /// <param name="context">The context to get the runtime from.</param>
    /// <param name="runtime">The runtime the context belongs to.</param>
    /// <returns>
    ///     The code <c>JsNoError</c> if the operation succeeded, a failure code otherwise.
    /// </returns>
    STDAPI_(JsErrorCode)
        JsGetRuntime(
        _In_ JsContextRef context,
        _Out_ JsRuntimeHandle *runtime);

    /// <summary>
    ///     Starts debugging in the current context.
    /// </summary>
    /// <param name="debugApplication">The debug application to use for debugging.</param>
    /// <returns>
    ///     The code <c>JsNoError</c> if the operation succeeded, a failure code otherwise.
    /// </returns>
    STDAPI_(JsErrorCode)
        JsStartDebugging(
        _In_ IDebugApplication *debugApplication);



    /// <summary>
    ///     Tells the runtime to do any idle processing it need to do.
    /// </summary>
    /// <remarks>
    ///     <para>
    ///     If idle processing has been enabled for the current runtime, calling <c>JsIdle</c> will 
    ///     inform the current runtime that the host is idle and that the runtime can perform 
    ///     memory cleanup tasks.
    ///     </para>
    ///     <para>
    ///     <c>JsIdle</c> can also return the number of system ticks until there will be more idle work
    ///     for the runtime to do. Calling <c>JsIdle</c> before this number of ticks has passed will do
    ///     no work.
    ///     </para>
    ///     <para>
    ///     Requires an active script context.
    ///     </para>
    /// </remarks>
    /// <param name="nextIdleTick">
    ///     The next system tick when there will be more idle work to do. Can be null. Returns the 
    ///     maximum number of ticks if there no upcoming idle work to do.
    /// </param>
    /// <returns>
    ///     The code <c>JsNoError</c> if the operation succeeded, a failure code otherwise.
    /// </returns>
    STDAPI_(JsErrorCode)
        JsIdle(
        _Out_opt_ unsigned int *nextIdleTick);

    /// <summary>
    ///     Parses a script and returns a function representing the script.
    /// </summary>
    /// <remarks>
    ///     Requires an active script context.
    /// </remarks>
    /// <param name="script">The script to parse.</param>
    /// <param name="sourceContext">
    ///     A cookie identifying the script that can be used by debuggable script contexts.
    /// </param>
    /// <param name="sourceUrl">The location the script came from.</param>
    /// <param name="result">A function representing the script code.</param>
    /// <returns>
    ///     The code <c>JsNoError</c> if the operation succeeded, a failure code otherwise.
    /// </returns>
    STDAPI_(JsErrorCode)
        JsParseScript(
        _In_z_ const wchar_t *script,
        _In_ JsSourceContext sourceContext,
        _In_z_ const wchar_t *sourceUrl,
        _Out_ JsValueRef *result);

    /// <summary>
    ///     Executes a script.
    /// </summary>
    /// <remarks>
    ///     Requires an active script context.
    /// </remarks>
    /// <param name="script">The script to run.</param>
    /// <param name="sourceContext">
    ///     A cookie identifying the script that can be used by debuggable script contexts.
    /// </param>
    /// <param name="sourceUrl">The location the script came from.</param>
    /// <param name="result">The result of the script, if any. This parameter can be null.</param>
    /// <returns>
    ///     The code <c>JsNoError</c> if the operation succeeded, a failure code otherwise.
    /// </returns>
    STDAPI_(JsErrorCode)
        JsRunScript(
        _In_z_ const wchar_t *script,
        _In_ JsSourceContext sourceContext,
        _In_z_ const wchar_t *sourceUrl,
        _Out_ JsValueRef *result);

    /// <summary>
    ///     Serializes a parsed script to a buffer than can be reused.
    /// </summary>
    /// <remarks>
    ///     <para>
    ///     <c>JsSerializeScript</c> parses a script and then stores the parsed form of the script in a 
    ///     runtime-independent format. The serialized script then can be deserialized in any
    ///     runtime without requiring the script to be re-parsed.
    ///     </para>
    ///     <para>
    ///     Requires an active script context.
    ///     </para>
    /// </remarks>
    /// <param name="script">The script to serialize.</param>
    /// <param name="buffer">The buffer to put the serialized script into. Can be null.</param>
    /// <param name="bufferSize">
    ///     On entry, the size of the buffer, in bytes; on exit, the size of the buffer, in bytes, 
    ///     required to hold the serialized script.
    /// </param>
    /// <returns>
    ///     The code <c>JsNoError</c> if the operation succeeded, a failure code otherwise.
    /// </returns>
    STDAPI_(JsErrorCode)
        JsSerializeScript(
        _In_z_ const wchar_t *script,
        _Out_writes_to_opt_(*bufferSize, *bufferSize) BYTE *buffer,
        _Inout_ unsigned long *bufferSize);

    /// <summary>
    ///     Parses a serialized script and returns a function representing the script.
    /// </summary>
    /// <remarks>
    ///     <para>
    ///     Requires an active script context.
    ///     </para>
    ///     <para>
    ///     The runtime will hold on to the buffer until all instances of any functions created from
    ///     the buffer are garbage collected.
    ///     </para>
    /// </remarks>
    /// <param name="script">The script to parse.</param>
    /// <param name="buffer">The serialized script.</param>
    /// <param name="sourceContext">
    ///     A cookie identifying the script that can be used by debuggable script contexts.
    /// </param>
    /// <param name="sourceUrl">The location the script came from.</param>
    /// <param name="result">A function representing the script code.</param>
    /// <returns>
    ///     The code <c>JsNoError</c> if the operation succeeded, a failure code otherwise.
    /// </returns>
    STDAPI_(JsErrorCode)
        JsParseSerializedScript(
        _In_z_ const wchar_t *script,
        _In_ BYTE *buffer,
        _In_ JsSourceContext sourceContext,
        _In_z_ const wchar_t *sourceUrl,
        _Out_ JsValueRef *result);

    /// <summary>
    ///     Runs a serialized script.
    /// </summary>
    /// <remarks>
    ///     <para>
    ///     Requires an active script context.
    ///     </para>
    ///     <para>
    ///     The runtime will hold on to the buffer until all instances of any functions created from
    ///     the buffer are garbage collected.
    ///     </para>
    /// </remarks>
    /// <param name="script">The source code of the serialized script.</param>
    /// <param name="buffer">The serialized script.</param>
    /// <param name="sourceContext">
    ///     A cookie identifying the script that can be used by debuggable script contexts.
    /// </param>
    /// <param name="sourceUrl">The location the script came from.</param>
    /// <param name="result">
    ///     The result of running the script, if any. This parameter can be null.
    /// </param>
    /// <returns>
    ///     The code <c>JsNoError</c> if the operation succeeded, a failure code otherwise.
    /// </returns>
    STDAPI_(JsErrorCode)
        JsRunSerializedScript(
        _In_z_ const wchar_t *script,
        _In_ BYTE *buffer,
        _In_ JsSourceContext sourceContext,
        _In_z_ const wchar_t *sourceUrl,
        _Out_ JsValueRef *result);

    /// <summary>
    ///     Gets the property ID associated with the name. 
    /// </summary>
    /// <remarks>
    ///     <para>
    ///     Property IDs are specific to a context and cannot be used across contexts.
    ///     </para>
    ///     <para>
    ///     Requires an active script context.
    ///     </para>
    /// </remarks>
    /// <param name="name">
    ///     The name of the property ID to get or create. The name may consist of only digits.
    /// </param>
    /// <param name="propertyId">The property ID in this runtime for the given name.</param>
    /// <returns>
    ///     The code <c>JsNoError</c> if the operation succeeded, a failure code otherwise.
    /// </returns>
    STDAPI_(JsErrorCode)
        JsGetPropertyIdFromName(
        _In_z_ const wchar_t *name,
        _Out_ JsPropertyIdRef *propertyId);

    /// <summary>
    ///     Gets the name associated with the property ID.
    /// </summary>
    /// <remarks>
    ///     <para>
    ///     Requires an active script context.
    ///     </para>
    ///     <para>
    ///     The returned buffer is valid as long as the runtime is alive and cannot be used
    ///     once the runtime has been disposed.
    ///     </para>
    /// </remarks>
    /// <param name="propertyId">The property ID to get the name of.</param>
    /// <param name="name">The name associated with the property ID.</param>
    /// <returns>
    ///     The code <c>JsNoError</c> if the operation succeeded, a failure code otherwise.
    /// </returns>
    STDAPI_(JsErrorCode)
        JsGetPropertyNameFromId(
        _In_ JsPropertyIdRef propertyId,
        _Outptr_result_z_ const wchar_t **name);

    /// <summary>
    ///     Gets the value of <c>undefined</c> in the current script context.
    /// </summary>
    /// <remarks>
    ///     Requires an active script context.
    /// </remarks>
    /// <param name="undefinedValue">The <c>undefined</c> value.</param>
    /// <returns>
    ///     The code <c>JsNoError</c> if the operation succeeded, a failure code otherwise.
    /// </returns>
    STDAPI_(JsErrorCode)
        JsGetUndefinedValue(
        _Out_ JsValueRef *undefinedValue);

    /// <summary>
    ///     Gets the value of <c>null</c> in the current script context.
    /// </summary>
    /// <remarks>
    ///     Requires an active script context.
    /// </remarks>
    /// <param name="nullValue">The <c>null</c> value.</param>
    /// <returns>
    ///     The code <c>JsNoError</c> if the operation succeeded, a failure code otherwise.
    /// </returns>
    STDAPI_(JsErrorCode)
        JsGetNullValue(
        _Out_ JsValueRef *nullValue);

    /// <summary>
    ///     Gets the value of <c>true</c> in the current script context.
    /// </summary>
    /// <remarks>
    ///     Requires an active script context.
    /// </remarks>
    /// <param name="trueValue">The <c>true</c> value.</param>
    /// <returns>
    ///     The code <c>JsNoError</c> if the operation succeeded, a failure code otherwise.
    /// </returns>
    STDAPI_(JsErrorCode)
        JsGetTrueValue(
        _Out_ JsValueRef *trueValue);

    /// <summary>
    ///     Gets the value of <c>false</c> in the current script context.
    /// </summary>
    /// <remarks>
    ///     Requires an active script context.
    /// </remarks>
    /// <param name="falseValue">The <c>false</c> value.</param>
    /// <returns>
    ///     The code <c>JsNoError</c> if the operation succeeded, a failure code otherwise.
    /// </returns>
    STDAPI_(JsErrorCode)
        JsGetFalseValue(
        _Out_ JsValueRef *falseValue);

    /// <summary>
    ///     Creates a Boolean value from a <c>bool</c> value.
    /// </summary>
    /// <remarks>
    ///     Requires an active script context.
    /// </remarks>
    /// <param name="value">The value to be converted.</param>
    /// <param name="booleanValue">The converted value.</param>
    /// <returns>
    ///     The code <c>JsNoError</c> if the operation succeeded, a failure code otherwise.
    /// </returns>
    STDAPI_(JsErrorCode)
        JsBoolToBoolean(
        _In_ bool value,
        _Out_ JsValueRef *booleanValue);

    /// <summary>
    ///     Retrieves the <c>bool</c> value of a Boolean value.
    /// </summary>
    /// <remarks>
    ///     Requires an active script context.
    /// </remarks>
    /// <param name="value">The value to be converted.</param>
    /// <param name="boolValue">The converted value.</param>
    /// <returns>
    ///     The code <c>JsNoError</c> if the operation succeeded, a failure code otherwise.
    /// </returns>
    STDAPI_(JsErrorCode)
        JsBooleanToBool(
        _In_ JsValueRef value,
        _Out_ bool *boolValue);

    /// <summary>
    ///     Converts the value to Boolean using standard JavaScript semantics.
    /// </summary>
    /// <remarks>
    ///     Requires an active script context.
    /// </remarks>
    /// <param name="value">The value to be converted.</param>
    /// <param name="booleanValue">The converted value.</param>
    /// <returns>
    ///     The code <c>JsNoError</c> if the operation succeeded, a failure code otherwise.
    /// </returns>
    STDAPI_(JsErrorCode)
        JsConvertValueToBoolean(
        _In_ JsValueRef value,
        _Out_ JsValueRef *booleanValue);

    /// <summary>
    ///     The JavaScript type of a JsValueRef.
    /// </summary>
    typedef enum JsValueType
    {
        /// <summary>
        ///     The value is the <c>undefined</c> value.
        /// </summary>
        JsUndefined = 0,
        /// <summary>
        ///     The value is the <c>null</c> value.
        /// </summary>
        JsNull = 1,
        /// <summary>
        ///     The value is a JavaScript number value.
        /// </summary>
        JsNumber = 2,
        /// <summary>
        ///     The value is a JavaScript string value.
        /// </summary>
        JsString = 3,
        /// <summary>
        ///     The value is a JavaScript Boolean value.
        /// </summary>
        JsBoolean = 4,
        /// <summary>
        ///     The value is a JavaScript object value.
        /// </summary>
        JsObject = 5,
        /// <summary>
        ///     The value is a JavaScript function object value.
        /// </summary>
        JsFunction = 6,
        /// <summary>
        ///     The value is a JavaScript error object value.
        /// </summary>
        JsError = 7,
        /// <summary>
        ///     The value is a JavaScript array object value.
        /// </summary>
        JsArray = 8,
    }JsValueType;

    /// <summary>
    ///     Gets the JavaScript type of a JsValueRef.
    /// </summary>
    /// <remarks>
    ///     Requires an active script context.
    /// </remarks>
    /// <param name="value">The value whose type is to be returned.</param>
    /// <param name="type">The type of the value.</param>
    /// <returns>
    ///     The code <c>JsNoError</c> if the operation succeeded, a failure code otherwise.
    /// </returns>
    STDAPI_(JsErrorCode)
        JsGetValueType(
        _In_ JsValueRef value,
        _Out_ JsValueType *type);

    /// <summary>
    ///     Creates a number value from a <c>double</c> value.
    /// </summary>
    /// <remarks>
    ///     Requires an active script context.
    /// </remarks>
    /// <param name="doubleValue">The <c>double</c> to convert to a number value.</param>
    /// <param name="value">The new number value.</param>
    /// <returns>
    ///     The code <c>JsNoError</c> if the operation succeeded, a failure code otherwise.
    /// </returns>
    STDAPI_(JsErrorCode)
        JsDoubleToNumber(
        _In_ double doubleValue,
        _Out_ JsValueRef *value);

    /// <summary>
    ///     Creates a number value from an <c>int</c> value.
    /// </summary>
    /// <remarks>
    ///     Requires an active script context.
    /// </remarks>
    /// <param name="intValue">The <c>int</c> to convert to a number value.</param>
    /// <param name="value">The new number value.</param>
    /// <returns>
    ///     The code <c>JsNoError</c> if the operation succeeded, a failure code otherwise.
    /// </returns>
    STDAPI_(JsErrorCode)
        JsIntToNumber(
        _In_ int intValue,
        _Out_ JsValueRef *value);

    /// <summary>
    ///     Retrieves the <c>double</c> value of a number value.
    /// </summary>
    /// <remarks>
    ///     <para>
    ///     This function retrieves the value of a number value. It will fail with 
    ///     <c>JsErrorInvalidArgument</c> if the type of the value is not number.
    ///     </para>
    ///     <para>
    ///     Requires an active script context.
    ///     </para>
    /// </remarks>
    /// <param name="value">The number value to convert to a <c>double</c> value.</param>
    /// <param name="doubleValue">The <c>double</c> value.</param>
    /// <returns>
    ///     The code <c>JsNoError</c> if the operation succeeded, a failure code otherwise.
    /// </returns>
    STDAPI_(JsErrorCode)
        JsNumberToDouble(
        _In_ JsValueRef value,
        _Out_ double *doubleValue);

    /// <summary>
    ///     Converts the value to number using standard JavaScript semantics.
    /// </summary>
    /// <remarks>
    ///     Requires an active script context.
    /// </remarks>
    /// <param name="value">The value to be converted.</param>
    /// <param name="numberValue">The converted value.</param>
    /// <returns>
    ///     The code <c>JsNoError</c> if the operation succeeded, a failure code otherwise.
    /// </returns>
    STDAPI_(JsErrorCode)
        JsConvertValueToNumber(
        _In_ JsValueRef value,
        _Out_ JsValueRef *numberValue);

    /// <summary>
    ///     Gets the length of a string value.
    /// </summary>
    /// <remarks>
    ///     Requires an active script context.
    /// </remarks>
    /// <param name="stringValue">The string value to get the length of.</param>
    /// <param name="length">The length of the string.</param>
    /// <returns>
    ///     The code <c>JsNoError</c> if the operation succeeded, a failure code otherwise.
    /// </returns>
    STDAPI_(JsErrorCode)
        JsGetStringLength(
        _In_ JsValueRef stringValue,
        _Out_ int *length);

    /// <summary>
    ///     Creates a string value from a string pointer.
    /// </summary>
    /// <remarks>
    ///     Requires an active script context.
    /// </remarks>
    /// <param name="stringValue">The string pointer to convert to a string value.</param>
    /// <param name="stringLength">The length of the string to convert.</param>
    /// <param name="value">The new string value.</param>
    /// <returns>
    ///     The code <c>JsNoError</c> if the operation succeeded, a failure code otherwise.
    /// </returns>
    STDAPI_(JsErrorCode)
        JsPointerToString(
        _In_reads_(stringLength) const wchar_t *stringValue,
        _In_ size_t stringLength,
        _Out_ JsValueRef *value);

    /// <summary>
    ///     Retrieves the string pointer of a string value.
    /// </summary>
    /// <remarks>
    ///     <para>
    ///     This function retrieves the string pointer of a string value. It will fail with 
    ///     <c>JsErrorInvalidArgument</c> if the type of the value is not string. The lifetime
    ///     of the string returned will be the same as the lifetime of the value it came from, however
    ///     the string pointer is not considered a reference to the value (and so will not keep it
    ///     from being collected).
    ///     </para>
    ///     <para>
    ///     Requires an active script context.
    ///     </para>
    /// </remarks>
    /// <param name="value">The string value to convert to a string pointer.</param>
    /// <param name="stringValue">The string pointer.</param>
    /// <param name="stringLength">The length of the string.</param>
    /// <returns>
    ///     The code <c>JsNoError</c> if the operation succeeded, a failure code otherwise.
    /// </returns>
    STDAPI_(JsErrorCode)
        JsStringToPointer(
        _In_ JsValueRef value,
        _Outptr_result_buffer_(*stringLength) const wchar_t **stringValue,
        _Out_ size_t *stringLength);

    /// <summary>
    ///     Converts the value to string using standard JavaScript semantics.
    /// </summary>
    /// <remarks>
    ///     Requires an active script context.
    /// </remarks>
    /// <param name="value">The value to be converted.</param>
    /// <param name="stringValue">The converted value.</param>
    /// <returns>
    ///     The code <c>JsNoError</c> if the operation succeeded, a failure code otherwise.
    /// </returns>
    STDAPI_(JsErrorCode)
        JsConvertValueToString(
        _In_ JsValueRef value,
        _Out_ JsValueRef *stringValue);

    /// <summary>
    ///     Creates a JavaScript value that is a projection of the passed in <c>VARIANT</c>.
    /// </summary>
    /// <remarks>
    ///     <para>
    ///     The projected value can be used by script to call a COM automation object from script. 
    ///     Hosts are responsible for enforcing COM threading rules.
    ///     </para>
    ///     <para>
    ///     Requires an active script context.
    ///     </para>
    /// </remarks>
    /// <param name="variant">A <c>VARIANT</c> to be projected.</param>
    /// <param name="value">A JavaScript value that is a projection of the <c>VARIANT</c>.</param>
    /// <returns>
    ///     The code <c>JsNoError</c> if the operation succeeded, a failure code otherwise.
    /// </returns>
    STDAPI_(JsErrorCode)
        JsVariantToValue(
        _In_ VARIANT *variant,
        _Out_ JsValueRef *value);

    /// <summary>
    ///     Initializes the passed in <c>VARIANT</c> as a projection of a JavaScript value.
    /// </summary>
    /// <remarks>
    ///     <para>
    ///     The projection <c>VARIANT</c> can be used by COM automation clients to call into the
    ///     projected JavaScript object.
    ///     </para>
    ///     <para>
    ///     Requires an active script context.
    ///     </para>
    /// </remarks>
    /// <param name="object">A JavaScript value to project as a <c>VARIANT</c>.</param>
    /// <param name="variant">
    ///     A pointer to a <c>VARIANT</c> struct that will be initialized as a projection.
    /// </param>
    STDAPI_(JsErrorCode)
        JsValueToVariant(
        _In_ JsValueRef object,
        _Out_ VARIANT *variant);

    /// <summary>
    ///     Gets the global object in the current script context.
    /// </summary>
    /// <remarks>
    ///     Requires an active script context.
    /// </remarks>
    /// <param name="globalObject">The global object.</param>
    /// <returns>
    ///     The code <c>JsNoError</c> if the operation succeeded, a failure code otherwise.
    /// </returns>
    STDAPI_(JsErrorCode)
        JsGetGlobalObject(
        _Out_ JsValueRef *globalObject);

    /// <summary>
    ///     Creates a new object.
    /// </summary>
    /// <remarks>
    ///     Requires an active script context.
    /// </remarks>
    /// <param name="object">The new object.</param>
    /// <returns>
    ///     The code <c>JsNoError</c> if the operation succeeded, a failure code otherwise.
    /// </returns>
    STDAPI_(JsErrorCode)
        JsCreateObject(
        _Out_ JsValueRef *object);

    /// <summary>
    ///     A finalizer callback.
    /// </summary>
    /// <param name="data">
    ///     The external data that was passed in when creating the object being finalized.
    /// </param>
    typedef void (CALLBACK *JsFinalizeCallback)(_In_opt_ void *data);

    /// <summary>
    ///     Creates a new object that stores some external data.
    /// </summary>
    /// <remarks>
    ///     Requires an active script context.
    /// </remarks>
    /// <param name="data">External data that the object will represent. May be null.</param>
    /// <param name="finalizeCallback">
    ///     A callback for when the object is finalized. May be null.
    /// </param>
    /// <param name="object">The new object.</param>
    /// <returns>
    ///     The code <c>JsNoError</c> if the operation succeeded, a failure code otherwise.
    /// </returns>
    STDAPI_(JsErrorCode)
        JsCreateExternalObject(
        _In_opt_ void *data,
        _In_opt_ JsFinalizeCallback finalizeCallback,
        _Out_ JsValueRef *object);

    /// <summary>
    ///     Converts the value to object using standard JavaScript semantics.
    /// </summary>
    /// <remarks>
    ///     Requires an active script context.
    /// </remarks>
    /// <param name="value">The value to be converted.</param>
    /// <param name="object">The converted value.</param>
    /// <returns>
    ///     The code <c>JsNoError</c> if the operation succeeded, a failure code otherwise.
    /// </returns>
    STDAPI_(JsErrorCode)
        JsConvertValueToObject(
        _In_ JsValueRef value,
        _Out_ JsValueRef *object);

    /// <summary>
    ///     Returns the prototype of an object.
    /// </summary>
    /// <remarks>
    ///     Requires an active script context.
    /// </remarks>
    /// <param name="object">The object whose prototype is to be returned.</param>
    /// <param name="prototypeObject">The object's prototype.</param>
    /// <returns>
    ///     The code <c>JsNoError</c> if the operation succeeded, a failure code otherwise.
    /// </returns>
    STDAPI_(JsErrorCode)
        JsGetPrototype(
        _In_ JsValueRef object,
        _Out_ JsValueRef *prototypeObject);

    /// <summary>
    ///     Sets the prototype of an object.
    /// </summary>
    /// <remarks>
    ///     Requires an active script context.
    /// </remarks>
    /// <param name="object">The object whose prototype is to be changed.</param>
    /// <param name="prototypeObject">The object's new prototype.</param>
    /// <returns>
    ///     The code <c>JsNoError</c> if the operation succeeded, a failure code otherwise.
    /// </returns>
    STDAPI_(JsErrorCode)
        JsSetPrototype(
        _In_ JsValueRef object,
        _In_ JsValueRef prototypeObject);

    /// <summary>
    ///     Returns a value that indicates whether an object is extensible or not.
    /// </summary>
    /// <remarks>
    ///     Requires an active script context.
    /// </remarks>
    /// <param name="object">The object to test.</param>
    /// <param name="value">Whether the object is extensible or not.</param>
    /// <returns>
    ///     The code <c>JsNoError</c> if the operation succeeded, a failure code otherwise.
    /// </returns>
    STDAPI_(JsErrorCode)
        JsGetExtensionAllowed(
        _In_ JsValueRef object,
        _Out_ bool *value);

    /// <summary>
    ///     Makes an object non-extensible.
    /// </summary>
    /// <remarks>
    ///     Requires an active script context.
    /// </remarks>
    /// <param name="object">The object to make non-extensible.</param>
    /// <returns>
    ///     The code <c>JsNoError</c> if the operation succeeded, a failure code otherwise.
    /// </returns>
    STDAPI_(JsErrorCode)
        JsPreventExtension(
        _In_ JsValueRef object);

    /// <summary>
    ///     Gets an object's property.
    /// </summary>
    /// <remarks>
    ///     Requires an active script context.
    /// </remarks>
    /// <param name="object">The object that contains the property.</param>
    /// <param name="propertyId">The ID of the property.</param>
    /// <param name="value">The value of the property.</param>
    /// <returns>
    ///     The code <c>JsNoError</c> if the operation succeeded, a failure code otherwise.
    /// </returns>
    STDAPI_(JsErrorCode)
        JsGetProperty(
        _In_ JsValueRef object,
        _In_ JsPropertyIdRef propertyId,
        _Out_ JsValueRef *value);

    /// <summary>
    ///     Gets a property descriptor for an object's own property.
    /// </summary>
    /// <remarks>
    ///     Requires an active script context.
    /// </remarks>
    /// <param name="object">The object that has the property.</param>
    /// <param name="propertyId">The ID of the property.</param>
    /// <param name="propertyDescriptor">The property descriptor.</param>
    /// <returns>
    ///     The code <c>JsNoError</c> if the operation succeeded, a failure code otherwise.
    /// </returns>
    STDAPI_(JsErrorCode)
        JsGetOwnPropertyDescriptor(
        _In_ JsValueRef object,
        _In_ JsPropertyIdRef propertyId,
        _Out_ JsValueRef *propertyDescriptor);

    /// <summary>
    ///     Gets the list of all properties on the object.
    /// </summary>
    /// <remarks>
    ///     Requires an active script context.
    /// </remarks>
    /// <param name="object">The object from which to get the property names.</param>
    /// <param name="propertyNames">An array of property names.</param>
    /// <returns>
    ///     The code <c>JsNoError</c> if the operation succeeded, a failure code otherwise.
    /// </returns>
    STDAPI_(JsErrorCode)
        JsGetOwnPropertyNames(
        _In_ JsValueRef object,
        _Out_ JsValueRef *propertyNames);

    /// <summary>
    ///     Puts an object's property.
    /// </summary>
    /// <remarks>
    ///     Requires an active script context.
    /// </remarks>
    /// <param name="object">The object that contains the property.</param>
    /// <param name="propertyId">The ID of the property.</param>
    /// <param name="value">The new value of the property.</param>
    /// <param name="useStrictRules">The property set should follow strict mode rules.</param>
    /// <returns>
    ///     The code <c>JsNoError</c> if the operation succeeded, a failure code otherwise.
    /// </returns>
    STDAPI_(JsErrorCode)
        JsSetProperty(
        _In_ JsValueRef object,
        _In_ JsPropertyIdRef propertyId,
        _In_ JsValueRef value,
        _In_ bool useStrictRules);

    /// <summary>
    ///     Determines whether an object has a property.
    /// </summary>
    /// <remarks>
    ///     Requires an active script context.
    /// </remarks>
    /// <param name="object">The object that may contain the property.</param>
    /// <param name="propertyId">The ID of the property.</param>
    /// <param name="hasProperty">Whether the object (or a prototype) has the property.</param>
    /// <returns>
    ///     The code <c>JsNoError</c> if the operation succeeded, a failure code otherwise.
    /// </returns>
    STDAPI_(JsErrorCode)
        JsHasProperty(
        _In_ JsValueRef object,
        _In_ JsPropertyIdRef propertyId,
        _Out_ bool *hasProperty);

    /// <summary>
    ///     Deletes an object's property.
    /// </summary>
    /// <remarks>
    ///     Requires an active script context.
    /// </remarks>
    /// <param name="object">The object that contains the property.</param>
    /// <param name="propertyId">The ID of the property.</param>
    /// <param name="result">Whether the property was deleted.</param>
    /// <param name="useStrictRules">The property set should follow strict mode rules.</param>
    /// <returns>
    ///     The code <c>JsNoError</c> if the operation succeeded, a failure code otherwise.
    /// </returns>
    STDAPI_(JsErrorCode)
        JsDeleteProperty(
        _In_ JsValueRef object,
        _In_ JsPropertyIdRef propertyId,
        _In_ bool useStrictRules,
        _Out_ JsValueRef *result);

    /// <summary>
    ///     Defines a new object's own property from a property descriptor.
    /// </summary>
    /// <remarks>
    ///     Requires an active script context.
    /// </remarks>
    /// <param name="object">The object that has the property.</param>
    /// <param name="propertyId">The ID of the property.</param>
    /// <param name="propertyDescriptor">The property descriptor.</param>
    /// <param name="result">Whether the property was defined.</param>
    /// <returns>
    ///     The code <c>JsNoError</c> if the operation succeeded, a failure code otherwise.
    /// </returns>
    STDAPI_(JsErrorCode)
        JsDefineProperty(
        _In_ JsValueRef object,
        _In_ JsPropertyIdRef propertyId,
        _In_ JsValueRef propertyDescriptor,
        _Out_ bool *result);

    /// <summary>
    ///     Tests whether an object has a value at the specified index.
    /// </summary>
    /// <remarks>
    ///     Requires an active script context.
    /// </remarks>
    /// <param name="object">The object to operate on.</param>
    /// <param name="index">The index to test.</param>
    /// <param name="result">Whether the object has an value at the specified index.</param>
    /// <returns>
    ///     The code <c>JsNoError</c> if the operation succeeded, a failure code otherwise.
    /// </returns>
    STDAPI_(JsErrorCode)
        JsHasIndexedProperty(
        _In_ JsValueRef object,
        _In_ JsValueRef index,
        _Out_ bool *result);

    /// <summary>
    ///     Retrieve the value at the specified index of an object.
    /// </summary>
    /// <remarks>
    ///     Requires an active script context.
    /// </remarks>
    /// <param name="object">The object to operate on.</param>
    /// <param name="index">The index to retrieve.</param>
    /// <param name="result">The retrieved value.</param>
    /// <returns>
    ///     The code <c>JsNoError</c> if the operation succeeded, a failure code otherwise.
    /// </returns>
    STDAPI_(JsErrorCode)
        JsGetIndexedProperty(
        _In_ JsValueRef object,
        _In_ JsValueRef index,
        _Out_ JsValueRef *result);

    /// <summary>
    ///     Set the value at the specified index of an object.
    /// </summary>
    /// <remarks>
    ///     Requires an active script context.
    /// </remarks>
    /// <param name="object">The object to operate on.</param>
    /// <param name="index">The index to set.</param>
    /// <param name="value">The value to set.</param>
    /// <returns>
    ///     The code <c>JsNoError</c> if the operation succeeded, a failure code otherwise.
    /// </returns>
    STDAPI_(JsErrorCode)
        JsSetIndexedProperty(
        _In_ JsValueRef object,
        _In_ JsValueRef index,
        _In_ JsValueRef value);

    /// <summary>
    ///     Delete the value at the specified index of an object.
    /// </summary>
    /// <remarks>
    ///     Requires an active script context.
    /// </remarks>
    /// <param name="object">The object to operate on.</param>
    /// <param name="index">The index to delete.</param>
    /// <returns>
    ///     The code <c>JsNoError</c> if the operation succeeded, a failure code otherwise.
    /// </returns>
    STDAPI_(JsErrorCode)
        JsDeleteIndexedProperty(
        _In_ JsValueRef object,
        _In_ JsValueRef index);

    /// <summary>
    ///     Compare two JavaScript values for equality.
    /// </summary>
    /// <remarks>
    ///     <para>
    ///     This function is equivalent to the <c>==</c> operator in Javascript.
    ///     </para>
    ///     <para>
    ///     Requires an active script context.
    ///     </para>
    /// </remarks>
    /// <param name="object1">The first object to compare.</param>
    /// <param name="object2">The second object to compare.</param>
    /// <param name="result">Whether the values are equal.</param>
    /// <returns>
    ///     The code <c>JsNoError</c> if the operation succeeded, a failure code otherwise.
    /// </returns>
    STDAPI_(JsErrorCode)
        JsEquals(
        _In_ JsValueRef object1,
        _In_ JsValueRef object2,
        _Out_ bool *result);

    /// <summary>
    ///     Compare two JavaScript values for strict equality.
    /// </summary>
    /// <remarks>
    ///     <para>
    ///     This function is equivalent to the <c>===</c> operator in Javascript.
    ///     </para>
    ///     <para>
    ///     Requires an active script context.
    ///     </para>
    /// </remarks>
    /// <param name="object1">The first object to compare.</param>
    /// <param name="object2">The second object to compare.</param>
    /// <param name="result">Whether the values are strictly equal.</param>
    /// <returns>
    ///     The code <c>JsNoError</c> if the operation succeeded, a failure code otherwise.
    /// </returns>
    STDAPI_(JsErrorCode)
        JsStrictEquals(
        _In_ JsValueRef object1,
        _In_ JsValueRef object2,
        _Out_ bool *result);

    /// <summary>
    ///     Determines whether an object is an external object.
    /// </summary>
    /// <remarks>
    ///     Requires an active script context.
    /// </remarks>
    /// <param name="object">The object.</param>
    /// <param name="value">Whether the object is an external object.</param>
    /// <returns>
    ///     The code <c>JsNoError</c> if the operation succeeded, a failure code otherwise.
    /// </returns>
    STDAPI_(JsErrorCode)
        JsHasExternalData(
        _In_ JsValueRef object,
        _Out_ bool *value);

    /// <summary>
    ///     Retrieves the data from an external object.
    /// </summary>
    /// <remarks>
    ///     Requires an active script context.
    /// </remarks>
    /// <param name="object">The external object.</param>
    /// <param name="externalData">
    ///     The external data stored in the object. Can be null if no external data is stored in the 
    ///     object.
    /// </param>
    /// <returns>
    ///     The code <c>JsNoError</c> if the operation succeeded, a failure code otherwise.
    /// </returns>
    STDAPI_(JsErrorCode)
        JsGetExternalData(
        _In_ JsValueRef object,
        _Out_ void **externalData);

    /// <summary>
    ///     Sets the external data on an external object.
    /// </summary>
    /// <remarks>
    ///     Requires an active script context.
    /// </remarks>
    /// <param name="object">The external object.</param>
    /// <param name="externalData">
    ///     The external data to be stored in the object. Can be null if no external data is 
    ///     to be stored in the object.
    /// </param>
    /// <returns>
    ///     The code <c>JsNoError</c> if the operation succeeded, a failure code otherwise.
    /// </returns>
    STDAPI_(JsErrorCode)
        JsSetExternalData(
        _In_ JsValueRef object,
        _In_opt_ void *externalData);

    /// <summary>
    ///     Creates a Javascript array object.
    /// </summary>
    /// <remarks>
    ///     Requires an active script context.
    /// </remarks>
    /// <param name="length">The initial length of the array.</param>
    /// <param name="result">The new array object.</param>
    /// <returns>
    ///     The code <c>JsNoError</c> if the operation succeeded, a failure code otherwise.
    /// </returns>
    STDAPI_(JsErrorCode)
        JsCreateArray(
        _In_ unsigned int length,
        _Out_ JsValueRef *result);

    /// <summary>
    ///     Invokes a function.
    /// </summary>
    /// <remarks>
    ///     Requires an active script context.
    /// </remarks>
    /// <param name="function">The function to invoke.</param>
    /// <param name="arguments">The arguments to the call.</param>
    /// <param name="argumentCount">The number of arguments being passed in to the function.</param>
    /// <param name="result">The value returned from the function invocation, if any.</param>
    /// <returns>
    ///     The code <c>JsNoError</c> if the operation succeeded, a failure code otherwise.
    /// </returns>
    STDAPI_(JsErrorCode)
        JsCallFunction(
        _In_ JsValueRef function,
        _In_reads_(argumentCount) JsValueRef *arguments,
        _In_ unsigned short argumentCount,
        _Out_opt_ JsValueRef *result);

    /// <summary>
    ///     Invokes a function as a constructor.
    /// </summary>
    /// <remarks>
    ///     Requires an active script context.
    /// </remarks>
    /// <param name="function">The function to invoke as a constructor.</param>
    /// <param name="arguments">The arguments to the call.</param>
    /// <param name="argumentCount">The number of arguments being passed in to the function.</param>
    /// <param name="result">The value returned from the function invocation.</param>
    /// <returns>
    ///     The code <c>JsNoError</c> if the operation succeeded, a failure code otherwise.
    /// </returns>
    STDAPI_(JsErrorCode)
        JsConstructObject(
        _In_ JsValueRef function,
        _In_reads_(argumentCount) JsValueRef *arguments,
        _In_ unsigned short argumentCount,
        _Out_ JsValueRef *result);

    /// <summary>
    ///     A function callback.
    /// </summary>
    /// <param name="callee">
    ///     A function object that represents the function being invoked.
    /// </param>
    /// <param name="isConstructCall">Indicates whether this is a regular call or a 'new' call.</param>
    /// <param name="arguments">The arguments to the call.</param>
    /// <param name="argumentCount">The number of arguments.</param>
    /// <param name="callbackState">
    ///     The state passed to <c>JsCreateFunction</c>.
    /// </param>
    /// <returns>The result of the call, if any.</returns>
    typedef _Ret_maybenull_ JsValueRef(CALLBACK * JsNativeFunction)(_In_ JsValueRef callee, _In_ bool isConstructCall, _In_ JsValueRef *arguments, _In_ unsigned short argumentCount, _In_opt_ void *callbackState);

    /// <summary>
    ///     Creates a new JavaScript function.
    /// </summary>
    /// <remarks>
    ///     Requires an active script context.
    /// </remarks>
    /// <param name="nativeFunction">The method to call when the function is invoked.</param>
    /// <param name="callbackState">
    ///     User provided state that will be passed back to the callback.
    /// </param>
    /// <param name="function">The new function object.</param>
    /// <returns>The result of the call, if any.</returns>
    STDAPI_(JsErrorCode)
        JsCreateFunction(
        _In_ JsNativeFunction nativeFunction,
        _In_opt_ void *callbackState,
        _Out_ JsValueRef *function);

    /// <summary>
    ///     Creates a new JavaScript error object
    /// </summary>
    /// <remarks>
    ///     Requires an active script context.
    /// </remarks>
    /// <param name="message">Message for the error object.</param>
    /// <param name="error">The new error object.</param>
    /// <returns>
    ///     The code <c>JsNoError</c> if the operation succeeded, a failure code otherwise.
    /// </returns>
    STDAPI_(JsErrorCode)
        JsCreateError(
        _In_ JsValueRef message,
        _Out_ JsValueRef *error);

    /// <summary>
    ///     Creates a new JavaScript RangeError error object
    /// </summary>
    /// <remarks>
    ///     Requires an active script context.
    /// </remarks>
    /// <param name="message">Message for the error object.</param>
    /// <param name="error">The new error object.</param>
    /// <returns>
    ///     The code <c>JsNoError</c> if the operation succeeded, a failure code otherwise.
    /// </returns>
    STDAPI_(JsErrorCode)
        JsCreateRangeError(
        _In_ JsValueRef message,
        _Out_ JsValueRef *error);

    /// <summary>
    ///     Creates a new JavaScript ReferenceError error object
    /// </summary>
    /// <remarks>
    ///     Requires an active script context.
    /// </remarks>
    /// <param name="message">Message for the error object.</param>
    /// <param name="error">The new error object.</param>
    /// <returns>
    ///     The code <c>JsNoError</c> if the operation succeeded, a failure code otherwise.
    /// </returns>
    STDAPI_(JsErrorCode)
        JsCreateReferenceError(
        _In_ JsValueRef message,
        _Out_ JsValueRef *error);

    /// <summary>
    ///     Creates a new JavaScript SyntaxError error object
    /// </summary>
    /// <remarks>
    ///     Requires an active script context.
    /// </remarks>
    /// <param name="message">Message for the error object.</param>
    /// <param name="error">The new error object.</param>
    /// <returns>
    ///     The code <c>JsNoError</c> if the operation succeeded, a failure code otherwise.
    /// </returns>
    STDAPI_(JsErrorCode)
        JsCreateSyntaxError(
        _In_ JsValueRef message,
        _Out_ JsValueRef *error);

    /// <summary>
    ///     Creates a new JavaScript TypeError error object
    /// </summary>
    /// <remarks>
    ///     Requires an active script context.
    /// </remarks>
    /// <param name="message">Message for the error object.</param>
    /// <param name="error">The new error object.</param>
    /// <returns>
    ///     The code <c>JsNoError</c> if the operation succeeded, a failure code otherwise.
    /// </returns>
    STDAPI_(JsErrorCode)
        JsCreateTypeError(
        _In_ JsValueRef message,
        _Out_ JsValueRef *error);

    /// <summary>
    ///     Creates a new JavaScript URIError error object
    /// </summary>
    /// <remarks>
    ///     Requires an active script context.
    /// </remarks>
    /// <param name="message">Message for the error object.</param>
    /// <param name="error">The new error object.</param>
    /// <returns>
    ///     The code <c>JsNoError</c> if the operation succeeded, a failure code otherwise.
    /// </returns>
    STDAPI_(JsErrorCode)
        JsCreateURIError(
        _In_ JsValueRef message,
        _Out_ JsValueRef *error);

    /// <summary>
    ///     Determines whether the runtime of the current context is in an exception state.
    /// </summary>
    /// <remarks>
    ///     <para>
    ///     If a call into the runtime results in an exception (either as the result of running a 
    ///     script or due to something like a conversion failure), the runtime is placed into an 
    ///     "exception state." All calls into any context created by the runtime (except for the 
    ///     exception APIs) will fail with <c>JsErrorInExceptionState</c> until the exception is 
    ///     cleared.
    ///     </para>
    ///     <para>
    ///     If the runtime of the current context is in the exception state when a callback returns 
    ///     into the engine, the engine will automatically rethrow the exception.
    ///     </para>
    ///     <para>
    ///     Requires an active script context.
    ///     </para>
    /// </remarks>
    /// <param name="hasException">
    ///     Whether the runtime of the current context is in the exception state.
    /// </param>
    /// <returns>
    ///     The code <c>JsNoError</c> if the operation succeeded, a failure code otherwise.
    /// </returns>
    STDAPI_(JsErrorCode)
        JsHasException(
        _Out_ bool *hasException);

    /// <summary>
    ///     Returns the exception that caused the runtime of the current context to be in the 
    ///     exception state and resets the exception state for that runtime.
    /// </summary>
    /// <remarks>
    ///     <para>
    ///     If the runtime of the current context is not in an exception state, this API will return
    ///     <c>JsErrorInvalidArgument</c>. If the runtime is disabled, this will return an exception
    ///     indicating that the script was terminated, but it will not clear the exception (the 
    ///     exception will be cleared if the runtime is re-enabled using 
    ///     <c>JsEnableRuntimeExecution</c>).
    ///     </para>
    ///     <para>
    ///     Requires an active script context.
    ///     </para>
    /// </remarks>
    /// <param name="exception">The exception for the runtime of the current context.</param>
    /// <returns>
    ///     The code <c>JsNoError</c> if the operation succeeded, a failure code otherwise.
    /// </returns>
    STDAPI_(JsErrorCode)
        JsGetAndClearException(
        _Out_ JsValueRef *exception);

    /// <summary>
    ///     Sets the runtime of the current context to an exception state.
    /// </summary>
    /// <remarks>
    ///     <para>
    ///     If the runtime of the current context is already in an exception state, this API will 
    ///     return <c>JsErrorInExceptionState</c>.
    ///     </para>
    ///     <para>
    ///     Requires an active script context.
    ///     </para>
    /// </remarks>
    /// <param name="exception">
    ///     The JavaScript exception to set for the runtime of the current context.
    /// </param>
    /// <returns>
    ///     JsNoError if the engine was set into an exception state, a failure code otherwise.
    /// </returns>
    STDAPI_(JsErrorCode)
        JsSetException(
        _In_ JsValueRef exception);

    /// <summary>
    ///     Suspends script execution and terminates any running scripts in a runtime.
    /// </summary>
    /// <remarks>
    ///     <para>
    ///     Calls to a suspended runtime will fail until <c>JsEnableRuntimeExecution</c> is called.
    ///     </para>
    ///     <para>
    ///     This API does not have to be called on the thread the runtime is active on. Although the
    ///     runtime will be set into a suspended state, an executing script may not be suspended 
    ///     immediately; a running script will be terminated with an uncatchable exception as soon as 
    ///     possible.
    ///     </para>
    ///     <para>
    ///     Suspending execution in a runtime that is already suspended is a no-op.
    ///     </para>
    /// </remarks>
    /// <param name="runtime">The runtime to be suspended.</param>
    /// <returns>
    ///     The code <c>JsNoError</c> if the operation succeeded, a failure code otherwise.
    /// </returns>
    STDAPI_(JsErrorCode)
        JsDisableRuntimeExecution(
        _In_ JsRuntimeHandle runtime);

    /// <summary>
    ///     Enables script execution in a runtime.
    /// </summary>
    /// <remarks>
    ///     Enabling script execution in a runtime that already has script execution enabled is a 
    ///     no-op. 
    /// </remarks>
    /// <param name="runtime">The runtime to be enabled.</param>
    /// <returns>
    ///     The code <c>JsNoError</c> if the operation succeeded, a failure code otherwise.
    /// </returns>
    STDAPI_(JsErrorCode)
        JsEnableRuntimeExecution(
        _In_ JsRuntimeHandle runtime);

    /// <summary>
    ///     Returns a value that indicates whether script execution is disabled in the runtime.
    /// </summary>
    /// <param name="runtime">Specifies the runtime to check if execution is disabled.</param>
    /// <param name="isDisabled">If execution is disabled, <c>true</c>, <c>false</c> otherwise.</param>
    /// <returns>
    ///     The code <c>JsNoError</c> if the operation succeeded, a failure code otherwise.
    /// </returns>
    STDAPI_(JsErrorCode)
        JsIsRuntimeExecutionDisabled(
        _In_ JsRuntimeHandle runtime,
        _Out_ bool *isDisabled);

    /// <summary>
    ///     Starts profiling in the current context.
    /// </summary>
    /// <remarks>
    ///     Requires an active script context.
    /// </remarks>
    /// <param name="callback">The profiling callback to use.</param>
    /// <param name="eventMask">The profiling events to callback with.</param>
    /// <param name="context">A context to pass to the profiling callback.</param>
    /// <returns>
    ///     The code <c>JsNoError</c> if the operation succeeded, a failure code otherwise.
    /// </returns>
    STDAPI_(JsErrorCode)
        JsStartProfiling(
        _In_ IActiveScriptProfilerCallback *callback,
        _In_ PROFILER_EVENT_MASK eventMask,
        _In_ unsigned long context);

    /// <summary>
    ///     Stops profiling in the current context.
    /// </summary>
    /// <remarks>
    ///     <para>
    ///     Will not return an error if profiling has not started.
    ///     </para>
    ///     <para>
    ///     Requires an active script context.
    ///     </para>
    /// </remarks>
    /// <param name="reason">
    ///     The reason for stopping profiling to pass to the profiler callback.
    /// </param>
    /// <returns>
    ///     The code <c>JsNoError</c> if the operation succeeded, a failure code otherwise.
    /// </returns>
    STDAPI_(JsErrorCode)
        JsStopProfiling(
        _In_ HRESULT reason);

    /// <summary>
    ///     Enumerates the heap of the current context.
    /// </summary>
    /// <remarks>
    ///     <para>
    ///     While the heap is being enumerated, the current context cannot be removed, and all calls to
    ///     modify the state of the context will fail until the heap enumerator is released.
    ///     </para>
    ///     <para>
    ///     Requires an active script context.
    ///     </para>
    /// </remarks>
    /// <param name="enumerator">The heap enumerator.</param>
    /// <returns>
    ///     The code <c>JsNoError</c> if the operation succeeded, a failure code otherwise.
    /// </returns>
    STDAPI_(JsErrorCode)
        JsEnumerateHeap(
        _Out_ IActiveScriptProfilerHeapEnum **enumerator);

    /// <summary>
    ///     Returns a value that indicates whether the heap of the current context is being enumerated.
    /// </summary>
    /// <remarks>
    ///     Requires an active script context.
    /// </remarks>
    /// <param name="isEnumeratingHeap">Whether the heap is being enumerated.</param>
    /// <returns>
    ///     The code <c>JsNoError</c> if the operation succeeded, a failure code otherwise.
    /// </returns>
    STDAPI_(JsErrorCode)
        JsIsEnumeratingHeap(
        _Out_ bool *isEnumeratingHeap);


#endif // WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)
#pragma endregion

#endif // NTDDI_VERSION

#endif // _JSRT9_H_
