#include <unknwnbase.h>

struct ComStats {
    unsigned long references;
    unsigned long adds;
    unsigned long releases;
    unsigned long queries;
    unsigned long created;
    unsigned long destroyed;
};

extern "C" IClassFactory* ComFactory(ComStats* factory, ComStats* instance);
