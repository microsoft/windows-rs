struct Row {
    int values[3];
};

struct Matrix {
    Row rows[2];
    float weights[2][3];
};

struct alignas(16) Aligned {
    int value;
};

struct Wrapped {
    Aligned values[2];
};

extern "C" void Use(Wrapped value);
