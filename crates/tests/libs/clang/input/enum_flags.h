// Flags enum using DEFINE_ENUM_FLAG_OPERATORS pattern.
enum FileAccess {
    FileAccess_Read = 1,
    FileAccess_Write = 2,
    FileAccess_Execute = 4,
};

// Flag enum with a high-bit member (0x80000000). Under the default `int` backing
// MSVC wraps it to a negative value; because the enum is a flag enum the scraper
// promotes it to unsigned so the member stays positive (regression guard for the
// real-world `CLSCTX_PS_DLL = 0x80000000` case).
enum HighBitFlags {
    HighBitFlags_None = 0,
    HighBitFlags_Low = 1,
    HighBitFlags_High = 0x80000000,
};

#ifdef __cplusplus
inline FileAccess operator|(FileAccess a, FileAccess b) {
    return static_cast<FileAccess>(static_cast<int>(a) | static_cast<int>(b));
}
#endif

#define DEFINE_ENUM_FLAG_OPERATORS(T)
DEFINE_ENUM_FLAG_OPERATORS(FileAccess)
DEFINE_ENUM_FLAG_OPERATORS(HighBitFlags)
