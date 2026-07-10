//! flat
//! namespace Test
//! map RawExport036=test.dll
//! map VarFromReal=test.dll
//! map GetThingA=test.dll
//! map GetThingW=test.dll

// Object-like `#define <Alias> <Export>` forwarders. clang reports the macro-expanded
// export as the cursor name, but the prototype (and its calling-convention token) is
// written under the friendly alias. Exercises `Parser::alias_map` + `token_names_function`
// in `fn.rs`: the function is emitted under the recovered friendly name with the raw export
// as the P/Invoke import symbol, and the `__stdcall` convention is recovered by anchoring
// detection on the source spelling (a plain export would wrongly fall back to `extern "C"`).

// Forwarder: prototype written as the friendly name, rewritten to the export before parse.
// Emitted as `GenRandom` with `import = "RawExport036"` and `extern "system"`.
#define GenRandom RawExport036
int __stdcall GenRandom(void* buffer, unsigned int length);

// Back-compat alias: the real prototype is written under the export name and the macro only
// adds a legacy spelling. Must NOT be renamed — stays `VarFromReal`, no import symbol.
#define VarFromLegacy VarFromReal
int __stdcall VarFromReal(int value);

// The `A`/`W` charset-selector idiom must never be treated as an alias, else the `A`/`W`
// split would collapse. `GetThingA`/`GetThingW` are emitted unchanged.
#define GetThing GetThingW
int __stdcall GetThingA(int value);
int __stdcall GetThingW(int value);
