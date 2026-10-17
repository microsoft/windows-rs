#ifndef CONTRACT
#define CONTRACT __attribute__((annotate("windows_clang:sal:2:_In_\n()\n()")))
#endif
typedef unsigned Count;
typedef Count Bare(CONTRACT int* value);
typedef Bare BareAgain;
typedef Bare* Pointer;
typedef Bare** Indirect;
typedef Count OtherBare(CONTRACT int* value);
typedef Bare* Factory();
typedef void NoThrow(CONTRACT int* value) noexcept;
typedef NoThrow** NoThrowIndirect;
typedef void Dispatcher(Bare callback, Bare* pointer, BareAgain* again, Pointer alias,
    Bare** indirect, Bare* const* constant, OtherBare* other, int values[3],
    NoThrow* no_throw[2]);
typedef void __stdcall StdcallBare(Count count);

struct FunctionTypes {
    Bare* direct;
    BareAgain* again;
    Pointer alias;
    Bare** indirect;
    Bare* const* constant;
    Bare* array[2];
    Dispatcher* dispatch;
    Factory* factory;
    NoThrow* no_throw;
    StdcallBare* stdcall_callback;
    Count (*inline_pointer)(int* value);
    NoThrow** no_throw_address;
    NoThrow* no_throw_array[2];
    NoThrowIndirect no_throw_alias;
};
