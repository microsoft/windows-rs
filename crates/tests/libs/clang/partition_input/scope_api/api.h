// In-scope API header (the `um`/`shared` analogue). It pulls in the out-of-scope C-runtime
// header and references one of its types (`APITYPE`); that reference is what keeps
// `APITYPE` alive through the reachability sweep, while the unreferenced C-runtime noise
// in the same out-of-scope header is dropped.
#include "../scope_crt/crt.h"

void ApiCall(APITYPE p);
