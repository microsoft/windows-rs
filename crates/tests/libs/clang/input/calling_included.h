//! namespace IncludedCalling
//! library api.dll
//! args -x c++ --target=i686-pc-windows-msvc -fms-extensions
//! file abi.h
#define ABI __stdcall
//! input api.h
#include "abi.h"
void ABI First();
#undef ABI
#define ABI __cdecl
void ABI Second();
