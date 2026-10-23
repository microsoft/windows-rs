#define FLAGS __attribute__((annotate("windows_clang:sal:2:_Enum_is_bitflag_\n()\n()")))
#ifndef FORWARD_CONTRACT
#define FORWARD_CONTRACT
#endif
#ifndef DEFINITION_CONTRACT
#define DEFINITION_CONTRACT FLAGS
#endif
typedef enum FORWARD_CONTRACT _StorageKind StorageKind;
#ifndef FORWARD_ONLY
typedef enum _StorageKind {
    StorageDefault,
    StoragePaged,
    StorageNonPaged = 512,
} DEFINITION_CONTRACT StorageKind;
#endif
