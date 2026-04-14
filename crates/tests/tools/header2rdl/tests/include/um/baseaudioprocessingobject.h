// BaseAudioProcessingObject.h -- Copyright (c) 2002 Microsoft Corporation
//
// Description:
//
//  CBaseAudioProcessingObject declaration
//

#pragma once
#include <winapifamily.h>

#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)


#include <AudioEngineBaseAPO.h>

// define this to keep older string functions from being deprecated
// this is needed because some windows headers are still using deprecated string functions.
#define STRSAFE_NO_DEPRECATE  
#include <strsafe.h>
#include <ks.h>
#include <ksmedia.h>

#include <intsafe.h>
#include <crtdbg.h>


// Locked memory utility APIs for System Effect APOs
extern "C" 
{
    //-------------------------------------------------------------------------
    // Description: Locked memory allocator
    //
    // Parameters:
    //      size  - [in] Number of input connections.
    //      pMemory - [out] Void** pointer.
    //
    // Return codes:
    //      S_OK  - pMemory points to allocated memory.
    //      E_OUTOFMEMORY - No memory is available.
    //
    _Success_(return == S_OK)
    STDAPI AERT_Allocate(_In_ size_t size, _Outptr_result_bytebuffer_(size) void **pMemory);

    //-------------------------------------------------------------------------
    // Description: Locked memory free
    //
    // Parameters:
    //      pMemory - [in] Pointer to memory to free.
    //
    // Return codes:
    //      S_OK
    //
    STDAPI AERT_Free(_In_ void *pMemory);
}


// TODO: REMOVE THESE WHEN WE HAVE THESE IN OFFICIAL HEADERS"
#ifndef AVRT_CODE_BEGIN
// These are copied out of the old AvRt.h file.
#define AVRT_CODE_BEGIN   code_seg( push, "RT_CODE" )
#define AVRT_DATA_BEGIN   data_seg( push, "RT_DATA" )
#define AVRT_BSS_BEGIN    bss_seg( push, "RT_BSS" )
#define AVRT_CONST_BEGIN  const_seg( push, "RT_CONST" )
#define AVRT_VTABLES_BEGIN AVRT_CONST_BEGIN
#define AVRT_CODE_END   code_seg( pop )
#define AVRT_DATA_END   data_seg( pop )
#define AVRT_BSS_END    bss_seg( pop )
#define AVRT_CONST_END  const_seg( pop )
#define AVRT_VTABLES_END AVRT_CONST_END
#define AVRT_DATA __declspec(allocate("RT_DATA"))
#define AVRT_BSS __declspec(allocate("RT_BSS"))
#define AVRT_CONST __declspec(allocate("RT_CONST"))

// Now declare our sections to the compiler so AVRT_DATA, AVRT_BSS and
// AVRT_CONST work.

#if defined(_IA64_)
#pragma section( "RT_CONST", read, long)
#endif

#pragma AVRT_CODE_BEGIN
#pragma AVRT_CODE_END
#pragma AVRT_CONST_BEGIN
#pragma AVRT_CONST_END
#pragma AVRT_BSS_BEGIN
#pragma AVRT_BSS_END
#pragma AVRT_DATA_BEGIN
#pragma AVRT_DATA_END

#define ASSERT_REALTIME()    // Used to mark real-time code.
#define ASSERT_NONREALTIME() // Used to mark non-real time code
#endif

#pragma AVRT_VTABLES_BEGIN
//  Base class for APOs
class __declspec(novtable) CBaseAudioProcessingObject : public IAudioProcessingObject, public IAudioProcessingObjectRT, public IAudioProcessingObjectConfiguration
{
public:
    CBaseAudioProcessingObject(_In_ const APO_REG_PROPERTIES* pRegProperties);
    virtual ~CBaseAudioProcessingObject();

public:

    //-------------------------------------------------------------------------
    // Description:
    //
    //  This must be implemented by subclasses.
    //
    // Parameters:
    //
    //      u32NumInputConnections - [in] Number of input connections.
    //
    //      ppInputConnections - [in] Array of input connection property
    //              structures, one per input connection.
    //
    //      u32NumOutputConnections - [in] Number of output connections.
    //
    //      ppOutputConnections - [in, out] Array of output connection
    //              property structures, one per output connection.
    //
    STDMETHOD_(void, APOProcess)(_In_ UINT32 u32NumInputConnections,
        _In_reads_(u32NumInputConnections) APO_CONNECTION_PROPERTY** ppInputConnections,
        _In_ UINT32 u32NumOutputConnections,
        _Inout_updates_(u32NumOutputConnections) APO_CONNECTION_PROPERTY** ppOutputConnections) = 0;
    STDMETHOD_(UINT32, CalcInputFrames)(_In_ UINT32 u32OutputFrameCount);
    STDMETHOD_(UINT32, CalcOutputFrames)(_In_ UINT32 u32InputFrameCount);

    STDMETHOD(LockForProcess)(_In_ UINT32 u32NumInputConnections, _In_reads_(u32NumInputConnections) APO_CONNECTION_DESCRIPTOR** ppInputConnections, _In_ UINT32 u32NumOutputConnections, _In_reads_(u32NumOutputConnections) APO_CONNECTION_DESCRIPTOR** ppOutputConnections);
    STDMETHOD(UnlockForProcess)(void);
    STDMETHOD(Reset)(void);
    STDMETHOD(GetLatency)(_Out_ HNSTIME* pTime);
    STDMETHOD(GetRegistrationProperties)(_Outptr_opt_ APO_REG_PROPERTIES** ppRegProps);
    STDMETHOD(Initialize)(_In_ UINT32 cbDataSize, _In_reads_bytes_(cbDataSize) BYTE* pbyData);
    STDMETHOD(IsInputFormatSupported)(_In_opt_ IAudioMediaType* pOutputFormat, _In_opt_ IAudioMediaType* pRequestedInputFormat, _Outptr_opt_ IAudioMediaType** ppSupportedInputFormat);
    STDMETHOD(IsOutputFormatSupported)(_In_opt_ IAudioMediaType* pInputFormat, _In_opt_ IAudioMediaType* pRequestedOutputFormat, _Outptr_opt_ IAudioMediaType** ppSupportedOutputFormat);
    STDMETHOD(GetInputChannelCount)(_Out_ UINT32* pu32ChannelCount);

protected:
    HRESULT IsFormatTypeSupported( 
        _In_ IAudioMediaType* pOppositeFormat,
        _In_ IAudioMediaType* pRequestedFormat,
        _Outptr_ IAudioMediaType** ppSupportedFormat,
        _In_ bool bIsInput );
    HRESULT __fastcall ValidateConnection(_In_ const UNCOMPRESSEDAUDIOFORMAT &pUncompressedAudioFormat);
    virtual HRESULT ValidateAndCacheConnectionInfo(_In_ UINT32 u32NumInputConnections,
        _In_reads_(u32NumInputConnections) APO_CONNECTION_DESCRIPTOR** ppInputConnections,
        _In_ UINT32 u32NumOutputConnections,
        _In_reads_(u32NumOutputConnections) APO_CONNECTION_DESCRIPTOR** ppOutputConnections);

    bool BuffersOverlap(_In_ UINT32 u32NumInputConnections,
        _In_reads_(u32NumInputConnections) APO_CONNECTION_DESCRIPTOR** ppInputConnections,
        _In_ UINT32 u32NumOutputConnections,
        _In_reads_(u32NumOutputConnections) APO_CONNECTION_DESCRIPTOR** ppOutputConnections);

    UINT32 __fastcall GetSamplesPerFrame();
    UINT32 __fastcall GetBytesPerSampleContainer();
    UINT32 __fastcall GetValidBitsPerSample();
    FLOAT32 __fastcall GetFramesPerSecond();

    HRESULT __fastcall ValidateInitializeParameters(_In_ UINT32 cbDataSize, _In_reads_bytes_(cbDataSize) BYTE* pbyData, _In_ REFCLSID clsid, _In_ UINT32 cbStructSize);
    virtual HRESULT __fastcall ValidateDefaultAPOFormat(_In_ UNCOMPRESSEDAUDIOFORMAT& audioFormat, _In_ bool bIsInput);

protected:
    // Flag that determines if the APO is locked.
    bool                            m_bIsLocked;

    // Flag that determines if the APO has been initialized.
    bool                            m_bIsInitialized;

    // pointer to registration properties for this APO
    const APO_REG_PROPERTIES*       m_pRegProperties;

    // critical section for APOs
    CRITICAL_SECTION                m_CritSec;

    // Number of samples per frame.  Only valid if the APO_FLAG_SAMPLESPERFRAME_MUST_MATCH
    // registry property is set.
    UINT32                          m_u32SamplesPerFrame;

private:
    // The following variables are stored for convenience and for validation purposes.  They are all set to
    // zero initially and any time there are no valid connections attached to the APO.

    // The size of the container for each audio sample.  Only valid if the APO_FLAG_BITSPERSAMPLE_MUST_MATCH
    // registry property is set.
    UINT32                          m_u32BytesPerSampleContainer;

    // Valid number of bits per sample.   Only valid if the APO_FLAG_BITSPERSAMPLE_MUST_MATCH
    // registry property is set.
    UINT32                          m_u32ValidBitsPerSample;

    // Frame rate, in frames per second (hertz).  Only valid if the APO_FLAG_FRAMESPERSECOND_MUST_MATCH
    // registry property is set.
    FLOAT32                         m_f32FramesPerSecond;
};
#pragma AVRT_VTABLES_END

// The default APO registration flags
#define DEFAULT_APOREG_FLAGS                            APO_FLAG_DEFAULT
// The default minimum number of connection for input
#define DEFAULT_APOREG_MININPUTCONNECTIONS              1
// The default maximum number of connection for input
#define DEFAULT_APOREG_MAXINPUTCONNECTIONS              1
// The default minimum number of connections for output
#define DEFAULT_APOREG_MINOUTPUTCONNECTIONS             1
// The default maximum number of connections for output
#define DEFAULT_APOREG_MAXOUTPUTCONNECTIONS             1
// The maximum number of instances
#define DEFAULT_APOREG_MAXINSTANCES                     ULONG_MAX


// default format values, used in ValidateDefaultAPOFormat()

// The default format
#define DEFAULT_APO_FORMAT                      KSDATAFORMAT_SUBTYPE_IEEE_FLOAT
// The default minimum samples per frame
#define DEFAULT_APO_MINSAMPLESPERFRAME          AUDIO_MIN_CHANNELS
// The default maximum samples per frame
#define DEFAULT_APO_MAXSAMPLESPERFRAME          AUDIO_MAX_CHANNELS
// The default bytes per sample contrainer
#define DEFAULT_APO_BYTESPERSAMPLECONTAINER     4
// The default bits per sample
#define DEFAULT_APO_VALIDBITSPERSAMPLE          32
// The default minimum frames per second
#define DEFAULT_APO_MINFRAMESPERSECOND          AUDIO_MIN_FRAMERATE
// The default maximum frames per second
#define DEFAULT_APO_MAXFRAMESPERSECOND          AUDIO_MAX_FRAMERATE


//-------------------------------------------------------------------------
// Description:
//
//  Template class for CRegAPOProperties.
//
//  Helper class for APO_REG_PROPERTIES that handles the variable-size issue
//  and does some default initialization of the struct.
//
// Parameters:
//
//  NumAPOInterfaces - [in] The number of APO interfaces in this properties class.
//
template<int NumAPOInterfaces>
class CRegAPOProperties
{
public:
    CRegAPOProperties(_In_ REFCLSID clsid,
                      _In_z_ LPCWSTR pszFriendlyName, _In_z_ LPCWSTR pszCopyrightInfo,
                      _In_ UINT32 u32MajorVersion, _In_ UINT32 u32MinorVersion,
                      _In_ REFIID iidAPOInterface1,
                      _In_ APO_FLAG Flags = DEFAULT_APOREG_FLAGS,
                      _In_ UINT32 u32MinInputConnections = DEFAULT_APOREG_MININPUTCONNECTIONS,
                      _In_ UINT32 u32MaxInputConnections = DEFAULT_APOREG_MAXINPUTCONNECTIONS,
                      _In_ UINT32 u32MinOutputConnections = DEFAULT_APOREG_MINOUTPUTCONNECTIONS,
                      _In_ UINT32 u32MaxOutputConnections = DEFAULT_APOREG_MAXOUTPUTCONNECTIONS,
                      _In_ UINT32 u32MaxInstances = DEFAULT_APOREG_MAXINSTANCES,
                      ...)
    {
        int index;
        va_list va;

        m_Properties.clsid = clsid;
        m_Properties.Flags = Flags;
#ifdef _DEBUG
        HRESULT hResult = StringCbCopyW(m_Properties.szFriendlyName, sizeof(m_Properties.szFriendlyName), pszFriendlyName);
        _ASSERTE(S_OK == hResult);     // nothing to do here except assert
        hResult = StringCbCopyW(m_Properties.szCopyrightInfo, sizeof(m_Properties.szCopyrightInfo), pszCopyrightInfo);
        _ASSERTE(S_OK == hResult);     // nothing to do here except assert
#else
        StringCbCopyW(m_Properties.szFriendlyName, sizeof(m_Properties.szFriendlyName), pszFriendlyName);
        StringCbCopyW(m_Properties.szCopyrightInfo, sizeof(m_Properties.szCopyrightInfo), pszCopyrightInfo);
#endif
        m_Properties.u32MajorVersion = u32MajorVersion;
        m_Properties.u32MinorVersion = u32MinorVersion;
        m_Properties.u32MinInputConnections = u32MinInputConnections;
        m_Properties.u32MaxInputConnections = u32MaxInputConnections;
        m_Properties.u32MinOutputConnections = u32MinOutputConnections;
        m_Properties.u32MaxOutputConnections = u32MaxOutputConnections;
        m_Properties.u32MaxInstances = u32MaxInstances;
        m_Properties.u32NumAPOInterfaces = NumAPOInterfaces;
        m_Properties.iidAPOInterfaceList[0] = iidAPOInterface1;
        va_start(va, u32MaxInstances);
        // this is a "while" instead of a "for" loop for Prefast conformance
        index = 0;
        while (index < NumAPOInterfaces - 1)
        {
            m_aAdditionalAPOIIDs[index] = va_arg(va, IID);
            index++;
        }
        va_end(va);
    }

    operator const APO_REG_PROPERTIES&() const
    {
        return m_Properties;
    }

    //-------------------------------------------------------------------------
    // Description:
    //
    //  The dereference operator returns a reference to the APO registration
    //  properties.
    //
    // Return values:
    //
    //      Address of property member
    //
    operator const APO_REG_PROPERTIES*() const
    {
        return &m_Properties;
    }

public:
    //
    // Holds the properties for this APO
    //
    APO_REG_PROPERTIES m_Properties;
#pragma warning(push)
#pragma warning(disable: 4200) // zero length array
    IID                m_aAdditionalAPOIIDs[NumAPOInterfaces - 1]; // Set up the array of interface IDs this APO supports
#pragma warning(pop)
};




#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion

