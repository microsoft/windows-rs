#define RPC_READ __attribute__((annotate("windows_clang:sal:1.2:__RPC__in\n()\n()")))
#ifdef RPC_INPUT
#define READ RPC_READ
#define OPTIONAL __attribute__((annotate("windows_clang:sal:1.2:__RPC__in_opt\n()\n()")))
#else
#define READ __attribute__((annotate("windows_clang:sal:2:_In_\n()\n()")))
#define OPTIONAL __attribute__((annotate("windows_clang:sal:2:_In_opt_\n()\n()")))
#endif
#define WRITE __attribute__((annotate("windows_clang:sal:2:_Out_\n()\n()")))

typedef int Number;
typedef Number NumberChain;
enum Choice { First, Second };
typedef Choice ChoiceAlias;
typedef ChoiceAlias ChoiceChain;
typedef void* Handle;
struct Opaque;
typedef Opaque* OpaqueHandle;
struct Value { int* data; };
struct __declspec(uuid("11111111-2222-3333-4444-555555555555")) IObject {
    virtual void __stdcall Call() = 0;
};

#define PARAMETERS READ int integer, READ unsigned int unsigned_integer, \
    READ bool boolean, READ float single, READ double real, READ wchar_t character, \
    READ long long wide, READ Choice choice, READ ChoiceAlias choice_alias, \
    READ ChoiceChain choice_chain, READ Number number, READ NumberChain number_chain, \
    READ const int* pointer, OPTIONAL void* optional_pointer, READ Handle handle, \
    READ OpaqueHandle opaque_handle, READ Value record, READ const int& reference, \
    READ IObject* object

extern "C" void Inputs(PARAMETERS);
typedef void (__stdcall *InputCallback)(PARAMETERS);
struct __declspec(uuid("22222222-2222-3333-4444-555555555555")) IInputs {
    virtual void __stdcall Apply(PARAMETERS) = 0;
};
extern "C" void ScalarConflict(READ int value);
extern "C" void EnumConflict(READ Choice value);
