//! library fallback.dll
//! map CreateHandle=kernel32.dll
//! map CloseHandle=KERNELBASE.dll
// Per-symbol DLL overrides take precedence over the fallback library.

extern "C" {
    int AddValues(int a, int b);
    void* CreateHandle(const char* name);
    void CloseHandle(void* handle);
}
