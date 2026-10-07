#define DEFINE_ENUM_FLAG_OPERATORS(E) \
    inline E operator|(E a, E b) { return static_cast<E>(static_cast<int>(a) | static_cast<int>(b)); }

enum Flags { None = 0, One = 1, Two = 2 };
namespace Second {
    enum Flags { None = 0, One = 1, Two = 2 };
}
DEFINE_ENUM_FLAG_OPERATORS(Flags)

enum Plain { PlainNone = 0, PlainOne = 1, PlainTwo = 2 };
inline Plain operator|(Plain a, Plain b) { return static_cast<Plain>(static_cast<int>(a) | static_cast<int>(b)); }

#undef DEFINE_ENUM_FLAG_OPERATORS
#define DEFINE_ENUM_FLAG_OPERATORS(E)
DEFINE_ENUM_FLAG_OPERATORS(Second::Flags)
