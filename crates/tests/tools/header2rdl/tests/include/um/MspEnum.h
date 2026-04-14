/*

    Copyright (c) Microsoft Corporation. All rights reserved.

*/

#ifndef _MSPENUM_H_
#define _MSPENUM_H_
#include <winapifamily.h>

#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)


//////////////////////////////////////////////////////////////////////////////
//
// CSafeComEnum
//
// All TAPI 3.0 system components and MSPs use the CSafeComEnum class instead
// of ATL 2.1's CComEnum class when implementing enumerator objects that are
// accessible to applications. This is needed for the following reasons:
//
// 1. CComEnum does not perform IsBadWritePtr checks on the pointer arguments
//    to the enumerator methods. This allows the component exposing the
//    enumerator to AV when called with invalid pointer arguments.
//
// 2. CComEnum does not support free thread marshaling, and therefore cannot
//    be used from an apartment threaded application.
//
// Note: No debug tracing is done here, to facilitate use of this template
// independent of the rest of the MSP Base Classes.
//
/////////////////////////////////////////////////////////////////////////////


template <class Base, const IID* piid, class T, class Copy,
          class ThreadModel = CComObjectThreadModel>
class ATL_NO_VTABLE CSafeComEnum :
        public CComEnumImpl<Base, piid, T, Copy>,
        public CComObjectRootEx< ThreadModel >
{
    typedef CSafeComEnum<Base, piid, T, Copy, ThreadModel> ThisClass;
    typedef CComEnumImpl<Base, piid, T, Copy>              BaseClass;

    STDMETHOD(Next)(ULONG celt, T* rgelt, ULONG* pceltFetched)
    {
        //
        // Check if the return array is valid for as many elements as
        // specified. No need to explicitly check if celt is zero here
        // celt itself will be checked in the base class method.
        //

        if ( !rgelt )
        {
            return E_POINTER;
        }

        //
        // if pceltFetched == NULL,this may still be a valid call. pceltFetched == NULL implies that
        // celt should be equal to 1, but that will be checked in the
        // base class method.
        //

        

        //
        // Everything OK so far; proceed with base class method.
        //

        return BaseClass::Next(celt, rgelt, pceltFetched);
    }

    STDMETHOD(Clone)(Base** ppEnum)
    {
        //
        // Check if the return pointer is valid.
        //

        if (!ppEnum)
        {
            return E_POINTER;
        }

        //
        // Everything OK so far; proceed with base class method.
        //

        return BaseClass::Clone(ppEnum);
    }

    //
    // We do not override Skip or Reset as they have no pointer arguments.
    //

    //
    // The rest of this class involves support for free thread marshaling.
    //

    BEGIN_COM_MAP( ThisClass )

		COM_INTERFACE_ENTRY_IID( *piid, BaseClass )
        COM_INTERFACE_ENTRY_AGGREGATE( IID_IMarshal, m_pFTM )

    END_COM_MAP()

    DECLARE_GET_CONTROLLING_UNKNOWN()

    HRESULT Init(T* begin, T* end, IUnknown* pUnk,
            CComEnumFlags flags = AtlFlagNoCopy)
    {
        //
        // We do not check the pointer arguments in this method because this
        // method is not exposed to the application (it is not a COM interface
        // method).
        //
        
        HRESULT hr;

        IUnknown * pIU = GetControllingUnknown();

        hr = CoCreateFreeThreadedMarshaler( pIU, 
                                            & m_pFTM );

        if ( FAILED(hr) )
        {
            return hr;
        }

        return BaseClass::Init(begin, end, pUnk, flags);
    }

    CSafeComEnum()
    {
        m_pFTM = NULL;
    }

    void FinalRelease(void)
    {
        if ( m_pFTM )
        {
            m_pFTM->Release();
        }

        CComObjectRootEx< ThreadModel >::FinalRelease();
    }

protected:
    IUnknown * m_pFTM; // pointer to free thread marshaler
};


#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion

#endif // _MSPENUM_H_
