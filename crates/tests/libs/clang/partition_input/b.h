// Header `b.h` (partition `Test.B`) references the shared handle `HFOO` and the shared
// alias `PSHARED` defined in `shared.inl`; both must resolve to `Test.Shared` (the
// header that defines them), not be re-emitted locally.
#include "shared.inl"

void BThing(HFOO h, PSHARED p);

LRESULT BReturn(void);

SIZE_T BSize(SIZE_T count);
