extern int Runtime();
int storage;
void* const Relocated = &storage;
void* const SideEffects = (Runtime(), (void*)-1);
void* const Undefined = (void*)(0x7fffffff + 1);
void* Mutable = (void*)1;
typedef void (*Callback)();
Callback const CodeAddress = (Callback)-1;
struct __declspec(uuid("33333333-1111-1111-1111-111111111111")) IObject {
    virtual void __stdcall Run() = 0;
};
IObject* const ObjectAddress = (IObject*)-1;
void* const Safe = nullptr;
