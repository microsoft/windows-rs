// Copyright (c) Microsoft Corporation. All rights reserved.

#pragma once
#define _THREADS_H

#include <vcruntime.h>
#include <corecrt.h>
#include <stdint.h>
#include <time.h>

#pragma warning(push)
#pragma warning(disable : _UCRT_DISABLED_WARNINGS)
_UCRT_DISABLE_CLANG_WARNINGS
_CRT_BEGIN_C_HEADER

#ifndef __cplusplus
#define thread_local _Thread_local
#endif

enum {
    mtx_plain     = 0,
    mtx_recursive = 1 << 0,
    mtx_timed     = 1 << 1,
};

typedef struct {
    uintptr_t _Type;
    void* _Ptr;
    void* _Cv;
    uint32_t _Owner;
    uint32_t _Cnt;
} mtx_t;

void __cdecl mtx_destroy(mtx_t* _Mtx);
int __cdecl mtx_init(_Out_ mtx_t* _Mtx, int _Type);
_Acquires_lock_(*_Mtx) int __cdecl mtx_lock(mtx_t* _Mtx);

int __cdecl _mtx_timedlock32(
    mtx_t* __restrict _Mtx, const struct _timespec32* __restrict _Ts);
int __cdecl _mtx_timedlock64(
    mtx_t* __restrict _Mtx, const struct _timespec64* __restrict _Ts);

#ifndef _CRT_NO_TIME_T
#ifdef _USE_32BIT_TIME_T
static inline int __cdecl mtx_timedlock(mtx_t* __restrict _Mtx, const struct timespec* __restrict _Ts) {
    return _mtx_timedlock32(_Mtx, (struct _timespec32*) _Ts);
}
#else // ^^^ _CRT_32BIT_TIME_T / vvv 64-bit
static inline int __cdecl mtx_timedlock(mtx_t* __restrict _Mtx, const struct timespec* __restrict _Ts) {
    return _mtx_timedlock64(_Mtx, (struct _timespec64*) _Ts);
}
#endif // _CRT_32BIT_TIME_T
#endif // _CRT_NO_TIME_T

int __cdecl mtx_trylock(mtx_t* _Mtx);

_Releases_lock_(*_Mtx) int __cdecl mtx_unlock(mtx_t* _Mtx);

typedef struct {
    void* _Ptr;
} cnd_t;

int __cdecl cnd_broadcast(cnd_t* _Cond);
void __cdecl cnd_destroy(cnd_t* _Cond);
int __cdecl cnd_init(_Out_ cnd_t* _Cond);
int __cdecl cnd_signal(cnd_t* _Cond);
int __cdecl _cnd_timedwait32(cnd_t* _Cond, mtx_t* _Mtx, const struct _timespec32* _Ts);
int __cdecl _cnd_timedwait64(cnd_t* _Cond, mtx_t* _Mtx, const struct _timespec64* _Ts);

#ifndef _CRT_NO_TIME_T
#ifdef _USE_32BIT_TIME_T
static inline int __cdecl cnd_timedwait(cnd_t* _Cond, mtx_t* _Mtx, const struct timespec* _Ts) {
    return _cnd_timedwait32(_Cond, _Mtx, (struct _timespec32*) _Ts);
}
#else
static inline int __cdecl cnd_timedwait(cnd_t* _Cond, mtx_t* _Mtx, const struct timespec* _Ts) {
    return _cnd_timedwait64(_Cond, _Mtx, (struct _timespec64*) _Ts);
}
#endif
#endif // _CRT_NO_TIME_T
int cnd_wait(cnd_t* _Cond, mtx_t* _Mtx);


typedef struct {
    void* _Handle;
    uint32_t _Tid;
} thrd_t;

enum { thrd_success, thrd_nomem, thrd_timedout, thrd_busy, thrd_error };

typedef int(__cdecl* thrd_start_t)(void*);

_Success_(return == thrd_success) int __cdecl thrd_create(_Out_ thrd_t* _Thr, thrd_start_t _Func, void* _Arg);
thrd_t __cdecl thrd_current(void);
int __cdecl thrd_detach(thrd_t _Thr);
int __cdecl thrd_equal(thrd_t _Thr0, thrd_t _Thr1);

#ifdef __cplusplus // TRANSITION, [[_Noreturn]]
[[noreturn]] void __cdecl thrd_exit(int _Res);
#else
_Noreturn void __cdecl thrd_exit(int _Res);
#endif

int __cdecl thrd_join(thrd_t _Thr, int* _Res);
int __cdecl _thrd_sleep32(
    const struct _timespec32* duration, struct _timespec32* remaining);
int __cdecl _thrd_sleep64(
    const struct _timespec64* duration, struct _timespec64* remaining);

#ifndef _CRT_NO_TIME_T
#ifdef _USE_32BIT_TIME_T
static inline int __cdecl thrd_sleep(const struct timespec* duration, struct timespec* remaining) {
    return _thrd_sleep32((struct _timespec32*) duration, (struct _timespec32*) remaining);
}
#else
static inline int __cdecl thrd_sleep(const struct timespec* duration, struct timespec* remaining) {
    return _thrd_sleep64((struct _timespec64*) duration, (struct _timespec64*) remaining);
}
#endif
#endif

void __cdecl thrd_yield(void);

#define TSS_DTOR_ITERATIONS 1
typedef struct {
    uint32_t _Idx;
} tss_t;

typedef void (*tss_dtor_t)(void*);

int __cdecl tss_create(tss_t* _Key, tss_dtor_t _Dtor);
void __cdecl tss_delete(tss_t _Key);
void* __cdecl tss_get(tss_t _Key);
int __cdecl tss_set(tss_t _Key, void* _Val);

typedef struct {
    void* _Opaque;
} once_flag;

void __cdecl call_once(once_flag* _Flag, void(*_Func)(void));

#define ONCE_FLAG_INIT { 0 }

_CRT_END_C_HEADER
_UCRT_RESTORE_CLANG_WARNINGS
#pragma warning(pop) // _UCRT_DISABLED_WARNINGS
