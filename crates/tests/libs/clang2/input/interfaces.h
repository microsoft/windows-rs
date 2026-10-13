#pragma once

typedef long Result;
typedef unsigned long Count;

#ifdef __clang__
struct __attribute__((type_visibility("default"))) Identifier {
#else
struct Identifier {
#endif
    unsigned long first;
    unsigned short second;
    unsigned short third;
    unsigned char fourth[8];
};

#pragma pack(push, 8)
struct __declspec(uuid("00112233-4455-6677-8899-aabbccddeeff")) __declspec(novtable) Root {
    virtual Result __stdcall Query(const Identifier &id, void **object) = 0;
    virtual Count __stdcall Retain() = 0;
    virtual Count __stdcall Drop() = 0;

    template<class T> Result Query(T **object) {
        Identifier id{};
        return Query(id, reinterpret_cast<void **>(object));
    }
};

struct __declspec(uuid("11223344-5566-7788-99aa-bbccddeeff00")) __declspec(novtable) Stream : Root {
    virtual Result __stdcall Read(void *data, Count capacity, Count *actual) = 0;
    virtual Result __stdcall Write(const void *data, Count count, Count *actual) = 0;
};
#pragma pack(pop)
