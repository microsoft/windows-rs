struct Data { int value; };
typedef void (__stdcall* FunctionAddress)(int value);
struct __declspec(uuid("BDFA2E95-A51E-4E14-AB17-B1F5F3269F09")) Object {
    virtual void __stdcall Invoke() = 0;
};
#ifndef ADDRESS_TYPE
#define ADDRESS_TYPE Data*
#endif
typedef ADDRESS_TYPE Address;
typedef const Data* ConstAddress;
typedef Data** AddressStorage;
typedef Data* __ptr64 Address64;
#ifndef TYPED_ADDRESS_TYPE
#define TYPED_ADDRESS_TYPE int*
#endif
typedef TYPED_ADDRESS_TYPE TypedAddress;
struct SharedPacket {
    Address data;
};
#ifndef CALLBACK_ABI
#define CALLBACK_ABI __stdcall
#endif
#ifndef CALLBACK_PARAMETER
#define CALLBACK_PARAMETER int
#endif
struct SharedCallbacks {
    void (CALLBACK_ABI* callback)(Address data, CALLBACK_PARAMETER value);
};
struct LocalPacket {
    Address data;
    ConstAddress constant;
    AddressStorage storage;
    Address64 wide;
    TypedAddress typed;
    SharedPacket shared;
    SharedCallbacks callbacks;
};
