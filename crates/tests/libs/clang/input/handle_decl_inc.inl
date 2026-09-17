// The STRICT DECLARE_HANDLE(name) expansion as it appears in the Windows SDK
// system headers (winnt.h): a one-off tag `name##__` plus a pointer typedef.
// These reach the scraper as *included* declarations, so the dummy `__` tag
// structs are never emitted; only the handle typedefs are referenced by the
// main-file function and resolved in the follow-up pass.
struct HWND__ { int unused; };
typedef struct HWND__ *HWND;

struct HINSTANCE__ { int unused; };
typedef struct HINSTANCE__ *HINSTANCE;
