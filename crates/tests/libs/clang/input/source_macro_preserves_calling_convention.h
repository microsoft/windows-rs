//! namespace Calling
//! library api.dll
//! args -x c++ --target=x86_64-pc-windows-msvc -fms-extensions

#define WINAPI __stdcall
             #define CALLBACK __stdcall
             void WINAPI SystemCall();
             void PlainCall();
             typedef void CALLBACK CALLBACK_TYPE(void* context);
             typedef CALLBACK_TYPE *PCALLBACK_TYPE;
