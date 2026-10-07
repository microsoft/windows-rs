// Capture the SDK's SAL source markers, not a list of supported projection families.
// Install SDK wrappers first so their definitions cannot replace these markers.
#pragma once
#if defined(_SAL_VERSION) && defined(__SPECSTRINGS_STRICT_LEVEL) && __SPECSTRINGS_STRICT_LEVEL > 0
#error Include the clang2 SAL capture header before specstrings.h.
#endif
// Strict syntax checking replaces source contracts with empty __allowed markers.
#ifdef __SPECSTRINGS_STRICT_LEVEL
#undef __SPECSTRINGS_STRICT_LEVEL
#endif
#define __SPECSTRINGS_STRICT_LEVEL 0
#include <specstrings.h>
#include <sal.h>

#define __CLANG2_SAL_CAPTURE 1

#undef _SAL1_Source_
#define _SAL1_Source_(name, args, body) __attribute__((annotate(#name #args)))
#undef _SAL1_1_Source_
#define _SAL1_1_Source_(name, args, body) __attribute__((annotate(#name #args)))
#undef _SAL1_2_Source_
#define _SAL1_2_Source_(name, args, body) __attribute__((annotate(#name #args)))
#undef _SAL2_Source_
#define _SAL2_Source_(name, args, body) __attribute__((annotate(#name #args)))
#undef _SAL_L_Source_
#define _SAL_L_Source_(name, args, body) __attribute__((annotate(#name #args)))

// Combinators retain their complete operands, including nested annotations.
#undef _At_
#define _At_(target, body) __attribute__((annotate("_At_(" #target "," #body ")")))
#undef _At_buffer_
#define _At_buffer_(target, iter, bound, body) \
    __attribute__((annotate("_At_buffer_(" #target "," #iter "," #bound "," #body ")")))
#undef _When_
#define _When_(condition, body) __attribute__((annotate("_When_(" #condition "," #body ")")))
#undef _Group_
#define _Group_(body) __attribute__((annotate("_Group_(" #body ")")))
#undef _On_failure_
#define _On_failure_(body) __attribute__((annotate("_On_failure_(" #body ")")))
#undef _Always_
#define _Always_(body) __attribute__((annotate("_Always_(" #body ")")))

#undef _Use_decl_annotations_
#define _Use_decl_annotations_ __attribute__((annotate("_Use_decl_annotations_")))
#undef _Notref_
#define _Notref_ __attribute__((annotate("_Notref_")))
#undef _Pre_
#define _Pre_ __attribute__((annotate("_Pre_")))
#undef _Post_
#define _Post_ __attribute__((annotate("_Post_")))
#undef _Deref_
#define _Deref_ __attribute__((annotate("_Deref_")))
#undef _Notvalid_
#define _Notvalid_ __attribute__((annotate("_Notvalid_")))

// SAL primitives used without a source marker must also remain visible.
#undef _SA_annotes0
#define _SA_annotes0(name) __attribute__((annotate(#name)))
#undef _SA_annotes1
#define _SA_annotes1(name, a) __attribute__((annotate(#name "(" #a ")")))
#undef _SA_annotes2
#define _SA_annotes2(name, a, b) __attribute__((annotate(#name "(" #a "," #b ")")))
#undef _SA_annotes3
#define _SA_annotes3(name, a, b, c) __attribute__((annotate(#name "(" #a "," #b "," #c ")")))
