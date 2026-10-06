//! namespace Redeclared
//! library api.dll
//! args -x c++ --target=x86_64-pc-windows-msvc
void Shared(int value) asm("Shared");
void Shared(long value) asm("Shared");
