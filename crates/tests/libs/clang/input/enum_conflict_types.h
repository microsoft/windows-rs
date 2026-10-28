#ifndef BASE
#define BASE int
#endif
#ifndef VALUE
#define VALUE 1
#endif
#ifndef SCOPE
#define SCOPE
#endif
#ifndef TAG
#define TAG Number
#endif
enum SCOPE TAG : BASE
#ifdef FORWARD_ONLY
;
#else
{ Value = VALUE };
#endif
typedef TAG NumberAlias;
