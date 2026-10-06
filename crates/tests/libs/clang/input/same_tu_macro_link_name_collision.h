//! namespace Redeclared
//! library api.dll
//! args -x c++ --target=x86_64-pc-windows-msvc
#define DECLARE(type) void Shared(type value) asm("Shared")
DECLARE(int);
DECLARE(long);
