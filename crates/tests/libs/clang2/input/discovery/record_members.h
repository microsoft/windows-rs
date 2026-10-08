struct Outer {
    struct Inner { int value; };
    enum Mode { Active = 1 };
    Inner inner;
    Mode mode;
};
