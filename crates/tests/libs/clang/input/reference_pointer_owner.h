typedef void* Address;
typedef const void* ConstAddress;
typedef void** AddressStorage;
typedef void* __ptr64 Address64;
typedef int* TypedAddress;
struct SharedPacket {
    Address data;
};
typedef void (__stdcall* Completion)(Address data, int value);
struct SharedCallbacks {
    Completion callback;
};
