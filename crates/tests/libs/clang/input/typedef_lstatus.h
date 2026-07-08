// LSTATUS is the Win32 error-code domain: signed `LONG` in C, but it carries `ERROR_*`
// values, so the scraper remaps it to a plain `type WIN32_ERROR = u32`. See
// `error_code_typedef`. In the flat corpus every LSTATUS *reference* (the `Reg*` /
// `shlwapi` return types) likewise resolves to WIN32_ERROR; that path is covered by the
// full-scrape corpus. This fixture pins the definition remap.
typedef long LSTATUS;