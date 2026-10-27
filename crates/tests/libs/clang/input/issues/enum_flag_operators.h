#define DEFINE_ENUM_FLAG_OPERATORS(ENUMTYPE) \
    extern "C++" { \
    inline ENUMTYPE operator|(ENUMTYPE a, ENUMTYPE b) { return ENUMTYPE(((int)a) | ((int)b)); } \
    inline ENUMTYPE operator&(ENUMTYPE a, ENUMTYPE b) { return ENUMTYPE(((int)a) & ((int)b)); } \
    }

enum MacroFlags {
    MacroFlagsNone = 0,
    MacroFlagsA = 1,
    MacroFlagsB = 2,
};
DEFINE_ENUM_FLAG_OPERATORS(MacroFlags)

enum Plain {
    PlainA = 1,
    PlainB = 2,
};
extern "C++" inline int operator|(Plain a, Plain b) { return (int)a | (int)b; }
