#define Good 17
#define Missing (absent_identifier + 1)
#define Type int
#define Broken (
#define Removed 1
#undef Removed
#define Function(value) (value)
#define Helper Good
#define Changed(value) (value)
#undef Changed
#define Changed 23
struct Packet { int value; };
extern "C" void Use(Packet* packet);
