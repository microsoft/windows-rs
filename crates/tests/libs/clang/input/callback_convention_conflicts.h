#if !defined(_M_IX86) && defined(OTHER)
#define CONVENTION __cdecl
#else
#define CONVENTION __stdcall
#endif

typedef int (CONVENTION *ConventionCallback)(int value);
struct ConventionRecord {
    int (CONVENTION *callback)(int value);
};
extern "C" int CONVENTION ConventionFunction(int value);
