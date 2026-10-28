namespace Left {
    typedef int Number;
    struct Item { Number value; };
    enum Kind { First = 1 };
    const Number Value = 1;
}
namespace Right {
    typedef unsigned Number;
    struct Item { Number value; };
    enum Kind { First = 2 };
    const Number Value = 2;
}
namespace Left {
    struct Container {
        struct Child { Item* item; };
        Child child;
        struct { Number value; } anonymous;
    };
    typedef Number (__stdcall* Callback)(Item* item);
    struct Callbacks { Callback callback; };
    enum __MIDL_fixture_0001 { Loose = 7 };
}
struct Consumer {
    Left::Item* left;
    Right::Item* right;
    Left::Kind left_kind;
    Right::Kind right_kind;
};
