// Regression for windows-rs#4995. The two native declarations have distinct linkage.
extern "C" int PublicValue(int value);
#define PublicValue PrivateValue
extern "C" int PublicValue(int value);
