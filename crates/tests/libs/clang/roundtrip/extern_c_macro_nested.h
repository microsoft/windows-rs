// Test that a macro-expanded extern "C" inside an explicit extern "C" block
// is handled correctly.  C() is declared using the EXTERN_C macro (which
// expands to `extern "C"`) inside an outer `extern "C" {}` block, producing
// a nested CXCursor_LinkageSpec in the AST.  Both C() and D() must appear.
// Regression test for https://github.com/microsoft/windows-rs/issues/4259
#include "extern_c_macro_dep.h"

extern "C" {
    EXTERN_C void C();
    void D();
}
