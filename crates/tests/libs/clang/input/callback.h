// Function pointer typedef (callback).
typedef int (*CompareFunc)(const void* a, const void* b);
typedef void (*NotifyCallback)(int code, const char* message);

// Variadic function-pointer typedef: cannot be represented as a metadata
// delegate, so it is emitted as an opaque pointer alias (*mut u8) rather than a
// callback, keeping references to it resolvable.
typedef unsigned long (*DbgPrint)(const char* format, ...);

// Two-level typedef (as in winnt.h's TP_SIMPLE_CALLBACK / PTP_SIMPLE_CALLBACK):
// the base is a bare function TYPE that carries the parameter names, and a
// second typedef forms the pointer. The pointer typedef must inherit the names
// from the base declaration rather than emitting param0/param1.
typedef void __stdcall CallbackType(int code, const char* message);
typedef CallbackType* PCallbackType;

// Explicit non-default conventions are recovered from the source tokens (clang
// erases them from the type on 64-bit targets) and spelled as extern "C" /
// extern "fastcall". A __stdcall / CALLBACK callback is the platform default and
// stays a bare `extern fn`.
typedef int (__cdecl *CdeclCallback)(int value);
typedef int (__fastcall *FastcallCallback)(int value);

// Two-level __cdecl typedef: the convention sits on the base function type, so the
// pointer typedef must follow its TypeRef child to recover extern "C".
typedef int __cdecl CdeclBase(int value);
typedef CdeclBase* PCdeclBase;

// SAL annotations on callback params flow through the shared parse_params, exactly
// like functions and COM methods: direction (_In_/_Out_) and element-count buffer
// sizing (_In_reads_) are captured and emitted.
#define _In_ __attribute__((annotate("_In_")))
#define _Out_ __attribute__((annotate("_Out_")))
#define _In_reads_(c) __attribute__((annotate("_In_reads_(" #c ")")))

typedef void (*BufferCallback)(_In_reads_(count) const int* data, int count, _Out_ int* result);

// Unnamed callback params are named param{idx}, matching the function path.
typedef int (*UnnamedCallback)(int, const int*);

