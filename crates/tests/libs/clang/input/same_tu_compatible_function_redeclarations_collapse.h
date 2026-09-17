//! namespace Redeclared
//! library api.dll

#define IN __attribute__((annotate("_In_")))
             extern "C" void Shared(int first);
             extern "C" void Shared(IN int second);
