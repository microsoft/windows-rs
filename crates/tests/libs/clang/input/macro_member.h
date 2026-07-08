// A COM method whose identifier collides with a Win32 A/W name macro. Without
// UNICODE defined, `#define DrawText DrawTextA` rewrites the method spelling to
// `DrawTextA`; the scraper must restore the faithful member name `DrawText`.
// A free function keeps its real `A` symbol and must not be rewritten.
#define DrawText DrawTextA
#define GetMessage GetMessageW

struct __declspec(uuid("20000000-0000-0000-c000-000000000046")) IRender {
    virtual int DrawText(const wchar_t* text) = 0;
    virtual int GetMessage(void* msg) = 0;
    virtual int Paint() = 0;
};
