//! namespace Aliases
//! input first.h
#line 1 "first-private.h"
typedef int INNER;
#line 1 "first.h"
typedef INNER OUTER;
//! input second.h
#line 1 "second-private.h"
typedef unsigned int INNER;
#line 1 "second.h"
typedef INNER OUTER;
