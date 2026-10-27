#define DEFAULT(expr) __attribute__((annotate("windows_clang:sal:2:_Return_type_success_\n(" #expr ")\n(" #expr ")")))

#ifdef _WIN64
typedef unsigned long long Word;
typedef long long SignedWord;
typedef unsigned long long MixedSignedness;
typedef double Floating;
typedef unsigned long long Only64;
#else
typedef unsigned int Word;
typedef long SignedWord;
typedef int MixedSignedness;
typedef float Floating;
typedef unsigned int Only32;
#endif

#ifdef _M_X64
typedef unsigned long long OnlyX64;
#endif

#ifdef _M_ARM64
typedef unsigned int WrongArm64;
#else
typedef Word WrongArm64;
#endif

typedef Word Copy;
typedef Word* Pointer;
typedef Word Array[2];
typedef unsigned int Fixed32;
typedef unsigned long long Fixed64;
typedef DEFAULT(return >= 0) SignedWord Status;
typedef Status CopyStatus;
typedef Word (__stdcall *Callback)(SignedWord count, Fixed64 fixed);
enum NativeEnum : Word { EnumValue = 1 };
typedef NativeEnum EnumAlias;

const Word ONE = 1;
const SignedWord NEGATIVE = -1;

struct Holder {
    Word word;
    SignedWord signed_word;
    Copy copy;
    Pointer pointer;
    Array array;
    Fixed32 fixed32;
    Fixed64 fixed64;
    Callback callback;
};
