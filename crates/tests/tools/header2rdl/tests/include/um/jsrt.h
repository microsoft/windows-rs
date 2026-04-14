// Copyright (C) Microsoft. All rights reserved.

/// \mainpage Chakra Hosting API Reference
///
/// Chakra is Microsoft's JavaScript engine. It is an integral part of Internet Explorer but can 
/// also be hosted independently by other applications. This reference describes the APIs available
/// to applications to host Chakra.
///
/// As of Windows 10, Chakra is forked into two binary implementations.  The binary in jscript9.dll
/// is frozen at Internet Explorer 11, plus security and other servicing fixes, but the underlying
/// platform is the same.  The binary in chakra.dll is considered "edge-mode," and is a 
/// continuously-updating platform, consistent with the evergreen browsers environment.
///
/// To compile using edge-mode, you must #define USE_EDGEMODE_JSRT before including this header 
/// file in your project, and link against chakrart.lib.  If you want to continue using the 
/// jscript9.dll implementation, do not define the edge-mode macro, and link against jsrt.lib.

/// \file
/// \brief The base Chakra hosting API.
///
/// This file contains a flat C API layer. This is the API exported by jscript9.dll.

#ifdef _MSC_VER
#pragma once
#endif  // _MSC_VER

#ifndef _JSRT_
#define _JSRT_

#if NTDDI_VERSION >= NTDDI_WIN7

#ifdef USE_EDGEMODE_JSRT
#include <chakrart.h>
#else
#include <jsrt9.h>
#endif // USE_EDGEMODE_JSRT 

#endif // NTDDI_VERSION

#endif // _JSRT_
