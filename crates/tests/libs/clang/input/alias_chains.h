typedef char* PSTR;
typedef PSTR LPSTR;
typedef LPSTR TEXT;
typedef const unsigned short* PCWSTR;
typedef PCWSTR LPCWSTR;
typedef LPCWSTR WTEXT;
extern "C" void Strings(LPSTR* text, LPCWSTR* wide, TEXT direct, WTEXT other);
struct Text {
    TEXT narrow;
    WTEXT wide;
};
