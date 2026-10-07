typedef void* Handle;
typedef Handle Alias;
typedef const unsigned short* Text;

struct PointerRecord {
    Handle handle;
    Text text;
};

struct __declspec(uuid("12345678-1234-1234-1234-123456789abc")) IHandles {
    virtual void __stdcall Set(Alias value, Text text) = 0;
    virtual Handle __stdcall Get() = 0;
};

extern "C" Handle Use(PointerRecord* record, Handle* output, Text* text, const Text* input);
