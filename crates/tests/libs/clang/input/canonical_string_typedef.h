//! namespace CanonicalStringTypedef
//! library api.dll
#define IN_Z __attribute__((annotate("_In_z_")))
typedef char* LPSTR;
typedef LPSTR LPTSTR;
struct Example {
    LPTSTR text;
};
extern "C" void ReadText(IN_Z const char* text);
