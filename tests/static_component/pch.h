#pragma once

// The _DEBUG preprocessor definition is used to determine if various runtime debug
// facilities are used (e.g. _calloc_dbg). Because the Rust compiler currently
// links against a non-debug runtime unconditionally [1], we need to ensure none of
// those facilities are used.
//
// [1] https://github.com/rust-lang/rust/issues/39016

#if defined(STATIC_COMPONENT) && defined(_DEBUG)
#undef _DEBUG
#define _RESTORE_DEBUG
#endif

#include <windows.h>

#if defined(_RESTORE_DEBUG)
#define _DEBUG
#undef _RESTORE_DEBUG
#endif
