typedef volatile int VolatileInt;
struct VolatileField { volatile int value; };
struct VolatileAliasField { VolatileInt value; };
struct VolatileArray { volatile int values[2]; };
struct VolatilePointer { int* volatile value; };
struct RestrictedPointer { int* __restrict value; };
struct SafePointer { volatile int* value; };
const volatile int VOLATILE_CONSTANT = 7;
extern "C" void UseReference(volatile int& value);
