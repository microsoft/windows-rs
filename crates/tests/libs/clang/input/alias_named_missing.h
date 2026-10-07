typedef const char* LPCSTR;
typedef char* LPSTR;
typedef const unsigned short* LPCWSTR;
typedef unsigned short* LPWSTR;
extern "C" void Narrow(LPCSTR input, LPSTR output);
extern "C" void Wide(LPCWSTR input, LPWSTR output);
extern "C" LPCSTR Text();
typedef LPCWSTR (*CALLBACK)(LPWSTR input);
struct Strings {
    LPCSTR narrow;
    LPWSTR wide;
    LPSTR buffers[2];
};
