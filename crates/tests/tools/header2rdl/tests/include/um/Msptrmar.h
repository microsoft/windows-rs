///////////////////////////////////////////////////////////////////////////////
//
//
//      Copyright (c) Microsoft Corporation. All rights reserved.
//
//
//      Name: msptrmar.h
//
// Description: Definition of the CAudioRenderTerminal class
//
///////////////////////////////////////////////////////////////////////////////

#ifndef _MSPTRMAR_H_
#define _MSPTRMAR_H_
#include <winapifamily.h>

#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)


#define WAVEOUT_NAME L"WaveOut Terminal"
#define MIXER_NAME L"PCM Mixer"

/////////////////////////////////////////////////////////////////////////////
// CAudioRenderTerminal

class CAudioRenderTerminal : 
    public IDispatchImpl<ITBasicAudioTerminal, &IID_ITBasicAudioTerminal, &LIBID_TAPI3Lib>, 
    public IDispatchImpl<ITStaticAudioTerminal, &IID_ITStaticAudioTerminal, &LIBID_TAPI3Lib>, 
    public CSingleFilterStaticTerminal,
    public CMSPObjectSafetyImpl
{
public:
    CAudioRenderTerminal();
    virtual ~CAudioRenderTerminal();

    // Helper methods.
    HRESULT InitializeDefaultTerminal();

    static HRESULT CreateTerminal(
        IN    CComPtr<IMoniker>    pMoniker,
        IN    MSP_HANDLE           htAddress,
        OUT   ITTerminal         **ppTerm
        );

    HRESULT FindTerminalPin();

BEGIN_COM_MAP(CAudioRenderTerminal)
    COM_INTERFACE_ENTRY(IObjectSafety)
    COM_INTERFACE_ENTRY(ITBasicAudioTerminal)
    COM_INTERFACE_ENTRY(ITStaticAudioTerminal)
    COM_INTERFACE_ENTRY_CHAIN(CSingleFilterStaticTerminal)
END_COM_MAP()

DECLARE_VQI()
DECLARE_LOG_ADDREF_RELEASE(CAudioRenderTerminal)

// ITBasicAudioTerminal
public:
    STDMETHOD(get_Balance)(OUT  long *pVal);
    STDMETHOD(put_Balance)(IN   long newVal);
    STDMETHOD(get_Volume) (OUT  long *pVal);
    STDMETHOD(put_Volume) (IN   long newVal);

// ITStaticAudioTerminal
public:

    STDMETHOD(get_WaveId) (OUT  long * plWaveId);

// Implementation
public:

    // CBaseTerminal overrides 

    STDMETHODIMP CompleteConnectTerminal(void);

    STDMETHODIMP DisconnectTerminal(
            IN      IGraphBuilder  * pGraph,
            IN      DWORD            dwReserved
            );
    
    virtual HRESULT AddFiltersToGraph();

    virtual DWORD GetSupportedMediaTypes(void)
    {
        return (DWORD) TAPIMEDIATYPE_AUDIO;
    }

    HRESULT CreateFilters();

private:

    // Keeps track of whether we need to unreserve WaveOut
    bool m_bResourceReserved;

    CComPtr<IBasicAudio> m_pIBasicAudio;

};


#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion

#endif // _MSPTRMAR_H_
