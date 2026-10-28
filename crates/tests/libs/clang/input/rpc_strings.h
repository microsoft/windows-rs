#ifndef VERSION
#define VERSION "1.2"
#endif
#ifndef ARGUMENTS
#define ARGUMENTS ""
#endif
#define CONTRACT(name) __attribute__((annotate("windows_clang:sal:" VERSION ":" name "\n(" ARGUMENTS ")\n(" ARGUMENTS ")")))
#define READ CONTRACT("__RPC__in_string")
#define READ_OPT CONTRACT("__RPC__in_opt_string")
#define UPDATE CONTRACT("__RPC__inout_string")
#define UPDATE_OPT CONTRACT("__RPC__inout_opt_string")
#define PARAMETERS READ unsigned short* input, READ_OPT unsigned short* optional_input, UPDATE unsigned short* update, UPDATE_OPT unsigned short* optional_update

typedef void (*Buffer)(PARAMETERS);
extern "C" void BufferFunction(PARAMETERS);
struct __declspec(uuid("11111111-2222-3333-4444-555555555555")) IBuffer {
    virtual void __stdcall Read(PARAMETERS) = 0;
};
