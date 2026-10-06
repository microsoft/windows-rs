//! reference-default
#define IN_Z __attribute__((annotate("_In_z_")))
typedef char* LPSTR;
typedef LPSTR TEXT;
typedef unsigned short* LPWSTR;
extern "C" void Strings(IN_Z char* input, TEXT text, LPWSTR* wide);
