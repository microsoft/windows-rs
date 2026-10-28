#pragma once
#include "projection_types.h"

struct Good { int value; };
typedef int ScalarGood;
typedef void Nothing;
struct VoidUser { Nothing* value; };
typedef const int** MixedPointer;
struct PointerUser { MixedPointer value; };

int unsupported_function(int value);
union PlainUnion { int a; float b; };
struct Broken { Broken* next; unsigned& value; };
struct Dependent { Broken* value; };
typedef Dependent BadAlias;
struct Transitive { BadAlias* value; };

struct PlatformBad {
#if defined(_M_IX86)
    unsigned& value;
#else
    int value;
#endif
};
struct PlatformConsumer { PlatformBad* value; };

struct ProjectionBad { const int value; };
struct ProjectionConsumer { ProjectionBad* value; };
struct Incomplete;
struct OpaqueDependent { Incomplete* value; };

struct Ambiguous { int value; };
typedef Ambiguous ChoiceA;
typedef Ambiguous ChoiceB;
struct AmbiguousConsumer { ChoiceA* value; };

#pragma pack(push, 2)
struct UnknownPacking { char tag; int value; };
#pragma pack(pop)
struct PackingConsumer { UnknownPacking* value; };
