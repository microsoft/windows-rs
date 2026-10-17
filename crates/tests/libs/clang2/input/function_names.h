#ifndef VARIANT
#define VARIANT 0
#endif

#if VARIANT == 0
extern "C" int NativeSum(int, int);
#elif VARIANT == 1
extern "C" int NativeSum(int alpha, int);
#elif VARIANT == 2
extern "C" int NativeSum(int zeta, int alpha);
#else
extern "C" int NativeSum(int alpha, int zeta);
#endif
