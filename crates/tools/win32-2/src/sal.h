// Preserve SDK annotation evidence without replacing individual SAL contracts.
// Strict specstrings mode erases some contracts, including _COM_Outptr_.
#ifndef __SPECSTRINGS_STRICT_LEVEL
#define __SPECSTRINGS_STRICT_LEVEL 0
#elif __SPECSTRINGS_STRICT_LEVEL != 0
#error SAL capture requires __SPECSTRINGS_STRICT_LEVEL=0
#endif
#include <sal.h>

#define __windows_clang_stringify_inner(...) #__VA_ARGS__
#define __windows_clang_stringify(...) __windows_clang_stringify_inner(__VA_ARGS__)

// Retain arguments at the source hook and after any further macro expansion.
// Outer SAL macros may already have substituted their arguments before this hook.
// The decoder rejects ambiguous field separators rather than guessing payload boundaries.
#undef _SAL1_Source_
#define _SAL1_Source_(Name, args, annotes) __attribute__((annotate("windows_clang:sal:1:" #Name "\n" #args "\n" __windows_clang_stringify(args))))
#undef _SAL1_1_Source_
#define _SAL1_1_Source_(Name, args, annotes) __attribute__((annotate("windows_clang:sal:1.1:" #Name "\n" #args "\n" __windows_clang_stringify(args))))
#undef _SAL1_2_Source_
#define _SAL1_2_Source_(Name, args, annotes) __attribute__((annotate("windows_clang:sal:1.2:" #Name "\n" #args "\n" __windows_clang_stringify(args))))
#undef _SAL2_Source_
#define _SAL2_Source_(Name, args, annotes) __attribute__((annotate("windows_clang:sal:2:" #Name "\n" #args "\n" __windows_clang_stringify(args))))
#undef _SAL_L_Source_
#define _SAL_L_Source_(Name, args, annotes) __attribute__((annotate("windows_clang:sal:L:" #Name "\n" #args "\n" __windows_clang_stringify(args))))

// Raw qualifiers and validity properties bypass source hooks. Keep the properties too,
// so a qualifier cannot skip an erased property and attach to a later annotation.
#define __windows_clang_primitive(Name) \
    __attribute__((annotate("windows_clang:sal:2:" Name "\n()\n()")))
#undef _Pre_
#define _Pre_ __windows_clang_primitive("_Pre_")
#undef _Post_
#define _Post_ __windows_clang_primitive("_Post_")
#undef _Notref_
#define _Notref_ __windows_clang_primitive("_Notref_")
#undef _Pre_defensive_
#define _Pre_defensive_ __windows_clang_primitive("_Pre_defensive_")
#undef _Post_defensive_
#define _Post_defensive_ __windows_clang_primitive("_Post_defensive_")
#undef _Valid_
#define _Valid_ __windows_clang_primitive("_Valid_")
#undef _Notvalid_
#define _Notvalid_ __windows_clang_primitive("_Notvalid_")
#undef _Maybevalid_
#define _Maybevalid_ __windows_clang_primitive("_Maybevalid_")

// Structural SAL bypasses the source hooks. Preserve the scope rather than emitting
// its nested annotations as unconditional contracts on the enclosing declaration.
#define __windows_clang_structure(Name, source, ...) \
    __attribute__((annotate("windows_clang:sal:2:" Name "\n(" source ")\n" \
        __windows_clang_stringify((__VA_ARGS__)))))
#undef _When_
#define _When_(expr, annos) \
    __windows_clang_structure("_When_", #expr "," #annos, expr, annos)
#undef _At_
#define _At_(target, annos) \
    __windows_clang_structure("_At_", #target "," #annos, target, annos)
#undef _At_buffer_
#define _At_buffer_(target, iter, bound, annos) \
    __windows_clang_structure("_At_buffer_", #target "," #iter "," #bound "," #annos, \
        target, iter, bound, annos)
#undef _Group_
#define _Group_(annos) __windows_clang_structure("_Group_", #annos, annos)
#undef _GrouP_
#define _GrouP_(annos) __windows_clang_structure("_GrouP_", #annos, annos)
#undef _On_failure_
#define _On_failure_(annos) __windows_clang_structure("_On_failure_", #annos, annos)
#undef _Always_
#define _Always_(annos) __windows_clang_structure("_Always_", #annos, annos)
