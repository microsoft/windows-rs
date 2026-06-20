//! library test.dll
// Extern "C" function declarations.

extern "C" {
    int AddValues(int a, int b);
    void* CreateHandle(const char* name);
    void CloseHandle(void* handle);
}
