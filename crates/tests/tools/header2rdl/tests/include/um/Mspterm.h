/*++

Copyright (c) Microsoft Corporation. All rights reserved.

Module Name:

    mspterm.h

Abstract:

    Definitions for the CBaseTerminal and CSingleFilterTerminal classes.

--*/

#ifndef _MSPTERM_H_
#define _MSPTERM_H_
#include <winapifamily.h>

#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)


template <class T>
class  ITTerminalVtblBase : public ITTerminal
{
};

/////////////////////////////////////////////////////////////////////////////
/////////////////////////////////////////////////////////////////////////////
//                                                                         
// CBaseTerminal                                                           
//                                                                         
// This is the base terminal implementation. All terminals must derive     
// from this class.                                                         
//                                                                         
/////////////////////////////////////////////////////////////////////////////
/////////////////////////////////////////////////////////////////////////////

class CBaseTerminal : 
    virtual public CComObjectRootEx<CComMultiThreadModelNoCS>, // we have our own CS implementation
    public IDispatchImpl<ITTerminalVtblBase<CBaseTerminal>, &IID_ITTerminal, &LIBID_TAPI3Lib>,
    public ITTerminalControl
{

BEGIN_COM_MAP(CBaseTerminal)
    COM_INTERFACE_ENTRY(IDispatch)
    COM_INTERFACE_ENTRY(ITTerminal)

    COM_INTERFACE_ENTRY(ITTerminalControl)
    COM_INTERFACE_ENTRY_AGGREGATE(IID_IMarshal, m_pFTM)
END_COM_MAP()

DECLARE_VQI()
DECLARE_GET_CONTROLLING_UNKNOWN()

public:

    CBaseTerminal();
    virtual ~CBaseTerminal();

// ITTerminal -- COM interface for use by MSP or application
public:
    STDMETHOD(get_TerminalClass)(OUT  BSTR *pVal);
    STDMETHOD(get_TerminalType) (OUT  TERMINAL_TYPE *pVal);
    STDMETHOD(get_State)        (OUT  TERMINAL_STATE *pVal);
    STDMETHOD(get_Name)         (OUT  BSTR *pVal);
    STDMETHOD(get_MediaType)    (OUT  long * plMediaType);
    STDMETHOD(get_Direction)    (OUT  TERMINAL_DIRECTION *pDirection);


public:
    // Public methods that the MSP implementation calls.
    
    virtual HRESULT Initialize (
            IN  IID                   iidTerminalClass,
            IN  DWORD                 dwMediaType,
            IN  TERMINAL_DIRECTION    Direction,
            IN  MSP_HANDLE            htAddress
            );

public:
// ITTerminalControl -- COM interface for use by MSP only
// This has to be a COM interface rather than a set of public methods because
// the MSP needs to be able to call them for dynamic terminals as well.

    //
    // We implement get_AddressHandle, ConnectTerminal and DisconnectTerminal
    // The derived classes must implement RunRenderFilter and
    // StopRenderFilter (implementation depends on # of filters)
    //

    STDMETHOD (get_AddressHandle) (
            OUT     MSP_HANDLE    * phtAddress
            );

    //
    // enters each of the internal filters into the filter graph
    // connects the internal filters together (if applicable)
    // and returns all the filters to be used as connection points
    //

    STDMETHOD (ConnectTerminal) (
            IN      IGraphBuilder  * pGraph,
            IN      DWORD            dwTerminalDirection,
            IN OUT  DWORD          * pdwNumPins,
            OUT     IPin          ** ppPins
            );

    //
    // CompleteConnectTerminal -- called after a successful ConnectTerminal
    // so that the terminal can do post-connection intitialization
    //

    STDMETHOD (CompleteConnectTerminal) (void);

    //
    // disconnects the internal filters from each other (if applicable)
    // and removes them from the filter graph (thus breaking connections to
    // the stream). 
    // Filter graph parameter is used for validation, to make sure the terminal
    // is disconnected from the same graph that it was originally connected to.
    //

    STDMETHOD (DisconnectTerminal) (
            IN      IGraphBuilder  * pGraph,
            IN      DWORD            dwReserved
            );

    //
    // stops the rightmost render filter in the terminal
    // (needed for dynamic filter graphs)
    //

    STDMETHOD (RunRenderFilter) (void) = 0;

    //
    // stops the rightmost render filter in the terminal
    // (needed for dynamic filter graphs)
    //

    STDMETHOD (StopRenderFilter) (void) = 0;

protected:
    // The lock that protects the data members.
    CMSPCritSection     m_CritSec;

public:

    TERMINAL_DIRECTION  m_TerminalDirection;
    TERMINAL_TYPE       m_TerminalType;
    TERMINAL_STATE      m_TerminalState;
    TCHAR               m_szName[MAX_PATH + 1];
    IID                 m_TerminalClassID;
    DWORD               m_dwMediaType;
    MSP_HANDLE          m_htAddress;

    // Pointer to the free threaded marshaler.
    IUnknown *          m_pFTM;

    // stores the filter graph builder (derives from IFilterGraph)
    CComPtr<IGraphBuilder> m_pGraph;

    // The following functions are to be implemented by the derived terminals

    virtual HRESULT AddFiltersToGraph() = 0;

    // By default terminals do nothing for preconnect
    virtual HRESULT ConnectFilters() { return S_OK; }

    // Returns the number of pins that will be exposed by
    // GetExposedPins(). The implementation can use pGraph
    // to actually mess with filters in a graph if it needs to
    // do so in order to figure out how many pins it has, but normally
    // that's not the case.
    // Arguments are checked by the caller.

    virtual HRESULT GetNumExposedPins(
        IN   IGraphBuilder * pGraph,
        OUT  DWORD         * pdwNumPins
        ) = 0;

    // Returns an array of pins that the stream can connect to.
    // Arguments are checked by the caller.

    virtual HRESULT GetExposedPins(
        OUT    IPin  ** ppPins
        ) = 0;

    virtual DWORD GetSupportedMediaTypes(void) = 0;

    virtual HRESULT RemoveFiltersFromGraph() = 0;

    // Do we support this media?
    BOOL MediaTypeSupported(long lMediaType);
};

/////////////////////////////////////////////////////////////////////////////
/////////////////////////////////////////////////////////////////////////////
//                                                                         //
// CSingleFilterTerminal                                                   //
//                                                                         //
// This is a base class for a terminal with a single filter and pin. The   //
// terminal could be any direction or media type, and it could be static   //
// or dynamic.                                                             //
//                                                                         //
/////////////////////////////////////////////////////////////////////////////
/////////////////////////////////////////////////////////////////////////////

class CSingleFilterTerminal :
    public CBaseTerminal
{

// If we add any additional interfaces to this class then
// we must uncomment and expand the following.
//
// BEGIN_COM_MAP(CSingleFilterTerminal)
//    COM_INTERFACE_ENTRY_CHAIN(CBaseTerminal)
// END_COM_MAP()


public:
    // Implementation: We know we have a single filter.
    CComPtr<IPin>        m_pIPin;
    CComPtr<IBaseFilter> m_pIFilter;


public:
// ITCoreTerminal

    // the rest of this interface is implemented by CBaseTerminal

    // stops the rightmost render filter in the terminal
    // (needed for dynamic filter graphs)
    STDMETHOD(RunRenderFilter)(void);

    // stops the rightmost render filter in the terminal
    // (needed for dynamic filter graphs)
    STDMETHOD(StopRenderFilter)(void);


// CBaseTerminal overrides for non-COM methods

    // AddFiltersToGraph cannot be implemented here because of the various
    // hacks regarding their names

    virtual HRESULT GetNumExposedPins(
        IN   IGraphBuilder * pGraph,
        OUT  DWORD         * pdwNumPins
        );

    virtual HRESULT GetExposedPins(
        OUT    IPin  ** ppPins
        );

    virtual HRESULT RemoveFiltersFromGraph();
};

/////////////////////////////////////////////////////////////////////////////
/////////////////////////////////////////////////////////////////////////////
//                                                                         //
// CSingleFilterStaticTerminal                                             //
//                                                                         //
// This is a base class for a static terminal with a single filter and     //
// pin. The terminal could be any direction or media type.                 //
//                                                                         //
//                                                                         //
/////////////////////////////////////////////////////////////////////////////
/////////////////////////////////////////////////////////////////////////////

class CSingleFilterStaticTerminal :
    public CSingleFilterTerminal
{

// If we add any additional interfaces to this class then
// we must uncomment and expand the following.
//
// BEGIN_COM_MAP(CSingleFilterStaticTerminal)
//    COM_INTERFACE_ENTRY_CHAIN(CSingleFilterTerminal)
// END_COM_MAP()


public:
    // public because CreateTerminal and CMSPAddress::UpdateTerminalListForPnp accesses it
    CComPtr<IMoniker> m_pMoniker;

    // this flag allows CMSPAddress::UpdateTerminalListForPnp to perform a mark and sweep
    // on the terminal list
    BOOL m_bMark;

    //
    // Compares this terminal's moniker to pMoniker, returns S_OK if they match, S_FALSE if they don't
    //
    virtual HRESULT CompareMoniker(
                                    IMoniker *pMoniker
                                  );
};


#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion

#endif // _MSPTERM_H_
