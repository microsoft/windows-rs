struct Safe { int value; };
struct Broken { unsigned& value; };
typedef Broken* (*BadResult)(Safe* first, Safe* second);
typedef void (*BadParameter)(Safe* first, Broken* second);
typedef void (*Transitive)(BadParameter callback);
struct ResultUser { BadResult callback; };
struct ParameterUser { BadParameter callback; };
struct TransitiveUser { Transitive callback; };
typedef void (*Good)(Safe* value);
struct GoodUser { Good callback; };
typedef void (*Variadic)(int count, ...);
typedef void (__attribute__((noreturn)) *NoReturn)();
typedef NoReturn (*ReturningNoReturn)();
typedef void (*TakingNoReturn)(NoReturn callback);
typedef void (*Annotated)(int value __attribute__((annotate("not decoded"))));
#if defined(_M_IX86)
typedef int (__attribute__((regparm(0))) *ZeroRegisters)(int value);
typedef int (__attribute__((regparm(2))) *Registers)(int first, int second);
#else
typedef int (*ZeroRegisters)(int value);
typedef int (*Registers)(int first, int second);
#endif
struct RegisterUser { Registers value; };
typedef void (*Volatile)(volatile int* value);
struct VolatileUser { Volatile value; };
typedef void (*UnnamedNested)(void (*callback)(char* value));
struct UnnamedUser { UnnamedNested value; };
