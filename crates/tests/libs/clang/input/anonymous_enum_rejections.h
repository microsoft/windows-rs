#if defined(_M_ARM64)
#define ENUM_ALIGNMENT __attribute__((aligned(8)))
#else
#define ENUM_ALIGNMENT
#endif
enum ENUM_ALIGNMENT { Aligned = 1 };
enum { Attributed __attribute__((annotate("unhandled"))) = 2, Neighbor = 3 };
typedef enum { PointerValue = 4 } *EnumPointer;
struct PointerUser { EnumPointer value; };
enum { Survives = 5 };
typedef enum { AmbiguousValue = 6 } AliasA, AliasB;
