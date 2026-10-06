#define IN __attribute__((annotate("_In_")))
#define OUT __attribute__((annotate("_Out_")))
typedef char* LPSTR;
typedef unsigned short* LPWSTR;
extern "C" void Strings(IN LPSTR input, OUT LPSTR output, IN LPWSTR wide, OUT LPWSTR buffer);
typedef void (*CALLBACK)(IN LPSTR input, OUT LPWSTR output);
