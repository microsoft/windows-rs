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

struct __declspec(align(4)) AlignFour { long long value; };
struct __declspec(align(8)) AlignEight { long long value; };
struct alignas(0) AlignZero { char value; };
struct alignas(16) AlignStandard { char value; };
struct __attribute__((type_visibility("default"))) Visible { int value; };
struct __attribute__((type_visibility("hidden"))) Hidden { int value; };
struct __attribute__((type_visibility("protected"))) Protected { int value; };
struct __attribute__((visibility("hidden"), type_visibility("default"))) TypeVisible {
    int value;
};
struct __attribute__((visibility("hidden"))) SymbolHidden { int value; };
struct _GUID {
    unsigned long Data1;
    unsigned short Data2;
    unsigned short Data3;
    unsigned char Data4[8];
};
struct alignas(AlignStandard) AlignType { char value; };
struct __attribute__((aligned(32))) AlignGNU { char value; };
struct __attribute__((aligned)) AlignDefault { char value; };
template<unsigned N> struct alignas(N) AlignDependent { char value; };
typedef void (*PlainVoid)(int value);
typedef void (__attribute__((noreturn)) *NoReturn)(int value);
typedef void QualifiedFunction() const;

typedef int PlainInt;
typedef const volatile int CVInt;
typedef CVInt InheritedCVInt;
typedef int* __restrict RestrictedPointer;
typedef __unaligned int UnalignedInt;
typedef UnalignedInt InheritedUnalignedInt;
typedef __attribute__((address_space(1))) int AddressSpaceInt;
typedef void (*PlainPointer)(int*);
typedef void (*UnalignedPointer)(__unaligned int*);
#if defined(_M_X64) || defined(_M_ARM64)
static_assert(!__is_same(PlainPointer, UnalignedPointer));
#endif
