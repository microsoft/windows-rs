namespace Left {
    class Item { int value = 0; };
    struct Dependent { Item* item; };
}
namespace Right {
    struct Item { int value; };
    struct Dependent { Item* item; };
}
