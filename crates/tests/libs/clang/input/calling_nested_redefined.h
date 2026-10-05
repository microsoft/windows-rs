//! namespace NestedCalling
//! args -x c++ --target=x86_64-pc-windows-msvc -fms-extensions
#define ABI __stdcall
#define DECLARE_CALLBACK(name) typedef void (ABI *name)(void)
DECLARE_CALLBACK(FIRST_CALLBACK);
#undef ABI
#define ABI __cdecl
DECLARE_CALLBACK(SECOND_CALLBACK);
