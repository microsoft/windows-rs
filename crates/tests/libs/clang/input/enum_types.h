#pragma once
typedef enum _State {
    State_Negative = -1,
    State_Shift = 1 << 4,
    State_Alias = State_Shift,
    State_Next
} State, *StatePointer;
typedef State StateAlias;

enum class Signed64 : long long {
    Minimum = -9223372036854775807LL - 1,
    Maximum = 9223372036854775807LL
};
enum class Unsigned64 : unsigned long long { Maximum = 18446744073709551615ULL };
enum class Signed8 : signed char { Minimum = -128, Maximum = 127 };
enum class Unsigned8 : unsigned char { Maximum = 255 };
enum class Signed16 : short { Minimum = -32768, Maximum = 32767 };
enum class Unsigned16 : unsigned short { Maximum = 65535 };
enum class Unsigned32 : unsigned int { Maximum = 4294967295U };
enum class Forward : unsigned int;
enum class Forward : unsigned int { Ready = 7 };
enum Empty : unsigned int {};

#if defined(_M_IX86)
enum class ArchValue : unsigned int { Arch_Maximum = 4294967295U };
#else
enum class ArchValue : unsigned long long { Arch_Maximum = 18446744073709551615ULL };
#endif

struct EnumUser {
    State state;
    Signed64 wide;
    Unsigned8 small;
    Forward completed;
    State values[2];
};
