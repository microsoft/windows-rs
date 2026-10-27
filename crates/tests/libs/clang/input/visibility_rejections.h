struct __attribute__((type_visibility("hidden"))) Hidden { int value; };
struct __attribute__((type_visibility("protected"))) Protected { int value; };
enum __attribute__((type_visibility("hidden"))) HiddenEnum : int { Value = 1 };
typedef Hidden HiddenAlias;
struct Dependent { HiddenAlias* value; };
struct __attribute__((visibility("default"))) SymbolVisible { int value; };
struct __attribute__((visibility("hidden"), type_visibility("default"))) Mixed {
    int value;
};
struct Survivor { int value; };
