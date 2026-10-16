extern "C" int Base(int value);

#if defined(_M_IX86)
#define FunctionAlias Base
#define AddressAlias (&Base)
struct Broken {
private:
    int value;
};
#define DependencyAlias ((Broken*)0)
#define ProjectionAlias ((volatile int*)1)
#define PARSE_BAD unsigned int
#define EVAL_BAD (1 / 0)
extern "C" int NativeBad(int value, ...);
#else
extern "C" int FunctionAlias(int value);
extern "C" int AddressAlias(int value);
extern "C" int DependencyAlias(int value);
extern "C" int ProjectionAlias(int value);
const int PARSE_BAD = 7;
#define EVAL_BAD 7
extern "C" int NativeBad(int value);
#endif
