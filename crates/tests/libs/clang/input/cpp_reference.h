// C++ references (`const T&` / `T&`) have no ABI representation of their own; they
// are passed exactly like pointers. The scraper maps them to `*const T` / `*mut T`
// (as win32metadata does) rather than the WinRT-logical `RefConst`/`RefMut` forms.

typedef struct GUID {
    unsigned long Data1;
    unsigned short Data2;
    unsigned short Data3;
    unsigned char Data4[8];
} GUID;

typedef const GUID& REFGUID;
typedef int HRESULT;

HRESULT CoThing(REFGUID rguid, GUID& out, const int& count);
