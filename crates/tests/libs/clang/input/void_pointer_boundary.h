//! namespace VoidPointerBoundary
//! library api.dll
typedef void* PVOID;
extern "C" void Compare(PVOID const* expected, PVOID* old);
