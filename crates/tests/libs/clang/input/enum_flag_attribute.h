//! namespace Flags
enum [[clang::flag_enum]] NativeFlags : unsigned int {
    NativeNone = 0,
    NativeRead = 1,
    NativeWrite = 2,
    NativeAll = 3,
};

enum PlainEnum : unsigned int {
    PlainNone = 0,
    PlainRead = 1,
    PlainWrite = 2,
    PlainAll = 3,
};

enum class [[clang::flag_enum]] ScopedFlags : unsigned long {
    ScopedNone = 0,
    ScopedRead = 1,
};

enum __attribute__((flag_enum)) GnuFlags {
    GnuNone = 0,
    GnuRead = 1,
};

typedef enum [[clang::flag_enum]] {
    SignedNone = 0,
    SignedAll = -1,
} SIGNED_FLAGS;
