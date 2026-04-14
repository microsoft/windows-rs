#ifndef _HTTP_COMPRESSION_H_
#define _HTTP_COMPRESSION_H_

#include <winapifamily.h>

#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)

/*++

    Copyright (c) 2009 Microsoft Corporation

    Module Name:

        httpcompression.h

    Abstract:

        Definition of extensibility APIs required
        to write a compression scheme for IIS

--*/

//
// Compression operation for Compress2
//
//
// Process input data.
// The encoder may choose to buffer the data and postpone flushing output.
// 
#define IIS_COMPRESSION_OPERATION_PROCESS   0

//
// Flush all pending output data buffered in the encoder.
// Flush is performed when the available input data is depleted.
// 
#define IIS_COMPRESSION_OPERATION_FLUSH     1

//
// Finalize the stream.
// Finalization happens when the input stream reaches the end.
// 
#define IIS_COMPRESSION_OPERATION_FINISH    2

//
// Initialize compression scheme
// When used with IIS, InitCompression is called once as soon
// as compression scheme dll is loaded by IIS compression module
//
HRESULT
WINAPI
InitCompression(
    VOID
);

//
// Uninitialize compression scheme
// When used with IIS, this method is called before compression
// scheme dll is unloaded by IIS compression module
//
VOID
WINAPI
DeInitCompression(
    VOID
);

//
// Create a new compression context
//
HRESULT
WINAPI
CreateCompression(
    OUT PVOID *context,
    IN  ULONG reserved
);

//
// Compress data
//
HRESULT WINAPI Compress(
    IN  OUT PVOID           context,            // compression context
    IN      CONST BYTE *    input_buffer,       // input buffer
    IN      LONG            input_buffer_size,  // size of input buffer
    IN      PBYTE           output_buffer,      // output buffer
    IN      LONG            output_buffer_size, // size of output buffer
    OUT     PLONG           input_used,         // amount of input buffer used
    OUT     PLONG           output_used,        // amount of output buffer used
    IN      INT             compression_level   // compression level
);

//
// Compress data with a given compression operation
//
HRESULT WINAPI Compress2(
    IN  OUT PVOID           context,            // compression context
    IN      CONST BYTE *    input_buffer,       // input buffer
    IN      LONG            input_buffer_size,  // size of input buffer
    IN      PBYTE           output_buffer,      // output buffer
    IN      LONG            output_buffer_size, // size of output buffer
    OUT     PLONG           input_used,         // amount of input buffer used
    OUT     PLONG           output_used,        // amount of output buffer used
    IN      INT             compression_level,  // compression level
    IN      INT             operation           // compression operation
);

//
// Destroy compression context
//
VOID
WINAPI
DestroyCompression(
    IN PVOID context
);

//
// Reset compression state
// Required export but not used on Windows Vista and Windows Server 2008 - IIS 7.0
// Deprecated and not required export for Windows 7 and Windows Server 2008 R2 - IIS 7.5
//
HRESULT
WINAPI
ResetCompression(
    IN OUT PVOID context
);


#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion

#endif // _HTTP_COMPRESSION_H_
