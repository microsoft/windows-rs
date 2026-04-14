/***
* ==++==
*
* Copyright (c) Microsoft Corporation. All rights reserved.
*
* ==--==
* =+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+
*
* ppl.h
*
* Parallel Patterns Library
*
* =-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-
****/

#pragma once

#include <concrt.h>
#include <crtdefs.h>
#include <malloc.h>

#include <algorithm>
#include <functional>
#include <iterator>
#include <memory>
#include <stdexcept>
#include <type_traits>

#include <pplwin.h>

#define _PPL_H

#pragma pack(push,_CRT_PACKING)
#pragma push_macro("new")
#undef new

// Define the level of tracing to use

#define _TRACE_LEVEL_INFORMATION 4

/// <summary>
///     The <c>Concurrency</c> namespace provides classes and functions that give you access to the Concurrency Runtime,
///     a concurrent programming framework for C++. For more information, see <see cref="Concurrency Runtime"/>.
/// </summary>
/**/
namespace Concurrency
{
namespace details
{
    _CONCRTIMP size_t __cdecl _GetCombinableSize();
} // namespace details

class structured_task_group;
class task_group;

/// <summary>
///     The <c>task_handle</c> class represents an individual parallel work item. It encapsulates the instructions and the data required
///     to execute a piece of work.
/// </summary>
/// <typeparam name="_Function">
///     The type of the function object that will be invoked to execute the work represented by the <c>task_handle</c> object.
/// </typeparam>
/// <remarks>
///     <c>task_handle</c> objects can be used in conjunction with a <c>structured_task_group</c> or a more general <c>task_group</c> object,
///     to decompose work into parallel tasks. For more information, see <see cref="Task Parallelism"/>.
///     <para>Note that the creator of a <c>task_handle</c> object is responsible for maintaining the lifetime of the created
///     <c>task_handle</c> object until it is no longer required by the Concurrency Runtime. Typically, this means that the <c>task_handle</c>
///     object must not destruct until either the <c>wait</c> or <c>run_and_wait</c> method of the <c>task_group</c> or
///     <c>structured_task_group</c> to which it is queued has been called.</para>
///     <para><c>task_handle</c> objects are typically used in conjunction with C++ lambdas. Because you do not know the true type of the lambda,
///     the <see cref="make_task Function">make_task</see> function is typically used to create a <c>task_handle</c> object.</para>
///     <para>The runtime creates a copy of the work function that you pass to a <c>task_handle</c> object. Therefore, any state changes that occur in a
///     function object that you pass to a <c>task_handle</c> object will not appear in your copy of that function object.</para>
/// </remarks>
/// <seealso cref="task_group Class"/>
/// <seealso cref="structured_task_group Class"/>
/// <seealso cref="make_task Function"/>
/// <seealso cref="task_group::run Method"/>
/// <seealso cref="task_group::wait Method"/>
/// <seealso cref="task_group::run_and_wait Method"/>
/// <seealso cref="structured_task_group::run Method"/>
/// <seealso cref="structured_task_group::wait Method"/>
/// <seealso cref="structured_task_group::run_and_wait Method"/>
/**/
template<typename _Function>
class task_handle : public ::Concurrency::details::_UnrealizedChore
{
public:
    /// <summary>
    ///     Constructs a new <c>task_handle</c> object. The work of the task is performed by invoking the function specified as
    ///     a parameter to the constructor.
    /// </summary>
    /// <param name="_Func">
    ///     The function that will be invoked to execute the work represented by the <c>task_handle</c> object. This may be a lambda functor,
    ///     a pointer to a function, or any object that supports a version of the function call operator with the signature <c>void operator()()</c>.
    /// </param>
    /// <remarks>
    ///     The runtime creates a copy of the work function that you pass to the constructor. Therefore, any state changes that occur in a function
    ///     object that you pass to a <c>task_handle</c> object will not appear in your copy of that function object.
    /// </remarks>
    /**/
    task_handle(const _Function& _Func) : _M_function(_Func)
    {
        m_pFunction = reinterpret_cast <TaskProc> (&::Concurrency::details::_UnrealizedChore::_InvokeBridge<task_handle>);
    }

    /// <summary>
    ///     Destroys the <c>task_handle</c> object.
    /// </summary>
    /**/
    ~task_handle()
    {
        //
        // We only need to perform a liveness check if the client owns the lifetime of the handle. Doing this for runtime owned handles
        // is not only unnecessary -- it is also dangerous.
        //
        if (_OwningCollection() != nullptr && !_GetRuntimeOwnsLifetime())
        {
            _CheckTaskCollection();
        }
    }

    /// <summary>
    ///     The function call operator that the runtime invokes to perform the work of the task handle.
    /// </summary>
    /**/
    void operator()() const
    {
        _M_function();
    }

private:

    friend class task_group;
    friend class structured_task_group;

    // The function object invoked to perform the body of the task.
    _Function _M_function;

    task_handle const & operator=(task_handle const&);    // no assignment operator

};

/// <summary>
///     A factory method for creating a <c>task_handle</c> object.
/// </summary>
/// <typeparam name="_Function">
///     The type of the function object that will be invoked to execute the work represented by the <c>task_handle</c> object.
/// </typeparam>
/// <param name="_Func">
///     The function that will be invoked to execute the work represented by the <c>task_handle</c> object. This may be a lambda functor,
///     a pointer to a function, or any object that supports a version of the function call operator with the signature <c>void operator()()</c>.
/// </param>
/// <returns>
///     A <c>task_handle</c> object.
/// </returns>
/// <remarks>
///     This function is useful when you need to create a <c>task_handle</c> object with a lambda expression, because it allows you to
///     create the object without knowing the true type of the lambda functor.
/// </remarks>
/// <seealso cref="task_handle Class"/>
/// <seealso cref="task_group Class"/>
/// <seealso cref="structured_task_group Class"/>
/**/
template <class _Function>
task_handle<_Function> make_task(const _Function& _Func)
{
    return task_handle<_Function>(_Func);
}

/// <summary>
///     The <c>structured_task_group</c> class represents a highly structured collection of parallel work. You can queue individual parallel tasks to
///     a <c>structured_task_group</c> using <c>task_handle</c> objects, and wait for them to complete, or cancel the task group before they have finished
///     executing, which will abort any tasks that have not begun execution.
/// </summary>
/// <remarks>
///     There are a number of severe restrictions placed on usage of a <c>structured_task_group</c> object in order to gain performance:
///     <list type="bullet">
///     <item>
///     <description>A single <c>structured_task_group</c> object cannot be used by multiple threads. All operations on a <c>structured_task_group</c> object
///     must be performed by the thread that created the object. The two exceptions to this rule are the member functions <c>cancel</c> and
///     <c>is_canceling</c>. The object may not be in the capture list of a lambda expression and be used within a task, unless the task is using one
///     of the cancellation operations.</description>
///     </item>
///     <item>
///     <description>All tasks scheduled to a <c>structured_task_group</c> object are scheduled through the use of <c>task_handle</c> objects which
///     you must explicitly manage the lifetime of.</description>
///     </item>
///     <item>
///     <description>Multiple groups may only be used in absolutely nested order. If two <c>structured_task_group</c> objects are declared, the second
///     one being declared (the inner one) must destruct before any method except <c>cancel</c> or <c>is_canceling</c> is called on the first one
///     (the outer one). This condition holds true in both the case of simply declaring multiple <c>structured_task_group</c> objects within the same
///     or functionally nested scopes as well as the case of a task that was queued to the <c>structured_task_group</c> via the <c>run</c> or
///     <c>run_and_wait</c> methods.</description>
///     </item>
///     <item>
///     <description>Unlike the general <c>task_group</c> class, all states in the <c>structured_task_group</c> class are final. After you have queued tasks to the
///     group and waited for them to complete, you may not use the same group again.</description>
///     </item>
///     </list>
///     <para>For more information, see <see cref="Task Parallelism"/>.</para>
/// </remarks>
/// <seealso cref="task_group Class"/>
/// <seealso cref="task_handle Class"/>
/**/
class structured_task_group
{
public:

    /// <summary>
    ///     Constructs a new <c>structured_task_group</c> object.
    /// </summary>
    /// <remarks>
    ///     The constructor that takes a cancellation token creates a <c>structured_task_group</c> that will be canceled when the source associated with
    ///     the token is canceled. Providing an explicit cancellation token also isolates this structured task group from participating in an implicit
    ///     cancellation from a parent group with a different token or no token.
    /// </remarks>
    /// <seealso cref="Task Parallelism"/>
    /**/
    structured_task_group()
    {
    }

    /// <summary>
    ///     Constructs a new <c>structured_task_group</c> object.
    /// </summary>
    /// <param name="_CancellationToken">
    ///     A cancellation token to associate with this structured task group. The structured task group will be canceled when the token is canceled.
    /// </param>
    /// <remarks>
    ///     The constructor that takes a cancellation token creates a <c>structured_task_group</c> that will be canceled when the source associated with
    ///     the token is canceled. Providing an explicit cancellation token also isolates this structured task group from participating in an implicit
    ///     cancellation from a parent group with a different token or no token.
    /// </remarks>
    /// <seealso cref="Task Parallelism"/>
    /**/
    structured_task_group(cancellation_token _CancellationToken) :
        _M_task_collection(_CancellationToken._GetImpl() != nullptr ? _CancellationToken._GetImpl() : ::Concurrency::details::_CancellationTokenState::_None())
    {
    }

    /// <summary>
    ///     Destroys a <c>structured_task_group</c> object. You are expected to call either the <c>wait</c> or <c>run_and_wait</c> method on the
    ///     object prior to the destructor executing, unless the destructor is executing as a result of stack unwinding due to an exception.
    /// </summary>
    /// <remarks>
    ///     If the destructor runs as the result of normal execution (for example, not stack unwinding due to an exception) and neither the <c>wait</c> nor
    ///     <c>run_and_wait</c> methods have been called, the destructor may throw a <see cref="missing_wait Class">missing_wait</see> exception.
    /// </remarks>
    /// <seealso cref="structured_task_group::wait Method"/>
    /// <seealso cref="structured_task_group::run_and_wait Method"/>
    /**/
    ~structured_task_group()
    {
    }

    /// <summary>
    ///     Schedules a task on the <c>structured_task_group</c> object. The caller manages the lifetime of the <c>task_handle</c> object passed
    ///     in the <paramref name="_Task_handle"/> parameter. The version that takes the parameter <paramref name="_Placement"/> causes the task
    ///     to be biased towards executing at the location specified by that parameter.
    /// </summary>
    /// <typeparam name="_Function">
    ///     The type of the function object that will be invoked to execute the body of the task handle.
    /// </typeparam>
    /// <param name="_Task_handle">
    ///     A handle to the work being scheduled. Note that the caller has responsibility for the lifetime of this object. The runtime will
    ///     continue to expect it to live until either the <c>wait</c> or <c>run_and_wait</c> method has been called on this
    ///     <c>structured_task_group</c> object.
    /// </param>
    /// <remarks>
    ///     The runtime creates a copy of the work function that you pass to this method. Any state changes that occur in a function object that you
    ///     pass to this method will not appear in your copy of that function object.
    ///     <para>If the <c>structured_task_group</c> destructs as the result of stack unwinding from an exception, you do not need to guarantee
    ///     that a call has been made to either the <c>wait</c> or <c>run_and_wait</c> method. In this case, the destructor will appropriately
    ///     cancel and wait for the task represented by the <paramref name="_Task_handle"/> parameter to complete.</para>
    ///     <para>Throws an <see cref="invalid_multiple_scheduling Class">invalid_multiple_scheduling</see> exception if the task handle given by
    ///     the <paramref name="_Task_handle"/> parameter has already been scheduled onto a task group object via the <c>run</c> method and there has been
    ///     no intervening call to either the <c>wait</c> or <c>run_and_wait</c> method on that task group.</para>
    /// </remarks>
    /// <seealso cref="structured_task_group::wait Method"/>
    /// <seealso cref="structured_task_group::run_and_wait Method"/>
    /// <seealso cref="Task Parallelism"/>
    /// <seealso cref="location class"/>
    /**/
    template<class _Function>
    void run(task_handle<_Function>& _Task_handle)
    {
        _Task_handle._SetRuntimeOwnsLifetime(false);
        _M_task_collection._Schedule(&_Task_handle);
    }

    /// <summary>
    ///     Schedules a task on the <c>structured_task_group</c> object. The caller manages the lifetime of the <c>task_handle</c> object passed
    ///     in the <paramref name="_Task_handle"/> parameter. The version that takes the parameter <paramref name="_Placement"/> causes the task
    ///     to be biased towards executing at the location specified by that parameter.
    /// </summary>
    /// <typeparam name="_Function">
    ///     The type of the function object that will be invoked to execute the body of the task handle.
    /// </typeparam>
    /// <param name="_Task_handle">
    ///     A handle to the work being scheduled. Note that the caller has responsibility for the lifetime of this object. The runtime will
    ///     continue to expect it to live until either the <c>wait</c> or <c>run_and_wait</c> method has been called on this
    ///     <c>structured_task_group</c> object.
    /// </param>
    /// <param name="_Placement">
    ///     A reference to the location where the task represented by the <paramref name="_Task_handle"/> parameter should execute.
    /// </param>
    /// <remarks>
    ///     The runtime creates a copy of the work function that you pass to this method. Any state changes that occur in a function object that you
    ///     pass to this method will not appear in your copy of that function object.
    ///     <para>If the <c>structured_task_group</c> destructs as the result of stack unwinding from an exception, you do not need to guarantee
    ///     that a call has been made to either the <c>wait</c> or <c>run_and_wait</c> method. In this case, the destructor will appropriately
    ///     cancel and wait for the task represented by the <paramref name="_Task_handle"/> parameter to complete.</para>
    ///     <para>Throws an <see cref="invalid_multiple_scheduling Class">invalid_multiple_scheduling</see> exception if the task handle given by
    ///     the <paramref name="_Task_handle"/> parameter has already been scheduled onto a task group object via the <c>run</c> method and there has been
    ///     no intervening call to either the <c>wait</c> or <c>run_and_wait</c> method on that task group.</para>
    /// </remarks>
    /// <seealso cref="structured_task_group::wait Method"/>
    /// <seealso cref="structured_task_group::run_and_wait Method"/>
    /// <seealso cref="Task Parallelism"/>
    /// <seealso cref="location class"/>
    /**/
    template<class _Function>
    void run(task_handle<_Function>& _Task_handle, location& _Placement)
    {
        _Task_handle._SetRuntimeOwnsLifetime(false);
        _M_task_collection._Schedule(&_Task_handle, &_Placement);
    }

    /// <summary>
    ///     Waits until all work on the <c>structured_task_group</c> has completed or is canceled.
    /// </summary>
    /// <returns>
    ///     An indication of whether the wait was satisfied or the task group was canceled, due to either an explicit cancel operation or an exception
    ///     being thrown from one of its tasks. For more information, see <see cref="task_group_status Enumeration">task_group_status</see>
    /// </returns>
    /// <remarks>
    ///     Note that one or more of the tasks scheduled to this <c>structured_task_group</c> object may execute inline on the calling context.
    ///     <para>If one or more of the tasks scheduled to this <c>structured_task_group</c> object throws an exception, the
    ///     runtime will select one such exception of its choosing and propagate it out of the call to the <c>wait</c> method.</para>
    ///     <para>After this function returns, the <c>structured_task_group</c> object is considered in a final state and should not be used. Note that
    ///     utilization after the <c>wait</c> method returns will result in undefined behavior.</para>
    ///     <para>In the non-exceptional path of execution, you have a mandate to call either this method or the <c>run_and_wait</c> method before
    ///     the destructor of the <c>structured_task_group</c> executes.</para>
    /// </remarks>
    /// <seealso cref="structured_task_group::wait Method"/>
    /// <seealso cref="structured_task_group::run_and_wait Method"/>
    /// <seealso cref="Task Parallelism"/>
    /**/
    task_group_status wait()
    {
        //
        // The underlying scheduler's definitions map exactly to the PPL's. No translation beyond the cast is necessary.
        //
        return (task_group_status)_M_task_collection._Wait();
    }

    /// <summary>
    ///     Schedules a task to be run inline on the calling context with the assistance of the <c>structured_task_group</c> object for full
    ///     cancellation support. If a <c>task_handle</c> object is passed as a parameter to <c>run_and_wait</c>, the caller is
    ///     responsible for managing the lifetime of the <c>task_handle</c> object. The function then waits until all work on the
    ///     <c>structured_task_group</c> object has either completed or been canceled.
    /// </summary>
    /// <typeparam name="_Function">
    ///     The type of the function object that will be invoked to execute the body of the task handle.
    /// </typeparam>
    /// <param name="_Task_handle">
    ///     A handle to the task which will be run inline on the calling context. Note that the caller has responsibility for the lifetime of this object.
    ///     The runtime will continue to expect it to live until the <c>run_and_wait</c> method finishes execution.
    /// </param>
    /// <returns>
    ///     An indication of whether the wait was satisfied or the task group was canceled, due to either an explicit cancel operation or an exception
    ///     being thrown from one of its tasks. For more information, see <see cref="task_group_status Enumeration">task_group_status</see>
    /// </returns>
    /// <remarks>
    ///     Note that one or more of the tasks scheduled to this <c>structured_task_group</c> object may execute inline on the calling context.
    ///     <para>If one or more of the tasks scheduled to this <c>structured_task_group</c> object throws an exception, the
    ///     runtime will select one such exception of its choosing and propagate it out of the call to the <c>run_and_wait</c> method.</para>
    ///     <para>After this function returns, the <c>structured_task_group</c> object is considered in a final state and should not be used.
    ///     Note that utilization after the <c>run_and_wait</c> method returns will result in undefined behavior.</para>
    ///     <para>In the non-exceptional path of execution, you have a mandate to call either this method or the <c>wait</c> method before
    ///     the destructor of the <c>structured_task_group</c> executes.</para>
    /// </remarks>
    /// <seealso cref="structured_task_group::run Method"/>
    /// <seealso cref="structured_task_group::wait Method"/>
    /// <seealso cref="Task Parallelism"/>
    /**/
    template<class _Function>
    task_group_status run_and_wait(task_handle<_Function>& _Task_handle)
    {
        //
        // The underlying scheduler's definitions map exactly to the PPL's. No translation beyond the cast is necessary.
        //
        return (task_group_status)_M_task_collection._RunAndWait(&_Task_handle);
    }

    /// <summary>
    ///     Schedules a task to be run inline on the calling context with the assistance of the <c>structured_task_group</c> object for full
    ///     cancellation support. If a <c>task_handle</c> object is passed as a parameter to <c>run_and_wait</c>, the caller is
    ///     responsible for managing the lifetime of the <c>task_handle</c> object. The function then waits until all work on the
    ///     <c>structured_task_group</c> object has either completed or been canceled.
    /// </summary>
    /// <typeparam name="_Function">
    ///     The type of the function object that will be invoked to execute the task.
    /// </typeparam>
    /// <param name="_Func">
    ///     A function which will be called to invoke the body of the work. This may be a lambda or other object which supports
    ///     a version of the function call operator with the signature <c>void operator()()</c>.
    /// </param>
    /// <returns>
    ///     An indication of whether the wait was satisfied or the task group was canceled, due to either an explicit cancel operation or an exception
    ///     being thrown from one of its tasks. For more information, see <see cref="task_group_status Enumeration">task_group_status</see>
    /// </returns>
    /// <remarks>
    ///     Note that one or more of the tasks scheduled to this <c>structured_task_group</c> object may execute inline on the calling context.
    ///     <para>If one or more of the tasks scheduled to this <c>structured_task_group</c> object throws an exception, the
    ///     runtime will select one such exception of its choosing and propagate it out of the call to the <c>run_and_wait</c> method.</para>
    ///     <para>After this function returns, the <c>structured_task_group</c> object is considered in a final state and should not be used.
    ///     Note that utilization after the <c>run_and_wait</c> method returns will result in undefined behavior.</para>
    ///     <para>In the non-exceptional path of execution, you have a mandate to call either this method or the <c>wait</c> method before
    ///     the destructor of the <c>structured_task_group</c> executes.</para>
    /// </remarks>
    /// <seealso cref="structured_task_group::run Method"/>
    /// <seealso cref="structured_task_group::wait Method"/>
    /// <seealso cref="Task Parallelism"/>
    /**/
    template<class _Function>
    task_group_status run_and_wait(const _Function& _Func)
    {
        //
        // The underlying scheduler's definitions map exactly to the PPL's. No translation beyond the cast is necessary.
        //
        task_handle<_Function> _Task(_Func);
        return (task_group_status)_M_task_collection._RunAndWait(&_Task);
    }

    /// <summary>
    ///     Makes a best effort attempt to cancel the sub-tree of work rooted at this task group. Every task scheduled on the task group
    ///     will get canceled transitively if possible.
    /// </summary>
    /// <remarks>
    ///     For more information, see <see cref="Cancellation in the PPL"/>.
    /// </remarks>
    /**/
    void cancel()
    {
        _M_task_collection._Cancel();
    }

    /// <summary>
    ///     Informs the caller whether or not the task group is currently in the midst of a cancellation. This
    ///     does not necessarily indicate that the <c>cancel</c> method was called on the <c>structured_task_group</c> object
    ///     (although such certainly qualifies this method to return <c>true</c>). It may be the case that the <c>structured_task_group</c> object
    ///     is executing inline and a task group further up in the work tree was canceled. In cases such as these where the runtime can determine ahead
    ///     of time that cancellation will flow through this <c>structured_task_group</c> object, <c>true</c> will be returned as well.
    /// </summary>
    /// <returns>
    ///     An indication of whether the <c>structured_task_group</c> object is in the midst of a cancellation (or is guaranteed to be shortly).
    /// </returns>
    /// <remarks>
    ///     For more information, see <see cref="Cancellation in the PPL"/>.
    /// </remarks>
    /**/
    bool is_canceling()
    {
        return _M_task_collection._IsCanceling();
    }

private:

    // Disallow passing in an r-value for a task handle argument
    template<class _Function> void run(task_handle<_Function>&& _Task_handle);

    // The underlying group of tasks as known to the runtime.
    ::Concurrency::details::_StructuredTaskCollection _M_task_collection;
};

/// <summary>
///     The <c>task_group</c> class represents a collection of parallel work which can be waited on or canceled.
/// </summary>
/// <remarks>
///     Unlike the heavily restricted <c>structured_task_group</c> class, the <c>task_group</c> class is much more general construct.
///     It does not have any of the restrictions described by <see cref="structured_task_group Class">structured_task_group</see>. <c>task_group</c>
///     objects may safely be used across threads and utilized in free-form ways. The disadvantage of the <c>task_group</c> construct is that
///     it may not perform as well as the <c>structured_task_group</c> construct for tasks which perform small amounts of work.
///     <para>For more information, see <see cref="Task Parallelism"/>.</para>
/// </remarks>
/// <seealso cref="structured_task_group Class"/>
/// <seealso cref="task_handle Class"/>
/**/
class task_group
{
public:

    /// <summary>
    ///     Constructs a new <c>task_group</c> object.
    /// </summary>
    /// <remarks>
    ///     The constructor that takes a cancellation token creates a <c>task_group</c> that will be canceled when the source associated with
    ///     the token is canceled. Providing an explicit cancellation token also isolates this task group from participating in an implicit
    ///     cancellation from a parent group with a different token or no token.
    /// </remarks>
    /// <seealso cref="Task Parallelism"/>
    /**/
    task_group()
    {
    }
    /// <summary>
    ///     Constructs a new <c>task_group</c> object.
    /// </summary>
    /// <param name="_CancellationToken">
    ///     A cancellation token to associate with this task group. The task group will be canceled when the token is canceled.
    /// </param>
    /// <remarks>
    ///     The constructor that takes a cancellation token creates a <c>task_group</c> that will be canceled when the source associated with
    ///     the token is canceled. Providing an explicit cancellation token also isolates this task group from participating in an implicit
    ///     cancellation from a parent group with a different token or no token.
    /// </remarks>
    /// <seealso cref="Task Parallelism"/>
    /**/
    task_group(cancellation_token _CancellationToken) :
        _M_task_collection(_CancellationToken._GetImpl() != nullptr ? _CancellationToken._GetImpl() : ::Concurrency::details::_CancellationTokenState::_None())
    {
    }

    /// <summary>
    ///     Destroys a <c>task_group</c> object. You are expected to call the either the <c>wait</c> or <c>run_and_wait</c> method on the object
    ///     prior to the destructor executing, unless the destructor is executing as the result of stack unwinding due to an exception.
    /// </summary>
    /// <remarks>
    ///     If the destructor runs as the result of normal execution (for example, not stack unwinding due to an exception) and neither the <c>wait</c> nor
    ///     <c>run_and_wait</c> methods have been called, the destructor may throw a <see cref="missing_wait Class">missing_wait</see> exception.
    /// </remarks>
    /// <seealso cref="task_group::wait Method"/>
    /// <seealso cref="task_group::run_and_wait Method"/>
    /**/
    ~task_group()
    {
    }

    /// <summary>
    ///     Schedules a task on the <c>task_group</c> object. If a <c>task_handle</c> object is passed as a parameter to <c>run</c>, the caller is
    ///     responsible for managing the lifetime of the <c>task_handle</c> object. The version of the method that takes a reference to a function
    ///     object as a parameter involves heap allocation inside the runtime which may be perform less well than using the version that takes a
    ///     reference to a <c>task_handle</c> object. The version which takes the parameter <paramref name="_Placement"/> causes the task to be biased
    ///     towards executing at the location specified by that parameter.
    /// </summary>
    /// <typeparam name="_Function">
    ///     The type of the function object that will be invoked to execute the body of the task handle.
    /// </typeparam>
    /// <param name="_Func">
    ///     A function which will be called to invoke the body of the task. This may be a lambda expression or other object which supports
    ///     a version of the function call operator with the signature <c>void operator()()</c>.
    /// </param>
    /// <remarks>
    ///     The runtime schedules the provided work function to run at a later time, which can be after the calling function returns.
    ///     This method uses a <see cref="task_handle Class">task_handle</see> object to hold a copy of the provided work function.
    ///     Therefore, any state changes that occur in a function object that you pass to this method will not appear in your copy of that function object.
    ///     In addition, make sure that the lifetime of any objects that you pass by pointer or by reference to the work function remain valid until
    ///     the work function returns.
    ///     <para>If the <c>task_group</c> destructs as the result of stack unwinding from an exception, you do not need to guarantee
    ///     that a call has been made to either the <c>wait</c> or <c>run_and_wait</c> method. In this case, the destructor will appropriately
    ///     cancel and wait for the task represented by the <paramref name="_Task_handle"/> parameter to complete.</para>
    ///     <para>The method throws an <see cref="invalid_multiple_scheduling Class">invalid_multiple_scheduling</see> exception if the task
    ///     handle given by the <paramref name="_Task_handle"/> parameter has already been scheduled onto a task group object via the <c>run</c>
    ///     method and there has been no intervening call to either the <c>wait</c> or <c>run_and_wait</c> method on that task group.</para>
    /// </remarks>
    /// <seealso cref="task_group::wait Method"/>
    /// <seealso cref="task_group::run_and_wait Method"/>
    /// <seealso cref="Task Parallelism"/>
    /// <seealso cref="location class"/>
    /**/
    template<typename _Function>
    void run(const _Function& _Func)
    {
        _M_task_collection._Schedule(::Concurrency::details::_UnrealizedChore::_InternalAlloc<task_handle<_Function>, _Function>(_Func));
    }

    /// <summary>
    ///     Schedules a task on the <c>task_group</c> object. If a <c>task_handle</c> object is passed as a parameter to <c>run</c>, the caller is
    ///     responsible for managing the lifetime of the <c>task_handle</c> object. The version of the method that takes a reference to a function
    ///     object as a parameter involves heap allocation inside the runtime which may be perform less well than using the version that takes a
    ///     reference to a <c>task_handle</c> object. The version which takes the parameter <paramref name="_Placement"/> causes the task to be biased
    ///     towards executing at the location specified by that parameter.
    /// </summary>
    /// <typeparam name="_Function">
    ///     The type of the function object that will be invoked to execute the body of the task handle.
    /// </typeparam>
    /// <param name="_Func">
    ///     A function which will be called to invoke the body of the task. This may be a lambda expression or other object which supports
    ///     a version of the function call operator with the signature <c>void operator()()</c>.
    /// </param>
    /// <param name="_Placement">
    ///     A reference to the location where the task represented by the <paramref name="_Func"/> parameter should execute.
    /// </param>
    /// <remarks>
    ///     The runtime schedules the provided work function to run at a later time, which can be after the calling function returns.
    ///     This method uses a <see cref="task_handle Class">task_handle</see> object to hold a copy of the provided work function.
    ///     Therefore, any state changes that occur in a function object that you pass to this method will not appear in your copy of that function object.
    ///     In addition, make sure that the lifetime of any objects that you pass by pointer or by reference to the work function remain valid until
    ///     the work function returns.
    ///     <para>If the <c>task_group</c> destructs as the result of stack unwinding from an exception, you do not need to guarantee
    ///     that a call has been made to either the <c>wait</c> or <c>run_and_wait</c> method. In this case, the destructor will appropriately
    ///     cancel and wait for the task represented by the <paramref name="_Task_handle"/> parameter to complete.</para>
    ///     <para>The method throws an <see cref="invalid_multiple_scheduling Class">invalid_multiple_scheduling</see> exception if the task
    ///     handle given by the <paramref name="_Task_handle"/> parameter has already been scheduled onto a task group object via the <c>run</c>
    ///     method and there has been no intervening call to either the <c>wait</c> or <c>run_and_wait</c> method on that task group.</para>
    /// </remarks>
    /// <seealso cref="task_group::wait Method"/>
    /// <seealso cref="task_group::run_and_wait Method"/>
    /// <seealso cref="Task Parallelism"/>
    /// <seealso cref="location class"/>
    /**/
    template<typename _Function>
    void run(const _Function& _Func, location& _Placement)
    {
        _M_task_collection._Schedule(::Concurrency::details::_UnrealizedChore::_InternalAlloc<task_handle<_Function>, _Function>(_Func), &_Placement);
    }

    /// <summary>
    ///     Schedules a task on the <c>task_group</c> object. If a <c>task_handle</c> object is passed as a parameter to <c>run</c>, the caller is
    ///     responsible for managing the lifetime of the <c>task_handle</c> object. The version of the method that takes a reference to a function
    ///     object as a parameter involves heap allocation inside the runtime which may be perform less well than using the version that takes a
    ///     reference to a <c>task_handle</c> object. The version which takes the parameter <paramref name="_Placement"/> causes the task to be biased
    ///     towards executing at the location specified by that parameter.
    /// </summary>
    /// <typeparam name="_Function">
    ///     The type of the function object that will be invoked to execute the body of the task handle.
    /// </typeparam>
    /// <param name="_Task_handle">
    ///     A handle to the work being scheduled. Note that the caller has responsibility for the lifetime of this object. The runtime will
    ///     continue to expect it to live until either the <c>wait</c> or <c>run_and_wait</c> method has been called on this
    ///     <c>task_group</c> object.
    /// </param>
    /// <remarks>
    ///     The runtime schedules the provided work function to run at a later time, which can be after the calling function returns.
    ///     This method uses a <see cref="task_handle Class">task_handle</see> object to hold a copy of the provided work function.
    ///     Therefore, any state changes that occur in a function object that you pass to this method will not appear in your copy of that function object.
    ///     In addition, make sure that the lifetime of any objects that you pass by pointer or by reference to the work function remain valid until
    ///     the work function returns.
    ///     <para>If the <c>task_group</c> destructs as the result of stack unwinding from an exception, you do not need to guarantee
    ///     that a call has been made to either the <c>wait</c> or <c>run_and_wait</c> method. In this case, the destructor will appropriately
    ///     cancel and wait for the task represented by the <paramref name="_Task_handle"/> parameter to complete.</para>
    ///     <para>The method throws an <see cref="invalid_multiple_scheduling Class">invalid_multiple_scheduling</see> exception if the task
    ///     handle given by the <paramref name="_Task_handle"/> parameter has already been scheduled onto a task group object via the <c>run</c>
    ///     method and there has been no intervening call to either the <c>wait</c> or <c>run_and_wait</c> method on that task group.</para>
    /// </remarks>
    /// <seealso cref="task_group::wait Method"/>
    /// <seealso cref="task_group::run_and_wait Method"/>
    /// <seealso cref="Task Parallelism"/>
    /// <seealso cref="location class"/>
    /**/
    template<typename _Function>
    void run(task_handle<_Function>& _Task_handle)
    {
        _Task_handle._SetRuntimeOwnsLifetime(false);
        _M_task_collection._Schedule(&_Task_handle);
    }

    /// <summary>
    ///     Schedules a task on the <c>task_group</c> object. If a <c>task_handle</c> object is passed as a parameter to <c>run</c>, the caller is
    ///     responsible for managing the lifetime of the <c>task_handle</c> object. The version of the method that takes a reference to a function
    ///     object as a parameter involves heap allocation inside the runtime which may be perform less well than using the version that takes a
    ///     reference to a <c>task_handle</c> object. The version which takes the parameter <paramref name="_Placement"/> causes the task to be biased
    ///     towards executing at the location specified by that parameter.
    /// </summary>
    /// <typeparam name="_Function">
    ///     The type of the function object that will be invoked to execute the body of the task handle.
    /// </typeparam>
    /// <param name="_Task_handle">
    ///     A handle to the work being scheduled. Note that the caller has responsibility for the lifetime of this object. The runtime will
    ///     continue to expect it to live until either the <c>wait</c> or <c>run_and_wait</c> method has been called on this
    ///     <c>task_group</c> object.
    /// </param>
    /// <param name="_Placement">
    ///     A reference to the location where the task represented by the <paramref name="_Func"/> parameter should execute.
    /// </param>
    /// <remarks>
    ///     The runtime schedules the provided work function to run at a later time, which can be after the calling function returns.
    ///     This method uses a <see cref="task_handle Class">task_handle</see> object to hold a copy of the provided work function.
    ///     Therefore, any state changes that occur in a function object that you pass to this method will not appear in your copy of that function object.
    ///     In addition, make sure that the lifetime of any objects that you pass by pointer or by reference to the work function remain valid until
    ///     the work function returns.
    ///     <para>If the <c>task_group</c> destructs as the result of stack unwinding from an exception, you do not need to guarantee
    ///     that a call has been made to either the <c>wait</c> or <c>run_and_wait</c> method. In this case, the destructor will appropriately
    ///     cancel and wait for the task represented by the <paramref name="_Task_handle"/> parameter to complete.</para>
    ///     <para>The method throws an <see cref="invalid_multiple_scheduling Class">invalid_multiple_scheduling</see> exception if the task
    ///     handle given by the <paramref name="_Task_handle"/> parameter has already been scheduled onto a task group object via the <c>run</c>
    ///     method and there has been no intervening call to either the <c>wait</c> or <c>run_and_wait</c> method on that task group.</para>
    /// </remarks>
    /// <seealso cref="task_group::wait Method"/>
    /// <seealso cref="task_group::run_and_wait Method"/>
    /// <seealso cref="Task Parallelism"/>
    /// <seealso cref="location class"/>
    /**/
    template<typename _Function>
    void run(task_handle<_Function>& _Task_handle, location& _Placement)
    {
        _Task_handle._SetRuntimeOwnsLifetime(false);
        _M_task_collection._Schedule(&_Task_handle, &_Placement);
    }

    /// <summary>
    ///     Waits until all work on the <c>task_group</c> object has either completed or been canceled.
    /// </summary>
    /// <returns>
    ///     An indication of whether the wait was satisfied or the task group was canceled, due to either an explicit cancel operation or an exception
    ///     being thrown from one of its tasks. For more information, see <see cref="task_group_status Enumeration">task_group_status</see>.
    /// </returns>
    /// <remarks>
    ///     Note that one or more of the tasks scheduled to this <c>task_group</c> object may execute inline on the calling context.
    ///     <para>If one or more of the tasks scheduled to this <c>task_group</c> object throws an exception, the
    ///     runtime will select one such exception of its choosing and propagate it out of the call to the <c>wait</c> method.</para>
    ///     <para>Calling <c>wait</c> on a <c>task_group</c> object resets it to a clean state where it can be reused. This includes the case
    ///     where the <c>task_group</c> object was canceled.</para>
    ///     <para>In the non-exceptional path of execution, you have a mandate to call either this method or the <c>run_and_wait</c> method before
    ///     the destructor of the <c>task_group</c> executes.</para>
    /// </remarks>
    /**/
    task_group_status wait()
    {
        //
        // The underlying scheduler's definitions map exactly to the PPL's. No translation beyond the cast is necessary.
        //
        return static_cast<task_group_status>(_M_task_collection._Wait());
    }

    /// <summary>
    ///     Schedules a task to be run inline on the calling context with the assistance of the <c>task_group</c> object for full cancellation support.
    ///     The function then waits until all work on the <c>task_group</c> object has either completed or been canceled. If a <c>task_handle</c> object
    ///     is passed as a parameter to <c>run_and_wait</c>, the caller is responsible for managing the lifetime of the <c>task_handle</c> object.
    /// </summary>
    /// <typeparam name="_Function">
    ///     The type of the function object that will be invoked to execute the body of the task handle.
    /// </typeparam>
    /// <param name="_Task_handle">
    ///     A handle to the task which will be run inline on the calling context. Note that the caller has responsibility for the lifetime of this object.
    ///     The runtime will continue to expect it to live until the <c>run_and_wait</c> method finishes execution.
    /// </param>
    /// <returns>
    ///     An indication of whether the wait was satisfied or the task group was canceled, due to either an explicit cancel operation or an exception
    ///     being thrown from one of its tasks. For more information, see <see cref="task_group_status Enumeration">task_group_status</see>.
    /// </returns>
    /// <remarks>
    ///     Note that one or more of the tasks scheduled to this <c>task_group</c> object may execute inline on the calling context.
    ///     <para>If one or more of the tasks scheduled to this <c>task_group</c> object throws an exception, the
    ///     runtime will select one such exception of its choosing and propagate it out of the call to the <c>run_and_wait</c> method.</para>
    ///     <para>Upon return from the <c>run_and_wait</c> method on a <c>task_group</c> object, the runtime resets the object to a clean state where it can be
    ///     reused. This includes the case where the <c>task_group</c> object was canceled.</para>
    ///     <para>In the non-exceptional path of execution, you have a mandate to call either this method or the <c>wait</c> method before
    ///     the destructor of the <c>task_group</c> executes.</para>
    /// </remarks>
    /// <seealso cref="task_group::run Method"/>
    /// <seealso cref="task_group::wait Method"/>
    /// <seealso cref="Task Parallelism"/>
    /**/
    template<class _Function>
    task_group_status run_and_wait(task_handle<_Function>& _Task_handle)
    {
        //
        // The underlying scheduler's definitions map exactly to the PPL's. No translation beyond the cast is necessary.
        //
        _Task_handle._SetRuntimeOwnsLifetime(false);
        return (task_group_status)_M_task_collection._RunAndWait(&_Task_handle);
    }

    /// <summary>
    ///     Schedules a task to be run inline on the calling context with the assistance of the <c>task_group</c> object for full cancellation support.
    ///     The function then waits until all work on the <c>task_group</c> object has either completed or been canceled. If a <c>task_handle</c> object
    ///     is passed as a parameter to <c>run_and_wait</c>, the caller is responsible for managing the lifetime of the <c>task_handle</c> object.
    /// </summary>
    /// <typeparam name="_Function">
    ///     The type of the function object that will be invoked to execute the body of the task.
    /// </typeparam>
    /// <param name="_Func">
    ///     A function which will be called to invoke the body of the work. This may be a lambda expression or other object which supports
    ///     a version of the function call operator with the signature <c>void operator()()</c>.
    /// </param>
    /// <returns>
    ///     An indication of whether the wait was satisfied or the task group was canceled, due to either an explicit cancel operation or an exception
    ///     being thrown from one of its tasks. For more information, see <see cref="task_group_status Enumeration">task_group_status</see>.
    /// </returns>
    /// <remarks>
    ///     Note that one or more of the tasks scheduled to this <c>task_group</c> object may execute inline on the calling context.
    ///     <para>If one or more of the tasks scheduled to this <c>task_group</c> object throws an exception, the
    ///     runtime will select one such exception of its choosing and propagate it out of the call to the <c>run_and_wait</c> method.</para>
    ///     <para>Upon return from the <c>run_and_wait</c> method on a <c>task_group</c> object, the runtime resets the object to a clean state where it can be
    ///     reused. This includes the case where the <c>task_group</c> object was canceled.</para>
    ///     <para>In the non-exceptional path of execution, you have a mandate to call either this method or the <c>wait</c> method before
    ///     the destructor of the <c>task_group</c> executes.</para>
    /// </remarks>
    /// <seealso cref="task_group::run Method"/>
    /// <seealso cref="task_group::wait Method"/>
    /// <seealso cref="Task Parallelism"/>
    /**/
    template<class _Function>
    task_group_status run_and_wait(const _Function& _Func)
    {
        //
        // The underlying scheduler's definitions map exactly to the PPL's. No translation beyond the cast is necessary.
        //
        return (task_group_status)_M_task_collection._RunAndWait(::Concurrency::details::_UnrealizedChore::_InternalAlloc<task_handle<_Function>, _Function>(_Func));
    }

    /// <summary>
    ///     Makes a best effort attempt to cancel the sub-tree of work rooted at this task group. Every task scheduled on the task group
    ///     will get canceled transitively if possible.
    /// </summary>
    /// <remarks>
    ///     For more information, see <see cref="Cancellation in the PPL"/>.
    /// </remarks>
    /**/
    void cancel()
    {
        _M_task_collection._Cancel();
    }

    /// <summary>
    ///     Informs the caller whether or not the task group is currently in the midst of a cancellation. This
    ///     does not necessarily indicate that the <c>cancel</c> method was called on the <c>task_group</c> object
    ///     (although such certainly qualifies this method to return <c>true</c>). It may be the case that the <c>task_group</c> object
    ///     is executing inline and a task group further up in the work tree was canceled. In cases such as these where the runtime can determine ahead
    ///     of time that cancellation will flow through this <c>task_group</c> object, <c>true</c> will be returned as well.
    /// </summary>
    /// <returns>
    ///     An indication of whether the <c>task_group</c> object is in the midst of a cancellation (or is guaranteed to be shortly).
    /// </returns>
    /// <remarks>
    ///     For more information, see <see cref="Cancellation in the PPL"/>.
    /// </remarks>
    /**/
    bool is_canceling()
    {
        return _M_task_collection._IsCanceling();
    }

private:

    // Disallow passing in an r-value for a task handle argument
    template<class _Function> void run(task_handle<_Function>&& _Task_handle);

    // The underlying group of tasks as known to the runtime.
    ::Concurrency::details::_TaskCollection _M_task_collection;
};

/// <summary>
///     Executes a function object immediately and synchronously in the context of a given cancellation token.
/// </summary>
/// <typeparam name="_Function">
///     The type of the function object that will be invoked.
/// </typeparam>
/// <param name="_Func">
///     The function object which will be executed. This object must support the function call operator with a signature of void().
/// </param>
/// <param name="_Ct">
///     The cancellation token which will control implicit cancellation of the function object. Use <c>cancellation_token::none()</c> if you want the
///     function execute without any possibility of implicit cancellation from a parent task group being canceled.
/// </param>
/// <remarks>
///     Any interruption points in the function object will be triggered when the <c>cancellation_token</c> is canceled.
///     The explicit token <paramref name="_Ct"/> will isolate this <paramref name="_Func"/> from parent cancellation if the parent has a
///     different token or no token.
/// </remarks>
/**/
template<typename _Function>
void run_with_cancellation_token(const _Function& _Func, cancellation_token _Ct)
{
    structured_task_group _Stg(_Ct);
    _Stg.run_and_wait(_Func);
}

/// <summary>
///     Creates an interruption point for cancellation. If a cancellation is in progress in the context where this function is called, this will throw an internal
///     exception that aborts the execution of the currently executing parallel work. If cancellation is not in progress, the function does nothing.
/// </summary>
/// <remarks>
///     You should not catch the internal cancellation exception thrown by the <c>interruption_point()</c> function. The exception will be caught and handled by
///     the runtime, and catching it may cause your program to behave abnormally.
/// </remarks>
/**/
inline void interruption_point()
{
    structured_task_group _Stg;
    _Stg.wait();
}

/// <summary>
///     Returns an indication of whether the task group which is currently executing inline on the current context
///     is in the midst of an active cancellation (or will be shortly). Note that if there is no task group currently
///     executing inline on the current context, <c>false</c> will be returned.
/// </summary>
/// <returns>
///     <c>true</c> if the task group which is currently executing is canceling, <c>false</c> otherwise.
/// </returns>
/// <remarks>
///     For more information, see <see cref="Cancellation in the PPL"/>.
/// </remarks>
/// <seealso cref="task_group Class"/>
/// <seealso cref="structured_task_group Class"/>
/**/
_CONCRTIMP bool __cdecl is_current_task_group_canceling();

// Parallel Algorithms and Patterns

// Helper function that implements parallel_invoke with two functions
// Used by parallel_for and parallel_for_each implementations

template <typename _Function1, typename _Function2>
void _Parallel_invoke_impl(const _Function1& _Func1, const _Function2& _Func2)
{
    structured_task_group _Task_group;

    task_handle<_Function1> _Task_handle1(_Func1);
    _Task_group.run(_Task_handle1);

    // We inline the last item to prevent the unnecessary push/pop on the work queue.
    task_handle<_Function2> _Task_handle2(_Func2);
    _Task_group.run_and_wait(_Task_handle2);
}

/// <summary>
///     Executes the function objects supplied as parameters in parallel, and blocks until they have finished executing. Each function object
///     could be a lambda expression, a pointer to function, or any object that supports the function call operator with the signature
///     <c>void operator()()</c>.
/// </summary>
/// <typeparam name="_Function1">
///     The type of the first function object to be executed in parallel.
/// </typeparam>
/// <typeparam name="_Function2">
///     The type of the second function object to be executed in parallel.
/// </typeparam>
/// <param name="_Func1">
///     The first function object to be executed in parallel.
/// </param>
/// <param name="_Func2">
///     The second function object to be executed in parallel.
/// </param>
/// <remarks>
///     Note that one or more of the function objects supplied as parameters may execute inline on the calling context.
///     <para>If one or more of the function objects passed as parameters to this function throws an exception, the
///     runtime will select one such exception of its choosing and propagate it out of the call to <c>parallel_invoke</c>.</para>
///     <para>For more information, see <see cref="Parallel Algorithms"/>.</para>
/// </remarks>
/**/
template <typename _Function1, typename _Function2>
void parallel_invoke(const _Function1& _Func1, const _Function2& _Func2)
{
    _Trace_ppl_function(PPLParallelInvokeEventGuid, _TRACE_LEVEL_INFORMATION, CONCRT_EVENT_START);

    _Parallel_invoke_impl(_Func1, _Func2);

    _Trace_ppl_function(PPLParallelInvokeEventGuid, _TRACE_LEVEL_INFORMATION, CONCRT_EVENT_END);
}

/// <summary>
///     Executes the function objects supplied as parameters in parallel, and blocks until they have finished executing. Each function object
///     could be a lambda expression, a pointer to function, or any object that supports the function call operator with the signature
///     <c>void operator()()</c>.
/// </summary>
/// <typeparam name="_Function1">
///     The type of the first function object to be executed in parallel.
/// </typeparam>
/// <typeparam name="_Function2">
///     The type of the second function object to be executed in parallel.
/// </typeparam>
/// <typeparam name="_Function3">
///     The type of the third function object to be executed in parallel.
/// </typeparam>
/// <param name="_Func1">
///     The first function object to be executed in parallel.
/// </param>
/// <param name="_Func2">
///     The second function object to be executed in parallel.
/// </param>
/// <param name="_Func3">
///     The third function object to be executed in parallel.
/// </param>
/// <remarks>
///     Note that one or more of the function objects supplied as parameters may execute inline on the calling context.
///     <para>If one or more of the function objects passed as parameters to this function throws an exception, the
///     runtime will select one such exception of its choosing and propagate it out of the call to <c>parallel_invoke</c>.</para>
///     <para>For more information, see <see cref="Parallel Algorithms"/>.</para>
/// </remarks>
/**/
template <typename _Function1, typename _Function2, typename _Function3>
void parallel_invoke(const _Function1& _Func1, const _Function2& _Func2, const _Function3& _Func3)
{
    _Trace_ppl_function(PPLParallelInvokeEventGuid, _TRACE_LEVEL_INFORMATION, CONCRT_EVENT_START);

    structured_task_group _Task_group;

    task_handle<_Function1> _Task_handle1(_Func1);
    _Task_group.run(_Task_handle1);

    task_handle<_Function2> _Task_handle2(_Func2);
    _Task_group.run(_Task_handle2);

    task_handle<_Function3> _Task_handle3(_Func3);
    _Task_group.run_and_wait(_Task_handle3);

    _Trace_ppl_function(PPLParallelInvokeEventGuid, _TRACE_LEVEL_INFORMATION, CONCRT_EVENT_END);
}

/// <summary>
///     Executes the function objects supplied as parameters in parallel, and blocks until they have finished executing. Each function object
///     could be a lambda expression, a pointer to function, or any object that supports the function call operator with the signature
///     <c>void operator()()</c>.
/// </summary>
/// <typeparam name="_Function1">
///     The type of the first function object to be executed in parallel.
/// </typeparam>
/// <typeparam name="_Function2">
///     The type of the second function object to be executed in parallel.
/// </typeparam>
/// <typeparam name="_Function3">
///     The type of the third function object to be executed in parallel.
/// </typeparam>
/// <typeparam name="_Function4">
///     The type of the fourth function object to be executed in parallel.
/// </typeparam>
/// <param name="_Func1">
///     The first function object to be executed in parallel.
/// </param>
/// <param name="_Func2">
///     The second function object to be executed in parallel.
/// </param>
/// <param name="_Func3">
///     The third function object to be executed in parallel.
/// </param>
/// <param name="_Func4">
///     The fourth function object to be executed in parallel.
/// </param>
/// <remarks>
///     Note that one or more of the function objects supplied as parameters may execute inline on the calling context.
///     <para>If one or more of the function objects passed as parameters to this function throws an exception, the
///     runtime will select one such exception of its choosing and propagate it out of the call to <c>parallel_invoke</c>.</para>
///     <para>For more information, see <see cref="Parallel Algorithms"/>.</para>
/// </remarks>
/**/
template <typename _Function1, typename _Function2, typename _Function3, typename _Function4>
void parallel_invoke(const _Function1& _Func1, const _Function2& _Func2, const _Function3& _Func3, const _Function4& _Func4)
{
    _Trace_ppl_function(PPLParallelInvokeEventGuid, _TRACE_LEVEL_INFORMATION, CONCRT_EVENT_START);

    structured_task_group _Task_group;

    task_handle<_Function1> _Task_handle1(_Func1);
    _Task_group.run(_Task_handle1);

    task_handle<_Function2> _Task_handle2(_Func2);
    _Task_group.run(_Task_handle2);

    task_handle<_Function3> _Task_handle3(_Func3);
    _Task_group.run(_Task_handle3);

    task_handle<_Function4> _Task_handle4(_Func4);
    _Task_group.run_and_wait(_Task_handle4);

    _Trace_ppl_function(PPLParallelInvokeEventGuid, _TRACE_LEVEL_INFORMATION, CONCRT_EVENT_END);
}

/// <summary>
///     Executes the function objects supplied as parameters in parallel, and blocks until they have finished executing. Each function object
///     could be a lambda expression, a pointer to function, or any object that supports the function call operator with the signature
///     <c>void operator()()</c>.
/// </summary>
/// <typeparam name="_Function1">
///     The type of the first function object to be executed in parallel.
/// </typeparam>
/// <typeparam name="_Function2">
///     The type of the second function object to be executed in parallel.
/// </typeparam>
/// <typeparam name="_Function3">
///     The type of the third function object to be executed in parallel.
/// </typeparam>
/// <typeparam name="_Function4">
///     The type of the fourth function object to be executed in parallel.
/// </typeparam>
/// <typeparam name="_Function5">
///     The type of the fifth function object to be executed in parallel.
/// </typeparam>
/// <param name="_Func1">
///     The first function object to be executed in parallel.
/// </param>
/// <param name="_Func2">
///     The second function object to be executed in parallel.
/// </param>
/// <param name="_Func3">
///     The third function object to be executed in parallel.
/// </param>
/// <param name="_Func4">
///     The fourth function object to be executed in parallel.
/// </param>
/// <param name="_Func5">
///     The fifth function object to be executed in parallel.
/// </param>
/// <remarks>
///     Note that one or more of the function objects supplied as parameters may execute inline on the calling context.
///     <para>If one or more of the function objects passed as parameters to this function throws an exception, the
///     runtime will select one such exception of its choosing and propagate it out of the call to <c>parallel_invoke</c>.</para>
///     <para>For more information, see <see cref="Parallel Algorithms"/>.</para>
/// </remarks>
/**/
template <typename _Function1, typename _Function2, typename _Function3, typename _Function4, typename _Function5>
void parallel_invoke(const _Function1& _Func1, const _Function2& _Func2, const _Function3& _Func3, const _Function4& _Func4, const _Function5& _Func5)
{
    _Trace_ppl_function(PPLParallelInvokeEventGuid, _TRACE_LEVEL_INFORMATION, CONCRT_EVENT_START);

    structured_task_group _Task_group;

    task_handle<_Function1> _Task_handle1(_Func1);
    _Task_group.run(_Task_handle1);

    task_handle<_Function2> _Task_handle2(_Func2);
    _Task_group.run(_Task_handle2);

    task_handle<_Function3> _Task_handle3(_Func3);
    _Task_group.run(_Task_handle3);

    task_handle<_Function4> _Task_handle4(_Func4);
    _Task_group.run(_Task_handle4);

    task_handle<_Function5> _Task_handle5(_Func5);
    _Task_group.run_and_wait(_Task_handle5);

    _Trace_ppl_function(PPLParallelInvokeEventGuid, _TRACE_LEVEL_INFORMATION, CONCRT_EVENT_END);
}

/// <summary>
///     Executes the function objects supplied as parameters in parallel, and blocks until they have finished executing. Each function object
///     could be a lambda expression, a pointer to function, or any object that supports the function call operator with the signature
///     <c>void operator()()</c>.
/// </summary>
/// <typeparam name="_Function1">
///     The type of the first function object to be executed in parallel.
/// </typeparam>
/// <typeparam name="_Function2">
///     The type of the second function object to be executed in parallel.
/// </typeparam>
/// <typeparam name="_Function3">
///     The type of the third function object to be executed in parallel.
/// </typeparam>
/// <typeparam name="_Function4">
///     The type of the fourth function object to be executed in parallel.
/// </typeparam>
/// <typeparam name="_Function5">
///     The type of the fifth function object to be executed in parallel.
/// </typeparam>
/// <typeparam name="_Function6">
///     The type of the sixth function object to be executed in parallel.
/// </typeparam>
/// <param name="_Func1">
///     The first function object to be executed in parallel.
/// </param>
/// <param name="_Func2">
///     The second function object to be executed in parallel.
/// </param>
/// <param name="_Func3">
///     The third function object to be executed in parallel.
/// </param>
/// <param name="_Func4">
///     The fourth function object to be executed in parallel.
/// </param>
/// <param name="_Func5">
///     The fifth function object to be executed in parallel.
/// </param>
/// <param name="_Func6">
///     The sixth function object to be executed in parallel.
/// </param>
/// <remarks>
///     Note that one or more of the function objects supplied as parameters may execute inline on the calling context.
///     <para>If one or more of the function objects passed as parameters to this function throws an exception, the
///     runtime will select one such exception of its choosing and propagate it out of the call to <c>parallel_invoke</c>.</para>
///     <para>For more information, see <see cref="Parallel Algorithms"/>.</para>
/// </remarks>
/**/
template <typename _Function1, typename _Function2, typename _Function3, typename _Function4, typename _Function5,
    typename _Function6>
void parallel_invoke(const _Function1& _Func1, const _Function2& _Func2, const _Function3& _Func3, const _Function4& _Func4, const _Function5& _Func5,
    const _Function6& _Func6)
{
    _Trace_ppl_function(PPLParallelInvokeEventGuid, _TRACE_LEVEL_INFORMATION, CONCRT_EVENT_START);

    structured_task_group _Task_group;

    task_handle<_Function1> _Task_handle1(_Func1);
    _Task_group.run(_Task_handle1);

    task_handle<_Function2> _Task_handle2(_Func2);
    _Task_group.run(_Task_handle2);

    task_handle<_Function3> _Task_handle3(_Func3);
    _Task_group.run(_Task_handle3);

    task_handle<_Function4> _Task_handle4(_Func4);
    _Task_group.run(_Task_handle4);

    task_handle<_Function5> _Task_handle5(_Func5);
    _Task_group.run(_Task_handle5);

    task_handle<_Function6> _Task_handle6(_Func6);
    _Task_group.run_and_wait(_Task_handle6);

    _Trace_ppl_function(PPLParallelInvokeEventGuid, _TRACE_LEVEL_INFORMATION, CONCRT_EVENT_END);
}

/// <summary>
///     Executes the function objects supplied as parameters in parallel, and blocks until they have finished executing. Each function object
///     could be a lambda expression, a pointer to function, or any object that supports the function call operator with the signature
///     <c>void operator()()</c>.
/// </summary>
/// <typeparam name="_Function1">
///     The type of the first function object to be executed in parallel.
/// </typeparam>
/// <typeparam name="_Function2">
///     The type of the second function object to be executed in parallel.
/// </typeparam>
/// <typeparam name="_Function3">
///     The type of the third function object to be executed in parallel.
/// </typeparam>
/// <typeparam name="_Function4">
///     The type of the fourth function object to be executed in parallel.
/// </typeparam>
/// <typeparam name="_Function5">
///     The type of the fifth function object to be executed in parallel.
/// </typeparam>
/// <typeparam name="_Function6">
///     The type of the sixth function object to be executed in parallel.
/// </typeparam>
/// <typeparam name="_Function7">
///     The type of the seventh function object to be executed in parallel.
/// </typeparam>
/// <param name="_Func1">
///     The first function object to be executed in parallel.
/// </param>
/// <param name="_Func2">
///     The second function object to be executed in parallel.
/// </param>
/// <param name="_Func3">
///     The third function object to be executed in parallel.
/// </param>
/// <param name="_Func4">
///     The fourth function object to be executed in parallel.
/// </param>
/// <param name="_Func5">
///     The fifth function object to be executed in parallel.
/// </param>
/// <param name="_Func6">
///     The sixth function object to be executed in parallel.
/// </param>
/// <param name="_Func7">
///     The seventh function object to be executed in parallel.
/// </param>
/// <remarks>
///     Note that one or more of the function objects supplied as parameters may execute inline on the calling context.
///     <para>If one or more of the function objects passed as parameters to this function throws an exception, the
///     runtime will select one such exception of its choosing and propagate it out of the call to <c>parallel_invoke</c>.</para>
///     <para>For more information, see <see cref="Parallel Algorithms"/>.</para>
/// </remarks>
/**/
template <typename _Function1, typename _Function2, typename _Function3, typename _Function4, typename _Function5,
    typename _Function6, typename _Function7>
void parallel_invoke(const _Function1& _Func1, const _Function2& _Func2, const _Function3& _Func3, const _Function4& _Func4, const _Function5& _Func5,
    const _Function6& _Func6, const _Function7& _Func7)
{
    _Trace_ppl_function(PPLParallelInvokeEventGuid, _TRACE_LEVEL_INFORMATION, CONCRT_EVENT_START);

    structured_task_group _Task_group;

    task_handle<_Function1> _Task_handle1(_Func1);
    _Task_group.run(_Task_handle1);

    task_handle<_Function2> _Task_handle2(_Func2);
    _Task_group.run(_Task_handle2);

    task_handle<_Function3> _Task_handle3(_Func3);
    _Task_group.run(_Task_handle3);

    task_handle<_Function4> _Task_handle4(_Func4);
    _Task_group.run(_Task_handle4);

    task_handle<_Function5> _Task_handle5(_Func5);
    _Task_group.run(_Task_handle5);

    task_handle<_Function6> _Task_handle6(_Func6);
    _Task_group.run(_Task_handle6);

    task_handle<_Function7> _Task_handle7(_Func7);
    _Task_group.run_and_wait(_Task_handle7);

    _Trace_ppl_function(PPLParallelInvokeEventGuid, _TRACE_LEVEL_INFORMATION, CONCRT_EVENT_END);
}

/// <summary>
///     Executes the function objects supplied as parameters in parallel, and blocks until they have finished executing. Each function object
///     could be a lambda expression, a pointer to function, or any object that supports the function call operator with the signature
///     <c>void operator()()</c>.
/// </summary>
/// <typeparam name="_Function1">
///     The type of the first function object to be executed in parallel.
/// </typeparam>
/// <typeparam name="_Function2">
///     The type of the second function object to be executed in parallel.
/// </typeparam>
/// <typeparam name="_Function3">
///     The type of the third function object to be executed in parallel.
/// </typeparam>
/// <typeparam name="_Function4">
///     The type of the fourth function object to be executed in parallel.
/// </typeparam>
/// <typeparam name="_Function5">
///     The type of the fifth function object to be executed in parallel.
/// </typeparam>
/// <typeparam name="_Function6">
///     The type of the sixth function object to be executed in parallel.
/// </typeparam>
/// <typeparam name="_Function7">
///     The type of the seventh function object to be executed in parallel.
/// </typeparam>
/// <typeparam name="_Function8">
///     The type of the eighth function object to be executed in parallel.
/// </typeparam>
/// <param name="_Func1">
///     The first function object to be executed in parallel.
/// </param>
/// <param name="_Func2">
///     The second function object to be executed in parallel.
/// </param>
/// <param name="_Func3">
///     The third function object to be executed in parallel.
/// </param>
/// <param name="_Func4">
///     The fourth function object to be executed in parallel.
/// </param>
/// <param name="_Func5">
///     The fifth function object to be executed in parallel.
/// </param>
/// <param name="_Func6">
///     The sixth function object to be executed in parallel.
/// </param>
/// <param name="_Func7">
///     The seventh function object to be executed in parallel.
/// </param>
/// <param name="_Func8">
///     The eighth function object to be executed in parallel.
/// </param>
/// <remarks>
///     Note that one or more of the function objects supplied as parameters may execute inline on the calling context.
///     <para>If one or more of the function objects passed as parameters to this function throws an exception, the
///     runtime will select one such exception of its choosing and propagate it out of the call to <c>parallel_invoke</c>.</para>
///     <para>For more information, see <see cref="Parallel Algorithms"/>.</para>
/// </remarks>
/**/
template <typename _Function1, typename _Function2, typename _Function3, typename _Function4, typename _Function5,
    typename _Function6, typename _Function7, typename _Function8>
void parallel_invoke(const _Function1& _Func1, const _Function2& _Func2, const _Function3& _Func3, const _Function4& _Func4, const _Function5& _Func5,
    const _Function6& _Func6, const _Function7& _Func7, const _Function8& _Func8)
{
    _Trace_ppl_function(PPLParallelInvokeEventGuid, _TRACE_LEVEL_INFORMATION, CONCRT_EVENT_START);

    structured_task_group _Task_group;

    task_handle<_Function1> _Task_handle1(_Func1);
    _Task_group.run(_Task_handle1);

    task_handle<_Function2> _Task_handle2(_Func2);
    _Task_group.run(_Task_handle2);

    task_handle<_Function3> _Task_handle3(_Func3);
    _Task_group.run(_Task_handle3);

    task_handle<_Function4> _Task_handle4(_Func4);
    _Task_group.run(_Task_handle4);

    task_handle<_Function5> _Task_handle5(_Func5);
    _Task_group.run(_Task_handle5);

    task_handle<_Function6> _Task_handle6(_Func6);
    _Task_group.run(_Task_handle6);

    task_handle<_Function7> _Task_handle7(_Func7);
    _Task_group.run(_Task_handle7);

    task_handle<_Function8> _Task_handle8(_Func8);
    _Task_group.run_and_wait(_Task_handle8);

    _Trace_ppl_function(PPLParallelInvokeEventGuid, _TRACE_LEVEL_INFORMATION, CONCRT_EVENT_END);
}

/// <summary>
///     Executes the function objects supplied as parameters in parallel, and blocks until they have finished executing. Each function object
///     could be a lambda expression, a pointer to function, or any object that supports the function call operator with the signature
///     <c>void operator()()</c>.
/// </summary>
/// <typeparam name="_Function1">
///     The type of the first function object to be executed in parallel.
/// </typeparam>
/// <typeparam name="_Function2">
///     The type of the second function object to be executed in parallel.
/// </typeparam>
/// <typeparam name="_Function3">
///     The type of the third function object to be executed in parallel.
/// </typeparam>
/// <typeparam name="_Function4">
///     The type of the fourth function object to be executed in parallel.
/// </typeparam>
/// <typeparam name="_Function5">
///     The type of the fifth function object to be executed in parallel.
/// </typeparam>
/// <typeparam name="_Function6">
///     The type of the sixth function object to be executed in parallel.
/// </typeparam>
/// <typeparam name="_Function7">
///     The type of the seventh function object to be executed in parallel.
/// </typeparam>
/// <typeparam name="_Function8">
///     The type of the eighth function object to be executed in parallel.
/// </typeparam>
/// <typeparam name="_Function9">
///     The type of the ninth function object to be executed in parallel.
/// </typeparam>
/// <param name="_Func1">
///     The first function object to be executed in parallel.
/// </param>
/// <param name="_Func2">
///     The second function object to be executed in parallel.
/// </param>
/// <param name="_Func3">
///     The third function object to be executed in parallel.
/// </param>
/// <param name="_Func4">
///     The fourth function object to be executed in parallel.
/// </param>
/// <param name="_Func5">
///     The fifth function object to be executed in parallel.
/// </param>
/// <param name="_Func6">
///     The sixth function object to be executed in parallel.
/// </param>
/// <param name="_Func7">
///     The seventh function object to be executed in parallel.
/// </param>
/// <param name="_Func8">
///     The eighth function object to be executed in parallel.
/// </param>
/// <param name="_Func9">
///     The ninth function object to be executed in parallel.
/// </param>
/// <remarks>
///     Note that one or more of the function objects supplied as parameters may execute inline on the calling context.
///     <para>If one or more of the function objects passed as parameters to this function throws an exception, the
///     runtime will select one such exception of its choosing and propagate it out of the call to <c>parallel_invoke</c>.</para>
///     <para>For more information, see <see cref="Parallel Algorithms"/>.</para>
/// </remarks>
/**/
template <typename _Function1, typename _Function2, typename _Function3, typename _Function4, typename _Function5,
    typename _Function6, typename _Function7, typename _Function8, typename _Function9>
void parallel_invoke(const _Function1& _Func1, const _Function2& _Func2, const _Function3& _Func3, const _Function4& _Func4, const _Function5& _Func5,
    const _Function6& _Func6, const _Function7& _Func7, const _Function8& _Func8, const _Function9& _Func9)
{
    _Trace_ppl_function(PPLParallelInvokeEventGuid, _TRACE_LEVEL_INFORMATION, CONCRT_EVENT_START);

    structured_task_group _Task_group;

    task_handle<_Function1> _Task_handle1(_Func1);
    _Task_group.run(_Task_handle1);

    task_handle<_Function2> _Task_handle2(_Func2);
    _Task_group.run(_Task_handle2);

    task_handle<_Function3> _Task_handle3(_Func3);
    _Task_group.run(_Task_handle3);

    task_handle<_Function4> _Task_handle4(_Func4);
    _Task_group.run(_Task_handle4);

    task_handle<_Function5> _Task_handle5(_Func5);
    _Task_group.run(_Task_handle5);

    task_handle<_Function6> _Task_handle6(_Func6);
    _Task_group.run(_Task_handle6);

    task_handle<_Function7> _Task_handle7(_Func7);
    _Task_group.run(_Task_handle7);

    task_handle<_Function8> _Task_handle8(_Func8);
    _Task_group.run(_Task_handle8);

    task_handle<_Function9> _Task_handle9(_Func9);
    _Task_group.run_and_wait(_Task_handle9);

    _Trace_ppl_function(PPLParallelInvokeEventGuid, _TRACE_LEVEL_INFORMATION, CONCRT_EVENT_END);
}

/// <summary>
///     Executes the function objects supplied as parameters in parallel, and blocks until they have finished executing. Each function object
///     could be a lambda expression, a pointer to function, or any object that supports the function call operator with the signature
///     <c>void operator()()</c>.
/// </summary>
/// <typeparam name="_Function1">
///     The type of the first function object to be executed in parallel.
/// </typeparam>
/// <typeparam name="_Function2">
///     The type of the second function object to be executed in parallel.
/// </typeparam>
/// <typeparam name="_Function3">
///     The type of the third function object to be executed in parallel.
/// </typeparam>
/// <typeparam name="_Function4">
///     The type of the fourth function object to be executed in parallel.
/// </typeparam>
/// <typeparam name="_Function5">
///     The type of the fifth function object to be executed in parallel.
/// </typeparam>
/// <typeparam name="_Function6">
///     The type of the sixth function object to be executed in parallel.
/// </typeparam>
/// <typeparam name="_Function7">
///     The type of the seventh function object to be executed in parallel.
/// </typeparam>
/// <typeparam name="_Function8">
///     The type of the eighth function object to be executed in parallel.
/// </typeparam>
/// <typeparam name="_Function9">
///     The type of the ninth function object to be executed in parallel.
/// </typeparam>
/// <typeparam name="_Function10">
///     The type of the tenth function object to be executed in parallel.
/// </typeparam>
/// <param name="_Func1">
///     The first function object to be executed in parallel.
/// </param>
/// <param name="_Func2">
///     The second function object to be executed in parallel.
/// </param>
/// <param name="_Func3">
///     The third function object to be executed in parallel.
/// </param>
/// <param name="_Func4">
///     The fourth function object to be executed in parallel.
/// </param>
/// <param name="_Func5">
///     The fifth function object to be executed in parallel.
/// </param>
/// <param name="_Func6">
///     The sixth function object to be executed in parallel.
/// </param>
/// <param name="_Func7">
///     The seventh function object to be executed in parallel.
/// </param>
/// <param name="_Func8">
///     The eighth function object to be executed in parallel.
/// </param>
/// <param name="_Func9">
///     The ninth function object to be executed in parallel.
/// </param>
/// <param name="_Func10">
///     The tenth function object to be executed in parallel.
/// </param>
/// <remarks>
///     Note that one or more of the function objects supplied as parameters may execute inline on the calling context.
///     <para>If one or more of the function objects passed as parameters to this function throws an exception, the
///     runtime will select one such exception of its choosing and propagate it out of the call to <c>parallel_invoke</c>.</para>
///     <para>For more information, see <see cref="Parallel Algorithms"/>.</para>
/// </remarks>
/**/
template <typename _Function1, typename _Function2, typename _Function3, typename _Function4, typename _Function5,
    typename _Function6, typename _Function7, typename _Function8, typename _Function9, typename _Function10>
void parallel_invoke(const _Function1& _Func1, const _Function2& _Func2, const _Function3& _Func3, const _Function4& _Func4, const _Function5& _Func5,
    const _Function6& _Func6, const _Function7& _Func7, const _Function8& _Func8, const _Function9& _Func9, const _Function10& _Func10)
{
    _Trace_ppl_function(PPLParallelInvokeEventGuid, _TRACE_LEVEL_INFORMATION, CONCRT_EVENT_START);

    structured_task_group _Task_group;

    task_handle<_Function1> _Task_handle1(_Func1);
    _Task_group.run(_Task_handle1);

    task_handle<_Function2> _Task_handle2(_Func2);
    _Task_group.run(_Task_handle2);

    task_handle<_Function3> _Task_handle3(_Func3);
    _Task_group.run(_Task_handle3);

    task_handle<_Function4> _Task_handle4(_Func4);
    _Task_group.run(_Task_handle4);

    task_handle<_Function5> _Task_handle5(_Func5);
    _Task_group.run(_Task_handle5);

    task_handle<_Function6> _Task_handle6(_Func6);
    _Task_group.run(_Task_handle6);

    task_handle<_Function7> _Task_handle7(_Func7);
    _Task_group.run(_Task_handle7);

    task_handle<_Function8> _Task_handle8(_Func8);
    _Task_group.run(_Task_handle8);

    task_handle<_Function9> _Task_handle9(_Func9);
    _Task_group.run(_Task_handle9);

    task_handle<_Function10> _Task_handle10(_Func10);
    _Task_group.run_and_wait(_Task_handle10);

    _Trace_ppl_function(PPLParallelInvokeEventGuid, _TRACE_LEVEL_INFORMATION, CONCRT_EVENT_END);
}

/// <summary>
///     The <c>auto_partitioner</c> class represents the default method <c>parallel_for</c>, <c>parallel_for_each</c> and
///     <c>parallel_transform</c> use to partition the range they iterates over. This method of partitioning employes range stealing
///     for load balancing as well as per-iterate cancellation.
/// </summary>
/**/
class auto_partitioner
{
public:
    /// <summary>
    ///     Constructs a <c>auto_partitioner</c> object.
    /// </summary>
    /**/
    auto_partitioner() {}

    /// <summary>
    ///     Destroys a <c>auto_partitioner</c> object.
    /// </summary>
    /**/
    ~auto_partitioner() {}

    template<class _Type>
    _Type _Get_num_chunks(_Type ) const
    {
        return static_cast<_Type>(::Concurrency::details::_CurrentScheduler::_GetNumberOfVirtualProcessors());
    }
};

/// <summary>
///     The <c>static_partitioner</c> class represents a static partitioning of the range iterated over by <c>parallel_for</c>. The partitioner
///     divides the range into as many chunks as there are workers available to the underyling scheduler.
/// </summary>
/**/
class static_partitioner
{
public:
    /// <summary>
    ///     Constructs a <c>static_partitioner</c> object.
    /// </summary>
    /**/
    static_partitioner() = default;

    /// <summary>
    ///     Destroys a <c>static_partitioner</c> object.
    /// </summary>
    /**/
    ~static_partitioner() = default;

    template<class _Type>
    _Type _Get_num_chunks(_Type ) const
    {
        return static_cast<_Type>(::Concurrency::details::_CurrentScheduler::_GetNumberOfVirtualProcessors());
    }
};

/// <summary>
///     The <c>simple_partitioner</c> class represents a static partitioning of the range iterated over by <c>parallel_for</c>. The partitioner
///     divides the range into chunks such that each chunk has at least the number of iterations specified by the chunk size.
/// </summary>
/**/
class simple_partitioner
{
private:
    typedef unsigned long long _Size_type;

public:
    /// <summary>
    ///     Constructs a <c>simple_partitioner</c> object.
    /// </summary>
    /// <param name="_M_chunk_size">
    ///     Number of iterations per chunk.
    /// </param>
    /**/
    explicit simple_partitioner(_Size_type _Chunk_size) : _M_chunk_size(_Chunk_size)
    {
        if (_Chunk_size == 0)
        {
            throw ::std::invalid_argument("_Chunk_size");
        }
    }

    /// <summary>
    ///     Destroys a <c>simple_partitioner</c> object.
    /// </summary>
    /**/
    ~simple_partitioner() {}

    template<class _Type>
    _Type _Get_num_chunks(_Type _Range_arg) const
    {
        static_assert(sizeof(_Type) <= sizeof(_Size_type), "Potential truncation of _Range_arg");
        _Size_type _Num_chunks = (static_cast<_Size_type>(_Range_arg) / _M_chunk_size);

        if (_Num_chunks == 0)
        {
           _Num_chunks = 1;
        }

        return static_cast<_Type>(_Num_chunks);
    }

private:

    _Size_type _M_chunk_size;
};

/// <summary>
///     The <c>affinity_partitioner</c> class is similar to the <c>static_partitioner</c> class, but it improves cache affinity
///     by its choice of mapping subranges to worker threads. It can improve performance significantly when a loop is re-executed over
///     the same data set, and the data fits in cache. Note that the same <c>affinity_partitioner</c> object must be used with subsequent
///     iterations of a parallel loop that is executed over a particular data set, to benefit from data locality.
/// </summary>
/**/
class affinity_partitioner
{
public:

    /// <summary>
    ///     Constructs an <c>affinity_partitioner</c> object.
    /// </summary>
    /**/
    affinity_partitioner() : _M_num_chunks(0), _M_pChunk_locations(nullptr)
    {
    }

    /// <summary>
    ///     Destroys an <c>affinity_partitioner</c> object.
    /// </summary>
    /**/
    ~affinity_partitioner()
    {
        delete [] _M_pChunk_locations;
    }

    location& _Get_chunk_location(unsigned int _ChunkIndex)
    {
        return _M_pChunk_locations[_ChunkIndex];
    }

    template<class _Type>
    _Type _Get_num_chunks(_Type )
    {
        if (_M_num_chunks == 0)
        {
            _M_num_chunks = ::Concurrency::details::_CurrentScheduler::_GetNumberOfVirtualProcessors();
            _M_pChunk_locations = new location[_M_num_chunks];
        }

        return static_cast<_Type>(_M_num_chunks);
    }

private:
    // The number of chunks the partitioner will record affinity for.
    unsigned int _M_num_chunks;

    // Array of remembered locations.
    location * _M_pChunk_locations;
};

// Helper methods for scheduling and executing parallel tasks

// Disable C4180: qualifier applied to function type has no meaning; ignored
// Warning fires for passing Foo function pointer to parallel_for instead of &Foo.
#pragma warning(push)
#pragma warning(disable: 4180)
// Disable C6263: using _alloca in a loop; this can quickly overflow stack
#pragma warning(disable: 6263)

// Template class that invokes user function on a parallel_for_each

template <typename _Random_iterator, typename _Index_type, typename _Function, bool _Is_iterator>
class _Parallel_chunk_helper_invoke
{
public:
    static void __cdecl _Invoke(const _Random_iterator& _First, _Index_type& _Index, const _Function& _Func)
    {
        _Func(_First[_Index]);
    }
};

// Template specialized class that invokes user function on a parallel_for

template <typename _Random_iterator, typename _Index_type, typename _Function>
class _Parallel_chunk_helper_invoke<_Random_iterator, _Index_type, _Function, false>
{
public:
    static void __cdecl _Invoke(const _Random_iterator& _First, _Index_type& _Index, const _Function& _Func)
    {
        _Func(static_cast<_Random_iterator>(_First + _Index));
    }
};

// Represents a range of iteration

template<typename _Index_type>
class _Range
{
public:

    // Construct an object for the range [_Current_iteration, _Last_iteration)
    _Range(_Index_type _Current_iteration, _Index_type _Last_iteration)
        : _M_current(_Current_iteration),
        _M_last(_Last_iteration)
    {
        // On creation, the range shall have at least 1 iteration.
        _CONCRT_ASSERT(_Number_of_iterations() > 0);
    }

    // Send a portion of the range to the helper
    void _Send_range(_Range<_Index_type> * _Helper_range)
    {
        // If there are no iterations other than the current one left until finish, there is no help
        // needed. Set the pointer to a special value that helper will understand and continue
        // doing the work.
        _Index_type _Remaining_iterations = _Number_of_iterations();
        if (_Remaining_iterations > 1)
        {
            // Compute the two pieces of the work range: one for the worker and one for helper class.
            _M_last_iteration = _M_current_iteration + _Remaining_iterations / 2;

            // There needs to be at least 1 iteration left because the current iteration cannot be sent.
            _CONCRT_ASSERT(_Number_of_iterations() > 0);
        }

        // This is also a signal for the helper that a range has been sent to it.
        _Helper_range->_M_current_iteration = _M_last_iteration;
    }

    // Steal the entire range and give it to the helper
    void _Steal_range(_Range<_Index_type>  * _Helper_range)
    {
        // We allow stealing only from a range that has at least 1 iteration
        _CONCRT_ASSERT(_Number_of_iterations() > 0);

        _Index_type _Current_iter = _M_current_iteration;

        _Helper_range->_M_current_iteration = _Current_iter + 1;
        _Helper_range->_M_last_iteration = _M_last_iteration;

        _M_last_iteration = _Current_iter + 1;
    }

    // Returns the number of iterations in this range
    _Index_type _Number_of_iterations() const
    {
        return (_M_last_iteration - _M_current_iteration);
    }

    // Returns the current iteration in the range
    _Index_type _Get_current_iteration() const
    {
        return _M_current;
    }

    // Sets the current iteration in the range
    void _Set_current_iteration(const _Index_type _I)
    {
        _M_current = _I;
    }

    __declspec(property(get=_Get_current_iteration, put=_Set_current_iteration)) _Index_type _M_current_iteration;

    // Returns the last iteration in the range
    _Index_type _Get_last_iteration() const
    {
        return _M_last;
    }

    // Sets the last iteration in the range
    void _Set_last_iteration(const _Index_type _I)
    {
        _M_last = _I;
    }

    __declspec(property(get=_Get_last_iteration, put=_Set_last_iteration)) _Index_type _M_last_iteration;

private:

    // These members are volatile because they are updated by the helper
    // and used by the worker.
    volatile _Index_type _M_current;
    volatile _Index_type _M_last;
};

// A proxy for the worker responsible for maintaining communication with the helper

template<typename _Index_type>
class _Worker_proxy
{
public:
    _Worker_proxy(_Worker_proxy *_PParent_worker = nullptr)
        : _M_pHelper_range(nullptr),
        _M_pParent_worker(_PParent_worker),
        _M_beacon(),
        _M_context(),
        _M_completion_count(0),
        _M_pWorker_range(nullptr),
        _M_stop_iterating(0)
    {
        _M_context = ::Concurrency::details::_Context::_CurrentContext();
    }

    ~_Worker_proxy()
    {
        // Make the check to avoid doing extra work in the non-exceptional cases
        if (_M_completion_count != _Tree_Complete)
        {
            // On exception, notify our parent so it breaks out of its loop.
            _Propagate_cancel();

            // On exception, we need to set _M_completion_count to ensure that the helper breaks out of its spin wait.
            _Set_done();
        }
    }

    // Obtain a range from the worker
    bool _Receive_range(_Range<_Index_type> * _Helper_range)
    {
        // If the worker already finished, then there is no work left for the helper
        if (_M_completion_count)
        {
            return false;
        }

        _CONCRT_ASSERT(_Helper_range != nullptr);

        // There are two special values for _M_current_iteration that are not valid: one is the
        // initial value of the working class which it will never share, and the other is
        // the last exclusive iteration of the working class, which has no work to be done.
        // We use the former value so that we can understand worker's response.
        _Index_type _Cached_first_iteration = _Helper_range->_M_current_iteration;

        // Following operation is not done via interlocked operation because it does not have to.
        // Helper lazily registers that it would like to help the worker, but it allows for some
        // time to elapse before that information has made it over to the worker. The idea
        // is not to disturb the worker if it is not necessary. It is possible to add interlocked
        // operation in the future if the time spent in the busy wait loop is too big.
        _CONCRT_ASSERT(_M_pHelper_range == nullptr);
        _M_pHelper_range = _Helper_range;

        ::Concurrency::details::_SpinWaitBackoffNone spinWait(::Concurrency::details::_Context::_Yield);

        // If the worker is done, it will flush the store buffer and signal the helper by
        // changing _M_current_iteration in the helper's range.
        while ((_Helper_range->_M_current_iteration == _Cached_first_iteration) && !_M_completion_count)
        {
            if ((_M_pWorker_range != nullptr) && _M_context._IsSynchronouslyBlocked())
            {
                // Attempt to steal the entire range from the worker if it is synchronously blocked.

                // Make sure that worker makes no forward progress while helper is attempting to
                // steal its range. If worker does get unblocked, simply back off in the helper.
                // Note that there could be another helper running if a range has already been
                // sent to us.
                long _Stop_iterating = _InterlockedIncrement(&_M_stop_iterating);
                _CONCRT_ASSERT(_Stop_iterating > 0);

                // We need to make a local copy as the pointer could be changed by the worker.
                _Range<_Index_type> * _Worker_range = _M_pWorker_range;

                // The order of comparison needs to be preserved. If the parent is blocked, then
                // it cannot send a range (because _M_stop_iterating is already set). If it sent a range
                // before being synchronously blocked, then we are no longer the helper. Refrain
                // from intrusively stealing the range.
                if ((_Worker_range != nullptr) && _M_context._IsSynchronouslyBlocked()
                    && (_Helper_range->_M_current_iteration == _Cached_first_iteration) && !_M_completion_count)
                {
                    _CONCRT_ASSERT(_M_pHelper_range == _Helper_range);

                    _M_pHelper_range = nullptr;
                    _Worker_range->_Steal_range(_Helper_range);

                    _CONCRT_ASSERT(_Helper_range->_M_current_iteration != _Cached_first_iteration);
                }

                // At this point, worker is either:
                //
                // a) no longer blocked so range will come to the helper naturally, or
                // b) out of iterations because helper stole all of it
                _Stop_iterating = _InterlockedDecrement(&_M_stop_iterating);
                _CONCRT_ASSERT(_Stop_iterating >= 0);
            }
            else
            {
                // If there is no work received in a full spin, then start yielding the context
                spinWait._SpinOnce();
            }
        }

        // If the initial iteration is the same as the original first iteration then the
        // worker class is sending the signal that it does not need any help.
        if (_Helper_range->_M_current_iteration == _Cached_first_iteration)
        {
            return false;
        }

        return (_Helper_range->_Number_of_iterations() > 0);
    }

    // Send a portion of our range and notify the helper.
    bool _Send_range(_Range<_Index_type> * _Worker_range)
    {
        // Worker range shall not be available for stealing at this time.
        _CONCRT_ASSERT(_M_pWorker_range == nullptr);

        // Helper shall be registered.
        _CONCRT_ASSERT(_M_pHelper_range != nullptr);

        // Send the range
        _Worker_range->_Send_range(_M_pHelper_range);

        // Notify the helper. The fence ensures that the prior updates are visible.
        _InterlockedExchangePointer(reinterpret_cast<void * volatile *>(&_M_pHelper_range), nullptr);

        // The current iteration should still be left
        _CONCRT_ASSERT(_Worker_range->_Number_of_iterations() >= 1);

        // Indicate if we need another helper
        return (_Worker_range->_Number_of_iterations() > 1);
    }

    // Let the helper know that it is ok to intrusively steal range from the worker by publishing the
    // remaining range.
    void _Enable_intrusive_steal(_Range<_Index_type> * _Worker_range)
    {
        _M_pWorker_range = _Worker_range;
    }

    // Prevent the helper from intrusively stealing range from the worker
    void _Disable_intrusive_steal()
    {
        _M_pWorker_range = nullptr;
        _Wait_on_intrusive_steal();
    }

    bool _Is_helper_registered()
    {
        return (_M_pHelper_range != nullptr);
    }

    bool _Is_done()
    {
        return (_M_completion_count != 0);
    }

    void _Set_done()
    {
        // Let the helper know that this class is done with work and flush the store buffer. This operation
        // ensures that any buffered store to helper range in _Send_range is flushed and
        // available in _Receive_range (so there will be no lost ranges).
        _InterlockedExchange(&_M_completion_count, 1);
    }

    void _Set_tree_done()
    {
        // Make sure that **WE** know when our destructor hits that the entire tree is complete.
        _M_completion_count = _Tree_Complete;
    }

    bool _Is_beacon_signaled()
    {
        return _M_beacon._Is_signaled();
    }

    bool _Verify_beacon_cancellation()
    {
        return _M_beacon._Confirm_cancel();
    }

private:

    // Spin wait for any intrusive steal that is in progress.
    void _Wait_on_intrusive_steal()
    {
        // This code is used to synchronize with helper in case of worker cooperative blocking.
        if (_M_stop_iterating != 0)
        {
            ::Concurrency::details::_SpinWaitBackoffNone spinWait;

            while (_M_stop_iterating != 0)
            {
                spinWait._SpinOnce();
            }
        }
    }

    void _NotifyCancel()
    {
        _M_beacon._Raise();
    }

    void _Propagate_cancel()
    {
        if (_M_pParent_worker != nullptr)
        {
            _M_pParent_worker->_NotifyCancel();
        }
    }

    // Constant indicating sub-tree completion
    static constexpr long _Tree_Complete = 2;

    // Read in the loop
    _Range<_Index_type> * volatile               _M_pHelper_range;

    // Read at the end of the loop
    _Worker_proxy *                              _M_pParent_worker;

    // Written rarely
    ::Concurrency::details::_Cancellation_beacon _M_beacon;
    ::Concurrency::details::_Context             _M_context;

    volatile long                                _M_completion_count;

    // Written to in the loop
    _Range<_Index_type> * volatile               _M_pWorker_range;
    volatile long                                _M_stop_iterating;

    _Worker_proxy const & operator=(_Worker_proxy const&);    // no assignment operator

};

// parallel_for -- Performs parallel iteration over a range of indices from _First to _Last,
// excluding _Last. The order in which each iteration is executed is unspecified and non-deterministic.

// Closure (binding) classes for invoking parallel_for and parallel_for_each, with chunks

// A dynamically rebalancing closure class used for packaging parallel_for or parallel_for_each for invocation in chunks.
// If some tasks finish earlier than others, helper tasks get executed which ensures further distribution of work.

template <typename _Random_iterator, typename _Index_type, typename _Function, typename _Partitioner, bool _Is_iterator>
class _Parallel_chunk_helper
{
public:
    _Parallel_chunk_helper(_Index_type, const _Random_iterator& _First, _Index_type _First_iteration, _Index_type _Last_iteration, const _Index_type& _Step,
        const _Function& _Func, const _Partitioner&, _Worker_proxy<_Index_type> * const _Parent_data = nullptr)
            : _M_first(_First),
            _M_step(_Step),
            _M_function(_Func),
            _M_first_iteration(_First_iteration),
            _M_last_iteration(_Last_iteration),
            _M_parent_worker(_Parent_data)
    {
        // Empty constructor because members are already assigned
    }

        // Constructor overload that accepts a range
    _Parallel_chunk_helper(const _Random_iterator& _First,  const _Index_type& _Step, const _Function& _Func,
        const _Range<_Index_type>& _Worker_range, _Worker_proxy<_Index_type> * const _Parent_data = nullptr)
            : _M_first(_First),
            _M_step(_Step),
            _M_function(_Func),
            _M_first_iteration(_Worker_range._M_current_iteration),
            _M_last_iteration(_Worker_range._M_last_iteration),
            _M_parent_worker(_Parent_data)
    {
        // Empty constructor because members are already assigned
    }

    // The main helper function which iterates over the given collection and invokes user function on every iteration.
    // Function is marked as const even though it does mutate some of its members (those are declared as mutable). This is done
    // in order to easily communicate between a worker and a helper instance, without holding references to many local variables.
    // However, this function does not mutate any state that is visible to anyone outside of this class, nor would that be
    // possible due to the implicit copy of the functor that happens when a new task_handle is created.
    void operator()() const
    {
        _Range<_Index_type> _Worker_range(_M_first_iteration, _M_last_iteration);

        // This class has two modes: worker and helper. The originally split chunk is always a
        // worker, while any subsequent class spawned from this class is in the helper
        // mode, which is signified using a link to the worker class through _M_pOwning_worker
        // handle. So, it will wait for work to be dished out by the working class while in helper mode.
        if (_M_parent_worker != nullptr && !_M_parent_worker->_Receive_range(&_Worker_range))
        {
            // If the worker class rejected the help, simply return
            return;
        }

        // Keep the secondary, scaled, loop index for quick indexing into the data structure
        _Index_type _Current_iteration = _Worker_range._M_current_iteration;
        _Index_type _Scaled_index = _Current_iteration * _M_step;

        // If there is only one iteration to be executed there is no need to initialize any
        // helper classes (work is indivisible).
        if (_Worker_range._Number_of_iterations() == 1)
        {
            // Execute one iteration
            _Parallel_chunk_helper_invoke<_Random_iterator, _Index_type, _Function, _Is_iterator>::_Invoke(_M_first, _Scaled_index, _M_function);
            return;
        }

        // If the execution reaches this point it means that this class now has a chunk of work
        // that it needs to get done, so it has transitioned into the worker mode.
        structured_task_group _Helper_group;

        // Initialize fields that are needed in the helper
        _Worker_proxy<_Index_type> _Worker(_M_parent_worker);

        // Instantiate a helper class for this working class and put it on the work queue.
        // If some thread is idle it will be able to steal the helper and help this class
        // finish its work by stealing a piece of the work range.
        task_handle<_Parallel_chunk_helper> _Helper_task(_Parallel_chunk_helper(_M_first, _M_step, _M_function, _Worker_range, &_Worker));

        _Helper_group.run(_Helper_task);

        ::Concurrency::details::_MallocaListHolder<task_handle<_Parallel_chunk_helper>> _Holder;

        // Normally, for a cancellation semantic in cooperation with the helper, we would run_and_wait the below code on the Helper_group. Unfortunately,
        // the capture by reference of things which must be shared (_Worker, and so forth) will cause the loop below to add additional indirection
        // instructions. The loop below *MUST* be as tight as possible with the defined semantics. Instead, we will manually notify our parent if the
        // worker's destructor runs without hitting the bottom of our chunk. This is done through notification on the beacon.

        for (_Index_type _I = _Current_iteration; _I < _Worker_range._M_last_iteration; (_I++, _Worker_range._M_current_iteration =_I, _Scaled_index += _M_step))
        {
            if (_Worker._Is_beacon_signaled())
            {
                // Either a parent task group is canceled or one of the other iterations
                // threw an exception. Abort the remaining iterations
                //
                // Note that this could be a false positive that we must verify.
                if (_Worker._Is_done() || _Worker._Verify_beacon_cancellation())
                {
                    break;
                }
            }

            if (_Worker._Is_helper_registered())
            {
                // The helper class (there can only be one) registered to help this class with the work.
                // Thus, figure out if this class needs help and split the range among the two classes.

                if (_Worker._Send_range(&_Worker_range))
                {
                    // Construct every new instance of a helper class on the stack because it is beneficial to use
                    // a structured task group where the class itself is responsible for task handle's lifetime.
                    task_handle<_Parallel_chunk_helper> * _Helper_subtask = _Holder._AddRawMallocaNode(_malloca(_Holder._GetAllocationSize()));

                    new(_Helper_subtask) task_handle<_Parallel_chunk_helper>
                        (_Parallel_chunk_helper(_M_first, _M_step, _M_function, _Worker_range, &_Worker));

                    // If _Send_range returns true, that means that there is still some non-trivial
                    // work to be done, so this class will potentially need another helper.
                    _Helper_group.run(*_Helper_subtask);
                }
            }

            // Allow intrusive stealing by the helper
            _Worker._Enable_intrusive_steal(&_Worker_range);

            // Execute one iteration: the element is at scaled index away from the first element.
            _Parallel_chunk_helper_invoke<_Random_iterator, _Index_type, _Function, _Is_iterator>::_Invoke(_M_first, _Scaled_index, _M_function);

            // Helper shall not steal a range after this call
            _Worker._Disable_intrusive_steal();
        }

        // Indicate that the worker is done with its iterations.
        _Worker._Set_done();

        // Wait for all worker/helper iterations to finish
        _Helper_group.wait();

        // Make sure that we've signaled that the tree is complete. This is used to detect any exception out of either _Parallel_chunk_helper_invoke or
        // _Helper_group.wait() above as a cancellation of the loop which must propagate upwards because we do not wrap the loop body in run_and_wait.
        _Worker._Set_tree_done();
    }

private:

    const _Random_iterator&            _M_first;
    const _Index_type&                 _M_step;
    const _Function&                   _M_function;

    const _Index_type                  _M_first_iteration;
    const _Index_type                  _M_last_iteration;

    _Worker_proxy<_Index_type> * const _M_parent_worker;

    _Parallel_chunk_helper const & operator=(_Parallel_chunk_helper const&);    // no assignment operator
};

template <typename _Random_iterator, typename _Index_type, typename _Function, typename _Partitioner, bool _Is_iterator>
class _Parallel_fixed_chunk_helper
{
public:
    _Parallel_fixed_chunk_helper(_Index_type, const _Random_iterator& _First, _Index_type _First_iteration,
         _Index_type _Last_iteration, const _Index_type& _Step, const _Function& _Func, const _Partitioner&)
            : _M_first(_First),
            _M_step(_Step),
            _M_function(_Func),
            _M_first_iteration(_First_iteration),
            _M_last_iteration(_Last_iteration)
    {
        // Empty constructor because members are already assigned
    }

    void operator()() const
    {
        // Keep the secondary, scaled, loop index for quick indexing into the data structure
        _Index_type _Scaled_index = _M_first_iteration * _M_step;

        for (_Index_type _I = _M_first_iteration; _I < _M_last_iteration; (_I++, _Scaled_index += _M_step))
        {
            // Execute one iteration: the element is at scaled index away from the first element.
            _Parallel_chunk_helper_invoke<_Random_iterator, _Index_type, _Function, _Is_iterator>::_Invoke(_M_first, _Scaled_index, _M_function);
        }
    }
private:

    const _Random_iterator&            _M_first;
    const _Index_type&                 _M_step;
    const _Function&                   _M_function;

    const _Index_type                  _M_first_iteration;
    const _Index_type                  _M_last_iteration;

    _Parallel_fixed_chunk_helper const & operator=(_Parallel_fixed_chunk_helper const&);    // no assignment operator
};

template <typename _Random_iterator, typename _Index_type, typename _Function, bool _Is_iterator>
class _Parallel_localized_chunk_helper
{
public:
    typedef _Parallel_fixed_chunk_helper<_Random_iterator, _Index_type, _Function, static_partitioner, _Is_iterator> _Base;

    _Parallel_localized_chunk_helper(_Index_type _Chunk_index, const _Random_iterator& _First, _Index_type _First_iteration, _Index_type _Last_iteration, const _Index_type& _Step,
        const _Function& _Func, affinity_partitioner& _Part)
            : _M_chunk_location(_Part._Get_chunk_location(static_cast<unsigned int>(_Chunk_index))),
            _M_fixed_helper(_Chunk_index, _First, _First_iteration, _Last_iteration, _Step, _Func, static_partitioner())
    {
        // Empty constructor because members are already assigned
    }

    // Override the operator() in the base class. Note that this is not a virtual override.
    void operator()() const
    {
        // Check here if location needs to be saved.
        if (_M_chunk_location._Is_system())
        {
            _M_chunk_location = location::current();
        }

        _M_fixed_helper();
    }
private:

    location&       _M_chunk_location;
    _Base           _M_fixed_helper;
    _Parallel_localized_chunk_helper const & operator=(_Parallel_localized_chunk_helper const&);    // no assignment operator
};

#pragma warning(pop)

template <typename _Worker_class, typename _Index_type, typename Partitioner>
void _Parallel_chunk_task_group_run(structured_task_group& _Task_group,
                                    task_handle<_Worker_class>* _Chunk_helpers,
                                    const Partitioner&,
                                    _Index_type _I)
{
    _Task_group.run(_Chunk_helpers[_I]);
}

template <typename _Worker_class, typename _Index_type>
void _Parallel_chunk_task_group_run(structured_task_group& _Task_group,
                                    task_handle<_Worker_class>* _Chunk_helpers,
                                    affinity_partitioner& _Part,
                                    _Index_type _I)
{
     _Task_group.run(_Chunk_helpers[_I], _Part._Get_chunk_location(static_cast<unsigned int>(_I)));
}


// Helper functions that implement parallel_for

template <typename _Worker_class, typename _Random_iterator, typename _Index_type, typename _Function, typename _Partitioner>
void _Parallel_chunk_impl(const _Random_iterator& _First, _Index_type _Range_arg, const _Index_type& _Step, const _Function& _Func, _Partitioner&& _Part)
{
    _CONCRT_ASSERT(_Range_arg > 1);
    _CONCRT_ASSERT(_Step > 0);

    _Index_type _Num_iterations = _Range_arg;
    if (_Step != 1)
    {
        --_Num_iterations;
        _Num_iterations /= _Step;
        ++_Num_iterations;
    }

    _CONCRT_ASSERT(_Num_iterations > 1);

    _Index_type _Num_chunks = _Part._Get_num_chunks(_Num_iterations);
    _CONCRT_ASSERT(_Num_chunks > 0);

    // Allocate memory on the stack for task_handles to ensure everything is properly structured.
    ::Concurrency::details::_MallocaArrayHolder<task_handle<_Worker_class>> _Holder;
    task_handle<_Worker_class> * _Chunk_helpers = _Holder._InitOnRawMalloca(_malloca(sizeof(task_handle<_Worker_class>) * static_cast<size_t>(_Num_chunks)));

    structured_task_group _Task_group;

    _Index_type _Iterations_per_chunk = _Num_iterations / _Num_chunks;
    _Index_type _Remaining_iterations = _Num_iterations % _Num_chunks;

    // If there are less iterations than desired chunks, set the chunk number
    // to be the number of iterations.
    if (_Iterations_per_chunk == 0)
    {
        _Num_chunks = _Remaining_iterations;
    }

    _Index_type _Work_size = 0;
    _Index_type _Start_iteration = 0;
    _Index_type _I;

    // Split the available work in chunks
    for (_I = 0; _I < _Num_chunks - 1; _I++)
    {
        if (_Remaining_iterations > 0)
        {
            // Iterations are not divided evenly, so add 1 remainder iteration each time
            _Work_size = _Iterations_per_chunk + 1;
            _Remaining_iterations--;
        }
        else
        {
            _Work_size = _Iterations_per_chunk;
        }

        // New up a task_handle "in-place", in the array preallocated on the stack
        new(&_Chunk_helpers[_I]) task_handle<_Worker_class>(_Worker_class(_I, _First, _Start_iteration, _Start_iteration + _Work_size, _Step, _Func, ::std::forward<_Partitioner>(_Part)));
        _Holder._IncrementConstructedElemsCount();

        // Run each of the chunk tasks in parallel
        _Parallel_chunk_task_group_run(_Task_group, _Chunk_helpers, ::std::forward<_Partitioner>(_Part), _I);

        // Prepare for the next iteration
        _Start_iteration += _Work_size;
    }

    // Because this is the last iteration, then work size might be different
    _CONCRT_ASSERT((_Remaining_iterations == 0) || ((_Iterations_per_chunk == 0) && (_Remaining_iterations == 1)));
    _Work_size = _Num_iterations - _Start_iteration;

    // New up a task_handle "in-place", in the array preallocated on the stack
    new(&_Chunk_helpers[_I]) task_handle<_Worker_class>(_Worker_class(_I, _First, _Start_iteration, _Start_iteration + _Work_size, _Step, _Func, ::std::forward<_Partitioner>(_Part)));
    _Holder._IncrementConstructedElemsCount();

    _Task_group.run_and_wait(_Chunk_helpers[_I]);
}

template <typename _Worker_class, typename _Random_iterator, typename _Index_type, typename _Function>
void _Parallel_chunk_impl(const _Random_iterator& _First, _Index_type _Range_arg, const _Index_type& _Step, const _Function& _Func)
{
    _Parallel_chunk_impl<_Worker_class>(_First, _Range_arg, _Step, _Func, auto_partitioner());
}

// Helper for the parallel for API with the default dynamic partitioner which implements range-stealing to balance load.
template <typename _Index_type, typename _Diff_type, typename _Function>
void _Parallel_for_partitioned_impl(_Index_type _First, _Diff_type _Range_arg, _Diff_type _Step, const _Function& _Func, const auto_partitioner& _Part)
{
    typedef _Parallel_chunk_helper<_Index_type, _Diff_type, _Function, auto_partitioner, false> _Worker_class;
    _Parallel_chunk_impl<_Worker_class>(_First, _Range_arg, _Step, _Func, _Part);
}

// Helper for the parallel_for API with a static partitioner - creates a fixed number of chunks up front with no range-stealing enabled.
template <typename _Index_type, typename _Diff_type, typename _Function>
void _Parallel_for_partitioned_impl(_Index_type _First, _Diff_type _Range_arg, _Diff_type _Step, const _Function& _Func, const static_partitioner& _Part)
{
    typedef _Parallel_fixed_chunk_helper<_Index_type, _Diff_type, _Function, static_partitioner, false> _Worker_class;
    _Parallel_chunk_impl<_Worker_class>(_First, _Range_arg, _Step, _Func, _Part);
}

// Helper for the parallel_for API with a simple partitioner - creates a fixed number of chunks up front with no range-stealing enabled.
template <typename _Index_type, typename _Diff_type, typename _Function>
void _Parallel_for_partitioned_impl(_Index_type _First, _Diff_type _Range_arg, _Diff_type _Step, const _Function& _Func, const simple_partitioner& _Part)
{
    typedef _Parallel_fixed_chunk_helper<_Index_type, _Diff_type, _Function, simple_partitioner, false> _Worker_class;
    _Parallel_chunk_impl<_Worker_class>(_First, _Range_arg, _Step, _Func, _Part);
}

// Helper for the parallel_for API with an affinity partitioner - creates a fixed number of chunks up front with no range-stealing enabled. subsequent
// calls to parallel_for with the same affinity partitioner (pass in as a non-const reference) are scheduled to the same location they previously ran on
template <typename _Index_type, typename _Diff_type, typename _Function>
void _Parallel_for_partitioned_impl(_Index_type _First, _Diff_type _Range_arg, _Diff_type _Step, const _Function& _Func, affinity_partitioner& _Part)
{
    typedef _Parallel_localized_chunk_helper<_Index_type, _Diff_type, _Function, false> _Worker_class;
    _Parallel_chunk_impl<_Worker_class>(_First, _Range_arg, _Step, _Func, _Part);
}

template <typename _Index_type, typename _Function, typename _Partitioner>
void _Parallel_for_impl(_Index_type _First, _Index_type _Last, _Index_type _Step, const _Function& _Func, _Partitioner&& _Part)
{
    // The step argument must be 1 or greater; otherwise it is an invalid argument
    if (_Step < 1)
    {
        throw ::std::invalid_argument("_Step");
    }

    // If there are no elements in this range we just return
    if (_First >= _Last)
    {
        return;
    }

    using _Diff_type = ::std::conditional_t<::std::is_integral_v<_Index_type>,
        ::std::conditional_t<sizeof(_Index_type) <= sizeof(long), unsigned long, unsigned long long>,
        decltype(_Last - _First)>;

    _Diff_type _Range_size = static_cast<_Diff_type>(_Last) - static_cast<_Diff_type>(_First);
    _Diff_type _Diff_step = _Step;

    if (_Range_size <= _Diff_step)
    {
        _Func(_First);
    }
    else
    {
        _Parallel_for_partitioned_impl<_Index_type, _Diff_type, _Function>(_First, _Range_size, _Step, _Func, ::std::forward<_Partitioner>(_Part));
    }
}

template <typename _Index_type, typename _Function>
void _Parallel_for_impl(_Index_type _First, _Index_type _Last, _Index_type _Step, const _Function& _Func)
{
    _Parallel_for_impl(_First, _Last, _Step, _Func, auto_partitioner());
}

/// <summary>
///     <c>parallel_for</c> iterates over a range of indices and executes a user-supplied function at each iteration, in parallel.
/// </summary>
/// <typeparam name="_Index_type">
///     The type of the index being used for the iteration.
/// </typeparam>
/// <typeparam name="_Function">
///     The type of the function that will be executed at each iteration.
/// </typeparam>
/// <typeparam name="_Partitioner">
///     The type of the partitioner that is used to partition the supplied range.
/// </typeparam>
/// <param name="_First">
///     The first index to be included in the iteration.
/// </param>
/// <param name="_Last">
///     The index one past the last index to be included in the iteration.
/// </param>
/// <param name="_Step">
///     The value by which to step when iterating from <paramref name="_First"/> to <paramref name="_Last"/>. The step must be positive.
///     <see cref="invalid_argument Class">invalid_argument</see> is thrown if the step is less than 1.
/// </param>
/// <param name="_Func">
///     The function to be executed at each iteration. This may be a lambda expression, a function pointer, or any object
///     that supports a version of the function call operator with the signature
///     <c>void operator()(</c><typeparamref name="_Index_type"/><c>)</c>.
/// </param>
/// <param name="_Part">
///     A reference to the partitioner object. The argument can be one of
///     <c>const</c> <see ref="auto_partitioner Class">auto_partitioner</see><c>&amp;</c>,
///     <c>const</c> <see ref="static_partitioner Class">static_partitioner</see><c>&amp;</c>,
///     <c>const</c> <see ref="simple_partitioner Class">simple_partitioner</see><c>&amp;</c> or
///     <see ref="affinity_partitioner Class">affinity_partitioner</see><c>&amp;</c>
///     If an <see ref="affinity_partitioner Class">affinity_partitioner</see> object is used, the reference must be a non-const l-value reference,
///     so that the algorithm can store state for future loops to re-use.
/// </param>
/// <remarks>
///     For more information, see <see cref="Parallel Algorithms"/>.
/// </remarks>
/**/
template <typename _Index_type, typename _Function, typename _Partitioner>
void parallel_for(_Index_type _First, _Index_type _Last, _Index_type _Step, const _Function& _Func, _Partitioner&& _Part)
{
    _Trace_ppl_function(PPLParallelForEventGuid, _TRACE_LEVEL_INFORMATION, CONCRT_EVENT_START);
    _Parallel_for_impl(_First, _Last, _Step, _Func, ::std::forward<_Partitioner>(_Part));
    _Trace_ppl_function(PPLParallelForEventGuid, _TRACE_LEVEL_INFORMATION, CONCRT_EVENT_END);
}

/// <summary>
///     <c>parallel_for</c> iterates over a range of indices and executes a user-supplied function at each iteration, in parallel.
/// </summary>
/// <typeparam name="_Index_type">
///     The type of the index being used for the iteration. <paramref name="_Index_type"/> must be an integral type.
/// </typeparam>
/// <typeparam name="_Function">
///     The type of the function that will be executed at each iteration.
/// </typeparam>
/// <param name="_First">
///     The first index to be included in the iteration.
/// </param>
/// <param name="_Last">
///     The index one past the last index to be included in the iteration.
/// </param>
/// <param name="_Step">
///     The value by which to step when iterating from <paramref name="_First"/> to <paramref name="_Last"/>. The step must be positive.
///     <see cref="invalid_argument Class">invalid_argument</see> is thrown if the step is less than 1.
/// </param>
/// <param name="_Func">
///     The function to be executed at each iteration. This may be a lambda expression, a function pointer, or any object
///     that supports a version of the function call operator with the signature
///     <c>void operator()(</c><typeparamref name="_Index_type"/><c>)</c>.
/// </param>
/// <remarks>
///     For more information, see <see cref="Parallel Algorithms"/>.
/// </remarks>
/**/
template <typename _Index_type, typename _Function>
void parallel_for(_Index_type _First, _Index_type _Last, _Index_type _Step, const _Function& _Func)
{
    parallel_for(_First, _Last, _Step, _Func, auto_partitioner());
}

/// <summary>
///     <c>parallel_for</c> iterates over a range of indices and executes a user-supplied function at each iteration, in parallel.
/// </summary>
/// <typeparam name="_Index_type">
///     The type of the index being used for the iteration.
/// </typeparam>
/// <typeparam name="_Function">
///     The type of the function that will be executed at each iteration.
/// </typeparam>
/// <param name="_First">
///     The first index to be included in the iteration.
/// </param>
/// <param name="_Last">
///     The index one past the last index to be included in the iteration.
/// </param>
/// <param name="_Func">
///     The function to be executed at each iteration. This may be a lambda expression, a function pointer, or any object
///     that supports a version of the function call operator with the signature
///     <c>void operator()(</c><typeparamref name="_Index_type"/><c>)</c>.
/// </param>
/// <param name="_Part">
///     A reference to the partitioner object. The argument can be one of
///     <c>const</c> <see ref="auto_partitioner Class">auto_partitioner</see><c>&amp;</c>,
///     <c>const</c> <see ref="static_partitioner Class">static_partitioner</see><c>&amp;</c>,
///     <c>const</c> <see ref="simple_partitioner Class">simple_partitioner</see><c>&amp;</c> or
///     <see ref="affinity_partitioner Class">affinity_partitioner</see><c>&amp;</c>
///     If an <see ref="affinity_partitioner Class">affinity_partitioner</see> object is used, the reference must be a non-const l-value reference,
///     so that the algorithm can store state for future loops to re-use.
/// </param>
/// <remarks>
///     For more information, see <see cref="Parallel Algorithms"/>.
/// </remarks>
/**/
template <typename _Index_type, typename _Function>
void parallel_for(_Index_type _First, _Index_type _Last, const _Function& _Func, const auto_partitioner& _Part = auto_partitioner())
{
    parallel_for(_First, _Last, static_cast<_Index_type>(1), _Func, _Part);
}

/// <summary>
///     <c>parallel_for</c> iterates over a range of indices and executes a user-supplied function at each iteration, in parallel.
/// </summary>
/// <typeparam name="_Index_type">
///     The type of the index being used for the iteration.
/// </typeparam>
/// <typeparam name="_Function">
///     The type of the function that will be executed at each iteration.
/// </typeparam>
/// <param name="_First">
///     The first index to be included in the iteration.
/// </param>
/// <param name="_Last">
///     The index one past the last index to be included in the iteration.
/// </param>
/// <param name="_Func">
///     The function to be executed at each iteration. This may be a lambda expression, a function pointer, or any object
///     that supports a version of the function call operator with the signature
///     <c>void operator()(</c><typeparamref name="_Index_type"/><c>)</c>.
/// </param>
/// <param name="_Part">
///     A reference to the partitioner object. The argument can be one of
///     <c>const</c> <see ref="auto_partitioner Class">auto_partitioner</see><c>&amp;</c>,
///     <c>const</c> <see ref="static_partitioner Class">static_partitioner</see><c>&amp;</c>,
///     <c>const</c> <see ref="simple_partitioner Class">simple_partitioner</see><c>&amp;</c> or
///     <see ref="affinity_partitioner Class">affinity_partitioner</see><c>&amp;</c>
///     If an <see ref="affinity_partitioner Class">affinity_partitioner</see> object is used, the reference must be a non-const l-value reference,
///     so that the algorithm can store state for future loops to re-use.
/// </param>
/// <remarks>
///     For more information, see <see cref="Parallel Algorithms"/>.
/// </remarks>
/**/
template <typename _Index_type, typename _Function>
void parallel_for(_Index_type _First, _Index_type _Last, const _Function& _Func, const static_partitioner& _Part)
{
    parallel_for(_First, _Last, static_cast<_Index_type>(1), _Func, _Part);
}

/// <summary>
///     <c>parallel_for</c> iterates over a range of indices and executes a user-supplied function at each iteration, in parallel.
/// </summary>
/// <typeparam name="_Index_type">
///     The type of the index being used for the iteration.
/// </typeparam>
/// <typeparam name="_Function">
///     The type of the function that will be executed at each iteration.
/// </typeparam>
/// <param name="_First">
///     The first index to be included in the iteration.
/// </param>
/// <param name="_Last">
///     The index one past the last index to be included in the iteration.
/// </param>
/// <param name="_Func">
///     The function to be executed at each iteration. This may be a lambda expression, a function pointer, or any object
///     that supports a version of the function call operator with the signature
///     <c>void operator()(</c><typeparamref name="_Index_type"/><c>)</c>.
/// </param>
/// <param name="_Part">
///     A reference to the partitioner object. The argument can be one of
///     <c>const</c> <see ref="auto_partitioner Class">auto_partitioner</see><c>&amp;</c>,
///     <c>const</c> <see ref="static_partitioner Class">static_partitioner</see><c>&amp;</c>,
///     <c>const</c> <see ref="simple_partitioner Class">simple_partitioner</see><c>&amp;</c> or
///     <see ref="affinity_partitioner Class">affinity_partitioner</see><c>&amp;</c>
///     If an <see ref="affinity_partitioner Class">affinity_partitioner</see> object is used, the reference must be a non-const l-value reference,
///     so that the algorithm can store state for future loops to re-use.
/// </param>
/// <remarks>
///     For more information, see <see cref="Parallel Algorithms"/>.
/// </remarks>
/**/
template <typename _Index_type, typename _Function>
void parallel_for(_Index_type _First, _Index_type _Last, const _Function& _Func, const simple_partitioner& _Part)
{
    parallel_for(_First, _Last, static_cast<_Index_type>(1), _Func, _Part);
}

/// <summary>
///     <c>parallel_for</c> iterates over a range of indices and executes a user-supplied function at each iteration, in parallel.
/// </summary>
/// <typeparam name="_Index_type">
///     The type of the index being used for the iteration.
/// </typeparam>
/// <typeparam name="_Function">
///     The type of the function that will be executed at each iteration.
/// </typeparam>
/// <param name="_First">
///     The first index to be included in the iteration.
/// </param>
/// <param name="_Last">
///     The index one past the last index to be included in the iteration.
/// </param>
/// <param name="_Func">
///     The function to be executed at each iteration. This may be a lambda expression, a function pointer, or any object
///     that supports a version of the function call operator with the signature
///     <c>void operator()(</c><typeparamref name="_Index_type"/><c>)</c>.
/// </param>
/// <param name="_Part">
///     A reference to the partitioner object. The argument can be one of
///     <c>const</c> <see ref="auto_partitioner Class">auto_partitioner</see><c>&amp;</c>,
///     <c>const</c> <see ref="static_partitioner Class">static_partitioner</see><c>&amp;</c>,
///     <c>const</c> <see ref="simple_partitioner Class">simple_partitioner</see><c>&amp;</c> or
///     <see ref="affinity_partitioner Class">affinity_partitioner</see><c>&amp;</c>
///     If an <see ref="affinity_partitioner Class">affinity_partitioner</see> object is used, the reference must be a non-const l-value reference,
///     so that the algorithm can store state for future loops to re-use.
/// </param>
/// <remarks>
///     For more information, see <see cref="Parallel Algorithms"/>.
/// </remarks>
/**/
template <typename _Index_type, typename _Function>
void parallel_for(_Index_type _First, _Index_type _Last, const _Function& _Func, affinity_partitioner& _Part)
{
    parallel_for(_First, _Last, static_cast<_Index_type>(1), _Func, _Part);
}

// parallel_for_each -- This function will iterate over all elements in the iterator's range.

// Closure (binding) classes for invoking parallel_for_each recursively

// A closure class used for packaging chunk of elements in parallel_for_each for parallel invocation

// Forward iterator for_each using unstructured task group

// Disable C4180: qualifier applied to function type has no meaning; ignored
// Warning fires for passing Foo function pointer to parallel_for instead of &Foo.
#pragma warning(push)
#pragma warning(disable: 4180)

template <typename _Forward_iterator, typename _Function, unsigned int _Chunk_size>
class _Parallel_for_each_helper
{
public:
    typedef typename ::std::iterator_traits<_Forward_iterator>::value_type _Value_type;
    static constexpr unsigned int _Size = _Chunk_size;

    _Parallel_for_each_helper(_Forward_iterator& _First, const _Forward_iterator& _Last, const _Function& _Func) :
        _M_function(_Func), _M_len(0)
    {
        static_assert(::std::is_lvalue_reference_v<decltype(*_First)>, "lvalue required for forward iterator operator *");
        // Add a batch of work items to this functor's array
        for (unsigned int _Index=0; (_Index < _Size) && (_First != _Last); _Index++)
        {
            _M_element[_M_len++] = &(*_First++);
        }
    }

    void operator()() const
    {
        // Invoke parallel_for on the batched up array of elements
        _Parallel_for_impl(0U, _M_len, 1U,
            [this] (unsigned int _Index)
            {
                _M_function(*(_M_element[_Index]));
            }
        );
    }

private:

    const _Function& _M_function;
    typename ::std::iterator_traits<_Forward_iterator>::pointer    _M_element[_Size];
    unsigned int     _M_len;

    _Parallel_for_each_helper const & operator=(_Parallel_for_each_helper const&);    // no assignment operator
};

#pragma warning(pop)

// Helper functions that implement parallel_for_each

template <typename _Forward_iterator, typename _Function>
void _Parallel_for_each_chunk(_Forward_iterator& _First, const _Forward_iterator& _Last, const _Function& _Func, task_group& _Task_group)
{
    // The chunk size selection depends more on the internal implementation of parallel_for than
    // on the actual input. Also, it does not have to be dynamically computed, but it helps
    // parallel_for if it is a power of 2 (easy to divide).
    const unsigned int _Chunk_size = 1024;

    // This functor will be copied on the heap and will execute the chunk in parallel
    _Parallel_for_each_helper<_Forward_iterator, _Function, _Chunk_size> _Functor(_First, _Last, _Func);

    // Because this is an unstructured task group, running the task will make a copy of the necessary data
    // on the heap, ensuring that it is available at the time of execution.
    _Task_group.run(_Functor);
}

template <typename _Forward_iterator, typename _Function>
void _Parallel_for_each_forward_impl(_Forward_iterator& _First, const _Forward_iterator& _Last, const _Function& _Func, task_group& _Task_group)
{
    _Parallel_for_each_chunk(_First, _Last, _Func, _Task_group);

    // If there is a tail, push the tail
    if (_First != _Last)
    {
        _Task_group.run(
            [&_First, &_Last, &_Func, &_Task_group]
            {
                ::Concurrency::_Parallel_for_each_forward_impl(_First, _Last, _Func, _Task_group);
            }
        );
    }
}

template <typename _Forward_iterator, typename _Function>
void _Parallel_for_each_impl(_Forward_iterator _First, const _Forward_iterator& _Last, const _Function& _Func, const auto_partitioner&, ::std::forward_iterator_tag)
{
    // Because this is a forward iterator, it is difficult to validate that _First comes before _Last, so
    // it is up to the user to provide valid range.
    if (_First != _Last)
    {
        task_group _Task_group;

        _Parallel_for_each_forward_impl(_First, _Last, _Func, _Task_group);

        _Task_group.wait();
    }
}

template <typename _Random_iterator, typename _Index_type, typename _Function>
void _Parallel_for_each_partitioned_impl(const _Random_iterator& _First, _Index_type _Range_arg, _Index_type _Step, const _Function& _Func, const auto_partitioner& _Part)
{
    typedef _Parallel_chunk_helper<_Random_iterator, _Index_type, _Function, auto_partitioner, true> _Worker_class;
        // Use the same function that schedules work for parallel for
    _Parallel_chunk_impl<_Worker_class>(_First, _Range_arg, _Step, _Func, _Part);
}

template <typename _Random_iterator, typename _Index_type, typename _Function>
void _Parallel_for_each_partitioned_impl(const _Random_iterator& _First, _Index_type _Range_arg, _Index_type _Step, const _Function& _Func, const static_partitioner& _Part)
{
    typedef _Parallel_fixed_chunk_helper<_Random_iterator, _Index_type, _Function, static_partitioner, true> _Worker_class;
    // Use the same function that schedules work for parallel for
    _Parallel_chunk_impl<_Worker_class>(_First, _Range_arg, _Step, _Func, _Part);
}

template <typename _Random_iterator, typename _Index_type, typename _Function>
void _Parallel_for_each_partitioned_impl(const _Random_iterator& _First, _Index_type _Range_arg, _Index_type _Step, const _Function& _Func, const simple_partitioner& _Part)
{
    typedef _Parallel_fixed_chunk_helper<_Random_iterator, _Index_type, _Function, simple_partitioner, true> _Worker_class;
    // Use the same function that schedules work for parallel for
    _Parallel_chunk_impl<_Worker_class>(_First, _Range_arg, _Step, _Func, _Part);
}

template <typename _Random_iterator, typename _Index_type, typename _Function>
void _Parallel_for_each_partitioned_impl(const _Random_iterator& _First, _Index_type _Range_arg, _Index_type _Step, const _Function& _Func, affinity_partitioner& _Part)
{
    typedef _Parallel_localized_chunk_helper<_Random_iterator, _Index_type, _Function, true> _Worker_class;
        // Use the same function that schedules work for parallel for
    _Parallel_chunk_impl<_Worker_class>(_First, _Range_arg, _Step, _Func, _Part);
}

template <typename _Random_iterator, typename _Function, typename _Partitioner>
void _Parallel_for_each_impl(const _Random_iterator& _First, const _Random_iterator& _Last, const _Function& _Func, _Partitioner&& _Part, ::std::random_access_iterator_tag)
{
    typedef typename ::std::iterator_traits<_Random_iterator>::difference_type _Index_type;

    // Exit early if there is nothing in the collection
    if (_First >= _Last)
    {
        return;
    }

    _Index_type _Range_size = _Last - _First;

    if (_Range_size == 1)
    {
        _Func(*_First);
    }
    else
    {
        _Index_type _Step = 1;

        _Parallel_for_each_partitioned_impl(_First, _Range_size, _Step, _Func, ::std::forward<_Partitioner>(_Part));
   }
}

/// <summary>
///     <c>parallel_for_each</c> applies a specified function to each element within a range, in parallel. It is semantically
///     equivalent to the <c>for_each</c> function in the <c>std</c> namespace, except that iteration over the elements is
///     performed in parallel, and the order of iteration is unspecified. The argument <paramref name="_Func"/> must support
///     a function call operator of the form <c>operator()(T)</c> where the parameter <paramref name="T"/> is the item type
///     of the container being iterated over.
/// </summary>
/// <typeparam name="_Iterator">
///     The type of the iterator being used to iterate over the container.
/// </typeparam>
/// <typeparam name="_Function">
///     The type of the function that will be applied to each element within the range.
/// </typeparam>
/// <param name="_First">
///     An iterator addressing the position of the first element to be included in parallel iteration.
/// </param>
/// <param name="_Last">
///     An iterator addressing the position one past the final element to be included in parallel iteration.
/// </param>
/// <param name="_Func">
///     A user-defined function object that is applied to each element in the range.
/// </param>
/// <remarks>
///     <see ref="auto_partitioner Class">auto_partitioner</see> will be used for the overload without an explicit partitioner.
///     <para>For iterators that do not support random access, only <see ref="auto_partitioner Class">auto_partitioner</see> is supported.</para>
///     <para>For more information, see <see cref="Parallel Algorithms"/>.</para>
/// </remarks>
/**/
template <typename _Iterator, typename _Function>
void parallel_for_each(_Iterator _First, _Iterator _Last, const _Function& _Func)
{
    parallel_for_each(_First, _Last, _Func, auto_partitioner());
}

/// <summary>
///     <c>parallel_for_each</c> applies a specified function to each element within a range, in parallel. It is semantically
///     equivalent to the <c>for_each</c> function in the <c>std</c> namespace, except that iteration over the elements is
///     performed in parallel, and the order of iteration is unspecified. The argument <paramref name="_Func"/> must support
///     a function call operator of the form <c>operator()(T)</c> where the parameter <paramref name="T"/> is the item type
///     of the container being iterated over.
/// </summary>
/// <typeparam name="_Iterator">
///     The type of the iterator being used to iterate over the container.
/// </typeparam>
/// <typeparam name="_Function">
///     The type of the function that will be applied to each element within the range.
/// </typeparam>
/// <param name="_First">
///     An iterator addressing the position of the first element to be included in parallel iteration.
/// </param>
/// <param name="_Last">
///     An iterator addressing the position one past the final element to be included in parallel iteration.
/// </param>
/// <param name="_Func">
///     A user-defined function object that is applied to each element in the range.
/// </param>
/// <param name="_Part">
///     A reference to the partitioner object. The argument can be one of
///     <c>const</c> <see ref="auto_partitioner Class">auto_partitioner</see><c>&amp;</c>,
///     <c>const</c> <see ref="static_partitioner Class">static_partitioner</see><c>&amp;</c>,
///     <c>const</c> <see ref="simple_partitioner Class">simple_partitioner</see><c>&amp;</c> or
///     <see ref="affinity_partitioner Class">affinity_partitioner</see><c>&amp;</c>
///     If an <see ref="affinity_partitioner Class">affinity_partitioner</see> object is used, the reference must be a non-const l-value reference,
///     so that the algorithm can store state for future loops to re-use.
/// </param>
/// <remarks>
///     <see ref="auto_partitioner Class">auto_partitioner</see> will be used for the overload without an explicit partitioner.
///     <para>For iterators that do not support random access, only <see ref="auto_partitioner Class">auto_partitioner</see> is supported.</para>
///     <para>For more information, see <see cref="Parallel Algorithms"/>.</para>
/// </remarks>
/**/
template <typename _Iterator, typename _Function, typename _Partitioner>
void parallel_for_each(_Iterator _First, _Iterator _Last, const _Function& _Func, _Partitioner&& _Part)
{
    _Trace_ppl_function(PPLParallelForeachEventGuid, _TRACE_LEVEL_INFORMATION, CONCRT_EVENT_START);
    _Parallel_for_each_impl(_First, _Last, _Func, ::std::forward<_Partitioner>(_Part), typename ::std::iterator_traits<_Iterator>::iterator_category());
    _Trace_ppl_function(PPLParallelForeachEventGuid, _TRACE_LEVEL_INFORMATION, CONCRT_EVENT_END);
}

// Disable C4180: qualifier applied to function type has no meaning; ignored
// Warning fires for passing Foo function pointer to parallel_for instead of &Foo.
#pragma warning(push)
#pragma warning(disable: 4180)

// Helper function assemble all functors
template <typename _Reduce_type_param, typename _Sub_function, typename _Combinable_type>
struct _Reduce_functor_helper
{
    typedef _Reduce_type_param _Reduce_type;

    const _Sub_function &_Sub_fun;
    const _Reduce_type &_Identity_value;

    _Combinable_type &_Combinable;

    typedef typename _Combinable_type::_Bucket _Bucket_type;

    _Reduce_functor_helper(const _Reduce_type &_Identity, const _Sub_function &_Sub_fun, _Combinable_type &&_Comb)
        : _Sub_fun(_Sub_fun),
        _Identity_value(_Identity),
        _Combinable(_Comb)
    {
    }

private:
    _Reduce_functor_helper &operator =(const _Reduce_functor_helper &_Other);
};

// Ordered serial combinable object
template<typename _Ty, typename _Sym_fun>
class _Order_combinable
{
public:
    // Only write once, limited contention will be caused
    struct _Bucket
    {
        // Allocate enough space in the Bucket to hold a value
        char _Value[(sizeof(_Ty) / sizeof(char))];
        _Bucket * _Next;

        _Bucket(_Bucket *_N)
            : // _Value(), intentionally garbage-init
            _Next(_N)
        {
        }

        void _Insert(_Bucket *_Item)
        {
            // No need to lock, only one thread will insert
            _Item->_Next = _Next;
            _Next = _Item;
        }

        // Construct value in bucket
        void _Put(const _Ty &_Cur)
        {
            new(reinterpret_cast<_Ty *>(&_Value)) _Ty(_Cur);
        }
    };

private:
    const _Sym_fun &_M_fun;
    size_t _M_number;
    _Bucket *_M_root;
    _Order_combinable &operator =(const _Order_combinable &_Other);

public:
    _Bucket *_Construct(_Bucket *_Par = 0)
    {
        _Bucket * _Ret = static_cast<_Bucket *>(::Concurrency::Alloc(sizeof(_Bucket)));
        return new(_Ret)_Bucket(_Par);
    }

    _Order_combinable(const _Sym_fun &_Fun)
        : _M_fun(_Fun),
        _M_number(0),
        _M_root(0)
    {
    }

    ~_Order_combinable()
    {
        while (_M_root)
        {
            _Bucket *_Cur = _M_root;
            _M_root = _M_root->_Next;
            reinterpret_cast<_Ty &>(_Cur->_Value).~_Ty();
            ::Concurrency::Free(_Cur);
        }
    }

    // Serially combine and release the list, return result
    _Ty _Serial_combine_release()
    {
        _Ty _Ret(reinterpret_cast<_Ty &>(_M_root->_Value));
        _Bucket *_Cur = _M_root;
        _M_root = _M_root->_Next;

        while (_M_root)
        {
            reinterpret_cast<_Ty &>(_Cur->_Value).~_Ty();
            ::Concurrency::Free(_Cur);
            _Cur = _M_root;
            _Ret = _M_fun(reinterpret_cast <_Ty &> (_Cur->_Value), _Ret);
            _M_root = _M_root->_Next;
        }

        reinterpret_cast<_Ty &>(_Cur->_Value).~_Ty();
        ::Concurrency::Free(_Cur);

        return _Ret;
    }

    // allocate a bucket and push back to the list
    _Bucket *_Unsafe_push_back()
    {
        return _M_root = _Construct(_M_root);
    }
};

/// <summary>
///     Computes the sum of all elements in a specified range by computing successive partial sums, or computes the result of successive partial
///     results similarly obtained from using a specified binary operation other than sum, in parallel. <c>parallel_reduce</c> is semantically similar to
///     <c>std::accumulate</c>, except that it requires the binary operation to be associative, and requires an identity value instead of an initial value.
/// </summary>
/// <typeparam name="_Reduce_type">
///     The type that the input will reduce to, which can be different from the input element type.
///     The return value and identity value will has this type.
/// </typeparam>
/// <typeparam name="_Forward_iterator">
///     The iterator type of input range.
/// </typeparam>
/// <typeparam name="_Range_reduce_fun">
///     The type of the range reduction function. This must be a function type with signature <c>_Reduce_type _Range_fun(_Forward_iterator, _Forward_iterator, _Reduce_type)</c>,
///     _Reduce_type is the same as the identity type and the result type of the reduction.
/// </typeparam>
/// <typeparam name="_Sym_reduce_fun">
///     The type of the symmetric reduction function. This must be a function type with signature <c>_Reduce_type _Sym_fun(_Reduce_type, _Reduce_type)</c>, where
///     _Reduce_type is the same as the identity type and the result type of the reduction. For the third overload, this should be consistent
///     with the output type of <c>_Range_reduce_fun</c>.
/// </typeparam>
/// <param name="_Begin">
///     An input iterator addressing the first element in the range to be reduced.
/// </param>
/// <param name="_End">
///     An input iterator addressing the element that is one position beyond the final element in the range to be reduced.
/// </param>
/// <param name="_Identity">
///     The identity value <paramref name="_Identity"/> is of the same type as the result type of the reduction and also the <c>value_type</c> of the iterator
///     for the first and second overloads. For the third overload, the identity value must have the same type as the result type of the reduction, but can be
///     different from the <c>value_type</c> of the iterator. It must have an appropriate value such that the range reduction operator <paramref name="_Range_fun"/>,
///     when applied to a range of a single element of type <c>value_type</c> and the identity value, behaves like a type cast of the value from type
///     <c>value_type</c> to the identity type.
/// </param>
/// <param name="_Range_fun">
///     The function that will be used in the first phase of the reduction. Refer to Remarks for more information.
/// </param>
/// <param name="_Sym_fun">
///     The symmetric function that will be used in the second of the reduction. Refer to Remarks for more information.
/// </param>
/// <returns>
///     The result of the reduction.
/// </returns>
/// <remarks>
///     To perform a parallel reduction, the function divides the range into chunks based on the number of workers available to the underlying
///     scheduler. The reduction takes place in two phases, the first phase performs a reduction within each chunk, and the second phase performs
///     a reduction between the partial results from each chunk.
///     <para>The first overload requires that the iterator's <c>value_type</c>, <c>T</c>, be the same as the identity value type as well as the reduction
///     result type. The element type T must provide the operator <c>T T::operator + (T)</c> to reduce elements in each chunk. The same operator is
///     used in the second phase as well.</para>
///     <para>The second overload also requires that the iterator's <c>value_type</c> be the same as the identity value type as well as the reduction
///     result type. The supplied binary operator <paramref name="_Sym_fun"/> is used in both reduction phases, with the identity value as the initial
///     value for the first phase.</para>
///     <para>For the third overload, the identity value type must be the same as the reduction result type, but the iterator's <c>value_type</c> may be
///     different from both. The range reduction function <paramref name="_Range_fun"/> is used in the first phase with the identity
///     value as the initial value, and the binary function <paramref name="_Sym_reduce_fun"/> is applied to sub results in the second phase.</para>
/// </remarks>
/**/
template<typename _Reduce_type, typename _Forward_iterator, typename _Range_reduce_fun, typename _Sym_reduce_fun>
inline _Reduce_type parallel_reduce(_Forward_iterator _Begin, _Forward_iterator _End, const _Reduce_type& _Identity,
    const _Range_reduce_fun &_Range_fun, const _Sym_reduce_fun &_Sym_fun)
{
    typedef typename ::std::iterator_traits<_Forward_iterator>::value_type _Value_type;

    static_assert(!::std::is_same_v<typename ::std::iterator_traits<_Forward_iterator>::iterator_category, ::std::input_iterator_tag>
        && !::std::is_same_v<typename ::std::iterator_traits<_Forward_iterator>::iterator_category, ::std::output_iterator_tag>,
        "iterator can not be input_iterator or output_iterator.");

    return _Parallel_reduce_impl(_Begin, _End,
        _Reduce_functor_helper<_Reduce_type, _Range_reduce_fun,
            _Order_combinable<_Reduce_type, _Sym_reduce_fun>>(_Identity, _Range_fun, _Order_combinable<_Reduce_type, _Sym_reduce_fun>(_Sym_fun)),
        typename ::std::iterator_traits<_Forward_iterator>::iterator_category());
}

/// <summary>
///     Computes the sum of all elements in a specified range by computing successive partial sums, or computes the result of successive partial
///     results similarly obtained from using a specified binary operation other than sum, in parallel. <c>parallel_reduce</c> is semantically similar to
///     <c>std::accumulate</c>, except that it requires the binary operation to be associative, and requires an identity value instead of an initial value.
/// </summary>
/// <typeparam name="_Forward_iterator">
///     The iterator type of input range.
/// </typeparam>
/// <typeparam name="_Sym_reduce_fun">
///     The type of the symmetric reduction function. This must be a function type with signature <c>_Reduce_type _Sym_fun(_Reduce_type, _Reduce_type)</c>, where
///     _Reduce_type is the same as the identity type and the result type of the reduction. For the third overload, this should be consistent
///     with the output type of <c>_Range_reduce_fun</c>.
/// </typeparam>
/// <param name="_Begin">
///     An input iterator addressing the first element in the range to be reduced.
/// </param>
/// <param name="_End">
///     An input iterator addressing the element that is one position beyond the final element in the range to be reduced.
/// </param>
/// <param name="_Identity">
///     The identity value <paramref name="_Identity"/> is of the same type as the result type of the reduction and also the <c>value_type</c> of the iterator
///     for the first and second overloads. For the third overload, the identity value must have the same type as the result type of the reduction, but can be
///     different from the <c>value_type</c> of the iterator. It must have an appropriate value such that the range reduction operator <paramref name="_Range_fun"/>,
///     when applied to a range of a single element of type <c>value_type</c> and the identity value, behaves like a type cast of the value from type
///     <c>value_type</c> to the identity type.
/// </param>
/// <param name="_Sym_fun">
///     The symmetric function that will be used in the second of the reduction. Refer to Remarks for more information.
/// </param>
/// <returns>
///     The result of the reduction.
/// </returns>
/// <remarks>
///     To perform a parallel reduction, the function divides the range into chunks based on the number of workers available to the underlying
///     scheduler. The reduction takes place in two phases, the first phase performs a reduction within each chunk, and the second phase performs
///     a reduction between the partial results from each chunk.
///     <para>The first overload requires that the iterator's <c>value_type</c>, <c>T</c>, be the same as the identity value type as well as the reduction
///     result type. The element type T must provide the operator <c>T T::operator + (T)</c> to reduce elements in each chunk. The same operator is
///     used in the second phase as well.</para>
///     <para>The second overload also requires that the iterator's <c>value_type</c> be the same as the identity value type as well as the reduction
///     result type. The supplied binary operator <paramref name="_Sym_fun"/> is used in both reduction phases, with the identity value as the initial
///     value for the first phase.</para>
///     <para>For the third overload, the identity value type must be the same as the reduction result type, but the iterator's <c>value_type</c> may be
///     different from both. The range reduction function <paramref name="_Range_fun"/> is used in the first phase with the identity
///     value as the initial value, and the binary function <paramref name="_Sym_reduce_fun"/> is applied to sub results in the second phase.</para>
/// </remarks>
/**/
template<typename _Forward_iterator, typename _Sym_reduce_fun>
inline typename ::std::iterator_traits<_Forward_iterator>::value_type parallel_reduce(_Forward_iterator _Begin, _Forward_iterator _End,
    const typename ::std::iterator_traits<_Forward_iterator>::value_type &_Identity, _Sym_reduce_fun _Sym_fun)
{
    typedef ::std::remove_cv_t<typename ::std::iterator_traits<_Forward_iterator>::value_type> _Reduce_type;

    return parallel_reduce(_Begin, _End, _Identity,
        [_Sym_fun](_Forward_iterator _Begin, _Forward_iterator _End, _Reduce_type _Init)->_Reduce_type
        {
            while (_Begin != _End)
            {
                _Init = _Sym_fun(_Init, *_Begin++);
            }

            return _Init;
        },
        _Sym_fun);
}

/// <summary>
///     Computes the sum of all elements in a specified range by computing successive partial sums, or computes the result of successive partial
///     results similarly obtained from using a specified binary operation other than sum, in parallel. <c>parallel_reduce</c> is semantically similar to
///     <c>std::accumulate</c>, except that it requires the binary operation to be associative, and requires an identity value instead of an initial value.
/// </summary>
/// <typeparam name="_Forward_iterator">
///     The iterator type of input range.
/// </typeparam>
/// <param name="_Begin">
///     An input iterator addressing the first element in the range to be reduced.
/// </param>
/// <param name="_End">
///     An input iterator addressing the element that is one position beyond the final element in the range to be reduced.
/// </param>
/// <param name="_Identity">
///     The identity value <paramref name="_Identity"/> is of the same type as the result type of the reduction and also the <c>value_type</c> of the iterator
///     for the first and second overloads. For the third overload, the identity value must have the same type as the result type of the reduction, but can be
///     different from the <c>value_type</c> of the iterator. It must have an appropriate value such that the range reduction operator <paramref name="_Range_fun"/>,
///     when applied to a range of a single element of type <c>value_type</c> and the identity value, behaves like a type cast of the value from type
///     <c>value_type</c> to the identity type.
/// </param>
/// <returns>
///     The result of the reduction.
/// </returns>
/// <remarks>
///     To perform a parallel reduction, the function divides the range into chunks based on the number of workers available to the underlying
///     scheduler. The reduction takes place in two phases, the first phase performs a reduction within each chunk, and the second phase performs
///     a reduction between the partial results from each chunk.
///     <para>The first overload requires that the iterator's <c>value_type</c>, <c>T</c>, be the same as the identity value type as well as the reduction
///     result type. The element type T must provide the operator <c>T T::operator + (T)</c> to reduce elements in each chunk. The same operator is
///     used in the second phase as well.</para>
///     <para>The second overload also requires that the iterator's <c>value_type</c> be the same as the identity value type as well as the reduction
///     result type. The supplied binary operator <paramref name="_Sym_fun"/> is used in both reduction phases, with the identity value as the initial
///     value for the first phase.</para>
///     <para>For the third overload, the identity value type must be the same as the reduction result type, but the iterator's <c>value_type</c> may be
///     different from both. The range reduction function <paramref name="_Range_fun"/> is used in the first phase with the identity
///     value as the initial value, and the binary function <paramref name="_Sym_reduce_fun"/> is applied to sub results in the second phase.</para>
/// </remarks>
/**/
template<typename _Forward_iterator>
inline typename ::std::iterator_traits<_Forward_iterator>::value_type parallel_reduce(
    _Forward_iterator _Begin, _Forward_iterator _End, const typename ::std::iterator_traits<_Forward_iterator>::value_type &_Identity)
{
    return parallel_reduce(_Begin, _End, _Identity, ::std::plus<typename ::std::iterator_traits<_Forward_iterator>::value_type>());
}

// Implementation for the parallel reduce
template <typename _Forward_iterator, typename _Function>
typename _Function::_Reduce_type _Parallel_reduce_impl(_Forward_iterator _First, const _Forward_iterator& _Last, const _Function& _Func,
    ::std::forward_iterator_tag)
{
    // Because this is a forward iterator, it is difficult to validate that _First comes before _Last, so
    // it is up to the user to provide valid range.
    if (_First != _Last)
    {
        task_group _Task_group;
        _Parallel_reduce_forward_executor(_First, _Last, _Func, _Task_group);
        _Task_group.wait();
        return _Func._Combinable._Serial_combine_release();
    }
    else
    {
        return _Func._Identity_value;
    }
}

// All the code below is the worker without range stealing
template<typename _Forward_iterator, typename _Functor>
class _Parallel_reduce_fixed_worker
{
public:
    // The bucket allocation order will depend on the worker construction order
    _Parallel_reduce_fixed_worker(_Forward_iterator _Begin, _Forward_iterator _End, const _Functor &_Fun)
        : _M_fun(_Fun),
        _M_begin(_Begin),
        _M_end(_End),
        _M_bucket(_M_fun._Combinable._Unsafe_push_back())
    {
    }

    void operator ()() const
    {
        _M_bucket->_Put(_M_fun._Sub_fun(_M_begin, _M_end, _M_fun._Identity_value));
    }

private:
    const _Functor &_M_fun;
    const _Forward_iterator _M_begin, _M_end;
    typename _Functor::_Bucket_type * const _M_bucket;
    _Parallel_reduce_fixed_worker &operator =(const _Parallel_reduce_fixed_worker &_Other);
};

template <typename _Worker, typename _Random_iterator, typename _Function>
void _Parallel_reduce_random_executor(_Random_iterator _Begin, _Random_iterator _End, const _Function& _Fun);

template <typename _Random_iterator, typename _Function>
typename _Function::_Reduce_type _Parallel_reduce_impl(_Random_iterator _First, _Random_iterator _Last, const _Function& _Func,
    ::std::random_access_iterator_tag)
{
    typedef _Parallel_reduce_fixed_worker<_Random_iterator, _Function> _Worker_class;

    // Special case for 0, 1 element
    if (_First >= _Last)
    {
        return _Func._Identity_value;
    }
    // Directly compute if size is too small
    else if (_Last - _First <= 1)
    {
        _Worker_class(_First, _Last, _Func)();
        return _Func._Combinable._Serial_combine_release();
    }
    else
    {
        // Use fixed ordered chunk partition to schedule works
        _Parallel_reduce_random_executor<_Worker_class>(_First, _Last, _Func);
        return _Func._Combinable._Serial_combine_release();
    }
}

// the parallel worker executor for fixed iterator
// it will divide fixed number of chunks
// almost same as fixed parallel for, except keep the chunk dividing order
template <typename _Worker, typename _Random_iterator, typename _Function>
void _Parallel_reduce_random_executor(_Random_iterator _Begin, _Random_iterator _End, const _Function& _Fun)
{
    size_t _Cpu_num = static_cast<size_t>(::Concurrency::details::_CurrentScheduler::_GetNumberOfVirtualProcessors()), _Size = _End - _Begin;

    structured_task_group _Tg;
    ::Concurrency::details::_MallocaArrayHolder<task_handle<_Worker>> _Holder;
    task_handle<_Worker> *_Tasks = _Holder._InitOnRawMalloca(_malloca(sizeof(task_handle<_Worker>) * (_Cpu_num - 1)));

    size_t _Begin_index = 0;
    size_t _Step = _Size / _Cpu_num;
    size_t _NumRemaining = _Size - _Step * _Cpu_num;

    for(size_t _I = 0; _I < _Cpu_num - 1; _I++)
    {
        size_t _Next = _Begin_index + _Step;

        // Add remaining to each chunk
        if (_NumRemaining)
        {
            --_NumRemaining;
            ++_Next;
        }

        // New up a task_handle "in-place", in the array preallocated on the stack
        new (_Tasks + _I) task_handle<_Worker>(_Worker(_Begin + _Begin_index, _Begin + _Next, _Fun));
        _Holder._IncrementConstructedElemsCount();

        // Run each of the chunk _Tasks in parallel
        _Tg.run(_Tasks[_I]);
        _Begin_index = _Next;
    }

    task_handle<_Worker> _Tail(_Worker(_Begin + _Begin_index, _End, _Fun));
    _Tg.run_and_wait(_Tail);
}

// The parallel worker executor for forward iterators
// Divide chunks on the fly
template <typename _Forward_iterator, typename _Function, int _Default_worker_size, int _Default_chunk_size>
struct _Parallel_reduce_forward_executor_helper
{
    typedef _Parallel_reduce_fixed_worker<_Forward_iterator, _Function> _Worker_class;
    mutable task_handle<_Worker_class> * _Workers;
    int _Worker_size;

    _Parallel_reduce_forward_executor_helper(_Forward_iterator &_First, _Forward_iterator _Last, const _Function& _Func)
        : _Workers(static_cast<task_handle<_Worker_class> *>(
            ::Concurrency::Alloc(sizeof(task_handle<_Worker_class>) * _Default_worker_size))),
        _Worker_size(0)
    {
        while (_Worker_size < _Default_worker_size && _First != _Last)
        {
            // Copy the range _Head
            _Forward_iterator _Head = _First;

            // Read from forward iterator
            for (size_t _I = 0; _I < _Default_chunk_size && _First != _Last; ++_I, ++_First)
            {
                // Body is empty
            }

            // _First will be the end of current chunk
            new (_Workers + _Worker_size++) task_handle<_Worker_class>(_Worker_class(_Head, _First, _Func));
        }
    }

    _Parallel_reduce_forward_executor_helper(const _Parallel_reduce_forward_executor_helper &_Other):
        _Workers(_Other._Workers), _Worker_size(_Other._Worker_size)
    {
        _Other._Workers = 0; // _Other releases ownership of _Workers on copy.
    }

    void operator ()() const
    {
        structured_task_group _Tg;
        for(int _I = 0; _I < _Worker_size; _I++)
        {
            _Tg.run(_Workers[_I]);
        }
        _Tg.wait();
    }

    ~_Parallel_reduce_forward_executor_helper()
    {
        if (_Workers)
        {
            for (int _I = 0; _I < _Worker_size; _I++)
            {
                _Workers[_I].~task_handle<_Worker_class>();
            }
            ::Concurrency::Free(_Workers);
            _Workers = 0;
        }
    }
};

template <typename _Forward_iterator, typename _Function>
void _Parallel_reduce_forward_executor(_Forward_iterator _First, _Forward_iterator _Last, const _Function& _Func, task_group& _Task_group)
{
    static constexpr int _Internal_worker_number = 1024;
    static constexpr int _Default_chunk_size = 512;
    typedef _Parallel_reduce_fixed_worker<_Forward_iterator, _Function> _Worker_class;

    structured_task_group _Worker_group;
    ::Concurrency::details::_MallocaArrayHolder<task_handle<_Worker_class>> _Holder;
    task_handle<_Worker_class>* _Workers = _Holder._InitOnRawMalloca(_malloca(_Internal_worker_number * sizeof(task_handle<_Worker_class>)));

    // Start execution first
    int _Index = 0;
    while (_Index < _Internal_worker_number && _First != _Last)
    {
        // Copy the range _Head
        _Forward_iterator _Head = _First;

        // Read from forward iterator
        for (size_t _I = 0; _I < _Default_chunk_size && _First != _Last; ++_I, ++_First)
        {
            // Body is empty
        };

        // Create a new task, _First is range _End
        new (_Workers + _Index) task_handle<_Worker_class>(_Worker_class(_Head, _First, _Func));
        _Holder._IncrementConstructedElemsCount();
        _Worker_group.run(_Workers[_Index]);
        ++_Index;
    }

    // Divide and append the left
    while (_First != _Last)
    {
        _Task_group.run(_Parallel_reduce_forward_executor_helper<_Forward_iterator, _Function, _Internal_worker_number, _Default_chunk_size>(_First, _Last, _Func));
    }

    _Worker_group.wait();
}

#pragma warning(pop)


// Disable C4180: qualifier applied to function type has no meaning; ignored
// Warning fires for passing Foo function pointer to parallel_for instead of &Foo.
#pragma warning(push)
#pragma warning(disable: 4180)

//
// Dispatch the execution and handle the condition that all of the iterators are random access
//
template<typename _Any_input_traits, typename _Any_output_traits>
struct _Unary_transform_impl_helper
{
    template<typename _Input_iterator, typename _Output_iterator, typename _Unary_operator>
    static void _Parallel_transform_unary_impl(_Input_iterator _Begin, _Input_iterator _End, _Output_iterator& _Result, const _Unary_operator& _Unary_op, const auto_partitioner&)
    {
        task_group _Tg;
        _Parallel_transform_unary_impl2(_Begin, _End, _Result, _Unary_op, _Tg);
        _Tg.wait();
    }
};

template<>
struct _Unary_transform_impl_helper<::std::random_access_iterator_tag, ::std::random_access_iterator_tag>
{
    template<typename _Random_input_iterator, typename _Random_output_iterator, typename _Unary_operator, typename _Partitioner>
    static void _Parallel_transform_unary_impl(_Random_input_iterator _Begin, _Random_input_iterator _End,
        _Random_output_iterator& _Result, const _Unary_operator& _Unary_op, _Partitioner&& _Part)
    {
        if (_Begin < _End)
        {
            ::Concurrency::_Parallel_for_impl(static_cast<size_t>(0), static_cast<size_t>(_End - _Begin), static_cast<size_t>(1),
                [_Begin, &_Result, &_Unary_op](size_t _Index)
                {
                    _Result[_Index] = _Unary_op(_Begin[_Index]);
                },
                ::std::forward<_Partitioner>(_Part));
            _Result += _End - _Begin;
        }
    }
};

template<typename _Any_input_traits1, typename _Any_input_traits2, typename _Any_output_traits>
struct _Binary_transform_impl_helper
{

    template<typename _Input_iterator1, typename _Input_iterator2, typename _Output_iterator, typename _Binary_operator>
    static void _Parallel_transform_binary_impl(_Input_iterator1 _Begin1, _Input_iterator1 _End1, _Input_iterator2 _Begin2,
        _Output_iterator& _Result, const _Binary_operator& _Binary_op, const auto_partitioner&)
    {
        task_group _Tg;
        _Parallel_transform_binary_impl2(_Begin1, _End1, _Begin2, _Result, _Binary_op, _Tg);
        _Tg.wait();
    }
};

template<>
struct _Binary_transform_impl_helper<::std::random_access_iterator_tag, ::std::random_access_iterator_tag, ::std::random_access_iterator_tag>
{
    template<typename _Random_input_iterator1, typename _Random_input_iterator2, typename _Random_output_iterator, typename _Binary_operator, typename _Partitioner>
    static void _Parallel_transform_binary_impl(_Random_input_iterator1 _Begin1, _Random_input_iterator1 _End1,
        _Random_input_iterator2 _Begin2, _Random_output_iterator& _Result, const _Binary_operator& _Binary_op, _Partitioner&& _Part)
    {
        if (_Begin1 < _End1)
        {
            ::Concurrency::_Parallel_for_impl(static_cast<size_t>(0), static_cast<size_t>(_End1 - _Begin1), static_cast<size_t>(1),
                [_Begin1, _Begin2, &_Result, &_Binary_op](size_t _Index)
                {
                    _Result[_Index] = _Binary_op(_Begin1[_Index], _Begin2[_Index]);
                },
                ::std::forward<_Partitioner>(_Part));
            _Result += _End1 - _Begin1;
        }
    }
};

//
// The implementation for at least one of the iterator is forward iterator
//
template <typename _Forward_iterator, typename _Iterator_kind>
class _Iterator_helper
{
public:
    static constexpr size_t _Size = 1024;
    typedef typename ::std::iterator_traits<_Forward_iterator>::value_type value_type;

    _Iterator_helper()
    {
        static_assert(!::std::is_same_v<_Iterator_kind, ::std::input_iterator_tag>
            && !::std::is_same_v<_Iterator_kind, ::std::output_iterator_tag>,
            "iterator can not be input_iterator or output_iterator.");
    }

    size_t _Populate(_Forward_iterator& _First, _Forward_iterator _Last)
    {
        size_t _Length = 0;
        static_assert(::std::is_lvalue_reference_v<decltype(*_First)>, "lvalue required for forward iterator operator *");

        for (size_t _Index=0; (_Index < _Size) && (_First != _Last); _Index++)
        {
            // We only support l-value here, so it's safe
            _M_element_array[_Length++] = &(*_First++);
        }

        return _Length;
    }

    void _Populate(_Forward_iterator& _First, size_t _Length)
    {
        for (size_t _Index=0; _Index < _Length; _Index++)
        {
            _M_element_array[_Index] = &(*_First++);
        }
    }

    void _Store(const value_type& _Elem, size_t _Index) const
    {
        *(_M_element_array[_Index]) = _Elem;
    }

    typename ::std::iterator_traits<_Forward_iterator>::reference _Load(size_t _Index) const
    {
        return *(_M_element_array[_Index]);
    }

private:
    typename ::std::iterator_traits<_Forward_iterator>::pointer _M_element_array[_Size];
};

template <typename _Random_iterator>
class _Iterator_helper<_Random_iterator, ::std::random_access_iterator_tag>
{
public:
    static constexpr size_t _Size = 1024;
    typedef typename ::std::iterator_traits<_Random_iterator>::value_type value_type;

    _Iterator_helper()
    {
    }

    size_t _Populate(_Random_iterator& _First, _Random_iterator _Last)
    {
        typename ::std::iterator_traits<_Random_iterator>::difference_type _Range_size = _Last - _First;
        typename ::std::iterator_traits<_Random_iterator>::difference_type _Sized = _Size;
        _M_first = _First;

        if (_Range_size > _Sized)
        {
            _First += _Size;
            return _Size;
        }
        else
        {
            _First += _Range_size;
            return static_cast<size_t>(_Range_size);
        }
    }

    void _Populate(_Random_iterator& _First, size_t _Length)
    {
        _M_first = _First;
        _First += _Length;
    }

    void _Store(const value_type& _Elem, size_t _Index) const
    {
        _M_first[_Index] = _Elem;
    }

    typename ::std::iterator_traits<_Random_iterator>::reference _Load(size_t _Index) const
    {
        // We only support l-value here
        return _M_first[_Index];
    }

private:
    _Random_iterator _M_first;
};

template <typename _Input_iterator1, typename _Input_iterator2, typename _Output_iterator, typename _Binary_operator>
class _Parallel_transform_binary_helper
{
public:
    _Parallel_transform_binary_helper(_Input_iterator1& _First1, _Input_iterator1 _Last1, _Input_iterator2& _First2,
        _Output_iterator& _Result, const _Binary_operator& _Binary_op)
            : _M_input_helper1(),
            _M_input_helper2(),
            _M_output_helper(),
            _M_binary_op(_Binary_op),
            _M_len(_M_input_helper1._Populate(_First1, _Last1))
        {
            _M_input_helper2._Populate(_First2, _M_len);
            _M_output_helper._Populate(_Result, _M_len);
        }

        void operator()() const
        {
            // Invoke parallel_for on the batched up array of elements
            ::Concurrency::_Parallel_for_impl(static_cast<size_t>(0), _M_len, static_cast<size_t>(1),
                [this] (size_t _Index)
                {
                    _M_output_helper._Store(_M_binary_op(_M_input_helper1._Load(_Index), _M_input_helper2._Load(_Index)), _Index);
                });
        }

private:

    _Iterator_helper<_Input_iterator1, typename ::std::iterator_traits<_Input_iterator1>::iterator_category> _M_input_helper1;
    _Iterator_helper<_Input_iterator2, typename ::std::iterator_traits<_Input_iterator2>::iterator_category> _M_input_helper2;
    _Iterator_helper<_Output_iterator, typename ::std::iterator_traits<_Output_iterator>::iterator_category> _M_output_helper;
    const _Binary_operator&                                                                                  _M_binary_op;
    size_t                                                                                                   _M_len;

    _Parallel_transform_binary_helper const & operator=(_Parallel_transform_binary_helper const&);    // no assignment operator
};

template <typename _Input_iterator1, typename _Input_iterator2, typename _Output_iterator, typename _Binary_operator>
void _Parallel_transform_binary_impl2(_Input_iterator1 _First1, _Input_iterator1 _Last1, _Input_iterator2 _First2, _Output_iterator &_Result,
    const _Binary_operator& _Binary_op, task_group& _Tg)
{
    // This functor will be copied on the heap and will execute the chunk in parallel
    {
        _Parallel_transform_binary_helper<_Input_iterator1, _Input_iterator2, _Output_iterator, _Binary_operator> _Functor(_First1, _Last1, _First2, _Result, _Binary_op);
        _Tg.run(_Functor);
    }

    // If there is a tail, push the tail
    if (_First1 != _Last1)
    {
        _Tg.run(
            [=, &_Result, &_Binary_op, &_Tg]
            {
                _Parallel_transform_binary_impl2(_First1, _Last1, _First2, _Result, _Binary_op, _Tg);
            });
    }
}

template <typename _Input_iterator, typename _Output_iterator, typename _Unary_operator>
class _Parallel_transform_unary_helper
{
public:
    _Parallel_transform_unary_helper(_Input_iterator& _First, _Input_iterator _Last, _Output_iterator &_Result, const _Unary_operator& _Unary_op)
            : _M_input_helper(),
            _M_output_helper(),
            _M_unary_op(_Unary_op),
            _M_len(_M_input_helper._Populate(_First, _Last))
        {
            _M_output_helper._Populate(_Result, _M_len);
        }

        void operator()() const
        {
            // Invoke parallel_for on the batched up array of elements
            ::Concurrency::_Parallel_for_impl(static_cast<size_t>(0), _M_len, static_cast<size_t>(1),
                [this] (size_t _Index)
                {
                    _M_output_helper._Store(_M_unary_op(_M_input_helper._Load(_Index)), _Index);
                });
        }

private:

    _Iterator_helper<_Input_iterator, typename ::std::iterator_traits<_Input_iterator>::iterator_category>   _M_input_helper;
    _Iterator_helper<_Output_iterator, typename ::std::iterator_traits<_Output_iterator>::iterator_category> _M_output_helper;
    const _Unary_operator&                                                                                   _M_unary_op;
    size_t                                                                                                   _M_len;

    _Parallel_transform_unary_helper const & operator=(_Parallel_transform_unary_helper const&);    // no assignment operator
};

template <typename _Input_iterator, typename _Output_iterator, typename _Unary_operator>
void _Parallel_transform_unary_impl2(_Input_iterator _First, _Input_iterator _Last, _Output_iterator &_Result,
    const _Unary_operator& _Unary_op, task_group& _Tg)
{
    // This functor will be copied on the heap and will execute the chunk in parallel
    {
        _Parallel_transform_unary_helper<_Input_iterator, _Output_iterator, _Unary_operator> _Functor(_First, _Last, _Result, _Unary_op);
        _Tg.run(_Functor);
    }

    // If there is a tail, push the tail
    if (_First != _Last)
    {
        _Tg.run(
            [=, &_Result, &_Unary_op, &_Tg]
            {
                _Parallel_transform_unary_impl2(_First, _Last, _Result, _Unary_op, _Tg);
            });
    }
}

template <typename _Input_iterator, typename _Output_iterator, typename _Unary_operator, typename _Partitioner>
_Output_iterator _Parallel_transform_unary_impl(_Input_iterator _First, _Input_iterator _Last, _Output_iterator _Result, const _Unary_operator& _Unary_op, _Partitioner&& _Part)
{
    typedef typename ::std::iterator_traits<_Input_iterator>::iterator_category _Input_iterator_type;
    typedef typename ::std::iterator_traits<_Output_iterator>::iterator_category _Output_iterator_type;

    if (_First != _Last)
    {
        _Unary_transform_impl_helper<_Input_iterator_type, _Output_iterator_type>
            ::_Parallel_transform_unary_impl(_First, _Last, _Result, _Unary_op, ::std::forward<_Partitioner>(_Part));
    }

    return _Result;
}

/// <summary>
///     Applies a specified function object to each element in a source range, or to a pair of elements from two source ranges,
///     and copies the return values of the function object into a destination range, in parallel. This functional is semantically
///     equivalent to <c>std::transform</c>.
/// </summary>
/// <typeparam name="_Input_iterator1">
///     The type of the first or only input iterator.
/// </typeparam>
/// <typeparam name="_Output_iterator">
///     The type of the output iterator.
/// </typeparam>
/// <typeparam name="_Unary_operator">
///     The type of the unary functor to be executed on each element in the input range.
/// </typeparam>
/// <param name="_First1">
///     An input iterator addressing the position of the first element in the first or only source range to be operated on.
/// </param>
/// <param name="_Last1">
///     An input iterator addressing the position one past the final element in the first or only source range to be operated on.
/// </param>
/// <param name="_Result">
///     An output iterator addressing the position of the first element in the destination range.
/// </param>
/// <param name="_Unary_op">
///     A user-defined unary function object that is applied to each element in the source range.
/// </param>
/// <param name="_Part">
///     A reference to the partitioner object. The argument can be one of
///     <c>const</c> <see ref="auto_partitioner Class">auto_partitioner</see><c>&amp;</c>,
///     <c>const</c> <see ref="static_partitioner Class">static_partitioner</see><c>&amp;</c>,
///     <c>const</c> <see ref="simple_partitioner Class">simple_partitioner</see><c>&amp;</c> or
///     <see ref="affinity_partitioner Class">affinity_partitioner</see><c>&amp;</c>
///     If an <see ref="affinity_partitioner Class">affinity_partitioner</see> object is used, the reference must be a non-const l-value reference,
///     so that the algorithm can store state for future loops to re-use.
/// </param>
/// <returns>
///     An output iterator addressing the position one past the final element in the destination range that is receiving the output elements
///     transformed by the function object.
/// </returns>
/// <remarks>
///     <see ref="auto_partitioner Class">auto_partitioner</see> will be used for the overloads without an explicit partitioner argument.
///     <para>For iterators that do not support random access, only <see ref="auto_partitioner Class">auto_partitioner</see> is supported.</para>
///     <para>The overloads that take the argument <paramref name="_Unary_op"/> transform the input range into the output range by applying
///     the unary functor to each element in the input range. <paramref name="_Unary_op"/> must support the function call operator with signature
///     <c>operator()(T)</c> where <c>T</c> is the value type of the range being iterated over.</para>
///     <para>The overloads that take the argument <paramref name="_Binary_op"/> transform two input ranges into the output range by applying the
///     binary functor to one element from the first input range and one element from the second input range. <paramref name="_Binary_op"/> must support
///     the function call operator with signature <c>operator()(T, U)</c> where <c>T</c>, <c>U</c> are value types of the two input iterators.</para>
///     <para>For more information, see <see cref="Parallel Algorithms"/>.</para>
/// </remarks>
/**/
template <typename _Input_iterator1, typename _Output_iterator, typename _Unary_operator>
_Output_iterator parallel_transform(_Input_iterator1 _First1, _Input_iterator1 _Last1, _Output_iterator _Result, const _Unary_operator& _Unary_op, const auto_partitioner& _Part = auto_partitioner())
{
    return _Parallel_transform_unary_impl(_First1, _Last1, _Result, _Unary_op, _Part);
}

/// <summary>
///     Applies a specified function object to each element in a source range, or to a pair of elements from two source ranges,
///     and copies the return values of the function object into a destination range, in parallel. This functional is semantically
///     equivalent to <c>std::transform</c>.
/// </summary>
/// <typeparam name="_Input_iterator1">
///     The type of the first or only input iterator.
/// </typeparam>
/// <typeparam name="_Output_iterator">
///     The type of the output iterator.
/// </typeparam>
/// <typeparam name="_Unary_operator">
///     The type of the unary functor to be executed on each element in the input range.
/// </typeparam>
/// <param name="_First1">
///     An input iterator addressing the position of the first element in the first or only source range to be operated on.
/// </param>
/// <param name="_Last1">
///     An input iterator addressing the position one past the final element in the first or only source range to be operated on.
/// </param>
/// <param name="_Result">
///     An output iterator addressing the position of the first element in the destination range.
/// </param>
/// <param name="_Unary_op">
///     A user-defined unary function object that is applied to each element in the source range.
/// </param>
/// <param name="_Part">
///     A reference to the partitioner object. The argument can be one of
///     <c>const</c> <see ref="auto_partitioner Class">auto_partitioner</see><c>&amp;</c>,
///     <c>const</c> <see ref="static_partitioner Class">static_partitioner</see><c>&amp;</c>,
///     <c>const</c> <see ref="simple_partitioner Class">simple_partitioner</see><c>&amp;</c> or
///     <see ref="affinity_partitioner Class">affinity_partitioner</see><c>&amp;</c>
///     If an <see ref="affinity_partitioner Class">affinity_partitioner</see> object is used, the reference must be a non-const l-value reference,
///     so that the algorithm can store state for future loops to re-use.
/// </param>
/// <returns>
///     An output iterator addressing the position one past the final element in the destination range that is receiving the output elements
///     transformed by the function object.
/// </returns>
/// <remarks>
///     <see ref="auto_partitioner Class">auto_partitioner</see> will be used for the overloads without an explicit partitioner argument.
///     <para>For iterators that do not support random access, only <see ref="auto_partitioner Class">auto_partitioner</see> is supported.</para>
///     <para>The overloads that take the argument <paramref name="_Unary_op"/> transform the input range into the output range by applying
///     the unary functor to each element in the input range. <paramref name="_Unary_op"/> must support the function call operator with signature
///     <c>operator()(T)</c> where <c>T</c> is the value type of the range being iterated over.</para>
///     <para>The overloads that take the argument <paramref name="_Binary_op"/> transform two input ranges into the output range by applying the
///     binary functor to one element from the first input range and one element from the second input range. <paramref name="_Binary_op"/> must support
///     the function call operator with signature <c>operator()(T, U)</c> where <c>T</c>, <c>U</c> are value types of the two input iterators.</para>
///     <para>For more information, see <see cref="Parallel Algorithms"/>.</para>
/// </remarks>
/**/
template <typename _Input_iterator1, typename _Output_iterator, typename _Unary_operator>
_Output_iterator parallel_transform(_Input_iterator1 _First1, _Input_iterator1 _Last1, _Output_iterator _Result, const _Unary_operator& _Unary_op, const static_partitioner& _Part)
{
    return _Parallel_transform_unary_impl(_First1, _Last1, _Result, _Unary_op, _Part);
}

/// <summary>
///     Applies a specified function object to each element in a source range, or to a pair of elements from two source ranges,
///     and copies the return values of the function object into a destination range, in parallel. This functional is semantically
///     equivalent to <c>std::transform</c>.
/// </summary>
/// <typeparam name="_Input_iterator1">
///     The type of the first or only input iterator.
/// </typeparam>
/// <typeparam name="_Output_iterator">
///     The type of the output iterator.
/// </typeparam>
/// <typeparam name="_Unary_operator">
///     The type of the unary functor to be executed on each element in the input range.
/// </typeparam>
/// <param name="_First1">
///     An input iterator addressing the position of the first element in the first or only source range to be operated on.
/// </param>
/// <param name="_Last1">
///     An input iterator addressing the position one past the final element in the first or only source range to be operated on.
/// </param>
/// <param name="_Result">
///     An output iterator addressing the position of the first element in the destination range.
/// </param>
/// <param name="_Unary_op">
///     A user-defined unary function object that is applied to each element in the source range.
/// </param>
/// <param name="_Part">
///     A reference to the partitioner object. The argument can be one of
///     <c>const</c> <see ref="auto_partitioner Class">auto_partitioner</see><c>&amp;</c>,
///     <c>const</c> <see ref="static_partitioner Class">static_partitioner</see><c>&amp;</c>,
///     <c>const</c> <see ref="simple_partitioner Class">simple_partitioner</see><c>&amp;</c> or
///     <see ref="affinity_partitioner Class">affinity_partitioner</see><c>&amp;</c>
///     If an <see ref="affinity_partitioner Class">affinity_partitioner</see> object is used, the reference must be a non-const l-value reference,
///     so that the algorithm can store state for future loops to re-use.
/// </param>
/// <returns>
///     An output iterator addressing the position one past the final element in the destination range that is receiving the output elements
///     transformed by the function object.
/// </returns>
/// <remarks>
///     <see ref="auto_partitioner Class">auto_partitioner</see> will be used for the overloads without an explicit partitioner argument.
///     <para>For iterators that do not support random access, only <see ref="auto_partitioner Class">auto_partitioner</see> is supported.</para>
///     <para>The overloads that take the argument <paramref name="_Unary_op"/> transform the input range into the output range by applying
///     the unary functor to each element in the input range. <paramref name="_Unary_op"/> must support the function call operator with signature
///     <c>operator()(T)</c> where <c>T</c> is the value type of the range being iterated over.</para>
///     <para>The overloads that take the argument <paramref name="_Binary_op"/> transform two input ranges into the output range by applying the
///     binary functor to one element from the first input range and one element from the second input range. <paramref name="_Binary_op"/> must support
///     the function call operator with signature <c>operator()(T, U)</c> where <c>T</c>, <c>U</c> are value types of the two input iterators.</para>
///     <para>For more information, see <see cref="Parallel Algorithms"/>.</para>
/// </remarks>
/**/
template <typename _Input_iterator1, typename _Output_iterator, typename _Unary_operator>
_Output_iterator parallel_transform(_Input_iterator1 _First1, _Input_iterator1 _Last1, _Output_iterator _Result, const _Unary_operator& _Unary_op, const simple_partitioner& _Part)
{
    return _Parallel_transform_unary_impl(_First1, _Last1, _Result, _Unary_op, _Part);
}

/// <summary>
///     Applies a specified function object to each element in a source range, or to a pair of elements from two source ranges,
///     and copies the return values of the function object into a destination range, in parallel. This functional is semantically
///     equivalent to <c>std::transform</c>.
/// </summary>
/// <typeparam name="_Input_iterator1">
///     The type of the first or only input iterator.
/// </typeparam>
/// <typeparam name="_Output_iterator">
///     The type of the output iterator.
/// </typeparam>
/// <typeparam name="_Unary_operator">
///     The type of the unary functor to be executed on each element in the input range.
/// </typeparam>
/// <param name="_First1">
///     An input iterator addressing the position of the first element in the first or only source range to be operated on.
/// </param>
/// <param name="_Last1">
///     An input iterator addressing the position one past the final element in the first or only source range to be operated on.
/// </param>
/// <param name="_Result">
///     An output iterator addressing the position of the first element in the destination range.
/// </param>
/// <param name="_Unary_op">
///     A user-defined unary function object that is applied to each element in the source range.
/// </param>
/// <param name="_Part">
///     A reference to the partitioner object. The argument can be one of
///     <c>const</c> <see ref="auto_partitioner Class">auto_partitioner</see><c>&amp;</c>,
///     <c>const</c> <see ref="static_partitioner Class">static_partitioner</see><c>&amp;</c>,
///     <c>const</c> <see ref="simple_partitioner Class">simple_partitioner</see><c>&amp;</c> or
///     <see ref="affinity_partitioner Class">affinity_partitioner</see><c>&amp;</c>
///     If an <see ref="affinity_partitioner Class">affinity_partitioner</see> object is used, the reference must be a non-const l-value reference,
///     so that the algorithm can store state for future loops to re-use.
/// </param>
/// <returns>
///     An output iterator addressing the position one past the final element in the destination range that is receiving the output elements
///     transformed by the function object.
/// </returns>
/// <remarks>
///     <see ref="auto_partitioner Class">auto_partitioner</see> will be used for the overloads without an explicit partitioner argument.
///     <para>For iterators that do not support random access, only <see ref="auto_partitioner Class">auto_partitioner</see> is supported.</para>
///     <para>The overloads that take the argument <paramref name="_Unary_op"/> transform the input range into the output range by applying
///     the unary functor to each element in the input range. <paramref name="_Unary_op"/> must support the function call operator with signature
///     <c>operator()(T)</c> where <c>T</c> is the value type of the range being iterated over.</para>
///     <para>The overloads that take the argument <paramref name="_Binary_op"/> transform two input ranges into the output range by applying the
///     binary functor to one element from the first input range and one element from the second input range. <paramref name="_Binary_op"/> must support
///     the function call operator with signature <c>operator()(T, U)</c> where <c>T</c>, <c>U</c> are value types of the two input iterators.</para>
///     <para>For more information, see <see cref="Parallel Algorithms"/>.</para>
/// </remarks>
/**/
template <typename _Input_iterator1, typename _Output_iterator, typename _Unary_operator>
_Output_iterator parallel_transform(_Input_iterator1 _First1, _Input_iterator1 _Last1, _Output_iterator _Result, const _Unary_operator& _Unary_op, affinity_partitioner& _Part)
{
    return _Parallel_transform_unary_impl(_First1, _Last1, _Result, _Unary_op, _Part);
}

/// <summary>
///     Applies a specified function object to each element in a source range, or to a pair of elements from two source ranges,
///     and copies the return values of the function object into a destination range, in parallel. This functional is semantically
///     equivalent to <c>std::transform</c>.
/// </summary>
/// <typeparam name="_Input_iterator1">
///     The type of the first or only input iterator.
/// </typeparam>
/// <typeparam name="_Input_iterator2">
///     The type of second input iterator.
/// </typeparam>
/// <typeparam name="_Output_iterator">
///     The type of the output iterator.
/// </typeparam>
/// <typeparam name="_Binary_operator">
///     The type of the binary functor executed pairwise on elements from the two source ranges.
/// </typeparam>
/// <param name="_First1">
///     An input iterator addressing the position of the first element in the first or only source range to be operated on.
/// </param>
/// <param name="_Last1">
///     An input iterator addressing the position one past the final element in the first or only source range to be operated on.
/// </param>
/// <param name="_First2">
///     An input iterator addressing the position of the first element in the second source range to be operated on.
/// </param>
/// <param name="_Result">
///     An output iterator addressing the position of the first element in the destination range.
/// </param>
/// <param name="_Binary_op">
///     A user-defined binary function object that is applied pairwise, in a forward order, to the two source ranges.
/// </param>
/// <param name="_Part">
///     A reference to the partitioner object. The argument can be one of
///     <c>const</c> <see ref="auto_partitioner Class">auto_partitioner</see><c>&amp;</c>,
///     <c>const</c> <see ref="static_partitioner Class">static_partitioner</see><c>&amp;</c>,
///     <c>const</c> <see ref="simple_partitioner Class">simple_partitioner</see><c>&amp;</c> or
///     <see ref="affinity_partitioner Class">affinity_partitioner</see><c>&amp;</c>
///     If an <see ref="affinity_partitioner Class">affinity_partitioner</see> object is used, the reference must be a non-const l-value reference,
///     so that the algorithm can store state for future loops to re-use.
/// </param>
/// <returns>
///     An output iterator addressing the position one past the final element in the destination range that is receiving the output elements
///     transformed by the function object.
/// </returns>
/// <remarks>
///     <see ref="auto_partitioner Class">auto_partitioner</see> will be used for the overloads without an explicit partitioner argument.
///     <para>For iterators that do not support random access, only <see ref="auto_partitioner Class">auto_partitioner</see> is supported.</para>
///     <para>The overloads that take the argument <paramref name="_Unary_op"/> transform the input range into the output range by applying
///     the unary functor to each element in the input range. <paramref name="_Unary_op"/> must support the function call operator with signature
///     <c>operator()(T)</c> where <c>T</c> is the value type of the range being iterated over.</para>
///     <para>The overloads that take the argument <paramref name="_Binary_op"/> transform two input ranges into the output range by applying the
///     binary functor to one element from the first input range and one element from the second input range. <paramref name="_Binary_op"/> must support
///     the function call operator with signature <c>operator()(T, U)</c> where <c>T</c>, <c>U</c> are value types of the two input iterators.</para>
///     <para>For more information, see <see cref="Parallel Algorithms"/>.</para>
/// </remarks>
/**/
template <typename _Input_iterator1, typename _Input_iterator2, typename _Output_iterator, typename _Binary_operator, typename _Partitioner>
_Output_iterator parallel_transform(_Input_iterator1 _First1, _Input_iterator1 _Last1, _Input_iterator2 _First2,
    _Output_iterator _Result, const _Binary_operator& _Binary_op, _Partitioner&& _Part)
{
    typedef typename ::std::iterator_traits<_Input_iterator1>::iterator_category _Input_iterator_type1;
    typedef typename ::std::iterator_traits<_Input_iterator2>::iterator_category _Input_iterator_type2;
    typedef typename ::std::iterator_traits<_Output_iterator>::iterator_category _Output_iterator_type;

    if (_First1 != _Last1)
    {
        _Binary_transform_impl_helper<_Input_iterator_type1, _Input_iterator_type2, _Output_iterator_type>
            ::_Parallel_transform_binary_impl(_First1, _Last1, _First2, _Result, _Binary_op, ::std::forward<_Partitioner>(_Part));
    }

    return _Result;
}

/// <summary>
///     Applies a specified function object to each element in a source range, or to a pair of elements from two source ranges,
///     and copies the return values of the function object into a destination range, in parallel. This functional is semantically
///     equivalent to <c>std::transform</c>.
/// </summary>
/// <typeparam name="_Input_iterator1">
///     The type of the first or only input iterator.
/// </typeparam>
/// <typeparam name="_Input_iterator2">
///     The type of second input iterator.
/// </typeparam>
/// <typeparam name="_Output_iterator">
///     The type of the output iterator.
/// </typeparam>
/// <typeparam name="_Binary_operator">
///     The type of the binary functor executed pairwise on elements from the two source ranges.
/// </typeparam>
/// <param name="_First1">
///     An input iterator addressing the position of the first element in the first or only source range to be operated on.
/// </param>
/// <param name="_Last1">
///     An input iterator addressing the position one past the final element in the first or only source range to be operated on.
/// </param>
/// <param name="_First2">
///     An input iterator addressing the position of the first element in the second source range to be operated on.
/// </param>
/// <param name="_Result">
///     An output iterator addressing the position of the first element in the destination range.
/// </param>
/// <param name="_Binary_op">
///     A user-defined binary function object that is applied pairwise, in a forward order, to the two source ranges.
/// </param>
/// <returns>
///     An output iterator addressing the position one past the final element in the destination range that is receiving the output elements
///     transformed by the function object.
/// </returns>
/// <remarks>
///     <see ref="auto_partitioner Class">auto_partitioner</see> will be used for the overloads without an explicit partitioner argument.
///     <para>For iterators that do not support random access, only <see ref="auto_partitioner Class">auto_partitioner</see> is supported.</para>
///     <para>The overloads that take the argument <paramref name="_Unary_op"/> transform the input range into the output range by applying
///     the unary functor to each element in the input range. <paramref name="_Unary_op"/> must support the function call operator with signature
///     <c>operator()(T)</c> where <c>T</c> is the value type of the range being iterated over.</para>
///     <para>The overloads that take the argument <paramref name="_Binary_op"/> transform two input ranges into the output range by applying the
///     binary functor to one element from the first input range and one element from the second input range. <paramref name="_Binary_op"/> must support
///     the function call operator with signature <c>operator()(T, U)</c> where <c>T</c>, <c>U</c> are value types of the two input iterators.</para>
///     <para>For more information, see <see cref="Parallel Algorithms"/>.</para>
/// </remarks>
/**/
template <typename _Input_iterator1, typename _Input_iterator2, typename _Output_iterator, typename _Binary_operator>
_Output_iterator parallel_transform(_Input_iterator1 _First1, _Input_iterator1 _Last1, _Input_iterator2 _First2,
    _Output_iterator _Result, const _Binary_operator& _Binary_op)
{
    return parallel_transform(_First1, _Last1, _First2, _Result, _Binary_op, auto_partitioner());
}

#pragma warning(pop)

#pragma warning(push)
// object allocated on the heap may not be aligned 64
#pragma warning(disable: 4316)

/// <summary>
///     The <c>combinable&lt;T&gt;</c> object is intended to provide thread-private copies of data, to perform lock-free
///     thread-local sub-computations during parallel algorithms. At the end of the parallel operation, the
///     thread-private sub-computations can then be merged into a final result. This class can be used instead of
///     a shared variable, and can result in a performance improvement if there would otherwise be a lot of
///     contention on that shared variable.
/// </summary>
/// <typeparam name="_Ty">
///     The data type of the final merged result. The type must have a copy constructor and a default constructor.
/// </typeparam>
/// <remarks>
///     For more information, see <see cref="Parallel Containers and Objects"/>.
/// </remarks>
/**/
template<typename _Ty>
class combinable
{
private:

// Disable warning C4324: structure was padded due to __declspec(align())
// This padding is expected and necessary.
#pragma warning(push)
#pragma warning(disable: 4324)
    __declspec(align(64))
    struct _Node
    {
        unsigned long _M_key;
        _Ty _M_value;
        _Node* _M_chain;

        _Node(unsigned long _Key, _Ty _InitialValue)
            : _M_key(_Key),
            _M_value(::std::move(_InitialValue)),
            _M_chain(nullptr)
        {
        }
    };
#pragma warning(pop)

    static _Ty _DefaultInit()
    {
        return _Ty();
    }

public:
    /// <summary>
    ///     Constructs a new <c>combinable</c> object.
    /// </summary>
    /// <remarks>
    ///     <para>The first constructor initializes new elements with the default constructor for the type <paramref name="_Ty"/>.</para>
    ///     <para>The second constructor initializes new elements using the initialization functor supplied as the
    ///           <paramref name="_FnInitialize"/> parameter.</para>
    ///     <para>The third constructor is the copy constructor.</para>
    /// </remarks>
    /// <seealso cref="Parallel Containers and Objects"/>
    /**/
    combinable()
        : _M_buckets(),
        _M_size(),
        _M_fnInitialize(_DefaultInit)
    {
        _InitNew();
    }

    /// <summary>
    ///     Constructs a new <c>combinable</c> object.
    /// </summary>
    /// <typeparam name="_Function">
    ///     The type of the initialization functor object.
    /// </typeparam>
    /// <param name="_FnInitialize">
    ///     A function which will be called to initialize each new thread-private value of the type <paramref name="_Ty"/>.
    ///     It must support a function call operator with the signature <c>_Ty ()</c>.
    /// </param>
    /// <remarks>
    ///     <para>The first constructor initializes new elements with the default constructor for the type <paramref name="_Ty"/>.</para>
    ///     <para>The second constructor initializes new elements using the initialization functor supplied as the
    ///           <paramref name="_FnInitialize"/> parameter.</para>
    ///     <para>The third constructor is the copy constructor.</para>
    /// </remarks>
    /// <seealso cref="Parallel Containers and Objects"/>
    /**/
    template <typename _Function>
    explicit combinable(_Function _FnInitialize)
        : _M_buckets(),
        _M_size(),
        _M_fnInitialize(::std::move(_FnInitialize))
    {
        _InitNew();
    }

    /// <summary>
    ///     Constructs a new <c>combinable</c> object.
    /// </summary>
    /// <param name="_Other">
    ///     An existing <c>combinable</c> object to be copied into this one.
    /// </param>
    /// <remarks>
    ///     <para>The first constructor initializes new elements with the default constructor for the type <paramref name="_Ty"/>.</para>
    ///     <para>The second constructor initializes new elements using the initialization functor supplied as the
    ///           <paramref name="_FnInitialize"/> parameter.</para>
    ///     <para>The third constructor is the copy constructor.</para>
    /// </remarks>
    /// <seealso cref="Parallel Containers and Objects"/>
    /**/
    combinable(const combinable& _Other)
        : _M_buckets(),
        _M_size(_Other._M_size),
        _M_fnInitialize(_Other._M_fnInitialize) // throws
    {
        _M_buckets = _InitCopy(_Other);
    }

    /// <summary>
    ///     Assigns to a <c>combinable</c> object from another <c>combinable</c> object.
    /// </summary>
    /// <param name="_Other">
    ///     An existing <c>combinable</c> object to be copied into this one.
    /// </param>
    /// <returns>
    ///     A reference to this <c>combinable</c> object.
    /// </returns>
    /**/
    combinable& operator=(const combinable& _Other)
    {
        auto _Fn_initialize_copy = _Other._M_fnInitialize; // throws
        auto _New_buckets = _InitCopy(_Other); // throws
        // remaining ops cannot throw
        clear();
        delete [] _M_buckets;
        _M_buckets = _New_buckets;
        _M_fnInitialize.swap(_Fn_initialize_copy);
        _M_size = _Other._M_size;

        return *this;
    }

    /// <summary>
    ///     Destroys a <c>combinable</c> object.
    /// </summary>
    /**/
    ~combinable()
    {
        clear();
        delete [] _M_buckets;
    }

    /// <summary>
    ///     Returns a reference to the thread-private sub-computation.
    /// </summary>
    /// <returns>
    ///     A reference to the thread-private sub-computation.
    /// </returns>
    /// <seealso cref="Parallel Containers and Objects"/>
    /**/
    _Ty& local()
    {
        unsigned long _Key = ::Concurrency::details::platform::GetCurrentThreadId();
        size_t _Index;
        _Node* _ExistingNode = _FindLocalItem(_Key, &_Index);
        if (_ExistingNode == nullptr)
        {
            _ExistingNode = _AddLocalItem(_Key, _Index);
        }

        _CONCRT_ASSERT(_ExistingNode != nullptr);
        return _ExistingNode->_M_value;
    }

    /// <summary>
    ///     Returns a reference to the thread-private sub-computation.
    /// </summary>
    /// <param name="_Exists">
    ///     A reference to a boolean. The boolean value referenced by this argument will be
    ///     set to <c>true</c> if the sub-computation already existed on this thread, and set to
    ///     <c>false</c> if this was the first sub-computation on this thread.
    /// </param>
    /// <returns>
    ///     A reference to the thread-private sub-computation.
    /// </returns>
    /// <seealso cref="Parallel Containers and Objects"/>
    /**/
    _Ty& local(bool& _Exists)
    {
        unsigned long _Key = ::Concurrency::details::platform::GetCurrentThreadId();
        size_t _Index;
        _Node* _ExistingNode = _FindLocalItem(_Key, &_Index);
        if (_ExistingNode == nullptr)
        {
            _Exists = false;
            _ExistingNode = _AddLocalItem(_Key, _Index);
        }
        else
        {
            _Exists = true;
        }

        _CONCRT_ASSERT(_ExistingNode != nullptr);
        return _ExistingNode->_M_value;
    }

    /// <summary>
    ///     Clears any intermediate computational results from a previous usage.
    /// </summary>
    /**/
    void clear()
    {
        for (size_t _Index = 0; _Index < _M_size; ++_Index)
        {
            _Node* _CurrentNode = _M_buckets[_Index];
            while (_CurrentNode != nullptr)
            {
                _Node* _NextNode = _CurrentNode->_M_chain;
                delete _CurrentNode;
                _CurrentNode = _NextNode;
            }
        }
        memset((void*)_M_buckets, 0, _M_size * sizeof _M_buckets[0]);
    }

    /// <summary>
    ///     Computes a final value from the set of thread-local sub-computations by calling the supplied combine functor.
    /// </summary>
    /// <typeparam name="_Function">
    ///     The type of the function object that will be invoked to combine two thread-local sub-computations.
    /// </typeparam>
    /// <param name="_FnCombine">
    ///     The functor that is used to combine the sub-computations. Its signature is <c>T (T, T)</c> or
    ///     <c>T (const T&amp;, const T&amp;)</c>, and it must be associative and commutative.
    /// </param>
    /// <returns>
    ///     The final result of combining all the thread-private sub-computations.
    /// </returns>
    /// <seealso cref="Parallel Containers and Objects"/>
    /**/
    template<typename _Function>
    _Ty combine(_Function _FnCombine) const
    {
        _Node* _CurrentNode = nullptr;
        size_t _Index;

        // Look for the first value in the set, and use (a copy of) that as the result.
        // This eliminates a single call (of unknown cost) to _M_fnInitialize.
        for (_Index = 0; _Index < _M_size; ++_Index)
        {
            _CurrentNode = _M_buckets[_Index];
            if (_CurrentNode != nullptr)
            {
                 break;
            }
        }

        // No values... return the initializer value.
        if (_CurrentNode == nullptr)
        {
            return _M_fnInitialize();
        }

        // Accumulate the rest of the items in the current bucket.
        _Ty _Result = _CurrentNode->_M_value;
        for (_CurrentNode = _CurrentNode->_M_chain; _CurrentNode != nullptr; _CurrentNode = _CurrentNode->_M_chain)
        {
            _Result = _FnCombine(_Result, _CurrentNode->_M_value);
        }

        // Accumulate values from the rest of the buckets.
        _CONCRT_ASSERT(_Index < _M_size);
        for (++_Index; _Index < _M_size; ++_Index)
        {
            for (_CurrentNode = _M_buckets[_Index]; _CurrentNode != nullptr; _CurrentNode = _CurrentNode->_M_chain)
            {
                _Result = _FnCombine(_Result, _CurrentNode->_M_value);
            }
        }

        return _Result;
    }

    /// <summary>
    ///     Computes a final value from the set of thread-local sub-computations by calling the supplied combine functor
    ///     once per thread-local sub-computation. The final result is accumulated by the function object.
    /// </summary>
    /// <typeparam name="_Function">
    ///     The type of the function object that will be invoked to combine a single thread-local sub-computation.
    /// </typeparam>
    /// <param name="_FnCombine">
    ///     The functor that is used to combine one sub-computation. Its signature is <c>void (T)</c> or
    ///     <c>void (const T&amp;)</c>, and must be associative and commutative.
    /// </param>
    /// <seealso cref="Parallel Containers and Objects"/>
    /**/
    template<typename _Function>
    void combine_each(_Function _FnCombine) const
    {
        for (size_t _Index = 0; _Index < _M_size; ++_Index)
        {
            for (_Node* _CurrentNode = _M_buckets[_Index]; _CurrentNode != nullptr; _CurrentNode = _CurrentNode->_M_chain)
            {
                _FnCombine(_CurrentNode->_M_value);
            }
        }
    }

private:
    void _InitNew()
    {
        _M_size = ::Concurrency::details::_GetCombinableSize();
        _M_buckets = new _Node*[_M_size]{};
    }

    struct _InitCopyOp
    {
        ::std::unique_ptr<_Node*[]> _M_new_buckets;
        size_t _M_index; // invariant: !_M_new_buckets || _M_index < _Size

        explicit _InitCopyOp(size_t _Size)
            : _M_new_buckets(),
            _M_index(0)
        {
            if (_Size != 0)
            {
                _M_new_buckets = ::std::make_unique<_Node*[]>(_Size);
            }
        }

        _Node ** _DoCopy(size_t _Size, const combinable& _Other)
        {
            for (; _M_index < _Size; ++_M_index)
            {
                for (_Node* _CurrentNode = _Other._M_buckets[_M_index]; _CurrentNode != nullptr;
                    _CurrentNode = _CurrentNode->_M_chain)
                {
                    // allocate node and push_front
                    _Node* _NewNode = new _Node(_CurrentNode->_M_key, _CurrentNode->_M_value);
                    _NewNode->_M_chain = _M_new_buckets[_M_index];
                    _M_new_buckets[_M_index] = _NewNode;
                }
            }

            return _M_new_buckets.release(); // also muzzles destructor
        }

        ~_InitCopyOp()
        {
            if (_M_new_buckets)
            {
                // if we get here, an exception was thrown in _DoCopy; note we must back out including the
                // _M_index-th entry (where the exception was thrown), hence <=
                for (size_t _Next = 0; _Next <= _M_index; ++_Next)
                {
                    _Node* _CurrentNode = _M_new_buckets[_Next];
                    while (_CurrentNode)
                    {
                        const auto _NextNode = _CurrentNode->_M_chain;
                        delete _CurrentNode;
                        _CurrentNode = _NextNode;
                    }
                }
            }
        }
    };

    static _Node ** _InitCopy(const combinable& _Other)
    {
        _InitCopyOp _Op{_Other._M_size};
        return _Op._DoCopy(_Other._M_size, _Other);
    }

    _Node* _FindLocalItem(unsigned long _Key, size_t* _PIndex)
    {
        _CONCRT_ASSERT(_PIndex != nullptr);

        *_PIndex = _Key % _M_size;

        // Search at this index for an existing value.
        for (_Node* _CurrentNode = _M_buckets[*_PIndex]; _CurrentNode != nullptr; _CurrentNode = _CurrentNode->_M_chain)
        {
            if (_CurrentNode->_M_key == _Key)
            {
                return _CurrentNode;
            }
        }

        return nullptr;
    }

    _Node* _AddLocalItem(unsigned long _Key, size_t _Index)
    {
        _Node* _NewNode = new _Node(_Key, _M_fnInitialize());
        _Node* _TopNode;
        do
        {
            _TopNode = _M_buckets[_Index];
            _NewNode->_M_chain = _TopNode;
        } while (_InterlockedCompareExchangePointer(reinterpret_cast<void * volatile *>(&_M_buckets[_Index]), _NewNode, _TopNode) != _TopNode);

        return _NewNode;
    }

private:
    _Node *volatile * _M_buckets;
    size_t _M_size;
    ::std::function<_Ty ()> _M_fnInitialize;
};

#pragma warning(pop) // C4316

#pragma push_macro("_MAX_NUM_TASKS_PER_CORE")
#pragma push_macro("_FINE_GRAIN_CHUNK_SIZE")
#pragma push_macro("_SORT_MAX_RECURSION_DEPTH")

// This number is used to control dynamic task splitting
// The ideal chunk (task) division is that the number of cores is equal to the number of tasks, but it will
// perform very poorly when tasks are not balanced. The simple solution is to allocate more tasks than number
// of cores. _MAX_NUM_TASKS_PER_CORE provides a maximum number of tasks that will be allocated per core.
// If this number is too small, the load balancing problem will affect efficiency very seriously, especially
// when the compare operation is expensive.
//
// Note that this number is a maximum number -- the dynamic partition system will reduce the number of partitions
// per core based on the dynamic load. If all cores are very busy, the number of partitions will shrink to
// reduce the scheduler overhead.
//
// Initially, the total tasks(chunks) number of partitions "_Div_num" will be: core number * _MAX_NUM_TASKS_PER_CORE.
// The _Div_num will be divided by 2 after each task splitting. There are two special numbers for _Div_num:
//     1. When _Div_num reaches the point that _Div_num < _MAX_NUM_TASKS_PER_CORE, it means we have split more tasks than cores.
//     2. When _Div_num reaches the point that _Div_num <= 1, it means stop splitting more tasks and begin sorting serially.
#define _MAX_NUM_TASKS_PER_CORE 1024

// This is a number mainly is used to control the sampling and dynamic task splitting strategies.
// If the user configurable minimal divisible chunk size (default is 2048) is smaller than FINE_GRAIN_CHUNK_SIZE,
// the random sampling algorithm for quicksort will enter fine-grained mode, and take a strategy that reduces the sampling
// overhead. Also, the dynamic task splitting will enter fine-grained mode, which will split as many tasks as possible.
#define _FINE_GRAIN_CHUNK_SIZE 512

// This is the maximum depth that the quicksort will be called recursively. If we allow too far, a stack overflow may occur.
#define _SORT_MAX_RECURSION_DEPTH 64

template<typename _Random_iterator, typename _Function>
inline size_t _Median_of_three(const _Random_iterator &_Begin, size_t _A, size_t _B, size_t _C, const _Function &_Func, bool &_Potentially_equal)
{
    _Potentially_equal = false;
    if (_Func(_Begin[_A], _Begin[_B]))
    {
        if (_Func(_Begin[_A], _Begin[_C]))
        {
            return _Func(_Begin[_B], _Begin[_C]) ? _B : _C;
        }
        else
        {
            return _A;
        }
    }
    else
    {
        if (_Func(_Begin[_B], _Begin[_C]))
        {
            return _Func(_Begin[_A], _Begin[_C]) ? _A : _C;
        }
        else
        {
            _Potentially_equal = true;
            return _B;
        }
    }
}

template<typename _Random_iterator, typename _Function>
inline size_t _Median_of_nine(const _Random_iterator &_Begin, size_t _Size, const _Function &_Func, bool &_Potentially_equal)
{
    size_t _Offset = _Size / 8;
    size_t _A = _Median_of_three(_Begin, 0, _Offset, _Offset * 2, _Func, _Potentially_equal),
        _B = _Median_of_three(_Begin, _Offset * 3, _Offset * 4, _Offset * 5, _Func, _Potentially_equal),
        _C = _Median_of_three(_Begin, _Offset * 6, _Offset * 7, _Size - 1, _Func, _Potentially_equal);
    _B = _Median_of_three(_Begin, _A, _B, _C, _Func, _Potentially_equal);

    if (_Potentially_equal)
    {
        _Potentially_equal = !_Func(_Begin[_C], _Begin[_A]);
    }

    return _B;
}

// _Potentially_equal means that potentially all the values in the buffer are equal to the pivot value
template<typename _Random_iterator, typename _Function>
inline size_t _Select_median_pivot(const _Random_iterator &_Begin, size_t _Size, const _Function &_Func, const size_t _Chunk_size, bool &_Potentially_equal)
{
    // Base on different chunk size, apply different sampling optimization
    if (_Chunk_size < _FINE_GRAIN_CHUNK_SIZE && _Size <= ::std::max<size_t>(_Chunk_size * 4, static_cast<size_t>(15)))
    {
        bool _Never_care_equal;
        return _Median_of_three(_Begin, 0, _Size / 2, _Size - 1, _Func, _Never_care_equal);
    }
    else
    {
        return _Median_of_nine(_Begin, _Size, _Func, _Potentially_equal);
    }
}

// Find out two middle points for two sorted arrays by binary search so that the number of total elements on the left part of two middle points is equal
// to the number of total elements on the right part of two sorted arrays and all elements on the left part is smaller than right part.
template<typename _Random_iterator, typename _Random_buffer_iterator, typename _Function>
size_t _Search_mid_point(const _Random_iterator &_Begin1, size_t &_Len1, const _Random_buffer_iterator &_Begin2, size_t &_Len2, const _Function &_Func)
{
    size_t _Len = (_Len1 + _Len2) / 2, _Index1 = 0, _Index2 = 0;

    while (_Index1 < _Len1 && _Index2 < _Len2)
    {
        size_t _Mid1 = (_Index1 + _Len1) / 2, _Mid2 = (_Index2 + _Len2) / 2;
        if (_Func(_Begin1[_Mid1], _Begin2[_Mid2]))
        {
            if (_Mid1 + _Mid2 < _Len)
            {
                _Index1 = _Mid1 + 1;
            }
            else
            {
                _Len2 = _Mid2;
            }
        }
        else
        {
            if (_Mid1 + _Mid2 < _Len)
            {
                _Index2 = _Mid2 + 1;
            }
            else
            {
                _Len1 = _Mid1;
            }
        }
    }

    if (_Index1 == _Len1)
    {
        _Len2 = _Len - _Len1;
    }
    else
    {
        _Len1 = _Len - _Len2;
    }

    return _Len;
}

// "move" operation is applied between buffers
template<typename _Random_iterator, typename _Random_buffer_iterator, typename _Random_output_iterator, typename _Function>
void _Merge_chunks(_Random_iterator _Begin1, const _Random_iterator &_End1, _Random_buffer_iterator _Begin2, const _Random_buffer_iterator &_End2,
    _Random_output_iterator _Output, const _Function &_Func)
{
    while (_Begin1 != _End1 && _Begin2 != _End2)
    {
        if (_Func(*_Begin1, *_Begin2))
        {
            *_Output++ = ::std::move(*_Begin1++);
        }
        else
        {
            *_Output++ = ::std::move(*_Begin2++);
        }
    }

    if (_Begin1 != _End1)
    {
        ::std::move(_Begin1, _End1, _Output);
    }
    else if (_Begin2 != _End2)
    {
        ::std::move(_Begin2, _End2, _Output);
    }
}

// _Div_num of threads(tasks) merge two chunks in parallel, _Div_num should be power of 2, if not, the largest power of 2 that is
// smaller than _Div_num will be used
template<typename _Random_iterator, typename _Random_buffer_iterator, typename _Random_output_iterator, typename _Function>
void _Parallel_merge(_Random_iterator _Begin1, size_t _Len1, _Random_buffer_iterator _Begin2, size_t _Len2, _Random_output_iterator _Output,
    const _Function &_Func, size_t _Div_num)
{
    // Turn to serial merge or continue splitting chunks base on "_Div_num"
    if (_Div_num <= 1 || (_Len1 <= 1 && _Len2 <= 1))
    {
        _Merge_chunks(_Begin1, _Begin1 + _Len1, _Begin2, _Begin2 + _Len2, _Output, _Func);
    }
    else
    {
        size_t _Mid_len1 = _Len1, _Mid_len2 = _Len2;
        size_t _Mid = _Search_mid_point(_Begin1, _Mid_len1, _Begin2, _Mid_len2, _Func);

        structured_task_group _Tg;
        auto _Handle = make_task([&]
        {
            _Parallel_merge(_Begin1, _Mid_len1, _Begin2, _Mid_len2, _Output, _Func, _Div_num / 2);
        });
        _Tg.run(_Handle);

        _Parallel_merge(_Begin1 + _Mid_len1, _Len1 - _Mid_len1, _Begin2 + _Mid_len2, _Len2 - _Mid_len2, _Output + _Mid, _Func, _Div_num / 2);

        _Tg.wait();
    }
}

// Return current sorting byte from key
template<typename _Ty, typename _Function>
inline size_t _Radix_key(const _Ty& _Val, size_t _Radix, _Function _Proj_func)
{
    return static_cast<size_t>(_Proj_func(_Val) >> static_cast<int>(8 * _Radix) & 255);
}

// One pass of radix sort
template<typename _Random_iterator, typename _Random_buffer_iterator, typename _Function>
void _Integer_radix_pass(const _Random_iterator &_Begin, size_t _Size, const _Random_buffer_iterator &_Output, size_t _Radix, _Function _Proj_func)
{
    if (!_Size)
    {
        return;
    }

    size_t _Pos[256] = {0};

    for (size_t _I = 0; _I < _Size; _I++)
    {
        ++_Pos[_Radix_key(_Begin[_I], _Radix, _Proj_func)];
    }

    for (size_t _I = 1; _I < 256; _I++)
    {
        _Pos[_I] += _Pos[_I - 1];
    }

    // _Size > 0
    for (size_t _I = _Size - 1; _I != 0; _I--)
    {
        _Output[--_Pos[_Radix_key(_Begin[_I], _Radix, _Proj_func)]] = ::std::move(_Begin[_I]);
    }

    _Output[--_Pos[_Radix_key(_Begin[0], _Radix, _Proj_func)]] = ::std::move(_Begin[0]);
}

// Serial least-significant-byte radix sort, it will sort base on last "_Radix" number of bytes
template<typename _Random_iterator, typename _Random_buffer_iterator, typename _Function>
void _Integer_radix_sort(const _Random_iterator &_Begin, size_t _Size, const _Random_buffer_iterator &_Output,
    size_t _Radix, _Function _Proj_func, size_t _Deep = 0)
{
    size_t _Cur_radix = 0;
    if (_Size == 0)
    {
        return;
    }

    while (_Cur_radix < _Radix)
    {
        _Integer_radix_pass(_Begin, _Size, _Output, _Cur_radix++, _Proj_func);
        _Integer_radix_pass(_Output, _Size, _Begin, _Cur_radix++, _Proj_func);
    }

    if (_Cur_radix == _Radix)
    {
        _Integer_radix_pass(_Begin, _Size, _Output, _Cur_radix++, _Proj_func);
    }

    // if odd round is passed, then move result back to input buffer
    if (_Deep + _Radix + 1 & 1)
    {
        if (_Radix + 1 & 1)
        {
            ::std::move(_Output, _Output + _Size, _Begin);
        }
        else
        {
            ::std::move(_Begin, _Begin + _Size, _Output);
        }
    }
}

// Parallel most-significant-byte _Radix sort.
// In the end, it will turn to serial least-significant-byte radix sort
template<typename _Random_iterator, typename _Random_buffer_iterator, typename _Function>
void _Parallel_integer_radix_sort(const _Random_iterator &_Begin, size_t _Size, const _Random_buffer_iterator &_Output,
    size_t _Radix, _Function _Proj_func, const size_t _Chunk_size, size_t _Deep = 0)
{
    // If the chunk _Size is too small, then turn to serial least-significant-byte radix sort
    if (_Size <= _Chunk_size || _Radix < 1)
    {
        return _Integer_radix_sort(_Begin, _Size, _Output, _Radix, _Proj_func, _Deep);
    }

    size_t _Threads_num = ::Concurrency::details::_CurrentScheduler::_GetNumberOfVirtualProcessors();
    size_t _Buffer_size = sizeof(size_t) * 256 * _Threads_num;
    size_t _Step = _Size / _Threads_num;
    size_t _Remain = _Size % _Threads_num;

    ::Concurrency::details::_MallocaArrayHolder<size_t> _Holder;
    using _Chunk_ptr_t = size_t (*)[256];
    _Chunk_ptr_t _Chunks = reinterpret_cast<_Chunk_ptr_t>(_Holder._InitOnRawMalloca(_malloca(_Buffer_size)));

    memset(_Chunks, 0, _Buffer_size);

    // Our purpose is to map unsorted data in buffer "_Begin" to buffer "_Output" so that all elements who have the same
    // byte value in the "_Radix" position will be grouped together in the buffer "_Output"
    //
    // Serial version:
    // To understand this algorithm, first consider a serial version. In following example, we treat 1 digit as 1 byte, so we have a
    // total of 10 elements for each digit instead of 256 elements in each byte. Let's suppose "_Radix" == 1 (right most is 0), and:
    //
    //      begin:  [ 32 | 62 | 21 | 43 | 55 | 43 | 23 | 44 ]
    //
    // We want to divide the output buffer "_Output" into 10 chunks, and each the element in the "_Begin" buffer should be mapped into
    // the proper destination chunk based on its current digit (byte) indicated by "_Radix"
    //
    // Because "_Radix" == 1, after a pass of this function, the chunks in the "_Output" should look like:
    //
    //      buffer: [   |   | 21 23 | 32 | 43 43 44 | 55 | 62 |   |   |   ]
    //                0   1     2      3      4        5    6   7   8   9
    //
    // The difficulty is determining where to insert values into the "_Output" to get the above result. The way to get the
    // start position of each chunk of the buffer is:
    //      1. Count the number of elements for each chunk (in above example, chunk0 is 0, chunk1 is 0, chunk2 is 2, chunk3 is 1 ...
    //      2. Make a partial sum for these chunks( in above example,  we will get chunk0=chunk0=0, chunk1=chunk0+chunk1=0,
    //         chunk2=chunk0+chunk1+chunk2=2, chunk3=chunk0+chunk1+chunk2+chunk3=3
    //
    // After these steps, we will get the end position of each chunk in the "_Output". The begin position of each chunk will be the end
    // point of last chunk (begin point is close but the end point is open). After that,  we can scan the original array again and directly
    // put elements from original buffer "_Begin" into specified chunk on buffer "_Output".
    // Finally, we invoke _parallel_integer_radix_sort in parallel for each chunk and sort them in parallel based on the next digit (byte).
    // Because this is a STABLE sort algorithm, if two numbers has same key value on this byte (digit), their original order should be kept.
    //
    // Parallel version:
    // Almost the same as the serial version, the differences are:
    //      1. The count for each chunk is executed in parallel, and each thread will count one segment of the input buffer "_Begin".
    //         The count result will be separately stored in their own chunk size counting arrays so we have a total of threads-number
    //         of chunk count arrays.
    //         For example, we may have chunk00, chunk01, ..., chunk09 for first thread, chunk10, chunk11, ..., chunk19 for second thread, ...
    //      2. The partial sum should be executed across these chunk counting arrays that belong to different threads, instead of just
    //         making a partial sum in one counting array.
    //         This is because we need to put values from different segments into one final buffer, and the absolute buffer position for
    //         each chunkXX is needed.
    //      3. Make a parallel scan for original buffer again, and move numbers in parallel into the corresponding chunk on each buffer based
    //         on these threads' chunk size counters.

    // Count in parallel and separately save their local results without reducing
    ::Concurrency::parallel_for(static_cast<size_t>(0), _Threads_num, [=](size_t _Index)
    {
        size_t _Beg_index, _End_index;

        // Calculate the segment position
        if (_Index < _Remain)
        {
            _Beg_index = _Index * (_Step + 1);
            _End_index = _Beg_index + (_Step + 1);
        }
        else
        {
            _Beg_index = _Remain * (_Step + 1) + (_Index - _Remain) * _Step;
            _End_index = _Beg_index + _Step;
        }

        // Do a counting
        while (_Beg_index != _End_index)
        {
            ++_Chunks[_Index][_Radix_key(_Begin[_Beg_index++], _Radix, _Proj_func)];
        }
    });

    int _Index = -1, _Count = 0;

    // Partial sum cross different threads' chunk counters
    for (int _I = 0; _I < 256; _I++)
    {
        size_t _Last = _I ? _Chunks[_Threads_num - 1][_I - 1] : 0;
        _Chunks[0][_I] += _Last;

        for (size_t _J = 1; _J < _Threads_num; _J++)
        {
            _Chunks[_J][_I] += _Chunks[_J - 1][_I];
        }

        // "_Chunks[_Threads_num - 1][_I] - _Last" will get the global _Size for chunk _I(including all threads local _Size for chunk _I)
        // this will chunk whether the chunk _I is empty or not. If it's not empty, it will be recorded.
        if (_Chunks[_Threads_num - 1][_I] - _Last)
        {
            ++_Count;
            _Index = _I;
        }
    }

    // If there is more than 1 chunk that has content, then continue the original algorithm
    if (_Count > 1)
    {
        // Move the elements in parallel into each chunk
        ::Concurrency::parallel_for(static_cast<size_t>(0), _Threads_num, [=](size_t _Index)
        {
            size_t _Beg_index, _End_index;

            // Calculate the segment position
            if (_Index < _Remain)
            {
                _Beg_index = _Index * (_Step + 1);
                _End_index = _Beg_index + (_Step + 1);
            }
            else
            {
                _Beg_index = _Remain * (_Step + 1) + (_Index - _Remain) * _Step;
                _End_index = _Beg_index + _Step;
            }

            // Do a move operation to directly put each value into its destination chunk
            // Chunk pointer is moved after each put operation.
            if (_Beg_index != _End_index--)
            {
                while (_Beg_index != _End_index)
                {
                    _Output[--_Chunks[_Index][_Radix_key(_Begin[_End_index], _Radix, _Proj_func)]] = ::std::move(_Begin[_End_index]);
                    --_End_index;
                }
                _Output[--_Chunks[_Index][_Radix_key(_Begin[_End_index], _Radix, _Proj_func)]] = ::std::move(_Begin[_End_index]);
            }
        });

        // Invoke _parallel_integer_radix_sort in parallel for each chunk
        ::Concurrency::parallel_for(static_cast<size_t>(0), static_cast<size_t>(256), [=](size_t _Index)
        {
            if (_Index < 256 - 1)
            {
                _Parallel_integer_radix_sort(_Output + _Chunks[0][_Index], _Chunks[0][_Index + 1] - _Chunks[0][_Index],
                    _Begin + _Chunks[0][_Index], _Radix - 1, _Proj_func, _Chunk_size, _Deep + 1);
            }
            else
            {
                _Parallel_integer_radix_sort(_Output + _Chunks[0][_Index], _Size - _Chunks[0][_Index],
                    _Begin + _Chunks[0][_Index], _Radix - 1, _Proj_func, _Chunk_size, _Deep + 1);
            }
        });
    }
    else
    {
        // Only one chunk has content
        // A special optimization is applied because one chunk means all numbers have a same value on this particular byte (digit).
        // Because we cannot sort them at all (they are all equal at this point), directly call _parallel_integer_radix_sort to
        // sort next byte (digit)
        _Parallel_integer_radix_sort(_Begin, _Size, _Output, _Radix - 1, _Proj_func, _Chunk_size, _Deep);
    }
}

template<typename _Random_iterator, typename _Random_buffer_iterator, typename _Function>
void _Parallel_integer_sort_asc(const _Random_iterator &_Begin, size_t _Size, const _Random_buffer_iterator &_Output,
    _Function _Proj_func, const size_t _Chunk_size)
{
    typedef typename ::std::iterator_traits<_Random_iterator>::value_type _Value_type;
    // The key type of the radix sort, this must be an "unsigned integer-like" type, that is, it needs support:
    //     operator>> (int), operator>>= (int), operator& (int), operator <, operator size_t ()
    typedef ::std::remove_const_t<::std::remove_reference_t<decltype(_Proj_func(*_Begin))>> _Integer_type;

    // Find out the max value, which will be used to determine the highest differing byte (the radix position)
    _Integer_type _Max_val = ::Concurrency::parallel_reduce(_Begin, _Begin + _Size, _Proj_func(*_Begin),
        [=](_Random_iterator _Begin, _Random_iterator _End, _Integer_type _Init) -> _Integer_type
        {
            while (_Begin != _End)
            {
                _Integer_type _Ret = _Proj_func(*_Begin++);
                if (_Init < _Ret)
                {
                    _Init = _Ret;
                }
            }

            return _Init;
        }, [](const _Integer_type &_A, const _Integer_type &_B) -> const _Integer_type& {return (_A < _B)? _B : _A;});
    size_t _Radix = 0;

    // Find out highest differing byte
    while (_Max_val >>= 8)
    {
        ++_Radix;
    }

    _Parallel_integer_radix_sort(_Begin, _Size, _Output, _Radix, _Proj_func, _Chunk_size);
}

template<typename _Random_iterator, typename _Function>
void _Parallel_quicksort_impl(const _Random_iterator &_Begin, size_t _Size, const _Function &_Func, size_t _Div_num, const size_t _Chunk_size, int _Depth)
{
    if (_Depth >= _SORT_MAX_RECURSION_DEPTH || _Size <= _Chunk_size || _Size <= static_cast<size_t>(3) || _Chunk_size >= _FINE_GRAIN_CHUNK_SIZE && _Div_num <= 1)
    {
        return ::std::sort(_Begin, _Begin + _Size, _Func);
    }

    // Determine whether we need to do a three-way quick sort
    // We benefit from three-way merge if there are a lot of elements that are EQUAL to the median value,
    // _Select_median_pivot function will test redundant density by sampling
    bool _Is_three_way_split = false;
    size_t _Mid_index = _Select_median_pivot(_Begin, _Size, _Func, _Chunk_size, _Is_three_way_split);

    // Move the median value to the _Begin position.
    if (_Mid_index)
    {
        ::std::swap(*_Begin, _Begin[_Mid_index]);
    }
    size_t _I = 1, _J = _Size - 1;

    // Three-way or two-way partition
    // _Div_num < _MAX_NUM_TASKS_PER_CORE is checked to make sure it will never do three-way split before splitting enough tasks
    if (_Is_three_way_split && _Div_num < _MAX_NUM_TASKS_PER_CORE)
    {
        while (_Func(*_Begin, _Begin[_J]))
        {
            --_J;
        }

        while (_Func(_Begin[_I], *_Begin))
        {
            ++_I;
        }

        // Starting from this point, left side of _I will less than median value, right side of _J will be greater than median value,
        // and the middle part will be equal to median. _K is used to scan between _I and _J
        size_t _K = _J;
        while (_I <= _K)
        {
            if (_Func(_Begin[_K], *_Begin))
            {
                ::std::swap(_Begin[_I++], _Begin[_K]);
            }
            else
            {
                --_K;
            }

            while (_Func(*_Begin, _Begin[_K]))
            {
                ::std::swap(_Begin[_K--], _Begin[_J--]);
            }
        }

        ++_J;
    }
    else
    {
        while (_I <= _J)
        {
            // Will stop before _Begin
            while (_Func(*_Begin, _Begin[_J]))
            {
                --_J;
            }

            // There must be another element equal or greater than *_Begin
            while (_Func(_Begin[_I], *_Begin))
            {
                ++_I;
            }

            if (_I < _J)
            {
                ::std::swap(_Begin[_I++], _Begin[_J--]);
            }
            else
            {
                break;
            }
        }

        _I = ++_J;
    }

    ::std::swap(*_Begin, _Begin[--_I]);

    structured_task_group _Tg;
    volatile size_t _Next_div = _Div_num / 2;
    auto _Handle = make_task([&]
    {
        _Parallel_quicksort_impl(_Begin + _J, _Size - _J, _Func, _Next_div, _Chunk_size, _Depth+1);
    });
    _Tg.run(_Handle);

    _Parallel_quicksort_impl(_Begin, _I, _Func, _Next_div, _Chunk_size, _Depth+1);

    // If at this point, the work hasn't been scheduled, then slow down creating new tasks
    if (_Div_num < _MAX_NUM_TASKS_PER_CORE)
    {
        _Next_div /= 2;
    }

    _Tg.wait();
}

// This function will be called to sort the elements in the "_Begin" buffer. However, we can't tell whether the result will end up in buffer
// "_Begin", or buffer "_Output" when it returned. The return value is designed to indicate which buffer holds the sorted result.
// Return true if the merge result is in the "_Begin" buffer; return false if the result is in the "_Output" buffer.
// We can't always put the result into one assigned buffer because that may cause frequent buffer copies at return time.
template<typename _Random_iterator, typename _Random_buffer_iterator, typename _Function>
inline bool _Parallel_buffered_sort_impl(const _Random_iterator &_Begin, size_t _Size, _Random_buffer_iterator _Output, const _Function &_Func,
    int _Div_num, const size_t _Chunk_size)
{
    static_assert(::std::is_same_v<typename ::std::iterator_traits<_Random_iterator>::value_type, typename ::std::iterator_traits<_Random_buffer_iterator>::value_type>,
        "same value type expected");

    if (_Div_num <= 1 || _Size <= _Chunk_size)
    {
        _Parallel_quicksort_impl(_Begin, _Size, _Func, _MAX_NUM_TASKS_PER_CORE, _Chunk_size, 0);

        // In case _Size <= _Chunk_size happened BEFORE the planned stop time (when _Div_num == 1) we need to calculate how many turns of
        // binary divisions are left. If there are an odd number of turns left, then the buffer move is necessary to make sure the final
        // merge result will be in the original input array.
        int _Left_div_turns = 0;
        while (_Div_num >>= 1)
        {
            _Left_div_turns++;
        }

        if (_Left_div_turns & 1)
        {
            ::std::move(_Begin, _Begin + _Size, _Output);
            return true;
        }
        else
        {
            return false;
        }
    }
    else
    {
        size_t _Mid = _Size / 2;
        structured_task_group _Tg;

        auto _Handle = make_task([&, _Chunk_size]
        {
            _Parallel_buffered_sort_impl(_Begin, _Mid, _Output, _Func, _Div_num / 2, _Chunk_size);
        });
        _Tg.run(_Handle);

        bool _Is_buffer_swap = _Parallel_buffered_sort_impl(_Begin + _Mid, _Size - _Mid, _Output + _Mid, _Func, _Div_num / 2, _Chunk_size);

        _Tg.wait();

        if (_Is_buffer_swap)
        {
            _Parallel_merge(_Output, _Mid, _Output + _Mid, _Size - _Mid, _Begin, _Func, _Div_num);
        }
        else
        {
            _Parallel_merge(_Begin, _Mid, _Begin + _Mid, _Size - _Mid, _Output, _Func, _Div_num);
        }

        return !_Is_buffer_swap;
    }
}

// Disable the warning saying constant value in condition expression.
// This is by design that lets the compiler optimize the trivial constructor.
#pragma warning (push)
#pragma warning (disable: 4127)

// Allocate and construct a buffer
template<typename _Allocator>
inline typename ::std::allocator_traits<_Allocator>::pointer _Construct_buffer(size_t _N, _Allocator &_Alloc)
{
    using _Traits = ::std::allocator_traits<_Allocator>;
    using _Value_type = typename _Allocator::value_type;
    using _Pointer = typename _Traits::pointer;

    const _Pointer _P = _Alloc.allocate(_N);

    // If the objects being sorted have trivial default initialization, they do not need to be
    // initialized here. This can benefit performance.
    if (!::std::is_trivially_default_constructible_v<_Value_type>)
    {
        for (size_t _I = 0; _I < _N; _I++)
        {
            // Objects being sorted must be default-initializable
            _Traits::construct(_Alloc, _P + _I);
        }
    }

    return _P;
}

// Destroy and deallocate a buffer
template<typename _Allocator>
inline void _Destroy_buffer(typename ::std::allocator_traits<_Allocator>::pointer _P, size_t _N, _Allocator &_Alloc)
{
    using _Traits = ::std::allocator_traits<_Allocator>;

    // If the objects being sorted have trivial destruction, they do not need to be
    // destroyed here. This can benefit performance.
    if (!::std::is_trivially_destructible_v<typename _Allocator::value_type>)
    {
        for (size_t _I = 0; _I < _N; _I++)
        {
            _Traits::destroy(_Alloc, _P + _I);
        }
    }

    _Alloc.deallocate(_P, _N);
}

//
// Exception safe RAII wrapper for the allocated buffers
//

template<typename _Allocator>
class _AllocatedBufferHolder
{
public:
    _AllocatedBufferHolder(size_t _Size, const _Allocator &_Alloc)
        : _M_size(_Size),
        _M_alloc(_Alloc),
        _M_buffer(_Construct_buffer(_Size, _M_alloc))
    {
    }

    ~_AllocatedBufferHolder()
    {
        _Destroy_buffer(_M_buffer, _M_size, _M_alloc);
    }

    typename ::std::allocator_traits<_Allocator>::pointer _Get_buffer()
    {
        return _M_buffer;
    }

private:
    size_t _M_size;
    _Allocator _M_alloc;
    typename ::std::allocator_traits<_Allocator>::pointer _M_buffer;
};


#pragma warning (pop)

/// <summary>
///     Arranges the elements in a specified range into a nondescending order, or according to an ordering criterion specified by a binary predicate,
///     in parallel. This function is semantically similar to <c>std::sort</c> in that it is a compare-based, unstable, in-place sort.
/// </summary>
/// <typeparam name="_Random_iterator">
///     The iterator type of the input range.
/// </typeparam>
/// <typeparam name="_Function">
///     The type of the binary comparison functor.
/// </typeparam>
/// <param name="_Begin">
///     A random-access iterator addressing the position of the first element in the range to be sorted.
/// </param>
/// <param name="_End">
///     A random-access iterator addressing the position one past the final element in the range to be sorted.
/// </param>
/// <param name="_Func">
///     A user-defined predicate function object that defines the comparison criterion to be satisfied by successive elements in the ordering.
///     A binary predicate takes two arguments and returns <c>true</c> when satisfied and <c>false</c> when not satisfied. This comparator function
///     must impose a strict weak ordering on pairs of elements from the sequence.
/// </param>
/// <param name="_Chunk_size">
///     The minimum size of a chunk that will be split into two for parallel execution.
/// </param>
/// <remarks>
///     The first overload uses the binary comparator <c>std::less</c>.
///     <para>The second overloaded uses the supplied binary comparator that should have the signature <c>bool _Func(T, T)</c> where <c>T</c>
///     is the type of the elements in the input range.</para>
///     <para>The algorithm divides the input range into two chunks and successively divides each chunk into two sub-chunks for execution in parallel. The optional
///     argument <paramref name="_Chunk_size"/> can be used to indicate to the algorithm that it should handles chunks of size &lt; <paramref name="_Chunk_size"/>
///     serially.</para>
/// </remarks>
/**/
template<typename _Random_iterator, typename _Function>
inline void parallel_sort(const _Random_iterator &_Begin, const _Random_iterator &_End, const _Function &_Func, const size_t _Chunk_size = 2048)
{
    _CONCRT_ASSERT(_Chunk_size > 0);

    // Check for cancellation before the algorithm starts.
    interruption_point();

    size_t _Size = _End - _Begin;
    size_t _Core_num = ::Concurrency::details::_CurrentScheduler::_GetNumberOfVirtualProcessors();

    if (_Size <= _Chunk_size || _Core_num < 2)
    {
        return ::std::sort(_Begin, _End, _Func);
    }

    _Parallel_quicksort_impl(_Begin, _Size, _Func, _Core_num * _MAX_NUM_TASKS_PER_CORE, _Chunk_size, 0);
}

/// <summary>
///     Arranges the elements in a specified range into a nondescending order, or according to an ordering criterion specified by a binary predicate,
///     in parallel. This function is semantically similar to <c>std::sort</c> in that it is a compare-based, unstable, in-place sort.
/// </summary>
/// <typeparam name="_Random_iterator">
///     The iterator type of the input range.
/// </typeparam>
/// <param name="_Begin">
///     A random-access iterator addressing the position of the first element in the range to be sorted.
/// </param>
/// <param name="_End">
///     A random-access iterator addressing the position one past the final element in the range to be sorted.
/// </param>
/// <remarks>
///     The first overload uses the binary comparator <c>std::less</c>.
///     <para>The second overloaded uses the supplied binary comparator that should have the signature <c>bool _Func(T, T)</c> where <c>T</c>
///     is the type of the elements in the input range.</para>
///     <para>The algorithm divides the input range into two chunks and successively divides each chunk into two sub-chunks for execution in parallel. The optional
///     argument <paramref name="_Chunk_size"/> can be used to indicate to the algorithm that it should handles chunks of size &lt; <paramref name="_Chunk_size"/>
///     serially.</para>
/// </remarks>
/**/
template<typename _Random_iterator>
inline void parallel_sort(const _Random_iterator &_Begin, const _Random_iterator &_End)
{
    parallel_sort(_Begin, _End, ::std::less<typename ::std::iterator_traits<_Random_iterator>::value_type>());
}

/// <summary>
///     Arranges the elements in a specified range into a nondescending order, or according to an ordering criterion specified by a binary predicate,
///     in parallel. This function is semantically similar to <c>std::sort</c> in that it is a compare-based, unstable, in-place sort except that
///     it needs <c>O(n)</c> additional space, and requires default initialization for the elements being sorted.
/// </summary>
/// <typeparam name="_Allocator">
///     The type of an STL compatible memory allocator.
/// </typeparam>
/// <typeparam name="_Random_iterator">
///     The iterator type of the input range.
/// </typeparam>
/// <typeparam name="_Function">
///     The type of the binary comparator.
/// </typeparam>
/// <param name="_Alloc">
///     An instance of an STL compatible memory allocator.
/// </param>
/// <param name="_Begin">
///     A random-access iterator addressing the position of the first element in the range to be sorted.
/// </param>
/// <param name="_End">
///     A random-access iterator addressing the position one past the final element in the range to be sorted.
/// </param>
/// <param name="_Func">
///     A user-defined predicate function object that defines the comparison criterion to be satisfied by successive elements in the ordering.
///     A binary predicate takes two arguments and returns <c>true</c> when satisfied and <c>false</c> when not satisfied. This comparator function
///     must impose a strict weak ordering on pairs of elements from the sequence.
/// </param>
/// <param name="_Chunk_size">
///     The minimum size of a chunk that will be split into two for parallel execution.
/// </param>
/// <remarks>
///     All overloads require <c>n * sizeof(T)</c> additional space, where <c>n</c> is the number of elements to be sorted, and <c>T</c> is the element type.
///     In most cases parallel_buffered_sort will show an improvement in performance over <see cref="parallel_sort Function">parallel_sort</see>, and you should
///     use it over parallel_sort if you have the memory available.
///     <para>If you do not supply a binary comparator <c>std::less</c> is used as the default, which requires the element type to provide the
///     operator <c>operator&lt;()</c>.</para>
///     <para>If you do not supply an allocator type or instance, the STL memory allocator <c>std::allocator&lt;T&gt;</c> is used to allocate the buffer.</para>
///     <para>The algorithm divides the input range into two chunks and successively divides each chunk into two sub-chunks for execution in parallel. The optional
///     argument <paramref name="_Chunk_size"/> can be used to indicate to the algorithm that it should handles chunks of size &lt; <paramref name="_Chunk_size"/>
///     serially.</para>
/// </remarks>
/**/
template<typename _Allocator, typename _Random_iterator, typename _Function>
inline void parallel_buffered_sort(const _Allocator& _Alloc, const _Random_iterator &_Begin, const _Random_iterator &_End, const _Function &_Func, const size_t _Chunk_size = 2048)
{
    _CONCRT_ASSERT(_Chunk_size > 0);

    // Check cancellation before the algorithm starts.
    interruption_point();

    size_t _Size = _End - _Begin;
    size_t _Core_num = ::Concurrency::details::_CurrentScheduler::_GetNumberOfVirtualProcessors();

    if (_Size <= _Chunk_size || _Core_num < 2)
    {
        return ::std::sort(_Begin, _End, _Func);
    }
    static constexpr size_t _CORE_NUM_MASK = 0x55555555;

    _AllocatedBufferHolder<_Allocator> _Holder(_Size, _Alloc);

    // Prevent cancellation from happening during the algorithm in case it leaves buffers in unknown state.
    run_with_cancellation_token([&]() {
        // This buffered sort algorithm will divide chunks and apply parallel quicksort on each chunk. In the end, it will
        // apply parallel merge to these sorted chunks.
        //
        // We need to decide on the number of chunks to divide the input buffer into. If we divide it into n chunks, log(n)
        // merges will be needed to get the final sorted result. In this algorithm, we have two buffers for each merge
        // operation, let's say buffer A and B. Buffer A is the original input array, buffer B is the additional allocated
        // buffer. Each turn's merge will put the merge result into the other buffer; for example, if we decided to split
        // into 8 chunks in buffer A at very beginning, after one pass of merging, there will be 4 chunks in buffer B.
        // If we apply one more pass of merging, there will be 2 chunks in buffer A again.
        //
        // The problem is we want to the final merge pass to put the result back in buffer A, so that we don't need
        // one extra copy to put the sorted data back to buffer A.
        // To make sure the final result is in buffer A (original input array), we need an even number of merge passes,
        // which means log(n) must be an even number. Thus n must be a number power(2, even number). For example, when the
        // even number is 2, n is power(2, 2) = 4, when even number is 4, n is power(2, 4) = 16. When we divide chunks
        // into these numbers, the final merge result will be in the original input array. Now we need to decide the chunk(split)
        // number based on this property and the number of cores.
        //
        // We want to get a chunk (split) number close to the core number (or a little more than the number of cores),
        // and it also needs to satisfy above property. For a 8 core machine, the best chunk number should be 16, because it's
        // the smallest number that satisfies the above property and is bigger than the core number (so that we can utilize all
        // cores, a little more than core number is OK, we need to split more tasks anyway).
        //
        // In this algorithm, we will make this alignment by bit operations (it's easy and clear). For a binary representation,
        // all the numbers that satisfy power(2, even number) will be 1, 100, 10000, 1000000, 100000000 ...
        // After OR-ing these numbers together, we will get a mask (... 0101 0101 0101) which is all possible combinations of
        // power(2, even number). We use _Core_num & _CORE_NUM_MASK | _Core_num << 1 & _CORE_NUM_MASK, a bit-wise operation to align
        // _Core_num's highest bit into a power(2, even number).
        //
        // It means if _Core_num = 8, the highest bit in binary is bin(1000) which is not power(2, even number). After this
        // bit-wise operation, it will align to bin(10000) = 16 which is power(2, even number). If the _Core_num = 16, after
        // alignment it still returns 16. The trick is to make sure the highest bit of _Core_num will align to the "1" bit of the
        // mask bin(... 0101 0101 0101) We don't care about the other bits on the aligned result except the highest bit, because they
        // will be ignored in the function.
        _Parallel_buffered_sort_impl(_Begin, _Size, _Holder._Get_buffer(),
            _Func, _Core_num & _CORE_NUM_MASK | _Core_num << 1 & _CORE_NUM_MASK, _Chunk_size);
    }, cancellation_token::none());

}

/// <summary>
///     Arranges the elements in a specified range into a nondescending order, or according to an ordering criterion specified by a binary predicate,
///     in parallel. This function is semantically similar to <c>std::sort</c> in that it is a compare-based, unstable, in-place sort except that
///     it needs <c>O(n)</c> additional space, and requires default initialization for the elements being sorted.
/// </summary>
/// <typeparam name="_Allocator">
///     The type of an STL compatible memory allocator.
/// </typeparam>
/// <typeparam name="_Random_iterator">
///     The iterator type of the input range.
/// </typeparam>
/// <typeparam name="_Function">
///     The type of the binary comparator.
/// </typeparam>
/// <param name="_Begin">
///     A random-access iterator addressing the position of the first element in the range to be sorted.
/// </param>
/// <param name="_End">
///     A random-access iterator addressing the position one past the final element in the range to be sorted.
/// </param>
/// <param name="_Func">
///     A user-defined predicate function object that defines the comparison criterion to be satisfied by successive elements in the ordering.
///     A binary predicate takes two arguments and returns <c>true</c> when satisfied and <c>false</c> when not satisfied. This comparator function
///     must impose a strict weak ordering on pairs of elements from the sequence.
/// </param>
/// <param name="_Chunk_size">
///     The minimum size of a chunk that will be split into two for parallel execution.
/// </param>
/// <remarks>
///     All overloads require <c>n * sizeof(T)</c> additional space, where <c>n</c> is the number of elements to be sorted, and <c>T</c> is the element type.
///     In most cases parallel_buffered_sort will show an improvement in performance over <see cref="parallel_sort Function">parallel_sort</see>, and you should
///     use it over parallel_sort if you have the memory available.
///     <para>If you do not supply a binary comparator <c>std::less</c> is used as the default, which requires the element type to provide the
///     operator <c>operator&lt;()</c>.</para>
///     <para>If you do not supply an allocator type or instance, the STL memory allocator <c>std::allocator&lt;T&gt;</c> is used to allocate the buffer.</para>
///     <para>The algorithm divides the input range into two chunks and successively divides each chunk into two sub-chunks for execution in parallel. The optional
///     argument <paramref name="_Chunk_size"/> can be used to indicate to the algorithm that it should handles chunks of size &lt; <paramref name="_Chunk_size"/>
///     serially.</para>
/// </remarks>
/**/
template<typename _Allocator, typename _Random_iterator, typename _Function>
inline void parallel_buffered_sort(const _Random_iterator &_Begin, const _Random_iterator &_End, const _Function &_Func, const size_t _Chunk_size = 2048)
{
    _Allocator _Alloc;
    return parallel_buffered_sort<_Allocator, _Random_iterator, _Function>(_Alloc, _Begin, _End, _Func, _Chunk_size);
}

/// <summary>
///     Arranges the elements in a specified range into a nondescending order, or according to an ordering criterion specified by a binary predicate,
///     in parallel. This function is semantically similar to <c>std::sort</c> in that it is a compare-based, unstable, in-place sort except that
///     it needs <c>O(n)</c> additional space, and requires default initialization for the elements being sorted.
/// </summary>
/// <typeparam name="_Random_iterator">
///     The iterator type of the input range.
/// </typeparam>
/// <typeparam name="_Function">
///     The type of the binary comparator.
/// </typeparam>
/// <param name="_Begin">
///     A random-access iterator addressing the position of the first element in the range to be sorted.
/// </param>
/// <param name="_End">
///     A random-access iterator addressing the position one past the final element in the range to be sorted.
/// </param>
/// <param name="_Func">
///     A user-defined predicate function object that defines the comparison criterion to be satisfied by successive elements in the ordering.
///     A binary predicate takes two arguments and returns <c>true</c> when satisfied and <c>false</c> when not satisfied. This comparator function
///     must impose a strict weak ordering on pairs of elements from the sequence.
/// </param>
/// <param name="_Chunk_size">
///     The minimum size of a chunk that will be split into two for parallel execution.
/// </param>
/// <remarks>
///     All overloads require <c>n * sizeof(T)</c> additional space, where <c>n</c> is the number of elements to be sorted, and <c>T</c> is the element type.
///     In most cases parallel_buffered_sort will show an improvement in performance over <see cref="parallel_sort Function">parallel_sort</see>, and you should
///     use it over parallel_sort if you have the memory available.
///     <para>If you do not supply a binary comparator <c>std::less</c> is used as the default, which requires the element type to provide the
///     operator <c>operator&lt;()</c>.</para>
///     <para>If you do not supply an allocator type or instance, the STL memory allocator <c>std::allocator&lt;T&gt;</c> is used to allocate the buffer.</para>
///     <para>The algorithm divides the input range into two chunks and successively divides each chunk into two sub-chunks for execution in parallel. The optional
///     argument <paramref name="_Chunk_size"/> can be used to indicate to the algorithm that it should handles chunks of size &lt; <paramref name="_Chunk_size"/>
///     serially.</para>
/// </remarks>
/**/
template<typename _Random_iterator, typename _Function>
inline void parallel_buffered_sort(const _Random_iterator &_Begin, const _Random_iterator &_End, const _Function &_Func, const size_t _Chunk_size = 2048)
{
    parallel_buffered_sort<::std::allocator<typename ::std::iterator_traits<_Random_iterator>::value_type>>(_Begin, _End, _Func, _Chunk_size);
}

/// <summary>
///     Arranges the elements in a specified range into a nondescending order, or according to an ordering criterion specified by a binary predicate,
///     in parallel. This function is semantically similar to <c>std::sort</c> in that it is a compare-based, unstable, in-place sort except that
///     it needs <c>O(n)</c> additional space, and requires default initialization for the elements being sorted.
/// </summary>
/// <typeparam name="_Random_iterator">
///     The iterator type of the input range.
/// </typeparam>
/// <param name="_Begin">
///     A random-access iterator addressing the position of the first element in the range to be sorted.
/// </param>
/// <param name="_End">
///     A random-access iterator addressing the position one past the final element in the range to be sorted.
/// </param>
/// <remarks>
///     All overloads require <c>n * sizeof(T)</c> additional space, where <c>n</c> is the number of elements to be sorted, and <c>T</c> is the element type.
///     In most cases parallel_buffered_sort will show an improvement in performance over <see cref="parallel_sort Function">parallel_sort</see>, and you should
///     use it over parallel_sort if you have the memory available.
///     <para>If you do not supply a binary comparator <c>std::less</c> is used as the default, which requires the element type to provide the
///     operator <c>operator&lt;()</c>.</para>
///     <para>If you do not supply an allocator type or instance, the STL memory allocator <c>std::allocator&lt;T&gt;</c> is used to allocate the buffer.</para>
///     <para>The algorithm divides the input range into two chunks and successively divides each chunk into two sub-chunks for execution in parallel. The optional
///     argument <paramref name="_Chunk_size"/> can be used to indicate to the algorithm that it should handles chunks of size &lt; <paramref name="_Chunk_size"/>
///     serially.</para>
/// </remarks>
/**/
template<typename _Random_iterator>
inline void parallel_buffered_sort(const _Random_iterator &_Begin, const _Random_iterator &_End)
{
    parallel_buffered_sort<::std::allocator<typename ::std::iterator_traits<_Random_iterator>::value_type>>(_Begin, _End,
        ::std::less<typename ::std::iterator_traits<_Random_iterator>::value_type>());
}

/// <summary>
///     Arranges the elements in a specified range into a nondescending order, or according to an ordering criterion specified by a binary predicate,
///     in parallel. This function is semantically similar to <c>std::sort</c> in that it is a compare-based, unstable, in-place sort except that
///     it needs <c>O(n)</c> additional space, and requires default initialization for the elements being sorted.
/// </summary>
/// <typeparam name="_Allocator">
///     The type of an STL compatible memory allocator.
/// </typeparam>
/// <typeparam name="_Random_iterator">
///     The iterator type of the input range.
/// </typeparam>
/// <param name="_Begin">
///     A random-access iterator addressing the position of the first element in the range to be sorted.
/// </param>
/// <param name="_End">
///     A random-access iterator addressing the position one past the final element in the range to be sorted.
/// </param>
/// <remarks>
///     All overloads require <c>n * sizeof(T)</c> additional space, where <c>n</c> is the number of elements to be sorted, and <c>T</c> is the element type.
///     In most cases parallel_buffered_sort will show an improvement in performance over <see cref="parallel_sort Function">parallel_sort</see>, and you should
///     use it over parallel_sort if you have the memory available.
///     <para>If you do not supply a binary comparator <c>std::less</c> is used as the default, which requires the element type to provide the
///     operator <c>operator&lt;()</c>.</para>
///     <para>If you do not supply an allocator type or instance, the STL memory allocator <c>std::allocator&lt;T&gt;</c> is used to allocate the buffer.</para>
///     <para>The algorithm divides the input range into two chunks and successively divides each chunk into two sub-chunks for execution in parallel. The optional
///     argument <paramref name="_Chunk_size"/> can be used to indicate to the algorithm that it should handles chunks of size &lt; <paramref name="_Chunk_size"/>
///     serially.</para>
/// </remarks>
/**/
template<typename _Allocator, typename _Random_iterator>
inline void parallel_buffered_sort(const _Random_iterator &_Begin, const _Random_iterator &_End)
{
    parallel_buffered_sort<_Allocator>(_Begin, _End,
        ::std::less<typename ::std::iterator_traits<_Random_iterator>::value_type>());
}

/// <summary>
///     Arranges the elements in a specified range into a nondescending order, or according to an ordering criterion specified by a binary predicate,
///     in parallel. This function is semantically similar to <c>std::sort</c> in that it is a compare-based, unstable, in-place sort except that
///     it needs <c>O(n)</c> additional space, and requires default initialization for the elements being sorted.
/// </summary>
/// <typeparam name="_Allocator">
///     The type of an STL compatible memory allocator.
/// </typeparam>
/// <typeparam name="_Random_iterator">
///     The iterator type of the input range.
/// </typeparam>
/// <param name="_Alloc">
///     An instance of an STL compatible memory allocator.
/// </param>
/// <param name="_Begin">
///     A random-access iterator addressing the position of the first element in the range to be sorted.
/// </param>
/// <param name="_End">
///     A random-access iterator addressing the position one past the final element in the range to be sorted.
/// </param>
/// <remarks>
///     All overloads require <c>n * sizeof(T)</c> additional space, where <c>n</c> is the number of elements to be sorted, and <c>T</c> is the element type.
///     In most cases parallel_buffered_sort will show an improvement in performance over <see cref="parallel_sort Function">parallel_sort</see>, and you should
///     use it over parallel_sort if you have the memory available.
///     <para>If you do not supply a binary comparator <c>std::less</c> is used as the default, which requires the element type to provide the
///     operator <c>operator&lt;()</c>.</para>
///     <para>If you do not supply an allocator type or instance, the STL memory allocator <c>std::allocator&lt;T&gt;</c> is used to allocate the buffer.</para>
///     <para>The algorithm divides the input range into two chunks and successively divides each chunk into two sub-chunks for execution in parallel. The optional
///     argument <paramref name="_Chunk_size"/> can be used to indicate to the algorithm that it should handles chunks of size &lt; <paramref name="_Chunk_size"/>
///     serially.</para>
/// </remarks>
/**/
template<typename _Allocator, typename _Random_iterator>
inline void parallel_buffered_sort(const _Allocator& _Alloc, const _Random_iterator &_Begin, const _Random_iterator &_End)
{
    parallel_buffered_sort<_Allocator>(_Alloc, _Begin, _End, ::std::less<typename ::std::iterator_traits<_Random_iterator>::value_type>());
}

#pragma warning(push)
#pragma warning (disable: 4127)
//
// This is a default function used for parallel_radixsort which will return just the value.
// It also performs compile-time checks to ensure that the data type is integral.
//
template <typename _DataType>
struct _Radix_sort_default_function
{
    size_t operator()(const _DataType& _Val) const
    {
        // An instance of the type predicate returns the value if the type _DataType is one of the integral types, otherwise it
        // statically asserts.
        // An integral type is one of: bool, char, unsigned char, signed char, wchar_t, short, unsigned short, int, unsigned int, long,
        // and unsigned long.
        // In addition, with compilers that provide them, an integral type can be one of long long, unsigned long long, __int64, and
        // unsigned __int64
        static_assert(::std::is_integral_v<_DataType>,
            "Type should be integral to use default radix function. For more information on integral types, please refer to https://msdn.microsoft.com/en-us/library/bb983099.aspx.");
        static_assert((sizeof(_DataType) <= sizeof(size_t)), "Passed Type is bigger than size_t.");

        if (::std::is_unsigned_v<_DataType>)
        {
            return _Val;
        }
        else
        {
            // The default function needs to take the signed integer-like representation and map it to an unsigned one. The
            // following code will take the midpoint of the unsigned representable range (SIZE_MAX/2)+1 and does an unsigned
            // add of the value. Thus, it maps a [-signed_min,+signed_max] range into a [0, unsigned_max] range.
            return (((SIZE_MAX/2) + 1) + static_cast<size_t>(_Val));
        }
    }
};
#pragma warning (pop)

/// <summary>
///     Arranges elements in a specified range into an non descending order using a radix sorting algorithm. This is a stable sort function which requires a
///     projection function that can project elements to be sorted into unsigned integer-like keys. Default initialization is required for the elements being sorted.
/// </summary>
/// <typeparam name="_Random_iterator">
///     The iterator type of the input range.
/// </typeparam>
/// <param name="_Begin">
///     A random-access iterator addressing the position of the first element in the range to be sorted.
/// </param>
/// <param name="_End">
///     A random-access iterator addressing the position one past the final element in the range to be sorted.
/// </param>
/// <remarks>
///     All overloads require <c>n * sizeof(T)</c> additional space, where <c>n</c> is the number of elements to be sorted, and <c>T</c> is the element type.
///     A unary projection functor with the signature<c>I _Proj_func(T)</c> is required to return a key when given an element, where <c>T</c> is the element
///     type and <c>I</c> is an unsigned integer-like type.
///     <para>If you do not supply a projection function, a default projection function which simply returns the element is used for integral types. The function
///     will fail to compile if the element is not an integral type in the absence of a projection function.</para>
///     <para>If you do not supply an allocator type or instance, the STL memory allocator <c>std::allocator&lt;T&gt;</c> is used to allocate the buffer.</para>
///     <para>The algorithm divides the input range into two chunks and successively divides each chunk into two sub-chunks for execution in parallel. The optional
///     argument <paramref name="_Chunk_size"/> can be used to indicate to the algorithm that it should handles chunks of size &lt; <paramref name="_Chunk_size"/>
///     serially.</para>
/// </remarks>
/**/
template<typename _Random_iterator>
inline void parallel_radixsort(const _Random_iterator &_Begin, const _Random_iterator &_End)
{
    typedef typename ::std::iterator_traits<_Random_iterator>::value_type _DataType;

    _Radix_sort_default_function<_DataType> _Proj_func;

    parallel_radixsort<::std::allocator<_DataType>>(_Begin, _End, _Proj_func, 256 * 256);
}

/// <summary>
///     Arranges elements in a specified range into an non descending order using a radix sorting algorithm. This is a stable sort function which requires a
///     projection function that can project elements to be sorted into unsigned integer-like keys. Default initialization is required for the elements being sorted.
/// </summary>
/// <typeparam name="_Allocator">
///     The type of an STL compatible memory allocator.
/// </typeparam>
/// <typeparam name="_Random_iterator">
///     The iterator type of the input range.
/// </typeparam>
/// <param name="_Alloc">
///     An instance of an STL compatible memory allocator.
/// </param>
/// <param name="_Begin">
///     A random-access iterator addressing the position of the first element in the range to be sorted.
/// </param>
/// <param name="_End">
///     A random-access iterator addressing the position one past the final element in the range to be sorted.
/// </param>
/// <remarks>
///     All overloads require <c>n * sizeof(T)</c> additional space, where <c>n</c> is the number of elements to be sorted, and <c>T</c> is the element type.
///     A unary projection functor with the signature<c>I _Proj_func(T)</c> is required to return a key when given an element, where <c>T</c> is the element
///     type and <c>I</c> is an unsigned integer-like type.
///     <para>If you do not supply a projection function, a default projection function which simply returns the element is used for integral types. The function
///     will fail to compile if the element is not an integral type in the absence of a projection function.</para>
///     <para>If you do not supply an allocator type or instance, the STL memory allocator <c>std::allocator&lt;T&gt;</c> is used to allocate the buffer.</para>
///     <para>The algorithm divides the input range into two chunks and successively divides each chunk into two sub-chunks for execution in parallel. The optional
///     argument <paramref name="_Chunk_size"/> can be used to indicate to the algorithm that it should handles chunks of size &lt; <paramref name="_Chunk_size"/>
///     serially.</para>
/// </remarks>
/**/
template<typename _Allocator, typename _Random_iterator>
inline void parallel_radixsort(const _Allocator& _Alloc, const _Random_iterator &_Begin, const _Random_iterator &_End)
{
    typedef typename ::std::iterator_traits<_Random_iterator>::value_type _DataType;

    _Radix_sort_default_function<_DataType> _Proj_func;

    parallel_radixsort<_Allocator>(_Alloc, _Begin, _End, _Proj_func);
}

/// <summary>
///     Arranges elements in a specified range into an non descending order using a radix sorting algorithm. This is a stable sort function which requires a
///     projection function that can project elements to be sorted into unsigned integer-like keys. Default initialization is required for the elements being sorted.
/// </summary>
/// <typeparam name="_Allocator">
///     The type of an STL compatible memory allocator.
/// </typeparam>
/// <typeparam name="_Random_iterator">
///     The iterator type of the input range.
/// </typeparam>
/// <param name="_Begin">
///     A random-access iterator addressing the position of the first element in the range to be sorted.
/// </param>
/// <param name="_End">
///     A random-access iterator addressing the position one past the final element in the range to be sorted.
/// </param>
/// <remarks>
///     All overloads require <c>n * sizeof(T)</c> additional space, where <c>n</c> is the number of elements to be sorted, and <c>T</c> is the element type.
///     A unary projection functor with the signature<c>I _Proj_func(T)</c> is required to return a key when given an element, where <c>T</c> is the element
///     type and <c>I</c> is an unsigned integer-like type.
///     <para>If you do not supply a projection function, a default projection function which simply returns the element is used for integral types. The function
///     will fail to compile if the element is not an integral type in the absence of a projection function.</para>
///     <para>If you do not supply an allocator type or instance, the STL memory allocator <c>std::allocator&lt;T&gt;</c> is used to allocate the buffer.</para>
///     <para>The algorithm divides the input range into two chunks and successively divides each chunk into two sub-chunks for execution in parallel. The optional
///     argument <paramref name="_Chunk_size"/> can be used to indicate to the algorithm that it should handles chunks of size &lt; <paramref name="_Chunk_size"/>
///     serially.</para>
/// </remarks>
/**/
template<typename _Allocator, typename _Random_iterator>
inline void parallel_radixsort(const _Random_iterator &_Begin, const _Random_iterator &_End)
{
    _Allocator _Alloc;
    return parallel_radixsort<_Allocator, _Random_iterator>(_Alloc, _Begin, _End);
}

/// <summary>
///     Arranges elements in a specified range into an non descending order using a radix sorting algorithm. This is a stable sort function which requires a
///     projection function that can project elements to be sorted into unsigned integer-like keys. Default initialization is required for the elements being sorted.
/// </summary>
/// <typeparam name="_Allocator">
///     The type of an STL compatible memory allocator.
/// </typeparam>
/// <typeparam name="_Random_iterator">
///     The iterator type of the input range.
/// </typeparam>
/// <typeparam name="_Function">
///     The type of the projection function.
/// </typeparam>
/// <param name="_Alloc">
///     An instance of an STL compatible memory allocator.
/// </param>
/// <param name="_Begin">
///     A random-access iterator addressing the position of the first element in the range to be sorted.
/// </param>
/// <param name="_End">
///     A random-access iterator addressing the position one past the final element in the range to be sorted.
/// </param>
/// <param name="_Proj_func">
///     A user-defined projection function object that converts an element into an integral value.
/// </param>
/// <param name="_Chunk_size">
///     The minimum size of a chunk that will be split into two for parallel execution.
/// </param>
/// <remarks>
///     All overloads require <c>n * sizeof(T)</c> additional space, where <c>n</c> is the number of elements to be sorted, and <c>T</c> is the element type.
///     A unary projection functor with the signature<c>I _Proj_func(T)</c> is required to return a key when given an element, where <c>T</c> is the element
///     type and <c>I</c> is an unsigned integer-like type.
///     <para>If you do not supply a projection function, a default projection function which simply returns the element is used for integral types. The function
///     will fail to compile if the element is not an integral type in the absence of a projection function.</para>
///     <para>If you do not supply an allocator type or instance, the STL memory allocator <c>std::allocator&lt;T&gt;</c> is used to allocate the buffer.</para>
///     <para>The algorithm divides the input range into two chunks and successively divides each chunk into two sub-chunks for execution in parallel. The optional
///     argument <paramref name="_Chunk_size"/> can be used to indicate to the algorithm that it should handles chunks of size &lt; <paramref name="_Chunk_size"/>
///     serially.</para>
/// </remarks>
/**/
template<typename _Allocator, typename _Random_iterator, typename _Function>
inline void parallel_radixsort(const _Allocator& _Alloc, const _Random_iterator &_Begin, const _Random_iterator &_End, const _Function &_Proj_func, const size_t _Chunk_size = 256 * 256)
{
    _CONCRT_ASSERT(_Chunk_size > 0);

    // Check for cancellation before the algorithm starts.
    interruption_point();

    size_t _Size = _End - _Begin;

    // If _Size <= 1, no more sorting needs to be done.
    if (_Size <= 1)
    {
        return;
    }

    _AllocatedBufferHolder<_Allocator> _Holder(_Size, _Alloc);

    // Prevent cancellation from happening during the algorithm in case it leaves the buffers in unknown state.
    run_with_cancellation_token([&]() {
        _Parallel_integer_sort_asc(_Begin, _Size, _Holder._Get_buffer(), _Proj_func, _Chunk_size);
    }, cancellation_token::none());
}

/// <summary>
///     Arranges elements in a specified range into an non descending order using a radix sorting algorithm. This is a stable sort function which requires a
///     projection function that can project elements to be sorted into unsigned integer-like keys. Default initialization is required for the elements being sorted.
/// </summary>
/// <typeparam name="_Allocator">
///     The type of an STL compatible memory allocator.
/// </typeparam>
/// <typeparam name="_Random_iterator">
///     The iterator type of the input range.
/// </typeparam>
/// <typeparam name="_Function">
///     The type of the projection function.
/// </typeparam>
/// <param name="_Begin">
///     A random-access iterator addressing the position of the first element in the range to be sorted.
/// </param>
/// <param name="_End">
///     A random-access iterator addressing the position one past the final element in the range to be sorted.
/// </param>
/// <param name="_Proj_func">
///     A user-defined projection function object that converts an element into an integral value.
/// </param>
/// <param name="_Chunk_size">
///     The minimum size of a chunk that will be split into two for parallel execution.
/// </param>
/// <remarks>
///     All overloads require <c>n * sizeof(T)</c> additional space, where <c>n</c> is the number of elements to be sorted, and <c>T</c> is the element type.
///     A unary projection functor with the signature<c>I _Proj_func(T)</c> is required to return a key when given an element, where <c>T</c> is the element
///     type and <c>I</c> is an unsigned integer-like type.
///     <para>If you do not supply a projection function, a default projection function which simply returns the element is used for integral types. The function
///     will fail to compile if the element is not an integral type in the absence of a projection function.</para>
///     <para>If you do not supply an allocator type or instance, the STL memory allocator <c>std::allocator&lt;T&gt;</c> is used to allocate the buffer.</para>
///     <para>The algorithm divides the input range into two chunks and successively divides each chunk into two sub-chunks for execution in parallel. The optional
///     argument <paramref name="_Chunk_size"/> can be used to indicate to the algorithm that it should handles chunks of size &lt; <paramref name="_Chunk_size"/>
///     serially.</para>
/// </remarks>
/**/
template<typename _Allocator, typename _Random_iterator, typename _Function>
inline void parallel_radixsort(const _Random_iterator &_Begin, const _Random_iterator &_End, const _Function &_Proj_func, const size_t _Chunk_size = 256 * 256)
{
    _Allocator _Alloc;
    return parallel_radixsort<_Allocator, _Random_iterator, _Function>(_Alloc, _Begin, _End, _Proj_func, _Chunk_size);
}

/// <summary>
///     Arranges elements in a specified range into an non descending order using a radix sorting algorithm. This is a stable sort function which requires a
///     projection function that can project elements to be sorted into unsigned integer-like keys. Default initialization is required for the elements being sorted.
/// </summary>
/// <typeparam name="_Random_iterator">
///     The iterator type of the input range.
/// </typeparam>
/// <typeparam name="_Function">
///     The type of the projection function.
/// </typeparam>
/// <param name="_Begin">
///     A random-access iterator addressing the position of the first element in the range to be sorted.
/// </param>
/// <param name="_End">
///     A random-access iterator addressing the position one past the final element in the range to be sorted.
/// </param>
/// <param name="_Proj_func">
///     A user-defined projection function object that converts an element into an integral value.
/// </param>
/// <param name="_Chunk_size">
///     The minimum size of a chunk that will be split into two for parallel execution.
/// </param>
/// <remarks>
///     All overloads require <c>n * sizeof(T)</c> additional space, where <c>n</c> is the number of elements to be sorted, and <c>T</c> is the element type.
///     A unary projection functor with the signature<c>I _Proj_func(T)</c> is required to return a key when given an element, where <c>T</c> is the element
///     type and <c>I</c> is an unsigned integer-like type.
///     <para>If you do not supply a projection function, a default projection function which simply returns the element is used for integral types. The function
///     will fail to compile if the element is not an integral type in the absence of a projection function.</para>
///     <para>If you do not supply an allocator type or instance, the STL memory allocator <c>std::allocator&lt;T&gt;</c> is used to allocate the buffer.</para>
///     <para>The algorithm divides the input range into two chunks and successively divides each chunk into two sub-chunks for execution in parallel. The optional
///     argument <paramref name="_Chunk_size"/> can be used to indicate to the algorithm that it should handles chunks of size &lt; <paramref name="_Chunk_size"/>
///     serially.</para>
/// </remarks>
/**/
template<typename _Random_iterator, typename _Function>
inline void parallel_radixsort(const _Random_iterator &_Begin, const _Random_iterator &_End, const _Function &_Proj_func, const size_t _Chunk_size = 256 * 256)
{
    parallel_radixsort<::std::allocator<typename ::std::iterator_traits<_Random_iterator>::value_type>>(
        _Begin, _End, _Proj_func, _Chunk_size);
}

#pragma pop_macro("_SORT_MAX_RECURSION_DEPTH")
#pragma pop_macro("_MAX_NUM_TASKS_PER_CORE")
#pragma pop_macro("_FINE_GRAIN_CHUNK_SIZE")
}

namespace concurrency = ::Concurrency;

#pragma pop_macro("new")
#pragma pack(pop)
