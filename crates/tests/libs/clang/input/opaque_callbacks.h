typedef int (*PlainCallback)(int first, int second);
typedef int (__cdecl *CdeclCallback)(int first, int second);
typedef int (__stdcall *StdcallCallback)(int first, int second);
#if defined(_M_IX86)
typedef int (__attribute__((regparm(2))) *RegisterCallback)(int first, int second);
static_assert(!__is_same(PlainCallback, RegisterCallback));
#else
typedef int (*RegisterCallback)(int first, int second);
#endif
typedef void (*PlainVoidCallback)(int value);
typedef void (__attribute__((noreturn)) *NoReturnCallback)(int value);
struct CallbackUser { RegisterCallback callback; };
struct Safe { int value; };
