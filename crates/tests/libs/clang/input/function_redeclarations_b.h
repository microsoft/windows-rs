typedef int INT;
#ifdef _M_IX86
extern "C" int NativeSum(int right);
#else
extern "C" int NativeSum(int left);
#endif
extern "C" int ReadPayload(INT* value);
const int STABLE = 7;
