// SAL capture shim.
//
// The Windows SDK's <sal.h>, under a clang `*-windows-msvc` target, expands every
// SAL macro to either nothing or to `[SAL_annotes(...)]` Microsoft attributes that
// clang parses and then silently drops — so libclang exposes no SAL information on
// parameters at all. The only annotation form clang reliably surfaces (as a
// `CXCursor_AnnotateAttr` child of the `ParmDecl`) is `__attribute__((annotate("…")))`.
//
// This shim is force-included (`-include`) ahead of the translation unit. It first
// pulls in the real <sal.h> so the SDK's own include guard is tripped (every later
// `#include <sal.h>` becomes a no-op) and the long tail of SAL macros keeps its
// real, harmless definitions. It then redefines just the high-value macros the
// scraper captures (`clang/annotation.rs`) as friendly `annotate` stubs that carry
// the original macro name — and, for the buffer-size macros, the size expression —
// verbatim into the annotation string.

#include <sal.h>

// --- Direction / optionality (no argument) ---
#undef _In_
#define _In_ __attribute__((annotate("_In_")))
#undef _In_z_
#define _In_z_ __attribute__((annotate("_In_z_")))
#undef _In_opt_
#define _In_opt_ __attribute__((annotate("_In_opt_")))
#undef _In_opt_z_
#define _In_opt_z_ __attribute__((annotate("_In_opt_z_")))
#undef _Out_
#define _Out_ __attribute__((annotate("_Out_")))
#undef _Out_z_
#define _Out_z_ __attribute__((annotate("_Out_z_")))
#undef _Out_opt_
#define _Out_opt_ __attribute__((annotate("_Out_opt_")))
#undef _Inout_
#define _Inout_ __attribute__((annotate("_Inout_")))
#undef _Inout_z_
#define _Inout_z_ __attribute__((annotate("_Inout_z_")))
#undef _Inout_opt_
#define _Inout_opt_ __attribute__((annotate("_Inout_opt_")))
#undef _Outptr_
#define _Outptr_ __attribute__((annotate("_Outptr_")))
#undef _Outptr_opt_
#define _Outptr_opt_ __attribute__((annotate("_Outptr_opt_")))
#undef _Outptr_result_maybenull_
#define _Outptr_result_maybenull_ __attribute__((annotate("_Outptr_result_maybenull_")))
#undef _Outptr_opt_result_maybenull_
#define _Outptr_opt_result_maybenull_ __attribute__((annotate("_Outptr_opt_result_maybenull_")))
#undef _COM_Outptr_
#define _COM_Outptr_ __attribute__((annotate("_COM_Outptr_")))
#undef _COM_Outptr_opt_
#define _COM_Outptr_opt_ __attribute__((annotate("_COM_Outptr_opt_")))
#undef _COM_Outptr_result_maybenull_
#define _COM_Outptr_result_maybenull_ __attribute__((annotate("_COM_Outptr_result_maybenull_")))
#undef _COM_Outptr_opt_result_maybenull_
#define _COM_Outptr_opt_result_maybenull_ __attribute__((annotate("_COM_Outptr_opt_result_maybenull_")))
#undef _Reserved_
#define _Reserved_ __attribute__((annotate("_Reserved_")))

// --- Buffer-size macros (one count/byte argument) ---
#undef _In_reads_
#define _In_reads_(c) __attribute__((annotate("_In_reads_(" #c ")")))
#undef _In_reads_opt_
#define _In_reads_opt_(c) __attribute__((annotate("_In_reads_opt_(" #c ")")))
#undef _In_reads_bytes_
#define _In_reads_bytes_(c) __attribute__((annotate("_In_reads_bytes_(" #c ")")))
#undef _In_reads_bytes_opt_
#define _In_reads_bytes_opt_(c) __attribute__((annotate("_In_reads_bytes_opt_(" #c ")")))
#undef _Out_writes_
#define _Out_writes_(c) __attribute__((annotate("_Out_writes_(" #c ")")))
#undef _Out_writes_z_
#define _Out_writes_z_(c) __attribute__((annotate("_Out_writes_z_(" #c ")")))
#undef _Out_writes_opt_
#define _Out_writes_opt_(c) __attribute__((annotate("_Out_writes_opt_(" #c ")")))
#undef _Out_writes_bytes_
#define _Out_writes_bytes_(c) __attribute__((annotate("_Out_writes_bytes_(" #c ")")))
#undef _Out_writes_bytes_opt_
#define _Out_writes_bytes_opt_(c) __attribute__((annotate("_Out_writes_bytes_opt_(" #c ")")))
#undef _Out_writes_bytes_all_
#define _Out_writes_bytes_all_(c) __attribute__((annotate("_Out_writes_bytes_all_(" #c ")")))
#undef _Outptr_result_buffer_
#define _Outptr_result_buffer_(c) __attribute__((annotate("_Outptr_result_buffer_(" #c ")")))
#undef _Inout_updates_
#define _Inout_updates_(c) __attribute__((annotate("_Inout_updates_(" #c ")")))
#undef _Inout_updates_z_
#define _Inout_updates_z_(c) __attribute__((annotate("_Inout_updates_z_(" #c ")")))
#undef _Inout_updates_opt_
#define _Inout_updates_opt_(c) __attribute__((annotate("_Inout_updates_opt_(" #c ")")))
#undef _Inout_updates_bytes_
#define _Inout_updates_bytes_(c) __attribute__((annotate("_Inout_updates_bytes_(" #c ")")))

// --- `_all_` buffer-size macros (one count/byte argument; whole buffer valid) ---
#undef _Out_writes_all_
#define _Out_writes_all_(c) __attribute__((annotate("_Out_writes_all_(" #c ")")))
#undef _Out_writes_all_opt_
#define _Out_writes_all_opt_(c) __attribute__((annotate("_Out_writes_all_opt_(" #c ")")))
#undef _Out_writes_bytes_all_opt_
#define _Out_writes_bytes_all_opt_(c) __attribute__((annotate("_Out_writes_bytes_all_opt_(" #c ")")))
#undef _Inout_updates_all_
#define _Inout_updates_all_(c) __attribute__((annotate("_Inout_updates_all_(" #c ")")))
#undef _Inout_updates_all_opt_
#define _Inout_updates_all_opt_(c) __attribute__((annotate("_Inout_updates_all_opt_(" #c ")")))
#undef _Inout_updates_bytes_all_
#define _Inout_updates_bytes_all_(c) __attribute__((annotate("_Inout_updates_bytes_all_(" #c ")")))
#undef _Inout_updates_bytes_all_opt_
#define _Inout_updates_bytes_all_opt_(c) __attribute__((annotate("_Inout_updates_bytes_all_opt_(" #c ")")))
#undef _Inout_updates_bytes_opt_
#define _Inout_updates_bytes_opt_(c) __attribute__((annotate("_Inout_updates_bytes_opt_(" #c ")")))

// --- Optional null-terminated writable buffers (one count argument) ---
#undef _Out_writes_opt_z_
#define _Out_writes_opt_z_(c) __attribute__((annotate("_Out_writes_opt_z_(" #c ")")))
#undef _Inout_updates_opt_z_
#define _Inout_updates_opt_z_(c) __attribute__((annotate("_Inout_updates_opt_z_(" #c ")")))

// --- `_to_` buffer-size macros (capacity + valid-extent arguments; capture the
// first, the buffer capacity). ---
#undef _Out_writes_to_
#define _Out_writes_to_(s, c) __attribute__((annotate("_Out_writes_to_(" #s "," #c ")")))
#undef _Out_writes_to_opt_
#define _Out_writes_to_opt_(s, c) __attribute__((annotate("_Out_writes_to_opt_(" #s "," #c ")")))
#undef _Out_writes_bytes_to_
#define _Out_writes_bytes_to_(s, c) __attribute__((annotate("_Out_writes_bytes_to_(" #s "," #c ")")))
#undef _Out_writes_bytes_to_opt_
#define _Out_writes_bytes_to_opt_(s, c) __attribute__((annotate("_Out_writes_bytes_to_opt_(" #s "," #c ")")))
#undef _Inout_updates_to_
#define _Inout_updates_to_(s, c) __attribute__((annotate("_Inout_updates_to_(" #s "," #c ")")))
#undef _Inout_updates_to_opt_
#define _Inout_updates_to_opt_(s, c) __attribute__((annotate("_Inout_updates_to_opt_(" #s "," #c ")")))
#undef _Inout_updates_bytes_to_
#define _Inout_updates_bytes_to_(s, c) __attribute__((annotate("_Inout_updates_bytes_to_(" #s "," #c ")")))
#undef _Inout_updates_bytes_to_opt_
#define _Inout_updates_bytes_to_opt_(s, c) __attribute__((annotate("_Inout_updates_bytes_to_opt_(" #s "," #c ")")))

// --- `_to_ptr_` writable buffers bounded by an end pointer. Only the direction is
// faithful; the pointer-difference extent is not an element/byte count, so no size
// is carried. ---
#undef _Out_writes_to_ptr_
#define _Out_writes_to_ptr_(p) __attribute__((annotate("_Out_writes_to_ptr_")))
#undef _Out_writes_to_ptr_opt_
#define _Out_writes_to_ptr_opt_(p) __attribute__((annotate("_Out_writes_to_ptr_opt_")))
#undef _Out_writes_to_ptr_z_
#define _Out_writes_to_ptr_z_(p) __attribute__((annotate("_Out_writes_to_ptr_z_")))
#undef _Out_writes_to_ptr_opt_z_
#define _Out_writes_to_ptr_opt_z_(p) __attribute__((annotate("_Out_writes_to_ptr_opt_z_")))
