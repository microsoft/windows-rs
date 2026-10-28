#ifndef REQUEST
#define REQUEST 4
#endif
#ifndef TAG
#define TAG Probe
#endif
#ifndef ALIGNMENT
#define ALIGNMENT __declspec(align(REQUEST))
#endif
struct ALIGNMENT TAG { long long value; };
typedef TAG PublicProbe;
