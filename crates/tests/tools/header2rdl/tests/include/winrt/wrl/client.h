//
// Copyright (C) Microsoft Corporation
// All rights reserved.
//
// Code in details namespace is for internal usage within the library code
//

#ifndef _WRL_CLIENT_H_
#define _WRL_CLIENT_H_

#ifdef _MSC_VER
#pragma once
#endif  // _MSC_VER

#pragma region includes
#include <stddef.h>
#include <unknwn.h>
#include <weakreference.h>
#include <roapi.h>
#ifdef BUILD_WINDOWS
#include <winrt.h>
#endif

#include <wrl\def.h>
#include <wrl\internal.h>

// Set packing
#include <pshpack8.h>

#pragma endregion

#pragma warning(push)
// 'class' : default constructor could not be generated
#pragma warning(disable: 4510)
// object 'class' can never be instantiated - user-defined constructor required
#pragma warning(disable: 4610)
//'derived class' : destructor could not be generated because a base class destructor is inaccessible
#pragma warning(disable: 4624)
// Warnings 4510 and 4610 are generated because of as<>() helper object
// that don't require any constructors thus disabling those warnings
// Warning 4624 is generated if the object has private destructor and the object cannot be created on the stack
// ComPtr does not require the object to be created on the stack thus disabling the warning

// Legacy artifact: This enabled sealed / final optimizations for classes in release builds, and blocking
// of access to AddRef/Release through ComPtr in checked builds. This guard has been removed, so prefer the C++
// keywords intead.
#if defined(_DEBUG) || defined(DBG)
#define WrlFinal
#define WrlSealed
#else
#define WrlFinal final
#define WrlSealed sealed
#endif

namespace Microsoft {
namespace WRL {

namespace Details
{
// Helper object that makes IUnknown methods private
template <typename T>
class RemoveIUnknownBase : public T
{
private:
    ~RemoveIUnknownBase();

    // STDMETHOD macro implies virtual.
    // ComPtr can be used with any class that implements the 3 methods of IUnknown.
    // ComPtr does not require these methods to be virtual.
    // When ComPtr is used with a class without a virtual table, marking the functions
    // as virtual in this class adds unnecessary overhead.

    // WRL::ComPtr is designed to take over responsibility for managing
    // COM ref-counts, reducing the risk of leaking references to COM objects
    // (implying memory leaks) or of over-releasing (leading to crashes and/or
    // heap corruption).  To maximize the safey of ComPtr-using code, RemoveIUnknown
    // is used to hide from ComPtr clients direct access to the native IUnknown
    // refcounting methods.  However, RemoveIUnknown is only used in checked/debug builds.
    // Its implementation is incompatible with the performance win that
    // comes from marking WRL-implemented runtimeclasses as "final" (or "sealed").
    // That performance win is enabled in production/release builds, while
    // ComPtr's use of RemoveIUnknown is enabled in check/debug builds.
    // This difference in compile-time behavior between production/release and
    // checked/debug builds leads to developer friction for any projects
    // that do production builds more frequently than they do checked/debug builds;
    // code that compiles in production builds fails in checked/debug builds.
    // Experience has shown that ComPtr clients attempt to access
    // IUnknown::QueryInterface far more commonly than they do IUnknown::AddRef
    // or IUnknown::Release.  And that same experience has shown that they tend
    // to use QueryInterface in a safe way, appropriately using a ComPtr to
    // provide the output-paremeter to QueryInteface.  Based on that experience,
    // RemoveIUnknown will no longer block access to QueryInterface.  (It is worth
    // noting, however, that there is never an actual requirement to access
    // QueryInterface directly; any caller could instead use a call
    // to CompPtr::CopyTo(REFIID riid, void** ptr).)
    //
    // HRESULT __stdcall QueryInterface(REFIID riid, _COM_Outptr_ void **ppvObject);
    ULONG __stdcall AddRef();
    ULONG __stdcall Release();
};

template<typename T>
struct RemoveIUnknown
{
    typedef RemoveIUnknownBase<T> ReturnType;
};

template<typename T>
struct RemoveIUnknown<const T>
{
    typedef const RemoveIUnknownBase<T> ReturnType;
};

template <typename T> // T should be the ComPtr<T> or a derived type of it, not just the interface
class ComPtrRefBase
{
public:
    typedef typename T::InterfaceType InterfaceType;

#ifndef __WRL_CLASSIC_COM__
    operator IInspectable**() const throw()
    {
        static_assert(__is_base_of(IInspectable, InterfaceType), "Invalid cast: InterfaceType does not derive from IInspectable");
        return reinterpret_cast<IInspectable**>(ptr_->ReleaseAndGetAddressOf());
    }
#endif

    operator IUnknown**() const throw()
    {
        static_assert(__is_base_of(IUnknown, InterfaceType), "Invalid cast: InterfaceType does not derive from IUnknown");
        return reinterpret_cast<IUnknown**>(ptr_->ReleaseAndGetAddressOf());
    }

protected:
    T* ptr_;
};

template <typename T>
class ComPtrRef : public Details::ComPtrRefBase<T> // T should be the ComPtr<T> or a derived type of it, not just the interface
{
    using Super = Details::ComPtrRefBase<T>;
    using InterfaceType = typename Super::InterfaceType;
public:
    ComPtrRef(_In_opt_ T* ptr) throw()
    {
         this->ptr_ = ptr;
    }

    // Conversion operators
    operator void**() const throw()
    {
        return reinterpret_cast<void**>(this->ptr_->ReleaseAndGetAddressOf());
    }

    // This is our operator ComPtr<U> (or the latest derived class from ComPtr (e.g. WeakRef))
    operator T*() throw()
    {
        *this->ptr_ = nullptr;
        return this->ptr_;
    }

    // We define operator InterfaceType**() here instead of on ComPtrRefBase<T>, since
    // if InterfaceType is IUnknown or IInspectable, having it on the base will collide.
    operator InterfaceType**() throw()
    {
        return this->ptr_->ReleaseAndGetAddressOf();
    }

    // This is used for IID_PPV_ARGS in order to do __uuidof(**(ppType)).
    // It does not need to clear  ptr_ at this point, it is done at IID_PPV_ARGS_Helper(ComPtrRef&) later in this file.
    InterfaceType* operator *() throw()
    {
        return this->ptr_->Get();
    }

    // Explicit functions
    InterfaceType* const * GetAddressOf() const throw()
    {
        return this->ptr_->GetAddressOf();
    }

    InterfaceType** ReleaseAndGetAddressOf() throw()
    {
        return this->ptr_->ReleaseAndGetAddressOf();
    }
};

} // namespace Details

class WeakRef;

#if (NTDDI_VERSION >= NTDDI_WINBLUE)
class AgileRef;

#pragma region Application Family or OneCore Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM)

template<typename T>
HRESULT AsAgile(_In_ T* p, _Out_ AgileRef* pAgile) throw();

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM) */
#pragma endregion

#endif // (NTDDI_VERSION >= NTDDI_WINBLUE)

template<typename T>
HRESULT AsWeak(_In_ T* p, _Out_ WeakRef* pWeak) throw();

template <typename T>
class ComPtr
{
public:
    typedef T InterfaceType;

protected:
    InterfaceType *ptr_;
    template<class U> friend class ComPtr;

    void InternalAddRef() const throw()
    {
        if (ptr_ != nullptr)
        {
            ptr_->AddRef();
        }
    }

    unsigned long InternalRelease() throw()
    {
        unsigned long ref = 0;
        T* temp = ptr_;

        if (temp != nullptr)
        {
            ptr_ = nullptr;
            ref = temp->Release();
        }

        return ref;
    }

public:
#pragma region constructors
    ComPtr() throw() : ptr_(nullptr)
    {
    }

    ComPtr(decltype(__nullptr)) throw() : ptr_(nullptr)
    {
    }

    template<class U>
    ComPtr(_In_opt_ U *other) throw() : ptr_(other)
    {
        InternalAddRef();
    }

    ComPtr(const ComPtr& other) throw() : ptr_(other.ptr_)
    {
        InternalAddRef();
    }

    // copy constructor that allows to instantiate class when U* is convertible to T*
    template<class U>
    ComPtr(const ComPtr<U> &other, typename Details::EnableIf<Details::IsConvertible<U*, T*>::value, void *>::type * = 0) throw() :
        ptr_(other.ptr_)
    {
        InternalAddRef();
    }

    ComPtr(_Inout_ ComPtr &&other) throw() : ptr_(nullptr)
    {
        if (this != reinterpret_cast<ComPtr*>(&reinterpret_cast<unsigned char&>(other)))
        {
            Swap(other);
        }
    }

    // Move constructor that allows instantiation of a class when U* is convertible to T*
    template<class U>
    ComPtr(_Inout_ ComPtr<U>&& other, typename Details::EnableIf<Details::IsConvertible<U*, T*>::value, void *>::type * = 0) throw() :
        ptr_(other.ptr_)
    {
        other.ptr_ = nullptr;
    }
#pragma endregion

#pragma region destructor
    ~ComPtr() throw()
    {
        InternalRelease();
    }
#pragma endregion

#pragma region assignment
    ComPtr& operator=(decltype(__nullptr)) throw()
    {
        InternalRelease();
        return *this;
    }

    ComPtr& operator=(_In_opt_ T *other) throw()
    {
        if (ptr_ != other)
        {
            ComPtr(other).Swap(*this);
        }
        return *this;
    }

    template <typename U>
    ComPtr& operator=(_In_opt_ U *other) throw()
    {
        ComPtr(other).Swap(*this);
        return *this;
    }

    ComPtr& operator=(const ComPtr &other) throw()
    {
        if (ptr_ != other.ptr_)
        {
            ComPtr(other).Swap(*this);
        }
        return *this;
    }

    template<class U>
    ComPtr& operator=(const ComPtr<U>& other) throw()
    {
        ComPtr(other).Swap(*this);
        return *this;
    }

    ComPtr& operator=(_Inout_ ComPtr &&other) throw()
    {
        ComPtr(static_cast<ComPtr&&>(other)).Swap(*this);
        return *this;
    }

    template<class U>
    ComPtr& operator=(_Inout_ ComPtr<U>&& other) throw()
    {
        ComPtr(static_cast<ComPtr<U>&&>(other)).Swap(*this);
        return *this;
    }
#pragma endregion

#pragma region modifiers
    void Swap(_Inout_ ComPtr&& r) throw()
    {
        T* tmp = ptr_;
        ptr_ = r.ptr_;
        r.ptr_ = tmp;
    }

    void Swap(_Inout_ ComPtr& r) throw()
    {
        T* tmp = ptr_;
        ptr_ = r.ptr_;
        r.ptr_ = tmp;
    }
#pragma endregion

    operator Details::BoolType() const throw()
    {
        return Get() != nullptr ? &Details::BoolStruct::Member : nullptr;
    }

    T* Get() const throw()
    {
        return ptr_;
    }

#if (defined(_DEBUG) || defined(DBG)) && defined(__REMOVE_IUNKNOWN_METHODS__)
    typename Details::RemoveIUnknown<InterfaceType>::ReturnType* operator->() const throw()
    {
        return static_cast<typename Details::RemoveIUnknown<InterfaceType>::ReturnType*>(ptr_);
    }
#else
    InterfaceType* operator->() const throw()
    {
        return ptr_;
    }
#endif

    Details::ComPtrRef<ComPtr<T>> operator&() throw()
    {
        return Details::ComPtrRef<ComPtr<T>>(this);
    }

    const Details::ComPtrRef<const ComPtr<T>> operator&() const throw()
    {
        return Details::ComPtrRef<const ComPtr<T>>(this);
    }

    T* const* GetAddressOf() const throw()
    {
        return &ptr_;
    }

    T** GetAddressOf() throw()
    {
        return &ptr_;
    }

    T** ReleaseAndGetAddressOf() throw()
    {
        InternalRelease();
        return &ptr_;
    }

    T* Detach() throw()
    {
        T* ptr = ptr_;
        ptr_ = nullptr;
        return ptr;
    }

    void Attach(_In_opt_ InterfaceType* other) throw()
    {
        if (ptr_ != nullptr)
        {
            auto ref = ptr_->Release();
            (void)ref;
            // Attaching to the same object only works if duplicate references are being coalesced. Otherwise
            // re-attaching will cause the pointer to be released and may cause a crash on a subsequent dereference.
            __WRL_ASSERT__(ref != 0 || ptr_ != other);
        }

        ptr_ = other;
    }

    unsigned long Reset()
    {
        return InternalRelease();
    }

    // Previously, unsafe behavior could be triggered when 'this' is ComPtr<IInspectable> or ComPtr<IUnknown> and CopyTo is used to copy to another type U.
    // The user will use operator& to convert the destination into a ComPtrRef, which can then implicit cast to IInspectable** and IUnknown**.
    // If this overload of CopyTo is not present, it will implicitly cast to IInspectable or IUnknown and match CopyTo(InterfaceType**) instead.
    // A valid polymoprhic downcast requires run-time type checking via QueryInterface, so CopyTo(InterfaceType**) will break type safety.
    // This overload matches ComPtrRef before the implicit cast takes place, preventing the unsafe downcast.
    template <typename U>
    HRESULT CopyTo(Details::ComPtrRef<ComPtr<U>> ptr, typename Details::EnableIf<
      ((Details::IsSame<T, IInspectable>::value && !Details::IsSame<U, IUnknown>::value) || (Details::IsSame<T, IUnknown>::value))
      && !Details::IsSame<U*, T*>::value, void *>::type * = 0) const throw()
    {
        return ptr_->QueryInterface(__uuidof(U), ptr);
    }

    HRESULT CopyTo(_Outptr_result_maybenull_ InterfaceType** ptr) const throw()
    {
        InternalAddRef();
        *ptr = ptr_;
        return S_OK;
    }

    HRESULT CopyTo(REFIID riid, _Outptr_result_nullonfailure_ void** ptr) const throw()
    {
        return ptr_->QueryInterface(riid, ptr);
    }

    template<typename U>
    HRESULT CopyTo(_Outptr_result_nullonfailure_ U** ptr) const throw()
    {
        return ptr_->QueryInterface(__uuidof(U), reinterpret_cast<void**>(ptr));
    }

    // query for U interface
    template<typename U>
    HRESULT As(_Inout_ Details::ComPtrRef<ComPtr<U>> p) const throw()
    {
        return ptr_->QueryInterface(__uuidof(U), p);
    }

    // query for U interface
    template<typename U>
    HRESULT As(_Out_ ComPtr<U>* p) const throw()
    {
        return ptr_->QueryInterface(__uuidof(U), reinterpret_cast<void**>(p->ReleaseAndGetAddressOf()));
    }

    // query for riid interface and return as IUnknown
    HRESULT AsIID(REFIID riid, _Out_ ComPtr<IUnknown>* p) const throw()
    {
        return ptr_->QueryInterface(riid, reinterpret_cast<void**>(p->ReleaseAndGetAddressOf()));
    }

    HRESULT AsWeak(_Out_ WeakRef* pWeakRef) const throw()
    {
        return ::Microsoft::WRL::AsWeak(ptr_, pWeakRef);
    }

#pragma region Application Family or OneCore Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM)

#if (NTDDI_VERSION >= NTDDI_WINBLUE)

    HRESULT AsAgile(_Out_ AgileRef* pAgile) const throw()
    {
        return ::Microsoft::WRL::AsAgile(ptr_, pAgile);
    }

#endif // (NTDDI_VERSION >= NTDDI_WINBLUE)

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM) */
#pragma endregion

};    // ComPtr

class WeakRef : public ComPtr<IWeakReference>
{
private:
    void operator->();
protected:
    HRESULT InternalResolve(REFIID riid, _Outptr_result_maybenull_ IInspectable** inspectable) const throw()
    {
        *inspectable = nullptr;

        if (ptr_ == nullptr)
        {
            // For compatibility with original release, treat nullptr as successful
            return S_OK;
        }

        return ptr_->Resolve(riid, inspectable);
    }
public:
    Details::ComPtrRef<WeakRef> operator&() throw()
    {
        return Details::ComPtrRef<WeakRef>(this);
    }

    const Details::ComPtrRef<const WeakRef> operator&() const throw()
    {
        return Details::ComPtrRef<const WeakRef>(this);
    }

    WeakRef() throw() : ComPtr(nullptr)
    {
    }

    WeakRef(decltype(__nullptr)) throw() : ComPtr(nullptr)
    {
    }

    WeakRef(_In_opt_ IWeakReference* ptr) throw() : ComPtr(ptr)
    {
    }

    WeakRef(const ComPtr<IWeakReference>& ptr) throw() : ComPtr(ptr)
    {
    }

    WeakRef(const WeakRef& ptr) throw() : ComPtr(ptr)
    {
    }

    WeakRef(_Inout_ WeakRef&& ptr) throw() : ComPtr(static_cast<ComPtr<IWeakReference>&&>(ptr))
    {
    }

    ~WeakRef() throw()
    {
    }

    // Resolve U interface
    template<typename U>
    HRESULT As(_Inout_ Details::ComPtrRef<ComPtr<U>> ptr) const throw()
    {
        static_assert(!Details::IsSame<IWeakReference, U>::value, "IWeakReference cannot resolve IWeakReference object.");
        static_assert(__is_base_of(IInspectable, U), "WeakRef::As() can only be used on types derived from IInspectable");

        return InternalResolve(__uuidof(U), ptr);
    }

    template<typename U>
    HRESULT As(_Out_ ComPtr<U>* ptr) const throw()
    {
        static_assert(!Details::IsSame<IWeakReference, U>::value, "IWeakReference cannot resolve IWeakReference object.");
        static_assert(__is_base_of(IInspectable, U), "WeakRef::As() can only be used on types derived from IInspectable");

        return InternalResolve(__uuidof(U), ptr->ReleaseAndGetAddressOf());
    }

    HRESULT AsIID(REFIID riid, _Out_ ComPtr<IInspectable>* ptr) const throw()
    {
        __WRL_ASSERT__(riid != __uuidof(IWeakReference));

        return InternalResolve(riid, ptr->ReleaseAndGetAddressOf());
    }

    HRESULT CopyTo(REFIID riid, _Outptr_result_maybenull_ IInspectable** ptr) const throw()
    {
        __WRL_ASSERT__(riid != __uuidof(IWeakReference));

        return InternalResolve(riid, ptr);
    }

    template<typename U>
    HRESULT CopyTo(_Outptr_result_maybenull_ U** ptr) const throw()
    {
        static_assert(__is_base_of(IInspectable, U), "WeakRef::CopyTo() can only be used on types derived from IInspectable");

        return InternalResolve(__uuidof(U), reinterpret_cast<IInspectable**>(ptr));
    }

    HRESULT CopyTo(_Outptr_result_maybenull_ IWeakReference** ptr) const throw()
    {
        InternalAddRef();
        *ptr = ptr_;
        return S_OK;
    }

    WeakRef& operator=(_In_ const WeakRef &other) throw()
    {
        ComPtr::operator=(static_cast<const ComPtr&>(other));
        return *this;
    }

    WeakRef& operator=(_Inout_ WeakRef &&other) throw()
    {
        ComPtr::operator=(static_cast<ComPtr&&>(other));
        return *this;
    }
};

template<typename T>
HRESULT AsWeak(_In_ T* p, _Out_ WeakRef* pWeak) throw()
{
    static_assert(!Details::IsSame<IWeakReference,T>::value, "Cannot get IWeakReference object to IWeakReference.");
    ComPtr<IWeakReferenceSource> refSource;

    HRESULT hr = p->QueryInterface(IID_PPV_ARGS(refSource.GetAddressOf()));
    if (FAILED(hr))
    {
        return hr;
    }

    ComPtr<IWeakReference> weakref;
    hr = refSource->GetWeakReference(weakref.GetAddressOf());
    if (FAILED(hr))
    {
        return hr;
    }

    *pWeak = WeakRef(weakref);
    return S_OK;
}

#if (NTDDI_VERSION >= NTDDI_WINBLUE)
class AgileRef : public ComPtr<IAgileReference>
{
protected:
    HRESULT InternalResolve(REFIID riid, _Outptr_result_maybenull_ void** ptr) const
    {
        *ptr = nullptr;

        if (ptr_ == nullptr)
        {
            // For compatibility with original release, treat nullptr as successful
            return S_OK;
        }

        return ptr_->Resolve(riid, ptr);
    }
public:
    Details::ComPtrRef<AgileRef> operator&() throw()
    {
        return Details::ComPtrRef<AgileRef>(this);
    }

    const Details::ComPtrRef<const AgileRef> operator&() const throw()
    {
        return Details::ComPtrRef<const AgileRef>(this);
    }

    AgileRef() throw() : ComPtr(nullptr)
    {
    }

    AgileRef(decltype(__nullptr)) throw() : ComPtr(nullptr)
    {
    }

    AgileRef(_In_opt_ IAgileReference* ptr) throw() : ComPtr(ptr)
    {
    }

    AgileRef(const ComPtr<IAgileReference>& ptr) throw() : ComPtr(ptr)
    {
    }

    AgileRef(const AgileRef& ptr) throw() : ComPtr(ptr)
    {
    }

    AgileRef(_Inout_ AgileRef&& ptr) throw() : ComPtr(static_cast<ComPtr<IAgileReference>&&>(ptr))
    {
    }

    ~AgileRef() throw()
    {
    }

    template<typename U>
    HRESULT As(_Inout_ Details::ComPtrRef<ComPtr<U>> ptr) const throw()
    {
        static_assert(!Details::IsSame<IAgileReference, U>::value, "IAgileReference cannot resolve IAgileReference object.");
        static_assert(__is_base_of(IUnknown, U), "AgileRef::As() can only be used on types derived from IUnknown");

        return InternalResolve(__uuidof(U), ptr);
    }

    template<typename U>
    HRESULT As(_Out_ ComPtr<U>* ptr) const throw()
    {
        static_assert(!Details::IsSame<IAgileReference, U>::value, "IAgileReference cannot resolve IAgileReference object.");
        static_assert(__is_base_of(IUnknown, U), "AgileRef::As() can only be used on types derived from IUnknown");

        return InternalResolve(__uuidof(U), reinterpret_cast<void**>(ptr->ReleaseAndGetAddressOf()));
    }

    HRESULT AsIID(REFIID riid, _Out_ ComPtr<IUnknown>* ptr) const throw()
    {
        __WRL_ASSERT__(riid != __uuidof(IAgileReference));

        return InternalResolve(riid, reinterpret_cast<void**>(ptr->ReleaseAndGetAddressOf()));
    }

    HRESULT CopyTo(REFIID riid, _Outptr_result_maybenull_ IUnknown** ptr) const throw()
    {
        __WRL_ASSERT__(riid != __uuidof(IAgileReference));

        return InternalResolve(riid, reinterpret_cast<void**>(ptr));
    }

    template<typename U>
    HRESULT CopyTo(_Outptr_result_maybenull_ U** ptr) const throw()
    {
        static_assert(__is_base_of(IUnknown, U), "AgileRef::As() can only be used on types derived from IUnknown");

        return InternalResolve(__uuidof(U), reinterpret_cast<void**>(ptr));
    }

    HRESULT CopyTo(_Outptr_result_maybenull_ IAgileReference** ptr) const throw()
    {
        InternalAddRef();
        *ptr = ptr_;
        return S_OK;
    }

    AgileRef& operator=(_In_ const AgileRef &other) throw()
    {
        ComPtr::operator=(static_cast<const ComPtr&>(other));
        return *this;
    }

    AgileRef& operator=(_Inout_ AgileRef &&other) throw()
    {
        ComPtr::operator=(static_cast<ComPtr&&>(other));
        return *this;
    }

    void operator->() = delete;
};

#pragma region Application Family or OneCore Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM)

template<typename T>
HRESULT AsAgile(_In_opt_ T* p, _Inout_ Microsoft::WRL::AgileRef* pAgile) throw()
{
    static_assert(!Details::IsSame<IAgileReference, T>::value, "Cannot get IAgileReference object to IAgileReference.");

    HRESULT hr = S_OK;
    if (p)
    {
        hr = RoGetAgileReference(AGILEREFERENCE_DEFAULT, __uuidof(T), p, pAgile->ReleaseAndGetAddressOf());
    }
    else
    {
        *pAgile = nullptr;
    }

    return hr;
}

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM) */
#pragma endregion

#endif // (NTDDI_VERSION >= NTDDI_WINBLUE)

// Comparison operators - don't compare COM object identity
template<class T, class U>
bool operator==(const ComPtr<T>& a, const ComPtr<U>& b) throw()
{
    static_assert(__is_base_of(T, U) || __is_base_of(U, T), "'T' and 'U' pointers must be comparable");
    return a.Get() == b.Get();
}

template<class T>
bool operator==(const ComPtr<T>& a, decltype(__nullptr)) throw()
{
    return a.Get() == nullptr;
}

template<class T>
bool operator==(decltype(__nullptr), const ComPtr<T>& a) throw()
{
    return a.Get() == nullptr;
}

template<class T, class U>
bool operator!=(const ComPtr<T>& a, const ComPtr<U>& b) throw()
{
    static_assert(__is_base_of(T, U) || __is_base_of(U, T), "'T' and 'U' pointers must be comparable");
    return a.Get() != b.Get();
}

template<class T>
bool operator!=(const ComPtr<T>& a, decltype(__nullptr)) throw()
{
    return a.Get() != nullptr;
}

template<class T>
bool operator!=(decltype(__nullptr), const ComPtr<T>& a) throw()
{
    return a.Get() != nullptr;
}

template<class T, class U>
bool operator<(const ComPtr<T>& a, const ComPtr<U>& b) throw()
{
    static_assert(__is_base_of(T, U) || __is_base_of(U, T), "'T' and 'U' pointers must be comparable");
    return a.Get() < b.Get();
}

//// Details::ComPtrRef comparisons
template<class T, class U>
bool operator==(const Details::ComPtrRef<ComPtr<T>>& a, const Details::ComPtrRef<ComPtr<U>>& b) throw()
{
    static_assert(__is_base_of(T, U) || __is_base_of(U, T), "'T' and 'U' pointers must be comparable");
    return a.GetAddressOf() == b.GetAddressOf();
}

template<class T>
bool operator==(const Details::ComPtrRef<ComPtr<T>>& a, decltype(__nullptr)) throw()
{
    return a.GetAddressOf() == nullptr;
}

template<class T>
bool operator==(decltype(__nullptr), const Details::ComPtrRef<ComPtr<T>>& a) throw()
{
    return a.GetAddressOf() == nullptr;
}

template<class T>
bool operator==(const Details::ComPtrRef<ComPtr<T>>& a, void* b) throw()
{
    return a.GetAddressOf() == b;
}

template<class T>
bool operator==(void* b, const Details::ComPtrRef<ComPtr<T>>& a) throw()
{
    return a.GetAddressOf() == b;
}

template<class T, class U>
bool operator!=(const Details::ComPtrRef<ComPtr<T>>& a, const Details::ComPtrRef<ComPtr<U>>& b) throw()
{
    static_assert(__is_base_of(T, U) || __is_base_of(U, T), "'T' and 'U' pointers must be comparable");
    return a.GetAddressOf() != b.GetAddressOf();
}

template<class T>
bool operator!=(const Details::ComPtrRef<ComPtr<T>>& a, decltype(__nullptr)) throw()
{
    return a.GetAddressOf() != nullptr;
}

template<class T>
bool operator!=(decltype(__nullptr), const Details::ComPtrRef<ComPtr<T>>& a) throw()
{
    return a.GetAddressOf() != nullptr;
}

template<class T>
bool operator!=(const Details::ComPtrRef<ComPtr<T>>& a, void* b) throw()
{
    return a.GetAddressOf() != b;
}

template<class T>
bool operator!=(void* b, const Details::ComPtrRef<ComPtr<T>>& a) throw()
{
    return a.GetAddressOf() != b;
}

template<class T, class U>
bool operator<(const Details::ComPtrRef<ComPtr<T>>& a, const Details::ComPtrRef<ComPtr<U>>& b) throw()
{
    static_assert(__is_base_of(T, U) || __is_base_of(U, T), "'T' and 'U' pointers must be comparable");
    return a.GetAddressOf() < b.GetAddressOf();
}

}} // namespace Microsoft::WRL

// Overloaded global function to provide to IID_PPV_ARGS that support Details::ComPtrRef
template<typename T>
void** IID_PPV_ARGS_Helper(_Inout_ ::Microsoft::WRL::Details::ComPtrRef<T> pp) throw()
{
    static_assert(__is_base_of(IUnknown, typename T::InterfaceType), "T has to derive from IUnknown");
    return pp;
}

// Overloaded global function to provide to IID_INS_ARGS that support Details::ComPtrRef
template<typename T>
void** IID_INS_ARGS_Helper(_Inout_ ::Microsoft::WRL::Details::ComPtrRef<T> pp) throw()
{
    static_assert(__is_base_of(IInspectable, typename T::InterfaceType), "T has to derive from IInspectable");
    return pp;
}

namespace Windows
{
    namespace Foundation
    {
        template<typename T>
        inline HRESULT ActivateInstance(
            _In_ HSTRING activatableClassId,
            _Inout_ ::Microsoft::WRL::Details::ComPtrRef<T> instance) throw()
        {
            return ActivateInstance(activatableClassId, instance.ReleaseAndGetAddressOf());
        }

        template<typename T>
        inline HRESULT GetActivationFactory(
            _In_  HSTRING activatableClassId,
            _Inout_ ::Microsoft::WRL::Details::ComPtrRef<T>  factory) throw()
        {
            return GetActivationFactory(activatableClassId, factory.ReleaseAndGetAddressOf());
        }
    }
}

namespace ABI
{
    namespace Windows
    {
        namespace Foundation
        {
            template<typename T>
            inline HRESULT ActivateInstance(
                _In_ HSTRING activatableClassId,
                _Inout_ ::Microsoft::WRL::Details::ComPtrRef<T> instance) throw()
            {
                return ActivateInstance(activatableClassId, instance.ReleaseAndGetAddressOf());
            }

            template<typename T>
            inline HRESULT GetActivationFactory(
                _In_  HSTRING activatableClassId,
                _Inout_ ::Microsoft::WRL::Details::ComPtrRef<T>  factory) throw()
            {
                return GetActivationFactory(activatableClassId, factory.ReleaseAndGetAddressOf());
            }
        }
    }
}
#pragma warning(pop)

// Restore packing
#include <poppack.h>

#endif // _WRL_CLIENT_H_
