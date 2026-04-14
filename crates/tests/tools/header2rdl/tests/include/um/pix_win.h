/*==========================================================================;
 *
 *  Copyright (C) Microsoft Corporation.  All Rights Reserved.
 *
 *  File:       pix_win.h
 *  Content:    PIX implementation details
 *              Don't include this file directly - use pix.h
 *
 ****************************************************************************/

#pragma once

#ifndef _PIX_WIN_H_
#define _PIX_WIN_H_

#include <utility>

namespace DirectX
{
    namespace Detail
    {
        static const UINT PIX_EVENT_UNICODE_VERSION = 0;
        static const UINT PIX_EVENT_ANSI_VERSION = 1;
        static const size_t PIX_STRING_BUFFER_COUNT = 1024;
    }
}

// ----------------------------------------------------------------------------
// PIXSetMarker
// ----------------------------------------------------------------------------

inline void PIXSetMarker(UINT64 /*metadata*/, PCWSTR /*pFormat*/, ...)
{
    // Not implemented on Windows
}

inline void PIXSetMarker(UINT64 /*metadata*/, PCSTR /*pFormat*/, ...)
{
    // Not implemented on Windows
}


#if defined(__d3d12_h__)

inline void PIXSetMarker(ID3D12GraphicsCommandList* pCommandList, UINT64 /*metadata*/, PCWSTR pFormat)
{
    using namespace DirectX::Detail;
    UINT size = static_cast<UINT>((wcslen(pFormat) + 1) * sizeof(pFormat[0]));
    pCommandList->SetMarker(PIX_EVENT_UNICODE_VERSION, pFormat, size);
}

inline void PIXSetMarker(ID3D12CommandQueue* pCommandQueue, UINT64 /*metadata*/, PCWSTR pFormat)
{
    using namespace DirectX::Detail;
    UINT size = static_cast<UINT>((wcslen(pFormat) + 1) * sizeof(pFormat[0]));
    pCommandQueue->SetMarker(PIX_EVENT_UNICODE_VERSION, pFormat, size);
}

inline void PIXSetMarker(ID3D12GraphicsCommandList* pCommandList, UINT64 /*metadata*/, PCSTR pFormat)
{
    using namespace DirectX::Detail;
    UINT size = static_cast<UINT>((strlen(pFormat) + 1) * sizeof(pFormat[0]));
    pCommandList->SetMarker(PIX_EVENT_ANSI_VERSION, pFormat, size);
}

inline void PIXSetMarker(ID3D12CommandQueue* pCommandQueue, UINT64 /*metadata*/, PCSTR pFormat)
{
    using namespace DirectX::Detail;
    UINT size = static_cast<UINT>((strlen(pFormat) + 1) * sizeof(pFormat[0]));
    pCommandQueue->SetMarker(PIX_EVENT_ANSI_VERSION, pFormat, size);
}

template <typename... TArgs>
inline void PIXSetMarker(ID3D12GraphicsCommandList* pCommandList, UINT64 /*metadata*/, PCWSTR pFormat, TArgs&&... args)
{
    using namespace DirectX::Detail;

    wchar_t buf[PIX_STRING_BUFFER_COUNT];
    int count = _snwprintf_s(buf, _TRUNCATE, pFormat, std::forward<TArgs>(args)...) + 1;
    if (count == 0) // string was truncated
        count = PIX_STRING_BUFFER_COUNT;

    UINT size = static_cast<UINT>(count * sizeof(buf[0]));
    pCommandList->SetMarker(PIX_EVENT_UNICODE_VERSION, buf, size);
}

template <typename... TArgs>
inline void PIXSetMarker(ID3D12CommandQueue* pCommandQueue, UINT64 /*metadata*/, PCWSTR pFormat, TArgs&&... args)
{
    using namespace DirectX::Detail;

    wchar_t buf[PIX_STRING_BUFFER_COUNT];
    int count = _snwprintf_s(buf, _TRUNCATE, pFormat, std::forward<TArgs>(args)...) + 1;
    if (count == 0) // string was truncated
        count = PIX_STRING_BUFFER_COUNT;

    UINT size = static_cast<UINT>(count * sizeof(buf[0]));
    pCommandQueue->SetMarker(PIX_EVENT_UNICODE_VERSION, buf, size);
}

template <typename... TArgs>
inline void PIXSetMarker(ID3D12GraphicsCommandList* pCommandList, UINT64 /*metadata*/, PCSTR pFormat, TArgs&&... args)
{
    using namespace DirectX::Detail;

    char buf[PIX_STRING_BUFFER_COUNT];
    int count = _snprintf_s(buf, _TRUNCATE, pFormat, std::forward<TArgs>(args)...) + 1;
    if (count == 0) // string was truncated
        count = PIX_STRING_BUFFER_COUNT;

    UINT size = static_cast<UINT>(count * sizeof(buf[0]));
    pCommandList->SetMarker(PIX_EVENT_ANSI_VERSION, buf, size);
}

template <typename... TArgs>
inline void PIXSetMarker(ID3D12CommandQueue* pQueue, UINT64 /*metadata*/, PCSTR pFormat, TArgs&&... args)
{
    using namespace DirectX::Detail;

    char buf[PIX_STRING_BUFFER_COUNT];
    int count = _snprintf_s(buf, _TRUNCATE, pFormat, std::forward<TArgs>(args)...) + 1;
    if (count == 0) // string was truncated
        count = PIX_STRING_BUFFER_COUNT;

    UINT size = static_cast<UINT>(count * sizeof(buf[0]));
    pQueue->SetMarker(PIX_EVENT_ANSI_VERSION, buf, size);
}

#endif // defined(__d3d12_h__)


#if defined(__d3d11_1_h__)

inline void PIXSetMarker(ID3DUserDefinedAnnotation* pAnnotation, UINT64 /*metadata*/, PCWSTR pFormat)
{
    pAnnotation->SetMarker(pFormat);
}

inline void PIXSetMarker(ID3DUserDefinedAnnotation* pAnnotation, UINT64 /*metadata*/, PCSTR pFormat)
{
    using namespace DirectX::Detail;

    wchar_t buf[PIX_STRING_BUFFER_COUNT];
    MultiByteToWideChar(CP_ACP, MB_PRECOMPOSED, pFormat, -1, buf, ARRAYSIZE(buf));

    pAnnotation->SetMarker(buf);
}

template <typename... TArgs>
inline void PIXSetMarker(ID3DUserDefinedAnnotation* pAnnotation, UINT64 /*metadata*/, PCWSTR pFormat, TArgs&&... args)
{
    using namespace DirectX::Detail;

    wchar_t buf[PIX_STRING_BUFFER_COUNT];
    _snwprintf_s(buf, _TRUNCATE, pFormat, std::forward<TArgs>(args)...);

    pAnnotation->SetMarker(buf);
}

template <typename... TArgs>
inline void PIXSetMarker(ID3DUserDefinedAnnotation* pAnnotation, UINT64 /*metadata*/, PCSTR pFormat, TArgs&&... args)
{
    using namespace DirectX::Detail;

    char buf[PIX_STRING_BUFFER_COUNT];
    _snprintf_s(buf, _TRUNCATE, pFormat, std::forward<TArgs>(args)...);

    wchar_t bufW[PIX_STRING_BUFFER_COUNT];
    MultiByteToWideChar(CP_ACP, MB_PRECOMPOSED, buf, -1, bufW, ARRAYSIZE(bufW));

    pAnnotation->SetMarker(bufW);
}

#endif // defined(__d3d11_1_h__)


#if defined(__d3d11_2_h__)

inline void PIXSetMarker(ID3D11DeviceContext2* pContext, UINT64 /*metadata*/, PCWSTR pFormat)
{
    pContext->SetMarkerInt(pFormat, 0);
}

inline void PIXSetMarker(ID3D11DeviceContext2* pContext, UINT64 /*metadata*/, PCSTR pFormat)
{
    using namespace DirectX::Detail;

    wchar_t buf[PIX_STRING_BUFFER_COUNT];
    MultiByteToWideChar(CP_ACP, MB_PRECOMPOSED, pFormat, -1, buf, ARRAYSIZE(buf));

    pContext->SetMarkerInt(buf, 0);
}

template <typename... TArgs>
inline void PIXSetMarker(ID3D11DeviceContext2* pContext, UINT64 /*metadata*/, PCWSTR pFormat, TArgs&&... args)
{
    using namespace DirectX::Detail;

    wchar_t buf[PIX_STRING_BUFFER_COUNT];
    _snwprintf_s(buf, _TRUNCATE, pFormat, std::forward<TArgs>(args)...);

    pContext->SetMarkerInt(buf, 0);
}

template <typename... TArgs>
inline void PIXSetMarker(ID3D11DeviceContext2* pContext, UINT64 /*metadata*/, PCSTR pFormat, TArgs&&... args)
{
    using namespace DirectX::Detail;

    char buf[PIX_STRING_BUFFER_COUNT];
    _snprintf_s(buf, _TRUNCATE, pFormat, std::forward<TArgs>(args)...);

    wchar_t bufW[PIX_STRING_BUFFER_COUNT];
    MultiByteToWideChar(CP_ACP, MB_PRECOMPOSED, buf, -1, bufW, ARRAYSIZE(bufW));

    pContext->SetMarkerInt(bufW, 0);
}

#endif // defined(__d3d11_2_h__)


// ----------------------------------------------------------------------------
// PIXBeginEvent
// ----------------------------------------------------------------------------

inline void PIXBeginEvent(UINT64 /*metadata*/, PCWSTR /*pFormat*/, ...)
{
    // Not implemented on Windows
}

inline void PIXBeginEvent(UINT64 /*metadata*/, PCSTR /*pFormat*/, ...)
{
    // Not implemented on Windows
}


#if defined(__d3d12_h__)

inline void PIXBeginEvent(ID3D12GraphicsCommandList* pCommandList, UINT64 /*metadata*/, PCWSTR pFormat)
{
    using namespace DirectX::Detail;
    UINT size = static_cast<UINT>((wcslen(pFormat) + 1) * sizeof(pFormat[0]));
    pCommandList->BeginEvent(PIX_EVENT_UNICODE_VERSION, pFormat, size);
}

inline void PIXBeginEvent(ID3D12CommandQueue* pCommandQueue, UINT64 /*metadata*/, PCWSTR pFormat)
{
    using namespace DirectX::Detail;
    UINT size = static_cast<UINT>((wcslen(pFormat) + 1) * sizeof(pFormat[0]));
    pCommandQueue->BeginEvent(PIX_EVENT_UNICODE_VERSION, pFormat, size);
}

inline void PIXBeginEvent(ID3D12GraphicsCommandList* pCommandList, UINT64 /*metadata*/, PCSTR pFormat)
{
    using namespace DirectX::Detail;
    UINT size = static_cast<UINT>((strlen(pFormat) + 1) * sizeof(pFormat[0]));
    pCommandList->BeginEvent(PIX_EVENT_ANSI_VERSION, pFormat, size);
}

inline void PIXBeginEvent(ID3D12CommandQueue* pCommandQueue, UINT64 /*metadata*/, PCSTR pFormat)
{
    using namespace DirectX::Detail;
    UINT size = static_cast<UINT>((strlen(pFormat) + 1) * sizeof(pFormat[0]));
    pCommandQueue->BeginEvent(PIX_EVENT_ANSI_VERSION, pFormat, size);
}

template <typename... TArgs>
inline void PIXBeginEvent(ID3D12GraphicsCommandList* pCommandList, UINT64 /*metadata*/, PCWSTR pFormat, TArgs&&... args)
{
    using namespace DirectX::Detail;

    wchar_t buf[PIX_STRING_BUFFER_COUNT];
    int count = _snwprintf_s(buf, _TRUNCATE, pFormat, std::forward<TArgs>(args)...) + 1;
    if (count == 0) // string was truncated
        count = PIX_STRING_BUFFER_COUNT;

    UINT size = static_cast<UINT>(count * sizeof(buf[0]));
    pCommandList->BeginEvent(PIX_EVENT_UNICODE_VERSION, buf, size);
}

template <typename... TArgs>
inline void PIXBeginEvent(ID3D12CommandQueue* pCommandQueue, UINT64 /*metadata*/, PCWSTR pFormat, TArgs&&... args)
{
    using namespace DirectX::Detail;

    wchar_t buf[PIX_STRING_BUFFER_COUNT];
    int count = _snwprintf_s(buf, _TRUNCATE, pFormat, std::forward<TArgs>(args)...) + 1;
    if (count == 0) // string was truncated
        count = PIX_STRING_BUFFER_COUNT;

    UINT size = static_cast<UINT>(count * sizeof(buf[0]));
    pCommandQueue->BeginEvent(PIX_EVENT_UNICODE_VERSION, buf, size);
}

template <typename... TArgs>
inline void PIXBeginEvent(ID3D12GraphicsCommandList* pCommandList, UINT64 /*metadata*/, PCSTR pFormat, TArgs&&... args)
{
    using namespace DirectX::Detail;

    char buf[PIX_STRING_BUFFER_COUNT];
    int count = _snprintf_s(buf, _TRUNCATE, pFormat, std::forward<TArgs>(args)...) + 1;
    if (count == 0) // string was truncated
        count = PIX_STRING_BUFFER_COUNT;

    UINT size = static_cast<UINT>(count * sizeof(buf[0]));
    pCommandList->BeginEvent(PIX_EVENT_ANSI_VERSION, buf, size);
}

template <typename... TArgs>
inline void PIXBeginEvent(ID3D12CommandQueue* pCommandQueue, UINT64 /*metadata*/, PCSTR pFormat, TArgs&&... args)
{
    using namespace DirectX::Detail;

    char buf[PIX_STRING_BUFFER_COUNT];
    int count = _snprintf_s(buf, _TRUNCATE, pFormat, std::forward<TArgs>(args)...) + 1;
    if (count == 0) // string was truncated
        count = PIX_STRING_BUFFER_COUNT;

    UINT size = static_cast<UINT>(count * sizeof(buf[0]));
    pCommandQueue->BeginEvent(PIX_EVENT_ANSI_VERSION, buf, size);
}

#endif // defined(__d3d12_h__)


#if defined(__d3d11_1_h__)

inline void PIXBeginEvent(ID3DUserDefinedAnnotation* pAnnotation, UINT64 /*metadata*/, PCWSTR pFormat)
{
    pAnnotation->BeginEvent(pFormat);
}

inline void PIXBeginEvent(ID3DUserDefinedAnnotation* pAnnotation, UINT64 /*metadata*/, PCSTR pFormat)
{
    using namespace DirectX::Detail;

    wchar_t buf[PIX_STRING_BUFFER_COUNT];
    MultiByteToWideChar(CP_ACP, MB_PRECOMPOSED, pFormat, -1, buf, ARRAYSIZE(buf));

    pAnnotation->BeginEvent(buf);
}

template <typename... TArgs>
inline void PIXBeginEvent(ID3DUserDefinedAnnotation* pAnnotation, UINT64 /*metadata*/, PCWSTR pFormat, TArgs&&... args)
{
    using namespace DirectX::Detail;

    wchar_t buf[PIX_STRING_BUFFER_COUNT];
    _snwprintf_s(buf, _TRUNCATE, pFormat, std::forward<TArgs>(args)...);

    pAnnotation->BeginEvent(buf);
}

template <typename... TArgs>
inline void PIXBeginEvent(ID3DUserDefinedAnnotation* pAnnotation, UINT64 /*metadata*/, PCSTR pFormat, TArgs&&... args)
{
    using namespace DirectX::Detail;

    char buf[PIX_STRING_BUFFER_COUNT];
    _snprintf_s(buf, _TRUNCATE, pFormat, std::forward<TArgs>(args)...);

    wchar_t bufW[PIX_STRING_BUFFER_COUNT];
    MultiByteToWideChar(CP_ACP, MB_PRECOMPOSED, buf, -1, bufW, ARRAYSIZE(bufW));

    pAnnotation->BeginEvent(bufW);
}

#endif // defined(__d3d11_1_h__)


#if defined(__d3d11_2_h__)

inline void PIXBeginEvent(ID3D11DeviceContext2* pContext, UINT64 /*metadata*/, PCWSTR pFormat)
{
    pContext->BeginEventInt(pFormat, 0);
}

inline void PIXBeginEvent(ID3D11DeviceContext2* pContext, UINT64 /*metadata*/, PCSTR pFormat)
{
    using namespace DirectX::Detail;

    wchar_t buf[PIX_STRING_BUFFER_COUNT];
    MultiByteToWideChar(CP_ACP, MB_PRECOMPOSED, pFormat, -1, buf, ARRAYSIZE(buf));

    pContext->BeginEventInt(buf, 0);
}

template <typename... TArgs>
inline void PIXBeginEvent(ID3D11DeviceContext2* pContext, UINT64 /*metadata*/, PCWSTR pFormat, TArgs&&... args)
{
    using namespace DirectX::Detail;

    wchar_t buf[PIX_STRING_BUFFER_COUNT];
    _snwprintf_s(buf, _TRUNCATE, pFormat, std::forward<TArgs>(args)...);

    pContext->BeginEventInt(buf, 0);
}

template <typename... TArgs>
inline void PIXBeginEvent(ID3D11DeviceContext2* pContext, UINT64 /*metadata*/, PCSTR pFormat, TArgs&&... args)
{
    using namespace DirectX::Detail;

    char buf[PIX_STRING_BUFFER_COUNT];
    _snprintf_s(buf, _TRUNCATE, pFormat, std::forward<TArgs>(args)...);

    wchar_t bufW[PIX_STRING_BUFFER_COUNT];
    MultiByteToWideChar(CP_ACP, MB_PRECOMPOSED, buf, -1, bufW, ARRAYSIZE(bufW));

    pContext->BeginEventInt(bufW, 0);
}

#endif // defined(__d3d11_2_h__)


// ----------------------------------------------------------------------------
// PIXEndEvent
// ----------------------------------------------------------------------------

inline void PIXEndEvent()
{
    // Not implemented on Windows
}


#if defined(__d3d12_h__)

inline void PIXEndEvent(ID3D12GraphicsCommandList* pCommandList)
{
    pCommandList->EndEvent();
}

inline void PIXEndEvent(ID3D12CommandQueue* pCommandQueue)
{
    pCommandQueue->EndEvent();
}

#endif // defined(__d3d12_h__)


#if defined(__d3d11_1_h__)

inline void PIXEndEvent(ID3DUserDefinedAnnotation* pAnnotation)
{
    pAnnotation->EndEvent();
}

#endif // defined(__d3d11_1_h__)


#if defined(__d3d11_2_h__)

inline void PIXEndEvent(ID3D11DeviceContext2* pContext)
{
    pContext->EndEvent();
}

#endif // defined(__d3d11_2_h__)

#endif // _PIX_WIN_H_