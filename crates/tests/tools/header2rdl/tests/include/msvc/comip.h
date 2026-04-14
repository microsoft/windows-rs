/***
* comip.h - Native C++ compiler COM support - COM interface pointers header
*
* Copyright (c) Microsoft Corporation. All rights reserved.
*
****/

#pragma once
#ifdef _M_CEE_PURE
#error comip.h header cannot be included under /clr:safe or /clr:pure
#endif

#if !defined(_INC_COMIP)
#define _INC_COMIP

#include <Ole2.h>
#include <comutil.h>
#include <malloc.h>
#include <type_traits>

class _com_error;

void __stdcall _com_issue_error(HRESULT);
struct __declspec(uuid("00000000-0000-0000-c000-000000000046")) IUnknown;

// Provide Interface to IID association
//
template<typename _Interface, const IID* _IID /*= &__uuidof(_Interface)*/>
class _com_IIID {
public:
    typedef _Interface Interface;

    static _Interface* GetInterfacePtr() noexcept
    {
        return nullptr;
    }

    static _Interface& GetInterface() noexcept
    {
        return *GetInterfacePtr();
    }

    static const IID& GetIID() noexcept
    {
        return *_IID;
    }
};

template<typename _IIID> class _com_ptr_t {
public:
    // Declare interface type so that the type may be available outside
    // the scope of this template.
    //
    using ThisIIID = _IIID;
    using Interface = typename _IIID::Interface;

    // When the compiler supports references in template parameters,
    // _CLSID will be changed to a reference.  To avoid conversion
    // difficulties this function should be used to obtain the
    // CLSID.
    //
    static const IID& GetIID() noexcept
    {
        return ThisIIID::GetIID();
    }

    // Constructs a smart-pointer from any other smart pointer.
    //
    template<typename _OtherIID, ::std::enable_if_t<!::std::is_same<_IIID, _OtherIID>::value, int> = 0>
    _com_ptr_t(const _com_ptr_t<_OtherIID>& p)
        : m_pInterface(nullptr)
    {
        HRESULT hr = _QueryInterface(p);

        if (FAILED(hr) && (hr != E_NOINTERFACE)) {
            _com_issue_error(hr);
        }
    }

    // Constructs a smart-pointer from any IUnknown-based interface pointer.
    //
    template<typename _InterfaceType,
        ::std::enable_if_t<!(
            ::std::is_same<_InterfaceType, Interface>::value // call Interface* pInterface
            || ::std::is_same<_InterfaceType, _com_ptr_t>::value // call _com_ptr_t* p
            || ::std::is_same<_InterfaceType, char>::value // call PCSTR str, IUnknown* pOuter, DWORD dwClsContext
            || ::std::is_same<_InterfaceType, const char>::value
            || ::std::is_same<_InterfaceType, wchar_t>::value // call LPCWSTR str, IUnknown* pOuter, DWORD dwClsContext
            || ::std::is_same<_InterfaceType, const wchar_t>::value
        ), int> = 0>
    _com_ptr_t(_InterfaceType* p)
        : m_pInterface(nullptr)
    {
        HRESULT hr = _QueryInterface(p);

        if (FAILED(hr) && (hr != E_NOINTERFACE)) {
            _com_issue_error(hr);
        }
    }

    explicit _com_ptr_t(_com_ptr_t* p)
        : m_pInterface(nullptr)
    {
        if (p == nullptr) {
            _com_issue_error(E_POINTER);
        }
        else {
            m_pInterface = p->m_pInterface;
            AddRef();
        }
    }

    _com_ptr_t() = default;
    _com_ptr_t(decltype(nullptr)) noexcept { }

    _com_ptr_t(int null)
    {
        if (null != 0)
        {
            _com_issue_error(E_POINTER);
        }
    }

    _com_ptr_t(const _com_ptr_t& cp) noexcept
        : m_pInterface(cp.m_pInterface)
    {
        _AddRef();
    }

    _com_ptr_t(_com_ptr_t&& cp) noexcept
        : m_pInterface(cp.m_pInterface)
    {
        cp.m_pInterface = nullptr;
    }

    _com_ptr_t(Interface* pInterface) noexcept
        : m_pInterface(pInterface)
    {
        _AddRef();
    }

    // Copies the pointer. If fAddRef is true, the interface will be AddRef()ed.
    _com_ptr_t(Interface* pInterface, bool fAddRef) noexcept
        : m_pInterface(pInterface)
    {
        if (fAddRef) {
            _AddRef();
        }
    }

    _com_ptr_t(const _variant_t& varSrc)
    {
        HRESULT hr = QueryStdInterfaces(varSrc);

        if (FAILED(hr) && (hr != E_NOINTERFACE)) {
            _com_issue_error(hr);
        }
    }

    // Calls CoCreateClass with the provided CLSID.
    explicit _com_ptr_t(const CLSID& clsid, IUnknown* pOuter = nullptr, DWORD dwClsContext = CLSCTX_ALL)
    {
        HRESULT hr = CreateInstance(clsid, pOuter, dwClsContext);

        if (FAILED(hr) && (hr != E_NOINTERFACE)) {
            _com_issue_error(hr);
        }
    }

    // Calls CoCreateClass with the provided CLSID retrieved from the string.
    explicit _com_ptr_t(LPCWSTR str, IUnknown* pOuter = nullptr, DWORD dwClsContext = CLSCTX_ALL)
    {
        HRESULT hr = CreateInstance(str, pOuter, dwClsContext);

        if (FAILED(hr) && (hr != E_NOINTERFACE)) {
            _com_issue_error(hr);
        }
    }

    // Calls CoCreateClass with the provided SBCS CLSID retrieved from the string.
    explicit _com_ptr_t(LPCSTR str, IUnknown* pOuter = nullptr, DWORD dwClsContext = CLSCTX_ALL)
    {
        HRESULT hr = CreateInstance(str, pOuter, dwClsContext);

        if (FAILED(hr) && (hr != E_NOINTERFACE)) {
            _com_issue_error(hr);
        }
    }

    _com_ptr_t& operator=(const _com_ptr_t& p) noexcept {
        if (m_pInterface != p.m_pInterface)
        {
            _com_ptr_t copied{p};
            swap(*this, copied);
        }

        return *this;
    }

    _com_ptr_t& operator=(_com_ptr_t&& cp) noexcept
    {
        if (m_pInterface != cp.m_pInterface) {
            Interface* pOldInterface = m_pInterface;

            m_pInterface = cp.m_pInterface;
            cp.m_pInterface = nullptr;

            if (pOldInterface) {
                pOldInterface->Release();
            }
        }

        return *this;
    }

    _com_ptr_t& operator=(::std::nullptr_t) noexcept
    {
        _Release();
        m_pInterface = nullptr;
        return *this;
    }

    friend void swap(_com_ptr_t& left, _com_ptr_t& right) noexcept
    {
        Interface* tmp = left.m_pInterface;
        left.m_pInterface = right.m_pInterface;
        right.m_pInterface = tmp;
    }

    ~_com_ptr_t() noexcept
    {
        _Release();
    }

    // Saves/sets the interface without AddRef()ing. This call
    // will release any previously acquired interface.
    //
    void Attach(Interface* pInterface) noexcept
    {
        _Release();
        m_pInterface = pInterface;
    }

    // Saves/sets the interface only AddRef()ing if fAddRef is true.
    // This call will release any previously acquired interface.
    //
    void Attach(Interface* pInterface, bool fAddRef)
    {
        _Release();
        m_pInterface = pInterface;

        if (fAddRef) {
            if (pInterface) {
                pInterface->AddRef();
            }
            else {
                _com_issue_error(E_POINTER);
            }
        }
    }

    // Set the interface pointer to nullptr, so that it isn't Release()'ed.
    //
    Interface* Detach() noexcept
    {
        Interface* const old = m_pInterface;
        m_pInterface = nullptr;
        return old;
    }

    operator Interface*() const noexcept
    {
        return m_pInterface;
    }

    operator Interface&() const
    {
        if (m_pInterface)
        {
            return *m_pInterface;
        }

        _com_issue_error(E_POINTER);
    }

    // Allows an instance of this class to act as though it were the actual interface.
    Interface& operator*() const
    {
        if (m_pInterface)
        {
            return *m_pInterface;
        }

        _com_issue_error(E_POINTER);
    }

    // Returns the address of the interface pointer contained in this class. This is useful when using the COM/OLE
    // interfaces to create this interface.
    Interface** operator&() noexcept
    {
        _Release();
        m_pInterface = nullptr;
        return &m_pInterface;
    }

    // Allows this class to be used as the interface itself.
    Interface* operator->() const
    {
        if (m_pInterface)
        {
            return m_pInterface;
        }

        _com_issue_error(E_POINTER);
    }

#ifndef _COM_DISABLE_EXPLICIT_OPERATOR_BOOL
    explicit
#endif
    operator bool() const noexcept
    {
        return m_pInterface != nullptr;
    }

    bool operator==(const _com_ptr_t& p) const
    {
        if (m_pInterface == p.m_pInterface)
        {
            return true;
        }

        return _CompareUnknown(p.m_pInterface) == 0;
    }

    template<typename _OtherIID, ::std::enable_if_t<!::std::is_same<_IIID, _OtherIID>::value, int> = 0>
    bool operator==(const _com_ptr_t<_OtherIID>& p) const
    {
        return _CompareUnknown(p) == 0;
    }

    bool operator==(Interface* p) const
    {
        if (m_pInterface == p)
        {
            return true;
        }

        return _CompareUnknown(p) == 0;
    }

    friend bool operator==(Interface* p, const _com_ptr_t& _This)
    {
        if (_This.m_pInterface == p)
        {
            return true;
        }

        return _This._CompareUnknown(p) == 0;
    }

    template<typename _InterfaceType,
        ::std::enable_if_t<!::std::is_same<_InterfaceType, Interface>::value, int> = 0>
    bool operator==(_InterfaceType* p) const
    {
        return _CompareUnknown(p) == 0;
    }

    template<typename _InterfaceType,
        ::std::enable_if_t<!::std::is_same<_InterfaceType, Interface>::value, int> = 0>
    friend bool operator==(_InterfaceType* p, const _com_ptr_t& _This)
    {
        return _This._CompareUnknown(p) == 0;
    }

    bool operator==(decltype(nullptr)) const noexcept
    {
        return m_pInterface == nullptr;
    }

    friend bool operator==(decltype(nullptr), const _com_ptr_t& _This) noexcept
    {
        return _This.m_pInterface == nullptr;
    }

    bool operator==(int null) const
    {
        if (null != 0)
        {
            _com_issue_error(E_POINTER);
        }

        return m_pInterface == nullptr;
    }

    friend bool operator==(int null, const _com_ptr_t& _This)
    {
        if (null != 0)
        {
            _com_issue_error(E_POINTER);
        }

        return _This.m_pInterface == nullptr;
    }

    bool operator!=(const _com_ptr_t& p) const
    {
        return !(*this == p);
    }

    template<typename _OtherIID, ::std::enable_if_t<!::std::is_same<_IIID, _OtherIID>::value, int> = 0>
    bool operator!=(const _com_ptr_t<_OtherIID>& p) const
    {
        return !(*this == p);
    }

    bool operator!=(Interface* p) const
    {
        return !(*this == p);
    }

    friend bool operator!=(Interface* p, const _com_ptr_t& _This)
    {
        return !(_This == p);
    }

    template<typename _InterfaceType,
        ::std::enable_if_t<!::std::is_same<_InterfaceType, Interface>::value, int> = 0>
    bool operator!=(_InterfaceType* p) const
    {
        return !(*this == p);
    }

    template<typename _InterfaceType,
        ::std::enable_if_t<!::std::is_same<_InterfaceType, Interface>::value, int> = 0>
    friend bool operator!=(_InterfaceType* p, const _com_ptr_t& _This)
    {
        return !(_This == p);
    }

    bool operator!=(decltype(nullptr)) const noexcept
    {
        return m_pInterface != nullptr;
    }

    friend bool operator!=(decltype(nullptr), const _com_ptr_t& _This) noexcept
    {
        return _This.m_pInterface != nullptr;
    }

    bool operator!=(int null) const
    {
        if (null != 0)
        {
            _com_issue_error(E_POINTER);
        }

        return m_pInterface != nullptr;
    }

    friend bool operator!=(int null, const _com_ptr_t& _This)
    {
        if (null != 0)
        {
            _com_issue_error(E_POINTER);
        }

        return _This.m_pInterface != nullptr;
    }

    bool operator<(const _com_ptr_t& p) const
    {
        return _CompareUnknown(p.m_pInterface) < 0;
    }

    template<typename _OtherIID, ::std::enable_if_t<!::std::is_same<_IIID, _OtherIID>::value, int> = 0>
    bool operator<(const _com_ptr_t<_OtherIID>& p) const
    {
        return _CompareUnknown(p.GetInterfacePtr()) < 0;
    }

    bool operator<(Interface* p) const
    {
        return _CompareUnknown(p) < 0;
    }

    friend bool operator<(Interface* p, const _com_ptr_t& _This)
    {
        return _This._CompareUnknown(p) > 0;
    }

    template<typename _InterfaceType,
        ::std::enable_if_t<!::std::is_same<_InterfaceType, Interface>::value, int> = 0>
    bool operator<(_InterfaceType* p) const
    {
        return _CompareUnknown(p) < 0;
    }

    template<typename _InterfaceType,
        ::std::enable_if_t<!::std::is_same<_InterfaceType, Interface>::value, int> = 0>
    friend bool operator<(_InterfaceType* p, const _com_ptr_t& _This)
    {
        return _This._CompareUnknown(p) > 0;
    }

    bool operator<(decltype(nullptr)) const noexcept
    {
        return false;
    }

    friend bool operator<(decltype(nullptr), const _com_ptr_t& _This) noexcept
    {
        return _This.m_pInterface != nullptr;
    }

    bool operator<(int null) const
    {
        if (null != 0)
        {
            _com_issue_error(E_POINTER);
        }

        return false;
    }

    friend bool operator<(int null, const _com_ptr_t& _This)
    {
        if (null != 0)
        {
            _com_issue_error(E_POINTER);
        }

        return _This.m_pInterface != nullptr;
    }

    bool operator>(const _com_ptr_t& p) const
    {
        return _CompareUnknown(p.m_pInterface) > 0;
    }

    template<typename _OtherIID, ::std::enable_if_t<!::std::is_same<_IIID, _OtherIID>::value, int> = 0>
    bool operator>(const _com_ptr_t<_OtherIID>& p) const
    {
        return _CompareUnknown(p.GetInterfacePtr()) > 0;
    }

    bool operator>(Interface* p) const
    {
        return _CompareUnknown(p) > 0;
    }

    friend bool operator>(Interface* p, const _com_ptr_t& _This)
    {
        return _This._CompareUnknown(p) < 0;
    }

    template<typename _InterfaceType,
        ::std::enable_if_t<!::std::is_same<_InterfaceType, Interface>::value, int> = 0>
    bool operator>(_InterfaceType* p) const
    {
        return _CompareUnknown(p) > 0;
    }

    template<typename _InterfaceType,
        ::std::enable_if_t<!::std::is_same<_InterfaceType, Interface>::value, int> = 0>
    friend bool operator>(_InterfaceType* p, const _com_ptr_t& _This)
    {
        return _This._CompareUnknown(p) < 0;
    }

    bool operator>(decltype(nullptr)) const noexcept
    {
        return m_pInterface != nullptr;
    }

    friend bool operator>(decltype(nullptr), const _com_ptr_t& _This) noexcept
    {
        (void)_This;
        return false;
    }

    bool operator>(int null) const
    {
        if (null != 0)
        {
            _com_issue_error(E_POINTER);
        }

        return m_pInterface != nullptr;
    }

    friend bool operator>(int null, const _com_ptr_t& _This)
    {
        if (null != 0)
        {
            _com_issue_error(E_POINTER);
        }

        (void)_This;
        return false;
    }

    bool operator<=(const _com_ptr_t& p) const
    {
        return _CompareUnknown(p.m_pInterface) <= 0;
    }

    template<typename _OtherIID, ::std::enable_if_t<!::std::is_same<_IIID, _OtherIID>::value, int> = 0>
    bool operator<=(const _com_ptr_t<_OtherIID>& p) const
    {
        return _CompareUnknown(p.GetInterfacePtr()) <= 0;
    }

    bool operator<=(Interface* p) const
    {
        return _CompareUnknown(p) <= 0;
    }

    friend bool operator<=(Interface* p, const _com_ptr_t& _This)
    {
        return _This._CompareUnknown(p) >= 0;
    }

    template<typename _InterfaceType,
        ::std::enable_if_t<!::std::is_same<_InterfaceType, Interface>::value, int> = 0>
    bool operator<=(_InterfaceType* p) const
    {
        return _CompareUnknown(p) <= 0;
    }

    template<typename _InterfaceType,
        ::std::enable_if_t<!::std::is_same<_InterfaceType, Interface>::value, int> = 0>
    friend bool operator<=(_InterfaceType* p, const _com_ptr_t& _This)
    {
        return _This._CompareUnknown(p) >= 0;
    }

    bool operator<=(decltype(nullptr)) const noexcept
    {
        return m_pInterface == nullptr;
    }

    friend bool operator<=(decltype(nullptr), const _com_ptr_t&) noexcept
    {
        return true;
    }

    bool operator<=(int null) const
    {
        if (null != 0)
        {
            _com_issue_error(E_POINTER);
        }

        return m_pInterface == nullptr;
    }

    friend bool operator<=(int null, const _com_ptr_t& _This)
    {
        if (null != 0)
        {
            _com_issue_error(E_POINTER);
        }

        (void)_This;
        return true;
    }

    bool operator>=(const _com_ptr_t& p) const
    {
        return _CompareUnknown(p.m_pInterface) >= 0;
    }

    template<typename _OtherIID, ::std::enable_if_t<!::std::is_same<_IIID, _OtherIID>::value, int> = 0>
    bool operator>=(const _com_ptr_t<_OtherIID>& p) const
    {
        return _CompareUnknown(p.GetInterfacePtr()) >= 0;
    }

    bool operator>=(Interface* p) const
    {
        return _CompareUnknown(p) >= 0;
    }

    friend bool operator>=(Interface* p, const _com_ptr_t& _This)
    {
        return _This._CompareUnknown(p) <= 0;
    }

    template<typename _InterfaceType,
        ::std::enable_if_t<!::std::is_same<_InterfaceType, Interface>::value, int> = 0>
    bool operator>=(_InterfaceType* p) const
    {
        return _CompareUnknown(p) >= 0;
    }

    template<typename _InterfaceType,
        ::std::enable_if_t<!::std::is_same<_InterfaceType, Interface>::value, int> = 0>
    friend bool operator>=(_InterfaceType* p, const _com_ptr_t& _This)
    {
        return _This._CompareUnknown(p) <= 0;
    }

    bool operator>=(decltype(nullptr)) const noexcept
    {
        return true;
    }

    friend bool operator>=(decltype(nullptr), const _com_ptr_t& _This) noexcept
    {
        return _This.m_pInterface == nullptr;
    }

    bool operator>=(int null) const
    {
        if (null != 0)
        {
            _com_issue_error(E_POINTER);
        }

        return true;
    }

    friend bool operator>=(int null, const _com_ptr_t& _This)
    {
        if (null != 0)
        {
            _com_issue_error(E_POINTER);
        }

        return _This.m_pInterface == nullptr;
    }

    void Release()
    {
        if (m_pInterface)
        {
            m_pInterface->Release();
            m_pInterface = nullptr;
        }
        else
        {
            _com_issue_error(E_POINTER);
        }
    }

    void AddRef()
    {
        if (m_pInterface)
        {
            m_pInterface->AddRef();
        }
        else
        {
            _com_issue_error(E_POINTER);
        }
    }

    Interface* GetInterfacePtr() const noexcept
    {
        return m_pInterface;
    }

    Interface*& GetInterfacePtr() noexcept
    {
        return m_pInterface;
    }

    // Loads an interface for the provided CLSID.
    // Returns an HRESULT.  Any previous interface is unconditionally released.
    HRESULT CreateInstance(const CLSID& rclsid, IUnknown* pOuter = nullptr, DWORD dwClsContext = CLSCTX_ALL) noexcept
    {
        HRESULT hr;

        _Release();

        if (dwClsContext & (CLSCTX_LOCAL_SERVER | CLSCTX_REMOTE_SERVER)) {
            IUnknown* pIUnknown;
            hr = CoCreateInstance(rclsid, pOuter, dwClsContext, __uuidof(IUnknown), reinterpret_cast<void**>(&pIUnknown));

            if (SUCCEEDED(hr)) {
                hr = OleRun(pIUnknown);

                if (SUCCEEDED(hr)) {
                    hr = pIUnknown->QueryInterface(GetIID(), reinterpret_cast<void**>(&m_pInterface));
                }

                pIUnknown->Release();
            }
        }
        else {
            hr = CoCreateInstance(rclsid, pOuter, dwClsContext, GetIID(), reinterpret_cast<void**>(&m_pInterface));
        }

        if (FAILED(hr)) {
            // just in case refcount = 0 and dtor gets called
            m_pInterface = nullptr;
        }

        return hr;
    }

    // Creates the class specified by clsidString. clsidString may contain a class id, or a prog id string.
    HRESULT CreateInstance(LPCWSTR clsidString, IUnknown* pOuter = nullptr, DWORD dwClsContext = CLSCTX_ALL) noexcept
    {
        if (clsidString == nullptr) {
            return E_INVALIDARG;
        }

        CLSID clsid;
        HRESULT hr;

        if (clsidString[0] == L'{') {
            hr = CLSIDFromString(const_cast<LPWSTR> (clsidString), &clsid);
        }
        else {
            hr = CLSIDFromProgID(const_cast<LPWSTR> (clsidString), &clsid);
        }

        if (FAILED(hr)) {
            return hr;
        }

        return CreateInstance(clsid, pOuter, dwClsContext);
    }

    // Creates the class specified by SBCS clsidString. clsidString may contain a class id, or a prog id string.
    HRESULT CreateInstance(LPCSTR clsidStringA, IUnknown* pOuter = nullptr, DWORD dwClsContext = CLSCTX_ALL) noexcept
    {
        if (clsidStringA == nullptr) {
            return E_INVALIDARG;
        }

        size_t const size = strlen(clsidStringA) + 1;

        if (size > INT_MAX) {
            return E_INVALIDARG;
        }

        int const destSize = MultiByteToWideChar(CP_ACP, 0, clsidStringA, static_cast<int>(size), nullptr, 0);

        if (destSize == 0) {
            return HRESULT_FROM_WIN32(GetLastError());
        }

        LPWSTR clsidStringW;
        clsidStringW = static_cast<LPWSTR>(_malloca(destSize * sizeof(WCHAR)));

        if (clsidStringW == nullptr) {
            return E_OUTOFMEMORY;
        }

        if (MultiByteToWideChar(CP_ACP, 0, clsidStringA, static_cast<int>(size), clsidStringW, destSize) == 0) {
           _freea(clsidStringW);
           return HRESULT_FROM_WIN32(GetLastError());
        }

        HRESULT hr=CreateInstance(clsidStringW, pOuter, dwClsContext);
        _freea(clsidStringW);
        return hr;
    }

    // Attach to the active object specified by rclsid.
    // Any previous interface is released.
    //
    HRESULT GetActiveObject(const CLSID& rclsid) noexcept
    {
        _Release();

        IUnknown* pIUnknown;

        HRESULT hr = ::GetActiveObject(rclsid, nullptr, &pIUnknown);

        if (SUCCEEDED(hr)) {
            hr = pIUnknown->QueryInterface(GetIID(), reinterpret_cast<void**>(&m_pInterface));

            pIUnknown->Release();
        }

        if (FAILED(hr)) {
            // just in case refcount = 0 and dtor gets called
            m_pInterface = nullptr;
        }

        return hr;
    }

    // Attach to the active object specified by clsidString.
    // First convert the LPCWSTR to a CLSID.
    //
    HRESULT GetActiveObject(LPCWSTR clsidString) noexcept
    {
        if (clsidString == nullptr) {
            return E_INVALIDARG;
        }

        CLSID clsid;
        HRESULT hr;

        if (clsidString[0] == '{') {
            hr = CLSIDFromString(const_cast<LPWSTR> (clsidString), &clsid);
        }
        else {
            hr = CLSIDFromProgID(const_cast<LPWSTR> (clsidString), &clsid);
        }

        if (FAILED(hr)) {
            return hr;
        }

        return GetActiveObject(clsid);
    }

    // Attach to the active object specified by clsidStringA.
    // First convert the LPCSTR to a LPCWSTR.
    //
    HRESULT GetActiveObject(LPCSTR clsidStringA) noexcept
    {
        if (clsidStringA == nullptr) {
            return E_INVALIDARG;
        }

        size_t const size = strlen(clsidStringA) + 1;

        if (size > INT_MAX) {
            return E_INVALIDARG;
        }

        int const destSize = MultiByteToWideChar(CP_ACP, 0, clsidStringA, static_cast<int>(size), nullptr, 0);

        LPWSTR clsidStringW;
        __try {
            clsidStringW = static_cast<LPWSTR>(_alloca(destSize * sizeof(WCHAR)));
        }
        __except (GetExceptionCode() == STATUS_STACK_OVERFLOW) {
            clsidStringW = nullptr;
        }

        if (clsidStringW == nullptr) {
            return E_OUTOFMEMORY;
        }

        if (MultiByteToWideChar(CP_ACP, 0, clsidStringA, static_cast<int>(size), clsidStringW, destSize) == 0) {
            return HRESULT_FROM_WIN32(GetLastError());
        }

        return GetActiveObject(clsidStringW);
    }

    // Performs the QI for the specified IID and returns it in p.
    // As with all QIs, the interface will be AddRef'd.
    //
    template<typename _InterfaceType> HRESULT QueryInterface(const IID& iid, _InterfaceType*& p) noexcept
    {
        if (m_pInterface) {
            return m_pInterface->QueryInterface(iid, reinterpret_cast<void**>(&p));
        }

        return E_POINTER;
    }

    // Performs the QI for the specified IID and returns it in p.
    // As with all QIs, the interface will be AddRef'd.
    //
    template<typename _InterfaceType> HRESULT QueryInterface(const IID& iid, _InterfaceType** p) noexcept
    {
        return QueryInterface(iid, *p);
    }

private:
    // The Interface.
    //
    Interface* m_pInterface{};

    // Releases only if the interface is not null.
    // The interface is not set to nullptr.
    //
    void _Release() noexcept
    {
        if (m_pInterface) {
            m_pInterface->Release();
        }
    }

    // AddRefs only if the interface is not nullptr
    //
    void _AddRef() noexcept
    {
        if (m_pInterface) {
            m_pInterface->AddRef();
        }
    }

    // Performs a QI on pUnknown for the interface type returned
    // for this class.  The interface is stored.  If pUnknown is
    // nullptr, or the QI fails, E_NOINTERFACE is returned and
    // _pInterface is set to nullptr.
    //
    template<typename _InterfacePtr> HRESULT _QueryInterface(_InterfacePtr p) noexcept
    {
        HRESULT hr;

        // Can't QI nullptr
        //
        if (p) {
            // Query for this interface
            //
            Interface* pInterface{};
            hr = p->QueryInterface(GetIID(), reinterpret_cast<void**>(&pInterface));

            // Save the interface without AddRef()ing.
            //
            Attach(SUCCEEDED(hr) ? pInterface: nullptr);
        }
        else {
            operator=(static_cast<Interface*>(nullptr));
            hr = E_NOINTERFACE;
        }

        return hr;
    }

    // Compares the provided pointer with this by obtaining IUnknown interfaces
    // for each pointer and then returning the difference.
    //
    template<typename _InterfacePtr> int _CompareUnknown(_InterfacePtr p) const
    {
        IUnknown* pu1{};
        IUnknown* pu2{};

        if (m_pInterface) {
            HRESULT hr = m_pInterface->QueryInterface(__uuidof(IUnknown), reinterpret_cast<void**>(&pu1));

            if (FAILED(hr)) {
                pu1 = nullptr;
                _com_issue_error(hr);
            }
            else {
                pu1->Release();
            }
        }

        if (p) {
            HRESULT hr = p->QueryInterface(__uuidof(IUnknown), reinterpret_cast<void**>(&pu2));

            if (FAILED(hr)) {
                pu2 = nullptr;
                _com_issue_error(hr);
            }
            else {
                pu2->Release();
            }
        }

        if (pu1 == pu2)
        {
            return 0;
        }

        return (pu1 > pu2) ? 1 : -1;
    }

    // Try to extract either IDispatch* or an IUnknown* from
    // the VARIANT
    //
    HRESULT QueryStdInterfaces(const _variant_t& varSrc) noexcept
    {
        if (V_VT(&varSrc) == VT_DISPATCH) {
            return _QueryInterface(V_DISPATCH(&varSrc));
        }

        if (V_VT(&varSrc) == VT_UNKNOWN) {
            return _QueryInterface(V_UNKNOWN(&varSrc));
        }

        // We have something other than an IUnknown or an IDispatch.
        // Can we convert it to either one of these?
        // Try IDispatch first
        //
        VARIANT varDest;
        VariantInit(&varDest);

        HRESULT hr = VariantChangeType(&varDest, const_cast<VARIANT*>(static_cast<const VARIANT*>(&varSrc)), 0, VT_DISPATCH);
        if (SUCCEEDED(hr)) {
            hr = _QueryInterface(V_DISPATCH(&varDest));
        }

        if (hr == E_NOINTERFACE) {
            // That failed ... so try IUnknown
            //
            VariantInit(&varDest);
            hr = VariantChangeType(&varDest, const_cast<VARIANT*>(static_cast<const VARIANT*>(&varSrc)), 0, VT_UNKNOWN);
            if (SUCCEEDED(hr)) {
                hr = _QueryInterface(V_UNKNOWN(&varDest));
            }
        }

        VariantClear(&varDest);
        return hr;
    }
};
#endif // _INC_COMIP
