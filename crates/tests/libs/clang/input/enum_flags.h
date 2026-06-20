// Flags enum using DEFINE_ENUM_FLAG_OPERATORS pattern.
enum FileAccess {
    FileAccess_Read = 1,
    FileAccess_Write = 2,
    FileAccess_Execute = 4,
};

#ifdef __cplusplus
inline FileAccess operator|(FileAccess a, FileAccess b) {
    return static_cast<FileAccess>(static_cast<int>(a) | static_cast<int>(b));
}
#endif

#define DEFINE_ENUM_FLAG_OPERATORS(T)
DEFINE_ENUM_FLAG_OPERATORS(FileAccess)
