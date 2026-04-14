// Copyright (C) Microsoft Corporation. All rights reserved.
#pragma once

#include <objidl.h>

#ifdef __cplusplus
namespace Windows { namespace Foundation {

struct __declspec(uuid("5b0d3235-4dba-4d44-865e-8f1d0e4fd04d")) IMemoryBufferByteAccess :
    public IUnknown
{
    // An IMemoryBuffer object is created by a client, and the buffer is provided by IBufferByteAccess::GetBuffer.
    // When IMemoryBufferReference::Close() is called, the code that is using this buffer should set "value" to nullptr,
    // effectively "forgetting" the pointer ot the buffer.
    STDMETHOD(GetBuffer)(_Outptr_result_buffer_(*capacity) BYTE** value, _Out_ UINT32* capacity) = 0;
};

} } // Windows.Foundation
#endif /* __cplusplus */


