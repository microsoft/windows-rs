#ifndef LOOSE_TAG
#define LOOSE_TAG __MIDL_Example_0001
#endif
enum LOOSE_TAG { LooseLimit = 260, LooseNegative = -1 };
typedef enum __MIDL___MIDL_itf_example_0000_0000_0002 { ModeReady = 1 } Mode;
enum Authored { AuthoredReady = 2 };
enum __MIDL_AuthoredName { SimilarPrefix = 3 };
enum class __MIDL_Example_0003 { ScopedValue = 4 };
enum __MIDL_Example_0004 { LaterValue = 5 };
typedef enum __MIDL_Example_0004 Later;
struct User { Mode mode; Authored authored; Later later; };
