//! namespace NonFiniteFloats
const float FLOAT_INFINITY = __builtin_huge_valf();
const double DOUBLE_INFINITY = __builtin_huge_val();
const double NOT_A_NUMBER = __builtin_nan("");
#define MACRO_INFINITY (__builtin_huge_val())
#define MACRO_NAN (__builtin_nan(""))
#define NARROWED_INFINITY 1.0e100f
const float SDK_POSITIVE_INFINITY = ((float)(1e308 * 10));
const float SDK_NEGATIVE_INFINITY = ((float)(-1e308 * 10));
const float SDK_NAN = ((float)((1e308 * 10) * 0.));
const float FINITE_FLOAT = 1.5f;
#define FINITE_MACRO 2.5
