typedef int Count;
struct Item { Count value; };
typedef void (*ArrayInput)(const Item values[]);
typedef void (*ArrayOutput)(Item values[]);
typedef void (*ArrayRows)(Count rows[][3]);
typedef const Item InputItems[];
typedef void (*AliasInput)(InputItems values);
struct Users {
    ArrayInput input;
    ArrayOutput output;
    ArrayRows rows;
    AliasInput alias;
};
