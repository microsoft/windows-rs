// Annotation transport for issue fixtures. Not a macro-discovery header.
#define IN __attribute__((annotate("windows_clang:sal:2:_In_\n()\n()")))
#define IN_Z __attribute__((annotate("windows_clang:sal:2:_In_z_\n()\n()")))
#define INOUT __attribute__((annotate("windows_clang:sal:2:_Inout_\n()\n()")))
#define INOUT_OPT __attribute__((annotate("windows_clang:sal:2:_Inout_opt_\n()\n()")))
#define OUT __attribute__((annotate("windows_clang:sal:2:_Out_\n()\n()")))
#define OUTPTR __attribute__((annotate("windows_clang:sal:2:_Outptr_\n()\n()")))
