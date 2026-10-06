typedef void* PVOID;
typedef void* LPVOID;
typedef PVOID HANDLE;
typedef PVOID const* VIEW;
struct Pointers {
    PVOID direct;
    PVOID* output;
    PVOID const* input;
    PVOID const* const* nested;
    PVOID array[2];
    LPVOID const& reference;
};
extern "C" void Compare(PVOID direct, PVOID* output, PVOID const* input, HANDLE handle, VIEW view);
typedef void (*CALLBACK)(LPVOID const* input, LPVOID* output);
