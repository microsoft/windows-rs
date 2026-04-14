/***
* ==++==
*
* Copyright (c) Microsoft Corporation. All rights reserved.
* Licensed under the Apache License, Version 2.0 (the "License");
* you may not use this file except in compliance with the License.
* You may obtain a copy of the License at
* https://www.apache.org/licenses/LICENSE-2.0
*
* Unless required by applicable law or agreed to in writing, software
* distributed under the License is distributed on an "AS IS" BASIS,
* WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
* See the License for the specific language governing permissions and
* limitations under the License.
*
* ==--==
* =+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+
*
* pplcancellation_token.h
*
* Parallel Patterns Library : cancellation_token
*
* =-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-
****/

#ifndef _PPLCANCELLATION_TOKEN_H
#define _PPLCANCELLATION_TOKEN_H

#ifndef _PPLWIN_H
#error This header must not be included directly
#endif

#include <yvals_core.h>

#if _STL_COMPILER_PREPROCESSOR

#include <pplinterface.h>
#include <mutex>
#include <condition_variable>

#pragma pack(push, _CRT_PACKING)
#pragma warning(push, _STL_WARNING_LEVEL)
#pragma warning(disable : _STL_DISABLED_WARNINGS)
_STL_DISABLE_CLANG_WARNINGS
#pragma push_macro("new")
#undef new

extern "C++" { // attach declarations in namespace Concurrency to the global module, see N4910 [module.unit]/7

namespace Concurrency
{

namespace details
{

    // Base class for all reference counted objects
    class _RefCounter
    {
    public:

        virtual ~_RefCounter()
        {
            _ASSERTE(_M_refCount == 0);
        }

        // Acquires a reference
        // Returns the new reference count.
        long _Reference()
        {
            const long _Refcount = _InterlockedIncrement(&_M_refCount);

            // 0 - 1 transition is illegal
            _ASSERTE(_Refcount > 1);
            return _Refcount;
        }

        // Releases the reference
        // Returns the new reference count
        long _Release()
        {
            const long _Refcount = _InterlockedDecrement(&_M_refCount);
            _ASSERTE(_Refcount >= 0);

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
        _RefCounter(long _InitialCount = 1) : _M_refCount(_InitialCount)
        {
            _ASSERTE(_M_refCount > 0);
        }

        // Reference count
        volatile long _M_refCount;
    };

    class _CancellationTokenState;

    class _CancellationTokenRegistration : public _RefCounter
    {
    private:

        static constexpr long _STATE_CLEAR = 0;
        static constexpr long _STATE_DEFER_DELETE = 1;
        static constexpr long _STATE_SYNCHRONIZE = 2;
        static constexpr long _STATE_CALLED = 3;

    public:

        _CancellationTokenRegistration(long _InitialRefs = 1) :
            _RefCounter(_InitialRefs),
            _M_state(_STATE_CALLED),
            _M_signaled(false),
            _M_pTokenState(nullptr)
        {
        }

        _CancellationTokenState *_GetToken() const
        {
            return _M_pTokenState;
        }

    protected:

        virtual ~_CancellationTokenRegistration()
        {
            _ASSERTE(_M_state != _STATE_CLEAR);
        }

        virtual void _Exec() = 0;

    private:

        friend class _CancellationTokenState;

        void _Invoke()
        {
            const long _Tid = ::Concurrency::details::platform::GetCurrentThreadId();
            _ASSERTE((_Tid & 0x3) == 0); // If this ever fires, we need a different encoding for this.

            long _Result = atomic_compare_exchange(_M_state, _Tid, _STATE_CLEAR);

            if (_Result == _STATE_CLEAR)
            {
                _Exec();

                _Result = atomic_compare_exchange(_M_state, _STATE_CALLED, _Tid);

                if (_Result == _STATE_SYNCHRONIZE)
                {
                    {
                        ::std::lock_guard<::std::mutex> _Lock(_M_Mutex);
                        _M_signaled = true;
                    }
                    _M_CondVar.notify_all();
                }
            }
            _Release();
        }

        atomic_long               _M_state;
        ::std::condition_variable _M_CondVar;
        ::std::mutex              _M_Mutex;
        bool                      _M_signaled;
        _CancellationTokenState * _M_pTokenState;
    };

    template<typename _Function>
    class _CancellationTokenCallback : public _CancellationTokenRegistration
    {
    public:

        _CancellationTokenCallback(const _Function& _Func) :
            _M_function(_Func)
        {
        }

    protected:

        virtual void _Exec() override
        {
            _M_function();
        }

    private:

        _Function _M_function;
    };

    class CancellationTokenRegistration_TaskProc : public _CancellationTokenRegistration
    {
    public:

        CancellationTokenRegistration_TaskProc(TaskProc_t _Proc, _In_ void *_PData, int _InitialRefs) :
            _CancellationTokenRegistration(_InitialRefs), _M_proc(_Proc), _M_pData(_PData)
        {
        }

    protected:

        virtual void _Exec() override
        {
            _M_proc(_M_pData);
        }

    private:

        TaskProc_t _M_proc;
        void *_M_pData;

    };

    // The base implementation of a cancellation token.
    class _CancellationTokenState : public _RefCounter
    {
    protected:
        class TokenRegistrationContainer
        {
        private:
            typedef struct _Node {
                _CancellationTokenRegistration* _M_token;
                _Node *_M_next;

                _Node(_CancellationTokenRegistration* _Token) : _M_token(_Token), _M_next(nullptr)
                {
                }
            } Node;

        public:
            TokenRegistrationContainer() : _M_begin(nullptr), _M_last(nullptr)
            {
            }

            ~TokenRegistrationContainer()
            {
                auto _Node = _M_begin;
                while (_Node != nullptr)
                {
                    Node* _Tmp = _Node;
                    _Node = _Node->_M_next;
                    delete _Tmp;
                }
            }

            void swap(TokenRegistrationContainer& _List)
            {
                ::std::swap(_List._M_begin, _M_begin);
                ::std::swap(_List._M_last, _M_last);
            }

            bool empty()
            {
                return _M_begin == nullptr;
            }

            template<typename _Ty>
            void for_each(_Ty _Lambda)
            {
                Node* _Node = _M_begin;

                while (_Node != nullptr)
                {
                    _Lambda(_Node->_M_token);
                    _Node = _Node->_M_next;
                }
            }

            void push_back(_CancellationTokenRegistration* _Token)
            {
                auto _Node = new Node(_Token);
                if (_M_begin == nullptr)
                {
                    _M_begin = _Node;
                }
                else
                {
                    _M_last->_M_next = _Node;
                }

                _M_last = _Node;
            }

            void remove(_CancellationTokenRegistration* _Token)
            {
                Node* _Node = _M_begin;
                Node* _Prev = nullptr;

                while (_Node != nullptr)
                {
                    if (_Node->_M_token == _Token) {
                        if (_Prev == nullptr)
                        {
                            _M_begin = _Node->_M_next;
                        }
                        else
                        {
                            _Prev->_M_next = _Node->_M_next;
                        }

                        if (_Node->_M_next == nullptr)
                        {
                            _M_last = _Prev;
                        }

                        delete _Node;
                        break;
                    }

                    _Prev = _Node;
                    _Node = _Node->_M_next;
                }
            }

        private:
            Node *_M_begin;
            Node *_M_last;
        };

    public:
        static _CancellationTokenState * _NewTokenState()
        {
            return new _CancellationTokenState();
        }

        static _CancellationTokenState *_None()
        {
            return reinterpret_cast<_CancellationTokenState *>(2);
        }

        static bool _IsValid(_In_opt_ _CancellationTokenState *_PToken)
        {
            return (_PToken != nullptr && _PToken != _None());
        }

        _CancellationTokenState() :
            _M_stateFlag(0)
        {
        }

        ~_CancellationTokenState()
        {
            TokenRegistrationContainer _RundownList;
            {
                ::std::lock_guard<::std::mutex> _Lock(_M_listLock);
                _M_registrations.swap(_RundownList);
            }

            _RundownList.for_each([](_CancellationTokenRegistration * _PRegistration)
            {
                _PRegistration->_M_state = _CancellationTokenRegistration::_STATE_SYNCHRONIZE;
                _PRegistration->_Release();
            });
        }

        bool _IsCanceled() const
        {
            return (_M_stateFlag != 0);
        }

        void _Cancel()
        {
            if (atomic_compare_exchange(_M_stateFlag, 1l, 0l) == 0)
            {
                TokenRegistrationContainer _RundownList;
                {
                    ::std::lock_guard<::std::mutex> _Lock(_M_listLock);
                    _M_registrations.swap(_RundownList);
                }

                _RundownList.for_each([](_CancellationTokenRegistration * _PRegistration)
                {
                    _PRegistration->_Invoke();
                });

                _M_stateFlag = 2;
            }
        }

        _CancellationTokenRegistration *_RegisterCallback(TaskProc_t _PCallback, _In_ void *_PData, int _InitialRefs = 1)
        {
            _CancellationTokenRegistration *_PRegistration = new CancellationTokenRegistration_TaskProc(_PCallback, _PData, _InitialRefs);
            _RegisterCallback(_PRegistration);
            return _PRegistration;
        }

        void _RegisterCallback(_In_ _CancellationTokenRegistration *_PRegistration)
        {
            _PRegistration->_M_state = _CancellationTokenRegistration::_STATE_CLEAR;
            _PRegistration->_Reference();
            _PRegistration->_M_pTokenState = this;

            bool _Invoke = true;

            if (!_IsCanceled())
            {
                ::std::lock_guard<::std::mutex> _Lock(_M_listLock);

                if (!_IsCanceled())
                {
                    _Invoke = false;
                    _M_registrations.push_back(_PRegistration);
                }
            }

            if (_Invoke)
            {
                _PRegistration->_Invoke();
            }
        }

        void _DeregisterCallback(_In_ _CancellationTokenRegistration *_PRegistration)
        {
            bool _Synchronize = false;

            {
                ::std::lock_guard<::std::mutex> _Lock(_M_listLock);

                //
                // If a cancellation has occurred, the registration list is guaranteed to be empty if we've observed it under the auspices of the
                // lock.  In this case, we must synchronize with the cancelling thread to guarantee that the cancellation is finished by the time
                // we return from this method.
                //
                if (!_M_registrations.empty())
                {
                    _M_registrations.remove(_PRegistration);
                    _PRegistration->_M_state = _CancellationTokenRegistration::_STATE_SYNCHRONIZE;
                    _PRegistration->_Release();
                }
                else
                {
                    _Synchronize = true;
                }
            }

            //
            // If the list is empty, we are in one of several situations:
            //
            // - The callback has already been made         --> do nothing
            // - The callback is about to be made           --> flag it so it doesn't happen and return
            // - The callback is in progress elsewhere      --> synchronize with it
            // - The callback is in progress on this thread --> do nothing
            //
            if (_Synchronize)
            {
                const long _Result = atomic_compare_exchange(
                    _PRegistration->_M_state,
                    _CancellationTokenRegistration::_STATE_DEFER_DELETE,
                    _CancellationTokenRegistration::_STATE_CLEAR
                    );

                switch(_Result)
                {
                    case _CancellationTokenRegistration::_STATE_CLEAR:
                    case _CancellationTokenRegistration::_STATE_CALLED:
                        break;
                    case _CancellationTokenRegistration::_STATE_DEFER_DELETE:
                    case _CancellationTokenRegistration::_STATE_SYNCHRONIZE:
                        _ASSERTE(false);
                        break;
                    default:
                    {
                        if (_Result == ::Concurrency::details::platform::GetCurrentThreadId())
                        {
                            //
                            // It is entirely legal for a caller to Deregister during a callback instead of having to provide their own synchronization
                            // mechanism between the two.  In this case, we do *NOT* need to explicitly synchronize with the callback as doing so would
                            // deadlock.  If the call happens during, skip any extra synchronization.
                            //
                            break;
                        }

                        const long _Result_1 = atomic_exchange(_PRegistration->_M_state, _CancellationTokenRegistration::_STATE_SYNCHRONIZE);

                        if (_Result_1 != _CancellationTokenRegistration::_STATE_CALLED)
                        {
                            ::std::unique_lock<::std::mutex> _Lock(_PRegistration->_M_Mutex);
                            _PRegistration->_M_CondVar.wait(_Lock,
                                [_PRegistration]{ return _PRegistration->_M_signaled; });

                            _ASSERTE(_PRegistration->_M_signaled);
                        }

                        break;
                    }
                }
            }
        }

    private:

        // The flag for the token state (whether it is canceled or not)
        atomic_long _M_stateFlag;

        // Lock to protect the registrations list
        ::std::mutex _M_listLock;

        // The protected list of registrations
        TokenRegistrationContainer _M_registrations;
    };

} // namespace details

class cancellation_token_source;
class cancellation_token;


/// <summary>
///     The <c>cancellation_token_registration</c> class represents a callback notification from a <c>cancellation_token</c>.  When the <c>register</c>
///     method on a <c>cancellation_token</c> is used to receive notification of when cancellation occurs, a <c>cancellation_token_registration</c>
///     object is returned as a handle to the callback so that the caller can request a specific callback no longer be made through use of
///     the <c>deregister</c> method.
/// </summary>
class cancellation_token_registration
{
public:

    cancellation_token_registration() noexcept :
        _M_pRegistration(nullptr)
    {
    }

    ~cancellation_token_registration()
    {
        _Clear();
    }

    cancellation_token_registration(const cancellation_token_registration& _Src) noexcept
    {
        _Assign(_Src._M_pRegistration);
    }

    cancellation_token_registration(cancellation_token_registration&& _Src) noexcept
    {
        _Move(_Src._M_pRegistration);
    }

    cancellation_token_registration& operator=(const cancellation_token_registration& _Src) noexcept
    {
        if (this != &_Src)
        {
            _Clear();
            _Assign(_Src._M_pRegistration);
        }
        return *this;
    }

    cancellation_token_registration& operator=(cancellation_token_registration&& _Src) noexcept
    {
        if (this != &_Src)
        {
            _Clear();
            _Move(_Src._M_pRegistration);
        }
        return *this;
    }

    bool operator==(const cancellation_token_registration& _Rhs) const
    {
        return _M_pRegistration == _Rhs._M_pRegistration;
    }

    bool operator!=(const cancellation_token_registration& _Rhs) const
    {
        return !(operator==(_Rhs));
    }

private:

    friend class cancellation_token;

    cancellation_token_registration(_In_ details::_CancellationTokenRegistration *_PRegistration) noexcept :
        _M_pRegistration(_PRegistration)
    {
    }

    void _Clear() noexcept
    {
        if (_M_pRegistration != nullptr)
        {
            _M_pRegistration->_Release();
        }
        _M_pRegistration = nullptr;
    }

    void _Assign(_In_ details::_CancellationTokenRegistration *_PRegistration) noexcept
    {
        if (_PRegistration != nullptr)
        {
            _PRegistration->_Reference();
        }
        _M_pRegistration = _PRegistration;
    }

    void _Move(_In_ details::_CancellationTokenRegistration *&_PRegistration) noexcept
    {
        _M_pRegistration = _PRegistration;
        _PRegistration = nullptr;
    }

    details::_CancellationTokenRegistration *_M_pRegistration;
};


/// <summary>
///     The <c>cancellation_token</c> class represents the ability to determine whether some operation has been requested to cancel.  A given token can
///     be associated with a <c>task_group</c>, <c>structured_task_group</c>, or <c>task</c> to provide implicit cancellation.  It can also be polled for
///     cancellation or have a callback registered for if and when the associated <c>cancellation_token_source</c> is canceled.
/// </summary>
class cancellation_token
{
public:

    typedef details::_CancellationTokenState * _ImplType;

    /// <summary>
    ///     Returns a cancellation token which can never be subject to cancellation.
    /// </summary>
    /// <returns>
    ///     A cancellation token that cannot be canceled.
    /// </returns>
    static cancellation_token none()
    {
        return cancellation_token();
    }

    cancellation_token(const cancellation_token& _Src)
    {
        _Assign(_Src._M_Impl);
    }

    cancellation_token(cancellation_token&& _Src) noexcept
    {
        _Move(_Src._M_Impl);
    }

    cancellation_token& operator=(const cancellation_token& _Src)
    {
        if (this != &_Src)
        {
            _Clear();
            _Assign(_Src._M_Impl);
        }
        return *this;
    }

    cancellation_token& operator=(cancellation_token&& _Src) noexcept
    {
        if (this != &_Src)
        {
            _Clear();
            _Move(_Src._M_Impl);
        }
        return *this;
    }

    bool operator==(const cancellation_token& _Src) const
    {
        return _M_Impl == _Src._M_Impl;
    }

    bool operator!=(const cancellation_token& _Src) const
    {
        return !(operator==(_Src));
    }

    ~cancellation_token()
    {
        _Clear();
    }

    /// <summary>
    ///     Returns an indication of whether this token can be canceled or not.
    /// </summary>
    /// <returns>
    ///     An indication of whether this token can be canceled or not.
    /// </returns>
    bool is_cancelable() const
    {
        return (_M_Impl != nullptr);
    }

    /// <summary>
    /// Returns <c>true</c> if the token has been canceled.
    /// </summary>
    /// <returns>
    /// The value <c>true</c> if the token has been canceled; otherwise, the value <c>false</c>.
    /// </returns>
    bool is_canceled() const
    {
        return (_M_Impl != nullptr && _M_Impl->_IsCanceled());
    }

    /// <summary>
    ///     Registers a callback function with the token.  If and when the token is canceled, the callback will be made.  Note that if the token
    ///     is already canceled at the point where this method is called, the callback will be made immediately and synchronously.
    /// </summary>
    /// <typeparam name="_Function">
    ///     The type of the function object that will be called back when this <c>cancellation_token</c> is canceled.
    /// </typeparam>
    /// <param name="_Func">
    ///     The function object that will be called back when this <c>cancellation_token</c> is canceled.
    /// </param>
    /// <returns>
    ///     A <c>cancellation_token_registration</c> object which can be utilized in the <c>deregister</c> method to deregister a previously registered
    ///     callback and prevent it from being made. The method will throw an <see cref="invalid_operation Class">invalid_operation </see> exception if
    ///     it is called on a <c>cancellation_token</c> object that was created using the <see cref="cancellation_token::none Method">cancellation_token::none </see>
    ///     method.
    /// </returns>
    template<typename _Function>
    ::Concurrency::cancellation_token_registration register_callback(const _Function& _Func) const
    {
        if (_M_Impl == nullptr)
        {
            // A callback cannot be registered if the token does not have an associated source.
            throw invalid_operation();
        }
#pragma warning(suppress: 28197)
        details::_CancellationTokenCallback<_Function> *_PCallback = new details::_CancellationTokenCallback<_Function>(_Func);
        _M_Impl->_RegisterCallback(_PCallback);
        return cancellation_token_registration(_PCallback);
    }

    /// <summary>
    ///     Removes a callback previously registered via the <c>register</c> method based on the <c>cancellation_token_registration</c> object returned
    ///     at the time of registration.
    /// </summary>
    /// <param name="_Registration">
    ///     The <c>cancellation_token_registration</c> object corresponding to the callback to be deregistered.  This token must have been previously
    ///     returned from a call to the <c>register</c> method.
    /// </param>
    void deregister_callback(const cancellation_token_registration& _Registration) const
    {
        _M_Impl->_DeregisterCallback(_Registration._M_pRegistration);
    }

    _ImplType _GetImpl() const
    {
        return _M_Impl;
    }

    _ImplType _GetImplValue() const
    {
        return (_M_Impl == nullptr) ? ::Concurrency::details::_CancellationTokenState::_None() : _M_Impl;
    }

    static cancellation_token _FromImpl(_ImplType _Impl)
    {
        return cancellation_token(_Impl);
    }

private:

    friend class cancellation_token_source;

    _ImplType _M_Impl;

    void _Clear()
    {
        if (_M_Impl != nullptr)
        {
            _M_Impl->_Release();
        }
        _M_Impl = nullptr;
    }

    void _Assign(_ImplType _Impl)
    {
        if (_Impl != nullptr)
        {
            _Impl->_Reference();
        }
        _M_Impl = _Impl;
    }

    void _Move(_ImplType &_Impl)
    {
        _M_Impl = _Impl;
        _Impl = nullptr;
    }

    cancellation_token() :
        _M_Impl(nullptr)
    {
    }

    cancellation_token(_ImplType _Impl) :
        _M_Impl(_Impl)
    {
        if (_M_Impl == ::Concurrency::details::_CancellationTokenState::_None())
        {
            _M_Impl = nullptr;
        }

        if (_M_Impl != nullptr)
        {
            _M_Impl->_Reference();
        }
    }
};

/// <summary>
///     The <c>cancellation_token_source</c> class represents the ability to cancel some cancelable operation.
/// </summary>
class cancellation_token_source
{
public:

    typedef ::Concurrency::details::_CancellationTokenState * _ImplType;

    /// <summary>
    ///     Constructs a new <c>cancellation_token_source</c>.  The source can be used to flag cancellation of some cancelable operation.
    /// </summary>
    cancellation_token_source()
    {
        _M_Impl = new ::Concurrency::details::_CancellationTokenState;
    }

    cancellation_token_source(const cancellation_token_source& _Src)
    {
        _Assign(_Src._M_Impl);
    }

    cancellation_token_source(cancellation_token_source&& _Src) noexcept
    {
        _Move(_Src._M_Impl);
    }

    cancellation_token_source& operator=(const cancellation_token_source& _Src)
    {
        if (this != &_Src)
        {
            _Clear();
            _Assign(_Src._M_Impl);
        }
        return *this;
    }

    cancellation_token_source& operator=(cancellation_token_source&& _Src) noexcept
    {
        if (this != &_Src)
        {
            _Clear();
            _Move(_Src._M_Impl);
        }
        return *this;
    }

    bool operator==(const cancellation_token_source& _Src) const
    {
        return _M_Impl == _Src._M_Impl;
    }

    bool operator!=(const cancellation_token_source& _Src) const
    {
        return !(operator==(_Src));
    }

    ~cancellation_token_source()
    {
        if (_M_Impl != nullptr)
        {
            _M_Impl->_Release();
        }
    }

    /// <summary>
    ///     Returns a cancellation token associated with this source.  The returned token can be polled for cancellation
    ///     or provide a callback if and when cancellation occurs.
    /// </summary>
    /// <returns>
    ///     A cancellation token associated with this source.
    /// </returns>
    cancellation_token get_token() const
    {
        return cancellation_token(_M_Impl);
    }

    /// <summary>
    ///     Creates a <c>cancellation_token_source</c> which is canceled when the provided token is canceled.
    /// </summary>
    /// <param name="_Src">
    ///     A token whose cancellation will cause cancellation of the returned token source.  Note that the returned token source can also be canceled
    ///     independently of the source contained in this parameter.
    /// </param>
    /// <returns>
    ///     A <c>cancellation_token_source</c> which is canceled when the token provided by the <paramref name="_Src"/> parameter is canceled.
    /// </returns>
    static cancellation_token_source create_linked_source(cancellation_token& _Src)
    {
        cancellation_token_source _NewSource;
        _Src.register_callback( [_NewSource](){ _NewSource.cancel(); } );
        return _NewSource;
    }

    /// <summary>
    ///     Creates a <c>cancellation_token_source</c> which is canceled when one of a series of tokens represented by an STL iterator
    ///     pair is canceled.
    /// </summary>
    /// <param name="_Begin">
    ///     The STL iterator corresponding to the beginning of the range of tokens to listen for cancellation of.
    /// </param>
    /// <param name="_End">
    ///     The STL iterator corresponding to the ending of the range of tokens to listen for cancellation of.
    /// </param>
    /// <returns>
    ///     A <c>cancellation_token_source</c> which is canceled when any of the tokens provided by the range described by the STL iterators
    ///     contained in the <paramref name="_Begin"/> and <paramref name="_End"/> parameters is canceled.
    /// </returns>
    template<typename _Iter>
    static cancellation_token_source create_linked_source(_Iter _Begin, _Iter _End)
    {
        cancellation_token_source _NewSource;
        for (_Iter _It = _Begin; _It != _End; ++_It)
        {
            _It->register_callback( [_NewSource](){ _NewSource.cancel(); } );
        }
        return _NewSource;
    }

    /// <summary>
    ///     Cancels the token.  Any <c>task_group</c>, <c>structured_task_group</c>, or <c>task</c> which utilizes the token will be
    ///     canceled upon this call and throw an exception at the next interruption point.
    /// </summary>
    void cancel() const
    {
        _M_Impl->_Cancel();
    }

    _ImplType _GetImpl() const
    {
        return _M_Impl;
    }

    static cancellation_token_source _FromImpl(_ImplType _Impl)
    {
        return cancellation_token_source(_Impl);
    }

private:

    _ImplType _M_Impl;

    void _Clear()
    {
        if (_M_Impl != nullptr)
        {
            _M_Impl->_Release();
        }
        _M_Impl = nullptr;
    }

    void _Assign(_ImplType _Impl)
    {
        if (_Impl != nullptr)
        {
            _Impl->_Reference();
        }
        _M_Impl = _Impl;
    }

    void _Move(_ImplType &_Impl)
    {
        _M_Impl = _Impl;
        _Impl = nullptr;
    }

    cancellation_token_source(_ImplType _Impl) :
        _M_Impl(_Impl)
    {
        if (_M_Impl == ::Concurrency::details::_CancellationTokenState::_None())
        {
            _M_Impl = nullptr;
        }

        if (_M_Impl != nullptr)
        {
            _M_Impl->_Reference();
        }
    }
};

} // namespace Concurrency

} // extern "C++"

#pragma pop_macro("new")
_STL_RESTORE_CLANG_WARNINGS
#pragma warning(pop)
#pragma pack(pop)

#endif // _STL_COMPILER_PREPROCESSOR

#endif // _PPLCANCELLATION_TOKEN_H
