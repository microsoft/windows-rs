// C++ scoped enums (`enum class`) -> ScopedEnum.
//   - A plain `enum` carries no ScopedEnum attribute.
//   - A scoped enum that is also a flags type carries both #[flags] and ScopedEnum,
//     verifying the attribute ordering matches the writer (`#[flags]` then ScopedEnum).

enum class Color : int {
    Red = 1,
    Green = 2,
    Blue = 4,
};

enum Plain {
    PlainA = 1,
    PlainB = 2,
};

enum class Access : unsigned int {
    Read = 1,
    Write = 2,
};

#define DEFINE_ENUM_FLAG_OPERATORS(T)
DEFINE_ENUM_FLAG_OPERATORS(Access)
