/***
* comdef.h - Native C++ compiler COM support - main definitions header
*
* Copyright (c) Microsoft Corporation. All rights reserved.
*
****/

#pragma once

#if !defined(_INC_COMDEF)
#define _INC_COMDEF
#if !defined(RC_INVOKED)

#ifndef  __cplusplus
#error Native Compiler support only available in C++ compiler
#endif

#ifdef _M_CEE_PURE
#error comdef.h header cannot be included under /clr:safe or /clr:pure
#endif

#ifdef WINAPI_FAMILY
#include <winapifamily.h>
#if !WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)
#define _COMDEF_NOT_WINAPI_FAMILY_DESKTOP_APP
#endif /* !WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#if defined WINAPI_FAMILY_PHONE_APP && WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_PHONE_APP)
#define _COMDEF_WINAPI_FAMILY_PHONE_APP
#endif /* defined WINAPI_FAMILY_PHONE_APP && WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_PHONE_APP) */
#endif /* WINAPI_FAMILY */

#ifndef _COMDEF_WINAPI_FAMILY_PHONE_APP
#include <Ole2.h>

#include <comutil.h>
#endif /* _COMDEF_WINAPI_FAMILY_PHONE_APP */

#pragma warning(push)
#pragma warning(disable: 4244)

#ifdef _COMDEF_NOT_WINAPI_FAMILY_DESKTOP_APP

#include <roerrorapi.h>
#include <new.h>
#include <wchar.h>

inline void __stdcall _com_issue_error(HRESULT hr);

class _com_error
{
protected:
    static wchar_t* AllocateString(const wchar_t* message)
    {
        wchar_t* value = nullptr;
        if (message != nullptr)
        {
            auto length = ::wcslen(message) + 1; // add 1 for null terminator
            value = new (std::nothrow) wchar_t[length];
            if (value == nullptr)
            {
                _com_issue_error(E_OUTOFMEMORY);
            }

            ::wmemcpy(value, message, length);
        }

        return value;
    }
public:
    _com_error(HRESULT hr, const wchar_t* message) : m_hr(hr), m_message(nullptr)
    {
        m_message = AllocateString(message);
    }

    _com_error(const _com_error& other)
    {
        m_hr = other.m_hr;
        m_message = AllocateString(other.m_message);
    }

    _com_error(_com_error&& other)
    {
        m_hr = other.m_hr;
        m_message = other.m_message;
        other.m_message = nullptr;
    }

    ~_com_error() throw()
    {
        delete [] m_message;
    }

    _com_error& operator=(const _com_error& other)
    {
        if (this != &other)
        {
            m_hr = other.m_hr;
            delete [] m_message;
            m_message = AllocateString(other.m_message);
        }
        return *this;
    }

    _com_error& operator=(_com_error&& other)
    {
        if (this != &other)
        {
            m_hr = other.m_hr;
            delete [] m_message;
            m_message = other.m_message;
            other.m_message = nullptr;
        }
        return *this;
    }

    HRESULT Error() const throw()
    {
        return m_hr;
    }

    const wchar_t* ErrorMessage() const throw()
    {
        if (m_message == nullptr)
        {
            wchar_t buffer[4096];
            if (::FormatMessageW(FORMAT_MESSAGE_FROM_SYSTEM,
                                    nullptr,
                                    m_hr,
                                    MAKELANGID(LANG_NEUTRAL, SUBLANG_NEUTRAL),
                                    buffer,
                                    _countof(buffer),
                                    nullptr))
            {
                m_message = AllocateString(buffer);
            }
        }
        return m_message;
    }
protected:
    HRESULT m_hr;
    mutable wchar_t* m_message;
};

inline void __declspec(noreturn) __stdcall _com_raise_error(HRESULT hr, const wchar_t* message)
{
    size_t length = (message == nullptr) ? 0 : ::wcslen(message);

    if (UINT_MAX < length)
    {
        length = 0;
    }

    ::Windows::Foundation::Diagnostics::OriginateError(hr, static_cast<unsigned int>(length), message);
    throw _com_error(hr, message);
}

typedef void (__stdcall *__errorPfnType)(HRESULT hr, const wchar_t* message);

// throw exceptions by default
__declspec(selectany) __errorPfnType __errorPfn = &_com_raise_error;

inline void __stdcall _com_issue_errorex(HRESULT hr, const wchar_t* message)
{
    __errorPfn(hr, message);
}

inline void __stdcall _com_issue_error(HRESULT hr)
{
    __errorPfn(hr, nullptr);
}

inline void __stdcall _set_com_error_handler(void (__stdcall *pHandler)(HRESULT, const wchar_t*))
{
    __errorPfn = pHandler;
}

#else

#include <OleCtl.h>

#ifdef _NATIVE_WCHAR_T_DEFINED
#if defined(_GUARDED_CRT)
# ifdef _DEBUG
# pragma comment(lib, "comsuppwgd.lib")
# else
# pragma comment(lib, "comsuppwg.lib")
# endif
#else
# ifdef _DEBUG
# pragma comment(lib, "comsuppwd.lib")
# else
# pragma comment(lib, "comsuppw.lib")
# endif
#endif
#else
#if defined(_GUARDED_CRT)
# ifdef _DEBUG
# pragma comment(lib, "comsuppgd.lib")
# else
# pragma comment(lib, "comsuppg.lib")
# endif
#else
# ifdef _DEBUG
# pragma comment(lib, "comsuppd.lib")
# else
# pragma comment(lib, "comsupp.lib")
# endif
#endif
#endif

#pragma comment(lib, "user32.lib")
#pragma comment(lib, "ole32.lib")
#pragma comment(lib, "oleaut32.lib")

class _com_error;

void __declspec(noreturn) __stdcall
    _com_raise_error(HRESULT hr, IErrorInfo* perrinfo = 0) ;

void __stdcall
    _set_com_error_handler(void (__stdcall *pHandler)(HRESULT hr, IErrorInfo* perrinfo));

void __stdcall
    _com_issue_error(HRESULT) ;
void __stdcall
    _com_issue_errorex(HRESULT, IUnknown*, REFIID) ;

HRESULT __stdcall
    _com_dispatch_propget(IDispatch*, DISPID, VARTYPE, void*) ;
HRESULT __cdecl
    _com_dispatch_propput(IDispatch*, DISPID, VARTYPE, ...) ;
HRESULT __cdecl
    _com_dispatch_method(IDispatch*, DISPID, WORD, VARTYPE, void*,
                         const wchar_t*, ...) ;

HRESULT __stdcall
    _com_dispatch_raw_propget(IDispatch*, DISPID, VARTYPE, void*) throw();
HRESULT __cdecl
    _com_dispatch_raw_propput(IDispatch*, DISPID, VARTYPE, ...) throw();
HRESULT __cdecl
    _com_dispatch_raw_method(IDispatch*, DISPID, WORD, VARTYPE, void*,
                             const wchar_t*, ...) throw();

class _com_error {
public:
    // Constructors
    //
    _com_error(HRESULT hr,
               IErrorInfo* perrinfo = NULL,
               bool fAddRef = false) throw();
    _com_error(const _com_error& that) throw();

    // Destructor
    //
    virtual ~_com_error() noexcept;

    // Assignment operator
    //
    _com_error& operator=(const _com_error& that) throw();

    // Accessors
    //
    HRESULT Error() const throw();
    WORD WCode() const throw();
    IErrorInfo * ErrorInfo() const throw();

    // IErrorInfo method accessors
    //
    _bstr_t Description() const ;
    DWORD HelpContext() const throw();
    _bstr_t HelpFile() const ;
    _bstr_t Source() const ;
    GUID GUID() const throw();

    // FormatMessage accessors
    //
    const TCHAR * ErrorMessage() const throw();

    // EXCEPINFO.wCode <-> HRESULT mappers
    //
    static HRESULT WCodeToHRESULT(WORD wCode) throw();
    static WORD HRESULTToWCode(HRESULT hr) throw();

private:
    enum {
        WCODE_HRESULT_FIRST = MAKE_HRESULT(SEVERITY_ERROR, FACILITY_ITF, 0x200),
        WCODE_HRESULT_LAST = MAKE_HRESULT(SEVERITY_ERROR, FACILITY_ITF+1, 0) - 1
    };
    const HRESULT           m_hresult;
    IErrorInfo *            m_perrinfo;
    mutable TCHAR *         m_pszMsg;
};

inline _com_error::_com_error(HRESULT hr,
                              IErrorInfo* perrinfo,
                              bool fAddRef) throw()
    : m_hresult(hr), m_perrinfo(perrinfo), m_pszMsg(NULL)
{
    if (m_perrinfo != NULL && fAddRef) {
        m_perrinfo->AddRef();
    }
}

inline _com_error::_com_error(const _com_error& that) throw()
    : m_hresult(that.m_hresult), m_perrinfo(that.m_perrinfo), m_pszMsg(NULL)
{
    if (m_perrinfo != NULL) {
        m_perrinfo->AddRef();
    }
}

inline _com_error::~_com_error() noexcept
{
    if (m_perrinfo != NULL) {
        m_perrinfo->Release();
    }
    if (m_pszMsg != NULL) {
        LocalFree((HLOCAL)m_pszMsg);
    }
}

inline _com_error& _com_error::operator=(const _com_error& that) throw()
{
    if (this != &that) {
        this->_com_error::~_com_error();
        this->_com_error::_com_error(that);
    }
    return *this;
}

inline HRESULT _com_error::Error() const throw()
{
    return m_hresult;
}

inline WORD _com_error::WCode() const throw()
{
    return HRESULTToWCode(m_hresult);
}

inline IErrorInfo * _com_error::ErrorInfo() const throw()
{
    if (m_perrinfo != NULL) {
        m_perrinfo->AddRef();
    }
    return m_perrinfo;
}

inline _bstr_t _com_error::Description() const
{
    BSTR bstr = NULL;
    if (m_perrinfo != NULL) {
        if (FAILED(m_perrinfo->GetDescription(&bstr))) {
            bstr = NULL;
        }
    }
    return _bstr_t(bstr, false);
}

inline DWORD _com_error::HelpContext() const throw()
{
    DWORD dwHelpContext = 0;
    if (m_perrinfo != NULL) {
        if (FAILED(m_perrinfo->GetHelpContext(&dwHelpContext))) {
            dwHelpContext = 0;
        }
    }
    return dwHelpContext;
}

inline _bstr_t _com_error::HelpFile() const
{
    BSTR bstr = NULL;
    if (m_perrinfo != NULL) {
        if (FAILED(m_perrinfo->GetHelpFile(&bstr))) {
            bstr = NULL;
        }
    }
    return _bstr_t(bstr, false);
}

inline _bstr_t _com_error::Source() const
{
    BSTR bstr = NULL;
    if (m_perrinfo != NULL) {
        if (FAILED(m_perrinfo->GetSource(&bstr))) {
            bstr = NULL;
        }
    }
    return _bstr_t(bstr, false);
}

inline _GUID _com_error::GUID() const throw()
{
    _GUID guid{};
    if (m_perrinfo != NULL) {
        if (FAILED(m_perrinfo->GetGUID(&guid))) {
            guid = _GUID{};
        }
    }
    return guid;
}

inline const TCHAR * _com_error::ErrorMessage() const throw()
{
    if (m_pszMsg == NULL) {
        FormatMessage(FORMAT_MESSAGE_ALLOCATE_BUFFER|
                          FORMAT_MESSAGE_FROM_SYSTEM|
                          FORMAT_MESSAGE_IGNORE_INSERTS,
                      NULL,
                      m_hresult,
                      MAKELANGID(LANG_NEUTRAL, SUBLANG_DEFAULT),
                      (LPTSTR)&m_pszMsg,
                      0,
                      NULL);
        if (m_pszMsg != NULL) {
            #ifdef UNICODE
            size_t const nLen = wcslen(m_pszMsg);
            #else
            size_t const nLen = strlen(m_pszMsg);
            #endif
            if (nLen > 1 && m_pszMsg[nLen - 1] == '\n') {
                m_pszMsg[nLen - 1] = 0;
                if (m_pszMsg[nLen - 2] == '\r') {
                        m_pszMsg[nLen - 2] = 0;
                }
            }
        }
        else {
            m_pszMsg = (LPTSTR)LocalAlloc(0, 32 * sizeof(TCHAR));
            if (m_pszMsg != NULL) {
                WORD wCode = WCode();
                if (wCode != 0) {
                    _COM_PRINTF_S_1(m_pszMsg, 32, TEXT("IDispatch error #%d"), (int)wCode);
                }
                else {
                    _COM_PRINTF_S_1(m_pszMsg, 32, TEXT("Unknown error 0x%0lX"), m_hresult);
                }
            }
        }
    }
    return m_pszMsg;
}

inline HRESULT _com_error::WCodeToHRESULT(WORD wCode) throw()
{
    return wCode >= 0xFE00 ? WCODE_HRESULT_LAST : WCODE_HRESULT_FIRST + wCode;
}

inline WORD _com_error::HRESULTToWCode(HRESULT hr) throw()
{
    return (hr >= WCODE_HRESULT_FIRST && hr <= WCODE_HRESULT_LAST)
        ? WORD(hr - WCODE_HRESULT_FIRST)
        : 0;
}

//
// give missing types from dependent type libraries a chance
//
typedef int __missing_type__;

#if !defined(_COM_SMARTPTR)
 #if !defined(_INC_COMIP)
  #include <comip.h>
 #endif
 #define _COM_SMARTPTR        _com_ptr_t
 #define _COM_SMARTPTR_LEVEL2 _com_IIID
#endif
#if defined(_COM_SMARTPTR)
 #if !defined(_COM_SMARTPTR_TYPEDEF)
  #if defined(_COM_SMARTPTR_LEVEL2)
   #define _COM_SMARTPTR_TYPEDEF(Interface, IID) \
    typedef _COM_SMARTPTR<_COM_SMARTPTR_LEVEL2<Interface, &IID> > \
            Interface ## Ptr
  #else
   #define _COM_SMARTPTR_TYPEDEF(Interface, IID) \
    typedef _COM_SMARTPTR<Interface, &IID> \
            Interface ## Ptr
  #endif
 #endif
#endif

#if !defined(_COM_NO_STANDARD_GUIDS_)
#if !defined(_ATL_MODULES)
// hard-coded smart pointer defs
#if defined(__IFontDisp_INTERFACE_DEFINED__)
__if_not_exists(Font)
{
    struct Font : IFontDisp {};
}
_COM_SMARTPTR_TYPEDEF(Font, __uuidof(IDispatch));
#endif
#if defined(__IFontEventsDisp_INTERFACE_DEFINED__)
__if_not_exists(FontEvents)
{
    struct FontEvents : IFontEventsDisp {};
}
_COM_SMARTPTR_TYPEDEF(FontEvents, __uuidof(IDispatch));
#endif
#if defined(__IPictureDisp_INTERFACE_DEFINED__)
__if_not_exists(Picture)
{
    struct Picture : IPictureDisp {};
}
_COM_SMARTPTR_TYPEDEF(Picture, __uuidof(IDispatch));
#endif
#endif // !defined(_ATL_MODULES)

#include "comdefsp.h"

#endif  /* !defined(_COM_NO_STANDARD_GUIDS_) */

#endif /* _COMDEF_NOT_WINAPI_FAMILY_DESKTOP_APP */

#pragma warning(pop)

#endif /* RC_INVOKED */
#endif  /* _INC_COMDEF */
