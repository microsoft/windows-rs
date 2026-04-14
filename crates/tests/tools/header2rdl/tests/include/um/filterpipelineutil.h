//+-------------------------------------------------------------------------
//
//  Microsoft Windows
//  Copyright (c) Microsoft Corporation. All rights reserved.
//
//--------------------------------------------------------------------------

#ifndef _PRINT_FILTER_UTIL_813b22ee_62f7_4200_
#define _PRINT_FILTER_UTIL_813b22ee_62f7_4200_

#include <winapifamily.h>

#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)

#if defined(__cplusplus)

//
// print filter pipeline
//
namespace pfp
{

//
// Helpful when you want to use a print read interface with XML SAX
// which needs an ISequentialStream
//
class PrintReadStreamToSeqStream : public ISequentialStream
{
public:

    PrintReadStreamToSeqStream(
        _In_    IPrintReadStream    *pReadStream
        ) : m_cRef(1),
            m_pStream(pReadStream),
            m_bEof(FALSE)
    {
        m_pStream->AddRef();
    }

    ~PrintReadStreamToSeqStream()
    {
        m_pStream->Release();
    }

    STDMETHODIMP_(ULONG)
    AddRef(
        VOID
        )
    {
        return InterlockedIncrement(&m_cRef);
    }

    STDMETHODIMP_(ULONG)
    Release(
        VOID
        )
    {
        ULONG cRefCount = InterlockedDecrement(&m_cRef);

        if (cRefCount)
        {
            return cRefCount;
        }

        delete this;

        return 0;
    }

    STDMETHODIMP
    QueryInterface(
        _In_         REFIID      riid,
        _COM_Outptr_ VOID        **ppv
        )
    {
        HRESULT hRes = E_POINTER;

        if (ppv)
        {
            hRes = E_NOINTERFACE;

            *ppv = NULL;

            if (riid == IID_ISequentialStream)
            {
                *ppv = static_cast<ISequentialStream *>(this);
            }
            else if (riid == IID_IUnknown)
            {
                *ppv = static_cast<IUnknown *>(this);
            }

            if (*ppv)
            {
                AddRef();

                hRes = S_OK;
            }
        }

        return hRes;
    }

    STDMETHODIMP
    Read(
        _Out_writes_bytes_to_(cb, *pcbRead)   void*    pv,
        _In_               ULONG    cb,
        _Out_opt_          ULONG    *pcbRead
        )
    {
        if (pcbRead == NULL)
        {
            return E_INVALIDARG;
        }

        *pcbRead = 0;

        HRESULT hr = S_OK;

        //
        // ISequentialStream::Read is expected to block until any of the following occurs:
        //  - the requested number of bytes have been read
        //  - the end of the stream is reached
        //  - a failure occurs
        //
        // Since IPrintReadStream::ReadBytes is non-blocking, we must call ReadBytes until one of the
        // conditions is met.
        //

        while (SUCCEEDED(hr) &&
                !m_bEof &&
                *pcbRead < cb)
        {
            DWORD cbRead = 0;
            BOOL bEof = FALSE;

            hr = m_pStream->ReadBytes(reinterpret_cast<PVOID>(reinterpret_cast<PBYTE>(pv) + *pcbRead), cb - *pcbRead, &cbRead, &bEof);

            if (SUCCEEDED(hr))
            {
                if (bEof)
                {
                    m_bEof = TRUE;
                }

                *pcbRead += cbRead;
            }
        }

        if (SUCCEEDED(hr) &&
            *pcbRead < cb)
        {
            //
            // ISequentialStream::Read returns S_FALSE when the number of bytes returned is fewer than
            // the number of bytes requested. (i.e. at the end of the stream)
            //

            hr = S_FALSE;
        }

        return hr;
    }

    STDMETHODIMP
    Write(
        _In_reads_bytes_(cb)    void const*    pv,
        _In_               ULONG          cb,
        _Out_opt_          ULONG          *pcbWritten
        )
    {
        UNREFERENCED_PARAMETER(pv);
        UNREFERENCED_PARAMETER(cb);
        UNREFERENCED_PARAMETER(pcbWritten);
        return E_NOTIMPL;
    }

private:
    BOOL               m_bEof;
    LONG               m_cRef;
    IPrintReadStream  *m_pStream;
};

//
// Helpful when you want to use a print write interface with XML SAX
// which needs an ISequentialStream
//
class PrintWriteStreamToSeqStream : public ISequentialStream
{
public:

    PrintWriteStreamToSeqStream(
        _In_    IPrintWriteStream    *pWriteStream
        ) : m_cRef(1),
            m_pStream(pWriteStream)
    {
        m_pStream->AddRef();
    }

    ~PrintWriteStreamToSeqStream()
    {
        m_pStream->Close();

        m_pStream->Release();
    }

    STDMETHODIMP_(ULONG)
    AddRef(
        VOID
        )
    {
        return InterlockedIncrement(&m_cRef);
    }

    STDMETHODIMP_(ULONG)
    Release(
        VOID
        )
    {
        ULONG cRefCount = InterlockedDecrement(&m_cRef);

        if (cRefCount)
        {
            return cRefCount;
        }

        delete this;

        return 0;
    }

    STDMETHODIMP
    QueryInterface(
        _In_      REFIID      riid,
        _Out_     VOID        **ppv
        )
    {
        HRESULT hRes = E_POINTER;

        if (ppv)
        {
            hRes = E_NOINTERFACE;

            *ppv = NULL;

            if (riid == IID_ISequentialStream)
            {
                *ppv = static_cast<ISequentialStream *>(this);
            }
            else if (riid == IID_IUnknown)
            {
                *ppv = static_cast<IUnknown *>(this);
            }

            if (*ppv)
            {
                AddRef();

                hRes = S_OK;
            }
        }

        return hRes;
    }

    STDMETHODIMP
    Read(
        _Out_writes_bytes_(cb)   void*    pv,
        _In_               ULONG    cb,
        _Out_              ULONG    *pcbRead
        )
    {
        UNREFERENCED_PARAMETER(pv);
        UNREFERENCED_PARAMETER(cb);
        UNREFERENCED_PARAMETER(pcbRead);
        return E_NOTIMPL;
    }

    STDMETHODIMP
    Write(
        _In_reads_bytes_(cb)    void const*    pv,
        _In_               ULONG          cb,
        _Out_              ULONG          *pcbWritten
        )
    {
        return m_pStream->WriteBytes(pv, cb, pcbWritten);
    }

private:

    LONG               m_cRef;
    IPrintWriteStream  *m_pStream;
};

}; // namespace pfp

#endif // if defined(__cplusplus)


#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion

#endif // #ifndef _PRINT_FILTER_UTIL_813b22ee-62f7-4200-9c85-73d139eaa579_
