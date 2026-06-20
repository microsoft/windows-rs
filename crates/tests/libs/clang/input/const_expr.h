// Constant-expression #define macros that the token parser cannot handle and
// that are therefore deferred to the libclang batch evaluator
// (Const::evaluate_macros): arithmetic, bit shifts, and cross-macro references.
#define BASE 16
#define SUM (1 + 2)
#define SHIFT (1 << 4)
#define DERIVED (BASE * 2)
#define MASK (0xFF00 | 0x00FF)
