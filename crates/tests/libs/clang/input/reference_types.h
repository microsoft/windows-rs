#pragma once
#define RESULT_CONTRACT __attribute__((annotate("windows_clang:sal:2:_Return_type_success_\n(return >= 0)\n(return >= 0)")))
typedef RESULT_CONTRACT long HRESULT;
typedef unsigned int Count;
typedef Count CountAlias;
typedef const unsigned short* Text;
#if defined(_M_IX86)
typedef unsigned int NativeSize;
#else
typedef unsigned long long NativeSize;
#endif
enum State { Ready, Finished };
typedef struct _Payload {
    Count count;
    State state;
} Payload;
union VARIANT {
    int number;
    void* pointer;
};
typedef HRESULT (__stdcall *Completion)(Count count);
struct Envelope {
    struct Inner { Count count; };
    Inner value;
};
