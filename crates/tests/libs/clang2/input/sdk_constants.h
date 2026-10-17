#include <initguid.h>
#include <windows.h>
#include <propsys.h>
#include <propkey.h>
#include <knownfolders.h>
#include <windows.foundation.h>
#include <roerrorapi.h>

static_assert(sizeof(CLSCTX) == 4);
static_assert(CLSCTX_VALID_MASK < 0);
static_assert(CLSCTX_VALID_MASK == (CLSCTX)-1612712929);
static_assert((unsigned int)CLSCTX_VALID_MASK == 2682254367u);
