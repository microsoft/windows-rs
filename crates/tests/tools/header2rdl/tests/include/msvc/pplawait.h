/***
 * ==++==
 *
 * Copyright (c) Microsoft Corporation. All rights reserved.
 *
 * ==--==
 * =+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+
 *
 * pplawait.h
 *
 * Await Compiler Support for Parallel Patterns Library - PPL Tasks
 *
 * =-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-
 ****/

#pragma once

#ifndef _PPLAWAIT_H
#define _PPLAWAIT_H

#ifdef __cpp_impl_coroutine
#include <coroutine>
#else // ^^^ C++20 coroutines / TS coroutines vvv
#include <experimental/resumable>
#endif // ^^^ TS coroutines ^^^

#include <functional>
#include <ppltasks.h>
#include <ppltaskscheduler.h>
#include <type_traits>
#include <utility>

#define __resumable

_STD_BEGIN

#ifndef __cpp_impl_coroutine
namespace experimental {
#endif // __cpp_impl_coroutine
    template <typename _Ty, typename... _Whatever>
    struct coroutine_traits<::concurrency::task<_Ty>, _Whatever...> {
        struct promise_type {
            auto get_return_object() const {
                return ::concurrency::create_task(_M_tce);
            }

            suspend_never initial_suspend() const noexcept {
                return {};
            }

            suspend_never final_suspend() const noexcept {
                return {};
            }

            void return_value(const _Ty& _Val) {
                _M_tce.set(_Val);
            }

            void unhandled_exception() {
                _M_tce.set_exception(_STD current_exception());
            }

        private:
            ::concurrency::task_completion_event<_Ty> _M_tce;
        };
    };

    template <typename... _Whatever>
    struct coroutine_traits<::concurrency::task<void>, _Whatever...> {
        struct promise_type {
            auto get_return_object() const {
                return ::concurrency::create_task(_M_tce);
            }

            suspend_never initial_suspend() const noexcept {
                return {};
            }

            suspend_never final_suspend() const noexcept {
                return {};
            }

            void return_void() {
                _M_tce.set();
            }

            void unhandled_exception() {
                _M_tce.set_exception(_STD current_exception());
            }

        private:
            ::concurrency::task_completion_event<void> _M_tce;
        };
    };
#ifndef __cpp_impl_coroutine
} // namespace experimental
#endif // __cpp_impl_coroutine

_STD_END

#ifdef __cplusplus_winrt
namespace Windows {
    namespace Foundation {
        // When GetResults is called for an operation in the Canceled state, an E_ILLEGAL_METHOD_CALL exception will be
        // thrown. We preempt that by throwing an OperationCanceled exception, which better meets caller expectations.
        inline void _VerifyStateForResultsCall(Windows::Foundation::AsyncStatus _Status) {
            if (_Status == AsyncStatus::Canceled) {
                throw ::Platform::Exception::CreateException(E_ABORT);
            }
        }
    } // namespace Foundation
} // namespace Windows
#endif // __cplusplus_winrt

#ifndef _RESUMABLE_ENABLE_LEGACY_AWAIT_ADAPTERS
namespace Concurrency {
#if defined(__cpp_impl_coroutine) || defined(_RESUMABLE_FUNCTIONS_SUPPORTED)
    template <typename _Ty>
    struct _Ppltask_awaiter {
        task<_Ty>& _Task;

        bool await_ready() const {
            return _Task.is_done();
        }

        void await_suspend(_STD
#ifdef _RESUMABLE_FUNCTIONS_SUPPORTED
                experimental::
#endif
                    coroutine_handle<>
                        _ResumeCb) {
            _Task.then(
                [_ResumeCb](const task<_Ty>&) { _ResumeCb(); }, task_continuation_context::get_current_winrt_context());
        }

        decltype(auto) await_resume() {
            return _Task.get();
        }
    };

    template <typename _Ty>
    auto operator co_await(task<_Ty>&& _Task) {
        return _Ppltask_awaiter<_Ty>{_Task};
    }

    template <typename _Ty>
    auto operator co_await(task<_Ty>& _Task) {
        return _Ppltask_awaiter<_Ty>{_Task};
    }
#endif // defined(__cpp_impl_coroutine) || defined(_RESUMABLE_FUNCTIONS_SUPPORTED)

    struct [[deprecated("warning PPL4001: "
                        "The await_resume_context class is deprecated and will be removed in a future release. "
                        "You can define _RESUMABLE_ENABLE_LEGACY_AWAIT_ADAPTERS and add /await to the command line "
                        "to acknowledge that you have received this warning and get access to "
                        "await_resume_context.")]] await_resume_context{};

} // namespace Concurrency

// The Windows-internal CXAwait.h provides alternative bindings for IAsyncXXX and coroutines. These defines ensure the
// headers are included in the correct order, cxawait.h should be first.
#if defined(__cplusplus_winrt) && (defined(__cpp_impl_coroutine) || defined(_RESUMABLE_FUNCTIONS_SUPPORTED)) \
    && !defined(_COAWAIT_FROM_CXAWAIT)
#pragma detect_mismatch("_COAWAIT_FROM_CXAWAIT", "INCLUDE CXAWAIT FIRST")
namespace Windows {
    namespace Foundation {
        template <typename _TaskTy, typename _HandlerTy>
        struct _IAsync_awaiter {
            _TaskTy ^ _Task;

            bool await_ready() const {
                return _Task->Status >= AsyncStatus::Completed;
            }

            void await_suspend(_STD
#ifdef _RESUMABLE_FUNCTIONS_SUPPORTED
                    experimental::
#endif
                        coroutine_handle<>
                            _ResumeCb) {
                _Task->Completed = ref new _HandlerTy(
                    [_ResumeCb](_TaskTy ^, AsyncStatus) { _ResumeCb(); }, ::Platform::CallbackContext::Same);
            }

            auto await_resume() {
                _VerifyStateForResultsCall(_Task->Status);
                return _Task->GetResults();
            }
        };

        inline auto operator co_await(IAsyncAction ^ _Expr) {
            return _IAsync_awaiter<IAsyncAction, AsyncActionCompletedHandler>{_Expr};
        }

        template <typename _Pr>
        auto operator co_await(IAsyncActionWithProgress<_Pr> ^ _Expr) {
            return _IAsync_awaiter<IAsyncActionWithProgress<_Pr>, AsyncActionWithProgressCompletedHandler<_Pr>>{_Expr};
        }

        template <typename _Ty>
        auto operator co_await(IAsyncOperation<_Ty> ^ _Expr) {
            return _IAsync_awaiter<IAsyncOperation<_Ty>, AsyncOperationCompletedHandler<_Ty>>{_Expr};
        }

        template <typename _Ty, typename _Pr>
        auto operator co_await(IAsyncOperationWithProgress<_Ty, _Pr> ^ _Expr) {
            return _IAsync_awaiter<IAsyncOperationWithProgress<_Ty, _Pr>,
                AsyncOperationWithProgressCompletedHandler<_Ty, _Pr>>{_Expr};
        }
    } // namespace Foundation
} // namespace Windows
#endif // defined(__cplusplus_winrt) && (defined(__cpp_impl_coroutine) || defined(_RESUMABLE_FUNCTIONS_SUPPORTED))

#else // _RESUMABLE_ENABLE_LEGACY_AWAIT_ADAPTERS

_STD_BEGIN
namespace experimental {
    namespace details {
        namespace awaitabletraitsimpl {
            template <typename _Ty>
            inline bool _TryAwaitReady(_Ty& _Obj, decltype(_STD declval<_Ty>().await_ready(), 0), int) {
                return _Obj.await_ready();
            }
            template <typename _Ty>
            inline bool _TryAwaitReady(_Ty& _Obj, decltype(await_ready(_STD declval<_Ty>()), 0), ...) {
                return await_ready(_Obj);
            }

            template <typename _Ty, typename _Handle>
            inline void _TryAwaitSuspend(
                _Ty& _Obj, _Handle _Hnd, decltype(_STD declval<_Ty>().await_suspend(_STD declval<_Handle>()), 0), int) {
                _Obj.await_suspend(_Hnd);
            }
            template <typename _Ty, typename _Handle>
            inline void _TryAwaitSuspend(_Ty& _Obj, _Handle _Hnd,
                decltype(await_suspend(_STD declval<_Ty>(), _STD declval<_Handle>()), 0), ...) {
                await_suspend(_Obj, _Hnd);
            }

            template <typename _Ty>
            inline auto _TryAwaitResume(_Ty& _Obj, decltype(_STD declval<_Ty>().await_resume(), 0), int) {
                return _Obj.await_resume();
            }
            template <typename _Ty>
            inline auto _TryAwaitResume(_Ty& _Obj, decltype(await_resume(_STD declval<_Ty>()), 0), ...) {
                return await_resume(_Obj);
            }

            template <typename _Ty>
            inline true_type _TryAwaitable(_Ty, decltype(_TryAwaitReady(_STD declval<decay_t<_Ty>>(), 0, 0),
                                                    _TryAwaitSuspend(_STD declval<decay_t<_Ty>>(),
                                                        _STD
#ifdef _RESUMABLE_FUNCTIONS_SUPPORTED
                                                            experimental::
#endif
                                                                coroutine_handle<>(),
                                                        0, 0),
                                                    _TryAwaitResume(_STD declval<decay_t<_Ty>>(), 0, 0), 0));
            inline false_type _TryAwaitable(...);
        } // namespace awaitabletraitsimpl
    } // namespace details

    // Traits for
    //   1. detecting whether a type satisfies awaitable contract requirement
    //   2. invoking its awaitable contract methods

    template <typename _Ty>
    struct awaitable_traits : decltype(details::awaitabletraitsimpl::_TryAwaitable(_STD declval<_Ty>(), 0)) {
        static bool invoke_await_ready(_Ty& _Obj) {
            return details::awaitabletraitsimpl::_TryAwaitReady(_Obj, 0, 0);
        }

        template <typename _Handle>
        static void invoke_await_suspend(_Ty& _Obj, _Handle _Hnd) {
            details::awaitabletraitsimpl::_TryAwaitSuspend(_Obj, _Hnd, 0, 0);
        }

        static auto invoke_await_resume(_Ty& _Obj) {
            return details::awaitabletraitsimpl::_TryAwaitResume(_Obj, 0, 0);
        }
    };

    template <typename _Ty>
    using is_awaitable = awaitable_traits<_Ty>;

    template <typename _Ty>
    using is_awaitable_t = typename awaitable_traits<_Ty>::type;

} // namespace experimental

_STD_END

namespace Concurrency {
    // PPLTask awaitable extension

    template <typename _Ty>
    bool await_ready(const task<_Ty>& _Task) {
        return _Task.is_done();
    }

    template <typename _Ty, typename _Handle>
    void await_suspend(task<_Ty>& _Task, _Handle _ResumeCb) {
        _Task.then(
            [_ResumeCb](const task<_Ty>&) { _ResumeCb(); }, task_continuation_context::get_current_winrt_context());
    }

    template <typename _Ty>
    auto await_resume(const task<_Ty>& _Task) {
        return _Task.get();
    }


    class await_resume_context {
        class ThreadpoolContext {
            details::_Threadpool_chore _M_chore;
            static void __cdecl _ChoreCallback(void* _TpTask) {
                auto _Context = static_cast<ThreadpoolContext*>(_TpTask);
                _Context->_M_func();
            }
            _STD function<void()> _M_func;

        public:
            template <typename _Handle>
            void _CallInContext(_Handle _Hnd) {
                _M_func              = _Hnd;
                _M_chore._M_callback = &ThreadpoolContext::_ChoreCallback;
                _M_chore._M_data     = this;
                details::_Schedule_chore(&_M_chore);
            }

            ThreadpoolContext() {
                _M_chore._M_work = nullptr;
            }

            ~ThreadpoolContext() {
                // Releasing chore multiple times is fine
                details::_Release_chore(&_M_chore);
            }
        };

        details::_ContextCallback _M_context;
        ThreadpoolContext _M_defaultContext;

    public:
        static await_resume_context current() {
            await_resume_context _Context;
            _Context._M_context = details::_ContextCallback::_CaptureCurrent();
            return _Context;
        }

        static await_resume_context any() {
            return await_resume_context{};
        }

        template <typename _Ty>
        class _CallbackChain {
            _Ty _M_awaitable;
            await_resume_context* _M_contextPtr;

        public:
            _CallbackChain(const _Ty& _Awaitable, await_resume_context* _ContextPtr)
                : _M_awaitable(_Awaitable), _M_contextPtr(_ContextPtr) {}

            bool await_ready() {
                return false;
            }

            template <typename _Handle>
            void await_suspend(_Handle _Hnd) {
                _STD experimental::awaitable_traits<_Ty>::invoke_await_suspend(_M_awaitable, [=] {
                    if (_M_contextPtr->_M_context == details::_ContextCallback()) {
                        _M_contextPtr->_M_defaultContext._CallInContext([=] { _Hnd(); });
                    } else {
                        _M_contextPtr->_M_context._CallInContext([=] { _Hnd(); });
                    }
                });
            }

            auto await_resume() {
                return _STD experimental::awaitable_traits<_Ty>::invoke_await_resume(_M_awaitable);
            }
        };

        class _Callback {
            await_resume_context* _M_contextPtr;

        public:
            explicit _Callback(await_resume_context* _ContextPtr) : _M_contextPtr(_ContextPtr) {}

            bool await_ready() {
                return false;
            }

            template <typename _Handle>
            void await_suspend(_Handle _Hnd) {
                if (_M_contextPtr->_M_context == details::_ContextCallback()) {
                    _M_contextPtr->_M_defaultContext._CallInContext([=] { _Hnd(); });
                } else {
                    _M_contextPtr->_M_context._CallInContext([=] { _Hnd(); });
                }
            }

            void await_resume() {}
        };

        template <typename _Ty>
        auto get_awaitable(_Ty _Awaitable) {
            return _CallbackChain<_STD decay_t<_Ty>>(_Awaitable, this);
        }

        auto get_awaitable() {
            return _Callback(this);
        }
    };
} // namespace Concurrency

#ifdef __cplusplus_winrt

// WinRT IAsync(Operation|Action) awaitable extension

namespace Windows {
    namespace Foundation {
        inline bool await_ready(IAsyncInfo ^ _Task) {
            return _Task->Status >= AsyncStatus::Completed;
        }

        template <typename _Handle>
        void await_suspend(IAsyncAction ^ _Task, _Handle _ResumeCb) {
            _Task->Completed = ref new AsyncActionCompletedHandler(
                [_ResumeCb](IAsyncAction ^, AsyncStatus) { _ResumeCb(); }, ::Platform::CallbackContext::Same);
        }

        template <typename _Ty, typename _Handle>
        void await_suspend(IAsyncOperation<_Ty> ^ _Task, _Handle _ResumeCb) {
            _Task->Completed = ref new AsyncOperationCompletedHandler<_Ty>(
                [_ResumeCb](IAsyncOperation<_Ty> ^, AsyncStatus) { _ResumeCb(); }, ::Platform::CallbackContext::Same);
        }

        template <typename _Pr, typename _Handle>
        void await_suspend(IAsyncActionWithProgress<_Pr> ^ _Task, _Handle _ResumeCb) {
            _Task->Completed = ref new AsyncActionWithProgressCompletedHandler<_Pr>(
                [_ResumeCb](IAsyncActionWithProgress<_Pr> ^, AsyncStatus) { _ResumeCb(); },
                ::Platform::CallbackContext::Same);
        }

        template <typename _Ty, typename _Pr, typename _Handle>
        void await_suspend(IAsyncOperationWithProgress<_Ty, _Pr> ^ _Task, _Handle _ResumeCb) {
            _Task->Completed = ref new AsyncOperationWithProgressCompletedHandler<_Ty, _Pr>(
                [_ResumeCb](IAsyncOperationWithProgress<_Ty, _Pr> ^, AsyncStatus) { _ResumeCb(); },
                ::Platform::CallbackContext::Same);
        }

        inline void await_resume(Windows::Foundation::IAsyncAction ^ _Task) {
            _VerifyStateForResultsCall(_Task->Status);
            _Task->GetResults();
        }

        template <typename _Ty>
        _Ty await_resume(Windows::Foundation::IAsyncOperation<_Ty> ^ _Task) {
            _VerifyStateForResultsCall(_Task->Status);
            return _Task->GetResults();
        }

        template <typename _Pr>
        void await_resume(Windows::Foundation::IAsyncActionWithProgress<_Pr> ^ _Task) {
            _VerifyStateForResultsCall(_Task->Status);
            _Task->GetResults();
        }

        template <typename _Ty, typename _Pr>
        _Ty await_resume(Windows::Foundation::IAsyncOperationWithProgress<_Ty, _Pr> ^ _Task) {
            _VerifyStateForResultsCall(_Task->Status);
            return _Task->GetResults();
        }
    } // namespace Foundation
} // namespace Windows

#endif // __cplusplus_winrt
#endif // _RESUMABLE_ENABLE_LEGACY_AWAIT_ADAPTERS

#endif // _PPLAWAIT_H
