/*

    Copyright (c) Microsoft Corporation. All rights reserved.

*/

#ifndef _MSPCOLL_H_
#define _MSPCOLL_H_
#include <winapifamily.h>

#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)



////////////////////////////////////////////////////////////////////////
// CTapiIfCollection -- adapted from tapi3 code
//      Collection template for collections of IDispatch interfaces
//
////////////////////////////////////////////////////////////////////////

template <class T> class CTapiIfCollection :
    public IDispatchImpl<ITCollection, &IID_ITCollection, &LIBID_TAPI3Lib>,
    public CComObjectRootEx<CComMultiThreadModelNoCS>
{
public:
    typedef CTapiIfCollection<T> _CTapiCollectionBase;
    
BEGIN_COM_MAP(_CTapiCollectionBase)
    COM_INTERFACE_ENTRY(IDispatch)
    COM_INTERFACE_ENTRY(ITCollection)
END_COM_MAP()

private:

    int                 m_nSize;
    CComVariant *       m_Var;
    
public:

    CTapiIfCollection(void) : m_nSize(0), m_Var(NULL) { }

    // initialize
    HRESULT STDMETHODCALLTYPE Initialize(
                                         DWORD dwSize,
                                         T * pBegin,
                                         T * pEnd                                         
                                        )
    {
        int                     i;
        HRESULT                 hr;
        T *                     iter;

        LOG((MSP_TRACE, "CTapiCollection::Initialize - enter"));

        // create variant array
        m_nSize = dwSize;

        m_Var = new CComVariant[m_nSize];
        if (m_Var == NULL)
        {
            // debug output
            return E_OUTOFMEMORY;
        }

        i = 0;

        for (iter = pBegin; iter != pEnd; iter++)
        {
            // get IDispatch pointer
            IDispatch * pDisp = NULL;

            hr = (*iter)->QueryInterface(IID_IDispatch, (void**)&pDisp);

            if (hr != S_OK)
            {
                return hr;
            }

            // create a variant and add it to the collection
            CComVariant& var = m_Var[i];

            VariantInit(&var);
            
            var.vt = VT_DISPATCH;
            var.pdispVal = pDisp;

            i++;
        }

        LOG((MSP_TRACE, "CTapiCollection::Initialize - exit"));
        
        return S_OK;
    }

    void FinalRelease()
    {
        LOG((MSP_TRACE, "CTapiCollection::FinalRelease - enter"));

        //
        // We "new"ed an array of objects. Delete each object in the array. The
        // destructor for each object calls VariantClear to release the pointer
        // in that object, based on the variant's tag.
        //

        delete [] m_Var;

        LOG((MSP_TRACE, "CTapiCollection::FinalRelease - exit"));
    }
    
    STDMETHOD(get_Count)(
                         long* retval
                        )
    {
        HRESULT         hr = S_OK;
        
        LOG((MSP_TRACE, "CTapiCollection::get_Count - enter"));
        
        try
        {
            *retval = m_nSize;
        }
        catch(...)
        {
            hr = E_INVALIDARG;
        }

        LOG((MSP_TRACE, "CTapiCollection::get_Count - exit"));

        return hr;
    }

    STDMETHOD(get_Item)(
                                       long Index, 
                                       VARIANT* retval
                                      )
    {
        HRESULT         hr = S_OK;

        LOG((MSP_TRACE, "CTapiCollection::get_Item - enter"));
        
        if (retval == NULL)
        {
            return E_POINTER;
        }

        try
        {
            VariantInit(retval);
        }
        catch(...)
        {
            hr = E_INVALIDARG;
        }

        if (hr != S_OK)
        {
            return hr;
        }

        retval->vt = VT_UNKNOWN;
        retval->punkVal = NULL;

        // use 1-based index, VB like
        if ((Index < 1) || (Index > m_nSize))
        {
            return E_INVALIDARG;
        }


        hr = VariantCopy(retval, &m_Var[Index-1]);

        if (FAILED(hr))
        {
            LOG((MSP_ERROR, 
                "CTapiCollection::get_Item - VariantCopy failed. hr = %lx", 
                hr));

            return hr;
        }

        LOG((MSP_TRACE, "CTapiCollection::get_Item - exit"));
        
        return S_OK;
    }

    HRESULT STDMETHODCALLTYPE get__NewEnum(
                                           IUnknown** retval
                                          )
    
    {
        HRESULT         hr;

        LOG((MSP_TRACE, "CTapiCollection::new__Enum - enter"));
        
        if (retval == NULL)
        {
            return E_POINTER;
        }

        *retval = NULL;

        typedef CComObject<CSafeComEnum<IEnumVARIANT, &IID_IEnumVARIANT, VARIANT, _Copy<VARIANT> > > enumvar;

        enumvar* p; // = new enumvar;
        hr = enumvar::CreateInstance( &p );

        if ( FAILED(hr) )
        {
            // debug output
            return hr;
        }

        hr = p->Init(&m_Var[0], &m_Var[m_nSize], NULL, AtlFlagCopy);

        if (SUCCEEDED(hr))
        {
            hr = p->QueryInterface(IID_IEnumVARIANT, (void**)retval);
        }

        if (FAILED(hr))
        {
            delete p;
        }

        LOG((MSP_TRACE, "CTapiCollection::new__Enum - exit"));
        
        return hr;

    }
};

////////////////////////////////////////////////////////////////////////
// CTapiBstrCollection -- adapted from tapi3 code
//    Collection of BSTRs.
////////////////////////////////////////////////////////////////////////
class CTapiBstrCollection :
    public CComObjectRootEx<CComMultiThreadModelNoCS>,
    public IDispatchImpl<ITCollection, &IID_ITCollection, &LIBID_TAPI3Lib>,
    public CMSPObjectSafetyImpl
{
public:
    
BEGIN_COM_MAP(CTapiBstrCollection)
    COM_INTERFACE_ENTRY(IDispatch)
    COM_INTERFACE_ENTRY(ITCollection)
    COM_INTERFACE_ENTRY(IObjectSafety)
END_COM_MAP()

private:

    DWORD               m_dwSize;
    CComVariant *       m_Var;
    
public:

    CTapiBstrCollection(void) : m_dwSize(0), m_Var(NULL) { }

    // initialize
    HRESULT STDMETHODCALLTYPE Initialize(
                                         DWORD dwSize,
                                         BSTR * pBegin,
                                         BSTR * pEnd                                         
                                        )
    {
        BSTR *  i;
        DWORD   dw = 0;

        LOG((MSP_TRACE, "CTapiBstrCollection::Initialize - enter"));

        // create variant array
        m_dwSize = dwSize;

        m_Var = new CComVariant[m_dwSize];

        if (m_Var == NULL)
        {
            // debug output
            return E_OUTOFMEMORY;
        }

        for (i = pBegin; i != pEnd; i++)
        {
            // create a variant and add it to the collection
            CComVariant& var = m_Var[dw];

            var.vt = VT_BSTR;
            var.bstrVal = *i;

            dw++;
        }

        LOG((MSP_TRACE, "CTapiBstrCollection::Initialize - exit"));
        
        return S_OK;
    }
    
    STDMETHOD(get_Count)(
                         long* retval
                        )
    {
        HRESULT         hr = S_OK;

        LOG((MSP_TRACE, "CTapiBstrCollection::get_Count - enter"));        

        try
        {
            *retval = m_dwSize;
        }
        catch(...)
        {
            hr = E_INVALIDARG;
        }

        LOG((MSP_TRACE, "CTapiBstrCollection::get_Count - exit"));
        
        return hr;
    }

    STDMETHOD(get_Item)(
                        long Index, 
                        VARIANT* retval
                       )
    {
        HRESULT         hr = S_OK;

        LOG((MSP_TRACE, "CTapiBstrCollection::get_Item - enter"));
        
        if (retval == NULL)
        {
            return E_POINTER;
        }

        try
        {
            VariantInit(retval);
        }
        catch(...)
        {
            hr = E_INVALIDARG;
        }

        if (hr != S_OK)
        {
            return hr;
        }

        retval->vt = VT_BSTR;
        retval->bstrVal = NULL;

        // use 1-based index, VB like
        // no problem with signed/unsigned, since
        // if Index < 0 then first clause is true, making it
        // irrelevant if the second clause is correct or not.

        if ((Index < 1) || ( (DWORD) Index > m_dwSize))
        {
            return E_INVALIDARG;
        }

        //
        // This copies the string, not just the pointer.
        //

        hr = VariantCopy(retval, &m_Var[Index-1]);

        if (FAILED(hr))
        {
            LOG((MSP_ERROR, 
                "CTapiBstrCollection::get_Item - VariantCopy failed. hr = %lx", 
                hr));

            return hr;
        }


        LOG((MSP_TRACE, "CTapiBstrCollection::get_Item - exit"));

        return S_OK;
    }

    HRESULT STDMETHODCALLTYPE get__NewEnum(
                                           IUnknown** retval
                                          )
    
    {
        HRESULT         hr;

        LOG((MSP_TRACE, "CTapiBstrCollection::get__NumEnum - enter"));
        
        if (retval == NULL)
        {
            return E_POINTER;
        }

        *retval = NULL;

        typedef CComObject<CSafeComEnum<IEnumVARIANT, &IID_IEnumVARIANT, VARIANT, _Copy<VARIANT> > > enumvar;

        enumvar* p = new enumvar;

        if ( p == NULL)
        {
            // debug output
            return E_OUTOFMEMORY;
        }

        hr = p->Init(&m_Var[0], &m_Var[m_dwSize], NULL, AtlFlagCopy);

        if (SUCCEEDED(hr))
        {
            hr = p->QueryInterface(IID_IEnumVARIANT, (void**)retval);
        }

        if (FAILED(hr))
        {
            delete p;
        }

        LOG((MSP_TRACE, "CTapiBstrCollection::get__NewEnum - exit"));
        
        return hr;

    }

    void FinalRelease()
    {
        LOG((MSP_TRACE, "CTapiBstrCollection::FinalRelease() - enter"));

        //
        // We "new"ed an array of objects. Delete each object in the array. The
        // destructor for each object calls VariantClear to release the pointer
        // in that object, based on the variant's tag.
        //

        delete [] m_Var;

        LOG((MSP_TRACE, "CTapiBstrCollection::FinalRelease() - exit"));
    }

};


#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion

#endif // _MSPCOLL_H_

// eof
