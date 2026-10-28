#if defined(ANNOTATED_ALIAS)
typedef struct IForeign IForeign
    __attribute__((annotate("windows_clang:sal:2:_Null_terminated_\n()\n()")));
#else
typedef struct IForeign IForeign;
#endif

struct Holder {
    IForeign* value;
};

#include "native_reference.h"

#if BY_VALUE == 2
extern "C" long Create(IBase value);
#elif defined(BY_VALUE)
extern "C" long Create(IForeign value);
#else
extern "C" long Create(Holder* value);
#endif
