typedef long HRESULT;
typedef unsigned int DWORD;
typedef void* HANDLE;

int Add(int a, int b);
HRESULT CreateHandle(DWORD flags, HANDLE* out);
void CloseHandle(HANDLE handle);
DWORD GetCount(const void* data, DWORD length);
