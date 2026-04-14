// Copyright (C) Microsoft Corporation. All rights reserved.

#if defined(_MSC_VER)
#pragma once
#endif

#ifndef _ROBUFFER_H
#define _ROBUFFER_H

#ifdef _CONTRACT_GEN
#include <nt.h>
#include <ntrtl.h>
#include <nturtl.h>
#endif
#include <apiset.h>
#include <apisetcconv.h>
#include <objidl.h>

STDAPI
RoGetBufferMarshaler(
    _Outptr_ IMarshal** bufferMarshaler
    );

#ifdef __cplusplus
namespace Windows { namespace Storage { namespace Streams {

struct __declspec(uuid("905a0fef-bc53-11df-8c49-001e4fc686da")) IBufferByteAccess : public IUnknown
{
    // an IBuffer object is created by a client, and the buffer is provided by IBufferByteAccess::Buffer.
    STDMETHOD(Buffer)(_Outptr_result_buffer_(_Inexpressible_("size given by different API")) byte **value) = 0;
};

} } } // Windows.Storage.Streams
#endif /* __cplusplus */

#endif /* _ROBUFFER_H */

