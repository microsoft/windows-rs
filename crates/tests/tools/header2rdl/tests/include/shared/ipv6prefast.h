/*++

Copyright (c) 2005  Microsoft Corporation

Module Name:

    ipv6prefast.h

Abstract:

    Provides control over IPv6-related PREfast warnings.

Revision History:

--*/

#pragma once

#if defined(_PREFAST_) && defined(IPV6_PREFAST_SAFE)

#pragma prefast(disable:24001 24002 24003 24004 24007, "This code is IPv6 ready and IPv4-specific warnings may be safely disabled.")

#endif // _PREFAST_ ...
