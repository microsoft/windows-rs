// LSTATUS is the Win32 error-code domain: signed `LONG` in C. The scraper preserves it
// verbatim by name as `type LSTATUS = i32`, exactly as authored in the SDK header — no
// synthetic remap to any other type. In the flat corpus every LSTATUS *reference* (the
// `Reg*` / `shlwapi` return types) likewise resolves to `LSTATUS`; that path is covered by
// the full-scrape corpus. This fixture pins the definition.
typedef long LSTATUS;