#ifdef _WIN64
typedef unsigned long long ULONG_PTR;
#else
typedef unsigned long ULONG_PTR;
#endif
typedef const void* VIEW;
struct Payload {
    unsigned marker;
    ULONG_PTR size;
    char name[3];
    VIEW view;
};
enum Mode : unsigned { Small = 1, Large = 0x80000000 };
extern "C" unsigned __stdcall Send(Payload* value, const void** object);
#define Count 7
struct __declspec(uuid("00112233-4455-6677-8899-aabbccddeeff")) Root {
    virtual unsigned __stdcall Echo(const Payload* value) = 0;
    virtual void __stdcall Other() = 0;
};
