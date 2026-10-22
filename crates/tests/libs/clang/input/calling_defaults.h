extern "C" int Plain(int value);
extern "C" int __cdecl Cdecl(int value);
extern "C" int __stdcall Stdcall(int value);

typedef int (*PlainCallback)(int value);
typedef int (__cdecl *CdeclCallback)(int value);
typedef int (__stdcall *StdcallCallback)(int value);
