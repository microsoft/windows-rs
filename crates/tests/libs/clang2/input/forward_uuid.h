#ifdef HAS_UUID
struct __declspec(uuid(GUID_VALUE)) IFoo
#else
struct IFoo
#endif
#ifdef DEFINITION
{
    virtual void __stdcall Call() = 0;
}
#endif
;
