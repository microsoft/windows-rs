/***
* ==++==
*
* Copyright (c) Microsoft Corporation. All rights reserved.
*
* ==--==
* =+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+
*
* pplinterface.h
*
* Parallel Patterns Library - PPL interface definitions
*
* =-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-
****/

#ifndef _PPLINTERFACE_H
#define _PPLINTERFACE_H

#include <yvals_core.h>

#if _STL_COMPILER_PREPROCESSOR

#include <memory>
#include <atomic>

#pragma pack(push, _CRT_PACKING)
#pragma warning(push, _STL_WARNING_LEVEL)
#pragma warning(disable : _STL_DISABLED_WARNINGS)
_STL_DISABLE_CLANG_WARNINGS
#pragma push_macro("new")
#undef new

extern "C++" { // attach declarations in namespace Concurrency to the global module, see N4910 [module.unit]/7

namespace Concurrency
{

/// <summary>
///     An elementary abstraction for a task, defined as <c>void (__cdecl * TaskProc_t)(void *)</c>. A <c>TaskProc</c> is called to
///     invoke the body of a task.
/// </summary>
/**/
typedef void (__cdecl * TaskProc_t)(void *);

/// <summary>
///     Scheduler Interface
/// </summary>
struct __declspec(novtable) scheduler_interface
{
    virtual void schedule( TaskProc_t, void* ) = 0;
};

/// <summary>
///     Represents a pointer to a scheduler. This class exists to allow the
///     the specification of a shared lifetime by using shared_ptr or just
///     a plain reference by using raw pointer.
/// </summary>
struct scheduler_ptr
{
    /// <summary>
    /// Creates a scheduler pointer from shared_ptr to scheduler
    /// </summary>
    explicit scheduler_ptr(::std::shared_ptr<scheduler_interface> _Scheduler) : _M_sharedScheduler(::std::move(_Scheduler))
    {
        _M_scheduler = _M_sharedScheduler.get();
    }

    /// <summary>
    /// Creates a scheduler pointer from raw pointer to scheduler
    /// </summary>
    explicit scheduler_ptr(_In_opt_ scheduler_interface * _PScheduler) : _M_scheduler(_PScheduler)
    {
    }

    /// <summary>
    /// Behave like a pointer
    /// </summary>
    scheduler_interface *operator->() const
    {
        return get();
    }

    /// <summary>
    ///  Returns the raw pointer to the scheduler
    /// </summary>
    scheduler_interface * get() const
    {
        return _M_scheduler;
    }

    /// <summary>
    /// Test whether the scheduler pointer is non-null
    /// </summary>
    operator bool() const { return get() != nullptr; }

private:

    ::std::shared_ptr<scheduler_interface> _M_sharedScheduler;
    scheduler_interface * _M_scheduler;
};

/// <summary>
///     Describes the execution status of a <c>task_group</c> or <c>structured_task_group</c> object.  A value of this type is returned
///     by numerous methods that wait on tasks scheduled to a task group to complete.
/// </summary>
/// <seealso cref="task_group Class"/>
/// <seealso cref="task_group::wait Method"/>
/// <seealso cref="task_group::run_and_wait Method"/>
/// <seealso cref="structured_task_group Class"/>
/// <seealso cref="structured_task_group::wait Method"/>
/// <seealso cref="structured_task_group::run_and_wait Method"/>
/**/
enum task_group_status
{
    /// <summary>
    ///     The tasks queued to the <c>task_group</c> object have not completed.  Note that this value is not presently returned by
    ///     the Concurrency Runtime.
    /// </summary>
    /**/
    not_complete,

    /// <summary>
    ///     The tasks queued to the <c>task_group</c> or <c>structured_task_group</c> object completed successfully.
    /// </summary>
    /**/
    completed,

    /// <summary>
    ///     The <c>task_group</c> or <c>structured_task_group</c> object was canceled.  One or more tasks may not have executed.
    /// </summary>
    /**/
    canceled
};

/// <summary>
///     This class describes an exception thrown when an invalid operation is performed that is not more accurately
///     described by another exception type thrown by the Concurrency Runtime.
/// </summary>
/// <remarks>
///     The various methods which throw this exception will generally document under what circumstances they will throw it.
/// </remarks>
/**/
class invalid_operation : public ::std::exception
{
public:
    /// <summary>
    ///     Constructs an <c>invalid_operation</c> object.
    /// </summary>
    /// <param name="_Message">
    ///     A descriptive message of the error.
    /// </param>
    /**/
    explicit invalid_operation(_In_z_ const char* _Message) noexcept
        : exception(_Message)
    { }

    /// <summary>
    ///     Constructs an <c>invalid_operation</c> object.
    /// </summary>
    /**/
    invalid_operation() noexcept
        : exception()
    {}
};

/// <summary>
///     This class describes an exception thrown by the PPL tasks layer in order to force the current task
///     to cancel. It is also thrown by the <c>get()</c> method on <see cref="task Class">task</see>, for a
///     canceled task.
/// </summary>
/// <seealso cref="task::get Method"/>
/// <seealso cref="cancel_current_task Method"/>
/**/
class task_canceled : public ::std::exception
{
public:
    /// <summary>
    ///     Constructs a <c>task_canceled</c> object.
    /// </summary>
    /// <param name="_Message">
    ///     A descriptive message of the error.
    /// </param>
    /**/
    explicit task_canceled(_In_z_ const char * _Message) noexcept
        : exception(_Message)
    {}

    /// <summary>
    ///     Constructs a <c>task_canceled</c> object.
    /// </summary>
    /**/
    task_canceled() noexcept
        : exception()
    {}
};

namespace details
{

//
// An internal exception that is used for cancellation. Users do not "see" this exception except through the
// resulting stack unwind. This exception should never be intercepted by user code. It is intended
// for use by the runtime only.
//
class _Interruption_exception : public ::std::exception
{
public:
    explicit _Interruption_exception(const char * _Message) noexcept
        : exception(_Message)
    {}

    _Interruption_exception() noexcept
        : exception()
    {}
};

/// <summary>
///     The enum defines inlining scheduling policy for ppltasks.
///     Scheduling a chore or a functor with _TaskInliningMode will give
///     scheduler a hint on whether apply inline execution or not.
/// </summary>
/// <remarks>
///     As an optimization, we assigned an integer number to each option in the enum,
///     which effectively stands for the maximal inlining depth (threshold) for current chore,
///     and the scheduler will compare this threshold with current context's inlining depth to
///     make inline decision.
///     If the current context's inlining depth greater than this threshold,
///     the chore will be scheduled on a new context, otherwise the chore will be scheduled inline.
///     Minimal threshold 0 means do not inline; maximal threshold -1 (0xFFFFFFFF....) means always inline.
///     16 is a good default inlining threshold we figured out from experiment.
/// </remarks>
enum _TaskInliningMode
{
    // Disable inline scheduling
    _NoInline = 0,
    // Let runtime decide whether to do inline scheduling or not
    _DefaultAutoInline = 16,
    // Always do inline scheduling
    _ForceInline = -1,
};

/// <summary>
///     Atomics
/// </summary>
typedef ::std::atomic<long> atomic_long;
typedef ::std::atomic<size_t> atomic_size_t;

template<typename _T>
_T atomic_compare_exchange(::std::atomic<_T>& _Target, _T _Exchange, _T _Comparand)
{
    _T _Result = _Comparand;
    _Target.compare_exchange_strong(_Result, _Exchange);
    return _Result;
}

template<typename _T>
_T atomic_exchange(::std::atomic<_T>& _Target, _T _Value)
{
    return _Target.exchange(_Value);
}

template<typename _T>
_T atomic_increment(::std::atomic<_T>& _Target)
{
    return _Target.fetch_add(1) + 1;
}

template<typename _T>
_T atomic_decrement(::std::atomic<_T>& _Target)
{
    return _Target.fetch_sub(1) - 1;
}

template<typename _T>
_T atomic_add(::std::atomic<_T>& _Target, _T _Value)
{
    return _Target.fetch_add(_Value) + _Value;
}

}} // namespace Concurrency::details

} // extern "C++"

#pragma pop_macro("new")
_STL_RESTORE_CLANG_WARNINGS
#pragma warning(pop)
#pragma pack(pop)

#endif // _STL_COMPILER_PREPROCESSOR

#endif // _PPLINTERFACE_H
