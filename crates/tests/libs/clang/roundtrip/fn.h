unsigned int A();
void B(unsigned a, signed b);

extern "C"
{
    unsigned int C();
    void D(unsigned a, signed b);
}

// Variadic function declarations.  A variadic C++ function (no extern "C"
// linkage) is emitted with default ABI.  An extern "C" variadic function
// is emitted with "C" ABI.  Both must carry the trailing `...` parameter.
int VarArgFn(unsigned int count, ...);

extern "C"
{
    int VarArgCFn(unsigned int count, ...);
}

// Lvalue reference parameters (e.g. REFIID-style arguments).
struct _GUID {
    unsigned long Data1;
    unsigned short Data2;
    unsigned short Data3;
    unsigned char Data4[8];
};

unsigned int __stdcall NoOleCoCreate(
    const struct _GUID &rclsid,
    const struct _GUID &riid,
    void **ppv);

// HANDLE is defined in the reference winmd; the typedef here is suppressed and
// the function return type resolves to the qualified reference name.
typedef void* HANDLE;
HANDLE __stdcall TestHandle();
