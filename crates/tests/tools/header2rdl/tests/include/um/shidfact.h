//
//    Copyright (C) Microsoft.  All rights reserved.
//
#if defined(_MSC_VER) && (_MSC_VER >= 1200)
#pragma once
#endif

#ifndef _INC_SHIDFACT
#define _INC_SHIDFACT

#include <winapifamily.h>

#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)

#ifdef __cplusplus

#include <shobjidl_core.h>  // for IDelegateFolder
#include <propvarutil.h>  // for PropVariantToVariant

template <class T, DWORD Magic>
class CItemIDFactory : public IDelegateFolder
{
protected:
    CItemIDFactory() : _pmalloc(NULL) {}

    // if we have an IMalloc from IDelegateFolder::SetItemAlloc then clean it up

    virtual ~CItemIDFactory()
    {
        if (_pmalloc)
        {
            _pmalloc->Release();
        }
    }

public:
    // IUnknown provided by derived classes

    // IDelegateFolder
    IFACEMETHODIMP SetItemAlloc(_In_opt_ IMalloc *pmalloc)
    {
        IUnknown_Set((IUnknown**)&_pmalloc, pmalloc);
        return S_OK;
    }

    BOOL IsDelegateFolder()
    {
        return (_pmalloc != NULL);
    }

    // get a read only pointer to the client provided structure in the first ItemID in the IDList.
    // returns NULL if the IDList isn't valid.

    static const UNALIGNED T* GetDataFromIDList(_In_opt_ PCUIDLIST_RELATIVE pidl)
    {
        PCITEM pitem = _IsValid(pidl);
        return pitem ? &pitem->innerData : NULL;
    }

    static HRESULT GetDataFromIDList(_In_ PCUIDLIST_RELATIVE pidl, _Outptr_ const UNALIGNED T** ppData)
    {
        *ppData = GetDataFromIDList(pidl);
        return (*ppData) ? S_OK : E_INVALIDARG;
    }

    // return a read only pointer to the serialized property storage that we use for storing metadata

    static PCUSERIALIZEDPROPSTORAGE GetPropertyStorage(_In_opt_ PCUIDLIST_RELATIVE pidl, _Out_ DWORD* pcb)
    {
        PCITEM pitem = _IsValid(pidl);
        if (pitem && pitem->cbPropStore)
        {
            *pcb = pitem->cbPropStore;
            return (PCUSERIALIZEDPROPSTORAGE)(pitem + 1);
        }
        return NULL;
    }

    // return a property from the IPropertyStore within the IDList, convert the resulting PROPVARIANT
    // to a variant (useful when implementing IShellFolder2::GetDetailsEx)
    // return vt == VT_EMPTY if not found

    static HRESULT GetPropertyFromIDList(_In_ PCUIDLIST_RELATIVE pidl, _In_ REFPROPERTYKEY rkey, _Out_ VARIANT *pvar)
    {
        PROPVARIANT pv = {pvar->vt};
        HRESULT hr = GetPropertyFromIDList(pidl, rkey, &pv);
        if (SUCCEEDED(hr))
        {
            hr = PropVariantToVariant(&pv, pvar);
            PropVariantClear(&pv);
        }
        return hr;
    }

    static HRESULT GetPropertyFromIDList(_In_ PCUIDLIST_RELATIVE pidl, _In_ PCWSTR pszName, _Out_ VARIANT *pvar)
    {
        PROPVARIANT pv = {pvar->vt};
        HRESULT hr = GetPropertyFromIDList(pidl, pszName, &pv);
        if (SUCCEEDED(hr))
        {
            hr = PropVariantToVariant(&pv, pvar);
            PropVariantClear(&pv);
        }
        return hr;
    }

    // read a PROPVARIANT from the IPropertyStore within the first ItemID in the IDList.

    static HRESULT GetPropertyFromIDList(_In_ PCUIDLIST_RELATIVE pidl, _In_ REFPROPERTYKEY rkey, _Out_ PROPVARIANT *pv)
    {
        PropVariantInit(pv);
        HRESULT hr = E_INVALIDARG;
        DWORD cb;
        PCUSERIALIZEDPROPSTORAGE psps = GetPropertyStorage(pidl, &cb);
        if (psps)
        {
            hr = PSGetPropertyFromPropertyStorage(psps, cb, rkey, pv);
            if (SUCCEEDED(hr) && (pv->vt == VT_EMPTY))
            {
                hr = HRESULT_FROM_WIN32(ERROR_UNKNOWN_PROPERTY);
            }
        }
        return hr;
    }

    // read a PROPVARIANT from the IPropertyStore within the first ItemID in the IDList using the named property

    static HRESULT GetPropertyFromIDList(_In_ PCUIDLIST_RELATIVE pidl, _In_ PCWSTR pszName, _Out_ PROPVARIANT *pv)
    {
        PropVariantInit(pv);
        HRESULT hr = E_INVALIDARG;
        DWORD cb;
        PCUSERIALIZEDPROPSTORAGE psps = GetPropertyStorage(pidl, &cb);
        if (psps)
        {
            hr = PSGetNamedPropertyFromPropertyStorage(psps, cb, pszName, pv);
            if (SUCCEEDED(hr) && (pv->vt == VT_EMPTY))
            {
                hr = HRESULT_FROM_WIN32(ERROR_UNKNOWN_PROPERTY);
            }
        }
        return hr;
    }

    // create an instance of the IPropertyStore based on the serialized property storage associated
    // with the first ItemID.

    static HRESULT GetPropertyStorageFromIDList(_In_ PCUIDLIST_RELATIVE pidl, _In_ REFIID riid, _Outptr_ void **ppv)
    {
        HRESULT hr = E_INVALIDARG;
        if (pidl && ppv)
        {
            *ppv = NULL;
            DWORD cb;
            PCUSERIALIZEDPROPSTORAGE psps = GetPropertyStorage(pidl, &cb);
            if (psps)
            {
                IPersistSerializedPropStorage* ppsps;
                hr = PSCreateMemoryPropertyStore(IID_PPV_ARGS(&ppsps));
                if (SUCCEEDED(hr))
                {
                    hr = ppsps->SetPropertyStorage(psps, cb);
                    if (SUCCEEDED(hr))
                    {
                        hr = ppsps->SetFlags(FPSPS_READONLY);
                        if (SUCCEEDED(hr))
                        {
                            hr = ppsps->QueryInterface(riid, ppv);
                        }
                    }
                    ppsps->Release();
                }
            }
        }
        return hr;
    }

    // package up the user supplied data into an ItemID.  pinner points to the clients structure
    // that should be copied, and the IPropertyStore is serialized into the ItemID.   If the
    // client has called IDelegateFolder::SetItemAlloc then we will allocate with that
    // allocator, otherwise the COM allocator is used.  the size of the user supplied data must
    // equal sizeof(T). don't use structs with variably allocated array/string members. the
    // struct must also follow standard SHITEMID for persistance and portability. see
    // shtypes.idl for details.
    HRESULT CreateItemID(_In_opt_ const UNALIGNED T *pinner, _Inout_opt_ IPropertyStore *pps, _Outptr_ PITEMID_CHILD *ppidl)
    {
        return s_CreateItemID(pinner, pps, ppidl, _pmalloc);
    }

    static HRESULT s_CreateItemID(_In_opt_ const UNALIGNED T *pinner, _Inout_opt_ IPropertyStore *pps, _Outptr_ PITEMID_CHILD *ppidl, _In_opt_ IMalloc *pMalloc=NULL)
    {
        *ppidl = NULL;

        SERIALIZEDPROPSTORAGE *pspstg = NULL;
        DWORD cbPropStore = 0;

        // do we have a IPropertyStore, if so we are going to add this to the ItemID
        // so we need to get the serialized version of it.

        HRESULT hr = S_OK;
        if (pps)
        {
            IPersistSerializedPropStorage *ppsps;
            hr = pps->QueryInterface(IID_PPV_ARGS(&ppsps));
            if (SUCCEEDED(hr))
            {
                hr = ppsps->GetPropertyStorage(&pspstg, &cbPropStore);
                ppsps->Release();
            }
        }

        // either we succeeded at getting the serialized data, or an IPropertyStore wasn't passed to us.

        if (SUCCEEDED(hr))
        {
            UINT cbInner = sizeof(CHILDITEMID) - (sizeof(DELEGATEITEMID) - 1) + cbPropStore;
            CHILDITEMID *pitem = s_Alloc(cbInner + sizeof(USHORT), pMalloc);
            if (pitem)
            {
                pitem->dwMagic = Magic;
                pitem->cbPropStore = (WORD)cbPropStore;             // size of the property store
                pitem->cbInnerData = sizeof(pitem->innerData);      // size of the "innerData" structure (useful for debugging)

                if (pinner)
                    pitem->innerData = *pinner;                     // copy the structure provided by the user

                if (pspstg)
                {
                    BYTE *pbData = (BYTE*)(pitem+1);
                    memcpy(pbData, pspstg, cbPropStore);            // copy property store to ItemID
                }

                *ppidl = (PITEMID_CHILD)pitem;
                hr = S_OK;
            }
            else
            {
                hr = E_OUTOFMEMORY;
            }

            CoTaskMemFree(pspstg);
        }

        return hr;
    }

private:

    // ItemID form is derived from the delegate folder verison, this allows us to easily
    // switch allocators without having to recompute the ItemID structure dynamically.

    #include <pshpack1.h>
    typedef struct              // typedef struct
    {                           // {
        // these need to line up -----------------------
        WORD cbSize;            //     WORD cbSize;         // Size of entire item ID
        WORD wOuter;            //     WORD wOuter;         // Private data owned by the outer folder
        WORD cbInner;           //     WORD cbInner;        // Size of delegate's data
        // ---------------------------------------------
        DWORD dwMagic;                                      // guard word used to compare to ensure its valid
        WORD cbPropStore;                                   // size of the property store at the end of the ItemID
        WORD cbInnerData;                                   // size of the innerData structure
        T innerData;                                        // inner data returned by the client
    } CHILDITEMID;
    #include <poppack.h>

    typedef UNALIGNED CHILDITEMID * PITEM;
    typedef const UNALIGNED CHILDITEMID * PCITEM;

    // given the size of the ItemID we want, let's allocate using the supplied
    // allocator and initialize the structure

    IMalloc *_pmalloc;                                      // used to allocate ItemID when we are a delegate folder

    static CHILDITEMID* s_Alloc(SIZE_T cbInner, _In_opt_ IMalloc *pMalloc)
    {
        DELEGATEITEMID *pidl;
        if (pMalloc)
        {
            pidl = (DELEGATEITEMID *)pMalloc->Alloc(cbInner);
        }
        else
        {
            SIZE_T cbAlloc =
                sizeof(DELEGATEITEMID) - sizeof(pidl->rgb[0]) + // header
                cbInner +                                       // inner
                sizeof(WORD);                                   // trailing null (pidl terminator)

            const SIZE_T cbSizeMax = (1 << (sizeof(pidl->cbSize)*8)) - 1;
            if (cbAlloc > cbSizeMax + sizeof(WORD))
            {
                // cbSize will overflow
                pidl = NULL;
            }
            else
            {
                pidl = (DELEGATEITEMID *)CoTaskMemAlloc(cbAlloc);
                if (pidl)
                {
                    ZeroMemory(pidl, cbAlloc);
                    pidl->cbSize = (WORD)(cbAlloc - sizeof(WORD));
                    pidl->cbInner = (WORD)cbInner;
                }
            }
        }
        return (CHILDITEMID*)pidl;
    }

    // given a relative IDList, determine if it's valid and therefore if
    // we should continue to process the contents of it.

    static PCITEM _IsValid(_In_opt_ PCUIDLIST_RELATIVE pidl)
    {
        PCITEM pitem = (PCITEM)pidl;

        if (!pitem || pitem->cbSize < sizeof(CHILDITEMID))
            return NULL;

        if ((pitem->dwMagic != Magic) || (pitem->cbSize < (sizeof(CHILDITEMID) + pitem->cbPropStore)))
            return NULL;

        return pitem;
    }
};

#endif  // __cplusplus



#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion

#endif  // _INC_SHIDFACT
