#include <intsafe.h>
#include <winsock2.h>
#include <windows.h>
#include <inspectable.h>
#include <winstring.h>
#include <perflib.h>

static_assert(__is_same(ULONG, DWORD));
static_assert(__is_same(UCHAR, BYTE));
static_assert(__is_same(PHANDLE, HANDLE*));
