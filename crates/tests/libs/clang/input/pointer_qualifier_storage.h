#ifndef VOLATILE
#define VOLATILE volatile
#endif
#ifndef RESTRICT
#define RESTRICT __restrict
#endif
typedef VOLATILE int VolatileInt;
struct VolatileField { VOLATILE int value; };
struct VolatileAliasField { VolatileInt value; };
struct VolatileArray { VOLATILE int values[2]; };
struct VolatilePointer { int* VOLATILE value; };
struct RestrictedPointer { int* RESTRICT value; };
struct SafePointer { volatile int* value; };
const volatile int VOLATILE_CONSTANT = 7;
extern "C" void UseReference(volatile int& value);
