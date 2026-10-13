#include <cstddef>
#include "interfaces.h"

struct _GUID {
    unsigned long Data1;
    unsigned short Data2;
    unsigned short Data3;
    unsigned char Data4[8];
};

static_assert(sizeof(Identifier) == 16);
static_assert(alignof(Identifier) == 4);
static_assert(sizeof(Root) == sizeof(void *));
static_assert(sizeof(Stream) == sizeof(void *));
static_assert(alignof(Root) == alignof(void *));
static_assert(alignof(Stream) == alignof(void *));

struct Instance final : Stream {
    Result __stdcall Query(const Identifier &id, void **object) override {
        *object = this;
        return static_cast<Result>(id.first);
    }
    Count __stdcall Retain() override { return 11; }
    Count __stdcall Drop() override { return 13; }
    Result __stdcall Read(void *data, Count capacity, Count *actual) override {
        *static_cast<Count *>(data) = capacity + 17;
        *actual = capacity;
        return 19;
    }
    Result __stdcall Write(const void *data, Count count, Count *actual) override {
        *actual = *static_cast<const Count *>(data) + count;
        return 23;
    }
};

int main() {
    const auto &root_id = __uuidof(Root);
    const auto &stream_id = __uuidof(Stream);
    if (root_id.Data1 != 0x00112233 || root_id.Data2 != 0x4455 ||
        root_id.Data3 != 0x6677 || stream_id.Data1 != 0x11223344 ||
        stream_id.Data2 != 0x5566 || stream_id.Data3 != 0x7788 ||
        sizeof(root_id) != 16) {
        return 1;
    }
    constexpr unsigned char root_bytes[] = {0x88, 0x99, 0xaa, 0xbb, 0xcc, 0xdd, 0xee, 0xff};
    constexpr unsigned char stream_bytes[] = {0x99, 0xaa, 0xbb, 0xcc, 0xdd, 0xee, 0xff, 0};
    for (unsigned index = 0; index < 8; ++index) {
        if (root_id.Data4[index] != root_bytes[index] ||
            stream_id.Data4[index] != stream_bytes[index]) {
            return 1;
        }
    }
    Instance instance;
    Root *root = &instance;
    Stream *stream = &instance;
    if (static_cast<void *>(root) != static_cast<void *>(stream)) {
        return 2;
    }
    auto **table = *reinterpret_cast<void ***>(stream);
    using Query = Result (__stdcall *)(Root *, const Identifier &, void **);
    using Counter = Count (__stdcall *)(Root *);
    using Read = Result (__stdcall *)(Stream *, void *, Count, Count *);
    using Write = Result (__stdcall *)(Stream *, const void *, Count, Count *);
    Identifier id{29, 0, 0, {}};
    void *object = nullptr;
    if (reinterpret_cast<Query>(table[0])(root, id, &object) != 29 || object != stream ||
        reinterpret_cast<Counter>(table[1])(root) != 11 ||
        reinterpret_cast<Counter>(table[2])(root) != 13) {
        return 3;
    }
    Count data = 0;
    Count actual = 0;
    if (reinterpret_cast<Read>(table[3])(stream, &data, 31, &actual) != 19 ||
        data != 48 || actual != 31 ||
        reinterpret_cast<Write>(table[4])(stream, &data, 37, &actual) != 23 || actual != 85) {
        return 4;
    }
    return 0;
}
