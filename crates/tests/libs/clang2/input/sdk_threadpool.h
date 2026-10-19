#include <intsafe.h>
#include <winsock2.h>
#include <windows.h>

#ifdef _WIN64
static_assert(sizeof(TP_CALLBACK_ENVIRON) == 72);
static_assert(__builtin_offsetof(TP_CALLBACK_ENVIRON, ActivationContext) == 40);
#else
static_assert(sizeof(TP_CALLBACK_ENVIRON) == 40);
static_assert(__builtin_offsetof(TP_CALLBACK_ENVIRON, ActivationContext) == 20);
#endif
