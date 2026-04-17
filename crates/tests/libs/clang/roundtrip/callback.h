typedef int BOOL;
typedef void* HWND;
typedef long long LPARAM;

typedef BOOL (*WNDENUMPROC)(HWND hwnd, LPARAM lParam);

BOOL EnumWindows(WNDENUMPROC lpEnumFunc, LPARAM lParam);
