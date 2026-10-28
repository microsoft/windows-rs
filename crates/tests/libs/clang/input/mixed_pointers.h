typedef const int *Input;
typedef int *Output;
typedef Input *Inputs;
struct Pointers {
    const int **mutable_pointer;
    int *const *constant_pointer;
    const int ***deep_mutable;
    int *const *const *deep_constant;
    const int *const **alternating;
    const int **array[3];
    int *const (*array_pointer)[3];
    void *const *opaque;
    Input *named_input;
    Output const *named_output;
};
typedef const int **(__cdecl *Callback)(int *const *);
