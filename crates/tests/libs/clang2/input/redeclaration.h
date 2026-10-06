typedef int Required;
extern "C" void Use(Required value);
#ifdef REDECLARE
extern "C" void Use(int value);
#endif
