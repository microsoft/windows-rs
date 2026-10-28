struct __declspec(deprecated("old record")) LegacyRecord { int value; };
typedef int OldCount __attribute__((deprecated("old count", "NewCount")));
typedef int (*LegacyCallback)(int value) __attribute__((deprecated("old callback")));
typedef struct OwnedTag { int value; } Owned __attribute__((deprecated("old typedef")));

enum class Mode {
    Old [[deprecated("old mode")]] = 1,
    Current = 2,
};
enum {
    LegacyValue [[deprecated("old anonymous")]] = 4,
};
[[deprecated]] const int LegacyConstant = 7;
struct Uses {
    __declspec(deprecated("use \"new\"\nnext")) int value;
};
struct __declspec(uuid("12345678-1234-5678-1234-567812345681"))
    __declspec(deprecated("old interface")) ILegacy {
    virtual __declspec(deprecated("old method")) int __stdcall Use(int value) = 0;
};
extern "C" __declspec(deprecated("old function")) int __stdcall NativeSum(int value);
