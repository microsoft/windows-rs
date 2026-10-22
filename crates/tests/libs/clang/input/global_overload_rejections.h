extern "C" int External(int value);
int External(double value);
extern "C" int Defined(int value) { return value; }
inline int Defined(double value) { return static_cast<int>(value); }
inline int Only(double value) { return static_cast<int>(value); }
extern "C" int Missing(int value);
inline int Missing(double value);
#if !defined(_M_IX86)
extern "C" int Architecture(int value);
#endif
inline int Architecture(double value) { return static_cast<int>(value); }
struct Safe { int value; };
static inline int Internal(int value) { return value; }
extern "C" int StaticOverload(int value);
static inline int StaticOverload(double value) { return static_cast<int>(value); }
