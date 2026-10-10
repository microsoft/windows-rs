struct Primary { int value; };
typedef Primary Shared;
struct Stable { int value; };
#define ProfileValue 17
extern "C" void UsePrimary(Shared* value);
