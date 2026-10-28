typedef int (__cdecl *VariadicCallback)(int first, ...);
typedef int (__cdecl *FixedCallback)(int first);
extern "C" int UseFixed(FixedCallback callback);
extern "C" int UseCallback(VariadicCallback callback);
extern "C" int UsePointer(int (__cdecl *callback)(int first, ...));
extern "C" int Fixed(int first);

struct __declspec(uuid("12345678-1234-1234-1234-123456789abc")) VariadicInterface {
    virtual int __cdecl Call(int first, ...) = 0;
};
