#define CONTRACT(name) __attribute__((annotate("windows_clang:sal:1.2:" name "\n(count)\n(count)")))
#define PARAMETERS unsigned int count, \
    CONTRACT("__RPC__out_ecount") unsigned short* writable, \
    CONTRACT("__RPC__out_ecount_full") unsigned short* initialized, \
    CONTRACT("__RPC__out_ecount_full_string") unsigned short* terminated, \
    CONTRACT("__RPC__inout_ecount_full_opt_string") unsigned short* update

typedef void (*Buffer)(PARAMETERS);
extern "C" void BufferFunction(PARAMETERS);
struct __declspec(uuid("11111111-2222-3333-4444-555555555555")) IBuffer {
    virtual void __stdcall Read(PARAMETERS) = 0;
};
