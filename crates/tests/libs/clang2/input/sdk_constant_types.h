#include <Windows.h>
static_assert(__is_same(decltype(CTRL_C_EVENT), int));
static_assert(__is_same(decltype(ERROR_INVALID_DATA), long));
static_assert(__is_same(decltype(WINEVENT_OUTOFCONTEXT), int));
static_assert(__is_same(decltype(GetLastError()), unsigned long));
static_assert(__is_same(decltype(&SetLastError), void (WINAPI*)(unsigned long)));
static_assert(__is_same(PHANDLER_ROUTINE, BOOL (WINAPI*)(unsigned long)));
