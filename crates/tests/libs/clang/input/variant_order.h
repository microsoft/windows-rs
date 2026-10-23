#if defined(_M_IX86)
extern "C" int Public(int value);
#else
extern "C" long long Public(long long value);
#endif
