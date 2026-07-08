//! flat
//! namespace Windows.Win32
// Flat per-header scrape (`tool_win32` mode, `header_root.is_some()`): string-pointer
// aliases (`LPCWSTR`, `LPWSTR`, `LPOLESTR`, the `P*`/`PC*` spellings) normalise to their
// canonical `PWSTR`/`PCWSTR`/`PSTR`/`PCSTR` at *every* reference — struct field, return
// type, and parameter alike — so bindgen's core string projection applies uniformly.
// Const-ness follows the alias spelling (`LPCWSTR` -> `PCWSTR`, `LPWSTR` -> `PWSTR`). The
// redundant `LP*`/`*OLESTR` typedef *definitions* are suppressed (the canonical four are
// defined once in `winnt.rdl` in the real corpus). Contrast `string_alias_normalize.h`,
// the namespaced (`tool_webview`) counterpart, where only parameters normalise and the
// `LP*` definitions are kept and referenced verbatim. See ledger #9.
typedef char *LPSTR;
typedef const char *LPCSTR;
typedef unsigned short *LPWSTR;
typedef const unsigned short *LPCWSTR;
typedef unsigned short *LPOLESTR;
typedef const unsigned short *LPCOLESTR;

struct STRINGS {
    LPCWSTR name;
    LPWSTR buffer;
    LPCSTR ansi;
    LPOLESTR ole;
};

LPWSTR ReturnsWide(LPCWSTR in_ptr, LPWSTR out_ptr);
