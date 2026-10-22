#ifndef REQUEST
#define REQUEST 8
#endif
#ifdef FORWARD
struct __declspec(align(REQUEST)) Forward;
#else
struct __declspec(align(REQUEST)) Forward { long long value; };
#endif
