typedef unsigned int Count;
extern "C" int Sum(Count count, ...);
extern "C" int __cdecl Explicit(int first, ...);
extern "C" int NoFixed(...);
extern "C" int Fixed(int first);
extern "C" void Variadic(int first, ...);
extern "C" int __stdcall Requested(int first, ...);
static_assert(__is_same(decltype(Requested), int(int, ...)));
#define SumAlias Sum
