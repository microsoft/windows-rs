// Tests non-default enum underlying types and negative enum variant values.
// The default i32 repr is already exercised by enum.h.

// u8 (unsigned char)
enum U8Enum : unsigned char { U8A = 1, U8B = 200 };

// i8 (signed char)
enum I8Enum : signed char { I8A = 1, I8B = -1 };

// u16 (unsigned short)
enum U16Enum : unsigned short { U16A = 1, U16B = 60000 };

// i16 (short)
enum I16Enum : short { I16A = 1, I16B = -1 };

// u32 (unsigned int)
enum U32Enum : unsigned int { U32A = 1, U32B = 3000000000 };

// i64 (long long)
enum I64Enum : long long { I64A = 1, I64B = -1 };

// u64 (unsigned long long)
enum U64Enum : unsigned long long { U64A = 1, U64B = 10000000000 };

// Negative variants on a default (i32) enum.
enum WithNeg { WN_NEG = -1, WN_ZERO = 0, WN_POS = 1 };
