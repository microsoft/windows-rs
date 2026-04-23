// Test that extern "C" spelled via a macro (defined in an included header)
// produces the same RDL as an explicit extern "C" declaration.
// Regression test for https://github.com/microsoft/windows-rs/issues/4259
#include "extern_c_macro_dep.h"

EXTERN_C void A();
extern "C" void B();
