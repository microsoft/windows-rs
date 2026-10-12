#include <clang-c/Index.h>
#include <stdint.h>

extern "C" __declspec(dllexport) uint32_t windows_clang_layout_abi() {
    return TEST_ABI;
}

extern "C" __declspec(dllexport) const void *windows_clang_layout_parser() {
    return reinterpret_cast<const void *>(&clang_createIndex);
}

extern "C" __declspec(dllexport) const char *windows_clang_layout_fingerprint() {
    return "0000000000000000000000000000000000000000000000000000000000000000";
}
