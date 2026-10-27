#ifndef VOLATILE
#define VOLATILE volatile
#endif
#ifndef RESTRICT
#define RESTRICT __restrict
#endif
typedef VOLATILE int VolatileInt;
typedef VOLATILE __unaligned unsigned char VolatileByte;
typedef const VOLATILE int* ReadOnlyAddress;
typedef void Signal(VOLATILE void* address);
typedef int (__stdcall* PointerCallback)(VOLATILE int* first, int* RESTRICT second);
struct RawPointers {
    VOLATILE int* value;
    int* RESTRICT* slot;
    ReadOnlyAddress readonly;
    VolatileInt* alias;
    VOLATILE int (*array)[2];
    PointerCallback callback;
    Signal* signal;
};
extern "C" int __stdcall Observe(VOLATILE void* address, int* RESTRICT compare);
extern "C" const VOLATILE int* __stdcall GetAddress();
