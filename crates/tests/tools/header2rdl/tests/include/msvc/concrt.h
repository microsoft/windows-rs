/***
* ==++==
*
* Copyright (c) Microsoft Corporation. All rights reserved.
*
* ==--==
* =+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+
*
* concrt.h
*
* Main public header file for ConcRT. This is the only header file a C++ program must include to use the core concurrency runtime features.
*
* The Agents And Message Blocks Library and the Parallel Patterns Library (PPL) are defined in separate header files.
* =-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-
****/

#pragma once

#include <crtdefs.h>

#if defined(_CRT_WINDOWS)
#error Must not be included during CRT build with _CRT_WINDOWS flag enabled
#endif

#if !(defined (_M_X64) || defined (_M_IX86) || defined (_M_ARM) || defined (_M_ARM64))
    #error ERROR: Concurrency Runtime is supported only on X64, X86, ARM, and ARM64 architectures.
#endif  /* !(defined (_M_X64) || defined (_M_IX86) || defined (_M_ARM) || defined (_M_ARM64)) */

#if defined (_M_CEE)
    #error ERROR: Concurrency Runtime is not supported when compiling /clr.
#endif  /* defined (_M_CEE) */

#ifndef __cplusplus
    #error ERROR: Concurrency Runtime is supported only for C++.
#endif  /* __cplusplus */

#define _CONCRT_H

#include <exception>
#include <malloc.h>
#include <sal.h>
#include <limits.h>
#include <crtdbg.h>
#include <guiddef.h>
#include <intrin.h>
#include <new>

#include <pplinterface.h>

 #pragma pack(push,_CRT_PACKING)
 #pragma warning(push,_STL_WARNING_LEVEL)
 #pragma warning(disable: _STL_DISABLED_WARNINGS)
 _STL_DISABLE_CLANG_WARNINGS
 #pragma push_macro("new")
 #undef new

// Forward declare structs needed from Windows header files

struct _SECURITY_ATTRIBUTES;
typedef _SECURITY_ATTRIBUTES* LPSECURITY_ATTRIBUTES;

struct _GROUP_AFFINITY;
typedef _GROUP_AFFINITY* PGROUP_AFFINITY;

// Define essential types needed from Windows header files

typedef unsigned long DWORD;
#ifndef _HRESULT_DEFINED
#define _HRESULT_DEFINED
#ifdef __midl
typedef LONG HRESULT;
#else  /* __midl */
typedef _Return_type_success_(return >= 0) long HRESULT;
#endif  /* __midl */
#endif  /* _HRESULT_DEFINED */
typedef void * HANDLE;

#pragma push_macro("_YieldProcessor")
#undef _YieldProcessor

#if (defined (_M_IX86) || defined (_M_X64)) && !defined(_M_HYBRID) && !defined(_M_ARM64EC)
#define _YieldProcessor _mm_pause
#else
inline void _YieldProcessor() {}
#endif

#if (defined (_M_IX86) || defined (_M_ARM))

#define _InterlockedIncrementSizeT(_Target) static_cast<size_t>(_InterlockedIncrement(reinterpret_cast<long volatile *>(_Target)))
#define _InterlockedDecrementSizeT(_Target) static_cast<size_t>(_InterlockedDecrement(reinterpret_cast<long volatile *>(_Target)))
#define _InterlockedCompareExchangeSizeT(_Target, _Exchange, _Comparand) static_cast<size_t>(_InterlockedCompareExchange( \
    reinterpret_cast<long volatile *>(_Target), \
    static_cast<long>(_Exchange), \
    static_cast<long>(_Comparand)))

typedef unsigned long DWORD_PTR, *PDWORD_PTR;

#else  /* (defined (_M_IX86) || defined (_M_ARM)) */

#define _InterlockedIncrementSizeT(_Target) static_cast<size_t>(_InterlockedIncrement64(reinterpret_cast<__int64 volatile *>(_Target)))
#define _InterlockedDecrementSizeT(_Target) static_cast<size_t>(_InterlockedDecrement64(reinterpret_cast<__int64 volatile *>(_Target)))
#define _InterlockedCompareExchangeSizeT(_Target, _Exchange, _Comparand) static_cast<size_t>(_InterlockedCompareExchange64( \
    reinterpret_cast<__int64 volatile *>(_Target), \
    static_cast<__int64>(_Exchange), \
    static_cast<__int64>(_Comparand)))

typedef unsigned __int64 DWORD_PTR, *PDWORD_PTR;

#endif  /* (defined (_M_IX86) || defined (_M_ARM)) */

#ifdef _DEBUG
#ifdef _MSC_VER
// Turn off compiler warnings that are exacerbated by constructs in this
// file's definitions:

// Warning C4127: conditional expression is constant. This is caused by
//      the macros with "do { ... } while (false)" syntax. The syntax is
//      a good way to ensure that a statement-like macro can be used in all
//      contexts (specifically if statements), but the compiler warns about
//      the "while (false)" part.

#define _CONCRT_ASSERT(x)   __pragma (warning (suppress: 4127)) do {_ASSERTE(x); __assume(x);} while(false)
#else  /* _MSC_VER */
#define _CONCRT_ASSERT(x)   do {_ASSERTE(x); __assume(x);} while(false)
#endif  /* _MSC_VER */
#else  /* _DEBUG */
#define _CONCRT_ASSERT(x)   __assume(x)
#endif  /* _DEBUG */

// Used internally to represent the smallest unit in which to allocate hidden types


typedef void * _CONCRT_BUFFER;
#define _LISTENTRY_SIZE ((2 * sizeof(void *) + sizeof(_CONCRT_BUFFER) - 1) / sizeof(_CONCRT_BUFFER))
#define _SAFERWLIST_SIZE ((3 * sizeof(void *) + 2 * sizeof(long) + sizeof(_CONCRT_BUFFER) - 1) / sizeof(_CONCRT_BUFFER))

/// <summary>
///     The <c>Concurrency</c> namespace provides classes and functions that access the Concurrency Runtime,
///     a concurrent programming framework for C++. For more information, see <see cref="Concurrency Runtime"/>.
/// </summary>
/**/
namespace Concurrency
{
/// <summary>
///     Pauses the current context for a specified amount of time.
/// </summary>
/// <param name="_Milliseconds">
///     The number of milliseconds the current context should be paused for. If the <paramref name="_Milliseconds"/> parameter is set to
///     the value <c>0</c>, the current context should yield execution to other runnable contexts before continuing.
/// </param>
/// <remarks>
///     If this method is called on a Concurrency Runtime scheduler context, the scheduler will find a different context to run on the underlying
///     resource. Because the scheduler is cooperative in nature, this context cannot resume exactly after the number of milliseconds specified.
///     If the scheduler is busy executing other tasks that do not cooperatively yield to the scheduler, the wait period could be
///     indefinite.
/// </remarks>
/**/
_CONCRTIMP void __cdecl wait(unsigned int _Milliseconds);

/// <summary>
///     Allocates a block of memory of the size specified from the Concurrency Runtime Caching Suballocator.
/// </summary>
/// <param name="_NumBytes">
///     The number of bytes of memory to allocate.
/// </param>
/// <returns>
///     A pointer to newly allocated memory.
/// </returns>
/// <remarks>
///     For more information about which scenarios in your application could benefit from using the Caching Suballocator,
///     see <see cref="Task Scheduler (Concurrency Runtime)"/>.
/// </remarks>
/// <seealso cref="Concurrency::Free Function"/>
/**/
_CONCRTIMP void * __cdecl Alloc(size_t _NumBytes);

/// <summary>
///     Releases a block of memory previously allocated by the <c>Alloc</c> method to the Concurrency Runtime Caching Suballocator.
/// </summary>
/// <param name="_PAllocation">
///     A pointer to memory previously allocated by the <c>Alloc</c> method which is to be freed. If the parameter <paramref name="_PAllocation"/>
///     is set to the value <c>NULL</c>, this method will ignore it and return immediately.
/// </param>
/// <remarks>
///     For more information about which scenarios in your application could benefit from using the Caching Suballocator,
///     see <see cref="Task Scheduler (Concurrency Runtime)"/>.
/// </remarks>
/// <seealso cref="Concurrency::Alloc Function"/>
/**/
_CONCRTIMP void __cdecl Free(_Pre_maybenull_ _Post_invalid_ void * _PAllocation);

/// <summary>
///     Concurrency::details contains definitions of support routines in the public namespaces and one or more macros.
///     Users should not directly interact with this internal namespace.
/// </summary>
/**/

#if defined(_CRT_USE_WINAPI_FAMILY_DESKTOP_APP) || defined(_XBOX_ONE)

/// <summary>
///     Restricts the execution resources used by the Concurrency Runtime internal worker threads to the affinity set specified.
///     <para> It is valid to call this method only before the Resource Manager has been created, or between two Resource Manager lifetimes.
///     It can be invoked multiple times as long as the Resource Manager does not exist at the time of invocation. After an affinity limit
///     has been set, it remains in effect until the next valid call to the <c>set_task_execution_resources</c> method.</para>
///     <para>The affinity mask provided need not be a subset of the process affinity mask. The process affinity will be updated if necessary.</para>
/// </summary>
/// <param name="_ProcessAffinityMask">
///     The affinity mask that the Concurrency Runtime worker threads are to be restricted to. Use this method on a system with greater than 64
///     hardware threads only if you want to limit the Concurrency Runtime to a subset of the current processor group. In general, you should
///     use the version of the method that accepts an array of group affinities as a parameter, to restrict affinity on machines with greater
///     than 64 hardware threads.
/// </param>
/// <remarks>
///     The method will throw an <see cref="invalid_operation Class">invalid_operation </see> exception if a Resource Manager is present at
///     the time it is invoked, and an <see cref="invalid_argument">invalid_argument </see> exception if the affinity specified results in an empty set of resources.
///     <para>The version of the method that takes an array of group affinities as a parameter should only be used on operating systems with version
///     Windows 7 or higher. Otherwise, an <see cref="invalid_operation Class">invalid_operation </see> exception is thrown.</para>
///     <para>Programatically modifying the process affinity after this method has been invoked will not cause the Resource Manager to re-evaluate
///     the affinity it is restricted to. Therefore, all changes to process affinity should be made before calling this method.</para>
/// </remarks>
/**/
_CONCRTIMP void __cdecl set_task_execution_resources(DWORD_PTR _ProcessAffinityMask);

#endif /* defined(_CRT_USE_WINAPI_FAMILY_DESKTOP_APP) || defined(_XBOX_ONE) */

#ifdef _CRT_USE_WINAPI_FAMILY_DESKTOP_APP

/// <summary>
///     Restricts the execution resources used by the Concurrency Runtime internal worker threads to the affinity set specified.
///     <para> It is valid to call this method only before the Resource Manager has been created, or between two Resource Manager lifetimes.
///     It can be invoked multiple times as long as the Resource Manager does not exist at the time of invocation. After an affinity limit
///     has been set, it remains in effect until the next valid call to the <c>set_task_execution_resources</c> method.</para>
///     <para>The affinity mask provided need not be a subset of the process affinity mask. The process affinity will be updated if necessary.</para>
/// </summary>
/// <param name="_Count">
///    The number of <c>GROUP_AFFINITY</c> entries in the array specified by the parameter <paramref name="_PGroupAffinity"/>.
/// </param>
/// <param name="_PGroupAffinity">
///     An array of <c>GROUP_AFFINITY</c> entries.
/// </param>
/// <remarks>
///     The method will throw an <see cref="invalid_operation Class">invalid_operation </see> exception if a Resource Manager is present at
///     the time it is invoked, and an <see cref="invalid_argument">invalid_argument </see> exception if the affinity specified results in an empty set of resources.
///     <para>The version of the method that takes an array of group affinities as a parameter should only be used on operating systems with version
///     Windows 7 or higher. Otherwise, an <see cref="invalid_operation Class">invalid_operation </see> exception is thrown.</para>
///     <para>Programatically modifying the process affinity after this method has been invoked will not cause the Resource Manager to re-evaluate
///     the affinity it is restricted to. Therefore, all changes to process affinity should be made before calling this method.</para>
/// </remarks>
/**/
_CONCRTIMP void __cdecl set_task_execution_resources(unsigned short _Count, PGROUP_AFFINITY _PGroupAffinity);

#endif  /* _CRT_USE_WINAPI_FAMILY_DESKTOP_APP */

/// <summary>
///     An elementary abstraction for a task, defined as <c>void (__cdecl * TaskProc)(void *)</c>. A <c>TaskProc</c> is called to
///     invoke the body of a task.
/// </summary>
/**/
typedef void (__cdecl * TaskProc)(void *);

//
// Forward declarations:
//
class Scheduler;
class ScheduleGroup;
class Context;

namespace details
{
    //
    // Forward declarations:
    //
    class ContextBase;
    class _TaskCollectionBase;

    //
    // A utility to hide operator delete from certain objects while still allowing the runtime to delete them internally.
    //
    template<class _Ty>
    void _InternalDeleteHelper(_Ty * _PObject)
    {
        delete _PObject;
    }

    // The purpose of the class is solely to direct allocations of ConcRT classes
    // through a single point, using an internal allocator.
    struct _AllocBase
    {
        // Standard operator new
        void * operator new(size_t _Size)
        {
            return ::Concurrency::Alloc(_Size);
        }

        // Standard operator delete
        void operator delete(void * _Ptr) noexcept
        {
            ::Concurrency::Free(_Ptr);
        }

        // Standard operator new, no-throw version
        void * operator new(size_t _Size, const ::std::nothrow_t&) noexcept
        {
            void * _Ptr;

            try
            {
                _Ptr = ::Concurrency::Alloc(_Size);
            }
            catch(...)
            {
                _Ptr = nullptr;
            }

            return (_Ptr);
        }

        // Standard operator delete, no-throw version
        void operator delete(void * _Ptr, const ::std::nothrow_t&) noexcept
        {
            operator delete(_Ptr);
        }

        // Standard operator new array
        void * operator new[](size_t _Size)
        {
            return operator new(_Size);
        }

        // Standard operator delete array
        void operator delete[](void * _Ptr) noexcept
        {
            operator delete(_Ptr);
        }

        // Standard operator new array, no-throw version
        void * operator new[](size_t _Size, const ::std::nothrow_t& _No_throw) noexcept
        {
            return operator new(_Size, _No_throw);
        }

        // Standard operator delete array, no-throw version
        void operator delete[](void * _Ptr, const ::std::nothrow_t& _No_throw) noexcept
        {
            operator delete(_Ptr, _No_throw);
        }

        // Standard operator new with void* placement
        void * operator new(size_t, void * _Location) noexcept
        {
            return _Location;
        }

        // Standard operator delete with void* placement
        void operator delete(void *, void *) noexcept
        {
        }

        // Standard operator new array with void* placement
        void * __cdecl operator new[](size_t, void * _Location) noexcept
        {
            return _Location;
        }

        // Standard operator delete array with void* placement
        void __cdecl operator delete[](void *, void *) noexcept
        {
        }
    };

    // Stubs to allow the header files to access runtime functionality for WINAPI_PARTITION apps.
    class _Context
    {
    public:
        _CONCRTIMP _Context(::Concurrency::Context * _PContext = nullptr) : _M_pContext(_PContext) {}
        _CONCRTIMP static _Context __cdecl _CurrentContext();
        _CONCRTIMP static void __cdecl _Yield();
        _CONCRTIMP static void __cdecl _Oversubscribe(bool _BeginOversubscription);
        _CONCRTIMP bool _IsSynchronouslyBlocked() const;
    private:
        ::Concurrency::Context * _M_pContext;
    };

    class _Scheduler
    {
    public:
        _CONCRTIMP _Scheduler(::Concurrency::Scheduler * _PScheduler = nullptr) : _M_pScheduler(_PScheduler) {}
        _CONCRTIMP unsigned int _Reference();
        _CONCRTIMP unsigned int _Release();
        _CONCRTIMP ::Concurrency::Scheduler * _GetScheduler() { return _M_pScheduler; }

    private:
        ::Concurrency::Scheduler * _M_pScheduler;
    };

    class _CurrentScheduler
    {
    public:
        _CONCRTIMP static void __cdecl _ScheduleTask(TaskProc _Proc, void * _Data);
        _CONCRTIMP static unsigned int __cdecl _Id();
        _CONCRTIMP static unsigned int __cdecl _GetNumberOfVirtualProcessors();
        _CONCRTIMP static _Scheduler __cdecl _Get();
    };

    //
    // Wrappers for atomic access
    //
    template <size_t _Size>
    struct _Subatomic_impl { };

    template<>
    struct _Subatomic_impl<4> {
        template <typename _Ty>
        static void _StoreWithRelease(volatile _Ty& _Location, _Ty _Rhs) {
            // For the compiler, a volatile write has release semantics. In addition, on ARM,
            // the volatile write will emit a data memory barrier before the write.
            _Location = _Rhs;
        }

        template <typename _Ty>
        static _Ty _LoadWithAquire(volatile _Ty& _Location) {
            // For the compiler, a volatile read has acquire semantics. In addition, on ARM,
            // the volatile read will emit a data memory barrier after the read.
            return _Location;
        }

        template <typename _Ty>
        static _Ty _CompareAndSwap(volatile _Ty& _Location, _Ty _NewValue, _Ty _Comperand) {
            return (_Ty)_InterlockedCompareExchange((volatile long*)&_Location, (long)_NewValue, (long)_Comperand);
        }

        template <typename _Ty>
        static _Ty _FetchAndAdd(volatile _Ty& _Location, _Ty _Addend) {
            return (_Ty)_InterlockedExchangeAdd((volatile long*)&_Location, (long)_Addend);
        }

        template <typename _Ty>
        static _Ty _Increment(volatile _Ty& _Location) {
            return (_Ty)_InterlockedIncrement((volatile long*)&_Location);
        }

        template <typename _Ty>
        static _Ty _Decrement(volatile _Ty& _Location) {
            return (_Ty)_InterlockedDecrement((volatile long*)&_Location);
        }
    };

#if defined (_WIN64)
    template<>
    struct _Subatomic_impl<8> {
        template <typename _Ty>
        static void _StoreWithRelease(volatile _Ty& _Location, _Ty _Rhs) {
            // For the compiler, a volatile write has release semantics.
            _Location = _Rhs;
        }

        template <typename _Ty>
        static _Ty _LoadWithAquire(volatile _Ty& _Location) {
            // For the compiler, a volatile read has acquire semantics.
            return _Location;
        }

        template <typename _Ty>
        static _Ty _CompareAndSwap(volatile _Ty& _Location, _Ty _NewValue, _Ty _Comperand) {
            return (_Ty)_InterlockedCompareExchange64((volatile __int64*)&_Location, (__int64)_NewValue, (__int64)_Comperand);
        }

        template <typename _Ty>
        static _Ty _FetchAndAdd(volatile _Ty& _Location, _Ty _Addend) {
            return (_Ty)_InterlockedExchangeAdd64((volatile __int64*)&_Location, (__int64)_Addend);
        }

        template <typename _Ty>
        static _Ty _Increment(volatile _Ty& _Location) {
            return (_Ty)_InterlockedIncrement64((volatile __int64*)&_Location);
        }

        template <typename _Ty>
        static _Ty _Decrement(volatile _Ty& _Location) {
            return (_Ty)_InterlockedDecrement64((volatile __int64*)&_Location);
        }
    };
#endif  /* defined (_M_X64) */


    //
    // Wrapper for atomic access. Only works for 4-byte or 8-byte types (for example, int, long, long long, size_t, pointer).
    // Anything else might fail to compile.
    //
    template <typename _Ty>
    class _Subatomic {
    private:
        volatile _Ty _M_value;

    public:
        operator _Ty() const volatile {
            return _Subatomic_impl<sizeof(_Ty)>::_LoadWithAquire(_M_value);
        }

        _Ty operator=(_Ty _Rhs) {
            _Subatomic_impl<sizeof(_Ty)>::_StoreWithRelease(_M_value, _Rhs);
            return _Rhs;
        }

        _Ty _CompareAndSwap(_Ty _NewValue, _Ty _Comperand) {
            return _Subatomic_impl<sizeof(_Ty)>::_CompareAndSwap(_M_value, _NewValue, _Comperand);
        }

        _Ty _FetchAndAdd(_Ty _Addend) {
            return _Subatomic_impl<sizeof(_Ty)>::_FetchAndAdd(_M_value, _Addend);
        }

        _Ty operator++() {
            return _Subatomic_impl<sizeof(_Ty)>::_Increment(_M_value);
        }

        _Ty operator++(int) {
            return _Subatomic_impl<sizeof(_Ty)>::_Increment(_M_value) - 1;
        }

        _Ty operator--() {
            return _Subatomic_impl<sizeof(_Ty)>::_Decrement(_M_value);
        }

        _Ty operator--(int) {
            return _Subatomic_impl<sizeof(_Ty)>::_Decrement(_M_value) + 1;
        }

        _Ty operator+=(_Ty _Addend) {
            return _FetchAndAdd(_Addend) + _Addend;
        }
    };

    //
    // An RAII class that spin-waits on a "rented" flag.
    //
    class _SpinLock
    {
    private:
        volatile long& _M_flag;

    public:
        _CONCRTIMP _SpinLock(volatile long& _Flag);
        _CONCRTIMP ~_SpinLock();

    private:
        _SpinLock(const _SpinLock&);
        void operator=(const _SpinLock&);
    };

    //
    // A class that holds the count used for spinning and is dependent
    // on the number of hardware threads
    //
    struct _SpinCount
    {
        // Initializes the spinCount to either 0 or SPIN_COUNT, depending on
        // the number of hardware threads.
        static void __cdecl _Initialize();

        // Returns the current value of s_spinCount
        _CONCRTIMP static unsigned int __cdecl _Value();

        // The number of iterations used for spinning
        static unsigned int _S_spinCount;
    };

    /// <summary>
    ///     Default method for yielding during a spin wait
    /// </summary>
    /**/
    void _CONCRTIMP __cdecl _UnderlyingYield();

    /// <summary>
    ///     Returns the hardware concurrency available to the Concurrency Runtime, taking into account process affinity, or any restrictions
    ///     in place because of the set_task_execution_resources method.
    /// </summary>
    /**/
    unsigned int _CONCRTIMP __cdecl _GetConcurrency();

    /// <summary>
    ///     Implements busy wait with no backoff
    /// </summary>
    /**/
    template<unsigned int _YieldCount = 1>
    class _CONCRTIMP _SpinWait
    {
    public:

        typedef void (__cdecl *_YieldFunction)();

        /// <summary>
        ///     Construct a spin wait object
        /// </summary>
        /**/
        _SpinWait(_YieldFunction _YieldMethod = _UnderlyingYield)
            : _M_state(_StateInitial), _M_yieldFunction(_YieldMethod)
        {
            // Defer initialization of other fields to _SpinOnce().
        }

        /// <summary>
        ///     Set a dynamic spin count.
        /// </summary>
        /**/
        void _SetSpinCount(unsigned int _Count)
        {
            _CONCRT_ASSERT(_M_state == _StateInitial);
            if (_Count == 0)
            {
                // Specify a count of 0 if we are on a single proc.
                _M_state = _StateSingle;
            }
            else
            {
                _M_currentSpin = _Count;
                _M_currentYield = _YieldCount;
                _M_state = _StateSpin;
            }
        }

        /// <summary>
        ///     Spins for one time quantum,until a maximum spin is reached.
        /// </summary>
        /// <returns>
        ///     false if spin count has reached steady state, true otherwise.
        /// </returns>
        /// <remarks>
        ///     If the spin count is not changing do not spin again
        ///     because there is either only one processor, or the maximum spin has been reached and blocking is
        ///     probably a better solution. However, if called again, SpinOnce will spin for a maximum spin count.
        /// </remarks>
        /**/
        bool _SpinOnce()
        {
            switch (_M_state)
            {
            case _StateSpin:
            {
                unsigned long _Count = _NumberOfSpins();

                for (unsigned long _I = 0; _I < _Count; _I++)
                {
                    _YieldProcessor();
                }

                if (!_ShouldSpinAgain())
                {
                    _M_state = (_M_currentYield == 0) ? _StateBlock : _StateYield;
                }

                return true;
            }

            case _StateYield:
                _CONCRT_ASSERT(_M_currentYield > 0);
                if (--_M_currentYield == 0)
                {
                    _M_state = _StateBlock;
                }

                // Execute the yield
                _DoYield();
                return true;

            case _StateBlock:
                // Reset to defaults if client does not block
                _Reset();
                return false;

            case _StateSingle:
                // No need to spin on a single processor: just execute the yield
                _DoYield();
                return false;

            case _StateInitial:
                // Reset counters to their default value and Spin once.
                _Reset();
                return _SpinOnce();
            default:
                // Unreached
                return false;
            };
        }

    protected:

        /// <summary>
        ///     State of the spin wait class.
        /// </summary>
        /**/
        enum _SpinState
        {
            _StateInitial,
            _StateSpin,
            _StateYield,
            _StateBlock,
            _StateSingle
        };

        /// <summary>
        ///     Yields its time slice using the specified yieldFunction
        /// </summary>
        /**/
        void _DoYield()
        {
            constexpr bool _ShouldYield = (_YieldCount != 0);
            if constexpr (_ShouldYield)
            {
                _CONCRT_ASSERT(_M_yieldFunction != nullptr);
                _M_yieldFunction();
            }
            else
            {
                _YieldProcessor();
            }
        }

        /// <summary>
        ///     Resets the counts and state to the default.
        /// </summary>
        /**/
        void _Reset()
        {
            _M_state = _StateInitial;

            // Reset to the default spin value. The value specified
            // by the client is ignored on a reset.
            _SetSpinCount(_SpinCount::_Value());

            _CONCRT_ASSERT(_M_state != _StateInitial);
        }

        /// <summary>
        ///     Determines the current spin count
        /// </summary>
        /// <returns>
        ///     The number of spins to execute for this iteration
        /// </returns>
        /**/
        unsigned long _NumberOfSpins()
        {
            return 1;
        }

        /// <summary>
        ///     Determines whether maximum spin has been reached
        /// </summary>
        /// <returns>
        ///     false if spin count has reached steady state, true otherwise.
        /// </returns>
        /**/
        bool _ShouldSpinAgain()
        {
            return (--_M_currentSpin > 0);
        }

        unsigned long  _M_currentSpin{};
        unsigned long  _M_currentYield{};
        _SpinState     _M_state;
        _YieldFunction _M_yieldFunction;
    };

    typedef _SpinWait<>   _SpinWaitBackoffNone;
    typedef _SpinWait<0>  _SpinWaitNoYield;

    //
    // This reentrant lock uses CRITICAL_SECTION and is intended for use when kernel blocking
    // is desirable and where it is either known that the lock will be taken recursively in
    // the same thread, or not known that a non-reentrant lock can be used safely.
    //
    class _ReentrantBlockingLock
    {
    public:
        // Constructor for _ReentrantBlockingLock
        _CONCRTIMP _ReentrantBlockingLock();

        // Destructor for _ReentrantBlockingLock
        _CONCRTIMP ~_ReentrantBlockingLock();

        // Acquire the lock, spin if necessary
        _CONCRTIMP void _Acquire();

        // Tries to acquire the lock, does not spin.
        // Returns true if the acquisition worked, false otherwise
        _CONCRTIMP bool _TryAcquire();

        // Releases the lock
        _CONCRTIMP void _Release();


        // An exception safe RAII wrapper.
        class _Scoped_lock
        {
        public:
            // Constructs a holder and acquires the specified lock
            explicit _Scoped_lock(_ReentrantBlockingLock& _Lock) : _M_lock(_Lock)
            {
                _M_lock._Acquire();
            }

            // Destroys the holder and releases the lock
            ~_Scoped_lock()
            {
                _M_lock._Release();
            }
        private:
            _ReentrantBlockingLock& _M_lock;

            _Scoped_lock(const _Scoped_lock&);                    // no copy constructor
            _Scoped_lock const & operator=(const _Scoped_lock&);  // no assignment operator
        };

    private:
        // Critical section requires windows.h. Hide the implementation so that
        // user code need not include windows.
        _CONCRT_BUFFER _M_criticalSection[(4 * sizeof(void *) + 2 * sizeof(long) + sizeof(_CONCRT_BUFFER) - 1) / sizeof(_CONCRT_BUFFER)]{};
    };

    //
    // This reentrant lock is a pure spin lock and is intended for use when kernel blocking
    // is desirable and where it is either known that the lock will be taken recursively in
    // the same thread, or not known that a non-reentrant lock can be used safely.
    //
    class _ReentrantLock
    {
    public:
        // Constructor for _ReentrantLock
        _CONCRTIMP _ReentrantLock();

        // Acquire the lock, spin if necessary
        _CONCRTIMP void _Acquire();

        // Tries to acquire the lock, does not spin
        // Returns true if the acquisition worked, false otherwise
        _CONCRTIMP bool _TryAcquire();

        // Releases the lock
        _CONCRTIMP void _Release();

        // An exception safe RAII wrapper.
        class _Scoped_lock
        {
        public:
            // Constructs a holder and acquires the specified lock
            explicit _Scoped_lock(_ReentrantLock& _Lock) : _M_lock(_Lock)
            {
                _M_lock._Acquire();
            }

            // Destroys the holder and releases the lock
            ~_Scoped_lock()
            {
                _M_lock._Release();
            }
        private:
            _ReentrantLock& _M_lock;

            _Scoped_lock(const _Scoped_lock&);                    // no copy constructor
            _Scoped_lock const & operator=(const _Scoped_lock&);  // no assignment operator
        };

    private:
        long _M_recursionCount;
        volatile long _M_owner;
    };

    //
    // This non-reentrant lock uses CRITICAL_SECTION and is intended for use in situations
    // where it is known that the lock will not be taken recursively, and can be more
    // efficiently implemented.
    //
    class _NonReentrantBlockingLock
    {
    public:
        // Constructor for _NonReentrantBlockingLock
        //
        // The constructor is exported because _NonReentrantLock is
        // included in DevUnitTests.
        _CONCRTIMP _NonReentrantBlockingLock();

        // Constructor for _NonReentrantBlockingLock
        _CONCRTIMP ~_NonReentrantBlockingLock();

        // Acquire the lock, spin if necessary
        _CONCRTIMP void _Acquire();

        // Tries to acquire the lock, does not spin
        // Returns true if the lock is taken, false otherwise
        _CONCRTIMP bool _TryAcquire();

        // Releases the lock
        _CONCRTIMP void _Release();

        // An exception safe RAII wrapper.
        class _Scoped_lock
        {
        public:
            // Constructs a holder and acquires the specified lock
            explicit _Scoped_lock(_NonReentrantBlockingLock& _Lock) : _M_lock(_Lock)
            {
                _M_lock._Acquire();
            }

            // Destroys the holder and releases the lock
            ~_Scoped_lock()
            {
                _M_lock._Release();
            }
        private:
            _NonReentrantBlockingLock& _M_lock;

            _Scoped_lock(const _Scoped_lock&);                    // no copy constructor
            _Scoped_lock const & operator=(const _Scoped_lock&);  // no assignment operator
        };

    private:
        // Critical section requires windows.h. Hide the implementation so that
        // user code need not include windows.h
        _CONCRT_BUFFER _M_criticalSection[(4 * sizeof(void *) + 2 * sizeof(long) + sizeof(_CONCRT_BUFFER) - 1) / sizeof(_CONCRT_BUFFER)]{};
    };

    //
    // A Reader-Writer Lock is intended for use in situations with many readers and rare
    // writers.
    //
    // A writer request immediately blocks future readers and then waits until all current
    // readers drain. A reader request does not block future writers and must wait until
    // all writers are done, even those that cut in front of it. In any race between a
    // reader and a writer, the writer always wins.
    //
    class _ReaderWriterLock
    {
    public:
        // Constructor for _ReaderWriterLock
        //
        // The constructor and destructor are exported because _ReaderWriterLock is
        // included in DevUnitTests.
        _CONCRTIMP _ReaderWriterLock();

        // Acquire lock for reading. Spins until all writers finish, new writers
        // can cut in front of a waiting reader.
        _CONCRTIMP void _AcquireRead();

        // Release lock for reading. The last reader changes m_state to State.kFree
        _CONCRTIMP void _ReleaseRead();

        // Acquire lock for writing. Spin until no readers exist, then acquire lock
        // and prevent new readers.
        _CONCRTIMP void _AcquireWrite();

        // Release lock for writing.
        _CONCRTIMP void _ReleaseWrite();

        // Try to acquire the write lock, do not spin if unable to acquire.
        // Returns true if the acquisition worked, false otherwise
        _CONCRTIMP bool _TryAcquireWrite();

        // Returns true if it is in write state, false otherwise
        bool _HasWriteLock() const
        {
            return (_M_state == _Write);
        }

        // Guarantees that all writers are out of the lock. This does nothing if there are no pending writers.
        void _FlushWriteOwners();

        // An exception safe RAII wrapper.
        class _Scoped_lock
        {
        public:
            // Constructs a holder and acquires the writer lock
            explicit _Scoped_lock(_ReaderWriterLock& _Lock) : _M_lock(_Lock)
            {
                _M_lock._AcquireWrite();
            }

            // Destroys the holder and releases the writer lock
            ~_Scoped_lock()
            {
                _M_lock._ReleaseWrite();
            }

        private:

            _ReaderWriterLock& _M_lock;

            _Scoped_lock(const _Scoped_lock&);                    // no copy constructor
            _Scoped_lock const & operator=(const _Scoped_lock&);  // no assignment operator
        };

        // An exception safe RAII wrapper for reads.
        class _Scoped_lock_read
        {
        public:
            // Constructs a holder and acquires the reader lock
            explicit _Scoped_lock_read(_ReaderWriterLock& _Lock) : _M_lock(_Lock)
            {
                _M_lock._AcquireRead();
            }

            // Destroys the holder and releases the reader lock
            ~_Scoped_lock_read()
            {
                _M_lock._ReleaseRead();
            }

        private:

            _ReaderWriterLock& _M_lock;

            _Scoped_lock_read(const _Scoped_lock_read&);                    // no copy constructor
            _Scoped_lock_read const & operator=(const _Scoped_lock_read&);  // no assignment operator
        };

    private:
        // State enum where:
        // -1    --> write mode
        // 0     --> free
        // n > 0 --> n readers have locked in read mode.
        enum _State
        {
          _Write = -1,
          _Free  = 0,
          _Read  = 1
        };

        // The current state of the lock, mapping to the State enum. This is also
        // an indicator of the number of readers holding the lock, for any number > 0.
        volatile long _M_state;

        // A writer increments this as soon as it wants to lock and decrements this
        // after releasing the lock. To prevent writers from starving, a reader will
        // wait until this counter is zero, and only then will try to obtain the lock.
        volatile long _M_numberOfWriters;

        // Spin-Wait-Until variant
        static void __cdecl _WaitEquals(volatile const long& _Location, long _Value, long _Mask = 0xFFFFFFFF);
    };

    //
    // Exception safe RAII wrappers for _malloca()
    //

    //
    // _MallocaArrayHolder is used when the allocation size is known up front, and the memory must be allocated in a contiguous space
    //
    template<typename _ElemType>
    class _MallocaArrayHolder
    {
    public:

        _MallocaArrayHolder() : _M_ElemArray(nullptr), _M_ElemsConstructed(0) {}

        // _Initialize takes the pointer to the memory allocated by the user using _malloca
        void _Initialize(_ElemType * _Elem)
        {
            // The object must be initialized exactly once
            _CONCRT_ASSERT(_M_ElemArray == nullptr && _M_ElemsConstructed == 0);
            _M_ElemArray = _Elem;
            _M_ElemsConstructed = 0;
        }

        // _InitOnRawMalloca take the raw pointer returned by _malloca directly
        // It will initialize itself with that pointer and return a strong typed pointer.
        // To be noted that the constructor will NOT be called.
        _ElemType * _InitOnRawMalloca(void * _MallocaRet)
        {
            if (_MallocaRet == nullptr)
                throw ::std::bad_alloc();
            _Initialize(static_cast<_ElemType *>(_MallocaRet));
            return static_cast<_ElemType *>(_MallocaRet);
        }

        // Register the next slot for destruction. Because we only keep the index of the last slot to be destructed,
        // this method must be called sequentially from 0 to N where N < _ElemCount.
        void _IncrementConstructedElemsCount()
        {
            _CONCRT_ASSERT(_M_ElemArray != nullptr); // must already be initialized
            _M_ElemsConstructed++;
        }

        virtual ~_MallocaArrayHolder()
        {
            for( size_t _I=0; _I < _M_ElemsConstructed; ++_I )
            {
                _M_ElemArray[_I].~_ElemType();
            }
            // Works even when object was not initialized, that is, _M_ElemArray == NULL
            _freea(_M_ElemArray);
        }
    private:
        _ElemType * _M_ElemArray;
        size_t     _M_ElemsConstructed;

        // Copy construction and assignment are not supported.
        _MallocaArrayHolder(const _MallocaArrayHolder & );
        _MallocaArrayHolder&  operator = (const _MallocaArrayHolder & );
    };

    //
    // _MallocaListHolder is used when the allocation size is not known up front, and the elements are added to the list dynamically
    //
    template<typename _ElemType>
    class _MallocaListHolder
    {
    public:
        // Returns the size required to allocate the payload itself and the pointer to the next element
        size_t _GetAllocationSize() const
        {
            return sizeof(_ElemNodeType);
        }

        _MallocaListHolder() : _M_FirstNode(nullptr)
        {
        }

        // Add the next element to the list. The memory is allocated in the caller's frame by _malloca
        void _AddNode(_ElemType * _Elem)
        {
            _ElemNodeType * _Node = reinterpret_cast<_ElemNodeType *>(_Elem);
            _Node->_M_Next = _M_FirstNode;
            _M_FirstNode = reinterpret_cast<_ElemNodeType *>(_Elem);
        }

        // _AddRawMallocaNode take the raw pointer returned by _malloca directly
        // It will add that bucket of memory to the list and return a strong typed pointer.
        // To be noted that the constructor will NOT be called.
        _ElemType * _AddRawMallocaNode(void * _MallocaRet)
        {
            if (_MallocaRet == nullptr)
                throw ::std::bad_alloc();
            _AddNode(static_cast<_ElemType *>(_MallocaRet));
            return static_cast<_ElemType *>(_MallocaRet);
        }

        // Walk the list and destruct, then free each element
        _At_(this->_M_FirstNode, _Pre_valid_) virtual ~_MallocaListHolder()
        {
            for( _ElemNodeType * _Node = _M_FirstNode; _Node != nullptr; )
            {
                auto _M_Next = _Node->_M_Next;
                _Node->_M_Elem._ElemType::~_ElemType();
                 _freea(_Node);
                _Node = _M_Next;
            }
        }

    private:

        class _ElemNodeType
        {
            friend class _MallocaListHolder;
            _ElemType _M_Elem;
            _ElemNodeType * _M_Next;
            // Always instantiated using malloc, so default constructor and destructor are not needed.
            _ElemNodeType();
            ~_ElemNodeType();
            // Copy construction and assignment are not supported.
            _ElemNodeType(const _ElemNodeType & );
            _ElemNodeType &  operator = (const _ElemNodeType & );
        };

        _ElemNodeType* _M_FirstNode;

        // Copy construction and assignment are not supported.
        _MallocaListHolder(const _MallocaListHolder & );
        _MallocaListHolder &  operator = (const _MallocaListHolder & );
    };

    // Forward declarations
    class _StructuredTaskCollection;
    class _TaskCollection;
    class _UnrealizedChore;
} // namespace details

//**************************************************************************
// Public Namespace:
//
// Anything in the Concurrency namespace is intended for direct client consumption.
//
//**************************************************************************

/// <summary>
///     This class describes an exception thrown because of a failure to acquire a critical resource in the Concurrency Runtime.
/// </summary>
/// <remarks>
///     This exception is typically thrown when a call to the operating system from within the Concurrency Runtime
///     fails. The error code which would normally be returned from a call to the Win32 method <c>GetLastError</c> is
///     converted to a value of type <c>HRESULT</c> and can be retrieved using the <c>get_error_code</c> method.
/// </remarks>
/**/
class scheduler_resource_allocation_error : public ::std::exception
{
public:
    /// <summary>
    ///     Constructs a <c>scheduler_resource_allocation_error</c> object.
    /// </summary>
    /// <param name="_Message">
    ///     A descriptive message of the error.
    /// </param>
    /// <param name="_Hresult">
    ///     The <c>HRESULT</c> value of the error that caused the exception.
    /// </param>
    /**/
    _CONCRTIMP scheduler_resource_allocation_error(_In_z_ const char * _Message, HRESULT _Hresult) noexcept;

    /// <summary>
    ///     Constructs a <c>scheduler_resource_allocation_error</c> object.
    /// </summary>
    /// <param name="_Hresult">
    ///     The <c>HRESULT</c> value of the error that caused the exception.
    /// </param>
    /**/
    explicit _CONCRTIMP scheduler_resource_allocation_error(HRESULT _Hresult) noexcept;

    /// <summary>
    ///     Returns the error code that caused the exception.
    /// </summary>
    /// <returns>
    ///     The <c>HRESULT</c> value of the error that caused the exception.
    /// </returns>
    /**/
    _CONCRTIMP HRESULT get_error_code() const noexcept;

private:
    HRESULT _Hresult;
};

/// <summary>
///     This class describes an exception thrown because of a failure to create a worker execution context in the Concurrency Runtime.
/// </summary>
/// <remarks>
///     This exception is typically thrown when a call to the operating system to create execution contexts from within the Concurrency Runtime
///     fails. Execution contexts are threads that execute tasks in the Concurrency Runtime. The error code which would normally be returned
///     from a call to the Win32 method <c>GetLastError</c> is converted to a value of type <c>HRESULT</c> and can be retrieved using the base
///     class method <c>get_error_code</c>.
/// </remarks>
/**/
class scheduler_worker_creation_error : public scheduler_resource_allocation_error
{
public:
    /// <summary>
    ///     Constructs a <c>scheduler_worker_creation_error</c> object.
    /// </summary>
    /// <param name="_Message">
    ///     A descriptive message of the error.
    /// </param>
    /// <param name="_Hresult">
    ///     The <c>HRESULT</c> value of the error that caused the exception.
    /// </param>
    /**/
    _CONCRTIMP scheduler_worker_creation_error(_In_z_ const char * _Message, HRESULT _Hresult) noexcept;

    /// <summary>
    ///     Constructs a <c>scheduler_worker_creation_error</c> object.
    /// </summary>
    /// <param name="_Hresult">
    ///     The <c>HRESULT</c> value of the error that caused the exception.
    /// </param>
    /**/
    explicit _CONCRTIMP scheduler_worker_creation_error(HRESULT _Hresult) noexcept;
};

/// <summary>
///     This class describes an exception thrown when an unsupported operating system is used.
/// </summary>
/**/
class unsupported_os  : public ::std::exception
{
public:
    /// <summary>
    ///     Constructs an <c>unsupported_os</c> object.
    /// </summary>
    /// <param name="_Message">
    ///     A descriptive message of the error.
    /// </param>
    /**/
    explicit _CONCRTIMP unsupported_os(_In_z_ const char * _Message) noexcept;

    /// <summary>
    ///     Constructs an <c>unsupported_os</c> object.
    /// </summary>
    /**/
    _CONCRTIMP unsupported_os() noexcept;
};

/// <summary>
///     This class describes an exception thrown when an operation is performed which requires a scheduler
///     to be attached to the current context and one is not.
/// </summary>
/// <seealso cref="Scheduler Class"/>
/// <seealso cref="Scheduler::Attach Method"/>
/**/
class scheduler_not_attached  : public ::std::exception
{
public:
    /// <summary>
    ///     Constructs a <c>scheduler_not_attached</c> object.
    /// </summary>
    /// <param name="_Message">
    ///     A descriptive message of the error.
    /// </param>
    /**/
    explicit _CONCRTIMP scheduler_not_attached(_In_z_ const char * _Message) noexcept;

    /// <summary>
    ///     Constructs a <c>scheduler_not_attached</c> object.
    /// </summary>
    /**/
    _CONCRTIMP scheduler_not_attached() noexcept;
};

/// <summary>
///     This class describes an exception thrown when the <c>Attach</c> method is called on a <c>Scheduler</c>
///     object which is already attached to the current context.
/// </summary>
/// <seealso cref="Scheduler Class"/>
/// <seealso cref="Scheduler::Attach Method"/>
/**/
class improper_scheduler_attach : public ::std::exception
{
public:
    /// <summary>
    ///     Constructs an <c>improper_scheduler_attach</c> object.
    /// </summary>
    /// <param name="_Message">
    ///     A descriptive message of the error.
    /// </param>
    /**/
    explicit _CONCRTIMP improper_scheduler_attach(_In_z_ const char * _Message) noexcept;

    /// <summary>
    ///     Constructs an <c>improper_scheduler_attach</c> object.
    /// </summary>
    /**/
    _CONCRTIMP improper_scheduler_attach() noexcept;
};

/// <summary>
///     This class describes an exception thrown when the <c>CurrentScheduler::Detach</c> method is called on
///     a context which has not been attached to any scheduler using the <c>Attach</c> method of a <c>Scheduler</c> object.
/// </summary>
/// <seealso cref="Scheduler Class"/>
/// <seealso cref="CurrentScheduler::Detach Method"/>
/// <seealso cref="Scheduler::Attach Method"/>
/**/
class improper_scheduler_detach : public ::std::exception
{
public:

    /// <summary>
    ///     Constructs an <c>improper_scheduler_detach</c> object.
    /// </summary>
    /// <param name="_Message">
    ///     A descriptive message of the error.
    /// </param>
    /**/
    explicit _CONCRTIMP improper_scheduler_detach(_In_z_ const char * _Message) noexcept;

    /// <summary>
    ///     Constructs an <c>improper_scheduler_detach</c> object.
    /// </summary>
    /**/
    _CONCRTIMP improper_scheduler_detach() noexcept;
};

/// <summary>
///     This class describes an exception thrown when the <c>Reference</c> method is called on a <c>Scheduler</c>
///     object that is shutting down, from a context that is not part of that scheduler.
/// </summary>
/// <seealso cref="Scheduler Class"/>
/// <seealso cref="Scheduler::Reference Method"/>
/**/
class improper_scheduler_reference : public ::std::exception
{
public:

    /// <summary>
    ///     Constructs an <c>improper_scheduler_reference</c> object.
    /// </summary>
    /// <param name="_Message">
    ///     A descriptive message of the error.
    /// </param>
    /**/
    explicit _CONCRTIMP improper_scheduler_reference(_In_z_ const char* _Message) noexcept;

    /// <summary>
    ///     Constructs an <c>improper_scheduler_reference</c> object.
    /// </summary>
    /**/
    _CONCRTIMP improper_scheduler_reference() noexcept;
};

/// <summary>
///     This class describes an exception thrown when the <c>Scheduler::SetDefaultSchedulerPolicy</c> method is
///     called when a default scheduler already exists within the process.
/// </summary>
/// <seealso cref="Scheduler::SetDefaultSchedulerPolicy Method"/>
/**/
class default_scheduler_exists : public ::std::exception
{
public:
    /// <summary>
    ///     Constructs a <c>default_scheduler_exists</c> object.
    /// </summary>
    /// <param name="_Message">
    ///     A descriptive message of the error.
    /// </param>
    /**/
    explicit _CONCRTIMP default_scheduler_exists(_In_z_ const char * _Message) noexcept;

    /// <summary>
    ///     Constructs a <c>default_scheduler_exists</c> object.
    /// </summary>
    /**/
    _CONCRTIMP default_scheduler_exists() noexcept;
};

/// <summary>
///     This class describes an exception thrown when calls to the <c>Block</c> and <c>Unblock</c> methods of a
///     <c>Context</c> object are not properly paired.
/// </summary>
/// <remarks>
///     Calls to the <c>Block</c> and <c>Unblock</c> methods of a <c>Context</c> object must always be properly paired.
///     The Concurrency Runtime allows the operations to happen in either order. For example, a call to <c>Block</c>
///     can be followed by a call to <c>Unblock</c>, or vice-versa. This exception would be thrown if, for instance, two calls to the
///     <c>Unblock</c> method were made in a row, on a <c>Context</c> object which was not blocked.
/// </remarks>
/// <seealso cref="Context Class"/>
/// <seealso cref="Context::Unblock Method"/>
/// <seealso cref="Context::Block Method"/>
/**/
class context_unblock_unbalanced : public ::std::exception
{
public:
    /// <summary>
    ///     Constructs a <c>context_unblock_unbalanced</c> object.
    /// </summary>
    /// <param name="_Message">
    ///     A descriptive message of the error.
    /// </param>
    /**/
    explicit _CONCRTIMP context_unblock_unbalanced(_In_z_ const char * _Message) noexcept;

    /// <summary>
    ///     Constructs a <c>context_unblock_unbalanced</c> object.
    /// </summary>
    /**/
    _CONCRTIMP context_unblock_unbalanced() noexcept;
};

/// <summary>
///     This class describes an exception thrown when the <c>Unblock</c> method of a <c>Context</c> object is called
///     from the same context. This would indicate an attempt by a given context to unblock itself.
/// </summary>
/// <seealso cref="Context Class"/>
/// <seealso cref="Context::Unblock Method"/>
/**/
class context_self_unblock : public ::std::exception
{
public:
    /// <summary>
    ///     Constructs a <c>context_self_unblock</c> object.
    /// </summary>
    /// <param name="_Message">
    ///     A descriptive message of the error.
    /// </param>
    /**/
    explicit _CONCRTIMP context_self_unblock(_In_z_ const char * _Message) noexcept;

    /// <summary>
    ///     Constructs a <c>context_self_unblock</c> object.
    /// </summary>
    /**/
    _CONCRTIMP context_self_unblock() noexcept;
};

/// <summary>
///     This class describes an exception thrown when there are tasks still scheduled to a <c>task_group</c> or
///     <c>structured_task_group</c> object at the time that object's destructor executes. This exception will never be thrown
///     if the destructor is reached because of a stack unwinding as the result of an exception.
/// </summary>
/// <remarks>
///     Absent exception flow, you are responsible for calling either the <c>wait</c> or <c>run_and_wait</c> method of a <c>task_group</c> or
///     <c>structured_task_group</c> object before allowing that object to destruct. The runtime throws this exception as an
///     indication that you forgot to call the <c>wait</c> or <c>run_and_wait</c> method.
/// </remarks>
/// <seealso cref="task_group Class"/>
/// <seealso cref="task_group::wait Method"/>
/// <seealso cref="task_group::run_and_wait Method"/>
/// <seealso cref="structured_task_group Class"/>
/// <seealso cref="structured_task_group::wait Method"/>
/// <seealso cref="structured_task_group::run_and_wait Method"/>
/**/
class missing_wait : public ::std::exception
{
public:
    /// <summary>
    ///     Constructs a <c>missing_wait</c> object.
    /// </summary>
    /// <param name="_Message">
    ///     A descriptive message of the error.
    /// </param>
    /**/
    explicit _CONCRTIMP missing_wait(_In_z_ const char * _Message) noexcept;

    /// <summary>
    ///     Constructs a <c>missing_wait</c> object.
    /// </summary>
    /**/
    _CONCRTIMP missing_wait() noexcept;
};

/// <summary>
///     This class describes an exception thrown when a messaging block is given a pointer to a target which is
///     invalid for the operation being performed.
/// </summary>
/// <remarks>
///     This exception is typically thrown for reasons such as a target attempting to consume a message which is reserved
///     for a different target or releasing a reservation that it does not hold.
/// </remarks>
/// <seealso cref="Asynchronous Message Blocks"/>
/**/
class bad_target : public ::std::exception
{
public:
    /// <summary>
    ///     Constructs a <c>bad_target</c> object.
    /// </summary>
    /// <param name="_Message">
    ///     A descriptive message of the error.
    /// </param>
    /**/
    explicit _CONCRTIMP bad_target(_In_z_ const char * _Message) noexcept;

    /// <summary>
    ///     Constructs a <c>bad_target</c> object.
    /// </summary>
    /**/
    _CONCRTIMP bad_target() noexcept;
};

/// <summary>
///     This class describes an exception thrown when a messaging block is unable to find a requested message.
/// </summary>
/// <seealso cref="Asynchronous Message Blocks"/>
/**/
class message_not_found : public ::std::exception
{
public:
    /// <summary>
    ///     Constructs a <c>message_not_found</c> object.
    /// </summary>
    /// <param name="_Message">
    ///     A descriptive message of the error.
    /// </param>
    /**/
    explicit _CONCRTIMP message_not_found(_In_z_ const char * _Message) noexcept;

    /// <summary>
    ///     Constructs a <c>message_not_found</c> object.
    /// </summary>
    /**/
    _CONCRTIMP message_not_found() noexcept;
};

/// <summary>
///     This class describes an exception thrown when the <c>link_target</c> method of a messaging block is
///     called and the messaging block is unable to link to the target. This can be the result of exceeding the number of
///     links the messaging block is allowed or attempting to link a specific target twice to the same source.
/// </summary>
/// <seealso cref="Asynchronous Message Blocks"/>
/**/
class invalid_link_target : public ::std::exception
{
public:
    /// <summary>
    ///     Constructs an <c>invalid_link_target</c> object.
    /// </summary>
    /// <param name="_Message">
    ///     A descriptive message of the error.
    /// </param>
    /**/
    explicit _CONCRTIMP invalid_link_target(_In_z_ const char * _Message) noexcept;

    /// <summary>
    ///     Constructs an <c>invalid_link_target</c> object.
    /// </summary>
    /**/
    _CONCRTIMP invalid_link_target() noexcept;
};

/// <summary>
///     This class describes an exception thrown when an invalid or unknown key is passed to a <c>SchedulerPolicy</c>
///     object constructor, or the <c>SetPolicyValue</c> method of a <c>SchedulerPolicy</c> object is passed a key that must
///     be changed using other means such as the <c>SetConcurrencyLimits</c> method.
/// </summary>
/// <seealso cref="SchedulerPolicy Class"/>
/// <seealso cref="PolicyElementKey Enumeration"/>
/// <seealso cref="SchedulerPolicy::SetPolicyValue Method"/>
/// <seealso cref="SchedulerPolicy::SetConcurrencyLimits Method"/>
/**/
class invalid_scheduler_policy_key : public ::std::exception
{
public:
    /// <summary>
    ///     Constructs an <c>invalid_scheduler_policy_key</c> object.
    /// </summary>
    /// <param name="_Message">
    ///     A descriptive message of the error.
    /// </param>
    /**/
    explicit _CONCRTIMP invalid_scheduler_policy_key(_In_z_ const char * _Message) noexcept;

    /// <summary>
    ///     Constructs an <c>invalid_scheduler_policy_key</c> object.
    /// </summary>
    /**/
    _CONCRTIMP invalid_scheduler_policy_key() noexcept;
};

/// <summary>
///     This class describes an exception thrown when a policy key of a <c>SchedulerPolicy</c> object is
///     set to an invalid value for that key.
/// </summary>
/// <seealso cref="SchedulerPolicy Class"/>
/// <seealso cref="PolicyElementKey Enumeration"/>
/// <seealso cref="SchedulerPolicy::SetPolicyValue Method"/>
/// <seealso cref="SchedulerPolicy::SetConcurrencyLimits Method"/>
/**/
class invalid_scheduler_policy_value : public ::std::exception
{
public:
    /// <summary>
    ///     Constructs an <c>invalid_scheduler_policy_value</c> object.
    /// </summary>
    /// <param name="_Message">
    ///     A descriptive message of the error.
    /// </param>
    /**/
    explicit _CONCRTIMP invalid_scheduler_policy_value(_In_z_ const char * _Message) noexcept;

    /// <summary>
    ///     Constructs an <c>invalid_scheduler_policy_value</c> object.
    /// </summary>
    /**/
    _CONCRTIMP invalid_scheduler_policy_value() noexcept;
};

/// <summary>
///     This class describes an exception thrown when an attempt is made to set the concurrency limits of a
///     <c>SchedulerPolicy</c> object such that the value of the <c>MinConcurrency</c> key is less than the value of the
///     <c>MaxConcurrency</c> key.
/// </summary>
/// <seealso cref="SchedulerPolicy Class"/>
/// <seealso cref="PolicyElementKey Enumeration"/>
/// <seealso cref="SchedulerPolicy::SetConcurrencyLimits Method"/>
/**/
class invalid_scheduler_policy_thread_specification : public ::std::exception
{
public:
    /// <summary>
    ///     Constructs an <c>invalid_scheduler_policy_value</c> object.
    /// </summary>
    /// <param name="_Message">
    ///     A descriptive message of the error.
    /// </param>
    /**/
    explicit _CONCRTIMP invalid_scheduler_policy_thread_specification(_In_z_ const char * _Message) noexcept;

    /// <summary>
    ///     Constructs an <c>invalid_scheduler_policy_value</c> object.
    /// </summary>
    /**/
    _CONCRTIMP invalid_scheduler_policy_thread_specification() noexcept;
};

/// <summary>
///     This class describes an exception thrown when the Concurrency Runtime detects that you neglected to call the
///     <c>CurrentScheduler::Detach</c> method on a context that attached to a second scheduler using the <c>Attach</c> method
///     of the <c>Scheduler</c> object.
/// </summary>
/// <remarks>
///     This exception is thrown only when you nest one scheduler inside another by calling the <c>Attach</c> method of a
///     <c>Scheduler</c> object on a context that is already owned by or attached to another scheduler. The Concurrency Runtime
///     throws this exception opportunistically when it can detect the scenario as an aid to locating the problem. Not every
///     instance of neglecting to call the <c>CurrentScheduler::Detach</c> method is guaranteed to throw this exception.
/// </remarks>
/// <seealso cref="Scheduler Class"/>
/// <seealso cref="CurrentScheduler::Detach Method"/>
/// <seealso cref="Scheduler::Attach Method"/>
/**/
class nested_scheduler_missing_detach : public ::std::exception
{
public:
    /// <summary>
    ///     Constructs a <c>nested_scheduler_missing_detach</c> object.
    /// </summary>
    /// <param name="_Message">
    ///     A descriptive message of the error.
    /// </param>
    /**/
    explicit _CONCRTIMP nested_scheduler_missing_detach(_In_z_ const char * _Message) noexcept;

    /// <summary>
    ///     Constructs a <c>nested_scheduler_missing_detach</c> object.
    /// </summary>
    /**/
    _CONCRTIMP nested_scheduler_missing_detach() noexcept;
};

/// <summary>
///     This class describes an exception thrown when an operation has timed out.
/// </summary>
/**/
class operation_timed_out : public ::std::exception
{
public:
    /// <summary>
    ///     Constructs an <c>operation_timed_out</c> object.
    /// </summary>
    /// <param name="_Message">
    ///     A descriptive message of the error.
    /// </param>
    /**/
    explicit _CONCRTIMP operation_timed_out(_In_z_ const char * _Message) noexcept;

    /// <summary>
    ///     Constructs an <c>operation_timed_out</c> object.
    /// </summary>
    /**/
    _CONCRTIMP operation_timed_out() noexcept;
};

/// <summary>
///     This class describes an exception thrown when a <c>task_handle</c> object is scheduled multiple times
///     using the <c>run</c> method of a <c>task_group</c> or <c>structured_task_group</c> object without an intervening
///     call to either the <c>wait</c> or <c>run_and_wait</c> methods.
/// </summary>
/// <seealso cref="task_handle Class"/>
/// <seealso cref="task_group Class"/>
/// <seealso cref="task_group::run Method"/>
/// <seealso cref="task_group::wait Method"/>
/// <seealso cref="task_group::run_and_wait Method"/>
/// <seealso cref="structured_task_group Class"/>
/// <seealso cref="structured_task_group::run Method"/>
/// <seealso cref="structured_task_group::wait Method"/>
/// <seealso cref="structured_task_group::run_and_wait Method"/>
/**/
class invalid_multiple_scheduling : public ::std::exception
{
public:
    /// <summary>
    ///     Constructs an <c>invalid_multiple_scheduling</c> object.
    /// </summary>
    /// <param name="_Message">
    ///     A descriptive message of the error.
    /// </param>
    /**/
    explicit _CONCRTIMP invalid_multiple_scheduling(_In_z_ const char * _Message) noexcept;

    /// <summary>
    ///     Constructs an <c>invalid_multiple_scheduling</c> object.
    /// </summary>
    /**/
    _CONCRTIMP invalid_multiple_scheduling() noexcept;
};

/// <summary>
///     This class describes an exception thrown when the <c>Context::Oversubscribe</c> method is called with
///     the <paramref name="_BeginOversubscription"/> parameter set to <c>false</c> without a prior call to the
///     <c>Context::Oversubscribe</c> method with the <paramref name="_BeginOversubscription"/> parameter set to <c>true</c>.
/// </summary>
/// <seealso cref="Context::Oversubscribe Method"/>
/**/
class invalid_oversubscribe_operation : public ::std::exception
{
public:
    /// <summary>
    ///     Constructs an <c>invalid_oversubscribe_operation</c> object.
    /// </summary>
    /// <param name="_Message">
    ///     A descriptive message of the error.
    /// </param>
    /**/
    explicit _CONCRTIMP invalid_oversubscribe_operation(_In_z_ const char * _Message) noexcept;

    /// <summary>
    ///     Constructs an <c>invalid_oversubscribe_operation</c> object.
    /// </summary>
    /**/
    _CONCRTIMP invalid_oversubscribe_operation() noexcept;
};

/// <summary>
///     This class describes an exception thrown when a lock is acquired improperly.
/// </summary>
/// <remarks>
///     Typically, this exception is thrown when an attempt is made to acquire a non-reentrant lock
///     recursively on the same context.
/// </remarks>
/// <seealso cref="critical_section Class"/>
/// <seealso cref="reader_writer_lock Class"/>
/**/
class improper_lock : public ::std::exception
{
public:

    /// <summary>
    ///     Constructs an <c>improper_lock exception</c>.
    /// </summary>
    /// <param name="_Message">
    ///     A descriptive message of the error.
    /// </param>
    /**/
    explicit _CONCRTIMP improper_lock(_In_z_ const char * _Message) noexcept;

    /// <summary>
    ///     Constructs an <c>improper_lock</c> exception.
    /// </summary>
    /**/
    _CONCRTIMP improper_lock() noexcept;
};

/// <summary>
///     An abstraction of a physical location on hardware.
/// </summary>
/**/
class location
{
public:

    /// <summary>
    ///     Constructs a <c>location</c> object.
    /// </summary>
    /// <remarks>
    ///     A default constructed location represents the system as a whole.
    /// </remarks>
    /**/
    location() :
        _M_type(_System),
        _M_reserved(0),
        _M_bindingId(0),
        _M_ptr(nullptr),
        _M_pBinding(nullptr)
    {
    }

    /// <summary>
    ///     Constructs a <c>location</c> object.
    /// </summary>
    /**/
    location(const location& _Src)
    {
        _Assign(_Src);
    }

#ifdef _CRT_USE_WINAPI_FAMILY_DESKTOP_APP

    /// <summary>
    ///     Returns a <c>location</c> object which represents a given NUMA node.
    /// </summary>
    /// <param name="_NumaNodeNumber">
    ///     The NUMA node number to construct a location for.
    /// </param>
    /// <returns>
    ///     A location representing the NUMA node specified by the <paramref name="_NumaNodeNumber"/> parameter.
    /// </returns>
    /**/
    _CONCRTIMP static location __cdecl from_numa_node(unsigned short _NumaNodeNumber);

#endif  /* _CRT_USE_WINAPI_FAMILY_DESKTOP_APP */

    /// <summary>
    ///     Returns a <c>location</c> object representing the most specific place the calling thread is executing.
    /// </summary>
    /// <returns>
    ///     A location representing the most specific place the calling thread is executing.
    /// </returns>
    /**/
    _CONCRTIMP static location __cdecl current();

    /// <summary>
    ///     Assigns the contents of a different <c>location</c> object to this one.
    /// </summary>
    /// <param name="_Rhs">
    ///     The source <c>location</c> object.
    /// </param>
    /**/
    location& operator=(const location& _Rhs)
    {
        _Assign(_Rhs);
        return *this;
    }

    /// <summary>
    ///     Destroys a <c>location</c> object.
    /// </summary>
    /**/
    ~location()
    {
    }

    /// <summary>
    ///     Determines whether two <c>location</c> objects represent the same location.
    /// </summary>
    /// <returns>
    ///     <c>true</c> if the two locations are identical, and <c>false</c> otherwise.
    /// </returns>
    /**/
    bool operator==(const location& _Rhs) const
    {
        return (_M_type == _Rhs._M_type && _M_ptr == _Rhs._M_ptr);
    }

    /// <summary>
    ///     Determines whether two <c>location</c> objects represent different location.
    /// </summary>
    /// <returns>
    ///     <c>true</c> if the two locations are different, <c>false</c> otherwise.
    /// </returns>
    /**/
    bool operator!=(const location& _Rhs) const
    {
        return !operator==(_Rhs);
    }

    //**************************************************
    //
    // Runtime internal public pieces of location. No code outside the core of ConcRT can depend on anything
    // below. It is internal implementation detail:
    //

    /// <summary>
    ///     Returns a location representing the scheduling node that the calling thread is executing.
    /// </summary>
    /**/
    _CONCRTIMP static location __cdecl _Current_node();

    /// <summary>
    ///     Describes the type of the given location.
    /// </summary>
    /**/
    enum _Type
    {
        /// <summary>
        ///     Indicates that the location represents the "system location". This has no specific affinity.
        /// </summary>
        _System, // _M_id is meaningless

        /// <summary>
        ///     Indicates that the location represents a particular NUMA node.
        /// </summary>
        _NumaNode, // _M_id is the Windows NUMA node number

        /// <summary>
        ///     Indicates that the location represents a particular scheduling node.
        /// </summary>
        _SchedulingNode, // _M_id is the unique identifier for the scheduling node

        /// <summary>
        ///     Indicates that the location represents a particular execution resource.
        /// </summary>
        _ExecutionResource, // _M_id is the unique identifier for the execution resource
    };

    /// <summary>
    ///     Constructs a specific location.
    /// </summary>
    /**/
    location(_Type _LocationType, unsigned int _Id, unsigned int _BindingId = 0, _Inout_opt_ void *_PBinding = nullptr);

    /// <summary>
    ///     Determines whether two locations have an intersection. This is a fast intersection which avoids certain checks by knowing that
    ///     the *this* pointer is a virtual processor location for a validly bound virtual processor.
    /// </summary>
    /// <param name="_Rhs">
    ///     The location to intersect with this.
    /// </param>
    /// <returns>
    ///     An indication as to whether the two locations intersect.
    /// </returns>
    /**/
    bool _FastVPIntersects(const location& _Rhs) const;

    /// <summary>
    ///     Determines whether two locations have an intersection. This is a fast intersection which avoids certain checks by knowing that
    ///     the *this* pointer is a node for a validly bound node.
    /// </summary>
    /// <param name="_Rhs">
    ///     The location to intersect with this.
    /// </param>
    /// <returns>
    ///     An indication as to whether the two locations intersect.
    /// </returns>
    /**/
    bool _FastNodeIntersects(const location& _Rhs) const;

    /// <summary>
    ///     Assigns _Rhs to this location.
    /// </summary>
    /**/
    void _Assign(const location& _Rhs)
    {
        _M_type = _Rhs._M_type;
        _M_reserved = _Rhs._M_reserved;

        _M_ptr = _Rhs._M_ptr;

        _M_bindingId = _Rhs._M_bindingId;
        _M_pBinding = _Rhs._M_pBinding;
    }

    /// <summary>
    ///     Internal routine that tells whether a location represents the "system location". This indicates no specific placement.
    /// </summary>
    /**/
    bool _Is_system() const
    {
        return (_Type)_M_type == _System;
    }

    /// <summary>
    ///     Returns the internal binding as a specified object.
    /// </summary>
    /**/
    template<typename _Ty>
    _Ty* _As() const
    {
        return reinterpret_cast<_Ty *>(_M_pBinding);
    }

    /// <summary>
    ///     Returns the ID which this location object represents.
    /// </summary>
    /**/
    unsigned int _GetId() const
    {
        return _M_id;
    }

    /// <summary>
    ///     Returns the type which this location object represents.
    /// </summary>
    /**/
    _Type _GetType() const
    {
        return (_Type)_M_type;
    }

    /// <summary>
    ///     Gets the binding ID for this location.
    /// </summary>
    /**/
    unsigned int _GetBindingId() const
    {
        return _M_bindingId;
    }

private:

    // Indicates the type of location (as _Type)
    unsigned int _M_type : 28;

    // Flags on the location. Reserved for future use.
    unsigned int _M_reserved : 4;

    // If the location has a tight binding, this is the unique identifier of the scheduler to which the binding has specific meaning.
    unsigned int _M_bindingId;

    // Defines the agnostic (abstract hardware) binding of the location.
    union
    {
        // The identifier for the binding (NUMA node number, scheduler node ID, execution resource ID)
        unsigned int _M_id;

        // Pointer binding.
        void *_M_ptr;
    };

    // The specific binding to a scheduler. (For example, a specific virtual processor for something like location::current() )
    // This will be NULL if there is no tight binding.
    void *_M_pBinding;
};

#ifdef _CRT_USE_WINAPI_FAMILY_DESKTOP_APP

/// <summary>
///     Represents an abstraction for a schedule group. Schedule groups organize a set of related work that benefits from being
///     scheduled close together either temporally, by executing another task in the same group before moving to another group, or
///     spatially, by executing multiple items within the same group on the same NUMA node or physical socket.
/// </summary>
/// <seealso cref="CurrentScheduler Class"/>
/// <seealso cref="Scheduler Class"/>
/// <seealso cref="Task Scheduler (Concurrency Runtime)"/>
/**/
class ScheduleGroup
{
public:

    /// <summary>
    ///     Schedules a light-weight task within the schedule group.
    /// </summary>
    /// <param name="_Proc">
    ///     A pointer to the function to execute to perform the body of the light-weight task.
    /// </param>
    /// <param name="_Data">
    ///     A void pointer to the data that will be passed as a parameter to the body of the task.
    /// </param>
    /// <remarks>
    ///     Calling the <c>ScheduleTask</c> method implicitly places a reference count on the schedule group which is removed by the runtime
    ///     at an appropriate time after the task executes.
    /// </remarks>
    /// <seealso cref="ScheduleGroup::Reference Method"/>
    /**/
    virtual void ScheduleTask(TaskProc _Proc, _Inout_opt_ void * _Data) =0;

    /// <summary>
    ///     Returns an identifier for the schedule group that is unique within the scheduler to which the group belongs.
    /// </summary>
    /// <returns>
    ///     An identifier for the schedule group that is unique within the scheduler to which the group belongs.
    /// </returns>
    /**/
    virtual unsigned int Id() const =0;

    /// <summary>
    ///     Increments the schedule group reference count.
    /// </summary>
    /// <returns>
    ///     The newly incremented reference count.
    /// </returns>
    /// <remarks>
    ///     This is typically used to manage the lifetime of the schedule group for composition. When the reference count of a schedule
    ///     group falls to zero, the schedule group is deleted by the runtime. A schedule group created using either the
    ///     <see cref="CurrentScheduler::CreateScheduleGroup Method">CurrentScheduler::CreateScheduleGroup</see> method, or the
    ///     <see cref="Scheduler::CreateScheduleGroup Method">Scheduler::CreateScheduleGroup</see> method starts out with a reference
    ///     count of one.
    /// </remarks>
    /// <seealso cref="ScheduleGroup::Release Method"/>
    /// <seealso cref="CurrentScheduler::CreateScheduleGroup Method"/>
    /// <seealso cref="Scheduler::CreateScheduleGroup Method"/>
    /**/
    virtual unsigned int Reference() =0;

    /// <summary>
    ///     Decrements the scheduler group reference count.
    /// </summary>
    /// <returns>
    ///     The newly decremented reference count.
    /// </returns>
    /// <remarks>
    ///     This is typically used to manage the lifetime of the schedule group for composition. When the reference count of a schedule
    ///     group falls to zero, the schedule group is deleted by the runtime. After you have called the <c>Release</c> method the specific number
    ///     of times to remove the creation reference count and any additional references placed using the <c>Reference</c> method, you cannot
    ///     utilize the schedule group further. Doing so will result in undefined behavior.
    ///     <para>A schedule group is associated with a particular scheduler instance. You must ensure that all references to the
    ///     schedule group are released before all references to the scheduler are released, because the latter could result in the scheduler
    ///     being destroyed. Doing otherwise results in undefined behavior.</para>
    /// </remarks>
    /// <seealso cref="ScheduleGroup::Reference Method"/>
    /// <seealso cref="CurrentScheduler::CreateScheduleGroup Method"/>
    /// <seealso cref="Scheduler::CreateScheduleGroup Method"/>
    /**/
    virtual unsigned int Release() =0;

protected:

    //
    // Privatize operator delete. Clients should utilize Release to relinquish a schedule group.
    //
    template<class _Ty> friend void ::Concurrency::details::_InternalDeleteHelper(_Ty * _PObject);

    virtual ~ScheduleGroup() {};
};

/// <summary>
///     Special value for the policy keys <c>MinConcurrency</c> and <c>MaxConcurrency</c>. Defaults to the number of hardware
///     threads on the machine in the absence of other constraints.
/// </summary>
/// <seealso cref="PolicyElementKey Enumeration"/>
/**/
const unsigned int MaxExecutionResources = 0xFFFFFFFF;

/// <summary>
///     Special value for the policy key <c>ContextPriority</c> indicating that the thread priority of all contexts in the scheduler
///     should be the same as that of the thread which created the scheduler.
/// </summary>
/// <seealso cref="PolicyElementKey Enumeration"/>
/**/
const unsigned int INHERIT_THREAD_PRIORITY = 0x0000F000;

/// <summary>
///     Policy keys describing aspects of scheduler behavior. Each policy element is described by a key-value pair. For more information
///     about scheduler policies and their impact on schedulers, see <see cref="Task Scheduler (Concurrency Runtime)"/>.
/// </summary>
/// <seealso cref="SchedulerPolicy Class"/>
/// <seealso cref="CurrentScheduler Class"/>
/// <seealso cref="Scheduler Class"/>
/// <seealso cref="Task Scheduler (Concurrency Runtime)"/>
/**/
enum PolicyElementKey
{
    /// <summary>
    ///     The type of threads that the scheduler will utilize for underlying execution contexts. For more information, see
    ///     <see cref="SchedulerType Enumeration"/>.
    ///     <para>Valid values  : A member of the <c>SchedulerType</c> enumeration, for example, <c>ThreadScheduler</c></para>
    ///     <para>Default value : <c>ThreadScheduler</c>. This translates to Win32 threads on all operating systems.</para>
    /// </summary>
    /**/
    SchedulerKind,

    /// <summary>
    ///     The maximum concurrency level desired by the scheduler. The resource manager will try to initially allocate this many virtual processors.
    ///     The special value <see cref="MaxExecutionResources Constant">MaxExecutionResources</see> indicates that the desired concurrency level
    ///     is same as the number of hardware threads on the machine. If the value specified for <c>MinConcurrency</c> is greater than the number
    ///     of hardware threads on the machine and <c>MaxConcurrency</c> is specified as <c>MaxExecutionResources</c>, the value for <c>MaxConcurrency</c>
    ///     is raised to match what is set for <c>MinConcurrency</c>.
    ///     <para>Valid values  : Positive integers and the special value <c>MaxExecutionResources</c></para>
    ///     <para>Default value : <c>MaxExecutionResources</c></para>
    /// </summary>
    /**/
    MaxConcurrency,

    /// <summary>
    ///     The minimum concurrency level that must be provided to the scheduler by the resource manager. The number of virtual processors assigned
    ///     to a scheduler will never go below the minimum. The special value <see cref="MaxExecutionResources Constant">MaxExecutionResources</see>
    ///     indicates that the minimum concurrency level is same as the number of hardware threads on the machine. If the value specified for
    ///     <c>MaxConcurrency</c> is less than the number of hardware threads on the machine and <c>MinConcurrency</c> is specified as
    ///     <c>MaxExecutionResources</c>, the value for <c>MinConcurrency</c> is lowered to match what is set for <c>MaxConcurrency</c>.
    ///     <para>Valid values  : Non-negative integers and the special value <c>MaxExecutionResources</c>. Note that for scheduler policies
    ///     used for the construction of Concurrency Runtime schedulers, the value <c>0</c> is invalid.</para>
    ///     <para>Default value : <c>1</c></para>
    /// </summary>
    /**/
    MinConcurrency,

    /// <summary>
    ///     Tentative number of virtual processors per hardware thread. The target oversubscription factor can be increased by the Resource Manager,
    ///     if necessary, to satisfy <c>MaxConcurrency</c> with the hardware threads on the machine.
    ///     <para>Valid values  : Positive integers</para>
    ///     <para>Default value : <c>1</c></para>
    /// </summary>
    /**/
    TargetOversubscriptionFactor,

    /// <summary>
    ///     When the <c>SchedulingProtocol</c> policy key is set to the value <c>EnhanceScheduleGroupLocality</c>, this specifies the maximum number
    ///     of runnable contexts allowed to be cached in per virtual processor local queues. Such contexts will typically run in last-in-first-out
    ///     (LIFO) order on the virtual processor that caused them to become runnable. Note that this policy key has no meaning when the
    ///     <c>SchedulingProtocol</c> key is set to the value <c>EnhanceForwardProgress</c>.
    ///     <para>Valid values  : Non-negative integers</para>
    ///     <para>Default value : <c>8</c></para>
    /// </summary>
    /**/
    LocalContextCacheSize,

    /// <summary>
    ///     The reserved stack size of each context in the scheduler in kilobytes.
    ///     <para>Valid values  : Positive integers</para>
    ///     <para>Default value : <c>0</c>, indicating that the process' default value for stack size be used.</para>
    /// </summary>
    /**/
    ContextStackSize,

    /// <summary>
    ///     The operating system thread priority of each context in the scheduler. If this key is set to the value <see cref="INHERIT_THREAD_PRIORITY Constant">
    ///     INHERIT_THREAD_PRIORITY</see> the contexts in the scheduler will inherit the priority of the thread that created the scheduler.
    ///     <para>Valid values  : Any of the valid values for the Windows <c>SetThreadPriority</c> function and the special value
    ///     <c>INHERIT_THREAD_PRIORITY</c></para>
    ///     <para>Default value : <c>THREAD_PRIORITY_NORMAL</c></para>
    /// </summary>
    /**/
    ContextPriority,

    /// <summary>
    ///     Describes which scheduling algorithm will be used by the scheduler. For more information, see <see cref="SchedulingProtocolType Enumeration"/>.
    ///     <para>Valid values  : A member of the <c>SchedulingProtocolType</c> enumeration, either <c>EnhanceScheduleGroupLocality</c>
    ///     or <c>EnhanceForwardProgress</c></para>
    ///     <para>Default value : <c>EnhanceScheduleGroupLocality</c></para>
    /// </summary>
    /**/
    SchedulingProtocol,

    /// <summary>
    ///     Determines whether the resources for the scheduler will be rebalanced according to statistical information gathered from the
    ///     scheduler or only based on the subscription level of underlying hardware threads. For more information, see
    ///     <see cref="DynamicProgressFeedbackType Enumeration"/>.
    ///     <para>Valid values  : A member of the <c>DynamicProgressFeedbackType</c> enumeration, either <c>ProgressFeedbackEnabled</c> or
    ///     <c>ProgressFeedbackDisabled</c></para>
    ///     <para>Default value : <c>ProgressFeedbackEnabled</c></para>
    /// </summary>
    /**/
    DynamicProgressFeedback,

    /// <summary>
    ///     Determines whether and how scheduler threads will initialize the Windows Runtime. This policy key only carries meaning for applications
    ///     executing on operating systems with version Windows 8 or higher. For more information, see <see cref="WinRTInitializationType Enumeration"/>.
    ///     <para>Valid values  : A member of the <c>WinRTInitializationType</c> enumeration, either <c>InitializeWinRTAsMTA</c> or
    ///     <c>DoNotInitializeWinRT</c></para>
    ///     <para>Default value : <c>InitializeWinRTAsMTA</c>
    /// </summary>
    /**/
    WinRTInitialization,

    /// <summary>
    ///     The maximum policy element key. Not a valid element key.
    /// </summary>
    /**/
    MaxPolicyElementKey
};

/// <summary>
///     Used by the <c>SchedulerKind</c> policy to describe the type of threads that the scheduler should utilize for underlying execution contexts.
///     For more information on available scheduler policies, see <see cref="PolicyElementKey Enumeration"/>.
/// </summary>
/// <seealso cref="SchedulerPolicy Class"/>
/// <seealso cref="PolicyElementKey Enumeration"/>
/**/
enum SchedulerType
{
    /// <summary>
    ///     Indicates an explicit request of regular Win32 threads.
    /// </summary>
    /**/
    ThreadScheduler,
};

/// <summary>
///     Used by the <c>SchedulingProtocol</c> policy to describe which scheduling algorithm will be utilized for the scheduler. For more
///     information on available scheduler policies, see <see cref="PolicyElementKey Enumeration"/>.
/// </summary>
/// <seealso cref="SchedulerPolicy Class"/>
/// <seealso cref="PolicyElementKey Enumeration"/>
/**/
enum SchedulingProtocolType
{
    /// <summary>
    ///     The scheduler prefers to continue to work on tasks within the current schedule group before moving to another schedule group.
    ///     Unblocked contexts are cached per virtual-processor and are typically scheduled in a last-in-first-out (LIFO) fashion by the
    ///     virtual processor which unblocked them.
    /// </summary>
    /**/
    EnhanceScheduleGroupLocality,

    /// <summary>
    ///     The scheduler prefers to round-robin through schedule groups after executing each task. Unblocked contexts are typically
    ///     scheduled in a first-in-first-out (FIFO) fashion. Virtual processors do not cache unblocked contexts.
    /// </summary>
    /**/
    EnhanceForwardProgress
};

/// <summary>
///     Used by the <c>DynamicProgressFeedback</c> policy to describe whether resources for the scheduler will be rebalanced according to
///     statistical information gathered from the scheduler or only based on virtual processors going in and out of the idle state through
///     calls to the <c>Activate</c> and <c>Deactivate</c> methods on the <c>IVirtualProcessorRoot</c> interface. For more information
///     on available scheduler policies, see <see cref="PolicyElementKey Enumeration"/>.
/// </summary>
/// <seealso cref="PolicyElementKey Enumeration"/>
/**/
enum DynamicProgressFeedbackType
{
    /// <summary>
    ///     The scheduler does not gather progress information. Rebalancing is done based solely on the subscription level of the underlying
    ///     hardware thread. For more information on subscription levels, see
    ///     <see cref="IExecutionResource::CurrentSubscriptionLevel Method">IExecutionResource::CurrentSubscriptionLevel</see>.
    ///     <para>This value is reserved for use by the runtime.</para>
    /// </summary>
    /**/
    ProgressFeedbackDisabled,

    /// <summary>
    ///     The scheduler gathers progress information and passes it to the resource manager. The resource manager will utilize this statistical
    ///     information to rebalance resources on behalf of the scheduler in addition to the subscription level of the underlying
    ///     hardware thread. For more information on subscription levels, see
    ///     <see cref="IExecutionResource::CurrentSubscriptionLevel Method">IExecutionResource::CurrentSubscriptionLevel</see>.
    /// </summary>
    /**/
    ProgressFeedbackEnabled
};

/// <summary>
///     Used by the <c>WinRTInitialization</c> policy to describe whether and how the Windows Runtime will be initialized on scheduler threads
///     for an application which runs on operating systems with version Windows 8 or higher. For more information on available scheduler policies,
///     see <see cref="PolicyElementKey Enumeration"/>.
/// </summary>
/// <seealso cref="PolicyElementKey Enumeration"/>
/**/
enum WinRTInitializationType
{
    /// <summary>
    ///     When the application is run on operating systems with version Windows 8 or higher, each thread within the scheduler will initialize the
    ///     Windows Runtime and declare that it is part of the multithreaded apartment.
    /// </summary>
    /**/
    InitializeWinRTAsMTA,

    /// <summary>
    ///     When the application is run on operating systems with version Windows 8 or higher, threads within the scheduler will not initialize the
    ///     Windows Runtime .
    /// </summary>
    /**/
    DoNotInitializeWinRT
};

/// <summary>
///     The <c>SchedulerPolicy</c> class contains a set of key/value pairs, one for each policy element, that control the behavior of a
///     scheduler instance.
/// </summary>
/// <remarks>
///     For more information about the policies which can be controlled using the <c>SchedulerPolicy</c> class, see
///     <see cref="PolicyElementKey Enumeration"/>.
/// </remarks>
/// <seealso cref="PolicyElementKey Enumeration"/>
/// <seealso cref="CurrentScheduler Class"/>
/// <seealso cref="Scheduler Class"/>
/// <seealso cref="Task Scheduler (Concurrency Runtime)"/>
/**/
class SchedulerPolicy
{
public:

    /// <summary>
    ///     Constructs a new scheduler policy and populates it with values for <see cref="PolicyElementKey Enumeration">policy keys</see>
    ///     supported by Concurrency Runtime schedulers and the Resource Manager.
    /// </summary>
    /// <remarks>
    ///     <para>The first constructor creates a new scheduler policy where all policies will be initialized to their default values.</para>
    ///     <para>The second constructor creates a new scheduler policy that uses a named-parameter style of initialization. Values after
    ///     the <paramref name="_PolicyKeyCount"/> parameter are supplied as key/value pairs. Any policy key which is not specified in this
    ///     constructor will have its default value. This constructor could throw the exceptions <see cref="invalid_scheduler_policy_key Class">
    ///     invalid_scheduler_policy_key</see>, <see cref="invalid_scheduler_policy_value Class">invalid_scheduler_policy_value </see> or
    ///     <see cref="invalid_scheduler_policy_thread_specification Class"> invalid_scheduler_policy_thread_specification</see>.</para>
    ///     <para>The third constructor is a copy constructor. Often, the most convenient way to define a new scheduler policy is to copy an
    ///     existing policy and modify it using the <c>SetPolicyValue</c> or <c>SetConcurrencyLimits</c> methods.</para>
    /// </remarks>
    /// <seealso cref="SchedulerPolicy::SetPolicyValue Method"/>
    /// <seealso cref="SchedulerPolicy::GetPolicyValue Method"/>
    /// <seealso cref="SchedulerPolicy::SetConcurrencyLimits Method"/>
    /// <seealso cref="PolicyElementKey Enumeration"/>
    /**/
    _CONCRTIMP SchedulerPolicy();

    /// <summary>
    ///     Constructs a new scheduler policy and populates it with values for <see cref="PolicyElementKey Enumeration">policy keys</see>
    ///     supported by Concurrency Runtime schedulers and the Resource Manager.
    /// </summary>
    /// <param name="_PolicyKeyCount">
    ///     The number of key/value pairs that follow the <paramref name="_PolicyKeyCount"/> parameter.
    /// </param>
    /// <remarks>
    ///     <para>The first constructor creates a new scheduler policy where all policies will be initialized to their default values.</para>
    ///     <para>The second constructor creates a new scheduler policy that uses a named-parameter style of initialization. Values after </para>
    ///     the <paramref name="_PolicyKeyCount"/> parameter are supplied as key/value pairs. Any policy key which is not specified in this
    ///     constructor will have its default value. This constructor could throw the exceptions <see cref="invalid_scheduler_policy_key Class">
    ///     invalid_scheduler_policy_key</see>, <see cref="invalid_scheduler_policy_value Class">invalid_scheduler_policy_value </see> or
    ///     <see cref="invalid_scheduler_policy_thread_specification Class"> invalid_scheduler_policy_thread_specification</see>.
    ///     <para>The third constructor is a copy constructor. Often, the most convenient way to define a new scheduler policy is to copy an
    ///     existing policy and modify it using the <c>SetPolicyValue</c> or <c>SetConcurrencyLimits</c> methods.</para>
    /// </remarks>
    /// <seealso cref="SchedulerPolicy::SetPolicyValue Method"/>
    /// <seealso cref="SchedulerPolicy::GetPolicyValue Method"/>
    /// <seealso cref="SchedulerPolicy::SetConcurrencyLimits Method"/>
    /// <seealso cref="PolicyElementKey Enumeration"/>
    /**/
    _CONCRTIMP SchedulerPolicy(size_t _PolicyKeyCount, ...);

    /// <summary>
    ///     Constructs a new scheduler policy and populates it with values for <see cref="PolicyElementKey Enumeration">policy keys</see>
    ///     supported by Concurrency Runtime schedulers and the Resource Manager.
    /// </summary>
    /// <param name="_SrcPolicy">
    ///     The source policy to copy.
    /// </param>
    /// <remarks>
    ///     <para>The first constructor creates a new scheduler policy where all policies will be initialized to their default values.</para>
    ///     <para>The second constructor creates a new scheduler policy that uses a named-parameter style of initialization. Values after </para>
    ///     the <paramref name="_PolicyKeyCount"/> parameter are supplied as key/value pairs. Any policy key which is not specified in this
    ///     constructor will have its default value. This constructor could throw the exceptions <see cref="invalid_scheduler_policy_key Class">
    ///     invalid_scheduler_policy_key</see>, <see cref="invalid_scheduler_policy_value Class">invalid_scheduler_policy_value </see> or
    ///     <see cref="invalid_scheduler_policy_thread_specification Class"> invalid_scheduler_policy_thread_specification</see>.
    ///     <para>The third constructor is a copy constructor. Often, the most convenient way to define a new scheduler policy is to copy an
    ///     existing policy and modify it using the <c>SetPolicyValue</c> or <c>SetConcurrencyLimits</c> methods.</para>
    /// </remarks>
    /// <seealso cref="SchedulerPolicy::SetPolicyValue Method"/>
    /// <seealso cref="SchedulerPolicy::GetPolicyValue Method"/>
    /// <seealso cref="SchedulerPolicy::SetConcurrencyLimits Method"/>
    /// <seealso cref="PolicyElementKey Enumeration"/>
    /**/
    _CONCRTIMP SchedulerPolicy(const SchedulerPolicy& _SrcPolicy);

    /// <summary>
    ///     Assigns the scheduler policy from another scheduler policy.
    /// </summary>
    /// <param name="_RhsPolicy">
    ///     The policy to assign to this policy.
    /// </param>
    /// <returns>
    ///     A reference to the scheduler policy.
    /// </returns>
    /// <remarks>
    ///     Often, the most convenient way to define a new scheduler policy is to copy an existing policy and modify it using the
    ///     <c>SetPolicyValue</c> or <c>SetConcurrencyLimits</c> methods.
    /// </remarks>
    /// <seealso cref="SchedulerPolicy::SetPolicyValue Method"/>
    /// <seealso cref="SchedulerPolicy::SetConcurrencyLimits Method"/>
    /// <seealso cref="PolicyElementKey Enumeration"/>
    /**/
    _CONCRTIMP SchedulerPolicy& operator=(const SchedulerPolicy& _RhsPolicy);

    /// <summary>
    ///     Destroys a scheduler policy.
    /// </summary>
    /**/
    _CONCRTIMP ~SchedulerPolicy();

    /// <summary>
    ///     Retrieves the value of the policy key supplied as the <paramref name="_Key"/> parameter.
    /// </summary>
    /// <param name="_Key">
    ///     The policy key to retrieve a value for.
    /// </param>
    /// <returns>
    ///     If the key specified by the <paramref name="_Key"/> parameter is supported, the policy value for the key cast to an <c>unsigned int</c>.
    /// </returns>
    /// <remarks>
    ///     The method will throw <see cref="invalid_scheduler_policy_key Class">invalid_scheduler_policy_key</see> for an invalid policy key.
    /// </remarks>
    /// <seealso cref="SchedulerPolicy::SetPolicyValue Method"/>
    /// <seealso cref="SchedulerPolicy::SetConcurrencyLimits Method"/>
    /// <seealso cref="PolicyElementKey Enumeration"/>
    /**/
    _CONCRTIMP unsigned int GetPolicyValue(PolicyElementKey _Key) const;

    /// <summary>
    ///     Sets the value of the policy key supplied as the <paramref name="_Key"/> parameter and returns the old value.
    /// </summary>
    /// <param name="_Key">
    ///     The policy key to set a value for.
    /// </param>
    /// <param name="_Value">
    ///     The value to set the policy key to.
    /// </param>
    /// <returns>
    ///     If the key specified by the <paramref name="_Key"/> parameter is supported, the old policy value for the key cast to an <c>unsigned int</c>.
    /// </returns>
    /// <remarks>
    ///     The method will throw <see cref="invalid_scheduler_policy_key Class">invalid_scheduler_policy_key </see> for an invalid policy key
    ///     or any policy key whose value cannot be set by the <c>SetPolicyValue</c> method.
    ///     <para>The method will throw <see cref="invalid_scheduler_policy_value Class">invalid_scheduler_policy_value</see> for a value that
    ///     is not supported for the key specified by the <paramref name="_Key"/> parameter.</para>
    ///     <para>Note that this method is not allowed to set the <c>MinConcurrency</c> or <c>MaxConcurrency</c> policies. To set these values, use
    ///     the <see cref="SchedulerPolicy::SetConcurrencyLimits Method">SetConcurrencyLimits</see> method.</para>
    /// </remarks>
    /// <seealso cref="SchedulerPolicy::GetPolicyValue Method"/>
    /// <seealso cref="SchedulerPolicy::SetConcurrencyLimits Method"/>
    /// <seealso cref="PolicyElementKey Enumeration"/>
    /**/
    _CONCRTIMP unsigned int SetPolicyValue(PolicyElementKey _Key, unsigned int _Value);

    /// <summary>
    ///     Simultaneously sets the <c>MinConcurrency</c> and <c>MaxConcurrency</c> policies on the <c>SchedulerPolicy</c> object.
    /// </summary>
    /// <param name="_MinConcurrency">
    ///     The value for the <c>MinConcurrency</c> policy key.
    /// </param>
    /// <param name="_MaxConcurrency">
    ///     The value for the <c>MaxConcurrency</c> policy key.
    /// </param>
    /// <remarks>
    ///     The method will throw <see cref="invalid_scheduler_policy_thread_specification Class">invalid_scheduler_policy_thread_specification
    ///     </see> if the value specified for the <c>MinConcurrency</c> policy is greater than that specified for the <c>MaxConcurrency</c> policy.
    ///     <para>The method can also throw <see cref="invalid_scheduler_policy_value Class">invalid_scheduler_policy_value </see> for other
    ///     invalid values.</para>
    /// </remarks>
    /// <seealso cref="SchedulerPolicy::GetPolicyValue Method"/>
    /// <seealso cref="SchedulerPolicy::SetPolicyValue Method"/>
    /// <seealso cref="PolicyElementKey Enumeration"/>
    /**/
    _CONCRTIMP void SetConcurrencyLimits(unsigned int _MinConcurrency, unsigned int _MaxConcurrency = MaxExecutionResources);

    /// <summary>
    ///     Checks if this policy is a valid policy for a Concurrency Runtime scheduler. If it is not, an appropriate exception will be thrown.
    /// </summary>
    /// <remarks>
    ///     The method will throw <see cref="invalid_scheduler_policy_value Class">invalid_scheduler_policy_value </see> if a policy value supplied
    ///     in the <c>SchedulerPolicy</c> object cannot be used to create a Concurrency Runtime scheduler. Note that such a policy is not necessarily
    ///     invalid. The Concurrency Runtime Resource Manager also utilizes the <c>SchedulerPolicy</c> class to describe requirements.
    /// </remarks>
    /**/
    void _ValidateConcRTPolicy() const;

private:

    struct _PolicyBag
    {
        union
        {
            unsigned int _M_pPolicyBag[MaxPolicyElementKey];
            struct
            {
                SchedulerType _M_schedulerKind;
                unsigned int _M_maxConcurrency;
                unsigned int _M_minConcurrency;
                unsigned int _M_targetOversubscriptionFactor;
                unsigned int _M_localContextCacheSize;
                unsigned int _M_contextStackSize;
                unsigned int _M_contextPriority;
                SchedulingProtocolType _M_schedulingProtocol;
                DynamicProgressFeedbackType _M_dynamicProgressFeedback;
                WinRTInitializationType _M_WinRTInitialization;
            } _M_specificValues;
        } _M_values;
    } *_M_pPolicyBag;

    /// <summary>
    ///     Initializes the scheduler policy.
    /// </summary>
    /**/
    void _Initialize(size_t _PolicyKeyCount, va_list * _PArgs);

    /// <summary>
    ///     Make this policy a copy of the source policy.
    /// </summary>
    /**/
    void _Assign(const SchedulerPolicy& _SrcPolicy);

    /// <summary>
    ///     Returns true if the key supplied is a supported key.
    /// </summary>
    /**/
    static bool __cdecl _ValidPolicyKey(PolicyElementKey _Key);

    /// <summary>
    ///     Returns true if a policy value is in a valid range.
    /// </summary>
    /**/
    static bool __cdecl _ValidPolicyValue(PolicyElementKey _Key, unsigned int _Value);

    /// <summary>
    ///     Returns true if concurrency limit combinations are valid.
    /// </summary>
    /**/
    static bool __cdecl _AreConcurrencyLimitsValid(unsigned int _MinConcurrency, unsigned int _MaxConcurrency);
    bool _AreConcurrencyLimitsValid() const;

    /// <summary>
    ///     Test the concurrency combinations of a policy.
    /// </summary>
    /**/
    bool _ArePolicyCombinationsValid() const;

    /// <summary>
    ///     Resolves one or more of the policy keys that are set to defaults, based on the characteristics of the underlying system.
    /// </summary>
    /**/
    void _ResolvePolicyValues();

    /// <summary>
    ///     Stringify policy keys.
    /// </summary>
    /**/
    static const char * __cdecl _StringFromPolicyKey(unsigned int _Index);
};

/// <summary>
///     Represents an abstraction for the current scheduler associated with the calling context.
/// </summary>
/// <remarks>
///     If there is no scheduler (see <see cref="Scheduler Class">Scheduler</see>) associated with the calling context, many
///     methods within the <c>CurrentScheduler</c> class will result in attachment of the process' default scheduler. This may
///     also imply that the process' default scheduler is created during such a call.
/// </remarks>
/// <seealso cref="Scheduler Class"/>
/// <seealso cref="PolicyElementKey Enumeration"/>
/// <seealso cref="Task Scheduler (Concurrency Runtime)"/>
/**/
class CurrentScheduler
{
private:
    CurrentScheduler() {}

public:
    /// <summary>
    ///     Returns a unique identifier for the current scheduler.
    /// </summary>
    /// <returns>
    ///     If a scheduler is associated with the calling context, a unique identifier for that scheduler; otherwise, the value <c>-1</c>.
    /// </returns>
    /// <remarks>
    ///     This method will not result in scheduler attachment if the calling context is not already associated with a scheduler.
    /// </remarks>
    /**/
    _CONCRTIMP static unsigned int __cdecl Id();

    /// <summary>
    ///     Returns a copy of the policy that the current scheduler was created with.
    /// </summary>
    /// <returns>
    ///     A copy of the policy that that the current scheduler was created with.
    /// </returns>
    /// <remarks>
    ///     This method will result in the process' default scheduler being created and/or attached to the calling context if there is no
    ///     scheduler currently associated with the calling context.
    /// </remarks>
    /// <seealso cref="SchedulerPolicy Class"/>
    /**/
    _CONCRTIMP static SchedulerPolicy __cdecl GetPolicy();

    /// <summary>
    ///     Returns a pointer to the scheduler associated with the calling context, also referred to as the current scheduler.
    /// </summary>
    /// <returns>
    ///     A pointer to the scheduler associated with the calling context (the current scheduler).
    /// </returns>
    /// <remarks>
    ///     This method will result in the process' default scheduler being created and/or attached to the calling context if there is no
    ///     scheduler currently associated with the calling context. No additional reference is placed on the <c>Scheduler</c> object
    ///     returned by this method.
    /// </remarks>
    /**/
    _CONCRTIMP static Scheduler * __cdecl Get();

    /// <summary>
    ///     Returns the current number of virtual processors for the scheduler associated with the calling context.
    /// </summary>
    /// <returns>
    ///     If a scheduler is associated with the calling context, the current number of virtual processors for that scheduler; otherwise,
    ///     the value <c>-1</c>.
    /// </returns>
    /// <remarks>
    ///     This method will not result in scheduler attachment if the calling context is not already associated with a scheduler.
    ///     <para>The return value from this method is an instantaneous sampling of the number of virtual processors for the scheduler associated
    ///     with the calling context. This value can be stale the moment it is returned.</para>
    /// </remarks>
    /**/
    _CONCRTIMP static unsigned int __cdecl GetNumberOfVirtualProcessors();

    /// <summary>
    ///     Creates a new scheduler whose behavior is described by the <paramref name="_Policy"/> parameter and attaches it to the calling context.
    ///     The newly created scheduler will become the current scheduler for the calling context.
    /// </summary>
    /// <param name="_Policy">
    ///     The scheduler policy that describes the behavior of the newly created scheduler.
    /// </param>
    /// <remarks>
    ///     The attachment of the scheduler to the calling context implicitly places a reference count on the scheduler.
    ///     <para>After a scheduler is created with the <c>Create</c> method, you must call the <see cref="CurrentScheduler::Detach Method">
    ///     CurrentScheduler::Detach</see> method at some point in the future in order to allow the scheduler to shut down.</para>
    ///     <para>If this method is called from a context that is already attached to a different scheduler, the existing scheduler is remembered
    ///     as the previous scheduler, and the newly created scheduler becomes the current scheduler. When you call the <c>CurrentScheduler::Detach</c>
    ///     method at a later point, the previous scheduler is restored as the current scheduler.</para>
    ///     <para>This method can throw a variety of exceptions, including <see cref="scheduler_resource_allocation_error Class">
    ///     scheduler_resource_allocation_error</see> and <see cref="invalid_scheduler_policy_value Class">invalid_scheduler_policy_value</see>.</para>
    /// </remarks>
    /// <seealso cref="SchedulerPolicy Class"/>
    /// <seealso cref="CurrentScheduler::Detach Method"/>
    /// <seealso cref="Scheduler::Reference Method"/>
    /// <seealso cref="Scheduler::Release Method"/>
    /// <seealso cref="Task Scheduler (Concurrency Runtime)"/>
    /**/
    _CONCRTIMP static void __cdecl Create(const SchedulerPolicy& _Policy);

    /// <summary>
    ///     Detaches the current scheduler from the calling context and restores the previously attached scheduler as the current
    ///     scheduler, if one exists. After this method returns, the calling context is then managed by the scheduler that was previously
    ///     attached to the context using either the <c>CurrentScheduler::Create</c> or <c>Scheduler::Attach</c> method.
    /// </summary>
    /// <remarks>
    ///     The <c>Detach</c> method implicitly removes a reference count from the scheduler.
    ///     <para>If there is no scheduler attached to the calling context, calling this method will result in a <see cref="scheduler_not_attached Class">
    ///     scheduler_not_attached</see> exception being thrown.</para>
    ///     <para>Calling this method from a context that is internal to and managed by a scheduler, or a context that was attached using
    ///     a method other than the <see cref="Scheduler::Attach Method">Scheduler::Attach</see> or <see cref="CurrentScheduler::Create Method">
    ///     CurrentScheduler::Create</see> methods, will result in an <see cref="improper_scheduler_detach Class">improper_scheduler_detach</see>
    ///     exception being thrown.</para>
    /// </remarks>
    /// <seealso cref="Scheduler::Attach Method"/>
    /// <seealso cref="CurrentScheduler::Create Method"/>
    /**/
    _CONCRTIMP static void __cdecl Detach();

    /// <summary>
    ///     Causes the Windows event handle passed in the <paramref name="_ShutdownEvent"/> parameter to be signaled when the scheduler associated with
    ///     the current context shuts down and destroys itself. At the time the event is signaled, all work that had been scheduled to the
    ///     scheduler is complete. Multiple shutdown events can be registered through this method.
    /// </summary>
    /// <param name="_ShutdownEvent">
    ///     A handle to a Windows event object which will be signaled by the runtime when the scheduler associated with the current context
    ///     shuts down and destroys itself.
    /// </param>
    /// <remarks>
    ///     If there is no scheduler attached to the calling context, calling this method will result in a <see cref="scheduler_not_attached Class">
    ///     scheduler_not_attached </see> exception being thrown.
    /// </remarks>
    /**/
    _CONCRTIMP static void __cdecl RegisterShutdownEvent(HANDLE _ShutdownEvent);

    /// <summary>
    ///     Creates a new schedule group within the scheduler associated with the calling context. The version that takes the parameter
    ///     <paramref name="_Placement"/> causes tasks within the newly created schedule group to be biased towards executing at the location
    ///     specified by that parameter.
    /// </summary>
    /// <returns>
    ///     A pointer to the newly created schedule group. This <c>ScheduleGroup</c> object has an initial reference count placed on it.
    /// </returns>
    /// <remarks>
    ///     This method will result in the process' default scheduler being created and/or attached to the calling context if there is no
    ///     scheduler currently associated with the calling context.
    ///     <para>You must invoke the <see cref="ScheduleGroup::Release Method">Release</see> method on a schedule group when you are
    ///     done scheduling work to it. The scheduler will destroy the schedule group when all work queued to it has completed.</para>
    ///     <para>Note that if you explicitly created this scheduler, you must release all references to schedule groups within it, before
    ///     you release your reference on the scheduler, by detaching the current context from it.</para>
    /// </remarks>
    /// <seealso cref="ScheduleGroup Class"/>
    /// <seealso cref="ScheduleGroup::Release Method"/>
    /// <seealso cref="Task Scheduler (Concurrency Runtime)"/>
    /// <seealso cref="location Class"/>
    /**/
    _CONCRTIMP static ScheduleGroup * __cdecl CreateScheduleGroup();

    /// <summary>
    ///     Creates a new schedule group within the scheduler associated with the calling context. The version that takes the parameter
    ///     <paramref name="_Placement"/> causes tasks within the newly created schedule group to be biased towards executing at the location
    ///     specified by that parameter.
    /// </summary>
    /// <param name="_Placement">
    ///     A reference to a location where the tasks within the schedule group will be biased towards executing at.
    /// </param>
    /// <returns>
    ///     A pointer to the newly created schedule group. This <c>ScheduleGroup</c> object has an initial reference count placed on it.
    /// </returns>
    /// <remarks>
    ///     This method will result in the process' default scheduler being created and/or attached to the calling context if there is no
    ///     scheduler currently associated with the calling context.
    ///     <para>You must invoke the <see cref="ScheduleGroup::Release Method">Release</see> method on a schedule group when you are
    ///     done scheduling work to it. The scheduler will destroy the schedule group when all work queued to it has completed.</para>
    ///     <para>Note that if you explicitly created this scheduler, you must release all references to schedule groups within it, before
    ///     you release your reference on the scheduler, by detaching the current context from it.</para>
    /// </remarks>
    /// <seealso cref="ScheduleGroup Class"/>
    /// <seealso cref="ScheduleGroup::Release Method"/>
    /// <seealso cref="Task Scheduler (Concurrency Runtime)"/>
    /// <seealso cref="location Class"/>
    /**/
    _CONCRTIMP static ScheduleGroup * __cdecl CreateScheduleGroup(location& _Placement);

    /// <summary>
    ///     Schedules a light-weight task within the scheduler associated with the calling context. The light-weight task will be placed
    ///     in a schedule group determined by the runtime. The version that takes the parameter <paramref name="_Placement"/> causes the task
    ///     to be biased towards executing at the specified location.
    /// </summary>
    /// <param name="_Proc">
    ///     A pointer to the function to execute to perform the body of the light-weight task.
    /// </param>
    /// <param name="_Data">
    ///     A void pointer to the data that will be passed as a parameter to the body of the task.
    /// </param>
    /// <remarks>
    ///     This method will result in the process' default scheduler being created and/or attached to the calling context if there is no
    ///     scheduler currently associated with the calling context.
    /// </remarks>
    /// <seealso cref="Task Scheduler (Concurrency Runtime)"/>
    /// <seealso cref="ScheduleGroup Class"/>
    /// <seealso cref="location Class"/>
    /**/
    _CONCRTIMP static void __cdecl ScheduleTask(_In_ TaskProc _Proc, _Inout_opt_ void * _Data);

    /// <summary>
    ///     Schedules a light-weight task within the scheduler associated with the calling context. The light-weight task will be placed
    ///     in a schedule group determined by the runtime. The version that takes the parameter <paramref name="_Placement"/> causes the task
    ///     to be biased towards executing at the specified location.
    /// </summary>
    /// <param name="_Proc">
    ///     A pointer to the function to execute to perform the body of the light-weight task.
    /// </param>
    /// <param name="_Data">
    ///     A void pointer to the data that will be passed as a parameter to the body of the task.
    /// </param>
    /// <param name="_Placement">
    ///     A reference to a location where the light-weight task will be biased towards executing at.
    /// </param>
    /// <remarks>
    ///     This method will result in the process' default scheduler being created and/or attached to the calling context if there is no
    ///     scheduler currently associated with the calling context.
    /// </remarks>
    /// <seealso cref="Task Scheduler (Concurrency Runtime)"/>
    /// <seealso cref="ScheduleGroup Class"/>
    /// <seealso cref="location Class"/>
    /**/
    _CONCRTIMP static void __cdecl ScheduleTask(TaskProc _Proc, _Inout_opt_ void * _Data, location& _Placement);

    /// <summary>
    ///     Determines whether a given location is available on the current scheduler.
    /// </summary>
    /// <param name="_Placement">
    ///     A reference to the location to query the current scheduler about.
    /// </param>
    /// <returns>
    ///     An indication of whether or not the location specified by the <paramref name="_Placement"/> argument is available on the current
    ///     scheduler.
    /// </returns>
    /// <remarks>
    ///     This method will not result in scheduler attachment if the calling context is not already associated with a scheduler.
    ///     <para>Note that the return value is an instantaneous sampling of whether the given location is available. In the presence of multiple
    ///     schedulers, dynamic resource management can add or take away resources from schedulers at any point. Should this happen, the given
    ///     location can change availability.</para>
    /// </remarks>
    /**/
    _CONCRTIMP static bool __cdecl IsAvailableLocation(const location& _Placement);
};

/// <summary>
///     Represents an abstraction for a Concurrency Runtime scheduler.
/// </summary>
/// <remarks>
///     The Concurrency Runtime scheduler uses execution contexts, which map to the operating system execution contexts, such as a thread,
///     to execute the work queued to it by your application. At any time, the concurrency level of a scheduler is equal to the number of virtual processor
///     granted to it by the Resource Manager. A virtual processor is an abstraction for a processing resource and maps to a hardware thread on the
///     underlying system. Only a single scheduler context can execute on a virtual processor at a given time.
///     <para> The Concurrency Runtime will create a default scheduler per process to execute parallel work. In addition you can create your own scheduler
///     instances and manipulate it using this class.</para>
/// </remarks>
/// <seealso cref="Scheduler Class"/>
/// <seealso cref="PolicyElementKey Enumeration"/>
/// <seealso cref="Task Scheduler (Concurrency Runtime)"/>
/**/
class Scheduler
{
protected:
    /// <summary>
    ///     An object of the <c>Scheduler</c> class can only created using factory methods, or implicitly.
    /// </summary>
    /// <remarks>
    ///     The process' default scheduler is created implicitly when you utilize many of the runtime functions which require a scheduler
    ///     to be attached to the calling context. Methods within the <c>CurrentScheduler</c> class and features of the PPL and agents layers
    ///     typically perform implicit attachment.
    ///     <para>You can also create a scheduler explicitly through either the <c>CurrentScheduler::Create</c> method or the <c>Scheduler::Create</c>
    ///     method.</para>
    /// </remarks>
    /// <seealso cref="CurrentScheduler Class"/>
    /// <seealso cref="CurrentScheduler::Create Method"/>
    /// <seealso cref="Scheduler::Create Method"/>
    /**/
    Scheduler() {}

    /// <summary>
    ///     An object of the <c>Scheduler</c> class is implicitly destroyed when all external references to it cease to exist.
    /// </summary>
    /**/
    virtual ~Scheduler() {}

public:

    /// <summary>
    ///     Creates a new scheduler whose behavior is described by the <paramref name="_Policy"/> parameter, places an initial reference on
    ///     the scheduler, and returns a pointer to it.
    /// </summary>
    /// <param name="_Policy">
    ///     The scheduler policy that describes behavior of the newly created scheduler.
    /// </param>
    /// <returns>
    ///     A pointer to a newly created scheduler. This <c>Scheduler</c> object has an initial reference count placed on it.
    /// </returns>
    /// <remarks>
    ///     After a scheduler is created with the <c>Create</c> method, you must call the <see cref="Release Method">Release</see> method at some point
    ///     in the future in order to remove the initial reference count and allow the scheduler to shut down.
    ///     <para>A scheduler created with this method is not attached to the calling context. It can be attached to a context using the
    ///     <see cref="Scheduler::Attach Method">Attach</see> method.</para>
    ///     <para>This method can throw a variety of exceptions, including <see cref="scheduler_resource_allocation_error Class">
    ///     scheduler_resource_allocation_error</see> and <see cref="invalid_scheduler_policy_value Class">invalid_scheduler_policy_value</see>.</para>
    /// </remarks>
    /// <seealso cref="Scheduler::Release Method"/>
    /// <seealso cref="Scheduler::Attach Method"/>
    /// <seealso cref="CurrentScheduler::Create Method"/>
    /// <seealso cref="PolicyElementKey Enumeration"/>
    /// <seealso cref="Task Scheduler (Concurrency Runtime)"/>
    /**/
    _CONCRTIMP static Scheduler * __cdecl Create(const SchedulerPolicy& _Policy);

    /// <summary>
    ///     Returns a unique identifier for the scheduler.
    /// </summary>
    /// <returns>
    ///     A unique identifier for the scheduler.
    /// </returns>
    /**/
    virtual unsigned int Id() const =0;

    /// <summary>
    ///     Returns the current number of virtual processors for the scheduler.
    /// </summary>
    /// <returns>
    ///     The current number of virtual processors for the scheduler.
    ///     <para>The return value from this method is an instantaneous sampling of the number of virtual processors for the scheduler.
    ///     This value can be stale the moment it is returned.</para>
    /// </returns>
    /**/
    virtual unsigned int GetNumberOfVirtualProcessors() const =0;


    /// <summary>
    ///     Returns a copy of the policy that the scheduler was created with.
    /// </summary>
    /// <returns>
    ///     A copy of the policy that the scheduler was created with.
    /// </returns>
    /// <seealso cref="SchedulerPolicy Class"/>
    /// <seealso cref="PolicyElementKey Enumeration"/>
    /// <seealso cref="Task Scheduler (Concurrency Runtime)"/>
    /**/
    virtual SchedulerPolicy GetPolicy() const =0;

    /// <summary>
    ///     Increments the scheduler reference count.
    /// </summary>
    /// <returns>
    ///     The newly incremented reference count.
    /// </returns>
    /// <remarks>
    ///     This is typically used to manage the lifetime of the scheduler for composition. When the reference count of a scheduler
    ///     falls to zero, the scheduler will shut down and destruct itself after all work on the scheduler has completed.
    ///     <para>The method will throw an <see cref="improper_scheduler_reference Class">improper_scheduler_reference</see> exception if the reference
    ///     count prior to calling the <c>Reference</c> method was zero and the call is made from a context that is not owned by the scheduler.</para>
    /// </remarks>
    /// <seealso cref="Scheduler::Release Method"/>
    /// <seealso cref="Scheduler::Create Method"/>
    /**/
    virtual unsigned int Reference() =0 ;

    /// <summary>
    ///     Decrements the scheduler reference count.
    /// </summary>
    /// <returns>
    ///     The newly decremented reference count.
    /// </returns>
    /// <remarks>
    ///     This is typically used to manage the lifetime of the scheduler for composition. When the reference count of a scheduler
    ///     falls to zero, the scheduler will shut down and destruct itself after all work on the scheduler has completed.
    /// </remarks>
    /// <seealso cref="Scheduler::Reference Method"/>
    /// <seealso cref="Scheduler::Create Method"/>
    /**/
    virtual unsigned int Release() =0;

    /// <summary>
    ///     Causes the Windows event handle passed in the <paramref name="_Event"/> parameter to be signaled when the scheduler
    ///     shuts down and destroys itself. At the time the event is signaled, all work that had been scheduled to the
    ///     scheduler is complete. Multiple shutdown events can be registered through this method.
    /// </summary>
    /// <param name="_Event">
    ///     A handle to a Windows event object which will be signaled by the runtime when the scheduler shuts down and destroys itself.
    /// </param>
    /**/
    virtual void RegisterShutdownEvent(HANDLE _Event) =0;

    /// <summary>
    ///     Attaches the scheduler to the calling context. After this method returns, the calling context is managed by the scheduler and
    ///     the scheduler becomes the current scheduler.
    /// </summary>
    /// <remarks>
    ///     Attaching a scheduler implicitly places a reference on the scheduler.
    ///     <para>At some point in the future, you must call the <see cref="CurrentScheduler::Detach Method">CurrentScheduler::Detach</see>
    ///     method in order to allow the scheduler to shut down.</para>
    ///     <para>If this method is called from a context that is already attached to a different scheduler, the existing scheduler is remembered
    ///     as the previous scheduler, and the newly created scheduler becomes the current scheduler. When you call the <c>CurrentScheduler::Detach</c>
    ///     method at a later point, the previous scheduler is restored as the current scheduler.</para>
    ///     <para>This method will throw an <see cref="improper_scheduler_attach Class">improper_scheduler_attach</see> exception if this scheduler
    ///     is the current scheduler of the calling context.</para>
    /// </remarks>
    /// <seealso cref="CurrentScheduler::Detach Method"/>
    /**/
    virtual void Attach() =0;

    /// <summary>
    ///     Allows a user defined policy to be used to create the default scheduler. This method can be called only when no default
    ///     scheduler exists within the process. After a default policy has been set, it remains in effect until the next valid call
    ///     to either the <c>SetDefaultSchedulerPolicy</c> or the <see cref="Scheduler::ResetDefaultSchedulerPolicy Method">ResetDefaultSchedulerPolicy
    ///     </see> method.
    /// </summary>
    /// <param name="_Policy">
    ///     The policy to be set as the default scheduler policy.
    /// </param>
    /// <remarks>
    ///     If the <c>SetDefaultSchedulerPolicy</c> method is called when a default scheduler already exists within the process, the runtime
    ///     will throw a <see cref="default_scheduler_exists Class">default_scheduler_exists</see> exception.
    /// </remarks>
    /// <seealso cref="Scheduler::ResetDefaultSchedulerPolicy Method"/>
    /// <seealso cref="SchedulerPolicy Class"/>
    /// <seealso cref="PolicyElementKey Enumeration"/>
    /// <seealso cref="Task Scheduler (Concurrency Runtime)"/>
    /**/
    _CONCRTIMP static void __cdecl SetDefaultSchedulerPolicy(const SchedulerPolicy& _Policy);

    /// <summary>
    ///     Resets the default scheduler policy to the runtime default. The next time a default scheduler is created, it will use the
    ///     runtime default policy settings.
    /// </summary>
    /// <remarks>
    ///     This method can be called while a default scheduler exists within the process. It will not affect the policy of the existing
    ///     default scheduler. However, if the default scheduler were to shutdown, and a new default were to be created at a later
    ///     point, the new scheduler would use the runtime default policy settings.
    /// </remarks>
    /// <seealso cref="Scheduler::SetDefaultSchedulerPolicy Method"/>
    /// <seealso cref="SchedulerPolicy Class"/>
    /**/
    _CONCRTIMP static void __cdecl ResetDefaultSchedulerPolicy();

    /// <summary>
    ///     Creates a new schedule group within the scheduler. The version that takes the parameter <paramref name="_Placement"/> causes tasks
    ///     within the newly created schedule group to be biased towards executing at the location specified by that parameter.
    /// </summary>
    /// <returns>
    ///     A pointer to the newly created schedule group. This <c>ScheduleGroup</c> object has an initial reference count placed on it.
    /// </returns>
    /// <remarks>
    ///     You must invoke the <see cref="ScheduleGroup::Release Method">Release</see> method on a schedule group when you are
    ///     done scheduling work to it. The scheduler will destroy the schedule group when all work queued to it has completed.
    ///     <para>Note that if you explicitly created this scheduler, you must release all references to schedule groups within it, before
    ///     you release your references on the scheduler.</para>
    /// </remarks>
    /// <seealso cref="ScheduleGroup Class"/>
    /// <seealso cref="ScheduleGroup::Release Method"/>
    /// <seealso cref="Task Scheduler (Concurrency Runtime)"/>
    /// <seealso cref="location Class"/>
    /**/
    virtual ScheduleGroup * CreateScheduleGroup() =0;

    /// <summary>
    ///     Creates a new schedule group within the scheduler. The version that takes the parameter <paramref name="_Placement"/> causes tasks
    ///     within the newly created schedule group to be biased towards executing at the location specified by that parameter.
    /// </summary>
    /// <param name="_Placement">
    ///     A reference to a location where the tasks within the schedule group will biased towards executing at.
    /// </param>
    /// <returns>
    ///     A pointer to the newly created schedule group. This <c>ScheduleGroup</c> object has an initial reference count placed on it.
    /// </returns>
    /// <remarks>
    ///     You must invoke the <see cref="ScheduleGroup::Release Method">Release</see> method on a schedule group when you are
    ///     done scheduling work to it. The scheduler will destroy the schedule group when all work queued to it has completed.
    ///     <para>Note that if you explicitly created this scheduler, you must release all references to schedule groups within it, before
    ///     you release your references on the scheduler.</para>
    /// </remarks>
    /// <seealso cref="ScheduleGroup Class"/>
    /// <seealso cref="ScheduleGroup::Release Method"/>
    /// <seealso cref="Task Scheduler (Concurrency Runtime)"/>
    /// <seealso cref="location Class"/>
    /**/
    virtual ScheduleGroup * CreateScheduleGroup(location& _Placement) =0;

    /// <summary>
    ///     Schedules a light-weight task within the scheduler. The light-weight task will be placed in a schedule group determined by the runtime.
    ///     The version that takes the parameter <paramref name="_Placement"/> causes the task to be biased towards executing at the specified location.
    /// </summary>
    /// <param name="_Proc">
    ///     A pointer to the function to execute to perform the body of the light-weight task.
    /// </param>
    /// <param name="_Data">
    ///     A void pointer to the data that will be passed as a parameter to the body of the task.
    /// </param>
    /// <seealso cref="Task Scheduler (Concurrency Runtime)"/>
    /// <seealso cref="ScheduleGroup Class"/>
    /// <seealso cref="location Class"/>
    /**/
    virtual void ScheduleTask(TaskProc _Proc, _Inout_opt_ void * _Data) =0;

    /// <summary>
    ///     Schedules a light-weight task within the scheduler. The light-weight task will be placed in a schedule group determined by the runtime.
    ///     The version that takes the parameter <paramref name="_Placement"/> causes the task to be biased towards executing at the specified location.
    /// </summary>
    /// <param name="_Proc">
    ///     A pointer to the function to execute to perform the body of the light-weight task.
    /// </param>
    /// <param name="_Data">
    ///     A void pointer to the data that will be passed as a parameter to the body of the task.
    /// </param>
    /// <param name="_Placement">
    ///     A reference to a location where the light-weight task will be biased towards executing at.
    /// </param>
    /// <seealso cref="Task Scheduler (Concurrency Runtime)"/>
    /// <seealso cref="ScheduleGroup Class"/>
    /// <seealso cref="location Class"/>
    /**/
    virtual void ScheduleTask(TaskProc _Proc, _Inout_opt_ void * _Data, location& _Placement) =0;

    /// <summary>
    ///     Determines whether a given location is available on the scheduler.
    /// </summary>
    /// <param name="_Placement">
    ///     A reference to the location to query the scheduler about.
    /// </param>
    /// <returns>
    ///     An indication of whether or not the location specified by the <paramref name="_Placement"/> argument is available on the scheduler.
    /// </returns>
    /// <remarks>
    ///     Note that the return value is an instantaneous sampling of whether the given location is available. In the presence of multiple
    ///     schedulers, dynamic resource management can add or take away resources from schedulers at any point. Should this happen, the given
    ///     location can change availability.
    /// </remarks>
    /**/
    virtual bool IsAvailableLocation(const location& _Placement) const =0;
};

/// <summary>
///     Represents an abstraction for an execution context.
/// </summary>
/// <remarks>
///     The Concurrency Runtime scheduler (see <see cref="Scheduler Class">Scheduler</see>) uses execution contexts to execute the work queued
///     to it by your application. A Win32 thread is an example of an execution context on a Windows
///     operating system.
///     <para>At any time, the concurrency level of a scheduler is equal to the number of virtual processors granted to it by the Resource Manager.
///     A virtual processor is an abstraction for a processing resource and maps to a hardware thread on the underlying system. Only a single scheduler
///     context can execute on a virtual processor at a given time.</para>
///     <para> The scheduler is cooperative in nature and an executing context can yield its virtual processor to a different context at any time if
///     it wishes to enter a wait state. When its wait it satisfied, it cannot resume until an available virtual processor from the scheduler begins
///     executing it.</para>
/// </remarks>
/// <seealso cref="Scheduler Class"/>
/// <seealso cref="Task Scheduler (Concurrency Runtime)"/>
/**/
class Context
{
public:
    /// <summary>
    ///     Returns an identifier for the context that is unique within the scheduler to which the context belongs.
    /// </summary>
    /// <returns>
    ///     An identifier for the context that is unique within the scheduler to which the context belongs.
    /// </returns>
    /**/
    virtual unsigned int GetId() const =0;

    /// <summary>
    ///     Returns an identifier for the virtual processor that the context is currently executing on.
    /// </summary>
    /// <returns>
    ///     If the context is currently executing on a virtual processor, an identifier for the virtual processor that the context
    ///     is currently executing on; otherwise, the value <c>-1</c>.
    /// </returns>
    /// <remarks>
    ///     The return value from this method is an instantaneous sampling of the virtual processor that the context is executing
    ///     on. This value can be stale the moment it is returned and cannot be relied upon. Typically, this method is used
    ///     for debugging or tracing purposes only.
    /// </remarks>
    /**/
    virtual unsigned int GetVirtualProcessorId() const =0;

    /// <summary>
    ///     Returns an identifier for the schedule group that the context is currently working on.
    /// </summary>
    /// <returns>
    ///     An identifier for the schedule group the context is currently working on.
    /// </returns>
    /// <remarks>
    ///     The return value from this method is an instantaneous sampling of the schedule group that the context is executing
    ///     on. If this method is called on a context other than the current context, the value can be stale the moment it is
    ///     returned and cannot be relied upon. Typically, this method is used for debugging or tracing purposes only.
    /// </remarks>
    /// <seealso cref="ScheduleGroup Class"/>
    /**/
    virtual unsigned int GetScheduleGroupId() const =0;

    /// <summary>
    ///     Returns an identifier for the current context that is unique within the scheduler to which the current context belongs.
    /// </summary>
    /// <returns>
    ///     If the current context is attached to a scheduler, an identifier for the current context that is unique within the scheduler
    ///     to which the current context belongs; otherwise, the value <c>-1</c>.
    /// </returns>
    /**/
    _CONCRTIMP static unsigned int __cdecl Id();

    /// <summary>
    ///     Returns an identifier for the virtual processor that the current context is executing on.
    /// </summary>
    /// <returns>
    ///     If the current context is attached to a scheduler, an identifier for the virtual processor that the current context is
    ///     executing on; otherwise, the value <c>-1</c>.
    /// </returns>
    /// <remarks>
    ///     The return value from this method is an instantaneous sampling of the virtual processor that the current context is executing
    ///     on. This value can be stale the moment it is returned and cannot be relied upon. Typically, this method is used
    ///     for debugging or tracing purposes only.
    /// </remarks>
    /**/
    _CONCRTIMP static unsigned int __cdecl VirtualProcessorId();

    /// <summary>
    ///     Returns an identifier for the schedule group that the current context is working on.
    /// </summary>
    /// <returns>
    ///     If the current context is attached to a scheduler and working on a schedule group, an identifier for the scheduler group that the
    ///     current context is working on; otherwise, the value <c>-1</c>.
    /// </returns>
    /// <seealso cref="ScheduleGroup Class"/>
    /**/
    _CONCRTIMP static unsigned int __cdecl ScheduleGroupId();

    /// <summary>
    ///     Blocks the current context.
    /// </summary>
    /// <remarks>
    ///     This method will result in the process' default scheduler being created and/or attached to the calling context if there is no
    ///     scheduler currently associated with the calling context.
    ///     <para>If the calling context is running on a virtual processor, the virtual processor will find another runnable context to
    ///     execute or can potentially create a new one.</para>
    ///     <para>After the <c>Block</c> method has been called or will be called, you must pair it with a call to the <see cref="Context::Unblock Method">
    ///     Unblock</see> method from another execution context in order for it to run again. Be aware that there is a critical period between
    ///     the point where your code publishes its context for another thread to be able to call the <c>Unblock</c> method and the point
    ///     where the actual method call to <c>Block</c> is made. During this period, you must not call any method which
    ///     can in turn block and unblock for its own reasons (for example, acquiring a lock). Calls to the <c>Block</c> and <c>Unblock</c> method
    ///     do not track the reason for the blocking and unblocking. Only one object should have ownership of a <c>Block</c>-<c>Unblock</c>
    ///     pair.</para>
    ///     <para>This method can throw a variety of exceptions, including <see cref="scheduler_resource_allocation_error Class">
    ///     scheduler_resource_allocation_error</see>.</para>
    /// </remarks>
    /// <seealso cref="Context::Unblock Method"/>
    /// <seealso cref="Task Scheduler (Concurrency Runtime)"/>
    /**/
    _CONCRTIMP static void __cdecl Block();

    /// <summary>
    ///     Unblocks the context and causes it to become runnable.
    /// </summary>
    /// <remarks>
    ///     It is perfectly legal for a call to the <c>Unblock</c> method to come before a corresponding call to the <see cref="Context::Block Method">
    ///     Block</see> method. As long as calls to the <c>Block</c> and <c>Unblock</c> methods are properly paired, the runtime properly handles the natural race of
    ///     either ordering. An <c>Unblock</c> call coming before a <c>Block</c> call simply negates the effect of the <c>Block</c> call.
    ///     <para>There are several exceptions which can be thrown from this method. If a context attempts to call the <c>Unblock</c> method on
    ///     itself, a <see cref="context_self_unblock Class">context_self_unblock</see> exception will be thrown. If calls to <c>Block</c> and
    ///     <c>Unblock</c> are not properly paired (for example, two calls to <c>Unblock</c> are made for a context which is currently running), a
    ///     <see cref="context_unblock_unbalanced Class">context_unblock_unbalanced</see> exception will be thrown.</para>
    ///
    ///     <para>Be aware that there is a critical period between the point where your code publishes its context for another thread to
    ///     be able to call the <c>Unblock</c> method and the point where the actual method call to <c>Block</c> is made. During this period,
    ///     you must not call any method which can in turn block and unblock for its own reasons (for example, acquiring a lock).
    ///     Calls to the <c>Block</c> and <c>Unblock</c> method do not track the reason for the blocking and unblocking. Only one object should have
    ///     ownership of a <c>Block</c> and <c>Unblock</c> pair.</para>
    /// </remarks>
    /// <seealso cref="Context::Block Method"/>
    /// <seealso cref="Task Scheduler (Concurrency Runtime)"/>
    /**/
    virtual void Unblock() =0;

    /// <summary>
    ///     Determines whether or not the context is synchronously blocked. A context is considered to be synchronously
    ///     blocked if it explicitly performed an action which led to blocking.
    /// </summary>
    /// <returns>
    ///     Whether the context is synchronously blocked.
    /// </returns>
    /// <remarks>
    ///     A context is considered to be synchronously blocked if it explicitly performed an action which led to blocking. On the thread scheduler,
    ///     this would indicate a direct call to the <c>Context::Block</c> method or a synchronization object which was built using the
    ///     <c>Context::Block</c> method.
    ///     <para>The return value from this method is an instantaneous sample of whether the context is synchronously blocked. This value may
    ///     be stale the moment it is returned and can only be used under very specific circumstances.</para>
    /// </remarks>
    /// <seealso cref="Context::Block Method"/>
    /**/
    virtual bool IsSynchronouslyBlocked() const =0;

    /// <summary>
    ///     Yields execution so that another context can execute. If no other context is available to yield to,
    ///     the method simply returns.
    /// </summary>
    /// <remarks>
    ///     This yield variant is intended for use within spin loops.
    ///     <para>This method will result in the process' default scheduler being created and/or attached to the calling context if there is no
    ///     scheduler currently associated with the calling context.</para>
    /// </remarks>
    /**/
    _CONCRTIMP static void __cdecl _SpinYield();

    /// <summary>
    ///     Yields execution so that another context can execute. If no other context is available to yield to, the scheduler
    ///     can yield to another operating system thread.
    /// </summary>
    /// <remarks>
    ///     This method will result in the process' default scheduler being created and/or attached to the calling context if there is no
    ///     scheduler currently associated with the calling context.
    /// </remarks>
    /// <seealso cref="Context::Block Method"/>
    /// <seealso cref="Context::Unblock Method"/>
    /**/
    _CONCRTIMP static void (__cdecl Yield)();

    /// <summary>
    ///     Yields execution so that another context can execute. If no other context is available to yield to, the scheduler can yield to
    ///     another operating system thread.
    /// </summary>
    /// <remarks>
    ///     Yield is defined as a function-like macro in the Windows SDK headers, thus requiring the unfamiliar (Yield)() construct to call
    ///     the function as opposed to using the macro. This alias is provided to avoid it.
    /// </remarks>
    /// <seealso cref="Context::Block Method"/>
    /// <seealso cref="Context::Unblock Method"/>
    /**/
    static inline void __cdecl YieldExecution()
    {
        (Yield)();
    }

    /// <summary>
    ///     Returns an indication of whether the task collection which is currently executing inline on the current context
    ///     is in the midst of an active cancellation (or will be shortly).
    /// </summary>
    /// <returns>
    ///     If a scheduler is attached to the calling context and a task group is executing a task inline on that context,
    ///     an indication of whether that task group is in the midst of an active cancellation (or will be shortly); otherwise,
    ///     the value <c>false</c>.
    ///     <para>This method will not result in scheduler attachment if the calling context is not already associated with a scheduler.</para>
    /// </returns>
    /**/
    _CONCRTIMP static bool __cdecl IsCurrentTaskCollectionCanceling();

    /// <summary>
    ///     Returns a pointer to the current context.
    /// </summary>
    /// <returns>
    ///     A pointer to the current context.
    /// </returns>
    /// <remarks>
    ///     This method will result in the process' default scheduler being created and/or attached to the calling context if there is no
    ///     scheduler currently associated with the calling context.
    /// </remarks>
    /**/
    _CONCRTIMP static _Ret_notnull_ Context * __cdecl CurrentContext();

    /// <summary>
    ///     Injects an additional virtual processor into a scheduler for the duration of a block of code when invoked on a context executing
    ///     on one of the virtual processors in that scheduler.
    /// </summary>
    /// <param name="_BeginOversubscription">
    ///     If <c>true</c>, an indication that an extra virtual processor should be added for the duration of the oversubscription.
    ///     If <c>false</c>, an indication that the oversubscription should end and the previously added virtual processor should be removed.
    /// </param>
    /// <seealso cref="Task Scheduler (Concurrency Runtime)"/>
    /**/
    _CONCRTIMP static void __cdecl Oversubscribe(bool _BeginOversubscription);

protected:

    //
    // Privatize operator delete. The scheduler internally manages contexts.
    //
    template<class _Ty> friend void ::Concurrency::details::_InternalDeleteHelper(_Ty * _PObject);

    virtual ~Context() {};
};

#endif  /* _CRT_USE_WINAPI_FAMILY_DESKTOP_APP */

/// <summary>
///     Value indicating that a wait timed out.
/// </summary>
/// <seealso cref="event Class"/>
/// <seealso cref="event::wait Method"/>
/// <seealso cref="event::wait_for_multiple Method"/>
/**/
const size_t COOPERATIVE_WAIT_TIMEOUT = SIZE_MAX;

/// <summary>
///     Value indicating that a wait should never time out.
/// </summary>
/// <seealso cref="event Class"/>
/// <seealso cref="event::wait Method"/>
/// <seealso cref="event::wait_for_multiple Method"/>
/**/
const unsigned int COOPERATIVE_TIMEOUT_INFINITE = (unsigned int)-1;

/// <summary>
///     A non-reentrant mutex which is explicitly aware of the Concurrency Runtime.
/// </summary>
/// <remarks>
///     For more information, see <see cref="Synchronization Data Structures"/>.
/// </remarks>
/// <seealso cref="reader_writer_lock Class"/>
/**/
class critical_section
{
public:

    /// <summary>
    ///     Constructs a new critical section.
    /// </summary>
    /**/
    _CONCRTIMP critical_section();

    /// <summary>
    ///     Destroys a critical section.
    /// </summary>
    /// <remarks>
    ///     It is expected that the lock is no longer held when the destructor runs. Allowing the critical section to destruct with the lock
    ///     still held results in undefined behavior.
    /// </remarks>
    /**/
    _CONCRTIMP ~critical_section();

    /// <summary>
    ///     Acquires this critical section.
    /// </summary>
    /// <remarks>
    ///     It is often safer to utilize the <see cref="critical_section::scoped_lock Class">scoped_lock</see> construct to acquire and release
    ///     a <c>critical_section</c> object in an exception safe way.
    ///     <para>If the lock is already held by the calling context, an <see cref="improper_lock Class">improper_lock</see> exception will be
    ///     thrown.</para>
    /// </remarks>
    /// <seealso cref="critical_section::unlock Method"/>
    /// <seealso cref="critical_section::scoped_lock Class"/>
    /**/
    _CONCRTIMP void lock();

    /// <summary>
    ///     Tries to acquire the lock without blocking.
    /// </summary>
    /// <returns>
    ///     If the lock was acquired, the value <c>true</c>; otherwise, the value <c>false</c>.
    /// </returns>
    /// <seealso cref="critical_section::unlock Method"/>
    /**/
    _CONCRTIMP bool try_lock();

    /// <summary>
    ///     Tries to acquire the lock without blocking for a specific number of milliseconds.
    /// </summary>
    /// <param name="_Timeout">
    ///     The number of milliseconds to wait before timing out.
    /// </param>
    /// <returns>
    ///     If the lock was acquired, the value <c>true</c>; otherwise, the value <c>false</c>.
    /// </returns>
    /// <seealso cref="critical_section::unlock Method"/>
    /**/
    _CONCRTIMP bool try_lock_for(unsigned int _Timeout);

    /// <summary>
    ///     Unlocks the critical section.
    /// </summary>
    /// <seealso cref="critical_section::lock Method"/>
    /// <seealso cref="critical_section::try_lock Method"/>
    /**/
    _CONCRTIMP void unlock();

    /// <summary>
    ///     A reference to a <c>critical_section</c> object.
    /// </summary>
    /**/
    typedef critical_section& native_handle_type;

    /// <summary>
    ///     Returns a platform specific native handle, if one exists.
    /// </summary>
    /// <returns>
    ///     A reference to the critical section.
    /// </returns>
    /// <remarks>
    ///     A <c>critical_section</c> object is not associated with a platform specific native handle for the Windows operating system.
    ///     The method simply returns a reference to the object itself.
    /// </remarks>
    /**/
    _CONCRTIMP native_handle_type native_handle();

    /// <summary>
    ///     Guarantees that if any context holds the lock at the time the method is called, that context has released
    ///     the lock before this method returns.
    /// </summary>
    /// <remarks>
    ///     If no context holds the lock at the instant this method is called, it returns instantly.
    /// </remarks>
    /**/
    void _Flush_current_owner();

    /// <summary>
    ///     Acquires this critical section given a specific node to lock.
    /// </summary>
    /// <param name="_PLockingNode">
    ///     The node that needs to own the lock.
    /// </param>
    /// <param name="_FHasExternalNode">
    ///     An indication if the node being locked is external to the critical_section.
    /// </param>
    /// <remarks>
    ///     If the lock is already held by the calling context, an <see cref="improper_lock Class">.improper_lock</see> exception will be thrown.
    /// </remarks>
    /**/
    bool _Acquire_lock(void * _PLockingNode, bool _FHasExternalNode);

    /// <summary>
    ///     An exception safe RAII wrapper for a <c>critical_section</c> object.
    /// </summary>
    /**/
    class scoped_lock
    {
    public:

        /// <summary>
        ///     Constructs a <c>scoped_lock</c> object and acquires the <c>critical_section</c> object passed in the <paramref name="_Critical_section"/>
        ///     parameter. If the critical section is held by another thread, this call will block.
        /// </summary>
        /// <param name="_Critical_section">
        ///     The critical section to lock.
        /// </param>
        /// <seealso cref="critical_section Class"/>
        /**/
        explicit _CONCRTIMP scoped_lock(critical_section& _Critical_section);

        /// <summary>
        ///     Destroys a <c>scoped_lock</c> object and releases the critical section supplied in its constructor.
        /// </summary>
        /// <seealso cref="critical_section Class"/>
        /**/
        _CONCRTIMP ~scoped_lock();

    private:

        critical_section& _M_critical_section;
        _CONCRT_BUFFER _M_node[(4 * sizeof(void *) + 2 * sizeof(unsigned int) + sizeof(_CONCRT_BUFFER) - 1) / sizeof(_CONCRT_BUFFER)]{};

        scoped_lock(const scoped_lock&);                    // no copy constructor
        scoped_lock const & operator=(const scoped_lock&);  // no assignment operator
    };

private:
    /// <summary>
    ///     The node allocated on the stack never really owns the lock because it would go out of scope and the insides would not be visible
    ///     in unlock() where it could potentially need to unblock the next in the queue. Instead, its state is transferred to the internal
    ///     node which is used as a scratch node.
    /// </summary>
    /// <param name="_PLockingNode">
    ///     The node that needs to own the lock.
    /// </param>
    /**/
    void _Switch_to_active(void * _PLockingNode);

    _CONCRT_BUFFER  _M_activeNode[(4 * sizeof(void *) + 2 * sizeof(unsigned int) + sizeof(_CONCRT_BUFFER) - 1) / sizeof(_CONCRT_BUFFER)]{};
    void * volatile _M_pHead;
    void * volatile _M_pTail;

    /// <summary>
    ///     Hide copy constructor for a critical section
    /// </summary>
    /**/
    critical_section(const critical_section&);

    /// <summary>
    ///     Hide assignment operator for a critical section
    /// </summary>
    /**/
    critical_section& operator=(const critical_section&);
};

/// <summary>
///    A writer-preference queue-based reader-writer lock with local only spinning. The lock grants first in - first out (FIFO) access to writers
///    and starves readers under a continuous load of writers.
/// </summary>
/// <remarks>
///     For more information, see <see cref="Synchronization Data Structures"/>.
/// </remarks>
/// <seealso cref="critical_section Class"/>
/**/
class reader_writer_lock
{
public:

    /// <summary>
    ///     Constructs a new <c>reader_writer_lock</c> object.
    /// </summary>
    /**/
    _CONCRTIMP reader_writer_lock();

    /// <summary>
    ///     Destroys the <c>reader_writer_lock</c> object.
    /// </summary>
    /// <remarks>
    ///     It is expected that the lock is no longer held when the destructor runs. Allowing the reader writer lock to destruct with the lock
    ///     still held results in undefined behavior.
    /// </remarks>
    /**/
    _CONCRTIMP ~reader_writer_lock();

    /// <summary>
    ///     Acquires the reader-writer lock as a writer.
    /// </summary>
    /// <remarks>
    ///     It is often safer to utilize the <see cref="reader_writer_lock::scoped_lock Class">scoped_lock</see> construct to acquire and release
    ///     a <c>reader_writer_lock</c> object as a writer in an exception safe way.
    ///     <para>After a writer attempts to acquire the lock, any future readers will block until the writers have successfully acquired
    ///     and released the lock. This lock is biased towards writers and can starve readers under a continuous load of writers.</para>
    ///     <para>Writers are chained so that a writer exiting the lock releases the next writer in line.</para>
    ///     <para>If the lock is already held by the calling context, an <see cref="improper_lock Class">improper_lock</see> exception will be
    ///     thrown.</para>
    /// </remarks>
    /// <seealso cref="reader_writer_lock::unlock Method"/>
    /**/
    _CONCRTIMP void lock();

    /// <summary>
    ///     Attempts to acquire the reader-writer lock as a writer without blocking.
    /// </summary>
    /// <returns>
    ///     If the lock was acquired, the value <c>true</c>; otherwise, the value <c>false</c>.
    /// </returns>
    /// <seealso cref="reader_writer_lock::unlock Method"/>
    /**/
    _CONCRTIMP bool try_lock();

    /// <summary>
    ///     Acquires the reader-writer lock as a reader. If there are writers, active readers have to wait until they are done.
    ///     The reader simply registers an interest in the lock and waits for writers to release it.
    /// </summary>
    /// <remarks>
    ///     It is often safer to utilize the <see cref="reader_writer_lock::scoped_lock_read Class">scoped_lock_read</see> construct to acquire
    ///     and release a <c>reader_writer_lock</c> object as a reader in an exception safe way.
    ///     <para>If there are writers waiting on the lock, the reader will wait until all writers in line have acquired
    ///     and released the lock. This lock is biased towards writers and can starve readers under a continuous load of writers.</para>
    /// </remarks>
    /// <seealso cref="reader_writer_lock::unlock Method"/>
    /**/
    _CONCRTIMP void lock_read();

    /// <summary>
    ///     Attempts to acquire the reader-writer lock as a reader without blocking.
    /// </summary>
    /// <returns>
    ///     If the lock was acquired, the value <c>true</c>; otherwise, the value <c>false</c>.
    /// </returns>
    /// <seealso cref="reader_writer_lock::unlock Method"/>
    /**/
    _CONCRTIMP bool try_lock_read();

    /// <summary>
    ///     Unlocks the reader-writer lock based on who locked it, reader or writer.
    /// </summary>
    /// <remarks>
    ///     If there are writers waiting on the lock, the release of the lock will always go to the next writer in FIFO
    ///     order.  This lock is biased towards writers and can starve readers under a continuous load of writers.
    /// </remarks>
    /// <seealso cref="reader_writer_lock::lock Method"/>
    /// <seealso cref="reader_writer_lock::lock_read Method"/>
    /// <seealso cref="reader_writer_lock::try_lock Method"/>
    /// <seealso cref="reader_writer_lock::try_lock_read Method"/>
    /**/
    _CONCRTIMP void unlock();

    /// <summary>
    ///     Acquires a write lock given a specific write node to lock.
    /// </summary>
    /// <param name="_PLockingNode">
    ///     The node that needs to own the lock.
    /// </param>
    /// <param name="_FHasExternalNode">
    ///     An indication if the node being locked is external to the <c>reader_writer_lock</c> object.
    /// </param>
    /// <remarks>
    ///     If the lock is already held by the calling context, an <see cref="improper_lock Class">.improper_lock</see> exception will be
    ///     thrown.
    /// </remarks>
    /**/
    void _Acquire_lock(void * _PLockingNode, bool _FHasExternalNode);

    /// <summary>
    ///     An exception safe RAII wrapper that can be used to acquire <c>reader_writer_lock</c> lock objects as a writer.
    /// </summary>
    /**/
    class scoped_lock
    {
    public:
        /// <summary>
        ///     Constructs a <c>scoped_lock</c> object and acquires the <c>reader_writer_lock</c> object passed in the
        ///     <paramref name="_Reader_writer_lock"/> parameter as a writer. If the lock is held by another thread, this call will block.
        /// </summary>
        /// <param name="_Reader_writer_lock">
        ///     The <c>reader_writer_lock</c> object to acquire as a writer.
        /// </param>
        /**/
        explicit _CONCRTIMP scoped_lock(reader_writer_lock& _Reader_writer_lock);

        /// <summary>
        ///     Destroys a <c>reader_writer_lock</c> object and releases the lock supplied in its constructor.
        /// </summary>
        /**/
        _CONCRTIMP ~scoped_lock();

    private:

        reader_writer_lock& _M_reader_writer_lock;
        _CONCRT_BUFFER _M_writerNode[(4 * sizeof(void *) + 2 * sizeof(unsigned int) + sizeof(_CONCRT_BUFFER) - 1) / sizeof(_CONCRT_BUFFER)]{};

        scoped_lock(const scoped_lock&);                    // no copy constructor
        scoped_lock const & operator=(const scoped_lock&);  // no assignment operator
    };

    /// <summary>
    ///     An exception safe RAII wrapper that can be used to acquire <c>reader_writer_lock</c> lock objects as a reader.
    /// </summary>
    /**/
    class scoped_lock_read
    {
    public:
        /// <summary>
        ///     Constructs a <c>scoped_lock_read</c> object and acquires the <c>reader_writer_lock</c> object passed in the
        ///     <paramref name="_Reader_writer_lock"/> parameter as a reader. If the lock is held by another thread as a writer or there
        ///     are pending writers, this call will block.
        /// </summary>
        /// <param name="_Reader_writer_lock">
        ///     The <c>reader_writer_lock</c> object to acquire as a reader.
        /// </param>
        /**/
        explicit _CONCRTIMP scoped_lock_read(reader_writer_lock& _Reader_writer_lock);

        /// <summary>
        ///     Destroys a <c>scoped_lock_read</c> object and releases the lock supplied in its constructor.
        /// </summary>
        /**/
        _CONCRTIMP ~scoped_lock_read();

    private:

        reader_writer_lock& _M_reader_writer_lock;

        scoped_lock_read(const scoped_lock_read&);                    // no copy constructor
        scoped_lock_read const & operator=(const scoped_lock_read&);  // no assignment operator
    };

private:

    /// <summary>
    ///     Called for the first context in the writer queue. It sets the queue head and it tries to
    ///     claim the lock if readers are not active.
    /// </summary>
    /// <param name="_PWriter">
    ///     The first writer in the queue.
    /// </param>
    /**/
    bool _Set_next_writer(void * _PWriter);

    /// <summary>
    ///     Called when writers are done with the lock, or when lock was free for claiming by
    ///     the first reader coming in. If in the meantime there are more writers interested
    ///     the list of readers is finalized and they are convoyed, while head of the list
    ///     is reset to NULL.
    /// </summary>
    /// <returns>
    ///     Pointer to the head of the reader list.
    /// </returns>
    /**/
    void * _Get_reader_convoy();

    /// <summary>
    ///     Called from unlock() when a writer is holding the lock. Writer unblocks the next writer in the list
    ///     and is being retired. If there are no more writers, but there are readers interested, then readers
    ///     are unblocked.
    /// </summary>
    /**/
    void _Unlock_writer();

    /// <summary>
    ///     Called from unlock() when a reader is holding the lock. Reader count is decremented and if this
    ///     is the last reader it checks whether there are interested writers that need to be unblocked.
    /// </summary>
    /**/
    void _Unlock_reader();

    /// <summary>
    ///     When the last writer leaves the lock, it needs to reset the tail to NULL so that the next coming
    ///     writer would know to try to grab the lock. If the CAS to NULL fails, then some other writer
    ///     managed to grab the tail before the reset, so this writer needs to wait until the link to
    ///     the next writer is complete before trying to release the next writer.
    /// </summary>
    /// <param name="_PWriter">
    ///     Last writer in the queue.
    /// </param>
    /**/
    void _Remove_last_writer(void * _PWriter);

    /// <summary>
    ///     The writer node allocated on the stack never really owns the lock because it would go out of scope and the insides would not be
    ///     visible in unlock() where it could potentially need to unblock the next writer in the queue. Instead, its state is transferred to the internal
    ///     writer node which is used as a scratch node.
    /// </summary>
    /// <param name="_PWriter">
    ///     The writer that needs to own the lock.
    /// </param>
    /**/
    void _Switch_to_active(void * _PWriter);

    _CONCRT_BUFFER _M_activeWriter[(4 * sizeof(void *) + 2 * sizeof(unsigned int) + sizeof(_CONCRT_BUFFER) - 1) / sizeof(_CONCRT_BUFFER)]{};
    void *         _M_pReaderHead;
    void *         _M_pWriterHead;
    void *         _M_pWriterTail;
    volatile long  _M_lockState;

    /// <summary>
    ///     Hide copy constructor for a reader_writer_lock
    /// </summary>
    /**/
    reader_writer_lock (const reader_writer_lock& _Lock);

    /// <summary>
    ///     Hide assignment operator for a reader_writer_lock
    /// </summary>
    /**/
    reader_writer_lock& operator=(const reader_writer_lock& _Lock);
};

/// <summary>
///     A manual reset event which is explicitly aware of the Concurrency Runtime.
/// </summary>
/// <remarks>
///     For more information, see <see cref="Synchronization Data Structures"/>.
/// </remarks>
/**/
class event
{
public:

    /// <summary>
    ///     Constructs a new event.
    /// </summary>
    /**/
    _CONCRTIMP event();

    /// <summary>
    ///     Destroys an event.
    /// </summary>
    /// <remarks>
    ///     It is expected that there are no threads waiting on the event when the destructor runs. Allowing the event to destruct with threads
    ///     still waiting on it results in undefined behavior.
    /// </remarks>
    /**/
    _CONCRTIMP ~event();

    /// <summary>
    ///     Waits for the event to become signaled.
    /// </summary>
    /// <param name="_Timeout">
    ///     Indicates the number of milliseconds before the wait times out. The value <c>COOPERATIVE_TIMEOUT_INFINITE</c> signifies that
    ///     there is no timeout.
    /// </param>
    /// <returns>
    ///     If the wait was satisfied, the value <c>0</c> is returned; otherwise, the value <c>COOPERATIVE_WAIT_TIMEOUT</c> to indicate that
    ///     the wait timed out without the event becoming signaled.
    /// </returns>
    /// <seealso cref="event::set Method"/>
    /// <seealso cref="COOPERATIVE_TIMEOUT_INFINITE Constant">COOPERATIVE_TIMEOUT_INFINITE</seealso>
    /// <seealso cref="COOPERATIVE_WAIT_TIMEOUT Constant">COOPERATIVE_WAIT_TIMEOUT</seealso>
    /**/
    _CONCRTIMP size_t wait(unsigned int _Timeout = COOPERATIVE_TIMEOUT_INFINITE);

    /// <summary>
    ///     Signals the event.
    /// </summary>
    /// <remarks>
    ///     Signaling the event can cause an arbitrary number of contexts waiting on the event to become runnable.
    /// </remarks>
    /// <seealso cref="event::wait Method"/>
    /// <seealso cref="event::reset Method"/>
    /**/
    _CONCRTIMP void set();

    /// <summary>
    ///     Resets the event to a non-signaled state.
    /// </summary>
    /// <seealso cref="event::set Method"/>
    /// <seealso cref="event::wait Method"/>
    /**/
    _CONCRTIMP void reset();

    /// <summary>
    ///     Waits for multiple events to become signaled.
    /// </summary>
    /// <param name="_PPEvents">
    ///     An array of events to wait on. The number of events within the array is indicated by the <paramref name="_Count"/> parameter.
    /// </param>
    /// <param name="_Count">
    ///     The count of events within the array supplied in the <paramref name="_PPEvents"/> parameter.
    /// </param>
    /// <param name="_FWaitAll">
    ///     If set to the value <c>true</c>, the parameter specifies that all events within the array supplied in the <paramref name="_PPEvents"/>
    ///     parameter must become signaled in order to satisfy the wait. If set to the value <c>false</c>, it specifies that any event within the
    ///     array supplied in the <paramref name="_PPEvents"/>  parameter becoming signaled will satisfy the wait.
    /// </param>
    /// <param name="_Timeout">
    ///     Indicates the number of milliseconds before the wait times out. The value <c>COOPERATIVE_TIMEOUT_INFINITE</c> signifies that
    ///     there is no timeout.
    /// </param>
    /// <returns>
    ///     If the wait was satisfied, the index within the array supplied in the <paramref name="_PPEvents"/> parameter which satisfied
    ///     the wait condition; otherwise, the value <c>COOPERATIVE_WAIT_TIMEOUT</c> to indicate that the wait timed out without the condition
    ///     being satisfied.
    /// </returns>
    /// <remarks>
    ///     If the parameter <paramref name="_FWaitAll"/> is set to the value <c>true</c> to indicate that all events must become signaled to satisfy
    ///     the wait, the index returned by the function carries no special significance other than the fact that it is not the value
    ///     <c>COOPERATIVE_WAIT_TIMEOUT</c>.
    /// </remarks>
    /// <seealso cref="event::wait Method"/>
    /// <seealso cref="COOPERATIVE_TIMEOUT_INFINITE Constant">COOPERATIVE_TIMEOUT_INFINITE</seealso>
    /// <seealso cref="COOPERATIVE_WAIT_TIMEOUT Constant">COOPERATIVE_WAIT_TIMEOUT</seealso>
    /**/
    _CONCRTIMP static size_t __cdecl wait_for_multiple(_In_reads_(_Count) event ** _PPEvents, size_t _Count, bool _FWaitAll, unsigned int _Timeout = COOPERATIVE_TIMEOUT_INFINITE);


    /// <summary>
    ///     Value indicating that a wait should never time out.
    /// </summary>
    static constexpr unsigned int timeout_infinite = COOPERATIVE_TIMEOUT_INFINITE;
private:

    // Prevent bad usage of copy-constructor and copy-assignment
    event(const event& _Event);
    event& operator=(const event& _Event);

    void * volatile _M_pWaitChain;
    void * _M_pResetChain;
    ::Concurrency::critical_section _M_lock;
};

namespace details
{
    // Base class for all reference counted objects
    class _RefCounterBase
    {
    public:

        virtual ~_RefCounterBase()
        {
            _CONCRT_ASSERT(_M_refCount == 0);
        }

        // Acquires a reference
        // Returns the new reference count.
        long _Reference()
        {
            long _Refcount = _InterlockedIncrement(&_M_refCount);

            // 0 - 1 transition is illegal
            _CONCRT_ASSERT(_Refcount > 1);
            return _Refcount;
        }

        // Releases the reference
        // Returns the new reference count
        long _Release()
        {
            long _Refcount = _InterlockedDecrement(&_M_refCount);
            _CONCRT_ASSERT(_Refcount >= 0);

            if (_Refcount == 0)
            {
                _Destroy();
            }

            return _Refcount;
        }

    protected:

        // Allow derived classes to provide their own deleter
        virtual void _Destroy()
        {
            delete this;
        }

        // Only allow instantiation through derived class
        _RefCounterBase(long _InitialCount = 1) : _M_refCount(_InitialCount)
        {
            _CONCRT_ASSERT(_M_refCount > 0);
        }

        // Reference count
        volatile long _M_refCount;
    };

    class _CancellationTokenState;
    class _CancellationTokenRegistration;

    // This is a non-reentrant lock wrapper around the ConcRT critical-section
    // and used by agents/messaging
    class _NonReentrantPPLLock
    {
    public:

        // Constructor for _NonReentrantPPLLock
        _CONCRTIMP _NonReentrantPPLLock();
        _NonReentrantPPLLock(const _NonReentrantPPLLock&) = delete;

        _NonReentrantPPLLock& operator=(const _NonReentrantPPLLock&) = delete;
        // Acquire the lock, spin if necessary
        _CONCRTIMP void _Acquire(void * _Lock_node);

        // Releases the lock
        _CONCRTIMP void _Release();

        // An exception safe RAII wrapper.
        class _Scoped_lock
        {
        public:
            // Constructs a holder and acquires the specified lock
            _CONCRTIMP explicit _Scoped_lock(_NonReentrantPPLLock& _Lock);

            _Scoped_lock(const _Scoped_lock&) = delete;
            _Scoped_lock& operator=(const _Scoped_lock&) = delete;

            // Destroys the holder and releases the lock
            _CONCRTIMP ~_Scoped_lock();

        private:
            _NonReentrantPPLLock& _M_lock;
            _CONCRT_BUFFER  _M_lockNode[(4 * sizeof(void *) + 2 * sizeof(unsigned int) + sizeof(_CONCRT_BUFFER) - 1) / sizeof(_CONCRT_BUFFER)];
        };

    private:
        // critical_section
        ::Concurrency::critical_section _M_criticalSection;
    };

    // This is a reentrant lock implemented using the ConcRT critical section
    class _ReentrantPPLLock
    {
    public:
        // Constructor for _ReentrantPPLLock
        _CONCRTIMP _ReentrantPPLLock();
        _ReentrantPPLLock(const _ReentrantPPLLock&) = delete;

        _ReentrantPPLLock& operator=(const _ReentrantPPLLock&) = delete;

        // Acquire the lock, spin if necessary
        _CONCRTIMP void _Acquire(void * _Lock_node);

        // Releases the lock
        _CONCRTIMP void _Release();

        // An exception safe RAII wrapper.
        class _Scoped_lock
        {
        public:
            // Constructs a holder and acquires the specified lock
            _CONCRTIMP explicit _Scoped_lock(_ReentrantPPLLock& _Lock);

            // Destroys the holder and releases the lock
            _CONCRTIMP ~_Scoped_lock();

        private:
            _ReentrantPPLLock& _M_lock;
            _CONCRT_BUFFER  _M_lockNode[(4 * sizeof(void *) + 2 * sizeof(unsigned int) + sizeof(_CONCRT_BUFFER) - 1) / sizeof(_CONCRT_BUFFER)];

            _Scoped_lock(const _Scoped_lock&);                    // no copy constructor
            _Scoped_lock const & operator=(const _Scoped_lock&);  // no assignment operator
        };

    private:
        // critical_section
        ::Concurrency::critical_section _M_criticalSection;

        // The number of times this lock has been taken recursively
        long _M_recursionCount;

        // The current owner of the lock
        volatile long _M_owner;
    };

    struct _Chore
    {
    protected:
        // Constructors.
        explicit _Chore(TaskProc _PFunction) : m_pFunction(_PFunction)
        {
        }

        _Chore()
        {
        }

        virtual ~_Chore()
        {
        }

    public:

        // The function which invokes the work of the chore.
        TaskProc m_pFunction{};
    };

    // _UnrealizedChore represents an unrealized chore -- a unit of work that scheduled in a work
    // stealing capacity. Some higher level construct (language or library) will map atop this to provide
    // a usable abstraction to clients.
    class _UnrealizedChore : public _Chore, public _AllocBase
    {
    public:
        // Constructor for an unrealized chore.
        _UnrealizedChore() :
            _M_pTaskCollection(nullptr)
        {
        }
        virtual ~_UnrealizedChore() {}


        // Method that executes the unrealized chore.
        void _Invoke()
        {
            _M_pChoreFunction(this);
        }

        // Sets the attachment state of the chore at the time of stealing.
        void _SetDetached(bool _FDetached);

        // Returns the owning collection of the chore.
        ::Concurrency::details::_TaskCollectionBase* _OwningCollection() const
        {
            return _M_pTaskCollection;
        }

        // Set flag that indicates whether the scheduler owns the lifetime of the object and is responsible for freeing it.
        // The flag is ignored by _StructuredTaskCollection
        void _SetRuntimeOwnsLifetime(bool _FValue)
        {
            _M_fRuntimeOwnsLifetime = _FValue;
        }

        // Returns the flag that indicates whether the scheduler owns the lifetime of the object and is responsible for freeing it.
        // The flag is ignored by _StructuredTaskCollection
        bool _GetRuntimeOwnsLifetime() const
        {
            return _M_fRuntimeOwnsLifetime;
        }

        // Allocator to be used when runtime owns lifetime.
        template <typename _ChoreType, typename _Function>
        static _ChoreType * _InternalAlloc(const _Function& _Func)
        {
            // This is always invoked from the PPL layer by the user and can never be attached to the default scheduler. Therefore '_concrt_new' is not required here
            _ChoreType * _Chore = new _ChoreType(_Func);
            _Chore->_M_fRuntimeOwnsLifetime = true;
            return _Chore;
        }

        // Internal helper routine to prepare for execution as a stolen chore.
        void _PrepareSteal(ContextBase *_PContext);

    protected:
        // Invocation bridge between the _UnrealizedChore and PPL.
        template <typename _ChoreType>
        static void __cdecl _InvokeBridge(void * _PContext)
        {
            auto _PChore = static_cast<_ChoreType *>(_PContext);
            (*_PChore)();
        }

        // Place associated task collection in a safe state.
        _CONCRTIMP void _CheckTaskCollection();

    private:

        friend class _StructuredTaskCollection;
        friend class _TaskCollection;
        typedef void (__cdecl * CHOREFUNC)(_UnrealizedChore * _PChore);

        // The collection of work to which this particular chore belongs.
        ::Concurrency::details::_TaskCollectionBase * _M_pTaskCollection;

        // Internal invocation inside the scheduler.
        CHOREFUNC _M_pChoreFunction{};

        // Indicates whether the scheduler owns the lifetime of the object and is responsible for freeing it.
        // This flag is ignored by _StructuredTaskCollection
        bool _M_fRuntimeOwnsLifetime{};

        // An indication of whether the chore (if stolen) was detached.
        bool _M_fDetached{};

        // Helper routines
        void _PrepareStealStructured(ContextBase *_PContext);
        void _PrepareStealUnstructured(ContextBase *_PContext);

        // The internal wrapper around invocation of stolen structured chores.
        __declspec(noinline)
        static void __cdecl _StructuredChoreWrapper(_UnrealizedChore * _PChore);

        // The internal wrapper around invocation of stolen unstructured chores.
        __declspec(noinline)
        static void __cdecl _UnstructuredChoreWrapper(_UnrealizedChore * _PChore);

        // To free memory allocated with _InternalAlloc.
        static void _InternalFree(_UnrealizedChore * _PChore);

        // Cancellation via token to a stolen chore
        static void __cdecl _CancelViaToken(::Concurrency::details::ContextBase *_PContext);
    };

    // Represents possible results of waiting on a task collection.
    enum _TaskCollectionStatus
    {
        _NotComplete,
        _Completed,
        _Canceled
    };

    // _TaskCollectionBase represents an abstract set of work and provides shared waiting semantics for stolen work.
    class _TaskCollectionBase
    {
    public:
        // Constructs a new task collection.
        _TaskCollectionBase() :
            _M_inliningDepth(_S_notInlined),
            _M_pTokenState(nullptr),
            _M_unpoppedChores(0),
            _M_completedStolenChores(_CollectionNotInitialized),
            _M_pException(nullptr)
        {
        }
        _TaskCollectionBase(const _TaskCollectionBase&) = delete;

        // Constructs a new task collection based on a given cancellation token.
        _TaskCollectionBase(_CancellationTokenState *_PTokenState) :
            _M_inliningDepth(_S_notInlined),
            _M_pTokenState(_PTokenState),
            _M_unpoppedChores(0),
            _M_completedStolenChores(_CollectionNotInitialized),
            _M_pException(nullptr)
        {
        }

        _TaskCollectionBase& operator=(const _TaskCollectionBase&) = delete;

        // Returns the owning context of the task collection.
        void * _OwningContext() const
        {
            return _M_pOwningContext;
        }

        // Returns the inlining depth.
        int _InliningDepth() const
        {
            return _M_inliningDepth;
        }

        // Tells if the task collection is inlined - some thread somewhere is currently invoking wait on it.
        bool _IsCurrentlyInlined() const
        {
            return (_M_inliningDepth != _S_notInlined);
        }

        // Returns whether this is a structured collection or not.
        bool _IsStructured()
        {
            return (_M_inlineFlags & _S_structured) != 0;
        }

        // Returns the token state associated with this task collection
        _CancellationTokenState *_GetTokenState(_CancellationTokenRegistration **_PRegistration = nullptr);

    protected:

        friend class ::Concurrency::details::_UnrealizedChore;
        friend class ::Concurrency::details::ContextBase;

        enum _TaskCollectionBaseState
        {
            _CollectionNotInitialized = LONG_MIN,
            _CollectionInitializationInProgress = LONG_MIN+1,
            _CollectionInitialized = 0
        };

        // Returns the exception portion of _M_pException.
        ::std::exception_ptr * _Exception() const
        {
            return (::std::exception_ptr *) ((size_t)_M_pException & ~_S_cancelBitsMask);
        }

        // Indicates whether or not this task collection has an abnormal exit.
        bool _IsAbnormalExit() const
        {
            return _M_pException != nullptr;
        }

        // Returns the cancel flags.
        size_t _CancelState() const
        {
            return (size_t) _M_pException & _S_cancelBitsMask;
        }

        // Returns whether or not the collection is marked for cancellation.
        bool _IsMarkedForCancellation() const
        {
            return (_CancelState() & _S_cancelBitsMask) != 0;
        }

        // Returns whether an inline cancellation was performed.
        bool _PerformedInlineCancel() const
        {
            _CONCRT_ASSERT(_CancelState() != _S_cancelStarted);
            return _CancelState() == _S_cancelShotdownOwner;
        }

        bool _PerformedPendingCancel() const
        {
            _CONCRT_ASSERT(_CancelState() != _S_cancelStarted);
            return _CancelState() == _S_cancelDeferredShootdownOwner;
        }

        // Returns the parent collection safely.
        _TaskCollectionBase *_SafeGetParent()
        {
            return ((_M_inliningDepth != _S_notInlined) ? _M_pParent : nullptr);
        }

        // Called in order to determine whether this task collection will interrupt for a pending cancellation at or above it.
        bool _WillInterruptForPendingCancel();

        // Called when an exception is raised on a chore on a given task collection, this makes a determination of what to do with the exception
        // and saves it for potential transport back to the thread performing a join on a chore collection.
        void _RaisedException();

        // Potentially rethrows the exception which was set with _RaisedException. The caller has responsibility to ensure that _RaisedException
        //  was called prior to calling this and that _M_pException has progressed beyond the _S_nonNull state.
        void _RethrowException();

        // Marks the collection for cancellation and returns whether the collection was marked.
        bool _MarkCancellation();

        // Finishes the cancellation state (changing from _S_cancelStarted to one of the other states). Note that only the
        // thread which successfully marked cancellation can call this.
        void _FinishCancelState(size_t _NewCancelState);

        // Called when a cancellation is raised on a chore on a given task collection. This makes a determination of what to do with the exception
        // and saves it for potential transport back to the thread performing a join on a chore collection. Note that every other exception
        // has precedence over a cancellation.
        void _RaisedCancel();

        // Tracks the parent collection. (For example, A task collection B created during execution of a chore C on task collection A is
        // considered a child of A).
        _TaskCollectionBase * _M_pParent{};

        // Tracks the inlining depth of this collection for cancellation purposes and packs a series of definition bits.
        int _M_inliningDepth : 28;
        int _M_inlineFlags : 4;

        // The cancellation token for the task collection.
        _CancellationTokenState *_M_pTokenState;

        // The context which owns the task collection. This is the context where the collection is created.
        void * _M_pOwningContext{};

        // The number of unpopped chores associated with the task collection (set by the derived
        // class during chore association.
        long _M_unpoppedChores;

        // The number of stolen chores executed so far.
        volatile long _M_completedStolenChores;

        // The stored exception which has been marshaled from the thread a stolen chore ran upon to the thread that is waiting on the
        // task collection.
        //
        // The lower two bits of _M_pException are utilized for the cancellation state machine. The upper 30 are the exception pointer. This implies
        // that the exception pointer must be 4-byte aligned. Because of intermediate states, the exception pointer cannot be between 0x8 and 0xF. The heap should
        // not be allocating such...
        //
        ::std::exception_ptr * _M_pException;

        // Cancellation states
        static constexpr size_t _S_cancelBitsMask = 0x3;
        static constexpr size_t _S_cancelNone = 0x0;
        static constexpr size_t _S_cancelStarted = 0x1;
        static constexpr size_t _S_cancelDeferredShootdownOwner = 0x2;
        static constexpr size_t _S_cancelShotdownOwner = 0x3;

        // Intermediate exceptions.
        static constexpr size_t _S_nonNull = 0x8;
        static constexpr size_t _S_cancelException = 0xC;

        // initialization state for inlining depth.
        static constexpr int _S_notInlined = -1;

        // Inline flags.
        static constexpr int _S_structured = 0x00000001;
        static constexpr int _S_localCancel = 0x00000002;
        static constexpr int _S_reserved = 0x0000000C;
    };

    /// <summary>
    ///     Structured task collections represent groups of work which follow a strictly LIFO ordered paradigm
    ///     queueing and waiting respectively. They can only be waited on once and can only be used from a single thread of execution.
    /// </summary>
    /**/
    class _StructuredTaskCollection : public _TaskCollectionBase
    {
    public:

        /// <summary>
        ///     Construct a new structured task collection.
        /// </summary>
        /**/
        _StructuredTaskCollection()
        {
            _Construct();
            _M_pTokenState = nullptr;
        }

        _StructuredTaskCollection(const _StructuredTaskCollection&) = delete;
        _StructuredTaskCollection& operator=(const _StructuredTaskCollection&) = delete;
        /// <summary>
        ///     Construct a new structured task collection whose cancellation is governed by the supplied cancellation token.
        /// </summary>
        /// <param name="_PTokenState">
        ///     When this cancellation token is canceled, the structured task group will be canceled.
        /// </param>
        /**/
        _CONCRTIMP _StructuredTaskCollection(_CancellationTokenState *_PTokenState);

        /// <summary>
        ///     Destruct a task collection and wait on all associated work to finish. Clients must call '_StructuredTaskCollection::_Wait'
        ///     or '_StructuredTaskCollection::_RunAndWait' prior to destructing the object. If there are chores remaining in the queues, an
        ///     exception (missing_wait) is thrown. If the destructor is running because of exception unwinding, it will abort any scheduled work.
        ///     If another exception occurs because work is aborted, the process will terminate (C++ semantics).
        /// </summary>
        /**/
        _CONCRTIMP ~_StructuredTaskCollection();

        /// <summary>
        ///     Schedules a chore that can potentially run in parallel. The chore is pushed onto the associated workstealing queue, and
        ///     will be executed in a LIFO order. Note that the specified chore can be scheduled only on a single task collection at a given time.
        ///     Any attempt to schedule the same chore multiple times on one or more task collection will result in an invalid_multiple_scheduling
        ///     exception. After the chore is guaranteed to have been executed (by calling the _Wait method), it can be rescheduled to an
        ///     arbitrary task collection.
        /// </summary>
        /// <param name="_PChore">
        ///     The new unrealized chore to schedule
        /// </param>
        /// <param name="_PLocation">
        ///     The location where the unrealized chore should execute. Specifying the value NULL here indicates that the unrealized chore does not
        ///     have specific placement.
        /// </param>
        /**/
        _CONCRTIMP void _Schedule(_UnrealizedChore * _PChore, location * _PLocation);

        /// <summary>
        ///     Schedules a chore that can potentially run in parallel. The chore is pushed onto the associated workstealing queue, and
        ///     will be executed in a LIFO order. Note that the specified chore can be scheduled only on a single task collection at a given time.
        ///     Any attempt to schedule the same chore multiple times on one or more task collection will result in an invalid_multiple_scheduling
        ///     exception. After the chore is guaranteed to have been executed (by calling the _Wait method), it can be rescheduled to an
        ///     arbitrary task collection.
        /// </summary>
        /// <param name="_PChore">
        ///     The new unrealized chore to schedule
        /// </param>
        /**/
        _CONCRTIMP void _Schedule(_UnrealizedChore * _PChore);

        /// <summary>
        ///     Cancels work on the task collection.
        /// </summary>
        /**/
        _CONCRTIMP void _Cancel();

        /// <summary>
        ///     Informs the caller whether or not the task collection is currently in the midst of cancellation. Note that this
        ///     does not necessarily indicate that Cancel was called on the collection (although such certainly qualifies this function
        ///     to return true). It can be the case that the task collection is executing inline and a task collection further up in the work
        ///     tree was canceled. In cases such as these where we can determine ahead of time that cancellation will flow through
        ///     this collection, true will be returned as well.
        /// </summary>
        /// <returns>
        ///     An indication of whether the task collection is in the midst of a cancellation (or is guaranteed to be shortly).
        /// </returns>
        /**/
        _CONCRTIMP bool _IsCanceling();

        /// <summary>
        ///     A cancellation friendly wrapper with which to execute _PChore and then
        ///     waits for all chores running in the _StructuredTaskCollection to finish (normally or abnormally). This method encapsulates
        ///     all the running tasks in an exception handling block, and will re-throw any exceptions that occur in any of it tasks
        ///     (if those exceptions occur on another thread, they are marshaled from that thread to the thread where the _StructuredTaskCollection
        ///     was created, and re-thrown). After this function returns, the _StructuredTaskCollection cannot be used for scheduling further work.
        /// </summary>
        /// <param name="_PChore">
        ///     An _UnrealizedChore which when non-null will be called to invoke the chore in a cancellation friendly manner.
        /// </param>
        /// <returns>
        ///     An indication of the status of the wait.
        /// </returns>
        /**/
        _CONCRTIMP _TaskCollectionStatus __stdcall _RunAndWait(_UnrealizedChore * _PChore = nullptr);

        /// <summary>
        ///     Waits for all chores running in the _StructuredTaskCollection to finish (normally or abnormally). This method encapsulates
        ///     all the running tasks in an exception handling block, and will re-throw any exceptions that occur in any of it tasks
        ///     (if those exceptions occur on another thread, they are marshaled from that thread to the thread where the _StructuredTaskCollection
        ///     was created, and re-thrown). After this function returns, the _StructuredTaskCollection cannot be used for scheduling further work.
        /// </summary>
        /// <returns>
        ///     An indication of the status of the wait.
        /// </returns>
        /**/
        _TaskCollectionStatus _Wait()
        {
            return _RunAndWait();
        }

        /// <summary>
        ///     Called to cancel any contexts which stole chores from the given collection.
        /// </summary>
        /**/
        void _CancelStolenContexts();

    private:

        friend class _UnrealizedChore;

        void _Construct()
        {
            _M_pOwningContext = nullptr;
            _M_inlineFlags = _S_structured;
        }

        /// <summary>
        ///     Internal routine to abort work on the task collection.
        /// </summary>
        /**/
        _CONCRTIMP void _Abort();

        /// <summary>
        ///     Internal routine to clean up after a cancellation token.
        /// </summary>
        _CONCRTIMP void _CleanupToken();

        /// <summary>
        ///     Performs task cleanup normally done at destruction time.
        /// </summary>
        /**/
        bool _TaskCleanup()
        {
            //
            // Users are required to call Wait() before letting the destructor run. Otherwise, throw. Note that before throwing,
            // we must actually wait on the tasks because they contain pointers into stack frames and unwinding without the wait is
            // instant stack corruption.
            //
            if (_M_unpoppedChores > 0)
            {
                _Abort();

                if (!__uncaught_exception())
                {
                    return false;
                }
            }

            return true;
        }

        /// <summary>
        ///     Internal initialization of the structured task collection
        /// </summary>
        /**/
        void _Initialize();

        /// <summary>
        ///     Waits on a specified number of stolen chores.
        /// </summary>
        /// <param name="_StolenChoreCount">
        ///     The number of stolen chores to wait on.
        /// </param>
        /**/
        void _WaitOnStolenChores(long _StolenChoreCount);

        /// <summary>
        ///     Indicates that a stolen chore has completed.
        /// </summary>
        /**/
        void _CountUp();

        /// <summary>
        ///     The callback which is made when a cancellation occurs via a token associated with a structured_task_group on the boundary
        ///     of two cancellation tokens.
        /// </summary>
        /**/
        static void __cdecl _CancelViaToken(_StructuredTaskCollection *_PCollection);

        //
        // _StructuredTaskCollection::_M_event is used to construct an structured event object only when it is needed to block. The structured event object
        //  has no state to cleanup, therefore no dtor code is required.
        //
        _CONCRT_BUFFER _M_event[(sizeof(void*) + sizeof(_CONCRT_BUFFER) - 1) / sizeof(_CONCRT_BUFFER)];
    };

    /// <summary>
    ///     Task collections represent groups of work which step outside the strict structuring of the
    ///     _StructuredTaskCollection definition. Any groups of work which do not follow LIFO ordering, are waited
    ///     on multiple times, or are passed between arbitrary threads require utilization of this definition
    ///     of a task collection. It has additional overhead over the _StructuredTaskCollection.
    /// </summary>
    /**/
    class _TaskCollection : public _TaskCollectionBase
    {
    public:

        /// <summary>
        ///     Constructs a new task collection.
        /// </summary>
        /**/
        _CONCRTIMP _TaskCollection();
        _TaskCollection(const _TaskCollection&) = delete;

        _TaskCollection& operator=(const _TaskCollection&) = delete;
        /// <summary>
        ///     Constructs a new task collection whose cancellation is governed by the specified cancellation token state.
        /// </summary>
        /// <param name="_PTokenState">
        ///     When this cancellation token is canceled, the task collection is canceled.
        /// </param>
        /**/
        _CONCRTIMP _TaskCollection(_CancellationTokenState *_PTokenState);

        /// <summary>
        ///     Destroys a task collection. Clients must call '_TaskCollection::_Wait' or '_TaskCollection::_RunAndWait' prior to destructing
        ///     the object. If there are chores remaining in the queues, an exception (missing_wait) is thrown. If the destructor
        ///     is running because of exception unwinding, it will abort any scheduled work. If another exception occurs because work
        ///     is aborted, the process will terminate (C++ semantics).
        /// </summary>
        /**/
        _CONCRTIMP ~_TaskCollection();

        /// <summary>
        ///     Schedules a chore that can potentially run in parallel. The chore is pushed onto the associated workstealing queue, and
        ///     will be executed in a LIFO order. The tasks scheduled into a _TaskCollection are scheduled into the current scheduler.
        ///     Note that the specified chore can be scheduled only on a single task collection at a given time. Any attempt to schedule the same
        ///     chore multiple times on one or more task collections will result in an invalid_multiple_scheduling exception. After the chore is
        ///     guaranteed to have been executed (by calling the Wait method), it can be rescheduled to an arbitrary task collection.
        /// </summary>
        /// <param name="_PChore">
        ///     The new unrealized chore to schedule
        /// </param>
        /// <param name="_PLocation">
        ///     The location where the unrealized chore should execute. Specifying the value NULL here indicates that the unrealized chore does not
        ///     have specific placement.
        /// </param>
        /**/
        _CONCRTIMP void _Schedule(_UnrealizedChore * _PChore, location * _PLocation);

        /// <summary>
        ///     Schedules a chore that can potentially run in parallel. The chore is pushed onto the associated workstealing queue, and
        ///     will be executed in a LIFO order. The tasks scheduled into a _TaskCollection are scheduled into the current scheduler.
        ///     Note that the specified chore can be scheduled only on a single task collection at a given time. Any attempt to schedule the same
        ///     chore multiple times on one or more task collections will result in an invalid_multiple_scheduling exception. After the chore is
        ///     guaranteed to have been executed (by calling the Wait method), it can be rescheduled to an arbitrary task collection.
        /// </summary>
        /// <param name="_PChore">
        ///     The new unrealized chore to schedule
        /// </param>
        /**/
        _CONCRTIMP void _Schedule(_UnrealizedChore * _PChore);

        /// <summary>
        ///     Cancels work on the task collection.
        /// </summary>
        /**/
        _CONCRTIMP void _Cancel();

        /// <summary>
        ///     Informs the caller whether or not the task collection is currently in the midst of a cancellation. Note that this
        ///     does not necessarily indicate that Cancel was called on the collection (although such certainly qualifies this function
        ///     to return true). It can be the case that the task collection is executing inline and a task collection further up in the work
        ///     tree was canceled. In cases such as these where we can determine ahead of time that cancellation will flow through
        ///     this collection, true will be returned as well.
        /// </summary>
        /// <returns>
        ///     An indication of whether the task collection is in the midst of a cancellation (or is guaranteed to be shortly).
        /// </returns>
        /**/
        _CONCRTIMP bool _IsCanceling();

        /// <summary>
        ///     A cancellation friendly wrapper with which to execute _PChore and then
        ///     waits for all chores running in the _TaskCollection to finish (normally or abnormally). This method encapsulates
        ///     all the running tasks in an exception handling block, and will re-throw any exceptions that occur in any of it tasks
        ///     (if those exceptions occur on another thread, they are marshaled from that thread to the thread where the _TaskCollection
        ///     was created, and re-thrown). After this function returns, the  _TaskCollection cannot be used for scheduling further work.
        /// </summary>
        /// <param name="_PChore">
        ///     An _UnrealizedChore which when non-null will be called to invoke the chore in a cancellation friendly manner.
        /// </param>
        /// <returns>
        ///     An indication of the status of the wait.
        /// </returns>
        /// </summary>
        /**/
        _CONCRTIMP _TaskCollectionStatus __stdcall _RunAndWait(_UnrealizedChore * _PChore = nullptr);

        /// <summary>
        ///     Waits for all chores running in the _TaskCollection to finish (normally or abnormally). This method encapsulates
        ///     all the running tasks in an exception handling block, and will re-throw any exceptions that occur in any of it tasks
        ///     (if those exceptions occur on another thread, they are marshaled from that thread to the thread where the _TaskCollection
        ///     was created, and re-thrown). After this function returns, the  _TaskCollection cannot be used for scheduling further work.
        /// </summary>
        /// <returns>
        ///     An indication of the status of the wait.
        /// </returns>
        /// </summary>
        /**/
        _TaskCollectionStatus _Wait()
        {
            return _RunAndWait();
        }

        /// <summary>
        ///     Returns whether this task collection is marked for abnormal exit.
        /// </summary>
        /**/
        bool _IsMarkedForAbnormalExit() const;

        /// <summary>
        ///     Returns the object which this is an alias for.
        /// </summary>
        /**/
        _TaskCollection * _OriginalCollection() const;

        /// <summary>
        ///     Returns whether the task collection is an alias.
        /// </summary>
        /**/
        bool _IsAlias() const;

        /// <summary>
        ///     Registers a notification handler for completion of chores
        /// </summary>
        /// <param name="_Func">
        ///     The callback function
        /// </param>
        /// <param name="_PCompletionContext">
        ///     The completion context for the callback function
        /// </param>
        /**/
        void _RegisterCompletionHandler(TaskProc _Func, void * _PCompletionContext);

    private:

        friend class _UnrealizedChore;
        friend class ::Concurrency::details::ContextBase;

        /// <summary>
        ///     Determines if the task collection is a stale alias (an object which was left over from a deferred delete
        ///     of a direct alias but which happens to match the hash key for a newly allocated task collection)
        /// </summary>
        /**/
        bool _IsStaleAlias() const;

        /// <summary>
        ///     Releases an alias -- this will free it if the release is the last man out.
        /// </summary>
        /**/
        void _ReleaseAlias();

        /// <summary>
        ///     Constructs an alias collection based on a specified origin collection
        /// </summary>
        /// <param name="_POriginCollection">
        ///     Specifies which collection the newly constructed one will alias
        /// </param>
        /// <param name="_FDirectAlias">
        ///     Specifies whether the newly constructed collection is a direct alias
        /// </param>
        /**/
        _TaskCollection(_TaskCollection * _POriginCollection, bool _FDirectAlias);

        /// <summary>
        ///     Returns the local alias of a task collection on the current context.
        /// </summary>
        /**/
        _TaskCollection * _Alias();

        /// <summary>
        ///     Internal routine to abort work on the task collection.
        /// </summary>
        /// <param name="_FLeaveCanceled">
        ///     An indication as to whether or not to leave the task collection canceled after the abort.
        /// </param>
        /**/
        void _Abort(bool _FLeaveCanceled = false);

        /// <summary>
        ///     Returns whether the task collection is an indirect alias.
        /// </summary>
        /**/
        bool _IsIndirectAlias() const;

        /// <summary>
        ///     Returns whether the task collection is a direct alias.
        /// </summary>
        /**/
        bool _IsDirectAlias() const;

        /// <summary>
        ///     Returns whether this task collection has a direct alias.
        /// </summary>
        /**/
        bool _HasDirectAlias() const;

        /// <summary>
        ///     Cancels work on the task collection. This is an internal version.
        /// </summary>
        /// <param name="_InsideException">
        ///     Indicates whether the cancellation is taking place because of
        ///     exception unwinding within the runtime
        /// </param>
        /// <param name="_PSnapPoint">
        ///     A snapshot of the direct alias list which is what the call will effect
        /// </param>
        /**/
        void _Cancel(bool _InsideException, _TaskCollection * _PSnapPoint);

        /// <summary>
        ///     Called for every new chore put into the task collection. Assures appropriate synchronization with waiters.
        /// </summary>
        /**/
        void _NotifyNewChore();

        /// <summary>
        ///     Called for every completed chore from the task collection. Assures appropriate synchronization with waiters.
        /// </summary>
        /// <param name="_PChore">
        ///     An _UnrealizedChore which will be freed if its lifetime is owned by the Runtime.
        /// </param>
        /**/
        void _NotifyCompletedChoreAndFree(_UnrealizedChore * _PChore = nullptr);

        /// <summary>
        ///     Waits on the given task collection and every alias.
        /// </summary>
        /// <param name="_PSnapPoint">
        ///     A snapshot of the direct alias list which is what the call will effect
        /// </param>
        /**/
        void _FullAliasWait(_TaskCollection * _PSnapPoint);

        /// <summary>
        ///     Resets the task collection for future usage.
        /// </summary>
        /// <param name="_PSnapPoint">
        ///     A snapshot of the direct alias list which is what the call will effect
        /// </param>
        /**/
        void _Reset(_TaskCollection * _PSnapPoint);

        /// <summary>
        ///     Called when an exception is raised on a chore on an unstructured task collection, this makes a determination of what to do with the exception
        ///     and saves it for potential transport back to the thread performing a join on a task collection. This specifically handles situations
        ///     on for unstructured task collections before calling _TaskCollectionBase::_RaisedException.
        /// </summary>
        /**/
        void _RaisedException();

        /// <summary>
        ///     Called when a cancellation is raised on a chore on a given task collection. This makes a determination of what to do with the exception
        ///     and saves it for potential transport back to the thread performing a join on a chore collection. Note that every other exception
        ///     has precedence over a cancellation.
        /// </summary>
        /**/
        void _RaisedCancel();

        /// <summary>
        ///     Called in order to set the cancellation status of the collection.
        /// </summary>
        /// <param name="_Status">
        ///     The cancellation status to set
        /// </param>
        /// <returns>
        ///     An indication of whether the set succeeded. The set will fail if the task collection already has a cancellation status.
        /// </returns>
        /**/
        bool _SetCancelState(long _Status);

        /// <summary>
        ///     Called to cancel a single alias of a task collection from an arbitrary thread.
        /// </summary>
        /// <param name="_InsideException">
        ///     Indicates whether the cancellation is taking place because of
        ///     exception unwinding within the runtime
        /// </param>
        /**/
        void _CancelFromArbitraryThread(bool _InsideException);

        /// <summary>
        ///     Cancels all direct aliases of the task collection.
        /// </summary>
        /// <param name="_InsideException">
        ///     Indicates whether the cancellation is taking place because of
        ///     exception unwinding within the runtime
        /// </param>
        /// <param name="_PSnapPoint">
        ///     A snapshot of the direct alias list which is what the call will effect
        /// </param>
        /**/
        void _CancelDirectAliases(bool _InsideException, _TaskCollection * _PSnapPoint);

        /// <summary>
        ///     Called to cancel any contexts which stole chores from the given collection. This is *PART* of a cancellation
        ///     scheme. The remainder must be handled by the derived class in particular. This should be called last.
        /// </summary>
        /// <param name="_InsideException">
        ///     Indicates whether the cancellation is taking place because of
        ///     exception unwinding within the runtime
        /// </param>
        /// <param name="_FInlineGated">
        ///     Indicates whether the inline context is safe and blocked from becoming inaccessible during
        ///     the duration of the call
        /// </param>
        /**/
        void _CancelStolenContexts(bool _InsideException, bool _FInlineGated);

        /// <summary>
        ///     Returns the steal tracking list.
        /// </summary>
        /**/
        void *_GetStealTrackingList() const;

        /// <summary>
        ///     Internal initialization of the task collection
        /// </summary>
        /**/
        void _Initialize();

        /// <summary>
        ///     Performs an abortive sweep of the WSQ for inline stack overflow.
        /// </summary>
        /// <param name="_PCtx">
        ///     The context to sweep
        /// </param>
        /**/
        void _AbortiveSweep(void *_PCtx);

        /// <summary>
        ///     A predicate function checking whether a given chore belongs to a given collection.
        /// </summary>
        /// <param name="_PChore">
        ///     The chore to check
        /// </param>
        /// <param name="_PData">
        ///     The data to check against
        /// </param>
        /// <returns>
        ///     Whether or not the chore belongs to the collection
        /// </returns>
        /**/
        static bool __cdecl _CollectionMatchPredicate(_UnrealizedChore *_PChore, void *_PData);

        /// <summary>
        ///     Called to sweep an aborted chore in the case of inline stack overflow.
        /// </summary>
        /// <param name="_PChore">
        ///     The chore to sweep
        /// </param>
        /// <param name="_PData">
        ///     The data that was passed to the sweep predicate
        /// </param>
        /// <returns>
        ///     An indication of whether the chore is now gone
        /// </returns>
        /**/
        static bool __cdecl _SweepAbortedChore(_UnrealizedChore *_PChore, void *_PData);

        /// <summary>
        ///     Performs task cleanup normally done at destruction time.
        /// </summary>
        /// <param name="_FExceptional">
        ///     An indication if the cleanup is exceptional and the collection should be left in a canceled state.
        /// </param>
        /**/
        bool _TaskCleanup(bool _FExceptional);

        /// <summary>
        ///     Called when the task collection is canceled via a cancellation token.
        /// </summary>
        /**/
        static void __cdecl _CancelViaToken(_TaskCollection *_PCollection);

        /// <summary>
        ///     Tracks contexts that have stolen chores from this collection. This is storage for an internal list and lock. Note that this list is only
        ///     used for detached schedule groups.
        /// </summary>
        /**/
        _CONCRT_BUFFER _M_stealTracker[_SAFERWLIST_SIZE];

        /// <summary>
        ///     A count of active stealers for *CANCELLATION PURPOSES ONLY*. This is non-interlocked and guarded by the same lock as the
        ///     stealers list on this task collection.
        /// </summary>
        /**/
        long _M_activeStealersForCancellation;

        /// <summary>
        ///     An indication of the exit code of the chore. Anything non-zero here indicates cancellation of one
        ///     form or another.
        /// </summary>
        /**/
        volatile long _M_exitCode;

        /// <summary>
        ///     The status of the task collection.
        /// </summary>
        /**/
        volatile long _M_executionStatus;

        /// <summary>
        ///     An event on which to wait for stolen chores to complete.
        /// </summary>
        /**/
        event _M_event;

        _TaskCollection * _M_pOriginalCollection;
        _TaskCollection * _M_pNextAlias;
        void * _M_pTaskExtension;

        int _M_taskCookies[2];

        volatile long _M_flags;
        volatile long _M_chaining;

        DWORD _M_boundQueueId;
        int _M_stackPos;

        TaskProc _M_completionHandler;
        void * _M_pCompletionContext;
    };

    /// <summary>
    ///     RAII wrapper used to maintain and limit ppltask maximum inline schedule depth.
    ///     This class will keep a reference to the depth slot on current context.
    /// </summary>
    class _StackGuard
    {
    public:
        _StackGuard() : _Depth(_GetCurrentInlineDepth())
        {
            // _Depth is the reference to the depth slot on context.
            ++_Depth;
        }
        _StackGuard(const _StackGuard&) = delete;

        _StackGuard& operator=(const _StackGuard&) = delete;

        ~_StackGuard()
        {
            // _Depth is the reference to the depth slot on context.
            --_Depth;
        }

        bool _ShouldInline(_TaskInliningMode _InliningMode) const
        {
            // As _TaskInliningMode is defined as inlining threshold, we can directly convert
            // it into size_t, and compare with current context inlining depth.
            return _Depth <= static_cast<size_t>(_InliningMode);
        }
    private:
        size_t & _Depth;

        /// <summary>
        ///     Return a reference to the ppltask inline schedule depth slot on current context
        ///     The inline depth will be set to 0 when the context is first initialized,
        ///     and the caller is responsible to maintain that depth.
        /// </summary>
        _CONCRTIMP static size_t & __cdecl _GetCurrentInlineDepth();
    };

    /// <summary>
    ///     Async Task collections is a thin wrapper over task collection to cater to the execution of asynchronous
    ///     chores (or tasks defined in ppltasks.h). Specifically, they manage their own lifetime by using reference
    ///     counts. Scheduling a chore acquires a reference and on completion of its execution the reference is released.
    /// </summary>
    class _AsyncTaskCollection : public _RefCounterBase
    {
    public:
        _AsyncTaskCollection(const _AsyncTaskCollection&) = delete;
        _AsyncTaskCollection& operator=(const _AsyncTaskCollection&) = delete;
        /// <summary>
        ///     Constructs a new task collection whose cancellation is governed by the specified cancellation token state.
        /// </summary>
        /// <param name="_PTokenState">
        ///     When this cancellation token is canceled, the task collection is canceled.
        /// </param>
        /// <returns>
        ///     Pointer to a new instance of _AsyncTaskCollection.
        /// </returns>
        _CONCRTIMP static _AsyncTaskCollection * __cdecl _NewCollection(_CancellationTokenState *_PTokenState);

        /// <summary>
        ///     Schedule a chore with automatic inlining. The chore is pushed onto the associated workstealing queue, and
        ///     will be executed in a LIFO order. The tasks scheduled into a _TaskCollection are scheduled into the current scheduler.
        ///     Note that the specified chore can be scheduled only on a single task collection at a given time. Any attempt to schedule the same
        ///     chore multiple times on one or more task collections will result in an invalid_multiple_scheduling exception. After the chore is
        ///     guaranteed to have been executed (by calling the Wait method), it can be rescheduled to an arbitrary task collection.
        ///     This schedule method will perform automatic inlining base on <paramref value="_InliningMode"/>.
        /// </summary>
        /// <param name="_PChore">
        ///     The new unrealized chore need to be scheduled. The chore will be deleted after scheduling.
        /// </param>
        /// <param name="_InliningMode">
        ///     The inlining scheduling policy for current chore.
        /// </param>
        /// <returns>
        ///     An indication of current chore status after scheduling.
        /// </returns>
        _TaskCollectionStatus _ScheduleWithAutoInline(_UnrealizedChore * _PChore, _TaskInliningMode _InliningMode)
        {
            _CONCRT_ASSERT(_PChore != nullptr);
            _Reference();

            if (_InliningMode == _NoInline)
            {
                _M_taskCollection._Schedule(_PChore);
                return _NotComplete;
            }
            else
            {
                _StackGuard _Guard;
                if (_Guard._ShouldInline(_InliningMode))
                {
                    return _M_taskCollection._RunAndWait(_PChore);
                }
                else
                {
                    _M_taskCollection._Schedule(_PChore);
                    return _NotComplete;
                }
            }
        }

        /// <summary>
        ///     Cancels work on the task collection.
        /// </summary>
        void _Cancel()
        {
            _M_taskCollection._Cancel();
        }

        /// <summary>
        ///     A cancellation friendly wrapper with which to execute _PChore and then
        ///     waits for all chores running in the _TaskCollection to finish (normally or abnormally). This method encapsulates
        ///     all the running tasks in an exception handling block, and will re-throw any exceptions that occur in any of it tasks
        ///     (if those exceptions occur on another thread, they are marshaled from that thread to the thread where the _TaskCollection
        ///     was created, and re-thrown). After this function returns, the  _TaskCollection cannot be used for scheduling further work.
        /// </summary>
        /// <param name="_PChore">
        ///     An _UnrealizedChore which when non-null will be called to invoke the chore in a cancellation friendly manner.
        /// </param>
        /// <returns>
        ///     An indication of the status of the wait.
        /// </returns>
        _TaskCollectionStatus _RunAndWait()
        {
            // Note that _Guard is NOT unused variable, the constructor and destructor will be called to maintain inline depth.
            _StackGuard _Guard;
            return _M_taskCollection._RunAndWait();
        }

    private:

        void _NotificationHandler();

        _CONCRTIMP virtual void _Destroy();

        // Private constructor
        _AsyncTaskCollection(_CancellationTokenState *_PTokenState);

        __declspec(noinline)
        static void __cdecl _CompletionHandler(void * _PCompletionContext);

    private:

        // Underlying task collection where the chore is scheduled to run
        _TaskCollection _M_taskCollection;
    };

    /// <summary>
    ///     Internal maintenance structure for beacons.
    /// </summary>
    struct _Beacon_reference
    {
        volatile long _M_signals;
    };

    /// <summary>
    ///     A cancellation beacon is a flag which can be polled in an inlinable fashion using the is_signaled method in lieu of polling on
    ///     the more expensive non inlinable is_current_task_group_canceling method.
    /// </summary>
    /// <remarks>
    ///     Cancellation beacons can be used only in the same way as structured_task_group and _StructuredTaskCollection. They are intended
    ///     as stack based objects utilized in strictly nested RAII fashion. A beacon can *NOT* be passed to another thread or allocated on the
    ///     heap.
    /// </remarks>
    class _Cancellation_beacon
    {
    public:

        _CONCRTIMP _Cancellation_beacon();

        _CONCRTIMP ~_Cancellation_beacon();

        bool _Is_signaled() const
        {
            return (_M_pRef->_M_signals != 0);
        }

        // This method should only be called when the beacon is signaled. It confirms whether a cancellation is indeed happening and that the beacon
        // was not flagged due to a false positive race. If the cancellation is not confirmed, the beacon is lowered.
        _CONCRTIMP bool _Confirm_cancel();

        void _Raise()
        {
            _InterlockedIncrement(&_M_pRef->_M_signals);
        }

        void _Lower()
        {
            _InterlockedDecrement(&_M_pRef->_M_signals);
        }

    private:

        _Beacon_reference *_M_pRef;

    };

    //
    // Internal stub class.
    //
    class _TimerStub;

    //
    // Internal wrapper around timers in order to allow timer messaging blocks to share implementation with internal ConcRT runtime
    // timers.
    //
    class _Timer
    {
    protected:
        // Constructs a new timer.
        //
        // _Ms: The duration and period of the timer in milliseconds.
        // _FRepeating: An indication of whether the timer is repeating (periodic) or not.
        _CONCRTIMP _Timer(unsigned int _Ms, bool _FRepeating);

        // Destroys the timer.
        _CONCRTIMP virtual ~_Timer();

        // Starts the timer.
        _CONCRTIMP void _Start();

        // Stops the timer.
        _CONCRTIMP void _Stop();

    private:
        friend class _TimerStub;

        // Called when the timer fires.
        virtual void _Fire() =0;

        // The actual timer
        HANDLE _M_hTimer;

        // The duration and period of the timer.
        unsigned int _M_ms;

        // Whether the timer is repeating (periodic by _M_ms)
        bool _M_fRepeating;
    };

    //
    // Internal runtime structure that holds the trace flags and level for ETW events
    // provided by the Concurrent runtime.
    //
    struct _CONCRT_TRACE_INFO
    {
        volatile unsigned long EnableFlags;    // Determines which class of events to log
        volatile unsigned char EnableLevel;    // Determines the severity of events to log

        void _EnableTrace(unsigned char _Level, unsigned long _Flags)
        {
            EnableFlags = _Flags;
            EnableLevel = _Level;
        }
#pragma warning ( push )
#pragma warning ( disable : 5393 )  // unreachable code
        void _DisableTrace()
        {
            EnableLevel = 0;
            EnableFlags = 0;
        }
#pragma warning ( pop )

        bool _IsEnabled(unsigned char _Level, unsigned long _Flags) const
        {
            return ((_Level <= EnableLevel) &&  ((EnableFlags & _Flags) == _Flags));
        }
    };

    /// <summary>
    ///     Retrieves a pointer to the internal trace flags and level information for
    ///     the Concurrency runtime ETW provider.
    /// </summary>
    /**/
    _CONCRTIMP const _CONCRT_TRACE_INFO * _GetConcRTTraceInfo();

    /// <summary>
    ///     Register ConcRT as an ETW Event Provider.
    /// </summary>
    /**/
    void _RegisterConcRTEventTracing();

    /// <summary>
    ///     Unregister ConcRT as an ETW Event Provider.
    /// </summary>
    /**/
    void _UnregisterConcRTEventTracing();

} // namespace details


/// <summary>
///     Enables tracing in the Concurrency Runtime. This function is deprecated because ETW tracing is now on by default.
/// </summary>
/// <returns>
///     If tracing was correctly initiated, <c>S_OK</c> is returned; otherwise, <c>E_NOT_STARTED</c> is returned.
/// </returns>
/**/
__declspec(deprecated("Concurrency::EnableTracing is a deprecated function.")) _CONCRTIMP HRESULT __cdecl EnableTracing();

/// <summary>
///     Disables tracing in the Concurrency Runtime. This function is deprecated because ETW tracing is unregistered by default.
/// </summary>
/// <returns>
///     If tracing was correctly disabled, <c>S_OK</c> is returned. If tracing was not previously initiated,
///     <c>E_NOT_STARTED</c> is returned
/// </returns>
/**/
__declspec(deprecated("Concurrency::DisableTracing is a deprecated function.")) _CONCRTIMP HRESULT __cdecl DisableTracing();

/// <summary>
///     The types of events that can be traced using the tracing functionality offered by the Concurrency Runtime.
/// </summary>
/**/
enum ConcRT_EventType
{
    /// <summary>
    ///     An event type used for miscellaneous events.
    /// </summary>
    /**/
    CONCRT_EVENT_GENERIC    = 0,
    /// <summary>
    ///     An event type that marks the beginning of a start/end event pair.
    /// </summary>
    /**/
    CONCRT_EVENT_START      = 1,
    /// <summary>
    ///     An event type that marks the beginning of a start/end event pair.
    /// </summary>
    /**/
    CONCRT_EVENT_END        = 2,
    /// <summary>
    ///     An event type that represents the act of a context blocking.
    /// </summary>
    /**/
    CONCRT_EVENT_BLOCK      = 3,
    /// <summary>
    ///     An event type that represents the act of unblocking a context.
    /// </summary>
    /**/
    CONCRT_EVENT_UNBLOCK    = 4,
    /// <summary>
    ///     An event type that represents the act of a context yielding.
    /// </summary>
    /**/
    CONCRT_EVENT_YIELD      = 5,
    /// <summary>
    ///     An event type that represents the act of a context becoming idle.
    /// </summary>
    /**/
    CONCRT_EVENT_IDLE       = 6,
    /// <summary>
    ///     An event type that represents the act of a attaching to a scheduler.
    /// </summary>
    /**/
    CONCRT_EVENT_ATTACH     = 7,
    /// <summary>
    ///     An event type that represents the act of a detaching from a scheduler.
    /// </summary>
    /**/
    CONCRT_EVENT_DETACH     = 8,
};

// Common trace header structure for all ConcRT diagnostic events
//      struct CONCRT_TRACE_EVENT_HEADER_COMMON
//      {
//          EVENT_TRACE_HEADER header;
//          DWORD VirtualProcessorID;
//          DWORD SchedulerID;
//          DWORD ContextID;
//          DWORD ScheduleGroupID;
//      };

/// <summary>
///     The ETW provider GUID for the Concurrency Runtime.
/// </summary>
/**/
_CONCRTIMP extern const GUID ConcRT_ProviderGuid;

//
// GUIDS for events
//

/// <summary>
///     A category GUID describing ETW events fired by the Concurrency Runtime that are not more specifically described by another category.
/// </summary>
/// <remarks>
///     This category of events is not currently fired by the Concurrency Runtime.
/// </remarks>
/**/
_CONCRTIMP extern const GUID ConcRTEventGuid;

/// <summary>
///     A category GUID describing ETW events fired by the Concurrency Runtime that are directly related to scheduler activity.
/// </summary>
/// <seealso cref="CurrentScheduler Class"/>
/// <seealso cref="Scheduler Class"/>
/**/
_CONCRTIMP extern const GUID SchedulerEventGuid;

/// <summary>
///     A category GUID describing ETW events fired by the Concurrency Runtime that are directly related to schedule groups.
/// </summary>
/// <remarks>
///     This category of events is not currently fired by the Concurrency Runtime.
/// </remarks>
/// <seealso cref="ScheduleGroup Class"/>
/**/
_CONCRTIMP extern const GUID ScheduleGroupEventGuid;

/// <summary>
///     A category GUID describing ETW events fired by the Concurrency Runtime that are directly related to contexts.
/// </summary>
/// <seealso cref="Context Class"/>
/**/
_CONCRTIMP extern const GUID ContextEventGuid;

/// <summary>
///     A category GUID describing ETW events fired by the Concurrency Runtime that are directly related to chores or tasks.
/// </summary>
/// <remarks>
///     This category of events is not currently fired by the Concurrency Runtime.
/// </remarks>
/// <seealso cref="task_group Class"/>
/// <seealso cref="structured_task_group Class"/>
/**/
_CONCRTIMP extern const GUID ChoreEventGuid;

/// <summary>
///     A category GUID describing ETW events fired by the Concurrency Runtime that are directly related to virtual processors.
/// </summary>
/**/
_CONCRTIMP extern const GUID VirtualProcessorEventGuid;

/// <summary>
///     A category GUID describing ETW events fired by the Concurrency Runtime that are directly related to locks.
/// </summary>
/// <remarks>
///     This category of events is not currently fired by the Concurrency Runtime.
/// </remarks>
/// <seealso cref="critical_section Class"/>
/// <seealso cref="reader_writer_lock Class"/>
/**/
_CONCRTIMP extern const GUID LockEventGuid;

/// <summary>
///     A category GUID describing ETW events fired by the Concurrency Runtime that are directly related to the resource manager.
/// </summary>
/// <remarks>
///     This category of events is not currently fired by the Concurrency Runtime.
/// </remarks>
/// <seealso cref="IResourceManager Structure"/>
/**/
_CONCRTIMP extern const GUID ResourceManagerEventGuid;

/// <summary>
///     A category GUID describing ETW events fired by the Concurrency Runtime that are directly related to usage of the <c>parallel_invoke</c>
///     function.
/// </summary>
/// <seealso cref="parallel_invoke Function"/>
/**/
_CONCRTIMP extern const GUID PPLParallelInvokeEventGuid;

/// <summary>
///     A category GUID describing ETW events fired by the Concurrency Runtime that are directly related to usage of the <c>parallel_for</c>
///     function.
/// </summary>
/// <seealso cref="parallel_for Function"/>
/**/
_CONCRTIMP extern const GUID PPLParallelForEventGuid;

/// <summary>
///     A category GUID describing ETW events fired by the Concurrency Runtime that are directly related to usage of the <c>parallel_for_each</c>
///     function.
/// </summary>
/// <seealso cref="parallel_for_each Function"/>
/**/
_CONCRTIMP extern const GUID PPLParallelForeachEventGuid;

/// <summary>
///     A category GUID ({B9B5B78C-0713-4898-A21A-C67949DCED07}) describing ETW events fired by the Agents library in the Concurrency Runtime.
/// </summary>
/**/
_CONCRTIMP extern const GUID AgentEventGuid;

// Trace an event signaling a parallel function
_CONCRTIMP void __cdecl _Trace_ppl_function(const GUID& _Guid, unsigned char _Level, ConcRT_EventType _Type);

/// <summary>
///     Trace flags for the event types
/// </summary>
/**/
enum Concrt_TraceFlags
{
    SchedulerEventFlag              = 0x1,
    ContextEventFlag                = 0x2,
    VirtualProcessorEventFlag       = 0x4,
    ResourceManagerEventFlag        = 0x8,
    PPLEventFlag                    = 0x10,
    AgentEventFlag                  = 0x20,

    AllEventsFlag                   = 0xFFFFFFFF
};

/// <summary>
///     The types of events that can be traced using the tracing functionality offered by the Agents Library
/// </summary>
/**/
enum Agents_EventType
{
    /// <summary>
    ///     An event type that represents the creation of an object
    /// </summary>
    /**/
    AGENTS_EVENT_CREATE   = 0,

    /// <summary>
    ///     An event type that represents the initiation of some processing
    /// </summary>
    /**/
    AGENTS_EVENT_START    = 1,

    /// <summary>
    ///     An event type that represents the conclusion of some processing
    /// </summary>
    /**/
    AGENTS_EVENT_END      = 2,

    /// <summary>
    ///     An event type that represents the deletion of an object
    /// </summary>
    /**/
    AGENTS_EVENT_DESTROY  = 3,

    /// <summary>
    ///     An event type that represents the scheduling of a process
    /// </summary>
    /**/
    AGENTS_EVENT_SCHEDULE = 4,

    /// <summary>
    ///     An event type that represents the linking of message blocks
    /// </summary>
    /**/
    AGENTS_EVENT_LINK     = 5,

    /// <summary>
    ///     An event type that represents the unlinking of message blocks
    /// </summary>
    /**/
    AGENTS_EVENT_UNLINK   = 6,

    /// <summary>
    ///     An event type that represents the name for an object
    /// </summary>
    /**/
    AGENTS_EVENT_NAME     = 7

};

//        // Common trace payload for agents
//
//        struct AGENTS_TRACE_PAYLOAD
//        {
//            // Identifier of the agent or message block that is emitting the event
//            __int64  AgentId1;
//            union
//            {
//                // The identifier of a target block for link/unlink event
//                __int64  AgentId2;
//
//                // Count of messages processed for the end event
//                long    Count;
//
//                // Name of this agent for the purposes of the ETW trace
//                wchar_t Name[32];
//            };
//        };

// Emit a trace event specific to the agents library of the given type and payload
_CONCRTIMP void __cdecl _Trace_agents(Agents_EventType _Type, __int64 _AgentId, ...);
}

#ifndef _NO_DEFAULT_CONCRT_LIB

#undef _DEBUG_AFFIX
#undef _IDL_AFFIX
#undef _IDL_DEFAULT
#undef _LIB_STEM

#ifdef _DEBUG
    #define _DEBUG_AFFIX "d"
    #define _IDL_DEFAULT 2
#else
    #define _DEBUG_AFFIX ""
    #define _IDL_DEFAULT 0
#endif /* _DEBUG */

#if defined(_DLL) && !defined(_STATIC_CPPLIB)
    #define _LIB_STEM "concrt"
#else /* defined(_DLL) && !defined(_STATIC_CPPLIB) */
    #define _LIB_STEM "libconcrt"
    #if _ITERATOR_DEBUG_LEVEL != _IDL_DEFAULT
        #define _IDL_AFFIX _CRT_STRINGIZE(_ITERATOR_DEBUG_LEVEL)
    #endif /* _ITERATOR_DEBUG_LEVEL != _IDL_DEFAULT */
#endif /* defined(_DLL) && !defined(_STATIC_CPPLIB) */

#ifndef _IDL_AFFIX
    #define _IDL_AFFIX ""
#endif /* _IDL_AFFIX */

#pragma comment(lib, _LIB_STEM _DEBUG_AFFIX _IDL_AFFIX)

#undef _DEBUG_AFFIX
#undef _IDL_AFFIX
#undef _IDL_DEFAULT
#undef _LIB_STEM

#endif /* _NO_DEFAULT_CONCRT_LIB */

namespace concurrency = ::Concurrency;

 #pragma pop_macro("_YieldProcessor")
 #pragma pop_macro("new")
 _STL_RESTORE_CLANG_WARNINGS
 #pragma warning(pop)
 #pragma pack(pop)
