// Transport from microsoft/win32metadata@6becf8d2d76aafe86c8cb05b7a0ed6c0e038450a.
#define _WIN32META_ANNOTATION_(...) \
    __attribute__((annotate("Win32Metadata." __VA_ARGS__)))
#define _Win32_ImportLibrary_(...) _WIN32META_ANNOTATION_("ImportLibrary", __VA_ARGS__)
#define _Win32_SetLastError_ _WIN32META_ANNOTATION_("SetLastError")

enum class Ordinal16 : char16_t { Entry = 42 };
enum class Ordinal32 : char32_t { Entry = 65535 };

extern "C" {
_Win32_ImportLibrary_("actual.dll") _Win32_SetLastError_
int __stdcall DllOnly(int value);
_Win32_ImportLibrary_("act" "ual.dll", ("NativeEntry"))
int __stdcall Named(int value);
_Win32_ImportLibrary_("actual.dll", 40 + 2)
int __stdcall Ordinal(int value);
_Win32_ImportLibrary_(R"(actual.dll)", "#65535")
int __stdcall StringOrdinal(int value);
_Win32_ImportLibrary_("actual.dll", static_cast<char16_t>(42))
int __stdcall Char16Ordinal(int value);
_Win32_ImportLibrary_("actual.dll", static_cast<char32_t>(65535))
int __stdcall Char32Ordinal(int value);
_Win32_ImportLibrary_("actual.dll", Ordinal16::Entry)
int __stdcall Enum16Ordinal(int value);
_Win32_ImportLibrary_("actual.dll", Ordinal32::Entry)
int __stdcall Enum32Ordinal(int value);
_Win32_ImportLibrary_("actual.dll")
int DefaultConvention(int value);
}
