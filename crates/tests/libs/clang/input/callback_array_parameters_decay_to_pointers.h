//! namespace Callback
//! no-library
//! args -x c++ --target=x86_64-pc-windows-msvc -fms-extensions

typedef int (__stdcall *CALLBACK)(unsigned count, const unsigned values[],              const wchar_t* const names[]);
