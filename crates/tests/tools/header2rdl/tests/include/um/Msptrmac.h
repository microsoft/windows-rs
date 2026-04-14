///////////////////////////////////////////////////////////////////////////////
//
//        Copyright (c) Microsoft Corporation. All rights reserved.
//
//        Name: tmaudcap.h
//
// Description: Definition of the CAudioCaptureTerminal class
//
///////////////////////////////////////////////////////////////////////////////

#ifndef _MSPTRMAC_H_
#define _MSPTRMAC_H_
#include <winapifamily.h>

#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)


#define WAVEIN_NAME L"WaveIn Terminal"

/////////////////////////////////////////////////////////////////////////////
// CAudioCaptureTerminal

class CAudioCaptureTerminal : 
    public IDispatchImpl<ITBasicAudioTerminal, &IID_ITBasicAudioTerminal, &LIBID_TAPI3Lib>, 
    public IDispatchImpl<ITStaticAudioTerminal, &IID_ITStaticAudioTerminal, &LIBID_TAPI3Lib>, 
    public CSingleFilterStaticTerminal,
    public CMSPObjectSafetyImpl

{

BEGIN_COM_MAP(CAudioCaptureTerminal)
    COM_INTERFACE_ENTRY(IObjectSafety)
    COM_INTERFACE_ENTRY(ITBasicAudioTerminal)
    COM_INTERFACE_ENTRY(ITStaticAudioTerminal)
    COM_INTERFACE_ENTRY_CHAIN(CSingleFilterStaticTerminal)
END_COM_MAP()

DECLARE_VQI()
DECLARE_LOG_ADDREF_RELEASE(CAudioCaptureTerminal)

public:
    CAudioCaptureTerminal();
    virtual ~CAudioCaptureTerminal();

    static HRESULT CreateTerminal(
        IN    CComPtr<IMoniker>    pMoniker,
        IN    MSP_HANDLE           htAddress,
        OUT   ITTerminal         **ppTerm
        );

    HRESULT FindTerminalPin();

// ITBasicAudioTerminal
public:

    STDMETHOD(get_Balance)(OUT  long *pVal);
    STDMETHOD(put_Balance)(IN   long newVal);
    STDMETHOD(get_Volume) (OUT  long *pVal);
    STDMETHOD(put_Volume) (IN   long newVal);

// ITStaticAudioTerminal
public:

    STDMETHOD(get_WaveId) (OUT  long * plWaveId);

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

    // Helper methods.
    HRESULT CreateFilters();

    // checks if the filters need to be created
    inline HRESULT CreateFiltersIfRequired();

private:
    bool m_bResourceReserved; // keeps track of whether we need to unreserve WaveOut
    CComPtr<IAMAudioInputMixer> m_pIAMAudioInputMixer;
};


    
inline HRESULT
CAudioCaptureTerminal::CreateFiltersIfRequired(
    )
{                                           
    if (m_pIFilter == NULL)   return CreateFilters();          
    
    return S_OK;
}


#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion

#endif // _MSPTRMAC_H_

// eof
