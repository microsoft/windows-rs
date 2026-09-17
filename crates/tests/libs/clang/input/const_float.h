// Floating-point constants, which the flat Win32 metadata would otherwise drop
// entirely. Two sources are exercised:
//   * object-like `#define` macros whose body is a float literal (parsed by the
//     token literal parser: `f`/`F` suffix -> f32, otherwise f64), and
//   * file-scope `const float`/`const double` variable declarations (scraped
//     from the VarDecl and constant-evaluated). An integer initializer on a
//     floating-point variable is coerced to the declared float type, and a
//     non-const variable is not emitted.
#define FVAL (1.0f)
#define DVAL 3.14
#define NEG_F -0.5f

const float FConst = 2.5f;
const double DConst = -1;
double NotConst = 9.0;
