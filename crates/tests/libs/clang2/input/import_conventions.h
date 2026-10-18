extern "C" {
    typedef int (__cdecl *CdeclCallback)(int value);
    typedef int (__stdcall *SharedCallback)(int value);
    int __cdecl Cdecl(int value);
    int __stdcall Shared(int value);
#if defined(_M_IX86)
    int __stdcall Changing(int value);
    typedef int (__stdcall *ChangingCallback)(int value);
    typedef int (__stdcall *VaryingCallback)(unsigned int value);
    int __stdcall Varying(unsigned int value);
#else
    int __cdecl Changing(int value);
    typedef int (__cdecl *ChangingCallback)(int value);
    typedef int (__stdcall *VaryingCallback)(unsigned long long value);
    typedef int (__stdcall *Only64Callback)(int value);
    int __stdcall Varying(unsigned long long value);
    int __stdcall Only64(int value);
#endif
}
