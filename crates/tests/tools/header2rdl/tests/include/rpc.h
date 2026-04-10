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
 * We declare them as forward-declared structs so clang can parse the
 * function declarations without needing the full definitions.
 * ---------------------------------------------------------------------- */
typedef struct _RPC_MESSAGE       *PRPC_MESSAGE;
typedef struct IRpcStubBuffer     IRpcStubBuffer;
typedef struct IRpcChannelBuffer  IRpcChannelBuffer;
typedef struct IRpcProxyBuffer    IRpcProxyBuffer;
