#pragma pack(push, 1)
struct PackOne { char value; };
#pragma pack(pop)
#pragma pack(push, 2)
struct PackTwo { char value; };
#pragma pack(pop)

typedef int (*Plain)(int first, int second);
typedef int (__cdecl *Cdecl)(int first, int second);
typedef int (__stdcall *Stdcall)(int first, int second);
#if defined(_M_IX86)
typedef int (__attribute__((regparm(2))) *Registers)(int first, int second);
typedef int (__attribute__((regparm(0))) *ZeroRegisters)(int first, int second);
static_assert(!__is_same(Plain, Registers));
static_assert(!__is_same(Plain, ZeroRegisters));
#else
typedef int (*Registers)(int first, int second);
typedef int (*ZeroRegisters)(int first, int second);
#endif
typedef void (*PlainVoid)(int value);
typedef void (__attribute__((noreturn)) *NoReturn)(int value);
typedef void QualifiedFunction() const;
