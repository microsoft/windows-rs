#include "interfaces.h"
struct VolatileObject { volatile IBase* object; };
struct ConstVolatileObject { const volatile IBase* object; };
struct RestrictedStorage { IBase* __restrict object; };
struct Safe { IBase* object; };
