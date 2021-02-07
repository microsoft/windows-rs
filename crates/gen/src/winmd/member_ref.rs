use super::*;
macros::table!(MemberRef);

impl MemberRef {
    pub fn parent(&self) -> MemberRefParent {
        self.reader.decode(self.row, 0)
    }

    pub fn name(&self) -> &'static str {
        self.reader.str(self.row, 1)
    }
}
