struct Dependency {
    int value;
};
extern "C" int __stdcall Selected(Dependency* value);
extern "C" __attribute__((annotate("windows_clang:sal:2:_IRQL_requires_max_\n(0)\n(0)")))
int __stdcall Ignored(Dependency* value);
extern "C" __attribute__((annotate("windows_clang:sal:2:_IRQL_requires_max_\n(1)\n(1)")))
int __stdcall Ignored(Dependency* value);
#define SelectedAlias Selected
#define IgnoredAlias Ignored
#define ScopeConstant 17
