//
// Copyright (C) Microsoft Corporation
// All rights reserved.
//
// Code in details namespace is for internal usage within the library code
//

#ifndef _WRL_COREWRAPPERS_H_
#define _WRL_COREWRAPPERS_H_

#ifdef _MSC_VER
#pragma once
#endif  // _MSC_VER

#include <windows.h>
#include <intsafe.h>
#include <winstring.h>
#include <roapi.h>
#ifdef BUILD_WINDOWS
#include <winrt.h>
#endif

#include <wrl\def.h>
#include <wrl\internal.h>

// Set packing
#include <pshpack8.h>

namespace Microsoft {
namespace WRL {

namespace Details {
    // Need a type that isn't assignable from int (see other overloads of Initialize, that accept int as second arg)
    // this is needed as part of EnableIf
    struct Dummy {};
}

namespace Wrappers {

namespace HandleTraits
{
// Handle specializations for implemented RAII wrappers
struct HANDLENullTraits
{
    typedef HANDLE Type;

    inline static bool Close(_In_ Type h) throw()
    {
        return ::CloseHandle(h) != FALSE;
    }

    inline static Type GetInvalidValue() throw()
    {
        return nullptr;
    }
};

struct HANDLETraits
{
    typedef HANDLE Type;

    inline static bool Close(_In_ Type h) throw()
    {
        return ::CloseHandle(h) != FALSE;
    }

    inline static HANDLE GetInvalidValue() throw()
    {
        return INVALID_HANDLE_VALUE;
    }
};

struct FileHandleTraits : HANDLETraits
{
};

struct CriticalSectionTraits
{
    typedef CRITICAL_SECTION* Type;

    inline static Type GetInvalidValue() throw()
    {
        return nullptr;
    }

    _Releases_lock_(*cs)
    inline static void Unlock(_In_ Type cs) throw()
    {
        ::LeaveCriticalSection(cs);
    }
};

struct MutexTraits : HANDLENullTraits
{
    _Releases_lock_(h)
    inline static void Unlock(_In_ Type h) throw()
    {
        if (::ReleaseMutex(h) == FALSE)
        {
            // If we cannot release mutex it indicates
            // bug in somebody code thus we raise an exception
            ::Microsoft::WRL::Details::RaiseException(HRESULT_FROM_WIN32(GetLastError()));
        }
    }
};

struct SemaphoreTraits : HANDLENullTraits
{
    inline static void Unlock(_In_ Type h) throw()
    {
        if (::ReleaseSemaphore(h, 1, NULL) == FALSE)
        {
            // If we cannot release semaphore it indicates
            // bug in somebody code thus we raise an exception
            ::Microsoft::WRL::Details::RaiseException(HRESULT_FROM_WIN32(GetLastError()));
        }
    }
};

struct EventTraits : HANDLENullTraits
{
};

struct SRWLockSharedTraits
{
    typedef SRWLOCK* Type;

    inline static Type GetInvalidValue() throw()
    {
        return nullptr;
    }

    _Releases_shared_lock_(*srwlock)
    inline static void Unlock(_In_ Type srwlock) throw()
    {
        ::ReleaseSRWLockShared(srwlock);
    }
};

struct SRWLockExclusiveTraits
{
    typedef SRWLOCK* Type;

    inline static Type GetInvalidValue() throw()
    {
        return nullptr;
    }

    _Releases_exclusive_lock_(*srwlock)
    inline static void Unlock(_In_ Type srwlock) throw()
    {
        ::ReleaseSRWLockExclusive(srwlock);
    }
};

} // namespace HandleTraits

// Handle object implementation specialized with traits
template <typename HandleTraits>
class HandleT
{
public:
    explicit HandleT(typename HandleTraits::Type h = HandleTraits::GetInvalidValue()) throw() :
        handle_(h)
    {
    }

    HandleT(_Inout_ HandleT&& h) throw() : handle_(h.handle_)
    {
        h.handle_ = HandleTraits::GetInvalidValue();
    }

    HandleT(const HandleT&) = delete;
    HandleT& operator=(const HandleT&) = delete;

    ~HandleT() throw()
    {
        Close();
    }

    HandleT& operator=(_Inout_ HandleT&& h) throw()
    {
        Close();
        handle_ = h.handle_;
        h.handle_ = HandleTraits::GetInvalidValue();
        return *this;
    }

    void Attach(typename HandleTraits::Type h) throw()
    {
        if (h != handle_)
        {
            Close();
            handle_ = h;
        }
    }

    typename HandleTraits::Type Detach() throw()
    {
        typename HandleTraits::Type h = handle_;
        handle_ = HandleTraits::GetInvalidValue();
        return h;
    }

    typename HandleTraits::Type Get() const throw()
    {
        return handle_;
    }

    void Close() throw()
    {
        if (handle_ != HandleTraits::GetInvalidValue())
        {
            bool ret = InternalClose();
            if (!ret)
            {
                // If we cannot close the handle it indicates
                // bug in somebody code thus we raise an exception
                ::Microsoft::WRL::Details::RaiseException(HRESULT_FROM_WIN32(GetLastError()));
            }
            handle_ = HandleTraits::GetInvalidValue();
        }
    }

    bool IsValid() const throw()
    {
        return handle_ != HandleTraits::GetInvalidValue();
    }

    typename HandleTraits::Type* GetAddressOf() throw()
    {
        return &handle_;
    }

    typename HandleTraits::Type* ReleaseAndGetAddressOf() throw()
    {
        Close();
        return &handle_;
    }

    typedef HandleTraits Traits;
protected:
    virtual bool InternalClose() throw()
    {
        return HandleTraits::Close(handle_);
    }

    typename HandleTraits::Type handle_;
};

// HandleT comparison operators
template<class T>
bool operator==(const HandleT<T>& rhs, const HandleT<T>& lhs) throw()
{
    return rhs.Get() == lhs.Get();
}

template<class T>
bool operator==(const typename HandleT<T>::Traits::Type& lhs, const HandleT<T>& rhs) throw()
{
    return lhs == rhs.Get();
}

template<class T>
bool operator==(const HandleT<T>& lhs, const typename HandleT<T>::Traits::Type& rhs) throw()
{
    return lhs.Get() == rhs;
}

template<class T>
bool operator!=(const HandleT<T>& lhs, const HandleT<T>& rhs) throw()
{
    return lhs.Get() != rhs.Get();
}

template<class T>
bool operator!=(const typename HandleT<T>::Traits::Type& lhs, const HandleT<T>& rhs) throw()
{
    return lhs != rhs.Get();
}

template<class T>
bool operator!=(const HandleT<T>& lhs, const typename HandleT<T>::Traits::Type& rhs) throw()
{
    return lhs.Get() != rhs;
}

template<class T>
bool operator<(const HandleT<T>& lhs, const HandleT<T>& rhs) throw()
{
    return lhs.Get() < rhs.Get();
}

typedef HandleT<HandleTraits::FileHandleTraits>          FileHandle;


// Forward declarations
class CriticalSection;
class Mutex;
class Semaphore;
class SRWLock;

namespace Details {

#pragma warning(push)
// Disable unheld lock warning for RIAA objects
#pragma warning(disable: 26165)

// Shared lock object for CriticalSection
class SyncLockCriticalSection
{
public:
    SyncLockCriticalSection(_Inout_ SyncLockCriticalSection&& other) throw() : sync_(other.sync_)
    {
        other.sync_ = HandleTraits::CriticalSectionTraits::GetInvalidValue();
    }

    SyncLockCriticalSection(const SyncLockCriticalSection&) = delete;
    SyncLockCriticalSection& operator=(const SyncLockCriticalSection&) = delete;

     _Releases_lock_(*sync_)
    ~SyncLockCriticalSection() throw()
    {
        InternalUnlock();
    }

    _Releases_lock_(*sync_)
    void Unlock() throw()
    {
        __WRL_ASSERT__(IsLocked());
        InternalUnlock();
    }

    bool IsLocked() const throw()
    {
        return (sync_ != HandleTraits::CriticalSectionTraits::GetInvalidValue());
    }

    friend class Wrappers::CriticalSection;

protected:
    explicit SyncLockCriticalSection(CRITICAL_SECTION* sync = HandleTraits::CriticalSectionTraits::GetInvalidValue()) throw() : sync_(sync)
    {
    }

    CRITICAL_SECTION* sync_;

private:
    _Releases_lock_(*sync_)
    void InternalUnlock() throw()
    {
        if (IsLocked())
        {
            // Instances of this class should be used on the stack
            // and should not be passed across threads.
            // Unlock can fail if it is called from the wrong thread
            // or with an Invalid Handle, both of which are bugs
            // Traits::Unlock should raise an SEH in case it cannot
            // release the lock

            HandleTraits::CriticalSectionTraits::Unlock(sync_);
            sync_ = HandleTraits::CriticalSectionTraits::GetInvalidValue();
        }
    }
};

// Exclusive lock object for SRWLock
class SyncLockExclusive
{
public:
    SyncLockExclusive(_Inout_ SyncLockExclusive&& other) throw() : sync_(other.sync_)
    {
        other.sync_ = HandleTraits::SRWLockExclusiveTraits::GetInvalidValue();
    }

    SyncLockExclusive(const SyncLockExclusive&) = delete;
    SyncLockExclusive& operator=(const SyncLockExclusive&) = delete;

    _Releases_exclusive_lock_(*sync_)
    ~SyncLockExclusive() throw()
    {
        InternalUnlock();
    }

    _Releases_exclusive_lock_(*sync_)
    void Unlock() throw()
    {
        __WRL_ASSERT__(IsLocked());
        InternalUnlock();
    }

    bool IsLocked() const throw()
    {
        return (sync_ != HandleTraits::SRWLockExclusiveTraits::GetInvalidValue());
    }

    friend class Wrappers::SRWLock;

protected:
    explicit SyncLockExclusive(SRWLOCK* sync = HandleTraits::SRWLockExclusiveTraits::GetInvalidValue()) throw() : sync_(sync)
    {

    }

    SRWLOCK* sync_;

private:
    _Releases_exclusive_lock_(*sync_)
    void InternalUnlock() throw()
    {
        if(IsLocked())
        {
            HandleTraits::SRWLockExclusiveTraits::Unlock(sync_);
            sync_ = HandleTraits::SRWLockExclusiveTraits::GetInvalidValue();;
        }
    }
};

// Shared lock object for SRWLock
class SyncLockShared
{
public:
    SyncLockShared(_Inout_ SyncLockShared&& other) throw() : sync_(other.sync_)
    {
        other.sync_ = HandleTraits::SRWLockSharedTraits::GetInvalidValue();
    }

    SyncLockShared(const SyncLockShared&) = delete;
    SyncLockShared& operator=(const SyncLockShared&) = delete;

    _Releases_shared_lock_(*sync_)
    ~SyncLockShared() throw()
    {
        InternalUnlock();
    }

    _Releases_shared_lock_(*sync_)
    void Unlock() throw()
    {
        __WRL_ASSERT__(IsLocked());
        InternalUnlock();
    }

    bool IsLocked() const throw()
    {
        return (sync_ != HandleTraits::SRWLockSharedTraits::GetInvalidValue());
    }

    friend class Wrappers::SRWLock;

protected:
    explicit SyncLockShared(SRWLOCK* sync = HandleTraits::SRWLockSharedTraits::GetInvalidValue()) throw() : sync_(sync)
    {

    }

    SRWLOCK* sync_;

private:
    _Releases_shared_lock_(*sync_)
    void InternalUnlock() throw()
    {
        if(IsLocked())
        {
            HandleTraits::SRWLockSharedTraits::Unlock(sync_);
            sync_ = HandleTraits::SRWLockSharedTraits::GetInvalidValue();
        }
    }
};

#pragma warning(pop)

#pragma warning(push)
// Missing annotation _Releases_*
// Possibly releasing unheld lock
#pragma warning(disable: 26165 26167 26135)
// Lock object implemenatation customzed with traits
template <typename SyncTraits>
class SyncLockWithStatusT
{
public:
    SyncLockWithStatusT(_Inout_ SyncLockWithStatusT&& other) throw()
        : status_(other.status_), sync_(other.sync_)
    {
        other.sync_ = SyncTraits::GetInvalidValue();
    }

    SyncLockWithStatusT(const SyncLockWithStatusT&) = delete;
    SyncLockWithStatusT& operator=(const SyncLockWithStatusT&) = delete;

    ~SyncLockWithStatusT() throw()
    {
        InternalUnlock();
    }

    void Unlock() throw()
    {
        __WRL_ASSERT__(IsLocked());
        InternalUnlock();
    }

    bool IsLocked() const throw()
    {
        return sync_ != SyncTraits::GetInvalidValue() && (status_ == 0 || status_ == WAIT_ABANDONED);
    }
    // status value 0 indicates success
    DWORD GetStatus() const throw()
    {
        return status_;
    }

    friend class Wrappers::Mutex;
    friend class Wrappers::Semaphore;
protected:
    explicit SyncLockWithStatusT(typename SyncTraits::Type sync, DWORD status) throw()
        : status_(status), sync_(sync)
    {
    }

    DWORD status_;
    typename SyncTraits::Type sync_;
private:
    void InternalUnlock() throw()
    {
        if (IsLocked())
        {
            // Instances of this class should be used on the stack
            // and should not be passed across threads.
            // Unlock can fail if it is called from the wrong thread
            // or with an Invalid Handle, both of which are bugs
            // Traits::Unlock should raise an SEH in case it cannot
            // release the lock

// Cannot use _Analysis_assume_lock_held_(sync)
// because template instantiations have differing
// levels of indirection to the lock
#pragma warning(suppress:26110)
            SyncTraits::Unlock(sync_);
            sync_ = SyncTraits::GetInvalidValue();
        }
    }
};

#pragma warning(pop)

} // namespace Details

// Critical section implementation
class CriticalSection
{
public:
    typedef Details::SyncLockCriticalSection SyncLock;

    CriticalSection() throw() : CriticalSection(0) 
    {
    }

    explicit CriticalSection(ULONG spincount) throw()
    {
        ::InitializeCriticalSectionEx(&cs_, spincount, 0);
    }

    CriticalSection(const CriticalSection&) = delete;
    CriticalSection& operator=(const CriticalSection&) = delete;

    ~CriticalSection() throw()
    {
        ::DeleteCriticalSection(&cs_);
    }

    _Acquires_lock_(*return.sync_)
    _Post_same_lock_(*return.sync_, cs_)
    SyncLock Lock() throw()
    {
        return Lock(&cs_);
    }

    _Acquires_lock_(*return.sync_)
    _Post_same_lock_(*return.sync_, *cs)
    static SyncLock Lock(_In_ CRITICAL_SECTION* cs) throw()
    {
        ::EnterCriticalSection(cs);
        return SyncLock(cs);
    }

    _Acquires_lock_(*return.sync_)
    _Post_same_lock_(*return.sync_, cs_)
    SyncLock TryLock() throw()
    {
        return TryLock(&cs_);
    }

    _Acquires_lock_(*return.sync_)
    _Post_same_lock_(*return.sync_, *cs)
    static SyncLock TryLock(_In_ CRITICAL_SECTION* cs) throw()
    {
        bool acquired = !!::TryEnterCriticalSection(cs);
        _Analysis_assume_lock_held_(*cs);
        return SyncLock((acquired) ? cs : nullptr);
    }

    bool IsValid() const throw()
    {
        return true;
    }
protected:
    CRITICAL_SECTION cs_;
};

// Mutex handle implementation
class Mutex : public HandleT<HandleTraits::MutexTraits>
{
public:
    typedef Details::SyncLockWithStatusT<HandleTraits::MutexTraits> SyncLock;

    explicit Mutex(HANDLE h) throw() : HandleT(h)
    {
    }

    Mutex(_Inout_ Mutex&& h) throw() : HandleT(::Microsoft::WRL::Details::Move(h))
    {
    }

    Mutex& operator=(_Inout_ Mutex&& h) throw()
    {
        *static_cast<HandleT*>(this) = ::Microsoft::WRL::Details::Move(h);
        return *this;
    }

    SyncLock Lock(DWORD milliseconds = INFINITE) throw()
    {
        return Lock(Get(), milliseconds);
    }

    static SyncLock Lock(HANDLE h, DWORD milliseconds = INFINITE) throw()
    {
        DWORD const status = ::WaitForSingleObjectEx(h, milliseconds, FALSE);
        return SyncLock(h, status == WAIT_OBJECT_0 ? 0 : status);
    }
private:
    void Close();
    HANDLE Detach();
    void Attach(HANDLE);
    HANDLE* GetAddressOf();
    HANDLE* ReleaseAndGetAddressOf();
};

// Semaphore handle implementation
class Semaphore : public HandleT<HandleTraits::SemaphoreTraits>
{
public:
    typedef Details::SyncLockWithStatusT<HandleTraits::SemaphoreTraits> SyncLock;

    explicit Semaphore(HANDLE h) throw() : HandleT(h)
    {
    }

    Semaphore(_Inout_ Semaphore&& h) throw() : HandleT(::Microsoft::WRL::Details::Move(h))
    {
    }

    Semaphore& operator=(_Inout_ Semaphore&& h) throw()
    {
        *static_cast<HandleT*>(this) = ::Microsoft::WRL::Details::Move(h);
        return *this;
    }

    SyncLock Lock(DWORD milliseconds = INFINITE) throw()
    {
        return Lock(Get(), milliseconds);
    }

    static SyncLock Lock(HANDLE h, DWORD milliseconds = INFINITE) throw()
    {
        DWORD const status = ::WaitForSingleObjectEx(h, milliseconds, FALSE);
        return SyncLock(h, status == WAIT_OBJECT_0 ? 0 : status);
    }
private:
    void Close();
    HANDLE Detach();
    void Attach(HANDLE);
    HANDLE* GetAddressOf();
    HANDLE* ReleaseAndGetAddressOf();
};

// Event handle implementation
class Event : public HandleT<HandleTraits::EventTraits>
{
public:
    Event() throw() : Event(HandleT::Traits::GetInvalidValue())
    {
    }

    explicit Event(HANDLE h) throw() : HandleT(h)
    {
    }

    Event(_Inout_ Event&& h) throw() : HandleT(::Microsoft::WRL::Details::Move(h))
    {
    }

    Event& operator=(_Inout_ Event&& h) throw()
    {
        *static_cast<HandleT*>(this) = ::Microsoft::WRL::Details::Move(h);
        return *this;
    }
};

// SRW lock implementation
class SRWLock
{
public:
    typedef Details::SyncLockExclusive SyncLockExclusive;
    typedef Details::SyncLockShared SyncLockShared;

    SRWLock() throw()
    {
        ::InitializeSRWLock(&SRWLock_);
    }

    SRWLock(const SRWLock&) = delete;
    SRWLock& operator=(const SRWLock&) = delete;

    ~SRWLock() throw()
    {
    }

    _Acquires_exclusive_lock_(*return.sync_)
    _Post_same_lock_(*return.sync_, SRWLock_)
    SyncLockExclusive LockExclusive() throw()
    {
        return LockExclusive(&SRWLock_);
    }

    _Acquires_exclusive_lock_(*return.sync_)
    _Post_same_lock_(*return.sync_, *lock)
    static SyncLockExclusive LockExclusive(_In_ SRWLOCK* lock) throw()
    {
        ::AcquireSRWLockExclusive(lock);
        return SyncLockExclusive(lock);
    }

    _Acquires_exclusive_lock_(*return.sync_)
    _Post_same_lock_(*return.sync_, SRWLock_)
    SyncLockExclusive TryLockExclusive() throw()
    {
        return TryLockExclusive(&SRWLock_);
    }

    _Acquires_exclusive_lock_(*return.sync_)
    _Post_same_lock_(*return.sync_, *lock)
    static SyncLockExclusive TryLockExclusive(_In_ SRWLOCK* lock) throw()
    {
        bool acquired = !!::TryAcquireSRWLockExclusive(lock);
        _Analysis_assume_lock_held_(*lock);
        return SyncLockExclusive((acquired) ? lock : nullptr);
    }

    _Acquires_shared_lock_(*return.sync_)
    _Post_same_lock_(*return.sync_, SRWLock_)
    SyncLockShared LockShared() throw()
    {
        return LockShared(&SRWLock_);
    }

    _Acquires_shared_lock_(*return.sync_)
    _Post_same_lock_(*return.sync_, *lock)
    static SyncLockShared LockShared(_In_ SRWLOCK* lock) throw()
    {
        ::AcquireSRWLockShared(lock);
        return SyncLockShared(lock);
    }

    _Acquires_shared_lock_(*return.sync_)
    _Post_same_lock_(*return.sync_, SRWLock_)
    SyncLockShared TryLockShared() throw()
    {
        return TryLockShared(&SRWLock_);
    }

    _Acquires_shared_lock_(*return.sync_)
    _Post_same_lock_(*return.sync_, *lock)
    static SyncLockShared TryLockShared(_In_ SRWLOCK* lock) throw()
    {
        bool acquired = !!::TryAcquireSRWLockShared(lock);
        _Analysis_assume_lock_held_(*lock);
        return SyncLockShared((acquired) ? lock : nullptr);
    }

protected:
    SRWLOCK SRWLock_;
};

#pragma region Application Family or OneCore Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM)

class HStringReference;

class HString
{
public:
    HString() throw() : hstr_(nullptr)
    {
    }

    HString(_Inout_ HString&& other) throw() : hstr_(other.hstr_)
    {
        other.hstr_ = nullptr;
    }

    HString(_In_ const HString&) = delete;
    HString& operator=(_In_ const HString&) = delete;

    // HSTRING operator
    operator HSTRING() const throw()
    {
        return hstr_;
    }

    ~HString() throw()
    {
        Release();
    }

    HString& operator=(_Inout_ HString&& other) throw()
    {
        Release();
        hstr_ = other.hstr_;
        other.hstr_ = nullptr;
        return *this;
    }

    // Initialize this string from a source string. A copy is made in this call.
    // The str parameter doesn't need to be null terminated, and it may have embedded NUL characters.
    HRESULT Set(_In_reads_opt_(len) const wchar_t* str, unsigned int len) throw()
    {
        Release();
        return ::WindowsCreateString(str, len, &hstr_);
    }

    // Initialize the string from a const array of wchar_t.  A copy is made in this call.
    // The primary scenario here is the creation of a buffer from a string literal.  Because the size is
    // known, there is no need for length to be an explicit parameter.
    template <size_t sizeDest>
    HRESULT Set(const wchar_t (&str)[sizeDest]) throw()
    {
        static_assert(static_cast<size_t>(static_cast<UINT32>(sizeDest-1)) == sizeDest - 1, "String length underflow or overflow");

        return Set(str, sizeDest - 1);
    }

    // Initialize the string from a non-const array of wchar_t. A copy is made in this call.
    // The input array must include a terminating NULL. This case differs from the
    // one immediately above. This is intended to handle the case where the buffer size is known
    // but the buffer may be oversized. This practice of using a buffer of known size that is
    // guaranteed to be larger than necessary is a common optimization. This overload of initialize
    // calls wcslen to get the length of the string and uses the size up to the first null as the length.
    // If the caller desires to use a non-const buffer but is interested in getting support for
    // embedded nulls, then the caller should use the overload that takes a length.
    // Without this overload, the template above would match, and we actually want different behavior.
    template <size_t sizeDest>
    HRESULT Set(_In_z_ wchar_t (&strRef)[sizeDest]) throw()
    {
        const wchar_t* str = static_cast<const wchar_t*>(strRef);
        unsigned int length;
        HRESULT hr = SizeTToUInt32(::wcslen(str), &length);
        if (SUCCEEDED(hr))
        {
            hr = Set(str, length);
        }

        return hr;
    }

    // Initialize this string from a source string. A copy is made in this call.  The input string must have a terminating NULL.
    // The EnableIf ensures that this overload is only for type convertible to const wchar_t*.  Without the EnableIf,
    // this overload would be chosen for type that is implicitly convertible to HSTRING, but we want the HSTRING overload
    // to be chosen in such cases. A template version that matches a const wchar_t* is required because a
    // simple non-templated overload with const wchar_t* argument would match before any templated version.
    // And so the templated version above that infers length  would never get called.
    //
    // WARNING: If Initialize is invoked with an extern wchar_t array of unknown size, this overload will be selected,
    // but a compilation error will occur, reporting that the unknown-size array parameter could not be converted to
    // 'const unsigned short (&)[1]' (or 'const wchar_t (&)[1]', depending on compiler switches).  If such a compilation
    // error is encountered, replace the extern declaration in a header file with an extern selectany initialization:
    //    extern const __declspec(selectany) WCHAR SomeString[] = L"SomeText";
    // Alternatively, (but delivering less efficiency,) add an explicit static_cast to const wchar_t* at the call site.
    template <typename T>
    HRESULT Set(
        __in_opt const T &strRef,    // const-ref required in case caller has a type convertible to const wchar_t*, but not copy-able
        typename ::Microsoft::WRL::Details::EnableIf< __is_convertible_to(T,const wchar_t*), ::Microsoft::WRL::Details::Dummy>::type = ::Microsoft::WRL::Details::Dummy()) throw()
    {
        HRESULT hr = S_OK;
        const wchar_t* str = static_cast<PCWSTR>(strRef);
        _Analysis_assume_nullterminated_(static_cast<void*>(const_cast<wchar_t*>(str))); // we trust the caller's conversion gave us a null-terminated string as-advertised.

        if (str != nullptr)
        {
            unsigned int length;
            hr = SizeTToUInt32(::wcslen(str), &length);
            if (SUCCEEDED(hr))
            {
                hr = Set(str, length);
            }
        }
        else
        {
            hr = Set(L"", 0);
        }
        return hr;
    }

    // Initialize this string from an HSTRING. A copy is made in this call.
    HRESULT Set(const HSTRING& str) throw()
    {
        HRESULT hr = S_OK;

        // Guard against the case where this method is called with the argument this->hstr_ when this->hstr_ is a valid HSTRING.
        // If it's called with nullptr, allow ::WindowsDuplicateString() to return E_INVALIDARG no matter the value of this->hstr_
        //if (static_cast<HSTRING>(str) == nullptr || static_cast<HSTRING>(str) != hstr_)
        if (str == nullptr || str != hstr_)
        {
            Release();
            hr = ::WindowsDuplicateString(str, &hstr_);
        }

        return hr;
    }

    void Attach(_In_opt_ HSTRING hstr) throw()
    {
        ::WindowsDeleteString(hstr_);
        hstr_ = hstr;
    }

    HSTRING Detach() throw()
    {
        HSTRING tmp = hstr_;
        hstr_ = nullptr;
        return tmp;
    }

    HSTRING* GetAddressOf() throw()
    {
        Release(); // Note, this is a non-standard implementation, others don't release the resource.
        return &hstr_;
    }

    HSTRING* ReleaseAndGetAddressOf() throw()
    {
        Release();
        return &hstr_;
    }

    HSTRING Get() const throw()
    {
        return hstr_;
    }

    void Release() throw()
    {
        ::WindowsDeleteString(hstr_);
        hstr_ = nullptr;
    }

    bool IsValid() const throw()
    {
        return hstr_ != nullptr;
    }

    UINT32 Length() const throw()
    {
        return ::WindowsGetStringLen(hstr_);
    }

    const wchar_t* GetRawBuffer(_Out_opt_ unsigned int* length) const
    {
        return ::WindowsGetStringRawBuffer(hstr_, length);
    }

    HRESULT CopyTo(_Outptr_result_maybenull_ _Result_nullonfailure_ HSTRING *str) const throw()
    {
        return ::WindowsDuplicateString(hstr_, str);
    }

    // Duplicate from another String.
    HRESULT Duplicate(const HString& other) throw()
    {
        HSTRING local;
        HRESULT hr = ::WindowsDuplicateString(other, &local);
        return ReleaseAndAssignOnSuccess(hr, local, *this);
    }

    bool IsEmpty() const throw()
    {
        return hstr_ == nullptr;
    } 

    HRESULT Concat(const HString& string, HString& newString) const throw()
    {
        HSTRING local;
        HRESULT hr = ::WindowsConcatString(hstr_, string, &local);
        return ReleaseAndAssignOnSuccess(hr, local, newString);
    }

    HRESULT TrimStart(const HString& trimString, HString& newString) const throw()
    {
        HSTRING local;
        HRESULT hr = ::WindowsTrimStringStart(hstr_, trimString, &local);
        return ReleaseAndAssignOnSuccess(hr, local, newString);
    }

    HRESULT TrimEnd(const HString& trimString, HString& newString) const throw()
    {
        HSTRING local;
        HRESULT hr = ::WindowsTrimStringEnd(hstr_, trimString, &local);
        return ReleaseAndAssignOnSuccess(hr, local, newString);
    }

    HRESULT Substring(UINT32 startIndex, HString& newString) const throw()
    {
        HSTRING local;
        HRESULT hr = ::WindowsSubstring(hstr_, startIndex, &local);
        return ReleaseAndAssignOnSuccess(hr, local, newString);
    }

    HRESULT Substring(UINT32 startIndex, UINT32 length, HString& newString) const throw()
    {
        HSTRING local;
        HRESULT hr = ::WindowsSubstringWithSpecifiedLength(hstr_, startIndex, length, &local);
        return ReleaseAndAssignOnSuccess(hr, local, newString);
    }

    HRESULT Replace(const HString& stringReplaced, const HString& stringReplaceWith, HString& newString) const throw()
    {
        HSTRING local;
        HRESULT hr = ::WindowsReplaceString(hstr_, stringReplaced, stringReplaceWith, &local);
        return ReleaseAndAssignOnSuccess(hr, local, newString);
    }

    template<unsigned int sizeDest>
    static HStringReference MakeReference(wchar_t const (&str)[sizeDest]) throw();

    template<unsigned int sizeDest>
    static HStringReference MakeReference(wchar_t const (&str)[sizeDest], unsigned int len) throw();

private:

    // Always returns the passed in HRESULT. If the HRESULT indicates success, frees any previous target string,
    // over-writes and takes ownership of new value
    static HRESULT ReleaseAndAssignOnSuccess(HRESULT hr, HSTRING newValue, HString& target)
    {
        if (SUCCEEDED(hr))
        {
            // InterlockedExchangePointer wouldn't have much value, unless we also modified
            // all readers of *target to insert a ReadBarrier.
            *target.ReleaseAndGetAddressOf() = newValue;
        }

        return hr;
    }

protected:
    HSTRING hstr_;
};

class HStringReference
{
private:
    // const HString& operator
    const HString& AsString() const throw()
    {
        return reinterpret_cast<const HString&>(hstr_);
    }

    void CreateReference(const wchar_t* str, unsigned int bufferLen, unsigned int len)
    {
        __WRL_ASSERT__(len < bufferLen);
        if (len >= bufferLen)
        {
            len = bufferLen - 1;
        }

        HRESULT hr = ::WindowsCreateStringReference(str, len, &header_, &hstr_);
        // Failfast if developers try to create a reference to a non-NUL terminated string
        if (FAILED(hr))
        {
            ::Microsoft::WRL::Details::RaiseException(hr);
        }
    }

    HStringReference() : hstr_(nullptr)
    {
    }

    // Returns 1 + augend
    // Failfast on addition overflow
    static unsigned int AddOne(const unsigned int augend) throw()
    {
        unsigned int result = 0; // init to make compiler happy
        HRESULT hr = UIntAdd(augend, 1, &result);
        if (FAILED(hr))
        {
            ::Microsoft::WRL::Details::RaiseException(hr);
        }
        return result;
    }

public:
    // Constructor which takes an existing string buffer and its length as the parameters.
    // It fills an HSTRING_HEADER struct with the parameter.
    //
    // Warning: The caller must ensure the lifetime of the buffer outlives this
    // object as it does not make a copy of the wide string memory.
    HStringReference(const wchar_t* str, unsigned int len) throw() : hstr_(nullptr)
    {
        unsigned int lenPlusOne = AddOne(len);
        CreateReference(str, lenPlusOne, len);
    }

    // Constructor which takes an existing literal string or const string buffer and infers its length.
    // It fills an HSTRING_HEADER struct with the parameter and length.
    template<unsigned int sizeDest>
    explicit HStringReference(wchar_t const (&str)[sizeDest]) throw() : hstr_(nullptr)
    {
        static_assert(static_cast<size_t>(static_cast<unsigned int>(sizeDest-1)) == sizeDest-1,
            "String length underflow or overflow");

        CreateReference(str, sizeDest, sizeDest - 1);
    }

    // Constructor which takes an non-const string buffer, and uses wcslen() to determine the string's length.
    // It fills an HSTRING_HEADER struct with the parameter and length.
    // For additional information, see the comments for String.Initialize above that takes the same template parameters
    template <size_t sizeDest>
    explicit HStringReference(_In_z_ wchar_t (&strRef)[sizeDest]) throw()
    {
        const wchar_t* str = static_cast<const wchar_t*>(strRef);

        unsigned int length;
        HRESULT hr = SizeTToUInt32(::wcslen(str), &length);
        if (FAILED(hr))
        {
            ::Microsoft::WRL::Details::RaiseException(hr);
        }

        unsigned int lengthPlusOne = AddOne(length);
        CreateReference(str, lengthPlusOne, length);
    }

    // Constructor which takes an existing zero-terminated string buffer.
    // It fills an HSTRING_HEADER struct with the parameter and length.
    template<typename T>
    explicit HStringReference(
        _In_ const T& strRef, // const-ref required in case caller has a type convertible to const wchar_t*, but not copy-able
        typename ::Microsoft::WRL::Details::EnableIf<__is_convertible_to(T, const wchar_t*), ::Microsoft::WRL::Details::Dummy>::type = ::Microsoft::WRL::Details::Dummy()) throw() : hstr_(nullptr)
    {
        const wchar_t* str = static_cast<const wchar_t*>(strRef);
        _Analysis_assume_nullterminated_(static_cast<void*>(const_cast<wchar_t*>(str)));

        unsigned int length;
        HRESULT hr = SizeTToUInt32(::wcslen(str), &length);
        if (FAILED(hr))
        {
            ::Microsoft::WRL::Details::RaiseException(hr);
        }

        unsigned int lengthPlusOne = AddOne(length);
        CreateReference(str, lengthPlusOne, length);
    }

    HStringReference(_In_ const HStringReference& other) throw() : hstr_(nullptr)
    {
        unsigned int length = 0;
        const wchar_t* value = other.GetRawBuffer(&length);

        unsigned int lengthPlusOne = AddOne(length);
        CreateReference(value, lengthPlusOne, length);
    }

    ~HStringReference() throw()
    {
        hstr_ = nullptr;
    }

    HStringReference& operator=(_In_ const HStringReference& other) throw()
    {
        unsigned int length = 0;
        const wchar_t* value = other.GetRawBuffer(&length);

        unsigned int lengthPlusOne = AddOne(length);
        CreateReference(value, lengthPlusOne, length);

        return *this;
    }

    HSTRING Get() const throw()
    {
        return hstr_;
    }

    // const HString& operator
    operator const HString&() const throw()
    {
        return AsString();
    }

    const wchar_t* GetRawBuffer(_Out_opt_ unsigned int* length) const
    {
        return ::WindowsGetStringRawBuffer(hstr_, length);
    }

    HRESULT CopyTo(_Outptr_result_maybenull_ _Result_nullonfailure_ HSTRING *str) const throw()
    {
        return ::WindowsDuplicateString(hstr_, str);
    }

    // Concatenation
    HRESULT Concat(const HString& otherString, HString& newString)  const throw()
    {
        return AsString().Concat(otherString, newString);
    }

    HRESULT TrimStart(const HString& trimString, HString& newString) const throw()
    {
        return AsString().TrimStart(trimString, newString);
    }

    HRESULT TrimEnd(const HString& trimString, HString& newString) const throw()
    {
        return AsString().TrimEnd(trimString, newString);
    }

    HRESULT Substring(UINT32 startIndex, HString& newString) const throw()
    {
        return AsString().Substring(startIndex, newString);
    }

    HRESULT Substring(UINT32 startIndex, UINT32 length, HString& newString) const throw()
    {
        return AsString().Substring(startIndex, length, newString);
    }

    HRESULT Replace(const HString& stringReplaced, const HString& stringReplaceWith, HString& newString) const throw()
    {
        return AsString().Replace(stringReplaced, stringReplaceWith, newString);
    }
    
    // Data Access
    UINT32 Length() const throw()
    {
        return AsString().Length();
    }

    friend class HString;

protected:
    HSTRING_HEADER header_;
    HSTRING hstr_;
};

template<unsigned int sizeDest>
inline HStringReference HString::MakeReference(wchar_t const (&str)[sizeDest]) throw()
{
    static_assert(static_cast<size_t>(static_cast<unsigned int>(sizeDest-1)) == sizeDest-1,
        "String length underflow or overflow");

    HStringReference hstringRef;
    hstringRef.CreateReference(str, sizeDest, sizeDest - 1);
    return hstringRef;
}

template<unsigned int sizeDest>
inline HStringReference HString::MakeReference(wchar_t const (&str)[sizeDest], unsigned int len) throw()
{
    HStringReference hstringRef;
    hstringRef.CreateReference(str, sizeDest, len);
    return hstringRef;
}

namespace Details {

inline INT32 CompareStringOrdinal(HSTRING lhs, HSTRING rhs)
{
    INT32 result = 0;
    HRESULT hr = S_OK;
    // Ignore the HRESULT from the following call
    hr = ::WindowsCompareStringOrdinal(lhs, rhs, &result);
    if (SUCCEEDED(hr))
    {
        return result;
    }
    else
    {
        ::Microsoft::WRL::Details::RaiseException(hr);
    }
}

} // namespace Details

// Specialization for HString
inline bool operator==(const HString& lhs, const HString& rhs) throw()
{
    return Details::CompareStringOrdinal(lhs.Get(), rhs.Get()) == 0;
}

inline bool operator==(const HString& lhs, const HStringReference& rhs) throw()
{
    return Details::CompareStringOrdinal(lhs.Get(), rhs.Get()) == 0;
}

inline bool operator==(const HStringReference& lhs, const HString& rhs) throw()
{
    return Details::CompareStringOrdinal(lhs.Get(), rhs.Get()) == 0;
}

inline bool operator==(const HSTRING& lhs, const HString& rhs) throw()
{
    return Details::CompareStringOrdinal(lhs, rhs.Get()) == 0;
}

inline bool operator==(const HString& lhs, const HSTRING& rhs) throw()
{
    return Details::CompareStringOrdinal(lhs.Get(), rhs) == 0;
}

inline bool operator!=(const HString& lhs, const HString& rhs) throw()
{
    return Details::CompareStringOrdinal(lhs.Get(), rhs.Get()) != 0;
}

inline bool operator!=(const HStringReference& lhs, const HString& rhs) throw()
{
    return Details::CompareStringOrdinal(lhs.Get(), rhs.Get()) != 0;
}

inline bool operator!=(const HString& lhs, const HStringReference& rhs) throw()
{
    return Details::CompareStringOrdinal(lhs.Get(), rhs.Get()) != 0;
}

inline bool operator!=(const HSTRING& lhs, const HString& rhs) throw()
{
    return Details::CompareStringOrdinal(lhs, rhs.Get()) != 0;
}

inline bool operator!=(const HString& lhs, const HSTRING& rhs) throw()
{
    return Details::CompareStringOrdinal(lhs.Get(), rhs) != 0;
}

inline bool operator<(const HString& lhs, const HString& rhs) throw()
{
    return Details::CompareStringOrdinal(lhs.Get(), rhs.Get()) == -1;
}

inline bool operator>(const HString& lhs, const HString& rhs) throw()
{
    return Details::CompareStringOrdinal(lhs.Get(), rhs.Get()) == 1;
}

// Specialization for HStringReference
inline bool operator==(const HStringReference& rhs, const HStringReference& lhs) throw()
{
    return Details::CompareStringOrdinal(lhs.Get(), rhs.Get()) == 0;
}

inline bool operator==(const HSTRING& lhs, const HStringReference& rhs) throw()
{
    return Details::CompareStringOrdinal(lhs, rhs.Get()) == 0;
}

inline bool operator==(const HStringReference& lhs, const HSTRING& rhs) throw()
{
    return Details::CompareStringOrdinal(lhs.Get(), rhs) == 0;
}

inline bool operator!=(const HStringReference& lhs, const HStringReference& rhs) throw()
{
    return Details::CompareStringOrdinal(lhs.Get(), rhs.Get()) != 0;
}

inline bool operator!=(const HSTRING& lhs, const HStringReference& rhs) throw()
{
    return Details::CompareStringOrdinal(lhs, rhs.Get()) != 0;
}

inline bool operator!=(const HStringReference& lhs, const HSTRING& rhs) throw()
{
    return Details::CompareStringOrdinal(lhs.Get(), rhs) != 0;
}

inline bool operator<(const HStringReference& lhs, const HStringReference& rhs) throw()
{
    return Details::CompareStringOrdinal(lhs.Get(), rhs.Get()) == 1;
}

class RoInitializeWrapper
{
    HRESULT _hr;
public:
    RoInitializeWrapper(RO_INIT_TYPE flags)
    {
        _hr = ::Windows::Foundation::Initialize(flags);
    }
    ~RoInitializeWrapper()
    {
        if (SUCCEEDED(_hr))
        {
            ::Windows::Foundation::Uninitialize();
        }
    }
    operator HRESULT()
    {
        return _hr;
    }
};

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM) */
#pragma endregion

} } } // namespace ::Microsoft::WRL::Wrappers

// Restore packing
#include <poppack.h>

#endif // _WRL_COREWRAPPERS_H_
