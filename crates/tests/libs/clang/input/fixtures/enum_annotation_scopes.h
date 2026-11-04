#define _Win32_AssociatedEnum_(name) \
    __attribute__((annotate("Win32Metadata.AssociatedEnum", "Win32Metadata.EnumName:" #name)))

enum First { FirstValue };
enum Second { SecondValue };

struct GlobalArguments {
    _Win32_AssociatedEnum_(First) int first;
    _Win32_AssociatedEnum_(Second) int second;
    _Win32_AssociatedEnum_(First) int again;
};

namespace Nested {
    enum Local { LocalValue };

    struct LocalArguments {
        _Win32_AssociatedEnum_(Local) int local;
        _Win32_AssociatedEnum_(First) int global;
        _Win32_AssociatedEnum_(Second) int other;
    };

    extern "C" void Select(
        _Win32_AssociatedEnum_(Local) int local,
        _Win32_AssociatedEnum_(First) int first,
        _Win32_AssociatedEnum_(Second) int second);
}

extern "C" _Win32_AssociatedEnum_(First) int Read(
    _Win32_AssociatedEnum_(Second) int second,
    _Win32_AssociatedEnum_(Nested::Local) int local);
