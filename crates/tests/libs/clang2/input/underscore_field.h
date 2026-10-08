struct NativeUnderscore {
    int _;
    unsigned unused;
};
union UnionUnderscore {
    int _;
    float value;
};
struct NestedUnderscore {
    struct {
        int _;
    } value;
};
