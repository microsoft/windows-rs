// Header `a.h` (partition `Test.A`) references the shared alias `PSHARED` defined in
// `shared.inl`, which `b.h` references too — both must resolve to `Test.Shared`.
#include "shared.inl"

void AThing(PSHARED p);
