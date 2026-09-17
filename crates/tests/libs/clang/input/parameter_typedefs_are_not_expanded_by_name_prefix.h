//! namespace ParameterTypedefs
//! library api.dll
//! args -x c++ --target=x86_64-pc-windows-msvc -fms-extensions

typedef __int64 LONG_PTR;
             typedef LONG_PTR LPARAM;
             typedef long PROPERTYID;
             extern "C" void UseValues(LPARAM value, PROPERTYID property);
