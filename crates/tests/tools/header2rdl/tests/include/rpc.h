#pragma once
/* Minimal shim for <rpc.h> — only the macros and types needed to parse COM/MIDL headers. */
#include <windows.h>

/* -------------------------------------------------------------------------
 * RPC calling-convention macros
 * ---------------------------------------------------------------------- */
#ifndef __RPC_STUB
#define __RPC_STUB
#endif

/* -------------------------------------------------------------------------
 * Opaque RPC types used in Proxy/Stub function signatures.
 * These are defined as minimal structs so they are emitted in the RDL and
 * the generated output can roundtrip through the windows-rdl reader/writer.
 * PRPC_MESSAGE avoids the underscore-prefixed _RPC_MESSAGE name since
 * header2rdl filters out leading-underscore identifiers.
 * ---------------------------------------------------------------------- */
struct RPC_MESSAGE {
    HANDLE  Handle;
    ULONG   DataRepresentation;
    void   *Buffer;
    UINT    BufferLength;
    UINT    ProcNum;
    void   *TransferSyntax;
    void   *RpcInterfaceInformation;
    void   *ReservedForRuntime;
    void   *ManagerEpv;
    void   *ImportContext;
    ULONG   RpcFlags;
};
typedef RPC_MESSAGE *PRPC_MESSAGE;

struct IRpcStubBuffer    { void *lpVtbl; };
struct IRpcChannelBuffer { void *lpVtbl; };
struct IRpcProxyBuffer   { void *lpVtbl; };
