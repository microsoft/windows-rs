//! library kernel32.dll
// Functions with various parameter types.

typedef unsigned long DWORD;
typedef void* HANDLE;
typedef int BOOL;
typedef const char* LPCSTR;

extern "C" {
    HANDLE OpenFile(LPCSTR name, DWORD flags);
    BOOL ReadFile(HANDLE file, void* buffer, DWORD size, DWORD* bytesRead);
    BOOL WriteFile(HANDLE file, const void* buffer, DWORD size, DWORD* bytesWritten);
    BOOL CloseFile(HANDLE file);
}
