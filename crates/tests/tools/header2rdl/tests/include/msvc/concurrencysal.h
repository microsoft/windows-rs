/***
*concurrencysal.h - markers for documenting the concurrent semantics of APIs
*
*       Copyright (c) Microsoft Corporation. All rights reserved.
*
*Purpose:
*       This file contains macros for Concurrency SAL annotations. Definitions
*       starting with _Internal are low level macros that are subject to change.
*       Users should not use those low level macros directly.
*       [ANSI]
*
*       [Public]
*
****/

#ifndef CONCURRENCYSAL_H
#define CONCURRENCYSAL_H

#pragma once

#ifdef  __cplusplus // [
extern "C" {
#endif  // ]

#if !defined(__midl) && defined(_PREFAST_) && !defined(_SDV_)

__ANNOTATION(SAL_guarded_by(__deferTypecheck void *));
__ANNOTATION(SAL_write_guarded_by(__deferTypecheck void *));
__ANNOTATION(SAL_requires_lock_held(__deferTypecheck void *));
__ANNOTATION(SAL_requires_exclusive_lock_held(__deferTypecheck void *));
__ANNOTATION(SAL_requires_shared_lock_held(__deferTypecheck void *));
__ANNOTATION(SAL_requires_lock_not_held(__deferTypecheck void *));
__ANNOTATION(SAL_requires_no_locks_held(void));
__ANNOTATION(SAL_set_lock_count_to_zero(__deferTypecheck void *));
__ANNOTATION(SAL_set_lock_count_to_one(__deferTypecheck void *));
__ANNOTATION(SAL_acquires_lock(__deferTypecheck void *));
__ANNOTATION(SAL_acquires_exclusive_lock(__deferTypecheck void *));
__ANNOTATION(SAL_acquires_shared_lock(__deferTypecheck void *));
__ANNOTATION(SAL_releases_lock(__deferTypecheck void *));
__ANNOTATION(SAL_releases_exclusive_lock(__deferTypecheck void *));
__ANNOTATION(SAL_releases_shared_lock(__deferTypecheck void *));
__ANNOTATION(SAL_ignore_lock_match(__deferTypecheck void *));
__ANNOTATION(SAL_has_lock_property(__AuToQuOtE __In_impl_ char *));
__ANNOTATION(SAL_has_lock_level(__AuToQuOtE __In_impl_ char *));
__ANNOTATION(SAL_lock_level_order(__deferTypecheck void *, __deferTypecheck void *));
__ANNOTATION(SAL_no_competing_thread(void));
__ANNOTATION(SAL_set_same_lock(__deferTypecheck void *, __deferTypecheck void *));

/*
 * pre-defined global system locks
 */
extern int _Global_interlock_;
extern int _Global_cancel_spin_lock_;
extern int _Global_critical_region_;

/*
 * Annotation identifiers
 */
#define _Internal_create_CSAL_identifier_(id) const char id[] = "";

_Internal_create_CSAL_identifier_(_Lock_kind_mutex_)
_Internal_create_CSAL_identifier_(_Lock_kind_event_)
_Internal_create_CSAL_identifier_(_Lock_kind_semaphore_)
_Internal_create_CSAL_identifier_(_Lock_kind_spin_lock_)
_Internal_create_CSAL_identifier_(_Lock_kind_critical_section_)

/*
 * data protection
 */
#define _Guarded_by_(lock) _SAL2_Source_(_Guarded_by_, (lock), _SA_annotes1(SAL_guarded_by,lock))
#define _Write_guarded_by_(lock) _SAL2_Source_(_Write_guarded_by_, (lock), _SA_annotes1(SAL_write_guarded_by,lock))
#define _Interlocked_ _Guarded_by_(_Global_interlock_)

/*
 * interlocked operand used in interlocked instructions
 */
#ifndef _Interlocked_operand_
#define _Interlocked_operand_ _SAL2_Source_(_Interlocked_operand_, (), _Pre_ _SA_annotes0(SAL_interlocked))
#endif

/*
 * caller/callee locking contracts
 */
#define _Requires_lock_held_(lock)  _SAL2_Source_(_Requires_lock_held_, (lock), _Pre_ _SA_annotes1(SAL_requires_lock_held,lock))
#define _Requires_exclusive_lock_held_(lock)  _SAL2_Source_(_Requires_exclusive_lock_held_, (lock), _Pre_ _SA_annotes1(SAL_requires_exclusive_lock_held,lock))
#define _Requires_shared_lock_held_(lock)  _SAL2_Source_(_Requires_shared_lock_held_, (lock), _Pre_ _SA_annotes1(SAL_requires_shared_lock_held,lock))

#define _Requires_lock_not_held_(lock)  _SAL2_Source_(_Requires_lock_not_held_, (lock), _Pre_ _SA_annotes1(SAL_requires_lock_not_held,lock))
#define _Requires_no_locks_held_  _SAL2_Source_(_Requires_no_locks_held_, (), _Pre_ _SA_annotes0(SAL_requires_no_locks_held))

/*
 * acquire/release locking side effects
 */
#define _Acquires_lock_(lock)  _SAL2_Source_(_Acquires_lock_, (lock), _Post_ _SA_annotes1(SAL_acquires_lock,lock))
#define _Acquires_exclusive_lock_(lock)  _SAL2_Source_(_Acquires_exclusive_lock_, (lock), _Post_ _SA_annotes1(SAL_acquires_exclusive_lock,lock))
#define _Acquires_shared_lock_(lock)  _SAL2_Source_(_Acquires_shared_lock_, (lock), _Post_ _SA_annotes1(SAL_acquires_shared_lock,lock))

#define _Releases_lock_(lock)  _SAL2_Source_(_Releases_lock_, (lock), _Post_ _SA_annotes1(SAL_releases_lock,lock))
#define _Releases_exclusive_lock_(lock)  _SAL2_Source_(_Releases_exclusive_lock_, (lock), _Post_ _SA_annotes1(SAL_releases_exclusive_lock,lock))
#define _Releases_shared_lock_(lock)  _SAL2_Source_(_Releases_shared_lock_, (lock), _Post_ _SA_annotes1(SAL_releases_shared_lock,lock))

/*
 * acquire/release locking side effects for non-reentrant locks
 */
#define _Acquires_nonreentrant_lock_(lock) \
     _SAL2_Source_(_Acquires_nonreentrant_lock_, (lock), \
    _Requires_lock_not_held_(lock) \
    _Acquires_lock_(lock))

#define _Releases_nonreentrant_lock_(lock) \
     _SAL2_Source_(_Releases_nonreentrant_lock_, (lock), \
    _Requires_lock_held_(lock) \
    _Releases_lock_(lock))

#define _Post_same_lock_(a,b)  _SAL2_Source_(_Post_same_lock_, (a,b), _Post_ _SA_annotes2(SAL_set_same_lock,a,b))

/*
 * lock level
 */
#define _Create_lock_level_(level) _Internal_create_CSAL_identifier_(level)

#define _Has_lock_level_(level)  _SAL2_Source_(_Has_lock_level_, (level), _SA_annotes1(SAL_has_lock_level,#level))

#define _Internal_lock_level_order_(a,b)  _SAL2_Source_(_Internal_lock_level_order_, (a,b), _SA_annotes2(SAL_lock_level_order,a,b))
#define _Csalcat1_(x,y) x##y
#define _Csalcat2_(x,y) _Csalcat1_(x,y)

#define _Lock_level_order_(a,b) \
    extern _Internal_lock_level_order_(a,b) void _Sal_order_##a##_##b(_In_z_ const char*a, _In_z_ const char*b); \
    static __inline void CSALCAT2(CSAL_LockOrder,__COUNTER__)(void){_Sal_order_##a##_##b(#a,#b);}

/*
 * threading context
 */
#define _No_competing_thread_  _SAL2_Source_(_No_competing_thread_, (), _Pre_ _SA_annotes0(SAL_no_competing_thread))

/*
 * refinement and suppression
 */
extern _Acquires_lock_(*plock) void _Internal_acquires_lock_(void* plock);
extern _Releases_lock_(*plock) void _Internal_releases_lock_(void* plock);

#define _Internal_set_lock_count_to_zero_(lock)  _SAL2_Source_(Internal_set_lock_count_to_zero_, (lock), _Post_ _SA_annotes1(SAL_set_lock_count_to_zero,lock))
#define _Internal_set_lock_count_to_one_(lock)  _SAL2_Source_(_Internal_set_lock_count_to_one_, (lock), _Post_ _SA_annotes1(SAL_set_lock_count_to_one,lock))

extern _Internal_set_lock_count_to_one_(*plock) void _Internal_lock_held_(void* plock);
extern _Internal_set_lock_count_to_zero_(*plock) void _Internal_lock_not_held_(void* plock);
extern _Post_same_lock_(*plock1, *plock2) void _Internal_same_lock_(void* plock1, void* plock2);

#define _Analysis_assume_lock_acquired_(lock)  _Internal_acquires_lock_((void*)(&(lock)))
#define _Analysis_assume_lock_released_(lock)  _Internal_releases_lock_((void*)(&(lock)))

#define _Analysis_assume_lock_held_(lock) _Internal_lock_held_((void*)(&(lock)))
#define _Analysis_assume_lock_not_held_(lock) _Internal_lock_not_held_((void*)(&(lock)))
#define _Analysis_assume_same_lock_(lock1, lock2) _Internal_same_lock_((void*)(&(lock1)), (void*)(&(lock2)))

/*
 * _Function_ignore_lock_checking_ may be deprecated in future versions of SAL
 */
#define _Function_ignore_lock_checking_(lock)  _SAL2_Source_(_Function_ignore_lock_checking_, (lock), _Pre_ _SA_annotes1(SAL_ignore_lock_match,lock))
extern _Function_ignore_lock_checking_(*plock) void _Internal_suppress_lock_checking_(void* plock);

/*
 * _Analysis_suppress_lock_checking_ may be deprecated in future versions of SAL
 */
#define _Analysis_suppress_lock_checking_(lock) _Internal_suppress_lock_checking_((void*)(&(lock)));

#define _Benign_race_begin_ __pragma(warning(push)) __pragma(warning(disable:26100 26101 26150 26130 26180 26131 26181 28112))
#define _Benign_race_end_ __pragma(warning(pop))

#define _No_competing_thread_begin_ __pragma(warning(push)) __pragma(warning(disable:26100 26101 26150 26101 26151 26110 26160 26130 26180 26131 26181 28112))
#define _No_competing_thread_end_ __pragma(warning(pop))

/*
 * lock kinds
 */
#define _Has_lock_kind_(kind)  _SAL2_Source_(_Has_lock_kind_, (kind), _SA_annotes1(SAL_has_lock_property,#kind))

/*
 * smart locks (RAII lock wrappers)
 */
#ifdef __cplusplus
} // extern "C"

extern "C++"
{
    template<class _SmartLockType> _Acquires_lock_(_smartLock) void _Internal_acquires_smart_lock_(const _SmartLockType& _smartLock);
    template<class _SmartLockType> _Releases_lock_(_smartLock) void _Internal_releases_smart_lock_(const _SmartLockType& _smartLock);
} // extern "C++"

#define _Analysis_assume_smart_lock_acquired_(lock) _Internal_acquires_smart_lock_((lock))
#define _Analysis_assume_smart_lock_released_(lock) _Internal_releases_smart_lock_((lock))

extern "C" {
#endif

/*
 * move semantics
 */
#define _Internal_swap_lock_ 0

#define _Detaches_lock_(detached, lock) _Post_same_lock_(_Internal_swap_lock_, (lock)) _Post_same_lock_((detached), _Internal_swap_lock_)
#define _Moves_lock_(target, source) _Post_same_lock_(_Internal_swap_lock_, (source)) _Post_same_lock_((target), _Internal_swap_lock_)
#define _Replaces_lock_(target, source) _Post_same_lock_(_Internal_swap_lock_, (target)) _Post_same_lock_((target), (source)) _Releases_lock_(_Internal_swap_lock_)
#define _Swaps_locks_(left, right) _Post_same_lock_(_Internal_swap_lock_, (left)) _Post_same_lock_((left), (right)) _Post_same_lock_((right), _Internal_swap_lock_)

/*
 * Old spelling
 * Note: the old version may be deprecated in the future!!!
 */
extern int __system_interlock;
extern int __system_cancel_spinlock;
extern int __system_critical_region;

#define __guarded_by(lock) _SAL1_1_Source_(__guarded_by, (lock), _SA_annotes1(SAL_guarded_by,lock))
#define __write_guarded_by(lock) _SAL1_1_Source_(__write_guarded_by, (lock), _SA_annotes1(SAL_write_guarded_by,lock))
#define __interlocked __guarded_by(_Global_interlock_)

/*
 * caller/callee locking contracts
 */
#define __requires_lock_held(lock) _SAL1_1_Source_(__requires_lock_held, (lock), __pre _SA_annotes1(SAL_requires_lock_held,lock))
#define __requires_exclusive_lock_held(lock) _SAL1_1_Source_(__requires_exclusive_lock_held, (lock), __pre _SA_annotes1(SAL_requires_exclusive_lock_held,lock))
#define __requires_shared_lock_held(lock) _SAL1_1_Source_(__requires_shared_lock_held, (lock), __pre _SA_annotes1(SAL_requires_shared_lock_held,lock))

#define __requires_lock_not_held(lock) _SAL1_1_Source_(__requires_lock_not_held, (lock), __pre _SA_annotes1(SAL_requires_lock_not_held,lock))
#define __requires_no_locks_held _SAL1_1_Source_(__requires_no_locks_held, (), __pre _SA_annotes0(SAL_requires_no_locks_held))

/*
 * acquire/release locking side effects
 */
#define __acquires_lock(lock) _SAL1_1_Source_(__acquires_lock, (lock), __post _SA_annotes1(SAL_acquires_lock,lock))
#define __acquires_exclusive_lock(lock) _SAL1_1_Source_(__acquires_exclusive_lock, (lock), __post _SA_annotes1(SAL_acquires_exclusive_lock,lock))
#define __acquires_shared_lock(lock) _SAL1_1_Source_(__acquires_shared_lock, (lock), __post _SA_annotes1(SAL_acquires_shared_lock,lock))

#define __releases_lock(lock) _SAL1_1_Source_(__releases_lock, (lock), __post _SA_annotes1(SAL_releases_lock,lock))
#define __releases_exclusive_lock(lock) _SAL1_1_Source_(__releases_exclusive_lock, (lock),__post _SA_annotes1(SAL_releases_exclusive_lock,lock))
#define __releases_shared_lock(lock) _SAL1_1_Source_(__releases_shared_lock, (lock), __post _SA_annotes1(SAL_releases_shared_lock,lock))

/*
 * lock properties
 * The following kind options are supported:
 * __has_lock_property(MUTEX)
 * __has_lock_property(EVENT)
 * __has_lock_property(SEMAPHORE)
 * __has_lock_property(OTHER_HANDLE)
 * __has_lock_property(REENTRANT)
 * __has_lock_property(NON_REENTRANT)
 */
#define __has_lock_property(kind) _SAL1_1_Source_(__has_lock_property, (kind), _SA_annotes1(SAL_has_lock_property,#kind))

/*
 * lock level
 */
#define __declare_lock_level(level) _Internal_create_CSAL_identifier_(level)
#define __has_lock_level(level) _SAL1_1_Source_(__has_lock_level, (level), _SA_annotes1(SAL_has_lock_level,#level))

#define __internal_lock_level_order(a,b) _SAL1_1_Source_(__internal_lock_level_order, (a,b), _SA_annotes2(SAL_lock_level_order,#a,#b))
#define CSALCAT1(x,y) x##y
#define CSALCAT2(x,y) CSALCAT1(x,y)

#define __lock_level_order(a,b) \
    extern __internal_lock_level_order(a,b) void __sal_order_##a##_##b(__in_z char*a, __in_z char*b); \
    static __inline void CSALCAT2(CSAL_LockOrder,__COUNTER__)(void){__sal_order_##a##_##b(#a,#b);}

/*
 * threading context
 */
#define __no_competing_thread _SAL1_1_Source_(__no_competing_thread, (), __pre _SA_annotes0(SAL_no_competing_thread))

/*
 * refinement and suppression
 */
extern __acquires_lock(*plock) void __internal_acquires_lock(void* plock);
extern __releases_lock(*plock) void __internal_releases_lock(void* plock);

#define __analysis_assume_lock_acquired(lock) __internal_acquires_lock((void*)(&(lock)))
#define __analysis_assume_lock_released(lock) __internal_releases_lock((void*)(&(lock)))

#define __function_ignore_lock_checking(lock) _SAL1_1_Source_(__function_ignore_lock_cleanup, (lock), __pre _SA_annotes1(SAL_ignore_lock_match,lock))
extern __function_ignore_lock_checking(*plock) void __internal_suppress_lock_checking(void* plock);

#define __analysis_suppress_lock_checking(lock) __internal_suppress_lock_checking((void*)(&(lock)));

#define BENIGN_RACE_BEGIN __pragma(warning(push)) __pragma(warning(disable:26100 26150 26130 26180 26131 26181))
#define BENIGN_RACE_END __pragma(warning(pop))

#define NO_COMPETING_THREAD_BEGIN __pragma(warning(push)) __pragma(warning(disable:26100 26150 26101 26151 26110 26160 26130 26180 26131 26181))
#define NO_COMPETING_THREAD_END __pragma(warning(pop))

#else

#ifndef _Interlocked_operand_
#define _Interlocked_operand_
#endif

#define _Guarded_by_(lock)
#define _Write_guarded_by_(lock)
#define _Interlocked_
#define _Requires_lock_held_(lock)
#define _Requires_exclusive_lock_held_(lock)
#define _Requires_shared_lock_held_(lock)
#define _Requires_lock_not_held_(lock)
#define _Requires_no_locks_held_
#define _Acquires_lock_(lock)
#define _Acquires_exclusive_lock_(lock)
#define _Acquires_shared_lock_(lock)
#define _Releases_lock_(lock)
#define _Releases_exclusive_lock_(lock)
#define _Releases_shared_lock_(lock)
#define _Acquires_nonreentrant_lock_(lock)
#define _Releases_nonreentrant_lock_(lock)

#define _Post_same_lock_(lock1,lock2)

#define _Internal_set_lock_count_(lock, count)

#define _Create_lock_level_(level)
#define _Has_lock_level_(level)
#define _Internal_lock_level_order_(a,b)
#define _Csalcat1_(x,y)
#define _Csalcat2_(x,y)
#define _Lock_level_order_(a,b)
#define _No_competing_thread_
#define _Analysis_assume_lock_acquired_(lock)
#define _Analysis_assume_lock_released_(lock)
#define _Analysis_assume_lock_held_(lock)
#define _Analysis_assume_lock_not_held_(lock)
#define _Analysis_assume_same_lock_(lock1, lock2)
#define _Function_ignore_lock_checking_(lock)
#define _Analysis_suppress_lock_checking_(lock)

#define _Benign_race_begin_ __pragma(warning(push))
#define _Benign_race_end_ __pragma(warning(pop))

#define _No_competing_thread_begin_ __pragma(warning(push))
#define _No_competing_thread_end_ __pragma(warning(pop))

#define _Has_lock_kind_(kind)

#ifdef __cplusplus
#define _Analysis_assume_smart_lock_acquired_(lock)
#define _Analysis_assume_smart_lock_released_(lock)
#endif

#define _Detaches_lock_(detached, lock)
#define _Moves_lock_(target, source)
#define _Replaces_lock_(target, source)
#define _Swaps_locks_(left, right)

/*
 * Old spelling: will be deprecated
 */
#define __guarded_by(lock)
#define __write_guarded_by(lock)
#define __interlocked
#define __requires_lock_held(lock)
#define __requires_exclusive_lock_held(lock)
#define __requires_shared_lock_held(lock)
#define __requires_lock_not_held(lock)
#define __requires_no_locks_held
#define __acquires_lock(lock)
#define __acquires_exclusive_lock(lock)
#define __acquires_shared_lock(lock)
#define __releases_lock(lock)
#define __releases_exclusive_lock(lock)
#define __releases_shared_lock(lock)

#define __has_lock_property(kind)
#define __declare_lock_level(level)
#define __has_lock_level(level)
#define __internal_lock_level_order(a,b)
#define CSALCAT1(x,y)
#define CSALCAT2(x,y)
#define __lock_level_order(a,b)
#define __no_competing_thread
#define __analysis_assume_lock_acquired(lock)
#define __analysis_assume_lock_released(lock)
#define __function_ignore_lock_checking(lock)
#define __analysis_suppress_lock_checking(lock)

#define BENIGN_RACE_BEGIN __pragma(warning(push))
#define BENIGN_RACE_END __pragma(warning(pop))

#define NO_COMPETING_THREAD_BEGIN __pragma(warning(push))
#define NO_COMPETING_THREAD_END __pragma(warning(pop))

#endif

#ifdef __cplusplus
}
#endif

#endif // CONCURRENCYSAL_H
