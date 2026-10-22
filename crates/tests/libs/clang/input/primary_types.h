#pragma once
typedef struct _Zed { int value; } Zed, *PZed;
typedef Zed Aardvark;
typedef struct _Zed Another;
using Before = Zed;
struct Standalone { int value; };
typedef Standalone Alias;
typedef enum _State { Ready = 1 } State;
typedef State AState;
