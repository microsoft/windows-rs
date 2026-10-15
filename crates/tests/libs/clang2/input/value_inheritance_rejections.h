#include "value_inheritance.h"
struct Other { int value; };
struct Multiple : Base, Other { int last; };
struct Private : private Base {};
struct Virtual : virtual Base {};
struct Collision : Base { int base; };
struct Empty {};
struct EmptyBase : Empty { int value; };
struct alignas(16) Aligned { int value; };
#pragma pack(push, 1)
struct PackedAligned : Aligned { int value; };
#pragma pack(pop)
extern "C" Derived NativeSum();
typedef Derived (*Returns)();
typedef void (*Takes)(Derived value);
typedef void (*TakesContainer)(Container value);
struct CallbackHolder { Derived (*returns)(); };
