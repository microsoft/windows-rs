// Regression for windows-rs#4982.
#ifndef RESULT
#define RESULT int
#endif
extern "C" RESULT SharedFunction(int value);
