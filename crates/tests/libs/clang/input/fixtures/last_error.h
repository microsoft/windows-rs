// Transport from microsoft/win32metadata@6becf8d2d76aafe86c8cb05b7a0ed6c0e038450a.
#define _WIN32META_ANNOTATION_(...) \
    __attribute__((annotate("Win32Metadata." __VA_ARGS__)))
#define _Win32_SetLastError_ _WIN32META_ANNOTATION_("SetLastError")

extern "C" {
_Win32_SetLastError_ int __stdcall CaptureError(int value);
int __stdcall CaptureError(int value);
int __stdcall Plain(int value);
}
