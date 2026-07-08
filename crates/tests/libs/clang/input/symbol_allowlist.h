//! namespace Test.Wdk
//! library ntdll.dll
//! reference symbol_allowlist_ref
//! symbols GetTheVersion

// The `symbols` allowlist restricts emission to `GetTheVersion` alone — the
// carve-a-handful-of-symbols-out-of-an-enormous-header case (e.g. `RtlGetVersion`
// from the WDK's `wdm.h`). Every other root must be suppressed: the sibling
// function `OtherRoutine`, the `SCRATCH` struct, the `MODE` enum, and the
// `SCRATCH_LIMIT` macro constant. `GetTheVersion`'s parameter (`INFO`) and return
// (`STATUS`) types are declared here so the source compiles, but they also live in
// the reference winmd under a *different* namespace (`Test.Win32`), so they resolve
// to qualified cross-namespace references and are not re-emitted. The output is the
// single function.

typedef int STATUS;

typedef struct _INFO {
    unsigned long Size;
    unsigned long Value;
} INFO;

struct SCRATCH { int a; int b; };

enum MODE { MODE_OFF = 0, MODE_ON = 1 };

#define SCRATCH_LIMIT 64

STATUS OtherRoutine(int code);

STATUS GetTheVersion(INFO *info);
