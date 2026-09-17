//! namespace Callback
//! no-library

#define IN_OPT __attribute__((annotate("_In_opt_")))
             #define INOUT __attribute__((annotate("_Inout_")))
             typedef void (*CALLBACK)(IN_OPT void* context, INOUT unsigned* value);
