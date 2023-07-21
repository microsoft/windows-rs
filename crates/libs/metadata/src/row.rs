#[derive(Copy, Clone, PartialEq, PartialOrd, Eq, Ord, Hash, Debug)]
pub struct Row {
    pub row: usize,
    pub file: usize,
}

impl Row {
    pub fn new(row: usize, file: usize) -> Self {
        Self { row, file }
    }

    fn next(&self) -> Self {
        Self { row: self.row + 1, file: self.file }
    }
}

pub trait AsRow: Copy {
    const TABLE: usize;
    fn to_row(&self) -> Row;
    fn from_row(row: Row) -> Self;

    fn file(&self) -> usize {
        self.to_row().file
    }

    fn index(&self) -> usize {
        self.to_row().row
    }

    fn next(&self) -> Self {
        Self::from_row(self.to_row().next())
    }
}

pub struct RowIterator<R: AsRow> {
    file: usize,
    rows: std::ops::Range<usize>,
    phantom: std::marker::PhantomData<R>,
}

impl<R: AsRow> RowIterator<R> {
    pub fn new(file: usize, rows: std::ops::Range<usize>) -> Self {
        Self { file, rows, phantom: std::marker::PhantomData }
    }
}

impl<R: AsRow> Iterator for RowIterator<R> {
    type Item = R;

    fn next(&mut self) -> Option<Self::Item> {
        self.rows.next().map(|row| R::from_row(Row::new(row, self.file)))
    }
}

macro_rules! tables {
    ($(($name:ident, $table:literal))+) => {
        $(
        #[derive(Copy, Clone, PartialEq, PartialOrd, Eq, Ord, Hash, Debug)]
        pub struct $name(pub(crate) Row);
        impl AsRow for $name {
            const TABLE: usize = $table;
            fn to_row(&self) -> Row {
                self.0
            }
            fn from_row(row: Row) -> Self {
                $name(row)
            }
        }
    )*
};
}

tables! {
    (Attribute, 1)
    (ClassLayout, 16)
    (Constant, 0)
    (Field, 2)
    (GenericParam, 3)
    (ImplMap, 11)
    (InterfaceImpl, 4)
    (MemberRef, 5)
    (MethodDef, 6)
    (Module, 14)
    (ModuleRef, 12)
    (AssemblyRef, 15)
    (Param, 7)
    (TypeDef, 8)
    (TypeRef, 9)
    (TypeSpec, 10)
    (NestedClass, 13)
}
