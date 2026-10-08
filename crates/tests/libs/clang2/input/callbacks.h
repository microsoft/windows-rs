#define IN __attribute__((annotate("_In_")))
#define STRINGIFY_(n) #n
#define STRINGIFY(n) STRINGIFY_(n)
#define BUFFER(n) __attribute__((annotate("_In_reads_(" STRINGIFY(n) ")")))
#define OPTIONAL __attribute__((annotate("_In_opt_")))
struct Packet { int value; };
typedef int (__stdcall *Callback)(IN int* value, BUFFER(count) const char* data, unsigned count);
typedef Callback CallbackAlias;
typedef Callback* CallbackSlot;
typedef int (__cdecl Function)(IN int* value);
typedef Function* FunctionPointer;
typedef int (__stdcall *Factory)(Callback callback);
struct Callbacks { Callback callback; CallbackAlias alias; CallbackSlot slot; Factory factory; Function* function; };
extern "C" int __stdcall Use(OPTIONAL Callback callback, CallbackSlot slot, Function* function, FunctionPointer pointer);
typedef int (__stdcall *Variadic)(int value, ...);
typedef int (__fastcall *Fast)(int value);
