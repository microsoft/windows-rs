/*++

Copyright (c) Microsoft Corporation. All rights reserved.

Module Name:

    MSPCall.h

Abstract:

    Definitions for CMSPCall class.

--*/

#ifndef __MSPCALL_H_
#define __MSPCALL_H_
#include <winapifamily.h>

#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)



/*++

Class Description:

    Represents a active call that has media streams.

--*/

class ATL_NO_VTABLE CMSPCallBase :
    public CComObjectRootEx<CComMultiThreadModelNoCS>,
    public IDispatchImpl<ITStreamControl, &IID_ITStreamControl, &LIBID_TAPI3Lib>
{
public:

// No need for free thread marshaling, because the MSP call object is
// always aggregated by the TAPI3 call object.

DECLARE_POLY_AGGREGATABLE(CMSPCallBase)

BEGIN_COM_MAP(CMSPCallBase)
    COM_INTERFACE_ENTRY(IDispatch)
    COM_INTERFACE_ENTRY(ITStreamControl)
END_COM_MAP()

DECLARE_GET_CONTROLLING_UNKNOWN()

DECLARE_VQI()

    CMSPCallBase();

    virtual ~CMSPCallBase();

//
// Private addref and release for the MSP call. See Platform SDK documentation.
//
    virtual ULONG MSPCallAddRef  (void) = 0;
    virtual ULONG MSPCallRelease (void) = 0;

// ITStreamControl methods, called by the app.
    STDMETHOD (CreateStream) (
        IN      long                lMediaType,
        IN      TERMINAL_DIRECTION  Direction,
        IN OUT  ITStream **         ppStream
        );

    STDMETHOD (EnumerateStreams) (
        OUT     IEnumStream **      ppEnumStream
        );

    STDMETHOD (RemoveStream) (
        IN      ITStream *          pStream
        ) = 0;

    STDMETHOD (get_Streams) (
        OUT     VARIANT *           pStreams
        );

// methods called by the MSPAddress object.
    virtual HRESULT Init(
        IN      CMSPAddress *       pMSPAddress,
        IN      MSP_HANDLE          htCall,
        IN      DWORD               dwReserved,
        IN      DWORD               dwMediaType
        ) = 0;

    virtual HRESULT ShutDown(
        ) = 0;

    virtual HRESULT ReceiveTSPCallData(
        IN      PBYTE               pBuffer,
        IN      DWORD               dwSize
        );

// methods called by the MSPstream object.
    HRESULT HandleStreamEvent(
        IN      MSPEVENTITEM *      EventItem
        ) const;

protected:
    virtual HRESULT InternalCreateStream(
        IN      DWORD               dwMediaType,
        IN      TERMINAL_DIRECTION  Direction,
        IN OUT  ITStream **         ppStream
        ) = 0;

    virtual HRESULT CreateStreamObject(
        IN      DWORD               dwMediaType,
        IN      TERMINAL_DIRECTION  Direction,
        IN      IMediaEvent *       pGraph,
        IN      ITStream **         ppStream
        ) = 0;

protected:

    // The pointer to the address object. It is used to post events to TAPI3.
    // It also carries a refcount so that the address will not go away while
    // the call is still alive.
    CMSPAddress*                m_pMSPAddress;

    // The handle to the call in TAPI3. Used in firing call events.
    MSP_HANDLE                  m_htCall;

    // The media type of this call. It is a bitmask of media types.
    DWORD                       m_dwMediaType;

    // The list of stream objects in the call.
    CMSPArray <ITStream *>      m_Streams;

    // The lock that protects the stream lists.
    CMSPCritSection             m_lock;
};


/*++

Class Description:

    Represents a call that uses one DirectShow filter graph for each stream.

--*/

class ATL_NO_VTABLE CMSPCallMultiGraph : public CMSPCallBase
{
public:
    typedef struct
    {
        CMSPCallMultiGraph *    pMSPCall;
        ITStream *              pITStream;
        IMediaEvent *           pIMediaEvent;

    } MSPSTREAMCONTEXT, *PMSPSTREAMCONTEXT;

    typedef struct _THREADPOOLWAITBLOCK
    {
        HANDLE              hWaitHandle;
        MSPSTREAMCONTEXT *  pContext;

        BOOL operator ==(struct _THREADPOOLWAITBLOCK &t)
        {
            return ((hWaitHandle == t.hWaitHandle)
                && (pContext == t.pContext));
        }

    } THREADPOOLWAITBLOCK, *PTHREADPOOLWAITBLOCK;

public:
    CMSPCallMultiGraph();

    virtual ~CMSPCallMultiGraph();

// ITStreamControl methods (overriden)

    STDMETHOD (RemoveStream) (
        IN      ITStream *          ppStream
        );

// methods called by the MSPAddress object. (overriden)
    HRESULT Init(
        IN      CMSPAddress *       pMSPAddress,
        IN      MSP_HANDLE          htCall,
        IN      DWORD               dwReserved,
        IN      DWORD               dwMediaType
        );

    HRESULT ShutDown(
        );

// methods called by the thread pool
    static VOID NTAPI DispatchGraphEvent(
        IN      VOID *              pContext,
        IN      BOOLEAN             bFlag
        );

    virtual VOID HandleGraphEvent(
        IN      MSPSTREAMCONTEXT *  pContext
    );

    virtual HRESULT ProcessGraphEvent(
        IN      ITStream *          pITStream,
        IN      long                lEventCode,
        IN      LONG_PTR            lParam1,
        IN      LONG_PTR            lParam2
    );


protected:
// helper function:
    HRESULT RegisterWaitEvent(
        IN      IMediaEvent *       pIMediaEvent,
        IN      ITStream *           pITStream
        );

    HRESULT UnregisterWaitEvent(
        IN      int                 index
        );

    virtual HRESULT InternalCreateStream(
        IN      DWORD               dwMediaType,
        IN      TERMINAL_DIRECTION  Direction,
        IN OUT  ITStream **         ppStream
        );

protected:

    // The wait blocks store the information about the wait registered to
    // the thread pool. It includes the wait handles returned by the
    // RegisterWaitForSingleObject() call and a pointer to the context
    // structure. Each block in the array is for a graph in one of the
    // stream objects. The offset of a block in this array is the same
    // as the offset of the stream that owns the graph.
    CMSPArray <THREADPOOLWAITBLOCK>      m_ThreadPoolWaitBlocks;

};

//
// Event handling definitions.
//

struct MULTI_GRAPH_EVENT_DATA
{
    CMSPCallMultiGraph * pCall;
    ITStream           * pITStream;
    long                 lEventCode;
    LONG_PTR             lParam1;
    LONG_PTR             lParam2;
    IMediaEvent        * pIMediaEvent;

    MULTI_GRAPH_EVENT_DATA()
        :pIMediaEvent(NULL),
        pITStream(NULL),
        lEventCode(0),
        lParam1(0),
        lParam2(0)
    {}

};

DWORD WINAPI AsyncMultiGraphEvent(LPVOID pVoid);


#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion

#endif // __MSPCALL_H_
