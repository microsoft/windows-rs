// Tests that DEFINE_ENUM_FLAG_OPERATORS(X) macro invocations cause the
// corresponding enum to be emitted with #[flags] in the RDL output.

#define DEFINE_ENUM_FLAG_OPERATORS(X)

enum Options {
    None = 0,
    Read = 1,
    Write = 2,
};

DEFINE_ENUM_FLAG_OPERATORS(Options)
