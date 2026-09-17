//! namespace InputPointerAliases
//! library api.dll

#define IN __attribute__((annotate("_In_")))
             #define RESERVED __attribute__((annotate("_Reserved_")))
             typedef void *PVOID;
             typedef unsigned short OLECHAR;
             struct TP_CALLBACK_ENVIRON_V3 {};
             typedef TP_CALLBACK_ENVIRON_V3 *PTP_CALLBACK_ENVIRON;
             void Submit(IN PVOID context, RESERVED PVOID reserved,                  IN PTP_CALLBACK_ENVIRON environment, IN OLECHAR **names);
