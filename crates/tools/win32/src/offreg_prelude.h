// offreg.h ships in the WDK `km` include folder but is a *user-mode* API (offreg.dll):
// it references the standard Win32 `um` typedefs (DWORD, PDWORD, BYTE, PBYTE, PWSTR,
// PCWSTR, PFILETIME, PSECURITY_DESCRIPTOR, SECURITY_INFORMATION) which the kernel-mode
// translation unit (ntifs.h + wdm.h) does not bring in. This force-included prelude
// supplies just those aliases so offreg.h parses. It is a `-include` header, not the main
// input and not in `scope_headers`, so none of these declarations is emitted; and every one
// of them already exists in the Win32 winmd exclusion reference, so offreg's functions
// resolve their parameter types against Win32 by bare name.
//
// The aliases use only built-in types (or a forward declaration for FILETIME) so they are
// valid at `-include` time, before ntifs.h/wdm.h run. C++ permits an identical typedef to be
// repeated, so redefining a name the kernel headers also define is harmless as long as the
// underlying type matches.

#pragma once

typedef unsigned long DWORD;
typedef unsigned long *PDWORD;
typedef unsigned char BYTE;
typedef unsigned char *PBYTE;
typedef wchar_t *PWSTR;
typedef const wchar_t *PCWSTR;
typedef unsigned long SECURITY_INFORMATION;
typedef void *PSECURITY_DESCRIPTOR;

typedef struct _FILETIME {
    DWORD dwLowDateTime;
    DWORD dwHighDateTime;
} FILETIME, *PFILETIME;
