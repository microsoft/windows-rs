#include "interfaces.h"
typedef const IBase ReadOnlyBase;
typedef IBase ObjectAlias;
struct Plain { int value; };
struct Qualified {
    const IBase* object;
    IBase* const* slots;
    const IBase* const* both;
    const IBase** objects;
    ReadOnlyBase* alias;
    const Plain* data;
    ObjectAlias* named;
};
typedef const IBase* (*QualifiedCallback)(const IBase* object, IBase* const* slots);
