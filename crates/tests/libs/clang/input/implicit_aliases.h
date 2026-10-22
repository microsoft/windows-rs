typedef __builtin_va_list Arguments;
typedef Arguments OtherArguments;
typedef int ArgumentCallback(Arguments args);
struct ArgumentUser { Arguments args; OtherArguments other; ArgumentCallback* callback; };
extern "C" int NativeSum(Arguments args);
