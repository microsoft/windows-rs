#define INPUT __attribute__((annotate("windows_clang:sal:2:_In_\n()\n()")))

typedef int (__stdcall *SystemCallback)(INPUT int value);
typedef int (__cdecl *CdeclCallback)(INPUT int value);

struct SystemCallbacks {
    SystemCallback named;
    int (__stdcall *structural)(int value);
    int (__stdcall *(__cdecl *factory)(SystemCallback callback))(int value);
    void (__cdecl *accept)(int (__stdcall callback)(int value));
};

struct CdeclCallbacks {
    CdeclCallback named;
    int (__cdecl *structural)(int value);
};

typedef int (__stdcall *NamedSystem)(unsigned int value);
typedef int (__cdecl *NamedCdecl)(unsigned int value);
typedef void (__cdecl *NestedSystem)(int (__stdcall *callback)(unsigned int));
typedef void (__cdecl *NestedCdecl)(int (__cdecl *callback)(unsigned int));

struct CallbackSelection {
    int (__stdcall *system)(unsigned int value);
    int (__cdecl *cdecl)(unsigned int value);
    void (__cdecl *nested_system)(int (__stdcall *callback)(unsigned int));
    void (__cdecl *nested_cdecl)(int (__cdecl *callback)(unsigned int));
};

static_assert(__is_same(decltype(SystemCallbacks::named), decltype(SystemCallbacks::structural)));
static_assert(__is_same(decltype(CdeclCallbacks::named), decltype(CdeclCallbacks::structural)));
static_assert(__is_same(decltype(CallbackSelection::system), NamedSystem));
static_assert(__is_same(decltype(CallbackSelection::cdecl), NamedCdecl));
static_assert(__is_same(decltype(CallbackSelection::nested_system), NestedSystem));
static_assert(__is_same(decltype(CallbackSelection::nested_cdecl), NestedCdecl));
