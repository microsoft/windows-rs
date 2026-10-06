//! input missing.h
#define IN_Z __attribute__((annotate("_In_z_")))
typedef const char* LPCSTR;
extern "C" void Missing(IN_Z char* text, LPCSTR named);
//! input present.h
#define IN_Z __attribute__((annotate("_In_z_")))
typedef char* PSTR;
typedef const char* PCSTR;
typedef const char* LPCSTR;
extern "C" void Present(IN_Z char* text, LPCSTR named);
