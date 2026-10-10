#ifdef PROFILE_PRIMARY
struct Shared { decltype(nullptr) value; };
#else
struct Shared { int value; };
#endif
extern "C" void Use(Shared* value);
