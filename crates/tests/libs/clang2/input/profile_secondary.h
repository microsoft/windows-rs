struct Secondary { int value; };
typedef Secondary Shared;
struct Stable { int value; };
#define ProfileValue 23
extern "C" void UseSecondary(Shared* value);
extern "C" void UseStable(Stable* value);
