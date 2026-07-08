// SDK error-code typedef wrapper macros (`_HRESULT_TYPEDEF_`,
// `_NDIS_ERROR_TYPEDEF_`) expand to a fixed cast, so a constant wrapped in one
// carries that deliberate type (HRESULT / DWORD) rather than flattening to a
// bare integer on the evaluation path. This mirrors winerror.h's `E_*` idiom.
#define _HRESULT_TYPEDEF_(_sc) ((HRESULT)_sc)
#define _NDIS_ERROR_TYPEDEF_(_sc) (DWORD)(_sc)

#define S_OK ((HRESULT)0L)
#define E_FAIL _HRESULT_TYPEDEF_(0x80004005L)
#define E_UNEXPECTED _HRESULT_TYPEDEF_(0x8000FFFFL)
#define NDIS_STATUS_SUCCESS _NDIS_ERROR_TYPEDEF_(0x00000000L)
