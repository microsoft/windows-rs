struct NoPrototype { int (*invoke)(); };
struct FixedPrototype { int (*invoke)(int value); };
