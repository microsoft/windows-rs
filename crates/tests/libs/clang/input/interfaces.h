typedef long HRESULT;
typedef unsigned long ULONG;

struct GUID {
    unsigned long Data1;
    unsigned short Data2;
    unsigned short Data3;
    unsigned char Data4[8];
};
typedef const GUID& REFIID;

struct __declspec(uuid("00000000-0000-0000-c000-000000000046")) __declspec(novtable) IBase {
    virtual HRESULT __stdcall QueryInterface(REFIID riid, void** object) = 0;
    virtual ULONG __stdcall AddRef() = 0;
    virtual ULONG __stdcall Release() = 0;
    template<class T> HRESULT QueryInterface(REFIID riid, T** object) {
        return QueryInterface(riid, reinterpret_cast<void**>(object));
    }
};

typedef IBase BaseAlias;
struct __declspec(uuid("01234567-89ab-cdef-0123-456789abcdef")) __declspec(novtable)
IDerived : BaseAlias {
    virtual HRESULT __stdcall Read(ULONG index, IBase** object) = 0;
    virtual ULONG __stdcall Count() = 0;
};
