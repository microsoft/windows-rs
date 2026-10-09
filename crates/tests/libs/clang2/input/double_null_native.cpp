#include "double_null_native.h"

extern "C" unsigned MultiLength(const WCHAR* value) {
    unsigned length = 0;
    while (value[length] != 0 || value[length + 1] != 0) {
        ++length;
    }
    return length + 2;
}

extern "C" void MakeMulti(WCHAR* value) {
    const WCHAR contents[] = {'a', 0, 'b', 'c', 0, 0};
    for (unsigned i = 0; i < 6; ++i) {
        value[i] = contents[i];
    }
}
