use super::*;


// TODO: do we need this for other types?

impl Ord for CppStruct {
    fn cmp(&self, other: &Self) -> Ordering {
        // TODO: need to do the same for other Cpp types that may have multiple arches
        (self.def.name(), self.def).cmp(&(other.def.name(), other.def))
    }
}

impl PartialOrd for CppStruct {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for CppDelegate {
    fn cmp(&self, other: &Self) -> Ordering {
        self.def.name().cmp(other.def.name())
    }
}

impl PartialOrd for CppDelegate {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for CppFn {
    fn cmp(&self, other: &Self) -> Ordering {
        self.method.name().cmp(other.method.name())
    }
}

impl PartialOrd for CppFn {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for CppEnum {
    fn cmp(&self, other: &Self) -> Ordering {
        self.def.name().cmp(other.def.name())
    }
}

impl PartialOrd for CppEnum {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}


impl Ord for CppConst {
    fn cmp(&self, other: &Self) -> Ordering {
        self.field.name().cmp(other.field.name())
    }
}

impl PartialOrd for CppConst {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}