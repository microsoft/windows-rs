#define IMPORT __declspec(dllimport)
#define EXPORT __declspec(dllexport)

struct Unrelated { int value; };

extern "C" IMPORT int Imported();
extern "C" EXPORT int Exported();
