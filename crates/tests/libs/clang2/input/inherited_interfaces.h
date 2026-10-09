#include <unknwnbase.h>

struct __declspec(uuid("1e5b7c91-241a-4175-b520-552b1687b411")) IBaseValue : IUnknown {
    virtual long __stdcall Read() = 0;
};
struct __declspec(uuid("1e5b7c92-241a-4175-b520-552b1687b411")) IMarker : IBaseValue {};
struct __declspec(uuid("1e5b7c93-241a-4175-b520-552b1687b411")) ILeaf : IMarker {};
struct __declspec(uuid("1e5b7c94-241a-4175-b520-552b1687b411")) IExtended : ILeaf {
    virtual long __stdcall Extra() = 0;
};

struct InheritedStats {
    unsigned long references;
    unsigned long adds;
    unsigned long releases;
    unsigned long queries;
    unsigned long destroyed;
};

extern "C" IExtended* InheritedCreate(InheritedStats* stats);
extern "C" long InheritedRead(ILeaf* value);
extern "C" long InheritedExtra(IExtended* value);
