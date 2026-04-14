/*++

Copyright (c) Microsoft Corporation. All rights reserved.

Module Name:

    MSPaddr.h

Abstract:

Declaration of the CMSPAddress

--*/

#ifndef __MSPADDR_H_
#define __MSPADDR_H_
#include <winapifamily.h>

#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)



typedef struct 
{
   LIST_ENTRY       Link;           // The link node. See ntrtl.h for detail.
   MSP_EVENT_INFO   MSPEventInfo;   // The event code.
   
} MSPEVENTITEM, *PMSPEVENTITEM;


//
// these functions should be used to allocate and deallocate MSPEVENTITEM
// structures. In case of failure, the caller can call GetLastError()
// to get exact cause of the failure.
//

//
// nExtraBytes specifies how many extra (in addition to sizeof(MSPEVENTITEM)) 
// bytes to allocate.
//

MSPEVENTITEM *AllocateEventItem(SIZE_T nExtraBytes = 0);

BOOL FreeEventItem(MSPEVENTITEM *pEventItemToFree);


typedef HRESULT (*PFNCREATETERM) (
    IN  CComPtr<IMoniker>   pMoniker,
    IN  MSP_HANDLE          htAddress,
    OUT ITTerminal        **pTerm
    );

typedef struct
{
    DWORD                dwMediaType;
    const CLSID        * clsidClassManager;
    PFNCREATETERM        pfnCreateTerm;

} STATIC_TERMINAL_TYPE;

class ATL_NO_VTABLE CPlugTerminalClassInfo : 
    public IDispatchImpl<ITPluggableTerminalClassInfo, &IID_ITPluggableTerminalClassInfo, &LIBID_TAPI3Lib>,
    public CComObjectRootEx<CComMultiThreadModel>,
    public CMSPObjectSafetyImpl
{
public:
DECLARE_GET_CONTROLLING_UNKNOWN()
virtual HRESULT FinalConstruct(void);


BEGIN_COM_MAP(CPlugTerminalClassInfo)
    COM_INTERFACE_ENTRY(ITPluggableTerminalClassInfo)
    COM_INTERFACE_ENTRY(IDispatch)
    COM_INTERFACE_ENTRY(IObjectSafety)
        COM_INTERFACE_ENTRY_AGGREGATE(IID_IMarshal, m_pFTM)
END_COM_MAP()

public:
    CPlugTerminalClassInfo() :
        m_bstrName(NULL),
        m_bstrCompany(NULL),
        m_bstrVersion(NULL),
        m_bstrCLSID(NULL),
        m_bstrTerminalClass(NULL),
        m_lMediaType(1),
        m_Direction(TD_CAPTURE),
        m_pFTM(NULL)
    {
    }

    ~CPlugTerminalClassInfo()
    {
        if( m_bstrName )
        {
            SysFreeString( m_bstrName );
        }

        if( m_bstrCompany )
        {
            SysFreeString( m_bstrCompany );
        }

        if( m_bstrVersion )
        {
            SysFreeString( m_bstrVersion );
        }

        if( m_bstrCLSID )
        {
            SysFreeString( m_bstrCLSID );
        }

        if( m_bstrTerminalClass )
        {
            SysFreeString( m_bstrTerminalClass );
        }

        if( m_pFTM )
        {
            m_pFTM->Release();
        }
    }

public:
    STDMETHOD(get_Name)(
        /*[out, retval]*/ BSTR*     pName
        );

    STDMETHOD(get_Company)(
        /*[out, retval]*/ BSTR*     pCompany
        );

    STDMETHOD(get_Version)(
        /*[out, retval]*/ BSTR*     pVersion
        );

    STDMETHOD(get_TerminalClass)(
        /*[out, retval]*/ BSTR*     pTerminalClass
        );

    STDMETHOD(get_CLSID)(
        /*[out, retval]*/ BSTR*     pCLSID
        );

    STDMETHOD(get_Direction)(
        /*[out, retval]*/ TERMINAL_DIRECTION*  pDirection
        );

    STDMETHOD(get_MediaTypes)(
        /*[out, retval]*/ long*     pMediaTypes
        );

private:
    CMSPCritSection     m_CritSect;     // Critical Section 

    BSTR    m_bstrName;
    BSTR    m_bstrCompany;
    BSTR    m_bstrVersion;
    BSTR    m_bstrTerminalClass;
    BSTR    m_bstrCLSID;
    long    m_lMediaType;
    TERMINAL_DIRECTION   m_Direction;

    IUnknown*            m_pFTM;         // pointer to the free threaded marshaler

private:
    STDMETHOD(put_Name)(
        /*[in]*/    BSTR            bstrName
        );

    STDMETHOD(put_Company)(
        /*[in]*/    BSTR            bstrCompany
        );

    STDMETHOD(put_Version)(
       /*[in]*/    BSTR            bstrVersion
        );

    STDMETHOD(put_TerminalClass)(
        /*[in]*/    BSTR            bstrTerminalClass
        );

    STDMETHOD(put_CLSID)(
        /*[in]*/    BSTR            bstrCLSID
        );

    STDMETHOD(put_Direction)(
        /*[in]*/    TERMINAL_DIRECTION  nDirection
        );

    STDMETHOD(put_MediaTypes)(
        /*[in]*/    long            nMediaTypes
        );

friend class CMSPAddress;
};

class ATL_NO_VTABLE CPlugTerminalSuperclassInfo : 
    public IDispatchImpl<ITPluggableTerminalSuperclassInfo, &IID_ITPluggableTerminalSuperclassInfo, &LIBID_TAPI3Lib>,
    public CComObjectRootEx<CComMultiThreadModel>,
    public CMSPObjectSafetyImpl
{
public:

DECLARE_GET_CONTROLLING_UNKNOWN()
virtual HRESULT FinalConstruct(void);

BEGIN_COM_MAP(CPlugTerminalSuperclassInfo)
    COM_INTERFACE_ENTRY(ITPluggableTerminalSuperclassInfo)
    COM_INTERFACE_ENTRY(IDispatch)
    COM_INTERFACE_ENTRY(IObjectSafety)
        COM_INTERFACE_ENTRY_AGGREGATE(IID_IMarshal, m_pFTM)
END_COM_MAP()

public:
    CPlugTerminalSuperclassInfo() :
        m_bstrCLSID(NULL),
        m_bstrName(NULL),
        m_pFTM(NULL)
    {
    }

    ~CPlugTerminalSuperclassInfo()
    {
        if( m_bstrName )
        {
            SysFreeString( m_bstrName );
        }

        if( m_bstrCLSID )
        {
            SysFreeString( m_bstrCLSID );
        }

        if( m_pFTM )
        {
            m_pFTM->Release();
        }
    }

public:
    STDMETHOD(get_Name)(
        /*[out, retval]*/ BSTR*          pName
        );

    STDMETHOD(get_CLSID)(
        /*[out, retval]*/ BSTR*           pCLSID
        );

private:
    CMSPCritSection     m_CritSect;     // Critical Section 

    BSTR    m_bstrCLSID;
    BSTR    m_bstrName;

    IUnknown*            m_pFTM;         // pointer to the free threaded marshaler

private:
    STDMETHOD(put_Name)(
        /*[in]*/          BSTR            bstrName
        );

    STDMETHOD(put_CLSID)(
        /*[in]*/         BSTR            bstrCLSID
        );

friend class CMSPAddress;
};

/*++

Class Description:

    Represents an MSP address.

--*/

class ATL_NO_VTABLE CMSPAddress : 
    public CComObjectRootEx<CComMultiThreadModelNoCS>,
    public ITMSPAddress,
    public IDispatchImpl<ITTerminalSupport2, &IID_ITTerminalSupport2, &LIBID_TAPI3Lib>
{
public:

// No need for free thread marshaling, because the MSP address object is
// always aggregated by the TAPI3 address object.

BEGIN_COM_MAP( CMSPAddress )
    COM_INTERFACE_ENTRY( ITMSPAddress )
    COM_INTERFACE_ENTRY( IDispatch )
    COM_INTERFACE_ENTRY( ITTerminalSupport )
    COM_INTERFACE_ENTRY( ITTerminalSupport2 )
END_COM_MAP()

// The DERIVED class should DECLARE_AGGREGATABLE(className)

DECLARE_GET_CONTROLLING_UNKNOWN()

DECLARE_VQI()

    CMSPAddress();
    virtual ~CMSPAddress();
    virtual ULONG MSPAddressAddRef(void) = 0;
    virtual ULONG MSPAddressRelease(void) = 0;



// ITMSPAddress methods, called by TAPI.
    STDMETHOD (Initialize) (
        IN      MSP_HANDLE          htEvent
        );

    STDMETHOD (Shutdown) ();

    STDMETHOD (CreateMSPCall) (
        IN      MSP_HANDLE          htCall,
        IN      DWORD               dwReserved,
        IN      DWORD               dwMediaType,
        IN      IUnknown *          pOuterUnknown,
        OUT     IUnknown **         ppMSPCall
        ) = 0;

    STDMETHOD (ShutdownMSPCall) (
        IN      IUnknown *          pMSPCall
        ) = 0;

    STDMETHOD (ReceiveTSPData) (
        IN      IUnknown        *   pMSPCall,
        IN      LPBYTE              pBuffer,
        IN      DWORD               dwBufferSize
        );

    STDMETHOD (GetEvent) (
        IN OUT  DWORD *             pdwSize,
        OUT     BYTE *              pBuffer
        );

// ITTerminalSupport methods, called by TAPI and/or the app.
    STDMETHOD (get_StaticTerminals) (
            OUT  VARIANT * pVariant
            );

    STDMETHOD (EnumerateStaticTerminals) (
            OUT  IEnumTerminal ** ppTerminalEnumerator
            );

    STDMETHOD (get_DynamicTerminalClasses) (
            OUT  VARIANT * pVariant
            );

    STDMETHOD (EnumerateDynamicTerminalClasses) (
            OUT  IEnumTerminalClass ** ppTerminalClassEnumerator
            );

    STDMETHOD (CreateTerminal) (
            IN   BSTR pTerminalClass,
            IN   long lMediaType,
            IN   TERMINAL_DIRECTION Direction,
            OUT  ITTerminal ** ppTerminal
            );
    
    STDMETHOD (GetDefaultStaticTerminal) (
        IN      long                lMediaType,
        IN      TERMINAL_DIRECTION  Direction,
        OUT     ITTerminal **       ppTerminal
        );

    STDMETHOD (get_PluggableSuperclasses)( 
        OUT VARIANT * pVariant
        );

    STDMETHOD (EnumeratePluggableSuperclasses)( 
        OUT IEnumPluggableSuperclassInfo** ppSuperclassEnumerator 
        );

    STDMETHOD (get_PluggableTerminalClasses)( 
        IN  BSTR bstrTerminalSuperclass,
        IN  long lMediaType,
        OUT VARIANT * pVariant
        );

    STDMETHOD (EnumeratePluggableTerminalClasses)(
        IN  CLSID iidTerminalSuperclass,
        IN  long lMediaType,
        OUT IEnumPluggableTerminalClassInfo ** ppClassEnumerator 
        );


protected:
    // ITTerminalSupport helper methods

    virtual HRESULT GetStaticTerminals (
        IN OUT  DWORD       *       pdwNumTerminals,
        OUT     ITTerminal **       ppTerminals
        );

    virtual HRESULT GetDynamicTerminalClasses (
        IN OUT  DWORD *             pdwNumClasses,
        OUT     IID *               pTerminalClasses
        );

public:
// methods used by the MSPCall object.

    //
    // Check to see if the mediatype is non-zero and is in the mask.
    // Your MSP can override this if it needs to do special checks on
    // specific combinations of media types (e.g., can never have more
    // than one media type on a call, can never have video without
    // audio, etc.) The default implementation accepts any nonempty
    // set of media types that is a subset of the set of all supported
    // media types (specified via the GetCallMediaTypes method).
    //
    virtual BOOL IsValidSetOfMediaTypes(DWORD dwMediaType, DWORD dwMask);

    // Note: the eventItem must be allocated by malloc or new
    // (when the event is processed, it is deleted).
    virtual HRESULT PostEvent(
        IN      MSPEVENTITEM *      EventItem
        );

// method used by template function

    virtual DWORD GetCallMediaTypes(void) = 0;

protected:

    // Private helper function (protected so derived class can call it)

    virtual HRESULT IsMonikerInTerminalList(IMoniker* pMoniker);

    virtual HRESULT UpdateTerminalListForPnp(
        IN      BOOL                bDeviceArrival
        );

    virtual HRESULT UpdateTerminalList(void);

    virtual HRESULT ReceiveTSPAddressData(
        IN      PBYTE               pBuffer,
        IN      DWORD               dwSize
        );

public:
// methods used by the MSPThread object.

    virtual HRESULT PnpNotifHandler(
        IN      BOOL                bDeviceArrival
        );
    
protected:

    // The handle to TAPI's event, which is used to notify TAPI that the MSP 
    // wants to send data to it.
    HANDLE              m_htEvent;

    // List of events.
    LIST_ENTRY          m_EventList;

    // The lock that protects the data related to event handling with TAPI.
    CMSPCritSection     m_EventDataLock;


    // The pointer to the terminal manager object.
    ITTerminalManager * m_pITTerminalManager;

    // The list of static terminals that can be used on the address.
    CMSPArray <ITTerminal *>  m_Terminals;
    BOOL                m_fTerminalsUpToDate;

    // The lock that protects the data members for terminal operations.
    CMSPCritSection     m_TerminalDataLock;

private:
    static const STATIC_TERMINAL_TYPE m_saTerminalTypes[];
    static const DWORD m_sdwTerminalTypesCount;
};

template <class T>
HRESULT CreateMSPCallHelper(
    IN      CMSPAddress *       pCMSPAddress,
    IN      MSP_HANDLE          htCall,
    IN      DWORD               dwReserved,
    IN      DWORD               dwMediaType,
    IN      IUnknown *          pOuterUnknown,
    OUT     IUnknown **         ppMSPCall,
    OUT     T **                ppCMSPCall
    )
{
    LOG((MSP_TRACE, "CreateMSPCallHelper - enter"));

    HRESULT hr;
    T * pMSPCall;
    IUnknown *pUnknown = NULL;

    //
    // Check parameters.
    //
    if (!pCMSPAddress)
    {
        LOG((MSP_ERROR, "CreateMSPCallHelper - "
            "bad address pointer - exit E_POINTER"));
        
        return E_POINTER;
    }
    if ( !pOuterUnknown)
    {
        LOG((MSP_ERROR, "CreateMSPCallHelper - "
            "bad outer unknown - we require aggregation - exit E_POINTER"));
        
        return E_POINTER;
    }
    if (!ppMSPCall)
    {
        LOG((MSP_ERROR, "CreateMSPCallHelper - "
            "bad iunknown return ptr - exit E_POINTER"));
        
        return E_POINTER;
    }

    if (!ppCMSPCall)
    {
        LOG((MSP_ERROR, "CreateMSPCallHelper - "
            "bad class return ptr - exit E_POINTER"));
        
        return E_POINTER;
    }

    if ( ! pCMSPAddress->IsValidSetOfMediaTypes(
                                        dwMediaType,
                                        pCMSPAddress->GetCallMediaTypes() ) )
    {
        LOG((MSP_ERROR, "CreateMSPCallHelper - "
            "unsupported media types - exit TAPI_E_INVALIDMEDIATYPE"));
        
        return TAPI_E_INVALIDMEDIATYPE;
    }

    // dwReserved is meaningless.
    // We have no way of checking htCall.

    // the pOuterUnknown is not NULL. This object is going to be aggregated.
    CComAggObject<T> * pCall;

    pCall = new CComAggObject<T>(pOuterUnknown);

    if (pCall == NULL)
    {
        LOG((MSP_ERROR, "CreateMSPCallHelper - "
            "could not create agg call instance - exit E_OUTOFMEMORY"));

        return E_OUTOFMEMORY;
    }

    // query the interface on the containing object.
    hr = pCall->QueryInterface(IID_IUnknown, (void **)&pUnknown);

    if (FAILED(hr))
    {
        LOG((MSP_ERROR, "CreateMSPCallHelper - "
            "QueryInterface failed: %x", hr));

        delete pCall;
        return hr;
    }

    hr = pCall->FinalConstruct();

    if (FAILED(hr))
    {
        LOG((MSP_ERROR, "CreateMSPCallHelper - "
            "FinalConstruct failed: %x.", hr));

        pUnknown->Release();
        return hr;
    }

    // Get a pointer to the real MSPCall object.
    pMSPCall = dynamic_cast<T *>(&(pCall->m_contained));
    if (pMSPCall == NULL)
    {
        LOG((MSP_ERROR, "CreateMSPCallHelper - "
            "can not cast to agg object to class pointer - "
            "exit E_UNEXPECTED"));
    
        pUnknown->Release();
        return E_UNEXPECTED;
    }

    //
    // initialize the call.
    //
    
    hr = pMSPCall->Init(pCMSPAddress, htCall, dwReserved, dwMediaType);

    if (FAILED(hr))
    {
        LOG((MSP_ERROR, "CreateMSPCallHelper - "
            "call init failed: %x", hr));

        pUnknown->Release();
        return hr;
    }

    *ppMSPCall = pUnknown;
    *ppCMSPCall = pMSPCall;

    LOG((MSP_TRACE, "CreateMSPCallHelper - exit S_OK"));

    return hr;
}

template <class T>
HRESULT ShutdownMSPCallHelper(
    IN      IUnknown *          pUnknown,
    OUT     T **                ppCMSPCall
    )
{
    LOG((MSP_TRACE, "ShutdownMSPCallHelper - enter"));

    if (!pUnknown)
    {
        LOG((MSP_ERROR, "ShutdownMSPCallHelper - "
            "bad IUnknown pointer - exit E_POINTER"));

        return E_POINTER;
    }

    if (!ppCMSPCall)
    {
        LOG((MSP_ERROR, "ShutdownMSPCallHelper - "
            "bad return pointer - exit E_POINTER"));

        return E_POINTER;
    }

    T * pMSPCall;

    CComAggObject<T> * pCall = dynamic_cast<CComAggObject<T> *> (pUnknown);

    if (pCall == NULL)
    {
        LOG((MSP_ERROR, "ShutdownMSPCallHelper - "
            "can't cast unknown to agg object pointer - exit E_UNEXPECTED"));
    
        return E_UNEXPECTED;
    }

    //
    // It was aggregated. Get a pointer to the real MSPCall object.
    //

    pMSPCall = dynamic_cast<T *> (&(pCall->m_contained));

    if (pMSPCall == NULL)
    {
        LOG((MSP_ERROR, "ShutdownMSPCallHelper - "
            "can't cast contained unknown to class pointer - "
            "exit E_UNEXPECTED"));
    
        return E_UNEXPECTED;
    }

    //
    // Now we have a call to shut down. Shut it down.
    //

    HRESULT hr = pMSPCall->ShutDown();

    if (FAILED(hr))
    {
        LOG((MSP_ERROR, "ShutdownMSPCallHelper - "
            "ShutDownMSPCall failed: %x", hr));
        
        return hr;
    }

    *ppCMSPCall = pMSPCall;

    LOG((MSP_TRACE, "ShutdownMSPCallHelper - exit S_OK"));

    return S_OK;
}


#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion

#endif //__MSPADDRESS_H_
