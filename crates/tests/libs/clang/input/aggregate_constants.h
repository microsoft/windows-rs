typedef struct _GUID {
    unsigned long Data1;
    unsigned short Data2;
    unsigned short Data3;
    unsigned char Data4[8];
} GUID;
typedef GUID IID;
typedef struct _tagpropertykey {
    GUID fmtid;
    unsigned long pid;
} PROPERTYKEY;
typedef unsigned long DEVPROPID;
typedef struct _DEVPROPKEY {
    GUID fmtid;
    DEVPROPID pid;
} DEVPROPKEY;

extern const GUID FORWARD_GUID;
extern const GUID FORWARD_GUID = {0xffffffff, 0xabcd, 0x1234, {0x80, 1, 2, 3, 4, 5, 6, 0xff}};
constexpr GUID ZERO_GUID = {};
constexpr IID COPIED_GUID = ZERO_GUID;
[[deprecated("use another key")]] constexpr PROPERTYKEY KEY = {{0x12345678, 0x9abc, 0xdef0, {1, 2, 3, 4, 5, 6, 7, 8}}, 42};
[[deprecated("use next")]] extern __declspec(selectany) const GUID SELECTED_GUID = {1u << 4, 2 + 3, 6, {7}};
constexpr GUID ARCH_GUID = {sizeof(void*), 0, 0, {}};
constexpr DEVPROPKEY DEVICE_KEY = {{0x12345678, 0x9abc, 0xdef0, {1, 2, 3, 4, 5, 6, 7, 8}}, 0xffffffff};
constexpr DEVPROPKEY ARCH_DEVICE_KEY = {{sizeof(void*), 0, 0, {}}, sizeof(void*)};
const wchar_t WIDE_TEXT[] = L"A\U0001F600\0B";
const char ASCII_TEXT[8] = "a";
const char16_t UTF16_TEXT[] = u"wide";
#define NARROW_MACRO "macro"
#define WIDE_MACRO L"wide"
#define GUID_MACRO (GUID{0x12345678, 0, 0, {0}})
