/***
* ==++==
*
* Copyright (c) Microsoft Corporation. All rights reserved.
*
* ==--==
* =+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+
*
* pplwin.h
*
* Parallel Patterns Library - PPLTasks helpers
*
* =-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-
****/

#ifndef _PPLWIN_H
#define _PPLWIN_H

#include <yvals_core.h>

#if _STL_COMPILER_PREPROCESSOR

#include <pplinterface.h>
#include <condition_variable>
#include <mutex>
#include <system_error>
#include <memory>
#include <ppltaskscheduler.h>

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

class _DefaultPPLTaskScheduler final : public scheduler_interface
{
public:

    class _PPLTaskChore
    {
        _Threadpool_chore _M_Chore;
        TaskProc_t _M_proc;
        void *_M_param;

        static void __cdecl _Callback(void *_Args)
        {
            auto _Chore = ::std::unique_ptr<_PPLTaskChore>(static_cast<_PPLTaskChore*>(_Args));
            _Chore->_M_proc(_Chore->_M_param);
        }

    public:
        ~_PPLTaskChore()
        {
            _Release_chore(&_M_Chore);
        }

#pragma warning(push)
#pragma warning(disable : 4355) // 'this': used in base member initializer list (/Wall)
        _PPLTaskChore(TaskProc_t _Proc, void *_Param) : _M_Chore{&_Callback, this}, _M_proc(_Proc), _M_param(_Param)
        {
        }
#pragma warning(pop)

        void _Schedule()
        {
            if (_Schedule_chore(&_M_Chore) != 0)
            {
                delete this;
                _THROW(::std::runtime_error("Fail to schedule the chore!"));
            }
        }
    };

    virtual void schedule( TaskProc_t _Proc, void* _Param) override
    {
        (new _PPLTaskChore{ _Proc, _Param })->_Schedule();
    }
};

inline ::std::shared_ptr<scheduler_interface> * _GetStaticAmbientSchedulerStorage()
{
    // thread-unsafe local static OK here because we're guarded by ::std::call_once in the caller
#pragma warning(suppress: 4640)
    static ::std::shared_ptr<scheduler_interface> _S_scheduler;
    return &_S_scheduler;
}

inline ::std::shared_ptr<scheduler_interface> & _GetStaticAmbientSchedulerRef()
{
    static ::std::once_flag _Flag;
    static ::std::shared_ptr<scheduler_interface> * _S_scheduler_address;
    ::std::call_once(_Flag, [] {
        _S_scheduler_address = _GetStaticAmbientSchedulerStorage();
    });
    return *_S_scheduler_address;
}

} // namespace details

inline const ::std::shared_ptr<scheduler_interface> &get_ambient_scheduler()
{
    return details::_GetStaticAmbientSchedulerRef();
}

inline void set_ambient_scheduler(const ::std::shared_ptr<scheduler_interface>& _Scheduler)
{
    details::_GetStaticAmbientSchedulerRef() = _Scheduler;
}

namespace details
{
    // Used to report unobserved task exceptions in ppltasks.h
    [[noreturn]] _CRTIMP2 void __cdecl _ReportUnobservedException();

    namespace platform
    {
        _CRTIMP2 unsigned int __cdecl GetNextAsyncId();
        _CRTIMP2 size_t __cdecl CaptureCallstack(void **, size_t, size_t);
        _CRTIMP2 long __cdecl GetCurrentThreadId();
    }
}

} // Concurrency

} // extern "C++"

#include <pplcancellation_token.h>

extern "C++" { // attach declarations in namespace Concurrency to the global module, see N4910 [module.unit]/7

namespace Concurrency
{

namespace details
{

// It has to be a macro because the debugger needs __debugbreak
// breaks on the frame with exception pointer.
// It can be only used within _ExceptionHolder
#ifndef _REPORT_PPLTASK_UNOBSERVED_EXCEPTION
#define _REPORT_PPLTASK_UNOBSERVED_EXCEPTION() do { \
        ReportUnhandledError(); \
        __debugbreak(); \
        ::Concurrency::details::_ReportUnobservedException(); \
} while (false)

#endif

    struct _TaskProcHandle
    {
        _TaskProcHandle() {}
        virtual ~_TaskProcHandle() {}
        virtual void invoke() const = 0;

        void operator()() const
        {
            this->invoke();
        }

        static void __cdecl _RunChoreBridge(void * _Parameter)
        {
            ::std::unique_ptr<_TaskProcHandle> {static_cast<_TaskProcHandle *>(_Parameter)}->invoke();
        }
    };

    class _TaskCollectionBaseImpl
    {
    protected:
        enum _TaskCollectionState {
            _New,
            _Scheduled,
            _Completed
        };

        void _SetCollectionState(_TaskCollectionState _NewState)
        {
            _ASSERTE(_NewState != _New);
            ::std::lock_guard<::std::mutex> _Lock(_M_Cs);
            if (_M_State < _NewState)
            {
                _M_State = _NewState;
            }

            _M_StateChanged.notify_all();
        }

        void WaitUntilStateChangedTo(_TaskCollectionState _State)
        {
            ::std::unique_lock<::std::mutex> _Lock(_M_Cs);

            while(_M_State < _State)
            {
                _M_StateChanged.wait(_Lock);
            }
        }
    public:

        typedef _TaskProcHandle _TaskProcHandle_t;

        _TaskCollectionBaseImpl(::Concurrency::scheduler_ptr _PScheduler)
            : _M_pScheduler(_PScheduler), _M_State(_New)
        {
        }

        void _ScheduleTask(_TaskProcHandle_t* _Parameter, _TaskInliningMode _InliningMode)
        {
            if (_InliningMode == _ForceInline)
            {
                _TaskProcHandle_t::_RunChoreBridge(_Parameter);
            }
            else
            {
                if (_M_pScheduler)
                {
                    _M_pScheduler->schedule(_TaskProcHandle_t::_RunChoreBridge, _Parameter);
                }
                else
                {
                    _DefaultPPLTaskScheduler().schedule(_TaskProcHandle_t::_RunChoreBridge, _Parameter);
                }
            }
        }

        void _Cancel()
        {
            // No cancellation support
        }

        void _RunAndWait()
        {
            _Wait();
        }

        void _Wait()
        {
            WaitUntilStateChangedTo(_Completed);
        }

        void _Complete()
        {
            // Ensure that RunAndWait makes progress.
            _SetCollectionState(_Completed);
        }

        ::Concurrency::scheduler_ptr _GetScheduler() const
        {
            return _M_pScheduler;
        }

        // Fire and forget
        static void _RunTask(TaskProc_t _Proc, void * _Parameter, _TaskInliningMode _InliningMode)
        {
            if (_InliningMode == _ForceInline)
            {
                _Proc(_Parameter);
            }
            else
            {
                // Schedule the work on the default scheduler if the ambient scheduler is not set
                auto _Ptr = get_ambient_scheduler();
                if (_Ptr)
                {
                    _Ptr->schedule(_Proc, _Parameter);
                }
                else
                {
                    _DefaultPPLTaskScheduler().schedule(_Proc, _Parameter);
                }
            }
        }
    protected:
        ::std::condition_variable _M_StateChanged;
        ::std::mutex _M_Cs;
        ::Concurrency::scheduler_ptr _M_pScheduler;
        _TaskCollectionState _M_State;
    };

    typedef _TaskCollectionBaseImpl _TaskCollection_t;

    // Over-subscription is no longer needed for threadpool based ppltasks
    struct _Task_generator_oversubscriber {};

    typedef _TaskInliningMode _TaskInliningMode_t;
    typedef _Task_generator_oversubscriber _Task_generator_oversubscriber_t;

} // details
} // Concurrency

namespace concurrency = ::Concurrency;

} // extern "C++"

#pragma pop_macro("new")
_STL_RESTORE_CLANG_WARNINGS
#pragma warning(pop)
#pragma pack(pop)

#endif // _STL_COMPILER_PREPROCESSOR

#endif // _PPLWIN_H
