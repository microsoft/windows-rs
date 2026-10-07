//! namespace CallbackConvention
//! args -x c++ --target=x86_64-pc-windows-msvc -fms-extensions
#define CALLBACK __stdcall
#define DECLARE_CALLBACK(name) typedef void (CALLBACK *name)(void* context)
DECLARE_CALLBACK(CALLBACK_TYPE);
