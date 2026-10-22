#include "interfaces.h"

#define DEFAULT_COUNT 7
int RuntimeDefault();

struct __declspec(uuid("12345678-1234-5678-1234-567812345678")) ICppMembers : IDerived {
    using IDerived::Read;
    int Read(int value) const { return value; }
    static int Helper() { return 1; }
    virtual int __stdcall Inspect(
        int count = DEFAULT_COUNT,
        const int* pointer = nullptr,
        int slots[3] = nullptr,
        int runtime = RuntimeDefault()) const noexcept = 0;
};
