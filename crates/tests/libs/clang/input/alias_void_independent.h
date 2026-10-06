//! input a.h
typedef void* PVOID;
extern "C" void Mutable(PVOID* output);
//! input b.h
typedef void* PVOID;
extern "C" void Constant(PVOID const* input);
