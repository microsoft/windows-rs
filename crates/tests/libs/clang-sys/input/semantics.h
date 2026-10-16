#pragma pack(push, 1)
struct PackOne { char value; };
#pragma pack(pop)

struct __declspec(uuid("01234567-89AB-CDEF-0123-456789ABCDEF")) NativeUuid {};
extern "C" int NativeC(int);
int NativeCpp(int);
extern "C" const int NativeCValue = 3;
const int NativeCppValue = 4;
const int NativeSideEffect = (NativeCpp(0), 3);
const float NativeFloat = 1.25f;
const double NativeDouble = -0.0;
const bool NativeBool = true;
typedef unsigned short NativeMacroWord;
const __typeof__((NativeMacroWord)33) NativeTypeOfWord = (NativeMacroWord)33;
const __typeof__(("backup")) NativeTypeOfText = ("backup");
const __typeof__(sizeof(void*)) NativeTypeOfSize = sizeof(void*);
void* const NativePointerNull = nullptr;
unsigned char* const NativePointerSentinel = (unsigned char*)-1;
void* const NativePointer32 = (void*)0xffffffffULL;
const double* const NativePointerRelocation = &NativeDouble;
void* const NativePointerSideEffect = (NativeCpp(0), (void*)-1);
void* const NativePointerUndefined = (void*)(0x7fffffff + 1);
const double NativeFloatSideEffect = (NativeCpp(0), 1.25);
extern const double NativeUninitialized;
struct NativeLayout { char tag; void* value; };
const auto NativeMemberEnd = (int)((char*)&((NativeLayout*)0)->value - (char*)0)
    + sizeof(((NativeLayout*)0)->value);
const auto NativeMemberOffset = (unsigned long long)&((NativeLayout*)0)->value;
const auto NativeSignedOverflow = 0x7fffffff + 1;
const auto NativeDivisionByZero = 1 / 0;
const auto NativeNullRead = *(int*)0;
#pragma pack(push, 2)
struct PackTwo { char value; };
#pragma pack(pop)

typedef struct NativeTag { int value; } NativeOwned;
typedef NativeOwned NativeAlias;
typedef struct NativeTag NativeTagAlias;
typedef NativeOwned* NativePointer;
using NativeUsing = NativeOwned;
typedef struct NativeForward NativeForwardAlias;
struct NativeForward { int value; };
typedef struct NativeMulti { int value; } NativeOne, NativeTwo, *NativeMultiPointer;
typedef struct NativeArrayTag { int value; } NativeArray[2];
typedef enum NativeEnum { NativeEnumValue = 1 } NativeEnumAlias;
typedef struct { int value; } NativeAnonymous;
#define DEFINE_NATIVE(NAME) typedef struct Native##NAME { int value; } NAME
DEFINE_NATIVE(MacroOwned);

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
const _GUID NativeGuid = {0xffffffff, 0x8000, 2 + 3, {0xff, 1}};
const _GUID NativeGuidSideEffect = {(unsigned long)NativeCpp(0), 0, 0, {}};
const wchar_t NativeWideString[] = L"A\U0001F600";
const char NativeNegativeByte[] = "\xff";
struct alignas(AlignStandard) AlignType { char value; };
struct __attribute__((aligned(32))) AlignGNU { char value; };
struct __attribute__((aligned)) AlignDefault { char value; };
template<unsigned N> struct alignas(N) AlignDependent { char value; };
typedef void (*PlainVoid)(int value);
typedef void (__attribute__((noreturn)) *NoReturn)(int value);
typedef void QualifiedFunction() const;
typedef void VolatileFunction() volatile;
typedef void CVFunction() const volatile;
typedef void RestrictedFunction() __restrict;
void NativeDefaults(int value, int count = 3, int* pointer = nullptr);
__declspec(deprecated("use \"new\"\nnext")) void NativeDeprecated();
__attribute__((deprecated("old", "NewFunction"))) void NativeReplacement();
[[deprecated]] void NativeEmptyDeprecation();

typedef int PlainInt;
typedef const volatile int CVInt;
typedef CVInt InheritedCVInt;
typedef int* __restrict RestrictedPointer;
typedef __unaligned int UnalignedInt;
typedef UnalignedInt InheritedUnalignedInt;
typedef __attribute__((address_space(1))) int AddressSpaceInt;
typedef void (*PlainPointer)(int*);
enum { NativeExtent = 3 };
typedef int NativeComputed[NativeExtent + 1];
struct NativeExpressions {
    int bound[NativeExtent + 1];
    int initialized = NativeExtent + 1;
    unsigned bits : (NativeExtent + 1);
    unsigned initialized_bits : (NativeExtent + 1) = 1;
    int initialized_array[NativeExtent + 1] = {};
};
typedef void (*UnalignedPointer)(__unaligned int*);
#if defined(_M_X64) || defined(_M_ARM64)
static_assert(!__is_same(PlainPointer, UnalignedPointer));
#endif
struct NativeSlotBase {
    virtual int __stdcall Start() = 0;
};
struct NativeSlots : NativeSlotBase {
    virtual int __stdcall Read(int value) = 0;
    virtual int __stdcall Tail() = 0;
    virtual int __stdcall Read(float value) = 0;
    virtual int __stdcall Read(int value) const = 0;
    int Helper();
};
struct NativeSlotVirtual : virtual NativeSlotBase {
    virtual int __stdcall Start() override = 0;
};
