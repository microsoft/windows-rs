/*++

Copyright (c) Microsoft Corporation

Module Name:

    afunix.h

Abstract:

    This file contains the definitions for the AF_UNIX socket address family.

--*/

#ifndef _AFUNIX_
#define _AFUNIX_

#pragma once

#define UNIX_PATH_MAX 108

typedef struct sockaddr_un
{
     ADDRESS_FAMILY sun_family;     /* AF_UNIX */
     char sun_path[UNIX_PATH_MAX];  /* pathname */
} SOCKADDR_UN, *PSOCKADDR_UN;

#define SIO_AF_UNIX_GETPEERPID _WSAIOR(IOC_VENDOR, 256) // Returns ULONG PID of the connected peer process
#define SIO_AF_UNIX_SETBINDPARENTPATH _WSAIOW(IOC_VENDOR, 257) // Set the parent path for bind calls
#define SIO_AF_UNIX_SETCONNPARENTPATH _WSAIOW(IOC_VENDOR, 258) // Set the parent path for connect calls
// NOTE: setting the parent path is not thread safe.

#endif /* _AFUNIX_ */