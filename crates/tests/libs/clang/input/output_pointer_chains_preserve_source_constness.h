//! namespace OutputPointerChain
//! library api.dll

#define OUT __attribute__((annotate("_Out_")))
             extern "C" void GetValue(OUT const int** value);
