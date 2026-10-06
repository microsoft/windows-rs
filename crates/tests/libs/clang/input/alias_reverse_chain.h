typedef char* LPSTR;
typedef LPSTR PSTR;
typedef PSTR TEXT;
typedef const unsigned short* LPCWSTR;
typedef LPCWSTR PCWSTR;
extern "C" void Strings(LPSTR* text, LPCWSTR* wide, TEXT direct, PCWSTR wide_direct);
