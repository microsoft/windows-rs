#ifndef SHARED_CONSTANT
#define SHARED_CONSTANT 7
#endif
#ifndef RESULT_TYPE
#define RESULT_TYPE int
#endif
const int SharedConstant = SHARED_CONSTANT;
extern "C" RESULT_TYPE __stdcall SharedFunction(int value);
struct IndependentValue { int value; };
