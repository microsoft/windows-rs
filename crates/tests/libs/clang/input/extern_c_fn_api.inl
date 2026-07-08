// The namespace's API header (matched by the filter). The `extern "C"`-wrapped
// function must be emitted even though clang reports the wrapping linkage-spec's
// spelling location at the EXTERN_C macro definition in the (unfiltered) defs
// header — the function's own expansion location is here.
STDAPI DoApiThing(int value);
