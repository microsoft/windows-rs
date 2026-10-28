namespace Unsupported {
    typedef __declspec(align(16)) int Number;
}
namespace Supported {
    typedef int Number;
    typedef int Other;
    extern "C" int Equivalent(Number value);
    extern "C" int Equivalent(Other value);
}
