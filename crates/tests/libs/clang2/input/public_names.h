#pragma once
typedef double Seconds;
typedef unsigned Count;
typedef Count CountAlias;
typedef Count* CountPointer;
typedef void Nothing;
typedef Count (*AReader)(Count value);
typedef Count (*ZReader)(CountAlias value);
typedef ZReader ReaderAlias;
struct PublicNames {
    Seconds elapsed;
    Count count;
    const Count* values;
    CountPointer pointer;
    CountAlias array[2];
    ZReader reader;
    ReaderAlias alias;
    Nothing* context;
};
