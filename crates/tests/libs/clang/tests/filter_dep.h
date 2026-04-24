// Simulates a dependency header (e.g. windows.h) that should NOT be filtered
// in.  Only types that are *used* by filtered headers will appear in the
// generated RDL (pulled in transitively via the pending-typedef mechanism).
typedef long HRESULT;
// This type is never referenced by api1/api2 and must not appear in the output.
struct DepOnly {
    int x;
};
