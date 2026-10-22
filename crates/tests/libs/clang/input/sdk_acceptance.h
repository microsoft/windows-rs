#include <intsafe.h>
#include <winsock2.h>
#include <windows.h>
#include <winternl.h>
#include <sapiddk.h>
#include <tsvirtualchannels.h>

static_assert(sizeof(IMAGE_DOS_HEADER) == 64);
static_assert(alignof(IMAGE_DOS_HEADER) == 2);
static_assert(__builtin_offsetof(IMAGE_DOS_HEADER, e_lfanew) == 60);
static_assert(SOMAXCONN == 0x7fffffff);
