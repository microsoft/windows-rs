//
// Copyright (C) Microsoft Corporation
// All rights reserved.
//
// Code in Details namespace is for internal usage within the library code
//

#ifndef _WRL_MODULE_H_
#define _WRL_MODULE_H_

#ifdef _MSC_VER
#pragma once
#endif  // _MSC_VER

#include <roapi.h>
#ifdef BUILD_WINDOWS
#include <winrt.h>
#endif
#include <activation.h>
#include <WinString.h>
#include <intrin.h>
#include <winapifamily.h>

#include <wrl\def.h>
#include <wrl\internal.h>
#include <wrl\client.h>
#include <wrl\implements.h>
#include <wrl\ftm.h>
#include <wrl\wrappers\corewrappers.h>

// Set packing
#include <pshpack8.h>

namespace Microsoft {
namespace WRL {

enum ModuleType
{
    InProc                   = 0x1,  // inproc server,
    OutOfProc                = 0x2,  // outproc server
    DisableCaching           = 0x4,   // disable caching mechanism on Module<>
    InProcDisableCaching     = InProc | DisableCaching,
    OutOfProcDisableCaching  = OutOfProc | DisableCaching
};

enum FactoryCacheFlags
{
    FactoryCacheDefault,
    FactoryCacheEnabled,
    FactoryCacheDisabled
};

namespace Details
{

// Keep information about factory and cookie data
struct FactoryCache
{
    IUnknown* factory;
    union {
        RO_REGISTRATION_COOKIE winrt;
        DWORD com;
    } cookie;
};

// Map contains information how to initialize, register and unregister objects
//
// How to compare activation data depending on classic COM or WinRT factory
// Keeps information about factory cache, server name for interface
struct CreatorMap
{
    // Factory creator function
    HRESULT(STDMETHODCALLTYPE *factoryCreator)(_In_ unsigned int*, _In_ const CreatorMap*, REFIID, _COM_Outptr_ IUnknown**) throw();
    // Object id
    union {
        const IID* clsid;
        const wchar_t* (STDMETHODCALLTYPE *getRuntimeName)();
    } activationId;
    // Trust level for WinRT otherwise nullptr
    ::TrustLevel (STDMETHODCALLTYPE *getTrustLevel)();
    // Factory cache, group id data members
    FactoryCache* factoryCache;
    const wchar_t* serverName;
};

class FactoryBase
{
};

// Compare server name strings
inline bool IsServerNameEqual(_In_ const CreatorMap* entry, const wchar_t* serverName) throw()
{
    if (serverName == nullptr)
    {
        return true;
    }
    else if (entry->serverName == nullptr)
    {
        return false;
    }

    return ::wcscmp(entry->serverName, serverName) == 0;
}

#pragma warning(push)
// Disable unheld lock warning in the case of static SRWLock methods being called with pre-existing SRWLOCK
#pragma warning(disable: 26165)

// Terminate class factories stored in the cache
inline bool TerminateMap(_In_ ModuleBase *modulePtr, _In_opt_z_ const wchar_t *serverName, bool forceTerminate) throw()
{
    auto entry = modulePtr->GetFirstEntryPointer() + 1;
    auto last = modulePtr->GetLastEntryPointer();

    // Walk the linker generated list of pointers to CreatorMap
    // It's necessary to start from COM objects and ends with WinRT
    for (; entry < last; entry++)
    {
        // Linker generated list can have null pointer values
        if (*entry != nullptr && IsServerNameEqual(*entry, serverName))
        {
            // We should not terminate cache if we have objects alive
            if (modulePtr->GetObjectCount() > 0 && !forceTerminate)
            {
                return false;
            }

            if (static_cast<IUnknown* volatile&>((*entry)->factoryCache->factory) == nullptr)
            {
                continue;
            }

            // Make sure that nobody is taking object from cache when we terminate factories
            void* factoryPointer = nullptr;
            { //Open scope for lock
                auto lock = ::Microsoft::WRL::Wrappers::SRWLock::LockExclusive(modulePtr->GetLock());

                // Don't need read memory barrier because lock adds one
                if ((*entry)->factoryCache->factory == nullptr)
                {
                    continue;
                }

                factoryPointer = (*entry)->factoryCache->factory;
                (*entry)->factoryCache->factory = nullptr;
            } // End of lock scope

            __WRL_ASSERT__(factoryPointer != nullptr);
            IUnknown *factory = reinterpret_cast<IUnknown*>(::DecodePointer(factoryPointer));
            _Analysis_assume_(factory != nullptr);
            factory->Release();
        }
    }

    return modulePtr->GetObjectCount() == 0 ? true : false;
}

// Gets the factory from the cache entry and if not available create one.
inline HRESULT GetCacheEntry(_In_ ModuleBase* modulePtr, _In_ unsigned int *flags, REFIID riid, _In_ const CreatorMap* entry, _COM_Outptr_ IUnknown **ppFactory) throw()
{
    *ppFactory = nullptr;

    IUnknown *factory = nullptr;

    // Check if the object is available in the cache
    if (static_cast<IUnknown* volatile&>(entry->factoryCache->factory) != nullptr)
    { // Read lock scope
        // Make sure that none of factories will be destroyed when WRL gets element from cache
        auto readLock = ::Microsoft::WRL::Wrappers::SRWLock::LockShared(modulePtr->GetLock());

        void* factoryPointer = entry->factoryCache->factory;
        if (factoryPointer != nullptr)
        {
            factory = reinterpret_cast<IUnknown*>(::DecodePointer(factoryPointer));
            _Analysis_assume_(factory != nullptr);
            __WRL_ASSERT__(factory != nullptr);
            return factory->QueryInterface(riid, reinterpret_cast<void**>(ppFactory));
        }
    } // End of read lock scope

    HRESULT hr = entry->factoryCreator(flags, entry, riid, &factory);
    if (FAILED(hr))
    {
        return hr;
    }

    // If caching enabled
    if ((*flags & DisableCaching) == 0)
    {
        IUnknown *cachedFactory = nullptr;
        { // Write lock scope
            auto writeLock = ::Microsoft::WRL::Wrappers::SRWLock::LockExclusive(modulePtr->GetLock());

            // Don't need read memory barrier because lock adds one
            void* factoryPointer = entry->factoryCache->factory;
            if (factoryPointer == nullptr)
            {
                // Put factory in cache
                entry->factoryCache->factory = reinterpret_cast<IUnknown*>(::EncodePointer(factory));
            }
            else
            {
                // Get factory from the cache if it's already there
                cachedFactory = reinterpret_cast<IUnknown*>(::DecodePointer(factoryPointer));
                _Analysis_assume_(cachedFactory != nullptr);
                cachedFactory->AddRef();
            }
        }  // End of write lock scope

        if (cachedFactory != nullptr)
        {
            // Release factory that was created
            // Requires double release because factoryCreator does two AddRef
            factory->Release();
            factory->Release();
            factory = cachedFactory;
        }
    }

    *ppFactory = factory;
    __WRL_ASSERT__(*ppFactory != nullptr);
    return S_OK;
}

#pragma warning(pop)

template<unsigned int flags>
inline HRESULT GetClassObject(_In_ ModuleBase *modulePtr, _In_opt_z_ const wchar_t* serverName, REFCLSID clsid, REFIID riid, _Outptr_result_nullonfailure_ void **ppv) throw()
{
    *ppv = nullptr;

    auto entry = modulePtr->GetFirstEntryPointer() + 1;
    auto last = modulePtr->GetMidEntryPointer();

    // Walk the linker generated list of pointers to CreatorMap for COM objects
    for (; entry < last; entry++)
    {
        // Linker generated list can have null pointer values
        if (*entry != nullptr &&  IsServerNameEqual(*entry, serverName))
        {
            if (*(*entry)->activationId.clsid == clsid)
            {
                unsigned int currentFlags = flags;
#pragma warning(push)
// Conditional expression is constant
#pragma warning(disable: 4127)
// Potential comparison of a constant with another constant
#pragma warning(disable: 6326)
                if ((flags & DisableCaching) == 0)
#pragma warning(pop)
                {
                    // Does not require QI
                    return GetCacheEntry(modulePtr, &currentFlags, riid, *entry, reinterpret_cast<IUnknown**>(ppv));
                }
                else
                {
                    return (*entry)->factoryCreator(&currentFlags, *entry, riid, reinterpret_cast<IUnknown**>(ppv));
                }
            }
        }
    }

    return CLASS_E_CLASSNOTAVAILABLE;
}

#pragma region Application Family or OneCore Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM)

template<unsigned int flags>
inline HRESULT GetActivationFactory(_In_ ModuleBase* modulePtr, _In_opt_z_ const wchar_t* serverName, _In_opt_ HSTRING activatibleClassId, _COM_Outptr_ IActivationFactory **ppFactory) throw()
{
    *ppFactory = nullptr;

    BOOL hasEmbedNull;
    if (::WindowsIsStringEmpty(activatibleClassId) ||
        (FAILED(::WindowsStringHasEmbeddedNull(activatibleClassId, &hasEmbedNull)) || hasEmbedNull == TRUE))
    {
#if (NTDDI_VERSION >= NTDDI_WINBLUE)
        WCHAR const pszParamName[] = L"activatibleClassId";
        ::RoOriginateErrorW(E_INVALIDARG, ARRAYSIZE(pszParamName) - 1, pszParamName);
#endif // (NTDDI_VERSION >= NTDDI_WINBLUE)
        return E_INVALIDARG;
    }

    const wchar_t* id = ::WindowsGetStringRawBuffer(activatibleClassId, nullptr);

    auto entry = modulePtr->GetMidEntryPointer() + 1;
    auto last = modulePtr->GetLastEntryPointer();

    // Walk the linker generated list of pointers to CreatorMap for WinRT objects
    for (; entry < last; entry++)
    {
        // Linker generated list can have null pointer values
        if (*entry != nullptr && IsServerNameEqual(*entry, serverName))
        {
            const wchar_t* runtimeName = ((*entry)->activationId.getRuntimeName)();
            __WRL_ASSERT__(runtimeName != nullptr);
            _Analysis_assume_(runtimeName != nullptr);

            if (::wcscmp(id, runtimeName) == 0)
            {
                unsigned int currentFlags = flags;

#pragma warning(push)
// Conditional expression is constant
#pragma warning(disable: 4127)
// Potential comparison of a constant with another constant
#pragma warning(disable: 6326)
                if ((flags & DisableCaching) == 0)
#pragma warning(pop)
                {
                    // Does not require QI
                    return GetCacheEntry(modulePtr, &currentFlags, __uuidof(IActivationFactory), *entry, reinterpret_cast<IUnknown**>(ppFactory));
                }
                else
                {
                    return (*entry)->factoryCreator(&currentFlags, *entry, __uuidof(IActivationFactory), reinterpret_cast<IUnknown**>(ppFactory));
                }
            }
        }
    }

#if (NTDDI_VERSION >= NTDDI_WINBLUE)
    ::RoOriginateError(CLASS_E_CLASSNOTAVAILABLE, activatibleClassId);
#endif // (NTDDI_VERSION >= NTDDI_WINBLUE)
    return CLASS_E_CLASSNOTAVAILABLE;
}

// Activation callback function that is called when object is requested by WinRT API
template<unsigned int flags>
inline HRESULT STDAPICALLTYPE ActivationFactoryCallback(_In_opt_ HSTRING activationId, _COM_Outptr_ IActivationFactory **ppFactory) throw()
{
    auto modulePtr = ::Microsoft::WRL::GetModuleBase();
    __WRL_ASSERT__(modulePtr != nullptr);

    return GetActivationFactory<flags>(modulePtr, nullptr, activationId, ppFactory);
}

template<unsigned int flags>
inline HRESULT RegisterWinRTObject(_In_opt_z_ const wchar_t*, _In_reads_(count) _Deref_pre_z_ const wchar_t** activatableClassIds, _Inout_ RO_REGISTRATION_COOKIE* cookie, unsigned int count) throw()
{
    PFNGETACTIVATIONFACTORY* activationFactoryCallbacks = new (std::nothrow) PFNGETACTIVATIONFACTORY[count];
    HSTRING* activatableClassIdsHstring = new (std::nothrow) HSTRING[count];
    HRESULT hr = S_OK;

    if (activationFactoryCallbacks == nullptr || activatableClassIdsHstring == nullptr)
    {
        hr = E_OUTOFMEMORY;
    }

    if (SUCCEEDED(hr))
    {
        unsigned int index = 0;
        for(;index < count && SUCCEEDED(hr); index++)
        {
            activationFactoryCallbacks[index] = &Details::ActivationFactoryCallback<flags>;
            hr = ::WindowsCreateString(activatableClassIds[index], static_cast<UINT32>(::wcslen(activatableClassIds[index])), &activatableClassIdsHstring[index]);
        }

        if (SUCCEEDED(hr))
        {
            hr = ::Windows::Foundation::RegisterActivationFactories(activatableClassIdsHstring, activationFactoryCallbacks, count, cookie);
        }

        for (unsigned int i = 0; i < index; i++)
        {
            ::WindowsDeleteString(activatableClassIdsHstring[i]);
        }
    }

    delete [] activationFactoryCallbacks;
    delete [] activatableClassIdsHstring;

    return hr;
}

#endif //WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM) */
#pragma endregion

template<unsigned int comFlags>
inline HRESULT RegisterCOMObject(_In_opt_z_ const wchar_t*, _In_reads_(count) IID* clsids, _In_reads_(count) IClassFactory** factories, _Inout_updates_(count) DWORD* cookies, unsigned int count) throw()
{
    HRESULT hr = S_OK;
    unsigned int index = 0;

    for (; index < count && SUCCEEDED(hr); index++)
    {
        hr = ::CoRegisterClassObject(clsids[index], factories[index], CLSCTX_LOCAL_SERVER, comFlags | REGCLS_SUSPENDED, &cookies[index]);
    }

    if (SUCCEEDED(hr))
    {
        //Resume all registered objects
        hr = ::CoResumeClassObjects();
    }

    // Unregister all objects that were already registered
    if (FAILED(hr))
    {
        for (unsigned int i = 0; i < index; i++)
        {
            ::CoRevokeClassObject(cookies[i]);
            cookies[i] = 0;
        }
    }

   return hr;
}

_Ret_range_(<=, end - first) inline unsigned int CountObjectEntries(_In_reads_(end - first)  const CreatorMap** first, _In_ const CreatorMap** end, _In_opt_z_ const wchar_t* serverName) throw()
{
    unsigned int count = 0;

    for (const CreatorMap** entry = first + 1; entry < end; entry++)
    {
        if (*entry != nullptr && IsServerNameEqual(*entry, serverName))
        {
            count++;
        }
    }

    return count;
}

template<unsigned int flags>
inline HRESULT RegisterObjects(_In_ ModuleBase* modulePtr, _In_opt_z_ const wchar_t* serverName) throw()
{
    HRESULT hr = S_OK;

    auto firstEntry = modulePtr->GetFirstEntryPointer();
    auto midEntry = modulePtr->GetMidEntryPointer();

    // Count how many COM objects are in the map
    unsigned int objectCount = CountObjectEntries(firstEntry, midEntry, serverName);

    // COM entries
    if (objectCount > 0)
    {
        // Allocate memory for temporary cookie, factory and clsid's arrays
        DWORD* cookies = new(std::nothrow) DWORD[objectCount];
        IClassFactory** factories = new (std::nothrow) IClassFactory*[objectCount];
        IID* clsids = new (std::nothrow) IID[objectCount];

        if (cookies == nullptr || factories == nullptr || clsids == nullptr)
        {
            hr = E_OUTOFMEMORY;
        }

        if (SUCCEEDED(hr))
        {
            unsigned int index = 0;
            // Instantiate factories and copy clsid to temporary storage
            for (const CreatorMap** entry = firstEntry + 1; entry < midEntry && SUCCEEDED(hr); entry++)
            {
                if (*entry != nullptr && IsServerNameEqual(*entry, serverName))
                {
                    unsigned int currentFlags = flags;
                    IUnknown* factory = nullptr;
                    hr = (*entry)->factoryCreator(&currentFlags, *entry, __uuidof(IClassFactory), &factory);
                    if (SUCCEEDED(hr))
                    {
                        factories[index] = reinterpret_cast<IClassFactory*>(factory);
                        clsids[index] = *(*entry)->activationId.clsid;
                        index++;
                    }
                }
            }

            if (SUCCEEDED(hr))
            {
                // Register COM objects
                hr = modulePtr->RegisterCOMObject(serverName, clsids, factories, cookies, objectCount);
                if (SUCCEEDED(hr))
                {
                    // Store COM cookies in WRL map
                    index = 0;
                    for (const CreatorMap** entry = firstEntry + 1; entry < midEntry; entry++)
                    {
                        if (*entry != nullptr && IsServerNameEqual(*entry, serverName))
                        {
                            (*entry)->factoryCache->cookie.com = cookies[index];
                            index++;
                        }
                    }
                }
            }

            // Release local copy of factories
            for (unsigned int i = 0; i < index; i++)
            {
                factories[i]->Release();
            }
        }

        delete [] cookies;
        delete [] clsids;
        delete [] factories;
    }

#if defined(_DEBUG) && !defined(__WRL_WINRT_STRICT__)
    else
    {
        ::OutputDebugStringW(L"No COM object defined. You may optimize your binary by using __WRL_WINRT_STRICT__ define");
    }
#endif

    // WinRT entries
    if (SUCCEEDED(hr))
    {
        auto lastEntry = modulePtr->GetLastEntryPointer();

        // Count how many WinRT objects are in the map
        objectCount = CountObjectEntries(midEntry, lastEntry, serverName);

        if (objectCount > 0)
        {
            // Create local storage for activatable class ids
            const wchar_t** activatableClassIds = new (std::nothrow) const wchar_t*[objectCount];

            if (activatableClassIds == nullptr)
            {
                hr = E_OUTOFMEMORY;
            }

            if (SUCCEEDED(hr))
            {
                RO_REGISTRATION_COOKIE cookie = nullptr;
                // Copy activatable class ids from WRL creator map
                unsigned int classCount = 0;
                for (const CreatorMap** entry = midEntry + 1; entry < lastEntry; entry++)
                {
                    if (*entry != nullptr && IsServerNameEqual(*entry, serverName))
                    {
                        const wchar_t* id = ((*entry)->activationId.getRuntimeName)();
                        __WRL_ASSERT__(id != nullptr);
                        activatableClassIds[classCount] = id;
                        classCount++;
                    }
                }

                hr = modulePtr->RegisterWinRTObject(serverName, activatableClassIds, &cookie, classCount);
                if (SUCCEEDED(hr))
                {
                    // Copy cookie to the map
                    for (const CreatorMap** entry = midEntry + 1; entry < lastEntry; entry++)
                    {
                        if (*entry != nullptr && IsServerNameEqual(*entry, serverName))
                        {
                             (*entry)->factoryCache->cookie.winrt = cookie;
                        }
                    }
                }
            }

            delete [] activatableClassIds;
        }
#if defined(_DEBUG) && !defined(__WRL_CLASSIC_COM_STRICT__)
        else
        {
            ::OutputDebugStringW(L"No WinRT object defined. You may optimize your binary by using __WRL_CLASSIC_COM_STRICT__ define");
        }
#endif
    }

    return hr;
}

inline HRESULT UnregisterObjects(_In_ ModuleBase* modulePtr, _In_opt_z_ const wchar_t* serverName) throw()
{
    HRESULT hr = S_OK;

    auto firstEntry = modulePtr->GetFirstEntryPointer();
    auto midEntry = modulePtr->GetMidEntryPointer();

    // Count how many COM objects are in the map
    unsigned int objectCount = CountObjectEntries(firstEntry, midEntry, serverName);

    //COM entries
    if (objectCount > 0)
    {
        // Allocate temporary array for COM cookies
        DWORD* cookies = new(std::nothrow) DWORD[objectCount];

        if (cookies == nullptr)
        {
            hr = E_OUTOFMEMORY;
        }

        if (SUCCEEDED(hr))
        {
            // Copy all COM cookies to temporary array
            unsigned int index = 0;
            for (const CreatorMap** entry = firstEntry + 1; entry < midEntry; entry++)
            {
                _Analysis_assume_(index < objectCount);
                if (*entry != nullptr && IsServerNameEqual(*entry, serverName))
                {
                    cookies[index] = (*entry)->factoryCache->cookie.com;
                    index++;
                }
            }
            _Analysis_assume_(index == objectCount);

            // Unregister COM objects
            hr = modulePtr->UnregisterCOMObject(serverName, cookies, objectCount);

            // Copy all cookies back to WRL entry map
            index = 0;
            for (const CreatorMap** entry = firstEntry + 1; entry < midEntry; entry++)
            {
                _Analysis_assume_(index < objectCount);
                if (*entry != nullptr && IsServerNameEqual(*entry, serverName))
                {
                    (*entry)->factoryCache->cookie.com = cookies[index];
                    index++;
                }
            }
        }

        delete [] cookies;
    }

    // WinRT entries
    if (SUCCEEDED(hr))
    {
        RO_REGISTRATION_COOKIE cookie = { 0 };
        bool foundCookie = false;
        auto lastEntry = modulePtr->GetLastEntryPointer();

        // Get the cookie for the server, all cookies are the same for specific server thus it's enough to find first entry and abort
        for (const CreatorMap** entry = midEntry + 1; entry < lastEntry; entry++)
        {
            if (*entry != nullptr && IsServerNameEqual(*entry, serverName))
            {
                 cookie = (*entry)->factoryCache->cookie.winrt;
                 foundCookie = true;
                 break;
            }
        }

        // If valid cookie for the server found WRL can unregister WinRt objects
        if (foundCookie)
        {
            // Unregister WinRt objects
            hr = modulePtr->UnregisterWinRTObject(serverName, cookie);

            if (SUCCEEDED(hr))
            {
                // Reset cookies if unregister succeeded
                for (const CreatorMap** entry = midEntry + 1; entry < lastEntry; entry++)
                {
                    if (*entry != nullptr && IsServerNameEqual(*entry, serverName))
                    {
                         (*entry)->factoryCache->cookie.winrt = 0;
                    }
                }
            }
        }
    }

    // Release all factories
    TerminateMap(modulePtr, serverName, false);
    return hr;
}

#pragma warning(push)
// PREFAST: *ppvObject' might be '0': this does not adhere to the specification for the function
#pragma warning(disable: 6387)

// Class factory creator function is templated on factory type instantiate the factory according to flags settings
// that changes caching/ref counting behavior. There are following scenarios available:
// 1) User is creating factory with Make<ClassFactory> than:
//       - no object counting on Module<> is enabled if user want to lock server it's required to call LockServer method on factory
//       - caching disabled
// 2) OutOfProc server
//       - no object counting on Module<> is enabled
//       - caching disabled
//       - DisableCaching flag doesn't change behavior
// 3) InProc server
//       - caching enabled
//       - object counting on Module<> is enabled
//       - when ref count in factory AddRef reaches 2 WRL increments object count
//       - when ref count in factory Release reaches 1 WRL decrements object count
// 4) InProc | DisabledCaching
//       - caching disabled
//       - object counting on Module<> is enabled
//       - object count is incremented when CreateClassFactory function create factory
//       - object count is decremented when ref count on factory Release reaches 0
//
// When caching is enabled CreateClassFactory function always return ppFactory with ref count 2 otherwise 1
template<typename Factory>
inline HRESULT STDMETHODCALLTYPE CreateClassFactory(_In_ unsigned int *flags, _In_ const CreatorMap*, REFIID riid, _Outptr_ IUnknown **ppFactory) throw()
{
    static_assert(__is_base_of(IClassFactory, Factory), "'Factory' must inherit from 'IClassFactory'");
    static_assert(__is_base_of(FactoryBase, Factory), "'Factory' must inherit from '::Microsoft::WRL::ClassFactory'");

    // Set factory flags that will enable/disable caching behavior depending on flags specified on factories
    switch(Factory::cacheFlag)
    {
        case FactoryCacheFlags::FactoryCacheEnabled:
            if ((*flags & DisableCaching) != 0)
            {
                __WRL_ASSERT__("Mismatched Module<> and 'Factory' configuration. 'Factory' is cacheable and Module<> doesn't support caching");
                *flags |= DisableCaching;
            }
            else
            {
                *flags &= ~DisableCaching;
            }
            break;
        case FactoryCacheFlags::FactoryCacheDisabled:
            *flags |= DisableCaching;
            break;
        default:
            break;
    }

    ComPtr<Factory> classFactory;
    HRESULT hr = MakeAndInitialize<Factory>(classFactory.GetAddressOf());
    if (FAILED(hr))
    {
        return hr;
    }

    classFactory.Get()->flags_ = *flags;

    hr = classFactory.CopyTo(riid, reinterpret_cast<void**>(ppFactory));
    if ((*flags & InProc) != 0)
    {
        if (SUCCEEDED(hr))
        {
            if ((*flags & DisableCaching) != 0)
            {
                auto modulePtr = ::Microsoft::WRL::GetModuleBase();
                __WRL_ASSERT__(modulePtr != nullptr);
                // Need increment object count in case the object is not cached
                modulePtr->IncrementObjectCount();
            }
            else
            {
                // Make sure that we will not call release on factory that will cause
                // decrement object and call to release callback in out of proc server
                classFactory.Detach();
            }
        }
        else
        {
            // In case QI fail make sure that ClassFactory will not decrement object count
            classFactory.Get()->flags_ &= ~(InProc | DisableCaching);
        }
    }

    return hr;
}

// Activation factory creator function is templated on factory type. Instantiate the factory according to flags settings
// that changes caching/ref counting behavior. There are following scenarios available:
// 1) User is creating factory with Make<ActivationFactory> than:
//       - object counting on Module<> is enabled if Module<> was instantiated
//       - object count is incremented in constructor of ActivationFactory if Module<> was instantiated
//       - object count is decremented in Release when ref count on factory is equal 0 if Module<> was instantiated
//       - no caching
// 2) OutOfProc server
//       - object counting on Module<> is enabled
//       - caching enabled
//       - when ref count in factory AddRef reaches 2 WRL increments object count
//       - when ref count in factory Release reaches 1 WRL decrements object count
// 3) OutOfProc | DisabledCaching server
//       - object counting on Module<> is enabled
//       - caching disabled
//       - object count is incremented in constructor of ActivationFactory
//       - object count is decremented in Release when ref count on factory is equal 0
// 4) InProc server
//       - caching enabled
//       - object counting on Module<> is enabled
//       - when ref count in factory AddRef reaches 2 WRL increments object count
//       - when ref count in factory Release reaches 1 WRL decrements object count
// 5) InProc | DisabledCaching
//       - caching disabled
//       - object counting on Module<> is enabled
//       - object count is incremented in constructor of ActivationFactory
//       - object count is decremented when ref count on factory Release reaches 0
//
// When caching is enabled CreateActivationFactory function always return ppFactory with ref count 2 otherwise 1
template<typename Factory>
inline HRESULT STDMETHODCALLTYPE CreateActivationFactory(_In_ unsigned int *flags, _In_ const CreatorMap* entry, REFIID riid, _Outptr_ IUnknown **ppFactory) throw()
{
    static_assert(__is_base_of(IActivationFactory, Factory), "'Factory' must inherit from 'IActivationFactory'");
    static_assert(__is_base_of(FactoryBase, Factory), "'Factory' must inherit from '::Microsoft::WRL::IActivationFactory'");

    // Set factory flags that will enable/disable caching behavior depending on flags specified on factories
    switch(Factory::cacheFlag)
    {
        case FactoryCacheFlags::FactoryCacheEnabled:
            if ((*flags & DisableCaching) != 0)
            {
                __WRL_ASSERT__("Mismatched Module<> and 'Factory' configuration. 'Factory' is cacheable and Module<> doesn't support caching");
                *flags |= DisableCaching;
            }
            else
            {
                *flags &= ~DisableCaching;
            }
            break;
        case FactoryCacheFlags::FactoryCacheDisabled:
            *flags |= DisableCaching;
            break;
        default:
            break;
    }

    ComPtr<Factory> activationFactory;
    HRESULT hr = MakeAndInitialize<Factory>(activationFactory.GetAddressOf());
    if (FAILED(hr))
    {
        return hr;
    }

    // QI without calling AddRef because we incremented object count in constructor of Factory
    // The factory always implements IActivationFactory thus this should succeed always because riid is __uuidof(IActivationFactory)
    hr = activationFactory->CanCastTo(riid, reinterpret_cast<void**>(ppFactory));
    if (FAILED(hr))
    {
        return hr;
    }

    // When cache is enabled it's required to increment ref count to 2 for ActivationFactory
    // because CanCastTo didn't do it
    if ((*flags & DisableCaching) == 0)
    {
        activationFactory->InternalAddRef();
    }

    // Set factory flags that will enable/disable caching behavior
    activationFactory.Get()->flags_ = *flags;
    // Make sure that entry is nulled
    __WRL_ASSERT__(activationFactory.Get()->entry_ == nullptr);
    // Assign entry information to get corresponding runtime class name and trust level
    activationFactory.Get()->entry_ = const_cast<CreatorMap*>(entry);
    // Detach factory if CanCastTo succeeded
    activationFactory.Detach();

    return S_OK;
}

#pragma warning(pop) // C6387

#ifdef _DEBUG
template<typename T>
inline void CheckForDuplicateEntries(const CreatorMap** firstEntry, const CreatorMap** lastEntry, T validateEntry) throw()
{
    __WRL_ASSERT__(firstEntry <= lastEntry);
    if (firstEntry == lastEntry)
    {
        return;
    }

    for (const CreatorMap** entry = firstEntry; entry < (lastEntry - 1); entry++)
    {
        if (*entry == nullptr)
        {
            continue;
        }
        // Walk the linker generated list of pointers to CreatorMap
        for (const CreatorMap** entry2 = (entry + 1); entry2 < lastEntry; entry2++)
        {
            if (*entry2 != nullptr)
            {
                (validateEntry)(*entry, *entry2);
            }
        }
    }
}
#endif // _DEBUG

} // namespace Details

#pragma warning(push)
// PREFast cannot see through template instantiation for AsIID()
#pragma warning(disable: 6388)


// ClassFactory implementation provides registration methods and basic functionality for IClassFactory interface
// It enables developer to provide custom factory implementation.
// Example:
// struct MyClassFactory : public IClassFactory<IMyAddtionalInterfaceOnFactory>
// {
//        STDMETHOD(CreateInstance)(_Inout_opt_ IUnknown* pUnkOuter, REFIID riid, _Outptr_result_nullonfailure_ void** ppvObject)
//        {
//            my custom implementation
//        }
// };
// CoCreatableClassWithFactory(MyClass, MyClassFactory)
//
// When more then 3 interfaces are required to be implemented on factory than:
// struct MyFactory : ClassFactory<Implements<I1, I2, I3>, I4, I5>
template <typename I0 = Details::Nil, typename I1 = Details::Nil, typename I2 = Details::Nil, FactoryCacheFlags cacheFlagValue = FactoryCacheDefault>
class ClassFactory :
    public Details::RuntimeClass<typename Details::InterfaceListHelper<IClassFactory, I0, I1, I2, Details::Nil>::TypeT, RuntimeClassFlags<ClassicCom | InhibitWeakReference>, false>,
    private Details::FactoryBase
{
private:
    static const unsigned int cacheFlag = cacheFlagValue;
public:
    ClassFactory() throw() : flags_(DisableCaching)
    {
    }

    // IUnknown methods
    STDMETHOD_(ULONG, AddRef)()
    {
        auto refcount = Super::InternalAddRef();

        // Increment object count only when InProc and caching enabled
        if ((flags_ & (OutOfProc | DisableCaching)) == 0 && refcount == 2)
        {
            auto modulePtr = ::Microsoft::WRL::GetModuleBase();
            __WRL_ASSERT__(modulePtr != nullptr);

            if (modulePtr != nullptr)
            {
                modulePtr->IncrementObjectCount();
            }
        }

        return refcount;
    }

    STDMETHOD_(ULONG, Release)()
    {
        auto refcount = Super::InternalRelease();

        if (refcount == 0)
        {
            bool isInProcWithoutCaching = (flags_ & (InProc | DisableCaching)) == (InProc | DisableCaching);
            delete this;

            // Decrement object count only when InProc without caching
            if (isInProcWithoutCaching)
            {
                auto modulePtr = ::Microsoft::WRL::GetModuleBase();
                __WRL_ASSERT__(modulePtr != nullptr);

                if (modulePtr != nullptr)
                {
                    modulePtr->DecrementObjectCount();
                }
            }
        }
        // Decrement object count when InProc and caching enabled
        else if ((flags_ & (OutOfProc | DisableCaching)) == 0 && refcount == 1)
        {
            auto modulePtr = ::Microsoft::WRL::GetModuleBase();
            __WRL_ASSERT__(modulePtr != nullptr);

            if (modulePtr != nullptr)
            {
                modulePtr->DecrementObjectCount();
            }
        }

        return refcount;
    }

    STDMETHOD(QueryInterface)(REFIID riid, _Outptr_result_nullonfailure_ void **ppvObject)
    {
        return Super::AsIID(this, riid, ppvObject);
    }

    // IClassFactory method
    STDMETHOD(LockServer)(BOOL fLock)
    {
        auto modulePtr = ::Microsoft::WRL::GetModuleBase();
        if (modulePtr == nullptr)
        {
#if (NTDDI_VERSION >= NTDDI_WINBLUE)
            ::RoOriginateError(E_FAIL, nullptr);
#endif // (NTDDI_VERSION >= NTDDI_WINBLUE)
            return E_FAIL;
        }

        if (fLock)
        {
            modulePtr->IncrementObjectCount();
        }
        else
        {
            modulePtr->DecrementObjectCount();
        }

        return S_OK;
    }

    // Factory creation mechanism is internal for WRL thus declared outside factory as friend
    template<typename Factory> friend HRESULT STDMETHODCALLTYPE Details::CreateClassFactory(_In_ unsigned int*, _In_ const Details::CreatorMap*, REFIID, _Outptr_ IUnknown **) throw();

protected:
    using Super = Details::RuntimeClass<typename Details::InterfaceListHelper<IClassFactory, I0, I1, I2, Details::Nil>::TypeT, RuntimeClassFlags<ClassicCom | InhibitWeakReference>, false>;

private:
    unsigned int flags_;
};

#pragma warning(pop) // C6388

#pragma warning(push)
// PREFAST: *ppvObject' might be '0': this does not adhere to the specification for the function
#pragma warning(disable: 6387 6388)

// SimpleClassFactory provides basic creation mechanism for base class
// Base class must provide default constructor
// Example:
// ActivatableClassWithFactoryEx(MyClass, SimpleClassFactory<MyClass>, 1)
template<typename Base, FactoryCacheFlags cacheFlagValue = FactoryCacheDefault>
class SimpleClassFactory : public ClassFactory<Details::Nil, Details::Nil, Details::Nil, cacheFlagValue>
{
public:
    // IClassFactory method
    STDMETHOD(CreateInstance)(_Inout_opt_ IUnknown* pUnkOuter, REFIID riid, _Outptr_result_nullonfailure_ void** ppvObject)
    {
#ifdef __WRL_STRICT__
        static_assert(__is_base_of(Details::RuntimeClassBase, Base), "SimpleClassFactory can only instantiate 'Base' that derive from RuntimeClass");
        static_assert((Base::ClassFlags::value & ::Microsoft::WRL::ClassicCom) == ::Microsoft::WRL::ClassicCom,
            "SimpleClassFactory can only instantiate 'Base' that is configured with ClassicCom or WinRtClassicComMix flags");
#endif

        *ppvObject = nullptr;

        if (pUnkOuter != nullptr)
        {
#if (NTDDI_VERSION >= NTDDI_WINBLUE)
            ::RoOriginateError(CLASS_E_NOAGGREGATION, nullptr);
#endif // (NTDDI_VERSION >= NTDDI_WINBLUE)
            return CLASS_E_NOAGGREGATION;
        }

        ComPtr<IUnknown> unk;
        HRESULT hr = MakeAndInitialize<Base>(unk.GetAddressOf());
        if (FAILED(hr))
        {
            return hr;
        }

        return unk.CopyTo(riid, ppvObject);
    }
};

#pragma warning(pop) // C6387

#pragma warning(push)
// PREFast cannot see through template instantiation for AsIID()
#pragma warning(disable: 6388)

// Deprecated, use AgileActivationFactory instead.
template <typename I0 = Details::Nil, typename I1 = Details::Nil, typename I2 = Details::Nil, FactoryCacheFlags cacheFlagValue = FactoryCacheDefault>
class ActivationFactory :
    public Details::RuntimeClass<typename Details::InterfaceListHelper<IActivationFactory, I0, I1, I2, Details::Nil>::TypeT, RuntimeClassFlags<WinRt | InhibitWeakReference | InhibitFtmBase>, false>,
    private Details::FactoryBase
{
private:
    static const unsigned int cacheFlag = cacheFlagValue;
public:
    typedef ActivationFactory ActivationFactoryT;
    typedef I0 FirstInterface;

    ActivationFactory() throw() : entry_(nullptr), flags_(DisableCaching)
    {
        auto modulePtr = ::Microsoft::WRL::GetModuleBase();
        if (modulePtr != nullptr)
        {
            modulePtr->IncrementObjectCount();
        }
    }
    // IUnknown methods
    STDMETHOD_(ULONG, AddRef)()
    {
        auto refcount = Super::InternalAddRef();

        // When caching enabled we increment object count on factory when refcount reaches 2
        if ((flags_ & DisableCaching) == 0 && refcount == 2)
        {
            auto modulePtr = ::Microsoft::WRL::GetModuleBase();
            __WRL_ASSERT__(modulePtr != nullptr);

            if (modulePtr != nullptr)
            {
                modulePtr->IncrementObjectCount();
            }
        }

        return refcount;
    }

    STDMETHOD_(ULONG, Release)()
    {
        auto refcount = Super::InternalRelease();

        if (refcount == 0)
        {
            bool isCacheDisabled = (flags_ & DisableCaching) != 0;
            delete this;

            auto modulePtr = ::Microsoft::WRL::GetModuleBase();
            if (isCacheDisabled && modulePtr != nullptr)
            {
                __WRL_ASSERT__(modulePtr != nullptr);

                modulePtr->DecrementObjectCount();
            }
        }
        // When caching enabled WRL decrement object count on factory when it reaches 1
        else if ((flags_ & DisableCaching) == 0 && refcount == 1)
        {
            auto modulePtr = ::Microsoft::WRL::GetModuleBase();
            __WRL_ASSERT__(modulePtr != nullptr);
            if (modulePtr != nullptr)
            {
                modulePtr->DecrementObjectCount();
            }
        }

        return refcount;
    }

    STDMETHOD(QueryInterface)(REFIID riid, _Outptr_result_nullonfailure_ void **ppvObject)
    {
        return Super::AsIID(this, riid, ppvObject);
    }
    // IInspectable methods
    STDMETHOD(GetIids)(
        _Out_ ULONG *iidCount,
        _When_(*iidCount == 0, _At_(*iids, _Post_null_))
        _When_(*iidCount > 0, _At_(*iids, _Post_notnull_))
        _Result_nullonfailure_ IID **iids)
    {
        return Super::GetImplementedIIDS(this, iidCount, iids);
    }
    // Factory runtime class name is the same as RuntimeClass that it is exposing
    STDMETHOD(GetRuntimeClassName)(_Out_ HSTRING* runtimeName)
    {
        *runtimeName = nullptr;
#if (NTDDI_VERSION >= NTDDI_WINBLUE)
        ::RoOriginateError(E_ILLEGAL_METHOD_CALL, nullptr);
#endif // (NTDDI_VERSION >= NTDDI_WINBLUE)
        return E_ILLEGAL_METHOD_CALL;
    }
    // Factory trust level is the same as RuntimeClass that it is exposing
    STDMETHOD(GetTrustLevel)(_Out_ ::TrustLevel* trustLvl)
    {
        if (entry_ != nullptr)
        {
            *trustLvl = (entry_->getTrustLevel)();
        }
        else
        {
            __WRL_ASSERT__(false && "Use 'InspectableClassStatic' on static ONLY factories or override 'GetTrustLevel' method to set trust level.");
            *trustLvl = ::TrustLevel::FullTrust;
        }

        return S_OK;
    }
    // IActivationFactory method
    STDMETHOD_CHPE_PATCHABLE(ActivateInstance)(_Outptr_result_nullonfailure_ IInspectable **ppvObject)
    {
        *ppvObject = nullptr;
#if (NTDDI_VERSION >= NTDDI_WINBLUE)
        ::RoOriginateError(E_NOTIMPL, nullptr);
#endif // (NTDDI_VERSION >= NTDDI_WINBLUE)
        return E_NOTIMPL;
    }
    // Factory creation mechanism is internal for WRL thus declared outside factory as friend
    template<typename Factory> friend HRESULT STDMETHODCALLTYPE Details::CreateActivationFactory(_In_ unsigned int*, _In_ const Details::CreatorMap*, REFIID, _Outptr_ IUnknown **) throw();

protected:
    using Super = Details::RuntimeClass<typename Details::InterfaceListHelper<IActivationFactory, I0, I1, I2, Details::Nil>::TypeT, RuntimeClassFlags<WinRt | InhibitWeakReference | InhibitFtmBase>, false>;

private:
    Details::CreatorMap* entry_;
    unsigned int flags_;
};

// AgileActivationFactory implementation provides registration methods and basic functionality for IActivationFactory interface
// It enables developer to provide custom factory implementation.
// Example:
// struct MyClassFactory : public AgileActivationFactory<IMyAddtionalInterfaceOnFactory>
// {
//     IFACEMETHODIMP ActivateInstance(_COM_Outptr_ IInspectable** result) override
//     {
//         // custom implementation
//     }
// };
// ActivatableClassWithFactory(MyClass, MyClassFactory)
// or if default factory is used
// ActivatableClassWithFactory(MyClass, SimpleActivationFactory<MyClass>)
//
// When more than 3 interfaces are required to be implemented on factory then:
// struct MyFactory : AgileActivationFactory<Implements<I1, I2, I3>, I4, I5>

template <typename I0 = Details::Nil, typename I1 = Details::Nil, typename I2 = Details::Nil, FactoryCacheFlags cacheFlagValue = FactoryCacheDefault>
class AgileActivationFactory : public ActivationFactory<Implements<FtmBase, I0>, I1, I2, cacheFlagValue>
{
};

#pragma warning(pop) // C6388

#pragma warning(push)
// PREFAST: *ppvObject' might be '0': this does not adhere to the specification for the function
#pragma warning(disable: 6387 6388)

// SimpleActivationFactory provides basic creation mechanism for base class
// Base class must provide default constructor
// Example:
// ActivatableClassWithFactoryEx(MyClass, SimpleActivationFactory<MyClass>, 1)
template<typename Base, FactoryCacheFlags cacheFlagValue = FactoryCacheDefault>
class SimpleActivationFactory : public ActivationFactory<Details::Nil, Details::Nil, Details::Nil, cacheFlagValue>
{
public:
    // IActivationFactory method
    STDMETHOD_CHPE_PATCHABLE(ActivateInstance)(_Outptr_result_nullonfailure_ IInspectable **ppvObject)
    {
#ifdef __WRL_STRICT__
        static_assert(__is_base_of(Details::RuntimeClassBase, Base), "SimpleActivationFactory can only instantiate 'Base' that derive from RuntimeClass");
        static_assert((Base::ClassFlags::value & ::Microsoft::WRL::WinRt) == ::Microsoft::WRL::WinRt,
            "SimpleActivationFactory can only instantiate 'Base' that is configured with WinRt or WinRtClassicComMix flags");
#endif

        return MakeAndInitialize<Base>(ppvObject);
    }
};

template<typename Base, FactoryCacheFlags cacheFlagValue = FactoryCacheDefault>
class SimpleSealedActivationFactory WrlFinal : public SimpleActivationFactory<Base, cacheFlagValue>
{
};

// Agile alternative to SimpleActivationFactory
template<typename Base, FactoryCacheFlags cacheFlagValue = FactoryCacheDefault>
class SimpleAgileActivationFactory : public AgileActivationFactory<Details::Nil, Details::Nil, Details::Nil, cacheFlagValue>
{
public:
    STDMETHOD_CHPE_PATCHABLE(ActivateInstance)(_Outptr_result_nullonfailure_ IInspectable **ppvObject)
    {
#ifdef __WRL_STRICT__
        static_assert(__is_base_of(Details::RuntimeClassBase, Base), "SimpleAgileActivationFactory can only instantiate 'Base' that derive from RuntimeClass");
        static_assert((Base::ClassFlags::value & ::Microsoft::WRL::WinRt) == ::Microsoft::WRL::WinRt,
            "SimpleAgileActivationFactory can only instantiate 'Base' that is configured with WinRt or WinRtClassicComMix flags");
#endif

        return MakeAndInitialize<Base>(ppvObject);
    }
};

// Agile alternative to SimpleSealedActivationFactory
template<typename Base, FactoryCacheFlags cacheFlagValue = FactoryCacheDefault>
class SimpleSealedAgileActivationFactory WrlFinal : public SimpleAgileActivationFactory<Base, cacheFlagValue>
{
};

#pragma warning(pop) // C6387

// It's required to #undef following macros because they are always defined in 'wrl/implements.h'
// for better error recognition when somebody forgets to include 'wrl/module.h'
// Please make sure that they are in sync with the version from 'wrl/implements.h'
#undef ActivatableClassWithFactoryEx
#undef ActivatableClassWithFactory
#undef ActivatableClass
#undef ActivatableStaticOnlyFactoryEx
#undef ActivatableStaticOnlyFactory
#undef WrlCreatorMapIncludePragma
#undef WrlCreatorMapIncludePragmaEx

#undef CoCreatableClassWithFactoryEx
#undef CoCreatableClassWithFactory
#undef CoCreatableClass
#undef CoCreatableClassWrlCreatorMapInclude
#undef CoCreatableClassWrlCreatorMapIncludeEx

// Force reference to the symbol so linker does not optimize it away due to /OPT:REF
#pragma comment(linker, "/merge:minATL=.rdata")
#if defined(_M_IX86)
#define WrlCreatorMapIncludePragma(className) __pragma(comment(linker, "/include:___minATLObjMap_" #className));
#define WrlCreatorMapIncludePragmaEx(className, serverName) __pragma(comment(linker, "/include:___minATLObjMap_" #className "_" #serverName));
#else
#define WrlCreatorMapIncludePragma(className) __pragma(comment(linker, "/include:__minATLObjMap_" #className));
#define WrlCreatorMapIncludePragmaEx(className, serverName) __pragma(comment(linker, "/include:__minATLObjMap_" #className "_" #serverName));
#endif

// COM specific
#define CoCreatableClassWrlCreatorMapInclude(className) WrlCreatorMapIncludePragma(className##_COM)
#define CoCreatableClassWrlCreatorMapIncludeEx(className, serverName) WrlCreatorMapIncludePragmaEx(className##_COM, serverName)

#define InternalWrlCreateCreatorMapEx(className, serverName, runtimeClassName, trustLevel, creatorFunction, section) \
    __declspec(selectany) ::Microsoft::WRL::Details::FactoryCache __objectFactory__##className##_##serverName = { nullptr, { 0 } }; \
    extern __declspec(selectany) const ::Microsoft::WRL::Details::CreatorMap __object_##className##_##serverName = { \
        creatorFunction, \
        { runtimeClassName }, \
        trustLevel, \
        &__objectFactory__##className##_##serverName,\
        L## #serverName}; \
    extern "C" __declspec(allocate(section)) __declspec(selectany) const ::Microsoft::WRL::Details::CreatorMap* const __minATLObjMap_##className##_##serverName = &__object_##className##_##serverName; \
    WrlCreatorMapIncludePragmaEx(className, serverName)

#define InternalWrlCreateCreatorMap(className, runtimeClassName, trustLevel, creatorFunction, section) \
    __declspec(selectany) ::Microsoft::WRL::Details::FactoryCache __objectFactory__##className = { nullptr, { 0 } }; \
    extern __declspec(selectany) const ::Microsoft::WRL::Details::CreatorMap __object_##className = { \
        creatorFunction, \
        { runtimeClassName }, \
        trustLevel, \
        &__objectFactory__##className,\
        nullptr}; \
    extern "C" __declspec(allocate(section)) __declspec(selectany) const ::Microsoft::WRL::Details::CreatorMap* const __minATLObjMap_##className = &__object_##className; \
    WrlCreatorMapIncludePragma(className)

// Server name used on ActivatableClassWithFactoryEx or CoCreatableClassWithFactoryEx is used to filter objects registered on the module
// during registration/unregistration or acquiring objects
// The ActivatableClass, ActivatableClassWithFactory, CoCreatableClass and CoCreatableClassWithFactory use serverName id 0.

// Activation macros specific for WinRT
#ifndef __WRL_CLASSIC_COM_STRICT__
#define ActivatableClassWithFactoryEx(className, factory, serverName) \
    InternalWrlCreateCreatorMapEx(className, serverName, reinterpret_cast<const IID*>(&className::InternalGetRuntimeClassName), &className::InternalGetTrustLevel, ::Microsoft::WRL::Details::CreateActivationFactory<factory>, "minATL$__r")

#define ActivatableClassWithFactory(className, factory) \
    InternalWrlCreateCreatorMap(className, reinterpret_cast<const IID*>(&className::InternalGetRuntimeClassName), &className::InternalGetTrustLevel, ::Microsoft::WRL::Details::CreateActivationFactory<factory>, "minATL$__r")

#define ActivatableClass(className) \
    ActivatableClassWithFactory(className, ::Microsoft::WRL::SimpleSealedActivationFactory<className>)

#define AgileActivatableClass(className) \
    ActivatableClassWithFactory(className, ::Microsoft::WRL::SimpleSealedAgileActivationFactory<className>)

#define ActivatableStaticOnlyFactoryEx(factory, serverName) \
    InternalWrlCreateCreatorMapEx(factory, serverName, reinterpret_cast<const IID*>(&factory::InternalGetRuntimeClassNameStatic), &factory::InternalGetTrustLevelStatic, ::Microsoft::WRL::Details::CreateActivationFactory<factory>, "minATL$__r")

#define ActivatableStaticOnlyFactory(factory) \
    InternalWrlCreateCreatorMap(factory, reinterpret_cast<const IID*>(&factory::InternalGetRuntimeClassNameStatic), &factory::InternalGetTrustLevelStatic, ::Microsoft::WRL::Details::CreateActivationFactory<factory>, "minATL$__r")

#define InspectableClassStatic(runtimeClassName, trustLevel) \
    public: \
        static const wchar_t* STDMETHODCALLTYPE InternalGetRuntimeClassNameStatic() throw() \
        { \
            static_assert(__is_base_of(IActivationFactory, ActivationFactoryT) && __is_base_of(::Microsoft::WRL::Details::FactoryBase, ActivationFactoryT), "'InspectableClassStatic' macro can only be used with ::Windows::WRL::ActivationFactory types"); \
            static_assert(!__is_base_of(ActivationFactoryT::FirstInterface, ::Microsoft::WRL::Details::Nil), "ActivationFactory with 'InspectableClassStatic' macro requires to specify custom interfaces"); \
            return runtimeClassName; \
        } \
        static ::TrustLevel STDMETHODCALLTYPE InternalGetTrustLevelStatic() throw() \
        { \
            return trustLevel; \
        } \
        STDMETHOD(GetRuntimeClassName)(_Out_ HSTRING* runtimeName) \
        { \
            *runtimeName = nullptr; \
            return E_ILLEGAL_METHOD_CALL; \
        } \
        STDMETHOD(GetTrustLevel)(_Out_ ::TrustLevel* trustLvl) \
        { \
            *trustLvl = trustLevel; \
            return S_OK; \
        } \
        STDMETHOD(GetIids)(_Out_ ULONG *iidCount, \
            _When_(*iidCount == 0, _At_(*iids, _Post_null_)) \
            _When_(*iidCount > 0, _At_(*iids, _Post_notnull_)) \
            _Result_nullonfailure_ IID **iids) \
        { \
            return ActivationFactoryT::GetIids(iidCount, iids); \
        } \
        STDMETHOD(QueryInterface)(REFIID riid, _Outptr_result_nullonfailure_ void **ppvObject) \
        { \
            return ActivationFactoryT::QueryInterface(riid, ppvObject); \
        } \
        STDMETHOD_(ULONG, Release)() \
        { \
            return ActivationFactoryT::Release(); \
        } \
        STDMETHOD_(ULONG, AddRef)() \
        { \
            return ActivationFactoryT::AddRef(); \
        } \
    private:

#else
// When there is classic com only defined those macros should never be used
#define ActivatableClassWithFactoryEx(className, factory, serverName) \
    static_assert(false, "Activation of COM components. Please make sure that that you either undefine __WRL_WINRT_STRICT__ or remove ActivatableClassWithFactoryEx macro");
#define ActivatableClassWithFactory(className, factory) \
    static_assert(false, "Activation of COM components. Please make sure that that you either undefine __WRL_WINRT_STRICT__ or remove ActivatableClassWithFactory macro");
#define ActivatableClass(className) \
    static_assert(false, "Activation of COM components. Please make sure that that you either undefine __WRL_WINRT_STRICT__ or remove ActivatableClass macro");
#define ActivatableStaticOnlyFactoryEx(factory, serverName) \
    static_assert(false, "Activation of COM components. Please make sure that that you either undefine __WRL_WINRT_STRICT__ or remove  macro");
#define ActivatableStaticOnlyFactory(factory) \
    static_assert(false, "Activation of COM components. Please make sure that that you either undefine __WRL_WINRT_STRICT__ or remove ActivatableStaticOnlyFactory macro");
#endif

// Activation macros specific for COM
#ifndef __WRL_WINRT_STRICT__
#define CoCreatableClassWithFactoryEx(className, factory, serverName) \
    InternalWrlCreateCreatorMapEx(className##_COM, serverName, &__uuidof(className), nullptr, ::Microsoft::WRL::Details::CreateClassFactory<factory>, "minATL$__f")

#define CoCreatableClassWithFactory(className, factory) \
    InternalWrlCreateCreatorMap(className##_COM, &__uuidof(className), nullptr, ::Microsoft::WRL::Details::CreateClassFactory<factory>, "minATL$__f")

#define CoCreatableClass(className) \
    CoCreatableClassWithFactory(className, ::Microsoft::WRL::SimpleClassFactory<className>)

#else
// When there is WINRT strict only defined those macros should never be used
#define CoCreatableClassWithFactoryEx(className, factory, serverName) \
    static_assert(false, "Activation of COM components. Please make sure that that you either undefine __WRL_WINRT_STRICT__ or remove CoCreatableClassWithFactoryEx macro");
#define CoCreatableClassWithFactory(className, factory) \
    static_assert(false, "Activation of COM components. Please make sure that that you either undefine __WRL_WINRT_STRICT__ or remove CoCreatableClassWithFactory macro");
#define CoCreatableClass(className) \
    static_assert(false, "Activation of COM components. Please make sure that that you either undefine __WRL_WINRT_STRICT__ or remove CoCreatableClass macro");
#endif

namespace Details
{
// Forwarding declaration of DefaultModule<>
template<ModuleType moduleType>
class DefaultModule;

// Discriminator value for separate static storage instances for each usage. Each usage of StaticStorage
// should have a unique entry.
enum class StorageInstance {
    InProcCreate,
    OutOfProcCreate,
    OutOfProcCallbackBuffer1,
    OutOfProcCallbackBuffer2
};

// StaticStorage instances can be used in place of statics to avoid the DllMain thread
// attach penalty associated with using magic statics in a DLL. StaticStorage *does not* perform static
// initialization. The calling code is responsible for initialization. The type discriminator parameter on
// StaticStorage can further be used to ensure that each instantiation of a template gets a unique
// instance of static storage. If using a template function within a template class, be sure to account
// for both the class & function template parameters in the discriminator.
template <typename StorageT, StorageInstance instance, typename discriminator = int>
class alignas(alignof(StorageT)) StaticStorage
{
    BYTE data_[sizeof(StorageT)];
    bool initialized_;

    static StaticStorage instance_;

public:

    StaticStorage()
        // we can't force static construction ordering, and it's zero - init'd in the binary, so don't explicitly zero - out.
        // : initialized_{},
        // data_{} -
    {
    }

    ~StaticStorage()
    {
        if (initialized_)
        {
            reinterpret_cast<StorageT*>(data_)->~StorageT();
            initialized_ = false;
        }
    };

    static StorageT* Data()
    {
        instance_.initialized_ = true;
        return reinterpret_cast<StorageT*>(instance_.data_);
    }
};

template <typename StorageT, StorageInstance instance, typename discriminator>
__declspec(selectany) StaticStorage<StorageT, instance, discriminator> StaticStorage<StorageT, instance, discriminator>::instance_ = {};

} // namespace Details

// Forwarding declaraton of Module<>
template<ModuleType moduleType, typename ModuleT = Details::DefaultModule<moduleType>>
class Module;

#pragma warning(push)
#pragma warning(disable: 6388) // PREFast cannot see through some template function calls
template<typename ModuleT>
class Module<InProc, ModuleT> :
    public Details::ModuleBase
{
private:
    void VerifyEntries() throw()
    {
        // Walk the linker generated list of pointers to CreatorMap for WinRT objects
        for (const Details::CreatorMap** entry = GetMidEntryPointer() + 1; entry < GetLastEntryPointer(); entry++)
        {
            if (*entry == nullptr)
            {
                continue;
            }

            const wchar_t* name = ((*entry)->activationId.getRuntimeName)();
            (void)(name);
            // Make sure that runtime class name is not nullptr and it has no empty string
            __WRL_ASSERT__(name != nullptr && ::wcslen(name) != 0);
        }

#ifdef _DEBUG
        Details::CheckForDuplicateEntries((GetFirstEntryPointer() + 1), GetMidEntryPointer(),
            [](const Details::CreatorMap* entry, const Details::CreatorMap* entry2) -> void {
                __WRL_ASSERT__(entry->activationId.clsid != entry2->activationId.clsid && "Duplicate CLSID!");
            }
        );

        Details::CheckForDuplicateEntries((GetMidEntryPointer() + 1), GetLastEntryPointer(),
            [](const Details::CreatorMap* entry, const Details::CreatorMap* entry2) -> void {
                __WRL_ASSERT__(::wcscmp((entry->activationId.getRuntimeName)(), (entry2->activationId.getRuntimeName)()) != 0 && "Duplicate runtime class name!");
            }
        );
#endif
    }

// If static initialization is not available there is no need
// to keep isInitialized and StaticInitialize
#ifndef __WRL_DISABLE_STATIC_INITIALIZE__
    static bool StaticInitialize()
    {
        ModuleT::Create();
        return true;
    }
    static bool isInitialized;
#endif
protected:
    Module()
    {
#ifdef _DEBUG
        VerifyEntries();
#endif
    }

    // inproc module should be a single v-table.

    static INIT_ONCE initOnceInProc_;

public:
    virtual ~Module() throw()
    {
        Details::TerminateMap(this, nullptr, true);
#ifndef __WRL_DISABLE_STATIC_INITIALIZE__
        // Needs to be changed to avoid compiler optimization
        isInitialized = false;
#endif
    }

    static ModuleT& Create() throw()
    {
#ifdef __WRL_ENABLE_FUNCTION_STATICS__
        // Enabling function statics may incur a performance penalty, but avoids a dependency
        // on InitOnceExecuteOnce, which may not be available on some older operating systems.
        // This has the disadvantage of requiring thread attach calls to DllMain, which could
        // increase working set.
        static ModuleT moduleSingleton;
        return moduleSingleton;
#else
        typedef Details::StaticStorage<ModuleT, Details::StorageInstance::InProcCreate> InprocInstanceStorage;
        InitOnceExecuteOnce(
            &initOnceInProc_,
            [](PINIT_ONCE, PVOID, PVOID*) -> BOOL
                {
                    new (InprocInstanceStorage::Data()) ModuleT();
                    return TRUE;
                },
            nullptr,
            nullptr);
        return *InprocInstanceStorage::Data();
#endif
    }

    static ModuleT& GetModule() throw()
    {
        return Create();
    }

#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM)
    HRESULT GetActivationFactory(_In_opt_ HSTRING activatibleClassId, _COM_Outptr_ IActivationFactory **ppIFactory, _In_opt_z_ const wchar_t* serverName = nullptr) throw()
    {
        return Details::GetActivationFactory<InProc>(this, serverName, activatibleClassId, ppIFactory);
    }
#endif

    HRESULT GetClassObject(REFCLSID clsid, REFIID riid, _Outptr_result_nullonfailure_ void **ppv, _In_opt_z_ const wchar_t* serverName = nullptr) throw()
    {
        return Details::GetClassObject<InProc>(this, serverName, clsid, riid, ppv);
    }

    bool Terminate(_In_opt_z_ const wchar_t* serverName = nullptr, bool forceTerminate = false) throw()
    {
        return Details::TerminateMap(this, serverName, forceTerminate);
    }

    // Number of active objects in the module
    STDMETHOD_(unsigned long, IncrementObjectCount)()
    {
        // InterlockedIncrement calls _InterlockedIncrement intrinsic thus we call directly _InterlockedIncrement to save the call
        return static_cast<unsigned long>(_InterlockedIncrement(reinterpret_cast<volatile long*>(&objectCount_)));
    }

    STDMETHOD_(unsigned long, DecrementObjectCount)()
    {
        // InterlockedDecrement calls _InterlockedDecrement intrinsic thus we call directly _InterlockedDecrement to save the call
        return static_cast<unsigned long>(_InterlockedDecrement(reinterpret_cast<volatile long*>(&objectCount_)));
    }

    // InProc module doesn't implement any registration API's
    STDMETHOD(RegisterWinRTObject)(_In_opt_z_ const wchar_t*, _In_ _Deref_pre_z_ const wchar_t**, _Inout_ RO_REGISTRATION_COOKIE*, unsigned int)
    {
#if (NTDDI_VERSION >= NTDDI_WINBLUE)
        ::RoOriginateError(E_NOTIMPL, nullptr);
#endif // (NTDDI_VERSION >= NTDDI_WINBLUE)
        return E_NOTIMPL;
    }

    STDMETHOD(UnregisterWinRTObject)(_In_opt_z_ const wchar_t*, _In_ RO_REGISTRATION_COOKIE)
    {
#if (NTDDI_VERSION >= NTDDI_WINBLUE)
        ::RoOriginateError(E_NOTIMPL, nullptr);
#endif // (NTDDI_VERSION >= NTDDI_WINBLUE)
        return E_NOTIMPL;
    }

    STDMETHOD(RegisterCOMObject)(_In_opt_z_ const wchar_t*, _In_ IID*, _In_ IClassFactory**, _Inout_ DWORD*, unsigned int)
    {
#if (NTDDI_VERSION >= NTDDI_WINBLUE)
        ::RoOriginateError(E_NOTIMPL, nullptr);
#endif // (NTDDI_VERSION >= NTDDI_WINBLUE)
        return E_NOTIMPL;
    }

    STDMETHOD(UnregisterCOMObject)(_In_opt_z_ const wchar_t*, _Inout_ DWORD *, unsigned int)
    {
#if (NTDDI_VERSION >= NTDDI_WINBLUE)
        ::RoOriginateError(E_NOTIMPL, nullptr);
#endif // (NTDDI_VERSION >= NTDDI_WINBLUE)
        return E_NOTIMPL;
    }
};

#pragma warning(pop) // C6388

// Trigger static initialization unless explicitly disabled. Static initialization can be disabled
// to avoid dependencies on C++ constructors when using a DllEntry point other than the Crt startup
// entry point.
#ifndef __WRL_DISABLE_STATIC_INITIALIZE__
template<typename ModuleT>
__declspec(selectany) bool Module<InProc, ModuleT>::isInitialized = Module<InProc, ModuleT>::StaticInitialize();
#endif

template<typename ModuleT>
__declspec(selectany) INIT_ONCE Module<InProc, ModuleT>::initOnceInProc_ = {};

#pragma warning(push)
// PREFAST: '*ppIFactory' might not be '0': this does not adhere to the specification for the function
// PREFAST: '*ppv' might not be '0': this does not adhere to the specification for the function
#pragma warning(disable: 6388)

template<typename ModuleT>
class Module<InProcDisableCaching, ModuleT> :
    public Module<InProc, ModuleT>
{
public:
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM)
    HRESULT GetActivationFactory(_In_opt_ HSTRING activatibleClassId, _COM_Outptr_  IActivationFactory **ppIFactory, _In_opt_z_ const wchar_t* serverName = nullptr) throw()
    {
        return Details::GetActivationFactory<InProcDisableCaching>(this, serverName, activatibleClassId, ppIFactory);
    }
#endif

    HRESULT GetClassObject(REFCLSID clsid, REFIID riid, _Outptr_result_nullonfailure_ void **ppv, _In_opt_z_ const wchar_t* serverName = nullptr) throw()
    {
        return Details::GetClassObject<InProcDisableCaching>(this, serverName, clsid, riid, ppv);
    }
};

#pragma warning(pop) // C6388

namespace Details
{

template<typename ModuleT>
class OutOfProcModuleBase :
    public Module<InProc, ModuleT>
{
private:
    // GetObjectCount returns always zero for out of proc servers
    // This method is called in TerminateMap helper function
    STDMETHOD_(unsigned long, GetObjectCount)() const
    {
        return 0;
    }

    template <typename a, typename b>
    struct WrlPair {};

protected:
    // Generic notification handler interface required to fire
    // when the last object on the module was released
    class ReleaseNotifier
    {
    public:
        ReleaseNotifier(bool release) throw() : release_(release)
        {
        }
        virtual ~ReleaseNotifier() throw()
        {
        }
        void Release() throw()
        {
            if (release_)
            {
                delete this;
            }
        }
        virtual void Invoke() throw() = 0;
    private:
        bool release_;
    };
    // Specialization for notify handler made with lambda, functors or pointer to function
    template<typename T>
    class GenericReleaseNotifier : public ReleaseNotifier
    {
    public:
        GenericReleaseNotifier(T callback, bool release) throw() : ReleaseNotifier(release), callback_(callback)
        {
        }
        void Invoke() throw()
        {
            callback_();
        }
    protected:
        T callback_;
    };
    // Specialization for notify handler made with pointer to the method
    template<typename T>
    class MethodReleaseNotifier : public ReleaseNotifier
    {
    public:
        MethodReleaseNotifier(_In_ T* object, _In_ void (T::* method)(), bool release) throw() :
            ReleaseNotifier(release), object_(object), method_(method)
        {
        }
        void Invoke() throw()
        {
            (object_->*method_)();
        }
    protected:
        T* object_;
        void (T::* method_)();
    };

   ReleaseNotifier *releaseNotifier_;

    OutOfProcModuleBase() throw() : releaseNotifier_(nullptr)
    {
    }

    // The initialization functions provided to register notify handler
    // when the module is created with new/delete
    template<typename T>
    HRESULT Initialize(T callback) throw()
    {
        // Module was already initialized
        __WRL_ASSERT__(releaseNotifier_ == nullptr);

        releaseNotifier_ = new(std::nothrow) GenericReleaseNotifier<T>(callback, true);
        if (releaseNotifier_ == nullptr)
        {
            return E_OUTOFMEMORY;
        }
        return S_OK;
    }

    template<typename T>
    HRESULT Initialize(_In_ T* object, _In_ void (T::* method)()) throw()
    {
        // Module was already initialized
        __WRL_ASSERT__(releaseNotifier_ == nullptr);

        releaseNotifier_ = new(std::nothrow) MethodReleaseNotifier<T>(object, method, true);
        if (releaseNotifier_ == nullptr)
        {
            return E_OUTOFMEMORY;
        }
        return S_OK;
    }

    static INIT_ONCE initOnceOutOfProc_;

public:
    virtual ~OutOfProcModuleBase() throw()
    {
        if (releaseNotifier_ != nullptr)
        {
            releaseNotifier_->Release();
            releaseNotifier_ = nullptr;
        }
    }

    static ModuleT& Create() throw()
    {
#ifdef __WRL_ENABLE_FUNCTION_STATICS__
        static ModuleT moduleSingleton;
        return moduleSingleton;
#else
        typedef Details::StaticStorage<ModuleT, Details::StorageInstance::OutOfProcCreate> OutofprocInstanceStorage;
        InitOnceExecuteOnce(
            &initOnceOutOfProc_,
            [](PINIT_ONCE, PVOID, PVOID*) -> BOOL
            {
                new (OutofprocInstanceStorage::Data()) ModuleT();
                return TRUE;
            },
            nullptr,
            nullptr);
        return *OutofprocInstanceStorage::Data();
#endif
    }


    template<typename T>
    static ModuleT& Create(T callback) throw()
    {
        typedef Details::StaticStorage<
            GenericReleaseNotifier<T>,
            Details::StorageInstance::OutOfProcCallbackBuffer1,
            ModuleT> CallbackStorage;

        auto &moduleRef = Create();

        // Module was already initialized
        __WRL_ASSERT__(moduleRef.releaseNotifier_ == nullptr);

        if (moduleRef.releaseNotifier_ == nullptr)
        {
            moduleRef.releaseNotifier_ = new (CallbackStorage::Data()) GenericReleaseNotifier<T>(callback, false);
        }
        return moduleRef;
    }


    template<typename T>
    static ModuleT& Create(_In_ T* object, _In_ void (T::* method)()) throw()
    {
        typedef Details::StaticStorage<
            MethodReleaseNotifier<T>,
            Details::StorageInstance::OutOfProcCallbackBuffer2,
            ModuleT> CallbackStorage;

        auto &moduleRef = Create();

        // Module was already created initialized
        __WRL_ASSERT__(moduleRef.releaseNotifier_ == nullptr);

        if (moduleRef.releaseNotifier_ == nullptr)
        {
            moduleRef.releaseNotifier_ = new (CallbackStorage::Data()) MethodReleaseNotifier<T>(object, method, false);
        }
        return moduleRef;
    }

    static ModuleT& GetModule() throw()
    {
        auto &moduleRef = Create();
        return moduleRef;
    }
};

template<typename ModuleT>
__declspec(selectany) INIT_ONCE OutOfProcModuleBase<ModuleT>::initOnceOutOfProc_ = {};

} // Details

#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_PC_APP) && !WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)

namespace Details
{
// WRL defines private copies of interface because collisions between windows.winmd and *.h files while compiling with /ZW
class __declspec(uuid("4EDB8EE2-96DD-49A7-94F7-4607DDAB8E3C")) __declspec(novtable) IGetActivationFactoryAbiType : public IInspectable
{
public:
    virtual HRESULT STDMETHODCALLTYPE GetActivationFactory(
        /* [in] */ __RPC__in HSTRING activatableClassId,
        /* [out][retval] */ __RPC__deref_out_opt IInspectable **factory) = 0;
};

class __declspec(uuid("518DC408-C077-475B-809E-0BC0C57E4B74")) __declspec(novtable) ICoreApplicationUseCountAbiType : public IInspectable
{
public:
    virtual HRESULT STDMETHODCALLTYPE IncrementApplicationUseCount(void) = 0;
    virtual HRESULT STDMETHODCALLTYPE DecrementApplicationUseCount(void) = 0;

};

class __declspec(uuid("0AACF7A4-5E1D-49DF-8034-FB6A68BC5ED1")) __declspec(novtable) ICoreApplicationAbiType : public IInspectable
{
public:
    virtual HRESULT STDMETHODCALLTYPE Reserved1() = 0;
    virtual HRESULT STDMETHODCALLTYPE Reserved2() = 0;
    virtual HRESULT STDMETHODCALLTYPE Reserved3() = 0;
    virtual HRESULT STDMETHODCALLTYPE Reserved4() = 0;
    virtual HRESULT STDMETHODCALLTYPE Reserved5() = 0;
    virtual HRESULT STDMETHODCALLTYPE Reserved6() = 0;
    virtual HRESULT STDMETHODCALLTYPE Reserved7() = 0;
    virtual HRESULT STDMETHODCALLTYPE Reserved8() = 0;
    virtual HRESULT STDMETHODCALLTYPE RunWithActivationFactories(
        /* [in] */ __RPC__in_opt IGetActivationFactoryAbiType *activationFactoryCallback) = 0;
};

class ActivationHelper :
    public ::Microsoft::WRL::RuntimeClass<
              ::Microsoft::WRL::RuntimeClassFlags< ::Microsoft::WRL::InhibitWeakReference | ::Microsoft::WRL::WinRt>,
              IGetActivationFactoryAbiType, FtmBase>
{
    InspectableClass(nullptr, BaseTrust)
public:
    // add the string if necessary
    STDMETHOD(GetActivationFactory)(_In_ HSTRING activationId, _COM_Outptr_ IInspectable **factory)
    {
        auto modulePtr = ::Microsoft::WRL::GetModuleBase();
        __WRL_ASSERT__(modulePtr != nullptr);

        // App version of out of proc server doesn't use caching at all
        return ::Microsoft::WRL::Details::GetActivationFactory<InProcDisableCaching>(modulePtr, nullptr, activationId, reinterpret_cast< ::IActivationFactory**>(factory));
    }
};

}

template<typename ModuleT>
class Module<OutOfProc, ModuleT> :
    public Details::OutOfProcModuleBase<ModuleT>
{
private:
    // defining type 'Super' for other compilers since '__super' is a VC++-specific language extension
    using Super = Details::OutOfProcModuleBase<ModuleT>;
    ::Microsoft::WRL::ComPtr< Details::ICoreApplicationUseCountAbiType> count_;

protected:
    Module() throw()
    {
    }

public:
    STDMETHOD_(unsigned long, IncrementObjectCount)()
    {
        if (count_ != nullptr)
        {
            count_->IncrementApplicationUseCount();

            return Super::IncrementObjectCount();
        }

        return 0;
    }

    STDMETHOD_(unsigned long, DecrementObjectCount)()
    {
        unsigned long ref = 0;

        if (count_ != nullptr)
        {
            count_->DecrementApplicationUseCount();
            ref = Super::DecrementObjectCount();
            if (ref == 0 && Super::releaseNotifier_)
            {
                Super::releaseNotifier_->Invoke();
            }
        }

        return ref;
    }

    HRESULT RegisterObjects(_In_opt_z_ const wchar_t* serverName = nullptr) throw()
    {
        ComPtr< Details::IGetActivationFactoryAbiType> getActivationFactory;
        ComPtr< Details::ICoreApplicationAbiType> coreApplication;

        // ActivationHelper must be instantiated before we assign count_
        // Otherwise it will prevent CoreApplication from shooting down the process
        HRESULT hr = MakeAndInitialize< Details::ActivationHelper>(getActivationFactory.GetAddressOf());
        if (FAILED(hr))
        {
            return hr;
        }

        hr = ::Windows::Foundation::GetActivationFactory(
            Wrappers::HStringReference(L"Windows.ApplicationModel.Core.CoreApplication").Get(),
                coreApplication.GetAddressOf());

        if (FAILED(hr))
        {
            return hr;
        }

        hr = coreApplication.As(&count_);
        if (FAILED(hr))
        {
            return hr;
        }

        hr = coreApplication->RunWithActivationFactories(getActivationFactory.Get());
        count_ = nullptr;
        return hr;
    }

    HRESULT UnregisterObjects(_In_opt_z_ const wchar_t* serverName = nullptr) throw()
    {
        return S_OK;
    }
};

#else // WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_PC_APP) && !WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)

template<typename ModuleT>
class Module<OutOfProc, ModuleT> :
    public Details::OutOfProcModuleBase<ModuleT>
{
    // defining type 'Super' for other compilers since '__super' is a VC++-specific language extension
    using Super = Details::OutOfProcModuleBase<ModuleT>;
public:
#ifndef __WRL_WINRT_STRICT__
    STDMETHOD(RegisterCOMObject)(_In_opt_z_ const wchar_t* serverName, _In_reads_(count)  IID* clsids, _In_reads_(count)  IClassFactory** factories, _Inout_updates_(count) DWORD* cookies, unsigned int count)
    {
        return Details::RegisterCOMObject<REGCLS_MULTIPLEUSE>(serverName, clsids, factories, cookies, count);
    }

    STDMETHOD(UnregisterCOMObject)(_In_opt_z_ const wchar_t*, _Inout_updates_(count) DWORD* cookies, unsigned int count)
    {
        HRESULT hr = S_OK;

        for (unsigned int i = 0 ; i < count && SUCCEEDED(hr); i++)
        {
            if (cookies[i] != 0)
            {
                hr = ::CoRevokeClassObject(cookies[i]);
                if (SUCCEEDED(hr))
                {
                    cookies[i] = 0;
                }
            }
        }

        return hr;
    }
#else
    STDMETHOD(RegisterCOMObject)(_In_opt_z_ const wchar_t*, _In_ IID*, _In_ IClassFactory**, _Inout_ DWORD*, unsigned int)
    {
        __WRL_ASSERT__(false && "COM components found. Please make sure that that you either undefine __WRL_WINRT_STRICT__ or remove COM components");
        return S_OK;
    }

    STDMETHOD(UnregisterCOMObject)(_In_opt_z_ const wchar_t*, _Inout_ DWORD*, unsigned int)
    {
        __WRL_ASSERT__(false && "COM components found. Please make sure that that you either undefine __WRL_WINRT_STRICT__ or remove COM components");
        return S_OK;
    }
#endif  // __WRL_WINRT_STRICT__

#if (!defined(__WRL_CLASSIC_COM_STRICT__)) && WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM)
    STDMETHOD(RegisterWinRTObject)(_In_opt_z_ const wchar_t* serverName, _In_reads_(count) _Deref_pre_z_ const wchar_t** activatableClassIds, _Inout_ RO_REGISTRATION_COOKIE* cookie, unsigned int count)
    {
        return Details::RegisterWinRTObject<OutOfProc>(serverName, activatableClassIds, cookie, count);
    }


    STDMETHOD(UnregisterWinRTObject)(_In_opt_z_ const wchar_t*, _In_ RO_REGISTRATION_COOKIE cookie)
    {
        ::Windows::Foundation::RevokeActivationFactories(cookie);
        return S_OK;
    }
#else
    STDMETHOD(RegisterWinRTObject)(_In_opt_z_ const wchar_t*, _In_ _Deref_pre_z_ const wchar_t**, _Inout_ RO_REGISTRATION_COOKIE*, unsigned int)
    {
        __WRL_ASSERT__(false && "WinRT components found. Please make sure that that you either undefine __WRL_CLASSIC_COM_STRICT__ or remove WinRT components");
        return S_OK;
    }

    STDMETHOD(UnregisterWinRTObject)(_In_opt_z_ const wchar_t*, _In_ RO_REGISTRATION_COOKIE)
    {
        __WRL_ASSERT__(false && "WinRT components found. Please make sure that that you either undefine __WRL_CLASSIC_COM_STRICT__ or remove WinRT components");
        return S_OK;
    }
#endif // (!defined(__WRL_CLASSIC_COM_STRICT__)) && WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM)

    HRESULT RegisterObjects(_In_opt_z_ const wchar_t* serverName = nullptr) throw()
    {
        return Details::RegisterObjects<OutOfProc>(this, serverName);
    }

    HRESULT UnregisterObjects(_In_opt_z_ const wchar_t* serverName = nullptr) throw()
    {
       return Details::UnregisterObjects(this, serverName);
    }

    STDMETHOD_(unsigned long, IncrementObjectCount)()
    {
        return ::CoAddRefServerProcess();
    }

    STDMETHOD_(unsigned long, DecrementObjectCount)()
    {
        auto ref = ::CoReleaseServerProcess();
        if (ref == 0 && Super::releaseNotifier_)
        {
            Super::releaseNotifier_->Invoke();
        }

        return ref;
    }
};

#pragma warning(push)
// PREFAST: '*ppIFactory' might not be '0': this does not adhere to the specification for the function
// PREFAST: '*ppv' might not be '0': this does not adhere to the specification for the function
#pragma warning(disable: 6388)

template<typename ModuleT>
class Module<OutOfProcDisableCaching, ModuleT> :
    public Module<OutOfProc, ModuleT>
{
public:
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM)
    HRESULT GetActivationFactory(_In_opt_ HSTRING activatibleClassId, _COM_Outptr_  IActivationFactory **ppIFactory, _In_opt_z_ const wchar_t* serverName = nullptr) throw()
    {
        // Those methods are called in context of InProc always
        return Details::GetActivationFactory<InProcDisableCaching>(this, serverName, activatibleClassId, ppIFactory);
    }
#endif

    HRESULT GetClassObject(REFCLSID clsid, REFIID riid, _Outptr_result_nullonfailure_ void **ppv, _In_opt_z_ const wchar_t* serverName = nullptr) throw()
    {
        // Those methods are called in context of InProc always
        return Details::GetClassObject<InProcDisableCaching>(this, serverName, clsid, riid, ppv);
    }

    HRESULT RegisterObjects(_In_opt_z_ const wchar_t* serverName = nullptr) throw()
    {
        return Details::RegisterObjects<OutOfProcDisableCaching>(this, serverName);
    }

#if (!defined(__WRL_CLASSIC_COM_STRICT__)) && WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM)
    STDMETHOD(RegisterWinRTObject)(_In_opt_z_ const wchar_t* serverName, _In_reads_(count) _Deref_pre_z_ const wchar_t** activatableClassIds, _Inout_updates_(count) RO_REGISTRATION_COOKIE* cookies, unsigned int count)
    {
        return Details::RegisterWinRTObject<OutOfProcDisableCaching>(serverName, activatableClassIds, cookies, count);
    }
#endif
};

#pragma warning(pop) // C6388

#endif // WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_PC_APP) && !WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)

namespace Details
{
template <ModuleType moduleType>
class DefaultModule :
    public Module<moduleType, DefaultModule<moduleType>>
{
};
} // namespace Details

}} // namespace Microsoft::WRL

// Restore packing
#include <poppack.h>

#endif // _WRL_MODULE_H_
