/* ++

Copyright (c) Microsoft Corporation.  All rights reserved.

Module Name:

    TraceLoggingActivity.h

Abstract:

    TraceLogging activity logging helper classes and macros for C++ for
    utilizing ETW activity ids.

Environment:

    User mode.

--*/

#pragma once
#ifndef RC_INVOKED

#include "TraceLoggingProvider.h"

/*
Quick start:

#include <windows.h>
#include <TraceLoggingProvider.h>
#include <TraceLoggingActivity.h>
#include <winmeta.h> // optional, used here for WINEVENT_LEVEL_ERROR

TRACELOGGING_DEFINE_PROVIDER(
    g_hProvider,
    "MyProvider",
    (0xb3864c38, 0x4273, 0x58c5, 0x54, 0x5b, 0x8b, 0x36, 0x08, 0x34, 0x34, 0x71));

int main(int argc, char* argv[])
{
    const ULONGLONG MyKeywordValue = 0x10;

    TraceLoggingRegister(g_hProvider);

    // Declare a local activity variable.
    // Use the default (0 = none) for the activity's keyword.
    // Use the default (5 = verbose) for the activity's level.
    TraceLoggingActivity<g_hProvider> parentActivity;

    // Write the activity's START event. Note that you must not specify keyword
    // or level for START and STOP events because they always use the activity's
    // keyword and level.
    TraceLoggingWriteStart(
        parentActivity,
        "ParentActivity", // The name of the start event is used as the activity name.
        TraceLoggingValue(argc, "SampleField1"));

    {
        // Declare a local (nested) activity variable, with explicit keyword and level:
        TraceLoggingActivity<g_hProvider, MyKeywordValue, WINEVENT_LEVEL_ERROR> nestedActivity;

        // Connect the nested activity to the parent:
        nestedActivity.SetRelatedActivity(parentActivity);

        // Write the activity's START event.
        TraceLoggingWriteStart(nestedActivity, "NestedActivity");

        // TraceLoggingWriteTagged writes an event that is tagged with an activity
        // ID. Note that TraceLoggingWriteTagged is just like TraceLoggingWrite
        // except that the first parameter is an activity (not a provider) and the
        // resulting event is explicitly tagged with the activity ID. In particular,
        // TraceLoggingWriteTagged does NOT use the activity's level and keyword --
        // they are specified just as they would be for a TraceLoggingWrite event.
        TraceLoggingWriteTagged(
            nestedActivity,
            "NestedActivityIdTaggedEvent",
            TraceLoggingValue(argc, "SampleField2"),
            TraceLoggingKeyword(MyKeywordValue));

        // Write the activity's STOP event.
        TraceLoggingWriteStop(nestedActivity, "NestedActivity");

        // Note that if the activity is destroyed without being stopped, a default
        // "ActivityStoppedAutomatically" event will be written by the destructor.
    }

    TraceLoggingWriteStop(parentActivity, "ParentActivity");

    TraceLoggingUnregister(g_hProvider);
    return 0;
}
*/

#pragma region Public Interface

/*
Classes:

- TraceLoggingActivity: General purpose activity class.
- TraceLoggingThreadActivity: Automatic activity class. (Desktop apps only.)
- TraceLoggingThreadActivityIdSetter: Helper for controlling the thread-local
  activity ID. (Desktop apps only.)

Macros:

- TraceLoggingWriteStart: Writes an activity START event.
- TraceLoggingWriteStop: Writes an activity STOP event.
- TraceLoggingWriteTagged: Writes an event explicitly tagged with an activity ID.
- TraceLoggingWriteTaggedIfStarted: TraceLoggingWriteTagged if the activity was
  started.
- TraceLoggingFunction: Creates a TraceLoggingThreadActivity and writes a START
  event with the current function's name. (Desktop apps only.)

Basic usage:

1. Create an instance of TraceLoggingActivity or TraceLoggingThreadActivity.
2. Call TraceLoggingWriteStart to write the activity's START event.
3. Call TraceLoggingWriteTagged to write events associated with the activity.
4. Call TraceLoggingWriteStop to write the activity's STOP event.
*/

/*
TraceLoggingActivity implements general-purpose activity support. It supports
nesting via an explicit childActivity.SetRelatedActivity(parentActivity) method.
Events associated with the activity can then be written using
TraceLoggingWriteTagged.

Activities can be nested by calling childActivity.SetRelatedActivity(parentActivity)
before calling TraceLoggingWriteStart(childActivity, ...).

Use TraceLoggingWriteStart to generate a unique activity ID and write the START
event for the activity.

Use TraceLoggingWriteTagged to write events that are associated with the activity.

Use TraceLoggingWriteStop to write the STOP event for the activity.

If the activity is destroyed without being stopped, it will log a default STOP
event.
*/
template<
    TraceLoggingHProvider const& provider,
    UINT64 keyword = 0, // defaults to 0 = none
    UINT8 level = 5, // defaults to 5 = WINEVENT_LEVEL_VERBOSE
    typename TlgReflectorTag = _TlgReflectorTag_Param0IsHProvider> // helps TlgReflector understand that this is a wrapper type
class TraceLoggingActivity;

#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)

/*
TraceLoggingThreadActivity implements automatic activities using a thread-local
CurrentActivityId variable. Each new activity is automatically nested within the
activity that previously set the CurrentActivityId variable, and all events
written using TraceLoggingWrite are automatically associated with the
CurrentActivityId. This automatic behavior can be convenient in some cases, but it
only works correctly when the activities nest precisely and do not cross thread
boundaries. In addition, TraceLoggingThreadActivity is not available on the
Windows Runtime (i.e. this class cannot be used for Windows Store or Windows Phone
apps).

PLEASE NOTE: use TraceLoggingThreadActivity only when you can guarantee that
lifetimes of all activities of this type on the thread fully nest in all cases, i.e.
a parent activity will not end before a child activity, even in error or exceptional
cases. Activities defined in function-local scopes typically meet this criteria.

Use TraceLoggingWriteStart to generate a unique activity ID and write the START
event for the activity.

Use TraceLoggingWrite to write events that are automatically associated with the
activity.

Use TraceLoggingWriteTagged to write events that are explicitly associated with the
activity.

Use TraceLoggingWriteStop to write the STOP event for the activity.

If the activity is destroyed without being stopped, it will log a default STOP
event.
*/
template<
    TraceLoggingHProvider const& provider,
    UINT64 keyword = 0, // defaults to 0 = none
    UINT8 level = 5, // defaults to 5 = WINEVENT_LEVEL_VERBOSE
    typename TlgReflectorTag = _TlgReflectorTag_Param0IsHProvider> // helps TlgReflector understand that this is a wrapper type
class TraceLoggingThreadActivity;

/*
TraceLoggingThreadActivityIdSetter is a helper class that sets the thread-local
CurrentActivity variable during construction and saves the original value. It then
restores the original value during destruction. This class is not available on the
Windows Runtime (i.e. it cannot be used for Windows Store or Windows Phone apps).

As with thread activities, lifetime of an instance of this class should
be carefully managed to avoid imperfect nesting.
*/
class TraceLoggingThreadActivityIdSetter;

#endif // WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)

/*
Macro TraceLoggingWriteStart(activity, "EventName", args...):
Invoke this macro to start an activity and log the start event.

The event name must be a string literal (not a variable) and must not contain
any '\0' characters.

Each arg must be a wrapper macro such as TraceLoggingValue, TraceLoggingInt32,
etc.

Do not specify TraceLoggingKeyword or TraceLoggingLevel as args for this macro.
The START and STOP events always use the activity's keyword and level.
*/
#define TraceLoggingWriteStart(activity, name, ...) \
    __pragma(warning(push)) __pragma(warning(disable:4127)) \
    do { \
        _tlgActivityDecl(activity) \
        static const UINT64 _tlgActivity_Keyword = _tlgActivityRef(activity).Keyword;\
        static const UINT8 _tlgActivity_Level = _tlgActivityRef(activity).Level;\
        static_assert( \
            _tlgActivity_Keyword == (_tlgActivity_Keyword _tlg_FOREACH(_tlgKeywordVal, __VA_ARGS__)), \
            "Do not use TraceLoggingKeyword in TraceLoggingWriteStart. Keywords for START events are " \
            "specified in the activity type, e.g. TraceLoggingActivity<Provider,Keyword,Level>."); \
        static_assert( \
            _tlgActivity_Level == (_tlgActivity_Level _tlg_FOREACH(_tlgLevelVal, __VA_ARGS__)), \
            "Do not use TraceLoggingLevel in TraceLoggingWriteStart. The Level for START events is " \
            "specified in the activity type, e.g. TraceLoggingActivity<Provider,Keyword,Level>."); \
        _tlgActivityRef(activity).zInternalStart(); \
        _tlgActivityWriteStartStop(_tlg_IS_EMPTY(__VA_ARGS__), ( \
            activity, \
            (name), \
            _tlgActivityRef(activity).Id(), \
            _tlgActivityRef(activity).zInternalRelatedId(), \
            1 /* WINEVENT_OPCODE_START */, \
            _tlgActivity_Keyword, \
            _tlgActivity_Level, \
            __VA_ARGS__)); \
    } while(0) \
    __pragma(warning(pop)) \

/*
Macro TraceLoggingWriteStop(activity, "EventName", args...):
Invoke this macro to stop an activity and log the stop event.

The event name must be a string literal (not a variable) and must not contain
any '\0' characters.

Each arg must be a wrapper macro such as TraceLoggingValue, TraceLoggingInt32,
etc.

Do not specify TraceLoggingKeyword or TraceLoggingLevel as args for this macro.
The START and STOP events always use the activity's keyword and level.

If an activity is started and is destroyed before it is stopped, a default
event will be logged from the destructor.
*/
#define TraceLoggingWriteStop(activity, name, ...) \
    __pragma(warning(push)) __pragma(warning(disable:4127)) \
    do { \
        _tlgActivityDecl(activity) \
        static const UINT64 _tlgActivity_Keyword = _tlgActivityRef(activity).Keyword;\
        static const UINT8 _tlgActivity_Level = _tlgActivityRef(activity).Level;\
        static_assert( \
            _tlgActivity_Keyword == (_tlgActivity_Keyword _tlg_FOREACH(_tlgKeywordVal, __VA_ARGS__)), \
            "Do not use TraceLoggingKeyword in TraceLoggingWriteStop. Keywords for STOP events are " \
            "specified in the activity type, e.g. TraceLoggingActivity<Provider,Keyword,Level>."); \
        static_assert( \
            _tlgActivity_Level == (_tlgActivity_Level _tlg_FOREACH(_tlgLevelVal, __VA_ARGS__)), \
            "Do not use TraceLoggingLevel in TraceLoggingWriteStop. The Level for STOP events is " \
            "specified in the activity type, e.g. TraceLoggingActivity<Provider,Keyword,Level>."); \
        _tlgActivityRef(activity).zInternalStop(); \
        _tlgActivityWriteStartStop(_tlg_IS_EMPTY(__VA_ARGS__), ( \
            activity, \
            (name), \
            _tlgActivityRef(activity).Id(), \
            NULL, \
            2 /* WINEVENT_OPCODE_STOP */,\
            _tlgActivity_Keyword,\
            _tlgActivity_Level,\
            __VA_ARGS__)); \
    } while(0) \
    __pragma(warning(pop)) \

/*
Macro TraceLoggingWriteTagged(activity, "EventName", args...):
Invoke this macro to log an event that is explicitly tagged with the activity id
of the specified activity.

The event name must be a string literal (not a variable) and must not contain
any '\0' characters.

Each arg must be a wrapper macro such as TraceLoggingValue, TraceLoggingInt32,
etc.

Note that events written with this macro do NOT use the activity's keyword and
level. If the level is not specified via TraceLoggingLevel, the level will default
to verbose (5). If the keyword is not specified via TraceLoggingKeyword, the keyword
will default to none (0).
*/
#define TraceLoggingWriteTagged(activity, name, ...) \
    __pragma(warning(push)) __pragma(warning(disable:4127)) \
    do { \
        _tlgActivityDecl(activity) \
        TraceLoggingWriteActivity( \
            _tlgActivityRef(activity).Provider(), \
            (name), \
            _tlgActivityRef(activity).Id(), \
            NULL, \
            __VA_ARGS__); \
    } while(0) \
    __pragma(warning(pop)) \

/*
Macro TraceLoggingWriteTaggedIfStarted(activity, "EventName", args...):
Invoke this macro to log an event that is explicitly tagged with the activity id
of the specified activity. This macro logs the event only if the activity is
started.

The event name must be a string literal (not a variable) and must not contain
any '\0' characters.

Each arg must be a wrapper macro such as TraceLoggingValue, TraceLoggingInt32,
etc.

Note that events written with this macro do NOT use the activity's keyword and
level. If the level is not specified via TraceLoggingLevel, the level will default
to verbose (5). If the keyword is not specified via TraceLoggingKeyword, the keyword
will default to none (0).
*/
#define TraceLoggingWriteTaggedIfStarted(activity, name, ...) \
    __pragma(warning(push)) __pragma(warning(disable:4127)) \
    do { \
        _tlgActivityDecl(activity) \
        if (_tlgActivityRef(activity).IsStarted()) { \
        TraceLoggingWriteActivity( \
            _tlgActivityRef(activity).Provider(), \
            (name), \
            _tlgActivityRef(activity).Id(), \
            NULL, \
            __VA_ARGS__); \
        } \
    } while(0) \
    __pragma(warning(pop)) \

/*
Macro TraceLoggingFunction(hProvider, args...):
Invoke this macro at the beginning of a function to define an activity and
start it. This will log a start event with the name of the function. When
the function exits, the activity class will log an automatically-generated
stop event.

Each arg must be a wrapper macro such as TraceLoggingValue, TraceLoggingInt32,
etc.

Since this macro uses a thread activity, please read the description for
TraceLoggingThreadActivity and ensure the function call or other thread
activities started within perfectly nest within each other even in cases of
errors and exceptions.
*/
#define TraceLoggingFunction(providerHandle, ...)\
    TraceLoggingThreadActivity< \
        providerHandle, \
        0 _tlg_FOREACH(_tlgKeywordVal, __VA_ARGS__), \
        5 _tlg_FOREACH(_tlgLevelVal, __VA_ARGS__)> \
        _tlgFnActivity; \
    TraceLoggingWriteStart(_tlgFnActivity, _tlgThisFunctionName, __VA_ARGS__)

#pragma endregion

#pragma region Implementation

/*
Private implementation macros. For internal use only.
Avoid IntelliSense errors about __FUNCTION__ only being valid within a function.
*/
#ifdef __INTELLISENSE__
#define _tlgThisFunctionName "ThisFunctionName"
#else
#define _tlgThisFunctionName __FUNCTION__
#endif

/*
Private implementation macros. For internal use only.
If the auto keyword is available, we can avoid referencing the activity
parameter more than once.
*/
#if defined(_MSC_VER) && (_MSC_VER >= 1600)
#define _tlgActivityDecl(activity) auto& _tlgActivity(activity);
#define _tlgActivityRef(activity) _tlgActivity
#else
#define _tlgActivityDecl(activity)
#define _tlgActivityRef(activity) (activity)
#endif

/*
Private implementation macros. For internal use only.
Avoid extra comma when invoking TraceLoggingWriteActivity.
*/
#define _tlgActivityWriteStartStop(is_empty, args) \
    _tlg_PASTE2(_tlgActivityWriteStartStop_imp, is_empty) args
#define _tlgActivityWriteStartStop_imp0(activity, eventName, pActivityId, pRelatedId, opcode, keyword, level, ...) \
    TraceLoggingWriteActivity( \
        _tlgActivityRef(activity).Provider(), \
        eventName, \
        pActivityId, \
        pRelatedId, \
        TraceLoggingOpcode(opcode), \
        TraceLoggingKeyword(keyword), \
        TraceLoggingLevel(level), \
        TraceLoggingDescription("~^" _tlg_LSTRINGIZE(activity) L"^~") \
        , __VA_ARGS__ \
        )
#define _tlgActivityWriteStartStop_imp1(activity, eventName, pActivityId, pRelatedId, opcode, keyword, level, ...) \
    TraceLoggingWriteActivity( \
        _tlgActivityRef(activity).Provider(), \
        eventName, \
        pActivityId, \
        pRelatedId, \
        TraceLoggingOpcode(opcode), \
        TraceLoggingKeyword(keyword), \
        TraceLoggingLevel(level), \
        TraceLoggingDescription("~^" _tlg_LSTRINGIZE(activity) L"^~") \
        )

/*
Private implementation function. For internal use only.
Separate function to minimize code bloat by consolidating the auto-stop events.
*/
template<UINT64 keyword, UINT8 level>
void _tlgWriteActivityAutoStop(
    TraceLoggingHProvider provider,
    _In_ GUID const* pActivityId)
{
    TraceLoggingWriteActivity(
        provider,
        "ActivityStoppedAutomatically",
        pActivityId,
        NULL,
        TraceLoggingOpcode(2 /* WINEVENT_OPCODE_STOP */),
        TraceLoggingKeyword(keyword),
        TraceLoggingLevel(level));
}

/*
Private implementation function. For internal use only.
*/
inline bool _tlgGuidIsZero(GUID const& g)
{
    INT32 const* p = reinterpret_cast<INT32 const*>(&g);
    return
        p[0] == 0 &&
        p[1] == 0 &&
        p[2] == 0 &&
        p[3] == 0;
}

/*
Private implementation function. For internal use only.
*/
inline bool _tlgGuidEqual(GUID const& g1, GUID const& g2)
{
    INT32 const* p1 = reinterpret_cast<INT32 const*>(&g1);
    INT32 const* p2 = reinterpret_cast<INT32 const*>(&g2);
    return
        p1[0] == p2[0] &&
        p1[1] == p2[1] &&
        p1[2] == p2[2] &&
        p1[3] == p2[3];
}

/*
Private implementation class. For internal use only.
*/
template<
    typename DerivedTy, // DerivedTy must define Provider(), OnStarted(), OnStopped().
    UINT64 keyword,
    UINT8 level>
class _TlgActivityBase
{
    _TlgActivityBase(const _TlgActivityBase&); // = delete
    _TlgActivityBase& operator=(const _TlgActivityBase&); // = delete

    void Reset()
    {
        m_State = TlgStateCreated;
        m_HasRelatedId = false;
    }

    enum TlgState
    {
        TlgStateCreated,
        TlgStateStarted,
        TlgStateStopped,
        TlgStateDestroyed
    };

    TlgState m_State;
    bool m_HasRelatedId;
    GUID m_Id;
    GUID m_CapturedRelatedId;

protected:

    ~_TlgActivityBase()
    {
        if (m_State == TlgStateStarted)
        {
            zInternalStop();
            _tlgWriteActivityAutoStop<keyword, level>(
                static_cast<DerivedTy*>(this)->Provider(),
                &m_Id);
        }

        m_State = TlgStateDestroyed;
    }

    _TlgActivityBase()
    {
        Reset();
    }

#if defined(_MSC_VER) && (_MSC_VER >= 1600)

    _TlgActivityBase(_TlgActivityBase&& rhs)
    {
        Reset();
        *this = static_cast<_TlgActivityBase&&>(rhs);
    }

    _TlgActivityBase& operator=(_TlgActivityBase&& rhs)
    {
        _tlg_ASSERT(m_State == TlgStateCreated, "Move-assign to newly created activities only");
        m_State = rhs.m_State;
        m_HasRelatedId = rhs.m_HasRelatedId;
        m_Id = rhs.m_Id;
        m_CapturedRelatedId = rhs.m_CapturedRelatedId;
        rhs.Reset();
        return *this;
    }

#endif // _MSC_VER >= 1600

    /*
    Should be called during activity setup (before activity starts).
    If this is used, do NOT use PushThreadActivityId or PopThreadActivityId.
    */
    void SetRelatedId(const GUID& relatedActivityId)
    {
        _tlg_ASSERT(m_State == TlgStateCreated, "_TlgActivityBase::SetRelatedId called from invalid state.");
        _tlg_ASSERT(!m_HasRelatedId, "_TlgActivityBase::RelatedActivity was already set.");
        m_CapturedRelatedId = relatedActivityId;
        m_HasRelatedId = true;
    }

    /*
    Should be called at an appropriate point in activity setup, after SetRelatedId() is called.
    Returns the related activity Id, if there is one.
    Returns NULL if the related activity Id has not been set or if it is GUID_NULL.
    */
    _Ret_opt_ const GUID* GetRelatedId() const
    {
        return m_HasRelatedId && !_tlgGuidIsZero(m_CapturedRelatedId) ? &m_CapturedRelatedId : NULL;
    }

#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)

    /*
    Should be called from OnStarted if applicable.
    If this is used, do NOT use SetRelatedId.
    */
    void PushThreadActivityId()
    {
        _tlg_ASSERT(m_State == TlgStateCreated, "_TlgActivityBase::PushThreadActivityId called from invalid state.");
        _tlg_ASSERT(!m_HasRelatedId, "_TlgActivityBase::RelatedActivity was already set.");
        m_CapturedRelatedId = m_Id;
        ::EventActivityIdControl(EVENT_ACTIVITY_CTRL_GET_SET_ID, &m_CapturedRelatedId);
        m_HasRelatedId = true;
    }

    /*
    Should be called from OnStopped if applicable.
    If this is used, do NOT use SetRelatedId.
    */
    void PopThreadActivityId()
    {
        _tlg_ASSERT(m_State == TlgStateStarted, "_TlgActivityBase::PopThreadActivityId called from invalid state.");
        if (m_HasRelatedId)
        {
            ::EventActivityIdControl(EVENT_ACTIVITY_CTRL_GET_SET_ID, &m_CapturedRelatedId);

            /*
            If this assertion fails, it means that your thread activity's ID was corrupted.
            For example:

            TraceLoggingWriteStart(activity1, ...); // Saves activity1.SavedId = thread.ActivityId, sets thread.ActivityId = activity1.Id().
            DoWork();
            TraceLoggingWrite(...); // Uses thread.ActivityId.
            TraceLoggingWriteStop(activity1, ...); // Verifies that thread.ActivityId == activity1.Id(), restores thread.ActivityId = activity1.SavedId.
            
            If DoWork() leaves thread.ActivityId unchanged, or if it properly restores
            thread.ActivityId to its original value before returning, then everything
            will work as expected. If DoWork changes thread.ActivityId to its own value
            and does not restore it before returning, the TraceLoggingWrite call will
            use the activity ID set by DoWork instead of using activity1.Id(). The
            assertion fires to warn you about this situation.

            The following are possible solutions for this issue:

            1. If you can change the DoWork() code: Fix the DoWork() code to correctly
               restore the activity ID before returning. Note that the activity ID should
               be restored even in cases where DoWork() exits via exception.
            2. If you cannot change the DoWork() code: Save the activity ID before
               calling DoWork(), then restore the activity ID after DoWork() returns. You
               can use the EventActivityIdControl function to save and restore the
               activity ID.
            3. Use TraceLoggingActivity instead of TraceLoggingThreadActivity so that you
               are not impacted by code that incorrectly manages thread.ActivityId. In
               this case, you will also need to switch from TraceLoggingWrite to
               TraceLoggingWriteTagged.

            Example of solution 2:

            GUID savedActivityId;
            EventActivityIdControl(EVENT_ACTIVITY_CTRL_GET_ID, &savedActivityId);
            CodeThatDoesNotRestoreActivityId();
            EventActivityIdControl(EVENT_ACTIVITY_CTRL_SET_ID, &savedActivityId);

            Note that this example solution is sufficient only in cases where
            the code in question does not throw exceptions.
            */
            _tlg_ASSERT(_tlgGuidEqual(m_Id, m_CapturedRelatedId), "_TlgActivityBase: current id does not match set id!");
        }
    }

#endif // WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)

public:

    /*
    The activity's Keyword, which is used as the keyword for the START and STOP events.
    */
    static const UINT64 Keyword = keyword;

    /*
    The activity's Level, which is used as the level for the START and STOP events.
    */
    static const UINT8 Level = level;

    /*
    Returns the activity Id.
    */
    _Ret_ const GUID* Id() const
    {
        _tlg_ASSERT(m_State >= TlgStateStarted, "TraceLoggingActivity::Id() called from invalid state");
        _tlg_ASSERT(m_State != TlgStateDestroyed, "TraceLoggingActivity::Id() called after destruction");
        return &m_Id;
    }

    /*
    Returns true if the activity has been started and has not yet been stopped.
    */
    bool IsStarted() const
    {
        return m_State == TlgStateStarted;
    }

    /*
    Private implementation method. For internal use only.
    Called by TraceLoggingWriteStart. Do not call directly.
    Returns the related activity Id, if there is one.
    Returns NULL if the related activity Id has not been set or if it is GUID_NULL.
    */
    _Ret_opt_ const GUID* zInternalRelatedId() const
    {
        _tlg_ASSERT(m_State == TlgStateStarted, "TraceLoggingWriteStart race condition");
        return GetRelatedId();
    }

    /*
    Private implementation method. For internal use only.
    Called by TraceLoggingWriteStart. Do not call directly.
    */
    void zInternalStart()
    {
        _tlg_ASSERT(m_State == TlgStateCreated, "TraceLoggingWriteStart called from invalid state.");

        DerivedTy* pDerived = static_cast<DerivedTy*>(this);
        if (TraceLoggingProviderEnabled(pDerived->Provider(), level, keyword))
        {
            CreateActivityId(0, pDerived, m_Id);
            pDerived->OnStarted();
        }
        else
        {
            // Zero the activity id in case we end up logging the stop.
            ZeroMemory(&m_Id, sizeof(m_Id));
        }

        m_State = TlgStateStarted;
    }

    /*
    Private implementation method. For internal use only.
    Called by TraceLoggingWriteStop. Do not call directly.
    */
    void zInternalStop()
    {
        _tlg_ASSERT(m_State == TlgStateStarted, "TraceLoggingWriteStop called from invalid state");

        DerivedTy* pDerived = static_cast<DerivedTy*>(this);
        pDerived->OnStopped();

        m_State = TlgStateStopped;
    }

private:
    // SFINAE dispatching on presence of DerivedTy::CreateActivityId
    template<typename _DerivedTy>
    static auto CreateActivityId(int, _In_ _DerivedTy* Derived, _Out_ GUID& ChildActivityId) ->
        decltype(Derived->CreateActivityId(ChildActivityId), void(0))
    {
        Derived->CreateActivityId(ChildActivityId);
    }

    template<typename _DerivedTy>
    static auto CreateActivityId(long, _In_ _DerivedTy* Derived, _Out_ GUID& ChildActivityId) ->
        void
    {
        (void)Derived;
        ::EventActivityIdControl(EVENT_ACTIVITY_CTRL_CREATE_ID, &ChildActivityId);
    }
};

template<
    TraceLoggingHProvider const& provider,
    UINT64 keyword,
    UINT8 level,
    typename TlgReflectorTag>
class TraceLoggingActivity
    : public _TlgActivityBase<TraceLoggingActivity<provider, keyword, level, TlgReflectorTag>, keyword, level>
{
    typedef
        _TlgActivityBase<TraceLoggingActivity<provider, keyword, level, TlgReflectorTag>, keyword, level>
        BaseTy;
    friend BaseTy;

    void OnStarted()
    {
    }

    void OnStopped()
    {
    }

public:

    TraceLoggingActivity()
    {
    }

#if defined(_MSC_VER) && (_MSC_VER >= 1600)

    TraceLoggingActivity(TraceLoggingActivity&& rhs)
        : BaseTy(static_cast<TraceLoggingActivity&&>(rhs))
    {
    }

    TraceLoggingActivity& operator=(TraceLoggingActivity&& rhs)
    {
        BaseTy::operator=(static_cast<TraceLoggingActivity&&>(rhs));
        return *this;
    }

#endif // _MSC_VER >= 1600

    /*
    Returns a handle to the TraceLogging provider associated with this activity.
    */
    TraceLoggingHProvider Provider() const
    {
        return provider;
    }

    /*
    Sets the related (parent) activity.
    May only be called once. Must be called before starting the activity.
    */
    template<typename ActivityTy>
    void SetRelatedActivity(_In_ const ActivityTy& relatedActivity)
    {
        BaseTy::SetRelatedId(*relatedActivity.Id());
    }

    /*
    Sets the related (parent) activity.
    May only be called once. Must be called before starting the activity.
    */
    void SetRelatedActivityId(_In_ const GUID& relatedActivityId)
    {
        BaseTy::SetRelatedId(relatedActivityId);
    }

    /*
    Sets the related (parent) activity.
    May only be called once. Must be called before starting the activity.
    */
    void SetRelatedActivityId(_In_ const GUID* relatedActivityId)
    {
        _tlg_ASSERT(relatedActivityId != NULL, "TraceLoggingActivity SetRelatedActivity called with NULL id.");
        BaseTy::SetRelatedId(*relatedActivityId);
    }
};

#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)

template<
    TraceLoggingHProvider const& provider,
    UINT64 keyword,
    UINT8 level,
    typename TlgReflectorTag>
class TraceLoggingThreadActivity
    : public _TlgActivityBase<TraceLoggingThreadActivity<provider, keyword, level, TlgReflectorTag>, keyword, level>
{
    typedef
        _TlgActivityBase<TraceLoggingThreadActivity<provider, keyword, level, TlgReflectorTag>, keyword, level>
        BaseTy;
    friend BaseTy;

    void OnStarted()
    {
        BaseTy::PushThreadActivityId();
    }

    void OnStopped()
    {
        BaseTy::PopThreadActivityId();
    }

public:

    TraceLoggingThreadActivity()
    {
    }

#if defined(_MSC_VER) && (_MSC_VER >= 1600)

    TraceLoggingThreadActivity(TraceLoggingThreadActivity&& rhs)
        : BaseTy(static_cast<TraceLoggingThreadActivity&&>(rhs))
    {
    }

    TraceLoggingThreadActivity& operator=(TraceLoggingThreadActivity&& rhs)
    {
        BaseTy::operator=(static_cast<TraceLoggingThreadActivity&&>(rhs));
        return *this;
    }

#endif // _MSC_VER >= 1600

    /*
    Returns a handle to the TraceLogging provider associated with this activity.
    */
    TraceLoggingHProvider Provider() const
    {
        return provider;
    }
};

class TraceLoggingThreadActivityIdSetter
{
    GUID m_ActivityId;
    GUID m_SavedActivityId;

    TraceLoggingThreadActivityIdSetter(const TraceLoggingThreadActivityIdSetter&); // = delete
    const TraceLoggingThreadActivityIdSetter& operator=(const TraceLoggingThreadActivityIdSetter&); // = delete

public:

    explicit TraceLoggingThreadActivityIdSetter(_In_ const GUID& activityId)
        : m_ActivityId(activityId)
        , m_SavedActivityId(activityId)
    {
        ::EventActivityIdControl(EVENT_ACTIVITY_CTRL_GET_SET_ID, &m_SavedActivityId);
    }

    explicit TraceLoggingThreadActivityIdSetter(_In_ const GUID* activityId)
        : m_ActivityId(*activityId)
        , m_SavedActivityId(*activityId)
    {
        ::EventActivityIdControl(EVENT_ACTIVITY_CTRL_GET_SET_ID, &m_SavedActivityId);
    }

    ~TraceLoggingThreadActivityIdSetter()
    {
        ::EventActivityIdControl(EVENT_ACTIVITY_CTRL_GET_SET_ID, &m_SavedActivityId);
        _tlg_ASSERT(_tlgGuidEqual(m_ActivityId, m_SavedActivityId), "TraceLoggingThreadActivityIdSetter current id does not match set id!");
    }
};

#endif // WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)

#pragma endregion

#endif // RC_INVOKED
