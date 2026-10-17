typedef unsigned long DWORD;
enum _Options {
    OPTION_NONE = 0,
    OPTION_HIGH = (int)0x80000000,
    OPTION_ALL = (int)0xffffffff,
};
typedef DWORD Options;
const _Options ENUM_VALUE = OPTION_ALL;
const Options INTEGER_VALUE = (Options)OPTION_ALL;
extern "C" void __stdcall BufferFunction(Options flags);
