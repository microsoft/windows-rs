#ifndef VISIBILITY
#define VISIBILITY __attribute__((type_visibility("default")))
#endif
#ifndef TAG
#define TAG Probe
#endif
#ifdef FORWARD
struct VISIBILITY TAG;
#else
struct VISIBILITY TAG { int value; };
#endif
typedef TAG PublicProbe;
enum VISIBILITY Enumeration : int { Value = 1 };
