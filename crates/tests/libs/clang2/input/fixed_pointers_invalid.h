#if CASE == 0
typedef void* __ptr32 __sptr Bad;
#elif CASE == 1
typedef void* __ptr32 __uptr Bad;
#elif CASE == 2
typedef void (*__ptr64 Bad)();
#elif CASE == 3
typedef volatile int* __ptr64 Bad;
#elif CASE == 4
typedef void* __ptr64 Address;
typedef void (*Bad)(Address);
#elif CASE == 5
typedef void* __ptr64 Address;
typedef Address (*Bad)();
#elif CASE == 6
struct Carrier { void* __ptr64 address; };
typedef void (*Bad)(Carrier);
#elif CASE == 7
typedef __attribute__((address_space(3))) int Element;
typedef Element* __ptr64 Bad;
#elif CASE == 8
struct __declspec(uuid("00000000-0000-0000-0000-000000000001")) Face {
    virtual void __stdcall Invoke() = 0;
};
typedef Face* __ptr64 Bad;
#endif
struct Dependent { Bad value; };
struct Safe { int value; };
