// MAKEINTRESOURCE-family macros name a resource by integer *ordinal*: the macro
// expands to a string pointer that holds the id (not a character buffer), so the
// batch evaluator drops it as pointer-valued. Such constants are recognised by
// name from the raw `#define` body and emitted as `PWSTR`/`PSTR` constants
// carrying the ordinal (bindgen projects the const spelling, e.g.
// `IDC_ARROW: PCWSTR = PCWSTR(32512 as _)`). The scrape runs without `UNICODE`,
// so bare `MAKEINTRESOURCE` is matched from the raw token and treated as wide
// (the reference-metadata convention), independent of its `…A`/`…W` expansion.
#define MAKEINTRESOURCEW(i) ((wchar_t*)((unsigned long long)((unsigned short)(i))))
#define MAKEINTRESOURCEA(i) ((char*)((unsigned long long)((unsigned short)(i))))
#define MAKEINTRESOURCE MAKEINTRESOURCEW

#define IDC_ARROW       MAKEINTRESOURCE(32512)
#define IDI_APPLICATION MAKEINTRESOURCEW(32512)
#define OCR_NORMAL      MAKEINTRESOURCEA(32512)

// A *negative* ordinal (`TD_ERROR_ICON`/`TD_WARNING_ICON` in commctrl.h) is truncated by
// the macro's `(WORD)(i)` cast *before* widening, so it is emitted as a zero-extended 16-bit
// ordinal (`(WORD)-2 == 65534`, projected `PCWSTR(65534 as _)`): `TD_ERROR_ICON.0 as i16 == -2`
// while the pointer's high word stays zero (so `LoadIcon` treats it as an ordinal, not a
// string pointer to dereference).
#define TD_ERROR_ICON   MAKEINTRESOURCEW(-2)
#define OCR_NEGATIVE    MAKEINTRESOURCEA(-3)

