#pragma once
typedef struct _Alpha { int value; } Alpha;
#if defined(_M_IX86)
typedef _Alpha Zeta;
#else
typedef struct _Beta { int value; } Zeta;
#endif
