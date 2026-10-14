#pragma once
struct _Alpha { int value; };
struct _Beta { int value; };
typedef _Alpha Alpha;
#if defined(_M_IX86)
typedef _Alpha Zeta;
#else
typedef _Beta Zeta;
#endif
