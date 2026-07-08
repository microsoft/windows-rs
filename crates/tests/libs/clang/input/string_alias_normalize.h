// Namespaced scrape (`tool_webview` mode, `header_root.is_none()`): here the universal
// string normalization is NOT applied — only *parameters* normalise to the canonical
// `PWSTR`/`PCWSTR` (via the SAL const path), while struct fields and return types keep
// their verbatim `LP*` spelling and the `LP*` typedef *definitions* are emitted and
// referenced locally (the reference winmd lacks the const wrappers, so the scrape must
// carry its own). Contrast `flat_string_alias.h`, the flat (`tool_win32`) counterpart,
// where every reference normalises and the `LP*` definitions are suppressed. See ledger #9.
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
