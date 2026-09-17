//! namespace CallingRedefined
//! library api.dll
//! args -x c++ --target=i686-pc-windows-msvc -fms-extensions

#define ABI __stdcall
             void ABI First();
             #undef ABI
             #define ABI __cdecl
             void ABI Second();
