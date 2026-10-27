struct CallbackArgument { int value; };
struct OtherArgument { int value; };
struct CallbackUses {
    void (*scalar)(int);
    void (*record)(CallbackArgument*);
    void (*other)(OtherArgument*);
    void (*nested)(void (*)(int));
    void (*pair)(int, int);
};
#ifdef CALLBACK_NAMES
typedef void (*ALong)(long);
typedef void (*BInt)(int);
typedef void (*COther)(OtherArgument*);
typedef void (*DRecord)(CallbackArgument*);
typedef void (*ENested)(void (*)(int));
typedef void (*ALongPair)(long, int);
typedef void (*BLongPair)(long, long);
typedef void (*CIntPair)(int, int);
#endif
